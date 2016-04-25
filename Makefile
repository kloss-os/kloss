# FLAGSc
arch ?= x86_64
target ?= $(arch)-unknown-linux-gnu

# PATHS
kernel := build/kernel-$(arch).bin
iso := build/os-$(arch).iso

linker_script := src/arch/$(arch)/linker.ld
grub_cfg := src/arch/$(arch)/grub.cfg

assembly_source_files := $(wildcard src/arch/$(arch)/*.asm)
assembly_object_files := $(patsubst src/arch/$(arch)/%.asm, \
	build/arch/$(arch)/%.o, $(assembly_source_files))

rust_os := target/$(target)/debug/libbanjos.a


###################################################################
# OS Detection in Makefile, taken from stackoverflow:
# http://stackoverflow.com/questions/714100/os-detecting-makefile
#
# Modded to work with target systems
###################################################################
ifeq ($(OS),Windows_NT)
    # NOT SUPPORTED
else
    # Get OS
    UNAME_S := $(shell uname -s)
    # Get architecture
    UNAME_P := $(shell uname -p)

    # Act upon OS and Architecture
    # ifeq ($(UNAME_S),Linux)
    #     ifeq ($(UNAME_P),x86_64)
    #         CFLAGS += -D LINUX_64
    #     endif
    #     ifeq ($(UNAME_P),sparc)
    #         CFLAGS += -D LINUX_SPARC
    #     endif
    #     ifneq ($(filter %86,$(UNAME_P)),)
    #         CFLAGS += -D LINUX_32
    #     endif
    # endif
    # ifeq ($(UNAME_S), SunOS)
    #     ifeq ($(UNAME_P),sparc)
    #         CFLAGS += -D SPARC
    #     endif
    #     ifneq ($(filter %86,$(UNAME_P)),)
    #         CFLAGS += -D SOLARIS_32
    #     endif
    # endif
    # ifeq ($(UNAME_S),Darwin) # OS X
    #     CFLAGS += -D LINUX_64
    # endif
endif

# Text formatting for Linux & OSX
TEXT_RED     := $$(tput setaf 1)
TEXT_GREEN   := $$(tput setaf 2)
TEXT_YELLOW  := $$(tput setaf 3)
TEXT_BLUE    := $$(tput setaf 4)
TEXT_BOLD    := $$(tput bold)
TEXT_RESET   := $$(tput sgr0)


.PHONY: all clean run iso test debug gdb

all: $(kernel)

clean:
	rm -rf build
	rm -rf target

run: $(iso)
	qemu-system-x86_64 -cdrom $(iso) -s

iso: $(iso)

$(iso): $(kernel) $(grub_cfg)
	mkdir -p build/isofiles/boot/grub
	cp $(kernel) build/isofiles/boot/kernel.bin
	cp $(grub_cfg) build/isofiles/boot/grub
	grub-mkrescue -o $(iso) build/isofiles
	rm -r build/isofiles

$(kernel): cargo $(rust_os) $(assembly_object_files) $(linker_script)
	ld -n --gc-sections -T $(linker_script) -o $(kernel) $(assembly_object_files) $(rust_os)

cargo:
	@cargo rustc --target $(target) -- -Z no-landing-pads

# compile assembly files
build/arch/$(arch)/%.o: src/arch/$(arch)/%.asm
	mkdir -p $(shell dirname $@)
	nasm -felf64 $< -o $@


# TESTS & DEBUG
debug: $(iso)
	@qemu-system-x86_64 -cdrom $(iso) -s -S -monitor stdio

gdb:
	@rust-os-gdb/bin/rust-gdb $(kernel) -ex "target remote :1234"

test: cargo_test sys_test 

cargo_test:
	@echo "$(TEXT_RED)Not implemented$(TEXT_RESET)"
#	@echo "$(TEXT_GREEN)■ ■ ■ ■ ■ ■$(TEXT_RED) ■$(TEXT_GREEN) ■$(TEXT_RESET)"

sys_test:
	@echo "$(TEXT_RED)Not implemented$(TEXT_RESET)"


# generate internal documentation
dev_doc:
	cargo rustdoc -- --no-defaults --passes strip-hidden --passes collapse-docs --passes unindent-comments --passes strip-priv-imports
