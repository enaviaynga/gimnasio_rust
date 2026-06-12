use thiserror::Error;

use crate::utiles::comprobaciones::texto_con_4_tipos;

#[derive(Debug)]
pub(crate) struct Usuario {
    nombre_usuario: NombreUsuario,
    contraseña: Contraseña,
    // hash_contraseña: String,
}

impl Usuario {
    pub fn new(nombre_usuario: String, contraseña: String) -> Result<Self, ErrorUsuario> {
        let nombre_usuario =
            NombreUsuario::new(nombre_usuario).ok_or(ErrorUsuario::UserNameInvalido)?;
        let contraseña = Contraseña::new(contraseña).ok_or(ErrorUsuario::ContraseñaInvalida)?;

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
        !(self.contraseña.0.is_empty() && self.nombre_usuario.0.is_empty())
    }
}

#[derive(Debug)]
pub(crate) struct NombreUsuario(String);

impl NombreUsuario {
    fn new(nombre_usuario: String) -> Option<Self> {
        Some(Self(nombre_usuario))
    }
}

#[derive(Debug)]
pub(crate) struct Contraseña(String);

impl Contraseña {
    pub fn new(contraseña: String) -> Option<Self> {
        let contraseña = Self(contraseña);
        if !contraseña.es_seguro_pre_2017() {
            println!("Intento de acceso con contraseña invalida");
            return None;
        }
        Some(contraseña)
    }

    /// no es 100% segura, puede quedar en ram o escrito en disco
    ///
    /// se recomienda crates que ahora mismo no me acuerdo
    fn vaciar_contraseña(&mut self) {
        self.0.clear();
    }

    fn es_contraseña_extraña(&self) -> bool {
        texto_con_4_tipos(&self.0)
    }

    /// Siguiendo las indicaciones del profesor, no las
    /// recomendaciones en ciberseguridad actual
    fn es_seguro_pre_2017(&self) -> bool {
        self.0.chars().count() >= 8 && self.es_contraseña_extraña()
    }

    /// NIST recomienda un minimo de 15 caracteres, pero
    /// si sigue la regla de lista de palabras basta que
    /// tenga un minimo de 25 de largo
    fn es_realmente_segura(&self) -> bool {
        self.0.chars().count() >= 25
            || (self.0.chars().count() >= 15 && self.es_contraseña_extraña())
    }
}

#[derive(Debug, Error, PartialEq, Eq)]
pub enum ErrorUsuario {
    #[error("Nombre de usuario Invalido")]
    UserNameInvalido,
    #[error("Contraseña invalida")]
    ContraseñaInvalida,
}

#[cfg(test)]
mod test_usuario {
    use super::Contraseña;

    #[test]
    fn verificar_validador_contraseña() {
        let mut contraseña = Contraseña("contraseña invalida".to_string());
        assert!(!contraseña.es_contraseña_extraña());

        contraseña.0 = "Contraseña".to_string();
        assert!(!contraseña.es_contraseña_extraña());

        contraseña.0 = "C0traseña-valida".to_string();
        assert!(contraseña.es_contraseña_extraña());

        contraseña.0 = "ivory rescue average dish urge".to_string();
        assert!(contraseña.es_realmente_segura());

        contraseña.0 = "Crisálida Altramuz Horizonte Burbuja Espejismo".to_string();
        assert!(contraseña.es_realmente_segura());

        contraseña.0 = "CrisálidaAltramuzHorizonteBurbujaEspejismoVolcánMelodíaZafiro".to_string();
        assert!(contraseña.es_realmente_segura());
    }
}
