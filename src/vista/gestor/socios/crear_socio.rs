use freya::prelude::*;

use crate::vista::{
  componentes::{alerta::mensaje_alerta, boton_cerrar_y_volver_al_inicio::boton_cerrar},
  gestor::socios::PantallaSocio,
};
use hexagonal_gimnasio::gestion_socio::{
  aplicacion::caso_registrar_socio::CasoRegistrarSocio,
  dominio::socio::{Socio, SocioInvalido},
};
use infraestructura::persistencia::mysql::contenedor::{self, ContenedorRepos};

#[derive(PartialEq)]
pub struct CrearSocioGui;

impl Component for CrearSocioGui {
  fn render(&self) -> impl IntoElement {
    let mut pantalla_socio_actual: State<PantallaSocio> = use_consume();
    let contenedor: State<ContenedorRepos> = use_consume();
    let mut error_dato_invalido_socio = use_state(|| SocioInvalido::SIN_ERROR);

    let nombre = use_state(String::new);
    let apellidos = use_state(String::new);
    let dni = use_state(String::new);
    let telefono = use_state(String::new);
    let correo = use_state(String::new);
    let direccion = use_state(String::new);

    rect()
      .width(Size::fill())
      .height(Size::fill())
      .center()
      .child(boton_cerrar(pantalla_socio_actual, PantallaSocio::Tabla))
      .child(
        rect()
          .width(Size::percent(55.0))
          .height(Size::auto())
          .padding(Gaps::new_all(28.0))
          .vertical()
          .padding(Gaps::new_symmetric(20.0, 0.0))
          .corner_radius(CornerRadius::new_all(14.0))
          .child(
            label()
              .text("Nuevo Socio")
              .font_size(24.0)
              .font_weight(FontWeight::BOLD),
          )
          .child(
            rect()
              .content(Content::Flex)
              .direction(Direction::Horizontal)
              .width(Size::fill())
              .spacing(20.)
              .child(
                rect()
                  .width(Size::flex(1.))
                  .vertical()
                  .padding(Gaps::new_symmetric(6.0, 0.0))
                  .child(label().text("Nombre"))
                  .child(Input::new(nombre).width(Size::fill())),
              )
              .child(
                rect()
                  .width(Size::flex(1.))
                  .vertical()
                  .padding(Gaps::new_symmetric(6.0, 0.0))
                  .child(label().text("Apellidos"))
                  .child(Input::new(apellidos).width(Size::fill())),
              ),
          )
          .child(
            rect()
              .content(Content::Flex)
              .direction(Direction::Horizontal)
              .width(Size::fill())
              .spacing(20.)
              .child(
                rect()
                  .width(Size::flex(1.))
                  .vertical()
                  .padding(Gaps::new_symmetric(6.0, 0.0))
                  .child(label().text("DNI"))
                  .child(Input::new(dni).width(Size::fill())),
              )
              .child(
                rect()
                  .width(Size::flex(1.))
                  .vertical()
                  .padding(Gaps::new_symmetric(6.0, 0.0))
                  .child(label().text("Teléfono"))
                  .child(Input::new(telefono).width(Size::fill())),
              ),
          )
          .child(
            rect()
              .width(Size::fill())
              .vertical()
              .padding(Gaps::new_symmetric(6.0, 0.0))
              .child(label().text("Correo Electrónico"))
              .child(Input::new(correo).width(Size::fill())),
          )
          .child(
            rect()
              .width(Size::fill())
              .vertical()
              .padding(Gaps::new_symmetric(6.0, 0.0))
              .child(label().text("Dirección"))
              .child(Input::new(direccion).width(Size::fill())),
          )
          .child(
            rect()
              .width(Size::fill())
              .margin(Gaps::new(12.0, 0.0, 0.0, 0.0))
              .child(
                Button::new()
                  .on_press(move |_| {
                    let t = telefono.read();
                    let c = correo.read();
                    let d = direccion.read();
                    match Socio::new(
                      nombre.read().clone(),
                      apellidos.read().clone(),
                      dni.read().clone(),
                      (!t.is_empty()).then(|| t.clone()),
                      (!c.is_empty()).then(|| c.clone()),
                      (!d.is_empty()).then(|| d.clone()),
                    ) {
                      Ok(socio) => {
                        spawn(async move {
                          let a = CasoRegistrarSocio::new((*contenedor.read().socio_repo).clone())
                            .ejecutar(socio)
                            .await
                            .inspect_err(|e| tracing::info!("Error: {e}"));
                          *pantalla_socio_actual.write() = PantallaSocio::Tabla;
                        });
                      }
                      Err(e) => {
                        tracing::warn!("No se registro nuevo socio, razon: {e}");
                        *error_dato_invalido_socio.write() = e;
                      }
                    };
                  })
                  .child(
                    rect()
                      .width(Size::fill())
                      .padding(Gaps::new_symmetric(4.0, 0.0))
                      .center()
                      .child(label().text("Registrar Socio")),
                  ),
              ),
          )
          .maybe_child(alerta(error_dato_invalido_socio)),
      )
  }
}

fn alerta(resultado: State<SocioInvalido>) -> Option<impl IntoElement> {
  let resultado = *resultado.read();
  if resultado == SocioInvalido::SIN_ERROR {
    return None;
  }
  let mut mensaje = "Hay campos invalidos:".to_string();
  if resultado.contains(SocioInvalido::NOMBRE) {
    mensaje.push_str("\n- El/los nombre/s debe/n ocupar menos de 50 caracteres");
  }
  if resultado.contains(SocioInvalido::APELLIDO) {
    mensaje.push_str(
      "\n- Los apellidos deben ocupar menos de 50 caracteres, y tener un espacio de separacion.",
    );
  }
  if resultado.contains(SocioInvalido::DNI) {
    mensaje.push_str("\n- El dni debe tener exactamente 8 numeros.");
  }
  if resultado.contains(SocioInvalido::TELEFONO) {
    mensaje.push_str("\n- El telefono debe tener exactamente 9 numeros");
  }
  if resultado.contains(SocioInvalido::CORREO) {
    mensaje.push_str("\n- El correo debe tener un '@' y un '.'");
  }
  if resultado.contains(SocioInvalido::DIRECCION) {
    mensaje.push_str("\n- La direccion no puede ser mas de 150 de largo. Los caracteres no ascii pueden ocupar de un espacio.");
  }
  Some(mensaje_alerta(mensaje))
}
