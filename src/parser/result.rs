pub use success::ParserSuccess;
pub use failure::{ParserFailure, FailureSeverity};

/// ```ParserResult``` is a type alias for the Result type returned by the parsers in the ```rusty-parsec``` library.
pub type ParserResult<T> = Result<ParserSuccess<T>, ParserFailure>;

/// ```Position``` describes the current position of the parser state -- the line, column number, and the current index of the input string.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Position {
    line: usize,
    column: usize,
    index: usize,
}

impl Position {
    /// ```new``` creates a new instance of the ```Position``` struct.
    pub fn new(line: usize, column: usize, index: usize) -> Position {
        Position { line, column, index }
    }
}

pub mod success {
    use super::Position;

    /// ```ParserSuccess``` is the type returned by a parser when it succeeds in parsing the input string. 
    /// When a parser succeeds both the value parsed and the current position of the parser state is returned.
    #[derive(Debug, PartialEq)]
    pub struct ParserSuccess<T> {
        result: T,
        position: Position,
    }
    
    impl<T> ParserSuccess<T> {
        /// ```new``` creates a new instance of the ```ParserSuccess``` struct.
        pub fn new(result: T, position: Position) -> ParserSuccess<T> {
            ParserSuccess { result, position }
        }
    
        /// ```map_result``` maps the result value returned by the successful parsing of the input string to a different type based on the mapping function parameter.
        pub(in crate::parser) fn map_result<U>(self, f: impl Fn(T) -> U) -> ParserSuccess<U> {
            let position = self.get_position();
            let new_result = f(self.get_result());
    
            ParserSuccess::new(new_result, position)
        }
    
        /// ```with_result``` returns a new ```ParserSuccess``` struct, replacing the parser result value with the ```new_result``` parameter.
        pub(in crate::parser) fn with_result<U>(self, new_result: U) -> ParserSuccess<U> {
            ParserSuccess::new(new_result, self.get_position())
        }
    
        /// ```with_position``` returns a new ```ParserSuccess``` struct, replacing the position with the ```new_position``` parameter.
        pub(in crate::parser) fn with_position(self, new_position: Position) -> ParserSuccess<T> {
            ParserSuccess::new(self.get_result(), new_position)
        }
    
        /// ```get_result``` returns the result value of a ```ParserSuccess``` struct.
        ///
        /// # Examples
        /// 
        /// ```
        /// use rusty_parsec::*;
        /// 
        /// let success = ParserSuccess::new(15, Position::new(1, 4, 3));
        ///
        /// assert_eq!(15, success.get_result());
        /// ```
        pub fn get_result(self) -> T {
            self.result
        }
    

        /// ```get_position``` returns the parser state position of a ```ParserSuccess``` struct.
        ///
        /// # Examples
        /// 
        /// ```
        /// use rusty_parsec::*;
        /// 
        /// let position = Position::new(1, 4, 3);
        /// let success = ParserSuccess::new(15, position);
        ///
        /// assert_eq!(position, success.get_position());
        /// ```
        pub fn get_position(&self) -> Position {
            self.position
        }
    }
}

pub mod failure {
    use super::Position;

    /// ```FailureSeverity is an enum used by the ParserFailure struct to distingish between two different types of parser failure.
    /// The ```Error``` arm is used when a parser fails, but the ParserState struct has not been changed. If one parser returns an ```Error```
    /// type, it is possible that another parser consuming that result will be able to recover and continue parsing.
    /// 
    /// ```Fatal``` failures are ones that have changed the parser state, and therefore attempting to continue parsing will only produce
    /// bad results because the position of the ```ParserState``` struct has been changed.
    /// 
    /// For example, the ```or``` method on the ```Parser``` struct returns a success if one of two parsers succeeds. 
    /// If the first parser fails and the resulting failure is not fatal (meaning the state of the parser has not been changed and the result
    /// returned by the parser is a ```ParserFailure``` with the ```Error``` severity) then the second parser can be attempted.
    /// 
    /// However if the ```ParserState``` struct was changed by the first parser and a ```Fatal``` failure is returned, then the second 
    /// parser should not be attempted, because that would mean it is being applied at the incorrect index of the input string.
    #[derive(Debug, PartialEq)]
    pub enum FailureSeverity {
        Error,
        FatalError
    }
    
        /// ```ParserFailure``` is the type returned by a parser when it fails in parsing the input string. 
    /// When a parser fails, the expected string value is returned along with the severity of the failure (see ```FailureSeverity```),
    /// and the position of the ParserState at the time of the failure. 
    /// 
    /// Optionally the ```ParserFailure``` struct will include the 
    /// string content that was parsed to aid in debugging, however not all parsers are able to provide this information.
    #[derive(Debug, PartialEq)]
    pub struct ParserFailure {
        expected: String,
        actual: Option<String>,
        severity: FailureSeverity,
        position: Position,
    }
    
    impl ParserFailure {
        /// ```new_err``` creates a new instance of the ```ParserFailure``` struct with a failure severity of ```Error```.
        pub fn new_err(expected: String, actual: Option<String>, position: Position) -> ParserFailure {
            ParserFailure { position, severity: FailureSeverity::Error, expected, actual, }
        }

        /// ```new_fatal_err``` creates a new instance of the ```ParserFailure``` struct with a failure severity of ```Fatal```.
        pub fn new_fatal_err(expected: String, actual: Option<String>, position: Position) -> ParserFailure {
            ParserFailure { position, severity: FailureSeverity::FatalError, expected, actual, }
        }
    
        /// ```to_err``` changes the ```FailureSeverity``` of a ```ParserFailure``` to the ```Error``` type. This is only used when
        /// a parser capable of rolling back the parser state encounters a fatal error but can recover the initial parser state before the failure.
        /// The ```ParserFailure``` returned by a parser with this capabality can safely return an ```Error``` type after reverting the parser state.
        pub(in crate::parser) fn to_err(self) -> ParserFailure {
            ParserFailure::new_err(self.expected, self.actual, self.position)
        }
    
        /// ```to_fatal_err``` changes the ```FailureSeverity``` of a ```ParserFailure``` to the ```Fatal``` type.
        pub(in crate::parser) fn to_fatal_err(self) -> ParserFailure {
            ParserFailure::new_fatal_err(self.expected, self.actual, self.position)
        }
    
        /// ```is_fatal``` returns ```true``` if the ```FailureSeverity``` of a ```ParserFailure``` is ```Fatal```, otherwise it returns ```false```.
        ///
        /// # Examples
        /// 
        /// ```
        /// use rusty_parsec::*;
        /// 
        /// let fatal_failure = ParserFailure::new_fatal_err("hello".to_string(), None, Position::new(1, 4, 3));
        /// assert!(fatal_failure.is_fatal());
        /// 
        /// let failure = ParserFailure::new_err("hello".to_string(), None, Position::new(1, 4, 3));
        /// assert!(!failure.is_fatal());
        /// ```
        pub fn is_fatal(&self) -> bool {
            self.severity == FailureSeverity::FatalError
        }
    
        /// ```to_err_msg``` takes a ```ParserFailure``` struct and returns the information it contains in a user friendly way.
        /// This method is primarily used for error messaging to help with debugging when a parser fails.
        pub(in crate::parser) fn to_err_msg(&self) -> String {
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