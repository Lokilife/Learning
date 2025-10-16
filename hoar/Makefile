ARCH=i686
CONFIGURATION=debug
TARGET = $(ARCH)-kernel
RUST_TARGET = target/$(TARGET)/$(CONFIGURATION)/libkernel.a

ASM_SOURCES = src/arch/$(ARCH)/boot.asm src/arch/$(ARCH)/interrupts.asm src/arch/$(ARCH)/gdt.asm
ASM_OBJECTS = $(ASM_SOURCES:.asm=.o)

KERNEL = target/$(TARGET)/$(CONFIGURATION)/kernel.bin
ISO = target/$(TARGET)/$(CONFIGURATION)/os.iso

CARGO_BUILD_ARGS := +nightly build -Z build-std=core --target $(TARGET).json
ifneq ($(CONFIGURATION),debug)
CARGO_BUILD_ARGS := $(CARGO_BUILD_ARGS) --$(CONFIGURATION)
endif

$(RUST_TARGET):
	cargo $(CARGO_BUILD_ARGS) 

%.o: %.asm
	nasm -f elf32 -o $@ $<

$(KERNEL): $(RUST_TARGET) $(ASM_OBJECTS)
	mkdir -p target/
	ld -m elf_i386 -n -T src/arch/$(ARCH)/linker-$(CONFIGURATION).ld -nostdlib `if [ "$(CONFIGURATION)" != "debug" ]; then echo "-s"; fi` -o $(KERNEL) $(ASM_OBJECTS) target/$(TARGET)/$(CONFIGURATION)/libkernel.a

$(ISO): $(KERNEL)
	mkdir -p target/isofiles/boot/grub
	cp $(KERNEL) target/isofiles/boot/kernel.bin

	mkdir -p target/isofiles/boot/grub
	cp $(KERNEL) target/isofiles/boot/
	cp src/arch/$(ARCH)/grub.cfg target/isofiles/boot/grub/grub.cfg

	grub-mkrescue -o $(ISO) target/isofiles

.PHONY: all run clean
all: $(ISO)

run: $(ISO)
	qemu-system-i386 -cdrom $(ISO)

clean:
	cargo clean
	rm -rf target

kernel: $(RUST_TARGET)
boot: $(BOOT)
link: $(KERNEL)
iso: $(ISO)

is-multiboot:
	if grub-file --is-x86-multiboot $(KERNEL); then echo multiboot confirmed; else echo the file is not multiboot; fi
