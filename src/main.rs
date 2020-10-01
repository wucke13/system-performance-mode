//! A small helper to allow power mode control on certain Lenovo Laptops
#![deny(missing_docs)]
#![deny(unsafe_code)]

use std::fs::OpenOptions;
use std::io::prelude::*;
use std::io::SeekFrom;

use clap::Clap;

/// Sets the performance mode on an Lenovo IdeaPad 5 14are05
#[derive(Clap)]
#[clap(version, author, about)]
enum Command {
    /// Get the mode
    #[clap()]
    Get,

    /// Set the mode
    #[clap()]
    Set(Mode),
}

/// The different power modes
#[derive(Clap)]
enum Mode {
    #[clap()]
    BatterySaving,

    #[clap()]
    ExtremePerformance,

    #[clap()]
    IntelligentCooling,
}

fn main() -> std::io::Result<()> {
    let opts: Command = Command::parse();

    let mut acpi_call = OpenOptions::new()
        .read(true)
        .write(true)
        .open("/proc/acpi/call")?;

    match opts {
        Command::Get => {
            let mut stmd = String::default();
            let mut qtmd = String::default();
            acpi_call.write_all(br"\_SB.PCI0.LPC0.EC0.STMD")?;
            acpi_call.read_to_string(&mut stmd)?;
            acpi_call.seek(SeekFrom::Start(0))?;
            acpi_call.write_all(br"\_SB.PCI0.LPC0.EC0.QTMD")?;
            acpi_call.read_to_string(&mut qtmd)?;

            const X0: &str = "0x0\u{0}called\u{0}";
            const X1: &str = "0x1\u{0}called\u{0}";

            println!(
                "{}",
                match (stmd.as_str(), qtmd.as_str()) {
                    (X0, X0) => "Extreme Performance",
                    (X0, X1) => "Battery Saving",
                    (X1, X0) => "Intelligent Cooling",
                    _ => "Unknown",
                }
            );
        }
        Command::Set(Mode::BatterySaving) => {
            acpi_call.write_all(br"\_SB.PCI0.LPC0.EC0.VPC0.DYTC 0x0013B001")?;
        }
        Command::Set(Mode::ExtremePerformance) => {
            acpi_call.write_all(br"\_SB.PCI0.LPC0.EC0.VPC0.DYTC 0x0012B001")?;
        }
        Command::Set(Mode::IntelligentCooling) => {
            acpi_call.write_all(br"\_SB.PCI0.LPC0.EC0.VPC0.DYTC 0x000FB001")?;
        }
    }
    Ok(())
}
