# 0001 — Rust as the primary language
Status: Accepted
Date: 2026-09-15

## Context
Solo developer; C++ or Rust acceptable; one machine should build all targets; Python bindings required. (Report §1, §8.1, §9.2)

## Options
1. **Rust** — cargo cross-compilation is the easiest available; PyO3/maturin; wgpu/Bevy/Rapier/Avian native.
2. **C++** — Jolt, PhysX and MPM research codes are native; but CMake + vcpkg/Conan with three build environments.

## Decision
Rust. C++ only via FFI where a C++ solver is the only mature option (and then with an ADR).

## Consequences
Every `-sys` crate is a cross-compilation liability; prefer Rust-native dependencies.

## Revisit if
A required capability exists only as a C++ library and the FFI cost exceeds a rewrite.
