use iced::{Sandbox, Settings};
use iced::widget::{Button, Column, Row, Text, TextInput};
use iced::{Element};

pub fn main() -> iced::Result {
    MyApp::run(Settings::default())
}

struct MyApp {
    rows: Vec<Vec<String>>, // Each row contains a list of cell contents
    row_input: String,
    cell_input: String,
}

#[derive(Debug, Clone)]
pub enum Message {
    AddRow,
    AddCell,
    UpdateRowInput(String),
    UpdateCellInput(String),
}

impl Sandbox for MyApp {
    type Message = Message;

    fn new() -> Self {
        Self {
            rows: vec![vec!["Hello, Cell!".to_string()]],
            row_input: String::new(),
            cell_input: String::new(),
        }
    }

    fn title(&self) -> String {
        String::from("My Grid Application")
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::AddRow => {
                self.rows.push(vec![]); // Add an empty row
            }
            Message::AddCell => {
                if let Ok(row_index) = self.row_input.parse::<usize>() {
                    if let Some(row) = self.rows.get_mut(row_index) {
                        row.push(self.cell_input.clone()); // Add the cell content to the specified row
                        self.cell_input.clear(); // Clear the input
                    }
                }
            }
            Message::UpdateRowInput(input) => {
                self.row_input = input;
            }
            Message::UpdateCellInput(input) => {
                self.cell_input = input;
            }
        }
    }

    fn view(&self) -> Element<Self::Message> {
        let mut content = Column::new().spacing(20);

        // Button to add a new row
        let add_row_button = Button::new(Text::new("Add Row")).on_press(Message::AddRow);

        content = content.push(add_row_button);

        // Row for input and adding a cell
        content = content.push(
            Row::new()
                .spacing(10)
                .push(
                    TextInput::new("Enter row number", &self.row_input)
                        .padding(5)
                        .size(20)
                        .on_input(Message::UpdateRowInput),
                )
                .push(
                    TextInput::new("Enter cell content", &self.cell_input)
                        .padding(5)
                        .size(20)
                        .on_input(Message::UpdateCellInput),
                )
                .push(Button::new(Text::new("Add Cell")).on_press(Message::AddCell)),
        );

        // Display the grid
        for (row_index, row) in self.rows.iter().enumerate() {
            let row_view = Row::new()
                .spacing(10)
                .push(Text::new(format!("Row {}:", row_index))); // Label each row
            let cells = row
                .iter()
                .map(|cell_content| {
                    Text::new(cell_content.clone())
                        .size(20)
                        .into()
                })
                .collect::<Vec<Element<Message>>>(); // Convert cell contents to iced `Text` widgets

            let full_row = cells
                .into_iter()
                .fold(row_view, |row_view, cell| row_view.push(cell));

            content = content.push(full_row);
        }

        content.into()
    }
}
