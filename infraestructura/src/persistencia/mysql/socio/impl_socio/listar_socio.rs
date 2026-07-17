use crate::persistencia::mysql::socio::socio_repository::MySqlxSocio;
use hexagonal_gimnasio::gestion_socio::{
  dominio::socio::Socio,
  puertos::trait_socios::{ErrorSocio, ListarSocios},
};

#[async_trait::async_trait]
impl ListarSocios for MySqlxSocio {
  async fn listar(&self) -> Result<Vec<Socio>, ErrorSocio> {
    let lista_socio_bd = sqlx::query!("SELECT * FROM socios WHERE activo = true")
      .fetch_all(self.ref_pool())
      .await
      .map_err(|e| ErrorSocio::OtroError)?;
    let mut lista_socio_vec = vec![];
    for i in lista_socio_bd {
      lista_socio_vec.push(
        Socio::existente(
          i.idsocio as u64,
          i.nombre,
          i.apellidos,
          i.dni,
          i.telefono,
          i.correo,
          i.direccion,
        )
        .map_err(ErrorSocio::ErrorEnGenerar)?,
      )
    }
    Ok(lista_socio_vec)
  }
}
