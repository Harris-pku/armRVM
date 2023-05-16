sudo ./tools/jailhouse disable
sudo rmmod jailhouse
sudo insmod ./driver/jailhouse.ko
sudo chown $(whoami) /dev/jailhouse
sudo ./tools/jailhouse enable ./configs/arm64/qemu-arm64.cell
