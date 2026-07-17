use freya::prelude::*;
use hexagonal_gimnasio::control_asistencia::{
  aplicacion::{
    caso_obtener_asistencia::CasoListarAsistenciaDto,
    caso_obtener_info_asistencia_socio::CasoObtenerInfoAsistenciaCompleta,
    caso_registrar_asistencia::CasoRegistrarAsistencia,
  },
  puertos::asistencia_dto::{AccesoSocioDTO, AsistenciaSocioMembresiaDto},
};
use infraestructura::persistencia::mysql::contenedor::ContenedorRepos;
use itertools::{Either, Itertools};
use std::fmt::Display;

use crate::vista::gestor::asistencia::PantallaAsistencia;

#[derive(PartialEq, Clone)]
enum OrderBy {
  Id,
  Nombre,
  Apellido,
  Dni,
  FechaHora,
  Resultado,
}

impl Display for OrderBy {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      OrderBy::Id => f.write_str("Id"),
      OrderBy::Nombre => f.write_str("Nombre"),
      OrderBy::Apellido => f.write_str("Apellido"),
      OrderBy::Dni => f.write_str("DNI"),
      OrderBy::FechaHora => f.write_str("Fecha y hora"),
      OrderBy::Resultado => f.write_str("Resultado"),
    }
  }
}

#[derive(PartialEq)]
pub struct TablaAsistencia;

impl Component for TablaAsistencia {
  fn render(&self) -> impl IntoElement {
    let contenedor: State<ContenedorRepos> = use_consume();
    let caso_asistencia =
      CasoListarAsistenciaDto::new((*contenedor.read().asistencia_repo).clone());

    let vec_datos_asistencia = use_state(Vec::new);
    let recargar = use_state(|| ());

    use_side_effect_with_deps(&recargar, move |r| {
      r.read(); // para que rastree el "cambio" de recarga
      let mut vec_datos_asistencia = vec_datos_asistencia;
      let caso_asistencia = caso_asistencia.clone();
      spawn(async move {
        let r = caso_asistencia.ejecutar().await.unwrap_or_default();
        vec_datos_asistencia.set(r);
      });
    });

    rect().child(tabla_asistencia(vec_datos_asistencia, recargar))
  }
}

