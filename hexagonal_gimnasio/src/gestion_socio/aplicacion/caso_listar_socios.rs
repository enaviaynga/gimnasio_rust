use crate::gestion_socio::{
  dominio::socio::Socio,
  puertos::trait_socios::{ErrorSocio, ListarSocios},
};

#[derive(Debug, Clone)]
pub struct CasoBuscarSocioPorFiltro<R: ListarSocios> {
  repo: R,
}

impl<R: ListarSocios> CasoBuscarSocioPorFiltro<R> {
  pub fn new(repo: R) -> Self {
    Self { repo }
  }

  pub async fn ejecutar(&self) -> Result<Vec<Socio>, ErrorSocio> {
    self.repo.listar().await
  }
}
