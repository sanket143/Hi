HI_BIN = ../hi
HEADER_A = ./include/header.a
PARSER_A = ./parser/parser.a
CC = g++ -std=gnu++11 -Wall -Wextra

build:
	cd include && make
	cd parser && make
	$(CC) main.cpp $(PARSER_A) $(HEADER_A) -o $(HI_BIN)

clean:
	cd include && make clean
	cd parser && make clean
	rm -rf $(HI_BIN) 
