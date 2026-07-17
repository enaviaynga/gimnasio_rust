use std::sync::Arc;

use sqlx::{MySql, Pool};

use crate::persistencia::mysql::{
  asistencia::asistencia_repositorio::MySqlxAsistencia,
  empleado::empleado_repository::MySqlxEmpleado, membresia::membresia_repositorio::MySqlxMembresia,
  socio::socio_repository::MySqlxSocio,
};

#[derive(Debug, Clone)]
pub struct ContenedorRepos {
  pub empleado_repo: Arc<MySqlxEmpleado>,
  pub socio_repo: Arc<MySqlxSocio>,
  pub membresia_repo: Arc<MySqlxMembresia>,
  pub asistencia_repo: Arc<MySqlxAsistencia>,
}

impl ContenedorRepos {
  pub async fn new(pool: Pool<MySql>) -> Self {
    let empleado_repo = MySqlxEmpleado::new(pool.clone()).await;
    let socio_repo = MySqlxSocio::new(pool.clone()).await;
    let membresia_repo = MySqlxMembresia::new(pool.clone()).await;
    let asistencia_repo = MySqlxAsistencia::new(pool.clone()).await;
    Self {
      empleado_repo: Arc::new(empleado_repo),
      socio_repo: Arc::new(socio_repo),
      membresia_repo: Arc::new(membresia_repo),
      asistencia_repo: Arc::new(asistencia_repo),
    }
  }
}
