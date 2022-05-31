use iced::{button, text_input, Alignment, Button, Column, Container, Length, Row, Text};

use crate::bl3_ui::{Bl3Message, InteractionMessage};
use crate::bl3_ui_style::Bl3UiStyle;
use crate::resources::fonts::{ST_HEI_TI_LIGHT};
use crate::views::manage_save::ManageSaveInteractionMessage;
use crate::views::InteractionExt;
use crate::widgets::labelled_element::LabelledElement;
use crate::widgets::number_input::NumberInput;

#[derive(Debug, Default)]
pub struct CurrencyState {
    pub money_input: i32,
    pub money_input_state: text_input::State,
    pub eridium_input: i32,
    pub eridium_input_state: text_input::State,
    pub max_eridium_button_state: button::State,
    pub max_money_button_state: button::State,
}

#[derive(Debug, Clone)]
pub enum SaveCurrencyInteractionMessage {
    Money(i32),
    Eridium(i32),
    MaxMoneyPressed,
    MaxEridiumPressed,
}

pub fn view(currency_state: &mut CurrencyState) -> Container<Bl3Message> {
    let money = Container::new(
        Row::new()
            .push(
                LabelledElement::create(
                    "金钱",
                    Length::Units(75),
                    NumberInput::new(
                        &mut currency_state.money_input_state,
                        currency_state.money_input,
                        0,
                        None,
                        |v| {
                            InteractionMessage::ManageSaveInteraction(
                                ManageSaveInteractionMessage::Currency(
                                    SaveCurrencyInteractionMessage::Money(v),
                                ),
                            )
                        },
                    )
                    .0
                    .font(ST_HEI_TI_LIGHT)
                    .padding(15)
                    .size(17)
                    .style(Bl3UiStyle)
                    .into_element(),
                )
                .spacing(15)
                .width(Length::FillPortion(9))
                .align_items(Alignment::Center),
            )
            .push(
                Button::new(
                    &mut currency_state.max_money_button_state,
                    Text::new("最大").font(ST_HEI_TI_LIGHT).size(17),
                )
                .on_press(InteractionMessage::ManageSaveInteraction(
                    ManageSaveInteractionMessage::Currency(
                        SaveCurrencyInteractionMessage::MaxMoneyPressed,
                    ),
                ))
                .padding(10)
                .style(Bl3UiStyle)
                .into_element(),
            )
            .align_items(Alignment::Center),
    )
    .width(Length::Fill)
    .height(Length::Units(36))
    .style(Bl3UiStyle);

    let eridium = Container::new(
        Row::new()
            .push(
                LabelledElement::create(
                    "E 币/镒币",
                    Length::Units(75),
                    NumberInput::new(
                        &mut currency_state.eridium_input_state,
                        currency_state.eridium_input,
                        0,
                        None,
                        |v| {
                            InteractionMessage::ManageSaveInteraction(
                                ManageSaveInteractionMessage::Currency(
                                    SaveCurrencyInteractionMessage::Eridium(v),
                                ),
                            )
                        },
                    )
                    .0
                    .font(ST_HEI_TI_LIGHT)
                    .padding(15)
                    .size(17)
                    .style(Bl3UiStyle)
                    .into_element(),
                )
                .spacing(15)
                .width(Length::FillPortion(9))
                .align_items(Alignment::Center),
            )
            .push(
                Button::new(
                    &mut currency_state.max_eridium_button_state,
                    Text::new("最大").font(ST_HEI_TI_LIGHT).size(17),
                )
                .on_press(InteractionMessage::ManageSaveInteraction(
                    ManageSaveInteractionMessage::Currency(
                        SaveCurrencyInteractionMessage::MaxEridiumPressed,
                    ),
                ))
                .padding(10)
                .style(Bl3UiStyle)
                .into_element(),
            )
            .align_items(Alignment::Center),
    )
    .width(Length::Fill)
    .height(Length::Units(36))
    .style(Bl3UiStyle);

    let all_contents = Column::new().push(money).push(eridium).spacing(20);

    Container::new(all_contents).padding(30)
}
