Vestra Programming Language

Vestra is a lightweight, flexible, and powerful interpreted language designed for simplicity and seamless integration with Rust. Inspired by Lua and built from the ground up in Rust, Vestra aims to provide a clean, efficient scripting experience both as an embeddable language and as a standalone tool.

Why Choose Vestra?

Easy to Embed: Vestra was built with Rust in mind — embedding it in your Rust projects is straightforward and efficient.

Simple and Elegant Syntax: Inspired by Lua, Vestra prioritizes readability and simplicity without sacrificing power.

Flexible Execution: Vestra works just as well as an embedded scripting language or as a standalone interpreter.

Powerful Control Structures: Vestra supports conditionals, loops, and variable assignments in a clear and expressive format.

Dynamic Typing: Work with strings, numbers, booleans, and nil values without unnecessary complexity.

Getting Started

Prerequisites

Rust (latest stable edition)

Cargo (Rust’s package manager)

Installation

Clone the Vestra repository and build the project:

git clone https://github.com/yourusername/vestra.git
cd vestra
cargo build --release

Running a Vestra Script

Create a simple Vestra script (example.ves):

set greeting = "Hello, Vestra!"
print greeting

Run it:

cargo run -- example.ves

Output:

Hello, Vestra!

Language Features

Variables

set name = "Vestra"
set version = 1.0
set is_cool = true

Printing

print "Vestra is awesome!"

Conditionals

set lang = "Vestra"
if lang == "Vestra"
    print "You're using Vestra!"
else
    print "This isn’t Vestra!"

Loops

set count = 0
while count < 5
    print count
    set count = count + 1

Contributing

We welcome contributions! Feel free to open issues, submit pull requests, or suggest new features.

License

Vestra is open-source and available under the MIT License.
