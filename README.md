# Skeið

**a linear algebra library for game graphics and physics**

## Is this ready to use in my project?

There are more rational alternatives, but it's getting there.

The library has tests to verify that the bulk of the functionality present works as expected,
and the library has many working features that can be used if you're interested.

However take note that the API is not and will not be stable until 1.0,
for which there is no planned release date.
Until then, I can and will be changing load-bearing parts of the library until I'm happy enough
with the architecture to make a commitment to it.

Also make sure that you read `LICENSE` and understand the license terms and disclaimers of
liability before you make use of this library.

## What features does Skeið provide right now?

- Generic `Matrix` and `Vector` structs
- Vector-Vector addition, subtract, dot-product, and cross-product
- Vector-Scalar addition, subtraction, multiplication, and division
- Matrix-Matrix addition and subtraction
- Matrix-Scalar multiplication
- Matrix-Vector multiplication
- WebAssembly compatibility

## What features are planned for the future?

- Generic `Quaternion` struct
- Quaternion math
- Methods specifically for creating transformation and projection matrices
- Portable SIMD (when stable in rustc)

## How is Skeið pronounced?

While the name can be anglicized to **skeid**,
the original Old Norse word would have been pronounced like the English word **scathe**,
which is also how the name of this library should be pronounced.

## What does Skeið mean?

A **skeið** is a type of Scandinavian viking-age warship.
They were the largest type of longship, having 30 or more rowing benches.

The name **skeið** means "slider" in Old Norse, possibly referring to the high speed of these ships.
This word is cognate with the modern English word **sheath**,
via Old English word **scēaþ** meaning the same as the modern english,
and having the Proto-Indo-European root **\*skei-** meaning "to cut or split".

It is named as such because I thought it's a cool word,
because other components of the TimberWolf game engine have Norse-inspired names,
and because I want this math to move easily and quickly just like the eponymous ship.

## How to Use

This is a Rust library, available through crates.io.
To install in your project, add it to your Cargo.toml as a dependency:

```toml
[dependencies]
skeid = "0.21.0"
```

Or run the following command from within your project directory:

```shell
cargo add skeid
```
