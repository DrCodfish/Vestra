# **Vestra Programming Language**

Vestra is a lightweight, flexible, and powerful interpreted language designed for simplicity and seamless integration with Rust. Inspired by Lua and built from the ground up in Rust, Vestra aims to provide a clean, efficient scripting experience both as an embeddable language and as a standalone tool.

## **Why Choose Vestra?**

- **Easy to Embed**: Vestra was built with Rust in mind — embedding it in your Rust projects is straightforward and efficient.
- **Simple and Elegant Syntax**: Inspired by Lua, Vestra prioritizes readability and simplicity without sacrificing power.
- **Flexible Execution**: Vestra works just as well as an embedded scripting language or as a standalone interpreter.
- **Powerful Control Structures**: Vestra supports conditionals, loops, and variable assignments in a clear and expressive format.
- **Dynamic Typing**: Work with strings, numbers, booleans, and nil values without unnecessary complexity.

## **Getting Started**

### Prerequisites

- Rust (Specific edition and version compatibility not yet known)
- Cargo (Rust's package manager)

### Installation

Clone the Vestra repository and build the project:

```bash
git clone https://github.com/DrCodfish/vestra-interpreter.git
cd vestra-interpreter
cargo build --release
```

### Running a Vestra Script

Create a simple Vestra script (example.vss):

```bash
set greeting = "Hello, Vestra!"
print greeting
```

Run it:

```bash
cargo run -- example.vss
```

Output:

```bash
Hello Vestra!
```

## **Language Features**

### Variables

```bash
print "Vestra is awesome!"
```

### Conditions

```bash
set lang = "Vestra"
if lang == "Vestra"
    print "You're using Vestra!"
else
    print "This isn’t Vestra!"
```

### Loops

```bash
set count = 0
while count < 5
    print count
    set count = count + 1
```

## **Contributing**

Contributions are welcome! Please open an issue or submit a pull request.

## **License**

This project is licensed under the MPL 2.0 License.
