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