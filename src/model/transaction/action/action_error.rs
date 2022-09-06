#[derive(Clone, Eq, PartialEq, Debug)]
pub struct ActionError {
    code: u32,
    text: String,
}

impl ActionError {
    pub fn new(code: u32, text: String) -> ActionError {
        ActionError { code, text }
    }

    pub fn code(&self) -> u32 {
        self.code
    }

    pub fn text(&self) -> String {
        self.text.clone()
    }
}
