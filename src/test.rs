<<<<<<< HEAD
use entry::Entry;
use field::Field;
// use parser::Parser;

#[test]
fn parser() {
}

#[test]
fn serialization() {
    let bib = include_str!("../test/single_sorted.bib");
    let mut fields = Vec::new();
    fields.push(Field::new("title", "The Historiographical {Jesus}: Memory, Typology, and the Son of {David}"));
    fields.push(Field::new("shorttitle", "The Historiographical Jesus"));
    fields.push(Field::new("author", "Le Donne, Anthony"));
    fields.push(Field::new("location", "Waco, TX"));
    fields.push(Field::new("publisher", "Baylor University Press"));
    fields.push(Field::new("year", "2009"));

    let mut entry = Entry::new("book", "le-donne2009", fields);
    
    assert_eq!(entry.to_string(), bib);
=======
#[test]
fn parse() {    
    assert_eq!(true, true);
>>>>>>> 550a13840f81e6e40319fedc1e2c3858a663c513
}

