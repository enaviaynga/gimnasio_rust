pub(crate) mod actualizar_socio;
pub(crate) mod crear_socio;
mod tabla_socios;

use freya::{prelude::*, router::*};
use tokio::task::spawn_blocking;

use crate::vista::gestor::socios::{
  actualizar_socio::ActualizarSocioGui,
  crear_socio::CrearSocioGui,
  tabla_socios::{MostrarTablaSocios, tabla_socios},
};
use hexagonal_gimnasio::gestion_socio::{
  aplicacion::caso_buscar_socio::CasoBuscarSocioPorId, dominio::socio::Socio,
};
use infraestructura::persistencia::mysql::contenedor::ContenedorRepos;

#[derive(PartialEq)]
pub struct GestionSociosGui {}
impl Component for GestionSociosGui {
  fn render(&self) -> impl IntoElement {
    let pantalla_selecionada = use_state(|| PantallaSocio::Tabla);
    let pantalla_activa = use_provide_context(|| pantalla_selecionada);
    let socio_a_editar: State<Socio> = use_state(Socio::default);
    use_provide_context(|| socio_a_editar);

    // pasarlo como parametro a mostrar tabla
    rect().child(match *pantalla_activa.read() {
      PantallaSocio::Crear => rect().child(CrearSocioGui),
      PantallaSocio::Tabla => rect().child(MostrarTablaSocios),
      PantallaSocio::Modificar => rect().child(ActualizarSocioGui),
    })
  }
}

#[derive(Clone, Copy)]
pub(crate) enum PantallaSocio {
  Tabla,
  Crear,
  Modificar,
}
