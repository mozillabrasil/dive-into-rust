// Loops
fn main() {
    let mut contagem = 1;
    loop {
        canta(contagem);
        contagem += 1;
        if contagem == 10 {
            println!("todos no mesmo bote");
            break;
        }
    }

    let mut contagem = 1;
    while contagem < 10 {
        canta(contagem);
        contagem += 1;
    }
    println!("todos no mesmo bote");

    for contagem in 1..10 {
        canta(contagem);
    }
    println!("todos no mesmo bote");

    let vetor = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
    for n in vetor {
        canta(n);
    }
    println!("todos no mesmo bote");
}

fn canta(n: i32) {
    print!("{}, ", n);
    if n == 3 || n == 6 || n == 9 {
        println!("índiozinhos");
    }
}