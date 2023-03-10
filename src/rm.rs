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
    fn getopt_long(
        ___argc: libc::c_int,
        ___argv: *const *mut libc::c_char,
        __shortopts: *const libc::c_char,
        __longopts: *const option,
        __longind: *mut libc::c_int,
    ) -> libc::c_int;
    static mut optarg: *mut libc::c_char;
    static mut optind: libc::c_int;
    fn lstat(__file: *const libc::c_char, __buf: *mut stat) -> libc::c_int;
    fn isatty(__fd: libc::c_int) -> libc::c_int;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strncmp(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn __errno_location() -> *mut libc::c_int;
    fn quotearg_style(s: quoting_style, arg: *const libc::c_char) -> *mut libc::c_char;
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
    fn dcngettext(
        __domainname: *const libc::c_char,
        __msgid1: *const libc::c_char,
        __msgid2: *const libc::c_char,
        __n: libc::c_ulong,
        __category: libc::c_int,
    ) -> *mut libc::c_char;
    fn textdomain(__domainname: *const libc::c_char) -> *mut libc::c_char;
    fn bindtextdomain(
        __domainname: *const libc::c_char,
        __dirname: *const libc::c_char,
    ) -> *mut libc::c_char;
    fn close_stdin();
    fn atexit(__func: Option::<unsafe extern "C" fn() -> ()>) -> libc::c_int;
    fn exit(_: libc::c_int) -> !;
    static mut argmatch_die: argmatch_exit_fn;
    fn __xargmatch_internal(
        context: *const libc::c_char,
        arg: *const libc::c_char,
        arglist: *const *const libc::c_char,
        vallist: *const libc::c_void,
        valsize: size_t,
        exit_fn: argmatch_exit_fn,
        allow_abbreviation: bool,
    ) -> ptrdiff_t;
    fn error(
        __status: libc::c_int,
        __errnum: libc::c_int,
        __format: *const libc::c_char,
        _: ...
    );
    fn rm(file: *const *mut libc::c_char, x: *const rm_options) -> RM_status;
    fn get_root_dev_ino(root_d_i: *mut dev_ino) -> *mut dev_ino;
    fn yesno() -> bool;
}
pub type size_t = libc::c_ulong;
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
pub type ptrdiff_t = libc::c_long;
pub type ino_t = __ino_t;
pub type dev_t = __dev_t;
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
pub type uintmax_t = __uintmax_t;
pub const PLURAL_REDUCER: C2RustUnnamed = 1000000;
pub type C2RustUnnamed = libc::c_uint;
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
pub type argmatch_exit_fn = Option::<unsafe extern "C" fn() -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dev_ino {
    pub st_ino: ino_t,
    pub st_dev: dev_t,
}
pub type rm_interactive = libc::c_uint;
pub const RMI_NEVER: rm_interactive = 5;
pub const RMI_SOMETIMES: rm_interactive = 4;
pub const RMI_ALWAYS: rm_interactive = 3;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct rm_options {
    pub ignore_missing_files: bool,
    pub interactive: rm_interactive,
    pub one_file_system: bool,
    pub recursive: bool,
    pub remove_empty_directories: bool,
    pub root_dev_ino: *mut dev_ino,
    pub preserve_all_root: bool,
    pub stdin_tty: bool,
    pub verbose: bool,
    pub require_restore_cwd: bool,
}
pub type RM_status = libc::c_uint;
pub const RM_NONEMPTY_DIR: RM_status = 6;
pub const RM_ERROR: RM_status = 5;
pub const RM_USER_DECLINED: RM_status = 4;
pub const RM_USER_ACCEPTED: RM_status = 3;
pub const RM_OK: RM_status = 2;
pub type C2RustUnnamed_1 = libc::c_uint;
pub const PRESUME_INPUT_TTY_OPTION: C2RustUnnamed_1 = 260;
pub const PRESERVE_ROOT: C2RustUnnamed_1 = 259;
pub const NO_PRESERVE_ROOT: C2RustUnnamed_1 = 258;
pub const ONE_FILE_SYSTEM: C2RustUnnamed_1 = 257;
pub const INTERACTIVE_OPTION: C2RustUnnamed_1 = 256;
pub type interactive_type = libc::c_uint;
pub const interactive_always: interactive_type = 2;
pub const interactive_once: interactive_type = 1;
pub const interactive_never: interactive_type = 0;
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
unsafe extern "C" fn select_plural(mut n: uintmax_t) -> libc::c_ulong {
    return if n
        <= (9223372036854775807 as libc::c_long as libc::c_ulong)
            .wrapping_mul(2 as libc::c_ulong)
            .wrapping_add(1 as libc::c_ulong)
    {
        n
    } else {
        n.wrapping_rem(PLURAL_REDUCER as libc::c_int as libc::c_ulong)
            .wrapping_add(PLURAL_REDUCER as libc::c_int as libc::c_ulong)
    };
}
#[inline]
unsafe extern "C" fn priv_set_remove_linkdir() -> libc::c_int {
    return -(1 as libc::c_int);
}
static mut long_opts: [option; 12] = [
    {
        let mut init = option {
            name: b"force\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'f' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"interactive\0" as *const u8 as *const libc::c_char,
            has_arg: 2 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: INTERACTIVE_OPTION as libc::c_int,
        };
        init
    },
    {
        let mut init = option {
            name: b"one-file-system\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: ONE_FILE_SYSTEM as libc::c_int,
        };
        init
    },
    {
        let mut init = option {
            name: b"no-preserve-root\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: NO_PRESERVE_ROOT as libc::c_int,
        };
        init
    },
    {
        let mut init = option {
            name: b"preserve-root\0" as *const u8 as *const libc::c_char,
            has_arg: 2 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: PRESERVE_ROOT as libc::c_int,
        };
        init
    },
    {
        let mut init = option {
            name: b"-presume-input-tty\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: PRESUME_INPUT_TTY_OPTION as libc::c_int,
        };
        init
    },
    {
        let mut init = option {
            name: b"recursive\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'r' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"dir\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'd' as i32,
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
static mut interactive_args: [*const libc::c_char; 7] = [
    b"never\0" as *const u8 as *const libc::c_char,
    b"no\0" as *const u8 as *const libc::c_char,
    b"none\0" as *const u8 as *const libc::c_char,
    b"once\0" as *const u8 as *const libc::c_char,
    b"always\0" as *const u8 as *const libc::c_char,
    b"yes\0" as *const u8 as *const libc::c_char,
    0 as *const libc::c_char,
];
static mut interactive_types: [interactive_type; 6] = [
    interactive_never,
    interactive_never,
    interactive_never,
    interactive_once,
    interactive_always,
    interactive_always,
];
unsafe extern "C" fn diagnose_leading_hyphen(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) {
    let mut i: libc::c_int = 1 as libc::c_int;
    while i < argc {
        let mut arg: *const libc::c_char = *argv.offset(i as isize);
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
        if *arg.offset(0 as libc::c_int as isize) as libc::c_int == '-' as i32
            && *arg.offset(1 as libc::c_int as isize) as libc::c_int != 0
            && lstat(arg, &mut st) == 0 as libc::c_int
        {
            fprintf(
                stderr,
                dcgettext(
                    0 as *const libc::c_char,
                    b"Try '%s ./%s' to remove the file %s.\n\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
                *argv.offset(0 as libc::c_int as isize),
                quotearg_n_style(1 as libc::c_int, shell_escape_quoting_style, arg),
                quotearg_style(shell_escape_always_quoting_style, arg),
            );
            break;
        } else {
            i += 1;
        }
    }
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
                b"Usage: %s [OPTION]... [FILE]...\n\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
            program_name,
        );
        fputs_unlocked(
            dcgettext(
                0 as *const libc::c_char,
                b"Remove (unlink) the FILE(s).\n\n  -f, --force           ignore nonexistent files and arguments, never prompt\n  -i                    prompt before every removal\n\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            stdout,
        );
        fputs_unlocked(
            dcgettext(
                0 as *const libc::c_char,
                b"  -I                    prompt once before removing more than three files, or\n                          when removing recursively; less intrusive than -i,\n                          while still giving protection against most mistakes\n      --interactive[=WHEN]  prompt according to WHEN: never, once (-I), or\n                          always (-i); without WHEN, prompt always\n\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            stdout,
        );
        fputs_unlocked(
            dcgettext(
                0 as *const libc::c_char,
                b"      --one-file-system  when removing a hierarchy recursively, skip any\n                          directory that is on a file system different from\n                          that of the corresponding command line argument\n\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            stdout,
        );
        fputs_unlocked(
            dcgettext(
                0 as *const libc::c_char,
                b"      --no-preserve-root  do not treat '/' specially\n      --preserve-root[=all]  do not remove '/' (default);\n                              with 'all', reject any command line argument\n                              on a separate device from its parent\n\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            stdout,
        );
        fputs_unlocked(
            dcgettext(
                0 as *const libc::c_char,
                b"  -r, -R, --recursive   remove directories and their contents recursively\n  -d, --dir             remove empty directories\n  -v, --verbose         explain what is being done\n\0"
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
                b"\nBy default, rm does not remove directories.  Use the --recursive (-r or -R)\noption to remove each listed directory, too, along with all of its contents.\n\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            stdout,
        );
        printf(
            dcgettext(
                0 as *const libc::c_char,
                b"\nTo remove a file whose name starts with a '-', for example '-foo',\nuse one of these commands:\n  %s -- -foo\n\n  %s ./-foo\n\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            program_name,
            program_name,
        );
        fputs_unlocked(
            dcgettext(
                0 as *const libc::c_char,
                b"\nNote that if you use rm to remove a file, it might be possible to recover\nsome of its contents, given sufficient expertise and/or time.  For greater\nassurance that the contents are truly unrecoverable, consider using shred(1).\n\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            stdout,
        );
        emit_ancillary_info(b"rm\0" as *const u8 as *const libc::c_char);
    }
    exit(status);
}
unsafe extern "C" fn rm_option_init(mut x: *mut rm_options) {
    (*x).ignore_missing_files = 0 as libc::c_int != 0;
    (*x).interactive = RMI_SOMETIMES;
    (*x).one_file_system = 0 as libc::c_int != 0;
    (*x).remove_empty_directories = 0 as libc::c_int != 0;
    (*x).recursive = 0 as libc::c_int != 0;
    (*x).root_dev_ino = 0 as *mut dev_ino;
    (*x).preserve_all_root = 0 as libc::c_int != 0;
    (*x).stdin_tty = isatty(0 as libc::c_int) != 0;
    (*x).verbose = 0 as libc::c_int != 0;
    (*x).require_restore_cwd = 0 as libc::c_int != 0;
}
unsafe fn main_0(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut preserve_root: bool = 1 as libc::c_int != 0;
    let mut x: rm_options = rm_options {
        ignore_missing_files: false,
        interactive: 0 as rm_interactive,
        one_file_system: false,
        recursive: false,
        remove_empty_directories: false,
        root_dev_ino: 0 as *mut dev_ino,
        preserve_all_root: false,
        stdin_tty: false,
        verbose: false,
        require_restore_cwd: false,
    };
    let mut prompt_once: bool = 0 as libc::c_int != 0;
    let mut c: libc::c_int = 0;
    set_program_name(*argv.offset(0 as libc::c_int as isize));
    setlocale(6 as libc::c_int, b"\0" as *const u8 as *const libc::c_char);
    bindtextdomain(
        b"coreutils\0" as *const u8 as *const libc::c_char,
        b"/usr/local/share/locale\0" as *const u8 as *const libc::c_char,
    );
    textdomain(b"coreutils\0" as *const u8 as *const libc::c_char);
    atexit(Some(close_stdin as unsafe extern "C" fn() -> ()));
    rm_option_init(&mut x);
    priv_set_remove_linkdir();
    loop {
        c = getopt_long(
            argc,
            argv,
            b"dfirvIR\0" as *const u8 as *const libc::c_char,
            long_opts.as_ptr(),
            0 as *mut libc::c_int,
        );
        if !(c != -(1 as libc::c_int)) {
            break;
        }
        match c {
            100 => {
                x.remove_empty_directories = 1 as libc::c_int != 0;
            }
            102 => {
                x.interactive = RMI_NEVER;
                x.ignore_missing_files = 1 as libc::c_int != 0;
                prompt_once = 0 as libc::c_int != 0;
            }
            105 => {
                x.interactive = RMI_ALWAYS;
                x.ignore_missing_files = 0 as libc::c_int != 0;
                prompt_once = 0 as libc::c_int != 0;
            }
            73 => {
                x.interactive = RMI_SOMETIMES;
                x.ignore_missing_files = 0 as libc::c_int != 0;
                prompt_once = 1 as libc::c_int != 0;
            }
            114 | 82 => {
                x.recursive = 1 as libc::c_int != 0;
            }
            256 => {
                let mut i: libc::c_int = 0;
                if !optarg.is_null() {
                    i = interactive_types[__xargmatch_internal(
                        b"--interactive\0" as *const u8 as *const libc::c_char,
                        optarg,
                        interactive_args.as_ptr(),
                        interactive_types.as_ptr() as *const libc::c_void,
                        ::core::mem::size_of::<interactive_type>() as libc::c_ulong,
                        argmatch_die,
                        1 as libc::c_int != 0,
                    ) as usize] as libc::c_int;
                } else {
                    i = interactive_always as libc::c_int;
                }
                match i {
                    0 => {
                        x.interactive = RMI_NEVER;
                        prompt_once = 0 as libc::c_int != 0;
                    }
                    1 => {
                        x.interactive = RMI_SOMETIMES;
                        x.ignore_missing_files = 0 as libc::c_int != 0;
                        prompt_once = 1 as libc::c_int != 0;
                    }
                    2 => {
                        x.interactive = RMI_ALWAYS;
                        x.ignore_missing_files = 0 as libc::c_int != 0;
                        prompt_once = 0 as libc::c_int != 0;
                    }
                    _ => {}
                }
            }
            257 => {
                x.one_file_system = 1 as libc::c_int != 0;
            }
            258 => {
                if !(strcmp(
                    *argv.offset((optind - 1 as libc::c_int) as isize),
                    b"--no-preserve-root\0" as *const u8 as *const libc::c_char,
                ) == 0 as libc::c_int)
                {
                    if ::core::mem::size_of::<C2RustUnnamed_4>() as libc::c_ulong != 0 {
                        error(
                            1 as libc::c_int,
                            0 as libc::c_int,
                            dcgettext(
                                0 as *const libc::c_char,
                                b"you may not abbreviate the --no-preserve-root option\0"
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
                                b"you may not abbreviate the --no-preserve-root option\0"
                                    as *const u8 as *const libc::c_char,
                                5 as libc::c_int,
                            ),
                        );
                        if 0 as libc::c_int != 0 {} else {
                            unreachable!();
                        };
                    };
                }
                preserve_root = 0 as libc::c_int != 0;
            }
            259 => {
                if !optarg.is_null() {
                    if strcmp(optarg, b"all\0" as *const u8 as *const libc::c_char)
                        == 0 as libc::c_int
                    {
                        x.preserve_all_root = 1 as libc::c_int != 0;
                    } else {
                        if ::core::mem::size_of::<C2RustUnnamed_3>() as libc::c_ulong
                            != 0
                        {
                            error(
                                1 as libc::c_int,
                                0 as libc::c_int,
                                dcgettext(
                                    0 as *const libc::c_char,
                                    b"unrecognized --preserve-root argument: %s\0" as *const u8
                                        as *const libc::c_char,
                                    5 as libc::c_int,
                                ),
                                quotearg_style(shell_escape_always_quoting_style, optarg),
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
                                    b"unrecognized --preserve-root argument: %s\0" as *const u8
                                        as *const libc::c_char,
                                    5 as libc::c_int,
                                ),
                                quotearg_style(shell_escape_always_quoting_style, optarg),
                            );
                            if 0 as libc::c_int != 0 {} else {
                                unreachable!();
                            };
                        };
                    }
                }
                preserve_root = 1 as libc::c_int != 0;
            }
            260 => {
                x.stdin_tty = 1 as libc::c_int != 0;
            }
            118 => {
                x.verbose = 1 as libc::c_int != 0;
            }
            -2 => {
                usage(0 as libc::c_int);
            }
            -3 => {
                version_etc(
                    stdout,
                    b"rm\0" as *const u8 as *const libc::c_char,
                    b"GNU coreutils\0" as *const u8 as *const libc::c_char,
                    Version,
                    b"Paul Rubin\0" as *const u8 as *const libc::c_char,
                    b"David MacKenzie\0" as *const u8 as *const libc::c_char,
                    b"Richard M. Stallman\0" as *const u8 as *const libc::c_char,
                    b"Jim Meyering\0" as *const u8 as *const libc::c_char,
                    0 as *mut libc::c_void as *mut libc::c_char,
                );
                exit(0 as libc::c_int);
            }
            _ => {
                diagnose_leading_hyphen(argc, argv);
                usage(1 as libc::c_int);
            }
        }
    }
    if argc <= optind {
        if x.ignore_missing_files {
            return 0 as libc::c_int
        } else {
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
    }
    if x.recursive as libc::c_int != 0 && preserve_root as libc::c_int != 0 {
        static mut dev_ino_buf: dev_ino = dev_ino { st_ino: 0, st_dev: 0 };
        x.root_dev_ino = get_root_dev_ino(&mut dev_ino_buf);
        if (x.root_dev_ino).is_null() {
            if ::core::mem::size_of::<C2RustUnnamed_2>() as libc::c_ulong != 0 {
                error(
                    1 as libc::c_int,
                    *__errno_location(),
                    dcgettext(
                        0 as *const libc::c_char,
                        b"failed to get attributes of %s\0" as *const u8
                            as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    quotearg_style(
                        shell_escape_always_quoting_style,
                        b"/\0" as *const u8 as *const libc::c_char,
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
                        b"failed to get attributes of %s\0" as *const u8
                            as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    quotearg_style(
                        shell_escape_always_quoting_style,
                        b"/\0" as *const u8 as *const libc::c_char,
                    ),
                );
                if 0 as libc::c_int != 0 {} else {
                    unreachable!();
                };
            };
        }
    }
    let mut n_files: uintmax_t = (argc - optind) as uintmax_t;
    let mut file: *mut *mut libc::c_char = argv.offset(optind as isize);
    if prompt_once as libc::c_int != 0
        && (x.recursive as libc::c_int != 0
            || (3 as libc::c_int as libc::c_ulong) < n_files)
    {
        fprintf(
            stderr,
            if x.recursive as libc::c_int != 0 {
                dcngettext(
                    0 as *const libc::c_char,
                    b"%s: remove %lu argument recursively? \0" as *const u8
                        as *const libc::c_char,
                    b"%s: remove %lu arguments recursively? \0" as *const u8
                        as *const libc::c_char,
                    select_plural(n_files),
                    5 as libc::c_int,
                )
            } else {
                dcngettext(
                    0 as *const libc::c_char,
                    b"%s: remove %lu argument? \0" as *const u8 as *const libc::c_char,
                    b"%s: remove %lu arguments? \0" as *const u8 as *const libc::c_char,
                    select_plural(n_files),
                    5 as libc::c_int,
                )
            },
            program_name,
            n_files,
        );
        if !yesno() {
            return 0 as libc::c_int;
        }
    }
    let mut status: RM_status = rm(file, &mut x);
    if status as libc::c_uint == RM_OK as libc::c_int as libc::c_uint
        || status as libc::c_uint == RM_USER_ACCEPTED as libc::c_int as libc::c_uint
        || status as libc::c_uint == RM_USER_DECLINED as libc::c_int as libc::c_uint
        || status as libc::c_uint == RM_ERROR as libc::c_int as libc::c_uint
    {} else {
        __assert_fail(
            b"VALID_STATUS (status)\0" as *const u8 as *const libc::c_char,
            b"src/rm.c\0" as *const u8 as *const libc::c_char,
            371 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 23],
                &[libc::c_char; 23],
            >(b"int main(int, char **)\0"))
                .as_ptr(),
        );
    }
    return if status as libc::c_uint == RM_ERROR as libc::c_int as libc::c_uint {
        1 as libc::c_int
    } else {
        0 as libc::c_int
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
