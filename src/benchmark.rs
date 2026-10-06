use std::time::Instant;

use crate::graph::Grafo;
use crate::recommendation::recomendar_produtos;

pub fn executar_benchmark() {
    let volumes = [100, 500, 1000, 5000];

    println!("\n=== BENCHMARK ===");

    for volume in volumes {
        let mut grafo = Grafo::novo();

        for cliente_id in 1..=volume {
            grafo.adicionar_cliente(cliente_id);

            let produto_id = 100_000 + cliente_id;

            grafo.adicionar_produto(produto_id);
            grafo.adicionar_compra(cliente_id, produto_id);

            // Cria conexões entre clientes através de produtos
            if cliente_id > 1 {
                grafo.adicionar_compra(cliente_id, produto_id - 1);
            }
        }

        let inicio = Instant::now();

        let _ = recomendar_produtos(&grafo, 1, 10);

        let duracao = inicio.elapsed();

        println!(
            "Volume: {:>5} | Tempo: {:?}",
            volume,
            duracao
        );
    }
}
