# Iced_grid

This library provides a flexible grid-based UI component built with the `iced` crate. It offers functionality for managing rows and cells, supporting various cell types like text, buttons, and dynamic containers. It allows for easy interaction with the grid, including cell edits, removals, and click events.

## Installation

To use this library in your project, add it to your `Cargo.toml` as a dependency:

```toml
[dependencies]
iced = "0.12"
iced_grid = { path = "<location to iced grid>" }
```

Make sure the `iced` crate is included as a dependency in your `Cargo.toml` if it's not already added.

## Features

- **Grid Management**: Manage rows and cells, including functionality to add new rows and cells dynamically.
- **Cell Flexibility**: Cells can display text, act as buttons, or contain more complex widgets (via containers).
- **Cell Interaction**: Supports actions like clicks, edits, and removals for each individual cell.
- **Dynamic Cell Containers**: Cells can be populated dynamically through closures, allowing for flexible content rendering.

## Key Components

### `Grid`

The `Grid` struct represents the entire grid and holds multiple rows. Each row is an instance of `RowData`.

#### Methods

- **`new()`**: Creates a new, empty grid.
- **`get_cell(row_index, cell_index)`**: Retrieves a mutable reference to a cell at the specified position.
- **`get_row(row_index)`**: Retrieves a mutable reference to a row at the specified position. New rows are automatically created if necessary.
- **`row_count()`**: Returns the number of rows in the grid.
- **`add_row()`**: Adds a new row to the grid.
- **`view()`**: Returns the grid’s view, rendering the cells and rows.

#### Example Usage

```rust
use iced::{Sandbox, Settings, Text};
use my_grid_app::{Grid, GridMessage};

pub fn main() -> iced::Result {
    MyApp::run(Settings::default())
}

struct MyApp {
    grid: Grid,
}

impl Sandbox for MyApp {
    type Message = GridMessage;

    fn new() -> Self {
        let mut grid = Grid::new();
        grid.add_row();
        grid.get_row(0).push_text("Hello, Cell!".to_string());

        Self { grid }
    }

    fn title(&self) -> String {
        String::from("My Grid Application")
    }

    fn update(&mut self, message: GridMessage) {
        match message {
            GridMessage::AddCell(row_index) => {
                self.grid.get_row(row_index).push_text("New Cell".to_string());
            }
            GridMessage::Cell(row_index, cell_index, CellMessage::Edit) => {
                if let Some(cell) = self.grid.get_cell(row_index, cell_index) {
                    cell.edit_text("Edited!".to_string());
                }
            }
            GridMessage::Cell(row_index, cell_index, CellMessage::Remove) => {
                if let Some(cell) = self.grid.get_cell(row_index, cell_index) {
                    cell.remove();
                }
            }
            GridMessage::Cell(row_index, cell_index, CellMessage::Clicked) => {
                // Handle click event
            }
        }
    }

    fn view(&self) -> iced::Element<Self::Message> {
        self.grid.view()
    }
}
```

### `RowData`

`RowData` represents a single row in the grid and contains a collection of cells.

#### Methods

- **`push_text(content: String)`**: Adds a new text cell to the row.
- **`push_button(label: String, on_press: CellMessage)`**: Adds a new button cell to the row with specified label and press action.
- **`push_container<F>(factory: F)`**: Adds a container cell to the row, where `F` is a closure that generates an element dynamically.
- **`get_mut(index: usize)`**: Retrieves a mutable reference to a cell at the specified index.

### `Cell`

`Cell` represents a single cell within a row. Cells can be of various types, such as text, buttons, or containers.

#### Methods

- **`view()`**: Returns the view of the cell, which is either a text element or a button, or a dynamic container generated via a closure.
  
### `CellMessage`

`CellMessage` represents the possible actions that can be performed on a cell.

#### Variants

- **`Edit`**: Indicates that the cell is being edited.
- **`Remove`**: Indicates that the cell should be removed or cleared.
- **`Clicked`**: Indicates that the cell was clicked.

### `GridMessage`

`GridMessage` represents the possible messages for interacting with the grid and its cells.

#### Variants

- **`AddCell(usize)`**: A message to add a new cell to the specified row.
- **`Cell(usize, usize, CellMessage)`**: A message to interact with a specific cell, identified by its row and cell index, and a `CellMessage` (such as edit, remove, or click).

## Examples

### Adding a New Row

To add a new row to the grid, use the `add_row()` method:

```rust
grid.add_row();
```

This will create a new row without cells. You can then add cells to the row using methods like `push_text()` or `push_button()`.

### Adding a New Cell

To add a new cell to a specific row, call `get_row(row_index)` and use methods like `push_text()` or `push_button()` to insert cells:

```rust
grid.get_row(0).push_text("New Cell".to_string());
```

### Handling Cell Clicks

Each cell can be configured to act as a button. When clicked, a `GridMessage::Cell` message is generated. You can handle clicks by processing the `GridMessage::Cell` in the `update()` method:

```rust
GridMessage::Cell(row_index, cell_index, CellMessage::Clicked) => {
    // Handle cell click
}
```

## Contributing

Contributions to this library are welcome! If you encounter any issues or have feature requests, feel free to open an issue or submit a pull request.