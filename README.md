# image-resize-wasm

![a full resolution image on the left with cropped and resized versions overlaid on top each other to the right.](./assets/banner.jpg)

Resize images by width, height, or both using the [Rust Image crate](https://crates.io/crates/image) and [WASM](https://webassembly.org/).

This package uses [wasm-pack](https://rustwasm.github.io/wasm-pack/) as a build tool. The origin of this project was to use an Edge Function to resize images on the fly.

## Usage

### Vercel Edge Function

<https://vercel.com/docs/functions/edge-functions/wasm>

> **Warning**
> This example is currently **nonfunctional**!

The following example works with `vercel dev`, but failes to deploy with the following error:

```bash
Error: The Edge Function "api/[resize]" is referencing unsupported modules:
        - image-resize-wasm: vc-blob-asset:image_resize_wasm_bg.wasm
```

I was eventually able to "fix" the build by manually changing the build outputs, however I couldn't get the example working.

```bash
RuntimeError: unreachable
    at (wasm://wasm/001256da:1:169628)
    at (wasm://wasm/001256da:1:209588)
    at (wasm://wasm/001256da:1:186105)
    at (wasm://wasm/001256da:1:193864)
    at (node_modules/.pnpm/image-resize-wasm@1.1.8/node_modules/image-resize-wasm/image_resize_wasm_bg.js:48:0)
    at (api/[resize].js:44:62)
```

```ts
// api/index.ts
// http://localhost:3000/api?url=https://example.com/example.png&w=1024

// @ts-ignore
import wasm from "image-resize-wasm/image_resize_wasm_bg.wasm?module";
import init, {
  scale_by_width,
  scale_by_height,
  resize,
} from "image-resize-wasm";

export const config = {
  runtime: "edge",
};

export default async function handler(request: Request) {
  await init(wasm);
  const { searchParams } = new URL(request.url);
  const url = searchParams.get("url"),
    wString = searchParams.get("w"),
    hString = searchParams.get("h");

  if (!url || (!wString && !hString)) throw new Error();
  const w = Number(wString),
    h = Number(hString);

  const response = await fetch(url);
  const imageUint8Array = new Uint8Array(await response.arrayBuffer());

  let scaledImage;
  if (w && h) {
    scaledImage = resize(imageUint8Array, w, h);
  } else if (w) {
    scaledImage = scale_by_width(imageUint8Array, w);
  } else if (h) {
    scaledImage = scale_by_height(imageUint8Array, h);
  }

  return new Response(scaledImage, {
    status: 200,
    headers: {
      "Content-Type": "image/jpeg",
    },
  });
}
```

## Contributing

### Build

```bash
wasm-pack build --target web
```

### Publish

```bash
cd pkg
pnpm publish
```

## License

Copyright © 2023 Alexander Liu

MIT License
