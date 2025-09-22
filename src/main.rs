
use std::{io::{self, Read, Write}, net::{TcpListener, TcpStream}, process::exit, time::Duration};
use console::{ask_default, confirm_or_exit, escape_nl};
use net_util::{signal, stream_read};
use tungstenite::{accept_hdr, handshake::{client::Request, server::Response}, WebSocket};

mod console;
mod net_util;

const USER_AGENT: &str = "ImpressControl";
const DEFAULT_IP: &str = "localhost";

const DEFAULT_IMPRESS_PORT: u16 = 1599;
const DEFAULT_SERVER_PORT: u16 = 1600;

const TIMEOUT: Duration = Duration::from_secs(5);

fn main() {
    println!("Welcome to ImpressProxy");

    let mut server = make_server();

    let mut client = make_client();
    handshake(&mut client);
    
    for stream in server.incoming() {
        match stream {
            Ok(stream) => {
                let str_address = stream.local_addr().map(|a| a.to_string()).unwrap_or("unknown".to_string());
                println!("Incoming connection from: {}", str_address);

                let mut ws = init_websocket(&server, &stream);
                net_loop(&mut ws, &mut client);
            },
            Err(err) => {
                eprintln!("Incoming connection failed: {}", err);
                return;
            }
        }
    }
}

fn init_websocket<'a>(server: &TcpListener, stream: &'a TcpStream) -> WebSocket<&'a TcpStream> {
    stream.set_nonblocking(false).unwrap();

    let ws = accept_hdr(stream, |_req: &Request, res: Response| {
        Ok(res)
    });
    
    match ws {
        Ok(ws) => {
            stream.set_nonblocking(true).unwrap();
            ws
        },
        Err(err) => {
            eprintln!("Failed to initialize WebSocket: {}", err);
            exit(1);
        }
    }
}

fn make_server() -> TcpListener {
    match TcpListener::bind(format!("127.0.0.1:{}", DEFAULT_SERVER_PORT)) {
        Ok(server) => server,
        Err(err) => {
            eprintln!("Failed to start server: {}", err);
            exit(1);
        }
    }
}

fn net_loop<'a>(websocket: &mut WebSocket<&'a TcpStream>, impress_client: &mut TcpStream) {
    let mut impress_buf = [0u8; 256];
    
    loop {
        match websocket.read() {
            Ok(msg) => {
                match msg.to_text() {
                    Ok(str) => {
                        let string = escape_nl(str.to_string());
                        println!("WebSocket -> '{}' -> Impress", string);
                    },
                    Err(err) => eprintln!("Unable to decode data from WebSocket: {}", err)
                }
            },
            Err(err) => match err {
                tungstenite::Error::ConnectionClosed | tungstenite::Error::AlreadyClosed => {
                    println!("Disconnected, waiting for new connections");
                    break;
                },
                tungstenite::Error::Io(_) => (),
                _ => eprintln!("WebSocket error: {}", err)
            }
        }

        match impress_client.read(&mut impress_buf) {
            Ok(_) =>  {
                let msg = String::from_utf8(impress_buf.to_vec());

                match msg {
                    Ok(msg) => {
                        println!("Impress -> '{}' -> WebSocket", escape_nl(msg));
                    },
                    Err(err) => eprintln!("Invalid message from Impress: {}", err)
                }
            },
            Err(err) => {
                let kind = err.kind();

                match kind {
                    io::ErrorKind::WouldBlock => (),
                    _ => eprintln!("Impress read error: {}", kind)
                }
            }
        }
    }
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
            println!("Pairing successful! \n");            
            break;
        }

        stream.set_nonblocking(true).expect("Unable to make nonblocking client");
    }
}

fn make_client() -> TcpStream {
    let ip = ask_default("IP address", DEFAULT_IP);
    
    let stream_res = TcpStream::connect(format!("{}:{}", ip, DEFAULT_IMPRESS_PORT));

    match stream_res {
        Ok(stream) => {
            stream.set_write_timeout(Some(TIMEOUT)).expect("Failed to set write timeout");
            stream.set_read_timeout(Some(TIMEOUT)).expect("Failed to set read timeout");

            stream
        },
        Err(err) => {
            eprintln!("Failed to connect: {}", err);
            confirm_or_exit("Try again?");

            return make_client();
        }
    }
}

