package scanner

import "fmt"

type Scanner struct {
	source  *string
	tokens  []Token
	start   uint
	current uint
	line    uint
}

func NewScanner(source *string) Scanner {
	return Scanner{source: source, tokens: []Token{}, start: 0, current: 0, line: 1}
}

func (s *Scanner) isEOF() bool {
	return s.current >= uint(len(*s.source))
}

func (s *Scanner) advanceCursor(ammount uint) byte {
	char := (*s.source)[s.current]
	s.current += ammount
	return char
}

func (s *Scanner) matchesChar(expected byte) bool {
	if s.isEOF() || (*s.source)[s.current] != expected {
		return false
	}

	s.current++
	return true
}

func (s *Scanner) peekChar(offset uint) byte {
	if s.current+offset >= uint(len(*s.source)) {
		return 0
	}

	return (*s.source)[s.current+offset-1]
}

func (s *Scanner) addToken(tokenType uint, literal interface{}) {
	lexeme := (*s.source)[s.start:s.current]
	token := Token{Type: tokenType, Lexeme: lexeme, Literal: literal, Line: s.line}
	s.tokens = append(s.tokens, token)

}

func (s *Scanner) isNumeric(char byte) bool {
	return char >= byte('0') && char <= byte('9')
}

func (s *Scanner) isAlpha(char byte) bool {
	return (char >= byte('A') && char <= byte('z')) || char == byte('_')
}

func (s *Scanner) isAlphaNumeric(char byte) bool {
	return s.isAlpha(char) || s.isNumeric(char)
}

func (s *Scanner) scanLiteralString() error {
	stringDelimiter := s.peekChar(0)
	for !s.isEOF() && (s.peekChar(1) != stringDelimiter || s.peekChar(0) == '\\') && s.peekChar(1) != '\n' {
		s.advanceCursor(1)
	}

	if s.isEOF() || s.peekChar(1) == '\n' {
		return fmt.Errorf("unterminated string on line %d", s.line)
	}

	s.advanceCursor(1) // consume delimiter itself

	literal := (*s.source)[s.start+1 : s.current-1]
	s.addToken(LITERAL_STRING, literal)

	return nil
}

func (s *Scanner) scanMultilineLiteralString() error {
	for !s.isEOF() && s.peekChar(0) != ']' && s.peekChar(1) != ']' {
		if s.peekChar(0) == '\n' {
			s.line++
		}
		s.advanceCursor(1)
	}

	if s.isEOF() {
		return fmt.Errorf("unterminated string on line %d", s.line)
	}

	if s.peekChar(0) == '\n' {
		s.line++
		s.advanceCursor(1)
	}

	s.advanceCursor(1) // consume ], second ] will be consumed on next iteration

	literal := (*s.source)[s.start+2 : s.current-2]
	s.addToken(LITERAL_STRING, literal)

	return nil
}

func (s *Scanner) scanNumeral() {
	for s.isNumeric(s.peekChar(1)) {
		s.advanceCursor(1)
	}

	// TODO: handle hex - 0xff 0xBEBADA
	// TODO: handle powers - 314.16e-2 0.31416E1 34e1 0x0.1E  0xA23p-4 0X1.921FB54442D18P+1

	if s.peekChar(1) == '.' && s.isNumeric(s.peekChar(2)) {
		s.advanceCursor(1) // consume .

		for s.isNumeric(s.peekChar(1)) {
			s.advanceCursor(1)
		}
	}

	s.addToken(NUMERAL, (*s.source)[s.start:s.current])
}

func (s *Scanner) scanIdentifier() {
	for s.isAlphaNumeric(s.peekChar(1)) {
		s.advanceCursor(1)
	}

	value := (*s.source)[s.start:s.current]
	switch value {
	case "end":
		s.addToken(END, nil)
	case "function":
		s.addToken(FUNCTION, nil)
	case "nil":
		s.addToken(NIL, nil)
	case "false":
		s.addToken(FALSE, nil)
	case "true":
		s.addToken(TRUE, nil)
	case "return":
		s.addToken(RETURN, nil)
	case "local":
		s.addToken(LOCAL, nil)
	case "for":
		s.addToken(FOR, nil)
	case "do":
		s.addToken(DO, nil)
	case "in":
		s.addToken(IN, nil)
	case "if":
		s.addToken(IF, nil)
	case "else":
		if s.peekChar(2) == 'i' && s.peekChar(3) == 'f' {
			s.addToken(ELSEIF, nil)
		} else {
			s.addToken(ELSE, nil)
		}
	case "then":
		s.addToken(THEN, nil)
	case "repeat":
		s.addToken(REPEAT, nil)
	case "until":
		s.addToken(UNTIL, nil)
	case "while":
		s.addToken(WHILE, nil)
	case "goto":
		s.addToken(GOTO, nil)
	case "break":
		s.addToken(BREAK, nil)
	default:
		s.addToken(IDENTIFIER, nil)
	}
}

