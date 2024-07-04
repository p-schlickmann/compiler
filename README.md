# Simple Rust Compiler

Based on the [Dragon Book](https://en.wikipedia.org/wiki/Compilers:_Principles,_Techniques,_and_Tools), this compiler
was crafted for the course "Introduction to Compilers", taught at the Federal University of Santa Catarina in the
first semester of 2024. This project includes the first two phases of compilation:
1. Lexical analysis
2. Syntactical analysis

## Running the Compiler
There are several `.lsi` files with expressions to be analyzed by our compiler. Some of these files include invalid expressions; this is on purpose so we can ensure our lexer and parser work correctly.
To test it yourself, run:
```bash
cargo run examples/0-valid-small.lsi
```
Make sure cargo is installed. [Installation link](https://doc.rust-lang.org/cargo/getting-started/installation.html)

## Grammar
The grammar for this compiler was provided by our teacher, Daniel Santana de Freitas. Our task was to refactor the grammar, remove left recursion and make it LL(1). This was the result:

```
MAIN -> STMT
MAIN -> FLIST
MAIN -> ''

FLIST -> FDEF FLIST1

FLIST1 -> FDEF
FLIST1 -> ''

FDEF -> def id ( PARLIST ) { STMTLIST }

PARLIST -> int id PARLIST1
PARLIST -> ''

PARLIST1 -> , PARLIST
PARLIST1 -> ''

STMT -> int id ;
STMT -> ATRIBSTMT ;
STMT -> PRINTSTMT ;
STMT -> RETURNSTMT ;
STMT -> IFSTMT
STMT -> ;
STMT -> { STMTLIST }

ATRIBSTMT -> id ATRIBSTMT1

ATRIBSTMT1 -> = EXPR
ATRIBSTMT1 -> ( PARLISTCALL )

PARLISTCALL -> id PARLISTCALL1
PARLISTCALL -> ''

PARLISTCALL1 -> , PARLISTCALL
PARLISTCALL1 -> ''
 
PRINTSTMT -> print EXPR

RETURNSTMT -> return

IFSTMT -> if ( EXPR ) STMT IFSTMT1

IFSTMT1 -> else STMT
IFSTMT1 -> ''

STMTLIST -> STMT STMTLIST1

STMTLIST1 -> STMTLIST
STMTLIST1 -> ''

EXPR -> NUMEXPR EXPR1

EXPR1 -> < NUMEXPR
EXPR1 -> > NUMEXPR
EXPR1 -> == NUMEXPR
EXPR1 -> ''

NUMEXPR -> TERM NUMEXPR1

NUMEXPR1 -> + TERM NUMEXPR1
NUMEXPR1 -> - TERM NUMEXPR1
NUMEXPR1 -> ''

TERM -> FACTOR TERM1

TERM1 -> * FACTOR TERM1
TERM1 -> ''

FACTOR -> num
FACTOR -> ( NUMEXPR )
FACTOR -> id
```

## Parse Table
For simplicity reasons, our parse table is hardcoded in [table.rs](./src/table.rs). The parse table was generated using the tool: https://jsmachines.sourceforge.net/machines/ll1.html. Big thank you to whoever created that!
 
