#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![feature(extern_types, label_break_value)]
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type hash_table;
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
    fn rpl_asprintf(
        result: *mut *mut libc::c_char,
        format: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
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
    fn free(_: *mut libc::c_void);
    fn rpl_mbrtowc(
        pwc: *mut wchar_t,
        s: *const libc::c_char,
        n: size_t,
        ps: *mut mbstate_t,
    ) -> size_t;
    fn iswcntrl(__wc: wint_t) -> libc::c_int;
    fn close(__fd: libc::c_int) -> libc::c_int;
    fn fstat(__fd: libc::c_int, __buf: *mut stat) -> libc::c_int;
    fn isatty(__fd: libc::c_int) -> libc::c_int;
    fn sync();
    fn memmove(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strncmp(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn strdup(_: *const libc::c_char) -> *mut libc::c_char;
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strspn(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_ulong;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn __errno_location() -> *mut libc::c_int;
    fn atexit(__func: Option::<unsafe extern "C" fn() -> ()>) -> libc::c_int;
    fn exit(_: libc::c_int) -> !;
    fn getenv(__name: *const libc::c_char) -> *mut libc::c_char;
    fn stat(__file: *const libc::c_char, __buf: *mut stat) -> libc::c_int;
    fn quotearg_n_style_colon(
        n: libc::c_int,
        s: quoting_style,
        arg: *const libc::c_char,
    ) -> *mut libc::c_char;
    fn quotearg_style(s: quoting_style, arg: *const libc::c_char) -> *mut libc::c_char;
    static mut Version: *const libc::c_char;
    fn open(__file: *const libc::c_char, __oflag: libc::c_int, _: ...) -> libc::c_int;
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
    fn xreallocarray(p: *mut libc::c_void, n: size_t, s: size_t) -> *mut libc::c_void;
    fn xstrdup(str: *const libc::c_char) -> *mut libc::c_char;
    fn xnmalloc(n: size_t, s: size_t) -> *mut libc::c_void;
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
    fn canonicalize_file_name(__name: *const libc::c_char) -> *mut libc::c_char;
    fn canonicalize_filename_mode(
        _: *const libc::c_char,
        _: canonicalize_mode_t,
    ) -> *mut libc::c_char;
    fn error(
        __status: libc::c_int,
        __errnum: libc::c_int,
        __format: *const libc::c_char,
        _: ...
    );
    fn get_fs_usage(
        file: *const libc::c_char,
        disk: *const libc::c_char,
        fsp: *mut fs_usage,
    ) -> libc::c_int;
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
    fn ambsalign(
        src: *const libc::c_char,
        width: *mut size_t,
        align: mbs_align_t,
        flags: libc::c_int,
    ) -> *mut libc::c_char;
    fn gnu_mbswidth(string: *const libc::c_char, flags: libc::c_int) -> libc::c_int;
    fn read_file_system_list(need_fs_type: bool) -> *mut mount_entry;
    fn free_mount_entry(entry: *mut mount_entry);
    fn quote(arg: *const libc::c_char) -> *const libc::c_char;
    fn find_mount_point(_: *const libc::c_char, _: *const stat) -> *mut libc::c_char;
    fn hash_lookup(
        table_0: *const Hash_table,
        entry: *const libc::c_void,
    ) -> *mut libc::c_void;
    fn hash_free(table_0: *mut Hash_table);
    fn hash_initialize(
        candidate: size_t,
        tuning: *const Hash_tuning,
        hasher: Hash_hasher,
        comparator: Hash_comparator,
        data_freer: Hash_data_freer,
    ) -> *mut Hash_table;
    fn hash_insert(
        table_0: *mut Hash_table,
        entry: *const libc::c_void,
    ) -> *mut libc::c_void;
    fn xstrtol_fatal(
        _: strtol_error,
        _: libc::c_int,
        _: libc::c_char,
        _: *const option,
        _: *const libc::c_char,
    );
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
pub struct __mbstate_t {
    pub __count: libc::c_int,
    pub __value: C2RustUnnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub __wch: libc::c_uint,
    pub __wchb: [libc::c_char; 4],
}
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
pub type wchar_t = libc::c_uint;
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
pub type wint_t = libc::c_uint;
pub type mbstate_t = __mbstate_t;
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
pub type canonicalize_mode_t = libc::c_uint;
pub const CAN_NOLINKS: canonicalize_mode_t = 4;
pub const CAN_MISSING: canonicalize_mode_t = 2;
pub const CAN_ALL_BUT_LAST: canonicalize_mode_t = 1;
pub const CAN_EXISTING: canonicalize_mode_t = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct fs_usage {
    pub fsu_blocksize: uintmax_t,
    pub fsu_blocks: uintmax_t,
    pub fsu_bfree: uintmax_t,
    pub fsu_bavail: uintmax_t,
    pub fsu_bavail_top_bit_set: bool,
    pub fsu_files: uintmax_t,
    pub fsu_ffree: uintmax_t,
}
pub type strtol_error = libc::c_uint;
pub const LONGINT_INVALID: strtol_error = 4;
pub const LONGINT_INVALID_SUFFIX_CHAR_WITH_OVERFLOW: strtol_error = 3;
pub const LONGINT_INVALID_SUFFIX_CHAR: strtol_error = 2;
pub const LONGINT_OVERFLOW: strtol_error = 1;
pub const LONGINT_OK: strtol_error = 0;
pub type C2RustUnnamed_1 = libc::c_uint;
pub const human_B: C2RustUnnamed_1 = 256;
pub const human_SI: C2RustUnnamed_1 = 128;
pub const human_space_before_unit: C2RustUnnamed_1 = 64;
pub const human_base_1024: C2RustUnnamed_1 = 32;
pub const human_autoscale: C2RustUnnamed_1 = 16;
pub const human_suppress_point_zero: C2RustUnnamed_1 = 8;
pub const human_group_digits: C2RustUnnamed_1 = 4;
pub const human_floor: C2RustUnnamed_1 = 2;
pub const human_round_to_nearest: C2RustUnnamed_1 = 1;
pub const human_ceiling: C2RustUnnamed_1 = 0;
pub type mbs_align_t = libc::c_uint;
pub const MBS_ALIGN_CENTER: mbs_align_t = 2;
pub const MBS_ALIGN_RIGHT: mbs_align_t = 1;
pub const MBS_ALIGN_LEFT: mbs_align_t = 0;
pub type C2RustUnnamed_2 = libc::c_uint;
pub const MBA_NO_RIGHT_PAD: C2RustUnnamed_2 = 8;
pub const MBA_NO_LEFT_PAD: C2RustUnnamed_2 = 4;
pub const MBA_UNIBYTE_ONLY: C2RustUnnamed_2 = 2;
pub const MBA_UNIBYTE_FALLBACK: C2RustUnnamed_2 = 1;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct devlist {
    pub dev_num: dev_t,
    pub me: *mut mount_entry,
    pub next: *mut devlist,
    pub seen_last: *mut devlist,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct fs_type_list {
    pub fs_name: *mut libc::c_char,
    pub fs_next: *mut fs_type_list,
}
pub type C2RustUnnamed_3 = libc::c_uint;
pub const OUTPUT_MODE: C2RustUnnamed_3 = 4;
pub const POSIX_MODE: C2RustUnnamed_3 = 3;
pub const HUMAN_MODE: C2RustUnnamed_3 = 2;
pub const INODES_MODE: C2RustUnnamed_3 = 1;
pub const DEFAULT_MODE: C2RustUnnamed_3 = 0;
pub type display_field_t = libc::c_uint;
pub const INVALID_FIELD: display_field_t = 12;
pub const FILE_FIELD: display_field_t = 11;
pub const TARGET_FIELD: display_field_t = 10;
pub const IPCENT_FIELD: display_field_t = 9;
pub const IAVAIL_FIELD: display_field_t = 8;
pub const IUSED_FIELD: display_field_t = 7;
pub const ITOTAL_FIELD: display_field_t = 6;
pub const PCENT_FIELD: display_field_t = 5;
pub const AVAIL_FIELD: display_field_t = 4;
pub const USED_FIELD: display_field_t = 3;
pub const SIZE_FIELD: display_field_t = 2;
pub const FSTYPE_FIELD: display_field_t = 1;
pub const SOURCE_FIELD: display_field_t = 0;
pub type field_type_t = libc::c_uint;
pub const OTHER_FLD: field_type_t = 2;
pub const INODE_FLD: field_type_t = 1;
pub const BLOCK_FLD: field_type_t = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct field_data_t {
    pub field: display_field_t,
    pub arg: *const libc::c_char,
    pub field_type: field_type_t,
    pub caption: *const libc::c_char,
    pub width: size_t,
    pub align: mbs_align_t,
    pub used: bool,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct field_values_t {
    pub input_units: uintmax_t,
    pub output_units: uintmax_t,
    pub total: uintmax_t,
    pub available: uintmax_t,
    pub negate_available: bool,
    pub available_to_root: uintmax_t,
    pub used: uintmax_t,
    pub negate_used: bool,
}
pub type C2RustUnnamed_4 = libc::c_uint;
pub const OUTPUT_OPTION: C2RustUnnamed_4 = 259;
pub const TOTAL_OPTION: C2RustUnnamed_4 = 258;
pub const SYNC_OPTION: C2RustUnnamed_4 = 257;
pub const NO_SYNC_OPTION: C2RustUnnamed_4 = 256;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_5 {
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
unsafe extern "C" fn c_iscntrl(mut c: libc::c_int) -> bool {
    match c {
        7 | 8 | 12 | 10 | 13 | 9 | 11 | 0 | 1 | 2 | 3 | 4 | 5 | 6 | 14 | 15 | 16 | 17
        | 18 | 19 | 20 | 21 | 22 | 23 | 24 | 25 | 26 | 27 | 28 | 29 | 30 | 31 | 127 => {
            return 1 as libc::c_int != 0;
        }
        _ => return 0 as libc::c_int != 0,
    };
}
#[inline]
unsafe extern "C" fn to_uchar(mut ch: libc::c_char) -> libc::c_uchar {
    return ch as libc::c_uchar;
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
static mut devlist_table: *mut Hash_table = 0 as *const Hash_table as *mut Hash_table;
static mut show_all_fs: bool = false;
static mut show_local_fs: bool = false;
static mut show_listed_fs: bool = false;
static mut human_output_opts: libc::c_int = 0;
static mut output_block_size: uintmax_t = 0;
static mut file_systems_processed: bool = false;
static mut require_sync: bool = false;
static mut exit_status: libc::c_int = 0;
static mut fs_select_list: *mut fs_type_list = 0 as *const fs_type_list
    as *mut fs_type_list;
static mut fs_exclude_list: *mut fs_type_list = 0 as *const fs_type_list
    as *mut fs_type_list;
static mut mount_list: *mut mount_entry = 0 as *const mount_entry as *mut mount_entry;
static mut print_type: bool = false;
static mut print_grand_total: bool = false;
static mut grand_fsu: fs_usage = fs_usage {
    fsu_blocksize: 0,
    fsu_blocks: 0,
    fsu_bfree: 0,
    fsu_bavail: 0,
    fsu_bavail_top_bit_set: false,
    fsu_files: 0,
    fsu_ffree: 0,
};
static mut header_mode: libc::c_int = DEFAULT_MODE as libc::c_int;
static mut field_data: [field_data_t; 12] = [
    {
        let mut init = field_data_t {
            field: SOURCE_FIELD,
            arg: b"source\0" as *const u8 as *const libc::c_char,
            field_type: OTHER_FLD,
            caption: b"Filesystem\0" as *const u8 as *const libc::c_char,
            width: 14 as libc::c_int as size_t,
            align: MBS_ALIGN_LEFT,
            used: 0 as libc::c_int != 0,
        };
        init
    },
    {
        let mut init = field_data_t {
            field: FSTYPE_FIELD,
            arg: b"fstype\0" as *const u8 as *const libc::c_char,
            field_type: OTHER_FLD,
            caption: b"Type\0" as *const u8 as *const libc::c_char,
            width: 4 as libc::c_int as size_t,
            align: MBS_ALIGN_LEFT,
            used: 0 as libc::c_int != 0,
        };
        init
    },
    {
        let mut init = field_data_t {
            field: SIZE_FIELD,
            arg: b"size\0" as *const u8 as *const libc::c_char,
            field_type: BLOCK_FLD,
            caption: b"blocks\0" as *const u8 as *const libc::c_char,
            width: 5 as libc::c_int as size_t,
            align: MBS_ALIGN_RIGHT,
            used: 0 as libc::c_int != 0,
        };
        init
    },
    {
        let mut init = field_data_t {
            field: USED_FIELD,
            arg: b"used\0" as *const u8 as *const libc::c_char,
            field_type: BLOCK_FLD,
            caption: b"Used\0" as *const u8 as *const libc::c_char,
            width: 5 as libc::c_int as size_t,
            align: MBS_ALIGN_RIGHT,
            used: 0 as libc::c_int != 0,
        };
        init
    },
    {
        let mut init = field_data_t {
            field: AVAIL_FIELD,
            arg: b"avail\0" as *const u8 as *const libc::c_char,
            field_type: BLOCK_FLD,
            caption: b"Available\0" as *const u8 as *const libc::c_char,
            width: 5 as libc::c_int as size_t,
            align: MBS_ALIGN_RIGHT,
            used: 0 as libc::c_int != 0,
        };
        init
    },
    {
        let mut init = field_data_t {
            field: PCENT_FIELD,
            arg: b"pcent\0" as *const u8 as *const libc::c_char,
            field_type: BLOCK_FLD,
            caption: b"Use%\0" as *const u8 as *const libc::c_char,
            width: 4 as libc::c_int as size_t,
            align: MBS_ALIGN_RIGHT,
            used: 0 as libc::c_int != 0,
        };
        init
    },
    {
        let mut init = field_data_t {
            field: ITOTAL_FIELD,
            arg: b"itotal\0" as *const u8 as *const libc::c_char,
            field_type: INODE_FLD,
            caption: b"Inodes\0" as *const u8 as *const libc::c_char,
            width: 5 as libc::c_int as size_t,
            align: MBS_ALIGN_RIGHT,
            used: 0 as libc::c_int != 0,
        };
        init
    },
    {
        let mut init = field_data_t {
            field: IUSED_FIELD,
            arg: b"iused\0" as *const u8 as *const libc::c_char,
            field_type: INODE_FLD,
            caption: b"IUsed\0" as *const u8 as *const libc::c_char,
            width: 5 as libc::c_int as size_t,
            align: MBS_ALIGN_RIGHT,
            used: 0 as libc::c_int != 0,
        };
        init
    },
    {
        let mut init = field_data_t {
            field: IAVAIL_FIELD,
            arg: b"iavail\0" as *const u8 as *const libc::c_char,
            field_type: INODE_FLD,
            caption: b"IFree\0" as *const u8 as *const libc::c_char,
            width: 5 as libc::c_int as size_t,
            align: MBS_ALIGN_RIGHT,
            used: 0 as libc::c_int != 0,
        };
        init
    },
    {
        let mut init = field_data_t {
            field: IPCENT_FIELD,
            arg: b"ipcent\0" as *const u8 as *const libc::c_char,
            field_type: INODE_FLD,
            caption: b"IUse%\0" as *const u8 as *const libc::c_char,
            width: 4 as libc::c_int as size_t,
            align: MBS_ALIGN_RIGHT,
            used: 0 as libc::c_int != 0,
        };
        init
    },
    {
        let mut init = field_data_t {
            field: TARGET_FIELD,
            arg: b"target\0" as *const u8 as *const libc::c_char,
            field_type: OTHER_FLD,
            caption: b"Mounted on\0" as *const u8 as *const libc::c_char,
            width: 0 as libc::c_int as size_t,
            align: MBS_ALIGN_LEFT,
            used: 0 as libc::c_int != 0,
        };
        init
    },
    {
        let mut init = field_data_t {
            field: FILE_FIELD,
            arg: b"file\0" as *const u8 as *const libc::c_char,
            field_type: OTHER_FLD,
            caption: b"File\0" as *const u8 as *const libc::c_char,
            width: 0 as libc::c_int as size_t,
            align: MBS_ALIGN_LEFT,
            used: 0 as libc::c_int != 0,
        };
        init
    },
];
static mut all_args_string: *const libc::c_char = b"source,fstype,itotal,iused,iavail,ipcent,size,used,avail,pcent,file,target\0"
    as *const u8 as *const libc::c_char;
static mut columns: *mut *mut field_data_t = 0 as *const *mut field_data_t
    as *mut *mut field_data_t;
static mut ncolumns: size_t = 0;
static mut table: *mut *mut *mut libc::c_char = 0 as *const *mut *mut libc::c_char
    as *mut *mut *mut libc::c_char;
static mut nrows: size_t = 0;
static mut long_options: [option; 17] = [
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
            name: b"block-size\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'B' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"inodes\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'i' as i32,
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
            name: b"si\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'H' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"local\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'l' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"output\0" as *const u8 as *const libc::c_char,
            has_arg: 2 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: OUTPUT_OPTION as libc::c_int,
        };
        init
    },
    {
        let mut init = option {
            name: b"portability\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'P' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"print-type\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'T' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"sync\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: SYNC_OPTION as libc::c_int,
        };
        init
    },
    {
        let mut init = option {
            name: b"no-sync\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: NO_SYNC_OPTION as libc::c_int,
        };
        init
    },
    {
        let mut init = option {
            name: b"total\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: TOTAL_OPTION as libc::c_int,
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
            name: b"exclude-type\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'x' as i32,
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
unsafe extern "C" fn automount_stat_err(
    mut file: *const libc::c_char,
    mut st: *mut stat,
) -> libc::c_int {
    let mut fd: libc::c_int = open(
        file,
        0 as libc::c_int | 0o400 as libc::c_int | 0o4000 as libc::c_int,
    );
    if fd < 0 as libc::c_int {
        if *__errno_location() == 2 as libc::c_int
            || *__errno_location() == 20 as libc::c_int
        {
            return *__errno_location();
        }
        return if stat(file, st) == 0 as libc::c_int {
            0 as libc::c_int
        } else {
            *__errno_location()
        };
    } else {
        let mut err: libc::c_int = if fstat(fd, st) == 0 as libc::c_int {
            0 as libc::c_int
        } else {
            *__errno_location()
        };
        close(fd);
        return err;
    };
}
unsafe extern "C" fn replace_control_chars(mut cell: *mut libc::c_char) {
    let mut p: *mut libc::c_char = cell;
    while *p != 0 {
        if c_iscntrl(to_uchar(*p) as libc::c_int) {
            *p = '?' as i32 as libc::c_char;
        }
        p = p.offset(1);
    }
}
unsafe extern "C" fn replace_invalid_chars(mut cell: *mut libc::c_char) {
    let mut srcend: *mut libc::c_char = cell.offset(strlen(cell) as isize);
    let mut dst: *mut libc::c_char = cell;
    let mut mbstate: mbstate_t = {
        let mut init = __mbstate_t {
            __count: 0 as libc::c_int,
            __value: C2RustUnnamed { __wch: 0 },
        };
        init
    };
    let mut n: size_t = 0;
    let mut src: *mut libc::c_char = cell;
    while src != srcend {
        let mut wc: wchar_t = 0;
        let mut srcbytes: size_t = srcend.offset_from(src) as libc::c_long as size_t;
        n = rpl_mbrtowc(&mut wc, src, srcbytes, &mut mbstate);
        let mut ok: bool = n <= srcbytes;
        if ok {
            ok = iswcntrl(wc) == 0;
        } else {
            n = 1 as libc::c_int as size_t;
        }
        if ok {
            memmove(dst as *mut libc::c_void, src as *const libc::c_void, n);
            dst = dst.offset(n as isize);
        } else {
            let fresh1 = dst;
            dst = dst.offset(1);
            *fresh1 = '?' as i32 as libc::c_char;
            memset(
                &mut mbstate as *mut mbstate_t as *mut libc::c_void,
                0 as libc::c_int,
                ::core::mem::size_of::<mbstate_t>() as libc::c_ulong,
            );
        }
        src = src.offset(n as isize);
    }
    *dst = '\0' as i32 as libc::c_char;
}
unsafe extern "C" fn replace_problematic_chars(mut cell: *mut libc::c_char) {
    static mut tty_out: libc::c_int = -(1 as libc::c_int);
    if tty_out < 0 as libc::c_int {
        tty_out = isatty(1 as libc::c_int);
    }
    if tty_out != 0 {
        Some(replace_invalid_chars as unsafe extern "C" fn(*mut libc::c_char) -> ())
    } else {
        Some(replace_control_chars as unsafe extern "C" fn(*mut libc::c_char) -> ())
    }
        .expect("non-null function pointer")(cell);
}
unsafe extern "C" fn alloc_table_row() {
    nrows = nrows.wrapping_add(1);
    table = xnrealloc(
        table as *mut libc::c_void,
        nrows,
        ::core::mem::size_of::<*mut *mut libc::c_char>() as libc::c_ulong,
    ) as *mut *mut *mut libc::c_char;
    let ref mut fresh2 = *table
        .offset(nrows.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize);
    *fresh2 = xnmalloc(
        ncolumns,
        ::core::mem::size_of::<*mut libc::c_char>() as libc::c_ulong,
    ) as *mut *mut libc::c_char;
}
unsafe extern "C" fn print_table() {
    let mut row: size_t = 0;
    row = 0 as libc::c_int as size_t;
    while row < nrows {
        let mut col: size_t = 0;
        col = 0 as libc::c_int as size_t;
        while col < ncolumns {
            let mut cell: *mut libc::c_char = *(*table.offset(row as isize))
                .offset(col as isize);
            if col != 0 as libc::c_int as libc::c_ulong {
                putchar_unlocked(' ' as i32);
            }
            let mut flags: libc::c_int = 0 as libc::c_int;
            if col == ncolumns.wrapping_sub(1 as libc::c_int as libc::c_ulong) {
                flags = MBA_NO_RIGHT_PAD as libc::c_int;
            }
            let mut width: size_t = (**columns.offset(col as isize)).width;
            cell = ambsalign(
                cell,
                &mut width,
                (**columns.offset(col as isize)).align,
                flags,
            );
            fputs_unlocked(
                if !cell.is_null() {
                    cell
                } else {
                    *(*table.offset(row as isize)).offset(col as isize)
                },
                stdout,
            );
            free(cell as *mut libc::c_void);
            col = col.wrapping_add(1);
        }
        putchar_unlocked('\n' as i32);
        row = row.wrapping_add(1);
    }
}
unsafe extern "C" fn alloc_field(mut f: libc::c_int, mut c: *const libc::c_char) {
    ncolumns = ncolumns.wrapping_add(1);
    columns = xnrealloc(
        columns as *mut libc::c_void,
        ncolumns,
        ::core::mem::size_of::<*mut field_data_t>() as libc::c_ulong,
    ) as *mut *mut field_data_t;
    let ref mut fresh3 = *columns
        .offset(ncolumns.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize);
    *fresh3 = &mut *field_data.as_mut_ptr().offset(f as isize) as *mut field_data_t;
    if !c.is_null() {
        let ref mut fresh4 = (**columns
            .offset(ncolumns.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize))
            .caption;
        *fresh4 = c;
    }
    if field_data[f as usize].used {
        if (b"field used\0" as *const u8 as *const libc::c_char).is_null() {} else {
            __assert_fail(
                b"!\"field used\"\0" as *const u8 as *const libc::c_char,
                b"src/df.c\0" as *const u8 as *const libc::c_char,
                425 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 36],
                    &[libc::c_char; 36],
                >(b"void alloc_field(int, const char *)\0"))
                    .as_ptr(),
            );
        }
    }
    field_data[f as usize].used = 1 as libc::c_int != 0;
}
unsafe extern "C" fn decode_output_arg(mut arg: *const libc::c_char) {
    let mut arg_writable: *mut libc::c_char = xstrdup(arg);
    let mut s: *mut libc::c_char = arg_writable;
    loop {
        let mut comma: *mut libc::c_char = strchr(s, ',' as i32);
        if !comma.is_null() {
            let fresh5 = comma;
            comma = comma.offset(1);
            *fresh5 = 0 as libc::c_int as libc::c_char;
        }
        let mut field: display_field_t = INVALID_FIELD;
        let mut i: libc::c_uint = 0 as libc::c_int as libc::c_uint;
        while (i as libc::c_ulong)
            < (::core::mem::size_of::<[field_data_t; 12]>() as libc::c_ulong)
                .wrapping_div(::core::mem::size_of::<field_data_t>() as libc::c_ulong)
        {
            if strcmp(field_data[i as usize].arg, s) == 0 as libc::c_int {
                field = i as display_field_t;
                break;
            } else {
                i = i.wrapping_add(1);
            }
        }
        if field as libc::c_uint == INVALID_FIELD as libc::c_int as libc::c_uint {
            error(
                0 as libc::c_int,
                0 as libc::c_int,
                dcgettext(
                    0 as *const libc::c_char,
                    b"option --output: field %s unknown\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
                quote(s),
            );
            usage(1 as libc::c_int);
        }
        if field_data[field as usize].used {
            error(
                0 as libc::c_int,
                0 as libc::c_int,
                dcgettext(
                    0 as *const libc::c_char,
                    b"option --output: field %s used more than once\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
                quote(field_data[field as usize].arg),
            );
            usage(1 as libc::c_int);
        }
        match field as libc::c_uint {
            0 | 1 | 3 | 5 | 6 | 7 | 8 | 9 | 10 | 11 => {
                alloc_field(field as libc::c_int, 0 as *const libc::c_char);
            }
            2 => {
                alloc_field(
                    field as libc::c_int,
                    b"Size\0" as *const u8 as *const libc::c_char,
                );
            }
            4 => {
                alloc_field(
                    field as libc::c_int,
                    b"Avail\0" as *const u8 as *const libc::c_char,
                );
            }
            _ => {
                if (b"invalid field\0" as *const u8 as *const libc::c_char).is_null()
                {} else {
                    __assert_fail(
                        b"!\"invalid field\"\0" as *const u8 as *const libc::c_char,
                        b"src/df.c\0" as *const u8 as *const libc::c_char,
                        496 as libc::c_int as libc::c_uint,
                        (*::core::mem::transmute::<
                            &[u8; 37],
                            &[libc::c_char; 37],
                        >(b"void decode_output_arg(const char *)\0"))
                            .as_ptr(),
                    );
                }
            }
        }
        s = comma;
        if s.is_null() {
            break;
        }
    }
    free(arg_writable as *mut libc::c_void);
}
unsafe extern "C" fn get_field_list() {
    match header_mode {
        0 => {
            alloc_field(SOURCE_FIELD as libc::c_int, 0 as *const libc::c_char);
            if print_type {
                alloc_field(FSTYPE_FIELD as libc::c_int, 0 as *const libc::c_char);
            }
            alloc_field(SIZE_FIELD as libc::c_int, 0 as *const libc::c_char);
            alloc_field(USED_FIELD as libc::c_int, 0 as *const libc::c_char);
            alloc_field(AVAIL_FIELD as libc::c_int, 0 as *const libc::c_char);
            alloc_field(PCENT_FIELD as libc::c_int, 0 as *const libc::c_char);
            alloc_field(TARGET_FIELD as libc::c_int, 0 as *const libc::c_char);
        }
        2 => {
            alloc_field(SOURCE_FIELD as libc::c_int, 0 as *const libc::c_char);
            if print_type {
                alloc_field(FSTYPE_FIELD as libc::c_int, 0 as *const libc::c_char);
            }
            alloc_field(
                SIZE_FIELD as libc::c_int,
                b"Size\0" as *const u8 as *const libc::c_char,
            );
            alloc_field(USED_FIELD as libc::c_int, 0 as *const libc::c_char);
            alloc_field(
                AVAIL_FIELD as libc::c_int,
                b"Avail\0" as *const u8 as *const libc::c_char,
            );
            alloc_field(PCENT_FIELD as libc::c_int, 0 as *const libc::c_char);
            alloc_field(TARGET_FIELD as libc::c_int, 0 as *const libc::c_char);
        }
        1 => {
            alloc_field(SOURCE_FIELD as libc::c_int, 0 as *const libc::c_char);
            if print_type {
                alloc_field(FSTYPE_FIELD as libc::c_int, 0 as *const libc::c_char);
            }
            alloc_field(ITOTAL_FIELD as libc::c_int, 0 as *const libc::c_char);
            alloc_field(IUSED_FIELD as libc::c_int, 0 as *const libc::c_char);
            alloc_field(IAVAIL_FIELD as libc::c_int, 0 as *const libc::c_char);
            alloc_field(IPCENT_FIELD as libc::c_int, 0 as *const libc::c_char);
            alloc_field(TARGET_FIELD as libc::c_int, 0 as *const libc::c_char);
        }
        3 => {
            alloc_field(SOURCE_FIELD as libc::c_int, 0 as *const libc::c_char);
            if print_type {
                alloc_field(FSTYPE_FIELD as libc::c_int, 0 as *const libc::c_char);
            }
            alloc_field(SIZE_FIELD as libc::c_int, 0 as *const libc::c_char);
            alloc_field(USED_FIELD as libc::c_int, 0 as *const libc::c_char);
            alloc_field(AVAIL_FIELD as libc::c_int, 0 as *const libc::c_char);
            alloc_field(
                PCENT_FIELD as libc::c_int,
                b"Capacity\0" as *const u8 as *const libc::c_char,
            );
            alloc_field(TARGET_FIELD as libc::c_int, 0 as *const libc::c_char);
        }
        4 => {
            if ncolumns == 0 {
                decode_output_arg(all_args_string);
            }
        }
        _ => {
            if (b"invalid header_mode\0" as *const u8 as *const libc::c_char).is_null()
            {} else {
                __assert_fail(
                    b"!\"invalid header_mode\"\0" as *const u8 as *const libc::c_char,
                    b"src/df.c\0" as *const u8 as *const libc::c_char,
                    565 as libc::c_int as libc::c_uint,
                    (*::core::mem::transmute::<
                        &[u8; 26],
                        &[libc::c_char; 26],
                    >(b"void get_field_list(void)\0"))
                        .as_ptr(),
                );
            }
        }
    };
}
unsafe extern "C" fn get_header() {
    let mut col: size_t = 0;
    alloc_table_row();
    col = 0 as libc::c_int as size_t;
    while col < ncolumns {
        let mut cell: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut header: *const libc::c_char = dcgettext(
            0 as *const libc::c_char,
            (**columns.offset(col as isize)).caption,
            5 as libc::c_int,
        );
        if (**columns.offset(col as isize)).field as libc::c_uint
            == SIZE_FIELD as libc::c_int as libc::c_uint
            && (header_mode == DEFAULT_MODE as libc::c_int
                || header_mode == OUTPUT_MODE as libc::c_int
                    && human_output_opts & human_autoscale as libc::c_int == 0)
        {
            let mut buf: [libc::c_char; 652] = [0; 652];
            let mut opts: libc::c_int = human_suppress_point_zero as libc::c_int
                | human_autoscale as libc::c_int | human_SI as libc::c_int
                | human_output_opts
                    & (human_group_digits as libc::c_int | human_base_1024 as libc::c_int
                        | human_B as libc::c_int);
            let mut q1000: uintmax_t = output_block_size;
            let mut q1024: uintmax_t = output_block_size;
            let mut divisible_by_1000: bool = false;
            let mut divisible_by_1024: bool = false;
            loop {
                divisible_by_1000 = q1000
                    .wrapping_rem(1000 as libc::c_int as libc::c_ulong)
                    == 0 as libc::c_int as libc::c_ulong;
                q1000 = (q1000 as libc::c_ulong)
                    .wrapping_div(1000 as libc::c_int as libc::c_ulong) as uintmax_t
                    as uintmax_t;
                divisible_by_1024 = q1024
                    .wrapping_rem(1024 as libc::c_int as libc::c_ulong)
                    == 0 as libc::c_int as libc::c_ulong;
                q1024 = (q1024 as libc::c_ulong)
                    .wrapping_div(1024 as libc::c_int as libc::c_ulong) as uintmax_t
                    as uintmax_t;
                if !(divisible_by_1000 as libc::c_int & divisible_by_1024 as libc::c_int
                    != 0)
                {
                    break;
                }
            }
            if (divisible_by_1000 as libc::c_int) < divisible_by_1024 as libc::c_int {
                opts |= human_base_1024 as libc::c_int;
            }
            if (divisible_by_1024 as libc::c_int) < divisible_by_1000 as libc::c_int {
                opts &= !(human_base_1024 as libc::c_int);
            }
            if opts & human_base_1024 as libc::c_int == 0 {
                opts |= human_B as libc::c_int;
            }
            let mut num: *mut libc::c_char = human_readable(
                output_block_size,
                buf.as_mut_ptr(),
                opts,
                1 as libc::c_int as uintmax_t,
                1 as libc::c_int as uintmax_t,
            );
            header = dcgettext(
                0 as *const libc::c_char,
                b"blocks\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            );
            if rpl_asprintf(
                &mut cell as *mut *mut libc::c_char,
                dcgettext(
                    0 as *const libc::c_char,
                    b"%s-%s\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
                num,
                header,
            ) == -(1 as libc::c_int)
            {
                cell = 0 as *mut libc::c_char;
            }
        } else if header_mode == POSIX_MODE as libc::c_int
            && (**columns.offset(col as isize)).field as libc::c_uint
                == SIZE_FIELD as libc::c_int as libc::c_uint
        {
            let mut buf_0: [libc::c_char; 21] = [0; 21];
            let mut num_0: *mut libc::c_char = umaxtostr(
                output_block_size,
                buf_0.as_mut_ptr(),
            );
            if rpl_asprintf(
                &mut cell as *mut *mut libc::c_char,
                dcgettext(
                    0 as *const libc::c_char,
                    b"%s-%s\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
                num_0,
                header,
            ) == -(1 as libc::c_int)
            {
                cell = 0 as *mut libc::c_char;
            }
        } else {
            cell = strdup(header);
        }
        if cell.is_null() {
            xalloc_die();
        }
        replace_problematic_chars(cell);
        let ref mut fresh6 = *(*table
            .offset(nrows.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize))
            .offset(col as isize);
        *fresh6 = cell;
        let mut cell_width: size_t = gnu_mbswidth(cell, 0 as libc::c_int) as size_t;
        (**columns.offset(col as isize))
            .width = if (**columns.offset(col as isize)).width > cell_width {
            (**columns.offset(col as isize)).width
        } else {
            cell_width
        };
        col = col.wrapping_add(1);
    }
}
unsafe extern "C" fn selected_fstype(mut fstype: *const libc::c_char) -> bool {
    let mut fsp: *const fs_type_list = 0 as *const fs_type_list;
    if fs_select_list.is_null() || fstype.is_null() {
        return 1 as libc::c_int != 0;
    }
    fsp = fs_select_list;
    while !fsp.is_null() {
        if strcmp(fstype, (*fsp).fs_name) == 0 as libc::c_int {
            return 1 as libc::c_int != 0;
        }
        fsp = (*fsp).fs_next;
    }
    return 0 as libc::c_int != 0;
}
unsafe extern "C" fn excluded_fstype(mut fstype: *const libc::c_char) -> bool {
    let mut fsp: *const fs_type_list = 0 as *const fs_type_list;
    if fs_exclude_list.is_null() || fstype.is_null() {
        return 0 as libc::c_int != 0;
    }
    fsp = fs_exclude_list;
    while !fsp.is_null() {
        if strcmp(fstype, (*fsp).fs_name) == 0 as libc::c_int {
            return 1 as libc::c_int != 0;
        }
        fsp = (*fsp).fs_next;
    }
    return 0 as libc::c_int != 0;
}
unsafe extern "C" fn devlist_hash(
    mut x: *const libc::c_void,
    mut table_size: size_t,
) -> size_t {
    let mut p: *const devlist = x as *const devlist;
    return ((*p).dev_num).wrapping_rem(table_size);
}
unsafe extern "C" fn devlist_compare(
    mut x: *const libc::c_void,
    mut y: *const libc::c_void,
) -> bool {
    let mut a: *const devlist = x as *const devlist;
    let mut b: *const devlist = y as *const devlist;
    return (*a).dev_num == (*b).dev_num;
}
unsafe extern "C" fn devlist_for_dev(mut dev: dev_t) -> *mut devlist {
    if devlist_table.is_null() {
        return 0 as *mut devlist;
    }
    let mut dev_entry: devlist = devlist {
        dev_num: 0,
        me: 0 as *mut mount_entry,
        next: 0 as *mut devlist,
        seen_last: 0 as *mut devlist,
    };
    dev_entry.dev_num = dev;
    let mut found: *mut devlist = hash_lookup(
        devlist_table,
        &mut dev_entry as *mut devlist as *const libc::c_void,
    ) as *mut devlist;
    if found.is_null() {
        return 0 as *mut devlist;
    }
    return (*found).seen_last;
}
unsafe extern "C" fn filter_mount_list(mut devices_only: bool) {
    let mut me: *mut mount_entry = 0 as *mut mount_entry;
    let mut device_list: *mut devlist = 0 as *mut devlist;
    let mut mount_list_size: libc::c_int = 0 as libc::c_int;
    me = mount_list;
    while !me.is_null() {
        mount_list_size += 1;
        me = (*me).me_next;
    }
    devlist_table = hash_initialize(
        mount_list_size as size_t,
        0 as *const Hash_tuning,
        Some(
            devlist_hash as unsafe extern "C" fn(*const libc::c_void, size_t) -> size_t,
        ),
        Some(
            devlist_compare
                as unsafe extern "C" fn(*const libc::c_void, *const libc::c_void) -> bool,
        ),
        None,
    );
    if devlist_table.is_null() {
        xalloc_die();
    }
    me = mount_list;
    while !me.is_null() {
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
        let mut discard_me: *mut mount_entry = 0 as *mut mount_entry;
        if (*me).me_remote() as libc::c_int != 0 && show_local_fs as libc::c_int != 0
            || (*me).me_dummy() as libc::c_int != 0 && !show_all_fs && !show_listed_fs
            || (!selected_fstype((*me).me_type)
                || excluded_fstype((*me).me_type) as libc::c_int != 0)
            || -(1 as libc::c_int) == stat((*me).me_mountdir, &mut buf)
        {
            buf.st_dev = (*me).me_dev;
        } else {
            let mut seen_dev: *mut devlist = devlist_for_dev(buf.st_dev);
            if !seen_dev.is_null() {
                let mut target_nearer_root: bool = strlen((*(*seen_dev).me).me_mountdir)
                    > strlen((*me).me_mountdir);
                let mut source_below_root: bool = !((*(*seen_dev).me).me_mntroot)
                    .is_null() && !((*me).me_mntroot).is_null()
                    && strlen((*(*seen_dev).me).me_mntroot) < strlen((*me).me_mntroot);
                if !(!print_grand_total && (*me).me_remote() as libc::c_int != 0
                    && (*(*seen_dev).me).me_remote() as libc::c_int != 0
                    && !(strcmp((*(*seen_dev).me).me_devname, (*me).me_devname)
                        == 0 as libc::c_int))
                {
                    if !(strchr((*me).me_devname, '/' as i32)).is_null()
                        && (strchr((*(*seen_dev).me).me_devname, '/' as i32)).is_null()
                        || target_nearer_root as libc::c_int != 0 && !source_below_root
                        || !(strcmp((*(*seen_dev).me).me_devname, (*me).me_devname)
                            == 0 as libc::c_int)
                            && strcmp((*me).me_mountdir, (*(*seen_dev).me).me_mountdir)
                                == 0 as libc::c_int
                    {
                        discard_me = (*seen_dev).me;
                        (*seen_dev).me = me;
                    } else {
                        discard_me = me;
                    }
                }
            }
        }
        if !discard_me.is_null() {
            me = (*me).me_next;
            if !devices_only {
                free_mount_entry(discard_me);
            }
        } else {
            let mut devlist: *mut devlist = xmalloc(
                ::core::mem::size_of::<devlist>() as libc::c_ulong,
            ) as *mut devlist;
            (*devlist).me = me;
            (*devlist).dev_num = buf.st_dev;
            (*devlist).next = device_list;
            device_list = devlist;
            let mut hash_entry: *mut devlist = hash_insert(
                devlist_table,
                devlist as *const libc::c_void,
            ) as *mut devlist;
            if hash_entry.is_null() {
                xalloc_die();
            }
            (*hash_entry).seen_last = devlist;
            me = (*me).me_next;
        }
    }
    if !devices_only {
        mount_list = 0 as *mut mount_entry;
        while !device_list.is_null() {
            me = (*device_list).me;
            (*me).me_next = mount_list;
            mount_list = me;
            let mut next: *mut devlist = (*device_list).next;
            free(device_list as *mut libc::c_void);
            device_list = next;
        }
        hash_free(devlist_table);
        devlist_table = 0 as *mut Hash_table;
    }
}
unsafe extern "C" fn me_for_dev(mut dev: dev_t) -> *const mount_entry {
    let mut dl: *mut devlist = devlist_for_dev(dev);
    if !dl.is_null() {
        return (*dl).me;
    }
    return 0 as *const mount_entry;
}
unsafe extern "C" fn known_value(mut n: uintmax_t) -> bool {
    return n
        < (18446744073709551615 as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong);
}
unsafe extern "C" fn df_readable(
    mut negative: bool,
    mut n: uintmax_t,
    mut buf: *mut libc::c_char,
    mut input_units: uintmax_t,
    mut output_units: uintmax_t,
) -> *const libc::c_char {
    if !known_value(n) && !negative {
        return b"-\0" as *const u8 as *const libc::c_char
    } else {
        let mut p: *mut libc::c_char = human_readable(
            if negative as libc::c_int != 0 { n.wrapping_neg() } else { n },
            buf.offset(negative as libc::c_int as isize),
            human_output_opts,
            input_units,
            output_units,
        );
        if negative {
            p = p.offset(-1);
            *p = '-' as i32 as libc::c_char;
        }
        return p;
    };
}
unsafe extern "C" fn add_uint_with_neg_flag(
    mut dest: *mut uintmax_t,
    mut dest_neg: *mut bool,
    mut src: uintmax_t,
    mut src_neg: bool,
) {
    if *dest_neg as libc::c_int == src_neg as libc::c_int {
        *dest = (*dest as libc::c_ulong).wrapping_add(src) as uintmax_t as uintmax_t;
        return;
    }
    if *dest_neg {
        *dest = (*dest).wrapping_neg();
    }
    if src_neg {
        src = src.wrapping_neg();
    }
    if src < *dest {
        *dest = (*dest as libc::c_ulong).wrapping_sub(src) as uintmax_t as uintmax_t;
    } else {
        *dest = src.wrapping_sub(*dest);
        *dest_neg = src_neg;
    }
    if *dest_neg {
        *dest = (*dest).wrapping_neg();
    }
}
unsafe extern "C" fn has_uuid_suffix(mut s: *const libc::c_char) -> bool {
    let mut len: size_t = strlen(s);
    return (36 as libc::c_int as libc::c_ulong) < len
        && strspn(
            s.offset(len as isize).offset(-(36 as libc::c_int as isize)),
            b"-0123456789abcdefABCDEF\0" as *const u8 as *const libc::c_char,
        ) == 36 as libc::c_int as libc::c_ulong;
}
unsafe extern "C" fn get_field_values(
    mut bv: *mut field_values_t,
    mut iv: *mut field_values_t,
    mut fsu: *const fs_usage,
) {
    (*iv).output_units = 1 as libc::c_int as uintmax_t;
    (*iv).input_units = (*iv).output_units;
    (*iv).total = (*fsu).fsu_files;
    (*iv).available_to_root = (*fsu).fsu_ffree;
    (*iv).available = (*iv).available_to_root;
    (*iv).negate_available = 0 as libc::c_int != 0;
    (*iv).used = 18446744073709551615 as libc::c_ulong;
    (*iv).negate_used = 0 as libc::c_int != 0;
    if known_value((*iv).total) as libc::c_int != 0
        && known_value((*iv).available_to_root) as libc::c_int != 0
    {
        (*iv).used = ((*iv).total).wrapping_sub((*iv).available_to_root);
        (*iv).negate_used = (*iv).total < (*iv).available_to_root;
    }
    (*bv).input_units = (*fsu).fsu_blocksize;
    (*bv).output_units = output_block_size;
    (*bv).total = (*fsu).fsu_blocks;
    (*bv).available = (*fsu).fsu_bavail;
    (*bv).available_to_root = (*fsu).fsu_bfree;
    (*bv)
        .negate_available = (*fsu).fsu_bavail_top_bit_set as libc::c_int != 0
        && known_value((*fsu).fsu_bavail) as libc::c_int != 0;
    (*bv).used = 18446744073709551615 as libc::c_ulong;
    (*bv).negate_used = 0 as libc::c_int != 0;
    if known_value((*bv).total) as libc::c_int != 0
        && known_value((*bv).available_to_root) as libc::c_int != 0
    {
        (*bv).used = ((*bv).total).wrapping_sub((*bv).available_to_root);
        (*bv).negate_used = (*bv).total < (*bv).available_to_root;
    }
}
unsafe extern "C" fn add_to_grand_total(
    mut bv: *mut field_values_t,
    mut iv: *mut field_values_t,
) {
    if known_value((*iv).total) {
        grand_fsu
            .fsu_files = (grand_fsu.fsu_files as libc::c_ulong).wrapping_add((*iv).total)
            as uintmax_t as uintmax_t;
    }
    if known_value((*iv).available) {
        grand_fsu
            .fsu_ffree = (grand_fsu.fsu_ffree as libc::c_ulong)
            .wrapping_add((*iv).available) as uintmax_t as uintmax_t;
    }
    if known_value((*bv).total) {
        grand_fsu
            .fsu_blocks = (grand_fsu.fsu_blocks as libc::c_ulong)
            .wrapping_add(((*bv).input_units).wrapping_mul((*bv).total)) as uintmax_t
            as uintmax_t;
    }
    if known_value((*bv).available_to_root) {
        grand_fsu
            .fsu_bfree = (grand_fsu.fsu_bfree as libc::c_ulong)
            .wrapping_add(((*bv).input_units).wrapping_mul((*bv).available_to_root))
            as uintmax_t as uintmax_t;
    }
    if known_value((*bv).available) {
        add_uint_with_neg_flag(
            &mut grand_fsu.fsu_bavail,
            &mut grand_fsu.fsu_bavail_top_bit_set,
            ((*bv).input_units).wrapping_mul((*bv).available),
            (*bv).negate_available,
        );
    }
}
unsafe extern "C" fn get_dev(
    mut device: *const libc::c_char,
    mut mount_point: *const libc::c_char,
    mut file: *const libc::c_char,
    mut stat_file: *const libc::c_char,
    mut fstype: *const libc::c_char,
    mut me_dummy: bool,
    mut me_remote: bool,
    mut force_fsu: *const fs_usage,
    mut process_all: bool,
) {
    if me_remote as libc::c_int != 0 && show_local_fs as libc::c_int != 0 {
        return;
    }
    if me_dummy as libc::c_int != 0 && !show_all_fs && !show_listed_fs {
        return;
    }
    if !selected_fstype(fstype) || excluded_fstype(fstype) as libc::c_int != 0 {
        return;
    }
    if force_fsu.is_null() && !mount_point.is_null()
        && !(*mount_point.offset(0 as libc::c_int as isize) as libc::c_int == '/' as i32)
    {
        return;
    }
    if stat_file.is_null() {
        stat_file = if !mount_point.is_null() { mount_point } else { device };
    }
    let mut fsu: fs_usage = fs_usage {
        fsu_blocksize: 0,
        fsu_blocks: 0,
        fsu_bfree: 0,
        fsu_bavail: 0,
        fsu_bavail_top_bit_set: false,
        fsu_files: 0,
        fsu_ffree: 0,
    };
    if !force_fsu.is_null() {
        fsu = *force_fsu;
    } else if get_fs_usage(stat_file, device, &mut fsu) != 0 {
        if process_all as libc::c_int != 0
            && (*__errno_location() == 13 as libc::c_int
                || *__errno_location() == 2 as libc::c_int)
        {
            if !show_all_fs {
                return;
            }
            fstype = b"-\0" as *const u8 as *const libc::c_char;
            fsu.fsu_bavail_top_bit_set = 0 as libc::c_int != 0;
            fsu.fsu_ffree = 18446744073709551615 as libc::c_ulong;
            fsu.fsu_files = fsu.fsu_ffree;
            fsu.fsu_bavail = fsu.fsu_files;
            fsu.fsu_bfree = fsu.fsu_bavail;
            fsu.fsu_blocks = fsu.fsu_bfree;
            fsu.fsu_blocksize = fsu.fsu_blocks;
        } else {
            error(
                0 as libc::c_int,
                *__errno_location(),
                b"%s\0" as *const u8 as *const libc::c_char,
                quotearg_n_style_colon(
                    0 as libc::c_int,
                    shell_escape_quoting_style,
                    stat_file,
                ),
            );
            exit_status = 1 as libc::c_int;
            return;
        }
    } else if process_all as libc::c_int != 0 && show_all_fs as libc::c_int != 0 {
        let mut sb: stat = stat {
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
        if stat(stat_file, &mut sb) == 0 as libc::c_int {
            let mut dev_me: *const mount_entry = me_for_dev(sb.st_dev);
            if !dev_me.is_null()
                && !(strcmp((*dev_me).me_devname, device) == 0 as libc::c_int)
                && ((*dev_me).me_remote() == 0 || !me_remote)
            {
                fstype = b"-\0" as *const u8 as *const libc::c_char;
                fsu.fsu_bavail_top_bit_set = 0 as libc::c_int != 0;
                fsu.fsu_ffree = 18446744073709551615 as libc::c_ulong;
                fsu.fsu_files = fsu.fsu_ffree;
                fsu.fsu_bavail = fsu.fsu_files;
                fsu.fsu_bfree = fsu.fsu_bavail;
                fsu.fsu_blocks = fsu.fsu_bfree;
                fsu.fsu_blocksize = fsu.fsu_blocks;
            }
        }
    }
    if fsu.fsu_blocks == 0 as libc::c_int as libc::c_ulong && !show_all_fs
        && !show_listed_fs
    {
        return;
    }
    if force_fsu.is_null() {
        file_systems_processed = 1 as libc::c_int != 0;
    }
    alloc_table_row();
    if device.is_null() {
        device = b"-\0" as *const u8 as *const libc::c_char;
    }
    if file.is_null() {
        file = b"-\0" as *const u8 as *const libc::c_char;
    }
    let mut dev_name: *mut libc::c_char = xstrdup(device);
    let mut resolved_dev: *mut libc::c_char = 0 as *mut libc::c_char;
    if process_all as libc::c_int != 0 && has_uuid_suffix(dev_name) as libc::c_int != 0
        && {
            resolved_dev = canonicalize_filename_mode(dev_name, CAN_EXISTING);
            !resolved_dev.is_null()
        }
    {
        free(dev_name as *mut libc::c_void);
        dev_name = resolved_dev;
    }
    if fstype.is_null() {
        fstype = b"-\0" as *const u8 as *const libc::c_char;
    }
    let mut block_values: field_values_t = field_values_t {
        input_units: 0,
        output_units: 0,
        total: 0,
        available: 0,
        negate_available: false,
        available_to_root: 0,
        used: 0,
        negate_used: false,
    };
    let mut inode_values: field_values_t = field_values_t {
        input_units: 0,
        output_units: 0,
        total: 0,
        available: 0,
        negate_available: false,
        available_to_root: 0,
        used: 0,
        negate_used: false,
    };
    get_field_values(&mut block_values, &mut inode_values, &mut fsu);
    if print_grand_total as libc::c_int != 0 && force_fsu.is_null() {
        add_to_grand_total(&mut block_values, &mut inode_values);
    }
    let mut col: size_t = 0;
    col = 0 as libc::c_int as size_t;
    while col < ncolumns {
        let mut buf: [libc::c_char; 653] = [0; 653];
        let mut cell: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut v: *mut field_values_t = 0 as *mut field_values_t;
        match (**columns.offset(col as isize)).field_type as libc::c_uint {
            0 => {
                v = &mut block_values;
            }
            1 => {
                v = &mut inode_values;
            }
            2 => {
                v = 0 as *mut field_values_t;
            }
            _ => {
                v = 0 as *mut field_values_t;
                if (b"bad field_type\0" as *const u8 as *const libc::c_char).is_null()
                {} else {
                    __assert_fail(
                        b"!\"bad field_type\"\0" as *const u8 as *const libc::c_char,
                        b"src/df.c\0" as *const u8 as *const libc::c_char,
                        1152 as libc::c_int as libc::c_uint,
                        (*::core::mem::transmute::<
                            &[u8; 129],
                            &[libc::c_char; 129],
                        >(
                            b"void get_dev(const char *, const char *, const char *, const char *, const char *, _Bool, _Bool, const struct fs_usage *, _Bool)\0",
                        ))
                            .as_ptr(),
                    );
                }
            }
        }
        match (**columns.offset(col as isize)).field as libc::c_uint {
            0 => {
                cell = xstrdup(dev_name);
            }
            1 => {
                cell = xstrdup(fstype);
            }
            2 | 6 => {
                cell = xstrdup(
                    df_readable(
                        0 as libc::c_int != 0,
                        (*v).total,
                        buf.as_mut_ptr(),
                        (*v).input_units,
                        (*v).output_units,
                    ),
                );
            }
            3 | 7 => {
                cell = xstrdup(
                    df_readable(
                        (*v).negate_used,
                        (*v).used,
                        buf.as_mut_ptr(),
                        (*v).input_units,
                        (*v).output_units,
                    ),
                );
            }
            4 | 8 => {
                cell = xstrdup(
                    df_readable(
                        (*v).negate_available,
                        (*v).available,
                        buf.as_mut_ptr(),
                        (*v).input_units,
                        (*v).output_units,
                    ),
                );
            }
            5 | 9 => {
                let mut pct: libc::c_double = -(1 as libc::c_int) as libc::c_double;
                if !(!known_value((*v).used) || !known_value((*v).available)) {
                    if !(*v).negate_used
                        && (*v).used
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
                        && ((*v).used).wrapping_add((*v).available)
                            != 0 as libc::c_int as libc::c_ulong
                        && (((*v).used).wrapping_add((*v).available) < (*v).used)
                            as libc::c_int == (*v).negate_available as libc::c_int
                    {
                        let mut u100: uintmax_t = ((*v).used)
                            .wrapping_mul(100 as libc::c_int as libc::c_ulong);
                        let mut nonroot_total: uintmax_t = ((*v).used)
                            .wrapping_add((*v).available);
                        pct = u100
                            .wrapping_div(nonroot_total)
                            .wrapping_add(
                                (u100.wrapping_rem(nonroot_total)
                                    != 0 as libc::c_int as libc::c_ulong) as libc::c_int
                                    as libc::c_ulong,
                            ) as libc::c_double;
                    } else {
                        let mut u: libc::c_double = if (*v).negate_used as libc::c_int
                            != 0
                        {
                            -(((*v).used).wrapping_neg() as libc::c_double)
                        } else {
                            (*v).used as libc::c_double
                        };
                        let mut a: libc::c_double = if (*v).negate_available
                            as libc::c_int != 0
                        {
                            -(((*v).available).wrapping_neg() as libc::c_double)
                        } else {
                            (*v).available as libc::c_double
                        };
                        let mut nonroot_total_0: libc::c_double = u + a;
                        if nonroot_total_0 != 0. {
                            pct = u * 100 as libc::c_int as libc::c_double
                                / nonroot_total_0;
                            let mut lipct: libc::c_long = pct as libc::c_long;
                            let mut ipct: libc::c_double = lipct as libc::c_double;
                            if (ipct - 1 as libc::c_int as libc::c_double) < pct
                                && pct <= ipct + 1 as libc::c_int as libc::c_double
                            {
                                pct = ipct + (ipct < pct) as libc::c_int as libc::c_double;
                            }
                        }
                    }
                }
                if 0 as libc::c_int as libc::c_double <= pct {
                    if rpl_asprintf(
                        &mut cell as *mut *mut libc::c_char,
                        b"%.0f%%\0" as *const u8 as *const libc::c_char,
                        pct,
                    ) == -(1 as libc::c_int)
                    {
                        cell = 0 as *mut libc::c_char;
                    }
                } else {
                    cell = strdup(b"-\0" as *const u8 as *const libc::c_char);
                }
                if cell.is_null() {
                    xalloc_die();
                }
            }
            11 => {
                cell = xstrdup(file);
            }
            10 => {
                cell = xstrdup(mount_point);
            }
            _ => {
                if (b"unhandled field\0" as *const u8 as *const libc::c_char).is_null()
                {} else {
                    __assert_fail(
                        b"!\"unhandled field\"\0" as *const u8 as *const libc::c_char,
                        b"src/df.c\0" as *const u8 as *const libc::c_char,
                        1254 as libc::c_int as libc::c_uint,
                        (*::core::mem::transmute::<
                            &[u8; 129],
                            &[libc::c_char; 129],
                        >(
                            b"void get_dev(const char *, const char *, const char *, const char *, const char *, _Bool, _Bool, const struct fs_usage *, _Bool)\0",
                        ))
                            .as_ptr(),
                    );
                }
            }
        }
        if cell.is_null() {
            if (b"empty cell\0" as *const u8 as *const libc::c_char).is_null() {} else {
                __assert_fail(
                    b"!\"empty cell\"\0" as *const u8 as *const libc::c_char,
                    b"src/df.c\0" as *const u8 as *const libc::c_char,
                    1258 as libc::c_int as libc::c_uint,
                    (*::core::mem::transmute::<
                        &[u8; 129],
                        &[libc::c_char; 129],
                    >(
                        b"void get_dev(const char *, const char *, const char *, const char *, const char *, _Bool, _Bool, const struct fs_usage *, _Bool)\0",
                    ))
                        .as_ptr(),
                );
            }
        }
        replace_problematic_chars(cell);
        let mut cell_width: size_t = gnu_mbswidth(cell, 0 as libc::c_int) as size_t;
        (**columns.offset(col as isize))
            .width = if (**columns.offset(col as isize)).width > cell_width {
            (**columns.offset(col as isize)).width
        } else {
            cell_width
        };
        let ref mut fresh7 = *(*table
            .offset(nrows.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize))
            .offset(col as isize);
        *fresh7 = cell;
        col = col.wrapping_add(1);
    }
    free(dev_name as *mut libc::c_void);
}
unsafe extern "C" fn last_device_for_mount(
    mut mount: *const libc::c_char,
) -> *mut libc::c_char {
    let mut me: *const mount_entry = 0 as *const mount_entry;
    let mut le: *const mount_entry = 0 as *const mount_entry;
    me = mount_list;
    while !me.is_null() {
        if strcmp((*me).me_mountdir, mount) == 0 as libc::c_int {
            le = me;
        }
        me = (*me).me_next;
    }
    if !le.is_null() {
        let mut devname: *mut libc::c_char = (*le).me_devname;
        let mut canon_dev: *mut libc::c_char = canonicalize_file_name(devname);
        if !canon_dev.is_null()
            && *canon_dev.offset(0 as libc::c_int as isize) as libc::c_int == '/' as i32
        {
            return canon_dev;
        }
        free(canon_dev as *mut libc::c_void);
        return xstrdup((*le).me_devname);
    } else {
        return 0 as *mut libc::c_char
    };
}
unsafe extern "C" fn get_device(mut device: *const libc::c_char) -> bool {
    let mut me: *const mount_entry = 0 as *const mount_entry;
    let mut best_match: *const mount_entry = 0 as *const mount_entry;
    let mut best_match_accessible: bool = 0 as libc::c_int != 0;
    let mut eclipsed_device: bool = 0 as libc::c_int != 0;
    let mut file: *const libc::c_char = device;
    let mut resolved: *mut libc::c_char = canonicalize_file_name(device);
    if !resolved.is_null()
        && *resolved.offset(0 as libc::c_int as isize) as libc::c_int == '/' as i32
    {
        device = resolved;
    }
    let mut best_match_len: size_t = 18446744073709551615 as libc::c_ulong;
    me = mount_list;
    while !me.is_null() {
        let mut devname: *mut libc::c_char = (*me).me_devname;
        let mut canon_dev: *mut libc::c_char = canonicalize_file_name((*me).me_devname);
        if !canon_dev.is_null()
            && *canon_dev.offset(0 as libc::c_int as isize) as libc::c_int == '/' as i32
        {
            devname = canon_dev;
        }
        if strcmp(device, devname) == 0 as libc::c_int {
            let mut last_device: *mut libc::c_char = last_device_for_mount(
                (*me).me_mountdir,
            );
            eclipsed_device = !last_device.is_null()
                && !(strcmp(last_device, devname) == 0 as libc::c_int);
            let mut len: size_t = strlen((*me).me_mountdir);
            if !eclipsed_device && (!best_match_accessible || len < best_match_len) {
                let mut device_stats: stat = stat {
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
                let mut this_match_accessible: bool = 0 as libc::c_int != 0;
                if stat((*me).me_mountdir, &mut device_stats) == 0 as libc::c_int {
                    this_match_accessible = 1 as libc::c_int != 0;
                    best_match_accessible = this_match_accessible;
                }
                if this_match_accessible as libc::c_int != 0
                    || !best_match_accessible && len < best_match_len
                {
                    best_match = me;
                    if len == 1 as libc::c_int as libc::c_ulong {
                        free(last_device as *mut libc::c_void);
                        free(canon_dev as *mut libc::c_void);
                        break;
                    } else {
                        best_match_len = len;
                    }
                }
            }
            free(last_device as *mut libc::c_void);
        }
        free(canon_dev as *mut libc::c_void);
        me = (*me).me_next;
    }
    free(resolved as *mut libc::c_void);
    if !best_match.is_null() {
        get_dev(
            (*best_match).me_devname,
            (*best_match).me_mountdir,
            file,
            0 as *const libc::c_char,
            (*best_match).me_type,
            (*best_match).me_dummy() != 0,
            (*best_match).me_remote() != 0,
            0 as *const fs_usage,
            0 as libc::c_int != 0,
        );
        return 1 as libc::c_int != 0;
    } else {
        if eclipsed_device {
            error(
                0 as libc::c_int,
                0 as libc::c_int,
                dcgettext(
                    0 as *const libc::c_char,
                    b"cannot access %s: over-mounted by another device\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
                quotearg_style(shell_escape_always_quoting_style, file),
            );
            exit_status = 1 as libc::c_int;
            return 1 as libc::c_int != 0;
        }
    }
    return 0 as libc::c_int != 0;
}
unsafe extern "C" fn get_point(mut point: *const libc::c_char, mut statp: *const stat) {
    let mut device_stats: stat = stat {
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
    let mut me: *mut mount_entry = 0 as *mut mount_entry;
    let mut best_match: *const mount_entry = 0 as *const mount_entry;
    let mut resolved: *mut libc::c_char = canonicalize_file_name(point);
    if !resolved.is_null()
        && *resolved.offset(0 as libc::c_int as isize) as libc::c_int == '/' as i32
    {
        let mut resolved_len: size_t = strlen(resolved);
        let mut best_match_len: size_t = 0 as libc::c_int as size_t;
        me = mount_list;
        while !me.is_null() {
            if !(strcmp((*me).me_type, b"lofs\0" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int)
                && (best_match.is_null() || (*best_match).me_dummy() as libc::c_int != 0
                    || (*me).me_dummy() == 0)
            {
                let mut len: size_t = strlen((*me).me_mountdir);
                if best_match_len <= len && len <= resolved_len
                    && (len == 1 as libc::c_int as libc::c_ulong
                        || (len == resolved_len
                            || *resolved.offset(len as isize) as libc::c_int
                                == '/' as i32)
                            && strncmp((*me).me_mountdir, resolved, len)
                                == 0 as libc::c_int)
                {
                    best_match = me;
                    best_match_len = len;
                }
            }
            me = (*me).me_next;
        }
    }
    free(resolved as *mut libc::c_void);
    if !best_match.is_null()
        && (stat((*best_match).me_mountdir, &mut device_stats) != 0 as libc::c_int
            || device_stats.st_dev != (*statp).st_dev)
    {
        best_match = 0 as *const mount_entry;
    }
    if best_match.is_null() {
        me = mount_list;
        while !me.is_null() {
            if (*me).me_dev == -(1 as libc::c_int) as dev_t {
                if stat((*me).me_mountdir, &mut device_stats) == 0 as libc::c_int {
                    (*me).me_dev = device_stats.st_dev;
                } else {
                    if *__errno_location() == 5 as libc::c_int {
                        error(
                            0 as libc::c_int,
                            *__errno_location(),
                            b"%s\0" as *const u8 as *const libc::c_char,
                            quotearg_n_style_colon(
                                0 as libc::c_int,
                                shell_escape_quoting_style,
                                (*me).me_mountdir,
                            ),
                        );
                        exit_status = 1 as libc::c_int;
                    }
                    (*me).me_dev = -(2 as libc::c_int) as dev_t;
                }
            }
            if (*statp).st_dev == (*me).me_dev
                && !(strcmp((*me).me_type, b"lofs\0" as *const u8 as *const libc::c_char)
                    == 0 as libc::c_int)
                && (best_match.is_null() || (*best_match).me_dummy() as libc::c_int != 0
                    || (*me).me_dummy() == 0)
            {
                if stat((*me).me_mountdir, &mut device_stats) != 0 as libc::c_int
                    || device_stats.st_dev != (*me).me_dev
                {
                    (*me).me_dev = -(2 as libc::c_int) as dev_t;
                } else {
                    best_match = me;
                }
            }
            me = (*me).me_next;
        }
    }
    if !best_match.is_null() {
        get_dev(
            (*best_match).me_devname,
            (*best_match).me_mountdir,
            point,
            point,
            (*best_match).me_type,
            (*best_match).me_dummy() != 0,
            (*best_match).me_remote() != 0,
            0 as *const fs_usage,
            0 as libc::c_int != 0,
        );
    } else {
        let mut mp: *mut libc::c_char = find_mount_point(point, statp);
        if !mp.is_null() {
            get_dev(
                0 as *const libc::c_char,
                mp,
                point,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as libc::c_int != 0,
                0 as libc::c_int != 0,
                0 as *const fs_usage,
                0 as libc::c_int != 0,
            );
            free(mp as *mut libc::c_void);
        }
    };
}
unsafe extern "C" fn get_entry(mut name: *const libc::c_char, mut statp: *const stat) {
    if ((*statp).st_mode & 0o170000 as libc::c_int as libc::c_uint
        == 0o60000 as libc::c_int as libc::c_uint
        || (*statp).st_mode & 0o170000 as libc::c_int as libc::c_uint
            == 0o20000 as libc::c_int as libc::c_uint)
        && get_device(name) as libc::c_int != 0
    {
        return;
    }
    get_point(name, statp);
}
unsafe extern "C" fn get_all_entries() {
    let mut me: *mut mount_entry = 0 as *mut mount_entry;
    filter_mount_list(show_all_fs);
    me = mount_list;
    while !me.is_null() {
        get_dev(
            (*me).me_devname,
            (*me).me_mountdir,
            0 as *const libc::c_char,
            0 as *const libc::c_char,
            (*me).me_type,
            (*me).me_dummy() != 0,
            (*me).me_remote() != 0,
            0 as *const fs_usage,
            1 as libc::c_int != 0,
        );
        me = (*me).me_next;
    }
}
unsafe extern "C" fn add_fs_type(mut fstype: *const libc::c_char) {
    let mut fsp: *mut fs_type_list = 0 as *mut fs_type_list;
    fsp = xmalloc(::core::mem::size_of::<fs_type_list>() as libc::c_ulong)
        as *mut fs_type_list;
    (*fsp).fs_name = fstype as *mut libc::c_char;
    (*fsp).fs_next = fs_select_list;
    fs_select_list = fsp;
}
unsafe extern "C" fn add_excluded_fs_type(mut fstype: *const libc::c_char) {
    let mut fsp: *mut fs_type_list = 0 as *mut fs_type_list;
    fsp = xmalloc(::core::mem::size_of::<fs_type_list>() as libc::c_ulong)
        as *mut fs_type_list;
    (*fsp).fs_name = fstype as *mut libc::c_char;
    (*fsp).fs_next = fs_exclude_list;
    fs_exclude_list = fsp;
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
                b"Show information about the file system on which each FILE resides,\nor all file systems by default.\n\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            stdout,
        );
        emit_mandatory_arg_note();
        fputs_unlocked(
            dcgettext(
                0 as *const libc::c_char,
                b"  -a, --all             include pseudo, duplicate, inaccessible file systems\n  -B, --block-size=SIZE  scale sizes by SIZE before printing them; e.g.,\n                           '-BM' prints sizes in units of 1,048,576 bytes;\n                           see SIZE format below\n  -h, --human-readable  print sizes in powers of 1024 (e.g., 1023M)\n  -H, --si              print sizes in powers of 1000 (e.g., 1.1G)\n\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            stdout,
        );
        fputs_unlocked(
            dcgettext(
                0 as *const libc::c_char,
                b"  -i, --inodes          list inode information instead of block usage\n  -k                    like --block-size=1K\n  -l, --local           limit listing to local file systems\n      --no-sync         do not invoke sync before getting usage info (default)\n\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            stdout,
        );
        fputs_unlocked(
            dcgettext(
                0 as *const libc::c_char,
                b"      --output[=FIELD_LIST]  use the output format defined by FIELD_LIST,\n                               or print all fields if FIELD_LIST is omitted.\n  -P, --portability     use the POSIX output format\n      --sync            invoke sync before getting usage info\n\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            stdout,
        );
        fputs_unlocked(
            dcgettext(
                0 as *const libc::c_char,
                b"      --total           elide all entries insignificant to available space,\n                          and produce a grand total\n\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            stdout,
        );
        fputs_unlocked(
            dcgettext(
                0 as *const libc::c_char,
                b"  -t, --type=TYPE       limit listing to file systems of type TYPE\n  -T, --print-type      print file system type\n  -x, --exclude-type=TYPE   limit listing to file systems not of type TYPE\n  -v                    (ignored)\n\0"
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
        emit_blocksize_note(b"DF\0" as *const u8 as *const libc::c_char);
        emit_size_note();
        fputs_unlocked(
            dcgettext(
                0 as *const libc::c_char,
                b"\nFIELD_LIST is a comma-separated list of columns to be included.  Valid\nfield names are: 'source', 'fstype', 'itotal', 'iused', 'iavail', 'ipcent',\n'size', 'used', 'avail', 'pcent', 'file' and 'target' (see info page).\n\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            stdout,
        );
        emit_ancillary_info(b"df\0" as *const u8 as *const libc::c_char);
    }
    exit(status);
}
unsafe fn main_0(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut stats: *mut stat = 0 as *mut stat;
    set_program_name(*argv.offset(0 as libc::c_int as isize));
    setlocale(6 as libc::c_int, b"\0" as *const u8 as *const libc::c_char);
    bindtextdomain(
        b"coreutils\0" as *const u8 as *const libc::c_char,
        b"/usr/local/share/locale\0" as *const u8 as *const libc::c_char,
    );
    textdomain(b"coreutils\0" as *const u8 as *const libc::c_char);
    atexit(Some(close_stdout as unsafe extern "C" fn() -> ()));
    fs_select_list = 0 as *mut fs_type_list;
    fs_exclude_list = 0 as *mut fs_type_list;
    show_all_fs = 0 as libc::c_int != 0;
    show_listed_fs = 0 as libc::c_int != 0;
    human_output_opts = -(1 as libc::c_int);
    print_type = 0 as libc::c_int != 0;
    file_systems_processed = 0 as libc::c_int != 0;
    exit_status = 0 as libc::c_int;
    print_grand_total = 0 as libc::c_int != 0;
    grand_fsu.fsu_blocksize = 1 as libc::c_int as uintmax_t;
    let mut posix_format: bool = 0 as libc::c_int != 0;
    let mut msg_mut_excl: *const libc::c_char = dcgettext(
        0 as *const libc::c_char,
        b"options %s and %s are mutually exclusive\0" as *const u8
            as *const libc::c_char,
        5 as libc::c_int,
    );
    loop {
        let mut oi: libc::c_int = -(1 as libc::c_int);
        let mut c: libc::c_int = getopt_long(
            argc,
            argv,
            b"aB:iF:hHklmPTt:vx:\0" as *const u8 as *const libc::c_char,
            long_options.as_ptr(),
            &mut oi,
        );
        if c == -(1 as libc::c_int) {
            break;
        }
        match c {
            97 => {
                show_all_fs = 1 as libc::c_int != 0;
            }
            66 => {
                let mut e: strtol_error = human_options(
                    optarg,
                    &mut human_output_opts,
                    &mut output_block_size,
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
            }
            105 => {
                if header_mode == OUTPUT_MODE as libc::c_int {
                    error(
                        0 as libc::c_int,
                        0 as libc::c_int,
                        msg_mut_excl,
                        b"-i\0" as *const u8 as *const libc::c_char,
                        b"--output\0" as *const u8 as *const libc::c_char,
                    );
                    usage(1 as libc::c_int);
                }
                header_mode = INODES_MODE as libc::c_int;
            }
            104 => {
                human_output_opts = human_autoscale as libc::c_int
                    | human_SI as libc::c_int | human_base_1024 as libc::c_int;
                output_block_size = 1 as libc::c_int as uintmax_t;
            }
            72 => {
                human_output_opts = human_autoscale as libc::c_int
                    | human_SI as libc::c_int;
                output_block_size = 1 as libc::c_int as uintmax_t;
            }
            107 => {
                human_output_opts = 0 as libc::c_int;
                output_block_size = 1024 as libc::c_int as uintmax_t;
            }
            108 => {
                show_local_fs = 1 as libc::c_int != 0;
            }
            109 => {
                human_output_opts = 0 as libc::c_int;
                output_block_size = (1024 as libc::c_int * 1024 as libc::c_int)
                    as uintmax_t;
            }
            84 => {
                if header_mode == OUTPUT_MODE as libc::c_int {
                    error(
                        0 as libc::c_int,
                        0 as libc::c_int,
                        msg_mut_excl,
                        b"-T\0" as *const u8 as *const libc::c_char,
                        b"--output\0" as *const u8 as *const libc::c_char,
                    );
                    usage(1 as libc::c_int);
                }
                print_type = 1 as libc::c_int != 0;
            }
            80 => {
                if header_mode == OUTPUT_MODE as libc::c_int {
                    error(
                        0 as libc::c_int,
                        0 as libc::c_int,
                        msg_mut_excl,
                        b"-P\0" as *const u8 as *const libc::c_char,
                        b"--output\0" as *const u8 as *const libc::c_char,
                    );
                    usage(1 as libc::c_int);
                }
                posix_format = 1 as libc::c_int != 0;
            }
            257 => {
                require_sync = 1 as libc::c_int != 0;
            }
            256 => {
                require_sync = 0 as libc::c_int != 0;
            }
            70 | 116 => {
                add_fs_type(optarg);
            }
            118 => {}
            120 => {
                add_excluded_fs_type(optarg);
            }
            259 => {
                if header_mode == INODES_MODE as libc::c_int {
                    error(
                        0 as libc::c_int,
                        0 as libc::c_int,
                        msg_mut_excl,
                        b"-i\0" as *const u8 as *const libc::c_char,
                        b"--output\0" as *const u8 as *const libc::c_char,
                    );
                    usage(1 as libc::c_int);
                }
                if posix_format as libc::c_int != 0
                    && header_mode == DEFAULT_MODE as libc::c_int
                {
                    error(
                        0 as libc::c_int,
                        0 as libc::c_int,
                        msg_mut_excl,
                        b"-P\0" as *const u8 as *const libc::c_char,
                        b"--output\0" as *const u8 as *const libc::c_char,
                    );
                    usage(1 as libc::c_int);
                }
                if print_type {
                    error(
                        0 as libc::c_int,
                        0 as libc::c_int,
                        msg_mut_excl,
                        b"-T\0" as *const u8 as *const libc::c_char,
                        b"--output\0" as *const u8 as *const libc::c_char,
                    );
                    usage(1 as libc::c_int);
                }
                header_mode = OUTPUT_MODE as libc::c_int;
                if !optarg.is_null() {
                    decode_output_arg(optarg);
                }
            }
            258 => {
                print_grand_total = 1 as libc::c_int != 0;
            }
            -2 => {
                usage(0 as libc::c_int);
            }
            -3 => {
                version_etc(
                    stdout,
                    b"df\0" as *const u8 as *const libc::c_char,
                    b"GNU coreutils\0" as *const u8 as *const libc::c_char,
                    Version,
                    b"Torbjorn Granlund\0" as *const u8 as *const libc::c_char,
                    b"David MacKenzie\0" as *const u8 as *const libc::c_char,
                    b"Paul Eggert\0" as *const u8 as *const libc::c_char,
                    0 as *mut libc::c_void as *mut libc::c_char,
                );
                exit(0 as libc::c_int);
            }
            _ => {
                usage(1 as libc::c_int);
            }
        }
    }
    if human_output_opts == -(1 as libc::c_int) {
        if posix_format {
            human_output_opts = 0 as libc::c_int;
            output_block_size = (if !(getenv(
                b"POSIXLY_CORRECT\0" as *const u8 as *const libc::c_char,
            ))
                .is_null()
            {
                512 as libc::c_int
            } else {
                1024 as libc::c_int
            }) as uintmax_t;
        } else {
            human_options(
                getenv(b"DF_BLOCK_SIZE\0" as *const u8 as *const libc::c_char),
                &mut human_output_opts,
                &mut output_block_size,
            );
        }
    }
    if !(header_mode == INODES_MODE as libc::c_int
        || header_mode == OUTPUT_MODE as libc::c_int)
    {
        if human_output_opts & human_autoscale as libc::c_int != 0 {
            header_mode = HUMAN_MODE as libc::c_int;
        } else if posix_format {
            header_mode = POSIX_MODE as libc::c_int;
        }
    }
    let mut match_0: bool = 0 as libc::c_int != 0;
    let mut fs_incl: *mut fs_type_list = 0 as *mut fs_type_list;
    fs_incl = fs_select_list;
    while !fs_incl.is_null() {
        let mut fs_excl: *mut fs_type_list = 0 as *mut fs_type_list;
        fs_excl = fs_exclude_list;
        while !fs_excl.is_null() {
            if strcmp((*fs_incl).fs_name, (*fs_excl).fs_name) == 0 as libc::c_int {
                error(
                    0 as libc::c_int,
                    0 as libc::c_int,
                    dcgettext(
                        0 as *const libc::c_char,
                        b"file system type %s both selected and excluded\0" as *const u8
                            as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    quote((*fs_incl).fs_name),
                );
                match_0 = 1 as libc::c_int != 0;
                break;
            } else {
                fs_excl = (*fs_excl).fs_next;
            }
        }
        fs_incl = (*fs_incl).fs_next;
    }
    if match_0 {
        return 1 as libc::c_int;
    }
    if (0 as libc::c_int) < optind {} else {
        unreachable!();
    };
    if optind < argc {
        stats = xnmalloc(
            (argc - optind) as size_t,
            ::core::mem::size_of::<stat>() as libc::c_ulong,
        ) as *mut stat;
        let mut i: libc::c_int = optind;
        while i < argc {
            let mut err: libc::c_int = automount_stat_err(
                *argv.offset(i as isize),
                &mut *stats.offset((i - optind) as isize),
            );
            if err != 0 as libc::c_int {
                error(
                    0 as libc::c_int,
                    err,
                    b"%s\0" as *const u8 as *const libc::c_char,
                    quotearg_n_style_colon(
                        0 as libc::c_int,
                        shell_escape_quoting_style,
                        *argv.offset(i as isize),
                    ),
                );
                exit_status = 1 as libc::c_int;
                let ref mut fresh8 = *argv.offset(i as isize);
                *fresh8 = 0 as *mut libc::c_char;
            }
            i += 1;
        }
    }
    mount_list = read_file_system_list(
        !fs_select_list.is_null() || !fs_exclude_list.is_null()
            || print_type as libc::c_int != 0
            || field_data[FSTYPE_FIELD as libc::c_int as usize].used as libc::c_int != 0
            || show_local_fs as libc::c_int != 0,
    );
    if mount_list.is_null() {
        let mut status: libc::c_int = 0 as libc::c_int;
        if !(optind < argc)
            || (show_all_fs as libc::c_int != 0 || show_local_fs as libc::c_int != 0
                || !fs_select_list.is_null() || !fs_exclude_list.is_null())
        {
            status = 1 as libc::c_int;
        }
        let mut warning: *const libc::c_char = if status == 0 as libc::c_int {
            dcgettext(
                0 as *const libc::c_char,
                b"Warning: \0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ) as *const libc::c_char
        } else {
            b"\0" as *const u8 as *const libc::c_char
        };
        error(
            status,
            *__errno_location(),
            b"%s%s\0" as *const u8 as *const libc::c_char,
            warning,
            dcgettext(
                0 as *const libc::c_char,
                b"cannot read table of mounted file systems\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
    }
    if require_sync {
        sync();
    }
    get_field_list();
    get_header();
    if !stats.is_null() {
        show_listed_fs = 1 as libc::c_int != 0;
        let mut i_0: libc::c_int = optind;
        while i_0 < argc {
            if !(*argv.offset(i_0 as isize)).is_null() {
                get_entry(
                    *argv.offset(i_0 as isize),
                    &mut *stats.offset((i_0 - optind) as isize),
                );
            }
            i_0 += 1;
        }
    } else {
        get_all_entries();
    }
    if file_systems_processed {
        if print_grand_total {
            get_dev(
                b"total\0" as *const u8 as *const libc::c_char,
                if field_data[SOURCE_FIELD as libc::c_int as usize].used as libc::c_int
                    != 0
                {
                    b"-\0" as *const u8 as *const libc::c_char
                } else {
                    b"total\0" as *const u8 as *const libc::c_char
                },
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as libc::c_int != 0,
                0 as libc::c_int != 0,
                &mut grand_fsu,
                0 as libc::c_int != 0,
            );
        }
        print_table();
    } else if exit_status == 0 as libc::c_int {
        if ::core::mem::size_of::<C2RustUnnamed_5>() as libc::c_ulong != 0 {
            error(
                1 as libc::c_int,
                0 as libc::c_int,
                dcgettext(
                    0 as *const libc::c_char,
                    b"no file systems processed\0" as *const u8 as *const libc::c_char,
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
                    b"no file systems processed\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
            );
            if 0 as libc::c_int != 0 {} else {
                unreachable!();
            };
        };
    }
    exit(exit_status);
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
