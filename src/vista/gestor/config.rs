use freya::{prelude::*, router::*};

use crate::vista::{
  configuracion::ConfiguracionGim,
  temas::{
    TemasClaros, TemasOscuros, tema_claro, tema_claro_no_state, tema_oscuro, tema_oscuro_no_state,
  },
};

#[derive(PartialEq)]
pub struct TemaSelector {}
impl Component for TemaSelector {
  fn render(&self) -> impl IntoElement {
    let mut conf: State<ConfiguracionGim> = use_consume();
    // let mut tema_oscuro_selecionado: State<TemasOscuros> = use_consume();
    // let mut tema_claro_selecionado: State<TemasClaros> = use_consume();

    let mut tema = use_theme();
    rect()
      .horizontal()
      .child(
        rect()
          .child(
            Tile::new()
              .on_select(move |_| {
                conf.write().tema_oscuro = TemasOscuros::Generico;
                tema.set(tema_oscuro_no_state(conf.read().tema_oscuro));
              })
              .child(Checkbox::new().selected(conf.read().tema_oscuro == TemasOscuros::Generico))
              .leading("Generico"),
          )
          .child(
            Tile::new()
              .on_select(move |_| {
                conf.write().tema_oscuro = TemasOscuros::Nord;
                tema.set(tema_oscuro_no_state(conf.read().tema_oscuro));
              })
              .child(Checkbox::new().selected(conf.read().tema_oscuro == TemasOscuros::Nord))
              .leading("Nord"),
          )
          .child(
            Tile::new()
              .on_select(move |_| {
                conf.write().tema_oscuro = TemasOscuros::Premium;
                tema.set(tema_oscuro_no_state(conf.read().tema_oscuro));
              })
              .child(Checkbox::new().selected(conf.read().tema_oscuro == TemasOscuros::Premium))
              .leading("Premiun"),
          )
          .child(
            Tile::new()
              .on_select(move |_| {
                conf.write().tema_oscuro = TemasOscuros::Cozy;
                tema.set(tema_oscuro_no_state(conf.read().tema_oscuro));
              })
              .child(Checkbox::new().selected(conf.read().tema_oscuro == TemasOscuros::Cozy))
              .leading("Cozy"),
          )
          .child(
            Tile::new()
              .on_select(move |_| {
                conf.write().tema_oscuro = TemasOscuros::Espacio;
                tema.set(tema_oscuro_no_state(conf.read().tema_oscuro));
              })
              .child(Checkbox::new().selected(conf.read().tema_oscuro == TemasOscuros::Espacio))
              .leading("Espacio"),
          )
          .child(
            Tile::new()
              .on_select(move |_| {
                conf.write().tema_oscuro = TemasOscuros::Matrix;
                tema.set(tema_oscuro_no_state(conf.read().tema_oscuro));
              })
              .child(Checkbox::new().selected(conf.read().tema_oscuro == TemasOscuros::Matrix))
              .leading("Matrix"),
          ),
      )
      .child(
        rect()
          .child(
            Tile::new()
              .on_select(move |_| {
                conf.write().tema_claro = TemasClaros::Generico;
                tema.set(tema_claro_no_state(conf.read().tema_claro));
              })
              .child(Checkbox::new().selected(conf.read().tema_claro == TemasClaros::Generico))
              .leading("Generico"),
          )
          .child(
            Tile::new()
              .on_select(move |_| {
                conf.write().tema_claro = TemasClaros::Premium;
                tema.set(tema_claro_no_state(conf.read().tema_claro));
              })
              .child(Checkbox::new().selected(conf.read().tema_claro == TemasClaros::Premium))
              .leading("Premiun"),
          )
          .child(
            Tile::new()
              .on_select(move |_| {
                conf.write().tema_claro = TemasClaros::Retro;
                tema.set(tema_claro_no_state(conf.read().tema_claro));
              })
              .child(Checkbox::new().selected(conf.read().tema_claro == TemasClaros::Retro))
              .leading("Retro"),
          )
          .child(
            Tile::new()
              .on_select(move |_| {
                conf.write().tema_claro = TemasClaros::Pastel;
                tema.set(tema_claro_no_state(conf.read().tema_claro));
              })
              .child(Checkbox::new().selected(conf.read().tema_claro == TemasClaros::Pastel))
              .leading("Pastel"),
          )
          .child(
            Tile::new()
              .on_select(move |_| {
                conf.write().tema_claro = TemasClaros::Cyber;
                tema.set(tema_claro_no_state(conf.read().tema_claro));
              })
              .child(Checkbox::new().selected(conf.read().tema_claro == TemasClaros::Cyber))
              .leading("Cyber"),
          )
          .child(
            Tile::new()
              .on_select(move |_| {
                conf.write().tema_claro = TemasClaros::Naturaleza;
                tema.set(tema_claro_no_state(conf.read().tema_claro));
              })
              .child(Checkbox::new().selected(conf.read().tema_claro == TemasClaros::Naturaleza))
              .leading("Naturaleza"),
          ),
      )
      .child(Button::new().child("Guardar").on_press(move |_| {
        if let Err(e) = conf.read().guardar() {
          tracing::warn!("Error al guardar configuracion: {e}")
        };
      }))
  }
}
// #[derive(PartialEq)]
// pub struct TemaSelector {}
// impl Component for TemaSelector {
//   fn render(&self) -> impl IntoElement {
//     let mut tema_oscuro_selecionado: State<TemasOscuros> = use_consume();
//     let mut tema_claro_selecionado: State<TemasClaros> = use_consume();

