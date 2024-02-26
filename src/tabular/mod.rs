#[cfg(feature = "colored-output")]
use colored::*;

#[derive(Debug, Clone)]
#[cfg(feature = "tabular-output")]
pub(crate) struct Cell {
    content: String,
    #[cfg(feature = "colored-output")]
    pub(crate) color: Option<Color>,
}

#[cfg(feature = "tabular-output")]
impl Cell {
    pub(crate) fn new(content: &str) -> Self {
        #[cfg(all(feature = "colored-output", feature = "tabular-output"))]
        return Cell { content: content.to_string(), color: None };
        #[cfg(not(all(feature = "colored-output", feature = "tabular-output")))]
        return Cell { content: content.to_string() };
    }

    #[cfg(feature = "colored-output")]
    pub(crate) fn with_color(mut self, color: Color) -> Self {
        self.color = Some(color);
        self
    }

    #[cfg(feature = "colored-output")]
    pub(crate) fn get_colored_content(&self) -> ColoredString {
        if let Some(color) = self.color {
            self.content.color(color).to_string().into()
        } else {
            self.content.clone().into()
        }
    }

    #[cfg(feature = "tabular-output")]
    pub(crate) fn get_contents(&self) -> String{
        self.content.clone()
    }
}

#[cfg(feature = "tabular-output")]
#[derive(Debug, Clone)]
pub(crate) struct Row {
    cells: Vec<Cell>,
}

#[cfg(feature = "tabular-output")]
impl Row {
    pub(crate) fn new() -> Self {
        Row { cells: Vec::new() }
    }

    pub(crate) fn with_cell(mut self, cell: Cell) -> Self {
        self.cells.push(cell);
        self
    }

    pub(crate) fn with_cell_str(mut self, content: &str) -> Self {
        self.cells.push(Cell::new(content));
        self
    }

    pub(crate) fn get_cells(&self) -> Vec<Cell> {
        self.cells.clone()
    }
}

#[cfg(feature = "tabular-output")]
#[derive(Debug, Clone)]
pub(crate) enum BorderStyle {
    None,
    Ascii,
    Rounded,
    Double,
    // Add more styles as needed...
}

#[cfg(feature = "tabular-output")]
#[derive(Debug, Clone)]
pub(crate) struct Table {
    rows: Vec<Row>,
    border_style: BorderStyle,
}

#[cfg(feature = "tabular-output")]
impl Table {
    pub(crate) fn get_rows(&self) -> Vec<Row> {
        self.rows.clone()
    }
}

#[cfg(feature = "tabular-output")]
impl Table {
    pub(crate) fn new(header: &str) -> Self {
        let header_row = Row::new().with_cell(Cell::new(header));
        Table { rows: vec![header_row], border_style: BorderStyle::Ascii }
    }

    pub(crate) fn add_row(&mut self, row: Row) {
        self.rows.push(row);
    }

    pub(crate) fn with_border_style(mut self, border_style: BorderStyle) -> Self {
        self.border_style = border_style;
        self
    }

    pub(crate) fn to_string(&self) -> String {
        String::new()
    }
}
