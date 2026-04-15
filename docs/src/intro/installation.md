# Installation

Rustywoof is distributed as a single standalone binary. Choose the installation method that best fits your operating system and environment.

## Option 1: Quick Install _`(macOS / Linux)`_

The fastest way to get started on Unix-like systems is via our installation script. This will download the latest pre-compiled binary and place it in your path.

```bash
curl --proto '=https' --tlsv1.2 -LsSf [https://github.com/ianramy/rustywoof/releases/latest/download/installer.sh](https://github.com/ianramy/rustywoof/releases/latest/download/installer.sh) | sh
```

## Option 2: Via _`cargo`_ (Rust Developers)

If you already have a Rust toolchain installed, you can compile Rustywoof from source or install the pre-compiled binaries directly via Cargo.

```bash

# Compile from source
cargo install rustywoof

# OR install pre-compiled binaries via cargo-binstall (faster)
cargo binstall rustywoof
```

## Option 3: Windows (_`PowerShell`_)

For Windows users, we provide a native PowerShell installation script. Open your PowerShell terminal and run:

```powershell
irm [https://github.com/ianramy/rustywoof/releases/latest/download/installer.ps1](https://github.com/ianramy/rustywoof/releases/latest/download/installer.ps1) | iex
```

Verifying the Installation
Once installed, verify that the woof CLI is available in your environment by checking the version:

```bash
woof --version
```

To see a full list of available commands and global flags, run the help command:

```bash
woof help
```

> [!TIP]
> Ready to secure your repository? Head over to the [CLI Reference](../../cli/) to learn how to run your first woof patrol sweep.
