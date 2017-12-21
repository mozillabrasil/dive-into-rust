struct Pessoa {
    nome: String,
    estado_civil: Status,
    escolaridade: Escolaridade    
}

enum Status {
    Solteiro,
    Separado,
    Casado(String),
}

// Exercicio 1: Defina o enum escolaridade.

impl Pessoa {
    fn conjuge(&self) -> Option<String> {
        match self.estado_civil {
            Status::Casado(ref conjuge) => Some(conjuge.clone()),
            _ => None
        }
    }
    
    // Exercicio 2: Defina um metodo que retorna bool
    //              indicando se a pessoa possui ensino medio.
}

fn main() {
    // Exercicio 3: Crie uma pessoa casada.
}