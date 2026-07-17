use thiserror::Error;

use crate::gestion_socio::dominio::socio::{Socio, SocioInvalido};

#[async_trait::async_trait]
pub trait BuscarSocio {
  async fn socio_por_id(&self, id: u64) -> Result<Option<Socio>, ErrorSocio>;
  async fn socio_por_nombre(&self, nombre: &str) -> Result<Vec<Socio>, ErrorSocio>;
  async fn socio_por_apellidos(&self, apellido: &str) -> Result<Vec<Socio>, ErrorSocio>;
  async fn socio_por_dni(&self, dni: &str) -> Result<Option<Socio>, ErrorSocio>;
  async fn socio_por_telefono(&self, telefono: &str) -> Result<Vec<Socio>, ErrorSocio>;
  async fn socio_por_correo(&self, correo: &str) -> Result<Vec<Socio>, ErrorSocio>;
  async fn socio_por_direcion(&self, direccion: &str) -> Result<Vec<Socio>, ErrorSocio>;
}

#[async_trait::async_trait]
pub trait BuscarSocios {
  // pendiente añadir result
  async fn socios_por_id(&self, id: u64) -> Vec<Socio>;
  async fn socios_por_nombre(&self, nombre: &str) -> Vec<Socio>;
  async fn socios_por_apellidos(&self, apellido: &str) -> Vec<Socio>;
  async fn socios_por_dni(&self, dni: &str) -> Vec<Socio>;
  // async fn socios_por_telefono(&self,telefono: &str) -> Vec<Socio>;
  async fn socios_por_correo(&self, correo: &str) -> Vec<Socio>;
  // async fn socios_por_direcion(&self,direccion: &str) -> Vec<Socio>;
}

#[async_trait::async_trait]
pub trait ListarSocios {
  async fn listar(&self) -> Result<Vec<Socio>, ErrorSocio>;
}

#[async_trait::async_trait]
pub trait RegistrarSocio {
  async fn registrar_socio(&self, socio: Socio) -> Result<(), ErrorSocio>;
}

#[async_trait::async_trait]
pub trait ActualizarSocio {
  async fn actualizar_datos_de_socio(&self, datos: Socio) -> Result<(), ErrorSocio>;
}

#[async_trait::async_trait]
pub trait EliminarSocio {
  async fn eliminar_por_id(&self, id: u64) -> Result<(), ErrorSocio>;
}

#[derive(Debug, Error, Clone, Copy)]
pub enum ErrorSocio {
  #[error("Error en la generacion de Socio: {0}")]
  ErrorEnGenerar(SocioInvalido),
  // quitar a futuro este error porque es un comodin
  #[error("Otro error no determinado")]
  OtroError,
}
