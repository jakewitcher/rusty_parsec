use rusty_parsec::*;

#[test]
fn succeeds_parsing_with_and() {
    let expected = Ok(ParserSuccess::new(('a', 'b'), Position::new(1, 3, 2)));

    let actual = 
        p_char('a')
            .and(p_char('b'))
            .run("abc".to_string());

    assert_eq!(expected, actual);
}

#[test]
fn fails_parsing_with_and_at_first_parser() {
    let expected = Err(ParserFailure::new("a".to_string(), Some("b".to_string()), Severity::Error, Position::new(1, 1, 0)));
    let err_msg = "expected 'a' but found 'b' at line 1, column 1".to_string();

    let actual = 
        p_char('a')
            .and(p_char('b'))
            .run("bca".to_string());

    assert_eq!(expected, actual);
    assert_eq!(err_msg, actual.unwrap_err().to_err_msg());
}

#[test]
fn fails_parsing_with_and_at_second_parser() {
    let expected = Err(ParserFailure::new("b".to_string(), Some("c".to_string()), Severity::FatalError, Position::new(1, 2, 1)));
    let err_msg = "expected 'b' but found 'c' at line 1, column 2".to_string();

    let actual = 
        p_char('a')
            .and(p_char('b'))
            .run("acb".to_string());

    assert_eq!(expected, actual);
    assert_eq!(err_msg, actual.unwrap_err().to_err_msg());
}

#[test]
fn succeeds_parsing_with_or_at_first_parser() {
    let expected = Ok(ParserSuccess::new('a', Position::new(1, 2, 1)));

    let actual = 
        p_char('a')
            .or(p_char('b'))
            .run("abc".to_string());

    assert_eq!(expected, actual);
}

#[test]
fn succeeds_parsing_with_or_at_second_parser() {
    let expected = Ok(ParserSuccess::new('b', Position::new(1, 2, 1)));

    let actual = 
        p_char('a')
            .or(p_char('b'))
            .run("bac".to_string());

    assert_eq!(expected, actual);
}

#[test]
fn fails_parsing_with_or_at_second_parser() {
    let expected = Err(ParserFailure::new("b".to_string(), Some("c".to_string()), Severity::Error, Position::new(1, 1, 0)));
    let err_msg = "expected 'b' but found 'c' at line 1, column 1".to_string();

    let actual = 
        p_char('a')
            .or(p_char('b'))
            .run("cba".to_string());

    assert_eq!(expected, actual);
    assert_eq!(err_msg, actual.unwrap_err().to_err_msg());
}

#[test]
fn fails_parsing_with_or_at_second_parser_fatal_err() {
    let expected = Err(ParserFailure::new("b".to_string(), Some("c".to_string()), Severity::FatalError, Position::new(1, 2, 1)));
    let err_msg = "expected 'b' but found 'c' at line 1, column 2".to_string();

    let actual = 
        p_char('a').and(p_char('b'))
            .or(p_char('c').and(p_char('d')))
            .run("aca".to_string());

    assert_eq!(expected, actual);
    assert_eq!(err_msg, actual.unwrap_err().to_err_msg());
}

#[test]
fn succeeds_parsing_with_take_prev() {
    let expected = Ok(ParserSuccess::new('a', Position::new(1, 3, 2)));

    let actual = 
        p_char('a')
            .take_prev(p_char('b'))
            .run("abc".to_string());

    assert_eq!(expected, actual);
}

#[test]
fn fails_parsing_with_take_prev_at_first_parser() {
    let expected = Err(ParserFailure::new("a".to_string(), Some("b".to_string()), Severity::Error, Position::new(1, 1, 0)));
    let err_msg = "expected 'a' but found 'b' at line 1, column 1".to_string();

    let actual = 
        p_char('a')
            .take_prev(p_char('b'))
            .run("bac".to_string());

    assert_eq!(expected, actual);
    assert_eq!(err_msg, actual.unwrap_err().to_err_msg());
}

