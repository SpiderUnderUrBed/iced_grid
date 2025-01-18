# `grid` - A Grid Wrapper for `iced`

This crate provides a thin wrapper around the `iced` library's `Column` widget, making it easier to create and manage a grid of cells. The grid is composed of cells that hold elements and their associated styles. You can customize the grid's cell dimensions, gutter spacing, and padding, as well as the individual cell styles.

## Modules

### `Cell`

The `Cell` struct represents an individual cell in the grid. It stores an `Element` (the content of the cell) and a `Style` (the style of the cell).

#### Methods

- **`From<E> for Cell<'a, M, T, R>`**: Converts an `Element` into a `Cell`. The style of the `Cell` is set to the default style.
  
- **`style`**: Allows you to set a custom style for the cell.

#### Example

```rust
let cell = Cell::from(Text::new("Hello"));
let styled_cell = cell.style(Style::default());
```

---

### `Factory`

The `Factory` struct is used to create grid cells. You can configure it to generate cells with a default or custom style.

#### Methods

- **`from_element`**: Creates a factory from an `Element`, using the default style for the cell.
- **`from_element_and_style`**: Creates a factory from an `Element` and a specific style.
- **`from_factory`**: Creates a factory from a provided function that generates a `Cell`.

#### Example

```rust
let factory = Factory::from_element_and_style(Text::new("Custom"), Style::default());
```

---

### `Grid`

The `Grid` struct represents a grid of cells. It supports multiple rows and provides methods to adjust the cell dimensions, gutter size, and padding. It can also be converted into an `Element` for use in an `iced` GUI.

#### Methods

- **`new`**: Creates a new empty grid.
- **`with_row`**: Adds a single row to the grid.
- **`with_rows`**: Adds multiple rows to the grid.
- **`cell_width`**: Sets the width of each cell.
- **`cell_height`**: Sets the height of each cell.
- **`gutter`**: Sets the gutter (spacing) between cells.
- **`padding`**: Sets the padding around the entire grid.

#### Example

```rust
let grid = Grid::new()
    .with_row([Factory::from_element(Text::new("A")), Factory::from_element(Text::new("B"))])
    .cell_width(100)
    .cell_height(100)
    .gutter(10)
    .padding(5);
```

---

## Demo

This is an example of how to use the grid system to create a simple calendar layout where the current day is highlighted in red:

```rust
use grid::{Cell, Factory, Grid};
use iced::{Background, Color, Element, Length, advanced::widget::Text, run, widget::{Container, container::Style}};
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
                                    background: Some(Background::Color(Color::from_rgb8(255, 0, 0))),
                                    text_color: Some(Color::from_rgb8(255, 255, 255)),
                                    ..Style::default()
                                }
                            } else {
                                Style {
                                    background: Some(Background::Color(Color::from_rgb8(255, 255, 255))),
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
```

This example creates a grid with the days of the week and adds cells representing the dates of the month. The current day is highlighted in red with a white text color.

