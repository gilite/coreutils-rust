#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![feature(extern_types)]
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type selinux_opt;
    pub type selabel_handle;
    pub type mode_change;
    static mut stdout: *mut FILE;
    static mut stderr: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn fputs_unlocked(__s: *const libc::c_char, __stream: *mut FILE) -> libc::c_int;
    static mut optarg: *mut libc::c_char;
    static mut optind: libc::c_int;
    fn getopt_long(
        ___argc: libc::c_int,
        ___argv: *const *mut libc::c_char,
        __shortopts: *const libc::c_char,
        __longopts: *const option,
        __longind: *mut libc::c_int,
    ) -> libc::c_int;
    fn __errno_location() -> *mut libc::c_int;
    fn mkdir(__path: *const libc::c_char, __mode: __mode_t) -> libc::c_int;
    fn umask(__mask: __mode_t) -> __mode_t;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strncmp(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn free(_: *mut libc::c_void);
    fn quotearg_style(s: quoting_style, arg: *const libc::c_char) -> *mut libc::c_char;
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
    fn last_component(filename: *const libc::c_char) -> *mut libc::c_char;
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
    fn make_dir_parents(
        dir: *mut libc::c_char,
        wd: *mut savewd,
        make_ancestor_0: Option::<
            unsafe extern "C" fn(
                *const libc::c_char,
                *const libc::c_char,
                *mut libc::c_void,
            ) -> libc::c_int,
        >,
        options: *mut libc::c_void,
        mode: mode_t,
        announce: Option::<
            unsafe extern "C" fn(*const libc::c_char, *mut libc::c_void) -> (),
        >,
        mode_bits: mode_t,
        owner: uid_t,
        group: gid_t,
        preserve_existing: bool,
    ) -> bool;
    fn mode_compile(_: *const libc::c_char) -> *mut mode_change;
    fn mode_adjust(
        _: mode_t,
        _: bool,
        _: mode_t,
        _: *const mode_change,
        _: *mut mode_t,
    ) -> mode_t;
    fn prog_fprintf(fp: *mut FILE, fmt: *const libc::c_char, _: ...);
    fn quote(arg: *const libc::c_char) -> *const libc::c_char;
    fn savewd_process_files(
        n_files: libc::c_int,
        file: *mut *mut libc::c_char,
        act: Option::<
            unsafe extern "C" fn(
                *mut libc::c_char,
                *mut savewd,
                *mut libc::c_void,
            ) -> libc::c_int,
        >,
        options: *mut libc::c_void,
    ) -> libc::c_int;
}
pub type size_t = libc::c_ulong;
pub type __uid_t = libc::c_uint;
pub type __gid_t = libc::c_uint;
pub type __mode_t = libc::c_uint;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __pid_t = libc::c_int;
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
pub type mode_t = __mode_t;
pub type uid_t = __uid_t;
pub type pid_t = __pid_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct option {
    pub name: *const libc::c_char,
    pub has_arg: libc::c_int,
    pub flag: *mut libc::c_int,
    pub val: libc::c_int,
}
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct savewd {
    pub state: C2RustUnnamed_1,
    pub val: C2RustUnnamed_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_0 {
    pub fd: libc::c_int,
    pub errnum: libc::c_int,
    pub child: pid_t,
}
pub type C2RustUnnamed_1 = libc::c_uint;
pub const FINAL_STATE: C2RustUnnamed_1 = 5;
pub const ERROR_STATE: C2RustUnnamed_1 = 4;
pub const FORKING_STATE: C2RustUnnamed_1 = 3;
pub const FD_POST_CHDIR_STATE: C2RustUnnamed_1 = 2;
pub const FD_STATE: C2RustUnnamed_1 = 1;
pub const INITIAL_STATE: C2RustUnnamed_1 = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mkdir_options {
    pub make_ancestor_function: Option::<
        unsafe extern "C" fn(
            *const libc::c_char,
            *const libc::c_char,
            *mut libc::c_void,
        ) -> libc::c_int,
    >,
    pub umask_ancestor: mode_t,
    pub umask_self: mode_t,
    pub mode: mode_t,
    pub mode_bits: mode_t,
    pub set_security_context: *mut selabel_handle,
    pub created_directory_format: *const libc::c_char,
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
#[inline]
unsafe extern "C" fn selabel_open(
    mut backend: libc::c_int,
    mut options: *mut selinux_opt,
    mut nopt: libc::c_uint,
) -> *mut selabel_handle {
    *__errno_location() = 95 as libc::c_int;
    return 0 as *mut selabel_handle;
}
#[inline]
unsafe extern "C" fn setfscreatecon(mut con: *const libc::c_char) -> libc::c_int {
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
unsafe extern "C" fn ignorable_ctx_err(mut err: libc::c_int) -> bool {
    return err == 95 as libc::c_int || err == 61 as libc::c_int;
}
#[inline]
unsafe extern "C" fn restorecon(
    mut selabel_handle: *mut selabel_handle,
    mut path: *const libc::c_char,
    mut recurse: bool,
) -> bool {
    *__errno_location() = 95 as libc::c_int;
    return 0 as libc::c_int != 0;
}
#[inline]
unsafe extern "C" fn defaultcon(
    mut selabel_handle: *mut selabel_handle,
    mut path: *const libc::c_char,
    mut mode: mode_t,
) -> libc::c_int {
    *__errno_location() = 95 as libc::c_int;
    return -(1 as libc::c_int);
}
#[inline]
unsafe extern "C" fn smack_set_label_for_self(
    mut label: *const libc::c_char,
) -> libc::c_int {
    return -(1 as libc::c_int);
}
#[inline]
unsafe extern "C" fn is_smack_enabled() -> bool {
    return 0 as libc::c_int != 0;
}
static mut longopts: [option; 7] = [
    {
        let mut init = option {
            name: b"context\0" as *const u8 as *const libc::c_char,
            has_arg: 2 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'Z' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"mode\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'm' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"parents\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'p' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"verbose\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'v' as i32,
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
                b"Usage: %s [OPTION]... DIRECTORY...\n\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
            program_name,
        );
        fputs_unlocked(
            dcgettext(
                0 as *const libc::c_char,
                b"Create the DIRECTORY(ies), if they do not already exist.\n\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            stdout,
        );
        emit_mandatory_arg_note();
        fputs_unlocked(
            dcgettext(
                0 as *const libc::c_char,
                b"  -m, --mode=MODE   set file mode (as in chmod), not a=rwx - umask\n  -p, --parents     no error if existing, make parent directories as needed,\n                    with their file modes unaffected by any -m option.\n  -v, --verbose     print a message for each created directory\n\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            stdout,
        );
        fputs_unlocked(
            dcgettext(
                0 as *const libc::c_char,
                b"  -Z                   set SELinux security context of each created directory\n                         to the default type\n      --context[=CTX]  like -Z, or if CTX is specified then set the SELinux\n                         or SMACK security context to CTX\n\0"
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
        emit_ancillary_info(b"mkdir\0" as *const u8 as *const libc::c_char);
    }
    exit(status);
}
unsafe extern "C" fn announce_mkdir(
    mut dir: *const libc::c_char,
    mut options: *mut libc::c_void,
) {
    let mut o: *const mkdir_options = options as *const mkdir_options;
    if !((*o).created_directory_format).is_null() {
        prog_fprintf(
            stdout,
            (*o).created_directory_format,
            quotearg_style(shell_escape_always_quoting_style, dir),
        );
    }
}
unsafe extern "C" fn make_ancestor(
    mut dir: *const libc::c_char,
    mut component: *const libc::c_char,
    mut options: *mut libc::c_void,
) -> libc::c_int {
    let mut o: *const mkdir_options = options as *const mkdir_options;
    if !((*o).set_security_context).is_null()
        && defaultcon(
            (*o).set_security_context,
            component,
            0o40000 as libc::c_int as mode_t,
        ) < 0 as libc::c_int && !ignorable_ctx_err(*__errno_location())
    {
        error(
            0 as libc::c_int,
            *__errno_location(),
            dcgettext(
                0 as *const libc::c_char,
                b"failed to set default creation context for %s\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
            quotearg_style(shell_escape_always_quoting_style, dir),
        );
    }
    if (*o).umask_ancestor != (*o).umask_self {
        umask((*o).umask_ancestor);
    }
    let mut r: libc::c_int = mkdir(
        component,
        (0o400 as libc::c_int | 0o200 as libc::c_int | 0o100 as libc::c_int
            | (0o400 as libc::c_int | 0o200 as libc::c_int | 0o100 as libc::c_int)
                >> 3 as libc::c_int
            | (0o400 as libc::c_int | 0o200 as libc::c_int | 0o100 as libc::c_int)
                >> 3 as libc::c_int >> 3 as libc::c_int) as __mode_t,
    );
    if (*o).umask_ancestor != (*o).umask_self {
        let mut mkdir_errno: libc::c_int = *__errno_location();
        umask((*o).umask_self);
        *__errno_location() = mkdir_errno;
    }
    if r == 0 as libc::c_int {
        r = ((*o).umask_ancestor & 0o400 as libc::c_int as libc::c_uint
            != 0 as libc::c_int as libc::c_uint) as libc::c_int;
        announce_mkdir(dir, options);
    }
    return r;
}
unsafe extern "C" fn process_dir(
    mut dir: *mut libc::c_char,
    mut wd: *mut savewd,
    mut options: *mut libc::c_void,
) -> libc::c_int {
    let mut o: *const mkdir_options = options as *const mkdir_options;
    if !((*o).set_security_context).is_null() {
        if ((*o).make_ancestor_function).is_none()
            && defaultcon(
                (*o).set_security_context,
                dir,
                0o40000 as libc::c_int as mode_t,
            ) < 0 as libc::c_int && !ignorable_ctx_err(*__errno_location())
        {
            error(
                0 as libc::c_int,
                *__errno_location(),
                dcgettext(
                    0 as *const libc::c_char,
                    b"failed to set default creation context for %s\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
                quotearg_style(shell_escape_always_quoting_style, dir),
            );
        }
    }
    let mut ret: libc::c_int = if make_dir_parents(
        dir,
        wd,
        (*o).make_ancestor_function,
        options,
        (*o).mode,
        Some(
            announce_mkdir
                as unsafe extern "C" fn(*const libc::c_char, *mut libc::c_void) -> (),
        ),
        (*o).mode_bits,
        -(1 as libc::c_int) as uid_t,
        -(1 as libc::c_int) as gid_t,
        1 as libc::c_int != 0,
    ) as libc::c_int != 0
    {
        0 as libc::c_int
    } else {
        1 as libc::c_int
    };
    if ret == 0 as libc::c_int && !((*o).set_security_context).is_null()
        && ((*o).make_ancestor_function).is_some()
    {
        if !restorecon(
            (*o).set_security_context,
            last_component(dir),
            0 as libc::c_int != 0,
        ) && !ignorable_ctx_err(*__errno_location())
        {
            error(
                0 as libc::c_int,
                *__errno_location(),
                dcgettext(
                    0 as *const libc::c_char,
                    b"failed to restore context for %s\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
                quotearg_style(shell_escape_always_quoting_style, dir),
            );
        }
    }
    return ret;
}
unsafe fn main_0(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut specified_mode: *const libc::c_char = 0 as *const libc::c_char;
    let mut optc: libc::c_int = 0;
    let mut scontext: *const libc::c_char = 0 as *const libc::c_char;
    let mut options: mkdir_options = mkdir_options {
        make_ancestor_function: None,
        umask_ancestor: 0,
        umask_self: 0,
        mode: 0,
        mode_bits: 0,
        set_security_context: 0 as *mut selabel_handle,
        created_directory_format: 0 as *const libc::c_char,
    };
    options.make_ancestor_function = None;
    options
        .mode = (0o400 as libc::c_int | 0o200 as libc::c_int | 0o100 as libc::c_int
        | (0o400 as libc::c_int | 0o200 as libc::c_int | 0o100 as libc::c_int)
            >> 3 as libc::c_int
        | (0o400 as libc::c_int | 0o200 as libc::c_int | 0o100 as libc::c_int)
            >> 3 as libc::c_int >> 3 as libc::c_int) as mode_t;
    options.mode_bits = 0 as libc::c_int as mode_t;
    options.created_directory_format = 0 as *const libc::c_char;
    options.set_security_context = 0 as *mut selabel_handle;
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
            b"pm:vZ\0" as *const u8 as *const libc::c_char,
            longopts.as_ptr(),
            0 as *mut libc::c_int,
        );
        if !(optc != -(1 as libc::c_int)) {
            break;
        }
        match optc {
            112 => {
                options
                    .make_ancestor_function = Some(
                    make_ancestor
                        as unsafe extern "C" fn(
                            *const libc::c_char,
                            *const libc::c_char,
                            *mut libc::c_void,
                        ) -> libc::c_int,
                );
            }
            109 => {
                specified_mode = optarg;
            }
            118 => {
                options
                    .created_directory_format = dcgettext(
                    0 as *const libc::c_char,
                    b"created directory %s\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                );
            }
            90 => {
                if is_smack_enabled() {
                    scontext = optarg;
                } else if 0 as libc::c_int > 0 as libc::c_int {
                    if !optarg.is_null() {
                        scontext = optarg;
                    } else {
                        options
                            .set_security_context = selabel_open(
                            0 as libc::c_int,
                            0 as *mut selinux_opt,
                            0 as libc::c_int as libc::c_uint,
                        );
                        if (options.set_security_context).is_null() {
                            error(
                                0 as libc::c_int,
                                *__errno_location(),
                                dcgettext(
                                    0 as *const libc::c_char,
                                    b"warning: ignoring --context\0" as *const u8
                                        as *const libc::c_char,
                                    5 as libc::c_int,
                                ),
                            );
                        }
                    }
                } else if !optarg.is_null() {
                    error(
                        0 as libc::c_int,
                        0 as libc::c_int,
                        dcgettext(
                            0 as *const libc::c_char,
                            b"warning: ignoring --context; it requires an SELinux/SMACK-enabled kernel\0"
                                as *const u8 as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                    );
                }
            }
            -2 => {
                usage(0 as libc::c_int);
            }
            -3 => {
                version_etc(
                    stdout,
                    b"mkdir\0" as *const u8 as *const libc::c_char,
                    b"GNU coreutils\0" as *const u8 as *const libc::c_char,
                    Version,
                    b"David MacKenzie\0" as *const u8 as *const libc::c_char,
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
    if !scontext.is_null() {
        let mut ret: libc::c_int = 0 as libc::c_int;
        if is_smack_enabled() {
            ret = smack_set_label_for_self(scontext);
        } else {
            ret = setfscreatecon(scontext);
        }
        if ret < 0 as libc::c_int {
            if ::core::mem::size_of::<C2RustUnnamed_3>() as libc::c_ulong != 0 {
                error(
                    1 as libc::c_int,
                    *__errno_location(),
                    dcgettext(
                        0 as *const libc::c_char,
                        b"failed to set default file creation context to %s\0"
                            as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    quote(scontext),
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
                        b"failed to set default file creation context to %s\0"
                            as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    quote(scontext),
                );
                if 0 as libc::c_int != 0 {} else {
                    unreachable!();
                };
            };
        }
    }
    if (options.make_ancestor_function).is_some() || !specified_mode.is_null() {
        let mut umask_value: mode_t = umask(0 as libc::c_int as __mode_t);
        options
            .umask_ancestor = umask_value
            & !(0o200 as libc::c_int | 0o100 as libc::c_int) as libc::c_uint;
        if !specified_mode.is_null() {
            let mut change: *mut mode_change = mode_compile(specified_mode);
            if change.is_null() {
                if ::core::mem::size_of::<C2RustUnnamed_2>() as libc::c_ulong != 0 {
                    error(
                        1 as libc::c_int,
                        0 as libc::c_int,
                        dcgettext(
                            0 as *const libc::c_char,
                            b"invalid mode %s\0" as *const u8 as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                        quote(specified_mode),
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
                            b"invalid mode %s\0" as *const u8 as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                        quote(specified_mode),
                    );
                    if 0 as libc::c_int != 0 {} else {
                        unreachable!();
                    };
                };
            }
            options
                .mode = mode_adjust(
                (0o400 as libc::c_int | 0o200 as libc::c_int | 0o100 as libc::c_int
                    | (0o400 as libc::c_int | 0o200 as libc::c_int
                        | 0o100 as libc::c_int) >> 3 as libc::c_int
                    | (0o400 as libc::c_int | 0o200 as libc::c_int
                        | 0o100 as libc::c_int) >> 3 as libc::c_int >> 3 as libc::c_int)
                    as mode_t,
                1 as libc::c_int != 0,
                umask_value,
                change,
                &mut options.mode_bits,
            );
            options.umask_self = umask_value & !options.mode;
            free(change as *mut libc::c_void);
        } else {
            options
                .mode = (0o400 as libc::c_int | 0o200 as libc::c_int
                | 0o100 as libc::c_int
                | (0o400 as libc::c_int | 0o200 as libc::c_int | 0o100 as libc::c_int)
                    >> 3 as libc::c_int
                | (0o400 as libc::c_int | 0o200 as libc::c_int | 0o100 as libc::c_int)
                    >> 3 as libc::c_int >> 3 as libc::c_int) as mode_t;
            options.umask_self = umask_value;
        }
        umask(options.umask_self);
    }
    return savewd_process_files(
        argc - optind,
        argv.offset(optind as isize),
        Some(
            process_dir
                as unsafe extern "C" fn(
                    *mut libc::c_char,
                    *mut savewd,
                    *mut libc::c_void,
                ) -> libc::c_int,
        ),
        &mut options as *mut mkdir_options as *mut libc::c_void,
    );
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
