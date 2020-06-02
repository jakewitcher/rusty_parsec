use rusty_parsec::*;

#[test]
fn succeeds_parsing_with_and() {
    let expected = Ok(ParserSuccess::new(('a', 'b'), Position::new(1, 3, 2)));

    let actual = 
        Combinator::new(p_char('a'))
            .and(p_char('b'))
            .run("abc".to_string());

    assert_eq!(expected, actual);
}

#[test]
fn fails_parsing_with_and_at_first_parser() {
    let expected = Err(ParserFailure::new("a".to_string(), Some("b".to_string()), Position::new(1, 1, 0)));
    let err_msg = "expected 'a' but found 'b' at line 1, column 1".to_string();

    let actual = 
        Combinator::new(p_char('a'))
            .and(p_char('b'))
            .run("bca".to_string());

    assert_eq!(expected, actual);
    assert_eq!(err_msg, actual.unwrap_err().to_err_msg());
}

#[test]
fn fails_parsing_with_and_at_second_parser() {
    let expected = Err(ParserFailure::new("b".to_string(), Some("c".to_string()), Position::new(1, 2, 1)));
    let err_msg = "expected 'b' but found 'c' at line 1, column 2".to_string();

    let actual = 
        Combinator::new(p_char('a'))
            .and(p_char('b'))
            .run("acb".to_string());

    assert_eq!(expected, actual);
    assert_eq!(err_msg, actual.unwrap_err().to_err_msg());
}

#[test]
fn succeeds_parsing_with_or_at_first_parser() {
    let expected = Ok(ParserSuccess::new('a', Position::new(1, 2, 1)));

    let actual = 
        Combinator::new(p_char('a'))
            .or(p_char('b'))
            .run("abc".to_string());

    assert_eq!(expected, actual);
}

#[test]
fn succeeds_parsing_with_or_at_second_parser() {
    let expected = Ok(ParserSuccess::new('b', Position::new(1, 2, 1)));

    let actual = 
        Combinator::new(p_char('a'))
            .or(p_char('b'))
            .run("bac".to_string());

    assert_eq!(expected, actual);
}

#[test]
fn fails_parsing_with_or_at_second_parser() {
    let expected = Err(ParserFailure::new("b".to_string(), Some("c".to_string()), Position::new(1, 1, 0)));
    let err_msg = "expected 'b' but found 'c' at line 1, column 1".to_string();

    let actual = 
        Combinator::new(p_char('a'))
            .or(p_char('b'))
            .run("cba".to_string());

    assert_eq!(expected, actual);
    assert_eq!(err_msg, actual.unwrap_err().to_err_msg());
}

#[test]
fn succeeds_parsing_with_and_or_combinators() {
    let expected = Ok(ParserSuccess::new(('b', 'c'), Position::new(1, 3, 2)));

    let actual = 
        Combinator::new(p_char('a'))
            .or(p_char('b'))
            .and(p_char('c'))
            .run("bca".to_string());

    assert_eq!(expected, actual);
}

#[test]
fn succeeds_parsing_with_take_prev() {
    let expected = Ok(ParserSuccess::new('a', Position::new(1, 3, 2)));

    let actual = 
        Combinator::new(p_char('a'))
            .take_prev(p_char('b'))
            .run("abc".to_string());

    assert_eq!(expected, actual);
}

#[test]
fn fails_parsing_with_take_prev_at_first_parser() {
    let expected = Err(ParserFailure::new("a".to_string(), Some("b".to_string()), Position::new(1, 1, 0)));
    let err_msg = "expected 'a' but found 'b' at line 1, column 1".to_string();

    let actual = 
        Combinator::new(p_char('a'))
            .take_prev(p_char('b'))
            .run("bac".to_string());

    assert_eq!(expected, actual);
    assert_eq!(err_msg, actual.unwrap_err().to_err_msg());
}

#[test]
fn fails_parsing_with_take_prev_at_second_parser() {
    let expected = Err(ParserFailure::new("b".to_string(), Some("c".to_string()), Position::new(1, 2, 1)));
    let err_msg = "expected 'b' but found 'c' at line 1, column 2".to_string();

    let actual = 
        Combinator::new(p_char('a'))
            .take_prev(p_char('b'))
            .run("acb".to_string());

    assert_eq!(expected, actual);
    assert_eq!(err_msg, actual.unwrap_err().to_err_msg());
}

#[test]
fn succeeds_parsing_with_take_next() {
    let expected = Ok(ParserSuccess::new('b', Position::new(1, 3, 2)));

    let actual = 
        Combinator::new(p_char('a'))
            .take_next(p_char('b'))
            .run("abc".to_string());

    assert_eq!(expected, actual);
}

