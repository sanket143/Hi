build:
	cd parser && make
	cd compiler && make

install:
	cp hi /usr/local/bin/

clean:
	cd parser && make clean
	cd compiler && make clean
