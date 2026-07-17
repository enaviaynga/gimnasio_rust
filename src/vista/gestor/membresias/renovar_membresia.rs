use chrono::{Datelike, Local, NaiveDate};
use freya::prelude::*;
use hexagonal_gimnasio::membresias::{
  aplicacion::{
    caso_obtener_membresia::CasoBuscarMembresiaPorSocio,
    caso_renovar_membresia::CasoRenovarMembresia,
  },
  dominio::membresia::{EstadoMembresia, Membresia, TipoMembresia},
  puertos::membresias_dto::MembresiaSocioDto,
};
use infraestructura::persistencia::mysql::contenedor::ContenedorRepos;

use crate::vista::{
  componentes::boton_cerrar_y_volver_al_inicio::boton_cerrar, gestor::membresias::PantallaMembresia,
};

#[derive(PartialEq)]
pub struct RenovarMembresia;
impl Component for RenovarMembresia {
  fn render(&self) -> impl IntoElement {
    let pantalla_actual: State<PantallaMembresia> = use_consume();

    rect()
      .width(Size::fill())
      .height(Size::fill())
      .center()
      .child(boton_cerrar(pantalla_actual, PantallaMembresia::Inicio))
      .child(rect().child(modificar_membresia_gui()))
  }
}

pub fn modificar_membresia_gui() -> impl IntoElement {
  let contenedor: State<ContenedorRepos> = use_consume();

  let tipo_membresia =
    use_hook(|| vec!["Mensual", "Bimestral", "Trimestral", "Semestral", "Anual"]);
  let mut selecionado = use_state(|| 0);
  let mut fecha_inicio = use_state(|| None::<CalendarDate>);
  let mut fecha_vista = use_state(CalendarDate::now);
  let dni_socio_text = use_state(String::new);
  let mut membresia_actual = use_state(|| None);
  let mut membresia_socio_dto: State<Option<MembresiaSocioDto>> = use_state(|| None);

  let mut fecha_minima = use_state(CalendarDate::now);

  rect()
    .width(Size::fill())
    .height(Size::fill())
    .center()
    .padding(Gaps::new_all(16.))
    .child(
      rect()
        .center()
        .width(Size::percent(95.))
        .max_width(Size::px(900.))
        .height(Size::auto())
        .horizontal()
        .padding(Gaps::new_all(24.))
        .child(
          rect()
            .width(Size::percent(48.))
            .spacing(14.)
            .child(
              Input::new(dni_socio_text)
                .auto_focus(true)
                .placeholder("Ingrese dni del socio")
                .width(Size::percent(75.)),
            )
            .maybe_child(item_datos_membresia_socio(
              membresia_socio_dto,
              dni_socio_text,
            ))
            .child(Button::new().child("Buscar Socio").on_press(move |_| {
              spawn(async move {
                if let Ok(msd) =
                  CasoBuscarMembresiaPorSocio::new((*contenedor.read().membresia_repo).clone())
                    .ejecutar(dni_socio_text.read().clone(), None)
                    .await
                {
                  if let Some(msd) = msd.clone() {
                    let fecha_final = msd.fecha_vencimiento;
                    let fecha_final_calendar_date = CalendarDate {
                      year: fecha_final.year(),
                      month: fecha_final.month(),
                      day: fecha_final.day(),
                    };
                    fecha_inicio.set(Some(fecha_final_calendar_date));
                    fecha_vista.set(fecha_final_calendar_date);
                    fecha_minima.set(fecha_final_calendar_date);
                    selecionado.set(match msd.tipo_membresia {
                      TipoMembresia::Mensual => 0,
                      TipoMembresia::Bimestral => 1,
                      TipoMembresia::Trimestral => 2,
                      TipoMembresia::Semestral => 3,
                      TipoMembresia::Anual => 4,
                    });
                    membresia_actual.set(Some(Membresia::existente(
                      msd.id as u64,
                      msd.id_s as u64,
                      msd.tipo_membresia,
                      msd.fecha_inicio,
                      fecha_final,
                      msd.costo,
                    )));
                  }
                  membresia_socio_dto.set(msd);
                } else {
                  fecha_minima.set(CalendarDate::now());
                  membresia_actual.set(None);
                  membresia_socio_dto.set(None);
                };
              });
            })),
        )
        .child(rect().width(Size::percent(4.)).height(Size::fill()))
        .child(
          rect()
            .width(Size::percent(48.))
            .spacing(14.)
            .child(label().text("Nuevos Datos de Membresía").font_size(16.))
            .maybe_child(modificacion_datos_membresia(
              &tipo_membresia,
              selecionado,
              fecha_inicio,
              fecha_vista,
              membresia_actual,
              fecha_minima,
            )),
        ),
    )
}

