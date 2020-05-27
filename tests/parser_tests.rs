use rusty_parsec::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn succeeds_parsing_expected_char() {        
        let expected = Ok('a');

        let actual = 
            Combinator::new(p_char('a')).run(String::from("abc"));
        
        assert_eq!(expected, actual);
    }

    #[test]
    fn fails_parsing_expected_char() {
        let expected = String::from("expected 'b' but found 'a' at line 1, column 1");        
        
        let actual = 
            Combinator::new(p_char('b')).run(String::from("abc"));

        assert_eq!(expected, actual.unwrap_err().to_err_msg());
    }
   
    #[test]
    fn succeeds_parsing_two_chars() {
        let expected = Ok(('a', 'b'));

        let actual = 
            Combinator::new(p_char('a'))
                .and(p_char('b'))
                .run(String::from("abc"));

        assert_eq!(expected, actual);
    }

    #[test]
    fn fails_parsing_first_of_two_chars() {
        let expected = String::from("expected 'a' but found 'b' at line 1, column 1");

        let actual = 
            Combinator::new(p_char('a'))
                .and(p_char('b'))
                .run(String::from("bca"));

        assert_eq!(expected, actual.unwrap_err().to_err_msg());
    }

    #[test]
    fn fails_parsing_second_of_two_chars() {
        let expected = String::from("expected 'b' but found 'c' at line 1, column 2");

        let actual = 
            Combinator::new(p_char('a'))
                .and(p_char('b'))
                .run(String::from("acb"));

        assert_eq!(expected, actual.unwrap_err().to_err_msg());
    }

    #[test]
    fn succeeds_parsing_first_of_two_char_options() {
        let expected = Ok('a');

        let actual = 
            Combinator::new(p_char('a'))
                .or(p_char('b'))
                .run(String::from("abc"));

        assert_eq!(expected, actual);
    }

    #[test]
    fn succeeds_parsing_second_of_two_char_options() {
        let expected = Ok('b');

        let actual = 
            Combinator::new(p_char('a'))
                .or(p_char('b'))
                .run(String::from("bac"));

        assert_eq!(expected, actual);
    }

    #[test]
    fn fails_parsing_second_of_two_char_options() {
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
    fn succeeds_parsing_two_parsers_keeps_first() {
        let expected = Ok('a');

        let actual = 
            Combinator::new(p_char('a'))
                .take_prev(p_char('b'))
                .run(String::from("abc"));

        assert_eq!(expected, actual);
    }

    #[test]
    fn fails_first_parser_parsing_two_parsers_keeps_first() {
        let expected = String::from("expected 'a' but found 'b' at line 1, column 1");

        let actual = 
            Combinator::new(p_char('a'))
                .take_prev(p_char('b'))
                .run(String::from("bac"));

        assert_eq!(expected, actual.unwrap_err().to_err_msg());
    }

    #[test]
    fn fails_second_parser_parsing_two_parsers_keeps_first() {
        let expected = String::from("expected 'b' but found 'c' at line 1, column 2");

        let actual = 
            Combinator::new(p_char('a'))
                .take_prev(p_char('b'))
                .run(String::from("acb"));

        assert_eq!(expected, actual.unwrap_err().to_err_msg());
    }

    #[test]
    fn succeeds_parsing_two_parsers_keeps_second() {
        let expected = Ok('b');

        let actual = 
            Combinator::new(p_char('a'))
                .take_next(p_char('b'))
                .run(String::from("abc"));

        assert_eq!(expected, actual);
    }

    #[test]
    fn fails_first_parser_parsing_two_parsers_keeps_second() {
        let expected = String::from("expected 'a' but found 'b' at line 1, column 1");

        let actual = 
            Combinator::new(p_char('a'))
                .take_next(p_char('b'))
                .run(String::from("bac"));

        assert_eq!(expected, actual.unwrap_err().to_err_msg());
    }

    #[test]
    fn fails_second_parser_parsing_two_parsers_keeps_second() {
        let expected = String::from("expected 'b' but found 'c' at line 1, column 2");

        let actual = 
            Combinator::new(p_char('a'))
                .take_next(p_char('b'))
                .run(String::from("acb"));

        assert_eq!(expected, actual.unwrap_err().to_err_msg());
    }

    #[test]
    fn succeeds_parsing_expected_string() {    
        let expected = Ok(String::from("hello"));

        let p_hello = p_string(String::from("hello"));

        let actual = 
            Combinator::new(p_hello)
                .run(String::from("hello, world"));

        assert_eq!(expected, actual);
    }

    #[test]
    fn fails_parsing_expected_string() {
        let expected = String::from("expected 'hello' but found 'chell' at line 1, column 1");
        
        let p_hello = p_string(String::from("hello"));
        
        let actual = 
            Combinator::new(p_hello)
                .run(String::from("chello, world"));

        assert_eq!(expected, actual.unwrap_err().to_err_msg());
    }

    #[test]
    fn fails_parsing_expected_string_when_input_is_too_short() {
        let expected = String::from("expected 'hello' but found unknown error at line 1, column 1");
        
        let p_hello = p_string(String::from("hello"));
        
        let actual = 
            Combinator::new(p_hello)
                .run(String::from("hell"));

        assert_eq!(expected, actual.unwrap_err().to_err_msg());
    }

    #[test]
    fn succeeds_parsing_expected_char_followed_by_string() {
        let expected = Ok(String::from("hello"));
        
        let p_hello = p_string(String::from("hello"));
        
        let actual = 
            Combinator::new(p_char('c'))
                .take_next(p_hello)
                .run(String::from("chello, world"));

        assert_eq!(expected, actual);
    }

    #[test]
    fn succeeds_parsing_expected_string_followed_by_second_string() {
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
    fn succeeds_parsing_hello_world_from_chello_comma_world() {
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
    fn succeeds_parsing_hello_or_goodbye_world_from_chello_comma_world() {
        let expected = Ok((String::from("goodbye"), String::from("world")));
        
        let p_hello = p_string(String::from("hello"));
        let p_goodbye = p_string(String::from("goodbye"));

        let p_hello_goodbye = 
            Combinator::new(p_hello).or(p_goodbye).get_parser();

        let p_comma = p_string(String::from(", "));
        let p_world = p_string(String::from("world"));
        
        let actual =
            Combinator::new(p_char('c'))
                .take_next(p_hello_goodbye)
                .take_prev(p_comma)
                .and(p_world)
                .run(String::from("cgoodbye, world"));

        assert_eq!(expected, actual);
    }

    #[test]
    fn succeeds_parsing_and_maps_to_different_result() {
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
    fn succeeds_parsing_spaces_before_and_after_char() {
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
    fn fails_parsing_spaces_before_and_after_char() {
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
    fn succeeds_in_tracking_line_and_column_numbers() {
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

    #[test]
    fn succeeds_parsing_integer_value() {
        let expected = Ok(123);

        let actual =
            Combinator::new(p_int())
            .run(String::from("123abc"));

        assert_eq!(expected, actual);
    }

    #[test]
    fn fails_parsing_integer_value() {
        let expected = String::from("expected 'integral value' but found unknown error at line 1, column 1");

        let actual =
            Combinator::new(p_int())
            .run(String::from("abc"));

        assert_eq!(expected, actual.unwrap_err().to_err_msg());
    }

    #[test]
    fn succeeds_parsing_integer_value_followed_by_string() {
        let expected = Ok((123, String::from("abc")));

        let actual =
            Combinator::new(p_int())
            .and(p_string(String::from("abc")))
            .run(String::from("123abc"));

        assert_eq!(expected, actual);
    }
}
