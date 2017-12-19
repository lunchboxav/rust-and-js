# Introduction to Rust

## Adityo Pratomo [@kotakmakan](http://www.twitter.com/kotakmakan)

---

# This Talk is Also Available at

[github.com/froyoframework/rust-intro/slide](http://www.github.com/froyoframework/rust-intro)

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

# Thank You

[didit@froyo.co.id](didit@froyo.co.id)

