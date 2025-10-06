use iced::{
    widget::{button, column, container, row, text, pick_list, scrollable, text_input},
    alignment::Alignment,
    executor, Application, Command, Element, Length, Settings, Theme, Color,
};
use std::f64::consts::PI;

pub fn main() -> iced::Result {
    Calculator::run(Settings {
        window: iced::window::Settings {
            size: (600, 800),
            ..Default::default()
        },
        ..Default::default()
    })
}

#[derive(Debug, Clone)]
enum Message {
    ButtonPressed(String),
    Clear,
    Calculate,
    Delete,
    ToggleScientific,
    TogglePhysics,
    ToggleEconomics,
    ToggleThemeEditor,
    ChangeTheme(ThemePreset),
    UpdateColor(ColorTarget, ColorChannel, String),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ThemePreset {
    Dark,
    Light,
    Dracula,
    Nord,
    Monokai,
    Solarized,
    Custom,
}

impl ThemePreset {
    const ALL: [ThemePreset; 7] = [
        ThemePreset::Dark,
        ThemePreset::Light,
        ThemePreset::Dracula,
        ThemePreset::Nord,
        ThemePreset::Monokai,
        ThemePreset::Solarized,
        ThemePreset::Custom,
    ];
}

impl std::fmt::Display for ThemePreset {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                ThemePreset::Dark => "Ciemny",
                ThemePreset::Light => "Jasny",
                ThemePreset::Dracula => "Dracula",
                ThemePreset::Nord => "Nord",
                ThemePreset::Monokai => "Monokai",
                ThemePreset::Solarized => "Solarized",
                ThemePreset::Custom => "Własny",
            }
        )
    }
}

#[derive(Debug, Clone, Copy)]
enum ColorTarget {
    Background,
    Display,
    Number,
    Operator,
    Function,
    DisplayText,
}

#[derive(Debug, Clone, Copy)]
enum ColorChannel {
    Red,
    Green,
    Blue,
}

#[derive(Debug, Clone, Copy)]
struct CalcTheme {
    background: Color,
    display: Color,
    display_text: Color,
    number_btn: Color,
    operator_btn: Color,
    function_btn: Color,
}

impl CalcTheme {
    fn dark() -> Self {
        Self {
            background: Color::from_rgb(0.1, 0.1, 0.12),
            display: Color::from_rgb(0.15, 0.15, 0.18),
            display_text: Color::from_rgb(0.9, 0.9, 0.95),
            number_btn: Color::from_rgb(0.2, 0.2, 0.25),
            operator_btn: Color::from_rgb(0.3, 0.5, 0.7),
            function_btn: Color::from_rgb(0.5, 0.3, 0.7),
        }
    }

    fn light() -> Self {
        Self {
            background: Color::from_rgb(0.95, 0.95, 0.97),
            display: Color::from_rgb(0.98, 0.98, 1.0),
            display_text: Color::from_rgb(0.1, 0.1, 0.15),
            number_btn: Color::from_rgb(0.9, 0.9, 0.92),
            operator_btn: Color::from_rgb(0.4, 0.6, 0.85),
            function_btn: Color::from_rgb(0.7, 0.4, 0.85),
        }
    }

    fn dracula() -> Self {
        Self {
            background: Color::from_rgb(0.16, 0.16, 0.21),
            display: Color::from_rgb(0.2, 0.2, 0.27),
            display_text: Color::from_rgb(0.95, 0.95, 1.0),
            number_btn: Color::from_rgb(0.27, 0.27, 0.35),
            operator_btn: Color::from_rgb(1.0, 0.47, 0.78),
            function_btn: Color::from_rgb(0.74, 0.58, 0.98),
        }
    }

    fn nord() -> Self {
        Self {
            background: Color::from_rgb(0.18, 0.2, 0.25),
            display: Color::from_rgb(0.23, 0.26, 0.32),
            display_text: Color::from_rgb(0.92, 0.94, 0.96),
            number_btn: Color::from_rgb(0.3, 0.34, 0.42),
            operator_btn: Color::from_rgb(0.53, 0.75, 0.82),
            function_btn: Color::from_rgb(0.7, 0.56, 0.68),
        }
    }

