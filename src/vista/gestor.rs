pub mod asistencia;
pub(crate) mod config;
pub mod membresias;
pub mod reporte;
pub(crate) mod socios;
pub mod venta;

use freya::{prelude::*, router::*};

use crate::vista::{
  componentes::selector_barra_menu::barra_btn,
  configuracion::{ConfiguracionGim, TipoTema},
  gestor::{
    asistencia::AsistenciaGui, membresias::GestionMembresiaGui, reporte::ReporteGui,
    venta::GestionVentaGui,
  },
  temas::{tema_claro_no_state, tema_oscuro_no_state},
};
use hexagonal_gimnasio::empleado::dominio::empleado::{EmpleadoEnum, Permisos};

use config::TemaSelector;
use socios::GestionSociosGui;

#[derive(PartialEq)]
pub struct ContenedorGeneral {}
impl Component for ContenedorGeneral {
  fn render(&self) -> impl IntoElement {
    let mut tema_general = use_theme();
    let mut conf: State<ConfiguracionGim> = use_consume();

    let empleado_global: State<EmpleadoEnum> = use_consume();

    rect()
      .spacing(6.)
      .padding(Gaps::new_all(4.))
      .child(
        rect()
          .direction(Direction::Horizontal)
          .child(
            rect()
              .horizontal()
              .child(barra_btn(
                &empleado_global,
                Permisos::SIN_PERMISOS,
                "Inicio",
                Rutas::Inicio,
              ))
              .child(barra_btn(
                &empleado_global,
                Permisos::SOCIOS,
                "Socios",
                Rutas::GestionSociosGui,
              ))
              .child(barra_btn(
                &empleado_global,
                Permisos::MEMBRESIAS,
                "Membresia",
                Rutas::GestionMembresiaGui,
              ))
              .child(barra_btn(
                &empleado_global,
                Permisos::ASISTENCIA,
                "Asistencia",
                Rutas::AsistenciaGui,
              ))
              .child(barra_btn(
                &empleado_global,
                Permisos::INVENTARIO | Permisos::VENTAS,
                "Venta",
                Rutas::GestionVentaGui,
              ))
              .child(barra_btn(
                &empleado_global,
                Permisos::ADMINISTRADOR,
                "Reporte",
                Rutas::ReporteGui,
              ))
              .child(barra_btn(
                &empleado_global,
                Permisos::SIN_PERMISOS,
                "Configuracion",
                Rutas::TemaSelector,
              )),
          )
          .child(
            rect()
              .position(Position::new_absolute().right(0.5))
              .horizontal()
              .center()
              .child(
                match conf.read().tema {
                  super::configuracion::TipoTema::Claro => "☀️",
                  super::configuracion::TipoTema::Oscuro => "🌘",
                }
                .to_string(),
              )
              .child(
                Switch::new()
                  .toggled(conf.read().tema == TipoTema::Oscuro)
                  .on_toggle(move |_| {
                    conf.write().tema.invertir();
                    tracing::info!("Configuracion info: {:?}", conf.read().tema);
                    match conf.read().tema {
                      super::configuracion::TipoTema::Claro => {
                        tema_general.set(tema_claro_no_state(conf.read().tema_claro))
                      }
                      super::configuracion::TipoTema::Oscuro => {
                        tema_general.set(tema_oscuro_no_state(conf.read().tema_oscuro))
                      }
                    }
                  }),
              ),
          ),
      )
      .child(
        rect()
          .border(
            Border::new()
              .alignment(BorderAlignment::Inner)
              .width(3.0)
              .fill(tema_general.read().colors.border),
          )
          .center()
          .expanded()
          .child(Outlet::<Rutas>::new()),
      )
  }
}

#[derive(PartialEq)]
pub struct Inicio {}
impl Component for Inicio {
  fn render(&self) -> impl IntoElement {
    let empleado_global: State<EmpleadoEnum> = use_consume();
    let temp = match &*empleado_global.read() {
      EmpleadoEnum::Activo(empleado) => &format!(
        "nombre usuario: {}\npermisos: {:?}",
        empleado.ref_name(),
        empleado.get_permisos()
      ),

      _ => "usuario inexistente",
    };
    rect().child(temp)
  }
}

#[derive(Routable, Clone, Debug, PartialEq)]
#[rustfmt::skip]
pub enum Rutas {
    #[layout(ContenedorGeneral)]
        #[route("/")]
        Inicio,
        #[route("/socios")]
        GestionSociosGui,
        #[route("/temas")]
        TemaSelector,
        #[route("/membresia")]
        GestionMembresiaGui,
        #[route("/asistencia")]
        AsistenciaGui,
        #[route("/venta")]
        GestionVentaGui,
        #[route("/reporte")]
        ReporteGui
}
