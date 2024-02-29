pub mod output_adapters;
#[cfg(feature = "tabular-output")]
#[allow(unused_imports)]
use prettytable::{table, Table, Row, Cell, row};

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
use std::fmt::Display;
use std::path::{Path, PathBuf};
use cargo_toml_workspace::cargo_toml;

use rayon::prelude::*;
use serde::Deserialize;

#[cfg(feature = "default")]
use output_adapters::*;

#[cfg(feature = "toml-config")]
use toml::de::Error as TomlError;
#[cfg(feature = "json-config")]
use serde_json::Error as JsonError;
#[cfg(feature = "yaml-config")]
use serde_yaml::Error as YamlError;
#[cfg(feature = "xml-config")]
use quick_xml::de::DeError as XmlError;

#[cfg(feature = "default")]
pub const CONFIG_TXT_CONTENTS: &str = include_str!("config.txt");

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
pub trait PathHelpers{
    fn is_relative_path(path: &str) -> bool;
    fn find_src_folder() -> Option<PathBuf>;
    #[allow(dead_code)]
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
    TomlError(String),
    JsonError(String),
    YamlError(String),
    XmlError(String),
}

#[cfg(feature = "default")]
impl fmt::Display for LocCounterError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            LocCounterError::IoError(err) => write!(f, "IO Error: {}", err),
            LocCounterError::ConfigError(msg) => write!(f, "Configuration Error: {}", msg),
            LocCounterError::JsonError(err) => write!(f, "JSON Error: {}", err),
            LocCounterError::YamlError(err) => write!(f, "YAML Error: {}", err),
            LocCounterError::XmlError(err) => write!(f, "XML Error: {}", err),
            LocCounterError::TomlError(msg) => write!(f, "TOML Error: {}", msg),
        }
    }
}

#[cfg(feature = "default")]
impl Error for LocCounterError {}


#[cfg(feature = "toml-output")]
impl From<toml::de::Error> for LocCounterError{
    fn from(err: toml::de::Error) -> Self {
        LocCounterError::TomlError(err.to_string())
    }
}
///
/// Configuration section
///
/// `Config` - The Heart of `CountRoo`'s Configuration 🖤🔧
///
/// Embodies the core settings that guide `CountRoo` in its quest to analyze your Rust project.
/// Whether you're diving into TOML, JSON, YAML, XML, or even plain text configurations, `Config`
/// stands ready to adapt, ensuring `CountRoo` knows exactly what to look for and where to start.
///
/// ## Features Overview
/// - **Project Source Path**: Marks the trailhead for `CountRoo`'s exploration, pointing to
///   your project's source directory. 🚩
/// - **Config Path**: Optionally specifies the path to an external configuration file, allowing
///   for even more tailored analysis settings. 🗂️ (Enabled with `toml-config`, `json-config`,
///   `yaml-config`, `xml-config`, or `newline-config` features.)
/// - **Extensions**: A list of file extensions `CountRoo` should consider during its analysis,
///   ensuring nothing important gets overlooked. 📚
/// - **Count Empty Lines**: A boolean toggle that dictates whether `CountRoo` counts those
///   silent, empty lines as part of the total. 📉🚫
///
/// ## Configuration Flexibility
/// Enabled with various feature flags, `Config` can morph to understand different configuration
/// formats, making it a versatile companion for any Rust project setup. Whether you prefer the
/// simplicity of a TOML file, the agility of JSON, or the elegance of YAML, `Config` is your
/// chameleon. 🦎
///
/// ## Example Usage
/// ```rust
/// // Assuming `toml-config` feature is enabled
/// use countroo::prelude::*;
/// let config = Config::default();
/// ```
///
/// `Config` is where your `CountRoo` journey begins, setting the stage for a comprehensive
/// code analysis tailored to your project's unique landscape. 🌄🔍
#[cfg(any(feature = "toml-config", feature = "json-config", feature = "yaml-config", feature = "xml-config", feature = "newline-config"))]
#[derive(Debug, Deserialize, Eq, PartialEq, Clone)]
pub struct Config {
    pub project_src_path: String,
    #[cfg(any(feature = "toml-config", feature = "json-config", feature = "yaml-config", feature = "xml-config", feature = "newline-config"))]
    pub config_path: Option<String>,
    pub extensions: Vec<String>,
    pub count_empty_lines: bool,
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
                project_src_path: p.to_string_lossy().to_string(), // Equates to src folder in this context.
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
    /// Creates a `Config` from a newline-separated string of file extensions, alongside flags
    /// for counting empty lines and specifying the project path. 📝↩️🛠️
    ///
    /// This handy method allows for a quick setup of your `Config` directly from a string,
    /// where each line represents a new file extension `CountRoo` should consider during its
    /// exploration. It's akin to giving `CountRoo` a list of destinations for its code counting
    /// journey, written on a scroll. 📜🗺️
    ///
    /// ## Parameters
    /// - `config`: A string with each line specifying a file extension to include in the analysis.
    ///   This method treats each line as a separate extension, transforming your string into a
    ///   ticket for `CountRoo` to explore files of those types.
    /// - `count_empty_lines`: A boolean indicating whether to count empty lines within files.
    ///   This toggles whether `CountRoo` counts all lines or just those with content.
    /// - `project_path`: The starting point for `CountRoo`'s adventure, specified as a path
    ///   to the project's source directory.
    ///
    /// ## Returns
    /// - `Ok(Config)`: A `Config` ready to guide `CountRoo` on a tailored exploration, equipped
    ///   with your specified preferences.
    /// - `Err(LocCounterError)`: An error detailing what went wrong during the `Config` creation.
    ///
    /// ## Example Usage
    /// ```rust
    /// use countroo::Config;
    /// let config_str = "rs\ncs\ndart\ngo\njs\nts\n";
    /// let custom_config = Config::from_line_separated_string(
    ///     config_str,
    ///     true,
    ///     "path/to/my/project".to_string()
    /// )
    /// .expect("Failed to create a Config from the provided string");
    /// ```
    ///
    /// Utilize `from_line_separated_string` to easily convert a simple string into a complex
    /// `Config` for `CountRoo`, ensuring your analysis is as detailed or as broad as you wish. 🌟🔍
    pub fn from_line_separated_string(config: &str, count_empty_lines: bool, project_path: String) -> Result<Config, LocCounterError> {
        let extensions: Vec<String> = config.lines().map(|line| line.to_string()).collect();
        Ok(Config {
            project_src_path: project_path,
            config_path: None,
            extensions,
            count_empty_lines,
        })
    }

