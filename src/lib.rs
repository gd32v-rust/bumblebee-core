#![no_std]

#[export_name = "error: bumblebee-core appears more than once in the dependency graph"]
#[doc(hidden)]
pub static __ONCE__: () = ();

