
use std::str::Chars;

fn main() {
    let input = include_str!("bib.bib");
    let mut iter = input.chars();

    print!("{:?}\n", parse(iter));

}

fn parse(mut iter: Chars) -> Vec<Entry> {
    
    let mut lib = vec![];

    while let Some(ch) = iter.next() {
        match ch {
            '@' => { 
                let (entry, i) = next_entry(iter);
                lib.push(entry);
                iter = i;
             },
             _  => { continue },
        }

    }

    lib
}

fn next_entry(mut iter: Chars) -> (Entry, Chars) {
    
    let mut buffer = String::new();
    let mut brace_depth = 0;

    while let Some(ch) = iter.next() {
        match ch {
            '}' if brace_depth <= 1 => { break },
            '}' => {
                buffer.push(ch);
                brace_depth = brace_depth - 1;
                },
            '{' => {
                buffer.push(ch);
                brace_depth = brace_depth + 1 
                },
            _ => buffer.push(ch)
        }
    }
    (Entry{content: buffer}, iter)
}

#[derive(Debug)]
struct Entry { content: String }

impl FromStr for Entry {
    
}