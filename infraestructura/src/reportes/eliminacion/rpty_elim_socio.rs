use sqlx::MySqlPool;
use std::fs;
use std::process::Command;

#[derive(Debug)]
pub struct ReporteSocioFila {
  pub id_socio: u32,
  pub nombre: String,
  pub apellidos: String,
  pub dni: String,
  pub telefono: Option<String>,
  pub correo: Option<String>,
  pub direccion: Option<String>,
  pub activo: i8, // MySQL maneja BOOLEAN como TINYINT/i8 internamente en sqlx
}

pub async fn generar_reporte_previo_eliminacion_socio(
  pool: &MySqlPool,
  id_socio: u32,
) -> Result<String, Box<dyn std::error::Error>> {
  // 1. Consultar los datos del socio ANTES de borrarlo
  let socio = sqlx::query_as!(
    ReporteSocioFila,
    r#"
        SELECT
            idsocio AS "id_socio: u32",
            nombre,
            apellidos,
            dni,
            telefono,
            correo,
            direccion,
            activo AS "activo: i8"
        FROM socios
        WHERE idsocio = ?;
        "#,
    id_socio
  )
  .fetch_optional(pool)
  .await?
  .ok_or_else(|| format!("No se encontró el socio con ID: {}", id_socio))?;

  // 2. Preparar los strings de datos procesados
  let nombre_completo = format!("{}, {}", socio.apellidos, socio.nombre);
  // let estado_str = if socio.activo == 1 {
  //   "Activo"
  // } else {
  //   "Inactivo"
  // };
  let estado_str = "Inactivo";
  let telefono_str = socio
    .telefono
    .unwrap_or_else(|| "No registrado".to_string());
  let correo_str = socio.correo.unwrap_or_else(|| "No registrado".to_string());
  let direccion_str = socio
    .direccion
    .unwrap_or_else(|| "No registrada".to_string());

  // 3. Plantilla Typst adaptada para Socio (A5 Horizontal)
  let mut contenido_plantilla = r##"
      #set page(
        paper: "a4",
        flipped: true,
        margin: (x: 3.5cm, y: 3.5cm),
        fill: rgb("#fdfdfd")
      )
      #set text(font: "Liberation Sans", size: 11pt, fill: rgb("#2c3e50"))

      // Encabezado
      #align(right)[
        #text(8pt, fill: gray.darken(20%))[Reporte de Control / Auditoría de Bajas de Socios]
      ]
      #v(0.5em)

      // Título
      #align(center)[
        #text(16pt, weight: "bold", fill: rgb("#c0392b"))[Expediente de Socio a Eliminar]
      ]
      #v(1.5em)

      // Contenedor de datos
      #block(
        width: 100%,
        stroke: 0.5pt + rgb("#bdc3c7"),
        radius: 6pt,
        inset: 18pt,
        fill: white
      )[
        #text(12pt, weight: "bold", fill: rgb("#c0392b"))[Datos de Identificación y Contacto]
        #v(1em)

        #grid(
          columns: (1fr, 2fr),
          column-gutter: 1.5em,
          row-gutter: 1.2em,
          [ *ID Socio:* ], [ {{ID_SOCIO}} ],
          [ *Nombre Completo:* ], "{{SOCIO}}",
          [ *DNI / Identificación:* ], "{{DNI}}",
          [ *Teléfono:* ], "{{TELEFONO}}",
          [ *Correo Electrónico:* ], "{{CORREO}}",
          [ *Dirección:* ], "{{DIRECCION}}",
          [ *Estado Actual:* ], "{{ESTADO}}"
        )
      ]
      "##
    .to_string();

  // 4. Reemplazar los marcadores directamente
  contenido_plantilla = contenido_plantilla
    .replace("{{ID_SOCIO}}", &socio.id_socio.to_string())
    .replace("{{SOCIO}}", &nombre_completo)
    .replace("{{DNI}}", &socio.dni)
    .replace("{{TELEFONO}}", &telefono_str)
    .replace("{{CORREO}}", &correo_str)
    .replace("{{DIRECCION}}", &direccion_str)
    .replace("{{ESTADO}}", estado_str);

  // 5. Escritura y compilación nativa
  let archivo_temporal = format!("reporte_previo_borrado_socio_{}.typ", id_socio);
  let pdf_salida = format!("reporte_socio_eliminado_{}.pdf", id_socio);
  fs::write(&archivo_temporal, contenido_plantilla)?;

  tracing::info!("Compilando PDF de pre-eliminación de socio con Typst...");
  let output = Command::new("typst")
    .arg("compile")
    .arg(&archivo_temporal)
    .arg(&pdf_salida)
    .output()?;

  let _ = fs::remove_file(archivo_temporal);

  // 6. Validación y retorno de la ruta del archivo
  if output.status.success() {
    tracing::info!("¡PDF de socio generado exitosamente en: {}!", pdf_salida);
    Ok(pdf_salida)
  } else {
    let error = String::from_utf8_lossy(&output.stderr);
    tracing::warn!(
      "Error al compilar el reporte del socio con Typst:\n{}",
      error
    );
    Err(format!("Error en compilación de Typst: {}", error).into())
  }
}
