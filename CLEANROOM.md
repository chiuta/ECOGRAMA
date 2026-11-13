# ECOGRAMA provenance and independent-implementation procedure

ECOGRAMA is **independently implemented from public specifications, standards and textbooks. No third-party source code was copied.** This file says what was done; it is an engineering discipline that keeps provenance auditable, not legal advice, and not a strict clean-room guarantee (no separate specification team exists).

## Rules
1. **Inputs allowed:** public standards and specifications, textbooks and peer-reviewed papers, public documentation and *user-visible* behaviour of other tools.
2. **Inputs forbidden:** decompiled or leaked code, EULA-restricted SDKs, proprietary format reverse engineering, other tools' generated tables, names, UI strings or sample content.
3. **Provenance per module:** name the public source in the module's doc comment and add a line to the table below.
4. **Dependencies:** permissive licences only (MIT, Apache-2.0, BSD, ISC, Zlib, CC0); `cargo deny check` enforces the list in `deny.toml`.
5. **Own names and schemas** for anything without a public standard.
6. **Related projects** are listed in `docs/LANDSCAPE.md`; reading their *documentation* for scope is fine, copying their code is not. Check a project's licence before even reading its source if you might reimplement it.

## AI assistance disclosure
The initial skeleton and its starting module were written with an AI model (Claude, Anthropic). Its training data is not under the author's control; the code was written from the public sources named below. A reviewer who needs stronger assurance should treat this as an independent implementation with a documented residual risk.

## Provenance log
| Module | Basis (public) | Notes |
|---|---|---|
| hann, dft_magnitude | textbook DSP (Hann window, DFT definition) |  |
