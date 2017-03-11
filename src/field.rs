use std::cmp::Ordering;

#[derive(Eq)]
pub struct Field<'a> (&'a str, &'a str);

impl<'a> Field<'a> {
    pub fn new(key: &'a str, value: &'a str) -> Field<'a> {
        Field(key, value)
    }
    pub fn key(&self) -> &str {
        &self.0
    }
    pub fn value(&self) -> &str {
        &self.1
    }
}

impl<'a> Ord for Field<'a>  {
    fn cmp(&self, other: &Field) -> Ordering {
        self.0.cmp(&other.0)
    }
}

impl<'a> PartialOrd for Field<'a>  {
    fn partial_cmp(&self, other: &Field) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<'a> PartialEq for Field<'a> {
    fn eq(&self, other: &Field) -> bool {
        self.0 == other.0
    }
}