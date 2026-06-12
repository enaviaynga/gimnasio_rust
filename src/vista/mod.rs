use freya::{prelude::*, router::*};

use crate::{
    hexagonal::empleado::dominio::empleado::Empleado,
    infraestructura::BDsqlx,
    vista::{
        gestor::Rutas,
        temas::{TemasClaros, tema_claro, tema_oscuro},
    },
};

pub(crate) mod componentes;
pub(crate) mod gestor;
pub(crate) mod login;
pub(crate) mod temas;

pub(crate) fn app(logeado: State<bool>) -> impl IntoElement {
    let bd: State<Option<BDsqlx>> = use_state(|| None::<BDsqlx>);
    let a = use_hook(|| {
        spawn(async move {
            let mut bd = bd;
            let bd_url = std::env::var("BD_url").unwrap_or_default();
            if let Ok(base_mysqlx) = BDsqlx::new(&bd_url).await {
                println!("Base de datos configurada");
                // 3. Actualizamos el estado cuando la conexión termina
                *bd.write() = Some(base_mysqlx);
            } else {
                println!("Fallo en la conexión a la base de datos");
            }
        });
        bd
    });
    use_provide_context(|| bd);

    let tema = tema_claro(use_state(|| TemasClaros::Generico));
    use_provide_theme(|| tema);
    let empleado = use_provide_context(Empleado::new_vacio);

    rect()
        .expanded()
        .theme_background()
        .theme_color()
        .child(if *logeado.read() {
            Router::<Rutas>::new(|| RouterConfig::default().with_initial_path(Rutas::Inicio))
                .into_element()
        } else {
            login::login(logeado).into_element()
        })
}
