# Run this script inside the vm to mount the host directory

sudo modprobe 9p
sudo modprobe 9pnet
sudo modprobe 9pnet_virtio

sudo mkdir -p /mnt/hostshare
sudo mount -t 9p -o trans=virtio,version=9p2000.L hostshare /mnt/hostshare