use std::collections::HashMap;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};

use crate::graph::Grafo;
use crate::models::{Cliente, Produto};
use crate::recommendation::recomendar_produtos;

pub fn iniciar_servidor(
    grafo: Grafo,
    clientes: Vec<Cliente>,
    produtos: Vec<Produto>,
) {
    let listener = TcpListener::bind("127.0.0.1:8080")
        .expect("Não foi possível iniciar o servidor.");

    println!();
    println!("╔══════════════════════════════════════╗");
    println!("║          CONECTASTORE WEB            ║");
    println!("╚══════════════════════════════════════╝");
    println!();
    println!("Sistema disponível em:");
    println!("http://127.0.0.1:8080");
    println!();
    println!("Pressione Ctrl+C para encerrar.");
    println!();

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                tratar_requisicao(stream, &grafo, &clientes, &produtos);
            }
            Err(erro) => {
                eprintln!("Erro na conexão: {}", erro);
            }
        }
    }
}

fn tratar_requisicao(
    mut stream: TcpStream,
    grafo: &Grafo,
    clientes: &[Cliente],
    produtos: &[Produto],
) {
    let mut buffer = [0; 8192];

    if stream.read(&mut buffer).is_err() {
        return;
    }

    let requisicao = String::from_utf8_lossy(&buffer);

    let primeira_linha = match requisicao.lines().next() {
        Some(linha) => linha,
        None => return,
    };

    let partes: Vec<&str> = primeira_linha.split_whitespace().collect();

    if partes.len() < 2 {
        return;
    }

    let caminho = partes[1];

    if caminho.starts_with("/api/recomendacoes") {
        responder_recomendacoes(
            &mut stream,
            caminho,
            grafo,
            clientes,
            produtos,
        );
    } else {
        responder_html(&mut stream, grafo, clientes, produtos);
    }
}

fn responder_recomendacoes(
    stream: &mut TcpStream,
    caminho: &str,
    grafo: &Grafo,
    _clientes: &[Cliente],
    produtos: &[Produto],
) {
    let cliente_id = extrair_parametro(caminho, "cliente")
        .and_then(|valor| valor.parse::<u32>().ok())
        .unwrap_or(1);

    let recomendacoes = recomendar_produtos(grafo, cliente_id, 5);

    let nomes: Vec<String> = recomendacoes
        .iter()
        .filter_map(|id| {
            produtos
                .iter()
                .find(|produto| produto.id == *id)
                .map(|produto| {
                    format!(
                        "{{\"id\":{},\"nome\":\"{}\",\"categoria\":\"{}\"}}",
                        produto.id,
                        escapar_json(&produto.nome),
                        escapar_json(&produto.categoria)
                    )
                })
        })
        .collect();

    let json = format!(
        "{{\"cliente\":{},\"recomendacoes\":[{}]}}",
        cliente_id,
        nomes.join(",")
    );

    responder_json(stream, &json);
}

fn extrair_parametro(caminho: &str, parametro: &str) -> Option<String> {
    let query = caminho.split('?').nth(1)?;

    for item in query.split('&') {
        let mut partes = item.split('=');

        let chave = partes.next()?;
        let valor = partes.next()?;

        if chave == parametro {
            return Some(valor.to_string());
        }
    }

    None
}

fn escapar_json(valor: &str) -> String {
    valor
        .replace('\\', "\\\\")
        .replace('"', "\\\"")
}

fn responder_json(stream: &mut TcpStream, json: &str) {
    let resposta = format!(
        "HTTP/1.1 200 OK\r\n\
         Content-Type: application/json; charset=utf-8\r\n\
         Content-Length: {}\r\n\
         Connection: close\r\n\
         \r\n\
         {}",
        json.len(),
        json
    );

    let _ = stream.write_all(resposta.as_bytes());
}

