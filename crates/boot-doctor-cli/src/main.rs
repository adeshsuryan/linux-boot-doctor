use std::path::PathBuf;
use std::process::ExitCode;

use boot_doctor_core::{
    capture_live, evaluate, load_capture_dir, render_json, render_text, Budgets, FailOn,
};
use clap::{Parser, ValueEnum};

#[derive(Debug, Clone, ValueEnum)]
enum Format {
    Text,
    Json,
}

#[derive(Debug, Parser)]
#[command(
    name = "linux-boot-doctor",
    version,
    about = "Diagnose slow Linux boots from systemd-analyze evidence (read-only)"
)]
struct Cli {
    #[command(subcommand)]
    cmd: Commands,
}

#[derive(Debug, clap::Subcommand)]
enum Commands {
    /// Analyze current boot or a saved capture directory
    Check {
        /// Capture directory with analyze-*.txt files
        #[arg(long)]
        input: Option<PathBuf>,

        /// Boot index for LIVE mode (systemd-analyze -b N)
        #[arg(long)]
        boot: Option<i32>,

        #[arg(long, value_enum, default_value = "text")]
        format: Format,

        #[arg(long, default_value_t = 30.0)]
        max_total: f64,

        #[arg(long, default_value_t = 5.0)]
        max_unit_sec: f64,

        #[arg(long, default_value_t = 32.0)]
        max_initrd_mb: f64,

        #[arg(long, default_value_t = 8.0)]
        max_initrd_sec: f64,

        /// Optional initrd size in bytes (overrides discovery / meta.json)
        #[arg(long)]
        initrd_bytes: Option<u64>,

        /// Fail when findings at this severity or worse are present (high|medium|low|none)
        #[arg(long, default_value = "high")]
        fail_on: String,

        /// Ignore rule id (repeatable)
        #[arg(long = "ignore-rule")]
        ignore_rule: Vec<String>,
    },
}

fn main() -> ExitCode {
    let cli = Cli::parse();
    match cli.cmd {
        Commands::Check {
            input,
            boot,
            format,
            max_total,
            max_unit_sec,
            max_initrd_mb,
            max_initrd_sec,
            initrd_bytes,
            fail_on,
            ignore_rule,
        } => {
            let fail_on = match FailOn::parse(&fail_on) {
                Some(v) => v,
                None => {
                    eprintln!("invalid --fail-on (use high|medium|low|none)");
                    return ExitCode::from(2);
                }
            };

            let mut evidence = match (&input, boot) {
                (Some(dir), _) => match load_capture_dir(dir) {
                    Ok(e) => e,
                    Err(e) => {
                        eprintln!("capture error: {e}");
                        return ExitCode::from(2);
                    }
                },
                (None, idx) => match capture_live(idx) {
                    Ok(e) => e,
                    Err(e) => {
                        let msg = e.to_string();
                        eprintln!("{msg}");
                        if msg.contains("unsupported environment") {
                            return ExitCode::from(3);
                        }
                        return ExitCode::from(2);
                    }
                },
            };

            if let Some(b) = initrd_bytes {
                evidence.initrd_bytes = Some(b);
            }

            let budgets = Budgets {
                max_total,
                max_unit_sec,
                max_initrd_mb,
                max_initrd_sec,
                ignore: ignore_rule,
                ..Budgets::default()
            };

            let findings = evaluate(&evidence, &budgets);
            let version = env!("CARGO_PKG_VERSION");

            match format {
                Format::Text => print!("{}", render_text(&evidence, &findings, &budgets)),
                Format::Json => match render_json(&evidence, &findings, &budgets, version) {
                    Ok(s) => println!("{s}"),
                    Err(e) => {
                        eprintln!("json error: {e}");
                        return ExitCode::from(2);
                    }
                },
            }

            let should_fail = findings.iter().any(|f| fail_on.should_fail(f.severity));
            if should_fail {
                ExitCode::from(1)
            } else {
                ExitCode::SUCCESS
            }
        }
    }
}
