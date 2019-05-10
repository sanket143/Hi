%{
#include "rules.tab.h"
#include <string.h>
%}

%%

"+"                     { return ADD; }
"-"                     { return SUB; }
"*"                     { return MUL; }
"/"                     { return DIV; }
"%"                     { return MOD; }
"("                     { return PAREN_O; }
")"                     { return PAREN_C; }
"\""                    { return DOUBLE_QUOTE; }
"\'"                    { return SINGLE_QUOTE; }
[0-9]+                  { yylval.val = atoi(yytext); return NUMBER; }
\"([^\\\"]|\\.)*\"      {
                            yylval.str = yytext;
                            printf("%d\n", strlen(yylval.str));
                            return STRING;
                        }
"\n"                    { return EOL; }

"print"                 { return PRINT; } 

[ \t]                   { }
.                       { printf("Anything\n"); }

%%

int main(int argc, char **argv){
    if(argc > 1){
        if(!(yyin = fopen(argv[1], "r"))){
            perror(argv[1]);
            return 1;
        }
    } else {
        fprintf(stderr, "hi: no input file\n");
        return 1;
    }

    yyparse();
    return 0;
}
