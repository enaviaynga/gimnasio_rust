use chrono::Datelike;
use rust_decimal::Decimal;
use sqlx::{MySqlPool, Row};
use std::error::Error;
use std::fs;
use std::process::Command;

// Estructura interna limpia con tipos Decimal directos de Rust
#[derive(sqlx::FromRow, Debug)]
struct FilaMetricasMes {
  mes: i32,
  tasa_retencion: Decimal,
  rotacion_inventario: Decimal,
  arpu: Decimal,
  morosidad: Decimal,
  efectividad_cobranza: Decimal,
  crecimiento_membresias: Decimal,
  tasa_desercion: Decimal,
}

#[tracing::instrument(skip(pool), fields(anio = %anio))]
pub async fn generar_reporte_anual_typst(
  pool: &MySqlPool,
  anio: i32,
) -> Result<String, Box<dyn Error>> {
  tracing::info!(
    "Iniciando generación de reporte anual consolidado para el año {}",
    anio
  );

  // --- 1. CONSULTA DE ASISTENCIA PROMEDIO ANUAL ---
  let row_asistencia = sqlx::query(
    r#"
        WITH asistencias_diarias AS (
            SELECT
                a.fecha,
                COUNT(a.idAsistencia) AS cant_asistencias,
                (
                    SELECT COUNT(DISTINCT m.idsocio)
                    FROM membresias m
                    WHERE a.fecha BETWEEN m.fechaInicio AND m.fechaVencimiento
                ) AS socios_activos
            FROM asistencia a
            WHERE YEAR(a.fecha) = ?
            GROUP BY a.fecha
        )
        SELECT
            COALESCE(
                AVG(
                    CASE
                        WHEN socios_activos > 0 THEN (cant_asistencias / socios_activos) * 100
                        ELSE 0
                    END
                ),
                0
            ) AS asistencia_promedio
        FROM asistencias_diarias
        "#,
  )
  .bind(anio)
  .fetch_one(pool)
  .await?;

  let asistencia_promedio_anual: Decimal = row_asistencia
    .get::<Decimal, _>("asistencia_promedio")
    .round_dp(2);

  // --- 2. QUERY METRICAS CONSOLIDADAS ---
  tracing::info!("Fase 2: Ejecutando consulta analítica de métricas mensuales...");

  let query_kpis = r#"
        WITH RECURSIVE meses AS (
            SELECT 1 AS mes
            UNION ALL
            SELECT mes + 1 FROM meses WHERE mes < 12
        ),
        fechas_mes AS (
            SELECT
                mes,
                STR_TO_DATE(CONCAT(?, '-', mes, '-01'), '%Y-%m-%d') AS fecha_inicio,
                LAST_DAY(STR_TO_DATE(CONCAT(?, '-', mes, '-01'), '%Y-%m-%d')) AS fecha_fin
            FROM meses
        )
        SELECT
            f.mes,

            -- 1. TASA DE RETENCIÓN
            COALESCE(
                (
                    SELECT
                        CASE
                            WHEN (sf - sn) <= 0 THEN 0.00
                            ELSE ROUND(((sf - sn) / (sf - sn)) * 100, 2)
                        END
                    FROM (
                        SELECT
                            (SELECT COUNT(*) FROM socios WHERE activo = TRUE) AS sf,
                            COALESCE(
                                (
                                    SELECT COUNT(DISTINCT m1.idsocio)
                                    FROM membresias m1
                                    LEFT JOIN membresias m2 ON m1.idsocio = m2.idsocio AND m2.fechaInicio < f.fecha_inicio
                                    WHERE m1.fechaInicio BETWEEN f.fecha_inicio AND f.fecha_fin
                                    AND m2.idsocio IS NULL
                                ), 0
                            ) AS sn
                    ) AS ret_calc
                ), 0.00
            ) AS tasa_retencion,

            -- 2. ROTACIÓN DE INVENTARIO
            COALESCE(
                (
                    SELECT
                        CASE
                            WHEN (inv_actual + (inv_actual + costo_t)) / 2 = 0 THEN 0.00
                            ELSE ROUND(costo_t / ((inv_actual + (inv_actual + costo_t)) / 2), 4)
                        END
                    FROM (
                        SELECT
                            COALESCE((SELECT SUM(stock * precio) FROM producto), 0) AS inv_actual,
                            COALESCE((
                                SELECT SUM(dv.cantidad * dv.precioUnitario)
                                FROM detalle_venta dv
                                INNER JOIN venta v ON dv.idVenta = v.idVenta
                                WHERE v.fechaVenta BETWEEN f.fecha_inicio AND f.fecha_fin
                            ), 0) AS costo_t
                    ) AS rot_calc
                ), 0.00
            ) AS rotacion_inventario,

            -- 3. ARPU
            COALESCE(
                (
                    SELECT
                        CASE
                            WHEN socios_act = 0 THEN 0.00
                            ELSE ROUND((ing_pagos + ing_ventas) / socios_act, 2)
                        END
                    FROM (
                        SELECT
                            COALESCE((SELECT SUM(monto) FROM pago WHERE estadoPago = 'PAGADO' AND fechaPago BETWEEN f.fecha_inicio AND f.fecha_fin), 0) AS ing_pagos,
                            COALESCE((SELECT SUM(total) FROM venta WHERE fechaVenta BETWEEN f.fecha_inicio AND f.fecha_fin), 0) AS ing_ventas,
                            COALESCE((SELECT COUNT(DISTINCT idsocio) FROM membresias WHERE fechaInicio <= f.fecha_fin AND fechaVencimiento >= f.fecha_inicio), 0) AS socios_act
                    ) AS arpu_calc
                ), 0.00
            ) AS arpu,

            -- 4. MOROSIDAD
            COALESCE(
                (
                    SELECT
                        CASE
                            WHEN activos_totales = 0 THEN 0.00
                            ELSE ROUND((morosos / activos_totales) * 100, 2)
                        END
                    FROM (
                        SELECT
                            COALESCE(
                                (
                                    SELECT COUNT(DISTINCT m.idsocio)
                                    FROM membresias m
                                    LEFT JOIN pago p ON m.idMembresia = p.idMembresia AND p.estadoPago = 'PAGADO'
                                    WHERE m.fechaVencimiento < DATE_ADD(f.fecha_inicio, INTERVAL 14 DAY)
                                    AND p.idPago IS NULL
                                ), 0
                            ) AS morosos,
                            COALESCE((SELECT COUNT(*) FROM socios WHERE activo = TRUE), 0) AS activos_totales
                    ) AS mor_calc
                ), 0.00
            ) AS morosidad,

            -- 5. EFECTIVIDAD DE COBRANZA
            COALESCE(
                (
                    SELECT
                        CASE
                            WHEN facturado = 0 THEN 0.00
                            ELSE ROUND((cobrado / facturado) * 100, 2)
                        END
                    FROM (
                        SELECT
                            COALESCE((SELECT SUM(monto) FROM pago WHERE estadoPago = 'PAGADO' AND fechaPago BETWEEN f.fecha_inicio AND f.fecha_fin), 0) AS cobrado,
                            COALESCE((SELECT SUM(costo) FROM membresias WHERE fechaInicio BETWEEN f.fecha_inicio AND f.fecha_fin), 0) AS facturado
                    ) AS cob_calc
                ), 0.00
            ) AS efectividad_cobranza,

            -- 6. CRECIMIENTO DE MEMBRESÍAS
            COALESCE(
                (
                    SELECT
                        CASE
                            WHEN anterior = 0 THEN 0.00
                            ELSE ROUND(((actual - anterior) / anterior) * 100, 2)
                        END
                    FROM (
                        SELECT
                            COALESCE((SELECT COUNT(*) FROM membresias WHERE f.fecha_fin BETWEEN fechaInicio AND fechaVencimiento), 0) AS actual,
                            COALESCE((SELECT COUNT(*) FROM membresias WHERE LAST_DAY(DATE_SUB(f.fecha_inicio, INTERVAL 1 MONTH)) BETWEEN fechaInicio AND fechaVencimiento), 0) AS anterior
                    ) AS cre_calc
                ), 0.00
            ) AS crecimiento_membresias,

            -- 7. TASA DE DESERCIÓN
            COALESCE(
                (
                    SELECT
                        CASE
                            WHEN activos_ini = 0 THEN 0.00
                            ELSE ROUND((desertores / activos_ini) * 100, 2)
                        END
                    FROM (
                        SELECT
                            COALESCE((SELECT COUNT(DISTINCT idsocio) FROM membresias WHERE f.fecha_inicio BETWEEN fechaInicio AND fechaVencimiento), 0) AS activos_ini,
                            COALESCE(
                                (
                                    SELECT COUNT(DISTINCT m.idsocio)
                                    FROM membresias m
                                    WHERE f.fecha_inicio BETWEEN m.fechaInicio AND m.fechaVencimiento
                                    AND m.idsocio NOT IN (
                                        SELECT DISTINCT idsocio FROM asistencia WHERE fecha BETWEEN f.fecha_inicio AND f.fecha_fin
                                    )
                                    AND m.idsocio NOT IN (
                                        SELECT DISTINCT idsocio FROM membresias WHERE fechaVencimiento > f.fecha_fin
                                    )
                                ), 0
                            ) AS desertores
                    ) AS des_calc
                ), 0.00
            ) AS tasa_desercion

        FROM fechas_mes f
        ORDER BY f.mes;
    "#;

  let kpis_mensuales: Vec<FilaMetricasMes> = sqlx::query_as::<_, FilaMetricasMes>(query_kpis)
    .bind(anio)
    .bind(anio)
    .fetch_all(pool)
    .await?;

  // Revisa que tengas esta dependencia al inicio de tu archivo para manejar el tiempo:
  // use chrono::{Datelike, Local};

  // --- 3. CONSTRUCCIÓN DE FILAS DE TABLA PARA TYPST ---
  let mut filas_tabla_rendimiento = String::new();
  let meses_nombres = [
    "Enero",
    "Febrero",
    "Marzo",
    "Abril",
    "Mayo",
    "Junio",
    "Julio",
    "Agosto",
    "Septiembre",
    "Octubre",
    "Noviembre",
    "Diciembre",
  ];

  // Obtener el año y mes actual del sistema
  let fecha_actual = chrono::Local::now();
  let anio_actual = fecha_actual.year();
  let mes_actual = fecha_actual.month() as i32;

  for i in 0..12 {
    let numero_mes = (i + 1) as i32;
    let mes_nombre = meses_nombres[i];

    // CONTROL DE MESES FUTUROS: Si el año del reporte es el año actual,
    // no permitir meses que superen al mes actual en el calendario.
    if anio == anio_actual && numero_mes > mes_actual {
      break; // Detiene el bucle por completo al llegar a los meses futuros
    }

    // Buscar si existen datos para este mes en el resultado de la BD
    if let Some(kpi) = kpis_mensuales.iter().find(|m| m.mes == numero_mes) {
      // Validación complementaria para meses pasados sin movimiento
      if kpi.rotacion_inventario.is_zero()
        && kpi.arpu.is_zero()
        && kpi.morosidad.is_zero()
        && kpi.efectividad_cobranza.is_zero()
        && kpi.crecimiento_membresias.is_zero()
        && kpi.tasa_desercion.is_zero()
      {
        continue;
      }

      // Renderizado limpio en Soles
      filas_tabla_rendimiento.push_str(&format!(
        "  [{}], [{}%], [{}], [S/ {}], [{}%], [{}%], [{}%], [{}%],\n",
        mes_nombre,
        kpi.tasa_retencion,
        kpi.rotacion_inventario,
        kpi.arpu,
        kpi.morosidad,
        kpi.efectividad_cobranza,
        kpi.crecimiento_membresias,
        kpi.tasa_desercion
      ));
    }
  }

  // --- 4. PLANTILLA TYPST ---
  let mut plantilla_typst = r##"
#set page(
  paper: "a4",
  flipped: true,
  margin: (x: 1.5cm, y: 1.5cm),
  fill: rgb("#fcfcfc")
)
#set text(font: "Liberation Sans", size: 9pt, fill: rgb("#2c3e50"))

