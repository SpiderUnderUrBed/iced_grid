

# Iced_grid
This library provides a simple grid-based UI component built with the `iced` crate. It includes functionality for managing rows and cells within a grid, with different cell types like text and buttons.

## Installation

To use this library in your project, add it to your `Cargo.toml` as a dependency:

```toml
[dependencies]
iced = "0.12"
my_grid_app = { path = "path/to/your/library" }
```

Ensure that the `iced` crate is added as a dependency in your `Cargo.toml` if you don't already have it.

## Features

- **Grid Management**: Manage rows and cells, with functionality to add new rows and cells.
- **Cell Configuration**: Cells can display text or act as buttons.
- **Interaction**: Cells support interaction, such as clicks and edits.

## Key Components

### `Grid`

The `Grid` struct manages the overall structure of the grid, containing multiple rows. Each row is an instance of `RowData`.

#### Methods

- **`new()`**: Creates a new, empty grid.
- **`get_cell(row_index, cell_index)`**: Retrieves a mutable reference to a cell at the specified position.
- **`get_row(row_index)`**: Retrieves a mutable reference to a row at the specified position, creating new rows if necessary.
- **`row_count()`**: Returns the number of rows in the grid.
- **`add_row()`**: Adds a new row to the grid.
- **`view()`**: Returns the view of the grid, rendering the cells and rows, with an "Add Cell" button for each row.

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
        grid.get_row(0).push(CellConfig::Text("Hello, Cell!".to_string()));

        Self { grid }
    }

    fn title(&self) -> String {
        String::from("My Grid Application")
    }

    fn update(&mut self, message: GridMessage) {
        match message {
            GridMessage::AddCell(row_index) => {
                self.grid
                    .get_row(row_index)
                    .push(CellConfig::Text("New Cell".to_string()));
            }
            GridMessage::Cell(row_index, cell_index, CellMessage::Edit) => {
                if let Some(cell) = self.grid.get_cell(row_index, cell_index) {
                    cell.edit(CellConfig::Text("Edited!".to_string()));
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

`RowData` represents a single row in the grid, containing multiple cells.

#### Methods

- **`push(config: CellConfig)`**: Adds a new cell with the specified configuration to the row.
- **`get_mut(index: usize)`**: Retrieves a mutable reference to a cell at the specified index.

### `Cell`

`Cell` represents a single cell in a row. Each cell can be configured to display text or act as a button.

#### Methods

- **`new(config: CellConfig)`**: Creates a new cell with the given configuration.
- **`edit(new_config: CellConfig)`**: Edits the configuration of the cell.
- **`remove()`**: Clears the content of the cell (sets it to an empty `Text`).
- **`view()`**: Returns the view of the cell, which is either a text element or a button.

### `CellConfig`

`CellConfig` defines the possible configurations of a cell. A cell can either display text or act as a button.

#### Variants

- **`Text(String)`**: A text cell, displaying the specified string.
- **`Button(String)`**: A button cell, displaying the specified label.

### `CellMessage`

`CellMessage` represents the possible actions for a cell.

#### Variants

- **`Edit`**: Indicates that the cell is being edited.
- **`Remove`**: Indicates that the cell should be removed or cleared.
- **`Clicked`**: Indicates that the cell was clicked.

### `GridMessage`

`GridMessage` represents the possible messages for interacting with the grid.

#### Variants

- **`AddCell(usize)`**: A message to add a new cell to the specified row.
- **`Cell(usize, usize, CellMessage)`**: A message to interact with a specific cell, identified by its row and cell index, and a `CellMessage` (such as edit, remove, or click).

## Examples

### Adding a New Row

To add a new row to the grid, simply call the `add_row()` method on the `Grid` instance:

```rust
grid.add_row();
```

This will create a new row with no cells. You can then add cells to it.

### Adding a New Cell

To add a new cell to a specific row, use the `get_row(row_index)` method and call `push()` to add a cell:

```rust
grid.get_row(0).push(CellConfig::Text("New Cell".to_string()));
```

### Handling Cell Clicks

Each cell can be configured to act as a button, and when clicked, a `GridMessage::Cell` message is sent. To handle clicks, you can implement logic in the `update` method of your `Sandbox` implementation:

```rust
GridMessage::Cell(row_index, cell_index, CellMessage::Clicked) => {
    // Handle cell click
}
```

## Contributing

Contributions to this library are welcome! If you encounter any bugs or have feature requests, feel free to open an issue or submit a pull request.

