#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![feature(c_variadic, extern_types)]
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    static mut stdout: *mut FILE;
    static mut stderr: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn fputs_unlocked(__s: *const libc::c_char, __stream: *mut FILE) -> libc::c_int;
    fn stat(__file: *const libc::c_char, __buf: *mut stat) -> libc::c_int;
    fn lstat(__file: *const libc::c_char, __buf: *mut stat) -> libc::c_int;
    fn euidaccess(__name: *const libc::c_char, __type: libc::c_int) -> libc::c_int;
    fn geteuid() -> __uid_t;
    fn getegid() -> __gid_t;
    fn isatty(__fd: libc::c_int) -> libc::c_int;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strncmp(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn __errno_location() -> *mut libc::c_int;
    fn strtol(
        _: *const libc::c_char,
        _: *mut *mut libc::c_char,
        _: libc::c_int,
    ) -> libc::c_long;
    fn abort() -> !;
    fn atexit(__func: Option::<unsafe extern "C" fn() -> ()>) -> libc::c_int;
    fn exit(_: libc::c_int) -> !;
    static mut Version: *const libc::c_char;
    static mut exit_failure: libc::c_int;
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
    fn umaxtostr(_: uintmax_t, _: *mut libc::c_char) -> *mut libc::c_char;
    fn quote_n(n: libc::c_int, arg: *const libc::c_char) -> *const libc::c_char;
    fn quote(arg: *const libc::c_char) -> *const libc::c_char;
    fn strintcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn verror(
        __status: libc::c_int,
        __errnum: libc::c_int,
        __format: *const libc::c_char,
        __args: ::core::ffi::VaList,
    );
}
pub type __builtin_va_list = __va_list;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __va_list {
    pub __stack: *mut libc::c_void,
    pub __gr_top: *mut libc::c_void,
    pub __vr_top: *mut libc::c_void,
    pub __gr_offs: libc::c_int,
    pub __vr_offs: libc::c_int,
}
pub type size_t = libc::c_ulong;
pub type va_list = __builtin_va_list;
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
pub type gid_t = __gid_t;
pub type uid_t = __uid_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
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
pub struct infomap {
    pub program: *const libc::c_char,
    pub node: *const libc::c_char,
}
pub type C2RustUnnamed_0 = libc::c_uint;
pub const TEST_FAILURE: C2RustUnnamed_0 = 2;
pub const TEST_FALSE: C2RustUnnamed_0 = 1;
pub const TEST_TRUE: C2RustUnnamed_0 = 0;
unsafe fn main_0(
    mut margc: libc::c_int,
    mut margv: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut value: bool = false;
    set_program_name(*margv.offset(0 as libc::c_int as isize));
    setlocale(6 as libc::c_int, b"\0" as *const u8 as *const libc::c_char);
    bindtextdomain(
        b"coreutils\0" as *const u8 as *const libc::c_char,
        b"/usr/local/share/locale\0" as *const u8 as *const libc::c_char,
    );
    textdomain(b"coreutils\0" as *const u8 as *const libc::c_char);
    initialize_exit_failure(TEST_FAILURE as libc::c_int);
    atexit(Some(close_stdout as unsafe extern "C" fn() -> ()));
    argv = margv;
    if margc == 2 as libc::c_int {
        if strcmp(
            *margv.offset(1 as libc::c_int as isize),
            b"--help\0" as *const u8 as *const libc::c_char,
        ) == 0 as libc::c_int
        {
            usage(0 as libc::c_int);
        }
        if strcmp(
            *margv.offset(1 as libc::c_int as isize),
            b"--version\0" as *const u8 as *const libc::c_char,
        ) == 0 as libc::c_int
        {
            version_etc(
                stdout,
                b"[\0" as *const u8 as *const libc::c_char,
                b"GNU coreutils\0" as *const u8 as *const libc::c_char,
                Version,
                b"Kevin Braunsdorf\0" as *const u8 as *const libc::c_char,
                b"Matthew Bradburn\0" as *const u8 as *const libc::c_char,
                0 as *mut libc::c_void as *mut libc::c_char,
            );
            return 0 as libc::c_int;
        }
    }
    if margc < 2 as libc::c_int
        || !(strcmp(
            *margv.offset((margc - 1 as libc::c_int) as isize),
            b"]\0" as *const u8 as *const libc::c_char,
        ) == 0 as libc::c_int)
    {
        test_syntax_error(
            dcgettext(
                0 as *const libc::c_char,
                b"missing %s\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            quote(b"]\0" as *const u8 as *const libc::c_char),
        );
    }
    margc -= 1;
    argc = margc;
    pos = 1 as libc::c_int;
    if pos >= argc {
        return TEST_FALSE as libc::c_int;
    }
    value = posixtest(argc - 1 as libc::c_int);
    if pos != argc {
        test_syntax_error(
            dcgettext(
                0 as *const libc::c_char,
                b"extra argument %s\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            quote(*argv.offset(pos as isize)),
        );
    }
    return if value as libc::c_int != 0 {
        TEST_TRUE as libc::c_int
    } else {
        TEST_FALSE as libc::c_int
    };
}
unsafe extern "C" fn posixtest(mut nargs: libc::c_int) -> bool {
    let mut value: bool = false;
    let mut current_block_11: u64;
    match nargs {
        1 => {
            value = one_argument();
            current_block_11 = 7976072742316086414;
        }
        2 => {
            value = two_arguments();
            current_block_11 = 7976072742316086414;
        }
        3 => {
            value = three_arguments();
            current_block_11 = 7976072742316086414;
        }
        4 => {
            if strcmp(
                *argv.offset(pos as isize),
                b"!\0" as *const u8 as *const libc::c_char,
            ) == 0 as libc::c_int
            {
                advance(1 as libc::c_int != 0);
                value = !three_arguments();
                current_block_11 = 7976072742316086414;
            } else if strcmp(
                *argv.offset(pos as isize),
                b"(\0" as *const u8 as *const libc::c_char,
            ) == 0 as libc::c_int
                && strcmp(
                    *argv.offset((pos + 3 as libc::c_int) as isize),
                    b")\0" as *const u8 as *const libc::c_char,
                ) == 0 as libc::c_int
            {
                advance(0 as libc::c_int != 0);
                value = two_arguments();
                advance(0 as libc::c_int != 0);
                current_block_11 = 7976072742316086414;
            } else {
                current_block_11 = 18137218449117516539;
            }
        }
        5 | _ => {
            current_block_11 = 18137218449117516539;
        }
    }
    match current_block_11 {
        18137218449117516539 => {
            if nargs <= 0 as libc::c_int {
                abort();
            }
            value = expr();
        }
        _ => {}
    }
    return value;
}
unsafe extern "C" fn binop(mut s: *const libc::c_char) -> bool {
    return strcmp(s, b"=\0" as *const u8 as *const libc::c_char) == 0 as libc::c_int
        || strcmp(s, b"!=\0" as *const u8 as *const libc::c_char) == 0 as libc::c_int
        || strcmp(s, b"==\0" as *const u8 as *const libc::c_char) == 0 as libc::c_int
        || strcmp(s, b"-nt\0" as *const u8 as *const libc::c_char) == 0 as libc::c_int
        || strcmp(s, b"-ot\0" as *const u8 as *const libc::c_char) == 0 as libc::c_int
        || strcmp(s, b"-ef\0" as *const u8 as *const libc::c_char) == 0 as libc::c_int
        || strcmp(s, b"-eq\0" as *const u8 as *const libc::c_char) == 0 as libc::c_int
        || strcmp(s, b"-ne\0" as *const u8 as *const libc::c_char) == 0 as libc::c_int
        || strcmp(s, b"-lt\0" as *const u8 as *const libc::c_char) == 0 as libc::c_int
        || strcmp(s, b"-le\0" as *const u8 as *const libc::c_char) == 0 as libc::c_int
        || strcmp(s, b"-gt\0" as *const u8 as *const libc::c_char) == 0 as libc::c_int
        || strcmp(s, b"-ge\0" as *const u8 as *const libc::c_char) == 0 as libc::c_int;
}
#[inline]
unsafe extern "C" fn initialize_exit_failure(mut status: libc::c_int) {
    if status != 1 as libc::c_int {
        ::core::ptr::write_volatile(&mut exit_failure as *mut libc::c_int, status);
    }
}
#[inline]
unsafe extern "C" fn timespec_cmp(mut a: timespec, mut b: timespec) -> libc::c_int {
    return 2 as libc::c_int
        * ((a.tv_sec > b.tv_sec) as libc::c_int - (a.tv_sec < b.tv_sec) as libc::c_int)
        + ((a.tv_nsec > b.tv_nsec) as libc::c_int
            - (a.tv_nsec < b.tv_nsec) as libc::c_int);
}
#[inline]
unsafe extern "C" fn to_uchar(mut ch: libc::c_char) -> libc::c_uchar {
    return ch as libc::c_uchar;
}
unsafe extern "C" fn term() -> bool {
    let mut value: bool = false;
    let mut negated: bool = 0 as libc::c_int != 0;
    while pos < argc
        && *(*argv.offset(pos as isize)).offset(0 as libc::c_int as isize) as libc::c_int
            == '!' as i32
        && *(*argv.offset(pos as isize)).offset(1 as libc::c_int as isize) as libc::c_int
            == '\0' as i32
    {
        advance(1 as libc::c_int != 0);
        negated = !negated;
    }
    if pos >= argc {
        beyond();
    }
    if *(*argv.offset(pos as isize)).offset(0 as libc::c_int as isize) as libc::c_int
        == '(' as i32
        && *(*argv.offset(pos as isize)).offset(1 as libc::c_int as isize) as libc::c_int
            == '\0' as i32
    {
        let mut nargs: libc::c_int = 0;
        advance(1 as libc::c_int != 0);
        nargs = 1 as libc::c_int;
        while pos + nargs < argc
            && !(strcmp(
                *argv.offset((pos + nargs) as isize),
                b")\0" as *const u8 as *const libc::c_char,
            ) == 0 as libc::c_int)
        {
            if nargs == 4 as libc::c_int {
                nargs = argc - pos;
                break;
            } else {
                nargs += 1;
            }
        }
        value = posixtest(nargs);
        if (*argv.offset(pos as isize)).is_null() {
            test_syntax_error(
                dcgettext(
                    0 as *const libc::c_char,
                    b"%s expected\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
                quote(b")\0" as *const u8 as *const libc::c_char),
            );
        } else if *(*argv.offset(pos as isize)).offset(0 as libc::c_int as isize)
            as libc::c_int != ')' as i32
            || *(*argv.offset(pos as isize)).offset(1 as libc::c_int as isize)
                as libc::c_int != 0
        {
            test_syntax_error(
                dcgettext(
                    0 as *const libc::c_char,
                    b"%s expected, found %s\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
                quote_n(0 as libc::c_int, b")\0" as *const u8 as *const libc::c_char),
                quote_n(1 as libc::c_int, *argv.offset(pos as isize)),
            );
        }
        advance(0 as libc::c_int != 0);
    } else if 4 as libc::c_int <= argc - pos
        && strcmp(
            *argv.offset(pos as isize),
            b"-l\0" as *const u8 as *const libc::c_char,
        ) == 0 as libc::c_int
        && binop(*argv.offset((pos + 2 as libc::c_int) as isize)) as libc::c_int != 0
    {
        value = binary_operator(1 as libc::c_int != 0);
    } else if 3 as libc::c_int <= argc - pos
        && binop(*argv.offset((pos + 1 as libc::c_int) as isize)) as libc::c_int != 0
    {
        value = binary_operator(0 as libc::c_int != 0);
    } else if *(*argv.offset(pos as isize)).offset(0 as libc::c_int as isize)
        as libc::c_int == '-' as i32
        && *(*argv.offset(pos as isize)).offset(1 as libc::c_int as isize) as libc::c_int
            != 0
        && *(*argv.offset(pos as isize)).offset(2 as libc::c_int as isize) as libc::c_int
            == '\0' as i32
    {
        value = unary_operator();
    } else {
        value = *(*argv.offset(pos as isize)).offset(0 as libc::c_int as isize)
            as libc::c_int != '\0' as i32;
        advance(0 as libc::c_int != 0);
    }
    return negated as libc::c_int ^ value as libc::c_int != 0;
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
        fputs_unlocked(
            dcgettext(
                0 as *const libc::c_char,
                b"Usage: test EXPRESSION\n  or:  test\n  or:  [ EXPRESSION ]\n  or:  [ ]\n  or:  [ OPTION\n\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            stdout,
        );
        fputs_unlocked(
            dcgettext(
                0 as *const libc::c_char,
                b"Exit with the status determined by EXPRESSION.\n\n\0" as *const u8
                    as *const libc::c_char,
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
                b"\nAn omitted EXPRESSION defaults to false.  Otherwise,\nEXPRESSION is true or false and sets exit status.  It is one of:\n\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            stdout,
        );
        fputs_unlocked(
            dcgettext(
                0 as *const libc::c_char,
                b"\n  ( EXPRESSION )               EXPRESSION is true\n  ! EXPRESSION                 EXPRESSION is false\n  EXPRESSION1 -a EXPRESSION2   both EXPRESSION1 and EXPRESSION2 are true\n  EXPRESSION1 -o EXPRESSION2   either EXPRESSION1 or EXPRESSION2 is true\n\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            stdout,
        );
        fputs_unlocked(
            dcgettext(
                0 as *const libc::c_char,
                b"\n  -n STRING            the length of STRING is nonzero\n  STRING               equivalent to -n STRING\n  -z STRING            the length of STRING is zero\n  STRING1 = STRING2    the strings are equal\n  STRING1 != STRING2   the strings are not equal\n\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            stdout,
        );
        fputs_unlocked(
            dcgettext(
                0 as *const libc::c_char,
                b"\n  INTEGER1 -eq INTEGER2   INTEGER1 is equal to INTEGER2\n  INTEGER1 -ge INTEGER2   INTEGER1 is greater than or equal to INTEGER2\n  INTEGER1 -gt INTEGER2   INTEGER1 is greater than INTEGER2\n  INTEGER1 -le INTEGER2   INTEGER1 is less than or equal to INTEGER2\n  INTEGER1 -lt INTEGER2   INTEGER1 is less than INTEGER2\n  INTEGER1 -ne INTEGER2   INTEGER1 is not equal to INTEGER2\n\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            stdout,
        );
        fputs_unlocked(
            dcgettext(
                0 as *const libc::c_char,
                b"\n  FILE1 -ef FILE2   FILE1 and FILE2 have the same device and inode numbers\n  FILE1 -nt FILE2   FILE1 is newer (modification date) than FILE2\n  FILE1 -ot FILE2   FILE1 is older than FILE2\n\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            stdout,
        );
        fputs_unlocked(
            dcgettext(
                0 as *const libc::c_char,
                b"\n  -b FILE     FILE exists and is block special\n  -c FILE     FILE exists and is character special\n  -d FILE     FILE exists and is a directory\n  -e FILE     FILE exists\n\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            stdout,
        );
        fputs_unlocked(
            dcgettext(
                0 as *const libc::c_char,
                b"  -f FILE     FILE exists and is a regular file\n  -g FILE     FILE exists and is set-group-ID\n  -G FILE     FILE exists and is owned by the effective group ID\n  -h FILE     FILE exists and is a symbolic link (same as -L)\n  -k FILE     FILE exists and has its sticky bit set\n\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            stdout,
        );
        fputs_unlocked(
            dcgettext(
                0 as *const libc::c_char,
                b"  -L FILE     FILE exists and is a symbolic link (same as -h)\n  -N FILE     FILE exists and has been modified since it was last read\n  -O FILE     FILE exists and is owned by the effective user ID\n  -p FILE     FILE exists and is a named pipe\n  -r FILE     FILE exists and the user has read access\n  -s FILE     FILE exists and has a size greater than zero\n\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            stdout,
        );
        fputs_unlocked(
            dcgettext(
                0 as *const libc::c_char,
                b"  -S FILE     FILE exists and is a socket\n  -t FD       file descriptor FD is opened on a terminal\n  -u FILE     FILE exists and its set-user-ID bit is set\n  -w FILE     FILE exists and the user has write access\n  -x FILE     FILE exists and the user has execute (or search) access\n\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            stdout,
        );
        fputs_unlocked(
            dcgettext(
                0 as *const libc::c_char,
                b"\nExcept for -h and -L, all FILE-related tests dereference symbolic links.\nBeware that parentheses need to be escaped (e.g., by backslashes) for shells.\nINTEGER may also be -l STRING, which evaluates to the length of STRING.\n\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            stdout,
        );
        fputs_unlocked(
            dcgettext(
                0 as *const libc::c_char,
                b"\nNOTE: Binary -a and -o are inherently ambiguous.  Use 'test EXPR1 && test\nEXPR2' or 'test EXPR1 || test EXPR2' instead.\n\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            stdout,
        );
        fputs_unlocked(
            dcgettext(
                0 as *const libc::c_char,
                b"\nNOTE: [ honors the --help and --version options, but test does not.\ntest treats each of those as it treats any other nonempty STRING.\n\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            stdout,
        );
        printf(
            dcgettext(
                0 as *const libc::c_char,
                b"\nNOTE: your shell may have its own version of %s, which usually supersedes\nthe version described here.  Please refer to your shell's documentation\nfor details about the options it supports.\n\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            dcgettext(
                0 as *const libc::c_char,
                b"test and/or [\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
        emit_ancillary_info(b"[\0" as *const u8 as *const libc::c_char);
    }
    exit(status);
}
#[inline]
unsafe extern "C" fn get_stat_atime(mut st: *const stat) -> timespec {
    return (*st).st_atim;
}
#[inline]
unsafe extern "C" fn get_stat_mtime(mut st: *const stat) -> timespec {
    return (*st).st_mtim;
}
static mut pos: libc::c_int = 0;
static mut argc: libc::c_int = 0;
static mut argv: *mut *mut libc::c_char = 0 as *const *mut libc::c_char
    as *mut *mut libc::c_char;
unsafe extern "C" fn unary_operator() -> bool {
    let mut stat_buf: stat = stat {
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
    match *(*argv.offset(pos as isize)).offset(1 as libc::c_int as isize) as libc::c_int
    {
        101 => {
            unary_advance();
            return stat(*argv.offset((pos - 1 as libc::c_int) as isize), &mut stat_buf)
                == 0 as libc::c_int;
        }
        114 => {
            unary_advance();
            return euidaccess(
                *argv.offset((pos - 1 as libc::c_int) as isize),
                4 as libc::c_int,
            ) == 0 as libc::c_int;
        }
        119 => {
            unary_advance();
            return euidaccess(
                *argv.offset((pos - 1 as libc::c_int) as isize),
                2 as libc::c_int,
            ) == 0 as libc::c_int;
        }
        120 => {
            unary_advance();
            return euidaccess(
                *argv.offset((pos - 1 as libc::c_int) as isize),
                1 as libc::c_int,
            ) == 0 as libc::c_int;
        }
        78 => {
            unary_advance();
            if stat(*argv.offset((pos - 1 as libc::c_int) as isize), &mut stat_buf)
                != 0 as libc::c_int
            {
                return 0 as libc::c_int != 0;
            }
            let mut atime: timespec = get_stat_atime(&mut stat_buf);
            let mut mtime: timespec = get_stat_mtime(&mut stat_buf);
            return timespec_cmp(mtime, atime) > 0 as libc::c_int;
        }
        79 => {
            unary_advance();
            if stat(*argv.offset((pos - 1 as libc::c_int) as isize), &mut stat_buf)
                != 0 as libc::c_int
            {
                return 0 as libc::c_int != 0;
            }
            *__errno_location() = 0 as libc::c_int;
            let mut euid: uid_t = geteuid();
            let mut NO_UID: uid_t = -(1 as libc::c_int) as uid_t;
            return !(euid == NO_UID && *__errno_location() != 0)
                && euid == stat_buf.st_uid;
        }
        71 => {
            unary_advance();
            if stat(*argv.offset((pos - 1 as libc::c_int) as isize), &mut stat_buf)
                != 0 as libc::c_int
            {
                return 0 as libc::c_int != 0;
            }
            *__errno_location() = 0 as libc::c_int;
            let mut egid: gid_t = getegid();
            let mut NO_GID: gid_t = -(1 as libc::c_int) as gid_t;
            return !(egid == NO_GID && *__errno_location() != 0)
                && egid == stat_buf.st_gid;
        }
        102 => {
            unary_advance();
            return stat(*argv.offset((pos - 1 as libc::c_int) as isize), &mut stat_buf)
                == 0 as libc::c_int
                && stat_buf.st_mode & 0o170000 as libc::c_int as libc::c_uint
                    == 0o100000 as libc::c_int as libc::c_uint;
        }
        100 => {
            unary_advance();
            return stat(*argv.offset((pos - 1 as libc::c_int) as isize), &mut stat_buf)
                == 0 as libc::c_int
                && stat_buf.st_mode & 0o170000 as libc::c_int as libc::c_uint
                    == 0o40000 as libc::c_int as libc::c_uint;
        }
        115 => {
            unary_advance();
            return stat(*argv.offset((pos - 1 as libc::c_int) as isize), &mut stat_buf)
                == 0 as libc::c_int
                && (0 as libc::c_int as libc::c_long) < stat_buf.st_size;
        }
        83 => {
            unary_advance();
            return stat(*argv.offset((pos - 1 as libc::c_int) as isize), &mut stat_buf)
                == 0 as libc::c_int
                && stat_buf.st_mode & 0o170000 as libc::c_int as libc::c_uint
                    == 0o140000 as libc::c_int as libc::c_uint;
        }
        99 => {
            unary_advance();
            return stat(*argv.offset((pos - 1 as libc::c_int) as isize), &mut stat_buf)
                == 0 as libc::c_int
                && stat_buf.st_mode & 0o170000 as libc::c_int as libc::c_uint
                    == 0o20000 as libc::c_int as libc::c_uint;
        }
        98 => {
            unary_advance();
            return stat(*argv.offset((pos - 1 as libc::c_int) as isize), &mut stat_buf)
                == 0 as libc::c_int
                && stat_buf.st_mode & 0o170000 as libc::c_int as libc::c_uint
                    == 0o60000 as libc::c_int as libc::c_uint;
        }
        112 => {
            unary_advance();
            return stat(*argv.offset((pos - 1 as libc::c_int) as isize), &mut stat_buf)
                == 0 as libc::c_int
                && stat_buf.st_mode & 0o170000 as libc::c_int as libc::c_uint
                    == 0o10000 as libc::c_int as libc::c_uint;
        }
        76 | 104 => {
            unary_advance();
            return lstat(*argv.offset((pos - 1 as libc::c_int) as isize), &mut stat_buf)
                == 0 as libc::c_int
                && stat_buf.st_mode & 0o170000 as libc::c_int as libc::c_uint
                    == 0o120000 as libc::c_int as libc::c_uint;
        }
        117 => {
            unary_advance();
            return stat(*argv.offset((pos - 1 as libc::c_int) as isize), &mut stat_buf)
                == 0 as libc::c_int
                && stat_buf.st_mode & 0o4000 as libc::c_int as libc::c_uint != 0;
        }
        103 => {
            unary_advance();
            return stat(*argv.offset((pos - 1 as libc::c_int) as isize), &mut stat_buf)
                == 0 as libc::c_int
                && stat_buf.st_mode & 0o2000 as libc::c_int as libc::c_uint != 0;
        }
        107 => {
            unary_advance();
            return stat(*argv.offset((pos - 1 as libc::c_int) as isize), &mut stat_buf)
                == 0 as libc::c_int
                && stat_buf.st_mode & 0o1000 as libc::c_int as libc::c_uint != 0;
        }
        116 => {
            let mut fd: libc::c_long = 0;
            let mut arg: *const libc::c_char = 0 as *const libc::c_char;
            unary_advance();
            arg = find_int(*argv.offset((pos - 1 as libc::c_int) as isize));
            *__errno_location() = 0 as libc::c_int;
            fd = strtol(arg, 0 as *mut *mut libc::c_char, 10 as libc::c_int);
            return *__errno_location() != 34 as libc::c_int
                && 0 as libc::c_int as libc::c_long <= fd
                && fd <= 2147483647 as libc::c_int as libc::c_long
                && isatty(fd as libc::c_int) != 0;
        }
        110 => {
            unary_advance();
            return *(*argv.offset((pos - 1 as libc::c_int) as isize))
                .offset(0 as libc::c_int as isize) as libc::c_int != 0 as libc::c_int;
        }
        122 => {
            unary_advance();
            return *(*argv.offset((pos - 1 as libc::c_int) as isize))
                .offset(0 as libc::c_int as isize) as libc::c_int == '\0' as i32;
        }
        _ => {
            test_syntax_error(
                dcgettext(
                    0 as *const libc::c_char,
                    b"%s: unary operator expected\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
                quote(*argv.offset(pos as isize)),
            );
            return 0 as libc::c_int != 0;
        }
    };
}
unsafe extern "C" fn unary_advance() {
    advance(1 as libc::c_int != 0);
    pos += 1;
}
unsafe extern "C" fn advance(mut f: bool) {
    pos += 1;
    if f as libc::c_int != 0 && pos >= argc {
        beyond();
    }
}
unsafe extern "C" fn beyond() {
    test_syntax_error(
        dcgettext(
            0 as *const libc::c_char,
            b"missing argument after %s\0" as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        ),
        quote(*argv.offset((argc - 1 as libc::c_int) as isize)),
    );
}
unsafe extern "C" fn test_syntax_error(mut format: *const libc::c_char, mut args: ...) {
    let mut ap: ::core::ffi::VaListImpl;
    ap = args.clone();
    verror(0 as libc::c_int, 0 as libc::c_int, format, ap.as_va_list());
    exit(TEST_FAILURE as libc::c_int);
}
unsafe extern "C" fn and() -> bool {
    let mut value: bool = 1 as libc::c_int != 0;
    loop {
        value = (value as libc::c_int & term() as libc::c_int) as bool;
        if !(pos < argc
            && strcmp(
                *argv.offset(pos as isize),
                b"-a\0" as *const u8 as *const libc::c_char,
            ) == 0 as libc::c_int)
        {
            return value;
        }
        advance(0 as libc::c_int != 0);
    };
}
unsafe extern "C" fn find_int(mut string: *const libc::c_char) -> *const libc::c_char {
    let mut p: *const libc::c_char = 0 as *const libc::c_char;
    let mut number_start: *const libc::c_char = 0 as *const libc::c_char;
    p = string;
    while *(*__ctype_b_loc()).offset(to_uchar(*p) as libc::c_int as isize) as libc::c_int
        & _ISblank as libc::c_int as libc::c_ushort as libc::c_int != 0
    {
        p = p.offset(1);
    }
    if *p as libc::c_int == '+' as i32 {
        p = p.offset(1);
        number_start = p;
    } else {
        number_start = p;
        p = p.offset((*p as libc::c_int == '-' as i32) as libc::c_int as isize);
    }
    let fresh0 = p;
    p = p.offset(1);
    if (*fresh0 as libc::c_uint).wrapping_sub('0' as i32 as libc::c_uint)
        <= 9 as libc::c_int as libc::c_uint
    {
        while (*p as libc::c_uint).wrapping_sub('0' as i32 as libc::c_uint)
            <= 9 as libc::c_int as libc::c_uint
        {
            p = p.offset(1);
        }
        while *(*__ctype_b_loc()).offset(to_uchar(*p) as libc::c_int as isize)
            as libc::c_int & _ISblank as libc::c_int as libc::c_ushort as libc::c_int
            != 0
        {
            p = p.offset(1);
        }
        if *p == 0 {
            return number_start;
        }
    }
    test_syntax_error(
        dcgettext(
            0 as *const libc::c_char,
            b"invalid integer %s\0" as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        ),
        quote(string),
    );
    panic!("Reached end of non-void function without returning");
}
unsafe extern "C" fn binary_operator(mut l_is_l: bool) -> bool {
    let mut op: libc::c_int = 0;
    let mut stat_buf: stat = stat {
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
    let mut stat_spare: stat = stat {
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
    let mut r_is_l: bool = false;
    if l_is_l {
        advance(0 as libc::c_int != 0);
    }
    op = pos + 1 as libc::c_int;
    if op < argc - 2 as libc::c_int
        && strcmp(
            *argv.offset((op + 1 as libc::c_int) as isize),
            b"-l\0" as *const u8 as *const libc::c_char,
        ) == 0 as libc::c_int
    {
        r_is_l = 1 as libc::c_int != 0;
        advance(0 as libc::c_int != 0);
    } else {
        r_is_l = 0 as libc::c_int != 0;
    }
    if *(*argv.offset(op as isize)).offset(0 as libc::c_int as isize) as libc::c_int
        == '-' as i32
    {
        if ((*(*argv.offset(op as isize)).offset(1 as libc::c_int as isize)
            as libc::c_int == 'l' as i32
            || *(*argv.offset(op as isize)).offset(1 as libc::c_int as isize)
                as libc::c_int == 'g' as i32)
            && (*(*argv.offset(op as isize)).offset(2 as libc::c_int as isize)
                as libc::c_int == 'e' as i32
                || *(*argv.offset(op as isize)).offset(2 as libc::c_int as isize)
                    as libc::c_int == 't' as i32)
            || *(*argv.offset(op as isize)).offset(1 as libc::c_int as isize)
                as libc::c_int == 'e' as i32
                && *(*argv.offset(op as isize)).offset(2 as libc::c_int as isize)
                    as libc::c_int == 'q' as i32
            || *(*argv.offset(op as isize)).offset(1 as libc::c_int as isize)
                as libc::c_int == 'n' as i32
                && *(*argv.offset(op as isize)).offset(2 as libc::c_int as isize)
                    as libc::c_int == 'e' as i32)
            && *(*argv.offset(op as isize)).offset(3 as libc::c_int as isize) == 0
        {
            let mut lbuf: [libc::c_char; 21] = [0; 21];
            let mut rbuf: [libc::c_char; 21] = [0; 21];
            let mut l: *const libc::c_char = if l_is_l as libc::c_int != 0 {
                umaxtostr(
                    strlen(*argv.offset((op - 1 as libc::c_int) as isize)),
                    lbuf.as_mut_ptr(),
                ) as *const libc::c_char
            } else {
                find_int(*argv.offset((op - 1 as libc::c_int) as isize))
            };
            let mut r: *const libc::c_char = if r_is_l as libc::c_int != 0 {
                umaxtostr(
                    strlen(*argv.offset((op + 2 as libc::c_int) as isize)),
                    rbuf.as_mut_ptr(),
                ) as *const libc::c_char
            } else {
                find_int(*argv.offset((op + 1 as libc::c_int) as isize))
            };
            let mut cmp: libc::c_int = strintcmp(l, r);
            let mut xe_operator: bool = *(*argv.offset(op as isize))
                .offset(2 as libc::c_int as isize) as libc::c_int == 'e' as i32;
            pos += 3 as libc::c_int;
            return if *(*argv.offset(op as isize)).offset(1 as libc::c_int as isize)
                as libc::c_int == 'l' as i32
            {
                (cmp < xe_operator as libc::c_int) as libc::c_int
            } else if *(*argv.offset(op as isize)).offset(1 as libc::c_int as isize)
                as libc::c_int == 'g' as i32
            {
                (cmp > -(xe_operator as libc::c_int)) as libc::c_int
            } else {
                ((cmp != 0 as libc::c_int) as libc::c_int == xe_operator as libc::c_int)
                    as libc::c_int
            } != 0;
        }
        match *(*argv.offset(op as isize)).offset(1 as libc::c_int as isize)
            as libc::c_int
        {
            110 => {
                if *(*argv.offset(op as isize)).offset(2 as libc::c_int as isize)
                    as libc::c_int == 't' as i32
                    && *(*argv.offset(op as isize)).offset(3 as libc::c_int as isize)
                        == 0
                {
                    let mut lt: timespec = timespec { tv_sec: 0, tv_nsec: 0 };
                    let mut rt: timespec = timespec { tv_sec: 0, tv_nsec: 0 };
                    let mut le: bool = false;
                    let mut re: bool = false;
                    pos += 3 as libc::c_int;
                    if l_is_l as libc::c_int != 0 || r_is_l as libc::c_int != 0 {
                        test_syntax_error(
                            dcgettext(
                                0 as *const libc::c_char,
                                b"-nt does not accept -l\0" as *const u8
                                    as *const libc::c_char,
                                5 as libc::c_int,
                            ),
                        );
                    }
                    le = get_mtime(
                        *argv.offset((op - 1 as libc::c_int) as isize),
                        &mut lt,
                    );
                    re = get_mtime(
                        *argv.offset((op + 1 as libc::c_int) as isize),
                        &mut rt,
                    );
                    return le as libc::c_int != 0
                        && (!re || timespec_cmp(lt, rt) > 0 as libc::c_int);
                }
            }
            101 => {
                if *(*argv.offset(op as isize)).offset(2 as libc::c_int as isize)
                    as libc::c_int == 'f' as i32
                    && *(*argv.offset(op as isize)).offset(3 as libc::c_int as isize)
                        == 0
                {
                    pos += 3 as libc::c_int;
                    if l_is_l as libc::c_int != 0 || r_is_l as libc::c_int != 0 {
                        test_syntax_error(
                            dcgettext(
                                0 as *const libc::c_char,
                                b"-ef does not accept -l\0" as *const u8
                                    as *const libc::c_char,
                                5 as libc::c_int,
                            ),
                        );
                    }
                    return stat(
                        *argv.offset((op - 1 as libc::c_int) as isize),
                        &mut stat_buf,
                    ) == 0 as libc::c_int
                        && stat(
                            *argv.offset((op + 1 as libc::c_int) as isize),
                            &mut stat_spare,
                        ) == 0 as libc::c_int && stat_buf.st_dev == stat_spare.st_dev
                        && stat_buf.st_ino == stat_spare.st_ino;
                }
            }
            111 => {
                if 't' as i32
                    == *(*argv.offset(op as isize)).offset(2 as libc::c_int as isize)
                        as libc::c_int
                    && '\0' as i32
                        == *(*argv.offset(op as isize)).offset(3 as libc::c_int as isize)
                            as libc::c_int
                {
                    let mut lt_0: timespec = timespec { tv_sec: 0, tv_nsec: 0 };
                    let mut rt_0: timespec = timespec { tv_sec: 0, tv_nsec: 0 };
                    let mut le_0: bool = false;
                    let mut re_0: bool = false;
                    pos += 3 as libc::c_int;
                    if l_is_l as libc::c_int != 0 || r_is_l as libc::c_int != 0 {
                        test_syntax_error(
                            dcgettext(
                                0 as *const libc::c_char,
                                b"-ot does not accept -l\0" as *const u8
                                    as *const libc::c_char,
                                5 as libc::c_int,
                            ),
                        );
                    }
                    le_0 = get_mtime(
                        *argv.offset((op - 1 as libc::c_int) as isize),
                        &mut lt_0,
                    );
                    re_0 = get_mtime(
                        *argv.offset((op + 1 as libc::c_int) as isize),
                        &mut rt_0,
                    );
                    return re_0 as libc::c_int != 0
                        && (!le_0 || timespec_cmp(lt_0, rt_0) < 0 as libc::c_int);
                }
            }
            _ => {}
        }
        test_syntax_error(
            dcgettext(
                0 as *const libc::c_char,
                b"%s: unknown binary operator\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            quote(*argv.offset(op as isize)),
        );
    }
    if *(*argv.offset(op as isize)).offset(0 as libc::c_int as isize) as libc::c_int
        == '=' as i32
        && (*(*argv.offset(op as isize)).offset(1 as libc::c_int as isize) == 0
            || *(*argv.offset(op as isize)).offset(1 as libc::c_int as isize)
                as libc::c_int == '=' as i32
                && *(*argv.offset(op as isize)).offset(2 as libc::c_int as isize) == 0)
    {
        let mut value: bool = strcmp(
            *argv.offset(pos as isize),
            *argv.offset((pos + 2 as libc::c_int) as isize),
        ) == 0 as libc::c_int;
        pos += 3 as libc::c_int;
        return value;
    }
    if strcmp(*argv.offset(op as isize), b"!=\0" as *const u8 as *const libc::c_char)
        == 0 as libc::c_int
    {
        let mut value_0: bool = !(strcmp(
            *argv.offset(pos as isize),
            *argv.offset((pos + 2 as libc::c_int) as isize),
        ) == 0 as libc::c_int);
        pos += 3 as libc::c_int;
        return value_0;
    }
    abort();
}
unsafe extern "C" fn get_mtime(
    mut filename: *const libc::c_char,
    mut mtime: *mut timespec,
) -> bool {
    let mut finfo: stat = stat {
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
    let mut ok: bool = stat(filename, &mut finfo) == 0 as libc::c_int;
    if ok {
        *mtime = get_stat_mtime(&mut finfo);
    }
    return ok;
}
unsafe extern "C" fn two_arguments() -> bool {
    let mut value: bool = false;
    if strcmp(*argv.offset(pos as isize), b"!\0" as *const u8 as *const libc::c_char)
        == 0 as libc::c_int
    {
        advance(0 as libc::c_int != 0);
        value = !one_argument();
    } else if *(*argv.offset(pos as isize)).offset(0 as libc::c_int as isize)
        as libc::c_int == '-' as i32
        && *(*argv.offset(pos as isize)).offset(1 as libc::c_int as isize) as libc::c_int
            != '\0' as i32
        && *(*argv.offset(pos as isize)).offset(2 as libc::c_int as isize) as libc::c_int
            == '\0' as i32
    {
        value = unary_operator();
    } else {
        beyond();
    }
    return value;
}
unsafe extern "C" fn one_argument() -> bool {
    let fresh2 = pos;
    pos = pos + 1;
    return *(*argv.offset(fresh2 as isize)).offset(0 as libc::c_int as isize)
        as libc::c_int != '\0' as i32;
}
unsafe extern "C" fn three_arguments() -> bool {
    let mut value: bool = false;
    if binop(*argv.offset((pos + 1 as libc::c_int) as isize)) {
        value = binary_operator(0 as libc::c_int != 0);
    } else if strcmp(
        *argv.offset(pos as isize),
        b"!\0" as *const u8 as *const libc::c_char,
    ) == 0 as libc::c_int
    {
        advance(1 as libc::c_int != 0);
        value = !two_arguments();
    } else if strcmp(
        *argv.offset(pos as isize),
        b"(\0" as *const u8 as *const libc::c_char,
    ) == 0 as libc::c_int
        && strcmp(
            *argv.offset((pos + 2 as libc::c_int) as isize),
            b")\0" as *const u8 as *const libc::c_char,
        ) == 0 as libc::c_int
    {
        advance(0 as libc::c_int != 0);
        value = one_argument();
        advance(0 as libc::c_int != 0);
    } else if strcmp(
        *argv.offset((pos + 1 as libc::c_int) as isize),
        b"-a\0" as *const u8 as *const libc::c_char,
    ) == 0 as libc::c_int
        || strcmp(
            *argv.offset((pos + 1 as libc::c_int) as isize),
            b"-o\0" as *const u8 as *const libc::c_char,
        ) == 0 as libc::c_int
    {
        value = expr();
    } else {
        test_syntax_error(
            dcgettext(
                0 as *const libc::c_char,
                b"%s: binary operator expected\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            quote(*argv.offset((pos + 1 as libc::c_int) as isize)),
        );
    }
    return value;
}
unsafe extern "C" fn expr() -> bool {
    if pos >= argc {
        beyond();
    }
    return or();
}
unsafe extern "C" fn or() -> bool {
    let mut value: bool = 0 as libc::c_int != 0;
    loop {
        value = (value as libc::c_int | and() as libc::c_int) as bool;
        if !(pos < argc
            && strcmp(
                *argv.offset(pos as isize),
                b"-o\0" as *const u8 as *const libc::c_char,
            ) == 0 as libc::c_int)
        {
            return value;
        }
        advance(0 as libc::c_int != 0);
    };
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
