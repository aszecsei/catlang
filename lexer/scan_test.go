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
})
