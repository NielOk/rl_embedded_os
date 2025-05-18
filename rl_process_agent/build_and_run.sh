# Mount debugfs if not already mounted
sudo mount -t debugfs none /sys/kernel/debug

# Enable BPF syscall support (usually default in Ubuntu 20+)
sudo ls /sys/kernel/debug/tracing/available_filter_functions | grep openat

sudo apt update
sudo apt install -y pkg-config libssl-dev build-essential clang llvm libelf-dev libbpf-dev linux-headers-$(uname -r)
sudo apt install linux-tools-$(uname -r)

# Install Rust
sudo apt remove rustc cargo
curl https://sh.rustup.rs -sSf | sh
source $HOME/.cargo/env

# Install cargo-generate (optional)
cargo install cargo-generate

# Build the kernel-space eBPF program
cd ebpf
cargo build --release --target bpfel-unknown-none

# Build the user-space RL agent
cd ../user
cargo build --release

# Load all BPF programs
sudo bpftool prog loadall ../ebpf/target/bpfel-unknown-none/release/ebpf /sys/fs/bpf/rl_agent

# Attach to relevant kernel hooks
sudo bpftool prog attach name trace_exec    type kprobe attach_prog sched_process_exec
sudo bpftool prog attach name trace_read    type kprobe attach_prog __x64_sys_read
sudo bpftool prog attach name trace_connect type kprobe attach_prog __x64_sys_connect
sudo bpftool prog attach name trace_switch  type kprobe attach_prog sched_switch

sudo ./target/release/user

# Start the user simulator
cd ../
pip install -r requirements.txt
python3 simulate_processes.py