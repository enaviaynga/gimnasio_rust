use crate::persistencia::mysql::socio::socio_repository::MySqlxSocio;
use hexagonal_gimnasio::gestion_socio::{
  dominio::socio::Socio,
  puertos::trait_socios::{BuscarSocio, ErrorSocio},
};

#[async_trait::async_trait]
impl BuscarSocio for MySqlxSocio {
  async fn socio_por_id(&self, id: u64) -> Result<Option<Socio>, ErrorSocio> {
    let socio_bd = sqlx::query!("SELECT * FROM socios WHERE idsocio = ?;", id)
      .fetch_one(self.ref_pool())
      .await
      .map_err(|e| -> ErrorSocio {
        tracing::debug!("Error al buscar por id: {e}");
        ErrorSocio::OtroError
      })?;

    Ok(Some(
      Socio::existente(
        socio_bd.idsocio as u64,
        socio_bd.nombre,
        socio_bd.apellidos,
        socio_bd.dni,
        socio_bd.telefono,
        socio_bd.correo,
        socio_bd.direccion,
      )
      .map_err(ErrorSocio::ErrorEnGenerar)?,
    ))
  }

  async fn socio_por_nombre(&self, nombre: &str) -> Result<Vec<Socio>, ErrorSocio> {
    let socio_bd = sqlx::query!("SELECT * FROM socios WHERE nombre = ?;", nombre)
      .fetch_all(self.ref_pool())
      .await
      .map_err(|e| {
        tracing::debug!("Error al buscar por nombre: {e}");
        ErrorSocio::OtroError
      })?;
    let mut socios = vec![];

    for socio in socio_bd {
      socios.push(
        Socio::existente(
          socio.idsocio as u64,
          socio.nombre,
          socio.apellidos,
          socio.dni,
          socio.telefono,
          socio.correo,
          socio.direccion,
        )
        .map_err(ErrorSocio::ErrorEnGenerar)?,
      );
    }
    Ok(socios)
  }

  async fn socio_por_apellidos(&self, apellido: &str) -> Result<Vec<Socio>, ErrorSocio> {
    let socio_bd = sqlx::query!("SELECT * FROM socios WHERE apellidos = ?;", apellido)
      .fetch_all(self.ref_pool())
      .await
      .map_err(|e| {
        tracing::debug!("Error al buscar por apellidos: {e}");
        ErrorSocio::OtroError
      })?;
    let mut socios = vec![];

    for socio in socio_bd {
      socios.push(
        Socio::existente(
          socio.idsocio as u64,
          socio.nombre,
          socio.apellidos,
          socio.dni,
          socio.telefono,
          socio.correo,
          socio.direccion,
        )
        .map_err(ErrorSocio::ErrorEnGenerar)?,
      );
    }
    Ok(socios)
  }

  async fn socio_por_dni(&self, dni: &str) -> Result<Option<Socio>, ErrorSocio> {
    let socio_bd = sqlx::query!("SELECT * FROM socios WHERE dni = ?;", dni)
      .fetch_one(self.ref_pool())
      .await
      .map_err(|e| {
        tracing::debug!("Error al buscar por dni: {e}");
        ErrorSocio::OtroError
      })?;

    Ok(Some(
      Socio::existente(
        socio_bd.idsocio as u64,
        socio_bd.nombre,
        socio_bd.apellidos,
        socio_bd.dni,
        socio_bd.telefono,
        socio_bd.correo,
        socio_bd.direccion,
      )
      .map_err(ErrorSocio::ErrorEnGenerar)?,
    ))
  }

  async fn socio_por_correo(&self, correo: &str) -> Result<Vec<Socio>, ErrorSocio> {
    let socio_bd = sqlx::query!("SELECT * FROM socios WHERE correo = ?;", correo)
      .fetch_all(self.ref_pool())
      .await
      .map_err(|e| {
        tracing::debug!("Error al buscar por correo: {e}");
        ErrorSocio::OtroError
      })?;
    let mut socios = vec![];

    for socio in socio_bd {
      socios.push(
        Socio::existente(
          socio.idsocio as u64,
          socio.nombre,
          socio.apellidos,
          socio.dni,
          socio.telefono,
          socio.correo,
          socio.direccion,
        )
        .map_err(ErrorSocio::ErrorEnGenerar)?,
      );
    }
    Ok(socios)
  }

  async fn socio_por_telefono(&self, telefono: &str) -> Result<Vec<Socio>, ErrorSocio> {
    let socio_bd = sqlx::query!("SELECT * FROM socios WHERE telefono = ?;", telefono)
      .fetch_all(self.ref_pool())
      .await
      .map_err(|e| {
        tracing::debug!("Error al buscar por telefono: {e}");
        ErrorSocio::OtroError
      })?;
    let mut socios = vec![];

    for socio in socio_bd {
      socios.push(
        Socio::existente(
          socio.idsocio as u64,
          socio.nombre,
          socio.apellidos,
          socio.dni,
          socio.telefono,
          socio.correo,
          socio.direccion,
        )
        .map_err(ErrorSocio::ErrorEnGenerar)?,
      );
    }
    Ok(socios)
  }

  async fn socio_por_direcion(&self, direccion: &str) -> Result<Vec<Socio>, ErrorSocio> {
    let socio_bd = sqlx::query!("SELECT * FROM socios WHERE direccion = ?;", direccion)
      .fetch_all(self.ref_pool())
      .await
      .map_err(|e| {
        tracing::debug!("Error al buscar por direccion: {e}");
        ErrorSocio::OtroError
      })?;
    let mut socios = vec![];

    for socio in socio_bd {
      socios.push(
        Socio::existente(
          socio.idsocio as u64,
          socio.nombre,
          socio.apellidos,
          socio.dni,
          socio.telefono,
          socio.correo,
          socio.direccion,
        )
        .map_err(ErrorSocio::ErrorEnGenerar)?,
      );
    }
    Ok(socios)
  }
}
