use iced::{
    Theme,
    widget::{Button, Column, Row, Text, container},
    Element, Sandbox, Settings,
};

// use style::Catalog;
mod style;
//use style::Catalog;
use crate::style::Catalog;

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
    Container(Arc<RefCell<dyn Fn() -> Element<'a, CellMessage> + 'a>>),
}

impl<'a> Clone for Cell<'a> {
    fn clone(&self) -> Self {
        match self {
            Cell::Text(content) => Cell::Text(content.clone()),
            Cell::Button { label, on_press } => Cell::Button {
                label: label.clone(),
                on_press: on_press.clone(),
            },
            Cell::Container(container) => {
                Cell::Container(Arc::clone(container))
            }
        }
    }
}

impl Cell<'_> {
    pub fn view(&self) -> Element<CellMessage> {
        match self {
            Cell::Text(content) => Text::new(content.clone()).into(),
            Cell::Button { label, on_press } => {
                Button::new(Text::new(label.clone()))
                    .on_press(on_press.clone())
                    .into()
            }
            Cell::Container(factory) => (factory.borrow())(),
        }
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

    pub fn push_container<F>(&mut self, factory: F)
    where
        F: Fn() -> Element<'static, CellMessage> + 'static,
    {
        self.cells.push(Cell::Container(Arc::new(RefCell::new(factory))));
    }

    pub fn get_mut(&mut self, index: usize) -> Option<&mut Cell<'static>> {
        self.cells.get_mut(index)
    }
}

pub struct Grid
 where
  Theme: Catalog,
 {
    rows: Vec<RowData>,
    style: <Theme as Catalog>::Style,
}

impl Grid {
    pub fn new() -> Self {
        Self {
            rows: Vec::new(),
            style: Default::default(), // Use default value
        }
    }

    // pub fn set_style(&mut self, new_style: <Theme as style::Catalog>::Style) {
    //     self.style = new_style;
    // }
    
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

        for row_index in 0..self.rows.len() {
            let mut row_view = Row::new().spacing(10);
            for cell_index in 0..self.rows[row_index].cells.len() {
                let cell = &self.rows[row_index].cells[cell_index];
                let cell_view: Element<GridMessage> = match cell {
                    Cell::Text(ref text) => {
                        Text::new(text.clone()).into()
                    }
                    Cell::Button { ref label, on_press } => {
                        Button::new(Text::new(label.clone()))
                            .on_press(GridMessage::Cell(row_index, cell_index, on_press.clone()))
                            .into()
                    }
                    Cell::Container(factory) => (factory.borrow())().map(move |cell_msg| {
                        GridMessage::Cell(row_index, cell_index, cell_msg)
                    }),
                };
                row_view = row_view.push(cell_view);
            }
            column = column.push(row_view);
        }

        column
    }

    pub fn view<'a>(&'a self) -> iced::Element<'a, GridMessage> {
        self.create_grid()
            .into()
    }
}