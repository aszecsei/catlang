package token_test

import (
	"testing"

	"github.com/apoydence/onpar"
	. "github.com/apoydence/onpar/expect"

	. "github.com/apoydence/onpar/matchers"

	"github.com/aszecsei/catlang/token"
)

func TestPosition(t *testing.T) {
	o := onpar.New()
	defer o.Run(t)

	o.Group("nameless files", func() {
		o.BeforeEach(func(t *testing.T) (*testing.T, token.Position) {
			f := token.NewFile("", 1, 15)
			f.AddLine(0)
			p := f.Position(token.Pos(1))
			return t, p
		})

		o.Spec("position should be correct", func(t *testing.T, p token.Position) {
			Expect(t, p.String()).To(Equal("1:1"))
		})
	})

	o.Group("named files", func() {
		o.BeforeEach(func(t *testing.T) (*testing.T, token.Position) {
			f := token.NewFile("test.cat", 1, 15)
			f.AddLine(0)
			p := f.Position(token.Pos(1))
			return t, p
		})

		o.Spec("position should be correct", func(t *testing.T, p token.Position) {
			Expect(t, p.String()).To(Equal("test.cat:1:1"))
		})
	})

	o.Group("various positions", func() {
		o.BeforeEach(func(t *testing.T) (*testing.T, *token.File) {
			var test_expr = "(2 + 3)\n(5 - 4)"
			f := token.NewFile("test.cat", 1, len(test_expr))
			f.AddLine(0)
			f.AddLine(6)

			return t, f
		})

		o.Spec("position 1", func(t *testing.T, f *token.File) {
			p := f.Position(token.Pos(1))
			Expect(t, p.Col).To(Equal(1))
			Expect(t, p.Row).To(Equal(1))
		})

		o.Spec("position 8", func(t *testing.T, f *token.File) {
			p := f.Position(token.Pos(8))
			Expect(t, p.Col).To(Equal(1))
			Expect(t, p.Row).To(Equal(2))
		})

		o.Spec("position 14", func(t *testing.T, f *token.File) {
			p := f.Position(token.Pos(14))
			Expect(t, p.Col).To(Equal(7))
			Expect(t, p.Row).To(Equal(2))
		})
	})
}