#[test]
fn fails_parsing_with_take_next_at_first_parser() {
    let expected = Err(ParserFailure::new("a".to_string(), Some("b".to_string()), Position::new(1, 1, 0)));
    let err_msg = "expected 'a' but found 'b' at line 1, column 1".to_string();

    let actual = 
        Combinator::new(p_char('a'))
            .take_next(p_char('b'))
            .run("bac".to_string());

    assert_eq!(expected, actual);
    assert_eq!(err_msg, actual.unwrap_err().to_err_msg());
}

#[test]
fn fails_parsing_with_take_next_at_second_parser() {
    let expected = Err(ParserFailure::new("b".to_string(), Some("c".to_string()), Position::new(1, 2, 1)));
    let err_msg = "expected 'b' but found 'c' at line 1, column 2".to_string();

    let actual = 
        Combinator::new(p_char('a'))
            .take_next(p_char('b'))
            .run("acb".to_string());

    assert_eq!(expected, actual);
    assert_eq!(err_msg, actual.unwrap_err().to_err_msg());
}

#[test]
fn succeeds_parsing_with_combinator_of_four_parsers() {
    let expected = Ok(ParserSuccess::new(("hello".to_string(), "world".to_string()), Position::new(1, 14, 13)));
    
    let p_hello = p_string("hello".to_string());
    let p_comma = p_string(", ".to_string());
    let p_world = p_string("world".to_string());
    
    let actual = 
        Combinator::new(p_char('c'))
            .take_next(p_hello)
            .take_prev(p_comma)
            .and(p_world)
            .run("chello, world".to_string());

    assert_eq!(expected, actual);
}

#[test]
fn succeeds_parsing_with_combinator_of_combinators() {
    let expected = Ok(ParserSuccess::new(("goodbye".to_string(), "world".to_string()), Position::new(1, 16, 15)));
    
    let p_hello = p_string("hello".to_string());
    let p_goodbye = p_string("goodbye".to_string());

    let p_hello_goodbye = 
        Combinator::new(p_hello).or(p_goodbye).get_parser();

    let p_comma = p_char(',');
    let p_world = p_string("world".to_string());

    let p_comma_world =
        Combinator::new(p_comma)
            .take_next(ws())
            .take_next(p_world)
            .get_parser();
    
    let actual =
        Combinator::new(p_char('c'))
            .take_next(p_hello_goodbye)
            .and(p_comma_world)
            .run("cgoodbye, world".to_string());

    assert_eq!(expected, actual);
}

#[test]
fn succeeds_parsing_with_map() {
    let expected = Ok(ParserSuccess::new("hello, world".to_string(), Position::new(1, 6, 5)));
    
    let p_hello = p_string("hello".to_string());
    let to_hello_world = Box::new(|result: String| format!("{}, world", result));

    let actual =
        Combinator::new(p_hello)
            .map(to_hello_world)
            .run("hello, y'all".to_string());

    assert_eq!(expected, actual);
}

#[test]
fn succeeds_parsing_then_return() {
    let expected = Ok(ParserSuccess::new(true, Position::new(1, 5, 4)));

    let p_true = p_string("true".to_string());

    let actual =
        Combinator::new(p_true)
            .then_return(true)
            .run("true".to_string());

    assert_eq!(expected, actual);
}

#[test]
fn fails_parsing_then_return() {
    let expected = Err(ParserFailure::new("true".to_string(), Some("blue".to_string()), Position::new(1, 1, 0)));
    let err_msg = "expected 'true' but found 'blue' at line 1, column 1".to_string();

    let p_true = p_string("true".to_string());

    let actual =
        Combinator::new(p_true)
            .then_return(true)
            .run("blue".to_string());

    assert_eq!(expected, actual);
    assert_eq!(err_msg, actual.unwrap_err().to_err_msg());
}

#[test]
fn succeeds_parsing_between() {
    let expected = Ok(ParserSuccess::new("hello".to_string(), Position::new(1, 8, 7)));

    let actual =
        Combinator::new(p_string("hello".to_string()))
            .between(p_char('{'), p_char('}'))
            .run("{hello}".to_string());

    assert_eq!(expected, actual);
}

#[test]
fn tracks_line_and_column_number_for_error_messaging() {
    let expected = Err(ParserFailure::new("b".to_string(), Some("c".to_string()), Position::new(5, 3, 12)));
    let err_msg = "expected 'b' but found 'c' at line 5, column 3".to_string();

    let p_hello = p_string("hello".to_string());
    let p_ab = p_string("ab".to_string());

    let actual = 
        Combinator::new(p_hello)
            .take_prev(ws())
            .take_prev(p_char('a'))
            .take_prev(ws())
            .take_prev(p_ab)
            .and(p_char('b'))
            .run("hello\n\na\n\nabc".to_string());

    assert_eq!(expected, actual);
    assert_eq!(err_msg, actual.unwrap_err().to_err_msg());
}