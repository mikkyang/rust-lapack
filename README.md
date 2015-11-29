# RBLAS

Rust bindings and wrappers for LAPACK (Linear Algebra PACKage).

## Overview

RLAPACK wraps each external call in a trait with the same name (but capitalized).
This trait contains a single static method, of the same name. These traits are
generic over the four main types of numbers LAPACK supports: `f32`, `f64`,
`Complex32`, and `Complex64`.

For example the functions `sgesv_`, `dgesv_`, `cgesv_`, and
`zgesv_` are called with the function `Gesv::gesv`.

Additionally, RLAPACK introduces a `Matrix` trait to shorten calls to these LAPACK
functions, implemented for types that implement matrix-like characteristics.

[Documentation](http://mikkyang.github.io/rust-lapack/doc/rlapack/index.html)

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
rlapack = "0.0.2"
```

and this to your crate root:
```rust
extern crate rlapack;
```
