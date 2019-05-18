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
"="                     { return EQUATE; }
"("                     { return PAREN_O; }
")"                     { return PAREN_C; }
";"                     { return SEMICOLON; }

"print"                 { return PRINT; } 
"println"               { return PRINTLN; }
"var"                   { return VAR; }

[0-9]+                  { yylval.dval = atof(yytext); return NUMBER; }
[0-9]+.[0-9]+           { yylval.dval = atof(yytext); return NUMBER; }
[A-Za-z]+               {
                            yylval.str.text = yytext;
                            yylval.str.length = strlen(yytext);
                            return NAME;
                        }
\"([^\\\"]|\\.)*\"      {
                            yylval.str.text = yytext;
                            yylval.str.length = strlen(yytext);
                            return STRING;
                        }
"\n"
[ \t]
.                       { printf("Anything\n"); }

%%

