use crate::empleado::{
  dominio::empleado::{Empleado, EstadoActivo},
  puertos::errores::ErrorRepositorioEmpleado,
};

pub(crate) trait RegistrarEmpleado {
  async fn registrar(
    &self,
    empleado: Empleado<EstadoActivo>,
  ) -> Result<(), ErrorRepositorioEmpleado>;
}
