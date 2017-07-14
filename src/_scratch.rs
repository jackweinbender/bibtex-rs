#[macro_use] extern crate nom;

mod bibtex;

use std::str;
use bibtex::Bibtex;

named!(entry_type, 
    do_parse!(
        tag!("@")                   >>
        entry_type: is_not_s!("{")  >>
        tag!("{")                   >>

        (entry_type)
    )
);

#[test]
fn entry_type() {
    unimplemented!();
}

named!(entry_key, 
    do_parse!(
        entry_key: is_not_s!(" \t\r\n,}=")   >>
        ws!(opt!( tag!(",") ))              >>
        
        (entry_key)
    )
);

named!(entry<&[u8], Bibtex>,
    do_parse!(
        t: entry_type               >>
        k: entry_key                >>
        ws!( tag!("}"))             >>
        ws!( opt!( tag!(",") ) )    >>
        
        ( // Return Enum
            Bibtex::Entry {
                t: str::from_utf8(t).unwrap(), 
                key: str::from_utf8(k).unwrap(),
                attrs: vec![]
            }
        )
    )
);

named!(string<&[u8], Bibtex>,
    do_parse!(
        entry_type                  >>
        k: entry_key                >>
        ws!(tag!("="))              >>
        v: delimited!(char!('{'), is_not!("}"), char!('}')) >>
        
        ( // Return Enum
            Bibtex::String{
                key: str::from_utf8(k).unwrap(), 
                value: str::from_utf8(v).unwrap()
            }
        )
    )
);

named!(bib<&[u8], Vec<Bibtex> >, ws!(many0!( alt!(entry | string) )));

fn main() {
    let input = b"@entry{vermes,\n\t},\n\t@string{vanderkam = {James VanderKam},";
    let output = bib(input);


    println!("{:?}", output);
}