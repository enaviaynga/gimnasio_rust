use std::sync::Arc;

use chrono::NaiveDate;
use rust_decimal::Decimal;

use crate::membresias::dominio::membresia::{EstadoMembresia, TipoMembresia};

#[derive(Clone)]
pub struct MembresiaSocioDto {
  pub id: u32,
  pub id_s: u32,
  pub nombre_s: Arc<str>,
  pub apellido_s: Arc<str>,
  pub dni_s: Arc<str>,
  pub tipo_membresia: TipoMembresia,
  pub fecha_inicio: NaiveDate,
  pub fecha_vencimiento: NaiveDate,
  pub estado: EstadoMembresia,
  pub costo: Decimal,
}
