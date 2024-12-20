use iced::{
    advanced::widget, widget::{Button, Text}, Element, Renderer, Size, Theme
};

use iced_widget::{container, scrollable, Column, Container, Row};

use std::sync::Arc;
use std::cell::RefCell;

use style::wrapper::{Target, Wrapper, 
};
pub use style::Catalog;
pub mod style;


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
    Container(Element<'a, CellMessage>),
    
}


#[derive(Default)]
pub struct RowData {
    pub cells: Vec<Cell<'static>>,
}

impl RowData {
    pub fn push_text(&mut self, content: String) {
        self.cells.push(Cell::Text(content));
    }

    pub fn push_button(&mut self, label: String, on_press: CellMessage) {
        self.cells.push(Cell::Button { label, on_press });
    }
    pub fn push_container(&mut self, container: Container<'static, CellMessage>) {
        self.cells.push(Cell::Container(Element::new(container)));
    }
    

    pub fn len(&self) -> usize {
        self.cells.len()
    }

    pub fn get_mut(&mut self, index: usize) -> Option<&mut Cell<'static>> {
        self.cells.get_mut(index)
    }
}

   
pub struct Grid<Message, Theme>
where
    Theme: style::Catalog,
{
    rows: Vec<RowData>,
    pub style: <Theme as style::Catalog>::Style,
    on_sync: fn(scrollable::AbsoluteOffset) -> Message,
    width: f32,
    height: f32,
    intrinsic_size: Size,
}


impl<'a, GridMessage, Theme> From<Grid<GridMessage, Theme>> for Element<'_, GridMessage, Theme>
where
    Theme: style::Catalog + 'a,
    GridMessage: 'a,
{
    fn from(grid: Grid<GridMessage, Theme>) -> Self {
        Element::from(grid)
    }
}

impl<'a, Message, Theme: style::Catalog> Grid<Message, Theme>


 {
    pub fn new(rows: Vec<RowData>, style: <Theme as style::Catalog>::Style, on_sync: fn(scrollable::AbsoluteOffset) -> Message, width: f32, height: f32, intrinsic_size: Size) -> Self {
        Self { rows, style, on_sync, width, height, intrinsic_size }
    }
    
    pub fn style(&mut self, style: impl Into<<Theme as style::Catalog>::Style>) {
        self.style = style.into();
    }
    

    pub fn get_cell(&mut self, row_index: usize, cell_index: usize) -> Option<&mut Cell<'static>> {
        self.rows.get_mut(row_index).and_then(|row| row.cells.get_mut(cell_index))
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

    pub fn add_row(&mut self, row: RowData) {
        self.rows.push(row);
    }

    pub fn get_row_mut(&mut self, row: usize) -> Option<&mut RowData> {
        if row < self.rows.len() {
            Some(&mut self.rows[row])
        } else {
            None
        }
    }
    pub fn add_rows(&mut self, count: usize) {
        for _ in 0..count {
            self.rows.push(RowData::default());
        }
    }
    
    pub fn add_cells_to_row(&mut self, row_index: usize, count: usize) {
        let row = self.get_row(row_index); 
        for _ in 0..count {
            row.cells.push(Cell::Text("Default".to_string())); 
        }
    }

    
    pub fn add_cells_to_all_rows(&mut self, count: usize) {
        for row in &mut self.rows {
            for _ in 0..count {
                row.cells.push(Cell::Text("Default".to_string())); 
            }
        }
    }

    pub fn to_element(&'a self) -> iced_core::Element<'a, Message, Theme, Renderer> {
        Element::new(
            Wrapper {
                content: self,
                target: Target::Style,
                style: self.style.clone(),
            }
        )
    }
    

    pub fn create_grid(&self) -> Column<'_, GridMessage> {
        let mut column = Column::new().spacing(10);

        for (row_index, row) in self.rows.iter().enumerate() {
            let mut row_view = Row::new().spacing(10);

            for (cell_index, cell) in row.cells.iter().enumerate() {
                let cell_view: Element<GridMessage> = match cell {
                    Cell::Text(ref text) => Text::new(text.clone()).into(),
                    Cell::Button { ref label, on_press } => {
                        Button::new(Text::new(label.clone()))
                            .on_press(GridMessage::Cell(row_index, cell_index, on_press.clone()))
                            .into()
                    }
                    Cell::Container(element) => container("test").into(),
                };
                row_view = row_view.push(cell_view);
            }

            column = column.push(row_view);
        }

        column
    }

    pub fn view(self) -> iced::Element<'a, GridMessage>
    where
        Message: 'a,
        GridMessage: 'a,
        Theme: style::Catalog<Style = iced_widget::container::Style>, 
        iced::Element<'a, GridMessage>: From<Grid<Message, Theme>>, 
    {
        iced::Element::from(self) 
    }
    
    
    

}