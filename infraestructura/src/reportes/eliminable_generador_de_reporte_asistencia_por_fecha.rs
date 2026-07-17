use chrono::{NaiveDate, NaiveTime};
use sqlx::MySqlPool;
use std::collections::HashMap;
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

pub async fn generar_reporte_asistencias_por_fecha(
  pool: &MySqlPool,
  fecha_inicio: NaiveDate,
  fecha_fin: NaiveDate,
) -> Result<(), Box<dyn std::error::Error>> {
  // 1. Consultar la base de datos uniendo asistencia con el socio correspondiente
  let asistencias = sqlx::query_as!(
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
        WHERE a.fecha BETWEEN ? AND ?
        ORDER BY a.fecha ASC, a.horaIngreso ASC;
        "#,
    fecha_inicio,
    fecha_fin
  )
  .fetch_all(pool)
  .await?;

  // --- CÁLCULO DE ESTADÍSTICAS ---
  let total_asistencias = asistencias.len();

  // Calculamos la cantidad de días totales en el rango seleccionado (inclusive)
  let dias_seleccionados = (fecha_fin - fecha_inicio).num_days() + 1;
  let promedio_asistencias = total_asistencias as f64 / dias_seleccionados as f64;

  // Agrupar por fecha para obtener máximos y mínimos
  let mut conteo_por_dia: HashMap<NaiveDate, usize> = HashMap::new();
  for asis in &asistencias {
    *conteo_por_dia.entry(asis.fecha).or_insert(0) += 1;
  }

  // Encontrar día con máxima asistencia
  let max_dia = conteo_por_dia
    .iter()
    .max_by_key(|&(_, count)| count)
    .map(|(date, count)| format!("{} ({} asistencias)", date.format("%d/%m/%Y"), count))
    .unwrap_or_else(|| "N/A".to_string());

  // Encontrar día con menor asistencia (considerando solo los días que tuvieron al menos 1 asistencia)
  let min_dia = conteo_por_dia
    .iter()
    .min_by_key(|&(_, count)| count)
    .map(|(date, count)| format!("{} ({} asistencias)", date.format("%d/%m/%Y"), count))
    .unwrap_or_else(|| "N/A".to_string());
  // -------------------------------

  // 2. Construir dinámicamente las filas de la tabla para Typst
  let mut filas_typst = String::new();

  if asistencias.is_empty() {
    filas_typst
      .push_str("  table.cell(colspan: 6)[No se registraron asistencias en este periodo.],\n");
  } else {
    for asis in &asistencias {
      let nombre_completo = format!("{}, {}", asis.apellidos_socio, asis.nombre_socio);
      let fecha_str = asis.fecha.format("%d/%m/%Y").to_string();
      let hora_str = asis.hora_ingreso.format("%H:%M").to_string();

      filas_typst.push_str(&format!(
        "  [{}], [{}], [{}], [{}], [{} {}], [{}],\n",
        asis.id_asistencia,
        asis.id_socio,
        nombre_completo,
        asis.dni_socio,
        fecha_str,
        hora_str,
        asis.resultado_validacion
      ));
    }
  }

  // 3. Definir la plantilla base en Typst con el cuadro de resumen
  let mut contenido_plantilla = r##"
    #let reporte_asistencias(
      inicio: "",
      fin: "",
    ) = {
      set page(
        paper: "a4",
        flipped: false,
        margin: (x: 1.5cm, y: 1.5cm),
        fill: rgb("#f8f9fa"),
      )
      set text(font: "Liberation Sans", size: 10pt)

      // Encabezado del Sistema
      align(right)[
        #text(8pt, fill: gray.darken(20%))[Reporte Automatizado de Control de Accesos]
      ]

      // Título Principal
      align(center)[
        #text(16pt, weight: "bold", fill: rgb("#1d3557"))[Reporte General de Asistencias]
        #v(0.2em)
        #text(11pt, style: "italic", fill: gray.darken(40%))[Periodo: #inicio al #fin]
        #v(0.8em)
      ]

      // Línea de diseño
      line(length: 100%, stroke: 1.5pt + rgb("#1d3557"))
      v(0.5em)

      // --- Cuadro de Resumen Estadístico ---
      block(
        fill: rgb("#f1f4f9"),
        inset: 10pt,
        radius: 4pt,
        width: 100%,
        stroke: 0.5pt + rgb("#d5dbdb"),
        [
          #text(weight: "bold", fill: rgb("#1d3557"))[Resumen del Periodo:]
          #v(0.3em)
          #grid(
            columns: (1fr, 1fr),
            row-gutter: 6pt,
            [Total Asistencias: *{{TOTAL_ASISTENCIAS}}*],
            [Promedio Diario: *{{PROMEDIO_ASISTENCIAS}}*],
            [Día Máxima Asistencia: *{{MAX_DIA}}*],
            [Día Mínima Asistencia: *{{MIN_DIA}}*],
          )
        ]
      )
      v(1em)

      // Tabla de Datos
      table(
        columns: (0.8fr, 0.8fr, 2.5fr, 1.2fr, 1.8fr, 1.5fr),
        inset: 8pt,
        align: (center, center, left, center, center, left),
        fill: (x, y) => if y == 0 { rgb("#1d3557") } else if calc.even(y) { rgb("#eaf2f8") } else { white },
        stroke: 0.3pt + rgb("#d5dbdb"),

        // Cabeceras (Texto blanco y en negrita)
        table.header(
          [*ID*], [*ID Socio*], [*Socio (Apellidos, Nombre)*], [*DNI*], [*Fecha/Hora*], [*Validación*]
        ),

        // Celdas dinámicas de Rust
        {{FILAS_DINAMICAS}}
      )
    }

    // Inicialización por marcadores
    #reporte_asistencias(
      inicio: "{{FECHA_INICIO}}",
      fin: "{{FECHA_FIN}}",
    )
    "##.to_string();

  // 4. Reemplazar marcadores con la data procesada e indicadores
  contenido_plantilla = contenido_plantilla
    .replace(
      "{{FECHA_INICIO}}",
      &fecha_inicio.format("%d/%m/%Y").to_string(),
    )
    .replace("{{FECHA_FIN}}", &fecha_fin.format("%d/%m/%Y").to_string())
    .replace("{{TOTAL_ASISTENCIAS}}", &total_asistencias.to_string())
    .replace(
      "{{PROMEDIO_ASISTENCIAS}}",
      &format!("{:.2}", promedio_asistencias),
    )
    .replace("{{MAX_DIA}}", &max_dia)
    .replace("{{MIN_DIA}}", &min_dia)
    .replace("{{FILAS_DINAMICAS}}", &filas_typst);

  // 5. Flujo de escritura y compilación nativa
  let archivo_temporal = "reporte_asistencia_gen.typ";
  let pdf_salida = "reporte_asistencias.pdf";
  fs::write(archivo_temporal, contenido_plantilla)?;

  tracing::info!("Compilando Reporte PDF con Typst...");
  let output = Command::new("typst")
    .arg("compile")
    .arg(archivo_temporal)
    .arg(pdf_salida)
    .output()?;

  let _ = fs::remove_file(archivo_temporal);

  // 6. Validación del estado de salida
  if output.status.success() {
    tracing::info!("¡Reporte PDF generado exitosamente!");
    opener::open(pdf_salida)?;
  } else {
    let error = String::from_utf8_lossy(&output.stderr);
    tracing::warn!("Error al compilar el reporte con Typst:\n{}", error);
  }

  Ok(())
}
