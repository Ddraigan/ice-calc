use crate::app::App;
mod action;
mod app;
mod screen;
mod standard_calc;

fn main() -> iced::Result {
    iced::application(App::new, App::update, App::view)
        .theme(App::theme)
        .run()
}
