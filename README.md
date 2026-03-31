# Flood

![Continuous integration](https://github.com/dlalic/flood/workflows/Continuous%20integration/badge.svg)
[![CodeFactor](https://www.codefactor.io/repository/github/dlalic/flood/badge)](https://www.codefactor.io/repository/github/dlalic/flood)

**Elevation void filling in the browser** – Rust + WebAssembly

A client‑side tool that fills depressions in digital elevation models (DEMs).  

## Usage

[Online demo](https://flood-dlalic-4e5cb431.koyeb.app/s)

### Build

```
npm run build
```

### Run
```
npm run start
```
Point your browser to http://localhost:8080

### Why this algorithm

Used the **Liu, Zhang, Xu (2017) algorithm** as the base. Since then, several more advanced algorithms have been published:

| Year | Authors | Approach | Complexity | Best for | Why not used here |
|------|---------|----------|------------|----------|------------------|
| 2001 | Planchon & Darboux | Iterative lowering | `O(N²)` | Small DEMs | Slow; multiple passes |
| 2006 | Wang & Liu | Priority‑queue (edge flooding) | `O(N log N)` | Hydrological flow | Designed for edges, not depressions |
| **2017** | **Liu, Zhang, Xu** | **Priority‑queue (depression filling)** | **`O(N log N)`** | **Regional DEMs** | **Simple, exact, single pass** |
| 2020 | Zhou et al. | Parallel multi‑core | `O(N log N)` parallel | Shared‑memory systems | Adds threading complexity |
| 2022 | Ceballos et al. | GPU‑based (CUDA) | `O(N)` | Massive DEMs (>1B cells) | Requires CUDA – not browser‑friendly |
| 2023 | Barnes & Lehman | Hybrid CPU/GPU | `O(N)` + overhead | Terrain‑scale rasters | Same GPU constraint |
| 2024 | Copernicus DEM team | Tiled + streaming | `O(N)` with I/O | Continental‑scale | Designed for offline processing |
| 2025 | Zhang et al. | Learned priors (ML) | `O(N)` + inference | Experimental | Not yet proven for hydrological accuracy |
