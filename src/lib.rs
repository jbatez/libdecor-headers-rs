#![allow(nonstandard_style)]
#![doc = include_str!("../README.md")]
#![cfg_attr(not(test), no_std)]

pub mod libdecor;

#[cfg(test)]
mod test;
