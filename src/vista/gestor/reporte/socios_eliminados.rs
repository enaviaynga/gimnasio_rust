use freya::prelude::{
  Button, ChildrenExt, Component, ContainerSizeExt, ContainerWithContentExt, Size, State, rect,
  spawn, use_consume,
};
use infraestructura::{
  persistencia::mysql::contenedor::ContenedorRepos,
  reportes::eliminacion::rpty_lista_socios_elim::generar_reporte_socios_inactivos,
};

use crate::vista::{
  componentes::boton_cerrar_y_volver_al_inicio::boton_cerrar, gestor::reporte::PantallaReporte,
};

#[derive(PartialEq)]
pub struct ReporteSociosEliminados;
impl Component for ReporteSociosEliminados {
  fn render(&self) -> impl freya::prelude::IntoElement {
    let contenedor: State<ContenedorRepos> = use_consume();
    let pantalla_actual: State<PantallaReporte> = use_consume();

    rect()
      .expanded()
      .center()
      .child(
        rect().center().child(
          Button::new()
            .child("Generar reporte socios eliminados")
            .on_press(move |_| {
              spawn(async move {
                if let Ok(ok) =
                  generar_reporte_socios_inactivos((*contenedor.read().asistencia_repo).ref_pool())
                    .await
                  && let Err(e) = opener::open(&ok)
                {
                  tracing::warn!("Error al abrir el reporte socios eliminados: {e}");
                };
              });
            }),
        ),
      )
      .child(boton_cerrar(pantalla_actual, PantallaReporte::Inicio))
  }
}
