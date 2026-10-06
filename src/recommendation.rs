use std::collections::{HashSet, VecDeque};

use crate::graph::Grafo;

pub fn recomendar_produtos(
    grafo: &Grafo,
    cliente_id: u32,
    limite: usize,
) -> Vec<u32> {
    let produtos_comprados: HashSet<u32> =
        grafo.produtos_do_cliente(cliente_id)
            .into_iter()
            .collect();

    let mut visitados_clientes = HashSet::new();
    let mut fila = VecDeque::new();
    let mut recomendacoes = Vec::new();
    let mut recomendados = HashSet::new();

    fila.push_back(cliente_id);
    visitados_clientes.insert(cliente_id);

    while let Some(cliente_atual) = fila.pop_front() {
        let produtos = grafo.produtos_do_cliente(cliente_atual);

        for produto_id in produtos {
            if !produtos_comprados.contains(&produto_id)
                && recomendados.insert(produto_id)
            {
                recomendacoes.push(produto_id);

                if recomendacoes.len() >= limite {
                    return recomendacoes;
                }
            }

            let clientes = grafo.clientes_do_produto(produto_id);

            for outro_cliente in clientes {
                if visitados_clientes.insert(outro_cliente) {
                    fila.push_back(outro_cliente);
                }
            }
        }
    }

    recomendacoes
}
