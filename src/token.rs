#[derive(Debug, PartialEq, Eq)]
pub enum TokenType {
    Dot,       // .
    Comma,     // ,
    Colon,     // :
    Semicolon, // ;

    Equal,          // =
    EqualEqual,     // ==
    TildeEqual,     // ~=
    Less,           // <
    LessEqual,      // <=
    LessLess,       // <<
    Greater,        // >
    GreaterEqual,   // >=
    GreaterGreater, // >>

    Plus,       // +
    Minus,      // -
    Star,       // *
    Slash,      // /
    SlashSlash, // //
    Caret,      // ^
    Percent,    // %
    Ampersand,  // &
    Tilde,      // ~
    Pipe,       // |
    Hash,       // #

    LeftBracket,  // [
    RightBracket, // ]
    LeftParen,    // (
    RightParen,   // )
    LeftBrace,    // {
    RightBrace,   // }

    DotDot, // ..
    Spread, // ...

    Identifier,
    LiteralString,
    Numeral,

    Or,
    And,
    End,
    Function,
    Nil,
    Not,
    False,
    True,
    Return,
    Local,
    For,
    Do,
    In,
    If,
    Else,
    Elseif,
    Then,
    Repeat,
    Until,
    While,
    Goto,
    Break,

    Eof,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Token<'t> {
    pub token_type: TokenType,
    pub lexeme: &'t str,
    pub lexeme_start: usize,
    pub literal: Option<&'t str>,
    pub line: usize,
}

impl<'t> Token<'t> {
    pub fn new(
        token_type: TokenType,
        lexeme: &'t str,
        lexeme_start: usize,
        literal: Option<&'t str>,
        line: usize,
    ) -> Token<'t> {
        Token {
            token_type,
            lexeme,
            lexeme_start,
            literal,
            line,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_create_token() {
        let token = Token::new(TokenType::Function, "test", 0, None, 1);
        assert_eq!(token.token_type, TokenType::Function);
    }
}
