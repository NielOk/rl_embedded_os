use aya::{include_bytes_aligned, Bpf};
use aya::maps::ringbuf::RingBuf;
use aya::programs::KProbe; // Needed for attaching kprobes
use rand::Rng;
use std::{
    collections::HashMap,
    fs,
    path::{Path, PathBuf},
    str,
    time::{Duration, Instant},
};

#[repr(C)]
pub struct FileOpenEvent {
    pub pid: u32,
    pub filename: [u8; 256],
}

struct RLAgent {
    q: HashMap<String, f64>,
    last_action: HashMap<String, (bool, Instant)>,
    original_paths: HashMap<String, String>,
    alpha: f64,
    decay: f64,
}

impl RLAgent {
    fn new(alpha: f64, decay: f64) -> Self {
        Self {
            q: HashMap::new(),
            last_action: HashMap::new(),
            original_paths: HashMap::new(),
            alpha,
            decay,
        }
    }

    fn softmax(&self, val: f64) -> f64 {
        let e = val.exp();
        e / (1.0 + e)
    }

    fn step(&mut self, filename: &str) -> bool {
        let q_val = *self.q.get(filename).unwrap_or(&0.0);
        let p = self.softmax(q_val);
        let r: f64 = rand::thread_rng().gen();
        let move_it = r < p;

        self.last_action.insert(filename.to_string(), (move_it, Instant::now()));
        move_it
    }

    fn reward(&mut self, filename: &str) {
        if let Some((was_moved, time)) = self.last_action.get(filename) {
            if *was_moved && time.elapsed().as_secs() < 30 {
                let q = self.q.entry(filename.to_string()).or_insert(0.0);
                *q += self.alpha * (1.0 - *q);
                println!("Rewarded {} → Q = {:.2}", filename, *q);
            }
        }
    }

    fn decay_all(&mut self) {
        for val in self.q.values_mut() {
            *val *= self.decay;
        }
    }

    fn promote(&mut self, filename: &str) {
        if filename.starts_with("/home/nielok/frequent/") {
            return;
        }

        let name = Path::new(filename)
            .file_name()
            .unwrap_or_default()
            .to_str()
            .unwrap_or("unknown.txt");

        let dest = format!("/home/nielok/frequent/{}", name);
        if fs::rename(filename, &dest).is_ok() {
            self.original_paths.insert(dest.clone(), filename.to_string());
            println!("Promoted to frequent/: {}", name);
        }
    }

    fn demote(&mut self, filename: &str) {
        if !filename.starts_with("/home/nielok/frequent/") {
            return;
        }

        if let Some(original) = self.original_paths.get(filename) {
            let parent = Path::new(original).parent();
            if let Some(p) = parent {
                let _ = fs::create_dir_all(p);
            }

            if fs::rename(filename, original).is_ok() {
                println!("Demoted back: {}", Path::new(filename).file_name().unwrap().to_str().unwrap_or("?"));
            }
        }
    }
}

fn resolve_path(filename: &str) -> Option<String> {
    let bases = ["/home/nielok/sim_user", "/home/nielok/frequent"];
    for base in &bases {
        let full = Path::new(base).join(filename);
        if full.exists() {
            return Some(full.to_string_lossy().to_string());
        }
    }
    None
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut bpf = Bpf::load(include_bytes_aligned!(
        "../target/bpfel-unknown-none/release/ebpf"
    ))?;

    // Attach trace_openat to __x64_sys_openat
    let program: &mut KProbe = bpf.program_mut("trace_openat").unwrap().try_into()?;
    program.load()?;
    program.attach("__x64_sys_openat", 0)?;

    let mut ringbuf = RingBuf::try_from(bpf.map_mut("EVENTS")?)?;
    ringbuf.open()?;

    let mut agent = RLAgent::new(0.5, 0.99);

    println!("RL Agent Running...");

    loop {
        agent.decay_all();

        ringbuf.poll(Duration::from_secs(1), |data| {
            let event = unsafe { &*(data.as_ptr() as *const FileOpenEvent) };
            let rel_path = str::from_utf8(&event.filename)
                .unwrap_or("")
                .trim_end_matches(char::from(0))
                .to_string();

            println!("Raw filename from BPF: '{}'", rel_path);

            if let Some(full_path) = resolve_path(&rel_path) {
                println!("Access: {}", full_path);

                agent.reward(&full_path);

                let q_val = *agent.q.get(&full_path).unwrap_or(&0.0);

                if q_val > 0.8 {
                    agent.promote(&full_path);
                } else if q_val < 0.2 {
                    agent.demote(&full_path);
                }
            }

            Ok(())
        })?;
    }
}