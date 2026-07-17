use freya::{prelude::*, router::*};

use crate::vista::{
  configuracion::{ConfiguracionGim, TipoTema},
  gestor::Rutas,
  login::EstadoLogin,
  temas::{tema_claro_no_state, tema_oscuro_no_state},
};
use hexagonal_gimnasio::empleado::dominio::empleado::EmpleadoEnum;
use infraestructura::persistencia::mysql::contenedor::ContenedorRepos;

pub(crate) mod componentes;
pub mod configuracion;
pub(crate) mod gestor;
pub(crate) mod login;
pub(crate) mod temas;

pub(crate) fn app(bd: State<ContenedorRepos>, conf: State<ConfiguracionGim>) -> impl IntoElement {
  use_provide_context(|| bd);
  use_provide_context(|| conf);

  let tema = match conf.read().tema {
    TipoTema::Claro => tema_claro_no_state(conf.read().tema_claro),
    TipoTema::Oscuro => tema_oscuro_no_state(conf.read().tema_oscuro),
  };
  use_provide_theme(|| tema);
  let empleado = use_state(|| {
    tracing::info!("creando enum empleado vacio");
    EmpleadoEnum::Vacio
  });
  use_provide_context(|| empleado);

  let estado_login = use_state(|| EstadoLogin::Cerrado);
  use_provide_context(|| estado_login);

  // use_drop(move || {
  //   tracing::info!("Cerrando el contenedor principal");
  //   let _ = conf.read().guardar();
  // });

  rect().expanded().theme_background().theme_color().child(
    if EstadoLogin::Exito == *estado_login.read() {
      Router::<Rutas>::new(|| RouterConfig::default().with_initial_path(Rutas::Inicio))
        .into_element()
    } else {
      login::login().into_element()
    },
  )
  // rect().child(tabla_asistencia())
}
