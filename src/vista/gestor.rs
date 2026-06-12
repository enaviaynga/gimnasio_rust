use freya::{prelude::*, router::*};

use crate::vista::temas::{TemasClaros, TemasOscuros, tema_claro, tema_oscuro};

#[derive(PartialEq)]
pub struct ContenedorGeneral {}
impl Component for ContenedorGeneral {
    fn render(&self) -> impl IntoElement {
        let mut tema_general = use_theme();

        let tema_oscuro_selecionado = use_state(|| TemasOscuros::Generico);
        use_provide_context(|| tema_oscuro_selecionado);

        let tema_claro_selecionado = use_state(|| TemasClaros::Generico);
        use_provide_context(|| tema_claro_selecionado);

        let nombre_tema = tema_general.read().name;
        let es_tema_oscuro = nombre_tema == "oscuro";

        rect()
            .spacing(6.)
            .padding(Gaps::new_all(4.))
            .child(
                rect()
                    .direction(Direction::Horizontal)
                    .child(
                        rect()
                            .horizontal()
                            .child(
                                Button::new()
                                    .on_press(move |_| {
                                        let _ = RouterContext::get().replace(Rutas::Inicio);
                                    })
                                    .child("Inicio"),
                            )
                            .child(
                                Button::new()
                                    .on_press(move |_| {
                                        let _ = RouterContext::get().replace(Rutas::Configuracion);
                                    })
                                    .child("Config"),
                            )
                            .child(
                                Button::new()
                                    .on_press(move |_| {
                                        let _ = RouterContext::get().replace(Rutas::TemaSelector);
                                    })
                                    .child("Temas"),
                            ),
                    )
                    .child(
                        rect()
                            .position(Position::new_absolute().right(0.5))
                            .horizontal()
                            .center()
                            .child((if es_tema_oscuro { "🌘" } else { "☀️" }).to_string())
                            .child(Switch::new().toggled(es_tema_oscuro).on_toggle(move |_| {
                                if es_tema_oscuro {
                                    tema_general.set(tema_claro(tema_claro_selecionado));
                                } else {
                                    tema_general.set(tema_oscuro(tema_oscuro_selecionado));
                                }
                            })),
                    ),
            )
            .child(
                rect()
                    .border(
                        Border::new()
                            .alignment(BorderAlignment::Inner)
                            .width(3.0)
                            .fill(tema_general.read().colors.border),
                    )
                    .center()
                    .expanded()
                    .child(Outlet::<Rutas>::new()),
            )
    }
}

#[derive(PartialEq)]
pub struct Inicio {}
impl Component for Inicio {
    fn render(&self) -> impl IntoElement {
        rect().child("Mostrando inicio")
    }
}

#[derive(PartialEq)]
pub struct Configuracion {}
impl Component for Configuracion {
    fn render(&self) -> impl IntoElement {
        rect().child("Mostrando configuracion")
    }
}

