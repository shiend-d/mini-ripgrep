# Mini-Ripgrep: High-Throughput Parallel Text Search Engine

A high-performance parallel text search engine written in Rust, inspired by [`ripgrep`](https://github.com/BurntSushi/ripgrep). It leverages **Thread-Local Buffering** and **Dynamic File Chunking** to search across thousands of files with maximum throughput and zero lock contention.

Developed for the Parallel and Distributed Computing course (*UTS Komputasi Paralel dan Terdistribusi*).

---

## Features & Execution Modes

1. **Sequential (`sequential` / `seq`)**:
   - Single-threaded baseline for speedup comparison ($S = 1.00\times$).
2. **Multithreading Final (`threaded` / `thread`)**:
   - High-throughput search using a thread pool and thread-local accumulator buffers (lock-free hot loop, single atomic addition per thread).
3. **Multithreading Bug (`threaded-bug` / `bug`)**:
   - Educational demonstration of **Lock Contention / Thread Serialization** caused by locking a shared `Arc<Mutex<usize>>` on every match inside the inner loop.
4. **Multiprocessing Final (`multiprocess` / `process`)**:
   - Isolated memory concurrency via OS child processes communicating over IPC stdout streams.
5. **Full Benchmark (`benchmark` / `bench`)**:
   - Automated benchmark suite across 3 dataset sizes (`1,102`, `11,102`, and `111,102` files) with varying worker counts (`2, 4, 8, 16`), complete with recapitulation tables, speedup calculations, and Amdahl's Law analysis.
6. **Interactive Wizard (`interactive` / running without arguments)**:
   - User-friendly terminal prompts to guide search configurations and parameters.

---

## Quick Start (Standard / Cargo)

### Prerequisites
- [Rust & Cargo](https://www.rust-lang.org/) (Rust 2021 edition or newer).

### Build
```bash
cargo build --release
```
Binary output will be available at `./target/release/mini_ripgrep`.

### General Usage

```bash
# 1. Interactive Wizard Mode (easiest way to start)
cargo run --release

# 2. Generate test datasets (Small, Medium, Large) with keyword embedded
cargo run --release -- generate KEYWORD

# 3. Run Multithreading with 8 threads
cargo run --release -- threaded KEYWORD 8

# 4. Run Multiprocessing with 4 child processes
cargo run --release -- multiprocess KEYWORD 4

# 5. Run Sequential baseline
cargo run --release -- sequential KEYWORD

# 6. Run Mutex Contention Bug mode
cargo run --release -- threaded-bug KEYWORD 8

# 7. Run Comprehensive Benchmark Suite (All datasets & worker counts)
cargo run --release -- benchmark KEYWORD
```

#### CLI Options
```text
mini_ripgrep [MODE] <KEYWORD> [WORKERS] [OPTIONS]

Options:
  -d, --dir <PATH>       Target directory (default: ./dataset_11102)
  -k, --keyword <KEY>    Search keyword
  -w, --workers <NUM>    Number of worker threads/processes (default: 8)
  -c, --count <NUM>      File count for dataset generation
  -h, --help             Show help message
```

---

## Nix / NixOS Usage

This repository includes a [`flake.nix`](./flake.nix) for reproducible dev environment on NixOS:

1. **Enter Development Shell**:
   ```bash
   nix develop
   ```
   *(Or automatically with `direnv` if `use flake` is in `.envrc`)*

2. **Use with Cargo**:
   Once inside the shell, all tools (`rustc`, `cargo`, `rust-analyzer`) are available. Simply run:
   ```bash
   cargo run --release -- benchmark KEYWORD
   ```

---

## Project Structure

```text
├── Cargo.toml               # Rust package metadata
├── flake.nix                # Nix Flake definition (NixOS support)
├── flake.lock               # Pinned Nix flake inputs
├── src/
│   ├── main.rs              # CLI entry point, benchmark runner & wizard
│   ├── dataset_gen.rs       # Synthetic dataset generator
│   ├── sequential.rs        # Baseline sequential search
│   ├── multithreading.rs    # Optimized thread-local parallel search
│   ├── multithreading_bug.rs# Mutex lock contention demonstration
│   └── multiprocessing.rs   # Multi-process IPC search
└── README.md
```
