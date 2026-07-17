use std::{default, num::NonZeroU64, sync::Arc};

use bitflags::bitflags;
use thiserror::Error;

#[derive(Debug, Clone, Default)]
pub struct Socio {
  id: Option<NonZeroU64>,
  nombre: NombreSocio,
  apellidos: ApellidoSocio,
  dni: DniSocio,
  telefono: Option<TelefonoSocio>,
  correo: Option<CorreoSocio>,
  direccion: Option<DirecionSocio>,
}

impl Socio {
  pub fn new(
    nombre: String,
    apellidos: String,
    dni: String,
    telefono: Option<String>,
    correo: Option<String>,
    direccion: Option<String>,
  ) -> Result<Self, SocioInvalido> {
    let mut errores = SocioInvalido::SIN_ERROR;

    let mut nombre_valido = None;
    let mut apellidos_valido = None;
    let mut dni_valido = None;
    let mut telefono_valido = None;
    let mut correo_valido = None;
    let mut direccion_valido = None;

    match NombreSocio::new(nombre) {
      Ok(v) => nombre_valido = Some(v),
      Err(e) => errores |= e,
    }

    match ApellidoSocio::new(apellidos) {
      Ok(v) => apellidos_valido = Some(v),
      Err(e) => errores |= e,
    }

    match DniSocio::new(dni) {
      Ok(v) => dni_valido = Some(v),
      Err(e) => errores |= e,
    }

    if let Some(tel) = telefono {
      match TelefonoSocio::new(tel) {
        Ok(v) => telefono_valido = Some(v),
        Err(e) => errores |= e,
      }
    }

    if let Some(corr) = correo {
      match CorreoSocio::new(corr) {
        Ok(v) => correo_valido = Some(v),
        Err(e) => errores |= e,
      }
    }

    if let Some(dir) = direccion {
      match DirecionSocio::new(dir) {
        Ok(v) => direccion_valido = Some(v),
        Err(e) => errores |= e,
      }
    }

    if errores == SocioInvalido::SIN_ERROR
      && let (Some(nombre), Some(apellidos), Some(dni)) =
        (nombre_valido, apellidos_valido, dni_valido)
    {
      Ok(Self {
        id: None,
        nombre,
        apellidos,
        dni,
        telefono: telefono_valido,
        correo: correo_valido,
        direccion: direccion_valido,
      })
    } else {
      Err(errores)
    }
  }

  pub fn existente(
    id: u64,
    nombre: String,
    apellidos: String,
    dni: String,
    telefono: Option<String>,
    correo: Option<String>,
    direccion: Option<String>,
  ) -> Result<Self, SocioInvalido> {
    let nombre = NombreSocio::new(nombre)?;
    let apellidos = ApellidoSocio::new(apellidos)?;
    let dni = DniSocio::new(dni)?;
    let telefono = telefono.map(TelefonoSocio::new).transpose()?;
    let correo = correo.map(CorreoSocio::new).transpose()?;
    let direccion = direccion.map(DirecionSocio::new).transpose()?;
    Ok(Self {
      id: NonZeroU64::new(id),
      nombre,
      apellidos,
      dni,
      telefono,
      correo,
      direccion,
    })
  }

  pub fn get_id(&self) -> Option<u64> {
    self.id.map(|id| id.get())
  }

  pub fn get_nombre(&self) -> &str {
    &self.nombre.0
  }

  pub fn get_apellido(&self) -> &str {
    &self.apellidos.0
  }

  pub fn get_dni(&self) -> &str {
    &self.dni.0
  }

  pub fn get_telefono(&self) -> Option<&str> {
    // self.telefono.as_deref().map(|t| &t.0) // solo si self implementa Deref
    self.telefono.as_ref().map(|t| t.0.as_ref())
  }

  pub fn get_correo(&self) -> Option<&str> {
    self.correo.as_ref().map(|c| c.0.as_ref())
  }

  pub fn get_direccion(&self) -> Option<&str> {
    self.direccion.as_ref().map(|d| d.0.as_ref())
  }

  pub fn set_telefono(&mut self, telefono: String) -> Result<(), SocioInvalido> {
    self.telefono = Some(TelefonoSocio::new(telefono)?);
    Ok(())
  }

  pub fn set_correo(&mut self, correo: String) -> Result<(), SocioInvalido> {
    self.correo = Some(CorreoSocio::new(correo)?);
    Ok(())
  }

  pub fn set_direccion(&mut self, direccion: String) -> Result<(), SocioInvalido> {
    self.direccion = Some(DirecionSocio::new(direccion)?);
    Ok(())
  }

  pub fn set_telefono_none(&mut self) {
    self.telefono = None;
  }

  pub fn set_correo_none(&mut self) {
    self.correo = None;
  }

