
use std::{io::Write, net::TcpStream, time::Duration};
use console::{ask_default, confirm_or_exit};
use global_hotkey::{hotkey::{Code, HotKey}, GlobalHotKeyEvent, GlobalHotKeyManager, HotKeyState};
use net_util::{signal, stream_read};

mod console;
mod net_util;

const USER_AGENT: &str = "ImpressControl";
const DEFAULT_IP: &str = "localhost";
const DEFAULT_PORT: u16 = 1599;

const TIMEOUT: Duration = Duration::from_secs(5);

const KEY_PREV: Code = Code::KeyK;
const KEY_NEXT: Code = Code::KeyL;

fn main() {
    println!("Welcome to ImpressControl");

    let input_manager = GlobalHotKeyManager::new().expect("Failed to create keyboard input manager");
    let key_prev = HotKey::new(None, KEY_PREV);
    let key_next = HotKey::new(None, KEY_NEXT);
    
    input_manager.register(key_prev).expect("Failed to register key");
    input_manager.register(key_next).expect("Failed to register key");
    
    let mut stream = make_stream();
    stream.set_write_timeout(Some(TIMEOUT)).expect("Failed to set write timeout");
    stream.set_read_timeout(Some(TIMEOUT)).expect("Failed to set read timeout");
    
    handshake(&mut stream);
    signal(&mut stream, "presentation_start");

    loop {
        process_input(&mut stream, &key_prev, &key_next);
    }
}

fn process_input(stream: &mut TcpStream, key_prev: &HotKey, key_next: &HotKey) {
    if let Ok(event) = GlobalHotKeyEvent::receiver().try_recv() {
        if event.state() != HotKeyState::Pressed {
            return;
        }

        if event.id() == key_prev.id() {
            signal(stream, "transition_previous");
        }

        if event.id() == key_next.id() {
            signal(stream, "transition_next");
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
            println!("Use the following keys to control the presentation: ");

            println!("{}: Previous slide/animation", KEY_PREV);
            println!("{}: Next slide/animation", KEY_NEXT);
            
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

