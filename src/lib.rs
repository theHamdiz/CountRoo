mod colorizer;
mod output_adapters;
#[cfg(feature = "tabular-output")]
mod tabular;

///
/// Imports Section!
///
use std::collections::HashMap;
use num_format::{Locale, ToFormattedString};
use std::fs::File;
use std::io::{self, BufRead, BufReader, Read};
use walkdir::WalkDir;
use std::error::Error;
use std::{env, fmt};
use std::path::{Path, PathBuf};
use std::str::FromStr;
use rayon::prelude::*;
use serde::Deserialize;

#[cfg(feature = "default")]
use output_adapters::*;

#[cfg(feature = "colored-output")]
use colored::{Color, ColoredString};

#[cfg(feature = "tabular-output")]
use tabular::{Row, Table};

#[cfg(feature = "toml-config")]
use toml::de::Error as TomlError;
#[cfg(feature = "json-config")]
use serde_json::Error as JsonError;
#[cfg(feature = "yaml-config")]
use serde_yaml::Error as YamlError;
#[cfg(feature = "xml-config")]
use quick_xml::de::DeError as XmlError;
#[cfg(feature = "colored-output")]
use crate::colorizer::{AccentColor, LanguageBrandings, LanguageColorMapping};
#[cfg(feature = "tabular-output")]
use crate::tabular::{BorderStyle};

#[cfg(feature = "default")]
const CONFIG_TXT_CONTENTS: &str = include_str!("config.txt");

///
/// Custom Traits & Types Section
///
#[cfg(feature = "default")]

pub trait CertainTypesCounter {
    fn count_lines_of_code_for_certain_types(&mut self) -> Result<usize, LocCounterError>;
}

#[cfg(feature = "default")]
pub trait AllTypesCounter {
    fn count_lines_of_code_for_all_types(&mut self) -> Result<usize, LocCounterError>;
}

#[cfg(feature = "default")]
pub trait FormattedOutput {
    fn formatted_output(&self) -> String;
}

#[cfg(feature = "default")]
pub trait TabularOutput {
    fn tabular_output(&self, output_writer: &dyn OutputWriter);
}

#[cfg(feature = "colored-output")]
pub(crate) trait ColorizedOutput {
    fn apply_colors_to_table(&self, table: &Table, color_map: &[LanguageColorMapping]) -> ColoredString;
}


#[cfg(feature = "default")]
pub(crate) trait PathHelpers{
    fn is_relative_path(path: &str) -> bool;
    fn find_src_folder() -> Option<PathBuf>;
    fn from_rel_file_path(rel_path: &str) -> Result<Config, LocCounterError>;
    fn from_abs_file_path(config_path: &str, project_path: String) -> Result<Config, LocCounterError>;
}

///
/// Error handling section
///
#[cfg(feature = "default")]
#[derive(Debug)]
pub enum LocCounterError {
    IoError(io::Error),
    ConfigError(String),
}

#[cfg(feature = "default")]
impl fmt::Display for LocCounterError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            LocCounterError::IoError(err) => write!(f, "IO Error: {}", err),
            LocCounterError::ConfigError(msg) => write!(f, "Configuration Error: {}", msg),
        }
    }
}

#[cfg(feature = "default")]
impl Error for LocCounterError {}

///
/// Configuration section
///
#[cfg(any(feature = "toml-config", feature = "json-config", feature = "yaml-config", feature = "xml-config", feature = "newline-config"))]
#[derive(Debug, Deserialize, Eq, PartialEq, Clone)]
pub struct Config {
    project_path: String,
    #[cfg(any(feature = "toml-config", feature = "json-config", feature = "yaml-config", feature = "xml-config", feature = "newline-config"))]
    config_path: Option<String>,
    extensions: Vec<String>,
    count_empty_lines: bool,
}


