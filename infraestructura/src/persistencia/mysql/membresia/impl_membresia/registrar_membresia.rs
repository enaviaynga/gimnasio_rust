use hexagonal_gimnasio::membresias::{
  dominio::membresia::Membresia,
  puertos::trait_membresia::{ErrorMembresia, RegistrarNuevaMembresia},
};

use crate::persistencia::mysql::membresia::membresia_repositorio::MySqlxMembresia;

#[derive(Debug)]
pub struct DatosSocioReporte {
  pub nombre: String,
  pub apellidos: String,
  pub dni: String,
  pub telefono: Option<String>,
  pub correo: Option<String>,
  pub direccion: Option<String>,
}

#[async_trait::async_trait]
impl RegistrarNuevaMembresia for MySqlxMembresia {
  async fn registrar_en_db(&self, mut membresia: Membresia) -> Result<(), ErrorMembresia> {
    let pool = self.ref_pool();

    // 0. Validar que el socio exista y esté activo
    let socio_activo = sqlx::query!(
      r#"
      SELECT activo FROM socios WHERE idsocio = ?
      "#,
      membresia.get_id_socio()
    )
    .fetch_optional(pool) // Usamos fetch_optional por si el ID del socio ni siquiera existe
    .await
    .map_err(|e| ErrorMembresia::Otro(format!("Error al verificar el estado del socio: {e}")))?;

    match socio_activo {
      None => {
        return Err(ErrorMembresia::Otro(format!(
          "El socio con ID {} no existe en el sistema.",
          membresia.get_id_socio()
        )));
      }
      Some(registro) if registro.activo == 0 => {
        // En MySQL, BOOLEAN es un alias de TINYINT (0 es falso)
        return Err(ErrorMembresia::Otro(format!(
          "No se puede registrar la membresía. El socio con ID {} se encuentra INACTIVO.",
          membresia.get_id_socio()
        )));
      }
      _ => {} // Socio existe y está activo, continuamos
    }

    // 1. Validar solapamiento de fechas con membresías existentes para este socio
    let tiene_membresia_activa = sqlx::query!(
      r#"
      SELECT EXISTS(
          SELECT 1 FROM membresias
          WHERE idsocio = ?
            AND ? <= fechaVencimiento
            AND ? >= fechaInicio
      ) AS "solapado!"
      "#,
      membresia.get_id_socio(),
      membresia.get_fecha_inicio(), // Nueva fecha de inicio <= vencimiento existente
      membresia.get_fecha_final()   // Nuevo vencimiento >= inicio existente
    )
    .fetch_one(pool)
    .await
    .map_err(|e| {
      ErrorMembresia::Otro(format!(
        "Error al verificar solapamiento de membresías: {e}"
      ))
    })?;

    // Si el resultado es 1, significa que las fechas chocan con otra membresía activa
    if tiene_membresia_activa.solapado != 0 {
      return Err(ErrorMembresia::Otro(format!(
        "El socio ya cuenta con una membresía activa o programada dentro del rango solicitado ({} a {}).",
        membresia.get_fecha_inicio(),
        membresia.get_fecha_final()
      )));
    }

    // 2. Insertar la nueva membresía (solo si no hay colisión de fechas)
    let resultado = sqlx::query!(
      "INSERT INTO membresias (idsocio, tipoMembresia, fechaInicio, fechaVencimiento, costo) VALUES
            (?, ?, ?, ?, ?);",
      membresia.get_id_socio(),
      membresia.get_membresia().to_string(),
      membresia.get_fecha_inicio(),
      membresia.get_fecha_final(),
      membresia.get_costo(),
    )
    .execute(pool)
    .await
    .map_err(|e| ErrorMembresia::Otro(format!("Error al insertar membresía: {e}")))?;

    // 3. Obtener datos del socio para el reporte
    let socio = sqlx::query_as!(
      DatosSocioReporte,
      "SELECT nombre, apellidos, dni, telefono, correo, direccion FROM socios WHERE idsocio = ?;",
      membresia.get_id_socio()
    )
    .fetch_one(pool)
    .await
    .map_err(|e| ErrorMembresia::Otro(format!("No se encontró al socio para el reporte: {e}")))?;

    // 4. Guardar registro externo/reporte
    if guardar_registro_nueva_membresia(membresia, resultado.last_insert_id(), socio)
      .inspect_err(|e| {
        tracing::warn!("Error al guardar el registro: {e}");
      })
      .is_ok()
    {};

    Ok(())
  }
}

