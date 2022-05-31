use iced::alignment::{Horizontal, Vertical};
use iced::{
    button, scrollable, text_input, Alignment, Button, Checkbox, Color, Column, Container, Element,
    Length, Row, Scrollable, Text,
};
use rayon::iter::IntoParallelRefIterator;
use rayon::iter::ParallelIterator;

use bl3_save_edit_core::bl3_item::Bl3Item;
use bl3_save_edit_core::resources::{ResourceCategorizedParts, ResourcePart};

use crate::bl3_ui::{Bl3Message, InteractionMessage};
use crate::bl3_ui_style::{Bl3UiStyle, Bl3UiStyleNoBorder};
use crate::resources::fonts::{ST_HEI_TI_LIGHT};
use crate::views::item_editor::extra_part_info::add_extra_part_info;
use crate::views::item_editor::item_button_style::ItemEditorButtonStyle;
use crate::views::item_editor::parts::filter_parts;
use crate::views::item_editor::parts_tab_bar::AvailablePartType;
use crate::views::item_editor::ItemEditorInteractionMessage;
use crate::views::tab_bar_button::tab_bar_button;
use crate::views::{InteractionExt, NO_SEARCH_RESULTS_FOUND_MESSAGE};
use crate::widgets::text_input_limited::TextInputLimited;

#[derive(Debug, Copy, Clone, Default)]
pub struct AvailablePartTypeIndex {
    pub category_index: usize,
    pub part_index: usize,
}

#[derive(Debug, Clone)]
pub struct AvailableCategorizedPart {
    pub category: String,
    pub parts: Vec<AvailableResourcePart>,
}

impl AvailableCategorizedPart {
    pub fn new(
        category_id: usize,
        category: String,
        part_type: AvailablePartType,
        parts: Vec<ResourcePart>,
    ) -> Self {
        let parts = parts
            .into_iter()
            .enumerate()
            .map(|(id, p)| AvailableResourcePart::new(category_id, id, part_type.clone(), p))
            .collect();

        Self { category, parts }
    }

