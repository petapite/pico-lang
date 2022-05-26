# ☀️ Sol

A dynamically-typed (for now) and procedural programming language.

## About

Sol is a lightweight language primarily designed for scripting environments. It uses a compile-to-JS approach at runtime, converting all Sol source-code into readable JavaScript code. (Bet you didn't see that coming >:D)

This JavaScript code is then executed on an embedded JavaScript machine.

## Syntax Highlighting
Sol now has a VS Code extension! (Warning: there are a ton of extensions that appear if you write Sol or Sol Tools, so instead write this (FOR NOW): publisher:"Joshua Colell")

### Why not write JavaScript?

JavaScript is also a great language for scripting. By writing a language on top of JavaScript, we can take advantage of it's more powerful features such as promises, async/await without having to implement it from scratch. The QuickJS engine is also incredibly lightweight and fast enough for 99% of use-cases.

## Example

```rust
fn fib(n) {
    if n < 2 {
        return n
    }

    return fib(n - 1) + fib(n - 2)
}

println(fib(27)) // -> 196418
```

#### Ok so... where do I start?
### You can start of from the Wiki!

## Limitations

During testing and development, these are some of the limitations I have found so far:

> A checked box marks the limitation as resolved or fixed.

- [x] A recursive `fib(n)` function that exceeds `fib(27)` causes a runtime stack-overflow. Realistically, nobody is going to be using this much recursion but it's normally a good benchmark for raw recursive performance.

## TODO

> This is the list of things I needed (and want) to add to Sol:

TODO | WIP | Implemented | Released in latest version
:------------ | :-------------| :-------------| :-------------
Built-In Modules | ✔️ | ✔️ | ✔️
Comments | ✔️ |  ❌ | ❌
Package Manager & Custom Modules | ✔️ |  ❌ | ❌
