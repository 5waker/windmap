use iced::{
    font::{Family, Weight}, theme::Theme, widget::{button, column, container, text}, Element, Font, Length, Sandbox, Settings
};

mod model;
fn main() -> iced::Result {
    MainMenu::run(Settings {
        // Not Support include_bytes!("../resource/SourceHanSerifSC-VF.ttf")
        default_font: Font{
            #[cfg(target_os = "linux")]
            family: Family::Name("文泉驿微米黑"),
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
struct MainMenu {
    theme: Theme,
    theme_name: String,
}

#[derive(Debug, Clone, Copy)]
enum Message {
    ChangeTheme,
}

impl Sandbox for MainMenu {
    type Message = Message;

    fn new() -> Self {
        Self { theme: Theme::Dark, theme_name: "暗".to_owned() }
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
        let text_a = text(String::from("你好世界 Hello World"));
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
