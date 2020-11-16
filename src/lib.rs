//! This library provides wrappers for LLVM that are memory-safe and follow
//! Rust idioms.
//!
//! The original LLVM reference is available [here](http://llvm.org/doxygen/)
//! but take note that this isn't as thorough as this documentation.

extern crate cbox;
extern crate libc;
extern crate llvm_sys as ffi;

#[macro_use]
mod macros;
mod binary;
mod block;
mod buffer;
mod builder;
mod compile;
mod context;
mod engine;
mod module;
mod target;
pub mod types;
mod util;
pub mod value;

pub use binary::{Binary, BinaryType};
pub use block::BasicBlock;
pub use builder::Builder;
pub use cbox::{CBox, CSemiBox};
pub use compile::Compile;
pub use context::{Context, GetContext};
pub use engine::{
    ExecutionEngine, GenericValue, GenericValueCast, Interpreter, JitEngine, JitOptions,
};
pub use module::{AddressSpace, Functions, Module};
pub use target::{Target, TargetData};
pub use types::*;
pub use util::Sub;
pub use value::{
    Alias, Attribute, Function, GlobalValue, GlobalVariable, Linkage, Param, Predicate, Value,
};
