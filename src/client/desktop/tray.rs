// adapted from https://github.com/fokklz/dioxus-tray-starter/blob/main/src/extensions/tray.rs
use std::path::Path;

use dioxus::LaunchBuilder;
use dioxus::desktop::{
    Config, WindowBuilder, WindowCloseBehaviour, WindowEvent,
    tao::event::Event,
    trayicon::{
        TrayIconBuilder,
        menu::{Menu, MenuItem},
    },
    use_wry_event_handler, window,
};

use dioxus::desktop::use_muda_event_handler;

#[cfg(target_os = "windows")]
use dioxus::desktop::tao::platform::windows::WindowBuilderExtWindows;

use dioxus::prelude::*;

pub fn launch(app: fn() -> Element) {
    LaunchBuilder::new()
        .with_cfg(desktop! {
            Config::new()
                .with_window(build_window())
                .with_close_behaviour(WindowCloseBehaviour::WindowHides)
        })
        .launch(app)
}

fn build_window() -> WindowBuilder {
    let icon = load_icon(dioxus::desktop::tao::window::Icon::from_rgba).ok();
    let builder = WindowBuilder::new()
        .with_title("Busy Monkey")
        .with_window_icon(icon.clone())
        .with_visible(true);
    #[cfg(target_os = "windows")]
    {
        builder.with_taskbar_icon(icon)
    }
    #[cfg(not(target_os = "windows"))]
    {
        builder
    }
}

struct TrayMenu {
    menu: Menu,
    toggle: MenuItem,
}

impl TrayMenu {
    fn new() -> Self {
        let menu = Menu::new();
        let toggle = MenuItem::with_id("toggle", "Close", true, None);
        let quit = MenuItem::with_id("quit", "Quit", true, None);
        menu.append_items(&[&toggle, &quit]).unwrap();
        Self { menu, toggle }
    }

    fn boxed(self) -> Box<Menu> {
        Box::new(self.menu)
    }
}

impl Default for TrayMenu {
    fn default() -> Self {
        Self::new()
    }
}

pub fn init() -> anyhow::Result<()> {
    let menu = TrayMenu::new();

    bind_tray_menu_handler(menu.toggle.clone());

    bind_window_closed_handler(menu.toggle.clone());
    bind_context(menu)?;
    Ok(())
}

fn bind_context(menu: TrayMenu) -> anyhow::Result<()> {
    provide_context(
        TrayIconBuilder::new()
            .with_menu(menu.boxed())
            .with_menu_on_left_click(false)
            .with_icon(load_icon(dioxus::desktop::trayicon::Icon::from_rgba)?)
            .build()?,
    );
    Ok(())
}

fn bind_tray_menu_handler(toggle: MenuItem) {
    fn toggle_tray(toggle: MenuItem) {
        let visible = window().window.is_visible();
        toggle.set_text(match visible {
            true => "Open",
            false => "Close",
        });
        window().window.set_visible(!visible);
    }

    use_muda_event_handler(move |event| {
        info!("{event:?}");
        match event.id.0.as_str() {
            "quit" => std::process::exit(0),
            "toggle" => toggle_tray(toggle.clone()),
            _ => {}
        }
    });
}

fn bind_window_closed_handler(toggle: MenuItem) {
    use_wry_event_handler(move |event, _| window_closed(toggle.clone(), event));
}

fn window_closed<E>(toggle: MenuItem, event: &Event<E>) {
    if let Event::WindowEvent {
        window_id: _,
        event,
        ..
    } = event
        && event == &WindowEvent::CloseRequested
    {
        toggle.set_text("Open");
    }
}

fn load_icon<I, E, F>(factory: F) -> anyhow::Result<I>
where
    F: FnOnce(Vec<u8>, u32, u32) -> Result<I, E>,
    E: std::error::Error + Send + Sync + 'static,
{
    let path = Path::new(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/favicon.ico"));
    let image = image::open(path)?.into_rgba8();
    let (width, height) = image.dimensions();
    Ok(factory(image.into_raw(), width, height)?)
}
