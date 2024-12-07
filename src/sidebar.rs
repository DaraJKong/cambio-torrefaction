use iced::{
    widget::{button, column, row, rule, text, vertical_rule},
    Alignment, Background, Color, Element, Length,
};

use crate::icons::icon;

pub enum Tab {
    Icon(char),
    Text(String),
    IconText(char, String),
}

pub struct Sidebar {
    tabs: Vec<Tab>,
    selected: usize,
}

#[derive(Clone, Debug)]
pub enum Message {
    TabSelected(usize),
}

impl Tab {
    fn view(&self, id: usize, selected: usize) -> Element<Message> {
        let indicator = vertical_rule(2).style(move |theme| {
            let mut style = rule::default(theme);

            style.width = 2;

            if id == selected {
                style.color = theme.palette().primary;
            } else {
                style.color = Color::TRANSPARENT;
            }

            style
        });

        let button = match &self {
            Tab::Icon(char) => button(icon(*char)),
            Tab::Text(string) => button(text(string)),
            Tab::IconText(char, string) => {
                let icon = icon(*char);
                let text = text(string);

                button(column![icon, text].spacing(10))
            }
        }
        .width(65)
        .height(65)
        .style(move |theme, status| {
            let mut style = button::text(theme, status);

            if id == selected {
                style.text_color = theme.extended_palette().background.weak.text;
            }

            style
        })
        .on_press(Message::TabSelected(id));

        row![indicator, button].height(65).into()
    }
}

impl Sidebar {
    pub fn update(&mut self, message: Message) {
        match message {
            Message::TabSelected(id) => {
                self.selected = id;
            }
        }
    }

    pub fn view(&self) -> Element<Message> {
        column(
            self.tabs
                .iter()
                .enumerate()
                .map(|(id, tab)| tab.view(id, self.selected)),
        )
        .into()
    }
}

pub fn icon_tab(char: char) -> Tab {
    Tab::Icon(char)
}

pub fn text_tab(string: &str) -> Tab {
    Tab::Text(string.to_string())
}

pub fn icon_text_tab(char: char, string: &str) -> Tab {
    Tab::IconText(char, string.to_string())
}

pub fn sidebar(tabs: Vec<Tab>, selected: usize) -> Sidebar {
    Sidebar { tabs, selected }
}
