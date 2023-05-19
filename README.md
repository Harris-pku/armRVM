# RVM1.5 arm移植

使用rust写的armv8 hypervisor，Porting from 
https://github.com/rcore-os/RVM1.5

## 0. qemu启动arm64虚拟机

```shell
sudo apt-get update
sudo apt-get install qemu
sudo apt-get install qemu-system-arm
```

```shell
qemu-system-aarch64 \
	-m 1G -cpu cortex-a57 \
	-smp 16 \
	-machine virt,gic-version=3,virtualization=on -nographic \
	-pflash flash0.img -pflash flash1.img \
	-drive if=none,file=ubuntu-18.04-server-cloudimg-arm64.img,id=hd0 \
	-drive file=user-data.img,format=raw,id=cloud \
	-device virtio-blk-device,drive=hd0 \
	-net user,id=net,hostfwd=tcp::30022-:22 -net nic \
	-serial mon:stdio
```

## 1. 编译运行jailhouse

### 1.0 安装依赖包

```shell
sudo sed -i "s/http:\/\/archive.ubuntu.com/http:\/\/mirrors.tuna.tsinghua.edu.cn/g" /etc/apt/sources.list
sudo apt-get update
sudo apt-get install build-essential python3-mako
```

### 1.1 编译jailhouse

```shell
git clone https://github.com/siemens/jailhouse.git
cd jailhouse
# 删掉 ./configs/arm64/dts/ 暂时没用
rm -rf ./configs/arm64/dts/
sudo make ARCH=arm64
sudo make ARCH=arm64 install
```

### 1.2 jailhouse enable

```shell
sudo insmod ./driver/jailhouse.ko
sudo chown $(whoami) /dev/jailhouse
sudo ./tools/jailhouse enable ./configs/arm64/qemu-arm64.cell
```

运行成功：

```
Initializing Jailhouse hypervisor v0.12 on CPU 6
Code location: 0x0000ffffc0200800
Page pool usage after early setup: mem 87/992, remap 0/131072
Initializing processors:
 CPU 6... OK
 CPU 13... OK
 CPU 4... OK
 CPU 15... OK
 CPU 8... OK
 CPU 11... OK
 CPU 3... OK
 CPU 0... OK
 CPU 9... OK
 CPU 7... OK
 CPU 12... OK
 CPU 5... OK
 CPU 1... OK
 CPU 10... OK
 CPU 2... OK
 CPU 14... OK
Initializing unit: irqchip
Initializing unit: ARM SMMU v3
Initializing unit: ARM SMMU
Initializing unit: PVU IOMMU
Initializing unit: PCI
Adding virtual PCI device 00:00.0 to cell "qemu-arm64"
Adding virtual PCI device 00:01.0 to cell "qemu-arm64"
Page pool usage after late setup: mem 144/992, remap 528/131072
Activating hypervisor
```

### 1.3 jailhouse disable

```shell
sudo ./tools/jailhouse disable
sudo rmmod jailhouse
```

## 2 编译运行armRVM

```
# in host
make
scp -P 30022 -r rvmarm.bin ubuntu@localhost:/home/ubuntu
scp -P 30022 -r rvmarm.patch ubuntu@localhost:/home/ubuntu

# in guest
sudo mkdir -p /lib/firmware
sudo ln -sf ~/rvmarm.bin /lib/firmware

# 给jailhouse打补丁
patch -f -p1 < ../rvmarm.patch
```

打完patch后的运行结果：

```
patching file Kbuild
patching file configs/Makefile
patching file driver/main.c
patching file gen-config.sh
patching file hypervisor/arch/x86/include/asm/jailhouse_header.h
patching file hypervisor/include/jailhouse/header.h
patching file include/jailhouse/cell-config.h
patching file pyjailhouse/sysfs_parser.py
patching file tools/jailhouse-config-create
patching file tools/root-cell-config.c.tmpl
```

继续执行

```
sudo make ARCH=arm64
sudo update-grub
sudo reboot
```

## 3.调试armRVM

