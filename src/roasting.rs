use iced::{
    Alignment, Element,
    Length::Fill,
    Point, Rectangle, Renderer, Subscription, Task, Theme, mouse,
    time::{self, milliseconds},
    widget::{
        canvas,
        canvas::{Program, Frame, Geometry, Path, Stroke},
        column, container, horizontal_space, row, text,
    },
};
use std::time::Instant;

use crate::sensor;
use sensor::{Error, TempData};

#[derive(Clone, Debug)]
pub struct Roasting {
    bean_curve: RoastCurve,
    sensors: Vec<TempSensor>,
    last_id: usize,
}

#[derive(Debug, Clone)]
pub enum Message {
    SensorUpdated(usize, Update),
    TryReconnect(Instant),
}

impl Roasting {
    pub fn new_sensor(&mut self, name: &str, channel: i32) -> Task<Update> {
        let id = self.last_id;
        self.last_id += 1;
        self.sensors
            .push(TempSensor::new(id, name, 0, 572104, channel));
        self.sensors[id].connect()
    }

    pub fn boot() -> (Self, Task<Message>) {
        let mut roasting = Self {
            bean_curve: RoastCurve::default(),
            sensors: Vec::new(),
            last_id: 0,
        };

        let bean_task = roasting.new_sensor("Bean", 0);
        let exhaust_task = roasting.new_sensor("Exhaust", 1);

        (
            roasting,
            Task::batch([
                bean_task.map(|update| Message::SensorUpdated(0, update)),
                exhaust_task.map(|update| Message::SensorUpdated(1, update)),
            ]),
        )
    }

    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::SensorUpdated(id, update) => {
                let _ = self.sensors[id].update(update);
                if id == 0 {
                    if let State::Connected(temp_data) = &self.sensors[0].state {
                        self.bean_curve.points.push(temp_data.clone());
                    }
                }
                Task::none()
            }
            Message::TryReconnect(_) => {
                Task::batch(self.sensors.iter_mut().enumerate().map(|(i, s)| {
                    match s.state {
                        State::Disconnected | State::Errored(_) => s
                            .connect()
                            .map(move |update| Message::SensorUpdated(i, update)),
                        _ => Task::none(),
                    }
                }))
            }
        }
    }

    pub fn subscription(&self) -> Subscription<Message> {
        Subscription::batch(self.sensors.iter().map(|s| s.subscription()))
    }

    pub fn view(&self) -> Element<Message> {
        let title = text("Roasting").size(30);

        let sensors = column(self.sensors.iter().map(|s| s.view()));

        let canvas = canvas(&self.bean_curve);

        let roasting = column![title, sensors, canvas].spacing(20);

        container(roasting.max_width(800).spacing(20))
            .center_x(Fill)
            .padding(20)
            .into()
    }
}

#[derive(Debug, Clone)]
pub enum Update {
    EventReceived(sensor::Event),
    Disconnected(Result<(), Error>),
}

#[derive(Debug, Default, Clone)]
struct TempSensor {
    id: usize,
    name: String,
    hub_port: i32,
    serial_number: i32,
    channel: i32,
    state: State,
}

#[derive(Debug, Default, Clone)]
enum State {
    #[default]
    Created,
    Connected(TempData),
    Disconnected,
    Errored(Error),
}

impl TempSensor {
    fn new(id: usize, name: &str, hub_port: i32, serial_number: i32, channel: i32) -> Self {
        Self {
            id,
            name: name.to_string(),
            hub_port,
            serial_number,
            channel,
            state: State::default(),
        }
    }

    fn connect(&mut self) -> Task<Update> {
        match self.state {
            State::Created | State::Disconnected | State::Errored(_) => Task::sip(
                sensor::connect_temperature(self.hub_port, self.serial_number, self.channel),
                Update::EventReceived,
                Update::Disconnected,
            ),
            State::Connected(_) => Task::none(),
        }
    }

    fn update(&mut self, update: Update) -> Task<Update> {
        match update {
            Update::EventReceived(event) => match event {
                sensor::Event::Change(td) => {
                    self.state = State::Connected(td);
                    Task::none()
                }
                sensor::Event::Attach => {
                    println!("Attach: {}", self.name);
                    Task::none()
                }
                sensor::Event::Detach => {
                    println!("Detach: {}", self.name);
                    self.state = State::Disconnected;
                    Task::none()
                }
            },
            Update::Disconnected(result) => {
                self.state = match result {
                    Ok(_) => State::Disconnected,
                    Err(error) => State::Errored(error),
                };
                Task::none()
            }
        }
    }

    fn subscription(&self) -> Subscription<Message> {
        match self.state {
            State::Disconnected | State::Errored(_) => {
                time::every(milliseconds(100)).map(Message::TryReconnect)
            }
            _ => Subscription::none(),
        }
    }

    fn view(&self) -> Element<Message> {
        let temp = match &self.state {
            State::Created => text("Loading...").style(text::base),
            State::Connected(temp_data) => {
                text(format!("{:.1} °C", temp_data.temp)).style(text::success)
            }
            State::Disconnected => text("Disconnected!").style(text::danger),
            State::Errored(error) => text(format!("Error! {}", error)).style(text::danger),
        };

        row![
            text(format!("{}:", self.name)),
            horizontal_space(),
            temp.size(25)
        ]
        .width(200)
        .align_y(Alignment::Center)
        .into()
    }
}

#[derive(Debug, Clone, Default)]
struct RoastCurve {
    points: Vec<TempData>,
}

impl<Message> Program<Message> for RoastCurve {
    type State = ();

    fn draw(
        &self,
        _state: &(),
        renderer: &Renderer,
        _theme: &Theme,
        bounds: Rectangle,
        _cursor: mouse::Cursor,
    ) -> Vec<Geometry> {
        let mut frame = Frame::new(renderer, bounds.size());

        let curve = Path::new(|p| {
            let mut points = self.points.iter();
            if let Some(temp_data) = points.next() {
                let start_time = temp_data.time;
                p.move_to(Point::new(temp_data.temp as f32, 0.0));
                for temp_data in points {
                    p.line_to(Point::new(
                        temp_data.temp as f32,
                        temp_data.time.duration_since(start_time).as_secs() as f32,
                    ));
                }
            }
        });

        frame.stroke(&curve, Stroke::default());

        vec![frame.into_geometry()]
    }
}
