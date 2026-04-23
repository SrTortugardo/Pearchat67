use local_ip_address::local_ip;
use std::net::IpAddr;

pub fn get_ip() -> Option<IpAddr> {
    match local_ip() {
        Ok(ip) => Some(ip),
        Err(e) => {
            eprintln!("Creo que hubo un pequeñisimo problema a la hora de sacarte la ip local. prueba quizas ejecutando ip a en tu term para sacar tu ip. Ademas verifica que estes conectado : {}", e);
            None
        }
    }
}
