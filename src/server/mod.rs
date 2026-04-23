use std::io::{BufRead, BufReader, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, Mutex};
use std::thread;

type Client = Arc<Mutex<TcpStream>>; /* esto es como un alias para simplificarme la vida */

pub fn start() { /* ahora si chavles vamos a iniciar esto */
    let listener = TcpListener::bind("0.0.0.0:6767") /* empieza */
        .expect("No se pudo iniciar el serve en el puerto 6767"); /* esto es si falla */

    println!("El servidor se esta ejecutando en tu puerto 6767"); /* nos notifica en que puerto corre */

    let clients: Arc<Mutex<Vec<Client>>> = Arc::new(Mutex::new(Vec::new())); /* esta es la lista de los clientes que atiende nuestro servidor */

    for incoming in listener.incoming() { /* el bucle supremo GG */
        let stream = match incoming {
            Ok(s) => s,
            Err(e) => {
                println!("Oops, honestamente no se que paso pero aqui te dejo el error {}", e);
                continue;
            }
        };

        let clients = Arc::clone(&clients); /* clon de clientes pero aqui */

        let read_stream = match stream.try_clone() { /* clonamos stream, ¿oara que perejiles?, lectura independiente o una cosa asi */
            Ok(s) => s,
            Err(e) => {
                println!("No se pudo clonar el stream porque no eres sigma: {}", e);
                continue;
            }
        };

        let client: Client = Arc::new(Mutex::new(stream)); /* volvemos al stream OG?, ya me perdi un poco */

        clients.lock().unwrap().push(Arc::clone(&client)); /* se agrega lista el cliente a la lista de epstin*/

        thread::spawn(move || { /* Ahora si, creamos un hilito por cada cliente */
            let mut reader = BufReader::new(read_stream); /* lineas */
            let mut line = String::new();

            if reader.read_line(&mut line).is_err() { /* la primer linea es username */
                return;
            }

            let username = line.trim().to_string();
            println!("{} Se ha unido", username); /* pal saludo de pol macarni mclennon*/

            loop {
                line.clear();

                match reader.read_line(&mut line) {
                    Ok(0) => {
                        /* esto es que el cliente ya no esta con nosotros... porque salio*/
                        println!("User {} left", username);
                        break;
                    }
                    Ok(_) => {
                        /* cuando se envian mensajes */
                        let msg = line.trim();
                        if msg.is_empty() { /* mensajes vacios, aqui no*/
                            continue;
                        }

                        let full_msg = format!("[{}] {}\n", username, msg); /* formato del mensaje final que se imprime en pantalla */
                        println!("{}", full_msg.trim_end()); /* lo imprime para que el server se entere de la smajaderias que andan diciendo */

                        let mut clients_guard = clients.lock().unwrap(); /* le enviamos a todos los clientes para que sepan */

                        clients_guard.retain(|client| { /* al enviarlo lo borramos */
                            let mut c = client.lock().unwrap();
                            c.write_all(full_msg.as_bytes()).is_ok()
                        });
                    }
                    Err(_) => { /* algo malio sal */
                        println!("Error leyendo un mensaje de parte de nuestro buen amigo {}", username);
                        break;
                    }
                }
            }
        });
    }
}
