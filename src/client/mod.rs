use std::net::TcpStream;
use std::io::{Write, Read, stdin};
use std::thread;

pub fn start_client(ip: &str) { /* este es nuestro cliente */
    let mut stream = TcpStream::connect(format!("{}:6767", ip))
        .expect("Hubo un herror uniendose a la sala que especificaste");

    /* ocupas poner username asi que pues bueno, lo pedimos */
    println!("Porfavor, escribe un nombre de usuario(asi te veran los demas usuarios) :");
    let mut username = String::new();
    stdin().read_line(&mut username).unwrap();

    stream.write_all(username.as_bytes()).unwrap(); /* *le enviamos el bello y hermoso nombre al señor servidor */ 

    println!("Conectado uwu");

    let mut read_stream = stream.try_clone().unwrap();

    thread::spawn(move || { /* para saber los mensajes */
        let mut buffer = [0; 1024];

        loop {
            match read_stream.read(&mut buffer) {
                Ok(0) => break,
                Ok(n) => {
                    let msg = String::from_utf8_lossy(&buffer[..n]);
                    println!("{}", msg); /* ya el server no los da en formato username + mensaje */
                }
                Err(_) => break,
            }
        }
    });

    loop { /* el bucle de enviar mensajes */
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();

        stream.write_all(input.as_bytes()).unwrap(); /* le envia esto al server */
    }
}
