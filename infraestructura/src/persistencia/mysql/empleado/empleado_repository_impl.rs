use crate::persistencia::mysql::empleado::empleado_repository::MySqlxEmpleado;
use hexagonal_gimnasio::empleado::{
  dominio::{
    empleado::{Empleado, EmpleadoEnum, EstadoPendiente, Permisos, RolEmpleado},
    usuario::Usuario,
  },
  puertos::{errores::ErrorRepositorioEmpleado, traits_login::BuscarEmpleadoPorUsuario},
};

#[async_trait::async_trait]
impl BuscarEmpleadoPorUsuario for MySqlxEmpleado {
  async fn buscar_por_usuario(
    &self,
    usuario: Usuario,
  ) -> Result<EmpleadoEnum, ErrorRepositorioEmpleado> {
    tracing::debug!("{usuario:?}");
    let nombre_usuario = usuario.get_ref_nombre_usuario();
    let empleado_bd = sqlx::query!(
      "SELECT * FROM usuarios WHERE nombreUsuario = ?;",
      nombre_usuario
    )
    .fetch_one(self.ref_pool())
    .await
    .map_err(|e| {
      tracing::debug!("Error en el login: {e:?}");
      match e {
        sqlx::Error::RowNotFound => ErrorRepositorioEmpleado::NoExiste,
        _ => ErrorRepositorioEmpleado::ErrorPeticion,
      }
    })?;

    let rol = match empleado_bd.rol.as_ref() {
      "Administrador" => Permisos::ADMINISTRADOR,
      "Recepcionista" => Permisos::RECEPCIONISTA,
      "Entrenador" => Permisos::ENTRENADOR,
      a => match a.parse::<u8>() {
        Ok(n) => Permisos::from_bits_retain(n),
        Err(_) => return Err(ErrorRepositorioEmpleado::ValorInvalido),
      },
    };

    if !usuario.contraseña_igual(&empleado_bd.contrasena) {
      tracing::info!(
        "Un intento a acceder a la base de datos con una contraseña valida pero no registrada"
      );
      return Err(ErrorRepositorioEmpleado::ContraseñaInvalida);
    }

    Ok(
      Empleado::<EstadoPendiente>::existente(
        empleado_bd.idUsuario as u64,
        rol,
        empleado_bd.nombreUsuario,
      )
      .estado_activo(true),
    )
  }
}
