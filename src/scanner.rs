use crate::token::{Token, TokenType};
use std::fmt;

pub struct Scanner<'s> {
    source: &'s str,
    tokens: Vec<Token<'s>>,
    start: usize,
    current: usize,
    line: usize,
}

impl<'s> fmt::Debug for Scanner<'s> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Scanner")
            .field("tokens", &self.tokens)
            .finish()
    }
}

impl<'s> Scanner<'s> {
    pub fn new(source: &str) -> Scanner {
        Scanner {
            source,
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1,
        }
    }

    fn is_eof(&self) -> bool {
        self.current >= self.source.chars().count()
    }

    fn is_numeric(&self, char: char) -> bool {
        char >= '0' && char <= '9'
    }

    fn is_alpha(&self, char: char) -> bool {
        (char >= 'A' && char <= 'z') || char == '_'
    }

    fn is_alphanumeric(&self, char: char) -> bool {
        self.is_numeric(char) || self.is_alpha(char)
    }

    fn char_at(&self, index: usize) -> Option<char> {
        self.source.chars().nth(index)
    }

    fn advance_cursor(&mut self, offset: usize) -> Option<char> {
        let char = self.char_at(self.current);
        self.current += offset;
        char
    }

    fn consume_matching(&mut self, expected: char) -> bool {
        if self.is_eof() || self.char_at(self.current) != Some(expected) {
            return false;
        }

        self.current += 1;
        return true;
    }

    fn consume_comment(&mut self) {
        if self.char_at(self.current + 1).unwrap_or('\0') == '['
            && self.char_at(self.current + 2).unwrap_or('\0') == '['
        {
            self.advance_cursor(3); // consume -[[

            while !self.is_eof()
                && self.char_at(self.current) != Some(']')
                && self.char_at(self.current + 1) != Some(']')
            {
                if self.char_at(self.current) == Some('\n') {
                    self.line += 1;
                }

                self.advance_cursor(1);
            }

            if self.char_at(self.current) == Some('\n') {
                self.line += 1;
                self.advance_cursor(1);
            }

            self.advance_cursor(1); // consume ], second ] will be consumed on next iteration
        } else {
            while !self.is_eof() && self.char_at(self.current + 1) != Some('\n') {
                self.advance_cursor(1);
            }
        }
    }

    fn add_token(&mut self, token_type: TokenType, literal: Option<&'s str>) {
        let lexeme = &self.source[self.start..self.current];
        let token = Token::new(token_type, lexeme, self.start, literal, self.line);
        self.tokens.push(token);
    }

    fn scan_literal_string(&mut self) -> Result<(), String> {
        match self.char_at(self.current - 1) {
            Some(delimiter) => {
                while !self.is_eof()
                    && (self.char_at(self.current) != Some(delimiter)
                        || self.char_at(self.current) == Some('\\'))
                    && self.char_at(self.current) != Some('\n')
                {
                    self.advance_cursor(1);
                }

                if self.is_eof() || self.char_at(self.current + 1) == Some('\n') {
                    return Err(format!("unterminated string on line: {}", self.line));
                }

                self.advance_cursor(1); // consume delimiter itself

                let literal = &self.source[self.start + 1..self.current - 1];
                self.add_token(TokenType::LiteralString, Some(literal));

                Ok(())
            }
            _ => Err(format!(
                "cannot detect literal string delimiter on line: {}",
                self.line
            )),
        }
    }

    fn scan_multiline_literal_string(&mut self) -> Result<(), String> {
        while !self.is_eof()
            && self.char_at(self.current) != Some(']')
            && self.char_at(self.current + 1) != Some(']')
        {
            if self.char_at(self.current) == Some('\n') {
                self.line += 1;
            }
            self.advance_cursor(1);
        }

        if self.is_eof() {
            return Err(format!("unterminated string on line: {}", self.line));
        }

        if self.char_at(self.current) == Some('\n') {
            self.line += 1;
            self.advance_cursor(1);
        }

        self.advance_cursor(1); // consume ], second ] will be consumed on next iteration

        let literal = &self.source[self.start + 2..self.current - 2];
        self.add_token(TokenType::LiteralString, Some(literal));

        Ok(())
    }

