use super::{Combinator, Parser, ParserState, ParserSuccess};

pub fn pipe_2<T, U, V>(p1: Parser<T>, p2: Parser<U>, f: Box<dyn Fn (T, U) -> V>) -> Combinator<V> 
where T: 'static, U: 'static
{
    let parser =
        Box::new(
            move |state: &mut ParserState| {
                let r1 = p1(state)?;
                let r2 = p2(state)?;

                let position = r2.get_position();
                let result = 
                    f(
                        r1.get_result(), 
                        r2.get_result()
                    );

                Ok(ParserSuccess::new(result, position))
            }
        );

    Combinator::new(parser)
}

pub fn pipe_3<T, U, V, W>(p1: Parser<T>, p2: Parser<U>, p3: Parser<V>, f: Box<dyn Fn (T, U, V) -> W>) -> Combinator<W> 
where T: 'static, U: 'static, V: 'static
{
    let parser =
        Box::new(
            move |state: &mut ParserState| {
                let r1 = p1(state)?;
                let r2 = p2(state)?;
                let r3 = p3(state)?;

                let position = r3.get_position();
                let result = 
                    f(
                        r1.get_result(), 
                        r2.get_result(), 
                        r3.get_result()
                    );

                Ok(ParserSuccess::new(result, position))
            }
        );

    Combinator::new(parser)
}

pub fn pipe_4<T, U, V, W, X>(p1: Parser<T>, p2: Parser<U>, p3: Parser<V>, p4: Parser<W>, f: Box<dyn Fn (T, U, V, W) -> X>) -> Combinator<X> 
where T: 'static, U: 'static, V: 'static, W: 'static
{
    let parser =
        Box::new(
            move |state: &mut ParserState| {
                let r1 = p1(state)?;
                let r2 = p2(state)?;
                let r3 = p3(state)?;
                let r4 = p4(state)?;

                let position = r4.get_position();
                let result = 
                    f(
                        r1.get_result(), 
                        r2.get_result(), 
                        r3.get_result(), 
                        r4.get_result()
                    );

                Ok(ParserSuccess::new(result, position))
            }
        );

    Combinator::new(parser)
}

pub fn pipe_5<T, U, V, W, X, Y>(p1: Parser<T>, p2: Parser<U>, p3: Parser<V>, p4: Parser<W>, p5: Parser<X>, f: Box<dyn Fn (T, U, V, W, X) -> Y>) -> Combinator<Y> 
where T: 'static, U: 'static, V: 'static, W: 'static, X: 'static
{
    let parser =
        Box::new(
            move |state: &mut ParserState| {
                let r1 = p1(state)?;
                let r2 = p2(state)?;
                let r3 = p3(state)?;
                let r4 = p4(state)?;
                let r5 = p5(state)?;

                let position = r5.get_position();
                let result = 
                    f(
                        r1.get_result(), 
                        r2.get_result(), 
                        r3.get_result(), 
                        r4.get_result(), 
                        r5.get_result()
                    );

                Ok(ParserSuccess::new(result, position))
            }
        );

    Combinator::new(parser)
}

pub fn tuple_2<T, U>(p1: Parser<T>, p2: Parser<U>) -> Combinator<(T, U)> {
    pipe_2(p1, p2, Box::new(|x1, x2| (x1, x2)))
}

pub fn tuple_3<T, U, V>(p1: Parser<T>, p2: Parser<U>, p3: Parser<V>) -> Combinator<(T, U, V)> {
    pipe_3(p1, p2, p3, Box::new(|x1, x2, x3| (x1, x2, x3)))
}

pub fn tuple_4<T, U, V, W>(p1: Parser<T>, p2: Parser<U>, p3: Parser<V>, p4: Parser<W>) -> Combinator<(T, U, V, W)> {
    pipe_4(p1, p2, p3, p4, Box::new(|x1, x2, x3, x4| (x1, x2, x3, x4)))
}

pub fn tuple_5<T, U, V, W, X>(p1: Parser<T>, p2: Parser<U>, p3: Parser<V>, p4: Parser<W>, p5: Parser<X>) -> Combinator<(T, U, V, W, X)> {
    pipe_5(p1, p2, p3, p4, p5, Box::new(|x1, x2, x3, x4, x5| (x1, x2, x3, x4, x5)))
}