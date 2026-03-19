use iced::{
    Alignment, Element, Length,
    widget::{button, column, container, grid, text},
};

use crate::{
    action::{Action, Operator},
    app::{Display, InputState},
};

impl Operator {
    pub fn symbol(&self) -> &str {
        match self {
            Operator::Add => "+",
            Operator::Subtract => "-",
            Operator::Multiply => "*",
            Operator::Divide => "/",
        }
    }
}

#[derive(Clone)]
pub enum Message {
    DigitPressed(u8),
    OperatorPressed(Operator),
    DecimalPressed,
    ActionPerformed(Instruction),
    Calculate,
    Clear,
}

pub struct Standard {
    input_display: Display,
    previous_value: f64,
    current_operator: Option<Operator>,
    input_state: InputState,
}

impl Standard {
    pub fn new() -> Self {
        Self {
            input_display: Display("0".to_string()),
            previous_value: 0.0,
            current_operator: None,
            input_state: InputState::Inputting,
        }
    }

    pub fn update(&mut self, message: Message) -> Action<Instruction, Message> {
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
                };
                Action::none()
            }
            Message::OperatorPressed(op) => {
                if self.current_operator.is_some() && self.input_state == InputState::Inputting {
                    self.calculate_result()
                }
                self.previous_value = self.input_display.parse().unwrap_or_default();
                self.current_operator = Some(op);
                self.input_state = InputState::Complete;
                Action::none()
            }
            Message::ActionPerformed(instruction) => Action::from_instruction(instruction),
            Message::Calculate => {
                self.calculate_result();
                self.current_operator = None;
                self.input_state = InputState::Complete;
                Action::none()
            }
            Message::Clear => {
                self.input_display = Display("0".to_string());
                self.previous_value = 0.0;
                self.current_operator = None;
                self.input_state = InputState::Inputting;
                Action::none()
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
                Action::none()
            }
        }
    }

    pub fn view(&self) -> Element<'_, Message> {
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

    fn calculate_result(&mut self) {
        if let Some(op) = &self.current_operator {
            let current_val: f64 = self.input_display.parse().unwrap_or_default();

            let result = match op {
                Operator::Add => self.previous_value + current_val,
                Operator::Subtract => self.previous_value - current_val,
                Operator::Multiply => self.previous_value * current_val,
                Operator::Divide => {
                    if current_val == 0.0 {
                        0.0
                    } else {
                        self.previous_value / current_val
                    }
                }
            };

            self.input_display = Display(result.to_string());
            self.previous_value = result;
        }
    }
}

fn calc_button(label: &str, message: Message) -> Element<'_, Message> {
    button(text(label).center().size(20))
        .on_press(message)
        .padding(20)
        .width(Length::Fill)
        .into()
}
