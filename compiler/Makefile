CC = gcc
HI_BIN = ../hi
LYOBJ = lex.yy.o y.tab.o
YTAB = y.tab.c y.tab.h

build: $(LYOBJ)
	$(CC) -o $(HI_BIN) $(LYOBJ) -lfl

$(LYOBJ): lex.yy.c y.tab.c
	$(CC) -c lex.yy.c y.tab.c -lfl

lex.yy.c: $(YTAB) rules.l
	lex rules.l

$(YTAB): rules.y
	yacc -d rules.y

clean:
	rm -rf $(LYOBJ) lex.yy.c y.tab.c y.tab.h
