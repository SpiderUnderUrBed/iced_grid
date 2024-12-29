use iced::{
    widget::{Button, Text},
    Element, Theme,
};

use iced_widget::{
    container, scrollable, Column, Container, Row 

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
    Clicked(usize, usize, fn()),
}

#[derive(Debug, Clone)]
pub enum GridMessage {
    AddCell(usize),
    // Cell(usize, usize, CellMessage),
}

pub enum Cell<'a> {
    Text(String),
    Button {
        label: String,
        on_press: fn(),
    },
    Container(Container<'a, CellMessage>)
    //Container(Arc<RefCell<dyn Fn() -> Element<'a, CellMessage> + 'a>>),
}

// impl<'a, Message, Theme, Renderer> From<&Container<'a, Message, Theme, Renderer>>
//     for Element<'a, Message, Theme, Renderer>
// where
//     Message: 'a,
//     Theme: Catalog + iced_widget::container::Catalog + 'a,
//     Renderer: iced_core::Renderer + 'a,
// {
//     fn from(
//         column: Container<'a, Message, Theme, Renderer>,
//     ) -> Element<'a, Message, Theme, Renderer> {
//         Element::new(column)
//     }
// }

impl Cell<'_> {
    pub fn view(&self) -> Element<CellMessage> {
        let element = match self {
            Cell::Text(content) => Text::new(content.clone()).into(),
            Cell::Button { label, on_press } => {
                Button::new(Text::new(label.clone()))
                    .on_press(on_press.clone())
                    .into()
            }
            Cell::Container(factory) => (factory.borrow())(),
        };
        element
    }
}

#[derive(Default)]
pub struct RowData {
    pub cells: Vec<Cell<'static>>,
}

impl RowData {
    pub fn push_text(&mut self, content: String) {
        self.cells.push(Cell::Text(content));
    }

    pub fn push_button(&mut self, label: String, on_press: fn()) {
        self.cells.push(Cell::Button { label, on_press });
    }

    // pub fn push_container<F>(&mut self, factory: F)
    // where
    //     F: Fn() -> Element<'static, CellMessage> + 'static,
    pub fn push_container(&mut self, container: Container<'static, CellMessage>) {
        self.cells.push(Cell::Container(container));
        //self.cells.push(Cell::Container(Arc::new(RefCell::new(factory))));
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
// impl<'a, Message, Theme> Widget<Message, Theme, iced::Renderer>for Grid<Message, Theme>
// where
//     Theme: super::Catalog<Themes = iced_core::Theme> + iced_widget::container::Catalog
//     + Clone + iced_widget::text::Catalog,
//     iced_core::Element<'a, Message, Theme, iced_widget::Renderer>: From<Grid<Message, Theme>>,
// {
// }
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

impl<Message, Theme> From<iced::Element<'_, Message, Theme>> for Grid<Message, Theme>
where
    Theme: style::Catalog,
{
    fn from(_element: iced::Element<'_, Message, Theme>) -> Self {
        
        
        Grid {
            rows: Vec::new(),
            style: Default::default(), 
            on_sync: |_| panic!("Conversion from Element not implemented"),
        }
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
                row.cells.push(Cell::Text("Default".to_string())); 
            }
        }
    }

    pub fn create_grid(&'a self) -> Column<'a, CellMessage> {
        let mut column = Column::new().spacing(10);
    
        for row_index in 0..self.rows.len() {
            let mut row_view = Row::new().spacing(10);
            for cell_index in 0..self.rows[row_index].cells.len() {
                let cell = &self.rows[row_index].cells[cell_index];
                let cell_view: Element<CellMessage> = match cell {
                    Cell::Text(ref text) => {
                        Text::new(text.clone()).into()
                    }
                    Cell::Button { ref label, on_press } => {
                        Button::new(Text::new(label.clone()))
                            .on_press(CellMessage::Clicked(row_index, cell_index, on_press.clone()))
                            .into()
                    }
                    Cell::Container(container) => {
                         let test: Element<CellMessage> = <Container<'_, CellMessage> as Into<Element<CellMessage>>>::into(container);
                        test
                        // Element::new(container)
                    }
                };
                row_view = row_view.push(cell_view);
            }
            column = column.push(row_view);
        }
    
        column
    }
    

    pub fn view(&'a self) -> iced::Element<'a, CellMessage> 
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