fn main() {
    let um = 1;
    let mentira = false;
    println!("Um é: {}", um);
    println!("Olha a cobra: {}", mentira);

    let dois: i32 = 2;
    let verdade: bool = true;
    println!("Um mais dois é: {}", um + dois);
    println!("verdade é verdadeiro: {}", verdade == true);

    let n = 10;
    if n % 2 == 0 {
        println!("é par");
    } else {
        println!("é ímpar");
    }

    let mut mensagem;
    if n % 2 == 0 {
        mensagem = "é par";
    } else {
        mensagem = "é impar";
    }
    println!("o numero {} é {}", n, mensagem);
    
    mensagem = if n % 2 == 0 {
        "é par"
    } else {
        "é impar"
    };
    
    println!("o numero {} é {}", n, mensagem);
    println!("o numero {} é {}", n, par(n));
}

fn par(n : i32) -> &'static str {
    if n % 2 == 0 {
        "é par"
    } else {
        "é impar"
    }
}
