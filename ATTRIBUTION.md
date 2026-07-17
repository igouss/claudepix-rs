# Attribution & licence status

This repository is **two things** with **two different licences**, and this file draws
the line between them so nobody is misled about what they may reuse.

## The code — MIT OR Apache-2.0

Everything I wrote — the on-device sprite format, the packing, the pure animation clock,
the two-path renderer, the test framebuffer, the babashka generator, the example, and the
tests — is Copyright © 2026 Iouri Goussev and licensed under
[MIT](LICENSE-MIT) OR [Apache-2.0](LICENSE-APACHE), at your option. Reuse it freely.

## The artwork — third-party, **no licence granted**

The creature frames themselves are **not mine**. They are the
[**ClaudePix**](https://claudepix.vercel.app/) *Pixel Animation Library* (v0.1, retrieved
2026-07-09): 13 presets on a 20×20 grid, 216 frames. They are vendored here as
`gen/frames.json` and, derived from it, `src/sprite/generated.rs`.

**The upstream site states no licence.** There is no `LICENSE` route, no repository link,
no copyright line, and no attribution anywhere on the site. That means:

> The artwork is included here **without a redistribution grant from its author.**

It is reproduced for interoperability and credit, with a link back to the source, and with
no claim of ownership. If you are the author of ClaudePix and want this removed, or want to
attach a licence, please open an issue — I will act on it immediately. If you intend to
reuse the *art* (as opposed to the code), the licence you need is the upstream author's,
not mine, and I cannot grant it.

The MIT/Apache licence above covers the code around the art; it does **not** extend to the
216 vendored frames.

## How the frames were obtained (and why they are trustworthy)

A ClaudePix frame is not stored anywhere on the site — it is *computed* at page load by the
site's own JavaScript. The frames were resolved by **executing that JavaScript once**, in a
`node:vm` with a DOM stub, one fresh context per preset. **No JavaScript lives in this
repository**; `gen/frames.json` holds only the resulting explicit 20×20 grids of palette
indices and each frame's hold in milliseconds, and `gen/generate.clj` (babashka) turns that
JSON into `src/sprite/generated.rs`.

Each preset's frames were hashed by **two independent paths** — the headless extraction, and
the same presets re-derived inside a real browser on the live site. All 13 agreed. Those
digests are frozen in `gen/generate.clj` (`verified-digests`), which refuses to emit
`generated.rs` unless the vendored JSON still hashes to them, so an edited or truncated copy
fails the build rather than silently shipping different art.
