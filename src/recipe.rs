use iced::widget::{
    button, center, checkbox, column, progress_bar, row, scrollable, slider, text_input, toggler,
    vertical_rule, vertical_space,
};
use iced::{Center, Element, Fill};

#[derive(Clone, Default, Debug)]
pub struct Recipe {
    input_value: String,
    slider_value: f32,
    checkbox_value: bool,
    toggler_value: bool,
}

#[derive(Debug, Clone)]
pub enum Message {
    InputChanged(String),
    ButtonPressed,
    SliderChanged(f32),
    CheckboxToggled(bool),
    TogglerToggled(bool),
}

impl Recipe {
    pub fn update(&mut self, message: Message) {
        match message {
            Message::InputChanged(value) => self.input_value = value,
            Message::ButtonPressed => {}
            Message::SliderChanged(value) => self.slider_value = value,
            Message::CheckboxToggled(value) => self.checkbox_value = value,
            Message::TogglerToggled(value) => self.toggler_value = value,
        }
    }

    pub fn view(&self) -> Element<Message> {
        let text_input = text_input("Type something...", &self.input_value)
            .on_input(Message::InputChanged)
            .padding(10)
            .size(20);

        let button = button("Submit")
            .padding(10)
            .on_press(Message::ButtonPressed);

        let slider = slider(0.0..=100.0, self.slider_value, Message::SliderChanged);

        let progress_bar = progress_bar(0.0..=100.0, self.slider_value);

        let scrollable = scrollable(column![
            "Scroll me!",
            vertical_space().height(800),
            "You did it!"
        ])
        .width(Fill)
        .height(100);

        let checkbox =
            checkbox("Check me!", self.checkbox_value).on_toggle(Message::CheckboxToggled);

        let toggler = toggler(self.toggler_value)
            .label("Toggle me!")
            .on_toggle(Message::TogglerToggled)
            .spacing(10);

        let content = center(
            column![
                row![text_input, button].spacing(10).align_y(Center),
                slider,
                progress_bar,
                row![
                    scrollable,
                    vertical_rule(38),
                    column![checkbox, toggler].spacing(20)
                ]
                .spacing(10)
                .height(100)
                .align_y(Center),
            ]
            .spacing(20)
            .padding(20)
            .max_width(600),
        );

        content.into()
    }
}
