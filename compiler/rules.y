%{
#include <stdio.h>
#include <string.h>

void yyerror(char *s);
int yylex(void);
extern char *yytext;
%}

%token NUMBER STRING
%token ADD SUB MUL DIV MOD ABS
%token PAREN_O PAREN_C
%token DOUBLE_QUOTE SINGLE_QUOTE
%token PRINT
%token EOL

%union {
    struct str_params {
        char *text;
        int length;
    };
    struct str_params str;
    int val;
}

%type <str> STRING
%type <val> NUMBER
%type <val> exp
%type <val> term
%type <val> factor

%%

instructionlist     :
                    | instructionlist exp EOL { }
                    | instructionlist PRINT PAREN_O exp PAREN_C EOL { printf("%d\n", $4); }
                    | instructionlist PRINT PAREN_O STRING PAREN_C EOL {
                        printf("%.*s\n", ($4).length - 2, ($4).text + 1 );
                    }
                    ;

exp                 : factor            { $$ = $1; }
                    | exp ADD factor    { $$ = $1 + $3; }
                    | exp SUB factor    { $$ = $1 - $3; }
                    ;

factor              : term              { $$ = $1; }
                    | factor MUL NUMBER { $$ = $1 * $3; }
                    | factor DIV NUMBER { $$ = $1 / $3; }
                    ;

term                : NUMBER            { $$ = $1; }
                    | ABS term          { $$ = $2 >= 0 ? $2 : - $2; }
                    ;

%%

void yyerror(char *s){
    fprintf(stderr, "error: %s\n", s);
}
