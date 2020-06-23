mod common;
use common::*;
use rusty_parsec::*;

#[test]
fn succeeds_parsing_with_tuple_2() {
    let expected = Ok(ParserSuccess::new((123, "hello".to_string()), Position::new(1, 9, 8)));

    let actual =
        tuple_2(
            p_u32(), 
            p_hello()
        ).run("123hello".to_string());

    assert_eq!(expected, actual);
}

#[test] 
fn fails_parsing_with_tuple_2_at_first_parser() {
    let expected = Err(ParserFailure::new_err("integral value".to_string(), None, Position::new(1, 1, 0)));

    let actual =
        tuple_2(
            p_u32(), 
            p_hello()
        ).run("hello123".to_string());

    assert_eq!(expected, actual);
}

#[test] 
fn fails_parsing_with_tuple_2_at_second_parser() {
    let expected = Err(ParserFailure::new_fatal_err("hello".to_string(), Some("world".to_string()), Position::new(1, 4, 3)));

    let actual =
        tuple_2(
            p_u32(), 
            p_hello()
        ).run("123world".to_string());

    assert_eq!(expected, actual);
}

#[test]
fn succeeds_parsing_with_tuple_3() {
    let expected = Ok(ParserSuccess::new(("hello".to_string(), 123 , true), Position::new(1, 13, 12)));

    let actual =
        tuple_3(
            p_hello(), 
            p_u32(), 
            p_true()
        ).run("hello123true".to_string());

    assert_eq!(expected, actual);
}

#[test] 
fn fails_parsing_with_tuple_3_at_first_parser() {
    let expected = Err(ParserFailure::new_err("hello".to_string(), Some("world".to_string()), Position::new(1, 1, 0)));

    let actual =
        tuple_3(
            p_hello(), 
            p_u32(), 
            p_true()
        ).run("world123true".to_string());

    assert_eq!(expected, actual);
}

#[test] 
fn fails_parsing_with_tuple_3_at_second_parser() {
    let expected = Err(ParserFailure::new_fatal_err("integral value".to_string(), None, Position::new(1, 6, 5)));

    let actual =
        tuple_3(
            p_hello(), 
            p_u32(), 
            p_true()
        ).run("helloabctrue".to_string());

    assert_eq!(expected, actual);
}

#[test] 
fn fails_parsing_with_tuple_3_at_third_parser() {
    let expected = Err(ParserFailure::new_fatal_err("true".to_string(), Some("fals".to_string()), Position::new(1, 9, 8)));

    let actual =
        tuple_3(
            p_hello(), 
            p_u32(), 
            p_true()
        ).run("hello123false".to_string());

    assert_eq!(expected, actual);
}

#[test]
fn succeeds_parsing_with_tuple_4() {
    let expected = Ok(ParserSuccess::new(("hello".to_string(), 123 , true, 1.5), Position::new(1, 16, 15)));

    let actual =
        tuple_4(
            p_hello(), 
            p_u32(), 
            p_true(), 
            p_f32()
        ).run("hello123true1.5".to_string());

    assert_eq!(expected, actual);
}

#[test] 
fn fails_parsing_with_tuple_4_at_first_parser() {
    let expected = Err(ParserFailure::new_err("hello".to_string(), Some("world".to_string()), Position::new(1, 1, 0)));

    let actual =
        tuple_4(
            p_hello(), 
            p_u32(), 
            p_true(), 
            p_f32()
        ).run("world123true1.5".to_string());

    assert_eq!(expected, actual);
}

#[test] 
fn fails_parsing_with_tuple_4_at_second_parser() {
    let expected = Err(ParserFailure::new_fatal_err("integral value".to_string(), None, Position::new(1, 6, 5)));

    let actual =
        tuple_4(
            p_hello(), 
            p_u32(), 
            p_true(), 
            p_f32()
        ).run("helloabctrue1.5".to_string());

    assert_eq!(expected, actual);
}

#[test] 
fn fails_parsing_with_tuple_4_at_third_parser() {
    let expected = Err(ParserFailure::new_fatal_err("true".to_string(), Some("fals".to_string()), Position::new(1, 9, 8)));

    let actual =
        tuple_4(
            p_hello(), 
            p_u32(), 
            p_true(), 
            p_f32()
        ).run("hello123false1.5".to_string());

    assert_eq!(expected, actual);
}

#[test] 
fn fails_parsing_with_tuple_4_at_fourth_parser() {
    let expected = Err(ParserFailure::new_fatal_err("floating point value".to_string(), None, Position::new(1, 13, 12)));

    let actual =
        tuple_4(
            p_hello(), 
            p_u32(), 
            p_true(), 
            p_f32()
        ).run("hello123trueabc".to_string());

    assert_eq!(expected, actual);
}

#[test]
fn succeeds_parsing_with_tuple_5() {
    let expected = Ok(ParserSuccess::new(("hello".to_string(), 123 , true, 1.5, 'a'), Position::new(1, 17, 16)));

    let actual =
        tuple_5(
            p_hello(), 
            p_u32(), 
            p_true(), 
            p_f32(), 
            satisfy(Box::new(|c| c == 'a'))
        ).run("hello123true1.5a".to_string());

    assert_eq!(expected, actual);
}

#[test] 
fn fails_parsing_with_tuple_5_at_first_parser() {
    let expected = Err(ParserFailure::new_err("hello".to_string(), Some("world".to_string()), Position::new(1, 1, 0)));

    let actual =
        tuple_5(
            p_hello(), 
            p_u32(), 
            p_true(), 
            p_f32(), 
            satisfy(Box::new(|c| c == 'a'))
        ).run("world123true1.5a".to_string());

    assert_eq!(expected, actual);
}

#[test] 
fn fails_parsing_with_tuple_5_at_second_parser() {
    let expected = Err(ParserFailure::new_fatal_err("integral value".to_string(), None, Position::new(1, 6, 5)));

    let actual =
        tuple_5(
            p_hello(), 
            p_u32(), 
            p_true(), 
            p_f32(), 
            satisfy(Box::new(|c| c == 'a'))
        ).run("helloabctrue1.5a".to_string());

    assert_eq!(expected, actual);
}

#[test] 
fn fails_parsing_with_tuple_5_at_third_parser() {
    let expected = Err(ParserFailure::new_fatal_err("true".to_string(), Some("fals".to_string()), Position::new(1, 9, 8)));

    let actual =
        tuple_5(
            p_hello(), 
            p_u32(), p_true(), 
            p_f32(), 
            satisfy(Box::new(|c| c == 'a'))
        ).run("hello123false1.5a".to_string());

    assert_eq!(expected, actual);
}

#[test] 
fn fails_parsing_with_tuple_5_at_fourth_parser() {
    let expected = Err(ParserFailure::new_fatal_err("floating point value".to_string(), None, Position::new(1, 13, 12)));

    let actual =
        tuple_5(
            p_hello(), 
            p_u32(), 
            p_true(), 
            p_f32(), 
            satisfy(Box::new(|c| c == 'a'))
        ).run("hello123trueabca".to_string());

    assert_eq!(expected, actual);
}

#[test] 
fn fails_parsing_with_tuple_5_at_fifth_parser() {
    let expected = Err(ParserFailure::new_fatal_err("char satisfying the condition".to_string(), None, Position::new(1, 16, 15)));
    
    let actual =
        tuple_5(
            p_hello(), 
            p_u32(), 
            p_true(), 
            p_f32(), 
            satisfy(Box::new(|c| c == 'a'))
        ).run("hello123true1.5c".to_string());

    assert_eq!(expected, actual);
}