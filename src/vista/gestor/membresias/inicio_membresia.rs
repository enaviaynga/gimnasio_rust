use freya::prelude::*;

use crate::vista::gestor::membresias::PantallaMembresia;

#[derive(PartialEq)]
pub struct InicioMembresiaGui;
impl Component for InicioMembresiaGui {
  fn render(&self) -> impl IntoElement {
    let mut pantalla_membresia_selecionada: State<PantallaMembresia> = use_consume();

    rect().expanded().center().child(
      rect()
        .center()
        .spacing(10.)
        .width(Size::percent(55.))
        .height(Size::percent(70.))
        .content(Content::Flex)
        .child(
          rect()
            .spacing(10.)
            .horizontal()
            .height(Size::flex(1.))
            .content(Content::Flex)
            .child(
              Button::new()
                .width(Size::flex(1.))
                .height(Size::flex(1.))
                .child("Registrar Membresia")
                .on_press(move |_| pantalla_membresia_selecionada.set(PantallaMembresia::Crear)),
            )
            .child(
              Button::new()
                .width(Size::flex(1.))
                .height(Size::flex(1.))
                .child("Listar Membresias")
                .on_press(move |_| pantalla_membresia_selecionada.set(PantallaMembresia::Listar)),
            ),
        )
        .child(
          rect()
            .horizontal()
            .spacing(10.)
            .height(Size::flex(1.))
            .content(Content::Flex)
            .child(
              Button::new()
                .width(Size::flex(1.))
                .height(Size::flex(1.))
                .child("Revisar Valides de Membresias")
                .on_press(move |_| pantalla_membresia_selecionada.set(PantallaMembresia::Validar)),
            )
            .child(
              Button::new()
                .width(Size::flex(1.))
                .height(Size::flex(1.))
                .child("Buscar y Renovar Membresia")
                .on_press(move |_| pantalla_membresia_selecionada.set(PantallaMembresia::Renovar)),
            ),
        ),
    )
  }
}
