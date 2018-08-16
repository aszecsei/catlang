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
	s.skipWhitespace()             // Skip whitespace
	// TODO: Skip comments
	if unicode.IsDigit(s.ch) {
		s.scanNumber()
	}
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

func (s *Scanner) selectToken(r rune, a, b TokenType) TokenType {
	if s.ch == r {
		s.next()
		return a
	}
	return b
}

func (s *Scanner) skipWhitespace() {
	for unicode.IsSpace(s.ch) {
		s.next()
	}
}

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
