CC = gcc
TARGET = parser.a
LYOBJ = rules.tab.o lex.yy.o
YACC_R = rules.tab.h rules.tab.c

$(TARGET): $(LYOBJ) 
	ar rvs $@ $(LYOBJ)

$(LYOBJ): $(YACC_R) lex.yy.c
	$(CC) -c lex.yy.c rules.tab.c -lfl

lex.yy.c: tokens.l
	flex tokens.l

$(YACC_R): rules.y
	bison -d rules.y

clean:
	rm -rf $(LYOBJ) $(YACC_R) $(TARGET) lex.yy.c
