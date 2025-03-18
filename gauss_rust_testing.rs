#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
use libc::c;
unsafe extern "C" {
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn atoi(__nptr: *const libc::c_char) -> libc::c_int;
    fn rand() -> libc::c_int;
    fn srand(__seed: libc::c_uint);
    fn exit(_: libc::c_int) -> !;
    fn times(__buffer: *mut tms) -> clock_t;
    fn gettimeofday(__tv: *mut timeval, __tz: *mut libc::c_void) -> libc::c_int;
}
pub type __clock_t = libc::c_long;
pub type __time_t = libc::c_long;
pub type __suseconds_t = libc::c_long;
pub type clock_t = __clock_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timeval {
    pub tv_sec: __time_t,
    pub tv_usec: __suseconds_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tms {
    pub tms_utime: clock_t,
    pub tms_stime: clock_t,
    pub tms_cutime: clock_t,
    pub tms_cstime: clock_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timezone {
    pub tz_minuteswest: libc::c_int,
    pub tz_dsttime: libc::c_int,
}
#[unsafe(no_mangle)]
pub static mut N: libc::c_int = 0;
#[unsafe(no_mangle)]
pub static mut A: [[libc::c_float; 2000]; 2000] = [[0.; 2000]; 2000];
#[unsafe(no_mangle)]
pub static mut B: [libc::c_float; 2000] = [0.; 2000];
#[unsafe(no_mangle)]
pub static mut X: [libc::c_float; 2000] = [0.; 2000];
#[unsafe(no_mangle)]
pub unsafe extern "C" fn time_seed() -> libc::c_uint {
    let mut t: timeval = timeval { tv_sec: 0, tv_usec: 0 };
    let mut tzdummy: timezone = timezone {
        tz_minuteswest: 0,
        tz_dsttime: 0,
    };
    gettimeofday(&mut t, &mut tzdummy as *mut timezone as *mut libc::c_void);
    return t.tv_usec as libc::c_uint;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn parameters(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) {
    let mut seed: libc::c_int = 0 as libc::c_int;
    let mut uid: [libc::c_char; 32] = [0; 32];
    srand(time_seed());
    if argc == 3 as libc::c_int {
        seed = atoi(*argv.offset(2 as libc::c_int as isize));
        srand(seed as libc::c_uint);
        printf(b"Random seed = %i\n\0" as *const u8 as *const libc::c_char, seed);
    }
    if argc >= 2 as libc::c_int {
        N = atoi(*argv.offset(1 as libc::c_int as isize));
        if N < 1 as libc::c_int || N > 2000 as libc::c_int {
            printf(
                b"N = %i is out of range.\n\0" as *const u8 as *const libc::c_char,
                N,
            );
            exit(0 as libc::c_int);
        }
    } else {
        printf(
            b"Usage: %s <matrix_dimension> [random seed]\n\0" as *const u8
                as *const libc::c_char,
            *argv.offset(0 as libc::c_int as isize),
        );
        exit(0 as libc::c_int);
    }
    printf(b"\nMatrix dimension N = %i.\n\0" as *const u8 as *const libc::c_char, N);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn initialize_inputs() {
    let mut row: libc::c_int = 0;
    let mut col: libc::c_int = 0;
    printf(b"\nInitializing...\n\0" as *const u8 as *const libc::c_char);
    col = 0 as libc::c_int;
    while col < N {
        row = 0 as libc::c_int;
        while row < N {
            ::core::ptr::write_volatile(
                &mut A[row as usize][col as usize] as *mut libc::c_float,
                (rand() as libc::c_float as libc::c_double / 32768.0f64) as libc::c_float,
            );
            row += 1;
            row;
        }
        ::core::ptr::write_volatile(
            &mut B[col as usize] as *mut libc::c_float,
            (rand() as libc::c_float as libc::c_double / 32768.0f64) as libc::c_float,
        );
        ::core::ptr::write_volatile(
            &mut X[col as usize] as *mut libc::c_float,
            0.0f64 as libc::c_float,
        );
        col += 1;
        col;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn print_inputs() {
    let mut row: libc::c_int = 0;
    let mut col: libc::c_int = 0;
    if N < 10 as libc::c_int {
        printf(b"\nA =\n\t\0" as *const u8 as *const libc::c_char);
        row = 0 as libc::c_int;
        while row < N {
            col = 0 as libc::c_int;
            while col < N {
                printf(
                    b"%5.2f%s\0" as *const u8 as *const libc::c_char,
                    A[row as usize][col as usize] as libc::c_double,
                    if col < N - 1 as libc::c_int {
                        b", \0" as *const u8 as *const libc::c_char
                    } else {
                        b";\n\t\0" as *const u8 as *const libc::c_char
                    },
                );
                col += 1;
                col;
            }
            row += 1;
            row;
        }
        printf(b"\nB = [\0" as *const u8 as *const libc::c_char);
        col = 0 as libc::c_int;
        while col < N {
            printf(
                b"%5.2f%s\0" as *const u8 as *const libc::c_char,
                B[col as usize] as libc::c_double,
                if col < N - 1 as libc::c_int {
                    b"; \0" as *const u8 as *const libc::c_char
                } else {
                    b"]\n\0" as *const u8 as *const libc::c_char
                },
            );
            col += 1;
            col;
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn print_X() {
    let mut row: libc::c_int = 0;
    if N < 100 as libc::c_int {
        printf(b"\nX = [\0" as *const u8 as *const libc::c_char);
        row = 0 as libc::c_int;
        while row < N {
            printf(
                b"%5.2f%s\0" as *const u8 as *const libc::c_char,
                X[row as usize] as libc::c_double,
                if row < N - 1 as libc::c_int {
                    b"; \0" as *const u8 as *const libc::c_char
                } else {
                    b"]\n\0" as *const u8 as *const libc::c_char
                },
            );
            row += 1;
            row;
        }
    }
}
unsafe fn main_0(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut etstart: timeval = timeval { tv_sec: 0, tv_usec: 0 };
    let mut etstop: timeval = timeval { tv_sec: 0, tv_usec: 0 };
    let mut tzdummy: timezone = timezone {
        tz_minuteswest: 0,
        tz_dsttime: 0,
    };
    let mut etstart2: clock_t = 0;
    let mut etstop2: clock_t = 0;
    let mut usecstart: libc::c_ulonglong = 0;
    let mut usecstop: libc::c_ulonglong = 0;
    let mut cputstart: tms = tms {
        tms_utime: 0,
        tms_stime: 0,
        tms_cutime: 0,
        tms_cstime: 0,
    };
    let mut cputstop: tms = tms {
        tms_utime: 0,
        tms_stime: 0,
        tms_cutime: 0,
        tms_cstime: 0,
    };
    parameters(argc, argv);
    initialize_inputs();
    print_inputs();
    printf(b"\nStarting clock.\n\0" as *const u8 as *const libc::c_char);
    gettimeofday(&mut etstart, &mut tzdummy as *mut timezone as *mut libc::c_void);
    etstart2 = times(&mut cputstart);
    gauss();
    gettimeofday(&mut etstop, &mut tzdummy as *mut timezone as *mut libc::c_void);
    etstop2 = times(&mut cputstop);
    printf(b"Stopped clock.\n\0" as *const u8 as *const libc::c_char);
    usecstart = (etstart.tv_sec as libc::c_ulonglong)
        .wrapping_mul(1000000 as libc::c_int as libc::c_ulonglong)
        .wrapping_add(etstart.tv_usec as libc::c_ulonglong);
    usecstop = (etstop.tv_sec as libc::c_ulonglong)
        .wrapping_mul(1000000 as libc::c_int as libc::c_ulonglong)
        .wrapping_add(etstop.tv_usec as libc::c_ulonglong);
    print_X();
    printf(
        b"\nElapsed time = %g ms.\n\0" as *const u8 as *const libc::c_char,
        (usecstop.wrapping_sub(usecstart) as libc::c_float
            / 1000 as libc::c_int as libc::c_float) as libc::c_double,
    );
    printf(
        b"(CPU times are accurate to the nearest %g ms)\n\0" as *const u8
            as *const libc::c_char,
        1.0f64 / 1000000 as libc::c_int as __clock_t as libc::c_float as libc::c_double
            * 1000.0f64,
    );
    printf(
        b"My total CPU time for parent = %g ms.\n\0" as *const u8 as *const libc::c_char,
        ((cputstop.tms_utime + cputstop.tms_stime
            - (cputstart.tms_utime + cputstart.tms_stime)) as libc::c_float
            / 1000000 as libc::c_int as __clock_t as libc::c_float
            * 1000 as libc::c_int as libc::c_float) as libc::c_double,
    );
    printf(
        b"My system CPU time for parent = %g ms.\n\0" as *const u8
            as *const libc::c_char,
        ((cputstop.tms_stime - cputstart.tms_stime) as libc::c_float
            / 1000000 as libc::c_int as __clock_t as libc::c_float
            * 1000 as libc::c_int as libc::c_float) as libc::c_double,
    );
    printf(
        b"My total CPU time for child processes = %g ms.\n\0" as *const u8
            as *const libc::c_char,
        ((cputstop.tms_cutime + cputstop.tms_cstime
            - (cputstart.tms_cutime + cputstart.tms_cstime)) as libc::c_float
            / 1000000 as libc::c_int as __clock_t as libc::c_float
            * 1000 as libc::c_int as libc::c_float) as libc::c_double,
    );
    printf(
        b"--------------------------------------------\n\0" as *const u8
            as *const libc::c_char,
    );
    exit(0 as libc::c_int);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn gauss() {
    let mut norm: libc::c_int = 0;
    let mut row: libc::c_int = 0;
    let mut col: libc::c_int = 0;
    let mut multiplier: libc::c_float = 0.;
    printf(b"Computing Serially.\n\0" as *const u8 as *const libc::c_char);
    norm = 0 as libc::c_int;
    while norm < N - 1 as libc::c_int {
        row = norm + 1 as libc::c_int;
        while row < N {
            multiplier = A[row as usize][norm as usize]
                / A[norm as usize][norm as usize];
            col = norm;
            while col < N {
                ::core::ptr::write_volatile(
                    &mut A[row as usize][col as usize] as *mut libc::c_float,
                    ::core::ptr::read_volatile::<
                        libc::c_float,
                    >(&A[row as usize][col as usize] as *const libc::c_float)
                        - A[norm as usize][col as usize] * multiplier,
                );
                col += 1;
                col;
            }
            ::core::ptr::write_volatile(
                &mut B[row as usize] as *mut libc::c_float,
                ::core::ptr::read_volatile::<
                    libc::c_float,
                >(&B[row as usize] as *const libc::c_float)
                    - B[norm as usize] * multiplier,
            );
            row += 1;
            row;
        }
        norm += 1;
        norm;
    }
    row = N - 1 as libc::c_int;
    while row >= 0 as libc::c_int {
        ::core::ptr::write_volatile(
            &mut X[row as usize] as *mut libc::c_float,
            B[row as usize],
        );
        col = N - 1 as libc::c_int;
        while col > row {
            ::core::ptr::write_volatile(
                &mut X[row as usize] as *mut libc::c_float,
                ::core::ptr::read_volatile::<
                    libc::c_float,
                >(&X[row as usize] as *const libc::c_float)
                    - A[row as usize][col as usize] * X[col as usize],
            );
            col -= 1;
            col;
        }
        ::core::ptr::write_volatile(
            &mut X[row as usize] as *mut libc::c_float,
            ::core::ptr::read_volatile::<
                libc::c_float,
            >(&X[row as usize] as *const libc::c_float) / A[row as usize][row as usize],
        );
        row -= 1;
        row;
    }
}
pub fn main() {
    let mut args: Vec::<*mut libc::c_char> = Vec::new();
    for arg in ::std::env::args() {
        args.push(
            (::std::ffi::CString::new(arg))
                .expect("Failed to convert argument into CString.")
                .into_raw(),
        );
    }
    args.push(::core::ptr::null_mut());
    unsafe {
        ::std::process::exit(
            main_0(
                (args.len() - 1) as libc::c_int,
                args.as_mut_ptr() as *mut *mut libc::c_char,
            ) as i32,
        )
    }
}

