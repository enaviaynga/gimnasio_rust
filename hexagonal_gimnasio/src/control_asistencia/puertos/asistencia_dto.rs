use chrono::{NaiveDate, NaiveTime};
use rust_decimal::Decimal;

use crate::{
  control_asistencia::dominio::asistencia::ResulValidacion,
  membresias::dominio::membresia::EstadoMembresia,
};

#[derive(Debug)]
pub struct AccesoSocioDTO {
  pub id_asistencia: u32,
  pub nombre_socio: String,
  pub apellido_socio: String,
  pub dni: String,
  pub fecha_asistencia: NaiveDate,
  pub hora_asistencia: NaiveTime,
  pub resultado_validacion: ResulValidacion,
}

#[derive(Clone, PartialEq, Debug)]
pub struct AsistenciaSocioMembresiaDto {
  // Datos del Socio
  pub id_socio: u32,
  pub nombre_s: String,
  pub apellido_s: String,
  pub dni_s: String,
  pub activo: bool,

  // Datos de la Membresía
  pub id_membresia: u32,
  pub tipo_membresia: String,
  pub fecha_inicio: String,
  pub fecha_vencimiento: String,
  pub costo: Decimal,
  pub estado: EstadoMembresia,

  // Datos de la Asistencia Seleccionada
  pub id_asistencia: u32,
  pub fecha_asistencia: String,
  pub hora_ingreso: String,
  pub resultado_validacion: ResulValidacion,
}
