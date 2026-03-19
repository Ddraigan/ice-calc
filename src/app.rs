use iced::{Element, Length, Task, Theme, widget::container};

use crate::standard_calc::{Message, Standard};

#[derive(Eq, PartialEq)]
pub enum InputState {
    Inputting,
    Complete,
}

#[derive(Clone)]
pub enum Instruction {
    ChangeScreen(Screen),
    UpdateTheme(Theme),
}

#[derive(Clone)]
pub enum Screen {
    Standard,
    History,
    Settings,
    Scientific,
}

pub struct App {
    active_screen: Screen,
    theme: Theme,
    standard_state: Standard,
}

impl App {
    pub fn new() -> Self {
        Self {
            active_screen: Screen::Standard,
            theme: Theme::CatppuccinMocha,
            standard_state: Standard::new(),
        }
    }

    pub fn theme(&self) -> Theme {
        self.theme.clone()
    }

    pub fn update(&mut self, message: Message) -> Task<Message> {
        let action = match self.active_screen {
            Screen::Standard => self.standard_state.update(message),
            Screen::History => todo!(),
            Screen::Settings => todo!(),
            Screen::Scientific => todo!(),
        };

        let instruction_task = if let Some(instruction) = action.instruction {
            self.handle_instruction(instruction)
        } else {
            Task::none()
        };

        instruction_task.chain(action.task)
    }

    fn handle_instruction(&mut self, instruction: Instruction) -> Task<Message> {
        match instruction {
            Instruction::ChangeScreen(new_screen) => {
                self.active_screen = new_screen;
                Task::none()
            }
            Instruction::UpdateTheme(new_theme) => {
                self.theme = new_theme;
                Task::none()
            }
        }
    }

    pub fn view(&self) -> Element<'_, Message> {
        let content = match self.active_screen {
            Screen::Standard => self.standard_state.view(),
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
}

#[derive(PartialEq)]
pub struct Display(pub String);

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
