use std::collections::{HashMap, HashSet};

#[derive(Debug)]
pub struct Grafo {
    pub clientes: HashMap<u32, HashSet<u32>>,
    pub produtos: HashMap<u32, HashSet<u32>>,
}

impl Grafo {
    pub fn novo() -> Self {
        Self {
            clientes: HashMap::new(),
            produtos: HashMap::new(),
        }
    }

    pub fn adicionar_cliente(&mut self, cliente_id: u32) {
        self.clientes.entry(cliente_id).or_default();
    }

    pub fn adicionar_produto(&mut self, produto_id: u32) {
        self.produtos.entry(produto_id).or_default();
    }

    pub fn adicionar_compra(&mut self, cliente_id: u32, produto_id: u32) {
        self.clientes
            .entry(cliente_id)
            .or_default()
            .insert(produto_id);

        self.produtos
            .entry(produto_id)
            .or_default()
            .insert(cliente_id);
    }

    pub fn produtos_do_cliente(&self, cliente_id: u32) -> Vec<u32> {
        self.clientes
            .get(&cliente_id)
            .map(|produtos| produtos.iter().copied().collect())
            .unwrap_or_default()
    }

    pub fn clientes_do_produto(&self, produto_id: u32) -> Vec<u32> {
        self.produtos
            .get(&produto_id)
            .map(|clientes| clientes.iter().copied().collect())
            .unwrap_or_default()
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deve_adicionar_compra() {
        let mut grafo = Grafo::novo();

        grafo.adicionar_cliente(1);
        grafo.adicionar_produto(100);
        grafo.adicionar_compra(1, 100);

        let produtos = grafo.produtos_do_cliente(1);

        assert_eq!(produtos.len(), 1);
        assert_eq!(produtos[0], 100);
    }

    #[test]
    fn deve_retornar_clientes_do_produto() {
        let mut grafo = Grafo::novo();

        grafo.adicionar_compra(1, 100);
        grafo.adicionar_compra(2, 100);

        let clientes = grafo.clientes_do_produto(100);

        assert_eq!(clientes.len(), 2);
        assert!(clientes.contains(&1));
        assert!(clientes.contains(&2));
    }
}
