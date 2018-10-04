package ast

type Kind int

const (
	None Kind = iota + 1
	FuncDecl
	VarDecl
	TypeDecl
)

func (k Kind) String() string {
	switch k {
	case FuncDecl:
		return "function"
	case VarDecl:
		return "variable"
	case TypeDecl:
		return "type"
	default:
		return ""
	}
}
