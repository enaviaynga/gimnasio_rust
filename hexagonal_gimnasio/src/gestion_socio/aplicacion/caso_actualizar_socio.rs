use crate::gestion_socio::{
  dominio::socio::Socio,
  puertos::trait_socios::{ActualizarSocio, ErrorSocio},
};

#[derive(Clone)]
pub struct CasoActualizarSocio<R: ActualizarSocio> {
  repo: R,
}

impl<R: ActualizarSocio> CasoActualizarSocio<R> {
  pub fn new(repo: R) -> Self {
    Self { repo }
  }

  pub async fn ejecutar(&self, datos: Socio) -> Result<(), ErrorSocio> {
    self.repo.actualizar_datos_de_socio(datos).await
  }
}