  pub fn set_direccion_none(&mut self) {
    self.direccion = None;
  }
}

bitflags! {
    #[derive(Debug, Default, PartialEq, Eq, Clone, Copy, Error)]
    #[error("Errores de validacion en: {0}")]
    pub struct SocioInvalido: u8 {
        const SIN_ERROR = 0;

        const NOMBRE = 1 << 0;
        const APELLIDO = 1 << 1;
        const DNI = 1 << 2;
        const TELEFONO = 1 << 3;
        const CORREO = 1 << 4;
        const DIRECCION = 1 << 5;
    }
}

#[derive(Debug, Clone, Default)]
struct NombreSocio(Arc<str>);

impl NombreSocio {
  fn new(nombre: String) -> Result<Self, SocioInvalido> {
    let nombre = nombre.trim();

    if nombre.chars().count() <= 50 {
      Ok(Self(nombre.into()))
    } else {
      Err(SocioInvalido::NOMBRE)
    }
  }
}

#[derive(Debug, Clone, Default)]
struct ApellidoSocio(Arc<str>);

impl ApellidoSocio {
  fn new(apellido: String) -> Result<Self, SocioInvalido> {
    let apellido = apellido.trim();

    if apellido.chars().count() <= 50 && apellido.contains(' ') {
      Ok(Self(apellido.into()))
    } else {
      Err(SocioInvalido::APELLIDO)
    }
  }
}

#[derive(Debug, Clone, Default)]
struct DniSocio(Arc<str>);

impl DniSocio {
  fn new(dni: String) -> Result<Self, SocioInvalido> {
    let dni = dni.trim();

    if dni.chars().count() == 8 && dni.chars().all(|c| c.is_numeric()) {
      Ok(Self(dni.into()))
    } else {
      Err(SocioInvalido::DNI)
    }
  }
}

#[derive(Debug, Clone)]
struct TelefonoSocio(Arc<str>);

impl TelefonoSocio {
  pub fn new(telefono: String) -> Result<Self, SocioInvalido> {
    let telefono = telefono.trim();

    // logica para verificar formato telefonico pendiente
    // E.164, indica maximo 15 digitos,
    // iniciando + seguido del numero

    if telefono.chars().all(|c| c.is_numeric()) && telefono.len() == 9 {
      Ok(Self(telefono.into()))
    } else {
      Err(SocioInvalido::TELEFONO)
    }
  }

  fn get(&self) -> &str {
    &self.0
  }
}

#[derive(Debug, Clone)]
struct CorreoSocio(Arc<str>);

impl CorreoSocio {
  pub fn new(correo: String) -> Result<Self, SocioInvalido> {
    let correo = correo.trim();
    if correo.chars().count() <= 100 && ['@', '.'].iter().all(|&c| correo.contains(c)) {
      Ok(Self(correo.into()))
    } else {
      Err(SocioInvalido::CORREO)
    }
  }

  fn get(&self) -> &str {
    &self.0
  }
}

#[derive(Debug, Clone)]
struct DirecionSocio(Arc<str>);

impl DirecionSocio {
  pub fn new(direccion: String) -> Result<Self, SocioInvalido> {
    let direccion = direccion.trim();

    if direccion.len() <= 150 {
      Ok(Self(direccion.into()))
    } else {
      Err(SocioInvalido::DIRECCION)
    }
  }

  fn get(&self) -> &str {
    &self.0
  }
}

#[cfg(test)]
mod test_socios {
  use super::*;

  #[test]
  fn validar_error_default() {
    assert_eq!(SocioInvalido::default(), SocioInvalido::SIN_ERROR);
  }

  #[test]
  fn validar_error() {
    // Validación de Nombre
    let e = NombreSocio::new("r".repeat(51)).unwrap_err();
    assert!(e.contains(SocioInvalido::NOMBRE));

    // Validación de Apellido()
    let e = ApellidoSocio::new("r".repeat(51)).unwrap_err();
    assert!(e.contains(SocioInvalido::APELLIDO));

    // Validación de DNI
    let e = DniSocio::new("r".repeat(7)).unwrap_err();
    assert!(e.contains(SocioInvalido::DNI));

    // Validación de Teléfono
    let e = TelefonoSocio::new("r".repeat(16)).unwrap_err();
    assert!(e.contains(SocioInvalido::TELEFONO));

    // Validación de Correo
    let e = CorreoSocio::new("r".repeat(101)).unwrap_err();
    assert!(e.contains(SocioInvalido::CORREO));

    // Validación de Dirección (Longitud)
    let e = DirecionSocio::new("r".repeat(151)).unwrap_err();
    assert!(e.contains(SocioInvalido::DIRECCION));
  }
}
