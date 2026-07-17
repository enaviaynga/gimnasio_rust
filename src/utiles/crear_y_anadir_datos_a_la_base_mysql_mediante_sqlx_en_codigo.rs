use anyhow::Result;
use sqlx::{
  MySql, migrate,
  mysql::{self, MySqlConnectOptions, MySqlPoolOptions},
};
use tracing::info;

pub(crate) async fn migracion_completa(url_bd: &str) -> Result<()> {
  let pool = MySqlPoolOptions::new().connect(url_bd).await?;
  // pendiente: mala practica insertar datos en migration, quitarlo luego que termine el ciclo
  // sqlx::migrate!("./migrations").run(&pool).await?;
  // info!("migracion completada");
  Ok(())
}

pub(crate) async fn crear_tablas(url_bd: &str) -> Result<()> {
  let pool = MySqlPoolOptions::new().connect(url_bd).await?;

  sqlx::query(
    r#"CREATE TABLE IF NOT EXISTS usuarios (
        idUsuario INT AUTO_INCREMENT,
        nombreUsuario VARCHAR(50) NOT NULL UNIQUE,
        contrasena VARCHAR(255) NOT NULL,
        rol VARCHAR(30) NOT NULL,
        CONSTRAINT PK_usuarios PRIMARY KEY (idUsuario)
    );"#,
  )
  .execute(&pool)
  .await?;
  sqlx::query(
    r#"CREATE TABLE IF NOT EXISTS socios (
        idsocio INT AUTO_INCREMENT,
        nombre VARCHAR(50) NOT NULL,
        apellidos VARCHAR(50) NOT NULL,
        dni VARCHAR(8) NOT NULL UNIQUE,
        telefono VARCHAR(15),
        correo VARCHAR(100),
        direccion VARCHAR(150),
        CONSTRAINT PK_socios PRIMARY KEY (idsocio)
    );"#,
  )
  .execute(&pool)
  .await?;
  sqlx::query(
    r#"CREATE TABLE IF NOT EXISTS membresias (
        idMembresia INT AUTO_INCREMENT,
        idsocio INT NOT NULL,
        tipoMembresia VARCHAR(50) NOT NULL, -- Ej: Mensual, Trimestral, Anual
        fechaInicio DATE NOT NULL,
        fechaVencimiento DATE NOT NULL,
        costo DECIMAL(10,2) NOT NULL,
        CONSTRAINT PK_membresias PRIMARY KEY (idMembresia),
        CONSTRAINT FK_membresias_socios FOREIGN KEY (idsocio)
            REFERENCES socios (idsocio)
            ON DELETE CASCADE
            ON UPDATE CASCADE
    );"#,
  )
  .execute(&pool)
  .await?;
  sqlx::query(
    r#"CREATE TABLE IF NOT EXISTS pago (
        idPago INT AUTO_INCREMENT,
        idMembresia INT NOT NULL,
        fechaPago TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
        monto DECIMAL(10,2) NOT NULL,
        metodoPago VARCHAR(30) NOT NULL, -- Ej: Efectivo, Tarjeta, Yape
        estadoPago VARCHAR(20) NOT NULL, -- Ej: Pagado, Pendiente
        CONSTRAINT PK_pago PRIMARY KEY (idPago),
        CONSTRAINT FK_pago_membresias FOREIGN KEY (idMembresia)
            REFERENCES membresias (idMembresia)
            ON DELETE CASCADE
            ON UPDATE CASCADE
    );"#,
  )
  .execute(&pool)
  .await?;
  sqlx::query(
    r#"CREATE TABLE IF NOT EXISTS asistencia (
        idAsistencia INT AUTO_INCREMENT,
        idsocio INT NOT NULL,
        fecha DATE NOT NULL,
        horaIngreso TIME NOT NULL,
        resultadoValidacion VARCHAR(50) NOT NULL, -- Ej: 'Permitido', 'Denegado - Vencido'
        CONSTRAINT PK_asistencia PRIMARY KEY (idAsistencia),
        CONSTRAINT FK_asistencia_socios FOREIGN KEY (idsocio)
            REFERENCES socios (idsocio)
            ON DELETE CASCADE
            ON UPDATE CASCADE
    );"#,
  )
  .execute(&pool)
  .await?;
  sqlx::query(
    r#"CREATE TABLE IF NOT EXISTS producto (
        idProducto INT AUTO_INCREMENT,
        nombreProducto VARCHAR(100) NOT NULL,
        descripcion TEXT,
        precio DECIMAL(10,2) NOT NULL,
        stock INT NOT NULL,
        CONSTRAINT PK_producto PRIMARY KEY (idProducto)
    );"#,
  )
  .execute(&pool)
  .await?;
  sqlx::query(
    r#"CREATE TABLE IF NOT EXISTS reporte (
        idReporte INT AUTO_INCREMENT,
        idUsuario INT NOT NULL,
        tipoReporte VARCHAR(50) NOT NULL, -- Ej: Financiero, Asistencias, Inventario
        fechaGeneracion TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
        periodoInicio DATE NOT NULL,
        periodoFin DATE NOT NULL,
        CONSTRAINT PK_reporte PRIMARY KEY (idReporte),
        CONSTRAINT FK_reporte_usuarios FOREIGN KEY (idUsuario)
            REFERENCES usuarios (idUsuario)
            ON DELETE CASCADE
            ON UPDATE CASCADE
    );"#,
  )
  .execute(&pool)
  .await?;
  sqlx::query(
    r#"CREATE TABLE IF NOT EXISTS venta (
        idVenta INT AUTO_INCREMENT,
        idsocio INT NOT NULL,       -- Qué socio compra el suplemento/bebida
        idUsuario INT NOT NULL,     -- Qué recepcionista/administrador realiza la venta
        fechaVenta TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
        total DECIMAL(10,2) NOT NULL,
        CONSTRAINT PK_venta PRIMARY KEY (idVenta),
        CONSTRAINT FK_venta_socios FOREIGN KEY (idsocio)
            REFERENCES socios (idsocio)
            ON DELETE CASCADE
            ON UPDATE CASCADE,
        CONSTRAINT FK_venta_usuarios FOREIGN KEY (idUsuario)
            REFERENCES usuarios (idUsuario)
            ON DELETE CASCADE
            ON UPDATE CASCADE
    );"#,
  )
  .execute(&pool)
  .await?;
  sqlx::query(
    r#"CREATE TABLE IF NOT EXISTS detalle_venta (
        idDetalleVenta INT AUTO_INCREMENT,
        idVenta INT NOT NULL,
        idProducto INT NOT NULL,
        cantidad INT NOT NULL,
        precioUnitario DECIMAL(10,2) NOT NULL, -- Histórico del precio al momento de la compra
        CONSTRAINT PK_detalle_venta PRIMARY KEY (idDetalleVenta),
        CONSTRAINT FK_detalle_venta_venta FOREIGN KEY (idVenta)
            REFERENCES venta (idVenta)
            ON DELETE CASCADE
            ON UPDATE CASCADE,
        CONSTRAINT FK_detalle_venta_producto FOREIGN KEY (idProducto)
            REFERENCES producto (idProducto)
            ON DELETE CASCADE
            ON UPDATE CASCADE
            );"#,
  )
  .execute(&pool)
  .await?;
  Ok(())
}

