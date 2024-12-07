use iced::{
    widget::{row, vertical_rule},
    Element, Theme,
};

mod icons;
mod recipe;
mod settings;
mod sidebar;

use recipe::Recipe;
use settings::Settings;
use sidebar::{icon_tab, sidebar, Sidebar};

pub struct App {
    screen: Screen,
    sidebar: Sidebar,
    recipe: Recipe,
    settings: Settings,
}

impl Default for App {
    fn default() -> Self {
        App {
            screen: Screen::default(),
            sidebar: sidebar(vec![icon_tab('\u{E801}'), icon_tab('\u{E800}')], 0),
            recipe: Recipe::default(),
            settings: Settings::default(),
        }
    }
}

#[derive(Clone, Debug)]
pub enum Screen {
    Recipe,
    Settings,
}

impl Default for Screen {
    fn default() -> Self {
        Screen::Recipe
    }
}

impl TryFrom<usize> for Screen {
    type Error = ();

    fn try_from(v: usize) -> Result<Self, Self::Error> {
        match v {
            x if x == Screen::Recipe as usize => Ok(Screen::Recipe),
            x if x == Screen::Settings as usize => Ok(Screen::Settings),
            _ => Err(()),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Message {
    ScreenSelected(Screen),
    Sidebar(sidebar::Message),
    Recipe(recipe::Message),
    Settings(settings::Message),
}

impl App {
    pub fn update(app: &mut App, message: Message) {
        match message {
            Message::ScreenSelected(selected) => app.screen = selected,
            Message::Sidebar(message) => {
                match message {
                    sidebar::Message::TabSelected(id) => {
                        app.screen = id.try_into().unwrap();
                    }
                }

                app.sidebar.update(message);
            }
            Message::Recipe(message) => app.recipe.update(message),
            Message::Settings(message) => app.settings.update(message),
        }
    }

    pub fn view(app: &App) -> Element<Message> {
        let sidebar = app.sidebar.view().map(Message::Sidebar);

        let screen = match &app.screen {
            Screen::Recipe => app.recipe.view().map(Message::Recipe),
            Screen::Settings => app.settings.view().map(Message::Settings),
        };

        row![sidebar, vertical_rule(1), screen].into()
    }

    pub fn theme(app: &App) -> Theme {
        app.settings.theme()
    }
}
