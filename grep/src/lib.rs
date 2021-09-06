/// While using `&[&str]` to handle flags is convenient for exercise purposes,
/// and resembles the output of [`std::env::args`], in real-world projects it is
/// both more convenient and more idiomatic to contain runtime configuration in
/// a dedicated struct. Therefore, we suggest that you do so in this exercise.
///
/// In the real world, it's common to use crates such as [`clap`] or
/// [`structopt`] to handle argument parsing, and of course doing so is
/// permitted in this exercise as well, though it may be somewhat overkill.
///
/// [`clap`]: https://crates.io/crates/clap
/// [`std::env::args`]: https://doc.rust-lang.org/std/env/fn.args.html
/// [`structopt`]: https://crates.io/crates/structopt
///
use std::fs;
use std::io::Error;

pub mod GrepFlag {
    pub const PrintLineNumber: u8 = 0x01;
    pub const PrintFileNameOnly: u8 = 0x02;
    pub const CaseInSensitive: u8 = 0x04;
    pub const Invert: u8 = 0x08;
    pub const MatchEntireLine: u8 = 0x10;
}

#[derive(Debug)]
pub struct Flags(u8);

impl Flags {
    pub fn new(flags: &[&str]) -> Self {
        let mut f = 0;
        flags.iter().for_each(|&x| match x {
            "-n" => f = f | GrepFlag::PrintLineNumber,
            "-l" => f = f | GrepFlag::PrintFileNameOnly,
            "-i" => f = f | GrepFlag::CaseInSensitive,
            "-v" => f = f | GrepFlag::Invert,
            "-x" => f = f | GrepFlag::MatchEntireLine,
            _ => {}
        });
        Flags(f)
    }

    pub fn has_flag(&self, flag: u8) -> bool {
        return self.0 & flag == flag;
    }
}

pub fn grep(pattern: &str, flags: &Flags, files: &[&str]) -> Result<Vec<String>, Error> {
    Ok(files
        .iter()
        .map(&|filename| {
            let mut filename_exists = false;
            fs::read_to_string(filename).and_then(|contents| {
                Ok(contents
                    .split("\n")
                    .into_iter()
                    .zip(1..)
                    .filter_map(|(line, line_num)| {
                        if match_pattern(pattern, line, flags) && line.len() > 0 {
                            if flags.has_flag(GrepFlag::PrintFileNameOnly) && !filename_exists {
                                filename_exists = true;
                                return Some(format!("{}", filename));
                            } else if filename_exists {
                                return None;
                            }

                            let file_name = if files.len() > 1 {
                                format!("{}:", filename)
                            } else {
                                "".to_owned()
                            };

                            let line_number = if flags.has_flag(GrepFlag::PrintLineNumber) {
                                format!("{}:", line_num)
                            } else {
                                "".to_owned()
                            };

                            Some(format!("{}{}{}", file_name, line_number, line))
                        } else {
                            None
                        }
                    })
                    .collect::<Vec<String>>())
            })
        })
        .collect::<Result<Vec<_>, _>>()?
        .into_iter()
        .flatten()
        .collect())
}

pub fn match_pattern(pattern: &str, line: &str, flags: &Flags) -> bool {
    let length = pattern.chars().map(|c| 1).sum();
    let match_f = |a: &str, b: &str, case_insensitive: bool| {
        if case_insensitive {
            a.to_lowercase() == b.to_lowercase()
        } else {
            a == b
        }
    };

    let result = if flags.has_flag(GrepFlag::MatchEntireLine) {
        match_f(line, pattern, flags.has_flag(GrepFlag::CaseInSensitive))
    } else {
        line.chars()
            .collect::<Vec<_>>()
            .windows(length)
            .map(|x| x.iter().collect::<String>())
            .any(|x| match_f(&x, pattern, flags.has_flag(GrepFlag::CaseInSensitive)))
    };

    if flags.has_flag(GrepFlag::Invert) {
        !result
    } else {
        result
    }
}
