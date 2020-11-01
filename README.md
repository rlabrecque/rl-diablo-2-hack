# RL Diablo 2 Hack

⚠ This project is largely abandoned. ⚠

This was a fun little sideproject I created while I was replaying Diablo 2, it was intended to be a playground to learn more Rust, and to gain more insight into how Diablo 2 works under the hood.
This should only ever be used in Singleplayer, there is absolutely no protection against using it on Battle.NET and will likely result in getting banned instantly.
It's completely Rust based 🦀.

It's split into 3 packages:

- *rld2hack* a DLL library that provides memory viewing, function execution, and function hooking.
- *rlinjector* a simple DLL injector application to inject rld2hack.dll into Diablo 2's Game.exe.
- *rlwindows* a hacky library wrapper around Win32 API calls.

Note this project supports Windows only. Currently supports Diablo 2 LOD patch 1.14d.

## Building

Run the following from the project root:

```batch
cargo build
```

## Running

After building run the following from the project root:

```batch
./target/debug/rlinjector.exe ./target/debug/rld2hack.dll -p Game.exe
```

## License

Licensed under either of

* Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
* MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.
