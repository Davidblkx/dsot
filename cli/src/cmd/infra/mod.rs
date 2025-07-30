#[macro_use]
pub mod declare_arg_macros;
#[macro_use]
pub mod command_gen;
pub mod output;

mod command;

pub use command::*;
