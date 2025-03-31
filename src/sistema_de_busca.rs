// Usando a importação correta para acessar os módulos da aplicação principal
use super::produto::Produto;
use super::sistema_de_busca::SistemaDeBusca;

#[test]
fn test_criar_produto() {
    // Testa a criação de um produto
    let produto = Produto::new("Smartphone", "Eletrônicos", 1499.99);
    
    assert_eq!(produto.nome, "Smartphone");
    assert_eq!(produto.categoria, "Eletrônicos");
    assert_eq!(produto.preco, 1499.99);
}

#[test]
fn test_adicionar_produto() {
    // Testa adicionar produto ao sistema de busca
    let mut sistema = SistemaDeBusca::new();
    let produto = Produto::new("Smartphone", "Eletrônicos", 1499.99);
    
    sistema.adicionar_produto(produto);
    
    let produto_encontrado = sistema.buscar_produto("Smartphone");
    
    assert!(produto_encontrado.is_some()); // Verifica se o produto foi encontrado
    assert_eq!(produto_encontrado.unwrap().nome, "Smartphone");
}

#[test]
fn test_buscar_produto_inexistente() {
    // Testa buscar um produto que não existe no sistema de busca
    let sistema = SistemaDeBusca::new();
    
    let produto_encontrado = sistema.buscar_produto("ProdutoInexistente");
    
    assert!(produto_encontrado.is_none()); // Verifica que o produto não foi encontrado
}
