use std::env;

/* Ahora si, importamos los modulos */
mod network;
mod server;
mod client;

fn main() {
    let args: Vec<String> = env::args().collect(); /* Aqui conseguimos argumentos */

    match args.get(1).map(|s| s.as_str()) { /* El primer argumento del juicio xd */
        Some("help") => {
            println!("pearchat : un chat para enviar mensajitos por red");
            println!("  ip : Esto imprime tu IP local asi se pueden conectar a ti, esto es como tu \"ID\"");
            println!("  join : Este te une a una sala, ocupas pasar como parametro la IP. Por ejemplo pearchat join 192.168.100.9");
            println!("  host : Esto hace que tu computadora actue como servidor para que los demas usuarios puedan unirse. No puedes enviar mensajes desde el servidor")
        }

        Some("ip") => {
            if let Some(ip) = network::get_ip() {
                println!("La ip local es : \"{}\"", ip);
            } else {
                println!("Creo que hubo un pequeñisimo problema a la hora de sacarte la ip local. prueba quizas ejecutando ip a en tu term para sacar tu ip. Ademas verifica que estes conectado"); /* *el mismo msg del mod network */
            }
        }

        Some("host") => {
            server::start();
        }

        Some("join") => {
            if let Some(ip) = args.get(2) {
                client::start_client(ip);
            }
        }
        _ => {
            println!("Ocupas decirle al programa que quieres hacer, ejecuta con la bandera help(no --help) para saber los comandos");
        }
    }
}
