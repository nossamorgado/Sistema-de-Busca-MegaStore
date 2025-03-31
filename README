# Sistema de Busca de Produtos

Este projeto é um sistema de busca eficiente para a MegaStore, permitindo que os usuários encontrem rapidamente produtos disponíveis na loja. O sistema é projetado para lidar com grandes volumes de dados e realizar buscas rápidas por nome e categoria de produtos.

## Funcionalidades

- Buscar produtos por nome.
- Buscar produtos por categoria.

## Tecnologias Utilizadas

- **Rust**: Linguagem de programação utilizada para desenvolver o sistema.
- **HashMap**: Estrutura de dados usada para otimizar as buscas por nome e categoria de produtos.

## Como Usar

1. Compile e execute o projeto com o comando:
    ```bash
    cargo run
    ```

2. Adicione produtos ao sistema:
    - Crie instâncias de `Produto`.
    - Utilize o método `adicionar_produto` para adicionar ao sistema.

3. Realize buscas:
    - `buscar_por_nome("Nome do Produto")`
    - `buscar_por_categoria("Categoria do Produto")`

## Exemplo de Uso

Para adicionar um produto ao sistema, crie um produto e adicione-o:

```rust
let produto = Produto::new("Nome do Produto", "Categoria X", 100.0);
sistema.adicionar_produto(produto);
Para buscar um produto por nome:

rust
Copiar
let resultado = sistema.buscar_por_nome("Nome do Produto");
println!("{:?}", resultado);
E para buscar por categoria:

rust
Copiar
let resultado = sistema.buscar_por_categoria("Categoria X");
println!("{:?}", resultado);

Este projeto implementa um sistema básico de busca de produtos, utilizando HashMap para otimizar as pesquisas. Algumas melhorias futuras podem incluir:

Implementação de buscas mais complexas (por preço, avaliação, etc.).

Suporte a buscas com expressão regular.

Testes de desempenho em grande escala.
