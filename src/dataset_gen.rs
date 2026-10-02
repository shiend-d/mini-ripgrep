// ============================================================
// dataset_gen.rs - Generator Dataset Teks untuk Mini-Ripgrep
// ============================================================
// Membuat file teks di folder target:
//   - dataset_1102   (1.102 file - 4 digit NIM)
//   - dataset_11102  (11.102 file - 5 digit NIM)
//   - dataset_111102 (111.102 file - Skala Besar)
//
// Setiap file berisi 50 baris dengan 4 baris mengandung keyword.

use std::fs;
use std::io::Write;
use std::path::Path;

#[allow(dead_code)]
pub const DEFAULT_NUM_FILES: usize = 11102; // 5 digit akhir NIM: 247006111102
pub const LINES_PER_FILE: usize = 50;       // 50 baris/file → generate cepat
pub const MATCHES_PER_FILE: usize = 4;

pub const DATASET_CONFIGS: &[(&str, usize)] = &[
    ("./dataset_1102", 1102),
    ("./dataset_11102", 11102),
    ("./dataset_111102", 111102),
];

// Baris-baris "noise" yang tidak mengandung keyword
static NOISE_LINES: &[&str] = &[
    "System initialized successfully.",
    "Loading configuration from disk...",
    "Connection established to remote host.",
    "Processing batch job #4721.",
    "Cache hit ratio: 98.7%",
    "Allocated 512MB memory for worker pool.",
    "Thread scheduler started with 8 workers.",
    "File checksum verified: OK",
    "Compressing output buffer to 64KB chunks.",
    "Database query executed in 3.2ms.",
    "Network packet received from 192.168.1.1.",
    "User authentication token refreshed.",
    "Background sync task completed.",
    "Index rebuild finished: 1.4 seconds.",
    "Disk write speed: 420 MB/s.",
    "CPU utilization: 67%",
    "Memory pressure: LOW",
    "Task queue depth: 12 items.",
    "Snapshot saved to /var/data/snapshot_v3.bin",
    "Log rotation triggered at midnight.",
    "Parsing structured document: report_q3.xml",
    "API response status: 200 OK",
    "Retry attempt 1 of 3...",
    "Idle worker thread parking...",
    "Wakeup signal received from scheduler.",
    "Output buffer flushed to stdout.",
    "Runtime assertion passed: invariant holds.",
    "Compiled regex pattern: [a-z0-9_]+",
    "Spawning child process for archival task.",
    "Remote I/O latency: 8ms avg.",
    "Starting graceful shutdown sequence.",
    "All handles closed. Resources freed.",
    "Waiting for pending writes to complete...",
    "Pipeline stage 3 flushed.",
    "Merkle root computed: a3f9e...",
    "Heartbeat timeout: 30 seconds.",
    "Replication lag: 0ms (in sync).",
    "Compaction job scheduled for 02:00.",
    "Read-ahead buffer pre-loaded.",
    "Fan speed adjusted: 1800 RPM.",
    "Thermal throttling: INACTIVE",
    "BIOS clock synchronized with NTP.",
    "Module libfoo.so unloaded safely.",
    "Benchmark iteration 50 of 100 complete.",
    "Checkpointing state to persistent store.",
    "Garbage collector ran: freed 3.2MB.",
    "Signal handler registered for SIGTERM.",
    "Watcher thread resumed after I/O wait.",
];

pub fn generate_dataset(dir: &str, keyword: &str, num_files: usize) {
    let path = Path::new(dir);
    if !path.exists() {
        fs::create_dir_all(path).expect("Gagal membuat direktori dataset");
        println!("[GEN] Direktori dibuat: {}", dir);
    } else {
        println!("[GEN] Direktori target: {}", dir);
    }

    println!("[GEN] Keyword yang di-embed: \"{}\"", keyword);
    println!("[GEN] Membuat {} file teks di '{}'...", num_files, dir);

    for i in 1..=num_files {
        let filename = format!("{}/file_{:04}.txt", dir, i);
        let mut file = fs::File::create(&filename)
            .unwrap_or_else(|_| panic!("Gagal membuat file: {}", filename));

        // Tentukan posisi baris yang akan berisi keyword (tersebar merata)
        // baris ke-5, 15, 30, 45 dari 50 baris
        let match_positions: std::collections::HashSet<usize> = [5, 15, 30, 45].iter().cloned().collect();
        assert_eq!(match_positions.len(), MATCHES_PER_FILE);

        for line_num in 1..=LINES_PER_FILE {
            let line = if match_positions.contains(&line_num) {
                // Baris dengan keyword
                format!("[CRITICAL] {} detected in file {:04} at scan line {}.\n", keyword, i, line_num)
            } else {
                // Baris noise, rotasi dari NOISE_LINES
                let noise_idx = (i * LINES_PER_FILE + line_num) % NOISE_LINES.len();
                format!("{}\n", NOISE_LINES[noise_idx])
            };
            file.write_all(line.as_bytes())
                .expect("Gagal menulis ke file");
        }
    }

    println!("[GEN] Selesai! {} file berhasil dibuat di '{}'", num_files, dir);
    println!("[GEN] Setiap file berisi {} matches → Total: {} matches",
        MATCHES_PER_FILE, num_files * MATCHES_PER_FILE);
}

/// Generate ketiga folder dataset sekaligus (1102, 11102, 111102 files)
pub fn generate_all_datasets(keyword: &str) {
    println!("╔══════════════════════════════════════════════════════╗");
    println!("║       GENERATE 3 VARIASI DATASET MINI-RIPGREP        ║");
    println!("╚══════════════════════════════════════════════════════╝");
    for (dir, count) in DATASET_CONFIGS {
        println!("\n▶ Generating '{}' ({} files)...", dir, count);
        generate_dataset(dir, keyword, *count);
    }
    println!("\n✓ Ketiga folder dataset (1102, 11102, 111102) siap digunakan!");
}
