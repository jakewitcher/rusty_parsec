use rusty_parsec::*;

#[test]
fn succeeds_parsing_with_p_char() {        
    let expected = Ok(ParserSuccess::new('a', Position::new(1, 2, 1)));

    let actual = 
        p_char('a').run("abc".to_string());
    
    assert_eq!(expected, actual);
}

#[test]
fn fails_parsing_with_p_char() {
    let expected = Err(ParserFailure::new_err("b".to_string(), Some("a".to_string()), Position::new(1, 1, 0)));
    
    let actual = 
        p_char('b').run("abc".to_string());

    assert_eq!(expected, actual);
}

#[test]
fn succeeds_parsing_with_p_string() {    
    let expected = Ok(ParserSuccess::new("hello".to_string(), Position::new(1, 6, 5)));

    let actual = 
        p_string("hello".to_string())
            .run("hello, world".to_string());

    assert_eq!(expected, actual);
}

#[test]
fn fails_parsing_with_p_string() {
    let expected = Err(ParserFailure::new_err("hello".to_string(), Some("chell".to_string()), Position::new(1, 1, 0)));
        
    let actual = 
        p_string("hello".to_string())
            .run("chello, world".to_string());

    assert_eq!(expected, actual);
}

#[test]
fn fails_parsing_with_p_string_when_input_is_too_short() {
    let expected = Err(ParserFailure::new_err("hello".to_string(), None, Position::new(1, 1, 0)));
        
    let actual = 
        p_string("hello".to_string())
            .run("hell".to_string());

    assert_eq!(expected, actual);
}

#[test]
fn succeeds_parsing_with_p_char_followed_by_p_string() {
    let expected = Ok(ParserSuccess::new("hello".to_string(), Position::new(1, 7, 6)));

    let actual = 
        p_char('c')
            .take_next(p_string("hello".to_string()))
            .run("chello".to_string());

    assert_eq!(expected, actual);
}

#[test]
fn succeeds_parsing_with_p_string_followed_by_p_string() {
    let expected = Ok(ParserSuccess::new(("hello".to_string(), "world".to_string()), Position::new(1, 11, 10)));
        
    let actual = 
        p_string("hello".to_string())
            .and(p_string("world".to_string()))
            .run("helloworld".to_string());

    assert_eq!(expected, actual);
}

#[test]
fn succeeds_parsing_whitespace_with_ws() {
    let expected = Ok(ParserSuccess::new(('a', 'b'), Position::new(3, 2, 10)));

    let actual = 
        ws().take_next(p_char('a'))
            .take_prev(ws())
            .and(p_char('b'))
            .run("  \na\t  \r\nb".to_string());

    assert_eq!(expected, actual);
}

#[test]
fn fails_parsing_whitespace_with_ws() {
    let expected = Err(ParserFailure::new_fatal_err("b".to_string(), Some("c".to_string()), Position::new(3, 1, 9)));

    let actual = 
        ws().take_next(p_char('a'))
            .take_prev(ws())
            .and(p_char('b'))
            .run("  \na\t  \r\nc".to_string());

    assert_eq!(expected, actual);
}

#[test]
fn succeeds_parsing_with_p_u32() {
    let expected = Ok(ParserSuccess::new(123, Position::new(1, 4, 3)));

    let actual =
        p_u32().run("123abc".to_string());

    assert_eq!(expected, actual);
}

#[test]
fn succeeds_parsing_negative_integer_with_p_i32() {
    let expected = Ok(ParserSuccess::new(-123, Position::new(1, 5, 4)));

    let actual =
        p_i32().run("-123abc".to_string());

    assert_eq!(expected, actual);
}

#[test]
fn fails_parsing_with_p_i32() {
    let expected = Err(ParserFailure::new_err("integral value".to_string(), None, Position::new(1, 1, 0)));

    let actual =
        p_i32().run("abc".to_string());

    assert_eq!(expected, actual);
}

#[test]
fn fails_parsing_with_p_i32_integer_greater_than_i32_max() {
    let expected = Err(ParserFailure::new_err("integral value".to_string(), None, Position::new(1, 1, 0)));

    let actual =
        p_i32().run("2147483900".to_string());

    assert_eq!(expected, actual);
}

#[test]
fn succeeds_parsing_with_p_i64() {
    let expected = Ok(ParserSuccess::new(2147483900, Position::new(1, 11, 10)));

    let actual =
        p_i64().run("2147483900".to_string());

    assert_eq!(expected, actual);
}

