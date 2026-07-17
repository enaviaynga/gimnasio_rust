use std::error;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum ErrorRepositorioEmpleado {
  #[error("No se encontro el empleado en el sistema")]
  NoExiste,
  #[error("Error en la peticion")]
  ErrorPeticion,
  #[error("Se intento procesar un valor invalido")]
  ValorInvalido,
  #[error("La contraseña no es valida")]
  ContraseñaInvalida,
}
