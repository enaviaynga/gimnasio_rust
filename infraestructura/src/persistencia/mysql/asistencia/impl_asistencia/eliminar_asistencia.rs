use hexagonal_gimnasio::control_asistencia::puertos::trait_asistencia::{
  EliminarAsistencia, ErrorAsistencia,
};
use sqlx::query;

use crate::{
  persistencia::mysql::asistencia::asistencia_repositorio::MySqlxAsistencia,
  reportes::eliminacion::rprt_elim_asistencia::generar_reporte_previo_eliminacion_unica,
};

#[async_trait::async_trait]
impl EliminarAsistencia for MySqlxAsistencia {
  async fn eliminar_asistencia_por_id(&self, id: u64) -> Result<(), ErrorAsistencia> {
    // let ruta_pdf = generar_reporte_previo_eliminacion_unica(self.ref_pool(), id as u32)
    //   .await
    //   .map_err(|e| ErrorAsistencia::General(e.to_string()))?;
    query!(
      "DELETE FROM asistencia
        WHERE idAsistencia = ?;",
      id
    )
    .execute(self.ref_pool())
    .await
    .map_err(|e| ErrorAsistencia::General(e.to_string()))?;

    // if let Err(e) = opener::open(&ruta_pdf) {
    //   // Un fallo al abrir el visor de PDF del sistema operativo no debería tirar abajo
    //   // la transacción/operación si ya se borró de la BD, pero lo registramos en logs.
    //   tracing::error!(
    //     "El registro se eliminó pero no se pudo abrir el visor de PDF: {}",
    //     e
    //   );
    //   Err(ErrorAsistencia::General(
    //     "Error al abrir el reporte pdf con el programa default".to_string(),
    //   ))
    // } else {
    //   tracing::info!("¡Proceso completado! Registro eliminado de la BD y reporte abierto.");
    //   Ok(())
    // }
    Ok(())
  }
}
