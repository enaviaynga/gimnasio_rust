use bitflags::bitflags;

#[derive(Debug, Default, Clone)]
pub struct Empleado {
    id: Option<u64>,
    rol: Permisos,
    // Pendiente revisar como usar un Cow
    nombre_usuario: String,
    // hash_sesion: String
}

impl Empleado {
    pub fn new_con_id(id: u64, rol: RolEmpleado, nombre_usuario: String) -> Self {
        let rol = rol.casteo_permisos();
        Self {
            id: Some(id),
            rol,
            nombre_usuario,
        }
    }

    pub fn new_vacio() -> Self {
        Self {
            id: None,
            rol: Permisos::SIN_PERMISOS,
            nombre_usuario: "".to_string(),
        }
    }

    pub fn get_id(&self) -> Option<u64> {
        self.id
    }

    pub fn get_rol(&self) -> RolEmpleado {
        self.rol.casteo_rol()
    }

    pub(crate) fn ref_user_name(&self) -> &str {
        &self.nombre_usuario
    }

    fn quitar_permisos(&mut self, permisos: Permisos) {
        self.rol.remove(permisos);
    }

    fn añadir_permisos(&mut self, permisos: Permisos) {
        self.rol.insert(permisos);
    }

    fn tiene_permiso(&self, permisos: Permisos) -> bool {
        self.rol.contains(permisos)
    }

    pub fn datos_completos(&self) -> bool {
        if let Some(id) = self.id {
            !(id == 0 || self.rol == Permisos::SIN_PERMISOS)
        } else {
            false
        }
    }
}

bitflags! {
    #[derive(Debug, Default, PartialEq, Eq, Clone, Copy)]
    struct Permisos: u32 {
        const SIN_PERMISOS  = 0b0000_0000_0000_0000;

        const MEMBRESIAS    = 0b0000_0000_0000_0001;
        const SOCIOS        = 0b0000_0000_0000_0010;
        const VENTAS        = 0b0000_0000_0000_0100;

        // const

        const RECEPCIONISTA = Permisos::MEMBRESIAS.bits() | Permisos::SOCIOS.bits() | Permisos::VENTAS.bits();

        const ADMINISTRADOR = 0b1000_0000_0000_0000;
    }
}

impl Permisos {
    fn casteo_rol(self) -> RolEmpleado {
        match self {
            Permisos::ADMINISTRADOR => RolEmpleado::Administrador,
            Permisos::RECEPCIONISTA => RolEmpleado::Recepcionista,
            _ => RolEmpleado::SinRol,
        }
    }
}

/// Adaptacion para generar Empleado con los permisos correctos
pub enum RolEmpleado {
    Recepcionista,
    Administrador,
    SinRol,
}

impl RolEmpleado {
    fn casteo_permisos(&self) -> Permisos {
        match self {
            RolEmpleado::Recepcionista => Permisos::RECEPCIONISTA,
            RolEmpleado::Administrador => Permisos::ADMINISTRADOR,
            RolEmpleado::SinRol => Permisos::SIN_PERMISOS,
        }
    }
}

#[cfg(test)]
mod test_empleado {
    use super::Permisos;

    #[test]
    fn prueba_rol_default() {
        let rol_default = Permisos::default();
        assert_eq!(rol_default, Permisos::SIN_PERMISOS);
    }
}
