# VIctory-RDNA

VIctory-RDNA is a Direct3D 12 compatibility layer for Xbox Series titles, tuned for AMD RDNA3 GPUs. The game launcher runs on Windows and Linux. The runtime runs directly on Windows and uses Wine on Linux. The project builds on [VIctory](https://github.com/Marvisak/VIctory) and [VIctory-7900XTX](https://github.com/avatarmanz/VIctory-7900XTX).

This is an experimental project. GTA VI is the title used for the performance measurements below. Compatibility varies by title, build, and console profile. Supply your own game files from a copy you are authorized to use and keep the executable alongside its data files.

The latest GTA VI capture on an **RX 7900 XT** recorded a **15.75 FPS median across 134 heavy-rendering samples** at **4K output** in Series X mode. It reached **19.7 FPS at 50.8 ms/frame** in two samples with more than 4,000 draws per frame and exited cleanly after **13 min 37 s**. [GTA VI measurements](#gta-vi-measurements)

## Get started

- **System** with an x64 AVX2-capable CPU running Windows 10/11 or desktop Linux.
- **Desktop launcher** uses Python 3.10 or newer with Tkinter. On Windows, use the [Python installer](https://www.python.org/downloads/) with Tcl/Tk enabled. On Linux, install your distribution's Python Tk package, commonly named `python3-tk` or `python3-tkinter`.
- **Graphics** with Direct3D 12, Shader Model 6.6, and mesh shader support for in-world rendering. The default preset is tuned for AMD RDNA3, including the RX 7900 XT and XTX. Linux needs a Wine setup with Direct3D 12 support and a compatible Vulkan driver. [VKD3D-Proton](https://github.com/HansKristian-Work/vkd3d-proton)
- **Memory** of at least 32 GB is recommended for GTA VI, with 64 GB preferred. Available Windows commit also matters at startup.
- **Windows build tools** if you are building from source. Install Visual Studio Build Tools with the **Desktop development with C++** workload and install Rust. Cargo uses the project's pinned toolchain. [rustup](https://rustup.rs/) · [rust-toolchain.toml](rust-toolchain.toml)

```powershell
git clone https://github.com/XaviFortes/VIctory-RDNA.git
cd VIctory-RDNA
```

1. **Open the game library.** On Windows, double-click `launch.bat`. On Linux, run `sh ./launch.sh` from the project folder.
2. **Select a game.** On first run the launcher looks for GTA VI's `game_scarlett_master.exe` in the project folder, its parent, and the folder you started from. Use **Add game…** to select an executable elsewhere or add another title. Entries are saved for the next time you open the launcher.
3. **Choose a console profile and click Play.** On Windows the launcher builds `victory.exe` if the selected build is missing. On Linux, supply the Windows build as described below. Use **Open last log** to see build or runtime output.

### Linux setup

1. Install Wine so both `wine` and `winepath` are on your `PATH`. Use an x64 Wine environment with Direct3D 12 support. Start the launcher with `WINEPREFIX` set if you use a separate prefix.
2. Build `victory.exe` on Windows with `cargo build --locked --release -p victory`, or obtain that build from someone you trust. Copy it to `target/release/victory.exe` on Linux, or select the executable under **Advanced**. For a diagnostics build, use `target/diag/victory.exe` and choose **Diagnostics**.
3. Run the launcher from your Linux desktop. Title execution through Wine is experimental. Compatibility depends on the Wine setup, graphics driver, and game.

```sh
WINEPREFIX="$HOME/.wine-victory" sh ./launch.sh
```

### Launch settings

| Setting | Default | Behavior |
| --- | --- | --- |
| Console profile | Xbox Series X | Selects the console characteristics presented to the title, such as memory budgets. GTA VI was benchmarked with Series X. Series S may behave differently. |
| FPS target | `0` | Uses the title's frame pacing. Set a positive target such as `60` to request a different frame interval. Actual FPS depends on performance. |
| Safe mode | Off | Disables newer experimental renderer paths. Try it for crashes or rendering issues. |
| Ray tracing | Off | Enables the title's ray-traced indirect lighting at a performance cost. |
| Build profile | Release | Use **Diagnostics** to build with runtime diagnostics. |
| Rebuild before play | Off | On Windows, runs Cargo for the selected profile before launch. |
| Environment overrides | Empty | Under **Advanced**, enter one `VICTORY_NAME=value` per line. For example, `VICTORY_PANEL=1`. |

### Logs

The desktop launcher saves a fresh timestamped log for every session. Use **Logs folder** to find previous sessions. For an experiment, enter a unique filename such as `series-s-after-change.log` or browse to a full path. A filename alone goes into the Logs folder. The launcher protects existing logs and clears the custom name after starting a session so the next run gets a fresh log by default.

| Platform | Launcher data folder |
| --- | --- |
| Windows | `%LOCALAPPDATA%\VIctory-RDNA` |
| Linux | `${XDG_DATA_HOME:-$HOME/.local/share}/victory-rdna` |

The launcher keeps `games.json` and the `logs` directory here. Title saves, `crashes.log`, and derived data are handled by the runtime. Their defaults are `%LOCALAPPDATA%\victory\data` and `%LOCALAPPDATA%\victory\cache` inside Windows or the selected Wine prefix. See [victory.example.toml](victory.example.toml) for runtime path overrides.

## Launch from PowerShell on Windows

From the **game folder**, with the project cloned inside it, keep using your existing command. Relative `-Log` paths are resolved from the directory you ran the command in.

```powershell
.\VIctory-RDNA\scripts\play-rdna.ps1 -Log .\victory-rdna.log
.\VIctory-RDNA\scripts\play-rdna.ps1 -Log .\after-renderer-change.log
```

From the **project folder**, pass `-Exe` to point to a game stored elsewhere.

```powershell
.\scripts\play-rdna.ps1 -Exe "D:\Games\XboxTitle\game_scarlett_master.exe" -Console series-s -Safe -Log .\series-s-safe.log
.\scripts\play-rdna.ps1 -Fps 60 -Build diag -Rebuild -More @{ VICTORY_PANEL = '1' } -Log .\diag-60.log
```

| Parameter | Default | Purpose |
| --- | --- | --- |
| `-Exe <path>` | Auto-detect | Explicit title executable. Auto-detection checks the calling directory, then the project's parent and the project itself. |
| `-Console <mode>` | `series-x` | Accepts `series-x` or `series-s`. |
| `-Fps <N>` | `0` | Controls the target frame interval. |
| `-Output <mode>` | Title default | Requests a supported output mode such as `1440p` or `1080p`. |
| `-Safe` / `-RayTracing` | Off | Selects the same renderer options as the desktop launcher. |
| `-Build <profile>` | `release` | Accepts `release` or `diag`. |
| `-Rebuild` | Off | Invokes Cargo even when the binary already exists. |
| `-More @{...}` | Empty | Applies environment overrides after the RDNA preset. |
| `-Log <path>` | No log file | Shows output in the terminal and writes it to the specified file. Use a different name each run to preserve earlier logs. |

From Command Prompt, `launch.bat -Log run.log` still invokes the command-line flow.

## RDNA3 tuning

- A CPU-side cache for descriptor strides and view addresses skips redundant uncached PCIe writes when a mirrored slot is unchanged.
- GPU-side comparisons for shader-write buffers of at least 1 MiB reduce the amount of data the CPU has to compare and snapshot.
- The descriptor patch path limits missed-store fallback warnings to the first ten occurrences.
- The launch preset also uses experimental paths from VIctory-7900XTX, including multithreaded command-list translation, zero-copy constants, descriptor tracking, GPU-side fills, AVX2 copies, and copy-queue texture uploads.

### GTA VI measurements

#### Latest Series X capture

`victory-rdna5.log` records a **26 November 2026** Windows D3D12 release run on an **RX 7900 XT**, using the RDNA3 Fast preset and a **3840 × 2160 presentation surface**. Heavy rendering first appeared at **1 min 57 s**. The run ended through the normal window-close path with **exit code 0**.

| Median across heavy-rendering samples | Earlier warm run | Latest run |
| --- | ---: | ---: |
| Samples | 509 | 134 |
| Rendered FPS | 12.1 | **15.75** |
| Frame time | 82.9 ms | **63.6 ms** |
| Submission preparation | 57.9 ms | 45.35 ms |
| Copy preparation | 24.3 ms (73.3 MB/frame) | 15.8 ms (53.75 MB/frame) |
| Heap-buffer pass | 12.9 ms | 11.15 ms |
| Copy-back | 22.3 ms | 20.85 ms |
| GPU settle waits | 0.0 ms | 0.0 ms |

The latest capture's sampled median was **30% higher**, with a **23% lower median frame time**. The runs covered different durations and workloads, so these figures describe the two captures. In the latest run's final minute, the heap-buffer pass was **8.55 ms** at the median and GPU settle waits were **0.4 ms** at the median. The two **19.7 FPS** samples came at **4,073** and **4,371** draws per frame. A lighter sample reached **24.9 FPS** at **1,415** draws per frame.

Port 6 (`VICTORY_PORT_GUARD=1`) was active. At shutdown, its counters showed **452.5 kernel queries skipped per frame**, **156.3 faults handled under a shared lock** with **0.70 ms/frame** waiting for that lock, and **121.5 refresh guards handled in shared mode** per frame.

The comparison uses periodic swap-chain samples with at least 1,000 draws per frame. The 13 min 37 s is total runtime, including startup and loading. A full-session FPS average and continuous time under player control are unverified. The largest measured CPU-side costs are still submission preparation and copy-back. Those timings can overlap, and the GPU settle counter measures synchronization waits.

#### Warm-cache run with direct constant-buffer reads

On **26 November 2026**, `victory-rdna4-warm.log` recorded a Windows D3D12 run on an **RX 7900 XT**, using the **RDNA3 Fast** release preset and **Series X** profile at **3840 × 2160 output**. The run reached heavy rendering at about **1 min 08 s** and continued until a GPU device removal at **48 min 26 s**.

| Metric | Heavy-rendering samples across the run | Final 60 seconds before device removal |
| --- | ---: | ---: |
| Samples | 509 | 11 |
| Median FPS | **12.1** | **8.5** |
| Median frame time | 82.9 ms | 117.7 ms |
| Median submission preparation | 57.9 ms | 63.6 ms |
| Median copy preparation | 24.3 ms (73.3 MB/frame) | 23.3 ms (72.7 MB/frame) |
| Median heap-buffer pass | 12.9 ms | 22.9 ms |
| Median GPU settle waits | 0.0 ms | 17.1 ms |

The direct constant-buffer path (`VICTORY_PORT_CBV=1`) recorded a median of **14,227 table constant-buffer reads in place per frame**, avoiding about **5,446 KB/frame** of copies in samples with at least 1,001 direct reads. The fallback still copied a median of **1,305** bindings per frame in those same samples.

The timing table uses periodic swap-chain measurements with at least 1,000 draws per frame. The elapsed time includes startup and loading. The capture does not establish continuous player-controlled time or a full-session FPS average.

#### Short captures

Two local captures from **26 November 2026** on Windows D3D12 with an **AMD Radeon RX 7900 XT**. Both used the **RDNA3 Fast** release preset, **Xbox Series X**, the title's **33.333 ms** frame interval, SDR, and a **3840 × 2160 presentation surface**. That is the output size reported by the swap chain. The logs do not report the internal render resolution. The fast preset leaves ray tracing off by default.

| Session | `victory-rdna2.log` | `victory-rdna3.log` |
| --- | ---: | ---: |
| First presentation surface logged | 0.99 s | 1.00 s |
| Total logged runtime, including startup and loading | 4 min 20 s | 4 min 11 s |
| Median FPS before sustained high draw counts | 30.0 | 30.0 |
| First sample at 1,000+ draws/frame | 1 min 26 s | 56 s |
| Samples at 1,000+ draws/frame | 34 | 37 |
| Median FPS at 1,000+ draws/frame | 13.05 | 12.5 |
| FPS range at 1,000+ draws/frame | 6.3–17.7 | 0.2–19.3 |
| Median frame time at 1,000+ draws/frame | 76.6 ms | 80.1 ms |
| 90th-percentile sampled frame time at 1,000+ draws/frame | 119.8 ms | 155.7 ms |
| Samples at 10 FPS or higher within that range | 29 of 34 | 26 of 37 |
| Final 60 s median FPS | **13.6** | **12.9** |
| Final 60 s FPS range | 10.9–16.1 | 10.2–15.8 |
| Final 60 s median frame time | **73.4 ms** | **77.5 ms** |
| Final 60 s frame-time range | 62.0–92.1 ms | 63.4–97.7 ms |
| Shutdown | Window closed, exit code 0 | Window closed, exit code 0 |

The second capture reached **19.3 FPS at 51.9 ms/frame** in one sample. It also had samples at **4,625.2 ms/frame** and **1,131.3 ms/frame** after the draw count increased. Both captures ended via the window-close path, with no ERROR-level messages or GPU device loss reported. The four-minute runtimes include startup and loading, and continuous time under player control was not annotated.

**How to read these numbers.** The runtime counts frames accepted by the swap chain. The FPS and frame-time lines appear about every five seconds, with each line summarizing the latest measurement window of at least one second. The medians, ranges, and percentile above use those sampled windows. The logs do not contain individual frame times, so they cannot provide a session-wide FPS average or 1% lows. The 1,000-draw threshold helps separate heavy rendering from startup and menus and can include cutscenes and transitions. The final-minute rows include all samples logged during the 60 seconds before the window closed (12 in the first capture and 11 in the second).

Both runs report the same title executable fingerprint and load an existing pipeline cache. The capture names identify sessions on the same GPU. The logs do not identify the VIctory-RDNA commit or the route played in each run, so differences between the captures cannot be assigned to a particular change.

<details>
<summary>Renderer timings, memory and diagnostics</summary>

The tables below show the median per-frame value in the final 60 seconds unless a row says otherwise. Timings on different threads and queues overlap. The all-thread rows sum worker time.

| Renderer timing | `victory-rdna2.log` | `victory-rdna3.log` |
| --- | ---: | ---: |
| Preparing submissions | 60.8 ms | 59.1 ms |
| Copy preparation | 23.9 ms | 21.7 ms |
| Heap buffer pass | 19.4 ms | 13.9 ms |
| Texture work during submission preparation | 1.6 ms | 1.9 ms |
| Texture refreshes across all threads | 12.7 ms | 13.4 ms |
| Copy-back | 34.6 ms | 32.6 ms |
| Merging fetched data | 18.7 ms | 18.2 ms |
| Asynchronous submission thread work | 64.3 ms | 62.3 ms |
| Title-side waits at presents | 10.7 ms | 19.0 ms |
| GPU settle waits | 0.15 ms | 0.0 ms |
| Additional card-memory waits | 11.4 ms | 10.0 ms |
| Time without this layer's work queued on the GPU | 26.4 ms | 31.5 ms |
| GPU busy estimate from queue timing | 63.5% | 58% |
| Command-list translation across all workers | 349.2 ms | 397.1 ms |

The second capture had a lower median heap-buffer pass time and shorter copy preparation, while present waits and median frame time were higher. The GPU busy estimate comes from the runtime's queue timing. The settle counter measures wait time at a specific synchronization point.

| Rendering and data movement | `victory-rdna2.log` | `victory-rdna3.log` |
| --- | ---: | ---: |
| Draws per frame | 5,433 | 5,625 |
| Draws dropped per frame for missing host pipelines | 416 | 425 |
| Indirect drops reported per frame | 0 | 0 |
| Asynchronous submissions per frame | 28.7 | 27.6 |
| Command-list calls translated per frame | 42,437 | 46,673 |
| Copy preparation traffic per frame | 90.0 MB | 86.8 MB |
| Texture traffic in preparation per frame | 19.0 MB | 24.9 MB |
| Copy-back traffic per frame | 45.0 MB | 44.0 MB |
| Title-buffer data compared on the CPU per frame | 28.4 MB | 28.8 MB |
| Pieces compared on the GPU per frame | 11,165 | 11,139 |
| Data fetched via the card-memory path per frame | 88.7 MB | 94.8 MB |
| Constant buffer views bound directly per frame | Counter absent | 415 (85.3 KB) |

| Memory and diagnostics | `victory-rdna2.log` | `victory-rdna3.log` |
| --- | ---: | ---: |
| Local video memory in use at the last logged budget change | 12,901 MB | 12,896 MB |
| Shared GPU memory in use at that snapshot | 10,755 MB | 10,340 MB |
| Local video-memory budget at that snapshot | 16,705 MB | 16,668 MB |
| Indexed texture uploads logged by the last meter line | 177,557 (14,790 MB) | 124,930 (8,810 MB) |
| Host pipelines loaded from cache by shutdown | 2,738 | 2,748 |
| Host pipelines created during the session | 9 | 7 |
| WARN-level messages | 210 | 157 |
| Warnings about indexed textures changing after upload | 151 | 99 |

The memory figures are D3D12 budget snapshots. Texture upload volume is cumulative. The missing-pipeline drops and texture warnings document remaining renderer limitations. Neither capture records enough visual detail to score picture quality.

</details>

## Known problems

- **Long-session stability.** The earlier warm run ended with `DXGI_ERROR_INVALID_CALL` (`0x887A0001`) after **48 min 26 s**. The final sample was **6.0 FPS at 167.6 ms/frame**, with **42.5 ms** in the heap-buffer pass and **21.5 ms** in GPU settle waits. The runtime recorded large resource sweeps before the failure. The cause remains unverified, and the latest clean run was shorter.
- **Rendering gaps.** Missing host pipelines can drop draws, and the texture path still reports refresh warnings.

These are two separate follow-up tests for the warm run. Their effect on frame rate and stability has yet to be measured.

```powershell
.\VIctory-RDNA\scripts\play-rdna.ps1 -Output 1440p -Log .\victory-rdna-1440p.log
.\VIctory-RDNA\scripts\play-rdna.ps1 -More @{ VICTORY_RESIDENCY = '1' } -Log .\victory-rdna-residency.log
```

## Troubleshooting

- **Build failed.** Open the session log and look for the first Cargo or linker error. Check that Rust and the MSVC C++ Build Tools are installed, then reopen the launcher.
- **Wine launch failed.** Open the session log and check whether it found `wine`, `winepath`, and your Windows build. Check the Wine prefix's Direct3D 12 support if the runtime reports a graphics initialization error.
- **Title closes or renders incorrectly.** Try Series X with **Safe mode** enabled, **Ray tracing** disabled, and the FPS target set to `0`. Check the session log and the runtime's `crashes.log` for details.
- **Windows error 1455.** Check **Task Manager → Performance → Memory → Committed**. Free memory or increase the Windows paging file, then retry.

For command references, input mappings, and renderer notes, see [the upstream documentation](docs/README-fxceless.md) and [the technical findings](docs/findings.md).

## Credits

- **Marvisak / VIctory** for the original loader, HLE runtime, Direct3D 12 backend, and shader recompiler. [VIctory](https://github.com/Marvisak/VIctory)
- **avatarmanz / VIctory-7900XTX** for the direct upstream base and RDNA3 renderer work. [VIctory-7900XTX](https://github.com/avatarmanz/VIctory-7900XTX)
- **fxceless / victory-fork** for upstream optimizations and community contributions. [victory-fork](https://github.com/fxceless/victory-fork)
- **DexNom**, **Belobug**, **FQY55kai**, **myokex**, **Geckomacabre**, and **cxy0072019-sketch**. [DexNom](https://github.com/DexNom/VIctory) · [Belobug](https://github.com/Belobug/VIctory-Belobug-AMD) · [FQY55kai](https://github.com/FQY55kai/ZJOU-VICTORY) · [myokex](https://github.com/myokex/victory-amd)

## License

GPL-3.0-or-later. See [LICENSE](LICENSE) and the [asset policy](docs/asset-policy.md).
