# VIctory-RDNA

VIctory-RDNA is a Direct3D 12 compatibility layer for Xbox Series titles, tuned for AMD RDNA3 GPUs. The game launcher runs on Windows and Linux. The runtime runs directly on Windows and uses Wine on Linux. The project builds on [VIctory](https://github.com/Marvisak/VIctory) and [VIctory-7900XTX](https://github.com/avatarmanz/VIctory-7900XTX).

This is an experimental project. GTA VI is the title used for the performance measurements below. Compatibility varies by title, build, and console profile. Supply your own game files from a copy you are authorized to use and keep the executable alongside its data files.

**23.2 FPS at 4K output in Series X mode** on an **AMD Radeon RX 7900 XT**, at **43.0 ms/frame** with **4,083 draws/frame**. The latest in-place buffer experiment recorded an **18.75 FPS sampled median in its final minute** and exited cleanly. An earlier build logged about **40 minutes of hands-on gameplay**. [GTA VI status](#gta-vi-status) · [Benchmarks](#gta-vi-measurements) · [Contribute](#contribute)

## GTA VI status

| Lane | Maintainer-reported status |
| --- | --- |
| Tested on | Windows, AMD Radeon RX 7900 XT, Direct3D 12. Xbox Series X profile with 3840 × 2160 presentation output. |
| Stage | **In-game (stage 6)**. The maintainer played the earlier warm-cache build by hand. |
| Picture | In-world rendering with known gaps. The latest log shows draws dropped for missing host pipelines and texture refresh warnings. A visual rating still needs a scene-by-scene check. |
| Speed | Latest Series X capture peaked at **23.2 FPS (43.0 ms/frame)** and recorded an **18.75 FPS sampled median in its final minute**. The earlier hands-on run had a **12.1 FPS sampled median**. |
| Stability | **About 40 minutes of hands-on gameplay** on the earlier warm build, which hit GPU device removal at **48 min 26 s** from startup. The earlier `rdna5` capture exited cleanly after **13 min 37 s** total. The latest `rdna17` capture exited cleanly after **4 min 46 s** total. |

These are measurements from one machine. The hands-on duration is the maintainer's estimate. The **12.1 FPS** figure comes from 509 periodic samples at 1,000 or more draws per frame, which can include cutscenes and transitions. Total runtime includes startup and loading. Output resolution describes the presentation surface.

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

#### In-place buffer experiments · 26 November 2026

Four Windows release captures on the **RX 7900 XT** used **Series X** with **3840 × 2160 output**. All four exited with code 0. These runs use different gameplay windows. The table compares the final minute of each capture, using the same sampling rule for all four.

| Final-minute metric | `rdna14` baseline | `rdna15` broad buffer placement | `rdna16` selective placement | `rdna17` GPU copy/fill fix |
| --- | ---: | ---: | ---: | ---: |
| Samples | 12 | 12 | 11 | 12 |
| Sampled median FPS | 17.3 | 14.85 | 15.9 | **18.75** |
| Sampled median frame time | 57.9 ms | 67.4 ms | 62.8 ms | **53.4 ms** |
| FPS range | 12.5–20.2 | 8.7–18.4 | 13.1–21.6 | **14.6–21.7** |
| CPU submission preparation | 39.4–51.5 ms | 32.5–40.3 ms | 33.8–43.9 ms | **30.4–39.6 ms** |
| CPU copy preparation | 12.6–20.5 ms | 10.4–15.6 ms | 10.4–15.1 ms | **9.0–13.5 ms** |
| Median copy preparation traffic | 47.5 MB/frame | 36.15 MB/frame | 36.1 MB/frame | 36.65 MB/frame |
| Samples reporting 0.0 ms of GPU settle waits | 11 of 12 | 8 of 12 | 0 of 11 | **11 of 12** |
| Total runtime, clean exit | 7 min 30 s | 4 min 48 s | 5 min 17 s | 4 min 46 s |

Across the entire `rdna17` capture, samples with **4,000 or more draws per frame** had an **18.7 FPS median** (18 samples) and peaked at **23.2 FPS at 43.0 ms/frame** with **4,083 draws/frame**. The equivalent `rdna14` subset had a **15.85 FPS median** (26 samples). These are periodic swap-chain samples. A same-scene comparison and a full-session gameplay average are still open.

#### Why lower CPU costs did not immediately raise FPS

1. **Broad placement traded CPU work for GPU memory access.** With `VICTORY_PORT_BUFFERS=1`, `rdna15` reported **124 contiguous regions in place (561 MB)** and a final card-copy reading of **12.0 MB sent / 31.4 MB fetched per frame**, down from **18.6 / 48.9 MB** in `rdna14`. CPU preparation fell, while median frame time rose. The placement policy let large, frequently accessed buffers move from local VRAM to GPU-accessible system memory. That explains the direction of the result, though these captures cannot isolate the PCIe cost. Commit [`4487275`](https://github.com/XaviFortes/VIctory-RDNA/commit/4487275) keeps buffers classified as large at title addresses above 4 GiB in local VRAM.
2. **Selective placement removed most CPU comparisons.** With the guard and `VICTORY_INPLACE=2`, `rdna16` reported **842 MB in place** across 3,345 private buffers, 91 windows, and 97 buffers and windows on the contiguous-section path. Final CPU shader-write comparisons dropped from **28.4 MB across 370 ranges** to **0.6 MB in one range**. The final copied constant-buffer count dropped from **1,528 to 510 per frame**. Title-memory reads still triggered GPU settles, reaching **9.1 ms/frame** in the final minute.
3. **GPU copies and fills removed most of those waits.** Commit [`5e6fec2`](https://github.com/XaviFortes/VIctory-RDNA/commit/5e6fec2) allows card-side copies and fills to use in-place buffers. In `rdna17`, settle waits read **0.0 ms** in 11 of the final 12 samples, with one **0.3 ms** reading. The last submitting-thread reading fell from **47.4 ms** in `rdna16` to **40.7 ms** in `rdna17`.

#### Remaining bottlenecks and reproduction

A **21.0 FPS** sample in `rdna17` took **47.7 ms/frame**. The queue meter recorded **13.4 ms** without this layer's work queued, including **11.8 ms** while the submitting thread was active. Subtracting the idle interval leaves an estimated **34.3 ms** of queue-busy time. The meter gives an estimate of queue activity. Direct GPU timing and a controlled run would be needed to establish a rendering ceiling.

The final readings show **13.8 joined submissions per frame** in `rdna17` versus **20.3** in `rdna14`, while total submissions stayed near 37. A same-scene trace can test whether the smaller batches explain the remaining gap. The in-place options are opt-in. Series S at a 60 FPS target is a follow-up test with performance still to be measured.

```powershell
# From the game folder, rebuild and capture the Series X comparison.
.\VIctory-RDNA\scripts\play-rdna.ps1 -Rebuild -Log .\series-x-inplace.log -More @{ VICTORY_PORT_BUFFERS = '1'; VICTORY_INPLACE = '2' }

# Try the same build in Series S mode and request a 60 FPS target.
.\VIctory-RDNA\scripts\play-rdna.ps1 -Console series-s -Fps 60 -Log .\series-s-inplace.log -More @{ VICTORY_PORT_BUFFERS = '1'; VICTORY_INPLACE = '2' }
```

## Known problems

- **Long-session stability.** The earlier hands-on run ended with `DXGI_ERROR_INVALID_CALL` (`0x887A0001`). A separate earlier capture hit `DXGI_ERROR_DEVICE_HUNG` (`0x887A0006`). The latest clean exit establishes the outcome of that shorter run.
- **Rendering gaps.** Missing host pipelines can drop draws, and the texture path still reports refresh warnings.

## Troubleshooting

- **Build failed.** Open the session log and look for the first Cargo or linker error. Check that Rust and the MSVC C++ Build Tools are installed, then reopen the launcher.
- **Wine launch failed.** Open the session log and check whether it found `wine`, `winepath`, and your Windows build. Check the Wine prefix's Direct3D 12 support if the runtime reports a graphics initialization error.
- **Title closes or renders incorrectly.** Try Series X with **Safe mode** enabled, **Ray tracing** disabled, and the FPS target set to `0`. Check the session log and the runtime's `crashes.log` for details.
- **Windows error 1455.** Check **Task Manager → Performance → Memory → Committed**. Free memory or increase the Windows paging file, then retry.

For command references, input mappings, and renderer notes, see [the upstream documentation](docs/README-fxceless.md) and [the technical findings](docs/findings.md).

## Contribute

Help reproduce the **23.2 FPS Series X result**, document picture quality in a specific scene, or isolate the long-session GPU failure. Useful contributions include a 10-minute hands-on benchmark, a same-scene before-and-after trace for a renderer change, or a diagnostics log that points to the command preceding device removal.

Include the tested commit, GPU and driver, console profile, output resolution, scene, and time under player control. Report average rendered FPS when available and describe how the run ended. Open an issue or pull request with a short summary and your own measurements. Keep game files out of submissions. [GitHub repository](https://github.com/XaviFortes/VIctory-RDNA)

## Credits

- **Marvisak / VIctory** for the original loader, HLE runtime, Direct3D 12 backend, and shader recompiler. [VIctory](https://github.com/Marvisak/VIctory)
- **avatarmanz / VIctory-7900XTX** for the direct upstream base and RDNA3 renderer work. [VIctory-7900XTX](https://github.com/avatarmanz/VIctory-7900XTX)
- **fxceless / victory-fork** for upstream optimizations and community contributions. [victory-fork](https://github.com/fxceless/victory-fork)
- **DexNom**, **Belobug**, **FQY55kai**, **myokex**, **Geckomacabre**, and **cxy0072019-sketch**. [DexNom](https://github.com/DexNom/VIctory) · [Belobug](https://github.com/Belobug/VIctory-Belobug-AMD) · [FQY55kai](https://github.com/FQY55kai/ZJOU-VICTORY) · [myokex](https://github.com/myokex/victory-amd)

## License

GPL-3.0-or-later. See [LICENSE](LICENSE) and the [asset policy](docs/asset-policy.md).
