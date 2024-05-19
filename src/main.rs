mod cfg;

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

#[cfg(target_os = "windows")]
use tao::platform::windows::WindowExtWindows;
#[cfg(target_os = "windows")]
use windows::{
    Win32::Foundation::HWND,
    Win32::UI::WindowsAndMessaging::{SetWindowLongW, SetWindowPos, GWL_EXSTYLE, HWND_BOTTOM, SWP_NOMOVE, SWP_NOSIZE, SWP_NOACTIVATE},
    Win32::UI::WindowsAndMessaging::{WS_EX_LAYERED, WS_EX_NOACTIVATE, WS_EX_TOOLWINDOW, WS_EX_TRANSPARENT},
};

fn main() {
    let config = cfg::Config::get("paperplex.toml");

    if config.url.is_empty() {
        println!("Please specify a valid url");
        std::process::exit(1);
    }

    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_decorations(false)
        .with_transparent(true)
        .with_always_on_bottom(true)
        .with_position(tao::dpi::LogicalPosition::new(0, 0))
        .with_maximized(true)
        .build(&event_loop).unwrap();

    let monitor = window.primary_monitor().unwrap();
    window.set_inner_size(monitor.size());
    window.set_maximized(true);

    #[cfg(target_os = "windows")]
    {
        let hwnd = HWND(window.hwnd() as isize);

        unsafe {
            let ex_style = WS_EX_LAYERED | WS_EX_TRANSPARENT | WS_EX_TOOLWINDOW | WS_EX_NOACTIVATE;
            SetWindowLongW(hwnd, GWL_EXSTYLE, ex_style.0 as i32);
            SetWindowPos(
                hwnd,
                HWND_BOTTOM,
                0,
                0,
                0,
                0,
                SWP_NOMOVE | SWP_NOSIZE | SWP_NOACTIVATE,
            ).unwrap();
        }
    }

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
