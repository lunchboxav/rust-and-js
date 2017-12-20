# Introduction to Rust

## Adityo Pratomo [@kotakmakan](http://www.twitter.com/kotakmakan)

---

# This Talk is Also Available at

[github.com/lunchboxav/rust-and-js](https://github.com/lunchboxav/rust-and-js)

---

# Background

- Chief Academic Officer at Framework
- Chief Technology Officer at Labtek Indie
- Certified Unity Developer
- Loves to code, play guitar, PC game and watch *wrestling*

---

# Framework

- Providing software development course, training and workshop
- For Company and general public
- Based in Bintaro

![](https://scontent.cdninstagram.com/t51.2885-19/s150x150/12501964_776556112474284_1583292585_a.jpg)

---

# Labtek Indie

- Rapid Prototyping As A Service
- Based in Bandung

![](https://d1qb2nb5cznatu.cloudfront.net/startups/i/321497-a83fd6324ed6c76e05dc137012e59bef-medium_jpg.jpg?buster=1389436901)

---

# Today's Topic

- Rust and JS
    + Backend
    + Frontend
- They work hand-in-hand complimentarily
- In general case, Rust can boost JS' performance    

---

# Backend: NodeJS + Rust

---

# FFI: The Glue

- FFI (Foreign Function Interface) is a mechanism that allows different programming languages to communicate between each other
- This includes calling different functions and calling services from one to another
- Uses underlying C from these languages
- Rust has implemented FFI, so we can use it
- FFI is also available in other language
- NodeJS has an `ffi` package

---

# NodeJS and Rust via FFI

![](https://lh3.googleusercontent.com/-V1AhzICeGPi2Oju6fKBdre8904FSwTUGnrksE_niR3FrLCbGLStaar0cL1oeIfctVYWLo1uJbN2rFAyqymy=w1651-h889)

---

# Benefits

- NodeJS' nature is a single threaded operation
- NodeJS is very good for writing network application, especially for parallel I/O operation executed asynchronously
- However, isn't very good at CPU intensive operation -> can create blocking
- Rust can help speed up this, to prevent blocking
- Rust operates on a lower level, thus is more performant

---

# General Steps: Rust

1. Write code that exposes external function to be called from NodeJS
2. Handles C specific data properly inside that function
3. Compiles code as lib -> Generates DLL/dylib

---

# General Steps: NodeJS

1. Include `ffi` library
2. Load the resulted DLL/dylib
3. Call the external function using `ffi`

---

# Example #1: Fibonacci Generator

- `1 1 2 3 5 8 13 ...`
- Involves a lot of recursion
- CPU intensive

---

# Example #2: Text Randomizer

- We will create an application that cipher a text by shifting the letter x to the right
- Example for x=3: 
    + `ab` -> `de`
- Lighter than fibonacci, but might be more real-world case
- Showcases detail of FFI when handling String in Rust

---

# Frontend: Rust to JS via WASM

---

# WASM

- WebAssembly(WASM) is a binary format and compile target for web browser
- You can compile C/C++/Rust to `.wasm` and it can be run on the browser
- The code is faster and smaller than JS
- WASM won't replace JS anytime soon, but they can coexist and cooperate

---

# How WASM Works

![](https://2r4s9p1yi1fa2jd7j43zph8r-wpengine.netdna-ssl.com/files/2017/02/04-01-langs09.png)

---

# How WASM Works

![](https://2r4s9p1yi1fa2jd7j43zph8r-wpengine.netdna-ssl.com/files/2017/02/04-02-langs08.png)

---

# Compiling Rust to WASM

- Rust code can be compiled into `.wasm` by using:
    + Emscripten compile target for Rust (managed via `rustup`), will act as a bridge from Rust to Emscripten
    + Emscripten SDK that will compile Rust to asm.js and convert that to WebAssembly

---

# Compiling Rust to WASM

![](https://2r4s9p1yi1fa2jd7j43zph8r-wpengine.netdna-ssl.com/files/2017/02/04-03-toolchain07.png)

---

# Interfacing WASM and JS

- We still need JS code that will act as glue between JS and WASM
- the WASM module can then be loaded in JS
- Afterwards, we JS can call the function declared in WASM module

---

# Example: Hello World

- Simple Rust program that sends String
- The String will then be read by JS in browser client
- It will then be printed out in console

---

# Current State of WASM in Rust

- Rust target compilation is available not only using emscripten, but also using Rust's built in backend `wasm32-unknown-unknown` (*new!*)
- To interact with DOM, you can use external Crate, such as `stdweb`
- Not yet prime for production, but it's coming

---

# Thank You

[didit@froyo.co.id](didit@froyo.co.id)

