mod produto; // Adiciona o módulo produto

use produto::Produto;

fn main() {
    let produto1 = Produto::new("Smartphone", "Eletrônicos", 1499.99);
    let produto2 = Produto::new("Tênis", "Calçados", 299.99);
    let produto3 = Produto::new("Camiseta", "Vestuário", 79.90);

    let produtos = vec![produto1, produto2, produto3];

    for produto in produtos {
        println!("{}", produto);
    }
}
