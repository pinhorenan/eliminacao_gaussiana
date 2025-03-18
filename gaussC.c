#include <stdio.h>
#include <stdlib.h>
#include <math.h>
#include <windows.h>
#include <time.h>  // Adicionado para clock()

/* Program Parameters */
#define MAXN 5000  /* Max value of N */
int N;  /* Matrix size */

/* Matrices and vectors */
float **A, *B, *X;  // Alocação dinâmica

/* Prototype */
void gauss();

/* Windows timing replacements */
typedef struct {
    LARGE_INTEGER start;
    LARGE_INTEGER stop;
} win_timer;

static win_timer timer;

/* Substitute for gettimeofday() */
void windows_gettimeofday(LARGE_INTEGER *t) {
    QueryPerformanceCounter(t);
}

/* Returns time difference in milliseconds */
double get_elapsed_time(win_timer *t) {
    LARGE_INTEGER frequency;
    QueryPerformanceFrequency(&frequency);
    return (double)(t->stop.QuadPart - t->start.QuadPart) * 1000.0 / frequency.QuadPart;  // Convertido para ms
}

/* Returns a seed for srand based on the time */
unsigned int time_seed() {
    LARGE_INTEGER t;
    QueryPerformanceCounter(&t);
    return (unsigned int)(t.QuadPart & 0xFFFFFFFF);
}

/* Set the program parameters from the command-line arguments */
void parameters(int argc, char **argv) {
    int seed = 0;  /* Random seed */

    /* Read command-line arguments */
    srand(time_seed());  /* Randomize */

    if (argc == 3) {
        seed = atoi(argv[2]);
        srand(seed);
        printf("Random seed = %i\n", seed);
    }
    if (argc >= 2) {
        N = atoi(argv[1]);
        if (N < 1 || N > MAXN) {
            printf("N = %i is out of range. Must be between 1 and %d.\n", N, MAXN);
            exit(1);
        }
    } else {
        printf("Usage: %s <matrix_dimension> [random seed]\n", argv[0]);
        exit(1);
    }

    /* Print parameters */
    printf("\nMatrix dimension N = %i.\n", N);
}

/* Initialize A and B (and X to 0.0s) */
void initialize_inputs() {
    int row, col;

    printf("\nInitializing...\n");

    // Alocação dinâmica
    A = (float **)malloc(N * sizeof(float *));
    for (row = 0; row < N; row++) {
        A[row] = (float *)malloc(N * sizeof(float));
    }
    B = (float *)malloc(N * sizeof(float));
    X = (float *)malloc(N * sizeof(float));

    for (col = 0; col < N; col++) {
        for (row = 0; row < N; row++) {
            A[row][col] = (float)rand() / RAND_MAX;
        }
        B[col] = (float)rand() / RAND_MAX;
        X[col] = 0.0;
    }
}

/* Print input matrices */
void print_inputs() {
    int row, col;

    if (N < 10) {
        printf("\nA =\n\t");
        for (row = 0; row < N; row++) {
            for (col = 0; col < N; col++) {
                printf("%5.2f%s", A[row][col], (col < N-1) ? ", " : ";\n\t");
            }
        }
        printf("\nB = [");
        for (col = 0; col < N; col++) {
            printf("%5.2f%s", B[col], (col < N-1) ? "; " : "]\n");
        }
    }
}

/* Print solution vector X */
void print_X() {
    int row;

    if (N < 100) {
        printf("\nX = [");
        for (row = 0; row < N; row++) {
            printf("%5.2f%s", X[row], (row < N-1) ? "; " : "]\n");
        }
    }
}

/* Gaussian elimination without pivoting */
void gauss() {
    int norm, row, col;
    float multiplier;

    printf("Computing Gaussian elimination...\n");

    /* Gaussian elimination */
    for (norm = 0; norm < N - 1; norm++) {
        for (row = norm + 1; row < N; row++) {
            multiplier = A[row][norm] / A[norm][norm];
            for (col = norm; col < N; col++) {
                A[row][col] -= A[norm][col] * multiplier;
            }
            B[row] -= B[norm] * multiplier;
        }
    }

    /* Back substitution */
    for (row = N - 1; row >= 0; row--) {
        X[row] = B[row];
        for (col = N-1; col > row; col--) {
            X[row] -= A[row][col] * X[col];
        }
        X[row] /= A[row][row];
    }

    printf("Calculation complete. X[0] = %f\n", X[0]);  // Verificação
}

int main(int argc, char **argv) {
    double user_start, user_end, system_start, system_end;
    clock_t start_cpu, end_cpu;  // Variáveis para medir o tempo de CPU

    /* Process program parameters */
    parameters(argc, argv);

    /* Initialize A and B */
    initialize_inputs();

    /* Print input matrices */
    print_inputs();

    /* Start timing */
    printf("\nStarting clock.\n");
    start_cpu = clock();  // Início da medição de CPU
    windows_gettimeofday(&timer.start);

    /* Gaussian Elimination */
    gauss();

    /* Stop timing */
    windows_gettimeofday(&timer.stop);
    end_cpu = clock();  // Fim da medição de CPU

    /* Display results */
    print_X();

    /* Calculate and print times */
    double elapsed = get_elapsed_time(&timer);  // Já em ms
    double cpu_time_used = (double)(end_cpu - start_cpu) * 1000.0 / CLOCKS_PER_SEC;  // Convertido para ms

    // Usar GetProcessTimes para separar tempo de usuário e sistema
    FILETIME createTime, exitTime, kernelTime, userTime;
    GetProcessTimes(GetCurrentProcess(), &createTime, &exitTime, &kernelTime, &userTime);

    ULARGE_INTEGER ulKernel, ulUser;
    ulKernel.LowPart = kernelTime.dwLowDateTime;
    ulKernel.HighPart = kernelTime.dwHighDateTime;
    ulUser.LowPart = userTime.dwLowDateTime;
    ulUser.HighPart = userTime.dwHighDateTime;

    double user_time = ulUser.QuadPart * 0.0001;  // Convertido para ms
    double system_time = ulKernel.QuadPart * 0.0001;  // Convertido para ms

    // Ajuste para garantir que o System CPU time não seja zero
    if (system_time < 0.001) {
        system_time = cpu_time_used - user_time;  // Calcula como diferença
        if (system_time < 0.0) system_time = 0.0;  // Evita valores negativos
    }

    printf("\nElapsed time = %.3f ms\n", elapsed);
    printf("User CPU time = %.3f ms\n", user_time);
    printf("System CPU time = %.3f ms\n", system_time);
    printf("Total CPU time = %.3f ms\n", cpu_time_used);
    printf("--------------------------------------------\n");

    /* Free allocated memory */
    for (int i = 0; i < N; i++) {
        free(A[i]);
    }
    free(A);
    free(B);
    free(X);

    return 0;
}