use crate::format::TokenExpected;

#[derive(Debug)]
pub struct FormatterContext {
    pub current_expected: Option<TokenExpected>,
    pub is_line_first_token: bool,
    pub text: String,
}

impl FormatterContext {
    pub fn new() -> Self {
        Self {
            current_expected: None,
            is_line_first_token: true,
            text: String::new(),
        }
    }

    pub fn reset_whitespace(&mut self) {
        while self.text.ends_with(' ') {
            self.text.pop();
        }
    }
}
