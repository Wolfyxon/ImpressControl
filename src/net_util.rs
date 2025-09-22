use std::{io::Read, net::TcpStream};

pub fn stream_read(stream: &mut TcpStream) -> String {
    let mut buf = [0; 128];

    match stream.read(&mut buf) {
        Ok(_) => (),
        Err(err) => {
            eprintln!("Read error: {}", err);
            return String::new();
        }
    };

    match String::from_utf8(buf.to_vec()) {
        Ok(string) => string,
        Err(err) => {
            eprintln!("Failed to decode packet: {}", err);
            return String::new();
        }
    }
}

/*
pub fn signal(stream: &mut TcpStream, message: impl Into<String>) {
    let msg = message.into();
    let data = format!("{}\n\n", msg);

    stream.write(data.as_bytes()).unwrap_or_else(|err| {
        eprintln!("Send error '{msg}': {err}");
        0
    });
}
*/
