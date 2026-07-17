use crate::control_asistencia::puertos::{
  asistencia_dto::AsistenciaSocioMembresiaDto, trait_asistencia::ErrorAsistencia,
};

#[async_trait::async_trait]
pub trait ObtenerInfoAsistenciaMembresiaSocio {
  async fn buscar_info_completa_asistencia(
    &self,
    id_asistencia: u64,
  ) -> Result<AsistenciaSocioMembresiaDto, ErrorAsistencia>;
}

#[derive(Clone)]
pub struct CasoObtenerInfoAsistenciaCompleta<R: ObtenerInfoAsistenciaMembresiaSocio> {
  repo: R,
}

impl<R: ObtenerInfoAsistenciaMembresiaSocio> CasoObtenerInfoAsistenciaCompleta<R> {
  pub fn new(repo: R) -> Self {
    Self { repo }
  }

  pub async fn ejecutar(
    &self,
    id_asistencia: u64,
  ) -> Result<AsistenciaSocioMembresiaDto, ErrorAsistencia> {
    self
      .repo
      .buscar_info_completa_asistencia(id_asistencia)
      .await
  }
}
