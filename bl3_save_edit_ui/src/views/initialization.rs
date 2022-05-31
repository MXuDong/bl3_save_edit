use iced::alignment::{Horizontal, Vertical};
use iced::{Color, Container, Length, Text};

use crate::bl3_ui::Bl3Message;
use crate::resources::fonts::{ST_HEI_TI_LIGHT};

#[derive(Debug, Clone)]
pub enum InitializationMessage {
    LoadSaves,
}

pub fn view<'a>() -> Container<'a, Bl3Message> {
    let initializing_text = Text::new("初始化中...")
        .font(ST_HEI_TI_LIGHT)
        .size(20)
        .color(Color::from_rgb8(220, 220, 220));

    Container::new(initializing_text)
        .width(Length::Fill)
        .height(Length::Fill)
        .align_x(Horizontal::Center)
        .align_y(Vertical::Center)
}
