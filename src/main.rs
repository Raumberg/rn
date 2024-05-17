use std::fs;
use std::path::PathBuf;
use std::io;
use clap::Parser;
use fern::Dispatch;
use regex;
use log::{debug, error, info, trace, warn};
use chrono::prelude::*;
use chrono_tz::Tz;

/// PROGRAM (Rust compiled .rs) FOR GRIDSEARCHING PDF FILENAMES AND RENAME ACCORDINGLY TO A SPECIFIC PATTERN 
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Folder to search (ex: C:\Users\user\Documents\...)
    #[arg(short, long)]
    folder: PathBuf,

    /// Regex pattern for renaming (ex: "^a-zA-Z0-9_-")
    #[arg(long)]
    regex: Option<String>,

    /// Enable log writing
    #[arg(long, default_value_t=false)]
    log: bool,

    #[arg(long, default_value_t=String::from("pdf"))]
    ext: String,
}

fn logger(log: bool) -> Result<(), fern::InitError> {
    let mut dispatch: Dispatch = fern::Dispatch::new()
        .format(|out, message, record| {
            let timezone = Tz::Europe__Moscow;
            let now = Utc::now().with_timezone(&timezone);
            out.finish(format_args!(
                "[{:02}:{:02}:{:02} {} |{}|] {}",
                now.hour(), now.minute(), now.second(),
                record.level(),
                record.target(),
                message
            ))
        })
        .level(log::LevelFilter::Debug);

    if log {
        dispatch = dispatch.chain(fern::log_file("logs.log")?);
    }

    dispatch.chain(std::io::stdout())
        .apply()?;
    Ok(())
}

fn main() {
    let args: Args = Args::parse();
    debug!("CLI Arguments: {:?}", args);

    if let Err(e) = logger(args.log) {
        eprintln!("Failed to initialize logger: {}", e);
        return;
    }

    if !args.folder.exists() {
        error!("Folder {} does not exist", args.folder.display());
        eprintln!("Press enter to exit");
        let mut buffer = String::new();
        if let Err(e) = io::stdin().read_line(&mut buffer) {
            error!("Error reading from argument: {}", e);
        }
    }

    let entries = fs::read_dir(args.folder).expect("Failed to read directory");
    for (i, entry) in entries.enumerate() {
        let entry = match entry {
            Ok(entry) => entry,
            Err(e) => {
                error!("Failed to read entry: {}", e);
                continue;
            }
        };
        let path = entry.path();
        if let Some(ext) = path.extension() {
            if ext.to_str().unwrap() == args.ext {
                let filename = path.file_name().unwrap().to_str().unwrap();
                let new_filename = char_replace(&filename);
                let new_path = path.with_file_name(&new_filename);
                match fs::rename(&path, &new_path) {
                    Ok(_) => { info!("Renamed {} to {}", filename, new_filename); },
                    Err(e) => {
                        warn!("Failed to rename {}: {}", path.display(), e);
                        continue;
                    }
                }
                info!("Renamed {} to {}", path.display(), new_path.display());
                if (i + 1) % 10 == 0 {
                    trace!("Processed {} files", i + 1);
                }
                }
            }
        }
    info!("Process finished.");
    println!("Press enter to exit");
    let mut buffer = String::new();
    if let Err(e) = io::stdin().read_line(&mut buffer) {
        error!("Error reading from stdin: {}", e);
    }
}

fn char_replace(filename: &str) -> String {
    let re = regex::Regex::new(r"[^a-zA-Z0-9_-]").unwrap();
    debug!("Expression: {}", &re);
    re.replace_all(filename, "").to_string()
}