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

We are going to try out: https://docs.rs/nsvg/latest/nsvg/

### Try it out in Rust

We started with the [nsvg](https://docs.rs/nsvg/latest/nsvg/) example code and modified it to take an SVG string and return bytes.

The [nsvg](https://docs.rs/nsvg/latest/nsvg/) library can scale the output image, but in our initial tests we left the scale unchanged.

Our integration test has an associated test that shows we can convert from an SVG string to bytes.

### Next Steps

- Write it to compile to Wasm
- Write a workflow that uses the function
- Test it with Homestar
