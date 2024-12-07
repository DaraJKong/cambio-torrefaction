use cambio_torrefaction::App;

fn main() -> iced::Result {
    iced::application("Cambio Torréfaction", App::update, App::view)
        .font(include_bytes!("../fonts/app-icons.ttf"))
        .theme(App::theme)
        .antialiasing(true)
        .centered()
        .run()
}
