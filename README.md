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

### Write a workflow that uses the function

To run our function on Homestar, we need to include in a workflow. Our workflow tasks contain metadata, proofs, run input<args, func>, nonce, operation and resource.

<details>
<summary>View workflow JSON</summary>

```json
{
  "tasks": [
    {
      "cause": null,
      "meta": {
        "fuel": 18446744073709552000,
        "memory": 4294967296,
        "time": 100000
      },
      "prf": [],
      "run": {
        "input": {
          "args": [
            "<svg viewBox=\"0 0 100 100\" xmlns=\"http://www.w3.org/2000/svg\">\r\n    <rect fill=\"#f90\" height=\"100\" rx=\"4\" width=\"100\"/>\r\n    <rect fill=\"#ffb13b\" height=\"50\" rx=\"4\" width=\"50\"/>\r\n    <rect fill=\"#de8500\" height=\"50\" rx=\"4\" width=\"50\" x=\"50\" y=\"50\"/>\r\n    <g fill=\"#f90\">\r\n    <circle cx=\"50\" cy=\"18.4\" r=\"18.4\"/>\r\n    <circle cx=\"72.4\" cy=\"27.6\" r=\"18.4\"/>\r\n    <circle cx=\"81.6\" cy=\"50\" r=\"18.4\"/>\r\n    <circle cx=\"72.4\" cy=\"72.4\" r=\"18.4\"/>\r\n    <circle cx=\"50\" cy=\"81.6\" r=\"18.4\"/>\r\n    <circle cx=\"27.6\" cy=\"72.4\" r=\"18.4\"/>\r\n    <circle cx=\"18.4\" cy=\"50\" r=\"18.4\"/>\r\n    <circle cx=\"27.6\" cy=\"27.6\" r=\"18.4\"/>\r\n    </g>\r\n    <path d=\"m63.086 18.385c0-7.227-5.859-13.086-13.1-13.086-7.235 0-13.096 5.859-13.096 13.086-5.1-5.11-13.395-5.11-18.497 0-5.119 5.12-5.119 13.408 0 18.524-7.234 0-13.103 5.859-13.103 13.085 0 7.23 5.87 13.098 13.103 13.098-5.119 5.11-5.119 13.395 0 18.515 5.102 5.104 13.397 5.104 18.497 0 0 7.228 5.86 13.083 13.096 13.083 7.24 0 13.1-5.855 13.1-13.083 5.118 5.104 13.416 5.104 18.513 0 5.101-5.12 5.101-13.41 0-18.515 7.216 0 13.081-5.869 13.081-13.098 0-7.227-5.865-13.085-13.081-13.085 5.101-5.119 5.101-13.406 0-18.524-5.097-5.11-13.393-5.11-18.513 0z\"/>\r\n    <path d=\"m55.003 23.405v14.488l10.257-10.253c0-1.812.691-3.618 2.066-5.005 2.78-2.771 7.275-2.771 10.024 0 2.771 2.766 2.771 7.255 0 10.027-1.377 1.375-3.195 2.072-5.015 2.072l-10.234 10.248h14.489c1.29-1.28 3.054-2.076 5.011-2.076 3.9 0 7.078 3.179 7.078 7.087 0 3.906-3.178 7.088-7.078 7.088-1.957 0-3.721-.798-5.011-2.072h-14.49l10.229 10.244c1.824 0 3.642.694 5.015 2.086 2.774 2.759 2.774 7.25 0 10.01-2.75 2.774-7.239 2.774-10.025 0-1.372-1.372-2.064-3.192-2.064-5.003l-10.255-10.252v14.499c1.271 1.276 2.084 3.054 2.084 5.013 0 3.906-3.177 7.077-7.098 7.077-3.919 0-7.094-3.167-7.094-7.077 0-1.959.811-3.732 2.081-5.013v-14.499l-10.235 10.252c0 1.812-.705 3.627-2.084 5.003-2.769 2.772-7.251 2.772-10.024 0-2.775-2.764-2.775-7.253 0-10.012 1.377-1.39 3.214-2.086 5.012-2.086l10.257-10.242h-14.485c-1.289 1.276-3.072 2.072-5.015 2.072-3.917 0-7.096-3.18-7.096-7.088s3.177-7.087 7.096-7.087c1.94 0 3.725.796 5.015 2.076h14.488l-10.256-10.246c-1.797 0-3.632-.697-5.012-2.071-2.775-2.772-2.775-7.26 0-10.027 2.773-2.771 7.256-2.771 10.027 0 1.375 1.386 2.083 3.195 2.083 5.005l10.235 10.252v-14.488c-1.27-1.287-2.082-3.053-2.082-5.023 0-3.908 3.175-7.079 7.096-7.079 3.919 0 7.097 3.168 7.097 7.079-.002 1.972-.816 3.735-2.087 5.021z\" fill=\"#fff\"/>\r\n    <path d=\"m5.3 50h89.38v40q0 5-5 5h-79.38q-5 0-5-5z\"/>\r\n    <path d=\"m14.657 54.211h71.394c2.908 0 5.312 2.385 5.312 5.315v17.91c-27.584-3.403-54.926-8.125-82.011-7.683v-10.227c.001-2.93 2.391-5.315 5.305-5.315z\" fill=\"#3f3f3f\"/>\r\n    <g fill=\"#fff\" stroke=\"#000\" stroke-width=\".5035\">\r\n    <path d=\"m18.312 72.927c-2.103-2.107-3.407-5.028-3.407-8.253 0-6.445 5.223-11.672 11.666-11.672 6.446 0 11.667 5.225 11.667 11.672h-6.832c0-2.674-2.168-4.837-4.835-4.837-2.663 0-4.838 2.163-4.838 4.837 0 1.338.549 2.536 1.415 3.42.883.874 2.101 1.405 3.423 1.405v.012c3.232 0 6.145 1.309 8.243 3.416 2.118 2.111 3.424 5.034 3.424 8.248 0 6.454-5.221 11.68-11.667 11.68-6.442 0-11.666-5.222-11.666-11.68h6.828c0 2.679 2.175 4.835 4.838 4.835 2.667 0 4.835-2.156 4.835-4.835 0-1.329-.545-2.527-1.429-3.407-.864-.88-2.082-1.418-3.406-1.418-3.23 0-6.142-1.314-8.259-3.423z\"/>\r\n    <path d=\"m61.588 53.005-8.244 39.849h-6.85l-8.258-39.849h6.846l4.838 23.337 4.835-23.337z\"/>\r\n    <path d=\"m73.255 69.513h11.683v11.664c0 6.452-5.226 11.678-11.669 11.678-6.441 0-11.666-5.226-11.666-11.678v-16.501h-.017c0-6.447 5.241-11.676 11.667-11.676 6.459 0 11.683 5.225 11.683 11.676h-6.849c0-2.674-2.152-4.837-4.834-4.837-2.647 0-4.82 2.163-4.82 4.837v16.501c0 2.675 2.173 4.837 4.82 4.837 2.682 0 4.834-2.162 4.834-4.827v-.012-4.827h-4.834z\"/>\r\n    </g>\r\n</svg>\r\n"
          ],
          "func": "rasterize"
        },
        "nnc": "",
        "op": "wasm/run",
        "rsc": "ipfs://bafybeidret56l7ongppvvgv6nostulnx7pt6amtekapnb2wi3n3bvllxna"
      }
    }
  ]
}
```
</details>

