package lexer

import (
	"unicode"
	"unicode/utf8"
)

type Lexeme struct {
	tokenType TokenType
	literal   string
	Position  Pos
}

type Scanner struct {
	ch      rune   // The current character ("rune" for unicode)
	offset  int    // The offset of the current character
	roffset int    // The offset of the next character
	src     string // The actual source text
	file    *File  // A representation of the file, broken down into lines

	currentLexeme Lexeme
	nextLexeme    Lexeme
}

func (s *Scanner) Init(file *File, src string) {
	s.file = file
	s.offset, s.roffset = 0, 0
	s.src = src
	s.file.AddLine(s.offset)

	s.next()
}

func (s *Scanner) CurrentLexeme() Lexeme {
	return s.currentLexeme
}

func (s *Scanner) NextLexeme() Lexeme {
	return s.nextLexeme
}

func (s *Scanner) Advance() {
	s.currentLexeme = s.nextLexeme // slide the window forward
	s.scan()
}

func (s *Scanner) scan() {
	s.skipWhitespace() // Skip whitespace
	// TODO: Skip comments
	if unicode.IsDigit(s.ch) {
		s.scanNumber()
		return
	}

	// Check punctuation mark controls
	if unicode.IsPunct(s.ch) {
		s.scanPunct()
		return
	}

	// Handle EOF
	if s.ch == rune(0) {
		s.nextLexeme.literal = "EOF"
		s.nextLexeme.tokenType = EOF
		s.nextLexeme.Position = s.file.Pos(s.offset)
		return
	}

	// Identifiers
	s.scanIdentifier()
	return
}

func (s *Scanner) scanNumber() {
	// TODO: Handle floats
	// TODO: Handle overflow
	start := s.offset
	for unicode.IsDigit(s.ch) {
		s.next()
	}
	offset := s.offset
	if s.ch == rune(0) {
		offset++
	}
	s.nextLexeme.tokenType = INTEGER
	s.nextLexeme.literal = s.src[start:offset]
	s.nextLexeme.Position = s.file.Pos(start)
}

func (s *Scanner) scanPunct() {
	s.nextLexeme.literal = string(s.ch)
	s.nextLexeme.Position = s.file.Pos(s.offset)
	switch s.ch {
	case '(':
		s.nextLexeme.tokenType = LPAREN
	case ')':
		s.nextLexeme.tokenType = RPAREN
	case '{':
		s.nextLexeme.tokenType = LCURLYB
	case '}':
		s.nextLexeme.tokenType = RCURLYB
	case '[':
		s.nextLexeme.tokenType = LSQUAREB
	case ']':
		s.nextLexeme.tokenType = RSQUAREB
	case ':':
		s.nextLexeme.tokenType = s.selectToken(':', DOUBLECOLON, COLON)
	case ';':
		s.nextLexeme.tokenType = SEMICOLON
	case ',':
		s.nextLexeme.tokenType = COMMA
	case '@':
		s.nextLexeme.tokenType = AT
	case '+':
		s.nextLexeme.tokenType = s.selectToken('=', ADDASSIGN, ADD)
	case '-':
		s.nextLexeme.tokenType = s.selectToken('=', SUBASSIGN, s.selectToken('>', ARROW, SUB))
	case '*':
		s.nextLexeme.tokenType = s.selectToken('=', MULASSIGN, MUL)
	case '/':
		if s.peek() == '*' {
			s.skipComment(true)
			s.scan()
		} else if s.peek() == '/' {
			s.skipComment(false)
			s.scan()
		} else {
			s.nextLexeme.tokenType = s.selectToken('=', QUOASSIGN, QUO)
		}
	case '%':
		s.nextLexeme.tokenType = s.selectToken('=', REMASSIGN, REM)
	case '=':
		s.nextLexeme.tokenType = s.selectToken('=', EQL, ASSIGN)
	case '&':
		s.nextLexeme.tokenType = s.selectToken('&', AND, s.selectToken('=', BITANDASSIGN, BITAND))
	case '|':
		s.nextLexeme.tokenType = s.selectToken('|', OR, s.selectToken('=', BITORASSIGN, BITOR))
	case '!':
		s.nextLexeme.tokenType = s.selectToken('=', NEQ, NOT)
	case '<':
		s.nextLexeme.tokenType = s.selectToken('=', LTE, LT)
	case '>':
		s.nextLexeme.tokenType = s.selectToken('=', GTE, GT)
	case '?':
		s.nextLexeme.tokenType = OPTIONAL
	case '.':
		s.nextLexeme.tokenType = s.selectToken('.', DOTDOT, DOT)
	case '\'':
		s.scanCharacterLiteral()
	case '"':
		s.scanStringLiteral()
	default:
		s.nextLexeme.tokenType = ILLEGAL
	}
}

func (s *Scanner) skipComment(isBlock bool) {
	if isBlock {
		for s.ch != '*' || s.peek() != '/' {
			s.next()
		}
		s.next() // skip the *
		s.next() // skip the /
	} else {
		for s.ch != '\n' {
			s.next()
		}
		s.next() // skip the newline
	}
}

func (s *Scanner) scanCharacterLiteral() {

}

func (s *Scanner) scanStringLiteral() {

}

func (s *Scanner) selectToken(r rune, a, b TokenType) TokenType {
	if s.peek() == r {
		s.next()
		return a
	}
	return b
}

func (s *Scanner) scanIdentifier() {

}

func (s *Scanner) skipWhitespace() {
	for unicode.IsSpace(s.ch) {
		s.next()
	}
}

// next returns the next Unicode character in the source, and advances the scanner.
// It returns rune(0) if the scanner's position is at the last character of the source.
func (s *Scanner) next() {
	s.ch = rune(0)
	runeSize := 1
	if s.roffset < len(s.src) {
		s.offset = s.roffset
		s.ch, runeSize = utf8.DecodeRuneInString(s.src[s.offset:])
		if s.ch == '\n' {
			s.file.AddLine(s.offset)
		}
		s.roffset += runeSize
	}
}

// peek returns the next Unicode character in the source without advancing the scanner.
// It returns rune(0) if the scanner's position is at the last character of the source.
func (s *Scanner) peek() rune {
	ch := rune(0)
	if s.roffset < len(s.src) {
		ch, _ = utf8.DecodeRuneInString(s.src[s.roffset:])
	}
	return ch
}
