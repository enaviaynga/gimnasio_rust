use chrono::{Local, NaiveDate};
use freya::prelude::*;
use hexagonal_gimnasio::membresias::{
  aplicacion::caso_registrar_membresia::CasoRegistrarMembresia,
  dominio::membresia::{Membresia, TipoMembresia},
};
use infraestructura::persistencia::mysql::contenedor::ContenedorRepos;
use rust_decimal::{Decimal, prelude::FromPrimitive};

use crate::vista::{
  componentes::boton_cerrar_y_volver_al_inicio::boton_cerrar, gestor::membresias::PantallaMembresia,
};

#[derive(PartialEq)]
pub struct CrearMembresia;
impl Component for CrearMembresia {
  fn render(&self) -> impl freya::prelude::IntoElement {
    let contenedor: State<ContenedorRepos> = use_consume();
    let mut pantalla_selecionada: State<PantallaMembresia> = use_consume();

    let id_socio = use_state(String::new);
    let mut fecha_inicio = use_state(|| None::<CalendarDate>);
    let mut fecha_hoy = use_state(CalendarDate::now);
    use_hook(move || {
      fecha_inicio.set(Some(*fecha_hoy.read()));
    });
    let tipo_membresia =
      use_hook(|| vec!["Mensual", "Bimestral", "Trimestral", "Semestral", "Anual"]);
    let mut selecionado = use_state(|| 0);

    let tema = use_theme();

    rect()
      .width(Size::fill())
      .height(Size::fill())
      .center()
      .child(boton_cerrar(
        pantalla_selecionada,
        PantallaMembresia::Inicio,
      ))
      .child(
        rect()
          .max_width(Size::percent(65.0))
          .min_width(Size::percent(55.0))
          .width(Size::px(800.))
          .height(Size::auto())
          .vertical()
          .background(tema.read().colors.background)
          .corner_radius(CornerRadius::new_all(14.0))
          .child(
            label()
              .text("Nueva Membresía")
              .font_size(24.0)
              .font_weight(FontWeight::BOLD),
          )
          .child(
            rect().width(Size::fill()).child(
              rect()
                .horizontal()
                .content(Content::Flex)
                .child(
                  rect()
                    .width(Size::flex(1.))
                    .vertical()
                    .padding(Gaps::new_symmetric(6.0, 0.0))
                    .child(label().text("Fecha de Inicio (DD/MM/AAAA)"))
                    .child(
                      rect()
                        .center()
                        .child(
                          rect().height(Size::px(325.)).child(
                            Calendar::new()
                              .selected(fecha_inicio())
                              .view_date(fecha_hoy())
                              .on_change(move |date: CalendarDate| {
                                let hoy = Local::now();
                                if hoy.date_naive() <= NaiveDate::from_ymd_opt(date.year, date.month, date.day).unwrap_or_default() {
                                  fecha_inicio.set(Some(date))
                                }
                            })
                              .on_view_change(move |date| fecha_hoy.set(date)),
                          ),
                        )
                        .child(match fecha_inicio() {
                          Some(date) => {
                            format!("Selected: {}/{}/{}", date.day, date.month, date.year)
                          }
                          None => "No date selected".to_string(),
                        }),
                    ),
                )
                .child(
                  rect()
                    .width(Size::flex(1.))
                    .child(
                      rect().child(
                        rect()
                          .width(Size::fill())
                          .vertical()
                          .padding(Gaps::new_symmetric(6.0, 0.0))
                          .child(label().text("ID Socio"))
                          .child(Input::new(id_socio).width(Size::fill())),
                      ),
                    )
                    .child(
                      rect()
                        .vertical()
                        .padding(Gaps::new_symmetric(6.0, 0.0))
                        .child(label().text("Tipo de Membresía"))
                        .child(
                          rect().center().horizontal().spacing(6.).child(
                            Select::new()
                              .selected_item(tipo_membresia[selecionado()].to_string())
                              .children(tipo_membresia.iter().enumerate().map(|(i, val)| {
                                MenuItem::new()
                                  .selected(selecionado() == i)
                                  .on_press(move |_| selecionado.set(i))
                                  .child(val.to_string())
                                  .into()
                              })),
                          ),
                        ),
                    ),
                ),
            ),
          )
          .child(
            rect()
              .width(Size::fill())
              .margin(Gaps::new(12.0, 0.0, 0.0, 0.0))
              .child(
                Button::new()
                  .on_press(move |_| {
                    spawn(async move {
                      if let Ok(id) = id_socio.read().parse::<u64>() {
                        let tipo_membresia = match selecionado() {
                          0 => TipoMembresia::Mensual,
                          1 => TipoMembresia::Bimestral,
                          2 => TipoMembresia::Trimestral,
                          3 => TipoMembresia::Semestral,
                          4 => TipoMembresia::Anual,
                          _ => panic!("No esperado"),
                        };
                        let fecha_freya =
                          fecha_inicio().unwrap_or_else(|| {
                            tracing::error!("Esto no deberia poder pasar, la fecha se autoseleciona en hoy, y no configure niguna forma de quitar la selecion de fecha, sol cambiarla");
                            panic!("fecha no encontrada")
                        });
                        let fecha_selecionado = NaiveDate::from_ymd_opt(
                          fecha_freya.year,
                          fecha_freya.month,
                          fecha_freya.day,
                        )
                        .unwrap_or_default();
                        let membresia = Membresia::new(
                          id,
                          tipo_membresia,
                          fecha_selecionado,
                          Decimal::from_u128(150 * (tipo_membresia as u128)).unwrap_or_default(),
                        );
                        let e =
                          CasoRegistrarMembresia::new((*contenedor.read().membresia_repo).clone())
                            .ejecutar(membresia)
                            .await;
                        tracing::warn!("{e:?}");
                        if let Err(e) = e {
                          tracing::error!("Error al registrar membresia: {e}");
                        } else {
                          pantalla_selecionada.set(PantallaMembresia::Inicio);
                        }
                      };
                    });
                  })
                  .child(
                    rect()
                      .width(Size::fill())
                      .padding(Gaps::new_symmetric(6.0, 0.0))
                      .center()
                      .child(label().text("Registrar Membresía")),
                  ),
              ),
          ),
      )
  }
}
