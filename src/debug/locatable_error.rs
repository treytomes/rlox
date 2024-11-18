use std::error::Error;

use super::HasFileLocation;

pub trait LocatableError: Error + HasFileLocation + Send + Sync {
    fn as_error(&self) -> &(dyn Error + 'static);
    fn report(&self, source: &str);
}

impl<T> LocatableError for T
where
    T: Error + HasFileLocation + Send + Sync + 'static,
{
    fn as_error(&self) -> &(dyn Error + 'static) {
        self
    }

    fn report(&self, source: &str) {
        eprint!("\r\nerror: {}\r\n", self);

        // Take the 3rd line out the input text.
        let lines: Vec<&str> = source.split('\n').collect();
        let line = lines[self.get_line() - 1];

        // Convert line to a string and get the length of it.
        let len = self.get_line().to_string().len();

        eprint!("\r\n");
        eprint!("{} | {}\r\n", self.get_line(), line);
        eprint!(
            "{:>width$}-- Here.\r\n",
            "^",
            width = self.get_column() + len + 3
        );
    }
}
