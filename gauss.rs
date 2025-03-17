// Arquivo: gauss.rs (ou main.rs)

use std::io;
use std::time::Instant;

fn main() {
    // Exemplo: vamos ler N da linha de comando ou fixar
    let N: usize = 4; // Coloque seu valor

    // Cria e inicializa a matriz A e o vetor B (exemplo simples)
    let mut A = vec![vec![0.0f32; N]; N];
    let mut B = vec![0.0f32; N];

    // Inicialização fictícia (poderia ler de algum lugar ou gerar aleatório)
    for row in 0..N {
        for col in 0..N {
            A[row][col] = (row + col) as f32 + 1.0;
        }
        B[row] = row as f32 + 2.0;
    }

    // Inicia temporização
    let start = Instant::now();

    // Executa eliminação gaussiana
    let X = gauss(&mut A, &mut B);

    // Para imprimir X
    println!("Solução X:");
    for (i, val) in X.iter().enumerate() {
        println!("X[{}] = {}", i, val);
    }

    // Tempo decorrido
    let duration = start.elapsed();
    println!("Tempo de execução: {:?}", duration);
}

// Função gauss: Eliminação Gaussiana sem pivot
fn gauss(a: &mut Vec<Vec<f32>>, b: &mut Vec<f32>) -> Vec<f32> {
    let n = a.len();

    // Forward Elimination
    for norm in 0..(n - 1) {
        for row in (norm + 1)..n {
            let multiplier = a[row][norm] / a[norm][norm];
            for col in norm..n {
                a[row][col] -= a[norm][col] * multiplier;
            }
            b[row] -= b[norm] * multiplier;
        }
    }

    // Back Substitution
    let mut x = vec![0.0f32; n];
    for row in (0..n).rev() { // de n-1 até 0
        x[row] = b[row];
        for col in (row + 1)..n {
            x[row] -= a[row][col] * x[col];
        }
        x[row] /= a[row][row];
    }

    x
}
