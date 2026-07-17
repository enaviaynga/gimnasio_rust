use freya::{prelude::*, router::RouterContext};
use hexagonal_gimnasio::empleado::dominio::empleado::{EmpleadoEnum, Permisos};

use crate::vista::gestor::Rutas;

pub fn barra_btn(
  empleado_global: &State<EmpleadoEnum>,
  permisos: Permisos,
  texto: &str,
  ruta: Rutas,
) -> Rect {
  if let EmpleadoEnum::Activo(e) = &*empleado_global.read()
    && e.get_permisos().contains(permisos)
  {
    rect().child(
      Button::new()
        .on_press(move |_| {
          let _ = RouterContext::get().replace(ruta.clone());
        })
        .child(texto),
    )
  } else {
    rect()
  }
}
