use chrono::{NaiveDate, NaiveTime};
use freya::prelude::*;
use hexagonal_gimnasio::control_asistencia::puertos::asistencia_dto::AsistenciaSocioMembresiaDto;

use crate::vista::gestor::asistencia::{
  tabla_asistencia::TablaAsistencia, ver_asistencia::VerAsistencia,
};

pub mod tabla_asistencia;
pub mod ver_asistencia;

#[derive(Debug)]
pub struct AsistenciaSocio {
  pub id_asistencia: u32,
  pub id_socio: u32,
  pub nombre_socio: String,
  pub apellidos_socio: String,
  pub dni_socio: String,
  pub fecha: NaiveDate,
  pub hora_ingreso: NaiveTime,
  pub resultado_validacion: String,
}

#[derive(PartialEq)]
pub struct AsistenciaGui;
impl Component for AsistenciaGui {
  fn render(&self) -> impl IntoElement {
    let pantalla_selecionada = use_state(|| PantallaAsistencia::Inicio);
    use_provide_context(|| pantalla_selecionada);

    let asistencia_selecionada = use_state(|| None::<AsistenciaSocioMembresiaDto>);
    use_provide_context(|| asistencia_selecionada);
    rect().child(match *pantalla_selecionada.read() {
      PantallaAsistencia::Inicio => rect().child(TablaAsistencia),
      PantallaAsistencia::Ver => rect().child(VerAsistencia),
    })
  }
}

#[derive(Clone, Copy)]
pub enum PantallaAsistencia {
  Inicio, // listar
  Ver,
}