fn tabla_asistencia(
  vec_datos_asistencia: State<Vec<AccesoSocioDTO>>,
  recargar: State<()>,
) -> impl IntoElement {
  let contenedor: State<ContenedorRepos> = use_consume();

  let mut socio_asistencia: State<Option<AsistenciaSocioMembresiaDto>> = use_consume();

  tracing::debug!(
    "Estado de socio_asitencia: {:?}",
    &(*socio_asistencia.read())
  );

  let mut order_direction = use_state(|| OrderDirection::Up);
  let mut order = use_state(|| OrderBy::Id);
  let mut pantalla_selecionada: State<PantallaAsistencia> = use_consume();

  let columns = use_hook(|| {
    vec![
      OrderBy::Id,
      OrderBy::Nombre,
      OrderBy::Apellido,
      OrderBy::Dni,
      OrderBy::FechaHora,
      OrderBy::Resultado,
    ]
  });

  let raw_data = vec_datos_asistencia.read();
  let sorted_raw_data = {
    let sorted = raw_data.iter().sorted_by(|a, b| match *order.read() {
      OrderBy::Id => b.id_asistencia.cmp(&a.id_asistencia),
      OrderBy::Nombre => Ord::cmp(
        &a.nombre_socio.to_lowercase(),
        &b.nombre_socio.to_lowercase(),
      ),
      OrderBy::Apellido => Ord::cmp(
        &a.apellido_socio.to_lowercase(),
        &b.apellido_socio.to_lowercase(),
      ),
      OrderBy::Dni => Ord::cmp(&a.dni, &b.dni),

      OrderBy::FechaHora => Ord::cmp(
        &(a.fecha_asistencia, a.hora_asistencia),
        &(b.fecha_asistencia, b.hora_asistencia),
      ),
      OrderBy::Resultado => Ord::cmp(
        &a.resultado_validacion.to_string(),
        &b.resultado_validacion.to_string(),
      ),
    });

    if *order_direction.read() == OrderDirection::Down {
      Either::Right(sorted.rev())
    } else {
      Either::Left(sorted)
    }
  };

  let filtered_data: Vec<Vec<String>> = sorted_raw_data
    .map(|a| {
      vec![
        a.id_asistencia.to_string(),
        a.nombre_socio.to_string(),
        a.apellido_socio.to_string(),
        a.dni.to_string(),
        a.fecha_asistencia.to_string(),
        a.hora_asistencia.to_string(),
        a.resultado_validacion.to_string(),
      ]
    })
    .collect();

  let mut on_column_head_click = move |column_order: &OrderBy| {
    if &*order.read() == column_order {
      if *order_direction.read() == OrderDirection::Up {
        order_direction.set(OrderDirection::Down)
      } else {
        order_direction.set(OrderDirection::Up)
      }
    } else {
      order.set(column_order.clone());
      order_direction.set(OrderDirection::default())
    }
  };

  let mut dni_texto = use_state(String::new);

  rect()
    .expanded()
    .padding(8.)
    .child(
      rect()
        .padding(Gaps::new(0., 0., 5., 0.))
        .width(Size::percent(100.))
        .content(Content::Flex)
        .horizontal()
        .child(
          Input::new(dni_texto)
            .placeholder("Dni")
            .width(Size::flex(1.)),
        )
        .child(
          rect()
            .padding(2.)
            .center()
            // Validar el Dni, o mostrar el popup con la razon (si existe, o el dni es realista)
            .child(Button::new().child("Registrar").on_press(move |_| {
              let mut recargar = recargar;
              spawn(async move {
                let dni = dni_texto.read().clone();
                if let Err(e) =
                  CasoRegistrarAsistencia::new((*contenedor.read().asistencia_repo).clone())
                    .ejecutar(&dni)
                    .await
                {
                  tracing::warn!(
                    "Error al registrar la asistencia del socio con el dni: {dni} | error: {e}"
                  );
                } else {
                  dni_texto.write().clear();
                  recargar.set(()); // para forzar la actualizacion de la lista, me dio pereza rehacer la logica
                }
              });
            })),
        ),
    )
    .child(
      Table::new()
        .column_widths([
          Size::flex(1.),
          Size::flex(3.),
          Size::flex(3.),
          Size::flex(2.),
          Size::flex(2.),
          Size::flex(1.5),
          Size::flex(2.),
        ])
        .child(
          TableHead::new().child(
            TableRow::new().children(
              vec![
                (OrderBy::Id, "Id"),
                (OrderBy::Nombre, "Nombre"),
                (OrderBy::Apellido, "Apellido"),
                (OrderBy::Dni, "DNI"),
                (OrderBy::FechaHora, "Fecha"),
                (OrderBy::FechaHora, "Hora"),
                (OrderBy::Resultado, "Resultado"),
              ]
              .into_iter()
              .enumerate()
              .map(|(n, (order_by, label))| {
                TableCell::new()
                  .key(n)
                  .order_direction(if *order.read() == order_by {
                    Some(*order_direction.read())
                  } else {
                    None
                  })
                  .child(label.to_string())
                  .on_press(move |_| on_column_head_click(&order_by))
                  .into()
              }),
            ),
          ),
        )
        .child(TableBody::new().child(ScrollView::new().children(
          filtered_data.into_iter().enumerate().map(|(i, items)| {
            TableRow::new()
              .key(i)
              .children(items.iter().enumerate().map(|(n, item)| {
                TableCell::new()
                  .key(n)
                  .child(item.clone())
                  .on_press({
                    let value = items[0].clone();
                    move |_| {
                      tracing::debug!(
                        "intentando selecionar un elemento de la tabla: {:?}",
                        &value
                      );
                      let caso = CasoObtenerInfoAsistenciaCompleta::new(
                        (*contenedor.read().asistencia_repo).clone(),
                      );
                      let value = value.clone();
                      spawn(async move {
                        if let Ok(id_asistencia) = value.parse() {
                          let r = caso.ejecutar(id_asistencia).await;
                          match r {
                            Ok(ok) => {
                              socio_asistencia.set(Some(ok));
                              pantalla_selecionada.set(PantallaAsistencia::Ver);
                            }
                            // pendiente:
                            // - o crear un vista para las asistencia no permitidas (solo socio-asistencia, ya no membresia)
                            // - o añadir un popup que indica que no hay mas informacion para una asistencia rechazada (porque no hay membresia)
                            Err(e) => tracing::error!("Error hacer click en asistencia: {e}"),
                          }
                        }
                      });
                    }
                  })
                  .into()
              }))
              .into()
          }),
        ))),
    )
}
