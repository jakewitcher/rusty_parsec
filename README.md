# Rusty Parsec

Rusty Parsec is a parser combinator library in Rust based on the F# library FParsec. Which was based on the Haskell library Parsec. 

Par-secption.

Rusty Parsec is in very early stages of development and I am not an experienced Rust developer, as such this library has a long way to go.

Below is an example using the library to parse the phrases "hello, world" or "goodbye, world."

<pre><code>let p_hello = p_string("hello".to_string());
let p_goodbye = p_string("goodbye".to_string());
let p_world = p_string("world".to_string());
let p_comma = p_char(',');

let parser = 
    Combinator::new(p_hello)
        .or(p_goodbye)
        .take_prev(p_comma)
        .take_prev(ws())
        .and(p_world);

parser.run("hello, world".to_string()); 
    // => Ok("hello", "world")

parser.run("goodbye, world".to_string()); 
    // => Ok("goodbye", "world")
    
parser.run("hello, nerds".to_string()); 
    // => "expected 'world' but found 'nerds' at line 1, column 8"</code></pre>