use super::{ParserState, ParserSuccess, ParserFailure, Parser};

pub fn many<T>(parser: fn() -> Parser<T>) -> Parser<Vec<T>> {
    let parser_fn =
        Box::new(
            move |state: &mut ParserState| {
                let results: Vec<T> = apply_parser(parser, state);
                Ok(ParserSuccess::new(results, state.get_position()))
            }
        );

    Parser::new(parser_fn)
}

pub fn many_1<T>(parser: fn() -> Parser<T>) -> Parser<Vec<T>> {
    let parser_fn =
        Box::new(
            move |state: &mut ParserState| {
                let results: Vec<T> = apply_parser(parser, state);

                match results.len() {
                    0 => 
                        Err(ParserFailure::new_err(
                            "value satisfying parser at least once".to_string(),
                            None,
                            state.get_position()
                        )),
                    _ => 
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
                apply_parser(parser, state);
                Ok(ParserSuccess::new((), state.get_position()))
            }
        );

    Parser::new(parser_fn)
}

pub fn skip_many_1<T>(parser: fn() -> Parser<T>) -> Parser<()> {
    let parser_fn =
        Box::new(
            move |state: &mut ParserState| {
                match parser().parse(state) {
                    Ok(_) => {
                        apply_parser(parser, state);
                        Ok(ParserSuccess::new((), state.get_position()))
                    },
                    _ => {
                        Err(ParserFailure::new_err(
                            "value satisfying parser at least once".to_string(),
                            None,
                            state.get_position()
                        ))
                    },
                }
            }
        );

    Parser::new(parser_fn)
}

fn apply_parser<T>(p: fn() -> Parser<T>, state: &mut ParserState) -> Vec<T> {
    let mut results: Vec<T> = Vec::new();
    let mut parser_succeeds = true;

    while parser_succeeds {
        match p().parse(state) {
            Ok(success) => {
                results.push(success.get_result());
            },
            Err(_) => {
                parser_succeeds = false;
            },
        }
    }

    results
}