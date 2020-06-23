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