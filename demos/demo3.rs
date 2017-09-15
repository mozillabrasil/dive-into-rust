fn main() {
    // Tipos simples são copiados.
    let x = 1;
    let mut y = x;
    y += 1;
    println!("x continua {}, y agora é {}", x, y);

    // Uso após move, clone.
    let s = "oi".to_string();
    let a = s.clone(); // tente tirar clone
    grita(s); // tente duplicar essa linha
    grita(a);

    // Uso compartilhado (empréstimo, borrow, referência).
    let mut t = "tchau".to_string();
    grita_ref(&t);
    grita_ref(&t);

    // Referência exclusiva.
    grita_educadamente(&mut t);
    println!("{}", t);

    // Slicing.
    let x : &str = "estatico";
    let pedaco : &str = &t[1..5];
    println!("{}", pedaco);

    // grita_educadamente(&mut t); // uso de valor emprestado.
    // grita(t); // uso de valor emprestado.
}

fn grita(string : String) {
    println!("{}", string.to_uppercase());
}

fn grita_ref(string : &String) {
    println!("{}", string.to_uppercase());
}

fn grita_educadamente(string : &mut String) {
    string.push_str("!");
}
