use std::process::ExitCode;

use std::io;

use std::io::BufRead;

use rs_split::Config;
use rs_split::FileSyncType;
use rs_split::FILE_SYNC_TYPE_DEFAULT;
use rs_split::MAX_LINE_COUNT_PER_FILE_DEFAULT;
use rs_split::SHOW_PROGRESS_DEFAULT;

fn env_val_by_key(key: &'static str) -> Result<String, io::Error> {
    std::env::var(key).map_err(|e| io::Error::other(format!("env var {key} unknown: {e}")))
}

fn output_dir_name() -> Result<String, io::Error> {
    env_val_by_key("ENV_OUTPUT_DIR_NAME")
}

fn max_line_per_file() -> usize {
    env_val_by_key("ENV_MAX_LINE_PER_FILE")
        .ok()
        .and_then(|s| str::parse(s.as_str()).ok())
        .unwrap_or(MAX_LINE_COUNT_PER_FILE_DEFAULT)
}

fn file_sync_type() -> FileSyncType {
    env_val_by_key("ENV_FILE_SYNC_TYPE")
        .ok()
        .and_then(|s| str::parse(s.as_str()).ok())
        .unwrap_or(FILE_SYNC_TYPE_DEFAULT)
}

fn show_progress() -> bool {
    env_val_by_key("ENV_SHOW_PROGRESS")
        .ok()
        .and_then(|s| str::parse(s.as_str()).ok())
        .unwrap_or(SHOW_PROGRESS_DEFAULT)
}

fn config() -> Result<Config, io::Error> {
    output_dir_name().map(|dname: String| Config {
        output_dir_name: dname,
        max_line_per_file: max_line_per_file(),
        file_sync_type: file_sync_type(),
        show_progress: show_progress(),
    })
}

fn stdin2lines() -> impl Iterator<Item = Result<Vec<u8>, io::Error>> {
    std::io::stdin().lock().split(b'\n')
}

fn stdin2lines2splited() -> Result<(), io::Error> {
    let cfg: Config = config()?;
    let lines = stdin2lines();
    cfg.split_default(lines)
}

fn main() -> ExitCode {
    stdin2lines2splited()
        .map(|_| ExitCode::SUCCESS)
        .unwrap_or_else(|e| {
            eprintln!("{e}");
            ExitCode::FAILURE
        })
}
