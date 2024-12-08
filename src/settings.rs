use iced::widget::{center, column, pick_list, text};
use iced::{Element, Fill, Theme};

use crate::preferences::Preferences;

#[derive(Clone, Debug, Default)]
pub struct Settings {
    preferences: Preferences,
}

#[derive(Debug, Clone)]
pub enum Message {
    ThemeSelected(Theme),
}

impl Settings {
    pub fn new(preferences: Preferences) -> Self {
        Settings { preferences }
    }

    pub fn theme(&self) -> Theme {
        self.preferences.theme.clone()
    }

    pub fn update(&mut self, message: Message) {
        match message {
            Message::ThemeSelected(theme) => {
                self.preferences.theme = theme;
                self.preferences.save().ok();
            }
        }
    }

    pub fn view(&self) -> Element<Message> {
        let choose_theme = column![
            text("Theme:"),
            pick_list(
                Theme::ALL,
                Some(&self.preferences.theme),
                Message::ThemeSelected
            )
            .width(Fill),
        ]
        .spacing(10);

        let content: Element<'_, Message> =
            center(column![choose_theme].spacing(20).padding(20).max_width(600)).into();

        content.into()
    }
}
