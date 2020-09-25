use rusty_parsec::*;

#[test]
fn and_run_simple_parsers_success() {
    let expected = Ok(ParserSuccess::new(
        ('a', 'b'), 
        Position::new(1, 3, 2)
    ));

    let actual = p_char('a')
        .and(p_char('b'))
        .run(String::from("ab"));

    assert_eq!(expected, actual);
}

#[test]
fn and_run_simple_parsers_fails_with_error_at_first_parser() {
    let expected = Err(ParserFailure::new_err(
        String::from("a"), 
        Some(String::from("b")), 
        Position::new(1, 1, 0)
    ));

    let actual = p_char('a')
        .and(p_char('b'))
        .run(String::from("bc"));

    assert_eq!(expected, actual);
}

#[test]
fn and_run_simple_parsers_fails_with_fatal_error_at_second_parser() {
    let expected = Err(ParserFailure::new_fatal_err(
        String::from("b"), 
        Some(String::from("c")), 
        Position::new(1, 2, 1)
    ));

    let actual = p_char('a')
        .and(p_char('b'))
        .run(String::from("ac"));

    assert_eq!(expected, actual);
}

#[test]
fn and_run_complex_parsers_success() {
    let expected = Ok(ParserSuccess::new(
        (('a', 'b'), ('c', 'd')), 
        Position::new(1, 5, 4)
    ));

    let first = p_char('a').and(p_char('b'));
    let second = p_char('c').and(p_char('d'));

    let actual = first
        .and(second)
        .run(String::from("abcd"));

    assert_eq!(expected, actual);
}

#[test]
fn and_run_complex_parsers_fails_with_fatal_error_at_first_parser() {
    let expected = Err(ParserFailure::new_fatal_err(
        String::from("b"), 
        Some(String::from("c")), 
        Position::new(1, 2, 1)
    ));

    let first = p_char('a').and(p_char('b'));
    let second = p_char('c').and(p_char('d'));

    let actual = first
        .and(second)
        .run(String::from("acbd"));

    assert_eq!(expected, actual);
}

#[test]
fn and_run_complex_parsers_fails_with_fatal_error_at_second_parser() {
    let expected = Err(ParserFailure::new_fatal_err(
        String::from("d"), 
        Some(String::from("e")), 
        Position::new(1, 4, 3)
    ));

    let first = p_char('a').and(p_char('b'));
    let second = p_char('c').and(p_char('d'));

    let actual = first
        .and(second)
        .run(String::from("abce"));

    assert_eq!(expected, actual);
}

#[test]
fn and_try_run_simple_parsers_success() {
    let expected = Ok(ParserSuccess::new(
        ('a', 'b'), 
        Position::new(1, 3, 2)
    ));

    let actual = p_char('a')
        .and_try(p_char('b'))
        .run("ab".to_string());

    assert_eq!(expected, actual);
}

#[test]
fn and_try_run_simple_parsers_fails_with_error_at_second_parser() {
    let expected = Err(ParserFailure::new_err(
        String::from("b"), 
        Some(String::from("c")), 
        Position::new(1, 2, 1)
    ));

    let actual = p_char('a')
        .and_try(p_char('b'))
        .run(String::from("ac"));

    assert_eq!(expected, actual);
}

#[test]
fn and_try_run_simple_parsers_fails_with_fatal_error_at_second_parser() {
    let expected = Err(ParserFailure::new_fatal_err(
        String::from("d"), 
        Some(String::from("e")), 
        Position::new(1, 4, 3)
    ));

    let first = p_char('a').and(p_char('b'));
    let second = p_char('c').and(p_char('d'));

    let actual = first
        .and_try(second)
        .run(String::from("abce"));

    assert_eq!(expected, actual);
}

#[test]
fn or_run_simple_parsers_success_at_first_parser() {
    let expected = Ok(ParserSuccess::new(
        'a', 
        Position::new(1, 2, 1)
    ));

    let actual = p_char('a')
        .or(p_char('b'))
        .run(String::from("abc"));

    assert_eq!(expected, actual);
}

#[test]
fn or_run_simple_parsers_success_at_second_parser() {
    let expected = Ok(ParserSuccess::new(
        'b', 
        Position::new(1, 2, 1)
    ));

    let actual = p_char('a')
        .or(p_char('b'))
        .run(String::from("bac"));

    assert_eq!(expected, actual);
}

