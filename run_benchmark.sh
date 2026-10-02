#!/bin/bash
# ============================================================
# run_benchmark.sh - Script Benchmark Lengkap Mini-Ripgrep
# UTS Komputasi Paralel dan Terdistribusi
# NIM: 247006111102 | Parameter: 102
# ============================================================

set -e  # Exit on any error

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
BINARY="$SCRIPT_DIR/target/release/mini_ripgrep"
DATASET_DIR="$SCRIPT_DIR/dataset_text"
KEYWORD="${1:-ERROR_102}"

echo "╔══════════════════════════════════════════════════════╗"
echo "║     MINI-RIPGREP - UTS KPT BENCHMARK RUNNER          ║"
echo "╚══════════════════════════════════════════════════════╝"
echo "Keyword Pencarian: \"$KEYWORD\""
echo ""

# ── 1. Build Release ─────────────────────────────────────────
echo "⟳  [BUILD] Compiling in release mode..."
cd "$SCRIPT_DIR"
cargo build --release 2>&1
echo "✓  Build berhasil!"
echo ""

# ── 2. Generate Dataset (jika belum ada) ─────────────────────
if [ ! -d "$DATASET_DIR" ] || [ "$(ls -A "$DATASET_DIR" 2>/dev/null | wc -l)" -lt 100 ]; then
    echo "⟳  [GEN] Generating dataset (11.102 file)..."
    "$BINARY" generate "$KEYWORD" 11102
    echo ""
else
    echo "✓  Dataset sudah ada ($DATASET_DIR)"
    echo "   Files: $(ls "$DATASET_DIR" | wc -l)"
    echo ""
fi

# ── 3. Jalankan Semua Mode ───────────────────────────────────
echo "⟳  [RUN] Menjalankan semua mode benchmark..."
echo ""

"$BINARY" benchmark "$KEYWORD"

echo ""
echo "╔══════════════════════════════════════════════════════╗"
echo "║     SEMUA BENCHMARK SELESAI DIJALANKAN               ║"
echo "╚══════════════════════════════════════════════════════╝"
