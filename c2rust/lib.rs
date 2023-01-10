#![allow(dead_code)]
#![allow(mutable_transmutes)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_assignments)]
#![allow(unused_mut)]
#![feature(c_variadic)]
#![feature(core_intrinsics)]
#![feature(extern_types)]
#![feature(label_break_value)]
#![feature(register_tool)]
#![register_tool(c2rust)]

#[macro_use]
extern crate c2rust_bitfields;
extern crate f128;#[macro_use]
extern crate num_traits;
extern crate libc;
pub mod src {
pub mod shell;
pub mod src {
pub mod tclsqlite;
} // mod src
pub mod tool {
pub mod lemon;
pub mod mkkeywordhash;
pub mod mksourceid;
} // mod tool
} // mod src
