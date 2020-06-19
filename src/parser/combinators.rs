use super::{ParserState, ParserSuccess, ParserFailure, Parser};

pub fn many<T>(parser: fn() -> Parser<T>) -> Parser<Vec<T>> {
    let parser_fn =
        Box::new(
            move |state: &mut ParserState| {
                let mut results: Vec<T> = Vec::new();
                let mut parser_succeeds = true;

                while parser_succeeds {
                    match parser().parse(state) {
                        Ok(success) => {
                            results.push(success.get_result());
                        },
                        Err(_) => {
                            parser_succeeds = false;
                        },
                    }
                }

                Ok(ParserSuccess::new(results, state.get_position()))
            }
        );

    Parser::new(parser_fn)
}

pub fn many_1<T>(parser: fn() -> Parser<T>) -> Parser<Vec<T>> {
    let parser_fn =
        Box::new(
            move |state: &mut ParserState| {
                let mut results: Vec<T> = Vec::new();
                let mut parser_succeeds = true;

                while parser_succeeds {
                    match parser().parse(state) {
                        Ok(success) => {
                            results.push(success.get_result());
                        },
                        Err(_) => {
                            parser_succeeds = false;
                        },
                    }
                }

                if results.len() == 0 {
                    Err(ParserFailure::new_err(
                        "value satisfying parser at least once".to_string(),
                        None,
                        state.get_position()
                    ))
                } else {
                    Ok(ParserSuccess::new(results, state.get_position()))
                }
            }
        );

    Parser::new(parser_fn)
}

pub fn skip_many<T>(parser: fn() -> Parser<T>) -> Parser<()> {
    let parser_fn =
        Box::new(
            move |state: &mut ParserState| {
                let mut parser_succeeds = true;

                while parser_succeeds {
                    match parser().parse(state) {
                        Ok(_) => {
                            continue;
                        },
                        Err(_) => {
                            parser_succeeds = false;
                        },
                    }
                }

                Ok(ParserSuccess::new((), state.get_position()))
            }
        );

    Parser::new(parser_fn)
}

pub fn skip_many_1<T>(parser: fn() -> Parser<T>) -> Parser<()> {
    let parser_fn =
        Box::new(
            move |state: &mut ParserState| {
                if let Ok(_) = parser().parse(state) {
                    let mut parser_succeeds = true;

                    while parser_succeeds {
                        match parser().parse(state) {
                            Ok(_) => {
                                continue;
                            },
                            Err(_) => {
                                parser_succeeds = false;
                            },
                        }
                    }

                    return Ok(ParserSuccess::new((), state.get_position()))
                }
                
                Err(ParserFailure::new_err(
                    "value satisfying parser at least once".to_string(),
                    None,
                    state.get_position()
                ))
            }
        );

    Parser::new(parser_fn)
}

pub fn sep_by<T, U>(parser: fn() -> Parser<T>, separator: fn() -> Parser<U>) -> Parser<Vec<T>> 
where U: 'static
{
    let parser_fn =
        Box::new(
            move |state: &mut ParserState| {
                let mut results: Vec<T> = Vec::new();
                let mut parser_succeeds = true;

                while parser_succeeds {
                    match parser().parse(state) {
                        Ok(success) => {
                            results.push(success.get_result());

                            if separator().parse(state).is_err() {
                                parser_succeeds = false;
                            }
                        },
                        Err(_) => {
                            parser_succeeds = false;
                        },
                    }
                }

                Ok(ParserSuccess::new(results, state.get_position()))
            }
        );
    
    Parser::new(parser_fn)
}

pub fn sep_by_1<T, U>(parser: fn() -> Parser<T>, separator: fn() -> Parser<U>) -> Parser<Vec<T>> 
where U: 'static
{
    let parser_fn =
        Box::new(
            move |state: &mut ParserState| {
                let mut results: Vec<T> = Vec::new();
                let mut parser_succeeds = true;

                while parser_succeeds {
                    match parser().parse(state) {
                        Ok(success) => {
                            results.push(success.get_result());

                            if separator().parse(state).is_err() {
                                parser_succeeds = false;
                            }
                        },
                        Err(_) => {
                            parser_succeeds = false;
                        },
                    }
                }

                if results.len() == 0 {
                    Err(ParserFailure::new_err(
                        "value satisfying parser at least once".to_string(),
                        None,
                        state.get_position()
                    ))
                } else {
                    Ok(ParserSuccess::new(results, state.get_position()))
                }
            }
        );
    
    Parser::new(parser_fn)
}

pub fn skip_sep_by<T, U>(parser: fn() -> Parser<T>, separator: fn() -> Parser<U>) -> Parser<()> 
where U: 'static
{
    let parser_fn =
        Box::new(
            move |state: &mut ParserState| {
                let mut parser_succeeds = true;

                while parser_succeeds {
                    match parser().parse(state) {
                        Ok(_) => {
                            if separator().parse(state).is_err() {
                                parser_succeeds = false;
                            }
                        },
                        Err(_) => {
                            parser_succeeds = false;
                        },
                    }
                }

                Ok(ParserSuccess::new((), state.get_position()))
            }
        );
    
    Parser::new(parser_fn)
}

