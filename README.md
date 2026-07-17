# claudepix

**An animated pixel creature for embedded screens — `no_std`, zero-alloc, and a pure
animation clock so it moves without a timer.**

[![license](https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-blue.svg)](#licence)

A tiny Rust library that puts a 20×20 [ClaudePix](https://claudepix.vercel.app/) creature —
one that **breathes**, **blinks**, **dances**, and **codes** — onto any
[`embedded-graphics`](https://crates.io/crates/embedded-graphics) `DrawTarget`. The same
code drives an on-target panel (it was written for the ST7789 on an M5StickC Plus) and a
host framebuffer, so the whole thing is testable without a device attached.

> **On the artwork:** the creature *frames* are the third-party ClaudePix library and carry
> **no redistribution licence** — only the code around them is MIT/Apache. Read
> [ATTRIBUTION.md](ATTRIBUTION.md) before you reuse the art. If you are the ClaudePix
> author, open an issue and I'll act on it.

## Why it's nice

- **`no_std` + `forbid(unsafe_code)`.** Runs on a 520 KB, no-PSRAM ESP32 with nothing but
  `.rodata`. Frames are 4-bit palette indices packed two to a byte — **200 bytes a frame**,
  ~43 KB for all 13 presets — with no heap, no decompression, and nothing to initialise at
  boot. A firmware that names one sprite links only that sprite; each is its own `static`.
- **The animation is a pure function of time.** `sprite.frame_at(elapsed_ms)` is the *whole*
  of the animation logic — no timer, no mutable cursor, no frame counter to drift. It
  survives `u64::MAX` milliseconds without a panic. Who owns the clock is the caller's
  problem, not the library's.
- **A render path measured against real hardware.** `draw_onto` erases the previous frame in
  a **single contiguous fill**, not one rectangle per cell. The naïve per-cell version set a
  fresh panel address window 400 times a frame and cost **85 ms** against a 50 ms tick — the
  creature silently dropped frames and nothing on the glass said so. See the doc comment on
  [`draw_onto`](src/sprite/render.rs) for the story.
- **Tests that can catch a mirror.** The creature is left-right symmetric almost everywhere,
  so a transposed or mirrored decode *still looks like a creature*. The unit tests pin the
  decode against the one asymmetric row transcribed from upstream, and the vendored frames
  are frozen against browser-verified digests — a false-green a picture could never catch.

## The library

13 presets, in four categories:

| Category | Presets |
|---|---|
| **Idle** | `idle_breathe`, `idle_blink`, `idle_look_around` |
| **Expression** | `expression_wink`, `expression_surprise`, `expression_sleep` |
| **Dance** | `dance_bounce`, `dance_sway`, `dance_bounce_dj`, `dance_sway_dj`, `dance_djmix` |
| **Work** | `work_coding`, `work_think` |

Each is a `pub static` you can name directly (`sprite::IDLE_BREATHE`), or iterate over
`sprite::ALL`.

## Usage

```toml
[dependencies]
claudepix = { git = "https://github.com/igouss/claudepix-rs" }
embedded-graphics = "0.8"
```

Ask what should be on the glass *now*, and paint it:

```rust
use claudepix::sprite::{self, IDLE_BREATHE};
use embedded_graphics::pixelcolor::Rgb565;
use embedded_graphics::prelude::*;

fn tick<D: DrawTarget<Color = Rgb565>>(screen: &mut D, elapsed_ms: u64) -> Result<(), D::Error> {
    let frame = IDLE_BREATHE.frame_at(elapsed_ms);
    // 3× scale, overwriting its own box so successive frames don't smear or flash.
    sprite::draw_onto(screen, &IDLE_BREATHE, frame, Point::new(0, 0), 3, Rgb565::BLACK)
}
```

Two entry points:

- **`draw_onto`** — animates in place, painting a `background` where the sprite is
  transparent so each frame erases the one before it. This is the one you want in a loop.
- **`draw`** — composites over an existing background *once*, skipping transparent cells.
  Use it to lay a creature over a static picture.

Only redraw when the frame actually changes: `frame_index_at` returns the index, so a render
loop can compare it against what it last painted and skip the write when they match.

## See the creatures

The unit tests prove the decode arithmetic, but they can't *see*. This renders every sprite
to a PNG contact sheet — six frames sampled across each loop — so a human can confirm a
creature looks like a creature:

```sh
cargo run --example sprites     # → target/screens/sprites.png
```

## Develop

```sh
just ci              # fmt-check · sprites-check · clippy -D warnings · test
just sprites         # regenerate src/sprite/generated.rs from gen/frames.json (babashka)
just sprite-screens  # render the contact sheet
```

The frames are generated, not hand-edited: `gen/generate.clj` turns `gen/frames.json` into
`src/sprite/generated.rs` and refuses to do so unless the JSON still hashes to the
browser-verified digests. `just sprites-check` guards against drift in CI.

## Licence

The **code** is licensed under either of [Apache-2.0](LICENSE-APACHE) or
[MIT](LICENSE-MIT), at your option. The **creature artwork** is third-party and is **not**
covered by that licence — see [ATTRIBUTION.md](ATTRIBUTION.md).

Unless you state otherwise, any contribution you intentionally submit for inclusion in the
work, as defined in the Apache-2.0 licence, shall be dual licensed as above, without any
additional terms or conditions.
