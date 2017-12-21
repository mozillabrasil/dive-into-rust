fn main() {
    // Declaracao de variaveis.
    let um = 1;
    let mentira: bool = false;
    println!("Um é: {}. Mentira é: {}", um, mentira);

    // if
    let n = 10;
    if n % 2 == 0 {
        println!("par");
    } else {
        println!("ímpar");
    }

    // mut
    let mut mensagem;
    if n % 2 == 0 {
        mensagem = "par";
    } else {
        mensagem = "impar";
    }
    println!("o numero {} é {}", n, mensagem);
    
    // if como expressao
    mensagem = if n % 2 == 0 {
        "par"
    } else {
        "impar"
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