//     let mut tema = use_theme();
//     rect()
//       .horizontal()
//       .child(
//         rect()
//           .child(
//             Tile::new()
//               .on_select(move |_| {
//                 conf.write().tema_oscuro = TemasOscuros::Generico;
//                 tema.set(tema_oscuro(tema_oscuro_selecionado));
//               })
//               .child(
//                 Checkbox::new().selected(*tema_oscuro_selecionado.read() == TemasOscuros::Generico),
//               )
//               .leading("Generico"),
//           )
//           .child(
//             Tile::new()
//               .on_select(move |_| {
//                 conf.write().tema_oscuro = TemasOscuros::Nord;
//                 tema.set(tema_oscuro(tema_oscuro_selecionado));
//               })
//               .child(
//                 Checkbox::new().selected(*tema_oscuro_selecionado.read() == TemasOscuros::Nord),
//               )
//               .leading("Nord"),
//           )
//           .child(
//             Tile::new()
//               .on_select(move |_| {
//                 conf.write().tema_oscuro = TemasOscuros::Premium;
//                 tema.set(tema_oscuro(tema_oscuro_selecionado));
//               })
//               .child(
//                 Checkbox::new().selected(*tema_oscuro_selecionado.read() == TemasOscuros::Premium),
//               )
//               .leading("Premiun"),
//           )
//           .child(
//             Tile::new()
//               .on_select(move |_| {
//                 conf.write().tema_oscuro = TemasOscuros::Cozy;
//                 tema.set(tema_oscuro(tema_oscuro_selecionado));
//               })
//               .child(
//                 Checkbox::new().selected(*tema_oscuro_selecionado.read() == TemasOscuros::Cozy),
//               )
//               .leading("Cozy"),
//           )
//           .child(
//             Tile::new()
//               .on_select(move |_| {
//                 conf.write().tema_oscuro = TemasOscuros::Espacio;
//                 tema.set(tema_oscuro(tema_oscuro_selecionado));
//               })
//               .child(
//                 Checkbox::new().selected(*tema_oscuro_selecionado.read() == TemasOscuros::Espacio),
//               )
//               .leading("Espacio"),
//           )
//           .child(
//             Tile::new()
//               .on_select(move |_| {
//                 conf.write().tema_oscuro = TemasOscuros::Matrix;
//                 tema.set(tema_oscuro(tema_oscuro_selecionado));
//               })
//               .child(
//                 Checkbox::new().selected(*tema_oscuro_selecionado.read() == TemasOscuros::Matrix),
//               )
//               .leading("Matrix"),
//           ),
//       )
//       .child(
//         rect()
//           .child(
//             Tile::new()
//               .on_select(move |_| {
//                 *tema_claro_selecionado.write() = TemasClaros::Generico;
//                 tema.set(tema_claro(tema_claro_selecionado));
//               })
//               .child(
//                 Checkbox::new().selected(*tema_claro_selecionado.read() == TemasClaros::Generico),
//               )
//               .leading("Generico"),
//           )
//           .child(
//             Tile::new()
//               .on_select(move |_| {
//                 *tema_claro_selecionado.write() = TemasClaros::Premium;
//                 tema.set(tema_claro(tema_claro_selecionado));
//               })
//               .child(
//                 Checkbox::new().selected(*tema_claro_selecionado.read() == TemasClaros::Premium),
//               )
//               .leading("Premiun"),
//           )
//           .child(
//             Tile::new()
//               .on_select(move |_| {
//                 *tema_claro_selecionado.write() = TemasClaros::Retro;
//                 tema.set(tema_claro(tema_claro_selecionado));
//               })
//               .child(Checkbox::new().selected(*tema_claro_selecionado.read() == TemasClaros::Retro))
//               .leading("Retro"),
//           )
//           .child(
//             Tile::new()
//               .on_select(move |_| {
//                 *tema_claro_selecionado.write() = TemasClaros::Pastel;
//                 tema.set(tema_claro(tema_claro_selecionado));
//               })
//               .child(
//                 Checkbox::new().selected(*tema_claro_selecionado.read() == TemasClaros::Pastel),
//               )
//               .leading("Pastel"),
//           )
//           .child(
//             Tile::new()
//               .on_select(move |_| {
//                 *tema_claro_selecionado.write() = TemasClaros::Cyber;
//                 tema.set(tema_claro(tema_claro_selecionado));
//               })
//               .child(Checkbox::new().selected(*tema_claro_selecionado.read() == TemasClaros::Cyber))
//               .leading("Cyber"),
//           )
//           .child(
//             Tile::new()
//               .on_select(move |_| {
//                 *tema_claro_selecionado.write() = TemasClaros::Naturaleza;
//                 tema.set(tema_claro(tema_claro_selecionado));
//               })
//               .child(
//                 Checkbox::new().selected(*tema_claro_selecionado.read() == TemasClaros::Naturaleza),
//               )
//               .leading("Naturaleza"),
//           ),
//       )
//   }
// }
