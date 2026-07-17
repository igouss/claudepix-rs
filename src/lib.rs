#![cfg_attr(not(test), no_std)]
#![forbid(unsafe_code)]
//! # claudepix
//!
//! A tiny, `no_std`, allocation-free way to put an animated 20×20 pixel creature on an
//! embedded screen — and a pure animation clock so the creature moves without a timer,
//! a mutable cursor, or a frame counter to drift.
//!
//! Pixels go into any [`DrawTarget`](embedded_graphics::prelude::DrawTarget), so the
//! same code drives an on-target panel (e.g. the ST7789 on an M5StickC Plus) and a host
//! framebuffer. Nothing here names a concrete panel or owns a clock.
//!
//! - [`sprite`] — the [ClaudePix](https://claudepix.vercel.app/) creature library: 4-bit
//!   palette indices packed two to a byte (200 bytes a frame, ~43 KB for all 13 presets),
//!   with [`Sprite::frame_at`] — the whole animation as a pure function of elapsed
//!   milliseconds. See [`ATTRIBUTION.md`] for the art's provenance and licence.
//! - [`testing`] — a pixel-counting host `DrawTarget` for tests (feature `testing`); never
//!   compiled into firmware.
//!
//! [`ATTRIBUTION.md`]: https://github.com/igouss/claudepix-rs/blob/main/ATTRIBUTION.md
//!
//! ## The shape
//!
//! ```
//! use claudepix::sprite::{self, IDLE_BREATHE};
//! # use embedded_graphics::pixelcolor::Rgb565;
//! # use embedded_graphics::prelude::*;
//! # fn demo<D: DrawTarget<Color = Rgb565>>(screen: &mut D, elapsed_ms: u64) -> Result<(), D::Error> {
//! // Ask "what should be on the glass now?" — a pure function of elapsed time.
//! let frame = IDLE_BREATHE.frame_at(elapsed_ms);
//! // Paint it at 3× scale, erasing the previous frame in one contiguous fill.
//! sprite::draw_onto(screen, &IDLE_BREATHE, frame, Point::new(0, 0), 3, Rgb565::BLACK)?;
//! # Ok(())
//! # }
//! ```
//!
//! ## What a host render can and cannot prove
//!
//! Against the [`testing`] framebuffer you **can** prove the decode, the layout, and that
//! each frame erases the one before it. You **cannot** prove anything below
//! [`DrawTarget`](embedded_graphics::prelude::DrawTarget): the panel's colour order, its
//! CGRAM offset, its inversion, or whether the backlight is even powered — only the real
//! panel can catch a channel swap.

#[cfg(any(test, feature = "testing"))]
extern crate alloc;

pub mod sprite;
#[cfg(any(test, feature = "testing"))]
pub mod testing;
