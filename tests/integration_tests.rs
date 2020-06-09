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
fn succeeds_parsing_simple_json_object() {
    let mut key_value_map = HashMap::new();

    key_value_map.insert("account active".to_string(), Json::JBool(true));
    key_value_map.insert("name".to_string(), Json::JString("Bob".to_string()));
    key_value_map.insert("age".to_string(), Json::JNumber(27.0));

    let expected = Ok(ParserSuccess::new(Json::JObject(key_value_map), Position::new(5, 10, 97)));

    let actual = p_json().run(
        "{
            \"name\":\"Bob\", 
            \"age\": 27, 
            \"account active\": true
        }".to_string());

    assert_eq!(expected, actual);
}

#[test]
fn succeeds_parsing_json_object_with_list() {
    let mut key_value_map = HashMap::new();
    key_value_map.insert("account active".to_string(), Json::JBool(true));
    key_value_map.insert("name".to_string(), Json::JString("Bob".to_string()));
    key_value_map.insert("age".to_string(), Json::JNumber(27.0));
    key_value_map.insert("favorite numbers".to_string(), Json::JList(vec![Json::JNumber(1.0), Json::JNumber(2.0), Json::JNumber(3.0), Json::JNumber(4.0)]));

    let expected = Ok(ParserSuccess::new(Json::JObject(key_value_map), Position::new(11, 10, 221)));

    let actual = p_json().run(
        "{
            \"name\":\"Bob\",
            \"age\": 27,
            \"account active\": true,
            \"favorite numbers\":[
                1, 
                2, 
                3, 
                4
            ]
        }".to_string()
    );

    assert_eq!(expected, actual);
}

#[test]
fn succeed_parsing_list_of_json_objects() {
    let person1 = create_person("Bob".to_string(), 35.0);
    let person2 = create_person("Alex".to_string(), 48.0);
    let person3 = create_person("Sarah".to_string(), 23.0);

    let expected = Ok(ParserSuccess::new(Json::JList(vec![person1, person2, person3]), Position::new(12, 10, 253)));

    let actual = p_json().run(
        "[
            { 
                \"name\":\"Bob\", 
                \"age\": 35 
            }, { 
                \"name\":\"Alex\", 
                \"age\": 48 
            }, { 
                \"name\":\"Sarah\", 
                \"age\": 23 
            }
        ]".to_string()
    );

    assert_eq!(expected, actual);
}

fn create_person(name: String, age: f64) -> Json {
    let mut person = HashMap::new();
    person.insert("name".to_string(), Json::JString(name));
    person.insert("age".to_string(), Json::JNumber(age));

    Json::JObject(person)
}

#[test]
fn suceeds_parsing_nested_json_objects_and_lists() {
    let person1 = create_person_with_address("Bob".to_string(), 35.0, 1008.0, "Main St.".to_string());
    let person2 = create_person_with_address("Alex".to_string(), 48.0, 520.0, "Elm St.".to_string());
    let person3 = create_person_with_address("Sarah".to_string(), 23.0, 1300.0, "Vine St.".to_string());

    let mut obj = HashMap::new();
    obj.insert("persons".to_string(), Json::JList(vec![person1, person2, person3]));

    let expected = Ok(ParserSuccess::new(Json::JObject(obj), Position::new(41, 10, 1227)));

    let actual = p_json().run(
        "{
            \"persons\": [
                { 
                    \"name\":\"Bob\", 
                    \"age\": 35,
                    \"address\": {
                        \"number\": 1008,
                        \"street\": \"Main St.\"
                    },
                    \"primary colors\": [
                        \"red\",
                        \"blue\",
                        \"yellow\"
                    ]
                }, { 
                    \"name\":\"Alex\", 
                    \"age\": 48,
                    \"address\": {
                        \"number\": 520,
                        \"street\": \"Elm St.\"
                    },
                    \"primary colors\": [
                        \"red\",
                        \"blue\",
                        \"yellow\"
                    ]
                }, { 
                    \"name\":\"Sarah\", 
                    \"age\": 23,
                    \"address\": {
                        \"number\": 1300,
                        \"street\": \"Vine St.\"
                    },
                    \"primary colors\": [
                        \"red\",
                        \"blue\",
                        \"yellow\"
                    ]
                }
            ]
        }".to_string()
    );

    assert_eq!(expected, actual);
}

fn create_person_with_address(name: String, age: f64, number: f64, street:String) -> Json {
    let mut person = HashMap::new();
    person.insert("name".to_string(), Json::JString(name));
    person.insert("age".to_string(), Json::JNumber(age));
    person.insert("address".to_string(), create_address(number, street));
    person.insert(
        "primary colors".to_string(), 
        Json::JList(vec![
            Json::JString("red".to_string()), 
            Json::JString("blue".to_string()), 
            Json::JString("yellow".to_string())
        ])
    );

    Json::JObject(person)
}

fn create_address(number: f64, street: String) -> Json {
    let mut address = HashMap::new();
    address.insert("number".to_string(), Json::JNumber(number));
    address.insert("street".to_string(), Json::JString(street));

    Json::JObject(address)
}