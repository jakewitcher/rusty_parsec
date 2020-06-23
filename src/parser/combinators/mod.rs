pub mod many;
pub mod sep_by;
pub mod many_till;
pub mod pipe;

use super::{ParserState, ParserSuccess, ParserFailure, Parser};

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