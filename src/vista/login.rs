use freya::prelude::*;

use crate::{
    hexagonal::empleado::{
        dominio::usuario::Usuario, puertos::traits_login::BuscarEmpleadoPorUsuario,
    },
    infraestructura::BDsqlx,
};

pub(crate) fn login(login: State<bool>) -> impl IntoElement {
    let username = use_state(String::new);
    let contraseña = use_state(String::new);
    let login_estado = use_state(|| EstadoLogin::Cerrado);
    let bd_sqlx: State<Option<BDsqlx>> = use_consume();

    let tema = use_theme();

    rect()
        .width(Size::fill())
        .height(Size::fill())
        .center()
        .theme_background()
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
                .child(input_usuario(&username, login_estado))
                .child(input_contraseña(&contraseña, login_estado))
                .child(boton_login(
                    username,
                    contraseña,
                    login_estado,
                    bd_sqlx,
                    login,
                ))
                .maybe_child(alerta(&login_estado)),
        )
}

enum EstadoLogin {
    Cerrado,
    Intentando,
    Fallo,
    Invalido,
    Exito,
}

// --- Título del Formulario ---
fn titulo_login() -> impl IntoElement {
    label()
        .text("Iniciar Sesión")
        .font_size(20.0)
        .font_weight(FontWeight::BOLD)
        .margin(Gaps::new(0.0, 0.0, 16.0, 0.0))
}

// --- Cuadro de texto Usuario ---
fn input_usuario(
    username: &State<String>,
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
            Input::new(*username)
                .on_pre_key_down(move |_| {
                    *login_estado.write() = EstadoLogin::Intentando;
                    true
                })
                .placeholder("Nombre de usuario")
                .width(Size::fill()),
        )
}

// --- Cuadro de texto Contraseña ---
fn input_contraseña(
    contraseña: &State<String>,
    mut login_estado: State<EstadoLogin>,
) -> impl IntoElement {
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
            Input::new(*contraseña)
                .on_pre_key_down(move |_| {
                    *login_estado.write() = EstadoLogin::Intentando;
                    true
                })
                .placeholder("••••••••")
                .width(Size::fill())
                .mode(InputMode::Hidden('*')),
        )
}

// Boton login/acceso
fn boton_login(
    username: State<String>,
    contraseña: State<String>,
    mut login_estado: State<EstadoLogin>,
    bd_sqlx: State<Option<BDsqlx>>,
    mut login: State<bool>,
) -> impl IntoElement {
    rect().width(Size::fill()).center().child(
        Button::new()
            .on_press(move |_| {
                // pendiente añadir un block si la base de datos se demora en responder
                // no se si lo necesite al final pero lo dejo anotadp
                let u = username.read().clone();
                let c = contraseña.read().clone();

                if u.trim().is_empty() || c.trim().is_empty() {
                    *login_estado.write() = EstadoLogin::Fallo;
                } else {
                    let usuario = match Usuario::new(u.clone(), c.clone()) {
                        Ok(ok) => ok,
                        Err(_) => {
                            *login_estado.write() = EstadoLogin::Invalido;
                            return;
                        }
                    };
                    spawn(async move {
                        if let Some(bd_sqlx) = bd_sqlx.read().clone() {
                            // pendiente, usar este empleado para pasarlo al contexto y usarlo en el router
                            let _empleado = match bd_sqlx.login(usuario).await {
                                Ok(ok) => {
                                    println!("Empleado accedio con exito: {ok:?}");
                                    *login_estado.write() = EstadoLogin::Exito;
                                    login.toggle();
                                    ok
                                }
                                Err(e) => {
                                    println!("Error al acceder: {e:?}");
                                    *login_estado.write() = EstadoLogin::Invalido;
                                    return;
                                }
                            };
                        }
                    });
                }
            })
            .child(label().text("Ingresar").font_weight(FontWeight::BOLD)),
    )
}

// pendiente, usar match para un manejo mas elegante
// Alerta de error
fn alerta(login_estado: &State<EstadoLogin>) -> Option<impl IntoElement> {
    match *login_estado.read() {
        EstadoLogin::Cerrado => None,
        EstadoLogin::Intentando => None,
        EstadoLogin::Fallo => Some(mensaje_alerta("Ha ocurrido un fallo.")),
        EstadoLogin::Invalido => Some(mensaje_alerta("Usuario o contraseña incorrectos.")),
        EstadoLogin::Exito => None,
    }
}

fn mensaje_alerta(mensaje: &str) -> impl IntoElement {
    rect()
        .width(Size::fill())
        .margin(Gaps::new(14.0, 0.0, 0.0, 0.0))
        .padding(Gaps::new_all(10.0))
        .corner_radius(CornerRadius::new_all(6.0))
        .background(Color::from_rgb(254, 226, 226))
        .border(Border::new().width(1.0).fill(Color::from_rgb(239, 68, 68)))
        .child(
            label()
                .text(mensaje.to_string())
                .font_size(13.0)
                .color(Color::from_rgb(185, 28, 28)),
        )
}
