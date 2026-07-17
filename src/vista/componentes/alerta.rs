use freya::prelude::*;

pub fn mensaje_alerta(mensaje: String) -> impl IntoElement {
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
