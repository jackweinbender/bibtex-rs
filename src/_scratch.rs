#[macro_use] extern crate nom;

use std::str;
use nom::be_u8;

#[derive(Debug)]
struct TT<'a> {
    ty: &'a str,
    ky: &'a str
}

named!(entry<&[u8], TT>,
    do_parse!(
        tag!("@")           >>
        t: is_not_s!("{")   >>
        k: is_not_s!("}")   >>
        
        ( TT{
            ty: str::from_utf8(t).unwrap(), 
            ky: str::from_utf8(k).unwrap()
            } )
    )
);

fn main() {
    let input = b"@entry{vermes,} @entry{vanderkam}";
    let output = entry(input);

    // debug!("output: {:?}", output);

    println!("{:?}", output);
}