// Encabezado
#grid(
  columns: (1fr, 1fr),
  align(left)[
       #text(16pt, weight: "bold", fill: rgb("#1b3a4b"))[Gimnasio & Centro de Bienestar] \
       #text(10pt, fill: gray.darken(30%))[Indicadores de Rendimiento Clave (KPIs)]
  ],
  align(right)[
       #text(24pt, weight: "black", fill: rgb("#0077b6"))[{{ANIO}}] \
       #text(9pt, fill: gray.darken(20%))[Reporte Anual Consolidado]
  ]
)

#v(1em)
#line(length: 100%, stroke: 1.5pt + rgb("#0077b6"))
#v(1em)

// Resumen del Año (Se eliminó la Nota de Auditoría y ahora ocupa el ancho completo)
#grid(
  columns: (1fr),
  gutter: 20pt,
  block(
     width: 100%,
     stroke: 0.5pt + rgb("#bdc3c7"),
     radius: 4pt,
     inset: 10pt,
     fill: white
  )[
     #text(11pt, weight: "bold", fill: rgb("#1b3a4b"))[Resumen Operativo] \
     #v(0.5em)
     - *Asistencia Diaria Promedio en el Año:* #text(weight: "bold", fill: rgb("#2a9d8f"))[{{ASISTENCIA_PROMEDIO}}%]
     - *Frecuencia de Evaluación:* Mensual / Diario consolidado.
     - *Moneda de Operación:* Soles (S/) (Decimales de precisión).
  ]
)

