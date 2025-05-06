use aya::maps::perf::PerfEventArray;
use aya::{Bpf, include_bytes_aligned};
use std::{collections::HashMap, str, time::{Duration, Instant}, process::Command};

#[repr(C)]
pub struct ActivityEvent {
    pub pid: u32,
    pub comm: [u8; 16],
    pub activity: u8,
}

#[derive(Default)]
struct Features {
    reads: u32,
    connects: u32,
    execs: u32,
    last_seen: Instant,
}

struct Agent {
    q: HashMap<String, f64>,
    features: HashMap<String, Features>,
    alpha: f64,
    gamma: f64,
}

impl Agent {
    fn new(alpha: f64, gamma: f64) -> Self {
        Self { q: HashMap::new(), features: HashMap::new(), alpha, gamma }
    }

    fn update(&mut self, comm: &str, activity: u8, pid: u32) {
        let f = self.features.entry(comm.to_string()).or_default();
        match activity {
            0 => f.execs += 1,
            1 => f.reads += 1,
            2 => f.connects += 1,
            3 => f.execs += 1, // used for CPU tracking
            _ => {}
        }
        f.last_seen = Instant::now();

        let q = self.q.entry(comm.to_string()).or_insert(0.0);
        *q *= self.gamma;
        if f.reads > 5 || f.execs > 10 || f.connects > 3 {
            *q += self.alpha * (1.0 - *q);
        }

        if *q > 0.5 {
            println!("↑ Boost {} (Q={:.2})", comm, *q);
            let _ = Command::new("renice").arg("-n").arg("-5").arg("-p").arg(format!("{}", pid)).output();
        } else {
            println!("→ No boost for {} (Q={:.2})", comm, *q);
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut bpf = Bpf::load(include_bytes_aligned!("../target/bpfel-unknown-none/release/ebpf"))?;
    let mut perf_array = PerfEventArray::try_from(bpf.map_mut("EVENTS")?)?;
    perf_array.open()?;

    let mut agent = Agent::new(0.3, 0.98);
    println!("RL Agent running...");

    loop {
        perf_array.poll(Duration::from_secs(1), |data| {
            let event = unsafe { &*(data.as_ptr() as *const ActivityEvent) };
            let comm = str::from_utf8(&event.comm).unwrap_or("").trim_end_matches('\0');
            agent.update(comm, event.activity, event.pid);
        })?;
    }
}