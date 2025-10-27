use crate as rlox;

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

impl Token {
    fn try_from_keyword(keyword: &str) -> anyhow::Result<Self> {
        Ok(match keyword {
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
            _ => anyhow::bail!("invalid keyword: {}", keyword),
        })
    }
}

pub fn scan(source: &str) -> Result<Vec<Token>, Vec<rlox::LexError>> {
    let mut line = 0;
    let mut tokens = Vec::<Token>::new();
    let mut errors = Vec::<rlox::LexError>::new();
    let mut chars = source.chars().enumerate().peekable();

    while let Some((location, current_char)) = chars.next() {
        let next_char = chars.next_if(|(_, next_char)| {
            matches! {
                (current_char, next_char),
                ('!', '=') | ('=', '=') | ('<', '=') | ('>', '=') | ('/', '/')
            }
        });
        match (current_char, next_char) {
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
            ('/', Some((_, '/'))) => while chars.next_if(|(_, char)| *char != '\n').is_some() {},
            ('"', _) => {
                let mut literal = String::with_capacity(24);
                while let Some((_, char)) = chars.next_if(|(_, char)| *char != '"') {
                    if char == '\n' {
                        line += 1
                    }
                    literal.push(char);
                }
                if chars.next().is_none() {
                    let error = rlox::LexError::from((line, location, "unterminated string"));
                    errors.push(error)
                } else {
                    tokens.push(Token::String { literal })
                }
            }
            (char, _) if char.is_ascii_digit() => {
                let mut number_string = String::with_capacity(24);
                while let Some((_, char)) = chars.next_if(|(_, char)| char.is_ascii_digit()) {
                    number_string.push(char);
                }
                if let Some((_, dot)) = chars.next_if(|(_, char)| *char == '.') {
                    number_string.push(dot);
                    if chars.peek().is_some_and(|(_, char)| char.is_ascii_digit()) {
                        while let Some((_, char)) = chars.next_if(|(_, char)| char.is_ascii_digit())
                        {
                            number_string.push(char);
                        }
                        match number_string.parse::<f64>() {
                            Ok(literal) => tokens.push(Token::Number { literal }),
                            Err(_) => {
                                let error =
                                    rlox::LexError::from((line, location, "invalid number"));
                                errors.push(error)
                            }
                        };
                    } else {
                        tokens.push(Token::Dot);
                    }
                }
            }
            (char, _) if char.is_ascii_alphabetic() || char == '_' => {
                let mut identifier = String::new();
                while let Some((_, char)) =
                    chars.next_if(|(_, char)| char.is_ascii_alphanumeric() || *char == '_')
                {
                    identifier.push(char);
                }
                match Token::try_from_keyword(&identifier) {
                    Ok(reserved_keyword) => tokens.push(reserved_keyword),
                    Err(_) => tokens.push(Token::Identifier),
                }
            }
            _ => {
                let error = rlox::LexError::from((line, location, "unexpected character"));
                errors.push(error)
            }
        }
    }

    tokens.push(Token::Eof { line });

    if !errors.is_empty() {
        Err(errors)
    } else {
        Ok(tokens)
    }
}
