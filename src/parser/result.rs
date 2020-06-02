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
pub struct ParserSuccess<TResult> {
    result: TResult,
    position: Position,
}

impl<TResult> ParserSuccess<TResult> {
    pub fn new(result: TResult, position: Position) -> ParserSuccess<TResult> {
        ParserSuccess { result, position }
    }

    pub fn map_result<UResult>(self, f: impl Fn(TResult) -> UResult) -> ParserSuccess<UResult> {
        let position = self.get_position();
        let new_result = f(self.get_result());

        ParserSuccess::new(new_result, position)
    }

    pub fn map_position(self, f: impl Fn(Position) -> Position) -> ParserSuccess<TResult> {
        let position = f(self.get_position());

        ParserSuccess::new(self.get_result(), position)
    }

    pub fn get_result(self) -> TResult {
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

pub type ParserResult<TResult> = Result<ParserSuccess<TResult>, ParserFailure>;
