# VIctory-RDNA

VIctory-RDNA is a Direct3D 12 compatibility layer for Xbox Series titles, tuned for AMD RDNA3 GPUs. The game launcher runs on Windows and Linux. The runtime runs directly on Windows and uses Wine on Linux. The project builds on [VIctory](https://github.com/Marvisak/VIctory) and [VIctory-7900XTX](https://github.com/avatarmanz/VIctory-7900XTX).

This is an experimental project. GTA VI is the title used for the performance measurements below. Compatibility varies by title, build, and console profile. Supply your own game files from a copy you are authorized to use and keep the executable alongside its data files.

**21.9 FPS at 4K output in Series X mode** on an **AMD Radeon RX 7900 XT**, at **45.6 ms/frame** in the latest capture. The build exited cleanly. An earlier build logged a hands-on session of about **40 minutes**. [GTA VI status](#gta-vi-status) · [Benchmarks](#gta-vi-measurements) · [Contribute](#contribute)

## GTA VI status

| Lane | Maintainer-reported status |
| --- | --- |
| Tested on | Windows, AMD Radeon RX 7900 XT, Direct3D 12. Xbox Series X profile with 3840 × 2160 presentation output. |
| Stage | **In-game (stage 6)**. The maintainer played the earlier warm-cache build by hand. |
| Picture | In-world rendering with known gaps. The latest log shows draws dropped for missing host pipelines and texture refresh warnings. A visual rating still needs a scene-by-scene check. |
| Speed | **12.1 FPS sampled median** during the earlier hands-on run. The latest capture reached **21.9 FPS (45.6 ms/frame)**, with a **17.9 FPS sampled median in its final minute**. |
| Stability | **About 40 minutes of hands-on gameplay** on the earlier warm build, which hit GPU device removal at **48 min 26 s** from startup. The earlier `rdna5` capture exited cleanly after **13 min 37 s** total. The latest capture exited cleanly after **5 min 52 s** total. |

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

#### Series X at 4K output · 26 November 2026

Two Windows release captures on the RX 7900 XT compare `rdna11` with `rdna12` ([`5663897`](https://github.com/XaviFortes/VIctory-RDNA/commit/5663897)). Both exited with code 0. The FPS and preparation ranges below summarize the final 60 seconds before window close. The submission, descriptor and lock counters show the final two readings in each capture.

| Metric | `rdna11` | `rdna12` |
| --- | ---: | ---: |
| Total runtime, clean exit | 6 min 32 s | 5 min 52 s |
| FPS samples in final minute | 11 | 12 |
| FPS range | 15.9–19.2 | **15.1–21.9** |
| Median FPS | 17.2 | **17.9** |
| Median frame time | 58.1 ms | **56.0 ms** |
| Submission preparation | 43.0–50.9 ms/frame | **36.8–44.0 ms/frame** |
| Copy preparation | 14.5–20.3 ms/frame | **11.7–15.4 ms/frame** |
| Submitting thread work | 45.2–47.0 ms/frame | **44.4–46.0 ms/frame** |
| Title wait at present | 10.5–11.6 ms/frame | **9.9–11.9 ms/frame** |
| Descriptor-sync wait | 0.27–2.00 ms/frame | **0.41–0.76 ms/frame** |
| Heap-buffer pass, final reading | 11.9 ms/frame | **8.1 ms/frame** |
| Port 7 needless exclusive lock acquisitions | 45.0–45.6/frame | **18.4–18.8/frame** |
| Samples with zero GPU settle wait | 10 of 11 | **11 of 12** |

The final-minute peak rose by **2.7 FPS**. The `rdna12` peak of **21.9 FPS** was recorded twice during the run, at **1,498** and **3,811** draws per frame. At **4,000 or more draws per frame**, the whole-capture sampled medians were **17.2 FPS** across 28 samples in `rdna11` and **16.7 FPS** across eight samples in `rdna12`. Workload varies across the captures.

The FPS counter measures frames accepted by the swap chain in periodic windows. These captures provide sampled medians and peaks. A sustained 20 FPS gameplay average remains to be demonstrated.

#### Renderer profiling

- **Port 7 journal.** The `ask_once_active` fast path cuts needless exclusive lock acquisitions by about **59%** in the final two readings. Those readings still show **92–112 waits for a journal part per frame**, or **4.84–6.34 ms/frame** summed across threads. Inspect page-to-part distribution before changing the number of parts. [Write-watch and journal code](crates/victory-gpu/src/watch.rs)
- **Submission costs.** Copy preparation reached **11.7–15.4 ms/frame** in the final minute, with a median of **43.5 MB/frame** in copy traffic. Copy-back had a median of **19.0 ms/frame**. Profile page-protection calls, write-watch queries, snapshots, and card copies to identify work still on the submitting thread. These counters can overlap. [Submission timing](crates/victory-gpu/src/host.rs) · [Shadow copies](crates/victory-gpu/src/shadow.rs)

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

Help reproduce the **21.9 FPS Series X result**, document picture quality in a specific scene, or isolate the long-session GPU failure. Useful contributions include a 10-minute hands-on benchmark, a same-scene before-and-after trace for a renderer change, or a diagnostics log that points to the command preceding device removal.

Include the tested commit, GPU and driver, console profile, output resolution, scene, and time under player control. Report average rendered FPS when available and describe how the run ended. Open an issue or pull request with a short summary and your own measurements. Keep game files out of submissions. [GitHub repository](https://github.com/XaviFortes/VIctory-RDNA)

## Credits

- **Marvisak / VIctory** for the original loader, HLE runtime, Direct3D 12 backend, and shader recompiler. [VIctory](https://github.com/Marvisak/VIctory)
- **avatarmanz / VIctory-7900XTX** for the direct upstream base and RDNA3 renderer work. [VIctory-7900XTX](https://github.com/avatarmanz/VIctory-7900XTX)
- **fxceless / victory-fork** for upstream optimizations and community contributions. [victory-fork](https://github.com/fxceless/victory-fork)
- **DexNom**, **Belobug**, **FQY55kai**, **myokex**, **Geckomacabre**, and **cxy0072019-sketch**. [DexNom](https://github.com/DexNom/VIctory) · [Belobug](https://github.com/Belobug/VIctory-Belobug-AMD) · [FQY55kai](https://github.com/FQY55kai/ZJOU-VICTORY) · [myokex](https://github.com/myokex/victory-amd)

## License

GPL-3.0-or-later. See [LICENSE](LICENSE) and the [asset policy](docs/asset-policy.md).
