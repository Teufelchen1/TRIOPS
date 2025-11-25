//! This is TRIOPS entry point, where `main()` is located.
//! The scope of this file is:
//!  - The argument parsing and handling
//!  - The interactions with the filesystem
//!  - Setup and run the emulator
#![allow(clippy::upper_case_acronyms)]
#![allow(clippy::cast_possible_wrap)]
#![allow(clippy::cast_possible_truncation)]
#![allow(clippy::cast_sign_loss)]
mod app;
mod cli;
mod cpu;
mod events;
mod hifive1b;
mod instructions;
mod periph;
mod utils;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = cli::Config::parse()?;

    if config.headless {
        app::headless::headless(&config);
    } else {
        app::tui::tui(&config);
    }
    Ok(())
}
