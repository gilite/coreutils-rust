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
    fn getpwuid(__uid: __uid_t) -> *mut passwd;
    fn getpwnam(__name: *const libc::c_char) -> *mut passwd;
    fn getgrgid(__gid: __gid_t) -> *mut group;
    fn getopt_long(
        ___argc: libc::c_int,
        ___argv: *const *mut libc::c_char,
        __shortopts: *const libc::c_char,
        __longopts: *const option,
        __longind: *mut libc::c_int,
    ) -> libc::c_int;
    static mut optind: libc::c_int;
    fn __errno_location() -> *mut libc::c_int;
    fn getuid() -> __uid_t;
    fn geteuid() -> __uid_t;
    fn getgid() -> __gid_t;
    fn getegid() -> __gid_t;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strncmp(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn free(_: *mut libc::c_void);
    fn umaxtostr(_: uintmax_t, _: *mut libc::c_char) -> *mut libc::c_char;
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
    fn xstrdup(str: *const libc::c_char) -> *mut libc::c_char;
    fn getenv(__name: *const libc::c_char) -> *mut libc::c_char;
    fn exit(_: libc::c_int) -> !;
    fn atexit(__func: Option::<unsafe extern "C" fn() -> ()>) -> libc::c_int;
    fn error(
        __status: libc::c_int,
        __errnum: libc::c_int,
        __format: *const libc::c_char,
        _: ...
    );
    fn xgetgroups(
        username: *const libc::c_char,
        gid: gid_t,
        groups: *mut *mut gid_t,
    ) -> libc::c_int;
    fn quote(arg: *const libc::c_char) -> *const libc::c_char;
    fn print_group(_: gid_t, _: bool) -> bool;
    fn print_group_list(
        _: *const libc::c_char,
        _: uid_t,
        _: gid_t,
        _: gid_t,
        _: bool,
        _: libc::c_char,
    ) -> bool;
    fn parse_user_spec(
        spec_arg: *const libc::c_char,
        uid: *mut uid_t,
        gid: *mut gid_t,
        username_arg: *mut *mut libc::c_char,
        groupname_arg: *mut *mut libc::c_char,
    ) -> *const libc::c_char;
}
pub type size_t = libc::c_ulong;
pub type __uintmax_t = libc::c_ulong;
pub type __uid_t = libc::c_uint;
pub type __gid_t = libc::c_uint;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __ssize_t = libc::c_long;
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
pub type ssize_t = __ssize_t;
pub type gid_t = __gid_t;
pub type uid_t = __uid_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct passwd {
    pub pw_name: *mut libc::c_char,
    pub pw_passwd: *mut libc::c_char,
    pub pw_uid: __uid_t,
    pub pw_gid: __gid_t,
    pub pw_gecos: *mut libc::c_char,
    pub pw_dir: *mut libc::c_char,
    pub pw_shell: *mut libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct group {
    pub gr_name: *mut libc::c_char,
    pub gr_passwd: *mut libc::c_char,
    pub gr_gid: __gid_t,
    pub gr_mem: *mut *mut libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct option {
    pub name: *const libc::c_char,
    pub has_arg: libc::c_int,
    pub flag: *mut libc::c_int,
    pub val: libc::c_int,
}
pub type uintmax_t = __uintmax_t;
pub type C2RustUnnamed = libc::c_int;
pub const GETOPT_VERSION_CHAR: C2RustUnnamed = -3;
pub const GETOPT_HELP_CHAR: C2RustUnnamed = -2;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct infomap {
    pub program: *const libc::c_char,
    pub node: *const libc::c_char,
}
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
#[inline]
unsafe extern "C" fn putchar_unlocked(mut __c: libc::c_int) -> libc::c_int {
    return if ((*stdout)._IO_write_ptr >= (*stdout)._IO_write_end) as libc::c_int
        as libc::c_long != 0
    {
        __overflow(stdout, __c as libc::c_uchar as libc::c_int)
    } else {
        let fresh0 = (*stdout)._IO_write_ptr;
        (*stdout)._IO_write_ptr = ((*stdout)._IO_write_ptr).offset(1);
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
unsafe extern "C" fn smack_new_label_from_self(
    mut label: *mut *mut libc::c_char,
) -> ssize_t {
    return -(1 as libc::c_int) as ssize_t;
}
#[inline]
unsafe extern "C" fn is_smack_enabled() -> bool {
    return 0 as libc::c_int != 0;
}
static mut just_context: bool = 0 as libc::c_int != 0;
static mut opt_zero: bool = 0 as libc::c_int != 0;
static mut just_group_list: bool = 0 as libc::c_int != 0;
static mut just_group: bool = 0 as libc::c_int != 0;
static mut use_real: bool = 0 as libc::c_int != 0;
static mut just_user: bool = 0 as libc::c_int != 0;
static mut ok: bool = 1 as libc::c_int != 0;
static mut multiple_users: bool = 0 as libc::c_int != 0;
static mut use_name: bool = 0 as libc::c_int != 0;
static mut ruid: uid_t = 0;
static mut euid: uid_t = 0;
static mut rgid: gid_t = 0;
static mut egid: gid_t = 0;
static mut context: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
static mut longopts: [option; 10] = [
    {
        let mut init = option {
            name: b"context\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'Z' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"group\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'g' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"groups\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'G' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"name\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'n' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"real\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'r' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"user\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'u' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"zero\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'z' as i32,
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
                b"Usage: %s [OPTION]... [USER]...\n\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
            program_name,
        );
        fputs_unlocked(
            dcgettext(
                0 as *const libc::c_char,
                b"Print user and group information for each specified USER,\nor (when USER omitted) for the current process.\n\n\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            stdout,
        );
        fputs_unlocked(
            dcgettext(
                0 as *const libc::c_char,
                b"  -a             ignore, for compatibility with other versions\n  -Z, --context  print only the security context of the process\n  -g, --group    print only the effective group ID\n  -G, --groups   print all group IDs\n  -n, --name     print a name instead of a number, for -ugG\n  -r, --real     print the real ID instead of the effective ID, with -ugG\n  -u, --user     print only the effective user ID\n  -z, --zero     delimit entries with NUL characters, not whitespace;\n                   not permitted in default format\n\0"
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
                b"\nWithout any OPTION, print some useful set of identified information.\n\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            stdout,
        );
        emit_ancillary_info(b"id\0" as *const u8 as *const libc::c_char);
    }
    exit(status);
}
unsafe fn main_0(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut optc: libc::c_int = 0;
    let mut selinux_enabled: libc::c_int = (0 as libc::c_int > 0 as libc::c_int)
        as libc::c_int;
    let mut smack_enabled: bool = is_smack_enabled();
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
            b"agnruzGZ\0" as *const u8 as *const libc::c_char,
            longopts.as_ptr(),
            0 as *mut libc::c_int,
        );
        if !(optc != -(1 as libc::c_int)) {
            break;
        }
        match optc {
            97 => {}
            90 => {
                if selinux_enabled == 0 {
                    if ::core::mem::size_of::<C2RustUnnamed_9>() as libc::c_ulong != 0 {
                        error(
                            1 as libc::c_int,
                            0 as libc::c_int,
                            dcgettext(
                                0 as *const libc::c_char,
                                b"--context (-Z) works only on an SELinux-enabled kernel\0"
                                    as *const u8 as *const libc::c_char,
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
                                b"--context (-Z) works only on an SELinux-enabled kernel\0"
                                    as *const u8 as *const libc::c_char,
                                5 as libc::c_int,
                            ),
                        );
                        if 0 as libc::c_int != 0 {} else {
                            unreachable!();
                        };
                    };
                }
                just_context = 1 as libc::c_int != 0;
            }
            103 => {
                just_group = 1 as libc::c_int != 0;
            }
            110 => {
                use_name = 1 as libc::c_int != 0;
            }
            114 => {
                use_real = 1 as libc::c_int != 0;
            }
            117 => {
                just_user = 1 as libc::c_int != 0;
            }
            122 => {
                opt_zero = 1 as libc::c_int != 0;
            }
            71 => {
                just_group_list = 1 as libc::c_int != 0;
            }
            -2 => {
                usage(0 as libc::c_int);
            }
            -3 => {
                version_etc(
                    stdout,
                    b"id\0" as *const u8 as *const libc::c_char,
                    b"GNU coreutils\0" as *const u8 as *const libc::c_char,
                    Version,
                    b"Arnold Robbins\0" as *const u8 as *const libc::c_char,
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
    let mut n_ids: size_t = (argc - optind) as size_t;
    if n_ids != 0 && just_context as libc::c_int != 0 {
        if ::core::mem::size_of::<C2RustUnnamed_8>() as libc::c_ulong != 0 {
            error(
                1 as libc::c_int,
                0 as libc::c_int,
                dcgettext(
                    0 as *const libc::c_char,
                    b"cannot print security context when user specified\0" as *const u8
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
                    b"cannot print security context when user specified\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
            );
            if 0 as libc::c_int != 0 {} else {
                unreachable!();
            };
        };
    }
    if just_user as libc::c_int + just_group as libc::c_int
        + just_group_list as libc::c_int + just_context as libc::c_int > 1 as libc::c_int
    {
        if ::core::mem::size_of::<C2RustUnnamed_7>() as libc::c_ulong != 0 {
            error(
                1 as libc::c_int,
                0 as libc::c_int,
                dcgettext(
                    0 as *const libc::c_char,
                    b"cannot print \"only\" of more than one choice\0" as *const u8
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
                    b"cannot print \"only\" of more than one choice\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
            );
            if 0 as libc::c_int != 0 {} else {
                unreachable!();
            };
        };
    }
    let mut default_format: bool = !(just_user as libc::c_int != 0
        || just_group as libc::c_int != 0 || just_group_list as libc::c_int != 0
        || just_context as libc::c_int != 0);
    if default_format as libc::c_int != 0
        && (use_real as libc::c_int != 0 || use_name as libc::c_int != 0)
    {
        if ::core::mem::size_of::<C2RustUnnamed_6>() as libc::c_ulong != 0 {
            error(
                1 as libc::c_int,
                0 as libc::c_int,
                dcgettext(
                    0 as *const libc::c_char,
                    b"cannot print only names or real IDs in default format\0"
                        as *const u8 as *const libc::c_char,
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
                    b"cannot print only names or real IDs in default format\0"
                        as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
            );
            if 0 as libc::c_int != 0 {} else {
                unreachable!();
            };
        };
    }
    if default_format as libc::c_int != 0 && opt_zero as libc::c_int != 0 {
        if ::core::mem::size_of::<C2RustUnnamed_5>() as libc::c_ulong != 0 {
            error(
                1 as libc::c_int,
                0 as libc::c_int,
                dcgettext(
                    0 as *const libc::c_char,
                    b"option --zero not permitted in default format\0" as *const u8
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
                    b"option --zero not permitted in default format\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
            );
            if 0 as libc::c_int != 0 {} else {
                unreachable!();
            };
        };
    }
    if n_ids == 0 as libc::c_int as libc::c_ulong
        && (just_context as libc::c_int != 0
            || default_format as libc::c_int != 0
                && (getenv(b"POSIXLY_CORRECT\0" as *const u8 as *const libc::c_char))
                    .is_null())
    {
        if selinux_enabled != 0 && getcon(&mut context) != 0
            && just_context as libc::c_int != 0
            || smack_enabled as libc::c_int != 0
                && smack_new_label_from_self(&mut context)
                    < 0 as libc::c_int as libc::c_long
                && just_context as libc::c_int != 0
        {
            if ::core::mem::size_of::<C2RustUnnamed_4>() as libc::c_ulong != 0 {
                error(
                    1 as libc::c_int,
                    0 as libc::c_int,
                    dcgettext(
                        0 as *const libc::c_char,
                        b"can't get process context\0" as *const u8
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
                        b"can't get process context\0" as *const u8
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
    if n_ids >= 1 as libc::c_int as libc::c_ulong {
        multiple_users = if n_ids > 1 as libc::c_int as libc::c_ulong {
            1 as libc::c_int
        } else {
            0 as libc::c_int
        } != 0;
        n_ids = (n_ids as libc::c_ulong).wrapping_add(optind as libc::c_ulong) as size_t
            as size_t;
        while (optind as libc::c_ulong) < n_ids {
            let mut pw_name: *mut libc::c_char = 0 as *mut libc::c_char;
            let mut pwd: *mut passwd = 0 as *mut passwd;
            let mut spec: *const libc::c_char = *argv.offset(optind as isize);
            if *spec != 0 {
                if (parse_user_spec(
                    spec,
                    &mut euid,
                    0 as *mut gid_t,
                    &mut pw_name,
                    0 as *mut *mut libc::c_char,
                ))
                    .is_null()
                {
                    pwd = if !pw_name.is_null() {
                        getpwnam(pw_name)
                    } else {
                        getpwuid(euid)
                    };
                }
            }
            if pwd.is_null() {
                error(
                    0 as libc::c_int,
                    *__errno_location(),
                    dcgettext(
                        0 as *const libc::c_char,
                        b"%s: no such user\0" as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    quote(spec),
                );
                ok = (ok as libc::c_int & 0 as libc::c_int) as bool;
            } else {
                if pw_name.is_null() {
                    pw_name = xstrdup((*pwd).pw_name);
                }
                euid = (*pwd).pw_uid;
                ruid = euid;
                egid = (*pwd).pw_gid;
                rgid = egid;
                print_stuff(pw_name);
            }
            free(pw_name as *mut libc::c_void);
            optind += 1;
        }
    } else {
        let mut NO_UID: uid_t = -(1 as libc::c_int) as uid_t;
        let mut NO_GID: gid_t = -(1 as libc::c_int) as gid_t;
        if if just_user as libc::c_int != 0 {
            !use_real as libc::c_int
        } else {
            (!just_group && !just_group_list && !just_context) as libc::c_int
        } != 0
        {
            *__errno_location() = 0 as libc::c_int;
            euid = geteuid();
            if euid == NO_UID && *__errno_location() != 0 {
                if ::core::mem::size_of::<C2RustUnnamed_3>() as libc::c_ulong != 0 {
                    error(
                        1 as libc::c_int,
                        *__errno_location(),
                        dcgettext(
                            0 as *const libc::c_char,
                            b"cannot get effective UID\0" as *const u8
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
                        *__errno_location(),
                        dcgettext(
                            0 as *const libc::c_char,
                            b"cannot get effective UID\0" as *const u8
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
        if if just_user as libc::c_int != 0 {
            use_real as libc::c_int
        } else {
            (!just_group && (just_group_list as libc::c_int != 0 || !just_context))
                as libc::c_int
        } != 0
        {
            *__errno_location() = 0 as libc::c_int;
            ruid = getuid();
            if ruid == NO_UID && *__errno_location() != 0 {
                if ::core::mem::size_of::<C2RustUnnamed_2>() as libc::c_ulong != 0 {
                    error(
                        1 as libc::c_int,
                        *__errno_location(),
                        dcgettext(
                            0 as *const libc::c_char,
                            b"cannot get real UID\0" as *const u8 as *const libc::c_char,
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
                            b"cannot get real UID\0" as *const u8 as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                    );
                    if 0 as libc::c_int != 0 {} else {
                        unreachable!();
                    };
                };
            }
        }
        if !just_user
            && (just_group as libc::c_int != 0 || just_group_list as libc::c_int != 0
                || !just_context)
        {
            *__errno_location() = 0 as libc::c_int;
            egid = getegid();
            if egid == NO_GID && *__errno_location() != 0 {
                if ::core::mem::size_of::<C2RustUnnamed_1>() as libc::c_ulong != 0 {
                    error(
                        1 as libc::c_int,
                        *__errno_location(),
                        dcgettext(
                            0 as *const libc::c_char,
                            b"cannot get effective GID\0" as *const u8
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
                        *__errno_location(),
                        dcgettext(
                            0 as *const libc::c_char,
                            b"cannot get effective GID\0" as *const u8
                                as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                    );
                    if 0 as libc::c_int != 0 {} else {
                        unreachable!();
                    };
                };
            }
            *__errno_location() = 0 as libc::c_int;
            rgid = getgid();
            if rgid == NO_GID && *__errno_location() != 0 {
                if ::core::mem::size_of::<C2RustUnnamed_0>() as libc::c_ulong != 0 {
                    error(
                        1 as libc::c_int,
                        *__errno_location(),
                        dcgettext(
                            0 as *const libc::c_char,
                            b"cannot get real GID\0" as *const u8 as *const libc::c_char,
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
                            b"cannot get real GID\0" as *const u8 as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                    );
                    if 0 as libc::c_int != 0 {} else {
                        unreachable!();
                    };
                };
            }
        }
        print_stuff(0 as *const libc::c_char);
    }
    return if ok as libc::c_int != 0 { 0 as libc::c_int } else { 1 as libc::c_int };
}
unsafe extern "C" fn gidtostr_ptr(mut gid: *const gid_t) -> *mut libc::c_char {
    static mut buf: [libc::c_char; 21] = [0; 21];
    return umaxtostr(*gid as uintmax_t, buf.as_mut_ptr());
}
unsafe extern "C" fn uidtostr_ptr(mut uid: *const uid_t) -> *mut libc::c_char {
    static mut buf: [libc::c_char; 21] = [0; 21];
    return umaxtostr(*uid as uintmax_t, buf.as_mut_ptr());
}
unsafe extern "C" fn print_user(mut uid: uid_t) {
    let mut pwd: *mut passwd = 0 as *mut passwd;
    if use_name {
        pwd = getpwuid(uid);
        if pwd.is_null() {
            error(
                0 as libc::c_int,
                0 as libc::c_int,
                dcgettext(
                    0 as *const libc::c_char,
                    b"cannot find name for user ID %s\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
                uidtostr_ptr(&mut uid),
            );
            ok = (ok as libc::c_int & 0 as libc::c_int) as bool;
        }
    }
    let mut s: *mut libc::c_char = if !pwd.is_null() {
        (*pwd).pw_name
    } else {
        uidtostr_ptr(&mut uid)
    };
    fputs_unlocked(s, stdout);
}
unsafe extern "C" fn print_full_info(mut username: *const libc::c_char) {
    let mut pwd: *mut passwd = 0 as *mut passwd;
    let mut grp: *mut group = 0 as *mut group;
    printf(
        dcgettext(
            0 as *const libc::c_char,
            b"uid=%s\0" as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        ),
        uidtostr_ptr(&mut ruid),
    );
    pwd = getpwuid(ruid);
    if !pwd.is_null() {
        printf(b"(%s)\0" as *const u8 as *const libc::c_char, (*pwd).pw_name);
    }
    printf(
        dcgettext(
            0 as *const libc::c_char,
            b" gid=%s\0" as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        ),
        gidtostr_ptr(&mut rgid),
    );
    grp = getgrgid(rgid);
    if !grp.is_null() {
        printf(b"(%s)\0" as *const u8 as *const libc::c_char, (*grp).gr_name);
    }
    if euid != ruid {
        printf(
            dcgettext(
                0 as *const libc::c_char,
                b" euid=%s\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            uidtostr_ptr(&mut euid),
        );
        pwd = getpwuid(euid);
        if !pwd.is_null() {
            printf(b"(%s)\0" as *const u8 as *const libc::c_char, (*pwd).pw_name);
        }
    }
    if egid != rgid {
        printf(
            dcgettext(
                0 as *const libc::c_char,
                b" egid=%s\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            gidtostr_ptr(&mut egid),
        );
        grp = getgrgid(egid);
        if !grp.is_null() {
            printf(b"(%s)\0" as *const u8 as *const libc::c_char, (*grp).gr_name);
        }
    }
    let mut groups: *mut gid_t = 0 as *mut gid_t;
    let mut primary_group: gid_t = 0;
    if !username.is_null() {
        primary_group = if !pwd.is_null() {
            (*pwd).pw_gid
        } else {
            -(1 as libc::c_int) as libc::c_uint
        };
    } else {
        primary_group = egid;
    }
    let mut n_groups: libc::c_int = xgetgroups(username, primary_group, &mut groups);
    if n_groups < 0 as libc::c_int {
        if !username.is_null() {
            error(
                0 as libc::c_int,
                *__errno_location(),
                dcgettext(
                    0 as *const libc::c_char,
                    b"failed to get groups for user %s\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
                quote(username),
            );
        } else {
            error(
                0 as libc::c_int,
                *__errno_location(),
                dcgettext(
                    0 as *const libc::c_char,
                    b"failed to get groups for the current process\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
            );
        }
        ok = (ok as libc::c_int & 0 as libc::c_int) as bool;
        return;
    }
    if n_groups > 0 as libc::c_int {
        fputs_unlocked(
            dcgettext(
                0 as *const libc::c_char,
                b" groups=\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            stdout,
        );
    }
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < n_groups {
        if i > 0 as libc::c_int {
            putchar_unlocked(',' as i32);
        }
        fputs_unlocked(gidtostr_ptr(&mut *groups.offset(i as isize)), stdout);
        grp = getgrgid(*groups.offset(i as isize));
        if !grp.is_null() {
            printf(b"(%s)\0" as *const u8 as *const libc::c_char, (*grp).gr_name);
        }
        i += 1;
    }
    free(groups as *mut libc::c_void);
    if !context.is_null() {
        printf(
            dcgettext(
                0 as *const libc::c_char,
                b" context=%s\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            context,
        );
    }
}
unsafe extern "C" fn print_stuff(mut pw_name: *const libc::c_char) {
    if just_user {
        print_user(if use_real as libc::c_int != 0 { ruid } else { euid });
    } else if just_group {
        ok = (ok as libc::c_int
            & print_group(
                if use_real as libc::c_int != 0 { rgid } else { egid },
                use_name,
            ) as libc::c_int) as bool;
    } else if just_group_list {
        ok = (ok as libc::c_int
            & print_group_list(
                pw_name,
                ruid,
                rgid,
                egid,
                use_name,
                (if opt_zero as libc::c_int != 0 { '\0' as i32 } else { ' ' as i32 })
                    as libc::c_char,
            ) as libc::c_int) as bool;
    } else if just_context {
        fputs_unlocked(context, stdout);
    } else {
        print_full_info(pw_name);
    }
    if opt_zero as libc::c_int != 0 && just_group_list as libc::c_int != 0
        && multiple_users as libc::c_int != 0
    {
        putchar_unlocked('\0' as i32);
        putchar_unlocked('\0' as i32);
    } else {
        putchar_unlocked(
            if opt_zero as libc::c_int != 0 { '\0' as i32 } else { '\n' as i32 },
        );
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