    /// Constructs a `Config` from a vector of file extensions, a flag for counting empty lines,
    /// and a project path. 🛠️📋
    ///
    /// This method allows you to directly create a `Config` object from the ground up, specifying
    /// exactly which file types `CountRoo` should keep an eye on, whether to count empty lines,
    /// and where to start the code counting expedition. It's like packing your backpack with
    /// just the essentials for a hike through your codebase. 🎒🌲
    ///
    /// ## Parameters
    /// - `extensions`: A vector of `String`s representing the file extensions to be included in
    ///   the analysis. Think of each extension as a ticket inviting `CountRoo` to consider files
    ///   of that type. 🎟️
    /// - `count_empty_lines`: A boolean flag indicating whether empty lines should be counted
    ///   as part of the total lines of code. It's the difference between counting every blade
    ///   of grass in the field or just the flowers. 🌼🚫🌱
    /// - `project_path`: A `String` specifying the path to your project's source directory. This
    ///   is where `CountRoo` will start its journey, binoculars in hand. 🗺️🔭
    ///
    /// ## Returns
    /// - `Ok(Config)`: A fully equipped `Config` ready to guide `CountRoo` on its analytical
    ///   adventure.
    /// - `Err(LocCounterError)`: An error if something goes awry during the creation process.
    ///
    /// ## Example Usage
    /// ```rust
    /// use countroo::prelude::*;
    /// let custom_config = Config::from_str_vec(
    ///     vec!["rs".to_string(), "toml".to_string()],
    ///     true,
    ///     "path/to/my/rust/project".to_string(),
    /// )
    /// .expect("Failed to create custom Config");
    /// ```
    ///
    /// With `from_str_vec`, setting up `CountRoo` for a tailored code analysis is as straightforward
    /// as plotting a course on a map. Just decide where you're going, what you're looking for, and
    /// whether every little detail counts. 🌟🧭
    pub fn from_str_vec(extensions: Vec<String>, count_empty_lines: bool, project_path: String) -> Result<Config, LocCounterError> {
        Ok(Config {
            project_src_path: project_path,
            config_path: None,
            extensions,
            count_empty_lines,
        })
    }

    /// Constructs a new `ConfigBuilder` to kickstart your `CountRoo` configuration journey. 🚀🛠️
    ///
    /// This function is your gateway to creating a custom `Config` for `CountRoo`, allowing
    /// you to fluently specify project settings, file extensions to analyze, and whether to
    /// count those sneaky empty lines. It's like giving you the keys to the `CountRoo` rover;
    /// you decide where it goes and what it looks for! 🗝️🔍
    ///
    /// ## Returns
    /// A fresh `ConfigBuilder` instance with default settings, ready to be tailored to your
    /// project's needs.
    ///
    /// ## Example Usage
    /// ```rust
    /// use countroo::prelude::*;
    /// let my_config_builder = Config::builder()
    ///     .project_path("path/to/my/awesome/project")
    ///     .extension("rs")
    ///     .count_empty_lines(true)
    ///     .build()
    ///     .expect("Could not build config - check your paths and settings!");
    /// ```
    ///
    /// Start with `builder()`, chain your configurations, and build. It's that simple to
    /// prep `CountRoo` for a code counting adventure tailored just for you! 🌟🛠️
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


/// `ConfigBuilder` - The Architect Behind Your `CountRoo` Configuration 🏗️📐
///
/// Craft the perfect configuration for your `CountRoo` analysis with this builder.
/// Whether you're setting up a simple count or gearing up for a deep dive into your project's
/// structure, `ConfigBuilder` paves the way for a customized analysis experience. Think of it
/// as laying out the blueprint for `CountRoo`'s adventure through your codebase. 🗺️✨
///
/// ## Features
/// - **Project Path**: Direct `CountRoo` to the heart of your project. Whether it's the root
///   or a specific folder, just set the path, and off you go! 🚀
/// - **Extensions**: Specify which file extensions should catch `CountRoo`'s eye. Rust files?
///   Markdown? Configuration files? You decide! 📂👀
/// - **Count Empty Lines**: To count or not to count empty lines, that is the question.
///   With `ConfigBuilder`, the choice is yours. 📄🚫📄
///
/// ## Usage
/// Building your `Config` is as easy as chaining method calls with `ConfigBuilder`. Here’s a
/// quick example to get you started:
///
/// ```rust
/// use countroo::prelude::*;
/// let config = ConfigBuilder::default()
///     .project_path("src")
///     .extension("rs")
///     .count_empty_lines(true)
///     .build()
///     .expect("Failed to build the config. Please check your settings!");
/// ```
///
/// With `ConfigBuilder`, you're the master planner behind `CountRoo`'s analytical journey.
/// Set your preferences, build your config, and let the code counting commence! 🎉🔧
#[derive(Default)]
#[cfg(feature = "default")]
pub struct ConfigBuilder {
    project_path: Option<String>,
    extensions: Vec<String>,
    count_empty_lines: bool,
}

#[cfg(feature = "default")]
impl ConfigBuilder {
    /// Sets the project path for `CountRoo`'s code exploration journey. 🚀🗂️
    ///
    /// This method defines the starting point of your project, guiding `CountRoo` on where
    /// to begin its analysis. Whether it's nestled in your project's root or tucked away in
    /// a subdirectory, point `CountRoo` in the right direction, and it'll happily hop to it.
    ///
    /// ## Parameters
    /// - `path`: A string slice (`&str`) indicating the path to your project's source directory.
    ///   It can be an absolute path or a relative path from the location of `Cargo.toml`.
    ///
    /// ## Behavior
    /// - **Relative Paths**: If you pass a relative path, `CountRoo` smartly resolves it based
    ///   on the location of your `Cargo.toml` file. It's like giving `CountRoo` a little map. 🗺️
    /// - **Absolute Paths**: Direct `CountRoo` with confidence if you've got an absolute path.
    ///   It'll head straight there, no questions asked. 🏃‍♂️💨
    ///
    /// ## Returns
    /// Keeps the builder pattern grooving by returning `self`, allowing for more method chaining.
    ///
    /// ## Example Usage
    /// ```rust
    /// use countroo::prelude::*;
    /// let config = ConfigBuilder::default()
    ///     .project_path("src") // Directing to the src directory
    ///     .extension("rs") // We're only interested in Rust files
    ///     .count_empty_lines(true) // Counting every line, even the empty ones
    ///     .build()
    ///     .expect("Building config failed. Oh no!");
    /// ```
    ///
    /// By setting the project path, you ensure `CountRoo` starts its adventure from the
    /// correct location, making your code analysis as accurate and helpful as possible. 🌟
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

