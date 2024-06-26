# Define the target executable name
TARGET := main

# Define the source file
SRC := main.rs

# Default target to build and run the program
all: run

# Build the Rust program
build:
	rustc $(SRC)

# Run the compiled program
run: build
	./$(TARGET).exe

# Clean up the compiled files
clean:
	rm -f $(TARGET).exe $(TARGET)
