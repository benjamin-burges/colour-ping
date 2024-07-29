use config::{Config, Mods};
use std::{env, thread, time::Duration};
use winping::{Buffer, Pinger};

mod config;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("You must supply an address");
        return;
    }

    let config = match Config::parse_args(&args) {
        Ok(con) => con,
        Err(err) => {
            println!("Failed to parse arguments: {}", err);
            return;
        }
    };

    let mods = Mods::parse_flags(&config);

    println!("{:#?}", &config);
    println!("{:#?}", &mods);

    ping2(&config, mods);
}

fn ping2(config: &Config, mods: Mods) {
    let upperlim = mods.upper_lim;

    let pinger = Pinger::new().unwrap();
    let mut buffer = Buffer::new();

    for i in 0..upperlim {
        let ip = std::net::IpAddr::V4(config.ips[0].ip);
        match pinger.send(ip, &mut buffer) {
            Ok(rtt) => println!("Ping {} -- Response time {} ms.", i + 1, rtt),
            Err(err) => println!("Failed to ping {}: {}.", ip, err),
        }
        if i + 1 != upperlim {
            thread::sleep(Duration::from_secs(1));
        }
    }
}
