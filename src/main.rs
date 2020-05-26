use rusty_parsec::*;

fn main() {
    // p_string takes a String as an expected value and returns a parser that succeeds 
    // if the input String matches the expected String
    let p_hello = p_string(String::from("hello"));
    let p_goodbye = p_string(String::from("goodbye"));

    // calling "Combinator::new(parser_A).or(parser_B)" creates a parser that succeeds
    // if either the first parser or the second parser is successful. 
    // The parsers can be any type of parser like string parsers, char parsers, or a combination of several parsers
    let p_goodbye_or_hello =
        Combinator::new(p_goodbye).or(p_hello).get_parser();

    // p_char is similar to p_string but instead of strings values it works for char values
    let p_comma = p_char(',');
    let p_world = p_string(String::from("world"));

    // "Combinator::new(parser_A) creates a new Combinator struct which has methods like 
    // "or", "ws()" (which parses all whitespace characters) or "and" for combining parsers together
    let hello_world =
        Combinator::new(p_goodbye_or_hello)
            .take_prev(p_comma)
            .take_prev(ws())
            .and(p_world)
            .run(String::from("hello, world"));

    match hello_world {
        Ok((hello, world)) => 
            println!("successfully parsed '{}' and '{}'", hello, world),
        _ =>
            println!("Failed to parse string 'hello, world'"),
    }


    // error messages provide information about what the parser was expecting and what was found instead
    // along with line and column numnber where the error occurred
    let p_hello = p_string(String::from("hello"));
    let p_error = p_string(String::from("error"));

    let hello_error =
        Combinator::new(p_hello)
            .take_prev(ws())
            .take_prev(p_char(','))
            .take_prev(ws())
            .and(p_error)
            .run(String::from("hello\n\n\n, world"));

    match hello_error {
        Ok((hello, error)) => 
            println!("successfully parsed '{}' and '{}'", hello, error),
        Err(err) =>
            println!("{}", err.to_err_msg()),
    }
}
