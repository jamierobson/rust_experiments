# About
The goal of this small project is to manage the pieces I have in my guitar playing repetoire. I want to be able to hold a list of the numbers I have learned over time, with reference material, an indication of if I feel like I am still actively learning the piece, and other such metadata.

I'd like to be able to:

- Gather my reference material for a particular number, if I'd like to practice it
- Add and modify pieces
- Give me a random number to play, if I need inspiration

I mainly work with C# in the enterprise, which influences my development and code principles. I want to learn Rust. Once I've finished, I'd like invite rustaceons to help me identify which ideas don't hold in Rust, and what should be changed to be more "rust-like"

# Observations
This is a living document of the things that I learn as I go through this project.

## Lessons learned
- A class seems to be a [struct with methods implemented on it](https://medium.com/@jimmco/classes-in-rust-c5b72c0f0a4c)
- Defining a method reminds me a little of describing extension methods: passing in the first argument as `(self: &Self)`.
- There are two means to return a string from a literal, by the seems of things. "something" seems to be a &String type.Use `"something".to_string()`, or better `String::from("something")`.
- I'm guessing that the definition of `MethodTest();` hints to the compiler that there are no fields on the type, or that it allows the definition of a parameterless constructor
- Have to reference modules directly from `main.rs` to include them in the compilation chain. I'll explore this [lib.rs idea](https://users.rust-lang.org/t/vscode-rust-debugging/57881/2) later

## Opinions
- I dislike the shorthand `pub`, `fn`, `mod` shorthand tendencies for keywords. I hope that these are front-loaded.
- Having to list every file in a `mod.rs` file is really very tedious.
- Referencing types to use is a pain, having to navigate to the exact file, to pick the single type I want.
  - Can I at least pick multiple types out of one file?
  - Why bother with mod.rs if I anyway need to navigate the whole file structure?
  - Please tell me it's possible eventually to do something like "Find this one type name in core::domain" or something. Please?!

## Things I'm looking to implement
- DDD. It seems like I'm not a million miles away according to [this](https://github.com/jdomenechb/rust-ddd-example)
- DI is important to me, I hope that [this crate](https://medium.com/geekculture/dependency-injection-in-rust-3822bf689888) can help.
- [Traits](https://doc.rust-lang.org/book/ch10-02-traits.html) (Get it? I want to _implement_ traits)