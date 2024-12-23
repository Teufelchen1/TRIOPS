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
* Interact with the running executable via an UART - emulating the peripheral of the [Hifive1b](https://www.sifive.com/boards/hifive1-rev-b).
* Can also run without the TUI, attaching the UART directly to stdio.

### Limitations

There is currently no PLIC/CLIC and no interrupts. Control and status register (csr) are without effect.

## Requierments
On the Rust side, handled by cargo: `clap`, `anyhow`, `ratatui`, `crossterm`, `elf`.

For the `test_app`, which is in C: `riscv64-elf-gcc` and `riscv64-elf-objcopy`.

## Usage
#### Ready the emulator:
1. Clone the repository and `cd` into it.
2. Build it: `cargo build`
3. Test it: `cargo run -- --help`

#### Ready a RISC-V elf binary:
1. Enter the test application: `cd test_app/`
2. Build it: `./build.sh`
3. Results in `test.elf` and `test.bin`

#### Run the emulator:
1. `cd ..` back into the root directory.
2. `cargo run -- test_app/test.elf`

<video src='https://github.com/user-attachments/assets/0df8c42a-7468-4328-a70f-c0e969232ef2' width="100%"/></video>



## Goal

The purpose of this emulator is to teach me Rust and further deepen my love to RISC-V.

## Thanks

* [Einhornwolle](https://github.com/einhornwolle) for drawing this awesome logo.
* [EdJoPaTo](https://github.com/edjopato) for so much Rust feedback.
* [Kosmas12](https://github.com/kosmas12) for implementing the Multiplication extension.
* [Chrysn](https://github.com/chrysn) for responding to every single Rust-cry, I tooted on Mastodon.

## Bonus: Running RIOT OS within TRIOPS

[RIOT](https://github.com/RIOT-OS/RIOT), "the friendly Operating System for IoT", is a good example application to test for the correct behavior of TRIOPS. As an open-source microcontroller operating system, RIOT supports a huge amount of CPU architectures and development boards. Among those is the RISC-V based [Hifive1b](https://www.sifive.com/boards/hifive1-rev-b).

1. Clone RIOT into a folder of your choice: `git clone git@github.com:RIOT-OS/RIOT.git`
2. Go into `cd RIOT/examples/hello-world`
3. Compile for the Hifive1b: `BOARD=hifive1b make all`
4. After successfull compilation, RIOT will tell you where the resulting ELF file is stored, e.g. `/tmp/RIOT/examples/hello-world/bin/hifive1b/hello-world.elf`
5. Run TRIOPS with that file as input. For best results, disable the TUI to increase the execution speed:
`cargo run -- --headless /tmp/RIOT/examples/hello-world/bin/hifive1b/hello-world.elf`
6. It should look like this:
```
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.06s
     Running `target/debug/rv --headless /tmp/RIOT/examples/hello-world/bin/hifive1b/hello-world.elf`
main(): This is RIOT! (Version: 2025.01-devel-335-ga8b47)
Hello World!
You are running RIOT on a(n) hifive1b board.
This board features a(n) fe310 CPU.
```

Please note that the hello-world performs no further actions and the execution will halt. Additionally, whenever RIOT has nothing todo, it emits the `WFI` (wait for interrupt) instruction. This instruction is currently not supported by TRIOPS - unlike other unsupported instructions, which `panic!()` TRIOPS, `WFI` just hangs. This is a temporal fix to reduce the pain when playing around with RIOT.