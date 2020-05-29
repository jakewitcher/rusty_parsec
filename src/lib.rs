extern crate num_traits;

use num_traits::PrimInt;

pub struct ParserState {
    input: String,
    current_slice_start: usize,
    prev_slice_start: Vec<usize>,
    current_line_start: LineStart,
    prev_line_start: Vec<LineStart>,
}

impl ParserState {
    pub fn new(input: String) -> ParserState {

        ParserState {
            input,
            current_slice_start: 0,
            prev_slice_start: vec![0],
            current_line_start: LineStart::FirstLine,
            prev_line_start: vec![],
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
                self.prev_line_start.push(self.current_line_start.clone());
                
                self.current_line_start = LineStart::Index(self.current_slice_start + char_index);
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
        while ParserState::line_start_is_greater_than_slice_start(self.prev_line_start.last(), self.current_slice_start) {
            self.prev_line_start.pop();
        }

        self.current_line_start = 
            self.prev_line_start.pop()
                .unwrap_or(LineStart::FirstLine);
    }

    fn line_start_is_greater_than_slice_start(line_start: Option<&LineStart>, slice_start: usize) -> bool {
        match line_start {
            Some(LineStart::Index(i)) => *i > slice_start,
            _ => false
        }
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
        self.prev_line_start.len() + 1
    }

    fn get_column_number(&self) -> usize {
        match self.current_line_start {
            LineStart::FirstLine => self.current_slice_start + 1,
            LineStart::Index(index) => self.current_slice_start - index,
        }
    }
}

#[derive(Clone)]
pub enum LineStart {
    FirstLine,
    Index(usize),
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

type Parser<TResult> = Box<dyn FnOnce(&mut ParserState) -> ParserResult<TResult>>;

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
                move |parser_state: &mut ParserState| {
                    let self_parser = self.parser;
                    let left = self_parser(parser_state)?;
        
                    other_parser(parser_state)
                        .map(|right| (left, right))
                        .map_err(|err| {
                            parser_state.move_slice_start_back();
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
                move |parser_state: &mut ParserState| {
                    let self_parser = self.parser;

                    self_parser(parser_state).or_else(|_|other_parser(parser_state))
                }
            );

        Combinator::new(next_parser)
    }

    pub fn take_prev<UResult>(self, other_parser: Parser<UResult>) -> Combinator<TResult>
    where UResult: 'static
    {
        let next_parser =
            Box::new(
                move |parser_state: &mut ParserState| {
                    let self_parser = self.parser;
                    let prev = self_parser(parser_state)?;

                    other_parser(parser_state)
                        .map(|_|prev)
                        .map_err(|err| {
                            parser_state.move_input_state_back();
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
                move |parser_state: &mut ParserState| {
                    let self_parser = self.parser;

                    match self_parser(parser_state) {
                        Ok(_) => 
                            other_parser(parser_state),
                        Err(err) => {
                            parser_state.move_input_state_back();
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
                move |parser_state: &mut ParserState| {
                    let self_parser = self.parser;
                    let result = self_parser(parser_state)?;

                    Ok(f(result))
                }
            );

        Combinator::new(next_parser)
    }

    pub fn run(self, input: String) -> ParserResult<TResult> {
        let parser = self.parser;
        let mut parser_state = ParserState::new(input);

        parser(&mut parser_state)
    }


}

pub fn ws() -> Parser<()> {
    Box::new(
        move |parser_state: &mut ParserState| {
            let chars: Vec<char> = parser_state.current_slice().chars().collect();
            
            let mut ws_char_count = 0;

            for c in chars  {
                if c.is_ascii_whitespace() {
                    ws_char_count += 1;
                } else {
                    break;
                }
            }

            parser_state.move_input_state_forward(ws_char_count);
            
            Ok(())
        }
    )
}

pub fn p_char(target_char: char) -> Parser<char> {
    Box::new(
    move |parser_state: &mut ParserState| 
        match parser_state.len() {
            0 => {
                let err = ParserError::new(
                    parser_state.get_line_number(),
                    parser_state.get_column_number(),
                    target_char.to_string(),
                    None
                );

                Err(err)
            },
            
            _ => {
                let chars: Vec<char> = parser_state.current_slice().chars().collect();
                let source_char = chars[0];

                if source_char == target_char {
                    parser_state.move_input_state_forward(1);
                    Ok(source_char)
                } else {
                    let err = ParserError::new(
                        parser_state.get_line_number(),
                        parser_state.get_column_number(),
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
        move |parser_state: &mut ParserState| {
            let source_string = parser_state.get_slice(target_string.len());

            match source_string {
                Some(source) => {
                    if target_string == source {
                        parser_state.move_input_state_forward(target_string.len());
                        Ok(String::from(source))
                    } else {
                        let err = ParserError::new(
                            parser_state.get_line_number(),
                            parser_state.get_column_number(),
                            target_string,
                            Some(source)
                        );
        
                        Err(err)
                    }
                },
                None => {
                    let err = ParserError::new(
                        parser_state.get_line_number(),
                        parser_state.get_column_number(),
                        target_string,
                        None
                    );
    
                    Err(err)
                }
            }
        }
    )
}

pub fn p_i32() -> Parser<i32> {
    p_int(Box::new(|slice: String| slice.parse::<i32>()))
}

pub fn p_i64() -> Parser<i64> {
    p_int(Box::new(|slice: String| slice.parse::<i64>()))
}

pub fn p_u32() -> Parser<u32> {
    p_int(Box::new(|slice: String| slice.parse::<u32>()))
}

pub fn p_u64() -> Parser<u64> {
    p_int(Box::new(|slice: String| slice.parse::<u64>()))
}

pub fn p_isize() -> Parser<isize> {
    p_int(Box::new(|slice: String| slice.parse::<isize>()))
}

pub fn p_iusize() -> Parser<usize> {
    p_int(Box::new(|slice: String| slice.parse::<usize>()))
}

fn p_int<T>(parse_num: Box<dyn Fn(String) -> Result<T, std::num::ParseIntError>>) -> Parser<T> 
where T: PrimInt + 'static
{
    Box::new(
        move |parser_state: &mut ParserState| {
            let chars: Vec<char> = parser_state.current_slice().chars().collect();
            
            let mut int_char_count = 0;

            let err = ParserError::new(
                parser_state.get_line_number(),
                parser_state.get_column_number(),
                "integral value".to_string(),
                None
            );

            for c in chars  {
                if c.is_numeric() || c == '-' && int_char_count == 0 {
                    int_char_count += 1;
                } else {
                    break;
                }
            }

            if int_char_count == 0 {
                Err(err)
            } else {
                let int_slice = parser_state.get_slice(int_char_count);

                match int_slice {
                    Some(slice) => {
                        let integer_result = parse_num(slice);
                        match integer_result {
                            Ok(integer) => {
                                parser_state.move_input_state_forward(int_char_count);
                                Ok(integer)
                            },
                            _ => Err(err)
                        }
                    },
                    _ => Err(err)
                }
            }
        }
    )
}