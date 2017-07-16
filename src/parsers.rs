use std::str;
use bibtex::Bibtex;
use bibtex::AttrValue;


fn to_slice(input: &[u8]) -> &str {
    str::from_utf8(input).unwrap()
}

named!(entry_type, 
    do_parse!(
        tag!("@")                   >>
        entry_type: is_not_s!("{")  >>

        (entry_type)
    )
);
named!(entry_key, 
    do_parse!(
        ws!(tag!("{"))                        >>
        entry_key: is_not_s!(" \t\r\n,}=")    >>
        ws!( opt!( complete!( tag!(",") ) ) ) >> 
        
        (entry_key)
    )
);

named!(num,
    do_parse!(
        num: ws!( is_a_s!("0123456789") )   >>
        ws!( opt!( complete!( tag!(",") ) ) )    >>

        (num)
    )
);
named!(braced_string,
    do_parse!(

    )
);
// named!(attr<&[u8], (&str, Vec<AttrValue>)>,
//     do_parse!(
//         key: is_not_s!(" \t\r\n,}=")      >>
//         value: many1( alt!( num | string | alias ) )
//     )
// );
named!(string<&[u8], Bibtex>,
    do_parse!(
        entry_type                  >>
        k: entry_key                >>
        ws!(tag!("="))              >>
        v: delimited!(char!('{'), is_not!("}"), char!('}')) >>
        tag!("}")                   >>
        ( // Return Enum
            Bibtex::String{
                key: to_slice(k).trim(), 
                value: to_slice(v).trim()
            }
        )
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
                t:   to_slice(t), 
                key: to_slice(k),
                attrs: vec![]
            }
        )
    )
);

named!(bib<&[u8], Vec<Bibtex> >, ws!(many0!( alt!(entry | string) )));

#[cfg(test)]
mod tests {
    use super::*;
    use nom::IResult::Done;
    #[test]
    fn parse_entry_type() {
        
        let input = b"@article{";
        let output = entry_type(input);

        // Handles entry type, leaves brace
        assert_eq!(output, Done(&b"{"[..], &b"article"[..]));
    }

    #[test]
    fn parse_entry_key() {

        let the_key = &b"thekey"[..];

        // Handles key-only entries
        assert_eq!( 
            entry_key(b"{thekey}"), 
            Done(&b"}"[..], the_key));
        // Handles standard syntax (key with comma and \n)
        assert_eq!( 
            entry_key(b"{thekey,\n"), 
            Done(&b"\n"[..], the_key));

    }
    #[test]
    fn parse_string() {

        let input = b"@string{ vanderkam = { James VanderKam }}";
        let expected = Bibtex::String {
            key: "vanderkam",
            value: "James VanderKam"
        };

        // Parses Bibtex::String type
        assert_eq!( 
            string(input), 
            Done(&b""[..], expected));
    }
    #[test]
    fn parse_num() {
        
        let input_1 = b"1234,\n";
        let input_2 = b"1234\n";
        
        assert_eq!(num(input_1), Done(&b""[..], &b"1234"[..]));
        assert_eq!(num(input_2), Done(&b""[..], &b"1234"[..]));

    }
    #[test]
    fn parse_braced_string() {
        
        let input_1 = b"{In the {{Beginning}}, {God} created},\n";
        let input_2 = b"{In the {{Beginning}}, {God} created}\n";
        
        assert_eq!(braced_string(input_1), 
            Done(&b""[..], &b"In the {{Beginning}}, {God} created"[..]));
        assert_eq!(braced_string(input_2), 
            Done(&b""[..], &b"In the {{Beginning}}, {God} created"[..]));

    }
    // #[test]
    // fn parse_attr() {

    //     let input_1 = b"Title = {{Bib}\\TeX}";
    //     let input_2 = b"Title = \"{Bib}\\TeX\"";
    //     let input_3 = b"Title = \"{Bib}\" # \"\\TeX\"";

    //     let expected = "{Bib}\\TeX";

    //     assert_eq!(attrs(input_1), Done(&b""[..], expected));

    // }
}