#v(1.5em)

// Tabla de Métricas Consolidadas
#text(12pt, weight: "bold", fill: rgb("#1b3a4b"))[Matriz de Métricas Mensuales]
#v(0.5em)

#table(
  columns: (1.2fr, 1fr, 1fr, 1.2fr, 1fr, 1.2fr, 1.2fr, 1fr),
  inset: 7.5pt,
  align: (center + horizon),
  fill: (x, y) => if y == 0 { rgb("#1b3a4b") } else if calc.even(y) { rgb("#f8f9fa") } else { white },
  stroke: 0.3pt + rgb("#e9ecef"),

  [#text(white, weight: "bold")[Mes]],
  [#text(white, weight: "bold")[Retención]],
  [#text(white, weight: "bold")[Rot. Inv.]],
  [#text(white, weight: "bold")[ARPU]],
  [#text(white, weight: "bold")[Morosidad]],
  [#text(white, weight: "bold")[Efect. Cob.]],
  [#text(white, weight: "bold")[Crec. Memb.]],
  [#text(white, weight: "bold")[Deserción]],

{{FILAS_TABLA}}
)
"##.to_string();

  // --- 5. REEMPLAZAR MARCADORES ---
  plantilla_typst = plantilla_typst
    .replace("{{ANIO}}", &anio.to_string())
    .replace(
      "{{ASISTENCIA_PROMEDIO}}",
      &asistencia_promedio_anual.to_string(),
    )
    .replace("{{FILAS_TABLA}}", &filas_tabla_rendimiento);

  // --- 6. ESCRITURA Y COMPILACIÓN ---
  let archivo_temporal = format!("reporte_anual_consolidado_{}.typ", anio);
  let pdf_salida = format!("reporte_anual_kpi_{}.pdf", anio);

  tracing::info!("Escribiendo archivo temporal: {}", archivo_temporal);
  fs::write(&archivo_temporal, &plantilla_typst)?;

  let output = Command::new("typst")
    .arg("compile")
    .arg(&archivo_temporal)
    .arg(&pdf_salida)
    .output();

  let _ = fs::remove_file(&archivo_temporal);

  let output = match output {
    Ok(out) => out,
    Err(err) => {
      tracing::error!("Error crítico al intentar ejecutar 'typst': {:?}", err);
      return Err(err.into());
    }
  };

  if output.status.success() {
    tracing::info!("PDF consolidado generado exitosamente: {}", pdf_salida);
    Ok(pdf_salida)
  } else {
    let error = String::from_utf8_lossy(&output.stderr);
    tracing::warn!("Error al compilar el reporte con Typst:\n{}", error);
    Err(format!("Error en compilación de Typst: {}", error).into())
  }
}
