use sqlx::{MySql, Pool};
use thiserror::Error;

#[derive(Debug, Clone)]
pub struct MySqlxAsistencia {
  pool: Pool<MySql>,
}

impl MySqlxAsistencia {
  pub async fn new(pool: Pool<MySql>) -> Self {
    Self { pool }
  }

  pub fn ref_pool(&self) -> &Pool<MySql> {
    &self.pool
  }
}
