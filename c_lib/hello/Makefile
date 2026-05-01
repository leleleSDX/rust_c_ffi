CC = gcc
AR = ar
ARFLAGS = rcs
CFLAGS = -Wall -Wextra -fPIC
#SOURCES = src/hello.c
SRC_DIR = src/
SRC_FILE = hello.c
INCLUDE = -Iinclude
BUILD_DIR = ../build
#OBJECTS = $(SOURCES:.c=.o)
OBJECTS = $(SRC_FILE:%.c=$(BUILD_DIR)/%.o)
TARGET_DIR = ../target
TARGET_FILE = libhello.a
#TARGET = libhello.a
TARGET = $(TARGET_DIR)/$(TARGET_FILE)

.PHONY: all clean

all: $(TARGET)

#$(TARGET): $(OBJECTS)
#	$(CC) -shared -o $@ $^

# static builc
$(TARGET): $(OBJECTS)
	$(AR) $(ARFLAGS) $@ $^

$(BUILD_DIR)/%.o: $(SRC_DIR)/%.c
	$(CC) $(CFLAGS) $(INCLUDE) -c $< -o $@

clean:
	rm -f $(OBJECTS) $(TARGET)

print:
	@echo $(OBJECTS)
	@echo $(TARGET)
