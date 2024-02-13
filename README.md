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

### Write it to compile to a Wasm component

We want to compile our function as a Wasm component. To generate WIT (WebAssembly Interface Types) for our component, we write a WIT world that defines our interface.

The interface is in `wit/host.wit` and it looks like this:

```wit
package fission:svg-to-png@0.1.0

world svg-to-png {
  export rasterize: func(input: string) -> list<u8>
}
```

The package ID is structured as `namespace:name@semver-version`. The `semver-version` is optional.

The world name matches our module name, but this is not required. A module may contain more than one world, but we only need one here.

Our `rasterize` export is declared as a function in our `svg-to-png` world. The [WIT types](https://component-model.bytecodealliance.org/design/wit.html#built-in-types) correspond to our Rust types. `String` in Rust is string in WIT and `Vec<u8>` in Rust is `list<u8>` in WIT.

In `src/lib.rs`, we use `wit-bindgen::generate!` macro to generate bindings for our world to be implemented by the `Guest` in Rust. The WIT interface expects our `rasterize` function, and we just need to add it to the `Guest` implementation.

Lastly, we can build and componentize our Wasm component:

```sh
cargo build --target wasm32-unknown-unknown
wasm-tools component new target/wasm32-unknown-unknown/debug/svg_to_png.wasm -o output/svg_to_png.component.wasm
```

We can verify that our Wasm component has the correct WIT interface using `wasm-tools`:

```sh
wasm-tools component wit output/svg_to_png.component.wasm
```

It matches!

### Optimization interlude

We noticed that our Wasm component is 44.66 MB! That seems a bit much, so we decided to prematuraly optimize. 😇

We set our release profile to an `opt-level` of `s` to optimize for size and `lto` to `true` to enable link time optimizations.

Then we ran a new set of commands that includes `wasm-opt` to optimize even more! We build this time with the release target.

```sh
cargo build --target wasm32-unknown-unknown --release
wasm-opt -Os target/wasm32-unknown-unknown/release/svg_to_png.wasm -o output/svg_to_png.wasm
wasm-tools component new output/svg_to_png.wasm -o output/svg_to_png.component.wasm
```

These changes bring our Wasm component size down to 3.07 MB. A nice improvement!

### Next Steps

- Write a workflow that uses the function
- Test it with Homestar
- Error reporting and handling
