use core::net::IpAddr;
use core::net::Ipv4Addr;
use dns_lookup;
use std::error::Error;
use std::str::FromStr;

#[derive(Debug)]
pub struct Config<'a> {
    // pub ip: IpAddr,
    pub ips: Vec<Ip>,
    pub flags: Vec<&'a str>,
    pub lim: u32,
}

#[derive(Debug)]
pub struct Ip {
    pub ip: Ipv4Addr,
    pub local: bool,
}

impl Config<'_> {
    pub fn new<'a>() -> Config<'a> {
        Config {
            ips: vec![],
            flags: vec![],
            lim: 5,
        }
    }

    pub fn parse_args<'a>(args: &'a Vec<String>) -> Result<Config<'a>, Box<dyn Error>> {
        let mut config = Config::new();

        // [1..] to avoid the first arg wich is the location of the bin
        for arg in args[1..].iter() {
            if arg.contains(".") {
                match parse_address(&arg) {
                    Ok(ipv4s) => {
                        for ip in ipv4s {
                            config.ips.push(Ip {
                                ip: ip,
                                local: {
                                    if ip.is_private() {
                                        true
                                    } else {
                                        false
                                    }
                                },
                            })
                        }
                    }
                    Err(err) => return Err(err),
                }
            } else {
                config.flags.push(&arg)
            }
        }

        return Ok(config);
    }
}

fn parse_address(address: &str) -> Result<Vec<Ipv4Addr>, Box<dyn Error>> {
    match Ipv4Addr::from_str(address) {
        Ok(ipv4) => Ok(vec![ipv4]),
        Err(_) => resolve_dns(address),
    }
}

fn resolve_dns(address: &str) -> Result<Vec<Ipv4Addr>, Box<dyn Error>> {
    let ips = dns_lookup::lookup_host(address)?;
    let ipv4s: Vec<Ipv4Addr> = ips
        .into_iter()
        .filter_map(|ip| {
            if let IpAddr::V4(ipv4) = ip {
                Some(ipv4)
            } else {
                None
            }
        })
        .collect();
    Ok(ipv4s)
}
