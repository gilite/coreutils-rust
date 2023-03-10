#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![feature(extern_types)]
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type tm_zone;
    pub type quoting_options;
    static mut stdout: *mut FILE;
    static mut stderr: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn __overflow(_: *mut FILE, _: libc::c_int) -> libc::c_int;
    fn fputs_unlocked(__s: *const libc::c_char, __stream: *mut FILE) -> libc::c_int;
    fn getpwuid(__uid: __uid_t) -> *mut passwd;
    fn getgrgid(__gid: __gid_t) -> *mut group;
    fn statfs(__file: *const libc::c_char, __buf: *mut statfs) -> libc::c_int;
    fn __errno_location() -> *mut libc::c_int;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn memchr(
        _: *const libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn tzalloc(__name: *const libc::c_char) -> timezone_t;
    fn localtime_rz(
        __tz: timezone_t,
        __timer: *const time_t,
        __result: *mut tm,
    ) -> *mut tm;
    fn stat(__file: *const libc::c_char, __buf: *mut stat) -> libc::c_int;
    fn statx(
        __dirfd: libc::c_int,
        __path: *const libc::c_char,
        __flags: libc::c_int,
        __mask: libc::c_uint,
        __buf: *mut statx,
    ) -> libc::c_int;
    static mut optarg: *mut libc::c_char;
    static mut optind: libc::c_int;
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strncmp(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strspn(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_ulong;
    fn strstr(_: *const libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn free(_: *mut libc::c_void);
    fn strtol(
        _: *const libc::c_char,
        _: *mut *mut libc::c_char,
        _: libc::c_int,
    ) -> libc::c_long;
    fn atexit(__func: Option::<unsafe extern "C" fn() -> ()>) -> libc::c_int;
    fn exit(_: libc::c_int) -> !;
    fn getenv(__name: *const libc::c_char) -> *mut libc::c_char;
    fn canonicalize_file_name(__name: *const libc::c_char) -> *mut libc::c_char;
    static mut Version: *const libc::c_char;
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    fn setlocale(
        __category: libc::c_int,
        __locale: *const libc::c_char,
    ) -> *mut libc::c_char;
    fn localeconv() -> *mut lconv;
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
    fn xmalloc(s: size_t) -> *mut libc::c_void;
    fn xstrdup(str: *const libc::c_char) -> *mut libc::c_char;
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
    static quoting_style_args: [*const libc::c_char; 0];
    static quoting_style_vals: [quoting_style; 0];
    fn get_quoting_style(o: *const quoting_options) -> quoting_style;
    fn set_quoting_style(o: *mut quoting_options, s: quoting_style);
    fn quotearg_style(s: quoting_style, arg: *const libc::c_char) -> *mut libc::c_char;
    fn areadlink_with_size(
        filename: *const libc::c_char,
        size_hint: size_t,
    ) -> *mut libc::c_char;
    fn argmatch(
        arg: *const libc::c_char,
        arglist: *const *const libc::c_char,
        vallist: *const libc::c_void,
        valsize: size_t,
    ) -> ptrdiff_t;
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
    fn quote(arg: *const libc::c_char) -> *const libc::c_char;
    fn error(
        __status: libc::c_int,
        __errnum: libc::c_int,
        __format: *const libc::c_char,
        _: ...
    );
    fn file_type(_: *const stat) -> *const libc::c_char;
    fn filemodestring(statp: *const stat, str: *mut libc::c_char);
    fn getopt_long(
        ___argc: libc::c_int,
        ___argv: *const *mut libc::c_char,
        __shortopts: *const libc::c_char,
        __longopts: *const option,
        __longind: *mut libc::c_int,
    ) -> libc::c_int;
    fn read_file_system_list(need_fs_type: bool) -> *mut mount_entry;
    fn nstrftime(
        _: *mut libc::c_char,
        _: size_t,
        _: *const libc::c_char,
        _: *const tm,
        __tz: timezone_t,
        __ns: libc::c_int,
    ) -> size_t;
    fn find_mount_point(_: *const libc::c_char, _: *const stat) -> *mut libc::c_char;
    fn xasprintf(format: *const libc::c_char, _: ...) -> *mut libc::c_char;
}
pub type ptrdiff_t = libc::c_long;
pub type size_t = libc::c_ulong;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __fsid_t {
    pub __val: [libc::c_int; 2],
}
pub type __time_t = libc::c_long;
pub type __blksize_t = libc::c_int;
pub type __blkcnt_t = libc::c_long;
pub type __fsblkcnt_t = libc::c_ulong;
pub type __fsfilcnt_t = libc::c_ulong;
pub type __fsword_t = libc::c_long;
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
pub type dev_t = __dev_t;
pub type time_t = __time_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
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
pub struct statfs {
    pub f_type: __fsword_t,
    pub f_bsize: __fsword_t,
    pub f_blocks: __fsblkcnt_t,
    pub f_bfree: __fsblkcnt_t,
    pub f_bavail: __fsblkcnt_t,
    pub f_files: __fsfilcnt_t,
    pub f_ffree: __fsfilcnt_t,
    pub f_fsid: __fsid_t,
    pub f_namelen: __fsword_t,
    pub f_frsize: __fsword_t,
    pub f_flags: __fsword_t,
    pub f_spare: [__fsword_t; 4],
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
pub type __u16 = libc::c_ushort;
pub type __s32 = libc::c_int;
pub type __u32 = libc::c_uint;
pub type __s64 = libc::c_longlong;
pub type __u64 = libc::c_ulonglong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct statx_timestamp {
    pub tv_sec: __s64,
    pub tv_nsec: __u32,
    pub __reserved: __s32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct statx {
    pub stx_mask: __u32,
    pub stx_blksize: __u32,
    pub stx_attributes: __u64,
    pub stx_nlink: __u32,
    pub stx_uid: __u32,
    pub stx_gid: __u32,
    pub stx_mode: __u16,
    pub __spare0: [__u16; 1],
    pub stx_ino: __u64,
    pub stx_size: __u64,
    pub stx_blocks: __u64,
    pub stx_attributes_mask: __u64,
    pub stx_atime: statx_timestamp,
    pub stx_btime: statx_timestamp,
    pub stx_ctime: statx_timestamp,
    pub stx_mtime: statx_timestamp,
    pub stx_rdev_major: __u32,
    pub stx_rdev_minor: __u32,
    pub stx_dev_major: __u32,
    pub stx_dev_minor: __u32,
    pub stx_mnt_id: __u64,
    pub __spare2: __u64,
    pub __spare3: [__u64; 12],
}
pub type intmax_t = __intmax_t;
pub type uintmax_t = __uintmax_t;
pub type C2RustUnnamed = libc::c_uint;
pub const _ISalnum: C2RustUnnamed = 8;
pub const _ISpunct: C2RustUnnamed = 4;
pub const _IScntrl: C2RustUnnamed = 2;
pub const _ISblank: C2RustUnnamed = 1;
pub const _ISgraph: C2RustUnnamed = 32768;
pub const _ISprint: C2RustUnnamed = 16384;
pub const _ISspace: C2RustUnnamed = 8192;
pub const _ISxdigit: C2RustUnnamed = 4096;
pub const _ISdigit: C2RustUnnamed = 2048;
pub const _ISalpha: C2RustUnnamed = 1024;
pub const _ISlower: C2RustUnnamed = 512;
pub const _ISupper: C2RustUnnamed = 256;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct lconv {
    pub decimal_point: *mut libc::c_char,
    pub thousands_sep: *mut libc::c_char,
    pub grouping: *mut libc::c_char,
    pub int_curr_symbol: *mut libc::c_char,
    pub currency_symbol: *mut libc::c_char,
    pub mon_decimal_point: *mut libc::c_char,
    pub mon_thousands_sep: *mut libc::c_char,
    pub mon_grouping: *mut libc::c_char,
    pub positive_sign: *mut libc::c_char,
    pub negative_sign: *mut libc::c_char,
    pub int_frac_digits: libc::c_char,
    pub frac_digits: libc::c_char,
    pub p_cs_precedes: libc::c_char,
    pub p_sep_by_space: libc::c_char,
    pub n_cs_precedes: libc::c_char,
    pub n_sep_by_space: libc::c_char,
    pub p_sign_posn: libc::c_char,
    pub n_sign_posn: libc::c_char,
    pub int_p_cs_precedes: libc::c_char,
    pub int_p_sep_by_space: libc::c_char,
    pub int_n_cs_precedes: libc::c_char,
    pub int_n_sep_by_space: libc::c_char,
    pub int_p_sign_posn: libc::c_char,
    pub int_n_sign_posn: libc::c_char,
}
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
pub struct option {
    pub name: *const libc::c_char,
    pub has_arg: libc::c_int,
    pub flag: *mut libc::c_int,
    pub val: libc::c_int,
}
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
pub type C2RustUnnamed_1 = libc::c_uint;
pub const PRINTF_OPTION: C2RustUnnamed_1 = 256;
pub type cached_mode = libc::c_uint;
pub const cached_always: cached_mode = 2;
pub const cached_never: cached_mode = 1;
pub const cached_default: cached_mode = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct print_args {
    pub st: *mut stat,
    pub btime: timespec,
}
pub type fsid_word = libc::c_uint;
pub const MAX_ADDITIONAL_BYTES: C2RustUnnamed_3 = 2;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_2 {
    pub _gl_dummy: libc::c_int,
}
pub type C2RustUnnamed_3 = libc::c_uint;
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
unsafe extern "C" fn putchar_unlocked(mut __c: libc::c_int) -> libc::c_int {
    return if ((*stdout)._IO_write_ptr >= (*stdout)._IO_write_end) as libc::c_int
        as libc::c_long != 0
    {
        __overflow(stdout, __c as libc::c_uchar as libc::c_int)
    } else {
        let fresh1 = (*stdout)._IO_write_ptr;
        (*stdout)._IO_write_ptr = ((*stdout)._IO_write_ptr).offset(1);
        *fresh1 = __c as libc::c_char;
        *fresh1 as libc::c_uchar as libc::c_int
    };
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
unsafe extern "C" fn lgetfilecon(
    mut file: *const libc::c_char,
    mut con: *mut *mut libc::c_char,
) -> libc::c_int {
    *__errno_location() = 95 as libc::c_int;
    return -(1 as libc::c_int);
}
#[inline]
unsafe extern "C" fn gnu_dev_major(mut __dev: __dev_t) -> libc::c_uint {
    let mut __major: libc::c_uint = 0;
    __major = ((__dev & 0xfff00 as libc::c_uint as __dev_t) >> 8 as libc::c_int)
        as libc::c_uint;
    __major = (__major as libc::c_ulong
        | (__dev & 0xfffff00000000000 as libc::c_ulong) >> 32 as libc::c_int)
        as libc::c_uint;
    return __major;
}
#[inline]
unsafe extern "C" fn gnu_dev_minor(mut __dev: __dev_t) -> libc::c_uint {
    let mut __minor: libc::c_uint = 0;
    __minor = ((__dev & 0xff as libc::c_uint as __dev_t) >> 0 as libc::c_int)
        as libc::c_uint;
    __minor = (__minor as libc::c_ulong
        | (__dev & 0xffffff00000 as libc::c_ulong) >> 12 as libc::c_int) as libc::c_uint;
    return __minor;
}
#[inline]
unsafe extern "C" fn gnu_dev_makedev(
    mut __major: libc::c_uint,
    mut __minor: libc::c_uint,
) -> __dev_t {
    let mut __dev: __dev_t = 0;
    __dev = ((__major & 0xfff as libc::c_uint) as __dev_t) << 8 as libc::c_int;
    __dev |= ((__major & 0xfffff000 as libc::c_uint) as __dev_t) << 32 as libc::c_int;
    __dev |= ((__minor & 0xff as libc::c_uint) as __dev_t) << 0 as libc::c_int;
    __dev |= ((__minor & 0xffffff00 as libc::c_uint) as __dev_t) << 12 as libc::c_int;
    return __dev;
}
#[inline]
unsafe extern "C" fn to_uchar(mut ch: libc::c_char) -> libc::c_uchar {
    return ch as libc::c_uchar;
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
#[inline]
unsafe extern "C" fn statx_timestamp_to_timespec(mut tsx: statx_timestamp) -> timespec {
    let mut ts: timespec = timespec { tv_sec: 0, tv_nsec: 0 };
    ts.tv_sec = tsx.tv_sec as __time_t;
    ts.tv_nsec = tsx.tv_nsec as __syscall_slong_t;
    return ts;
}
#[inline]
unsafe extern "C" fn statx_to_stat(mut stx: *mut statx, mut stat_0: *mut stat) {
    (*stat_0).st_dev = gnu_dev_makedev((*stx).stx_dev_major, (*stx).stx_dev_minor);
    (*stat_0).st_ino = (*stx).stx_ino as __ino_t;
    (*stat_0).st_mode = (*stx).stx_mode as __mode_t;
    (*stat_0).st_nlink = (*stx).stx_nlink;
    (*stat_0).st_uid = (*stx).stx_uid;
    (*stat_0).st_gid = (*stx).stx_gid;
    (*stat_0).st_rdev = gnu_dev_makedev((*stx).stx_rdev_major, (*stx).stx_rdev_minor);
    (*stat_0).st_size = (*stx).stx_size as __off_t;
    (*stat_0).st_blksize = (*stx).stx_blksize as __blksize_t;
    (*stat_0).st_blocks = (*stx).stx_blocks as __blkcnt_t;
    (*stat_0).st_atim = statx_timestamp_to_timespec((*stx).stx_atime);
    (*stat_0).st_mtim = statx_timestamp_to_timespec((*stx).stx_mtime);
    (*stat_0).st_ctim = statx_timestamp_to_timespec((*stx).stx_ctime);
}
static mut digits: [libc::c_char; 11] = unsafe {
    *::core::mem::transmute::<&[u8; 11], &[libc::c_char; 11]>(b"0123456789\0")
};
static mut printf_flags: [libc::c_char; 8] = unsafe {
    *::core::mem::transmute::<&[u8; 8], &[libc::c_char; 8]>(b"'-+ #0I\0")
};
static mut fmt_terse_fs: [libc::c_char; 34] = unsafe {
    *::core::mem::transmute::<
        &[u8; 34],
        &[libc::c_char; 34],
    >(b"%n %i %l %t %s %S %b %f %a %c %d\n\0")
};
static mut fmt_terse_regular: [libc::c_char; 49] = unsafe {
    *::core::mem::transmute::<
        &[u8; 49],
        &[libc::c_char; 49],
    >(b"%n %s %b %f %u %g %D %i %h %t %T %X %Y %Z %W %o\n\0")
};
static mut fmt_terse_selinux: [libc::c_char; 52] = unsafe {
    *::core::mem::transmute::<
        &[u8; 52],
        &[libc::c_char; 52],
    >(b"%n %s %b %f %u %g %D %i %h %t %T %X %Y %Z %W %o %C\n\0")
};
static mut cached_args: [*const libc::c_char; 4] = [
    b"default\0" as *const u8 as *const libc::c_char,
    b"never\0" as *const u8 as *const libc::c_char,
    b"always\0" as *const u8 as *const libc::c_char,
    0 as *const libc::c_char,
];
static mut cached_modes: [cached_mode; 3] = [
    cached_default,
    cached_never,
    cached_always,
];
static mut long_options: [option; 9] = [
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
            name: b"file-system\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'f' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"format\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'c' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"printf\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: PRINTF_OPTION as libc::c_int,
        };
        init
    },
    {
        let mut init = option {
            name: b"terse\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 't' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"cached\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 0 as libc::c_int,
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
static mut follow_links: bool = false;
static mut interpret_backslash_escapes: bool = false;
static mut trailing_delim: *const libc::c_char = b"\0" as *const u8
    as *const libc::c_char;
static mut decimal_point: *const libc::c_char = 0 as *const libc::c_char;
static mut decimal_point_len: size_t = 0;
unsafe extern "C" fn human_fstype(mut statfsbuf: *const statfs) -> *const libc::c_char {
    match (*statfsbuf).f_type {
        1513908720 => return b"aafs\0" as *const u8 as *const libc::c_char,
        1633904243 => return b"acfs\0" as *const u8 as *const libc::c_char,
        44533 => return b"adfs\0" as *const u8 as *const libc::c_char,
        44543 => return b"affs\0" as *const u8 as *const libc::c_char,
        1397113167 => return b"afs\0" as *const u8 as *const libc::c_char,
        151263540 => return b"anon-inode FS\0" as *const u8 as *const libc::c_char,
        1635083891 => return b"aufs\0" as *const u8 as *const libc::c_char,
        391 => return b"autofs\0" as *const u8 as *const libc::c_char,
        325456742 => return b"balloon-kvm-fs\0" as *const u8 as *const libc::c_char,
        1111905073 => return b"befs\0" as *const u8 as *const libc::c_char,
        1650746742 => return b"bdevfs\0" as *const u8 as *const libc::c_char,
        464386766 => return b"bfs\0" as *const u8 as *const libc::c_char,
        1819242352 => return b"binderfs\0" as *const u8 as *const libc::c_char,
        3405662737 => return b"bpf_fs\0" as *const u8 as *const libc::c_char,
        1112100429 => return b"binfmt_misc\0" as *const u8 as *const libc::c_char,
        2435016766 => return b"btrfs\0" as *const u8 as *const libc::c_char,
        1936880249 => return b"btrfs_test\0" as *const u8 as *const libc::c_char,
        12805120 => return b"ceph\0" as *const u8 as *const libc::c_char,
        2613483 => return b"cgroupfs\0" as *const u8 as *const libc::c_char,
        1667723888 => return b"cgroup2fs\0" as *const u8 as *const libc::c_char,
        4283649346 => return b"cifs\0" as *const u8 as *const libc::c_char,
        1937076805 => return b"coda\0" as *const u8 as *const libc::c_char,
        19920823 => return b"coh\0" as *const u8 as *const libc::c_char,
        1650812272 => return b"configfs\0" as *const u8 as *const libc::c_char,
        684539205 => return b"cramfs\0" as *const u8 as *const libc::c_char,
        1161678120 => return b"cramfs-wend\0" as *const u8 as *const libc::c_char,
        1684300152 => return b"daxfs\0" as *const u8 as *const libc::c_char,
        1684170528 => return b"debugfs\0" as *const u8 as *const libc::c_char,
        4979 => return b"devfs\0" as *const u8 as *const libc::c_char,
        1162691661 => return b"devmem\0" as *const u8 as *const libc::c_char,
        7377 => return b"devpts\0" as *const u8 as *const libc::c_char,
        1145913666 => return b"dma-buf-fs\0" as *const u8 as *const libc::c_char,
        61791 => return b"ecryptfs\0" as *const u8 as *const libc::c_char,
        3730735588 => return b"efivarfs\0" as *const u8 as *const libc::c_char,
        4278867 => return b"efs\0" as *const u8 as *const libc::c_char,
        3774210530 => return b"erofs\0" as *const u8 as *const libc::c_char,
        538032816 => return b"exfat\0" as *const u8 as *const libc::c_char,
        1163413075 => return b"exfs\0" as *const u8 as *const libc::c_char,
        24053 => return b"exofs\0" as *const u8 as *const libc::c_char,
        4989 => return b"ext\0" as *const u8 as *const libc::c_char,
        61267 => return b"ext2/ext3\0" as *const u8 as *const libc::c_char,
        61265 => return b"ext2\0" as *const u8 as *const libc::c_char,
        4076150800 => return b"f2fs\0" as *const u8 as *const libc::c_char,
        16390 => return b"fat\0" as *const u8 as *const libc::c_char,
        428016422 => return b"fhgfs\0" as *const u8 as *const libc::c_char,
        1702057286 => return b"fuseblk\0" as *const u8 as *const libc::c_char,
        1702057283 => return b"fusectl\0" as *const u8 as *const libc::c_char,
        195894762 => return b"futexfs\0" as *const u8 as *const libc::c_char,
        18225520 => return b"gfs/gfs2\0" as *const u8 as *const libc::c_char,
        1196443219 => return b"gpfs\0" as *const u8 as *const libc::c_char,
        16964 => return b"hfs\0" as *const u8 as *const libc::c_char,
        18475 => return b"hfs+\0" as *const u8 as *const libc::c_char,
        18520 => return b"hfsx\0" as *const u8 as *const libc::c_char,
        12648430 => return b"hostfs\0" as *const u8 as *const libc::c_char,
        4187351113 => return b"hpfs\0" as *const u8 as *const libc::c_char,
        2508478710 => return b"hugetlbfs\0" as *const u8 as *const libc::c_char,
        288389204 => return b"inodefs\0" as *const u8 as *const libc::c_char,
        19993000 => return b"ibrix\0" as *const u8 as *const libc::c_char,
        732765674 => return b"inotifyfs\0" as *const u8 as *const libc::c_char,
        38496 => return b"isofs\0" as *const u8 as *const libc::c_char,
        16388 => return b"isofs\0" as *const u8 as *const libc::c_char,
        16384 => return b"isofs\0" as *const u8 as *const libc::c_char,
        1984 => return b"jffs\0" as *const u8 as *const libc::c_char,
        29366 => return b"jffs2\0" as *const u8 as *const libc::c_char,
        827541066 => return b"jfs\0" as *const u8 as *const libc::c_char,
        1799439955 => return b"k-afs\0" as *const u8 as *const libc::c_char,
        3380511080 => return b"logfs\0" as *const u8 as *const libc::c_char,
        198183888 => return b"lustre\0" as *const u8 as *const libc::c_char,
        1397109069 => return b"m1fs\0" as *const u8 as *const libc::c_char,
        4991 => return b"minix\0" as *const u8 as *const libc::c_char,
        5007 => return b"minix (30 char.)\0" as *const u8 as *const libc::c_char,
        9320 => return b"minix v2\0" as *const u8 as *const libc::c_char,
        9336 => return b"minix v2 (30 char.)\0" as *const u8 as *const libc::c_char,
        19802 => return b"minix3\0" as *const u8 as *const libc::c_char,
        427819522 => return b"mqueue\0" as *const u8 as *const libc::c_char,
        19780 => return b"msdos\0" as *const u8 as *const libc::c_char,
        22092 => return b"novell\0" as *const u8 as *const libc::c_char,
        26985 => return b"nfs\0" as *const u8 as *const libc::c_char,
        1852207972 => return b"nfsd\0" as *const u8 as *const libc::c_char,
        13364 => return b"nilfs\0" as *const u8 as *const libc::c_char,
        1853056627 => return b"nsfs\0" as *const u8 as *const libc::c_char,
        1397118030 => return b"ntfs\0" as *const u8 as *const libc::c_char,
        40865 => return b"openprom\0" as *const u8 as *const libc::c_char,
        1952539503 => return b"ocfs2\0" as *const u8 as *const libc::c_char,
        2035054128 => return b"overlayfs\0" as *const u8 as *const libc::c_char,
        2866260714 => return b"panfs\0" as *const u8 as *const libc::c_char,
        1346981957 => return b"pipefs\0" as *const u8 as *const libc::c_char,
        3344373136 => return b"ppc-cmm-fs\0" as *const u8 as *const libc::c_char,
        2088527475 => return b"prl_fs\0" as *const u8 as *const libc::c_char,
        40864 => return b"proc\0" as *const u8 as *const libc::c_char,
        1634035564 => return b"pstorefs\0" as *const u8 as *const libc::c_char,
        47 => return b"qnx4\0" as *const u8 as *const libc::c_char,
        1746473250 => return b"qnx6\0" as *const u8 as *const libc::c_char,
        2240043254 => return b"ramfs\0" as *const u8 as *const libc::c_char,
        124082209 => return b"rdt\0" as *const u8 as *const libc::c_char,
        1382369651 => return b"reiserfs\0" as *const u8 as *const libc::c_char,
        29301 => return b"romfs\0" as *const u8 as *const libc::c_char,
        1733912937 => return b"rpc_pipefs\0" as *const u8 as *const libc::c_char,
        1573531125 => return b"sdcardfs\0" as *const u8 as *const libc::c_char,
        1397048141 => return b"secretmem\0" as *const u8 as *const libc::c_char,
        1935894131 => return b"securityfs\0" as *const u8 as *const libc::c_char,
        4185718668 => return b"selinux\0" as *const u8 as *const libc::c_char,
        1128357203 => return b"smackfs\0" as *const u8 as *const libc::c_char,
        20859 => return b"smb\0" as *const u8 as *const libc::c_char,
        4266872130 => return b"smb2\0" as *const u8 as *const libc::c_char,
        3203391149 => return b"snfs\0" as *const u8 as *const libc::c_char,
        1397703499 => return b"sockfs\0" as *const u8 as *const libc::c_char,
        1936814952 => return b"squashfs\0" as *const u8 as *const libc::c_char,
        1650812274 => return b"sysfs\0" as *const u8 as *const libc::c_char,
        19920822 => return b"sysv2\0" as *const u8 as *const libc::c_char,
        19920821 => return b"sysv4\0" as *const u8 as *const libc::c_char,
        16914836 => return b"tmpfs\0" as *const u8 as *const libc::c_char,
        1953653091 => return b"tracefs\0" as *const u8 as *const libc::c_char,
        604313861 => return b"ubifs\0" as *const u8 as *const libc::c_char,
        352400198 => return b"udf\0" as *const u8 as *const libc::c_char,
        72020 => return b"ufs\0" as *const u8 as *const libc::c_char,
        1410924800 => return b"ufs\0" as *const u8 as *const libc::c_char,
        40866 => return b"usbdevfs\0" as *const u8 as *const libc::c_char,
        16914839 => return b"v9fs\0" as *const u8 as *const libc::c_char,
        2020557398 => return b"vboxsf\0" as *const u8 as *const libc::c_char,
        3133910204 => return b"vmhgfs\0" as *const u8 as *const libc::c_char,
        2768370933 => return b"vxfs\0" as *const u8 as *const libc::c_char,
        1448756819 => return b"vzfs\0" as *const u8 as *const libc::c_char,
        1397114950 => return b"wslfs\0" as *const u8 as *const libc::c_char,
        2881100148 => return b"xenfs\0" as *const u8 as *const libc::c_char,
        19920820 => return b"xenix\0" as *const u8 as *const libc::c_char,
        1481003842 => return b"xfs\0" as *const u8 as *const libc::c_char,
        19911021 => return b"xia\0" as *const u8 as *const libc::c_char,
        51 => return b"z3fold\0" as *const u8 as *const libc::c_char,
        801189825 => return b"zfs\0" as *const u8 as *const libc::c_char,
        1515144787 => return b"zonefs\0" as *const u8 as *const libc::c_char,
        1479104553 => return b"zsmallocfs\0" as *const u8 as *const libc::c_char,
        _ => {
            let mut type_0: libc::c_ulong = (*statfsbuf).f_type as libc::c_ulong;
            static mut buf: [libc::c_char; 29] = [0; 29];
            sprintf(
                buf.as_mut_ptr(),
                b"UNKNOWN (0x%lx)\0" as *const u8 as *const libc::c_char,
                type_0,
            );
            return buf.as_mut_ptr();
        }
    };
}
unsafe extern "C" fn human_access(mut statbuf: *const stat) -> *mut libc::c_char {
    static mut modebuf: [libc::c_char; 12] = [0; 12];
    filemodestring(statbuf, modebuf.as_mut_ptr());
    modebuf[10 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    return modebuf.as_mut_ptr();
}
unsafe extern "C" fn human_time(mut t: timespec) -> *mut libc::c_char {
    static mut str: [libc::c_char; 61] = [0; 61];
    static mut tz: timezone_t = 0 as *const tm_zone as *mut tm_zone;
    if tz.is_null() {
        tz = tzalloc(getenv(b"TZ\0" as *const u8 as *const libc::c_char));
    }
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
    let mut ns: libc::c_int = t.tv_nsec as libc::c_int;
    if !(localtime_rz(tz, &mut t.tv_sec, &mut tm)).is_null() {
        nstrftime(
            str.as_mut_ptr(),
            ::core::mem::size_of::<[libc::c_char; 61]>() as libc::c_ulong,
            b"%Y-%m-%d %H:%M:%S.%N %z\0" as *const u8 as *const libc::c_char,
            &mut tm,
            tz,
            ns,
        );
    } else {
        let mut secbuf: [libc::c_char; 21] = [0; 21];
        sprintf(
            str.as_mut_ptr(),
            b"%s.%09d\0" as *const u8 as *const libc::c_char,
            timetostr(t.tv_sec, secbuf.as_mut_ptr()),
            ns,
        );
    }
    return str.as_mut_ptr();
}
unsafe extern "C" fn make_format(
    mut pformat: *mut libc::c_char,
    mut prefix_len: size_t,
    mut allowed_flags: *const libc::c_char,
    mut suffix: *const libc::c_char,
) {
    let mut dst: *mut libc::c_char = pformat.offset(1 as libc::c_int as isize);
    let mut src: *const libc::c_char = 0 as *const libc::c_char;
    let mut srclim: *const libc::c_char = pformat.offset(prefix_len as isize);
    src = dst;
    while src < srclim && !(strchr(printf_flags.as_ptr(), *src as libc::c_int)).is_null()
    {
        if !(strchr(allowed_flags, *src as libc::c_int)).is_null() {
            let fresh2 = dst;
            dst = dst.offset(1);
            *fresh2 = *src;
        }
        src = src.offset(1);
    }
    while src < srclim {
        let fresh3 = src;
        src = src.offset(1);
        let fresh4 = dst;
        dst = dst.offset(1);
        *fresh4 = *fresh3;
    }
    strcpy(dst, suffix);
}
unsafe extern "C" fn out_string(
    mut pformat: *mut libc::c_char,
    mut prefix_len: size_t,
    mut arg: *const libc::c_char,
) {
    make_format(
        pformat,
        prefix_len,
        b"-\0" as *const u8 as *const libc::c_char,
        b"s\0" as *const u8 as *const libc::c_char,
    );
    printf(pformat, arg);
}
unsafe extern "C" fn out_int(
    mut pformat: *mut libc::c_char,
    mut prefix_len: size_t,
    mut arg: intmax_t,
) -> libc::c_int {
    make_format(
        pformat,
        prefix_len,
        b"'-+ 0\0" as *const u8 as *const libc::c_char,
        b"ld\0" as *const u8 as *const libc::c_char,
    );
    return printf(pformat, arg);
}
unsafe extern "C" fn out_uint(
    mut pformat: *mut libc::c_char,
    mut prefix_len: size_t,
    mut arg: uintmax_t,
) -> libc::c_int {
    make_format(
        pformat,
        prefix_len,
        b"'-0\0" as *const u8 as *const libc::c_char,
        b"lu\0" as *const u8 as *const libc::c_char,
    );
    return printf(pformat, arg);
}
unsafe extern "C" fn out_uint_o(
    mut pformat: *mut libc::c_char,
    mut prefix_len: size_t,
    mut arg: uintmax_t,
) {
    make_format(
        pformat,
        prefix_len,
        b"-#0\0" as *const u8 as *const libc::c_char,
        b"lo\0" as *const u8 as *const libc::c_char,
    );
    printf(pformat, arg);
}
unsafe extern "C" fn out_uint_x(
    mut pformat: *mut libc::c_char,
    mut prefix_len: size_t,
    mut arg: uintmax_t,
) {
    make_format(
        pformat,
        prefix_len,
        b"-#0\0" as *const u8 as *const libc::c_char,
        b"lx\0" as *const u8 as *const libc::c_char,
    );
    printf(pformat, arg);
}
unsafe extern "C" fn out_minus_zero(
    mut pformat: *mut libc::c_char,
    mut prefix_len: size_t,
) -> libc::c_int {
    make_format(
        pformat,
        prefix_len,
        b"'-+ 0\0" as *const u8 as *const libc::c_char,
        b".0f\0" as *const u8 as *const libc::c_char,
    );
    return printf(pformat, -0.25f64);
}
unsafe extern "C" fn out_epoch_sec(
    mut pformat: *mut libc::c_char,
    mut prefix_len: size_t,
    mut arg: timespec,
) {
    let mut dot: *mut libc::c_char = memchr(
        pformat as *const libc::c_void,
        '.' as i32,
        prefix_len,
    ) as *mut libc::c_char;
    let mut sec_prefix_len: size_t = prefix_len;
    let mut width: libc::c_int = 0 as libc::c_int;
    let mut precision: libc::c_int = 0 as libc::c_int;
    let mut frac_left_adjust: bool = 0 as libc::c_int != 0;
    if !dot.is_null() {
        sec_prefix_len = dot.offset_from(pformat) as libc::c_long as size_t;
        *pformat.offset(prefix_len as isize) = '\0' as i32 as libc::c_char;
        if (*dot.offset(1 as libc::c_int as isize) as libc::c_uint)
            .wrapping_sub('0' as i32 as libc::c_uint) <= 9 as libc::c_int as libc::c_uint
        {
            let mut lprec: libc::c_long = strtol(
                dot.offset(1 as libc::c_int as isize),
                0 as *mut *mut libc::c_char,
                10 as libc::c_int,
            );
            precision = (if lprec <= 2147483647 as libc::c_int as libc::c_long {
                lprec
            } else {
                2147483647 as libc::c_int as libc::c_long
            }) as libc::c_int;
        } else {
            precision = 9 as libc::c_int;
        }
        if precision != 0
            && (*dot.offset(-(1 as libc::c_int) as isize) as libc::c_uint)
                .wrapping_sub('0' as i32 as libc::c_uint)
                <= 9 as libc::c_int as libc::c_uint
        {
            let mut p: *mut libc::c_char = dot;
            *dot = '\0' as i32 as libc::c_char;
            loop {
                p = p.offset(-1);
                if !((*p.offset(-(1 as libc::c_int) as isize) as libc::c_uint)
                    .wrapping_sub('0' as i32 as libc::c_uint)
                    <= 9 as libc::c_int as libc::c_uint)
                {
                    break;
                }
            }
            let mut lwidth: libc::c_long = strtol(
                p,
                0 as *mut *mut libc::c_char,
                10 as libc::c_int,
            );
            width = (if lwidth <= 2147483647 as libc::c_int as libc::c_long {
                lwidth
            } else {
                2147483647 as libc::c_int as libc::c_long
            }) as libc::c_int;
            if (1 as libc::c_int) < width {
                p = p.offset((*p as libc::c_int == '0' as i32) as libc::c_int as isize);
                sec_prefix_len = p.offset_from(pformat) as libc::c_long as size_t;
                let mut w_d: libc::c_int = (if decimal_point_len < width as libc::c_ulong
                {
                    (width as libc::c_ulong).wrapping_sub(decimal_point_len)
                } else {
                    0 as libc::c_int as libc::c_ulong
                }) as libc::c_int;
                if (1 as libc::c_int) < w_d {
                    let mut w: libc::c_int = w_d - precision;
                    if (1 as libc::c_int) < w {
                        let mut dst: *mut libc::c_char = pformat;
                        let mut src: *const libc::c_char = dst;
                        while src < p as *const libc::c_char {
                            if *src as libc::c_int == '-' as i32 {
                                frac_left_adjust = 1 as libc::c_int != 0;
                            } else {
                                let fresh5 = dst;
                                dst = dst.offset(1);
                                *fresh5 = *src;
                            }
                            src = src.offset(1);
                        }
                        sec_prefix_len = (dst.offset_from(pformat) as libc::c_long
                            + (if frac_left_adjust as libc::c_int != 0 {
                                0 as libc::c_int
                            } else {
                                sprintf(dst, b"%d\0" as *const u8 as *const libc::c_char, w)
                            }) as libc::c_long) as size_t;
                    }
                }
            }
        }
    }
    let mut divisor: libc::c_int = 1 as libc::c_int;
    let mut i: libc::c_int = precision;
    while i < 9 as libc::c_int {
        divisor *= 10 as libc::c_int;
        i += 1;
    }
    let mut frac_sec: libc::c_int = (arg.tv_nsec / divisor as libc::c_long)
        as libc::c_int;
    let mut int_len: libc::c_int = 0;
    if !((0 as libc::c_int as time_t) < -(1 as libc::c_int) as time_t) {
        let mut minus_zero: bool = 0 as libc::c_int != 0;
        if arg.tv_sec < 0 as libc::c_int as libc::c_long
            && arg.tv_nsec != 0 as libc::c_int as libc::c_long
        {
            let mut frac_sec_modulus: libc::c_int = 1000000000 as libc::c_int / divisor;
            frac_sec = frac_sec_modulus - frac_sec
                - (arg.tv_nsec % divisor as libc::c_long
                    != 0 as libc::c_int as libc::c_long) as libc::c_int;
            arg.tv_sec += (frac_sec != 0 as libc::c_int) as libc::c_int as libc::c_long;
            minus_zero = arg.tv_sec == 0 as libc::c_int as libc::c_long;
        }
        int_len = if minus_zero as libc::c_int != 0 {
            out_minus_zero(pformat, sec_prefix_len)
        } else {
            out_int(pformat, sec_prefix_len, arg.tv_sec)
        };
    } else {
        int_len = out_uint(pformat, sec_prefix_len, arg.tv_sec as uintmax_t);
    }
    if precision != 0 {
        let mut prec: libc::c_int = if precision < 9 as libc::c_int {
            precision
        } else {
            9 as libc::c_int
        };
        let mut trailing_prec: libc::c_int = precision - prec;
        let mut ilen: libc::c_int = if int_len < 0 as libc::c_int {
            0 as libc::c_int
        } else {
            int_len
        };
        let mut trailing_width: libc::c_int = (if ilen < width
            && decimal_point_len < (width - ilen) as libc::c_ulong
        {
            ((width - ilen) as libc::c_ulong)
                .wrapping_sub(decimal_point_len)
                .wrapping_sub(prec as libc::c_ulong)
        } else {
            0 as libc::c_int as libc::c_ulong
        }) as libc::c_int;
        printf(
            b"%s%.*d%-*.*d\0" as *const u8 as *const libc::c_char,
            decimal_point,
            prec,
            frac_sec,
            trailing_width,
            trailing_prec,
            0 as libc::c_int,
        );
    }
}
unsafe extern "C" fn out_file_context(
    mut pformat: *mut libc::c_char,
    mut prefix_len: size_t,
    mut filename: *const libc::c_char,
) -> bool {
    let mut scontext: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut fail: bool = 0 as libc::c_int != 0;
    if (if follow_links as libc::c_int != 0 {
        getfilecon(filename, &mut scontext)
    } else {
        lgetfilecon(filename, &mut scontext)
    }) < 0 as libc::c_int
    {
        error(
            0 as libc::c_int,
            *__errno_location(),
            dcgettext(
                0 as *const libc::c_char,
                b"failed to get security context of %s\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
            quotearg_style(shell_escape_always_quoting_style, filename),
        );
        scontext = 0 as *mut libc::c_char;
        fail = 1 as libc::c_int != 0;
    }
    strcpy(
        pformat.offset(prefix_len as isize),
        b"s\0" as *const u8 as *const libc::c_char,
    );
    printf(
        pformat,
        if !scontext.is_null() {
            scontext as *const libc::c_char
        } else {
            b"?\0" as *const u8 as *const libc::c_char
        },
    );
    if !scontext.is_null() {
        freecon(scontext);
    }
    return fail;
}
unsafe extern "C" fn print_statfs(
    mut pformat: *mut libc::c_char,
    mut prefix_len: size_t,
    mut mod_0: libc::c_char,
    mut m: libc::c_char,
    mut fd: libc::c_int,
    mut filename: *const libc::c_char,
    mut data: *const libc::c_void,
) -> bool {
    let mut statfsbuf: *const statfs = data as *const statfs;
    let mut fail: bool = 0 as libc::c_int != 0;
    match m as libc::c_int {
        110 => {
            out_string(pformat, prefix_len, filename);
        }
        105 => {
            let mut p: *const fsid_word = &(*statfsbuf).f_fsid as *const __fsid_t
                as *mut fsid_word;
            let mut fsid: uintmax_t = 0 as libc::c_int as uintmax_t;
            let mut words: libc::c_int = (::core::mem::size_of::<__fsid_t>()
                as libc::c_ulong)
                .wrapping_div(::core::mem::size_of::<fsid_word>() as libc::c_ulong)
                as libc::c_int;
            let mut i: libc::c_int = 0 as libc::c_int;
            while i < words
                && (i as libc::c_ulong)
                    .wrapping_mul(::core::mem::size_of::<fsid_word>() as libc::c_ulong)
                    < ::core::mem::size_of::<uintmax_t>() as libc::c_ulong
            {
                let mut u: uintmax_t = *p.offset((words - 1 as libc::c_int - i) as isize)
                    as uintmax_t;
                fsid
                    |= u
                        << ((i * 8 as libc::c_int) as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<fsid_word>() as libc::c_ulong,
                            );
                i += 1;
            }
            out_uint_x(pformat, prefix_len, fsid);
        }
        108 => {
            out_uint(pformat, prefix_len, (*statfsbuf).f_namelen as uintmax_t);
        }
        116 => {
            out_uint_x(pformat, prefix_len, (*statfsbuf).f_type as uintmax_t);
        }
        84 => {
            out_string(pformat, prefix_len, human_fstype(statfsbuf));
        }
        98 => {
            out_int(pformat, prefix_len, (*statfsbuf).f_blocks as intmax_t);
        }
        102 => {
            out_int(pformat, prefix_len, (*statfsbuf).f_bfree as intmax_t);
        }
        97 => {
            out_int(pformat, prefix_len, (*statfsbuf).f_bavail as intmax_t);
        }
        115 => {
            out_uint(pformat, prefix_len, (*statfsbuf).f_bsize as uintmax_t);
        }
        83 => {
            let mut frsize: uintmax_t = (*statfsbuf).f_frsize as uintmax_t;
            if frsize == 0 {
                frsize = (*statfsbuf).f_bsize as uintmax_t;
            }
            out_uint(pformat, prefix_len, frsize);
        }
        99 => {
            out_uint(pformat, prefix_len, (*statfsbuf).f_files);
        }
        100 => {
            out_int(pformat, prefix_len, (*statfsbuf).f_ffree as intmax_t);
        }
        _ => {
            fputc_unlocked('?' as i32, stdout);
        }
    }
    return fail;
}
unsafe extern "C" fn find_bind_mount(
    mut name: *const libc::c_char,
) -> *const libc::c_char {
    let mut bind_mount: *const libc::c_char = 0 as *const libc::c_char;
    static mut mount_list: *mut mount_entry = 0 as *const mount_entry
        as *mut mount_entry;
    static mut tried_mount_list: bool = 0 as libc::c_int != 0;
    if !tried_mount_list {
        mount_list = read_file_system_list(0 as libc::c_int != 0);
        if mount_list.is_null() {
            error(
                0 as libc::c_int,
                *__errno_location(),
                b"%s\0" as *const u8 as *const libc::c_char,
                dcgettext(
                    0 as *const libc::c_char,
                    b"cannot read table of mounted file systems\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
            );
        }
        tried_mount_list = 1 as libc::c_int != 0;
    }
    let mut name_stats: stat = stat {
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
    if stat(name, &mut name_stats) != 0 as libc::c_int {
        return 0 as *const libc::c_char;
    }
    let mut me: *mut mount_entry = 0 as *mut mount_entry;
    me = mount_list;
    while !me.is_null() {
        if (*me).me_dummy() as libc::c_int != 0
            && *((*me).me_devname).offset(0 as libc::c_int as isize) as libc::c_int
                == '/' as i32 && strcmp((*me).me_mountdir, name) == 0 as libc::c_int
        {
            let mut dev_stats: stat = stat {
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
            if stat((*me).me_devname, &mut dev_stats) == 0 as libc::c_int
                && (name_stats.st_ino == dev_stats.st_ino
                    && name_stats.st_dev == dev_stats.st_dev)
            {
                bind_mount = (*me).me_devname;
                break;
            }
        }
        me = (*me).me_next;
    }
    return bind_mount;
}
unsafe extern "C" fn out_mount_point(
    mut filename: *const libc::c_char,
    mut pformat: *mut libc::c_char,
    mut prefix_len: size_t,
    mut statp: *const stat,
) -> bool {
    let mut current_block: u64;
    let mut np: *const libc::c_char = b"?\0" as *const u8 as *const libc::c_char;
    let mut bp: *const libc::c_char = 0 as *const libc::c_char;
    let mut mp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut fail: bool = 1 as libc::c_int != 0;
    if follow_links as libc::c_int != 0
        || !((*statp).st_mode & 0o170000 as libc::c_int as libc::c_uint
            == 0o120000 as libc::c_int as libc::c_uint)
    {
        let mut resolved: *mut libc::c_char = canonicalize_file_name(filename);
        if resolved.is_null() {
            error(
                0 as libc::c_int,
                *__errno_location(),
                dcgettext(
                    0 as *const libc::c_char,
                    b"failed to canonicalize %s\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
                quotearg_style(shell_escape_always_quoting_style, filename),
            );
            current_block = 9806015214421905677;
        } else {
            bp = find_bind_mount(resolved);
            free(resolved as *mut libc::c_void);
            if !bp.is_null() {
                fail = 0 as libc::c_int != 0;
                current_block = 9806015214421905677;
            } else {
                current_block = 10886091980245723256;
            }
        }
    } else {
        current_block = 10886091980245723256;
    }
    match current_block {
        10886091980245723256 => {
            mp = find_mount_point(filename, statp);
            if !mp.is_null() {
                bp = find_bind_mount(mp);
                fail = 0 as libc::c_int != 0;
            }
        }
        _ => {}
    }
    out_string(
        pformat,
        prefix_len,
        if !bp.is_null() {
            bp
        } else if !mp.is_null() {
            mp as *const libc::c_char
        } else {
            np
        },
    );
    free(mp as *mut libc::c_void);
    return fail;
}
#[inline]
unsafe extern "C" fn neg_to_zero(mut ts: timespec) -> timespec {
    if 0 as libc::c_int as libc::c_long <= ts.tv_nsec {
        return ts;
    }
    let mut z: timespec = {
        let mut init = timespec {
            tv_sec: 0 as libc::c_int as __time_t,
            tv_nsec: 0 as libc::c_int as __syscall_slong_t,
        };
        init
    };
    return z;
}
unsafe extern "C" fn getenv_quoting_style() {
    let mut q_style: *const libc::c_char = getenv(
        b"QUOTING_STYLE\0" as *const u8 as *const libc::c_char,
    );
    if !q_style.is_null() {
        let mut i: libc::c_int = argmatch(
            q_style,
            quoting_style_args.as_ptr(),
            quoting_style_vals.as_ptr() as *const libc::c_void,
            ::core::mem::size_of::<quoting_style>() as libc::c_ulong,
        ) as libc::c_int;
        if 0 as libc::c_int <= i {
            set_quoting_style(
                0 as *mut quoting_options,
                *quoting_style_vals.as_ptr().offset(i as isize),
            );
        } else {
            set_quoting_style(
                0 as *mut quoting_options,
                shell_escape_always_quoting_style,
            );
            error(
                0 as libc::c_int,
                0 as libc::c_int,
                dcgettext(
                    0 as *const libc::c_char,
                    b"ignoring invalid value of environment variable QUOTING_STYLE: %s\0"
                        as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
                quote(q_style),
            );
        }
    } else {
        set_quoting_style(0 as *mut quoting_options, shell_escape_always_quoting_style);
    };
}
unsafe extern "C" fn print_esc_char(mut c: libc::c_char) {
    match c as libc::c_int {
        97 => {
            c = '\u{7}' as i32 as libc::c_char;
        }
        98 => {
            c = '\u{8}' as i32 as libc::c_char;
        }
        101 => {
            c = '\u{1b}' as i32 as libc::c_char;
        }
        102 => {
            c = '\u{c}' as i32 as libc::c_char;
        }
        110 => {
            c = '\n' as i32 as libc::c_char;
        }
        114 => {
            c = '\r' as i32 as libc::c_char;
        }
        116 => {
            c = '\t' as i32 as libc::c_char;
        }
        118 => {
            c = '\u{b}' as i32 as libc::c_char;
        }
        34 | 92 => {}
        _ => {
            error(
                0 as libc::c_int,
                0 as libc::c_int,
                dcgettext(
                    0 as *const libc::c_char,
                    b"warning: unrecognized escape '\\%c'\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
                c as libc::c_int,
            );
        }
    }
    putchar_unlocked(c as libc::c_int);
}
unsafe extern "C" fn format_code_offset(mut directive: *const libc::c_char) -> size_t {
    let mut len: size_t = strspn(
        directive.offset(1 as libc::c_int as isize),
        printf_flags.as_ptr(),
    );
    let mut fmt_char: *const libc::c_char = directive
        .offset(len as isize)
        .offset(1 as libc::c_int as isize);
    fmt_char = fmt_char.offset(strspn(fmt_char, digits.as_ptr()) as isize);
    if *fmt_char as libc::c_int == '.' as i32 {
        fmt_char = fmt_char
            .offset(
                (1 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        strspn(
                            fmt_char.offset(1 as libc::c_int as isize),
                            digits.as_ptr(),
                        ),
                    ) as isize,
            );
    }
    return fmt_char.offset_from(directive) as libc::c_long as size_t;
}
unsafe extern "C" fn print_it(
    mut format: *const libc::c_char,
    mut fd: libc::c_int,
    mut filename: *const libc::c_char,
    mut print_func: Option::<
        unsafe extern "C" fn(
            *mut libc::c_char,
            size_t,
            libc::c_char,
            libc::c_char,
            libc::c_int,
            *const libc::c_char,
            *const libc::c_void,
        ) -> bool,
    >,
    mut data: *const libc::c_void,
) -> bool {
    let mut fail: bool = 0 as libc::c_int != 0;
    let mut n_alloc: size_t = (strlen(format))
        .wrapping_add(MAX_ADDITIONAL_BYTES as libc::c_int as libc::c_ulong)
        .wrapping_add(1 as libc::c_int as libc::c_ulong);
    let mut dest: *mut libc::c_char = xmalloc(n_alloc) as *mut libc::c_char;
    let mut b: *const libc::c_char = 0 as *const libc::c_char;
    b = format;
    while *b != 0 {
        match *b as libc::c_int {
            37 => {
                let mut len: size_t = format_code_offset(b);
                let mut fmt_char: libc::c_char = *b.offset(len as isize);
                let mut mod_char: libc::c_char = 0 as libc::c_int as libc::c_char;
                memcpy(dest as *mut libc::c_void, b as *const libc::c_void, len);
                b = b.offset(len as isize);
                let mut current_block_20: u64;
                match fmt_char as libc::c_int {
                    0 => {
                        b = b.offset(-1);
                        current_block_20 = 12151942652722875123;
                    }
                    37 => {
                        current_block_20 = 12151942652722875123;
                    }
                    72 | 76 => {
                        mod_char = fmt_char;
                        fmt_char = *b.offset(1 as libc::c_int as isize);
                        if print_func
                            == Some(
                                print_stat
                                    as unsafe extern "C" fn(
                                        *mut libc::c_char,
                                        size_t,
                                        libc::c_char,
                                        libc::c_char,
                                        libc::c_int,
                                        *const libc::c_char,
                                        *const libc::c_void,
                                    ) -> bool,
                            )
                            && (fmt_char as libc::c_int == 'd' as i32
                                || fmt_char as libc::c_int == 'r' as i32)
                        {
                            b = b.offset(1);
                        } else {
                            fmt_char = mod_char;
                            mod_char = 0 as libc::c_int as libc::c_char;
                        }
                        current_block_20 = 3550530338088759072;
                    }
                    _ => {
                        current_block_20 = 3550530338088759072;
                    }
                }
                match current_block_20 {
                    3550530338088759072 => {
                        fail = (fail as libc::c_int
                            | print_func
                                .expect(
                                    "non-null function pointer",
                                )(dest, len, mod_char, fmt_char, fd, filename, data)
                                as libc::c_int) as bool;
                    }
                    _ => {
                        if (1 as libc::c_int as libc::c_ulong) < len {
                            *dest.offset(len as isize) = fmt_char;
                            *dest
                                .offset(
                                    len.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                                ) = '\0' as i32 as libc::c_char;
                            if ::core::mem::size_of::<C2RustUnnamed_2>() as libc::c_ulong
                                != 0
                            {
                                error(
                                    1 as libc::c_int,
                                    0 as libc::c_int,
                                    dcgettext(
                                        0 as *const libc::c_char,
                                        b"%s: invalid directive\0" as *const u8
                                            as *const libc::c_char,
                                        5 as libc::c_int,
                                    ),
                                    quote(dest),
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
                                        b"%s: invalid directive\0" as *const u8
                                            as *const libc::c_char,
                                        5 as libc::c_int,
                                    ),
                                    quote(dest),
                                );
                                if 0 as libc::c_int != 0 {} else {
                                    unreachable!();
                                };
                            };
                        }
                        putchar_unlocked('%' as i32);
                    }
                }
            }
            92 => {
                if !interpret_backslash_escapes {
                    putchar_unlocked('\\' as i32);
                } else {
                    b = b.offset(1);
                    if '0' as i32 <= *b as libc::c_int && *b as libc::c_int <= '7' as i32
                    {
                        let mut esc_value: libc::c_int = *b as libc::c_int - '0' as i32;
                        let mut esc_length: libc::c_int = 1 as libc::c_int;
                        b = b.offset(1);
                        while esc_length < 3 as libc::c_int
                            && ('0' as i32 <= *b as libc::c_int
                                && *b as libc::c_int <= '7' as i32)
                        {
                            esc_value = esc_value * 8 as libc::c_int
                                + (*b as libc::c_int - '0' as i32);
                            esc_length += 1;
                            b = b.offset(1);
                        }
                        putchar_unlocked(esc_value);
                        b = b.offset(-1);
                    } else if *b as libc::c_int == 'x' as i32
                        && *(*__ctype_b_loc())
                            .offset(
                                to_uchar(*b.offset(1 as libc::c_int as isize))
                                    as libc::c_int as isize,
                            ) as libc::c_int
                            & _ISxdigit as libc::c_int as libc::c_ushort as libc::c_int
                            != 0
                    {
                        let mut esc_value_0: libc::c_int = if *b
                            .offset(1 as libc::c_int as isize) as libc::c_int
                            >= 'a' as i32
                            && *b.offset(1 as libc::c_int as isize) as libc::c_int
                                <= 'f' as i32
                        {
                            *b.offset(1 as libc::c_int as isize) as libc::c_int
                                - 'a' as i32 + 10 as libc::c_int
                        } else if *b.offset(1 as libc::c_int as isize) as libc::c_int
                            >= 'A' as i32
                            && *b.offset(1 as libc::c_int as isize) as libc::c_int
                                <= 'F' as i32
                        {
                            *b.offset(1 as libc::c_int as isize) as libc::c_int
                                - 'A' as i32 + 10 as libc::c_int
                        } else {
                            *b.offset(1 as libc::c_int as isize) as libc::c_int
                                - '0' as i32
                        };
                        b = b.offset(1);
                        if *(*__ctype_b_loc())
                            .offset(
                                to_uchar(*b.offset(1 as libc::c_int as isize))
                                    as libc::c_int as isize,
                            ) as libc::c_int
                            & _ISxdigit as libc::c_int as libc::c_ushort as libc::c_int
                            != 0
                        {
                            b = b.offset(1);
                            esc_value_0 = esc_value_0 * 16 as libc::c_int
                                + (if *b as libc::c_int >= 'a' as i32
                                    && *b as libc::c_int <= 'f' as i32
                                {
                                    *b as libc::c_int - 'a' as i32 + 10 as libc::c_int
                                } else {
                                    (if *b as libc::c_int >= 'A' as i32
                                        && *b as libc::c_int <= 'F' as i32
                                    {
                                        *b as libc::c_int - 'A' as i32 + 10 as libc::c_int
                                    } else {
                                        *b as libc::c_int - '0' as i32
                                    })
                                });
                        }
                        putchar_unlocked(esc_value_0);
                    } else if *b as libc::c_int == '\0' as i32 {
                        error(
                            0 as libc::c_int,
                            0 as libc::c_int,
                            dcgettext(
                                0 as *const libc::c_char,
                                b"warning: backslash at end of format\0" as *const u8
                                    as *const libc::c_char,
                                5 as libc::c_int,
                            ),
                        );
                        putchar_unlocked('\\' as i32);
                        b = b.offset(-1);
                    } else {
                        print_esc_char(*b);
                    }
                }
            }
            _ => {
                putchar_unlocked(*b as libc::c_int);
            }
        }
        b = b.offset(1);
    }
    free(dest as *mut libc::c_void);
    fputs_unlocked(trailing_delim, stdout);
    return fail;
}
unsafe extern "C" fn do_statfs(
    mut filename: *const libc::c_char,
    mut format: *const libc::c_char,
) -> bool {
    let mut statfsbuf: statfs = statfs {
        f_type: 0,
        f_bsize: 0,
        f_blocks: 0,
        f_bfree: 0,
        f_bavail: 0,
        f_files: 0,
        f_ffree: 0,
        f_fsid: __fsid_t { __val: [0; 2] },
        f_namelen: 0,
        f_frsize: 0,
        f_flags: 0,
        f_spare: [0; 4],
    };
    if strcmp(filename, b"-\0" as *const u8 as *const libc::c_char) == 0 as libc::c_int {
        error(
            0 as libc::c_int,
            0 as libc::c_int,
            dcgettext(
                0 as *const libc::c_char,
                b"using %s to denote standard input does not work in file system mode\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            quotearg_style(shell_escape_always_quoting_style, filename),
        );
        return 0 as libc::c_int != 0;
    }
    if statfs(filename, &mut statfsbuf) != 0 as libc::c_int {
        error(
            0 as libc::c_int,
            *__errno_location(),
            dcgettext(
                0 as *const libc::c_char,
                b"cannot read file system information for %s\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
            quotearg_style(shell_escape_always_quoting_style, filename),
        );
        return 0 as libc::c_int != 0;
    }
    let mut fail: bool = print_it(
        format,
        -(1 as libc::c_int),
        filename,
        Some(
            print_statfs
                as unsafe extern "C" fn(
                    *mut libc::c_char,
                    size_t,
                    libc::c_char,
                    libc::c_char,
                    libc::c_int,
                    *const libc::c_char,
                    *const libc::c_void,
                ) -> bool,
        ),
        &mut statfsbuf as *mut statfs as *const libc::c_void,
    );
    return !fail;
}
static mut dont_sync: bool = false;
static mut force_sync: bool = false;
unsafe extern "C" fn fmt_to_mask(mut fmt: libc::c_char) -> libc::c_uint {
    match fmt as libc::c_int {
        78 => return 0x2 as libc::c_uint,
        100 | 68 => return 0x2 as libc::c_uint,
        105 => return 0x100 as libc::c_uint,
        97 | 65 => return 0x2 as libc::c_uint,
        102 => return 0x2 as libc::c_uint | 0x1 as libc::c_uint,
        70 => return 0x1 as libc::c_uint,
        104 => return 0x4 as libc::c_uint,
        117 | 85 => return 0x8 as libc::c_uint,
        103 | 71 => return 0x10 as libc::c_uint,
        109 => return 0x2 as libc::c_uint | 0x100 as libc::c_uint,
        115 => return 0x200 as libc::c_uint,
        116 | 84 => return 0x2 as libc::c_uint,
        98 => return 0x400 as libc::c_uint,
        119 | 87 => return 0x800 as libc::c_uint,
        120 | 88 => return 0x20 as libc::c_uint,
        121 | 89 => return 0x40 as libc::c_uint,
        122 | 90 => return 0x80 as libc::c_uint,
        _ => {}
    }
    return 0 as libc::c_int as libc::c_uint;
}
unsafe extern "C" fn format_to_mask(mut format: *const libc::c_char) -> libc::c_uint {
    let mut mask: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    let mut b: *const libc::c_char = 0 as *const libc::c_char;
    b = format;
    while *b != 0 {
        if !(*b as libc::c_int != '%' as i32) {
            b = b.offset(format_code_offset(b) as isize);
            if *b as libc::c_int == '\0' as i32 {
                break;
            }
            mask |= fmt_to_mask(*b);
        }
        b = b.offset(1);
    }
    return mask;
}
unsafe extern "C" fn do_stat(
    mut filename: *const libc::c_char,
    mut format: *const libc::c_char,
    mut format2: *const libc::c_char,
) -> bool {
    let mut fd: libc::c_int = if strcmp(
        filename,
        b"-\0" as *const u8 as *const libc::c_char,
    ) == 0 as libc::c_int
    {
        0 as libc::c_int
    } else {
        -(100 as libc::c_int)
    };
    let mut flags: libc::c_int = 0 as libc::c_int;
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
    let mut stx: statx = {
        let mut init = statx {
            stx_mask: 0 as libc::c_int as __u32,
            stx_blksize: 0,
            stx_attributes: 0,
            stx_nlink: 0,
            stx_uid: 0,
            stx_gid: 0,
            stx_mode: 0,
            __spare0: [0; 1],
            stx_ino: 0,
            stx_size: 0,
            stx_blocks: 0,
            stx_attributes_mask: 0,
            stx_atime: statx_timestamp {
                tv_sec: 0,
                tv_nsec: 0,
                __reserved: 0,
            },
            stx_btime: statx_timestamp {
                tv_sec: 0,
                tv_nsec: 0,
                __reserved: 0,
            },
            stx_ctime: statx_timestamp {
                tv_sec: 0,
                tv_nsec: 0,
                __reserved: 0,
            },
            stx_mtime: statx_timestamp {
                tv_sec: 0,
                tv_nsec: 0,
                __reserved: 0,
            },
            stx_rdev_major: 0,
            stx_rdev_minor: 0,
            stx_dev_major: 0,
            stx_dev_minor: 0,
            stx_mnt_id: 0,
            __spare2: 0,
            __spare3: [0; 12],
        };
        init
    };
    let mut pathname: *const libc::c_char = filename;
    let mut pa: print_args = print_args {
        st: 0 as *mut stat,
        btime: timespec { tv_sec: 0, tv_nsec: 0 },
    };
    pa.st = &mut st;
    pa
        .btime = {
        let mut init = timespec {
            tv_sec: -(1 as libc::c_int) as __time_t,
            tv_nsec: -(1 as libc::c_int) as __syscall_slong_t,
        };
        init
    };
    if -(100 as libc::c_int) != fd {
        pathname = b"\0" as *const u8 as *const libc::c_char;
        flags = 0x1000 as libc::c_int;
    } else if !follow_links {
        flags = 0x100 as libc::c_int;
    }
    if dont_sync {
        flags |= 0x4000 as libc::c_int;
    } else if force_sync {
        flags |= 0x2000 as libc::c_int;
    }
    if !force_sync {
        flags |= 0x800 as libc::c_int;
    }
    fd = statx(fd, pathname, flags, format_to_mask(format), &mut stx);
    if fd < 0 as libc::c_int {
        if flags & 0x1000 as libc::c_int != 0 {
            error(
                0 as libc::c_int,
                *__errno_location(),
                dcgettext(
                    0 as *const libc::c_char,
                    b"cannot stat standard input\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
            );
        } else {
            error(
                0 as libc::c_int,
                *__errno_location(),
                dcgettext(
                    0 as *const libc::c_char,
                    b"cannot statx %s\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
                quotearg_style(shell_escape_always_quoting_style, filename),
            );
        }
        return 0 as libc::c_int != 0;
    }
    if stx.stx_mode as libc::c_int & 0o170000 as libc::c_int == 0o60000 as libc::c_int
        || stx.stx_mode as libc::c_int & 0o170000 as libc::c_int
            == 0o20000 as libc::c_int
    {
        format = format2;
    }
    statx_to_stat(&mut stx, &mut st);
    if stx.stx_mask & 0x800 as libc::c_uint != 0 {
        pa.btime = statx_timestamp_to_timespec(stx.stx_btime);
    }
    let mut fail: bool = print_it(
        format,
        fd,
        filename,
        Some(
            print_stat
                as unsafe extern "C" fn(
                    *mut libc::c_char,
                    size_t,
                    libc::c_char,
                    libc::c_char,
                    libc::c_int,
                    *const libc::c_char,
                    *const libc::c_void,
                ) -> bool,
        ),
        &mut pa as *mut print_args as *const libc::c_void,
    );
    return !fail;
}
unsafe extern "C" fn unsigned_file_size(mut size: off_t) -> uintmax_t {
    return (size as libc::c_ulong)
        .wrapping_add(
            ((size < 0 as libc::c_int as libc::c_long) as libc::c_int as libc::c_ulong)
                .wrapping_mul(
                    ((if (0 as libc::c_int as off_t) < -(1 as libc::c_int) as off_t {
                        -(1 as libc::c_int) as off_t
                    } else {
                        (((1 as libc::c_int as off_t)
                            << (::core::mem::size_of::<off_t>() as libc::c_ulong)
                                .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                            - 1 as libc::c_int as libc::c_long)
                            * 2 as libc::c_int as libc::c_long
                            + 1 as libc::c_int as libc::c_long
                    }) as uintmax_t)
                        .wrapping_sub(
                            !(if (0 as libc::c_int as off_t)
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
                            }) as libc::c_ulong,
                        )
                        .wrapping_add(1 as libc::c_int as libc::c_ulong),
                ),
        );
}
unsafe extern "C" fn print_stat(
    mut pformat: *mut libc::c_char,
    mut prefix_len: size_t,
    mut mod_0: libc::c_char,
    mut m: libc::c_char,
    mut fd: libc::c_int,
    mut filename: *const libc::c_char,
    mut data: *const libc::c_void,
) -> bool {
    let mut parg: *mut print_args = data as *mut print_args;
    let mut statbuf: *mut stat = (*parg).st;
    let mut btime: timespec = (*parg).btime;
    let mut pw_ent: *mut passwd = 0 as *mut passwd;
    let mut gw_ent: *mut group = 0 as *mut group;
    let mut fail: bool = 0 as libc::c_int != 0;
    match m as libc::c_int {
        110 => {
            out_string(pformat, prefix_len, filename);
        }
        78 => {
            out_string(
                pformat,
                prefix_len,
                quotearg_style(get_quoting_style(0 as *const quoting_options), filename),
            );
            if (*statbuf).st_mode & 0o170000 as libc::c_int as libc::c_uint
                == 0o120000 as libc::c_int as libc::c_uint
            {
                let mut linkname: *mut libc::c_char = areadlink_with_size(
                    filename,
                    (*statbuf).st_size as size_t,
                );
                if linkname.is_null() {
                    error(
                        0 as libc::c_int,
                        *__errno_location(),
                        dcgettext(
                            0 as *const libc::c_char,
                            b"cannot read symbolic link %s\0" as *const u8
                                as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                        quotearg_style(shell_escape_always_quoting_style, filename),
                    );
                    return 1 as libc::c_int != 0;
                }
                printf(b" -> \0" as *const u8 as *const libc::c_char);
                out_string(
                    pformat,
                    prefix_len,
                    quotearg_style(
                        get_quoting_style(0 as *const quoting_options),
                        linkname,
                    ),
                );
                free(linkname as *mut libc::c_void);
            }
        }
        100 => {
            if mod_0 as libc::c_int == 'H' as i32 {
                out_uint(
                    pformat,
                    prefix_len,
                    gnu_dev_major((*statbuf).st_dev) as uintmax_t,
                );
            } else if mod_0 as libc::c_int == 'L' as i32 {
                out_uint(
                    pformat,
                    prefix_len,
                    gnu_dev_minor((*statbuf).st_dev) as uintmax_t,
                );
            } else {
                out_uint(pformat, prefix_len, (*statbuf).st_dev);
            }
        }
        68 => {
            out_uint_x(pformat, prefix_len, (*statbuf).st_dev);
        }
        105 => {
            out_uint(pformat, prefix_len, (*statbuf).st_ino);
        }
        97 => {
            out_uint_o(
                pformat,
                prefix_len,
                ((*statbuf).st_mode
                    & (0o4000 as libc::c_int | 0o2000 as libc::c_int
                        | 0o1000 as libc::c_int
                        | (0o400 as libc::c_int | 0o200 as libc::c_int
                            | 0o100 as libc::c_int)
                        | (0o400 as libc::c_int | 0o200 as libc::c_int
                            | 0o100 as libc::c_int) >> 3 as libc::c_int
                        | (0o400 as libc::c_int | 0o200 as libc::c_int
                            | 0o100 as libc::c_int) >> 3 as libc::c_int
                            >> 3 as libc::c_int) as libc::c_uint) as uintmax_t,
            );
        }
        65 => {
            out_string(pformat, prefix_len, human_access(statbuf));
        }
        102 => {
            out_uint_x(pformat, prefix_len, (*statbuf).st_mode as uintmax_t);
        }
        70 => {
            out_string(pformat, prefix_len, file_type(statbuf));
        }
        104 => {
            out_uint(pformat, prefix_len, (*statbuf).st_nlink as uintmax_t);
        }
        117 => {
            out_uint(pformat, prefix_len, (*statbuf).st_uid as uintmax_t);
        }
        85 => {
            pw_ent = getpwuid((*statbuf).st_uid);
            out_string(
                pformat,
                prefix_len,
                if !pw_ent.is_null() {
                    (*pw_ent).pw_name as *const libc::c_char
                } else {
                    b"UNKNOWN\0" as *const u8 as *const libc::c_char
                },
            );
        }
        103 => {
            out_uint(pformat, prefix_len, (*statbuf).st_gid as uintmax_t);
        }
        71 => {
            gw_ent = getgrgid((*statbuf).st_gid);
            out_string(
                pformat,
                prefix_len,
                if !gw_ent.is_null() {
                    (*gw_ent).gr_name as *const libc::c_char
                } else {
                    b"UNKNOWN\0" as *const u8 as *const libc::c_char
                },
            );
        }
        109 => {
            fail = (fail as libc::c_int
                | out_mount_point(filename, pformat, prefix_len, statbuf) as libc::c_int)
                as bool;
        }
        115 => {
            out_uint(pformat, prefix_len, unsigned_file_size((*statbuf).st_size));
        }
        114 => {
            if mod_0 as libc::c_int == 'H' as i32 {
                out_uint(
                    pformat,
                    prefix_len,
                    gnu_dev_major((*statbuf).st_rdev) as uintmax_t,
                );
            } else if mod_0 as libc::c_int == 'L' as i32 {
                out_uint(
                    pformat,
                    prefix_len,
                    gnu_dev_minor((*statbuf).st_rdev) as uintmax_t,
                );
            } else {
                out_uint(pformat, prefix_len, (*statbuf).st_rdev);
            }
        }
        82 => {
            out_uint_x(pformat, prefix_len, (*statbuf).st_rdev);
        }
        116 => {
            out_uint_x(
                pformat,
                prefix_len,
                gnu_dev_major((*statbuf).st_rdev) as uintmax_t,
            );
        }
        84 => {
            out_uint_x(
                pformat,
                prefix_len,
                gnu_dev_minor((*statbuf).st_rdev) as uintmax_t,
            );
        }
        66 => {
            out_uint(pformat, prefix_len, 512 as libc::c_int as uintmax_t);
        }
        98 => {
            out_uint(pformat, prefix_len, (*statbuf).st_blocks as uintmax_t);
        }
        111 => {
            out_uint(
                pformat,
                prefix_len,
                (if (0 as libc::c_int) < (*statbuf).st_blksize
                    && (*statbuf).st_blksize as libc::c_ulong
                        <= (-(1 as libc::c_int) as size_t)
                            .wrapping_div(8 as libc::c_int as libc::c_ulong)
                            .wrapping_add(1 as libc::c_int as libc::c_ulong)
                {
                    (*statbuf).st_blksize
                } else {
                    512 as libc::c_int
                }) as uintmax_t,
            );
        }
        119 => {
            if btime.tv_nsec < 0 as libc::c_int as libc::c_long {
                out_string(
                    pformat,
                    prefix_len,
                    b"-\0" as *const u8 as *const libc::c_char,
                );
            } else {
                out_string(pformat, prefix_len, human_time(btime));
            }
        }
        87 => {
            out_epoch_sec(pformat, prefix_len, neg_to_zero(btime));
        }
        120 => {
            out_string(pformat, prefix_len, human_time(get_stat_atime(statbuf)));
        }
        88 => {
            out_epoch_sec(pformat, prefix_len, get_stat_atime(statbuf));
        }
        121 => {
            out_string(pformat, prefix_len, human_time(get_stat_mtime(statbuf)));
        }
        89 => {
            out_epoch_sec(pformat, prefix_len, get_stat_mtime(statbuf));
        }
        122 => {
            out_string(pformat, prefix_len, human_time(get_stat_ctime(statbuf)));
        }
        90 => {
            out_epoch_sec(pformat, prefix_len, get_stat_ctime(statbuf));
        }
        67 => {
            fail = (fail as libc::c_int
                | out_file_context(pformat, prefix_len, filename) as libc::c_int)
                as bool;
        }
        _ => {
            fputc_unlocked('?' as i32, stdout);
        }
    }
    return fail;
}
unsafe extern "C" fn default_format(
    mut fs: bool,
    mut terse: bool,
    mut device: bool,
) -> *mut libc::c_char {
    let mut format: *mut libc::c_char = 0 as *mut libc::c_char;
    if fs {
        if terse {
            format = xstrdup(fmt_terse_fs.as_ptr());
        } else {
            format = xstrdup(
                dcgettext(
                    0 as *const libc::c_char,
                    b"  File: \"%n\"\n    ID: %-8i Namelen: %-7l Type: %T\nBlock size: %-10s Fundamental block size: %S\nBlocks: Total: %-10b Free: %-10f Available: %a\nInodes: Total: %-10c Free: %d\n\0"
                        as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
            );
        }
    } else if terse {
        if (0 as libc::c_int) < 0 as libc::c_int {
            format = xstrdup(fmt_terse_selinux.as_ptr());
        } else {
            format = xstrdup(fmt_terse_regular.as_ptr());
        }
    } else {
        let mut temp: *mut libc::c_char = 0 as *mut libc::c_char;
        format = xstrdup(
            dcgettext(
                0 as *const libc::c_char,
                b"  File: %N\n  Size: %-10s\tBlocks: %-10b IO Block: %-6o %F\n\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
        temp = format;
        if device {
            format = xasprintf(
                b"%s%s\0" as *const u8 as *const libc::c_char,
                format,
                dcgettext(
                    0 as *const libc::c_char,
                    b"Device: %Hd,%Ld\tInode: %-10i  Links: %-5h Device type: %Hr,%Lr\n\0"
                        as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
            );
        } else {
            format = xasprintf(
                b"%s%s\0" as *const u8 as *const libc::c_char,
                format,
                dcgettext(
                    0 as *const libc::c_char,
                    b"Device: %Hd,%Ld\tInode: %-10i  Links: %h\n\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
            );
        }
        free(temp as *mut libc::c_void);
        temp = format;
        format = xasprintf(
            b"%s%s\0" as *const u8 as *const libc::c_char,
            format,
            dcgettext(
                0 as *const libc::c_char,
                b"Access: (%04a/%10.10A)  Uid: (%5u/%8U)   Gid: (%5g/%8G)\n\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
        free(temp as *mut libc::c_void);
        if (0 as libc::c_int) < 0 as libc::c_int {
            temp = format;
            format = xasprintf(
                b"%s%s\0" as *const u8 as *const libc::c_char,
                format,
                dcgettext(
                    0 as *const libc::c_char,
                    b"Context: %C\n\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
            );
            free(temp as *mut libc::c_void);
        }
        temp = format;
        format = xasprintf(
            b"%s%s\0" as *const u8 as *const libc::c_char,
            format,
            dcgettext(
                0 as *const libc::c_char,
                b"Access: %x\nModify: %y\nChange: %z\n Birth: %w\n\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
        free(temp as *mut libc::c_void);
    }
    return format;
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
                b"Usage: %s [OPTION]... FILE...\n\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            program_name,
        );
        fputs_unlocked(
            dcgettext(
                0 as *const libc::c_char,
                b"Display file or file system status.\n\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
            stdout,
        );
        emit_mandatory_arg_note();
        fputs_unlocked(
            dcgettext(
                0 as *const libc::c_char,
                b"  -L, --dereference     follow links\n  -f, --file-system     display file system status instead of file status\n\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            stdout,
        );
        fputs_unlocked(
            dcgettext(
                0 as *const libc::c_char,
                b"      --cached=MODE     specify how to use cached attributes;\n                          useful on remote file systems. See MODE below\n\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            stdout,
        );
        fputs_unlocked(
            dcgettext(
                0 as *const libc::c_char,
                b"  -c  --format=FORMAT   use the specified FORMAT instead of the default;\n                          output a newline after each use of FORMAT\n      --printf=FORMAT   like --format, but interpret backslash escapes,\n                          and do not output a mandatory trailing newline;\n                          if you want a newline, include \\n in FORMAT\n  -t, --terse           print the information in terse form\n\0"
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
                b"\nThe MODE argument of --cached can be: always, never, or default.\n'always' will use cached attributes if available, while\n'never' will try to synchronize with the latest attributes, and\n'default' will leave it up to the underlying file system.\n\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            stdout,
        );
        fputs_unlocked(
            dcgettext(
                0 as *const libc::c_char,
                b"\nThe valid format sequences for files (without --file-system):\n\n  %a   permission bits in octal (note '#' and '0' printf flags)\n  %A   permission bits and file type in human readable form\n  %b   number of blocks allocated (see %B)\n  %B   the size in bytes of each block reported by %b\n  %C   SELinux security context string\n\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            stdout,
        );
        fputs_unlocked(
            dcgettext(
                0 as *const libc::c_char,
                b"  %d   device number in decimal (st_dev)\n  %D   device number in hex (st_dev)\n  %Hd  major device number in decimal\n  %Ld  minor device number in decimal\n  %f   raw mode in hex\n  %F   file type\n  %g   group ID of owner\n  %G   group name of owner\n\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            stdout,
        );
        fputs_unlocked(
            dcgettext(
                0 as *const libc::c_char,
                b"  %h   number of hard links\n  %i   inode number\n  %m   mount point\n  %n   file name\n  %N   quoted file name with dereference if symbolic link\n  %o   optimal I/O transfer size hint\n  %s   total size, in bytes\n  %r   device type in decimal (st_rdev)\n  %R   device type in hex (st_rdev)\n  %Hr  major device type in decimal, for character/block device special files\n  %Lr  minor device type in decimal, for character/block device special files\n  %t   major device type in hex, for character/block device special files\n  %T   minor device type in hex, for character/block device special files\n\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            stdout,
        );
        fputs_unlocked(
            dcgettext(
                0 as *const libc::c_char,
                b"  %u   user ID of owner\n  %U   user name of owner\n  %w   time of file birth, human-readable; - if unknown\n  %W   time of file birth, seconds since Epoch; 0 if unknown\n  %x   time of last access, human-readable\n  %X   time of last access, seconds since Epoch\n  %y   time of last data modification, human-readable\n  %Y   time of last data modification, seconds since Epoch\n  %z   time of last status change, human-readable\n  %Z   time of last status change, seconds since Epoch\n\n\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            stdout,
        );
        fputs_unlocked(
            dcgettext(
                0 as *const libc::c_char,
                b"Valid format sequences for file systems:\n\n  %a   free blocks available to non-superuser\n  %b   total data blocks in file system\n  %c   total file nodes in file system\n  %d   free file nodes in file system\n  %f   free blocks in file system\n\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            stdout,
        );
        fputs_unlocked(
            dcgettext(
                0 as *const libc::c_char,
                b"  %i   file system ID in hex\n  %l   maximum length of filenames\n  %n   file name\n  %s   block size (for faster transfers)\n  %S   fundamental block size (for block counts)\n  %t   file system type in hex\n  %T   file system type in human readable form\n\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            stdout,
        );
        printf(
            dcgettext(
                0 as *const libc::c_char,
                b"\n--terse is equivalent to the following FORMAT:\n    %s\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            fmt_terse_regular.as_ptr(),
        );
        printf(
            dcgettext(
                0 as *const libc::c_char,
                b"--terse --file-system is equivalent to the following FORMAT:\n    %s\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            fmt_terse_fs.as_ptr(),
        );
        printf(
            dcgettext(
                0 as *const libc::c_char,
                b"\nNOTE: your shell may have its own version of %s, which usually supersedes\nthe version described here.  Please refer to your shell's documentation\nfor details about the options it supports.\n\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            b"stat\0" as *const u8 as *const libc::c_char,
        );
        emit_ancillary_info(b"stat\0" as *const u8 as *const libc::c_char);
    }
    exit(status);
}
unsafe fn main_0(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut c: libc::c_int = 0;
    let mut fs: bool = 0 as libc::c_int != 0;
    let mut terse: bool = 0 as libc::c_int != 0;
    let mut format: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut format2: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut ok: bool = 1 as libc::c_int != 0;
    set_program_name(*argv.offset(0 as libc::c_int as isize));
    setlocale(6 as libc::c_int, b"\0" as *const u8 as *const libc::c_char);
    bindtextdomain(
        b"coreutils\0" as *const u8 as *const libc::c_char,
        b"/usr/local/share/locale\0" as *const u8 as *const libc::c_char,
    );
    textdomain(b"coreutils\0" as *const u8 as *const libc::c_char);
    let mut locale: *const lconv = localeconv();
    decimal_point = if *((*locale).decimal_point).offset(0 as libc::c_int as isize)
        as libc::c_int != 0
    {
        (*locale).decimal_point as *const libc::c_char
    } else {
        b".\0" as *const u8 as *const libc::c_char
    };
    decimal_point_len = strlen(decimal_point);
    atexit(Some(close_stdout as unsafe extern "C" fn() -> ()));
    loop {
        c = getopt_long(
            argc,
            argv as *const *mut libc::c_char,
            b"c:fLt\0" as *const u8 as *const libc::c_char,
            long_options.as_ptr(),
            0 as *mut libc::c_int,
        );
        if !(c != -(1 as libc::c_int)) {
            break;
        }
        match c {
            256 => {
                format = optarg;
                interpret_backslash_escapes = 1 as libc::c_int != 0;
                trailing_delim = b"\0" as *const u8 as *const libc::c_char;
            }
            99 => {
                format = optarg;
                interpret_backslash_escapes = 0 as libc::c_int != 0;
                trailing_delim = b"\n\0" as *const u8 as *const libc::c_char;
            }
            76 => {
                follow_links = 1 as libc::c_int != 0;
            }
            102 => {
                fs = 1 as libc::c_int != 0;
            }
            116 => {
                terse = 1 as libc::c_int != 0;
            }
            0 => {
                match cached_modes[__xargmatch_internal(
                    b"--cached\0" as *const u8 as *const libc::c_char,
                    optarg,
                    cached_args.as_ptr(),
                    cached_modes.as_ptr() as *const libc::c_void,
                    ::core::mem::size_of::<cached_mode>() as libc::c_ulong,
                    argmatch_die,
                    1 as libc::c_int != 0,
                ) as usize] as libc::c_uint
                {
                    1 => {
                        force_sync = 1 as libc::c_int != 0;
                        dont_sync = 0 as libc::c_int != 0;
                    }
                    2 => {
                        force_sync = 0 as libc::c_int != 0;
                        dont_sync = 1 as libc::c_int != 0;
                    }
                    0 => {
                        force_sync = 0 as libc::c_int != 0;
                        dont_sync = 0 as libc::c_int != 0;
                    }
                    _ => {}
                }
            }
            -2 => {
                usage(0 as libc::c_int);
            }
            -3 => {
                version_etc(
                    stdout,
                    b"stat\0" as *const u8 as *const libc::c_char,
                    b"GNU coreutils\0" as *const u8 as *const libc::c_char,
                    Version,
                    b"Michael Meskes\0" as *const u8 as *const libc::c_char,
                    0 as *mut libc::c_void as *mut libc::c_char,
                );
                exit(0 as libc::c_int);
            }
            _ => {
                usage(1 as libc::c_int);
            }
        }
    }
    if argc == optind {
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
    if !format.is_null() {
        if !(strstr(format, b"%N\0" as *const u8 as *const libc::c_char)).is_null() {
            getenv_quoting_style();
        }
        format2 = format;
    } else {
        format = default_format(fs, terse, 0 as libc::c_int != 0);
        format2 = default_format(fs, terse, 1 as libc::c_int != 0);
    }
    let mut i: libc::c_int = optind;
    while i < argc {
        ok = (ok as libc::c_int
            & if fs as libc::c_int != 0 {
                do_statfs(*argv.offset(i as isize), format) as libc::c_int
            } else {
                do_stat(*argv.offset(i as isize), format, format2) as libc::c_int
            }) as bool;
        i += 1;
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
