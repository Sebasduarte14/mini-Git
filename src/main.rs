mod command;
mod repository;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    match command::parse(&args) {
        Ok(cmd) => println!("Comando reconocido: {:?}",cmd),
        Err(e) => eprintln!("Error: {}",e),
    }
}
