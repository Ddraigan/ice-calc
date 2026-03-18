use std::fmt::format;

use iced::{
    Alignment, Element, Length, Task, Theme,
    widget::{button, column, container, grid, text},
};

use crate::{
    action::{Action, Instruction, Message, Operator},
    screen::Screen,
};

#[derive(Eq, PartialEq)]
enum InputState {
    Inputting,
    Complete,
}

pub struct App {
    screen: Screen,
    theme: Theme,
    input_display: Display,
    previous_value: f64,
    current_operator: Option<Operator>,
    input_state: InputState,
}

impl App {
    pub fn new() -> Self {
        Self {
            screen: Screen::Standard,
            theme: Theme::CatppuccinMocha,
            input_display: Display("0".to_string()),
            previous_value: f64::default(),
            current_operator: None,
            input_state: InputState::Complete,
        }
    }

    pub fn theme(&self) -> Theme {
        self.theme.clone()
    }

    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::DigitPressed(digit) => {
                match self.input_state {
                    InputState::Inputting => {
                        if *self.input_display == "0" {
                            self.input_display = Display(digit.to_string());
                        } else {
                            self.input_display.push_str(&digit.to_string());
                        }
                    }
                    InputState::Complete => {
                        self.input_display = Display(digit.to_string());
                        self.input_state = InputState::Inputting;
                    }
                }
                Task::none()
            }
            Message::OperatorPressed(op) => {
                if self.current_operator.is_some() && self.input_state == InputState::Inputting {
                    self.calculate_result()
                }
                self.previous_value = self.input_display.parse().unwrap_or_default();
                self.current_operator = Some(op);
                self.input_state = InputState::Complete;
                Task::none()
            }
            Message::ActionPerformed(instruction) => {
                let action = self.handle_instruction(instruction);
                action.task
            }
            Message::Calculate => {
                self.calculate_result();
                self.current_operator = None;
                self.input_state = InputState::Complete;
                Task::none()
            }
            Message::Clear => {
                self.input_display = Display("0".to_string());
                self.previous_value = 0.0;
                self.current_operator = None;
                self.input_state = InputState::Inputting;
                Task::none()
            }
            Message::DecimalPressed => {
                match self.input_state {
                    InputState::Inputting => {
                        if !self.input_display.contains('.') {
                            self.input_display.push('.')
                        }
                    }
                    InputState::Complete => {
                        self.input_display = Display("0.".to_string());
                        self.input_state = InputState::Inputting;
                    }
                }
                Task::none()
            }
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

    fn calculate_result(&mut self) {
        if let Some(op) = &self.current_operator {
            let current_val: f64 = self.input_display.parse().unwrap_or_default();

            let result = match op {
                Operator::Add => self.previous_value + current_val,
                Operator::Subtract => self.previous_value - current_val,
                Operator::Multiply => self.previous_value * current_val,
                Operator::Divide => self.previous_value / current_val,
            };

            self.input_display = Display(result.to_string());
            self.previous_value = result;
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
        let display_text = if let Some(op) = &self.current_operator {
            if self.input_state == InputState::Complete {
                format!("{} {}", self.previous_value, op.symbol())
            } else {
                format!(
                    "{} {} {}",
                    self.previous_value,
                    op.symbol(),
                    *self.input_display,
                )
            }
        } else {
            self.input_display.to_string()
        };

        column![
            container(text(display_text).size(50))
                .width(Length::Fill)
                .align_x(Alignment::End)
                .padding(10),
            container(
                grid!(
                    calc_button("%", Message::OperatorPressed(Operator::Divide)),
                    calc_button("CE", Message::Clear),
                    calc_button("C", Message::Clear),
                    calc_button("BSP", Message::OperatorPressed(Operator::Divide)),
                    calc_button("H", Message::OperatorPressed(Operator::Divide)),
                    calc_button("H", Message::OperatorPressed(Operator::Divide)),
                    calc_button("H", Message::OperatorPressed(Operator::Divide)),
                    calc_button("÷", Message::OperatorPressed(Operator::Divide)),
                    calc_button("7", Message::DigitPressed(7)),
                    calc_button("8", Message::DigitPressed(8)),
                    calc_button("9", Message::DigitPressed(9)),
                    calc_button("×", Message::OperatorPressed(Operator::Multiply)),
                    calc_button("4", Message::DigitPressed(4)),
                    calc_button("5", Message::DigitPressed(5)),
                    calc_button("6", Message::DigitPressed(6)),
                    calc_button("−", Message::OperatorPressed(Operator::Subtract)),
                    calc_button("1", Message::DigitPressed(1)),
                    calc_button("2", Message::DigitPressed(2)),
                    calc_button("3", Message::DigitPressed(3)),
                    calc_button("+", Message::OperatorPressed(Operator::Add)),
                    calc_button("+/-", Message::OperatorPressed(Operator::Subtract)),
                    calc_button("0", Message::DigitPressed(0)),
                    calc_button(".", Message::DecimalPressed),
                    calc_button("=", Message::Calculate),
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

fn calc_button(label: &str, message: Message) -> Element<'_, Message> {
    button(text(label).center().size(20))
        .on_press(message)
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