fn guardar_registro_nueva_membresia(
  membresia: Membresia,
  id: u64,
  socio: DatosSocioReporte,
) -> Result<(), Box<dyn std::error::Error>> {
  let mut contenido_plantilla = r##"#let membresia_template(
    id: "N/A",
    id_socio: "",
    nombre_socio: "",
    dni_socio: "",
    telefono: "",
    correo: "",
    direccion: "",
    tipo_membresia: "",
    costo: "",
    fecha_inicio: "",
    fecha_final: "",
  ) = {
    set page(
      width: 14cm,
      height: auto,
      margin: (x: 1.2cm, y: 1.2cm),
      fill: rgb("#f8f9fa"),
    )
    set text(font: "Liberation Sans", size: 10pt)

    align(right)[
      #text(7pt, fill: gray.darken(20%))[Reporte de Sistema de Socios]
    ]

    align(center)[
      #text(14pt, weight: "bold", fill: rgb("#1d3557"))[Detalles de Membresía]
      #v(0.5em)
    ]

    line(length: 100%, stroke: 0.5pt + rgb("#e63946"))

    // Sin '#' porque ya estamos dentro del bloque de código de la función
    let celdas = (
      [*ID Membresía:*], [#id],
      [*Socio:*], [#nombre_socio (ID: #id_socio)],
      [*DNI:*], [#dni_socio],
    )

    // Agregamos condicionalmente SOLO si tienen un valor real
    if telefono != "N/A" { celdas.push([*Teléfono:*]); celdas.push([#telefono]) }
    if correo != "N/A" { celdas.push([*Correo:*]); celdas.push([#correo]) }
    if direccion != "N/A" { celdas.push([*Dirección:*]); celdas.push([#direccion]) }

    // Agregamos el resto de elementos fijos
    celdas.push([*Tipo de Membresía:*])
    celdas.push([#tipo_membresia])
    celdas.push([*Costo:*])
    celdas.push([\$#costo])
    celdas.push([*Fecha de Inicio:*])
    celdas.push([#fecha_inicio])
    celdas.push([*Fecha de Vencimiento:*])
    celdas.push([#fecha_final])

    // Renderizamos la tabla expandiendo el array dinámico con '..'
    grid(
      columns: (auto, 1fr),
      row-gutter: 1.2em,
      column-gutter: 1.5cm,
      ..celdas
    )
  }

  // Marcadores para el reemplazo desde Rust
  #membresia_template(
    id: "{{ID}}",
    id_socio: "{{ID_SOCIO}}",
    nombre_socio: "{{NOMBRE_SOCIO}}",
    dni_socio: "{{DNI_SOCIO}}",
    telefono: "{{TELEFONO}}",
    correo: "{{CORREO}}",
    direccion: "{{DIRECCION}}",
    tipo_membresia: "{{TIPO}}",
    costo: "{{COSTO}}",
    fecha_inicio: "{{INICIO}}",
    fecha_final: "{{FINAL}}",
  )
"##
    .to_string();

  let id_str = id.to_string();
  let tipo_str = format!("{:?}", membresia.get_membresia());

  let nombre_completo = format!("{}, {}", socio.nombre, socio.apellidos);

  let tel_str = socio.telefono.unwrap_or_else(|| "N/A".to_string());
  let corr_str = socio.correo.unwrap_or_else(|| "N/A".to_string());
  let dir_str = socio.direccion.unwrap_or_else(|| "N/A".to_string());

  contenido_plantilla = contenido_plantilla
    .replace("{{ID}}", &id_str)
    .replace("{{ID_SOCIO}}", &membresia.get_id_socio().to_string())
    .replace("{{NOMBRE_SOCIO}}", &nombre_completo)
    .replace("{{DNI_SOCIO}}", &socio.dni)
    .replace("{{TELEFONO}}", &tel_str)
    .replace("{{CORREO}}", &corr_str)
    .replace("{{DIRECCION}}", &dir_str)
    .replace("{{TIPO}}", &tipo_str)
    .replace("{{COSTO}}", &membresia.get_costo().to_string())
    .replace(
      "{{INICIO}}",
      &membresia.get_fecha_inicio().format("%d/%m/%Y").to_string(),
    )
    .replace(
      "{{FINAL}}",
      &membresia.get_fecha_final().format("%d/%m/%Y").to_string(),
    );

  let archivo_temporal = "membresia_generada.typ";
  let pdf_salida = "membresia.pdf";
  std::fs::write(archivo_temporal, contenido_plantilla)?;

  let output = std::process::Command::new("typst")
    .arg("compile")
    .arg(archivo_temporal)
    .arg(pdf_salida)
    .output()?;

  let _ = std::fs::remove_file(archivo_temporal);

  if output.status.success() {
    tracing::info!("¡PDF generado con éxito!");
    opener::open(pdf_salida)?;
  } else {
    let error = String::from_utf8_lossy(&output.stderr);
    tracing::warn!("Error al compilar con Typst:\n{}", error);
  }
  Ok(())
}
