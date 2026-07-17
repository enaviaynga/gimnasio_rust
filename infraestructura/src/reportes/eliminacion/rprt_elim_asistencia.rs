use chrono::{NaiveDate, NaiveTime};
use sqlx::MySqlPool;
use std::fs;
use std::process::Command;

#[derive(Debug)]
pub struct ReporteAsistenciaFila {
  pub id_asistencia: u32,
  pub id_socio: u32,
  pub nombre_socio: String,
  pub apellidos_socio: String,
  pub dni_socio: String,
  pub fecha: NaiveDate,
  pub hora_ingreso: NaiveTime,
  pub resultado_validacion: String,
}

pub async fn generar_reporte_previo_eliminacion_unica(
  pool: &MySqlPool,
  id_asistencia: u32,
) -> Result<String, Box<dyn std::error::Error>> {
  // 1. Consultar la asistencia específica ANTES de borrarla
  let asis = sqlx::query_as!(
    ReporteAsistenciaFila,
    r#"
        SELECT
            a.idAsistencia AS "id_asistencia: u32",
            a.idsocio AS "id_socio: u32",
            s.nombre AS "nombre_socio",
            s.apellidos AS "apellidos_socio",
            s.dni AS "dni_socio",
            a.fecha AS "fecha",
            a.horaIngreso AS "hora_ingreso",
            a.resultadoValidacion AS "resultado_validacion"
        FROM asistencia a
        INNER JOIN socios s ON a.idsocio = s.idsocio
        WHERE a.idAsistencia = ?;
        "#,
    id_asistencia
  )
  .fetch_optional(pool)
  .await?
  .ok_or_else(|| format!("No se encontró la asistencia con ID: {}", id_asistencia))?;

  // 2. Preparar los strings de datos procesados
  let nombre_completo = format!("{}, {}", asis.apellidos_socio, asis.nombre_socio);
  let fecha_hora_str = format!(
    "{} a las {}",
    asis.fecha.format("%d/%m/%Y"),
    asis.hora_ingreso.format("%H:%M")
  );

  // 3. Plantilla Typst plana y simplificada (en Horizontal/Landscape para evitar el exceso de alto)
  let mut contenido_plantilla = r##"
    #set page(
      paper: "a5",
      flipped: true,
      margin: (x: 4cm, y: 4cm), // Márgenes más grandes para "encoger" el contenido
      fill: rgb("#fdfdfd")
    )
    #set text(font: "Liberation Sans", size: 11pt, fill: rgb("#2c3e50"))

    // Encabezado
    #align(right)[
      #text(8pt, fill: gray.darken(20%))[Reporte de Control / Auditoría de Bajas]
    ]
    #v(1em)

    // Título
    #align(center)[
      #text(16pt, weight: "bold", fill: rgb("#c0392b"))[Detalle de Asistencia a Eliminar]
    ]
    #v(2em)

    // Contenedor de datos
    #block(
      width: 100%,
      stroke: 0.5pt + rgb("#bdc3c7"),
      radius: 6pt,
      inset: 20pt,
      fill: white
    )[
      #text(12pt, weight: "bold", fill: rgb("#c0392b"))[Datos del Registro]
      #v(1em)

      #grid(
        columns: (1fr, 1.5fr),
        column-gutter: 2em,
        row-gutter: 1.5em,
        [ *ID Asistencia:* ], [ {{ID_ASISTENCIA}} ],
        [ *ID Socio:* ], [ {{ID_SOCIO}} ],
        [ *Socio:* ], [ {{SOCIO}} ],
        [ *DNI:* ], [ {{DNI}} ],
        [ *Fecha y Hora:* ], [ {{FECHA_HORA}} ],
        [ *Resultado Validación:* ], [ {{VALIDACION}} ]
      )
    ]
    "##
    .to_string();

  // 4. Reemplazar los marcadores directamente de manera lineal
  contenido_plantilla = contenido_plantilla
    .replace("{{ID_ASISTENCIA}}", &asis.id_asistencia.to_string())
    .replace("{{ID_SOCIO}}", &asis.id_socio.to_string())
    .replace("{{SOCIO}}", &nombre_completo)
    .replace("{{DNI}}", &asis.dni_socio)
    .replace("{{FECHA_HORA}}", &fecha_hora_str)
    .replace("{{VALIDACION}}", &asis.resultado_validacion);

  // 5. Escritura y compilación nativa
  let archivo_temporal = format!("reporte_previo_borrado_{}.typ", id_asistencia);
  let pdf_salida = format!("reporte_asistencia_eliminada_{}.pdf", id_asistencia);
  fs::write(&archivo_temporal, contenido_plantilla)?;

  tracing::info!("Compilando PDF de pre-eliminación única con Typst...");
  let output = Command::new("typst")
    .arg("compile")
    .arg(&archivo_temporal)
    .arg(&pdf_salida)
    .output()?;

  let _ = fs::remove_file(archivo_temporal);

  // 6. Validación y retorno de la ruta del archivo
  if output.status.success() {
    tracing::info!("¡PDF generado exitosamente en: {}!", pdf_salida);
    Ok(pdf_salida)
  } else {
    let error = String::from_utf8_lossy(&output.stderr);
    tracing::warn!("Error al compilar el reporte con Typst:\n{}", error);
    Err(format!("Error en compilación de Typst: {}", error).into())
  }
}
