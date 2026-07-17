use sqlx::MySqlPool;
use std::fs;
use std::process::Command;

// Estructura ligera optimizada para el reporte (eliminamos dirección si no se usa)
#[derive(Debug)]
pub struct FilaSocioInactivo {
  pub id_socio: u32,
  pub nombre_completo: String,
  pub dni: String,
  pub telefono: Option<String>,
  pub correo: Option<String>,
}

pub async fn generar_reporte_socios_inactivos(
  pool: &MySqlPool,
) -> Result<String, Box<dyn std::error::Error>> {
  // 1. Consultar TODOS los socios que están inactivos (activo = 0)
  // Usamos "nombre_completo!" con el signo de exclamación para forzar a que NO sea Option
  let socios = sqlx::query_as!(
    FilaSocioInactivo,
    r#"
        SELECT
            idsocio AS "id_socio: u32",
            CONCAT(apellidos, ', ', nombre) AS "nombre_completo!",
            dni,
            telefono,
            correo
        FROM socios
        WHERE activo = 0
        ORDER BY apellidos ASC, nombre ASC;
        "#
  )
  .fetch_all(pool)
  .await?;

  // 2. Construir dinámicamente las filas de la tabla para Typst
  let mut filas_tabla_typst = String::new();
  for socio in socios {
    let tel = socio.telefono.unwrap_or_else(|| "-".to_string());
    let mail = socio.correo.unwrap_or_else(|| "-".to_string());

    filas_tabla_typst.push_str(&format!(
      "        [{}], \"{}\", \"{}\", \"{}\", \"{}\",\n",
      socio.id_socio, socio.nombre_completo, socio.dni, tel, mail
    ));
  }

  // Si no hay socios inactivos, agregamos una fila informativa
  if filas_tabla_typst.is_empty() {
    filas_tabla_typst = "        table.cell(colspan: 5, align: center)[_No se encontraron socios inactivos en el sistema._]\n".to_string();
  }

  // 3. Plantilla Typst (A4 Vertical)
  let mut contenido_plantilla = r##"
    #set page(
      paper: "a4",
      margin: (x: 2cm, y: 2.5cm),
      fill: rgb("#fdfdfd"),
      footer: context {
        let page_number = counter(page).get().first()
        let total_pages = counter(page).final().first()
        align(center)[
          #text(9pt, fill: gray)[Página #page_number de #total_pages]
        ]
      }
    )
    #set text(font: "Liberation Sans", size: 10pt, fill: rgb("#2c3e50"))

    // Encabezado corporativo
    #grid(
      columns: (1fr, 1fr),
      align(left)[#text(9pt, fill: gray.darken(20%))[Sistema de Gestión de Socios]],
      align(right)[#text(9pt, fill: gray.darken(20%))[Reporte de Control Interno]]
    )
    #line(length: 100%, stroke: 0.5pt + rgb("#bdc3c7"))
    #v(1em)

    // Título Principal
    #align(center)[
      #text(18pt, weight: "bold", fill: rgb("#c0392b"))[Socios Eliminados / Inactivos]
    ]
    #v(1.5em)

    // Tabla de Datos
    #table(
      columns: (0.6fr, 2fr, 1fr, 1.2fr, 2fr),
      stroke: (x, y) => if y == 0 { none } else { 0.5pt + rgb("#eaeded") },
      fill: (x, y) => if y == 0 { rgb("#c0392b") } else if calc.even(y) { rgb("#f9f9f9") } else { white },
      align: (col, row) => if row == 0 { center + horizon } else if col == 0 { center } else { left + horizon },

      table.header(
        [*ID*], [*Socio (Apellidos, Nombre)*], [*DNI*], [*Teléfono*], [*Correo Electrónico*]
      ),

      // Inyección de filas desde Rust
{{FILAS_TABLA}}
    )
    "##
    .to_string();

  // 4. Inyectar las filas generadas en la plantilla
  contenido_plantilla = contenido_plantilla.replace("{{FILAS_TABLA}}", &filas_tabla_typst);

  // 5. Escritura y compilación nativa
  let archivo_temporal = "reporte_socios_inactivos_temp.typ".to_string();
  let pdf_salida = "reporte_socios_inactivos.pdf".to_string();
  fs::write(&archivo_temporal, contenido_plantilla)?;

  tracing::info!("Compilando PDF del listado de socios inactivos con Typst...");
  let output = Command::new("typst")
    .arg("compile")
    .arg(&archivo_temporal)
    .arg(&pdf_salida)
    .output()?;

  let _ = fs::remove_file(archivo_temporal);

  // 6. Validación del proceso
  if output.status.success() {
    tracing::info!("¡PDF del listado generado exitosamente en: {}!", pdf_salida);
    Ok(pdf_salida)
  } else {
    let error = String::from_utf8_lossy(&output.stderr);
    tracing::warn!("Error al compilar el listado con Typst:\n{}", error);
    Err(format!("Error en compilación de Typst: {}", error).into())
  }
}
