use crate::gestion_socio::{
  dominio::socio::Socio,
  puertos::trait_socios::{ErrorSocio, RegistrarSocio},
};

// aqui se añade la logica para generar reporte
#[derive(Clone)]
pub struct CasoRegistrarSocio<R: RegistrarSocio> {
  repo: R,
  // generador_reporte: <G: GenerarReporteRegistro>
}

impl<R: RegistrarSocio> CasoRegistrarSocio<R> {
  pub fn new(repo: R) -> Self {
    Self { repo }
  }

  pub async fn ejecutar(&self, socio: Socio) -> Result<(), ErrorSocio> {
    self.repo.registrar_socio(socio).await
    // aqui se añade la generacion, se puede inspect para ejecutarlo solo si es Ok()
  }
}
