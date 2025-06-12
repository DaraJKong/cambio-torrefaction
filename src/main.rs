use cambio_torrefaction::App;
use iced::window;

fn main() -> iced::Result {
    iced::application(App::boot, App::update, App::view)
        .title("Cambio Torréfaction")
        .subscription(App::subscription)
        .theme(App::theme)
        .font(include_bytes!("../assets/fonts/app-icons.ttf"))
        .window(window::Settings {
            icon: Some(
                window::icon::from_file_data(
                    include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/icon.ico")),
                    None,
                )
                .expect("icon file should be reachable and in ICO file format"),
            ),
            ..Default::default()
        })
        .centered()
        .antialiasing(true)
        .run()
}
