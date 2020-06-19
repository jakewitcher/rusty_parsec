use rusty_parsec::*;

fn p_true() -> Parser<bool> {
    p_string("true".to_string())
        .then_return(true)
}

fn p_hello() -> Parser<String> {
    p_string("hello".to_string())
}

fn p_abc_123() -> Parser<(String, u32)> {
    tuple_2(p_string("abc".to_string()), p_u32())
}

#[test]
fn succeeds_parsing_with_many() {
    let expected = Ok(ParserSuccess::new(vec![ "hello".to_string(), "hello".to_string(), "hello".to_string()], Position::new(1, 16, 15)));

    let actual = many(p_hello).run("hellohellohello".to_string());

    assert_eq!(expected, actual);
}

#[test]
fn succeeds_parsing_with_many_returns_empty_vec() {
    let expected = Ok(ParserSuccess::new(Vec::new(), Position::new(1, 1, 0)));

    let actual = many(p_hello).run("worldworldworld".to_string());

    assert_eq!(expected, actual);
}

#[test]
fn succeeds_parsing_with_many_with_compound_parser() {
    let expected = Ok(ParserSuccess::new(vec![("abc".to_string(), 123), ("abc".to_string(), 456), ("abc".to_string(), 789)], Position::new(1, 19, 18)));

    let actual = many(p_abc_123).run("abc123abc456abc789".to_string());

    assert_eq!(expected, actual);
}

#[test]
fn succeeds_parsing_with_many_1() {
    let expected = Ok(ParserSuccess::new(vec![ "hello".to_string(), "hello".to_string(), "hello".to_string()], Position::new(1, 16, 15)));

    let actual = many_1(p_hello).run("hellohellohello".to_string());

    assert_eq!(expected, actual);
}

#[test]
fn fails_parsing_with_many_1() {
    let expected = Err(ParserFailure::new_err("value satisfying parser at least once".to_string(), None, Position::new(1, 1, 0)));

    let actual = many_1(p_hello).run("worldworldworld".to_string());

    assert_eq!(expected, actual);
}

#[test]
fn succeeds_parsing_with_skip_many() {
    let expected = Ok(ParserSuccess::new((), Position::new(1, 16, 15)));

    let actual = skip_many(p_hello).run("hellohellohello".to_string());

    assert_eq!(expected, actual);
}

#[test]
fn succeeds_parsing_with_skip_many_1() {
    let expected = Ok(ParserSuccess::new((), Position::new(1, 16, 15)));

    let actual = skip_many_1(p_hello).run("hellohellohello".to_string());

    assert_eq!(expected, actual);
}

#[test]
fn fails_parsing_with_skip_many_1() {
    let expected = Err(ParserFailure::new_err("value satisfying parser at least once".to_string(), None, Position::new(1, 1, 0)));

    let actual = skip_many_1(p_hello).run("abc".to_string());

    assert_eq!(expected, actual);
}

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

#[test]
fn succeeds_parsing_with_many_till() {
    let expected = Ok(ParserSuccess::new(vec![true, true, true], Position::new(1, 16, 15)));

    let actual =
        many_till(p_true, p_u32)
            .run("truetruetrue123".to_string());

    assert_eq!(expected, actual);
}

#[test]
fn fails_parsing_with_many_till() {
    let expected = Err(ParserFailure::new_err("true".to_string(), None, Position::new(1, 13, 12)));

    let actual =
        many_till(p_true, p_u32)
            .run("truetruetrueabc".to_string());

    assert_eq!(expected, actual);
}

#[test]
fn fails_parsing_with_many_till_fatal_err() {
    let expected = Err(ParserFailure::new_fatal_err("integral value".to_string(), None, Position::new(1, 16, 15)));

    let actual =
        many_till(p_true, p_abc_123)
            .run("truetruetrueabcfalse".to_string());

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