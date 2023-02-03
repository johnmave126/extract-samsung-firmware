//! Courtesy of https://blog.quindorian.org/2021/05/firmware-update-samsung-ssd-in-linux.html/

use std::{
    borrow::Cow,
    fs::{self, File},
    io::{self, BufReader},
    path::PathBuf,
};
#[cfg(unix)]
use std::{os::unix::fs::PermissionsExt, process::Command};

use anyhow::{anyhow, ensure, Context};
use clap::{Parser, ValueHint};
use flate2::bufread::GzDecoder;
use iso9660::{DirectoryEntry, ISO9660};
use path_slash::CowExt;

static LONG_ABOUT: &str = "\
Extract Samsung SSD firmware from the iso file

The Samsung SSD firmware updator has many problems:
1. It does not work very well with newer AMD chipsets.
2. USB keyboards do not work.

This utility extracts the firmware file from the updator so that user can run it directly on a Linux machine.\
";

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = LONG_ABOUT)]
/// Extract Samsung SSD firmware from the iso file
struct Cli {
    /// Firmware iso file location
    #[arg(short, long, value_name = "FILE", value_hint = ValueHint::FilePath)]
    file: PathBuf,

    /// Target directory to extract to
    #[arg(
        short,
        long,
        value_name = "DIR",
        value_hint = ValueHint::DirPath,
        default_value_os_t = PathBuf::from(".")
    )]
    output_dir: PathBuf,

    #[cfg(target_os = "linux")]
    /// DANGEROURS: Execute the extracted firmware updator
    ///
    /// Users should verify the integrity of the iso file before executing the updator.
    #[arg(short, long)]
    execute: bool,
}

// Masks for entries in a cpio archive
const DIR_MASK: u32 = 0o0040000;
const FILE_MASK: u32 = 0o0100000;

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    // Open file in buffered reading mode
    let iso = File::open(cli.file).context("cannot open specified firmware file")?;
    let iso = BufReader::new(iso);
    let iso = ISO9660::new(iso).context("cannot open firmware file as .iso")?;

    // Find `/initrd`, decompress it, and open as an cpio archive
    let initrd = iso
        .open("/initrd")
        .context("cannot open `/initrd` in the firmware file")?
        .and_then(|entry| match entry {
            DirectoryEntry::Directory(_) => None,
            DirectoryEntry::File(f) => Some(f),
        })
        .ok_or(anyhow!("cannot find `/initrd` in the firmware file"))?
        .read();
    let initrd = BufReader::new(initrd);
    let initrd = GzDecoder::new(initrd);
    let mut initrd = cpio_archive::reader(initrd)
        .context("cannot open `/initrd` as a compressed cpio archive")?;

    // Scan through cpio archive
    let mut has_extracted = false;
    while let Some(result) = initrd.next() {
        match result {
            Ok(header) if header.name().starts_with("root/fumagician") => {
                has_extracted = true;

                let path = header
                    .name()
                    .strip_prefix("root/")
                    .expect("All entries here should start with `root/fumagician`");
                let path = cli.output_dir.join(Cow::from_slash(path));

                // Only write directory or file
                if header.mode() & DIR_MASK != 0 {
                    fs::create_dir_all(&path)
                        .with_context(|| format!("cannot create directory `{}`", path.display()))?;
                    println!("create direcotry `{}`", path.display());
                } else if header.mode() & FILE_MASK != 0 {
                    let mut f = File::create(&path)
                        .with_context(|| format!("cannot create file `{}`", path.display()))?;
                    io::copy(&mut initrd, &mut f)
                        .with_context(|| format!("cannot write to file `{}`", path.display()))?;

                    #[cfg(unix)]
                    {
                        let metadata = f.metadata().with_context(|| {
                            format!("cannot modify metadata of file `{}`", path.display())
                        })?;
                        let mut permissions = metadata.permissions();
                        permissions.set_mode(header.mode() & 0o777);
                    }
                    println!("write file `{}`", path.display());
                } else {
                    eprintln!(
                        "WARN: skipping `{}` because it is neither a directory nor a file",
                        path.display()
                    );
                }
            }
            Err(e) => {
                eprintln!("WARN: error reading `/initrd`: {e}");
            }
            _ => (),
        }
    }

    ensure!(has_extracted, "no file or directory is extracted");

    #[cfg(target_os = "linux")]
    if cli.execute {
        let work_dir = fs::canonicalize(cli.output_dir.join("fumagician"))
            .context("cannot locate path of extracted files")?;
        let status = Command::new(work_dir.join("fumagician"))
            .current_dir(work_dir)
            .status()
            .context("failed to execute firmware updator")?;

        if status.success() {
            eprintln!("failed to execute firmware updator: {status}");
        }
    }
    Ok(())
}
