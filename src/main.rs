use anyhow::Context;
use std::{io::{Read, Write}, net::TcpStream, thread, time::Duration};


const DUMP1090_ADDR: &str = "localhost:30002";
const ADSBHUB_ADDR: &str = "data.adsbhub.org:5001";


fn forward_data() -> anyhow::Result<()> {
    let mut buf = [0u8; 1024];

    // Connection to servers
    let mut dump1090 = TcpStream::connect(DUMP1090_ADDR).context("Failed to connect to dump1090")?;
    let mut adsbhub = TcpStream::connect(ADSBHUB_ADDR).context("Failed to connect to ADSBHub")?;

    // Data transfer
    loop {
        // Reading from dump1090
        let size = dump1090.read(&mut buf).context("Failed to read from dump1090")?;
        if size == 0 {
            return Err(anyhow::anyhow!("Disconnection received from dump1090"));
        }

        // Transmission to ADSBHub
        adsbhub.write_all(&buf[..size]).context("Failed to write to ADSBHub")?;
    }
}


fn main() {
    loop {
        if let Err(e) = forward_data() {
            println!("{:#}", e);
        }
        thread::sleep(Duration::from_secs(5));
    }
}
