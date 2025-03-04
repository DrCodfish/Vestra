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
git clone https://github.com/DrCodfish/Vestra
cd Vestra
cargo build --release
```

### System-Wide Installation

To make Vestra accessible system-wide, follow these steps:

1. **Install the Binary:**
   ```bash
   sudo cp target/release/vestra /usr/local/bin/vestra
   ```

2. **Ensure the Binary is Executable:**
   ```bash
   sudo chmod +x /usr/local/bin/vestra
   ```

3. **Check the PATH:**
   Verify that `/usr/local/bin` is included in your PATH by running:
   ```bash
   echo $PATH
   ```
   If not, add it by modifying your shell profile file (e.g., `~/.bashrc`, `~/.zshrc`, etc.):
   ```bash
   export PATH=$PATH:/usr/local/bin
   ```
   After adding it to your profile file, source the file to apply the changes:
   ```bash
   source ~/.bashrc
   ```

4. **Create a Symbolic Link (Optional but Recommended):**
   ```bash
   sudo ln -s /usr/local/bin/vestra /usr/bin/vestra
   ```

### Using Release Files

You can also download and use the pre-built binaries from the [Releases](https://github.com/DrCodfish/Vestra/releases) page. Follow these steps:

1. **Download the Latest Release:**

   Go to the [Releases](https://github.com/DrCodfish/Vestra/releases) page and download the appropriate `.zip` or `.tar.gz` file for your operating system.

2. **Extract the Archive:**

   For `.zip` files:
   ```bash
   unzip vestra-x.y.z.zip
   ```

   For `.tar.gz` files:
   ```bash
   tar -xvzf vestra-x.y.z.tar.gz
   ```

3. **Install the Binary:**

   ```bash
   sudo cp vestra-x.y.z/vestra /usr/local/bin/vestra
   ```

4. **Create a Symbolic Link (Optional):**

   ```bash
   sudo ln -s /usr/local/bin/vestra /usr/bin/vestra
   ```

### Example: Installing from a .tar.gz File in the Home Directory

If you have downloaded the `.tar.gz` file to your home directory (`~/`), follow these steps to install Vestra:

1. **Navigate to Your Home Directory:**

   ```bash
   cd ~
   ```

2. **Extract the .tar.gz File:**

   ```bash
   tar -xvzf vestra-x.y.z.tar.gz
   ```

3. **Navigate to the Extracted Directory:**

   ```bash
   cd vestra-x.y.z
   ```

4. **Install the Binary:**

   ```bash
   sudo cp vestra /usr/local/bin/vestra
   ```

5. **Ensure the Binary is Executable:**

   ```bash
   sudo chmod +x /usr/local/bin/vestra
   ```

6. **Create a Symbolic Link (Optional but Recommended):**

   ```bash
   sudo ln -s /usr/local/bin/vestra /usr/bin/vestra
   ```

### Running a Vestra Script

Create a simple Vestra script (example.vs):

```bash
set greeting = "Hello, Vestra!"
print greeting
```

Run it:

```bash
vestra run example.vs
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

To contribute, view the CONTRIBUTING.md

# **License**

This project is licensed under the MPL 2.0 License.
