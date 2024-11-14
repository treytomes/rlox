use super::HasFileLocation;

#[derive(Debug, Clone, Copy)]
pub struct FileLocation {
    line: usize,
    column: usize,
}

impl FileLocation {
    pub fn new(line: usize, column: usize) -> Self {
        Self { line, column }
    }

    pub fn from_loc(other: &dyn HasFileLocation) -> Self {
        Self::new(other.get_line(), other.get_column())
    }
}

impl HasFileLocation for FileLocation {
    fn get_line(&self) -> usize {
        self.line
    }

    fn get_column(&self) -> usize {
        self.column
    }
}
