use std::io::{self, BufRead, BufReader, Write};
use std::net::TcpStream;
use std::str;

use log;
use log::{debug, error, log_enabled, info, Level};


mod cli;

fn main() {
    env_logger::init();
    let config = cli::Config::new("./Cargo.toml");

    debug!("config = {:?}", config);

    let ip = config.get_ip();
    let port = config.get_port();
    // let thread_num = config.get_thread_num();
    debug!("ip = {}", ip);
    debug!("port = {}", port);

    let ip_port = format!("{}:{}", ip, port);
    debug!("ip_port = {}", ip_port);
    let mut stream = TcpStream::connect(ip_port).expect("Could not connect to server");


    loop {
        let mut input = String::new();

        let mut buffer : Vec<u8> = Vec::new();

        io::stdin().read_line(&mut input)
            .expect("Failed to read from stdin");

        stream.write(input.as_bytes())
            .expect("Failed to write to server");
        
        let mut reader = BufReader::new(&stream);

        reader.read_until(b'\n', &mut buffer)
            .expect("Could not read into buffer");
        
        println!("{}", str::from_utf8(&buffer).expect("Could not write buffer as string"));
    }
}