
// Un blob representa el contenido de un archivo 

pub struct Blob{
    pub content: Vec<u8>,
}

// Un tree representa una carpeta 

pub struct TreeEntry{
    pub name: String,
    pub hash: String, 
}

pub struct Tree{
    pub entries: Vec<TreeEntry>,
}

// Un commit es un snapshot del proyecto en un momento dado

pub struct Commit{
    pub message: String,
    pub author: String,
    pub tree_hash: String,
    pub parent_hash: Option<String>,
}

// Enum unifica todo

pub enum GitObject{
    Blob(Blob),
    Tree(Tree),
    Commit(Commit),
}

use core::hash;
use std::collections::HashMap;

pub struct Repository{
    pub objects: HashMap<String,GitObject>,
    pub stanging: Vec<String>,
    pub head: Option<String>,
}

impl Repository {
    pub fn new() -> Repository {
        Repository { objects: HashMap::new(),
            stanging: Vec::new(), 
            head: None,
        }
    }
    pub fn add(&mut self, filename: String) {
        println!("Agregando {} al staging...",filename);
        self.stanging.push(filename);
    }
    pub  fn commit(&mut self, message: String) {
        if self.stanging.is_empty() {
            println!("Nada en el staging. Usa add primero.");
            return;
        }
        let hash = format!("{:x}",self.stanging.join("").len());

        let commit = Commit{
            message,
            author: "dev".to_string(),
            tree_hash: hash.clone(),
            parent_hash: self.head.clone()
        };
        

        self.objects.insert(hash.clone(), GitObject::Commit(commit));
        self.head = Some(hash.clone());
        self.stanging.clear();

        println!("Commit creado: {}", hash);
    }
    pub fn log(&self) {
    match &self.head {
        None => println!("No hay commits aún."),
        Some(hash) => {
            if let GitObject::Commit(c) = &self.objects[hash] {
                println!("Commit: {}", hash);
                println!("  Autor:   {}", c.author);
                println!("  Mensaje: {}", c.message);
                }
            }
        }   
    }
}