#[cfg(any(feature = "toml-config", feature = "json-config", feature = "yaml-config", feature = "xml-config", feature = "newline-config"))]
impl PathHelpers for Config{
     fn is_relative_path(path: &str) -> bool {
        Path::new(path).is_relative()
    }
     fn find_src_folder() -> Option<PathBuf> {
        let mut current_dir = env::current_dir().ok()?;
        loop {
            if current_dir.join("Cargo.toml").exists() {
                return Some(current_dir.join("src"));
            } else if !current_dir.pop() {
                // If unable to move up, we've reached the root without finding a Cargo.toml file
                break;
            }
        }
        None
    }
     fn from_rel_file_path(rel_path: &str) -> Result<Config, LocCounterError> {
        if let Some(p) = Self::find_src_folder(){
            Ok(Config {
                project_path: p.to_string_lossy().to_string(), // Equates to src folder in this context.
                config_path: Some(p.join(rel_path).to_string_lossy().to_string()),
                extensions: vec!["rs".to_string()],
                count_empty_lines: false,
            })
        } else {
            Err(LocCounterError::ConfigError("Unable to find src folder".to_string()))
        }
    }
     fn from_abs_file_path(config_path: &str, project_path: String) -> Result<Config, LocCounterError> {
        let extension = Path::new(config_path)
            .extension()
            .and_then(|ext| ext.to_str())
            .ok_or_else(|| LocCounterError::ConfigError("Invalid config file type".to_string()))?;

        let mut file = File::open(config_path).map_err(LocCounterError::IoError)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents).map_err(LocCounterError::IoError)?;

        match extension {
            "toml" => {
                #[cfg(feature = "toml-config")]
                return toml::from_str(&contents).map_err(|err: TomlError| LocCounterError::ConfigError(err.to_string()));

                #[cfg(not(feature = "toml-config"))]
                return Err(LocCounterError::ConfigError("TOML config support not enabled".to_string()));
            }
            "yaml" | "yml" => {
                #[cfg(feature = "yaml-config")]
                return serde_yaml::from_str(&contents).map_err(|err: YamlError| LocCounterError::ConfigError(err.to_string()));

                #[cfg(not(feature = "yaml-config"))]
                return Err(LocCounterError::ConfigError("YAML config support not enabled".to_string()));
            }

            "json" => {
                #[cfg(feature = "json-config")]
                return serde_json::from_str(&contents).map_err(|err: JsonError| LocCounterError::ConfigError(err.to_string()));

                #[cfg(not(feature = "json-config"))]
                return Err(LocCounterError::ConfigError("YAML config support not enabled".to_string()));
            }
            "xml" => {
                #[cfg(feature = "xml-config")]
                return quick_xml::de::from_str(&contents).map_err(|err: XmlError| LocCounterError::ConfigError(err.to_string()));

                #[cfg(not(feature = "xml-config"))]
                return Err(LocCounterError::ConfigError("Xml config support not enabled".to_string()));
            },

            #[cfg(feature = "default")]
            "txt" => {
                let txt: Vec<String> = contents.lines().map(|l| l.to_owned()).collect();
                Self::from_str_vec(txt, false, project_path)
            }
            _ => Err(LocCounterError::ConfigError("Unsupported config file type".to_string()))
        }
    }
}
#[cfg(any(feature = "toml-config", feature = "json-config", feature = "yaml-config", feature = "xml-config", feature = "newline-config"))]
impl Config {
    pub fn from_line_separated_string(config: &str, count_empty_lines: bool, project_path: String) -> Result<Config, LocCounterError> {
        let extensions: Vec<String> = config.lines().map(|line| line.to_string()).collect();
        Ok(Config {
            project_path,
            config_path: None,
            extensions,
            count_empty_lines,
        })
    }

    fn from_str_vec(extensions: Vec<String>, count_empty_lines: bool, project_path: String) -> Result<Config, LocCounterError> {
        Ok(Config {
            project_path,
            config_path: None,
            extensions,
            count_empty_lines,
        })
    }

    pub fn builder() -> ConfigBuilder {
        ConfigBuilder::default()
    }
}

