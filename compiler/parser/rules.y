%{
#include <stdio.h>
#include <string.h>

#include "base.hpp"
#define YYDEBUG 1

void yyerror(char *s);
int yylex(void);
extern char *yytext;
%}

%token NUMBER STRING NAME
%token ADD SUB MUL DIV MOD ABS EQUATE
%token CLT CGT CEQ CLEQ CGEQ CNEQ CSEQ
%token PAREN_O PAREN_C
%token DOUBLE_QUOTE SINGLE_QUOTE
%token SEMICOLON
%token PRINTLN PRINT VAR
%token TRUE FALSE

%union {
    struct str_params {
        char *text;
        int length;
    };
    struct str_params str;
    double dval;
    bool bval;
    int ival;
}

%type <str> STRING NAME
%type <dval> NUMBER
%type <ival> CLT CGT CEQ CLEQ CGEQ CNEQ CSEQ

%type <bval> boolexp
%type <ival> comparator
%type <dval> exp factor term

%%

instructionlist     :
                    | instructionlist exp
                    | instructionlist print
                    | instructionlist variable
                    | instructionlist boolexp
                    ;

print               : PRINT PAREN_O exp PAREN_C SEMICOLON { compiler::printd($3); }
                    | PRINT PAREN_O STRING PAREN_C SEMICOLON {
                        printf("%.*s", ($3).length - 2, ($3).text + 1 );
                    }
                    | PRINT PAREN_O boolexp PAREN_C SEMICOLON {
                        printf("%s", $3 ? "true" : "false");
                    }
                    | PRINT PAREN_O NAME PAREN_C SEMICOLON {
                        char var[($3).length];
                        int n = sprintf(var, "%.*s", ($3).length, ($3).text);
                        compiler::print(var);
                    }
                    | PRINTLN PAREN_O exp PAREN_C SEMICOLON { compiler::printdn($3); }
                    | PRINTLN PAREN_O STRING PAREN_C SEMICOLON {
                        printf("%.*s\n", ($3).length - 2, ($3).text + 1 );
                    }
                    | PRINTLN PAREN_O boolexp PAREN_C SEMICOLON {
                        printf("%s\n", $3 ? "true" : "false");
                    }
                    | PRINTLN PAREN_O NAME PAREN_C SEMICOLON {
                        char var[($3).length];
                        int n = sprintf(var, "%.*s", ($3).length, ($3).text);
                        compiler::println(var);
                    }
                    | PRINTLN PAREN_O PAREN_C SEMICOLON {
                        printf("\n");
                    }
                    ;

boolexp             : exp comparator exp {
                        $$ = compiler::numComparator($1, $2, $3);
                    }
                    | TRUE { $$ = true; }
                    | FALSE { $$ = false; }
                    ;

comparator          : CLT
                    | CGT
                    | CEQ
                    | CLEQ
                    | CGEQ
                    | CNEQ
                    | CSEQ
                    ;

variable            : NAME EQUATE exp SEMICOLON {
                        char var[($1).length];
                        int n = sprintf(var, "%.*s", ($1).length, ($1).text);
                        compiler::addNumVar(var, $3);
                    }
                    | VAR NAME EQUATE exp SEMICOLON {
                        char var[($2).length];
                        int n = sprintf(var, "%.*s", ($2).length, ($2).text);
                        compiler::addNumVar(var, $4);
                    }
                    | NAME EQUATE STRING SEMICOLON {
                        char var[($1).length];
                        char value[($3).length - 2];
                        int n;

                        n = sprintf(var, "%.*s", ($1).length, ($1).text);
                        n = sprintf(value, "%.*s", ($3).length - 2, ($3).text + 1);
                        compiler::addStringVar(var, value);
                    }
                    | VAR NAME EQUATE STRING SEMICOLON {
                        char var[($2).length];
                        char value[($4).length - 2];
                        int n;

                        n = sprintf(var, "%.*s", ($2).length, ($2).text);
                        n = sprintf(value, "%.*s", ($4).length - 2, ($4).text + 1);
                        compiler::addStringVar(var, value);
                    }
                    | NAME EQUATE boolexp SEMICOLON {
                        char var[($1).length];
                        int n;

                        n = sprintf(var, "%.*s", ($1).length, ($1).text);
                        compiler::addBoolVar(var, $3);
                    }
                    | VAR NAME EQUATE boolexp SEMICOLON {
                        char var[($2).length];
                        int n;

                        n = sprintf(var, "%.*s", ($2).length, ($2).text);
                        compiler::addBoolVar(var, $4);
                    }
                    ;

exp                 : factor            { $$ = $1; }
                    | exp ADD factor    { $$ = $1 + $3; }
                    | exp SUB factor    { $$ = $1 - $3; }
                    ;

factor              : term              { $$ = $1; }
                    | factor MOD term   {
                        if($3){
                            if($1 - (int)$1 || $3 - (int)$3){
                                fprintf(stderr, "Error: Decimals in '%' Operation\n");
                                exit(1);
                            } else {
                                $$ = (int)$1 % (int)$3;
                            }
                        } else {
                            fprintf(stderr, "Error: Division by 0\n");
                            exit(1);
                        }
                    }
                    | factor MUL term   { $$ = $1 * $3; }
                    | factor DIV term   {
                        if($3){
                            $$ = $1 / $3;
                        } else {
                            fprintf(stderr, "Error: Division by 0\n");
                            exit(1);
                        }
                    }
                    ;

term                : NUMBER
                    | NAME {
                        char var[($1).length];
                        int n = sprintf(var, "%.*s", ($1).length, ($1).text);
                        $$ = compiler::getNumValue(var);
                    }
                    | PAREN_O exp PAREN_C { $$ = $2; }
                    | boolexp {
                        $$ = $1;
                    }
                    ;

%%

void yyerror(char *s){
    fprintf(stderr, "error: %s\n", s);
}
