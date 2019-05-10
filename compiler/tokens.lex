%{
#include "rules.tab.h"
%}

%%

"+"     { return ADD; }
"-"     { return SUB; }
"*"     { return MUL; }
"/"     { return DIV; }
"%"     { return MOD; }
[0-9]+  { yylval = atoi(yytext); return NUMBER; }
"\n"    { return EOL; }
[ \t]   { }
.       { printf("Anything\n"); }

%%
