#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::env;
use tao::{
    event::{Event, WindowEvent},
    event_loop::ControlFlow,
    window::WindowBuilder,
};
use wry::WebViewBuilder;

#[cfg(not(target_os = "windows"))]
use tao::event_loop::EventLoop;
#[cfg(target_os = "windows")]
use tao::event_loop::EventLoopBuilder;
#[cfg(target_os = "windows")]
use tao::platform::windows::EventLoopBuilderExtWindows;

fn main() {
    let args: Vec<String> = env::args().collect();

    let url = if args.len() > 1 {
        args[1].clone()
    } else {
        "https://chatgpt.com".to_string()
    };

    let title = if args.len() > 2 {
        args[2].clone()
    } else {
        "AI Browser - Anti-Interview".to_string()
    };

    #[cfg(target_os = "windows")]
    let event_loop = EventLoopBuilder::new().with_any_thread(true).build();

    #[cfg(not(target_os = "windows"))]
    let event_loop = EventLoop::new();

    let window = WindowBuilder::new()
        .with_title(&title)
        .with_inner_size(tao::dpi::LogicalSize::new(1200, 800))
        .with_decorations(true)
        .build(&event_loop)
        .expect("Failed to create window");

    let _webview = WebViewBuilder::new()
        .with_url(&url)
        .build(&window)
        .expect("Failed to create webview");

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        if let Event::WindowEvent {
            event: WindowEvent::CloseRequested,
            ..
        } = event
        {
            *control_flow = ControlFlow::Exit;
        }
    });
}
