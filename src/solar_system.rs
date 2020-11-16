use iced::{
    canvas, executor, Application, Canvas, Command, Container, Element, Length, Subscription,
};

use super::canvas_state;
use super::time;
use std::time::Instant;

#[derive(Debug, Clone, Copy)]
pub enum Message {
    Tick(Instant),
}

pub struct SolarSystem {
    state: canvas_state::State,
    solar_system: canvas::layer::Cache<canvas_state::State>,
}

impl Application for SolarSystem {
    type Executor = executor::Default;
    type Message = Message;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<Message>) {
        (
            SolarSystem {
                state: canvas_state::State::new(),
                solar_system: Default::default(),
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("Solar system")
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::Tick(instant) => {
                self.state.update(instant);
                self.solar_system.clear();
            }
        }

        Command::none()
    }

    fn subscription(&self) -> Subscription<Message> {
        time::time::every(std::time::Duration::from_millis(10))
            .map(|instant| Message::Tick(instant))
    }

    fn view(&mut self) -> Element<Message> {
        let canvas = Canvas::new()
            .width(Length::Fill)
            .height(Length::Fill)
            .push(self.solar_system.with(&self.state));

        Container::new(canvas)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .into()
    }
}
