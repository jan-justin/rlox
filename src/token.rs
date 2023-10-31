use anyhow::Result;
use phf::{phf_map, Map};

#[derive(Clone)]
pub enum Token {
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Slash,
    Star,

    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    Identifier,
    String { literal: String },
    Number { literal: f64 },

    And,
    Class,
    Else,
    False,
    Fun,
    For,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,

    Eof { line: usize },
}

static KEYWORDS: Map<&'static str, Token> = phf_map! {
    "and" => Token::And,
    "class" => Token::Class,
    "else" => Token::Else,
    "false" => Token::False,
    "for" => Token::For,
    "fun" => Token::Fun,
    "if" => Token::If,
    "nil" => Token::Nil,
    "or" => Token::Or,
    "print" => Token::Print,
    "return" => Token::Return,
    "super" => Token::Super,
    "this" => Token::This,
    "true" => Token::True,
    "var" => Token::Var,
    "while" => Token::While,
};

pub type LexError = (usize, usize, &'static str);

pub fn scan(source: &str) -> Result<Vec<Token>, Vec<LexError>> {
    let mut line = 0;
    let mut tokens = Vec::new();
    let mut errors = Vec::new();
    let mut chars = source.chars().enumerate().peekable();
    while let Some((location, char)) = chars.next() {
        let next_char = chars.next_if(|(_, c)| {
            matches!(
                (char, c),
                ('!', '=') | ('=', '=') | ('<', '=') | ('>', '=') | ('/', '/')
            )
        });
        match (char, next_char) {
            ('\n', _) => line += 1,
            (' ', _) | ('\r', _) | ('\t', _) => {}
            ('.', _) => tokens.push(Token::Dot),
            (',', _) => tokens.push(Token::Comma),
            ('(', _) => tokens.push(Token::LeftParen),
            (')', _) => tokens.push(Token::RightParen),
            ('{', _) => tokens.push(Token::LeftBrace),
            ('}', _) => tokens.push(Token::RightBrace),
            (';', _) => tokens.push(Token::Semicolon),
            ('-', _) => tokens.push(Token::Minus),
            ('+', _) => tokens.push(Token::Plus),
            ('*', _) => tokens.push(Token::Star),
            ('!', None) => tokens.push(Token::Bang),
            ('=', None) => tokens.push(Token::Equal),
            ('<', None) => tokens.push(Token::Less),
            ('>', None) => tokens.push(Token::Greater),
            ('/', None) => tokens.push(Token::Slash),
            ('!', Some((_, '='))) => tokens.push(Token::BangEqual),
            ('=', Some((_, '='))) => tokens.push(Token::EqualEqual),
            ('<', Some((_, '='))) => tokens.push(Token::LessEqual),
            ('>', Some((_, '='))) => tokens.push(Token::GreaterEqual),
            ('/', Some((_, '/'))) => while chars.next_if(|(_, c)| *c != '\n').is_some() {},
            ('"', _) => {
                let mut literal = String::new();
                while let Some((_, c)) = chars.next_if(|(_, c)| *c != '"') {
                    if c == '\n' {
                        line += 1
                    }
                    literal.push(c);
                }
                if chars.next().is_none() {
                    errors.push((line, location, "Unterminated string."))
                } else {
                    tokens.push(Token::String { literal })
                }
            }
            (c, _) if c.is_ascii_digit() => {
                let mut number_string = String::new();
                while let Some((_, c)) = chars.next_if(|(_, c)| c.is_ascii_digit()) {
                    number_string.push(c);
                }
                if let Some((_, dot)) = chars.next_if(|(_, c)| *c == '.') {
                    number_string.push(dot);
                    if chars.peek().is_some_and(|(_, c)| c.is_ascii_digit()) {
                        while let Some((_, c)) = chars.next_if(|(_, c)| c.is_ascii_digit()) {
                            number_string.push(c);
                        }
                        match number_string.parse::<f64>() {
                            Ok(literal) => tokens.push(Token::Number { literal }),
                            Err(_) => errors.push((line, location, "Invalid number.")),
                        };
                    } else {
                        tokens.push(Token::Dot);
                    }
                }
            }
            (c, _) if c.is_ascii_alphabetic() || c == '_' => {
                let mut identifier = String::new();
                while let Some((_, c)) =
                    chars.next_if(|(_, c)| c.is_ascii_alphanumeric() || *c == '_')
                {
                    identifier.push(c);
                }
                match KEYWORDS.get(&identifier).cloned() {
                    Some(reserved_keyword) => tokens.push(reserved_keyword),
                    _ => tokens.push(Token::Identifier),
                }
            }
            _ => errors.push((line, location, "Unexpected character.")),
        }
    }
    tokens.push(Token::Eof { line });
    if !errors.is_empty() {
        Err(errors)
    } else {
        Ok(tokens)
    }
}
