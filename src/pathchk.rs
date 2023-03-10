#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![feature(extern_types)]
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    static mut stdout: *mut FILE;
    static mut stderr: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn fputs_unlocked(__s: *const libc::c_char, __stream: *mut FILE) -> libc::c_int;
    fn getopt_long(
        ___argc: libc::c_int,
        ___argv: *const *mut libc::c_char,
        __shortopts: *const libc::c_char,
        __longopts: *const option,
        __longind: *mut libc::c_int,
    ) -> libc::c_int;
    static mut optind: libc::c_int;
    fn rpl_mbrlen(s: *const libc::c_char, n: size_t, ps: *mut mbstate_t) -> size_t;
    fn lstat(__file: *const libc::c_char, __buf: *mut stat) -> libc::c_int;
    fn pathconf(__path: *const libc::c_char, __name: libc::c_int) -> libc::c_long;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strncmp(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strspn(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_ulong;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn __errno_location() -> *mut libc::c_int;
    fn quotearg_n_style_colon(
        n: libc::c_int,
        s: quoting_style,
        arg: *const libc::c_char,
    ) -> *mut libc::c_char;
    fn quotearg_style(s: quoting_style, arg: *const libc::c_char) -> *mut libc::c_char;
    fn quotearg_n_style_mem(
        n: libc::c_int,
        s: quoting_style,
        arg: *const libc::c_char,
        argsize: size_t,
    ) -> *mut libc::c_char;
    fn quotearg_n_style(
        n: libc::c_int,
        s: quoting_style,
        arg: *const libc::c_char,
    ) -> *mut libc::c_char;
    fn set_program_name(argv0: *const libc::c_char);
    static mut program_name: *const libc::c_char;
    fn version_etc(
        stream: *mut FILE,
        command_name: *const libc::c_char,
        package: *const libc::c_char,
        version: *const libc::c_char,
        _: ...
    );
    fn close_stdout();
    static mut Version: *const libc::c_char;
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
    fn exit(_: libc::c_int) -> !;
    fn atexit(__func: Option::<unsafe extern "C" fn() -> ()>) -> libc::c_int;
    fn error(
        __status: libc::c_int,
        __errnum: libc::c_int,
        __format: *const libc::c_char,
        _: ...
    );
    fn quote(arg: *const libc::c_char) -> *const libc::c_char;
}
pub type size_t = libc::c_ulong;
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
pub struct __mbstate_t {
    pub __count: libc::c_int,
    pub __value: C2RustUnnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub __wch: libc::c_uint,
    pub __wchb: [libc::c_char; 4],
}
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
pub type mbstate_t = __mbstate_t;
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
pub type C2RustUnnamed_0 = libc::c_uint;
pub const _PC_2_SYMLINKS: C2RustUnnamed_0 = 20;
pub const _PC_SYMLINK_MAX: C2RustUnnamed_0 = 19;
pub const _PC_ALLOC_SIZE_MIN: C2RustUnnamed_0 = 18;
pub const _PC_REC_XFER_ALIGN: C2RustUnnamed_0 = 17;
pub const _PC_REC_MIN_XFER_SIZE: C2RustUnnamed_0 = 16;
pub const _PC_REC_MAX_XFER_SIZE: C2RustUnnamed_0 = 15;
pub const _PC_REC_INCR_XFER_SIZE: C2RustUnnamed_0 = 14;
pub const _PC_FILESIZEBITS: C2RustUnnamed_0 = 13;
pub const _PC_SOCK_MAXBUF: C2RustUnnamed_0 = 12;
pub const _PC_PRIO_IO: C2RustUnnamed_0 = 11;
pub const _PC_ASYNC_IO: C2RustUnnamed_0 = 10;
pub const _PC_SYNC_IO: C2RustUnnamed_0 = 9;
pub const _PC_VDISABLE: C2RustUnnamed_0 = 8;
pub const _PC_NO_TRUNC: C2RustUnnamed_0 = 7;
pub const _PC_CHOWN_RESTRICTED: C2RustUnnamed_0 = 6;
pub const _PC_PIPE_BUF: C2RustUnnamed_0 = 5;
pub const _PC_PATH_MAX: C2RustUnnamed_0 = 4;
pub const _PC_NAME_MAX: C2RustUnnamed_0 = 3;
pub const _PC_MAX_INPUT: C2RustUnnamed_0 = 2;
pub const _PC_MAX_CANON: C2RustUnnamed_0 = 1;
pub const _PC_LINK_MAX: C2RustUnnamed_0 = 0;
pub type C2RustUnnamed_1 = libc::c_int;
pub const GETOPT_VERSION_CHAR: C2RustUnnamed_1 = -3;
pub const GETOPT_HELP_CHAR: C2RustUnnamed_1 = -2;
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
pub type C2RustUnnamed_2 = libc::c_uint;
pub const PORTABILITY_OPTION: C2RustUnnamed_2 = 256;
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
static mut longopts: [option; 4] = [
    {
        let mut init = option {
            name: b"portability\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: PORTABILITY_OPTION as libc::c_int,
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
                b"Usage: %s [OPTION]... NAME...\n\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            program_name,
        );
        fputs_unlocked(
            dcgettext(
                0 as *const libc::c_char,
                b"Diagnose invalid or unportable file names.\n\n  -p                  check for most POSIX systems\n  -P                  check for empty names and leading \"-\"\n      --portability   check for all POSIX systems (equivalent to -p -P)\n\0"
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
        emit_ancillary_info(b"pathchk\0" as *const u8 as *const libc::c_char);
    }
    exit(status);
}
unsafe fn main_0(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut ok: bool = 1 as libc::c_int != 0;
    let mut check_basic_portability: bool = 0 as libc::c_int != 0;
    let mut check_extra_portability: bool = 0 as libc::c_int != 0;
    let mut optc: libc::c_int = 0;
    set_program_name(*argv.offset(0 as libc::c_int as isize));
    setlocale(6 as libc::c_int, b"\0" as *const u8 as *const libc::c_char);
    bindtextdomain(
        b"coreutils\0" as *const u8 as *const libc::c_char,
        b"/usr/local/share/locale\0" as *const u8 as *const libc::c_char,
    );
    textdomain(b"coreutils\0" as *const u8 as *const libc::c_char);
    atexit(Some(close_stdout as unsafe extern "C" fn() -> ()));
    loop {
        optc = getopt_long(
            argc,
            argv,
            b"+pP\0" as *const u8 as *const libc::c_char,
            longopts.as_ptr(),
            0 as *mut libc::c_int,
        );
        if !(optc != -(1 as libc::c_int)) {
            break;
        }
        match optc {
            256 => {
                check_basic_portability = 1 as libc::c_int != 0;
                check_extra_portability = 1 as libc::c_int != 0;
            }
            112 => {
                check_basic_portability = 1 as libc::c_int != 0;
            }
            80 => {
                check_extra_portability = 1 as libc::c_int != 0;
            }
            -2 => {
                usage(0 as libc::c_int);
            }
            -3 => {
                version_etc(
                    stdout,
                    b"pathchk\0" as *const u8 as *const libc::c_char,
                    b"GNU coreutils\0" as *const u8 as *const libc::c_char,
                    Version,
                    b"Paul Eggert\0" as *const u8 as *const libc::c_char,
                    b"David MacKenzie\0" as *const u8 as *const libc::c_char,
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
    if optind == argc {
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
    while optind < argc {
        ok = (ok as libc::c_int
            & validate_file_name(
                *argv.offset(optind as isize),
                check_basic_portability,
                check_extra_portability,
            ) as libc::c_int) as bool;
        optind += 1;
    }
    return if ok as libc::c_int != 0 { 0 as libc::c_int } else { 1 as libc::c_int };
}
unsafe extern "C" fn no_leading_hyphen(mut file: *const libc::c_char) -> bool {
    let mut p: *const libc::c_char = 0 as *const libc::c_char;
    p = file;
    loop {
        p = strchr(p, '-' as i32);
        if p.is_null() {
            break;
        }
        if p == file
            || *p.offset(-(1 as libc::c_int) as isize) as libc::c_int == '/' as i32
        {
            error(
                0 as libc::c_int,
                0 as libc::c_int,
                dcgettext(
                    0 as *const libc::c_char,
                    b"leading '-' in a component of file name %s\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
                quotearg_style(shell_escape_always_quoting_style, file),
            );
            return 0 as libc::c_int != 0;
        }
        p = p.offset(1);
    }
    return 1 as libc::c_int != 0;
}
unsafe extern "C" fn portable_chars_only(
    mut file: *const libc::c_char,
    mut filelen: size_t,
) -> bool {
    let mut validlen: size_t = strspn(
        file,
        b"/ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789._-\0"
            as *const u8 as *const libc::c_char,
    );
    let mut invalid: *const libc::c_char = file.offset(validlen as isize);
    if *invalid != 0 {
        let mut mbstate: mbstate_t = {
            let mut init = __mbstate_t {
                __count: 0 as libc::c_int,
                __value: C2RustUnnamed { __wch: 0 },
            };
            init
        };
        let mut charlen: size_t = rpl_mbrlen(
            invalid,
            filelen.wrapping_sub(validlen),
            &mut mbstate,
        );
        error(
            0 as libc::c_int,
            0 as libc::c_int,
            dcgettext(
                0 as *const libc::c_char,
                b"nonportable character %s in file name %s\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
            quotearg_n_style_mem(
                1 as libc::c_int,
                locale_quoting_style,
                invalid,
                if charlen <= 16 as libc::c_int as libc::c_ulong {
                    charlen
                } else {
                    1 as libc::c_int as libc::c_ulong
                },
            ),
            quotearg_n_style(0 as libc::c_int, shell_escape_always_quoting_style, file),
        );
        return 0 as libc::c_int != 0;
    }
    return 1 as libc::c_int != 0;
}
unsafe extern "C" fn component_start(mut f: *mut libc::c_char) -> *mut libc::c_char {
    while *f as libc::c_int == '/' as i32 {
        f = f.offset(1);
    }
    return f;
}
unsafe extern "C" fn component_len(mut f: *const libc::c_char) -> size_t {
    let mut len: size_t = 0;
    len = 1 as libc::c_int as size_t;
    while *f.offset(len as isize) as libc::c_int != '/' as i32
        && *f.offset(len as isize) as libc::c_int != 0
    {
        len = len.wrapping_add(1);
    }
    return len;
}
unsafe extern "C" fn validate_file_name(
    mut file: *mut libc::c_char,
    mut check_basic_portability: bool,
    mut check_extra_portability: bool,
) -> bool {
    let mut filelen: size_t = strlen(file);
    let mut start: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut check_component_lengths: bool = false;
    let mut file_exists: bool = 0 as libc::c_int != 0;
    if check_extra_portability as libc::c_int != 0 && !no_leading_hyphen(file) {
        return 0 as libc::c_int != 0;
    }
    if (check_basic_portability as libc::c_int != 0
        || check_extra_portability as libc::c_int != 0)
        && filelen == 0 as libc::c_int as libc::c_ulong
    {
        error(
            0 as libc::c_int,
            0 as libc::c_int,
            dcgettext(
                0 as *const libc::c_char,
                b"empty file name\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
        return 0 as libc::c_int != 0;
    }
    if check_basic_portability {
        if !portable_chars_only(file, filelen) {
            return 0 as libc::c_int != 0;
        }
    } else {
        let mut st: stat = stat {
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
        if lstat(file, &mut st) == 0 as libc::c_int {
            file_exists = 1 as libc::c_int != 0;
        } else if *__errno_location() != 2 as libc::c_int
            || filelen == 0 as libc::c_int as libc::c_ulong
        {
            error(
                0 as libc::c_int,
                *__errno_location(),
                b"%s\0" as *const u8 as *const libc::c_char,
                quotearg_n_style_colon(
                    0 as libc::c_int,
                    shell_escape_quoting_style,
                    file,
                ),
            );
            return 0 as libc::c_int != 0;
        }
    }
    if check_basic_portability as libc::c_int != 0
        || !file_exists && 256 as libc::c_int as libc::c_ulong <= filelen
    {
        let mut maxsize: size_t = 0;
        if check_basic_portability {
            maxsize = 256 as libc::c_int as size_t;
        } else {
            let mut size: libc::c_long = 0;
            let mut dir: *const libc::c_char = if *file as libc::c_int == '/' as i32 {
                b"/\0" as *const u8 as *const libc::c_char
            } else {
                b".\0" as *const u8 as *const libc::c_char
            };
            *__errno_location() = 0 as libc::c_int;
            size = pathconf(dir, _PC_PATH_MAX as libc::c_int);
            if size < 0 as libc::c_int as libc::c_long
                && *__errno_location() != 0 as libc::c_int
            {
                error(
                    0 as libc::c_int,
                    *__errno_location(),
                    dcgettext(
                        0 as *const libc::c_char,
                        b"%s: unable to determine maximum file name length\0"
                            as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    dir,
                );
                return 0 as libc::c_int != 0;
            }
            maxsize = (if size < 9223372036854775807 as libc::c_long {
                size
            } else {
                9223372036854775807 as libc::c_long
            }) as size_t;
        }
        if maxsize <= filelen {
            let mut len: libc::c_ulong = filelen;
            let mut maxlen: libc::c_ulong = maxsize
                .wrapping_sub(1 as libc::c_int as libc::c_ulong);
            error(
                0 as libc::c_int,
                0 as libc::c_int,
                dcgettext(
                    0 as *const libc::c_char,
                    b"limit %lu exceeded by length %lu of file name %s\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
                maxlen,
                len,
                quotearg_style(shell_escape_always_quoting_style, file),
            );
            return 0 as libc::c_int != 0;
        }
    }
    check_component_lengths = check_basic_portability;
    if !check_component_lengths && !file_exists {
        start = file;
        loop {
            start = component_start(start);
            if !(*start != 0) {
                break;
            }
            let mut length: size_t = component_len(start);
            if (14 as libc::c_int as libc::c_ulong) < length {
                check_component_lengths = 1 as libc::c_int != 0;
                break;
            } else {
                start = start.offset(length as isize);
            }
        }
    }
    if check_component_lengths {
        let mut name_max: size_t = 14 as libc::c_int as size_t;
        let mut known_name_max: size_t = (if check_basic_portability as libc::c_int != 0
        {
            14 as libc::c_int
        } else {
            0 as libc::c_int
        }) as size_t;
        start = file;
        loop {
            start = component_start(start);
            if !(*start != 0) {
                break;
            }
            let mut length_0: size_t = 0;
            if known_name_max != 0 {
                name_max = known_name_max;
            } else {
                let mut len_0: libc::c_long = 0;
                let mut dir_0: *const libc::c_char = if start == file {
                    b".\0" as *const u8 as *const libc::c_char
                } else {
                    file as *const libc::c_char
                };
                let mut c: libc::c_char = *start;
                *__errno_location() = 0 as libc::c_int;
                *start = '\0' as i32 as libc::c_char;
                len_0 = pathconf(dir_0, _PC_NAME_MAX as libc::c_int);
                *start = c;
                if 0 as libc::c_int as libc::c_long <= len_0 {
                    name_max = (if len_0 < 9223372036854775807 as libc::c_long {
                        len_0
                    } else {
                        9223372036854775807 as libc::c_long
                    }) as size_t;
                } else {
                    match *__errno_location() {
                        0 => {
                            name_max = 18446744073709551615 as libc::c_ulong;
                        }
                        2 => {
                            known_name_max = name_max;
                        }
                        _ => {
                            *start = '\0' as i32 as libc::c_char;
                            error(
                                0 as libc::c_int,
                                *__errno_location(),
                                b"%s\0" as *const u8 as *const libc::c_char,
                                quotearg_n_style_colon(
                                    0 as libc::c_int,
                                    shell_escape_quoting_style,
                                    dir_0,
                                ),
                            );
                            *start = c;
                            return 0 as libc::c_int != 0;
                        }
                    }
                }
            }
            length_0 = component_len(start);
            if name_max < length_0 {
                let mut len_1: libc::c_ulong = length_0;
                let mut maxlen_0: libc::c_ulong = name_max;
                let mut c_0: libc::c_char = *start.offset(len_1 as isize);
                *start.offset(len_1 as isize) = '\0' as i32 as libc::c_char;
                error(
                    0 as libc::c_int,
                    0 as libc::c_int,
                    dcgettext(
                        0 as *const libc::c_char,
                        b"limit %lu exceeded by length %lu of file name component %s\0"
                            as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    maxlen_0,
                    len_1,
                    quote(start),
                );
                *start.offset(len_1 as isize) = c_0;
                return 0 as libc::c_int != 0;
            }
            start = start.offset(length_0 as isize);
        }
    }
    return 1 as libc::c_int != 0;
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