#[test]
fn fails_parsing_with_take_prev_at_second_parser() {
    let expected = Err(ParserFailure::new("b".to_string(), Some("c".to_string()), Severity::FatalError, Position::new(1, 2, 1)));
    let err_msg = "expected 'b' but found 'c' at line 1, column 2".to_string();

    let actual = 
        p_char('a')
            .take_prev(p_char('b'))
            .run("acb".to_string());

    assert_eq!(expected, actual);
    assert_eq!(err_msg, actual.unwrap_err().to_err_msg());
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
    let expected = Err(ParserFailure::new("a".to_string(), Some("b".to_string()), Severity::Error, Position::new(1, 1, 0)));
    let err_msg = "expected 'a' but found 'b' at line 1, column 1".to_string();

    let actual = 
        p_char('a')
            .take_next(p_char('b'))
            .run("bac".to_string());

    assert_eq!(expected, actual);
    assert_eq!(err_msg, actual.unwrap_err().to_err_msg());
}

#[test]
fn fails_parsing_with_take_next_at_second_parser() {
    let expected = Err(ParserFailure::new("b".to_string(), Some("c".to_string()), Severity::Error, Position::new(1, 2, 1)));
    let err_msg = "expected 'b' but found 'c' at line 1, column 2".to_string();

    let actual = 
        p_char('a')
            .take_next(p_char('b'))
            .run("acb".to_string());

    assert_eq!(expected, actual);
    assert_eq!(err_msg, actual.unwrap_err().to_err_msg());
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
    let expected = Err(ParserFailure::new("true".to_string(), Some("blue".to_string()), Severity::Error, Position::new(1, 1, 0)));
    let err_msg = "expected 'true' but found 'blue' at line 1, column 1".to_string();

    let actual =
        p_string("true".to_string())
            .then_return(true)
            .run("blue".to_string());

    assert_eq!(expected, actual);
    assert_eq!(err_msg, actual.unwrap_err().to_err_msg());
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
    let expected = Err(ParserFailure::new("{".to_string(), Some("[".to_string()), Severity::Error, Position::new(1, 1, 0)));
    let err_msg = "expected '{' but found '[' at line 1, column 1".to_string();

    let actual =
        p_string("hello".to_string())
            .between(p_char('{'), p_char('}'))
            .run("[hello}".to_string());

    assert_eq!(expected, actual);
    assert_eq!(err_msg, actual.unwrap_err().to_err_msg());
}

#[test] 
fn fails_parsing_with_between_at_middle() {
    let expected = Err(ParserFailure::new("hello".to_string(), Some("yello".to_string()), Severity::Error, Position::new(1, 2, 1)));
    let err_msg = "expected 'hello' but found 'yello' at line 1, column 2".to_string();

    let actual =
        p_string("hello".to_string())
            .between(p_char('{'), p_char('}'))
            .run("{yello}".to_string());

    assert_eq!(expected, actual);
    assert_eq!(err_msg, actual.unwrap_err().to_err_msg());
}

#[test] 
fn fails_parsing_with_between_at_close() {
    let expected = Err(ParserFailure::new("}".to_string(), Some("]".to_string()), Severity::Error, Position::new(1, 7, 6)));
    let err_msg = "expected '}' but found ']' at line 1, column 7".to_string();

    let actual =
        p_string("hello".to_string())
        .between(p_char('{'), p_char('}'))
        .run("{hello]".to_string());

    assert_eq!(expected, actual);
    assert_eq!(err_msg, actual.unwrap_err().to_err_msg());
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
    let expected = Err(ParserFailure::new("following parser to succeed".to_string(), None, Severity::Error, Position::new(1, 7, 6)));
    let err_msg = "expected 'following parser to succeed' but found unknown error at line 1, column 7".to_string();

    let p_helloworld =
        p_string("hello".to_string())
            .and(p_string("world".to_string()));

    let actual =
        p_u32().and(p_string("abc".to_string()))
            .followed_by(p_helloworld)
            .run("123abchellonerds".to_string());

    assert_eq!(expected, actual);
    assert_eq!(err_msg, actual.unwrap_err().to_err_msg());
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
    let expected = Err(ParserFailure::new("following parser to fail".to_string(), None, Severity::Error, Position::new(1, 7, 6)));
    let err_msg = "expected 'following parser to fail' but found unknown error at line 1, column 7".to_string();

    let p_helloworld =
        p_string("hello".to_string())
            .and(p_string("world".to_string()));

    let actual =
        p_u32().and(p_string("abc".to_string()))
            .not_followed_by(p_helloworld)
            .run("123abchelloworld".to_string());

    assert_eq!(expected, actual);
    assert_eq!(err_msg, actual.unwrap_err().to_err_msg());
}