fn item_datos_membresia_socio(
  m_s_d: State<Option<MembresiaSocioDto>>,
  dni_socio_text: State<String>,
) -> Option<impl IntoElement> {
  let contenedor: State<ContenedorRepos> = use_consume();
  (*m_s_d.read()).as_ref().map(|m_s_d| {
    rect()
      .child(label().text("Información Actual del Socio").font_size(16.))
      .child(
        rect()
          .width(Size::fill())
          .spacing(10.)
          .child(item_visualizacion("Dni:", m_s_d.dni_s.to_string()))
          .child(item_visualizacion(
            "Nombre Completo:",
            format!("{}, {}", m_s_d.nombre_s, m_s_d.apellido_s),
          ))
          .child(item_visualizacion(
            "Membresía Activa:",
            m_s_d.estado.to_string(),
          ))
          .child(item_visualizacion(
            "Vencimiento Anterior:",
            m_s_d.fecha_vencimiento.to_string(),
          ))
          .child(item_visualizacion(
            "Costo Registrado:",
            format!("S/ {}", m_s_d.costo),
          ))
          .child(
            rect()
              .horizontal()
              .cross_align(Alignment::Center)
              .spacing(8.)
              .padding(Gaps::new_all(4.))
              .child(
                rect()
                  .width(Size::px(8.))
                  .height(Size::px(8.))
                  .corner_radius(CornerRadius::new_all(4.)),
              )
              .child(
                label()
                  .text(EstadoMembresia::Activo.to_string())
                  .font_size(13.),
              ),
          ),
      )
  })
}

fn modificacion_datos_membresia(
  tipo_membresia: &[&str],
  mut selecionado: State<usize>,
  mut fecha_inicio: State<Option<CalendarDate>>,
  mut fecha_vista: State<CalendarDate>,
  membresia_actual: State<Option<Membresia>>,
  fecha_minima: State<CalendarDate>,
) -> Option<impl IntoElement> {
  let contenedor: State<ContenedorRepos> = use_consume();

  if membresia_actual.read().is_some() {
    Some(
      rect()
        .width(Size::fill())
        .spacing(12.)
        .child(
          rect()
            .width(Size::fill())
            .spacing(4.)
            .child(label().text("Tipo de Membresía:").font_size(13.))
            .child(
              Select::new()
                .selected_item(tipo_membresia[selecionado()].to_string())
                .children(tipo_membresia.iter().enumerate().map(|(i, val)| {
                  MenuItem::new()
                    .selected(selecionado() == i)
                    .on_press(move |_| selecionado.set(i))
                    .child(val.to_string())
                    .into()
                })),
            ),
        )
        .child(
          rect()
            .width(Size::fill())
            .spacing(4.)
            .child(label().text("Fecha Inicio:").font_size(13.))
            .child(
              Calendar::new()
                .selected(fecha_inicio())
                .view_date(fecha_vista())
                .on_change(move |date: CalendarDate| {
                  let hoy = Local::now();
                  if date >= *fecha_minima.read()
                  {
                    fecha_inicio.set(Some(date))
                  }
                })
                .on_view_change(move |date| fecha_vista.set(date)),
            ),
        )
        .child(
          rect()
            .width(Size::fill())
            .margin(Gaps::new(14., 0., 0., 0.))
            .maybe_child(boton_renovar(
              contenedor,
              membresia_actual,
              Local::now().date_naive(),
              TipoMembresia::Semestral,
            )),
        ),
    )
  } else {
    None
  }
}

fn item_visualizacion(titulo: &'static str, valor: String) -> impl IntoElement {
  rect()
    .width(Size::fill())
    .padding(Gaps::new_all(6.))
    .spacing(2.)
    .child(label().text(titulo).font_size(11.))
    .child(label().text(valor).font_size(14.))
}

fn boton_renovar(
  contenedor: State<ContenedorRepos>,
  membresia: State<Option<Membresia>>,
  fecha_elegida: NaiveDate,
  tipo_membresia: TipoMembresia,
) -> Option<impl IntoElement> {
  let membresia_actual = membresia.read().clone()?;

  let membresia_renovada =
    Membresia::renovacion_manual(membresia_actual, fecha_elegida, tipo_membresia);

  Some(
    Button::new()
      .filled()
      .on_press(move |_| {
        let m_renovada = membresia_renovada.clone();
        let repo_clone = (*contenedor.read().membresia_repo).clone();

        spawn(async move {
          let a = CasoRenovarMembresia::new(repo_clone)
            .ejecutar(m_renovada)
            .await;
        });

        println!("Acción: Nueva membresía guardada. Fecha de vencimiento auto-calculada.");
      })
      .child(label().text("Guardar y Modificar Membresía")),
  )
}
