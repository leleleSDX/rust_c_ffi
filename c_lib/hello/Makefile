CC = gcc
CFLAGS = -Wall -Wextra -fPIC
SOURCES = src/hello.c
OBJECTS = $(SOURCES:.c=.o)
INCLUDE = -Iinclude
TARGET = libhello.so

.PHONY: all clean

all: $(TARGET)

$(TARGET): $(OBJECTS)
	$(CC) -shared -o $@ $^

%.o: %.c
	$(CC) $(CFLAGS) $(INCLUDE) -c $< -o $@

clean:
	rm -f $(OBJECTS) $(TARGET)
