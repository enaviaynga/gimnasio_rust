use crate::persistencia::mysql::socio::socio_repository::MySqlxSocio;
use hexagonal_gimnasio::gestion_socio::{
  dominio::socio::Socio,
  puertos::trait_socios::{ActualizarSocio, ErrorSocio},
};

#[async_trait::async_trait]
impl ActualizarSocio for MySqlxSocio {
  async fn actualizar_datos_de_socio(&self, datos: Socio) -> Result<(), ErrorSocio> {
    sqlx::query!(
      // "UPDATE socios SET telefono = COALESCE(?,telefono), correo = COALESCE(?,correo), direccion = COALESCE(?,direccion) WHERE idsocio = ?",
      "UPDATE socios SET telefono = ?, correo = ?, direccion = ? WHERE idsocio = ?",
      datos.get_telefono(),
      datos.get_correo(),
      datos.get_direccion(),
      datos.get_id()
    )
    .fetch_all(self.ref_pool())
    .await
    .map_err(|_| ErrorSocio::OtroError)?;
    Ok(())
  }
}
