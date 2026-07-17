use hexagonal_gimnasio::control_asistencia::{
  aplicacion::caso_obtener_asistencia::ListarAsistenciaDto,
  dominio::asistencia::{Asistencia, ResulValidacion},
  puertos::{
    asistencia_dto::AccesoSocioDTO,
    trait_asistencia::{ErrorAsistencia, ListarAsistencias},
  },
};

use crate::persistencia::mysql::asistencia::asistencia_repositorio::MySqlxAsistencia;

#[async_trait::async_trait]
impl ListarAsistencias for MySqlxAsistencia {
  async fn listar_asistencias(&self) -> Result<Vec<Asistencia>, ErrorAsistencia> {
    let lista_membresia_bd = sqlx::query!("SELECT * FROM asistencia")
      .fetch_all(self.ref_pool())
      .await
      .map_err(|e| ErrorAsistencia::General(format!("{e}")))?;
    let mut lista_asistencia_vec = vec![];
    for i in lista_membresia_bd {
      lista_asistencia_vec.push(Asistencia::existente(
        i.idAsistencia as u64,
        i.idsocio as u64,
        i.fecha,
        i.horaIngreso,
        i.resultadoValidacion
          .parse()
          .map_err(|e| ErrorAsistencia::General(format!("{e}")))?,
      ))
    }
    Ok(lista_asistencia_vec)
  }
}

#[async_trait::async_trait]
impl ListarAsistenciaDto for MySqlxAsistencia {
  async fn obtener_listar_asistencias(&self) -> Result<Vec<AccesoSocioDTO>, ErrorAsistencia> {
    let registros = sqlx::query!(
      r#"
        SELECT
            s.nombre AS nombre_socio,
            s.apellidos AS apellido_socio,
            s.dni AS dni_socio,
            a.idAsistencia AS id_asistencia,
            a.fecha AS fecha_asistencia,
            a.horaIngreso AS hora_asistencia,
            a.resultadoValidacion AS resultado_val
        FROM asistencia a
        INNER JOIN socios s ON a.idsocio = s.idsocio
        WHERE s.activo = TRUE
      "#
    )
    .fetch_all(self.ref_pool())
    .await
    .map_err(|e| ErrorAsistencia::General(format!("{e}")))?;

    let mut lista_asistencia_vec = Vec::with_capacity(registros.len());

    for reg in registros {
      let resultado_validacion = match reg.resultado_val.as_str() {
        "Permitido" => ResulValidacion::Permitido,
        _ => ResulValidacion::NoPermitido,
      };

      lista_asistencia_vec.push(AccesoSocioDTO {
        id_asistencia: reg.id_asistencia,
        nombre_socio: reg.nombre_socio,
        apellido_socio: reg.apellido_socio,
        dni: reg.dni_socio,
        fecha_asistencia: reg.fecha_asistencia,
        hora_asistencia: reg.hora_asistencia,
        resultado_validacion,
      });
    }

    Ok(lista_asistencia_vec)
  }
}
