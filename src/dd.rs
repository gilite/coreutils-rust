#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![feature(c_variadic, extern_types)]
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn signal(__sig: libc::c_int, __handler: __sighandler_t) -> __sighandler_t;
    fn raise(__sig: libc::c_int) -> libc::c_int;
    fn sigemptyset(__set: *mut sigset_t) -> libc::c_int;
    fn sigaddset(__set: *mut sigset_t, __signo: libc::c_int) -> libc::c_int;
    fn sigismember(__set: *const sigset_t, __signo: libc::c_int) -> libc::c_int;
    fn sigprocmask(
        __how: libc::c_int,
        __set: *const sigset_t,
        __oset: *mut sigset_t,
    ) -> libc::c_int;
    fn sigaction(
        __sig: libc::c_int,
        __act: *const sigaction,
        __oact: *mut sigaction,
    ) -> libc::c_int;
    fn lseek(__fd: libc::c_int, __offset: __off_t, __whence: libc::c_int) -> __off_t;
    fn close(__fd: libc::c_int) -> libc::c_int;
    fn read(__fd: libc::c_int, __buf: *mut libc::c_void, __nbytes: size_t) -> ssize_t;
    fn write(__fd: libc::c_int, __buf: *const libc::c_void, __n: size_t) -> ssize_t;
    fn _exit(_: libc::c_int) -> !;
    static mut optind: libc::c_int;
    fn fsync(__fd: libc::c_int) -> libc::c_int;
    fn getpagesize() -> libc::c_int;
    fn ftruncate(__fd: libc::c_int, __length: __off_t) -> libc::c_int;
    fn fdatasync(__fildes: libc::c_int) -> libc::c_int;
    fn aligned_alloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn atexit(__func: Option::<unsafe extern "C" fn() -> ()>) -> libc::c_int;
    fn exit(_: libc::c_int) -> !;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn __errno_location() -> *mut libc::c_int;
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strncmp(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strcat(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn memcmp(
        _: *const libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn fstat(__fd: libc::c_int, __buf: *mut stat) -> libc::c_int;
    fn getenv(__name: *const libc::c_char) -> *mut libc::c_char;
    static mut Version: *const libc::c_char;
    fn posix_fadvise(
        __fd: libc::c_int,
        __offset: off_t,
        __len: off_t,
        __advise: libc::c_int,
    ) -> libc::c_int;
    fn rpl_fcntl(fd: libc::c_int, action: libc::c_int, _: ...) -> libc::c_int;
    fn __ctype_tolower_loc() -> *mut *const __int32_t;
    fn __ctype_toupper_loc() -> *mut *const __int32_t;
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
    static mut stdout: *mut FILE;
    static mut stderr: *mut FILE;
    fn quotearg_n_style_colon(
        n: libc::c_int,
        s: quoting_style,
        arg: *const libc::c_char,
    ) -> *mut libc::c_char;
    fn quotearg_style(s: quoting_style, arg: *const libc::c_char) -> *mut libc::c_char;
    fn quotearg_n_style_mem(
        n: libc::c_int,
        s: quoting_style,
        arg: *const libc::c_char,
        argsize: size_t,
    ) -> *mut libc::c_char;
    fn close_stdout();
    static mut program_name: *const libc::c_char;
    fn set_program_name(argv0: *const libc::c_char);
    fn snprintf(
        _: *mut libc::c_char,
        _: libc::c_ulong,
        _: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn fputs_unlocked(__s: *const libc::c_char, __stream: *mut FILE) -> libc::c_int;
    fn __overflow(_: *mut FILE, _: libc::c_int) -> libc::c_int;
    fn close_stream(stream: *mut FILE) -> libc::c_int;
    fn fd_reopen(
        _: libc::c_int,
        _: *const libc::c_char,
        _: libc::c_int,
        _: mode_t,
    ) -> libc::c_int;
    fn gethrxtime() -> xtime_t;
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
    fn parse_gnu_standard_options_only(
        argc: libc::c_int,
        argv: *mut *mut libc::c_char,
        command_name: *const libc::c_char,
        package: *const libc::c_char,
        version: *const libc::c_char,
        scan_all: bool,
        usage_func: Option::<unsafe extern "C" fn(libc::c_int) -> ()>,
        _: ...
    );
    fn quote_n(n: libc::c_int, arg: *const libc::c_char) -> *const libc::c_char;
    fn quote(arg: *const libc::c_char) -> *const libc::c_char;
    fn verror(
        __status: libc::c_int,
        __errnum: libc::c_int,
        __format: *const libc::c_char,
        __args: ::core::ffi::VaList,
    );
}
pub type __builtin_va_list = __va_list;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __va_list {
    pub __stack: *mut libc::c_void,
    pub __gr_top: *mut libc::c_void,
    pub __vr_top: *mut libc::c_void,
    pub __gr_offs: libc::c_int,
    pub __vr_offs: libc::c_int,
}
pub type __int32_t = libc::c_int;
pub type __uint32_t = libc::c_uint;
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
pub type __pid_t = libc::c_int;
pub type __clock_t = libc::c_long;
pub type __time_t = libc::c_long;
pub type __blksize_t = libc::c_int;
pub type __blkcnt_t = libc::c_long;
pub type __ssize_t = libc::c_long;
pub type __syscall_slong_t = libc::c_long;
pub type __sig_atomic_t = libc::c_int;
pub type mode_t = __mode_t;
pub type off_t = __off_t;
pub type ssize_t = __ssize_t;
pub type size_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __sigset_t {
    pub __val: [libc::c_ulong; 16],
}
pub type sigset_t = __sigset_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
pub type sig_atomic_t = __sig_atomic_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub union sigval {
    pub sival_int: libc::c_int,
    pub sival_ptr: *mut libc::c_void,
}
pub type __sigval_t = sigval;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct siginfo_t {
    pub si_signo: libc::c_int,
    pub si_errno: libc::c_int,
    pub si_code: libc::c_int,
    pub __pad0: libc::c_int,
    pub _sifields: C2RustUnnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub _pad: [libc::c_int; 28],
    pub _kill: C2RustUnnamed_8,
    pub _timer: C2RustUnnamed_7,
    pub _rt: C2RustUnnamed_6,
    pub _sigchld: C2RustUnnamed_5,
    pub _sigfault: C2RustUnnamed_2,
    pub _sigpoll: C2RustUnnamed_1,
    pub _sigsys: C2RustUnnamed_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_0 {
    pub _call_addr: *mut libc::c_void,
    pub _syscall: libc::c_int,
    pub _arch: libc::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_1 {
    pub si_band: libc::c_long,
    pub si_fd: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_2 {
    pub si_addr: *mut libc::c_void,
    pub si_addr_lsb: libc::c_short,
    pub _bounds: C2RustUnnamed_3,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_3 {
    pub _addr_bnd: C2RustUnnamed_4,
    pub _pkey: __uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_4 {
    pub _lower: *mut libc::c_void,
    pub _upper: *mut libc::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_5 {
    pub si_pid: __pid_t,
    pub si_uid: __uid_t,
    pub si_status: libc::c_int,
    pub si_utime: __clock_t,
    pub si_stime: __clock_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_6 {
    pub si_pid: __pid_t,
    pub si_uid: __uid_t,
    pub si_sigval: __sigval_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_7 {
    pub si_tid: libc::c_int,
    pub si_overrun: libc::c_int,
    pub si_sigval: __sigval_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_8 {
    pub si_pid: __pid_t,
    pub si_uid: __uid_t,
}
pub type __sighandler_t = Option::<unsafe extern "C" fn(libc::c_int) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sigaction {
    pub __sigaction_handler: C2RustUnnamed_9,
    pub sa_mask: __sigset_t,
    pub sa_flags: libc::c_int,
    pub sa_restorer: Option::<unsafe extern "C" fn() -> ()>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_9 {
    pub sa_handler: __sighandler_t,
    pub sa_sigaction: Option::<
        unsafe extern "C" fn(libc::c_int, *mut siginfo_t, *mut libc::c_void) -> (),
    >,
}
pub type ptrdiff_t = libc::c_long;
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
pub type intmax_t = __intmax_t;
pub type uintmax_t = __uintmax_t;
pub const PLURAL_REDUCER: C2RustUnnamed_10 = 1000000;
pub type C2RustUnnamed_10 = libc::c_uint;
pub type idx_t = ptrdiff_t;
pub type va_list = __builtin_va_list;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct infomap {
    pub program: *const libc::c_char,
    pub node: *const libc::c_char,
}
pub const O_NOCACHE: C2RustUnnamed_15 = 2;
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
pub type xtime_t = libc::c_longlong;
pub type strtol_error = libc::c_uint;
pub const LONGINT_INVALID: strtol_error = 4;
pub const LONGINT_INVALID_SUFFIX_CHAR_WITH_OVERFLOW: strtol_error = 3;
pub const LONGINT_INVALID_SUFFIX_CHAR: strtol_error = 2;
pub const LONGINT_OVERFLOW: strtol_error = 1;
pub const LONGINT_OK: strtol_error = 0;
pub type C2RustUnnamed_11 = libc::c_uint;
pub const human_B: C2RustUnnamed_11 = 256;
pub const human_SI: C2RustUnnamed_11 = 128;
pub const human_space_before_unit: C2RustUnnamed_11 = 64;
pub const human_base_1024: C2RustUnnamed_11 = 32;
pub const human_autoscale: C2RustUnnamed_11 = 16;
pub const human_suppress_point_zero: C2RustUnnamed_11 = 8;
pub const human_group_digits: C2RustUnnamed_11 = 4;
pub const human_floor: C2RustUnnamed_11 = 2;
pub const human_round_to_nearest: C2RustUnnamed_11 = 1;
pub const human_ceiling: C2RustUnnamed_11 = 0;
pub type C2RustUnnamed_12 = libc::c_uint;
pub const IO_BUFSIZE: C2RustUnnamed_12 = 131072;
pub type C2RustUnnamed_13 = libc::c_uint;
pub const C_SPARSE: C2RustUnnamed_13 = 65536;
pub const C_FSYNC: C2RustUnnamed_13 = 32768;
pub const C_FDATASYNC: C2RustUnnamed_13 = 16384;
pub const C_EXCL: C2RustUnnamed_13 = 8192;
pub const C_NOCREAT: C2RustUnnamed_13 = 4096;
pub const C_TWOBUFS: C2RustUnnamed_13 = 2048;
pub const C_SYNC: C2RustUnnamed_13 = 1024;
pub const C_NOTRUNC: C2RustUnnamed_13 = 512;
pub const C_NOERROR: C2RustUnnamed_13 = 256;
pub const C_SWAB: C2RustUnnamed_13 = 128;
pub const C_UCASE: C2RustUnnamed_13 = 64;
pub const C_LCASE: C2RustUnnamed_13 = 32;
pub const C_UNBLOCK: C2RustUnnamed_13 = 16;
pub const C_BLOCK: C2RustUnnamed_13 = 8;
pub const C_IBM: C2RustUnnamed_13 = 4;
pub const C_EBCDIC: C2RustUnnamed_13 = 2;
pub const C_ASCII: C2RustUnnamed_13 = 1;
pub type C2RustUnnamed_14 = libc::c_uint;
pub const STATUS_PROGRESS: C2RustUnnamed_14 = 4;
pub const STATUS_DEFAULT: C2RustUnnamed_14 = 3;
pub const STATUS_NOXFER: C2RustUnnamed_14 = 2;
pub const STATUS_NONE: C2RustUnnamed_14 = 1;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct symbol_value {
    pub symbol: [libc::c_char; 12],
    pub value: libc::c_int,
}
pub type C2RustUnnamed_15 = libc::c_int;
pub const O_SEEK_BYTES: C2RustUnnamed_15 = 16;
pub const v5: C2RustUnnamed_15 = -1432848;
pub const O_SKIP_BYTES: C2RustUnnamed_15 = 8;
pub const v4: C2RustUnnamed_15 = -1432840;
pub const O_COUNT_BYTES: C2RustUnnamed_15 = 4;
pub const v3: C2RustUnnamed_15 = -1432836;
pub const v2: C2RustUnnamed_15 = -1432834;
pub const O_FULLBLOCK: C2RustUnnamed_15 = 1;
pub const v: C2RustUnnamed_15 = -1432833;
pub type C2RustUnnamed_16 = libc::c_uint;
pub const human_opts: C2RustUnnamed_16 = 465;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_17 {
    pub _gl_dummy: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_18 {
    pub _gl_dummy: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_19 {
    pub _gl_dummy: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_20 {
    pub _gl_dummy: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_21 {
    pub _gl_dummy: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_22 {
    pub _gl_dummy: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_23 {
    pub _gl_dummy: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_24 {
    pub _gl_dummy: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_25 {
    pub _gl_dummy: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_26 {
    pub _gl_dummy: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_27 {
    pub _gl_dummy: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_28 {
    pub _gl_dummy: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_29 {
    pub _gl_dummy: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_30 {
    pub _gl_dummy: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_31 {
    pub _gl_dummy: libc::c_int,
}
#[inline]
unsafe extern "C" fn tolower(mut __c: libc::c_int) -> libc::c_int {
    return if __c >= -(128 as libc::c_int) && __c < 256 as libc::c_int {
        *(*__ctype_tolower_loc()).offset(__c as isize)
    } else {
        __c
    };
}
#[inline]
unsafe extern "C" fn toupper(mut __c: libc::c_int) -> libc::c_int {
    return if __c >= -(128 as libc::c_int) && __c < 256 as libc::c_int {
        *(*__ctype_toupper_loc()).offset(__c as isize)
    } else {
        __c
    };
}
#[inline]
unsafe extern "C" fn to_uchar(mut ch: libc::c_char) -> libc::c_uchar {
    return ch as libc::c_uchar;
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
unsafe extern "C" fn is_nul(mut buf: *const libc::c_void, mut length: size_t) -> bool {
    let mut p: *const libc::c_uchar = buf as *const libc::c_uchar;
    let mut word: libc::c_uchar = 0;
    if length == 0 {
        return 1 as libc::c_int != 0;
    }
    while (length
        & (::core::mem::size_of::<libc::c_uchar>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong)) as libc::c_long != 0
    {
        if *p != 0 {
            return 0 as libc::c_int != 0;
        }
        p = p.offset(1);
        length = length.wrapping_sub(1);
        if length == 0 {
            return 1 as libc::c_int != 0;
        }
    }
    loop {
        memcpy(
            &mut word as *mut libc::c_uchar as *mut libc::c_void,
            p as *const libc::c_void,
            ::core::mem::size_of::<libc::c_uchar>() as libc::c_ulong,
        );
        if word != 0 {
            return 0 as libc::c_int != 0;
        }
        p = p.offset(::core::mem::size_of::<libc::c_uchar>() as libc::c_ulong as isize);
        length = (length as libc::c_ulong)
            .wrapping_sub(::core::mem::size_of::<libc::c_uchar>() as libc::c_ulong)
            as size_t as size_t;
        if length == 0 {
            return 1 as libc::c_int != 0;
        }
        if (length & 15 as libc::c_int as libc::c_ulong) as libc::c_long
            == 0 as libc::c_int as libc::c_long
        {
            break;
        }
    }
    return memcmp(buf, p as *const libc::c_void, length) == 0 as libc::c_int;
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
unsafe extern "C" fn usable_st_size(mut sb: *const stat) -> bool {
    return (*sb).st_mode & 0o170000 as libc::c_int as libc::c_uint
        == 0o100000 as libc::c_int as libc::c_uint
        || (*sb).st_mode & 0o170000 as libc::c_int as libc::c_uint
            == 0o120000 as libc::c_int as libc::c_uint
        || ((*sb).st_mode).wrapping_sub((*sb).st_mode) != 0 || 0 as libc::c_int != 0;
}
#[inline]
unsafe extern "C" fn alignalloc(
    mut alignment: idx_t,
    mut size: idx_t,
) -> *mut libc::c_void {
    if (-(1 as libc::c_int) as size_t) < alignment as libc::c_ulong {
        alignment = -(1 as libc::c_int) as size_t as idx_t;
    }
    if (-(1 as libc::c_int) as size_t) < size as libc::c_ulong {
        size = -(1 as libc::c_int) as size_t as idx_t;
    }
    return aligned_alloc(alignment as libc::c_ulong, size as libc::c_ulong);
}
static mut input_file: *const libc::c_char = 0 as *const libc::c_char;
static mut output_file: *const libc::c_char = 0 as *const libc::c_char;
static mut page_size: idx_t = 0;
static mut input_blocksize: idx_t = 0 as libc::c_int as idx_t;
static mut output_blocksize: idx_t = 0 as libc::c_int as idx_t;
static mut conversion_blocksize: idx_t = 0 as libc::c_int as idx_t;
static mut skip_records: intmax_t = 0 as libc::c_int as intmax_t;
static mut skip_bytes: idx_t = 0 as libc::c_int as idx_t;
static mut seek_records: intmax_t = 0 as libc::c_int as intmax_t;
static mut seek_bytes: intmax_t = 0 as libc::c_int as intmax_t;
static mut final_op_was_seek: bool = false;
static mut max_records: intmax_t = 9223372036854775807 as libc::c_long;
static mut max_bytes: idx_t = 0 as libc::c_int as idx_t;
static mut conversions_mask: libc::c_int = 0 as libc::c_int;
static mut input_flags: libc::c_int = 0 as libc::c_int;
static mut output_flags: libc::c_int = 0 as libc::c_int;
static mut status_level: libc::c_int = STATUS_DEFAULT as libc::c_int;
static mut translation_needed: bool = 0 as libc::c_int != 0;
static mut w_partial: intmax_t = 0 as libc::c_int as intmax_t;
static mut w_full: intmax_t = 0 as libc::c_int as intmax_t;
static mut r_partial: intmax_t = 0 as libc::c_int as intmax_t;
static mut r_full: intmax_t = 0 as libc::c_int as intmax_t;
static mut w_bytes: intmax_t = 0 as libc::c_int as intmax_t;
static mut reported_w_bytes: intmax_t = -(1 as libc::c_int) as intmax_t;
static mut start_time: xtime_t = 0;
static mut next_time: xtime_t = 0;
static mut progress_len: libc::c_int = 0;
static mut input_seekable: bool = false;
static mut input_seek_errno: libc::c_int = 0;
static mut input_offset: off_t = 0;
static mut warn_partial_read: bool = false;
static mut r_truncate: intmax_t = 0 as libc::c_int as intmax_t;
static mut newline_character: libc::c_char = '\n' as i32 as libc::c_char;
static mut space_character: libc::c_char = ' ' as i32 as libc::c_char;
static mut ibuf: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
static mut obuf: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
static mut oc: idx_t = 0 as libc::c_int as idx_t;
static mut col: idx_t = 0 as libc::c_int as idx_t;
static mut caught_signals: sigset_t = sigset_t { __val: [0; 16] };
static mut interrupt_signal: sig_atomic_t = 0;
static mut info_signal_count: sig_atomic_t = 0;
static mut i_nocache: bool = false;
static mut o_nocache: bool = false;
static mut i_nocache_eof: bool = false;
static mut o_nocache_eof: bool = false;
static mut iread_fnc: Option::<
    unsafe extern "C" fn(libc::c_int, *mut libc::c_char, idx_t) -> ssize_t,
> = None;
static mut conversions: [symbol_value; 17] = unsafe {
    [
        {
            let mut init = symbol_value {
                symbol: *::core::mem::transmute::<
                    &[u8; 12],
                    &mut [libc::c_char; 12],
                >(b"ascii\0\0\0\0\0\0\0"),
                value: C_ASCII as libc::c_int | C_UNBLOCK as libc::c_int
                    | C_TWOBUFS as libc::c_int,
            };
            init
        },
        {
            let mut init = symbol_value {
                symbol: *::core::mem::transmute::<
                    &[u8; 12],
                    &mut [libc::c_char; 12],
                >(b"ebcdic\0\0\0\0\0\0"),
                value: C_EBCDIC as libc::c_int | C_BLOCK as libc::c_int
                    | C_TWOBUFS as libc::c_int,
            };
            init
        },
        {
            let mut init = symbol_value {
                symbol: *::core::mem::transmute::<
                    &[u8; 12],
                    &mut [libc::c_char; 12],
                >(b"ibm\0\0\0\0\0\0\0\0\0"),
                value: C_IBM as libc::c_int | C_BLOCK as libc::c_int
                    | C_TWOBUFS as libc::c_int,
            };
            init
        },
        {
            let mut init = symbol_value {
                symbol: *::core::mem::transmute::<
                    &[u8; 12],
                    &mut [libc::c_char; 12],
                >(b"block\0\0\0\0\0\0\0"),
                value: C_BLOCK as libc::c_int | C_TWOBUFS as libc::c_int,
            };
            init
        },
        {
            let mut init = symbol_value {
                symbol: *::core::mem::transmute::<
                    &[u8; 12],
                    &mut [libc::c_char; 12],
                >(b"unblock\0\0\0\0\0"),
                value: C_UNBLOCK as libc::c_int | C_TWOBUFS as libc::c_int,
            };
            init
        },
        {
            let mut init = symbol_value {
                symbol: *::core::mem::transmute::<
                    &[u8; 12],
                    &mut [libc::c_char; 12],
                >(b"lcase\0\0\0\0\0\0\0"),
                value: C_LCASE as libc::c_int | C_TWOBUFS as libc::c_int,
            };
            init
        },
        {
            let mut init = symbol_value {
                symbol: *::core::mem::transmute::<
                    &[u8; 12],
                    &mut [libc::c_char; 12],
                >(b"ucase\0\0\0\0\0\0\0"),
                value: C_UCASE as libc::c_int | C_TWOBUFS as libc::c_int,
            };
            init
        },
        {
            let mut init = symbol_value {
                symbol: *::core::mem::transmute::<
                    &[u8; 12],
                    &mut [libc::c_char; 12],
                >(b"sparse\0\0\0\0\0\0"),
                value: C_SPARSE as libc::c_int,
            };
            init
        },
        {
            let mut init = symbol_value {
                symbol: *::core::mem::transmute::<
                    &[u8; 12],
                    &mut [libc::c_char; 12],
                >(b"swab\0\0\0\0\0\0\0\0"),
                value: C_SWAB as libc::c_int | C_TWOBUFS as libc::c_int,
            };
            init
        },
        {
            let mut init = symbol_value {
                symbol: *::core::mem::transmute::<
                    &[u8; 12],
                    &mut [libc::c_char; 12],
                >(b"noerror\0\0\0\0\0"),
                value: C_NOERROR as libc::c_int,
            };
            init
        },
        {
            let mut init = symbol_value {
                symbol: *::core::mem::transmute::<
                    &[u8; 12],
                    &mut [libc::c_char; 12],
                >(b"nocreat\0\0\0\0\0"),
                value: C_NOCREAT as libc::c_int,
            };
            init
        },
        {
            let mut init = symbol_value {
                symbol: *::core::mem::transmute::<
                    &[u8; 12],
                    &mut [libc::c_char; 12],
                >(b"excl\0\0\0\0\0\0\0\0"),
                value: C_EXCL as libc::c_int,
            };
            init
        },
        {
            let mut init = symbol_value {
                symbol: *::core::mem::transmute::<
                    &[u8; 12],
                    &mut [libc::c_char; 12],
                >(b"notrunc\0\0\0\0\0"),
                value: C_NOTRUNC as libc::c_int,
            };
            init
        },
        {
            let mut init = symbol_value {
                symbol: *::core::mem::transmute::<
                    &[u8; 12],
                    &mut [libc::c_char; 12],
                >(b"sync\0\0\0\0\0\0\0\0"),
                value: C_SYNC as libc::c_int,
            };
            init
        },
        {
            let mut init = symbol_value {
                symbol: *::core::mem::transmute::<
                    &[u8; 12],
                    &mut [libc::c_char; 12],
                >(b"fdatasync\0\0\0"),
                value: C_FDATASYNC as libc::c_int,
            };
            init
        },
        {
            let mut init = symbol_value {
                symbol: *::core::mem::transmute::<
                    &[u8; 12],
                    &mut [libc::c_char; 12],
                >(b"fsync\0\0\0\0\0\0\0"),
                value: C_FSYNC as libc::c_int,
            };
            init
        },
        {
            let mut init = symbol_value {
                symbol: *::core::mem::transmute::<
                    &[u8; 12],
                    &mut [libc::c_char; 12],
                >(b"\0\0\0\0\0\0\0\0\0\0\0\0"),
                value: 0 as libc::c_int,
            };
            init
        },
    ]
};
static mut flags: [symbol_value; 19] = [symbol_value {
    symbol: [0; 12],
    value: 0,
}; 19];
static mut statuses: [symbol_value; 4] = unsafe {
    [
        {
            let mut init = symbol_value {
                symbol: *::core::mem::transmute::<
                    &[u8; 12],
                    &mut [libc::c_char; 12],
                >(b"none\0\0\0\0\0\0\0\0"),
                value: STATUS_NONE as libc::c_int,
            };
            init
        },
        {
            let mut init = symbol_value {
                symbol: *::core::mem::transmute::<
                    &[u8; 12],
                    &mut [libc::c_char; 12],
                >(b"noxfer\0\0\0\0\0\0"),
                value: STATUS_NOXFER as libc::c_int,
            };
            init
        },
        {
            let mut init = symbol_value {
                symbol: *::core::mem::transmute::<
                    &[u8; 12],
                    &mut [libc::c_char; 12],
                >(b"progress\0\0\0\0"),
                value: STATUS_PROGRESS as libc::c_int,
            };
            init
        },
        {
            let mut init = symbol_value {
                symbol: *::core::mem::transmute::<
                    &[u8; 12],
                    &mut [libc::c_char; 12],
                >(b"\0\0\0\0\0\0\0\0\0\0\0\0"),
                value: 0 as libc::c_int,
            };
            init
        },
    ]
};
static mut trans_table: [libc::c_uchar; 256] = [0; 256];
static mut ascii_to_ebcdic: [libc::c_char; 256] = [
    '\0' as i32 as libc::c_char,
    '\u{1}' as i32 as libc::c_char,
    '\u{2}' as i32 as libc::c_char,
    '\u{3}' as i32 as libc::c_char,
    '7' as i32 as libc::c_char,
    '-' as i32 as libc::c_char,
    '.' as i32 as libc::c_char,
    '/' as i32 as libc::c_char,
    '\u{16}' as i32 as libc::c_char,
    '\u{5}' as i32 as libc::c_char,
    '%' as i32 as libc::c_char,
    '\u{b}' as i32 as libc::c_char,
    '\u{c}' as i32 as libc::c_char,
    '\r' as i32 as libc::c_char,
    '\u{e}' as i32 as libc::c_char,
    '\u{f}' as i32 as libc::c_char,
    '\u{10}' as i32 as libc::c_char,
    '\u{11}' as i32 as libc::c_char,
    '\u{12}' as i32 as libc::c_char,
    '\u{13}' as i32 as libc::c_char,
    '<' as i32 as libc::c_char,
    '=' as i32 as libc::c_char,
    '2' as i32 as libc::c_char,
    '&' as i32 as libc::c_char,
    '\u{18}' as i32 as libc::c_char,
    '\u{19}' as i32 as libc::c_char,
    '?' as i32 as libc::c_char,
    '\'' as i32 as libc::c_char,
    '\u{1c}' as i32 as libc::c_char,
    '\u{1d}' as i32 as libc::c_char,
    '\u{1e}' as i32 as libc::c_char,
    '\u{1f}' as i32 as libc::c_char,
    '@' as i32 as libc::c_char,
    'Z' as i32 as libc::c_char,
    '\u{7f}' as i32 as libc::c_char,
    '{' as i32 as libc::c_char,
    '[' as i32 as libc::c_char,
    'l' as i32 as libc::c_char,
    'P' as i32 as libc::c_char,
    '}' as i32 as libc::c_char,
    'M' as i32 as libc::c_char,
    ']' as i32 as libc::c_char,
    '\\' as i32 as libc::c_char,
    'N' as i32 as libc::c_char,
    'k' as i32 as libc::c_char,
    '`' as i32 as libc::c_char,
    'K' as i32 as libc::c_char,
    'a' as i32 as libc::c_char,
    'ð' as i32 as libc::c_char,
    'ñ' as i32 as libc::c_char,
    'ò' as i32 as libc::c_char,
    'ó' as i32 as libc::c_char,
    'ô' as i32 as libc::c_char,
    'õ' as i32 as libc::c_char,
    'ö' as i32 as libc::c_char,
    '÷' as i32 as libc::c_char,
    'ø' as i32 as libc::c_char,
    'ù' as i32 as libc::c_char,
    'z' as i32 as libc::c_char,
    '^' as i32 as libc::c_char,
    'L' as i32 as libc::c_char,
    '~' as i32 as libc::c_char,
    'n' as i32 as libc::c_char,
    'o' as i32 as libc::c_char,
    '|' as i32 as libc::c_char,
    'Á' as i32 as libc::c_char,
    'Â' as i32 as libc::c_char,
    'Ã' as i32 as libc::c_char,
    'Ä' as i32 as libc::c_char,
    'Å' as i32 as libc::c_char,
    'Æ' as i32 as libc::c_char,
    'Ç' as i32 as libc::c_char,
    'È' as i32 as libc::c_char,
    'É' as i32 as libc::c_char,
    'Ñ' as i32 as libc::c_char,
    'Ò' as i32 as libc::c_char,
    'Ó' as i32 as libc::c_char,
    'Ô' as i32 as libc::c_char,
    'Õ' as i32 as libc::c_char,
    'Ö' as i32 as libc::c_char,
    '×' as i32 as libc::c_char,
    'Ø' as i32 as libc::c_char,
    'Ù' as i32 as libc::c_char,
    'â' as i32 as libc::c_char,
    'ã' as i32 as libc::c_char,
    'ä' as i32 as libc::c_char,
    'å' as i32 as libc::c_char,
    'æ' as i32 as libc::c_char,
    'ç' as i32 as libc::c_char,
    'è' as i32 as libc::c_char,
    'é' as i32 as libc::c_char,
    '\u{ad}' as i32 as libc::c_char,
    'à' as i32 as libc::c_char,
    '½' as i32 as libc::c_char,
    '\u{9a}' as i32 as libc::c_char,
    'm' as i32 as libc::c_char,
    'y' as i32 as libc::c_char,
    '\u{81}' as i32 as libc::c_char,
    '\u{82}' as i32 as libc::c_char,
    '\u{83}' as i32 as libc::c_char,
    '\u{84}' as i32 as libc::c_char,
    '\u{85}' as i32 as libc::c_char,
    '\u{86}' as i32 as libc::c_char,
    '\u{87}' as i32 as libc::c_char,
    '\u{88}' as i32 as libc::c_char,
    '\u{89}' as i32 as libc::c_char,
    '\u{91}' as i32 as libc::c_char,
    '\u{92}' as i32 as libc::c_char,
    '\u{93}' as i32 as libc::c_char,
    '\u{94}' as i32 as libc::c_char,
    '\u{95}' as i32 as libc::c_char,
    '\u{96}' as i32 as libc::c_char,
    '\u{97}' as i32 as libc::c_char,
    '\u{98}' as i32 as libc::c_char,
    '\u{99}' as i32 as libc::c_char,
    '¢' as i32 as libc::c_char,
    '£' as i32 as libc::c_char,
    '¤' as i32 as libc::c_char,
    '¥' as i32 as libc::c_char,
    '¦' as i32 as libc::c_char,
    '§' as i32 as libc::c_char,
    '¨' as i32 as libc::c_char,
    '©' as i32 as libc::c_char,
    'À' as i32 as libc::c_char,
    'O' as i32 as libc::c_char,
    'Ð' as i32 as libc::c_char,
    '_' as i32 as libc::c_char,
    '\u{7}' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    '!' as i32 as libc::c_char,
    '"' as i32 as libc::c_char,
    '#' as i32 as libc::c_char,
    '$' as i32 as libc::c_char,
    '\u{15}' as i32 as libc::c_char,
    '\u{6}' as i32 as libc::c_char,
    '\u{17}' as i32 as libc::c_char,
    '(' as i32 as libc::c_char,
    ')' as i32 as libc::c_char,
    '*' as i32 as libc::c_char,
    '+' as i32 as libc::c_char,
    ',' as i32 as libc::c_char,
    '\t' as i32 as libc::c_char,
    '\n' as i32 as libc::c_char,
    '\u{1b}' as i32 as libc::c_char,
    '0' as i32 as libc::c_char,
    '1' as i32 as libc::c_char,
    '\u{1a}' as i32 as libc::c_char,
    '3' as i32 as libc::c_char,
    '4' as i32 as libc::c_char,
    '5' as i32 as libc::c_char,
    '6' as i32 as libc::c_char,
    '\u{8}' as i32 as libc::c_char,
    '8' as i32 as libc::c_char,
    '9' as i32 as libc::c_char,
    ':' as i32 as libc::c_char,
    ';' as i32 as libc::c_char,
    '\u{4}' as i32 as libc::c_char,
    '\u{14}' as i32 as libc::c_char,
    '>' as i32 as libc::c_char,
    'á' as i32 as libc::c_char,
    'A' as i32 as libc::c_char,
    'B' as i32 as libc::c_char,
    'C' as i32 as libc::c_char,
    'D' as i32 as libc::c_char,
    'E' as i32 as libc::c_char,
    'F' as i32 as libc::c_char,
    'G' as i32 as libc::c_char,
    'H' as i32 as libc::c_char,
    'I' as i32 as libc::c_char,
    'Q' as i32 as libc::c_char,
    'R' as i32 as libc::c_char,
    'S' as i32 as libc::c_char,
    'T' as i32 as libc::c_char,
    'U' as i32 as libc::c_char,
    'V' as i32 as libc::c_char,
    'W' as i32 as libc::c_char,
    'X' as i32 as libc::c_char,
    'Y' as i32 as libc::c_char,
    'b' as i32 as libc::c_char,
    'c' as i32 as libc::c_char,
    'd' as i32 as libc::c_char,
    'e' as i32 as libc::c_char,
    'f' as i32 as libc::c_char,
    'g' as i32 as libc::c_char,
    'h' as i32 as libc::c_char,
    'i' as i32 as libc::c_char,
    'p' as i32 as libc::c_char,
    'q' as i32 as libc::c_char,
    'r' as i32 as libc::c_char,
    's' as i32 as libc::c_char,
    't' as i32 as libc::c_char,
    'u' as i32 as libc::c_char,
    'v' as i32 as libc::c_char,
    'w' as i32 as libc::c_char,
    'x' as i32 as libc::c_char,
    '\u{80}' as i32 as libc::c_char,
    '\u{8a}' as i32 as libc::c_char,
    '\u{8b}' as i32 as libc::c_char,
    '\u{8c}' as i32 as libc::c_char,
    '\u{8d}' as i32 as libc::c_char,
    '\u{8e}' as i32 as libc::c_char,
    '\u{8f}' as i32 as libc::c_char,
    '\u{90}' as i32 as libc::c_char,
    'j' as i32 as libc::c_char,
    '\u{9b}' as i32 as libc::c_char,
    '\u{9c}' as i32 as libc::c_char,
    '\u{9d}' as i32 as libc::c_char,
    '\u{9e}' as i32 as libc::c_char,
    '\u{9f}' as i32 as libc::c_char,
    '\u{a0}' as i32 as libc::c_char,
    'ª' as i32 as libc::c_char,
    '«' as i32 as libc::c_char,
    '¬' as i32 as libc::c_char,
    'J' as i32 as libc::c_char,
    '®' as i32 as libc::c_char,
    '¯' as i32 as libc::c_char,
    '°' as i32 as libc::c_char,
    '±' as i32 as libc::c_char,
    '²' as i32 as libc::c_char,
    '³' as i32 as libc::c_char,
    '´' as i32 as libc::c_char,
    'µ' as i32 as libc::c_char,
    '¶' as i32 as libc::c_char,
    '·' as i32 as libc::c_char,
    '¸' as i32 as libc::c_char,
    '¹' as i32 as libc::c_char,
    'º' as i32 as libc::c_char,
    '»' as i32 as libc::c_char,
    '¼' as i32 as libc::c_char,
    '¡' as i32 as libc::c_char,
    '¾' as i32 as libc::c_char,
    '¿' as i32 as libc::c_char,
    'Ê' as i32 as libc::c_char,
    'Ë' as i32 as libc::c_char,
    'Ì' as i32 as libc::c_char,
    'Í' as i32 as libc::c_char,
    'Î' as i32 as libc::c_char,
    'Ï' as i32 as libc::c_char,
    'Ú' as i32 as libc::c_char,
    'Û' as i32 as libc::c_char,
    'Ü' as i32 as libc::c_char,
    'Ý' as i32 as libc::c_char,
    'Þ' as i32 as libc::c_char,
    'ß' as i32 as libc::c_char,
    'ê' as i32 as libc::c_char,
    'ë' as i32 as libc::c_char,
    'ì' as i32 as libc::c_char,
    'í' as i32 as libc::c_char,
    'î' as i32 as libc::c_char,
    'ï' as i32 as libc::c_char,
    'ú' as i32 as libc::c_char,
    'û' as i32 as libc::c_char,
    'ü' as i32 as libc::c_char,
    'ý' as i32 as libc::c_char,
    'þ' as i32 as libc::c_char,
    'ÿ' as i32 as libc::c_char,
];
static mut ascii_to_ibm: [libc::c_char; 256] = [
    '\0' as i32 as libc::c_char,
    '\u{1}' as i32 as libc::c_char,
    '\u{2}' as i32 as libc::c_char,
    '\u{3}' as i32 as libc::c_char,
    '7' as i32 as libc::c_char,
    '-' as i32 as libc::c_char,
    '.' as i32 as libc::c_char,
    '/' as i32 as libc::c_char,
    '\u{16}' as i32 as libc::c_char,
    '\u{5}' as i32 as libc::c_char,
    '%' as i32 as libc::c_char,
    '\u{b}' as i32 as libc::c_char,
    '\u{c}' as i32 as libc::c_char,
    '\r' as i32 as libc::c_char,
    '\u{e}' as i32 as libc::c_char,
    '\u{f}' as i32 as libc::c_char,
    '\u{10}' as i32 as libc::c_char,
    '\u{11}' as i32 as libc::c_char,
    '\u{12}' as i32 as libc::c_char,
    '\u{13}' as i32 as libc::c_char,
    '<' as i32 as libc::c_char,
    '=' as i32 as libc::c_char,
    '2' as i32 as libc::c_char,
    '&' as i32 as libc::c_char,
    '\u{18}' as i32 as libc::c_char,
    '\u{19}' as i32 as libc::c_char,
    '?' as i32 as libc::c_char,
    '\'' as i32 as libc::c_char,
    '\u{1c}' as i32 as libc::c_char,
    '\u{1d}' as i32 as libc::c_char,
    '\u{1e}' as i32 as libc::c_char,
    '\u{1f}' as i32 as libc::c_char,
    '@' as i32 as libc::c_char,
    'Z' as i32 as libc::c_char,
    '\u{7f}' as i32 as libc::c_char,
    '{' as i32 as libc::c_char,
    '[' as i32 as libc::c_char,
    'l' as i32 as libc::c_char,
    'P' as i32 as libc::c_char,
    '}' as i32 as libc::c_char,
    'M' as i32 as libc::c_char,
    ']' as i32 as libc::c_char,
    '\\' as i32 as libc::c_char,
    'N' as i32 as libc::c_char,
    'k' as i32 as libc::c_char,
    '`' as i32 as libc::c_char,
    'K' as i32 as libc::c_char,
    'a' as i32 as libc::c_char,
    'ð' as i32 as libc::c_char,
    'ñ' as i32 as libc::c_char,
    'ò' as i32 as libc::c_char,
    'ó' as i32 as libc::c_char,
    'ô' as i32 as libc::c_char,
    'õ' as i32 as libc::c_char,
    'ö' as i32 as libc::c_char,
    '÷' as i32 as libc::c_char,
    'ø' as i32 as libc::c_char,
    'ù' as i32 as libc::c_char,
    'z' as i32 as libc::c_char,
    '^' as i32 as libc::c_char,
    'L' as i32 as libc::c_char,
    '~' as i32 as libc::c_char,
    'n' as i32 as libc::c_char,
    'o' as i32 as libc::c_char,
    '|' as i32 as libc::c_char,
    'Á' as i32 as libc::c_char,
    'Â' as i32 as libc::c_char,
    'Ã' as i32 as libc::c_char,
    'Ä' as i32 as libc::c_char,
    'Å' as i32 as libc::c_char,
    'Æ' as i32 as libc::c_char,
    'Ç' as i32 as libc::c_char,
    'È' as i32 as libc::c_char,
    'É' as i32 as libc::c_char,
    'Ñ' as i32 as libc::c_char,
    'Ò' as i32 as libc::c_char,
    'Ó' as i32 as libc::c_char,
    'Ô' as i32 as libc::c_char,
    'Õ' as i32 as libc::c_char,
    'Ö' as i32 as libc::c_char,
    '×' as i32 as libc::c_char,
    'Ø' as i32 as libc::c_char,
    'Ù' as i32 as libc::c_char,
    'â' as i32 as libc::c_char,
    'ã' as i32 as libc::c_char,
    'ä' as i32 as libc::c_char,
    'å' as i32 as libc::c_char,
    'æ' as i32 as libc::c_char,
    'ç' as i32 as libc::c_char,
    'è' as i32 as libc::c_char,
    'é' as i32 as libc::c_char,
    '\u{ad}' as i32 as libc::c_char,
    'à' as i32 as libc::c_char,
    '½' as i32 as libc::c_char,
    '_' as i32 as libc::c_char,
    'm' as i32 as libc::c_char,
    'y' as i32 as libc::c_char,
    '\u{81}' as i32 as libc::c_char,
    '\u{82}' as i32 as libc::c_char,
    '\u{83}' as i32 as libc::c_char,
    '\u{84}' as i32 as libc::c_char,
    '\u{85}' as i32 as libc::c_char,
    '\u{86}' as i32 as libc::c_char,
    '\u{87}' as i32 as libc::c_char,
    '\u{88}' as i32 as libc::c_char,
    '\u{89}' as i32 as libc::c_char,
    '\u{91}' as i32 as libc::c_char,
    '\u{92}' as i32 as libc::c_char,
    '\u{93}' as i32 as libc::c_char,
    '\u{94}' as i32 as libc::c_char,
    '\u{95}' as i32 as libc::c_char,
    '\u{96}' as i32 as libc::c_char,
    '\u{97}' as i32 as libc::c_char,
    '\u{98}' as i32 as libc::c_char,
    '\u{99}' as i32 as libc::c_char,
    '¢' as i32 as libc::c_char,
    '£' as i32 as libc::c_char,
    '¤' as i32 as libc::c_char,
    '¥' as i32 as libc::c_char,
    '¦' as i32 as libc::c_char,
    '§' as i32 as libc::c_char,
    '¨' as i32 as libc::c_char,
    '©' as i32 as libc::c_char,
    'À' as i32 as libc::c_char,
    'O' as i32 as libc::c_char,
    'Ð' as i32 as libc::c_char,
    '¡' as i32 as libc::c_char,
    '\u{7}' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    '!' as i32 as libc::c_char,
    '"' as i32 as libc::c_char,
    '#' as i32 as libc::c_char,
    '$' as i32 as libc::c_char,
    '\u{15}' as i32 as libc::c_char,
    '\u{6}' as i32 as libc::c_char,
    '\u{17}' as i32 as libc::c_char,
    '(' as i32 as libc::c_char,
    ')' as i32 as libc::c_char,
    '*' as i32 as libc::c_char,
    '+' as i32 as libc::c_char,
    ',' as i32 as libc::c_char,
    '\t' as i32 as libc::c_char,
    '\n' as i32 as libc::c_char,
    '\u{1b}' as i32 as libc::c_char,
    '0' as i32 as libc::c_char,
    '1' as i32 as libc::c_char,
    '\u{1a}' as i32 as libc::c_char,
    '3' as i32 as libc::c_char,
    '4' as i32 as libc::c_char,
    '5' as i32 as libc::c_char,
    '6' as i32 as libc::c_char,
    '\u{8}' as i32 as libc::c_char,
    '8' as i32 as libc::c_char,
    '9' as i32 as libc::c_char,
    ':' as i32 as libc::c_char,
    ';' as i32 as libc::c_char,
    '\u{4}' as i32 as libc::c_char,
    '\u{14}' as i32 as libc::c_char,
    '>' as i32 as libc::c_char,
    'á' as i32 as libc::c_char,
    'A' as i32 as libc::c_char,
    'B' as i32 as libc::c_char,
    'C' as i32 as libc::c_char,
    'D' as i32 as libc::c_char,
    'E' as i32 as libc::c_char,
    'F' as i32 as libc::c_char,
    'G' as i32 as libc::c_char,
    'H' as i32 as libc::c_char,
    'I' as i32 as libc::c_char,
    'Q' as i32 as libc::c_char,
    'R' as i32 as libc::c_char,
    'S' as i32 as libc::c_char,
    'T' as i32 as libc::c_char,
    'U' as i32 as libc::c_char,
    'V' as i32 as libc::c_char,
    'W' as i32 as libc::c_char,
    'X' as i32 as libc::c_char,
    'Y' as i32 as libc::c_char,
    'b' as i32 as libc::c_char,
    'c' as i32 as libc::c_char,
    'd' as i32 as libc::c_char,
    'e' as i32 as libc::c_char,
    'f' as i32 as libc::c_char,
    'g' as i32 as libc::c_char,
    'h' as i32 as libc::c_char,
    'i' as i32 as libc::c_char,
    'p' as i32 as libc::c_char,
    'q' as i32 as libc::c_char,
    'r' as i32 as libc::c_char,
    's' as i32 as libc::c_char,
    't' as i32 as libc::c_char,
    'u' as i32 as libc::c_char,
    'v' as i32 as libc::c_char,
    'w' as i32 as libc::c_char,
    'x' as i32 as libc::c_char,
    '\u{80}' as i32 as libc::c_char,
    '\u{8a}' as i32 as libc::c_char,
    '\u{8b}' as i32 as libc::c_char,
    '\u{8c}' as i32 as libc::c_char,
    '\u{8d}' as i32 as libc::c_char,
    '\u{8e}' as i32 as libc::c_char,
    '\u{8f}' as i32 as libc::c_char,
    '\u{90}' as i32 as libc::c_char,
    '\u{9a}' as i32 as libc::c_char,
    '\u{9b}' as i32 as libc::c_char,
    '\u{9c}' as i32 as libc::c_char,
    '\u{9d}' as i32 as libc::c_char,
    '\u{9e}' as i32 as libc::c_char,
    '\u{9f}' as i32 as libc::c_char,
    '\u{a0}' as i32 as libc::c_char,
    'ª' as i32 as libc::c_char,
    '«' as i32 as libc::c_char,
    '¬' as i32 as libc::c_char,
    '\u{ad}' as i32 as libc::c_char,
    '®' as i32 as libc::c_char,
    '¯' as i32 as libc::c_char,
    '°' as i32 as libc::c_char,
    '±' as i32 as libc::c_char,
    '²' as i32 as libc::c_char,
    '³' as i32 as libc::c_char,
    '´' as i32 as libc::c_char,
    'µ' as i32 as libc::c_char,
    '¶' as i32 as libc::c_char,
    '·' as i32 as libc::c_char,
    '¸' as i32 as libc::c_char,
    '¹' as i32 as libc::c_char,
    'º' as i32 as libc::c_char,
    '»' as i32 as libc::c_char,
    '¼' as i32 as libc::c_char,
    '½' as i32 as libc::c_char,
    '¾' as i32 as libc::c_char,
    '¿' as i32 as libc::c_char,
    'Ê' as i32 as libc::c_char,
    'Ë' as i32 as libc::c_char,
    'Ì' as i32 as libc::c_char,
    'Í' as i32 as libc::c_char,
    'Î' as i32 as libc::c_char,
    'Ï' as i32 as libc::c_char,
    'Ú' as i32 as libc::c_char,
    'Û' as i32 as libc::c_char,
    'Ü' as i32 as libc::c_char,
    'Ý' as i32 as libc::c_char,
    'Þ' as i32 as libc::c_char,
    'ß' as i32 as libc::c_char,
    'ê' as i32 as libc::c_char,
    'ë' as i32 as libc::c_char,
    'ì' as i32 as libc::c_char,
    'í' as i32 as libc::c_char,
    'î' as i32 as libc::c_char,
    'ï' as i32 as libc::c_char,
    'ú' as i32 as libc::c_char,
    'û' as i32 as libc::c_char,
    'ü' as i32 as libc::c_char,
    'ý' as i32 as libc::c_char,
    'þ' as i32 as libc::c_char,
    'ÿ' as i32 as libc::c_char,
];
static mut ebcdic_to_ascii: [libc::c_char; 256] = [
    '\0' as i32 as libc::c_char,
    '\u{1}' as i32 as libc::c_char,
    '\u{2}' as i32 as libc::c_char,
    '\u{3}' as i32 as libc::c_char,
    '\u{9c}' as i32 as libc::c_char,
    '\t' as i32 as libc::c_char,
    '\u{86}' as i32 as libc::c_char,
    '\u{7f}' as i32 as libc::c_char,
    '\u{97}' as i32 as libc::c_char,
    '\u{8d}' as i32 as libc::c_char,
    '\u{8e}' as i32 as libc::c_char,
    '\u{b}' as i32 as libc::c_char,
    '\u{c}' as i32 as libc::c_char,
    '\r' as i32 as libc::c_char,
    '\u{e}' as i32 as libc::c_char,
    '\u{f}' as i32 as libc::c_char,
    '\u{10}' as i32 as libc::c_char,
    '\u{11}' as i32 as libc::c_char,
    '\u{12}' as i32 as libc::c_char,
    '\u{13}' as i32 as libc::c_char,
    '\u{9d}' as i32 as libc::c_char,
    '\u{85}' as i32 as libc::c_char,
    '\u{8}' as i32 as libc::c_char,
    '\u{87}' as i32 as libc::c_char,
    '\u{18}' as i32 as libc::c_char,
    '\u{19}' as i32 as libc::c_char,
    '\u{92}' as i32 as libc::c_char,
    '\u{8f}' as i32 as libc::c_char,
    '\u{1c}' as i32 as libc::c_char,
    '\u{1d}' as i32 as libc::c_char,
    '\u{1e}' as i32 as libc::c_char,
    '\u{1f}' as i32 as libc::c_char,
    '\u{80}' as i32 as libc::c_char,
    '\u{81}' as i32 as libc::c_char,
    '\u{82}' as i32 as libc::c_char,
    '\u{83}' as i32 as libc::c_char,
    '\u{84}' as i32 as libc::c_char,
    '\n' as i32 as libc::c_char,
    '\u{17}' as i32 as libc::c_char,
    '\u{1b}' as i32 as libc::c_char,
    '\u{88}' as i32 as libc::c_char,
    '\u{89}' as i32 as libc::c_char,
    '\u{8a}' as i32 as libc::c_char,
    '\u{8b}' as i32 as libc::c_char,
    '\u{8c}' as i32 as libc::c_char,
    '\u{5}' as i32 as libc::c_char,
    '\u{6}' as i32 as libc::c_char,
    '\u{7}' as i32 as libc::c_char,
    '\u{90}' as i32 as libc::c_char,
    '\u{91}' as i32 as libc::c_char,
    '\u{16}' as i32 as libc::c_char,
    '\u{93}' as i32 as libc::c_char,
    '\u{94}' as i32 as libc::c_char,
    '\u{95}' as i32 as libc::c_char,
    '\u{96}' as i32 as libc::c_char,
    '\u{4}' as i32 as libc::c_char,
    '\u{98}' as i32 as libc::c_char,
    '\u{99}' as i32 as libc::c_char,
    '\u{9a}' as i32 as libc::c_char,
    '\u{9b}' as i32 as libc::c_char,
    '\u{14}' as i32 as libc::c_char,
    '\u{15}' as i32 as libc::c_char,
    '\u{9e}' as i32 as libc::c_char,
    '\u{1a}' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    '\u{a0}' as i32 as libc::c_char,
    '¡' as i32 as libc::c_char,
    '¢' as i32 as libc::c_char,
    '£' as i32 as libc::c_char,
    '¤' as i32 as libc::c_char,
    '¥' as i32 as libc::c_char,
    '¦' as i32 as libc::c_char,
    '§' as i32 as libc::c_char,
    '¨' as i32 as libc::c_char,
    'Õ' as i32 as libc::c_char,
    '.' as i32 as libc::c_char,
    '<' as i32 as libc::c_char,
    '(' as i32 as libc::c_char,
    '+' as i32 as libc::c_char,
    '|' as i32 as libc::c_char,
    '&' as i32 as libc::c_char,
    '©' as i32 as libc::c_char,
    'ª' as i32 as libc::c_char,
    '«' as i32 as libc::c_char,
    '¬' as i32 as libc::c_char,
    '\u{ad}' as i32 as libc::c_char,
    '®' as i32 as libc::c_char,
    '¯' as i32 as libc::c_char,
    '°' as i32 as libc::c_char,
    '±' as i32 as libc::c_char,
    '!' as i32 as libc::c_char,
    '$' as i32 as libc::c_char,
    '*' as i32 as libc::c_char,
    ')' as i32 as libc::c_char,
    ';' as i32 as libc::c_char,
    '~' as i32 as libc::c_char,
    '-' as i32 as libc::c_char,
    '/' as i32 as libc::c_char,
    '²' as i32 as libc::c_char,
    '³' as i32 as libc::c_char,
    '´' as i32 as libc::c_char,
    'µ' as i32 as libc::c_char,
    '¶' as i32 as libc::c_char,
    '·' as i32 as libc::c_char,
    '¸' as i32 as libc::c_char,
    '¹' as i32 as libc::c_char,
    'Ë' as i32 as libc::c_char,
    ',' as i32 as libc::c_char,
    '%' as i32 as libc::c_char,
    '_' as i32 as libc::c_char,
    '>' as i32 as libc::c_char,
    '?' as i32 as libc::c_char,
    'º' as i32 as libc::c_char,
    '»' as i32 as libc::c_char,
    '¼' as i32 as libc::c_char,
    '½' as i32 as libc::c_char,
    '¾' as i32 as libc::c_char,
    '¿' as i32 as libc::c_char,
    'À' as i32 as libc::c_char,
    'Á' as i32 as libc::c_char,
    'Â' as i32 as libc::c_char,
    '`' as i32 as libc::c_char,
    ':' as i32 as libc::c_char,
    '#' as i32 as libc::c_char,
    '@' as i32 as libc::c_char,
    '\'' as i32 as libc::c_char,
    '=' as i32 as libc::c_char,
    '"' as i32 as libc::c_char,
    'Ã' as i32 as libc::c_char,
    'a' as i32 as libc::c_char,
    'b' as i32 as libc::c_char,
    'c' as i32 as libc::c_char,
    'd' as i32 as libc::c_char,
    'e' as i32 as libc::c_char,
    'f' as i32 as libc::c_char,
    'g' as i32 as libc::c_char,
    'h' as i32 as libc::c_char,
    'i' as i32 as libc::c_char,
    'Ä' as i32 as libc::c_char,
    'Å' as i32 as libc::c_char,
    'Æ' as i32 as libc::c_char,
    'Ç' as i32 as libc::c_char,
    'È' as i32 as libc::c_char,
    'É' as i32 as libc::c_char,
    'Ê' as i32 as libc::c_char,
    'j' as i32 as libc::c_char,
    'k' as i32 as libc::c_char,
    'l' as i32 as libc::c_char,
    'm' as i32 as libc::c_char,
    'n' as i32 as libc::c_char,
    'o' as i32 as libc::c_char,
    'p' as i32 as libc::c_char,
    'q' as i32 as libc::c_char,
    'r' as i32 as libc::c_char,
    '^' as i32 as libc::c_char,
    'Ì' as i32 as libc::c_char,
    'Í' as i32 as libc::c_char,
    'Î' as i32 as libc::c_char,
    'Ï' as i32 as libc::c_char,
    'Ð' as i32 as libc::c_char,
    'Ñ' as i32 as libc::c_char,
    'å' as i32 as libc::c_char,
    's' as i32 as libc::c_char,
    't' as i32 as libc::c_char,
    'u' as i32 as libc::c_char,
    'v' as i32 as libc::c_char,
    'w' as i32 as libc::c_char,
    'x' as i32 as libc::c_char,
    'y' as i32 as libc::c_char,
    'z' as i32 as libc::c_char,
    'Ò' as i32 as libc::c_char,
    'Ó' as i32 as libc::c_char,
    'Ô' as i32 as libc::c_char,
    '[' as i32 as libc::c_char,
    'Ö' as i32 as libc::c_char,
    '×' as i32 as libc::c_char,
    'Ø' as i32 as libc::c_char,
    'Ù' as i32 as libc::c_char,
    'Ú' as i32 as libc::c_char,
    'Û' as i32 as libc::c_char,
    'Ü' as i32 as libc::c_char,
    'Ý' as i32 as libc::c_char,
    'Þ' as i32 as libc::c_char,
    'ß' as i32 as libc::c_char,
    'à' as i32 as libc::c_char,
    'á' as i32 as libc::c_char,
    'â' as i32 as libc::c_char,
    'ã' as i32 as libc::c_char,
    'ä' as i32 as libc::c_char,
    ']' as i32 as libc::c_char,
    'æ' as i32 as libc::c_char,
    'ç' as i32 as libc::c_char,
    '{' as i32 as libc::c_char,
    'A' as i32 as libc::c_char,
    'B' as i32 as libc::c_char,
    'C' as i32 as libc::c_char,
    'D' as i32 as libc::c_char,
    'E' as i32 as libc::c_char,
    'F' as i32 as libc::c_char,
    'G' as i32 as libc::c_char,
    'H' as i32 as libc::c_char,
    'I' as i32 as libc::c_char,
    'è' as i32 as libc::c_char,
    'é' as i32 as libc::c_char,
    'ê' as i32 as libc::c_char,
    'ë' as i32 as libc::c_char,
    'ì' as i32 as libc::c_char,
    'í' as i32 as libc::c_char,
    '}' as i32 as libc::c_char,
    'J' as i32 as libc::c_char,
    'K' as i32 as libc::c_char,
    'L' as i32 as libc::c_char,
    'M' as i32 as libc::c_char,
    'N' as i32 as libc::c_char,
    'O' as i32 as libc::c_char,
    'P' as i32 as libc::c_char,
    'Q' as i32 as libc::c_char,
    'R' as i32 as libc::c_char,
    'î' as i32 as libc::c_char,
    'ï' as i32 as libc::c_char,
    'ð' as i32 as libc::c_char,
    'ñ' as i32 as libc::c_char,
    'ò' as i32 as libc::c_char,
    'ó' as i32 as libc::c_char,
    '\\' as i32 as libc::c_char,
    '\u{9f}' as i32 as libc::c_char,
    'S' as i32 as libc::c_char,
    'T' as i32 as libc::c_char,
    'U' as i32 as libc::c_char,
    'V' as i32 as libc::c_char,
    'W' as i32 as libc::c_char,
    'X' as i32 as libc::c_char,
    'Y' as i32 as libc::c_char,
    'Z' as i32 as libc::c_char,
    'ô' as i32 as libc::c_char,
    'õ' as i32 as libc::c_char,
    'ö' as i32 as libc::c_char,
    '÷' as i32 as libc::c_char,
    'ø' as i32 as libc::c_char,
    'ù' as i32 as libc::c_char,
    '0' as i32 as libc::c_char,
    '1' as i32 as libc::c_char,
    '2' as i32 as libc::c_char,
    '3' as i32 as libc::c_char,
    '4' as i32 as libc::c_char,
    '5' as i32 as libc::c_char,
    '6' as i32 as libc::c_char,
    '7' as i32 as libc::c_char,
    '8' as i32 as libc::c_char,
    '9' as i32 as libc::c_char,
    'ú' as i32 as libc::c_char,
    'û' as i32 as libc::c_char,
    'ü' as i32 as libc::c_char,
    'ý' as i32 as libc::c_char,
    'þ' as i32 as libc::c_char,
    'ÿ' as i32 as libc::c_char,
];
static mut close_stdout_required: bool = 1 as libc::c_int != 0;
unsafe extern "C" fn maybe_close_stdout() {
    if close_stdout_required {
        close_stdout();
    } else if close_stream(stderr) != 0 as libc::c_int {
        _exit(1 as libc::c_int);
    }
}
unsafe extern "C" fn nl_error(
    mut status: libc::c_int,
    mut errnum: libc::c_int,
    mut fmt: *const libc::c_char,
    mut args: ...
) {
    if (0 as libc::c_int) < progress_len {
        fputc_unlocked('\n' as i32, stderr);
        progress_len = 0 as libc::c_int;
    }
    let mut ap: ::core::ffi::VaListImpl;
    ap = args.clone();
    verror(status, errnum, fmt, ap.as_va_list());
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
                b"Usage: %s [OPERAND]...\n  or:  %s OPTION\n\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
            program_name,
            program_name,
        );
        fputs_unlocked(
            dcgettext(
                0 as *const libc::c_char,
                b"Copy a file, converting and formatting according to the operands.\n\n  bs=BYTES        read and write up to BYTES bytes at a time (default: 512);\n                  overrides ibs and obs\n  cbs=BYTES       convert BYTES bytes at a time\n  conv=CONVS      convert the file as per the comma separated symbol list\n  count=N         copy only N input blocks\n  ibs=BYTES       read up to BYTES bytes at a time (default: 512)\n\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            stdout,
        );
        fputs_unlocked(
            dcgettext(
                0 as *const libc::c_char,
                b"  if=FILE         read from FILE instead of stdin\n  iflag=FLAGS     read as per the comma separated symbol list\n  obs=BYTES       write BYTES bytes at a time (default: 512)\n  of=FILE         write to FILE instead of stdout\n  oflag=FLAGS     write as per the comma separated symbol list\n  seek=N          (or oseek=N) skip N obs-sized output blocks\n  skip=N          (or iseek=N) skip N ibs-sized input blocks\n  status=LEVEL    The LEVEL of information to print to stderr;\n                  'none' suppresses everything but error messages,\n                  'noxfer' suppresses the final transfer statistics,\n                  'progress' shows periodic transfer statistics\n\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            stdout,
        );
        fputs_unlocked(
            dcgettext(
                0 as *const libc::c_char,
                b"\nN and BYTES may be followed by the following multiplicative suffixes:\nc=1, w=2, b=512, kB=1000, K=1024, MB=1000*1000, M=1024*1024, xM=M,\nGB=1000*1000*1000, G=1024*1024*1024, and so on for T, P, E, Z, Y, R, Q.\nBinary prefixes can be used, too: KiB=K, MiB=M, and so on.\nIf N ends in 'B', it counts bytes not blocks.\n\nEach CONV symbol may be:\n\n\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            stdout,
        );
        fputs_unlocked(
            dcgettext(
                0 as *const libc::c_char,
                b"  ascii     from EBCDIC to ASCII\n  ebcdic    from ASCII to EBCDIC\n  ibm       from ASCII to alternate EBCDIC\n  block     pad newline-terminated records with spaces to cbs-size\n  unblock   replace trailing spaces in cbs-size records with newline\n  lcase     change upper case to lower case\n  ucase     change lower case to upper case\n  sparse    try to seek rather than write all-NUL output blocks\n  swab      swap every pair of input bytes\n  sync      pad every input block with NULs to ibs-size; when used\n            with block or unblock, pad with spaces rather than NULs\n\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            stdout,
        );
        fputs_unlocked(
            dcgettext(
                0 as *const libc::c_char,
                b"  excl      fail if the output file already exists\n  nocreat   do not create the output file\n  notrunc   do not truncate the output file\n  noerror   continue after read errors\n  fdatasync  physically write output file data before finishing\n  fsync     likewise, but also write metadata\n\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            stdout,
        );
        fputs_unlocked(
            dcgettext(
                0 as *const libc::c_char,
                b"\nEach FLAG symbol may be:\n\n  append    append mode (makes sense only for output; conv=notrunc suggested)\n\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            stdout,
        );
        fputs_unlocked(
            dcgettext(
                0 as *const libc::c_char,
                b"  direct    use direct I/O for data\n\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
            stdout,
        );
        fputs_unlocked(
            dcgettext(
                0 as *const libc::c_char,
                b"  directory  fail unless a directory\n\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
            stdout,
        );
        fputs_unlocked(
            dcgettext(
                0 as *const libc::c_char,
                b"  dsync     use synchronized I/O for data\n\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
            stdout,
        );
        fputs_unlocked(
            dcgettext(
                0 as *const libc::c_char,
                b"  sync      likewise, but also for metadata\n\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
            stdout,
        );
        fputs_unlocked(
            dcgettext(
                0 as *const libc::c_char,
                b"  fullblock  accumulate full blocks of input (iflag only)\n\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            stdout,
        );
        fputs_unlocked(
            dcgettext(
                0 as *const libc::c_char,
                b"  nonblock  use non-blocking I/O\n\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
            stdout,
        );
        fputs_unlocked(
            dcgettext(
                0 as *const libc::c_char,
                b"  noatime   do not update access time\n\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
            stdout,
        );
        if O_NOCACHE as libc::c_int != 0 {
            fputs_unlocked(
                dcgettext(
                    0 as *const libc::c_char,
                    b"  nocache   Request to drop cache.  See also oflag=sync\n\0"
                        as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
                stdout,
            );
        }
        fputs_unlocked(
            dcgettext(
                0 as *const libc::c_char,
                b"  noctty    do not assign controlling terminal from file\n\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            stdout,
        );
        fputs_unlocked(
            dcgettext(
                0 as *const libc::c_char,
                b"  nofollow  do not follow symlinks\n\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
            stdout,
        );
        printf(
            dcgettext(
                0 as *const libc::c_char,
                b"\nSending a %s signal to a running 'dd' process makes it\nprint I/O statistics to standard error and then resume copying.\n\nOptions are:\n\n\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            if 10 as libc::c_int == 10 as libc::c_int {
                b"USR1\0" as *const u8 as *const libc::c_char
            } else {
                b"INFO\0" as *const u8 as *const libc::c_char
            },
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
        emit_ancillary_info(b"dd\0" as *const u8 as *const libc::c_char);
    }
    exit(status);
}
unsafe extern "C" fn alloc_ibuf() {
    if !ibuf.is_null() {
        return;
    }
    let mut extra_byte_for_swab: bool = conversions_mask & C_SWAB as libc::c_int != 0;
    ibuf = alignalloc(page_size, input_blocksize + extra_byte_for_swab as libc::c_long)
        as *mut libc::c_char;
    if ibuf.is_null() {
        let mut hbuf: [libc::c_char; 652] = [0; 652];
        if ::core::mem::size_of::<C2RustUnnamed_17>() as libc::c_ulong != 0 {
            nl_error(
                1 as libc::c_int,
                0 as libc::c_int,
                dcgettext(
                    0 as *const libc::c_char,
                    b"memory exhausted by input buffer of size %td bytes (%s)\0"
                        as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
                input_blocksize,
                human_readable(
                    input_blocksize as uintmax_t,
                    hbuf.as_mut_ptr(),
                    human_opts as libc::c_int | human_base_1024 as libc::c_int,
                    1 as libc::c_int as uintmax_t,
                    1 as libc::c_int as uintmax_t,
                ),
            );
            if 0 as libc::c_int != 0 {} else {
                unreachable!();
            };
        } else {
            nl_error(
                1 as libc::c_int,
                0 as libc::c_int,
                dcgettext(
                    0 as *const libc::c_char,
                    b"memory exhausted by input buffer of size %td bytes (%s)\0"
                        as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
                input_blocksize,
                human_readable(
                    input_blocksize as uintmax_t,
                    hbuf.as_mut_ptr(),
                    human_opts as libc::c_int | human_base_1024 as libc::c_int,
                    1 as libc::c_int as uintmax_t,
                    1 as libc::c_int as uintmax_t,
                ),
            );
            if 0 as libc::c_int != 0 {} else {
                unreachable!();
            };
        };
    }
}
unsafe extern "C" fn alloc_obuf() {
    if !obuf.is_null() {
        return;
    }
    if conversions_mask & C_TWOBUFS as libc::c_int != 0 {
        obuf = alignalloc(page_size, output_blocksize) as *mut libc::c_char;
        if obuf.is_null() {
            let mut hbuf: [libc::c_char; 652] = [0; 652];
            if ::core::mem::size_of::<C2RustUnnamed_18>() as libc::c_ulong != 0 {
                nl_error(
                    1 as libc::c_int,
                    0 as libc::c_int,
                    dcgettext(
                        0 as *const libc::c_char,
                        b"memory exhausted by output buffer of size %td bytes (%s)\0"
                            as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    output_blocksize,
                    human_readable(
                        output_blocksize as uintmax_t,
                        hbuf.as_mut_ptr(),
                        human_opts as libc::c_int | human_base_1024 as libc::c_int,
                        1 as libc::c_int as uintmax_t,
                        1 as libc::c_int as uintmax_t,
                    ),
                );
                if 0 as libc::c_int != 0 {} else {
                    unreachable!();
                };
            } else {
                nl_error(
                    1 as libc::c_int,
                    0 as libc::c_int,
                    dcgettext(
                        0 as *const libc::c_char,
                        b"memory exhausted by output buffer of size %td bytes (%s)\0"
                            as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    output_blocksize,
                    human_readable(
                        output_blocksize as uintmax_t,
                        hbuf.as_mut_ptr(),
                        human_opts as libc::c_int | human_base_1024 as libc::c_int,
                        1 as libc::c_int as uintmax_t,
                        1 as libc::c_int as uintmax_t,
                    ),
                );
                if 0 as libc::c_int != 0 {} else {
                    unreachable!();
                };
            };
        }
    } else {
        alloc_ibuf();
        obuf = ibuf;
    };
}
unsafe extern "C" fn translate_charset(mut new_trans: *const libc::c_char) {
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < 256 as libc::c_int {
        trans_table[i
            as usize] = *new_trans.offset(trans_table[i as usize] as isize)
            as libc::c_uchar;
        i += 1;
    }
    translation_needed = 1 as libc::c_int != 0;
}
#[inline]
unsafe extern "C" fn multiple_bits_set(mut i: libc::c_int) -> bool {
    return i & i - 1 as libc::c_int != 0 as libc::c_int;
}
unsafe extern "C" fn abbreviation_lacks_prefix(
    mut message: *const libc::c_char,
) -> bool {
    return *message
        .offset(
            (strlen(message)).wrapping_sub(2 as libc::c_int as libc::c_ulong) as isize,
        ) as libc::c_int == ' ' as i32;
}
unsafe extern "C" fn print_xfer_stats(mut progress_time: xtime_t) {
    let mut now: xtime_t = if progress_time != 0 { progress_time } else { gethrxtime() };
    static mut slash_s: [libc::c_char; 3] = unsafe {
        *::core::mem::transmute::<&[u8; 3], &[libc::c_char; 3]>(b"/s\0")
    };
    let mut hbuf: [[libc::c_char; 654]; 3] = [[0; 654]; 3];
    let mut delta_s: libc::c_double = 0.;
    let mut bytes_per_second: *const libc::c_char = 0 as *const libc::c_char;
    let mut si: *const libc::c_char = human_readable(
        w_bytes as uintmax_t,
        (hbuf[0 as libc::c_int as usize]).as_mut_ptr(),
        human_opts as libc::c_int,
        1 as libc::c_int as uintmax_t,
        1 as libc::c_int as uintmax_t,
    );
    let mut iec: *const libc::c_char = human_readable(
        w_bytes as uintmax_t,
        (hbuf[1 as libc::c_int as usize]).as_mut_ptr(),
        human_opts as libc::c_int | human_base_1024 as libc::c_int,
        1 as libc::c_int as uintmax_t,
        1 as libc::c_int as uintmax_t,
    );
    let mut bpsbuf: *mut libc::c_char = (hbuf[2 as libc::c_int as usize]).as_mut_ptr();
    let mut bpsbufsize: libc::c_int = ::core::mem::size_of::<[libc::c_char; 654]>()
        as libc::c_ulong as libc::c_int;
    if start_time < now {
        let mut XTIME_PRECISIONe0: libc::c_double = 1000000000 as libc::c_int
            as libc::c_double;
        let mut delta_xtime: xtime_t = now - start_time;
        delta_s = delta_xtime as libc::c_double / XTIME_PRECISIONe0;
        bytes_per_second = human_readable(
            w_bytes as uintmax_t,
            bpsbuf,
            human_opts as libc::c_int,
            1000000000 as libc::c_int as uintmax_t,
            delta_xtime as uintmax_t,
        );
        strcat(
            bpsbuf.offset(bytes_per_second.offset_from(bpsbuf) as libc::c_long as isize),
            slash_s.as_ptr(),
        );
    } else {
        delta_s = 0 as libc::c_int as libc::c_double;
        snprintf(
            bpsbuf,
            bpsbufsize as libc::c_ulong,
            b"%s B/s\0" as *const u8 as *const libc::c_char,
            dcgettext(
                0 as *const libc::c_char,
                b"Infinity\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
        bytes_per_second = bpsbuf;
    }
    if progress_time != 0 {
        fputc_unlocked('\r' as i32, stderr);
    }
    let mut delta_s_buf: [libc::c_char; 24] = [0; 24];
    snprintf(
        delta_s_buf.as_mut_ptr(),
        ::core::mem::size_of::<[libc::c_char; 24]>() as libc::c_ulong,
        if progress_time != 0 {
            b"%.0f s\0" as *const u8 as *const libc::c_char
        } else {
            b"%g s\0" as *const u8 as *const libc::c_char
        },
        delta_s,
    );
    let mut stats_len: libc::c_int = if abbreviation_lacks_prefix(si) as libc::c_int != 0
    {
        fprintf(
            stderr,
            dcngettext(
                0 as *const libc::c_char,
                b"%ld byte copied, %s, %s\0" as *const u8 as *const libc::c_char,
                b"%ld bytes copied, %s, %s\0" as *const u8 as *const libc::c_char,
                select_plural(w_bytes as uintmax_t),
                5 as libc::c_int,
            ),
            w_bytes,
            delta_s_buf.as_mut_ptr(),
            bytes_per_second,
        )
    } else if abbreviation_lacks_prefix(iec) as libc::c_int != 0 {
        fprintf(
            stderr,
            dcgettext(
                0 as *const libc::c_char,
                b"%ld bytes (%s) copied, %s, %s\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            w_bytes,
            si,
            delta_s_buf.as_mut_ptr(),
            bytes_per_second,
        )
    } else {
        fprintf(
            stderr,
            dcgettext(
                0 as *const libc::c_char,
                b"%ld bytes (%s, %s) copied, %s, %s\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
            w_bytes,
            si,
            iec,
            delta_s_buf.as_mut_ptr(),
            bytes_per_second,
        )
    };
    if progress_time != 0 {
        if 0 as libc::c_int <= stats_len && stats_len < progress_len {
            fprintf(
                stderr,
                b"%*s\0" as *const u8 as *const libc::c_char,
                progress_len - stats_len,
                b"\0" as *const u8 as *const libc::c_char,
            );
        }
        progress_len = stats_len;
    } else {
        fputc_unlocked('\n' as i32, stderr);
    }
    reported_w_bytes = w_bytes;
}
unsafe extern "C" fn print_stats() {
    if status_level == STATUS_NONE as libc::c_int {
        return;
    }
    if (0 as libc::c_int) < progress_len {
        fputc_unlocked('\n' as i32, stderr);
        progress_len = 0 as libc::c_int;
    }
    fprintf(
        stderr,
        dcgettext(
            0 as *const libc::c_char,
            b"%ld+%ld records in\n%ld+%ld records out\n\0" as *const u8
                as *const libc::c_char,
            5 as libc::c_int,
        ),
        r_full,
        r_partial,
        w_full,
        w_partial,
    );
    if r_truncate != 0 as libc::c_int as libc::c_long {
        fprintf(
            stderr,
            dcngettext(
                0 as *const libc::c_char,
                b"%ld truncated record\n\0" as *const u8 as *const libc::c_char,
                b"%ld truncated records\n\0" as *const u8 as *const libc::c_char,
                select_plural(r_truncate as uintmax_t),
                5 as libc::c_int,
            ),
            r_truncate,
        );
    }
    if status_level == STATUS_NOXFER as libc::c_int {
        return;
    }
    print_xfer_stats(0 as libc::c_int as xtime_t);
}
unsafe extern "C" fn interrupt_handler(mut sig: libc::c_int) {
    if 0x80000000 as libc::c_uint == 0 {
        signal(sig, None);
    }
    ::core::ptr::write_volatile(&mut interrupt_signal as *mut sig_atomic_t, sig);
}
unsafe extern "C" fn siginfo_handler(mut sig: libc::c_int) {
    if 1 as libc::c_int == 0 {
        signal(sig, Some(siginfo_handler as unsafe extern "C" fn(libc::c_int) -> ()));
    }
    ::core::ptr::write_volatile(
        &mut info_signal_count as *mut sig_atomic_t,
        ::core::ptr::read_volatile::<
            sig_atomic_t,
        >(&info_signal_count as *const sig_atomic_t) + 1,
    );
}
unsafe extern "C" fn install_signal_handlers() {
    let mut catch_siginfo: bool = !(10 as libc::c_int == 10 as libc::c_int
        && !(getenv(b"POSIXLY_CORRECT\0" as *const u8 as *const libc::c_char))
            .is_null());
    let mut act: sigaction = sigaction {
        __sigaction_handler: C2RustUnnamed_9 {
            sa_handler: None,
        },
        sa_mask: sigset_t { __val: [0; 16] },
        sa_flags: 0,
        sa_restorer: None,
    };
    sigemptyset(&mut caught_signals);
    if catch_siginfo {
        sigaddset(&mut caught_signals, 10 as libc::c_int);
    }
    sigaction(2 as libc::c_int, 0 as *const sigaction, &mut act);
    if act.__sigaction_handler.sa_handler
        != ::core::mem::transmute::<
            libc::intptr_t,
            __sighandler_t,
        >(1 as libc::c_int as libc::intptr_t)
    {
        sigaddset(&mut caught_signals, 2 as libc::c_int);
    }
    act.sa_mask = caught_signals;
    if sigismember(&mut caught_signals, 10 as libc::c_int) != 0 {
        act
            .__sigaction_handler
            .sa_handler = Some(
            siginfo_handler as unsafe extern "C" fn(libc::c_int) -> (),
        );
        act.sa_flags = 0 as libc::c_int;
        sigaction(10 as libc::c_int, &mut act, 0 as *mut sigaction);
    }
    if sigismember(&mut caught_signals, 2 as libc::c_int) != 0 {
        act
            .__sigaction_handler
            .sa_handler = Some(
            interrupt_handler as unsafe extern "C" fn(libc::c_int) -> (),
        );
        act
            .sa_flags = (0x40000000 as libc::c_int as libc::c_uint
            | 0x80000000 as libc::c_uint) as libc::c_int;
        sigaction(2 as libc::c_int, &mut act, 0 as *mut sigaction);
    }
}
unsafe extern "C" fn iclose(mut fd: libc::c_int) -> libc::c_int {
    if close(fd) != 0 as libc::c_int {
        loop {
            if *__errno_location() != 4 as libc::c_int {
                return -(1 as libc::c_int);
            }
            if !(close(fd) != 0 as libc::c_int
                && *__errno_location() != 9 as libc::c_int)
            {
                break;
            }
        }
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn cleanup() {
    if interrupt_signal == 0 {
        let mut sync_status: libc::c_int = synchronize_output();
        if sync_status != 0 {
            exit(sync_status);
        }
    }
    if iclose(0 as libc::c_int) != 0 as libc::c_int {
        if ::core::mem::size_of::<C2RustUnnamed_20>() as libc::c_ulong != 0 {
            nl_error(
                1 as libc::c_int,
                *__errno_location(),
                dcgettext(
                    0 as *const libc::c_char,
                    b"closing input file %s\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
                quotearg_style(shell_escape_always_quoting_style, input_file),
            );
            if 0 as libc::c_int != 0 {} else {
                unreachable!();
            };
        } else {
            nl_error(
                1 as libc::c_int,
                *__errno_location(),
                dcgettext(
                    0 as *const libc::c_char,
                    b"closing input file %s\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
                quotearg_style(shell_escape_always_quoting_style, input_file),
            );
            if 0 as libc::c_int != 0 {} else {
                unreachable!();
            };
        };
    }
    if iclose(1 as libc::c_int) != 0 as libc::c_int {
        if ::core::mem::size_of::<C2RustUnnamed_19>() as libc::c_ulong != 0 {
            nl_error(
                1 as libc::c_int,
                *__errno_location(),
                dcgettext(
                    0 as *const libc::c_char,
                    b"closing output file %s\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
                quotearg_style(shell_escape_always_quoting_style, output_file),
            );
            if 0 as libc::c_int != 0 {} else {
                unreachable!();
            };
        } else {
            nl_error(
                1 as libc::c_int,
                *__errno_location(),
                dcgettext(
                    0 as *const libc::c_char,
                    b"closing output file %s\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
                quotearg_style(shell_escape_always_quoting_style, output_file),
            );
            if 0 as libc::c_int != 0 {} else {
                unreachable!();
            };
        };
    }
}
unsafe extern "C" fn process_signals() {
    while interrupt_signal != 0 || info_signal_count != 0 {
        let mut interrupt: libc::c_int = 0;
        let mut infos: libc::c_int = 0;
        let mut oldset: sigset_t = sigset_t { __val: [0; 16] };
        sigprocmask(0 as libc::c_int, &mut caught_signals, &mut oldset);
        interrupt = interrupt_signal;
        infos = info_signal_count;
        if infos != 0 {
            ::core::ptr::write_volatile(
                &mut info_signal_count as *mut sig_atomic_t,
                infos - 1 as libc::c_int,
            );
        }
        sigprocmask(2 as libc::c_int, &mut oldset, 0 as *mut sigset_t);
        if interrupt != 0 {
            cleanup();
        }
        print_stats();
        if interrupt != 0 {
            raise(interrupt);
        }
    }
}
unsafe extern "C" fn finish_up() {
    process_signals();
    cleanup();
    print_stats();
}
unsafe extern "C" fn quit(mut code: libc::c_int) {
    finish_up();
    exit(code);
}
unsafe extern "C" fn cache_round(mut fd: libc::c_int, mut len: off_t) -> off_t {
    static mut i_pending: off_t = 0;
    static mut o_pending: off_t = 0;
    let mut pending: *mut off_t = if fd == 0 as libc::c_int {
        &mut i_pending
    } else {
        &mut o_pending
    };
    if len != 0 {
        let mut c_pending: intmax_t = 0;
        let (fresh1, fresh2) = (*pending).overflowing_add(len);
        *(&mut c_pending as *mut intmax_t) = fresh1;
        if fresh2 {
            c_pending = 9223372036854775807 as libc::c_long;
        }
        *pending = c_pending % IO_BUFSIZE as libc::c_int as libc::c_long;
        if c_pending > *pending {
            len = c_pending - *pending;
        } else {
            len = 0 as libc::c_int as off_t;
        }
    } else {
        len = *pending;
    }
    return len;
}
unsafe extern "C" fn invalidate_cache(mut fd: libc::c_int, mut len: off_t) -> bool {
    let mut adv_ret: libc::c_int = -(1 as libc::c_int);
    let mut offset: off_t = 0;
    let mut nocache_eof: bool = if fd == 0 as libc::c_int {
        i_nocache_eof as libc::c_int
    } else {
        o_nocache_eof as libc::c_int
    } != 0;
    let mut clen: off_t = cache_round(fd, len);
    if len != 0 && clen == 0 {
        return 1 as libc::c_int != 0
    } else {
        if len == 0 && clen == 0 && !nocache_eof {
            return 1 as libc::c_int != 0;
        }
    }
    let mut pending: off_t = if len != 0 {
        cache_round(fd, 0 as libc::c_int as off_t)
    } else {
        0 as libc::c_int as libc::c_long
    };
    if fd == 0 as libc::c_int {
        if input_seekable {
            offset = input_offset;
        } else {
            offset = -(1 as libc::c_int) as off_t;
            *__errno_location() = 29 as libc::c_int;
        }
    } else {
        static mut output_offset: off_t = -(2 as libc::c_int) as off_t;
        if output_offset != -(1 as libc::c_int) as libc::c_long {
            if output_offset < 0 as libc::c_int as libc::c_long {
                output_offset = lseek(fd, 0 as libc::c_int as __off_t, 1 as libc::c_int);
            } else if len != 0 {
                output_offset += clen + pending;
            }
        }
        offset = output_offset;
    }
    if 0 as libc::c_int as libc::c_long <= offset {
        if len == 0 && clen != 0 && nocache_eof as libc::c_int != 0 {
            pending = clen;
            clen = 0 as libc::c_int as off_t;
        }
        offset = offset - clen - pending;
        if clen == 0 as libc::c_int as libc::c_long {
            offset -= offset % page_size;
        }
        adv_ret = posix_fadvise(fd, offset, clen, 4 as libc::c_int);
    }
    return if adv_ret != -(1 as libc::c_int) {
        1 as libc::c_int
    } else {
        0 as libc::c_int
    } != 0;
}
unsafe extern "C" fn iread(
    mut fd: libc::c_int,
    mut buf: *mut libc::c_char,
    mut size: idx_t,
) -> ssize_t {
    let mut nread: ssize_t = 0;
    static mut prev_nread: ssize_t = 0;
    loop {
        process_signals();
        nread = read(fd, buf as *mut libc::c_void, size as size_t);
        if nread == -(1 as libc::c_int) as libc::c_long
            && *__errno_location() == 22 as libc::c_int
            && (0 as libc::c_int as libc::c_long) < prev_nread && prev_nread < size
            && input_flags & 0o200000 as libc::c_int != 0
        {
            *__errno_location() = 0 as libc::c_int;
            nread = 0 as libc::c_int as ssize_t;
        }
        if !(nread < 0 as libc::c_int as libc::c_long
            && *__errno_location() == 4 as libc::c_int)
        {
            break;
        }
    }
    if (0 as libc::c_int as libc::c_long) < nread && nread < size {
        process_signals();
    }
    if (0 as libc::c_int as libc::c_long) < nread
        && warn_partial_read as libc::c_int != 0
    {
        if (0 as libc::c_int as libc::c_long) < prev_nread && prev_nread < size {
            let mut prev: idx_t = prev_nread;
            if status_level != STATUS_NONE as libc::c_int {
                nl_error(
                    0 as libc::c_int,
                    0 as libc::c_int,
                    dcngettext(
                        0 as *const libc::c_char,
                        b"warning: partial read (%td byte); suggest iflag=fullblock\0"
                            as *const u8 as *const libc::c_char,
                        b"warning: partial read (%td bytes); suggest iflag=fullblock\0"
                            as *const u8 as *const libc::c_char,
                        select_plural(prev as uintmax_t),
                        5 as libc::c_int,
                    ),
                    prev,
                );
            }
            warn_partial_read = 0 as libc::c_int != 0;
        }
    }
    prev_nread = nread;
    return nread;
}
unsafe extern "C" fn iread_fullblock(
    mut fd: libc::c_int,
    mut buf: *mut libc::c_char,
    mut size: idx_t,
) -> ssize_t {
    let mut nread: ssize_t = 0 as libc::c_int as ssize_t;
    while (0 as libc::c_int as libc::c_long) < size {
        let mut ncurr: ssize_t = iread(fd, buf, size);
        if ncurr < 0 as libc::c_int as libc::c_long {
            return ncurr;
        }
        if ncurr == 0 as libc::c_int as libc::c_long {
            break;
        }
        nread += ncurr;
        buf = buf.offset(ncurr as isize);
        size -= ncurr;
    }
    return nread;
}
unsafe extern "C" fn iwrite(
    mut fd: libc::c_int,
    mut buf: *const libc::c_char,
    mut size: idx_t,
) -> idx_t {
    let mut total_written: idx_t = 0 as libc::c_int as idx_t;
    if output_flags & 0o200000 as libc::c_int != 0 && size < output_blocksize {
        let mut old_flags: libc::c_int = rpl_fcntl(1 as libc::c_int, 3 as libc::c_int);
        if rpl_fcntl(
            1 as libc::c_int,
            4 as libc::c_int,
            old_flags & !(0o200000 as libc::c_int),
        ) != 0 as libc::c_int && status_level != STATUS_NONE as libc::c_int
        {
            nl_error(
                0 as libc::c_int,
                *__errno_location(),
                dcgettext(
                    0 as *const libc::c_char,
                    b"failed to turn off O_DIRECT: %s\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
                quotearg_n_style_colon(
                    0 as libc::c_int,
                    shell_escape_quoting_style,
                    output_file,
                ),
            );
        }
        o_nocache_eof = 1 as libc::c_int != 0;
        invalidate_cache(1 as libc::c_int, 0 as libc::c_int as off_t);
        conversions_mask |= C_FSYNC as libc::c_int;
    }
    while total_written < size {
        let mut nwritten: ssize_t = 0 as libc::c_int as ssize_t;
        process_signals();
        final_op_was_seek = 0 as libc::c_int != 0;
        if conversions_mask & C_SPARSE as libc::c_int != 0
            && is_nul(buf as *const libc::c_void, size as size_t) as libc::c_int != 0
        {
            if lseek(fd, size, 1 as libc::c_int) < 0 as libc::c_int as libc::c_long {
                conversions_mask &= !(C_SPARSE as libc::c_int);
            } else {
                final_op_was_seek = 1 as libc::c_int != 0;
                nwritten = size;
            }
        }
        if nwritten == 0 {
            nwritten = write(
                fd,
                buf.offset(total_written as isize) as *const libc::c_void,
                (size - total_written) as size_t,
            );
        }
        if nwritten < 0 as libc::c_int as libc::c_long {
            if *__errno_location() != 4 as libc::c_int {
                break;
            }
        } else if nwritten == 0 as libc::c_int as libc::c_long {
            *__errno_location() = 28 as libc::c_int;
            break;
        } else {
            total_written += nwritten;
        }
    }
    if o_nocache as libc::c_int != 0 && total_written != 0 {
        invalidate_cache(fd, total_written);
    }
    return total_written;
}
unsafe extern "C" fn write_output() {
    let mut nwritten: idx_t = iwrite(1 as libc::c_int, obuf, output_blocksize);
    w_bytes += nwritten;
    if nwritten != output_blocksize {
        nl_error(
            0 as libc::c_int,
            *__errno_location(),
            dcgettext(
                0 as *const libc::c_char,
                b"writing to %s\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            quotearg_style(shell_escape_always_quoting_style, output_file),
        );
        if nwritten != 0 as libc::c_int as libc::c_long {
            w_partial += 1;
        }
        quit(1 as libc::c_int);
    } else {
        w_full += 1;
    }
    oc = 0 as libc::c_int as idx_t;
}
unsafe extern "C" fn ifdatasync(mut fd: libc::c_int) -> libc::c_int {
    let mut ret: libc::c_int = 0;
    loop {
        process_signals();
        ret = fdatasync(fd);
        if !(ret < 0 as libc::c_int && *__errno_location() == 4 as libc::c_int) {
            break;
        }
    }
    return ret;
}
unsafe extern "C" fn ifd_reopen(
    mut desired_fd: libc::c_int,
    mut file: *const libc::c_char,
    mut flag: libc::c_int,
    mut mode: mode_t,
) -> libc::c_int {
    let mut ret: libc::c_int = 0;
    loop {
        process_signals();
        ret = fd_reopen(desired_fd, file, flag, mode);
        if !(ret < 0 as libc::c_int && *__errno_location() == 4 as libc::c_int) {
            break;
        }
    }
    return ret;
}
unsafe extern "C" fn ifstat(mut fd: libc::c_int, mut st: *mut stat) -> libc::c_int {
    let mut ret: libc::c_int = 0;
    loop {
        process_signals();
        ret = fstat(fd, st);
        if !(ret < 0 as libc::c_int && *__errno_location() == 4 as libc::c_int) {
            break;
        }
    }
    return ret;
}
unsafe extern "C" fn ifsync(mut fd: libc::c_int) -> libc::c_int {
    let mut ret: libc::c_int = 0;
    loop {
        process_signals();
        ret = fsync(fd);
        if !(ret < 0 as libc::c_int && *__errno_location() == 4 as libc::c_int) {
            break;
        }
    }
    return ret;
}
unsafe extern "C" fn iftruncate(mut fd: libc::c_int, mut length: off_t) -> libc::c_int {
    let mut ret: libc::c_int = 0;
    loop {
        process_signals();
        ret = ftruncate(fd, length);
        if !(ret < 0 as libc::c_int && *__errno_location() == 4 as libc::c_int) {
            break;
        }
    }
    return ret;
}
unsafe extern "C" fn operand_matches(
    mut str: *const libc::c_char,
    mut pattern: *const libc::c_char,
    mut delim: libc::c_char,
) -> bool {
    while *pattern != 0 {
        let fresh3 = str;
        str = str.offset(1);
        let fresh4 = pattern;
        pattern = pattern.offset(1);
        if *fresh3 as libc::c_int != *fresh4 as libc::c_int {
            return 0 as libc::c_int != 0;
        }
    }
    return *str == 0 || *str as libc::c_int == delim as libc::c_int;
}
unsafe extern "C" fn parse_symbols(
    mut str: *const libc::c_char,
    mut table: *const symbol_value,
    mut exclusive: bool,
    mut error_msgid: *const libc::c_char,
) -> libc::c_int {
    let mut value: libc::c_int = 0 as libc::c_int;
    loop {
        let mut strcomma: *const libc::c_char = strchr(str, ',' as i32);
        let mut entry: *const symbol_value = 0 as *const symbol_value;
        entry = table;
        while !(operand_matches(
            str,
            ((*entry).symbol).as_ptr(),
            ',' as i32 as libc::c_char,
        ) as libc::c_int != 0 && (*entry).value != 0)
        {
            if (*entry).symbol[0 as libc::c_int as usize] == 0 {
                let mut slen: idx_t = (if !strcomma.is_null() {
                    strcomma.offset_from(str) as libc::c_long as libc::c_ulong
                } else {
                    strlen(str)
                }) as idx_t;
                nl_error(
                    0 as libc::c_int,
                    0 as libc::c_int,
                    b"%s: %s\0" as *const u8 as *const libc::c_char,
                    dcgettext(0 as *const libc::c_char, error_msgid, 5 as libc::c_int),
                    quotearg_n_style_mem(
                        0 as libc::c_int,
                        locale_quoting_style,
                        str,
                        slen as size_t,
                    ),
                );
                usage(1 as libc::c_int);
            }
            entry = entry.offset(1);
        }
        if exclusive {
            value = (*entry).value;
        } else {
            value |= (*entry).value;
        }
        if strcomma.is_null() {
            break;
        }
        str = strcomma.offset(1 as libc::c_int as isize);
    }
    return value;
}
unsafe extern "C" fn parse_integer(
    mut str: *const libc::c_char,
    mut invalid: *mut strtol_error,
) -> intmax_t {
    let mut indeterminate: libc::c_int = 0 as libc::c_int;
    let mut n: uintmax_t = indeterminate as uintmax_t;
    let mut suffix: *mut libc::c_char = 0 as *mut libc::c_char;
    static mut suffixes: [libc::c_char; 16] = unsafe {
        *::core::mem::transmute::<&[u8; 16], &[libc::c_char; 16]>(b"bcEGkKMPQRTwYZ0\0")
    };
    let mut e: strtol_error = xstrtoumax(
        str,
        &mut suffix,
        10 as libc::c_int,
        &mut n,
        suffixes.as_ptr(),
    );
    let mut result: intmax_t = 0;
    if e as libc::c_uint & !(LONGINT_OVERFLOW as libc::c_int) as libc::c_uint
        == LONGINT_INVALID_SUFFIX_CHAR as libc::c_int as libc::c_uint
        && *suffix.offset(-(1 as libc::c_int) as isize) as libc::c_int != 'B' as i32
        && *suffix as libc::c_int == 'B' as i32
    {
        suffix = suffix.offset(1);
        if *suffix == 0 {
            e = ::core::mem::transmute::<
                libc::c_uint,
                strtol_error,
            >(
                e as libc::c_uint
                    & !(LONGINT_INVALID_SUFFIX_CHAR as libc::c_int) as libc::c_uint,
            );
        }
    }
    if e as libc::c_uint & !(LONGINT_OVERFLOW as libc::c_int) as libc::c_uint
        == LONGINT_INVALID_SUFFIX_CHAR as libc::c_int as libc::c_uint
        && *suffix as libc::c_int == 'x' as i32
        && !(*suffix.offset(-(1 as libc::c_int) as isize) as libc::c_int == 'B' as i32
            && !(strchr(suffix.offset(1 as libc::c_int as isize), 'B' as i32)).is_null())
    {
        let mut o: uintmax_t = 0;
        let mut f: strtol_error = xstrtoumax(
            suffix.offset(1 as libc::c_int as isize),
            &mut suffix,
            10 as libc::c_int,
            &mut o,
            suffixes.as_ptr(),
        );
        if f as libc::c_uint & !(LONGINT_OVERFLOW as libc::c_int) as libc::c_uint
            != LONGINT_OK as libc::c_int as libc::c_uint
        {
            e = f;
            result = indeterminate as intmax_t;
        } else if (if (0 as libc::c_int as intmax_t) < -(1 as libc::c_int) as intmax_t
            && (if 1 as libc::c_int != 0 {
                0 as libc::c_int as libc::c_ulong
            } else {
                n
            })
                .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                < 0 as libc::c_int as libc::c_ulong
            && (if 1 as libc::c_int != 0 {
                0 as libc::c_int as libc::c_ulong
            } else {
                o
            })
                .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                < 0 as libc::c_int as libc::c_ulong
            && (if o < 0 as libc::c_int as libc::c_ulong {
                (if n < 0 as libc::c_int as libc::c_ulong {
                    (if (if 1 as libc::c_int != 0 {
                        0 as libc::c_int as libc::c_ulong
                    } else {
                        ((if 1 as libc::c_int != 0 {
                            0 as libc::c_int as libc::c_long
                        } else {
                            -(1 as libc::c_int) as intmax_t
                        }) as libc::c_ulong)
                            .wrapping_add(o)
                    })
                        .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                        < 0 as libc::c_int as libc::c_ulong
                    {
                        (n
                            < (-(1 as libc::c_int) as intmax_t as libc::c_ulong)
                                .wrapping_div(o)) as libc::c_int
                    } else {
                        ((if (if (if (if 1 as libc::c_int != 0 {
                            0 as libc::c_int as libc::c_ulong
                        } else {
                            o
                        })
                            .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                            < 0 as libc::c_int as libc::c_ulong
                        {
                            !((if 1 as libc::c_int != 0 {
                                0 as libc::c_int as libc::c_ulong
                            } else {
                                o
                            })
                                .wrapping_add(1 as libc::c_int as libc::c_ulong)
                                << (::core::mem::size_of::<uintmax_t>() as libc::c_ulong)
                                    .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                    .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                .wrapping_mul(2 as libc::c_int as libc::c_ulong)
                                .wrapping_add(1 as libc::c_int as libc::c_ulong)
                        } else {
                            (if 1 as libc::c_int != 0 {
                                0 as libc::c_int as libc::c_ulong
                            } else {
                                o
                            })
                                .wrapping_add(0 as libc::c_int as libc::c_ulong)
                        }) < 0 as libc::c_int as libc::c_ulong
                        {
                            (o
                                < (if (if 1 as libc::c_int != 0 {
                                    0 as libc::c_int as libc::c_ulong
                                } else {
                                    o
                                })
                                    .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                    < 0 as libc::c_int as libc::c_ulong
                                {
                                    ((if 1 as libc::c_int != 0 {
                                        0 as libc::c_int as libc::c_ulong
                                    } else {
                                        o
                                    })
                                        .wrapping_add(1 as libc::c_int as libc::c_ulong)
                                        << (::core::mem::size_of::<uintmax_t>() as libc::c_ulong)
                                            .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                            .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                        .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                        .wrapping_mul(2 as libc::c_int as libc::c_ulong)
                                        .wrapping_add(1 as libc::c_int as libc::c_ulong)
                                } else {
                                    (if 1 as libc::c_int != 0 {
                                        0 as libc::c_int as libc::c_ulong
                                    } else {
                                        o
                                    })
                                        .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                })
                                    .wrapping_neg()) as libc::c_int
                        } else {
                            ((0 as libc::c_int as libc::c_ulong) < o) as libc::c_int
                        }) != 0
                        {
                            (if 1 as libc::c_int != 0 {
                                0 as libc::c_int as libc::c_ulong
                            } else {
                                o
                            })
                                .wrapping_add(
                                    -(1 as libc::c_int) as intmax_t as libc::c_ulong,
                                )
                                >> (::core::mem::size_of::<uintmax_t>() as libc::c_ulong)
                                    .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                    .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                        } else {
                            (-(1 as libc::c_int) as intmax_t as libc::c_ulong)
                                .wrapping_div(o.wrapping_neg())
                        }) <= (-(1 as libc::c_int) as libc::c_ulong).wrapping_sub(n))
                            as libc::c_int
                    })
                } else {
                    (if (if (if (if 1 as libc::c_int != 0 {
                        0 as libc::c_int as libc::c_ulong
                    } else {
                        (if 1 as libc::c_int != 0 {
                            0 as libc::c_int as libc::c_ulong
                        } else {
                            o
                        })
                            .wrapping_add(0 as libc::c_int as libc::c_ulong)
                    })
                        .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                        < 0 as libc::c_int as libc::c_ulong
                    {
                        !((if 1 as libc::c_int != 0 {
                            0 as libc::c_int as libc::c_ulong
                        } else {
                            (if 1 as libc::c_int != 0 {
                                0 as libc::c_int as libc::c_ulong
                            } else {
                                o
                            })
                                .wrapping_add(0 as libc::c_int as libc::c_ulong)
                        })
                            .wrapping_add(1 as libc::c_int as libc::c_ulong)
                            << (::core::mem::size_of::<libc::c_ulong>() as libc::c_ulong)
                                .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                            .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(2 as libc::c_int as libc::c_ulong)
                            .wrapping_add(1 as libc::c_int as libc::c_ulong)
                    } else {
                        (if 1 as libc::c_int != 0 {
                            0 as libc::c_int as libc::c_ulong
                        } else {
                            (if 1 as libc::c_int != 0 {
                                0 as libc::c_int as libc::c_ulong
                            } else {
                                o
                            })
                                .wrapping_add(0 as libc::c_int as libc::c_ulong)
                        })
                            .wrapping_add(0 as libc::c_int as libc::c_ulong)
                    }) < 0 as libc::c_int as libc::c_ulong
                    {
                        ((if 1 as libc::c_int != 0 {
                            0 as libc::c_int as libc::c_ulong
                        } else {
                            o
                        })
                            .wrapping_add(0 as libc::c_int as libc::c_ulong)
                            < (if (if 1 as libc::c_int != 0 {
                                0 as libc::c_int as libc::c_ulong
                            } else {
                                (if 1 as libc::c_int != 0 {
                                    0 as libc::c_int as libc::c_ulong
                                } else {
                                    o
                                })
                                    .wrapping_add(0 as libc::c_int as libc::c_ulong)
                            })
                                .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                < 0 as libc::c_int as libc::c_ulong
                            {
                                ((if 1 as libc::c_int != 0 {
                                    0 as libc::c_int as libc::c_ulong
                                } else {
                                    (if 1 as libc::c_int != 0 {
                                        0 as libc::c_int as libc::c_ulong
                                    } else {
                                        o
                                    })
                                        .wrapping_add(0 as libc::c_int as libc::c_ulong)
                                })
                                    .wrapping_add(1 as libc::c_int as libc::c_ulong)
                                    << (::core::mem::size_of::<libc::c_ulong>()
                                        as libc::c_ulong)
                                        .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                        .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                    .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                    .wrapping_mul(2 as libc::c_int as libc::c_ulong)
                                    .wrapping_add(1 as libc::c_int as libc::c_ulong)
                            } else {
                                (if 1 as libc::c_int != 0 {
                                    0 as libc::c_int as libc::c_ulong
                                } else {
                                    (if 1 as libc::c_int != 0 {
                                        0 as libc::c_int as libc::c_ulong
                                    } else {
                                        o
                                    })
                                        .wrapping_add(0 as libc::c_int as libc::c_ulong)
                                })
                                    .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                            })
                                .wrapping_neg()) as libc::c_int
                    } else {
                        ((0 as libc::c_int as libc::c_ulong)
                            < (if 1 as libc::c_int != 0 {
                                0 as libc::c_int as libc::c_ulong
                            } else {
                                o
                            })
                                .wrapping_add(0 as libc::c_int as libc::c_ulong))
                            as libc::c_int
                    }) != 0 && o == -(1 as libc::c_int) as libc::c_ulong
                    {
                        (if (if 1 as libc::c_int != 0 {
                            0 as libc::c_int as libc::c_ulong
                        } else {
                            n
                        })
                            .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                            < 0 as libc::c_int as libc::c_ulong
                        {
                            ((0 as libc::c_int as libc::c_ulong)
                                < n.wrapping_add(0 as libc::c_int as libc::c_ulong))
                                as libc::c_int
                        } else {
                            ((0 as libc::c_int as libc::c_ulong) < n
                                && ((-(1 as libc::c_int) - 0 as libc::c_int)
                                    as libc::c_ulong)
                                    < n.wrapping_sub(1 as libc::c_int as libc::c_ulong))
                                as libc::c_int
                        })
                    } else {
                        ((0 as libc::c_int as libc::c_ulong).wrapping_div(o) < n)
                            as libc::c_int
                    })
                })
            } else {
                (if o == 0 as libc::c_int as libc::c_ulong {
                    0 as libc::c_int
                } else {
                    (if n < 0 as libc::c_int as libc::c_ulong {
                        (if (if (if (if 1 as libc::c_int != 0 {
                            0 as libc::c_int as libc::c_ulong
                        } else {
                            (if 1 as libc::c_int != 0 {
                                0 as libc::c_int as libc::c_ulong
                            } else {
                                n
                            })
                                .wrapping_add(0 as libc::c_int as libc::c_ulong)
                        })
                            .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                            < 0 as libc::c_int as libc::c_ulong
                        {
                            !((if 1 as libc::c_int != 0 {
                                0 as libc::c_int as libc::c_ulong
                            } else {
                                (if 1 as libc::c_int != 0 {
                                    0 as libc::c_int as libc::c_ulong
                                } else {
                                    n
                                })
                                    .wrapping_add(0 as libc::c_int as libc::c_ulong)
                            })
                                .wrapping_add(1 as libc::c_int as libc::c_ulong)
                                << (::core::mem::size_of::<libc::c_ulong>()
                                    as libc::c_ulong)
                                    .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                    .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                .wrapping_mul(2 as libc::c_int as libc::c_ulong)
                                .wrapping_add(1 as libc::c_int as libc::c_ulong)
                        } else {
                            (if 1 as libc::c_int != 0 {
                                0 as libc::c_int as libc::c_ulong
                            } else {
                                (if 1 as libc::c_int != 0 {
                                    0 as libc::c_int as libc::c_ulong
                                } else {
                                    n
                                })
                                    .wrapping_add(0 as libc::c_int as libc::c_ulong)
                            })
                                .wrapping_add(0 as libc::c_int as libc::c_ulong)
                        }) < 0 as libc::c_int as libc::c_ulong
                        {
                            ((if 1 as libc::c_int != 0 {
                                0 as libc::c_int as libc::c_ulong
                            } else {
                                n
                            })
                                .wrapping_add(0 as libc::c_int as libc::c_ulong)
                                < (if (if 1 as libc::c_int != 0 {
                                    0 as libc::c_int as libc::c_ulong
                                } else {
                                    (if 1 as libc::c_int != 0 {
                                        0 as libc::c_int as libc::c_ulong
                                    } else {
                                        n
                                    })
                                        .wrapping_add(0 as libc::c_int as libc::c_ulong)
                                })
                                    .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                    < 0 as libc::c_int as libc::c_ulong
                                {
                                    ((if 1 as libc::c_int != 0 {
                                        0 as libc::c_int as libc::c_ulong
                                    } else {
                                        (if 1 as libc::c_int != 0 {
                                            0 as libc::c_int as libc::c_ulong
                                        } else {
                                            n
                                        })
                                            .wrapping_add(0 as libc::c_int as libc::c_ulong)
                                    })
                                        .wrapping_add(1 as libc::c_int as libc::c_ulong)
                                        << (::core::mem::size_of::<libc::c_ulong>()
                                            as libc::c_ulong)
                                            .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                            .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                        .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                        .wrapping_mul(2 as libc::c_int as libc::c_ulong)
                                        .wrapping_add(1 as libc::c_int as libc::c_ulong)
                                } else {
                                    (if 1 as libc::c_int != 0 {
                                        0 as libc::c_int as libc::c_ulong
                                    } else {
                                        (if 1 as libc::c_int != 0 {
                                            0 as libc::c_int as libc::c_ulong
                                        } else {
                                            n
                                        })
                                            .wrapping_add(0 as libc::c_int as libc::c_ulong)
                                    })
                                        .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                })
                                    .wrapping_neg()) as libc::c_int
                        } else {
                            ((0 as libc::c_int as libc::c_ulong)
                                < (if 1 as libc::c_int != 0 {
                                    0 as libc::c_int as libc::c_ulong
                                } else {
                                    n
                                })
                                    .wrapping_add(0 as libc::c_int as libc::c_ulong))
                                as libc::c_int
                        }) != 0 && n == -(1 as libc::c_int) as libc::c_ulong
                        {
                            (if (if 1 as libc::c_int != 0 {
                                0 as libc::c_int as libc::c_ulong
                            } else {
                                o
                            })
                                .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                < 0 as libc::c_int as libc::c_ulong
                            {
                                ((0 as libc::c_int as libc::c_ulong)
                                    < o.wrapping_add(0 as libc::c_int as libc::c_ulong))
                                    as libc::c_int
                            } else {
                                (((-(1 as libc::c_int) - 0 as libc::c_int) as libc::c_ulong)
                                    < o.wrapping_sub(1 as libc::c_int as libc::c_ulong))
                                    as libc::c_int
                            })
                        } else {
                            ((0 as libc::c_int as libc::c_ulong).wrapping_div(n) < o)
                                as libc::c_int
                        })
                    } else {
                        ((-(1 as libc::c_int) as intmax_t as libc::c_ulong)
                            .wrapping_div(o) < n) as libc::c_int
                    })
                })
            }) != 0
        {
            let (fresh5, fresh6) = n.overflowing_mul(o);
            *(&mut result as *mut intmax_t) = fresh5;
            1 as libc::c_int
        } else {
            let (fresh7, fresh8) = n.overflowing_mul(o);
            *(&mut result as *mut intmax_t) = fresh7;
            fresh8 as libc::c_int
        }) != 0
            || result != 0 as libc::c_int as libc::c_long
                && (e as libc::c_uint | f as libc::c_uint)
                    & LONGINT_OVERFLOW as libc::c_int as libc::c_uint != 0
        {
            e = LONGINT_OVERFLOW;
            result = 9223372036854775807 as libc::c_long;
        } else {
            if result == 0 as libc::c_int as libc::c_long
                && strncmp(
                    str,
                    b"0x\0" as *const u8 as *const libc::c_char,
                    strlen(b"0x\0" as *const u8 as *const libc::c_char),
                ) == 0 as libc::c_int
            {
                nl_error(
                    0 as libc::c_int,
                    0 as libc::c_int,
                    dcgettext(
                        0 as *const libc::c_char,
                        b"warning: %s is a zero multiplier; use %s if that is intended\0"
                            as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    quote_n(
                        0 as libc::c_int,
                        b"0x\0" as *const u8 as *const libc::c_char,
                    ),
                    quote_n(
                        1 as libc::c_int,
                        b"00x\0" as *const u8 as *const libc::c_char,
                    ),
                );
            }
            e = LONGINT_OK;
        }
    } else if n <= 9223372036854775807 as libc::c_long as libc::c_ulong {
        result = n as intmax_t;
    } else {
        e = LONGINT_OVERFLOW;
        result = 9223372036854775807 as libc::c_long;
    }
    *invalid = e;
    return result;
}
unsafe extern "C" fn operand_is(
    mut operand: *const libc::c_char,
    mut name: *const libc::c_char,
) -> bool {
    return operand_matches(operand, name, '=' as i32 as libc::c_char);
}
unsafe extern "C" fn scanargs(
    mut argc: libc::c_int,
    mut argv: *const *mut libc::c_char,
) {
    let mut blocksize: idx_t = 0 as libc::c_int as idx_t;
    let mut count: intmax_t = 9223372036854775807 as libc::c_long;
    let mut skip_0: intmax_t = 0 as libc::c_int as intmax_t;
    let mut seek: intmax_t = 0 as libc::c_int as intmax_t;
    let mut count_B: bool = 0 as libc::c_int != 0;
    let mut skip_B: bool = 0 as libc::c_int != 0;
    let mut seek_B: bool = 0 as libc::c_int != 0;
    let mut i: libc::c_int = optind;
    while i < argc {
        let mut name: *const libc::c_char = *argv.offset(i as isize);
        let mut val: *const libc::c_char = strchr(name, '=' as i32);
        if val.is_null() {
            nl_error(
                0 as libc::c_int,
                0 as libc::c_int,
                dcgettext(
                    0 as *const libc::c_char,
                    b"unrecognized operand %s\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
                quote(name),
            );
            usage(1 as libc::c_int);
        }
        val = val.offset(1);
        if operand_is(name, b"if\0" as *const u8 as *const libc::c_char) {
            input_file = val;
        } else if operand_is(name, b"of\0" as *const u8 as *const libc::c_char) {
            output_file = val;
        } else if operand_is(name, b"conv\0" as *const u8 as *const libc::c_char) {
            conversions_mask
                |= parse_symbols(
                    val,
                    conversions.as_ptr(),
                    0 as libc::c_int != 0,
                    b"invalid conversion\0" as *const u8 as *const libc::c_char,
                );
        } else if operand_is(name, b"iflag\0" as *const u8 as *const libc::c_char) {
            input_flags
                |= parse_symbols(
                    val,
                    flags.as_ptr(),
                    0 as libc::c_int != 0,
                    b"invalid input flag\0" as *const u8 as *const libc::c_char,
                );
        } else if operand_is(name, b"oflag\0" as *const u8 as *const libc::c_char) {
            output_flags
                |= parse_symbols(
                    val,
                    flags.as_ptr(),
                    0 as libc::c_int != 0,
                    b"invalid output flag\0" as *const u8 as *const libc::c_char,
                );
        } else if operand_is(name, b"status\0" as *const u8 as *const libc::c_char) {
            status_level = parse_symbols(
                val,
                statuses.as_ptr(),
                1 as libc::c_int != 0,
                b"invalid status level\0" as *const u8 as *const libc::c_char,
            );
        } else {
            let mut invalid: strtol_error = LONGINT_OK;
            let mut n: intmax_t = parse_integer(val, &mut invalid);
            let mut has_B: bool = !(strchr(val, 'B' as i32)).is_null();
            let mut n_min: intmax_t = 0 as libc::c_int as intmax_t;
            let mut n_max: intmax_t = 9223372036854775807 as libc::c_long;
            let mut converted_idx: *mut idx_t = 0 as *mut idx_t;
            let mut max_blocksize: idx_t = if (9223372036854775807 as libc::c_long
                - 1 as libc::c_int as libc::c_long)
                < (if (9223372036854775807 as libc::c_long)
                    < (if (0 as libc::c_int as off_t) < -(1 as libc::c_int) as off_t {
                        -(1 as libc::c_int) as off_t
                    } else {
                        (((1 as libc::c_int as off_t)
                            << (::core::mem::size_of::<off_t>() as libc::c_ulong)
                                .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                            - 1 as libc::c_int as libc::c_long)
                            * 2 as libc::c_int as libc::c_long
                            + 1 as libc::c_int as libc::c_long
                    })
                {
                    9223372036854775807 as libc::c_long
                } else {
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
                    })
                })
            {
                9223372036854775807 as libc::c_long - 1 as libc::c_int as libc::c_long
            } else if (9223372036854775807 as libc::c_long)
                < (if (0 as libc::c_int as off_t) < -(1 as libc::c_int) as off_t {
                    -(1 as libc::c_int) as off_t
                } else {
                    (((1 as libc::c_int as off_t)
                        << (::core::mem::size_of::<off_t>() as libc::c_ulong)
                            .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                            .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                        - 1 as libc::c_int as libc::c_long)
                        * 2 as libc::c_int as libc::c_long
                        + 1 as libc::c_int as libc::c_long
                })
            {
                9223372036854775807 as libc::c_long
            } else if (0 as libc::c_int as off_t) < -(1 as libc::c_int) as off_t {
                -(1 as libc::c_int) as off_t
            } else {
                (((1 as libc::c_int as off_t)
                    << (::core::mem::size_of::<off_t>() as libc::c_ulong)
                        .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                        .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                    - 1 as libc::c_int as libc::c_long)
                    * 2 as libc::c_int as libc::c_long + 1 as libc::c_int as libc::c_long
            };
            if operand_is(name, b"ibs\0" as *const u8 as *const libc::c_char) {
                n_min = 1 as libc::c_int as intmax_t;
                n_max = max_blocksize;
                converted_idx = &mut input_blocksize;
            } else if operand_is(name, b"obs\0" as *const u8 as *const libc::c_char) {
                n_min = 1 as libc::c_int as intmax_t;
                n_max = max_blocksize;
                converted_idx = &mut output_blocksize;
            } else if operand_is(name, b"bs\0" as *const u8 as *const libc::c_char) {
                n_min = 1 as libc::c_int as intmax_t;
                n_max = max_blocksize;
                converted_idx = &mut blocksize;
            } else if operand_is(name, b"cbs\0" as *const u8 as *const libc::c_char) {
                n_min = 1 as libc::c_int as intmax_t;
                n_max = (if (18446744073709551615 as libc::c_ulong)
                    < 9223372036854775807 as libc::c_long as libc::c_ulong
                {
                    18446744073709551615 as libc::c_ulong
                } else {
                    9223372036854775807 as libc::c_long as libc::c_ulong
                }) as intmax_t;
                converted_idx = &mut conversion_blocksize;
            } else if operand_is(name, b"skip\0" as *const u8 as *const libc::c_char)
                as libc::c_int != 0
                || operand_is(name, b"iseek\0" as *const u8 as *const libc::c_char)
                    as libc::c_int != 0
            {
                skip_0 = n;
                skip_B = has_B;
            } else if operand_is(
                name
                    .offset(
                        (*name as libc::c_int == 'o' as i32) as libc::c_int as isize,
                    ),
                b"seek\0" as *const u8 as *const libc::c_char,
            ) {
                seek = n;
                seek_B = has_B;
            } else if operand_is(name, b"count\0" as *const u8 as *const libc::c_char) {
                count = n;
                count_B = has_B;
            } else {
                nl_error(
                    0 as libc::c_int,
                    0 as libc::c_int,
                    dcgettext(
                        0 as *const libc::c_char,
                        b"unrecognized operand %s\0" as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    quote(name),
                );
                usage(1 as libc::c_int);
            }
            if n < n_min {
                invalid = LONGINT_INVALID;
            } else if n_max < n {
                invalid = LONGINT_OVERFLOW;
            }
            if invalid as libc::c_uint != LONGINT_OK as libc::c_int as libc::c_uint {
                if ::core::mem::size_of::<C2RustUnnamed_26>() as libc::c_ulong != 0 {
                    nl_error(
                        1 as libc::c_int,
                        (if invalid as libc::c_uint
                            == LONGINT_OVERFLOW as libc::c_int as libc::c_uint
                        {
                            75 as libc::c_int
                        } else {
                            0 as libc::c_int
                        }),
                        b"%s: %s\0" as *const u8 as *const libc::c_char,
                        dcgettext(
                            0 as *const libc::c_char,
                            b"invalid number\0" as *const u8 as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                        quote(val),
                    );
                    if 0 as libc::c_int != 0 {} else {
                        unreachable!();
                    };
                } else {
                    nl_error(
                        1 as libc::c_int,
                        (if invalid as libc::c_uint
                            == LONGINT_OVERFLOW as libc::c_int as libc::c_uint
                        {
                            75 as libc::c_int
                        } else {
                            0 as libc::c_int
                        }),
                        b"%s: %s\0" as *const u8 as *const libc::c_char,
                        dcgettext(
                            0 as *const libc::c_char,
                            b"invalid number\0" as *const u8 as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                        quote(val),
                    );
                    if 0 as libc::c_int != 0 {} else {
                        unreachable!();
                    };
                };
            } else if !converted_idx.is_null() {
                *converted_idx = n;
            }
        }
        i += 1;
    }
    if blocksize != 0 {
        output_blocksize = blocksize;
        input_blocksize = output_blocksize;
    } else {
        conversions_mask |= C_TWOBUFS as libc::c_int;
    }
    if input_blocksize == 0 as libc::c_int as libc::c_long {
        input_blocksize = 512 as libc::c_int as idx_t;
    }
    if output_blocksize == 0 as libc::c_int as libc::c_long {
        output_blocksize = 512 as libc::c_int as idx_t;
    }
    if conversion_blocksize == 0 as libc::c_int as libc::c_long {
        conversions_mask &= !(C_BLOCK as libc::c_int | C_UNBLOCK as libc::c_int);
    }
    if input_flags & (0o10000 as libc::c_int | 0o4010000 as libc::c_int) != 0 {
        input_flags |= 0o4010000 as libc::c_int;
    }
    if output_flags & O_FULLBLOCK as libc::c_int != 0 {
        nl_error(
            0 as libc::c_int,
            0 as libc::c_int,
            b"%s: %s\0" as *const u8 as *const libc::c_char,
            dcgettext(
                0 as *const libc::c_char,
                b"invalid output flag\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            quote(b"fullblock\0" as *const u8 as *const libc::c_char),
        );
        usage(1 as libc::c_int);
    }
    if skip_B {
        input_flags |= O_SKIP_BYTES as libc::c_int;
    }
    if input_flags & O_SKIP_BYTES as libc::c_int != 0
        && skip_0 != 0 as libc::c_int as libc::c_long
    {
        skip_records = skip_0 / input_blocksize;
        skip_bytes = skip_0 % input_blocksize;
    } else if skip_0 != 0 as libc::c_int as libc::c_long {
        skip_records = skip_0;
    }
    if count_B {
        input_flags |= O_COUNT_BYTES as libc::c_int;
    }
    if input_flags & O_COUNT_BYTES as libc::c_int != 0
        && count != 9223372036854775807 as libc::c_long
    {
        max_records = count / input_blocksize;
        max_bytes = count % input_blocksize;
    } else if count != 9223372036854775807 as libc::c_long {
        max_records = count;
    }
    if seek_B {
        output_flags |= O_SEEK_BYTES as libc::c_int;
    }
    if output_flags & O_SEEK_BYTES as libc::c_int != 0
        && seek != 0 as libc::c_int as libc::c_long
    {
        seek_records = seek / output_blocksize;
        seek_bytes = seek % output_blocksize;
    } else if seek != 0 as libc::c_int as libc::c_long {
        seek_records = seek;
    }
    warn_partial_read = conversions_mask & C_TWOBUFS as libc::c_int == 0
        && input_flags & O_FULLBLOCK as libc::c_int == 0
        && (skip_records != 0
            || (0 as libc::c_int as libc::c_long) < max_records
                && max_records < 9223372036854775807 as libc::c_long
            || (input_flags | output_flags) & 0o200000 as libc::c_int != 0);
    iread_fnc = if input_flags & O_FULLBLOCK as libc::c_int != 0 {
        Some(
            iread_fullblock
                as unsafe extern "C" fn(libc::c_int, *mut libc::c_char, idx_t) -> ssize_t,
        )
    } else {
        Some(
            iread
                as unsafe extern "C" fn(libc::c_int, *mut libc::c_char, idx_t) -> ssize_t,
        )
    };
    input_flags &= !(O_FULLBLOCK as libc::c_int);
    if multiple_bits_set(
        conversions_mask
            & (C_ASCII as libc::c_int | C_EBCDIC as libc::c_int | C_IBM as libc::c_int),
    ) {
        if ::core::mem::size_of::<C2RustUnnamed_25>() as libc::c_ulong != 0 {
            nl_error(
                1 as libc::c_int,
                0 as libc::c_int,
                dcgettext(
                    0 as *const libc::c_char,
                    b"cannot combine any two of {ascii,ebcdic,ibm}\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
            );
            if 0 as libc::c_int != 0 {} else {
                unreachable!();
            };
        } else {
            nl_error(
                1 as libc::c_int,
                0 as libc::c_int,
                dcgettext(
                    0 as *const libc::c_char,
                    b"cannot combine any two of {ascii,ebcdic,ibm}\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
            );
            if 0 as libc::c_int != 0 {} else {
                unreachable!();
            };
        };
    }
    if multiple_bits_set(
        conversions_mask & (C_BLOCK as libc::c_int | C_UNBLOCK as libc::c_int),
    ) {
        if ::core::mem::size_of::<C2RustUnnamed_24>() as libc::c_ulong != 0 {
            nl_error(
                1 as libc::c_int,
                0 as libc::c_int,
                dcgettext(
                    0 as *const libc::c_char,
                    b"cannot combine block and unblock\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
            );
            if 0 as libc::c_int != 0 {} else {
                unreachable!();
            };
        } else {
            nl_error(
                1 as libc::c_int,
                0 as libc::c_int,
                dcgettext(
                    0 as *const libc::c_char,
                    b"cannot combine block and unblock\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
            );
            if 0 as libc::c_int != 0 {} else {
                unreachable!();
            };
        };
    }
    if multiple_bits_set(
        conversions_mask & (C_LCASE as libc::c_int | C_UCASE as libc::c_int),
    ) {
        if ::core::mem::size_of::<C2RustUnnamed_23>() as libc::c_ulong != 0 {
            nl_error(
                1 as libc::c_int,
                0 as libc::c_int,
                dcgettext(
                    0 as *const libc::c_char,
                    b"cannot combine lcase and ucase\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
            );
            if 0 as libc::c_int != 0 {} else {
                unreachable!();
            };
        } else {
            nl_error(
                1 as libc::c_int,
                0 as libc::c_int,
                dcgettext(
                    0 as *const libc::c_char,
                    b"cannot combine lcase and ucase\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
            );
            if 0 as libc::c_int != 0 {} else {
                unreachable!();
            };
        };
    }
    if multiple_bits_set(
        conversions_mask & (C_EXCL as libc::c_int | C_NOCREAT as libc::c_int),
    ) {
        if ::core::mem::size_of::<C2RustUnnamed_22>() as libc::c_ulong != 0 {
            nl_error(
                1 as libc::c_int,
                0 as libc::c_int,
                dcgettext(
                    0 as *const libc::c_char,
                    b"cannot combine excl and nocreat\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
            );
            if 0 as libc::c_int != 0 {} else {
                unreachable!();
            };
        } else {
            nl_error(
                1 as libc::c_int,
                0 as libc::c_int,
                dcgettext(
                    0 as *const libc::c_char,
                    b"cannot combine excl and nocreat\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
            );
            if 0 as libc::c_int != 0 {} else {
                unreachable!();
            };
        };
    }
    if multiple_bits_set(
        input_flags & (0o200000 as libc::c_int | O_NOCACHE as libc::c_int),
    ) as libc::c_int != 0
        || multiple_bits_set(
            output_flags & (0o200000 as libc::c_int | O_NOCACHE as libc::c_int),
        ) as libc::c_int != 0
    {
        if ::core::mem::size_of::<C2RustUnnamed_21>() as libc::c_ulong != 0 {
            nl_error(
                1 as libc::c_int,
                0 as libc::c_int,
                dcgettext(
                    0 as *const libc::c_char,
                    b"cannot combine direct and nocache\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
            );
            if 0 as libc::c_int != 0 {} else {
                unreachable!();
            };
        } else {
            nl_error(
                1 as libc::c_int,
                0 as libc::c_int,
                dcgettext(
                    0 as *const libc::c_char,
                    b"cannot combine direct and nocache\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
            );
            if 0 as libc::c_int != 0 {} else {
                unreachable!();
            };
        };
    }
    if input_flags & O_NOCACHE as libc::c_int != 0 {
        i_nocache = 1 as libc::c_int != 0;
        i_nocache_eof = max_records == 0 as libc::c_int as libc::c_long
            && max_bytes == 0 as libc::c_int as libc::c_long;
        input_flags &= !(O_NOCACHE as libc::c_int);
    }
    if output_flags & O_NOCACHE as libc::c_int != 0 {
        o_nocache = 1 as libc::c_int != 0;
        o_nocache_eof = max_records == 0 as libc::c_int as libc::c_long
            && max_bytes == 0 as libc::c_int as libc::c_long;
        output_flags &= !(O_NOCACHE as libc::c_int);
    }
}
unsafe extern "C" fn apply_translations() {
    let mut i: libc::c_int = 0;
    if conversions_mask & C_ASCII as libc::c_int != 0 {
        translate_charset(ebcdic_to_ascii.as_ptr());
    }
    if conversions_mask & C_UCASE as libc::c_int != 0 {
        i = 0 as libc::c_int;
        while i < 256 as libc::c_int {
            trans_table[i
                as usize] = ({
                let mut __res: libc::c_int = 0;
                if ::core::mem::size_of::<libc::c_uchar>() as libc::c_ulong
                    > 1 as libc::c_int as libc::c_ulong
                {
                    if 0 != 0 {
                        let mut __c: libc::c_int = trans_table[i as usize]
                            as libc::c_int;
                        __res = if __c < -(128 as libc::c_int)
                            || __c > 255 as libc::c_int
                        {
                            __c
                        } else {
                            *(*__ctype_toupper_loc()).offset(__c as isize)
                        };
                    } else {
                        __res = toupper(trans_table[i as usize] as libc::c_int);
                    }
                } else {
                    __res = *(*__ctype_toupper_loc())
                        .offset(trans_table[i as usize] as libc::c_int as isize);
                }
                __res
            }) as libc::c_uchar;
            i += 1;
        }
        translation_needed = 1 as libc::c_int != 0;
    } else if conversions_mask & C_LCASE as libc::c_int != 0 {
        i = 0 as libc::c_int;
        while i < 256 as libc::c_int {
            trans_table[i
                as usize] = ({
                let mut __res: libc::c_int = 0;
                if ::core::mem::size_of::<libc::c_uchar>() as libc::c_ulong
                    > 1 as libc::c_int as libc::c_ulong
                {
                    if 0 != 0 {
                        let mut __c: libc::c_int = trans_table[i as usize]
                            as libc::c_int;
                        __res = if __c < -(128 as libc::c_int)
                            || __c > 255 as libc::c_int
                        {
                            __c
                        } else {
                            *(*__ctype_tolower_loc()).offset(__c as isize)
                        };
                    } else {
                        __res = tolower(trans_table[i as usize] as libc::c_int);
                    }
                } else {
                    __res = *(*__ctype_tolower_loc())
                        .offset(trans_table[i as usize] as libc::c_int as isize);
                }
                __res
            }) as libc::c_uchar;
            i += 1;
        }
        translation_needed = 1 as libc::c_int != 0;
    }
    if conversions_mask & C_EBCDIC as libc::c_int != 0 {
        translate_charset(ascii_to_ebcdic.as_ptr());
        newline_character = ascii_to_ebcdic['\n' as i32 as usize];
        space_character = ascii_to_ebcdic[' ' as i32 as usize];
    } else if conversions_mask & C_IBM as libc::c_int != 0 {
        translate_charset(ascii_to_ibm.as_ptr());
        newline_character = ascii_to_ibm['\n' as i32 as usize];
        space_character = ascii_to_ibm[' ' as i32 as usize];
    }
}
unsafe extern "C" fn translate_buffer(mut buf: *mut libc::c_char, mut nread: idx_t) {
    let mut i: idx_t = 0;
    let mut cp: *mut libc::c_char = 0 as *mut libc::c_char;
    i = nread;
    cp = buf;
    while i != 0 {
        *cp = trans_table[to_uchar(*cp) as usize] as libc::c_char;
        i -= 1;
        cp = cp.offset(1);
    }
}
unsafe extern "C" fn swab_buffer(
    mut buf: *mut libc::c_char,
    mut nread: *mut idx_t,
    mut saved_byte: *mut libc::c_int,
) -> *mut libc::c_char {
    if *nread == 0 as libc::c_int as libc::c_long {
        return buf;
    }
    let mut prev_saved: libc::c_int = *saved_byte;
    if (prev_saved < 0 as libc::c_int) as libc::c_int as libc::c_long
        == *nread & 1 as libc::c_int as libc::c_long
    {
        *nread -= 1;
        let mut c: libc::c_uchar = *buf.offset(*nread as isize) as libc::c_uchar;
        *saved_byte = c as libc::c_int;
    } else {
        *saved_byte = -(1 as libc::c_int);
    }
    let mut i: idx_t = *nread;
    while (1 as libc::c_int as libc::c_long) < i {
        *buf
            .offset(
                i as isize,
            ) = *buf.offset((i - 2 as libc::c_int as libc::c_long) as isize);
        i -= 2 as libc::c_int as libc::c_long;
    }
    if prev_saved < 0 as libc::c_int {
        return buf.offset(1 as libc::c_int as isize);
    }
    *buf.offset(1 as libc::c_int as isize) = prev_saved as libc::c_char;
    *nread += 1;
    return buf;
}
unsafe extern "C" fn advance_input_offset(mut offset: intmax_t) {
    if 0 as libc::c_int as libc::c_long <= input_offset
        && {
            let (fresh9, fresh10) = input_offset.overflowing_add(offset);
            *(&mut input_offset as *mut off_t) = fresh9;
            fresh10 as libc::c_int != 0
        }
    {
        input_offset = -(1 as libc::c_int) as off_t;
    }
}
unsafe extern "C" fn skip(
    mut fdesc: libc::c_int,
    mut file: *const libc::c_char,
    mut records: intmax_t,
    mut blocksize: idx_t,
    mut bytes: *mut idx_t,
) -> intmax_t {
    *__errno_location() = 0 as libc::c_int;
    let mut offset: off_t = 0;
    if (if (0 as libc::c_int as off_t) < -(1 as libc::c_int) as off_t
        && ((if 1 as libc::c_int != 0 {
            0 as libc::c_int as libc::c_long
        } else {
            records
        }) - 1 as libc::c_int as libc::c_long) < 0 as libc::c_int as libc::c_long
        && ((if 1 as libc::c_int != 0 {
            0 as libc::c_int as libc::c_long
        } else {
            blocksize
        }) - 1 as libc::c_int as libc::c_long) < 0 as libc::c_int as libc::c_long
        && (if blocksize < 0 as libc::c_int as libc::c_long {
            (if records < 0 as libc::c_int as libc::c_long {
                (if ((if 1 as libc::c_int != 0 {
                    0 as libc::c_int as libc::c_long
                } else {
                    (if 1 as libc::c_int != 0 {
                        0 as libc::c_int as libc::c_long
                    } else {
                        -(1 as libc::c_int) as off_t
                    }) + blocksize
                }) - 1 as libc::c_int as libc::c_long) < 0 as libc::c_int as libc::c_long
                {
                    (records < -(1 as libc::c_int) as off_t / blocksize) as libc::c_int
                } else {
                    ((if (if (if ((if 1 as libc::c_int != 0 {
                        0 as libc::c_int as libc::c_long
                    } else {
                        blocksize
                    }) - 1 as libc::c_int as libc::c_long)
                        < 0 as libc::c_int as libc::c_long
                    {
                        !(((((if 1 as libc::c_int != 0 {
                            0 as libc::c_int as libc::c_long
                        } else {
                            blocksize
                        }) + 1 as libc::c_int as libc::c_long)
                            << (::core::mem::size_of::<idx_t>() as libc::c_ulong)
                                .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                            - 1 as libc::c_int as libc::c_long)
                            * 2 as libc::c_int as libc::c_long
                            + 1 as libc::c_int as libc::c_long)
                    } else {
                        (if 1 as libc::c_int != 0 {
                            0 as libc::c_int as libc::c_long
                        } else {
                            blocksize
                        }) + 0 as libc::c_int as libc::c_long
                    }) < 0 as libc::c_int as libc::c_long
                    {
                        (blocksize
                            < -(if ((if 1 as libc::c_int != 0 {
                                0 as libc::c_int as libc::c_long
                            } else {
                                blocksize
                            }) - 1 as libc::c_int as libc::c_long)
                                < 0 as libc::c_int as libc::c_long
                            {
                                ((((if 1 as libc::c_int != 0 {
                                    0 as libc::c_int as libc::c_long
                                } else {
                                    blocksize
                                }) + 1 as libc::c_int as libc::c_long)
                                    << (::core::mem::size_of::<idx_t>() as libc::c_ulong)
                                        .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                        .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                    - 1 as libc::c_int as libc::c_long)
                                    * 2 as libc::c_int as libc::c_long
                                    + 1 as libc::c_int as libc::c_long
                            } else {
                                (if 1 as libc::c_int != 0 {
                                    0 as libc::c_int as libc::c_long
                                } else {
                                    blocksize
                                }) - 1 as libc::c_int as libc::c_long
                            })) as libc::c_int
                    } else {
                        ((0 as libc::c_int as libc::c_long) < blocksize) as libc::c_int
                    }) != 0
                    {
                        (if 1 as libc::c_int != 0 {
                            0 as libc::c_int as libc::c_long
                        } else {
                            blocksize
                        }) + -(1 as libc::c_int) as off_t
                            >> (::core::mem::size_of::<idx_t>() as libc::c_ulong)
                                .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                    } else {
                        -(1 as libc::c_int) as off_t / -blocksize
                    }) <= -(1 as libc::c_int) as libc::c_long - records) as libc::c_int
                })
            } else {
                (if (if (if ((if 1 as libc::c_int != 0 {
                    0 as libc::c_int as libc::c_long
                } else {
                    (if 1 as libc::c_int != 0 {
                        0 as libc::c_int as libc::c_long
                    } else {
                        blocksize
                    }) + 0 as libc::c_int as libc::c_long
                }) - 1 as libc::c_int as libc::c_long) < 0 as libc::c_int as libc::c_long
                {
                    !(((((if 1 as libc::c_int != 0 {
                        0 as libc::c_int as libc::c_long
                    } else {
                        (if 1 as libc::c_int != 0 {
                            0 as libc::c_int as libc::c_long
                        } else {
                            blocksize
                        }) + 0 as libc::c_int as libc::c_long
                    }) + 1 as libc::c_int as libc::c_long)
                        << (::core::mem::size_of::<libc::c_long>() as libc::c_ulong)
                            .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                            .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                        - 1 as libc::c_int as libc::c_long)
                        * 2 as libc::c_int as libc::c_long
                        + 1 as libc::c_int as libc::c_long)
                } else {
                    (if 1 as libc::c_int != 0 {
                        0 as libc::c_int as libc::c_long
                    } else {
                        (if 1 as libc::c_int != 0 {
                            0 as libc::c_int as libc::c_long
                        } else {
                            blocksize
                        }) + 0 as libc::c_int as libc::c_long
                    }) + 0 as libc::c_int as libc::c_long
                }) < 0 as libc::c_int as libc::c_long
                {
                    (((if 1 as libc::c_int != 0 {
                        0 as libc::c_int as libc::c_long
                    } else {
                        blocksize
                    }) + 0 as libc::c_int as libc::c_long)
                        < -(if ((if 1 as libc::c_int != 0 {
                            0 as libc::c_int as libc::c_long
                        } else {
                            (if 1 as libc::c_int != 0 {
                                0 as libc::c_int as libc::c_long
                            } else {
                                blocksize
                            }) + 0 as libc::c_int as libc::c_long
                        }) - 1 as libc::c_int as libc::c_long)
                            < 0 as libc::c_int as libc::c_long
                        {
                            ((((if 1 as libc::c_int != 0 {
                                0 as libc::c_int as libc::c_long
                            } else {
                                (if 1 as libc::c_int != 0 {
                                    0 as libc::c_int as libc::c_long
                                } else {
                                    blocksize
                                }) + 0 as libc::c_int as libc::c_long
                            }) + 1 as libc::c_int as libc::c_long)
                                << (::core::mem::size_of::<libc::c_long>() as libc::c_ulong)
                                    .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                    .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                - 1 as libc::c_int as libc::c_long)
                                * 2 as libc::c_int as libc::c_long
                                + 1 as libc::c_int as libc::c_long
                        } else {
                            (if 1 as libc::c_int != 0 {
                                0 as libc::c_int as libc::c_long
                            } else {
                                (if 1 as libc::c_int != 0 {
                                    0 as libc::c_int as libc::c_long
                                } else {
                                    blocksize
                                }) + 0 as libc::c_int as libc::c_long
                            }) - 1 as libc::c_int as libc::c_long
                        })) as libc::c_int
                } else {
                    ((0 as libc::c_int as libc::c_long)
                        < (if 1 as libc::c_int != 0 {
                            0 as libc::c_int as libc::c_long
                        } else {
                            blocksize
                        }) + 0 as libc::c_int as libc::c_long) as libc::c_int
                }) != 0 && blocksize == -(1 as libc::c_int) as libc::c_long
                {
                    (if ((if 1 as libc::c_int != 0 {
                        0 as libc::c_int as libc::c_long
                    } else {
                        records
                    }) - 1 as libc::c_int as libc::c_long)
                        < 0 as libc::c_int as libc::c_long
                    {
                        ((0 as libc::c_int as libc::c_long)
                            < records + 0 as libc::c_int as libc::c_long) as libc::c_int
                    } else {
                        ((0 as libc::c_int as libc::c_long) < records
                            && ((-(1 as libc::c_int) - 0 as libc::c_int) as libc::c_long)
                                < records - 1 as libc::c_int as libc::c_long) as libc::c_int
                    })
                } else {
                    (0 as libc::c_int as libc::c_long / blocksize < records)
                        as libc::c_int
                })
            })
        } else {
            (if blocksize == 0 as libc::c_int as libc::c_long {
                0 as libc::c_int
            } else {
                (if records < 0 as libc::c_int as libc::c_long {
                    (if (if (if ((if 1 as libc::c_int != 0 {
                        0 as libc::c_int as libc::c_long
                    } else {
                        (if 1 as libc::c_int != 0 {
                            0 as libc::c_int as libc::c_long
                        } else {
                            records
                        }) + 0 as libc::c_int as libc::c_long
                    }) - 1 as libc::c_int as libc::c_long)
                        < 0 as libc::c_int as libc::c_long
                    {
                        !(((((if 1 as libc::c_int != 0 {
                            0 as libc::c_int as libc::c_long
                        } else {
                            (if 1 as libc::c_int != 0 {
                                0 as libc::c_int as libc::c_long
                            } else {
                                records
                            }) + 0 as libc::c_int as libc::c_long
                        }) + 1 as libc::c_int as libc::c_long)
                            << (::core::mem::size_of::<libc::c_long>() as libc::c_ulong)
                                .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                            - 1 as libc::c_int as libc::c_long)
                            * 2 as libc::c_int as libc::c_long
                            + 1 as libc::c_int as libc::c_long)
                    } else {
                        (if 1 as libc::c_int != 0 {
                            0 as libc::c_int as libc::c_long
                        } else {
                            (if 1 as libc::c_int != 0 {
                                0 as libc::c_int as libc::c_long
                            } else {
                                records
                            }) + 0 as libc::c_int as libc::c_long
                        }) + 0 as libc::c_int as libc::c_long
                    }) < 0 as libc::c_int as libc::c_long
                    {
                        (((if 1 as libc::c_int != 0 {
                            0 as libc::c_int as libc::c_long
                        } else {
                            records
                        }) + 0 as libc::c_int as libc::c_long)
                            < -(if ((if 1 as libc::c_int != 0 {
                                0 as libc::c_int as libc::c_long
                            } else {
                                (if 1 as libc::c_int != 0 {
                                    0 as libc::c_int as libc::c_long
                                } else {
                                    records
                                }) + 0 as libc::c_int as libc::c_long
                            }) - 1 as libc::c_int as libc::c_long)
                                < 0 as libc::c_int as libc::c_long
                            {
                                ((((if 1 as libc::c_int != 0 {
                                    0 as libc::c_int as libc::c_long
                                } else {
                                    (if 1 as libc::c_int != 0 {
                                        0 as libc::c_int as libc::c_long
                                    } else {
                                        records
                                    }) + 0 as libc::c_int as libc::c_long
                                }) + 1 as libc::c_int as libc::c_long)
                                    << (::core::mem::size_of::<libc::c_long>() as libc::c_ulong)
                                        .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                        .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                    - 1 as libc::c_int as libc::c_long)
                                    * 2 as libc::c_int as libc::c_long
                                    + 1 as libc::c_int as libc::c_long
                            } else {
                                (if 1 as libc::c_int != 0 {
                                    0 as libc::c_int as libc::c_long
                                } else {
                                    (if 1 as libc::c_int != 0 {
                                        0 as libc::c_int as libc::c_long
                                    } else {
                                        records
                                    }) + 0 as libc::c_int as libc::c_long
                                }) - 1 as libc::c_int as libc::c_long
                            })) as libc::c_int
                    } else {
                        ((0 as libc::c_int as libc::c_long)
                            < (if 1 as libc::c_int != 0 {
                                0 as libc::c_int as libc::c_long
                            } else {
                                records
                            }) + 0 as libc::c_int as libc::c_long) as libc::c_int
                    }) != 0 && records == -(1 as libc::c_int) as libc::c_long
                    {
                        (if ((if 1 as libc::c_int != 0 {
                            0 as libc::c_int as libc::c_long
                        } else {
                            blocksize
                        }) - 1 as libc::c_int as libc::c_long)
                            < 0 as libc::c_int as libc::c_long
                        {
                            ((0 as libc::c_int as libc::c_long)
                                < blocksize + 0 as libc::c_int as libc::c_long)
                                as libc::c_int
                        } else {
                            (((-(1 as libc::c_int) - 0 as libc::c_int) as libc::c_long)
                                < blocksize - 1 as libc::c_int as libc::c_long)
                                as libc::c_int
                        })
                    } else {
                        (0 as libc::c_int as libc::c_long / records < blocksize)
                            as libc::c_int
                    })
                } else {
                    (-(1 as libc::c_int) as off_t / blocksize < records) as libc::c_int
                })
            })
        }) != 0
    {
        let (fresh11, fresh12) = records.overflowing_mul(blocksize);
        *(&mut offset as *mut off_t) = fresh11;
        1 as libc::c_int
    } else {
        let (fresh13, fresh14) = records.overflowing_mul(blocksize);
        *(&mut offset as *mut off_t) = fresh13;
        fresh14 as libc::c_int
    }) == 0
        && {
            let (fresh15, fresh16) = offset.overflowing_add(*bytes);
            *(&mut offset as *mut off_t) = fresh15;
            !fresh16
        } && 0 as libc::c_int as libc::c_long <= lseek(fdesc, offset, 1 as libc::c_int)
    {
        if fdesc == 0 as libc::c_int {
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
            if ifstat(0 as libc::c_int, &mut st) != 0 as libc::c_int {
                if ::core::mem::size_of::<C2RustUnnamed_27>() as libc::c_ulong != 0 {
                    nl_error(
                        1 as libc::c_int,
                        *__errno_location(),
                        dcgettext(
                            0 as *const libc::c_char,
                            b"cannot fstat %s\0" as *const u8 as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                        quotearg_style(shell_escape_always_quoting_style, file),
                    );
                    if 0 as libc::c_int != 0 {} else {
                        unreachable!();
                    };
                } else {
                    nl_error(
                        1 as libc::c_int,
                        *__errno_location(),
                        dcgettext(
                            0 as *const libc::c_char,
                            b"cannot fstat %s\0" as *const u8 as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                        quotearg_style(shell_escape_always_quoting_style, file),
                    );
                    if 0 as libc::c_int != 0 {} else {
                        unreachable!();
                    };
                };
            }
            if usable_st_size(&mut st) as libc::c_int != 0
                && 0 as libc::c_int as libc::c_long <= input_offset
                && st.st_size - input_offset < offset
            {
                records = (offset - st.st_size) / blocksize;
                offset = st.st_size - input_offset;
            } else {
                records = 0 as libc::c_int as intmax_t;
            }
            advance_input_offset(offset);
        } else {
            records = 0 as libc::c_int as intmax_t;
            *bytes = 0 as libc::c_int as idx_t;
        }
        return records;
    } else {
        let mut lseek_errno: libc::c_int = *__errno_location();
        if lseek(fdesc, 0 as libc::c_int as __off_t, 2 as libc::c_int)
            >= 0 as libc::c_int as libc::c_long
        {
            if lseek_errno == 0 {
                lseek_errno = 75 as libc::c_int;
            }
            if fdesc == 0 as libc::c_int {
                nl_error(
                    0 as libc::c_int,
                    lseek_errno,
                    dcgettext(
                        0 as *const libc::c_char,
                        b"%s: cannot skip\0" as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    quotearg_n_style_colon(
                        0 as libc::c_int,
                        shell_escape_quoting_style,
                        file,
                    ),
                );
            } else {
                nl_error(
                    0 as libc::c_int,
                    lseek_errno,
                    dcgettext(
                        0 as *const libc::c_char,
                        b"%s: cannot seek\0" as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    quotearg_n_style_colon(
                        0 as libc::c_int,
                        shell_escape_quoting_style,
                        file,
                    ),
                );
            }
            quit(1 as libc::c_int);
        }
        let mut buf: *mut libc::c_char = 0 as *mut libc::c_char;
        if fdesc == 0 as libc::c_int {
            alloc_ibuf();
            buf = ibuf;
        } else {
            alloc_obuf();
            buf = obuf;
        }
        loop {
            let mut nread: ssize_t = iread_fnc
                .expect(
                    "non-null function pointer",
                )(fdesc, buf, if records != 0 { blocksize } else { *bytes });
            if nread < 0 as libc::c_int as libc::c_long {
                if fdesc == 0 as libc::c_int {
                    nl_error(
                        0 as libc::c_int,
                        *__errno_location(),
                        dcgettext(
                            0 as *const libc::c_char,
                            b"error reading %s\0" as *const u8 as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                        quotearg_style(shell_escape_always_quoting_style, file),
                    );
                    if conversions_mask & C_NOERROR as libc::c_int != 0 {
                        print_stats();
                    }
                } else {
                    nl_error(
                        0 as libc::c_int,
                        lseek_errno,
                        dcgettext(
                            0 as *const libc::c_char,
                            b"%s: cannot seek\0" as *const u8 as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                        quotearg_n_style_colon(
                            0 as libc::c_int,
                            shell_escape_quoting_style,
                            file,
                        ),
                    );
                }
                quit(1 as libc::c_int);
            } else {
                if nread == 0 as libc::c_int as libc::c_long {
                    break;
                }
                if fdesc == 0 as libc::c_int {
                    advance_input_offset(nread);
                }
            }
            if records != 0 as libc::c_int as libc::c_long {
                records -= 1;
            } else {
                *bytes = 0 as libc::c_int as idx_t;
            }
            if !(records != 0 || *bytes != 0) {
                break;
            }
        }
        return records;
    };
}
unsafe extern "C" fn advance_input_after_read_error(mut nbytes: idx_t) -> bool {
    if !input_seekable {
        if input_seek_errno == 29 as libc::c_int {
            return 1 as libc::c_int != 0;
        }
        *__errno_location() = input_seek_errno;
    } else {
        let mut offset: off_t = 0;
        advance_input_offset(nbytes);
        if input_offset < 0 as libc::c_int as libc::c_long {
            nl_error(
                0 as libc::c_int,
                0 as libc::c_int,
                dcgettext(
                    0 as *const libc::c_char,
                    b"offset overflow while reading file %s\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
                quotearg_style(shell_escape_always_quoting_style, input_file),
            );
            return 0 as libc::c_int != 0;
        }
        offset = lseek(0 as libc::c_int, 0 as libc::c_int as __off_t, 1 as libc::c_int);
        if 0 as libc::c_int as libc::c_long <= offset {
            let mut diff: off_t = 0;
            if offset == input_offset {
                return 1 as libc::c_int != 0;
            }
            diff = input_offset - offset;
            if !(0 as libc::c_int as libc::c_long <= diff && diff <= nbytes)
                && status_level != STATUS_NONE as libc::c_int
            {
                nl_error(
                    0 as libc::c_int,
                    0 as libc::c_int,
                    dcgettext(
                        0 as *const libc::c_char,
                        b"warning: invalid file offset after failed read\0" as *const u8
                            as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                );
            }
            if 0 as libc::c_int as libc::c_long
                <= lseek(0 as libc::c_int, diff, 1 as libc::c_int)
            {
                return 1 as libc::c_int != 0;
            }
            if *__errno_location() == 0 as libc::c_int {
                nl_error(
                    0 as libc::c_int,
                    0 as libc::c_int,
                    dcgettext(
                        0 as *const libc::c_char,
                        b"cannot work around kernel bug after all\0" as *const u8
                            as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                );
            }
        }
    }
    nl_error(
        0 as libc::c_int,
        *__errno_location(),
        dcgettext(
            0 as *const libc::c_char,
            b"%s: cannot seek\0" as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        ),
        quotearg_n_style_colon(0 as libc::c_int, shell_escape_quoting_style, input_file),
    );
    return 0 as libc::c_int != 0;
}
unsafe extern "C" fn copy_simple(mut buf: *const libc::c_char, mut nread: idx_t) {
    let mut start: *const libc::c_char = buf;
    loop {
        let mut nfree: idx_t = if nread < output_blocksize - oc {
            nread
        } else {
            output_blocksize - oc
        };
        memcpy(
            obuf.offset(oc as isize) as *mut libc::c_void,
            start as *const libc::c_void,
            nfree as libc::c_ulong,
        );
        nread -= nfree;
        start = start.offset(nfree as isize);
        oc += nfree;
        if oc >= output_blocksize {
            write_output();
        }
        if !(nread != 0 as libc::c_int as libc::c_long) {
            break;
        }
    };
}
unsafe extern "C" fn copy_with_block(mut buf: *const libc::c_char, mut nread: idx_t) {
    let mut i: idx_t = nread;
    while i != 0 {
        if *buf as libc::c_int == newline_character as libc::c_int {
            if col < conversion_blocksize {
                let mut j: idx_t = 0;
                j = col;
                while j < conversion_blocksize {
                    let fresh17 = oc;
                    oc = oc + 1;
                    *obuf.offset(fresh17 as isize) = space_character;
                    if oc >= output_blocksize {
                        write_output();
                    }
                    j += 1;
                }
            }
            col = 0 as libc::c_int as idx_t;
        } else {
            if col == conversion_blocksize {
                r_truncate += 1;
            } else if col < conversion_blocksize {
                let fresh18 = oc;
                oc = oc + 1;
                *obuf.offset(fresh18 as isize) = *buf;
                if oc >= output_blocksize {
                    write_output();
                }
            }
            col += 1;
        }
        i -= 1;
        buf = buf.offset(1);
    }
}
unsafe extern "C" fn copy_with_unblock(mut buf: *const libc::c_char, mut nread: idx_t) {
    static mut pending_spaces: idx_t = 0 as libc::c_int as idx_t;
    let mut i: idx_t = 0 as libc::c_int as idx_t;
    while i < nread {
        let mut c: libc::c_char = *buf.offset(i as isize);
        let fresh19 = col;
        col = col + 1;
        if fresh19 >= conversion_blocksize {
            pending_spaces = 0 as libc::c_int as idx_t;
            col = pending_spaces;
            i -= 1;
            let fresh20 = oc;
            oc = oc + 1;
            *obuf.offset(fresh20 as isize) = newline_character;
            if oc >= output_blocksize {
                write_output();
            }
        } else if c as libc::c_int == space_character as libc::c_int {
            pending_spaces += 1;
        } else {
            while pending_spaces != 0 {
                let fresh21 = oc;
                oc = oc + 1;
                *obuf.offset(fresh21 as isize) = space_character;
                if oc >= output_blocksize {
                    write_output();
                }
                pending_spaces -= 1;
            }
            let fresh22 = oc;
            oc = oc + 1;
            *obuf.offset(fresh22 as isize) = c;
            if oc >= output_blocksize {
                write_output();
            }
        }
        i += 1;
    }
}
unsafe extern "C" fn set_fd_flags(
    mut fd: libc::c_int,
    mut add_flags: libc::c_int,
    mut name: *const libc::c_char,
) {
    add_flags &= !(0o400 as libc::c_int | 0o100000 as libc::c_int);
    if add_flags != 0 {
        let mut old_flags: libc::c_int = rpl_fcntl(fd, 3 as libc::c_int);
        let mut new_flags: libc::c_int = old_flags | add_flags;
        let mut ok: bool = 1 as libc::c_int != 0;
        if old_flags < 0 as libc::c_int {
            ok = 0 as libc::c_int != 0;
        } else if old_flags != new_flags {
            if new_flags & (0o40000 as libc::c_int | 0 as libc::c_int) != 0 {
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
                if ifstat(fd, &mut st) != 0 as libc::c_int {
                    ok = 0 as libc::c_int != 0;
                } else if new_flags & 0o40000 as libc::c_int != 0
                    && !(st.st_mode & 0o170000 as libc::c_int as libc::c_uint
                        == 0o40000 as libc::c_int as libc::c_uint)
                {
                    *__errno_location() = 20 as libc::c_int;
                    ok = 0 as libc::c_int != 0;
                } else if new_flags & 0 as libc::c_int != 0
                    && (1 as libc::c_int as libc::c_uint) < st.st_nlink
                {
                    *__errno_location() = 31 as libc::c_int;
                    ok = 0 as libc::c_int != 0;
                }
                new_flags &= !(0o40000 as libc::c_int | 0 as libc::c_int);
            }
            if ok as libc::c_int != 0 && old_flags != new_flags
                && rpl_fcntl(fd, 4 as libc::c_int, new_flags) == -(1 as libc::c_int)
            {
                ok = 0 as libc::c_int != 0;
            }
        }
        if !ok {
            if ::core::mem::size_of::<C2RustUnnamed_28>() as libc::c_ulong != 0 {
                nl_error(
                    1 as libc::c_int,
                    *__errno_location(),
                    dcgettext(
                        0 as *const libc::c_char,
                        b"setting flags for %s\0" as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    quotearg_style(shell_escape_always_quoting_style, name),
                );
                if 0 as libc::c_int != 0 {} else {
                    unreachable!();
                };
            } else {
                nl_error(
                    1 as libc::c_int,
                    *__errno_location(),
                    dcgettext(
                        0 as *const libc::c_char,
                        b"setting flags for %s\0" as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    quotearg_style(shell_escape_always_quoting_style, name),
                );
                if 0 as libc::c_int != 0 {} else {
                    unreachable!();
                };
            };
        }
    }
}
unsafe extern "C" fn dd_copy() -> libc::c_int {
    let mut bufstart: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut nread: ssize_t = 0;
    let mut partread: idx_t = 0 as libc::c_int as idx_t;
    let mut exit_status: libc::c_int = 0 as libc::c_int;
    let mut n_bytes_read: idx_t = 0;
    if skip_records != 0 as libc::c_int as libc::c_long
        || skip_bytes != 0 as libc::c_int as libc::c_long
    {
        let mut us_bytes: intmax_t = 0;
        let mut us_bytes_overflow: bool = (if (0 as libc::c_int as intmax_t)
            < -(1 as libc::c_int) as intmax_t
            && ((if 1 as libc::c_int != 0 {
                0 as libc::c_int as libc::c_long
            } else {
                skip_records
            }) - 1 as libc::c_int as libc::c_long) < 0 as libc::c_int as libc::c_long
            && ((if 1 as libc::c_int != 0 {
                0 as libc::c_int as libc::c_long
            } else {
                input_blocksize
            }) - 1 as libc::c_int as libc::c_long) < 0 as libc::c_int as libc::c_long
            && (if input_blocksize < 0 as libc::c_int as libc::c_long {
                (if skip_records < 0 as libc::c_int as libc::c_long {
                    (if ((if 1 as libc::c_int != 0 {
                        0 as libc::c_int as libc::c_long
                    } else {
                        (if 1 as libc::c_int != 0 {
                            0 as libc::c_int as libc::c_long
                        } else {
                            -(1 as libc::c_int) as intmax_t
                        }) + input_blocksize
                    }) - 1 as libc::c_int as libc::c_long)
                        < 0 as libc::c_int as libc::c_long
                    {
                        (skip_records
                            < -(1 as libc::c_int) as intmax_t / input_blocksize)
                            as libc::c_int
                    } else {
                        ((if (if (if ((if 1 as libc::c_int != 0 {
                            0 as libc::c_int as libc::c_long
                        } else {
                            input_blocksize
                        }) - 1 as libc::c_int as libc::c_long)
                            < 0 as libc::c_int as libc::c_long
                        {
                            !(((((if 1 as libc::c_int != 0 {
                                0 as libc::c_int as libc::c_long
                            } else {
                                input_blocksize
                            }) + 1 as libc::c_int as libc::c_long)
                                << (::core::mem::size_of::<idx_t>() as libc::c_ulong)
                                    .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                    .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                - 1 as libc::c_int as libc::c_long)
                                * 2 as libc::c_int as libc::c_long
                                + 1 as libc::c_int as libc::c_long)
                        } else {
                            (if 1 as libc::c_int != 0 {
                                0 as libc::c_int as libc::c_long
                            } else {
                                input_blocksize
                            }) + 0 as libc::c_int as libc::c_long
                        }) < 0 as libc::c_int as libc::c_long
                        {
                            (input_blocksize
                                < -(if ((if 1 as libc::c_int != 0 {
                                    0 as libc::c_int as libc::c_long
                                } else {
                                    input_blocksize
                                }) - 1 as libc::c_int as libc::c_long)
                                    < 0 as libc::c_int as libc::c_long
                                {
                                    ((((if 1 as libc::c_int != 0 {
                                        0 as libc::c_int as libc::c_long
                                    } else {
                                        input_blocksize
                                    }) + 1 as libc::c_int as libc::c_long)
                                        << (::core::mem::size_of::<idx_t>() as libc::c_ulong)
                                            .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                            .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                        - 1 as libc::c_int as libc::c_long)
                                        * 2 as libc::c_int as libc::c_long
                                        + 1 as libc::c_int as libc::c_long
                                } else {
                                    (if 1 as libc::c_int != 0 {
                                        0 as libc::c_int as libc::c_long
                                    } else {
                                        input_blocksize
                                    }) - 1 as libc::c_int as libc::c_long
                                })) as libc::c_int
                        } else {
                            ((0 as libc::c_int as libc::c_long) < input_blocksize)
                                as libc::c_int
                        }) != 0
                        {
                            (if 1 as libc::c_int != 0 {
                                0 as libc::c_int as libc::c_long
                            } else {
                                input_blocksize
                            }) + -(1 as libc::c_int) as intmax_t
                                >> (::core::mem::size_of::<idx_t>() as libc::c_ulong)
                                    .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                    .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                        } else {
                            -(1 as libc::c_int) as intmax_t / -input_blocksize
                        }) <= -(1 as libc::c_int) as libc::c_long - skip_records)
                            as libc::c_int
                    })
                } else {
                    (if (if (if ((if 1 as libc::c_int != 0 {
                        0 as libc::c_int as libc::c_long
                    } else {
                        (if 1 as libc::c_int != 0 {
                            0 as libc::c_int as libc::c_long
                        } else {
                            input_blocksize
                        }) + 0 as libc::c_int as libc::c_long
                    }) - 1 as libc::c_int as libc::c_long)
                        < 0 as libc::c_int as libc::c_long
                    {
                        !(((((if 1 as libc::c_int != 0 {
                            0 as libc::c_int as libc::c_long
                        } else {
                            (if 1 as libc::c_int != 0 {
                                0 as libc::c_int as libc::c_long
                            } else {
                                input_blocksize
                            }) + 0 as libc::c_int as libc::c_long
                        }) + 1 as libc::c_int as libc::c_long)
                            << (::core::mem::size_of::<libc::c_long>() as libc::c_ulong)
                                .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                            - 1 as libc::c_int as libc::c_long)
                            * 2 as libc::c_int as libc::c_long
                            + 1 as libc::c_int as libc::c_long)
                    } else {
                        (if 1 as libc::c_int != 0 {
                            0 as libc::c_int as libc::c_long
                        } else {
                            (if 1 as libc::c_int != 0 {
                                0 as libc::c_int as libc::c_long
                            } else {
                                input_blocksize
                            }) + 0 as libc::c_int as libc::c_long
                        }) + 0 as libc::c_int as libc::c_long
                    }) < 0 as libc::c_int as libc::c_long
                    {
                        (((if 1 as libc::c_int != 0 {
                            0 as libc::c_int as libc::c_long
                        } else {
                            input_blocksize
                        }) + 0 as libc::c_int as libc::c_long)
                            < -(if ((if 1 as libc::c_int != 0 {
                                0 as libc::c_int as libc::c_long
                            } else {
                                (if 1 as libc::c_int != 0 {
                                    0 as libc::c_int as libc::c_long
                                } else {
                                    input_blocksize
                                }) + 0 as libc::c_int as libc::c_long
                            }) - 1 as libc::c_int as libc::c_long)
                                < 0 as libc::c_int as libc::c_long
                            {
                                ((((if 1 as libc::c_int != 0 {
                                    0 as libc::c_int as libc::c_long
                                } else {
                                    (if 1 as libc::c_int != 0 {
                                        0 as libc::c_int as libc::c_long
                                    } else {
                                        input_blocksize
                                    }) + 0 as libc::c_int as libc::c_long
                                }) + 1 as libc::c_int as libc::c_long)
                                    << (::core::mem::size_of::<libc::c_long>() as libc::c_ulong)
                                        .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                        .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                    - 1 as libc::c_int as libc::c_long)
                                    * 2 as libc::c_int as libc::c_long
                                    + 1 as libc::c_int as libc::c_long
                            } else {
                                (if 1 as libc::c_int != 0 {
                                    0 as libc::c_int as libc::c_long
                                } else {
                                    (if 1 as libc::c_int != 0 {
                                        0 as libc::c_int as libc::c_long
                                    } else {
                                        input_blocksize
                                    }) + 0 as libc::c_int as libc::c_long
                                }) - 1 as libc::c_int as libc::c_long
                            })) as libc::c_int
                    } else {
                        ((0 as libc::c_int as libc::c_long)
                            < (if 1 as libc::c_int != 0 {
                                0 as libc::c_int as libc::c_long
                            } else {
                                input_blocksize
                            }) + 0 as libc::c_int as libc::c_long) as libc::c_int
                    }) != 0 && input_blocksize == -(1 as libc::c_int) as libc::c_long
                    {
                        (if ((if 1 as libc::c_int != 0 {
                            0 as libc::c_int as libc::c_long
                        } else {
                            skip_records
                        }) - 1 as libc::c_int as libc::c_long)
                            < 0 as libc::c_int as libc::c_long
                        {
                            ((0 as libc::c_int as libc::c_long)
                                < skip_records + 0 as libc::c_int as libc::c_long)
                                as libc::c_int
                        } else {
                            ((0 as libc::c_int as libc::c_long) < skip_records
                                && ((-(1 as libc::c_int) - 0 as libc::c_int)
                                    as libc::c_long)
                                    < skip_records - 1 as libc::c_int as libc::c_long)
                                as libc::c_int
                        })
                    } else {
                        (0 as libc::c_int as libc::c_long / input_blocksize
                            < skip_records) as libc::c_int
                    })
                })
            } else {
                (if input_blocksize == 0 as libc::c_int as libc::c_long {
                    0 as libc::c_int
                } else {
                    (if skip_records < 0 as libc::c_int as libc::c_long {
                        (if (if (if ((if 1 as libc::c_int != 0 {
                            0 as libc::c_int as libc::c_long
                        } else {
                            (if 1 as libc::c_int != 0 {
                                0 as libc::c_int as libc::c_long
                            } else {
                                skip_records
                            }) + 0 as libc::c_int as libc::c_long
                        }) - 1 as libc::c_int as libc::c_long)
                            < 0 as libc::c_int as libc::c_long
                        {
                            !(((((if 1 as libc::c_int != 0 {
                                0 as libc::c_int as libc::c_long
                            } else {
                                (if 1 as libc::c_int != 0 {
                                    0 as libc::c_int as libc::c_long
                                } else {
                                    skip_records
                                }) + 0 as libc::c_int as libc::c_long
                            }) + 1 as libc::c_int as libc::c_long)
                                << (::core::mem::size_of::<libc::c_long>() as libc::c_ulong)
                                    .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                    .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                - 1 as libc::c_int as libc::c_long)
                                * 2 as libc::c_int as libc::c_long
                                + 1 as libc::c_int as libc::c_long)
                        } else {
                            (if 1 as libc::c_int != 0 {
                                0 as libc::c_int as libc::c_long
                            } else {
                                (if 1 as libc::c_int != 0 {
                                    0 as libc::c_int as libc::c_long
                                } else {
                                    skip_records
                                }) + 0 as libc::c_int as libc::c_long
                            }) + 0 as libc::c_int as libc::c_long
                        }) < 0 as libc::c_int as libc::c_long
                        {
                            (((if 1 as libc::c_int != 0 {
                                0 as libc::c_int as libc::c_long
                            } else {
                                skip_records
                            }) + 0 as libc::c_int as libc::c_long)
                                < -(if ((if 1 as libc::c_int != 0 {
                                    0 as libc::c_int as libc::c_long
                                } else {
                                    (if 1 as libc::c_int != 0 {
                                        0 as libc::c_int as libc::c_long
                                    } else {
                                        skip_records
                                    }) + 0 as libc::c_int as libc::c_long
                                }) - 1 as libc::c_int as libc::c_long)
                                    < 0 as libc::c_int as libc::c_long
                                {
                                    ((((if 1 as libc::c_int != 0 {
                                        0 as libc::c_int as libc::c_long
                                    } else {
                                        (if 1 as libc::c_int != 0 {
                                            0 as libc::c_int as libc::c_long
                                        } else {
                                            skip_records
                                        }) + 0 as libc::c_int as libc::c_long
                                    }) + 1 as libc::c_int as libc::c_long)
                                        << (::core::mem::size_of::<libc::c_long>() as libc::c_ulong)
                                            .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                            .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                        - 1 as libc::c_int as libc::c_long)
                                        * 2 as libc::c_int as libc::c_long
                                        + 1 as libc::c_int as libc::c_long
                                } else {
                                    (if 1 as libc::c_int != 0 {
                                        0 as libc::c_int as libc::c_long
                                    } else {
                                        (if 1 as libc::c_int != 0 {
                                            0 as libc::c_int as libc::c_long
                                        } else {
                                            skip_records
                                        }) + 0 as libc::c_int as libc::c_long
                                    }) - 1 as libc::c_int as libc::c_long
                                })) as libc::c_int
                        } else {
                            ((0 as libc::c_int as libc::c_long)
                                < (if 1 as libc::c_int != 0 {
                                    0 as libc::c_int as libc::c_long
                                } else {
                                    skip_records
                                }) + 0 as libc::c_int as libc::c_long) as libc::c_int
                        }) != 0 && skip_records == -(1 as libc::c_int) as libc::c_long
                        {
                            (if ((if 1 as libc::c_int != 0 {
                                0 as libc::c_int as libc::c_long
                            } else {
                                input_blocksize
                            }) - 1 as libc::c_int as libc::c_long)
                                < 0 as libc::c_int as libc::c_long
                            {
                                ((0 as libc::c_int as libc::c_long)
                                    < input_blocksize + 0 as libc::c_int as libc::c_long)
                                    as libc::c_int
                            } else {
                                (((-(1 as libc::c_int) - 0 as libc::c_int) as libc::c_long)
                                    < input_blocksize - 1 as libc::c_int as libc::c_long)
                                    as libc::c_int
                            })
                        } else {
                            (0 as libc::c_int as libc::c_long / skip_records
                                < input_blocksize) as libc::c_int
                        })
                    } else {
                        (-(1 as libc::c_int) as intmax_t / input_blocksize
                            < skip_records) as libc::c_int
                    })
                })
            }) != 0
        {
            let (fresh29, fresh30) = skip_records.overflowing_mul(input_blocksize);
            *(&mut us_bytes as *mut intmax_t) = fresh29;
            1 as libc::c_int
        } else {
            let (fresh31, fresh32) = skip_records.overflowing_mul(input_blocksize);
            *(&mut us_bytes as *mut intmax_t) = fresh31;
            fresh32 as libc::c_int
        }) != 0
            || {
                let (fresh33, fresh34) = skip_bytes.overflowing_add(us_bytes);
                *(&mut us_bytes as *mut intmax_t) = fresh33;
                fresh34 as libc::c_int != 0
            };
        let mut input_offset0: off_t = input_offset;
        let mut us_blocks: intmax_t = skip(
            0 as libc::c_int,
            input_file,
            skip_records,
            input_blocksize,
            &mut skip_bytes,
        );
        if (us_blocks != 0
            || 0 as libc::c_int as libc::c_long <= input_offset
                && (us_bytes_overflow as libc::c_int != 0
                    || us_bytes != input_offset - input_offset0))
            && status_level != STATUS_NONE as libc::c_int
        {
            nl_error(
                0 as libc::c_int,
                0 as libc::c_int,
                dcgettext(
                    0 as *const libc::c_char,
                    b"%s: cannot skip to specified offset\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
                quotearg_n_style_colon(
                    0 as libc::c_int,
                    shell_escape_quoting_style,
                    input_file,
                ),
            );
        }
    }
    if seek_records != 0 as libc::c_int as libc::c_long
        || seek_bytes != 0 as libc::c_int as libc::c_long
    {
        let mut bytes: idx_t = seek_bytes;
        let mut write_records: intmax_t = skip(
            1 as libc::c_int,
            output_file,
            seek_records,
            output_blocksize,
            &mut bytes,
        );
        if write_records != 0 as libc::c_int as libc::c_long
            || bytes != 0 as libc::c_int as libc::c_long
        {
            memset(
                obuf as *mut libc::c_void,
                0 as libc::c_int,
                (if write_records != 0 { output_blocksize } else { bytes })
                    as libc::c_ulong,
            );
            loop {
                let mut size: idx_t = if write_records != 0 {
                    output_blocksize
                } else {
                    bytes
                };
                if iwrite(1 as libc::c_int, obuf, size) != size {
                    nl_error(
                        0 as libc::c_int,
                        *__errno_location(),
                        dcgettext(
                            0 as *const libc::c_char,
                            b"writing to %s\0" as *const u8 as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                        quotearg_style(shell_escape_always_quoting_style, output_file),
                    );
                    quit(1 as libc::c_int);
                }
                if write_records != 0 as libc::c_int as libc::c_long {
                    write_records -= 1;
                } else {
                    bytes = 0 as libc::c_int as idx_t;
                }
                if !(write_records != 0 || bytes != 0) {
                    break;
                }
            }
        }
    }
    if max_records == 0 as libc::c_int as libc::c_long
        && max_bytes == 0 as libc::c_int as libc::c_long
    {
        return exit_status;
    }
    alloc_ibuf();
    alloc_obuf();
    let mut saved_byte: libc::c_int = -(1 as libc::c_int);
    loop {
        if status_level == STATUS_PROGRESS as libc::c_int {
            let mut progress_time: xtime_t = gethrxtime();
            if next_time <= progress_time {
                print_xfer_stats(progress_time);
                next_time += 1000000000 as libc::c_int as libc::c_longlong;
            }
        }
        if r_partial + r_full
            >= max_records + (max_bytes != 0) as libc::c_int as libc::c_long
        {
            break;
        }
        if conversions_mask & C_SYNC as libc::c_int != 0
            && conversions_mask & C_NOERROR as libc::c_int != 0
        {
            memset(
                ibuf as *mut libc::c_void,
                if conversions_mask & (C_BLOCK as libc::c_int | C_UNBLOCK as libc::c_int)
                    != 0
                {
                    ' ' as i32
                } else {
                    '\0' as i32
                },
                input_blocksize as libc::c_ulong,
            );
        }
        if r_partial + r_full >= max_records {
            nread = iread_fnc
                .expect("non-null function pointer")(0 as libc::c_int, ibuf, max_bytes);
        } else {
            nread = iread_fnc
                .expect(
                    "non-null function pointer",
                )(0 as libc::c_int, ibuf, input_blocksize);
        }
        if nread > 0 as libc::c_int as libc::c_long {
            advance_input_offset(nread);
            if i_nocache {
                invalidate_cache(0 as libc::c_int, nread);
            }
        } else if nread == 0 as libc::c_int as libc::c_long {
            i_nocache_eof = (i_nocache_eof as libc::c_int | i_nocache as libc::c_int)
                as bool;
            o_nocache_eof = (o_nocache_eof as libc::c_int
                | (o_nocache as libc::c_int != 0
                    && conversions_mask & C_NOTRUNC as libc::c_int == 0) as libc::c_int)
                as bool;
            break;
        } else {
            if conversions_mask & C_NOERROR as libc::c_int == 0
                || status_level != STATUS_NONE as libc::c_int
            {
                nl_error(
                    0 as libc::c_int,
                    *__errno_location(),
                    dcgettext(
                        0 as *const libc::c_char,
                        b"error reading %s\0" as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    quotearg_style(shell_escape_always_quoting_style, input_file),
                );
            }
            if conversions_mask & C_NOERROR as libc::c_int != 0 {
                print_stats();
                let mut bad_portion: idx_t = input_blocksize - partread;
                invalidate_cache(0 as libc::c_int, bad_portion);
                if !advance_input_after_read_error(bad_portion) {
                    exit_status = 1 as libc::c_int;
                    input_seekable = 0 as libc::c_int != 0;
                    input_seek_errno = 29 as libc::c_int;
                }
                if !(conversions_mask & C_SYNC as libc::c_int != 0 && partread == 0) {
                    continue;
                }
                nread = 0 as libc::c_int as ssize_t;
            } else {
                exit_status = 1 as libc::c_int;
                break;
            }
        }
        n_bytes_read = nread;
        if n_bytes_read < input_blocksize {
            r_partial += 1;
            partread = n_bytes_read;
            if conversions_mask & C_SYNC as libc::c_int != 0 {
                if conversions_mask & C_NOERROR as libc::c_int == 0 {
                    memset(
                        ibuf.offset(n_bytes_read as isize) as *mut libc::c_void,
                        if conversions_mask
                            & (C_BLOCK as libc::c_int | C_UNBLOCK as libc::c_int) != 0
                        {
                            ' ' as i32
                        } else {
                            '\0' as i32
                        },
                        (input_blocksize - n_bytes_read) as libc::c_ulong,
                    );
                }
                n_bytes_read = input_blocksize;
            }
        } else {
            r_full += 1;
            partread = 0 as libc::c_int as idx_t;
        }
        if ibuf == obuf {
            let mut nwritten: idx_t = iwrite(1 as libc::c_int, obuf, n_bytes_read);
            w_bytes += nwritten;
            if nwritten != n_bytes_read {
                nl_error(
                    0 as libc::c_int,
                    *__errno_location(),
                    dcgettext(
                        0 as *const libc::c_char,
                        b"error writing %s\0" as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    quotearg_style(shell_escape_always_quoting_style, output_file),
                );
                return 1 as libc::c_int;
            } else {
                if n_bytes_read == input_blocksize {
                    w_full += 1;
                } else {
                    w_partial += 1;
                }
            }
        } else {
            if translation_needed {
                translate_buffer(ibuf, n_bytes_read);
            }
            if conversions_mask & C_SWAB as libc::c_int != 0 {
                bufstart = swab_buffer(ibuf, &mut n_bytes_read, &mut saved_byte);
            } else {
                bufstart = ibuf;
            }
            if conversions_mask & C_BLOCK as libc::c_int != 0 {
                copy_with_block(bufstart, n_bytes_read);
            } else if conversions_mask & C_UNBLOCK as libc::c_int != 0 {
                copy_with_unblock(bufstart, n_bytes_read);
            } else {
                copy_simple(bufstart, n_bytes_read);
            }
        }
    }
    if 0 as libc::c_int <= saved_byte {
        let mut saved_char: libc::c_char = saved_byte as libc::c_char;
        if conversions_mask & C_BLOCK as libc::c_int != 0 {
            copy_with_block(&mut saved_char, 1 as libc::c_int as idx_t);
        } else if conversions_mask & C_UNBLOCK as libc::c_int != 0 {
            copy_with_unblock(&mut saved_char, 1 as libc::c_int as idx_t);
        } else {
            let fresh35 = oc;
            oc = oc + 1;
            *obuf.offset(fresh35 as isize) = saved_char;
            if oc >= output_blocksize {
                write_output();
            }
        }
    }
    if conversions_mask & C_BLOCK as libc::c_int != 0
        && col > 0 as libc::c_int as libc::c_long
    {
        let mut i: idx_t = col;
        while i < conversion_blocksize {
            let fresh36 = oc;
            oc = oc + 1;
            *obuf.offset(fresh36 as isize) = space_character;
            if oc >= output_blocksize {
                write_output();
            }
            i += 1;
        }
    }
    if col != 0 && conversions_mask & C_UNBLOCK as libc::c_int != 0 {
        let fresh37 = oc;
        oc = oc + 1;
        *obuf.offset(fresh37 as isize) = newline_character;
        if oc >= output_blocksize {
            write_output();
        }
    }
    if oc != 0 as libc::c_int as libc::c_long {
        let mut nwritten_0: idx_t = iwrite(1 as libc::c_int, obuf, oc);
        w_bytes += nwritten_0;
        if nwritten_0 != 0 as libc::c_int as libc::c_long {
            w_partial += 1;
        }
        if nwritten_0 != oc {
            nl_error(
                0 as libc::c_int,
                *__errno_location(),
                dcgettext(
                    0 as *const libc::c_char,
                    b"error writing %s\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
                quotearg_style(shell_escape_always_quoting_style, output_file),
            );
            return 1 as libc::c_int;
        }
    }
    if final_op_was_seek {
        let mut stdout_stat: stat = stat {
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
        if ifstat(1 as libc::c_int, &mut stdout_stat) != 0 as libc::c_int {
            nl_error(
                0 as libc::c_int,
                *__errno_location(),
                dcgettext(
                    0 as *const libc::c_char,
                    b"cannot fstat %s\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
                quotearg_style(shell_escape_always_quoting_style, output_file),
            );
            return 1 as libc::c_int;
        }
        if stdout_stat.st_mode & 0o170000 as libc::c_int as libc::c_uint
            == 0o100000 as libc::c_int as libc::c_uint
            || (stdout_stat.st_mode).wrapping_sub(stdout_stat.st_mode) != 0
        {
            let mut output_offset: off_t = lseek(
                1 as libc::c_int,
                0 as libc::c_int as __off_t,
                1 as libc::c_int,
            );
            if 0 as libc::c_int as libc::c_long <= output_offset
                && stdout_stat.st_size < output_offset
            {
                if iftruncate(1 as libc::c_int, output_offset) != 0 as libc::c_int {
                    nl_error(
                        0 as libc::c_int,
                        *__errno_location(),
                        dcgettext(
                            0 as *const libc::c_char,
                            b"failed to truncate to %ld bytes in output file %s\0"
                                as *const u8 as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                        output_offset,
                        quotearg_style(shell_escape_always_quoting_style, output_file),
                    );
                    return 1 as libc::c_int;
                }
            }
        }
    }
    if conversions_mask & (C_FDATASYNC as libc::c_int | C_FSYNC as libc::c_int) != 0
        && status_level == STATUS_PROGRESS as libc::c_int
        && 0 as libc::c_int as libc::c_long <= reported_w_bytes
        && reported_w_bytes < w_bytes
    {
        print_xfer_stats(0 as libc::c_int as xtime_t);
    }
    return exit_status;
}
unsafe extern "C" fn synchronize_output() -> libc::c_int {
    let mut exit_status: libc::c_int = 0 as libc::c_int;
    let mut mask: libc::c_int = conversions_mask;
    conversions_mask &= !(C_FDATASYNC as libc::c_int | C_FSYNC as libc::c_int);
    if mask & C_FDATASYNC as libc::c_int != 0
        && ifdatasync(1 as libc::c_int) != 0 as libc::c_int
    {
        if *__errno_location() != 38 as libc::c_int
            && *__errno_location() != 22 as libc::c_int
        {
            nl_error(
                0 as libc::c_int,
                *__errno_location(),
                dcgettext(
                    0 as *const libc::c_char,
                    b"fdatasync failed for %s\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
                quotearg_style(shell_escape_always_quoting_style, output_file),
            );
            exit_status = 1 as libc::c_int;
        }
        mask |= C_FSYNC as libc::c_int;
    }
    if mask & C_FSYNC as libc::c_int != 0 && ifsync(1 as libc::c_int) != 0 as libc::c_int
    {
        nl_error(
            0 as libc::c_int,
            *__errno_location(),
            dcgettext(
                0 as *const libc::c_char,
                b"fsync failed for %s\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            quotearg_style(shell_escape_always_quoting_style, output_file),
        );
        return 1 as libc::c_int;
    }
    return exit_status;
}
unsafe fn main_0(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut exit_status: libc::c_int = 0;
    let mut offset: off_t = 0;
    install_signal_handlers();
    set_program_name(*argv.offset(0 as libc::c_int as isize));
    setlocale(6 as libc::c_int, b"\0" as *const u8 as *const libc::c_char);
    bindtextdomain(
        b"coreutils\0" as *const u8 as *const libc::c_char,
        b"/usr/local/share/locale\0" as *const u8 as *const libc::c_char,
    );
    textdomain(b"coreutils\0" as *const u8 as *const libc::c_char);
    atexit(Some(maybe_close_stdout as unsafe extern "C" fn() -> ()));
    page_size = getpagesize() as idx_t;
    parse_gnu_standard_options_only(
        argc,
        argv,
        b"dd\0" as *const u8 as *const libc::c_char,
        b"coreutils\0" as *const u8 as *const libc::c_char,
        Version,
        1 as libc::c_int != 0,
        Some(usage as unsafe extern "C" fn(libc::c_int) -> ()),
        b"Paul Rubin\0" as *const u8 as *const libc::c_char,
        b"David MacKenzie\0" as *const u8 as *const libc::c_char,
        b"Stuart Kemp\0" as *const u8 as *const libc::c_char,
        0 as *mut libc::c_void as *const libc::c_char,
    );
    close_stdout_required = 0 as libc::c_int != 0;
    i = 0 as libc::c_int;
    while i < 256 as libc::c_int {
        trans_table[i as usize] = i as libc::c_uchar;
        i += 1;
    }
    scanargs(argc, argv);
    apply_translations();
    if input_file.is_null() {
        input_file = dcgettext(
            0 as *const libc::c_char,
            b"standard input\0" as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        );
        set_fd_flags(0 as libc::c_int, input_flags, input_file);
    } else if ifd_reopen(
        0 as libc::c_int,
        input_file,
        0 as libc::c_int | input_flags,
        0 as libc::c_int as mode_t,
    ) < 0 as libc::c_int
    {
        if ::core::mem::size_of::<C2RustUnnamed_31>() as libc::c_ulong != 0 {
            nl_error(
                1 as libc::c_int,
                *__errno_location(),
                dcgettext(
                    0 as *const libc::c_char,
                    b"failed to open %s\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
                quotearg_style(shell_escape_always_quoting_style, input_file),
            );
            if 0 as libc::c_int != 0 {} else {
                unreachable!();
            };
        } else {
            nl_error(
                1 as libc::c_int,
                *__errno_location(),
                dcgettext(
                    0 as *const libc::c_char,
                    b"failed to open %s\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
                quotearg_style(shell_escape_always_quoting_style, input_file),
            );
            if 0 as libc::c_int != 0 {} else {
                unreachable!();
            };
        };
    }
    offset = lseek(0 as libc::c_int, 0 as libc::c_int as __off_t, 1 as libc::c_int);
    input_seekable = 0 as libc::c_int as libc::c_long <= offset;
    input_offset = if 0 as libc::c_int as libc::c_long > offset {
        0 as libc::c_int as libc::c_long
    } else {
        offset
    };
    input_seek_errno = *__errno_location();
    if output_file.is_null() {
        output_file = dcgettext(
            0 as *const libc::c_char,
            b"standard output\0" as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        );
        set_fd_flags(1 as libc::c_int, output_flags, output_file);
    } else {
        let mut perms: mode_t = (0o400 as libc::c_int | 0o200 as libc::c_int
            | 0o400 as libc::c_int >> 3 as libc::c_int
            | 0o200 as libc::c_int >> 3 as libc::c_int
            | 0o400 as libc::c_int >> 3 as libc::c_int >> 3 as libc::c_int
            | 0o200 as libc::c_int >> 3 as libc::c_int >> 3 as libc::c_int) as mode_t;
        let mut opts: libc::c_int = output_flags
            | (if conversions_mask & C_NOCREAT as libc::c_int != 0 {
                0 as libc::c_int
            } else {
                0o100 as libc::c_int
            })
            | (if conversions_mask & C_EXCL as libc::c_int != 0 {
                0o200 as libc::c_int
            } else {
                0 as libc::c_int
            })
            | (if seek_records != 0 || conversions_mask & C_NOTRUNC as libc::c_int != 0 {
                0 as libc::c_int
            } else {
                0o1000 as libc::c_int
            });
        let mut size: off_t = 0;
        if ((if (0 as libc::c_int as off_t) < -(1 as libc::c_int) as off_t
            && ((if 1 as libc::c_int != 0 {
                0 as libc::c_int as libc::c_long
            } else {
                seek_records
            }) - 1 as libc::c_int as libc::c_long) < 0 as libc::c_int as libc::c_long
            && ((if 1 as libc::c_int != 0 {
                0 as libc::c_int as libc::c_long
            } else {
                output_blocksize
            }) - 1 as libc::c_int as libc::c_long) < 0 as libc::c_int as libc::c_long
            && (if output_blocksize < 0 as libc::c_int as libc::c_long {
                (if seek_records < 0 as libc::c_int as libc::c_long {
                    (if ((if 1 as libc::c_int != 0 {
                        0 as libc::c_int as libc::c_long
                    } else {
                        (if 1 as libc::c_int != 0 {
                            0 as libc::c_int as libc::c_long
                        } else {
                            -(1 as libc::c_int) as off_t
                        }) + output_blocksize
                    }) - 1 as libc::c_int as libc::c_long)
                        < 0 as libc::c_int as libc::c_long
                    {
                        (seek_records < -(1 as libc::c_int) as off_t / output_blocksize)
                            as libc::c_int
                    } else {
                        ((if (if (if ((if 1 as libc::c_int != 0 {
                            0 as libc::c_int as libc::c_long
                        } else {
                            output_blocksize
                        }) - 1 as libc::c_int as libc::c_long)
                            < 0 as libc::c_int as libc::c_long
                        {
                            !(((((if 1 as libc::c_int != 0 {
                                0 as libc::c_int as libc::c_long
                            } else {
                                output_blocksize
                            }) + 1 as libc::c_int as libc::c_long)
                                << (::core::mem::size_of::<idx_t>() as libc::c_ulong)
                                    .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                    .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                - 1 as libc::c_int as libc::c_long)
                                * 2 as libc::c_int as libc::c_long
                                + 1 as libc::c_int as libc::c_long)
                        } else {
                            (if 1 as libc::c_int != 0 {
                                0 as libc::c_int as libc::c_long
                            } else {
                                output_blocksize
                            }) + 0 as libc::c_int as libc::c_long
                        }) < 0 as libc::c_int as libc::c_long
                        {
                            (output_blocksize
                                < -(if ((if 1 as libc::c_int != 0 {
                                    0 as libc::c_int as libc::c_long
                                } else {
                                    output_blocksize
                                }) - 1 as libc::c_int as libc::c_long)
                                    < 0 as libc::c_int as libc::c_long
                                {
                                    ((((if 1 as libc::c_int != 0 {
                                        0 as libc::c_int as libc::c_long
                                    } else {
                                        output_blocksize
                                    }) + 1 as libc::c_int as libc::c_long)
                                        << (::core::mem::size_of::<idx_t>() as libc::c_ulong)
                                            .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                            .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                        - 1 as libc::c_int as libc::c_long)
                                        * 2 as libc::c_int as libc::c_long
                                        + 1 as libc::c_int as libc::c_long
                                } else {
                                    (if 1 as libc::c_int != 0 {
                                        0 as libc::c_int as libc::c_long
                                    } else {
                                        output_blocksize
                                    }) - 1 as libc::c_int as libc::c_long
                                })) as libc::c_int
                        } else {
                            ((0 as libc::c_int as libc::c_long) < output_blocksize)
                                as libc::c_int
                        }) != 0
                        {
                            (if 1 as libc::c_int != 0 {
                                0 as libc::c_int as libc::c_long
                            } else {
                                output_blocksize
                            }) + -(1 as libc::c_int) as off_t
                                >> (::core::mem::size_of::<idx_t>() as libc::c_ulong)
                                    .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                    .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                        } else {
                            -(1 as libc::c_int) as off_t / -output_blocksize
                        }) <= -(1 as libc::c_int) as libc::c_long - seek_records)
                            as libc::c_int
                    })
                } else {
                    (if (if (if ((if 1 as libc::c_int != 0 {
                        0 as libc::c_int as libc::c_long
                    } else {
                        (if 1 as libc::c_int != 0 {
                            0 as libc::c_int as libc::c_long
                        } else {
                            output_blocksize
                        }) + 0 as libc::c_int as libc::c_long
                    }) - 1 as libc::c_int as libc::c_long)
                        < 0 as libc::c_int as libc::c_long
                    {
                        !(((((if 1 as libc::c_int != 0 {
                            0 as libc::c_int as libc::c_long
                        } else {
                            (if 1 as libc::c_int != 0 {
                                0 as libc::c_int as libc::c_long
                            } else {
                                output_blocksize
                            }) + 0 as libc::c_int as libc::c_long
                        }) + 1 as libc::c_int as libc::c_long)
                            << (::core::mem::size_of::<libc::c_long>() as libc::c_ulong)
                                .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                            - 1 as libc::c_int as libc::c_long)
                            * 2 as libc::c_int as libc::c_long
                            + 1 as libc::c_int as libc::c_long)
                    } else {
                        (if 1 as libc::c_int != 0 {
                            0 as libc::c_int as libc::c_long
                        } else {
                            (if 1 as libc::c_int != 0 {
                                0 as libc::c_int as libc::c_long
                            } else {
                                output_blocksize
                            }) + 0 as libc::c_int as libc::c_long
                        }) + 0 as libc::c_int as libc::c_long
                    }) < 0 as libc::c_int as libc::c_long
                    {
                        (((if 1 as libc::c_int != 0 {
                            0 as libc::c_int as libc::c_long
                        } else {
                            output_blocksize
                        }) + 0 as libc::c_int as libc::c_long)
                            < -(if ((if 1 as libc::c_int != 0 {
                                0 as libc::c_int as libc::c_long
                            } else {
                                (if 1 as libc::c_int != 0 {
                                    0 as libc::c_int as libc::c_long
                                } else {
                                    output_blocksize
                                }) + 0 as libc::c_int as libc::c_long
                            }) - 1 as libc::c_int as libc::c_long)
                                < 0 as libc::c_int as libc::c_long
                            {
                                ((((if 1 as libc::c_int != 0 {
                                    0 as libc::c_int as libc::c_long
                                } else {
                                    (if 1 as libc::c_int != 0 {
                                        0 as libc::c_int as libc::c_long
                                    } else {
                                        output_blocksize
                                    }) + 0 as libc::c_int as libc::c_long
                                }) + 1 as libc::c_int as libc::c_long)
                                    << (::core::mem::size_of::<libc::c_long>() as libc::c_ulong)
                                        .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                        .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                    - 1 as libc::c_int as libc::c_long)
                                    * 2 as libc::c_int as libc::c_long
                                    + 1 as libc::c_int as libc::c_long
                            } else {
                                (if 1 as libc::c_int != 0 {
                                    0 as libc::c_int as libc::c_long
                                } else {
                                    (if 1 as libc::c_int != 0 {
                                        0 as libc::c_int as libc::c_long
                                    } else {
                                        output_blocksize
                                    }) + 0 as libc::c_int as libc::c_long
                                }) - 1 as libc::c_int as libc::c_long
                            })) as libc::c_int
                    } else {
                        ((0 as libc::c_int as libc::c_long)
                            < (if 1 as libc::c_int != 0 {
                                0 as libc::c_int as libc::c_long
                            } else {
                                output_blocksize
                            }) + 0 as libc::c_int as libc::c_long) as libc::c_int
                    }) != 0 && output_blocksize == -(1 as libc::c_int) as libc::c_long
                    {
                        (if ((if 1 as libc::c_int != 0 {
                            0 as libc::c_int as libc::c_long
                        } else {
                            seek_records
                        }) - 1 as libc::c_int as libc::c_long)
                            < 0 as libc::c_int as libc::c_long
                        {
                            ((0 as libc::c_int as libc::c_long)
                                < seek_records + 0 as libc::c_int as libc::c_long)
                                as libc::c_int
                        } else {
                            ((0 as libc::c_int as libc::c_long) < seek_records
                                && ((-(1 as libc::c_int) - 0 as libc::c_int)
                                    as libc::c_long)
                                    < seek_records - 1 as libc::c_int as libc::c_long)
                                as libc::c_int
                        })
                    } else {
                        (0 as libc::c_int as libc::c_long / output_blocksize
                            < seek_records) as libc::c_int
                    })
                })
            } else {
                (if output_blocksize == 0 as libc::c_int as libc::c_long {
                    0 as libc::c_int
                } else {
                    (if seek_records < 0 as libc::c_int as libc::c_long {
                        (if (if (if ((if 1 as libc::c_int != 0 {
                            0 as libc::c_int as libc::c_long
                        } else {
                            (if 1 as libc::c_int != 0 {
                                0 as libc::c_int as libc::c_long
                            } else {
                                seek_records
                            }) + 0 as libc::c_int as libc::c_long
                        }) - 1 as libc::c_int as libc::c_long)
                            < 0 as libc::c_int as libc::c_long
                        {
                            !(((((if 1 as libc::c_int != 0 {
                                0 as libc::c_int as libc::c_long
                            } else {
                                (if 1 as libc::c_int != 0 {
                                    0 as libc::c_int as libc::c_long
                                } else {
                                    seek_records
                                }) + 0 as libc::c_int as libc::c_long
                            }) + 1 as libc::c_int as libc::c_long)
                                << (::core::mem::size_of::<libc::c_long>() as libc::c_ulong)
                                    .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                    .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                - 1 as libc::c_int as libc::c_long)
                                * 2 as libc::c_int as libc::c_long
                                + 1 as libc::c_int as libc::c_long)
                        } else {
                            (if 1 as libc::c_int != 0 {
                                0 as libc::c_int as libc::c_long
                            } else {
                                (if 1 as libc::c_int != 0 {
                                    0 as libc::c_int as libc::c_long
                                } else {
                                    seek_records
                                }) + 0 as libc::c_int as libc::c_long
                            }) + 0 as libc::c_int as libc::c_long
                        }) < 0 as libc::c_int as libc::c_long
                        {
                            (((if 1 as libc::c_int != 0 {
                                0 as libc::c_int as libc::c_long
                            } else {
                                seek_records
                            }) + 0 as libc::c_int as libc::c_long)
                                < -(if ((if 1 as libc::c_int != 0 {
                                    0 as libc::c_int as libc::c_long
                                } else {
                                    (if 1 as libc::c_int != 0 {
                                        0 as libc::c_int as libc::c_long
                                    } else {
                                        seek_records
                                    }) + 0 as libc::c_int as libc::c_long
                                }) - 1 as libc::c_int as libc::c_long)
                                    < 0 as libc::c_int as libc::c_long
                                {
                                    ((((if 1 as libc::c_int != 0 {
                                        0 as libc::c_int as libc::c_long
                                    } else {
                                        (if 1 as libc::c_int != 0 {
                                            0 as libc::c_int as libc::c_long
                                        } else {
                                            seek_records
                                        }) + 0 as libc::c_int as libc::c_long
                                    }) + 1 as libc::c_int as libc::c_long)
                                        << (::core::mem::size_of::<libc::c_long>() as libc::c_ulong)
                                            .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                            .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                        - 1 as libc::c_int as libc::c_long)
                                        * 2 as libc::c_int as libc::c_long
                                        + 1 as libc::c_int as libc::c_long
                                } else {
                                    (if 1 as libc::c_int != 0 {
                                        0 as libc::c_int as libc::c_long
                                    } else {
                                        (if 1 as libc::c_int != 0 {
                                            0 as libc::c_int as libc::c_long
                                        } else {
                                            seek_records
                                        }) + 0 as libc::c_int as libc::c_long
                                    }) - 1 as libc::c_int as libc::c_long
                                })) as libc::c_int
                        } else {
                            ((0 as libc::c_int as libc::c_long)
                                < (if 1 as libc::c_int != 0 {
                                    0 as libc::c_int as libc::c_long
                                } else {
                                    seek_records
                                }) + 0 as libc::c_int as libc::c_long) as libc::c_int
                        }) != 0 && seek_records == -(1 as libc::c_int) as libc::c_long
                        {
                            (if ((if 1 as libc::c_int != 0 {
                                0 as libc::c_int as libc::c_long
                            } else {
                                output_blocksize
                            }) - 1 as libc::c_int as libc::c_long)
                                < 0 as libc::c_int as libc::c_long
                            {
                                ((0 as libc::c_int as libc::c_long)
                                    < output_blocksize + 0 as libc::c_int as libc::c_long)
                                    as libc::c_int
                            } else {
                                (((-(1 as libc::c_int) - 0 as libc::c_int) as libc::c_long)
                                    < output_blocksize - 1 as libc::c_int as libc::c_long)
                                    as libc::c_int
                            })
                        } else {
                            (0 as libc::c_int as libc::c_long / seek_records
                                < output_blocksize) as libc::c_int
                        })
                    } else {
                        (-(1 as libc::c_int) as off_t / output_blocksize < seek_records)
                            as libc::c_int
                    })
                })
            }) != 0
        {
            let (fresh38, fresh39) = seek_records.overflowing_mul(output_blocksize);
            *(&mut size as *mut off_t) = fresh38;
            1 as libc::c_int
        } else {
            let (fresh40, fresh41) = seek_records.overflowing_mul(output_blocksize);
            *(&mut size as *mut off_t) = fresh40;
            fresh41 as libc::c_int
        }) != 0
            || {
                let (fresh42, fresh43) = seek_bytes.overflowing_add(size);
                *(&mut size as *mut off_t) = fresh42;
                fresh43 as libc::c_int != 0
            }) && conversions_mask & C_NOTRUNC as libc::c_int == 0
        {
            if ::core::mem::size_of::<C2RustUnnamed_30>() as libc::c_ulong != 0 {
                nl_error(
                    1 as libc::c_int,
                    0 as libc::c_int,
                    dcgettext(
                        0 as *const libc::c_char,
                        b"offset too large: cannot truncate to a length of seek=%ld (%td-byte) blocks\0"
                            as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    seek_records,
                    output_blocksize,
                );
                if 0 as libc::c_int != 0 {} else {
                    unreachable!();
                };
            } else {
                nl_error(
                    1 as libc::c_int,
                    0 as libc::c_int,
                    dcgettext(
                        0 as *const libc::c_char,
                        b"offset too large: cannot truncate to a length of seek=%ld (%td-byte) blocks\0"
                            as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    seek_records,
                    output_blocksize,
                );
                if 0 as libc::c_int != 0 {} else {
                    unreachable!();
                };
            };
        }
        if (seek_records == 0
            || ifd_reopen(
                1 as libc::c_int,
                output_file,
                0o2 as libc::c_int | opts,
                perms,
            ) < 0 as libc::c_int)
            && ifd_reopen(
                1 as libc::c_int,
                output_file,
                0o1 as libc::c_int | opts,
                perms,
            ) < 0 as libc::c_int
        {
            if ::core::mem::size_of::<C2RustUnnamed_29>() as libc::c_ulong != 0 {
                nl_error(
                    1 as libc::c_int,
                    *__errno_location(),
                    dcgettext(
                        0 as *const libc::c_char,
                        b"failed to open %s\0" as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    quotearg_style(shell_escape_always_quoting_style, output_file),
                );
                if 0 as libc::c_int != 0 {} else {
                    unreachable!();
                };
            } else {
                nl_error(
                    1 as libc::c_int,
                    *__errno_location(),
                    dcgettext(
                        0 as *const libc::c_char,
                        b"failed to open %s\0" as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    quotearg_style(shell_escape_always_quoting_style, output_file),
                );
                if 0 as libc::c_int != 0 {} else {
                    unreachable!();
                };
            };
        }
        if seek_records != 0 as libc::c_int as libc::c_long
            && conversions_mask & C_NOTRUNC as libc::c_int == 0
        {
            if iftruncate(1 as libc::c_int, size) != 0 as libc::c_int {
                let mut ftruncate_errno: libc::c_int = *__errno_location();
                let mut stdout_stat: stat = stat {
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
                if ifstat(1 as libc::c_int, &mut stdout_stat) != 0 as libc::c_int {
                    nl_error(
                        0 as libc::c_int,
                        *__errno_location(),
                        dcgettext(
                            0 as *const libc::c_char,
                            b"cannot fstat %s\0" as *const u8 as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                        quotearg_style(shell_escape_always_quoting_style, output_file),
                    );
                    exit_status = 1 as libc::c_int;
                } else if stdout_stat.st_mode & 0o170000 as libc::c_int as libc::c_uint
                    == 0o100000 as libc::c_int as libc::c_uint
                    || stdout_stat.st_mode & 0o170000 as libc::c_int as libc::c_uint
                        == 0o40000 as libc::c_int as libc::c_uint
                    || (stdout_stat.st_mode).wrapping_sub(stdout_stat.st_mode) != 0
                {
                    let mut isize: intmax_t = size;
                    nl_error(
                        0 as libc::c_int,
                        ftruncate_errno,
                        dcgettext(
                            0 as *const libc::c_char,
                            b"failed to truncate to %ld bytes in output file %s\0"
                                as *const u8 as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                        isize,
                        quotearg_style(shell_escape_always_quoting_style, output_file),
                    );
                    exit_status = 1 as libc::c_int;
                }
            }
        }
    }
    start_time = gethrxtime();
    next_time = start_time + 1000000000 as libc::c_int as libc::c_longlong;
    exit_status = dd_copy();
    let mut sync_status: libc::c_int = synchronize_output();
    if sync_status != 0 {
        exit_status = sync_status;
    }
    if max_records == 0 as libc::c_int as libc::c_long
        && max_bytes == 0 as libc::c_int as libc::c_long
    {
        if i_nocache as libc::c_int != 0
            && !invalidate_cache(0 as libc::c_int, 0 as libc::c_int as off_t)
        {
            nl_error(
                0 as libc::c_int,
                *__errno_location(),
                dcgettext(
                    0 as *const libc::c_char,
                    b"failed to discard cache for: %s\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
                quotearg_n_style_colon(
                    0 as libc::c_int,
                    shell_escape_quoting_style,
                    input_file,
                ),
            );
            exit_status = 1 as libc::c_int;
        }
        if o_nocache as libc::c_int != 0
            && !invalidate_cache(1 as libc::c_int, 0 as libc::c_int as off_t)
        {
            nl_error(
                0 as libc::c_int,
                *__errno_location(),
                dcgettext(
                    0 as *const libc::c_char,
                    b"failed to discard cache for: %s\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
                quotearg_n_style_colon(
                    0 as libc::c_int,
                    shell_escape_quoting_style,
                    output_file,
                ),
            );
            exit_status = 1 as libc::c_int;
        }
    } else {
        if i_nocache as libc::c_int != 0 || i_nocache_eof as libc::c_int != 0 {
            invalidate_cache(0 as libc::c_int, 0 as libc::c_int as off_t);
        }
        if o_nocache as libc::c_int != 0 || o_nocache_eof as libc::c_int != 0 {
            invalidate_cache(1 as libc::c_int, 0 as libc::c_int as off_t);
        }
    }
    finish_up();
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
unsafe extern "C" fn run_static_initializers() {
    flags = [
        {
            let mut init = symbol_value {
                symbol: *::core::mem::transmute::<
                    &[u8; 12],
                    &mut [libc::c_char; 12],
                >(b"append\0\0\0\0\0\0"),
                value: 0o2000 as libc::c_int,
            };
            init
        },
        {
            let mut init = symbol_value {
                symbol: *::core::mem::transmute::<
                    &[u8; 12],
                    &mut [libc::c_char; 12],
                >(b"binary\0\0\0\0\0\0"),
                value: 0 as libc::c_int,
            };
            init
        },
        {
            let mut init = symbol_value {
                symbol: *::core::mem::transmute::<
                    &[u8; 12],
                    &mut [libc::c_char; 12],
                >(b"cio\0\0\0\0\0\0\0\0\0"),
                value: 0 as libc::c_int,
            };
            init
        },
        {
            let mut init = symbol_value {
                symbol: *::core::mem::transmute::<
                    &[u8; 12],
                    &mut [libc::c_char; 12],
                >(b"direct\0\0\0\0\0\0"),
                value: 0o200000 as libc::c_int,
            };
            init
        },
        {
            let mut init = symbol_value {
                symbol: *::core::mem::transmute::<
                    &[u8; 12],
                    &mut [libc::c_char; 12],
                >(b"directory\0\0\0"),
                value: 0o40000 as libc::c_int,
            };
            init
        },
        {
            let mut init = symbol_value {
                symbol: *::core::mem::transmute::<
                    &[u8; 12],
                    &mut [libc::c_char; 12],
                >(b"dsync\0\0\0\0\0\0\0"),
                value: 0o10000 as libc::c_int,
            };
            init
        },
        {
            let mut init = symbol_value {
                symbol: *::core::mem::transmute::<
                    &[u8; 12],
                    &mut [libc::c_char; 12],
                >(b"noatime\0\0\0\0\0"),
                value: 0o1000000 as libc::c_int,
            };
            init
        },
        {
            let mut init = symbol_value {
                symbol: *::core::mem::transmute::<
                    &[u8; 12],
                    &mut [libc::c_char; 12],
                >(b"nocache\0\0\0\0\0"),
                value: O_NOCACHE as libc::c_int,
            };
            init
        },
        {
            let mut init = symbol_value {
                symbol: *::core::mem::transmute::<
                    &[u8; 12],
                    &mut [libc::c_char; 12],
                >(b"noctty\0\0\0\0\0\0"),
                value: 0o400 as libc::c_int,
            };
            init
        },
        {
            let mut init = symbol_value {
                symbol: *::core::mem::transmute::<
                    &[u8; 12],
                    &mut [libc::c_char; 12],
                >(b"nofollow\0\0\0\0"),
                value: if 1 as libc::c_int != 0 {
                    0o100000 as libc::c_int
                } else {
                    0 as libc::c_int
                },
            };
            init
        },
        {
            let mut init = symbol_value {
                symbol: *::core::mem::transmute::<
                    &[u8; 12],
                    &mut [libc::c_char; 12],
                >(b"nolinks\0\0\0\0\0"),
                value: 0 as libc::c_int,
            };
            init
        },
        {
            let mut init = symbol_value {
                symbol: *::core::mem::transmute::<
                    &[u8; 12],
                    &mut [libc::c_char; 12],
                >(b"nonblock\0\0\0\0"),
                value: 0o4000 as libc::c_int,
            };
            init
        },
        {
            let mut init = symbol_value {
                symbol: *::core::mem::transmute::<
                    &[u8; 12],
                    &mut [libc::c_char; 12],
                >(b"sync\0\0\0\0\0\0\0\0"),
                value: 0o4010000 as libc::c_int,
            };
            init
        },
        {
            let mut init = symbol_value {
                symbol: *::core::mem::transmute::<
                    &[u8; 12],
                    &mut [libc::c_char; 12],
                >(b"text\0\0\0\0\0\0\0\0"),
                value: 0 as libc::c_int,
            };
            init
        },
        {
            let mut init = symbol_value {
                symbol: *::core::mem::transmute::<
                    &[u8; 12],
                    &mut [libc::c_char; 12],
                >(b"fullblock\0\0\0"),
                value: O_FULLBLOCK as libc::c_int,
            };
            init
        },
        {
            let mut init = symbol_value {
                symbol: *::core::mem::transmute::<
                    &[u8; 12],
                    &mut [libc::c_char; 12],
                >(b"count_bytes\0"),
                value: O_COUNT_BYTES as libc::c_int,
            };
            init
        },
        {
            let mut init = symbol_value {
                symbol: *::core::mem::transmute::<
                    &[u8; 12],
                    &mut [libc::c_char; 12],
                >(b"skip_bytes\0\0"),
                value: O_SKIP_BYTES as libc::c_int,
            };
            init
        },
        {
            let mut init = symbol_value {
                symbol: *::core::mem::transmute::<
                    &[u8; 12],
                    &mut [libc::c_char; 12],
                >(b"seek_bytes\0\0"),
                value: O_SEEK_BYTES as libc::c_int,
            };
            init
        },
        {
            let mut init = symbol_value {
                symbol: *::core::mem::transmute::<
                    &[u8; 12],
                    &mut [libc::c_char; 12],
                >(b"\0\0\0\0\0\0\0\0\0\0\0\0"),
                value: 0 as libc::c_int,
            };
            init
        },
    ];
}
#[used]
#[cfg_attr(target_os = "linux", link_section = ".init_array")]
#[cfg_attr(target_os = "windows", link_section = ".CRT$XIB")]
#[cfg_attr(target_os = "macos", link_section = "__DATA,__mod_init_func")]
static INIT_ARRAY: [unsafe extern "C" fn(); 1] = [run_static_initializers];
