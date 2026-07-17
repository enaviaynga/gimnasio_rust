use crate::persistencia::mysql::socio::socio_repository::MySqlxSocio;
use hexagonal_gimnasio::gestion_socio::{
  dominio::socio::Socio,
  puertos::trait_socios::{ErrorSocio, RegistrarSocio},
};

#[async_trait::async_trait]
impl RegistrarSocio for MySqlxSocio {
  async fn registrar_socio(&self, socio: Socio) -> Result<(), ErrorSocio> {
    let a = socio.get_nombre();
    sqlx::query!(
            "INSERT INTO socios (nombre, apellidos, dni, telefono, correo, direccion) VALUES (?, ?, ?, ?, ?, ?);",
            socio.get_nombre(),
            socio.get_apellido(),
            socio.get_dni(),
            socio.get_telefono(),
            socio.get_correo(),
            socio.get_direccion(),
        ).execute(self.ref_pool()).await.map_err(|_|ErrorSocio::OtroError)?;
    Ok(())
  }
}
