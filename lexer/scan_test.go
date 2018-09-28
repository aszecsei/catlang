package lexer_test

import (
	. "github.com/onsi/ginkgo"
	. "github.com/onsi/gomega"

	. "github.com/aszecsei/catlang/lexer"
)

var _ = Describe("Scan", func() {
	var (
		scanner *Scanner
	)

	Context("when scanning numbers", func() {
		BeforeEach(func() {
			scanner = &Scanner{}
			scanner.Init(NewFile("numbers.cc", 0, 100), "0 1 2 3 4 5 6 7 8 9 10")
		})

		It("should construct a list of number tokens", func() {
			results := []TokenType{
				INTEGER,
				INTEGER,
				INTEGER,
				INTEGER,
				INTEGER,
				INTEGER,
				INTEGER,
				INTEGER,
				INTEGER,
				INTEGER,
				INTEGER,
				EOF,
			}
			for _, res := range results {
				scanner.Advance()
				Expect(scanner.NextLexeme().Type).To(Equal(res))
			}
		})

		It("should have correct literal values for each number token", func() {
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
				Expect(scanner.NextLexeme().Literal).To(Equal(res))
			}
		})
	})

	Context("when scanning strings and chars", func() {
		BeforeEach(func() {
			scanner = &Scanner{}
			scanner.Init(NewFile("text.cc", 0, 100), "'a' \"hello, world\"")
		})

		It("should construct a list of char and string tokens", func() {
			results := []TokenType{
				CHAR,
				STRING,
				EOF,
			}
			for _, res := range results {
				scanner.Advance()
				Expect(scanner.NextLexeme().Type).To(Equal(res))
			}
		})

		It("should have correct literal values for each token", func() {
			results := []string{
				"a",
				"hello, world",
			}
			for _, res := range results {
				scanner.Advance()
				Expect(scanner.NextLexeme().Literal).To(Equal(res))
			}
		})
	})

	Context("when scanning boolean literals", func() {
		BeforeEach(func() {
			scanner = &Scanner{}
			scanner.Init(NewFile("bools.cc", 0, 100), "true false")
		})

		It("should construct a list of boolean tokens", func() {
			results := []TokenType{
				BOOL,
				BOOL,
				EOF,
			}
			for _, res := range results {
				scanner.Advance()
				Expect(scanner.NextLexeme().Type).To(Equal(res))
			}
		})

		It("should have correct literal values for each token", func() {
			results := []string{
				"true",
				"false",
			}
			for _, res := range results {
				scanner.Advance()
				Expect(scanner.NextLexeme().Literal).To(Equal(res))
			}
		})
	})

	Context("when scanning keywords and identifiers", func() {
		BeforeEach(func() {
			scanner = &Scanner{}
			scanner.Init(NewFile("keywords.cc", 0, 100), "for let x plus mul")
		})

		It("should construct a list of keyword and identifier tokens", func() {
			results := []TokenType{
				FOR,
				LET,
				IDENT,
				IDENT,
				IDENT,
				EOF,
			}
			for _, res := range results {
				scanner.Advance()
				Expect(scanner.NextLexeme().Type).To(Equal(res))
			}
		})

		It("should have correct literal values for each token", func() {
			results := []string{
				"for",
				"let",
				"x",
				"plus",
				"mul",
			}
			for _, res := range results {
				scanner.Advance()
				Expect(scanner.NextLexeme().Literal).To(Equal(res))
			}
		})
	})

	Context("when scanning a combination of tokens", func() {
		BeforeEach(func() {
			scanner = &Scanner{}
			scanner.Init(NewFile("test.cc", 0, 100), "function timesTwo(num: int) -> int {\nreturn num * 2;\n}")
		})

		It("should construct a list of tokens of the correct types", func() {
			results := []TokenType{
				FUNCTION,
				IDENT,
				LPAREN,
				IDENT,
				COLON,
				IDENT,
				RPAREN,
				ARROW,
				IDENT,
				LCURLYB,
				RETURN,
				IDENT,
				MUL,
				INTEGER,
				SEMICOLON,
				RCURLYB,
				EOF,
			}
			for _, res := range results {
				scanner.Advance()
				Expect(scanner.NextLexeme().Type).To(Equal(res))
			}
		})

		It("should have correct literal values for each token", func() {
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
				Expect(scanner.NextLexeme().Literal).To(Equal(res))
			}
		})
	})
})
