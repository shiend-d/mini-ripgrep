// ============================================================
// main.rs - Entry Point: Mini-Ripgrep UTS KPT
// ============================================================
// High-Throughput Parallel Text Search Engine ("Mini-Ripgrep")
// Berbasis Thread-Local Buffering dan Dynamic File Chunking
//
// NIM: 247006111102  |  Parameter: 102
// Mata Kuliah: Komputasi Paralel dan Terdistribusi
// Dosen: Rohmat Gunawan, M.T.

mod dataset_gen;
mod multithreading;
mod multithreading_bug;
mod multiprocessing;
mod sequential;

use std::env;
use std::io::{self, Write};
use std::path::Path;

// ── Konstanta Global ─────────────────────────────────────────
const NAME: &str = "Yusuf Abdurrahman";
const NIM: &str = "247006111102";
const DEFAULT_DIR: &str = "./dataset_11102";
const CHUNK_SIZE: usize = 102; // sesuai 3 digit akhir NIM: 102

// ── Helper: Cetak Banner Header ──────────────────────────────
fn print_header() {
    println!("======================================================");
    println!("  PARALLEL TEXT SEARCH ENGINE (MINI-RIPGREP)");
    println!("  By: {} (NIM: {})", NAME, NIM);
    println!("======================================================");
}

// ── Helper: Cetak Info Konfigurasi ───────────────────────────
fn print_info(mode_label: &str, workers: usize, keyword: &str, dir: &str) {
    println!("[INFO] Target Directory : {}", dir);
    println!("[INFO] Search Keyword   : \"{}\"", keyword);
    println!("[INFO] Execution Mode   : {}", mode_label);
    println!("[INFO] Active Workers   : {} Workers", workers);
    println!("[INFO] Chunk Size       : {} Files per Task", CHUNK_SIZE);
    println!("------------------------------------------------------");
}

// ── Helper: Cetak Hasil ──────────────────────────────────────
fn print_result(total_files: usize, total_matches: usize, elapsed: f64, baseline_secs: Option<f64>) {
    let throughput = if elapsed > 0.0 {
        total_files as f64 / elapsed
    } else {
        0.0
    };
    println!("SEARCH COMPLETED SUCCESSFULLY");
    println!("------------------------------------------------------");
    println!("Total Files Processed : {} Files", total_files);
    println!("Total Matches Found   : {} Lines", total_matches);
    println!("Total Execution Time  : {:.4} Seconds", elapsed);
    println!("Throughput            : {:.2} Files/Sec", throughput);

    if let Some(baseline) = baseline_secs {
        let speedup = if elapsed > 0.0 { baseline / elapsed } else { 1.0 };
        println!("Speedup vs Sequential : {:.2}x", speedup);
    } else {
        println!("Speedup vs Sequential : 1.00x (Baseline)");
    }
    println!("======================================================");
}

// Struct untuk menyimpan data benchmark per run
#[derive(Clone, Debug)]
#[allow(dead_code)]
struct BenchRow {
    dataset_label: &'static str,
    kategori: &'static str,
    konfigurasi: String,
    workers: usize,
    files: usize,
    matches: usize,
    waktu_secs: f64,
    speedup: f64,
    throughput: f64,
}

// ── Mode: Sequential ─────────────────────────────────────────
fn run_sequential(keyword: &str, dir: &str) -> (usize, usize, f64) {
    print_header();
    print_info("Sequential (Baseline)", 1, keyword, dir);
    let (files, matches, elapsed) = sequential::run(dir, keyword);
    print_result(files, matches, elapsed, None);
    (files, matches, elapsed)
}

// ── Mode: Multithreading (Bug) ───────────────────────────────
fn run_threaded_bug(workers: usize, baseline: Option<f64>, keyword: &str, dir: &str) -> (usize, usize, f64) {
    print_header();
    print_info("Multithreading (Code with Bug - Mutex Contention)", workers, keyword, dir);
    let (files, matches, elapsed) = multithreading_bug::run(dir, keyword, workers);
    print_result(files, matches, elapsed, baseline);
    (files, matches, elapsed)
}

