use super::{ParserState, ParserSuccess, Parser};

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
                                Err(failure) => {
                                    if failure.is_fatal() {
                                        return Err(failure)
                                    }

                                    results.push(success.get_result());
                                }
                            }
                        },
                        Err(failure) => {
                            if results.len() == 0 {
                                return Ok(ParserSuccess::new(results, state.get_position()))
                            } else
                            {
                                return Err(failure.to_fatal_err())
                            }
                        },
                    }
                }

                Ok(ParserSuccess::new(results, state.get_position()))
            }
        );

    Parser::new(parser_fn)
}

pub fn many_1_till<T, U>(parser: fn() -> Parser<T>, end_parser: fn() -> Parser<U>) -> Parser<Vec<T>> {
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
                                Err(failure) => {
                                    if failure.is_fatal() {
                                        return Err(failure)
                                    }

                                    results.push(success.get_result());
                                }
                            }
                        },
                        Err(failure) => {
                            let err = if results.len() == 0 {
                                Err(failure)
                            } else {
                                Err(failure.to_fatal_err())
                            };

                            return err
                        },
                    }
                }

                Ok(ParserSuccess::new(results, state.get_position()))
            }
        );

    Parser::new(parser_fn)
}

pub fn skip_many_till<T, U>(parser: fn() -> Parser<T>, end_parser: fn() -> Parser<U>) -> Parser<()> {
    let parser_fn =
        Box::new(
            move |state: &mut ParserState| {
                let mut end_parser_succeeds = false;
                let mut state_changed = false;

                while !end_parser_succeeds {
                    match parser().parse(state) {
                        Ok(_) => {
                            state_changed = true;

                            match end_parser().parse(state) {
                                Ok(_) => {
                                    end_parser_succeeds = true;
                                },
                                Err(failure) => {
                                    if failure.is_fatal() {
                                        return Err(failure)
                                    }
                                }
                            }
                        },
                        Err(failure) => {
                            if !state_changed {
                                return Ok(ParserSuccess::new((), state.get_position()))
                            } else
                            {
                                return Err(failure.to_fatal_err())
                            }
                        },
                    }
                }

                Ok(ParserSuccess::new((), state.get_position()))
            }
        );

    Parser::new(parser_fn)
}