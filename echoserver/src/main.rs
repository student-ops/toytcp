use std::error::Error;
use std::io::{Read, Write};
use std::net::TcpListener;
use std::{env, str, thread};

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    let addr = &args[1];
    echo_server(addr)?;
    Ok(())
}

fn echo_server(address: &str) -> Result<(), Box<dyn Error>> {
    let listener = TcpListener::bind(address)?;
    loop {
        let (mut stream, _) = listener.accept()?;
        thread::spawn(move || {
            let mut buf = [0u8; 1024];
            loop {
                let n = stream.read(&mut buf).unwrap();
                if n == 0 {
                    return;
                }
                print!("{}", str::from_utf8(&buf[..n]).unwrap());
                stream.write_all(&buf[..n]).unwrap();
            }
        });
    }
}
