use iced::{Task, Theme};
use std::fmt;

use crate::screen::Screen;

#[derive(Clone)]
pub enum Instruction {
    ChangeScreen(Screen),
    UpdateTheme(Theme),
}

#[derive(Clone)]
pub enum Operator {
    Add,
    Subtract,
    Multiply,
    Divide,
}

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

pub struct Action<Instruction, Message> {
    pub instruction: Option<Instruction>,
    pub task: Task<Message>,
}

impl<Instruction, Message> Action<Instruction, Message> {
    /// Create a new `Action` with no `Instruction` or [`Task`](iced::Task).
    pub fn none() -> Self {
        Self {
            instruction: None,
            task: Task::none(),
        }
    }

    /// Create a new `Action` with an `Instruction` and a [`Task`](iced::Task).
    pub fn new(instruction: Instruction, task: Task<Message>) -> Self {
        Self {
            instruction: Some(instruction),
            task,
        }
    }

    /// Create a new `Action` with an `Instruction` to be handled by some ancestor component.
    pub fn from_instruction(instruction: Instruction) -> Self {
        Self {
            instruction: Some(instruction),
            task: Task::none(),
        }
    }

    /// Create a new `Action` with a [`Task`](iced::Task).
    pub fn from_task(task: Task<Message>) -> Self {
        Self {
            instruction: None,
            task,
        }
    }

    /// Map the message of the `Action`'s [`Task`](iced::Task) to a different type.
    pub fn map<N>(self, f: impl Fn(Message) -> N + Send + 'static) -> Action<Instruction, N>
    where
        Message: Send + 'static,
        N: Send + 'static,
    {
        Action {
            instruction: self.instruction,
            task: self.task.map(f),
        }
    }

    /// Maps the `Instruction` of the `Action` to a different type.
    pub fn map_instruction<N>(
        self,
        f: impl Fn(Instruction) -> N + Send + 'static,
    ) -> Action<N, Message>
    where
        N: Send + 'static,
        Instruction: Send + 'static,
    {
        Action {
            instruction: self.instruction.map(f),
            task: self.task,
        }
    }

    /// Sets the `Instruction` of an `Action`.
    pub fn with_instruction(mut self, instruction: Instruction) -> Self {
        self.instruction = Some(instruction);
        self
    }

    /// Sets the [`Task`](iced::Task) of an `Action`.
    pub fn with_task(mut self, task: Task<Message>) -> Self {
        self.task = task;
        self
    }
}

impl<Instruction: fmt::Debug, Message> fmt::Debug for Action<Instruction, Message> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Action")
            .field("instruction", &self.instruction)
            .finish()
    }
}
