use std::{env::Args, ffi::OsString, path};

const USAGE: &str = "
data-cli
version: v1.0
desc: 根据指定的文件大小和文件数, 生成随机文件

USAGE:
    data-cli [OPTION]

OPTION:
    -n | --num      file number, default 1
    -s | --size     single file size, support unit: K/M/G
    -o | --output   where to put data, default ./output
                    overwrite it if exists
                    create directory if not exists
";

#[derive(Debug)]
pub struct Command {
    pub file_number: u16,
    pub file_size: u64,
    pub output: OsString,
}

impl Command {
    pub fn validate(&self) -> bool {
        self.file_size > 0
    }

    pub fn print_usage(&self) {
        println!("{}", USAGE);
    }
}

impl Default for Command {
    fn default() -> Self {
        let mut pwd = std::env::current_dir().expect("invalid current direction");
        pwd.push("output");

        Command {
            file_number: 1,
            file_size: 1024,
            output: pwd.into_os_string(),
        }
    }
}

impl From<Args> for Command {
    fn from(mut args: Args) -> Command {
        // skip first argrument: script name
        args.next();

        let mut cmd = Command::default();
        loop {
            let option = match args.next() {
                Some(s) => s,
                None => break,
            };

            match option.as_str() {
                "-n" | "--num" => {
                    if let Some(s) = args.next() {
                        if let Ok(num) = s.parse::<u16>() {
                            cmd.file_number = num;
                        }
                    }
                }
                "-s" | "--size" => {
                    if let Some(s) = args.next() {
                        let s = s.to_uppercase();

                        let (suffix, unit) = match s.bytes().last() {
                            Some(b'G') => (s.strip_suffix('G').unwrap(), 1024 * 1024 * 1024),
                            Some(b'M') => (s.strip_suffix('M').unwrap(), 1024 * 1024),
                            Some(b'K') => (s.strip_suffix('K').unwrap(), 1024),
                            _ => (s.as_str(), 1),
                        };

                        if let Ok(v) = suffix.parse::<u16>() {
                            cmd.file_size = (v as u64) * unit;
                        }
                    }
                }
                "-o" | "--output" => {
                    if let Some(p) = args.next() {
                        let mut path = path::PathBuf::from(p);
                        if path.exists() {
                            path.push("output");
                            cmd.output = path.into_os_string();
                        }
                    }
                }
                _ => cmd.print_usage(),
            }
        }
        println!("{:?}", &cmd);
        cmd
    }
}