func (s *Scanner) ScanTokens() ([]Token, error) {
	for !s.isEOF() {
		s.start = s.current

		char := s.advanceCursor(1)
		switch char {
		case '.':
			if s.matchesChar('.') {
				if s.matchesChar('.') {
					s.addToken(SPREAD, nil)
				} else {
					s.addToken(DOT_DOT, nil)
				}
			} else {
				s.addToken(DOT, nil)
			}
		case ',':
			s.addToken(COMMA, nil)
		case ':':
			s.addToken(COLON, nil)
		case ';':

			s.addToken(SEMICOLON, nil)
		case '=':
			if s.matchesChar('=') {
				s.addToken(EQUAL_EQUAL, nil)
			} else {
				s.addToken(EQUAL, nil)
			}
		case '<':
			if s.matchesChar('=') {
				s.addToken(LESS_EQUAL, nil)
			} else if s.matchesChar('<') {
				s.addToken(LESS_LESS, nil)
			} else {
				s.addToken(LESS, nil)
			}
		case '>':
			if s.matchesChar('=') {
				s.addToken(GREATER_EQUAL, nil)
			} else if s.matchesChar('>') {
				s.addToken(GREATER_EQUAL, nil)
			} else {
				s.addToken(GREATER, nil)
			}

		case '+':
			s.addToken(PLUS, nil)
		case '-':
			if s.matchesChar('-') {
				if s.peekChar(1) == '[' && s.peekChar(2) == '[' {
					s.advanceCursor(3) // consume -[[

					for !s.isEOF() && s.peekChar(0) != ']' && s.peekChar(1) != ']' {
						if s.peekChar(0) == '\n' {
							s.line++
						}
						s.advanceCursor(1)
					}

					if s.peekChar(0) == '\n' {
						s.line++
						s.advanceCursor(1)
					}

					s.advanceCursor(1) // consume ], second ] will be consumed on next iteration
				} else {
					for !s.isEOF() && s.peekChar(1) != '\n' {
						s.advanceCursor(1)
					}
				}
			} else {
				s.addToken(MINUS, nil)
			}
		case '*':
			s.addToken(STAR, nil)
		case '^':
			s.addToken(CARET, nil)
		case '%':
			s.addToken(PRECENT, nil)
		case '&':
			s.addToken(AMPERSAND, nil)
		case '~':
			if s.matchesChar('=') {
				s.addToken(TILDE_EQUAL, nil)
			} else {
				s.addToken(TILDE, nil)
			}
		case '|':
			s.addToken(PIPE, nil)
		case '#':
			s.addToken(HASH, nil)

		case '[':
			if s.matchesChar('[') {
				if err := s.scanMultilineLiteralString(); err != nil {
					return nil, err
				}
			} else {
				s.addToken(LEFT_BRACE, nil)
			}
		case ']':
			s.addToken(RIGHT_BRACE, nil)
		case '(':
			s.addToken(LEFT_PAREN, nil)
		case ')':
			s.addToken(RIGHT_PAREN, nil)
		case '{':
			s.addToken(LEFT_BRACKET, nil)
		case '}':
			s.addToken(RIGHT_BRACKET, nil)

		case '\'':
			fallthrough
		case '"':
			if err := s.scanLiteralString(); err != nil {
				return nil, err
			}

		case ' ':
			fallthrough
		case '\r':
			fallthrough
		case '\t':
			// noop
		case '\n':
			s.line++
		default:
			if s.isNumeric(char) {
				s.scanNumeral()
			} else if s.isAlpha(char) {
				s.scanIdentifier()
			} else {
				return nil, fmt.Errorf("unexpected character at line %d", s.line)
			}
		}
	}

	return s.tokens, nil
}

func (s *Scanner) DebugString() string {
	debugString := ""
	for _, token := range s.tokens {
		debugString += fmt.Sprintf("token t=%d lex=`%v` lit=`%v` line=%d \n", token.Type, token.Lexeme, token.Literal, token.Line)
	}
	return debugString
}
