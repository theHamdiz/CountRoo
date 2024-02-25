use num_format::{Locale, ToFormattedString};
use std::fs::File;
use std::io::{self, BufRead};
use walkdir::WalkDir;

#[cfg(feature = "default")]

pub trait CertainTypesCounter {
    fn count_lines_of_code_for_certain_types(&mut self) -> io::Result<u64>;
}

#[cfg(feature = "default")]
pub trait AllTypesCounter {
    fn count_lines_of_code_for_all_types(&mut self) -> io::Result<u64>;
}

#[cfg(any(feature = "default", feature = "formatted_output"))]
pub trait FormattedOutput {
    fn formatted_output(&self) -> String;
}

#[derive(Debug)]
#[cfg(feature = "default")]
pub struct CountRoo {
    total_lines: u64,
    project_path: Option<String>,
    extensions_to_sum: Option<Vec<String>>,
    count_empty_lines: bool,
}

#[cfg(feature = "default")]
impl Default for CountRoo {
    fn default() -> Self {
        CountRoo {
            total_lines: 0,
            project_path: Some("../".to_string()),
            extensions_to_sum: Some(vec![".rs".to_string()]),
            count_empty_lines: false,
        }
    }
}

#[cfg(feature = "default")]
impl CountRoo {
    pub fn new(project_path: Option<String>, extensions_to_sum: Option<Vec<String>>) -> Self {
        CountRoo {
            total_lines: 0,
            project_path,
            extensions_to_sum,
            count_empty_lines: false,
        }
    }

    pub fn set_project_path(&mut self, project_path: &str) {
        self.project_path = Some(project_path.to_string());
    }

    pub fn set_extensions_to_sum(&mut self, extensions: Vec<String>) {
        self.extensions_to_sum = Some(extensions);
    }

    fn count_lines_of_code(&mut self, extensions: &Option<Vec<String>>) -> io::Result<u64> {
        let project_path = self.project_path.as_ref().unwrap();

        let extensions: Vec<String> = extensions
            .as_ref()
            .unwrap_or(&vec![".rs".to_string()])
            .clone();

        for entry in WalkDir::new(project_path)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| {
                e.file_type().is_file() && extensions.iter().any(|ext| e.path().to_str().unwrap().ends_with(ext))
            })
        {
            let file = File::open(entry.path())?;
            let reader = io::BufReader::new(file);

            if self.count_empty_lines {
                // If counting empty lines, simply count all lines
                self.total_lines += reader.lines().count() as u64;
            } else {
                // If not counting empty lines, filter out empty lines before counting
                self.total_lines += reader.lines().filter_map(Result::ok).filter(|line| !line.trim().is_empty()).count() as u64;
            }
        }

        Ok(self.total_lines)
    }

}
#[cfg(feature = "default")]

impl CertainTypesCounter for CountRoo {
    fn count_lines_of_code_for_certain_types(&mut self) -> io::Result<u64> {
        let extensions = &Some(
            self.extensions_to_sum
                .clone()
                .unwrap_or_else(|| vec![".rs".to_string()]),
        );
        self.count_lines_of_code(extensions)
    }
}
#[cfg(feature = "default")]

impl AllTypesCounter for CountRoo {
    fn count_lines_of_code_for_all_types(&mut self) -> io::Result<u64> {
        self.extensions_to_sum = Some(vec![
            ".py".to_string(),
            ".js".to_string(),
            ".rs".to_string(),
            ".dart".to_string(),
            ".cpp".to_string(),
            ".c".to_string(),
            ".rb".to_string(),
            ".sh".to_string(),
            ".swift".to_string(),
            ".ts".to_string(),
            ".html".to_string(),
            ".css".to_string(),
            ".sql".to_string(),
        ]);
        let extensions = &Some(
            self.extensions_to_sum
                .clone()
                .unwrap_or_else(|| vec![".rs".to_string()]),
        );
        self.count_lines_of_code(extensions)
    }
}


impl FormattedOutput for CountRoo {
    fn formatted_output(&self) -> String {
        self.total_lines.to_formatted_string(&Locale::en)
    }
}