## AMD report: RX 7900 XTX / i7-13700KF / Windows 10, `2c5a395` against avatarmanz `c9c932c`

A community benchmark on an RX 7900 XTX, the card the README asks for. Fields as the README requests:

- **Commits:** this repo at `2c5a395` + avatarmanz/VIctory-7900XTX#165 (Agility SDK runtime export) + the `slot_story` build fix (see "Build note"). Compared with avatarmanz/VIctory-7900XTX `main` at `c9c932c` (renderer port 7 merged) + the same #165. Both built with `--profile diag --features diagnostics`. Neither tree has the port 1 ordering yet (`VICTORY_PORT_CONSTANTS_ORDER`, avatarmanz `7ab13ed`).
- **GPU and driver:** RX 7900 XTX, driver 32.0.32015.7029. CPU: i7-13700KF. OS: Windows 10 Pro 22H2 (19045), Agility SDK 1.620 runtime next to the executable.
- **Console profile:** Series X. **Output:** 4K with `VICTORY_RENDER_SCALE=0.5`, and 2560x1440.
- **Session:** 25 min a run, same save and same scripted input in every run. No device removal and no `ERROR` line in any run; I stopped each at 25 min.
- **Settings:** both sides the same base switches and renderer ports 1, 2, 5, 6, 7 (`VICTORY_PORT_CONSTANTS`, `_DESCRIPTORS`, `_CBV`, `_GUARD`, `_JOURNAL`). The RDNA side adds the rest of the "fast" preset as of `2c5a395`: `VICTORY_PORT_BUFFERS=1`, `VICTORY_INPLACE=2`, `VICTORY_ASK_SPREAD=1`, `VICTORY_RESIDENCY=1`, `VICTORY_SYNC_FINE=1`, `VICTORY_CARD_BUFFERS_MIN_MB=1`.

### Results

Frame time is the mean of the 5-second host lines with 3,000 to 4,999 draws (125 to 144 lines a run, about 4,800 draws on average). Prep is the CPU preparation of a frame from the same lines. Card is the median of the 3D engine busy samples (Windows performance counter, a sample about every 8 s) above 20%, so loading and menus are left out.

| Output | RDNA `2c5a395`, fast preset | avatarmanz `c9c932c`, same ports |
|---|---:|---:|
| 4K, render scale 0.5 | 35.0 ms, prep 23.2 ms, card 97% | 33.8 ms, prep 28.1 ms, card 92% |
| 2560x1440 | 33.5 ms, prep 24.0 ms, card 94% | 32.8 ms, prep 27.6 ms, card 88% |
| 2560x1440, without `PORT_BUFFERS` and `INPLACE` | 35.2 ms, prep 27.2 ms | |

### What it looks like from here

- On the RDNA tree, `VICTORY_PORT_BUFFERS=1` + `VICTORY_INPLACE=2` take 3.2 ms off the CPU preparation at 1440p (27.2 -> 24.0 ms) and the frame gets 1.7 ms faster (35.2 -> 33.5 ms).
- Against the avatarmanz tree the RDNA tree prepares 3.6-4.9 ms less, but the frame is not faster: 0.7 ms slower at 1440p, 1.2 ms slower at 4K with the card at 97%. The time moved from the CPU to the card, and at 4K the card is close to full.
- I did not run 4K without the two switches on the RDNA tree, so whether they are a net loss there is still open.

### Port 1 ordering in the preset

The "fast" preset now sets `VICTORY_PORT_CONSTANTS_ORDER=0` (before `8ba4f34` it was `VICTORY_PORT_CONSTANTS_ORDER_ALONE=0`). avatarmanz turned the ordering on in `7ab13ed` because without it trees and palms drop out, and measured that the batch of `ORDER_ALONE=0` was not enough (run pop-15, `3e04ca0` / `15c3d0e`). An option in between is avatarmanz/VIctory-7900XTX#438 (`VICTORY_PORT_CONSTANTS_ORDER_BATCH=1`, off by default), which batches only executions with nothing but state between them: in an A/B run on the avatarmanz tree on my 7900 XTX at 4K it took the median from 57.0 to 44.2 ms with about the same flicker. It is not a general win yet: an RTX 5090 replay under Proton got slower with it (comment on #438).

### Build note

The diagnostics build at `2c5a395` needs the `slot_story` fix that `rtx5070-nvidia.md` describes; I sent it tested as a separate PR (branch `rdna/fix-diag-dynamicbits`, reading a slot through `first_taken`).

*AI-assisted (Claude Code). Numbers from my run logs only, no game data or images.*
