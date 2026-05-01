CC = gcc
AR = ar
ARFLAGS = rcs
CFLAGS = -Wall -Wextra -fPIC
SOURCES = src/hello.c
INCLUDE = -Iinclude
BUILD_DIR = ../build
#OBJECTS = $(SOURCES:.c=.o)
OBJECTS = $(SOURCES:%=$(BUILD_DIR)/%.o)
TARGET_DIR = ../target
#TARGET = libhello.a
TARGET = $(TARGET_DIR)/libhello.a

.PHONY: all clean

all: $(TARGET)

#$(TARGET): $(OBJECTS)
#	$(CC) -shared -o $@ $^

# static builc
$(TARGET): $(OBJECTS)
	$(AR) $(ARFLAGS) $@ $^

%.o: %.c
	$(CC) $(CFLAGS) $(INCLUDE) -c $< -o $@

clean:
	rm -f $(OBJECTS) $(TARGET)
