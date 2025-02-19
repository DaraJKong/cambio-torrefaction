use iced::{
    widget::{button, column, container, horizontal_space, row, rule, svg, text, vertical_rule},
    Color, Element,
};

use crate::icons::icon;

const ICON_BOX: f32 = 65.0;
const TEXT_SIZE: f32 = 25.0;
const TEXT_PADDING: f32 = 15.0;

pub struct Tab {
    icon: Option<char>,
    text: Option<String>,
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
    pub fn new(icon: Option<char>, text: Option<&str>) -> Self {
        Tab {
            icon,
            text: text.map(|x| x.to_string()),
        }
    }

    pub fn icon(char: char) -> Tab {
        Tab::new(Some(char), None)
    }

    pub fn text(str: &str) -> Tab {
        Tab::new(None, Some(str))
    }

    pub fn icon_text(char: char, str: &str) -> Tab {
        Tab::new(Some(char), Some(str))
    }

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

        let mut row = row![indicator].height(ICON_BOX);

        if let Some(char) = self.icon {
            row = row.push(container(icon(char)).center(ICON_BOX));
        }

        if let Some(string) = &self.text {
            if self.icon.is_none() {
                row = row.push(horizontal_space().width(TEXT_PADDING));
            }
            row = row.push(container(text(string.clone()).size(TEXT_SIZE)).center_y(ICON_BOX));
            row = row.push(horizontal_space().width(TEXT_PADDING));
        }

        button(row)
            .padding(0)
            .style(move |theme, status| {
                let mut style = button::text(theme, status);

                if id == selected {
                    style.text_color = theme.extended_palette().background.weak.text;
                }

                style
            })
            .on_press(Message::TabSelected(id))
            .into()
    }
}

impl Sidebar {
    pub fn new(tabs: Vec<Tab>, selected: usize) -> Self {
        Sidebar { tabs, selected }
    }

    pub fn update(&mut self, message: Message) {
        match message {
            Message::TabSelected(id) => {
                self.selected = id;
            }
        }
    }

    pub fn view(&self) -> Element<Message> {
        let handle =
            svg::Handle::from_path(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/logo.svg"));

        row![
            column![
                container(svg(handle).width(40)).center(ICON_BOX + 2.0),
                column(
                    self.tabs
                        .iter()
                        .enumerate()
                        .map(|(id, tab)| tab.view(id, self.selected)),
                )
            ],
            vertical_rule(1)
        ]
        .into()
    }
}
