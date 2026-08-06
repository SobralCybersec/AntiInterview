// Standalone test for WRY browser
// Run with: cargo run --bin test_wry_browser

use tao::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};
use wry::WebViewBuilder;

fn main() -> wry::Result<()> {
    println!("=== WRY Browser Test ===");
    println!("Step 1: Creating event loop...");

    let event_loop = EventLoop::new();
    println!("✓ Event loop created");

    println!("Step 2: Creating window...");
    let window = WindowBuilder::new()
        .with_title("WRY Test Browser")
        .with_inner_size(tao::dpi::LogicalSize::new(800, 600))
        .build(&event_loop)
        .expect("Failed to create window");
    println!("✓ Window created");

    println!("Step 3: Creating WebView...");
    let url = "https://example.com";
    println!("Loading URL: {}", url);

    let _webview = WebViewBuilder::new().with_url(url).build(&window)?;
    println!("✓ WebView created");

    println!("Step 4: Starting event loop...");
    println!("Window should now be visible!");
    println!("Close the window to exit.");

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => {
                println!("Window close requested");
                *control_flow = ControlFlow::Exit;
            }
            Event::WindowEvent {
                event: WindowEvent::Focused(focused),
                ..
            } => {
                println!("Window focus changed: {}", focused);
            }
            _ => {}
        }
    });
}
