# OMP adapter (Phase 8)

Uses `beavermath_adapter::OmpAdapter` for automatic post-turn preview:

- Observes completed assistant content only (source while streaming).
- Emits `AdapterEventV1` with semantic preview and `MathArtifactV1` list.
- Never injects BeaverMath into model prompts or tool lists.
- Canonical session content remains byte-identical.

True inline span replacement requires an OMP core `registerAssistantContentRenderer`-class hook (Phase 9). Until upstream accepts that hook, extension-only supplemental preview is the supported path.
