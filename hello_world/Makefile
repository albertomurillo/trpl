.PHONY: all build run clean

all: build

build: main
main: main.rs
	rustc main.rs

run: build
	./main

clean:
	rm -rf main
