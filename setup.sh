#!/bin/bash

set -e

# Ensure dependencies
brew install qemu wget coreutils xorriso || true

DISK_IMG="rl-test-mac.img"
CLOUD_IMG="ubuntu-20.04-server-cloudimg-amd64.img"
SEED_ISO="seed-mac.iso"
SHARE_DIR="$HOME/RLTest"

read -p "Enter username (e.g. nielok): " USER
read -s -p "Enter password: " PASS
echo ""

mkdir -p "$SHARE_DIR"

# 1. Download Ubuntu cloud image
if [ ! -f "$CLOUD_IMG" ]; then
  wget https://cloud-images.ubuntu.com/focal/current/focal-server-cloudimg-amd64.img -O "$CLOUD_IMG"
fi

# 2. Create disk image
if [ ! -f "$DISK_IMG" ]; then
  qemu-img create -f qcow2 -b "$CLOUD_IMG" -F qcow2 "$DISK_IMG" 10G
fi

# 3. Write user-data and meta-data
cat > user-data <<EOF
#cloud-config
users:
  - name: $USER
    groups: sudo
    shell: /bin/bash
    sudo: ALL=(ALL) NOPASSWD:ALL
    lock_passwd: false
    plain_text_passwd: "$PASS"
packages:
  - python3
  - python3-pip
  - rustc
  - cargo
  - inotify-tools
  - git
  - vim
runcmd:
  - mkdir -p /mnt/hostshare
  - mount -t 9p -o trans=virtio hostshare /mnt/hostshare
EOF

echo "instance-id: $(uuidgen)" > meta-data
echo "local-hostname: rlvm" >> meta-data

# 4. Build seed ISO with xorriso
mkdir -p seed-iso-tmp
cp user-data meta-data seed-iso-tmp/

xorriso -as mkisofs \
  -output "$SEED_ISO" \
  -volid cidata \
  -joliet -rock seed-iso-tmp

rm -rf seed-iso-tmp