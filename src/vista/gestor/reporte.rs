use std::path::Path;

use freya::prelude::*;
use infraestructura::{
  persistencia::mysql::contenedor::ContenedorRepos,
  reportes::{
    eliminacion::rpty_lista_socios_elim::generar_reporte_socios_inactivos,
    formulas::reporte_final::generar_reporte_anual_typst,
  },
};
use opener::open;

use crate::vista::gestor::reporte::{
  asistencia_por_fecha::ReporteAsistenciaPorFecha, socios_eliminados::ReporteSociosEliminados,
};

pub mod asistencia_por_fecha;
pub mod socios_eliminados;

#[derive(PartialEq)]
pub struct ReporteGui;
impl Component for ReporteGui {
  fn render(&self) -> impl freya::prelude::IntoElement {
    let mut pantalla_seleccionada = use_state(|| PantallaReporte::Inicio);
    use_provide_context(|| pantalla_seleccionada);

    let contenedor: State<ContenedorRepos> = use_consume();

    rect().child(match *pantalla_seleccionada.read() {
      PantallaReporte::Inicio => rect()
        .child(Button::new().child("Por fecha").on_press(move |_| {
          pantalla_seleccionada.set(PantallaReporte::ReportePorFecha);
        }))
        .child(Button::new().child("Socios Eliminados").on_press(move |_| {
          spawn(async move {
            if let Ok(ok) =
              generar_reporte_socios_inactivos((*contenedor.read().asistencia_repo).ref_pool())
                .await
              && let Err(e) = opener::open(&ok)
            {
              tracing::warn!("Error al abrir el reporte socios eliminados: {e}");
            };
          });
        }))
        .child(
          Button::new()
            .child("Reporte Indicadores")
            .on_press(move |_| {
              spawn(async move {
                tracing::info!("Generando el reporte anual");
                let r =
                  generar_reporte_anual_typst(contenedor.read().asistencia_repo.ref_pool(), 2026)
                    .await;
                tracing::debug!("Resultado: {:?}", r);
                match r {
                  Ok(ruta) => {
                    let path = Path::new(&ruta);

                    match open(path) {
                      Ok(_) => println!("¡PDF abierto con éxito!"),
                      Err(e) => eprintln!("Error al intentar abrir el archivo: {}", e),
                    }
                  }
                  Err(e) => tracing::debug!("pendiente añadir logica aqui"),
                }
              });
            }),
        ),
      PantallaReporte::RepoteSociosEliminados => rect().child(ReporteSociosEliminados),
      PantallaReporte::ReportePorFecha => rect().child(ReporteAsistenciaPorFecha),
    })
  }
}

#[derive(Clone, Copy)]
enum PantallaReporte {
  Inicio,
  RepoteSociosEliminados,
  ReportePorFecha,
}
