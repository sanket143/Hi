HI_BIN = ../hi
PARSER_A = ../parser/parser.a
CC = g++ -std=c++11 -Wall -Wextra

build: $(PARSER_A) 
	$(CC) main.cpp $(PARSER_A) -o $(HI_BIN)

clean:
	rm -rf $(HI_BIN) 
