%{
#include <stdio.h>
#include <string.h>

#include "variable.hpp"
#include "print.hpp"

void yyerror(char *s);
int yylex(void);
extern char *yytext;
%}

%token NUMBER STRING NAME
%token ADD SUB MUL DIV MOD ABS EQUATE
%token PAREN_O PAREN_C
%token DOUBLE_QUOTE SINGLE_QUOTE
%token PRINTLN PRINT VAR
%token EOL

%union {
    struct str_params {
        char *text;
        int length;
    };
    struct str_params str;
    double dval;
}

%type <str> STRING
%type <str> NAME
%type <dval> NUMBER
%type <dval> exp
%type <dval> factor
%type <str> variable

%%

instructionlist     :
                    | instructionlist exp EOL
                    | instructionlist print EOL
                    | instructionlist variable EOL
                    ;

print               :
                    | PRINT PAREN_O exp PAREN_C { compiler::printd($3); }
                    | PRINT PAREN_O STRING PAREN_C {
                        printf("%.*s", ($3).length - 2, ($3).text + 1 );
                    }
                    | PRINT PAREN_O NAME PAREN_C {
                        char var[($3).length];
                        int n = sprintf(var, "%.*s", ($3).length, ($3).text);
                        compiler::print(var);
                    }
                    | PRINTLN PAREN_O exp PAREN_C { compiler::printdn($3); }
                    | PRINTLN PAREN_O STRING PAREN_C {
                        printf("%.*s\n", ($3).length - 2, ($3).text + 1 );
                    }
                    | PRINTLN PAREN_O NAME PAREN_C {
                        char var[($3).length];
                        int n = sprintf(var, "%.*s", ($3).length, ($3).text);
                        compiler::println(var);
                    }
                    ;

variable            :
                    | VAR NAME EQUATE exp {
                        char var[($2).length];
                        int n = sprintf(var, "%.*s", ($2).length, ($2).text);
                        compiler::addNumVar(var, $4);
                    }
                    | VAR NAME EQUATE STRING {
                        char var[($2).length];
                        char value[($4).length - 2];
                        int n;

                        n = sprintf(var, "%.*s", ($2).length, ($2).text);
                        n = sprintf(value, "%.*s", ($4).length - 2, ($4).text + 1);
                        compiler::addStringVar(var, value);
                    }
                    ;

exp                 : factor            { $$ = $1; }
                    | exp ADD factor    { $$ = $1 + $3; }
                    | exp SUB factor    { $$ = $1 - $3; }
                    ;

factor              : NUMBER            { $$ = $1; }
                    | factor MUL NUMBER { $$ = $1 * $3; }
                    | factor DIV NUMBER { $$ = $1 / $3; }
                    ;

%%

void yyerror(char *s){
    fprintf(stderr, "error: %s\n", s);
}