#[test]
fn or_run_simple_parsers_fails_with_error_at_second_parser() {
    let expected = Err(ParserFailure::new_err(
        String::from("b"), 
        Some(String::from("c")), 
        Position::new(1, 1, 0)
    ));

    let actual = p_char('a')
        .or(p_char('b'))
        .run(String::from("cba"));

    assert_eq!(expected, actual);
}

#[test]
fn or_run_complex_parsers_fails_with_fatal_error_at_second_parser() {
    let expected = Err(ParserFailure::new_fatal_err(
        String::from("d"), 
        Some(String::from("e")), 
        Position::new(1, 2, 1)
    ));

    let first = p_char('a').and(p_char('b'));
    let second = p_char('c').and(p_char('d'));

    let actual = first
        .or(second)
        .run(String::from("ce"));

    assert_eq!(expected, actual);
}

#[test]
fn take_prev_run_simple_parsers_success() {
    let expected = Ok(ParserSuccess::new(
        'a', 
        Position::new(1, 3, 2)
    ));

    let actual = p_char('a')
        .take_prev(p_char('b'))
        .run(String::from("ab"));

    assert_eq!(expected, actual);
}

#[test]
fn take_prev_run_simple_parsers_fails_with_error_at_first_parser() {
    let expected = Err(ParserFailure::new_err(
        String::from("a"), 
        Some(String::from("b")), 
        Position::new(1, 1, 0)
    ));

    let actual = p_char('a')
        .take_prev(p_char('b'))
        .run(String::from("ba"));

    assert_eq!(expected, actual);
}

#[test]
fn take_prev_run_simple_parsers_fails_with_fatal_error_at_second_parser() {
    let expected = Err(ParserFailure::new_fatal_err(
        String::from("b"), 
        Some(String::from("c")), 
        Position::new(1, 2, 1)));

    let actual = p_char('a')
        .take_prev(p_char('b'))
        .run(String::from("ac"));

    assert_eq!(expected, actual);
}

#[test]
fn succeeds_parsing_with_try_take_prev() {
    let expected = Ok(ParserSuccess::new('a', Position::new(1, 3, 2)));

    let actual = 
        p_char('a')
            .try_take_prev(p_char('b'))
            .run("abc".to_string());

    assert_eq!(expected, actual);
}

#[test]
fn fails_parsing_with_try_take_prev() {
    let expected = Err(ParserFailure::new_err("b".to_string(), Some("c".to_string()), Position::new(1, 2, 1)));

    let actual = 
        p_char('a')
            .try_take_prev(p_char('b'))
            .run("acb".to_string());

    assert_eq!(expected, actual);
}

#[test]
fn fails_parsing_with_try_take_prev_fatal_err() {
    let expected = Err(ParserFailure::new_fatal_err("c".to_string(), Some("d".to_string()), Position::new(1, 3, 2)));

    let actual = 
        p_char('a')
            .try_take_prev(p_char('b').and(p_char('c')))
            .run("abd".to_string());

    assert_eq!(expected, actual);
}

#[test]
fn succeeds_parsing_with_take_next() {
    let expected = Ok(ParserSuccess::new('b', Position::new(1, 3, 2)));

    let actual = 
        p_char('a')
            .take_next(p_char('b'))
            .run("abc".to_string());

    assert_eq!(expected, actual);
}

#[test]
fn fails_parsing_with_take_next_at_first_parser() {
    let expected = Err(ParserFailure::new_err("a".to_string(), Some("b".to_string()), Position::new(1, 1, 0)));

    let actual = 
        p_char('a')
            .take_next(p_char('b'))
            .run("bac".to_string());

    assert_eq!(expected, actual);
}

#[test]
fn fails_parsing_with_take_next_at_second_parser() {
    let expected = Err(ParserFailure::new_fatal_err("b".to_string(), Some("c".to_string()), Position::new(1, 2, 1)));

    let actual = 
        p_char('a')
            .take_next(p_char('b'))
            .run("acb".to_string());

    assert_eq!(expected, actual);
}

#[test]
fn succeeds_parsing_with_try_take_next() {
    let expected = Ok(ParserSuccess::new('b', Position::new(1, 3, 2)));

    let actual = 
        p_char('a')
            .try_take_next(p_char('b'))
            .run("abc".to_string());

    assert_eq!(expected, actual);
}

