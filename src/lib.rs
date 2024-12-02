use iced::{
    widget::{Button, Column, Container, Row, Text},
    Element, Length,
};

pub struct Grid {
    rows: Vec<RowData>,
}

impl Grid {
    pub fn new() -> Self {
        Self { rows: Vec::new() }
    }

    pub fn get_cell(&mut self, row_index: usize, cell_index: usize) -> Option<&mut Cell> {
        self.rows.get_mut(row_index).and_then(|row| row.get_mut(cell_index))
    }
    //self.rows.push(RowData::default());
    pub fn get_row(&mut self, row: usize) -> &mut RowData {
        if self.rows.len() <= row {
            self.rows.resize(row + 1, RowData::default());
        }
        &mut self.rows[row]
    }

    pub fn row_count(&self) -> usize {
        self.rows.len() // Assuming `self.rows` is a Vec or something similar
    }

    pub fn add_row(&mut self) {
        self.rows.push(RowData::default());
    }

    pub fn view<'a>(&'a self) -> iced::Element<'a, GridMessage> {
        let mut content = Column::new().spacing(20);

        for (row_index, row) in self.rows.iter().enumerate() {
            let mut row_view = Row::new().spacing(10);

            for (cell_index, cell) in row.cells.iter().enumerate() {
                let cell_view = cell.view().map(move |msg| GridMessage::Cell(row_index, cell_index, msg));
                row_view = row_view.push(cell_view);
            }

            content = content.push(row_view);
        }

        Container::new(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .padding(20)
            .center_x()
            .center_y()
            .into()
    }
}

#[derive(Default, Clone)]
pub struct RowData {
    cells: Vec<Cell>,
}

impl RowData {
    pub fn push(&mut self, config: CellConfig) {
        self.cells.push(Cell::new(config));
    }
    pub fn get_mut(&mut self, index: usize) -> Option<&mut Cell> {
        self.cells.get_mut(index)
    }
}


#[derive(Clone)]
pub struct Cell {
    config: CellConfig, // Store the configuration instead of the Element
}

impl Cell {
    pub fn new(config: CellConfig) -> Self {
        Self { config }
    }

    pub fn edit(&mut self, new_config: CellConfig) {
        self.config = new_config;
    }
    pub fn remove(&mut self) {
        // Logic to remove or clear cell content
        self.config = CellConfig::Text(String::new()); 
    }

    pub fn view(&self) -> Element<CellMessage> {
        match &self.config {
            CellConfig::Text(content) => Text::new(content.clone()).into(),
            CellConfig::Button(label) => {
                Button::new(Text::new(label.clone()))
                    .on_press(CellMessage::Clicked)
                    .into()
            }
        }
    }
}


#[derive(Clone)]
pub enum CellConfig {
    Text(String),
    Button(String),
}


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
