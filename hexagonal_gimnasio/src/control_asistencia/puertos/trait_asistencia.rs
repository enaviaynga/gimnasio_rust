use thiserror::Error;

use crate::control_asistencia::dominio::asistencia::Asistencia;

#[async_trait::async_trait]
pub trait ListarAsistencias {
  async fn listar_asistencias(&self) -> Result<Vec<Asistencia>, ErrorAsistencia>;
}

#[async_trait::async_trait]
pub trait EliminarAsistencia {
  async fn eliminar_asistencia_por_id(&self, id: u64) -> Result<(), ErrorAsistencia>;
}

#[derive(Debug, Error, Clone)]
pub enum ErrorAsistencia {
  #[error("Error general, cambiar a errores especificos a futuro: {0}")]
  General(String),
}
