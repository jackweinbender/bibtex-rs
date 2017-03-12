use entry::Entry;
use field::Field;
use parser::Parser;

#[test]
fn parser() {
    let bib = include_str!("../test/single_sorted.bib");
    // let mut v = Vec::new();
    // v.push(Field::new("title", "The Historiographical {Jesus}: Memory, Typology, and the Son of {David}"));
    // v.push(Field::new("shorttitle", "The Historiographical Jesus"));
    // v.push(Field::new("author", "Le Donne, Anthony"));
    // v.push(Field::new("location", "Waco, TX"));
    // v.push(Field::new("publisher", "Baylor University Press"));
    // v.push(Field::new("year", "2009"));

    // let mut entry = Entry::new("book", "le-donne2009", v);
    
    // assert_eq!(entry, Parser::entry_from_str(bib));
}

#[test]
fn serialization() {
    let bib = include_str!("../test/single_sorted.bib");
    let mut v = Vec::new();
    v.push(Field::new("title", "The Historiographical {Jesus}: Memory, Typology, and the Son of {David}"));
    v.push(Field::new("shorttitle", "The Historiographical Jesus"));
    v.push(Field::new("author", "Le Donne, Anthony"));
    v.push(Field::new("location", "Waco, TX"));
    v.push(Field::new("publisher", "Baylor University Press"));
    v.push(Field::new("year", "2009"));

    let mut entry = Entry::new("book", "le-donne2009", v);
    
    assert_eq!(entry.to_string(), bib);
}

