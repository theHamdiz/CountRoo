use std::cell::RefCell;
#[cfg(feature = "colored-output")]
use colored::*;

#[cfg(feature = "tabular-output")]
#[derive(Debug, Clone)]
pub(crate) enum BorderStyle {
    None,
    Ascii,
    Rounded,
    Double,
}

fn horizontal_border_line(width: usize, style: BorderStyle) -> String {
    match style {
        BorderStyle::None => "".to_string(),
        BorderStyle::Ascii => "-".repeat(width),
        BorderStyle::Rounded => "═".repeat(width),
        BorderStyle::Double => "║".repeat(width),
    }
}

fn corner_piece(style: BorderStyle) -> char {
    match style {
        BorderStyle::None => ' ',
        BorderStyle::Ascii => '+',
        BorderStyle::Rounded => '╬',
        BorderStyle::Double => '╠',
    }
}


#[cfg(feature = "tabular-output")]
use std::fmt::{Display, Debug};
#[cfg(feature = "colored-output")]
use crate::colorizer::LanguageBrandings;


// Feature independent
#[derive(Debug)]
#[cfg(feature = "tabular-output")]
pub(crate) struct Table {
    rows: Vec<Row>,
    border_style: BorderStyle,
}

#[cfg(feature = "tabular-output")]
impl Table {
    pub fn new() -> Table {
        Table {
            rows: Vec::new(),
            border_style: BorderStyle::Ascii,
        }
    }

    pub fn with_border_style(mut self, border_style: BorderStyle) -> Self {
        self.border_style = border_style;
        self
    }
}

#[cfg(feature = "tabular-output")]
impl Display for Table {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut max_cols = 0;
        // let empty_cell = Cell::empty();

        for row in &self.rows {
            max_cols = std::cmp::max(max_cols, row.cells.len());
        }
        
        
        let mut row_strs = Vec::new();
        
        let mut output: String = String::new();
        
        output.push_str(&corner_piece(self.border_style.clone()).to_string());
        output.push_str(&horizontal_border_line(max_cols * 4 + 1, self.border_style.clone()));
        output.push_str(&corner_piece(self.border_style.clone()).to_string());
        output.push('\n');
        row_strs.push(output);

        for row in &self.rows {
            output.push_str(&corner_piece(self.border_style.clone()).to_string());
            output.push_str(&row_strs.join(&format!(" {} ", corner_piece(self.border_style.clone())))); // Adjust spacing as needed
            output.push_str(&corner_piece(self.border_style.clone()).to_string());
            output.push('\n');

            if row != self.rows.last().unwrap() { // Don't add after the last row
                output.push_str(&corner_piece(self.border_style.clone()).to_string());
                output.push_str(&horizontal_border_line(max_cols * 4 + 1, self.border_style.clone()));
                output.push_str(&corner_piece(self.border_style.clone()).to_string());
                output.push('\n');
            }
        }

        output.push_str(&corner_piece(self.border_style.clone()).to_string());
        output.push_str(&horizontal_border_line(max_cols * 4 + 1, self.border_style.clone()).to_string());
        output.push_str(&corner_piece(self.border_style.clone()).to_string());
        output.push('\n');

        // for row in &self.rows {
        //     let mut cells_strs = Vec::new();
        //     for i in 0..max_cols {
        //         let cell = row.cells.get(i).unwrap_or_else(|| &empty_cell);
        //         cells_strs.push(format!("{: >width$}", cell.content, width=max_cols));
        //     }
        //     row_strs.push(format!("| {} |", cells_strs.join(" | ")));
        // }

        write!(f, "{}", row_strs.join("\n"))
    }
}


// Color-enabled
#[derive(Debug)]
#[cfg(feature = "colored-output")]
pub(crate) struct ColoredTable<'a> {
    rows: Vec<Row>,
    border_style: BorderStyle,
    language_brandings: Option<LanguageBrandings<'a>>,
}

