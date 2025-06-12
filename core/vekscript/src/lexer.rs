#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    // Palavras-chave
    Import,
    As,
    Object,
    Keyframe,
    Interpolate,
    Type,
    From,
    To,

    // Identificadores e Literais
    Identifier(String),       // identificador
    Number(f64),              // numeros para coisas como transformações, tempo, etc

    // Símbolos e pontuação
    LBrace,       // {
    RBrace,       // }
    LParen,       // (
    RParen,       // )
    LBracket,     // [
    RBracket,     // ]
    Colon,        // :
    Dot,          // .
    Comma,        // ,
    Arrow,        // ->

    // Outros
    Comment(String),
    Error(String),
    Newline,
    Eof,
}

pub fn tokenize(input: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut chars = input.chars().peekable();

    while let Some(c) = chars.next() {
        match c {
            '{' => tokens.push(Token::LBrace),
            '}' => tokens.push(Token::RBrace),
            '(' => tokens.push(Token::LParen),
            ')' => tokens.push(Token::RParen),
            '[' => tokens.push(Token::LBracket),
            ']' => tokens.push(Token::RBracket),
            ':' => tokens.push(Token::Colon),
            '.' => tokens.push(Token::Dot),
            ',' => tokens.push(Token::Comma),
            '-' if chars.peek() == Some(&'>') => {
                chars.next(); // Consume '>'
                tokens.push(Token::Arrow);
            }
            '/' if chars.peek() == Some(&'/') => {
                // It's a comment, consume until end of line
                let mut comment = String::new();
                while let Some(pc) = chars.peek() {
                    if *pc == '\n' {
                        break;
                    }
                    comment.push(chars.next().unwrap());
                }
                tokens.push(Token::Comment(comment.trim().to_string()));
            }
            c if c.is_alphabetic() => {
                let mut identifier = String::new();
                identifier.push(c);
                while let Some(&p) = chars.peek() {
                    if p.is_alphanumeric() || p == '_' || p == '-' {
                        identifier.push(chars.next().unwrap());
                    } else {
                        break;
                    }
                }
                match identifier.as_str() {
                    "import" => tokens.push(Token::Import),
                    "as" => tokens.push(Token::As),
                    "object" => tokens.push(Token::Object),
                    "keyframe" => tokens.push(Token::Keyframe),
                    "interpolate" => tokens.push(Token::Interpolate),
                    "type" => tokens.push(Token::Type),
                    "from" => tokens.push(Token::From),
                    "to" => tokens.push(Token::To),
                    _ => tokens.push(Token::Identifier(identifier)),
                }
            }
            c if c.is_digit(10) || (c == '-' && chars.peek().map_or(false, |c| c.is_digit(10))) => {
                let mut number_str = String::new();
                number_str.push(c);
                while let Some(&p) = chars.peek() {
                    if p.is_digit(10) || p == '.' {
                        number_str.push(chars.next().unwrap());
                    } else {
                        break;
                    }
                }
                match number_str.parse::<f64>() {
                    Ok(n) => tokens.push(Token::Number(n)),
                    Err(_) => tokens.push(Token::Error(format!("Invalid number: {}", number_str))),
                }
            }
            '\n' => tokens.push(Token::Newline),
            ' ' | '\t' | '\r' => { /* ignore whitespace */ }
            _ => tokens.push(Token::Error(format!("Unexpected character: {}", c))),
        }
    }

    tokens.push(Token::Eof);
    tokens
}

