#![feature(specialization)]

extern crate parsepatch;

#[macro_use]
extern crate pyo3;

pub mod counts;
pub mod common;
pub mod diffs;
pub mod init;