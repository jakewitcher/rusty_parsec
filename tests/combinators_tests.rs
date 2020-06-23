use rusty_parsec::*;

#[test]
fn succeeds_parsing_with_choice() {
    let expected = Ok(ParserSuccess::new("nerds".to_string(), Position::new(1, 6, 5)));

    let actual = 
        choice(vec![
            p_string("hello".to_string()), 
            p_string("goodbye".to_string()),
            p_string("nerds".to_string())
        ]).run("nerds".to_string());

    assert_eq!(expected, actual);
}

#[test]
fn fails_parsing_with_choice() {
    let expected = Err(ParserFailure::new_err("value satisfying choice".to_string(), None, Position::new(1, 1, 0)));

    let actual = 
        choice(vec![
            p_string("hello".to_string()), 
            p_string("goodbye".to_string()),
            p_string("nerds".to_string())
        ]).run("world".to_string());

    assert_eq!(expected, actual);
}

#[test]
fn fails_parsing_with_choice_fatal_err() {
    let expected = Err(ParserFailure::new_fatal_err("e".to_string(), Some("f".to_string()), Position::new(1, 2, 1)));

    let actual =
        choice(vec![
            p_char('a').and(p_char('b')),
            p_char('d').and(p_char('e'))
        ]).run("df".to_string());

    assert_eq!(expected, actual);
}

#[test]
fn succeeds_parsing_with_attempt() {
    let expected = Ok(ParserSuccess::new((123, "abc".to_string()), Position::new(1, 7, 6)));

    let parser = p_u32().and(p_string("abc".to_string()));

    let actual = attempt(parser).run("123abc".to_string());

    assert_eq!(expected, actual);
}

#[test]
fn fails_parsing_with_attempt() {
    let expected = Err(ParserFailure::new_err("abc".to_string(), Some("def".to_string()), Position::new(1, 4, 3)));

    let parser = p_u32().and(p_string("abc".to_string()));

    let actual = attempt(parser).run("123def".to_string());

    assert_eq!(expected, actual);
}