fn responder_html(
    stream: &mut TcpStream,
    grafo: &Grafo,
    clientes: &[Cliente],
    produtos: &[Produto],
) {
    let cliente_id = 1;

    let cliente = clientes
        .iter()
        .find(|cliente| cliente.id == cliente_id);

    let produtos_comprados = grafo.produtos_do_cliente(cliente_id);

    let produtos_comprados_html = produtos_comprados
        .iter()
        .filter_map(|id| {
            produtos
                .iter()
                .find(|produto| produto.id == *id)
        })
        .map(|produto| {
            format!(
                r#"
                <div class="product bought">
                    <div class="icon">◈</div>
                    <div>
                        <strong>{}</strong>
                        <span>{}</span>
                    </div>
                </div>
                "#,
                produto.nome,
                produto.categoria
            )
        })
        .collect::<Vec<_>>()
        .join("");

    let cliente_nome = cliente
        .map(|c| c.nome.as_str())
        .unwrap_or("Cliente");

    let html = format!(
        r#"<!DOCTYPE html>
<html lang="pt-BR">
<head>
<meta charset="UTF-8">
<meta name="viewport" content="width=device-width, initial-scale=1.0">
<title>ConectaStore</title>

<style>

* {{
    box-sizing: border-box;
}}

body {{
    margin: 0;
    font-family: Arial, Helvetica, sans-serif;
    background: #f4f7fb;
    color: #172033;
}}

.header {{
    background: linear-gradient(135deg, #172554, #2563eb);
    color: white;
    padding: 28px 42px;
}}

.header-content {{
    max-width: 1200px;
    margin: auto;
}}

.logo {{
    font-size: 28px;
    font-weight: 800;
}}

.subtitle {{
    margin-top: 6px;
    opacity: .8;
}}

.container {{
    max-width: 1200px;
    margin: 30px auto;
    padding: 0 20px;
}}

.grid {{
    display: grid;
    grid-template-columns: 1fr 1fr;
    gap: 22px;
}}

.card {{
    background: white;
    border-radius: 18px;
    padding: 24px;
    box-shadow: 0 8px 30px rgba(20,40,80,.08);
}}

.card h2 {{
    margin-top: 0;
}}

select {{
    width: 100%;
    padding: 14px;
    border: 1px solid #d8deea;
    border-radius: 10px;
    font-size: 16px;
}}

button {{
    width: 100%;
    margin-top: 16px;
    padding: 15px;
    border: 0;
    border-radius: 10px;
    background: #2563eb;
    color: white;
    font-size: 16px;
    font-weight: 700;
    cursor: pointer;
}}

button:hover {{
    background: #1d4ed8;
}}

.products {{
    display: grid;
    grid-template-columns: repeat(2, 1fr);
    gap: 12px;
}}

.product {{
    padding: 15px;
    border-radius: 12px;
    background: #f7f9fc;
    display: flex;
    align-items: center;
    gap: 12px;
}}

.product.recommendation {{
    background: linear-gradient(135deg, #eff6ff, #eef2ff);
    border: 1px solid #c7d2fe;
}}

.icon {{
    width: 42px;
    height: 42px;
    display: flex;
    align-items: center;
    justify-content: center;
    border-radius: 10px;
    background: #dbeafe;
    font-size: 20px;
}}

.product span {{
    display: block;
    margin-top: 4px;
    color: #64748b;
    font-size: 13px;
}}

.badge {{
    display: inline-block;
    padding: 6px 10px;
    background: #dcfce7;
    color: #166534;
    border-radius: 20px;
    font-size: 12px;
    font-weight: 700;
}}

.graph {{
    margin-top: 22px;
    text-align: center;
    padding: 25px;
    background: #f8fafc;
    border-radius: 15px;
}}

.node {{
    display: inline-block;
    padding: 12px 18px;
    margin: 7px;
    border-radius: 30px;
    font-weight: 700;
}}

.client {{
    background: #dbeafe;
    color: #1e40af;
}}

.product-node {{
    background: #dcfce7;
    color: #166534;
}}

.arrow {{
    color: #94a3b8;
    font-weight: bold;
}}

.stats {{
    display: grid;
    grid-template-columns: repeat(3, 1fr);
    gap: 12px;
    margin-top: 22px;
}}

.stat {{
    padding: 18px;
    background: #f8fafc;
    border-radius: 12px;
}}

.stat strong {{
    display: block;
    font-size: 24px;
    margin-top: 5px;
}}

.full {{
    grid-column: 1 / -1;
}}

.loading {{
    text-align: center;
    padding: 20px;
    color: #64748b;
}}

@media(max-width: 800px) {{
    .grid {{
        grid-template-columns: 1fr;
    }}

    .products {{
        grid-template-columns: 1fr;
    }}
}}

</style>
</head>

<body>

<div class="header">
    <div class="header-content">
        <div class="logo">◈ CONECTASTORE</div>
        <div class="subtitle">
            Sistema inteligente de recomendação baseado em grafos
        </div>
    </div>
</div>

<div class="container">

<div class="grid">

<div class="card">

    <span class="badge">● SISTEMA ONLINE</span>

    <h2>Cliente</h2>

    <select id="cliente">
        {}

    </select>

    <button onclick="gerarRecomendacoes()">
        ✦ Gerar recomendações
    </button>

    <h3>Histórico de compras</h3>

    <div class="products">
        {}
    </div>

</div>

<div class="card">

    <h2>Recomendações</h2>

    <div id="resultado" class="loading">
        Selecione um cliente e gere as recomendações.
    </div>

</div>

<div class="card full">

    <h2>Mapa de conexões</h2>

    <p>
        O algoritmo percorre as conexões entre clientes e produtos
        para encontrar itens que ainda não foram comprados.
    </p>

    <div class="graph">

        <div>
            <span class="node client">Camilla</span>
            <span class="arrow">→</span>
            <span class="node product-node">Notebook</span>
            <span class="arrow">→</span>
            <span class="node client">João</span>
        </div>

        <div>
            <span class="node client">Camilla</span>
            <span class="arrow">→</span>
            <span class="node product-node">Mouse</span>
        </div>

        <div>
            <span class="node client">João</span>
            <span class="arrow">→</span>
            <span class="node product-node">Teclado</span>
            <span class="arrow">→</span>
            <span class="node client">Maria</span>
        </div>

        <div>
            <span class="node client">Maria</span>
            <span class="arrow">→</span>
            <span class="node product-node">Monitor</span>
        </div>

    </div>

</div>

</div>

<div class="stats">

    <div class="stat">
        Clientes
        <strong>{}</strong>
    </div>

    <div class="stat">
        Produtos
        <strong>{}</strong>
    </div>

    <div class="stat">
        Estrutura
        <strong>Grafo</strong>
    </div>

</div>

</div>

<script>

async function gerarRecomendacoes() {{

    const cliente = document.getElementById("cliente").value;

    const resultado = document.getElementById("resultado");

    resultado.innerHTML = "Calculando recomendações...";

    try {{

        const resposta = await fetch(
            "/api/recomendacoes?cliente=" + cliente
        );

        const dados = await resposta.json();

        if (dados.recomendacoes.length === 0) {{
            resultado.innerHTML =
                "<div class='loading'>Nenhuma recomendação encontrada.</div>";
            return;
        }}

        resultado.innerHTML =
            "<div class='products'>" +
            dados.recomendacoes.map(produto => `
                <div class="product recommendation">
                    <div class="icon">★</div>
                    <div>
                        <strong>${{produto.nome}}</strong>
                        <span>${{produto.categoria}}</span>
                    </div>
                </div>
            `).join("") +
            "</div>";

    }} catch (erro) {{

        resultado.innerHTML =
            "<div class='loading'>Erro ao gerar recomendações.</div>";

    }}
}}

</script>

</body>
</html>"#,
        clientes
            .iter()
            .map(|c| {
                format!(
                    r#"<option value="{}">{}</option>"#,
                    c.id, c.nome
                )
            })
            .collect::<Vec<_>>()
            .join(""),
        produtos_comprados_html,
        clientes.len(),
        produtos.len()
    );

    let resposta = format!(
        "HTTP/1.1 200 OK\r\n\
         Content-Type: text/html; charset=utf-8\r\n\
         Content-Length: {}\r\n\
         Connection: close\r\n\
         \r\n\
         {}",
        html.len(),
        html
    );

    let _ = stream.write_all(resposta.as_bytes());
}
