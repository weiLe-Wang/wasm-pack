# wasm-pack build

The `wasm-pack generate` command creates a new RustWasm project for you,
using [`cargo-generate`] under the hood.

It takes 2 optional parameters, template and name:

```
wasm-pack generate --template <template> --name <name>
```

The template will default to the `rustwasm/wasm-pack-template` and the name
will default to `hello-wasm`.

## Template

The `wasm-pack generate` command can be given an optional template argument, e.g.:

```
wasm-pack generate --template https://github.com/rustwasm/wasm-pack-template
```

The template can be an address to a git repo that contains a [`cargo-generate`]
template.

## Name

The `wasm-pack generate` command can be given and optional name argument, e.g.:

```
wasm-pack generate --name myproject
```
