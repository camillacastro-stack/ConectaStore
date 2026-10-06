use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Produto {
    pub id: u32,
    pub nome: String,
    pub categoria: String,
}

#[derive(Debug, Clone)]
pub struct Cliente {
    pub id: u32,
    pub nome: String,
}

pub type Produtos = HashMap<u32, Produto>;
pub type Clientes = HashMap<u32, Cliente>;
