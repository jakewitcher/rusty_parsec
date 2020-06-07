use rusty_parsec::*;

#[test]
fn succeeds_parsing_tuple_2() {
    let expected = Ok(ParserSuccess::new((123, "hello".to_string()), Position::new(1, 9, 8)));

    let actual =
        tuple_2(
            p_u32(), 
            p_string("hello".to_string())
        ).run("123hello".to_string());

    assert_eq!(expected, actual);
}

#[test] 
fn fails_parsing_pipe_2_at_first_parser() {
    let expected = Err(ParserFailure::new("integral value".to_string(), None, Position::new(1, 1, 0)));
    let err_msg = "expected 'integral value' but found unknown error at line 1, column 1".to_string();

    let actual =
        tuple_2(
            p_u32(), 
            p_string("hello".to_string())
        ).run("hello123".to_string());

    assert_eq!(expected, actual);
    assert_eq!(err_msg, actual.unwrap_err().to_err_msg());
}

#[test] 
fn fails_parsing_tuple_2_at_second_parser() {
    let expected = Err(ParserFailure::new("hello".to_string(), Some("world".to_string()), Position::new(1, 4, 3)));
    let err_msg = "expected 'hello' but found 'world' at line 1, column 4".to_string();

    let actual =
        tuple_2(
            p_u32(), 
            p_string("hello".to_string())
        ).run("123world".to_string());

    assert_eq!(expected, actual);
    assert_eq!(err_msg, actual.unwrap_err().to_err_msg());
}

#[test]
fn succeeds_parsing_tuple_3() {
    let expected = Ok(ParserSuccess::new(("hello".to_string(), 123 , true), Position::new(1, 13, 12)));

    let actual =
        tuple_3(
            p_string("hello".to_string()), 
            p_u32(), 
            p_true()
        ).run("hello123true".to_string());

    assert_eq!(expected, actual);
}

#[test] 
fn fails_parsing_tuple_3_at_first_parser() {
    let expected = Err(ParserFailure::new("hello".to_string(), Some("world".to_string()), Position::new(1, 1, 0)));
    let err_msg = "expected 'hello' but found 'world' at line 1, column 1".to_string();

    let actual =
        tuple_3(
            p_string("hello".to_string()), 
            p_u32(), 
            p_true()
        ).run("world123true".to_string());

    assert_eq!(expected, actual);
    assert_eq!(err_msg, actual.unwrap_err().to_err_msg());
}

#[test] 
fn fails_parsing_tuple_3_at_second_parser() {
    let expected = Err(ParserFailure::new("integral value".to_string(), None, Position::new(1, 6, 5)));
    let err_msg = "expected 'integral value' but found unknown error at line 1, column 6".to_string();

    let actual =
        tuple_3(
            p_string("hello".to_string()), 
            p_u32(), 
            p_true()
        ).run("helloabctrue".to_string());

    assert_eq!(expected, actual);
    assert_eq!(err_msg, actual.unwrap_err().to_err_msg());
}

#[test] 
fn fails_parsing_tuple_3_at_third_parser() {
    let expected = Err(ParserFailure::new("true".to_string(), Some("fals".to_string()), Position::new(1, 9, 8)));
    let err_msg = "expected 'true' but found 'fals' at line 1, column 9".to_string();

    let actual =
        tuple_3(
            p_string("hello".to_string()), 
            p_u32(), 
            p_true()
        ).run("hello123false".to_string());

    assert_eq!(expected, actual);
    assert_eq!(err_msg, actual.unwrap_err().to_err_msg());
}

#[test]
fn succeeds_parsing_tuple_4() {
    let expected = Ok(ParserSuccess::new(("hello".to_string(), 123 , true, 1.5), Position::new(1, 16, 15)));

    let actual =
        tuple_4(
            p_string("hello".to_string()), 
            p_u32(), 
            p_true(), 
            p_f32()
        ).run("hello123true1.5".to_string());

    assert_eq!(expected, actual);
}

#[test] 
fn fails_parsing_tuple_4_at_first_parser() {
    let expected = Err(ParserFailure::new("hello".to_string(), Some("world".to_string()), Position::new(1, 1, 0)));
    let err_msg = "expected 'hello' but found 'world' at line 1, column 1".to_string();

    let actual =
        tuple_4(
            p_string("hello".to_string()), 
            p_u32(), 
            p_true(), 
            p_f32()
        ).run("world123true1.5".to_string());

    assert_eq!(expected, actual);
    assert_eq!(err_msg, actual.unwrap_err().to_err_msg());
}

#[test] 
fn fails_parsing_tuple_4_at_second_parser() {
    let expected = Err(ParserFailure::new("integral value".to_string(), None, Position::new(1, 6, 5)));
    let err_msg = "expected 'integral value' but found unknown error at line 1, column 6".to_string();

    let actual =
        tuple_4(
            p_string("hello".to_string()), 
            p_u32(), 
            p_true(), 
            p_f32()
        ).run("helloabctrue1.5".to_string());

    assert_eq!(expected, actual);
    assert_eq!(err_msg, actual.unwrap_err().to_err_msg());
}

