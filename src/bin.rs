use std::str::*;

const in_string: &str = "@article{key}";

pub fn main() {
    let slice = take_til(in_string, '{');
    println!("{:?}", slice);
}

fn take_til(input: &str, til: char) -> Option<&str> {
    let mut chs = input.char_indices();
    // let mut start = 0;
    // let mut output = Vec<&str>;

    for (indx, ch) in chs {
        match (indx, ch) {
            (0, ch) if ch == til => return None,
            (indx, ch) if ch == til => return Some(&input[..indx]),
            _ => continue,
        }
    }
    panic!("Input ended before finding terminating char '{}'.", til);
}
