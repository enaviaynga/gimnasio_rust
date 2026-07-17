use std::{marker::PhantomData, num::NonZeroU64, sync::Arc};

use bitflags::bitflags;
#[derive(Debug, Clone, Copy)]
pub struct EstadoActivo;
#[derive(Debug, Clone, Copy)]
pub struct EstadoInactivo;
#[derive(Debug, Clone, Copy)]
pub struct EstadoPendiente;

/// Las unicas formas validas de gestionar un empleado es Activo y Inactivo
#[derive(Debug, Clone)]
pub enum EmpleadoEnum {
  Activo(Empleado<EstadoActivo>),
  Inactivo(Empleado<EstadoInactivo>),
  Vacio,
}

#[derive(Debug, Default, Clone)]
pub struct Empleado<T> {
  id: Option<NonZeroU64>,
  permisos: Permisos,
  nombre_usuario: Arc<str>,
  // token_hash: Arc<str>
  estado: PhantomData<T>,
}

impl<EstadoPendiente> Empleado<EstadoPendiente> {
  pub fn existente(id: u64, rol: Permisos, nombre_usuario: String) -> Empleado<EstadoPendiente> {
    Empleado::<EstadoPendiente> {
      id: NonZeroU64::new(id),
      permisos: rol,
      nombre_usuario: nombre_usuario.into(),
      estado: PhantomData,
    }
  }

  pub fn new(rol: RolEmpleado, nombre_usuario: String) -> Empleado<EstadoPendiente> {
    let rol = rol.casteo_permisos();
    Empleado::<EstadoPendiente> {
      id: None,
      permisos: rol,
      nombre_usuario: nombre_usuario.into(),
      estado: PhantomData,
    }
  }

  pub fn estado_activo(self, estado: bool) -> EmpleadoEnum {
    if estado {
      EmpleadoEnum::Activo(Empleado {
        id: self.id,
        permisos: self.permisos,
        nombre_usuario: self.nombre_usuario,
        estado: PhantomData,
      })
    } else {
      EmpleadoEnum::Inactivo(Empleado {
        id: self.id,
        permisos: self.permisos,
        nombre_usuario: self.nombre_usuario,
        estado: PhantomData,
      })
    }
  }
}

impl<EstadoInactivo> Empleado<EstadoInactivo> {
  pub fn activar(self) -> Empleado<EstadoActivo> {
    Empleado {
      id: self.id,
      permisos: self.permisos,
      nombre_usuario: self.nombre_usuario,
      estado: PhantomData,
    }
  }
}

impl<EstadoActivo> Empleado<EstadoActivo> {
  fn quitar_permisos(&mut self, permisos: Permisos) {
    self.permisos.remove(permisos);
  }

  fn añadir_permisos(&mut self, permisos: Permisos) {
    self.permisos.insert(permisos);
  }

  fn tiene_permiso(&self, permisos: Permisos) -> bool {
    self.permisos.contains(permisos)
  }

  pub fn get_permisos(&self) -> Permisos {
    self.permisos
  }
}

impl<T> Empleado<T> {
  pub fn get_id(&self) -> Option<u64> {
    self.id.map(|id| id.get())
  }

  pub fn ref_name(&self) -> &str {
    &self.nombre_usuario
  }

  pub fn datos_completos(&self) -> bool {
    if let Some(id) = self.id {
      !(self.permisos == Permisos::SIN_PERMISOS)
    } else {
      false
    }
  }
}

bitflags! {
    #[derive(Debug, Default, PartialEq, Eq, Clone, Copy)]
    pub struct Permisos: u8 {
        const SIN_PERMISOS  = 0;

        const MEMBRESIAS    = 1 << 0;
        const SOCIOS        = 1 << 1;
        const VENTAS        = 1 << 2;
        const INVENTARIO    = 1 << 3;
        const ASISTENCIA    = 1 << 4;
        const ANALISIS      = 1 << 5;
        const EMPLEADOS     = 1 << 6;

        const ENTRENADOR = Self::ASISTENCIA.bits();

        const RECEPCIONISTA = Self::MEMBRESIAS.bits()
                            | Self::SOCIOS.bits()
                            | Self::VENTAS.bits()
                            | Self::ASISTENCIA.bits();

        const ADMINISTRADOR = Self::ANALISIS.bits()
                            | Self::MEMBRESIAS.bits()
                            | Self::SOCIOS.bits()
                            | Self::VENTAS.bits()
                            | Self::INVENTARIO.bits()
                            | Self::ASISTENCIA.bits()
                            | Self::EMPLEADOS.bits();
    }
}

/// Adaptacion para generar Empleado con los permisos correctos
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RolEmpleado {
  Recepcionista,
  Administrador,
  Entrenador,
  Personalizado(Permisos),
}

impl RolEmpleado {
  fn casteo_permisos(&self) -> Permisos {
    match self {
      RolEmpleado::Recepcionista => Permisos::RECEPCIONISTA,
      RolEmpleado::Administrador => Permisos::ADMINISTRADOR,
      RolEmpleado::Entrenador => Permisos::ENTRENADOR,
      RolEmpleado::Personalizado(p) => *p,
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
