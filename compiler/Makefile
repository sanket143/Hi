HI_BIN = ../hi
PARSER_A = ./parser/parser.a
CC = g++ -std=gnu++11 -Wall -Wextra

build: $(PARSER_A) 
	cd parser && make
	$(CC) main.cpp $(PARSER_A) -o $(HI_BIN)

clean:
	cd parser && make clean
	rm -rf $(HI_BIN) 
