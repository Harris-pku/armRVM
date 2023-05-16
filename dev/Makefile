QEMU ?= qemu-system-aarch64
PORT ?= 2333

qemu_args := \
	-m 1G -cpu cortex-a57 \
	-smp 16 \
	-machine virt,gic-version=3,virtualization=on -nographic \
	-pflash flash0.img -pflash flash1.img \
	-drive if=none,file=ubuntu-18.04-server-cloudimg-arm64.img,id=hd0 \
	-drive file=user-data.img,format=raw,id=cloud \
	-device virtio-blk-device,drive=hd0 \
	-net user,id=net,hostfwd=tcp::30022-:22 -net nic \
	-serial mon:stdio

.ONESHELL:
qemu:
	$(QEMU) $(qemu_args)

ssh:
	ssh -p $(PORT) ubuntu@localhost

