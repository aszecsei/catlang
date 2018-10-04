package ast

import (
	"github.com/aszecsei/catlang/token"
)

type Node interface {
	Pos() token.Pos
	End() token.Pos
}

type Block struct {
	Elements []BlockElement
	Opening  token.Pos
	Closing  token.Pos
}

func (b *Block) Pos() token.Pos { return b.Opening }
func (b *Block) End() token.Pos { return b.Closing }

var _ Node = (*Block)(nil) // Ensure block is a node

type BlockElement interface {
	Node
	blockElementNode()
}

type Declaration struct {
	Declarator Decl
	Opening    token.Pos
	Closing    token.Pos
}

func (d *Declaration) Pos() token.Pos    { return d.Opening }
func (d *Declaration) End() token.Pos    { return d.Closing }
func (d *Declaration) blockElementNode() {}

var _ BlockElement = (*Declaration)(nil) // InternalDeclaration implements BlockElement

type InternalDeclaration struct {
	Declaration
}

type ExternalDeclaration struct {
	Declaration
}

type Decl interface {
	Node
	declaratorNode()
}

type Declarator struct {
	Name    *Ident
	Opening token.Pos
	Closing token.Pos
}

func (d *Declarator) Pos() token.Pos  { return d.Opening }
func (d *Declarator) End() token.Pos  { return d.Closing }
func (d *Declarator) declaratorNode() {}

var _ Decl = (*Declarator)(nil) // Declarator implements Decl

type ConstantDeclarator struct {
	Declarator
	ConstPos  token.Pos
	EqualsPos token.Pos
	Value     Expr
}

type TypeDeclarator struct {
	Declarator
	TypedefPos token.Pos
	EqualsPos  token.Pos
	Type       Type
}

type VariableDeclarator struct {
	Declarator
	LetPos token.Pos
	Type   Type
	Value  Expr
}

type FunctionDeclarator struct {
	Declarator
	Params *FormalParameterList
	Block  *Block
}

type FormalParameterList struct {
	Parameters []*Parameter
	Opening    token.Pos
	Closing    token.Pos
}

func (f *FormalParameterList) Pos() token.Pos { return f.Opening }
func (f *FormalParameterList) End() token.Pos { return f.Closing }

var _ Node = (*FormalParameterList)(nil) // FormalParameterList implements Node

type Parameter struct {
	Name     *Ident
	ColonPos token.Pos
	Type     Type
}

func (p *Parameter) Pos() token.Pos { return p.Name.NamePos }
func (p *Parameter) End() token.Pos { return p.Type.End() }

var _ Node = (*Parameter)(nil) // Parameter implements Node

type ConstParameter struct {
	Parameter
}

type MutableParameter struct {
	Parameter
}

type StructDeclarator struct {
	Declarator
	Members *StructMemberList
}

type StructMemberList struct {
	StructMember []*StructMember
	Opening      token.Pos
	Closing      token.Pos
}

func (s *StructMemberList) Pos() token.Pos { return s.Opening }
func (s *StructMemberList) End() token.Pos { return s.Closing }

var _ Node = (*StructMemberList)(nil) // StructMemberList implements Node

type StructMember struct {
	Name         *Ident
	Type         Type
	InitialValue Expr
}

func (s *StructMember) Pos() token.Pos { return s.Name.NamePos }
func (s *StructMember) End() token.Pos {
	if s.InitialValue != nil {
		return s.InitialValue.End()
	} else {
		return s.Type.End()
	}
}

var _ Node = (*StructMember)(nil) // StructMember implements Node

type OwnedStructMember struct {
	StructMember
}

type EnumDeclarator struct {
	Declarator
	Values *EnumValueList
}

type EnumValueList struct {
	Names   []*Ident
	Opening token.Pos
	Closing token.Pos
}

func (e *EnumValueList) Pos() token.Pos { return e.Opening }
func (e *EnumValueList) End() token.Pos { return e.Closing }

var _ Node = (*EnumValueList)(nil) // EnumValueList implements Node

type Stmt interface {
	BlockElement
	statementNode()
}

type Statement struct {
}

func (s *Statement) blockElementNode() {}
func (s *Statement) statementNode()    {}

type ImportStatement struct {
	Statement
	Import     token.Pos
	ImportList *ImportList
	From       token.Pos
	Path       *StringLiteral
}

func (e *ImportStatement) Pos() token.Pos { return e.Import }
func (e *ImportStatement) End() token.Pos { return e.Path.End() }

var _ Stmt = (*ImportStatement)(nil) // ImportStatement implements Stmt

type ImportList struct {
	Names   []*ImportIdent
	Opening token.Pos
	Closing token.Pos
}

