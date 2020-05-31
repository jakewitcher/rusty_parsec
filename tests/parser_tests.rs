use rusty_parsec::*;

#[test]
fn succeeds_parsing_with_p_char() {        
    let expected = Ok('a');

    let actual = 
        Combinator::new(p_char('a')).run(String::from("abc"));
    
    assert_eq!(expected, actual);
}

#[test]
fn fails_parsing_with_p_char() {
    let expected = String::from("expected 'b' but found 'a' at line 1, column 1");        
    
    let actual = 
        Combinator::new(p_char('b')).run(String::from("abc"));

    assert_eq!(expected, actual.unwrap_err().to_err_msg());
}

#[test]
fn succeeds_parsing_with_p_string() {    
    let expected = Ok(String::from("hello"));

    let p_hello = p_string(String::from("hello"));

    let actual = 
        Combinator::new(p_hello)
            .run(String::from("hello, world"));

    assert_eq!(expected, actual);
}

#[test]
fn fails_parsing_with_p_string() {
    let expected = String::from("expected 'hello' but found 'chell' at line 1, column 1");
    
    let p_hello = p_string(String::from("hello"));
    
    let actual = 
        Combinator::new(p_hello)
            .run(String::from("chello, world"));

    assert_eq!(expected, actual.unwrap_err().to_err_msg());
}

#[test]
fn fails_parsing_with_p_string_when_input_is_too_short() {
    let expected = String::from("expected 'hello' but found unknown error at line 1, column 1");
    
    let p_hello = p_string(String::from("hello"));
    
    let actual = 
        Combinator::new(p_hello)
            .run(String::from("hell"));

    assert_eq!(expected, actual.unwrap_err().to_err_msg());
}

#[test]
fn succeeds_parsing_with_p_char_followed_by_p_string() {
    let expected = Ok(String::from("hello"));
    
    let p_hello = p_string(String::from("hello"));
    
    let actual = 
        Combinator::new(p_char('c'))
            .take_next(p_hello)
            .run(String::from("chello"));

    assert_eq!(expected, actual);
}

#[test]
fn succeeds_parsing_with_p_string_followed_by_p_string() {
    let expected = Ok((String::from("hello"), String::from("world")));
    
    let p_hello = p_string(String::from("hello"));
    let p_world = p_string(String::from("world"));
    
    let actual = 
        Combinator::new(p_hello)
            .and(p_world)
            .run(String::from("helloworld"));

    assert_eq!(expected, actual);
}

#[test]
fn succeeds_parsing_whitespace_with_ws() {
    let expected = Ok(('a', 'b'));

    let actual = 
        Combinator::new(ws())
            .take_next(p_char('a'))
            .take_prev(ws())
            .and(p_char('b'))
            .run(String::from("  \na\t  \r\nb"));

    assert_eq!(expected, actual);
}

#[test]
fn fails_parsing_whitespace_with_ws() {
    let expected = String::from("expected 'b' but found 'c' at line 3, column 1");

    let actual = 
        Combinator::new(ws())
            .take_next(p_char('a'))
            .take_prev(ws())
            .and(p_char('b'))
            .run(String::from("  \na\t  \r\nc"));

    assert_eq!(expected, actual.unwrap_err().to_err_msg());
}

#[test]
fn succeeds_parsing_with_p_u32() {
    let expected = Ok(123);

    let actual =
        Combinator::new(p_u32())
            .run(String::from("123abc"));

    assert_eq!(expected, actual);
}

#[test]
fn succeeds_parsing_negative_integer_with_p_i32() {
    let expected = Ok(-123);

    let actual =
        Combinator::new(p_i32())
            .run(String::from("-123abc"));

    assert_eq!(expected, actual);
}

#[test]
fn fails_parsing_with_p_i32() {
    let expected = String::from("expected 'integral value' but found unknown error at line 1, column 1");

    let actual =
        Combinator::new(p_i32())
        .run(String::from("abc"));

    assert_eq!(expected, actual.unwrap_err().to_err_msg());
}

#[test]
fn fails_parsing_with_p_i32_integer_greater_than_i32_max() {
    let expected = String::from("expected 'integral value' but found unknown error at line 1, column 1");

    let actual =
        Combinator::new(p_i32())
        .run(String::from("2147483900"));

    assert_eq!(expected, actual.unwrap_err().to_err_msg());
}

#[test]
fn succeeds_parsing_with_p_i64() {
    let expected = Ok(2147483900);

    let actual =
        Combinator::new(p_i64())
        .run(String::from("2147483900"));

    assert_eq!(expected, actual);
}

#[test]
fn succeeds_parsing_with_p_u32_followed_by_p_string() {
    let expected = Ok((123, String::from("abc")));

    let actual =
        Combinator::new(p_u32())
            .and(p_string(String::from("abc")))
            .run(String::from("123abc"));

    assert_eq!(expected, actual);
}

#[test]
fn succeeds_parsing_with_p_string_followed_by_p_i32() {
    let expected = Ok(-123);

    let actual =
        Combinator::new(p_string(String::from("abc")))
            .take_next(p_i32())
            .run(String::from("abc-123"));

    assert_eq!(expected, actual);
}

#[test]
fn succeeds_parsing_with_p_f32() {
    let expected = Ok(123.35);

    let actual =
        Combinator::new(p_f32())
            .run(String::from("123.35abc"));

    assert_eq!(expected, actual);
}

#[test]
fn succeeds_parsing_with_p_f32_followed_by_period() {
    let expected = Ok(123.35);

    let actual =
        Combinator::new(p_f32())
            .run(String::from("123.35.abc"));

    assert_eq!(expected, actual);
}

#[test]
fn succeeds_parsing_negative_integer_with_p_f32() {
    let expected = Ok(-123.35);

    let actual =
        Combinator::new(p_f32())
            .run(String::from("-123.35abc"));

    assert_eq!(expected, actual);
}

#[test]
fn fails_parsing_with_p_f32() {
    let expected = String::from("expected 'floating point value' but found unknown error at line 1, column 1");

    let actual =
        Combinator::new(p_f32())
        .run(String::from("abc"));

    assert_eq!(expected, actual.unwrap_err().to_err_msg());
}

#[test]
fn fails_parsing_with_p_f32_integer_greater_than_i32_max() {
    let expected = String::from("expected 'floating point value' but found unknown error at line 1, column 1");

    let actual =
        Combinator::new(p_f32())
        .run(String::from("340282500000000000000000000000000000000"));

    assert_eq!(expected, actual.unwrap_err().to_err_msg());
}

#[test]
fn succeeds_parsing_with_p_f64() {
    let expected = Ok(340282500000000000000000000000000000000.12);

    let actual =
        Combinator::new(p_f64())
        .run(String::from("340282500000000000000000000000000000000.12"));

    assert_eq!(expected, actual);
}