package lexer

import (
	"unicode"
	"unicode/utf8"

	"github.com/aszecsei/catlang/token"
)

type Lexeme struct {
	Type     token.TokenType
	Literal  string
	Position token.Pos
}

type Scanner struct {
	ch      rune        // The current character ("rune" for unicode)
	offset  int         // The offset of the current character
	roffset int         // The offset of the next character
	src     string      // The actual source text
	file    *token.File // A representation of the file, broken down into lines

	currentLexeme Lexeme
	nextLexeme    Lexeme
}

func (s *Scanner) Init(file *token.File, src string) {
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
		s.nextLexeme.Type = token.EOF
		s.nextLexeme.Position = s.file.Pos(s.offset)
		return
	}

	// Identifiers
	s.scanIdentifier()
	return
}

func (s *Scanner) scanNumber() {
	// TODO: Handle floats
	// TODO: Handle number bases other than 10
	start := s.offset
	for unicode.IsDigit(s.ch) {
		s.next()
	}
	offset := s.offset
	if s.ch == rune(0) {
		offset++
	}
	s.nextLexeme.Type = token.INTEGER
	s.nextLexeme.Literal = s.src[start:offset]
	s.nextLexeme.Position = s.file.Pos(start)
}

func (s *Scanner) scanPunct() {
	s.nextLexeme.Literal = string(s.ch)
	s.nextLexeme.Position = s.file.Pos(s.offset)
	switch s.ch {
	case '(':
		s.nextLexeme.Type = token.LPAREN
	case ')':
		s.nextLexeme.Type = token.RPAREN
	case '{':
		s.nextLexeme.Type = token.LCURLYB
	case '}':
		s.nextLexeme.Type = token.RCURLYB
	case '[':
		s.nextLexeme.Type = token.LSQUAREB
	case ']':
		s.nextLexeme.Type = token.RSQUAREB
	case ':':
		s.nextLexeme.Type = s.selectToken(':', token.DOUBLECOLON, token.COLON)
	case ';':
		s.nextLexeme.Type = token.SEMICOLON
	case ',':
		s.nextLexeme.Type = token.COMMA
	case '@':
		s.nextLexeme.Type = token.AT
	case '+':
		s.nextLexeme.Type = s.selectToken('=', token.ADDASSIGN, s.selectToken('+', token.INCREMENT, token.ADD))
	case '-':
		s.nextLexeme.Type = s.selectToken('=', token.SUBASSIGN, s.selectToken('-', token.DECREMENT, s.selectToken('>', token.ARROW, token.SUB)))
	case '*':
		s.nextLexeme.Type = s.selectToken('=', token.MULASSIGN, token.MUL)
	case '/':
		if s.peek() == '*' {
			s.skipComment(true)
			s.scan()
		} else if s.peek() == '/' {
			s.skipComment(false)
			s.scan()
		} else {
			s.nextLexeme.Type = s.selectToken('=', token.QUOASSIGN, token.QUO)
		}
	case '%':
		s.nextLexeme.Type = s.selectToken('=', token.REMASSIGN, token.REM)
	case '=':
		s.nextLexeme.Type = s.selectToken('=', token.EQL, token.ASSIGN)
	case '&':
		s.nextLexeme.Type = s.selectToken('&', token.AND, s.selectToken('=', token.BITANDASSIGN, token.BITAND))
	case '|':
		s.nextLexeme.Type = s.selectToken('|', token.OR, s.selectToken('=', token.BITORASSIGN, token.BITOR))
	case '^':
		s.nextLexeme.Type = s.selectToken('=', token.XORASSIGN, token.XOR)
	case '!':
		s.nextLexeme.Type = s.selectToken('=', token.NEQ, token.NOT)
	case '<':
		s.nextLexeme.Type = s.selectToken('=', token.LTE, s.selectToken('<', token.SHIFTL, token.LT))
	case '>':
		s.nextLexeme.Type = s.selectToken('=', token.GTE, s.selectToken('>', token.SHIFTR, token.GT))
	case '?':
		s.nextLexeme.Type = token.OPTIONAL
	case '.':
		s.nextLexeme.Type = s.selectToken('.', token.DOTDOT, token.DOT)
	case '\'':
		s.scanCharacterLiteral()
	case '"':
		s.scanStringLiteral()
	default:
		s.nextLexeme.Type = token.ILLEGAL
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
	s.nextLexeme.Type = token.CHAR
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
	s.nextLexeme.Type = token.STRING
	s.nextLexeme.Literal = s.src[start:offset]
	s.nextLexeme.Position = s.file.Pos(start)
}

func (s *Scanner) selectToken(r rune, a, b token.TokenType) token.TokenType {
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
	s.nextLexeme.Type = token.Lookup(s.nextLexeme.Literal)
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
	var runeSize int
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
