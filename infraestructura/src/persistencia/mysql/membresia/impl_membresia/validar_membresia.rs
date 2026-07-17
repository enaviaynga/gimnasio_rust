use hexagonal_gimnasio::membresias::{
  dominio::membresia::{EstadoMembresia, Membresia},
  puertos::trait_membresia::{ErrorMembresia, VerificarVencimientosProgramados},
};
use sqlx::types::chrono::{Local, NaiveDate};

use crate::persistencia::mysql::membresia::membresia_repositorio::MySqlxMembresia;

#[async_trait::async_trait]
impl VerificarVencimientosProgramados for MySqlxMembresia {
  fn verificar_una_membresia(&self, membresia: &Membresia) -> EstadoMembresia {
    let hoy = Local::now();
    tracing::info!("dato hoy: {hoy}");
    membresia.get_proximo_a_vencer_default(hoy.date_naive())
  }

  async fn verificar_por_id_socio(
    &self,
    id_socio: u64,
  ) -> Result<Option<EstadoMembresia>, ErrorMembresia> {
    let membresia_bd = sqlx::query!(
      "SELECT idMembresia, tipoMembresia, fechaInicio, fechaVencimiento, costo
        FROM membresias
        WHERE idsocio = ?
        ORDER BY fechaInicio DESC, idMembresia DESC
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
    let hoy = Local::now();
    tracing::info!("dato hoy: {hoy}");
    Ok(Some(
      membresias.get_proximo_a_vencer_default(hoy.date_naive()),
    ))
  }

  // async fn verificar_todos(&self) -> Result<Vec<EstadoMembresia>, ErrorMembresia> {
  //   todo!("Aun no implementado, sera util mas en reporte porque resumiria totales")
  // }
}
