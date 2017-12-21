struct Bicicleta {
    marcha : u32,
    andando : bool,
}

impl Bicicleta {
    fn new() -> Bicicleta {
        Bicicleta { marcha: 1, andando: false }
    }
    
    fn din_don(&self) {
        if self.andando {
            print!("din don! A bicicleta está andado");
        } else {
            print!("din don! A bicicleta está parada");
        }
    }

    fn sobe_marcha(&mut self, sobe: u32) {
        if self.andando {
            self.marcha += sobe;
        }
    }
}

fn main() {
    let mut bike = Bicicleta { marcha: 1, andando: false };
    bike.andando = true;

    let mut bike2 = Bicicleta::new();
    bike2.andando = false;

    bike.din_don();
    bike2.din_don();

    bike.sobe_marcha(2);
    println!("marcha da bike {}", bike.marcha);
}