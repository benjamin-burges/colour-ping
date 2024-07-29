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
    // pub lim: u32,
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
            // lim: 5,
        }
    }

    pub fn parse_args<'a>(args: &'a Vec<String>) -> Result<Config<'a>, Box<dyn Error>> {
        let mut config = Config::new();

        // [1..] to avoid the first arg wich is the location of the bin
        for arg in &args[1..] {
            if arg.contains('.') {
                match parse_address(arg) {
                    Ok(ipv4s) => {
                        for ip in ipv4s {
                            config.ips.push(Ip {
                                ip,
                                local: ip.is_private(),
                            });
                        }
                    }
                    Err(err) => return Err(err),
                }
            } else {
                config.flags.push(arg);
            }
        }
        Ok(config)
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

#[derive(Debug)]
pub struct Mods {
    pub upper_lim: u32,
    pub use_local: bool,
}

impl Mods {
    pub fn parse_flags(config: &Config) -> Mods {
        let mut mods = Mods {
            upper_lim: 4,
            use_local: false,
        };

        let flags = &config.flags;

        if flags.contains(&"-u") {
            mods.upper_lim = match get_element_after(flags) {
                Some(x) => x,
                None => 4,
            }
        }

        if flags.contains(&"-l") {
            mods.use_local = true;
        }

        return mods;
    }
}

fn get_element_after<'a>(flags: &'a Vec<&'a str>) -> Option<u32> {
    for window in flags.windows(2) {
        if let ["-u", value] = &window[..] {
            return match value.parse::<u32>() {
                Ok(n) => Some(n),
                Err(_) => None,
            };
        }
    }
    None
}
