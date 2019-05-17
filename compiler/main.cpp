#include <iostream>

extern "C" {
    int yylex();
    void yyerror(const char *);
}

extern int yyparse();
extern FILE *yyin;

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
