ARCH ?= aarch64
LOG ?=
STATS ?= off
PORT ?= 30022

# do not support debug mode
MODE := debug

export MODE
export LOG
export ARCH
export STATS

OBJCOPY ?= rust-objcopy --binary-architecture=$(ARCH)

build_path := target/$(ARCH)/$(MODE)
target_elf := $(build_path)/rvmarm
target_bin := $(build_path)/rvmarm.bin

features :=

ifeq ($(STATS), on)
  features += --features stats
endif

build_args := --features "$(features)" --target $(ARCH).json -Z build-std=core,alloc -Z build-std-features=compiler-builtins-mem

ifeq ($(MODE), release)
  build_args += --release
endif

# .PHONY: qemu-aarch64
# qemu-aarch64:
# 	cargo clean
# 	cargo build $(build_args)

.PHONY: all
all: $(target_bin)

.PHONY: elf
elf:
	cargo build $(build_args)

$(target_bin): elf
	$(OBJCOPY) $(target_elf) --strip-all -O binary $@

.PHONY: scp
scp:
	scp -P $(PORT) -r $(target_bin) ubuntu@localhost:/home/ubuntu

.PHONY: jailhouse
jailhouse:
	scp -P $(PORT) -r ./dev/jailhouse ubuntu@localhost:/home/ubuntu

.PHONY: patch
patch:
	scp -P $(PORT) ./scripts/guest/rvmarm.patch ubuntu@localhost:/home/ubuntu

.PHONY: rvmbin
rvmbin:
	scp -P $(PORT) rvmarm.bin ubuntu@localhost:/home/ubuntu