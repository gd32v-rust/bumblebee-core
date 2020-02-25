#![no_std]

#[macro_use]
mod macros;

#[export_name = "error: bumblebee-core appears more than once in the dependency graph"]
#[doc(hidden)]
pub static __ONCE__: () = ();

pub mod register;

