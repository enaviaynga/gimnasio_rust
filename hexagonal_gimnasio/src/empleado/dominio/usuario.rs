use bitflags::bitflags;
use thiserror::Error;

#[derive(Debug)]
pub struct Usuario {
  nombre_usuario: NombreUsuario,
  contraseña: Contraseña,
  // hash_contraseña: String,
}

impl Usuario {
  pub fn new(nombre_usuario: String, contraseña: String) -> Result<Self, ErrorUsuario> {
    let nombre_usuario =
      NombreUsuario::new(nombre_usuario).ok_or(ErrorUsuario::UserNameInvalido)?;
    let contraseña = Contraseña::new(contraseña).map_err(ErrorUsuario::ContraseñaInvalida)?;

    Ok(Self {
      nombre_usuario,
      contraseña,
    })
  }

  /// Esto mueve el valor, para clonar se recomienda tomar la
  /// referencia y clonarlo manualmente (`get_ref_nombre_usuario`)
  pub fn get_nombre_usuario(self) -> String {
    self.nombre_usuario.0
  }

  /// Esto toma una referencia, para tomar posecion usa
  /// `get_nombre_usuario`
  pub fn get_ref_nombre_usuario(&self) -> &str {
    &self.nombre_usuario.0
  }

  pub fn vaciar_contraseña(&mut self) {
    self.contraseña.0.clear();
  }

  pub fn datos_completos(&self) -> bool {
    !(self.contraseña.0.is_empty() || self.nombre_usuario.0.is_empty())
  }

  pub fn contraseña_igual(&self, contraseña_data_base: &str) -> bool {
    self.contraseña.0 == contraseña_data_base
  }
}

#[derive(Debug)]
pub(crate) struct NombreUsuario(String);

impl NombreUsuario {
  fn new(nombre_usuario: String) -> Option<Self> {
    if nombre_usuario.chars().count() <= 255 {
      Some(Self(nombre_usuario))
    } else {
      None
    }
  }
}

#[derive(Debug)]
pub(crate) struct Contraseña(String);

impl Contraseña {
  pub fn new(contraseña: String) -> Result<Self, CondicionesContraseña> {
    let contraseña = Self(contraseña);
    let c = contraseña.es_seguro_pre_2017();
    if c.contains(CondicionesContraseña::TODOS_CONDICIONES) {
      Ok(contraseña)
    } else {
      Err(c)
    }
  }

  /// no es 100% segura, puede quedar en ram o escrito en disco
  ///
  /// se recomienda crates que ahora mismo no me acuerdo
  fn vaciar_contraseña(&mut self) {
    self.0.clear();
  }

  /// Siguiendo las indicaciones del profesor, no las
  /// recomendaciones en ciberseguridad actual
  fn es_seguro_pre_2017(&self) -> CondicionesContraseña {
    let r = texto_con_4_tipos(&self.0);
    if self.0.chars().count() >= 8 {
      r | CondicionesContraseña::LARGO_SUFICIENTE
    } else {
      r
    }
  }

  /// NIST recomienda un minimo de 15 caracteres, pero
  /// si sigue la regla de lista de palabras basta que
  /// tenga un minimo de 25 de largo
  fn es_realmente_segura(&self) -> bool {
    if self.0.chars().count() >= 25 {
      return true;
    };
    self.0.chars().count() >= 15
      && texto_con_4_tipos(&self.0).contains(CondicionesContraseña::TODOS_SIMBOLOS)
  }
}

bitflags! {
    #[derive(Debug, PartialEq, Eq)]
    pub struct CondicionesContraseña: u8 {
        const SIN_TIPO = 0;

        const NUMERO = 1 << 0;
        const MINUSCULA = 1 << 1;
        const MAYUSCULA = 1 << 2;
        const SIMBOLO = 1 << 3;

        const LARGO_SUFICIENTE = 1 << 4;

        const TODOS_SIMBOLOS = Self::NUMERO.bits() | Self::MINUSCULA.bits() | Self::MAYUSCULA.bits() | Self::SIMBOLO.bits();

        const TODOS_CONDICIONES = Self::TODOS_SIMBOLOS.bits() | Self::LARGO_SUFICIENTE.bits();
    }
}

