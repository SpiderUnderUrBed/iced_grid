use iced::{
    widget::{Button, Text}, Element, Renderer, Theme
};


// use iced_core::overlay::Element;
use iced_widget::{container, scrollable, Column, Container, Row};

use std::sync::Arc;
use std::cell::RefCell;

use style::wrapper::{Target, 
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
    //element: Element<'static, Message, Theme, Renderer>
}
// impl<'a, GridMessage, Theme> From<Grid<GridMessage, Theme>> for Element<'_, GridMessage, Theme>
// where
//     Theme: style::Catalog + 'a,
//     GridMessage: 'a,
// {
//     fn from(grid: Grid<GridMessage, Theme>) -> Self {

//         iced::Element::new(Wrapper {
//             content: &mut iced::Element::from(grid),
//             target: Target::Style,
//             style: grid.style,
//         })
//     }
// }
impl<'a, GridMessage, Theme> From<Grid<GridMessage, Theme>> for Element<'_, GridMessage, Theme>
where
    Theme: style::Catalog + 'a,
    GridMessage: 'a,
{
    fn from(grid: Grid<GridMessage, Theme>) -> Self {
       // let content = grid.create_grid();
        
        Element::from(grid)
    }
}
// impl <Message>Into<iced_core::Element<'_, Message, Theme, Renderer>> for Grid<Message, Theme>
// {
//     fn into(self) -> iced_core::Element<'static, Message, Theme, Renderer> {
//         todo!()
//     }
// }

// impl<'a, GridMessage, Theme> From<&mut Grid<GridMessage, Theme>> for Element<'_, GridMessage, Theme>
// where
//     Theme: style::Catalog + 'a,
//     GridMessage: 'a,
// {
//     fn from(grid: &mut Grid<GridMessage, Theme>) -> Self {
//        // let content = grid.create_grid();
        
//         Element::from(grid)
//     }
// }
// impl<'a, GridMessage, Theme> From<&&mut Grid<GridMessage, Theme>> for Element<'_, GridMessage, Theme>
// where
//     Theme: style::Catalog + 'a,
//     GridMessage: 'a,
// {
//     fn from(grid: &&mut Grid<GridMessage, Theme>) -> Self {
//        // let content = grid.create_grid();
        
//         Element::from(grid)
//     }
// }
// impl<Message, Theme> From<iced::Element<'_, Message, Theme>> for Grid<Message, Theme>
// where
//     Theme: style::Catalog,
// {
//     fn from(_element: iced::Element<'_, Message, Theme>) -> Self {
//         Grid {
//             rows: Vec::new(),
//             style: Default::default(), 
//             on_sync: |_| panic!("Conversion from Element not implemented"),
//         }
//     }
// }

impl<'a, Message, Theme: style::Catalog> Grid<Message, Theme>
// where
// Renderer: iced_core::Renderer,
 {
    pub fn new(rows: Vec<RowData>, style: <Theme as style::Catalog>::Style, on_sync: fn(scrollable::AbsoluteOffset) -> Message) -> Self {
        Self { rows, style, on_sync }
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

    pub fn to_element(&self) -> iced_core::Element<'static, Message, Theme, Renderer> {
        todo!()
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