#[derive(PartialEq)]
pub struct TemaSelector {}
impl Component for TemaSelector {
    fn render(&self) -> impl IntoElement {
        let mut tema_oscuro_selecionado: State<TemasOscuros> = use_consume();
        let mut tema_claro_selecionado: State<TemasClaros> = use_consume();

        let mut tema = use_theme();
        rect()
            .horizontal()
            .child(
                rect()
                    .child(
                        Tile::new()
                            .on_select(move |_| {
                                *tema_oscuro_selecionado.write() = TemasOscuros::Generico;
                                tema.set(tema_oscuro(tema_oscuro_selecionado));
                            })
                            .child(Checkbox::new().selected(
                                *tema_oscuro_selecionado.read() == TemasOscuros::Generico,
                            ))
                            .leading("Generico"),
                    )
                    .child(
                        Tile::new()
                            .on_select(move |_| {
                                *tema_oscuro_selecionado.write() = TemasOscuros::Nord;
                                tema.set(tema_oscuro(tema_oscuro_selecionado));
                            })
                            .child(
                                Checkbox::new().selected(
                                    *tema_oscuro_selecionado.read() == TemasOscuros::Nord,
                                ),
                            )
                            .leading("Nord"),
                    )
                    .child(
                        Tile::new()
                            .on_select(move |_| {
                                *tema_oscuro_selecionado.write() = TemasOscuros::Premium;
                                tema.set(tema_oscuro(tema_oscuro_selecionado));
                            })
                            .child(
                                Checkbox::new().selected(
                                    *tema_oscuro_selecionado.read() == TemasOscuros::Premium,
                                ),
                            )
                            .leading("Premiun"),
                    )
                    .child(
                        Tile::new()
                            .on_select(move |_| {
                                *tema_oscuro_selecionado.write() = TemasOscuros::Cozy;
                                tema.set(tema_oscuro(tema_oscuro_selecionado));
                            })
                            .child(
                                Checkbox::new().selected(
                                    *tema_oscuro_selecionado.read() == TemasOscuros::Cozy,
                                ),
                            )
                            .leading("Cozy"),
                    )
                    .child(
                        Tile::new()
                            .on_select(move |_| {
                                *tema_oscuro_selecionado.write() = TemasOscuros::Espacio;
                                tema.set(tema_oscuro(tema_oscuro_selecionado));
                            })
                            .child(
                                Checkbox::new().selected(
                                    *tema_oscuro_selecionado.read() == TemasOscuros::Espacio,
                                ),
                            )
                            .leading("Espacio"),
                    )
                    .child(
                        Tile::new()
                            .on_select(move |_| {
                                *tema_oscuro_selecionado.write() = TemasOscuros::Matrix;
                                tema.set(tema_oscuro(tema_oscuro_selecionado));
                            })
                            .child(
                                Checkbox::new().selected(
                                    *tema_oscuro_selecionado.read() == TemasOscuros::Matrix,
                                ),
                            )
                            .leading("Matrix"),
                    ),
            )
            .child(
                rect()
                    .child(
                        Tile::new()
                            .on_select(move |_| {
                                *tema_claro_selecionado.write() = TemasClaros::Generico;
                                tema.set(tema_claro(tema_claro_selecionado));
                            })
                            .child(
                                Checkbox::new().selected(
                                    *tema_claro_selecionado.read() == TemasClaros::Generico,
                                ),
                            )
                            .leading("Generico"),
                    )
                    .child(
                        Tile::new()
                            .on_select(move |_| {
                                *tema_claro_selecionado.write() = TemasClaros::Premium;
                                tema.set(tema_claro(tema_claro_selecionado));
                            })
                            .child(
                                Checkbox::new().selected(
                                    *tema_claro_selecionado.read() == TemasClaros::Premium,
                                ),
                            )
                            .leading("Premiun"),
                    )
                    .child(
                        Tile::new()
                            .on_select(move |_| {
                                *tema_claro_selecionado.write() = TemasClaros::Retro;
                                tema.set(tema_claro(tema_claro_selecionado));
                            })
                            .child(
                                Checkbox::new()
                                    .selected(*tema_claro_selecionado.read() == TemasClaros::Retro),
                            )
                            .leading("Retro"),
                    )
                    .child(
                        Tile::new()
                            .on_select(move |_| {
                                *tema_claro_selecionado.write() = TemasClaros::Pastel;
                                tema.set(tema_claro(tema_claro_selecionado));
                            })
                            .child(
                                Checkbox::new().selected(
                                    *tema_claro_selecionado.read() == TemasClaros::Pastel,
                                ),
                            )
                            .leading("Pastel"),
                    )
                    .child(
                        Tile::new()
                            .on_select(move |_| {
                                *tema_claro_selecionado.write() = TemasClaros::Cyber;
                                tema.set(tema_claro(tema_claro_selecionado));
                            })
                            .child(
                                Checkbox::new()
                                    .selected(*tema_claro_selecionado.read() == TemasClaros::Cyber),
                            )
                            .leading("Cyber"),
                    )
                    .child(
                        Tile::new()
                            .on_select(move |_| {
                                *tema_claro_selecionado.write() = TemasClaros::Naturaleza;
                                tema.set(tema_claro(tema_claro_selecionado));
                            })
                            .child(Checkbox::new().selected(
                                *tema_claro_selecionado.read() == TemasClaros::Naturaleza,
                            ))
                            .leading("Naturaleza"),
                    ),
            )
    }
}

#[derive(Routable, Clone, Debug, PartialEq)]
#[rustfmt::skip]
pub enum Rutas {
    #[layout(ContenedorGeneral)]
        #[route("/")]
        Inicio,
        #[route("/config")]
        Configuracion,
#[route("/temas")]
TemaSelector,
}
