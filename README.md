# Kloss

Kloss is a minimal OS kernel for the x86_64 architecturre written in
Rust for a project as part of an undergraduate OS course (Operating
Systems and Process-Oriented Prrogramming, 1DT096) at Uppsala university
spring 2016 that was way too little about operating systems and way too
much about filling in various blanks.

The name means "brick" in Swedish, and is a reference to Lego bricks and
our intention of making this into a modular, build-your-own-kernel kit.

Oh, and also, none of us knew Rust starting this, but all of us have had
our minds destroyed by C, so expect terrible, _terrible_ code.


## What works
- Hardware interrupts and keyboard (via APIC!)
- Software interrupts (kind of)
- Timers (again, kind of)
- Some ACPI
- VGA console
- Virtual memory (though we do nothing interesting with it)

## What's in the future
- Processes
- Actually having an architecture
- Not terrible code
- Mostly just splitting everything into modules

## Prerequisites

You need:
- QEMU for x86_64
- [multirust](https://github.com/brson/multirust) (unfortunately not currently working with Rustup)
- NASM
- grub and grub-mkrescue
- xorriso

These (except Multirust) can be installed on Ubuntu (LTS) by running:
`sudo apt-get install quemu xorriso grub-mkrescue nasm`.

To get started, you need to:

1. Set up the latest  `multirust override nightly`.
2. Installera en patchad version av rust: `make custom_target`.

This will cross-compile and install a patched version of the Rust core
libraries against the nofp patch from thepowersgang.

### If you already have multirust installed
If you already have multirust installed and already have overrides for
nightly builds set up, you may have to purge your multirust directory
and start over again before running `make custom_target`:

```
rm -rf ~/.multirust && multirust override nightly
```

## Building and running

`make run` will compile a bootable ISO file and launch it in Qemu.

## Testing
`cargo test` or `make test` will run unit tests.

## Generating documentation

`cargo doc` will generate documentation for the kernel itself (read:
what would have been its public interface if it were a module).

Documentation for all modules can be generrated by  `make dev_doc`

In both cases, the documentaiton will be placed in `target/doc/banjos`.

## Licensing and credits

The entire code base is licensed under the MIT license. Large swaths of
it was borrowed (actually, typed in by hand) from [Philip Oppermann's
excellent tutorial](http://os.phil-opp.com/).

Other projects of note (in no particular order):

- Julia Evans' [Puddle](https://github.com/jvns/puddle) and [corresponding series of blog posts](http://jvns.ca/blog/2014/03/12/the-rust-os-story/) 
- [thepowersgang's Bare-Bones Kernel](https://github.com/thepowersgang/rust-barebones-kernel) (thanks for the nofp patch!)
- [Redox OS](https://github.com/redox-os/redox) (whose target json file we used as a template)
- Eric Kidd's blog series [Bare Metal Rust: Building kernels in Rust](http://www.randomhacks.net/bare-metal-rust/)
