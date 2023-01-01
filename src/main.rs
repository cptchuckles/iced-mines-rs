use iced::{
    widget::{container, text},
    Length, Sandbox, Settings,
};

fn main() -> Result<(), iced::Error> {
    Minefield::run(Settings::default())
}

struct Minefield {}

impl Sandbox for Minefield {
    type Message = ();

    fn new() -> Self {
        Self {}
    }

    fn title(&self) -> String {
        "Iced Mines".into()
    }

    fn update(&mut self, _message: Self::Message) {
        ()
    }

    fn view(&self) -> iced::Element<Self::Message> {
        container(text("Hello World").size(50))
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .into()
    }
}
