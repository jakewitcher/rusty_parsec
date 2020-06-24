use super::{ParserState, ParserSuccess, ParserResult, Parser};

pub fn pipe_2<T, U, V>(p1: Parser<T>, p2: Parser<U>, f: Box<dyn Fn (T, U) -> V>) -> Parser<V> 
where T: 'static, U: 'static
{
    let parser_fn =
        Box::new(
            move |state: &mut ParserState| {
                let r1 = p1.parse(state)?;
                let r2 = apply_parser(p2, state)?;

                let result = 
                    f(
                        r1.get_result(), 
                        r2.get_result()
                    );

                Ok(ParserSuccess::new(result, state.get_position()))
            }
        );

    Parser::new(parser_fn)
}

pub fn pipe_3<T, U, V, W>(p1: Parser<T>, p2: Parser<U>, p3: Parser<V>, f: Box<dyn Fn (T, U, V) -> W>) -> Parser<W> 
where T: 'static, U: 'static, V: 'static
{
    let parser_fn =
        Box::new(
            move |state: &mut ParserState| {
                let r1 = p1.parse(state)?;
                let r2 = apply_parser(p2, state)?;
                let r3 = apply_parser(p3, state)?;

                let result = 
                    f(
                        r1.get_result(), 
                        r2.get_result(), 
                        r3.get_result()
                    );

                Ok(ParserSuccess::new(result, state.get_position()))
            }
        );

    Parser::new(parser_fn)
}

pub fn pipe_4<T, U, V, W, X>(p1: Parser<T>, p2: Parser<U>, p3: Parser<V>, p4: Parser<W>, f: Box<dyn Fn (T, U, V, W) -> X>) -> Parser<X> 
where T: 'static, U: 'static, V: 'static, W: 'static
{
    let parser_fn =
        Box::new(
            move |state: &mut ParserState| {
                let r1 = p1.parse(state)?;
                let r2 = apply_parser(p2, state)?;
                let r3 = apply_parser(p3, state)?;
                let r4 = apply_parser(p4, state)?;

                let result = 
                    f(
                        r1.get_result(), 
                        r2.get_result(), 
                        r3.get_result(), 
                        r4.get_result()
                    );

                Ok(ParserSuccess::new(result, state.get_position()))
            }
        );

    Parser::new(parser_fn)
}

pub fn pipe_5<T, U, V, W, X, Y>(p1: Parser<T>, p2: Parser<U>, p3: Parser<V>, p4: Parser<W>, p5: Parser<X>, f: Box<dyn Fn (T, U, V, W, X) -> Y>) -> Parser<Y> 
where T: 'static, U: 'static, V: 'static, W: 'static, X: 'static
{
    let parser_fn =
        Box::new(
            move |state: &mut ParserState| {
                let r1 = p1.parse(state)?;
                let r2 = apply_parser(p2, state)?;
                let r3 = apply_parser(p3, state)?;
                let r4 = apply_parser(p4, state)?;
                let r5 = apply_parser(p5, state)?;

                let result = 
                    f(
                        r1.get_result(), 
                        r2.get_result(), 
                        r3.get_result(), 
                        r4.get_result(), 
                        r5.get_result()
                    );

                Ok(ParserSuccess::new(result, state.get_position()))
            }
        );

    Parser::new(parser_fn)
}

pub fn tuple_2<T, U>(p1: Parser<T>, p2: Parser<U>) -> Parser<(T, U)> {
    pipe_2(p1, p2, Box::new(|x1, x2| (x1, x2)))
}

pub fn tuple_3<T, U, V>(p1: Parser<T>, p2: Parser<U>, p3: Parser<V>) -> Parser<(T, U, V)> {
    pipe_3(p1, p2, p3, Box::new(|x1, x2, x3| (x1, x2, x3)))
}

pub fn tuple_4<T, U, V, W>(p1: Parser<T>, p2: Parser<U>, p3: Parser<V>, p4: Parser<W>) -> Parser<(T, U, V, W)> {
    pipe_4(p1, p2, p3, p4, Box::new(|x1, x2, x3, x4| (x1, x2, x3, x4)))
}

pub fn tuple_5<T, U, V, W, X>(p1: Parser<T>, p2: Parser<U>, p3: Parser<V>, p4: Parser<W>, p5: Parser<X>) -> Parser<(T, U, V, W, X)> {
    pipe_5(p1, p2, p3, p4, p5, Box::new(|x1, x2, x3, x4, x5| (x1, x2, x3, x4, x5)))
}

fn apply_parser<T>(p: Parser<T>, state: &mut ParserState) -> ParserResult<T> {
    p.parse(state).map_err(|failure| failure.to_fatal_err())
}