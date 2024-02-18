pub trait ToString {
    fn to_string(&self) -> String;
}

pub trait IsLetter {
    fn is_letter(self) -> bool;
}

impl IsLetter for char {
    fn is_letter(self) -> bool {
        return self.is_alphabetic() || self == '_';
    }
}

impl ToString for &[char] {
    fn to_string(&self) -> String {
        let mut string = String::new();

        for ch in self.iter() {
            string.push(ch.clone());
        }

        string
    }

}
