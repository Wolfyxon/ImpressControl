
use std::{io::Write, net::TcpStream, time::Duration};
use console::{ask_default, confirm_or_exit};
use net_util::stream_read;

mod console;
mod net_util;

const USER_AGENT: &str = "ImpressControl";
const DEFAULT_IP: &str = "localhost";
const DEFAULT_PORT: u16 = 1599;

const TIMEOUT: Duration = Duration::from_secs(5);

fn main() {
    println!("Welcome to ImpressControl");

    let mut stream = make_stream();
    stream.set_write_timeout(Some(TIMEOUT)).expect("Failed to set write timeout");
    stream.set_read_timeout(Some(TIMEOUT)).expect("Failed to set read timeout");
    
    handshake(&mut stream);
}

fn handshake(stream: &mut TcpStream) {
    let pin = "1234";
    let data = format!("LO_SERVER_CLIENT_PAIR\n{}\n{}\n\n", USER_AGENT, pin);

    stream.write(data.as_bytes()).expect("Handshake failed");
    
    println!("PIN: {}", pin);
    println!("Go to 'Slide Show' > 'Impress Remote' and enter the pin");
    
    await_auth(stream);
}

fn await_auth(stream: &mut TcpStream) {
    loop {
        let data = stream_read(stream);

        if data.contains("LO_SERVER_SERVER_PAIRED") {
            println!("Pairing successful!");
            break;
        }
    }
}

fn make_stream() -> TcpStream {
    let ip = ask_default("IP address", DEFAULT_IP);
    
    let stream_res = TcpStream::connect(format!("{}:{}", ip, DEFAULT_PORT));

    match stream_res {
        Ok(stream) => stream,
        Err(err) => {
            eprintln!("Failed to connect: {}", err);
            confirm_or_exit("Try again?");

            return make_stream();
        }
    }
}