Our `run.input.args` is an SVG string and `run.input.func` specifies the function name that we want to call from the Wasm component.

The `rsc` field contains a URI for our Wasm component. We upload our Wasm component to our local IPFS node.

```sh
ipfs add --cid-version 1 output/svg_to_png.component.wasm
```

Adding the Wasm component to IPFS returns the CID `bafybeidret56l7ongppvvgv6nostulnx7pt6amtekapnb2wi3n3bvllxna` for our component.

### Test it with Homestar

We start the Homestar runtime with:

```sh
homestar start
```

In a separate terminal window, we run the workflow:

```sh
homestar run -w examples/cli/workflow.json
```

The CLI reports workflow information when it starts running a workflow. On first run, we won't see results, but on a second run the CLI will report a replayed receipt that contains our PNG. (A future version of the CLI will include a means for checking the status of active and completed workflows.)

After running the workflow a second time, we check the replayed receipts section. We copy the `cid` from the receipts computed section and retrieve the associated receipt from IPFS:

```sh
ipfs dag get bafyrmiebgewvljpqvzbwcyop7viv6sbuhyu2pvkbgfer44uv3giq3vuaia
```

Resulting in a JSON receipt:

```json
{
  "iss": null,
  "meta": {
    "op": "rasterize"
  },
  "out": [
    "ok",
    {
      "/": {
        "bytes": "<truncated>"
      }
    }
  ],
  "prf": [],
  "ran": {
    "/": "bafyrmie26jbi5i23gefxbm74ce36fjmk4ieepth2t3scy2rdzyq4kignyq"
  }
}
```

