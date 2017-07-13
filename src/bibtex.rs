pub type BibtexBibliography = Vec<BibtexEntry>;

pub struct BibtexEntry {
    key: String,
    bibtex_type: String,
    attrs: Vec<EntryAttr>
}
pub struct BibtexString {
    key: String,
    value: String
}
pub struct BibtexPreamble{
    value: String
}
pub struct BibtexComment{
    value: String
}
struct EntryAttr {
    key: String,
    value: BibtexValue
}
enum BibtexValue {
    BibtexString,
    String
}