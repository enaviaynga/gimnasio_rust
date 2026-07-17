use freya::prelude::*;

pub fn boton_cerrar<T>(mut pantalla_actual: State<T>, pantalla_inicial: T) -> impl IntoElement
where
  T: Clone + 'static,
{
  rect()
    .position(Position::new_absolute().top(10.).right(10.))
    .child(
      Button::new()
        .child("Cerrar")
        .on_press(move |_| pantalla_actual.set(pantalla_inicial.clone())),
    )
}
