fn main() {
    let x = 1;
    let mut y = x;
    y = y + 1;
    println!("x continua {}, y agora é {}", x, y);

    let s = String::from("oi");
    let a = s.clone(); // tente tirar clone
    grita(s); // tente duplicar essa linha
    grita(a);

    let mut t = String::from("tchau");
    grita_ref(&t);
    grita_ref(&t);

    grita_educadamente(&mut t);
    println!("{}", t);

    let x : &str = "estatico";
    let pedaco : &str = &t[1..5];
    println!("{}", pedaco);

    // grita_educadamente(&mut t);
    // grita(t);
}

fn grita_educadamente(string : &mut String) {
    string.push_str("!");
}

fn grita(string : String) {
    println!("{}", string.to_uppercase());
}

fn grita_ref(string : &String) {
    println!("{}", string.to_uppercase());
}
