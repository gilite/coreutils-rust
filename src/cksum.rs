#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![feature(extern_types)]
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    static mut stdout: *mut FILE;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn __uflow(_: *mut FILE) -> libc::c_int;
    fn __overflow(_: *mut FILE, _: libc::c_int) -> libc::c_int;
    fn fread_unlocked(
        __ptr: *mut libc::c_void,
        __size: size_t,
        __n: size_t,
        __stream: *mut FILE,
    ) -> size_t;
    fn fwrite_unlocked(
        __ptr: *const libc::c_void,
        __size: size_t,
        __n: size_t,
        __stream: *mut FILE,
    ) -> size_t;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn __errno_location() -> *mut libc::c_int;
    fn umaxtostr(_: uintmax_t, _: *mut libc::c_char) -> *mut libc::c_char;
    fn dcgettext(
        __domainname: *const libc::c_char,
        __msgid: *const libc::c_char,
        __category: libc::c_int,
    ) -> *mut libc::c_char;
    fn error(
        __status: libc::c_int,
        __errnum: libc::c_int,
        __format: *const libc::c_char,
        _: ...
    );
    static mut cksum_debug: bool;
    static crctab: [[uint_fast32_t; 256]; 8];
}
pub type size_t = libc::c_ulong;
pub type __uint32_t = libc::c_uint;
pub type __uintmax_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
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
pub type uint32_t = __uint32_t;
pub type uint_fast32_t = libc::c_ulong;
pub type uintmax_t = __uintmax_t;
#[inline]
unsafe extern "C" fn getc_unlocked(mut __fp: *mut FILE) -> libc::c_int {
    return if ((*__fp)._IO_read_ptr >= (*__fp)._IO_read_end) as libc::c_int
        as libc::c_long != 0
    {
        __uflow(__fp)
    } else {
        let fresh0 = (*__fp)._IO_read_ptr;
        (*__fp)._IO_read_ptr = ((*__fp)._IO_read_ptr).offset(1);
        *(fresh0 as *mut libc::c_uchar) as libc::c_int
    };
}
#[inline]
unsafe extern "C" fn putc_unlocked(
    mut __c: libc::c_int,
    mut __stream: *mut FILE,
) -> libc::c_int {
    return if ((*__stream)._IO_write_ptr >= (*__stream)._IO_write_end) as libc::c_int
        as libc::c_long != 0
    {
        __overflow(__stream, __c as libc::c_uchar as libc::c_int)
    } else {
        let fresh1 = (*__stream)._IO_write_ptr;
        (*__stream)._IO_write_ptr = ((*__stream)._IO_write_ptr).offset(1);
        *fresh1 = __c as libc::c_char;
        *fresh1 as libc::c_uchar as libc::c_int
    };
}
#[inline]
unsafe extern "C" fn putchar_unlocked(mut __c: libc::c_int) -> libc::c_int {
    return if ((*stdout)._IO_write_ptr >= (*stdout)._IO_write_end) as libc::c_int
        as libc::c_long != 0
    {
        __overflow(stdout, __c as libc::c_uchar as libc::c_int)
    } else {
        let fresh2 = (*stdout)._IO_write_ptr;
        (*stdout)._IO_write_ptr = ((*stdout)._IO_write_ptr).offset(1);
        *fresh2 = __c as libc::c_char;
        *fresh2 as libc::c_uchar as libc::c_int
    };
}
#[inline]
unsafe extern "C" fn feof_unlocked(mut __stream: *mut FILE) -> libc::c_int {
    return ((*__stream)._flags & 0x10 as libc::c_int != 0 as libc::c_int) as libc::c_int;
}
#[inline]
unsafe extern "C" fn ferror_unlocked(mut __stream: *mut FILE) -> libc::c_int {
    return ((*__stream)._flags & 0x20 as libc::c_int != 0 as libc::c_int) as libc::c_int;
}
#[inline]
unsafe extern "C" fn __bswap_32(mut __bsx: __uint32_t) -> __uint32_t {
    return (__bsx & 0xff000000 as libc::c_uint) >> 24 as libc::c_int
        | (__bsx & 0xff0000 as libc::c_uint) >> 8 as libc::c_int
        | (__bsx & 0xff00 as libc::c_uint) << 8 as libc::c_int
        | (__bsx & 0xff as libc::c_uint) << 24 as libc::c_int;
}
static mut cksum_fp: Option::<
    unsafe extern "C" fn(*mut FILE, *mut uint_fast32_t, *mut uintmax_t) -> bool,
