#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![feature(extern_types)]
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type hash_table;
    static mut stdout: *mut FILE;
    static mut stderr: *mut FILE;
    fn renameat(
        __oldfd: libc::c_int,
        __old: *const libc::c_char,
        __newfd: libc::c_int,
        __new: *const libc::c_char,
    ) -> libc::c_int;
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
    fn close(__fd: libc::c_int) -> libc::c_int;
    fn linkat(
        __fromfd: libc::c_int,
        __from: *const libc::c_char,
        __tofd: libc::c_int,
        __to: *const libc::c_char,
        __flags: libc::c_int,
    ) -> libc::c_int;
    fn symlinkat(
        __from: *const libc::c_char,
        __tofd: libc::c_int,
        __to: *const libc::c_char,
    ) -> libc::c_int;
    fn fstatat(
        __fd: libc::c_int,
        __file: *const libc::c_char,
        __buf: *mut stat,
        __flag: libc::c_int,
    ) -> libc::c_int;
    fn fstat(__fd: libc::c_int, __buf: *mut stat) -> libc::c_int;
    fn stat(__file: *const libc::c_char, __buf: *mut stat) -> libc::c_int;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strncmp(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn free(_: *mut libc::c_void);
    fn __errno_location() -> *mut libc::c_int;
    fn atexit(__func: Option::<unsafe extern "C" fn() -> ()>) -> libc::c_int;
    fn exit(_: libc::c_int) -> !;
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
    fn xalloc_die();
    fn xmalloc(s: size_t) -> *mut libc::c_void;
    fn xstrdup(str: *const libc::c_char) -> *mut libc::c_char;
    fn last_component(filename: *const libc::c_char) -> *mut libc::c_char;
    fn dir_name(file: *const libc::c_char) -> *mut libc::c_char;
    fn strip_trailing_slashes(file: *mut libc::c_char) -> bool;
    fn close_stdin();
    fn version_etc(
        stream: *mut FILE,
        command_name: *const libc::c_char,
        package: *const libc::c_char,
        version: *const libc::c_char,
        _: ...
    );
    static mut program_name: *const libc::c_char;
    fn set_program_name(argv0: *const libc::c_char);
    fn set_simple_backup_suffix(_: *const libc::c_char);
    fn find_backup_file_name(
        _: libc::c_int,
        _: *const libc::c_char,
        _: backup_type,
    ) -> *mut libc::c_char;
    fn xget_version(
        context: *const libc::c_char,
        arg: *const libc::c_char,
    ) -> backup_type;
    fn error(
        __status: libc::c_int,
        __errnum: libc::c_int,
        __format: *const libc::c_char,
        _: ...
    );
    fn openat_safer(
        _: libc::c_int,
        _: *const libc::c_char,
        _: libc::c_int,
        _: ...
    ) -> libc::c_int;
    fn file_name_concat(
        dir: *const libc::c_char,
        base: *const libc::c_char,
        base_in_result: *mut *mut libc::c_char,
    ) -> *mut libc::c_char;
    fn hash_initialize(
        candidate: size_t,
        tuning: *const Hash_tuning,
        hasher: Hash_hasher,
        comparator: Hash_comparator,
        data_freer: Hash_data_freer,
    ) -> *mut Hash_table;
    fn record_file(ht: *mut Hash_table, file: *const libc::c_char, stats: *const stat);
    fn seen_file(
        ht: *const Hash_table,
        file: *const libc::c_char,
        stats: *const stat,
    ) -> bool;
    fn force_linkat(
        _: libc::c_int,
        _: *const libc::c_char,
        _: libc::c_int,
        _: *const libc::c_char,
        _: libc::c_int,
        _: bool,
        _: libc::c_int,
    ) -> libc::c_int;
    fn force_symlinkat(
        _: *const libc::c_char,
        _: libc::c_int,
        _: *const libc::c_char,
        _: bool,
        _: libc::c_int,
    ) -> libc::c_int;
    fn triple_hash(x: *const libc::c_void, table_size: size_t) -> size_t;
    fn triple_free(x: *mut libc::c_void);
    fn triple_compare(x: *const libc::c_void, y: *const libc::c_void) -> bool;
    fn relpath(
        can_fname: *const libc::c_char,
        can_reldir: *const libc::c_char,
        buf: *mut libc::c_char,
        len: size_t,
    ) -> bool;
    fn same_nameat(
        _: libc::c_int,
        _: *const libc::c_char,
        _: libc::c_int,
        _: *const libc::c_char,
    ) -> bool;
    fn yesno() -> bool;
    fn canonicalize_filename_mode(
        _: *const libc::c_char,
        _: canonicalize_mode_t,
    ) -> *mut libc::c_char;
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
pub type C2RustUnnamed = libc::c_uint;
pub const O_PATHSEARCH: C2RustUnnamed = 2097152;
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
pub type backup_type = libc::c_uint;
pub const numbered_backups: backup_type = 3;
pub const numbered_existing_backups: backup_type = 2;
pub const simple_backups: backup_type = 1;
pub const no_backups: backup_type = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct hash_tuning {
    pub shrink_threshold: libc::c_float,
    pub shrink_factor: libc::c_float,
    pub growth_threshold: libc::c_float,
    pub growth_factor: libc::c_float,
    pub is_n_buckets: bool,
}
pub type Hash_tuning = hash_tuning;
pub type Hash_table = hash_table;
pub type Hash_hasher = Option::<
    unsafe extern "C" fn(*const libc::c_void, size_t) -> size_t,
>;
pub type Hash_comparator = Option::<
    unsafe extern "C" fn(*const libc::c_void, *const libc::c_void) -> bool,
>;
pub type Hash_data_freer = Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>;
pub type canonicalize_mode_t = libc::c_uint;
pub const CAN_NOLINKS: canonicalize_mode_t = 4;
pub const CAN_MISSING: canonicalize_mode_t = 2;
pub const CAN_ALL_BUT_LAST: canonicalize_mode_t = 1;
pub const CAN_EXISTING: canonicalize_mode_t = 0;
pub type C2RustUnnamed_1 = libc::c_uint;
pub const DEST_INFO_INITIAL_CAPACITY: C2RustUnnamed_1 = 61;
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
unsafe extern "C" fn emit_backup_suffix_note() {
    fputs_unlocked(
        dcgettext(
            0 as *const libc::c_char,
            b"\nThe backup suffix is '~', unless set with --suffix or SIMPLE_BACKUP_SUFFIX.\nThe version control method may be selected via the --backup option or through\nthe VERSION_CONTROL environment variable.  Here are the values:\n\n\0"
                as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        ),
        stdout,
    );
    fputs_unlocked(
        dcgettext(
            0 as *const libc::c_char,
            b"  none, off       never make backups (even if --backup is given)\n  numbered, t     make numbered backups\n  existing, nil   numbered if numbered backups exist, simple otherwise\n  simple, never   always make simple backups\n\0"
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
unsafe extern "C" fn priv_set_remove_linkdir() -> libc::c_int {
    return -(1 as libc::c_int);
}
static mut backup_type: backup_type = no_backups;
static mut symbolic_link: bool = false;
static mut relative: bool = false;
static mut logical: bool = 0 as libc::c_int != 0;
static mut interactive: bool = false;
static mut remove_existing_files: bool = false;
static mut verbose: bool = false;
static mut hard_dir_link: bool = false;
static mut beware_hard_dir_link: bool = false;
static mut dereference_dest_dir_symlinks: bool = 1 as libc::c_int != 0;
static mut dest_set: *mut Hash_table = 0 as *const Hash_table as *mut Hash_table;
static mut long_options: [option; 16] = [
    {
        let mut init = option {
            name: b"backup\0" as *const u8 as *const libc::c_char,
            has_arg: 2 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'b' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"directory\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'F' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"no-dereference\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'n' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"no-target-directory\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'T' as i32,
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
            name: b"interactive\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'i' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"suffix\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'S' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"target-directory\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 't' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"logical\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'L' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"physical\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'P' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"relative\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'r' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"symbolic\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 's' as i32,
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
unsafe extern "C" fn errnoize(mut status: libc::c_int) -> libc::c_int {
    return if status < 0 as libc::c_int {
        *__errno_location()
    } else {
        0 as libc::c_int
    };
}
unsafe extern "C" fn convert_abs_rel(
    mut from: *const libc::c_char,
    mut target: *const libc::c_char,
) -> *mut libc::c_char {
    let mut targetdir: *mut libc::c_char = dir_name(target);
    let mut realdest: *mut libc::c_char = canonicalize_filename_mode(
        targetdir,
        CAN_MISSING,
    );
    let mut realfrom: *mut libc::c_char = canonicalize_filename_mode(from, CAN_MISSING);
    let mut relative_from: *mut libc::c_char = 0 as *mut libc::c_char;
    if !realdest.is_null() && !realfrom.is_null() {
        relative_from = xmalloc(4096 as libc::c_int as size_t) as *mut libc::c_char;
        if !relpath(realfrom, realdest, relative_from, 4096 as libc::c_int as size_t) {
            free(relative_from as *mut libc::c_void);
            relative_from = 0 as *mut libc::c_char;
        }
    }
    free(targetdir as *mut libc::c_void);
    free(realdest as *mut libc::c_void);
    free(realfrom as *mut libc::c_void);
    return if !relative_from.is_null() { relative_from } else { xstrdup(from) };
}
unsafe extern "C" fn atomic_link(
    mut source: *const libc::c_char,
    mut destdir_fd: libc::c_int,
    mut dest_base: *const libc::c_char,
) -> libc::c_int {
    return if symbolic_link as libc::c_int != 0 {
        if relative as libc::c_int != 0 {
            -(1 as libc::c_int)
        } else {
            errnoize(symlinkat(source, destdir_fd, dest_base))
        }
    } else if beware_hard_dir_link as libc::c_int != 0 {
        -(1 as libc::c_int)
    } else {
        errnoize(
            linkat(
                -(100 as libc::c_int),
                source,
                destdir_fd,
                dest_base,
                if logical as libc::c_int != 0 {
                    0x400 as libc::c_int
                } else {
                    0 as libc::c_int
                },
            ),
        )
    };
}
unsafe extern "C" fn do_link(
    mut source: *const libc::c_char,
    mut destdir_fd: libc::c_int,
    mut dest_base: *const libc::c_char,
    mut dest: *const libc::c_char,
    mut link_errno: libc::c_int,
) -> bool {
    let mut current_block: u64;
    let mut source_stats: stat = stat {
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
    let mut source_status: libc::c_int = 1 as libc::c_int;
    let mut backup_base: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut rel_source: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut nofollow_flag: libc::c_int = if logical as libc::c_int != 0 {
        0 as libc::c_int
    } else {
        0x100 as libc::c_int
    };
    if link_errno < 0 as libc::c_int {
        link_errno = atomic_link(source, destdir_fd, dest_base);
    }
    if (link_errno != 0 || !dest_set.is_null()) && !symbolic_link {
        source_status = fstatat(
            -(100 as libc::c_int),
            source,
            &mut source_stats,
            nofollow_flag,
        );
        if source_status != 0 as libc::c_int {
            error(
                0 as libc::c_int,
                *__errno_location(),
                dcgettext(
                    0 as *const libc::c_char,
                    b"failed to access %s\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
                quotearg_style(shell_escape_always_quoting_style, source),
            );
            return 0 as libc::c_int != 0;
        }
    }
    if link_errno != 0 {
        if !symbolic_link && !hard_dir_link
            && source_stats.st_mode & 0o170000 as libc::c_int as libc::c_uint
                == 0o40000 as libc::c_int as libc::c_uint
        {
            error(
                0 as libc::c_int,
                0 as libc::c_int,
                dcgettext(
                    0 as *const libc::c_char,
                    b"%s: hard link not allowed for directory\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
                quotearg_n_style_colon(
                    0 as libc::c_int,
                    shell_escape_quoting_style,
                    source,
                ),
            );
            return 0 as libc::c_int != 0;
        }
        if relative {
            rel_source = convert_abs_rel(source, dest);
            source = rel_source;
        }
        let mut force: bool = remove_existing_files as libc::c_int != 0
            || interactive as libc::c_int != 0
            || backup_type as libc::c_uint != no_backups as libc::c_int as libc::c_uint;
        if force {
            let mut dest_stats: stat = stat {
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
            if fstatat(destdir_fd, dest_base, &mut dest_stats, 0x100 as libc::c_int)
                != 0 as libc::c_int
            {
                if *__errno_location() != 2 as libc::c_int {
                    error(
                        0 as libc::c_int,
                        *__errno_location(),
                        dcgettext(
                            0 as *const libc::c_char,
                            b"failed to access %s\0" as *const u8 as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                        quotearg_style(shell_escape_always_quoting_style, dest),
                    );
                    current_block = 2493290401997341551;
                } else {
                    force = 0 as libc::c_int != 0;
                    current_block = 790185930182612747;
                }
            } else if dest_stats.st_mode & 0o170000 as libc::c_int as libc::c_uint
                == 0o40000 as libc::c_int as libc::c_uint
            {
                error(
                    0 as libc::c_int,
                    0 as libc::c_int,
                    dcgettext(
                        0 as *const libc::c_char,
                        b"%s: cannot overwrite directory\0" as *const u8
                            as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    quotearg_n_style_colon(
                        0 as libc::c_int,
                        shell_escape_quoting_style,
                        dest,
                    ),
                );
                current_block = 2493290401997341551;
            } else if seen_file(dest_set, dest, &mut dest_stats) {
                error(
                    0 as libc::c_int,
                    0 as libc::c_int,
                    dcgettext(
                        0 as *const libc::c_char,
                        b"will not overwrite just-created %s with %s\0" as *const u8
                            as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    quotearg_n_style(
                        0 as libc::c_int,
                        shell_escape_always_quoting_style,
                        dest,
                    ),
                    quotearg_n_style(
                        1 as libc::c_int,
                        shell_escape_always_quoting_style,
                        source,
                    ),
                );
                current_block = 2493290401997341551;
            } else {
                if if backup_type as libc::c_uint
                    != no_backups as libc::c_int as libc::c_uint
                {
                    !symbolic_link as libc::c_int
                } else {
                    remove_existing_files as libc::c_int
                } != 0
                {
                    if source_status != 0 as libc::c_int {
                        source_status = stat(source, &mut source_stats);
                    }
                    if source_status == 0 as libc::c_int
                        && (source_stats.st_ino == dest_stats.st_ino
                            && source_stats.st_dev == dest_stats.st_dev)
                        && (source_stats.st_nlink == 1 as libc::c_int as libc::c_uint
                            || same_nameat(
                                -(100 as libc::c_int),
                                source,
                                destdir_fd,
                                dest_base,
                            ) as libc::c_int != 0)
                    {
                        error(
                            0 as libc::c_int,
                            0 as libc::c_int,
                            dcgettext(
                                0 as *const libc::c_char,
                                b"%s and %s are the same file\0" as *const u8
                                    as *const libc::c_char,
                                5 as libc::c_int,
                            ),
                            quotearg_n_style(
                                0 as libc::c_int,
                                shell_escape_always_quoting_style,
                                source,
                            ),
                            quotearg_n_style(
                                1 as libc::c_int,
                                shell_escape_always_quoting_style,
                                dest,
                            ),
                        );
                        current_block = 2493290401997341551;
                    } else {
                        current_block = 14818589718467733107;
                    }
                } else {
                    current_block = 14818589718467733107;
                }
                match current_block {
                    2493290401997341551 => {}
                    _ => {
                        if link_errno < 0 as libc::c_int
                            || link_errno == 17 as libc::c_int
                        {
                            if interactive {
                                fprintf(
                                    stderr,
                                    dcgettext(
                                        0 as *const libc::c_char,
                                        b"%s: replace %s? \0" as *const u8 as *const libc::c_char,
                                        5 as libc::c_int,
                                    ),
                                    program_name,
                                    quotearg_style(shell_escape_always_quoting_style, dest),
                                );
                                if !yesno() {
                                    free(rel_source as *mut libc::c_void);
                                    return 0 as libc::c_int != 0;
                                }
                            }
                            if backup_type as libc::c_uint
                                != no_backups as libc::c_int as libc::c_uint
                            {
                                backup_base = find_backup_file_name(
                                    destdir_fd,
                                    dest_base,
                                    backup_type,
                                );
                                if renameat(destdir_fd, dest_base, destdir_fd, backup_base)
                                    != 0 as libc::c_int
                                {
                                    let mut rename_errno: libc::c_int = *__errno_location();
                                    free(backup_base as *mut libc::c_void);
                                    backup_base = 0 as *mut libc::c_char;
                                    if rename_errno != 2 as libc::c_int {
                                        error(
                                            0 as libc::c_int,
                                            rename_errno,
                                            dcgettext(
                                                0 as *const libc::c_char,
                                                b"cannot backup %s\0" as *const u8 as *const libc::c_char,
                                                5 as libc::c_int,
                                            ),
                                            quotearg_style(shell_escape_always_quoting_style, dest),
                                        );
                                        current_block = 2493290401997341551;
                                    } else {
                                        force = 0 as libc::c_int != 0;
                                        current_block = 790185930182612747;
                                    }
                                } else {
                                    current_block = 790185930182612747;
                                }
                            } else {
                                current_block = 790185930182612747;
                            }
                        } else {
                            current_block = 790185930182612747;
                        }
                    }
                }
            }
            match current_block {
                790185930182612747 => {}
                _ => {
                    free(rel_source as *mut libc::c_void);
                    return 0 as libc::c_int != 0;
                }
            }
        }
        link_errno = if symbolic_link as libc::c_int != 0 {
            force_symlinkat(source, destdir_fd, dest_base, force, link_errno)
        } else {
            force_linkat(
                -(100 as libc::c_int),
                source,
                destdir_fd,
                dest_base,
                if logical as libc::c_int != 0 {
                    0x400 as libc::c_int
                } else {
                    0 as libc::c_int
                },
                force,
                link_errno,
            )
        };
    }
    if link_errno <= 0 as libc::c_int {
        if !symbolic_link {
            record_file(dest_set, dest, &mut source_stats);
        }
        if verbose {
            let mut quoted_backup: *const libc::c_char = b"\0" as *const u8
                as *const libc::c_char;
            let mut backup_sep: *const libc::c_char = b"\0" as *const u8
                as *const libc::c_char;
            if !backup_base.is_null() {
                let mut backup: *mut libc::c_char = backup_base;
                let mut alloc: *mut libc::c_void = 0 as *mut libc::c_void;
                let mut destdirlen: ptrdiff_t = dest_base.offset_from(dest)
                    as libc::c_long;
                if (0 as libc::c_int as libc::c_long) < destdirlen {
                    alloc = xmalloc(
                        (destdirlen as libc::c_ulong)
                            .wrapping_add(strlen(backup_base))
                            .wrapping_add(1 as libc::c_int as libc::c_ulong),
                    );
                    backup = memcpy(
                        alloc,
                        dest as *const libc::c_void,
                        destdirlen as libc::c_ulong,
                    ) as *mut libc::c_char;
                    strcpy(backup.offset(destdirlen as isize), backup_base);
                }
                quoted_backup = quotearg_n_style(
                    2 as libc::c_int,
                    shell_escape_always_quoting_style,
                    backup,
                );
                backup_sep = b" ~ \0" as *const u8 as *const libc::c_char;
                free(alloc);
            }
            printf(
                b"%s%s%s %c> %s\n\0" as *const u8 as *const libc::c_char,
                quoted_backup,
                backup_sep,
                quotearg_n_style(
                    0 as libc::c_int,
                    shell_escape_always_quoting_style,
                    dest,
                ),
                if symbolic_link as libc::c_int != 0 { '-' as i32 } else { '=' as i32 },
                quotearg_n_style(
                    1 as libc::c_int,
                    shell_escape_always_quoting_style,
                    source,
                ),
            );
        }
    } else {
        error(
            0 as libc::c_int,
            link_errno,
            if symbolic_link as libc::c_int != 0 {
                if link_errno != 36 as libc::c_int && *source as libc::c_int != 0 {
                    dcgettext(
                        0 as *const libc::c_char,
                        b"failed to create symbolic link %s\0" as *const u8
                            as *const libc::c_char,
                        5 as libc::c_int,
                    )
                } else {
                    dcgettext(
                        0 as *const libc::c_char,
                        b"failed to create symbolic link %s -> %s\0" as *const u8
                            as *const libc::c_char,
                        5 as libc::c_int,
                    )
                }
            } else if link_errno == 31 as libc::c_int {
                dcgettext(
                    0 as *const libc::c_char,
                    b"failed to create hard link to %.0s%s\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                )
            } else if link_errno == 122 as libc::c_int || link_errno == 17 as libc::c_int
                || link_errno == 28 as libc::c_int || link_errno == 30 as libc::c_int
            {
                dcgettext(
                    0 as *const libc::c_char,
                    b"failed to create hard link %s\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                )
            } else {
                dcgettext(
                    0 as *const libc::c_char,
                    b"failed to create hard link %s => %s\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                )
            },
            quotearg_n_style(0 as libc::c_int, shell_escape_always_quoting_style, dest),
            quotearg_n_style(1 as libc::c_int, shell_escape_always_quoting_style, source),
        );
        if !backup_base.is_null() {
            if renameat(destdir_fd, backup_base, destdir_fd, dest_base)
                != 0 as libc::c_int
            {
                error(
                    0 as libc::c_int,
                    *__errno_location(),
                    dcgettext(
                        0 as *const libc::c_char,
                        b"cannot un-backup %s\0" as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    quotearg_style(shell_escape_always_quoting_style, dest),
                );
            }
        }
    }
    free(backup_base as *mut libc::c_void);
    free(rel_source as *mut libc::c_void);
    return link_errno <= 0 as libc::c_int;
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
                b"Usage: %s [OPTION]... [-T] TARGET LINK_NAME\n  or:  %s [OPTION]... TARGET\n  or:  %s [OPTION]... TARGET... DIRECTORY\n  or:  %s [OPTION]... -t DIRECTORY TARGET...\n\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            program_name,
            program_name,
            program_name,
            program_name,
        );
        fputs_unlocked(
            dcgettext(
                0 as *const libc::c_char,
                b"In the 1st form, create a link to TARGET with the name LINK_NAME.\nIn the 2nd form, create a link to TARGET in the current directory.\nIn the 3rd and 4th forms, create links to each TARGET in DIRECTORY.\nCreate hard links by default, symbolic links with --symbolic.\nBy default, each destination (name of new link) should not already exist.\nWhen creating hard links, each TARGET must exist.  Symbolic links\ncan hold arbitrary text; if later resolved, a relative link is\ninterpreted in relation to its parent directory.\n\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            stdout,
        );
        emit_mandatory_arg_note();
        fputs_unlocked(
            dcgettext(
                0 as *const libc::c_char,
                b"      --backup[=CONTROL]      make a backup of each existing destination file\n  -b                          like --backup but does not accept an argument\n  -d, -F, --directory         allow the superuser to attempt to hard link\n                                directories (note: will probably fail due to\n                                system restrictions, even for the superuser)\n  -f, --force                 remove existing destination files\n\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            stdout,
        );
        fputs_unlocked(
            dcgettext(
                0 as *const libc::c_char,
                b"  -i, --interactive           prompt whether to remove destinations\n  -L, --logical               dereference TARGETs that are symbolic links\n  -n, --no-dereference        treat LINK_NAME as a normal file if\n                                it is a symbolic link to a directory\n  -P, --physical              make hard links directly to symbolic links\n  -r, --relative              with -s, create links relative to link location\n  -s, --symbolic              make symbolic links instead of hard links\n\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            stdout,
        );
        fputs_unlocked(
            dcgettext(
                0 as *const libc::c_char,
                b"  -S, --suffix=SUFFIX         override the usual backup suffix\n  -t, --target-directory=DIRECTORY  specify the DIRECTORY in which to create\n                                the links\n  -T, --no-target-directory   treat LINK_NAME as a normal file always\n  -v, --verbose               print name of each linked file\n\0"
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
        emit_backup_suffix_note();
        printf(
            dcgettext(
                0 as *const libc::c_char,
                b"\nUsing -s ignores -L and -P.  Otherwise, the last option specified controls\nbehavior when a TARGET is a symbolic link, defaulting to %s.\n\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            if 0 as libc::c_int != 0 {
                b"-L\0" as *const u8 as *const libc::c_char
            } else {
                b"-P\0" as *const u8 as *const libc::c_char
            },
        );
        emit_ancillary_info(b"ln\0" as *const u8 as *const libc::c_char);
    }
    exit(status);
}
unsafe fn main_0(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut c: libc::c_int = 0;
    let mut ok: bool = false;
    let mut make_backups: bool = 0 as libc::c_int != 0;
    let mut backup_suffix: *const libc::c_char = 0 as *const libc::c_char;
    let mut version_control_string: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut target_directory: *const libc::c_char = 0 as *const libc::c_char;
    let mut destdir_fd: libc::c_int = 0;
    let mut no_target_directory: bool = 0 as libc::c_int != 0;
    let mut n_files: libc::c_int = 0;
    let mut file: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    let mut link_errno: libc::c_int = -(1 as libc::c_int);
    set_program_name(*argv.offset(0 as libc::c_int as isize));
    setlocale(6 as libc::c_int, b"\0" as *const u8 as *const libc::c_char);
    bindtextdomain(
        b"coreutils\0" as *const u8 as *const libc::c_char,
        b"/usr/local/share/locale\0" as *const u8 as *const libc::c_char,
    );
    textdomain(b"coreutils\0" as *const u8 as *const libc::c_char);
    atexit(Some(close_stdin as unsafe extern "C" fn() -> ()));
    hard_dir_link = 0 as libc::c_int != 0;
    verbose = hard_dir_link;
    interactive = verbose;
    remove_existing_files = interactive;
    symbolic_link = remove_existing_files;
    loop {
        c = getopt_long(
            argc,
            argv,
            b"bdfinrst:vFLPS:T\0" as *const u8 as *const libc::c_char,
            long_options.as_ptr(),
            0 as *mut libc::c_int,
        );
        if !(c != -(1 as libc::c_int)) {
            break;
        }
        match c {
            98 => {
                make_backups = 1 as libc::c_int != 0;
                if !optarg.is_null() {
                    version_control_string = optarg;
                }
            }
            100 | 70 => {
                hard_dir_link = 1 as libc::c_int != 0;
            }
            102 => {
                remove_existing_files = 1 as libc::c_int != 0;
                interactive = 0 as libc::c_int != 0;
            }
            105 => {
                remove_existing_files = 0 as libc::c_int != 0;
                interactive = 1 as libc::c_int != 0;
            }
            76 => {
                logical = 1 as libc::c_int != 0;
            }
            110 => {
                dereference_dest_dir_symlinks = 0 as libc::c_int != 0;
            }
            80 => {
                logical = 0 as libc::c_int != 0;
            }
            114 => {
                relative = 1 as libc::c_int != 0;
            }
            115 => {
                symbolic_link = 1 as libc::c_int != 0;
            }
            116 => {
                if !target_directory.is_null() {
                    if ::core::mem::size_of::<C2RustUnnamed_7>() as libc::c_ulong != 0 {
                        error(
                            1 as libc::c_int,
                            0 as libc::c_int,
                            dcgettext(
                                0 as *const libc::c_char,
                                b"multiple target directories specified\0" as *const u8
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
                                b"multiple target directories specified\0" as *const u8
                                    as *const libc::c_char,
                                5 as libc::c_int,
                            ),
                        );
                        if 0 as libc::c_int != 0 {} else {
                            unreachable!();
                        };
                    };
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
                    if stat(optarg, &mut st) != 0 as libc::c_int {
                        if ::core::mem::size_of::<C2RustUnnamed_6>() as libc::c_ulong
                            != 0
                        {
                            error(
                                1 as libc::c_int,
                                *__errno_location(),
                                dcgettext(
                                    0 as *const libc::c_char,
                                    b"failed to access %s\0" as *const u8
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
                                *__errno_location(),
                                dcgettext(
                                    0 as *const libc::c_char,
                                    b"failed to access %s\0" as *const u8
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
                    if !(st.st_mode & 0o170000 as libc::c_int as libc::c_uint
                        == 0o40000 as libc::c_int as libc::c_uint)
                    {
                        if ::core::mem::size_of::<C2RustUnnamed_5>() as libc::c_ulong
                            != 0
                        {
                            error(
                                1 as libc::c_int,
                                0 as libc::c_int,
                                dcgettext(
                                    0 as *const libc::c_char,
                                    b"target %s is not a directory\0" as *const u8
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
                                    b"target %s is not a directory\0" as *const u8
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
                target_directory = optarg;
            }
            84 => {
                no_target_directory = 1 as libc::c_int != 0;
            }
            118 => {
                verbose = 1 as libc::c_int != 0;
            }
            83 => {
                make_backups = 1 as libc::c_int != 0;
                backup_suffix = optarg;
            }
            -2 => {
                usage(0 as libc::c_int);
            }
            -3 => {
                version_etc(
                    stdout,
                    b"ln\0" as *const u8 as *const libc::c_char,
                    b"GNU coreutils\0" as *const u8 as *const libc::c_char,
                    Version,
                    b"Mike Parker\0" as *const u8 as *const libc::c_char,
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
    n_files = argc - optind;
    file = argv.offset(optind as isize);
    if n_files <= 0 as libc::c_int {
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
    if relative as libc::c_int != 0 && !symbolic_link {
        if ::core::mem::size_of::<C2RustUnnamed_4>() as libc::c_ulong != 0 {
            error(
                1 as libc::c_int,
                0 as libc::c_int,
                dcgettext(
                    0 as *const libc::c_char,
                    b"cannot do --relative without --symbolic\0" as *const u8
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
                    b"cannot do --relative without --symbolic\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
            );
            if 0 as libc::c_int != 0 {} else {
                unreachable!();
            };
        };
    }
    if !hard_dir_link {
        priv_set_remove_linkdir();
        beware_hard_dir_link = 1 as libc::c_int == 0;
    }
    if no_target_directory {
        if !target_directory.is_null() {
            if ::core::mem::size_of::<C2RustUnnamed_3>() as libc::c_ulong != 0 {
                error(
                    1 as libc::c_int,
                    0 as libc::c_int,
                    dcgettext(
                        0 as *const libc::c_char,
                        b"cannot combine --target-directory and --no-target-directory\0"
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
                        b"cannot combine --target-directory and --no-target-directory\0"
                            as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                );
                if 0 as libc::c_int != 0 {} else {
                    unreachable!();
                };
            };
        }
        if n_files != 2 as libc::c_int {
            if n_files < 2 as libc::c_int {
                error(
                    0 as libc::c_int,
                    0 as libc::c_int,
                    dcgettext(
                        0 as *const libc::c_char,
                        b"missing destination file operand after %s\0" as *const u8
                            as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    quotearg_style(
                        shell_escape_always_quoting_style,
                        *file.offset(0 as libc::c_int as isize),
                    ),
                );
            } else {
                error(
                    0 as libc::c_int,
                    0 as libc::c_int,
                    dcgettext(
                        0 as *const libc::c_char,
                        b"extra operand %s\0" as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    quotearg_style(
                        shell_escape_always_quoting_style,
                        *file.offset(2 as libc::c_int as isize),
                    ),
                );
            }
            usage(1 as libc::c_int);
        }
    } else if n_files < 2 as libc::c_int && target_directory.is_null() {
        target_directory = b".\0" as *const u8 as *const libc::c_char;
        destdir_fd = -(100 as libc::c_int);
    } else {
        if n_files == 2 as libc::c_int && target_directory.is_null() {
            link_errno = atomic_link(
                *file.offset(0 as libc::c_int as isize),
                -(100 as libc::c_int),
                *file.offset(1 as libc::c_int as isize),
            );
        }
        if link_errno < 0 as libc::c_int || link_errno == 17 as libc::c_int
            || link_errno == 20 as libc::c_int || link_errno == 22 as libc::c_int
        {
            let mut d: *const libc::c_char = if !target_directory.is_null() {
                target_directory
            } else {
                *file.offset((n_files - 1 as libc::c_int) as isize)
                    as *const libc::c_char
            };
            let mut flags: libc::c_int = O_PATHSEARCH as libc::c_int
                | 0o40000 as libc::c_int
                | (if dereference_dest_dir_symlinks as libc::c_int != 0 {
                    0 as libc::c_int
                } else {
                    0o100000 as libc::c_int
                });
            destdir_fd = openat_safer(-(100 as libc::c_int), d, flags);
            let mut err: libc::c_int = *__errno_location();
            if 0o40000 as libc::c_int == 0 && 0 as libc::c_int <= destdir_fd {
                let mut st_0: stat = stat {
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
                err = if fstat(destdir_fd, &mut st_0) != 0 as libc::c_int {
                    *__errno_location()
                } else if st_0.st_mode & 0o170000 as libc::c_int as libc::c_uint
                    == 0o40000 as libc::c_int as libc::c_uint
                {
                    0 as libc::c_int
                } else {
                    20 as libc::c_int
                };
                if err != 0 as libc::c_int {
                    close(destdir_fd);
                    destdir_fd = -(1 as libc::c_int);
                }
            }
            if 0 as libc::c_int <= destdir_fd {
                n_files -= target_directory.is_null() as libc::c_int;
                target_directory = d;
            } else if !(n_files == 2 as libc::c_int && target_directory.is_null()) {
                if ::core::mem::size_of::<C2RustUnnamed_2>() as libc::c_ulong != 0 {
                    error(
                        1 as libc::c_int,
                        err,
                        dcgettext(
                            0 as *const libc::c_char,
                            b"target %s\0" as *const u8 as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                        quotearg_style(shell_escape_always_quoting_style, d),
                    );
                    if 0 as libc::c_int != 0 {} else {
                        unreachable!();
                    };
                } else {
                    error(
                        1 as libc::c_int,
                        err,
                        dcgettext(
                            0 as *const libc::c_char,
                            b"target %s\0" as *const u8 as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                        quotearg_style(shell_escape_always_quoting_style, d),
                    );
                    if 0 as libc::c_int != 0 {} else {
                        unreachable!();
                    };
                };
            }
        }
    }
    backup_type = (if make_backups as libc::c_int != 0 {
        xget_version(
            dcgettext(
                0 as *const libc::c_char,
                b"backup type\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            version_control_string,
        ) as libc::c_uint
    } else {
        no_backups as libc::c_int as libc::c_uint
    }) as backup_type;
    set_simple_backup_suffix(backup_suffix);
    if !target_directory.is_null() {
        if 2 as libc::c_int <= n_files && remove_existing_files as libc::c_int != 0
            && !symbolic_link
            && backup_type as libc::c_uint
                != numbered_backups as libc::c_int as libc::c_uint
        {
            dest_set = hash_initialize(
                DEST_INFO_INITIAL_CAPACITY as libc::c_int as size_t,
                0 as *const Hash_tuning,
                Some(
                    triple_hash
                        as unsafe extern "C" fn(*const libc::c_void, size_t) -> size_t,
                ),
                Some(
                    triple_compare
                        as unsafe extern "C" fn(
                            *const libc::c_void,
                            *const libc::c_void,
                        ) -> bool,
                ),
                Some(triple_free as unsafe extern "C" fn(*mut libc::c_void) -> ()),
            );
            if dest_set.is_null() {
                xalloc_die();
            }
        }
        ok = 1 as libc::c_int != 0;
        let mut i: libc::c_int = 0 as libc::c_int;
        while i < n_files {
            let mut dest_base: *mut libc::c_char = 0 as *mut libc::c_char;
            let mut dest: *mut libc::c_char = file_name_concat(
                target_directory,
                last_component(*file.offset(i as isize)),
                &mut dest_base,
            );
            strip_trailing_slashes(dest_base);
            ok = (ok as libc::c_int
                & do_link(
                    *file.offset(i as isize),
                    destdir_fd,
                    dest_base,
                    dest,
                    -(1 as libc::c_int),
                ) as libc::c_int) as bool;
            free(dest as *mut libc::c_void);
            i += 1;
        }
    } else {
        ok = do_link(
            *file.offset(0 as libc::c_int as isize),
            -(100 as libc::c_int),
            *file.offset(1 as libc::c_int as isize),
            *file.offset(1 as libc::c_int as isize),
            link_errno,
        );
    }
    exit(if ok as libc::c_int != 0 { 0 as libc::c_int } else { 1 as libc::c_int });
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
