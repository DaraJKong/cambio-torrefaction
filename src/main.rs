use cambio_torrefaction::Styling;

fn main() -> iced::Result {
    iced::application("Iced Test", Styling::update, Styling::view)
        .theme(Styling::theme)
        .centered()
        .run()
}
