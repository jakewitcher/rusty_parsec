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

fn p_json() -> Combinator<Json> {
    Combinator::new(p_json_value())
}

fn p_json_null() -> Parser<Json> {
    Combinator::new(p_string("null".to_string()))
        .then_return(Json::JNull)
        .take_prev(ws())
        .get_parser()

}

fn p_json_bool() -> Parser<Json> {
    let p_true = 
        Combinator::new(p_string("true".to_string()))
            .then_return(Json::JBool(true)).get_parser();

    let p_false = 
        Combinator::new(p_string("false".to_string()))
            .then_return(Json::JBool(false)).get_parser();

    Combinator::new(p_true).or(p_false)
        .take_prev(ws())
        .get_parser()
}

fn p_json_number() -> Parser<Json> {
    Combinator::new(p_f64())
        .map(Box::new(|float| Json::JNumber(float)))
        .take_prev(ws())
        .get_parser()
}

fn p_json_string() -> Parser<Json> {
    Combinator::new(many_satisfy(Box::new(|c: char| c != '\"')))
        .map(Box::new(|result| Json::JString(result)))
        .between(p_char('"'), p_char('"'))
        .take_prev(ws())
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
        .take_prev(ws())
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
        )
        .take_prev(ws())
        .get_parser()
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

#[test]
fn suceeds_parsing_simple_json_object() {
    let mut key_value_map = HashMap::new();

    key_value_map.insert("account active".to_string(), Json::JBool(true));
    key_value_map.insert("name".to_string(), Json::JString("Bob".to_string()));
    key_value_map.insert("age".to_string(), Json::JNumber(27.0));

    let expected = Ok(ParserSuccess::new(Json::JObject(key_value_map), Position::new(1, 50, 49)));

    let actual = p_json().run("{\"name\":\"Bob\", \"age\": 27, \"account active\": true}".to_string());

    assert_eq!(expected, actual);
}

#[test]
fn suceeds_parsing_json_object_with_list() {
    let mut key_value_map = HashMap::new();
    key_value_map.insert("account active".to_string(), Json::JBool(true));
    key_value_map.insert("name".to_string(), Json::JString("Bob".to_string()));
    key_value_map.insert("age".to_string(), Json::JNumber(27.0));
    key_value_map.insert("favorite numbers".to_string(), Json::JList(vec![Json::JNumber(1.0), Json::JNumber(2.0), Json::JNumber(3.0), Json::JNumber(4.0)]));

    let expected = Ok(ParserSuccess::new(Json::JObject(key_value_map), Position::new(6, 2, 84)));

    let actual = p_json().run(
        "{\n\"name\":\"Bob\",\n\"age\": 27,\n\"account active\": true,\n\"favorite numbers\":[1, 2, 3, 4]\n}".to_string()
    );

    assert_eq!(expected, actual);
}