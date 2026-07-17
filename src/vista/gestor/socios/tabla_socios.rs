use std::fmt::Display;

use freya::prelude::*;
use itertools::{Either, Itertools};

use crate::vista::gestor::socios::PantallaSocio;
use hexagonal_gimnasio::{
  empleado::dominio::usuario::{self, Usuario},
  gestion_socio::{
    aplicacion::{
      caso_buscar_socio::CasoBuscarSocioPorId, caso_listar_socios::CasoBuscarSocioPorFiltro,
    },
    dominio::socio::Socio,
  },
};
use infraestructura::persistencia::mysql::{
  contenedor::ContenedorRepos, socio::impl_socio::actualizar_socio,
};

#[derive(PartialEq)]
pub struct MostrarTablaSocios;

impl Component for MostrarTablaSocios {
  fn render(&self) -> impl IntoElement {
    let contenedor: State<ContenedorRepos> = use_consume();
    let caso_socio = CasoBuscarSocioPorFiltro::new((*contenedor.read().socio_repo).clone());
    let socios_vec: State<Vec<Socio>> = use_state(std::vec::Vec::new);
    use_side_effect(move || {
      let mut socios_vec = socios_vec;
      let caso_socio = caso_socio.clone();
      spawn(async move {
        let r = caso_socio.ejecutar().await.unwrap_or_default();
        *socios_vec.write() = r;
      });
    });
    rect().child(tabla_socios(socios_vec))
  }
}

#[derive(Debug, PartialEq, Clone)]
enum OrdenPor {
  Id,
  Nombre,
  Apellido,
  Dni,
  Telefono,
  Correo,
  Direccion,
}

impl Display for OrdenPor {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      OrdenPor::Id => f.write_str("Id"),
      OrdenPor::Nombre => f.write_str("Nombre"),
      OrdenPor::Apellido => f.write_str("Apellido"),
      OrdenPor::Dni => f.write_str("Dni"),
      OrdenPor::Telefono => f.write_str("Telefono"),
      OrdenPor::Correo => f.write_str("Correo"),
      OrdenPor::Direccion => f.write_str("Direccion"),
    }
  }
}