func (i *ImportList) Pos() token.Pos { return i.Opening }
func (i *ImportList) End() token.Pos { return i.Closing }

var _ Node = (*ImportList)(nil) // ImportList implements Node

type ImportIdent struct {
	OldName *Ident
	As      token.Pos
	NewName *Ident
}

func (i *ImportIdent) Pos() token.Pos { return i.OldName.NamePos }
func (i *ImportIdent) End() token.Pos { return i.NewName.End() }

var _ Node = (*ImportIdent)(nil) // ImportIdent implements Node

type InnerBlock struct {
	Statement
	Opening token.Pos
	Block   *Block
	Closing token.Pos
}

func (i *InnerBlock) Pos() token.Pos { return i.Opening }
func (i *InnerBlock) End() token.Pos { return i.Closing }

var _ Stmt = (*InnerBlock)(nil) // InnerBlock implements Stmt

type Cond interface {
	Node
	conditionNode()
}

type Condition struct{}

func (c *Condition) conditionNode() {}

type If struct {
	Statement
	Condition
	If   token.Pos
	Cond Expr
	Then *Block
	Else Cond
}

func (i *If) Pos() token.Pos { return i.If }
func (i *If) End() token.Pos {
	if i.Else != nil {
		return i.Else.End()
	} else {
		return i.Then.End()
	}
}
func (e *If) conditionNode() {}

var _ Stmt = (*If)(nil) // If implements Statement
var _ Cond = (*If)(nil) // If implements Condition

type Else struct {
	Opening token.Pos
	Block   *Block
	Closing token.Pos
}

func (e *Else) Pos() token.Pos { return e.Opening }
func (e *Else) End() token.Pos { return e.Closing }
func (e *Else) conditionNode() {}

var _ Cond = (*Else)(nil) // Else implements Condition

type ForLoop struct {
	Statement
	For       token.Pos
	Initial   *Expression
	Condition *Expression
	Step      *Expression
	Block     *Block
	Closing   token.Pos
}

func (f *ForLoop) Pos() token.Pos { return f.For }
func (f *ForLoop) End() token.Pos { return f.Closing }

var _ Stmt = (*ForLoop)(nil) // ForLoop implements Statement

type WhileLoop struct {
	Statement
	While     token.Pos
	Condition *Expression
	Block     *Block
	Closing   token.Pos
}

func (w *WhileLoop) Pos() token.Pos { return w.While }
func (w *WhileLoop) End() token.Pos { return w.Closing }

var _ Stmt = (*WhileLoop)(nil) // WhileLoop implements Statement

type DoWhileLoop struct {
	Statement
	Do        token.Pos
	Block     *Block
	Condition *Expression
	Closing   token.Pos
}

func (w *DoWhileLoop) Pos() token.Pos { return w.Do }
func (w *DoWhileLoop) End() token.Pos { return w.Closing }

var _ Stmt = (*DoWhileLoop)(nil) // DoWhileLoop implements Loop

type JumpStatement struct {
	Statement
	Command    token.TokenType
	CommandPos token.Pos
	Returns    Expr
}

func (j *JumpStatement) Pos() token.Pos { return j.CommandPos }
func (j *JumpStatement) End() token.Pos {
	if j.Returns != nil {
		return j.Returns.End()
	} else {
		return j.CommandPos + token.Pos(len(j.Command.String()))
	}
}

var _ Stmt = (*JumpStatement)(nil) // JumpStatement implements Stmt

type Type interface {
	Node
	typeNode()
}

type TypeStr struct{}

func (t *TypeStr) typeNode() {}

type PointerType struct {
	TypeStr
	Pointer token.Pos
	Type    Type
}

func (p *PointerType) Pos() token.Pos { return p.Pointer }
func (p *PointerType) End() token.Pos { return p.Type.End() }

var _ Type = (*PointerType)(nil)

type SizedArrayType struct {
	TypeStr
	pos  token.Pos
	Size Expr
	Type Type
}

func (s *SizedArrayType) Pos() token.Pos { return s.pos }
func (s *SizedArrayType) End() token.Pos { return s.Type.End() }

var _ Type = (*SizedArrayType)(nil)

type UnsizedArrayType struct {
	TypeStr
	pos  token.Pos
	Type Type
}

func (u *UnsizedArrayType) Pos() token.Pos { return u.pos }
func (u *UnsizedArrayType) End() token.Pos { return u.Type.End() }

var _ Type = (*UnsizedArrayType)(nil)

type TypeUnion struct {
	TypeStr
	FirstType  Type
	Union      token.Pos
	SecondType Type
}

func (t *TypeUnion) Pos() token.Pos { return t.FirstType.Pos() }
func (t *TypeUnion) End() token.Pos { return t.SecondType.End() }

var _ Type = (*TypeUnion)(nil)

