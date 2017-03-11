use field::Field;

pub struct Entry<'a> {
    entry_type: &'a str,
    cite_key: &'a str,
    fields: Vec<Field<'a>>,
}

impl<'a> Entry<'a> {
    pub fn new(entry: &'a str, key: &'a str, fields: Vec<Field<'a>>) -> Entry<'a> {
        Entry {
            entry_type: entry,
            cite_key: key,
            fields: fields
        }
    }
    fn cite_key(&self) -> &str {
        &self.cite_key
    }
    fn entry_type(&self) -> &str {
        &self.entry_type
    }
    fn field(&self, field: &str) -> &str {
        let fields = &self.fields;
        let index = fields.iter().position(|x| 
            x.key() == field ).expect("No key found.");
        fields[index].value()
    }
    pub fn to_string(&mut self) -> String {
        &self.sort();
        let mut entry = String::new();

        entry.push_str("@");
        entry.push_str(&self.entry_type.to_string());
        entry.push_str("{");
        entry.push_str(&self.cite_key.to_string());
        
        if *&self.fields.is_empty() {
            entry.push_str("}}");
            return entry
        }

        for field in &self.fields {
            entry.push_str(",\n  ");
            entry.push_str(field.key());
            entry.push_str(" = {");
            entry.push_str(field.value());
            entry.push_str("}");
        }

        entry.push_str("\n}");
        entry
        
    }
    fn sort(&mut self) {
        &self.fields.sort();
    }
}