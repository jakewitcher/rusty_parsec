use super::{Parser, ParserState, ParserSuccess, ParserResult};

pub struct Combinator<T>
where T: 'static
{
    parser: Parser<T>
}

impl<T> Combinator<T> {
    pub fn new(parser: Parser<T>) -> Combinator<T> {
        Combinator { parser }
    }

    pub fn get_parser(self) -> Parser<T> {
        self.parser
    }

    pub fn and<U>(self, other: Parser<U>) -> Combinator<(T, U)>
    where U: 'static
    {
        let next_parser =
            Box::new(
                move |state: &mut ParserState| {
                    let p = self.get_parser();

                    let left = p(state)?;
                    let right = other(state)?;

                    let position = right.get_position();
                    let result = (left.get_result(), right.get_result());

                    Ok(ParserSuccess::new(result, position))
                }
            );

        Combinator::new(next_parser)
    }

    pub fn or(self, other: Parser<T>) -> Combinator<T>
    {
        let next_parser =
            Box::new(
                move |state: &mut ParserState| {
                    let p = self.get_parser();

                    p(state).or(other(state))
                }
            );

        Combinator::new(next_parser)
    }

    pub fn take_prev<U>(self, other: Parser<U>) -> Combinator<T>
    where U: 'static
    {
        let next_parser =
            Box::new(
                move |state: &mut ParserState| {
                    let p = self.get_parser();

                    let prev = p(state)?;
                    let next = other(state)?;
                    
                    Ok(prev.update_position(next.get_position()))
                }
            );

        Combinator::new(next_parser)
    }

    pub fn take_next<U>(self, other: Parser<U>) -> Combinator<U>
    where U: 'static
    {
        let next_parser =
            Box::new(
                move |state: &mut ParserState| {
                    let p = self.get_parser();

                    p(state).and(other(state))
                }
            );

        Combinator::new(next_parser)
    }

    pub fn then_return<U>(self, return_value: U) -> Combinator<U>
    where U: 'static
    {
        let next_parser =
            Box::new(
                move |state: &mut ParserState| {
                    let p = self.get_parser();

                    let result = p(state)?;
                    
                    Ok(ParserSuccess::new(return_value, result.get_position()))
                }
            );

        Combinator::new(next_parser)
    }

    pub fn between<U, V>(self, p_open: Parser<U>, p_close: Parser<V>) -> Combinator<T>
    where U: 'static, V: 'static
    {
        let next_parser =
            Box::new(
                move |state: &mut ParserState| {
                    let p = self.get_parser();

                    p_open(state)?;

                    let result = p(state)?;
                    let close = p_close(state)?;

                    Ok(result.update_position(close.get_position()))
                }
            );

        Combinator::new(next_parser)
    }

    pub fn pipe_2<U, V>(self, p2: Parser<U>, f: Box<dyn Fn (T, U) -> V>) -> Combinator<V> 
    where U: 'static
    {
        let next_parser =
            Box::new(
                move |state: &mut ParserState| {
                    let p1 = self.parser;

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

        Combinator::new(next_parser)
    }

    pub fn pipe_3<U, V, W>(self, p2: Parser<U>, p3: Parser<V>, f: Box<dyn Fn (T, U, V) -> W>) -> Combinator<W> 
    where U: 'static, V: 'static
    {
        let next_parser =
            Box::new(
                move |state: &mut ParserState| {
                    let p1 = self.parser;

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

        Combinator::new(next_parser)
    }

    pub fn pipe_4<U, V, W, X>(self, p2: Parser<U>, p3: Parser<V>, p4: Parser<W>, f: Box<dyn Fn (T, U, V, W) -> X>) -> Combinator<X> 
    where U: 'static, V: 'static, W: 'static
    {
        let next_parser =
            Box::new(
                move |state: &mut ParserState| {
                    let p1 = self.parser;

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

        Combinator::new(next_parser)
    }

    pub fn pipe_5<U, V, W, X, Y>(self, p2: Parser<U>, p3: Parser<V>, p4: Parser<W>, p5: Parser<X>, f: Box<dyn Fn (T, U, V, W, X) -> Y>) -> Combinator<Y> 
    where U: 'static, V: 'static, W: 'static, X: 'static
    {
        let next_parser =
            Box::new(
                move |state: &mut ParserState| {
                    let p1 = self.parser;

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

        Combinator::new(next_parser)
    }

    pub fn tuple_2<U>(self, p2: Parser<U>) -> Combinator<(T, U)> {
        self.pipe_2(p2, Box::new(|x1, x2| (x1, x2)))
    }

    pub fn tuple_3<U, V>(self, p2: Parser<U>, p3: Parser<V>) -> Combinator<(T, U, V)> {
        self.pipe_3(p2, p3, Box::new(|x1, x2, x3| (x1, x2, x3)))
    }

    pub fn tuple_4<U, V, W>(self, p2: Parser<U>, p3: Parser<V>, p4: Parser<W>) -> Combinator<(T, U, V, W)> {
        self.pipe_4(p2, p3, p4, Box::new(|x1, x2, x3, x4| (x1, x2, x3, x4)))
    }

    pub fn tuple_5<U, V, W, X>(self, p2: Parser<U>, p3: Parser<V>, p4: Parser<W>, p5: Parser<X>) -> Combinator<(T, U, V, W, X)> {
        self.pipe_5(p2, p3, p4, p5, Box::new(|x1, x2, x3, x4, x5| (x1, x2, x3, x4, x5)))
    }

    pub fn map<U>(self, f: Box<dyn Fn(T) -> U>) -> Combinator<U>
    where U: 'static
    {
        let next_parser =
            Box::new(
                move |state: &mut ParserState| {
                    let p = self.parser;
                    let result = p(state)?;

                    Ok(result.map_result(f))
                }
            );

        Combinator::new(next_parser)
    }

    pub fn run(self, input: String) -> ParserResult<T> {
        let parser = self.parser;
        let mut state = ParserState::new(input);

        parser(&mut state)
    }
}
