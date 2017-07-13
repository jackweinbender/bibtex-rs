named!(entry<&[u8], Vec<&[u8]> >, many0!(ws!(tag!("@"))));

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn parse_full_entries() {
        let input = "@entry{} @entry{}";
        let output = entry(input.as_bytes());
        let expected = "";

        println!("{:?}", output);

        assert!(output.unwrap(), expected);
    }
}