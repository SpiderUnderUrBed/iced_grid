use iced::{
    widget::{Button, Text},
    Element, Theme,
};

use iced_widget::{
    container, scrollable, Column, Row 

};

use std::sync::Arc;
use std::cell::RefCell;

use style::wrapper::{Target, Wrapper};
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
    pub cells: Vec<Cell<'static>>,
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
    style: <Theme as style::Catalog>::Style,
    on_sync: fn(scrollable::AbsoluteOffset) -> Message,
}

impl<'a, Message, Theme, Renderer> From<Grid<Message, Theme>>
for Element<'a, Message, Theme, Renderer>
where
   
    Renderer: iced_core::Renderer + 'a,
    Theme: style::Catalog + container::Catalog + scrollable::Catalog + 'a,
    Message: 'a + Clone,
{
    fn from(grid: Grid<Message, Theme>) -> Self {
        let style = grid.style.clone(); 
    
        Element::new(Wrapper { 
            content: grid.into(),
            target: Target::Style,
            style,
        })
    }    
}


impl<'a, Message, Theme: style::Catalog> Grid<Message, Theme> {
    pub fn new(rows: Vec<RowData>, style: <Theme as style::Catalog>::Style, on_sync: fn(scrollable::AbsoluteOffset) -> Message ) -> Self {
        Self { rows, style, on_sync }
    }
    
    pub fn style(&mut self, style: impl Into<<Theme as style::Catalog>::Style>) {
        self.style = style.into();
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
                println!("test");
                row.cells.push(Cell::Text("Default".to_string())); 
            }
        }
    }


    pub fn create_grid(&'a self) -> Column<'a, GridMessage> {
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

    pub fn view(&'a self) -> iced::Element<'a, GridMessage> 
    where iced_widget::container::Style: From<<Theme as style::Catalog>::Style>
    {
        
        container(
            self.create_grid()
                .padding(10) 
                .spacing(5), 
        )
        .style({
            move |theme: &crate::Theme| self.style.clone().into() 
        })
 
        
        
        .into()
    }

}