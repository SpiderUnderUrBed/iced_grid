use iced::{
    widget::{Button, Column, Row, Text},
    Element, Sandbox, Settings,
    Theme,
    Renderer,
};

use std::sync::Arc;
use std::cell::RefCell;

#[derive(Debug, Clone)]
pub enum CellMessage {
    Edit,
    Remove,
    Clicked,
}

#[derive(Debug, Clone)]
pub enum GridMessage {
    AddCell(usize),
    Cell(usize, usize, CellMessage),
}

pub enum Cell<'a> {
    Text(String),
    Button {
        label: String,
        on_press: CellMessage,
    },
    Container(Option<Element<'a, CellMessage>>),
}
fn wrap_in_arc_refcell<'a>(
    element: &Option<Element<'a, CellMessage, Theme, Renderer>>,
) -> Option<Arc<RefCell<Element<'a, CellMessage, Theme, Renderer>>>> {
    element.as_ref().map(|el| {
        Arc::new(RefCell::new(**el))
    })
}

impl<'a> Clone for Cell<'a> {
    fn clone(&self) -> Self {
        match self {
            Cell::Text(content) => Cell::Text(content.clone()),
            Cell::Button { label, on_press } => Cell::Button {
                label: label.clone(),
                on_press: on_press.clone(),
            },
            Cell::Container(content) => {
                // Use the wrap_in_arc_refcell function to wrap the content if it's Some
                let cloned_content = wrap_in_arc_refcell(content);
                Cell::Container(cloned_content)
            }
        }
    }
}

impl<'a> Cell<'a> {
    /// View the cell content as an `Element`
    pub fn view(&self) -> Element<CellMessage> {
        match self {
            Cell::Text(content) => Text::new(content.clone()).into(),
            Cell::Button { label, on_press } => {
                Button::new(Text::new(label.clone()))
                    .on_press(on_press.clone())
                    .into()
            }
            Cell::Container(Some(content)) => content.to_owned(),
            Cell::Container(None) => Text::new("Empty Container").into(),
        }
    }
    pub fn into_arc_refcell(self) -> Arc<RefCell<Self>> {
        Arc::new(RefCell::new(self))
    }
}


#[derive(Default, Clone)]
pub struct RowData {
    cells: Vec<Cell<'static>>,
}

impl RowData {
    pub fn push_text(&mut self, content: String) {
        self.cells.push(Cell::Text(content));
    }

    pub fn push_button(&mut self, label: String, on_press: CellMessage) {
        self.cells.push(Cell::Button { label, on_press });
    }

    pub fn push_container(&mut self, content: Option<Element<'static, CellMessage>>) {
        self.cells.push(Cell::Container(content));
    }

    pub fn get_mut(&mut self, index: usize) -> Option<&mut Cell<'static>> {
        self.cells.get_mut(index)
    }

    /// Wrap a specific cell in `Arc<RefCell>`
    pub fn wrap_cell(&mut self, index: usize) -> Option<Arc<RefCell<Cell<'static>>>> {
        self.cells
            .get_mut(index)
            .map(|cell| std::mem::replace(cell, Cell::Text(String::new())).into_arc_refcell())

    }
}

pub struct Grid {
    rows: Vec<RowData>,
}

impl Grid {
    pub fn new() -> Self {
        Self { rows: Vec::new() }
    }

    pub fn get_cell(&mut self, row_index: usize, cell_index: usize) -> Option<&mut Cell<'static>> {
        self.rows.get_mut(row_index).and_then(|row| row.get_mut(cell_index))
    }

    pub fn get_row(&mut self, row: usize) -> &mut RowData {
        if self.rows.len() <= row {
            self.rows.resize_with(row + 1, RowData::default);
        }
        &mut self.rows[row]
    }

    pub fn row_count(&self) -> usize {
        self.rows.len()
    }

    pub fn add_row(&mut self) {
        self.rows.push(RowData::default());
    }

    pub fn create_grid<'a>(&'a self) -> Column<'a, GridMessage> {
        let mut column = Column::new().spacing(10);

        for (row_index, row) in self.rows.iter().enumerate() {
            let mut row_view = Row::new().spacing(10);
            for (cell_index, cell) in row.cells.iter().enumerate() {
                let cell_view: Element<GridMessage> = cell.view().map(move |cell_msg| {
                    GridMessage::Cell(row_index, cell_index, cell_msg)
                });
                row_view = row_view.push(cell_view);
            }
            column = column.push(row_view);
        }

        column
    }

    pub fn view<'a>(&'a self) -> iced::Element<'a, GridMessage> {
        self.create_grid().into()
    }
}
