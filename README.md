# **Vestra Programming Language**

Vestra is a lightweight, flexible, and powerful interpreted language designed for simplicity and seamless integration with Rust. Inspired by Lua and built from the ground up in Rust, Vestra aims to provide a clean, efficient scripting experience both as an embeddable language and as a standalone tool.

## **Why Choose Vestra?**

- **Easy to Embed**: Vestra was built with Rust in mind — embedding it in your Rust projects is straightforward and efficient.
- **Simple and Elegant Syntax**: Inspired by Lua, Vestra prioritizes readability and simplicity without sacrificing power.
- **Flexible Execution**: Vestra works just as well as an embedded scripting language or as a standalone interpreter.
- **Powerful Control Structures**: Vestra supports conditionals, loops, and variable assignments in a clear and expressive format.
- **Dynamic Typing**: Work with strings, numbers, booleans, and nil values without unnecessary complexity.

### **System-wide instilation**

1. **Navigate to Your Home Directory:**

   ```bash
   cd ~
   ```

2. **Extract the .tar.gz File:**

   ```bash
   tar -xvzf vestra-vx.y.z.tar.gz
   ```

3. **Navigate to the created directory**

   ```bash
   cd vestra-vx.y.z
   ```

4. **Install the Binary:**

   ```bash
   sudo cp release/vestra /usr/release/vestra
   ```
   If you get a
   ```bash
   cp: cannot create regular file '/<usr>/release/vestra': No such file or directory
   ```
   Then run
   ```bash
   sudo mkdir -p /usr/local/release/
   sudo cp release/vestra /usr/release/vestra
   ```
   to create the directory and try again.

6. **Ensure the Binary is Executable:**

   ```bash
   sudo chmod +x /usr/local/release/vestra
   ```

7. **Create a Symbolic Link (Optional but Recommended):**

   ```bash
   sudo ln -s /usr//release/vestra /usr/bin/vestra
   ```
8. **Add it to your path**

   a.
   ```bash
   cd
   ```
   You can use any editor instead of nano to edit your .bashrc
   ```bash
   nano ~/.bashrc
   ```
   b. Add the Export Line:
   ```bash
   export PATH=/usr/local/release/:$PATH
   ```
   c. For nano, press CTRL + O to save and CTRL + X to exit.
   d. Apply the changes
   ```bash
   source ~/.bashrc
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
