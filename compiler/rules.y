%{
#include <stdio.h>

void yyerror(char *s);
int yylex(void);
%}

%token NUMBER
%token ADD SUB MUL DIV MOD ABS
%token PRINT
%token EOL

%%

instructionlist     :
                    | instructionlist exp EOL { }
                    | instructionlist PRINT exp EOL { printf("%d\n", $3); }
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