impl Default for Config{
    fn default() -> Self {
        let project_path = Self::find_src_folder().unwrap();
        let project_path = project_path.to_string_lossy().to_string();
        Config::from_line_separated_string(CONFIG_TXT_CONTENTS, false, project_path).unwrap()
    }
}


#[derive(Default)]
#[cfg(feature = "default")]
pub struct ConfigBuilder {
    project_path: Option<String>,
    extensions: Vec<String>,
    count_empty_lines: bool,
}

#[cfg(feature = "default")]
impl ConfigBuilder {
    pub fn project_path(mut self, path: &str) -> Self {
        let mut config_path = Config::find_src_folder()
            .unwrap_or_else(|| panic!("Could not find src folder"));
        
        config_path = match Config::is_relative_path(path){
            true => {
                
                match path.to_lowercase().as_str() {
                    "src" | "." => {
                       config_path
                    },
                    _ => {
                        config_path.push(path);
                        config_path
                    }
                }
            },
            false => config_path
        };
        
        self.project_path = Some(config_path.to_string_lossy().to_string());
        self
    }

    pub fn extension(mut self, ext: &str) -> Self {
        self.extensions.push(ext.to_owned());
        self
    }

    pub fn extensions(mut self, exts: Vec<&str>) -> Self {
        exts.into_iter().for_each(|ext| self.extensions.push(ext.to_owned()));
        self
    }

    pub fn count_empty_lines(mut self, count: bool) -> Self {
        self.count_empty_lines = count;
        self
    }

    pub fn build(self) -> Result<Config, &'static str> {
        Ok(Config {
            project_path: self.project_path.ok_or("Project or workspace path is required")?,
            config_path: None,
            extensions: self.extensions,
            count_empty_lines: self.count_empty_lines,
        })
    }
}


//
// End of Configuration section
//
//

///
/// Countroo Logic Section
///
#[cfg(feature = "default")]
// --- Core Logic ---
#[derive(Debug)]
pub struct CountRoo {
    config: Config,
    total_lines: usize,
}

#[cfg(feature = "default")]
impl CountRoo {
    fn new(config: Config) -> Self {
        CountRoo { config, total_lines: 0}
    }

    pub fn count_lines_of_code(&mut self) -> Result<usize, LocCounterError> {
        let extensions =  self.config.extensions.clone();
        
        let config = &self.config.clone();
        let project_path =  PathBuf::from(&self.config.project_path);

        let line_counts = WalkDir::new(project_path)
            .into_iter()
            .filter_map(Result::ok)
            .filter({
                let extensions = extensions.iter()
                    .par_bridge()
                    .map(|e| e.to_lowercase())
                    .collect::<Vec<_>>();
                move |e| {
                     e.file_type().is_file() &&
                        e.path().extension()
                            .and_then(|os_str| os_str.to_str())
                            .map(|ext| extensions.contains(&ext.to_lowercase()))
                            .unwrap_or(false)
                }
            })
            .par_bridge()
            .map(move |entry| {
                    println!("Processing file: {:?}", entry.path().to_string_lossy().to_string());
                    Self::count_lines_for_file(config.count_empty_lines, &entry.into_path().to_string_lossy().to_string()).unwrap_or(0)
            }).collect::<Vec<usize>>();


        for result in line_counts {
            self.total_lines += result;
        }

        Ok(self.total_lines)
    }

    pub fn count_lines_for_file(count_empty_lines: bool, entry: &String) -> Result<usize, LocCounterError> {
        println!("Counting lines for file: {:?}", entry);
        let file = File::open(entry).map_err(LocCounterError::IoError)?;
        let reader = BufReader::new(file);

        let mut count = 0;
        for line in reader.lines() {
            let line = line.map_err(LocCounterError::IoError)?;
            if count_empty_lines || !line.trim().is_empty() {
                count += 1;
            }
        }
        Ok(count)
    }

