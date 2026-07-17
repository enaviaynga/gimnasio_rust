use crate::membresias::{
  dominio::membresia::Membresia,
  puertos::{
    membresias_dto::MembresiaSocioDto,
    trait_membresia::{ErrorMembresia, ListarMembresia},
  },
};

#[derive(Debug, Clone)]
pub struct CasoListarMembresias<R: ListarMembresia> {
  repo: R,
}

impl<R: ListarMembresia> CasoListarMembresias<R> {
  pub fn new(repo: R) -> Self {
    Self { repo }
  }

  pub async fn ejecutar(&self) -> Result<Vec<MembresiaSocioDto>, ErrorMembresia> {
    self.repo.todas_las_membresias().await
  }
}
