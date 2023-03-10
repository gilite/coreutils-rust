#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![feature(extern_types, label_break_value)]
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type randread_source;
    pub type randint_source;
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
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
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn fputs_unlocked(__s: *const libc::c_char, __stream: *mut FILE) -> libc::c_int;
    fn ioctl(__fd: libc::c_int, __request: libc::c_ulong, _: ...) -> libc::c_int;
    fn lseek(__fd: libc::c_int, __offset: __off_t, __whence: libc::c_int) -> __off_t;
    fn close(__fd: libc::c_int) -> libc::c_int;
    fn write(__fd: libc::c_int, __buf: *const libc::c_void, __n: size_t) -> ssize_t;
    fn isatty(__fd: libc::c_int) -> libc::c_int;
    fn unlink(__name: *const libc::c_char) -> libc::c_int;
    fn fsync(__fd: libc::c_int) -> libc::c_int;
    fn sync();
    fn getpagesize() -> libc::c_int;
    fn chmod(__file: *const libc::c_char, __mode: __mode_t) -> libc::c_int;
    fn fstat(__fd: libc::c_int, __buf: *mut stat) -> libc::c_int;
    fn ftruncate(__fd: libc::c_int, __length: __off_t) -> libc::c_int;
    fn fdatasync(__fildes: libc::c_int) -> libc::c_int;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strncmp(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn free(_: *mut libc::c_void);
    fn __errno_location() -> *mut libc::c_int;
    fn atexit(__func: Option::<unsafe extern "C" fn() -> ()>) -> libc::c_int;
    fn exit(_: libc::c_int) -> !;
    static mut Version: *const libc::c_char;
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
    fn xstrdup(str: *const libc::c_char) -> *mut libc::c_char;
    fn xnmalloc(n: size_t, s: size_t) -> *mut libc::c_void;
    fn last_component(filename: *const libc::c_char) -> *mut libc::c_char;
    fn base_len(filename: *const libc::c_char) -> size_t;
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
    fn umaxtostr(_: uintmax_t, _: *mut libc::c_char) -> *mut libc::c_char;
    fn quotearg_n_style_colon(
        n: libc::c_int,
        s: quoting_style,
        arg: *const libc::c_char,
    ) -> *mut libc::c_char;
    fn time(__timer: *mut time_t) -> time_t;
    fn xalignalloc(_: idx_t, _: idx_t) -> *mut libc::c_void;
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
    fn xdectoumax(
        n_str: *const libc::c_char,
        min: uintmax_t,
        max: uintmax_t,
        suffixes: *const libc::c_char,
        err: *const libc::c_char,
        err_exit: libc::c_int,
    ) -> uintmax_t;
    fn xnumtoumax(
        n_str: *const libc::c_char,
        base: libc::c_int,
        min: uintmax_t,
        max: uintmax_t,
        suffixes: *const libc::c_char,
        err: *const libc::c_char,
        err_exit: libc::c_int,
    ) -> uintmax_t;
    fn error(
        __status: libc::c_int,
        __errnum: libc::c_int,
        __format: *const libc::c_char,
        _: ...
    );
    fn open_safer(_: *const libc::c_char, _: libc::c_int, _: ...) -> libc::c_int;
    fn human_readable(
        _: uintmax_t,
        _: *mut libc::c_char,
        _: libc::c_int,
        _: uintmax_t,
        _: uintmax_t,
    ) -> *mut libc::c_char;
    fn randint_all_new(_: *const libc::c_char, _: size_t) -> *mut randint_source;
    fn randread(_: *mut randread_source, _: *mut libc::c_void, _: size_t);
    fn randint_get_source(_: *const randint_source) -> *mut randread_source;
    fn randint_all_free(_: *mut randint_source) -> libc::c_int;
    fn randint_genmax(_: *mut randint_source, genmax: randint) -> randint;
    fn renameatu(
        _: libc::c_int,
        _: *const libc::c_char,
        _: libc::c_int,
        _: *const libc::c_char,
        _: libc::c_uint,
    ) -> libc::c_int;
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
pub type __ssize_t = libc::c_long;
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
pub type off_t = __off_t;
pub type ssize_t = __ssize_t;
pub type ptrdiff_t = libc::c_long;
pub type time_t = __time_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mtop {
    pub mt_op: libc::c_short,
    pub mt_count: libc::c_int,
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
pub type idx_t = ptrdiff_t;
pub type C2RustUnnamed = libc::c_int;
pub const GETOPT_VERSION_CHAR: C2RustUnnamed = -3;
pub const GETOPT_HELP_CHAR: C2RustUnnamed = -2;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct infomap {
    pub program: *const libc::c_char,
    pub node: *const libc::c_char,
}
pub const DEFAULT_PASSES: C2RustUnnamed_1 = 3;
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
pub type C2RustUnnamed_0 = libc::c_uint;
pub const human_B: C2RustUnnamed_0 = 256;
pub const human_SI: C2RustUnnamed_0 = 128;
pub const human_space_before_unit: C2RustUnnamed_0 = 64;
pub const human_base_1024: C2RustUnnamed_0 = 32;
pub const human_autoscale: C2RustUnnamed_0 = 16;
pub const human_suppress_point_zero: C2RustUnnamed_0 = 8;
pub const human_group_digits: C2RustUnnamed_0 = 4;
pub const human_floor: C2RustUnnamed_0 = 2;
pub const human_round_to_nearest: C2RustUnnamed_0 = 1;
pub const human_ceiling: C2RustUnnamed_0 = 0;
pub type randint = uintmax_t;
pub type C2RustUnnamed_1 = libc::c_uint;
pub type C2RustUnnamed_2 = libc::c_uint;
pub const VERBOSE_UPDATE: C2RustUnnamed_2 = 5;
pub type C2RustUnnamed_3 = libc::c_uint;
pub const SECTOR_SIZE: C2RustUnnamed_3 = 512;
pub type C2RustUnnamed_4 = libc::c_uint;
pub const SECTOR_MASK: C2RustUnnamed_4 = 511;
pub type remove_method = libc::c_uint;
pub const remove_wipesync: remove_method = 3;
pub const remove_wipe: remove_method = 2;
pub const remove_unlink: remove_method = 1;
pub const remove_none: remove_method = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Options {
    pub force: bool,
    pub n_iterations: size_t,
    pub size: off_t,
    pub remove_file: remove_method,
    pub verbose: bool,
    pub exact: bool,
    pub zero_fill: bool,
}
pub type C2RustUnnamed_5 = libc::c_uint;
pub const RANDOM_SOURCE_OPTION: C2RustUnnamed_5 = 256;
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
unsafe extern "C" fn alignfree(mut ptr: *mut libc::c_void) {
    free(ptr);
}
#[inline]
unsafe extern "C" fn randint_choose(
    mut s: *mut randint_source,
    mut choices: randint,
) -> randint {
    return randint_genmax(s, choices.wrapping_sub(1 as libc::c_int as libc::c_ulong));
}
static mut remove_args: [*const libc::c_char; 4] = [
    b"unlink\0" as *const u8 as *const libc::c_char,
    b"wipe\0" as *const u8 as *const libc::c_char,
    b"wipesync\0" as *const u8 as *const libc::c_char,
    0 as *const libc::c_char,
];
static mut remove_methods: [remove_method; 3] = [
    remove_unlink,
    remove_wipe,
    remove_wipesync,
];
static mut long_opts: [option; 11] = [
    {
        let mut init = option {
            name: b"exact\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'x' as i32,
        };
        init
    },
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
            name: b"iterations\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'n' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"size\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 's' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"random-source\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: RANDOM_SOURCE_OPTION as libc::c_int,
        };
        init
    },
    {
        let mut init = option {
            name: b"remove\0" as *const u8 as *const libc::c_char,
            has_arg: 2 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'u' as i32,
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
                b"Usage: %s [OPTION]... FILE...\n\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            program_name,
        );
        fputs_unlocked(
            dcgettext(
                0 as *const libc::c_char,
                b"Overwrite the specified FILE(s) repeatedly, in order to make it harder\nfor even very expensive hardware probing to recover the data.\n\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            stdout,
        );
        fputs_unlocked(
            dcgettext(
                0 as *const libc::c_char,
                b"\nIf FILE is -, shred standard output.\n\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
            stdout,
        );
        emit_mandatory_arg_note();
        printf(
            dcgettext(
                0 as *const libc::c_char,
                b"  -f, --force    change permissions to allow writing if necessary\n  -n, --iterations=N  overwrite N times instead of the default (%d)\n      --random-source=FILE  get random bytes from FILE\n  -s, --size=N   shred this many bytes (suffixes like K, M, G accepted)\n\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            DEFAULT_PASSES as libc::c_int,
        );
        fputs_unlocked(
            dcgettext(
                0 as *const libc::c_char,
                b"  -u             deallocate and remove file after overwriting\n      --remove[=HOW]  like -u but give control on HOW to delete;  See below\n  -v, --verbose  show progress\n  -x, --exact    do not round file sizes up to the next full block;\n                   this is the default for non-regular files\n  -z, --zero     add a final overwrite with zeros to hide shredding\n\0"
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
                b"\nDelete FILE(s) if --remove (-u) is specified.  The default is not to remove\nthe files because it is common to operate on device files like /dev/hda,\nand those files usually should not be removed.\nThe optional HOW parameter indicates how to remove a directory entry:\n'unlink' => use a standard unlink call.\n'wipe' => also first obfuscate bytes in the name.\n'wipesync' => also sync each obfuscated byte to the device.\nThe default mode is 'wipesync', but note it can be expensive.\n\n\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            stdout,
        );
        fputs_unlocked(
            dcgettext(
                0 as *const libc::c_char,
                b"CAUTION: shred assumes the file system and hardware overwrite data in place.\nAlthough this is common, many platforms operate otherwise.  Also, backups\nand mirrors may contain unremovable copies that will let a shredded file\nbe recovered later.  See the GNU coreutils manual for details.\n\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            stdout,
        );
        emit_ancillary_info(b"shred\0" as *const u8 as *const libc::c_char);
    }
    exit(status);
}
unsafe extern "C" fn periodic_pattern(mut type_0: libc::c_int) -> bool {
    if type_0 <= 0 as libc::c_int {
        return 0 as libc::c_int != 0;
    }
    let mut r: [libc::c_uchar; 3] = [0; 3];
    let mut bits: libc::c_uint = (type_0 & 0xfff as libc::c_int) as libc::c_uint;
    bits |= bits << 12 as libc::c_int;
    r[0 as libc::c_int
        as usize] = (bits >> 4 as libc::c_int & 255 as libc::c_int as libc::c_uint)
        as libc::c_uchar;
    r[1 as libc::c_int
        as usize] = (bits >> 8 as libc::c_int & 255 as libc::c_int as libc::c_uint)
        as libc::c_uchar;
    r[2 as libc::c_int
        as usize] = (bits & 255 as libc::c_int as libc::c_uint) as libc::c_uchar;
    return r[0 as libc::c_int as usize] as libc::c_int
        != r[1 as libc::c_int as usize] as libc::c_int
        || r[0 as libc::c_int as usize] as libc::c_int
            != r[2 as libc::c_int as usize] as libc::c_int;
}
unsafe extern "C" fn fillpattern(
    mut type_0: libc::c_int,
    mut r: *mut libc::c_uchar,
    mut size: size_t,
) {
    let mut i: size_t = 0;
    let mut bits: libc::c_uint = (type_0 & 0xfff as libc::c_int) as libc::c_uint;
    bits |= bits << 12 as libc::c_int;
    *r
        .offset(
            0 as libc::c_int as isize,
        ) = (bits >> 4 as libc::c_int & 255 as libc::c_int as libc::c_uint)
        as libc::c_uchar;
    *r
        .offset(
            1 as libc::c_int as isize,
        ) = (bits >> 8 as libc::c_int & 255 as libc::c_int as libc::c_uint)
        as libc::c_uchar;
    *r
        .offset(
            2 as libc::c_int as isize,
        ) = (bits & 255 as libc::c_int as libc::c_uint) as libc::c_uchar;
    i = 3 as libc::c_int as size_t;
    while i <= size.wrapping_div(2 as libc::c_int as libc::c_ulong) {
        memcpy(r.offset(i as isize) as *mut libc::c_void, r as *const libc::c_void, i);
        i = (i as libc::c_ulong).wrapping_mul(2 as libc::c_int as libc::c_ulong)
            as size_t as size_t;
    }
    if i < size {
        memcpy(
            r.offset(i as isize) as *mut libc::c_void,
            r as *const libc::c_void,
            size.wrapping_sub(i),
        );
    }
    if type_0 & 0x1000 as libc::c_int != 0 {
        i = 0 as libc::c_int as size_t;
        while i < size {
            let ref mut fresh0 = *r.offset(i as isize);
            *fresh0 = (*fresh0 as libc::c_int ^ 0x80 as libc::c_int) as libc::c_uchar;
            i = (i as libc::c_ulong)
                .wrapping_add(SECTOR_SIZE as libc::c_int as libc::c_ulong) as size_t
                as size_t;
        }
    }
}
unsafe extern "C" fn passname(
    mut data: *const libc::c_uchar,
    mut name: *mut libc::c_char,
) {
    if !data.is_null() {
        sprintf(
            name,
            b"%02x%02x%02x\0" as *const u8 as *const libc::c_char,
            *data.offset(0 as libc::c_int as isize) as libc::c_int,
            *data.offset(1 as libc::c_int as isize) as libc::c_int,
            *data.offset(2 as libc::c_int as isize) as libc::c_int,
        );
    } else {
        memcpy(
            name as *mut libc::c_void,
            b"random\0" as *const u8 as *const libc::c_char as *const libc::c_void,
            7 as libc::c_int as libc::c_ulong,
        );
    };
}
unsafe extern "C" fn ignorable_sync_errno(mut errno_val: libc::c_int) -> bool {
    return errno_val == 22 as libc::c_int || errno_val == 9 as libc::c_int
        || errno_val == 21 as libc::c_int;
}
unsafe extern "C" fn dosync(
    mut fd: libc::c_int,
    mut qname: *const libc::c_char,
) -> libc::c_int {
    let mut err: libc::c_int = 0;
    if fdatasync(fd) == 0 as libc::c_int {
        return 0 as libc::c_int;
    }
    err = *__errno_location();
    if !ignorable_sync_errno(err) {
        error(
            0 as libc::c_int,
            err,
            dcgettext(
                0 as *const libc::c_char,
                b"%s: fdatasync failed\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            qname,
        );
        *__errno_location() = err;
        return -(1 as libc::c_int);
    }
    if fsync(fd) == 0 as libc::c_int {
        return 0 as libc::c_int;
    }
    err = *__errno_location();
    if !ignorable_sync_errno(err) {
        error(
            0 as libc::c_int,
            err,
            dcgettext(
                0 as *const libc::c_char,
                b"%s: fsync failed\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            qname,
        );
        *__errno_location() = err;
        return -(1 as libc::c_int);
    }
    sync();
    return 0 as libc::c_int;
}
unsafe extern "C" fn direct_mode(mut fd: libc::c_int, mut enable: bool) {
    let mut fd_flags: libc::c_int = rpl_fcntl(fd, 3 as libc::c_int);
    if (0 as libc::c_int) < fd_flags {
        let mut new_flags: libc::c_int = if enable as libc::c_int != 0 {
            fd_flags | 0o200000 as libc::c_int
        } else {
            fd_flags & !(0o200000 as libc::c_int)
        };
        if new_flags != fd_flags {
            rpl_fcntl(fd, 4 as libc::c_int, new_flags);
        }
    }
}
unsafe extern "C" fn dorewind(mut fd: libc::c_int, mut st: *const stat) -> bool {
    if (*st).st_mode & 0o170000 as libc::c_int as libc::c_uint
        == 0o20000 as libc::c_int as libc::c_uint
    {
        let mut op: mtop = mtop { mt_op: 0, mt_count: 0 };
        op.mt_op = 6 as libc::c_int as libc::c_short;
        op.mt_count = 1 as libc::c_int;
        if ioctl(
            fd,
            ((1 as libc::c_uint)
                << 0 as libc::c_int + 8 as libc::c_int + 8 as libc::c_int
                    + 14 as libc::c_int
                | (('m' as i32) << 0 as libc::c_int + 8 as libc::c_int) as libc::c_uint
                | ((1 as libc::c_int) << 0 as libc::c_int) as libc::c_uint)
                as libc::c_ulong
                | (::core::mem::size_of::<mtop>() as libc::c_ulong)
                    << 0 as libc::c_int + 8 as libc::c_int + 8 as libc::c_int,
            &mut op as *mut mtop,
        ) == 0 as libc::c_int
        {
            return 1 as libc::c_int != 0;
        }
    }
    let mut offset: off_t = lseek(fd, 0 as libc::c_int as __off_t, 0 as libc::c_int);
    if (0 as libc::c_int as libc::c_long) < offset {
        *__errno_location() = 22 as libc::c_int;
    }
    return offset == 0 as libc::c_int as libc::c_long;
}
unsafe extern "C" fn known(mut size: off_t) -> bool {
    return 0 as libc::c_int as libc::c_long <= size;
}
unsafe extern "C" fn dopass(
    mut fd: libc::c_int,
    mut st: *const stat,
    mut qname: *const libc::c_char,
    mut sizep: *mut off_t,
    mut type_0: libc::c_int,
    mut s: *mut randread_source,
    mut k: libc::c_ulong,
    mut n: libc::c_ulong,
) -> libc::c_int {
    let mut current_block: u64;
    let mut size: off_t = *sizep;
    let mut offset: off_t = 0;
    let mut thresh: time_t = 0 as libc::c_int as time_t;
    let mut now: time_t = 0 as libc::c_int as time_t;
    let mut lim: size_t = 0;
    let mut soff: size_t = 0;
    let mut ssize: ssize_t = 0;
    let mut page_size: size_t = getpagesize() as size_t;
    let mut output_size: size_t = (if periodic_pattern(type_0) as libc::c_int != 0 {
        60 as libc::c_int * 1024 as libc::c_int
    } else {
        64 as libc::c_int * 1024 as libc::c_int
    }) as size_t;
    let mut pbuf: *mut libc::c_uchar = xalignalloc(
        page_size as idx_t,
        output_size
            .wrapping_add(2 as libc::c_int as libc::c_ulong)
            .wrapping_div(3 as libc::c_int as libc::c_ulong)
            .wrapping_mul(3 as libc::c_int as libc::c_ulong) as idx_t,
    ) as *mut libc::c_uchar;
    let mut pass_string: [libc::c_char; 7] = [0; 7];
    let mut write_error: bool = 0 as libc::c_int != 0;
    let mut other_error: bool = 0 as libc::c_int != 0;
    let mut previous_offset_buf: [libc::c_char; 652] = [0; 652];
    let mut previous_human_offset: *const libc::c_char = 0 as *const libc::c_char;
    let mut try_without_directio: bool = (0 as libc::c_int as libc::c_long) < size
        && (size as libc::c_ulong) < output_size;
    if !try_without_directio {
        direct_mode(fd, 1 as libc::c_int != 0);
    }
    if !dorewind(fd, st) {
        error(
            0 as libc::c_int,
            *__errno_location(),
            dcgettext(
                0 as *const libc::c_char,
                b"%s: cannot rewind\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            qname,
        );
        other_error = 1 as libc::c_int != 0;
    } else {
        if type_0 >= 0 as libc::c_int {
            lim = if known(size) as libc::c_int != 0
                && (size as libc::c_ulong)
                    < output_size
                        .wrapping_add(2 as libc::c_int as libc::c_ulong)
                        .wrapping_div(3 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(3 as libc::c_int as libc::c_ulong)
            {
                size as libc::c_ulong
            } else {
                output_size
                    .wrapping_add(2 as libc::c_int as libc::c_ulong)
                    .wrapping_div(3 as libc::c_int as libc::c_ulong)
                    .wrapping_mul(3 as libc::c_int as libc::c_ulong)
            };
            fillpattern(type_0, pbuf, lim);
            passname(pbuf, pass_string.as_mut_ptr());
        } else {
            passname(0 as *const libc::c_uchar, pass_string.as_mut_ptr());
        }
        if n != 0 {
            error(
                0 as libc::c_int,
                0 as libc::c_int,
                dcgettext(
                    0 as *const libc::c_char,
                    b"%s: pass %lu/%lu (%s)...\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
                qname,
                k,
                n,
                pass_string.as_mut_ptr(),
            );
            thresh = time(0 as *mut time_t)
                + VERBOSE_UPDATE as libc::c_int as libc::c_long;
            previous_human_offset = b"\0" as *const u8 as *const libc::c_char;
        }
        offset = 0 as libc::c_int as off_t;
        's_106: loop {
            lim = output_size;
            if known(size) as libc::c_int != 0
                && ((size - offset) as libc::c_ulong) < output_size
            {
                if size < offset {
                    current_block = 16779030619667747692;
                    break;
                }
                lim = (size - offset) as size_t;
                if lim == 0 {
                    current_block = 16779030619667747692;
                    break;
                }
            }
            if type_0 < 0 as libc::c_int {
                randread(s, pbuf as *mut libc::c_void, lim);
            }
            soff = 0 as libc::c_int as size_t;
            while soff < lim {
                ssize = write(
                    fd,
                    pbuf.offset(soff as isize) as *const libc::c_void,
                    lim.wrapping_sub(soff),
                );
                if (0 as libc::c_int as libc::c_long) < ssize {
                    if ssize as libc::c_ulong <= lim.wrapping_sub(soff) {} else {
                        unreachable!();
                    };
                } else if !known(size)
                    && (ssize == 0 as libc::c_int as libc::c_long
                        || *__errno_location() == 28 as libc::c_int)
                {
                    if soff
                        <= ((if (0 as libc::c_int as off_t)
                            < -(1 as libc::c_int) as off_t
                        {
                            -(1 as libc::c_int) as off_t
                        } else {
                            (((1 as libc::c_int as off_t)
                                << (::core::mem::size_of::<off_t>() as libc::c_ulong)
                                    .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                    .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                - 1 as libc::c_int as libc::c_long)
                                * 2 as libc::c_int as libc::c_long
                                + 1 as libc::c_int as libc::c_long
                        }) - offset) as libc::c_ulong
                    {
                        size = (offset as libc::c_ulong).wrapping_add(soff) as off_t;
                        *sizep = size;
                    }
                    break;
                } else {
                    let mut errnum: libc::c_int = *__errno_location();
                    let mut buf: [libc::c_char; 21] = [0; 21];
                    if !try_without_directio && *__errno_location() == 22 as libc::c_int
                    {
                        direct_mode(fd, 0 as libc::c_int != 0);
                        ssize = 0 as libc::c_int as ssize_t;
                        try_without_directio = 1 as libc::c_int != 0;
                    } else {
                        error(
                            0 as libc::c_int,
                            errnum,
                            dcgettext(
                                0 as *const libc::c_char,
                                b"%s: error writing at offset %s\0" as *const u8
                                    as *const libc::c_char,
                                5 as libc::c_int,
                            ),
                            qname,
                            umaxtostr(
                                (offset as libc::c_ulong).wrapping_add(soff),
                                buf.as_mut_ptr(),
                            ),
                        );
                        if errnum == 5 as libc::c_int && known(size) as libc::c_int != 0
                            && (soff | SECTOR_MASK as libc::c_int as libc::c_ulong) < lim
                        {
                            let mut soff1: size_t = (soff
                                | SECTOR_MASK as libc::c_int as libc::c_ulong)
                                .wrapping_add(1 as libc::c_int as libc::c_ulong);
                            if lseek(
                                fd,
                                (offset as libc::c_ulong).wrapping_add(soff1) as __off_t,
                                0 as libc::c_int,
                            ) != -(1 as libc::c_int) as libc::c_long
                            {
                                ssize = soff1.wrapping_sub(soff) as ssize_t;
                                write_error = 1 as libc::c_int != 0;
                                current_block = 15345278821338558188;
                            } else {
                                error(
                                    0 as libc::c_int,
                                    *__errno_location(),
                                    dcgettext(
                                        0 as *const libc::c_char,
                                        b"%s: lseek failed\0" as *const u8 as *const libc::c_char,
                                        5 as libc::c_int,
                                    ),
                                    qname,
                                );
                                current_block = 1356832168064818221;
                            }
                        } else {
                            current_block = 1356832168064818221;
                        }
                        match current_block {
                            15345278821338558188 => {}
                            _ => {
                                other_error = 1 as libc::c_int != 0;
                                current_block = 10255917320279925847;
                                break 's_106;
                            }
                        }
                    }
                }
                soff = (soff as libc::c_ulong).wrapping_add(ssize as libc::c_ulong)
                    as size_t as size_t;
            }
            if (((if (0 as libc::c_int as off_t) < -(1 as libc::c_int) as off_t {
                -(1 as libc::c_int) as off_t
            } else {
                (((1 as libc::c_int as off_t)
                    << (::core::mem::size_of::<off_t>() as libc::c_ulong)
                        .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                        .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                    - 1 as libc::c_int as libc::c_long)
                    * 2 as libc::c_int as libc::c_long + 1 as libc::c_int as libc::c_long
            }) - offset) as libc::c_ulong) < soff
            {
                error(
                    0 as libc::c_int,
                    0 as libc::c_int,
                    dcgettext(
                        0 as *const libc::c_char,
                        b"%s: file too large\0" as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    qname,
                );
                other_error = 1 as libc::c_int != 0;
                current_block = 10255917320279925847;
                break;
            } else {
                offset = (offset as libc::c_ulong).wrapping_add(soff) as off_t as off_t;
                let mut done: bool = offset == size;
                if !(n != 0
                    && (done as libc::c_int != 0
                        && *previous_human_offset as libc::c_int != 0
                        || {
                            now = time(0 as *mut time_t);
                            thresh <= now
                        }))
                {
                    continue;
                }
                let mut offset_buf: [libc::c_char; 652] = [0; 652];
                let mut size_buf: [libc::c_char; 652] = [0; 652];
                let mut human_progress_opts: libc::c_int = human_autoscale as libc::c_int
                    | human_SI as libc::c_int | human_base_1024 as libc::c_int
                    | human_B as libc::c_int;
                let mut human_offset: *const libc::c_char = human_readable(
                    offset as uintmax_t,
                    offset_buf.as_mut_ptr(),
                    human_floor as libc::c_int | human_progress_opts,
                    1 as libc::c_int as uintmax_t,
                    1 as libc::c_int as uintmax_t,
                );
                if !(done as libc::c_int != 0
                    || !(strcmp(previous_human_offset, human_offset)
                        == 0 as libc::c_int))
                {
                    continue;
                }
                if !known(size) {
                    error(
                        0 as libc::c_int,
                        0 as libc::c_int,
                        dcgettext(
                            0 as *const libc::c_char,
                            b"%s: pass %lu/%lu (%s)...%s\0" as *const u8
                                as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                        qname,
                        k,
                        n,
                        pass_string.as_mut_ptr(),
                        human_offset,
                    );
                } else {
                    let mut off: uintmax_t = offset as uintmax_t;
                    let mut percent: libc::c_int = (if size
                        == 0 as libc::c_int as libc::c_long
                    {
                        100 as libc::c_int as libc::c_ulong
                    } else if off
                        <= (if (0 as libc::c_int as uintmax_t)
                            < -(1 as libc::c_int) as uintmax_t
                        {
                            -(1 as libc::c_int) as uintmax_t
                        } else {
                            ((1 as libc::c_int as uintmax_t)
                                << (::core::mem::size_of::<uintmax_t>() as libc::c_ulong)
                                    .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                    .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                .wrapping_mul(2 as libc::c_int as libc::c_ulong)
                                .wrapping_add(1 as libc::c_int as libc::c_ulong)
                        })
                            .wrapping_div(100 as libc::c_int as libc::c_ulong)
                    {
                        off.wrapping_mul(100 as libc::c_int as libc::c_ulong)
                            .wrapping_div(size as libc::c_ulong)
                    } else {
                        off.wrapping_div(
                            (size / 100 as libc::c_int as libc::c_long) as libc::c_ulong,
                        )
                    }) as libc::c_int;
                    let mut human_size: *const libc::c_char = human_readable(
                        size as uintmax_t,
                        size_buf.as_mut_ptr(),
                        human_ceiling as libc::c_int | human_progress_opts,
                        1 as libc::c_int as uintmax_t,
                        1 as libc::c_int as uintmax_t,
                    );
                    if done {
                        human_offset = human_size;
                    }
                    error(
                        0 as libc::c_int,
                        0 as libc::c_int,
                        dcgettext(
                            0 as *const libc::c_char,
                            b"%s: pass %lu/%lu (%s)...%s/%s %d%%\0" as *const u8
                                as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                        qname,
                        k,
                        n,
                        pass_string.as_mut_ptr(),
                        human_offset,
                        human_size,
                        percent,
                    );
                }
                strcpy(previous_offset_buf.as_mut_ptr(), human_offset);
                previous_human_offset = previous_offset_buf.as_mut_ptr();
                thresh = now + VERBOSE_UPDATE as libc::c_int as libc::c_long;
                if !(dosync(fd, qname) != 0 as libc::c_int) {
                    continue;
                }
                if *__errno_location() != 5 as libc::c_int {
                    other_error = 1 as libc::c_int != 0;
                    current_block = 10255917320279925847;
                    break;
                } else {
                    write_error = 1 as libc::c_int != 0;
                }
            }
        }
        match current_block {
            10255917320279925847 => {}
            _ => {
                if dosync(fd, qname) != 0 as libc::c_int {
                    if *__errno_location() != 5 as libc::c_int {
                        other_error = 1 as libc::c_int != 0;
                    } else {
                        write_error = 1 as libc::c_int != 0;
                    }
                }
            }
        }
    }
    alignfree(pbuf as *mut libc::c_void);
    return if other_error as libc::c_int != 0 {
        -(1 as libc::c_int)
    } else {
        write_error as libc::c_int
    };
}
static mut patterns: [libc::c_int; 55] = [
    -(2 as libc::c_int),
    2 as libc::c_int,
    0 as libc::c_int,
    0xfff as libc::c_int,
    2 as libc::c_int,
    0x555 as libc::c_int,
    0xaaa as libc::c_int,
    -(1 as libc::c_int),
    6 as libc::c_int,
    0x249 as libc::c_int,
    0x492 as libc::c_int,
    0x6db as libc::c_int,
    0x924 as libc::c_int,
    0xb6d as libc::c_int,
    0xdb6 as libc::c_int,
    12 as libc::c_int,
    0x111 as libc::c_int,
    0x222 as libc::c_int,
    0x333 as libc::c_int,
    0x444 as libc::c_int,
    0x666 as libc::c_int,
    0x777 as libc::c_int,
    0x888 as libc::c_int,
    0x999 as libc::c_int,
    0xbbb as libc::c_int,
    0xccc as libc::c_int,
    0xddd as libc::c_int,
    0xeee as libc::c_int,
    -(1 as libc::c_int),
    8 as libc::c_int,
    0x1000 as libc::c_int,
    0x1249 as libc::c_int,
    0x1492 as libc::c_int,
    0x16db as libc::c_int,
    0x1924 as libc::c_int,
    0x1b6d as libc::c_int,
    0x1db6 as libc::c_int,
    0x1fff as libc::c_int,
    14 as libc::c_int,
    0x1111 as libc::c_int,
    0x1222 as libc::c_int,
    0x1333 as libc::c_int,
    0x1444 as libc::c_int,
    0x1555 as libc::c_int,
    0x1666 as libc::c_int,
    0x1777 as libc::c_int,
    0x1888 as libc::c_int,
    0x1999 as libc::c_int,
    0x1aaa as libc::c_int,
    0x1bbb as libc::c_int,
    0x1ccc as libc::c_int,
    0x1ddd as libc::c_int,
    0x1eee as libc::c_int,
    -(1 as libc::c_int),
    0 as libc::c_int,
];
unsafe extern "C" fn genpattern(
    mut dest: *mut libc::c_int,
    mut num: size_t,
    mut s: *mut randint_source,
) {
    let mut randpasses: size_t = 0;
    let mut p: *const libc::c_int = 0 as *const libc::c_int;
    let mut d: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut n: size_t = 0;
    let mut accum: size_t = 0;
    let mut top: size_t = 0;
    let mut swap: size_t = 0;
    let mut k: libc::c_int = 0;
    if num == 0 {
        return;
    }
    p = patterns.as_ptr();
    randpasses = 0 as libc::c_int as size_t;
    d = dest;
    n = num;
    loop {
        let fresh1 = p;
        p = p.offset(1);
        k = *fresh1;
        if k == 0 {
            p = patterns.as_ptr();
        } else if k < 0 as libc::c_int {
            k = -k;
            if k as size_t >= n {
                randpasses = (randpasses as libc::c_ulong).wrapping_add(n) as size_t
                    as size_t;
                break;
            } else {
                randpasses = (randpasses as libc::c_ulong)
                    .wrapping_add(k as libc::c_ulong) as size_t as size_t;
                n = (n as libc::c_ulong).wrapping_sub(k as libc::c_ulong) as size_t
                    as size_t;
            }
        } else if k as size_t <= n {
            memcpy(
                d as *mut libc::c_void,
                p as *const libc::c_void,
                (k as libc::c_ulong)
                    .wrapping_mul(::core::mem::size_of::<libc::c_int>() as libc::c_ulong),
            );
            p = p.offset(k as isize);
            d = d.offset(k as isize);
            n = (n as libc::c_ulong).wrapping_sub(k as libc::c_ulong) as size_t
                as size_t;
        } else if n < 2 as libc::c_int as libc::c_ulong
            || (3 as libc::c_int as libc::c_ulong).wrapping_mul(n) < k as size_t
        {
            randpasses = (randpasses as libc::c_ulong).wrapping_add(n) as size_t
                as size_t;
            break;
        } else {
            loop {
                if n == k as size_t || randint_choose(s, k as randint) < n {
                    let fresh2 = d;
                    d = d.offset(1);
                    *fresh2 = *p;
                    n = n.wrapping_sub(1);
                }
                p = p.offset(1);
                k -= 1;
                if !(n != 0) {
                    break;
                }
            }
            break;
        }
    }
    top = num.wrapping_sub(randpasses);
    randpasses = randpasses.wrapping_sub(1);
    accum = randpasses;
    n = 0 as libc::c_int as size_t;
    while n < num {
        if accum <= randpasses {
            accum = (accum as libc::c_ulong)
                .wrapping_add(num.wrapping_sub(1 as libc::c_int as libc::c_ulong))
                as size_t as size_t;
            let fresh3 = top;
            top = top.wrapping_add(1);
            *dest.offset(fresh3 as isize) = *dest.offset(n as isize);
            *dest.offset(n as isize) = -(1 as libc::c_int);
        } else {
            swap = n.wrapping_add(randint_choose(s, top.wrapping_sub(n)));
            k = *dest.offset(n as isize);
            *dest.offset(n as isize) = *dest.offset(swap as isize);
            *dest.offset(swap as isize) = k;
        }
        accum = (accum as libc::c_ulong).wrapping_sub(randpasses) as size_t as size_t;
        n = n.wrapping_add(1);
    }
}
unsafe extern "C" fn do_wipefd(
    mut fd: libc::c_int,
    mut qname: *const libc::c_char,
    mut s: *mut randint_source,
    mut flags: *const Options,
) -> bool {
    let mut current_block: u64;
    let mut i: size_t = 0;
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
    let mut size: off_t = 0;
    let mut i_size: off_t = 0 as libc::c_int as off_t;
    let mut n: libc::c_ulong = 0;
    let mut passarray: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut ok: bool = 1 as libc::c_int != 0;
    let mut rs: *mut randread_source = 0 as *mut randread_source;
    n = 0 as libc::c_int as libc::c_ulong;
    if (*flags).verbose {
        n = ((*flags).n_iterations).wrapping_add((*flags).zero_fill as libc::c_ulong);
    }
    if fstat(fd, &mut st) != 0 {
        error(
            0 as libc::c_int,
            *__errno_location(),
            dcgettext(
                0 as *const libc::c_char,
                b"%s: fstat failed\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            qname,
        );
        return 0 as libc::c_int != 0;
    }
    if st.st_mode & 0o170000 as libc::c_int as libc::c_uint
        == 0o20000 as libc::c_int as libc::c_uint && isatty(fd) != 0
        || st.st_mode & 0o170000 as libc::c_int as libc::c_uint
            == 0o10000 as libc::c_int as libc::c_uint
        || st.st_mode & 0o170000 as libc::c_int as libc::c_uint
            == 0o140000 as libc::c_int as libc::c_uint
    {
        error(
            0 as libc::c_int,
            0 as libc::c_int,
            dcgettext(
                0 as *const libc::c_char,
                b"%s: invalid file type\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            qname,
        );
        return 0 as libc::c_int != 0;
    } else {
        if st.st_mode & 0o170000 as libc::c_int as libc::c_uint
            == 0o100000 as libc::c_int as libc::c_uint
            && st.st_size < 0 as libc::c_int as libc::c_long
        {
            error(
                0 as libc::c_int,
                0 as libc::c_int,
                dcgettext(
                    0 as *const libc::c_char,
                    b"%s: file has negative size\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
                qname,
            );
            return 0 as libc::c_int != 0;
        }
    }
    passarray = xnmalloc(
        (*flags).n_iterations,
        ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
    ) as *mut libc::c_int;
    size = (*flags).size;
    if size == -(1 as libc::c_int) as libc::c_long {
        if st.st_mode & 0o170000 as libc::c_int as libc::c_uint
            == 0o100000 as libc::c_int as libc::c_uint
        {
            size = st.st_size;
            if !(*flags).exact {
                let mut remainder: off_t = size
                    % (if (0 as libc::c_int) < st.st_blksize
                        && st.st_blksize as libc::c_ulong
                            <= (-(1 as libc::c_int) as size_t)
                                .wrapping_div(8 as libc::c_int as libc::c_ulong)
                                .wrapping_add(1 as libc::c_int as libc::c_ulong)
                    {
                        st.st_blksize
                    } else {
                        512 as libc::c_int
                    }) as libc::c_long;
                if size != 0
                    && size
                        < (if (0 as libc::c_int) < st.st_blksize
                            && st.st_blksize as libc::c_ulong
                                <= (-(1 as libc::c_int) as size_t)
                                    .wrapping_div(8 as libc::c_int as libc::c_ulong)
                                    .wrapping_add(1 as libc::c_int as libc::c_ulong)
                        {
                            st.st_blksize
                        } else {
                            512 as libc::c_int
                        }) as libc::c_long
                {
                    i_size = size;
                }
                if remainder != 0 as libc::c_int as libc::c_long {
                    let mut size_incr: off_t = (if (0 as libc::c_int) < st.st_blksize
                        && st.st_blksize as libc::c_ulong
                            <= (-(1 as libc::c_int) as size_t)
                                .wrapping_div(8 as libc::c_int as libc::c_ulong)
                                .wrapping_add(1 as libc::c_int as libc::c_ulong)
                    {
                        st.st_blksize
                    } else {
                        512 as libc::c_int
                    }) as libc::c_long - remainder;
                    size
                        += if size_incr
                            < (if (0 as libc::c_int as off_t)
                                < -(1 as libc::c_int) as off_t
                            {
                                -(1 as libc::c_int) as off_t
                            } else {
                                (((1 as libc::c_int as off_t)
                                    << (::core::mem::size_of::<off_t>() as libc::c_ulong)
                                        .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                        .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                    - 1 as libc::c_int as libc::c_long)
                                    * 2 as libc::c_int as libc::c_long
                                    + 1 as libc::c_int as libc::c_long
                            }) - size
                        {
                            size_incr
                        } else {
                            (if (0 as libc::c_int as off_t)
                                < -(1 as libc::c_int) as off_t
                            {
                                -(1 as libc::c_int) as off_t
                            } else {
                                (((1 as libc::c_int as off_t)
                                    << (::core::mem::size_of::<off_t>() as libc::c_ulong)
                                        .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                        .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                    - 1 as libc::c_int as libc::c_long)
                                    * 2 as libc::c_int as libc::c_long
                                    + 1 as libc::c_int as libc::c_long
                            }) - size
                        };
                }
            }
        } else {
            size = lseek(fd, 0 as libc::c_int as __off_t, 2 as libc::c_int);
            if size <= 0 as libc::c_int as libc::c_long {
                size = -(1 as libc::c_int) as off_t;
            }
        }
    } else if st.st_mode & 0o170000 as libc::c_int as libc::c_uint
        == 0o100000 as libc::c_int as libc::c_uint
        && st.st_size
            < (if ((if (0 as libc::c_int) < st.st_blksize
                && st.st_blksize as libc::c_ulong
                    <= (-(1 as libc::c_int) as size_t)
                        .wrapping_div(8 as libc::c_int as libc::c_ulong)
                        .wrapping_add(1 as libc::c_int as libc::c_ulong)
            {
                st.st_blksize
            } else {
                512 as libc::c_int
            }) as libc::c_long) < size
            {
                (if (0 as libc::c_int) < st.st_blksize
                    && st.st_blksize as libc::c_ulong
                        <= (-(1 as libc::c_int) as size_t)
                            .wrapping_div(8 as libc::c_int as libc::c_ulong)
                            .wrapping_add(1 as libc::c_int as libc::c_ulong)
                {
                    st.st_blksize
                } else {
                    512 as libc::c_int
                }) as libc::c_long
            } else {
                size
            })
    {
        i_size = st.st_size;
    }
    genpattern(passarray, (*flags).n_iterations, s);
    rs = randint_get_source(s);
    's_166: loop {
        let mut pass_size: off_t = 0;
        let mut pn: libc::c_ulong = n;
        if i_size != 0 {
            pass_size = i_size;
            i_size = 0 as libc::c_int as off_t;
            pn = 0 as libc::c_int as libc::c_ulong;
        } else if size != 0 {
            pass_size = size;
            size = 0 as libc::c_int as off_t;
        } else {
            current_block = 2989495919056355252;
            break;
        }
        i = 0 as libc::c_int as size_t;
        while i
            < ((*flags).n_iterations).wrapping_add((*flags).zero_fill as libc::c_ulong)
        {
            let mut err: libc::c_int = 0 as libc::c_int;
            let mut type_0: libc::c_int = if i < (*flags).n_iterations {
                *passarray.offset(i as isize)
            } else {
                0 as libc::c_int
            };
            err = dopass(
                fd,
                &mut st,
                qname,
                &mut pass_size,
                type_0,
                rs,
                i.wrapping_add(1 as libc::c_int as libc::c_ulong),
                pn,
            );
            if err != 0 {
                ok = 0 as libc::c_int != 0;
                if err < 0 as libc::c_int {
                    current_block = 4132310808043861668;
                    break 's_166;
                }
            }
            i = i.wrapping_add(1);
        }
    }
    match current_block {
        2989495919056355252 => {
            if (*flags).remove_file as libc::c_uint != 0
                && ftruncate(fd, 0 as libc::c_int as __off_t) != 0 as libc::c_int
                && (st.st_mode & 0o170000 as libc::c_int as libc::c_uint
                    == 0o100000 as libc::c_int as libc::c_uint
                    || (st.st_mode).wrapping_sub(st.st_mode) != 0)
            {
                error(
                    0 as libc::c_int,
                    *__errno_location(),
                    dcgettext(
                        0 as *const libc::c_char,
                        b"%s: error truncating\0" as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    qname,
                );
                ok = 0 as libc::c_int != 0;
            }
        }
        _ => {}
    }
    free(passarray as *mut libc::c_void);
    return ok;
}
unsafe extern "C" fn wipefd(
    mut fd: libc::c_int,
    mut qname: *const libc::c_char,
    mut s: *mut randint_source,
    mut flags: *const Options,
) -> bool {
    let mut fd_flags: libc::c_int = rpl_fcntl(fd, 3 as libc::c_int);
    if fd_flags < 0 as libc::c_int {
        error(
            0 as libc::c_int,
            *__errno_location(),
            dcgettext(
                0 as *const libc::c_char,
                b"%s: fcntl failed\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            qname,
        );
        return 0 as libc::c_int != 0;
    }
    if fd_flags & 0o2000 as libc::c_int != 0 {
        error(
            0 as libc::c_int,
            0 as libc::c_int,
            dcgettext(
                0 as *const libc::c_char,
                b"%s: cannot shred append-only file descriptor\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
            qname,
        );
        return 0 as libc::c_int != 0;
    }
    return do_wipefd(fd, qname, s, flags);
}
static mut nameset: [libc::c_char; 65] = unsafe {
    *::core::mem::transmute::<
        &[u8; 65],
        &[libc::c_char; 65],
    >(b"0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ_.\0")
};
unsafe extern "C" fn incname(mut name: *mut libc::c_char, mut len: size_t) -> bool {
    loop {
        let fresh4 = len;
        len = len.wrapping_sub(1);
        if !(fresh4 != 0) {
            break;
        }
        let mut p: *const libc::c_char = strchr(
            nameset.as_ptr(),
            *name.offset(len as isize) as libc::c_int,
        );
        if !p.is_null() {} else {
            __assert_fail(
                b"p\0" as *const u8 as *const libc::c_char,
                b"src/shred.c\0" as *const u8 as *const libc::c_char,
                1000 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 30],
                    &[libc::c_char; 30],
                >(b"_Bool incname(char *, size_t)\0"))
                    .as_ptr(),
            );
        }
        if *p.offset(1 as libc::c_int as isize) != 0 {
            *name.offset(len as isize) = *p.offset(1 as libc::c_int as isize);
            return 1 as libc::c_int != 0;
        }
        *name.offset(len as isize) = nameset[0 as libc::c_int as usize];
    }
    return 0 as libc::c_int != 0;
}
unsafe extern "C" fn wipename(
    mut oldname: *mut libc::c_char,
    mut qoldname: *const libc::c_char,
    mut flags: *const Options,
) -> bool {
    let mut newname: *mut libc::c_char = xstrdup(oldname);
    let mut base: *mut libc::c_char = last_component(newname);
    let mut dir: *mut libc::c_char = dir_name(newname);
    let mut qdir: *mut libc::c_char = xstrdup(
        quotearg_n_style_colon(0 as libc::c_int, shell_escape_quoting_style, dir),
    );
    let mut first: bool = 1 as libc::c_int != 0;
    let mut ok: bool = 1 as libc::c_int != 0;
    let mut dir_fd: libc::c_int = -(1 as libc::c_int);
    if (*flags).remove_file as libc::c_uint
        == remove_wipesync as libc::c_int as libc::c_uint
    {
        dir_fd = open_safer(
            dir,
            0 as libc::c_int | 0o40000 as libc::c_int | 0o400 as libc::c_int
                | 0o4000 as libc::c_int,
        );
    }
    if (*flags).verbose {
        error(
            0 as libc::c_int,
            0 as libc::c_int,
            dcgettext(
                0 as *const libc::c_char,
                b"%s: removing\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            qoldname,
        );
    }
    if (*flags).remove_file as libc::c_uint
        != remove_unlink as libc::c_int as libc::c_uint
    {
        let mut len: size_t = base_len(base);
        while len != 0 as libc::c_int as libc::c_ulong {
            memset(
                base as *mut libc::c_void,
                nameset[0 as libc::c_int as usize] as libc::c_int,
                len,
            );
            *base.offset(len as isize) = 0 as libc::c_int as libc::c_char;
            let mut rename_ok: bool = false;
            loop {
                rename_ok = renameatu(
                    -(100 as libc::c_int),
                    oldname,
                    -(100 as libc::c_int),
                    newname,
                    ((1 as libc::c_int) << 0 as libc::c_int) as libc::c_uint,
                ) == 0 as libc::c_int;
                if !(!rename_ok && *__errno_location() == 17 as libc::c_int
                    && incname(base, len) as libc::c_int != 0)
                {
                    break;
                }
            }
            if rename_ok {
                if 0 as libc::c_int <= dir_fd && dosync(dir_fd, qdir) != 0 as libc::c_int
                {
                    ok = 0 as libc::c_int != 0;
                }
                if (*flags).verbose {
                    let mut old: *const libc::c_char = if first as libc::c_int != 0 {
                        qoldname
                    } else {
                        oldname as *const libc::c_char
                    };
                    error(
                        0 as libc::c_int,
                        0 as libc::c_int,
                        dcgettext(
                            0 as *const libc::c_char,
                            b"%s: renamed to %s\0" as *const u8 as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                        old,
                        newname,
                    );
                    first = 0 as libc::c_int != 0;
                }
                memcpy(
                    oldname.offset(base.offset_from(newname) as libc::c_long as isize)
                        as *mut libc::c_void,
                    base as *const libc::c_void,
                    len.wrapping_add(1 as libc::c_int as libc::c_ulong),
                );
            }
            len = len.wrapping_sub(1);
        }
    }
    if unlink(oldname) != 0 as libc::c_int {
        error(
            0 as libc::c_int,
            *__errno_location(),
            dcgettext(
                0 as *const libc::c_char,
                b"%s: failed to remove\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            qoldname,
        );
        ok = 0 as libc::c_int != 0;
    } else if (*flags).verbose {
        error(
            0 as libc::c_int,
            0 as libc::c_int,
            dcgettext(
                0 as *const libc::c_char,
                b"%s: removed\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            qoldname,
        );
    }
    if 0 as libc::c_int <= dir_fd {
        if dosync(dir_fd, qdir) != 0 as libc::c_int {
            ok = 0 as libc::c_int != 0;
        }
        if close(dir_fd) != 0 as libc::c_int {
            error(
                0 as libc::c_int,
                *__errno_location(),
                dcgettext(
                    0 as *const libc::c_char,
                    b"%s: failed to close\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
                qdir,
            );
            ok = 0 as libc::c_int != 0;
        }
    }
    free(newname as *mut libc::c_void);
    free(dir as *mut libc::c_void);
    free(qdir as *mut libc::c_void);
    return ok;
}
unsafe extern "C" fn wipefile(
    mut name: *mut libc::c_char,
    mut qname: *const libc::c_char,
    mut s: *mut randint_source,
    mut flags: *const Options,
) -> bool {
    let mut ok: bool = false;
    let mut fd: libc::c_int = 0;
    fd = open_safer(name, 0o1 as libc::c_int | 0o400 as libc::c_int | 0 as libc::c_int);
    if fd < 0 as libc::c_int
        && (*__errno_location() == 13 as libc::c_int
            && (*flags).force as libc::c_int != 0)
        && chmod(name, 0o200 as libc::c_int as __mode_t) == 0 as libc::c_int
    {
        fd = open_safer(
            name,
            0o1 as libc::c_int | 0o400 as libc::c_int | 0 as libc::c_int,
        );
    }
    if fd < 0 as libc::c_int {
        error(
            0 as libc::c_int,
            *__errno_location(),
            dcgettext(
                0 as *const libc::c_char,
                b"%s: failed to open for writing\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            qname,
        );
        return 0 as libc::c_int != 0;
    }
    ok = do_wipefd(fd, qname, s, flags);
    if close(fd) != 0 as libc::c_int {
        error(
            0 as libc::c_int,
            *__errno_location(),
            dcgettext(
                0 as *const libc::c_char,
                b"%s: failed to close\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            qname,
        );
        ok = 0 as libc::c_int != 0;
    }
    if ok as libc::c_int != 0 && (*flags).remove_file as libc::c_uint != 0 {
        ok = wipename(name, qname, flags);
    }
    return ok;
}
static mut randint_source: *mut randint_source = 0 as *const randint_source
    as *mut randint_source;
unsafe extern "C" fn clear_random_data() {
    randint_all_free(randint_source);
}
unsafe fn main_0(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut ok: bool = 1 as libc::c_int != 0;
    let mut flags: Options = {
        let mut init = Options {
            force: 0 as libc::c_int != 0,
            n_iterations: 0,
            size: 0,
            remove_file: remove_none,
            verbose: false,
            exact: false,
            zero_fill: false,
        };
        init
    };
    let mut file: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    let mut n_files: libc::c_int = 0;
    let mut c: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut random_source: *const libc::c_char = 0 as *const libc::c_char;
    set_program_name(*argv.offset(0 as libc::c_int as isize));
    setlocale(6 as libc::c_int, b"\0" as *const u8 as *const libc::c_char);
    bindtextdomain(
        b"coreutils\0" as *const u8 as *const libc::c_char,
        b"/usr/local/share/locale\0" as *const u8 as *const libc::c_char,
    );
    textdomain(b"coreutils\0" as *const u8 as *const libc::c_char);
    atexit(Some(close_stdout as unsafe extern "C" fn() -> ()));
    flags.n_iterations = DEFAULT_PASSES as libc::c_int as size_t;
    flags.size = -(1 as libc::c_int) as off_t;
    loop {
        c = getopt_long(
            argc,
            argv,
            b"fn:s:uvxz\0" as *const u8 as *const libc::c_char,
            long_opts.as_ptr(),
            0 as *mut libc::c_int,
        );
        if !(c != -(1 as libc::c_int)) {
            break;
        }
        match c {
            102 => {
                flags.force = 1 as libc::c_int != 0;
            }
            110 => {
                flags
                    .n_iterations = xdectoumax(
                    optarg,
                    0 as libc::c_int as uintmax_t,
                    if (9223372036854775807 as libc::c_long as libc::c_ulong)
                        .wrapping_mul(2 as libc::c_ulong)
                        .wrapping_add(1 as libc::c_ulong)
                        < (18446744073709551615 as libc::c_ulong)
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            )
                    {
                        (9223372036854775807 as libc::c_long as libc::c_ulong)
                            .wrapping_mul(2 as libc::c_ulong)
                            .wrapping_add(1 as libc::c_ulong)
                    } else {
                        (18446744073709551615 as libc::c_ulong)
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            )
                    },
                    b"\0" as *const u8 as *const libc::c_char,
                    dcgettext(
                        0 as *const libc::c_char,
                        b"invalid number of passes\0" as *const u8
                            as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    0 as libc::c_int,
                );
            }
            256 => {
                if !random_source.is_null()
                    && !(strcmp(random_source, optarg) == 0 as libc::c_int)
                {
                    if ::core::mem::size_of::<C2RustUnnamed_7>() as libc::c_ulong != 0 {
                        error(
                            1 as libc::c_int,
                            0 as libc::c_int,
                            dcgettext(
                                0 as *const libc::c_char,
                                b"multiple random sources specified\0" as *const u8
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
                                b"multiple random sources specified\0" as *const u8
                                    as *const libc::c_char,
                                5 as libc::c_int,
                            ),
                        );
                        if 0 as libc::c_int != 0 {} else {
                            unreachable!();
                        };
                    };
                }
                random_source = optarg;
            }
            117 => {
                if optarg.is_null() {
                    flags.remove_file = remove_wipesync;
                } else {
                    flags
                        .remove_file = remove_methods[__xargmatch_internal(
                        b"--remove\0" as *const u8 as *const libc::c_char,
                        optarg,
                        remove_args.as_ptr(),
                        remove_methods.as_ptr() as *const libc::c_void,
                        ::core::mem::size_of::<remove_method>() as libc::c_ulong,
                        argmatch_die,
                        1 as libc::c_int != 0,
                    ) as usize];
                }
            }
            115 => {
                flags
                    .size = xnumtoumax(
                    optarg,
                    0 as libc::c_int,
                    0 as libc::c_int as uintmax_t,
                    (if (0 as libc::c_int as off_t) < -(1 as libc::c_int) as off_t {
                        -(1 as libc::c_int) as off_t
                    } else {
                        (((1 as libc::c_int as off_t)
                            << (::core::mem::size_of::<off_t>() as libc::c_ulong)
                                .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                            - 1 as libc::c_int as libc::c_long)
                            * 2 as libc::c_int as libc::c_long
                            + 1 as libc::c_int as libc::c_long
                    }) as uintmax_t,
                    b"cbBkKMGTPEZYRQ0\0" as *const u8 as *const libc::c_char,
                    dcgettext(
                        0 as *const libc::c_char,
                        b"invalid file size\0" as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    0 as libc::c_int,
                ) as off_t;
            }
            118 => {
                flags.verbose = 1 as libc::c_int != 0;
            }
            120 => {
                flags.exact = 1 as libc::c_int != 0;
            }
            122 => {
                flags.zero_fill = 1 as libc::c_int != 0;
            }
            -2 => {
                usage(0 as libc::c_int);
            }
            -3 => {
                version_etc(
                    stdout,
                    b"shred\0" as *const u8 as *const libc::c_char,
                    b"GNU coreutils\0" as *const u8 as *const libc::c_char,
                    Version,
                    b"Colin Plumb\0" as *const u8 as *const libc::c_char,
                    0 as *mut libc::c_void as *mut libc::c_char,
                );
                exit(0 as libc::c_int);
            }
            _ => {
                usage(1 as libc::c_int);
            }
        }
    }
    file = argv.offset(optind as isize);
    n_files = argc - optind;
    if n_files == 0 as libc::c_int {
        error(
            0 as libc::c_int,
            0 as libc::c_int,
            dcgettext(
                0 as *const libc::c_char,
                b"missing file operand\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
        usage(1 as libc::c_int);
    }
    randint_source = randint_all_new(
        random_source,
        18446744073709551615 as libc::c_ulong,
    );
    if randint_source.is_null() {
        if ::core::mem::size_of::<C2RustUnnamed_6>() as libc::c_ulong != 0 {
            error(
                1 as libc::c_int,
                *__errno_location(),
                b"%s\0" as *const u8 as *const libc::c_char,
                quotearg_n_style_colon(
                    0 as libc::c_int,
                    shell_escape_quoting_style,
                    (if !random_source.is_null() {
                        random_source
                    } else {
                        b"getrandom\0" as *const u8 as *const libc::c_char
                    }),
                ),
            );
            if 0 as libc::c_int != 0 {} else {
                unreachable!();
            };
        } else {
            error(
                1 as libc::c_int,
                *__errno_location(),
                b"%s\0" as *const u8 as *const libc::c_char,
                quotearg_n_style_colon(
                    0 as libc::c_int,
                    shell_escape_quoting_style,
                    (if !random_source.is_null() {
                        random_source
                    } else {
                        b"getrandom\0" as *const u8 as *const libc::c_char
                    }),
                ),
            );
            if 0 as libc::c_int != 0 {} else {
                unreachable!();
            };
        };
    }
    atexit(Some(clear_random_data as unsafe extern "C" fn() -> ()));
    i = 0 as libc::c_int;
    while i < n_files {
        let mut qname: *mut libc::c_char = xstrdup(
            quotearg_n_style_colon(
                0 as libc::c_int,
                shell_escape_quoting_style,
                *file.offset(i as isize),
            ),
        );
        if strcmp(*file.offset(i as isize), b"-\0" as *const u8 as *const libc::c_char)
            == 0 as libc::c_int
        {
            ok = (ok as libc::c_int
                & wipefd(1 as libc::c_int, qname, randint_source, &mut flags)
                    as libc::c_int) as bool;
        } else {
            ok = (ok as libc::c_int
                & wipefile(*file.offset(i as isize), qname, randint_source, &mut flags)
                    as libc::c_int) as bool;
        }
        free(qname as *mut libc::c_void);
        i += 1;
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
