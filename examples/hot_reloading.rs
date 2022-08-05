// Based on https://github.com/hecrj/iced/blob/8d882d787e6b7fd7c2435f42f82933e2ed904edf/examples/styling/src/main.rs.

use iced::{
    button, scrollable, slider, text_input, Alignment, Application, Column, Command, Element,
    Length, Row, Settings, Space, Subscription, Text,
};
use iced_style_config::ReloadableTheme;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    Styling::run(Settings {
        flags: Some(theme(Some("examples/dark_theme.toml"))?),
        ..Default::default()
    })?;
    Ok(())
}

fn theme(
    #[allow(unused_variables)] path: Option<&str>,
) -> iced_style_config::Result<ReloadableTheme> {
    #[cfg(not(target_family = "wasm"))]
    if let Some(path) = path {
        return ReloadableTheme::from_file(path);
    }
    include_str!("dark_theme.toml").parse()
}

struct Styling {
    theme: ReloadableTheme,
    scroll: scrollable::State,
    input: text_input::State,
    input_value: String,
    button: button::State,
    slider: slider::State,
    slider_value: f32,
    toggle_value: bool,
}

#[derive(Debug, Clone)]
enum Message {
    InputChanged(String),
    ButtonPressed,
    SliderChanged(f32),
    CheckboxToggled(bool),
    ReloadTheme,
}

impl Application for Styling {
    type Executor = iced::executor::Default;
    type Message = Message;
    type Flags = Option<ReloadableTheme>;

    fn new(flags: Self::Flags) -> (Self, Command<Message>) {
        let theme = flags.unwrap();
        (
            Self {
                scroll: Default::default(),
                input: Default::default(),
                input_value: theme
                    .path()
                    .map(|p| p.to_string_lossy().into_owned())
                    .unwrap_or_default(),
                button: Default::default(),
                slider: Default::default(),
                slider_value: Default::default(),
                toggle_value: Default::default(),
                theme,
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("Styling")
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::InputChanged(value) => self.input_value = value,
            Message::ButtonPressed =>
            {
                #[cfg(not(target_family = "wasm"))]
                if let Err(e) = self.theme.set_path(&self.input_value) {
                    eprintln!("error: {e}");
                }
            }
            Message::SliderChanged(value) => self.slider_value = value,
            Message::CheckboxToggled(value) => self.toggle_value = value,
            Message::ReloadTheme => {
                if let Err(e) = self.theme.reload() {
                    eprintln!("error: {e}");
                }
            }
        }
        Command::none()
    }

    fn view(&mut self) -> Element<Message> {
        let text_input = self.theme.text_input().new(
            &mut self.input,
            "Type path to theme file...",
            &self.input_value,
            Message::InputChanged,
        );

        let button = self
            .theme
            .button()
            .new(&mut self.button, Text::new("Load theme"))
            .on_press(Message::ButtonPressed);

        let slider = self.theme.slider().new(
            &mut self.slider,
            0.0..=100.0,
            self.slider_value,
            Message::SliderChanged,
        );

        let progress_bar = self.theme.progress_bar().new(0.0..=100.0, self.slider_value);

        let scrollable = self
            .theme
            .scrollable()
            .new(&mut self.scroll)
            .push(Text::new("Scroll me!"))
            .push(Space::with_height(Length::Units(800)))
            .push(Text::new("You did it!"));

        let checkbox =
            self.theme.checkbox().new(self.toggle_value, "Toggle me!", Message::CheckboxToggled);

        let content = Column::new()
            .spacing(20)
            .padding(20)
            .max_width(600)
            .push(Row::new().spacing(10).push(Text::new("Theme")).push(text_input).push(button))
            .push(slider)
            .push(progress_bar)
            .push(
                Row::new()
                    .spacing(10)
                    .height(Length::Units(100))
                    .align_items(Alignment::Center)
                    .push(scrollable)
                    // .push(self.theme.rule().vertical(38))
                    .push(checkbox),
            );

        self.theme.container().new(content).into()
    }

    fn subscription(&self) -> Subscription<Self::Message> {
        self.theme.subscription().map(|_| Message::ReloadTheme)
    }
}
