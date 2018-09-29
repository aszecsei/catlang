package lexer

import (
	"unicode"
	"unicode/utf8"
)

type Lexeme struct {
	Type     TokenType
	Literal  string
	Position Pos
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
		s.nextLexeme.Literal = "EOF"
		s.nextLexeme.Type = EOF
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
	s.nextLexeme.Type = INTEGER
	s.nextLexeme.Literal = s.src[start:offset]
	s.nextLexeme.Position = s.file.Pos(start)
}

func (s *Scanner) scanPunct() {
	s.nextLexeme.Literal = string(s.ch)
	s.nextLexeme.Position = s.file.Pos(s.offset)
	switch s.ch {
	case '(':
		s.nextLexeme.Type = LPAREN
	case ')':
		s.nextLexeme.Type = RPAREN
	case '{':
		s.nextLexeme.Type = LCURLYB
	case '}':
		s.nextLexeme.Type = RCURLYB
	case '[':
		s.nextLexeme.Type = LSQUAREB
	case ']':
		s.nextLexeme.Type = RSQUAREB
	case ':':
		s.nextLexeme.Type = s.selectToken(':', DOUBLECOLON, COLON)
	case ';':
		s.nextLexeme.Type = SEMICOLON
	case ',':
		s.nextLexeme.Type = COMMA
	case '@':
		s.nextLexeme.Type = AT
	case '+':
		s.nextLexeme.Type = s.selectToken('=', ADDASSIGN, s.selectToken('+', INCREMENT, ADD))
	case '-':
		s.nextLexeme.Type = s.selectToken('=', SUBASSIGN, s.selectToken('-', DECREMENT, s.selectToken('>', ARROW, SUB)))
	case '*':
		s.nextLexeme.Type = s.selectToken('=', MULASSIGN, MUL)
	case '/':
		if s.peek() == '*' {
			s.skipComment(true)
			s.scan()
		} else if s.peek() == '/' {
			s.skipComment(false)
			s.scan()
		} else {
			s.nextLexeme.Type = s.selectToken('=', QUOASSIGN, QUO)
		}
	case '%':
		s.nextLexeme.Type = s.selectToken('=', REMASSIGN, REM)
	case '=':
		s.nextLexeme.Type = s.selectToken('=', EQL, ASSIGN)
	case '&':
		s.nextLexeme.Type = s.selectToken('&', AND, s.selectToken('=', BITANDASSIGN, BITAND))
	case '|':
		s.nextLexeme.Type = s.selectToken('|', OR, s.selectToken('=', BITORASSIGN, BITOR))
	case '^':
		s.nextLexeme.Type = s.selectToken('=', XORASSIGN, XOR)
	case '!':
		s.nextLexeme.Type = s.selectToken('=', NEQ, NOT)
	case '<':
		s.nextLexeme.Type = s.selectToken('=', LTE, s.selectToken('<', SHIFTL, LT))
	case '>':
		s.nextLexeme.Type = s.selectToken('=', GTE, s.selectToken('>', SHIFTR, GT))
	case '?':
		s.nextLexeme.Type = OPTIONAL
	case '.':
		s.nextLexeme.Type = s.selectToken('.', DOTDOT, DOT)
	case '\'':
		s.scanCharacterLiteral()
	case '"':
		s.scanStringLiteral()
	default:
		s.nextLexeme.Type = ILLEGAL
	}
	s.next()
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
	s.next()
	start := s.offset
	for s.ch != '\'' && s.ch != rune(0) {
		s.next()
	}
	offset := s.offset
	if s.ch == rune(0) {
		offset++
	} else {
		s.next()
	}
	s.nextLexeme.Type = CHAR
	s.nextLexeme.Literal = s.src[start:offset]
	s.nextLexeme.Position = s.file.Pos(start)
}

func (s *Scanner) scanStringLiteral() {
	s.next()
	start := s.offset
	for s.ch != '"' && s.ch != rune(0) {
		s.next()
	}
	offset := s.offset
	if s.ch == rune(0) {
		offset++
	}
	s.nextLexeme.Type = STRING
	s.nextLexeme.Literal = s.src[start:offset]
	s.nextLexeme.Position = s.file.Pos(start)
}

func (s *Scanner) selectToken(r rune, a, b TokenType) TokenType {
	if s.peek() == r {
		s.next()
		return a
	}
	return b
}

func (s *Scanner) scanIdentifier() {
	// Get the identifier
	start := s.offset
	for !unicode.IsSpace(s.ch) && !unicode.IsPunct(s.ch) && s.ch != rune(0) {
		s.next()
	}
	offset := s.offset
	if s.ch == rune(0) {
		offset++
	}
	s.nextLexeme.Literal = s.src[start:offset]
	s.nextLexeme.Position = s.file.Pos(start)
	s.nextLexeme.Type = Lookup(s.nextLexeme.Literal)
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
