use ConectaStore::graph::Grafo;
use ConectaStore::recommendation::recomendar_produtos;

#[test]
fn deve_recomendar_produto_que_cliente_ainda_nao_comprou() {
    let mut grafo = Grafo::novo();

    // Camilla comprou Notebook
    grafo.adicionar_compra(1, 101);

    // João comprou Notebook e Teclado
    grafo.adicionar_compra(2, 101);
    grafo.adicionar_compra(2, 102);

    let recomendacoes = recomendar_produtos(&grafo, 1, 5);

    assert!(recomendacoes.contains(&102));
    assert!(!recomendacoes.contains(&101));
}

#[test]
fn nao_deve_gerar_recomendacoes_duplicadas() {
    let mut grafo = Grafo::novo();

    grafo.adicionar_compra(1, 101);

    grafo.adicionar_compra(2, 101);
    grafo.adicionar_compra(2, 102);

    grafo.adicionar_compra(3, 101);
    grafo.adicionar_compra(3, 102);

    let recomendacoes = recomendar_produtos(&grafo, 1, 5);

    let quantidade_102 = recomendacoes
        .iter()
        .filter(|&&id| id == 102)
        .count();

    assert_eq!(quantidade_102, 1);
}
