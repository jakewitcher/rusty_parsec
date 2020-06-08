use std::collections::HashMap;
use rusty_parsec::*;

#[derive(Debug, PartialEq)]
enum Json {
    JString(String),
    JNumber(f64),
    JBool(bool),
    JNull,
    JList(Vec<Json>),
    JObject(HashMap<String, Json>),
}

fn p_json_null() -> Parser<Json> {
    Combinator::new(p_string("null".to_string()))
        .then_return(Json::JNull)
        .get_parser()

}

fn p_json_bool() -> Parser<Json> {
    let p_true = 
        Combinator::new(p_string("true".to_string()))
            .then_return(Json::JBool(true)).get_parser();

    let p_false = 
        Combinator::new(p_string("false".to_string()))
            .then_return(Json::JBool(false)).get_parser();

    Combinator::new(p_true).or(p_false).get_parser()
}

fn p_json_number() -> Parser<Json> {
    Combinator::new(p_f64())
        .map(Box::new(|float| Json::JNumber(float)))
        .get_parser()
}

fn p_json_string() -> Parser<Json> {
    Combinator::new(many_satisfy(Box::new(|c: char| c != '\"')))
        .map(Box::new(|result| Json::JString(result)))
        .between(p_char('"'), p_char('"'))
        .get_parser()
}

fn p_json_value() -> Parser<Json> {
    choice(vec![
        p_json_object(),
        p_json_list(),
        p_json_string(),
        p_json_number(),
        p_json_bool(),
        p_json_null()
         
    ]).get_parser()
}

fn p_comma() -> Parser<char> {
    Combinator::new(p_char(','))
        .take_prev(ws())
        .get_parser()
}

fn p_json_list() -> Parser<Json> {
    let p_list = sep_by(p_json_value, p_comma).get_parser();
    
    Combinator::new(ws())
        .take_next(p_list)
        .between(p_char('['), p_char(']'))
        .map(Box::new(|list| Json::JList(list)))
        .get_parser()
}

fn p_json_object() -> Parser<Json> {
    let p_object = sep_by(p_key_value, p_comma).get_parser();

    Combinator::new(ws())
        .take_next(p_object)
        .between(p_char('{'), p_char('}'))
        .map(
            Box::new(|list| {
                let mut results = HashMap::new();

                for (name, j_value) in list {
                    results.insert(name, j_value);
                }

                Json::JObject(results)
            })
        ).get_parser()
}

fn p_key_value() -> Parser<(String, Json)> {
    Combinator::new(p_key())
        .take_prev(ws())
        .take_prev(p_char(':'))
        .take_prev(ws())
        .and(p_json_value())
        .get_parser()
}

fn p_key() -> Parser<String> {
    Combinator::new(many_satisfy(Box::new(|c: char| c != '\"')))
        .between(p_char('"'), p_char('"'))
        .get_parser()
}