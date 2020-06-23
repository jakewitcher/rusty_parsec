use super::{ParserState, ParserSuccess, ParserFailure, Parser};

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