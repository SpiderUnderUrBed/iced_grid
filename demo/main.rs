use iced::{Sandbox, Settings};
use iced::widget::{Button, Column, Row, Text}; 
use iced::{Element};
use iced_grid::{CellMessage, Grid, GridMessage, CellConfig};

pub fn main() -> iced::Result {
    MyApp::run(Settings::default())
}

struct MyApp {
    grid: Grid,  
}

#[derive(Debug, Clone)]
pub enum Message {
    AddRow,        
    AddCell(usize), 
}

impl Sandbox for MyApp {
    type Message = Message;

    fn new() -> Self {
        let mut grid = Grid::new();
        grid.add_row();
        grid.get_row(0).push(CellConfig::Text("Hello, Cell!".to_string()));

        Self { grid }
    }

    fn title(&self) -> String {
        String::from("My Grid Application")
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::AddRow => {
                self.grid.add_row(); 
            }
            Message::AddCell(row_index) => {
                self.grid
                    .get_row(row_index)
                    .push(CellConfig::Text("New cell".to_string())); 
            }
        }
    }

    fn view(&self) -> Element<Self::Message> {
        let mut content = Column::new().spacing(10);

        
        let add_row_button = Button::new(Text::new("Add Row"))
            .on_press(Message::AddRow);

        content = content.push(add_row_button);

        
        for row_index in 0..self.grid.row_count() {
            let add_cell_button = Button::new(Text::new("Add Cell"))
                .on_press(Message::AddCell(row_index)); 
            
            
            let mut row_view = Row::new().spacing(10);
            row_view = row_view.push(add_cell_button);

            
            content = content.push(row_view);
        }

        
        
        let grid_element: Element<Message> = self.grid.view().map(|grid_message| {
            match grid_message {
                
                
                
                GridMessage::AddCell(row_index) => Message::AddCell(row_index),
                _ => Message::AddRow, 
            }
        });

        content = content.push(grid_element);

        content.into()
    }
}
