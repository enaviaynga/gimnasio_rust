use sqlx::{MySql, Pool};
use thiserror::Error;
use tokio::time::error;

#[derive(Debug, Clone)]
pub(crate) struct BDsqlx {
    bd: Pool<MySql>,
}

impl BDsqlx {
    pub(crate) async fn new(bd_url: &str) -> Result<Self, ErrorMySql> {
        Ok(Self {
            bd: sqlx::MySqlPool::connect(bd_url).await.map_err(|e| {
                println!("{e}");
                ErrorMySql::ErroBbInvalida
            })?,
        })
    }

    pub(crate) fn ref_pool(&self) -> &Pool<MySql> {
        &self.bd
    }
}
#[derive(Debug, Error)]
pub(crate) enum ErrorMySql {
    #[error("Base de datos invalida")]
    ErroBbInvalida,
    #[error("Error de coneccion")]
    ErrorConeccion,
}
