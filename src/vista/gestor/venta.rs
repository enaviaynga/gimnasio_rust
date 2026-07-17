pub mod principal;
pub mod producto;
pub mod reponer;

use freya::prelude::*;
use infraestructura::persistencia::mysql::contenedor::ContenedorRepos;
use sqlx::{MySql, Pool};

use crate::vista::gestor::venta::{
  principal::VentaProducto, producto::NuevoProducto, reponer::ReponerStok,
};

#[derive(PartialEq, Clone, Copy)]
pub enum PantallaVenta {
  Venta,
  Stock,
  NuevoStock,
}

#[derive(PartialEq)]
pub struct GestionVentaGui {}

impl Component for GestionVentaGui {
  fn render(&self) -> impl IntoElement {
    let contenedores: State<ContenedorRepos> = use_consume();
    let contendor = use_state(|| DbPoolContext {
      pool: (*contenedores.read().asistencia_repo.ref_pool()).clone(),
    });
    provide_context(contendor);

    let pantalla = use_state(|| PantallaVenta::Venta);
    provide_context(pantalla);

    rect().child(match *pantalla.read() {
      PantallaVenta::Venta => rect().child(VentaProducto),
      PantallaVenta::Stock => rect().child(ReponerStok),
      PantallaVenta::NuevoStock => rect().child(NuevoProducto),
    })
  }
}

#[derive(Clone)]
pub struct DbPoolContext {
  pub pool: Pool<MySql>,
}

impl PartialEq for DbPoolContext {
  fn eq(&self, _other: &Self) -> bool {
    true
  }
}
