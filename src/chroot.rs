#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![feature(extern_types)]
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    static mut optarg: *mut libc::c_char;
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
    fn getpwuid(__uid: __uid_t) -> *mut passwd;
    fn getgrnam(__name: *const libc::c_char) -> *mut group;
    fn setgroups(__n: size_t, __groups: *const __gid_t) -> libc::c_int;
    fn chdir(__path: *const libc::c_char) -> libc::c_int;
    fn execvp(
        __file: *const libc::c_char,
        __argv: *const *mut libc::c_char,
    ) -> libc::c_int;
    fn setuid(__uid: __uid_t) -> libc::c_int;
    fn setgid(__gid: __gid_t) -> libc::c_int;
    fn chroot(__path: *const libc::c_char) -> libc::c_int;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strncmp(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn strtok(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn free(_: *mut libc::c_void);
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
    fn x2nrealloc(p: *mut libc::c_void, pn: *mut size_t, s: size_t) -> *mut libc::c_void;
    fn xstrdup(str: *const libc::c_char) -> *mut libc::c_char;
    fn close_stdout();
    fn canonicalize_file_name(__name: *const libc::c_char) -> *mut libc::c_char;
    fn atexit(__func: Option::<unsafe extern "C" fn() -> ()>) -> libc::c_int;
    fn getenv(__name: *const libc::c_char) -> *mut libc::c_char;
    fn exit(_: libc::c_int) -> !;
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
    fn parse_user_spec(
        spec_arg: *const libc::c_char,
        uid: *mut uid_t,
        gid: *mut gid_t,
        username_arg: *mut *mut libc::c_char,
        groupname_arg: *mut *mut libc::c_char,
    ) -> *const libc::c_char;
    fn parse_user_spec_warn(
        spec_arg: *const libc::c_char,
        uid: *mut uid_t,
        gid: *mut gid_t,
        username_arg: *mut *mut libc::c_char,
        groupname_arg: *mut *mut libc::c_char,
        pwarn: *mut bool,
    ) -> *const libc::c_char;
    fn xstrtoumax(
        _: *const libc::c_char,
        _: *mut *mut libc::c_char,
        _: libc::c_int,
        _: *mut uintmax_t,
        _: *const libc::c_char,
    ) -> strtol_error;
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
pub type __uid_t = libc::c_uint;
pub type __gid_t = libc::c_uint;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __intptr_t = libc::c_long;
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
pub type intptr_t = __intptr_t;
pub type C2RustUnnamed = libc::c_uint;
pub const EXIT_ENOENT: C2RustUnnamed = 127;
pub const EXIT_CANNOT_INVOKE: C2RustUnnamed = 126;
pub const EXIT_CANCELED: C2RustUnnamed = 125;
pub const EXIT_TIMEDOUT: C2RustUnnamed = 124;
pub type uintmax_t = __uintmax_t;
pub type C2RustUnnamed_0 = libc::c_uint;
pub const _ISalnum: C2RustUnnamed_0 = 8;
pub const _ISpunct: C2RustUnnamed_0 = 4;
pub const _IScntrl: C2RustUnnamed_0 = 2;
pub const _ISblank: C2RustUnnamed_0 = 1;
pub const _ISgraph: C2RustUnnamed_0 = 32768;
pub const _ISprint: C2RustUnnamed_0 = 16384;
pub const _ISspace: C2RustUnnamed_0 = 8192;
pub const _ISxdigit: C2RustUnnamed_0 = 4096;
pub const _ISdigit: C2RustUnnamed_0 = 2048;
pub const _ISalpha: C2RustUnnamed_0 = 1024;
pub const _ISlower: C2RustUnnamed_0 = 512;
pub const _ISupper: C2RustUnnamed_0 = 256;
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
pub type strtol_error = libc::c_uint;
pub const LONGINT_INVALID: strtol_error = 4;
pub const LONGINT_INVALID_SUFFIX_CHAR_WITH_OVERFLOW: strtol_error = 3;
pub const LONGINT_INVALID_SUFFIX_CHAR: strtol_error = 2;
pub const LONGINT_OVERFLOW: strtol_error = 1;
pub const LONGINT_OK: strtol_error = 0;
pub type C2RustUnnamed_2 = libc::c_uint;
pub const SKIP_CHDIR: C2RustUnnamed_2 = 258;
pub const USERSPEC: C2RustUnnamed_2 = 257;
pub const GROUPS: C2RustUnnamed_2 = 256;
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
#[inline]
unsafe extern "C" fn bad_cast(mut s: *const libc::c_char) -> *mut libc::c_char {
    return s as *mut libc::c_char;
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
unsafe extern "C" fn initialize_exit_failure(mut status: libc::c_int) {
    if status != 1 as libc::c_int {
        ::core::ptr::write_volatile(&mut exit_failure as *mut libc::c_int, status);
    }
}
#[inline]
unsafe extern "C" fn to_uchar(mut ch: libc::c_char) -> libc::c_uchar {
    return ch as libc::c_uchar;
}
#[inline]
unsafe extern "C" fn uid_unset(mut uid: uid_t) -> bool {
    return uid == -(1 as libc::c_int) as uid_t;
}
#[inline]
unsafe extern "C" fn gid_unset(mut gid: gid_t) -> bool {
    return gid == -(1 as libc::c_int) as gid_t;
}
static mut long_opts: [option; 6] = [
    {
        let mut init = option {
            name: b"groups\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: GROUPS as libc::c_int,
        };
        init
    },
    {
        let mut init = option {
            name: b"userspec\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: USERSPEC as libc::c_int,
        };
        init
    },
    {
        let mut init = option {
            name: b"skip-chdir\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: SKIP_CHDIR as libc::c_int,
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
unsafe extern "C" fn parse_additional_groups(
    mut groups: *const libc::c_char,
    mut pgids: *mut *mut gid_t,
    mut pn_gids: *mut size_t,
    mut show_errors: bool,
) -> libc::c_int {
    let mut gids: *mut gid_t = 0 as *mut gid_t;
    let mut n_gids_allocated: size_t = 0 as libc::c_int as size_t;
    let mut n_gids: size_t = 0 as libc::c_int as size_t;
    let mut buffer: *mut libc::c_char = xstrdup(groups);
    let mut tmp: *const libc::c_char = 0 as *const libc::c_char;
    let mut ret: libc::c_int = 0 as libc::c_int;
    tmp = strtok(buffer, b",\0" as *const u8 as *const libc::c_char);
    while !tmp.is_null() {
        let mut g: *mut group = 0 as *mut group;
        let mut value: uintmax_t = 0;
        if xstrtoumax(
            tmp,
            0 as *mut *mut libc::c_char,
            10 as libc::c_int,
            &mut value,
            b"\0" as *const u8 as *const libc::c_char,
        ) as libc::c_uint == LONGINT_OK as libc::c_int as libc::c_uint
            && value
                <= (if (0 as libc::c_int as gid_t) < -(1 as libc::c_int) as gid_t {
                    -(1 as libc::c_int) as gid_t
                } else {
                    ((1 as libc::c_int as gid_t)
                        << (::core::mem::size_of::<gid_t>() as libc::c_ulong)
                            .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                            .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                        .wrapping_sub(1 as libc::c_int as libc::c_uint)
                        .wrapping_mul(2 as libc::c_int as libc::c_uint)
                        .wrapping_add(1 as libc::c_int as libc::c_uint)
                }) as libc::c_ulong
        {
            while *(*__ctype_b_loc()).offset(to_uchar(*tmp) as libc::c_int as isize)
                as libc::c_int & _ISspace as libc::c_int as libc::c_ushort as libc::c_int
                != 0
            {
                tmp = tmp.offset(1);
            }
            if *tmp as libc::c_int != '+' as i32 {
                g = getgrnam(tmp);
                if !g.is_null() {
                    value = (*g).gr_gid as uintmax_t;
                }
            }
            g = (0 as *mut libc::c_void).is_null() as libc::c_int as intptr_t
                as *mut group;
        } else {
            g = getgrnam(tmp);
            if !g.is_null() {
                value = (*g).gr_gid as uintmax_t;
            }
        }
        if g.is_null() {
            ret = -(1 as libc::c_int);
            if !show_errors {
                break;
            }
            error(
                0 as libc::c_int,
                *__errno_location(),
                dcgettext(
                    0 as *const libc::c_char,
                    b"invalid group %s\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
                quote(tmp),
            );
        } else {
            if n_gids == n_gids_allocated {
                gids = (if ::core::mem::size_of::<C2RustUnnamed_3>() as libc::c_ulong
                    != 0
                {
                    x2nrealloc(
                        gids as *mut libc::c_void,
                        &mut n_gids_allocated,
                        ::core::mem::size_of::<gid_t>() as libc::c_ulong,
                    )
                } else {
                    x2nrealloc(
                        gids as *mut libc::c_void,
                        &mut n_gids_allocated,
                        ::core::mem::size_of::<gid_t>() as libc::c_ulong,
                    )
                }) as *mut gid_t;
            }
            let fresh0 = n_gids;
            n_gids = n_gids.wrapping_add(1);
            *gids.offset(fresh0 as isize) = value as gid_t;
        }
        tmp = strtok(0 as *mut libc::c_char, b",\0" as *const u8 as *const libc::c_char);
    }
    if ret == 0 as libc::c_int && n_gids == 0 as libc::c_int as libc::c_ulong {
        if show_errors {
            error(
                0 as libc::c_int,
                0 as libc::c_int,
                dcgettext(
                    0 as *const libc::c_char,
                    b"invalid group list %s\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
                quote(groups),
            );
        }
        ret = -(1 as libc::c_int);
    }
    *pgids = gids;
    if ret == 0 as libc::c_int {
        *pn_gids = n_gids;
    }
    free(buffer as *mut libc::c_void);
    return ret;
}
unsafe extern "C" fn is_root(mut dir: *const libc::c_char) -> bool {
    let mut resolved: *mut libc::c_char = canonicalize_file_name(dir);
    let mut is_res_root: bool = !resolved.is_null()
        && strcmp(b"/\0" as *const u8 as *const libc::c_char, resolved)
            == 0 as libc::c_int;
    free(resolved as *mut libc::c_void);
    return is_res_root;
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
                b"Usage: %s [OPTION] NEWROOT [COMMAND [ARG]...]\n  or:  %s OPTION\n\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            program_name,
            program_name,
        );
        fputs_unlocked(
            dcgettext(
                0 as *const libc::c_char,
                b"Run COMMAND with root directory set to NEWROOT.\n\n\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
            stdout,
        );
        fputs_unlocked(
            dcgettext(
                0 as *const libc::c_char,
                b"      --groups=G_LIST        specify supplementary groups as g1,g2,..,gN\n\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            stdout,
        );
        fputs_unlocked(
            dcgettext(
                0 as *const libc::c_char,
                b"      --userspec=USER:GROUP  specify user and group (ID or name) to use\n\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            stdout,
        );
        printf(
            dcgettext(
                0 as *const libc::c_char,
                b"      --skip-chdir           do not change working directory to %s\n\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            quotearg_style(
                shell_escape_always_quoting_style,
                b"/\0" as *const u8 as *const libc::c_char,
            ),
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
                b"\nIf no command is given, run '\"$SHELL\" -i' (default: '/bin/sh -i').\n\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            stdout,
        );
        emit_exec_status(b"chroot\0" as *const u8 as *const libc::c_char);
        emit_ancillary_info(b"chroot\0" as *const u8 as *const libc::c_char);
    }
    exit(status);
}
unsafe fn main_0(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut c: libc::c_int = 0;
    let mut userspec: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut username: *const libc::c_char = 0 as *const libc::c_char;
    let mut groups: *const libc::c_char = 0 as *const libc::c_char;
    let mut skip_chdir: bool = 0 as libc::c_int != 0;
    let mut uid: uid_t = -(1 as libc::c_int) as uid_t;
    let mut gid: gid_t = -(1 as libc::c_int) as gid_t;
    let mut out_gids: *mut gid_t = 0 as *mut gid_t;
    let mut n_gids: size_t = 0 as libc::c_int as size_t;
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
            b"+\0" as *const u8 as *const libc::c_char,
            long_opts.as_ptr(),
            0 as *mut libc::c_int,
        );
        if !(c != -(1 as libc::c_int)) {
            break;
        }
        match c {
            257 => {
                userspec = optarg;
                let mut userlen: size_t = strlen(userspec);
                if userlen != 0
                    && *userspec
                        .offset(
                            userlen.wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                as isize,
                        ) as libc::c_int == ':' as i32
                {
                    *userspec
                        .offset(
                            userlen.wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                as isize,
                        ) = '\0' as i32 as libc::c_char;
                }
            }
            256 => {
                groups = optarg;
            }
            258 => {
                skip_chdir = 1 as libc::c_int != 0;
            }
            -2 => {
                usage(0 as libc::c_int);
            }
            -3 => {
                version_etc(
                    stdout,
                    b"chroot\0" as *const u8 as *const libc::c_char,
                    b"GNU coreutils\0" as *const u8 as *const libc::c_char,
                    Version,
                    b"Roland McGrath\0" as *const u8 as *const libc::c_char,
                    0 as *mut libc::c_void as *mut libc::c_char,
                );
                exit(0 as libc::c_int);
            }
            _ => {
                usage(EXIT_CANCELED as libc::c_int);
            }
        }
    }
    if argc <= optind {
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
    let mut newroot: *const libc::c_char = *argv.offset(optind as isize);
    let mut is_oldroot: bool = is_root(newroot);
    if !is_oldroot && skip_chdir as libc::c_int != 0 {
        error(
            0 as libc::c_int,
            0 as libc::c_int,
            dcgettext(
                0 as *const libc::c_char,
                b"option --skip-chdir only permitted if NEWROOT is old %s\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
            quotearg_style(
                shell_escape_always_quoting_style,
                b"/\0" as *const u8 as *const libc::c_char,
            ),
        );
        usage(EXIT_CANCELED as libc::c_int);
    }
    if !is_oldroot {
        if !userspec.is_null() {
            parse_user_spec(
                userspec,
                &mut uid,
                &mut gid,
                0 as *mut *mut libc::c_char,
                0 as *mut *mut libc::c_char,
            );
        }
        if !uid_unset(uid) && (groups.is_null() || gid_unset(gid) as libc::c_int != 0) {
            let mut pwd: *const passwd = 0 as *const passwd;
            pwd = getpwuid(uid);
            if !pwd.is_null() {
                if gid_unset(gid) {
                    gid = (*pwd).pw_gid;
                }
                username = (*pwd).pw_name;
            }
        }
        if !groups.is_null() && *groups as libc::c_int != 0 {
            parse_additional_groups(
                groups,
                &mut out_gids,
                &mut n_gids,
                0 as libc::c_int != 0,
            );
        } else if groups.is_null() && !gid_unset(gid) && !username.is_null() {
            let mut ngroups: libc::c_int = xgetgroups(username, gid, &mut out_gids);
            if (0 as libc::c_int) < ngroups {
                n_gids = ngroups as size_t;
            }
        }
    }
    if chroot(newroot) != 0 as libc::c_int {
        if ::core::mem::size_of::<C2RustUnnamed_10>() as libc::c_ulong != 0 {
            error(
                EXIT_CANCELED as libc::c_int,
                *__errno_location(),
                dcgettext(
                    0 as *const libc::c_char,
                    b"cannot change root directory to %s\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
                quotearg_style(shell_escape_always_quoting_style, newroot),
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
                    b"cannot change root directory to %s\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
                quotearg_style(shell_escape_always_quoting_style, newroot),
            );
            if 0 as libc::c_int != 0 {} else {
                unreachable!();
            };
        };
    }
    if !skip_chdir && chdir(b"/\0" as *const u8 as *const libc::c_char) != 0 {
        if ::core::mem::size_of::<C2RustUnnamed_9>() as libc::c_ulong != 0 {
            error(
                EXIT_CANCELED as libc::c_int,
                *__errno_location(),
                dcgettext(
                    0 as *const libc::c_char,
                    b"cannot chdir to root directory\0" as *const u8
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
                    b"cannot chdir to root directory\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
            );
            if 0 as libc::c_int != 0 {} else {
                unreachable!();
            };
        };
    }
    if argc == optind + 1 as libc::c_int {
        let mut shell: *mut libc::c_char = getenv(
            b"SHELL\0" as *const u8 as *const libc::c_char,
        );
        if shell.is_null() {
            shell = bad_cast(b"/bin/sh\0" as *const u8 as *const libc::c_char);
        }
        let ref mut fresh1 = *argv.offset(0 as libc::c_int as isize);
        *fresh1 = shell;
        let ref mut fresh2 = *argv.offset(1 as libc::c_int as isize);
        *fresh2 = bad_cast(b"-i\0" as *const u8 as *const libc::c_char);
        let ref mut fresh3 = *argv.offset(2 as libc::c_int as isize);
        *fresh3 = 0 as *mut libc::c_char;
    } else {
        argv = argv.offset((optind + 1 as libc::c_int) as isize);
    }
    if !userspec.is_null() {
        let mut warn: bool = false;
        let mut err: *const libc::c_char = parse_user_spec_warn(
            userspec,
            &mut uid,
            &mut gid,
            0 as *mut *mut libc::c_char,
            0 as *mut *mut libc::c_char,
            &mut warn,
        );
        if !err.is_null() {
            error(
                if warn as libc::c_int != 0 {
                    0 as libc::c_int
                } else {
                    EXIT_CANCELED as libc::c_int
                },
                0 as libc::c_int,
                b"%s\0" as *const u8 as *const libc::c_char,
                err,
            );
        }
    }
    if !uid_unset(uid) && (groups.is_null() || gid_unset(gid) as libc::c_int != 0) {
        let mut pwd_0: *const passwd = 0 as *const passwd;
        pwd_0 = getpwuid(uid);
        if !pwd_0.is_null() {
            if gid_unset(gid) {
                gid = (*pwd_0).pw_gid;
            }
            username = (*pwd_0).pw_name;
        } else if gid_unset(gid) {
            if ::core::mem::size_of::<C2RustUnnamed_8>() as libc::c_ulong != 0 {
                error(
                    EXIT_CANCELED as libc::c_int,
                    *__errno_location(),
                    dcgettext(
                        0 as *const libc::c_char,
                        b"no group specified for unknown uid: %d\0" as *const u8
                            as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    uid as libc::c_int,
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
                        b"no group specified for unknown uid: %d\0" as *const u8
                            as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    uid as libc::c_int,
                );
                if 0 as libc::c_int != 0 {} else {
                    unreachable!();
                };
            };
        }
    }
    let mut gids: *mut gid_t = out_gids;
    let mut in_gids: *mut gid_t = 0 as *mut gid_t;
    if !groups.is_null() && *groups as libc::c_int != 0 {
        if parse_additional_groups(groups, &mut in_gids, &mut n_gids, n_gids == 0)
            != 0 as libc::c_int
        {
            if n_gids == 0 {
                return EXIT_CANCELED as libc::c_int;
            }
        } else {
            gids = in_gids;
        }
    } else if groups.is_null() && !gid_unset(gid) && !username.is_null() {
        let mut ngroups_0: libc::c_int = xgetgroups(username, gid, &mut in_gids);
        if ngroups_0 <= 0 as libc::c_int {
            if n_gids == 0 {
                if ::core::mem::size_of::<C2RustUnnamed_7>() as libc::c_ulong != 0 {
                    error(
                        EXIT_CANCELED as libc::c_int,
                        *__errno_location(),
                        dcgettext(
                            0 as *const libc::c_char,
                            b"failed to get supplemental groups\0" as *const u8
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
                            b"failed to get supplemental groups\0" as *const u8
                                as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                    );
                    if 0 as libc::c_int != 0 {} else {
                        unreachable!();
                    };
                };
            }
        } else {
            n_gids = ngroups_0 as size_t;
            gids = in_gids;
        }
    }
    if (!uid_unset(uid) || !groups.is_null())
        && setgroups(n_gids, gids) != 0 as libc::c_int
    {
        if ::core::mem::size_of::<C2RustUnnamed_6>() as libc::c_ulong != 0 {
            error(
                EXIT_CANCELED as libc::c_int,
                *__errno_location(),
                dcgettext(
                    0 as *const libc::c_char,
                    b"failed to set supplemental groups\0" as *const u8
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
                    b"failed to set supplemental groups\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
            );
            if 0 as libc::c_int != 0 {} else {
                unreachable!();
            };
        };
    }
    free(in_gids as *mut libc::c_void);
    free(out_gids as *mut libc::c_void);
    if !gid_unset(gid) && setgid(gid) != 0 {
        if ::core::mem::size_of::<C2RustUnnamed_5>() as libc::c_ulong != 0 {
            error(
                EXIT_CANCELED as libc::c_int,
                *__errno_location(),
                dcgettext(
                    0 as *const libc::c_char,
                    b"failed to set group-ID\0" as *const u8 as *const libc::c_char,
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
                    b"failed to set group-ID\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
            );
            if 0 as libc::c_int != 0 {} else {
                unreachable!();
            };
        };
    }
    if !uid_unset(uid) && setuid(uid) != 0 {
        if ::core::mem::size_of::<C2RustUnnamed_4>() as libc::c_ulong != 0 {
            error(
                EXIT_CANCELED as libc::c_int,
                *__errno_location(),
                dcgettext(
                    0 as *const libc::c_char,
                    b"failed to set user-ID\0" as *const u8 as *const libc::c_char,
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
                    b"failed to set user-ID\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
            );
            if 0 as libc::c_int != 0 {} else {
                unreachable!();
            };
        };
    }
    execvp(*argv.offset(0 as libc::c_int as isize), argv as *const *mut libc::c_char);
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
