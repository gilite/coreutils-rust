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
    fn __overflow(_: *mut FILE, _: libc::c_int) -> libc::c_int;
    fn getopt_long(
        ___argc: libc::c_int,
        ___argv: *const *mut libc::c_char,
        __shortopts: *const libc::c_char,
        __longopts: *const option,
        __longind: *mut libc::c_int,
    ) -> libc::c_int;
    static mut optarg: *mut libc::c_char;
    static mut optind: libc::c_int;
    fn __errno_location() -> *mut libc::c_int;
    fn execv(
        __path: *const libc::c_char,
        __argv: *const *mut libc::c_char,
    ) -> libc::c_int;
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
    fn atexit(__func: Option::<unsafe extern "C" fn() -> ()>) -> libc::c_int;
    fn exit(_: libc::c_int) -> !;
    fn error(
        __status: libc::c_int,
        __errnum: libc::c_int,
        __format: *const libc::c_char,
        _: ...
    );
    fn quote(arg: *const libc::c_char) -> *const libc::c_char;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct option {
    pub name: *const libc::c_char,
    pub has_arg: libc::c_int,
    pub flag: *mut libc::c_int,
    pub val: libc::c_int,
}
pub type security_class_t = libc::c_ushort;
pub type context_t = libc::c_int;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_6 {
    pub _gl_dummy: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_7 {
    pub _gl_dummy: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_8 {
    pub _gl_dummy: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_9 {
    pub _gl_dummy: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_10 {
    pub _gl_dummy: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_11 {
    pub _gl_dummy: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_12 {
    pub _gl_dummy: libc::c_int,
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
#[inline]
unsafe extern "C" fn fputc_unlocked(
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
unsafe extern "C" fn getcon(mut con: *mut *mut libc::c_char) -> libc::c_int {
    *__errno_location() = 95 as libc::c_int;
    return -(1 as libc::c_int);
}
#[inline]
unsafe extern "C" fn freecon(mut con: *mut libc::c_char) {}
#[inline]
unsafe extern "C" fn getfilecon(
    mut file: *const libc::c_char,
    mut con: *mut *mut libc::c_char,
) -> libc::c_int {
    *__errno_location() = 95 as libc::c_int;
    return -(1 as libc::c_int);
}
#[inline]
unsafe extern "C" fn security_check_context(
    mut con: *const libc::c_char,
) -> libc::c_int {
    *__errno_location() = 95 as libc::c_int;
    return -(1 as libc::c_int);
}
#[inline]
unsafe extern "C" fn setexeccon(mut con: *const libc::c_char) -> libc::c_int {
    *__errno_location() = 95 as libc::c_int;
    return -(1 as libc::c_int);
}
#[inline]
unsafe extern "C" fn security_compute_create(
    mut scon: *const libc::c_char,
    mut tcon: *const libc::c_char,
    mut tclass: security_class_t,
    mut newcon: *mut *mut libc::c_char,
) -> libc::c_int {
    *__errno_location() = 95 as libc::c_int;
    return -(1 as libc::c_int);
}
#[inline]
unsafe extern "C" fn string_to_security_class(
    mut name: *const libc::c_char,
) -> security_class_t {
    *__errno_location() = 95 as libc::c_int;
    return 0 as libc::c_int as security_class_t;
}
#[inline]
unsafe extern "C" fn context_new(mut s: *const libc::c_char) -> context_t {
    *__errno_location() = 95 as libc::c_int;
    return 0 as libc::c_int;
}
#[inline]
unsafe extern "C" fn context_str(mut con: context_t) -> *mut libc::c_char {
    *__errno_location() = 95 as libc::c_int;
    return 0 as *mut libc::c_char;
}
#[inline]
unsafe extern "C" fn context_user_set(
    mut sc: context_t,
    mut s: *const libc::c_char,
) -> libc::c_int {
    *__errno_location() = 95 as libc::c_int;
    return -(1 as libc::c_int);
}
#[inline]
unsafe extern "C" fn context_role_set(
    mut sc: context_t,
    mut s: *const libc::c_char,
) -> libc::c_int {
    *__errno_location() = 95 as libc::c_int;
    return -(1 as libc::c_int);
}
#[inline]
unsafe extern "C" fn context_range_set(
    mut sc: context_t,
    mut s: *const libc::c_char,
) -> libc::c_int {
    *__errno_location() = 95 as libc::c_int;
    return -(1 as libc::c_int);
}
#[inline]
unsafe extern "C" fn context_type_set(
    mut sc: context_t,
    mut s: *const libc::c_char,
) -> libc::c_int {
    *__errno_location() = 95 as libc::c_int;
    return -(1 as libc::c_int);
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
static mut long_options: [option; 8] = [
    {
        let mut init = option {
            name: b"role\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'r' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"type\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 't' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"user\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'u' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"range\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'l' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"compute\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'c' as i32,
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
                b"Usage: %s CONTEXT COMMAND [args]\n  or:  %s [ -c ] [-u USER] [-r ROLE] [-t TYPE] [-l RANGE] COMMAND [args]\n\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            program_name,
            program_name,
        );
        fputs_unlocked(
            dcgettext(
                0 as *const libc::c_char,
                b"Run a program in a different SELinux security context.\nWith neither CONTEXT nor COMMAND, print the current security context.\n\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            stdout,
        );
        emit_mandatory_arg_note();
        fputs_unlocked(
            dcgettext(
                0 as *const libc::c_char,
                b"  CONTEXT            Complete security context\n  -c, --compute      compute process transition context before modifying\n  -t, --type=TYPE    type (for same role as parent)\n  -u, --user=USER    user identity\n  -r, --role=ROLE    role\n  -l, --range=RANGE  levelrange\n\0"
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
        emit_exec_status(b"runcon\0" as *const u8 as *const libc::c_char);
        emit_ancillary_info(b"runcon\0" as *const u8 as *const libc::c_char);
    }
    exit(status);
}
unsafe fn main_0(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut role: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut range: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut user: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut type_0: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut context: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut cur_context: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut file_context: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut new_context: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut compute_trans: bool = 0 as libc::c_int != 0;
    let mut con: context_t = 0;
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
        let mut option_index: libc::c_int = 0 as libc::c_int;
        let mut c: libc::c_int = getopt_long(
            argc,
            argv,
            b"+r:t:u:l:c\0" as *const u8 as *const libc::c_char,
            long_options.as_ptr(),
            &mut option_index,
        );
        if c == -(1 as libc::c_int) {
            break;
        }
        match c {
            114 => {
                if !role.is_null() {
                    if ::core::mem::size_of::<C2RustUnnamed_17>() as libc::c_ulong != 0 {
                        error(
                            EXIT_CANCELED as libc::c_int,
                            0 as libc::c_int,
                            dcgettext(
                                0 as *const libc::c_char,
                                b"multiple roles\0" as *const u8 as *const libc::c_char,
                                5 as libc::c_int,
                            ),
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
                                b"multiple roles\0" as *const u8 as *const libc::c_char,
                                5 as libc::c_int,
                            ),
                        );
                        if 0 as libc::c_int != 0 {} else {
                            unreachable!();
                        };
                    };
                }
                role = optarg;
            }
            116 => {
                if !type_0.is_null() {
                    if ::core::mem::size_of::<C2RustUnnamed_16>() as libc::c_ulong != 0 {
                        error(
                            EXIT_CANCELED as libc::c_int,
                            0 as libc::c_int,
                            dcgettext(
                                0 as *const libc::c_char,
                                b"multiple types\0" as *const u8 as *const libc::c_char,
                                5 as libc::c_int,
                            ),
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
                                b"multiple types\0" as *const u8 as *const libc::c_char,
                                5 as libc::c_int,
                            ),
                        );
                        if 0 as libc::c_int != 0 {} else {
                            unreachable!();
                        };
                    };
                }
                type_0 = optarg;
            }
            117 => {
                if !user.is_null() {
                    if ::core::mem::size_of::<C2RustUnnamed_15>() as libc::c_ulong != 0 {
                        error(
                            EXIT_CANCELED as libc::c_int,
                            0 as libc::c_int,
                            dcgettext(
                                0 as *const libc::c_char,
                                b"multiple users\0" as *const u8 as *const libc::c_char,
                                5 as libc::c_int,
                            ),
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
                                b"multiple users\0" as *const u8 as *const libc::c_char,
                                5 as libc::c_int,
                            ),
                        );
                        if 0 as libc::c_int != 0 {} else {
                            unreachable!();
                        };
                    };
                }
                user = optarg;
            }
            108 => {
                if !range.is_null() {
                    if ::core::mem::size_of::<C2RustUnnamed_14>() as libc::c_ulong != 0 {
                        error(
                            EXIT_CANCELED as libc::c_int,
                            0 as libc::c_int,
                            dcgettext(
                                0 as *const libc::c_char,
                                b"multiple levelranges\0" as *const u8
                                    as *const libc::c_char,
                                5 as libc::c_int,
                            ),
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
                                b"multiple levelranges\0" as *const u8
                                    as *const libc::c_char,
                                5 as libc::c_int,
                            ),
                        );
                        if 0 as libc::c_int != 0 {} else {
                            unreachable!();
                        };
                    };
                }
                range = optarg;
            }
            99 => {
                compute_trans = 1 as libc::c_int != 0;
            }
            -2 => {
                usage(0 as libc::c_int);
            }
            -3 => {
                version_etc(
                    stdout,
                    b"runcon\0" as *const u8 as *const libc::c_char,
                    b"GNU coreutils\0" as *const u8 as *const libc::c_char,
                    Version,
                    b"Russell Coker\0" as *const u8 as *const libc::c_char,
                    0 as *mut libc::c_void as *mut libc::c_char,
                );
                exit(0 as libc::c_int);
            }
            _ => {
                usage(EXIT_CANCELED as libc::c_int);
            }
        }
    }
    if argc - optind == 0 as libc::c_int {
        if getcon(&mut cur_context) < 0 as libc::c_int {
            if ::core::mem::size_of::<C2RustUnnamed_13>() as libc::c_ulong != 0 {
                error(
                    EXIT_CANCELED as libc::c_int,
                    *__errno_location(),
                    dcgettext(
                        0 as *const libc::c_char,
                        b"failed to get current context\0" as *const u8
                            as *const libc::c_char,
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
                        b"failed to get current context\0" as *const u8
                            as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                );
                if 0 as libc::c_int != 0 {} else {
                    unreachable!();
                };
            };
        }
        fputs_unlocked(cur_context, stdout);
        fputc_unlocked('\n' as i32, stdout);
        return 0 as libc::c_int;
    }
    if !(!user.is_null() || !role.is_null() || !type_0.is_null() || !range.is_null()
        || compute_trans as libc::c_int != 0)
    {
        if optind >= argc {
            error(
                0 as libc::c_int,
                0 as libc::c_int,
                dcgettext(
                    0 as *const libc::c_char,
                    b"you must specify -c, -t, -u, -l, -r, or context\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
            );
            usage(EXIT_CANCELED as libc::c_int);
        }
        let fresh1 = optind;
        optind = optind + 1;
        context = *argv.offset(fresh1 as isize);
    }
    if optind >= argc {
        error(
            0 as libc::c_int,
            0 as libc::c_int,
            dcgettext(
                0 as *const libc::c_char,
                b"no command specified\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
        usage(EXIT_CANCELED as libc::c_int);
    }
    if 0 as libc::c_int != 1 as libc::c_int {
        if ::core::mem::size_of::<C2RustUnnamed_12>() as libc::c_ulong != 0 {
            error(
                EXIT_CANCELED as libc::c_int,
                0 as libc::c_int,
                dcgettext(
                    0 as *const libc::c_char,
                    b"%s may be used only on a SELinux kernel\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
                program_name,
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
                    b"%s may be used only on a SELinux kernel\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
                program_name,
            );
            if 0 as libc::c_int != 0 {} else {
                unreachable!();
            };
        };
    }
    if !context.is_null() {
        con = context_new(context);
        if con == 0 {
            if ::core::mem::size_of::<C2RustUnnamed_11>() as libc::c_ulong != 0 {
                error(
                    EXIT_CANCELED as libc::c_int,
                    *__errno_location(),
                    dcgettext(
                        0 as *const libc::c_char,
                        b"failed to create security context: %s\0" as *const u8
                            as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    quote(context),
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
                        b"failed to create security context: %s\0" as *const u8
                            as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    quote(context),
                );
                if 0 as libc::c_int != 0 {} else {
                    unreachable!();
                };
            };
        }
    } else {
        if getcon(&mut cur_context) < 0 as libc::c_int {
            if ::core::mem::size_of::<C2RustUnnamed_10>() as libc::c_ulong != 0 {
                error(
                    EXIT_CANCELED as libc::c_int,
                    *__errno_location(),
                    dcgettext(
                        0 as *const libc::c_char,
                        b"failed to get current context\0" as *const u8
                            as *const libc::c_char,
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
                        b"failed to get current context\0" as *const u8
                            as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                );
                if 0 as libc::c_int != 0 {} else {
                    unreachable!();
                };
            };
        }
        if compute_trans {
            if getfilecon(*argv.offset(optind as isize), &mut file_context)
                == -(1 as libc::c_int)
            {
                if ::core::mem::size_of::<C2RustUnnamed_9>() as libc::c_ulong != 0 {
                    error(
                        EXIT_CANCELED as libc::c_int,
                        *__errno_location(),
                        dcgettext(
                            0 as *const libc::c_char,
                            b"failed to get security context of %s\0" as *const u8
                                as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                        quotearg_style(
                            shell_escape_always_quoting_style,
                            *argv.offset(optind as isize),
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
                            b"failed to get security context of %s\0" as *const u8
                                as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                        quotearg_style(
                            shell_escape_always_quoting_style,
                            *argv.offset(optind as isize),
                        ),
                    );
                    if 0 as libc::c_int != 0 {} else {
                        unreachable!();
                    };
                };
            }
            if security_compute_create(
                cur_context,
                file_context,
                string_to_security_class(
                    b"process\0" as *const u8 as *const libc::c_char,
                ),
                &mut new_context,
            ) != 0 as libc::c_int
            {
                if ::core::mem::size_of::<C2RustUnnamed_8>() as libc::c_ulong != 0 {
                    error(
                        EXIT_CANCELED as libc::c_int,
                        *__errno_location(),
                        dcgettext(
                            0 as *const libc::c_char,
                            b"failed to compute a new context\0" as *const u8
                                as *const libc::c_char,
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
                            b"failed to compute a new context\0" as *const u8
                                as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                    );
                    if 0 as libc::c_int != 0 {} else {
                        unreachable!();
                    };
                };
            }
            freecon(file_context);
            freecon(cur_context);
            cur_context = new_context;
        }
        con = context_new(cur_context);
        if con == 0 {
            if ::core::mem::size_of::<C2RustUnnamed_7>() as libc::c_ulong != 0 {
                error(
                    EXIT_CANCELED as libc::c_int,
                    *__errno_location(),
                    dcgettext(
                        0 as *const libc::c_char,
                        b"failed to create security context: %s\0" as *const u8
                            as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    quote(cur_context),
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
                        b"failed to create security context: %s\0" as *const u8
                            as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    quote(cur_context),
                );
                if 0 as libc::c_int != 0 {} else {
                    unreachable!();
                };
            };
        }
        if !user.is_null() && context_user_set(con, user) != 0 {
            if ::core::mem::size_of::<C2RustUnnamed_6>() as libc::c_ulong != 0 {
                error(
                    EXIT_CANCELED as libc::c_int,
                    *__errno_location(),
                    dcgettext(
                        0 as *const libc::c_char,
                        b"failed to set new user: %s\0" as *const u8
                            as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    quote(user),
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
                        b"failed to set new user: %s\0" as *const u8
                            as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    quote(user),
                );
                if 0 as libc::c_int != 0 {} else {
                    unreachable!();
                };
            };
        }
        if !type_0.is_null() && context_type_set(con, type_0) != 0 {
            if ::core::mem::size_of::<C2RustUnnamed_5>() as libc::c_ulong != 0 {
                error(
                    EXIT_CANCELED as libc::c_int,
                    *__errno_location(),
                    dcgettext(
                        0 as *const libc::c_char,
                        b"failed to set new type: %s\0" as *const u8
                            as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    quote(type_0),
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
                        b"failed to set new type: %s\0" as *const u8
                            as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    quote(type_0),
                );
                if 0 as libc::c_int != 0 {} else {
                    unreachable!();
                };
            };
        }
        if !range.is_null() && context_range_set(con, range) != 0 {
            if ::core::mem::size_of::<C2RustUnnamed_4>() as libc::c_ulong != 0 {
                error(
                    EXIT_CANCELED as libc::c_int,
                    *__errno_location(),
                    dcgettext(
                        0 as *const libc::c_char,
                        b"failed to set new range: %s\0" as *const u8
                            as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    quote(range),
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
                        b"failed to set new range: %s\0" as *const u8
                            as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    quote(range),
                );
                if 0 as libc::c_int != 0 {} else {
                    unreachable!();
                };
            };
        }
        if !role.is_null() && context_role_set(con, role) != 0 {
            if ::core::mem::size_of::<C2RustUnnamed_3>() as libc::c_ulong != 0 {
                error(
                    EXIT_CANCELED as libc::c_int,
                    *__errno_location(),
                    dcgettext(
                        0 as *const libc::c_char,
                        b"failed to set new role: %s\0" as *const u8
                            as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    quote(role),
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
                        b"failed to set new role: %s\0" as *const u8
                            as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    quote(role),
                );
                if 0 as libc::c_int != 0 {} else {
                    unreachable!();
                };
            };
        }
    }
    if security_check_context(context_str(con)) < 0 as libc::c_int {
        if ::core::mem::size_of::<C2RustUnnamed_2>() as libc::c_ulong != 0 {
            error(
                EXIT_CANCELED as libc::c_int,
                *__errno_location(),
                dcgettext(
                    0 as *const libc::c_char,
                    b"invalid context: %s\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
                quote(context_str(con)),
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
                    b"invalid context: %s\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
                quote(context_str(con)),
            );
            if 0 as libc::c_int != 0 {} else {
                unreachable!();
            };
        };
    }
    if setexeccon(context_str(con)) != 0 as libc::c_int {
        if ::core::mem::size_of::<C2RustUnnamed_1>() as libc::c_ulong != 0 {
            error(
                EXIT_CANCELED as libc::c_int,
                *__errno_location(),
                dcgettext(
                    0 as *const libc::c_char,
                    b"unable to set security context %s\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
                quote(context_str(con)),
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
                    b"unable to set security context %s\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
                quote(context_str(con)),
            );
            if 0 as libc::c_int != 0 {} else {
                unreachable!();
            };
        };
    }
    if !cur_context.is_null() {
        freecon(cur_context);
    }
    if compute_trans as libc::c_int != 0 {
        Some(
            execv
                as unsafe extern "C" fn(
                    *const libc::c_char,
                    *const *mut libc::c_char,
                ) -> libc::c_int,
        )
    } else {
        Some(
            execvp
                as unsafe extern "C" fn(
                    *const libc::c_char,
                    *const *mut libc::c_char,
                ) -> libc::c_int,
        )
    }
        .expect(
            "non-null function pointer",
        )(
        *argv.offset(optind as isize),
        argv.offset(optind as isize) as *const *mut libc::c_char,
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
        quote(*argv.offset(optind as isize)),
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
