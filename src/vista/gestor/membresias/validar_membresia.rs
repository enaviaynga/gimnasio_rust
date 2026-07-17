use std::{str::FromStr, sync::Arc};

use chrono::NaiveDate;
use freya::prelude::*;
use hexagonal_gimnasio::membresias::{
  aplicacion::caso_obtener_membresia::CasoBuscarMembresiaPorSocio,
  dominio::membresia::{EstadoMembresia, TipoMembresia},
  puertos::membresias_dto::MembresiaSocioDto,
};
use infraestructura::persistencia::mysql::{contenedor::ContenedorRepos, socio};
use rust_decimal::Decimal;

use crate::vista::{
  componentes::boton_cerrar_y_volver_al_inicio::boton_cerrar, gestor::membresias::PantallaMembresia,
};

#[derive(PartialEq)]
pub struct ValidarMembresia;
impl Component for ValidarMembresia {
  fn render(&self) -> impl IntoElement {
    let pantalla_actual: State<PantallaMembresia> = use_consume();
    let socio_membresia: State<Option<MembresiaSocioDto>> = use_state(|| None);

    rect()
      .width(Size::fill())
      .height(Size::fill())
      .center()
      .child(rect().child(validar_membresia_gui(socio_membresia)))
      .child(boton_cerrar(pantalla_actual, PantallaMembresia::Inicio))
  }
}

// pendiente, añadir la generacion de reportes con un boton, para no dejarlo como solo una vista sin mas
pub fn validar_membresia_gui(
  mut socio_membresia: State<Option<MembresiaSocioDto>>,
) -> impl IntoElement {
  let contenedor: State<ContenedorRepos> = use_consume();
  let dni_input = use_state(String::new);
  let dias_consideracion = use_state(|| "4".to_string());

  rect()
    .width(Size::fill())
    .height(Size::fill())
    .center()
    .padding(Gaps::new_all(16.))
    .child(
      rect()
        .width(Size::percent(90.))
        .max_width(Size::px(700.))
        .height(Size::auto())
        .padding(Gaps::new_all(24.))
        .spacing(20.)
        .child(
          rect()
            .horizontal()
            .width(Size::fill())
            .height(Size::auto())
            .spacing(16.)
            .content(Content::Flex)
            .child(
              rect()
                .width(Size::flex(1.))
                .spacing(6.)
                .child(label().text("DNI del Socio:"))
                .child(Input::new(dni_input).placeholder("Ingrese DNI").filled()),
            )
            .child(
              rect()
                .width(Size::flex(1.))
                .spacing(6.)
                .child(label().text("Días consideración:"))
                .child(
                  Input::new(dias_consideracion)
                    .on_pre_key_down(move |e: Event<KeyboardEventData>|
                      matches!(&e.key, Key::Named(NamedKey::Backspace)) ||
                    matches!(&e.key, Key::Character(s) if !s.is_empty() && s.as_bytes().iter().all(|c| c.is_ascii_digit())))
                    .placeholder("Ej. 4")
                    .filled(),
                ),
            )
            .child(
              rect()
                .width(Size::flex(1.))
                .spacing(6.)
                .cross_align(Alignment::Center)
                .child(rect().height(Size::px(18.)))
                .child(Button::new().child("Revisar").on_press(move |_| {
                  if (*dni_input.read()).is_empty() {
                    socio_membresia.set(None);
                  }
                  spawn(async move {
                    if let Ok(membresia_dto) = CasoBuscarMembresiaPorSocio::new((*contenedor.read().membresia_repo).clone()).ejecutar((*dni_input.read()).clone(), dias_consideracion.read().parse::<u16>().ok()).await {
                      socio_membresia.set(membresia_dto);
                    };
                  });
                })),
            ),
        )
        .maybe_child(
          targeta(socio_membresia),
        ),
    )
}

fn targeta(socio_membresia: State<Option<MembresiaSocioDto>>) -> Option<impl IntoElement> {
  (*socio_membresia.read()).clone().map(|s_m| {
    rect()
      .width(Size::fill())
      .spacing(14.)
      .child(
        label()
          .text("Detalles de la Membresía Encontrada")
          .font_size(16.),
      )
      .child(
        rect()
          .width(Size::fill())
          .spacing(12.)
          .child(
            rect()
              .horizontal()
              .width(Size::fill())
              .spacing(10.)
              .child(item_info(
                "ID Membresía:",
                s_m.id.to_string(),
                Size::percent(20.),
              ))
              .child(item_info(
                "Nombre:",
                s_m.nombre_s.to_string(),
                Size::percent(40.),
              ))
              .child(item_info(
                "Apellido:",
                s_m.apellido_s.to_string(),
                Size::percent(40.),
              )),
          )
          .child(
            rect()
              .horizontal()
              .width(Size::fill())
              .spacing(10.)
              .child(item_info(
                "DNI Registrado:",
                s_m.dni_s.to_string(),
                Size::percent(34.),
              ))
              .child(item_info(
                "Tipo Membresía:",
                s_m.tipo_membresia.to_string(),
                Size::percent(33.),
              ))
              .child(item_info(
                "Costo:",
                format!("S/ {}", s_m.costo),
                Size::percent(33.),
              )),
          )
          .child(
            rect()
              .horizontal()
              .width(Size::fill())
              .spacing(10.)
              .child(item_info(
                "Fecha Inicio:",
                s_m.fecha_inicio.to_string(),
                Size::percent(34.),
              ))
              .child(item_info(
                "Fecha Vencimiento:",
                s_m.fecha_vencimiento.to_string(),
                Size::percent(33.),
              ))
              .child(item_status_info(
                "Estado actual:",
                s_m.estado.to_string(),
                s_m.estado,
                Size::percent(33.),
              )),
          ),
      )
  })
}

fn item_info(titulo: &'static str, valor: String, tamano: Size) -> impl IntoElement {
  rect()
    .width(tamano)
    .padding(Gaps::new_all(8.))
    .spacing(4.)
    .child(label().text(titulo).font_size(12.))
    .child(label().text(valor).font_size(14.))
}

fn item_status_info(
  titulo: &'static str,
  valor: String,
  es_seguro: EstadoMembresia,
  tamano: Size,
) -> impl IntoElement {
  rect()
    .width(tamano)
    .padding(Gaps::new_all(8.))
    .spacing(4.)
    .child(label().text(titulo).font_size(12.))
    .child(
      rect()
        .horizontal()
        .cross_align(Alignment::Center)
        .spacing(6.)
        // Pequeño indicador visual de semáforo
        .child(
          rect()
            .width(Size::px(8.))
            .height(Size::px(8.))
            .background(Color::from(match es_seguro {
              EstadoMembresia::Activo => (40, 167, 69), // verde
              EstadoMembresia::CercaVencimiento => (255, 255, 30), // amarillo
              EstadoMembresia::Vencido => (220, 53, 69), // rojo
            }))
            .corner_radius(CornerRadius::new_all(4.)),
        )
        .child(label().text(valor).font_size(14.)),
    )
}