pub fn texto_con_4_tipos(texto: &str) -> CondicionesContraseña {
  let mut tipos = CondicionesContraseña::SIN_TIPO;

  for i in texto.chars() {
    if i.is_uppercase() {
      tipos |= CondicionesContraseña::MAYUSCULA;
    }

    if i.is_lowercase() {
      tipos |= CondicionesContraseña::MINUSCULA;
    }

    if i.is_numeric() {
      tipos |= CondicionesContraseña::NUMERO;
    }

    // if !i.is_alphanumeric() {
    //     tipos |= TiposCaracteres::SIMBOLO;
    // }

    if "!\"#$%&'()*+,-./:;<=>?@[\\]^_`{|}~".contains(i) {
      tipos |= CondicionesContraseña::SIMBOLO;
    }

    if tipos.contains(CondicionesContraseña::TODOS_SIMBOLOS) {
      return tipos;
    }
  }

  tipos
}

#[derive(Debug, Error, PartialEq, Eq)]
pub enum ErrorUsuario {
  #[error("Nombre de usuario Invalido, no puede superar los 50 caracteres")]
  UserNameInvalido,
  #[error("Contraseña invalida: {0:?}")]
  ContraseñaInvalida(CondicionesContraseña),
}

#[cfg(test)]
mod test_usuario {
  use super::Contraseña;
  use crate::empleado::dominio::usuario::{CondicionesContraseña, Usuario, texto_con_4_tipos};

  #[test]
  fn verificar_validador_contraseña() {
    let mut contraseña = Contraseña("contraseña invalida".to_string());
    assert_eq!(
      texto_con_4_tipos(&contraseña.0),
      // TiposCaracteres::SIMBOLO | // descomentar si se usa !i.is_alphanumeric()
      CondicionesContraseña::MINUSCULA
    );

    contraseña.0 = "Contraseña".to_string();
    assert_eq!(
      texto_con_4_tipos(&contraseña.0),
      CondicionesContraseña::MAYUSCULA | CondicionesContraseña::MINUSCULA
    );

    contraseña.0 = "C0traseña-valida".to_string();
    assert_eq!(
      texto_con_4_tipos(&contraseña.0),
      CondicionesContraseña::TODOS_SIMBOLOS
    );

    contraseña.0 = "ivory rescue average dish urge".to_string();
    assert!(contraseña.es_realmente_segura());

    contraseña.0 = "Crisálida Altramuz Horizonte Burbuja Espejismo".to_string();
    assert!(contraseña.es_realmente_segura());

    contraseña.0 = "CrisálidaAltramuzHorizonteBurbujaEspejismoVolcánMelodíaZafiro".to_string();
    assert!(contraseña.es_realmente_segura());

    contraseña.vaciar_contraseña();
  }

  #[test]
  fn usuario() {
    assert!(Usuario::new("nombre_usuario".to_string(), "contraseña".to_string()).is_err());
    assert!(Usuario::new("r".repeat(256), "contraseña".to_string()).is_err());

    let mut usuario_valido =
      Usuario::new("nombre_usuario".to_string(), "C0ntra$eña".to_string()).unwrap();
    let contraseña = usuario_valido.get_ref_nombre_usuario();

    assert!(usuario_valido.datos_completos());

    usuario_valido.vaciar_contraseña();
    assert!(!usuario_valido.datos_completos());

    let nombre = usuario_valido.get_nombre_usuario();
    let usuario_valido = Usuario::new("nombre_usuario".to_string(), "C0ntra$eña".to_string());
    assert!(usuario_valido.is_ok());
    let mut usuario_valido = usuario_valido.unwrap();
    usuario_valido.nombre_usuario.0.clear();
    assert!(!usuario_valido.datos_completos());
    usuario_valido.contraseña.0.clear();
    assert!(!usuario_valido.datos_completos());
  }
}
