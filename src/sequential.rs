// ============================================================
// sequential.rs - Pendekatan 1: Sequential Execution (Baseline)
// ============================================================
// Membaca file satu per satu di satu thread utama.
// Digunakan sebagai baseline untuk kalkulasi Speedup.

use std::fs;
use std::io::{self, BufRead};
use std::path::{Path, PathBuf};
use std::time::Instant;

/// Kumpulkan semua path file dari direktori target (sort untuk konsistensi)
pub fn collect_files(dir: &str) -> Vec<PathBuf> {
    let mut files: Vec<PathBuf> = fs::read_dir(dir)
        .unwrap_or_else(|_| panic!("Tidak bisa membaca direktori: {}", dir))
        .filter_map(|entry| {
            let entry = entry.ok()?;
            let path = entry.path();
            if path.is_file() {
                Some(path)
            } else {
                None
            }
        })
        .collect();
    files.sort(); // Sort agar urutan konsisten antar run
    files
}

/// Hitung berapa baris dalam file yang mengandung keyword
pub fn count_matches_in_file(path: &Path, keyword: &str) -> usize {
    let file = match fs::File::open(path) {
        Ok(f) => f,
        Err(_) => return 0,
    };
    let reader = io::BufReader::new(file);
    let mut count = 0;
    for line in reader.lines() {
        if let Ok(line) = line {
            if line.contains(keyword) {
                count += 1;
            }
        }
    }
    count
}

/// Jalankan sequential search. Returns (total_files, total_matches, elapsed_secs)
pub fn run(dir: &str, keyword: &str) -> (usize, usize, f64) {
    let files = collect_files(dir);
    let total_files = files.len();

    let start = Instant::now();
    let mut total_matches = 0usize;

    for file in &files {
        total_matches += count_matches_in_file(file, keyword);
    }

    let elapsed = start.elapsed().as_secs_f64();
    (total_files, total_matches, elapsed)
}
