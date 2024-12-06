use cambio_torrefaction::App;

fn main() -> iced::Result {
    iced::application("Cambio Torréfaction", App::update, App::view)
        .theme(App::theme)
        .centered()
        .run_with(App::new)
}
