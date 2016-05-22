#![feature(collections)]
#![no_std]

//extern crate core;
//#[cfg(feature = "core")]
#[macro_use]
extern crate collections;

//#[cfg(feature = "core")]
mod std {
  pub use core::{fmt, iter, option, ops, slice, mem};
  pub use collections::{boxed, vec, string};
  pub mod prelude {
    pub use core::prelude as v1;
  }
}

use std::prelude::v1::*;
use std::vec::Vec;

use collections::boxed::Box;

#[macro_use] extern crate nom;

use nom::{IResult,eof,ErrorKind};

pub fn it_works() -> i32 {
  42
}

named!(omnom_parser <&[u8],Vec<&[u8]> >,
  chain!(
          tag!("om")           ~
    noms: many1!(tag!("nom")) ~
          error!(ErrorKind::Custom(0),tag!("kthxbye"))      ~
          eof                  ,
          || { noms }
  )
);

pub fn count_noms(s:&[u8]) -> Option<usize> {
  //let a = 1;
  //let b = Box::new(a);
  match omnom_parser(s) {
    IResult::Done(_, res) => Some(res.len()),
    _                     => None,
    /*
    IResult::Error(Err::Code(e)) => {None},
    IResult::Error(Err::Position(e,p)) => {None},
    IResult::Error(Err::Node(e, n)) => {None},
    IResult::Error(Err::NodePosition(e,p,n)) => {None},
    IResult::Incomplete(i) => {None}
    */
  }
  /*if let IResult::Done(_, res) = omnom_parser(s) {
    Some(res.len())
  } else {
    None
  }*/

}
