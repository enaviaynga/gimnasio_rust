use std::num::NonZeroU64;

use chrono::{NaiveDate, NaiveDateTime, NaiveTime};
use derive_more::{Display, FromStr};

pub struct Asistencia {
  id: Option<NonZeroU64>,
  id_socio: u64,
  fecha_hora: NaiveDateTime,
  resultado_validacion: ResulValidacion,
}

impl Asistencia {
  pub fn new(
    id_socio: u64,
    fecha: NaiveDate,
    hora: NaiveTime,
    resultado_validacion: ResulValidacion,
  ) -> Self {
    let fecha_hora = fecha.and_time(hora);
    Self {
      id: None,
      id_socio,
      fecha_hora,
      resultado_validacion,
    }
  }

  pub fn existente(
    id: u64,
    id_socio: u64,
    fecha: NaiveDate,
    hora: NaiveTime,
    resultado_validacion: ResulValidacion,
  ) -> Self {
    let fecha_hora = fecha.and_time(hora);
    Self {
      id: NonZeroU64::new(id),
      id_socio,
      fecha_hora,
      resultado_validacion,
    }
  }

  pub fn get_id(&self) -> Option<u64> {
    self.id.map(|id| id.get())
  }

  pub fn get_id_socio(&self) -> u64 {
    self.id_socio
  }

  pub fn get_fecha(&self) -> NaiveDate {
    self.fecha_hora.date()
  }

  pub fn get_hora(&self) -> NaiveTime {
    self.fecha_hora.time()
  }

  pub fn get_validacion(&self) -> ResulValidacion {
    self.resultado_validacion
  }
}

#[derive(Debug, Clone, Copy, FromStr, Display, PartialEq)]
pub enum ResulValidacion {
  Permitido,
  NoPermitido,
}
