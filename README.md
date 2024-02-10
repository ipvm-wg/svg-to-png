# svg-to-png

Welcome to the Homestar function tutorial! @avivash, @quinnwilton, and @bgins are writing a custom function, and this tutorial shares our thought process while developing the function.

If you just want the function, [build](#build) instructions are below. But if you are interested in process, keep on reading.

## Build

**Build with a Wasm target:**

```sh
cargo build --target wasm32-unknown-unknown
```

**Componentize the Wasm binary:**

```sh
wasm-tools component new target/wasm32-unknown-unknown/debug/svg_to_png.wasm -o output/svg_to_png.wasm
```

[cli]: examples/cli/README.md

## Walkthrough

### Goal

Our goal is to write a function that rasterizes SVGs to PNG images. The function will be part of a componentized Wasm module. We should be able to run the function in the Homestar runtime.

### Finding a library

We want a Rust library that will do the conversion for us. Our execution environment is Wasmtime. We won't have GPUs or storage available to us, so we want a simple library.

Our initial attempt was to use [nsvg](https://docs.rs/nsvg/latest/nsvg/), but it wraps the C library [nanosvg](https://github.com/memononen/nanosvg), which gives us compilation headaches. Rust and Clang use different calling conventions in their Wasm ABI, so this prevents us from easily compiling the wrapped C code to Wasm.

Next, we tried [resvg](https://docs.rs/resvg/latest/resvg/). It worked great!

### Try it out in Rust

We wrote a `rasterize` function using [resvg](https://docs.rs/resvg/latest/resvg/) that takes an SVG string and returns bytes.

We would like to add a scale parameter to our function, but decided to hold off until we have it working as a Wasm function running in Homestar.

Our integration test shows we can convert from an SVG string to bytes.

### Compile to Wasm

A quick test of `cargo build --target wasm32-unknown-unknown` successfully compiled to Wasm.

Note that we needed to add a lib `crate-type` of `cdylib` for the Wasm target and `rlib` for the Rust target used by our integration test.

### Next Steps

- Write it to compile to Wasm
- Write a workflow that uses the function
- Test it with Homestar
