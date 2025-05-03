# Mount debugfs if not already mounted
sudo mount -t debugfs none /sys/kernel/debug

# Enable BPF syscall support (usually default in Ubuntu 20+)
sudo ls /sys/kernel/debug/tracing/available_filter_functions | grep openat

sudo apt update
sudo apt install -y pkg-config libssl-dev build-essential clang llvm libelf-dev libbpf-dev linux-headers-$(uname -r)

# Start the user simulator
pip install -r requirements.txt
python3 simulate_user.py

# Install Rust
sudo apt remove rustc cargo
curl https://sh.rustup.rs -sSf | sh
source $HOME/.cargo/env

# Install cargo-generate (optional)
cargo install cargo-generate

cd ebpf
cargo build --release --target bpfel-unknown-none
cd ../user
cargo build --release
sudo ./target/release/user