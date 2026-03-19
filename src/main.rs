use iced::Size;

use crate::app::App;
mod action;
mod app;
mod standard_calc;

fn main() -> iced::Result {
    iced::application(App::new, App::update, App::view)
        .theme(App::theme)
        .window_size(Size::new(400.0, 800.0))
        .run()
}
