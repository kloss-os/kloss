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

Gör så här:
1. Installera senaste nightlyn av Rust m.h.a. multirust: `multirust override nightly`.
2. `make run` bygger en bootbar ISO med kärnan och kör den i QEMU

## Testa
![Pooh Assembly]
(blob:https%3A//drive.google.com/02dd3d6c-342e-44d3-9941-15fa341608d8)

## Starta systemet


## Struktur

Projektet består av följande kataloger.

### doc

Dokumentation, projektrapporter och andra viktiga dokument.

### meta

- Presentation av gruppens medlemmar.
- Gruppkontrakt.
- Projektdagböcker.
- Reflektioner på gruppens arbete.
