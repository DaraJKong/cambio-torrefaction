use iced::{
    Alignment, Color, Element,
    Length::Fill,
    Point, Rectangle, Renderer, Size, Subscription, Task, Theme, mouse,
    time::{self, milliseconds},
    widget::{
        button, canvas,
        canvas::{Frame, Geometry, Path, Program, Stroke},
        column, container, horizontal_space, row, text,
    },
};
use std::time::Instant;

use crate::sensor;
use sensor::{Error, TempData};

#[derive(Clone, Debug)]
pub struct Roasting {
    sensors: Vec<TempSensor>,
    last_id: usize,
    roast: Option<Roast>,
    roasting: bool,
}

#[derive(Debug, Clone)]
pub enum Message {
    SensorUpdated(usize, Update),
    TryReconnect(Instant),
    StartRoast,
    StopRoast,
}

impl Roasting {
    pub fn new_sensor(
        &mut self,
        name: &str,
        channel: i32,
        color: Color,
        curve_settings: CurveSettings,
    ) -> Task<Update> {
        let id = self.last_id;
        self.last_id += 1;
        self.sensors.push(TempSensor::new(
            id,
            name,
            color,
            curve_settings,
            0,
            572104,
            channel,
        ));
        self.sensors[id].connect()
    }

    pub fn boot() -> (Self, Task<Message>) {
        let mut roasting = Self {
            sensors: Vec::new(),
            last_id: 0,
            roast: None,
            roasting: false,
        };

        let bean_task = roasting.new_sensor(
            "Bean",
            0,
            Color::from_rgb(0., 0.5, 1.),
            CurveSettings {
                min: 0.0,
                max: 230.0,
                fit: CurveFit::Normal,
            },
        );
        let exhaust_task = roasting.new_sensor(
            "Exhaust",
            1,
            Color::from_rgb(1., 0., 0.),
            CurveSettings {
                min: 0.0,
                max: 230.0,
                fit: CurveFit::Normal,
            },
        );

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
                if let State::Connected(temp_data) = &self.sensors[id].state {
                    if let Some(roast) = &mut self.roast {
                        roast.curves[id].points.push(temp_data.clone());
                        roast.last_time = temp_data.time;
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
            Message::StartRoast => {
                self.roast = Some(Roast::new(
                    &self.sensors,
                    CurveSettings {
                        min: 0.0,
                        max: 17.0 * 60.0,
                        fit: CurveFit::Padding(0.0, 10.0),
                    },
                ));
                Task::none()
            }
            Message::StopRoast => {
                self.roast = None;
                Task::none()
            }
        }
    }

    pub fn subscription(&self) -> Subscription<Message> {
        Subscription::batch(self.sensors.iter().map(|s| s.subscription()))
    }

    pub fn view(&self) -> Element<Message> {
        let title = text("Roasting").size(30);
        let sensors = column(self.sensors.iter().map(|s| s.view()))
            .max_width(800)
            .spacing(20);

        let canvas: Element<_> = if let Some(roast) = &self.roast {
            column![
                canvas(roast).width(Fill).height(Fill),
                button("Stop Roast").on_press(Message::StopRoast),
            ]
            .spacing(20)
            .into()
        } else {
            button("Start Roast").on_press(Message::StartRoast).into()
        };

        let roasting = column![
            container(title).center_x(Fill),
            container(sensors).center_x(Fill),
            canvas
        ];

        container(roasting.spacing(20))
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
enum State {
    #[default]
    Created,
    Connected(TempData),
    Disconnected,
    Errored(Error),
}

#[derive(Debug, Clone)]
struct TempSensor {
    id: usize,
    name: String,
    color: Color,
    curve_settings: CurveSettings,
    hub_port: i32,
    serial_number: i32,
    channel: i32,
    state: State,
}

impl TempSensor {
    fn new(
        id: usize,
        name: &str,
        color: Color,
        curve_settings: CurveSettings,
        hub_port: i32,
        serial_number: i32,
        channel: i32,
    ) -> Self {
        Self {
            id,
            name: name.to_string(),
            color,
            curve_settings,
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
            text(format!("{}:", self.name)).color(self.color),
            horizontal_space(),
            temp.size(25)
        ]
        .width(200)
        .align_y(Alignment::Center)
        .into()
    }
}

#[derive(Clone, Debug, Default)]
enum CurveFit {
    #[default]
    Normal,
    Padding(f32, f32),
    AlwaysFit(f32, f32),
}

#[derive(Debug, Clone)]
struct CurveSettings {
    min: f32,
    max: f32,
    fit: CurveFit,
}

impl CurveSettings {
    fn window(&self, min: f32, max: f32) -> (f32, f32) {
        match self.fit {
            CurveFit::Normal => (self.min, self.max),
            CurveFit::Padding(pl, pr) => (self.min.min(min - pl), self.max.max(max + pr)),
            CurveFit::AlwaysFit(pl, pr) => (min - pl, max + pr),
        }
    }

    fn fit(window: (f32, f32), v: f32, size: f32) -> f32 {
        (v - window.0) / (window.1 - window.0) * size
    }

    fn fit_flip(window: (f32, f32), v: f32, size: f32) -> f32 {
        (1.0 - (v - window.0) / (window.1 - window.0)) * size
    }
}

#[derive(Debug, Clone)]
struct RoastCurve {
    source_id: usize,
    color: Color,
    settings: CurveSettings,
    points: Vec<TempData>,
}

impl RoastCurve {
    fn new(id: usize, color: Color, curve_settings: CurveSettings) -> Self {
        Self {
            source_id: id,
            color: color,
            settings: curve_settings,
            points: Vec::new(),
        }
    }

    fn path(
        &self,
        start_time: Instant,
        last_time: Instant,
        t_settings: &CurveSettings,
        size: Size,
    ) -> Path {
        Path::new(|p| {
            let iter = self.points.iter().map(|p| p.temp as f32);
            let min = iter.clone().reduce(f32::min).unwrap_or(0.);
            let max = iter.reduce(f32::max).unwrap_or(0.);

            let mut points = self.points.iter();
            if let Some(temp_data) = points.next() {
                let t_window =
                    t_settings.window(0.0, last_time.duration_since(start_time).as_secs() as f32);
                let v_window = self.settings.window(min, max);

                p.move_to(Point::new(
                    CurveSettings::fit(
                        t_window,
                        temp_data.time.duration_since(start_time).as_secs() as f32,
                        size.width,
                    ),
                    CurveSettings::fit_flip(v_window, temp_data.temp as f32, size.height),
                ));

                for temp_data in points {
                    p.line_to(Point::new(
                        CurveSettings::fit(
                            t_window,
                            temp_data.time.duration_since(start_time).as_secs() as f32,
                            size.width,
                        ),
                        CurveSettings::fit_flip(v_window, temp_data.temp as f32, size.height),
                    ));
                }
            }
        })
    }
}

#[derive(Clone, Debug)]
struct Roast {
    start_time: Instant,
    last_time: Instant,
    curves: Vec<RoastCurve>,
    settings: CurveSettings,
}

impl Roast {
    fn new(sensors: &Vec<TempSensor>, curve_settings: CurveSettings) -> Self {
        let now = Instant::now();

        Self {
            start_time: now,
            last_time: now,
            curves: sensors
                .iter()
                .map(|s| RoastCurve::new(s.id, s.color, s.curve_settings.clone()))
                .collect(),
            settings: curve_settings,
        }
    }
}

impl<Message> Program<Message> for Roast {
    type State = ();

    fn draw(
        &self,
        _state: &(),
        renderer: &Renderer,
        theme: &Theme,
        bounds: Rectangle,
        _cursor: mouse::Cursor,
    ) -> Vec<Geometry> {
        let size = bounds.size();

        let mut frame = Frame::new(renderer, size);

        for curve in &self.curves {
            let path = curve.path(self.start_time, self.last_time, &self.settings, size);

            frame.stroke(
                &path,
                Stroke {
                    style: iced::widget::canvas::Style::Solid(curve.color),
                    width: 2.5,
                    ..Default::default()
                },
            );
        }

        frame.stroke(
            &Path::rectangle(Point::ORIGIN, frame.size()),
            Stroke {
                style: iced::widget::canvas::Style::Solid(theme.palette().text),
                width: 1.0,
                ..Default::default()
            },
        );

        vec![frame.into_geometry()]
    }
}
