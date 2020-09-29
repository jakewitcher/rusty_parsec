# Rusty Parsec

Rusty Parsec is a parser combinator library in Rust based on the F# library FParsec.

Below is an example using the library to parse the phrases "hello, world" or "goodbye, world."

<pre><code>let p_hello = p_string(String::from("hello"));
let p_goodbye = p_string(String::from("goodbye"));
let p_world = p_string(String::from("world"));
let p_comma = p_char(',');

let parser = p_hello
    .or(p_goodbye)
    .take_prev(p_comma)
    .take_prev(ws())
    .and(p_world);

parser.run(String::from("hello, world")); 
    // => ("hello", "world")

parser.run(String::from("goodbye, world")); 
    // => ("goodbye", "world")
    
parser.run(String::from("hello, nerds")); 
    // => "expected 'world' but found 'nerds' at line 1, column 8"</code></pre>