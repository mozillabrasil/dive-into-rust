struct Bicicleta {
    n_marchas : u32,
    marcha : u32,
    andando : bool,
}

impl Bicicleta {
    fn new(n_marchas: u32) -> Bicicleta {
        Bicicleta { n_marchas, marcha: 1, andando: false }
    }
    
    fn din_don(&self) {
        if self.andando {
            print!("din don! A bicicleta está andado");
        } else {
            print!("din don! A bicicleta está parada");
        }
    }

    fn parar(&mut self) {
        self.marcha = 1;
        self.andando = false;
    }

    fn sobe_marcha(&mut self, sobe: u32) {
        if self.andando {
            self.marcha += sobe;
            if self.marcha > self.n_marchas {
                self.marcha = self.n_marchas;
            }
        }
    }
}

fn main() {
    let mut bike = Bicicleta { n_marchas: 5, marcha: 1, andando: false };
    bike.andando = true;

    let mut bike2 = Bicicleta::new(5);
    bike2.andando = false;

    bike.din_don();
    bike2.din_don();

    bike.parar();
    bike.sobe_marcha(2);
}