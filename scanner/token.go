package scanner

const (
	DOT       = iota // .
	COMMA            // ,
	COLON            // :
	SEMICOLON        // ;

	EQUAL           // =
	EQUAL_EQUAL     // ==
	TILDE_EQUAL     // ~=
	LESS            // <
	LESS_EQUAL      // <=
	LESS_LESS       // <<
	GREATER         // >
	GREATER_EQUAL   // >=
	GREATER_GREATER // >>

	PLUS        // +
	MINUS       // -
	STAR        // *
	SLASH       // /
	SLASH_SLASH // //
	CARET       // ^
	PRECENT     // %
	AMPERSAND   // &
	TILDE       // ~
	PIPE        // |
	HASH        // #

	LEFT_BRACKET  // [
	RIGHT_BRACKET // ]
	LEFT_PAREN    // (
	RIGHT_PAREN   // )
	LEFT_BRACE    // {
	RIGHT_BRACE   // }

	DOT_DOT // ..
	SPREAD  // ...

	IDENTIFIER
	LITERAL_STRING
	NUMERAL

	END
	FUNCTION
	NIL
	FALSE
	TRUE
	RETURN
	LOCAL
	FOR
	DO
	IN
	IF
	ELSE
	ELSEIF
	THEN
	REPEAT
	UNTIL
	WHILE
	GOTO
	BREAK

	EOF
)

type Token struct {
	Type    uint
	Lexeme  string
	Literal interface{}
	Line    uint
}