> = None;
unsafe extern "C" fn pclmul_supported() -> bool {
    if cksum_debug {
        error(
            0 as libc::c_int,
            0 as libc::c_int,
            b"%s\0" as *const u8 as *const libc::c_char,
            dcgettext(
                0 as *const libc::c_char,
                b"using generic hardware support\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
    }
    return 0 as libc::c_int != 0;
}
unsafe extern "C" fn cksum_slice8(
    mut fp: *mut FILE,
    mut crc_out: *mut uint_fast32_t,
    mut length_out: *mut uintmax_t,
) -> bool {
    let mut buf: [uint32_t; 16384] = [0; 16384];
    let mut crc: uint_fast32_t = 0 as libc::c_int as uint_fast32_t;
    let mut length: uintmax_t = 0 as libc::c_int as uintmax_t;
    let mut bytes_read: size_t = 0;
    if fp.is_null() || crc_out.is_null() || length_out.is_null() {
        return 0 as libc::c_int != 0;
    }
    loop {
        bytes_read = (if 0 != 0 && 0 != 0
            && (1 as libc::c_int as size_t)
                .wrapping_mul(((1 as libc::c_int) << 16 as libc::c_int) as size_t)
                <= 8 as libc::c_int as libc::c_ulong
            && 1 as libc::c_int as size_t != 0 as libc::c_int as libc::c_ulong
        {
            ({
                let mut __ptr: *mut libc::c_char = buf.as_mut_ptr() as *mut libc::c_char;
                let mut __stream: *mut FILE = fp;
                let mut __cnt: size_t = 0;
                __cnt = (1 as libc::c_int as size_t)
                    .wrapping_mul(((1 as libc::c_int) << 16 as libc::c_int) as size_t);
                while __cnt > 0 as libc::c_int as libc::c_ulong {
                    let mut __c: libc::c_int = getc_unlocked(__stream);
                    if __c == -(1 as libc::c_int) {
                        break;
                    }
                    let fresh3 = __ptr;
                    __ptr = __ptr.offset(1);
                    *fresh3 = __c as libc::c_char;
                    __cnt = __cnt.wrapping_sub(1);
                }
                (1 as libc::c_int as size_t)
                    .wrapping_mul(((1 as libc::c_int) << 16 as libc::c_int) as size_t)
                    .wrapping_sub(__cnt)
                    .wrapping_div(1 as libc::c_int as size_t)
            })
        } else {
            (if 0 != 0 && 1 as libc::c_int as size_t == 0 as libc::c_int as libc::c_ulong
                || 0 != 0
                    && ((1 as libc::c_int) << 16 as libc::c_int) as size_t
                        == 0 as libc::c_int as libc::c_ulong
            {
                0 as libc::c_int as size_t
            } else {
                fread_unlocked(
                    buf.as_mut_ptr() as *mut libc::c_void,
                    1 as libc::c_int as size_t,
                    ((1 as libc::c_int) << 16 as libc::c_int) as size_t,
                    fp,
                )
            })
        });
        if !(bytes_read > 0 as libc::c_int as libc::c_ulong) {
            break;
        }
        let mut datap: *mut uint32_t = 0 as *mut uint32_t;
        if length.wrapping_add(bytes_read) < length {
            *__errno_location() = 75 as libc::c_int;
            return 0 as libc::c_int != 0;
        }
        length = (length as libc::c_ulong).wrapping_add(bytes_read) as uintmax_t
            as uintmax_t;
        if bytes_read == 0 as libc::c_int as libc::c_ulong {
            if ferror_unlocked(fp) != 0 {
                return 0 as libc::c_int != 0;
            }
        }
        datap = buf.as_mut_ptr();
        while bytes_read >= 8 as libc::c_int as libc::c_ulong {
            let fresh4 = datap;
            datap = datap.offset(1);
            let mut first: uint32_t = *fresh4;
            let fresh5 = datap;
            datap = datap.offset(1);
            let mut second: uint32_t = *fresh5;
            crc ^= __bswap_32(first) as libc::c_ulong;
            second = __bswap_32(second);
            crc = crctab[7 as libc::c_int
                as usize][(crc >> 24 as libc::c_int
                & 0xff as libc::c_int as libc::c_ulong) as usize]
                ^ crctab[6 as libc::c_int
                    as usize][(crc >> 16 as libc::c_int
                    & 0xff as libc::c_int as libc::c_ulong) as usize]
                ^ crctab[5 as libc::c_int
                    as usize][(crc >> 8 as libc::c_int
                    & 0xff as libc::c_int as libc::c_ulong) as usize]
                ^ crctab[4 as libc::c_int
                    as usize][(crc & 0xff as libc::c_int as libc::c_ulong) as usize]
                ^ crctab[3 as libc::c_int
                    as usize][(second >> 24 as libc::c_int
                    & 0xff as libc::c_int as libc::c_uint) as usize]
                ^ crctab[2 as libc::c_int
                    as usize][(second >> 16 as libc::c_int
                    & 0xff as libc::c_int as libc::c_uint) as usize]
                ^ crctab[1 as libc::c_int
                    as usize][(second >> 8 as libc::c_int
                    & 0xff as libc::c_int as libc::c_uint) as usize]
                ^ crctab[0 as libc::c_int
                    as usize][(second & 0xff as libc::c_int as libc::c_uint) as usize];
            bytes_read = (bytes_read as libc::c_ulong)
                .wrapping_sub(8 as libc::c_int as libc::c_ulong) as size_t as size_t;
        }
        let mut cp: *mut libc::c_uchar = datap as *mut libc::c_uchar;
        loop {
            let fresh6 = bytes_read;
            bytes_read = bytes_read.wrapping_sub(1);
            if !(fresh6 != 0) {
                break;
            }
            let fresh7 = cp;
            cp = cp.offset(1);
            crc = crc << 8 as libc::c_int
                ^ crctab[0 as libc::c_int
                    as usize][((crc >> 24 as libc::c_int ^ *fresh7 as libc::c_ulong)
                    & 0xff as libc::c_int as libc::c_ulong) as usize];
        }
        if feof_unlocked(fp) != 0 {
            break;
        }
    }
    *crc_out = crc;
    *length_out = length;
    return 1 as libc::c_int != 0;
}
#[no_mangle]
pub unsafe extern "C" fn crc_sum_stream(
    mut stream: *mut FILE,
    mut resstream: *mut libc::c_void,
    mut length: *mut uintmax_t,
) -> libc::c_int {
    let mut total_bytes: uintmax_t = 0 as libc::c_int as uintmax_t;
    let mut crc: uint_fast32_t = 0 as libc::c_int as uint_fast32_t;
    if cksum_fp.is_none() {
        if pclmul_supported() {
            cksum_fp = Some(
                cksum_slice8
                    as unsafe extern "C" fn(
                        *mut FILE,
                        *mut uint_fast32_t,
                        *mut uintmax_t,
                    ) -> bool,
            );
        } else {
            cksum_fp = Some(
                cksum_slice8
                    as unsafe extern "C" fn(
                        *mut FILE,
                        *mut uint_fast32_t,
                        *mut uintmax_t,
                    ) -> bool,
            );
        }
    }
    if !cksum_fp.expect("non-null function pointer")(stream, &mut crc, &mut total_bytes)
    {
        return -(1 as libc::c_int);
    }
    *length = total_bytes;
    while total_bytes != 0 {
        crc = crc << 8 as libc::c_int
            ^ crctab[0 as libc::c_int
                as usize][((crc >> 24 as libc::c_int ^ total_bytes)
                & 0xff as libc::c_int as libc::c_ulong) as usize];
        total_bytes >>= 8 as libc::c_int;
    }
    crc = !crc & 0xffffffff as libc::c_uint as libc::c_ulong;
    let mut crc_out: libc::c_uint = crc as libc::c_uint;
    memcpy(
        resstream,
        &mut crc_out as *mut libc::c_uint as *const libc::c_void,
        ::core::mem::size_of::<libc::c_uint>() as libc::c_ulong,
    );
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn output_crc(
    mut file: *const libc::c_char,
    mut binary_file: libc::c_int,
    mut digest: *const libc::c_void,
    mut raw: bool,
    mut tagged: bool,
    mut delim: libc::c_uchar,
    mut args: bool,
    mut length: uintmax_t,
) {
    if raw {
        let mut out_int: uint32_t = __bswap_32(*(digest as *mut uint32_t));
        if 0 != 0 && 0 != 0
            && (1 as libc::c_int as size_t)
                .wrapping_mul((32 as libc::c_int / 8 as libc::c_int) as size_t)
                <= 8 as libc::c_int as libc::c_ulong
            && 1 as libc::c_int as size_t != 0 as libc::c_int as libc::c_ulong
        {
            ({
                let mut __ptr: *const libc::c_char = &mut out_int as *mut uint32_t
                    as *const libc::c_char;
                let mut __stream: *mut FILE = stdout;
                let mut __cnt: size_t = 0;
                __cnt = (1 as libc::c_int as size_t)
                    .wrapping_mul((32 as libc::c_int / 8 as libc::c_int) as size_t);
                while __cnt > 0 as libc::c_int as libc::c_ulong {
                    let fresh8 = __ptr;
                    __ptr = __ptr.offset(1);
                    if putc_unlocked(*fresh8 as libc::c_int, __stream)
                        == -(1 as libc::c_int)
                    {
                        break;
                    }
                    __cnt = __cnt.wrapping_sub(1);
                }
                compile_error!("Binary expression is not supposed to be used")
            });
        } else {
            if 0 != 0 && 1 as libc::c_int as size_t == 0 as libc::c_int as libc::c_ulong
                || 0 != 0
                    && (32 as libc::c_int / 8 as libc::c_int) as size_t
                        == 0 as libc::c_int as libc::c_ulong
            {} else {
                fwrite_unlocked(
                    &mut out_int as *mut uint32_t as *const libc::c_void,
                    1 as libc::c_int as size_t,
                    (32 as libc::c_int / 8 as libc::c_int) as size_t,
                    stdout,
                );
            };
        };
        return;
    }
    let mut length_buf: [libc::c_char; 21] = [0; 21];
    printf(
        b"%u %s\0" as *const u8 as *const libc::c_char,
        *(digest as *mut libc::c_uint),
        umaxtostr(length, length_buf.as_mut_ptr()),
    );
    if args {
        printf(b" %s\0" as *const u8 as *const libc::c_char, file);
    }
    putchar_unlocked(delim as libc::c_int);
}
