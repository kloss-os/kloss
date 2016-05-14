# BanjOS

OSPP (1DT096) 2016 - Grupp 01

Projektarbete på kursen Operativsystem och processorienterad
programmering (1DT096) våren 2016, Uppsala universitet.



## Kompilera

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

### VIKTIGT!
Om du redan har en installation av rust via  `multirust override nightly`
kan du behöva installera om denna för att `make custom_target` skall fungera.
Detta kan göras genom att ta bort den lokala `~/.multirust`-mappen och
köra multirust override igen:

```
rm -rf ~/.multirust && multirust override nightly
```

## Testa
![Pooh Assembly]
(meta/images/pooh_assembly.png)

## Starta systemet


## Struktur

Projektet består av följande kataloger.

### doc

Dokumentation, projektrapporter och andra viktiga dokument.
Generera publik dokumentation med `cargo doc`.
Ovan inklusive moduler genereras med `make dev_doc`.

### meta

- Presentation av gruppens medlemmar.
- Gruppkontrakt.
- Projektdagböcker.
- Reflektioner på gruppens arbete.