pub(crate) async fn insertar_datos(url_bd: &str) -> Result<()> {
  let pool = MySqlPoolOptions::new().connect(url_bd).await?;

  sqlx::query(
    r#"INSERT INTO usuarios (nombreUsuario, contrasena, rol) VALUES
    ('admin_carlos', 'Admin-123', 'Administrador'),
    ('recepcion_ana', 'Ana--456', 'Recepcionista'),
    ('trainer_luis', 'Luis-789', 'Entrenador');"#,
  )
  .execute(&pool)
  .await?;
  sqlx::query(
    r#"INSERT INTO socios (nombre, apellidos, dni, telefono, correo, direccion) VALUES
    ('Juan', 'Perez', '12345678', '999888777', 'juan@email.com', 'Av. Lima 123'),
    ('Maria', 'Lopez', '87654321', '911222333', 'maria@email.com', 'Calle Cusco 456'),
    ('Carlos', 'Ruiz', '55544433', '988777666', 'carlos@email.com', 'Av. Arequipa 789');"#,
  )
  .execute(&pool)
  .await?;
  sqlx::query(
    r#"INSERT INTO membresias (idsocio, tipoMembresia, fechaInicio, fechaVencimiento, costo) VALUES
    (1, 'Mensual', '2026-06-01', '2026-07-01', 100.00),
    (2, 'Trimestral', '2026-05-15', '2026-08-15', 250.00),
    (3, 'Anual', '2026-01-01', '2027-01-01', 800.00);"#,
  )
  .execute(&pool)
  .await?;
  sqlx::query(
    r#"INSERT INTO pago (idMembresia, monto, metodoPago, estadoPago) VALUES
    (1, 100.00, 'Efectivo', 'Pagado'),
    (2, 250.00, 'Tarjeta', 'Pagado'),
    (3, 800.00, 'Yape', 'Pagado');"#,
  )
  .execute(&pool)
  .await?;
  sqlx::query(
    r#"INSERT INTO asistencia (idsocio, fecha, horaIngreso, resultadoValidacion) VALUES
    (1, '2026-06-13', '08:00:00', 'Permitido'),
    (2, '2026-06-13', '09:30:00', 'Permitido'),
    (3, '2026-06-13', '10:00:00', 'Permitido');"#,
  )
  .execute(&pool)
  .await?;
  sqlx::query(
    r#"INSERT INTO producto (nombreProducto, descripcion, precio, stock) VALUES
    ('Whey Protein', 'Proteína de suero 2kg', 150.00, 20),
    ('BCAA', 'Aminoácidos ramificados', 80.00, 50),
    ('Creatina', 'Monohidratada 500g', 60.00, 30);"#,
  )
  .execute(&pool)
  .await?;
  sqlx::query(
    r#"INSERT INTO reporte (idUsuario, tipoReporte, periodoInicio, periodoFin) VALUES
    (1, 'Financiero', '2026-06-01', '2026-06-13'),
    (1, 'Asistencias', '2026-06-01', '2026-06-13'),
    (1, 'Inventario', '2026-06-01', '2026-06-13');"#,
  )
  .execute(&pool)
  .await?;
  sqlx::query(
    r#"INSERT INTO venta (idsocio, idUsuario, total) VALUES
    (1, 2, 150.00),
    (2, 2, 80.00),
    (3, 2, 60.00);"#,
  )
  .execute(&pool)
  .await?;
  sqlx::query(
    r#"INSERT INTO detalle_venta (idVenta, idProducto, cantidad, precioUnitario) VALUES
    (1, 1, 1, 150.00),
    (2, 2, 1, 80.00),
    (3, 3, 1, 60.00);"#,
  )
  .execute(&pool)
  .await?;
  Ok(())
}

// pendiente: revisar la documentacion
// pub(crate) async fn migracion_init_up(url_bd: &str) -> Result<()> {
//     let pool = MySqlPoolOptions::new().connect(url_bd).await?;
//     let texto = std::fs::read("./migrations")?;

//     sqlx::query_file!("./migrations/20260614213315_init_schema.up.sql")
//         .execute(&pool)
//         .await?;
//     info!("migracion completada");
//     Ok(())
// }
