# ECOGRAMA — Spectrogram editor and audio restoration: select on the spectrogram, repair, export.

**English** · [Română](README.ro.md) · [Français](README.fr.md) · [Italiano](README.it.md) · [Español](README.es.md) · [Português](README.pt.md) · [Deutsch](README.de.md) · [Magyar](README.hu.md)

Version: 0.1.0-alpha.1 · Last updated: 2026-11-14

🌐 https://ecograma.chiuta.ro

## Status

Early skeleton (pre-alpha): the workspace builds and the tests pass in CI. Every planned feature is an open task in `docs/issues/`.

## Planned scope

- STFT with a configurable window and hop, colour maps (viridis, magma, grayscale) and WAV reading/writing (16/24-bit PCM, 32-bit float).
- A spectrogram view in egui with zoom and pan.
- Rectangle and lasso selections that produce a time–frequency mask, attenuated with overlap-add resynthesis.
- Spectral repair by interpolating surrounding frames, and de-hum that detects 50/60 Hz and harmonics.
- A CLAP plug-in wrapper (VST3 later) and a decision on how to relate to `cathar`: depend, interoperate or contribute upstream.

None of this is implemented yet: each item is an open task in `docs/issues/`.

## Build and test

```bash
cargo test --workspace
cargo run -p ecograma-cli -- selftest 64 8
```

## Contribute

Read `CONTRIBUTING.md`, pick a task labelled `good first issue` from `docs/issues/` and open a pull request. No contribution is too small: a table row, a test or a translation counts.

## Independent implementation

Written from public specifications and textbooks; no third-party source code is copied (see `CLEANROOM.md`).

## License

MIT — Copyright (c) 2026 Alexandru-Ionuț Chiuță (Alexio).

## Support

Trade-free: no tracking, no ads. If it is useful, you can support the author: https://buymeacoffee.com/echo.of.the.strings
