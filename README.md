# armRVM

使用rust写的armv8 hypervisor，Porting from 
https://github.com/rcore-os/RVM1.5

## 编译
```shell
make
```

## 启动qemu
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

## 加载jailhouse
```shell
sudo insmod ./driver/jailhouse.ko 
sudo chown $(whoami) /dev/jailhouse
sudo jailhouse enable ./configs/arm64/qemu-arm64.cell 
```
