use iced::{
    theme::Theme,
    widget::{button, column, container, text},
    Element, Length, Sandbox, Settings,
};

mod model;
fn main() -> iced::Result {
    MainMenu::run(Settings::default())
}

#[derive(Default)]
struct MainMenu {
    theme: Theme,
}

#[derive(Debug, Clone, Copy)]
enum Message {
    ChangeTheme,
}

impl Sandbox for MainMenu {
    type Message = Message;

    fn new() -> Self {
        Self { theme: Theme::Dark }
    }

    fn title(&self) -> String {
        String::from("Windmap")
    }

    fn update(&mut self, message: Self::Message) {
        match message {
            Message::ChangeTheme => {
                self.theme = if self.theme == Theme::Dark {
                    Theme::Light
                } else {
                    Theme::Dark
                }
            }
        }
    }

    fn view(&self) -> Element<Message> {
        let text_a = text(String::from("Hello world"));
        let theme_button = button(text("换主题").width(Length::Fill))
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
