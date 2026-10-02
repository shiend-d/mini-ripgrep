// ============================================================
// multiprocessing.rs - Pendekatan 3: Multiprocessing
// ============================================================
// Menggunakan std::process::Command untuk spawn child processes.
// Parent membagi file list → masing-masing child proses chunk-nya
// sendiri di memori terisolasi via stdin pipe → kirim hasil via stdout ke parent.
//
// Komunikasi: IPC Pipe (stdin untuk input file list, stdout untuk hasil count).

use std::env;
use std::io::{self, Read, Write};
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::time::Instant;

use crate::sequential::collect_files;

// Chunk size tetap 102 sesuai NIM
#[allow(dead_code)]
pub const CHUNK_SIZE: usize = 102; // Sesuai parameter NIM: 102

/// Bagi file ke N chunks (untuk N child processes)
fn chunk_by_workers(files: &[PathBuf], num_workers: usize) -> Vec<Vec<PathBuf>> {
    let per_worker = (files.len() + num_workers - 1) / num_workers;
    files.chunks(per_worker).map(|c| c.to_vec()).collect()
}

/// MODE WORKER (dipanggil oleh child process):
/// Membaca daftar path file dari stdin pipe, scan, print jumlah match ke stdout.
/// Dipanggil ketika program dijalankan dengan flag `--worker-mode`.
pub fn worker_mode(keyword: &str) {
    let mut input = String::new();
    if let Err(e) = io::stdin().read_to_string(&mut input) {
        eprintln!("[WORKER ERROR] Gagal membaca stdin: {}", e);
        println!("0");
        return;
    }

    let local_matches: usize = input
        .lines()
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .map(|line| crate::sequential::count_matches_in_file(Path::new(line), keyword))
        .sum();

    // Kirim hasil ke parent via stdout (satu angka saja)
    println!("{}", local_matches);
}

/// MODE PARENT: spawn child processes, streaming daftar file via stdin, kumpulkan hasilnya
pub fn run(dir: &str, keyword: &str, num_workers: usize) -> (usize, usize, f64) {
    let files = collect_files(dir);
    let total_files = files.len();

    if total_files == 0 {
        return (0, 0, 0.0);
    }

    // Path binary ini sendiri (untuk spawn child dengan flag --worker-mode)
    let current_exe = env::current_exe().expect("Tidak bisa mendapat path executable");

    // Bagi file list ke num_workers chunks
    let chunks = chunk_by_workers(&files, num_workers);

    let start = Instant::now();

    // Spawn semua child process secara paralel dengan stdin/stdout piped
    let mut child_handles = Vec::with_capacity(chunks.len());

    for chunk in chunks {
        let mut child = Command::new(&current_exe)
            .arg("--worker-mode")
            .arg(keyword)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()
            .expect("Gagal spawn child process");

        // Kirim list path file ke child via stdin pipe
        if let Some(mut stdin) = child.stdin.take() {
            let mut file_payload = String::new();
            for p in &chunk {
                file_payload.push_str(&p.to_string_lossy());
                file_payload.push('\n');
            }
            let _ = stdin.write_all(file_payload.as_bytes());
            // stdin ditutup otomatis saat out of scope, memberi EOF signal ke child
        }

        child_handles.push(child);
    }

    // Tunggu seluruh child process selesai dan akumulasikan hasil
    let mut total_matches = 0usize;
    for child in child_handles {
        let output = child.wait_with_output().expect("Gagal menunggu child process");
        let stdout = String::from_utf8_lossy(&output.stdout);
        total_matches += stdout.trim().parse::<usize>().unwrap_or(0);
    }

    let elapsed = start.elapsed().as_secs_f64();

    (total_files, total_matches, elapsed)
}
