%option noyywrap

%{
#include <stdio.h>

#include "rules.tab.h"
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
                            yylval.str.text = yytext;
                            yylval.str.length = strlen(yytext);
                            return STRING;
                        }
"\n"                    { return EOL; }

"print"                 { return PRINT; } 

[ \t]                   { }
.                       { printf("Anything\n"); }

%%