We can extract the PNG from the receipt's `out` field using `jq`. The bytes are encoded as base64, so we decode and output them as a PNG file:

```sh
ipfs dag get bafyrmiebgewvljpqvzbwcyop7viv6sbuhyu2pvkbgfer44uv3giq3vuaia | jq  -r '.out[1]["/"]["bytes"]' | base64 -d > foo.png
```

### Error reporting and handling

Our initial version of the function dangerously unwraps, which could cause of function to fail if we pass in an invalid SVG or `resvg` is unable to perform the rasterization.

It would be better to log an error when something goes wrong, which requires access to IO through the host environment. WASI is a system interface that we'll use to log errors from our Wasm component, using the `wasi-logging` package for WIT.

To manage this WIT dependency, we'll use a package manager for WIT called `wit-deps`:

```sh
cargo install wit-deps-cli
```

We'll start by creating `wit/deps.toml`:

```toml
logging = "https://github.com/WebAssembly/wasi-logging/archive/main.tar.gz"
```

This package exposes a `logging` world, which we can import from our `wit/host.wit`:

```wit
package fission:svg-to-png@0.1.0

world svg-to-png {
  import wasi:logging/logging;

  export rasterize: func(input: string) -> list<u8>
}
```

With these changes, we're now able to log messages from our Wasm component:

```rust
#[cfg(target_arch = "wasm32")]
use wasi::logging::logging::{log, Level};

impl Guest for Component {
    fn rasterize(input: String) -> Vec<u8> {
        #[cfg(target_arch = "wasm32")]
        log(Level::Info, "fission:svg-to-png", "rasterizing SVG to PNG");

        rasterize(input)
    }
}
```

The Homstar runtime will capture and include these log messages in its own logging:

```sh
ts=2024-02-13T22:49:44.904834Z level=info target=homestar_wasm::wasmtime::host::helpers message="rasterizing SVG to PNG" subject=wasm_execution category=fission:svg-to-png
```

We can now add error reporting to our Wasm component. We update our `rasterize` function to return a result and match on the `Ok` and `Err` cases:

```rust
match rasterize(input) {
    Ok(png) => {
        #[cfg(target_arch = "wasm32")]
        log(
            Level::Info,
            "fission:svg-to-png",
            "PNG generated successfully!",
        );

        png
    }
    Err(err) => {
        #[cfg(target_arch = "wasm32")]
        log(Level::Error, "fission:svg-to-png", err.to_string().as_str());

        panic!();
    }
}
```

In the `Ok` case, we log an info message and return our PNG bytes. In the `Err` case we log an error level message.

We've added a workflow with a broken workflow in `examples/cli/broken_workflow.json`. When we run this workflow, we see an error message in the Homestar logs:

```sh
ts=2024-02-13T23:03:19.368227Z level=error target=homestar_wasm::wasmtime::host::helpers message="SVG data parsing failed cause expected \'=\' not \'b\' at 1:9" subject=wasm_execution category=fission:svg-to-png
```

This message was passed up from the error reported by `resvg`.
