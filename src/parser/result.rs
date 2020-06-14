pub use success::ParserSuccess;
pub use failure::{ParserFailure, FailureSeverity};

pub type ParserResult<T> = Result<ParserSuccess<T>, ParserFailure>;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Position {
    line: usize,
    column: usize,
    index: usize,
}

impl Position {
    pub fn new(line: usize, column: usize, index: usize) -> Position {
        Position { line, column, index }
    }
}

pub mod success {
    use super::Position;

    #[derive(Debug, PartialEq)]
    pub struct ParserSuccess<T> {
        result: T,
        position: Position,
    }
    
    impl<T> ParserSuccess<T> {
        pub fn new(result: T, position: Position) -> ParserSuccess<T> {
            ParserSuccess { result, position }
        }
    
        pub fn map_result<U>(self, f: impl Fn(T) -> U) -> ParserSuccess<U> {
            let position = self.get_position();
            let new_result = f(self.get_result());
    
            ParserSuccess::new(new_result, position)
        }
    
        pub fn with_result<U>(self, new_result: U) -> ParserSuccess<U> {
            let position = self.get_position();
    
            ParserSuccess::new(new_result, position)
        }
    
        pub fn with_position(self, position: Position) -> ParserSuccess<T> {
            ParserSuccess::new(self.get_result(), position)
        }
    
        pub fn get_result(self) -> T {
            self.result
        }
    
        pub fn get_position(&self) -> Position {
            self.position
        }
    }
}

pub mod failure {
    use super::Position;

    #[derive(Debug, PartialEq)]
    pub enum FailureSeverity {
        Error,
        FatalError
    }
    
    #[derive(Debug, PartialEq)]
    pub struct ParserFailure {
        expected: String,
        actual: Option<String>,
        severity: FailureSeverity,
        position: Position,
    }
    
    impl ParserFailure {
        pub fn new_err(expected: String, actual: Option<String>, position: Position) -> ParserFailure {
            ParserFailure { position, severity: FailureSeverity::Error, expected, actual, }
        }

        pub fn new_fatal_err(expected: String, actual: Option<String>, position: Position) -> ParserFailure {
            ParserFailure { position, severity: FailureSeverity::FatalError, expected, actual, }
        }
    
        pub fn to_err(self) -> ParserFailure {
            ParserFailure::new_err(self.expected, self.actual, self.position)
        }
    
        pub fn to_fatal_err(self) -> ParserFailure {
            ParserFailure::new_fatal_err(self.expected, self.actual, self.position)
        }
    
        pub fn is_fatal(&self) -> bool {
            self.severity == FailureSeverity::FatalError
        }
    
        pub fn to_err_msg(&self) -> String {
            match &self.actual {
                Some(actual) => 
                    format!(
                        "expected '{}' but found '{}' at line {}, column {}", 
                        self.expected, 
                        actual, 
                        self.position.line, 
                        self.position.column
                    ),
                None => 
                    format!(
                        "expected '{}' but found unknown error at line {}, column {}", 
                        self.expected, 
                        self.position.line, 
                        self.position.column
                    ),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn writes_err_data_as_string_msg_with_some_expected() {
        let expected = "expected 'a' but found 'b' at line 1, column 1".to_string();

        let actual = ParserFailure::new_err("a".to_string(), Some("b".to_string()), Position::new(1, 1, 0)).to_err_msg();

        assert_eq!(expected, actual);
    }

    #[test]
    fn writes_err_data_as_string_msg_with_none_expected() {
        let expected = "expected 'a' but found unknown error at line 1, column 1".to_string();

        let actual = ParserFailure::new_err("a".to_string(), None, Position::new(1, 1, 0)).to_err_msg();

        assert_eq!(expected, actual);
    }
}