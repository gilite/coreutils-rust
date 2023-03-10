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
    static mut optarg: *mut libc::c_char;
    fn execvp(
        __file: *const libc::c_char,
        __argv: *const *mut libc::c_char,
    ) -> libc::c_int;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strncmp(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn __errno_location() -> *mut libc::c_int;
    static mut Version: *const libc::c_char;
    static mut exit_failure: libc::c_int;
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
    fn atexit(__func: Option::<unsafe extern "C" fn() -> ()>) -> libc::c_int;
    fn exit(_: libc::c_int) -> !;
    fn getpriority(__which: __priority_which_t, __who: id_t) -> libc::c_int;
    fn setpriority(
        __which: __priority_which_t,
        __who: id_t,
        __prio: libc::c_int,
    ) -> libc::c_int;
    fn error(
        __status: libc::c_int,
        __errnum: libc::c_int,
        __format: *const libc::c_char,
        _: ...
    );
    fn quote(arg: *const libc::c_char) -> *const libc::c_char;
    fn xstrtol(
        _: *const libc::c_char,
        _: *mut *mut libc::c_char,
        _: libc::c_int,
        _: *mut libc::c_long,
        _: *const libc::c_char,
    ) -> strtol_error;
}
pub type size_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __id_t = libc::c_uint;
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
pub type id_t = __id_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct option {
    pub name: *const libc::c_char,
    pub has_arg: libc::c_int,
    pub flag: *mut libc::c_int,
    pub val: libc::c_int,
}
pub type C2RustUnnamed = libc::c_uint;
pub const EXIT_ENOENT: C2RustUnnamed = 127;
pub const EXIT_CANNOT_INVOKE: C2RustUnnamed = 126;
pub const EXIT_CANCELED: C2RustUnnamed = 125;
pub const EXIT_TIMEDOUT: C2RustUnnamed = 124;
pub type C2RustUnnamed_0 = libc::c_int;
pub const GETOPT_VERSION_CHAR: C2RustUnnamed_0 = -3;
pub const GETOPT_HELP_CHAR: C2RustUnnamed_0 = -2;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct infomap {
    pub program: *const libc::c_char,
    pub node: *const libc::c_char,
}
pub type __priority_which = libc::c_uint;
pub const PRIO_USER: __priority_which = 2;
pub const PRIO_PGRP: __priority_which = 1;
pub const PRIO_PROCESS: __priority_which = 0;
pub type __priority_which_t = __priority_which;
pub type strtol_error = libc::c_uint;
pub const LONGINT_INVALID: strtol_error = 4;
pub const LONGINT_INVALID_SUFFIX_CHAR_WITH_OVERFLOW: strtol_error = 3;
pub const LONGINT_INVALID_SUFFIX_CHAR: strtol_error = 2;
pub const LONGINT_OVERFLOW: strtol_error = 1;
pub const LONGINT_OK: strtol_error = 0;
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
pub const MAX_ADJUSTMENT: C2RustUnnamed_4 = 39;
pub const MIN_ADJUSTMENT: C2RustUnnamed_4 = -39;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_3 {
    pub _gl_dummy: libc::c_int,
}
pub type C2RustUnnamed_4 = libc::c_int;
#[inline]
unsafe extern "C" fn ferror_unlocked(mut __stream: *mut FILE) -> libc::c_int {
    return ((*__stream)._flags & 0x20 as libc::c_int != 0 as libc::c_int) as libc::c_int;
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
unsafe extern "C" fn initialize_exit_failure(mut status: libc::c_int) {
    if status != 1 as libc::c_int {
        ::core::ptr::write_volatile(&mut exit_failure as *mut libc::c_int, status);
    }
}
static mut longopts: [option; 4] = [
    {
        let mut init = option {
            name: b"adjustment\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'n' as i32,
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
                b"Usage: %s [OPTION] [COMMAND [ARG]...]\n\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
            program_name,
        );
        printf(
            dcgettext(
                0 as *const libc::c_char,
                b"Run COMMAND with an adjusted niceness, which affects process scheduling.\nWith no COMMAND, print the current niceness.  Niceness values range from\n%d (most favorable to the process) to %d (least favorable to the process).\n\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            -(20 as libc::c_int),
            20 as libc::c_int - 1 as libc::c_int,
        );
        emit_mandatory_arg_note();
        fputs_unlocked(
            dcgettext(
                0 as *const libc::c_char,
                b"  -n, --adjustment=N   add integer N to the niceness (default 10)\n\0"
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
        printf(
            dcgettext(
                0 as *const libc::c_char,
                b"\nNOTE: your shell may have its own version of %s, which usually supersedes\nthe version described here.  Please refer to your shell's documentation\nfor details about the options it supports.\n\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            b"nice\0" as *const u8 as *const libc::c_char,
        );
        emit_exec_status(b"nice\0" as *const u8 as *const libc::c_char);
        emit_ancillary_info(b"nice\0" as *const u8 as *const libc::c_char);
    }
    exit(status);
}
unsafe extern "C" fn perm_related_errno(mut err: libc::c_int) -> bool {
    return err == 13 as libc::c_int || err == 1 as libc::c_int;
}
unsafe fn main_0(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut current_niceness: libc::c_int = 0;
    let mut adjustment: libc::c_int = 10 as libc::c_int;
    let mut adjustment_given: *const libc::c_char = 0 as *const libc::c_char;
    let mut ok: bool = false;
    let mut i: libc::c_int = 0;
    set_program_name(*argv.offset(0 as libc::c_int as isize));
    setlocale(6 as libc::c_int, b"\0" as *const u8 as *const libc::c_char);
    bindtextdomain(
        b"coreutils\0" as *const u8 as *const libc::c_char,
        b"/usr/local/share/locale\0" as *const u8 as *const libc::c_char,
    );
    textdomain(b"coreutils\0" as *const u8 as *const libc::c_char);
    initialize_exit_failure(EXIT_CANCELED as libc::c_int);
    atexit(Some(close_stdout as unsafe extern "C" fn() -> ()));
    i = 1 as libc::c_int;
    while i < argc {
        let mut s: *const libc::c_char = *argv.offset(i as isize);
        if *s.offset(0 as libc::c_int as isize) as libc::c_int == '-' as i32
            && (*s
                .offset(
                    (1 as libc::c_int
                        + (*s.offset(1 as libc::c_int as isize) as libc::c_int
                            == '-' as i32
                            || *s.offset(1 as libc::c_int as isize) as libc::c_int
                                == '+' as i32) as libc::c_int) as isize,
                ) as libc::c_uint)
                .wrapping_sub('0' as i32 as libc::c_uint)
                <= 9 as libc::c_int as libc::c_uint
        {
            adjustment_given = s.offset(1 as libc::c_int as isize);
            i += 1;
        } else {
            let mut c: libc::c_int = 0;
            let mut fake_argc: libc::c_int = argc - (i - 1 as libc::c_int);
            let mut fake_argv: *mut *mut libc::c_char = argv
                .offset((i - 1 as libc::c_int) as isize);
            let ref mut fresh0 = *fake_argv.offset(0 as libc::c_int as isize);
            *fresh0 = *argv.offset(0 as libc::c_int as isize);
            optind = 0 as libc::c_int;
            c = getopt_long(
                fake_argc,
                fake_argv,
                b"+n:\0" as *const u8 as *const libc::c_char,
                longopts.as_ptr(),
                0 as *mut libc::c_int,
            );
            i += optind - 1 as libc::c_int;
            match c {
                110 => {
                    adjustment_given = optarg;
                }
                -1 => {}
                -2 => {
                    usage(0 as libc::c_int);
                }
                -3 => {
                    version_etc(
                        stdout,
                        b"nice\0" as *const u8 as *const libc::c_char,
                        b"GNU coreutils\0" as *const u8 as *const libc::c_char,
                        Version,
                        b"David MacKenzie\0" as *const u8 as *const libc::c_char,
                        0 as *mut libc::c_void as *mut libc::c_char,
                    );
                    exit(0 as libc::c_int);
                }
                _ => {
                    usage(EXIT_CANCELED as libc::c_int);
                }
            }
            if c == -(1 as libc::c_int) {
                break;
            }
        }
    }
    if !adjustment_given.is_null() {
        let mut tmp: libc::c_long = 0;
        if (LONGINT_OVERFLOW as libc::c_int as libc::c_uint)
            < xstrtol(
                adjustment_given,
                0 as *mut *mut libc::c_char,
                10 as libc::c_int,
                &mut tmp,
                b"\0" as *const u8 as *const libc::c_char,
            ) as libc::c_uint
        {
            if ::core::mem::size_of::<C2RustUnnamed_3>() as libc::c_ulong != 0 {
                error(
                    EXIT_CANCELED as libc::c_int,
                    0 as libc::c_int,
                    dcgettext(
                        0 as *const libc::c_char,
                        b"invalid adjustment %s\0" as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    quote(adjustment_given),
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
                        b"invalid adjustment %s\0" as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    quote(adjustment_given),
                );
                if 0 as libc::c_int != 0 {} else {
                    unreachable!();
                };
            };
        }
        adjustment = (if MIN_ADJUSTMENT as libc::c_int as libc::c_long
            > (if tmp < MAX_ADJUSTMENT as libc::c_int as libc::c_long {
                tmp
            } else {
                MAX_ADJUSTMENT as libc::c_int as libc::c_long
            })
        {
            MIN_ADJUSTMENT as libc::c_int as libc::c_long
        } else if tmp < MAX_ADJUSTMENT as libc::c_int as libc::c_long {
            tmp
        } else {
            MAX_ADJUSTMENT as libc::c_int as libc::c_long
        }) as libc::c_int;
    }
    if i == argc {
        if !adjustment_given.is_null() {
            error(
                0 as libc::c_int,
                0 as libc::c_int,
                dcgettext(
                    0 as *const libc::c_char,
                    b"a command must be given with an adjustment\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
            );
            usage(EXIT_CANCELED as libc::c_int);
        }
        *__errno_location() = 0 as libc::c_int;
        current_niceness = getpriority(PRIO_PROCESS, 0 as libc::c_int as id_t);
        if current_niceness == -(1 as libc::c_int)
            && *__errno_location() != 0 as libc::c_int
        {
            if ::core::mem::size_of::<C2RustUnnamed_2>() as libc::c_ulong != 0 {
                error(
                    EXIT_CANCELED as libc::c_int,
                    *__errno_location(),
                    dcgettext(
                        0 as *const libc::c_char,
                        b"cannot get niceness\0" as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    ),
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
                        b"cannot get niceness\0" as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                );
                if 0 as libc::c_int != 0 {} else {
                    unreachable!();
                };
            };
        }
        printf(b"%d\n\0" as *const u8 as *const libc::c_char, current_niceness);
        return 0 as libc::c_int;
    }
    *__errno_location() = 0 as libc::c_int;
    current_niceness = getpriority(PRIO_PROCESS, 0 as libc::c_int as id_t);
    if current_niceness == -(1 as libc::c_int) && *__errno_location() != 0 as libc::c_int
    {
        if ::core::mem::size_of::<C2RustUnnamed_1>() as libc::c_ulong != 0 {
            error(
                EXIT_CANCELED as libc::c_int,
                *__errno_location(),
                dcgettext(
                    0 as *const libc::c_char,
                    b"cannot get niceness\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
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
                    b"cannot get niceness\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
            );
            if 0 as libc::c_int != 0 {} else {
                unreachable!();
            };
        };
    }
    ok = setpriority(
        PRIO_PROCESS,
        0 as libc::c_int as id_t,
        current_niceness + adjustment,
    ) == 0 as libc::c_int;
    if !ok {
        error(
            if perm_related_errno(*__errno_location()) as libc::c_int != 0 {
                0 as libc::c_int
            } else {
                EXIT_CANCELED as libc::c_int
            },
            *__errno_location(),
            dcgettext(
                0 as *const libc::c_char,
                b"cannot set niceness\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
        if ferror_unlocked(stderr) != 0 {
            return EXIT_CANCELED as libc::c_int;
        }
    }
    execvp(
        *argv.offset(i as isize),
        &mut *argv.offset(i as isize) as *mut *mut libc::c_char
            as *const *mut libc::c_char,
    );
    let mut exit_status: libc::c_int = if *__errno_location() == 2 as libc::c_int {
        EXIT_ENOENT as libc::c_int
    } else {
        EXIT_CANNOT_INVOKE as libc::c_int
    };
    error(
        0 as libc::c_int,
        *__errno_location(),
        b"%s\0" as *const u8 as *const libc::c_char,
        quote(*argv.offset(i as isize)),
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
