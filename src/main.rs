use iced::widget::{button, center, column, grid, text};
use iced::{Element, Task};
mod action;

fn main() -> iced::Result {
    iced::run(Calculator::update, Calculator::view)
}

#[derive(Clone, Copy)]
enum Message {
    Input(char),
    Clear,
}

#[derive(Debug, Default)]
struct Calculator {
    display: String,
}

impl Calculator {
    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Input(c) => self.display.push(c),
            Message::Clear => self.display.clear(),
        }
        Task::none()
    }

    fn view(&self) -> Element<'_, Message> {
        let content = column![
            text(&self.display).size(50),
            grid!(
                button("7").on_press(Message::Input('7')),
                button("8").on_press(Message::Input('8')),
                button("9").on_press(Message::Input('9')),
                button("4").on_press(Message::Input('4')),
                button("5").on_press(Message::Input('5')),
                button("6").on_press(Message::Input('6')),
                button("1").on_press(Message::Input('1')),
                button("2").on_press(Message::Input('2')),
                button("3").on_press(Message::Input('3')),
            )
            .columns(3)
            .spacing(10),
            button("Clear").on_press(Message::Clear),
        ]
        .spacing(20)
        .padding(20);

        center(content).into()
    }
}
