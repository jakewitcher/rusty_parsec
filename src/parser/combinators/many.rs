use super::{ParserState, ParserSuccess, ParserFailure, Parser};

pub fn many<T>(parser: fn() -> Parser<T>) -> Parser<Vec<T>> {
    let parser_fn =
        Box::new(
            move |state: &mut ParserState| {
                let results: Vec<T> = apply_parser(parser, state)?;
                Ok(ParserSuccess::new(results, state.get_position()))
            }
        );

    Parser::new(parser_fn)
}

pub fn many_1<T>(parser: fn() -> Parser<T>) -> Parser<Vec<T>> {
    let parser_fn =
        Box::new(
            move |state: &mut ParserState| {
                match parser().parse(state) {
                    Ok(success) => {
                        let mut results = apply_parser(parser, state)?;
                        results.insert(0, success.get_result());
                        Ok(ParserSuccess::new(results, state.get_position()))
                    },
                    Err(failure) => Err(failure),
                }
            }
        );

    Parser::new(parser_fn)
}

pub fn skip_many<T>(parser: fn() -> Parser<T>) -> Parser<()> {
    let parser_fn =
        Box::new(
            move |state: &mut ParserState| {
                let _ = apply_parser(parser, state)?;
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
                        let _ = apply_parser(parser, state)?;
                        Ok(ParserSuccess::new((), state.get_position()))
                    },
                    Err(failure) => Err(failure),
                }
            }
        );

    Parser::new(parser_fn)
}

fn apply_parser<T>(p: fn() -> Parser<T>, state: &mut ParserState) -> Result<Vec<T>, ParserFailure> {
    let mut results: Vec<T> = Vec::new();
    let mut parser_succeeds = true;

    while parser_succeeds {
        match p().parse(state) {
            Ok(success) => {
                results.push(success.get_result());
            },
            Err(failure) => {
                if failure.is_fatal() {
                    return Err(failure)
                }
                parser_succeeds = false;
            },
        }
    }

    Ok(results)
}