#[test]
fn succeeds_parsing_with_p_u32_followed_by_p_string() {
    let expected = Ok(ParserSuccess::new((123, "abc".to_string()), Position::new(1, 7, 6)));

    let actual =
        p_u32().and(p_string("abc".to_string()))
            .run("123abc".to_string());

    assert_eq!(expected, actual);
}

#[test]
fn succeeds_parsing_with_p_string_followed_by_p_i32() {
    let expected = Ok(ParserSuccess::new(-123, Position::new(1, 8, 7)));

    let actual =
        p_string("abc".to_string())
            .take_next(p_i32())
            .run("abc-123".to_string());

    assert_eq!(expected, actual);
}

#[test]
fn succeeds_parsing_with_p_f32() {
    let expected = Ok(ParserSuccess::new(123.35, Position::new(1, 7, 6)));

    let actual =
        p_f32().run("123.35abc".to_string());

    assert_eq!(expected, actual);
}

#[test]
fn succeeds_parsing_with_p_f32_followed_by_period() {
    let expected = Ok(ParserSuccess::new(123.35, Position::new(1, 7, 6)));

    let actual =
        p_f32().run("123.35.abc".to_string());

    assert_eq!(expected, actual);
}

#[test]
fn succeeds_parsing_negative_integer_with_p_f32() {
    let expected = Ok(ParserSuccess::new(-123.35, Position::new(1, 8, 7)));

    let actual =
        p_f32().run("-123.35abc".to_string());

    assert_eq!(expected, actual);
}

#[test]
fn fails_parsing_with_p_f32() {
    let expected = Err(ParserFailure::new_err("floating point value".to_string(), None, Position::new(1, 1, 0)));

    let actual =
        p_f32().run("abc".to_string());

    assert_eq!(expected, actual);
}

#[test]
fn fails_parsing_with_p_f32_integer_greater_than_i32_max() {
    let expected = Err(ParserFailure::new_err("floating point value".to_string(), None, Position::new(1, 1, 0)));

    let actual =
        p_f32().run("340282500000000000000000000000000000000".to_string());

    assert_eq!(expected, actual);
}

#[test]
fn succeeds_parsing_with_p_f64() {
    let expected = Ok(ParserSuccess::new(340282500000000000000000000000000000000.12, Position::new(1, 43, 42)));

    let actual =
        p_f64().run("340282500000000000000000000000000000000.12".to_string());

    assert_eq!(expected, actual);
}

#[test]
fn succeeds_parsing_with_satisfy() {
    let expected = Ok(ParserSuccess::new('c', Position::new(1, 2, 1)));

    let actual =
        satisfy(Box::new(|c:char|c.is_ascii_lowercase()))
            .run("cat".to_string());

    assert_eq!(expected, actual);
}

#[test]
fn fails_parsing_with_satisfy() {
    let expected = Err(ParserFailure::new_err("char satisfying the condition".to_string(), None, Position::new(1, 1, 0)));

    let actual =
        satisfy(Box::new(|c:char|c.is_ascii_lowercase()))
            .run("Cat".to_string());

    assert_eq!(expected, actual);
}

#[test]
fn succeeds_parsing_with_many_satisfy() {
    let expected = Ok(ParserSuccess::new("aaa".to_string(), Position::new(1, 4, 3)));

    let actual =
        many_satisfy(Box::new(|c:char|c == 'a'))
            .run("aaabbb".to_string());

    assert_eq!(expected, actual);
}

#[test]
fn succeeds_parsing_with_many_satisfy_returns_empty_string() {
    let expected = Ok(ParserSuccess::new("".to_string(), Position::new(1, 1, 0)));

    let actual =
        many_satisfy(Box::new(|c:char|c == 'a'))
            .run("bbbaaa".to_string());

    assert_eq!(expected, actual);
}

#[test]
fn succeeds_parsing_words_with_many_satisfy_returns_empty_string() {
    let expected = Ok(ParserSuccess::new("hello".to_string(), Position::new(1, 6, 5)));
    let actual = p_words().run("hello, world".to_string());

    assert_eq!(expected, actual);

    let expected = Ok(ParserSuccess::new("goodbye".to_string(), Position::new(1, 8, 7)));
    let actual = p_words().run("goodbye, world".to_string());
    assert_eq!(expected, actual);
}

fn p_words() -> Parser<String> {
    
        many_satisfy(Box::new(|c:char|c.is_ascii_lowercase()))
}