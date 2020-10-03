use rusty_parsec::*;

#[test]
fn choice_run_simple_parsers_succeeds() {
    let expected = Ok(ParserSuccess::new(
        String::from("nerds"), 
        Position::new(1, 6, 5)
    ));

    let actual = choice(vec![
        p_string(String::from("hello")), 
        p_string(String::from("goodbye")),
        p_string(String::from("nerds"))
    ]).run(String::from("nerds"));

    assert_eq!(actual, expected);
}

#[test]
fn choice_run_simple_parsers_fails_with_error() {
    let expected = Err(ParserFailure::new_err(
        String::from("value satisfying choice"), 
        None, 
        Position::new(1, 1, 0)
    ));

    let actual = choice(vec![
        p_string(String::from("hello")), 
        p_string(String::from("goodbye")),
        p_string(String::from("nerds"))
    ]).run(String::from("world"));

    assert_eq!(actual, expected);
}

#[test]
fn choice_run_complex_parsers_fails_with_fatal_error() {
    let expected = Err(ParserFailure::new_fatal_err(
        String::from("e"), 
        Some(String::from("f")), 
        Position::new(1, 2, 1)
    ));

    let actual = choice(vec![
        p_char('a').and(p_char('b')),
        p_char('d').and(p_char('e'))
    ]).run(String::from("df"));

    assert_eq!(actual, expected);
}

#[test]
fn attempt_run_complex_parsers_succeeds() {
    let expected = Ok(ParserSuccess::new(
        (123, String::from("abc")), 
        Position::new(1, 7, 6)
    ));

    let parser = p_u32()
        .and(p_string(String::from("abc")));

    let actual = attempt(parser)
        .run(String::from("123abc"));

    assert_eq!(actual, expected);
}

#[test]
fn attempt_run_complex_parsers_fails_with_error() {
    let expected = Err(ParserFailure::new_err(
        String::from("abc"), 
        Some(String::from("def")),
        Position::new(1, 4, 3)
    ));

    let parser = p_u32()
        .and(p_string(String::from("abc")));

    let actual = attempt(parser)
        .run(String::from("123def"));

    assert_eq!(actual, expected);
}