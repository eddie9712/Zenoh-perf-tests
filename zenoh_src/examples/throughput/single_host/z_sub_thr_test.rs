use clap::{App, Arg};
use std::time::Instant;
use zenoh::config::Config;
use zenoh::prelude::*;

fn main() {
    // initiate logging
    env_logger::init();

    let (config, m, n, payload_size, unit) = parse_args();

    println!("Start test of payload size: {}", payload_size);

    let session = zenoh::open(config).wait().unwrap();

    let key_expr = session.declare_expr("/test/thr").wait().unwrap();
    let mut count = 0u128;
    let mut nm = 0;
    let mut start = Instant::now();
    let _sub = session
        .subscribe(&key_expr)
        .callback(move |_sample| {
            if count == 0 {
                start = Instant::now();
                count += 1;
            } else if count < n {
                count += 1;
            } else {
                print_stats(start, n, payload_size, &unit);
                nm += 1;
                count = 0;
                if nm >= m {
                    println!("End test of payloadsize: {}", payload_size);
                    std::process::exit(0)
                }
            }
        })
        .wait()
        .unwrap();

    // Stop forever
    std::thread::park();
}
fn print_stats(start: Instant, n: u128, payload_size: usize, unit: &String) {
    let elapsed = start.elapsed().as_secs_f64();
    let thpt;
    if *unit == "MB/s" {
        thpt = (((n as usize) * payload_size) as f64) / (elapsed * 1048576.0);
        println!("{} MB/s", thpt);
    } else {
        thpt = ((n as usize) as f64) / elapsed;
        println!("{} msg/s", thpt);
    }
}
fn parse_args() -> (Config, u32, u128, usize, String) {
    let args = App::new("zenoh throughput sub example")
        .arg(
            Arg::from_usage("-m, --mode=[MODE]  'The zenoh session mode (peer by default).")
                .possible_values(&["peer", "client"]),
        )
        .arg(Arg::from_usage(
            "-e, --peer=[LOCATOR]...   'Peer locators used to initiate the zenoh session.'",
        ))
        .arg(Arg::from_usage(
            "-l, --listener=[LOCATOR]...   'Locators to listen on.'",
        ))
        .arg(
            Arg::from_usage("-s, --samples=[number] 'Number of throughput measurements.'")
                .default_value("10"),
        )
        .arg(
            Arg::from_usage(
                "-n, --number=[number] 'Number of messages in each throughput measurements.'",
            )
            .default_value("100000"),
        )
        .arg(Arg::from_usage(
            "-c, --config=[FILE]      'A configuration file.'",
        ))
        .arg(Arg::from_usage(
          "-p,   --payload_size=[number] 'The tested payload_size.'"
        ))
        .arg(Arg::from_usage(
            "-u, --unit=[UNIT]    'The throughput of unit being specified (msg/s or MB/s, msg/s by default).'")
            .possible_values(&["msgs/s","MB/s"]),
        )
        .get_matches();
    let payload_size;
    let mut config = if let Some(conf_file) = args.value_of("config") {
        Config::from_file(conf_file).unwrap()
    } else {
        Config::default()
    };
    if let Some(Ok(mode)) = args.value_of("mode").map(|mode| mode.parse()) {
        config.set_mode(Some(mode)).unwrap();
    }
    match args.value_of("mode").map(|m| m.parse()) {
        Some(Ok(mode)) => {
            config.set_mode(Some(mode)).unwrap();
        }
        Some(Err(())) => panic!("Invalid mode"),
        None => {}
    };
    if let Some(values) = args.values_of("peer") {
        config.peers.extend(values.map(|v| v.parse().unwrap()))
    }
    if let Some(values) = args.values_of("listeners") {
        config.listeners.extend(values.map(|v| v.parse().unwrap()))
    }
    if args.is_present("no-multicast-scouting") {
        config.scouting.multicast.set_enabled(Some(false)).unwrap();
    }
    let unit = args.value_of("unit").unwrap().parse::<String>().unwrap();

    payload_size = args
                .value_of("payload_size")
                .unwrap()
                .parse::<usize>()
                .unwrap();

    let samples: u32 = args.value_of("samples").unwrap().parse().unwrap();
    let number: u128 = args.value_of("number").unwrap().parse().unwrap();

    (config, samples, number, payload_size, unit)
}