    fn monokai() -> Self {
        Self {
            background: Color::from_rgb(0.16, 0.16, 0.14),
            display: Color::from_rgb(0.2, 0.2, 0.18),
            display_text: Color::from_rgb(0.97, 0.97, 0.95),
            number_btn: Color::from_rgb(0.25, 0.25, 0.22),
            operator_btn: Color::from_rgb(0.98, 0.47, 0.36),
            function_btn: Color::from_rgb(0.64, 0.86, 0.29),
        }
    }

    fn solarized() -> Self {
        Self {
            background: Color::from_rgb(0.0, 0.17, 0.21),
            display: Color::from_rgb(0.03, 0.21, 0.26),
            display_text: Color::from_rgb(0.51, 0.58, 0.59),
            number_btn: Color::from_rgb(0.07, 0.26, 0.31),
            operator_btn: Color::from_rgb(0.15, 0.55, 0.82),
            function_btn: Color::from_rgb(0.83, 0.21, 0.51),
        }
    }
}

struct Calculator {
    display: String,
    current_op: Option<String>,
    previous_value: Option<f64>,
    show_scientific: bool,
    show_physics: bool,
    show_economics: bool,
    show_theme_editor: bool,
    theme: CalcTheme,
    current_preset: ThemePreset,
}

impl Application for Calculator {
    type Executor = executor::Default;
    type Message = Message;
    type Theme = Theme;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<Message>) {
        (
            Self {
                display: String::from("0"),
                current_op: None,
                previous_value: None,
                show_scientific: false,
                show_physics: false,
                show_economics: false,
                show_theme_editor: false,
                theme: CalcTheme::dark(),
                current_preset: ThemePreset::Dark,
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("Kalkulator Pro | Rust Edition")
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::ButtonPressed(value) => {
                if self.display == "0" || self.display == "Error" {
                    self.display = value;
                } else {
                    self.display.push_str(&value);
                }
            }
            Message::Clear => {
                self.display = String::from("0");
                self.current_op = None;
                self.previous_value = None;
            }
            Message::Delete => {
                if self.display.len() > 1 {
                    self.display.pop();
                } else {
                    self.display = String::from("0");
                }
            }
            Message::Calculate => {
                if let Ok(result) = self.evaluate(&self.display) {
                    self.display = format!("{}", result);
                } else {
                    self.display = String::from("Error");
                }
            }
            Message::ToggleScientific => {
                self.show_scientific = !self.show_scientific;
            }
            Message::TogglePhysics => {
                self.show_physics = !self.show_physics;
            }
            Message::ToggleEconomics => {
                self.show_economics = !self.show_economics;
            }
            Message::ToggleThemeEditor => {
                self.show_theme_editor = !self.show_theme_editor;
            }
            Message::ChangeTheme(preset) => {
                self.current_preset = preset;
                self.theme = match preset {
                    ThemePreset::Dark => CalcTheme::dark(),
                    ThemePreset::Light => CalcTheme::light(),
                    ThemePreset::Dracula => CalcTheme::dracula(),
                    ThemePreset::Nord => CalcTheme::nord(),
                    ThemePreset::Monokai => CalcTheme::monokai(),
                    ThemePreset::Solarized => CalcTheme::solarized(),
                    ThemePreset::Custom => self.theme,
                };
            }
            Message::UpdateColor(target, channel, value) => {
                if let Ok(val) = value.parse::<f32>() {
                    let val = (val / 255.0).clamp(0.0, 1.0);
                    let color = match target {
                        ColorTarget::Background => &mut self.theme.background,
                        ColorTarget::Display => &mut self.theme.display,
                        ColorTarget::DisplayText => &mut self.theme.display_text,
                        ColorTarget::Number => &mut self.theme.number_btn,
                        ColorTarget::Operator => &mut self.theme.operator_btn,
                        ColorTarget::Function => &mut self.theme.function_btn,
                    };
                    match channel {
                        ColorChannel::Red => color.r = val,
                        ColorChannel::Green => color.g = val,
                        ColorChannel::Blue => color.b = val,
                    }
                    self.current_preset = ThemePreset::Custom;
                }
            }
        }
        Command::none()
    }

    fn view(&self) -> Element<Message> {
        let theme = self.theme;
        
        let display = container(
            text(&self.display)
                .size(48)
                .style(theme.display_text)
        )
        .padding(20)
        .width(Length::Fill)
        .center_x()
        .style(iced::theme::Container::Custom(Box::new(DisplayStyle(theme.display))));

        let mut content = column![display].spacing(10).padding(10);

        // Theme selector
        let theme_row = row![
            text("Motyw:").size(16),
            pick_list(
                &ThemePreset::ALL[..],
                Some(self.current_preset),
                Message::ChangeTheme
            ),
            button(text("Edytor").size(14))
                .on_press(Message::ToggleThemeEditor)
                .style(iced::theme::Button::Custom(Box::new(ButtonStyle(theme.function_btn))))
        ]
        .spacing(10)
        .align_items(Alignment::Center);

        content = content.push(theme_row);

        // Theme editor
        if self.show_theme_editor {
            let editor = self.create_theme_editor();
            content = content.push(editor);
        }

        // Basic number pad
        let numpad = self.create_numpad();
        content = content.push(numpad);

        // Scientific functions
        if self.show_scientific {
            let scientific = self.create_scientific_panel();
            content = content.push(scientific);
        }

        // Physics formulas
        if self.show_physics {
            let physics = self.create_physics_panel();
            content = content.push(scrollable(physics).height(Length::Fixed(200.0)));
        }

        // Economics formulas
        if self.show_economics {
            let economics = self.create_economics_panel();
            content = content.push(scrollable(economics).height(Length::Fixed(200.0)));
        }

        // Toggle buttons
        let toggles = row![
            button(text(if self.show_scientific { "🔬 Ukryj" } else { "🔬 Funkcje" }).size(14))
                .on_press(Message::ToggleScientific)
                .style(iced::theme::Button::Custom(Box::new(ButtonStyle(theme.function_btn)))),
            button(text(if self.show_physics { "⚛️ Ukryj" } else { "⚛️ Fizyka" }).size(14))
                .on_press(Message::TogglePhysics)
                .style(iced::theme::Button::Custom(Box::new(ButtonStyle(theme.function_btn)))),
            button(text(if self.show_economics { "💰 Ukryj" } else { "💰 Ekonomia" }).size(14))
                .on_press(Message::ToggleEconomics)
                .style(iced::theme::Button::Custom(Box::new(ButtonStyle(theme.function_btn)))),
        ]
        .spacing(5);

        content = content.push(toggles);

        container(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .padding(10)
            .style(iced::theme::Container::Custom(Box::new(DisplayStyle(theme.background))))
            .into()
    }

    fn theme(&self) -> Theme {
        Theme::custom(iced::theme::Palette {
            background: self.theme.background,
            text: self.theme.display_text,
            primary: self.theme.operator_btn,
            success: Color::from_rgb(0.3, 0.8, 0.3),
            danger: Color::from_rgb(0.8, 0.3, 0.3),
        })
    }
}

// Custom styles
struct DisplayStyle(Color);

impl container::StyleSheet for DisplayStyle {
    type Style = Theme;

    fn appearance(&self, _style: &Self::Style) -> container::Appearance {
        container::Appearance {
            background: Some(self.0.into()),
            border_radius: 15.0.into(),
            ..Default::default()
        }
    }
}

struct ButtonStyle(Color);

impl button::StyleSheet for ButtonStyle {
    type Style = Theme;

    fn active(&self, _style: &Self::Style) -> button::Appearance {
        button::Appearance {
            background: Some(self.0.into()),
            border_radius: 10.0.into(),
            text_color: Color::WHITE,
            ..Default::default()
        }
    }

    fn hovered(&self, _style: &Self::Style) -> button::Appearance {
        button::Appearance {
            background: Some(Color {
                r: (self.0.r * 1.2).min(1.0),
                g: (self.0.g * 1.2).min(1.0),
                b: (self.0.b * 1.2).min(1.0),
                a: self.0.a,
            }.into()),
            border_radius: 10.0.into(),
            text_color: Color::WHITE,
            ..Default::default()
        }
    }

    fn pressed(&self, _style: &Self::Style) -> button::Appearance {
        button::Appearance {
            background: Some(Color {
                r: self.0.r * 0.8,
                g: self.0.g * 0.8,
                b: self.0.b * 0.8,
                a: self.0.a,
            }.into()),
            border_radius: 10.0.into(),
            text_color: Color::WHITE,
            ..Default::default()
        }
    }
}

impl Calculator {
    fn create_numpad(&self) -> Element<Message> {
        let theme = self.theme;
        
        let row1 = row![
            self.calc_button("7", theme.number_btn),
            self.calc_button("8", theme.number_btn),
            self.calc_button("9", theme.number_btn),
            self.calc_button("/", theme.operator_btn),
            button(text("⌫").size(20))
                .on_press(Message::Delete)
                .padding(15)
                .style(iced::theme::Button::Custom(Box::new(ButtonStyle(theme.operator_btn)))),
        ]
        .spacing(5);

        let row2 = row![
            self.calc_button("4", theme.number_btn),
            self.calc_button("5", theme.number_btn),
            self.calc_button("6", theme.number_btn),
            self.calc_button("*", theme.operator_btn),
            button(text("C").size(20))
                .on_press(Message::Clear)
                .padding(15)
                .style(iced::theme::Button::Custom(Box::new(ButtonStyle(Color::from_rgb(0.8, 0.3, 0.3))))),
        ]
        .spacing(5);

        let row3 = row![
            self.calc_button("1", theme.number_btn),
            self.calc_button("2", theme.number_btn),
            self.calc_button("3", theme.number_btn),
            self.calc_button("-", theme.operator_btn),
            self.calc_button("^", theme.operator_btn),
        ]
        .spacing(5);

        let row4 = row![
            self.calc_button("0", theme.number_btn),
            self.calc_button(".", theme.number_btn),
            self.calc_button("π", theme.function_btn),
            self.calc_button("+", theme.operator_btn),
            button(text("=").size(20))
                .on_press(Message::Calculate)
                .padding(15)
                .style(iced::theme::Button::Custom(Box::new(ButtonStyle(Color::from_rgb(0.3, 0.8, 0.5))))),
        ]
        .spacing(5);

        column![row1, row2, row3, row4].spacing(5).into()
    }

    fn create_scientific_panel(&self) -> Element<Message> {
        let theme = self.theme;
        
        let row1 = row![
            self.calc_button("sin(", theme.function_btn),
            self.calc_button("cos(", theme.function_btn),
            self.calc_button("tan(", theme.function_btn),
            self.calc_button("√(", theme.function_btn),
        ]
        .spacing(5);

        let row2 = row![
            self.calc_button("log(", theme.function_btn),
            self.calc_button("ln(", theme.function_btn),
            self.calc_button("(", theme.operator_btn),
            self.calc_button(")", theme.operator_btn),
        ]
        .spacing(5);

        column![row1, row2].spacing(5).into()
    }

    fn create_physics_panel(&self) -> Element<Message> {
        let theme = self.theme;
        
        column![
            text("⚛️ WZORY FIZYCZNE").size(18).style(theme.display_text),
            self.formula_button("E=mc² (energia)", "299792458^2*"),
            self.formula_button("v=s/t (prędkość)", "/"),
            self.formula_button("a=Δv/t (przyśp.)", "/"),
            self.formula_button("F=ma (II zasada)", "*"),
            self.formula_button("Ek=½mv² (en.kin.)", "0.5**"),
            self.formula_button("Ep=mgh (en.pot.)", "**9.81*"),
            self.formula_button("P=W/t (moc)", "/"),
            self.formula_button("p=mv (pęd)", "*"),
        ]
        .spacing(5)
        .into()
    }

    fn create_economics_panel(&self) -> Element<Message> {
        let theme = self.theme;
        
        column![
            text("💰 WZORY EKONOMICZNE").size(18).style(theme.display_text),
            self.formula_button("Odsetki: I=P*r*t", "**"),
            self.formula_button("ROI: (gain-cost)/cost", "(-)/"),
            self.formula_button("VAT: cena*1.23", "*1.23"),
            self.formula_button("Marża: (cena-koszt)/cena", "(-)/"),
            self.formula_button("Narzut: (cena-koszt)/koszt", "(-)/"),
            self.formula_button("Deprecjacja liniowa", "/"),
        ]
        .spacing(5)
        .into()
    }

    fn create_theme_editor(&self) -> Element<Message> {
        let theme = self.theme;
        
        let bg_editor = self.color_editor("Tło", ColorTarget::Background, theme.background);
        let disp_editor = self.color_editor("Wyświetlacz", ColorTarget::Display, theme.display);
        let text_editor = self.color_editor("Tekst", ColorTarget::DisplayText, theme.display_text);
        let num_editor = self.color_editor("Przyciski cyfr", ColorTarget::Number, theme.number_btn);
        let op_editor = self.color_editor("Operatory", ColorTarget::Operator, theme.operator_btn);
        let fn_editor = self.color_editor("Funkcje", ColorTarget::Function, theme.function_btn);

        container(
            scrollable(
                column![
                    text("🎨 EDYTOR MOTYWU").size(20),
                    bg_editor,
                    disp_editor,
                    text_editor,
                    num_editor,
                    op_editor,
                    fn_editor,
                ]
                .spacing(10)
            )
            .height(Length::Fixed(250.0))
        )
        .padding(10)
        .style(iced::theme::Container::Custom(Box::new(EditorStyle(theme))))
        .into()
    }

    fn color_editor(&self, label: &str, target: ColorTarget, color: Color) -> Element<Message> {
        let r_val = (color.r * 255.0) as u8;
        let g_val = (color.g * 255.0) as u8;
        let b_val = (color.b * 255.0) as u8;

        column![
            text(label).size(14),
            row![
                text("R:").size(12),
                text_input(&r_val.to_string(), &r_val.to_string())
                    .on_input(move |v| Message::UpdateColor(target, ColorChannel::Red, v))
                    .width(Length::Fixed(50.0)),
                text("G:").size(12),
                text_input(&g_val.to_string(), &g_val.to_string())
                    .on_input(move |v| Message::UpdateColor(target, ColorChannel::Green, v))
                    .width(Length::Fixed(50.0)),
                text("B:").size(12),
                text_input(&b_val.to_string(), &b_val.to_string())
                    .on_input(move |v| Message::UpdateColor(target, ColorChannel::Blue, v))
                    .width(Length::Fixed(50.0)),
                container(text(""))
                    .width(Length::Fixed(30.0))
                    .height(Length::Fixed(30.0))
                    .style(iced::theme::Container::Custom(Box::new(ColorPreview(color))))
            ]
            .spacing(5)
            .align_items(Alignment::Center)
        ]
        .spacing(5)
        .into()
    }

    fn calc_button(&self, label: &str, color: Color) -> Element<Message> {
        button(text(label).size(20))
            .on_press(Message::ButtonPressed(label.to_string()))
            .padding(15)
            .style(iced::theme::Button::Custom(Box::new(ButtonStyle(color))))
            .into()
    }

    fn formula_button(&self, label: &str, formula: &str) -> Element<Message> {
        let theme = self.theme;
        button(text(label).size(14))
            .on_press(Message::ButtonPressed(formula.to_string()))
            .padding(10)
            .width(Length::Fill)
            .style(iced::theme::Button::Custom(Box::new(ButtonStyle(theme.function_btn))))
            .into()
    }

    fn evaluate(&self, expr: &str) -> Result<f64, String> {
        let expr = expr
            .replace("π", &PI.to_string())
            .replace("√", "sqrt");

        match meval::eval_str(&expr) {
            Ok(result) => Ok(result),
            Err(_) => Err("Error".to_string()),
        }
    }
}

struct EditorStyle(CalcTheme);

impl container::StyleSheet for EditorStyle {
    type Style = Theme;

    fn appearance(&self, _style: &Self::Style) -> container::Appearance {
        container::Appearance {
            background: Some(Color::from_rgba(self.0.display.r, self.0.display.g, self.0.display.b, 0.5).into()),
            border_radius: 10.0.into(),
            border_width: 1.0,
            border_color: self.0.operator_btn,
            ..Default::default()
        }
    }
}

struct ColorPreview(Color);

impl container::StyleSheet for ColorPreview {
    type Style = Theme;

    fn appearance(&self, _style: &Self::Style) -> container::Appearance {
        container::Appearance {
            background: Some(self.0.into()),
            border_radius: 5.0.into(),
            border_width: 1.0,
            border_color: Color::WHITE,
            ..Default::default()
        }
    }
}