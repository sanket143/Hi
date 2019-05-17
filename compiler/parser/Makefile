CC = g++
TARGET = parser.a
LYOBJ = rules.tab.o lex.yy.o
YACC_R = rules.tab.h rules.tab.c
INCLUDE = ../include

$(TARGET): $(LYOBJ) 
	ar rvs $@ $(LYOBJ)

$(LYOBJ): $(YACC_R) $(INCLUDE)/* lex.yy.c
	$(CC) -c lex.yy.c rules.tab.c -lfl -I $(INCLUDE)

lex.yy.c: tokens.l
	flex tokens.l

$(YACC_R): rules.y
	bison -d rules.y

clean:
	rm -rf $(LYOBJ) $(YACC_R) $(TARGET) lex.yy.c
