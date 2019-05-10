build:
	cd compiler && make

install:
	cp hi /usr/local/bin/

clean:
	rm hi
	cd compiler && make clean
