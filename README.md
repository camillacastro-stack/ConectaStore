# ConectaStore

Sistema de recomendação de produtos desenvolvido em **Rust**, utilizando estruturas de dados e algoritmos de grafos para identificar produtos que podem ser relevantes para clientes de uma plataforma de e-commerce.

---

## 1. Objetivo do projeto

O ConectaStore foi desenvolvido a partir do cenário da MegaStore, uma plataforma de comércio eletrônico que deseja melhorar suas recomendações de produtos.

O objetivo é utilizar o histórico de compras dos clientes para identificar relações entre clientes e produtos e, a partir dessas relações, gerar recomendações personalizadas.

O projeto demonstra a aplicação prática de:

- Grafos;
- Lista de adjacência;
- Busca em largura (BFS);
- `HashMap`;
- `HashSet`;
- `VecDeque`;
- Testes unitários;
- Testes de integração;
- Benchmark de desempenho.

---

## 2. Funcionalidades

O sistema possui as seguintes funcionalidades:

### Cadastro de clientes

Permite representar clientes por meio de um identificador e nome.

Exemplo:

```text
Cliente:
ID: 1
Nome: Camilla
```

### Cadastro de produtos

Os produtos possuem:

- ID;
- Nome;
- Categoria.

Exemplo:

```text
Produto:
ID: 101
Nome: Notebook
Categoria: Eletrônicos
```

### Registro de compras

O sistema registra a relação entre clientes e produtos.

Exemplo:

```text
Camilla → Notebook
Camilla → Mouse

João → Notebook
João → Teclado

Maria → Teclado
Maria → Monitor
```

### Recomendação de produtos

O sistema utiliza o histórico de compras para encontrar outros clientes conectados ao cliente consultado e identificar produtos que esses clientes compraram.

Produtos que o cliente já possui não são recomendados.

### Prevenção de duplicidade

O sistema utiliza `HashSet` para impedir que o mesmo produto apareça mais de uma vez nas recomendações.

---

## 3. Modelo do grafo

O ConectaStore utiliza um **grafo bipartido**.

Existem dois tipos de vértices:

- Cliente;
- Produto.

As arestas representam uma relação de compra.

### Exemplo

```text
          Cliente
            │
            │ comprou
            ▼
         Produto
            │
            │ comprado por
            ▼
          Cliente
```

No exemplo utilizado:

```text
Camilla ─── Notebook
   │
   └────── Mouse

João ───── Notebook
   │
   └────── Teclado

Maria ──── Teclado
   │
   └────── Monitor
```

Dessa forma, o sistema consegue descobrir produtos relacionados por meio de outros clientes.

---

## 4. Estruturas de dados utilizadas

### HashMap

O `HashMap` é utilizado para armazenar e localizar clientes e produtos por seus identificadores.

A estrutura permite acesso rápido aos elementos, com complexidade média próxima de:

```text
O(1)
```

### HashSet

O `HashSet` é utilizado para:

- controlar clientes visitados;
- evitar produtos duplicados;
- armazenar relações sem duplicidade.

### VecDeque

O `VecDeque` é utilizado como fila durante a execução do algoritmo BFS.

A fila permite inserir elementos no final e removê-los do início de forma eficiente.

### Lista de adjacência

O grafo é representado utilizando estruturas de adjacência.

Essa abordagem é adequada para o problema porque o grafo possui relativamente poucas conexões em comparação com todas as possíveis combinações entre clientes e produtos.

---

## 5. Algoritmo de recomendação

O algoritmo utilizado é o **BFS (Breadth-First Search)**, ou busca em largura.

O processo pode ser representado da seguinte forma:

```text
Cliente
   ↓
Produtos comprados
   ↓
Outros clientes
   ↓
Produtos comprados por esses clientes
   ↓
Recomendações
```

### Exemplo

Camilla comprou:

```text
Notebook
Mouse
```

João também comprou:

