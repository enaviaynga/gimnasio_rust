use std::sync::Arc;

use crate::gestion_socio::{
  dominio::socio::Socio,
  puertos::trait_socios::{BuscarSocio, ErrorSocio},
};

#[derive(Clone)]
pub struct CasoBuscarSocioPorId<R: BuscarSocio> {
  repo: R,
}

impl<R: BuscarSocio> CasoBuscarSocioPorId<R> {
  pub fn new(repo: R) -> Self {
    Self { repo }
  }

  pub async fn ejecutar(&self, id: u64) -> Result<Option<Socio>, ErrorSocio> {
    self.repo.socio_por_id(id).await
  }
}

pub struct CasoBuscarPorFiltro<R: BuscarSocio> {
  repo: R,
}

impl<R: BuscarSocio> CasoBuscarPorFiltro<R> {
  pub fn new(repo: R) -> Self {
    Self { repo }
  }

  pub async fn ejecutar(&self, filtro: FiltroSocio) -> Result<Vec<Socio>, ErrorSocio> {
    match filtro {
      FiltroSocio::Nombre(valor) => self.repo.socio_por_nombre(&valor).await,
      FiltroSocio::Apellido(valor) => self.repo.socio_por_apellidos(&valor).await,
      FiltroSocio::Dni(valor) => self
        .repo
        .socio_por_dni(&valor)
        .await
        .map(|ok| ok.into_iter().collect()),
      FiltroSocio::Telefono(valor) => self.repo.socio_por_telefono(&valor).await,
      FiltroSocio::Correo(valor) => self.repo.socio_por_correo(&valor).await,
      FiltroSocio::Direccion(valor) => self.repo.socio_por_direcion(&valor).await,
    }
  }
}

#[derive(Clone)]
pub struct CasoBuscarSocioPorDni<R: BuscarSocio> {
  repo: R,
}

impl<R: BuscarSocio> CasoBuscarSocioPorDni<R> {
  pub fn new(repo: R) -> Self {
    Self { repo }
  }

  pub async fn ejecutar(&self, dni: Arc<str>) -> Result<Option<Socio>, ErrorSocio> {
    self.repo.socio_por_dni(&dni).await
  }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FiltroSocio {
  Nombre(Arc<str>),
  Apellido(Arc<str>),
  Dni(Arc<str>),
  Telefono(Arc<str>),
  Correo(Arc<str>),
  Direccion(Arc<str>),
}