    pub fn calculate_extension_counts(&self) -> HashMap<String, usize> {
       Self::calculate_extension_counts_for(self.config.count_empty_lines, &self.config.project_path)
    }

    pub fn calculate_extension_counts_for(count_empty_lines: bool, path: &String) -> HashMap<String, usize> {
        let mut extension_counts = HashMap::new();
        for entry in WalkDir::new(path) {
            let entry = entry.unwrap();
            if entry.file_type().is_file() {
                if let Some(ext) = entry.path().extension().and_then(|ext| ext.to_str()) {
                    let count = extension_counts.entry(ext.to_string()).or_insert(0);
                    *count += Self::count_lines_for_file(count_empty_lines, &entry.path().to_string_lossy().to_string()).unwrap_or(0);
                }
            }
        }

        extension_counts
    }
    
    pub fn with_config_builder(config_builder: ConfigBuilder) -> Result<Self, LocCounterError> {
        let config = config_builder.build().map_err(|err| LocCounterError::ConfigError(err.to_string()))?;
        Ok(CountRoo::new(config))
    }

}

#[cfg(feature = "default")]
impl Default for CountRoo {
    fn default() -> Self {
        let project_path = Config::find_src_folder().unwrap();
        let project_path = project_path.to_string_lossy().to_string();
        CountRoo::new(Config::from_abs_file_path("config.txt", project_path).unwrap())
    }
}

#[cfg(feature = "default")]

impl CertainTypesCounter for CountRoo {
    fn count_lines_of_code_for_certain_types(&mut self) -> Result<usize, LocCounterError> {
        self.config.extensions = vec![
           "py".to_string(),
           "js".to_string(),
           "rs".to_string(),
           "dart".to_string(),
           "cpp".to_string(),
           "c".to_string(),
           "rb".to_string(),
           "sh".to_string(),
           "swift".to_string(),
           "ts".to_string(),
           "html".to_string(),
           "css".to_string(),
           "sql".to_string(),
           "cs".to_string(),
           "vb".to_string(),
           "ts".to_string(),
           "go".to_string(),
           "php".to_string(),
           "java".to_string(),
           "kt".to_string(),
           "tsx".to_string(),
           "jsx".to_string(),
           "vue".to_string(),
        ];

        self.count_lines_of_code()
    }
}
#[cfg(feature = "default")]

impl AllTypesCounter for CountRoo {
    fn count_lines_of_code_for_all_types(&mut self) -> Result<usize, LocCounterError> {
        self.count_lines_of_code()
    }
}

#[cfg(feature = "default")]

impl FormattedOutput for CountRoo {
    fn formatted_output(&self) -> String {
        self.total_lines.to_formatted_string(&Locale::en)
    }
}

#[cfg(feature = "tabular-output")]
impl TabularOutput for CountRoo {
    fn tabular_output(&self, output_writer: &dyn OutputWriter) {
        let mut table = Table::new("{:<} | {:<}")
            .with_border_style(BorderStyle::Rounded);
        table.add_row(Row::new().with_cell_str("File Extension").with_cell_str("Lines of Code"));

        let extension_counts = self.calculate_extension_counts();

        for (extension, count) in extension_counts {
            table.add_row(Row::new().with_cell_str(extension.as_str()).with_cell_str(count.to_formatted_string(&Locale::en).as_str()));
        }

        #[cfg(feature = "colored-output")]{
            let color_map = LanguageBrandings::default().color_map;
            let colored_table = self.apply_colors_to_table(&table, &color_map);
            output_writer.write(&colored_table.to_string()).unwrap()
        }

        #[cfg(not(feature = "colored-output"))]{
            output_writer.write(&table.to_string()).unwrap()
        }

    }
}

