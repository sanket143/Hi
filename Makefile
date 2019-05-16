build:
	cd compiler && make

install:
	cp hi /usr/local/bin/

clean:
	cd compiler && make clean
