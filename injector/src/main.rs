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

fn main() {
    tracing_subscriber::registry()
        .with(fmt::layer())
        .with(EnvFilter::from_default_env())
        .init();

    let window_repo = Arc::new(WindowsWindowRepository::new());
    let config_repo = Arc::new(FileConfigurationRepository::new());
    let window_service = Arc::new(WindowService::new(window_repo));

    let state = AppState::new(window_service, config_repo).expect("Failed to initialize app state");

    presentation::start(state);
}
