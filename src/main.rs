use iced::{
    font::{Family, Weight},
    theme::Theme,
    widget::{button, column, container, text},
    Element, Font, Length, Sandbox, Settings,
};

mod config;
mod model;

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
}

#[derive(Debug, Clone, Copy)]
enum Message {
    ChangeTheme,
}

impl Sandbox for Mind {
    type Message = Message;

    fn new() -> Self {
        Self {
            theme: Theme::Dark,
            theme_name: "暗".to_owned(),
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
        }
    }

    fn view(&self) -> Element<Message> {
        let text_a = text(String::from("你好小橙子"));
        let theme_button = button(text(&self.theme_name).width(Length::Fill))
            .on_press(Message::ChangeTheme)
            .padding(10);
        container(
            column![
                container(text_a).width(Length::Fill).center_x(),
                container(theme_button).width(Length::Fill).center_x(),
            ]
            .spacing(25),
        )
        .height(Length::Fill)
        .width(Length::Fill)
        .center_x()
        .center_y()
        .into()
    }

    fn theme(&self) -> Theme {
        self.theme.clone()
    }
}
