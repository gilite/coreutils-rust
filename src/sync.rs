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
    static mut optind: libc::c_int;
    fn getopt_long(
        ___argc: libc::c_int,
        ___argv: *const *mut libc::c_char,
        __shortopts: *const libc::c_char,
        __longopts: *const option,
        __longind: *mut libc::c_int,
    ) -> libc::c_int;
    static mut stdout: *mut FILE;
    static mut stderr: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn fputs_unlocked(__s: *const libc::c_char, __stream: *mut FILE) -> libc::c_int;
    fn close(__fd: libc::c_int) -> libc::c_int;
    fn fsync(__fd: libc::c_int) -> libc::c_int;
    fn syncfs(__fd: libc::c_int) -> libc::c_int;
    fn sync();
    fn fdatasync(__fildes: libc::c_int) -> libc::c_int;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strncmp(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn __errno_location() -> *mut libc::c_int;
    fn quotearg_style(s: quoting_style, arg: *const libc::c_char) -> *mut libc::c_char;
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
    fn open(__file: *const libc::c_char, __oflag: libc::c_int, _: ...) -> libc::c_int;
    fn rpl_fcntl(fd: libc::c_int, action: libc::c_int, _: ...) -> libc::c_int;
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
    fn atexit(__func: Option::<unsafe extern "C" fn() -> ()>) -> libc::c_int;
    fn exit(_: libc::c_int) -> !;
    fn error(
        __status: libc::c_int,
        __errnum: libc::c_int,
        __format: *const libc::c_char,
        _: ...
    );
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
pub type C2RustUnnamed = libc::c_int;
pub const GETOPT_VERSION_CHAR: C2RustUnnamed = -3;
pub const GETOPT_HELP_CHAR: C2RustUnnamed = -2;
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
pub type sync_mode = libc::c_uint;
pub const MODE_SYNC: sync_mode = 3;
pub const MODE_FILE_SYSTEM: sync_mode = 2;
pub const MODE_DATA: sync_mode = 1;
pub const MODE_FILE: sync_mode = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_0 {
    pub _gl_dummy: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_1 {
    pub _gl_dummy: libc::c_int,
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
static mut long_options: [option; 5] = [
    {
        let mut init = option {
            name: b"data\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'd' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"file-system\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'f' as i32,
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
                b"Usage: %s [OPTION] [FILE]...\n\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            program_name,
        );
        fputs_unlocked(
            dcgettext(
                0 as *const libc::c_char,
                b"Synchronize cached writes to persistent storage\n\nIf one or more files are specified, sync only them,\nor their containing file systems.\n\n\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            stdout,
        );
        fputs_unlocked(
            dcgettext(
                0 as *const libc::c_char,
                b"  -d, --data             sync only file data, no unneeded metadata\n\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            stdout,
        );
        fputs_unlocked(
            dcgettext(
                0 as *const libc::c_char,
                b"  -f, --file-system      sync the file systems that contain the files\n\0"
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
        emit_ancillary_info(b"sync\0" as *const u8 as *const libc::c_char);
    }
    exit(status);
}
unsafe extern "C" fn sync_arg(
    mut mode: sync_mode,
    mut file: *const libc::c_char,
) -> bool {
    let mut ret: bool = 1 as libc::c_int != 0;
    let mut open_flags: libc::c_int = 0 as libc::c_int | 0o4000 as libc::c_int;
    let mut fd: libc::c_int = 0;
    fd = open(file, open_flags);
    if fd < 0 as libc::c_int {
        let mut rd_errno: libc::c_int = *__errno_location();
        if open_flags != 0o1 as libc::c_int | 0o4000 as libc::c_int {
            fd = open(file, 0o1 as libc::c_int | 0o4000 as libc::c_int);
        }
        if fd < 0 as libc::c_int {
            error(
                0 as libc::c_int,
                rd_errno,
                dcgettext(
                    0 as *const libc::c_char,
                    b"error opening %s\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
                quotearg_style(shell_escape_always_quoting_style, file),
            );
            return 0 as libc::c_int != 0;
        }
    }
    let mut fdflags: libc::c_int = rpl_fcntl(fd, 3 as libc::c_int);
    if fdflags == -(1 as libc::c_int)
        || rpl_fcntl(fd, 4 as libc::c_int, fdflags & !(0o4000 as libc::c_int))
            < 0 as libc::c_int
    {
        error(
            0 as libc::c_int,
            *__errno_location(),
            dcgettext(
                0 as *const libc::c_char,
                b"couldn't reset non-blocking mode %s\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
            quotearg_style(shell_escape_always_quoting_style, file),
        );
        ret = 0 as libc::c_int != 0;
    }
    if ret as libc::c_int == 1 as libc::c_int {
        let mut sync_status: libc::c_int = -(1 as libc::c_int);
        match mode as libc::c_uint {
            1 => {
                sync_status = fdatasync(fd);
            }
            0 => {
                sync_status = fsync(fd);
            }
            2 => {
                sync_status = syncfs(fd);
            }
            _ => {
                if !(b"invalid sync_mode\0" as *const u8 as *const libc::c_char)
                    .is_null()
                {} else {
                    __assert_fail(
                        b"\"invalid sync_mode\"\0" as *const u8 as *const libc::c_char,
                        b"src/sync.c\0" as *const u8 as *const libc::c_char,
                        152 as libc::c_int as libc::c_uint,
                        (*::core::mem::transmute::<
                            &[u8; 45],
                            &[libc::c_char; 45],
                        >(b"_Bool sync_arg(enum sync_mode, const char *)\0"))
                            .as_ptr(),
                    );
                }
            }
        }
        if sync_status < 0 as libc::c_int {
            error(
                0 as libc::c_int,
                *__errno_location(),
                dcgettext(
                    0 as *const libc::c_char,
                    b"error syncing %s\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
                quotearg_style(shell_escape_always_quoting_style, file),
            );
            ret = 0 as libc::c_int != 0;
        }
    }
    if close(fd) < 0 as libc::c_int {
        error(
            0 as libc::c_int,
            *__errno_location(),
            dcgettext(
                0 as *const libc::c_char,
                b"failed to close %s\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            quotearg_style(shell_escape_always_quoting_style, file),
        );
        ret = 0 as libc::c_int != 0;
    }
    return ret;
}
unsafe fn main_0(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut c: libc::c_int = 0;
    let mut args_specified: bool = false;
    let mut arg_data: bool = 0 as libc::c_int != 0;
    let mut arg_file_system: bool = 0 as libc::c_int != 0;
    let mut mode: sync_mode = MODE_FILE;
    let mut ok: bool = 1 as libc::c_int != 0;
    set_program_name(*argv.offset(0 as libc::c_int as isize));
    setlocale(6 as libc::c_int, b"\0" as *const u8 as *const libc::c_char);
    bindtextdomain(
        b"coreutils\0" as *const u8 as *const libc::c_char,
        b"/usr/local/share/locale\0" as *const u8 as *const libc::c_char,
    );
    textdomain(b"coreutils\0" as *const u8 as *const libc::c_char);
    atexit(Some(close_stdout as unsafe extern "C" fn() -> ()));
    loop {
        c = getopt_long(
            argc,
            argv,
            b"df\0" as *const u8 as *const libc::c_char,
            long_options.as_ptr(),
            0 as *mut libc::c_int,
        );
        if !(c != -(1 as libc::c_int)) {
            break;
        }
        match c {
            100 => {
                arg_data = 1 as libc::c_int != 0;
            }
            102 => {
                arg_file_system = 1 as libc::c_int != 0;
            }
            -2 => {
                usage(0 as libc::c_int);
            }
            -3 => {
                version_etc(
                    stdout,
                    b"sync\0" as *const u8 as *const libc::c_char,
                    b"GNU coreutils\0" as *const u8 as *const libc::c_char,
                    Version,
                    b"Jim Meyering\0" as *const u8 as *const libc::c_char,
                    b"Giuseppe Scrivano\0" as *const u8 as *const libc::c_char,
                    0 as *mut libc::c_void as *mut libc::c_char,
                );
                exit(0 as libc::c_int);
            }
            _ => {
                usage(1 as libc::c_int);
            }
        }
    }
    args_specified = optind < argc;
    if arg_data as libc::c_int != 0 && arg_file_system as libc::c_int != 0 {
        if ::core::mem::size_of::<C2RustUnnamed_1>() as libc::c_ulong != 0 {
            error(
                1 as libc::c_int,
                0 as libc::c_int,
                dcgettext(
                    0 as *const libc::c_char,
                    b"cannot specify both --data and --file-system\0" as *const u8
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
                    b"cannot specify both --data and --file-system\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
            );
            if 0 as libc::c_int != 0 {} else {
                unreachable!();
            };
        };
    }
    if !args_specified && arg_data as libc::c_int != 0 {
        if ::core::mem::size_of::<C2RustUnnamed_0>() as libc::c_ulong != 0 {
            error(
                1 as libc::c_int,
                0 as libc::c_int,
                dcgettext(
                    0 as *const libc::c_char,
                    b"--data needs at least one argument\0" as *const u8
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
                    b"--data needs at least one argument\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
            );
            if 0 as libc::c_int != 0 {} else {
                unreachable!();
            };
        };
    }
    if !args_specified || arg_file_system as libc::c_int != 0 && 1 as libc::c_int == 0 {
        mode = MODE_SYNC;
    } else if arg_file_system {
        mode = MODE_FILE_SYSTEM;
    } else if !arg_data {
        mode = MODE_FILE;
    } else {
        mode = MODE_DATA;
    }
    if mode as libc::c_uint == MODE_SYNC as libc::c_int as libc::c_uint {
        sync();
    } else {
        while optind < argc {
            ok = (ok as libc::c_int
                & sync_arg(mode, *argv.offset(optind as isize)) as libc::c_int) as bool;
            optind += 1;
        }
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
