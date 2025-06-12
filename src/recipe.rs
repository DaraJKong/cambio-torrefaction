use iced::{
    Element,
    Length::Fill,
    widget::{column, combo_box, container, row, scrollable, text},
};

use crate::data;

#[derive(Clone, Debug)]
pub struct Recipe {
    recipes: combo_box::State<data::Recipe>,
    selected: Option<data::Recipe>,
}

impl Recipe {
    pub fn new() -> Self {
        Recipe {
            recipes: combo_box::State::new(vec![
                data::DUMB_RECIPE.clone(),
                data::TIME_RECIPE.clone(),
            ]),
            selected: None,
        }
    }
}

#[derive(Debug, Clone)]
pub enum Message {
    RecipeSelected(data::Recipe),
}

impl Recipe {
    pub fn update(&mut self, message: Message) {
        match message {
            Message::RecipeSelected(recipe) => self.selected = Some(recipe),
        }
    }

    pub fn view(&self) -> Element<Message> {
        let header = row![combo_box(
            &self.recipes,
            "Select a recipe...",
            self.selected.as_ref(),
            Message::RecipeSelected,
        )];

        let title = text(
            self.selected
                .as_ref()
                .map_or("No recipe selected", |recipe| &recipe.name()),
        )
        .size(30);

        let steps = scrollable(
            column(self.selected.as_ref().map_or(Vec::new(), |recipe| {
                recipe.steps().iter().map(|step| step.view()).collect()
            }))
            .spacing(5),
        )
        .width(Fill);

        let recipe = column![title, steps].spacing(20);

        container(column![header, recipe].max_width(800).spacing(20))
            .center_x(Fill)
            .padding(20)
            .into()
    }
}
