# OTDR Viewer

A desktop application for viewing fiber-optic reflectometry results (`.sor` files, Telcordia SR-4731 standard). The UI
is built with Rust/WebAssembly on top of Leptos, wrapped in a native Tauri 2 shell.

## Stack

| Layer         | Technology                                       | Purpose                                                                        |
|---------------|--------------------------------------------------|--------------------------------------------------------------------------------|
| UI framework  | [Leptos](https://leptos.dev/) 0.8 (CSR)          | Reactive UI, compiled to WebAssembly                                           |
| Language      | Rust (edition 2021)                              | Both frontend (WASM) and native backend                                        |
| Desktop shell | [Tauri](https://tauri.app/) 2                    | Native window, filesystem access, OS integration, frontend build orchestration |
| Styling       | [Stylance](https://github.com/basro/stylance-rs) | Scoped CSS modules, hashed class names generated at build time                 |

## Running the project

Install Rust, the Tauri CLI and Tauri's system dependencies by following
the [Tauri prerequisites guide](https://v2.tauri.app/start/prerequisites/) for your OS, then
install [stylance-cli](https://crates.io/crates/stylance-cli) per its own docs.

Start the app in development mode:

```bash
cargo tauri dev
```

This builds and serves the frontend (running the `stylance` CLI as a pre-build step to generate `styles/bundle.css`) on
`http://localhost:1420`, and, in parallel, builds and runs the native Tauri backend that opens a window pointing at that
address. Both sides hot-reload: changes in `src/` (Leptos) and `src-tauri/src/` (native backend) are picked up
automatically.

## Building the project

```bash
cargo tauri build
```

This produces a production frontend bundle (`dist/`) and packages a native installer/binary for the current OS into
`src-tauri/target/release/bundle/`.

## License

Licensed under the [Apache License 2.0](./LICENSE).