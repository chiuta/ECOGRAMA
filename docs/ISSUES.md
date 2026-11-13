# ECOGRAMA — first ten tasks

Create them on GitHub with `scripts/create-issues.sh OWNER/REPO`. Comment on an issue to claim it.

1. **STFT with configurable window and hop; test against `dft_magnitude`** — `good first issue`, `area:dsp`
   Use `rustfft` behind a feature; the naive DFT stays as the oracle. Max difference per bin < 1e-3.

2. **Colour maps as data: viridis, magma, grayscale** — `good first issue`, `data`, `area:gui`
   256-entry tables in `data/*.csv`, a `ColorMap` type, and a test that each table is monotone in luminance.

3. **WAV reading and writing (16/24-bit PCM and 32-bit float)** — `good first issue`, `area:io`
   Implement or choose a permissively-licensed crate; round-trip tests; reject malformed headers without panicking.

4. **Spectrogram view in egui: render, zoom and pan** — `help wanted`, `area:gui`
   Log-frequency option; handle 10-minute files by tiling. Screenshot in the PR.

5. **Selections: rectangle and lasso produce a time–frequency mask** — `help wanted`, `area:gui`
   Mask is a sparse structure with soft edges; unit-test mask union/difference.

6. **Attenuate the selection with overlap-add resynthesis** — `help wanted`, `area:dsp`
   Round-trip test: no selection = bit-exact within 1e-6; selection removes a synthetic tone by ≥ 40 dB.

7. **Spectral repair: fill a selection by interpolating surrounding frames** — `help wanted`, `area:dsp`
   Clicks and short dropouts first. Test with a sine + click; report SNR before/after.

8. **De-hum: detect 50/60 Hz and harmonics, apply narrow notches** — `good first issue`, `area:dsp`
   Synthetic hum + noise test; keep speech band untouched (measure it).

9. **CLAP plug-in wrapper (VST3 later)** — `help wanted`, `area:plugin`
   Check licences of the plug-in framework and the VST3 bindings first; write the finding in `docs/`.

10. **Compare with `cathar` and decide: depend, interoperate or contribute upstream** — `help wanted`, `docs`
   Benchmark de-click and spectral repair on the same synthetic signals; write `docs/ADR-001.md`.
