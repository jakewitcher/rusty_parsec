pub struct ParserInput {
    input: String,
    current_slice_start: usize,
    prev_slice_start: Vec<usize>,
    current_newline: isize,
    prev_newline: Vec<isize>,
}

impl ParserInput {
    pub fn new(input: String) -> ParserInput {

        ParserInput {
            input,
            current_slice_start: 0,
            prev_slice_start: vec![0],
            current_newline: -1,
            prev_newline: vec![],
        }
    }

    pub fn len(&self) -> usize {
        self.input.len()
    }

    pub fn current_slice(&self) -> &str {
        if self.current_slice_start >= self.len() {
            panic!("starting index of current slice exceeds length of parser input")
        }

        &self.input[self.current_slice_start..]
    }

    fn move_input_state_forward(&mut self, increment: usize) {
        self.move_newlines_forward(increment);
        self.move_slice_start_forward(increment);
    }

    fn move_slice_start_forward(&mut self, increment: usize) {
        

        self.prev_slice_start.push(self.current_slice_start);
        self.current_slice_start += increment;
    }

    fn move_newlines_forward(&mut self, increment: usize) {
        let current = self.current_slice_start;
        let new = current + increment;

        let current_slice = &self.input[current..new];
        let chars: Vec<char> = current_slice.chars().collect();
        let mut char_index = 0;

        for c in chars {
            if c == '\n' {
                self.prev_newline.push(self.current_newline);
                self.current_newline = (self.current_slice_start + char_index) as isize;
            }
            
            char_index += 1;
        }
    }

    fn move_input_state_back(&mut self) {
        self.move_slice_start_back();
        self.move_newlines_back();
    }

    fn move_slice_start_back(&mut self) {
        self.current_slice_start = self.prev_slice_start.pop().unwrap();
    }

    fn move_newlines_back(&mut self) {
        while *self.prev_newline.last().unwrap_or(&0) > self.current_slice_start as isize {
            self.prev_newline.pop();
        }

        self.current_newline = self.prev_newline.pop().unwrap_or(-1);
    }

    fn get_slice(&self, length: usize) -> Option<String> {
        let slice_end = self.current_slice_start + length;

        if slice_end > self.len() {
            None
        } else {
            let slice = &self.input[self.current_slice_start..slice_end];
            Some(String::from(slice))
        }
    }

    fn get_line_number(&self) -> usize {
        self.prev_newline.len() + 1
    }

    fn get_column_number(&self) -> usize {
        (self.current_slice_start as isize - self.current_newline) as usize
    }
}

#[derive(Debug, PartialEq)]
pub struct ParserError {
    line: usize,
    column: usize,
    expected: String,
    actual: Option<String>
}

