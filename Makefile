# FLAGS
arch ?= x86_64
target ?= $(arch)-unknown-none-gnu

# PATHS
kernel := build/kernel-$(arch).bin
iso := build/os-$(arch).iso

linker_script := src/arch/$(arch)/linker.ld
grub_cfg := src/arch/$(arch)/grub.cfg

assembly_source_files := $(wildcard src/arch/$(arch)/*.asm)
assembly_object_files := $(patsubst src/arch/$(arch)/%.asm, \
	build/arch/$(arch)/%.o, $(assembly_source_files))

rust_os := target/$(target)/debug/libbanjos.a

# CUSTOM TARGET
# important to order dependencies first to last. Eg.
# core first as alloc needs core, etc.
rustlibs := core alloc rustc_unicode collections

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


.PHONY: all clean run iso test debug gdb custom_target

all: $(kernel)

clean:
	rm -rf build
	rm -rf libcore rustc-nightly-src.tar.gz
	rm -rf target

run: $(iso)
	qemu-system-x86_64 -cdrom $(iso) -s -d int -no-reboot

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

	@cargo rustc --target $(target) -- -Z no-landing-pads -C no-redzone

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
	cargo test --features tests
	@echo ""


sys_test:
	@echo "$(TEXT_RED)System tests Not implemented$(TEXT_RESET)"
#	@echo "$(TEXT_GREEN)■ ■ ■ ■ ■ ■$(TEXT_RED) ■$(TEXT_GREEN) ■$(TEXT_RESET)
	@echo ""

# generate internal documentation
dev_doc:
	cargo rustdoc -- --no-defaults --passes strip-hidden --passes collapse-docs --passes unindent-comments --passes strip-priv-imports


# Build and install a patched custom target:
custom_target: $(patsubst %,lib%_install,$(rustlibs))
	rm -r $(patsubst %_install,./%,$^)

# Fetch latest nightly 
rustc-nightly-src.tar.gz:
	curl https://static.rust-lang.org/dist/rustc-nightly-src.tar.gz -o rustc-nightly-src.tar.gz

# Compile- and install to target folder
lib%_install: lib%/lib.rs
	mkdir -p build
	mkdir -p ~/.multirust/toolchains/nightly/lib/rustlib/$(target)/lib/
	rustc --target $(target) -Z no-landing-pads \
	    --cfg disable_float \
	    --out-dir ~/.multirust/toolchains/nightly/lib/rustlib/$(target)/lib \
	    $^

# Custom for libcore as to patch it
libcore/lib.rs: rustc-nightly-src.tar.gz libcore_nofp.patch
	tar -xmf rustc-nightly-src.tar.gz rustc-nightly/src/libcore --transform 's~^rustc-nightly/src/~~'
	patch -p0 < libcore_nofp.patch

# All other rust libraries
lib%/lib.rs: rustc-nightly-src.tar.gz libcore_nofp.patch
	tar -xmf rustc-nightly-src.tar.gz rustc-nightly/src/$(patsubst %/lib.rs,%,$@) --transform 's~^rustc-nightly/src/~~'


# # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # #
#                                                                     #
#  Since we compile our own rust core library, we only get the        #
#  core libraries we compile. Since we need the `alloc` library       #
#  (amongst others), we need to compile these as well.                #
#                                                                     #
#  This is because when compiling to our custom target, the           #
#  compiler does not look for libraries in the standard-target        #
#  if not found, but looks only in our custom target.                 #
#                                                                     #
#  What is needed, tho, is to look through the source of the          #
#  needed libraries and see wheter they utilize floating point        #
#  or not -- as well as whether it is patched or not -- before        #
#  adding them to the compile list.                                   #
#                                                                     #
#  The tar-command above extracts only `libcore` to the subfolder     #
#  named `rustc-nightly/src/libcore` as it is its name and path       #
#  within the tar-file. The `--transform 's~^rustc-nightly/src/~~'`   #
#  option in the tar-command is applied post-extraction and           #
#  simply moves the `libcore`-source folder to the current            #
#  directory (`./libcore`). At least that is what I grasp it          #
#  doing.                                                             #
#                                                                     #
# # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # 
