
use std::str::Chars;

fn main() {

    let mut lib = Library;

    let input = include_str!("bib.bib");
    let mut iter = input.chars();



    while let Some(ch) = iter.next() {

        match ch {
            '@' => {
                let (entry, i) = entry(iter.clone());
                print!("{:?}", entry);
            },
            _ => continue,
        }

    }

}
#[derive(Debug)]
struct Library;
#[derive(Debug)]
struct Entry;

fn entry(mut iter: Chars) -> (Entry, Chars) {


    (Entry, iter)
}