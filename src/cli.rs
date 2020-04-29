use std::path::PathBuf;
use std::time::Duration;
use structopt::StructOpt;

#[derive(Debug, Clone, StructOpt)]
#[structopt(
    name = "pinger",
    about = "Pinger takes a list of hosts and pings them periodically, printing the csv formatted result to stdout or file.",
    after_help = "Try:\n    pinger.exe 1.1.1.1\n    pinger.exe --interval 5 mozilla.org rust-lang.org\n    pinger.exe --output log.csv 1.1.1.1"
)]
pub struct Opt {
    /// Output file, stdout if not present
    #[structopt(short = "o", long = "output", parse(from_os_str))]
    pub output: Option<PathBuf>,

    /// Interval between pings (in seconds)
    #[structopt(short = "i", long = "interval", default_value = "10", parse(try_from_str = parse_duration))]
    pub interval: Duration,

    /// Timeout for each ping (in seconds)
    #[structopt(short = "t", long = "timeout", default_value = "3", parse(try_from_str = parse_duration))]
    pub timeout: Duration,

    /// Hosts to ping
    pub hosts: Vec<String>,
}

pub fn new_opt() -> Opt {
    Opt::from_args()
}

impl Opt {
    pub fn validate(&self) -> Result<(), String> {
        if self.interval < Duration::from_secs(1) {
            return Err(format!(
                "interval {} too short: use a value greater than 1 (second)",
                self.interval.as_secs()
            ));
        }
        if self.timeout < Duration::from_secs(1) {
            return Err(format!(
                "timeout {} too short: use a value greater than 1 (second)",
                self.timeout.as_secs()
            ));
        }
        if self.hosts.is_empty() {
            return Err(String::from("no hosts provided"));
        }
        Ok(())
    }

    pub fn hosts(&self) -> Vec<&str> {
        let args: Vec<&str> = self.hosts.iter().map(String::as_str).collect(); // convert Vec<String> into Vec<&str>
        let mut args = split_args(args);
        args.sort_unstable_by(|a, b| a.cmp(&b));
        args
    }
}

fn parse_duration(src: &str) -> Result<Duration, std::num::ParseIntError> {
    let n = src.trim().parse::<u64>();

    match n {
        Ok(n) => Ok(Duration::from_secs(n)),
        Err(e) => Err(e),
    }
}

fn split_args(v: Vec<&str>) -> Vec<&str> {
    let mut r: Vec<&str> = Vec::new();
    for s in &v {
        r.extend(s.split(',').map(|p| p.trim()));
    }
    r
}
