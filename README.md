# rust-and-js
Describing various ways how Rust and JS can work together. 

## Backend
In the backend, we have Node and Rust working together via FFI. Rust can improve performance of NodeJS code, thanks to its lower level access. This can help in preventing CPU-intensive task to block the whole application.
Examples:
  - Fibonacci series generator (CPU intensive, uses recursion)
  - String randomizer (showcases how FFI work with String, *hint: it's not straightforward*)

## Frontend
TBC

## Slides
Accompanying slide that come along with the codes. It is intended to be presented when discussing this topic.

Presentation History:
1. Rabu Rust no.4 - 20 December 2017, Mozilla Community Space Jakarta
