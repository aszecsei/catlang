---
id: grammar
title: Grammar
sidebar_label: Grammar
---

## EBNF

```ebnf
lexeme
    = identifier
    | reserved word
    | number
    | string literal
    | character literal
    | byte literal
    | byte string literal
    | punctuation
    | end of file
    ;

identifier
    = letter , { letter | digit | "_" }
    ;

number
    = decimal integer
    | decimal float
    | "0x" , hexadecimal integer
    | "0o" , octal integer
    | "0b" , binary integer
    ;

decimal integer
    = [ "+" | "-" ] , digit , { digit } [ integer type ]
    ;

decimal float
    = [ "+" | "-" ] , digit , { digit } , floating point type
    | [ "+" | "-" ] , digit , { digit } , "." , { digit } [ floating point type ]
    | [ "+" | "-" ] , "." , digit , { digit } [ floating point type ]
    ;

hexadecimal integer
    = hexadecimal digit , { hexadecimal digit }
    ;

octal integer
    = octal digit , { octal digit }
    ;

binary number
    = binary digit , { binary digit }
    ;

string literal
    = '"' , { string character } , '"'
    ;

character literal
    = "'" , string character , "'"
    ;

byte literal
    = "b'" , byte character , "'"
    ;

byte string literal
    = 'b"' , { byte character } , '"'
    ;

string character
    = ? utf-8 character ?
    | quote escape
    | ascii escape
    | unicode escape
    ;

byte character
    = ? ascii character ?
    | quote escape
    | byte escape
    ;

ascii escape
    = "\x" , 2*( hexadecimal digit ) /* 7-bit character code (exactly 2 digits, up to 0x7F) */
    | "\n" /* newline */
    | "\r" /* carriage return */
    | "\t" /* tab */
    | "\\" /* backslash */
    | "\0" /* null */
    ;

byte escape
    = "\x" , 2*( hexadecimal digit ) /* 8-bit character code (exactly two digits) */
    | "\n"
    | "\r"
    | "\t"
    | "\\"
    | "\0"
    ;

unicode escape
    = "\u{" , 6*( hexadecimal digit ) , "}" /* 24-bit Unicode character code (up to 6 digits) */
    ;

quote escape
    = "\'"
    | "\""
    ;

punctuation
    = "("
    | ")"
    | "{"
    | "}"
    | "#["
    | "["
    | "]"
    | ":"
    | "::"
    | ";"
    | ","
    | "@"
    | "+"
    | "+="
    | "++"
    | "-"
    | "-="
    | "--"
    | "*"
    | "*="
    | "/"
    | "/="
    | "%"
    | "%="
    | "="
    | "&"
    | "&="
    | "&&"
    | "&&="
    | "|"
    | "|="
    | "||"
    | "||="
    | "!"
    | "~"
    | "^"
    | "^="
    | "<<"
    | ">>"
    | "=="
    | "!="
    | "<"
    | ">"
    | "<="
    | ">="
    | "->"
    | "?"
    | "??"
    | "."
    | ".."
    | "..."
    ;

reserved words
    = "any"
    | "let"
    | "const"
    | "new"
    | "delete"
    | "typeof"
    | "is"
    | "as"
    | "in"
    | "function"
    | "return"
    | "struct"
    | "type"
    | "enum"
    | "SOA"
    | "owned"
    | "import"
    | "export"
    | "from"
    | "for"
    | "while"
    | "do"
    | "loop"
    | "if"
    | "else"
    | "break"
    | "continue"
    | "this"
    ;

catlang file
    = catlang module
    | catlang script
    ;

catlang module
    = { [ import | export | ( { attribute } , declaration ) } , end of file
    ;

catlang script
    = { { attribute } , declaration } , end of file
    ;

attribute
    = "#[" , attribute list , "]"
    ;

attribute list
    = attribute element , { "," , attribute element }
    ;

attribute element
    = identifier , [ "(" , [ attribute parameter list ] , ")" ]
    ;

attribute parameter list
    = attribute parameter , { "," , attribute parameter }
    ;

attribute parameter
    = expression
    ;

import
    = "import" , import list , "from" , string literal
    ;

export
    = { attribute } , "export" , declaration /* export declaration */
    | "export" , identifier , [ "as" , identifier ] /* export statement */
    | "export" , import list , "from" , string literal /* re-export */
    ;

import list
    = "{" , import identifier , { "," , import identifier } , "}"
    | "*" , "as", identifier
    ;

import identifier
    = identifier , [ "as" , identifier ]
    ;

block
    = "{" { declaration | statement } "}"
    ;

declaration
    = constant declaration
    | type declaration
    | variable declaration
    | function declaration
    | struct declaration
    | enum declaration
    ;

constant declaration
    = "const" , identifier , [ ":" , type expression ] , "=" , expression
    ;

type declaration
    = "type" , identifier , "=" , type expression
    ;

variable declaration
    = "let" , identifier , [ ":" , type expression ] , "=" , expression
    ;

function declaration
    = "function" , scoped value , "(" , [ formal parameter list ] , ")" , [ "->" , [ type expression ] ] , block
    ;

formal parameter list
    = "this" , { "," , parameter }
    | parameter , { "," , parameter }
    ;

parameter
    = identifier , ":" , type expression
    ;

struct declaration
    = "struct" , identifier , "{" , struct parameter list , "}"
    ;

struct parameter list
    = { struct parameter , [ ";" ] }
    ;

struct parameter
    = identifier , ":" , [ "owned" , "*" ] , type expression , [ "=" , expression ]
    ;

enum declaration
    = "enum" , identifier , [ ":" , enum representation ] , "{" , [ enum value list ] , "}"
    ;

enum value list
    = { enum value , "," } , enum value , [ "," ]
    ;

enum value
    = { identifier , [ "=" , expression ] }
    ;

enum representation
    = "s8"
    | "u8"
    | "s16"
    | "u16"
    | "s32"
    | "u32"
    | "s64"
    | "u64"
    ;

statement
    = block
    | if
    | loop
    | jump
    | expression
    ;

if
    = "if" , "(" , expression , ")" , statement , [ "else" , statement ]
    ;

loop
    = for
    | while
    | do while
    | infinite loop
    ;

for
    = "for" , "(" , identifier , "in", expression , ")" , statement
    ;

while
    = "while" , "(", condition , ")" , statement
    ;

do while
    = "do" , statement , "while" , "(" , condition , ")"
    ;

infinite loop
    = "loop" , statement
    ;

jump
    = "break"
    | "continue"
    | "return" , [ expression ]
    ;

type expression
    = type union
    | unary type expression
    ;

type union
    = unary type expression , "|" , type expression

unary type expression
    = pointer to
    | sized array
    | unsized array
    | const type
    | volatile type
    | optional type
    | simple type expression
    ;

pointer to
    = "*" , unary type expression
    ;
sized array
    = "[" , expression , "]" , unary type expression
    ;
unsized array
    = "[" , ".." , "]" , unary type expression
    ;
const type
    = "const" , unary type expression
    ;
volatile type
    = "volatile" , unary type expression
    ;
optional type
    = "?" , unary type expression
    ;

simple type expression
    = typeof expression
    | named type
    | primitive type
    | "(" type expression ")"
    | "any"
    ;

typeof expression
    = "typeof" , expression
    ;
named type
    = identifier { "::" , identifier } (* scoped types *)
    ;
primitive type
    = integer type
    | boolean type
    | floating point type
    | unvalued type
    | "type"
    ;

integer type
    = "s8"
    | "u8"
    | "s16"
    | "u16"
    | "s32"
    | "u32"
    | "s64"
    | "u64"
    | "char"
    | "short"
    | "int"
    | "long"
    | "c_short"
    | "c_ushort"
    | "c_int"
    | "c_uint"
    | "c_long"
    | "c_ulong"
    | "c_longlong"
    | "c_ulonglong"
    | "c_longdouble"
    ;

boolean type
    = "bool"
    ;

floating point type
    = "f32"
    | "f64"
    | "float"
    | "double"
    ;

unvalued type
    = "null"
    | "noreturn"
    | "c_void"
    ;

expression
    = assignment expression
    ;

assignment expression (* right-associative *)
    = scoped value , assignment operator , assignment expression
    | ternary expression
    ;
assignment operator
    = "="
    | "+="
    | "-="
    | "*="
    | "/="
    | "%="
    | "&="
    | "&&="
    | "|="
    | "||="
    | "^="
    | "<<="
    | ">>="
    | "??="
    ;

ternary expression (* right-associative *)
    = or expression , [ "?" , expression , ":" , expression ]
    ;

or expression (* left-associative *)
    = and expression , { "||" , and expression }
    ;

and expression (* left-associative *)
    = equality expression , { "&&" , equality expression }
    ;

equality expression (* left-associative *)
    = type test expression , { equality operator , type test expression }
    ;
equality operator
    = "=="
    | "!="
    ;

type test expression (* left-associative *)
    = comparison expression , [ "is" , type expression ]
    ;

comparison expression (* left-associative *)
    = bitwise or expression , { comparison operator , bitwise or expression }
    ;
comparison operator
    = "<"
    | "<="
    | ">"
    | ">="
    ;

bitwise or expression (* left-associative *)
    = bitwise xor expression , { "|" , bitwise xor expression }
    ;

bitwise xor expression (* left-associative *)
    = bitwise and expression , { "^" , bitwise and expression }
    ;

bitwise and expression (* left-associative *)
    = bitshift expression , { "&" , bitshift expression }
    ;

bitshift expression (* left-associative *)
    = range expression , { bitshift operator , range expression }
    ;
bitshift operator
    = ">>"
    | "<<"
    ;

range expression (* left-associative *)
    = adding expression , { range operator , adding expression }
    ;
range operator
    = ".."
    | "..."
    ;

adding expression (* left-associative *)
    = term , { adding operator , term }
    ;
adding operator
    = "+"
    | "-"
    ;

term (* left-associative *)
    = factor , { multiplying operator , factor }
    ;
multiplying operator
    = "*"
    | "/"
    | "%"
    ;

factor (* right-associative *)
    = prefix operator , factor
    | suffix expression , "as" , factor
    | suffix expression , "??" , factor
    | suffix expression
    ;
prefix operator
    = "++"
    | "--"
    | "+"
    | "-"
    | "!"
    | "~"
    | "@"
    | "*"
    ;

suffix expression (* left-associative *)
    = value , suffix operator
    | value , "(" , [ expression list ] , ")" (* function call *)
    | value , "[" , [ expression list ] , "]" (* subscript *)
    | value , { "." , suffix expression } (* member access *)
    | value
    ;
suffix operator
    = "++"
    | "--"
    | "!"
    | "?"
    ;

value
    = "sizeof" , "(" , type expression , ")"
    | lambda expression
    | type expression [ "::" identifier ]
    | number
    | string literal
    | character literal
    | byte literal
    | byte string literal
    | reference
    ;

lambda expression
    = "(" , [ formal parameter list ] , ")" , [ "->" , [ type expression ] ] , block
    ;

reference
    = identifier
    | "this"
    ;
```

## TODO

- float exponents (`0.2e1`)
- more enum representations (`c_int`?)
- let loops have return types via break/continue/etc?
- `suffix expression` feels poorly-defined
