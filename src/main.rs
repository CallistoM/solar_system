use iced::{Application, Settings};

pub mod canvas_state;
pub mod solar_system;
pub mod time;

pub fn main() {
    solar_system::SolarSystem::run(Settings {
        antialiasing: true,
        window: iced::window::Settings {
            decorations: true,
            size: (1024, 840),
            ..iced::window::Settings::default()
        },
        ..Settings::default()
    })
}
