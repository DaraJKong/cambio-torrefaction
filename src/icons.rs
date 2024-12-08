use iced::{widget::text, Element, Font};

pub fn icon<'a, Message>(codepoint: char) -> Element<'a, Message> {
    const ICONS_FONT: Font = Font::with_name("app-icons");

    text(codepoint).font(ICONS_FONT).size(35).center().into()
}

pub fn home_icon<'a, Message>() -> Element<'a, Message> {
    icon('\u{E801}')
}

pub fn cog_icon<'a, Message>() -> Element<'a, Message> {
    icon('\u{E800}')
}