    pub fn from_resource_categorized_parts(
        part_type: AvailablePartType,
        parts: &[ResourceCategorizedParts],
    ) -> Vec<Self> {
        parts
            .iter()
            .cloned()
            .enumerate()
            .map(|(cat_id, cat_p)| {
                AvailableCategorizedPart::new(
                    cat_id,
                    cat_p.category,
                    part_type.clone(),
                    cat_p.parts,
                )
            })
            .collect::<Vec<_>>()
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct AvailableResourcePart {
    category_index: usize,
    part_index: usize,
    part_type: AvailablePartType,
    pub part: ResourcePart,
    button_state: button::State,
}

impl AvailableResourcePart {
    pub fn new(
        category_index: usize,
        part_index: usize,
        part_type: AvailablePartType,
        part: ResourcePart,
    ) -> Self {
        Self {
            category_index,
            part_index,
            part_type,
            part,
            button_state: button::State::new(),
        }
    }

    pub fn view<F>(&mut self, is_active: bool, interaction_message: F) -> Element<Bl3Message>
    where
        F: Fn(ItemEditorInteractionMessage) -> InteractionMessage + 'static + Copy,
    {
        let part_contents_col = Column::new()
            .push(Text::new(&self.part.name).font(ST_HEI_TI_LIGHT).size(16))
            .spacing(10);

        let part_contents_col = add_extra_part_info(part_contents_col, &self.part.info);

        let part_contents = Container::new(part_contents_col).align_x(Horizontal::Left);

        let index = AvailablePartTypeIndex {
            category_index: self.category_index,
            part_index: self.part_index,
        };

        Button::new(&mut self.button_state, part_contents)
            .on_press(interaction_message(match self.part_type {
                AvailablePartType::Parts => {
                    ItemEditorInteractionMessage::AvailablePartPressed(index)
                }
                AvailablePartType::Anointments => {
                    ItemEditorInteractionMessage::AvailableAnointmentPressed(index)
                }
            }))
            .padding(10)
            .width(Length::Fill)
            .style(ItemEditorButtonStyle { is_active })
            .into_element()
    }
}

#[derive(Debug, Default)]
pub struct AvailableParts {
    pub scrollable_state: scrollable::State,
    pub part_type_index: AvailablePartTypeIndex,
    pub parts: Vec<AvailableCategorizedPart>,
    pub show_all_available_parts: bool,
    pub parts_tab_type: AvailablePartType,
    pub available_parts_tab_button_state: button::State,
    pub available_anointments_tab_button_state: button::State,
    pub search_input: String,
    pub search_input_state: text_input::State,
}

impl AvailableParts {
    pub fn view<F>(
        &mut self,
        item: &Bl3Item,
        anointments_list: &[ResourceCategorizedParts],
        specific_parts_list: Option<&Vec<ResourceCategorizedParts>>,
        all_parts_list: Option<&Vec<ResourceCategorizedParts>>,
        interaction_message: F,
    ) -> Container<Bl3Message>
    where
        F: Fn(ItemEditorInteractionMessage) -> InteractionMessage + 'static + Copy,
    {
        let selected_available_part_type_index = &self.part_type_index;

        let title_row = Row::new()
            .push(
                Container::new(tab_bar_button(
                    &mut self.available_parts_tab_button_state,
                    AvailablePartType::Parts,
                    &self.parts_tab_type,
                    interaction_message(ItemEditorInteractionMessage::AvailablePartsTabPressed),
                    None,
                ))
                .padding(1)
                .width(Length::FillPortion(2)),
            )
            .push(
                Container::new(tab_bar_button(
                    &mut self.available_anointments_tab_button_state,
                    AvailablePartType::Anointments,
                    &self.parts_tab_type,
                    interaction_message(
                        ItemEditorInteractionMessage::AvailableAnointmentsTabPressed,
                    ),
                    None,
                ))
                .padding(1)
                .width(Length::FillPortion(2)),
            )
            .align_items(Alignment::Center);

        let mut available_parts_column = Column::new().push(Container::new(title_row));

        let available_parts = if item.item_parts.is_some() {
            match self.parts_tab_type {
                AvailablePartType::Parts => {
                    let specific_parts = specific_parts_list.map(|i| {
                        AvailableCategorizedPart::from_resource_categorized_parts(
                            AvailablePartType::Parts,
                            i,
                        )
                    });

                    let all_parts = all_parts_list.map(|i| {
                        AvailableCategorizedPart::from_resource_categorized_parts(
                            AvailablePartType::Parts,
                            i,
                        )
                    });

                    let show_all_parts_checkbox =
                        Checkbox::new(self.show_all_available_parts, "Show All Parts", move |c| {
                            interaction_message(
                                ItemEditorInteractionMessage::ShowAllAvailablePartsSelected(c),
                            )
                        })
                        .size(17)
                        .font(ST_HEI_TI_LIGHT)
                        .text_color(Color::from_rgb8(220, 220, 220))
                        .text_size(17)
                        .style(Bl3UiStyle)
                        .into_element();

                    if specific_parts.is_some() {
                        available_parts_column = available_parts_column.push(
                            Container::new(
                                Container::new(show_all_parts_checkbox)
                                    .padding(15)
                                    .width(Length::Fill)
                                    .style(Bl3UiStyleNoBorder),
                            )
                            .padding(1),
                        );
                    }

                    if self.show_all_available_parts || specific_parts.is_none() {
                        all_parts
                    } else {
                        specific_parts
                    }
                }
                AvailablePartType::Anointments => {
                    Some(AvailableCategorizedPart::from_resource_categorized_parts(
                        AvailablePartType::Anointments,
                        anointments_list,
                    ))
                }
            }
        } else {
            None
        };

        if let Some(available_parts) = &available_parts {
            let amount: usize = available_parts.iter().map(|cat_p| cat_p.parts.len()).sum();

            let search_placeholder = match self.parts_tab_type {
                AvailablePartType::Parts => format!("Search {} Available Parts...", amount),
                AvailablePartType::Anointments => {
                    format!("Search {} Available Anointments...", amount)
                }
            };

            let search_input = TextInputLimited::new(
                &mut self.search_input_state,
                &search_placeholder,
                &self.search_input,
                500,
                move |s| {
                    interaction_message(
                        ItemEditorInteractionMessage::AvailablePartsSearchInputChanged(s),
                    )
                },
            )
            .0
            .font(ST_HEI_TI_LIGHT)
            .padding(10)
            .size(17)
            .style(Bl3UiStyle)
            .into_element();

            available_parts_column = available_parts_column.push(search_input);
        }

        if let Some(available_parts) = &available_parts {
            self.parts = available_parts.clone();

            let filtered_parts = filter_parts(&self.search_input, available_parts);

            let category_name = if self.parts_tab_type == AvailablePartType::Anointments {
                Some("Anointment")
            } else {
                None
            };

            if !filtered_parts.is_empty() {
                let available_parts_list = self.parts.iter_mut().enumerate().fold(
                    Column::new(),
                    |mut curr, (cat_index, cat_parts)| {
                        if cat_parts
                            .parts
                            .par_iter()
                            .any(|cat_p| filtered_parts.contains(&cat_p))
                        {
                            curr = curr.push(
                                Container::new(
                                    Text::new(category_name.unwrap_or(&cat_parts.category))
                                        .font(ST_HEI_TI_LIGHT)
                                        .size(17)
                                        .color(Color::from_rgb8(242, 203, 5)),
                                )
                                .width(Length::Fill)
                                .style(Bl3UiStyleNoBorder)
                                .padding(10),
                            );
                        }

                        for (part_index, p) in cat_parts.parts.iter_mut().enumerate() {
                            if filtered_parts.contains(&&*p) {
                                let is_active = selected_available_part_type_index.category_index
                                    == cat_index
                                    && selected_available_part_type_index.part_index == part_index;

                                curr = curr.push(p.view(is_active, interaction_message));
                            }
                        }

                        curr
                    },
                );

                available_parts_column = available_parts_column.push(
                    Container::new(
                        Scrollable::new(&mut self.scrollable_state)
                            .push(available_parts_list)
                            .height(Length::Fill)
                            .width(Length::Fill),
                    )
                    .padding(1),
                );
            } else {
                available_parts_column = available_parts_column.push(
                    Container::new(
                        Text::new(NO_SEARCH_RESULTS_FOUND_MESSAGE)
                            .font(ST_HEI_TI_LIGHT)
                            .size(17)
                            .color(Color::from_rgb8(220, 220, 220)),
                    )
                    .width(Length::Fill)
                    .height(Length::Fill)
                    .align_x(Horizontal::Center)
                    .align_y(Vertical::Center),
                );
            }
        } else {
            available_parts_column = available_parts_column.push(
                Container::new(
                    Text::new("No available parts or anointments found.")
                        .font(ST_HEI_TI_LIGHT)
                        .size(17)
                        .color(Color::from_rgb8(220, 220, 220)),
                )
                .width(Length::Fill)
                .height(Length::Fill)
                .align_x(Horizontal::Center)
                .align_y(Vertical::Center),
            )
        }

        Container::new(available_parts_column)
            .width(Length::FillPortion(2))
            .height(Length::Fill)
            .style(Bl3UiStyle)
    }
}
