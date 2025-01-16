# osc8er

[![Build Status](https://github.com/azam/osc8er/actions/workflows/build.yml/badge.svg)](https://github.com/azam/osc8er/actions/workflows/build.yml)
[![Crate](https://img.shields.io/crates/v/osc8er.svg)](https://crates.io/crates/osc8er)
[![Docs](https://docs.rs/osc8er/badge.svg)](https://docs.rs/osc8er)

CLI tool to convert file path or URL to terminal hyperlinks on terminal emulators that support OSC 8 hyperlinks.

## [License](LICENSE)

MIT Licence
Copyright (c) 2025 Azamshul Azizy

## Usage

Via pipes

```sh
find . -type f | osc8er
```

Via arguments

```sh
osc8er -a README.md
```
