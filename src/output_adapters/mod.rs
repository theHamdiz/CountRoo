#[cfg(feature = "default")]
pub trait OutputWriter {
    fn write(&self, data: &str) -> Result<(), std::io::Error>;
}
#[cfg(feature = "default")]
pub struct StdoutWriter {}
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
