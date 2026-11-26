# VIctory-RDNA

A performance-focused, AMD RDNA-tuned fork of [VIctory](https://github.com/Marvisak/VIctory) — a Direct3D 12 compatibility layer that runs Xbox Series titles natively on Windows from user-supplied files.

Built directly on [avatarmanz/VIctory-7900XTX](https://github.com/avatarmanz/VIctory-7900XTX) with selective performance ports from [fxceless/victory-fork](https://github.com/fxceless/victory-fork) and a one-command launcher with tuned RDNA3 defaults out of the box.

> **This repository contains no game files, code, or assets.** You must supply your own dump of the title. It is not affiliated with or endorsed by the game's developer or publisher.

---

## Quick Start (3 Steps)

### Requirements
- **OS:** Windows 10 / 11 x64
- **GPU:** Direct3D 12 GPU with Shader Model 6.6 & Mesh Shaders (tuned for AMD RDNA3 — RX 7900 XT / XTX, RX 7000 series; works on other SM 6.6 GPUs too)
- **CPU / RAM:** AVX2-capable CPU, 64 GB RAM recommended (32 GB minimum)
- **Toolchain:** [Rust](https://rustup.rs/) + MSVC Build Tools

### 1. Clone inside your game folder (or anywhere)
```powershell
git clone https://github.com/XaviFortes/VIctory-RDNA.git
cd VIctory-RDNA
```

### 2. Launch (auto-builds on first run!)
If `VIctory-RDNA` is inside your game folder (next to `game_scarlett_master.exe`), just double-click **`launch.bat`** or run:
```powershell
.\scripts\play-rdna.ps1
```

If your game is in a different folder, pass `-Exe`:
```powershell
.\scripts\play-rdna.ps1 -Exe "D:\Games\GTA6\game_scarlett_master.exe"
```

That's it — the launcher automatically builds the release binary if missing, clears stale `VICTORY_*` variables, and sets all tuned RDNA3 performance flags.

---

## Launcher Options (`scripts/play-rdna.ps1`)

| Flag | Default | What it does |
| :--- | :---: | :--- |
| `-Exe <path>` | Auto-detected | Path to `game_scarlett_master.exe` (auto-checks `.` and `..`) |
| `-Console <mode>` | `series-x` | Console profile: `series-x` or `series-s` |
| `-Safe` | Off | Disables newest experimental renderer ports (`VICTORY_THIN`, `VICTORY_PORT_CONSTANTS`, `VICTORY_PORT_DESCRIPTORS`) for troubleshooting |
| `-RayTracing` | Off | Enables ray-traced indirect lighting (`VICTORY_BVH_BUILDER=1`, ~30% slower) |
| `-Fps <N>` | `0` (30 fps) | Unlocks frame rate target via `VICTORY_FRAME_INTERVAL_MS` (e.g. `-Fps 60`) |
| `-Log <path>` | None | Saves full runtime output to a log file while displaying it |
| `-Build <profile>` | `release` | `release` (fastest) or `diag` (with `diagnostics` feature) |
| `-Rebuild` | Off | Forces a fresh `cargo build` before launching |
| `-More @{...}` | `{}` | Pass or override any `VICTORY_*` environment variable (e.g. `-More @{VICTORY_PANEL='1'}`) |

### Examples
```powershell
# Standard high-FPS RDNA3 run with log capture
.\scripts\play-rdna.ps1 -Log .\victory-rdna.log

# Unlock 60 FPS cap
.\scripts\play-rdna.ps1 -Fps 60

# Troubleshooting / safe mode
.\scripts\play-rdna.ps1 -Safe

# From Command Prompt (cmd.exe)
launch.bat -Log run.log
```

---

## What VIctory-RDNA Adds & Tunes

On top of the full `VIctory-7900XTX` RDNA3 engine (`VICTORY_THIN` 8-thread list translation, `VICTORY_PORT_CONSTANTS` zero-copy constants heap, `VICTORY_PORT_DESCRIPTORS` patched descriptor tracking, `VICTORY_CARD_STEPS` GPU-side fills, AVX2 `safecopy`, copy-queue texture uploads, `VICTORY_AMBIENT_SETTLE` time-based indoor lighting, and `VICTORY_FRAME_INTERVAL_MS` FPS unlock):

- **CPU-side `strides_shadow` & `view_addr_shadow` cache** in `descriptor/host.rs`: skips redundant uncached PCIe `write_volatile` writes on every mirrored descriptor slot.
- **GPU-offloaded 1 MiB+ shader-write compares (`VICTORY_CARD_BUFFERS_MIN_MB=1`)** enabled by default in the launcher (moves ~76 MB/frame of CPU compares & snapshots onto the GPU).
- **Silenced `port_descriptors` WARN spam**: caps missed-store fallback warnings to the first 10 occurrences instead of logging ~200 lines/frame on the hot path.
- **Zero-friction launchers (`scripts/play-rdna.ps1` + `launch.bat`)**: auto-detects the executable, auto-builds on first run, and applies the full RDNA3 preset in one command.

### Measured Performance (RX 7900 XT, Series X profile, RT Off)
- **Steady gameplay:** ~**13.6–18.5 FPS** (`54–73 ms/frame`)
- **GPU settle waits (`on the GPU`):** **0.0 ms/frame**
- **Heap buffer pass:** **6.8–9.9 ms/frame**
- **Texture refreshes (main thread):** **0.7–1.6 ms/frame**

---

## Credits & Upstream Projects

This project is a fork that builds directly on the incredible work of the VIctory community:

- **[Marvisak/VIctory](https://github.com/Marvisak/VIctory)** — original loader, HLE runtime, Direct3D 12 backend, and shader recompiler.
- **[avatarmanz/VIctory-7900XTX](https://github.com/avatarmanz/VIctory-7900XTX)** — the direct upstream base for this fork (RDNA3 renderer ports, `VICTORY_THIN`, `VICTORY_CARD_STEPS`, AVX2 `safecopy`, time-based ambient settle, FPS unlock).
- **[fxceless/victory-fork](https://github.com/fxceless/victory-fork)** — upstream optimizations and community PRs.
- **[DexNom/VIctory](https://github.com/DexNom/VIctory)**, **[Belobug/VIctory-Belobug-AMD](https://github.com/Belobug/VIctory-Belobug-AMD)**, **[FQY55kai/ZJOU-VICTORY](https://github.com/FQY55kai/ZJOU-VICTORY)**, **[myokex/victory-amd](https://github.com/myokex/victory-amd)**, **Geckomacabre**, and **cxy0072019-sketch**.

For full upstream technical notes and environment switch references, see [docs/README-fxceless.md](docs/README-fxceless.md) and [docs/findings.md](docs/findings.md).

## License

GPL-3.0-or-later, as VIctory. See [LICENSE](LICENSE).
