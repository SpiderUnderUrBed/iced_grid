Here's updated documentation for your code with improved details and relevance to your latest implementation:

---

# `iced_grid`

This library provides a grid-based UI component built with the `iced` crate. It allows developers to create and manage grids with customizable rows and cells. Each cell can represent different types of content, including text, buttons, and custom containers.

## Installation

Add the `iced` crate as a dependency in your `Cargo.toml`:

```toml
[dependencies]
iced = "0.12"
iced_grid = { path = "<path-to-iced_grid>" }
```

## Features

- **Dynamic Grid Management**: Easily manage rows and cells in a grid structure.
- **Cell Types**: Support for text, buttons, and custom containers within cells.
- **User Interaction**: Cells can respond to interactions like clicks or edits.
- **Custom Styling**: Support for theming and styles via the `style::Catalog` trait.

---

## Components Overview

### `Grid`

The `Grid` struct is the main component that manages rows (`RowData`) and cells (`Cell`). It supports rendering, adding rows, and interacting with cells.

#### Methods

- **`new(rows, style, on_sync)`**: Creates a new grid instance.
  - `rows`: Initial grid data as a `Vec<RowData>`.
  - `style`: Custom style implementing the `Catalog` trait or `container::style`.
  - `on_sync`: A callback for synchronizing scrollable offsets.

- **`get_cell(row_index, cell_index)`**: Returns a mutable reference to a cell at the specified position, if it exists.
  
- **`get_row(row_index)`**: Retrieves a mutable reference to the specified row, creating new rows if needed.
  
- **`add_row(row)`**: Adds a new `RowData` to the grid.
  
- **`row_count()`**: Returns the total number of rows in the grid.
  
- **`view()`**: Returns an `iced::Element` for rendering the entire grid.

- **`create_grid()`**: Constructs the visual representation of the grid as an `iced::Column`.

- **`add_cells_to_row(row_index, count)`**  
  Appends a specified number of default cells to the target row.

- **`add_cells_to_all_rows(count)`**  
  Adds the same number of cells to all rows.

  - **`add_rows(count)`**  
  Adds multiple rows to the grid.


---

### `RowData`

`RowData` represents a single row within the grid and contains a list of `Cell`s.

#### Methods

- **`push_text(content)`**: Adds a text cell to the row.
  
- **`push_button(label, on_press)`**: Adds a button cell with a label and an action.
  
- **`push_container(factory)`**: Adds a custom container cell using a closure to generate the content.

- **`get_mut(index)`**: Returns a mutable reference to a cell at the specified index, if it exists.

- **`len()`**: Returns the number of cells in the row.

---

### `Cell`

`Cell` represents the smallest unit within the grid and can display text, a button, or custom content.

#### Variants

- **`Text(content)`**: Displays a string as plain text.
  
- **`Button { label, on_press }`**: Displays a clickable button with a label.
  
- **`Container(factory)`**: Uses a closure to render custom elements dynamically.

#### Methods

- **`view()`**: Returns the visual representation of the cell as an `iced::Element`.

---

### `CellMessage`

This enum represents possible actions related to a cell.

- **`Edit`**: Represents editing a cell.
- **`Remove`**: Represents removing a cell.
- **`Clicked`**: Represents a click action on a cell.

---

### `GridMessage`

This enum represents grid-level actions.

- **`AddCell(row_index)`**: Adds a new cell to the specified row.
- **`Cell(row_index, cell_index, CellMessage)`**: Handles a specific action (`CellMessage`) for a cell identified by its row and column index.

---

### `style::Catalog`

This trait defines custom styling options for the grid. The exact implementation depends on your application's requirements.

---

## Example Usage

Here’s how you can use the `iced_grid` library in an `iced::Sandbox` application.

### Creating a Grid

Refer to the demo 

---

## Contributing

Contributions are welcome! If you find any issues or have feature suggestions, please open an issue or submit a pull request.

---

This documentation incorporates the latest updates to your code and explains the new features comprehensively.
