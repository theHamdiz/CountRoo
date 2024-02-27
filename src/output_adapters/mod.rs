#[cfg(feature = "colored-output")]
use colored::ColoredString;
#[cfg(feature = "colored-output")]
use crate::colorizer::LanguageColorMapping;
#[cfg(any(feature = "colored-output", feature = "tabular-output"))]
use crate::tabular::Table;

#[cfg(feature = "default")]
pub trait OutputWriter {
    fn write(&self, data: &str) -> Result<(), std::io::Error>;
}
#[cfg(feature = "default")]
pub struct StdoutWriter {}

#[cfg(feature = "default")]
impl StdoutWriter {
    pub fn new() -> Self {
        StdoutWriter {}
    }
}

#[cfg(feature = "default")]
impl Default for StdoutWriter {
    fn default() -> Self {
        StdoutWriter::new()
    }
}

#[cfg(feature = "default")]
impl OutputWriter for StdoutWriter {
    fn write(&self, data: &str) -> Result<(), std::io::Error> {
        println!("{}", data);
        Ok(())
    }
}

#[cfg(feature = "default")]
pub struct FileWriter {
    file_path: String,
}
#[cfg(feature = "default")]
impl FileWriter {
    fn new(file_path: &str) -> Self {
        FileWriter { file_path: file_path.to_string() }
    }
}
#[cfg(feature = "default")]
impl OutputWriter for FileWriter {
    fn write(&self, data: &str) -> Result<(), std::io::Error> {
        std::fs::write(&self.file_path, data)
    }
}


#[cfg(feature = "json-output")]
pub struct JsonWriter{
    file_path: String,
}

#[cfg(feature = "json-output")]
impl JsonWriter {
    pub fn new(file_path: &str) -> Self {
        JsonWriter { file_path: file_path.to_string() }
    }
}

#[cfg(feature = "json-output")]
impl OutputWriter for JsonWriter {
    fn write(&self, data: &str) -> Result<(), std::io::Error> {
        let json: serde_json::Value = serde_json::from_str(data).unwrap();
        let pretty_json = serde_json::to_string_pretty(&json).unwrap();
        std::fs::write(&self.file_path, pretty_json)
    }
}

#[cfg(feature = "xml-output")]

pub struct XmlWriter{
    file_path: String,
}

#[cfg(feature = "xml-output")]
impl XmlWriter {
    pub fn new(file_path: &str) -> Self {
        XmlWriter { file_path: file_path.to_string() }
    }
}

#[cfg(feature = "xml-output")]
impl OutputWriter for XmlWriter {
    fn write(&self, data: &str) -> Result<(), std::io::Error> {
        // let mut writer = std::fs::File::create(&self.file_path)?;
        // let mut writer = quick_xml::Writer::new(writer);
        // let reader = quick_xml::Reader::from_str(data);
        // let mut buf = Vec::new();
        // for result in reader.into_iter() {
        //     match result {
        //         Ok(quick_xml::events::Event::Start(ref e)) => {
        //             writer.write_event(quick_xml::events::Event::Start(e.clone()))?;
        //         }
        //         Ok(quick_xml::events::Event::End(ref e)) => {
        //             writer.write_event(quick_xml::events::Event::End(e.clone()))?;
        //         }
        //         Ok(quick_xml::events::Event::Text(e)) => {
        //             writer.write_event(quick_xml::events::Event::Text(e))?;
        //         }
        //         Ok(quick_xml::events::Event::Eof) => {
        //             writer.write_event(quick_xml::events::Event::Eof)?;
        //         }
        //         Err(e) => {
        //             return Err(std::io::Error::new(std::io::ErrorKind::Other, e));
        //         }
        //         _ => {}
        //     }
        // }
        Ok(())
    }
}

#[cfg(feature = "toml-output")]
pub struct TomlWriter{
    file_path: String,
}

#[cfg(feature = "toml-output")]
impl TomlWriter {
    pub fn new(file_path: &str) -> Self {
        TomlWriter { file_path: file_path.to_string() }
    }
}

#[cfg(feature = "toml-output")]
impl OutputWriter for TomlWriter {
    fn write(&self, data: &str) -> Result<(), std::io::Error> {
        std::fs::write(&self.file_path, data)
    }
}


#[cfg(feature = "yaml-output")]
pub struct YamlWriter{
    file_path: String,
}
#[cfg(feature = "yaml-output")]
impl YamlWriter {
    pub fn new(file_path: &str) -> Self {
        YamlWriter { file_path: file_path.to_string() }
    }
}
#[cfg(feature = "yaml-output")]
impl OutputWriter for YamlWriter {
    fn write(&self, data: &str) -> Result<(), std::io::Error> {
        let yaml: serde_yaml::Value = serde_yaml::from_str(data).unwrap();
        let pretty_yaml = serde_yaml::to_string(&yaml).unwrap();
        std::fs::write(&self.file_path, pretty_yaml)
    }
}

#[cfg(feature = "default")]
pub trait FormattedOutput {
    fn formatted_output(&self) -> String;
}

#[cfg(feature = "tabular-output")]
pub trait TabularOutput {
    /// prints the object as a formatted table.
    /// 
    /// # Arguments 
    /// 
    /// * `output_writer`: StadoutWriter - the writer to use for output in the console.
    /// 
    /// returns: () 
    /// 
    /// # Examples 
    /// 
    /// ```
    /// 
    /// tabular.print_table(StdoutWriter::new());
    /// ```
    fn print_table(&self, output_writer: StdoutWriter);
}

#[cfg(feature = "colored-output")]
pub(crate) trait ColorizedOutput {
    fn apply_colors_to_table(&self, table: &Table, color_map: &[LanguageColorMapping]) -> ColoredString;
}