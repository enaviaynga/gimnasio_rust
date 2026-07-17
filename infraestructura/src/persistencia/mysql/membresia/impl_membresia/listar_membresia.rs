use std::sync::Arc;

use chrono::{Local, TimeDelta};
use hexagonal_gimnasio::membresias::{
  dominio::membresia::{EstadoMembresia, Membresia},
  puertos::{
    membresias_dto::MembresiaSocioDto,
    trait_membresia::{ErrorMembresia, ListarMembresia},
  },
};
use sqlx::types::Decimal;

use crate::persistencia::mysql::membresia::membresia_repositorio::MySqlxMembresia;

#[async_trait::async_trait]
impl ListarMembresia for MySqlxMembresia {
  async fn todas_las_membresias(&self) -> Result<Vec<MembresiaSocioDto>, ErrorMembresia> {
    let lista_membresia_bd = sqlx::query!(
      "SELECT
        s.nombre,
        s.apellidos,
        s.dni,
        m.idMembresia,
        m.idsocio,
        m.tipoMembresia,
        m.fechaInicio,
        m.fechaVencimiento,
        m.costo
    FROM
        socios s
    INNER JOIN
        membresias m ON s.idsocio = m.idsocio;"
    )
    .fetch_all(self.ref_pool())
    .await
    .map_err(|e| ErrorMembresia::Otro(format!("{e}")))?;
    let mut lista_membresia_vec = vec![];
    for i in lista_membresia_bd {
      let tipo_membresia = i
        .tipoMembresia
        .parse()
        .map_err(|e| ErrorMembresia::Otro(format!("{e}")))?;
      let dias_restantes = i.fechaVencimiento - Local::now().date_naive();
      let estado = match dias_restantes {
        _ if dias_restantes <= TimeDelta::zero() => EstadoMembresia::Vencido,
        _ if dias_restantes <= TimeDelta::days(4) => EstadoMembresia::CercaVencimiento,
        _ => EstadoMembresia::Activo,
      };

      lista_membresia_vec.push(MembresiaSocioDto {
        id: i.idMembresia,
        id_s: i.idsocio,
        nombre_s: Arc::from(i.nombre),
        apellido_s: Arc::from(i.apellidos),
        dni_s: Arc::from(i.dni),
        tipo_membresia,
        fecha_inicio: i.fechaInicio,
        fecha_vencimiento: i.fechaVencimiento,
        estado,
        costo: i.costo,
      })
    }
    Ok(lista_membresia_vec)
  }
}