pub fn skip_sep_by_1<T, U>(parser: fn() -> Parser<T>, separator: fn() -> Parser<U>) -> Parser<()> 
where U: 'static
{
    let parser_fn =
        Box::new(
            move |state: &mut ParserState| {
                if let Ok(_) = parser().parse(state) {
                    let mut parser_succeeds = true;

                    if separator().parse(state).is_err() {
                        parser_succeeds = false;
                    }

                    while parser_succeeds {
                        match parser().parse(state) {
                            Ok(_) => {
    
                                if separator().parse(state).is_err() {
                                    parser_succeeds = false;
                                }
                            },
                            Err(_) => {
                                parser_succeeds = false;
                            },
                        }
                    }

                    return Ok(ParserSuccess::new((), state.get_position()))
                }

                Err(ParserFailure::new_err(
                    "value satisfying parser at least once".to_string(),
                    None,
                    state.get_position()
                ))
            }
        );
    
    Parser::new(parser_fn)
}

pub fn many_till<T, U>(parser: fn() -> Parser<T>, end_parser: fn() -> Parser<U>) -> Parser<Vec<T>> {
    let parser_fn =
        Box::new(
            move |state: &mut ParserState| {
                let mut results: Vec<T> = Vec::new();
                let mut end_parser_succeeds = false;

                while !end_parser_succeeds {
                    match parser().parse(state) {
                        Ok(success) => {
                            match end_parser().parse(state) {
                                Ok(_) => {
                                    results.push(success.get_result());

                                    end_parser_succeeds = true;
                                },
                                Err(err) => {
                                    if err.is_fatal() {
                                        return Err(err)
                                    }

                                    results.push(success.get_result());
                                }
                            }
                            
                        },
                        Err(err) => return Err(err),
                    }
                }

                Ok(ParserSuccess::new(results, state.get_position()))
            }
        );

    Parser::new(parser_fn)
}

pub fn choice<T>(parsers: Vec<Parser<T>>) -> Parser<T> {
    choice_l(parsers, "value satisfying choice".to_string())
}

pub fn choice_l<T>(parsers: Vec<Parser<T>>, label: String) -> Parser<T> {
    let parser_fn =
        Box::new(
            move |state: &mut ParserState| {
                for p in parsers.into_iter() {
                    match p.parse(state) {
                        Ok(success) => {
                            return Ok(success)
                        },
                        Err(failure) => {
                            if failure.is_fatal() {
                                return Err(failure)
                            }

                            continue;
                        },
                    } 
                }

                Err(ParserFailure::new_err(
                    label,
                    None,
                    state.get_position()
                ))
            }
        );

    Parser::new(parser_fn)
}

pub fn attempt<T>(parser: Parser<T>) -> Parser<T>
where T: 'static
{
    let parser_fn = 
        Box::new(
            move |state: &mut ParserState| {
                state.mark();
                match parser.parse(state) {
                    Ok(success) => {
                        state.remove_mark();
                        Ok(success)
                    },
                    Err(failure) => {
                        state.revert();
                        Err(failure.to_err())
                    },
                }
            }
        );

    Parser::new(parser_fn)
}

pub fn pipe_2<T, U, V>(p1: Parser<T>, p2: Parser<U>, f: Box<dyn Fn (T, U) -> V>) -> Parser<V> 
where T: 'static, U: 'static
{
    let parser_fn =
        Box::new(
            move |state: &mut ParserState| {
                let r1 = p1.parse(state)?;

                let r2 = match p2.parse(state) {
                    Ok(success) => success,
                    Err(failure) => return Err(failure.to_fatal_err()),
                };

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

                let r2 = match p2.parse(state) {
                    Ok(success) => success,
                    Err(failure) => return Err(failure.to_fatal_err()),
                };
                
                let r3 = match p3.parse(state) {
                    Ok(success) => success,
                    Err(failure) => return Err(failure.to_fatal_err()),
                };

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
                
                let r2 = match p2.parse(state) {
                    Ok(success) => success,
                    Err(failure) => return Err(failure.to_fatal_err()),
                };

                let r3 = match p3.parse(state) {
                    Ok(success) => success,
                    Err(failure) => return Err(failure.to_fatal_err()),
                };

                let r4 = match p4.parse(state) {
                    Ok(success) => success,
                    Err(failure) => return Err(failure.to_fatal_err()),
                };

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
                
                let r2 = match p2.parse(state) {
                    Ok(success) => success,
                    Err(failure) => return Err(failure.to_fatal_err()),
                };

                let r3 = match p3.parse(state) {
                    Ok(success) => success,
                    Err(failure) => return Err(failure.to_fatal_err()),
                };

                let r4 = match p4.parse(state) {
                    Ok(success) => success,
                    Err(failure) => return Err(failure.to_fatal_err()),
                };

                let r5 = match p5.parse(state) {
                    Ok(success) => success,
                    Err(failure) => return Err(failure.to_fatal_err()),
                };

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