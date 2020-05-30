#[derive(Debug, PartialEq)]
pub struct ParserError {
    line: usize,
    column: usize,
    expected: String,
    actual: Option<String>
}

impl ParserError {
    pub fn new(line: usize, column: usize, expected: String, actual: Option<String>) -> ParserError {
        ParserError {
            line,
            column,
            expected,
            actual,
        }
    }

    pub fn to_err_msg(&self) -> String {
        match &self.actual {
            Some(actual) => 
                format!("expected '{}' but found '{}' at line {}, column {}", self.expected, actual, self.line, self.column),
            None => 
                format!("expected '{}' but found unknown error at line {}, column {}", self.expected, self.line, self.column),
        }
    }
}