pub(crate) fn tabla_socios(socios: State<Vec<Socio>>) -> impl IntoElement {
  let contenedor: State<ContenedorRepos> = use_consume();
  let mut pantalla_socio_actual: State<PantallaSocio> = use_consume();
  let mut socio_editable: State<Socio> = use_consume();

  let mut orden_direccion = use_state(|| OrderDirection::Down);
  let mut orden = use_state(|| OrdenPor::Id);

  let buscar_texto = use_state(String::new);

  let datos = use_memo(move || {
    let socios = &*socios.read();
    let mut lista = vec![];

    for s in socios {
      tracing::trace!("{s:?}");
      if let Some(id) = s.get_id() {
        let nombre = s.get_nombre().to_string();
        let apellido = s.get_apellido().to_string();
        let dni = s.get_dni().to_string();
        let telefono = s.get_telefono().unwrap_or("").to_string();
        let correo = s.get_correo().unwrap_or("").to_string();
        let direccion = s.get_direccion().unwrap_or("").to_string();

        lista.push(vec![
          id.to_string(),
          nombre,
          apellido,
          dni,
          telefono,
          correo,
          direccion,
        ]);
      }
    }
    lista
  });
  let datos = datos.read();

  let columnas = use_hook(|| {
    vec![
      ("Id", OrdenPor::Id),
      ("Nombre", OrdenPor::Nombre),
      ("Apellido", OrdenPor::Apellido),
      ("Dni", OrdenPor::Dni),
      ("Telefono", OrdenPor::Telefono),
      ("Correo", OrdenPor::Correo),
      ("Direccion", OrdenPor::Direccion),
    ]
  });

  let datos_filtrados = {
    let texto = buscar_texto.read().to_lowercase();

    let filtrado = datos.iter().filter(|item| {
      item
        .iter()
        .any(|elemento| elemento.to_lowercase().contains(&texto))
    });

    let datos_filtrados = filtrado.sorted_by(|a, b| match *orden.read() {
      OrdenPor::Id => {
        let num_a = a[0].parse::<u32>().unwrap_or(0);
        let num_b = b[0].parse::<u32>().unwrap_or(0);

        num_a.cmp(&num_b)
      }
      OrdenPor::Nombre => Ord::cmp(&a[1].to_lowercase(), &b[1].to_lowercase()),
      OrdenPor::Apellido => Ord::cmp(&a[2].to_lowercase(), &b[2].to_lowercase()),
      OrdenPor::Dni => Ord::cmp(&a[3].to_lowercase(), &b[3].to_lowercase()),
      OrdenPor::Telefono => Ord::cmp(&a[4].to_lowercase(), &b[4].to_lowercase()),
      OrdenPor::Correo => Ord::cmp(&a[5].to_lowercase(), &b[5].to_lowercase()),
      OrdenPor::Direccion => Ord::cmp(&a[6].to_lowercase(), &b[6].to_lowercase()),
    });

    if *orden_direccion.read() == OrderDirection::Down {
      Either::Left(datos_filtrados)
    } else {
      Either::Right(datos_filtrados.rev())
    }
  };

  tracing::trace!("Datos filtrados");

  let mut click_en_columna = move |orden_columna: &OrdenPor| {
    if &*orden.read() == orden_columna {
      if *orden_direccion.read() == OrderDirection::Up {
        orden_direccion.set(OrderDirection::Down);
      } else {
        orden_direccion.set(OrderDirection::Up);
      }
    } else {
      orden.set(orden_columna.clone());
      orden_direccion.set(OrderDirection::default());
    }
  };


  rect()
    .padding(8.)
    .child(
      rect()
        .center()
        .content(Content::Flex)
        .width(Size::fill())
        .horizontal()
        .child(
          label()
            .text("Buscar: ")
            .padding(Gaps::new_symmetric(0., 5.))
            .font_size(24.0),
        )
        .child(Input::new(buscar_texto).width(Size::flex(1.)))
        .child(
          rect().center().padding(Gaps::new_symmetric(0., 5.)).child(
            Button::new()
              .child("Registrar usuario")
              .on_press(move |_| *pantalla_socio_actual.write() = PantallaSocio::Crear),
          ),
        ),
    )
    .child(
      Table::new()
        .column_widths([
          Size::flex(1.),
          Size::flex(2.),
          Size::flex(2.),
          Size::flex(1.5),
          Size::flex(1.5),
          Size::flex(3.),
          Size::flex(3.),
        ])
        .child(
          TableHead::new().child(
            TableRow::new().children(columnas.into_iter().enumerate().map(
              |(n, (texto, orden_por))| {
                TableCell::new()
                  .key(n)
                  .order_direction(if *orden.read() == orden_por {
                    Some(*orden_direccion.read())
                  } else {
                    None
                  })
                  .on_press(move |_| click_en_columna(&orden_por))
                  .child(texto.to_string())
                  .into()
              },
            )),
          ),
        )
        .child(
          TableBody::new().child(ScrollView::new().expanded().children(
            datos_filtrados.enumerate().map(|(i, items)| {
              TableRow::new()
                .key(i)
                .children(items.iter().enumerate().map(|(n, item)| {
                  let id = items[0].clone();

                  let elementos = items.clone();
                  TableCell::new()
                    .key(n)
                    .child(item.to_string())
                    .on_press(move |_| {
                      tracing::debug!("{:?}", elementos.clone());
                      if let Ok(id) = id.parse() {
                        spawn(async move {
                          if let Ok(Some(socio)) =
                            CasoBuscarSocioPorId::new((*contenedor.read().socio_repo).clone())
                              .ejecutar(id)
                              .await
                          {
                            *socio_editable.write() = socio;
                            *pantalla_socio_actual.write() = PantallaSocio::Modificar;
                          }
                        });
                      } else {
                        tracing::info!("Fallo en elegir socio para editar")
                      }
                    })
                    .into()
                }))
                .into()
            }),
          )),
        ),
    )
}
