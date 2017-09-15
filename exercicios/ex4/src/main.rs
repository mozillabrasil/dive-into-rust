struct Point {
    x: u32,
    y: u32,
}

// Exercicio 1: Defina um struct para representar um retangulo utilizando Point.

fn main() {
    // Cria um ponto.
    let point: Point = Point { x: 3, y: 4 };

    // Acessa campos do ponto.
    println!("point coordinates: ({}, {})", point.x, point.y);

    // Exercicio 2: Crie um retangulo.
}

// Exercicio 3: Faça um construtor `new_rect` para criar um retangulo.

// Exercicio 4: Faça um método `rect_area` para calcular a area de um retangulo.

// Desafio: Faça um método que verifica se um retangulo intersecta outro.