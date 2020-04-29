use async_std::net::{IpAddr, SocketAddr, ToSocketAddrs};
use async_std::prelude::*;
use async_std::{stream, task};

use winping::{AsyncPinger, Buffer};

extern crate chrono;
use chrono::Utc;

mod cli;
mod csv;
mod error;

use crate::cli::new_opt;
use crate::csv::CsvWriter;
use crate::error::CliError;

#[derive(Debug)]
struct Check {
    host: String,
    addr: String,
    available: bool,
    rtt: i64,
    error: Option<CliError>,
}

fn main() {
    task::block_on(task::spawn(async {
        // parse args
        let opt = new_opt();
        match opt.validate() {
            Ok(_) => {}
            Err(e) => {
                eprintln!("{}", e);
                return;
            }
        }
        let hosts = opt.hosts();

        // init and bind pinger
        let mut pinger = AsyncPinger::new();
        pinger.set_timeout(opt.timeout.as_millis() as u32);

        println!("using settings: {:#?}", opt);
        println!("\nexit with CTRL+C\n");

        // init csv writer and write header
        let path = opt.clone().output;
        let mut wtr = CsvWriter::new(path);
        let header = build_header(&hosts);
        match wtr.write_record(header) {
            Ok(_) => {}
            Err(e) => eprintln!("failed to write csv header with error: {:?}", e),
        }

        // handler for each series of checks
        let mut handle = |res: Result<Vec<Check>, CliError>| match res {
            Ok(checks) => {
                // print all failed checks to stderr
                checks.iter().filter(|c| c.error.is_some()).for_each(|c| {
                    eprintln!(
                        "host {} failed with error: {}",
                        c.host,
                        c.error.as_ref().unwrap()
                    )
                });

                // write and flush csv record
                let record = build_record(checks);
                match wtr.write_record(record) {
                    Ok(_) => {}
                    Err(e) => eprintln!("failed to write csv with error: {:?}", e),
                }
            }
            Err(e) => eprintln!("{:?}", e)
        };

        // start initial ping series
        let res: Result<Vec<Check>, CliError> = check_hosts(&pinger, &hosts).await;
        handle(res);

        // start timer
        let mut ticker = stream::interval(opt.interval);
        while let Some(_) = ticker.next().await {
            let res: Result<Vec<Check>, CliError> = check_hosts(&pinger, &hosts).await;
            handle(res);
        }
    }));
}

async fn check_hosts(pinger: &AsyncPinger, hosts: &[&str]) -> Result<Vec<Check>, CliError> {
    let mut checks: Vec<Check> = Vec::with_capacity(hosts.len());

    for host in hosts {
        let res = check_host(&pinger, &host).await;
        match res {
            Ok(res) => checks.push(res),
            Err(e) => {
                checks.push(Check {
                    host: (*host).to_string(),
                    addr: String::new(),
                    available: false,
                    rtt: -1,
                    error: e.into(),
                });
            }
        }
    }
    Ok(checks)
}

async fn check_host(pinger: &AsyncPinger, host: &str) -> Result<Check, CliError> {
    let addrs = resolve(host).await?;
    let dst = match pick_first_ip_addr(addrs) {
        Some(dst) => dst,
        None => {
            return Err(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                format!(r#"could not resolve address: `"{}"`"#, host),
            )
            .into());
        }
    };

    let res = ping(&pinger, &dst).await;
    match res {
        Ok(rtt) => Ok(Check {
            host: host.to_string(),
            addr: dst.to_string(),
            available: true,
            rtt,
            error: None,
        }),
        Err(e) => Err(CliError::WinpingError(e)),
    }
}

async fn resolve(host: &str) -> Result<Vec<SocketAddr>, std::io::Error> {
    let dst = format!("{}:53", host); // to_socket_addrs expects hostname + port
    let socket_addrs = dst.to_socket_addrs().await?;
    Ok(socket_addrs.collect())
}

fn pick_first_ip_addr(addrs: Vec<SocketAddr>) -> Option<IpAddr> {
    let ips: Vec<SocketAddr> = addrs
        .into_iter()
        .filter(|addr| addr.is_ipv4())
        .take(1)
        .collect();
    if ips.is_empty() {
        None
    } else {
        Some(ips[0].ip())
    }
}

async fn ping(pinger: &AsyncPinger, dst: &IpAddr) -> Result<i64, winping::Error> {
    let buf = Buffer::new();

    let res = pinger.send(*dst, buf).await;
    match res.result {
        Ok(rtt) => Ok(rtt as i64),
        Err(e) => Err(e),
    }
}

fn build_header(hosts: &[&str]) -> Vec<String> {
    let mut record: Vec<String> = Vec::with_capacity(hosts.len() * 5 + 1);

    record.push(String::from("Timestamp"));

    for _host in hosts {
        record.push(String::from("Host"));
        record.push(String::from("Address"));
        record.push(String::from("Available"));
        record.push(String::from("RTT"));
        record.push(String::from("Error"));
    }

    record
}

fn build_record(mut checks: Vec<Check>) -> Vec<String> {
    let mut record: Vec<String> = Vec::with_capacity(checks.len() * 5 + 1);
    checks.sort_unstable_by(|a, b| a.host.cmp(&b.host));

    let now = Utc::now().to_rfc3339();
    record.push(now);

    for check in checks {
        record.push(check.host);
        record.push(check.addr);
        record.push(check.available.to_string());
        record.push(check.rtt.to_string());
        if check.error.is_some() {
            record.push(check.error.unwrap().to_string());
        } else {
            record.push(String::new());
        }
    }

    record
}
