use freya::prelude::*;

use hexagonal_gimnasio::empleado::{
  aplicacion::caso_login::CasoLogin,
  dominio::{empleado::EmpleadoEnum, usuario::Usuario},
};
use infraestructura::persistencia::mysql::contenedor::ContenedorRepos;

use crate::vista::componentes::alerta::mensaje_alerta;

pub(crate) fn login() -> impl IntoElement {
  let username = use_state(|| "admin_carlos".to_string());
  let contraseña = use_state(|| "Admin-123".to_string());
  let login_estado: State<EstadoLogin> = use_consume();
  let contenedor_repos: State<ContenedorRepos> = use_consume();

  let tema = use_theme();

  rect()
    .width(Size::fill())
    .height(Size::fill())
    .center()
    .child(
      rect()
        .width(Size::percent(50.0))
        .min_width(Size::px(300.0))
        .max_width(Size::px(500.0))
        .height(Size::auto())
        .padding(Gaps::new_all(24.0))
        .vertical()
        .corner_radius(CornerRadius::new_all(12.0))
        .background(tema.read().colors.background)
        .border(
          Border::new()
            .alignment(BorderAlignment::Inner)
            .width(1.0)
            .fill(tema.read().colors.border),
        )
        .child(titulo_login())
        .child(input_usuario(username, login_estado))
        .child(input_contraseña(contraseña, login_estado))
        .child(boton_login(
          username,
          contraseña,
          login_estado,
          contenedor_repos,
        ))
        .maybe_child(alerta(&login_estado)),
    )
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) enum EstadoLogin {
  Cerrado,
  Intentando,
  Fallo,
  Invalido,
  Exito,
}

fn titulo_login() -> impl IntoElement {
  label()
    .text("Iniciar Sesión")
    .font_size(20.0)
    .font_weight(FontWeight::BOLD)
    .margin(Gaps::new(0.0, 0.0, 16.0, 0.0))
}

fn input_usuario(
  username: State<String>,
  mut login_estado: State<EstadoLogin>,
) -> impl IntoElement {
  rect()
    .width(Size::fill())
    .vertical()
    .margin(Gaps::new(0.0, 0.0, 12.0, 0.0))
    .child(
      label()
        .text("Usuario")
        .font_size(14.0)
        .margin(Gaps::new(0.0, 0.0, 6.0, 0.0)),
    )
    .child(
      // lastimosamente la libreria no incluye la notificacion de desplazasamiento,
      // solo funciona con la rueda del raton, pendiente crea una moficacion
      Input::new(username)
        .on_pre_key_down(move |_| {
          *login_estado.write() = EstadoLogin::Intentando;
          true
        })
        .auto_focus(true)
        .placeholder("Nombre de usuario")
        .width(Size::fill()),
    )
}

fn input_contraseña(
  contraseña: State<String>,
  mut login_estado: State<EstadoLogin>,
) -> impl IntoElement {
  tracing::trace!("input de contraseña mostrado.");
  let mut visible = use_state(|| false);
  rect()
    .width(Size::fill())
    .vertical()
    .margin(Gaps::new(0.0, 0.0, 20.0, 0.0))
    .child(
      label()
        .text("Contraseña")
        .font_size(14.0)
        .margin(Gaps::new(0.0, 0.0, 6.0, 0.0)),
    )
    .child(
      rect()
        .content(Content::Flex)
        .width(Size::fill())
        .height(Size::px(34.))
        .horizontal()
        .child(
          Input::new(contraseña)
            .on_pre_key_down(move |_| {
              *login_estado.write() = EstadoLogin::Intentando;
              true
            })
            .placeholder("••••••••")
            .width(Size::flex(1.))
            .mode(if *visible.read() {
              InputMode::Shown
            } else {
              InputMode::Hidden('*')
            }),
        )
        .child(
          // no tocar
          rect()
            .width(Size::px(40.))
            .height(Size::fill())
            .padding(Gaps::new_all(5.))
            .center()
            .child(
              // revisar el ejemplo component_input_form.rs ahi usa iconos
              Button::new()
                .expanded()
                .padding(Gaps::new_all(8.))
                .on_press(move |_| visible.toggle())
                .child(
                  SvgViewer::new(if *visible.read() {
                    freya::icons::lucide::eye()
                  } else {
                    freya::icons::lucide::eye_closed()
                  })
                  .width(Size::px(16.))
                  .height(Size::px(16.)),
                ),
            ),
        ),
    )
}

fn boton_login(
  username: State<String>,
  contraseña: State<String>,
  mut login_estado: State<EstadoLogin>,
  contenedor_repos: State<ContenedorRepos>,
) -> impl IntoElement {
  let mut empleado_global: State<EmpleadoEnum> = use_consume();
  rect()
    .padding(Gaps::new_all(10.))
    .width(Size::fill())
    .center()
    .child(
      Button::new()
        .on_press(move |_| {
          let u = username.read().clone();
          let c = contraseña.read().clone();

          if u.trim().is_empty() || c.trim().is_empty() {
            *login_estado.write() = EstadoLogin::Fallo;
          } else {
            match Usuario::new(u.clone(), c.clone()) {
              Ok(usuario) => {
                // spawn(async move {
                //   let caso_login = CasoLogin::new((*contenedor_repos.read().empleado_repo).clone());
                //   let empleado = match caso_login.ejecutar(usuario).await {
                //     Ok(ok) => {
                //       if let EmpleadoEnum::Activo(empl) = &ok {
                //         tracing::debug!("Empleado accedio con exito: {empl:?}");
                //         *login_estado.write() = EstadoLogin::Exito;
                //         ok
                //       } else {
                //         return;
                //       }
                //     }
                //     Err(e) => {
                //       tracing::debug!("Error al acceder: {e:?}");
                //       *login_estado.write() = EstadoLogin::Invalido;
                //       return;
                //     }
                //   };
                //   tracing::debug!("Cambiando el empleado default");
                //   *empleado_global.write() = empleado;
                // });
                // comentar lo anterior y descomentar lo siguiente para presentar al docente:
                *login_estado.write() = EstadoLogin::Exito;
              }
              Err(e) => {
                *login_estado.write() = EstadoLogin::Invalido;
                tracing::debug!("usuario no valido: {e}");
                // pendiente usar e para extraer lo que falta
              }
            };
          }
        })
        .child(label().text("Ingresar").font_weight(FontWeight::BOLD)),
    )
}

// pendiente, usar match para un manejo mas elegante
fn alerta(login_estado: &State<EstadoLogin>) -> Option<impl IntoElement> {
  match *login_estado.read() {
    EstadoLogin::Cerrado => None,
    EstadoLogin::Intentando => None,
    EstadoLogin::Fallo => Some(mensaje_alerta("Ha ocurrido un fallo.".to_owned())),
    EstadoLogin::Invalido => Some(mensaje_alerta(
      "Usuario o contraseña incorrectos.".to_owned(),
    )),
    EstadoLogin::Exito => None,
  }
}
