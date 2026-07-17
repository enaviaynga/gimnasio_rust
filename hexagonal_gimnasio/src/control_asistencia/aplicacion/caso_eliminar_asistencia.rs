use crate::control_asistencia::{
  dominio::asistencia::{self, Asistencia},
  puertos::trait_asistencia::{EliminarAsistencia, ErrorAsistencia},
};

#[derive(Clone)]
pub struct CasoEliminarAsistencia<R: EliminarAsistencia> {
  repo: R,
}

impl<R: EliminarAsistencia> CasoEliminarAsistencia<R> {
  pub fn new(repo: R) -> Self {
    Self { repo }
  }

  pub async fn ejecutar(&self, id: u64) -> Result<(), ErrorAsistencia> {
    self.repo.eliminar_asistencia_por_id(id).await
  }
}
