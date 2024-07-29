use config::Config;
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

    // let mods = parse_flags(&config);

    println!("{:#?}", &config);
    ping2(&config);
}

fn ping2(config: &Config) {
    let upperlim = config.lim;

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

// struct mods {
//     upper_lim: u32,
//     use_local: bool,
// }

// fn parse_flags(config: &Config) {
//     let mut mods = mods {
//         upper_lim: 4,
//         use_local: true,
//     };

//     let flags = &config.flags;

//     if flags.contains(&"-u") {
//         mods.upper_lim = match get_element_after_u(flags) {
//             Some(x) => x,
//             Err(_) => 4,
//         }
//     }
// }

// fn get_element_after_u<'a>(flags: &'a Vec<&'a str>) -> Option<&'a u32> {
//     for window in flags.windows(2) {
//         if let ["-u", value] = &window[..] {
//             Some(match value.parse::<u32>() {
//                 Ok(n) => return n,
//                 Err(_) => return 4,
//             })
//             };
//         }
//     }
//     None
// }
