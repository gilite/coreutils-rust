#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![feature(extern_types)]
use num_traits::ToPrimitive;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    static mut optarg: *mut libc::c_char;
    fn getopt_long(
        ___argc: libc::c_int,
        ___argv: *const *mut libc::c_char,
        __shortopts: *const libc::c_char,
        __longopts: *const option,
        __longind: *mut libc::c_int,
    ) -> libc::c_int;
    static mut optind: libc::c_int;
    static mut stdout: *mut FILE;
    static mut stderr: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn __overflow(_: *mut FILE, _: libc::c_int) -> libc::c_int;
    fn fputs_unlocked(__s: *const libc::c_char, __stream: *mut FILE) -> libc::c_int;
    fn fwrite_unlocked(
        __ptr: *const libc::c_void,
        __size: size_t,
        __n: size_t,
        __stream: *mut FILE,
    ) -> size_t;
    fn clearerr_unlocked(__stream: *mut FILE);
    fn rpl_asprintf(
        result: *mut *mut libc::c_char,
        format: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn memmove(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn memcmp(
        _: *const libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strncmp(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strcspn(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_ulong;
    fn strspn(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_ulong;
    fn mempcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn free(_: *mut libc::c_void);
    fn __errno_location() -> *mut libc::c_int;
    fn strtol(
        _: *const libc::c_char,
        _: *mut *mut libc::c_char,
        _: libc::c_int,
    ) -> libc::c_long;
    static mut Version: *const libc::c_char;
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    fn setlocale(
        __category: libc::c_int,
        __locale: *const libc::c_char,
    ) -> *mut libc::c_char;
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
    fn xalloc_die();
    fn xmalloc(s: size_t) -> *mut libc::c_void;
    fn xrealloc(p: *mut libc::c_void, s: size_t) -> *mut libc::c_void;
    fn xstrdup(str: *const libc::c_char) -> *mut libc::c_char;
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
    fn atexit(__func: Option::<unsafe extern "C" fn() -> ()>) -> libc::c_int;
    fn exit(_: libc::c_int) -> !;
    fn error(
        __status: libc::c_int,
        __errnum: libc::c_int,
        __format: *const libc::c_char,
        _: ...
    );
    fn cl_strtold(_: *const libc::c_char, _: *mut *mut libc::c_char) -> f128::f128;
    fn quote_n(n: libc::c_int, arg: *const libc::c_char) -> *const libc::c_char;
    fn quote(arg: *const libc::c_char) -> *const libc::c_char;
    fn xstrtold(
        str: *const libc::c_char,
        ptr: *mut *const libc::c_char,
        result: *mut f128::f128,
        convert: Option::<
            unsafe extern "C" fn(
                *const libc::c_char,
                *mut *mut libc::c_char,
            ) -> f128::f128,
        >,
    ) -> bool;
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct option {
    pub name: *const libc::c_char,
    pub has_arg: libc::c_int,
    pub flag: *mut libc::c_int,
    pub val: libc::c_int,
}
pub type size_t = libc::c_ulong;
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
pub type C2RustUnnamed_0 = libc::c_int;
pub const GETOPT_VERSION_CHAR: C2RustUnnamed_0 = -3;
pub const GETOPT_HELP_CHAR: C2RustUnnamed_0 = -2;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct infomap {
    pub program: *const libc::c_char,
    pub node: *const libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct operand {
    pub value: f128::f128,
    pub width: size_t,
    pub precision: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct layout {
    pub prefix_len: size_t,
    pub suffix_len: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_1 {
    pub _gl_dummy: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_2 {
    pub _gl_dummy: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_3 {
    pub _gl_dummy: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_4 {
    pub _gl_dummy: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_5 {
    pub _gl_dummy: libc::c_int,
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
        let fresh0 = (*__stream)._IO_write_ptr;
        (*__stream)._IO_write_ptr = ((*__stream)._IO_write_ptr).offset(1);
        *fresh0 = __c as libc::c_char;
        *fresh0 as libc::c_uchar as libc::c_int
    };
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
unsafe extern "C" fn to_uchar(mut ch: libc::c_char) -> libc::c_uchar {
    return ch as libc::c_uchar;
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
static mut locale_ok: bool = false;
static mut equal_width: bool = false;
static mut separator: *const libc::c_char = 0 as *const libc::c_char;
static mut terminator: [libc::c_char; 2] = unsafe {
    *::core::mem::transmute::<&[u8; 2], &[libc::c_char; 2]>(b"\n\0")
};
static mut long_options: [option; 6] = [
    {
        let mut init = option {
            name: b"equal-width\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'w' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"format\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'f' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"separator\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 's' as i32,
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
                b"Usage: %s [OPTION]... LAST\n  or:  %s [OPTION]... FIRST LAST\n  or:  %s [OPTION]... FIRST INCREMENT LAST\n\0"
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
                b"Print numbers from FIRST to LAST, in steps of INCREMENT.\n\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            stdout,
        );
        emit_mandatory_arg_note();
        fputs_unlocked(
            dcgettext(
                0 as *const libc::c_char,
                b"  -f, --format=FORMAT      use printf style floating-point FORMAT\n  -s, --separator=STRING   use STRING to separate numbers (default: \\n)\n  -w, --equal-width        equalize width by padding with leading zeroes\n\0"
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
                b"\nIf FIRST or INCREMENT is omitted, it defaults to 1.  That is, an\nomitted INCREMENT defaults to 1 even when LAST is smaller than FIRST.\nThe sequence of numbers ends when the sum of the current number and\nINCREMENT would become greater than LAST.\nFIRST, INCREMENT, and LAST are interpreted as floating point values.\nINCREMENT is usually positive if FIRST is smaller than LAST, and\nINCREMENT is usually negative if FIRST is greater than LAST.\nINCREMENT must not be 0; none of FIRST, INCREMENT and LAST may be NaN.\n\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            stdout,
        );
        fputs_unlocked(
            dcgettext(
                0 as *const libc::c_char,
                b"FORMAT must be suitable for printing one argument of type 'double';\nit defaults to %.PRECf if FIRST, INCREMENT, and LAST are all fixed point\ndecimal numbers with maximum precision PREC, and to %g otherwise.\n\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            stdout,
        );
        emit_ancillary_info(b"seq\0" as *const u8 as *const libc::c_char);
    }
    exit(status);
}
unsafe extern "C" fn scan_arg(mut arg: *const libc::c_char) -> operand {
    let mut ret: operand = operand {
        value: f128::f128::ZERO,
        width: 0,
        precision: 0,
    };
    if !xstrtold(
        arg,
        0 as *mut *const libc::c_char,
        &mut ret.value,
        Some(
            cl_strtold
                as unsafe extern "C" fn(
                    *const libc::c_char,
                    *mut *mut libc::c_char,
                ) -> f128::f128,
        ),
    ) {
        error(
            0 as libc::c_int,
            0 as libc::c_int,
            dcgettext(
                0 as *const libc::c_char,
                b"invalid floating point argument: %s\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
            quote(arg),
        );
        usage(1 as libc::c_int);
    }
    if ret.value != ret.value {
        error(
            0 as libc::c_int,
            0 as libc::c_int,
            dcgettext(
                0 as *const libc::c_char,
                b"invalid %s argument: %s\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            quote_n(
                0 as libc::c_int,
                b"not-a-number\0" as *const u8 as *const libc::c_char,
            ),
            quote_n(1 as libc::c_int, arg),
        );
        usage(1 as libc::c_int);
    }
    while *(*__ctype_b_loc()).offset(to_uchar(*arg) as libc::c_int as isize)
        as libc::c_int & _ISspace as libc::c_int as libc::c_ushort as libc::c_int != 0
        || *arg as libc::c_int == '+' as i32
    {
        arg = arg.offset(1);
    }
    ret.width = 0 as libc::c_int as size_t;
    ret.precision = 2147483647 as libc::c_int;
    let mut decimal_point: *const libc::c_char = strchr(arg, '.' as i32);
    if decimal_point.is_null() && (strchr(arg, 'p' as i32)).is_null() {
        ret.precision = 0 as libc::c_int;
    }
    if *arg.offset(strcspn(arg, b"xX\0" as *const u8 as *const libc::c_char) as isize)
        == 0
        && ret.value * f128::f128::new(0 as libc::c_int)
            == f128::f128::new(0 as libc::c_int)
    {
        let mut fraction_len: size_t = 0 as libc::c_int as size_t;
        ret.width = strlen(arg);
        if !decimal_point.is_null() {
            fraction_len = strcspn(
                decimal_point.offset(1 as libc::c_int as isize),
                b"eE\0" as *const u8 as *const libc::c_char,
            );
            if fraction_len <= 2147483647 as libc::c_int as libc::c_ulong {
                ret.precision = fraction_len as libc::c_int;
            }
            ret
                .width = (ret.width as libc::c_ulong)
                .wrapping_add(
                    (if fraction_len == 0 as libc::c_int as libc::c_ulong {
                        -(1 as libc::c_int)
                    } else {
                        (decimal_point == arg
                            || !((*decimal_point.offset(-(1 as libc::c_int) as isize)
                                as libc::c_uint)
                                .wrapping_sub('0' as i32 as libc::c_uint)
                                <= 9 as libc::c_int as libc::c_uint)) as libc::c_int
                    }) as libc::c_ulong,
                ) as size_t as size_t;
        }
        let mut e: *const libc::c_char = strchr(arg, 'e' as i32);
        if e.is_null() {
            e = strchr(arg, 'E' as i32);
        }
        if !e.is_null() {
            let mut exponent: libc::c_long = if strtol(
                e.offset(1 as libc::c_int as isize),
                0 as *mut *mut libc::c_char,
                10 as libc::c_int,
            ) > -(9223372036854775807 as libc::c_long)
            {
                strtol(
                    e.offset(1 as libc::c_int as isize),
                    0 as *mut *mut libc::c_char,
                    10 as libc::c_int,
                )
            } else {
                -(9223372036854775807 as libc::c_long)
            };
            ret
                .precision = (ret.precision as libc::c_long
                + if exponent < 0 as libc::c_int as libc::c_long {
                    -exponent
                } else {
                    -if (ret.precision as libc::c_long) < exponent {
                        ret.precision as libc::c_long
                    } else {
                        exponent
                    }
                }) as libc::c_int;
            ret
                .width = (ret.width as libc::c_ulong)
                .wrapping_sub(
                    (strlen(arg))
                        .wrapping_sub(
                            e.offset_from(arg) as libc::c_long as libc::c_ulong,
                        ),
                ) as size_t as size_t;
            if exponent < 0 as libc::c_int as libc::c_long {
                if !decimal_point.is_null() {
                    if e == decimal_point.offset(1 as libc::c_int as isize) {
                        ret.width = (ret.width).wrapping_add(1);
                    }
                } else {
                    ret.width = (ret.width).wrapping_add(1);
                }
                exponent = -exponent;
            } else {
                if !decimal_point.is_null() && ret.precision == 0 as libc::c_int
                    && fraction_len != 0
                {
                    ret.width = (ret.width).wrapping_sub(1);
                }
                exponent = (exponent as libc::c_ulong)
                    .wrapping_sub(
                        if fraction_len < exponent as libc::c_ulong {
                            fraction_len
                        } else {
                            exponent as libc::c_ulong
                        },
                    ) as libc::c_long as libc::c_long;
            }
            ret
                .width = (ret.width as libc::c_ulong)
                .wrapping_add(exponent as libc::c_ulong) as size_t as size_t;
        }
    }
    return ret;
}
unsafe extern "C" fn long_double_format(
    mut fmt: *const libc::c_char,
    mut layout: *mut layout,
) -> *const libc::c_char {
    let mut i: size_t = 0;
    let mut prefix_len: size_t = 0 as libc::c_int as size_t;
    let mut suffix_len: size_t = 0 as libc::c_int as size_t;
    let mut length_modifier_offset: size_t = 0;
    let mut has_L: bool = false;
    i = 0 as libc::c_int as size_t;
    while !(*fmt.offset(i as isize) as libc::c_int == '%' as i32
        && *fmt.offset(i.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize)
            as libc::c_int != '%' as i32)
    {
        if *fmt.offset(i as isize) == 0 {
            if ::core::mem::size_of::<C2RustUnnamed_4>() as libc::c_ulong != 0 {
                error(
                    1 as libc::c_int,
                    0 as libc::c_int,
                    dcgettext(
                        0 as *const libc::c_char,
                        b"format %s has no %% directive\0" as *const u8
                            as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    quote(fmt),
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
                        b"format %s has no %% directive\0" as *const u8
                            as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    quote(fmt),
                );
                if 0 as libc::c_int != 0 {} else {
                    unreachable!();
                };
            };
        }
        prefix_len = prefix_len.wrapping_add(1);
        i = (i as libc::c_ulong)
            .wrapping_add(
                ((*fmt.offset(i as isize) as libc::c_int == '%' as i32) as libc::c_int
                    + 1 as libc::c_int) as libc::c_ulong,
            ) as size_t as size_t;
    }
    i = i.wrapping_add(1);
    i = (i as libc::c_ulong)
        .wrapping_add(
            strspn(
                fmt.offset(i as isize),
                b"-+#0 '\0" as *const u8 as *const libc::c_char,
            ),
        ) as size_t as size_t;
    i = (i as libc::c_ulong)
        .wrapping_add(
            strspn(
                fmt.offset(i as isize),
                b"0123456789\0" as *const u8 as *const libc::c_char,
            ),
        ) as size_t as size_t;
    if *fmt.offset(i as isize) as libc::c_int == '.' as i32 {
        i = i.wrapping_add(1);
        i = (i as libc::c_ulong)
            .wrapping_add(
                strspn(
                    fmt.offset(i as isize),
                    b"0123456789\0" as *const u8 as *const libc::c_char,
                ),
            ) as size_t as size_t;
    }
    length_modifier_offset = i;
    has_L = *fmt.offset(i as isize) as libc::c_int == 'L' as i32;
    i = (i as libc::c_ulong).wrapping_add(has_L as libc::c_ulong) as size_t as size_t;
    if *fmt.offset(i as isize) as libc::c_int == '\0' as i32 {
        if ::core::mem::size_of::<C2RustUnnamed_3>() as libc::c_ulong != 0 {
            error(
                1 as libc::c_int,
                0 as libc::c_int,
                dcgettext(
                    0 as *const libc::c_char,
                    b"format %s ends in %%\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
                quote(fmt),
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
                    b"format %s ends in %%\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
                quote(fmt),
            );
            if 0 as libc::c_int != 0 {} else {
                unreachable!();
            };
        };
    }
    if (strchr(
        b"efgaEFGA\0" as *const u8 as *const libc::c_char,
        *fmt.offset(i as isize) as libc::c_int,
    ))
        .is_null()
    {
        if ::core::mem::size_of::<C2RustUnnamed_2>() as libc::c_ulong != 0 {
            error(
                1 as libc::c_int,
                0 as libc::c_int,
                dcgettext(
                    0 as *const libc::c_char,
                    b"format %s has unknown %%%c directive\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
                quote(fmt),
                *fmt.offset(i as isize) as libc::c_int,
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
                    b"format %s has unknown %%%c directive\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
                quote(fmt),
                *fmt.offset(i as isize) as libc::c_int,
            );
            if 0 as libc::c_int != 0 {} else {
                unreachable!();
            };
        };
    }
    i = i.wrapping_add(1);
    loop {
        if *fmt.offset(i as isize) as libc::c_int == '%' as i32
            && *fmt.offset(i.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize)
                as libc::c_int != '%' as i32
        {
            if ::core::mem::size_of::<C2RustUnnamed_1>() as libc::c_ulong != 0 {
                error(
                    1 as libc::c_int,
                    0 as libc::c_int,
                    dcgettext(
                        0 as *const libc::c_char,
                        b"format %s has too many %% directives\0" as *const u8
                            as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    quote(fmt),
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
                        b"format %s has too many %% directives\0" as *const u8
                            as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    quote(fmt),
                );
                if 0 as libc::c_int != 0 {} else {
                    unreachable!();
                };
            };
        } else if *fmt.offset(i as isize) != 0 {
            suffix_len = suffix_len.wrapping_add(1);
        } else {
            let mut format_size: size_t = i
                .wrapping_add(1 as libc::c_int as libc::c_ulong);
            let mut ldfmt: *mut libc::c_char = xmalloc(
                format_size.wrapping_add(1 as libc::c_int as libc::c_ulong),
            ) as *mut libc::c_char;
            memcpy(
                ldfmt as *mut libc::c_void,
                fmt as *const libc::c_void,
                length_modifier_offset,
            );
            *ldfmt.offset(length_modifier_offset as isize) = 'L' as i32 as libc::c_char;
            strcpy(
                ldfmt
                    .offset(length_modifier_offset as isize)
                    .offset(1 as libc::c_int as isize),
                fmt
                    .offset(length_modifier_offset as isize)
                    .offset(has_L as libc::c_int as isize),
            );
            (*layout).prefix_len = prefix_len;
            (*layout).suffix_len = suffix_len;
            return ldfmt;
        }
        i = (i as libc::c_ulong)
            .wrapping_add(
                ((*fmt.offset(i as isize) as libc::c_int == '%' as i32) as libc::c_int
                    + 1 as libc::c_int) as libc::c_ulong,
            ) as size_t as size_t;
    };
}
unsafe extern "C" fn io_error() {
    clearerr_unlocked(stdout);
    if ::core::mem::size_of::<C2RustUnnamed_5>() as libc::c_ulong != 0 {
        error(
            1 as libc::c_int,
            *__errno_location(),
            dcgettext(
                0 as *const libc::c_char,
                b"write error\0" as *const u8 as *const libc::c_char,
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
                b"write error\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
        if 0 as libc::c_int != 0 {} else {
            unreachable!();
        };
    };
}
unsafe extern "C" fn print_numbers(
    mut fmt: *const libc::c_char,
    mut layout: layout,
    mut first: f128::f128,
    mut step: f128::f128,
    mut last: f128::f128,
) {
    let mut out_of_range: bool = if step < f128::f128::new(0 as libc::c_int) {
        (first < last) as libc::c_int
    } else {
        (last < first) as libc::c_int
    } != 0;
    if !out_of_range {
        let mut x: f128::f128 = first;
        let mut i: f128::f128 = f128::f128::ZERO;
        i = f128::f128::new(1 as libc::c_int);
        loop {
            let mut x0: f128::f128 = x;
            if printf(fmt, x) < 0 as libc::c_int {
                io_error();
            }
            if out_of_range {
                break;
            }
            x = first + i * step;
            out_of_range = if step < f128::f128::new(0 as libc::c_int) {
                (x < last) as libc::c_int
            } else {
                (last < x) as libc::c_int
            } != 0;
            if out_of_range {
                let mut print_extra_number: bool = 0 as libc::c_int != 0;
                let mut x_val: f128::f128 = f128::f128::ZERO;
                let mut x_str: *mut libc::c_char = 0 as *mut libc::c_char;
                let mut x_strlen: libc::c_int = 0;
                if locale_ok {
                    setlocale(
                        1 as libc::c_int,
                        b"C\0" as *const u8 as *const libc::c_char,
                    );
                }
                x_strlen = rpl_asprintf(&mut x_str as *mut *mut libc::c_char, fmt, x);
                if locale_ok {
                    setlocale(
                        1 as libc::c_int,
                        b"\0" as *const u8 as *const libc::c_char,
                    );
                }
                if x_strlen < 0 as libc::c_int {
                    xalloc_die();
                }
                *x_str
                    .offset(
                        (x_strlen as libc::c_ulong).wrapping_sub(layout.suffix_len)
                            as isize,
                    ) = '\0' as i32 as libc::c_char;
                if xstrtold(
                    x_str.offset(layout.prefix_len as isize),
                    0 as *mut *const libc::c_char,
                    &mut x_val,
                    Some(
                        cl_strtold
                            as unsafe extern "C" fn(
                                *const libc::c_char,
                                *mut *mut libc::c_char,
                            ) -> f128::f128,
                    ),
                ) as libc::c_int != 0 && x_val == last
                {
                    let mut x0_str: *mut libc::c_char = 0 as *mut libc::c_char;
                    let mut x0_strlen: libc::c_int = rpl_asprintf(
                        &mut x0_str as *mut *mut libc::c_char,
                        fmt,
                        x0,
                    );
                    if x0_strlen < 0 as libc::c_int {
                        xalloc_die();
                    }
                    *x0_str
                        .offset(
                            (x0_strlen as libc::c_ulong).wrapping_sub(layout.suffix_len)
                                as isize,
                        ) = '\0' as i32 as libc::c_char;
                    print_extra_number = !(strcmp(x0_str, x_str) == 0 as libc::c_int);
                    free(x0_str as *mut libc::c_void);
                }
                free(x_str as *mut libc::c_void);
                if !print_extra_number {
                    break;
                }
            }
            if fputs_unlocked(separator, stdout) == -(1 as libc::c_int) {
                io_error();
            }
            i += f128::f128::new(1.);
        }
        if fputs_unlocked(terminator.as_ptr(), stdout) == -(1 as libc::c_int) {
            io_error();
        }
    }
}
unsafe extern "C" fn get_default_format(
    mut first: operand,
    mut step: operand,
    mut last: operand,
) -> *const libc::c_char {
    static mut format_buf: [libc::c_char; 28] = [0; 28];
    let mut prec: libc::c_int = if first.precision > step.precision {
        first.precision
    } else {
        step.precision
    };
    if prec != 2147483647 as libc::c_int && last.precision != 2147483647 as libc::c_int {
        if equal_width {
            let mut first_width: size_t = (first.width)
                .wrapping_add((prec - first.precision) as libc::c_ulong);
            let mut last_width: size_t = (last.width)
                .wrapping_add((prec - last.precision) as libc::c_ulong);
            if last.precision != 0 && prec == 0 as libc::c_int {
                last_width = last_width.wrapping_sub(1);
            }
            if last.precision == 0 as libc::c_int && prec != 0 {
                last_width = last_width.wrapping_add(1);
            }
            if first.precision == 0 as libc::c_int && prec != 0 {
                first_width = first_width.wrapping_add(1);
            }
            let mut width: size_t = if first_width > last_width {
                first_width
            } else {
                last_width
            };
            if width <= 2147483647 as libc::c_int as libc::c_ulong {
                let mut w: libc::c_int = width as libc::c_int;
                sprintf(
                    format_buf.as_mut_ptr(),
                    b"%%0%d.%dLf\0" as *const u8 as *const libc::c_char,
                    w,
                    prec,
                );
                return format_buf.as_mut_ptr();
            }
        } else {
            sprintf(
                format_buf.as_mut_ptr(),
                b"%%.%dLf\0" as *const u8 as *const libc::c_char,
                prec,
            );
            return format_buf.as_mut_ptr();
        }
    }
    return b"%Lg\0" as *const u8 as *const libc::c_char;
}
unsafe extern "C" fn incr(mut s0: *mut *mut libc::c_char, mut s_len: *mut size_t) {
    let mut s: *mut libc::c_char = *s0;
    let mut endp: *mut libc::c_char = s
        .offset(*s_len as isize)
        .offset(-(1 as libc::c_int as isize));
    loop {
        let fresh1 = *endp;
        *endp = *endp + 1;
        if (fresh1 as libc::c_int) < '9' as i32 {
            return;
        }
        let fresh2 = endp;
        endp = endp.offset(-1);
        *fresh2 = '0' as i32 as libc::c_char;
        if !(endp >= s) {
            break;
        }
    }
    *s0 = (*s0).offset(-1);
    **s0 = '1' as i32 as libc::c_char;
    *s_len = (*s_len).wrapping_add(1);
}
unsafe extern "C" fn cmp(
    mut a: *const libc::c_char,
    mut a_len: size_t,
    mut b: *const libc::c_char,
    mut b_len: size_t,
) -> libc::c_int {
    if a_len < b_len {
        return -(1 as libc::c_int);
    }
    if b_len < a_len {
        return 1 as libc::c_int;
    }
    return memcmp(a as *const libc::c_void, b as *const libc::c_void, a_len);
}
unsafe extern "C" fn trim_leading_zeros(
    mut s: *const libc::c_char,
) -> *const libc::c_char {
    let mut p: *const libc::c_char = s;
    while *s as libc::c_int == '0' as i32 {
        s = s.offset(1);
    }
    if *s == 0 && s != p {
        s = s.offset(-1);
    }
    return s;
}
unsafe extern "C" fn seq_fast(
    mut a: *const libc::c_char,
    mut b: *const libc::c_char,
    mut step: uintmax_t,
) {
    let mut inf: bool = strcmp(b, b"inf\0" as *const u8 as *const libc::c_char)
        == 0 as libc::c_int;
    a = trim_leading_zeros(a);
    b = trim_leading_zeros(b);
    let mut p_len: size_t = strlen(a);
    let mut q_len: size_t = if inf as libc::c_int != 0 {
        0 as libc::c_int as libc::c_ulong
    } else {
        strlen(b)
    };
    let mut inc_size: size_t = if (if p_len
        .wrapping_add(1 as libc::c_int as libc::c_ulong) > q_len
    {
        p_len.wrapping_add(1 as libc::c_int as libc::c_ulong)
    } else {
        q_len
    }) > 31 as libc::c_int as libc::c_ulong
    {
        if p_len.wrapping_add(1 as libc::c_int as libc::c_ulong) > q_len {
            p_len.wrapping_add(1 as libc::c_int as libc::c_ulong)
        } else {
            q_len
        }
    } else {
        31 as libc::c_int as libc::c_ulong
    };
    let mut p0: *mut libc::c_char = xmalloc(
        inc_size.wrapping_add(1 as libc::c_int as libc::c_ulong),
    ) as *mut libc::c_char;
    let mut p: *mut libc::c_char = memcpy(
        p0.offset(inc_size as isize).offset(-(p_len as isize)) as *mut libc::c_void,
        a as *const libc::c_void,
        p_len.wrapping_add(1 as libc::c_int as libc::c_ulong),
    ) as *mut libc::c_char;
    let mut q: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut q0: *mut libc::c_char = 0 as *mut libc::c_char;
    if !inf {
        q0 = xmalloc(inc_size.wrapping_add(1 as libc::c_int as libc::c_ulong))
            as *mut libc::c_char;
        q = memcpy(
            q0.offset(inc_size as isize).offset(-(q_len as isize)) as *mut libc::c_void,
            b as *const libc::c_void,
            q_len.wrapping_add(1 as libc::c_int as libc::c_ulong),
        ) as *mut libc::c_char;
    } else {
        q0 = 0 as *mut libc::c_char;
        q = q0;
    }
    let mut ok: bool = inf as libc::c_int != 0
        || cmp(p, p_len, q, q_len) <= 0 as libc::c_int;
    if ok {
        let mut buf_size: size_t = if 8192 as libc::c_int as libc::c_ulong
            > inc_size
                .wrapping_add(1 as libc::c_int as libc::c_ulong)
                .wrapping_mul(2 as libc::c_int as libc::c_ulong)
        {
            8192 as libc::c_int as libc::c_ulong
        } else {
            inc_size
                .wrapping_add(1 as libc::c_int as libc::c_ulong)
                .wrapping_mul(2 as libc::c_int as libc::c_ulong)
        };
        let mut buf: *mut libc::c_char = xmalloc(buf_size) as *mut libc::c_char;
        let mut buf_end: *const libc::c_char = buf.offset(buf_size as isize);
        let mut bufp: *mut libc::c_char = buf;
        bufp = mempcpy(bufp as *mut libc::c_void, p as *const libc::c_void, p_len)
            as *mut libc::c_char;
        loop {
            let mut n_incr: uintmax_t = step;
            while n_incr != 0 {
                incr(&mut p, &mut p_len);
                n_incr = n_incr.wrapping_sub(1);
            }
            if !inf && (0 as libc::c_int) < cmp(p, p_len, q, q_len) {
                break;
            }
            let fresh3 = bufp;
            bufp = bufp.offset(1);
            *fresh3 = *separator;
            if p_len == inc_size {
                inc_size = (inc_size as libc::c_ulong)
                    .wrapping_mul(2 as libc::c_int as libc::c_ulong) as size_t as size_t;
                p0 = xrealloc(
                    p0 as *mut libc::c_void,
                    inc_size.wrapping_add(1 as libc::c_int as libc::c_ulong),
                ) as *mut libc::c_char;
                p = memmove(
                    p0.offset(p_len as isize) as *mut libc::c_void,
                    p0 as *const libc::c_void,
                    p_len.wrapping_add(1 as libc::c_int as libc::c_ulong),
                ) as *mut libc::c_char;
                if buf_size
                    < inc_size
                        .wrapping_add(1 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(2 as libc::c_int as libc::c_ulong)
                {
                    let mut buf_offset: size_t = bufp.offset_from(buf) as libc::c_long
                        as size_t;
                    buf_size = inc_size
                        .wrapping_add(1 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(2 as libc::c_int as libc::c_ulong);
                    buf = xrealloc(buf as *mut libc::c_void, buf_size)
                        as *mut libc::c_char;
                    buf_end = buf.offset(buf_size as isize);
                    bufp = buf.offset(buf_offset as isize);
                }
            }
            bufp = mempcpy(bufp as *mut libc::c_void, p as *const libc::c_void, p_len)
                as *mut libc::c_char;
            if buf_end
                .offset(
                    -(p_len.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize),
                ) < bufp as *const libc::c_char
            {
                if (if 0 != 0 && 0 != 0
                    && (bufp.offset_from(buf) as libc::c_long as size_t)
                        .wrapping_mul(1 as libc::c_int as size_t)
                        <= 8 as libc::c_int as libc::c_ulong
                    && bufp.offset_from(buf) as libc::c_long as size_t
                        != 0 as libc::c_int as libc::c_ulong
                {
                    ({
                        let mut __ptr: *const libc::c_char = buf as *const libc::c_char;
                        let mut __stream: *mut FILE = stdout;
                        let mut __cnt: size_t = 0;
                        __cnt = (bufp.offset_from(buf) as libc::c_long as size_t)
                            .wrapping_mul(1 as libc::c_int as size_t);
                        while __cnt > 0 as libc::c_int as libc::c_ulong {
                            let fresh4 = __ptr;
                            __ptr = __ptr.offset(1);
                            if putc_unlocked(*fresh4 as libc::c_int, __stream)
                                == -(1 as libc::c_int)
                            {
                                break;
                            }
                            __cnt = __cnt.wrapping_sub(1);
                        }
                        (bufp.offset_from(buf) as libc::c_long as size_t)
                            .wrapping_mul(1 as libc::c_int as size_t)
                            .wrapping_sub(__cnt)
                            .wrapping_div(
                                bufp.offset_from(buf) as libc::c_long as size_t,
                            )
                    })
                } else {
                    (if 0 != 0
                        && bufp.offset_from(buf) as libc::c_long as size_t
                            == 0 as libc::c_int as libc::c_ulong
                        || 0 != 0
                            && 1 as libc::c_int as size_t
                                == 0 as libc::c_int as libc::c_ulong
                    {
                        0 as libc::c_int as size_t
                    } else {
                        fwrite_unlocked(
                            buf as *const libc::c_void,
                            bufp.offset_from(buf) as libc::c_long as size_t,
                            1 as libc::c_int as size_t,
                            stdout,
                        )
                    })
                }) != 1 as libc::c_int as libc::c_ulong
                {
                    io_error();
                }
                bufp = buf;
            }
        }
        let fresh5 = bufp;
        bufp = bufp.offset(1);
        *fresh5 = *terminator.as_ptr();
        if (if 0 != 0 && 0 != 0
            && (bufp.offset_from(buf) as libc::c_long as size_t)
                .wrapping_mul(1 as libc::c_int as size_t)
                <= 8 as libc::c_int as libc::c_ulong
            && bufp.offset_from(buf) as libc::c_long as size_t
                != 0 as libc::c_int as libc::c_ulong
        {
            ({
                let mut __ptr: *const libc::c_char = buf as *const libc::c_char;
                let mut __stream: *mut FILE = stdout;
                let mut __cnt: size_t = 0;
                __cnt = (bufp.offset_from(buf) as libc::c_long as size_t)
                    .wrapping_mul(1 as libc::c_int as size_t);
                while __cnt > 0 as libc::c_int as libc::c_ulong {
                    let fresh6 = __ptr;
                    __ptr = __ptr.offset(1);
                    if putc_unlocked(*fresh6 as libc::c_int, __stream)
                        == -(1 as libc::c_int)
                    {
                        break;
                    }
                    __cnt = __cnt.wrapping_sub(1);
                }
                (bufp.offset_from(buf) as libc::c_long as size_t)
                    .wrapping_mul(1 as libc::c_int as size_t)
                    .wrapping_sub(__cnt)
                    .wrapping_div(bufp.offset_from(buf) as libc::c_long as size_t)
            })
        } else {
            (if 0 != 0
                && bufp.offset_from(buf) as libc::c_long as size_t
                    == 0 as libc::c_int as libc::c_ulong
                || 0 != 0
                    && 1 as libc::c_int as size_t == 0 as libc::c_int as libc::c_ulong
            {
                0 as libc::c_int as size_t
            } else {
                fwrite_unlocked(
                    buf as *const libc::c_void,
                    bufp.offset_from(buf) as libc::c_long as size_t,
                    1 as libc::c_int as size_t,
                    stdout,
                )
            })
        }) != 1 as libc::c_int as libc::c_ulong
        {
            io_error();
        }
    }
    if ok {
        exit(0 as libc::c_int);
    }
    free(p0 as *mut libc::c_void);
    free(q0 as *mut libc::c_void);
}
unsafe extern "C" fn all_digits_p(mut s: *const libc::c_char) -> bool {
    let mut n: size_t = strlen(s);
    return (*s.offset(0 as libc::c_int as isize) as libc::c_uint)
        .wrapping_sub('0' as i32 as libc::c_uint) <= 9 as libc::c_int as libc::c_uint
        && n == strspn(s, b"0123456789\0" as *const u8 as *const libc::c_char);
}
unsafe fn main_0(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut optc: libc::c_int = 0;
    let mut first: operand = {
        let mut init = operand {
            value: f128::f128::new(1 as libc::c_int),
            width: 1 as libc::c_int as size_t,
            precision: 0 as libc::c_int,
        };
        init
    };
    let mut step: operand = {
        let mut init = operand {
            value: f128::f128::new(1 as libc::c_int),
            width: 1 as libc::c_int as size_t,
            precision: 0 as libc::c_int,
        };
        init
    };
    let mut last: operand = operand {
        value: f128::f128::ZERO,
        width: 0,
        precision: 0,
    };
    let mut layout: layout = {
        let mut init = layout {
            prefix_len: 0 as libc::c_int as size_t,
            suffix_len: 0 as libc::c_int as size_t,
        };
        init
    };
    let mut format_str: *const libc::c_char = 0 as *const libc::c_char;
    set_program_name(*argv.offset(0 as libc::c_int as isize));
    locale_ok = !(setlocale(6 as libc::c_int, b"\0" as *const u8 as *const libc::c_char))
        .is_null();
    bindtextdomain(
        b"coreutils\0" as *const u8 as *const libc::c_char,
        b"/usr/local/share/locale\0" as *const u8 as *const libc::c_char,
    );
    textdomain(b"coreutils\0" as *const u8 as *const libc::c_char);
    atexit(Some(close_stdout as unsafe extern "C" fn() -> ()));
    equal_width = 0 as libc::c_int != 0;
    separator = b"\n\0" as *const u8 as *const libc::c_char;
    while optind < argc {
        if *(*argv.offset(optind as isize)).offset(0 as libc::c_int as isize)
            as libc::c_int == '-' as i32
            && {
                optc = *(*argv.offset(optind as isize)).offset(1 as libc::c_int as isize)
                    as libc::c_int;
                optc == '.' as i32
                    || (optc as libc::c_uint).wrapping_sub('0' as i32 as libc::c_uint)
                        <= 9 as libc::c_int as libc::c_uint
            }
        {
            break;
        }
        optc = getopt_long(
            argc,
            argv,
            b"+f:s:w\0" as *const u8 as *const libc::c_char,
            long_options.as_ptr(),
            0 as *mut libc::c_int,
        );
        if optc == -(1 as libc::c_int) {
            break;
        }
        match optc {
            102 => {
                format_str = optarg;
            }
            115 => {
                separator = optarg;
            }
            119 => {
                equal_width = 1 as libc::c_int != 0;
            }
            -2 => {
                usage(0 as libc::c_int);
            }
            -3 => {
                version_etc(
                    stdout,
                    b"seq\0" as *const u8 as *const libc::c_char,
                    b"GNU coreutils\0" as *const u8 as *const libc::c_char,
                    Version,
                    b"Ulrich Drepper\0" as *const u8 as *const libc::c_char,
                    0 as *mut libc::c_void as *mut libc::c_char,
                );
                exit(0 as libc::c_int);
            }
            _ => {
                usage(1 as libc::c_int);
            }
        }
    }
    let mut n_args: libc::c_uint = (argc - optind) as libc::c_uint;
    if n_args < 1 as libc::c_int as libc::c_uint {
        error(
            0 as libc::c_int,
            0 as libc::c_int,
            dcgettext(
                0 as *const libc::c_char,
                b"missing operand\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
        usage(1 as libc::c_int);
    }
    if (3 as libc::c_int as libc::c_uint) < n_args {
        error(
            0 as libc::c_int,
            0 as libc::c_int,
            dcgettext(
                0 as *const libc::c_char,
                b"extra operand %s\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            quote(*argv.offset((optind + 3 as libc::c_int) as isize)),
        );
        usage(1 as libc::c_int);
    }
    if !format_str.is_null() {
        format_str = long_double_format(format_str, &mut layout);
    }
    if !format_str.is_null() && equal_width as libc::c_int != 0 {
        error(
            0 as libc::c_int,
            0 as libc::c_int,
            dcgettext(
                0 as *const libc::c_char,
                b"format string may not be specified when printing equal width strings\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
        usage(1 as libc::c_int);
    }
    let mut fast_step_ok: bool = 0 as libc::c_int != 0;
    if n_args != 3 as libc::c_int as libc::c_uint
        || all_digits_p(*argv.offset((optind + 1 as libc::c_int) as isize))
            as libc::c_int != 0
            && xstrtold(
                *argv.offset((optind + 1 as libc::c_int) as isize),
                0 as *mut *const libc::c_char,
                &mut step.value,
                Some(
                    cl_strtold
                        as unsafe extern "C" fn(
                            *const libc::c_char,
                            *mut *mut libc::c_char,
                        ) -> f128::f128,
                ),
            ) as libc::c_int != 0 && f128::f128::new(0 as libc::c_int) < step.value
            && step.value <= f128::f128::new(200 as libc::c_int)
    {
        fast_step_ok = 1 as libc::c_int != 0;
    }
    if all_digits_p(*argv.offset(optind as isize)) as libc::c_int != 0
        && (n_args == 1 as libc::c_int as libc::c_uint
            || all_digits_p(*argv.offset((optind + 1 as libc::c_int) as isize))
                as libc::c_int != 0)
        && (n_args < 3 as libc::c_int as libc::c_uint
            || fast_step_ok as libc::c_int != 0
                && all_digits_p(*argv.offset((optind + 2 as libc::c_int) as isize))
                    as libc::c_int != 0) && !equal_width && format_str.is_null()
        && strlen(separator) == 1 as libc::c_int as libc::c_ulong
    {
        let mut s1: *const libc::c_char = if n_args == 1 as libc::c_int as libc::c_uint {
            b"1\0" as *const u8 as *const libc::c_char
        } else {
            *argv.offset(optind as isize) as *const libc::c_char
        };
        let mut s2: *const libc::c_char = *argv
            .offset(
                (optind as libc::c_uint)
                    .wrapping_add(n_args.wrapping_sub(1 as libc::c_int as libc::c_uint))
                    as isize,
            );
        seq_fast(s1, s2, (step.value).to_u64().unwrap());
    }
    let fresh7 = optind;
    optind = optind + 1;
    last = scan_arg(*argv.offset(fresh7 as isize));
    if optind < argc {
        first = last;
        let fresh8 = optind;
        optind = optind + 1;
        last = scan_arg(*argv.offset(fresh8 as isize));
        if optind < argc {
            step = last;
            if step.value == f128::f128::new(0 as libc::c_int) {
                error(
                    0 as libc::c_int,
                    0 as libc::c_int,
                    dcgettext(
                        0 as *const libc::c_char,
                        b"invalid Zero increment value: %s\0" as *const u8
                            as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    quote(*argv.offset((optind - 1 as libc::c_int) as isize)),
                );
                usage(1 as libc::c_int);
            }
            let fresh9 = optind;
            optind = optind + 1;
            last = scan_arg(*argv.offset(fresh9 as isize));
        }
    }
    if first.precision == 0 as libc::c_int && step.precision == 0 as libc::c_int
        && last.precision == 0 as libc::c_int
        && first.value * f128::f128::new(0 as libc::c_int)
            == f128::f128::new(0 as libc::c_int)
        && f128::f128::new(0 as libc::c_int) <= first.value
        && f128::f128::new(0 as libc::c_int) <= last.value
        && f128::f128::new(0 as libc::c_int) < step.value
        && step.value <= f128::f128::new(200 as libc::c_int) && !equal_width
        && format_str.is_null() && strlen(separator) == 1 as libc::c_int as libc::c_ulong
    {
        let mut s1_0: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut s2_0: *mut libc::c_char = 0 as *mut libc::c_char;
        if rpl_asprintf(
            &mut s1_0 as *mut *mut libc::c_char,
            b"%0.Lf\0" as *const u8 as *const libc::c_char,
            first.value,
        ) < 0 as libc::c_int
        {
            xalloc_die();
        }
        if !(last.value * f128::f128::new(0 as libc::c_int)
            == f128::f128::new(0 as libc::c_int))
        {
            s2_0 = xstrdup(b"inf\0" as *const u8 as *const libc::c_char);
        } else if rpl_asprintf(
            &mut s2_0 as *mut *mut libc::c_char,
            b"%0.Lf\0" as *const u8 as *const libc::c_char,
            last.value,
        ) < 0 as libc::c_int
        {
            xalloc_die();
        }
        if *s1_0 as libc::c_int != '-' as i32 && *s2_0 as libc::c_int != '-' as i32 {
            seq_fast(s1_0, s2_0, (step.value).to_u64().unwrap());
        }
        free(s1_0 as *mut libc::c_void);
        free(s2_0 as *mut libc::c_void);
    }
    if format_str.is_null() {
        format_str = get_default_format(first, step, last);
    }
    print_numbers(format_str, layout, first.value, step.value, last.value);
    exit(0 as libc::c_int);
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
