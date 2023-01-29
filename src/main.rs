use std::net::{IpAddr, SocketAddr, TcpStream};
use clap::{Parser};
use std::ops::{RangeInclusive};
use std::time::{Instant, Duration};
use std::str::FromStr;
use std::thread::sleep;
use trust_dns_resolver::config::{ResolverConfig, ResolverOpts};
use trust_dns_resolver::Resolver;


/// Simple TCP Ping write with rust
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// -w 5 :ping every 5 seconds
    #[arg(short, default_value_t = 1)]
    w: u64,
    /// -l 5 :send 5 pings
    #[arg(short, default_value_t = 3)]
    l: i64,
    /// -t 5 :timeout 5 seconds
    #[arg(short, default_value_t = 2)]
    t: u64,
    /// target
    target: String,
    /// port
    #[arg(default_value_t = String::from("80"))]
    port: String,
}

fn main() {
    let cli = Cli::parse();
    let port = cli.port.split("-").map(|x|
        match port_in_range(&x) {
            Ok(port) => { port }
            Err(err) => {
                println!("{}", err);
                return 0u16;
            }
        }
    ).filter(|x| !x.eq(&0u16)).collect::<Vec<_>>();
    let mut start_port = port[0];
    let mut end_port = port[0];
    if port.len() > 1 {
        if port[0] > port[1] {
            start_port = port[1];
            end_port = port[0];
        } else {
            start_port = port[0];
            end_port = port[1];
        }
    }


    for port in RangeInclusive::new(start_port, end_port) {
        match parse(cli.target.clone(), port) {
            Ok(addr) => {
                let mut count = 0;
                let mut fail = 0;
                let mut min_time = Duration::from_secs(cli.t.clone() + 1);
                let mut max_time = Duration::from_secs(0);
                let mut count_time = Duration::from_secs(0);
                for _ in 1..=cli.l.clone() {
                    count += 1;
                    let start = Instant::now();
                    let p = ping(addr, Duration::from_secs(cli.t.clone()));
                    let duration = start.elapsed();
                    let str;
                    if p {
                        str = "Port is open";
                    } else {
                        fail += 1;
                        str = "No response";
                    }
                    if duration < min_time {
                        min_time = duration
                    }
                    if duration > max_time {
                        max_time = duration
                    }
                    count_time += duration;
                    println!("Ping {}/tcp - {} - time= {:.3}ms", addr, str, (duration.as_micros() as f64) / 1000.0);
                    if count < cli.l.clone() {
                        sleep(Duration::from_secs(cli.w.clone()));
                    }
                }
                println!("Ping statistics for {}", addr);
                println!("\t{} probes sent.", cli.l.clone());
                println!("\t{} successful, {} failed. ({:.2}% fail)", count - fail, fail, (fail as f64) / (count as f64) * 100.0);
                println!("Approximate trip times in milli-seconds:");
                println!("\t Minimum = {:.3}ms, Maximum = {:.3}ms, Average = {:.3}ms",
                         (min_time.as_micros() as f64) / 1000.0,
                         (max_time.as_micros() as f64) / 1000.0,
                         (count_time.as_micros() as f64 / count as f64 / 1000.0));
            }
            Err(err) => {
                println!("{}", err);
                return;
            }
        }
    }
}

fn parse(target: String, port: u16) -> Result<SocketAddr, String> {
    let mut addr: SocketAddr;

    let mut addr_str = String::from(target.as_str());
    addr_str.push(':');
    addr_str.push_str(port.to_string().as_str());
    if let Ok(x) = SocketAddr::from_str(addr_str.as_str()) {
        addr = x;
    } else {
        let resolver = Resolver::new(ResolverConfig::default(), ResolverOpts::default()).unwrap();
        let mut address: Option<IpAddr> = None;
        if let Ok(r) = resolver.lookup_ip(target.clone()) {
            if let Some(a) = r.iter().next() {
                address = Some(a);
            }
        }
        if let Some(s) = address {
            addr = SocketAddr::new(s.to_owned(), port);
        } else {
            return Err(format!("DNS: Could not find host - {} : {}", target, port));
        }
    }
    return Ok(addr);
}

fn ping(target: SocketAddr, timeout: Duration) -> bool {
    match TcpStream::connect_timeout(&target, timeout) {
        Ok(x) => {
            x.try_clone();
            true
        }
        Err(_) => {
            false
        }
    }
}


const PORT_RANGE: RangeInclusive<usize> = 1..=65535;

fn port_in_range(s: &str) -> Result<u16, String> {
    let port: usize = s
        .parse()
        .map_err(|_| format!("`{}` isn't a port number", s))?;
    if PORT_RANGE.contains(&port) {
        Ok(port as u16)
    } else {
        Err(format!(
            "`{}` port not in range {}-{}",
            s,
            PORT_RANGE.start(),
            PORT_RANGE.end()
        ))
    }
}