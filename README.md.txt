# Sistema de Busca de Produtos

Este projeto é um sistema de busca para a MegaStore, permitindo pesquisar produtos por nome e categoria.

## Funcionalidades

- Buscar produtos por nome.
- Buscar produtos por categoria.

## Estruturas de Dados

Utilizamos um `HashMap` para armazenar produtos por nome e por categoria.

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
