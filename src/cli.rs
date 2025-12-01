use std::os::unix::fs::FileTypeExt;

use anyhow::anyhow;
use anyhow::Context;
use clap::Parser;

#[derive(Parser, Debug)]
struct Args {
    /// If set, no TUI is started.
    ///
    /// TRIOPS will run as fast as your machine allows.
    /// The UART will be mapped to stdio unless a unix socket is specified, see `uart-socket`.
    #[arg(long, default_value_t = false, verbatim_doc_comment)]
    headless: bool,

    /// If set, connects UART0 TX/RX to the specified unix socket.
    /// If not set, the UART0 will be mapped to stdio.
    #[arg(long, verbatim_doc_comment)]
    uart0: Option<std::path::PathBuf>,

    /// If set, connects UART1 TX/RX to the specified unix socket.
    /// If not set, the UART1 will not be accessible.
    #[arg(long, verbatim_doc_comment)]
    uart1: Option<std::path::PathBuf>,

    /// If set, the emulation result will be checked.
    ///
    /// TRIOPS will probe the registers according to the riscv-software-src/riscv-tests.
    /// Their contents determine the return value. The checks are done after the emulation completed.
    /// Mainly used for CI.
    #[arg(long, default_value_t = false, verbatim_doc_comment)]
    testing: bool,

    /// If set, the provided file is treated as pure binary
    ///
    /// When used, the entry address and base address can also be set.
    #[arg(long, default_value_t = false, verbatim_doc_comment)]
    bin: bool,

    /// The entry address, where execution is started / PC is set to.
    ///
    /// Can be in hex or decimal.
    #[arg(long, default_value_t = String::from("0x20000000"), requires("bin"))]
    entryaddress: String,

    /// The base address, where the bin file is loaded to. Must be in RAM or ROM.
    ///
    /// Can be in hex or decimal.
    #[arg(long, default_value_t = String::from("0x20000000"), requires("bin"))]
    baseaddress: String,

    /// Path to the file that should be executed in the emulator
    file: std::path::PathBuf,
}

/// Little wrapper to do some conversions outside of main
/// Longterm goal is having a `Config` struct that can be used to save & replay the emulator
pub struct Config {
    pub headless: bool,
    pub uart0: Option<std::path::PathBuf>,
    pub uart1: Option<std::path::PathBuf>,
    pub testing: bool,
    pub bin: bool,
    pub entryaddress: usize,
    pub baseaddress: usize,
    pub file: Vec<u8>,
}

impl Config {
    pub fn parse() -> anyhow::Result<Self> {
        let args = Args::parse();
        let path = args.file;
        let file =
            std::fs::read(&path).context(format!("Could not read file {}", path.display()))?;

        clear_socket(args.uart0.as_ref())?;
        clear_socket(args.uart1.as_ref())?;

        let entryaddress = usize_from_str(&args.entryaddress);
        let baseaddress = usize_from_str(&args.baseaddress);
        Ok(Self {
            headless: args.headless,
            uart0: args.uart0,
            uart1: args.uart1,
            testing: args.testing,
            bin: args.bin,
            entryaddress,
            baseaddress,
            file,
        })
    }
}

fn clear_socket(uart_path: Option<&std::path::PathBuf>) -> anyhow::Result<()> {
    if let Some(ref socket_path) = uart_path {
        if socket_path.exists() {
            let attr = std::fs::metadata(socket_path).context(format!(
                "Unable to create unixsocket for the UART backend: {}",
                socket_path.display()
            ))?;
            if attr.file_type().is_socket() {
                let _ = std::fs::remove_file(socket_path);
            } else {
                return Err(anyhow!(std::io::ErrorKind::AlreadyExists)).context(format!(
                    "Unable to create unixsocket for the UART backend: {}",
                    socket_path.display()
                ));
            }
        }
    }
    Ok(())
}

fn usize_from_str(text: &str) -> usize {
    if text.starts_with("0x") {
        usize::from_str_radix(text.trim_start_matches("0x"), 16).unwrap()
    } else {
        text.parse().unwrap()
    }
}
