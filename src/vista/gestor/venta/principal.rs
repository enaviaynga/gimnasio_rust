use freya::prelude::*;
use rust_decimal::Decimal;
use sqlx::{MySql, Pool};
use std::str::FromStr;

use crate::vista::gestor::venta::{DbPoolContext, PantallaVenta};

#[derive(Clone, PartialEq)]
struct ProductoData {
  id_producto: u32,
  nombre_producto: String,
  descripcion: Option<String>,
  precio: Decimal,
  stock: u32,
}

#[derive(PartialEq, Clone)]
pub struct VentaProducto;

impl Component for VentaProducto {
  fn render(&self) -> impl IntoElement {
    let db_ctx: State<DbPoolContext> = use_consume();
    let mut pantalla: State<PantallaVenta> = use_consume();

    let id_producto_venta = use_state(String::new);
    let cantidad_venta = use_state(String::new);
    let id_usuario_mock = use_state(|| "1".to_string());
    let mut info_mensaje = use_state(String::new);

    let mut productos = use_state(Vec::<ProductoData>::new);

    let pool_for_effect = db_ctx.read().pool.clone();
    let mut productos_setter = productos;

    use_side_effect(move || {
      let pool = pool_for_effect.clone();
      let mut setter = productos_setter;
      spawn(async move {
        let resultado = sqlx::query!(
          "SELECT idProducto, nombreProducto, descripcion, precio, stock FROM producto"
        )
        .fetch_all(&pool)
        .await;

        if let Ok(filas) = resultado {
          let lista = filas
            .into_iter()
            .map(|f| ProductoData {
              id_producto: f.idProducto,
              nombre_producto: f.nombreProducto,
              descripcion: f.descripcion,
              precio: f.precio,
              stock: f.stock,
            })
            .collect::<Vec<_>>();
          *setter.write() = lista;
        }
      });
    });

    rect()
            .width(Size::fill())
            .height(Size::fill())
            .padding(Gaps::new_all(20.0))
            .vertical()
            .child(
                rect()
                    .direction(Direction::Horizontal)
                    .margin(Gaps::new(0., 0., 15.0, 0.))
                    .child(
                        Button::new()
                            .margin(Gaps::new(0., 10.0, 0., 0.))
                            .on_press(move |_| {
                                *pantalla.write() = PantallaVenta::NuevoStock;
                            })
                            .child(label().text("Añadir Producto")),
                    )
                    .child(
                        Button::new()
                            .on_press(move |_| {
                                *pantalla.write() = PantallaVenta::Stock;
                            })
                            .child(label().text("Reponer Stock")),
                    ),
            )
            .child(
                label()
                    .text("Registrar Nueva Venta")
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
                    .center()
                    .width(Size::percent(75.0))
                    .horizontal()
                    .padding(Gaps::new_all(15.0))
                    .content(Content::Flex)
                    .corner_radius(CornerRadius::new_all(10.0))
                    .spacing(5.)
                    .child(
                        rect()
                            .width(Size::flex(1.))
                            .child(label().text("ID de Producto"))
                            .child(Input::new(id_producto_venta).width(Size::fill())),
                    )
                    .child(
                        rect()
                            .width(Size::flex(1.))
                            .child(label().text("Cantidad a Vender"))
                            .child(Input::new(cantidad_venta).width(Size::fill())),
                    )
                    .child(
                        rect()
                            .width(Size::flex(1.))
                            .child(label().text("ID Usuario (Vendedor)"))
                            .child(Input::new(id_usuario_mock).width(Size::fill())),
                    )
                    .child(
                        Button::new()
                            .margin(Gaps::new(16.0, 0.0, 0.0, 0.0))
                            .on_press(move |_| {
                                let pool = db_ctx.read().pool.clone();
                                let id_prod_str = id_producto_venta.read().clone();
                                let cant_str = cantidad_venta.read().clone();
                                let id_user_str = id_usuario_mock.read().clone();
                                let mut msg = info_mensaje;
                                let mut setter_actualizar = productos; // clon para refrescar tabla tras vender

                                spawn(async move {
                                    let id_prod = match id_prod_str.parse::<u32>() {
                                        Ok(v) => v,
                                        Err(_) => {
                                            *msg.write() = "ID Producto inválido.".to_string();
                                            return;
                                        }
                                    };
                                    let cant = match cant_str.parse::<u32>() {
                                        Ok(v) => v,
                                        Err(_) => {
                                            *msg.write() = "Cantidad inválida.".to_string();
                                            return;
                                        }
                                    };
                                    let id_user = match id_user_str.parse::<u32>() {
                                        Ok(v) => v,
                                        Err(_) => {
                                            *msg.write() = "ID Usuario inválido.".to_string();
                                            return;
                                        }
                                    };

                                    if cant == 0 {
                                        *msg.write() = "La cantidad a vender debe ser mayor a 0.".to_string();
                                        return;
                                    }

                                    let mut tx = match pool.begin().await {
                                        Ok(t) => t,
                                        Err(e) => {
                                            *msg.write() = format!("Error de transacción: {}", e);
                                            return;
                                        }
                                    };

                                    let datos_producto = sqlx::query!(
                                        "SELECT precio, stock FROM producto WHERE idProducto = ? FOR UPDATE",
                                        id_prod
                                    )
                                    .fetch_optional(&mut *tx)
                                    .await;

                                    if let Ok(Some(prod)) = datos_producto {
                                        if prod.stock < cant {
                                            *msg.write() = format!("Stock insuficiente. Disponible actual: {}.", prod.stock);
                                            return;
                                        }

                                        let total_venta = prod.precio * Decimal::from(cant);

                                        if let Err(e) = sqlx::query!(
                                            "UPDATE producto SET stock = stock - ? WHERE idProducto = ?",
                                            cant,
                                            id_prod
                                        )
                                        .execute(&mut *tx)
                                        .await {
                                            *msg.write() = format!("Error al actualizar stock: {}", e);
                                            return;
                                        }

                                        if let Err(e) = sqlx::query!(
                                            "INSERT INTO venta (idUsuario, total) VALUES (?, ?)",
                                            id_user,
                                            total_venta
                                        )
                                        .execute(&mut *tx)
                                        .await {
                                            *msg.write() = format!("Error al registrar el ticket de venta: {}", e);
                                            return;
                                        }

                                        if tx.commit().await.is_ok() {
                                            *msg.write() = format!("¡Venta efectuada con éxito! Total cobrado: ${}", total_venta);

                                            if let Ok(filas) = sqlx::query!("SELECT idProducto, nombreProducto, descripcion, precio, stock FROM producto").fetch_all(&pool).await {
                                                *setter_actualizar.write() = filas.into_iter().map(|f| ProductoData {
                                                    id_producto: f.idProducto,
                                                    nombre_producto: f.nombreProducto,
                                                    descripcion: f.descripcion,
                                                    precio: f.precio,
                                                    stock: f.stock,
                                                }).collect();
                                            }
                                        } else {
                                            *msg.write() = "Error crítico al confirmar la transacción.".to_string();
                                        }
                                    } else {
                                        *msg.write() = "El ID del producto no existe en el sistema.".to_string();
                                    }
                                });
                            })
                            .child(label().text("Efectuar Venta")),
                    ),
            )
            .child(
                rect()
                    .margin(Gaps::new(20.0, 0.0, 0.0, 0.0))
                    .height(Size::fill())
                    .width(Size::fill())
                    .vertical()
                    .child(
                        label()
                            .text("Productos Disponibles en Almacén")
                            .font_size(18.0)
                            .font_weight(FontWeight::BOLD)
                            .margin(Gaps::new(0., 0., 10.0, 0.))
                    )
                    .child(
                        rect()
                            .padding(4.)
                            .height(Size::fill())
                            .width(Size::fill())
                            .child(
                                Table::new()
                                    .column_widths([Size::flex(1.), Size::flex(4.), Size::flex(2.), Size::flex(1.5)])
                                    .child(
                                        TableHead::new().child(
                                            TableRow::new()
                                                .child(TableCell::new().child(label().text("ID")))
                                                .child(TableCell::new().child(label().text("Producto")))
                                                .child(TableCell::new().child(label().text("Precio")))
                                                .child(TableCell::new().child(label().text("Stock")))
                                        )
                                    )
                                    .child(
                                        TableBody::new().child(
                                            ScrollView::new().children(
                                                productos.read().iter().enumerate().map(|(i, prod)| {
                                                    TableRow::new()
                                                        .key(i)
                                                        .child(TableCell::new().child(label().text(prod.id_producto.to_string())))
                                                        .child(TableCell::new().child(label().text(prod.nombre_producto.clone())))
                                                        .child(TableCell::new().child(label().text(format!("${}", prod.precio))))
                                                        .child(TableCell::new().child(label().text(prod.stock.to_string())))
                                                        .into()
                                                })
                                            )
                                        )
                                    )
                            )
                    )
            )
  }
}
