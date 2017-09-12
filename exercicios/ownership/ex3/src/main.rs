// Faça o código compilar.
// O compilador tem razão em reclamar do seu código?
fn main() {
    let vetor = vec![1, 2, 3, 4];
    println!("A metade é: {:?}", metade(&vetor));
}

fn metade(vetor: &Vec<i32>) -> &[i32] {
    let meio = vetor.len()/2 + 1;
    let mut metade = Vec::new();
    for i in 0..meio {
        metade.push(vetor[i]);
    }
    &metade
}
