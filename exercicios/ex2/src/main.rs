fn main() { 
    /* pode deixar vazio */
}

/// Dado um número positivo x:
///     Se x é par, divida x por 2 e repita.
///     Se x é ímpar, faça x igual a 3x + 1 e repita.
/// Retorne quantas vezes é necessário repetir o processo até que x = 1.
fn collatz(mut x: u32) -> u32 {

}


// Testes

#[test]
fn test_9() {
    assert_eq!(collatz(9), 19);
}

#[test]
fn test_97() {
    assert_eq!(collatz(97), 118);
}

#[test]
fn test_77_031() {
    assert_eq!(collatz(77_031), 350);
}
