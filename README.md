# BanjOS

OSPP (1DT096) 2016 - Grupp 01

Projektarbete på kursen Operativsystem och processorienterad
programmering (1DT096) våren 2016, Uppsala universitet.

![Pooh Assembly]
(meta/images/pooh_assembly.png)

## Kompilera och köra

Du behöver:
- en x86_64-maskin
- QEMU för x86_64
- [multirust](https://github.com/brson/multirust)
- NASM
- grub och grub-mkrescue
- xorriso

På Ubuntu (LTS) motsvarar det följande: `sudo apt-get install quemu xorriso grub-mkrescue nasm`.

**Gör så här:**

1. Installera senaste nightlyn av Rust m.h.a. multirust: `multirust override nightly`.
2. Installera en patchad version av rust: `make custom_target`.
3. `make run` bygger en bootbar ISO med kärnan och kör den i QEMU

### VIKTIGT! Om du redan har Multirust installerat
Om du redan har en installation av rust via  `multirust override nightly`
kan du behöva installera om denna för att `make custom_target` skall fungera.
Detta kan göras genom att ta bort den lokala `~/.multirust`-mappen och
köra multirust override igen:

```
rm -rf ~/.multirust && multirust override nightly
```

## Testa
`cargo test` eller `make test` kommer att köra enhetstester.

## Generera dokumentation
Generera publik dokumentation med `cargo doc`. Ovan inklusive moduler genereras med `make dev_doc`. All dokumentation finns i mappen `target/doc/banjos`.

## Använda systemet
När kärnan startar placeras du i ett enklare kommandoskal. Se `help` för information om vilka kommandon som finns.

## Struktur

Projektet består av följande kataloger:

### src
Källkod. Moduler har egna mappar, men följande filer är också värda att notera:

- lib.rs -- kärnans startpunkt

### libs
- Bibliotek skrivna av projektet men utseparerade till egna moduler. Det långsiktiga målet är att flytta större delen av all kod hit.

### target
- Här hamnar vissa binär- och biblioteksfiler. Allt i mappen är artefakter.

#### target/doc
- Här hamnar dokumentation (när den har byggts).

### build
- Här hamnar slutgiltiga binärfiler och (mer viktigt) filen `os-x86_64.iso`, som är den startbara skivavbild som projektet genererar.

### tests
Hjälpkod för tester (men inga faktiska testfiler).

### meta
- Presentation av gruppens medlemmar.
- Gruppkontrakt.
- Projektdagböcker.
- Reflektioner på gruppens arbete.
