use iced::application::Title;
use iced::{Application, Element, Settings, Subscription, Theme};
use iced_grid::{Grid, RowData, CellMessage};

#[derive(Debug, Clone)]
enum Message {
    Ui(UiMessage),
    Grid(iced_grid::GridMessage),
}

#[derive(Debug, Clone)]
enum UiMessage {
    AddRow,
    AddCell(usize), // usize represents the row to which a cell will be added
    ButtonClicked(usize, usize),
    Sync,
}

impl From<UiMessage> for Message {
    fn from(ui_message: UiMessage) -> Self {
        Message::Ui(ui_message)
    }
}

impl From<iced_grid::GridMessage> for Message {
    fn from(grid_message: iced_grid::GridMessage) -> Self {
        Message::Grid(grid_message)
    }
}

pub struct MyApp {
    grid: Grid<Message, MyTheme>,
}

impl Default for MyApp {
    fn default() -> Self {
        let rows = vec![];

        // Create the grid
        let mut grid = Grid::new(
            rows,
            (),
            |_offset: iced::widget::scrollable::AbsoluteOffset| UiMessage::Sync.into(),
        );

        // Add an initial row to the grid
        let mut row = RowData::default();
        row.push_text("Row 1, Cell 1".into());
        row.push_button("Add Row".into(), CellMessage::Clicked);
        row.push_button("Add Cell".into(), CellMessage::Clicked);
        grid.add_row(row);

        MyApp { grid }
    }
}

#[derive(Clone)]
pub struct MyTheme;

impl iced_grid::style::Catalog for MyTheme {
    type Style = ();

    fn TARGET(&self, _style: &Self::Style) -> iced::widget::container::Style {
        iced::widget::container::Style::default()
    }
}

impl MyApp {
    fn view<'a>(&'a self) -> iced::Element<'a, Message> {
        self.grid.view().map(Message::from)
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::Ui(ui_message) => match ui_message {
                UiMessage::AddRow => {
                    let mut new_row = RowData::default();
                    let row_index = self.grid.row_count();
                    new_row.push_text(format!("Row {}, Cell 1", row_index + 1).into());
                    new_row.push_button("Add Row".into(), CellMessage::Clicked);
                    new_row.push_button("Add Cell".into(), CellMessage::Clicked);
                    self.grid.add_row(new_row);
                }
                UiMessage::AddCell(row_index) => {
                    if let Some(row) = self.grid.get_row_mut(row_index) {
                        let cell_count = row.cells.len() - 2; // Exclude Add Row and Add Cell buttons
                        row.push_text(format!("Row {}, Cell {}", row_index + 1, cell_count + 1).into());
                    }
                }
                UiMessage::ButtonClicked(row, col) => {
                    println!("Button clicked in row {}, column {}", row, col);
                }
                UiMessage::Sync => {
                    println!("Syncing...");
                }
            },
            Message::Grid(grid_message) => match grid_message {
                iced_grid::GridMessage::Cell(row, col, CellMessage::Clicked) => {
                    // Determine action based on the column index
                    if col == 1 {
                        // Add Row button clicked
                        self.update(Message::Ui(UiMessage::AddRow));
                    } else if col == 2 {
                        // Add Cell button clicked
                        self.update(Message::Ui(UiMessage::AddCell(row)));
                    }
                }
                _ => {
                    // Handle other grid messages if necessary
                    println!("Grid message received: {:?}", grid_message);
                }
            },
        }
    }
    

    fn theme(&self) -> Theme {
        Theme::default()
    }
}

fn main() -> iced::Result {
    iced::run("main", MyApp::update, MyApp::view)
}
