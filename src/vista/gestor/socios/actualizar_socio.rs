use std::sync::Arc;

use freya::prelude::*;

use crate::vista::{
  componentes::{alerta::mensaje_alerta, boton_cerrar_y_volver_al_inicio::boton_cerrar},
  gestor::socios::PantallaSocio,
};
use hexagonal_gimnasio::gestion_socio::{
  aplicacion::{
    caso_actualizar_socio::CasoActualizarSocio, caso_eliminar_socio::CasoEliminarSocio,
  },
  dominio::socio::{Socio, SocioInvalido},
};
use infraestructura::{
  persistencia::mysql::{contenedor::ContenedorRepos, socio},
  reportes::eliminacion::rpty_elim_socio::generar_reporte_previo_eliminacion_socio,
};

#[derive(PartialEq)]
pub struct ActualizarSocioGui;

impl Component for ActualizarSocioGui {
  fn render(&self) -> impl IntoElement {
    let mut pantalla_socio_actual: State<PantallaSocio> = use_consume();
    let socio: State<Socio> = use_consume();
    let mut error_dato_invalido_socio = use_state(|| SocioInvalido::SIN_ERROR);
    let mut esta_copiado = use_state(|| false);

    let contenedor: State<ContenedorRepos> = use_consume();

    let mut id = use_state(|| 0);
    let mut telefono = use_state(String::new);
    let mut correo = use_state(String::new);
    let mut direccion = use_state(String::new);

    if !*esta_copiado.read() {
      tracing::debug!("{:?}", socio.read());
      let socio = socio.read();
      if let Some(id_s) = socio.get_id() {
        *id.write() = id_s;
      }
      if let Some(t) = socio.get_telefono() {
        *telefono.write() = t.to_string()
      };
      if let Some(c) = socio.get_correo() {
        *correo.write() = c.to_string()
      };
      if let Some(d) = socio.get_direccion() {
        *direccion.write() = d.to_string()
      };
      esta_copiado.toggle();
    };

    let alerta_theme_button = ButtonColorsThemePartial {
      background: Some(Preference::Specific(Color::from_rgb(220, 53, 69))),
      hover_background: Some(Preference::Specific(Color::from_rgb(200, 35, 51))),
      border_fill: None,
      focus_border_fill: None,
      color: Some(Preference::Specific(Color::WHITE)),
    };

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
              .text("Editar Datos de Contacto")
              .font_size(24.0)
              .font_weight(FontWeight::BOLD),
          )
          .child(
            rect()
              .width(Size::fill())
              .vertical()
              .padding(Gaps::new_symmetric(6.0, 0.0))
              .child(label().text("Teléfono"))
              .child(Input::new(telefono).width(Size::fill())),
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
              .child(label().text("Dirección de Residencia"))
              .child(Input::new(direccion).width(Size::fill())),
          )
          .child(
            rect()
              .width(Size::fill())
              .margin(Gaps::new(12.0, 0.0, 0.0, 0.0))
              .content(Content::Flex)
              .spacing(10.)
              .horizontal()
              .child(
                Button::new()
                  .width(Size::flex(2.))
                  .on_press(move |_| {
                    spawn(async move {
                      let telefono = telefono.read();
                      let correo = correo.read();
                      let direccion = direccion.read();
                      tracing::debug!(
                        "id: {} | t: {} | c: {} | d: {}",
                        id.read(),
                        telefono,
                        correo,
                        direccion
                      );
                      let mut errores = SocioInvalido::SIN_ERROR;
                      let mut socio = socio.read().clone();
                      if correo.is_empty() {
                        socio.set_correo_none()
                      } else {
                        if let Err(e) = socio.set_correo(correo.clone()) {
                          errores |= e;
                        }
                      };
                      if direccion.is_empty() {
                        socio.set_direccion_none();
                      } else {
                        if let Err(e) = socio.set_direccion(direccion.clone()) {
                          errores |= e;
                        }
                      };
                      if telefono.is_empty() {
                        socio.set_telefono_none();
                      } else {
                        if let Err(e) = socio.set_telefono(telefono.clone()) {
                          errores |= e;
                        }
                      };
                      *error_dato_invalido_socio.write() = errores;
                      if errores != SocioInvalido::SIN_ERROR {
                        return;
                      }
                      let _ = CasoActualizarSocio::new((*contenedor.read().socio_repo).clone())
                        .ejecutar(socio)
                        .await
                        .map_err(|e| tracing::info!("{e}"));
                      *pantalla_socio_actual.write() = PantallaSocio::Tabla
                    });
                  })
                  .child(
                    rect()
                      .width(Size::fill())
                      .padding(Gaps::new_symmetric(6.0, 0.0))
                      .center()
                      .child(label().text("Subir Cambios")),
                  ),
              )
              .child(
                Button::new()
                  .width(Size::flex(1.))
                  .theme_colors(alerta_theme_button)
                  .on_press(move |_| {
                    spawn(async move {
                      if let Some(id) = socio.read().get_id() {
                        let a = CasoEliminarSocio::new((*contenedor.read().socio_repo).clone())
                          .ejecutar(id)
                          .await
                          .inspect_err(|e| tracing::warn!("{}", e));
                        *pantalla_socio_actual.write() = PantallaSocio::Tabla;
                      }
                    });
                  })
                  .child(
                    rect()
                      .width(Size::fill())
                      .padding(Gaps::new_symmetric(6.0, 0.0))
                      .center()
                      .child(label().text("Eliminar")),
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
  if resultado.contains(SocioInvalido::TELEFONO) {
    mensaje.push_str("\n- El telefono debe tener 9 de largo, y ser solo numeros");
  }
  if resultado.contains(SocioInvalido::DIRECCION) {
    mensaje.push_str("\n- La direccion no puede ser mas de 150 de largo. Los caracteres no ascii pueden ocupar de un espacio.");
  }
  if resultado.contains(SocioInvalido::CORREO) {
    mensaje.push_str("\n- El correo debe tener un '@' y un '.'");
  }
  Some(mensaje_alerta(mensaje))
}
