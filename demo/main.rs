use grid::{Cell, Factory, Grid};
use iced::{
    Background, Color, Element, Length,
    advanced::widget::Text,
    run,
    widget::{Container, container::Style},
};
use itertools::Itertools;

struct State<'a> {
    grid: Grid<'a, Message>,
}

impl Default for State<'_> {
    fn default() -> Self {
        const DAYS_PER_WEEK: usize = 7;
        let today = 10;
        let grid = Grid::new()
            .with_row(["Sun", "Mon", "Tue", "Wed", "Thu", "Fri", "Sat"])
            .with_rows(
                (1..=31)
                    .map(move |day| {
                        Factory::from_factory(move || {
                            let red = day == today;
                            Cell::from(Text::new(day)).style(if red {
                                Style {
                                    background: Some(Background::Color(Color::from_rgb8(
                                        255, 0, 0,
                                    ))),
                                    text_color: Some(Color::from_rgb8(255, 255, 255)),
                                    ..Style::default()
                                }
                            } else {
                                Style {
                                    background: Some(Background::Color(Color::from_rgb8(
                                        255, 255, 255,
                                    ))),
                                    text_color: Some(Color::from_rgb8(0, 0, 0)),
                                    ..Style::default()
                                }
                            })
                        })
                    })
                    .chunks(DAYS_PER_WEEK)
                    .into_iter()
                    .map(Itertools::collect_vec)
                    .collect_vec(),
            )
            .cell_height(50)
            .cell_width(50)
            .padding(5);
        Self { grid }
    }
}

#[derive(Debug, Clone, Copy)]
enum Message {}

fn main() {
    run("grid", update, view).unwrap();
}

fn update(_state: &mut State, _message: Message) {}

fn view<'a>(state: &'a State) -> Element<'a, Message> {
    Container::new(&state.grid).center(Length::Fill).into()
}