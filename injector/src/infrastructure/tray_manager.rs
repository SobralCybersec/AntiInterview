use anyhow::Result;
use image::ImageReader;
use std::io::Cursor;
use tray_icon::{
    menu::{Menu, MenuEvent, MenuItem},
    Icon, TrayIcon, TrayIconBuilder,
};

pub struct TrayManager {
    _tray_icon: TrayIcon,
    show_item: MenuItem,
    quit_item: MenuItem,
}

impl TrayManager {
    pub fn new() -> Result<Self> {
        let icon_bytes = include_bytes!("../../../assets/icons/invicon.ico");
        let img = ImageReader::with_format(Cursor::new(icon_bytes), image::ImageFormat::Ico)
            .decode()?;
        
        let rgba = img.to_rgba8();
        let width = img.width();
        let height = img.height();
        
        let icon = Icon::from_rgba(rgba.into_raw(), width, height)?;

        let tray_menu = Menu::new();
        let show_item = MenuItem::new("Show", true, None);
        let quit_item = MenuItem::new("Quit", true, None);

        tray_menu.append(&show_item)?;
        tray_menu.append(&quit_item)?;

        let tray_icon = TrayIconBuilder::new()
            .with_menu(Box::new(tray_menu))
            .with_tooltip("Anti-Interview")
            .with_icon(icon)
            .build()?;

        Ok(Self {
            _tray_icon: tray_icon,
            show_item,
            quit_item,
        })
    }

    pub fn check_events(&self) -> Option<TrayEvent> {
        if let Ok(event) = MenuEvent::receiver().try_recv() {
            if event.id == self.show_item.id() {
                return Some(TrayEvent::Show);
            } else if event.id == self.quit_item.id() {
                return Some(TrayEvent::Quit);
            }
        }
        None
    }
}

#[derive(Debug, Clone, Copy)]
pub enum TrayEvent {
    Show,
    Quit,
}
