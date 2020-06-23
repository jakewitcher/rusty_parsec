use rusty_parsec::*;

pub fn p_true() -> Parser<bool> {
    p_string("true".to_string())
        .then_return(true)
}

pub fn p_hello() -> Parser<String> {
    p_string("hello".to_string())
}

pub fn p_abc_123() -> Parser<(String, u32)> {
    tuple_2(p_string("abc".to_string()), p_u32())
}