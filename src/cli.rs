use clap::Parser;

#[derive(Parser, Debug)]
struct Args {
    /// If set, no TUI is started.
    ///
    /// TRIOPS will run as fast as the CPU allows.
    /// The UART will be mapped to stdio.
    #[arg(long, default_value_t = false, verbatim_doc_comment)]
    headless: bool,

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
    pub testing: bool,
    pub bin: bool,
    pub entryaddress: usize,
    pub baseaddress: usize,
    pub file: Vec<u8>,
}

impl Config {
    pub fn parse() -> Self {
        let args = Args::parse();
        let path = args.file;
        let file = std::fs::read(&path)
            .unwrap_or_else(|_| panic!("Could not read file {}", path.display()));
        let entryaddress = usize_from_str(&args.entryaddress);
        let baseaddress = usize_from_str(&args.baseaddress);
        Self {
            headless: args.headless,
            testing: args.testing,
            bin: args.bin,
            entryaddress,
            baseaddress,
            file,
        }
    }
}

fn usize_from_str(text: &str) -> usize {
    if text.starts_with("0x") {
        usize::from_str_radix(text.trim_start_matches("0x"), 16).unwrap()
    } else {
        text.parse().unwrap()
    }
}
