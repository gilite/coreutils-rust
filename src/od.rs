#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![feature(extern_types, label_break_value)]
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    static mut stdin: *mut FILE;
    static mut stdout: *mut FILE;
    static mut stderr: *mut FILE;
    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
    fn setvbuf(
        __stream: *mut FILE,
        __buf: *mut libc::c_char,
        __modes: libc::c_int,
        __n: size_t,
    ) -> libc::c_int;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn fgetc(__stream: *mut FILE) -> libc::c_int;
    fn __uflow(_: *mut FILE) -> libc::c_int;
    fn __overflow(_: *mut FILE, _: libc::c_int) -> libc::c_int;
    fn fputs_unlocked(__s: *const libc::c_char, __stream: *mut FILE) -> libc::c_int;
    fn fread_unlocked(
        __ptr: *mut libc::c_void,
        __size: size_t,
        __n: size_t,
        __stream: *mut FILE,
    ) -> size_t;
    fn clearerr_unlocked(__stream: *mut FILE);
    fn fileno(__stream: *mut FILE) -> libc::c_int;
    fn rpl_fclose(stream: *mut FILE) -> libc::c_int;
    fn rpl_fseeko(fp: *mut FILE, offset: off_t, whence: libc::c_int) -> libc::c_int;
    static mut optarg: *mut libc::c_char;
    static mut optind: libc::c_int;
    fn getopt_long(
        ___argc: libc::c_int,
        ___argv: *const *mut libc::c_char,
        __shortopts: *const libc::c_char,
        __longopts: *const option,
        __longind: *mut libc::c_int,
    ) -> libc::c_int;
    fn fstat(__fd: libc::c_int, __buf: *mut stat) -> libc::c_int;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn memcmp(
        _: *const libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strncmp(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn free(_: *mut libc::c_void);
    fn __errno_location() -> *mut libc::c_int;
    fn abort() -> !;
    fn atexit(__func: Option::<unsafe extern "C" fn() -> ()>) -> libc::c_int;
    fn exit(_: libc::c_int) -> !;
    fn quotearg_n_style_colon(
        n: libc::c_int,
        s: quoting_style,
        arg: *const libc::c_char,
    ) -> *mut libc::c_char;
    static mut Version: *const libc::c_char;
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    fn setlocale(
        __category: libc::c_int,
        __locale: *const libc::c_char,
    ) -> *mut libc::c_char;
    fn localeconv() -> *mut lconv;
    fn dcgettext(
        __domainname: *const libc::c_char,
        __msgid: *const libc::c_char,
        __category: libc::c_int,
    ) -> *mut libc::c_char;
    fn textdomain(__domainname: *const libc::c_char) -> *mut libc::c_char;
    fn bindtextdomain(
        __domainname: *const libc::c_char,
        __dirname: *const libc::c_char,
    ) -> *mut libc::c_char;
    fn xmalloc(s: size_t) -> *mut libc::c_void;
    fn x2realloc(p: *mut libc::c_void, ps: *mut size_t) -> *mut libc::c_void;
    fn x2nrealloc(p: *mut libc::c_void, pn: *mut size_t, s: size_t) -> *mut libc::c_void;
    fn xnmalloc(n: size_t, s: size_t) -> *mut libc::c_void;
    fn close_stdout();
    fn version_etc(
        stream: *mut FILE,
        command_name: *const libc::c_char,
        package: *const libc::c_char,
        version: *const libc::c_char,
        _: ...
    );
    static mut program_name: *const libc::c_char;
    fn set_program_name(argv0: *const libc::c_char);
    static mut argmatch_die: argmatch_exit_fn;
    fn quote(arg: *const libc::c_char) -> *const libc::c_char;
    fn __xargmatch_internal(
        context: *const libc::c_char,
        arg: *const libc::c_char,
        arglist: *const *const libc::c_char,
        vallist: *const libc::c_void,
        valsize: size_t,
        exit_fn: argmatch_exit_fn,
        allow_abbreviation: bool,
    ) -> ptrdiff_t;
    fn error(
        __status: libc::c_int,
        __errnum: libc::c_int,
        __format: *const libc::c_char,
        _: ...
    );
    fn ftoastr(
        buf: *mut libc::c_char,
        bufsize: size_t,
        flags: libc::c_int,
        width: libc::c_int,
        x: libc::c_float,
    ) -> libc::c_int;
    fn dtoastr(
        buf: *mut libc::c_char,
        bufsize: size_t,
        flags: libc::c_int,
        width: libc::c_int,
        x: libc::c_double,
    ) -> libc::c_int;
    fn ldtoastr(
        buf: *mut libc::c_char,
        bufsize: size_t,
        flags: libc::c_int,
        width: libc::c_int,
        x: f128::f128,
    ) -> libc::c_int;
    fn xprintf(format: *const libc::c_char, _: ...) -> libc::c_int;
    fn xstrtoumax(
        _: *const libc::c_char,
        _: *mut *mut libc::c_char,
        _: libc::c_int,
        _: *mut uintmax_t,
        _: *const libc::c_char,
    ) -> strtol_error;
    fn xstrtol_fatal(
        _: strtol_error,
        _: libc::c_int,
        _: libc::c_char,
        _: *const option,
        _: *const libc::c_char,
    );
}
pub type size_t = libc::c_ulong;
pub type __uintmax_t = libc::c_ulong;
pub type __dev_t = libc::c_ulong;
pub type __uid_t = libc::c_uint;
pub type __gid_t = libc::c_uint;
pub type __ino_t = libc::c_ulong;
pub type __mode_t = libc::c_uint;
pub type __nlink_t = libc::c_uint;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __time_t = libc::c_long;
pub type __blksize_t = libc::c_int;
pub type __blkcnt_t = libc::c_long;
pub type __syscall_slong_t = libc::c_long;
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
pub type off_t = __off_t;
pub type ptrdiff_t = libc::c_long;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct option {
    pub name: *const libc::c_char,
    pub has_arg: libc::c_int,
    pub flag: *mut libc::c_int,
    pub val: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stat {
    pub st_dev: __dev_t,
    pub st_ino: __ino_t,
    pub st_mode: __mode_t,
    pub st_nlink: __nlink_t,
    pub st_uid: __uid_t,
    pub st_gid: __gid_t,
    pub st_rdev: __dev_t,
    pub __pad1: __dev_t,
    pub st_size: __off_t,
    pub st_blksize: __blksize_t,
    pub __pad2: libc::c_int,
    pub st_blocks: __blkcnt_t,
    pub st_atim: timespec,
    pub st_mtim: timespec,
    pub st_ctim: timespec,
    pub __glibc_reserved: [libc::c_int; 2],
}
pub type uintmax_t = __uintmax_t;
pub type C2RustUnnamed = libc::c_uint;
pub const _ISalnum: C2RustUnnamed = 8;
pub const _ISpunct: C2RustUnnamed = 4;
pub const _IScntrl: C2RustUnnamed = 2;
pub const _ISblank: C2RustUnnamed = 1;
pub const _ISgraph: C2RustUnnamed = 32768;
pub const _ISprint: C2RustUnnamed = 16384;
pub const _ISspace: C2RustUnnamed = 8192;
pub const _ISxdigit: C2RustUnnamed = 4096;
pub const _ISdigit: C2RustUnnamed = 2048;
pub const _ISalpha: C2RustUnnamed = 1024;
pub const _ISlower: C2RustUnnamed = 512;
pub const _ISupper: C2RustUnnamed = 256;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct lconv {
    pub decimal_point: *mut libc::c_char,
    pub thousands_sep: *mut libc::c_char,
    pub grouping: *mut libc::c_char,
    pub int_curr_symbol: *mut libc::c_char,
    pub currency_symbol: *mut libc::c_char,
    pub mon_decimal_point: *mut libc::c_char,
    pub mon_thousands_sep: *mut libc::c_char,
    pub mon_grouping: *mut libc::c_char,
    pub positive_sign: *mut libc::c_char,
    pub negative_sign: *mut libc::c_char,
    pub int_frac_digits: libc::c_char,
    pub frac_digits: libc::c_char,
    pub p_cs_precedes: libc::c_char,
    pub p_sep_by_space: libc::c_char,
    pub n_cs_precedes: libc::c_char,
    pub n_sep_by_space: libc::c_char,
    pub p_sign_posn: libc::c_char,
    pub n_sign_posn: libc::c_char,
    pub int_p_cs_precedes: libc::c_char,
    pub int_p_sep_by_space: libc::c_char,
    pub int_n_cs_precedes: libc::c_char,
    pub int_n_sep_by_space: libc::c_char,
    pub int_p_sign_posn: libc::c_char,
    pub int_n_sign_posn: libc::c_char,
}
pub type C2RustUnnamed_0 = libc::c_int;
pub const GETOPT_VERSION_CHAR: C2RustUnnamed_0 = -3;
pub const GETOPT_HELP_CHAR: C2RustUnnamed_0 = -2;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct infomap {
    pub program: *const libc::c_char,
    pub node: *const libc::c_char,
}
pub type quoting_style = libc::c_uint;
pub const custom_quoting_style: quoting_style = 10;
pub const clocale_quoting_style: quoting_style = 9;
pub const locale_quoting_style: quoting_style = 8;
pub const escape_quoting_style: quoting_style = 7;
pub const c_maybe_quoting_style: quoting_style = 6;
pub const c_quoting_style: quoting_style = 5;
pub const shell_escape_always_quoting_style: quoting_style = 4;
pub const shell_escape_quoting_style: quoting_style = 3;
pub const shell_always_quoting_style: quoting_style = 2;
pub const shell_quoting_style: quoting_style = 1;
pub const literal_quoting_style: quoting_style = 0;
pub type argmatch_exit_fn = Option::<unsafe extern "C" fn() -> ()>;
pub type strtol_error = libc::c_uint;
pub const LONGINT_INVALID: strtol_error = 4;
pub const LONGINT_INVALID_SUFFIX_CHAR_WITH_OVERFLOW: strtol_error = 3;
pub const LONGINT_INVALID_SUFFIX_CHAR: strtol_error = 2;
pub const LONGINT_OVERFLOW: strtol_error = 1;
pub const LONGINT_OK: strtol_error = 0;
pub type unsigned_long_long_int = libc::c_ulonglong;
pub type size_spec = libc::c_uint;
pub const N_SIZE_SPECS: size_spec = 9;
pub const FLOAT_LONG_DOUBLE: size_spec = 8;
pub const FLOAT_DOUBLE: size_spec = 7;
pub const FLOAT_SINGLE: size_spec = 6;
pub const LONG_LONG: size_spec = 5;
pub const LONG: size_spec = 4;
pub const INT: size_spec = 3;
pub const SHORT: size_spec = 2;
pub const CHAR: size_spec = 1;
pub const NO_SIZE: size_spec = 0;
pub type output_format = libc::c_uint;
pub const CHARACTER: output_format = 6;
pub const NAMED_CHARACTER: output_format = 5;
pub const FLOATING_POINT: output_format = 4;
pub const HEXADECIMAL: output_format = 3;
pub const OCTAL: output_format = 2;
pub const UNSIGNED_DECIMAL: output_format = 1;
pub const SIGNED_DECIMAL: output_format = 0;
pub type C2RustUnnamed_1 = libc::c_uint;
pub const FMT_BYTES_ALLOCATED: C2RustUnnamed_1 = 8;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tspec {
    pub fmt: output_format,
    pub size: size_spec,
    pub print_function: Option::<
        unsafe extern "C" fn(
            size_t,
            size_t,
            *const libc::c_void,
            *const libc::c_char,
            libc::c_int,
            libc::c_int,
        ) -> (),
    >,
    pub fmt_string: [libc::c_char; 8],
    pub hexl_mode_trailer: bool,
    pub field_width: libc::c_int,
    pub pad_width: libc::c_int,
}
pub type C2RustUnnamed_2 = libc::c_uint;
pub const ENDIAN_OPTION: C2RustUnnamed_2 = 257;
pub const TRADITIONAL_OPTION: C2RustUnnamed_2 = 256;
pub type endian_type = libc::c_uint;
pub const endian_big: endian_type = 1;
pub const endian_little: endian_type = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_3 {
    pub x: libc::c_schar,
    pub b: [libc::c_char; 1],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_4 {
    pub x: libc::c_uchar,
    pub b: [libc::c_char; 1],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_5 {
    pub x: libc::c_short,
    pub b: [libc::c_char; 2],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_6 {
    pub x: libc::c_ushort,
    pub b: [libc::c_char; 2],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_7 {
    pub x: libc::c_uint,
    pub b: [libc::c_char; 4],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_8 {
    pub x: libc::c_ulong,
    pub b: [libc::c_char; 8],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_9 {
    pub x: unsigned_long_long_int,
    pub b: [libc::c_char; 8],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_10 {
    pub x: libc::c_float,
    pub b: [libc::c_char; 4],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_11 {
    pub x: libc::c_double,
    pub b: [libc::c_char; 8],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_12 {
    pub x: f128::f128,
    pub b: [libc::c_char; 16],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_13 {
    pub _gl_dummy: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_14 {
    pub _gl_dummy: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_15 {
    pub _gl_dummy: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_16 {
    pub _gl_dummy: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_17 {
    pub _gl_dummy: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_18 {
    pub _gl_dummy: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_19 {
    pub _gl_dummy: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_20 {
    pub _gl_dummy: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_21 {
    pub _gl_dummy: libc::c_int,
}
#[inline]
unsafe extern "C" fn ferror_unlocked(mut __stream: *mut FILE) -> libc::c_int {
    return ((*__stream)._flags & 0x20 as libc::c_int != 0 as libc::c_int) as libc::c_int;
}
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
unsafe extern "C" fn gcd(mut u: size_t, mut v: size_t) -> size_t {
    loop {
        let mut t: size_t = u.wrapping_rem(v);
        u = v;
        v = t;
        if !(v != 0) {
            break;
        }
    }
    return u;
}
#[inline]
unsafe extern "C" fn lcm(mut u: size_t, mut v: size_t) -> size_t {
    return u.wrapping_mul(v.wrapping_div(gcd(u, v)));
}
#[inline]
unsafe extern "C" fn emit_stdin_note() {
    fputs_unlocked(
        dcgettext(
            0 as *const libc::c_char,
            b"\nWith no FILE, or when FILE is -, read standard input.\n\0" as *const u8
                as *const libc::c_char,
            5 as libc::c_int,
        ),
        stdout,
    );
}
#[inline]
unsafe extern "C" fn emit_mandatory_arg_note() {
    fputs_unlocked(
        dcgettext(
            0 as *const libc::c_char,
            b"\nMandatory arguments to long options are mandatory for short options too.\n\0"
                as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        ),
        stdout,
    );
}
#[inline]
unsafe extern "C" fn emit_ancillary_info(mut program: *const libc::c_char) {
    let infomap_0: [infomap; 7] = [
        {
            let mut init = infomap {
                program: b"[\0" as *const u8 as *const libc::c_char,
                node: b"test invocation\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = infomap {
                program: b"coreutils\0" as *const u8 as *const libc::c_char,
                node: b"Multi-call invocation\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = infomap {
                program: b"sha224sum\0" as *const u8 as *const libc::c_char,
                node: b"sha2 utilities\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = infomap {
                program: b"sha256sum\0" as *const u8 as *const libc::c_char,
                node: b"sha2 utilities\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = infomap {
                program: b"sha384sum\0" as *const u8 as *const libc::c_char,
                node: b"sha2 utilities\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = infomap {
                program: b"sha512sum\0" as *const u8 as *const libc::c_char,
                node: b"sha2 utilities\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = infomap {
                program: 0 as *const libc::c_char,
                node: 0 as *const libc::c_char,
            };
            init
        },
    ];
    let mut node: *const libc::c_char = program;
    let mut map_prog: *const infomap = infomap_0.as_ptr();
    while !((*map_prog).program).is_null()
        && !(strcmp(program, (*map_prog).program) == 0 as libc::c_int)
    {
        map_prog = map_prog.offset(1);
    }
    if !((*map_prog).node).is_null() {
        node = (*map_prog).node;
    }
    printf(
        dcgettext(
            0 as *const libc::c_char,
            b"\n%s online help: <%s>\n\0" as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        ),
        b"GNU coreutils\0" as *const u8 as *const libc::c_char,
        b"https://www.gnu.org/software/coreutils/\0" as *const u8 as *const libc::c_char,
    );
    let mut lc_messages: *const libc::c_char = setlocale(
        5 as libc::c_int,
        0 as *const libc::c_char,
    );
    if !lc_messages.is_null()
        && strncmp(
            lc_messages,
            b"en_\0" as *const u8 as *const libc::c_char,
            (::core::mem::size_of::<[libc::c_char; 4]>() as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong),
        ) != 0
    {
        fputs_unlocked(
            dcgettext(
                0 as *const libc::c_char,
                b"Report any translation bugs to <https://translationproject.org/team/>\n\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            stdout,
        );
    }
    let mut url_program: *const libc::c_char = if strcmp(
        program,
        b"[\0" as *const u8 as *const libc::c_char,
    ) == 0 as libc::c_int
    {
        b"test\0" as *const u8 as *const libc::c_char
    } else {
        program
    };
    printf(
        dcgettext(
            0 as *const libc::c_char,
            b"Full documentation <%s%s>\n\0" as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        ),
        b"https://www.gnu.org/software/coreutils/\0" as *const u8 as *const libc::c_char,
        url_program,
    );
    printf(
        dcgettext(
            0 as *const libc::c_char,
            b"or available locally via: info '(coreutils) %s%s'\n\0" as *const u8
                as *const libc::c_char,
            5 as libc::c_int,
        ),
        node,
        if node == program {
            b" invocation\0" as *const u8 as *const libc::c_char
        } else {
            b"\0" as *const u8 as *const libc::c_char
        },
    );
}
#[inline]
unsafe extern "C" fn usable_st_size(mut sb: *const stat) -> bool {
    return (*sb).st_mode & 0o170000 as libc::c_int as libc::c_uint
        == 0o100000 as libc::c_int as libc::c_uint
        || (*sb).st_mode & 0o170000 as libc::c_int as libc::c_uint
            == 0o120000 as libc::c_int as libc::c_uint
        || ((*sb).st_mode).wrapping_sub((*sb).st_mode) != 0 || 0 as libc::c_int != 0;
}
#[inline]
unsafe extern "C" fn xset_binary_mode_error() {}
#[inline]
unsafe extern "C" fn xset_binary_mode(mut fd: libc::c_int, mut mode: libc::c_int) {
    if set_binary_mode(fd, mode) < 0 as libc::c_int {
        xset_binary_mode_error();
    }
}
#[inline]
unsafe extern "C" fn __gl_setmode(
    mut fd: libc::c_int,
    mut mode: libc::c_int,
) -> libc::c_int {
    return 0 as libc::c_int;
}
#[inline]
unsafe extern "C" fn set_binary_mode(
    mut fd: libc::c_int,
    mut mode: libc::c_int,
) -> libc::c_int {
    return __gl_setmode(fd, mode);
}
static mut bytes_to_oct_digits: [libc::c_uint; 17] = [
    0 as libc::c_int as libc::c_uint,
    3 as libc::c_int as libc::c_uint,
    6 as libc::c_int as libc::c_uint,
    8 as libc::c_int as libc::c_uint,
    11 as libc::c_int as libc::c_uint,
    14 as libc::c_int as libc::c_uint,
    16 as libc::c_int as libc::c_uint,
    19 as libc::c_int as libc::c_uint,
    22 as libc::c_int as libc::c_uint,
    25 as libc::c_int as libc::c_uint,
    27 as libc::c_int as libc::c_uint,
    30 as libc::c_int as libc::c_uint,
    32 as libc::c_int as libc::c_uint,
    35 as libc::c_int as libc::c_uint,
    38 as libc::c_int as libc::c_uint,
    41 as libc::c_int as libc::c_uint,
    43 as libc::c_int as libc::c_uint,
];
static mut bytes_to_signed_dec_digits: [libc::c_uint; 17] = [
    1 as libc::c_int as libc::c_uint,
    4 as libc::c_int as libc::c_uint,
    6 as libc::c_int as libc::c_uint,
    8 as libc::c_int as libc::c_uint,
    11 as libc::c_int as libc::c_uint,
    13 as libc::c_int as libc::c_uint,
    16 as libc::c_int as libc::c_uint,
    18 as libc::c_int as libc::c_uint,
    20 as libc::c_int as libc::c_uint,
    23 as libc::c_int as libc::c_uint,
    25 as libc::c_int as libc::c_uint,
    28 as libc::c_int as libc::c_uint,
    30 as libc::c_int as libc::c_uint,
    33 as libc::c_int as libc::c_uint,
    35 as libc::c_int as libc::c_uint,
    37 as libc::c_int as libc::c_uint,
    40 as libc::c_int as libc::c_uint,
];
static mut bytes_to_unsigned_dec_digits: [libc::c_uint; 17] = [
    0 as libc::c_int as libc::c_uint,
    3 as libc::c_int as libc::c_uint,
    5 as libc::c_int as libc::c_uint,
    8 as libc::c_int as libc::c_uint,
    10 as libc::c_int as libc::c_uint,
    13 as libc::c_int as libc::c_uint,
    15 as libc::c_int as libc::c_uint,
    17 as libc::c_int as libc::c_uint,
    20 as libc::c_int as libc::c_uint,
    22 as libc::c_int as libc::c_uint,
    25 as libc::c_int as libc::c_uint,
    27 as libc::c_int as libc::c_uint,
    29 as libc::c_int as libc::c_uint,
    32 as libc::c_int as libc::c_uint,
    34 as libc::c_int as libc::c_uint,
    37 as libc::c_int as libc::c_uint,
    39 as libc::c_int as libc::c_uint,
];
static mut bytes_to_hex_digits: [libc::c_uint; 17] = [
    0 as libc::c_int as libc::c_uint,
    2 as libc::c_int as libc::c_uint,
    4 as libc::c_int as libc::c_uint,
    6 as libc::c_int as libc::c_uint,
    8 as libc::c_int as libc::c_uint,
    10 as libc::c_int as libc::c_uint,
    12 as libc::c_int as libc::c_uint,
    14 as libc::c_int as libc::c_uint,
    16 as libc::c_int as libc::c_uint,
    18 as libc::c_int as libc::c_uint,
    20 as libc::c_int as libc::c_uint,
    22 as libc::c_int as libc::c_uint,
    24 as libc::c_int as libc::c_uint,
    26 as libc::c_int as libc::c_uint,
    28 as libc::c_int as libc::c_uint,
    30 as libc::c_int as libc::c_uint,
    32 as libc::c_int as libc::c_uint,
];
static mut width_bytes: [libc::c_int; 9] = [
    -(1 as libc::c_int),
    ::core::mem::size_of::<libc::c_char>() as libc::c_ulong as libc::c_int,
    ::core::mem::size_of::<libc::c_short>() as libc::c_ulong as libc::c_int,
    ::core::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_int,
    ::core::mem::size_of::<libc::c_long>() as libc::c_ulong as libc::c_int,
    ::core::mem::size_of::<unsigned_long_long_int>() as libc::c_ulong as libc::c_int,
    ::core::mem::size_of::<libc::c_float>() as libc::c_ulong as libc::c_int,
    ::core::mem::size_of::<libc::c_double>() as libc::c_ulong as libc::c_int,
    ::core::mem::size_of::<f128::f128>() as libc::c_ulong as libc::c_int,
];
static mut charname: [[libc::c_char; 4]; 33] = unsafe {
    [
        *::core::mem::transmute::<&[u8; 4], &[libc::c_char; 4]>(b"nul\0"),
        *::core::mem::transmute::<&[u8; 4], &[libc::c_char; 4]>(b"soh\0"),
        *::core::mem::transmute::<&[u8; 4], &[libc::c_char; 4]>(b"stx\0"),
        *::core::mem::transmute::<&[u8; 4], &[libc::c_char; 4]>(b"etx\0"),
        *::core::mem::transmute::<&[u8; 4], &[libc::c_char; 4]>(b"eot\0"),
        *::core::mem::transmute::<&[u8; 4], &[libc::c_char; 4]>(b"enq\0"),
        *::core::mem::transmute::<&[u8; 4], &[libc::c_char; 4]>(b"ack\0"),
        *::core::mem::transmute::<&[u8; 4], &[libc::c_char; 4]>(b"bel\0"),
        *::core::mem::transmute::<&[u8; 4], &[libc::c_char; 4]>(b"bs\0\0"),
        *::core::mem::transmute::<&[u8; 4], &[libc::c_char; 4]>(b"ht\0\0"),
        *::core::mem::transmute::<&[u8; 4], &[libc::c_char; 4]>(b"nl\0\0"),
        *::core::mem::transmute::<&[u8; 4], &[libc::c_char; 4]>(b"vt\0\0"),
        *::core::mem::transmute::<&[u8; 4], &[libc::c_char; 4]>(b"ff\0\0"),
        *::core::mem::transmute::<&[u8; 4], &[libc::c_char; 4]>(b"cr\0\0"),
        *::core::mem::transmute::<&[u8; 4], &[libc::c_char; 4]>(b"so\0\0"),
        *::core::mem::transmute::<&[u8; 4], &[libc::c_char; 4]>(b"si\0\0"),
        *::core::mem::transmute::<&[u8; 4], &[libc::c_char; 4]>(b"dle\0"),
        *::core::mem::transmute::<&[u8; 4], &[libc::c_char; 4]>(b"dc1\0"),
        *::core::mem::transmute::<&[u8; 4], &[libc::c_char; 4]>(b"dc2\0"),
        *::core::mem::transmute::<&[u8; 4], &[libc::c_char; 4]>(b"dc3\0"),
        *::core::mem::transmute::<&[u8; 4], &[libc::c_char; 4]>(b"dc4\0"),
        *::core::mem::transmute::<&[u8; 4], &[libc::c_char; 4]>(b"nak\0"),
        *::core::mem::transmute::<&[u8; 4], &[libc::c_char; 4]>(b"syn\0"),
        *::core::mem::transmute::<&[u8; 4], &[libc::c_char; 4]>(b"etb\0"),
        *::core::mem::transmute::<&[u8; 4], &[libc::c_char; 4]>(b"can\0"),
        *::core::mem::transmute::<&[u8; 4], &[libc::c_char; 4]>(b"em\0\0"),
        *::core::mem::transmute::<&[u8; 4], &[libc::c_char; 4]>(b"sub\0"),
        *::core::mem::transmute::<&[u8; 4], &[libc::c_char; 4]>(b"esc\0"),
        *::core::mem::transmute::<&[u8; 4], &[libc::c_char; 4]>(b"fs\0\0"),
        *::core::mem::transmute::<&[u8; 4], &[libc::c_char; 4]>(b"gs\0\0"),
        *::core::mem::transmute::<&[u8; 4], &[libc::c_char; 4]>(b"rs\0\0"),
        *::core::mem::transmute::<&[u8; 4], &[libc::c_char; 4]>(b"us\0\0"),
        *::core::mem::transmute::<&[u8; 4], &[libc::c_char; 4]>(b"sp\0\0"),
    ]
};
static mut address_base: libc::c_int = 0;
static mut address_pad_len: libc::c_int = 0;
static mut string_min: size_t = 0;
static mut flag_dump_strings: bool = false;
static mut traditional: bool = false;
static mut flag_pseudo_start: bool = false;
static mut pseudo_offset: uintmax_t = 0;
static mut format_address: Option::<
    unsafe extern "C" fn(uintmax_t, libc::c_char) -> (),
> = None;
static mut n_bytes_to_skip: uintmax_t = 0 as libc::c_int as uintmax_t;
static mut limit_bytes_to_format: bool = 0 as libc::c_int != 0;
static mut max_bytes_to_format: uintmax_t = 0;
static mut end_offset: uintmax_t = 0;
static mut abbreviate_duplicate_blocks: bool = 1 as libc::c_int != 0;
static mut spec: *mut tspec = 0 as *const tspec as *mut tspec;
static mut n_specs: size_t = 0;
static mut n_specs_allocated: size_t = 0;
static mut bytes_per_block: size_t = 0;
static mut input_filename: *const libc::c_char = 0 as *const libc::c_char;
static mut file_list: *const *const libc::c_char = 0 as *const *const libc::c_char;
static mut default_file_list: [*const libc::c_char; 2] = [
    b"-\0" as *const u8 as *const libc::c_char,
    0 as *const libc::c_char,
];
static mut in_stream: *mut FILE = 0 as *const FILE as *mut FILE;
static mut have_read_stdin: bool = false;
static mut integral_type_size: [size_spec; 9] = [NO_SIZE; 9];
static mut fp_type_size: [size_spec; 17] = [NO_SIZE; 17];
static mut input_swap: bool = false;
static mut short_options: [libc::c_char; 35] = unsafe {
    *::core::mem::transmute::<
        &[u8; 35],
        &[libc::c_char; 35],
    >(b"A:aBbcDdeFfHhIij:LlN:OoS:st:vw::Xx\0")
};
static mut endian_args: [*const libc::c_char; 3] = [
    b"little\0" as *const u8 as *const libc::c_char,
    b"big\0" as *const u8 as *const libc::c_char,
    0 as *const libc::c_char,
];
static mut endian_types: [endian_type; 2] = [endian_little, endian_big];
static mut long_options: [option; 12] = [
    {
        let mut init = option {
            name: b"skip-bytes\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'j' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"address-radix\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'A' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"read-bytes\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'N' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"format\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 't' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"output-duplicates\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'v' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"strings\0" as *const u8 as *const libc::c_char,
            has_arg: 2 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'S' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"traditional\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: TRADITIONAL_OPTION as libc::c_int,
        };
        init
    },
    {
        let mut init = option {
            name: b"width\0" as *const u8 as *const libc::c_char,
            has_arg: 2 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'w' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"endian\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: ENDIAN_OPTION as libc::c_int,
        };
        init
    },
    {
        let mut init = option {
            name: b"help\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: GETOPT_HELP_CHAR as libc::c_int,
        };
        init
    },
    {
        let mut init = option {
            name: b"version\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: GETOPT_VERSION_CHAR as libc::c_int,
        };
        init
    },
    {
        let mut init = option {
            name: 0 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 0 as libc::c_int,
        };
        init
    },
];
#[no_mangle]
pub unsafe extern "C" fn usage(mut status: libc::c_int) {
    if status != 0 as libc::c_int {
        fprintf(
            stderr,
            dcgettext(
                0 as *const libc::c_char,
                b"Try '%s --help' for more information.\n\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
            program_name,
        );
    } else {
        printf(
            dcgettext(
                0 as *const libc::c_char,
                b"Usage: %s [OPTION]... [FILE]...\n  or:  %s [-abcdfilosx]... [FILE] [[+]OFFSET[.][b]]\n  or:  %s --traditional [OPTION]... [FILE] [[+]OFFSET[.][b] [+][LABEL][.][b]]\n\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            program_name,
            program_name,
            program_name,
        );
        fputs_unlocked(
            dcgettext(
                0 as *const libc::c_char,
                b"\nWrite an unambiguous representation, octal bytes by default,\nof FILE to standard output.  With more than one FILE argument,\nconcatenate them in the listed order to form the input.\n\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            stdout,
        );
        emit_stdin_note();
        fputs_unlocked(
            dcgettext(
                0 as *const libc::c_char,
                b"\nIf first and second call formats both apply, the second format is assumed\nif the last operand begins with + or (if there are 2 operands) a digit.\nAn OFFSET operand means -j OFFSET.  LABEL is the pseudo-address\nat first byte printed, incremented when dump is progressing.\nFor OFFSET and LABEL, a 0x or 0X prefix indicates hexadecimal;\nsuffixes may be . for octal and b for multiply by 512.\n\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            stdout,
        );
        emit_mandatory_arg_note();
        fputs_unlocked(
            dcgettext(
                0 as *const libc::c_char,
                b"  -A, --address-radix=RADIX   output format for file offsets; RADIX is one\n                                of [doxn], for Decimal, Octal, Hex or None\n      --endian={big|little}   swap input bytes according the specified order\n  -j, --skip-bytes=BYTES      skip BYTES input bytes first\n\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            stdout,
        );
        fputs_unlocked(
            dcgettext(
                0 as *const libc::c_char,
                b"  -N, --read-bytes=BYTES      limit dump to BYTES input bytes\n  -S BYTES, --strings[=BYTES]  output strings of at least BYTES graphic chars;\n                                3 is implied when BYTES is not specified\n  -t, --format=TYPE           select output format or formats\n  -v, --output-duplicates     do not use * to mark line suppression\n  -w[BYTES], --width[=BYTES]  output BYTES bytes per output line;\n                                32 is implied when BYTES is not specified\n      --traditional           accept arguments in third form above\n\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            stdout,
        );
        fputs_unlocked(
            dcgettext(
                0 as *const libc::c_char,
                b"      --help        display this help and exit\n\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
            stdout,
        );
        fputs_unlocked(
            dcgettext(
                0 as *const libc::c_char,
                b"      --version     output version information and exit\n\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            stdout,
        );
        fputs_unlocked(
            dcgettext(
                0 as *const libc::c_char,
                b"\n\nTraditional format specifications may be intermixed; they accumulate:\n  -a   same as -t a,  select named characters, ignoring high-order bit\n  -b   same as -t o1, select octal bytes\n  -c   same as -t c,  select printable characters or backslash escapes\n  -d   same as -t u2, select unsigned decimal 2-byte units\n\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            stdout,
        );
        fputs_unlocked(
            dcgettext(
                0 as *const libc::c_char,
                b"  -f   same as -t fF, select floats\n  -i   same as -t dI, select decimal ints\n  -l   same as -t dL, select decimal longs\n  -o   same as -t o2, select octal 2-byte units\n  -s   same as -t d2, select decimal 2-byte units\n  -x   same as -t x2, select hexadecimal 2-byte units\n\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            stdout,
        );
        fputs_unlocked(
            dcgettext(
                0 as *const libc::c_char,
                b"\n\nTYPE is made up of one or more of these specifications:\n  a          named character, ignoring high-order bit\n  c          printable character or backslash escape\n\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            stdout,
        );
        fputs_unlocked(
            dcgettext(
                0 as *const libc::c_char,
                b"  d[SIZE]    signed decimal, SIZE bytes per integer\n  f[SIZE]    floating point, SIZE bytes per float\n  o[SIZE]    octal, SIZE bytes per integer\n  u[SIZE]    unsigned decimal, SIZE bytes per integer\n  x[SIZE]    hexadecimal, SIZE bytes per integer\n\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            stdout,
        );
        fputs_unlocked(
            dcgettext(
                0 as *const libc::c_char,
                b"\nSIZE is a number.  For TYPE in [doux], SIZE may also be C for\nsizeof(char), S for sizeof(short), I for sizeof(int) or L for\nsizeof(long).  If TYPE is f, SIZE may also be F for sizeof(float), D\nfor sizeof(double) or L for sizeof(long double).\n\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            stdout,
        );
        fputs_unlocked(
            dcgettext(
                0 as *const libc::c_char,
                b"\nAdding a z suffix to any type displays printable characters at the end of\neach output line.\n\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            stdout,
        );
        fputs_unlocked(
            dcgettext(
                0 as *const libc::c_char,
                b"\n\nBYTES is hex with 0x or 0X prefix, and may have a multiplier suffix:\n  b    512\n  KB   1000\n  K    1024\n  MB   1000*1000\n  M    1024*1024\nand so on for G, T, P, E, Z, Y, R, Q.\nBinary prefixes can be used, too: KiB=K, MiB=M, and so on.\n\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            stdout,
        );
        emit_ancillary_info(b"od\0" as *const u8 as *const libc::c_char);
    }
    exit(status);
}
unsafe extern "C" fn print_s_char(
    mut fields: size_t,
    mut blank: size_t,
    mut block: *const libc::c_void,
    mut fmt_string: *const libc::c_char,
    mut width: libc::c_int,
    mut pad: libc::c_int,
) {
    let mut p: *const libc::c_schar = block as *const libc::c_schar;
    let mut i: uintmax_t = 0;
    let mut pad_remaining: libc::c_int = pad;
    i = fields;
    while blank < i {
        let mut next_pad: libc::c_int = (pad as libc::c_ulong)
            .wrapping_mul(i.wrapping_sub(1 as libc::c_int as libc::c_ulong))
            .wrapping_div(fields) as libc::c_int;
        let mut adjusted_width: libc::c_int = pad_remaining - next_pad + width;
        let mut x: libc::c_schar = 0;
        if input_swap as libc::c_int != 0
            && ::core::mem::size_of::<libc::c_schar>() as libc::c_ulong
                > 1 as libc::c_int as libc::c_ulong
        {
            let mut j: size_t = 0;
            let mut u: C2RustUnnamed_3 = C2RustUnnamed_3 { x: 0 };
            j = 0 as libc::c_int as size_t;
            while j < ::core::mem::size_of::<libc::c_schar>() as libc::c_ulong {
                u
                    .b[j
                    as usize] = *(p as *const libc::c_char)
                    .offset(
                        (::core::mem::size_of::<libc::c_schar>() as libc::c_ulong)
                            .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                            .wrapping_sub(j) as isize,
                    );
                j = j.wrapping_add(1);
            }
            x = u.x;
        } else {
            x = *p;
        }
        p = p.offset(1);
        xprintf(fmt_string, adjusted_width, x as libc::c_int);
        pad_remaining = next_pad;
        i = i.wrapping_sub(1);
    }
}
unsafe extern "C" fn print_char(
    mut fields: size_t,
    mut blank: size_t,
    mut block: *const libc::c_void,
    mut fmt_string: *const libc::c_char,
    mut width: libc::c_int,
    mut pad: libc::c_int,
) {
    let mut p: *const libc::c_uchar = block as *const libc::c_uchar;
    let mut i: uintmax_t = 0;
    let mut pad_remaining: libc::c_int = pad;
    i = fields;
    while blank < i {
        let mut next_pad: libc::c_int = (pad as libc::c_ulong)
            .wrapping_mul(i.wrapping_sub(1 as libc::c_int as libc::c_ulong))
            .wrapping_div(fields) as libc::c_int;
        let mut adjusted_width: libc::c_int = pad_remaining - next_pad + width;
        let mut x: libc::c_uchar = 0;
        if input_swap as libc::c_int != 0
            && ::core::mem::size_of::<libc::c_uchar>() as libc::c_ulong
                > 1 as libc::c_int as libc::c_ulong
        {
            let mut j: size_t = 0;
            let mut u: C2RustUnnamed_4 = C2RustUnnamed_4 { x: 0 };
            j = 0 as libc::c_int as size_t;
            while j < ::core::mem::size_of::<libc::c_uchar>() as libc::c_ulong {
                u
                    .b[j
                    as usize] = *(p as *const libc::c_char)
                    .offset(
                        (::core::mem::size_of::<libc::c_uchar>() as libc::c_ulong)
                            .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                            .wrapping_sub(j) as isize,
                    );
                j = j.wrapping_add(1);
            }
            x = u.x;
        } else {
            x = *p;
        }
        p = p.offset(1);
        xprintf(fmt_string, adjusted_width, x as libc::c_int);
        pad_remaining = next_pad;
        i = i.wrapping_sub(1);
    }
}
unsafe extern "C" fn print_s_short(
    mut fields: size_t,
    mut blank: size_t,
    mut block: *const libc::c_void,
    mut fmt_string: *const libc::c_char,
    mut width: libc::c_int,
    mut pad: libc::c_int,
) {
    let mut p: *const libc::c_short = block as *const libc::c_short;
    let mut i: uintmax_t = 0;
    let mut pad_remaining: libc::c_int = pad;
    i = fields;
    while blank < i {
        let mut next_pad: libc::c_int = (pad as libc::c_ulong)
            .wrapping_mul(i.wrapping_sub(1 as libc::c_int as libc::c_ulong))
            .wrapping_div(fields) as libc::c_int;
        let mut adjusted_width: libc::c_int = pad_remaining - next_pad + width;
        let mut x: libc::c_short = 0;
        if input_swap as libc::c_int != 0
            && ::core::mem::size_of::<libc::c_short>() as libc::c_ulong
                > 1 as libc::c_int as libc::c_ulong
        {
            let mut j: size_t = 0;
            let mut u: C2RustUnnamed_5 = C2RustUnnamed_5 { x: 0 };
            j = 0 as libc::c_int as size_t;
            while j < ::core::mem::size_of::<libc::c_short>() as libc::c_ulong {
                u
                    .b[j
                    as usize] = *(p as *const libc::c_char)
                    .offset(
                        (::core::mem::size_of::<libc::c_short>() as libc::c_ulong)
                            .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                            .wrapping_sub(j) as isize,
                    );
                j = j.wrapping_add(1);
            }
            x = u.x;
        } else {
            x = *p;
        }
        p = p.offset(1);
        xprintf(fmt_string, adjusted_width, x as libc::c_int);
        pad_remaining = next_pad;
        i = i.wrapping_sub(1);
    }
}
unsafe extern "C" fn print_short(
    mut fields: size_t,
    mut blank: size_t,
    mut block: *const libc::c_void,
    mut fmt_string: *const libc::c_char,
    mut width: libc::c_int,
    mut pad: libc::c_int,
) {
    let mut p: *const libc::c_ushort = block as *const libc::c_ushort;
    let mut i: uintmax_t = 0;
    let mut pad_remaining: libc::c_int = pad;
    i = fields;
    while blank < i {
        let mut next_pad: libc::c_int = (pad as libc::c_ulong)
            .wrapping_mul(i.wrapping_sub(1 as libc::c_int as libc::c_ulong))
            .wrapping_div(fields) as libc::c_int;
        let mut adjusted_width: libc::c_int = pad_remaining - next_pad + width;
        let mut x: libc::c_ushort = 0;
        if input_swap as libc::c_int != 0
            && ::core::mem::size_of::<libc::c_ushort>() as libc::c_ulong
                > 1 as libc::c_int as libc::c_ulong
        {
            let mut j: size_t = 0;
            let mut u: C2RustUnnamed_6 = C2RustUnnamed_6 { x: 0 };
            j = 0 as libc::c_int as size_t;
            while j < ::core::mem::size_of::<libc::c_ushort>() as libc::c_ulong {
                u
                    .b[j
                    as usize] = *(p as *const libc::c_char)
                    .offset(
                        (::core::mem::size_of::<libc::c_ushort>() as libc::c_ulong)
                            .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                            .wrapping_sub(j) as isize,
                    );
                j = j.wrapping_add(1);
            }
            x = u.x;
        } else {
            x = *p;
        }
        p = p.offset(1);
        xprintf(fmt_string, adjusted_width, x as libc::c_int);
        pad_remaining = next_pad;
        i = i.wrapping_sub(1);
    }
}
unsafe extern "C" fn print_int(
    mut fields: size_t,
    mut blank: size_t,
    mut block: *const libc::c_void,
    mut fmt_string: *const libc::c_char,
    mut width: libc::c_int,
    mut pad: libc::c_int,
) {
    let mut p: *const libc::c_uint = block as *const libc::c_uint;
    let mut i: uintmax_t = 0;
    let mut pad_remaining: libc::c_int = pad;
    i = fields;
    while blank < i {
        let mut next_pad: libc::c_int = (pad as libc::c_ulong)
            .wrapping_mul(i.wrapping_sub(1 as libc::c_int as libc::c_ulong))
            .wrapping_div(fields) as libc::c_int;
        let mut adjusted_width: libc::c_int = pad_remaining - next_pad + width;
        let mut x: libc::c_uint = 0;
        if input_swap as libc::c_int != 0
            && ::core::mem::size_of::<libc::c_uint>() as libc::c_ulong
                > 1 as libc::c_int as libc::c_ulong
        {
            let mut j: size_t = 0;
            let mut u: C2RustUnnamed_7 = C2RustUnnamed_7 { x: 0 };
            j = 0 as libc::c_int as size_t;
            while j < ::core::mem::size_of::<libc::c_uint>() as libc::c_ulong {
                u
                    .b[j
                    as usize] = *(p as *const libc::c_char)
                    .offset(
                        (::core::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                            .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                            .wrapping_sub(j) as isize,
                    );
                j = j.wrapping_add(1);
            }
            x = u.x;
        } else {
            x = *p;
        }
        p = p.offset(1);
        xprintf(fmt_string, adjusted_width, x);
        pad_remaining = next_pad;
        i = i.wrapping_sub(1);
    }
}
unsafe extern "C" fn print_long(
    mut fields: size_t,
    mut blank: size_t,
    mut block: *const libc::c_void,
    mut fmt_string: *const libc::c_char,
    mut width: libc::c_int,
    mut pad: libc::c_int,
) {
    let mut p: *const libc::c_ulong = block as *const libc::c_ulong;
    let mut i: uintmax_t = 0;
    let mut pad_remaining: libc::c_int = pad;
    i = fields;
    while blank < i {
        let mut next_pad: libc::c_int = (pad as libc::c_ulong)
            .wrapping_mul(i.wrapping_sub(1 as libc::c_int as libc::c_ulong))
            .wrapping_div(fields) as libc::c_int;
        let mut adjusted_width: libc::c_int = pad_remaining - next_pad + width;
        let mut x: libc::c_ulong = 0;
        if input_swap as libc::c_int != 0
            && ::core::mem::size_of::<libc::c_ulong>() as libc::c_ulong
                > 1 as libc::c_int as libc::c_ulong
        {
            let mut j: size_t = 0;
            let mut u: C2RustUnnamed_8 = C2RustUnnamed_8 { x: 0 };
            j = 0 as libc::c_int as size_t;
            while j < ::core::mem::size_of::<libc::c_ulong>() as libc::c_ulong {
                u
                    .b[j
                    as usize] = *(p as *const libc::c_char)
                    .offset(
                        (::core::mem::size_of::<libc::c_ulong>() as libc::c_ulong)
                            .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                            .wrapping_sub(j) as isize,
                    );
                j = j.wrapping_add(1);
            }
            x = u.x;
        } else {
            x = *p;
        }
        p = p.offset(1);
        xprintf(fmt_string, adjusted_width, x);
        pad_remaining = next_pad;
        i = i.wrapping_sub(1);
    }
}
unsafe extern "C" fn print_long_long(
    mut fields: size_t,
    mut blank: size_t,
    mut block: *const libc::c_void,
    mut fmt_string: *const libc::c_char,
    mut width: libc::c_int,
    mut pad: libc::c_int,
) {
    let mut p: *const unsigned_long_long_int = block as *const unsigned_long_long_int;
    let mut i: uintmax_t = 0;
    let mut pad_remaining: libc::c_int = pad;
    i = fields;
    while blank < i {
        let mut next_pad: libc::c_int = (pad as libc::c_ulong)
            .wrapping_mul(i.wrapping_sub(1 as libc::c_int as libc::c_ulong))
            .wrapping_div(fields) as libc::c_int;
        let mut adjusted_width: libc::c_int = pad_remaining - next_pad + width;
        let mut x: unsigned_long_long_int = 0;
        if input_swap as libc::c_int != 0
            && ::core::mem::size_of::<unsigned_long_long_int>() as libc::c_ulong
                > 1 as libc::c_int as libc::c_ulong
        {
            let mut j: size_t = 0;
            let mut u: C2RustUnnamed_9 = C2RustUnnamed_9 { x: 0 };
            j = 0 as libc::c_int as size_t;
            while j < ::core::mem::size_of::<unsigned_long_long_int>() as libc::c_ulong {
                u
                    .b[j
                    as usize] = *(p as *const libc::c_char)
                    .offset(
                        (::core::mem::size_of::<unsigned_long_long_int>()
                            as libc::c_ulong)
                            .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                            .wrapping_sub(j) as isize,
                    );
                j = j.wrapping_add(1);
            }
            x = u.x;
        } else {
            x = *p;
        }
        p = p.offset(1);
        xprintf(fmt_string, adjusted_width, x);
        pad_remaining = next_pad;
        i = i.wrapping_sub(1);
    }
}
unsafe extern "C" fn print_float(
    mut fields: size_t,
    mut blank: size_t,
    mut block: *const libc::c_void,
    mut fmt_string: *const libc::c_char,
    mut width: libc::c_int,
    mut pad: libc::c_int,
) {
    let mut p: *const libc::c_float = block as *const libc::c_float;
    let mut i: uintmax_t = 0;
    let mut pad_remaining: libc::c_int = pad;
    i = fields;
    while blank < i {
        let mut next_pad: libc::c_int = (pad as libc::c_ulong)
            .wrapping_mul(i.wrapping_sub(1 as libc::c_int as libc::c_ulong))
            .wrapping_div(fields) as libc::c_int;
        let mut adjusted_width: libc::c_int = pad_remaining - next_pad + width;
        let mut x: libc::c_float = 0.;
        if input_swap as libc::c_int != 0
            && ::core::mem::size_of::<libc::c_float>() as libc::c_ulong
                > 1 as libc::c_int as libc::c_ulong
        {
            let mut j: size_t = 0;
            let mut u: C2RustUnnamed_10 = C2RustUnnamed_10 { x: 0. };
            j = 0 as libc::c_int as size_t;
            while j < ::core::mem::size_of::<libc::c_float>() as libc::c_ulong {
                u
                    .b[j
                    as usize] = *(p as *const libc::c_char)
                    .offset(
                        (::core::mem::size_of::<libc::c_float>() as libc::c_ulong)
                            .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                            .wrapping_sub(j) as isize,
                    );
                j = j.wrapping_add(1);
            }
            x = u.x;
        } else {
            x = *p;
        }
        p = p.offset(1);
        let mut buf: [libc::c_char; 31] = [0; 31];
        ftoastr(
            buf.as_mut_ptr(),
            ::core::mem::size_of::<[libc::c_char; 31]>() as libc::c_ulong,
            0 as libc::c_int,
            0 as libc::c_int,
            x,
        );
        xprintf(
            b"%*s\0" as *const u8 as *const libc::c_char,
            adjusted_width,
            buf.as_mut_ptr(),
        );
        pad_remaining = next_pad;
        i = i.wrapping_sub(1);
    }
}
unsafe extern "C" fn print_double(
    mut fields: size_t,
    mut blank: size_t,
    mut block: *const libc::c_void,
    mut fmt_string: *const libc::c_char,
    mut width: libc::c_int,
    mut pad: libc::c_int,
) {
    let mut p: *const libc::c_double = block as *const libc::c_double;
    let mut i: uintmax_t = 0;
    let mut pad_remaining: libc::c_int = pad;
    i = fields;
    while blank < i {
        let mut next_pad: libc::c_int = (pad as libc::c_ulong)
            .wrapping_mul(i.wrapping_sub(1 as libc::c_int as libc::c_ulong))
            .wrapping_div(fields) as libc::c_int;
        let mut adjusted_width: libc::c_int = pad_remaining - next_pad + width;
        let mut x: libc::c_double = 0.;
        if input_swap as libc::c_int != 0
            && ::core::mem::size_of::<libc::c_double>() as libc::c_ulong
                > 1 as libc::c_int as libc::c_ulong
        {
            let mut j: size_t = 0;
            let mut u: C2RustUnnamed_11 = C2RustUnnamed_11 { x: 0. };
            j = 0 as libc::c_int as size_t;
            while j < ::core::mem::size_of::<libc::c_double>() as libc::c_ulong {
                u
                    .b[j
                    as usize] = *(p as *const libc::c_char)
                    .offset(
                        (::core::mem::size_of::<libc::c_double>() as libc::c_ulong)
                            .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                            .wrapping_sub(j) as isize,
                    );
                j = j.wrapping_add(1);
            }
            x = u.x;
        } else {
            x = *p;
        }
        p = p.offset(1);
        let mut buf: [libc::c_char; 40] = [0; 40];
        dtoastr(
            buf.as_mut_ptr(),
            ::core::mem::size_of::<[libc::c_char; 40]>() as libc::c_ulong,
            0 as libc::c_int,
            0 as libc::c_int,
            x,
        );
        xprintf(
            b"%*s\0" as *const u8 as *const libc::c_char,
            adjusted_width,
            buf.as_mut_ptr(),
        );
        pad_remaining = next_pad;
        i = i.wrapping_sub(1);
    }
}
unsafe extern "C" fn print_long_double(
    mut fields: size_t,
    mut blank: size_t,
    mut block: *const libc::c_void,
    mut fmt_string: *const libc::c_char,
    mut width: libc::c_int,
    mut pad: libc::c_int,
) {
    let mut p: *const f128::f128 = block as *const f128::f128;
    let mut i: uintmax_t = 0;
    let mut pad_remaining: libc::c_int = pad;
    i = fields;
    while blank < i {
        let mut next_pad: libc::c_int = (pad as libc::c_ulong)
            .wrapping_mul(i.wrapping_sub(1 as libc::c_int as libc::c_ulong))
            .wrapping_div(fields) as libc::c_int;
        let mut adjusted_width: libc::c_int = pad_remaining - next_pad + width;
        let mut x: f128::f128 = f128::f128::ZERO;
        if input_swap as libc::c_int != 0
            && ::core::mem::size_of::<f128::f128>() as libc::c_ulong
                > 1 as libc::c_int as libc::c_ulong
        {
            let mut j: size_t = 0;
            let mut u: C2RustUnnamed_12 = C2RustUnnamed_12 {
                x: f128::f128::ZERO,
            };
            j = 0 as libc::c_int as size_t;
            while j < ::core::mem::size_of::<f128::f128>() as libc::c_ulong {
                u
                    .b[j
                    as usize] = *(p as *const libc::c_char)
                    .offset(
                        (::core::mem::size_of::<f128::f128>() as libc::c_ulong)
                            .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                            .wrapping_sub(j) as isize,
                    );
                j = j.wrapping_add(1);
            }
            x = u.x;
        } else {
            x = *p;
        }
        p = p.offset(1);
        let mut buf: [libc::c_char; 60] = [0; 60];
        ldtoastr(
            buf.as_mut_ptr(),
            ::core::mem::size_of::<[libc::c_char; 60]>() as libc::c_ulong,
            0 as libc::c_int,
            0 as libc::c_int,
            x,
        );
        xprintf(
            b"%*s\0" as *const u8 as *const libc::c_char,
            adjusted_width,
            buf.as_mut_ptr(),
        );
        pad_remaining = next_pad;
        i = i.wrapping_sub(1);
    }
}
unsafe extern "C" fn dump_hexl_mode_trailer(
    mut n_bytes: size_t,
    mut block: *const libc::c_char,
) {
    fputs_unlocked(b"  >\0" as *const u8 as *const libc::c_char, stdout);
    let mut i: size_t = n_bytes;
    while i > 0 as libc::c_int as libc::c_ulong {
        let fresh3 = block;
        block = block.offset(1);
        let mut c: libc::c_uchar = *fresh3 as libc::c_uchar;
        let mut c2: libc::c_uchar = (if *(*__ctype_b_loc())
            .offset(c as libc::c_int as isize) as libc::c_int
            & _ISprint as libc::c_int as libc::c_ushort as libc::c_int != 0
        {
            c as libc::c_int
        } else {
            '.' as i32
        }) as libc::c_uchar;
        putchar_unlocked(c2 as libc::c_int);
        i = i.wrapping_sub(1);
    }
    putchar_unlocked('<' as i32);
}
unsafe extern "C" fn print_named_ascii(
    mut fields: size_t,
    mut blank: size_t,
    mut block: *const libc::c_void,
    mut unused_fmt_string: *const libc::c_char,
    mut width: libc::c_int,
    mut pad: libc::c_int,
) {
    let mut p: *const libc::c_uchar = block as *const libc::c_uchar;
    let mut i: uintmax_t = 0;
    let mut pad_remaining: libc::c_int = pad;
    i = fields;
    while blank < i {
        let mut next_pad: libc::c_int = (pad as libc::c_ulong)
            .wrapping_mul(i.wrapping_sub(1 as libc::c_int as libc::c_ulong))
            .wrapping_div(fields) as libc::c_int;
        let fresh4 = p;
        p = p.offset(1);
        let mut masked_c: libc::c_int = *fresh4 as libc::c_int & 0x7f as libc::c_int;
        let mut s: *const libc::c_char = 0 as *const libc::c_char;
        let mut buf: [libc::c_char; 2] = [0; 2];
        if masked_c == 127 as libc::c_int {
            s = b"del\0" as *const u8 as *const libc::c_char;
        } else if masked_c <= 0o40 as libc::c_int {
            s = (charname[masked_c as usize]).as_ptr();
        } else {
            buf[0 as libc::c_int as usize] = masked_c as libc::c_char;
            buf[1 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
            s = buf.as_mut_ptr();
        }
        xprintf(
            b"%*s\0" as *const u8 as *const libc::c_char,
            pad_remaining - next_pad + width,
            s,
        );
        pad_remaining = next_pad;
        i = i.wrapping_sub(1);
    }
}
unsafe extern "C" fn print_ascii(
    mut fields: size_t,
    mut blank: size_t,
    mut block: *const libc::c_void,
    mut unused_fmt_string: *const libc::c_char,
    mut width: libc::c_int,
    mut pad: libc::c_int,
) {
    let mut p: *const libc::c_uchar = block as *const libc::c_uchar;
    let mut i: uintmax_t = 0;
    let mut pad_remaining: libc::c_int = pad;
    i = fields;
    while blank < i {
        let mut next_pad: libc::c_int = (pad as libc::c_ulong)
            .wrapping_mul(i.wrapping_sub(1 as libc::c_int as libc::c_ulong))
            .wrapping_div(fields) as libc::c_int;
        let fresh5 = p;
        p = p.offset(1);
        let mut c: libc::c_uchar = *fresh5;
        let mut s: *const libc::c_char = 0 as *const libc::c_char;
        let mut buf: [libc::c_char; 4] = [0; 4];
        match c as libc::c_int {
            0 => {
                s = b"\\0\0" as *const u8 as *const libc::c_char;
            }
            7 => {
                s = b"\\a\0" as *const u8 as *const libc::c_char;
            }
            8 => {
                s = b"\\b\0" as *const u8 as *const libc::c_char;
            }
            12 => {
                s = b"\\f\0" as *const u8 as *const libc::c_char;
            }
            10 => {
                s = b"\\n\0" as *const u8 as *const libc::c_char;
            }
            13 => {
                s = b"\\r\0" as *const u8 as *const libc::c_char;
            }
            9 => {
                s = b"\\t\0" as *const u8 as *const libc::c_char;
            }
            11 => {
                s = b"\\v\0" as *const u8 as *const libc::c_char;
            }
            _ => {
                sprintf(
                    buf.as_mut_ptr(),
                    if *(*__ctype_b_loc()).offset(c as libc::c_int as isize)
                        as libc::c_int
                        & _ISprint as libc::c_int as libc::c_ushort as libc::c_int != 0
                    {
                        b"%c\0" as *const u8 as *const libc::c_char
                    } else {
                        b"%03o\0" as *const u8 as *const libc::c_char
                    },
                    c as libc::c_int,
                );
                s = buf.as_mut_ptr();
            }
        }
        xprintf(
            b"%*s\0" as *const u8 as *const libc::c_char,
            pad_remaining - next_pad + width,
            s,
        );
        pad_remaining = next_pad;
        i = i.wrapping_sub(1);
    }
}
unsafe extern "C" fn simple_strtoul(
    mut s: *const libc::c_char,
    mut p: *mut *const libc::c_char,
    mut val: *mut libc::c_ulong,
) -> bool {
    let mut sum: libc::c_ulong = 0;
    sum = 0 as libc::c_int as libc::c_ulong;
    while (*s as libc::c_uint).wrapping_sub('0' as i32 as libc::c_uint)
        <= 9 as libc::c_int as libc::c_uint
    {
        let fresh6 = s;
        s = s.offset(1);
        let mut c: libc::c_int = *fresh6 as libc::c_int - '0' as i32;
        if sum
            > (9223372036854775807 as libc::c_long as libc::c_ulong)
                .wrapping_mul(2 as libc::c_ulong)
                .wrapping_add(1 as libc::c_ulong)
                .wrapping_sub(c as libc::c_ulong)
                .wrapping_div(10 as libc::c_int as libc::c_ulong)
        {
            return 0 as libc::c_int != 0;
        }
        sum = sum
            .wrapping_mul(10 as libc::c_int as libc::c_ulong)
            .wrapping_add(c as libc::c_ulong);
    }
    *p = s;
    *val = sum;
    return 1 as libc::c_int != 0;
}
unsafe extern "C" fn decode_one_format(
    mut s_orig: *const libc::c_char,
    mut s: *const libc::c_char,
    mut next: *mut *const libc::c_char,
    mut tspec: *mut tspec,
) -> bool {
    let mut size_spec: size_spec = NO_SIZE;
    let mut size: libc::c_ulong = 0;
    let mut fmt: output_format = SIGNED_DECIMAL;
    let mut print_function: Option::<
        unsafe extern "C" fn(
            size_t,
            size_t,
            *const libc::c_void,
            *const libc::c_char,
            libc::c_int,
            libc::c_int,
        ) -> (),
    > = None;
    let mut p: *const libc::c_char = 0 as *const libc::c_char;
    let mut c: libc::c_char = 0;
    let mut field_width: libc::c_int = 0;
    if !tspec.is_null() {} else {
        __assert_fail(
            b"tspec != NULL\0" as *const u8 as *const libc::c_char,
            b"src/od.c\0" as *const u8 as *const libc::c_char,
            650 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 83],
                &[libc::c_char; 83],
            >(
                b"_Bool decode_one_format(const char *, const char *, const char **, struct tspec *)\0",
            ))
                .as_ptr(),
        );
    }
    match *s as libc::c_int {
        100 | 111 | 117 | 120 => {
            c = *s;
            s = s.offset(1);
            match *s as libc::c_int {
                67 => {
                    s = s.offset(1);
                    size = ::core::mem::size_of::<libc::c_char>() as libc::c_ulong;
                }
                83 => {
                    s = s.offset(1);
                    size = ::core::mem::size_of::<libc::c_short>() as libc::c_ulong;
                }
                73 => {
                    s = s.offset(1);
                    size = ::core::mem::size_of::<libc::c_int>() as libc::c_ulong;
                }
                76 => {
                    s = s.offset(1);
                    size = ::core::mem::size_of::<libc::c_long>() as libc::c_ulong;
                }
                _ => {
                    if !simple_strtoul(s, &mut p, &mut size) {
                        error(
                            0 as libc::c_int,
                            0 as libc::c_int,
                            dcgettext(
                                0 as *const libc::c_char,
                                b"invalid type string %s\0" as *const u8
                                    as *const libc::c_char,
                                5 as libc::c_int,
                            ),
                            quote(s_orig),
                        );
                        return 0 as libc::c_int != 0;
                    }
                    if p == s {
                        size = ::core::mem::size_of::<libc::c_int>() as libc::c_ulong;
                    } else {
                        if (::core::mem::size_of::<unsigned_long_long_int>()
                            as libc::c_ulong) < size
                            || integral_type_size[size as usize] as libc::c_uint
                                == NO_SIZE as libc::c_int as libc::c_uint
                        {
                            error(
                                0 as libc::c_int,
                                0 as libc::c_int,
                                dcgettext(
                                    0 as *const libc::c_char,
                                    b"invalid type string %s;\nthis system doesn't provide a %lu-byte integral type\0"
                                        as *const u8 as *const libc::c_char,
                                    5 as libc::c_int,
                                ),
                                quote(s_orig),
                                size,
                            );
                            return 0 as libc::c_int != 0;
                        }
                        s = p;
                    }
                }
            }
            size_spec = integral_type_size[size as usize];
            match c as libc::c_int {
                100 => {
                    fmt = SIGNED_DECIMAL;
                    field_width = bytes_to_signed_dec_digits[size as usize]
                        as libc::c_int;
                    sprintf(
                        ((*tspec).fmt_string).as_mut_ptr(),
                        b"%%*%s\0" as *const u8 as *const libc::c_char,
                        if size_spec as libc::c_uint
                            == LONG_LONG as libc::c_int as libc::c_uint
                        {
                            b"ld\0" as *const u8 as *const libc::c_char
                        } else if size_spec as libc::c_uint
                            == LONG as libc::c_int as libc::c_uint
                        {
                            b"ld\0" as *const u8 as *const libc::c_char
                        } else {
                            b"d\0" as *const u8 as *const libc::c_char
                        },
                    );
                }
                111 => {
                    fmt = OCTAL;
                    field_width = bytes_to_oct_digits[size as usize] as libc::c_int;
                    sprintf(
                        ((*tspec).fmt_string).as_mut_ptr(),
                        b"%%*.%d%s\0" as *const u8 as *const libc::c_char,
                        field_width,
                        if size_spec as libc::c_uint
                            == LONG_LONG as libc::c_int as libc::c_uint
                        {
                            b"lo\0" as *const u8 as *const libc::c_char
                        } else if size_spec as libc::c_uint
                            == LONG as libc::c_int as libc::c_uint
                        {
                            b"lo\0" as *const u8 as *const libc::c_char
                        } else {
                            b"o\0" as *const u8 as *const libc::c_char
                        },
                    );
                }
                117 => {
                    fmt = UNSIGNED_DECIMAL;
                    field_width = bytes_to_unsigned_dec_digits[size as usize]
                        as libc::c_int;
                    sprintf(
                        ((*tspec).fmt_string).as_mut_ptr(),
                        b"%%*%s\0" as *const u8 as *const libc::c_char,
                        if size_spec as libc::c_uint
                            == LONG_LONG as libc::c_int as libc::c_uint
                        {
                            b"lu\0" as *const u8 as *const libc::c_char
                        } else if size_spec as libc::c_uint
                            == LONG as libc::c_int as libc::c_uint
                        {
                            b"lu\0" as *const u8 as *const libc::c_char
                        } else {
                            b"u\0" as *const u8 as *const libc::c_char
                        },
                    );
                }
                120 => {
                    fmt = HEXADECIMAL;
                    field_width = bytes_to_hex_digits[size as usize] as libc::c_int;
                    sprintf(
                        ((*tspec).fmt_string).as_mut_ptr(),
                        b"%%*.%d%s\0" as *const u8 as *const libc::c_char,
                        field_width,
                        if size_spec as libc::c_uint
                            == LONG_LONG as libc::c_int as libc::c_uint
                        {
                            b"lx\0" as *const u8 as *const libc::c_char
                        } else if size_spec as libc::c_uint
                            == LONG as libc::c_int as libc::c_uint
                        {
                            b"lx\0" as *const u8 as *const libc::c_char
                        } else {
                            b"x\0" as *const u8 as *const libc::c_char
                        },
                    );
                }
                _ => {
                    abort();
                }
            }
            if strlen(((*tspec).fmt_string).as_mut_ptr())
                < FMT_BYTES_ALLOCATED as libc::c_int as libc::c_ulong
            {} else {
                __assert_fail(
                    b"strlen (tspec->fmt_string) < FMT_BYTES_ALLOCATED\0" as *const u8
                        as *const libc::c_char,
                    b"src/od.c\0" as *const u8 as *const libc::c_char,
                    749 as libc::c_int as libc::c_uint,
                    (*::core::mem::transmute::<
                        &[u8; 83],
                        &[libc::c_char; 83],
                    >(
                        b"_Bool decode_one_format(const char *, const char *, const char **, struct tspec *)\0",
                    ))
                        .as_ptr(),
                );
            }
            match size_spec as libc::c_uint {
                1 => {
                    print_function = if fmt as libc::c_uint
                        == SIGNED_DECIMAL as libc::c_int as libc::c_uint
                    {
                        Some(
                            print_s_char
                                as unsafe extern "C" fn(
                                    size_t,
                                    size_t,
                                    *const libc::c_void,
                                    *const libc::c_char,
                                    libc::c_int,
                                    libc::c_int,
                                ) -> (),
                        )
                    } else {
                        Some(
                            print_char
                                as unsafe extern "C" fn(
                                    size_t,
                                    size_t,
                                    *const libc::c_void,
                                    *const libc::c_char,
                                    libc::c_int,
                                    libc::c_int,
                                ) -> (),
                        )
                    };
                }
                2 => {
                    print_function = if fmt as libc::c_uint
                        == SIGNED_DECIMAL as libc::c_int as libc::c_uint
                    {
                        Some(
                            print_s_short
                                as unsafe extern "C" fn(
                                    size_t,
                                    size_t,
                                    *const libc::c_void,
                                    *const libc::c_char,
                                    libc::c_int,
                                    libc::c_int,
                                ) -> (),
                        )
                    } else {
                        Some(
                            print_short
                                as unsafe extern "C" fn(
                                    size_t,
                                    size_t,
                                    *const libc::c_void,
                                    *const libc::c_char,
                                    libc::c_int,
                                    libc::c_int,
                                ) -> (),
                        )
                    };
                }
                3 => {
                    print_function = Some(
                        print_int
                            as unsafe extern "C" fn(
                                size_t,
                                size_t,
                                *const libc::c_void,
                                *const libc::c_char,
                                libc::c_int,
                                libc::c_int,
                            ) -> (),
                    );
                }
                4 => {
                    print_function = Some(
                        print_long
                            as unsafe extern "C" fn(
                                size_t,
                                size_t,
                                *const libc::c_void,
                                *const libc::c_char,
                                libc::c_int,
                                libc::c_int,
                            ) -> (),
                    );
                }
                5 => {
                    print_function = Some(
                        print_long_long
                            as unsafe extern "C" fn(
                                size_t,
                                size_t,
                                *const libc::c_void,
                                *const libc::c_char,
                                libc::c_int,
                                libc::c_int,
                            ) -> (),
                    );
                }
                _ => {
                    abort();
                }
            }
        }
        102 => {
            fmt = FLOATING_POINT;
            s = s.offset(1);
            match *s as libc::c_int {
                70 => {
                    s = s.offset(1);
                    size = ::core::mem::size_of::<libc::c_float>() as libc::c_ulong;
                }
                68 => {
                    s = s.offset(1);
                    size = ::core::mem::size_of::<libc::c_double>() as libc::c_ulong;
                }
                76 => {
                    s = s.offset(1);
                    size = ::core::mem::size_of::<f128::f128>() as libc::c_ulong;
                }
                _ => {
                    if !simple_strtoul(s, &mut p, &mut size) {
                        error(
                            0 as libc::c_int,
                            0 as libc::c_int,
                            dcgettext(
                                0 as *const libc::c_char,
                                b"invalid type string %s\0" as *const u8
                                    as *const libc::c_char,
                                5 as libc::c_int,
                            ),
                            quote(s_orig),
                        );
                        return 0 as libc::c_int != 0;
                    }
                    if p == s {
                        size = ::core::mem::size_of::<libc::c_double>() as libc::c_ulong;
                    } else {
                        if size > ::core::mem::size_of::<f128::f128>() as libc::c_ulong
                            || fp_type_size[size as usize] as libc::c_uint
                                == NO_SIZE as libc::c_int as libc::c_uint
                        {
                            error(
                                0 as libc::c_int,
                                0 as libc::c_int,
                                dcgettext(
                                    0 as *const libc::c_char,
                                    b"invalid type string %s;\nthis system doesn't provide a %lu-byte floating point type\0"
                                        as *const u8 as *const libc::c_char,
                                    5 as libc::c_int,
                                ),
                                quote(s_orig),
                                size,
                            );
                            return 0 as libc::c_int != 0;
                        }
                        s = p;
                    }
                }
            }
            size_spec = fp_type_size[size as usize];
            let mut locale: *const lconv = localeconv();
            let mut decimal_point_len: size_t = if *((*locale).decimal_point)
                .offset(0 as libc::c_int as isize) as libc::c_int != 0
            {
                strlen((*locale).decimal_point)
            } else {
                1 as libc::c_int as libc::c_ulong
            };
            match size_spec as libc::c_uint {
                6 => {
                    print_function = Some(
                        print_float
                            as unsafe extern "C" fn(
                                size_t,
                                size_t,
                                *const libc::c_void,
                                *const libc::c_char,
                                libc::c_int,
                                libc::c_int,
                            ) -> (),
                    );
                    field_width = ((1 as libc::c_int
                        + ((24 as libc::c_int * 1 as libc::c_int * 146 as libc::c_int
                            + 484 as libc::c_int) / 485 as libc::c_int
                            + 1 as libc::c_int)) as libc::c_ulong)
                        .wrapping_add(decimal_point_len)
                        .wrapping_add(1 as libc::c_int as libc::c_ulong)
                        .wrapping_add(
                            (if -(100 as libc::c_int) < -(37 as libc::c_int)
                                && (38 as libc::c_int) < 100 as libc::c_int
                            {
                                3 as libc::c_int as libc::c_ulong
                            } else {
                                (if -(1000 as libc::c_int) < -(37 as libc::c_int)
                                    && (38 as libc::c_int) < 1000 as libc::c_int
                                {
                                    4 as libc::c_int as libc::c_ulong
                                } else {
                                    (if -(10000 as libc::c_int) < -(37 as libc::c_int)
                                        && (38 as libc::c_int) < 10000 as libc::c_int
                                    {
                                        5 as libc::c_int as libc::c_ulong
                                    } else {
                                        (if -(100000 as libc::c_int) < -(37 as libc::c_int)
                                            && (38 as libc::c_int) < 100000 as libc::c_int
                                        {
                                            6 as libc::c_int as libc::c_ulong
                                        } else {
                                            (if -(1000000 as libc::c_int) < -(37 as libc::c_int)
                                                && (38 as libc::c_int) < 1000000 as libc::c_int
                                            {
                                                7 as libc::c_int as libc::c_ulong
                                            } else {
                                                (::core::mem::size_of::<libc::c_int>() as libc::c_ulong)
                                                    .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                                    .wrapping_sub(
                                                        !((0 as libc::c_int) < -(1 as libc::c_int)) as libc::c_int
                                                            as libc::c_ulong,
                                                    )
                                                    .wrapping_mul(146 as libc::c_int as libc::c_ulong)
                                                    .wrapping_add(484 as libc::c_int as libc::c_ulong)
                                                    .wrapping_div(485 as libc::c_int as libc::c_ulong)
                                                    .wrapping_add(
                                                        !((0 as libc::c_int) < -(1 as libc::c_int)) as libc::c_int
                                                            as libc::c_ulong,
                                                    )
                                            })
                                        })
                                    })
                                })
                            }),
                        ) as libc::c_int;
                }
                7 => {
                    print_function = Some(
                        print_double
                            as unsafe extern "C" fn(
                                size_t,
                                size_t,
                                *const libc::c_void,
                                *const libc::c_char,
                                libc::c_int,
                                libc::c_int,
                            ) -> (),
                    );
                    field_width = ((1 as libc::c_int
                        + ((53 as libc::c_int * 1 as libc::c_int * 146 as libc::c_int
                            + 484 as libc::c_int) / 485 as libc::c_int
                            + 1 as libc::c_int)) as libc::c_ulong)
                        .wrapping_add(decimal_point_len)
                        .wrapping_add(1 as libc::c_int as libc::c_ulong)
                        .wrapping_add(
                            (if -(100 as libc::c_int) < -(307 as libc::c_int)
                                && (308 as libc::c_int) < 100 as libc::c_int
                            {
                                3 as libc::c_int as libc::c_ulong
                            } else {
                                (if -(1000 as libc::c_int) < -(307 as libc::c_int)
                                    && (308 as libc::c_int) < 1000 as libc::c_int
                                {
                                    4 as libc::c_int as libc::c_ulong
                                } else {
                                    (if -(10000 as libc::c_int) < -(307 as libc::c_int)
                                        && (308 as libc::c_int) < 10000 as libc::c_int
                                    {
                                        5 as libc::c_int as libc::c_ulong
                                    } else {
                                        (if -(100000 as libc::c_int) < -(307 as libc::c_int)
                                            && (308 as libc::c_int) < 100000 as libc::c_int
                                        {
                                            6 as libc::c_int as libc::c_ulong
                                        } else {
                                            (if -(1000000 as libc::c_int) < -(307 as libc::c_int)
                                                && (308 as libc::c_int) < 1000000 as libc::c_int
                                            {
                                                7 as libc::c_int as libc::c_ulong
                                            } else {
                                                (::core::mem::size_of::<libc::c_int>() as libc::c_ulong)
                                                    .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                                    .wrapping_sub(
                                                        !((0 as libc::c_int) < -(1 as libc::c_int)) as libc::c_int
                                                            as libc::c_ulong,
                                                    )
                                                    .wrapping_mul(146 as libc::c_int as libc::c_ulong)
                                                    .wrapping_add(484 as libc::c_int as libc::c_ulong)
                                                    .wrapping_div(485 as libc::c_int as libc::c_ulong)
                                                    .wrapping_add(
                                                        !((0 as libc::c_int) < -(1 as libc::c_int)) as libc::c_int
                                                            as libc::c_ulong,
                                                    )
                                            })
                                        })
                                    })
                                })
                            }),
                        ) as libc::c_int;
                }
                8 => {
                    print_function = Some(
                        print_long_double
                            as unsafe extern "C" fn(
                                size_t,
                                size_t,
                                *const libc::c_void,
                                *const libc::c_char,
                                libc::c_int,
                                libc::c_int,
                            ) -> (),
                    );
                    field_width = ((1 as libc::c_int
                        + ((113 as libc::c_int * 1 as libc::c_int * 146 as libc::c_int
                            + 484 as libc::c_int) / 485 as libc::c_int
                            + 1 as libc::c_int)) as libc::c_ulong)
                        .wrapping_add(decimal_point_len)
                        .wrapping_add(1 as libc::c_int as libc::c_ulong)
                        .wrapping_add(
                            (if -(100 as libc::c_int) < -(4931 as libc::c_int)
                                && (4932 as libc::c_int) < 100 as libc::c_int
                            {
                                3 as libc::c_int as libc::c_ulong
                            } else {
                                (if -(1000 as libc::c_int) < -(4931 as libc::c_int)
                                    && (4932 as libc::c_int) < 1000 as libc::c_int
                                {
                                    4 as libc::c_int as libc::c_ulong
                                } else {
                                    (if -(10000 as libc::c_int) < -(4931 as libc::c_int)
                                        && (4932 as libc::c_int) < 10000 as libc::c_int
                                    {
                                        5 as libc::c_int as libc::c_ulong
                                    } else {
                                        (if -(100000 as libc::c_int) < -(4931 as libc::c_int)
                                            && (4932 as libc::c_int) < 100000 as libc::c_int
                                        {
                                            6 as libc::c_int as libc::c_ulong
                                        } else {
                                            (if -(1000000 as libc::c_int) < -(4931 as libc::c_int)
                                                && (4932 as libc::c_int) < 1000000 as libc::c_int
                                            {
                                                7 as libc::c_int as libc::c_ulong
                                            } else {
                                                (::core::mem::size_of::<libc::c_int>() as libc::c_ulong)
                                                    .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                                    .wrapping_sub(
                                                        !((0 as libc::c_int) < -(1 as libc::c_int)) as libc::c_int
                                                            as libc::c_ulong,
                                                    )
                                                    .wrapping_mul(146 as libc::c_int as libc::c_ulong)
                                                    .wrapping_add(484 as libc::c_int as libc::c_ulong)
                                                    .wrapping_div(485 as libc::c_int as libc::c_ulong)
                                                    .wrapping_add(
                                                        !((0 as libc::c_int) < -(1 as libc::c_int)) as libc::c_int
                                                            as libc::c_ulong,
                                                    )
                                            })
                                        })
                                    })
                                })
                            }),
                        ) as libc::c_int;
                }
                _ => {
                    abort();
                }
            }
        }
        97 => {
            s = s.offset(1);
            fmt = NAMED_CHARACTER;
            size_spec = CHAR;
            print_function = Some(
                print_named_ascii
                    as unsafe extern "C" fn(
                        size_t,
                        size_t,
                        *const libc::c_void,
                        *const libc::c_char,
                        libc::c_int,
                        libc::c_int,
                    ) -> (),
            );
            field_width = 3 as libc::c_int;
        }
        99 => {
            s = s.offset(1);
            fmt = CHARACTER;
            size_spec = CHAR;
            print_function = Some(
                print_ascii
                    as unsafe extern "C" fn(
                        size_t,
                        size_t,
                        *const libc::c_void,
                        *const libc::c_char,
                        libc::c_int,
                        libc::c_int,
                    ) -> (),
            );
            field_width = 3 as libc::c_int;
        }
        _ => {
            error(
                0 as libc::c_int,
                0 as libc::c_int,
                dcgettext(
                    0 as *const libc::c_char,
                    b"invalid character '%c' in type string %s\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
                *s as libc::c_int,
                quote(s_orig),
            );
            return 0 as libc::c_int != 0;
        }
    }
    (*tspec).size = size_spec;
    (*tspec).fmt = fmt;
    (*tspec).print_function = print_function;
    (*tspec).field_width = field_width;
    (*tspec).hexl_mode_trailer = *s as libc::c_int == 'z' as i32;
    if (*tspec).hexl_mode_trailer {
        s = s.offset(1);
    }
    if !next.is_null() {
        *next = s;
    }
    return 1 as libc::c_int != 0;
}
unsafe extern "C" fn open_next_file() -> bool {
    let mut ok: bool = 1 as libc::c_int != 0;
    loop {
        input_filename = *file_list;
        if input_filename.is_null() {
            return ok;
        }
        file_list = file_list.offset(1);
        if strcmp(input_filename, b"-\0" as *const u8 as *const libc::c_char)
            == 0 as libc::c_int
        {
            input_filename = dcgettext(
                0 as *const libc::c_char,
                b"standard input\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            );
            in_stream = stdin;
            have_read_stdin = 1 as libc::c_int != 0;
            xset_binary_mode(0 as libc::c_int, 0 as libc::c_int);
        } else {
            in_stream = fopen(
                input_filename,
                if 0 as libc::c_int != 0 {
                    b"rb\0" as *const u8 as *const libc::c_char
                } else {
                    b"r\0" as *const u8 as *const libc::c_char
                },
            );
            if in_stream.is_null() {
                error(
                    0 as libc::c_int,
                    *__errno_location(),
                    b"%s\0" as *const u8 as *const libc::c_char,
                    quotearg_n_style_colon(
                        0 as libc::c_int,
                        shell_escape_quoting_style,
                        input_filename,
                    ),
                );
                ok = 0 as libc::c_int != 0;
            }
        }
        if !in_stream.is_null() {
            break;
        }
    }
    if limit_bytes_to_format as libc::c_int != 0 && !flag_dump_strings {
        setvbuf(
            in_stream,
            0 as *mut libc::c_char,
            2 as libc::c_int,
            0 as libc::c_int as size_t,
        );
    }
    return ok;
}
unsafe extern "C" fn check_and_close(mut in_errno: libc::c_int) -> bool {
    let mut ok: bool = 1 as libc::c_int != 0;
    if !in_stream.is_null() {
        if ferror_unlocked(in_stream) == 0 {
            in_errno = 0 as libc::c_int;
        }
        if strcmp(
            *file_list.offset(-(1 as libc::c_int) as isize),
            b"-\0" as *const u8 as *const libc::c_char,
        ) == 0 as libc::c_int
        {
            clearerr_unlocked(in_stream);
        } else if rpl_fclose(in_stream) != 0 as libc::c_int && in_errno == 0 {
            in_errno = *__errno_location();
        }
        if in_errno != 0 {
            error(
                0 as libc::c_int,
                in_errno,
                b"%s\0" as *const u8 as *const libc::c_char,
                quotearg_n_style_colon(
                    0 as libc::c_int,
                    shell_escape_quoting_style,
                    input_filename,
                ),
            );
            ok = 0 as libc::c_int != 0;
        }
        in_stream = 0 as *mut FILE;
    }
    if ferror_unlocked(stdout) != 0 {
        error(
            0 as libc::c_int,
            0 as libc::c_int,
            dcgettext(
                0 as *const libc::c_char,
                b"write error\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
        ok = 0 as libc::c_int != 0;
    }
    return ok;
}
unsafe extern "C" fn decode_format_string(mut s: *const libc::c_char) -> bool {
    let mut s_orig: *const libc::c_char = s;
    if !s.is_null() {} else {
        __assert_fail(
            b"s != NULL\0" as *const u8 as *const libc::c_char,
            b"src/od.c\0" as *const u8 as *const libc::c_char,
            987 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 41],
                &[libc::c_char; 41],
            >(b"_Bool decode_format_string(const char *)\0"))
                .as_ptr(),
        );
    }
    while *s as libc::c_int != '\0' as i32 {
        let mut next: *const libc::c_char = 0 as *const libc::c_char;
        if n_specs_allocated <= n_specs {
            spec = (if ::core::mem::size_of::<C2RustUnnamed_13>() as libc::c_ulong != 0 {
                x2nrealloc(
                    spec as *mut libc::c_void,
                    &mut n_specs_allocated,
                    ::core::mem::size_of::<tspec>() as libc::c_ulong,
                )
            } else {
                x2nrealloc(
                    spec as *mut libc::c_void,
                    &mut n_specs_allocated,
                    ::core::mem::size_of::<tspec>() as libc::c_ulong,
                )
            }) as *mut tspec;
        }
        if !decode_one_format(
            s_orig,
            s,
            &mut next,
            &mut *spec.offset(n_specs as isize),
        ) {
            return 0 as libc::c_int != 0;
        }
        if s != next {} else {
            __assert_fail(
                b"s != next\0" as *const u8 as *const libc::c_char,
                b"src/od.c\0" as *const u8 as *const libc::c_char,
                999 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 41],
                    &[libc::c_char; 41],
                >(b"_Bool decode_format_string(const char *)\0"))
                    .as_ptr(),
            );
        }
        s = next;
        n_specs = n_specs.wrapping_add(1);
    }
    return 1 as libc::c_int != 0;
}
unsafe extern "C" fn skip(mut n_skip: uintmax_t) -> bool {
    let mut ok: bool = 1 as libc::c_int != 0;
    let mut in_errno: libc::c_int = 0 as libc::c_int;
    if n_skip == 0 as libc::c_int as libc::c_ulong {
        return 1 as libc::c_int != 0;
    }
    while !in_stream.is_null() {
        let mut file_stats: stat = stat {
            st_dev: 0,
            st_ino: 0,
            st_mode: 0,
            st_nlink: 0,
            st_uid: 0,
            st_gid: 0,
            st_rdev: 0,
            __pad1: 0,
            st_size: 0,
            st_blksize: 0,
            __pad2: 0,
            st_blocks: 0,
            st_atim: timespec { tv_sec: 0, tv_nsec: 0 },
            st_mtim: timespec { tv_sec: 0, tv_nsec: 0 },
            st_ctim: timespec { tv_sec: 0, tv_nsec: 0 },
            __glibc_reserved: [0; 2],
        };
        if fstat(fileno(in_stream), &mut file_stats) == 0 as libc::c_int {
            let mut usable_size: bool = usable_st_size(&mut file_stats);
            if usable_size as libc::c_int != 0
                && ((if (0 as libc::c_int) < file_stats.st_blksize
                    && file_stats.st_blksize as libc::c_ulong
                        <= (-(1 as libc::c_int) as size_t)
                            .wrapping_div(8 as libc::c_int as libc::c_ulong)
                            .wrapping_add(1 as libc::c_int as libc::c_ulong)
                {
                    file_stats.st_blksize
                } else {
                    512 as libc::c_int
                }) as libc::c_long) < file_stats.st_size
            {
                if (file_stats.st_size as uintmax_t) < n_skip {
                    n_skip = (n_skip as libc::c_ulong)
                        .wrapping_sub(file_stats.st_size as libc::c_ulong) as uintmax_t
                        as uintmax_t;
                } else {
                    if rpl_fseeko(in_stream, n_skip as off_t, 1 as libc::c_int)
                        != 0 as libc::c_int
                    {
                        in_errno = *__errno_location();
                        ok = 0 as libc::c_int != 0;
                    }
                    n_skip = 0 as libc::c_int as uintmax_t;
                }
            } else if !usable_size
                && rpl_fseeko(in_stream, n_skip as off_t, 1 as libc::c_int)
                    == 0 as libc::c_int
            {
                n_skip = 0 as libc::c_int as uintmax_t;
            } else {
                let mut buf: [libc::c_char; 8192] = [0; 8192];
                let mut n_bytes_read: size_t = 0;
                let mut n_bytes_to_read: size_t = 8192 as libc::c_int as size_t;
                while (0 as libc::c_int as libc::c_ulong) < n_skip {
                    if n_skip < n_bytes_to_read {
                        n_bytes_to_read = n_skip;
                    }
                    n_bytes_read = if 0 != 0 && 0 != 0
                        && (1 as libc::c_int as size_t).wrapping_mul(n_bytes_to_read)
                            <= 8 as libc::c_int as libc::c_ulong
                        && 1 as libc::c_int as size_t
                            != 0 as libc::c_int as libc::c_ulong
                    {
                        ({
                            let mut __ptr: *mut libc::c_char = buf.as_mut_ptr();
                            let mut __stream: *mut FILE = in_stream;
                            let mut __cnt: size_t = 0;
                            __cnt = (1 as libc::c_int as size_t)
                                .wrapping_mul(n_bytes_to_read);
                            while __cnt > 0 as libc::c_int as libc::c_ulong {
                                let mut __c: libc::c_int = getc_unlocked(__stream);
                                if __c == -(1 as libc::c_int) {
                                    break;
                                }
                                let fresh7 = __ptr;
                                __ptr = __ptr.offset(1);
                                *fresh7 = __c as libc::c_char;
                                __cnt = __cnt.wrapping_sub(1);
                            }
                            (1 as libc::c_int as size_t)
                                .wrapping_mul(n_bytes_to_read)
                                .wrapping_sub(__cnt)
                                .wrapping_div(1 as libc::c_int as size_t)
                        })
                    } else if 0 != 0
                        && 1 as libc::c_int as size_t
                            == 0 as libc::c_int as libc::c_ulong
                        || 0 != 0 && n_bytes_to_read == 0 as libc::c_int as libc::c_ulong
                    {
                        0 as libc::c_int as size_t
                    } else {
                        fread_unlocked(
                            buf.as_mut_ptr() as *mut libc::c_void,
                            1 as libc::c_int as size_t,
                            n_bytes_to_read,
                            in_stream,
                        )
                    };
                    n_skip = (n_skip as libc::c_ulong).wrapping_sub(n_bytes_read)
                        as uintmax_t as uintmax_t;
                    if !(n_bytes_read != n_bytes_to_read) {
                        continue;
                    }
                    if ferror_unlocked(in_stream) != 0 {
                        in_errno = *__errno_location();
                        ok = 0 as libc::c_int != 0;
                        n_skip = 0 as libc::c_int as uintmax_t;
                        break;
                    } else if feof_unlocked(in_stream) != 0 {
                        break;
                    }
                }
            }
            if n_skip == 0 as libc::c_int as libc::c_ulong {
                break;
            }
        } else {
            error(
                0 as libc::c_int,
                *__errno_location(),
                b"%s\0" as *const u8 as *const libc::c_char,
                quotearg_n_style_colon(
                    0 as libc::c_int,
                    shell_escape_quoting_style,
                    input_filename,
                ),
            );
            ok = 0 as libc::c_int != 0;
        }
        ok = (ok as libc::c_int & check_and_close(in_errno) as libc::c_int) as bool;
        ok = (ok as libc::c_int & open_next_file() as libc::c_int) as bool;
    }
    if n_skip != 0 as libc::c_int as libc::c_ulong {
        if ::core::mem::size_of::<C2RustUnnamed_14>() as libc::c_ulong != 0 {
            error(
                1 as libc::c_int,
                0 as libc::c_int,
                dcgettext(
                    0 as *const libc::c_char,
                    b"cannot skip past end of combined input\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
            );
            if 0 as libc::c_int != 0 {} else {
                unreachable!();
            };
        } else {
            error(
                1 as libc::c_int,
                0 as libc::c_int,
                dcgettext(
                    0 as *const libc::c_char,
                    b"cannot skip past end of combined input\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
            );
            if 0 as libc::c_int != 0 {} else {
                unreachable!();
            };
        };
    }
    return ok;
}
unsafe extern "C" fn format_address_none(mut address: uintmax_t, mut c: libc::c_char) {}
unsafe extern "C" fn format_address_std(mut address: uintmax_t, mut c: libc::c_char) {
    let mut buf: [libc::c_char; 25] = [0; 25];
    let mut p: *mut libc::c_char = buf
        .as_mut_ptr()
        .offset(::core::mem::size_of::<[libc::c_char; 25]>() as libc::c_ulong as isize);
    let mut pbound: *const libc::c_char = 0 as *const libc::c_char;
    p = p.offset(-1);
    *p = '\0' as i32 as libc::c_char;
    p = p.offset(-1);
    *p = c;
    pbound = p.offset(-(address_pad_len as isize));
    match address_base {
        8 => {
            loop {
                p = p.offset(-1);
                *p = ('0' as i32 as libc::c_ulong)
                    .wrapping_add(address & 7 as libc::c_int as libc::c_ulong)
                    as libc::c_char;
                address >>= 3 as libc::c_int;
                if !(address != 0 as libc::c_int as libc::c_ulong) {
                    break;
                }
            }
        }
        10 => {
            loop {
                p = p.offset(-1);
                *p = ('0' as i32 as libc::c_ulong)
                    .wrapping_add(
                        address.wrapping_rem(10 as libc::c_int as libc::c_ulong),
                    ) as libc::c_char;
                address = (address as libc::c_ulong)
                    .wrapping_div(10 as libc::c_int as libc::c_ulong) as uintmax_t
                    as uintmax_t;
                if !(address != 0 as libc::c_int as libc::c_ulong) {
                    break;
                }
            }
        }
        16 => {
            loop {
                p = p.offset(-1);
                *p = (*::core::mem::transmute::<
                    &[u8; 17],
                    &[libc::c_char; 17],
                >(
                    b"0123456789abcdef\0",
                ))[(address & 15 as libc::c_int as libc::c_ulong) as usize];
                address >>= 4 as libc::c_int;
                if !(address != 0 as libc::c_int as libc::c_ulong) {
                    break;
                }
            }
        }
        _ => {}
    }
    while pbound < p as *const libc::c_char {
        p = p.offset(-1);
        *p = '0' as i32 as libc::c_char;
    }
    fputs_unlocked(p, stdout);
}
unsafe extern "C" fn format_address_paren(mut address: uintmax_t, mut c: libc::c_char) {
    putchar_unlocked('(' as i32);
    format_address_std(address, ')' as i32 as libc::c_char);
    if c != 0 {
        putchar_unlocked(c as libc::c_int);
    }
}
unsafe extern "C" fn format_address_label(mut address: uintmax_t, mut c: libc::c_char) {
    format_address_std(address, ' ' as i32 as libc::c_char);
    format_address_paren(address.wrapping_add(pseudo_offset), c);
}
unsafe extern "C" fn write_block(
    mut current_offset: uintmax_t,
    mut n_bytes: size_t,
    mut prev_block: *const libc::c_char,
    mut curr_block: *const libc::c_char,
) {
    static mut first: bool = 1 as libc::c_int != 0;
    static mut prev_pair_equal: bool = 0 as libc::c_int != 0;
    if abbreviate_duplicate_blocks as libc::c_int != 0 && !first
        && n_bytes == bytes_per_block
        && memcmp(
            prev_block as *const libc::c_void,
            curr_block as *const libc::c_void,
            bytes_per_block,
        ) == 0 as libc::c_int
    {
        if !prev_pair_equal {
            printf(b"*\n\0" as *const u8 as *const libc::c_char);
            prev_pair_equal = 1 as libc::c_int != 0;
        }
    } else {
        prev_pair_equal = 0 as libc::c_int != 0;
        let mut i: size_t = 0 as libc::c_int as size_t;
        while i < n_specs {
            let mut datum_width: libc::c_int = width_bytes[(*spec.offset(i as isize))
                .size as usize];
            let mut fields_per_block: libc::c_int = bytes_per_block
                .wrapping_div(datum_width as libc::c_ulong) as libc::c_int;
            let mut blank_fields: libc::c_int = bytes_per_block
                .wrapping_sub(n_bytes)
                .wrapping_div(datum_width as libc::c_ulong) as libc::c_int;
            if i == 0 as libc::c_int as libc::c_ulong {
                format_address
                    .expect(
                        "non-null function pointer",
                    )(current_offset, '\0' as i32 as libc::c_char);
            } else {
                printf(
                    b"%*s\0" as *const u8 as *const libc::c_char,
                    address_pad_len,
                    b"\0" as *const u8 as *const libc::c_char,
                );
            }
            (Some(
                ((*spec.offset(i as isize)).print_function)
                    .expect("non-null function pointer"),
            ))
                .expect(
                    "non-null function pointer",
                )(
                fields_per_block as size_t,
                blank_fields as size_t,
                curr_block as *const libc::c_void,
                ((*spec.offset(i as isize)).fmt_string).as_mut_ptr(),
                (*spec.offset(i as isize)).field_width,
                (*spec.offset(i as isize)).pad_width,
            );
            if (*spec.offset(i as isize)).hexl_mode_trailer {
                let mut field_width: libc::c_int = (*spec.offset(i as isize))
                    .field_width;
                let mut pad_width: libc::c_int = (*spec.offset(i as isize)).pad_width
                    * blank_fields / fields_per_block;
                printf(
                    b"%*s\0" as *const u8 as *const libc::c_char,
                    blank_fields * field_width + pad_width,
                    b"\0" as *const u8 as *const libc::c_char,
                );
                dump_hexl_mode_trailer(n_bytes, curr_block);
            }
            putchar_unlocked('\n' as i32);
            i = i.wrapping_add(1);
        }
    }
    first = 0 as libc::c_int != 0;
}
unsafe extern "C" fn read_char(mut c: *mut libc::c_int) -> bool {
    let mut ok: bool = 1 as libc::c_int != 0;
    *c = -(1 as libc::c_int);
    while !in_stream.is_null() {
        *c = fgetc(in_stream);
        if *c != -(1 as libc::c_int) {
            break;
        }
        ok = (ok as libc::c_int & check_and_close(*__errno_location()) as libc::c_int)
            as bool;
        ok = (ok as libc::c_int & open_next_file() as libc::c_int) as bool;
    }
    return ok;
}
unsafe extern "C" fn read_block(
    mut n: size_t,
    mut block: *mut libc::c_char,
    mut n_bytes_in_buffer: *mut size_t,
) -> bool {
    let mut ok: bool = 1 as libc::c_int != 0;
    if (0 as libc::c_int as libc::c_ulong) < n && n <= bytes_per_block {} else {
        __assert_fail(
            b"0 < n && n <= bytes_per_block\0" as *const u8 as *const libc::c_char,
            b"src/od.c\0" as *const u8 as *const libc::c_char,
            1295 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 43],
                &[libc::c_char; 43],
            >(b"_Bool read_block(size_t, char *, size_t *)\0"))
                .as_ptr(),
        );
    }
    *n_bytes_in_buffer = 0 as libc::c_int as size_t;
    while !in_stream.is_null() {
        let mut n_needed: size_t = 0;
        let mut n_read: size_t = 0;
        n_needed = n.wrapping_sub(*n_bytes_in_buffer);
        n_read = if 0 != 0 && 0 != 0
            && (1 as libc::c_int as size_t).wrapping_mul(n_needed)
                <= 8 as libc::c_int as libc::c_ulong
            && 1 as libc::c_int as size_t != 0 as libc::c_int as libc::c_ulong
        {
            ({
                let mut __ptr: *mut libc::c_char = block
                    .offset(*n_bytes_in_buffer as isize);
                let mut __stream: *mut FILE = in_stream;
                let mut __cnt: size_t = 0;
                __cnt = (1 as libc::c_int as size_t).wrapping_mul(n_needed);
                while __cnt > 0 as libc::c_int as libc::c_ulong {
                    let mut __c: libc::c_int = getc_unlocked(__stream);
                    if __c == -(1 as libc::c_int) {
                        break;
                    }
                    let fresh8 = __ptr;
                    __ptr = __ptr.offset(1);
                    *fresh8 = __c as libc::c_char;
                    __cnt = __cnt.wrapping_sub(1);
                }
                (1 as libc::c_int as size_t)
                    .wrapping_mul(n_needed)
                    .wrapping_sub(__cnt)
                    .wrapping_div(1 as libc::c_int as size_t)
            })
        } else if 0 != 0
            && 1 as libc::c_int as size_t == 0 as libc::c_int as libc::c_ulong
            || 0 != 0 && n_needed == 0 as libc::c_int as libc::c_ulong
        {
            0 as libc::c_int as size_t
        } else {
            fread_unlocked(
                block.offset(*n_bytes_in_buffer as isize) as *mut libc::c_void,
                1 as libc::c_int as size_t,
                n_needed,
                in_stream,
            )
        };
        *n_bytes_in_buffer = (*n_bytes_in_buffer as libc::c_ulong).wrapping_add(n_read)
            as size_t as size_t;
        if n_read == n_needed {
            break;
        }
        ok = (ok as libc::c_int & check_and_close(*__errno_location()) as libc::c_int)
            as bool;
        ok = (ok as libc::c_int & open_next_file() as libc::c_int) as bool;
    }
    return ok;
}
unsafe extern "C" fn get_lcm() -> libc::c_int {
    let mut l_c_m: libc::c_int = 1 as libc::c_int;
    let mut i: size_t = 0 as libc::c_int as size_t;
    while i < n_specs {
        l_c_m = lcm(
            l_c_m as size_t,
            width_bytes[(*spec.offset(i as isize)).size as usize] as size_t,
        ) as libc::c_int;
        i = i.wrapping_add(1);
    }
    return l_c_m;
}
unsafe extern "C" fn parse_old_offset(
    mut s: *const libc::c_char,
    mut offset: *mut uintmax_t,
) -> bool {
    let mut radix: libc::c_int = 0;
    if *s as libc::c_int == '\0' as i32 {
        return 0 as libc::c_int != 0;
    }
    if *s.offset(0 as libc::c_int as isize) as libc::c_int == '+' as i32 {
        s = s.offset(1);
    }
    if !(strchr(s, '.' as i32)).is_null() {
        radix = 10 as libc::c_int;
    } else if *s.offset(0 as libc::c_int as isize) as libc::c_int == '0' as i32
        && (*s.offset(1 as libc::c_int as isize) as libc::c_int == 'x' as i32
            || *s.offset(1 as libc::c_int as isize) as libc::c_int == 'X' as i32)
    {
        radix = 16 as libc::c_int;
    } else {
        radix = 8 as libc::c_int;
    }
    return xstrtoumax(
        s,
        0 as *mut *mut libc::c_char,
        radix,
        offset,
        b"Bb\0" as *const u8 as *const libc::c_char,
    ) as libc::c_uint == LONGINT_OK as libc::c_int as libc::c_uint;
}
unsafe extern "C" fn dump() -> bool {
    let mut block: [*mut libc::c_char; 2] = [0 as *mut libc::c_char; 2];
    let mut current_offset: uintmax_t = 0;
    let mut idx: bool = 0 as libc::c_int != 0;
    let mut ok: bool = 1 as libc::c_int != 0;
    let mut n_bytes_read: size_t = 0;
    block[0 as libc::c_int
        as usize] = xnmalloc(2 as libc::c_int as size_t, bytes_per_block)
        as *mut libc::c_char;
    block[1 as libc::c_int
        as usize] = (block[0 as libc::c_int as usize]).offset(bytes_per_block as isize);
    current_offset = n_bytes_to_skip;
    if limit_bytes_to_format {
        loop {
            let mut n_needed: size_t = 0;
            if current_offset >= end_offset {
                n_bytes_read = 0 as libc::c_int as size_t;
                break;
            } else {
                n_needed = if end_offset.wrapping_sub(current_offset) < bytes_per_block {
                    end_offset.wrapping_sub(current_offset)
                } else {
                    bytes_per_block
                };
                ok = (ok as libc::c_int
                    & read_block(n_needed, block[idx as usize], &mut n_bytes_read)
                        as libc::c_int) as bool;
                if n_bytes_read < bytes_per_block {
                    break;
                }
                if n_bytes_read == bytes_per_block {} else {
                    __assert_fail(
                        b"n_bytes_read == bytes_per_block\0" as *const u8
                            as *const libc::c_char,
                        b"src/od.c\0" as *const u8 as *const libc::c_char,
                        1406 as libc::c_int as libc::c_uint,
                        (*::core::mem::transmute::<
                            &[u8; 17],
                            &[libc::c_char; 17],
                        >(b"_Bool dump(void)\0"))
                            .as_ptr(),
                    );
                }
                write_block(
                    current_offset,
                    n_bytes_read,
                    block[!idx as libc::c_int as usize],
                    block[idx as usize],
                );
                current_offset = (current_offset as libc::c_ulong)
                    .wrapping_add(n_bytes_read) as uintmax_t as uintmax_t;
                idx = !idx;
            }
        }
    } else {
        loop {
            ok = (ok as libc::c_int
                & read_block(bytes_per_block, block[idx as usize], &mut n_bytes_read)
                    as libc::c_int) as bool;
            if n_bytes_read < bytes_per_block {
                break;
            }
            if n_bytes_read == bytes_per_block {} else {
                __assert_fail(
                    b"n_bytes_read == bytes_per_block\0" as *const u8
                        as *const libc::c_char,
                    b"src/od.c\0" as *const u8 as *const libc::c_char,
                    1420 as libc::c_int as libc::c_uint,
                    (*::core::mem::transmute::<
                        &[u8; 17],
                        &[libc::c_char; 17],
                    >(b"_Bool dump(void)\0"))
                        .as_ptr(),
                );
            }
            write_block(
                current_offset,
                n_bytes_read,
                block[!idx as libc::c_int as usize],
                block[idx as usize],
            );
            current_offset = (current_offset as libc::c_ulong).wrapping_add(n_bytes_read)
                as uintmax_t as uintmax_t;
            idx = !idx;
        }
    }
    if n_bytes_read > 0 as libc::c_int as libc::c_ulong {
        let mut l_c_m: libc::c_int = 0;
        let mut bytes_to_write: size_t = 0;
        l_c_m = get_lcm();
        bytes_to_write = (l_c_m as libc::c_ulong)
            .wrapping_mul(
                n_bytes_read
                    .wrapping_add(l_c_m as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                    .wrapping_div(l_c_m as libc::c_ulong),
            );
        memset(
            (block[idx as usize]).offset(n_bytes_read as isize) as *mut libc::c_void,
            0 as libc::c_int,
            bytes_to_write.wrapping_sub(n_bytes_read),
        );
        write_block(
            current_offset,
            n_bytes_read,
            block[!idx as libc::c_int as usize],
            block[idx as usize],
        );
        current_offset = (current_offset as libc::c_ulong).wrapping_add(n_bytes_read)
            as uintmax_t as uintmax_t;
    }
    format_address
        .expect(
            "non-null function pointer",
        )(current_offset, '\n' as i32 as libc::c_char);
    if limit_bytes_to_format as libc::c_int != 0 && current_offset >= end_offset {
        ok = (ok as libc::c_int & check_and_close(0 as libc::c_int) as libc::c_int)
            as bool;
    }
    free(block[0 as libc::c_int as usize] as *mut libc::c_void);
    return ok;
}
unsafe extern "C" fn dump_strings() -> bool {
    let mut bufsize: size_t = if 100 as libc::c_int as libc::c_ulong > string_min {
        100 as libc::c_int as libc::c_ulong
    } else {
        string_min
    };
    let mut buf: *mut libc::c_char = xmalloc(bufsize) as *mut libc::c_char;
    let mut address: uintmax_t = n_bytes_to_skip;
    let mut ok: bool = 1 as libc::c_int != 0;
    's_12: loop {
        let mut i: size_t = 0;
        let mut c: libc::c_int = 0;
        '_tryline: loop {
            if limit_bytes_to_format as libc::c_int != 0
                && (end_offset < string_min
                    || end_offset.wrapping_sub(string_min) <= address)
            {
                break 's_12;
            }
            i = 0 as libc::c_int as size_t;
            while i < string_min {
                ok = (ok as libc::c_int & read_char(&mut c) as libc::c_int) as bool;
                address = address.wrapping_add(1);
                if c < 0 as libc::c_int {
                    free(buf as *mut libc::c_void);
                    return ok;
                }
                if *(*__ctype_b_loc()).offset(c as isize) as libc::c_int
                    & _ISprint as libc::c_int as libc::c_ushort as libc::c_int == 0
                {
                    continue '_tryline;
                }
                *buf.offset(i as isize) = c as libc::c_char;
                i = i.wrapping_add(1);
            }
            loop {
                if !(!limit_bytes_to_format || address < end_offset) {
                    break '_tryline;
                }
                if i == bufsize {
                    buf = (if ::core::mem::size_of::<C2RustUnnamed_15>() as libc::c_ulong
                        != 0
                    {
                        x2realloc(buf as *mut libc::c_void, &mut bufsize)
                    } else {
                        x2realloc(buf as *mut libc::c_void, &mut bufsize)
                    }) as *mut libc::c_char;
                }
                ok = (ok as libc::c_int & read_char(&mut c) as libc::c_int) as bool;
                address = address.wrapping_add(1);
                if c < 0 as libc::c_int {
                    free(buf as *mut libc::c_void);
                    return ok;
                }
                if c == '\0' as i32 {
                    break '_tryline;
                }
                if *(*__ctype_b_loc()).offset(c as isize) as libc::c_int
                    & _ISprint as libc::c_int as libc::c_ushort as libc::c_int == 0
                {
                    break;
                }
                let fresh9 = i;
                i = i.wrapping_add(1);
                *buf.offset(fresh9 as isize) = c as libc::c_char;
            }
        }
        *buf.offset(i as isize) = 0 as libc::c_int as libc::c_char;
        format_address
            .expect(
                "non-null function pointer",
            )(
            address.wrapping_sub(i).wrapping_sub(1 as libc::c_int as libc::c_ulong),
            ' ' as i32 as libc::c_char,
        );
        i = 0 as libc::c_int as size_t;
        loop {
            c = *buf.offset(i as isize) as libc::c_int;
            if !(c != 0) {
                break;
            }
            match c {
                7 => {
                    fputs_unlocked(b"\\a\0" as *const u8 as *const libc::c_char, stdout);
                }
                8 => {
                    fputs_unlocked(b"\\b\0" as *const u8 as *const libc::c_char, stdout);
                }
                12 => {
                    fputs_unlocked(b"\\f\0" as *const u8 as *const libc::c_char, stdout);
                }
                10 => {
                    fputs_unlocked(b"\\n\0" as *const u8 as *const libc::c_char, stdout);
                }
                13 => {
                    fputs_unlocked(b"\\r\0" as *const u8 as *const libc::c_char, stdout);
                }
                9 => {
                    fputs_unlocked(b"\\t\0" as *const u8 as *const libc::c_char, stdout);
                }
                11 => {
                    fputs_unlocked(b"\\v\0" as *const u8 as *const libc::c_char, stdout);
                }
                _ => {
                    putc_unlocked(c, stdout);
                }
            }
            i = i.wrapping_add(1);
        }
        putchar_unlocked('\n' as i32);
    }
    free(buf as *mut libc::c_void);
    ok = (ok as libc::c_int & check_and_close(0 as libc::c_int) as libc::c_int) as bool;
    return ok;
}
unsafe fn main_0(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut n_files: libc::c_int = 0;
    let mut i: size_t = 0;
    let mut l_c_m: libc::c_int = 0;
    let mut desired_width: size_t = 0 as libc::c_int as size_t;
    let mut modern: bool = 0 as libc::c_int != 0;
    let mut width_specified: bool = 0 as libc::c_int != 0;
    let mut ok: bool = 1 as libc::c_int != 0;
    let mut width_per_block: size_t = 0 as libc::c_int as size_t;
    static mut multipliers: [libc::c_char; 15] = unsafe {
        *::core::mem::transmute::<&[u8; 15], &[libc::c_char; 15]>(b"bEGKkMmPQRTYZ0\0")
    };
    let mut pseudo_start: uintmax_t = 0 as libc::c_int as uintmax_t;
    set_program_name(*argv.offset(0 as libc::c_int as isize));
    setlocale(6 as libc::c_int, b"\0" as *const u8 as *const libc::c_char);
    bindtextdomain(
        b"coreutils\0" as *const u8 as *const libc::c_char,
        b"/usr/local/share/locale\0" as *const u8 as *const libc::c_char,
    );
    textdomain(b"coreutils\0" as *const u8 as *const libc::c_char);
    atexit(Some(close_stdout as unsafe extern "C" fn() -> ()));
    i = 0 as libc::c_int as size_t;
    while i <= ::core::mem::size_of::<unsigned_long_long_int>() as libc::c_ulong {
        integral_type_size[i as usize] = NO_SIZE;
        i = i.wrapping_add(1);
    }
    integral_type_size[::core::mem::size_of::<libc::c_char>() as libc::c_ulong
        as usize] = CHAR;
    integral_type_size[::core::mem::size_of::<libc::c_short>() as libc::c_ulong
        as usize] = SHORT;
    integral_type_size[::core::mem::size_of::<libc::c_int>() as libc::c_ulong
        as usize] = INT;
    integral_type_size[::core::mem::size_of::<libc::c_long>() as libc::c_ulong
        as usize] = LONG;
    integral_type_size[::core::mem::size_of::<unsigned_long_long_int>() as libc::c_ulong
        as usize] = LONG_LONG;
    i = 0 as libc::c_int as size_t;
    while i <= ::core::mem::size_of::<f128::f128>() as libc::c_ulong {
        fp_type_size[i as usize] = NO_SIZE;
        i = i.wrapping_add(1);
    }
    fp_type_size[::core::mem::size_of::<libc::c_float>() as libc::c_ulong
        as usize] = FLOAT_SINGLE;
    fp_type_size[::core::mem::size_of::<f128::f128>() as libc::c_ulong
        as usize] = FLOAT_LONG_DOUBLE;
    fp_type_size[::core::mem::size_of::<libc::c_double>() as libc::c_ulong
        as usize] = FLOAT_DOUBLE;
    n_specs = 0 as libc::c_int as size_t;
    n_specs_allocated = 0 as libc::c_int as size_t;
    spec = 0 as *mut tspec;
    format_address = Some(
        format_address_std as unsafe extern "C" fn(uintmax_t, libc::c_char) -> (),
    );
    address_base = 8 as libc::c_int;
    address_pad_len = 7 as libc::c_int;
    flag_dump_strings = 0 as libc::c_int != 0;
    loop {
        let mut tmp: uintmax_t = 0;
        let mut s_err: strtol_error = LONGINT_OK;
        let mut oi: libc::c_int = -(1 as libc::c_int);
        let mut c: libc::c_int = getopt_long(
            argc,
            argv,
            short_options.as_ptr(),
            long_options.as_ptr(),
            &mut oi,
        );
        if c == -(1 as libc::c_int) {
            break;
        }
        match c {
            65 => {
                modern = 1 as libc::c_int != 0;
                match *optarg.offset(0 as libc::c_int as isize) as libc::c_int {
                    100 => {
                        format_address = Some(
                            format_address_std
                                as unsafe extern "C" fn(uintmax_t, libc::c_char) -> (),
                        );
                        address_base = 10 as libc::c_int;
                        address_pad_len = 7 as libc::c_int;
                    }
                    111 => {
                        format_address = Some(
                            format_address_std
                                as unsafe extern "C" fn(uintmax_t, libc::c_char) -> (),
                        );
                        address_base = 8 as libc::c_int;
                        address_pad_len = 7 as libc::c_int;
                    }
                    120 => {
                        format_address = Some(
                            format_address_std
                                as unsafe extern "C" fn(uintmax_t, libc::c_char) -> (),
                        );
                        address_base = 16 as libc::c_int;
                        address_pad_len = 6 as libc::c_int;
                    }
                    110 => {
                        format_address = Some(
                            format_address_none
                                as unsafe extern "C" fn(uintmax_t, libc::c_char) -> (),
                        );
                        address_pad_len = 0 as libc::c_int;
                    }
                    _ => {
                        if ::core::mem::size_of::<C2RustUnnamed_21>() as libc::c_ulong
                            != 0
                        {
                            error(
                                1 as libc::c_int,
                                0 as libc::c_int,
                                dcgettext(
                                    0 as *const libc::c_char,
                                    b"invalid output address radix '%c'; it must be one character from [doxn]\0"
                                        as *const u8 as *const libc::c_char,
                                    5 as libc::c_int,
                                ),
                                *optarg.offset(0 as libc::c_int as isize) as libc::c_int,
                            );
                            if 0 as libc::c_int != 0 {} else {
                                unreachable!();
                            };
                        } else {
                            error(
                                1 as libc::c_int,
                                0 as libc::c_int,
                                dcgettext(
                                    0 as *const libc::c_char,
                                    b"invalid output address radix '%c'; it must be one character from [doxn]\0"
                                        as *const u8 as *const libc::c_char,
                                    5 as libc::c_int,
                                ),
                                *optarg.offset(0 as libc::c_int as isize) as libc::c_int,
                            );
                            if 0 as libc::c_int != 0 {} else {
                                unreachable!();
                            };
                        };
                    }
                }
            }
            106 => {
                modern = 1 as libc::c_int != 0;
                s_err = xstrtoumax(
                    optarg,
                    0 as *mut *mut libc::c_char,
                    0 as libc::c_int,
                    &mut n_bytes_to_skip,
                    multipliers.as_ptr(),
                );
                if s_err as libc::c_uint != LONGINT_OK as libc::c_int as libc::c_uint {
                    xstrtol_fatal(
                        s_err,
                        oi,
                        c as libc::c_char,
                        long_options.as_ptr(),
                        optarg,
                    );
                }
            }
            78 => {
                modern = 1 as libc::c_int != 0;
                limit_bytes_to_format = 1 as libc::c_int != 0;
                s_err = xstrtoumax(
                    optarg,
                    0 as *mut *mut libc::c_char,
                    0 as libc::c_int,
                    &mut max_bytes_to_format,
                    multipliers.as_ptr(),
                );
                if s_err as libc::c_uint != LONGINT_OK as libc::c_int as libc::c_uint {
                    xstrtol_fatal(
                        s_err,
                        oi,
                        c as libc::c_char,
                        long_options.as_ptr(),
                        optarg,
                    );
                }
            }
            83 => {
                modern = 1 as libc::c_int != 0;
                if optarg.is_null() {
                    string_min = 3 as libc::c_int as size_t;
                } else {
                    s_err = xstrtoumax(
                        optarg,
                        0 as *mut *mut libc::c_char,
                        0 as libc::c_int,
                        &mut tmp,
                        multipliers.as_ptr(),
                    );
                    if s_err as libc::c_uint != LONGINT_OK as libc::c_int as libc::c_uint
                    {
                        xstrtol_fatal(
                            s_err,
                            oi,
                            c as libc::c_char,
                            long_options.as_ptr(),
                            optarg,
                        );
                    }
                    if (18446744073709551615 as libc::c_ulong) < tmp {
                        if ::core::mem::size_of::<C2RustUnnamed_20>() as libc::c_ulong
                            != 0
                        {
                            error(
                                1 as libc::c_int,
                                0 as libc::c_int,
                                dcgettext(
                                    0 as *const libc::c_char,
                                    b"%s is too large\0" as *const u8 as *const libc::c_char,
                                    5 as libc::c_int,
                                ),
                                quote(optarg),
                            );
                            if 0 as libc::c_int != 0 {} else {
                                unreachable!();
                            };
                        } else {
                            error(
                                1 as libc::c_int,
                                0 as libc::c_int,
                                dcgettext(
                                    0 as *const libc::c_char,
                                    b"%s is too large\0" as *const u8 as *const libc::c_char,
                                    5 as libc::c_int,
                                ),
                                quote(optarg),
                            );
                            if 0 as libc::c_int != 0 {} else {
                                unreachable!();
                            };
                        };
                    }
                    string_min = tmp;
                }
                flag_dump_strings = 1 as libc::c_int != 0;
            }
            116 => {
                modern = 1 as libc::c_int != 0;
                ok = (ok as libc::c_int & decode_format_string(optarg) as libc::c_int)
                    as bool;
            }
            118 => {
                modern = 1 as libc::c_int != 0;
                abbreviate_duplicate_blocks = 0 as libc::c_int != 0;
            }
            256 => {
                traditional = 1 as libc::c_int != 0;
            }
            257 => {
                match endian_types[__xargmatch_internal(
                    b"--endian\0" as *const u8 as *const libc::c_char,
                    optarg,
                    endian_args.as_ptr(),
                    endian_types.as_ptr() as *const libc::c_void,
                    ::core::mem::size_of::<endian_type>() as libc::c_ulong,
                    argmatch_die,
                    1 as libc::c_int != 0,
                ) as usize] as libc::c_uint
                {
                    1 => {
                        input_swap = 0 as libc::c_int == 0;
                    }
                    0 => {
                        input_swap = 0 as libc::c_int != 0;
                    }
                    _ => {}
                }
            }
            97 => {
                ok = (ok as libc::c_int
                    & decode_format_string(b"a\0" as *const u8 as *const libc::c_char)
                        as libc::c_int) as bool;
            }
            98 => {
                ok = (ok as libc::c_int
                    & decode_format_string(b"o1\0" as *const u8 as *const libc::c_char)
                        as libc::c_int) as bool;
            }
            99 => {
                ok = (ok as libc::c_int
                    & decode_format_string(b"c\0" as *const u8 as *const libc::c_char)
                        as libc::c_int) as bool;
            }
            68 => {
                ok = (ok as libc::c_int
                    & decode_format_string(b"u4\0" as *const u8 as *const libc::c_char)
                        as libc::c_int) as bool;
            }
            100 => {
                ok = (ok as libc::c_int
                    & decode_format_string(b"u2\0" as *const u8 as *const libc::c_char)
                        as libc::c_int) as bool;
            }
            70 | 101 => {
                ok = (ok as libc::c_int
                    & decode_format_string(b"fD\0" as *const u8 as *const libc::c_char)
                        as libc::c_int) as bool;
            }
            102 => {
                ok = (ok as libc::c_int
                    & decode_format_string(b"fF\0" as *const u8 as *const libc::c_char)
                        as libc::c_int) as bool;
            }
            88 | 72 => {
                ok = (ok as libc::c_int
                    & decode_format_string(b"x4\0" as *const u8 as *const libc::c_char)
                        as libc::c_int) as bool;
            }
            105 => {
                ok = (ok as libc::c_int
                    & decode_format_string(b"dI\0" as *const u8 as *const libc::c_char)
                        as libc::c_int) as bool;
            }
            73 | 76 | 108 => {
                ok = (ok as libc::c_int
                    & decode_format_string(b"dL\0" as *const u8 as *const libc::c_char)
                        as libc::c_int) as bool;
            }
            79 => {
                ok = (ok as libc::c_int
                    & decode_format_string(b"o4\0" as *const u8 as *const libc::c_char)
                        as libc::c_int) as bool;
            }
            66 | 111 => {
                ok = (ok as libc::c_int
                    & decode_format_string(b"o2\0" as *const u8 as *const libc::c_char)
                        as libc::c_int) as bool;
            }
            115 => {
                ok = (ok as libc::c_int
                    & decode_format_string(b"d2\0" as *const u8 as *const libc::c_char)
                        as libc::c_int) as bool;
            }
            104 | 120 => {
                ok = (ok as libc::c_int
                    & decode_format_string(b"x2\0" as *const u8 as *const libc::c_char)
                        as libc::c_int) as bool;
            }
            119 => {
                modern = 1 as libc::c_int != 0;
                width_specified = 1 as libc::c_int != 0;
                if optarg.is_null() {
                    desired_width = 32 as libc::c_int as size_t;
                } else {
                    let mut w_tmp: uintmax_t = 0;
                    s_err = xstrtoumax(
                        optarg,
                        0 as *mut *mut libc::c_char,
                        10 as libc::c_int,
                        &mut w_tmp,
                        b"\0" as *const u8 as *const libc::c_char,
                    );
                    if s_err as libc::c_uint != LONGINT_OK as libc::c_int as libc::c_uint
                    {
                        xstrtol_fatal(
                            s_err,
                            oi,
                            c as libc::c_char,
                            long_options.as_ptr(),
                            optarg,
                        );
                    }
                    if (18446744073709551615 as libc::c_ulong) < w_tmp {
                        if ::core::mem::size_of::<C2RustUnnamed_19>() as libc::c_ulong
                            != 0
                        {
                            error(
                                1 as libc::c_int,
                                0 as libc::c_int,
                                dcgettext(
                                    0 as *const libc::c_char,
                                    b"%s is too large\0" as *const u8 as *const libc::c_char,
                                    5 as libc::c_int,
                                ),
                                quote(optarg),
                            );
                            if 0 as libc::c_int != 0 {} else {
                                unreachable!();
                            };
                        } else {
                            error(
                                1 as libc::c_int,
                                0 as libc::c_int,
                                dcgettext(
                                    0 as *const libc::c_char,
                                    b"%s is too large\0" as *const u8 as *const libc::c_char,
                                    5 as libc::c_int,
                                ),
                                quote(optarg),
                            );
                            if 0 as libc::c_int != 0 {} else {
                                unreachable!();
                            };
                        };
                    }
                    desired_width = w_tmp;
                }
            }
            -2 => {
                usage(0 as libc::c_int);
            }
            -3 => {
                version_etc(
                    stdout,
                    b"od\0" as *const u8 as *const libc::c_char,
                    b"GNU coreutils\0" as *const u8 as *const libc::c_char,
                    Version,
                    b"Jim Meyering\0" as *const u8 as *const libc::c_char,
                    0 as *mut libc::c_void as *mut libc::c_char,
                );
                exit(0 as libc::c_int);
            }
            _ => {
                usage(1 as libc::c_int);
            }
        }
    }
    if !ok {
        return 1 as libc::c_int;
    }
    if flag_dump_strings as libc::c_int != 0
        && n_specs > 0 as libc::c_int as libc::c_ulong
    {
        if ::core::mem::size_of::<C2RustUnnamed_18>() as libc::c_ulong != 0 {
            error(
                1 as libc::c_int,
                0 as libc::c_int,
                dcgettext(
                    0 as *const libc::c_char,
                    b"no type may be specified when dumping strings\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
            );
            if 0 as libc::c_int != 0 {} else {
                unreachable!();
            };
        } else {
            error(
                1 as libc::c_int,
                0 as libc::c_int,
                dcgettext(
                    0 as *const libc::c_char,
                    b"no type may be specified when dumping strings\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
            );
            if 0 as libc::c_int != 0 {} else {
                unreachable!();
            };
        };
    }
    n_files = argc - optind;
    if !modern || traditional as libc::c_int != 0 {
        let mut o1: uintmax_t = 0;
        let mut o2: uintmax_t = 0;
        match n_files {
            1 => {
                if (traditional as libc::c_int != 0
                    || *(*argv.offset(optind as isize)).offset(0 as libc::c_int as isize)
                        as libc::c_int == '+' as i32)
                    && parse_old_offset(*argv.offset(optind as isize), &mut o1)
                        as libc::c_int != 0
                {
                    n_bytes_to_skip = o1;
                    n_files -= 1;
                    argv = argv.offset(1);
                }
            }
            2 => {
                if (traditional as libc::c_int != 0
                    || *(*argv.offset((optind + 1 as libc::c_int) as isize))
                        .offset(0 as libc::c_int as isize) as libc::c_int == '+' as i32
                    || (*(*argv.offset((optind + 1 as libc::c_int) as isize))
                        .offset(0 as libc::c_int as isize) as libc::c_uint)
                        .wrapping_sub('0' as i32 as libc::c_uint)
                        <= 9 as libc::c_int as libc::c_uint)
                    && parse_old_offset(
                        *argv.offset((optind + 1 as libc::c_int) as isize),
                        &mut o2,
                    ) as libc::c_int != 0
                {
                    if traditional as libc::c_int != 0
                        && parse_old_offset(*argv.offset(optind as isize), &mut o1)
                            as libc::c_int != 0
                    {
                        n_bytes_to_skip = o1;
                        flag_pseudo_start = 1 as libc::c_int != 0;
                        pseudo_start = o2;
                        argv = argv.offset(2 as libc::c_int as isize);
                        n_files -= 2 as libc::c_int;
                    } else {
                        n_bytes_to_skip = o2;
                        n_files -= 1;
                        let ref mut fresh10 = *argv
                            .offset((optind + 1 as libc::c_int) as isize);
                        *fresh10 = *argv.offset(optind as isize);
                        argv = argv.offset(1);
                    }
                }
            }
            3 => {
                if traditional as libc::c_int != 0
                    && parse_old_offset(
                        *argv.offset((optind + 1 as libc::c_int) as isize),
                        &mut o1,
                    ) as libc::c_int != 0
                    && parse_old_offset(
                        *argv.offset((optind + 2 as libc::c_int) as isize),
                        &mut o2,
                    ) as libc::c_int != 0
                {
                    n_bytes_to_skip = o1;
                    flag_pseudo_start = 1 as libc::c_int != 0;
                    pseudo_start = o2;
                    let ref mut fresh11 = *argv
                        .offset((optind + 2 as libc::c_int) as isize);
                    *fresh11 = *argv.offset(optind as isize);
                    argv = argv.offset(2 as libc::c_int as isize);
                    n_files -= 2 as libc::c_int;
                }
            }
            _ => {}
        }
        if traditional as libc::c_int != 0 && (1 as libc::c_int) < n_files {
            error(
                0 as libc::c_int,
                0 as libc::c_int,
                dcgettext(
                    0 as *const libc::c_char,
                    b"extra operand %s\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
                quote(*argv.offset((optind + 1 as libc::c_int) as isize)),
            );
            error(
                0 as libc::c_int,
                0 as libc::c_int,
                b"%s\0" as *const u8 as *const libc::c_char,
                dcgettext(
                    0 as *const libc::c_char,
                    b"compatibility mode supports at most one file\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
            );
            usage(1 as libc::c_int);
        }
    }
    if flag_pseudo_start {
        if format_address
            == Some(
                format_address_none
                    as unsafe extern "C" fn(uintmax_t, libc::c_char) -> (),
            )
        {
            address_base = 8 as libc::c_int;
            address_pad_len = 7 as libc::c_int;
            format_address = Some(
                format_address_paren
                    as unsafe extern "C" fn(uintmax_t, libc::c_char) -> (),
            );
        } else {
            format_address = Some(
                format_address_label
                    as unsafe extern "C" fn(uintmax_t, libc::c_char) -> (),
            );
        }
    }
    if limit_bytes_to_format {
        end_offset = n_bytes_to_skip.wrapping_add(max_bytes_to_format);
        if end_offset < n_bytes_to_skip {
            if ::core::mem::size_of::<C2RustUnnamed_17>() as libc::c_ulong != 0 {
                error(
                    1 as libc::c_int,
                    0 as libc::c_int,
                    dcgettext(
                        0 as *const libc::c_char,
                        b"skip-bytes + read-bytes is too large\0" as *const u8
                            as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                );
                if 0 as libc::c_int != 0 {} else {
                    unreachable!();
                };
            } else {
                error(
                    1 as libc::c_int,
                    0 as libc::c_int,
                    dcgettext(
                        0 as *const libc::c_char,
                        b"skip-bytes + read-bytes is too large\0" as *const u8
                            as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                );
                if 0 as libc::c_int != 0 {} else {
                    unreachable!();
                };
            };
        }
    }
    if n_specs == 0 as libc::c_int as libc::c_ulong {
        decode_format_string(b"oS\0" as *const u8 as *const libc::c_char);
    }
    if n_files > 0 as libc::c_int {
        file_list = &mut *argv.offset(optind as isize) as *mut *mut libc::c_char
            as *const *const libc::c_char;
    } else {
        file_list = default_file_list.as_ptr();
    }
    ok = open_next_file();
    if !in_stream.is_null() {
        ok = (ok as libc::c_int & skip(n_bytes_to_skip) as libc::c_int) as bool;
        if !in_stream.is_null() {
            pseudo_offset = if flag_pseudo_start as libc::c_int != 0 {
                pseudo_start.wrapping_sub(n_bytes_to_skip)
            } else {
                0 as libc::c_int as libc::c_ulong
            };
            l_c_m = get_lcm();
            if width_specified {
                if desired_width != 0 as libc::c_int as libc::c_ulong
                    && desired_width.wrapping_rem(l_c_m as libc::c_ulong)
                        == 0 as libc::c_int as libc::c_ulong
                {
                    bytes_per_block = desired_width;
                } else {
                    error(
                        0 as libc::c_int,
                        0 as libc::c_int,
                        dcgettext(
                            0 as *const libc::c_char,
                            b"warning: invalid width %lu; using %d instead\0"
                                as *const u8 as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                        desired_width,
                        l_c_m,
                    );
                    bytes_per_block = l_c_m as size_t;
                }
            } else if l_c_m < 16 as libc::c_int {
                bytes_per_block = (l_c_m * (16 as libc::c_int / l_c_m)) as size_t;
            } else {
                bytes_per_block = l_c_m as size_t;
            }
            i = 0 as libc::c_int as size_t;
            while i < n_specs {
                let mut fields_per_block: libc::c_int = bytes_per_block
                    .wrapping_div(
                        width_bytes[(*spec.offset(i as isize)).size as usize]
                            as libc::c_ulong,
                    ) as libc::c_int;
                let mut block_width: libc::c_int = ((*spec.offset(i as isize))
                    .field_width + 1 as libc::c_int) * fields_per_block;
                if width_per_block < block_width as libc::c_ulong {
                    width_per_block = block_width as size_t;
                }
                i = i.wrapping_add(1);
            }
            i = 0 as libc::c_int as size_t;
            while i < n_specs {
                let mut fields_per_block_0: libc::c_int = bytes_per_block
                    .wrapping_div(
                        width_bytes[(*spec.offset(i as isize)).size as usize]
                            as libc::c_ulong,
                    ) as libc::c_int;
                let mut block_width_0: libc::c_int = (*spec.offset(i as isize))
                    .field_width * fields_per_block_0;
                (*spec.offset(i as isize))
                    .pad_width = width_per_block
                    .wrapping_sub(block_width_0 as libc::c_ulong) as libc::c_int;
                i = i.wrapping_add(1);
            }
            ok = (ok as libc::c_int
                & if flag_dump_strings as libc::c_int != 0 {
                    dump_strings() as libc::c_int
                } else {
                    dump() as libc::c_int
                }) as bool;
        }
    }
    if have_read_stdin as libc::c_int != 0 && rpl_fclose(stdin) == -(1 as libc::c_int) {
        if ::core::mem::size_of::<C2RustUnnamed_16>() as libc::c_ulong != 0 {
            error(
                1 as libc::c_int,
                *__errno_location(),
                dcgettext(
                    0 as *const libc::c_char,
                    b"standard input\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
            );
            if 0 as libc::c_int != 0 {} else {
                unreachable!();
            };
        } else {
            error(
                1 as libc::c_int,
                *__errno_location(),
                dcgettext(
                    0 as *const libc::c_char,
                    b"standard input\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
            );
            if 0 as libc::c_int != 0 {} else {
                unreachable!();
            };
        };
    }
    return if ok as libc::c_int != 0 { 0 as libc::c_int } else { 1 as libc::c_int };
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
