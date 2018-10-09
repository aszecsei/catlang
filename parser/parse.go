package ast

import (
	"fmt"
	"io/ioutil"
	"os"
	"path/filepath"

	"github.com/aszecsei/catlang/ast"
	"github.com/aszecsei/catlang/lexer"
	"github.com/aszecsei/catlang/token"
)

func ParseFile(fset *token.FileSet, filename, src string) (*ast.File, error) {
	if src == "" {
		fi, err := os.Stat(filename)
		if err != nil {
			return nil, err
		}

		if ext := filepath.Ext(fi.Name()); ext != ".cat" {
			return nil, fmt.Errorf("unknown file extension, must be .cat")
		}
		b, err := ioutil.ReadFile(filename)
		if err != nil {
			return nil, err
		}
		src = string(b)
	}
	file := fset.Add(filepath.Base(filename), src)
	var p parser
	p.init(file, filename, src, ast.NewScope(nil))
	f := p.parseFile()

	if p.errors.Count() > 0 {
		return nil, p.errors
	}

	return f, nil
}

type parser struct {
	file    *token.File
	errors  token.ErrorList
	scanner lexer.Scanner
	listok  bool

	curScope *ast.Scope
	topScope *ast.Scope
}

/* Utility */

func (p *parser) addError(args ...interface{}) {
	p.errors.Add(p.file.Position(p.scanner.CurrentLexeme().Position), args...)
}

func (p *parser) expect(tok token.TokenType) token.Pos {
	cur := p.scanner.CurrentLexeme()
	if cur.Type != tok {
		p.addError("Expected '" + tok.String() + "' but got '" + cur.Literal + "'")
		return cur.Position
	}
	defer p.next()
	return cur.Position
}

func (p *parser) init(file *token.File, fname, src string, s *ast.Scope) {
	if s == nil {
		s = ast.NewScope(nil)
	}
	p.file = file
	p.scanner.Init(p.file, src)
	p.listok = false
	p.curScope = s
	p.topScope = s
	p.next()
}

func (p *parser) next() {
	p.scanner.Advance()
}

func (p *parser) openScope() {
	p.curScope = ast.NewScope(p.curScope)
}

func (p *parser) closeScope() {
	p.curScope = p.curScope.Parent
}

/* Parsing */

func (p *parser) parseFile() *ast.File {
	b := p.parseBlock()
	defer p.expect(token.EOF)
	return &ast.File{
		Block: b,
	}
}

func isDeclarationStarter(t token.TokenType) bool {
	return t == token.EXPORT ||
				 t == token.CONST ||
				 t == token.TYPEDEF ||
				 t == token.LET ||
				 t == token.FUNCTION ||
				 t == token.STRUCT ||
				 t == token.ENUM
}

func (p *parser) parseBlock() *ast.Block {
	elems := make([]ast.BlockElement, 0)
	begin := p.scanner.CurrentLexeme().Position
	for p.scanner.CurrentLexeme().Type != token.EOF {
		cur := p.scanner.CurrentLexeme()
		var elem ast.BlockElement
		if isDeclarationStarter(cur.Type) {
			elem = p.parseDeclaration()
		} else {
			elem = p.parseStatement()
		}
		elems = append(elems, elem)
	}
	return &ast.Block{
		Elements: elems,
		Opening: begin,
	}
}

func (p *parser) parseDeclaration() *ast.Declaration {
	cur := p.scanner.CurrentLexeme()
	d := &ast.Declaration{}
	if cur.Type == token.EXPORT {
		d.IsExported = true
		p.next()
	}
	d.Declarator = p.parseDeclarator()
	return d
}

func (p *parser) parseDeclarator() ast.Decl {
	cur := p.scanner.CurrentLexeme()
	switch cur.Type {
	case token.CONST:
		return p.parseConstDeclarator()
	case token.TYPEDEF:
		return p.parseTypeDeclarator()
	case token.LET:
		return p.parseVariableDeclarator()
	case token.FUNCTION:
		return p.parseFunctionDeclarator()
	case token.STRUCT:
		return p.parseStructDeclarator()
	case token.ENUM:
		return p.parseEnumDeclarator()
	default:
		p.addError("Expected a declarator; got '" + cur.Literal + "'")
		return nil
	}
}

func (p *parser) parseConstDeclarator() *ast.ConstantDeclarator {
	// TODO
	return nil
}

func (p *parser) parseTypeDeclarator() *ast.TypeDeclarator {
	// TODO
	return nil
}

func (p *parser) parseVariableDeclarator() *ast.VariableDeclarator {
	// TODO
	return nil
}

func (p *parser) parseFunctionDeclarator() *ast.FunctionDeclarator {
	// TODO
	return nil
}

func (p *parser) parseStructDeclarator() *ast.StructDeclarator {
	// TODO
	return nil
}

func (p *parser) parseEnumDeclarator() *ast.EnumDeclarator {
	// TODO
	return nil
}

func (p *parser) parseStatement() ast.Stmt {
	return nil
}