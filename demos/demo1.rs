fn main() {
    // Declaracao de variaveis.
    let um = 1;
    let mentira = false;
    println!("Um é: {}", um);
    println!("Olha a cobra: {}", mentira);

    // Tipos explicitos.
    let dois: i32 = 2;
    let verdade: bool = true;
    // Expressoes.
    println!("Um mais dois é: {}", um + dois);
    println!("verdade é verdadeiro: {}", verdade == true);

    // if
    let n = 10;
    if n % 2 == 0 {
        println!("é par");
    } else {
        println!("é ímpar");
    }

    // mut
    let mut mensagem;
    if n % 2 == 0 {
        mensagem = "é par";
    } else {
        mensagem = "é impar";
    }
    println!("o numero {} é {}", n, mensagem);
    
    // if como expressao
    mensagem = if n % 2 == 0 {
        "é par"
    } else {
        "é impar"
    };
    println!("o numero {} é {}", n, mensagem);

    // funcoes e return
    println!("o numero {} é {}", n, par(n));
}


fn par(n : i32) -> &'static str {
    if n % 2 == 0 {
        "é par"
    } else {
        "é impar"
    }
}
