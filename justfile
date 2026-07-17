# claudepix — recipes

# Run the whole check suite: format, sprite sync, lint, test.
ci: fmt-check sprites-check lint test

# Regenerate src/sprite/generated.rs from gen/frames.json (babashka).
sprites:
    bb gen/generate.clj

# Fail if generated.rs has drifted from gen/frames.json. Part of `just ci`.
sprites-check:
    bb gen/generate.clj --check

# Render every sprite to target/screens/sprites.png — six frames sampled across each
# loop. The sprite unit tests cannot see: a transposed decode still paints a
# creature-shaped blob. This is how a human checks the creature is a creature.
sprite-screens:
    cargo run --quiet --example sprites

# All tests, including the `testing` feature (host framebuffer).
test:
    cargo test --all-features

# Clippy with warnings denied.
lint:
    cargo clippy --all-features --all-targets -- -D warnings

fmt:
    cargo fmt --all

fmt-check:
    cargo fmt --all -- --check
