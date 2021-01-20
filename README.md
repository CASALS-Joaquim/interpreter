#Monkey Interpreter

A Rust implementation of the Monkey Programming Language implemented in Golang in the `Writing an interpreter in Go` book.
As in the book, everything is written by hand, though there are still many bugs to fix.
One of these is the fact that it doesn't properly interact with the IO (stdin, stdout) I don't know exactly why (quite understandable since I'm just learning Rust).
The main functions like `tokenizer` are unsafe, which in case of Rust code should not be the case.