pub mod combinator;
pub mod result;
pub mod parsers;
pub mod combinators;
pub mod state;

pub use combinator::Parser;
pub use state::ParserState;
pub use result::{Position, ParserSuccess, ParserFailure, ParserResult};

pub type ParserFn <T> = Box<dyn FnOnce(&mut ParserState) -> ParserResult<T>>;