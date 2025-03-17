// Arquivo: gauss.go
package main

import (
    "fmt"
    "time"
)

func main() {
    N := 4 // ou ler de argumentos

    A := make([][]float32, N)
    for i := 0; i < N; i++ {
        A[i] = make([]float32, N)
    }
    B := make([]float32, N)

    // Inicializa a matriz e vetor (exemplo)
    for row := 0; row < N; row++ {
        for col := 0; col < N; col++ {
            A[row][col] = float32(row+col) + 1.0
        }
        B[row] = float32(row) + 2.0
    }

    start := time.Now()
    X := gauss(A, B)
    elapsed := time.Since(start)

    // Print da solução
    fmt.Println("Solução X:")
    for i, val := range X {
        fmt.Printf("X[%d] = %f\n", i, val)
    }
    fmt.Printf("Tempo de execução: %v\n", elapsed)
}

func gauss(A [][]float32, B []float32) []float32 {
    N := len(A)

    // Forward Elimination
    for norm := 0; norm < N-1; norm++ {
        for row := norm + 1; row < N; row++ {
            multiplier := A[row][norm] / A[norm][norm]
            for col := norm; col < N; col++ {
                A[row][col] -= A[norm][col] * multiplier
            }
            B[row] -= B[norm] * multiplier
        }
    }

    // Back Substitution
    X := make([]float32, N)
    for row := N - 1; row >= 0; row-- {
        X[row] = B[row]
        for col := N - 1; col > row; col-- {
            X[row] -= A[row][col] * X[col]
        }
        X[row] /= A[row][row]
    }

    return X
}
