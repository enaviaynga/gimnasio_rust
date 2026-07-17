use freya::prelude::*;
use infraestructura::{
  persistencia::mysql::contenedor::ContenedorRepos,
  reportes::{
    eliminable_generador_de_reporte_asistencia_por_fecha::generar_reporte_asistencias_por_fecha,
    ingresos_stock::ganancia_membresias::generar_reporte_ingresos_membresia,
  },
};

use crate::vista::{
  componentes::boton_cerrar_y_volver_al_inicio::boton_cerrar,
  gestor::{membresias::PantallaMembresia, reporte::PantallaReporte},
};

#[derive(PartialEq)]
pub struct ReporteAsistenciaPorFecha;
impl Component for ReporteAsistenciaPorFecha {
  fn render(&self) -> impl IntoElement {
    let pantalla_actual: State<PantallaReporte> = use_consume();

    rect()
      .width(Size::fill())
      .height(Size::fill())
      .center()
      // .child(boton_cerrar(pantalla_actual, PantallaReporte::Inicio))
      .child(rect().child(vista_reporte_asistencias_por_fecha()))
  }
}

use chrono::NaiveDate;
use freya::prelude::*;

pub(crate) fn vista_reporte_asistencias_por_fecha() -> impl IntoElement {
  let contenedor: State<ContenedorRepos> = use_consume();
  let pantalla_actual: State<PantallaReporte> = use_consume();

  let mut selected_inicio = use_state(|| None::<CalendarDate>);
  let mut view_inicio = use_state(CalendarDate::now);

  let mut selected_fin = use_state(|| None::<CalendarDate>);
  let mut view_fin = use_state(CalendarDate::now);

  let mut error_msg = use_state(|| None::<String>);

  let cargando = use_state(|| false);

  rect()
    .width(Size::fill())
    .height(Size::fill())
    .center()
    .padding(Gaps::new_all(16.))
    .child(
      rect()
        .vertical()
        .width(Size::percent(95.))
        .max_width(Size::px(920.))
        .height(Size::auto())
        .padding(Gaps::new_all(24.))
        .theme_background()
        .spacing(24.)
        .child(
          label()
            .text("Reporte de Asistencia por Rango de Fechas")
            .font_size(18.),
        )
        .child(
          rect()
            .horizontal()
            .width(Size::fill())
            .height(Size::auto())
            .main_align(Alignment::SpaceBetween)
            .child(
              rect()
                .vertical()
                .width(Size::percent(47.))
                .spacing(10.)
                .child(label().text("Fecha Desde:").font_size(14.))
                .child(
                  rect().width(Size::fill()).height(Size::auto()).child(
                    Calendar::new()
                      .selected(selected_inicio())
                      .view_date(view_inicio())
                      .on_change(move |date| {
                        selected_inicio.set(Some(date));
                        error_msg.set(None);
                      })
                      .on_view_change(move |date| view_inicio.set(date)),
                  ),
                )
                .child(
                  label()
                    .text(match selected_inicio() {
                      Some(date) => format!("{:02}/{:02}/{}", date.day, date.month, date.year),
                      None => "No seleccionado".to_string(),
                    })
                    .font_size(13.),
                ),
            )
            .child(
              rect()
                .vertical()
                .width(Size::percent(47.))
                .spacing(10.)
                .child(label().text("Fecha Hasta:").font_size(14.))
                .child(
                  rect().width(Size::fill()).height(Size::auto()).child(
                    Calendar::new()
                      .selected(selected_fin())
                      .view_date(view_fin())
                      .on_change(move |date| {
                        selected_fin.set(Some(date));
                        error_msg.set(None);
                      })
                      .on_view_change(move |date| view_fin.set(date)),
                  ),
                )
                .child(
                  label()
                    .text(match selected_fin() {
                      Some(date) => format!("{:02}/{:02}/{}", date.day, date.month, date.year),
                      None => "No seleccionado".to_string(),
                    })
                    .font_size(13.),
                ),
            ),
        )
        .child(
          rect()
            .width(Size::fill())
            .vertical()
            .spacing(12.)
            .child(
              rect()
                .width(Size::fill())
                .horizontal()
                .main_align(Alignment::End)
                .child(
                  Button::new()
                    .filled()
                    .on_press(move |_| match (selected_inicio(), selected_fin()) {
                      (Some(inicio), Some(fin)) => {
                        let t_inicio = (inicio.year, inicio.month, inicio.day);
                        let t_fin = (fin.year, fin.month, fin.day);

                        if t_inicio > t_fin {
                          error_msg.set(Some(
                            "La 'Fecha Desde' no puede ser posterior a la 'Fecha Hasta'."
                              .to_string(),
                          ));
                        } else {
                          error_msg.set(None);

                          let f_inicio_opt =
                            NaiveDate::from_ymd_opt(inicio.year, inicio.month, inicio.day);
                          let f_fin_opt = NaiveDate::from_ymd_opt(fin.year, fin.month, fin.day);

                          if let (Some(f_ini), Some(f_fn)) = (f_inicio_opt, f_fin_opt) {
                            let pool_clone = contenedor.read().asistencia_repo.ref_pool().clone();
                            let mut error_msg_async = error_msg;
                            let mut cargando_async = cargando;

                            cargando_async.set(true);

                            spawn(async move {
                              match generar_reporte_asistencias_por_fecha(&pool_clone, f_ini, f_fn)
                                .await
                              {
                                Ok(_) => {
                                  println!("¡Reporte generado con éxito!");
                                }
                                Err(e) => {
                                  error_msg_async
                                    .set(Some(format!("Error en base de datos: {}", e)));
                                }
                              }
                              cargando_async.set(false);
                            });
                          } else {
                            error_msg.set(Some(
                              "Error interno al procesar el formato de fecha.".to_string(),
                            ));
                          }
                        }
                      }
                      _ => {
                        error_msg.set(Some(
                          "Por favor, selecciona ambas fechas para continuar.".to_string(),
                        ));
                      }
                    })
                    .child(
                      label()
                        .text(if *cargando.read() {
                          "Generando..."
                        } else {
                          "Generar Reporte"
                        })
                        .font_size(14.),
                    ),
                ),
            )
            .child(if let Some(msg) = &*error_msg.read() {
              rect()
                .width(Size::fill())
                .horizontal()
                .main_align(Alignment::End)
                .child(
                  label()
                    .text(msg.clone())
                    .font_size(12.)
                    .color(Color::from_rgb(239, 68, 68)),
                )
            } else {
              rect()
            }),
        )
        .child(
          Button::new()
            .child("Generar reporte ganancias por membresia")
            .on_press(move |_| match (selected_inicio(), selected_fin()) {
              (Some(inicio), Some(fin)) => {
                let t_inicio = (inicio.year, inicio.month, inicio.day);
                let t_fin = (fin.year, fin.month, fin.day);

                if t_inicio > t_fin {
                  error_msg.set(Some(
                    "La 'Fecha Desde' no puede ser posterior a la 'Fecha Hasta'.".to_string(),
                  ));
                } else {
                  error_msg.set(None);

                  let f_inicio_opt = NaiveDate::from_ymd_opt(inicio.year, inicio.month, inicio.day);
                  let f_fin_opt = NaiveDate::from_ymd_opt(fin.year, fin.month, fin.day);

                  if let (Some(f_ini), Some(f_fn)) = (f_inicio_opt, f_fin_opt) {
                    let pool_clone = contenedor.read().asistencia_repo.ref_pool().clone();
                    let mut error_msg_async = error_msg;
                    let mut cargando_async = cargando;

                    cargando_async.set(true);

                    spawn(async move {
                      match generar_reporte_ingresos_membresia(&pool_clone, f_ini, f_fn).await {
                        Ok(_) => {
                          println!("¡Reporte generado con éxito!");
                        }
                        Err(e) => {
                          error_msg_async.set(Some(format!("Error en base de datos: {}", e)));
                        }
                      }
                      cargando_async.set(false);
                    });
                  } else {
                    error_msg.set(Some(
                      "Error interno al procesar el formato de fecha.".to_string(),
                    ));
                  }
                }
              }
              _ => {
                error_msg.set(Some(
                  "Por favor, selecciona ambas fechas para continuar.".to_string(),
                ));
              }
            }),
        ),
    )
    .child(boton_cerrar(pantalla_actual, PantallaReporte::Inicio))
}
