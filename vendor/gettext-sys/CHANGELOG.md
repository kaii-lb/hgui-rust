# Changelog

## 0.21.3 - 2022-03-16

### Changed
- `wbindtextdomain` is now a Rust function rather than a C symbol. The symbol is
    now named `libintl_wbindtextdomain`, for better compatibility with MinGW.
    This change only affects Windows (Marin)

### Fixed
- Only check for build dependencies when actually building the library (Ignacio
    Casal Quinteiro)



## 0.21.2 - 2021-07-21

### Fixed
- Build failure on some systems which put libraries into "lib64" directory (#73)
    (Alexander Batischev)



## 0.21.1 - 2021-07-16

### Changed
- Dependency on `tempdir` is replaced by dependency on `temp-dir`, which is way
    more lightweight (Amrit Brar)



## 0.21.0 - 2021-03-03

### Added
- A note regarding GNU gettext's LGPL license (Alexander Batischev)
- Checks for build tools required by GNU gettext (Dean Leggo)
- Bindings for `wbindtextdomain` (only available on Windows) (Alexander
    Batischev)
- Build-time dependency on `tempfile` (Alexander Batischev)

### Changed
- Bump bundled GNU gettext to 0.21 (Alexander Batischev)

### Fixed
- Build failure when a path contains spaces (Alexander Batischev)



## 0.19.9 - 2019-07-26

### Added
- Support for Windows+GNU (François Laignel)
- Support for musl libc (Rasmus Thomsen)
- `gettext-system` feature which asks the crate to use gettext that's built into
    libc (if available) (François Laignel)
- Use xz to compress the bundled GNU gettext tarball, to save space (Daniel
    García Moreno)



## 0.19.8 - 2018-05-23

Initial release (Konstantin V. Salikhov, Faizaan).
