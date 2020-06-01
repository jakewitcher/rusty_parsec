pub mod combinator;
pub mod result;
pub mod function;
pub mod state;

pub use combinator::Combinator;
pub use state::ParserState;
pub use result::{Position, ParserSuccess, ParserFailure, ParserResult};

pub type Parser<TResult> = Box<dyn FnOnce(&mut ParserState) -> ParserResult<TResult>>;