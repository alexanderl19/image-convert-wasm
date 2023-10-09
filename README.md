# image-resize-wasm

![a full resolution image on the left with cropped and resized versions overlaid on top each other to the right.](./assets/banner.jpg)

Resize images by width, height, or both using the [Rust Image crate](https://crates.io/crates/image) and [WASM](https://webassembly.org/).

This package uses [wasm-pack](https://rustwasm.github.io/wasm-pack/) as a build tool. The origin of this project was to use an Edge Function to resize images on the fly.

## Usage

## Contributing

### Build

```bash
wasm-pack build --target web
```

## License

Copyright © 2023 Alexander Liu

MIT License
