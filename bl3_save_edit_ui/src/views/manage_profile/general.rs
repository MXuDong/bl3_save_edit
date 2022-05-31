use iced::{
    pick_list, text_input, tooltip, Alignment, Column, Container, Length, PickList, TextInput,
    Tooltip,
};

use bl3_save_edit_core::parser::HeaderType;

use crate::bl3_ui::{Bl3Message, InteractionMessage};
use crate::bl3_ui_style::{Bl3UiStyle, Bl3UiTooltipStyle};
use crate::resources::fonts::ST_HEI_TI_LIGHT;
use crate::views::manage_profile::ManageProfileInteractionMessage;
use crate::views::InteractionExt;
use crate::widgets::labelled_element::LabelledElement;

#[derive(Debug, Default)]
pub struct GeneralState {
    pub filename_input: String,
    pub filename_input_state: text_input::State,
    pub profile_type_selector: pick_list::State<HeaderType>,
    pub profile_type_selected: HeaderType,
}

#[derive(Debug, Clone)]
pub enum ProfileGeneralInteractionMessage {
    ProfileTypeSelected(HeaderType),
}

pub fn view(general_state: &mut GeneralState) -> Container<Bl3Message> {
    let file = Container::new(
        LabelledElement::create(
            "File",
            Length::Units(110),
            Tooltip::new(
                TextInput::new(
                    &mut general_state.filename_input_state,
                    "profile.sav",
                    &general_state.filename_input,
                    |_| InteractionMessage::Ignore,
                )
                .font(ST_HEI_TI_LIGHT)
                .padding(10)
                .size(17)
                .style(Bl3UiStyle)
                .into_element(),
                "Not editable",
                tooltip::Position::Top,
            )
            .gap(10)
            .padding(10)
            .font(ST_HEI_TI_LIGHT)
            .size(17)
            .style(Bl3UiTooltipStyle),
        )
        .spacing(15)
        .width(Length::FillPortion(9))
        .align_items(Alignment::Center),
    )
    .width(Length::Fill)
    .height(Length::Units(36))
    .style(Bl3UiStyle);

    let profile_type = Container::new(
        LabelledElement::create(
            "Profile Type",
            Length::Units(110),
            PickList::new(
                &mut general_state.profile_type_selector,
                &HeaderType::PROFILE_TYPES[..],
                Some(general_state.profile_type_selected),
                |h| {
                    InteractionMessage::ManageProfileInteraction(
                        ManageProfileInteractionMessage::General(
                            ProfileGeneralInteractionMessage::ProfileTypeSelected(h),
                        ),
                    )
                },
            )
            .font(ST_HEI_TI_LIGHT)
            .text_size(17)
            .width(Length::Fill)
            .padding(10)
            .style(Bl3UiStyle)
            .into_element(),
        )
        .spacing(15)
        .align_items(Alignment::Center),
    )
    .width(Length::Fill)
    .height(Length::Units(36))
    .style(Bl3UiStyle);

    let all_contents = Column::new().push(file).push(profile_type).spacing(20);

    Container::new(all_contents).padding(30)
}
