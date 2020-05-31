use super::ParserResult;
use super::Parser;
use super::state::ParserState;

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

                    self_parser(parser_state).and(other_parser(parser_state))
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
                    let self_parser = self.parser;

                    self_parser(parser_state).and(Ok(return_value))
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
