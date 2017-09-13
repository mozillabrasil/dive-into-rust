enum Tipo {
    Normal,
    Mountain,
    Bmx,
    Speed,
    Eletrica(u32), // potencia em watts
}

impl Tipo {
    fn radical(&self) -> bool {
        use Tipo::*;
        match *self {
            Normal | Speed => false,
            Mountain | Bmx => true,
            Eletrica(w) => w > 2000
        }
    }
}

struct Bicicleta {
    n_marchas : u32,
    marcha : u32,
    andando : bool,
    tipo : Tipo
}

impl Bicicleta {
    fn new(n_marchas: u32) -> Bicicleta {
        Bicicleta { n_marchas, marcha: 1, andando: false, tipo: Tipo::Normal }
    }

    fn eletrica(n_marchas: u32, watts: u32) -> Bicicleta {
        Bicicleta { tipo: Tipo::Eletrica(watts), ..Bicicleta::new(n_marchas) }
    }

    fn radical(&self) -> bool {
        self.tipo.radical();
        // Ou
        use Tipo::*;
        match self.tipo {
            Normal | Speed => false,
            Mountain | Bmx => true,
            Eletrica(w) => w > 2000
        }
    }
}

fn main() {
    let bike_eletrica = Bicicleta::eletrica(5, 1000);
    println!("Essa bike é radical? {}", bike_eletrica.radical());
}