impl ParserError {
    fn new(line: usize, column: usize, expected: String, actual: Option<String>) -> ParserError {
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

type ParserResult<TResult> = Result<TResult, ParserError>;

type Parser<TResult> = Box<dyn FnOnce(&mut ParserInput) -> ParserResult<TResult>>;

pub struct Combinator<TResult>
where TResult: 'static
{
    parser: Parser<TResult>
}

impl<TResult> Combinator<TResult> {
    pub fn new(parser: Parser<TResult>) -> Combinator<TResult> {
        Combinator { parser }
    }

    pub fn get_parser(self) -> Parser<TResult> {
        self.parser
    }

    pub fn and<UResult>(self, other_parser: Parser<UResult>) -> Combinator<(TResult, UResult)>
    where UResult: 'static
    {
        let next_parser =
            Box::new(
                move |parser_input: &mut ParserInput| {
                    let self_parser = self.parser;
                    let left = self_parser(parser_input)?;
        
                    other_parser(parser_input)
                        .map(|right| (left, right))
                        .map_err(|err| {
                            parser_input.move_slice_start_back();
                            err
                        })
                }
            );

        Combinator::new(next_parser)
    }

    pub fn or(self, other_parser: Parser<TResult>) -> Combinator<TResult>
    {
        let next_parser =
            Box::new(
                move |parser_input: &mut ParserInput| {
                    let self_parser = self.parser;

                    self_parser(parser_input).or_else(|_|other_parser(parser_input))
                }
            );

        Combinator::new(next_parser)
    }

    pub fn take_prev<UResult>(self, other_parser: Parser<UResult>) -> Combinator<TResult>
    where UResult: 'static
    {
        let next_parser =
            Box::new(
                move |parser_input: &mut ParserInput| {
                    let self_parser = self.parser;
                    let prev = self_parser(parser_input)?;

                    other_parser(parser_input)
                        .map(|_|prev)
                        .map_err(|err| {
                            parser_input.move_input_state_back();
                            err
                        })
                }
            );

        Combinator::new(next_parser)
    }

    pub fn take_next<UResult>(self, other_parser: Parser<UResult>) -> Combinator<UResult>
    where UResult: 'static
    {
        let next_parser =
            Box::new(
                move |parser_input: &mut ParserInput| {
                    let self_parser = self.parser;

                    match self_parser(parser_input) {
                        Ok(_) => 
                            other_parser(parser_input),
                        Err(err) => {
                            parser_input.move_input_state_back();
                            Err(err)
                        }
                    }
                }
            );

        Combinator::new(next_parser)
    }

    pub fn map<UResult>(self, f: Box<dyn Fn(TResult) -> UResult>) -> Combinator<UResult>
    where UResult: 'static
    {
        let next_parser =
            Box::new(
                move |parser_input: &mut ParserInput| {
                    let self_parser = self.parser;
                    let result = self_parser(parser_input)?;

                    Ok(f(result))
                }
            );

        Combinator::new(next_parser)
    }

    pub fn run(self, input: String) -> ParserResult<TResult> {
        let parser = self.parser;
        let mut parser_input = ParserInput::new(input);

        parser(&mut parser_input)
    }


}

pub fn ws() -> Parser<()> {
    Box::new(
        move |parser_input: &mut ParserInput| {
            let chars: Vec<char> = parser_input.current_slice().chars().collect();
            
            let mut ws_char_count = 0;

            for c in chars  {
                if c.is_ascii_whitespace() {
                    ws_char_count += 1;
                } else {
                    break;
                }
            }

            parser_input.move_input_state_forward(ws_char_count);
            
            Ok(())
        }
    )
}

pub fn p_char(target_char: char) -> Parser<char> {
    Box::new(
    move |parser_input: &mut ParserInput| 
        match parser_input.len() {
            0 => {
                let err = ParserError::new(
                    parser_input.get_line_number(),
                    parser_input.get_column_number(),
                    target_char.to_string(),
                    None
                );

                Err(err)
            },
            
            _ => {
                let chars: Vec<char> = parser_input.current_slice().chars().collect();
                let source_char = chars[0];

                if source_char == target_char {
                    parser_input.move_input_state_forward(1);
                    Ok(source_char)
                } else {
                    let err = ParserError::new(
                        parser_input.get_line_number(),
                        parser_input.get_column_number(),
                        target_char.to_string(),
                        Some(source_char.to_string())
                    );
    
                    Err(err)
                }
            }
        }
    )
}

pub fn p_string(target_string: String) -> Parser<String> {
    Box::new(
        move |parser_input: &mut ParserInput| {
            let source_string = parser_input.get_slice(target_string.len());

            match source_string {
                Some(source) => {
                    if target_string == source {
                        parser_input.move_input_state_forward(target_string.len());
                        Ok(String::from(source))
                    } else {
                        let err = ParserError::new(
                            parser_input.get_line_number(),
                            parser_input.get_column_number(),
                            target_string,
                            Some(source)
                        );
        
                        Err(err)
                    }
                },
                None => {
                    let err = ParserError::new(
                        parser_input.get_line_number(),
                        parser_input.get_column_number(),
                        target_string,
                        None
                    );
    
                    Err(err)
                }
            }
        }
    )
}
