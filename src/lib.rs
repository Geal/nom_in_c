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
