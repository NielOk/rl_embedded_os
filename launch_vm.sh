DISK_IMG="rl-test-mac.img"
CLOUD_IMG="ubuntu-20.04-server-cloudimg-amd64.img"
SEED_ISO="seed-mac.iso"
SHARE_DIR="$HOME/RLTest"

# Copy rl_file_agent to shared directory
cp -r ./rl_file_agent ~/RLTest/

qemu-system-x86_64 \
  -m 2048 \
  -smp 2 \
  -hda "$DISK_IMG" \
  -cdrom "$SEED_ISO" \
  -net nic -net user,hostfwd=tcp::2222-:22 \
  -fsdev local,id=fsdev0,path="$SHARE_DIR",security_model=none \
  -device virtio-9p-pci,fsdev=fsdev0,mount_tag=hostshare \
  -nographic