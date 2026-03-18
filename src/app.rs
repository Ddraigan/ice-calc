use iced::{
    Alignment, Element, Length, Task, Theme,
    widget::{button, column, container, grid, text},
};

use crate::{
    action::{Action, Instruction, Message, Operator},
    screen::Screen,
};

pub struct App {
    screen: Screen,
    theme: Theme,
    input_display: Display,
    previous_value: f64,
    current_operator: Option<Operator>,
    waiting_for_input: bool,
}

impl App {
    pub fn new() -> Self {
        Self {
            screen: Screen::Standard,
            theme: Theme::CatppuccinMocha,
            input_display: Display("0".to_string()),
            previous_value: f64::default(),
            current_operator: None,
            waiting_for_input: false,
        }
    }

    pub fn theme(&self) -> Theme {
        self.theme.clone()
    }

    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::DigitPressed(digit) => {
                if self.waiting_for_input || self.input_display == "0" {
                    self.input_display = Display(digit.to_string());
                    self.waiting_for_input = false;
                } else {
                    self.input_display.push_str(Display(&digit.to_string()));
                };
                Task::none()
            }
            Message::OperatorPressed(op) => {
                self.previous_value = self.input_display.parse().unwrap_or_default();
                self.current_operator = Some(op);
                self.waiting_for_input = true;
                Task::none()
            }
            Message::ActionPerformed(instruction) => {
                let action = self.handle_instruction(instruction);
                action.task
            }
            Message::Calculate => todo!(),
            Message::Clear => todo!(),
        }
    }

    fn handle_instruction(&mut self, instruction: Instruction) -> Action<Instruction, Message> {
        match instruction {
            Instruction::ChangeScreen(requested_screen) => {
                self.screen = requested_screen;
                Action::none()
            }
            Instruction::UpdateTheme(requested_theme) => {
                self.theme = requested_theme;
                Action::none()
            }
        }
    }

    pub fn view(&self) -> Element<'_, Message> {
        let content = match self.screen {
            Screen::Standard => self.standard_view(),
            Screen::History => todo!(),
            Screen::Settings => todo!(),
            Screen::Scientific => todo!(),
        };

        container(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center(Length::Fill)
            .into()
    }

    fn standard_view(&self) -> Element<'_, Message> {
        column![
            container(text(&self.input_display).size(50))
                .width(Length::Fill)
                .align_x(Alignment::End)
                .padding(10),
            container(
                grid!(
                    calc_button("%", '%'),
                    calc_button("CE", '0'),
                    calc_button("C", 'C'),
                    calc_button("BSP", '0'),
                    calc_button("H", '0'),
                    calc_button("H", '0'),
                    calc_button("H", '0'),
                    calc_button("÷", '/'),
                    calc_button("7", '7'),
                    calc_button("8", '8'),
                    calc_button("9", '9'),
                    calc_button("×", '*'),
                    calc_button("4", '4'),
                    calc_button("5", '5'),
                    calc_button("6", '6'),
                    calc_button("−", '-'),
                    calc_button("1", '1'),
                    calc_button("2", '2'),
                    calc_button("3", '3'),
                    calc_button("+", '+'),
                    calc_button("+/-", '0'),
                    calc_button("0", '0'),
                    calc_button(".", '.'),
                    calc_button("=", '='),
                )
                .columns(4)
                .spacing(10),
            ),
        ]
        .spacing(20)
        .max_width(400)
        .padding(20)
        .into()
    }
}

fn calc_button(label: &str, msg: char) -> Element<'_, Message> {
    button(text(label).center().size(20))
        .on_press(Message::DigitPressed(msg))
        .padding(20)
        .width(Length::Fill)
        .into()
}

#[derive(PartialEq)]
struct Display(String);

impl std::ops::DerefMut for Display {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl std::ops::Deref for Display {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
