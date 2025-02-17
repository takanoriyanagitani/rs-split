use std::io;
use std::str::FromStr;

use std::io::BufWriter;
use std::io::Write;

use std::path::Path;

use std::fs::File;

pub const MAX_LINE_COUNT_PER_FILE_DEFAULT: usize = 1024;
pub const SHOW_PROGRESS_DEFAULT: bool = false;

pub fn index2basename(index: usize, basename: &mut String) {
    let name: String = format!("{index:08x}.txt");
    *basename = name;
}

pub fn file_sync_nop(_: &mut File) -> Result<(), io::Error> {
    Ok(())
}

pub fn file_sync_fsync(f: &mut File) -> Result<(), io::Error> {
    f.sync_all()
}

pub fn file_sync_fdatasync(f: &mut File) -> Result<(), io::Error> {
    f.sync_data()
}

pub fn lines2splited<I, P, F, S>(
    mut lines: I,
    output_dir_name: P,
    index_to_basename: F,
    max_line_per_file: usize,
    file_sync: S,
    show_progress: bool,
) -> Result<(), io::Error>
where
    I: Iterator<Item = Result<Vec<u8>, io::Error>>,
    P: AsRef<Path>,
    F: Fn(usize, &mut String),
    S: Fn(&mut File) -> Result<(), io::Error>,
{
    let mut basename: String = String::new();
    let mut ix: usize = 0;

    loop {
        basename.clear();

        index_to_basename(ix, &mut basename);
        let filename = output_dir_name.as_ref().join(&basename);
        let mut f = File::create(&filename)?;
        let mut bw = BufWriter::new(&mut f);

        let mut wrote_cnt: usize = 0;

        for _ in 0..max_line_per_file {
            let orl: Option<Result<Vec<u8>, _>> = lines.next();
            match orl {
                None => break,
                Some(Err(e)) => return Err(e),
                Some(Ok(line)) => {
                    bw.write_all(&line)?;
                    bw.write_all(b"\n")?;
                    wrote_cnt += 1;
                }
            }
        }

        ix += 1;

        if wrote_cnt < 1 {
            std::fs::remove_file(&filename)?;
            return Ok(());
        }

        bw.flush()?;
        drop(bw);

        f.flush()?;
        file_sync(&mut f)?;
        drop(f);

        if show_progress {
            eprintln!("{} wrote.", filename.display());
        }
    }
}

pub enum FileSyncType {
    Nop,
    Data,
    All,
}

pub const FILE_SYNC_TYPE_DEFAULT: FileSyncType = FileSyncType::Nop;

impl FromStr for FileSyncType {
    type Err = io::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "nop" => Ok(Self::Nop),
            "data" => Ok(Self::Data),
            "all" => Ok(Self::All),
            _ => Err(io::Error::other(format!("unknown type: {s}"))),
        }
    }
}

pub struct Config {
    pub output_dir_name: String,
    pub max_line_per_file: usize,
    pub file_sync_type: FileSyncType,
    pub show_progress: bool,
}

impl Config {
    pub fn split<I, F>(&self, lines: I, ix2base: F) -> Result<(), io::Error>
    where
        I: Iterator<Item = Result<Vec<u8>, io::Error>>,
        F: Fn(usize, &mut String),
    {
        match self.file_sync_type {
            FileSyncType::Nop => lines2splited(
                lines,
                &self.output_dir_name,
                ix2base,
                self.max_line_per_file,
                file_sync_nop,
                self.show_progress,
            ),
            FileSyncType::Data => lines2splited(
                lines,
                &self.output_dir_name,
                ix2base,
                self.max_line_per_file,
                file_sync_fdatasync,
                self.show_progress,
            ),
            FileSyncType::All => lines2splited(
                lines,
                &self.output_dir_name,
                ix2base,
                self.max_line_per_file,
                file_sync_fsync,
                self.show_progress,
            ),
        }
    }

    pub fn split_default<I>(&self, lines: I) -> Result<(), io::Error>
    where
        I: Iterator<Item = Result<Vec<u8>, io::Error>>,
    {
        self.split(lines, index2basename)
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            output_dir_name: "".into(),
            max_line_per_file: MAX_LINE_COUNT_PER_FILE_DEFAULT,
            file_sync_type: FILE_SYNC_TYPE_DEFAULT,
            show_progress: SHOW_PROGRESS_DEFAULT,
        }
    }
}
