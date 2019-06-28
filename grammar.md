# Vraw Grammar

```
program		:= [shape]
shape		  := <ident> [<arg>] [<stmt>]
arg			  := <ident>
stmt		  := <expr>
expr		  := <funcall> | <literal> | <binop> | <unop>
funcall		:= <ident> [<namedarg>]
namedarg	:= <ident> <expr>
binop     := <expr> <op> <expr>
unop      := <op> <expr>
literal   := <number> <string>
ident		  := [a-zA-Z_][a-zA-Z_0-9]*
```