#[test]
fn fails_parsing_with_try_take_next() {
    let expected = Err(ParserFailure::new_err("b".to_string(), Some("c".to_string()), Position::new(1, 2, 1)));

    let actual = 
        p_char('a')
            .try_take_next(p_char('b'))
            .run("acb".to_string());

    assert_eq!(expected, actual);
}

#[test]
fn fails_parsing_with_try_take_next_fatal_err() {
    let expected = Err(ParserFailure::new_fatal_err("c".to_string(), Some("d".to_string()), Position::new(1, 3, 2)));

    let actual = 
        p_char('a')
            .try_take_next(p_char('b').and(p_char('c')))
            .run("abd".to_string());

    assert_eq!(expected, actual);
}

#[test]
fn succeeds_parsing_with_map() {
    let expected = Ok(ParserSuccess::new("hello, world".to_string(), Position::new(1, 6, 5)));
    let to_hello_world = 
        Box::new(|result: String| format!("{}, world", result));

    let actual =
        p_string("hello".to_string())
            .map(to_hello_world)
            .run("hello, y'all".to_string());

    assert_eq!(expected, actual);
}

#[test]
fn succeeds_parsing_with_then_return() {
    let expected = Ok(ParserSuccess::new(true, Position::new(1, 5, 4)));

    let actual =
        p_string("true".to_string())
            .then_return(true)
            .run("true".to_string());

    assert_eq!(expected, actual);
}

#[test]
fn fails_parsing_with_then_return() {
    let expected = Err(ParserFailure::new_err("true".to_string(), Some("blue".to_string()), Position::new(1, 1, 0)));

    let actual =
        p_string("true".to_string())
            .then_return(true)
            .run("blue".to_string());

    assert_eq!(expected, actual);
}

#[test]
fn succeeds_parsing_with_or_return() {
    let expected = Ok(ParserSuccess::new("true".to_string(), Position::new(1, 5, 4)));

    let actual =
        p_string("true".to_string())
            .or_return("false".to_string())
            .run("true".to_string());

    assert_eq!(expected, actual);
}

#[test]
fn succeeds_parsing_with_or_return_using_default_return() {
    let expected = Ok(ParserSuccess::new("false".to_string(), Position::new(1, 1, 0)));

    let actual =
        p_string("true".to_string())
            .or_return("false".to_string())
            .run("hello, world".to_string());

    assert_eq!(expected, actual);
}

#[test]
fn succeeds_parsing_with_bind() {
    let expected = Ok(ParserSuccess::new("hello".to_string(), Position::new(1, 7, 6)));

    let actual =
        p_char('a')
            .then_return("hello".to_string())
            .bind(Box::new(|hello| p_string(hello)))
            .run("ahello".to_string());

    assert_eq!(expected, actual);
}

#[test]
fn fails_parsing_with_bind() {
    let expected = Err(ParserFailure::new_fatal_err("hello".to_string(), Some("world".to_string()), Position::new(1, 2, 1)));

    let actual =
        p_char('a')
            .then_return("hello".to_string())
            .bind(Box::new(|hello| p_string(hello)))
            .run("aworld".to_string());

    assert_eq!(expected, actual);
}

#[test]
fn succeeds_parsing_with_try_bind() {
    let expected = Ok(ParserSuccess::new("hello".to_string(), Position::new(1, 7, 6)));

    let actual =
        p_char('a')
            .then_return("hello".to_string())
            .try_bind(Box::new(|hello| p_string(hello)))
            .run("ahello".to_string());

    assert_eq!(expected, actual);
}

#[test]
fn fails_parsing_with_try_bind() {
    let expected = Err(ParserFailure::new_err("hello".to_string(), Some("world".to_string()), Position::new(1, 2, 1)));

    let actual =
        p_char('a')
            .then_return("hello".to_string())
            .try_bind(Box::new(|hello| p_string(hello)))
            .run("aworld".to_string());

    assert_eq!(expected, actual);
}

#[test]
fn fails_parsing_with_try_bind_fatal_err() {
    let expected = Err(ParserFailure::new_fatal_err("world".to_string(), Some("nerds".to_string()), Position::new(1, 7, 6)));

    let actual =
        p_char('a')
            .then_return("hello".to_string())
            .try_bind(Box::new(
                |hello| 
                    p_string(hello)
                        .and(p_string("world".to_string()))
            ))
            .run("ahellonerds".to_string());

    assert_eq!(expected, actual);
}

