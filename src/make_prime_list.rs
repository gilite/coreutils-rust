#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![feature(extern_types)]
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    static mut stdout: *mut FILE;
    static mut stderr: *mut FILE;
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn puts(__s: *const libc::c_char) -> libc::c_int;
    fn ferror(__stream: *mut FILE) -> libc::c_int;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strerror(_: libc::c_int) -> *mut libc::c_char;
    fn free(_: *mut libc::c_void);
    fn strtol(
        _: *const libc::c_char,
        _: *mut *mut libc::c_char,
        _: libc::c_int,
    ) -> libc::c_long;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn abort() -> !;
    fn exit(_: libc::c_int) -> !;
    fn __errno_location() -> *mut libc::c_int;
}
pub type __uintmax_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type uintmax_t = __uintmax_t;
pub type size_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_FILE {
    pub _flags: libc::c_int,
    pub _IO_read_ptr: *mut libc::c_char,
    pub _IO_read_end: *mut libc::c_char,
    pub _IO_read_base: *mut libc::c_char,
    pub _IO_write_base: *mut libc::c_char,
    pub _IO_write_ptr: *mut libc::c_char,
    pub _IO_write_end: *mut libc::c_char,
    pub _IO_buf_base: *mut libc::c_char,
    pub _IO_buf_end: *mut libc::c_char,
    pub _IO_save_base: *mut libc::c_char,
    pub _IO_backup_base: *mut libc::c_char,
    pub _IO_save_end: *mut libc::c_char,
    pub _markers: *mut _IO_marker,
    pub _chain: *mut _IO_FILE,
    pub _fileno: libc::c_int,
    pub _flags2: libc::c_int,
    pub _old_offset: __off_t,
    pub _cur_column: libc::c_ushort,
    pub _vtable_offset: libc::c_schar,
    pub _shortbuf: [libc::c_char; 1],
    pub _lock: *mut libc::c_void,
    pub _offset: __off64_t,
    pub _codecvt: *mut _IO_codecvt,
    pub _wide_data: *mut _IO_wide_data,
    pub _freeres_list: *mut _IO_FILE,
    pub _freeres_buf: *mut libc::c_void,
    pub __pad5: size_t,
    pub _mode: libc::c_int,
    pub _unused2: [libc::c_char; 20],
}
pub type _IO_lock_t = ();
pub type FILE = _IO_FILE;
pub type wide_uint = uintmax_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct prime {
    pub p: libc::c_uint,
    pub pinv: wide_uint,
    pub lim: wide_uint,
}
#[inline]
unsafe extern "C" fn atoi(mut __nptr: *const libc::c_char) -> libc::c_int {
    return strtol(
        __nptr,
        0 as *mut libc::c_void as *mut *mut libc::c_char,
        10 as libc::c_int,
    ) as libc::c_int;
}
unsafe extern "C" fn binvert(mut a: wide_uint) -> wide_uint {
    let mut x: wide_uint = (0xf5397db1 as libc::c_uint
        >> (4 as libc::c_int as libc::c_ulong)
            .wrapping_mul(
                a.wrapping_div(2 as libc::c_int as libc::c_ulong)
                    & 0x7 as libc::c_int as libc::c_ulong,
            )) as wide_uint;
    loop {
        let mut y: wide_uint = (2 as libc::c_int as libc::c_ulong)
            .wrapping_mul(x)
            .wrapping_sub(x.wrapping_mul(x).wrapping_mul(a));
        if y == x {
            return x;
        }
        x = y;
    };
}
unsafe extern "C" fn process_prime(mut info: *mut prime, mut p: libc::c_uint) {
    let mut max: wide_uint = -(1 as libc::c_int) as wide_uint;
    (*info).p = p;
    (*info).pinv = binvert(p as wide_uint);
    (*info).lim = max.wrapping_div(p as libc::c_ulong);
}
unsafe extern "C" fn print_wide_uint(
    mut n: wide_uint,
    mut nesting: libc::c_int,
    mut wide_uint_bits: libc::c_uint,
) {
    let mut hex_digits_per_literal: libc::c_int = 7 as libc::c_int;
    let mut bits_per_literal: libc::c_int = hex_digits_per_literal * 4 as libc::c_int;
    let mut remainder: libc::c_uint = (n
        & (((1 as libc::c_int) << bits_per_literal) - 1 as libc::c_int) as libc::c_ulong)
        as libc::c_uint;
    if n != remainder as libc::c_ulong {
        let mut needs_parentheses: libc::c_int = (n >> bits_per_literal
            >> bits_per_literal != 0 as libc::c_int as libc::c_ulong) as libc::c_int;
        if needs_parentheses != 0 {
            printf(b"(\0" as *const u8 as *const libc::c_char);
        }
        print_wide_uint(
            n >> bits_per_literal,
            nesting + 1 as libc::c_int,
            wide_uint_bits,
        );
        if needs_parentheses != 0 {
            printf(
                b")\n%*s\0" as *const u8 as *const libc::c_char,
                nesting + 3 as libc::c_int,
                b"\0" as *const u8 as *const libc::c_char,
            );
        }
        printf(b" << %d | \0" as *const u8 as *const libc::c_char, bits_per_literal);
    } else if nesting != 0 {
        printf(b"(uintmax_t) \0" as *const u8 as *const libc::c_char);
        hex_digits_per_literal = wide_uint_bits
            .wrapping_sub(1 as libc::c_int as libc::c_uint)
            .wrapping_rem(bits_per_literal as libc::c_uint)
            .wrapping_rem(4 as libc::c_int as libc::c_uint)
            .wrapping_add(1 as libc::c_int as libc::c_uint) as libc::c_int;
    }
    printf(
        b"0x%0*xU\0" as *const u8 as *const libc::c_char,
        hex_digits_per_literal,
        remainder,
    );
}
unsafe extern "C" fn output_primes(mut primes: *const prime, mut nprimes: libc::c_uint) {
    let mut i: libc::c_uint = 0;
    let mut p: libc::c_uint = 0;
    let mut is_prime: libc::c_int = 0;
    let mut wide_uint_bits: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    let mut mask: wide_uint = -(1 as libc::c_int) as wide_uint;
    wide_uint_bits = 0 as libc::c_int as libc::c_uint;
    while mask != 0 {
        mask >>= 1 as libc::c_int;
        wide_uint_bits = wide_uint_bits.wrapping_add(1);
    }
    puts(b"/* Generated file -- DO NOT EDIT */\n\0" as *const u8 as *const libc::c_char);
    printf(
        b"#define WIDE_UINT_BITS %u\n\0" as *const u8 as *const libc::c_char,
        wide_uint_bits,
    );
    i = 0 as libc::c_int as libc::c_uint;
    p = 2 as libc::c_int as libc::c_uint;
    while i < nprimes {
        let mut d8: libc::c_uint = if i.wrapping_add(8 as libc::c_int as libc::c_uint)
            < nprimes
        {
            ((*primes.offset(i.wrapping_add(8 as libc::c_int as libc::c_uint) as isize))
                .p)
                .wrapping_sub((*primes.offset(i as isize)).p)
        } else {
            0xff as libc::c_int as libc::c_uint
        };
        if (255 as libc::c_int as libc::c_uint) < d8 {
            abort();
        }
        printf(
            b"P (%u, %u,\n   (\0" as *const u8 as *const libc::c_char,
            ((*primes.offset(i as isize)).p).wrapping_sub(p),
            d8,
        );
        print_wide_uint(
            (*primes.offset(i as isize)).pinv,
            0 as libc::c_int,
            wide_uint_bits,
        );
        printf(
            b"),\n   UINTMAX_MAX / %u)\n\0" as *const u8 as *const libc::c_char,
            (*primes.offset(i as isize)).p,
        );
        p = (*primes.offset(i as isize)).p;
        i = i.wrapping_add(1);
    }
    printf(b"\n#undef FIRST_OMITTED_PRIME\n\0" as *const u8 as *const libc::c_char);
    loop {
        p = p.wrapping_add(2 as libc::c_int as libc::c_uint);
        i = 0 as libc::c_int as libc::c_uint;
        is_prime = 1 as libc::c_int;
        while is_prime != 0 {
            if ((*primes.offset(i as isize)).p)
                .wrapping_mul((*primes.offset(i as isize)).p) > p
            {
                break;
            }
            if (p as libc::c_ulong).wrapping_mul((*primes.offset(i as isize)).pinv)
                <= (*primes.offset(i as isize)).lim
            {
                is_prime = 0 as libc::c_int;
                break;
            } else {
                i = i.wrapping_add(1);
            }
        }
        if !(is_prime == 0) {
            break;
        }
    }
    printf(b"#define FIRST_OMITTED_PRIME %u\n\0" as *const u8 as *const libc::c_char, p);
}
unsafe extern "C" fn xalloc(mut s: size_t) -> *mut libc::c_void {
    let mut p: *mut libc::c_void = malloc(s);
    if !p.is_null() {
        return p;
    }
    fprintf(
        stderr,
        b"Virtual memory exhausted.\n\0" as *const u8 as *const libc::c_char,
    );
    exit(1 as libc::c_int);
}
unsafe fn main_0(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut limit: libc::c_int = 0;
    let mut sieve: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut size: size_t = 0;
    let mut i: size_t = 0;
    let mut prime_list: *mut prime = 0 as *mut prime;
    let mut nprimes: libc::c_uint = 0;
    if argc != 2 as libc::c_int {
        fprintf(
            stderr,
            b"Usage: %s LIMIT\nProduces a list of odd primes <= LIMIT\n\0" as *const u8
                as *const libc::c_char,
            *argv.offset(0 as libc::c_int as isize),
        );
        return 1 as libc::c_int;
    }
    limit = atoi(*argv.offset(1 as libc::c_int as isize));
    if limit < 3 as libc::c_int {
        return 0 as libc::c_int;
    }
    if limit & 1 as libc::c_int == 0 {
        limit -= 1;
    }
    size = ((limit - 1 as libc::c_int) / 2 as libc::c_int) as size_t;
    sieve = xalloc(size) as *mut libc::c_char;
    memset(sieve as *mut libc::c_void, 1 as libc::c_int, size);
    prime_list = xalloc(
        size.wrapping_mul(::core::mem::size_of::<prime>() as libc::c_ulong),
    ) as *mut prime;
    nprimes = 0 as libc::c_int as libc::c_uint;
    i = 0 as libc::c_int as size_t;
    while i < size {
        let mut p: libc::c_uint = (3 as libc::c_int as libc::c_ulong)
            .wrapping_add((2 as libc::c_int as libc::c_ulong).wrapping_mul(i))
            as libc::c_uint;
        let mut j: libc::c_uint = 0;
        let fresh0 = nprimes;
        nprimes = nprimes.wrapping_add(1);
        process_prime(&mut *prime_list.offset(fresh0 as isize), p);
        j = p
            .wrapping_mul(p)
            .wrapping_sub(3 as libc::c_int as libc::c_uint)
            .wrapping_div(2 as libc::c_int as libc::c_uint);
        while (j as libc::c_ulong) < size {
            *sieve.offset(j as isize) = 0 as libc::c_int as libc::c_char;
            j = j.wrapping_add(p);
        }
        loop {
            i = i.wrapping_add(1);
            if !(i < size
                && *sieve.offset(i as isize) as libc::c_int == 0 as libc::c_int)
            {
                break;
            }
        }
    }
    output_primes(prime_list, nprimes);
    free(sieve as *mut libc::c_void);
    free(prime_list as *mut libc::c_void);
    if ferror(stdout) + fclose(stdout) != 0 {
        fprintf(
            stderr,
            b"write error: %s\n\0" as *const u8 as *const libc::c_char,
            strerror(*__errno_location()),
        );
        return 1 as libc::c_int;
    }
    return 0 as libc::c_int;
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
