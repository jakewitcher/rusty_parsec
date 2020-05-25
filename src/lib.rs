pub struct ParserInput {
    input: String,
    prev_slice_start: Vec<usize>,
    slice_start: usize,
}

impl ParserInput {
    pub fn new(input: String) -> ParserInput {

        ParserInput {
            input,
            prev_slice_start: vec![0],
            slice_start: 0,
        }
    }

    pub fn len(&self) -> usize {
        self.input.len()
    }

    pub fn current_slice(&self) -> &str {
        if self.slice_start >= self.len() {
            panic!("starting index of current slice exceeds length of parser input")
        }

        &self.input[self.slice_start..]
    }

    fn move_slice_start_forward(&mut self, increment: usize) {
        self.prev_slice_start.push(self.slice_start);
        self.slice_start += increment;
    }

    fn move_slice_start_back(&mut self) {
        self.slice_start = self.prev_slice_start.pop().unwrap();
    }

    fn get_slice(&self, length: usize) -> Option<String> {
        let slice_end = self.slice_start + length;

        if slice_end > self.len() {
            None
        } else {
            let slice = &self.input[self.slice_start..slice_end];
            Some(String::from(slice))
        }
    }
}

type Parser<TResult> = Box<dyn FnOnce(&mut ParserInput) -> Result<TResult, String>>;

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
                            parser_input.move_slice_start_back();
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

                    self_parser(parser_input)
                        .and_then(|_|other_parser(parser_input))
                        .map_err(|err| {
                            parser_input.move_slice_start_back();
                            err
                        })
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

    pub fn run(self, input: String) -> Result<TResult, String> {
        let parser = self.parser;
        let mut parser_input = ParserInput::new(input);

        parser(&mut parser_input)
    }


}

pub fn p_char(target_char: char) -> Parser<char> {
    Box::new(
    move |parser_input: &mut ParserInput| 
    match parser_input.len() {
        0 => Err(String::from("no char could be found")),
        
        _ => {
            let chars: Vec<char> = parser_input.current_slice().chars().collect();
            let source_char = chars[0];

            if source_char == target_char {
                parser_input.move_slice_start_forward(1);
                Ok(source_char)
            } else {
                let err_msg = format!("expected '{}' but found '{}'", target_char, source_char);
                Err(err_msg)
            }
        }
    })
}

pub fn p_string(target_string: String) -> Parser<String> {
    Box::new(
        move |parser_input: &mut ParserInput| {
            let source_string = parser_input.get_slice(target_string.len());

            match source_string {
                Some(source) => {
                    if target_string == source {
                        parser_input.move_slice_start_forward(target_string.len());
                        Ok(String::from(source))
                    } else {
                        let err_msg = format!("expected '{}' but found '{}'", target_string, source);
                        Err(err_msg)
                    }
                },
                None => {
                    let err_msg = format!("expected '{}' but input string was not long enough", target_string);
                    Err(err_msg)
                }
            }
        }
    )
}
