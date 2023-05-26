pub struct Rule {
    code: String,
}
impl Rule {
    pub fn to_rust(&self) -> String {
        self.code.clone()
    }
}
impl From<String> for Rule {
    fn from(value: String) -> Self {
        Self { code: value }
    }
}
