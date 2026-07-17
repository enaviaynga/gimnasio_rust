use crate::control_asistencia::{
  dominio::asistencia::ResulValidacion,
  puertos::{asistencia_dto::AccesoSocioDTO, trait_asistencia::ErrorAsistencia},
};
use chrono::NaiveDate;

#[async_trait::async_trait]
pub trait ObtenerUnaAsistencia {
  async fn obtener_asistencia_por_dni(
    &self,
    dni: &str,
  ) -> Result<Vec<AccesoSocioDTO>, ErrorAsistencia>;
}

#[derive(Clone)]
pub struct CasoObtenerUnaAsistencia<R: ObtenerUnaAsistencia> {
  repo: R,
}

impl<R: ObtenerUnaAsistencia> CasoObtenerUnaAsistencia<R> {
  pub fn new(repo: R) -> Self {
    Self { repo }
  }

  pub async fn ejecutar(&self, dni: &str) -> Result<Vec<AccesoSocioDTO>, ErrorAsistencia> {
    self.repo.obtener_asistencia_por_dni(dni).await
  }
}

#[async_trait::async_trait]
pub trait ListarAsistenciaDto {
  async fn obtener_listar_asistencias(&self) -> Result<Vec<AccesoSocioDTO>, ErrorAsistencia>;
}

#[derive(Clone)]
pub struct CasoListarAsistenciaDto<R: ListarAsistenciaDto> {
  repo: R,
}

impl<R: ListarAsistenciaDto> CasoListarAsistenciaDto<R> {
  pub fn new(repo: R) -> Self {
    Self { repo }
  }

  pub async fn ejecutar(&self) -> Result<Vec<AccesoSocioDTO>, ErrorAsistencia> {
    self.repo.obtener_listar_asistencias().await
  }
}
