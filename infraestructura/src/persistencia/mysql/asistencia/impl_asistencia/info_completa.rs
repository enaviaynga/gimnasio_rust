use crate::persistencia::mysql::asistencia::asistencia_repositorio::MySqlxAsistencia;
use chrono::{NaiveDate, Utc};
use hexagonal_gimnasio::{
  control_asistencia::{
    aplicacion::caso_obtener_info_asistencia_socio::ObtenerInfoAsistenciaMembresiaSocio,
    dominio::asistencia::ResulValidacion,
    puertos::{asistencia_dto::AsistenciaSocioMembresiaDto, trait_asistencia::ErrorAsistencia},
  },
  membresias::dominio::membresia::EstadoMembresia,
};
use sqlx::query_as;

#[async_trait::async_trait]
impl ObtenerInfoAsistenciaMembresiaSocio for MySqlxAsistencia {
  async fn buscar_info_completa_asistencia(
    &self,
    id_asistencia: u64,
  ) -> Result<AsistenciaSocioMembresiaDto, ErrorAsistencia> {
    // Realizamos el JOIN entre las 3 tablas.
    // Nota: Ajusta los nombres de las columnas si difieren de las del DTO.
    // Se asume que el tipo de fecha en SQL se mapea a String.
    let resultado = sqlx::query!(
      r#"
            SELECT
                s.idsocio, s.nombre, s.apellidos, s.dni, s.activo,
                m.idMembresia, m.tipoMembresia, m.fechaInicio, m.fechaVencimiento, m.costo,
                a.idAsistencia, a.fecha, a.horaIngreso, a.resultadoValidacion
            FROM asistencia a
            INNER JOIN socios s ON a.idsocio = s.idsocio
            INNER JOIN membresias m ON s.idsocio = m.idsocio
            WHERE a.idAsistencia = ?
            LIMIT 1
            "#,
      id_asistencia
    )
    .fetch_one(self.ref_pool())
    .await
    .map_err(|e| ErrorAsistencia::General(format!("Error al buscar asistencia: {}", e)))?;

    let fecha_ven = resultado.fechaVencimiento;

    let fecha_asist =
      NaiveDate::parse_from_str(&resultado.fecha.to_string(), "%Y-%m-%d").map_err(|_| {
        ErrorAsistencia::General("Formato de fecha de asistencia inválido".to_string())
      })?;

    // 2. Definir lógica de estado
    let dias_para_vencimiento = (fecha_ven - fecha_asist).num_days();
    let margen_cerca_vencimiento = 3; // Ajusta este valor a tu gusto

    let estado_calculado = if !resultado.activo != 0 || fecha_asist > fecha_ven {
      EstadoMembresia::Vencido
    } else if dias_para_vencimiento <= margen_cerca_vencimiento {
      EstadoMembresia::CercaVencimiento
    } else {
      EstadoMembresia::Activo
    };

    Ok(AsistenciaSocioMembresiaDto {
      id_socio: resultado.idsocio as u32,
      nombre_s: resultado.nombre,
      apellido_s: resultado.apellidos,
      dni_s: resultado.dni,
      activo: resultado.activo != 0, // MySQL boolean es tinyint(1)

      id_membresia: resultado.idMembresia as u32,
      tipo_membresia: resultado.tipoMembresia,
      fecha_inicio: resultado.fechaInicio.to_string(),
      fecha_vencimiento: resultado.fechaVencimiento.to_string(),
      costo: resultado.costo,
      estado: estado_calculado,

      id_asistencia: resultado.idAsistencia,
      fecha_asistencia: resultado.fecha.to_string(),
      hora_ingreso: resultado.horaIngreso.to_string(),
      resultado_validacion: resultado
        .resultadoValidacion
        .parse::<ResulValidacion>()
        .map_err(|_| ErrorAsistencia::General("Estado de validación desconocido".to_string()))?,
    })
  }
}
