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
OBJECTS = $(SRC_FILE:%.c=%.o)
TARGET_DIR = ../target
TARGET = libhello.a

.PHONY: static clean rebuild

static: $(TARGET_DIR)/$(TARGET)

#$(TARGET): $(OBJECTS)
#	$(CC) -shared -o $@ $^

# static build
$(TARGET_DIR)/$(TARGET): $(BUILD_DIR)/$(OBJECTS)
	$(AR) $(ARFLAGS) $@ $^

$(BUILD_DIR)/%.o: $(SRC_DIR)/%.c
	$(CC) $(CFLAGS) $(INCLUDE) -c $< -o $@

main: $(SRC_DIR)/main.o
$(SRC_DIR)/main.o: $(SRC_DIR)/main.c
	$(CC) $(CFLAGS) $(INCLUDE) -c $< -o $@

clean:
	rm -f $(BUILD_DIR)/$(OBJECTS)
	rm -f $(TARGET_DIR)/$(TARGET)

rebuild: 
	$(MAKE) clean 
	$(MAKE) static

print:
	@echo $(BUILD_DIR)
	@echo $(OBJECTS)
	@echo $(TARGET_DIR)
	@echo $(TARGET)
