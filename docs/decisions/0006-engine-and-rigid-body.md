# 0006 — Engine and rigid-body library
Status: Proposed — HUMAN DECISION REQUIRED before Phase 2
Date: 2026-09-15

## Context
Need scene graph, PBR, assets, UI, input, and a rigid-body engine for everything the projectile hits (never the projectile itself). (Report §3, §4, §8.4)

## Options
Engine:
1. **Bevy 0.18** (report's recommendation) — ECS makes the solver 'just a system'; pre-1.0 API churn every release.
2. **Godot 4.7 + GDExtension** — stable editor, Jolt by default; engine-boundary friction.

Rigid body:
1. **Avian** — pure Rust, ECS-native, no FFI, follows Bevy's cadence.
2. **Jolt via jolt-rust** — mature, deterministic, AAA-proven; C++ FFI, Windows-only debug tooling, broad-phase layer trap.
3. **Rapier (f64)** — pure Rust with an f64 variant.

## Decision
<pending>

## Consequences
Whichever is chosen: solver stays engine-agnostic in `crates/ballistics`; pin the engine version and upgrade deliberately.

## Revisit if
Bevy API churn costs more than one week per upgrade; rigid-body step time misses budget at target scale.
