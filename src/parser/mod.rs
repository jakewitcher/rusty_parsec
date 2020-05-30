pub mod combinator;
pub mod error;
pub mod function;
pub mod state;

use error::ParserError;
use state::ParserState;

pub type ParserResult<TResult> = Result<TResult, ParserError>;
pub type Parser<TResult> = Box<dyn FnOnce(&mut ParserState) -> ParserResult<TResult>>;