# pico

A dynamically-typed (for now) and procedural programming language.

## Update !!!!!!!!!!!!!
Sol is going to have a ***HUGE*** re-design, but this means it's to be dead for a while. I'm currently working in [picopite](https://github.com/JoshuaColell/picopite/)! It's an operating system built from the ground up, and yes, that means I'm creating my own kernel for it. (Of course, it's gonna have linux support)

I'm also planning to make Sol a big language, that also means I'm using pico in picopite!

***Update 2:*** *Sol is now renamed to pico!*

***Current picopite state:*** *Early Stage*

Bye, for now, pico :wave:

## About

pico is a lightweight language primarily designed for scripting environments. It uses a compile-to-JS approach at runtime, converting all pico source-code into readable JavaScript code. (Bet you didn't see that coming >:D)

This JavaScript code is then executed on an embedded JavaScript machine.

### Why use pico?

pico is still not yet complete, but you could help making this programming language an actual thing! So why not use it? It's a simple language to understand with a bunch of examples.

### How do I compile pico?

So I made an effort to make a python script to make the compiling process easier. You only need Python and Rust installed, the script would to the rest! It is going to ask you for admin privilages (on Windows) or root (on Linux or Mac). Some dependencies in pico need admin/root to compile (like rquickjs). After compiling the project, you can start making changes to the project! Just making a pull request helps me develop this project faster.

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

> This is the list of things I needed (and want) to add to pico:

TODO | WIP | Implemented | Released in latest release
:------------ | :-------------| :-------------| :-------------
Built-In Modules | ✔️ | ✔️ | ✔️
Build script | ✔️ | ✔️ | ✔️
Comments | ✔️ |  ❌ | ❌
Package Manager & Custom Modules | ✔️ |  ❌ | ❌
