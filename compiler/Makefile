CC = gcc
HI_BIN = ../hi
LYOBJ = rules.tab.o lex.yy.o
YTAB = rules.tab.c rules.tab.h

build: $(LYOBJ)
	gcc -o $(HI_BIN) rules.tab.o lex.yy.o -lfl

$(LYOBJ): lex.yy.c rules.tab.c
	$(CC) -c lex.yy.c rules.tab.c -lfl

lex.yy.c: tokens.lex
	flex tokens.lex

$(YTAB): rules.y
	bison -d rules.y

clean:
	rm -rf $(LYOBJ) $(YTAB) lex.yy.c
