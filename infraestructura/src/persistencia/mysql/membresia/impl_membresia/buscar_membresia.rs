use std::sync::Arc;

use chrono::{Local, TimeDelta};
use hexagonal_gimnasio::membresias::{
  dominio::membresia::{EstadoMembresia, Membresia},
  puertos::{
    membresias_dto::MembresiaSocioDto,
    trait_membresia::{BuscarMembresia, ErrorMembresia},
  },
};

use crate::persistencia::mysql::membresia::membresia_repositorio::MySqlxMembresia;

/*
#[async_trait::async_trait]
impl BuscarMembresia for MySqlxMembresia {
    async fn membresia_por_id_socio(
        &self,
        id_socio: u64,
    ) -> Result<Option<Membresia>, ErrorMembresia> {
        let membresia_bd = sqlx::query!(
            "SELECT idMembresia, tipoMembresia, fechaInicio, fechaVencimiento, costo
            FROM membresias
            WHERE idsocio = ?
              AND CURDATE() BETWEEN fechaInicio AND fechaVencimiento
            LIMIT 1;",
            id_socio
        )
        .fetch_one(self.ref_pool())
        .await;
        if let Err(sqlx::Error::RowNotFound) = membresia_bd {
            return Ok(None);
        }
        let membresia_bd = membresia_bd.map_err(|e| ErrorMembresia::Otro(e.to_string()))?;

        let membresias = Membresia::existente(
            membresia_bd.idMembresia as u64,
            id_socio,
            membresia_bd
                .tipoMembresia
                .parse()
                .map_err(|e| ErrorMembresia::Otro(format!("{e}")))?,
            membresia_bd.fechaInicio,
            membresia_bd.fechaVencimiento,
            membresia_bd.costo,
        );

        Ok(Some(membresias))
    }
} // */
//*
#[async_trait::async_trait]
impl BuscarMembresia for MySqlxMembresia {
  async fn membresia_por_dni_socio(
    &self,
    dni: String,
    dias_consideracion: Option<u16>,
  ) -> Result<Option<MembresiaSocioDto>, ErrorMembresia> {
    let membresia_bd = sqlx::query!(
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
          membresias m ON s.idsocio = m.idsocio
      WHERE
          s.dni = ?
      ORDER BY
          m.fechaInicio DESC, m.idMembresia DESC
      LIMIT 1;",
      dni
    )
    .fetch_one(self.ref_pool())
    .await
    .map_err(|e| ErrorMembresia::Otro(format!("{e}")))?;
    let tipo_membresia = membresia_bd
      .tipoMembresia
      .parse()
      .map_err(|e| ErrorMembresia::Otro(format!("{e}")))?;
    let dias_restantes = membresia_bd.fechaVencimiento - Local::now().date_naive();
    let estado = match dias_restantes {
      _ if dias_restantes <= TimeDelta::zero() => EstadoMembresia::Vencido,
      _ if dias_restantes <= TimeDelta::days(dias_consideracion.unwrap_or_default() as i64) => {
        EstadoMembresia::CercaVencimiento
      }
      _ => EstadoMembresia::Activo,
    };

    Ok(Some(MembresiaSocioDto {
      id: membresia_bd.idMembresia,
      id_s: membresia_bd.idsocio,
      nombre_s: Arc::from(membresia_bd.nombre),
      apellido_s: Arc::from(membresia_bd.apellidos),
      dni_s: Arc::from(membresia_bd.dni),
      tipo_membresia,
      fecha_inicio: membresia_bd.fechaInicio,
      fecha_vencimiento: membresia_bd.fechaVencimiento,
      estado,
      costo: membresia_bd.costo,
    }))
  }
}
// */
