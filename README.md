# ☀️ Sol

A dynamically-typed (for now) and procedural programming language.

## About

Sol is a lightweight language primarily designed for scripting environments. It uses a compile-to-JS approach at runtime, converting all Sol source-code into readable JavaScript code. (Bet you didn't see that coming >:D)

This JavaScript code is then executed on an embedded JavaScript machine.

## Syntax Highlighting
Sol now has a VS Code extension! (Warning: there are a ton of extensions that appear if you write Sol or Sol Tools, so instead write this (FOR NOW): publisher:"Joshua Colell")

### Why use Sol?

Sol is still not yet complete, but you could help making this programming language an actual thing! So why not use it? It's a simple language to understand with a bunch of examples.

### How do I compile Sol?

So I made an effort to make a python script to make the compiling process easier. You only need Python and Rust installed, the script would to the rest! It is going to ask you for admin privilages (on Windows) or root (on Linux or Mac). Some dependencies in Sol need admin/root to compile (like rquickjs). After compiling the project, you can start making changes to the project! Just making a pull request helps me develop this project faster.

## Here's an example:

```rust
fn hello(keyword) {
    println("Hello, " + keyword + "!")
}

hello("World")
```

#### Ok so... where do I start?
### You can start of from the (non-existing) Documentation!

## TODO

> This is the list of things I needed (and want) to add to Sol:

TODO | WIP | Implemented | Released in latest release
:------------ | :-------------| :-------------| :-------------
Built-In Modules | ✔️ | ✔️ | ✔️
Build script | ✔️ | ✔️ | ✔️
Comments | ✔️ |  ❌ | ❌
Package Manager & Custom Modules | ✔️ |  ❌ | ❌
