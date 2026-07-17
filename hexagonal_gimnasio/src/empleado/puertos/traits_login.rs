use crate::empleado::dominio::empleado::EmpleadoEnum;
use crate::empleado::dominio::usuario::Usuario;
use crate::empleado::puertos::errores::ErrorRepositorioEmpleado;

#[async_trait::async_trait]
pub trait BuscarEmpleadoPorUsuario {
  async fn buscar_por_usuario(
    &self,
    usuario: Usuario,
  ) -> Result<EmpleadoEnum, ErrorRepositorioEmpleado>;
}