#[test] 
fn fails_parsing_tuple_4_at_third_parser() {
    let expected = Err(ParserFailure::new("true".to_string(), Some("fals".to_string()), Position::new(1, 9, 8)));
    let err_msg = "expected 'true' but found 'fals' at line 1, column 9".to_string();

    let actual =
        tuple_4(
            p_string("hello".to_string()), 
            p_u32(), 
            p_true(), 
            p_f32()
        ).run("hello123false1.5".to_string());

    assert_eq!(expected, actual);
    assert_eq!(err_msg, actual.unwrap_err().to_err_msg());
}

#[test] 
fn fails_parsing_tuple_4_at_fourth_parser() {
    let expected = Err(ParserFailure::new("floating point value".to_string(), None, Position::new(1, 13, 12)));
    let err_msg = "expected 'floating point value' but found unknown error at line 1, column 13".to_string();

    let actual =
        tuple_4(
            p_string("hello".to_string()), 
            p_u32(), 
            p_true(), 
            p_f32()
        ).run("hello123trueabc".to_string());

    assert_eq!(expected, actual);
    assert_eq!(err_msg, actual.unwrap_err().to_err_msg());
}

#[test]
fn succeeds_parsing_tuple_5() {
    let expected = Ok(ParserSuccess::new(("hello".to_string(), 123 , true, 1.5, 'a'), Position::new(1, 17, 16)));

    let actual =
        tuple_5(
            p_string("hello".to_string()), 
            p_u32(), 
            p_true(), 
            p_f32(), 
            satisfy(Box::new(|c| c == 'a'))
        ).run("hello123true1.5a".to_string());

    assert_eq!(expected, actual);
}

#[test] 
fn fails_parsing_tuple_5_at_first_parser() {
    let expected = Err(ParserFailure::new("hello".to_string(), Some("world".to_string()), Position::new(1, 1, 0)));
    let err_msg = "expected 'hello' but found 'world' at line 1, column 1".to_string();

    let actual =
        tuple_5(
            p_string("hello".to_string()), 
            p_u32(), 
            p_true(), 
            p_f32(), 
            satisfy(Box::new(|c| c == 'a'))
        ).run("world123true1.5a".to_string());

    assert_eq!(expected, actual);
    assert_eq!(err_msg, actual.unwrap_err().to_err_msg());
}

#[test] 
fn fails_parsing_tuple_5_at_second_parser() {
    let expected = Err(ParserFailure::new("integral value".to_string(), None, Position::new(1, 6, 5)));
    let err_msg = "expected 'integral value' but found unknown error at line 1, column 6".to_string();

    let actual =
        tuple_5(
            p_string("hello".to_string()), 
            p_u32(), 
            p_true(), 
            p_f32(), 
            satisfy(Box::new(|c| c == 'a'))
        ).run("helloabctrue1.5a".to_string());

    assert_eq!(expected, actual);
    assert_eq!(err_msg, actual.unwrap_err().to_err_msg());
}

#[test] 
fn fails_parsing_tuple_5_at_third_parser() {
    let expected = Err(ParserFailure::new("true".to_string(), Some("fals".to_string()), Position::new(1, 9, 8)));
    let err_msg = "expected 'true' but found 'fals' at line 1, column 9".to_string();

    let actual =
        tuple_5(
            p_string("hello".to_string()), 
            p_u32(), p_true(), 
            p_f32(), 
            satisfy(Box::new(|c| c == 'a'))
        ).run("hello123false1.5a".to_string());

    assert_eq!(expected, actual);
    assert_eq!(err_msg, actual.unwrap_err().to_err_msg());
}

#[test] 
fn fails_parsing_tuple_5_at_fourth_parser() {
    let expected = Err(ParserFailure::new("floating point value".to_string(), None, Position::new(1, 13, 12)));
    let err_msg = "expected 'floating point value' but found unknown error at line 1, column 13".to_string();

    let actual =
        tuple_5(
            p_string("hello".to_string()), 
            p_u32(), 
            p_true(), 
            p_f32(), 
            satisfy(Box::new(|c| c == 'a'))
        ).run("hello123trueabca".to_string());

    assert_eq!(expected, actual);
    assert_eq!(err_msg, actual.unwrap_err().to_err_msg());
}

#[test] 
fn fails_parsing_tuple_5_at_fifth_parser() {
    let expected = Err(ParserFailure::new("char satisfying the condition".to_string(), None, Position::new(1, 16, 15)));
    let err_msg = "expected 'char satisfying the condition' but found unknown error at line 1, column 16".to_string();
    
    let actual =
        tuple_5(
            p_string("hello".to_string()), 
            p_u32(), 
            p_true(), 
            p_f32(), 
            satisfy(Box::new(|c| c == 'a'))
        ).run("hello123true1.5c".to_string());

    assert_eq!(expected, actual);
    assert_eq!(err_msg, actual.unwrap_err().to_err_msg());
}

fn p_true() -> Parser<bool> {
    Combinator::new(p_string("true".to_string()))
        .then_return(true).get_parser()
}