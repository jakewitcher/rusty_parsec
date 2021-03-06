use rusty_parsec::*;

#[test]
fn sep_by_run_simple_parserss_succeeds() {
    let expected = Ok(ParserSuccess::new(
        vec![1, 2, 3], 
        Position::new(1, 6, 5)
    ));

    let actual = sep_by(
        p_u32,
        || p_char(';')
    ).run(String::from("1;2;3"));

    assert_eq!(actual, expected);
}

#[test]
fn sep_by_run_simple_parsers_succeeds_when_no_values_returned_by_parser() {
    let expected = Ok(ParserSuccess::new(
        Vec::new(), 
        Position::new(1, 1, 0)
    ));

    let actual = sep_by(
        p_u32,
        || p_char(';')
    ).run(String::from("a;b;c"));

    assert_eq!(actual, expected);
}

#[test]
fn sep_by_run_complex_parsers_fails_with_fatal_error() {
    let expected = Err(ParserFailure::new_fatal_err(
        String::from("A"), 
        Some(String::from("a")), 
        Position::new(1, 2, 1)
    ));

    let actual = sep_by(
        || p_u32().and(p_char('A')),
        || p_char(';')
    ).run(String::from("1a;2b;3c"));

    assert_eq!(actual, expected);
}

#[test]
fn sep_by_run_complex_parsers_fails_parsing_separator_with_fatal_error() {
    let expected = Err(ParserFailure::new_fatal_err(
        String::from(">"), 
        Some(String::from("?")), 
        Position::new(1, 6, 5)
    ));

    let actual = sep_by(
        p_u32,
        || p_char('<').and(p_char('>'))
    ).run(String::from("1<>2<?3"));

    assert_eq!(actual, expected);
}

#[test]
fn sep_by_1_run_simple_parserss_succeeds() {
    let expected = Ok(ParserSuccess::new(
        vec![1, 2, 3], 
        Position::new(1, 6, 5)
    ));

    let actual = sep_by_1(
        p_u32,
        || p_char(';')
    ).run(String::from("1;2;3"));

    assert_eq!(actual, expected);
}

#[test]
fn sep_by_1_run_simple_parsers_fails_with_error_when_no_values_returned_by_parser() {
    let expected = Err(ParserFailure::new_err(
        String::from("value satisfying parser at least once"), 
        None, 
        Position::new(1, 1, 0)
    ));

    let actual = sep_by_1(
        p_u32,
        || p_char(';')
    ).run(String::from("a;b;c"));

    assert_eq!(actual, expected);
}

#[test]
fn skip_sep_by_run_simple_parserss_succeeds() {
    let expected = Ok(ParserSuccess::new(
        (), 
        Position::new(1, 6, 5)
    ));

    let actual = skip_sep_by(
        p_u32,
        || p_char(';')
    ).run(String::from("1;2;3"));

    assert_eq!(actual, expected);
}

#[test]
fn skip_sep_by_run_complex_parsers_fails_with_fatal_error() {
    let expected = Err(ParserFailure::new_fatal_err(
        String::from("A"), 
        Some(String::from("a")), 
        Position::new(1, 2, 1)
    ));

    let actual = skip_sep_by(
        || p_u32().and(p_char('A')),
        || p_char(';')
    ).run(String::from("1a;2b;3c"));

    assert_eq!(actual, expected);
}

#[test]
fn skip_sep_by_run_complex_parsers_fails_parsing_separator_with_fatal_error() {
    let expected = Err(ParserFailure::new_fatal_err(
        String::from(">"), 
        Some(String::from("?")), 
        Position::new(1, 6, 5)
    ));

    let actual = skip_sep_by(
        p_u32,
        || p_char('<').and(p_char('>'))
    ).run(String::from("1<>2<?3"));

    assert_eq!(actual, expected);
}

#[test]
fn skip_sep_by_1_run_simple_parserss_succeeds() {
    let expected = Ok(ParserSuccess::new(
        (), 
        Position::new(1, 6, 5)
    ));

    let actual = skip_sep_by_1(
        p_u32,
        || p_char(';')
    ).run(String::from("1;2;3"));

    assert_eq!(actual, expected);
}

#[test]
fn skip_sep_by_1_run_simple_parsers_fails_with_error_when_no_values_returned_by_parser() {
    let expected = Err(ParserFailure::new_err(
        String::from("value satisfying parser at least once"), 
        None, 
        Position::new(1, 1, 0)
    ));

    let actual = skip_sep_by_1(
        p_u32,
        || p_char(';')
    ).run(String::from("a;b;c"));

    assert_eq!(actual, expected);
}