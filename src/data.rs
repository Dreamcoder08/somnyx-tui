// data.rs — Recoleccion de datos del sistema y workspace SOMNYX
use std::fs;
use std::path::Path;
use std::process::Command;

#[derive(Default, Clone)]
pub struct SystemStats {
    pub cpu_percent: f32,
    pub ram_used_gb: f32,
    pub ram_total_gb: f32,
    pub disk_used_gb: f32,
    pub disk_total_gb: f32,
    pub uptime: String,
}

#[derive(Default, Clone)]
pub struct WorkspaceStats {
    pub workspace_size: String,
    pub archive_size:   String,
    pub vault_size:     String,
    pub notes_size:     String,
    pub media_size:     String,
    pub inbox_size:     String,
    pub inbox_count:    usize,
    pub inbox_old:      usize,  // archivos > 7 dias
    pub journal_today:  bool,
    pub timer_clean:    String,
    pub timer_alert:    String,
}

// ── Sistema ───────────────────────────────────────────────────────────────────

pub fn get_system_stats() -> SystemStats {
    let cpu = read_cpu_percent();
    let (ram_used, ram_total) = read_memory_gb();
    let (disk_used, disk_total) = read_disk_gb();
    SystemStats {
        cpu_percent:    cpu,
        ram_used_gb:    ram_used,
        ram_total_gb:   ram_total,
        disk_used_gb:   disk_used,
        disk_total_gb:  disk_total,
        uptime:         read_uptime(),
    }
}

fn read_cpu_percent() -> f32 {
    fn stat() -> Option<(u64, u64)> {
        let txt = fs::read_to_string("/proc/stat").ok()?;
        let line = txt.lines().next()?;
        let nums: Vec<u64> = line.split_whitespace()
            .skip(1).filter_map(|s| s.parse().ok()).collect();
        let idle  = nums.get(3).copied().unwrap_or(0)
                  + nums.get(4).copied().unwrap_or(0);
        let total: u64 = nums.iter().sum();
        Some((idle, total))
    }
    let s1 = stat().unwrap_or((0, 1));
    std::thread::sleep(std::time::Duration::from_millis(150));
    let s2 = stat().unwrap_or((0, 1));
    let di = s2.0.saturating_sub(s1.0);
    let dt = s2.1.saturating_sub(s1.1);
    if dt == 0 { return 0.0; }
    ((1.0 - di as f32 / dt as f32) * 100.0).clamp(0.0, 100.0)
}

fn read_memory_gb() -> (f32, f32) {
    let txt = fs::read_to_string("/proc/meminfo").unwrap_or_default();
    let mut total = 0u64;
    let mut avail = 0u64;
    for line in txt.lines() {
        if line.starts_with("MemTotal:")     { total = parse_kb(line); }
        if line.starts_with("MemAvailable:") { avail = parse_kb(line); }
    }
    let used = total.saturating_sub(avail);
    let gb = 1024.0 * 1024.0;
    (used as f32 / gb, total as f32 / gb)
}

fn parse_kb(line: &str) -> u64 {
    line.split_whitespace().nth(1).and_then(|s| s.parse().ok()).unwrap_or(0)
}

fn read_disk_gb() -> (f32, f32) {
    let out = Command::new("df").args(["-BG", "/"]).output().ok();
    if let Some(o) = out {
        let s = String::from_utf8_lossy(&o.stdout);
        if let Some(line) = s.lines().nth(1) {
            let p: Vec<&str> = line.split_whitespace().collect();
            let total = p.get(1).and_then(|s| s.trim_end_matches('G').parse::<f32>().ok()).unwrap_or(0.0);
            let used  = p.get(2).and_then(|s| s.trim_end_matches('G').parse::<f32>().ok()).unwrap_or(0.0);
            return (used, total);
        }
    }
    (0.0, 0.0)
}

fn read_uptime() -> String {
    let txt = fs::read_to_string("/proc/uptime").unwrap_or_default();
    let secs = txt.split_whitespace().next()
        .and_then(|s| s.parse::<f64>().ok()).unwrap_or(0.0) as u64;
    let d = secs / 86400;
    let h = (secs % 86400) / 3600;
    let m = (secs % 3600) / 60;
    if d > 0 { format!("{}d {}h {}m", d, h, m) } else { format!("{}h {}m", h, m) }
}

// ── Workspace ─────────────────────────────────────────────────────────────────

pub fn get_workspace_stats() -> WorkspaceStats {
    let home = std::env::var("HOME").unwrap_or_else(|_| "/home/dreamcoder08".into());
    let ws    = std::env::var("SOMNYX_WORKSPACE").unwrap_or_else(|_| format!("{}/somnyx",  home));
    let arc   = std::env::var("SOMNYX_ARCHIVE")  .unwrap_or_else(|_| format!("{}/archive", home));
    let vlt   = std::env::var("SOMNYX_VAULT")    .unwrap_or_else(|_| format!("{}/vault",   home));
    let nts   = std::env::var("SOMNYX_NOTES")    .unwrap_or_else(|_| format!("{}/notes",   home));
    let mda   = std::env::var("SOMNYX_MEDIA")    .unwrap_or_else(|_| format!("{}/media",   home));
    let inb   = std::env::var("SOMNYX_INBOX")    .unwrap_or_else(|_| format!("{}/inbox",   home));

    let today = chrono::Local::now().format("%Y-%m-%d").to_string();
    let journal_path = format!("{}/journal/{}.md", nts, today);

    WorkspaceStats {
        workspace_size: dir_size(&ws),
        archive_size:   dir_size(&arc),
        vault_size:     dir_size(&vlt),
        notes_size:     dir_size(&nts),
        media_size:     dir_size(&mda),
        inbox_size:     dir_size(&inb),
        inbox_count:    count_files(&inb, 2),
        inbox_old:      count_old_files(&inb),
        journal_today:  Path::new(&journal_path).exists(),
        timer_clean:    timer_next("somnyx-clean"),
        timer_alert:    timer_next("somnyx-inbox-alert"),
    }
}

fn dir_size(path: &str) -> String {
    Command::new("du").args(["-sh", "--apparent-size", path]).output().ok()
        .and_then(|o| {
            let s = String::from_utf8_lossy(&o.stdout).to_string();
            s.split_whitespace().next().map(|s| s.to_string())
        })
        .unwrap_or_else(|| "?".into())
}

fn count_files(path: &str, depth: u32) -> usize {
    Command::new("fd")
        .args([".", path, "--max-depth", &depth.to_string(), "--type", "f"])
        .output()
        .map(|o| String::from_utf8_lossy(&o.stdout).lines()
            .filter(|l| !l.is_empty()).count())
        .unwrap_or(0)
}

fn count_old_files(path: &str) -> usize {
    Command::new("fd")
        .args([".", path, "--max-depth", "2", "--type", "f", "--older", "7d"])
        .output()
        .map(|o| String::from_utf8_lossy(&o.stdout).lines()
            .filter(|l| !l.is_empty()).count())
        .unwrap_or(0)
}

fn timer_next(name: &str) -> String {
    let out = Command::new("systemctl")
        .args(["--user", "list-timers", "--all", "--no-legend"])
        .output().ok();
    if let Some(o) = out {
        for line in String::from_utf8_lossy(&o.stdout).lines() {
            if line.contains(name) {
                let parts: Vec<&str> = line.split_whitespace().collect();
                // Format: NEXT LEFT LAST PASSED UNIT ACTIVATES
                // "Left" column (index 3) gives relative time
                return parts.get(3).unwrap_or(&"?").to_string();
            }
        }
    }
    "?".into()
}
