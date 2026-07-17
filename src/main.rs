#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]
#![allow(unused_variables, dead_code, unused_imports)]
#![allow(unused_mut)]

mod utiles;
mod vista;

use anyhow::{Context, Result};

use freya::prelude::*;
use thiserror::Error;
use tracing_subscriber::EnvFilter;

use crate::{
  utiles::crear_y_anadir_datos_a_la_base_mysql_mediante_sqlx_en_codigo::crear_tablas,
  vista::{app, configuracion::ConfiguracionGim},
};
use infraestructura::persistencia::mysql::contenedor::ContenedorRepos;

#[tokio::main]
async fn main() -> Result<()> {
  registros()?;
  tracing::info!("App iniciada");

  let bd_url = std::env::var("BD_url")
        .context("Fallo en obtener la url (BD_url) de las variables de entorno para la conección a la base de datos")?;
  crear_tablas(&bd_url)
    .await
    .context("Fallo en ejecutar la creación de tablas.")?;

  let my_pool = sqlx::MySqlPool::connect(&bd_url)
    .await
    .context("Fallo la creación del pool en base a la conección a la base de datos")?;
  let contenedor = ContenedorRepos::new(my_pool).await;

  let configuracion_struct = ConfiguracionGim::cargar().unwrap_or_default();

  tracing::info!("Base de datos conectado correctamente. Iniciando UI");
  launch(
    LaunchConfig::new().with_window(
      WindowConfig::new(move || {
        app(
          use_state(|| contenedor.clone()),
          use_state(|| configuracion_struct),
        )
      })
      .with_size(1600., 900.)
      .with_title("Gimnasio")
      .with_min_size(960., 600.),
    ),
  );

  Ok(())
}

#[derive(Error, Debug)]
enum ErrorMain {
  #[error("Error en iniciar el logger tracing")]
  Tracing,
}

fn registros() -> Result<()> {
  let nivel_log_global = "warn";
  tracing_subscriber::fmt()
    .with_env_filter(
      EnvFilter::from_default_env()
        .add_directive((format!("freya={nivel_log_global}")).parse()?)
        .add_directive((format!("freya_core={nivel_log_global}")).parse()?)
        .add_directive((format!("winit={nivel_log_global}")).parse()?)
        .add_directive((format!("sqlx={nivel_log_global}")).parse()?),
    )
    .try_init()
    .map_err(|_| ErrorMain::Tracing)?;
  Ok(())
}

#[cfg(test)]
mod test_main {
  use crate::registros;

  #[test]
  fn log() {
    registros().unwrap();
    assert!(registros().is_err());
  }
}
