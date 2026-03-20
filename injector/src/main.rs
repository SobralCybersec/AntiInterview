#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod application;
mod domain;
mod infrastructure;
mod presentation;

use application::WindowService;
use infrastructure::{FileConfigurationRepository, WindowsWindowRepository};
use presentation::AppState;
use std::sync::Arc;
use tracing_subscriber::{fmt, prelude::*, EnvFilter};
use std::fs::OpenOptions;

fn main() {
    let log_file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("anti-interview.log")
        .expect("Failed to create log file");

    let env_filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new("info"));

    if cfg!(debug_assertions) {
        tracing_subscriber::registry()
            .with(
                fmt::layer()
                    .with_writer(std::io::stdout)
                    .with_ansi(true)
            )
            .with(
                fmt::layer()
                    .with_writer(Arc::new(log_file))
                    .with_ansi(false)
            )
            .with(env_filter)
            .init();
    } else {
        tracing_subscriber::registry()
            .with(
                fmt::layer()
                    .with_writer(Arc::new(log_file))
                    .with_ansi(false)
            )
            .with(env_filter)
            .init();
    }

    tracing::info!("Anti-Interview Started");

    let window_repo = Arc::new(WindowsWindowRepository::new());
    let config_repo = Arc::new(FileConfigurationRepository::new());
    let window_service = Arc::new(WindowService::new(window_repo));

    let state = AppState::new(window_service, config_repo).expect("Failed to initialize app state");

    presentation::start(state);
}