```text
Notebook
Teclado
```

O sistema identifica que Camilla e João possuem uma conexão através do produto Notebook.

Como João comprou Teclado e Camilla ainda não comprou esse produto, o sistema recomenda:

```text
Teclado
```

---

## 6. Exemplo de execução

Ao executar o programa:

```powershell
cargo run
```

o sistema apresenta:

```text
=== CONECTASTORE ===
Cliente: Camilla

Produtos recomendados:
- Teclado | Categoria: Eletrônicos
- Monitor | Categoria: Eletrônicos
```

O resultado demonstra que o sistema conseguiu identificar produtos relacionados ao histórico de compras da cliente.

---

## 7. Testes

O projeto possui testes unitários e testes de integração.

Para executar todos os testes:

```powershell
cargo test
```

Os testes verificam:

- inclusão de compras;
- recuperação dos produtos de um cliente;
- recuperação dos clientes de um produto;
- recomendação de produtos;
- exclusão de produtos já comprados;
- prevenção de recomendações duplicadas.

### Resultado dos testes

```text
2 testes unitários passaram
2 testes de integração passaram
0 testes falharam
```

---

## 8. Benchmark de desempenho

Foi implementado um benchmark simples utilizando `std::time::Instant`.

Foram realizados testes com diferentes volumes de dados:

| Volume | Tempo |
|---:|---:|
| 100 | 38,9 µs |
| 500 | 38,2 µs |
| 1.000 | 46,9 µs |
| 5.000 | 45,3 µs |

### Análise

Nos testes realizados, o tempo de execução permaneceu na faixa de aproximadamente **38 a 47 microssegundos**, mesmo com o aumento do volume de dados.

Os resultados demonstram bom desempenho para o cenário utilizado no projeto.

Entretanto, como o benchmark foi executado com volumes relativamente pequenos, os resultados não permitem afirmar que o comportamento será exatamente o mesmo em bases muito maiores.

O desempenho pode ser influenciado pelo hardware, sistema operacional e condições de execução.

---

## 9. Complexidade

Considerando a representação do grafo por lista de adjacência, o algoritmo BFS possui complexidade aproximada de:

```text
O(V + E)
```

Onde:

- `V` = quantidade de vértices;
- `E` = quantidade de arestas.

A representação por lista de adjacência utiliza aproximadamente:

```text
O(V + E)
```

de espaço.

As operações médias de consulta e inserção em `HashMap` e `HashSet` possuem complexidade aproximada de:

```text
O(1)
```

---

## 10. Lista de adjacência x matriz de adjacência

Uma matriz de adjacência exigiria espaço proporcional a:

```text
O(V²)
```

Isso pode se tornar muito custoso quando o número de clientes e produtos cresce.

Por esse motivo, o projeto utiliza lista de adjacência.

A lista de adjacência armazena apenas as conexões existentes, sendo mais adequada para um grafo esparso.

### Comparação

| Característica | Lista de adjacência | Matriz de adjacência |
|---|---|---|
| Memória | O(V + E) | O(V²) |
| Grafos esparsos | Excelente | Menos eficiente |
| Representação das conexões | Apenas existentes | Todas as possibilidades |
| Escalabilidade | Melhor para grandes grafos esparsos | Pode consumir muita memória |

---

## 11. Arquitetura do projeto

O projeto foi organizado da seguinte forma:

```text
ConectaStore/
│
├── Cargo.toml
├── README.md
├── RELATORIO.md
├── ROTEIRO_VIDEO.md
├── .gitignore
│
├── src/
│   ├── main.rs
│   ├── lib.rs
│   ├── models.rs
│   ├── graph.rs
│   ├── recommendation.rs
│   └── benchmark.rs
│
└── tests/
    └── integracao.rs
```

### Responsabilidade dos arquivos

**`main.rs`**

Executa o programa e apresenta o exemplo de recomendação e o benchmark.

**`models.rs`**

Contém as estruturas de dados:

