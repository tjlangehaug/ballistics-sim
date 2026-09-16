# 0005 — macOS release builds happen only on macOS CI runners
Status: Accepted
Date: 2026-09-15

## Context
Apple SDK licence restricts use to Apple hardware; code signing and notarisation are impossible off a Mac. (Report §8.3)

## Options
1. **Native macOS runners for macOS builds and releases** — legal, signable.
2. **osxcross / zig Docker images** — iteration only; still cannot sign or notarise.

## Decision
Local cross-builds (`just cross`) cover Linux/Windows for iteration. All macOS release artefacts come from a macOS CI runner.

## Consequences
Private-repo macOS minutes cost ×10 — keep the repo public, label-gate the 3-OS matrix. Notarisation needs a paid Apple Developer account (deferred to Phase 5).

## Revisit if
Apple changes its SDK licence or signing tooling.
