#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]
#![allow(unused_variables, unused_mut, dead_code, unused_imports)]

pub(crate) mod hexagonal;
pub(crate) mod infraestructura;
pub mod utiles;
mod vista;

use anyhow::Result;

use freya::prelude::*;

use crate::vista::app;

#[tokio::main]
async fn main() -> Result<()> {
    println!("App iniciada");

    launch(
        LaunchConfig::new().with_window(
            WindowConfig::new(move || app(use_state(|| false)))
                .with_size(900., 600.)
                .with_title("Gimnasio")
                .with_min_size(400., 300.),
        ),
    );

    Ok(())
}