- `Cliente`;
- `Produto`.

**`graph.rs`**

Responsável pela construção e gerenciamento do grafo.

**`recommendation.rs`**

Contém o algoritmo de recomendação utilizando BFS.

**`benchmark.rs`**

Executa os testes de desempenho com diferentes volumes de dados.

**`tests/integracao.rs`**

Contém os testes de integração do sistema.

---

## 12. Tecnologias utilizadas

- **Rust**
- Cargo
- Estruturas de dados
- Algoritmos de grafos
- BFS
- HashMap
- HashSet
- VecDeque

---

## 13. Como executar

### Pré-requisito

Ter o Rust instalado.

Verifique com:

```powershell
rustc --version
```

e:

```powershell
cargo --version
```

### Compilar

Dentro da pasta do projeto:

```powershell
cargo build
```

### Verificar o projeto

```powershell
cargo check
```

### Executar

```powershell
cargo run
```

### Executar os testes

```powershell
cargo test
```

---

## 14. Exemplo completo do fluxo

O sistema utiliza o seguinte fluxo:

```text
                    ┌─────────────┐
                    │   Camilla   │
                    └──────┬──────┘
                           │
                  ┌────────┴────────┐
                  ▼                 ▼
             Notebook            Mouse
                  │
                  │
                  ▼
                João
                  │
                  ▼
               Teclado
                  │
                  ▼
              Recomendação
```

O algoritmo identifica a relação entre clientes por meio dos produtos compartilhados.

Assim, produtos adquiridos por clientes relacionados podem ser utilizados como recomendações.

---

## 15. Benefícios da solução

A solução apresenta alguns benefícios:

- utilização eficiente de estruturas de dados;
- recomendação baseada em histórico real de compras;
- prevenção de recomendações duplicadas;
- exclusão de produtos que o cliente já possui;
- possibilidade de expansão para grandes volumes;
- separação do código em módulos;
- testes automatizados;
- medição de desempenho.

---

## 16. Possibilidades de evolução

O projeto pode ser expandido futuramente para considerar:

- frequência de compras;
- categorias de produtos;
- valor das compras;
- avaliações dos clientes;
- pesos nas arestas do grafo;
- recomendações personalizadas por perfil;
- maior profundidade de busca;
- ranking de recomendações;
- banco de dados real;
- API para integração com uma plataforma de e-commerce.

Uma possível evolução seria atribuir pesos às conexões para priorizar produtos que possuem maior relação com o histórico do cliente.

---

## 17. Conclusão

O ConectaStore demonstra como estruturas de dados e algoritmos de grafos podem ser aplicados em um problema real de recomendação de produtos.

A utilização de um grafo bipartido permite representar as relações entre clientes e produtos, enquanto o algoritmo BFS possibilita percorrer essas conexões para encontrar novas possibilidades de recomendação.

A combinação de `HashMap`, `HashSet`, `VecDeque` e lista de adjacência proporciona uma estrutura adequada para o problema, permitindo consultas eficientes e controle das recomendações.

Os testes automatizados confirmaram o funcionamento das principais funcionalidades e o benchmark demonstrou tempos de execução reduzidos nos volumes avaliados.

---

## 18. Vídeo Pitch

**Link do vídeo:**  
> COLOCAR_LINK_DO_VIDEO_AQUI

O vídeo deve apresentar:

1. O problema da MegaStore;
2. O modelo do grafo;
3. As estruturas de dados utilizadas;
4. O funcionamento do sistema;
5. A recomendação de produtos;
6. Os testes realizados;
7. Os resultados do benchmark;
8. A possibilidade de escalabilidade da solução.

---

## 19. Repositório

**GitHub:**  
https://github.com/camillacastro-stack/ConectaStore/

---

## 20. Autoria

**Projeto:** ConectaStore  
**Linguagem:** Rust  
**Área:** Estruturas de Dados e Estratégias de Implementação
