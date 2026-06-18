# Gfx2 - Nintendo Wii U™ Shader Binary Format

Unlike more modern graphics APIs the Nintendo Wii U does not have a online shader compiler, so the shaders have to be pre-compiled and embedded into the program or stored in the file system. The most common format for such shader binaries is Gfx2 (`.gsh` / `.gtx`). This crate enables easy parsing of the binary format into "Rust-native" structures (Vec & String instead of raw pointers) and storing shaders in alternative data formats via serde.

## Usage

```
cargo add gfx2
```

### main.rs

```rust,no_run
use gfx2::Gfx2;
use std::fs;

fn main() {
    let bytes = fs::read("shader.gsh").unwrap();
    let gfx2 = Gfx2::parse(&bytes).unwrap();
}
```

## Sources

- [Gfx2 - Nintendo File Formats](https://nintendo-formats.com/libs/gfd/gfx2.html)
- [decaf-emu / libgfd](https://github.com/decaf-emu/decaf-emu)
- [wut / libwhb](https://github.com/devkitPro/wut)
