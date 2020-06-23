use rusty_parsec::*;

#[test]
fn succeeds_parsing_with_sep_by() {
    let expected = Ok(ParserSuccess::new(vec![1, 2, 3], Position::new(1, 6, 5)));

    let actual =
        sep_by(
            || p_u32(),
            || p_char(';')
        ).run("1;2;3".to_string());

    assert_eq!(expected, actual);
}

#[test]
fn succeeds_parsing_with_sep_by_returns_empty_vec() {
    let expected = Ok(ParserSuccess::new(Vec::new(), Position::new(1, 1, 0)));

    let actual =
        sep_by(
            || p_u32(),
            || p_char(';')
        ).run("a;b;c".to_string());

    assert_eq!(expected, actual);
}

#[test]
fn succeeds_parsing_with_sep_by_1() {
    let expected = Ok(ParserSuccess::new(vec![1, 2, 3], Position::new(1, 6, 5)));

    let actual =
        sep_by_1(
            || p_u32(),
            || p_char(';')
        ).run("1;2;3".to_string());

    assert_eq!(expected, actual);
}

#[test]
fn fails_parsing_with_sep_by_1() {
    let expected = Err(ParserFailure::new_err("value satisfying parser at least once".to_string(), None, Position::new(1, 1, 0)));

    let actual =
        sep_by_1(
            || p_u32(),
            || p_char(';')
        ).run("a;b;c".to_string());

    assert_eq!(expected, actual);
}

#[test]
fn succeeds_parsing_with_skip_sep_by() {
    let expected = Ok(ParserSuccess::new((), Position::new(1, 6, 5)));

    let actual =
        skip_sep_by(
            || p_u32(),
            || p_char(';')
        ).run("1;2;3".to_string());

    assert_eq!(expected, actual);
}

#[test]
fn succeeds_parsing_with_skip_sep_by_1() {
    let expected = Ok(ParserSuccess::new((), Position::new(1, 6, 5)));

    let actual =
        skip_sep_by_1(
            || p_u32(),
            || p_char(';')
        ).run("1;2;3".to_string());

    assert_eq!(expected, actual);
}

#[test]
fn fails_parsing_with_skip_sep_by_1() {
    let expected = Err(ParserFailure::new_err("value satisfying parser at least once".to_string(), None, Position::new(1, 1, 0)));

    let actual =
        skip_sep_by_1(
            || p_u32(),
            || p_char(';')
        ).run("a;b;c".to_string());

    assert_eq!(expected, actual);
}