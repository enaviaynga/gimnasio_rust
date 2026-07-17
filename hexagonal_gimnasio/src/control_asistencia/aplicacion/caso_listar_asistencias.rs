use crate::control_asistencia::{
  dominio::asistencia::Asistencia,
  puertos::trait_asistencia::{ErrorAsistencia, ListarAsistencias},
};

#[derive(Clone)]
pub struct CasoListarAsistencia<R: ListarAsistencias> {
  repo: R,
}

impl<R: ListarAsistencias> CasoListarAsistencia<R> {
  pub fn new(repo: R) -> Self {
    Self { repo }
  }

  pub async fn ejecutar(&self) -> Result<Vec<Asistencia>, ErrorAsistencia> {
    self.repo.listar_asistencias().await
  }
}
