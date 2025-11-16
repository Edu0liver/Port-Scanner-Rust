use clap::Parser;
use owo_colors::OwoColorize;
use rayon::{ThreadPool, prelude::*};
use thiserror::Error;
use std::{
    collections::HashMap,
    net::{IpAddr, SocketAddr, TcpStream, ToSocketAddrs},
    time::Duration,
};

#[derive(Parser, Debug)]
#[command(version, about)]
struct Args {
    #[arg(long, value_name = "HOST")]
    host: String,

    #[arg(long, value_name = "START"/*, required_unless_present = "ports"*/)]
    start_port: Option<u16>,

    #[arg(long, value_name = "END", requires = "start_port")]
    end_port: Option<u16>,

    #[arg(short, long, value_delimiter = ',', num_args = 1..)]
    ports: Option<Vec<u16>>,

    #[arg(short, long, default_value_t = 100)]
    threads: usize,

    #[arg(long, default_value_t = 1000)]
    timeout: u64,
}

struct Configs<'a> {
    args: &'a Args,
    pool: &'a ThreadPool,
}

#[derive(Error, Debug)]
pub enum ConnectError {
    #[error("Timeout ao conectar")]
    Timeout,
    #[error("Erro de conexão: {0}")]
    Connection(#[from] std::io::Error),
}

fn exec(configs: Configs) {
    let ports_vec: &Vec<u16> = if let Some(ports) = &configs.args.ports {
        ports
    } else if let (Some(start), Some(end)) = (configs.args.start_port, configs.args.end_port) {
        if end < start {
            eprintln!("Error: --end-port deve ser maior ou igual a --start-port.");
            std::process::exit(1);
        }

        &(start..=end).collect()
    } else {
        &(1u16..=65535).collect()
    };

    scan(&configs, &ports_vec);
}

fn get_service_name(port: u16) -> &'static str {
    static SERVICES: once_cell::sync::Lazy<HashMap<u16, &str>> = once_cell::sync::Lazy::new(|| {
        let m = HashMap::from([
            (21, "FTP"),
            (22, "SSH"),
            (23, "Telnet"),
            (25, "SMTP"),
            (53, "DNS"),
            (80, "HTTP"),
            (110, "POP3"),
            (143, "IMAP"),
            (443, "HTTPS"),
            (3306, "MySQL"),
            (3389, "RDP"),
            (5432, "PostgreSQL"),
            (8080, "HTTP-Alt"),
        ]);

        m
    });

    SERVICES.get(&port).copied().unwrap_or("Desconhecido")
}

fn scan(configs: &Configs, ports: &[u16]) {
    let host_ip: Option<IpAddr> = match configs.args.host.parse::<IpAddr>() {
        Ok(ip) => Some(ip),
        Err(_) => {
            match (configs.args.host.as_str(), 0)
                .to_socket_addrs()
                .ok()
                .and_then(|mut it| it.next())
            {
                Some(sa) => Some(sa.ip()),
                None => None,
            }
        }
    };

    if host_ip.is_none() {
        eprintln!("Could not parse or resolve host: {}", configs.args.host);
        return;
    }

    let host_ip = host_ip.unwrap();

    configs.pool.install(|| {
        ports.par_iter().for_each(|port| {
            let target = SocketAddr::new(host_ip, *port);
            let timeout = Duration::from_millis(configs.args.timeout);

            match TcpStream::connect_timeout(&target, timeout) {
                Ok(_) => {
                    let service = get_service_name(*port);
                    println!(
                        "{} {} - {}",
                        format!("{}/tcp", port).green(),
                        "ABERTA".bold(),
                        service
                    );
                }
                Err(e) if e.kind() == std::io::ErrorKind::TimedOut => (), // timed out / closed
                Err(e) => eprintln!("{}: error: {}", *port, e),
            }
        });
    })
}

fn main() {
    let args = Args::parse();
    let pool = rayon::ThreadPoolBuilder::new()
        .num_threads(args.threads)
        .build()
        .unwrap();

    exec(Configs {
        args: &args,
        pool: &pool,
    })
}
