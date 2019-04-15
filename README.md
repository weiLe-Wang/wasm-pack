# 📦✨  wasm-pack
> Your favorite rust -> wasm workflow tool!

[![Build Status](https://travis-ci.com/rustwasm/wasm-pack.svg?branch=master)](https://travis-ci.com/rustwasm/wasm-pack)
[![Build status](https://ci.appveyor.com/api/projects/status/iv1qtnqtv168ef8h?svg=true)](https://ci.appveyor.com/project/ashleygwilliams/wasm-pack-071k0)
[![crates.io](https://meritbadge.herokuapp.com/wasm-pack)](https://crates.io/crates/wasm-pack)


This tool seeks to be a one-stop shop for building and working with rust-
generated WebAssembly that you would like to interop with JavaScript, in the
browser or with Node.js. `wasm-pack` helps you build rust-generated
WebAssembly packages that you could publish to the npm registry, or otherwise use
alongside any javascript packages in workflows that you already use, such as [webpack]
or [greenkeeper].

[bundler-support]: https://github.com/rustwasm/team/blob/master/goals/bundler-integration.md#details
[webpack]: https://webpack.js.org/
[greenkeeper]: https://greenkeeper.io/

This project is a part of the [rust-wasm] group. You can find more info by
visiting that repo!

[rust-wasm]: https://github.com/rustwasm/team

![demo](demo.gif)

## 🔮 Prerequisities

This project requires Rust 1.30.0 or later.

- [Development Environment](https://rustwasm.github.io/wasm-pack/book/prerequisites/index.html)
- [Installation](https://rustwasm.github.io/wasm-pack/installer)
- [Project Setup](https://rustwasm.github.io/wasm-pack/book/project-setup/index.html)

## 🎙️ Commands

- [`generate`](https://rustwasm.github.io/wasm-pack/book/commands/generate.html): Generate a new RustWasm project using a template
- [`build`](https://rustwasm.github.io/wasm-pack/book/commands/build.html): Generate an npm wasm pkg from a rustwasm crate
- [`test`](https://rustwasm.github.io/wasm-pack/book/commands/test.html): Run browser tests
- [`pack` and `publish`](https://rustwasm.github.io/wasm-pack/book/commands/pack-and-publish.html): Create a tarball of your rustwasm pkg and/or publish to a registry

## 📝 Logging

`wasm-pack` uses [`env_logger`] to produces logs when `wasm-pack` runs.

To configure your log level, use the `RUST_LOG` environment variable. For example:

```
RUST_LOG=info wasm-pack build
```

[`env_logger`]: https://crates.io/crates/env_logger

## 👯 Contributing

Read our [guide] on getting up and running for developing `wasm-pack`, and
check out our [contribution policy].

[guide]: https://rustwasm.github.io/wasm-pack/book/contributing.html
[contribution policy]: CONTRIBUTING.md

## 🤹‍♀️ Governance

This project is part of the [rustwasm Working Group].

This project was started by [ashleygwilliams] and is co-maintained by [ashleygwilliams], [drager] and the Rust Wasm Working Group Core Team.

[ashleygwilliams]: https://github.com/ashleygwilliams
[drager]: https://github.com/drager
[rustwasm Working Group]: https://github.com/rustwasm/team

## ⚡ Quickstart Guide

1. Install this tool: `cargo install wasm-pack`
1. Run `wasm-pack generate`.
1. `cd hello-wasm`
1. Run `wasm-pack build`, optionally, pass a path to a dir or a scope (see above for details)
1. This tool generates files in a `pkg` dir
1. To publish to npm, run `wasm-pack publish`. You may need to login to the
   registry you want to publish to. You can login using `wasm-pack login`.
