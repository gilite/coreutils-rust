#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![feature(extern_types)]
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type __dirstream;
    pub type cycle_check_state;
    pub type hash_table;
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
    fn __errno_location() -> *mut libc::c_int;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strncmp(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn quotearg_n_style_colon(
        n: libc::c_int,
        s: quoting_style,
        arg: *const libc::c_char,
    ) -> *mut libc::c_char;
    fn quotearg_style(s: quoting_style, arg: *const libc::c_char) -> *mut libc::c_char;
    fn quotearg_n_style(
        n: libc::c_int,
        s: quoting_style,
        arg: *const libc::c_char,
    ) -> *mut libc::c_char;
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
    fn atexit(__func: Option::<unsafe extern "C" fn() -> ()>) -> libc::c_int;
    fn error(
        __status: libc::c_int,
        __errnum: libc::c_int,
        __format: *const libc::c_char,
        _: ...
    );
    fn quote_n(n: libc::c_int, arg: *const libc::c_char) -> *const libc::c_char;
    fn quote(arg: *const libc::c_char) -> *const libc::c_char;
    fn get_root_dev_ino(root_d_i: *mut dev_ino) -> *mut dev_ino;
    fn getfileconat(
        dir_fd: libc::c_int,
        file: *const libc::c_char,
        con: *mut *mut libc::c_char,
    ) -> libc::c_int;
    fn lgetfileconat(
        dir_fd: libc::c_int,
        file: *const libc::c_char,
        con: *mut *mut libc::c_char,
    ) -> libc::c_int;
    fn setfileconat(
        dir_fd: libc::c_int,
        file: *const libc::c_char,
        con: *const libc::c_char,
    ) -> libc::c_int;
    fn lsetfileconat(
        dir_fd: libc::c_int,
        file: *const libc::c_char,
        con: *const libc::c_char,
    ) -> libc::c_int;
    fn rpl_fts_close(_: *mut FTS) -> libc::c_int;
    fn rpl_fts_read(_: *mut FTS) -> *mut FTSENT;
    fn rpl_fts_set(_: *mut FTS, _: *mut FTSENT, _: libc::c_int) -> libc::c_int;
    fn xfts_open(
        _: *const *mut libc::c_char,
        options: libc::c_int,
        _: Option::<
            unsafe extern "C" fn(*mut *const FTSENT, *mut *const FTSENT) -> libc::c_int,
        >,
    ) -> *mut FTS;
    fn cycle_warning_required(fts: *const FTS, ent: *const FTSENT) -> bool;
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
pub type DIR = __dirstream;
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
pub struct dev_ino {
    pub st_ino: ino_t,
    pub st_dev: dev_t,
}
pub type context_t = libc::c_int;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct I_ring {
    pub ir_data: [libc::c_int; 4],
    pub ir_default_val: libc::c_int,
    pub ir_front: libc::c_uint,
    pub ir_back: libc::c_uint,
    pub ir_empty: bool,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FTS {
    pub fts_cur: *mut _ftsent,
    pub fts_child: *mut _ftsent,
    pub fts_array: *mut *mut _ftsent,
    pub fts_dev: dev_t,
    pub fts_path: *mut libc::c_char,
    pub fts_rfd: libc::c_int,
    pub fts_cwd_fd: libc::c_int,
    pub fts_pathlen: size_t,
    pub fts_nitems: size_t,
    pub fts_compar: Option::<
        unsafe extern "C" fn(*mut *const _ftsent, *mut *const _ftsent) -> libc::c_int,
    >,
    pub fts_options: libc::c_int,
    pub fts_leaf_optimization_works_ht: *mut hash_table,
    pub fts_cycle: C2RustUnnamed_0,
    pub fts_fd_ring: I_ring,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_0 {
    pub ht: *mut hash_table,
    pub state: *mut cycle_check_state,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _ftsent {
    pub fts_cycle: *mut _ftsent,
    pub fts_parent: *mut _ftsent,
    pub fts_link: *mut _ftsent,
    pub fts_dirp: *mut DIR,
    pub fts_number: libc::c_long,
    pub fts_pointer: *mut libc::c_void,
    pub fts_accpath: *mut libc::c_char,
    pub fts_path: *mut libc::c_char,
    pub fts_errno: libc::c_int,
    pub fts_symfd: libc::c_int,
    pub fts_pathlen: size_t,
    pub fts_fts: *mut FTS,
    pub fts_level: ptrdiff_t,
    pub fts_namelen: size_t,
    pub fts_info: libc::c_ushort,
    pub fts_flags: libc::c_ushort,
    pub fts_instr: libc::c_ushort,
    pub fts_statp: [stat; 1],
    pub fts_name: [libc::c_char; 0],
}
pub type FTSENT = _ftsent;
pub type C2RustUnnamed_1 = libc::c_uint;
pub const REFERENCE_FILE_OPTION: C2RustUnnamed_1 = 259;
pub const PRESERVE_ROOT: C2RustUnnamed_1 = 258;
pub const NO_PRESERVE_ROOT: C2RustUnnamed_1 = 257;
pub const DEREFERENCE_OPTION: C2RustUnnamed_1 = 256;
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
unsafe extern "C" fn context_free(mut c: context_t) {}
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
static mut affect_symlink_referent: bool = false;
static mut recurse: bool = false;
static mut verbose: bool = false;
static mut root_dev_ino: *mut dev_ino = 0 as *const dev_ino as *mut dev_ino;
static mut specified_context: *const libc::c_char = 0 as *const libc::c_char;
static mut specified_user: *const libc::c_char = 0 as *const libc::c_char;
static mut specified_role: *const libc::c_char = 0 as *const libc::c_char;
static mut specified_range: *const libc::c_char = 0 as *const libc::c_char;
static mut specified_type: *const libc::c_char = 0 as *const libc::c_char;
static mut long_options: [option; 14] = [
    {
        let mut init = option {
            name: b"recursive\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'R' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"dereference\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: DEREFERENCE_OPTION as libc::c_int,
        };
        init
    },
    {
        let mut init = option {
            name: b"no-dereference\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'h' as i32,
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
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: PRESERVE_ROOT as libc::c_int,
        };
        init
    },
    {
        let mut init = option {
            name: b"reference\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: REFERENCE_FILE_OPTION as libc::c_int,
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
            name: b"range\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'l' as i32,
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
unsafe extern "C" fn compute_context_from_mask(
    mut context: *const libc::c_char,
    mut ret: *mut context_t,
) -> libc::c_int {
    let mut ok: bool = 1 as libc::c_int != 0;
    let mut new_context: context_t = context_new(context);
    if new_context == 0 {
        error(
            0 as libc::c_int,
            *__errno_location(),
            dcgettext(
                0 as *const libc::c_char,
                b"failed to create security context: %s\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
            quote(context),
        );
        return 1 as libc::c_int;
    }
    if !specified_user.is_null() && context_user_set(new_context, specified_user) != 0 {
        error(
            0 as libc::c_int,
            *__errno_location(),
            dcgettext(
                0 as *const libc::c_char,
                b"failed to set %s security context component to %s\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
            b"user\0" as *const u8 as *const libc::c_char,
            quote(specified_user),
        );
        ok = 0 as libc::c_int != 0;
    }
    if !specified_range.is_null() && context_range_set(new_context, specified_range) != 0
    {
        error(
            0 as libc::c_int,
            *__errno_location(),
            dcgettext(
                0 as *const libc::c_char,
                b"failed to set %s security context component to %s\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
            b"range\0" as *const u8 as *const libc::c_char,
            quote(specified_range),
        );
        ok = 0 as libc::c_int != 0;
    }
    if !specified_role.is_null() && context_role_set(new_context, specified_role) != 0 {
        error(
            0 as libc::c_int,
            *__errno_location(),
            dcgettext(
                0 as *const libc::c_char,
                b"failed to set %s security context component to %s\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
            b"role\0" as *const u8 as *const libc::c_char,
            quote(specified_role),
        );
        ok = 0 as libc::c_int != 0;
    }
    if !specified_type.is_null() && context_type_set(new_context, specified_type) != 0 {
        error(
            0 as libc::c_int,
            *__errno_location(),
            dcgettext(
                0 as *const libc::c_char,
                b"failed to set %s security context component to %s\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
            b"type\0" as *const u8 as *const libc::c_char,
            quote(specified_type),
        );
        ok = 0 as libc::c_int != 0;
    }
    if !ok {
        let mut saved_errno: libc::c_int = *__errno_location();
        context_free(new_context);
        *__errno_location() = saved_errno;
        return 1 as libc::c_int;
    }
    *ret = new_context;
    return 0 as libc::c_int;
}
unsafe extern "C" fn change_file_context(
    mut fd: libc::c_int,
    mut file: *const libc::c_char,
) -> libc::c_int {
    let mut file_context: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut context: context_t = 0 as libc::c_int;
    let mut context_string: *const libc::c_char = 0 as *const libc::c_char;
    let mut errors: libc::c_int = 0 as libc::c_int;
    if specified_context.is_null() {
        let mut status: libc::c_int = if affect_symlink_referent as libc::c_int != 0 {
            getfileconat(fd, file, &mut file_context)
        } else {
            lgetfileconat(fd, file, &mut file_context)
        };
        if status < 0 as libc::c_int && *__errno_location() != 61 as libc::c_int {
            error(
                0 as libc::c_int,
                *__errno_location(),
                dcgettext(
                    0 as *const libc::c_char,
                    b"failed to get security context of %s\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
                quotearg_style(shell_escape_always_quoting_style, file),
            );
            return 1 as libc::c_int;
        }
        if file_context.is_null() {
            error(
                0 as libc::c_int,
                0 as libc::c_int,
                dcgettext(
                    0 as *const libc::c_char,
                    b"can't apply partial context to unlabeled file %s\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
                quotearg_style(shell_escape_always_quoting_style, file),
            );
            return 1 as libc::c_int;
        }
        if compute_context_from_mask(file_context, &mut context) != 0 {
            return 1 as libc::c_int;
        }
        context_string = context_str(context);
    } else {
        context_string = specified_context;
    }
    if file_context.is_null()
        || !(strcmp(context_string, file_context) == 0 as libc::c_int)
    {
        let mut fail: libc::c_int = if affect_symlink_referent as libc::c_int != 0 {
            setfileconat(fd, file, context_string)
        } else {
            lsetfileconat(fd, file, context_string)
        };
        if fail != 0 {
            errors = 1 as libc::c_int;
            error(
                0 as libc::c_int,
                *__errno_location(),
                dcgettext(
                    0 as *const libc::c_char,
                    b"failed to change context of %s to %s\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
                quotearg_n_style(
                    0 as libc::c_int,
                    shell_escape_always_quoting_style,
                    file,
                ),
                quote_n(1 as libc::c_int, context_string),
            );
        }
    }
    if specified_context.is_null() {
        context_free(context);
        freecon(file_context);
    }
    return errors;
}
unsafe extern "C" fn process_file(mut fts: *mut FTS, mut ent: *mut FTSENT) -> bool {
    let mut file_full_name: *const libc::c_char = (*ent).fts_path;
    let mut file: *const libc::c_char = (*ent).fts_accpath;
    let mut file_stats: *const stat = ((*ent).fts_statp).as_mut_ptr();
    let mut ok: bool = 1 as libc::c_int != 0;
    match (*ent).fts_info as libc::c_int {
        1 => {
            if recurse {
                if !root_dev_ino.is_null()
                    && ((*((*ent).fts_statp).as_mut_ptr()).st_ino
                        == (*root_dev_ino).st_ino
                        && (*((*ent).fts_statp).as_mut_ptr()).st_dev
                            == (*root_dev_ino).st_dev)
                {
                    if strcmp(file_full_name, b"/\0" as *const u8 as *const libc::c_char)
                        == 0 as libc::c_int
                    {
                        error(
                            0 as libc::c_int,
                            0 as libc::c_int,
                            dcgettext(
                                0 as *const libc::c_char,
                                b"it is dangerous to operate recursively on %s\0"
                                    as *const u8 as *const libc::c_char,
                                5 as libc::c_int,
                            ),
                            quotearg_style(
                                shell_escape_always_quoting_style,
                                file_full_name,
                            ),
                        );
                    } else {
                        error(
                            0 as libc::c_int,
                            0 as libc::c_int,
                            dcgettext(
                                0 as *const libc::c_char,
                                b"it is dangerous to operate recursively on %s (same as %s)\0"
                                    as *const u8 as *const libc::c_char,
                                5 as libc::c_int,
                            ),
                            quotearg_n_style(
                                0 as libc::c_int,
                                shell_escape_always_quoting_style,
                                file_full_name,
                            ),
                            quotearg_n_style(
                                1 as libc::c_int,
                                shell_escape_always_quoting_style,
                                b"/\0" as *const u8 as *const libc::c_char,
                            ),
                        );
                    }
                    error(
                        0 as libc::c_int,
                        0 as libc::c_int,
                        dcgettext(
                            0 as *const libc::c_char,
                            b"use --no-preserve-root to override this failsafe\0"
                                as *const u8 as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                    );
                    rpl_fts_set(fts, ent, 4 as libc::c_int);
                    rpl_fts_read(fts);
                    return 0 as libc::c_int != 0;
                }
                return 1 as libc::c_int != 0;
            }
        }
        6 => {
            if !recurse {
                return 1 as libc::c_int != 0;
            }
        }
        10 => {
            if (*ent).fts_level == 0 as libc::c_int as libc::c_long
                && (*ent).fts_number == 0 as libc::c_int as libc::c_long
            {
                (*ent).fts_number = 1 as libc::c_int as libc::c_long;
                rpl_fts_set(fts, ent, 1 as libc::c_int);
                return 1 as libc::c_int != 0;
            }
            error(
                0 as libc::c_int,
                (*ent).fts_errno,
                dcgettext(
                    0 as *const libc::c_char,
                    b"cannot access %s\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
                quotearg_style(shell_escape_always_quoting_style, file_full_name),
            );
            ok = 0 as libc::c_int != 0;
        }
        7 => {
            error(
                0 as libc::c_int,
                (*ent).fts_errno,
                b"%s\0" as *const u8 as *const libc::c_char,
                quotearg_n_style_colon(
                    0 as libc::c_int,
                    shell_escape_quoting_style,
                    file_full_name,
                ),
            );
            ok = 0 as libc::c_int != 0;
        }
        4 => {
            error(
                0 as libc::c_int,
                (*ent).fts_errno,
                dcgettext(
                    0 as *const libc::c_char,
                    b"cannot read directory %s\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
                quotearg_style(shell_escape_always_quoting_style, file_full_name),
            );
            ok = 0 as libc::c_int != 0;
        }
        2 => {
            if cycle_warning_required(fts, ent) {
                error(
                    0 as libc::c_int,
                    0 as libc::c_int,
                    dcgettext(
                        0 as *const libc::c_char,
                        b"WARNING: Circular directory structure.\nThis almost certainly means that you have a corrupted file system.\nNOTIFY YOUR SYSTEM MANAGER.\nThe following directory is part of the cycle:\n  %s\n\0"
                            as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    quotearg_n_style_colon(
                        0 as libc::c_int,
                        shell_escape_quoting_style,
                        file_full_name,
                    ),
                );
                return 0 as libc::c_int != 0;
            }
        }
        _ => {}
    }
    if (*ent).fts_info as libc::c_int == 6 as libc::c_int && ok as libc::c_int != 0
        && (!root_dev_ino.is_null()
            && ((*file_stats).st_ino == (*root_dev_ino).st_ino
                && (*file_stats).st_dev == (*root_dev_ino).st_dev))
    {
        if strcmp(file_full_name, b"/\0" as *const u8 as *const libc::c_char)
            == 0 as libc::c_int
        {
            error(
                0 as libc::c_int,
                0 as libc::c_int,
                dcgettext(
                    0 as *const libc::c_char,
                    b"it is dangerous to operate recursively on %s\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
                quotearg_style(shell_escape_always_quoting_style, file_full_name),
            );
        } else {
            error(
                0 as libc::c_int,
                0 as libc::c_int,
                dcgettext(
                    0 as *const libc::c_char,
                    b"it is dangerous to operate recursively on %s (same as %s)\0"
                        as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
                quotearg_n_style(
                    0 as libc::c_int,
                    shell_escape_always_quoting_style,
                    file_full_name,
                ),
                quotearg_n_style(
                    1 as libc::c_int,
                    shell_escape_always_quoting_style,
                    b"/\0" as *const u8 as *const libc::c_char,
                ),
            );
        }
        error(
            0 as libc::c_int,
            0 as libc::c_int,
            dcgettext(
                0 as *const libc::c_char,
                b"use --no-preserve-root to override this failsafe\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
        ok = 0 as libc::c_int != 0;
    }
    if ok {
        if verbose {
            printf(
                dcgettext(
                    0 as *const libc::c_char,
                    b"changing security context of %s\n\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
                quotearg_style(shell_escape_always_quoting_style, file_full_name),
            );
        }
        if change_file_context((*fts).fts_cwd_fd, file) != 0 as libc::c_int {
            ok = 0 as libc::c_int != 0;
        }
    }
    if !recurse {
        rpl_fts_set(fts, ent, 4 as libc::c_int);
    }
    return ok;
}
unsafe extern "C" fn process_files(
    mut files: *mut *mut libc::c_char,
    mut bit_flags: libc::c_int,
) -> bool {
    let mut ok: bool = 1 as libc::c_int != 0;
    let mut fts: *mut FTS = xfts_open(files, bit_flags, None);
    loop {
        let mut ent: *mut FTSENT = 0 as *mut FTSENT;
        ent = rpl_fts_read(fts);
        if ent.is_null() {
            if *__errno_location() != 0 as libc::c_int {
                error(
                    0 as libc::c_int,
                    *__errno_location(),
                    dcgettext(
                        0 as *const libc::c_char,
                        b"fts_read failed\0" as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                );
                ok = 0 as libc::c_int != 0;
            }
            break;
        } else {
            ok = (ok as libc::c_int & process_file(fts, ent) as libc::c_int) as bool;
        }
    }
    if rpl_fts_close(fts) != 0 as libc::c_int {
        error(
            0 as libc::c_int,
            *__errno_location(),
            dcgettext(
                0 as *const libc::c_char,
                b"fts_close failed\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
        ok = 0 as libc::c_int != 0;
    }
    return ok;
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
                b"Usage: %s [OPTION]... CONTEXT FILE...\n  or:  %s [OPTION]... [-u USER] [-r ROLE] [-l RANGE] [-t TYPE] FILE...\n  or:  %s [OPTION]... --reference=RFILE FILE...\n\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            program_name,
            program_name,
            program_name,
        );
        fputs_unlocked(
            dcgettext(
                0 as *const libc::c_char,
                b"Change the SELinux security context of each FILE to CONTEXT.\nWith --reference, change the security context of each FILE to that of RFILE.\n\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            stdout,
        );
        emit_mandatory_arg_note();
        fputs_unlocked(
            dcgettext(
                0 as *const libc::c_char,
                b"      --dereference      affect the referent of each symbolic link (this is\n                         the default), rather than the symbolic link itself\n  -h, --no-dereference   affect symbolic links instead of any referenced file\n\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            stdout,
        );
        fputs_unlocked(
            dcgettext(
                0 as *const libc::c_char,
                b"  -u, --user=USER        set user USER in the target security context\n  -r, --role=ROLE        set role ROLE in the target security context\n  -t, --type=TYPE        set type TYPE in the target security context\n  -l, --range=RANGE      set range RANGE in the target security context\n\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            stdout,
        );
        fputs_unlocked(
            dcgettext(
                0 as *const libc::c_char,
                b"      --no-preserve-root  do not treat '/' specially (the default)\n      --preserve-root    fail to operate recursively on '/'\n\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            stdout,
        );
        fputs_unlocked(
            dcgettext(
                0 as *const libc::c_char,
                b"      --reference=RFILE  use RFILE's security context rather than specifying\n                         a CONTEXT value\n\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            stdout,
        );
        fputs_unlocked(
            dcgettext(
                0 as *const libc::c_char,
                b"  -R, --recursive        operate on files and directories recursively\n\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            stdout,
        );
        fputs_unlocked(
            dcgettext(
                0 as *const libc::c_char,
                b"  -v, --verbose          output a diagnostic for every file processed\n\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            stdout,
        );
        fputs_unlocked(
            dcgettext(
                0 as *const libc::c_char,
                b"\nThe following options modify how a hierarchy is traversed when the -R\noption is also specified.  If more than one is specified, only the final\none takes effect.\n\n  -H                     if a command line argument is a symbolic link\n                         to a directory, traverse it\n  -L                     traverse every symbolic link to a directory\n                         encountered\n  -P                     do not traverse any symbolic links (default)\n\n\0"
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
        emit_ancillary_info(b"chcon\0" as *const u8 as *const libc::c_char);
    }
    exit(status);
}
unsafe fn main_0(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut bit_flags: libc::c_int = 0x10 as libc::c_int;
    let mut dereference: libc::c_int = -(1 as libc::c_int);
    let mut ok: bool = false;
    let mut preserve_root: bool = 0 as libc::c_int != 0;
    let mut component_specified: bool = 0 as libc::c_int != 0;
    let mut reference_file: *mut libc::c_char = 0 as *mut libc::c_char;
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
            b"HLPRhvu:r:t:l:\0" as *const u8 as *const libc::c_char,
            long_options.as_ptr(),
            0 as *mut libc::c_int,
        );
        if !(optc != -(1 as libc::c_int)) {
            break;
        }
        match optc {
            72 => {
                bit_flags = 0x1 as libc::c_int | 0x10 as libc::c_int;
            }
            76 => {
                bit_flags = 0x2 as libc::c_int;
            }
            80 => {
                bit_flags = 0x10 as libc::c_int;
            }
            104 => {
                dereference = 0 as libc::c_int;
            }
            256 => {
                dereference = 1 as libc::c_int;
            }
            257 => {
                preserve_root = 0 as libc::c_int != 0;
            }
            258 => {
                preserve_root = 1 as libc::c_int != 0;
            }
            259 => {
                reference_file = optarg;
            }
            82 => {
                recurse = 1 as libc::c_int != 0;
            }
            102 => {}
            118 => {
                verbose = 1 as libc::c_int != 0;
            }
            117 => {
                specified_user = optarg;
                component_specified = 1 as libc::c_int != 0;
            }
            114 => {
                specified_role = optarg;
                component_specified = 1 as libc::c_int != 0;
            }
            116 => {
                specified_type = optarg;
                component_specified = 1 as libc::c_int != 0;
            }
            108 => {
                specified_range = optarg;
                component_specified = 1 as libc::c_int != 0;
            }
            -2 => {
                usage(0 as libc::c_int);
            }
            -3 => {
                version_etc(
                    stdout,
                    b"chcon\0" as *const u8 as *const libc::c_char,
                    b"GNU coreutils\0" as *const u8 as *const libc::c_char,
                    Version,
                    b"Russell Coker\0" as *const u8 as *const libc::c_char,
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
    if recurse {
        if bit_flags == 0x10 as libc::c_int {
            if dereference == 1 as libc::c_int {
                if ::core::mem::size_of::<C2RustUnnamed_6>() as libc::c_ulong != 0 {
                    error(
                        1 as libc::c_int,
                        0 as libc::c_int,
                        dcgettext(
                            0 as *const libc::c_char,
                            b"-R --dereference requires either -H or -L\0" as *const u8
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
                            b"-R --dereference requires either -H or -L\0" as *const u8
                                as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                    );
                    if 0 as libc::c_int != 0 {} else {
                        unreachable!();
                    };
                };
            }
            affect_symlink_referent = 0 as libc::c_int != 0;
        } else {
            if dereference == 0 as libc::c_int {
                if ::core::mem::size_of::<C2RustUnnamed_5>() as libc::c_ulong != 0 {
                    error(
                        1 as libc::c_int,
                        0 as libc::c_int,
                        dcgettext(
                            0 as *const libc::c_char,
                            b"-R -h requires -P\0" as *const u8 as *const libc::c_char,
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
                            b"-R -h requires -P\0" as *const u8 as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                    );
                    if 0 as libc::c_int != 0 {} else {
                        unreachable!();
                    };
                };
            }
            affect_symlink_referent = 1 as libc::c_int != 0;
        }
    } else {
        bit_flags = 0x10 as libc::c_int;
        affect_symlink_referent = dereference != 0 as libc::c_int;
    }
    if argc - optind
        < (if !reference_file.is_null() || component_specified as libc::c_int != 0 {
            1 as libc::c_int
        } else {
            2 as libc::c_int
        })
    {
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
        } else {
            error(
                0 as libc::c_int,
                0 as libc::c_int,
                dcgettext(
                    0 as *const libc::c_char,
                    b"missing operand after %s\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
                quote(*argv.offset((argc - 1 as libc::c_int) as isize)),
            );
        }
        usage(1 as libc::c_int);
    }
    if !reference_file.is_null() {
        let mut ref_context: *mut libc::c_char = 0 as *mut libc::c_char;
        if getfilecon(reference_file, &mut ref_context) < 0 as libc::c_int {
            if ::core::mem::size_of::<C2RustUnnamed_4>() as libc::c_ulong != 0 {
                error(
                    1 as libc::c_int,
                    *__errno_location(),
                    dcgettext(
                        0 as *const libc::c_char,
                        b"failed to get security context of %s\0" as *const u8
                            as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    quotearg_style(shell_escape_always_quoting_style, reference_file),
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
                        b"failed to get security context of %s\0" as *const u8
                            as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    quotearg_style(shell_escape_always_quoting_style, reference_file),
                );
                if 0 as libc::c_int != 0 {} else {
                    unreachable!();
                };
            };
        }
        specified_context = ref_context;
    } else if component_specified {
        specified_context = 0 as *const libc::c_char;
    } else {
        let fresh0 = optind;
        optind = optind + 1;
        specified_context = *argv.offset(fresh0 as isize);
        if (0 as libc::c_int) < 0 as libc::c_int
            && security_check_context(specified_context) < 0 as libc::c_int
        {
            if ::core::mem::size_of::<C2RustUnnamed_3>() as libc::c_ulong != 0 {
                error(
                    1 as libc::c_int,
                    *__errno_location(),
                    dcgettext(
                        0 as *const libc::c_char,
                        b"invalid context: %s\0" as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    quote(specified_context),
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
                        b"invalid context: %s\0" as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    quote(specified_context),
                );
                if 0 as libc::c_int != 0 {} else {
                    unreachable!();
                };
            };
        }
    }
    if !reference_file.is_null() && component_specified as libc::c_int != 0 {
        error(
            0 as libc::c_int,
            0 as libc::c_int,
            dcgettext(
                0 as *const libc::c_char,
                b"conflicting security context specifiers given\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
        usage(1 as libc::c_int);
    }
    if recurse as libc::c_int != 0 && preserve_root as libc::c_int != 0 {
        static mut dev_ino_buf: dev_ino = dev_ino { st_ino: 0, st_dev: 0 };
        root_dev_ino = get_root_dev_ino(&mut dev_ino_buf);
        if root_dev_ino.is_null() {
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
    } else {
        root_dev_ino = 0 as *mut dev_ino;
    }
    ok = process_files(argv.offset(optind as isize), bit_flags | 0x8 as libc::c_int);
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
