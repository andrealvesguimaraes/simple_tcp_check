use clap::{arg, Command};
use std::fs::File;
use std::io::{BufReader, BufRead, Error};
use std::net::TcpStream;
use std::net::ToSocketAddrs;
use std::time::Duration;

fn main() -> Result<(), Error>{
    let matches = Command::new("HOSTS CHECK")
        .version("1.0")
        .author("Andre Guimaraes <andrealvesguimaraes@gmail.com>")
        .about("TESTE CONECTIVIDADE COM USO DE SOCKET TCP")
        .arg(arg!(-f --file <FILE>).required(true))
        .arg(arg!(-p --port <PORT>).required(true))
        .get_matches();

    let path = matches.get_one::<String>("file").expect("required");

    
    let input = File::open(path)?;
    let buffered = BufReader::new(input);

    for line in buffered.lines() {
        //println!("{:?}", (line?) );
        let ip_value = line?;
        let ip_add = ip_value.as_str();
        let tcp_port: &str = matches.get_one::<String>("port").unwrap().as_str();
        let socket = ip_add.to_owned()+":"+tcp_port;
        let socket_iter = socket.to_socket_addrs().unwrap().last().unwrap();
                if let Ok(_stream) = TcpStream::connect_timeout(&socket_iter,Duration::new(2, 0)) 
        {
            print!("Connected to the server!");
        } else {
            print!("Couldn't connect to server...");
        }
        println!(" - {} ",socket);
    }

    Ok(())
}
