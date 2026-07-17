use freya::prelude::*;
use hexagonal_gimnasio::{
  control_asistencia::{
    aplicacion::{
      caso_eliminar_asistencia::CasoEliminarAsistencia,
      caso_obtener_asistencia::CasoObtenerUnaAsistencia,
      caso_registrar_asistencia::CasoRegistrarAsistencia,
    },
    dominio::asistencia::ResulValidacion,
    puertos::asistencia_dto::AsistenciaSocioMembresiaDto,
  },
  membresias::dominio::membresia::EstadoMembresia,
};
use infraestructura::persistencia::mysql::contenedor::ContenedorRepos;

use crate::vista::{
  componentes::boton_cerrar_y_volver_al_inicio::boton_cerrar,
  gestor::asistencia::PantallaAsistencia,
};

#[derive(PartialEq)]
pub struct VerAsistencia;
impl Component for VerAsistencia {
  fn render(&self) -> impl IntoElement {
    let pantalla_selecionada: State<PantallaAsistencia> = use_consume();

    let socio_asistencia: State<Option<AsistenciaSocioMembresiaDto>> = use_consume();
    rect()
      .maybe_child(tarjeta_asistencia_socio(
        socio_asistencia,
        pantalla_selecionada,
      ))
      .child(boton_cerrar(
        pantalla_selecionada,
        PantallaAsistencia::Inicio,
      ))
  }
}

use freya::prelude::*;

fn tarjeta_asistencia_socio(
  socio_asistencia: State<Option<AsistenciaSocioMembresiaDto>>,
  mut pantalla_selecionada: State<PantallaAsistencia>,
) -> Option<impl IntoElement> {
  let datos_read = socio_asistencia.read();
  let s_a = datos_read.as_ref()?;
  let id = s_a.id_asistencia as u64;
  let contenedor: State<ContenedorRepos> = use_consume();

  let alerta_theme_button = ButtonColorsThemePartial {
    background: Some(Preference::Specific(Color::from_rgb(220, 53, 69))),
    hover_background: Some(Preference::Specific(Color::from_rgb(200, 35, 51))),
    border_fill: None,
    focus_border_fill: None,
    color: Some(Preference::Specific(Color::WHITE)),
  };

  Some(
    rect()
      .width(Size::fill())
      .height(Size::fill())
      .center()
      .child(
        rect()
          .vertical()
          .width(Size::percent(90.0))
          .max_width(Size::px(550.))
          .min_width(Size::px(340.))
          .height(Size::auto())
          .padding(Gaps::new_all(20.))
          .spacing(14.)
          .corner_radius(CornerRadius::new_all(12.))
          .child(
            rect().margin(Gaps::new(0., 0., 0., 6.)).child(
              label()
                .text("Resumen de Asistencia e Información del Socio")
                .font_size(15.)
                .font_weight(FontWeight::BOLD),
            ),
          )
          .child(
            rect()
              .horizontal()
              .width(Size::fill())
              .child(item_info("DNI:", s_a.dni_s.clone(), Size::percent(50.0)))
              .child(item_info(
                "Nombre y Apellidos:",
                format!("{} {}", s_a.nombre_s, s_a.apellido_s),
                Size::percent(50.0),
              )),
          )
          .child(
            rect()
              .horizontal()
              .width(Size::fill())
              .child(item_info(
                "Tipo Membresía:",
                s_a.tipo_membresia.clone(),
                Size::percent(50.0),
              ))
              .child(item_info(
                "Costo:",
                format!("S/ {:.2}", s_a.costo),
                Size::percent(50.0),
              )),
          )
          .child(
            rect()
              .horizontal()
              .width(Size::fill())
              .child(item_info(
                "Vigencia:",
                format!("Del {} al {}", s_a.fecha_inicio, s_a.fecha_vencimiento),
                Size::percent(50.0),
              ))
              .child(item_status_info(
                "Estado Membresía:",
                s_a.estado.to_string(),
                s_a.estado,
                Size::percent(50.0),
              )),
          )
          .child(
            rect()
              .width(Size::fill())
              .height(Size::px(1.))
              .margin(Gaps::new_symmetric(10., 0.)),
          )
          .child(
            rect()
              .horizontal()
              .width(Size::fill())
              .child(item_info(
                "ID Asistencia:",
                s_a.id_asistencia.to_string(),
                Size::percent(50.0),
              ))
              .child(item_result_info(
                "Validación:",
                s_a.resultado_validacion.to_string(),
                s_a.resultado_validacion,
                Size::percent(50.0),
              )),
          )
          .child(
            rect()
              .horizontal()
              .child(item_info(
                "Fecha Ingreso:",
                s_a.fecha_asistencia.clone(),
                Size::percent(50.0),
              ))
              .child(item_info(
                "Hora Ingreso:",
                s_a.hora_ingreso.clone(),
                Size::percent(50.0),
              )),
          )
          .child(
            Button::new()
              .child("Eliminar Asistencia")
              .width(Size::fill())
              .padding(Gaps::new_symmetric(8.0, 0.0))
              .theme_colors(alerta_theme_button)
              .on_press(move |_| {
                spawn(async move {
                  if CasoEliminarAsistencia::new((*contenedor.read().asistencia_repo).clone())
                    .ejecutar(id)
                    .await
                    .is_ok()
                  {
                    pantalla_selecionada.set(PantallaAsistencia::Inicio);
                  };
                });
              }),
          ),
      ),
  )
}

fn item_info(titulo: &'static str, valor: String, tamano: Size) -> impl IntoElement {
  rect()
    .width(tamano)
    .padding(Gaps::new_all(8.))
    .spacing(4.)
    .child(label().text(titulo).font_size(12.))
    .child(label().text(valor).font_size(14.))
}

fn item_status_info(
  titulo: &'static str,
  valor: String,
  es_seguro: EstadoMembresia,
  tamano: Size,
) -> impl IntoElement {
  rect()
    .width(tamano)
    .padding(Gaps::new_all(8.))
    .spacing(4.)
    .child(label().text(titulo).font_size(12.))
    .child(
      rect()
        .horizontal()
        .cross_align(Alignment::Center)
        .spacing(6.)
        .child(
          rect()
            .width(Size::px(8.))
            .height(Size::px(8.))
            .background(Color::from(match es_seguro {
              EstadoMembresia::Activo => (40, 167, 69),
              EstadoMembresia::CercaVencimiento => (255, 255, 30),
              EstadoMembresia::Vencido => (220, 53, 69),
            }))
            .corner_radius(CornerRadius::new_all(4.)),
        )
        .child(label().text(valor).font_size(14.)),
    )
}

fn item_result_info(
  titulo: &'static str,
  texto: String,
  resultado: ResulValidacion,
  tamano: Size,
) -> impl IntoElement {
  rect()
    .width(tamano)
    .padding(Gaps::new_all(8.))
    .spacing(4.)
    .child(label().text(titulo).font_size(12.))
    .child(
      rect()
        .horizontal()
        .cross_align(Alignment::Center)
        .spacing(6.)
        .child(
          rect()
            .width(Size::px(8.))
            .height(Size::px(8.))
            .background(Color::from(match resultado {
              ResulValidacion::Permitido => (40, 167, 69),
              ResulValidacion::NoPermitido => (255, 255, 30),
            }))
            .corner_radius(CornerRadius::new_all(4.)),
        )
        .child(label().text(texto).font_size(14.)),
    )
}
