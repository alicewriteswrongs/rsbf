# rsbf

This is a [Brainfuck](https://en.wikipedia.org/wiki/Brainfuck) interpreter that
I wrote in Rust as well as a (currently very sketchy) setup for running it in
the browser via wasm.

To build and run the rust part:

```sh
cargo build
```

then you can

```sh
cargo run -- --help
```

there are some example programs in `examples/` which you can run, for instance
you could run a 'hello world' program like so:

```sh
cargo run -- examples/hello_world.brainfuck
```

There's also a (crappy) web frontend. After building the rust code you can run
it like so:

```sh
pnpm i
npm run build:wasm
npm run dev
```
