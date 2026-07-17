use std::fs::{self};

use directories::ProjectDirs;
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::vista::temas::{TemasClaros, TemasOscuros};

#[derive(Serialize, Deserialize, PartialEq, Default, Clone, Copy, Debug)]
pub struct ConfiguracionGim {
  pub tema: TipoTema,
  pub tema_oscuro: TemasOscuros,
  pub tema_claro: TemasClaros,
}

impl ConfiguracionGim {
  pub fn _new(tema: TipoTema, tema_oscuro: TemasOscuros, tema_claro: TemasClaros) -> Self {
    Self {
      tema,
      tema_oscuro,
      tema_claro,
    }
  }

  pub fn guardar(&self) -> Result<(), ErroresConfiguracion> {
    if let Some(a) = ProjectDirs::from("rs", "eliminable", "gimnasio") {
      let ruta_conf = a.config_dir();
      std::fs::create_dir_all(ruta_conf).map_err(|e| {
        tracing::warn!(
          "Error con la ruta `{ruta_conf:?}` al crear la carpeta de configuracion: {e}"
        );
        ErroresConfiguracion::Guardado
      })?;

      let mut archivo_conf = ruta_conf.to_path_buf();
      archivo_conf.push("Configuracion.toml");

      toml::to_string(self)
        .map_err(|e| {
          tracing::warn!("Error al serializar la configuracion: {e}");
          std::io::Error::other(e)
        })
        .and_then(|conf_texto| match fs::read_to_string(&archivo_conf) {
          Ok(contenido_actual) => match toml::from_str::<Self>(&contenido_actual) {
            Ok(self_actual) if &self_actual == self => {
              tracing::info!("La configuración no ha cambiado. No se requiere escritura.");
              Ok(())
            }
            _ => fs::write(&archivo_conf, conf_texto.as_bytes()),
          },
          Err(ref e) if e.kind() == std::io::ErrorKind::NotFound => {
            fs::write(&archivo_conf, conf_texto.as_bytes())
          }
          Err(e) => Err(e),
        })
        .map_err(|e| {
          tracing::warn!("Error al momento de guardar: {e}");
          ErroresConfiguracion::Guardado
        })
    } else {
      Err(ErroresConfiguracion::NoEnvSistema)
    }
  }

  pub fn cargar() -> Result<Self, ErroresConfiguracion> {
    if let Some(a) = ProjectDirs::from("rs", "eliminable", "gimnasio") {
      let ruta_conf = a.config_dir();
      if let Err(e) = std::fs::create_dir_all(ruta_conf) {
        tracing::debug!("Error al crear la carpeta de configuracion: {e}");
        return Err(ErroresConfiguracion::Lectura);
      };

      let mut archivo_conf = ruta_conf.to_path_buf();
      archivo_conf.push("Configuracion.toml");

      fs::read_to_string(&archivo_conf)
        .map_err(|e| {
          tracing::warn!(
            "Error al serializar, el formato en Configuracion.toml es incorrecto: {e}"
          );
          ErroresConfiguracion::Lectura
        })
        .and_then(|e| {
          toml::from_str::<ConfiguracionGim>(&e).map_err(|e| {
            tracing::warn!(
              "Error al serializar, el formato en Configuracion.toml es incorrecto: {e}"
            );
            ErroresConfiguracion::Lectura
          })
        })
    } else {
      Err(ErroresConfiguracion::NoEnvSistema)
    }
  }
}

#[derive(Serialize, Deserialize, PartialEq, Default, Clone, Copy, Debug)]
pub enum TipoTema {
  #[default]
  Claro,
  Oscuro,
}

impl TipoTema {
  pub fn _es_oscuro(&self) -> bool {
    match self {
      TipoTema::Claro => false,
      TipoTema::Oscuro => true,
    }
  }

  pub fn _set_oscuro_bool(&mut self, es_oscuro: bool) {
    if es_oscuro {
      *self = TipoTema::Oscuro;
    } else {
      *self = TipoTema::Claro
    }
  }

  pub fn invertir(&mut self) {
    *self = match self {
      TipoTema::Claro => TipoTema::Oscuro,
      TipoTema::Oscuro => TipoTema::Claro,
    };
  }
}

#[derive(Clone, Copy, Error, Debug)]
pub enum ErroresConfiguracion {
  #[error("Fallo Lectura")]
  Lectura,
  #[error("Fallo Guardado")]
  Guardado,
  #[error(
    "No estan las variables de entorno del sistema necesarios para crear la carpeta propia del programa en la configuracion en el sistema"
  )]
  NoEnvSistema,
}
