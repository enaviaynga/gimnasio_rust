// adquirir_membresia(id_socio, id_plan):
// Registra la compra, calcula fechas de inicio/fin y emite el estado inicial.

// verificar_vencimientos_programados():
// Lógica (CRON o servicio periódico) que evalúa las fechas límite de
// hoy y cambia los estados expirados.

use thiserror::Error;

use crate::membresias::{
  dominio::membresia::{EstadoMembresia, Membresia},
  puertos::membresias_dto::MembresiaSocioDto,
};

#[async_trait::async_trait]
pub trait RegistrarNuevaMembresia {
  async fn registrar_en_db(&self, membresia: Membresia) -> Result<(), ErrorMembresia>;
}

#[async_trait::async_trait]
pub trait VerificarVencimientosProgramados {
  fn verificar_una_membresia(&self, membresia: &Membresia) -> EstadoMembresia;
  async fn verificar_por_id_socio(
    &self,
    id_socio: u64,
  ) -> Result<Option<EstadoMembresia>, ErrorMembresia>;
  // lo estoy pensando solo para estadistico, no para mostrar al usuario
  // async fn verificar_todos(&self) -> Result<Vec<EstadoMembresia>, ErrorMembresia>;
}

#[async_trait::async_trait]
pub trait BuscarMembresia {
  async fn membresia_por_dni_socio(
    &self,
    dni: String,
    dias_consideracion: Option<u16>,
  ) -> Result<Option<MembresiaSocioDto>, ErrorMembresia>;
}

#[async_trait::async_trait]
pub trait BuscarTodasMembresias {
  async fn membresia_por_id_socio(&self, id_socio: u64) -> Result<Vec<Membresia>, ErrorMembresia>;
}

#[async_trait::async_trait]
pub trait ListarMembresia {
  async fn todas_las_membresias(&self) -> Result<Vec<MembresiaSocioDto>, ErrorMembresia>;
}

#[derive(Debug, Error)]
pub enum ErrorMembresia {
  #[error("Error aun no definido: {0}")]
  Otro(String),
}