// ── Mode: Multithreading (Final) ─────────────────────────────
fn run_threaded(workers: usize, baseline: Option<f64>, keyword: &str, dir: &str) -> (usize, usize, f64) {
    print_header();
    let label = format!("Multithreading (Final Code - Thread-Local), {} Threads", workers);
    print_info(&label, workers, keyword, dir);
    let (files, matches, elapsed) = multithreading::run(dir, keyword, workers);
    print_result(files, matches, elapsed, baseline);
    (files, matches, elapsed)
}

// ── Mode: Multiprocessing (Final) ────────────────────────────
fn run_multiprocess(workers: usize, baseline: Option<f64>, keyword: &str, dir: &str) -> (usize, usize, f64) {
    print_header();
    let label = format!("Multiprocessing (Final Code - Child Processes), {} Processes", workers);
    print_info(&label, workers, keyword, dir);
    let (files, matches, elapsed) = multiprocessing::run(dir, keyword, workers);
    print_result(files, matches, elapsed, baseline);
    (files, matches, elapsed)
}

// ── Mode: Benchmark Lengkap (3 Variasi Dataset: 1102, 11102, 111102) ────────────────
fn run_benchmark_all_datasets(keyword: &str) {
    println!("\n╔══════════════════════════════════════════════════════════════════════╗");
    println!("║       BENCHMARK EKSPERIMEN LENGKAP MINI-RIPGREP (UTS KPT 2026)       ║");
    println!("║   Variasi 1: Ukuran Dataset (1.102, 11.102, 111.102 Files)           ║");
    println!("║   Variasi 2: Thread Worker (2, 4, 8, 16 Threads)                     ║");
    println!("║   Variasi 3: Process Worker (2, 4, 8, 16 Processes)                  ║");
    println!("║   Variasi 4: Multithreading BUG (Mutex Lock Contention 2, 4, 8, 16)  ║");
    println!("╚══════════════════════════════════════════════════════════════════════╝\n");

    // Pastikan ketiga dataset sudah ada
    for (dir, count) in dataset_gen::DATASET_CONFIGS {
        if !Path::new(dir).exists() || sequential::collect_files(dir).len() < *count {
            println!("[BENCHMARK PREPARE] Membuat dataset '{}' ({} files)...", dir, count);
            dataset_gen::generate_dataset(dir, keyword, *count);
        }
    }

    let worker_variations = [2usize, 4, 8, 16];
    let mut all_records: Vec<BenchRow> = Vec::new();

    let datasets = [
        ("Small (1.102 Files)", "./dataset_1102"),
        ("Medium (11.102 Files)", "./dataset_11102"),
        ("Large (111.102 Files)", "./dataset_111102"),
    ];

    for (dataset_label, dir) in datasets {
        println!("\n━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
        println!("  PENGUJIAN DATASET: {} [{}]", dataset_label, dir);
        println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");

        // 1. Sequential Baseline
        println!("▶ [1] Running Sequential Baseline...");
        let (files, matches, baseline) = run_sequential(keyword, dir);
        all_records.push(BenchRow {
            dataset_label,
            kategori: "Sequential",
            konfigurasi: "Baseline (1 Core)".to_string(),
            workers: 1,
            files,
            matches,
            waktu_secs: baseline,
            speedup: 1.0,
            throughput: files as f64 / baseline.max(0.0001),
        });
        println!();

        // 2. Multithreading Bug (Variasi 2, 4, 8, 16 threads)
        for &w in &worker_variations {
            println!("▶ [2] Running Multithreading BUG ({} Threads)...", w);
            let (f, m, elapsed) = run_threaded_bug(w, Some(baseline), keyword, dir);
            all_records.push(BenchRow {
                dataset_label,
                kategori: "Thread (Bug)",
                konfigurasi: format!("{} Threads (Mutex Lock)", w),
                workers: w,
                files: f,
                matches: m,
                waktu_secs: elapsed,
                speedup: baseline / elapsed.max(0.0001),
                throughput: f as f64 / elapsed.max(0.0001),
            });
            println!();
        }

        // 3. Multithreading Final (Variasi 2, 4, 8, 16 threads)
        for &w in &worker_variations {
            println!("▶ [3] Running Multithreading Final ({} Threads)...", w);
            let (f, m, elapsed) = run_threaded(w, Some(baseline), keyword, dir);
            all_records.push(BenchRow {
                dataset_label,
                kategori: "Multithreading",
                konfigurasi: format!("{} Threads (Thread-Local)", w),
                workers: w,
                files: f,
                matches: m,
                waktu_secs: elapsed,
                speedup: baseline / elapsed.max(0.0001),
                throughput: f as f64 / elapsed.max(0.0001),
            });
            println!();
        }

        // 4. Multiprocessing Final (Variasi 2, 4, 8, 16 processes)
        for &w in &worker_variations {
            println!("▶ [4] Running Multiprocessing Final ({} Processes)...", w);
            let (f, m, elapsed) = run_multiprocess(w, Some(baseline), keyword, dir);
            all_records.push(BenchRow {
                dataset_label,
                kategori: "Multiprocessing",
                konfigurasi: format!("{} Child Processes", w),
                workers: w,
                files: f,
                matches: m,
                waktu_secs: elapsed,
                speedup: baseline / elapsed.max(0.0001),
                throughput: f as f64 / elapsed.max(0.0001),
            });
            println!();
        }
    }

    // ── Cetak Rekap Tabel Hasil Percobaan UTS (Dipisah per Ukuran Dataset) ──
    println!("\n╔═══════════════════════════════════════════════════════════════════════════════════════════════════════════════╗");
    println!("║                           TABEL REKAPITULASI HASIL PERCOBAAN PER UKURAN DATASET (UTS KPT)                     ║");
    println!("║                           Nama: {} | NIM: {} (Parameter: {})                     ║", NAME, NIM, CHUNK_SIZE);
    println!("╚═══════════════════════════════════════════════════════════════════════════════════════════════════════════════╝\n");

    for (dataset_label, _) in datasets {
        let subset: Vec<&BenchRow> = all_records.iter().filter(|r| r.dataset_label == dataset_label).collect();
        println!("┌───────────────────────────────────────────────────────────────────────────────────────────────────────────────┐");
        println!("│ DATASET: {:<101} │", dataset_label);
        println!("├────┬─────────────────┬─────────────────────────────┬───────────────┬─────────────────────┬────────────────────┤");
        println!("│ No │ Pendekatan      │ Konfigurasi Worker          │ Waktu (Detik) │ Throughput (File/s) │ Speedup vs Base    │");
        println!("├────┼─────────────────┼─────────────────────────────┼───────────────┼─────────────────────┼────────────────────┤");

        for (idx, r) in subset.iter().enumerate() {
            println!(
                "│ {:<2} │ {:<15} │ {:<27} │ {:<13.4} │ {:<19.2} │ {:<18.2}x │",
                idx + 1,
                r.kategori,
                r.konfigurasi,
                r.waktu_secs,
                r.throughput,
                r.speedup
            );
        }
        println!("└────┴─────────────────┴─────────────────────────────┴───────────────┴─────────────────────┴────────────────────┘\n");
    }

    println!("╔═══════════════════════════════════════════════════════════════════════════════════════════════════════════════╗");
    println!("║                               ANALISIS DAN KESIMPULAN PERCOBAAN (BAGIAN D UTS KPT)                            ║");
    println!("╚═══════════════════════════════════════════════════════════════════════════════════════════════════════════════╝\n");

    println!("1. PERBEDAAN PERFORMA ANTAR KONFIGURASI (Speedup & Skalabilitas):");
    println!("   a. Sequential (Baseline 1 Core):");
    println!("      - Memproses file secara serial baris demi baris pada 1 thread tunggal.");
    println!("      - Menjadi acuan dasar (Speedup = 1.00x) dengan waktu eksekusi O(N) berbanding lurus jumlah data.");
    println!("   b. Multithreading Final (Thread-Local Buffering):");
    println!("      - Menunjukkan efisiensi dan throughput tertinggi pada dataset Medium dan Large (Speedup optimal di 4 Thread).");
    println!("      - Mengeliminasi lock contention pada loop utama dan hanya memicu 1 kali operasi atomik di akhir eksekusi.");
    println!("   c. Multithreading BUG (Mutex di Inner Loop):");
    println!("      - Mengalami fenomena Lock Contention (Thread Serialization) akibat .lock().unwrap() pada setiap penemuan kata.");
    println!("      - Pada dataset Small (1.102 file), efisiensi anjlok drastis (Speedup < 0.5x) karena frekuensi lock mendominasi waktu kerja CPU.");
    println!("      - Catatan Anomali: Pada dataset Medium/Large, speedup tetap naik namun tidak stabil dan kalah efisien dibanding Thread-Local.");
    println!("   d. Multiprocessing Final (Child Processes via IPC Pipe):");
    println!("      - Menawarkan isolasi memori mutlak, namun dibebani Initial Process Spawning Overhead.");
    println!("      - Pada dataset Small (1.102 file), spawn overhead membuat efisiensi rendah pada worker tinggi.");
    println!("      - Pada dataset Large, overhead teramortisasi sehingga menghasilkan throughput sangat kompetitif mendekati multithreading.\n");

    println!("2. FAKTOR YANG PALING MEMENGARUHI KECEPATAN (Analisis I/O, CPU, & Formula):");
    println!("   a. Disk I/O & OS Page Caching (Faktor Dominan):");
    println!("      - Operasi syscall read() mendominasi latensi. Penggunaan BufReader (8KB buffer) mereduksi I/O disk secara masif.");
    println!("      - Pemanfaatan OS Page Cache mempercepat pengujian berulang pada memori (warm cache read).");
    println!("   b. Model Komputasi & Formula Evaluasi:");
    println!("      - Formula Speedup    : S = T_sequential / T_parallel");
    println!("      - Formula Throughput : TP = Total_File / Waktu_Eksekusi (File/detik)");
    println!("      - Karakteristik sistem bergeser dari I/O-Bound (pada dataset kecil) menjadi CPU-Bound (pada dataset besar).\n");

    println!("3. KESIMPULAN UMUM, HUKUM AMDAHL, & EVALUASI HARDWARE (Intel Core i3 Gen 11):");
    println!("   - Hardware Uji: Intel Core i3 11th Gen (2 Physical Cores / 4 Logical Threads).");
    println!("   - Berdasarkan Hukum Amdahl, batas teoritis speedup pada hardware 2-Core fisik adalah mendekati ~2.00x.");
    println!("   - Titik jenuh (saturation point) optimal tercapai pada 4 Worker (sesuai jumlah logical threads).");
    println!("     Penambahan worker hingga 8-16 memicu thrashing dan context switching overhead masif tanpa kenaikan speedup berarti.");
    println!("   - REKOMENDASI ARSITEKTUR:");
    println!("     * Multithreading (Thread-Local) adalah pilihan terbaik untuk throughput komputasi lokal.");
    println!("     * Multiprocessing direkomendasikan jika sistem membutuhkan proteksi isolasi memori mutlak / fault-tolerance.");
    println!("=================================================================================================================\n");
}

// ── Interactive Wizard ───────────────────────────────────────
fn run_interactive() {
    print_header();
    println!("\n=== MODE INTERAKTIF MINI-RIPGREP ===");
    println!("Silakan masukkan parameter pencarian di bawah ini:\n");

    // 1. Input Keyword (Wajib)
    let keyword: String;
    loop {
        print!("▶ Masukkan Keyword Pencarian (WAJIB): ");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let trimmed = input.trim().to_string();
        if !trimmed.is_empty() {
            keyword = trimmed;
            break;
        }
        println!("  [PERINGATAN] Keyword tidak boleh kosong! Harap masukkan teks.");
    }

    // 2. Pilih Mode
    println!("\nPilih Mode Eksekusi:");
    println!("  [1] Multithreading (Final - Thread-Local Buffering)");
    println!("  [2] Multiprocessing (Final - Child Processes)");
    println!("  [3] Multithreading BUG (Mutex Contention Demo)");
    println!("  [4] Sequential (Baseline 1 Thread)");
    println!("  [5] Benchmark Lengkap 3 Dataset (1102, 11102, 111102 files + Semua Mode)");
    println!("  [6] Generate 3 Folder Dataset (1102, 11102, 111102 files)");
    print!("▶ Pilihan Anda [1-6] (default: 1): ");
    io::stdout().flush().unwrap();
    let mut mode_choice = String::new();
    io::stdin().read_line(&mut mode_choice).unwrap();
    let mode_choice = mode_choice.trim();

    if mode_choice == "5" {
        run_benchmark_all_datasets(&keyword);
        return;
    }

    if mode_choice == "6" {
        dataset_gen::generate_all_datasets(&keyword);
        return;
    }

    // 3. Input Target Folder
    print!("\n▶ Direktori Dataset (default: '{}'): ", DEFAULT_DIR);
    io::stdout().flush().unwrap();
    let mut dir_input = String::new();
    io::stdin().read_line(&mut dir_input).unwrap();
    let dir = if dir_input.trim().is_empty() {
        // Cek apakah DEFAULT_DIR ada, jika tidak cek dataset_text
        if Path::new(DEFAULT_DIR).exists() {
            DEFAULT_DIR.to_string()
        } else if Path::new("./dataset_text").exists() {
            "./dataset_text".to_string()
        } else {
            DEFAULT_DIR.to_string()
        }
    } else {
        dir_input.trim().to_string()
    };

    match mode_choice {
        "2" => {
            print!("▶ Jumlah Child Processes (default: 8): ");
            io::stdout().flush().unwrap();
            let mut w_input = String::new();
            io::stdin().read_line(&mut w_input).unwrap();
            let workers: usize = w_input.trim().parse().unwrap_or(8);

            println!("\n[INFO] Mengukur baseline sequential terlebih dahulu...");
            let (_, _, baseline) = sequential::run(&dir, &keyword);
            println!("[INFO] Baseline: {:.4}s\n", baseline);
            run_multiprocess(workers, Some(baseline), &keyword, &dir);
        }
        "3" => {
            print!("▶ Jumlah Threads Bug (default: 8): ");
            io::stdout().flush().unwrap();
            let mut w_input = String::new();
            io::stdin().read_line(&mut w_input).unwrap();
            let workers: usize = w_input.trim().parse().unwrap_or(8);

            println!("\n[INFO] Mengukur baseline sequential terlebih dahulu...");
            let (_, _, baseline) = sequential::run(&dir, &keyword);
            println!("[INFO] Baseline: {:.4}s\n", baseline);
            run_threaded_bug(workers, Some(baseline), &keyword, &dir);
        }
        "4" => {
            run_sequential(&keyword, &dir);
        }
        _ => {
            // Default: Multithreading Final
            print!("▶ Jumlah Threads Worker (default: 8): ");
            io::stdout().flush().unwrap();
            let mut w_input = String::new();
            io::stdin().read_line(&mut w_input).unwrap();
            let workers: usize = w_input.trim().parse().unwrap_or(8);

            println!("\n[INFO] Mengukur baseline sequential terlebih dahulu...");
            let (_, _, baseline) = sequential::run(&dir, &keyword);
            println!("[INFO] Baseline: {:.4}s\n", baseline);
            run_threaded(workers, Some(baseline), &keyword, &dir);
        }
    }
}

// ── Help Text ────────────────────────────────────────────────
fn print_help() {
    println!("PARALLEL TEXT SEARCH ENGINE (MINI-RIPGREP)");
    println!("By: {} | NIM: {} | Parameter: {}", NAME, NIM, CHUNK_SIZE);
    println!();
    println!("CATATAN PENTING:");
    println!("  Keyword pencarian WAJIB dimasukkan (tidak ada default keyword).");
    println!("  Jika dijalankan tanpa argumen, program akan membuka mode interaktif.");
    println!();
    println!("CARA PENGGUNAAN (CLI):");
    println!("  mini_ripgrep [COMMAND] <KEYWORD> [OPTIONS]");
    println!();
    println!("COMMANDS:");
    println!("  threaded <KEYWORD> [WORKERS]       Jalankan Multithreading FINAL (default 8 workers)");
    println!("  multiprocess <KEYWORD> [WORKERS]   Jalankan Multiprocessing FINAL (default 8 processes)");
    println!("  threaded-bug <KEYWORD> [WORKERS]   Jalankan Multithreading BUG dengan N workers");
    println!("  sequential <KEYWORD>               Jalankan mode Sequential (Baseline)");
    println!("  benchmark <KEYWORD>                Jalankan uji masal lengkap (3 dataset + variasi thread/process)");
    println!("  generate <KEYWORD> [NUM_FILES]     Generate dataset (jika tanpa NUM_FILES: buat 1102, 11102, 111102)");
    println!("  interactive                        Jalankan wizard tanya-jawab interaktif");
    println!("  help                               Tampilkan bantuan ini");
    println!();
    println!("OPTIONS:");
    println!("  -k, --keyword <TEKS>   Kata kunci yang dicari (WAJIB)");
    println!("  -w, --workers <N>      Jumlah worker threads / child processes");
    println!("  -n, --count <N>        Jumlah file saat generate");
    println!("  -d, --dir <PATH>       Folder target dataset (default: \"{}\")", DEFAULT_DIR);
    println!();
    println!("CONTOH:");
    println!("  mini_ripgrep generate ERROR_102            (Generate 3 dataset: 1102, 11102, 111102)");
    println!("  mini_ripgrep benchmark ERROR_102           (Benchmark semua dataset & mode)");
    println!("  mini_ripgrep threaded ERROR_102 8");
    println!("  mini_ripgrep multiprocess ERROR_102 4");
    println!("  mini_ripgrep sequential ERROR_102");
    println!("  mini_ripgrep                               (Masuk wizard interaktif)");
}

// ── Main ──────────────────────────────────────────────────────
fn main() {
    let all_args: Vec<String> = env::args().collect();
    let raw = &all_args[1..];

    // Tangkap flag internal multiprocessing --worker-mode
    if raw.first().map(|s| s.as_str()) == Some("--worker-mode") {
        let kw = raw.get(1).map(|s| s.as_str()).unwrap_or("");
        multiprocessing::worker_mode(kw);
        return;
    }

    // Jika tanpa argumen, jalankan wizard interaktif
    if raw.is_empty() {
        run_interactive();
        return;
    }

    // Cek apakah meminta help
    if raw.len() == 1 && (raw[0] == "help" || raw[0] == "--help" || raw[0] == "-h") {
        print_help();
        return;
    }

    if raw.len() == 1 && raw[0] == "interactive" {
        run_interactive();
        return;
    }

    // Parse flags dan positional arguments
    let mut keyword_opt: Option<String> = None;
    let mut dir = DEFAULT_DIR.to_string();
    let mut workers_opt: Option<usize> = None;
    let mut count_opt: Option<usize> = None;
    let mut mode_opt: Option<String> = None;
    let mut positional: Vec<String> = Vec::new();

    let mut i = 0;
    while i < raw.len() {
        match raw[i].as_str() {
            "-k" | "--keyword" => {
                i += 1;
                if let Some(val) = raw.get(i) {
                    keyword_opt = Some(val.clone());
                } else {
                    eprintln!("[ERROR] Flag --keyword membutuhkan nilai.");
                    std::process::exit(1);
                }
            }
            "-d" | "--dir" => {
                i += 1;
                if let Some(val) = raw.get(i) {
                    dir = val.clone();
                } else {
                    eprintln!("[ERROR] Flag --dir membutuhkan nilai.");
                    std::process::exit(1);
                }
            }
            "-w" | "--workers" => {
                i += 1;
                if let Some(val) = raw.get(i) {
                    if let Ok(w) = val.parse::<usize>() {
                        workers_opt = Some(w);
                    } else {
                        eprintln!("[ERROR] Flag --workers harus berupa angka bulat positif.");
                        std::process::exit(1);
                    }
                } else {
                    eprintln!("[ERROR] Flag --workers membutuhkan nilai.");
                    std::process::exit(1);
                }
            }
            "-n" | "--count" => {
                i += 1;
                if let Some(val) = raw.get(i) {
                    if let Ok(c) = val.parse::<usize>() {
                        count_opt = Some(c);
                    } else {
                        eprintln!("[ERROR] Flag --count harus berupa angka bulat positif.");
                        std::process::exit(1);
                    }
                } else {
                    eprintln!("[ERROR] Flag --count membutuhkan nilai.");
                    std::process::exit(1);
                }
            }
            "-m" | "--mode" => {
                i += 1;
                if let Some(val) = raw.get(i) {
                    mode_opt = Some(val.clone());
                } else {
                    eprintln!("[ERROR] Flag --mode membutuhkan nilai.");
                    std::process::exit(1);
                }
            }
            "-h" | "--help" => {
                print_help();
                return;
            }
            other => {
                positional.push(other.to_string());
            }
        }
        i += 1;
    }

    // Resolusi command & keyword dari positional args
    let mut command = mode_opt.unwrap_or_default();

    if !positional.is_empty() {
        let first = &positional[0];
        let recognized_modes = [
            "generate", "gen",
            "sequential", "seq",
            "threaded", "thread", "multithreading",
            "multiprocess", "process", "multiprocessing",
            "threaded-bug", "bug",
            "benchmark", "bench",
            "interactive", "wizard"
        ];

        if recognized_modes.contains(&first.as_str()) {
            if command.is_empty() {
                command = first.clone();
            }
            // Positional selanjutnya: keyword, worker/count
            if positional.len() > 1 && keyword_opt.is_none() {
                keyword_opt = Some(positional[1].clone());
            }
            if positional.len() > 2 {
                if let Ok(num) = positional[2].parse::<usize>() {
                    if command == "generate" || command == "gen" {
                        if count_opt.is_none() {
                            count_opt = Some(num);
                        }
                    } else if workers_opt.is_none() {
                        workers_opt = Some(num);
                    }
                }
            }
        } else {
            // Positional pertama bukan nama mode, melainkan keyword langsung
            if keyword_opt.is_none() {
                keyword_opt = Some(first.clone());
            }
            if positional.len() > 1 && workers_opt.is_none() {
                if let Ok(w) = positional[1].parse::<usize>() {
                    workers_opt = Some(w);
                }
            }
        }
    }

    if command.is_empty() {
        command = "threaded".to_string();
    }

    if command == "interactive" || command == "wizard" {
        run_interactive();
        return;
    }

    // Validasi Keyword: WAJIB DIISI!
    let keyword = match keyword_opt {
        Some(kw) if !kw.trim().is_empty() => kw,
        _ => {
            eprintln!("======================================================");
            eprintln!("[ERROR] Keyword pencarian WAJIB dimasukkan!");
            eprintln!("======================================================");
            eprintln!("Tidak ada default search keyword. Silakan tentukan keyword.");
            eprintln!();
            eprintln!("Contoh penggunaan:");
            eprintln!("  ./target/release/mini_ripgrep benchmark ERROR_102");
            eprintln!("  ./target/release/mini_ripgrep generate ERROR_102");
            eprintln!("  ./target/release/mini_ripgrep threaded ERROR_102 8");
            eprintln!("  ./target/release/mini_ripgrep  (Jalankan tanpa argumen untuk wizard interaktif)");
            eprintln!("======================================================");
            std::process::exit(1);
        }
    };

    let keyword = keyword.as_str();
    let workers = workers_opt.unwrap_or(8);

    match command.as_str() {
        "generate" | "gen" => {
            if let Some(c) = count_opt {
                dataset_gen::generate_dataset(&dir, keyword, c);
            } else {
                dataset_gen::generate_all_datasets(keyword);
            }
        }

        "sequential" | "seq" => {
            run_sequential(keyword, &dir);
        }

        "threaded-bug" | "bug" => {
            println!("[INFO] Mengukur baseline sequential terlebih dahulu...");
            let (_, _, baseline) = sequential::run(&dir, keyword);
            println!("[INFO] Baseline: {:.4}s\n", baseline);
            run_threaded_bug(workers, Some(baseline), keyword, &dir);
        }

        "threaded" | "thread" | "multithreading" => {
            println!("[INFO] Mengukur baseline sequential terlebih dahulu...");
            let (_, _, baseline) = sequential::run(&dir, keyword);
            println!("[INFO] Baseline: {:.4}s\n", baseline);
            run_threaded(workers, Some(baseline), keyword, &dir);
        }

        "multiprocess" | "process" | "multiprocessing" => {
            println!("[INFO] Mengukur baseline sequential terlebih dahulu...");
            let (_, _, baseline) = sequential::run(&dir, keyword);
            println!("[INFO] Baseline: {:.4}s\n", baseline);
            run_multiprocess(workers, Some(baseline), keyword, &dir);
        }

        "benchmark" | "bench" => {
            run_benchmark_all_datasets(keyword);
        }

        unknown => {
            eprintln!("[ERROR] Mode perintah tidak dikenal: '{}'", unknown);
            eprintln!("Gunakan './target/release/mini_ripgrep help' untuk melihat panduan.");
            std::process::exit(1);
        }
    }
}
