package ast_test

import (
	"testing"

	"github.com/apoydence/onpar"
	. "github.com/apoydence/onpar/expect"

	. "github.com/apoydence/onpar/matchers"

	. "github.com/aszecsei/catlang/ast"
	"github.com/aszecsei/catlang/token"
)

func TestAst(t *testing.T) {
	o := onpar.New()
	defer o.Run(t)

	o.Group("basic lit", func() {
		o.BeforeEach(func(t *testing.T) (*testing.T, *BasicLiteral) {
			return t, &BasicLiteral{
				LitPos: token.Pos(1),
				Kind:   token.INTEGER,
				Lit:    "24",
			}
		})

		o.Spec("has the correct pos", func(t *testing.T, b *BasicLiteral) {
			Expect(t, b.Pos()).To(Equal(token.Pos(1)))
		})
	})

	o.Group("binary expression", func() {
		o.BeforeEach(func(t *testing.T) (*testing.T, *BinaryExpression) {
			x := &BasicLiteral{
				LitPos: token.Pos(4),
				Kind:   token.INTEGER,
				Lit:    "3",
			}
			y := &BasicLiteral{
				LitPos: token.Pos(6),
				Kind:   token.INTEGER,
				Lit:    "5",
			}
			return t, &BinaryExpression{
				Op:            token.ADD,
				OpPos:         token.Pos(5),
				LeftHandSide:  x,
				RightHandSide: y,
			}
		})

		o.Spec("has the correct pos", func(t *testing.T, b *BinaryExpression) {
			Expect(t, b.Pos()).To(Equal(token.Pos(4)))
		})
	})
}
