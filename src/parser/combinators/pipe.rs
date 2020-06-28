use super::{ParserState, ParserSuccess, ParserResult, Parser};

/// ```pipe_2``` applies the parsers```p1``` and ```p2``` and invokes the function ```f``` with the results if all parsers were successful.
/// 
/// # Examples
/// 
/// ```
/// use rusty_parsec::*;
/// 
/// let p_hello = p_string("hello".to_string());
/// let p_world = p_string("world".to_string());
/// 
/// fn hello_world(hello: String, world: String) -> String {
///     format!("{}, {}", hello, world)
/// }
/// 
/// let expected = 
///     Ok(ParserSuccess::new(
///         "hello, world".to_string(), 
///         Position::new(1, 11, 10))
///     );
/// 
/// let actual = pipe_2(p_hello, p_world, Box::new(hello_world)).run("helloworld".to_string());
/// 
/// assert_eq!(expected, actual);
/// ```
pub fn pipe_2<T, U, V>(p1: Parser<T>, p2: Parser<U>, f: Box<dyn Fn (T, U) -> V>) -> Parser<V> 
where T: 'static, U: 'static
{
    let parser_fn =
        Box::new(
            move |state: &mut ParserState| {
                let r1 = p1.parse(state)?;
                let r2 = apply_parser(p2, state)?;

                let result = 
                    f(
                        r1.get_result(), 
                        r2.get_result()
                    );

                Ok(ParserSuccess::new(result, state.get_position()))
            }
        );

    Parser::new(parser_fn)
}

/// ```pipe_3``` applies the parsers```p1```, ```p2```, and ```p3``` and invokes the function ```f``` with the results if all parsers were successful.
/// 
/// # Examples
/// 
/// ```
/// use rusty_parsec::*;
/// 
/// #[derive(Debug, PartialEq)]
/// struct PhoneNumber {
///     area_code: u32,
///     prefix: u32,
///     line_number: u32,
/// }
/// 
/// impl PhoneNumber {
///     fn new(area_code: u32, prefix: u32, line_number: u32) -> PhoneNumber {
///         PhoneNumber { area_code, prefix, line_number}
///     }
/// }
/// 
/// let p_area_code = 
///     p_u32()
///         .between(p_char('('), p_char(')'))
///         .take_prev(p_char('-'));
/// 
/// let p_prefix = p_u32().take_prev(p_char('-'));
/// 
/// let p_line_number = p_u32();
/// 
/// let phone_number = PhoneNumber::new(555, 422, 1687);
/// 
/// let expected = 
///     Ok(ParserSuccess::new(
///         phone_number, 
///         Position::new(1, 15, 14))
///     );
/// 
/// let actual = pipe_3(
///     p_area_code, 
///     p_prefix, 
///     p_line_number, 
///     Box::new(PhoneNumber::new)
/// ).run("(555)-422-1687".to_string());
/// 
/// assert_eq!(expected, actual);
pub fn pipe_3<T, U, V, W>(p1: Parser<T>, p2: Parser<U>, p3: Parser<V>, f: Box<dyn Fn (T, U, V) -> W>) -> Parser<W> 
where T: 'static, U: 'static, V: 'static
{
    let parser_fn =
        Box::new(
            move |state: &mut ParserState| {
                let r1 = p1.parse(state)?;
                let r2 = apply_parser(p2, state)?;
                let r3 = apply_parser(p3, state)?;

                let result = 
                    f(
                        r1.get_result(), 
                        r2.get_result(), 
                        r3.get_result()
                    );

                Ok(ParserSuccess::new(result, state.get_position()))
            }
        );

    Parser::new(parser_fn)
}

/// ```pipe_4``` applies the parsers```p1```, ```p2```, ```p3```, and ```p4``` and invokes the function ```f``` with the results if all parsers were successful.
/// 
/// # Examples
/// 
/// ```
/// use rusty_parsec::*;
/// 
/// #[derive(Debug, PartialEq)]
/// struct LineItem {
///     id: String,
///     name: String,
///     price: f32,
///     qty: u32,
/// }
/// 
/// impl LineItem {
///     fn new(id: String, name: String, price: f32, qty: u32) -> LineItem {
///         LineItem { id, name, price, qty }
///     }
/// }
/// 
/// let p_id = 
///     many_satisfy(Box::new(|c:char|c.is_ascii_alphanumeric()))
///         .between(p_char('|'), p_char('|'));
/// 
/// let p_name = 
///     many_satisfy(Box::new(|c:char|c.is_ascii_alphabetic()))
///         .take_prev(p_char('|'));
/// 
/// let p_price = p_f32().take_prev(p_char('|'));
/// 
/// let p_qty = p_u32().take_prev(p_char('|'));
/// 
/// let line_item = 
///     LineItem::new("abc123".to_string(), "lamp".to_string(), 12.5, 2);
/// 
/// let expected = 
///     Ok(ParserSuccess::new(
///         line_item,
///         Position::new(1, 22, 21))
///     );
/// 
/// let actual = pipe_4(
///         p_id, 
///         p_name, 
///         p_price, 
///         p_qty, 
///         Box::new(LineItem::new)
/// ).run("|abc123|lamp|12.50|2|".to_string());
/// 
/// assert_eq!(expected, actual);
pub fn pipe_4<T, U, V, W, X>(p1: Parser<T>, p2: Parser<U>, p3: Parser<V>, p4: Parser<W>, f: Box<dyn Fn (T, U, V, W) -> X>) -> Parser<X> 
where T: 'static, U: 'static, V: 'static, W: 'static
{
    let parser_fn =
        Box::new(
            move |state: &mut ParserState| {
                let r1 = p1.parse(state)?;
                let r2 = apply_parser(p2, state)?;
                let r3 = apply_parser(p3, state)?;
                let r4 = apply_parser(p4, state)?;

                let result = 
                    f(
                        r1.get_result(), 
                        r2.get_result(), 
                        r3.get_result(), 
                        r4.get_result()
                    );

                Ok(ParserSuccess::new(result, state.get_position()))
            }
        );

    Parser::new(parser_fn)
}

