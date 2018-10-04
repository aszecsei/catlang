package token_test

import (
	"testing"

	"github.com/apoydence/onpar"
	. "github.com/apoydence/onpar/expect"

	. "github.com/apoydence/onpar/matchers"

	"github.com/aszecsei/catlang/token"
)

func TestToken(t *testing.T) {
	o := onpar.New()
	defer o.Run(t)

	o.Group("Lookup", func() {
		o.Spec("+", func(t *testing.T) {
			res := token.Lookup("+")
			Expect(t, res).To(Equal(token.ADD))
		})

		o.Spec("%", func(t *testing.T) {
			res := token.Lookup("%")
			Expect(t, res).To(Equal(token.REM))
		})

		o.Spec("const", func(t *testing.T) {
			res := token.Lookup("const")
			Expect(t, res).To(Equal(token.CONST))
		})

		o.Spec("ident", func(t *testing.T) {
			res := token.Lookup("foo")
			Expect(t, res).To(Equal(token.IDENT))
		})
	})

	o.Group("IsLiteral", func() {
		o.Spec("+", func(t *testing.T) {
			res := token.ADD.IsLiteral()
			Expect(t, res).To(BeFalse())
		})

		o.Spec("const", func(t *testing.T) {
			res := token.CONST.IsLiteral()
			Expect(t, res).To(BeFalse())
		})

		o.Spec("ident", func(t *testing.T) {
			res := token.IDENT.IsLiteral()
			Expect(t, res).To(BeTrue())
		})

		o.Spec("integer", func(t *testing.T) {
			res := token.INTEGER.IsLiteral()
			Expect(t, res).To(BeTrue())
		})
	})
}
