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
    Container(Container<'a, CellMessage>),
}


#[derive(Default)]
pub struct RowData {
    pub cells: Vec<Cell<'static>>,
}

impl RowData {
    pub fn new(cells: Vec<Cell<'static>>) -> Self {
        Self { cells }
    }
    pub fn push_text(&mut self, content: String) {
        
        let text =  iced::widget::Text::new("Hello, World!");


        
        let widget: &dyn iced_core::Widget<CellMessage, Theme, iced::Renderer> = &text;
        self.cells.push(Cell::Text(text));
    }

    pub fn push_button(&mut self, label: String, on_press: CellMessage) {
        self.cells.push(Cell::Button(Button::new(Text::new(label)).on_press(on_press)))
    }
    pub fn push_container(&mut self, container: Container<'static, CellMessage>) {
       
        self.cells.push(Cell::Container(container));
    }
    

    pub fn len(&self) -> usize {
        self.cells.len()
    }

    pub fn get_mut(&mut self, index: usize) -> Option<&mut Cell<'static>> {
        self.cells.get_mut(index)
    }
}

pub struct GridData<Message, Theme>
where
    Theme: style::Catalog,
{
    pub rows: Vec<RowData>,
    pub style: <Theme as style::Catalog>::Style,
    pub theme: Theme,
    pub on_sync: fn(scrollable::AbsoluteOffset) -> Message,
    pub width: f32,
    pub height: f32,
    pub intrinsic_size: Size,
}


pub struct Grid<Message, Theme>
where
    Theme: style::Catalog,
{
    pub data: GridData<Message, Theme>
}

impl<Message, Theme> Default for GridData<Message, Theme>
where
    Theme: style::Catalog + Default, 
{
    fn default() -> Self {
        Self {
            rows: Vec::new(), 
            style: Default::default(), 
            theme: Default::default(), 
            on_sync: |_offset: scrollable::AbsoluteOffset| panic!("on_sync not initialized"), 
            width: 0.0, 
            height: 0.0, 
            intrinsic_size: Size::default(), 
        }
    }
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

impl<Message, Theme> GridData<Message, Theme>
where
    Theme: style::Catalog,
{
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
}

impl<'a, Message, Theme> Grid<Message, Theme>
where
    Theme: style::Catalog<Themes = iced_core::Theme, Style = iced_widget::container::Style> + iced_widget::text::Catalog + iced_widget::container::Catalog + Clone,
{
    pub fn new<I>(rows: I, style: <Theme as style::Catalog>::Style, on_sync: fn(scrollable::AbsoluteOffset) -> Message, width: f32, height: f32, intrinsic_size: Size, theme: Theme) -> Self
    where
        I: IntoIterator<Item = RowData>,
    {
        let data = GridData::new(rows, style, on_sync, width, height, intrinsic_size, theme);
        
        Self { data }
    }


    pub fn style(&mut self, style: impl Into<<Theme as style::Catalog>::Style>) {
        self.data.style = style.into();
    }
    
    

    pub fn get_cell(&mut self, row_index: usize, cell_index: usize) -> Option<&mut Cell<'static>> {
        self.data.rows.get_mut(row_index).and_then(|row: &mut RowData| row.cells.get_mut(cell_index))
    }

    pub fn rows_mut_iter(&mut self) -> impl Iterator<Item = &mut RowData> {
        self.data.rows.iter_mut()
    }
    

    pub fn get_row(&mut self, row: usize) -> &mut RowData {
        if self.data.rows.len() <= row {
            self.data.rows.resize_with(row + 1, RowData::default);
        }
        &mut self.data.rows[row]
    }

    pub fn row_count(&self) -> usize {
        self.data.rows.len()
    }

    pub fn add_row(&mut self, mut row: RowData) {
        self.data.rows.push(row);
    }
    
    pub fn get_row_mut(&mut self, row: usize) -> Option<&mut RowData> {
        if row < self.data.rows.len() {
            Some(&mut self.data.rows[row])
        } else {
            None
        }
    }
    pub fn add_rows(&mut self, count: usize) {
        for _ in 0..count {
            self.data.rows.push(RowData::default());
        }
    }
    
    pub fn add_cells_to_row(&mut self, row_index: usize, count: usize) {
        let row = self.get_row(row_index); 
        for _ in 0..count {
            row.cells.push(Cell::Text(Text::new("Default"))); 
            
        }
    }

    
    pub fn add_cells_to_all_rows(&mut self, count: usize) {
        for row in &mut self.data.rows {
            for _ in 0..count {
                row.cells.push(Cell::Text(Text::new("Default"))); 
            }
        }
    }

    pub fn to_element(&'a self) -> iced_core::Element<'a, Message, Theme, Renderer> {
        Element::new(
            Wrapper {
                content: Box::new(self),
                target: Style,
                theme: self.data.theme.clone(),
                style: self.data.style.clone(),
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
    






















