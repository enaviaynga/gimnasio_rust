use crate::empleado::{
  dominio::{empleado::EmpleadoEnum, usuario::Usuario},
  puertos::{errores::ErrorRepositorioEmpleado, traits_login::BuscarEmpleadoPorUsuario},
};

pub struct CasoLogin<R: BuscarEmpleadoPorUsuario> {
  repo: R,
}

impl<R: BuscarEmpleadoPorUsuario> CasoLogin<R> {
  pub fn new(repo: R) -> Self {
    Self { repo }
  }

  pub async fn ejecutar(&self, usuario: Usuario) -> Result<EmpleadoEnum, ErrorRepositorioEmpleado> {
    self.repo.buscar_por_usuario(usuario).await
  }
}
