use rusty_parsec::*;

fn main() {
    let p_hello = p_string(String::from("hello"));
    let p_goodbye = p_string(String::from("goodbye"));

    let p_goodbye_or_hello =
        Combinator::new(p_goodbye).or(p_hello).get_parser();

    let p_comma = p_char(',');
    let p_world = p_string(String::from("world"));

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
}
