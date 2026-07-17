use crate::membresias::{
  dominio::membresia::Membresia,
  puertos::{
    membresias_dto::MembresiaSocioDto,
    trait_membresia::{BuscarMembresia, ErrorMembresia},
  },
};

// Estoy pensando que no hay logica en buscar una membresia por id
// pub struct CasoObtenerMembresia<R: BuscarMembresia> {
//     repo: R,
// }

// impl<R: BuscarMembresia> CasoObtenerMembresia<R> {
//     pub fn new(repo: R) -> Self {
//         Self { repo }
//     }

//     pub async fn ejecutar(&self, id: u64) -> Result<Option<Membresia>, ErrorMembresia> {
//         self.repo.membresia_por_id(id).await
//     }
// }

pub struct CasoBuscarMembresiaPorSocio<R: BuscarMembresia> {
  repo: R,
}

impl<R: BuscarMembresia> CasoBuscarMembresiaPorSocio<R> {
  pub fn new(repo: R) -> Self {
    Self { repo }
  }

  pub async fn ejecutar(
    &self,
    dni: String,
    dias_consideracion: Option<u16>,
  ) -> Result<Option<MembresiaSocioDto>, ErrorMembresia> {
    self
      .repo
      .membresia_por_dni_socio(dni, dias_consideracion)
      .await
  }
}
