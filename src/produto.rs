use std::fmt;

#[derive(Debug)]
pub struct Produto {
    nome: String,
    categoria: String,
    preco: f64,
}

impl Produto {
    pub fn new(nome: &str, categoria: &str, preco: f64) -> Self {
        Produto {
            nome: nome.to_string(),
            categoria: categoria.to_string(),
            preco,
        }
    }
}

impl fmt::Display for Produto {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} - {} - R${:.2}", self.nome, self.categoria, self.preco)
    }
}