#[test]
fn succeeds_parsing_with_between() {
    let expected = Ok(ParserSuccess::new("hello".to_string(), Position::new(1, 8, 7)));

    let actual =
        p_string("hello".to_string())
            .between(p_char('{'), p_char('}'))
            .run("{hello}".to_string());

    assert_eq!(expected, actual);
}

#[test] 
fn fails_parsing_with_between_at_open() {
    let expected = Err(ParserFailure::new_err("{".to_string(), Some("[".to_string()), Position::new(1, 1, 0)));

    let actual =
        p_string("hello".to_string())
            .between(p_char('{'), p_char('}'))
            .run("[hello}".to_string());

    assert_eq!(expected, actual);
}

#[test] 
fn fails_parsing_with_between_at_middle() {
    let expected = Err(ParserFailure::new_fatal_err("hello".to_string(), Some("yello".to_string()), Position::new(1, 2, 1)));

    let actual =
        p_string("hello".to_string())
            .between(p_char('{'), p_char('}'))
            .run("{yello}".to_string());

    assert_eq!(expected, actual);
}

#[test] 
fn fails_parsing_with_between_at_close() {
    let expected = Err(ParserFailure::new_fatal_err("}".to_string(), Some("]".to_string()), Position::new(1, 7, 6)));

    let actual =
        p_string("hello".to_string())
        .between(p_char('{'), p_char('}'))
        .run("{hello]".to_string());

    assert_eq!(expected, actual);
}

#[test]
fn succeeds_parsing_with_opt_returns_some() {
    let expected = Ok(ParserSuccess::new(Some(123), Position::new(1, 4, 3)));

    let actual = 
        p_u32().opt()
            .run("123".to_string());

    assert_eq!(expected, actual);
}

#[test]
fn succeeds_parsing_with_opt_returns_none() {
    let expected = Ok(ParserSuccess::new(None, Position::new(1, 1, 0)));

    let actual = 
        p_u32().opt()
            .run("abc".to_string());

    assert_eq!(expected, actual);
}

#[test]
fn succeeds_parsing_with_optional() {
    let expected = Ok(ParserSuccess::new((), Position::new(1, 4, 3)));

    let actual = 
        p_u32().optional()
            .run("123".to_string());

    assert_eq!(expected, actual);
}

#[test]
fn fails_parsing_with_optional_returns_unit() {
    let expected = Ok(ParserSuccess::new((), Position::new(1, 1, 0)));

    let actual = 
        p_u32().optional()
            .run("abc".to_string());

    assert_eq!(expected, actual);
}

#[test]
fn succeeds_parsing_with_followed_by() {
    let expected = Ok(ParserSuccess::new((123, "abc".to_string()), Position::new(1, 7, 6)));

    let p_helloworld =
        p_string("hello".to_string())
            .and(p_string("world".to_string()));

    let actual =
        p_u32().and(p_string("abc".to_string()))
            .followed_by(p_helloworld)
            .run("123abchelloworld".to_string());

    assert_eq!(expected, actual);
}

#[test]
fn fails_parsing_with_followed_by() {
    let expected = Err(ParserFailure::new_fatal_err("following parser to succeed".to_string(), None, Position::new(1, 7, 6)));

    let p_helloworld =
        p_string("hello".to_string())
            .and(p_string("world".to_string()));

    let actual =
        p_u32().and(p_string("abc".to_string()))
            .followed_by(p_helloworld)
            .run("123abchellonerds".to_string());

    assert_eq!(expected, actual);
}

#[test]
fn succeeds_parsing_with_not_followed_by() {
    let expected = Ok(ParserSuccess::new((123, "abc".to_string()), Position::new(1, 7, 6)));

    let p_helloworld =
        p_string("hello".to_string())
            .and(p_string("world".to_string()));

    let actual =
        p_u32().and(p_string("abc".to_string()))
            .not_followed_by(p_helloworld)
            .run("123abchellonerds".to_string());

    assert_eq!(expected, actual);
}

#[test]
fn fails_parsing_with_not_followed_by() {
    let expected = Err(ParserFailure::new_fatal_err("following parser to fail".to_string(), None, Position::new(1, 7, 6)));

    let p_helloworld =
        p_string("hello".to_string())
            .and(p_string("world".to_string()));

    let actual =
        p_u32().and(p_string("abc".to_string()))
            .not_followed_by(p_helloworld)
            .run("123abchelloworld".to_string());

    assert_eq!(expected, actual);
}