/// ```pipe_5``` applies the parsers```p1```, ```p2```, ```p3```,```p4```, and ```p5``` and invokes the function ```f``` with the results if all parsers were successful.
/// 
/// # Examples
/// 
/// ```
/// use rusty_parsec::*;
/// 
/// #[derive(Debug, PartialEq)]
/// struct Address {
///     number: u32,
///     street: String,
///     city: String,
///     state: String,
///     zipcode: u32,
/// }
/// 
/// impl Address {
///     fn new(number: u32, street: String, city: String, state: String, zipcode: u32) -> Address {
///         Address { number, street, city, state, zipcode }
///     }
/// }
/// 
/// fn p_alphabetic() -> Parser<String> {
///     many_satisfy(Box::new(|c:char|c.is_ascii_alphabetic()))
/// }
/// 
/// let p_number = p_u32().take_prev(ws());
/// 
/// let p_street = p_alphabetic().take_prev(ws());
/// 
/// let p_city = 
///     p_alphabetic()
///         .take_prev(p_char(','))
///         .take_prev(ws());
/// 
/// let p_state = p_alphabetic().take_prev(ws());
/// 
/// let p_zipcode = p_u32();
/// 
/// let address = Address::new(1200, "Oakwood".to_string(), "Cincinnati".to_string(), "Ohio".to_string(), 45242);
/// 
/// let expected = 
///     Ok(ParserSuccess::new(
///         address,
///         Position::new(1, 36, 35))
///     );
/// 
/// let actual = pipe_5(
///     p_number, 
///     p_street, 
///     p_city, 
///     p_state, 
///     p_zipcode, 
///     Box::new(Address::new)
/// ).run("1200 Oakwood Cincinnati, Ohio 45242".to_string());
/// 
/// assert_eq!(expected, actual);
pub fn pipe_5<T, U, V, W, X, Y>(p1: Parser<T>, p2: Parser<U>, p3: Parser<V>, p4: Parser<W>, p5: Parser<X>, f: Box<dyn Fn (T, U, V, W, X) -> Y>) -> Parser<Y> 
where T: 'static, U: 'static, V: 'static, W: 'static, X: 'static
{
    let parser_fn =
        Box::new(
            move |state: &mut ParserState| {
                let r1 = p1.parse(state)?;
                let r2 = apply_parser(p2, state)?;
                let r3 = apply_parser(p3, state)?;
                let r4 = apply_parser(p4, state)?;
                let r5 = apply_parser(p5, state)?;

                let result = 
                    f(
                        r1.get_result(), 
                        r2.get_result(), 
                        r3.get_result(), 
                        r4.get_result(), 
                        r5.get_result()
                    );

                Ok(ParserSuccess::new(result, state.get_position()))
            }
        );

    Parser::new(parser_fn)
}

pub fn tuple_2<T, U>(p1: Parser<T>, p2: Parser<U>) -> Parser<(T, U)> {
    pipe_2(p1, p2, Box::new(|x1, x2| (x1, x2)))
}

pub fn tuple_3<T, U, V>(p1: Parser<T>, p2: Parser<U>, p3: Parser<V>) -> Parser<(T, U, V)> {
    pipe_3(p1, p2, p3, Box::new(|x1, x2, x3| (x1, x2, x3)))
}

pub fn tuple_4<T, U, V, W>(p1: Parser<T>, p2: Parser<U>, p3: Parser<V>, p4: Parser<W>) -> Parser<(T, U, V, W)> {
    pipe_4(p1, p2, p3, p4, Box::new(|x1, x2, x3, x4| (x1, x2, x3, x4)))
}

pub fn tuple_5<T, U, V, W, X>(p1: Parser<T>, p2: Parser<U>, p3: Parser<V>, p4: Parser<W>, p5: Parser<X>) -> Parser<(T, U, V, W, X)> {
    pipe_5(p1, p2, p3, p4, p5, Box::new(|x1, x2, x3, x4, x5| (x1, x2, x3, x4, x5)))
}

fn apply_parser<T>(p: Parser<T>, state: &mut ParserState) -> ParserResult<T> {
    p.parse(state).map_err(|failure| failure.to_fatal_err())
}