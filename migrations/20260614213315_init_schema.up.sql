CREATE TABLE IF NOT EXISTS usuarios (
    idUsuario INT UNSIGNED AUTO_INCREMENT,
    nombreUsuario VARCHAR(50) NOT NULL UNIQUE,
    contrasena VARCHAR(255) NOT NULL,
    rol VARCHAR(30) NOT NULL,
    activo BOOLEAN NOT NULL DEFAULT TRUE,
    CONSTRAINT PK_usuarios PRIMARY KEY (idUsuario)
);

CREATE TABLE IF NOT EXISTS socios (
    idsocio INT UNSIGNED AUTO_INCREMENT,
    nombre VARCHAR(50) NOT NULL,
    apellidos VARCHAR(50) NOT NULL,
    dni VARCHAR(8) NOT NULL UNIQUE,
    telefono VARCHAR(15),
    correo VARCHAR(100),
    direccion VARCHAR(150),
    activo BOOLEAN NOT NULL DEFAULT TRUE,
    CONSTRAINT PK_socios PRIMARY KEY (idsocio)
);

CREATE TABLE IF NOT EXISTS producto (
    idProducto INT UNSIGNED AUTO_INCREMENT,
    nombreProducto VARCHAR(100) NOT NULL,
    descripcion TEXT,
    precio DECIMAL(10,2) NOT NULL,
    stock INT UNSIGNED NOT NULL,
    CONSTRAINT PK_producto PRIMARY KEY (idProducto)
);

CREATE TABLE IF NOT EXISTS membresias (
    idMembresia INT UNSIGNED AUTO_INCREMENT,
    idsocio INT UNSIGNED NOT NULL,
    tipoMembresia VARCHAR(50) NOT NULL,
    fechaInicio DATE NOT NULL,
    fechaVencimiento DATE NOT NULL,
    costo DECIMAL(10,2) NOT NULL,
    CONSTRAINT PK_membresias PRIMARY KEY (idMembresia),
    CONSTRAINT FK_membresias_socios FOREIGN KEY (idsocio)
        REFERENCES socios (idsocio)
        ON DELETE CASCADE
        ON UPDATE CASCADE
);

CREATE TABLE IF NOT EXISTS asistencia (
    idAsistencia INT UNSIGNED AUTO_INCREMENT,
    idsocio INT UNSIGNED NOT NULL,
    fecha DATE NOT NULL,
    horaIngreso TIME NOT NULL,
    resultadoValidacion VARCHAR(50) NOT NULL,
    CONSTRAINT PK_asistencia PRIMARY KEY (idAsistencia),
    CONSTRAINT FK_asistencia_socios FOREIGN KEY (idsocio)
        REFERENCES socios (idsocio)
        ON DELETE CASCADE
        ON UPDATE CASCADE
);

CREATE TABLE IF NOT EXISTS reporte (
    idReporte INT UNSIGNED AUTO_INCREMENT,
    idUsuario INT UNSIGNED NOT NULL,
    tipoReporte VARCHAR(50) NOT NULL,
    fechaGeneracion TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    periodoInicio DATE NOT NULL,
    periodoFin DATE NOT NULL,
    CONSTRAINT PK_reporte PRIMARY KEY (idReporte),
    CONSTRAINT FK_reporte_usuarios FOREIGN KEY (idUsuario)
        REFERENCES usuarios (idUsuario)
        ON DELETE CASCADE
        ON UPDATE CASCADE
);

CREATE TABLE IF NOT EXISTS venta (
    idVenta INT UNSIGNED AUTO_INCREMENT,
    idsocio INT UNSIGNED,
    idUsuario INT UNSIGNED NOT NULL,
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
        );

CREATE TABLE IF NOT EXISTS pago (
    idPago INT UNSIGNED AUTO_INCREMENT,
    idMembresia INT UNSIGNED NOT NULL,
    fechaPago TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    monto DECIMAL(10,2) NOT NULL,
    metodoPago VARCHAR(30) NOT NULL,
    estadoPago VARCHAR(20) NOT NULL,
    CONSTRAINT PK_pago PRIMARY KEY (idPago),
    CONSTRAINT FK_pago_membresias FOREIGN KEY (idMembresia)
        REFERENCES membresias (idMembresia)
        ON DELETE CASCADE
        ON UPDATE CASCADE
);

CREATE TABLE IF NOT EXISTS detalle_venta (
    idDetalleVenta INT UNSIGNED AUTO_INCREMENT,
    idVenta INT UNSIGNED NOT NULL,
    idProducto INT UNSIGNED NOT NULL,
    cantidad INT UNSIGNED NOT NULL,
    precioUnitario DECIMAL(10,2) NOT NULL,
    CONSTRAINT PK_detalle_venta PRIMARY KEY (idDetalleVenta),
    CONSTRAINT FK_detalle_venta_venta FOREIGN KEY (idVenta)
        REFERENCES venta (idVenta)
        ON DELETE CASCADE
        ON UPDATE CASCADE,
    CONSTRAINT FK_detalle_venta_producto FOREIGN KEY (idProducto)
        REFERENCES producto (idProducto)
        ON DELETE CASCADE
        ON UPDATE CASCADE
);
