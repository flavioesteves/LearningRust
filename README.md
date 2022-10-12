# Useful Docs:
  * [https://www.rust-lang.org/](https://www.rust-lang.org/)
  * [https://crates.io/](https://crates.io/)
  * [https://doc.rust-lang.org/book/](https://doc.rust-lang.org/book/)

# Section 1:Getting Started
01. Course Introduction
02. What is Rust
  * Memory safe
  * No Null
  * No Exceptions
  * Modern package manager(Cargo)
  * No Data Races
03. Installing Rust
04. Setting Up the Development Environnment
05. Cargo


# Section 2:Manual Memory Management
06. Code for this section
07. Introduction 
  * The Stack
  * The Heap
  * Pointers
  * Smart Pointers
08. The Stack
  * It's a special region of the process memory that stores variables by each function.
  * For every function call a new stack frame is allocated on top of the current one.
  * The size of every variable on the stack has to be known at compile time. 
  * When a function exits it's stack frame is released. 
09. What is the heap (and how is it different from the Stack)
  * It's a region of the process memory that is NOT automatically managed.
  * It has no size restrictions.
  * It's accessible by any function. anywhere in the program.
  * Heap allocations are expensive and we should avoid them when possible. 
  * IMPORTANT: always deallocate memory form the HEAP to avoid memory leaks.
10. Smart Pointers
  * Is a solution to the heap issues regarding the memory leaks.
11. Explore the Memory Layout in GDB
  * Box:: - Is a type of smart pointer in RUST


# Section 3:Building a Command Line Application
12. Code for this section 
13. Introduction
  * Building a Command Line Application to practice those concepts: 
Function, Basic Data Types, Standard Library, Memory Ownership
14. Basic Data Types
  * Booleans
    - bool:1 bit
  * Integers: u8,i8 --> u16,i16 --> u32,i32 --> u64,u64 --> u128,i128
    - "u" = unsigned: cannot hold negative values
    - "i" = signed: cand hold negative values
    - "NUMBER" = number of bits
    * Special kind of numbers: usize & isize: are architecture dependent types
      - They are 32 bits in 32-bits architecture
      - They are 64 bits in 64-bits architecture
  * Floats
    - f32: 32 bit
    - f64: 64 bit(double the precision)
  * Characters
    - char: always 4 bit for each char
15. Functions
16. Macros
  * '!' this is the special char for a macro (example: println!('This is a macro not a function call!!'))
  * Macro definition on the [book](https://doc.rust-lang.org/book/ch19-06-macros.html)
  * Macro definitoin is more complex than functions definitions because consists into wite code to write code.
  * Good tool for Macros: cargo-expand
17. Mutability
18. The Standard Library
  * [https://doc.rust-lang.org/std/](https://doc.rust-lang.org/std/)
19. Ownership (!IMPORTANT)
  * Exists 3 rules in Rust:
  * 1. Each value in Rust is owned by a variable.
  * 2. When the owner goes out of scope, the value will be deallocated.
  * 3. There can only be ONE owner at a time.
20. References and Borrowing (!IMPORTANT)
  * "&" before the type means Borrowing in RUST
  * "&mut" allow to be mutable. 
  * Important rule for references in the same scope:
    - Many immutable references
    - Single mutable reference 
21. Explore the Ownership and Borrowing in GDB
22. Finishing Touches
  * Unwrap method() : if the result of the function is an error then the program will terminate.

# Section 4:Building a HTTP Server From Scratch
23. Code for this section
24. Introduction
  * Learning Rust > Server Performance
25. The HTTP Protocol and the Architecture of Our Server
  * HTTP/1.1: L7 Protocol, Sent over TCP, Message based
  * Request / Response
  * Server: TCP Listener, HTTP Parser, Handler
26. Structs
  * Struct is a custom type, kind like a class in object oriented languages (Aggregates data)
  * Structs have blocks, the data block keyword "struct", implementation block keyword "impl"

