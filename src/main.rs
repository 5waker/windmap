use iced::{
    alignment, font::{Family, Weight}, mouse, theme::Theme, widget::{button, canvas ,canvas::{fill::Rule, Fill, Style}, column, container, text, text_input}, Color, Element, Font, Length, Point, Rectangle, Renderer, Sandbox, Settings, Size, Vector
};

mod config;
mod model;
mod error;

fn main() -> iced::Result {
    Mind::run(Settings {
        // Not Support include_bytes!("../resource/SourceHanSerifSC-VF.ttf")
        // TODO 当前是硬code,后面改
        default_font: Font {
            #[cfg(target_os = "linux")]
            family: Family::Name("霞鹜文楷"),
            #[cfg(target_os = "macos")]
            family: Family::Name("苹方-简"),
            #[cfg(target_os = "windows")]
            family: Family::Name("Microsoft Yahei"),
            weight: Weight::Normal,
            ..Default::default()
        },
        ..Settings::default()
    })
}

#[derive(Default)]
struct Mind {
    theme: Theme,
    theme_name: String,
    content: String,

    size: f32,
    angle: f32,
    scale: f32,
    cache: canvas::Cache,
}

#[derive(Debug, Clone)]
enum Message {
    ChangeTheme,
    NodeInput(String),
}

impl Sandbox for Mind {
    type Message = Message;

    fn new() -> Self {
        Self {
            theme: Theme::Dark,
            theme_name: "暗".to_owned(),
            content: "".to_owned(),

            size: 40.0,
            angle: 0.0,
            scale: 1.0,
            cache: canvas::Cache::new(),
        }
    }

    fn title(&self) -> String {
        String::from("Windmap")
    }

    fn update(&mut self, message: Self::Message) {
        match message {
            Message::ChangeTheme => {
                self.theme = if self.theme == Theme::Dark {
                    self.theme_name = "亮".to_owned();
                    Theme::Light
                } else {
                    self.theme_name = "暗".to_owned();
                    Theme::Dark
                }
            }
            Message::NodeInput(title) => self.content = title,
        }
    }

    fn view(&self) -> Element<Message> {
        let t_canvas = canvas(self as &Self)
            .width(Length::Fill)
            .height(Length::Fill);

        container(t_canvas)
            .width(Length::Fill)
            .height(Length::Fill)
            .padding(20)
            .into()
    }

    fn theme(&self) -> Theme {
        self.theme.clone()
    }
}

impl<Message> canvas::Program<Message> for Mind {
    type State = ();

    fn draw(
        &self,
        state: &Self::State,
        renderer: &Renderer,
        theme: &Theme,
        bounds: Rectangle,
        cursor: mouse::Cursor,
    ) -> Vec<canvas::Geometry> {
        let geometry = self.cache.draw(renderer, bounds.size(), |frame| {
            // let palette = theme.palette();
            let center = bounds.center();

            frame.translate(Vector::new(center.x, center.y));
            frame.scale(2.0);

            // frame.fill_text(canvas::Text {
            //     position: Point::new(0.0, 0.0),
            //     color: palette.text,
            //     size: self.size.into(),
            //     content: String::from("Vectorial Text! 🎉"),
            //     horizontal_alignment: alignment::Horizontal::Center,
            //     vertical_alignment: alignment::Vertical::Center,
            //     shaping: text::Shaping::Advanced,
            //     ..canvas::Text::default()
            // });
            frame.fill_rectangle(Point::new(-50.0, -50.0), Size::new(100.0, 100.0), Fill{
                style: Style::Solid(Color::WHITE),
                rule: Rule::NonZero,
            });
        });

        vec![geometry]
    }
}
