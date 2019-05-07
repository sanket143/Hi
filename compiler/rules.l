%{

%}
%%
sanket |
is |
the |
best |
developer   { printf("%s: yeah right", yytext); }

.           ;

%%

void main(){
    yylex();
}
