use std::rc::Rc;

use derivative::Derivative;
use iced::alignment::Horizontal;
use iced::{
    button, text_input, tooltip, Alignment, Button, Color, Column, Container, Length, Row, Text,
    Tooltip,
};

use bl3_save_edit_core::bl3_save::sdu::SaveSduSlot;

use crate::bl3_ui::{Bl3Message, InteractionMessage};
use crate::bl3_ui_style::{Bl3UiStyle, Bl3UiTooltipStyle};
use crate::resources::fonts::{ST_HEI_TI_LIGHT};
use crate::views::manage_save::character::{CharacterSduMessage, SaveCharacterInteractionMessage};
use crate::views::manage_save::ManageSaveInteractionMessage;
use crate::views::InteractionExt;
use crate::widgets::number_input::NumberInput;
use crate::widgets::text_margin::TextMargin;

#[derive(Derivative)]
#[derivative(Debug, Default)]
pub struct SduUnlockField {
    name: String,
    view_name: String,
    text_margin: usize,
    pub sdu_slot: SaveSduSlot,
    pub input: i32,
    input_state: text_input::State,
    #[derivative(
    Debug = "ignore",
    Default(value = "Rc::new(CharacterSduMessage::Backpack)")
    )]
    on_changed: Rc<dyn Fn(i32) -> CharacterSduMessage>,
}

impl SduUnlockField {
    pub fn new<F>(text_margin: usize, sdu_slot: SaveSduSlot, on_changed: F, vn: String) -> Self
        where
            F: 'static + Fn(i32) -> CharacterSduMessage,
    {
        SduUnlockField {
            view_name: vn,
            name: sdu_slot.to_string(),
            text_margin,
            sdu_slot,
            on_changed: Rc::new(on_changed),
            ..Default::default()
        }
    }

    pub fn view(&mut self) -> Row<Bl3Message> {
        let on_changed = self.on_changed.clone();
        let minimum = 0;
        let maximum = self.sdu_slot.maximum();

        Row::new()
            .push(
                TextMargin::new(&self.view_name, self.text_margin)
                    .0
                    .font(ST_HEI_TI_LIGHT)
                    .size(17)
                    .color(Color::from_rgb8(220, 220, 220))
                    .width(Length::FillPortion(8)),
            )
            .push(
                Tooltip::new(
                    NumberInput::new(
                        &mut self.input_state,
                        self.input,
                        minimum,
                        Some(maximum),
                        move |v| {
                            InteractionMessage::ManageSaveInteraction(
                                ManageSaveInteractionMessage::Character(
                                    SaveCharacterInteractionMessage::SduMessage(on_changed(v)),
                                ),
                            )
                        },
                    )
                        .0
                        .width(Length::FillPortion(3))
                        .font(ST_HEI_TI_LIGHT)
                        .padding(10)
                        .size(17)
                        .style(Bl3UiStyle)
                        .into_element(),
                    format!("等级必须在 {} 和 {} 之间", minimum, maximum),
                    tooltip::Position::Top,
                )
                    .gap(10)
                    .padding(10)
                    .font(ST_HEI_TI_LIGHT)
                    .size(17)
                    .style(Bl3UiTooltipStyle),
            )
            .width(Length::Fill)
            .align_items(Alignment::Center)
    }
}

#[derive(Debug)]
pub struct SduUnlocker {
    pub backpack: SduUnlockField,
    pub sniper: SduUnlockField,
    pub heavy: SduUnlockField,
    pub shotgun: SduUnlockField,
    pub grenade: SduUnlockField,
    pub smg: SduUnlockField,
    pub assault_rifle: SduUnlockField,
    pub pistol: SduUnlockField,
    unlock_all_button_state: button::State,
}

impl std::default::Default for SduUnlocker {
    fn default() -> Self {
        Self {
            backpack: SduUnlockField::new(0, SaveSduSlot::Backpack, CharacterSduMessage::Backpack, String::from("背包")),
            sniper: SduUnlockField::new(4, SaveSduSlot::Sniper, CharacterSduMessage::Sniper, String::from("狙击枪")),
            heavy: SduUnlockField::new(0, SaveSduSlot::Heavy, CharacterSduMessage::Heavy, String::from("重型武器")),
            shotgun: SduUnlockField::new(4, SaveSduSlot::Shotgun, CharacterSduMessage::Shotgun, String::from("霰弹枪")),
            grenade: SduUnlockField::new(0, SaveSduSlot::Grenade, CharacterSduMessage::Grenade, String::from("手榴弹")),
            smg: SduUnlockField::new(4, SaveSduSlot::Smg, CharacterSduMessage::Smg, String::from("冲锋枪")),
            assault_rifle: SduUnlockField::new(
                0,
                SaveSduSlot::Ar,
                CharacterSduMessage::AssaultRifle,
                String::from("步枪"),
            ),
            pistol: SduUnlockField::new(4, SaveSduSlot::Pistol, CharacterSduMessage::Pistol, String::from("手枪")),
            unlock_all_button_state: button::State::default(),
        }
    }
}

impl SduUnlocker {
    pub fn view(&mut self) -> Container<Bl3Message> {
        Container::new(
            Column::new()
                .push(
                    Container::new(
                        Text::new("弹药升级管理")
                            .font(ST_HEI_TI_LIGHT)
                            .size(17)
                            .color(Color::from_rgb8(242, 203, 5)),
                    )
                        .padding(10)
                        .align_x(Horizontal::Center)
                        .width(Length::Fill)
                        .style(Bl3UiStyle),
                )
                .push(
                    Container::new(
                        Column::new()
                            .push(
                                Row::new()
                                    .push(self.backpack.view())
                                    .push(self.sniper.view()),
                            )
                            .push(Row::new().push(self.heavy.view()).push(self.shotgun.view()))
                            .push(Row::new().push(self.grenade.view()).push(self.smg.view()))
                            .push(
                                Row::new()
                                    .push(self.assault_rifle.view())
                                    .push(self.pistol.view()),
                            )
                            .push(
                                Container::new(
                                    Button::new(
                                        &mut self.unlock_all_button_state,
                                        Text::new("将所有升级置为最高")
                                            .font(ST_HEI_TI_LIGHT)
                                            .size(17),
                                    )
                                        .on_press(InteractionMessage::ManageSaveInteraction(
                                            ManageSaveInteractionMessage::Character(
                                                SaveCharacterInteractionMessage::MaxSduSlotsPressed,
                                            ),
                                        ))
                                        .padding(10)
                                        .style(Bl3UiStyle)
                                        .into_element(),
                                )
                                    .padding(5),
                            )
                            .align_items(Alignment::Center)
                            .spacing(15),
                    )
                        .padding(20)
                        .style(Bl3UiStyle),
                ),
        )
    }
}
