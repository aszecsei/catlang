package ast

import (
	"github.com/aszecsei/catlang/token"
)

type Node interface {
	Pos() token.Pos
}

type Block struct {
	Elements []BlockElement
	Opening  token.Pos
}

func (b *Block) Pos() token.Pos { return b.Opening }

var _ Node = (*Block)(nil) // Ensure block is a node

type BlockElement interface {
	Node
	blockElementNode()
}

type Declaration struct {
	IsExported bool
	Declarator Decl
}

func (d *Declaration) Pos() token.Pos    { return d.Declarator.Pos() }
func (d *Declaration) blockElementNode() {}

var _ BlockElement = (*Declaration)(nil) // InternalDeclaration implements BlockElement

type Decl interface {
	Node
	declaratorNode()
}

type Declarator struct {
	Name *Ident
}

func (d *Declarator) Pos() token.Pos  { return token.NoPos }
func (d *Declarator) declaratorNode() {}

var _ Decl = (*Declarator)(nil) // Declarator implements Decl

type ConstantDeclarator struct {
	Declarator
	ConstPos  token.Pos
	EqualsPos token.Pos
	Value     Expr
}

func (d *ConstantDeclarator) Pos() token.Pos { return d.ConstPos }

type TypeDeclarator struct {
	Declarator
	TypedefPos token.Pos
	EqualsPos  token.Pos
	Type       Type
}

func (d *TypeDeclarator) Pos() token.Pos { return d.TypedefPos }

type VariableDeclarator struct {
	Declarator
	LetPos    token.Pos
	ColonPos  token.Pos
	Type      Type
	EqualsPos token.Pos
	Value     Expr
}

func (d *VariableDeclarator) Pos() token.Pos { return d.LetPos }

type FunctionDeclarator struct {
	Declarator
	FunctionPos token.Pos
	Params      *FormalParameterList
	OpenBlock   token.Pos
	Block       *Block
	CloseBlock  token.Pos
}

func (d *FunctionDeclarator) Pos() token.Pos { return d.FunctionPos }

type FormalParameterList struct {
	Open       token.Pos
	Parameters []*Parameter
	Close      token.Pos
}

func (f *FormalParameterList) Pos() token.Pos { return f.Open }

var _ Node = (*FormalParameterList)(nil) // FormalParameterList implements Node

type Parameter struct {
	Name     *Ident
	ColonPos token.Pos
	Type     Type
}

func (p *Parameter) Pos() token.Pos { return p.Name.NamePos }

var _ Node = (*Parameter)(nil) // Parameter implements Node

type ConstParameter struct {
	Parameter
	ConstPos token.Pos
}

func (d *ConstParameter) Pos() token.Pos { return d.ConstPos }

type StructDeclarator struct {
	Declarator
	StructPos token.Pos
	Members   *StructMemberList
}

func (d *StructDeclarator) Pos() token.Pos { return d.StructPos }

type StructMemberList struct {
	Open    token.Pos
	Members []*StructMember
	Close   token.Pos
}

func (s *StructMemberList) Pos() token.Pos { return s.Open }

var _ Node = (*StructMemberList)(nil) // StructMemberList implements Node

type StructMember struct {
	Name         *Ident
	ColonPos     token.Pos
	IsOwned      bool
	Type         Type
	InitialValue Expr
}

func (s *StructMember) Pos() token.Pos { return s.Name.NamePos }

var _ Node = (*StructMember)(nil) // StructMember implements Node

type EnumDeclarator struct {
	Declarator
	EnumPos token.Pos
	Values  *EnumValueList
}

func (d *EnumDeclarator) Pos() token.Pos { return d.EnumPos }

type EnumValueList struct {
	Open   token.Pos
	Values []*Ident
	Close  token.Pos
}

func (e *EnumValueList) Pos() token.Pos { return e.Open }

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

var _ Stmt = (*ImportStatement)(nil) // ImportStatement implements Stmt

type ImportList struct {
	Names []*ImportIdent
}

func (i *ImportList) Pos() token.Pos { return i.Names[0].Pos() }

var _ Node = (*ImportList)(nil) // ImportList implements Node

type ImportIdent struct {
	OldName *Ident
	As      token.Pos
	NewName *Ident
}

func (i *ImportIdent) Pos() token.Pos { return i.OldName.NamePos }

var _ Node = (*ImportIdent)(nil) // ImportIdent implements Node

type InnerBlock struct {
	Statement
	Open  token.Pos
	Block *Block
	Close token.Pos
}

func (i *InnerBlock) Pos() token.Pos { return i.Open }

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

var _ Stmt = (*If)(nil) // If implements Statement
var _ Cond = (*If)(nil) // If implements Condition