    fn scan_numeral(&mut self) {
        while self.is_numeric(self.char_at(self.current).unwrap_or('\0')) {
            self.advance_cursor(1);
        }

        // TODO: handle hex - 0xff 0xBEBADA
        // TODO: handle powers - 314.16e-2 0.31416E1 34e1 0x0.1E  0xA23p-4 0X1.921FB54442D18P+1

        if self.char_at(self.current + 1) == Some('.')
            && self.is_numeric(self.char_at(self.current + 2).unwrap_or('\0'))
        {
            self.advance_cursor(1);

            while self.is_numeric(self.char_at(self.current + 1).unwrap_or('\0')) {
                self.advance_cursor(1);
            }
        }

        let literal = &self.source[self.start..self.current];
        self.add_token(TokenType::Numeral, Some(literal));
    }

    fn scan_identifier(&mut self) {
        while self.is_alphanumeric(self.char_at(self.current).unwrap_or('\0')) {
            self.advance_cursor(1);
        }

        let literal = &self.source[self.start..self.current];
        match literal {
            "or" => self.add_token(TokenType::Or, None),
            "and" => self.add_token(TokenType::And, None),
            "end" => self.add_token(TokenType::End, None),
            "function" => self.add_token(TokenType::Function, None),
            "nil" => self.add_token(TokenType::Nil, None),
            "not" => self.add_token(TokenType::Not, None),
            "false" => self.add_token(TokenType::False, None),
            "true" => self.add_token(TokenType::True, None),
            "return" => self.add_token(TokenType::Return, None),
            "local" => self.add_token(TokenType::Local, None),
            "for" => self.add_token(TokenType::For, None),
            "do" => self.add_token(TokenType::Do, None),
            "in" => self.add_token(TokenType::In, None),
            "if" => {
                let elseif_replacement = match self.tokens.last() {
                    Some(token) if token.token_type == TokenType::Else => {
                        let lexeme = &self.source[token.lexeme_start..self.current];
                        let token = Token::new(
                            TokenType::Elseif,
                            lexeme,
                            token.lexeme_start,
                            None,
                            self.line,
                        );
                        Some(token)
                    }
                    _ => None,
                };

                match self.tokens.last_mut() {
                    Some(token) if token.token_type == TokenType::Else => {
                        if let Some(elseif_replacement) = elseif_replacement {
                            *token = elseif_replacement;
                        }
                    }
                    _ => {
                        self.add_token(TokenType::If, None);
                    }
                };
            }
            "else" => self.add_token(TokenType::Else, None),
            "then" => self.add_token(TokenType::Then, None),
            "repeat" => self.add_token(TokenType::Repeat, None),
            "until" => self.add_token(TokenType::Until, None),
            "while" => self.add_token(TokenType::While, None),
            "goto" => self.add_token(TokenType::Goto, None),
            "break" => self.add_token(TokenType::Break, None),
            _ => self.add_token(TokenType::Identifier, None),
        }
    }

