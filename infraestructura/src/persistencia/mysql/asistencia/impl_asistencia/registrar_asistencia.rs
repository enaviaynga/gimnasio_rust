use chrono::{Local, NaiveDate, NaiveDateTime, TimeDelta};
use hexagonal_gimnasio::control_asistencia::{
  aplicacion::caso_registrar_asistencia::RegistrarAsistencia, dominio::asistencia::ResulValidacion,
  puertos::trait_asistencia::ErrorAsistencia,
};

use crate::persistencia::mysql::asistencia::asistencia_repositorio::MySqlxAsistencia;

// En infraestructura/mysql/asistencia/impl_query.rs
#[async_trait::async_trait]
impl RegistrarAsistencia for MySqlxAsistencia {
  async fn registrar_por_dni(&self, dni: &str) -> Result<(), ErrorAsistencia> {
    let ahora = Local::now();
    let fecha_actual = ahora.date_naive();
    let hora_actual = ahora.time();

    let socio = sqlx::query!(
      "SELECT idsocio, nombre, apellidos FROM socios WHERE dni = ? LIMIT 1;",
      dni
    )
    .fetch_optional(self.ref_pool())
    .await
    .map_err(|e| ErrorAsistencia::General(e.to_string()))?;

    let socio = match socio {
      Some(s) => s,
      None => {
        return Err(ErrorAsistencia::General(format!(
          "No se encontró ningún socio con el DNI: {dni}"
        )));
      }
    };

    let membresia = sqlx::query!(
      "SELECT tipoMembresia, fechaVencimiento
        FROM membresias
        WHERE idsocio = ?
        ORDER BY fechaVencimiento DESC
        LIMIT 1;",
      socio.idsocio
    )
    .fetch_optional(self.ref_pool())
    .await
    .map_err(|e| ErrorAsistencia::General(e.to_string()))?;

    // 3. Evaluamos la regla de negocio
    let resultado_validacion = if let Some(m) = membresia
      && m.fechaVencimiento - fecha_actual > TimeDelta::zero()
    {
      ResulValidacion::Permitido
    } else {
      ResulValidacion::NoPermitido
    };

    let resultado_str = format!("{:?}", resultado_validacion);

    sqlx::query!(
      "INSERT INTO asistencia (idsocio, fecha, horaIngreso, resultadoValidacion)
        VALUES (?, ?, ?, ?);",
      socio.idsocio,
      fecha_actual,
      hora_actual,
      resultado_str
    )
    .execute(self.ref_pool())
    .await
    .map_err(|e| ErrorAsistencia::General(e.to_string()))?;

    Ok(())
  }
}

// async fn obtener_detalles_por_dni(
//   &self,
//   dni: &str,
// ) -> Result<Option<VerificacionAccesoDTO>, ErrorAsistencia> {
//   let ahora = Local::now();
//   let fecha_actual = ahora.date_naive();
//   let hora_actual = ahora.time();

//   let verificar_dto_bd = sqlx::query!(
//     "SELECT
//               s.idsocio,
//               s.nombre,
//               s.apellidos,
//               m.tipoMembresia AS tipoPlanMembresia,
//               m.fechaVencimiento
//           FROM socios s
//           INNER JOIN membresias m ON s.idsocio = m.idsocio
//           WHERE s.dni = ?
//           ORDER BY m.fechaVencimiento DESC;",
//     dni
//   )
//   .fetch_optional(self.ref_pool())
//   .await
//   .map_err(|e| ErrorAsistencia::General(e.to_string()))?;

//   let registro = match verificar_dto_bd {
//     Some(r) => r,
//     None => return Ok(None),
//   };

//   tracing::debug!("ejecutando sin errores la query para asistencia");

//   let resultado_validacion = if registro.fechaVencimiento - fecha_actual > TimeDelta::zero() {
//     ResulValidacion::Permitido
//   } else {
//     ResulValidacion::NoPermitido
//   };

//   let resultado_str = format!("{:?}", resultado_validacion);

//   sqlx::query!(
//     "INSERT INTO asistencia (idsocio, fecha, horaIngreso, resultadoValidacion)
//            VALUES (?, ?, ?, ?);",
//     registro.idsocio,
//     fecha_actual,
//     hora_actual,
//     resultado_str
//   )
//   .execute(self.ref_pool())
//   .await
//   .map_err(|e| ErrorAsistencia::General(e.to_string()))?;

//   let asistencia_dto = VerificacionAccesoDTO {
//     nombre_socio: registro.nombre,
//     apellido_socio: registro.apellidos,
//     dni: dni.to_string(),
//     fecha_vencimiento: registro.fechaVencimiento,
//     resultado_validacion,
//   };

//   Ok(Some(asistencia_dto))
// }
