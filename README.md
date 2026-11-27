<div align="center">

# VIctory-RDNA

**High-FPS Xbox Series X|S Direct3D 12 compatibility layer, engineered for AMD RDNA3.**

[![License: GPL v3](https://img.shields.io/badge/License-GPLv3-blue.svg)](LICENSE)
[![Platform: Windows & Linux](https://img.shields.io/badge/Platform-Windows%20%7C%20Linux%20(Wine)-0078D6.svg)](#quick-start)
[![GPU: AMD RDNA3 / RDNA4](https://img.shields.io/badge/Tuned%20For-AMD%20RDNA3%20%2F%20RDNA4%20(D3D12)-ED1C24.svg)](#what-makes-victory-rdna-fast)
[![Series X 4K: 23.1 FPS Median | 30.8 Peak](https://img.shields.io/badge/GTA%20VI%204K%20Series%20X-23.1%20FPS%20Median%20%7C%2030.8%20Peak-2EA44F.svg)](#benchmarks)

*Standing on the shoulders of [VIctory](https://github.com/Marvisak/VIctory) and [VIctory-7900XTX](https://github.com/avatarmanz/VIctory-7900XTX) — pushed to nearly **2× baseline performance** with zero resolution hacks and a 1-click desktop launcher.*

[Why VIctory-RDNA](#-what-makes-victory-rdna-fast) · [Benchmarks](#-benchmarks) · [Quick Start](#-quick-start) · [PowerShell CLI](#-powershell-cli-windows) · [Picture & Status](#-visual-quality-honest-status--known-limitations) · [Contribute](#-contribute) · [Credits](#-credits)

</div>

---

## ⚡ Headline Performance (GTA VI · Series X 4K)

Tested on **Windows 11 · AMD Radeon RX 7900 XT (20 GB, ReBAR) · Direct3D 12** at **native Xbox Series X 4K output (`3840 × 2160`)** — no resolution downgrades, no scale hacks, and **clean exit (`code 0`)**:

| Metric | Early RDNA Baseline (`rdna4`) | Latest `VIctory-RDNA` (`rdna18` / `rdna21`) | Improvement |
| --- | ---: | ---: | ---: |
| **Gameplay FPS (Series X 4K)** | ~12.6–13.1 FPS | **20.8–23.1 FPS median** *(**30.8 FPS peak**)* | **~1.8× – 2.4× faster** |
| **Heavy-Scene FPS (4,000+ draws/frame)** | ~12–15 FPS | **20.3 FPS median** *(144 samples, 24.4 FPS peak)* | **Consistently playable** |
| **CPU Submission Prep (`preparing`)** | 50–65+ ms | **29.5–33.8 ms median** *(down to 27.5 ms)* | **~2× lower CPU overhead** |
| **Long-Session `heap_buffers` Creep** | 10.3 → 42.5 ms *(creeping)* | **7.2 ms flat** *(23m 50s run, zero creep)* | **-83% & permanently flat** |
| **Port 7 Descriptor Wait (`the lane`)** | 10.9 ms | **0.22 ms** | **-98% faster** |
| **CPU Shader-Write Compare Traffic** | 30.7 MB / 7.1 ms | **0.6–0.7 MB / 0.2 ms** | **-98% CPU scan traffic** |
| **GPU Settle Waits (`on the GPU`)** | Up to 9.1 ms | **0.0 ms** *(zero stalls)* | **Stalls eliminated** |
| **Zero-Copy In-Place Memory** | 0 MB | **842 MB** *(smart GDDR6/in-place split)* | **Enabled by default** |

> [!TIP]
> **Optional AMD AFMF 2:** Because `VIctory-RDNA` delivers **22–25+ FPS native** with flat frame pacing and zero settle stalls on Series X 4K, enabling **AMD Fluid Motion Frames 2 (AFMF 2)** in Adrenalin brings perceived fluidity into the **45–50 FPS** range — while keeping full Series X visual fidelity. *(All benchmark numbers in this README are strictly **native rendered FPS** without frame generation.)*

> [!NOTE]
> `VIctory-RDNA` is an active, experimental compatibility layer. You must supply your own legally obtained game files — no copyrighted assets are included or linked here. Compatibility and performance vary by title, hardware, and build.

---

## 🚀 What Makes VIctory-RDNA Fast

`VIctory-RDNA` stays up to date with upstream **Renderer Ports 1–7** (`VICTORY_PORT_GUARD`, `VICTORY_PORT_JOURNAL`, `VICTORY_PORT_BUFFERS`) and stacks a dedicated **RDNA3 + CPU frame-pacing pipeline** on top — all **pre-configured out of the box** in [`scripts/rdna-preset.json`](scripts/rdna-preset.json):

- **Hybrid In-Place Memory (`842 MB` Zero-Copy, `0.0 ms` Settles)**
  Maps ~3,450 small/medium title buffers and windows directly in place (`VICTORY_PORT_BUFFERS=1` + `VICTORY_INPLACE=2`) while automatically keeping large (`≥ 1 MiB`) GPU-hot buffers in **800 GB/s local GDDR6 VRAM** instead of 32 GB/s PCIe. Full GPU-side `card_steps` copies/fills and `place_copy` support keep joined submissions high (`21+`/frame) and GPU settles at `0.0 ms`.
- **Zero Long-Session Slowdown (`BufferSweep` Cached Spans)**
  Deduplicates buffer sweep slots and caches per-part spans, completely eliminating the `heap_buffers` slowdown that used to creep from `10 ms` to `42.5 ms` over long play sessions.
- **L1-Resident Descriptor Allocator & Prebuilt Null Template**
  Replaces per-slot `Vec<bool>` scans with a **16 KiB packed `u64` bitset (`DynamicBits`)** and batched per-list releases (1 lock per command list instead of 12,000+ locks/frame), paired with a pre-created CPU null-descriptor template heap and PCIe write-skipping descriptor stride cache.
- **Lock-Free Shadow Checks & AVX2 Sweeps**
  A lock-free `AtomicBool` fast path + merged `RwLock` eliminates ~17,400+ mutex lock/unlock cycles per frame in `Shadow::nothing_new`, backed by AVX2-vectorized `word_runs` and 256 KiB parallel work chunks.
- **Parallel Texture Readbacks & Single-Pass Journal Sync**
  Parallelizes `texback::land_many` across worker threads with thread-local scratch buffers and per-flight water-readback dedup (`read-backs and lists`: `11–23 ms → 3.2 ms`), and unifies Port 7 `ask_once()` into a single parallel pass (`-35%` exclusive journal locks).
- **RDNA4 (RX 9070 XT), FSR 4.1.2 & Prelude Textures**
  Includes upstream submission-prelude texture uploads (fixing frame-order texture flicker), **RX 9070 XT (RDNA4)** fixes & upgraded live HUD (`VICTORY_PANEL=1`), **PC FidelityFX FSR 4.1.2 (`-Fsr4K`)**, and Renderer Port RT groundwork (`VICTORY_PORT_RT=1`, `VICTORY_AMBIENT_RT`), while keeping `VICTORY_PORT_CONSTANTS_ORDER=0` in the fast preset to preserve full GPU throughput.
- **1-Click Desktop Game Library (Windows & Linux)**
  No manually pasting 20 environment variables. Double-click `launch.bat` (or `sh ./launch.sh`), pick your game, choose Series X or Series S, and click **Play**.

---

## 🎮 Quick Start

### Requirements
- **CPU:** x64 with AVX2 support (Windows 10/11 or desktop Linux).
- **GPU:** Direct3D 12 with **Shader Model 6.6** and **Mesh Shaders**. Tuned for **AMD RDNA3** (RX 7900 XT / XTX), and works on other SM 6.6+ D3D12 GPUs. *(Linux uses Wine + [VKD3D-Proton](https://github.com/HansKristian-Work/vkd3d-proton).)*
- **RAM:** **32 GB minimum**, **64 GB recommended** for GTA VI (watch Windows available commit size at startup).
- **Launcher:** Python 3.10+ with Tkinter ([Python installer](https://www.python.org/downloads/) with Tcl/Tk on Windows; `python3-tk` / `python3-tkinter` on Linux).
- **Building from source (Windows):** [Rust (`rustup`)](https://rustup.rs/) + Visual Studio Build Tools (**Desktop development with C++**). Cargo automatically picks up the pinned toolchain from [`rust-toolchain.toml`](rust-toolchain.toml).

### 3-Step Setup (Windows)

```powershell
git clone https://github.com/XaviFortes/VIctory-RDNA.git
cd VIctory-RDNA
```

1. **Open the launcher:** Double-click **`launch.bat`** (Windows) or run **`sh ./launch.sh`** (Linux).
2. **Select your title:** On first launch, it automatically scans for `game_scarlett_master.exe` in the project folder, its parent, and your current working directory — or click **Add game…** to pick any title executable.
3. **Click Play:** Choose **Xbox Series X** (recommended) and hit **Play**. On Windows, the launcher automatically builds `victory.exe` in Release mode if it isn't built yet and applies the full tuned RDNA3 preset.

> [!IMPORTANT]
> **Best visual quality & stability on Series X:** Keep the default **Series X** profile at native **4K output** (do **not** force `-Output 1440p` or render scales below `0.67` — GTA VI's internal upscaler assumes `≥ 2/3` scale, and forcing lower scales causes jumping textures or crashes).

<details>
<summary><strong>Linux Setup (Wine + VKD3D-Proton)</strong></summary>

1. Install Wine with x64 Direct3D 12 support (`VKD3D-Proton`) and ensure `wine` and `winepath` are on your `PATH`.
2. Build `victory.exe` on Windows with `cargo build --locked --release -p victory` (or use a trusted release build) and copy it to `target/release/victory.exe` (or point to it under **Advanced** in the launcher).
3. Launch the desktop UI:

```sh
WINEPREFIX="$HOME/.wine-victory" sh ./launch.sh
```

*Note: Linux execution via Wine is experimental and depends on your Wine prefix, VKD3D-Proton, and Vulkan driver.*

</details>

<details>
<summary><strong>Desktop Launcher Settings & Log Locations</strong></summary>

| Setting | Default | Behavior |
| --- | --- | --- |
| **Console profile** | Xbox Series X | Selects the console profile & memory budget presented to the title (`series-x` or `series-s`). |
| **FPS target** | `0` | Uses the title's own frame pacing. Set a positive value (e.g. `60`) to request a shorter frame interval. |
| **Safe mode** | Off | Disables aggressive experimental paths (`VICTORY_UNDER_WIDE=0`, `VICTORY_LAND_ECHO=0`). Useful for isolating crashes. |
| **Ray tracing** | Off | Enables ray-traced indirect lighting (higher GPU cost). |
| **Build profile** | Release | Switch to **Diagnostics** to build/run with runtime diagnostics (`target/diag/victory.exe`). |
| **Rebuild before play** | Off | Re-runs Cargo before launching on Windows. |
| **Environment overrides** | Empty | Under **Advanced**, enter one `VICTORY_NAME=value` per line (e.g. `VICTORY_PANEL=1`). |

Every session automatically saves a fresh timestamped log without overwriting older captures:
- **Windows launcher data & logs:** `%LOCALAPPDATA%\VIctory-RDNA`
- **Linux launcher data & logs:** `${XDG_DATA_HOME:-$HOME/.local/share}/victory-rdna`
- **Runtime saves, cache & `crashes.log`:** `%LOCALAPPDATA%\victory\data` and `%LOCALAPPDATA%\victory\cache` (configurable via [`victory.example.toml`](victory.example.toml)).

</details>

---

## 📊 Benchmarks

All measurements below are from **periodic swap-chain logs on a single machine** (Windows 11, **AMD Radeon RX 7900 XT 20 GB**, Zen 3 24-thread CPU, 64 GB RAM, ReBAR enabled, Release build).

### 1. Latest Series X 4K Runs (`rdna18` & `rdna21` · Default Preset)

- **`victory-rdna21.log` (27 November 2026 — Extended 23 min 50 s Run):** Windows D3D12 release run on the **RX 7900 XT** at **3840 × 2160 output** (Series X, 30 FPS target) with upstream prelude texture uploads, RX 9070 XT fixes, and FSR 4.1.2 merged. Ran **23 min 50 s** continuously across **260 gameplay samples** (144 of them heavy `4,000+` draw scenes) and exited cleanly (**`code 0`**) with **`0.0 ms`** GPU settle waits and flat **`7.2 ms`** `heap_buffers`.
- **`victory-rdna18.log` (26 November 2026 — 9 min 38 s Run):** Same RX 7900 XT Series X 4K setup, clean exit (**`code 0`**).

| Sample Metric | `rdna21` (`1,000+` draws, 23m 50s) | `rdna21` (`4,000+` draws, heavy) | `rdna18` (`1,000+` draws, 9m 38s) | `rdna18` (`4,000+` draws, heavy) |
| --- | ---: | ---: | ---: | ---: |
| **Samples** | **260** | **144** | 101 | 25 |
| **Median Draws / Frame** | 4,228 | 4,357 | 3,890 | 4,682 |
| **Median FPS** | **20.8 FPS** | **20.3 FPS** | **23.1 FPS** | 19.1 FPS |
| **Peak FPS** | **30.8 FPS** | **22.2 FPS** | 29.3 FPS | 24.4 FPS |
| **Median Frame Time** | 48.05 ms | **49.3 ms** | **43.4 ms** | 52.3 ms |
| **Median CPU Prep** | 33.8 ms | **34.45 ms** | **29.5 ms** | 35.4 ms |
| **Median `heap_buffers`** | **7.2 ms** *(zero creep)* | **7.3 ms** | 7.7 ms | 11.5 ms |
| **GPU Settle Waits** | **0.0 ms** | **0.0 ms** | **0.0 ms** | **0.0 ms** |

*`rdna21` broke **30.8 FPS peak** on native Series X 4K and improved heavy-scene (`4,000+` draws) median FPS from **19.1 → 20.3 FPS** over a **23 min 50 s** continuous session with zero memory/descriptor creep.*

### 2. Series X 4K vs. Series S 1080p

With `rdna18`, **Series X at 4K output (`22.1–23.1 FPS` median, `25.4 FPS` final-minute peak)** now **matches or beats Series S at 1080p** while delivering substantially better visual quality:

| Final-Minute Metric | Series S 1080p (`rdna17-ss`) | Series X 4K (`rdna17`) | Series X 4K (`rdna18` · Latest) |
| --- | ---: | ---: | ---: |
| **Presentation Output** | 1920 × 1080 | 3840 × 2160 | **3840 × 2160** |
| **Sampled Median FPS** | 21.8 FPS | 18.75 FPS | **22.1 FPS** *(23.1 overall)* |
| **Sampled FPS Range** | 15.8 – 24.9 FPS | 14.6 – 21.7 FPS | **14.7 – 25.4 FPS** |
| **Median Frame Time** | 45.8 ms | 53.4 ms | **45.25 ms** *(43.4 overall)* |
| **Median CPU Prep** | 34.2 ms | 35.25 ms | **31.1 ms** *(29.5 overall)* |
| **0.0 ms GPU Settle Waits** | 11 / 11 | 11 / 12 | **11 / 12** |
| **Exit Status** | Clean (`code 0`) | Clean (`code 0`) | **Clean (`code 0`)** |

<details>
<summary><strong>3. Engineering Deep Dive: How Hybrid In-Place Memory Unlocked +5 FPS (<code>rdna14 → rdna18</code>)</strong></summary>

| Final-Minute Metric (Series X 4K) | `rdna14` Baseline | `rdna15` Broad In-Place | `rdna16` GDDR6 Big-Buffer Guard | `rdna17` GPU Copy/Fill | `rdna18` In-Place `place_copy` Join |
| --- | ---: | ---: | ---: | ---: | ---: |
| **Median FPS** | 17.3 | 14.85 | 15.9 | 18.75 | **22.1** |
| **Median Frame Time** | 57.9 ms | 67.4 ms | 62.8 ms | 53.4 ms | **45.25 ms** |
| **CPU Submission Prep** | 39.4–51.5 ms | 32.5–40.3 ms | 33.8–43.9 ms | 30.4–39.6 ms | **27.5–31.5 ms** |
| **Copy Prep Traffic** | 47.5 MB/f | 36.15 MB/f | 36.1 MB/f | 36.65 MB/f | **28–34 MB/f** |
| **0.0 ms GPU Settles** | 11 / 12 | 8 / 12 | 0 / 11 | 11 / 12 | **11 / 12** |

1. **`rdna15` (Why naive in-place slowed down):** Moving *all* >4 GiB contiguous buffers (`561 MB`) to PCIe-mapped system memory cut CPU copy work, but starved the GPU by pulling hot `≥ 1 MiB` buffers out of 800 GB/s GDDR6.
2. **`rdna16` ([`4487275`](https://github.com/XaviFortes/VIctory-RDNA/commit/4487275)):** Keeping large (`≥ 1 MiB`) buffers in local GDDR6 while enabling `VICTORY_INPLACE=2` for small/medium buffers mapped **842 MB** in place and slashed CPU shader-write scans by **98%** (`28.4 MB → 0.6 MB`), but left CPU-side copy settles at up to `9.1 ms`.
3. **`rdna17` ([`5e6fec2`](https://github.com/XaviFortes/VIctory-RDNA/commit/5e6fec2)):** Enabling GPU-side `card_steps` copies/fills on in-place buffers dropped settles back to **`0.0 ms`** (11/12 samples).
4. **`rdna18` ([`4c210de`](https://github.com/XaviFortes/VIctory-RDNA/commit/4c210de)):** Enabling GPU-side `place_copy` on in-place buffers restored `VICTORY_JOIN_PARTS` from `13.8 → 21.4` joined submissions/frame, pushing Series X 4K into **22–25.4 FPS** with `VICTORY_PORT_BUFFERS=1` and `VICTORY_INPLACE=2` promoted into the default preset.

</details>

---

## 💻 PowerShell CLI (Windows)

Prefer scripting, benchmarking, or custom overrides? Use `scripts/play-rdna.ps1` (or pass flags directly to `launch.bat`):

```powershell
# From the game folder (auto-detects game_scarlett_master.exe and applies full RDNA3 preset)
.\VIctory-RDNA\scripts\play-rdna.ps1 -Log .\victory-rdna.log

# From the project folder with an explicit executable and Series S / Safe Mode
.\scripts\play-rdna.ps1 -Exe "D:\Games\XboxTitle\game_scarlett_master.exe" -Console series-s -Safe -Log .\series-s-safe.log

# Rebuild diagnostics binary with the live stats overlay enabled
.\scripts\play-rdna.ps1 -Build diag -Rebuild -More @{ VICTORY_PANEL = '1' } -Log .\diag.log
```

<details>
<summary><strong>All CLI Parameters</strong></summary>

| Parameter | Default | Purpose |
| --- | --- | --- |
| `-Exe <path>` | Auto-detect | Path to the title executable (auto-checks cwd, project parent, and project dir). |
| `-Console <mode>` | `series-x` | `series-x` or `series-s`. |
| `-Fps <N>` | `0` | Requests a target frame interval (`0` = title default). |
| `-Output <mode>` | Title default | Requests a specific output resolution (leave default on Series X for GTA VI). |
| `-Safe` / `-RayTracing` | Off | Matches the desktop launcher's Safe Mode and Ray Tracing toggles. |
| `-Build <profile>` | `release` | `release` or `diag`. |
| `-Rebuild` | Off | Forces a Cargo rebuild even if the binary already exists. |
| `-More @{...}` | Empty | Extra environment overrides applied on top of `scripts/rdna-preset.json`. |
| `-Log <path>` | None | Tees live console output to a log file (use unique names to keep history). |

</details>

---

## 🔍 Visual Quality, Honest Status & Known Limitations

We believe in transparent benchmarks and honest visual/stability reporting:

### Picture Quality (Maintainer Visual Assessment · Series X 4K)
- **Overall Picture:** Coherent lighting, natural color reproduction, and clean, readable world geometry, characters, and interiors at native **Xbox Series X 4K output (`3840 × 2160`)**.
- **Minor Visual Artifacts:**
  - **LOD & Texture Streaming Pop-In:** Some models and textures briefly appear at lower detail (**LOD pop-in** / **texture-streaming mipmap pop-in**) for a moment while higher-detail assets stream in.
  - **Distant Vegetation Flicker:** Occasional flickering can still appear on distant trees and foliage.
  - **Missing Host Pipelines:** A small number of draws are still dropped where host pipelines are not yet implemented, and the texture path logs refresh warnings.

### Playability & Stability
- **In-Game (Stage 6):** Boots into full playable 3D open-world gameplay on GTA VI. Earlier warm-cache builds ran **~40 minutes of hands-on gameplay** (`48 min 26 s` total), and recent `rdna14`–`rdna19` captures all exited cleanly with **`code 0`** (including the `9 min 38 s` `rdna18` benchmark). A dedicated **30+ minute hands-on Series X 4K playthrough** on the current fast build is next to confirm extended hands-on stability.
- **Long-Session GPU Resets:** While recent captures (`rdna11`–`rdna19`) exited cleanly, earlier extended runs encountered `DXGI_ERROR_INVALID_CALL` (`0x887A0001`) or a one-off `DXGI_ERROR_DEVICE_HUNG` (`0x887A0006`). Hardening multi-hour stability remains an ongoing priority.
- **Single-Machine Measurements:** The numbers above were captured on one RX 7900 XT system across live gameplay windows (not a locked deterministic replay). Community benchmarks on other RDNA2/RDNA3/RDNA4/NVIDIA GPUs are very welcome!

<details>
<summary><strong>Troubleshooting Quick Fixes</strong></summary>

- **Build failed:** Open the session log and check the first Cargo/linker error. Verify Rust and Visual Studio C++ Build Tools are installed.
- **Windows error 1455 (commit limit):** Check **Task Manager → Performance → Memory → Committed**. Close memory-heavy apps or increase your Windows paging file.
- **Crash or visual artifacts:** Stick to **Series X**, default **4K output**, **Ray tracing Off**, **FPS target `0`**, and try **Safe mode**. Check the session log and `crashes.log`.
- **Wine launch failed (Linux):** Verify `wine`, `winepath`, `target/release/victory.exe`, and your Wine prefix's VKD3D-Proton setup.

For deeper technical notes, input mappings, and upstream references, see [`docs/README-fxceless.md`](docs/README-fxceless.md) and [`docs/findings.md`](docs/findings.md).

</details>

---

## 🤝 Contribute

Want to help push `VIctory-RDNA` toward a locked **30 FPS native** on Series X 4K? We'd love your help with:

- **Community benchmarks** on RX 7900 XTX, RX 7900 XT, RX 7800 XT, RDNA2, or NVIDIA RTX cards.
- **Same-scene before/after traces** to isolate remaining CPU/GPU pipeline gaps.
- **Long-session stability reports** (`-Build diag` logs capturing any device removal or hang).

When opening an issue or PR, please include your **commit hash**, **GPU + driver version**, **console profile**, **output resolution**, and **session duration** — and never attach copyrighted game files.

---

## 🙏 Credits

`VIctory-RDNA` would not exist without the incredible work of the upstream VIctory community:

- **[Marvisak / VIctory](https://github.com/Marvisak/VIctory)** — Original loader, HLE runtime, Direct3D 12 backend, and shader recompiler.
- **[avatarmanz / VIctory-7900XTX](https://github.com/avatarmanz/VIctory-7900XTX)** — Direct upstream base, Renderer Ports 1–7, and foundational RDNA3 renderer engineering.
- **[fxceless / victory-fork](https://github.com/fxceless/victory-fork)** — Upstream optimizations and community improvements.
- **Community contributors:** [DexNom](https://github.com/DexNom/VIctory), [Belobug](https://github.com/Belobug/VIctory-Belobug-AMD), [FQY55kai](https://github.com/FQY55kai/ZJOU-VICTORY), [myokex](https://github.com/myokex/victory-amd), **Geckomacabre**, and **cxy0072019-sketch**.

## 📄 License

Licensed under **GPL-3.0-or-later**. See [`LICENSE`](LICENSE) and [`docs/asset-policy.md`](docs/asset-policy.md).
