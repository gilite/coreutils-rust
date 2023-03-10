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
    static mut stdout: *mut FILE;
    static mut stderr: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn fputs_unlocked(__s: *const libc::c_char, __stream: *mut FILE) -> libc::c_int;
    fn rpl_asprintf(
        result: *mut *mut libc::c_char,
        format: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn getopt_long(
        ___argc: libc::c_int,
        ___argv: *const *mut libc::c_char,
        __shortopts: *const libc::c_char,
        __longopts: *const option,
        __longind: *mut libc::c_int,
    ) -> libc::c_int;
    static mut optarg: *mut libc::c_char;
    static mut optind: libc::c_int;
    fn access(__name: *const libc::c_char, __type: libc::c_int) -> libc::c_int;
    fn execvp(
        __file: *const libc::c_char,
        __argv: *const *mut libc::c_char,
    ) -> libc::c_int;
    fn stat(__file: *const libc::c_char, __buf: *mut stat) -> libc::c_int;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strncmp(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strtok(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn free(_: *mut libc::c_void);
    fn __errno_location() -> *mut libc::c_int;
    static mut Version: *const libc::c_char;
    static mut exit_failure: libc::c_int;
    fn __ctype_toupper_loc() -> *mut *const __int32_t;
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
    fn xstrdup(str: *const libc::c_char) -> *mut libc::c_char;
    fn dir_name(file: *const libc::c_char) -> *mut libc::c_char;
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
    fn exit(_: libc::c_int) -> !;
    fn getenv(__name: *const libc::c_char) -> *mut libc::c_char;
    fn putenv(__string: *mut libc::c_char) -> libc::c_int;
    fn atexit(__func: Option::<unsafe extern "C" fn() -> ()>) -> libc::c_int;
    fn error(
        __status: libc::c_int,
        __errnum: libc::c_int,
        __format: *const libc::c_char,
        _: ...
    );
    fn file_name_concat(
        dir: *const libc::c_char,
        base: *const libc::c_char,
        base_in_result: *mut *mut libc::c_char,
    ) -> *mut libc::c_char;
    fn quote(arg: *const libc::c_char) -> *const libc::c_char;
    fn xreadlink(filename: *const libc::c_char) -> *mut libc::c_char;
    fn xstrtoumax(
        _: *const libc::c_char,
        _: *mut *mut libc::c_char,
        _: libc::c_int,
        _: *mut uintmax_t,
        _: *const libc::c_char,
    ) -> strtol_error;
}
pub type size_t = libc::c_ulong;
pub type __int32_t = libc::c_int;
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
pub type C2RustUnnamed = libc::c_uint;
pub const EXIT_ENOENT: C2RustUnnamed = 127;
pub const EXIT_CANNOT_INVOKE: C2RustUnnamed = 126;
pub const EXIT_CANCELED: C2RustUnnamed = 125;
pub const EXIT_TIMEDOUT: C2RustUnnamed = 124;
pub type uintmax_t = __uintmax_t;
pub type C2RustUnnamed_0 = libc::c_int;
pub const GETOPT_VERSION_CHAR: C2RustUnnamed_0 = -3;
pub const GETOPT_HELP_CHAR: C2RustUnnamed_0 = -2;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct infomap {
    pub program: *const libc::c_char,
    pub node: *const libc::c_char,
}
pub type strtol_error = libc::c_uint;
pub const LONGINT_INVALID: strtol_error = 4;
pub const LONGINT_INVALID_SUFFIX_CHAR_WITH_OVERFLOW: strtol_error = 3;
pub const LONGINT_INVALID_SUFFIX_CHAR: strtol_error = 2;
pub const LONGINT_OVERFLOW: strtol_error = 1;
pub const LONGINT_OK: strtol_error = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_1 {
    pub size: size_t,
    pub optc: libc::c_int,
    pub optarg: *mut libc::c_char,
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
unsafe extern "C" fn initialize_exit_failure(mut status: libc::c_int) {
    if status != 1 as libc::c_int {
        ::core::ptr::write_volatile(&mut exit_failure as *mut libc::c_int, status);
    }
}
#[inline]
unsafe extern "C" fn toupper(mut __c: libc::c_int) -> libc::c_int {
    return if __c >= -(128 as libc::c_int) && __c < 256 as libc::c_int {
        *(*__ctype_toupper_loc()).offset(__c as isize)
    } else {
        __c
    };
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
unsafe extern "C" fn emit_exec_status(mut program: *const libc::c_char) {
    printf(
        dcgettext(
            0 as *const libc::c_char,
            b"\nExit status:\n  125  if the %s command itself fails\n  126  if COMMAND is found but cannot be invoked\n  127  if COMMAND cannot be found\n  -    the exit status of COMMAND otherwise\n\0"
                as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        ),
        program,
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
unsafe extern "C" fn c_isspace(mut c: libc::c_int) -> bool {
    match c {
        32 | 9 | 10 | 11 | 12 | 13 => return 1 as libc::c_int != 0,
        _ => return 0 as libc::c_int != 0,
    };
}
static mut program_path: *mut libc::c_char = 0 as *const libc::c_char
    as *mut libc::c_char;
static mut stdbuf: [C2RustUnnamed_1; 3] = [C2RustUnnamed_1 {
    size: 0,
    optc: 0,
    optarg: 0 as *const libc::c_char as *mut libc::c_char,
}; 3];
static mut longopts: [option; 6] = [
    {
        let mut init = option {
            name: b"input\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'i' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"output\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'o' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"error\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'e' as i32,
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
unsafe extern "C" fn parse_size(
    mut str: *const libc::c_char,
    mut size: *mut size_t,
) -> libc::c_int {
    let mut tmp_size: uintmax_t = 0;
    let mut e: strtol_error = xstrtoumax(
        str,
        0 as *mut *mut libc::c_char,
        10 as libc::c_int,
        &mut tmp_size,
        b"EGkKMPQRTYZ0\0" as *const u8 as *const libc::c_char,
    );
    if e as libc::c_uint == LONGINT_OK as libc::c_int as libc::c_uint
        && (18446744073709551615 as libc::c_ulong) < tmp_size
    {
        e = LONGINT_OVERFLOW;
    }
    if e as libc::c_uint == LONGINT_OK as libc::c_int as libc::c_uint {
        *__errno_location() = 0 as libc::c_int;
        *size = tmp_size;
        return 0 as libc::c_int;
    }
    *__errno_location() = if e as libc::c_uint
        == LONGINT_OVERFLOW as libc::c_int as libc::c_uint
    {
        75 as libc::c_int
    } else {
        *__errno_location()
    };
    return -(1 as libc::c_int);
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
        printf(
            dcgettext(
                0 as *const libc::c_char,
                b"Usage: %s OPTION... COMMAND\n\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            program_name,
        );
        fputs_unlocked(
            dcgettext(
                0 as *const libc::c_char,
                b"Run COMMAND, with modified buffering operations for its standard streams.\n\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            stdout,
        );
        emit_mandatory_arg_note();
        fputs_unlocked(
            dcgettext(
                0 as *const libc::c_char,
                b"  -i, --input=MODE   adjust standard input stream buffering\n  -o, --output=MODE  adjust standard output stream buffering\n  -e, --error=MODE   adjust standard error stream buffering\n\0"
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
                b"\nIf MODE is 'L' the corresponding stream will be line buffered.\nThis option is invalid with standard input.\n\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            stdout,
        );
        fputs_unlocked(
            dcgettext(
                0 as *const libc::c_char,
                b"\nIf MODE is '0' the corresponding stream will be unbuffered.\n\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            stdout,
        );
        fputs_unlocked(
            dcgettext(
                0 as *const libc::c_char,
                b"\nOtherwise MODE is a number which may be followed by one of the following:\nKB 1000, K 1024, MB 1000*1000, M 1024*1024, and so on for G,T,P,E,Z,Y,R,Q.\nBinary prefixes can be used, too: KiB=K, MiB=M, and so on.\nIn this case the corresponding stream will be fully buffered with the buffer\nsize set to MODE bytes.\n\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            stdout,
        );
        fputs_unlocked(
            dcgettext(
                0 as *const libc::c_char,
                b"\nNOTE: If COMMAND adjusts the buffering of its standard streams ('tee' does\nfor example) then that will override corresponding changes by 'stdbuf'.\nAlso some filters (like 'dd' and 'cat' etc.) don't use streams for I/O,\nand are thus unaffected by 'stdbuf' settings.\n\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            stdout,
        );
        emit_exec_status(b"stdbuf\0" as *const u8 as *const libc::c_char);
        emit_ancillary_info(b"stdbuf\0" as *const u8 as *const libc::c_char);
    }
    exit(status);
}
unsafe extern "C" fn set_program_path(mut arg: *const libc::c_char) {
    if !(strchr(arg, '/' as i32)).is_null() {
        program_path = dir_name(arg);
    } else {
        let mut path: *mut libc::c_char = xreadlink(
            b"/proc/self/exe\0" as *const u8 as *const libc::c_char,
        );
        if !path.is_null() {
            program_path = dir_name(path);
        } else {
            path = getenv(b"PATH\0" as *const u8 as *const libc::c_char);
            if !path.is_null() {
                let mut dir: *mut libc::c_char = 0 as *mut libc::c_char;
                path = xstrdup(path);
                dir = strtok(path, b":\0" as *const u8 as *const libc::c_char);
                while !dir.is_null() {
                    let mut candidate: *mut libc::c_char = file_name_concat(
                        dir,
                        arg,
                        0 as *mut *mut libc::c_char,
                    );
                    if access(candidate, 1 as libc::c_int) == 0 as libc::c_int {
                        program_path = dir_name(candidate);
                        free(candidate as *mut libc::c_void);
                        break;
                    } else {
                        free(candidate as *mut libc::c_void);
                        dir = strtok(
                            0 as *mut libc::c_char,
                            b":\0" as *const u8 as *const libc::c_char,
                        );
                    }
                }
            }
        }
        free(path as *mut libc::c_void);
    };
}
unsafe extern "C" fn optc_to_fileno(mut c: libc::c_int) -> libc::c_int {
    let mut ret: libc::c_int = -(1 as libc::c_int);
    match c {
        101 => {
            ret = 2 as libc::c_int;
        }
        105 => {
            ret = 0 as libc::c_int;
        }
        111 => {
            ret = 1 as libc::c_int;
        }
        _ => {}
    }
    return ret;
}
unsafe extern "C" fn set_LD_PRELOAD() {
    let mut ret: libc::c_int = 0;
    let mut preload_env: *const libc::c_char = b"LD_PRELOAD\0" as *const u8
        as *const libc::c_char;
    let mut old_libs: *mut libc::c_char = getenv(preload_env);
    let mut LD_PRELOAD: *mut libc::c_char = 0 as *mut libc::c_char;
    let search_path: [*const libc::c_char; 3] = [
        program_path as *const libc::c_char,
        b"/usr/local/libexec/coreutils\0" as *const u8 as *const libc::c_char,
        0 as *const libc::c_char,
    ];
    let mut path: *const *const libc::c_char = search_path.as_ptr();
    let mut libstdbuf: *mut libc::c_char = 0 as *mut libc::c_char;
    loop {
        let mut sb: stat = stat {
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
        if **path == 0 {
            libstdbuf = xstrdup(b"libstdbuf.so\0" as *const u8 as *const libc::c_char);
            break;
        } else {
            ret = rpl_asprintf(
                &mut libstdbuf as *mut *mut libc::c_char,
                b"%s/%s\0" as *const u8 as *const libc::c_char,
                *path,
                b"libstdbuf.so\0" as *const u8 as *const libc::c_char,
            );
            if ret < 0 as libc::c_int {
                xalloc_die();
            }
            if stat(libstdbuf, &mut sb) == 0 as libc::c_int {
                break;
            }
            free(libstdbuf as *mut libc::c_void);
            path = path.offset(1);
            if (*path).is_null() {
                if ::core::mem::size_of::<C2RustUnnamed_3>() as libc::c_ulong != 0 {
                    error(
                        EXIT_CANCELED as libc::c_int,
                        0 as libc::c_int,
                        dcgettext(
                            0 as *const libc::c_char,
                            b"failed to find %s\0" as *const u8 as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                        quote(b"libstdbuf.so\0" as *const u8 as *const libc::c_char),
                    );
                    if 0 as libc::c_int != 0 {} else {
                        unreachable!();
                    };
                } else {
                    error(
                        EXIT_CANCELED as libc::c_int,
                        0 as libc::c_int,
                        dcgettext(
                            0 as *const libc::c_char,
                            b"failed to find %s\0" as *const u8 as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                        quote(b"libstdbuf.so\0" as *const u8 as *const libc::c_char),
                    );
                    if 0 as libc::c_int != 0 {} else {
                        unreachable!();
                    };
                };
            }
        }
    }
    if !old_libs.is_null() {
        ret = rpl_asprintf(
            &mut LD_PRELOAD as *mut *mut libc::c_char,
            b"%s=%s:%s\0" as *const u8 as *const libc::c_char,
            preload_env,
            old_libs,
            libstdbuf,
        );
    } else {
        ret = rpl_asprintf(
            &mut LD_PRELOAD as *mut *mut libc::c_char,
            b"%s=%s\0" as *const u8 as *const libc::c_char,
            preload_env,
            libstdbuf,
        );
    }
    if ret < 0 as libc::c_int {
        xalloc_die();
    }
    free(libstdbuf as *mut libc::c_void);
    ret = putenv(LD_PRELOAD);
    if ret != 0 as libc::c_int {
        if ::core::mem::size_of::<C2RustUnnamed_2>() as libc::c_ulong != 0 {
            error(
                EXIT_CANCELED as libc::c_int,
                *__errno_location(),
                dcgettext(
                    0 as *const libc::c_char,
                    b"failed to update the environment with %s\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
                quote(LD_PRELOAD),
            );
            if 0 as libc::c_int != 0 {} else {
                unreachable!();
            };
        } else {
            error(
                EXIT_CANCELED as libc::c_int,
                *__errno_location(),
                dcgettext(
                    0 as *const libc::c_char,
                    b"failed to update the environment with %s\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
                quote(LD_PRELOAD),
            );
            if 0 as libc::c_int != 0 {} else {
                unreachable!();
            };
        };
    }
}
unsafe extern "C" fn set_libstdbuf_options() -> bool {
    let mut env_set: bool = 0 as libc::c_int != 0;
    let mut i: size_t = 0 as libc::c_int as size_t;
    while i
        < (::core::mem::size_of::<[C2RustUnnamed_1; 3]>() as libc::c_ulong)
            .wrapping_div(::core::mem::size_of::<C2RustUnnamed_1>() as libc::c_ulong)
    {
        if !(stdbuf[i as usize].optarg).is_null() {
            let mut var: *mut libc::c_char = 0 as *mut libc::c_char;
            let mut ret: libc::c_int = 0;
            if *stdbuf[i as usize].optarg as libc::c_int == 'L' as i32 {
                ret = rpl_asprintf(
                    &mut var as *mut *mut libc::c_char,
                    b"%s%c=L\0" as *const u8 as *const libc::c_char,
                    b"_STDBUF_\0" as *const u8 as *const libc::c_char,
                    ({
                        let mut __res: libc::c_int = 0;
                        if ::core::mem::size_of::<libc::c_int>() as libc::c_ulong
                            > 1 as libc::c_int as libc::c_ulong
                        {
                            if 0 != 0 {
                                let mut __c: libc::c_int = stdbuf[i as usize].optc;
                                __res = if __c < -(128 as libc::c_int)
                                    || __c > 255 as libc::c_int
                                {
                                    __c
                                } else {
                                    *(*__ctype_toupper_loc()).offset(__c as isize)
                                };
                            } else {
                                __res = toupper(stdbuf[i as usize].optc);
                            }
                        } else {
                            __res = *(*__ctype_toupper_loc())
                                .offset(stdbuf[i as usize].optc as isize);
                        }
                        __res
                    }),
                );
            } else {
                ret = rpl_asprintf(
                    &mut var as *mut *mut libc::c_char,
                    b"%s%c=%lu\0" as *const u8 as *const libc::c_char,
                    b"_STDBUF_\0" as *const u8 as *const libc::c_char,
                    ({
                        let mut __res: libc::c_int = 0;
                        if ::core::mem::size_of::<libc::c_int>() as libc::c_ulong
                            > 1 as libc::c_int as libc::c_ulong
                        {
                            if 0 != 0 {
                                let mut __c: libc::c_int = stdbuf[i as usize].optc;
                                __res = if __c < -(128 as libc::c_int)
                                    || __c > 255 as libc::c_int
                                {
                                    __c
                                } else {
                                    *(*__ctype_toupper_loc()).offset(__c as isize)
                                };
                            } else {
                                __res = toupper(stdbuf[i as usize].optc);
                            }
                        } else {
                            __res = *(*__ctype_toupper_loc())
                                .offset(stdbuf[i as usize].optc as isize);
                        }
                        __res
                    }),
                    stdbuf[i as usize].size,
                );
            }
            if ret < 0 as libc::c_int {
                xalloc_die();
            }
            if putenv(var) != 0 as libc::c_int {
                if ::core::mem::size_of::<C2RustUnnamed_4>() as libc::c_ulong != 0 {
                    error(
                        EXIT_CANCELED as libc::c_int,
                        *__errno_location(),
                        dcgettext(
                            0 as *const libc::c_char,
                            b"failed to update the environment with %s\0" as *const u8
                                as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                        quote(var),
                    );
                    if 0 as libc::c_int != 0 {} else {
                        unreachable!();
                    };
                } else {
                    error(
                        EXIT_CANCELED as libc::c_int,
                        *__errno_location(),
                        dcgettext(
                            0 as *const libc::c_char,
                            b"failed to update the environment with %s\0" as *const u8
                                as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                        quote(var),
                    );
                    if 0 as libc::c_int != 0 {} else {
                        unreachable!();
                    };
                };
            }
            env_set = 1 as libc::c_int != 0;
        }
        i = i.wrapping_add(1);
    }
    return env_set;
}
unsafe fn main_0(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut c: libc::c_int = 0;
    set_program_name(*argv.offset(0 as libc::c_int as isize));
    setlocale(6 as libc::c_int, b"\0" as *const u8 as *const libc::c_char);
    bindtextdomain(
        b"coreutils\0" as *const u8 as *const libc::c_char,
        b"/usr/local/share/locale\0" as *const u8 as *const libc::c_char,
    );
    textdomain(b"coreutils\0" as *const u8 as *const libc::c_char);
    initialize_exit_failure(EXIT_CANCELED as libc::c_int);
    atexit(Some(close_stdout as unsafe extern "C" fn() -> ()));
    loop {
        c = getopt_long(
            argc,
            argv,
            b"+i:o:e:\0" as *const u8 as *const libc::c_char,
            longopts.as_ptr(),
            0 as *mut libc::c_int,
        );
        if !(c != -(1 as libc::c_int)) {
            break;
        }
        let mut opt_fileno: libc::c_int = 0;
        match c {
            101 | 105 | 111 => {
                opt_fileno = optc_to_fileno(c);
                if 0 as libc::c_int <= opt_fileno
                    && (opt_fileno as libc::c_ulong)
                        < (::core::mem::size_of::<[C2RustUnnamed_1; 3]>()
                            as libc::c_ulong)
                            .wrapping_div(
                                ::core::mem::size_of::<C2RustUnnamed_1>() as libc::c_ulong,
                            )
                {} else {
                    __assert_fail(
                        b"0 <= opt_fileno && opt_fileno < ARRAY_CARDINALITY (stdbuf)\0"
                            as *const u8 as *const libc::c_char,
                        b"src/stdbuf.c\0" as *const u8 as *const libc::c_char,
                        337 as libc::c_int as libc::c_uint,
                        (*::core::mem::transmute::<
                            &[u8; 23],
                            &[libc::c_char; 23],
                        >(b"int main(int, char **)\0"))
                            .as_ptr(),
                    );
                }
                stdbuf[opt_fileno as usize].optc = c;
                while c_isspace(*optarg as libc::c_int) {
                    optarg = optarg.offset(1);
                }
                stdbuf[opt_fileno as usize].optarg = optarg;
                if c == 'i' as i32 && *optarg as libc::c_int == 'L' as i32 {
                    error(
                        0 as libc::c_int,
                        0 as libc::c_int,
                        dcgettext(
                            0 as *const libc::c_char,
                            b"line buffering stdin is meaningless\0" as *const u8
                                as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                    );
                    usage(EXIT_CANCELED as libc::c_int);
                }
                if !(strcmp(optarg, b"L\0" as *const u8 as *const libc::c_char)
                    == 0 as libc::c_int)
                    && parse_size(
                        optarg,
                        &mut (*stdbuf.as_mut_ptr().offset(opt_fileno as isize)).size,
                    ) == -(1 as libc::c_int)
                {
                    if ::core::mem::size_of::<C2RustUnnamed_5>() as libc::c_ulong != 0 {
                        error(
                            EXIT_CANCELED as libc::c_int,
                            *__errno_location(),
                            dcgettext(
                                0 as *const libc::c_char,
                                b"invalid mode %s\0" as *const u8 as *const libc::c_char,
                                5 as libc::c_int,
                            ),
                            quote(optarg),
                        );
                        if 0 as libc::c_int != 0 {} else {
                            unreachable!();
                        };
                    } else {
                        error(
                            EXIT_CANCELED as libc::c_int,
                            *__errno_location(),
                            dcgettext(
                                0 as *const libc::c_char,
                                b"invalid mode %s\0" as *const u8 as *const libc::c_char,
                                5 as libc::c_int,
                            ),
                            quote(optarg),
                        );
                        if 0 as libc::c_int != 0 {} else {
                            unreachable!();
                        };
                    };
                }
            }
            -2 => {
                usage(0 as libc::c_int);
            }
            -3 => {
                version_etc(
                    stdout,
                    b"stdbuf\0" as *const u8 as *const libc::c_char,
                    b"GNU coreutils\0" as *const u8 as *const libc::c_char,
                    Version,
                    b"Padraig Brady\0" as *const u8 as *const libc::c_char,
                    0 as *mut libc::c_void as *mut libc::c_char,
                );
                exit(0 as libc::c_int);
            }
            _ => {
                usage(EXIT_CANCELED as libc::c_int);
            }
        }
    }
    argv = argv.offset(optind as isize);
    argc -= optind;
    if argc < 1 as libc::c_int {
        error(
            0 as libc::c_int,
            0 as libc::c_int,
            dcgettext(
                0 as *const libc::c_char,
                b"missing operand\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
        usage(EXIT_CANCELED as libc::c_int);
    }
    if !set_libstdbuf_options() {
        error(
            0 as libc::c_int,
            0 as libc::c_int,
            dcgettext(
                0 as *const libc::c_char,
                b"you must specify a buffering mode option\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
        usage(EXIT_CANCELED as libc::c_int);
    }
    set_program_path(program_name);
    if program_path.is_null() {
        program_path = xstrdup(
            b"/usr/local/lib/coreutils\0" as *const u8 as *const libc::c_char,
        );
    }
    set_LD_PRELOAD();
    free(program_path as *mut libc::c_void);
    execvp(*argv, argv as *const *mut libc::c_char);
    let mut exit_status: libc::c_int = if *__errno_location() == 2 as libc::c_int {
        EXIT_ENOENT as libc::c_int
    } else {
        EXIT_CANNOT_INVOKE as libc::c_int
    };
    error(
        0 as libc::c_int,
        *__errno_location(),
        dcgettext(
            0 as *const libc::c_char,
            b"failed to run command %s\0" as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        ),
        quote(*argv.offset(0 as libc::c_int as isize)),
    );
    return exit_status;
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
