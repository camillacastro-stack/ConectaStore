use ConectaStore::benchmark::executar_benchmark;
use ConectaStore::graph::Grafo;
use ConectaStore::models::{Cliente, Produto};
use ConectaStore::recommendation::recomendar_produtos;

fn main() {
    let clientes = vec![
        Cliente {
            id: 1,
            nome: "Camilla".to_string(),
        },
        Cliente {
            id: 2,
            nome: "João".to_string(),
        },
        Cliente {
            id: 3,
            nome: "Maria".to_string(),
        },
    ];

    let produtos = vec![
        Produto {
            id: 101,
            nome: "Notebook".to_string(),
            categoria: "Eletrônicos".to_string(),
        },
        Produto {
            id: 102,
            nome: "Mouse".to_string(),
            categoria: "Eletrônicos".to_string(),
        },
        Produto {
            id: 103,
            nome: "Teclado".to_string(),
            categoria: "Eletrônicos".to_string(),
        },
        Produto {
            id: 104,
            nome: "Monitor".to_string(),
            categoria: "Eletrônicos".to_string(),
        },
    ];

    let mut grafo = Grafo::novo();

    for cliente in &clientes {
        grafo.adicionar_cliente(cliente.id);
    }

    for produto in &produtos {
        grafo.adicionar_produto(produto.id);
    }

    // Histórico de compras
    grafo.adicionar_compra(1, 101); // Camilla comprou Notebook
    grafo.adicionar_compra(1, 102); // Camilla comprou Mouse

    grafo.adicionar_compra(2, 101); // João comprou Notebook
    grafo.adicionar_compra(2, 103); // João comprou Teclado

    grafo.adicionar_compra(3, 103); // Maria comprou Teclado
    grafo.adicionar_compra(3, 104); // Maria comprou Monitor

    let recomendacoes = recomendar_produtos(&grafo, 1, 5);

    println!("=== CONECTASTORE ===");
    println!("Cliente: Camilla");
    println!();

    println!("Produtos recomendados:");

    for produto_id in recomendacoes {
        if let Some(produto) = produtos.iter().find(|p| p.id == produto_id) {
            println!(
                "- {} | Categoria: {}",
                produto.nome,
                produto.categoria
            );
        }
    }

    // Executa o benchmark
    executar_benchmark();
}
