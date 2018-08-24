package lexer

import "testing"

func TestScanner_Advance(t *testing.T) {
	scanner := &Scanner{}
	scanner.Init(NewFile("numbers.cc", 0, 19), "0 1 2 3 4 5 6 7 8 9")
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
		EOF,
	}
	for i, res := range results {
		scanner.Advance()
		if scanner.NextLexeme().tokenType != res {
			t.Errorf("%d: Lexeme type was incorrect; wanted %s but got %s", i, res.String(), scanner.NextLexeme().tokenType.String())
		}
	}
}
