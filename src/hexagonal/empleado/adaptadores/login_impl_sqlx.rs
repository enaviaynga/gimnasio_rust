use crate::{
    hexagonal::empleado::{
        dominio::{
            empleado::{self, Empleado, RolEmpleado},
            usuario::Usuario,
        },
        puertos::traits_login::{BuscarEmpleadoPorUsuario, ErrorMySqlEmpleado},
    },
    infraestructura::{BDsqlx, ErrorMySql},
};
use sqlx::Row;

impl BuscarEmpleadoPorUsuario for BDsqlx {
    async fn login(&self, usuario: Usuario) -> Result<Empleado, ErrorMySqlEmpleado> {
        println!("{usuario:?}");
        let empleado_bd = sqlx::query("SELECT * FROM usuarios WHERE nombreUsuario = ?;")
            .bind(usuario.get_ref_nombre_usuario())
            .fetch_one(self.ref_pool())
            .await
            .map_err(|e| {
                println!("Error en el login: {e:?}");
                match e {
                    sqlx::Error::RowNotFound => ErrorMySqlEmpleado::NoExiste,
                    _ => ErrorMySqlEmpleado::ErrorPeticion,
                }
            })?;

        let rol = match empleado_bd.get("rol") {
            "Administrador" => RolEmpleado::Administrador,
            "Recepcionista" => RolEmpleado::Recepcionista,
            _ => return Err(ErrorMySqlEmpleado::ValorInvalido),
        };

        let id_bd: i32 = empleado_bd.get("idUsuario");
        let id = if id_bd < 0 {
            return Err(ErrorMySqlEmpleado::ValorInvalido);
        } else {
            id_bd as u64
        };
        Ok(Empleado::new_con_id(
            id,
            rol,
            empleado_bd.get("nombreUsuario"),
        ))
    }
}
