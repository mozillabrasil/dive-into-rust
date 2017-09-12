// Faça o código compilar.
// O compilador tem razão em reclamar do seu código?
fn main() {
    let mut vetor = vec![1, 3, 2];
    let mut maior = &vetor[0];
    for x in &vetor {
        if x > maior {
            maior = x;
        }
    }
    vetor.push(maior + 1);
    println!("O maior é: {}", maior);
}
