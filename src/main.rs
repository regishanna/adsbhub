use anyhow::Context;
use std::{io::{Read, Write}, net::TcpStream, thread, time::Duration};


const DUMP1090_ADDR: &str = "localhost:30002";
const ADSBHUB_ADDR: &str = "data.adsbhub.org:5001";
//const ADSBHUB_ADDR: &str = "localhost:30003";


fn forward_data() -> anyhow::Result<()> {
    let mut buf = [0u8; 1024];

    // Connexion aux serveurs
    let mut dump1090 = TcpStream::connect(DUMP1090_ADDR).context("Echec de connexion a dump1090")?;
    let mut adsbhub = TcpStream::connect(ADSBHUB_ADDR).context("Echec de connexion a ADSBHub")?;

    // Transfert des donnees
    loop {
        // Lecture depuis dump1090
        let size = dump1090.read(&mut buf).context("Echec de lecture depuis dump1090")?;
        if size == 0 {
            return Err(anyhow::anyhow!("Deconnexion recue de dump1090"));
        }

        // Transmission a ADSBHub
        adsbhub.write_all(&buf[..size]).context("Echec d'ecriture vers ADSBHub")?;
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
