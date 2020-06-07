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

#[derive(Debug, PartialEq)]
pub struct ParserFailure {
    expected: String,
    actual: Option<String>,
    position: Position,
}

impl ParserFailure {
    pub fn new(expected: String, actual: Option<String>, position: Position) -> ParserFailure {
        ParserFailure { position, expected, actual, }
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

pub type ParserResult<T> = Result<ParserSuccess<T>, ParserFailure>;
