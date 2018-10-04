package ast_test

import (
	"testing"

	"github.com/apoydence/onpar"
	. "github.com/apoydence/onpar/expect"

	. "github.com/apoydence/onpar/matchers"

	. "github.com/aszecsei/catlang/ast"
)

func TestKind(t *testing.T) {
	o := onpar.New()
	defer o.Run(t)

	o.Group("function kinds", func() {
		o.BeforeEach(func(t *testing.T) (*testing.T, Kind) {
			return t, FuncDecl
		})

		o.Spec("has the correct string", func(t *testing.T, k Kind) {
			Expect(t, k.String()).To(Equal("function"))
		})
	})

	o.Group("variable kinds", func() {
		o.BeforeEach(func(t *testing.T) (*testing.T, Kind) {
			return t, VarDecl
		})

		o.Spec("has the correct string", func(t *testing.T, k Kind) {
			Expect(t, k.String()).To(Equal("variable"))
		})
	})

	o.Group("type kinds", func() {
		o.BeforeEach(func(t *testing.T) (*testing.T, Kind) {
			return t, TypeDecl
		})

		o.Spec("has the correct string", func(t *testing.T, k Kind) {
			Expect(t, k.String()).To(Equal("type"))
		})
	})

	o.Group("other kinds", func() {
		o.BeforeEach(func(t *testing.T) (*testing.T, Kind) {
			return t, None
		})

		o.Spec("has the correct string", func(t *testing.T, k Kind) {
			Expect(t, k.String()).To(Equal(""))
		})
	})
}
