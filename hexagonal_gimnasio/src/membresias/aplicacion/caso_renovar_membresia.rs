use crate::membresias::{
  dominio::membresia::Membresia,
  puertos::trait_membresia::{ErrorMembresia, RegistrarNuevaMembresia},
};

pub struct CasoRenovarMembresia<R: RegistrarNuevaMembresia> {
  repo: R,
}

impl<R: RegistrarNuevaMembresia> CasoRenovarMembresia<R> {
  pub fn new(repo: R) -> Self {
    Self { repo }
  }

  pub async fn ejecutar(&self, membresia: Membresia) -> Result<(), ErrorMembresia> {
    self.repo.registrar_en_db(membresia).await
  }
}
