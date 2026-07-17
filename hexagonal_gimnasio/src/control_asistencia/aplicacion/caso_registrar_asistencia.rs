use crate::control_asistencia::{
  dominio::asistencia::ResulValidacion, puertos::trait_asistencia::ErrorAsistencia,
};
use chrono::NaiveDate;

#[async_trait::async_trait]
pub trait RegistrarAsistencia {
  async fn registrar_por_dni(&self, dni: &str) -> Result<(), ErrorAsistencia>;
}

#[derive(Clone)]
pub struct CasoRegistrarAsistencia<R: RegistrarAsistencia> {
  repo: R,
}

impl<R: RegistrarAsistencia> CasoRegistrarAsistencia<R> {
  pub fn new(repo: R) -> Self {
    Self { repo }
  }

  pub async fn ejecutar(&self, dni: &str) -> Result<(), ErrorAsistencia> {
    self.repo.registrar_por_dni(dni).await
  }
}
