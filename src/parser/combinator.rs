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
                    let right = other_parser(parser_state)?;

                    let position = right.get_position();
                    let result = (left.get_result(), right.get_result());

                    Ok(ParserSuccess::new(result, position))
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

                    self_parser(parser_state).or(other_parser(parser_state))
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
                    let next = other_parser(parser_state)?;
                    
                    Ok(prev.update_position(next.get_position()))
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
                        .and(other_parser(parser_state))
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

                    let result = self_parser(parser_state)?;
                    
                    Ok(ParserSuccess::new(return_value, result.get_position()))
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

                    let result = self_parser(parser_state)?;
                    let close = p_close(parser_state)?;

                    Ok(result.update_position(close.get_position()))
                }
            );

        Combinator::new(next_parser)
    }

    pub fn pipe_2<UResult, VResult>(self, snd_parser: Parser<UResult>, f: Box<dyn Fn (TResult, UResult) -> VResult>) -> Combinator<VResult> 
    where UResult: 'static
    {
        let next_parser =
            Box::new(
                move |parser_state: &mut ParserState| {
                    let self_parser = self.parser;

                    let first = self_parser(parser_state)?;
                    let second = snd_parser(parser_state)?;

                    let position = second.get_position();
                    let result = 
                        f(
                            first.get_result(), 
                            second.get_result()
                        );

                    Ok(ParserSuccess::new(result, position))
                }
            );

        Combinator::new(next_parser)
    }

    pub fn pipe_3<UResult, VResult, WResult>(self, snd_parser: Parser<UResult>, third_parser: Parser<VResult>, f: Box<dyn Fn (TResult, UResult, VResult) -> WResult>) -> Combinator<WResult> 
    where UResult: 'static, VResult: 'static
    {
        let next_parser =
            Box::new(
                move |parser_state: &mut ParserState| {
                    let self_parser = self.parser;

                    let first = self_parser(parser_state)?;
                    let second = snd_parser(parser_state)?;
                    let third = third_parser(parser_state)?;

                    let position = third.get_position();
                    let result = 
                        f(
                            first.get_result(), 
                            second.get_result(), 
                            third.get_result()
                        );

                    Ok(ParserSuccess::new(result, position))
                }
            );

        Combinator::new(next_parser)
    }

    pub fn pipe_4<UResult, VResult, WResult, XResult>(self, snd_parser: Parser<UResult>, third_parser: Parser<VResult>, fourth_parser: Parser<WResult>, f: Box<dyn Fn (TResult, UResult, VResult, WResult) -> XResult>) -> Combinator<XResult> 
    where UResult: 'static, VResult: 'static, WResult: 'static
    {
        let next_parser =
            Box::new(
                move |parser_state: &mut ParserState| {
                    let self_parser = self.parser;

                    let first = self_parser(parser_state)?;
                    let second = snd_parser(parser_state)?;
                    let third = third_parser(parser_state)?;
                    let fourth = fourth_parser(parser_state)?;

                    let position = fourth.get_position();
                    let result = 
                        f(
                            first.get_result(), 
                            second.get_result(), 
                            third.get_result(), 
                            fourth.get_result()
                        );

                    Ok(ParserSuccess::new(result, position))
                }
            );

        Combinator::new(next_parser)
    }

    pub fn pipe_5<UResult, VResult, WResult, XResult, YResult>(self, snd_parser: Parser<UResult>, third_parser: Parser<VResult>, fourth_parser: Parser<WResult>, fifth_parser: Parser<XResult>, f: Box<dyn Fn (TResult, UResult, VResult, WResult, XResult) -> YResult>) -> Combinator<YResult> 
    where UResult: 'static, VResult: 'static, WResult: 'static, XResult: 'static
    {
        let next_parser =
            Box::new(
                move |parser_state: &mut ParserState| {
                    let self_parser = self.parser;

                    let first = self_parser(parser_state)?;
                    let second = snd_parser(parser_state)?;
                    let third = third_parser(parser_state)?;
                    let fourth = fourth_parser(parser_state)?;
                    let fifth = fifth_parser(parser_state)?;

                    let position = fifth.get_position();
                    let result = 
                        f(
                            first.get_result(), 
                            second.get_result(), 
                            third.get_result(), 
                            fourth.get_result(), 
                            fifth.get_result()
                        );

                    Ok(ParserSuccess::new(result, position))
                }
            );

        Combinator::new(next_parser)
    }

    pub fn tuple_2<UResult>(self, snd_parser: Parser<UResult>) -> Combinator<(TResult, UResult)> {
        self.pipe_2(
            snd_parser, 
            Box::new(|fst, snd| (fst, snd))
        )
    }

    pub fn tuple_3<UResult, VResult>(self, snd_parser: Parser<UResult>, third_parser: Parser<VResult>) -> Combinator<(TResult, UResult, VResult)> {
        self.pipe_3(
            snd_parser, 
            third_parser, 
            Box::new(|fst, snd, third| (fst, snd, third))
        )
    }

    pub fn tuple_4<UResult, VResult, WResult>(self, snd_parser: Parser<UResult>, third_parser: Parser<VResult>, fourth_parser: Parser<WResult>) -> Combinator<(TResult, UResult, VResult, WResult)> {
        self.pipe_4(
            snd_parser, 
            third_parser, 
            fourth_parser, 
            Box::new(|fst, snd, third, fourth| (fst, snd, third, fourth))
        )
    }

    pub fn tuple_5<UResult, VResult, WResult, XResult>(self, snd_parser: Parser<UResult>, third_parser: Parser<VResult>, fourth_parser: Parser<WResult>, fifth_parser: Parser<XResult>) -> Combinator<(TResult, UResult, VResult, WResult, XResult)> {
        self.pipe_5(
            snd_parser, 
            third_parser, 
            fourth_parser, 
            fifth_parser, 
            Box::new(|fst, snd, third, fourth, fifth| (fst, snd, third, fourth, fifth))
        )
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
