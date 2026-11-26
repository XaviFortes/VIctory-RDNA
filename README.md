# VIctory-RDNA

VIctory-RDNA is a Direct3D 12 compatibility layer for Xbox Series titles, tuned for AMD RDNA3 GPUs. The game launcher runs on Windows and Linux. The runtime runs directly on Windows and uses Wine on Linux. The project builds on [VIctory](https://github.com/Marvisak/VIctory) and [VIctory-7900XTX](https://github.com/avatarmanz/VIctory-7900XTX).

This is an experimental project. GTA VI is the title used for the performance measurements below. Compatibility varies by title, build, and console profile. Supply your own game files from a copy you are authorized to use and keep the executable alongside its data files.

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
| `-Safe` / `-RayTracing` | Off | Selects the same renderer options as the desktop launcher. |
| `-Build <profile>` | `release` | Accepts `release` or `diag`. |
| `-Rebuild` | Off | Invokes Cargo even when the binary already exists. |
| `-More @{...}` | Empty | Applies environment overrides after the RDNA preset. |
| `-Log <path>` | No log file | Shows output in the terminal and writes it to the specified file. Use a different name each run to preserve earlier logs. |

From Command Prompt, `launch.bat -Log run.log` still invokes the command-line flow.

## RDNA3 tuning

- A CPU-side cache for descriptor strides and view addresses skips redundant uncached PCIe writes when a mirrored slot is unchanged.
- GPU-side comparisons for shader-write buffers of at least 1 MiB move roughly 76 MB of comparisons and snapshots per frame off the CPU in the measured workload.
- The descriptor patch path limits missed-store fallback warnings to the first ten occurrences.
- The launch preset also uses experimental paths from VIctory-7900XTX, including multithreaded command-list translation, zero-copy constants, descriptor tracking, GPU-side fills, AVX2 copies, and copy-queue texture uploads.

### GTA VI measurements

Recorded on an RX 7900 XT using the Series X profile with ray tracing disabled.

| Metric | Observed |
| --- | --- |
| Steady gameplay | 13.6–18.5 FPS (54–73 ms/frame) |
| GPU settle waits | 0.0 ms/frame |
| Heap buffer pass | 6.8–9.9 ms/frame |
| Texture refreshes on the main thread | 0.7–1.6 ms/frame |

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
