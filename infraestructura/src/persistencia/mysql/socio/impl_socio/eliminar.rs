use crate::{
  persistencia::mysql::socio::socio_repository::MySqlxSocio,
  reportes::eliminacion::rpty_elim_socio::generar_reporte_previo_eliminacion_socio,
};
use hexagonal_gimnasio::gestion_socio::puertos::trait_socios::{EliminarSocio, ErrorSocio};

#[async_trait::async_trait]
impl EliminarSocio for MySqlxSocio {
  async fn eliminar_por_id(&self, id: u64) -> Result<(), ErrorSocio> {
    // let ruta_pdf = generar_reporte_previo_eliminacion_socio(self.ref_pool(), id as u32)
    //   .await
    //   .map_err(|e| {
    //     tracing::warn!("{e}");
    //     ErrorSocio::OtroError
    //   })?;
    sqlx::query!("UPDATE socios SET activo = ? WHERE idsocio = ?;", false, id)
      .execute(self.ref_pool())
      .await
      .map_err(|e| {
        tracing::debug!("Error al buscar por id: {e}");
        ErrorSocio::OtroError
      })?;
    // if let Err(e) = opener::open(&ruta_pdf) {
    //   tracing::error!(
    //     "El registro se eliminó pero no se pudo abrir el visor de PDF: {}",
    //     e
    //   );
    // } else {
    //   tracing::info!("¡Proceso completado! Registro eliminado de la BD y reporte abierto.");
    // }
    Ok(())
  }
}
