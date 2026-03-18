mod command;
mod repository;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let mut repo = repository::Repository::new();

    // Dividir args en sub-comandos separados por "/"
    // cargo run -- add main.rs / commit "primer commit" / log
    let commands: Vec<&[String]> = args[1..]
        .split(|a| a == "/")
        .collect();

    for cmd_args in commands {
        let full = std::iter::once(&args[0])
            .chain(cmd_args.iter())
            .cloned()
            .collect::<Vec<String>>();

        match command::parse(&full) {
            Ok(cmd) => match cmd {
                command::Command::Init       => println!("Repositorio inicializado."),
                command::Command::Add(f)     => repo.add(f),
                command::Command::Commit(m)  => repo.commit(m),
                command::Command::Log        => repo.log(),
            },
            Err(e) => eprintln!("Error: {}", e),
        }
    }
}