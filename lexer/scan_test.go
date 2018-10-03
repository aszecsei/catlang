package lexer_test

import (
	"testing"

	"github.com/apoydence/onpar"
	. "github.com/apoydence/onpar/expect"
	. "github.com/apoydence/onpar/matchers"

	. "github.com/aszecsei/catlang/lexer"
	"github.com/aszecsei/catlang/token"
)

func TestSpecs(t *testing.T) {
	o := onpar.New()
	defer o.Run(t)

	o.BeforeEach(func(t *testing.T) (*testing.T, *Scanner) {
		return t, &Scanner{}
	})

	o.Group("when scanning numbers", func() {
		o.BeforeEach(func(t *testing.T, scanner *Scanner) (*testing.T, *Scanner) {
			scanner.Init(token.NewFile("numbers.cc", 0, 100), "0 1 2 3 4 5 6 7 8 9 10")
			return t, scanner
		})

		o.Spec("should construct a list of number tokens", func(t *testing.T, scanner *Scanner) {
			results := []token.TokenType{
				token.INTEGER,
				token.INTEGER,
				token.INTEGER,
				token.INTEGER,
				token.INTEGER,
				token.INTEGER,
				token.INTEGER,
				token.INTEGER,
				token.INTEGER,
				token.INTEGER,
				token.INTEGER,
				token.EOF,
			}
			for _, res := range results {
				scanner.Advance()
				Expect(t, scanner.NextLexeme().Type).To(Equal(res))
			}
		})

		o.Spec("should have correct literal values for each number token", func(t *testing.T, scanner *Scanner) {
			results := []string{
				"0",
				"1",
				"2",
				"3",
				"4",
				"5",
				"6",
				"7",
				"8",
				"9",
				"10",
			}
			for _, res := range results {
				scanner.Advance()
				Expect(t, scanner.NextLexeme().Literal).To(Equal(res))
			}
		})
	})

	o.Group("when scanning strings, comments, and chars", func() {
		o.BeforeEach(func(t *testing.T, scanner *Scanner) (*testing.T, *Scanner) {
			scanner.Init(token.NewFile("text.cc", 0, 100), "'a' /* I'm a comment */ // I'm another comment\n\"hello, world\"")
			return t, scanner
		})

		o.Spec("should construct a list of char and string tokens", func(t *testing.T, scanner *Scanner) {
			results := []token.TokenType{
				token.CHAR,
				token.STRING,
				token.EOF,
			}
			for _, res := range results {
				scanner.Advance()
				Expect(t, scanner.NextLexeme().Type).To(Equal(res))
			}
		})

		o.Spec("should have correct literal values for each token", func(t *testing.T, scanner *Scanner) {
			results := []string{
				"a",
				"hello, world",
			}
			for _, res := range results {
				scanner.Advance()
				Expect(t, scanner.NextLexeme().Literal).To(Equal(res))
			}
		})
	})

	o.Group("when scanning boolean literals", func() {
		o.BeforeEach(func(t *testing.T, scanner *Scanner) (*testing.T, *Scanner) {
			scanner.Init(token.NewFile("bools.cc", 0, 100), "true false")
			return t, scanner
		})

		o.Spec("should construct a list of boolean tokens", func(t *testing.T, scanner *Scanner) {
			results := []token.TokenType{
				token.BOOL,
				token.BOOL,
				token.EOF,
			}
			for _, res := range results {
				scanner.Advance()
				Expect(t, scanner.NextLexeme().Type).To(Equal(res))
			}
		})

		o.Spec("should have correct literal values for each token", func(t *testing.T, scanner *Scanner) {
			results := []string{
				"true",
				"false",
			}
			for _, res := range results {
				scanner.Advance()
				Expect(t, scanner.NextLexeme().Literal).To(Equal(res))
			}
		})
	})

	o.Group("when scanning keywords and identifiers", func() {
		o.BeforeEach(func(t *testing.T, scanner *Scanner) (*testing.T, *Scanner) {
			scanner.Init(token.NewFile("keywords.cc", 0, 100), "for let x plus mul")
			return t, scanner
		})

		o.Spec("should construct a list of keyword and identifier tokens", func(t *testing.T, scanner *Scanner) {
			results := []token.TokenType{
				token.FOR,
				token.LET,
				token.IDENT,
				token.IDENT,
				token.IDENT,
				token.EOF,
			}
			for _, res := range results {
				scanner.Advance()
				Expect(t, scanner.NextLexeme().Type).To(Equal(res))
			}
		})

		o.Spec("should have correct literal values for each token", func(t *testing.T, scanner *Scanner) {
			results := []string{
				"for",
				"let",
				"x",
				"plus",
				"mul",
			}
			for _, res := range results {
				scanner.Advance()
				Expect(t, scanner.NextLexeme().Literal).To(Equal(res))
			}
		})
	})

	o.Group("when scanning a combination of tokens", func() {
		o.BeforeEach(func(t *testing.T, scanner *Scanner) (*testing.T, *Scanner) {
			scanner.Init(token.NewFile("test.cc", 0, 100), "function timesTwo(num: int) -> int {\nreturn num * 2;\n}")
			return t, scanner
		})

		o.Spec("should construct a list of tokens of the correct types", func(t *testing.T, scanner *Scanner) {
			results := []token.TokenType{
				token.FUNCTION,
				token.IDENT,
				token.LPAREN,
				token.IDENT,
				token.COLON,
				token.IDENT,
				token.RPAREN,
				token.ARROW,
				token.IDENT,
				token.LCURLYB,
				token.RETURN,
				token.IDENT,
				token.MUL,
				token.INTEGER,
				token.SEMICOLON,
				token.RCURLYB,
				token.EOF,
			}
			for _, res := range results {
				scanner.Advance()
				Expect(t, scanner.NextLexeme().Type).To(Equal(res))
			}
		})

		o.Spec("should have correct literal values for each token", func(t *testing.T, scanner *Scanner) {
			results := []string{
				"function",
				"timesTwo",
				"(",
				"num",
				":",
				"int",
				")",
				"-",
				"int",
				"{",
				"return",
				"num",
				"*",
				"2",
				";",
				"}",
			}
			for _, res := range results {
				scanner.Advance()
				Expect(t, scanner.NextLexeme().Literal).To(Equal(res))
			}
		})
	})
}
