use rand::{Rng, SeedableRng};
use rand::rngs::StdRng;
use std::env;
use std::time::Instant;
use winapi::shared::minwindef::FILETIME;
use winapi::um::processthreadsapi::{GetCurrentProcess, GetProcessTimes};
use std::mem::MaybeUninit;

const MAXN: usize = 5000;

fn filetime_to_u64(ft: &FILETIME) -> u64 {
    (ft.dwHighDateTime as u64) << 32 | ft.dwLowDateTime as u64
}

fn get_process_times() -> (f64, f64) {
    unsafe {
        let process = GetCurrentProcess();
        let mut creation = MaybeUninit::<FILETIME>::uninit();
        let mut exit = MaybeUninit::<FILETIME>::uninit();
        let mut kernel = MaybeUninit::<FILETIME>::uninit();
        let mut user = MaybeUninit::<FILETIME>::uninit();

        GetProcessTimes(
            process,
            creation.as_mut_ptr(),
            exit.as_mut_ptr(),
            kernel.as_mut_ptr(),
            user.as_mut_ptr(),
        );

        let user_time = filetime_to_u64(&user.assume_init()) as f64 * 1e-4;
        let kernel_time = filetime_to_u64(&kernel.assume_init()) as f64 * 1e-4;

        (user_time, kernel_time)
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <matrix_dimension> [random seed]", args[0]);
        return;
    }

    let n: usize = args[1].parse().unwrap_or_else(|_| {
        eprintln!("N must be between 1 and {}", MAXN);
        std::process::exit(1);
    });

    if n < 1 || n > MAXN {
        eprintln!("N must be between 1 and {}", MAXN);
        std::process::exit(1);
    }

    let seed: u64 = args.get(2).map_or_else(
        || Instant::now().elapsed().as_nanos() as u64,
        |s| s.parse().unwrap_or_else(|_| {
            eprintln!("Invalid seed");
            std::process::exit(1);
        })
    );

    let mut rng = StdRng::seed_from_u64(seed);

    // Inicialização das matrizes
    let mut a = vec![vec![0.0f32; n]; n];
    let mut b = vec![0.0f32; n];
    let mut x = vec![0.0f32; n];

    println!("\nInitializing...");
    for col in 0..n {
        for row in 0..n {
            a[row][col] = rng.gen::<f32>();
        }
        b[col] = rng.gen::<f32>();
    }

    // Impressão das matrizes
    if n < 10 {
        println!("\nA =");
        for row in 0..n {
            print!("\t");
            for col in 0..n {
                print!("{:5.2}", a[row][col]);
                if col < n - 1 { print!(", "); } else { println!(";"); }
            }
        }
        println!("\nB = [");
        for col in 0..n {
            print!("{:5.2}", b[col]);
            if col < n - 1 { print!("; "); } else { println!("]"); }
        }
    }

    // Medição de tempo
    let start_time = Instant::now();
    let (start_user, start_system) = get_process_times();

    // Eliminação Gaussiana
    gauss(&mut a, &mut b, &mut x, n);

    let (end_user, end_system) = get_process_times();
    let elapsed = start_time.elapsed();

    // Resultados
    if n < 100 {
        println!("\nX = [");
        for row in 0..n {
            print!("{:5.2}", x[row]);
            if row < n - 1 { print!("; "); } else { println!("]"); }
        }
    }

    println!("\nElapsed time: {:.3} ms", elapsed.as_secs_f64() * 1000.0);
    println!("User CPU time: {:.3} ms", end_user - start_user);
    println!("System CPU time: {:.3} ms", end_system - start_system);
    println!("--------------------------------------------");
}

fn gauss(a: &mut [Vec<f32>], b: &mut [f32], x: &mut [f32], n: usize) {
    println!("Computing Gaussian elimination...");

    for norm in 0..n - 1 {
        for row in (norm + 1)..n {
            let multiplier = a[row][norm] / a[norm][norm];
            for col in norm..n {
                a[row][col] -= a[norm][col] * multiplier;
            }
            b[row] -= b[norm] * multiplier;
        }
    }

    // Substituição reversa
    for row in (0..n).rev() {
        x[row] = b[row];
        for col in (row + 1)..n {
            x[row] -= a[row][col] * x[col];
        }
        x[row] /= a[row][row];
    }

    println!("Calculation complete. X[0] = {:.6}", x[0]);
}