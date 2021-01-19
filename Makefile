GCC_BIN ?= $(shell which g++)
CARGO_BIN ?= $(shell which cargo)

run: clean build
    ./build/interpreter
clean:
    $(CARGO_BIN) clean
    rm -f ./build/interpreter
build:
    $(CARGO_BIN) build --release
    $(GCC_BIN) -o ./build/interpreter ./c_cxx/main.cpp -Isrc  -L. -l:target/release/libinterpreter.so
