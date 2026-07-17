use crate::membresias::{
  dominio::membresia::{EstadoMembresia, Membresia},
  puertos::trait_membresia::{ErrorMembresia, VerificarVencimientosProgramados},
};

pub struct CasoVerificarVencimientoMembresia<R: VerificarVencimientosProgramados> {
  repo: R,
}

impl<R: VerificarVencimientosProgramados> CasoVerificarVencimientoMembresia<R> {
  pub fn new(repo: R) -> Self {
    Self { repo }
  }

  pub fn ejecutar(&self, membresia: &Membresia) -> EstadoMembresia {
    self.repo.verificar_una_membresia(membresia)
  }
}

// pub struct CasoVerificarVencimientosProximosMembresia<R: VerificarVencimientosProgramados> {
//   repo: R,
// }

// impl<R: VerificarVencimientosProgramados> CasoVerificarVencimientosProximosMembresia<R> {
//   pub fn new(repo: R) -> Self {
//     Self { repo }
//   }

//   pub async fn ejecutar(&self) -> Result<Vec<EstadoMembresia>, ErrorMembresia> {
//     self.repo.verificar_todos().await
//   }
// }

pub struct CasoVerificarMembresiaValidaPorSocio<R: VerificarVencimientosProgramados> {
  repo: R,
}

impl<R: VerificarVencimientosProgramados> CasoVerificarMembresiaValidaPorSocio<R> {
  pub fn new(repo: R) -> Self {
    Self { repo }
  }

  pub async fn ejecutar(&self, id_socio: u64) -> Result<Option<EstadoMembresia>, ErrorMembresia> {
    self.repo.verificar_por_id_socio(id_socio).await
  }
}
