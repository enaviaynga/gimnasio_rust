use freya::{icons::lucide::x, prelude::*};
use hexagonal_gimnasio::membresias::{
  aplicacion::caso_listar_membresias::CasoListarMembresias,
  puertos::membresias_dto::MembresiaSocioDto,
};
use infraestructura::persistencia::mysql::contenedor::ContenedorRepos;

use crate::vista::{
  componentes::boton_cerrar_y_volver_al_inicio::boton_cerrar, gestor::membresias::PantallaMembresia,
};

#[derive(PartialEq)]
pub struct ListarMembresiaGui;
impl Component for ListarMembresiaGui {
  fn render(&self) -> impl IntoElement {
    let contenedor: State<ContenedorRepos> = use_consume();
    let pantalla_actual: State<PantallaMembresia> = use_consume();
    let caso_membresia = CasoListarMembresias::new((*contenedor.read().membresia_repo).clone());
    let membresias = use_state(std::vec::Vec::new);

    use_side_effect(move || {
      let mut membresia_vec = membresias;
      let caso_membresia = caso_membresia.clone();
      spawn(async move {
        let r = caso_membresia.ejecutar().await.unwrap_or_default();
        *membresia_vec.write() = r;
      });
    });

    rect()
      .width(Size::fill())
      .height(Size::fill())
      .center()
      .child(boton_cerrar(pantalla_actual, PantallaMembresia::Inicio))
      .child(
        rect()
          .content(Content::Flex)
          .child(
            rect()
              .width(Size::flex(1.))
              .height(Size::flex(1.))
              .padding(Gaps::new_all(10.))
              .child(label().text("Lista de Membresias").font_size(24.)),
          )
          .child(listar_membresias(membresias)),
      )
  }
}

use itertools::{Either, Itertools};
use std::fmt::Display;

#[derive(PartialEq, Clone)]
enum OrdenPor {
  Id,
  NombreS,
  ApellidoS,
  DniS,
  TipoMembresia,
  FechaInicio,
  FechaVencimiento,
  Estado,
  Costo,
}

impl Display for OrdenPor {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      OrdenPor::Id => f.write_str("ID"),
      OrdenPor::NombreS => f.write_str("Nombre"),
      OrdenPor::ApellidoS => f.write_str("Apellido"),
      OrdenPor::DniS => f.write_str("DNI"),
      OrdenPor::TipoMembresia => f.write_str("Membresía"),
      OrdenPor::FechaInicio => f.write_str("F. Inicio"),
      OrdenPor::FechaVencimiento => f.write_str("F. Vencimiento"),
      OrdenPor::Estado => f.write_str("Estado"),
      OrdenPor::Costo => f.write_str("Costo"),
    }
  }
}

fn listar_membresias(membresias: State<Vec<MembresiaSocioDto>>) -> impl IntoElement {
  let mut order_direction = use_state(|| OrderDirection::Down);
  let mut order = use_state(|| OrdenPor::Id);

  let data = use_memo(move || {
    let membresias = &*membresias.read();
    let mut lista = vec![];

    for m in membresias {
      let id = m.id.to_string();
      let nombre = m.nombre_s.to_string();
      let apellido = m.apellido_s.to_string();
      let dni = m.dni_s.to_string();
      let tipo_membresia = m.tipo_membresia.to_string();
      let f_inicio = m.fecha_inicio.to_string();
      let f_vencimiento = m.fecha_vencimiento.to_string();
      let costo = m.costo.to_string();
      let estado = m.estado.to_string();

      lista.push(vec![
        id,
        nombre,
        apellido,
        dni,
        tipo_membresia,
        f_inicio,
        f_vencimiento,
        costo,
        estado,
      ]);
    }
    lista
  });

  let data = data.read();

  let columns = use_hook(|| {
    vec![
      ("ID", OrdenPor::Id),
      ("Nombre", OrdenPor::NombreS),
      ("Apellido", OrdenPor::ApellidoS),
      ("DNI", OrdenPor::DniS),
      ("Membresía", OrdenPor::TipoMembresia),
      ("F. Inicio", OrdenPor::FechaInicio),
      ("F. Vencimiento", OrdenPor::FechaVencimiento),
      ("Costo", OrdenPor::Costo),
      ("Estado", OrdenPor::Estado),
    ]
  });

  let filtered_data = {
    let filtered_data = data.iter().sorted_by(|a, b| match *order.read() {
      OrdenPor::Id => {
        let num_a = a[0].parse::<u32>().unwrap_or(0);
        let num_b = b[0].parse::<u32>().unwrap_or(0);

        num_a.cmp(&num_b)
      }
      OrdenPor::NombreS => Ord::cmp(&a[1].to_lowercase(), &b[1].to_lowercase()),
      OrdenPor::ApellidoS => Ord::cmp(&a[2].to_lowercase(), &b[2].to_lowercase()),
      OrdenPor::DniS => Ord::cmp(&a[3].to_lowercase(), &b[3].to_lowercase()),
      OrdenPor::TipoMembresia => Ord::cmp(&a[4].to_lowercase(), &b[4].to_lowercase()),
      OrdenPor::FechaInicio => Ord::cmp(&a[5].to_lowercase(), &b[5].to_lowercase()),
      OrdenPor::FechaVencimiento => Ord::cmp(&a[6].to_lowercase(), &b[6].to_lowercase()),
      OrdenPor::Estado => Ord::cmp(&a[7].to_lowercase(), &b[7].to_lowercase()),
      OrdenPor::Costo => Ord::cmp(&a[8].to_lowercase(), &b[8].to_lowercase()),
    });

    if *order_direction.read() == OrderDirection::Down {
      Either::Left(filtered_data)
    } else {
      Either::Right(filtered_data.rev())
    }
  };

  let mut on_column_head_click = move |column_order: &OrdenPor| {
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

  rect().padding((30., 8., 8., 8.)).child(
    Table::new()
      .column_widths([
        Size::flex(1.), 
        Size::flex(2.5),
        Size::flex(2.5),
        Size::flex(2.), 
        Size::flex(2.), 
        Size::flex(2.), 
        Size::flex(2.), 
        Size::flex(1.), 
        Size::flex(2.5),
      ])
      .child(
        TableHead::new().child(
          TableRow::new().children(
            columns
              .into_iter()
              .enumerate()
              .map(|(n, (text, order_by))| {
                TableCell::new()
                  .key(n)
                  .order_direction(if *order.read() == order_by {
                    Some(*order_direction.read())
                  } else {
                    None
                  })
                  .on_press(move |_| on_column_head_click(&order_by))
                  .child(text.to_string())
                  .into()
              }),
          ),
        ),
      )
      .child(
        TableBody::new().child(ScrollView::new().children(filtered_data.enumerate().map(
          |(i, items)| {
            TableRow::new()
              .key(i)
              .children(
                items
                  .iter()
                  .enumerate()
                  .map(|(n, item)| TableCell::new().key(n).child(item.to_string()).into()),
              )
              .into()
          },
        ))),
      ),
  )
}
