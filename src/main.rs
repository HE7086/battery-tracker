use anyhow::Result;
use chrono::Utc;
use clap::Parser;
use serde_json::{Map, Value};
use std::fs::{self, File, OpenOptions};
use std::io::{BufRead, BufReader, BufWriter, Write};
use std::path::{Path, PathBuf};
use std::thread::sleep;
use std::time::Duration;

#[derive(Parser, Debug)]
#[command(version, about)]
struct Args {
    /// Enable to record non-battery device
    #[arg(short, long)]
    all: bool,

    /// Run in foreground instead of start a daemon
    #[arg(long)]
    foreground: bool,

    /// Run only once instead of repeating
    #[arg(long)]
    once: bool,

    /// Interval of process, default 60s
    #[arg(short, long, default_value_t = 60)]
    interval: u64,

    /// Output location for logs
    #[arg(short, long, default_value_t = String::from("/var/lib/battery-tracker"))]
    output: String,
}

fn main() -> Result<()> {
    let mut args = Args::parse();

    if args.once {
        run(&args)?;
        return Ok(());
    }

    if !args.foreground {
        if !Path::exists(Path::new(args.output.as_str())) {
            File::create(args.output.as_str())?;
        }

        let mut file = PathBuf::from(&args.output);
        file.push(format!("battery-{}.log", Utc::now().to_rfc3339()));
        args.output = String::from(file.to_string_lossy());
    }

    loop {
        run(&args)?;
        sleep(Duration::new(args.interval, 0));
    }
}

fn run(args: &Args) -> Result<()> {
    let power_supply = Path::new("/sys/class/power_supply");
    for entry in fs::read_dir(power_supply)? {
        let name = entry?.file_name().into_string().unwrap();
        if !args.all && !name.starts_with("BAT") {
            continue;
        }

        let path = power_supply.join(name).join("uevent");
        let mut map = Map::new();
        let uevent = File::open(path)?;
        let lines = BufReader::new(uevent).lines();
        for line in lines {
            let line = line?;
            let mut split = line.split("=");

            #[cfg(debug_assertions)]
            assert_eq!(split.clone().count(), 2);

            let key = split.next().unwrap().trim_start_matches("POWER_SUPPLY_");
            let value = split.next().unwrap();
            map.insert(key.to_string(), Value::String(value.to_owned()));
        }
        map.insert("TIME".to_string(), Value::String(Utc::now().to_rfc3339()));

        if args.foreground {
            println!("{}", serde_json::to_string_pretty(&map)?);
        } else {
            let mut writer = BufWriter::new(OpenOptions::new().append(true).open(args.output.as_str())?);
            writer.write(serde_json::to_string(&map)?.as_bytes())?;
            writer.write("\n".as_bytes())?;
        }
    }
    Ok(())
}
