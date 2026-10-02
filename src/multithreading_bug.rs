// ============================================================
// multithreading_bug.rs - Pendekatan 2: Multithreading dengan BUG
// ============================================================
// BUG SKENARIO: Mutex dikunci di INNER LOOP (setiap baris yang match)
// Akibat: Ribuan lock/detik → Mutex Contention → Speedup < 1.0x
//
// Ini adalah demonstrasi INTENTIONAL BUG untuk keperluan akademis UTS.
// Lihat multithreading.rs untuk versi yang sudah diperbaiki.

use std::fs;
use std::io::{self, BufRead};
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Instant;

use crate::sequential::collect_files;

// Chunk size sesuai parameter NIM: 102
pub const CHUNK_SIZE: usize = 102;

/// Bagi daftar file menjadi chunks dengan ukuran tetap (CHUNK_SIZE)
fn chunk_files(files: &[PathBuf], chunk_size: usize) -> Vec<Vec<PathBuf>> {
    files
        .chunks(chunk_size)
        .map(|c| c.to_vec())
        .collect()
}

/// Hitung match di satu file — TANPA lock (digunakan internal per thread)
fn count_in_file_raw(path: &Path, keyword: &str) -> Vec<bool> {
    let file = match fs::File::open(path) {
        Ok(f) => f,
        Err(_) => return vec![],
    };
    let reader = io::BufReader::new(file);
    reader
        .lines()
        .filter_map(|l| l.ok())
        .map(|l| l.contains(keyword))
        .collect()
}

/// ⚠️  VERSI BUG: Mutex dikunci di INNER LOOP per baris yang cocok
///
/// Masalah: Setiap kali satu baris cocok, thread harus:
///   1. Mengambil lock global (blocking jika thread lain pegang)
///   2. Increment counter
///   3. Melepas lock
///
/// Dengan 8 thread dan ~4080 match tersebar di 1020 file,
/// ada ribuan operasi lock/unlock per detik →
/// Thread lebih banyak menunggu lock daripada bekerja.
///
/// Dampak nyata: Speedup ≈ 0.29x (LEBIH LAMBAT dari sequential)
pub fn run(dir: &str, keyword: &str, num_workers: usize) -> (usize, usize, f64) {
    let files = collect_files(dir);
    let total_files = files.len();

    // ⚠️ BUG: Shared Mutex yang akan diperebutkan dari banyak thread
    let shared_counter: Arc<Mutex<usize>> = Arc::new(Mutex::new(0));

    // Bagi file menjadi chunks (sesuai CHUNK_SIZE = 102)
    let _chunks = chunk_files(&files, CHUNK_SIZE); // tetap ada untuk dokumentasi chunk logic

    // Bagi chunks ke worker threads (num_workers thread, masing-masing ambil beberapa chunk)
    // Untuk sederhananya: setiap thread mendapat sejumlah chunk proporsional
    let files_arc = Arc::new(files.clone());
    let keyword_arc = Arc::new(keyword.to_string());

    // Re-chunk langsung berdasarkan num_workers untuk distribusi yang lebih merata
    let per_worker = (total_files + num_workers - 1) / num_workers;
    let worker_chunks: Vec<Vec<PathBuf>> = files_arc
        .chunks(per_worker)
        .map(|c| c.to_vec())
        .collect();

    let start = Instant::now();

    let handles: Vec<thread::JoinHandle<()>> = worker_chunks
        .into_iter()
        .map(|chunk| {
            let counter = Arc::clone(&shared_counter);
            let kw = Arc::clone(&keyword_arc);
            thread::spawn(move || {
                for file_path in &chunk {
                    let line_results = count_in_file_raw(file_path, &kw);
                    for is_match in line_results {
                        if is_match {
                            // ⚠️ BUG: Lock diambil DI DALAM inner loop!
                            // Setiap match → satu operasi lock!
                            let mut lock = counter.lock().unwrap();
                            *lock += 1;
                            // Lock dilepas di sini (drop) → langsung diperebutkan lagi
                        }
                    }
                }
            })
        })
        .collect();

    for handle in handles {
        handle.join().expect("Thread panicked");
    }

    let elapsed = start.elapsed().as_secs_f64();
    let total_matches = *shared_counter.lock().unwrap();

    (total_files, total_matches, elapsed)
}
