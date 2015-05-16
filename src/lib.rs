#![feature(no_std,core,collections)]
#![no_std]

#[macro_use]
extern crate core;
//#[cfg(feature = "core")]
extern crate collections;

//#[cfg(feature = "core")]
mod std {
#[macro_use]
  pub use core::{fmt, iter, option, ops, slice, mem};
  pub use collections::{boxed, vec, string};
  pub mod prelude {
    pub use core::prelude as v1;
  }
}

use std::prelude::v1::*;
use std::vec::Vec;

#[macro_use] extern crate nom;

use nom::{IResult, eof};

pub fn it_works() -> i32 {
  42
}

named!(omnom_parser <&[u8],Vec<&[u8]> >,
  chain!(
          tag!("om")           ~
    noms: many1!(tag!("nom")) ~
          tag!("kthxbye")      ~
          eof                  ,
          || { noms }
  )
);

pub fn count_noms(s:&[u8]) -> Option<usize> {
  if let IResult::Done(_, res) = omnom_parser(s) {
    Some(res.len())
  } else {
    None
  }

}
