enum Tipo {
    Normal,
    Mountain,
    Bmx,
    Eletrica(u32), // potencia em watts
}

impl Tipo {
    fn radical(&self) -> bool {
        use Tipo::*;
        match *self {
            Normal => false,
            Mountain | Bmx => true,
            Eletrica(w) => w > 2000
        }
    }
}

struct Bicicleta {
    marcha : u32,
    andando : bool,
    tipo : Tipo
}

impl Bicicleta {
    fn new() -> Bicicleta {
        Bicicleta { marcha: 1, andando: false, tipo: Tipo::Normal }
    }

    fn eletrica(watts: u32) -> Bicicleta {
        Bicicleta { tipo: Tipo::Eletrica(watts), ..Bicicleta::new() }
    }

    fn radical(&self) -> bool {
        self.tipo.radical();
        // Ou
        use Tipo::*;
        match self.tipo {
            Normal => false,
            Mountain | Bmx => true,
            Eletrica(w) => w > 2000
        }
    }
}

fn main() {
    let bike_eletrica = Bicicleta::eletrica(1000);
    println!("Essa bike é radical? {}", bike_eletrica.radical());
}