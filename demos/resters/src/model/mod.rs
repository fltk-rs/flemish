use json_tools::{TokenType,Span,BufferType,Buffer,Lexer};

#[derive(Clone)]
pub struct Model {
    pub method: u8,
    pub url: String,
    pub responce: String,
    pub status: String,
}

impl Model {
    pub fn default() -> Self {
        Self {
            method: 0,
            url: String::new(),
            responce: String::new(),
            status: String::new(),
        }
    }
}

pub fn fill_style_buffer(s: &str) -> String {
    let mut buffer = vec![b'A'; s.len()];
    for token in Lexer::new(s.bytes(), BufferType::Span) {
        let c = match token.kind {
            TokenType::CurlyOpen | TokenType::CurlyClose | TokenType::BracketOpen | TokenType::BracketClose | TokenType::Colon | TokenType::Comma | TokenType::Invalid => {
                'A'
            }
            TokenType::String => 'B',
            TokenType::BooleanTrue | TokenType::BooleanFalse | TokenType::Null => 'C',
            TokenType::Number => 'D',
        };
        if let Buffer::Span(Span { first, end }) = token.buf {
            let start = first as _;
            let last = end as _;
            buffer[start..last].copy_from_slice(c.to_string().repeat(last - start).as_bytes());
        }
    }
    String::from_utf8_lossy(&buffer).to_string()
}
