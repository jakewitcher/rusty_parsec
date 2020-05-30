use rusty_parsec::*;

#[test]
fn succeeds_parsing_with_and() {
    let expected = Ok(('a', 'b'));

    let actual = 
        Combinator::new(p_char('a'))
            .and(p_char('b'))
            .run(String::from("abc"));

    assert_eq!(expected, actual);
}

#[test]
fn fails_parsing_with_and_at_first_parser() {
    let expected = String::from("expected 'a' but found 'b' at line 1, column 1");

    let actual = 
        Combinator::new(p_char('a'))
            .and(p_char('b'))
            .run(String::from("bca"));

    assert_eq!(expected, actual.unwrap_err().to_err_msg());
}

#[test]
fn fails_parsing_with_and_at_second_parser() {
    let expected = String::from("expected 'b' but found 'c' at line 1, column 2");

    let actual = 
        Combinator::new(p_char('a'))
            .and(p_char('b'))
            .run(String::from("acb"));

    assert_eq!(expected, actual.unwrap_err().to_err_msg());
}

#[test]
fn succeeds_parsing_with_or_at_first_parser() {
    let expected = Ok('a');

    let actual = 
        Combinator::new(p_char('a'))
            .or(p_char('b'))
            .run(String::from("abc"));

    assert_eq!(expected, actual);
}

#[test]
fn succeeds_parsing_with_or_at_second_parser() {
    let expected = Ok('b');

    let actual = 
        Combinator::new(p_char('a'))
            .or(p_char('b'))
            .run(String::from("bac"));

    assert_eq!(expected, actual);
}

#[test]
fn fails_parsing_with_or_at_second_parser() {
    let expected = String::from("expected 'b' but found 'c' at line 1, column 1");

    let actual = 
        Combinator::new(p_char('a'))
            .or(p_char('b'))
            .run(String::from("cba"));

    assert_eq!(expected, actual.unwrap_err().to_err_msg());
}

#[test]
fn succeeds_parsing_with_and_or_combinators() {
    let expected = Ok(('b', 'c'));

    let actual = 
        Combinator::new(p_char('a'))
            .or(p_char('b'))
            .and(p_char('c'))
            .run(String::from("bca"));

    assert_eq!(expected, actual);
}

#[test]
fn succeeds_parsing_with_take_prev() {
    let expected = Ok('a');

    let actual = 
        Combinator::new(p_char('a'))
            .take_prev(p_char('b'))
            .run(String::from("abc"));

    assert_eq!(expected, actual);
}

#[test]
fn fails_parsing_with_take_prev_at_first_parser() {
    let expected = String::from("expected 'a' but found 'b' at line 1, column 1");

    let actual = 
        Combinator::new(p_char('a'))
            .take_prev(p_char('b'))
            .run(String::from("bac"));

    assert_eq!(expected, actual.unwrap_err().to_err_msg());
}

#[test]
fn fails_parsing_with_take_prev_at_second_parser() {
    let expected = String::from("expected 'b' but found 'c' at line 1, column 2");

    let actual = 
        Combinator::new(p_char('a'))
            .take_prev(p_char('b'))
            .run(String::from("acb"));

    assert_eq!(expected, actual.unwrap_err().to_err_msg());
}

#[test]
fn succeeds_parsing_with_take_next() {
    let expected = Ok('b');

    let actual = 
        Combinator::new(p_char('a'))
            .take_next(p_char('b'))
            .run(String::from("abc"));

    assert_eq!(expected, actual);
}

#[test]
fn fails_parsing_with_take_next_at_first_parser() {
    let expected = String::from("expected 'a' but found 'b' at line 1, column 1");

    let actual = 
        Combinator::new(p_char('a'))
            .take_next(p_char('b'))
            .run(String::from("bac"));

    assert_eq!(expected, actual.unwrap_err().to_err_msg());
}

#[test]
fn fails_parsing_with_take_next_at_second_parser() {
    let expected = String::from("expected 'b' but found 'c' at line 1, column 2");

    let actual = 
        Combinator::new(p_char('a'))
            .take_next(p_char('b'))
            .run(String::from("acb"));

    assert_eq!(expected, actual.unwrap_err().to_err_msg());
}

#[test]
fn succeeds_parsing_with_combinator_of_four_parsers() {
    let expected = Ok((String::from("hello"), String::from("world")));
    
    let p_hello = p_string(String::from("hello"));
    let p_comma = p_string(String::from(", "));
    let p_world = p_string(String::from("world"));
    
    let actual = 
        Combinator::new(p_char('c'))
            .take_next(p_hello)
            .take_prev(p_comma)
            .and(p_world)
            .run(String::from("chello, world"));

    assert_eq!(expected, actual);
}

#[test]
fn succeeds_parsing_with_combinator_of_combinators() {
    let expected = Ok((String::from("goodbye"), String::from("world")));
    
    let p_hello = p_string(String::from("hello"));
    let p_goodbye = p_string(String::from("goodbye"));

    let p_hello_goodbye = 
        Combinator::new(p_hello).or(p_goodbye).get_parser();

    let p_comma = p_char(',');
    let p_world = p_string(String::from("world"));

    let p_comma_world =
        Combinator::new(p_comma)
            .take_next(ws())
            .take_next(p_world)
            .get_parser();
    
    let actual =
        Combinator::new(p_char('c'))
            .take_next(p_hello_goodbye)
            .and(p_comma_world)
            .run(String::from("cgoodbye, world"));

    assert_eq!(expected, actual);
}

#[test]
fn succeeds_parsing_with_map() {
    let expected = Ok(String::from("hello, world"));
    
    let p_hello = p_string(String::from("hello"));
    let to_hello_world = Box::new(|result: String| format!("{}, world", result));

    let actual =
        Combinator::new(p_hello)
            .map(to_hello_world)
            .run(String::from("hello, y'all"));

    assert_eq!(expected, actual);
}

#[test]
fn tracks_line_and_column_number_for_error_messaging() {
    let expected = String::from("expected 'b' but found 'c' at line 5, column 3");

    let p_hello = p_string(String::from("hello"));
    let p_ab = p_string(String::from("ab"));

    let actual = 
        Combinator::new(p_hello)
            .take_prev(ws())
            .take_prev(p_char('a'))
            .take_prev(ws())
            .take_prev(p_ab)
            .and(p_char('b'))
            .run(String::from("hello\n\na\n\nabc"));

    assert_eq!(expected, actual.unwrap_err().to_err_msg());
}
