use thiserror::Error;

use crate::hexagonal::empleado::dominio::empleado::Empleado;
use crate::hexagonal::empleado::dominio::usuario::{self, ErrorUsuario, Usuario};

pub(crate) trait BuscarEmpleadoPorUsuario {
    async fn login(&self, usuario: Usuario) -> Result<Empleado, ErrorMySqlEmpleado>;
}

#[derive(Debug, Error)]
pub(crate) enum ErrorMySqlEmpleado {
    #[error("No existe en la base de datos")]
    NoExiste,
    #[error("Error en la peticion")]
    ErrorPeticion,
    #[error("Valor invalido")]
    ValorInvalido,
}
