use iced::{
    event,
    keyboard::{self, key},
    widget::{self, row},
    Element, Event, Subscription, Task, Theme,
};

mod icons;
mod preferences;
mod recipe;
mod settings;
mod sidebar;

use preferences::Preferences;
use recipe::Recipe;
use settings::Settings;
use sidebar::{Sidebar, Tab};

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
            sidebar: Sidebar::new(vec![Tab::icon('\u{E801}'), Tab::icon('\u{E800}')], 0),
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
    Event(Event),
}

impl App {
    pub fn init() -> (App, Task<Message>) {
        let preferences = Preferences::load();

        (
            App {
                settings: Settings::new(preferences.unwrap()),
                ..Default::default()
            },
            Task::none(),
        )
    }

    pub fn subscription(&self) -> Subscription<Message> {
        event::listen().map(Message::Event)
    }

    pub fn update(app: &mut App, message: Message) -> Task<Message> {
        match message {
            Message::ScreenSelected(selected) => {
                app.screen = selected;

                Task::none()
            }
            Message::Sidebar(message) => {
                match message {
                    sidebar::Message::TabSelected(id) => {
                        app.screen = id.try_into().unwrap();
                    }
                }

                app.sidebar.update(message);

                Task::none()
            }
            Message::Recipe(message) => {
                app.recipe.update(message);
                Task::none()
            }
            Message::Settings(message) => {
                app.settings.update(message);
                Task::none()
            }
            Message::Event(event) => match event {
                Event::Keyboard(keyboard::Event::KeyPressed {
                    key: keyboard::Key::Named(key::Named::Tab),
                    modifiers,
                    ..
                }) => {
                    if modifiers.shift() {
                        widget::focus_previous()
                    } else {
                        widget::focus_next()
                    }
                }
                _ => Task::none(),
            },
        }
    }

    pub fn view(app: &App) -> Element<Message> {
        let sidebar = app.sidebar.view().map(Message::Sidebar);

        let screen = match &app.screen {
            Screen::Recipe => app.recipe.view().map(Message::Recipe),
            Screen::Settings => app.settings.view().map(Message::Settings),
        };

        row![sidebar, screen].into()
    }

    pub fn theme(app: &App) -> Theme {
        app.settings.theme()
    }
}