    /// Adds a single file extension to the list for code analysis consideration. 📄🔍
    ///
    /// Tailor `CountRoo`'s adventurous path by specifying individual file types it should
    /// pay attention to. Perfect for when you want to focus on just one more extension
    /// without disturbing the existing list. Like inviting just one more friend to your
    /// coding party! 🎉
    ///
    /// ## Parameters
    /// - `ext`: A string slice (`&str`) representing a single file extension to include
    ///   in the analysis. Remember, no leading dot (`.`) needed!
    ///
    /// ## Returns
    /// Returns `self`, keeping the builder pattern fluent and fabulous. 🔄
    ///
    /// ## Example Usage
    /// ```rust
    /// use countroo::prelude::*;
    /// let config = ConfigBuilder::default()
    ///     .project_path("/path/to/glorious/project")
    ///     .extension("rs") // Just focusing on Rust files here!
    ///     .count_empty_lines(true) // Every line counts, even the empty ones!
    ///     .build()
    ///     .unwrap_or_else(|_| panic!("Failed to build the Config. This is not a drill!"));
    /// ```
    ///
    /// By specifying the types of files to analyze, you make `CountRoo`'s job both
    /// easier and more aligned with your project's needs. Let the targeted analysis begin! 🎯🚀
    pub fn extension(mut self, ext: &str) -> Self {
        self.extensions.push(ext.to_owned());
        self
    }

    /// Adds a list of file extensions to be included in the code analysis. 📁✨
    ///
    /// This method allows you to specify which file types `CountRoo` should consider
    /// when it embarks on its code counting journey. Whether you're looking at Rust files,
    /// Markdown documents, or even configuration files, just list their extensions here.
    ///
    /// ## Parameters
    /// - `exts`: A vector of string slices (`&str`) representing the file extensions
    ///   to include in the analysis, without the leading dot (`.`).
    ///
    /// ## Returns
    /// Returns `self`, enabling the delightful chaining of builder method calls. 🔄
    ///
    /// ## Example Usage
    /// ```rust
    /// use countroo::prelude::*;
    /// let config = ConfigBuilder::default()
    ///     .project_path("/path/to/project")
    ///     .extensions(vec!["rs", "md", "toml"]) // Rustacean essentials!
    ///     .count_empty_lines(false) // Skipping those empty lines.
    ///     .build()
    ///     .expect("Oops! Failed to construct the Config");
    /// ```
    ///
    /// Expand your analysis horizon by including a variety of file types,
    /// making `CountRoo` your versatile code exploration companion. 🌐🔍
    pub fn extensions(mut self, exts: Vec<&str>) -> Self {
        exts.into_iter().for_each(|ext| self.extensions.push(ext.to_owned()));
        self
    }

