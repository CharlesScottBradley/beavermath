# Claude Code in-chat (IP3)

**Status:** **oneshot / hook scaffold; interactive in-chat blocked without display API**  
**Present surface:** `beavermath present`  
**Oneshot proof:** `../oneshot/filter.sh` and `beavermath filter` binary tests  

## Paths

| Class | Status |
|-------|--------|
| P1 native display hook | Not documented as display-override; **blocked** until proven |
| P2 transform of display-only field | Unknown; treat as blocked |
| P3 TTY wrapper | Optional future; must not break headless |
| S hook-stop → sidecar | Shipped scaffold; **not** product bar |
| Oneshot print | **Live** via filter |

## Headless

Hooks and oneshot must keep machine stdout escape-free (enforced by `beavermath present` / `filter` / `hook`).
