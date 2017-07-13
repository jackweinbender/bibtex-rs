#[macro_use]

extern crate nom;

use std::str;

named!(entry<&[u8], Vec<&[u8]> >, many0!( ws!(tag!( "@entry{}" )) ) );

fn main() {
    let input = b"@entry{} @entry{}";
    let output = entry(input);

    let (res0, res1) = output.unwrap();

    let expected = "";

    println!("{:?}, {:?}", str::from_utf8(res0), res1);

    // assert!(output.unwrap(), expected);
}