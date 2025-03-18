package main

import (
	"fmt"
	"math/rand"
	"os"
	"strconv"
	"time"
)

func main() {
	if len(os.Args) < 2 {
		fmt.Printf("Usage: %s <matrix_dimension> [random seed]\n", os.Args[0])
		os.Exit(0)
	}

	N, err := strconv.Atoi(os.Args[1])
	if err != nil || N < 1 || N > 2000 {
		fmt.Printf("N deve estar entre 1 e 2000.\n")
		os.Exit(0)
	}

	if len(os.Args) >= 3 {
		seed, err := strconv.ParseInt(os.Args[2], 10, 64)
		if err != nil {
			fmt.Printf("Seed inválido: %s\n", os.Args[2])
			os.Exit(0)
		}
		rand.Seed(seed)
	} else {
		rand.Seed(time.Now().UnixNano())
	}

	A := make([][]float32, N)
	for i := range A {
		A[i] = make([]float32, N)
	}
	B := make([]float32, N)
	X := make([]float32, N)

	fmt.Println("\nInicializando...")
	for col := 0; col < N; col++ {
		for row := 0; row < N; row++ {
			A[row][col] = float32(rand.Intn(32768)) / 32768.0
		}
		B[col] = float32(rand.Intn(32768)) / 32768.0
	}

	printInputs(A, B, N)

	startTime := time.Now()

	gauss(A, B, X, N)

	elapsed := time.Since(startTime)

	printX(X, N)

	fmt.Printf("\nTempo decorrido = %v ms.\n", elapsed.Seconds()*1000)
}

func printInputs(A [][]float32, B []float32, N int) {
	if N < 10 {
		fmt.Println("\nA =")
		for row := 0; row < N; row++ {
			fmt.Print("\t")
			for col := 0; col < N; col++ {
				fmt.Printf("%5.2f", A[row][col])
				if col < N-1 {
					fmt.Print(", ")
				} else {
					fmt.Println(";")
				}
			}
		}
		fmt.Println("\nB = [")
		for col := 0; col < N; col++ {
			fmt.Printf("%5.2f", B[col])
			if col < N-1 {
				fmt.Print("; ")
			} else {
				fmt.Println("]")
			}
		}
	}
}

func printX(X []float32, N int) {
	if N < 100 {
		fmt.Println("\nX = [")
		for row := 0; row < N; row++ {
			fmt.Printf("%5.2f", X[row])
			if row < N-1 {
				fmt.Print("; ")
			} else {
				fmt.Println("]")
			}
		}
	}
}

func gauss(A [][]float32, B []float32, X []float32, N int) {
	fmt.Println("Calculando...")

	for norm := 0; norm < N-1; norm++ {
		for row := norm + 1; row < N; row++ {
			multiplier := A[row][norm] / A[norm][norm]
			for col := norm; col < N; col++ {
				A[row][col] -= A[norm][col] * multiplier
			}
			B[row] -= B[norm] * multiplier
		}
	}

	for row := N - 1; row >= 0; row-- {
		X[row] = B[row]
		for col := row + 1; col < N; col++ {
			X[row] -= A[row][col] * X[col]
		}
		X[row] /= A[row][row]
	}
}
