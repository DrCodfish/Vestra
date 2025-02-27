# **Vestra Programming Language**

Vestra is a lightweight, flexible, and powerful interpreted language designed for simplicity and seamless integration with Rust. Inspired by Lua and built from the ground up in Rust, Vestra aims to provide a clean, efficient scripting experience both as an embeddable language and as a standalone tool.

# **Why Choose Vestra?**

**Easy to Embed**: Vestra was built with Rust in mind — embedding it in your Rust projects is straightforward and efficient.

**Simple and Elegant Syntax**: Inspired by Lua, Vestra prioritizes readability and simplicity without sacrificing power.

**Flexible Execution**: Vestra works just as well as an embedded scripting language or as a standalone interpreter.

**Powerful Control Structures**: Vestra supports conditionals, loops, and variable assignments in a clear and expressive format.

**Dynamic Typing**: Work with strings, numbers, booleans, and nil values without unnecessary complexity.

# **Getting Started**

## If you built from source (ie: **the current git clone**) then you will need:

- Rust (Specific edition and version compatability not yet known)
- Cargo (Rust's package manager)

## This will not be a problem once options for a package manager or pre built executables becomes avaible. A list of these types is provided: 
- .appimage (linux)
- .deb or .rpm (linux)
- .exe (windows)
- .dmg (mac)

**Docker may not be avaible for some time**

# Pre-Alpha Install:

Clone the Vestra repository and build the project:

```bash
git clone https://github.com/DrCodfish/target.zip
cd vestra
cargo build --release
```

##**Running a Vestra script**

**Create a simple Vestra script (example.vss):**
```bash
set greeting = "Hello, Vestra!"
print greeting
```
**Run it**
```bash
cargo run -- example.vs
```
**Output:**
```bash
Hello Vestra!
```

##**Language Features**

**Variables**
```bash
print "Vestra is awesome!"
```

**Conditions**
```bash
set lang = "Vestra"
if lang == "Vestra"
    print "You're using Vestra!"
else
    print "This isn’t Vestra!"
```

**Loops**
```bash
set count = 0
while count < 5
    print count
    set count = count + 1
```








