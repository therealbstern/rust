// error-pattern: crate `cdylib_dep` required to be available in rlib format, but was not found
// aux-build:cdylib-dep.rs
// ignore-musl
// ignore-cloudabi
// ignore-emscripten

#![crate_type = "cdylib"]

extern crate cdylib_dep;
