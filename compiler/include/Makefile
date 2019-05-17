CC = g++
TARGET = header.a

$(TARGET): *.cpp *.hpp
	$(CC) -c *.cpp
	ar rvs $@ *.o

clean:
	rm -rf $(TARGET) *.o
