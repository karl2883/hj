# hj
hj will be a general-purpose, compiled programming language, made for fun. It will not be garbage-collected and will produce assembly by itself (only for one Intel-architecture). The compiler will be written in Rust. This hj project is a continuation of a much simpler, [interpreted version](https://github.com/trubiso/hj) by [Trubiso](https://github.com/trubiso) and myself.
## Goals
The main goal is for the language to be able to solve any problem from [Advent of Code](https://adventofcode.com/).
## Current state
A basic lexer that produces tokens from the source has been made. The compiler will read from a given file and output all of the tokens when debug mode (-d) is enabled.
Also, a basic parser has been made. The parser transforms the list of tokens into an AST (abstract syntax tree).
