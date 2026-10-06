use std::collections::HashMap;

use ConectaStore::benchmark::executar_benchmark;
use ConectaStore::graph::Grafo;
use ConectaStore::models::{Cliente, Produto};
use ConectaStore::recommendation::recomendar_produtos;

fn main() {
    // Cadastro de clientes utilizando HashMap.
    // A chave é o ID do cliente.
    let clientes: HashMap<u32, Cliente> = HashMap::from([
        (
            1,
            Cliente {
                id: 1,
                nome: "Camilla".to_string(),
            },
        ),
        (
            2,
            Cliente {
                id: 2,
                nome: "João".to_string(),
            },
        ),
        (
            3,
            Cliente {
                id: 3,
                nome: "Maria".to_string(),
            },
        ),
    ]);

    // Cadastro de produtos utilizando HashMap.
    // A chave é o ID do produto.
    let produtos: HashMap<u32, Produto> = HashMap::from([
        (
            101,
            Produto {
                id: 101,
                nome: "Notebook".to_string(),
                categoria: "Eletrônicos".to_string(),
            },
        ),
        (
            102,
            Produto {
                id: 102,
                nome: "Mouse".to_string(),
                categoria: "Eletrônicos".to_string(),
            },
        ),
        (
            103,
            Produto {
                id: 103,
                nome: "Teclado".to_string(),
                categoria: "Eletrônicos".to_string(),
            },
        ),
        (
            104,
            Produto {
                id: 104,
                nome: "Monitor".to_string(),
                categoria: "Eletrônicos".to_string(),
            },
        ),
    ]);

    // Criação do grafo.
    let mut grafo = Grafo::novo();

    // Adiciona os clientes ao grafo.
    for cliente_id in clientes.keys() {
        grafo.adicionar_cliente(*cliente_id);
    }

    // Adiciona os produtos ao grafo.
    for produto_id in produtos.keys() {
        grafo.adicionar_produto(*produto_id);
    }

    // Histórico de compras.
    //
    // Camilla comprou Notebook e Mouse.
    grafo.adicionar_compra(1, 101);
    grafo.adicionar_compra(1, 102);

    // João comprou Notebook e Teclado.
    grafo.adicionar_compra(2, 101);
    grafo.adicionar_compra(2, 103);

    // Maria comprou Teclado e Monitor.
    grafo.adicionar_compra(3, 103);
    grafo.adicionar_compra(3, 104);

    // Gera recomendações para a Camilla.
    let recomendacoes = recomendar_produtos(&grafo, 1, 5);

    println!("=== CONECTASTORE ===");
    println!("Cliente: {}", clientes[&1].nome);
    println!();

    println!("Produtos recomendados:");

    for produto_id in recomendacoes {
        if let Some(produto) = produtos.get(&produto_id) {
            println!(
                "- {} | Categoria: {}",
                produto.nome,
                produto.categoria
            );
        }
    }

    // Executa o benchmark.
    executar_benchmark();
}
