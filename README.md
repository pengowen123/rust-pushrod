# rust-pushrod

[![Build Status](https://travis-ci.org/KenSuenobu/rust-pushrod.svg?branch=master)](https://travis-ci.org/KenSuenobu/rust-pushrod)
[![](https://img.shields.io/crates/d/rust-pushrod.svg)](https://crates.io/crates/rust-pushrod)
[![docs.rs for rust-pushrod](https://docs.rs/rust-pushrod/badge.svg)](https://docs.rs/rust-pushrod)

**Cross Platform UI Widget Library for Rust.**

Draws inspiration from lots of GUI libraries.

## (Ever Evolving) Screenshot of Sample

[![](docs/sample-0.2.9.gif)](docs/sample-0.2.9.gif)

## Philosophy

The reason I created this library instead of extending another library was that
I wanted to keep these specific design ideas in mind:

- Maintainable with little effort
- Easily extensible
- Lightweight enough to run on minimalist hardware
- **Easy to use and understand**

These design ideas are critical.  **Keep it simple.  Keep it stupid simple.**

## Prerequisites for Pushrod

Pushrod requires the following minimum versions:

| Library | Version |
| ------- | ------- |
| piston_window | 0.89 |
| pistoncore-glfw_window | 0.49 |
| lazy_static | 1.3 |
| rust | 2018 |

## Runnable Examples

```
cargo run --example simple
```

This will only test window-related events with mouse interaction: mouse enter, mouse exit, mouse click, mouse
pointer move, and mouse scroll.

