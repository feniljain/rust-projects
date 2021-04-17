use std::{
    env,
    io::{self, Write},
    net::IpAddr,
    net::{SocketAddr, TcpStream},
    process,
    str::FromStr,
    sync::mpsc::{channel, Sender},
    thread,
    time::Duration,
};

const MAX: u16 = 65535;

struct Arguments {
    flag: String,
    ipaddr: IpAddr,
    threads: u16,
}

impl Arguments {
    fn new(args: &[String]) -> Result<Arguments, &'static str> {
        if args.len() < 2 {
            return Err("not enough arguments");
        } else if args.len() > 4 {
            return Err("too many arguments");
        }

        let ipadd_str = args[1].clone();
        match IpAddr::from_str(&ipadd_str) {
            Ok(ipaddr) => {
                return Ok(Arguments {
                    flag: String::from(""),
                    ipaddr,
                    threads: 4,
                })
            }
            Err(_) => {
                let flag = args[1].clone();
                if (flag.contains("-h") || flag.contains("--help")) && args.len() == 2 {
                    println!("-h || -help for help \n -j <threads> <ipaddr> to specify no. of threads to be used \n <ipaddr> to start scan with deafult params on given IP address");
                    return Err("help");
                } else if flag.contains("-h") || flag.contains("--help") {
                    return Err("too many arguments");
                } else if flag.contains("-j") && args.len() == 4 {
                    let ipaddr = match IpAddr::from_str(&args[3]) {
                        Ok(s) => s,
                        Err(_) => return Err("Invalid IP address"),
                    };

                    let threads: u16 = match args[2].parse() {
                        Ok(t) => t,
                        Err(_) => return Err("Invalid thread count"),
                    };

                    return Ok(Arguments {
                        flag,
                        ipaddr,
                        threads,
                    });
                } else {
                    return Err(
                        "Invalid syntax, run -h to understand the allowed flags and values",
                    );
                }
            }
        }
    }
}

fn scan(tx: Sender<u16>, start_port: u16, addr: &IpAddr, num_threads: u16) {
    let mut port: u16 = start_port + 1;

    let duration = Duration::new(0, 100);
    loop {
        let s = SocketAddr::new(*addr, port);
        match TcpStream::connect_timeout(&s, duration) {
            Ok(_) => {
                print!(".");
                io::stdout().flush().unwrap();
                tx.send(port).unwrap();
            }
            Err(_) => {}
        }

        if (MAX - port) <= num_threads {
            break;
        }

        port += num_threads;
    }
}

fn main() {
    let args_vec: Vec<String> = env::args().collect();

    let mode = args_vec[0].clone();
    let args = Arguments::new(&args_vec).unwrap_or_else(|err| {
        if err.contains("help") {
            process::exit(0);
        } else {
            eprintln!("{} problem parsing arguments: {}", mode, err);
            process::exit(0);
        }
    });

    let _mode = args.flag;
    let num_threads = args.threads;
    let addr = args.ipaddr;
    let (tx, rx) = channel();
    for i in 0..num_threads {
        let tx = tx.clone();

        thread::spawn(move || {
            scan(tx, i, &addr, num_threads);
        });
    }

    let mut out = vec![];
    drop(tx);
    for p in rx {
        out.push(p);
    }

    println!("");
    out.sort();
    for v in out {
        println!("{} is open", v);
    }
    //192.168.43.221
}
