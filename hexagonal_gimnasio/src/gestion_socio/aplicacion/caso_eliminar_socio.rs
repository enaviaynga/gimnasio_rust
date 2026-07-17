use crate::gestion_socio::puertos::trait_socios::{EliminarSocio, ErrorSocio};

pub struct CasoEliminarSocio<R: EliminarSocio> {
  repo: R,
}

impl<R: EliminarSocio> CasoEliminarSocio<R> {
  pub fn new(repo: R) -> Self {
    Self { repo }
  }

  pub async fn ejecutar(&self, id: u64) -> Result<(), ErrorSocio> {
    self.repo.eliminar_por_id(id).await
  }
}
