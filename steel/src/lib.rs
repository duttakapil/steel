extern crate im_rc;

// #[macro_use]
// extern crate lazy_static;

#[macro_use]
pub mod env;
// pub mod evaluator;
#[macro_use]
pub mod interpreter;
pub mod parser;
pub mod primitives;
#[macro_use]
pub mod repl;
#[macro_use]
pub mod rerrs;
pub mod rvals;
pub mod stdlib;
#[macro_use]
pub mod expander;

pub use self::{
    interpreter::SteelInterpreter,
    primitives::SteelFunctor,
    rerrs::SteelErr,
    rvals::{CustomType, SteelVal, StructFunctions},
    stdlib::PRELUDE,
};