type Else struct {
	Condition
	Else  token.Pos
	Block *Block
}

func (e *Else) Pos() token.Pos { return e.Else }

var _ Cond = (*Else)(nil) // Else implements Condition

type ForLoop struct {
	Statement
	For       token.Pos
	Initial   Expr
	Condition Expr
	Step      Expr
	Block     *Block
}

func (f *ForLoop) Pos() token.Pos { return f.For }

var _ Stmt = (*ForLoop)(nil) // ForLoop implements Statement

type WhileLoop struct {
	Statement
	While     token.Pos
	Condition Expr
	Block     *Block
}

func (w *WhileLoop) Pos() token.Pos { return w.While }

var _ Stmt = (*WhileLoop)(nil) // WhileLoop implements Statement

type DoWhileLoop struct {
	Statement
	Do        token.Pos
	Block     *Block
	Condition Expr
}

func (w *DoWhileLoop) Pos() token.Pos { return w.Do }

var _ Stmt = (*DoWhileLoop)(nil) // DoWhileLoop implements Loop

type JumpStatement struct {
	Statement
	Command    token.TokenType
	CommandPos token.Pos
	Returns    Expr
}

func (j *JumpStatement) Pos() token.Pos { return j.CommandPos }

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

var _ Type = (*PointerType)(nil)

type SizedArrayType struct {
	TypeStr
	Open  token.Pos
	Size  Expr
	Close token.Pos
	Type  Type
}

func (s *SizedArrayType) Pos() token.Pos { return s.Open }

var _ Type = (*SizedArrayType)(nil)

type UnsizedArrayType struct {
	TypeStr
	Open  token.Pos
	Close token.Pos
	Type  Type
}

func (u *UnsizedArrayType) Pos() token.Pos { return u.Open }

var _ Type = (*UnsizedArrayType)(nil)

type TypeUnion struct {
	TypeStr
	FirstType  Type
	Union      token.Pos
	SecondType Type
}

func (t *TypeUnion) Pos() token.Pos { return t.FirstType.Pos() }

var _ Type = (*TypeUnion)(nil)

type TypeofType struct {
	TypeStr
	Typeof     token.Pos
	Expression Expr
}

func (t *TypeofType) Pos() token.Pos { return t.Typeof }

var _ Type = (*TypeofType)(nil)

type OptionalType struct {
	TypeStr
	Type   Type
	Option token.Pos
}

func (o *OptionalType) Pos() token.Pos { return o.Type.Pos() }

var _ Type = (*OptionalType)(nil)

type NamedType struct {
	TypeStr
	Name *Ident
}

func (n *NamedType) Pos() token.Pos { return n.Name.NamePos }

var _ Type = (*NamedType)(nil)

type Expr interface {
	Stmt
	exprNode()
}

type Expression struct {
	Statement
}

func (e *Expression) exprNode() {}

type AssignmentExpression struct {
	Expression
	Equal token.Pos
	Name  *Ident
	Value Expr
}

func (e *AssignmentExpression) Pos() token.Pos { return e.Name.NamePos }

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

var _ Expr = (*TernaryExpression)(nil)

type BinaryExpression struct {
	Expression
	LeftHandSide  Expr
	Op            token.TokenType
	OpPos         token.Pos
	RightHandSide Expr
}

func (e *BinaryExpression) Pos() token.Pos { return e.LeftHandSide.Pos() }

var _ Expr = (*BinaryExpression)(nil)

type LambdaExpression struct {
	Expression
	Params *FormalParameterList
	Arrow  token.Pos
	Block  *Block
}

func (l *LambdaExpression) Pos() token.Pos { return l.Params.Pos() }

var _ Expr = (*LambdaExpression)(nil)

type BasicLiteral struct {
	LitPos token.Pos
	Kind   token.TokenType
	Lit    string
}

func (b *BasicLiteral) Pos() token.Pos    { return b.LitPos }
func (b *BasicLiteral) exprNode()         {}
func (b *BasicLiteral) blockElementNode() {}
func (b *BasicLiteral) statementNode()    {}

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
	Type    *Ident // may be nil (ie. Name is a type keyword)
}

func (i *Ident) Pos() token.Pos { return i.NamePos }

var _ Node = (*Ident)(nil)

type File struct {
	Block *Block
}

func (f *File) Pos() token.Pos { return token.NoPos }

type Package struct {
	Files []*File
}

func (p *Package) Pos() token.Pos { return token.NoPos }

// Code to deal with objects

type Object struct {
	Name    string
	NamePos token.Pos
	Kind    Kind
}

func (o *Object) Pos() token.Pos { return o.NamePos }

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
