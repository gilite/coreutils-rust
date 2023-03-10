#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![feature(extern_types, label_break_value)]
extern "C" {
    pub type tm_zone;
    pub type __dirstream;
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type argv_iterator;
    pub type di_set;
    pub type exclude;
    pub type cycle_check_state;
    pub type hash_table;
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    fn getopt_long(
        ___argc: libc::c_int,
        ___argv: *const *mut libc::c_char,
        __shortopts: *const libc::c_char,
        __longopts: *const option,
        __longind: *mut libc::c_int,
    ) -> libc::c_int;
    static mut optarg: *mut libc::c_char;
    static mut optind: libc::c_int;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strncmp(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn __errno_location() -> *mut libc::c_int;
    fn stat(__file: *const libc::c_char, __buf: *mut stat) -> libc::c_int;
    fn localtime_rz(
        __tz: timezone_t,
        __timer: *const time_t,
        __result: *mut tm,
    ) -> *mut tm;
    fn tzalloc(__name: *const libc::c_char) -> timezone_t;
    fn atexit(__func: Option::<unsafe extern "C" fn() -> ()>) -> libc::c_int;
    fn exit(_: libc::c_int) -> !;
    fn getenv(__name: *const libc::c_char) -> *mut libc::c_char;
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
    fn xcalloc(n: size_t, s: size_t) -> *mut libc::c_void;
    fn xreallocarray(p: *mut libc::c_void, n: size_t, s: size_t) -> *mut libc::c_void;
    static mut stdin: *mut FILE;
    static mut stdout: *mut FILE;
    static mut stderr: *mut FILE;
    fn fflush_unlocked(__stream: *mut FILE) -> libc::c_int;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn __overflow(_: *mut FILE, _: libc::c_int) -> libc::c_int;
    fn fputs_unlocked(__s: *const libc::c_char, __stream: *mut FILE) -> libc::c_int;
    fn rpl_fclose(stream: *mut FILE) -> libc::c_int;
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
    fn imaxtostr(_: intmax_t, _: *mut libc::c_char) -> *mut libc::c_char;
    fn umaxtostr(_: uintmax_t, _: *mut libc::c_char) -> *mut libc::c_char;
    fn quotearg_style(s: quoting_style, arg: *const libc::c_char) -> *mut libc::c_char;
    fn quotearg_n_style_colon(
        n: libc::c_int,
        s: quoting_style,
        arg: *const libc::c_char,
    ) -> *mut libc::c_char;
    static mut argmatch_die: argmatch_exit_fn;
    fn quote(arg: *const libc::c_char) -> *const libc::c_char;
    fn __xargmatch_internal(
        context: *const libc::c_char,
        arg: *const libc::c_char,
        arglist: *const *const libc::c_char,
        vallist: *const libc::c_void,
        valsize: size_t,
        exit_fn: argmatch_exit_fn,
        allow_abbreviation: bool,
    ) -> ptrdiff_t;
    fn argv_iter_free(_: *mut argv_iterator);
    fn argv_iter_init_argv(argv: *mut *mut libc::c_char) -> *mut argv_iterator;
    fn argv_iter_init_stream(fp: *mut FILE) -> *mut argv_iterator;
    fn argv_iter(_: *mut argv_iterator, _: *mut argv_iter_err) -> *mut libc::c_char;
    fn argv_iter_n_args(_: *const argv_iterator) -> size_t;
    fn di_set_free(_: *mut di_set);
    fn di_set_alloc() -> *mut di_set;
    fn di_set_insert(_: *mut di_set, _: dev_t, _: ino_t) -> libc::c_int;
    fn di_set_lookup(dis: *mut di_set, dev: dev_t, ino: ino_t) -> libc::c_int;
    fn error(
        __status: libc::c_int,
        __errnum: libc::c_int,
        __format: *const libc::c_char,
        _: ...
    );
    fn new_exclude() -> *mut exclude;
    fn add_exclude(_: *mut exclude, _: *const libc::c_char, _: libc::c_int);
    fn add_exclude_file(
        _: Option::<
            unsafe extern "C" fn(*mut exclude, *const libc::c_char, libc::c_int) -> (),
        >,
        _: *mut exclude,
        _: *const libc::c_char,
        _: libc::c_int,
        _: libc::c_char,
    ) -> libc::c_int;
    fn excluded_file_name(_: *const exclude, _: *const libc::c_char) -> bool;
    fn fprintftime(
        fp: *mut FILE,
        fmt: *const libc::c_char,
        tm: *const tm,
        zone: timezone_t,
        nanoseconds: libc::c_int,
    ) -> size_t;
    fn xstrtoimax(
        _: *const libc::c_char,
        _: *mut *mut libc::c_char,
        _: libc::c_int,
        _: *mut intmax_t,
        _: *const libc::c_char,
    ) -> strtol_error;
    fn xstrtoumax(
        _: *const libc::c_char,
        _: *mut *mut libc::c_char,
        _: libc::c_int,
        _: *mut uintmax_t,
        _: *const libc::c_char,
    ) -> strtol_error;
    fn human_readable(
        _: uintmax_t,
        _: *mut libc::c_char,
        _: libc::c_int,
        _: uintmax_t,
        _: uintmax_t,
    ) -> *mut libc::c_char;
    fn human_options(
        _: *const libc::c_char,
        _: *mut libc::c_int,
        _: *mut uintmax_t,
    ) -> strtol_error;
    fn read_file_system_list(need_fs_type: bool) -> *mut mount_entry;
    fn free_mount_entry(entry: *mut mount_entry);
    fn freopen_safer(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: *mut FILE,
    ) -> *mut FILE;
    fn xfts_open(
        _: *const *mut libc::c_char,
        options: libc::c_int,
        _: Option::<
            unsafe extern "C" fn(*mut *const FTSENT, *mut *const FTSENT) -> libc::c_int,
        >,
    ) -> *mut FTS;
    fn rpl_fts_close(_: *mut FTS) -> libc::c_int;
    fn rpl_fts_read(_: *mut FTS) -> *mut FTSENT;
    fn rpl_fts_set(_: *mut FTS, _: *mut FTSENT, _: libc::c_int) -> libc::c_int;
    fn cycle_warning_required(fts: *const FTS, ent: *const FTSENT) -> bool;
    fn xstrtol_fatal(
        _: strtol_error,
        _: libc::c_int,
        _: libc::c_char,
        _: *const option,
        _: *const libc::c_char,
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
pub type __intmax_t = libc::c_long;
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
pub type ino_t = __ino_t;
pub type dev_t = __dev_t;
pub type time_t = __time_t;
pub type size_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tm {
    pub tm_sec: libc::c_int,
    pub tm_min: libc::c_int,
    pub tm_hour: libc::c_int,
    pub tm_mday: libc::c_int,
    pub tm_mon: libc::c_int,
    pub tm_year: libc::c_int,
    pub tm_wday: libc::c_int,
    pub tm_yday: libc::c_int,
    pub tm_isdst: libc::c_int,
    pub tm_gmtoff: libc::c_long,
    pub tm_zone: *const libc::c_char,
}
pub type ptrdiff_t = libc::c_long;
pub type timezone_t = *mut tm_zone;
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
pub type intmax_t = __intmax_t;
pub type uintmax_t = __uintmax_t;
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
pub type argmatch_exit_fn = Option::<unsafe extern "C" fn() -> ()>;
pub type argv_iter_err = libc::c_uint;
pub const AI_ERR_READ: argv_iter_err = 4;
pub const AI_ERR_MEM: argv_iter_err = 3;
pub const AI_ERR_EOF: argv_iter_err = 2;
pub const AI_ERR_OK: argv_iter_err = 1;
pub type strtol_error = libc::c_uint;
pub const LONGINT_INVALID: strtol_error = 4;
pub const LONGINT_INVALID_SUFFIX_CHAR_WITH_OVERFLOW: strtol_error = 3;
pub const LONGINT_INVALID_SUFFIX_CHAR: strtol_error = 2;
pub const LONGINT_OVERFLOW: strtol_error = 1;
pub const LONGINT_OK: strtol_error = 0;
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
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct mount_entry {
    pub me_devname: *mut libc::c_char,
    pub me_mountdir: *mut libc::c_char,
    pub me_mntroot: *mut libc::c_char,
    pub me_type: *mut libc::c_char,
    pub me_dev: dev_t,
    #[bitfield(name = "me_dummy", ty = "libc::c_uint", bits = "0..=0")]
    #[bitfield(name = "me_remote", ty = "libc::c_uint", bits = "1..=1")]
    #[bitfield(name = "me_type_malloced", ty = "libc::c_uint", bits = "2..=2")]
    pub me_dummy_me_remote_me_type_malloced: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 7],
    pub me_next: *mut mount_entry,
}
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
    pub fts_cycle: C2RustUnnamed_1,
    pub fts_fd_ring: I_ring,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_1 {
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct duinfo {
    pub size: uintmax_t,
    pub inodes: uintmax_t,
    pub tmax: timespec,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dulevel {
    pub ent: duinfo,
    pub subdir: duinfo,
}
pub type time_type = libc::c_uint;
pub const time_atime: time_type = 2;
pub const time_ctime: time_type = 1;
pub const time_mtime: time_type = 0;
pub type C2RustUnnamed_2 = libc::c_uint;
pub const INODES_OPTION: C2RustUnnamed_2 = 263;
pub const TIME_STYLE_OPTION: C2RustUnnamed_2 = 262;
pub const TIME_OPTION: C2RustUnnamed_2 = 261;
pub const FTS_DEBUG: C2RustUnnamed_2 = 260;
pub const HUMAN_SI_OPTION: C2RustUnnamed_2 = 259;
pub const FILES0_FROM_OPTION: C2RustUnnamed_2 = 258;
pub const EXCLUDE_OPTION: C2RustUnnamed_2 = 257;
pub const APPARENT_SIZE_OPTION: C2RustUnnamed_2 = 256;
pub type time_style = libc::c_uint;
pub const iso_time_style: time_style = 2;
pub const long_iso_time_style: time_style = 1;
pub const full_iso_time_style: time_style = 0;
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
#[inline]
unsafe extern "C" fn timespec_cmp(mut a: timespec, mut b: timespec) -> libc::c_int {
    return 2 as libc::c_int
        * ((a.tv_sec > b.tv_sec) as libc::c_int - (a.tv_sec < b.tv_sec) as libc::c_int)
        + ((a.tv_nsec > b.tv_nsec) as libc::c_int
            - (a.tv_nsec < b.tv_nsec) as libc::c_int);
}
#[inline]
unsafe extern "C" fn xnrealloc(
    mut p: *mut libc::c_void,
    mut n: size_t,
    mut s: size_t,
) -> *mut libc::c_void {
    return xreallocarray(p, n, s);
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
unsafe extern "C" fn ferror_unlocked(mut __stream: *mut FILE) -> libc::c_int {
    return ((*__stream)._flags & 0x20 as libc::c_int != 0 as libc::c_int) as libc::c_int;
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
unsafe extern "C" fn emit_size_note() {
    fputs_unlocked(
        dcgettext(
            0 as *const libc::c_char,
            b"\nThe SIZE argument is an integer and optional unit (example: 10K is 10*1024).\nUnits are K,M,G,T,P,E,Z,Y,R,Q (powers of 1024) or KB,MB,... (powers of 1000).\nBinary prefixes can be used, too: KiB=K, MiB=M, and so on.\n\0"
                as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        ),
        stdout,
    );
}
#[inline]
unsafe extern "C" fn emit_blocksize_note(mut program: *const libc::c_char) {
    printf(
        dcgettext(
            0 as *const libc::c_char,
            b"\nDisplay values are in units of the first available SIZE from --block-size,\nand the %s_BLOCK_SIZE, BLOCK_SIZE and BLOCKSIZE environment variables.\nOtherwise, units default to 1024 bytes (or 512 if POSIXLY_CORRECT is set).\n\0"
                as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        ),
        program,
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
unsafe extern "C" fn timetostr(
    mut t: time_t,
    mut buf: *mut libc::c_char,
) -> *mut libc::c_char {
    return if !((0 as libc::c_int as time_t) < -(1 as libc::c_int) as time_t) {
        imaxtostr(t, buf)
    } else {
        umaxtostr(t as uintmax_t, buf)
    };
}
#[inline]
unsafe extern "C" fn bad_cast(mut s: *const libc::c_char) -> *mut libc::c_char {
    return s as *mut libc::c_char;
}
#[inline]
unsafe extern "C" fn usable_st_size(mut sb: *const stat) -> bool {
    return (*sb).st_mode & 0o170000 as libc::c_int as libc::c_uint
        == 0o100000 as libc::c_int as libc::c_uint
        || (*sb).st_mode & 0o170000 as libc::c_int as libc::c_uint
            == 0o120000 as libc::c_int as libc::c_uint
        || ((*sb).st_mode).wrapping_sub((*sb).st_mode) != 0 || 0 as libc::c_int != 0;
}
#[inline]
unsafe extern "C" fn get_stat_atime(mut st: *const stat) -> timespec {
    return (*st).st_atim;
}
#[inline]
unsafe extern "C" fn get_stat_ctime(mut st: *const stat) -> timespec {
    return (*st).st_ctim;
}
#[inline]
unsafe extern "C" fn get_stat_mtime(mut st: *const stat) -> timespec {
    return (*st).st_mtim;
}
static mut di_files: *mut di_set = 0 as *const di_set as *mut di_set;
static mut di_mnt: *mut di_set = 0 as *const di_set as *mut di_set;
static mut prev_level: size_t = 0;
#[inline]
unsafe extern "C" fn duinfo_init(mut a: *mut duinfo) {
    (*a).size = 0 as libc::c_int as uintmax_t;
    (*a).inodes = 0 as libc::c_int as uintmax_t;
    (*a)
        .tmax
        .tv_sec = !if (0 as libc::c_int as time_t) < -(1 as libc::c_int) as time_t {
        -(1 as libc::c_int) as time_t
    } else {
        (((1 as libc::c_int as time_t)
            << (::core::mem::size_of::<time_t>() as libc::c_ulong)
                .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                .wrapping_sub(2 as libc::c_int as libc::c_ulong))
            - 1 as libc::c_int as libc::c_long) * 2 as libc::c_int as libc::c_long
            + 1 as libc::c_int as libc::c_long
    };
    (*a).tmax.tv_nsec = -(1 as libc::c_int) as __syscall_slong_t;
}
#[inline]
unsafe extern "C" fn duinfo_set(
    mut a: *mut duinfo,
    mut size: uintmax_t,
    mut tmax: timespec,
) {
    (*a).size = size;
    (*a).inodes = 1 as libc::c_int as uintmax_t;
    (*a).tmax = tmax;
}
#[inline]
unsafe extern "C" fn duinfo_add(mut a: *mut duinfo, mut b: *const duinfo) {
    let mut sum: uintmax_t = ((*a).size).wrapping_add((*b).size);
    (*a)
        .size = if (*a).size <= sum {
        sum
    } else {
        18446744073709551615 as libc::c_ulong
    };
    (*a).inodes = ((*a).inodes).wrapping_add((*b).inodes);
    if timespec_cmp((*a).tmax, (*b).tmax) < 0 as libc::c_int {
        (*a).tmax = (*b).tmax;
    }
}
static mut opt_all: bool = 0 as libc::c_int != 0;
static mut apparent_size: bool = 0 as libc::c_int != 0;
static mut opt_count_all: bool = 0 as libc::c_int != 0;
static mut hash_all: bool = false;
static mut opt_nul_terminate_output: bool = 0 as libc::c_int != 0;
static mut print_grand_total: bool = 0 as libc::c_int != 0;
static mut opt_separate_dirs: bool = 0 as libc::c_int != 0;
static mut max_depth: size_t = 18446744073709551615 as libc::c_ulong;
static mut opt_threshold: intmax_t = 0 as libc::c_int as intmax_t;
static mut human_output_opts: libc::c_int = 0;
static mut opt_inodes: bool = 0 as libc::c_int != 0;
static mut opt_time: bool = 0 as libc::c_int != 0;
static mut time_type: time_type = time_mtime;
static mut time_style: *const libc::c_char = 0 as *const libc::c_char;
static mut time_format: *const libc::c_char = 0 as *const libc::c_char;
static mut localtz: timezone_t = 0 as *const tm_zone as *mut tm_zone;
static mut output_block_size: uintmax_t = 0;
static mut exclude: *mut exclude = 0 as *const exclude as *mut exclude;
static mut tot_dui: duinfo = duinfo {
    size: 0,
    inodes: 0,
    tmax: timespec { tv_sec: 0, tv_nsec: 0 },
};
static mut long_options: [option; 26] = [
    {
        let mut init = option {
            name: b"all\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'a' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"apparent-size\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: APPARENT_SIZE_OPTION as libc::c_int,
        };
        init
    },
    {
        let mut init = option {
            name: b"block-size\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'B' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"bytes\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'b' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"count-links\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'l' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"dereference\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'L' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"dereference-args\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'D' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"exclude\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: EXCLUDE_OPTION as libc::c_int,
        };
        init
    },
    {
        let mut init = option {
            name: b"exclude-from\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'X' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"files0-from\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: FILES0_FROM_OPTION as libc::c_int,
        };
        init
    },
    {
        let mut init = option {
            name: b"human-readable\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'h' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"inodes\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: INODES_OPTION as libc::c_int,
        };
        init
    },
    {
        let mut init = option {
            name: b"si\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: HUMAN_SI_OPTION as libc::c_int,
        };
        init
    },
    {
        let mut init = option {
            name: b"max-depth\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'd' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"null\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: '0' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"no-dereference\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'P' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"one-file-system\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'x' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"separate-dirs\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'S' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"summarize\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 's' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"total\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'c' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"threshold\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 't' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"time\0" as *const u8 as *const libc::c_char,
            has_arg: 2 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: TIME_OPTION as libc::c_int,
        };
        init
    },
    {
        let mut init = option {
            name: b"time-style\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: TIME_STYLE_OPTION as libc::c_int,
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
static mut time_args: [*const libc::c_char; 6] = [
    b"atime\0" as *const u8 as *const libc::c_char,
    b"access\0" as *const u8 as *const libc::c_char,
    b"use\0" as *const u8 as *const libc::c_char,
    b"ctime\0" as *const u8 as *const libc::c_char,
    b"status\0" as *const u8 as *const libc::c_char,
    0 as *const libc::c_char,
];
static mut time_types: [time_type; 5] = [
    time_atime,
    time_atime,
    time_atime,
    time_ctime,
    time_ctime,
];
static mut time_style_args: [*const libc::c_char; 4] = [
    b"full-iso\0" as *const u8 as *const libc::c_char,
    b"long-iso\0" as *const u8 as *const libc::c_char,
    b"iso\0" as *const u8 as *const libc::c_char,
    0 as *const libc::c_char,
];
static mut time_style_types: [time_style; 3] = [
    full_iso_time_style,
    long_iso_time_style,
    iso_time_style,
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
                b"Usage: %s [OPTION]... [FILE]...\n  or:  %s [OPTION]... --files0-from=F\n\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            program_name,
            program_name,
        );
        fputs_unlocked(
            dcgettext(
                0 as *const libc::c_char,
                b"Summarize device usage of the set of FILEs, recursively for directories.\n\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            stdout,
        );
        emit_mandatory_arg_note();
        fputs_unlocked(
            dcgettext(
                0 as *const libc::c_char,
                b"  -0, --null            end each output line with NUL, not newline\n  -a, --all             write counts for all files, not just directories\n      --apparent-size   print apparent sizes rather than device usage; although\n                          the apparent size is usually smaller, it may be\n                          larger due to holes in ('sparse') files, internal\n                          fragmentation, indirect blocks, and the like\n\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            stdout,
        );
        fputs_unlocked(
            dcgettext(
                0 as *const libc::c_char,
                b"  -B, --block-size=SIZE  scale sizes by SIZE before printing them; e.g.,\n                           '-BM' prints sizes in units of 1,048,576 bytes;\n                           see SIZE format below\n  -b, --bytes           equivalent to '--apparent-size --block-size=1'\n  -c, --total           produce a grand total\n  -D, --dereference-args  dereference only symlinks that are listed on the\n                          command line\n  -d, --max-depth=N     print the total for a directory (or file, with --all)\n                          only if it is N or fewer levels below the command\n                          line argument;  --max-depth=0 is the same as\n                          --summarize\n\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            stdout,
        );
        fputs_unlocked(
            dcgettext(
                0 as *const libc::c_char,
                b"      --files0-from=F   summarize device usage of the\n                          NUL-terminated file names specified in file F;\n                          if F is -, then read names from standard input\n  -H                    equivalent to --dereference-args (-D)\n  -h, --human-readable  print sizes in human readable format (e.g., 1K 234M 2G)\n      --inodes          list inode usage information instead of block usage\n\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            stdout,
        );
        fputs_unlocked(
            dcgettext(
                0 as *const libc::c_char,
                b"  -k                    like --block-size=1K\n  -L, --dereference     dereference all symbolic links\n  -l, --count-links     count sizes many times if hard linked\n  -m                    like --block-size=1M\n\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            stdout,
        );
        fputs_unlocked(
            dcgettext(
                0 as *const libc::c_char,
                b"  -P, --no-dereference  don't follow any symbolic links (this is the default)\n  -S, --separate-dirs   for directories do not include size of subdirectories\n      --si              like -h, but use powers of 1000 not 1024\n  -s, --summarize       display only a total for each argument\n\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            stdout,
        );
        fputs_unlocked(
            dcgettext(
                0 as *const libc::c_char,
                b"  -t, --threshold=SIZE  exclude entries smaller than SIZE if positive,\n                          or entries greater than SIZE if negative\n      --time            show time of the last modification of any file in the\n                          directory, or any of its subdirectories\n      --time=WORD       show time as WORD instead of modification time:\n                          atime, access, use, ctime or status\n      --time-style=STYLE  show times using STYLE, which can be:\n                            full-iso, long-iso, iso, or +FORMAT;\n                            FORMAT is interpreted like in 'date'\n\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            stdout,
        );
        fputs_unlocked(
            dcgettext(
                0 as *const libc::c_char,
                b"  -X, --exclude-from=FILE  exclude files that match any pattern in FILE\n      --exclude=PATTERN    exclude files that match PATTERN\n  -x, --one-file-system    skip directories on different file systems\n\0"
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
        emit_blocksize_note(b"DU\0" as *const u8 as *const libc::c_char);
        emit_size_note();
        emit_ancillary_info(b"du\0" as *const u8 as *const libc::c_char);
    }
    exit(status);
}
unsafe extern "C" fn hash_ins(
    mut di_set: *mut di_set,
    mut ino: ino_t,
    mut dev: dev_t,
) -> bool {
    let mut inserted: libc::c_int = di_set_insert(di_set, dev, ino);
    if inserted < 0 as libc::c_int {
        xalloc_die();
    }
    return inserted != 0;
}
unsafe extern "C" fn show_date(
    mut format: *const libc::c_char,
    mut when: timespec,
    mut tz: timezone_t,
) {
    let mut tm: tm = tm {
        tm_sec: 0,
        tm_min: 0,
        tm_hour: 0,
        tm_mday: 0,
        tm_mon: 0,
        tm_year: 0,
        tm_wday: 0,
        tm_yday: 0,
        tm_isdst: 0,
        tm_gmtoff: 0,
        tm_zone: 0 as *const libc::c_char,
    };
    if !(localtime_rz(tz, &mut when.tv_sec, &mut tm)).is_null() {
        fprintftime(stdout, format, &mut tm, tz, when.tv_nsec as libc::c_int);
    } else {
        let mut buf: [libc::c_char; 21] = [0; 21];
        let mut when_str: *mut libc::c_char = timetostr(when.tv_sec, buf.as_mut_ptr());
        error(
            0 as libc::c_int,
            0 as libc::c_int,
            dcgettext(
                0 as *const libc::c_char,
                b"time %s is out of range\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            quote(when_str),
        );
        fputs_unlocked(when_str, stdout);
    };
}
unsafe extern "C" fn print_only_size(mut n_bytes: uintmax_t) {
    let mut buf: [libc::c_char; 652] = [0; 652];
    fputs_unlocked(
        if n_bytes == 18446744073709551615 as libc::c_ulong {
            dcgettext(
                0 as *const libc::c_char,
                b"Infinity\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            )
        } else {
            human_readable(
                n_bytes,
                buf.as_mut_ptr(),
                human_output_opts,
                1 as libc::c_int as uintmax_t,
                output_block_size,
            )
        },
        stdout,
    );
}
unsafe extern "C" fn print_size(
    mut pdui: *const duinfo,
    mut string: *const libc::c_char,
) {
    print_only_size(
        if opt_inodes as libc::c_int != 0 { (*pdui).inodes } else { (*pdui).size },
    );
    if opt_time {
        putchar_unlocked('\t' as i32);
        show_date(time_format, (*pdui).tmax, localtz);
    }
    printf(
        b"\t%s%c\0" as *const u8 as *const libc::c_char,
        string,
        if opt_nul_terminate_output as libc::c_int != 0 {
            '\0' as i32
        } else {
            '\n' as i32
        },
    );
    fflush_unlocked(stdout);
}
unsafe extern "C" fn fill_mount_table() {
    let mut mnt_ent: *mut mount_entry = read_file_system_list(0 as libc::c_int != 0);
    while !mnt_ent.is_null() {
        let mut mnt_free: *mut mount_entry = 0 as *mut mount_entry;
        if (*mnt_ent).me_remote() == 0 && (*mnt_ent).me_dummy() == 0 {
            let mut buf: stat = stat {
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
            if stat((*mnt_ent).me_mountdir, &mut buf) == 0 {
                hash_ins(di_mnt, buf.st_ino, buf.st_dev);
            }
        }
        mnt_free = mnt_ent;
        mnt_ent = (*mnt_ent).me_next;
        free_mount_entry(mnt_free);
    }
}
unsafe extern "C" fn mount_point_in_fts_cycle(mut ent: *const FTSENT) -> bool {
    let mut cycle_ent: *const FTSENT = (*ent).fts_cycle;
    if di_mnt.is_null() {
        di_mnt = di_set_alloc();
        if di_mnt.is_null() {
            xalloc_die();
        }
        fill_mount_table();
    }
    while !ent.is_null() && ent != cycle_ent {
        if di_set_lookup(
            di_mnt,
            (*((*ent).fts_statp).as_ptr()).st_dev,
            (*((*ent).fts_statp).as_ptr()).st_ino,
        ) > 0 as libc::c_int
        {
            return 1 as libc::c_int != 0;
        }
        ent = (*ent).fts_parent;
    }
    return 0 as libc::c_int != 0;
}
unsafe extern "C" fn process_file(mut fts: *mut FTS, mut ent: *mut FTSENT) -> bool {
    let mut ok: bool = 1 as libc::c_int != 0;
    let mut dui: duinfo = duinfo {
        size: 0,
        inodes: 0,
        tmax: timespec { tv_sec: 0, tv_nsec: 0 },
    };
    let mut dui_to_print: duinfo = duinfo {
        size: 0,
        inodes: 0,
        tmax: timespec { tv_sec: 0, tv_nsec: 0 },
    };
    let mut level: size_t = 0;
    static mut n_alloc: size_t = 0;
    static mut dulvl: *mut dulevel = 0 as *const dulevel as *mut dulevel;
    let mut file: *const libc::c_char = (*ent).fts_path;
    let mut sb: *const stat = ((*ent).fts_statp).as_mut_ptr();
    let mut info: libc::c_int = (*ent).fts_info as libc::c_int;
    if info == 4 as libc::c_int {
        error(
            0 as libc::c_int,
            (*ent).fts_errno,
            dcgettext(
                0 as *const libc::c_char,
                b"cannot read directory %s\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            quotearg_style(shell_escape_always_quoting_style, file),
        );
        ok = 0 as libc::c_int != 0;
    } else if info != 6 as libc::c_int {
        let mut excluded: bool = excluded_file_name(exclude, file);
        if !excluded {
            if info == 11 as libc::c_int {
                rpl_fts_set(fts, ent, 1 as libc::c_int);
                let mut e: *const FTSENT = rpl_fts_read(fts);
                if e == ent as *const FTSENT {} else {
                    __assert_fail(
                        b"e == ent\0" as *const u8 as *const libc::c_char,
                        b"src/du.c\0" as *const u8 as *const libc::c_char,
                        527 as libc::c_int as libc::c_uint,
                        (*::core::mem::transmute::<
                            &[u8; 36],
                            &[libc::c_char; 36],
                        >(b"_Bool process_file(FTS *, FTSENT *)\0"))
                            .as_ptr(),
                    );
                }
                info = (*ent).fts_info as libc::c_int;
            }
            if info == 10 as libc::c_int || info == 13 as libc::c_int {
                error(
                    0 as libc::c_int,
                    (*ent).fts_errno,
                    dcgettext(
                        0 as *const libc::c_char,
                        b"cannot access %s\0" as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    quotearg_style(shell_escape_always_quoting_style, file),
                );
                return 0 as libc::c_int != 0;
            }
            if (*fts).fts_options & 0x40 as libc::c_int != 0
                && (0 as libc::c_int as libc::c_long) < (*ent).fts_level
                && (*fts).fts_dev != (*sb).st_dev
            {
                excluded = 1 as libc::c_int != 0;
            }
        }
        if excluded as libc::c_int != 0
            || !opt_count_all
                && (hash_all as libc::c_int != 0
                    || !((*sb).st_mode & 0o170000 as libc::c_int as libc::c_uint
                        == 0o40000 as libc::c_int as libc::c_uint)
                        && (1 as libc::c_int as libc::c_uint) < (*sb).st_nlink)
                && !hash_ins(di_files, (*sb).st_ino, (*sb).st_dev)
        {
            if info == 1 as libc::c_int {
                rpl_fts_set(fts, ent, 4 as libc::c_int);
                let mut e_0: *const FTSENT = rpl_fts_read(fts);
                if e_0 == ent as *const FTSENT {} else {
                    __assert_fail(
                        b"e == ent\0" as *const u8 as *const libc::c_char,
                        b"src/du.c\0" as *const u8 as *const libc::c_char,
                        560 as libc::c_int as libc::c_uint,
                        (*::core::mem::transmute::<
                            &[u8; 36],
                            &[libc::c_char; 36],
                        >(b"_Bool process_file(FTS *, FTSENT *)\0"))
                            .as_ptr(),
                    );
                }
            }
            return 1 as libc::c_int != 0;
        }
        match info {
            1 => return 1 as libc::c_int != 0,
            7 => {
                error(
                    0 as libc::c_int,
                    (*ent).fts_errno,
                    b"%s\0" as *const u8 as *const libc::c_char,
                    quotearg_n_style_colon(
                        0 as libc::c_int,
                        shell_escape_quoting_style,
                        file,
                    ),
                );
                ok = 0 as libc::c_int != 0;
            }
            2 => {
                if cycle_warning_required(fts, ent) as libc::c_int != 0
                    && !mount_point_in_fts_cycle(ent)
                {
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
                            file,
                        ),
                    );
                    return 0 as libc::c_int != 0;
                }
                return 1 as libc::c_int != 0;
            }
            _ => {}
        }
    }
    duinfo_set(
        &mut dui,
        if apparent_size as libc::c_int != 0 {
            (if usable_st_size(sb) as libc::c_int != 0 {
                if 0 as libc::c_int as libc::c_long > (*sb).st_size {
                    0 as libc::c_int as libc::c_long
                } else {
                    (*sb).st_size
                }
            } else {
                0 as libc::c_int as libc::c_long
            }) as libc::c_ulong
        } else {
            ((*sb).st_blocks as uintmax_t)
                .wrapping_mul(512 as libc::c_int as libc::c_ulong)
        },
        if time_type as libc::c_uint == time_mtime as libc::c_int as libc::c_uint {
            get_stat_mtime(sb)
        } else if time_type as libc::c_uint == time_atime as libc::c_int as libc::c_uint
        {
            get_stat_atime(sb)
        } else {
            get_stat_ctime(sb)
        },
    );
    level = (*ent).fts_level as size_t;
    dui_to_print = dui;
    if n_alloc == 0 as libc::c_int as libc::c_ulong {
        n_alloc = level.wrapping_add(10 as libc::c_int as libc::c_ulong);
        dulvl = xcalloc(n_alloc, ::core::mem::size_of::<dulevel>() as libc::c_ulong)
            as *mut dulevel;
    } else if !(level == prev_level) {
        if level > prev_level {
            if n_alloc <= level {
                dulvl = xnrealloc(
                    dulvl as *mut libc::c_void,
                    level,
                    (2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(::core::mem::size_of::<dulevel>() as libc::c_ulong),
                ) as *mut dulevel;
                n_alloc = level.wrapping_mul(2 as libc::c_int as libc::c_ulong);
            }
            let mut i: size_t = prev_level
                .wrapping_add(1 as libc::c_int as libc::c_ulong);
            while i <= level {
                duinfo_init(&mut (*dulvl.offset(i as isize)).ent);
                duinfo_init(&mut (*dulvl.offset(i as isize)).subdir);
                i = i.wrapping_add(1);
            }
        } else {
            if level == prev_level.wrapping_sub(1 as libc::c_int as libc::c_ulong)
            {} else {
                __assert_fail(
                    b"level == prev_level - 1\0" as *const u8 as *const libc::c_char,
                    b"src/du.c\0" as *const u8 as *const libc::c_char,
                    638 as libc::c_int as libc::c_uint,
                    (*::core::mem::transmute::<
                        &[u8; 36],
                        &[libc::c_char; 36],
                    >(b"_Bool process_file(FTS *, FTSENT *)\0"))
                        .as_ptr(),
                );
            }
            duinfo_add(&mut dui_to_print, &mut (*dulvl.offset(prev_level as isize)).ent);
            if !opt_separate_dirs {
                duinfo_add(
                    &mut dui_to_print,
                    &mut (*dulvl.offset(prev_level as isize)).subdir,
                );
            }
            duinfo_add(
                &mut (*dulvl.offset(level as isize)).subdir,
                &mut (*dulvl.offset(prev_level as isize)).ent,
            );
            duinfo_add(
                &mut (*dulvl.offset(level as isize)).subdir,
                &mut (*dulvl.offset(prev_level as isize)).subdir,
            );
        }
    }
    prev_level = level;
    if !(opt_separate_dirs as libc::c_int != 0
        && (info == 6 as libc::c_int || info == 4 as libc::c_int))
    {
        duinfo_add(&mut (*dulvl.offset(level as isize)).ent, &mut dui);
    }
    duinfo_add(&mut tot_dui, &mut dui);
    if (info == 6 as libc::c_int || info == 4 as libc::c_int) && level <= max_depth
        || opt_all as libc::c_int != 0 && level <= max_depth
        || level == 0 as libc::c_int as libc::c_ulong
    {
        let mut v: uintmax_t = if opt_inodes as libc::c_int != 0 {
            dui_to_print.inodes
        } else {
            dui_to_print.size
        };
        if if opt_threshold < 0 as libc::c_int as libc::c_long {
            (v <= -opt_threshold as libc::c_ulong) as libc::c_int
        } else {
            (v >= opt_threshold as libc::c_ulong) as libc::c_int
        } != 0
        {
            print_size(&mut dui_to_print, file);
        }
    }
    return ok;
}
unsafe extern "C" fn du_files(
    mut files: *mut *mut libc::c_char,
    mut bit_flags: libc::c_int,
) -> bool {
    let mut ok: bool = 1 as libc::c_int != 0;
    if !(*files).is_null() {
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
                            b"fts_read failed: %s\0" as *const u8 as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                        quotearg_n_style_colon(
                            0 as libc::c_int,
                            shell_escape_quoting_style,
                            (*fts).fts_path,
                        ),
                    );
                    ok = 0 as libc::c_int != 0;
                }
                prev_level = 0 as libc::c_int as size_t;
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
    }
    return ok;
}
static mut prefix_len: size_t = 0;
unsafe fn main_0(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut cwd_only: [*mut libc::c_char; 2] = [0 as *mut libc::c_char; 2];
    let mut max_depth_specified: bool = 0 as libc::c_int != 0;
    let mut ok: bool = 1 as libc::c_int != 0;
    let mut files_from: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut bit_flags: libc::c_int = 0x8 as libc::c_int;
    let mut symlink_deref_bits: libc::c_int = 0x10 as libc::c_int;
    let mut opt_summarize_only: bool = 0 as libc::c_int != 0;
    cwd_only[0 as libc::c_int
        as usize] = bad_cast(b".\0" as *const u8 as *const libc::c_char);
    cwd_only[1 as libc::c_int as usize] = 0 as *mut libc::c_char;
    set_program_name(*argv.offset(0 as libc::c_int as isize));
    setlocale(6 as libc::c_int, b"\0" as *const u8 as *const libc::c_char);
    bindtextdomain(
        b"coreutils\0" as *const u8 as *const libc::c_char,
        b"/usr/local/share/locale\0" as *const u8 as *const libc::c_char,
    );
    textdomain(b"coreutils\0" as *const u8 as *const libc::c_char);
    atexit(Some(close_stdout as unsafe extern "C" fn() -> ()));
    exclude = new_exclude();
    human_options(
        getenv(b"DU_BLOCK_SIZE\0" as *const u8 as *const libc::c_char),
        &mut human_output_opts,
        &mut output_block_size,
    );
    loop {
        let mut oi: libc::c_int = -(1 as libc::c_int);
        let mut c: libc::c_int = getopt_long(
            argc,
            argv,
            b"0abd:chHklmst:xB:DLPSX:\0" as *const u8 as *const libc::c_char,
            long_options.as_ptr(),
            &mut oi,
        );
        if c == -(1 as libc::c_int) {
            break;
        }
        match c {
            48 => {
                opt_nul_terminate_output = 1 as libc::c_int != 0;
            }
            97 => {
                opt_all = 1 as libc::c_int != 0;
            }
            256 => {
                apparent_size = 1 as libc::c_int != 0;
            }
            98 => {
                apparent_size = 1 as libc::c_int != 0;
                human_output_opts = 0 as libc::c_int;
                output_block_size = 1 as libc::c_int as uintmax_t;
            }
            99 => {
                print_grand_total = 1 as libc::c_int != 0;
            }
            104 => {
                human_output_opts = human_autoscale as libc::c_int
                    | human_SI as libc::c_int | human_base_1024 as libc::c_int;
                output_block_size = 1 as libc::c_int as uintmax_t;
            }
            259 => {
                human_output_opts = human_autoscale as libc::c_int
                    | human_SI as libc::c_int;
                output_block_size = 1 as libc::c_int as uintmax_t;
            }
            107 => {
                human_output_opts = 0 as libc::c_int;
                output_block_size = 1024 as libc::c_int as uintmax_t;
            }
            100 => {
                let mut tmp: uintmax_t = 0;
                if xstrtoumax(
                    optarg,
                    0 as *mut *mut libc::c_char,
                    0 as libc::c_int,
                    &mut tmp,
                    b"\0" as *const u8 as *const libc::c_char,
                ) as libc::c_uint == LONGINT_OK as libc::c_int as libc::c_uint
                    && tmp <= 18446744073709551615 as libc::c_ulong
                {
                    max_depth_specified = 1 as libc::c_int != 0;
                    max_depth = tmp;
                } else {
                    error(
                        0 as libc::c_int,
                        0 as libc::c_int,
                        dcgettext(
                            0 as *const libc::c_char,
                            b"invalid maximum depth %s\0" as *const u8
                                as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                        quote(optarg),
                    );
                    ok = 0 as libc::c_int != 0;
                }
            }
            109 => {
                human_output_opts = 0 as libc::c_int;
                output_block_size = (1024 as libc::c_int * 1024 as libc::c_int)
                    as uintmax_t;
            }
            108 => {
                opt_count_all = 1 as libc::c_int != 0;
            }
            115 => {
                opt_summarize_only = 1 as libc::c_int != 0;
            }
            116 => {
                let mut e: strtol_error = LONGINT_OK;
                e = xstrtoimax(
                    optarg,
                    0 as *mut *mut libc::c_char,
                    0 as libc::c_int,
                    &mut opt_threshold,
                    b"kKmMGTPEZYRQ0\0" as *const u8 as *const libc::c_char,
                );
                if e as libc::c_uint != LONGINT_OK as libc::c_int as libc::c_uint {
                    xstrtol_fatal(
                        e,
                        oi,
                        c as libc::c_char,
                        long_options.as_ptr(),
                        optarg,
                    );
                }
                if opt_threshold == 0 as libc::c_int as libc::c_long
                    && *optarg as libc::c_int == '-' as i32
                {
                    if ::core::mem::size_of::<C2RustUnnamed_5>() as libc::c_ulong != 0 {
                        error(
                            1 as libc::c_int,
                            0 as libc::c_int,
                            dcgettext(
                                0 as *const libc::c_char,
                                b"invalid --threshold argument '-0'\0" as *const u8
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
                                b"invalid --threshold argument '-0'\0" as *const u8
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
            120 => {
                bit_flags |= 0x40 as libc::c_int;
            }
            66 => {
                let mut e_0: strtol_error = human_options(
                    optarg,
                    &mut human_output_opts,
                    &mut output_block_size,
                );
                if e_0 as libc::c_uint != LONGINT_OK as libc::c_int as libc::c_uint {
                    xstrtol_fatal(
                        e_0,
                        oi,
                        c as libc::c_char,
                        long_options.as_ptr(),
                        optarg,
                    );
                }
            }
            72 | 68 => {
                symlink_deref_bits = 0x1 as libc::c_int | 0x10 as libc::c_int;
            }
            76 => {
                symlink_deref_bits = 0x2 as libc::c_int;
            }
            80 => {
                symlink_deref_bits = 0x10 as libc::c_int;
            }
            83 => {
                opt_separate_dirs = 1 as libc::c_int != 0;
            }
            88 => {
                if add_exclude_file(
                    Some(
                        add_exclude
                            as unsafe extern "C" fn(
                                *mut exclude,
                                *const libc::c_char,
                                libc::c_int,
                            ) -> (),
                    ),
                    exclude,
                    optarg,
                    (1 as libc::c_int) << 28 as libc::c_int,
                    '\n' as i32 as libc::c_char,
                ) != 0
                {
                    error(
                        0 as libc::c_int,
                        *__errno_location(),
                        b"%s\0" as *const u8 as *const libc::c_char,
                        quotearg_n_style_colon(
                            0 as libc::c_int,
                            shell_escape_quoting_style,
                            optarg,
                        ),
                    );
                    ok = 0 as libc::c_int != 0;
                }
            }
            258 => {
                files_from = optarg;
            }
            257 => {
                add_exclude(exclude, optarg, (1 as libc::c_int) << 28 as libc::c_int);
            }
            263 => {
                opt_inodes = 1 as libc::c_int != 0;
            }
            261 => {
                opt_time = 1 as libc::c_int != 0;
                time_type = (if !optarg.is_null() {
                    time_types[__xargmatch_internal(
                        b"--time\0" as *const u8 as *const libc::c_char,
                        optarg,
                        time_args.as_ptr(),
                        time_types.as_ptr() as *const libc::c_void,
                        ::core::mem::size_of::<time_type>() as libc::c_ulong,
                        argmatch_die,
                        1 as libc::c_int != 0,
                    ) as usize] as libc::c_uint
                } else {
                    time_mtime as libc::c_int as libc::c_uint
                }) as time_type;
                localtz = tzalloc(getenv(b"TZ\0" as *const u8 as *const libc::c_char));
            }
            262 => {
                time_style = optarg;
            }
            -2 => {
                usage(0 as libc::c_int);
            }
            -3 => {
                version_etc(
                    stdout,
                    b"du\0" as *const u8 as *const libc::c_char,
                    b"GNU coreutils\0" as *const u8 as *const libc::c_char,
                    Version,
                    b"Torbjorn Granlund\0" as *const u8 as *const libc::c_char,
                    b"David MacKenzie\0" as *const u8 as *const libc::c_char,
                    b"Paul Eggert\0" as *const u8 as *const libc::c_char,
                    b"Jim Meyering\0" as *const u8 as *const libc::c_char,
                    0 as *mut libc::c_void as *mut libc::c_char,
                );
                exit(0 as libc::c_int);
            }
            _ => {
                ok = 0 as libc::c_int != 0;
            }
        }
    }
    if !ok {
        usage(1 as libc::c_int);
    }
    if opt_all as libc::c_int != 0 && opt_summarize_only as libc::c_int != 0 {
        error(
            0 as libc::c_int,
            0 as libc::c_int,
            dcgettext(
                0 as *const libc::c_char,
                b"cannot both summarize and show all entries\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
        usage(1 as libc::c_int);
    }
    if opt_summarize_only as libc::c_int != 0 && max_depth_specified as libc::c_int != 0
        && max_depth == 0 as libc::c_int as libc::c_ulong
    {
        error(
            0 as libc::c_int,
            0 as libc::c_int,
            dcgettext(
                0 as *const libc::c_char,
                b"warning: summarizing is the same as using --max-depth=0\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
    }
    if opt_summarize_only as libc::c_int != 0 && max_depth_specified as libc::c_int != 0
        && max_depth != 0 as libc::c_int as libc::c_ulong
    {
        let mut d: libc::c_ulong = max_depth;
        error(
            0 as libc::c_int,
            0 as libc::c_int,
            dcgettext(
                0 as *const libc::c_char,
                b"warning: summarizing conflicts with --max-depth=%lu\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
            d,
        );
        usage(1 as libc::c_int);
    }
    if opt_summarize_only {
        max_depth = 0 as libc::c_int as size_t;
    }
    if opt_inodes {
        if apparent_size {
            error(
                0 as libc::c_int,
                0 as libc::c_int,
                dcgettext(
                    0 as *const libc::c_char,
                    b"warning: options --apparent-size and -b are ineffective with --inodes\0"
                        as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
            );
        }
        output_block_size = 1 as libc::c_int as uintmax_t;
    }
    if opt_time {
        if time_style.is_null() {
            time_style = getenv(b"TIME_STYLE\0" as *const u8 as *const libc::c_char);
            if time_style.is_null()
                || strcmp(time_style, b"locale\0" as *const u8 as *const libc::c_char)
                    == 0 as libc::c_int
            {
                time_style = b"long-iso\0" as *const u8 as *const libc::c_char;
            } else if *time_style as libc::c_int == '+' as i32 {
                let mut p: *mut libc::c_char = strchr(time_style, '\n' as i32);
                if !p.is_null() {
                    *p = '\0' as i32 as libc::c_char;
                }
            } else {
                static mut posix_prefix: [libc::c_char; 7] = unsafe {
                    *::core::mem::transmute::<&[u8; 7], &[libc::c_char; 7]>(b"posix-\0")
                };
                while strncmp(time_style, posix_prefix.as_ptr(), prefix_len)
                    == 0 as libc::c_int
                {
                    time_style = time_style.offset(prefix_len as isize);
                }
            }
        }
        if *time_style as libc::c_int == '+' as i32 {
            time_format = time_style.offset(1 as libc::c_int as isize);
        } else {
            match time_style_types[__xargmatch_internal(
                b"time style\0" as *const u8 as *const libc::c_char,
                time_style,
                time_style_args.as_ptr(),
                time_style_types.as_ptr() as *const libc::c_void,
                ::core::mem::size_of::<time_style>() as libc::c_ulong,
                argmatch_die,
                1 as libc::c_int != 0,
            ) as usize] as libc::c_uint
            {
                0 => {
                    time_format = b"%Y-%m-%d %H:%M:%S.%N %z\0" as *const u8
                        as *const libc::c_char;
                }
                1 => {
                    time_format = b"%Y-%m-%d %H:%M\0" as *const u8
                        as *const libc::c_char;
                }
                2 => {
                    time_format = b"%Y-%m-%d\0" as *const u8 as *const libc::c_char;
                }
                _ => {}
            }
        }
    }
    let mut ai: *mut argv_iterator = 0 as *mut argv_iterator;
    if !files_from.is_null() {
        if optind < argc {
            error(
                0 as libc::c_int,
                0 as libc::c_int,
                dcgettext(
                    0 as *const libc::c_char,
                    b"extra operand %s\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
                quote(*argv.offset(optind as isize)),
            );
            fprintf(
                stderr,
                b"%s\n\0" as *const u8 as *const libc::c_char,
                dcgettext(
                    0 as *const libc::c_char,
                    b"file operands cannot be combined with --files0-from\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
            );
            usage(1 as libc::c_int);
        }
        if !(strcmp(files_from, b"-\0" as *const u8 as *const libc::c_char)
            == 0 as libc::c_int
            || !(freopen_safer(
                files_from,
                b"r\0" as *const u8 as *const libc::c_char,
                stdin,
            ))
                .is_null())
        {
            if ::core::mem::size_of::<C2RustUnnamed_4>() as libc::c_ulong != 0 {
                error(
                    1 as libc::c_int,
                    *__errno_location(),
                    dcgettext(
                        0 as *const libc::c_char,
                        b"cannot open %s for reading\0" as *const u8
                            as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    quotearg_style(shell_escape_always_quoting_style, files_from),
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
                        b"cannot open %s for reading\0" as *const u8
                            as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    quotearg_style(shell_escape_always_quoting_style, files_from),
                );
                if 0 as libc::c_int != 0 {} else {
                    unreachable!();
                };
            };
        }
        ai = argv_iter_init_stream(stdin);
        hash_all = 1 as libc::c_int != 0;
    } else {
        let mut files: *mut *mut libc::c_char = if optind < argc {
            argv.offset(optind as isize)
        } else {
            cwd_only.as_mut_ptr()
        };
        ai = argv_iter_init_argv(files);
        hash_all = (optind + 1 as libc::c_int) < argc
            || symlink_deref_bits == 0x2 as libc::c_int;
    }
    if ai.is_null() {
        xalloc_die();
    }
    di_files = di_set_alloc();
    if di_files.is_null() {
        xalloc_die();
    }
    if opt_count_all as libc::c_int != 0 || !hash_all {
        bit_flags |= 0x100 as libc::c_int;
    }
    bit_flags |= symlink_deref_bits;
    static mut temp_argv: [*mut libc::c_char; 2] = [
        0 as *const libc::c_char as *mut libc::c_char,
        0 as *const libc::c_char as *mut libc::c_char,
    ];
    loop {
        let mut skip_file: bool = 0 as libc::c_int != 0;
        let mut ai_err: argv_iter_err = 0 as argv_iter_err;
        let mut file_name: *mut libc::c_char = argv_iter(ai, &mut ai_err);
        if file_name.is_null() {
            match ai_err as libc::c_uint {
                2 => {
                    break;
                }
                4 => {
                    error(
                        0 as libc::c_int,
                        *__errno_location(),
                        dcgettext(
                            0 as *const libc::c_char,
                            b"%s: read error\0" as *const u8 as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                        quotearg_n_style_colon(
                            0 as libc::c_int,
                            shell_escape_quoting_style,
                            files_from,
                        ),
                    );
                    ok = 0 as libc::c_int != 0;
                    break;
                }
                3 => {
                    xalloc_die();
                }
                _ => {}
            }
            if (b"unexpected error code from argv_iter\0" as *const u8
                as *const libc::c_char)
                .is_null()
            {} else {
                __assert_fail(
                    b"!\"unexpected error code from argv_iter\"\0" as *const u8
                        as *const libc::c_char,
                    b"src/du.c\0" as *const u8 as *const libc::c_char,
                    1081 as libc::c_int as libc::c_uint,
                    (*::core::mem::transmute::<
                        &[u8; 23],
                        &[libc::c_char; 23],
                    >(b"int main(int, char **)\0"))
                        .as_ptr(),
                );
            }
        }
        if !files_from.is_null()
            && strcmp(files_from, b"-\0" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
            && strcmp(file_name, b"-\0" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
        {
            error(
                0 as libc::c_int,
                0 as libc::c_int,
                dcgettext(
                    0 as *const libc::c_char,
                    b"when reading file names from stdin, no file name of %s allowed\0"
                        as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
                quotearg_style(shell_escape_always_quoting_style, file_name),
            );
            skip_file = 1 as libc::c_int != 0;
        }
        if *file_name.offset(0 as libc::c_int as isize) == 0 {
            if files_from.is_null() {
                error(
                    0 as libc::c_int,
                    0 as libc::c_int,
                    b"%s\0" as *const u8 as *const libc::c_char,
                    dcgettext(
                        0 as *const libc::c_char,
                        b"invalid zero-length file name\0" as *const u8
                            as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                );
            } else {
                let mut file_number: libc::c_ulong = argv_iter_n_args(ai);
                error(
                    0 as libc::c_int,
                    0 as libc::c_int,
                    b"%s:%lu: %s\0" as *const u8 as *const libc::c_char,
                    quotearg_n_style_colon(
                        0 as libc::c_int,
                        shell_escape_quoting_style,
                        files_from,
                    ),
                    file_number,
                    dcgettext(
                        0 as *const libc::c_char,
                        b"invalid zero-length file name\0" as *const u8
                            as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                );
            }
            skip_file = 1 as libc::c_int != 0;
        }
        if skip_file {
            ok = 0 as libc::c_int != 0;
        } else {
            temp_argv[0 as libc::c_int as usize] = file_name;
            ok = (ok as libc::c_int
                & du_files(temp_argv.as_mut_ptr(), bit_flags) as libc::c_int) as bool;
        }
    }
    argv_iter_free(ai);
    di_set_free(di_files);
    if !di_mnt.is_null() {
        di_set_free(di_mnt);
    }
    if !files_from.is_null()
        && (ferror_unlocked(stdin) != 0 || rpl_fclose(stdin) != 0 as libc::c_int)
        && ok as libc::c_int != 0
    {
        if ::core::mem::size_of::<C2RustUnnamed_3>() as libc::c_ulong != 0 {
            error(
                1 as libc::c_int,
                0 as libc::c_int,
                dcgettext(
                    0 as *const libc::c_char,
                    b"error reading %s\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
                quotearg_style(shell_escape_always_quoting_style, files_from),
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
                    b"error reading %s\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
                quotearg_style(shell_escape_always_quoting_style, files_from),
            );
            if 0 as libc::c_int != 0 {} else {
                unreachable!();
            };
        };
    }
    if print_grand_total {
        print_size(
            &mut tot_dui,
            dcgettext(
                0 as *const libc::c_char,
                b"total\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
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
unsafe extern "C" fn run_static_initializers() {
    prefix_len = (::core::mem::size_of::<[libc::c_char; 7]>() as libc::c_ulong)
        .wrapping_sub(1 as libc::c_int as libc::c_ulong);
}
#[used]
#[cfg_attr(target_os = "linux", link_section = ".init_array")]
#[cfg_attr(target_os = "windows", link_section = ".CRT$XIB")]
#[cfg_attr(target_os = "macos", link_section = "__DATA,__mod_init_func")]
static INIT_ARRAY: [unsafe extern "C" fn(); 1] = [run_static_initializers];