    /// Sets the flag to count (or not count) empty lines in the code analysis. 📝✔️❌
    ///
    /// This method configures whether `CountRoo` should consider empty lines as part of
    /// its line counting expedition across your project. It's like choosing whether to
    /// count the spaces between the trees in a forest. 🌳🌲
    ///
    /// ## Parameters
    /// - `count`: A boolean value where `true` signals `CountRoo` to count every single line,
    ///   including those sneaky empty ones, and `false` means skipping over those airy voids.
    ///
    /// ## Returns
    /// Returns `self`, allowing for fluent-style chaining of builder methods. 🔄
    ///
    /// ## Example Usage
    /// ```rust
    /// use countroo::prelude::*;
    /// let config = ConfigBuilder::default()
    ///     .project_path("/path/to/project")
    ///     .extension("rs")
    ///     .count_empty_lines(true) // We care about every line, even the empty ones!
    ///     .build()
    ///     .expect("Config build failed");
    /// ```
    ///
    /// Whether your coding style is verbose, with plenty of breathing room, or compact,
    /// with not an empty line in sight, `CountRoo` adapts to your preferences. 🎨
    pub fn count_empty_lines(mut self, count: bool) -> Self {
        self.count_empty_lines = count;
        self
    }

    /// Builds the `Config` object from the builder pattern setup. 🏗️🛠️
    ///
    /// Consumes the builder to produce a finalized `Config` instance, encapsulating all
    /// the project-specific settings necessary for the `CountRoo` analysis. This function
    /// checks that all required fields are set, ensuring the builder is in a valid state
    /// before attempting to construct a `Config` object.
    ///
    /// ## Returns
    /// - `Ok(Config)`: A shiny new `Config` object ready to guide `CountRoo` on its journey 🧭.
    /// - `Err(&'static str)`: A not-so-shiny error message indicating what went wrong during the build process. 😓
    ///
    /// ## Example Usage
    /// ```rust
    /// use countroo::prelude::*;
    /// let builder = ConfigBuilder::default()
    ///     .project_path("path/to/my/rust/project")
    ///     .extension("rs")
    ///     .count_empty_lines(true);
    ///
    /// let config = builder.build().expect("Failed to build config");
    /// ```
    ///
    /// Ensure your project path is set; otherwise, this build will politely refuse to proceed,
    /// citing a lack of directions. 🚫🗺️
    pub fn build(self) -> Result<Config, &'static str> {
        Ok(Config {
            project_src_path: self.project_path.ok_or("Project or workspace path is required")?,
            config_path: None,
            extensions: self.extensions,
            count_empty_lines: self.count_empty_lines,
        })
    }
}


/// `Exportable` 📤 - Making Your Data Dance Across the Output!
///
/// A trait for the adventurous Rustaceans who want their data to leap out of the console
/// and into the wild! Implement `Exportable` to enable your structs to export their data,
/// whether it's to a file, a terminal, or across the seven seas of your application. 🌊📄
///
/// ## Functionality
/// - **export**: Takes your data on a journey from the internal state to an external representation,
/// allowing it to be written, viewed, and shared far and wide. 🌍
///
/// ## Requirements
/// Implementors must define how their data is exported, providing a writer that adheres to the
/// `OutputWriter` trait. This writer handles the specifics of the output format and destination.
///
/// ## Example
/// ```rust
/// use countroo::prelude::*;
/// struct MyExportableData {
///     // Your data fields here
/// }
///
/// impl Exportable for MyExportableData {
///     fn export(&self, writer: Box<dyn OutputWriter>) -> Result<(), std::io::Error> {
///         // Implement how your data is written using the provided writer
///         // For example, converting your data to JSON, CSV, or plain text
///         todo!("Implement the export logic here")
///     }
/// }
///
/// ```
///
/// With `Exportable`, your data isn't just stored; it's a story waiting to be told.
pub trait Exportable {
    fn export(&self, writer: Box<dyn OutputWriter>) -> Result<(), io::Error>;
}

/// `Analyzable` 🔍 - Unearthing the Secrets of Your Rust Project!
///
/// For Rustaceans who love to dig deeper into their projects, `Analyzable` is the treasure map 🗺️
/// that leads to insights galore. Implement this trait to analyze your Rust project's structure,
/// dependencies, and more, uncovering information that helps you navigate the coding journey ahead. 🚢
///
/// ## Capabilities
/// - **Get Project Base**: Finds the base directory of your project. 🏠
/// - **Is Code File**: Determines if a given path points to a source code file. 📄✅
/// - **Get Manifest**: Retrieves the `Cargo.toml` manifest of your project. 📦
/// - **Count Code Files**: Tallies the number of source code files in your project. 🗂️
/// - **Count Crates**: Counts the crates on which your project depends. 🛠️
/// - **Get Project Name**: Extracts the name of your project from its manifest. 📛
/// - **Get Rust Version**: Determines the version of Rust used by your project. 🦀
/// - **Get Rust Edition**: Identifies the Rust edition your project adheres to. 📚
/// - **Count Rust Modules**: Counts the Rust modules within your project. 🧩
/// - **Analyze Code Base**: Performs a comprehensive analysis of your project's code base. 🕵️‍♂️
///
/// ## Example Implementation
/// ```rust
/// use countroo::prelude::*;
/// struct MyProjectAnalyzer {
///     // Project-specific fields
/// }
///
/// impl Analyzable for MyProjectAnalyzer {
///     fn get_project_base(&self) -> String {
///         todo!()
///     }
///
///     fn is_code_file(&self, path: &str) -> bool {
///         todo!()
///     }
///
///     fn get_manifest(&self) -> Result<Manifest, LocCounterError> {
///         todo!()
///     }
///
///     fn count_code_files(&self, project_path: &str) -> Result<usize, LocCounterError> {
///         todo!()
///     }
///
///     fn count_crates(&self) -> Result<usize, LocCounterError> {
///         todo!()
///     }
///
///     fn get_project_name(&self) -> Result<Option<String>, LocCounterError> {
///         todo!()
///     }
///
///     fn get_rust_version() -> Option<String> {
///         todo!()
///     }
///
///     fn get_rust_edition(&self) -> Option<String> {
///         todo!()
///     }
///
///     fn count_rust_modules(project_path: &str) -> Result<usize, LocCounterError> {
///         todo!()
///     }
///
///     fn analyze_code_base(&mut self) {
///         todo!()
///     }
///     // Implement the rest of the methods as per your project's requirements...
/// }
/// ```
///
/// Whether you're looking to optimize your project structure, curious about your dependency graph,
/// or just want to know how many lines of Rust you've written, `Analyzable` is your key to unlocking
/// those insights. Let the analysis begin! 🔎🚀
pub trait Analyzable{
    fn get_project_base(&self) -> String;
    fn is_code_file(&self, path: &str) -> bool;
    fn get_manifest(&self) -> Result<cargo_toml::Manifest, LocCounterError>;
    fn count_code_files(&self, project_path: &str) -> Result<usize, LocCounterError>;
    fn count_crates(&self) -> Result<usize, LocCounterError>;
    fn get_project_name(&self) -> Result<Option<String>, LocCounterError>;
    fn get_rust_version() -> Option<String>;
    fn get_rust_edition(&self) -> Option<String>;
    fn count_rust_modules(project_path: &str) -> Result<usize, LocCounterError>;
    fn analyze_code_base(&mut self);
}
//
// End of Configuration section
//
//

