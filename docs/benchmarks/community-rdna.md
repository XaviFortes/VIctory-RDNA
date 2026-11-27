# Community RDNA benchmarks

## RX 9070 XT Series X

KingKaom tested commit `9cd7782` with the diagnostics build and the shipped fast preset. The run was windowed on a 4K-class display. The tester translated shaders before launch and passed matching host settings via `-More` (`VICTORY_HOST_RDNA=1`, `VICTORY_HOST_WAVE=32`, and `VICTORY_VIEW_LIMIT=2097152`).

- **In-world window:** About 13 minutes, **22.6 FPS median (44.2 ms/frame)** over 153 samples.
- **Four scripted 40-second walks:** 22.0, 22.7, 22.4, and 21.8 FPS median, with 7–8 samples per window.
- **Exit:** Clean, with no reported exceptions or GPU device removal. The count of draws dropped without a host pipeline settled around 250–290 per frame in the steady world.

In a separate hands-on release-build run of about 3.5 minutes, the tester saw 16.1 FPS median and a clean exit. The intended host overrides did not reach that run. Jason's face rendered in the diagnostics run and was missing in the release run. The build, effective settings, and cache conditions differed, so the cause is undetermined.

[KingKaom's test report](https://github.com/XaviFortes/VIctory-RDNA/pull/3#issuecomment-7226296316)

## RX 7900 XTX Port 1 ordering

HAL9000RELOADED tested Series X on Windows 10 at 4K output with a 0.5 render scale. Each configuration ran for 10 minutes with a warmed pipeline cache and a scripted boat drive. The table uses log samples with 3,000–4,999 draws per frame. The scenes varied between runs.

| Build | Ordering | Frame time | CPU preparation | GPU busy |
| --- | --- | ---: | ---: | ---: |
| VIctory-RDNA `e3bfddf` | On | 67.7 ms | 21.1 ms | 98% |
| VIctory-RDNA `e3bfddf` | Off (fast preset) | 36.2 ms | 21.4 ms | 93% |
| Upstream `014d52d` | On | 67.4 ms | 23.7 ms | 97% |
| Upstream `014d52d` + `c110868` | On with batching | 41.3 ms | 23.2 ms | 93% |

Ordering addresses tree and palm dropouts. The upstream-only batching patch grouped eligible indirect executions, reducing the number of clamp passes from about 4,200 to 991 per frame in this test. An earlier upstream A/B still observed flicker with either ordering mode. These measurements cover one run per configuration and leave the visual and performance effect on VIctory-RDNA open.

[HAL9000RELOADED's test report and patch reference](https://github.com/XaviFortes/VIctory-RDNA/pull/3#issuecomment-7227560345)