type TypeofType struct {
	TypeStr
	Typeof     token.Pos
	Expression Expr
}

func (t *TypeofType) Pos() token.Pos { return t.Typeof }
func (t *TypeofType) End() token.Pos { return t.Expression.End() }

var _ Type = (*TypeofType)(nil)

type OptionalType struct {
	TypeStr
	Type   Type
	Option token.Pos
}

func (o *OptionalType) Pos() token.Pos { return o.Type.Pos() }
func (o *OptionalType) End() token.Pos { return o.Option }

var _ Type = (*OptionalType)(nil)

type NamedType struct {
	TypeStr
	Name *Ident
}

func (n *NamedType) Pos() token.Pos { return n.Name.NamePos }
func (n *NamedType) End() token.Pos { return n.Name.End() }

var _ Type = (*NamedType)(nil)

type Expr interface {
	Node
	exprNode()
}

type Expression struct {
}

func (e *Expression) exprNode() {}

type AssignmentExpression struct {
	Expression
	Equal token.Pos
	Name  *Ident
	Value Expr
}

func (e *AssignmentExpression) Pos() token.Pos { return e.Name.NamePos }
func (e *AssignmentExpression) End() token.Pos { return e.Value.End() }

var _ Expr = (*AssignmentExpression)(nil)

type TernaryExpression struct {
	Expression
	Condition    Expr
	QuestionMark token.Pos
	TrueCase     Expr
	Colon        token.Pos
	FalseCase    Expr
}

func (e *TernaryExpression) Pos() token.Pos { return e.Condition.Pos() }
func (e *TernaryExpression) End() token.Pos { return e.FalseCase.End() }

var _ Expr = (*TernaryExpression)(nil)

type BinaryExpression struct {
	Expression
	LeftHandSide  Expr
	Op            token.TokenType
	OpPos         token.Pos
	RightHandSide Expr
}

func (e *BinaryExpression) Pos() token.Pos { return e.LeftHandSide.Pos() }
func (e *BinaryExpression) End() token.Pos { return e.RightHandSide.End() }

var _ Expr = (*BinaryExpression)(nil)

type LambdaExpression struct {
	Expression
	Opening token.Pos
	Params  *FormalParameterList
	Arrow   token.Pos
	Block   *Block
	Closing token.Pos
}

func (l *LambdaExpression) Pos() token.Pos { return l.Opening }
func (l *LambdaExpression) End() token.Pos { return l.Closing }

var _ Expr = (*LambdaExpression)(nil)

type BasicLiteral struct {
	LitPos token.Pos
	Kind   token.TokenType
	Lit    string
}

func (b *BasicLiteral) Pos() token.Pos { return b.LitPos }
func (b *BasicLiteral) End() token.Pos { return b.LitPos + token.Pos(len(b.Lit)) }
func (b *BasicLiteral) exprNode()      {}

type IntegerLiteral struct {
	BasicLiteral
	Value int
}

var _ Expr = (*IntegerLiteral)(nil)

type LongLiteral struct {
	BasicLiteral
	Value int64
}

var _ Expr = (*LongLiteral)(nil)

type FloatLiteral struct {
	BasicLiteral
	Value float32
}

var _ Expr = (*FloatLiteral)(nil)

type DoubleLiteral struct {
	BasicLiteral
	Value float64
}

var _ Expr = (*DoubleLiteral)(nil)

type StringLiteral struct {
	BasicLiteral
	Value string
}

var _ Expr = (*StringLiteral)(nil)

type CharacterLiteral struct {
	BasicLiteral
	Value rune
}

var _ Expr = (*CharacterLiteral)(nil)

type Ident struct {
	NamePos token.Pos
	Name    string
	Object  *Object // may be nil (ie. Name is a type keyword)
}

func (i *Ident) End() token.Pos { return i.NamePos + token.Pos(len(i.Name)) }

// Code to deal with objects

type Object struct {
	DefinedAt token.Pos
	Name      string
	Kind      Kind
}

type Scope struct {
	Parent *Scope
	Table  map[string]*Object
}

func NewScope(parent *Scope) *Scope {
	return &Scope{Parent: parent, Table: make(map[string]*Object)}
}

func (s *Scope) Insert(ob *Object) *Object {
	if old, ok := s.Table[ob.Name]; ok {
		return old
	}
	s.Table[ob.Name] = ob
	return nil
}

func (s *Scope) Lookup(ident string) *Object {
	ob, ok := s.Table[ident]
	// If we find the object, or we're at the top-level scope, we return it as-is
	if ok || s.Parent == nil {
		return ob
	}
	// If we couldn't find the object and there's a parent scope, we ask the parent
	return s.Parent.Lookup(ident)
}
