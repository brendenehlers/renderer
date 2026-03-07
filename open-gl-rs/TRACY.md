# Tracy Profiler Integration

## Requirements

- [Tracy profiler](https://github.com/wolfpld/tracy) — download a release matching the client version below
- The `tracing-tracy` crate pins a specific Tracy protocol version; check [its changelog](https://github.com/nagisa/rust_tracy_client/blob/main/tracing-tracy/CHANGELOG.md) for the required Tracy release

`tracing-tracy = "0.11"` targets **Tracy 0.11.x**.

## Building with Tracy enabled

```bash
cargo build --features tracy
cargo run --features tracy
```

The binary connects to a running Tracy server on startup. If no server is listening the app runs normally with no overhead.

## Connecting Tracy

1. Launch the Tracy GUI (`tracy` or `Tracy.exe`)
2. Click **Connect** (default address `127.0.0.1:8086`)
3. Start the renderer with `--features tracy` — Tracy will begin capturing immediately

## What you'll see

All `tracing` spans and events are forwarded:

| Span | Level | Appears in Tracy as |
|---|---|---|
| `startup` | INFO | one-shot span at boot |
| `new` (Shader) | INFO | shader compile time |
| `load` (Model) | INFO | full model load incl. children |
| `process_node` / `process_mesh` / `create_texture` | DEBUG | asset pipeline breakdown |
| `frame` | TRACE | per-frame root span |
| `scene_draw` / `imgui` | TRACE | per-frame sub-spans |

TRACE-level spans (frame, scene_draw, imgui) are gated by `RUST_LOG`. To see them in Tracy:

```bash
RUST_LOG=trace cargo run --features tracy
```

## Tips

- Use **Statistics → Find Zone** to locate `scene_draw` and measure draw call cost
- The `frame` span carries a `frame` field — use **Plot** to graph it over time
- Tracy's **Frame Image** capture works independently of this integration
