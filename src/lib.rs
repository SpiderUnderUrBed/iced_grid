use iced::{
    widget::{Button, Text}, Element, Renderer, Size, Theme
};

use iced_widget::{button, container, scrollable, Column, Container, Row};

use std::sync::Arc;
use std::cell::RefCell;
use crate::style::wrapper::Style;
use style::wrapper::Wrapper;
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
    Sync
}


pub enum Cell<'a> {
    Text(Text<'a, Theme>),
    Button(Button<'a, CellMessage, Theme>),
    // Button {
    //     button: 
    //     // label: String,
    //     // on_press: CellMessage,
    // },
    Container(Container<'a, CellMessage>),
    
}


#[derive(Default)]
pub struct RowData {
    pub cells: Vec<Cell<'static>>,
}

impl RowData {
    pub fn push_text(&mut self, content: String) {
        //self.cells.push(Cell::Text(Text::new(content)));
        let text =  iced::widget::Text::new("Hello, World!");


        // Cast the wrapper to a `&dyn Widget<Message, Theme, Renderer>`
        let widget: &dyn iced_core::Widget<CellMessage, Theme, iced::Renderer> = &text;
        self.cells.push(Cell::Text(text));
    }

    pub fn push_button(&mut self, label: String, on_press: CellMessage) {
        self.cells.push(Cell::Button(Button::new(Text::new(label)).on_press(on_press)))
    }
    pub fn push_container(&mut self, container: Container<'static, CellMessage>) {
       // let widget: &dyn iced_core::Widget<CellMessage, Theme, iced::Renderer> = &container;
        self.cells.push(Cell::Container(container));
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
    pub theme: Theme,
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

impl<'a, Message, Theme: style::Catalog<Themes = iced_core::Theme, Style = iced_widget::container::Style> + iced_widget::text::Catalog  + iced_widget::container::Catalog + Clone> Grid<Message, Theme>
//Theme: super::Catalog<Style = iced_widget::container::Style>,

 {
    // pub fn new(rows: Vec<RowData>, style: <Theme as style::Catalog>::Style, on_sync: fn(scrollable::AbsoluteOffset) -> Message, width: f32, height: f32, intrinsic_size: Size, theme: Theme) -> Self {
    //     Self { rows, style, on_sync, width, height, intrinsic_size, theme }
    // }
    pub fn new<I>(rows: I, style: <Theme as style::Catalog>::Style, on_sync: fn(scrollable::AbsoluteOffset) -> Message, width: f32, height: f32, intrinsic_size: Size, theme: Theme) -> Self
where
    I: IntoIterator<Item = RowData>,
{
    Self {
        rows: rows.into_iter().collect(),
        style,
        on_sync,
        width,
        height,
        intrinsic_size,
        theme,
    }
}

    pub fn style(&mut self, style: impl Into<<Theme as style::Catalog>::Style>) {
        self.style = style.into();
    }
    
    //pub fn get_grid

    pub fn get_cell(&mut self, row_index: usize, cell_index: usize) -> Option<&mut Cell<'static>> {
        self.rows.get_mut(row_index).and_then(|row: &mut RowData| row.cells.get_mut(cell_index))
    }

    pub fn rows_mut_iter(&mut self) -> impl Iterator<Item = &mut RowData> {
        self.rows.iter_mut()
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

    pub fn add_row(&mut self, mut row: RowData) {
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
            row.cells.push(Cell::Text(Text::new("Default"))); 
            //"Default".to_string()
        }
    }

    
    pub fn add_cells_to_all_rows(&mut self, count: usize) {
        for row in &mut self.rows {
            for _ in 0..count {
                row.cells.push(Cell::Text(Text::new("Default"))); 
            }
        }
    }
//Theme: super::Catalog<Style = iced_widget::container::Style>,
    pub fn to_element(&'a self) -> iced_core::Element<'a, Message, Theme, Renderer> {
        Element::new(
            Wrapper {
                content: Box::new(self),
                target: Style,
                theme: self.theme.clone(),
                style: self.style.clone(),
            }
        )
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
    
// pub fn from_cells(
//     cells: Vec<Vec<Cell<'static>>>,
//     style: <Theme as style::Catalog>::Style,
//     on_sync: fn(scrollable::AbsoluteOffset) -> Message,
//     width: f32,
//     height: f32,
//     intrinsic_size: Size,
//     theme: Theme,
// ) -> Self { let rows = cells
//         .into_iter()
//         .map(|cell_row| RowData { cells: cell_row })
//         .collect();

//     Self {
//         rows,
//         style,
//         on_sync,
//         width,
//         height,
//         intrinsic_size,
//         theme,
//     }
// }