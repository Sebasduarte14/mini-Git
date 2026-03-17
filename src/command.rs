
#[derive(Debug)]
pub enum Command {
    Init,
    Add(String),
    Commit(String),
    Log,
}

pub fn parse(args: &[String]) -> Result<Command,String> {
    // arg[0] es el binario, args[1] es el comando
    // Ejemplo: ["mini-git", "add", "archivo.txt"]
    
    let subcomand = args.get(1).ok_or("No se proporciono un comando".to_string())?;

    match subcomand.as_str() {
        "init" => Ok(Command::Init),
        "log" => Ok(Command::Log),
        "add" => {
            let filename = args.get(2).ok_or("add requiere un nombre de archivo".to_string())?;
            Ok(Command::Add(filename.clone()))
        }
        "commit" => {
            let message = args.get(2).ok_or("commit requiere un mensaje".to_string())?;
            Ok(Command::Commit(message.clone()))
        }
        unknown => Err(format!("Comando desconocido {}",unknown)),
    }
}