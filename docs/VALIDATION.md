# What is and is not validated

> This statement is surfaced in the product UI and in every release. It may
> only be changed with evidence, never softened for marketing.

| Capability | Status | Basis |
|---|---|---|
| External ballistics — point mass (G1/G7/custom drag) | **Not yet validated** | Phase 0 target: published reference trajectories |
| External ballistics — modified point mass | Not yet validated | Phase 1 |
| External ballistics — 6-DOF | Not yet validated | Phase 1 |
| Terminal ballistics (penetration, fragmentation, spall) | **Physically principled, NOT empirically validated** | Open validated terminal test data is largely unavailable (report §6.3). Results derive from offline MPM/FDEM continuum models. |
| Rigid-body scene dynamics | Game-grade approximation | Engine documentation states it approximates real-world behaviour |
| GPU particles / visual effects | Display only | Never authoritative |
