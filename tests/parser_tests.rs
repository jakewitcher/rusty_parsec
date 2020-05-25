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
        let expected = Err(String::from("expected 'b' but found 'a'"));        
        
        let actual = 
            Combinator::new(p_char('b')).run(String::from("abc"));
        
        assert_eq!(expected, actual);
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
        let expected = Err(String::from("expected 'a' but found 'b'"));

        let actual = 
            Combinator::new(p_char('a'))
                .and(p_char('b'))
                .run(String::from("bca"));

        assert_eq!(expected, actual);
    }

    #[test]
    fn fails_parsing_second_of_two_chars() {
        let expected = Err(String::from("expected 'b' but found 'c'"));

        let actual = 
            Combinator::new(p_char('a'))
                .and(p_char('b'))
                .run(String::from("acb"));

        assert_eq!(expected, actual);
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
        let expected = Err(String::from("expected 'b' but found 'c'"));

        let actual = 
            Combinator::new(p_char('a'))
                .or(p_char('b'))
                .run(String::from("cba"));

        assert_eq!(expected, actual);
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
        let expected = Err(String::from("expected 'a' but found 'b'"));

        let actual = 
            Combinator::new(p_char('a'))
                .take_prev(p_char('b'))
                .run(String::from("bac"));

        assert_eq!(expected, actual);
    }

    #[test]
    fn fails_second_parser_parsing_two_parsers_keeps_first() {
        let expected = Err(String::from("expected 'b' but found 'c'"));

        let actual = 
            Combinator::new(p_char('a'))
                .take_prev(p_char('b'))
                .run(String::from("acb"));

        assert_eq!(expected, actual);
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
        let expected = Err(String::from("expected 'a' but found 'b'"));

        let actual = 
            Combinator::new(p_char('a'))
                .take_next(p_char('b'))
                .run(String::from("bac"));

        assert_eq!(expected, actual);
    }

    #[test]
    fn fails_second_parser_parsing_two_parsers_keeps_second() {
        let expected = Err(String::from("expected 'b' but found 'c'"));

        let actual = 
            Combinator::new(p_char('a'))
                .take_next(p_char('b'))
                .run(String::from("acb"));

        assert_eq!(expected, actual);
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
        let expected = Err(String::from("expected 'hello' but found 'chell'"));
        
        let p_hello = p_string(String::from("hello"));
        
        let actual = 
            Combinator::new(p_hello)
                .run(String::from("chello, world"));

        assert_eq!(expected, actual);
    }

    #[test]
    fn fails_parsing_expected_string_when_input_is_too_short() {
        let expected = Err(String::from("expected 'hello' but input string was not long enough"));
        
        let p_hello = p_string(String::from("hello"));
        
        let actual = 
            Combinator::new(p_hello)
                .run(String::from("hell"));

        assert_eq!(expected, actual);
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
}
