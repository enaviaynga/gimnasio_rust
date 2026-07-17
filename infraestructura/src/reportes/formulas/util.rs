use rust_decimal::Decimal;
use rust_decimal::prelude::FromPrimitive;
use sqlx::MySqlPool;
use std::error::Error;

pub fn dec(n: u32) -> Decimal {
  Decimal::from(n)
}

pub fn division_segura(numerador: Decimal, denominador: Decimal, precision: u32) -> Decimal {
  if denominador.is_zero() {
    Decimal::ZERO
  } else {
    (numerador / denominador).round_dp(precision)
  }
}