    pub fn scan_tokens(&mut self) -> Result<&Vec<Token>, String> {
        while !self.is_eof() {
            self.start = self.current;

            if let Some(char) = self.advance_cursor(1) {
                match char {
                    '.' => {
                        if self.consume_matching('.') {
                            if self.consume_matching('.') {
                                self.add_token(TokenType::Spread, None);
                            } else {
                                self.add_token(TokenType::DotDot, None);
                            }
                        } else {
                            self.add_token(TokenType::Dot, None);
                        }
                    }
                    ',' => self.add_token(TokenType::Comma, None),
                    ':' => self.add_token(TokenType::Colon, None),
                    ';' => self.add_token(TokenType::Semicolon, None),
                    '=' => {
                        if self.consume_matching('=') {
                            self.add_token(TokenType::EqualEqual, None)
                        } else {
                            self.add_token(TokenType::Equal, None)
                        }
                    }
                    '<' => {
                        if self.consume_matching('=') {
                            self.add_token(TokenType::LessEqual, None)
                        } else if self.consume_matching('<') {
                            self.add_token(TokenType::LessLess, None)
                        } else {
                            self.add_token(TokenType::Less, None)
                        }
                    }
                    '>' => {
                        if self.consume_matching('=') {
                            self.add_token(TokenType::GreaterEqual, None)
                        } else if self.consume_matching('>') {
                            self.add_token(TokenType::GreaterGreater, None)
                        } else {
                            self.add_token(TokenType::Greater, None)
                        }
                    }
                    '+' => self.add_token(TokenType::Plus, None),
                    '-' => {
                        if self.consume_matching('-') {
                            self.consume_comment();
                        } else {
                            self.add_token(TokenType::Minus, None)
                        }
                    }
                    '*' => self.add_token(TokenType::Star, None),
                    '/' => {
                        if self.consume_matching('/') {
                            self.add_token(TokenType::SlashSlash, None);
                        } else {
                            self.add_token(TokenType::Slash, None);
                        }
                    }
                    '^' => self.add_token(TokenType::Caret, None),
                    '%' => self.add_token(TokenType::Percent, None),
                    '&' => self.add_token(TokenType::Ampersand, None),
                    '~' => {
                        if self.consume_matching('=') {
                            self.add_token(TokenType::TildeEqual, None);
                        } else {
                            self.add_token(TokenType::Tilde, None);
                        }
                    }
                    '|' => self.add_token(TokenType::Pipe, None),
                    '#' => self.add_token(TokenType::Hash, None),
                    '[' => {
                        if self.consume_matching('[') {
                            if let Err(error) = self.scan_multiline_literal_string() {
                                return Err(error);
                            }
                        } else {
                            self.add_token(TokenType::LeftBrace, None);
                        }
                    }
                    ']' => self.add_token(TokenType::RightBrace, None),
                    '(' => self.add_token(TokenType::LeftParen, None),
                    ')' => self.add_token(TokenType::RightParen, None),
                    '{' => self.add_token(TokenType::LeftBracket, None),
                    '}' => self.add_token(TokenType::RightBracket, None),
                    '\'' | '"' => {
                        if let Err(error) = self.scan_literal_string() {
                            return Err(error);
                        }
                    }
                    ' ' | '\r' | '\t' => {
                        // Noop
                    }
                    '\n' => {
                        self.line += 1;
                    }
                    char if self.is_numeric(char) => self.scan_numeral(),
                    char if self.is_alpha(char) => self.scan_identifier(),
                    _ => {
                        return Err(format!(
                            "unexpected character `{}` on line: {}",
                            char, self.line
                        ));
                    }
                }
            }
        }

        self.tokens.push(Token::new(
            TokenType::Eof,
            "",
            self.current,
            None,
            self.line,
        ));

        Ok(&self.tokens)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_scan_unary_expressions() {
        assert_eq!(
            Scanner::new("-1").scan_tokens(),
            Ok(&vec![
                Token::new(TokenType::Minus, "-", 0, None, 1),
                Token::new(TokenType::Numeral, "1", 1, Some("1"), 1),
                Token::new(TokenType::Eof, "", 2, None, 1),
            ])
        );

        assert_eq!(
            Scanner::new("not true").scan_tokens(),
            Ok(&vec![
                Token::new(TokenType::Not, "not", 0, None, 1),
                Token::new(TokenType::True, "true", 4, None, 1),
                Token::new(TokenType::Eof, "", 8, None, 1),
            ])
        );

        assert_eq!(
            Scanner::new("#some_array").scan_tokens(),
            Ok(&vec![
                Token::new(TokenType::Hash, "#", 0, None, 1),
                Token::new(TokenType::Identifier, "some_array", 1, None, 1),
                Token::new(TokenType::Eof, "", 11, None, 1),
            ])
        );

        assert_eq!(
            Scanner::new("~value").scan_tokens(),
            Ok(&vec![
                Token::new(TokenType::Tilde, "~", 0, None, 1),
                Token::new(TokenType::Identifier, "value", 1, None, 1),
                Token::new(TokenType::Eof, "", 6, None, 1),
            ])
        );
    }

    #[test]
    fn should_scan_binary_expressions() {
        assert_eq!(
            Scanner::new("3 * 2").scan_tokens(),
            Ok(&vec![
                Token::new(TokenType::Numeral, "3", 0, Some("3"), 1),
                Token::new(TokenType::Star, "*", 2, None, 1),
                Token::new(TokenType::Numeral, "2", 4, Some("2"), 1),
                Token::new(TokenType::Eof, "", 5, None, 1),
            ])
        );

        assert_eq!(
            Scanner::new("3 ^ 2").scan_tokens(),
            Ok(&vec![
                Token::new(TokenType::Numeral, "3", 0, Some("3"), 1),
                Token::new(TokenType::Caret, "^", 2, None, 1),
                Token::new(TokenType::Numeral, "2", 4, Some("2"), 1),
                Token::new(TokenType::Eof, "", 5, None, 1),
            ])
        );

        assert_eq!(
            Scanner::new("3 / 2").scan_tokens(),
            Ok(&vec![
                Token::new(TokenType::Numeral, "3", 0, Some("3"), 1),
                Token::new(TokenType::Slash, "/", 2, None, 1),
                Token::new(TokenType::Numeral, "2", 4, Some("2"), 1),
                Token::new(TokenType::Eof, "", 5, None, 1),
            ])
        );

        assert_eq!(
            Scanner::new("3 // 2").scan_tokens(),
            Ok(&vec![
                Token::new(TokenType::Numeral, "3", 0, Some("3"), 1),
                Token::new(TokenType::SlashSlash, "//", 2, None, 1),
                Token::new(TokenType::Numeral, "2", 5, Some("2"), 1),
                Token::new(TokenType::Eof, "", 6, None, 1),
            ])
        );
    }

    #[test]
    fn should_scan_comparisons() {
        assert_eq!(
            Scanner::new("5 >= 5").scan_tokens(),
            Ok(&vec![
                Token::new(TokenType::Numeral, "5", 0, Some("5"), 1),
                Token::new(TokenType::GreaterEqual, ">=", 2, None, 1),
                Token::new(TokenType::Numeral, "5", 5, Some("5"), 1),
                Token::new(TokenType::Eof, "", 6, None, 1),
            ])
        );

        assert_eq!(
            Scanner::new("5 <= 5").scan_tokens(),
            Ok(&vec![
                Token::new(TokenType::Numeral, "5", 0, Some("5"), 1),
                Token::new(TokenType::LessEqual, "<=", 2, None, 1),
                Token::new(TokenType::Numeral, "5", 5, Some("5"), 1),
                Token::new(TokenType::Eof, "", 6, None, 1),
            ])
        );

        assert_eq!(
            Scanner::new("11 < 10").scan_tokens(),
            Ok(&vec![
                Token::new(TokenType::Numeral, "11", 0, Some("11"), 1),
                Token::new(TokenType::Less, "<", 3, None, 1),
                Token::new(TokenType::Numeral, "10", 5, Some("10"), 1),
                Token::new(TokenType::Eof, "", 7, None, 1),
            ])
        );
    }

    #[test]
    fn should_scan_concatenation() {
        assert_eq!(
            Scanner::new("'hello ' .. 'world'").scan_tokens(),
            Ok(&vec![
                Token::new(TokenType::LiteralString, "'hello '", 0, Some("hello "), 1),
                Token::new(TokenType::DotDot, "..", 9, None, 1),
                Token::new(TokenType::LiteralString, "'world'", 12, Some("world"), 1),
                Token::new(TokenType::Eof, "", 19, None, 1)
            ])
        );

        assert_eq!(
            Scanner::new("\"hello \" .. 'world'").scan_tokens(),
            Ok(&vec![
                Token::new(TokenType::LiteralString, "\"hello \"", 0, Some("hello "), 1),
                Token::new(TokenType::DotDot, "..", 9, None, 1),
                Token::new(TokenType::LiteralString, "'world'", 12, Some("world"), 1),
                Token::new(TokenType::Eof, "", 19, None, 1)
            ])
        );
    }
}
