use std::{
  cell::OnceCell,
  num::NonZeroU64,
  sync::{Mutex, OnceLock},
};

use chrono::{Months, NaiveDate, TimeDelta};
use derive_more::{Display, FromStr};

static NUMERO: Mutex<u8> = Mutex::new(8);

#[derive(Debug, Clone)]
pub struct Membresia {
  id: Option<NonZeroU64>,
  id_socio: u64,
  tipo_membresia: TipoMembresia,
  costo: rust_decimal::Decimal,
  fecha_inicio: NaiveDate,
  fecha_final: NaiveDate,
}

impl Membresia {
  pub fn new(
    id_socio: u64,
    tipo_membresia: TipoMembresia,
    fecha_inicio: NaiveDate,
    costo: rust_decimal::Decimal,
  ) -> Self {
    let fecha_final = fecha_inicio + Months::new(tipo_membresia as u32);
    Self {
      id: None,
      id_socio,
      tipo_membresia,
      costo,
      fecha_inicio,
      fecha_final,
    }
  }

  pub fn existente(
    id: u64,
    id_socio: u64,
    tipo_membresia: TipoMembresia,
    fecha_inicio: NaiveDate,
    fecha_final: NaiveDate,
    costo: rust_decimal::Decimal,
  ) -> Self {
    Self {
      id: NonZeroU64::new(id),
      id_socio,
      tipo_membresia,
      costo,
      fecha_inicio,
      fecha_final,
    }
  }

  pub fn renovacion_automatica(membresia: Self) -> Self {
    let fecha_inicio_nuevo = membresia.fecha_inicio + Months::new(membresia.tipo_membresia as u32);
    Self {
      id: None,
      fecha_inicio: fecha_inicio_nuevo,
      ..membresia
    }
  }

  pub fn renovacion_desfazada(membresia: Self, fecha_hoy: NaiveDate) -> Self {
    Self {
      id: None,
      fecha_inicio: fecha_hoy,
      ..membresia
    }
  }

  pub fn renovacion_manual(
    mut self,
    fecha_elegida: NaiveDate,
    tipo_membresia: TipoMembresia,
  ) -> Self {
    self.id = None;
    self.tipo_membresia = tipo_membresia;
    self.fecha_inicio = fecha_elegida;
    self
  }

  pub fn get_id(&self) -> Option<u64> {
    self.id.map(|id| id.get())
  }

  pub fn get_id_socio(&self) -> u64 {
    self.id_socio
  }

  pub fn get_membresia(&self) -> TipoMembresia {
    self.tipo_membresia
  }

  pub fn get_fecha_inicio(&self) -> NaiveDate {
    self.fecha_inicio
  }

  pub fn get_fecha_final(&self) -> NaiveDate {
    self.fecha_final
  }

  pub fn get_costo(&self) -> rust_decimal::Decimal {
    self.costo
  }

  pub fn get_activo(&self, fecha_hoy: NaiveDate) -> EstadoMembresia {
    if self.fecha_final - fecha_hoy >= TimeDelta::zero() {
      EstadoMembresia::Activo
    } else {
      EstadoMembresia::Vencido
    }
  }

  pub fn get_proximo_a_vencer(&self, fecha_hoy: NaiveDate, dias_limite: u16) -> EstadoMembresia {
    let dias_restantes = self.fecha_final - fecha_hoy;
    match dias_restantes {
      _ if dias_restantes <= TimeDelta::zero() => EstadoMembresia::Vencido,
      _ if dias_restantes <= TimeDelta::days(4) => EstadoMembresia::CercaVencimiento,
      _ => EstadoMembresia::Activo,
    }
  }

  /// se considera que a 4 dias es proximo a vencer
  pub fn get_proximo_a_vencer_default(&self, fecha_hoy: NaiveDate) -> EstadoMembresia {
    let dias_restantes = self.fecha_final - fecha_hoy;
    if dias_restantes <= TimeDelta::zero() {
      EstadoMembresia::Vencido
    } else if dias_restantes <= TimeDelta::days(4) {
      EstadoMembresia::CercaVencimiento
    } else {
      EstadoMembresia::Activo
    }
  }
}

#[derive(Debug, Clone, Copy, FromStr, Display)]
pub enum TipoMembresia {
  Mensual = 1,
  Bimestral = 2,
  Trimestral = 3,
  Semestral = 6,
  Anual = 12,
}

#[derive(Debug, Clone, Copy, Display, FromStr, PartialEq)]
pub enum EstadoMembresia {
  Activo, // vigente
  CercaVencimiento,
  Vencido,
}
