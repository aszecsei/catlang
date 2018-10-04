package ast

type Visitor interface {
	Visit(node Node) bool
}

func Walk(node Node, v Visitor) {
	if node == nil || !v.Visit(node) {
		return
	}

	switch n := node.(type) {
	case *Block:
		for _, elem := range n.Elements {
			Walk(elem, v)
		}
	case *Declaration:
		Walk(n.Declarator, v)
		Walk(n.Declarator, v)
	case *ExportedDeclaration:
		Walk(n.Declarator, v)
	case *Declarator:
		Walk(n.Name, v)
	case *ConstantDeclarator:
		Walk(n.Name, v)
		Walk(n.Value, v)
	case *TypeDeclarator:
		Walk(n.Name, v)
		Walk(n.Type, v)
	case *VariableDeclarator:
		Walk(n.Name, v)
		Walk(n.Type, v)
		Walk(n.Value, v)
	case *FunctionDeclarator:
		Walk(n.Name, v)
		Walk(n.Params, v)
		Walk(n.Block, v)
	case *FormalParameterList:
		for _, param := range n.Parameters {
			Walk(param, v)
		}
	case *Parameter:
		Walk(n.Name, v)
		Walk(n.Type, v)
	case *ConstParameter:
		Walk(n.Name, v)
		Walk(n.Type, v)
	case *StructDeclarator:
		Walk(n.Name, v)
		Walk(n.Members, v)
	case *StructMemberList:
		for _, member := range n.Members {
			Walk(member, v)
		}
	case *StructMember:
		Walk(n.Name, v)
		Walk(n.Type, v)
		Walk(n.InitialValue, v)
	case *OwnedStructMember:
		Walk(n.Name, v)
		Walk(n.Type, v)
		Walk(n.InitialValue, v)
	case *EnumDeclarator:
		Walk(n.Name, v)
		Walk(n.Values, v)
	case *EnumValueList:
		for _, value := range n.Values {
			Walk(value, v)
		}
	case *ImportStatement:
		Walk(n.ImportList, v)
		Walk(n.Path, v)
	case *ImportList:
		for _, name := range n.Names {
			Walk(name, v)
		}
	case *ImportIdent:
		Walk(n.OldName, v)
		Walk(n.NewName, v)
	case *InnerBlock:
		Walk(n.Block, v)
	case *If:
		Walk(n.Cond, v)
		Walk(n.Then, v)
		Walk(n.Else, v)
	case *Else:
		Walk(n.Block, v)
	case *ForLoop:
		Walk(n.Initial, v)
		Walk(n.Condition, v)
		Walk(n.Step, v)
		Walk(n.Block, v)
	case *WhileLoop:
		Walk(n.Condition, v)
		Walk(n.Block, v)
	case *DoWhileLoop:
		Walk(n.Block, v)
		Walk(n.Condition, v)
	case *JumpStatement:
		Walk(n.Returns, v)
	case *PointerType:
		Walk(n.Type, v)
	case *SizedArrayType:
		Walk(n.Size, v)
		Walk(n.Type, v)
	case *UnsizedArrayType:
		Walk(n.Type, v)
	case *TypeUnion:
		Walk(n.FirstType, v)
		Walk(n.SecondType, v)
	case *TypeofType:
		Walk(n.Expression, v)
	case *OptionalType:
		Walk(n.Type, v)
	case *NamedType:
		Walk(n.Name, v)
	case *AssignmentExpression:
		Walk(n.Name, v)
		Walk(n.Value, v)
	case *TernaryExpression:
		Walk(n.Condition, v)
		Walk(n.TrueCase, v)
		Walk(n.FalseCase, v)
	case *BinaryExpression:
		Walk(n.LeftHandSide, v)
		Walk(n.RightHandSide, v)
	case *LambdaExpression:
		Walk(n.Params, v)
		Walk(n.Block, v)
	case *BasicLiteral: /* do nothing */
	case *IntegerLiteral: /* do nothing */
	case *LongLiteral: /* do nothing */
	case *FloatLiteral: /* do nothing */
	case *DoubleLiteral: /* do nothing */
	case *StringLiteral: /* do nothing */
	case *CharacterLiteral: /* do nothing */
	case *Ident: /* do nothing */
	case *File:
		Walk(n.Block, v)
	case *Package:
		for _, file := range n.Files {
			Walk(file, v)
		}
	}
}