/// `CountRoo` 🦘 - Your Rust Project's Best Mate for Code Insights!
///
/// Embark on an adventure through your codebase with `CountRoo`, the struct that leaps across
/// your files, counting lines of code, tracking crates, and more, all while keeping it fun! 🎉
///
/// ## Features
/// - **Total Lines of Code** 📊: Keeps track of the total lines across your project.
/// - **Number of Files** 🗂️: Counts the files explored in the code safari.
/// - **Crate Count** 📦: Tallies up the crates your project depends on.
/// - **Project Name** 🏷️: Captures the name of your project, if available.
/// - **Rust Edition** 🦀: Identifies the Rust edition your project is using.
/// - **Rust Compiler Version** 🔧: Determines the version of the Rust compiler (`rustc`) in use.
/// - **Module Count** 📚: Counts the number of modules within your Rust project.
///
/// ## Example
/// ```rust
/// use countroo::prelude::*;
/// let config = Config::from_rel_file_path("path/to/config").unwrap();
/// let mut countroo = CountRoo::default();
/// let total_lines = countroo.count_lines_of_code().unwrap();
/// println!("Total lines of code: {}", total_lines);///
///
/// // Or you could just use macros for convenience such as count_some!() or count_it_all!()
/// ```
///
/// Dive in and let `CountRoo` uncover the hidden gems and forgotten fossils in your codebase. Happy counting! 🚀
#[cfg(feature = "default")]
// --- Core Logic ---
#[derive(Debug)]
pub struct CountRoo {
    pub config: Config,
    pub total_lines: usize,
    pub num_files: usize,
    pub num_crates: usize,
    pub project_name: Option<String>,
    pub rust_edition: Option<String>,
    pub rustc_version: Option<String>,
    pub num_modules: usize,
}

impl Exportable for CountRoo {
    fn export(&self, writer: Box<dyn OutputWriter>) -> Result<(), io::Error> {
        writer.write(self.to_string().as_str())
    }
}

#[cfg(feature = "default")]
impl CountRoo {
    fn new(config: Config) -> Self {
        let mut countroo = CountRoo { config, total_lines: 0, num_files: 0, num_crates: 0, project_name: None, rust_edition: None, rustc_version: None, num_modules: 0 };
        countroo.analyze_code_base();
        countroo
    }

