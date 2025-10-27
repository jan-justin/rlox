#[derive(Debug)]
pub struct LexError {
    line: usize,
    location: usize,
    error: &'static str,
}

impl From<(usize, usize, &'static str)> for LexError {
    fn from(tuple: (usize, usize, &'static str)) -> Self {
        let (line, location, error) = tuple;
        Self {
            line,
            location,
            error,
        }
    }
}

impl std::fmt::Display for LexError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let Self {
            line,
            location,
            error,
        } = self;

        write!(formatter, "[{line}:{location}] {error}")
    }
}

impl std::error::Error for LexError {}
