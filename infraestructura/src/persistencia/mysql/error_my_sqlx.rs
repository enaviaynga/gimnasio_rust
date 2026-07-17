use thiserror::Error;

#[derive(Debug, Error)]
pub enum ErrorMySql {
  #[error("Base de datos invalida")]
  ErroBbInvalida,
  #[error("Error de coneccion")]
  ErrorConeccion,
}
