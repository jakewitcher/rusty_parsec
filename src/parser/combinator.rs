use super::{Parser, ParserState, ParserSuccess, ParserResult};

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
                    let self_parser = self.get_parser();
                    let left = self_parser(parser_state)?;
        
                    other_parser(parser_state)
                        .map(|right| {
                            let position = right.get_position();

                            let success = 
                                ParserSuccess::new(
                                    (left.get_result(), right.get_result()), 
                                    position);

                            success
                        })
                        .map_err(|err| {
                            parser_state.move_input_state_back();
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
                    let self_parser = self.get_parser();

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
                    let self_parser = self.get_parser();
                    let prev = self_parser(parser_state)?;

                    other_parser(parser_state)
                        .map(|next|prev.map_position(|_|next.get_position()))
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
                    let self_parser = self.get_parser();

                    self_parser(parser_state)
                        .and(match other_parser(parser_state) {
                            Ok(next) => Ok(next),
                            Err(err) => {
                                parser_state.move_input_state_back();
                                Err(err)
                            }
                        })
                }
            );

        Combinator::new(next_parser)
    }

    pub fn then_return<UResult>(self, return_value: UResult) -> Combinator<UResult>
    where UResult: 'static
    {
        let next_parser =
            Box::new(
                move |parser_state: &mut ParserState| {
                    let self_parser = self.get_parser();

                    self_parser(parser_state)
                        .and(Ok(ParserSuccess::new(return_value, parser_state.get_position())))
                }
            );

        Combinator::new(next_parser)
    }

    pub fn between<UResult, VResult>(self, p_open: Parser<UResult>, p_close: Parser<VResult>) -> Combinator<TResult>
    where UResult: 'static, VResult: 'static
    {
        let next_parser =
            Box::new(
                move |parser_state: &mut ParserState| {
                    let self_parser = self.get_parser();

                    p_open(parser_state)?;
                    
                    match self_parser(parser_state) {
                        Ok(success) => {
                            match p_close(parser_state) {
                                Ok(close) => {
                                    Ok(success.map_position(|_|close.get_position()))
                                },
                                Err(err) => {
                                    parser_state.move_input_state_back();
                                    parser_state.move_input_state_back();
                                    Err(err)
                                },
                            }
                        },
                        Err(err) => {
                            parser_state.move_input_state_back();
                            Err(err)
                        },
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

                    Ok(result.map_result(f))
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
