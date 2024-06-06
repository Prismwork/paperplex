mod cfg;
mod platform;

use tao::{
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};
use tray_icon::{
    TrayIconBuilder,
    // Icon,
    menu::{Menu, MenuItem, MenuEvent}
};
use wry::WebViewBuilder;

fn main() {
    let config = cfg::Config::get("paperplex.toml");

    if config.url.is_empty() {
        println!("Please specify a valid url");
        std::process::exit(1);
    }

    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_decorations(false)
        // .with_transparent(true)
        // .with_always_on_bottom(true)
        .with_position(tao::dpi::LogicalPosition::new(0, 0))
        .with_maximized(true)
        .build(&event_loop).unwrap();

    let monitor = window.primary_monitor().unwrap();
    window.set_inner_size(monitor.size());
    window.set_maximized(true);

    let tray_menu = Menu::new();
    let quit_btn = MenuItem::new("Quit", true, None);
    tray_menu.append(&quit_btn).unwrap();

    let mut tray_icon = None;

    let menu_channel = MenuEvent::receiver();

    #[cfg(not(target_os = "linux"))]
    let builder = WebViewBuilder::new(&window);
    #[cfg(target_os = "linux")]
    let builder = WebViewBuilder::new_gtk(window.gtk_window());

    let _webview = builder
        .with_url(config.url)
        .build().unwrap();

    #[cfg(target_os = "windows")]
    platform::setup_window_win(&window);
    
    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::WaitUntil(
            std::time::Instant::now() + std::time::Duration::from_millis(16),
        );

        if let tao::event::Event::NewEvents(tao::event::StartCause::Init) = event {
            tray_icon = Some(
                TrayIconBuilder::new()
                    .with_menu(Box::new(tray_menu.clone()))
                    .with_tooltip("Paperplex")
                    // .with_icon(Icon::from_path("./icon.ico", None).unwrap())
                    .build().unwrap()
            );
        }

        if let Ok(event) = menu_channel.try_recv() {
            if event.id == quit_btn.id() {
                tray_icon.take();
                *control_flow = ControlFlow::Exit;
            }
        }
    });
}
