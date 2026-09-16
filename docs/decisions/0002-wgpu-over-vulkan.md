# 0002 — wgpu as the graphics/compute abstraction
Status: Accepted
Date: 2026-09-15

## Context
Three native APIs (D3D12, Metal, Vulkan). Portable compute required; CUDA excluded from the shipped product. (Report §2)

## Options
1. **wgpu** — MIT/Apache-2.0, native Metal backend (no MoltenVK), WGSL/SPIR-V/GLSL via Naga, compute is core API.
2. **Vulkan + MoltenVK** — full extension access; translation layer on macOS, extra debugging layer.

## Decision
wgpu.

## Consequences
WebGPU feature ceiling; weaker GPU debugging tooling than Nsight; API churn tracked by Bevy upgrades.

## Revisit if
A specific Vulkan-only extension becomes a hard requirement.
