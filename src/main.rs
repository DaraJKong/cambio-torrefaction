use cambio_torrefaction::App;
use iced::window;

fn main() -> iced::Result {
    iced::application("Cambio Torréfaction", App::update, App::view)
        .font(include_bytes!("../fonts/app-icons.ttf"))
        .theme(App::theme)
        .antialiasing(true)
        .centered()
        .window(window::Settings {
            icon: Some(
                window::icon::from_file_data(
                    include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/resources/icon.ico")),
                    None,
                )
                .expect("icon file should be reachable and in ICO file format"),
            ),
            ..Default::default()
        })
        .run()
}