    pub fn count_lines_of_code(&mut self) -> Result<usize, LocCounterError> {
        let extensions =  self.config.extensions.clone();
        
        let config = &self.config.clone();
        let project_path =  PathBuf::from(&self.config.project_src_path);

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
                    Self::count_lines_for_file(config.count_empty_lines, &entry.into_path().to_string_lossy().to_string()).unwrap_or(0)
            }).collect::<Vec<usize>>();


        for result in line_counts {
            self.total_lines += result;
        }

        Ok(self.total_lines)
    }

    pub fn count_lines_for_file(count_empty_lines: bool, entry: &String) -> Result<usize, LocCounterError> {
        // Return early if the path is not a file, just in case ( Fallback Guard ).
        if !Path::new(entry).is_file() {
            return Ok(0);
        }
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


    /// Calculates and aggregates the lines of code per occurrences of each file extension within the project's source directory.
    ///
    /// # Arguments
    ///
    /// * `self`: A reference to the current instance of the struct.
    ///
    /// # Returns
    ///
    /// A HashMap where:
    /// * Keys are strings representing unique file extensions found.
    /// * Values are the corresponding counts of files with that extension.
    ///
    /// # Assumptions
    ///
    /// * The `config` member of the struct holds relevant configuration data.
    /// * `config.count_empty_lines` influences the counting behavior (Toggling it to true marks whitespace new lines as code lines).
    /// * `config.project_src_path` defines the base directory for searching files (Usually src folder).
    pub fn calculate_extension_counts(&self) -> HashMap<String, usize> {
       Self::calculate_extension_counts_for(self.config.count_empty_lines, &self.config.project_src_path)
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

impl Analyzable for CountRoo{

    fn get_project_base(&self) -> String {
        PathBuf::from(&self.config.project_src_path).parent().unwrap().to_string_lossy().to_string()
    }
    fn is_code_file(&self, path: &str) -> bool {
        Path::new(path).extension().map(|ext| self.config.extensions.contains(&ext.to_string_lossy().to_string()))
            .unwrap_or(false)
    }

    fn get_manifest(&self) -> Result<cargo_toml::Manifest, LocCounterError> {
        let cargo_toml_path = Path::new(self.get_project_base().as_str()).join("Cargo.toml");
        if !cargo_toml_path.exists() {
            return Err(LocCounterError::TomlError("Cargo.toml not found".to_string()));
        }

        let contents = std::fs::read_to_string(cargo_toml_path).unwrap();
        let manifest: cargo_toml::Manifest = cargo_toml::Manifest::from_str(&contents).map_err(|err| LocCounterError::TomlError(err.to_string()))?;
        Ok(manifest)
    }

    fn count_code_files(&self, project_path: &str) -> Result<usize, LocCounterError> {
        let mut count = 0;
        for entry in WalkDir::new(project_path).into_iter().flatten() {
            if entry.file_type().is_file() && self.is_code_file(entry.path().to_string_lossy().as_ref()) {
                count += 1;
            }
        }
        Ok(count)
    }

    fn count_crates(&self) -> Result<usize, LocCounterError> {
        Ok(self.get_manifest()?.dependencies.len() + 1)
    }

    fn get_project_name(&self) -> Result<Option<String>, LocCounterError> {
        Ok(Some(self.get_manifest()?.package.unwrap().name.to_string()))
    }

    fn get_rust_version() -> Option<String> {
        let rustc_version = std::process::Command::new("rustc")
            .arg("--version")
            .output()
            .ok()?;

        let rustc_version = String::from_utf8(rustc_version.stdout).ok()?;
        let rustc_version = rustc_version.split_whitespace().collect::<Vec<&str>>()[1].to_string();
        Some(rustc_version)
    }

    fn get_rust_edition(&self) -> Option<String> {
        let manifest = self.get_manifest().map_err(|err| LocCounterError::TomlError(err.to_string())).unwrap();
        match manifest.package.unwrap().edition.unwrap(){
            cargo_toml::Edition::E2015 => Some("2015".to_string()),
            cargo_toml::Edition::E2018 => Some("2018".to_string()),
            cargo_toml::Edition::E2021 => Some("2021".to_string()),
            #[allow(unreachable_patterns)]
            _ => Some("2024".to_string()),
        }
    }

    fn count_rust_modules(project_path: &str) -> Result<usize, LocCounterError> {
        let mut count = 0;
        for entry in WalkDir::new(project_path)
            .into_iter()
            .filter_map(Result::ok)
            .filter(|e| e.file_type().is_file() && e.path().extension().map(|ext| ext == "rs").unwrap_or(false))
        {
            let contents = std::fs::read_to_string(entry.path()).expect("Failed to analyze whether a file is a rust module or not.");
            count += contents.matches("mod ").count();
        }
        Ok(count)
    }

    fn analyze_code_base(&mut self){
        let project_base = self.get_project_base();
        self.num_files = self.count_code_files(project_base.as_str()).expect("Failed to count code files");
        self.num_crates = self.count_crates().expect("Failed to count crates");
        self.project_name = self.get_project_name().expect("Failed to get project name");
        self.rustc_version = Self::get_rust_version();
        self.num_modules = Self::count_rust_modules(project_base.as_str()).expect("Failed to count modules");
        self.rust_edition = self.get_rust_edition();
    }
}

#[cfg(feature = "default")]
impl Default for CountRoo {
    fn default() -> Self {
        CountRoo::new(Config::from_rel_file_path("config.txt").unwrap())
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

impl Display for CountRoo{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let extension_counts = self.calculate_extension_counts();
        let total_count = extension_counts.values().sum::<usize>();

        let s1_header = row!["Project ", "Module #", "Dependency #"];
        let s1_data = row![self.project_name.as_ref().unwrap_or(&self.get_project_name().unwrap().unwrap()), self.num_modules, self.num_crates];
        let s2_footer = row!["Rust Edition", "Rustc Version", "Countroo Version"];
        let s2_data = row![self.rust_edition.as_ref().unwrap_or(&"None".to_string()), self.rustc_version.as_ref().unwrap_or(&"None".to_string()), "0.1.0"];
        let c_header = row!["Language", "Count", "Percentage %"];

        let mut tbl = Table::new();
        tbl.add_row(s1_header);
        tbl.add_row(s1_data);
        tbl.add_row(c_header);

        for (extension, count) in extension_counts {
            let formatted_extension = format!("> {}", extension);
            let formatted_count = count.to_formatted_string(&Locale::en);
            let percentage = ((count as f32/total_count as f32) * 100.0).round();
            let formatted_percentage = format!("{}%", percentage);

            tbl.add_row(row![formatted_extension, formatted_count, formatted_percentage]);
        }
        tbl.add_row(s2_footer);
        tbl.add_row(s2_data);
        write!(f, "{}", tbl)
    }

}

#[cfg(feature = "default")]
#[allow(unused_macros)]
#[macro_export]
macro_rules! count_some_generic {
    ($folder:ident) => {
        let mut counter = $crate::CountRoo::new(Config::default());
        counter.config.project_src_path = $folder.to_string_lossy().to_string();
        counter.count_lines_of_code_for_certain_types().expect("Failed to count lines of code");
        let writer_boxed = Box::<$crate::output_adapters::StdoutWriter>::default();
        counter.export(writer_boxed).expect("Failed to export to Stdout");
    }
}


/// Executes a selective line count using `CountRoo` with the default configuration. 📊🔍
///
/// This macro simplifies the process of counting lines of code for certain types within your
/// project using a predefined set of file extensions. It's like sending `CountRoo` on a
/// specific mission with a map and a checklist, ensuring it focuses on the files that matter
/// most to you.
///
/// Under the hood, it creates a new `CountRoo` instance with default settings, initiates the
/// counting process for specified file types, and then attempts to export the results to stdout.
/// It's a quick and easy way to get insights without getting bogged down in configuration details.
///
/// ## Requirements
/// - The `default` feature must be enabled for this macro to be available.
/// - `CountRoo`, `Config`, and a suitable `OutputWriter` (like `StdoutWriter`) must be defined
///   and available in your project.
///
/// ## Example Usage
/// Simply invoke the macro in your code where you need to perform the count:
/// ```rust
/// // Inside build.rs
/// use countroo::prelude::*;
/// fn main() {
///     count_some!();
///     // If you're using it for a workspace, you can just use it like this inside any project's build.rs
///     countroo::count_some!(workspace: true);
/// }
/// ```
///
/// This macro is perfect for quick checks or when integrating `CountRoo` into larger workflows
/// where you want a hassle-free setup. Just call `count_some!` and let `CountRoo` handle the rest,
/// delivering a concise report directly to your console. 🖨️✨
#[cfg(feature = "default")]
#[allow(unused_macros)]
#[macro_export]
macro_rules! count_some {
    () => {
       let pp = Config::find_src_folder().unwrap();
       count_some_generic!(pp);
    };
    (workspace: true) => {
        // Logic when workspace toggle is true
        let pp = Config::find_src_folder().unwrap();
        let pp = pp.parent().unwrap();
        count_some_generic!(pp);
    };
    (workspace: false) => {
        // Logic when workspace toggle is false
        let pp = Config::find_src_folder().unwrap();
        count_some_generic!(pp);
    };
    ($other:tt) => { 
        compile_error!(concat!("Invalid input to count_some! macro: ", stringify!($other)));
    }
}





#[cfg(feature = "default")]
#[allow(unused_macros)]
#[macro_export]
macro_rules! count_folder {
     ($folder:ident) => {
        let mut counter = $crate::CountRoo::default();
        counter.config.project_src_path = $folder.to_string_lossy().to_string();
        counter.count_lines_of_code_for_all_types().expect("Failed to count lines of code");
        let writer_boxed = Box::<$crate::output_adapters::StdoutWriter>::default();
        counter.export(writer_boxed).expect("Failed to export to Stdout");
    }
}

/// Fires up a comprehensive line count across all file types in your project with `CountRoo`. 🌍🔢
///
/// `count_it_all` is your one-stop macro for when you want to dive deep and get a full count
/// of every line of code, across all file types recognized by your `CountRoo` configuration.
/// It's like commissioning `CountRoo` for a grand tour of your entire codebase, leaving no
/// stone unturned.
///
/// This macro simplifies the process by automatically setting up `CountRoo` with a configuration
/// based on a relative file path to "config.txt", initiating a thorough line count, and then
/// exporting the tally to stdout. Ideal for when you need a bird's-eye view of your project's
/// size and scope.
///
/// ## Requirements
/// - The `default` feature flag must be active for this macro to come into play.
/// - Ensure that `CountRoo`, `Config`, and an appropriate `OutputWriter` (like `StdoutWriter`)
///   are in scope and properly defined.
///
/// ## Example Invocation
/// To get a comprehensive line count of your project, simply call:
/// ```rust
/// // Inside build.rs
/// use countroo::prelude::*;
/// fn main() {
///     count_it_all!();
///     // If you're using it for a workspace, you can just use it like this inside any project's build.rs
///     countroo::count_it_all!(workspace: true);
/// }
/// ```
///
/// Whether you're evaluating the scale of a new project, or you need detailed metrics for
/// documentation or analysis, `count_it_all!` offers a hassle-free solution to gather those
/// insights with minimal setup. 🚀📊
#[cfg(feature = "default")]
#[allow(unused_macros)]
#[macro_export]
macro_rules! count_it_all {
    () => {
        // Default behavior remains the same
        let pp = $crate::Config::find_src_folder().unwrap();
        count_folder!(pp);
    };
    (workspace: true) => {
        // Logic when workspace toggle is true
        let pp = $crate::Config::find_src_folder().unwrap();
        let pp = pp.parent().unwrap();
        count_folder!(pp);
    };
    (workspace: false) => {
        // Logic when workspace toggle is false
        let pp = $crate::Config::find_src_folder().unwrap();
        count_folder!(pp);
    };
    ($other:tt) => { // Catch any other inputs that don't match the patterns above
        compile_error!(concat!("Invalid input to count_it_all! macro: ", stringify!($other)));
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
    fn test_count_it_all(){
        count_it_all!();
    }

    #[test]
    fn test_count_it_all_for_workspace(){
        count_it_all!(workspace: true);
    }
    

    #[test]
    fn test_count_it_all_for_non_workspace(){
        count_it_all!(workspace: false);
    }

    #[test]
    fn test_count_some_for_workspace(){
        count_some!(workspace: true);
    }
    
    #[test]
    fn test_count_some_for_non_workspace(){
        count_some!(workspace: false);
    }

    #[test]
    fn test_count_some(){
        count_some!();
    }

    #[test]
    fn test_count_lines_of_code_for_certain_types_manual_construction() {
        let config: Config = Config {
            project_src_path: "src".to_string(),
            config_path: None,
            extensions: vec!["rs".to_string()],
            count_empty_lines: false,
        };

        let mut counter = CountRoo::new(config);
        let result = counter.count_lines_of_code_for_certain_types().unwrap();
        println!("Total number of lines: {:?}", result);
        let writer_boxed = Box::<StdoutWriter>::default();
        counter.export(writer_boxed).expect("Failed to export");
        assert_ne!(result, 0);
    }

    #[test]
    fn test_count_lines_of_code_for_certain_types_builder_pattern() {
        let config = ConfigBuilder::default()
            .project_path("src")
            .extension("rs")
            .count_empty_lines(false)
            .build()
            .unwrap();

        let mut counter = CountRoo::new(config);
        let result = counter.count_lines_of_code_for_certain_types().unwrap();
        println!("Total number of lines: {:?}", result);
        let writer_boxed = Box::<StdoutWriter>::default();
        counter.export(writer_boxed).expect("Failed to export");
        assert_ne!(result, 0);
    }

    #[test]
    fn test_count_lines_of_code_for_all_types() {
        let config: Config = Config::from_rel_file_path("config.txt").unwrap();
        let mut counter = CountRoo::new(config);
        let result = counter.count_lines_of_code_for_all_types().unwrap_or(0);
        assert_ne!(result, 0);
        let writer_boxed = Box::<StdoutWriter>::default();
        counter.export(writer_boxed).expect("Failed to export");
    }

    #[test]
    fn test_different_types_count_is_different(){
        let config = Config::from_rel_file_path("config.txt").unwrap();
        let mut counter = CountRoo::new(config);

        let config2 = ConfigBuilder::default()
            .project_path("src")
            .extension("rs")
            .count_empty_lines(false)
            .build()
            .unwrap();
        let mut counter2 = CountRoo::new(config2);

        let config3 = Config::default();
        let mut counter3 = CountRoo::new(config3);

        let result1 = counter.count_lines_of_code_for_all_types().unwrap_or(0);
        let result2 = counter.count_lines_of_code_for_certain_types().unwrap_or(0);

        assert_ne!(result1, result2);

        let result1 = counter2.count_lines_of_code_for_all_types().unwrap_or(0);
        let result2 = counter2.count_lines_of_code_for_certain_types().unwrap_or(0);

        assert_ne!(result1, result2);

        let result1 = counter3.count_lines_of_code_for_all_types().unwrap_or(0);
        let result2 = counter3.count_lines_of_code_for_certain_types().unwrap_or(0);

        assert_ne!(result1, result2);
    }

    #[test]
    fn test_different_config_counts_are_equal_for_all_types(){
        let config = Config::from_rel_file_path("config.txt").unwrap();
        let mut counter = CountRoo::new(config);

        let config2 = ConfigBuilder::default()
            .project_path("src")
            .extension("rs")
            .count_empty_lines(false)
            .build()
            .unwrap();
        let mut counter2 = CountRoo::new(config2);

        let config3 = Config::default();
        let mut counter3 = CountRoo::new(config3);


        let result1 = counter.count_lines_of_code_for_all_types().unwrap_or(0);
        let result2 = counter2.count_lines_of_code_for_all_types().unwrap_or(0);
        let result3 = counter3.count_lines_of_code_for_all_types().unwrap_or(0);

        assert_eq!(result1, result2);
        assert_eq!(result1, result3);
        let writer_boxed = Box::<StdoutWriter>::default();
        counter.export(writer_boxed).expect("Failed to export");
    }

    #[test]
    fn test_different_config_counts_are_equal_for_certain_types(){
        let config = Config::from_rel_file_path("config.txt").unwrap();
        let mut counter = CountRoo::new(config);

        let config2 = ConfigBuilder::default()
            .project_path("src")
            .extension("rs")
            .count_empty_lines(false)
            .build()
            .unwrap();
        let mut counter2 = CountRoo::new(config2);

        let config3 = Config::default();
        let mut counter3 = CountRoo::new(config3);


        let result1 = counter.count_lines_of_code_for_certain_types().unwrap_or(0);
        let result2 = counter2.count_lines_of_code_for_certain_types().unwrap_or(0);
        let result3 = counter3.count_lines_of_code_for_certain_types().unwrap_or(0);

        assert_eq!(result1, result2);
        assert_eq!(result1, result3);
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
            project_src_path: src_path.to_string_lossy().to_string(),
            config_path: None,
            extensions: vec!["rs".to_string(),"py".to_string()],
            count_empty_lines: false,
        };

        assert_eq!(config.project_src_path, config2.project_src_path);
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
            project_src_path: PathBuf::from("E:\\RustroverProjects\\countroo\\src").to_string_lossy().to_string() ,
            config_path: None,
            extensions: vec!["rs".to_string(),"py".to_string()],
            count_empty_lines: false,
        };

        assert_eq!(config.project_src_path, config2.project_src_path);
        assert_eq!(config.extensions, config2.extensions);
        assert_eq!(config.count_empty_lines, config2.count_empty_lines);
    }
}



pub mod prelude{
    pub use crate::Config;
    pub use crate::ConfigBuilder;
    pub use crate::CountRoo;
    pub use crate::Analyzable;
    pub use crate::Exportable;
    pub use crate::LocCounterError;
    pub use crate::PathHelpers;
    pub use crate::CertainTypesCounter;
    pub use crate::AllTypesCounter;
    pub use crate::output_adapters::*;
    
    pub use crate::{count_it_all, count_some, count_folder, count_some_generic};
}