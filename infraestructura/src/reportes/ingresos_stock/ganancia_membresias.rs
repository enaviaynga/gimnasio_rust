use chrono::NaiveDate;
use sqlx::MySqlPool;
use std::collections::HashMap;
use std::fs;
use std::process::Command;
// Importamos Decimal de forma correcta y las utilidades para convertir a f64 al compilar el Typst
use rust_decimal::Decimal;
use rust_decimal::prelude::ToPrimitive;

#[derive(Debug)]
pub struct ReporteIngresoFila {
  pub id_membresia: u32,
  pub id_socio: u32,
  pub nombre_socio: String,
  pub apellidos_socio: String,
  pub tipo_membresia: String,
  pub costo: Decimal, // Usamos tu tipo nativo directamente
  pub fecha_inicio: NaiveDate,
  pub fecha_vencimiento: NaiveDate,
}

pub async fn generar_reporte_ingresos_membresia(
  pool: &MySqlPool,
  fecha_inicio: NaiveDate,
  fecha_fin: NaiveDate,
) -> Result<(), Box<dyn std::error::Error>> {
  // 1. Consultar membresías vendidas en el rango de fechas
  let membresias = sqlx::query_as!(
    ReporteIngresoFila,
    r#"
        SELECT
            m.idMembresia AS "id_membresia: u32",
            m.idsocio AS "id_socio: u32",
            s.nombre AS "nombre_socio",
            s.apellidos AS "apellidos_socio",
            m.tipoMembresia AS "tipo_membresia",
            m.costo AS "costo: Decimal",
            m.fechaInicio AS "fecha_inicio",
            m.fechaVencimiento AS "fecha_vencimiento"
        FROM membresias m
        INNER JOIN socios s ON m.idsocio = s.idsocio
        WHERE m.fechaInicio BETWEEN ? AND ?
        ORDER BY m.fechaInicio ASC;
        "#,
    fecha_inicio,
    fecha_fin
  )
  .fetch_all(pool)
  .await?;

  // --- CÁLCULO DE ESTADÍSTICAS FINANCIERAS CON PRECISIÓN ---
  let mut total_ingresos = Decimal::ZERO;
  let mut rendimiento_mensual_proyectado = Decimal::ZERO; // Nueva variable exacta para la prorrata
  let mut conteo_por_tipo: HashMap<String, (u32, Decimal)> = HashMap::new();

  for m in &membresias {
    total_ingresos += m.costo;

    // Calculamos el aporte mensual según el tipo de membresía de manera exacta
    let meses_duracion = match m.tipo_membresia.to_lowercase().as_str() {
      "anual" => Decimal::from(12),
      "trimestral" => Decimal::from(3),
      "bimestral" => Decimal::from(2),
      "mensual" => Decimal::from(1),
      _ => Decimal::from(1), // Por defecto 1 si hay un tipo no mapeado
    };

    // El aporte mensual de esta venta específica es: costo / meses
    rendimiento_mensual_proyectado += m.costo / meses_duracion;

    let entrada = conteo_por_tipo
      .entry(m.tipo_membresia.clone())
      .or_insert((0, Decimal::ZERO));
    entrada.0 += 1;
    entrada.1 += m.costo;
  }

  // Convertimos a f64 únicamente para el formateador final de Typst si fuera necesario,
  // o mejor aún, usamos el Decimal directamente para mantener la precisión impecable.
  let ganancia_mensual_promedio = rendimiento_mensual_proyectado.to_f64().unwrap_or(0.0);

  // Calcular promedio mensual estimado basado en los días del rango
  let dias_seleccionados = (fecha_fin - fecha_inicio).num_days() + 1;
  let meses_estimados = (dias_seleccionados as f64 / 30.44).max(1.0);

  // Generar bloque de desglose por tipo de membresía para Typst
  let mut desglose_tipos_typst = String::new();
  for (tipo, (cantidad, suma)) in &conteo_por_tipo {
    desglose_tipos_typst.push_str(&format!(
      "  [Membresía {} (-)], [{} ventas], [*S/. {}*],\n",
      tipo, cantidad, suma
    ));
  }
  if desglose_tipos_typst.is_empty() {
    desglose_tipos_typst.push_str("  [Sin datos], [0], [S/. 0.00],\n");
  }

  // 2. Construir dinámicamente las filas de la tabla principal
  let mut filas_typst = String::new();

  if membresias.is_empty() {
    filas_typst
      .push_str("  table.cell(colspan: 6)[No se registraron ingresos en este periodo.],\n");
  } else {
    for m in &membresias {
      let nombre_completo = format!("{}, {}", m.apellidos_socio, m.nombre_socio);
      let f_inicio_str = m.fecha_inicio.format("%d/%m/%Y").to_string();
      let f_venc_str = m.fecha_vencimiento.format("%d/%m/%Y").to_string();

      filas_typst.push_str(&format!(
        "  [{}], [{}], [{}], [{}], [{} al {}], [S/. {}],\n",
        m.id_membresia,
        nombre_completo,
        m.tipo_membresia,
        m.fecha_inicio.format("%b %Y"),
        f_inicio_str,
        f_venc_str,
        m.costo // Se formatea automáticamente con sus decimales exactos
      ));
    }
  }

  // 3. Plantilla en Typst orientada a Finanzas
  let mut contenido_plantilla = r##"
    #let reporte_ingresos(
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

      // Encabezado
      align(right)[
        #text(8pt, fill: gray.darken(20%))[Reporte Financiero de Membresías | Confidencial]
      ]

      // Título Principal
      align(center)[
        #text(16pt, weight: "bold", fill: rgb("#1b4332"))[Reporte de Ingresos por Membresías]
        #v(0.2em)
        #text(11pt, style: "italic", fill: gray.darken(40%))[Periodo de Análisis: #inicio al #fin]
        #v(0.8em)
      ]

      line(length: 100%, stroke: 1.5pt + rgb("#1b4332"))
      v(0.5em)

      // KPls Resumen Financiero
      grid(
        columns: (1fr, 1fr),
        gutter: 12pt,
        block(
          fill: rgb("#e9f5ed"),
          inset: 10pt, radius: 4pt, width: 100%, stroke: 0.5pt + rgb("#2d6a4f"),
          [
            #text(weight: "bold", fill: rgb("#1b4332"))[Rendimiento Total:]
            #v(0.4em)
            - Total Recaudado: *S/. {{TOTAL_INGRESOS}}*
            - Rendimiento Mensual Promedio: *S/. {{GANANCIA_MENSUAL}}*
          ]
        ),
        block(
          fill: rgb("#f1f4f9"),
          inset: 10pt, radius: 4pt, width: 100%, stroke: 0.5pt + rgb("#d5dbdb"),
          [
            #text(weight: "bold", fill: rgb("#1d3557"))[Ingresos por Tipo de Membresía:]
            #v(0.2em)
            #table(
              columns: (1.5fr, 1fr, 1fr),
              stroke: none,
              inset: 3pt,
              align: (left, center, right),
              {{DESGLOSE_TIPOS}}
            )
          ]
        )
      )
      v(1em)

      // Tabla de Datos de Auditoría
      text(12pt, weight: "bold", fill: rgb("#1b4332"))[Detalle de Transacciones]
      v(0.3em)

      table(
        columns: (0.6fr, 2.2fr, 1.2fr, 1.2fr, 2.2fr, 1fr),
        inset: 7pt,
        align: (center, left, center, center, center, right),
        fill: (x, y) => if y == 0 { rgb("#1b4332") } else if calc.even(y) { rgb("#f4f9f4") } else { white },
        stroke: 0.3pt + rgb("#d5dbdb"),

        table.header(
          [*ID*], [*Socio (Apellidos, Nombre)*], [*Tipo*], [*Mes Alta*], [*Vigencia*], [*Monto*]
        ),

        {{FILAS_DINAMICAS}}
      )
    }

    #reporte_ingresos(
      inicio: "{{FECHA_INICIO}}",
      fin: "{{FECHA_FIN}}",
    )
    "##.to_string();

  // 4. Reemplazo de Marcadores Financieros
  contenido_plantilla = contenido_plantilla
    .replace(
      "{{FECHA_INICIO}}",
      &fecha_inicio.format("%d/%m/%Y").to_string(),
    )
    .replace("{{FECHA_FIN}}", &fecha_fin.format("%d/%m/%Y").to_string())
    .replace("{{TOTAL_INGRESOS}}", &total_ingresos.to_string())
    .replace(
      "{{GANANCIA_MENSUAL}}",
      &format!("{:.2}", ganancia_mensual_promedio),
    )
    .replace("{{DESGLOSE_TIPOS}}", &desglose_tipos_typst)
    .replace("{{FILAS_DINAMICAS}}", &filas_typst);

  // 5. Escritura y compilación nativa
  let archivo_temporal = "reporte_ingresos_gen.typ";
  let pdf_salida = "reporte_ingresos_membresias.pdf";
  fs::write(archivo_temporal, contenido_plantilla)?;

  tracing::info!("Compilando Reporte Financiero PDF con Typst...");
  let output = Command::new("typst")
    .arg("compile")
    .arg(archivo_temporal)
    .arg(pdf_salida)
    .output()?;

  let _ = fs::remove_file(archivo_temporal);

  if output.status.success() {
    tracing::info!("¡Reporte de ingresos generado exitosamente!");
    opener::open(pdf_salida)?;
  } else {
    let error = String::from_utf8_lossy(&output.stderr);
    tracing::warn!("Error al compilar el reporte con Typst:\n{}", error);
  }

  Ok(())
}