#[cfg(feature = "colored-output")]
impl<'a> ColoredTable<'a> {
    pub fn new(language_brandings: LanguageBrandings<'a>) -> ColoredTable<'a> {
        ColoredTable {
            rows: Vec::new(),
            border_style: BorderStyle::Ascii,
            language_brandings: Some(language_brandings)
        }
    }

    pub fn with_border_style(mut self, border_style: BorderStyle) -> Self {
        self.border_style = border_style;
        self
    }
}

#[cfg(feature = "colored-output")]
impl<'a> Display for ColoredTable<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut max_cols = 0;
        let empty_cell = Cell::empty();
        
        for row in &self.rows {
            max_cols = std::cmp::max(max_cols, row.cells.len());
        }

        #[cfg(feature = "colored-output")]
        {
            if let Some(brandings) = &self.language_brandings {
                for row in &self.rows {
                    for cell in &row.cells {
                        if let Some(color) = brandings.get_color_by_extension(&cell.content) {
                            cell.set_color(color);
                        }
                    }
                }
            }
        }
        
        let mut row_strs = Vec::new();
        for row in &self.rows {
            let mut cells_strs = Vec::new();
            for i in 0..max_cols {
                let cell = row.cells.get(i).unwrap_or_else(|| &empty_cell);

                if let Some(color) = *cell.color.borrow() {
                    cells_strs.push(format!("{: >width$}", cell.content.color(color), width=max_cols));
                } else {
                    cells_strs.push(format!("{: >width$}", cell.content, width=max_cols));
                }
            }
            row_strs.push(format!("| {} |", cells_strs.join(" | ")));
        }

        write!(f, "{}", row_strs.join("\n"))
    }
}


#[derive(Debug, Clone)]
#[cfg(feature = "tabular-output")]
pub(crate) struct Row {
    cells: Vec<Cell>,
}

#[cfg(feature = "tabular-output")]
impl From<&[String]> for Row {
    fn from(cols: &[String]) -> Self {
        Row {
            cells: cols.iter().map(|s| Cell::from(s.clone())).collect(),
        }
    }
}

#[cfg(feature = "tabular-output")]
#[derive(Debug, Clone)]
pub(crate) struct Cell {
    content: String,
    #[cfg(feature = "colored-output")]
    color: RefCell<Option<Color>>, // Use RefCell for mutability
}

#[cfg(feature = "tabular-output")]
impl From<&str> for Cell {
    fn from(s: &str) -> Self {
        Cell {
            content: s.to_string(),
            #[cfg(feature = "colored-output")]
            color: RefCell::new(None),
        }
    }
}

#[cfg(feature = "tabular-output")]
impl From<String> for Cell {
    fn from(s: String) -> Self {
        Cell {
            content: s,
            #[cfg(feature = "colored-output")]
            color: RefCell::new(None),
        }
    }
}



impl Cell {
    fn new(content: &str) -> Self {
        Cell { 
            content: content.to_string(),
            #[cfg(feature = "colored-output")]
            color: RefCell::new(None)
        }
    }

    fn empty() -> Self {
        Cell { 
            content: String::new(),
            #[cfg(feature = "colored-output")]
            color: RefCell::new(None) 
        }
    }

    #[cfg(feature = "colored-output")]
    fn set_color(&self, color: Color) {
        *self.color.borrow_mut() = Some(color);
    }
}

impl Display for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.content)
    }
}

#[macro_export]
#[cfg(feature = "tabular-output")]
macro_rules! ptable {
    ($table:expr) => {
        println!("{}", $table);
    };
}

#[macro_export]
#[cfg(feature = "colored-output")]
macro_rules! table {
    ($($cols:expr),+) => {
        Table {
            rows: vec![
                $(Row::from($cols)),+
            ],
            language_brandings: $crate::LanguageBrandings::get_color_by_extension(),
        }
    }}

    // Default implementation without feature flag
    #[cfg(not(feature = "colored-output"))]
    macro_rules! table {
        ($($cols:expr),+) => {
            Table {
                rows: vec![
                    $(Row::from($cols)),+
                ],
                language_brandings: None,
            }
        };
    }
