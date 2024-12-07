use iced::widget::{center, column, pick_list, text};
use iced::{Element, Fill, Theme};

#[derive(Clone, Debug)]
pub struct Settings {
    theme: Theme,
}

impl Default for Settings {
    fn default() -> Self {
        Settings {
            theme: Theme::TokyoNight,
        }
    }
}

#[derive(Debug, Clone)]
pub enum Message {
    ThemeSelected(Theme),
}

impl Settings {
    pub fn theme(&self) -> Theme {
        self.theme.clone()
    }

    pub fn update(&mut self, message: Message) {
        match message {
            Message::ThemeSelected(theme) => {
                self.theme = theme;
            }
        }
    }

    pub fn view(&self) -> Element<Message> {
        let choose_theme = column![
            text("Theme:"),
            pick_list(Theme::ALL, Some(&self.theme), Message::ThemeSelected).width(Fill),
        ]
        .spacing(10);

        let content: Element<'_, Message> =
            center(column![choose_theme].spacing(20).padding(20).max_width(600)).into();

        content.into()
    }
}
