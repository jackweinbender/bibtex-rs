use std::str;
use nom::IResult::Done;
use bibtex::Bibtex;

named!(entry_type, 
    do_parse!(
        tag!("@")                   >>
        entry_type: is_not_s!("{")  >>

        (entry_type)
    )
);

#[test]
fn parse_entry_type() {
    
    let input = b"@article{";
    let output = entry_type(input);

    assert_eq!(output, Done(&b"{"[..], &b"article"[..]));
}


named!(entry_key, 
    do_parse!(
        ws!(tag!("{"))                      >>
        entry_key: is_not_s!(" \t\r\n,}=")  >>
        opt!( tag!(",") )                   >>
        
        (entry_key)
    )
);

#[test]
fn parse_entry_key() {

    assert_eq!( // Key-only entries
        entry_key(b"{thekey}"), 
        Done(&b"}"[..], &b"thekey"[..]));
    assert_eq!( // Key with comma and \n
        entry_key(b"{thekey,\n"), 
        Done(&b"\n"[..], &b"thekey"[..]));

}

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