use sqlx::{MySql, Pool};
use thiserror::Error;

#[derive(Debug, Clone)]
pub struct MySqlxMembresia {
  pool: Pool<MySql>,
}

impl MySqlxMembresia {
  pub async fn new(pool: Pool<MySql>) -> Self {
    Self { pool }
  }

  pub fn ref_pool(&self) -> &Pool<MySql> {
    &self.pool
  }
}
