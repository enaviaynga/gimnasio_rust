use freya::prelude::*;
use rust_decimal::Decimal;
use sqlx::{MySql, Pool};
use std::str::FromStr;

use crate::vista::{
  componentes::boton_cerrar_y_volver_al_inicio::boton_cerrar,
  gestor::venta::{DbPoolContext, PantallaVenta},
};

#[derive(PartialEq, Clone)]
pub struct NuevoProducto;

impl Component for NuevoProducto {
  fn render(&self) -> impl IntoElement {
    let db_ctx: State<DbPoolContext> = use_consume();

    let nuevo_nombre = use_state(String::new);
    let nueva_descripcion = use_state(String::new);
    let nuevo_precio = use_state(String::new);
    let nuevo_stock = use_state(String::new);

    let id_producto_reposicion = use_state(String::new);
    let cantidad_reposicion = use_state(String::new);

    let id_producto_venta = use_state(String::new);
    let cantidad_venta = use_state(String::new);
    let id_usuario_mock = use_state(|| "1".to_string());
    let id_socio_mock = use_state(|| "1".to_string());

    let mut info_mensaje = use_state(String::new);

    let pantalla: State<PantallaVenta> = use_consume();

    rect()
            .width(Size::fill())
            .height(Size::fill())
            .padding(Gaps::new_all(20.0))
            .vertical()
            .child(
                label()
                    .text("Módulo Completo de Inventario y Ventas")
                    .font_size(26.0)
                    .font_weight(FontWeight::BOLD),
            )
            .child(
                rect()
                    .margin(Gaps::new_symmetric(10.0, 0.0))
                    .height(Size::auto())
                    .child(
                        label()
                            .text(info_mensaje.read().clone())
                            .color(Color::from_rgb(0, 120, 215)),
                    ),
            )
            .child(
                rect()
                    .direction(Direction::Horizontal)
                    .width(Size::fill())
                    .height(Size::flex(1.0))
                    .spacing(15.0)

                    .child(
                        rect()
                            .width(Size::fill())
                            .vertical()
                            .padding(Gaps::new_all(15.0))
                            .corner_radius(CornerRadius::new_all(10.0))
                            .child(
                                label()
                                    .text("Registrar Nuevo Producto")
                                    .font_size(18.0)
                                    .font_weight(FontWeight::BOLD),
                            )
                            .child(
                                rect().margin(Gaps::new_symmetric(4.0, 0.0))
                                    .child(label().text("Nombre del Producto"))
                                    .child(Input::new(nuevo_nombre).width(Size::fill())),
                            )
                            .child(
                                rect().margin(Gaps::new_symmetric(4.0, 0.0))
                                    .child(label().text("Descripción"))
                                    .child(Input::new(nueva_descripcion).width(Size::fill())),
                            )
                            .child(
                                rect().margin(Gaps::new_symmetric(4.0, 0.0))
                                    .child(label().text("Precio (Ej: 15.50)"))
                                    .child(Input::new(nuevo_precio).width(Size::fill())),
                            )
                            .child(
                                rect().margin(Gaps::new_symmetric(4.0, 0.0))
                                    .child(label().text("Stock Inicial"))
                                    .child(Input::new(nuevo_stock).width(Size::fill())),
                            )
                            .child(
                                Button::new()
                                    .margin(Gaps::new(12.0, 0.0, 0.0, 0.0))
                                    .on_press(move |_| {
                                        let pool = db_ctx.read().pool.clone();
                                        let nombre = nuevo_nombre.read().clone();
                                        let desc = nueva_descripcion.read().clone();
                                        let precio_str = nuevo_precio.read().clone();
                                        let stock_str = nuevo_stock.read().clone();
                                        let mut msg = info_mensaje;

                                        spawn(async move {
                                            if nombre.trim().is_empty() {
                                                *msg.write() = "El nombre del producto es obligatorio.".to_string();
                                                return;
                                            }

                                            let precio = match Decimal::from_str(&precio_str) {
                                                Ok(p) => p,
                                                Err(_) => {
                                                    *msg.write() = "Precio inválido (usa formato decimal 0.00).".to_string();
                                                    return;
                                                }
                                            };

                                            let stock = match stock_str.parse::<u32>() {
                                                Ok(s) => s,
                                                Err(_) => {
                                                    *msg.write() = "Stock inicial inválido.".to_string();
                                                    return;
                                                }
                                            };

                                            let resultado = sqlx::query!(
                                                "INSERT INTO producto (nombreProducto, descripcion, precio, stock) VALUES (?, ?, ?, ?)",
                                                nombre, desc, precio, stock
                                            )
                                            .execute(&pool)
                                            .await;

                                            match resultado {
                                                Ok(res) => {
                                                    *msg.write() = format!("¡Producto creado exitosamente! ID asignado: {}.", res.last_insert_id());
                                                }
                                                Err(e) => *msg.write() = format!("Error al guardar producto: {}", e),
                                            }
                                        });
                                    })
                                    .child(label().text("Guardar Producto")),
                            ),
                    )

                    .child(
                        rect()
                            .width(Size::fill())
                            .vertical()
                            .padding(Gaps::new_all(15.0))
                            .corner_radius(CornerRadius::new_all(10.0))
                            .child(
                                label()
                                    .text("Reponer Almacén (Aumentar)")
                                    .font_size(18.0)
                                    .font_weight(FontWeight::BOLD),
                            )
                            .child(
                                rect().margin(Gaps::new_symmetric(6.0, 0.0))
                                    .child(label().text("ID Producto"))
                                    .child(Input::new(id_producto_reposicion).width(Size::fill())),
                            )
                            .child(
                                rect().margin(Gaps::new_symmetric(6.0, 0.0))
                                    .child(label().text("Cantidad a Añadir"))
                                    .child(Input::new(cantidad_reposicion).width(Size::fill())),
                            )
                            .child(
                                Button::new()
                                    .margin(Gaps::new(12.0, 0.0, 0.0, 0.0))
                                    .on_press(move |_| {
                                        let pool = db_ctx.read().pool.clone();
                                        let id_prod_str = id_producto_reposicion.read().clone();
                                        let cant_str = cantidad_reposicion.read().clone();
                                        let mut msg = info_mensaje;

                                        spawn(async move {
                                            if let (Ok(id), Ok(cant)) = (id_prod_str.parse::<u32>(), cant_str.parse::<u32>()) {
                                                let resultado = sqlx::query!(
                                                    "UPDATE producto SET stock = stock + ? WHERE idProducto = ?",
                                                    cant, id
                                                )
                                                .execute(&pool)
                                                .await;

                                                match resultado {
                                                    Ok(res) if res.rows_affected() > 0 => {
                                                        *msg.write() = format!("¡Stock actualizado! Se añadieron {} unidades al producto {}.", cant, id);
                                                    }
                                                    Ok(_) => *msg.write() = "Error: El ID del producto no existe.".to_string(),
                                                    Err(e) => *msg.write() = format!("Error en Base de Datos: {}", e),
                                                }
                                            } else {
                                                *msg.write() = "Por favor, introduce valores numéricos válidos.".to_string();
                                            }
                                        });
                                    })
                                    .child(label().text("Procesar Entrada")),
                            ),
                    )

                    .child(
                        rect()
                            .width(Size::fill())
                            .vertical()
                            .padding(Gaps::new_all(15.0))
                            .corner_radius(CornerRadius::new_all(10.0))
                            .child(
                                label()
                                    .text("Generar Venta (Disminuir)")
                                    .font_size(18.0)
                                    .font_weight(FontWeight::BOLD),
                            )
                            .child(
                                rect().margin(Gaps::new_symmetric(4.0, 0.0))
                                    .child(label().text("ID Producto"))
                                    .child(Input::new(id_producto_venta).width(Size::fill())),
                            )
                            .child(
                                rect().margin(Gaps::new_symmetric(4.0, 0.0))
                                    .child(label().text("Cantidad a Vender"))
                                    .child(Input::new(cantidad_venta).width(Size::fill())),
                            )
                            .child(
                                rect().margin(Gaps::new_symmetric(4.0, 0.0))
                                    .child(label().text("ID Usuario (Vendedor)"))
                                    .child(Input::new(id_usuario_mock).width(Size::fill())),
                            )
                            .child(
                                Button::new()
                                    .margin(Gaps::new(12.0, 0.0, 0.0, 0.0))
                                    .on_press(move |_| {
                                        let pool = db_ctx.read().pool.clone();
                                        let id_prod_str = id_producto_venta.read().clone();
                                        let cant_str = cantidad_venta.read().clone();
                                        let id_user_str = id_usuario_mock.read().clone();
                                        let mut msg = info_mensaje;

                                        spawn(async move {
                                            let id_prod = match id_prod_str.parse::<u32>() {
                                                Ok(v) => v,
                                                Err(_) => { *msg.write() = "ID Producto inválido".to_string(); return; }
                                            };
                                            let cant = match cant_str.parse::<u32>() {
                                                Ok(v) => v,
                                                Err(_) => { *msg.write() = "Cantidad inválida".to_string(); return; }
                                            };
                                            let id_user = match id_user_str.parse::<u32>() {
                                                Ok(v) => v,
                                                Err(_) => { *msg.write() = "ID Usuario inválido".to_string(); return; }
                                            };

                                            let mut tx = match pool.begin().await {
                                                Ok(t) => t,
                                                Err(e) => { *msg.write() = format!("Error de transacción: {}", e); return; }
                                            };

                                            let datos_producto = sqlx::query!(
                                                "SELECT precio, stock FROM producto WHERE idProducto = ? FOR UPDATE",
                                                id_prod
                                            )
                                            .fetch_optional(&mut *tx)
                                            .await;

                                            if let Ok(Some(prod)) = datos_producto {
                                                if prod.stock < cant {
                                                    *msg.write() = format!("Stock insuficiente. Disponible: {}", prod.stock);
                                                    return;
                                                }

                                                let total_venta = prod.precio * Decimal::from(cant);

                                                if let Err(e) = sqlx::query!(
                                                    "UPDATE producto SET stock = stock - ? WHERE idProducto = ?",
                                                    cant, id_prod
                                                )
                                                .execute(&mut *tx)
                                                .await
                                                {
                                                    *msg.write() = format!("Error restando stock: {}", e);
                                                    return;
                                                }

                                                if let Err(e) = sqlx::query!(
                                                    "INSERT INTO venta (idUsuario, total) VALUES (?, ?)",
                                                    id_user, total_venta
                                                )
                                                .execute(&mut *tx)
                                                .await
                                                {
                                                    *msg.write() = format!("Error registrando ticket de venta: {}", e);
                                                    return;
                                                }

                                                if tx.commit().await.is_ok() {
                                                    *msg.write() = format!("Venta completada. Total cobrado: ${}", total_venta);
                                                } else {
                                                    *msg.write() = "Error al confirmar la transacción.".to_string();
                                                }
                                            } else {
                                                *msg.write() = "El producto especificado no existe.".to_string();
                                            }
                                        });
                                    })
                                    .child(label().text("Efectuar Venta")),
                            ),
                    ),
            )
            .child(boton_cerrar(pantalla, PantallaVenta::Venta))
  }
}
