pub mod crear_membresia;
pub mod inicio_membresia;
pub mod listar_membresias;
pub mod renovar_membresia;
pub mod validar_membresia;

use freya::prelude::*;

use crate::vista::gestor::membresias::{
  crear_membresia::CrearMembresia, inicio_membresia::InicioMembresiaGui,
  listar_membresias::ListarMembresiaGui, renovar_membresia::RenovarMembresia,
  validar_membresia::ValidarMembresia,
};

#[derive(PartialEq)]
pub struct GestionMembresiaGui;
impl Component for GestionMembresiaGui {
  fn render(&self) -> impl freya::prelude::IntoElement {
    let pantalla_selecionada = use_state(|| PantallaMembresia::Inicio);
    use_provide_context(|| pantalla_selecionada);

    rect().child(match *pantalla_selecionada.read() {
      PantallaMembresia::Inicio => rect().child(InicioMembresiaGui),
      PantallaMembresia::Listar => rect().child(ListarMembresiaGui),
      PantallaMembresia::Crear => rect().child(CrearMembresia),
      PantallaMembresia::Renovar => rect().child(RenovarMembresia),
      PantallaMembresia::Validar => rect().child(ValidarMembresia),
    })
  }
}

#[derive(Clone, Copy)]
pub enum PantallaMembresia {
  Inicio,
  Listar,
  Crear,
  Renovar,
  // Idea para validar, aqui se coloca el dni del socio a peticion para que este sepa cuanto tiempo le queda
  Validar,
}