#[cfg(feature = "colored-output")]
impl ColorizedOutput for CountRoo{
    fn apply_colors_to_table(&self, table: &Table, color_map: &[LanguageColorMapping<'_>]) -> ColoredString {
        let mut colored_table = String::new();

        // Helper function to get color from an extension
        fn get_color(extension: &str, color_map: &[LanguageColorMapping]) -> Option<Color> {
            color_map.iter().find_map(|mapping| {
                if mapping.extension == extension {
                    match mapping.color {
                        AccentColor::Hex(hex_code) => Some(Color::from_str(hex_code).unwrap_or(Color::TrueColor {r:255, g:255, b:255 })),
                    }
                } else {
                    None
                }
            })
        }

        // Iterate over rows
        for row in table.get_rows() {
            let mut colored_row = String::new();

            // Iterate over cells
            for mut cell in row.get_cells() {
                if let Some(color) = get_color(&cell.get_contents(), color_map) {
                    cell.color = Some(color);
                    colored_row.push_str(&cell.get_colored_content());
                } else {
                    colored_row.push_str(&cell.get_contents());
                }
                colored_row.push_str("  ");
            }

            colored_table.push_str(&colored_row);
            colored_table.push('\n');
        }

        colored_table.into()
    }
}

#[cfg(feature = "default")]
macro_rules! countroo_common {
    ($counter:ident) => {
        let mut countroo = CountRoo::new(Config::default()).unwrap();
        $countroo.count_lines_of_code_for_certain_types()?
    }
}

#[cfg(feature = "default")]
macro_rules! countroo_all {
    ($counter:ident) => {
        let mut countroo = CountRoo::new(Config::from_abs_file_path("config.txt").unwrap());
        $countroo.count_lines_of_code_for_all_types()?
    }
}
//
// End of Countroo Section!
//
//

///
/// Tests Section
///
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_lines_of_code_for_certain_types() {
        let config: Config = Config {
            project_path: "src".to_string(),
            config_path: None,
            extensions: vec!["rs".to_string()],
            count_empty_lines: false,
        };
        let mut counter = CountRoo::new(config);
        let result = counter.count_lines_of_code_for_certain_types().unwrap();
        assert_ne!(result, 0);
        println!("Code Analysis:\n {:?}", counter.tabular_output(&StdoutWriter{}));
    }

    #[test]
    fn test_count_lines_of_code_for_all_types() {
        let config: Config = Config::from_rel_file_path("config.txt").unwrap();
        let mut counter = CountRoo::new(config);
        let result = counter.count_lines_of_code_for_all_types().unwrap_or(0);
        assert_ne!(result, 0);
        println!("Code Analysis:\n {:?}", counter.tabular_output(&StdoutWriter{}));
    }

    #[test]
    fn test_builder_pattern_produces_same_object_with_extension(){
        let src_path = Config::find_src_folder().unwrap();
        let config = Config::builder()
            .project_path("src")
            .extension("rs")
            .extension("py")
            .count_empty_lines(false)
            .build()
            .unwrap();

        let config2 = Config {
            project_path: src_path.join("src").to_string_lossy().to_string(),
            config_path: None,
            extensions: vec!["rs".to_string(),"py".to_string()],
            count_empty_lines: false,
        };

        assert_eq!(config.project_path, config2.project_path);
        assert_eq!(config.extensions, config2.extensions);
        assert_eq!(config.count_empty_lines, config2.count_empty_lines);
    }

    #[test]
    fn test_builder_pattern_produces_same_object_with_extensions(){
        let config = Config::builder()
            .project_path("src")
            .extensions(vec!["rs","py"])
            .count_empty_lines(false)
            .build()
            .unwrap();

        let config2 = Config {
            project_path: PathBuf::from("E:\\RustRoverProjects\\countroo\\src").to_string_lossy().to_string().replace('\\', "/") ,
            config_path: None,
            extensions: vec!["rs".to_string(),"py".to_string()],
            count_empty_lines: false,
        };

        assert_eq!(config.project_path, config2.project_path);
        assert_eq!(config.extensions, config2.extensions);
        assert_eq!(config.count_empty_lines, config2.count_empty_lines);
    }
}