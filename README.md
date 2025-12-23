<picture><img alt="The TRIOPS logo. Showing a hand-drawn triops, brown colored with black lining (or white lining in dark mode)." src=".github/rustacean-flat-happy.svg" height="50em" align="right"></picture>

[![Clippy & co](https://github.com/Teufelchen1/rv/actions/workflows/rust.yml/badge.svg)](https://github.com/Teufelchen1/rv/actions/workflows/rust.yml) 

---

# TRIOPS
A RISC-V emulator written in Rust. 🦀

<picture>
  <source media="(prefers-color-scheme: dark)" srcset=".github/triops_logo_dark.svg">
  <source media="(prefers-color-scheme: light)" srcset=".github/triops_logo_light.svg">
  <img alt="The TRIOPS logo. Showing a hand-drawn triops, brown colored with black lining (or white lining in dark mode)." src=".github/triops_logo_light.svg" width="50%" align="right">
</picture>

Triops is a genus of small c**rust**aceans. They have three eyes 👀, live up to 90 days and their eggs can stay dormant for years.

### Features

* RV32IMAC ISA - Implementing Multiplication, Compressed Instructions and Atomics extension.
* Loads ELF and BIN files.
* Comes with an easy to use and pretty looking TUI - which is powered by [Ratatui](https://github.com/ratatui/ratatui).
* Single step or autostep through the executable.
* A minimal, simple and bare metal C project is in `test_app/` included. Build it, run it in the emulator, tinker with it and repeat!
* Interact with the running executable via the two UARTs - emulating the peripheral of the [Hifive1b](https://www.sifive.com/boards/hifive1-rev-b).
* Can map both UARTs to any unixsocket of your choice!
* Can also run without the TUI, attaching the UART0 directly to stdio!
* Run [RIOT](https://github.com/RIOT-OS/RIOT) in TRIOPS using the included example app in `RIOT_app/`! RIOT, "the friendly Operating System for IoT", is a good example application to test for the correct behavior of TRIOPS. As an open-source microcontroller operating system, RIOT supports a huge amount of CPU architectures and development boards. Among those is the RISC-V based [Hifive1b](https://www.sifive.com/boards/hifive1-rev-b), which TRIOPS aims for.

### Limitations

There is currently no PLIC/CLIC but there is a limited interrupt support for the most essential stuff. Control and status register (csr) are without effect.
The hardware emulation of the [Hifive1b](https://www.sifive.com/boards/hifive1-rev-b) is not hardware accurate and not intended to be so.

## Requierments
On the Rust side, handled by cargo: `clap`, `anyhow`, `ratatui`, `crossterm`, `elf`.

For the `test_app`, which is in C: `riscv64-unknown-elf-gcc` and `riscv64-unknown-elf-objcopy`.

For the RIOT example please refer to the RIOT documentation and the [Getting Started](https://guide.riot-os.org/getting-started/installing/) guide.

## Usage
#### Ready the emulator:
1. Clone the repository and `cd` into it.
2. Build it: `cargo build`
3. Test it: `cargo run -- --help`

```txt
Usage: triops [OPTIONS] <FILE>

Arguments:
  <FILE>
          Path to the file that should be executed in the emulator

Options:
      --headless
          If set, no TUI is started.
          
          TRIOPS will run as fast as your machine allows.
          The UART will be mapped to stdio unless a unix socket is specified, see `uart-socket`.

      --uart0 <UART0>
          If set, connects UART0 TX/RX to the specified unix socket.
          If not set, the UART0 will be mapped to stdio.

      --uart1 <UART1>
          If set, connects UART1 TX/RX to the specified unix socket.
          If not set, the UART1 will not be accessible.

      --testing
          If set, the emulation result will be checked.
          
          TRIOPS will probe the registers according to the riscv-software-src/riscv-tests.
          Their contents determine the return value. The checks are done after the emulation completed.
          Mainly used for CI.

      --bin
          If set, the provided file is treated as pure binary
          
          When used, the entry address and base address can also be set.

      --entryaddress <ENTRYADDRESS>
          The entry address, where execution is started / PC is set to.
          
          Can be in hex or decimal.
          
          [default: 0x20000000]

      --baseaddress <BASEADDRESS>
          The base address, where the bin file is loaded to. Must be in RAM or ROM.
          
          Can be in hex or decimal.
          
          [default: 0x20000000]

  -h, --help
          Print help (see a summary with '-h')
```

#### Ready a RISC-V elf binary:
1. Enter the test application: `cd test_app/`
2. Build it: `./build.sh`
3. Results in `test.elf` and `test.bin`

#### Run the emulator:
1. `cd ..` back into the root directory.
2. `cargo run -- test_app/test.elf`

<video src='https://github.com/user-attachments/assets/0df8c42a-7468-4328-a70f-c0e969232ef2' width="100%"/></video>



## Goal

The purpose of this emulator is to teach me Rust and having fun with RISC-V. Recently it also became an aid in my job as a RIOT maintainer. Turns out, having a custom emulator is really useful when tracing bugs in an embedded operating system.

## Thanks

* [Einhornwolle](https://github.com/einhornwolle) for drawing this awesome logo.
* [EdJoPaTo](https://github.com/edjopato) for so much Rust feedback.
* [Kosmas12](https://github.com/kosmas12) for implementing the Multiplication extension.
* [Chrysn](https://github.com/chrysn) for responding to every single Rust-cry that I tooted on Mastodon.
