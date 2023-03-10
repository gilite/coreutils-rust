#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![feature(extern_types, label_break_value)]
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type tm_zone;
    pub type __dirstream;
    pub type quoting_options;
    pub type hash_table;
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    fn ioctl(__fd: libc::c_int, __request: libc::c_ulong, _: ...) -> libc::c_int;
    static mut stdout: *mut FILE;
    static mut stderr: *mut FILE;
    fn fflush_unlocked(__stream: *mut FILE) -> libc::c_int;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn snprintf(
        _: *mut libc::c_char,
        _: libc::c_ulong,
        _: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn fputs_unlocked(__s: *const libc::c_char, __stream: *mut FILE) -> libc::c_int;
    fn fwrite_unlocked(
        __ptr: *const libc::c_void,
        __size: size_t,
        __n: size_t,
        __stream: *mut FILE,
    ) -> size_t;
    fn __overflow(_: *mut FILE, _: libc::c_int) -> libc::c_int;
    fn _setjmp(_: *mut __jmp_buf_tag) -> libc::c_int;
    fn longjmp(_: *mut __jmp_buf_tag, _: libc::c_int) -> !;
    static mut optarg: *mut libc::c_char;
    static mut optind: libc::c_int;
    fn getopt_long(
        ___argc: libc::c_int,
        ___argv: *const *mut libc::c_char,
        __shortopts: *const libc::c_char,
        __longopts: *const option,
        __longind: *mut libc::c_int,
    ) -> libc::c_int;
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
    fn signal(__sig: libc::c_int, __handler: __sighandler_t) -> __sighandler_t;
    fn tcgetpgrp(__fd: libc::c_int) -> __pid_t;
    fn isatty(__fd: libc::c_int) -> libc::c_int;
    fn __errno_location() -> *mut libc::c_int;
    fn mbsinit(__ps: *const mbstate_t) -> libc::c_int;
    fn wcwidth(__c: wchar_t) -> libc::c_int;
    fn free(_: *mut libc::c_void);
    fn rpl_mbrtowc(
        pwc: *mut wchar_t,
        s: *const libc::c_char,
        n: size_t,
        ps: *mut mbstate_t,
    ) -> size_t;
    fn nl_langinfo(__item: nl_item) -> *mut libc::c_char;
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
    fn memcmp(
        _: *const libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strncmp(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn strcoll(__s1: *const libc::c_char, __s2: *const libc::c_char) -> libc::c_int;
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strrchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn stpcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn statx(
        __dirfd: libc::c_int,
        __path: *const libc::c_char,
        __flags: libc::c_int,
        __mask: libc::c_uint,
        __buf: *mut statx,
    ) -> libc::c_int;
    fn stat(__file: *const libc::c_char, __buf: *mut stat) -> libc::c_int;
    fn localtime_rz(
        __tz: timezone_t,
        __timer: *const time_t,
        __result: *mut tm,
    ) -> *mut tm;
    fn tzalloc(__name: *const libc::c_char) -> timezone_t;
    fn stpncpy(
        _: *mut libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> *mut libc::c_char;
    fn __ctype_get_mb_cur_max() -> size_t;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn abort() -> !;
    fn atexit(__func: Option::<unsafe extern "C" fn() -> ()>) -> libc::c_int;
    fn exit(_: libc::c_int) -> !;
    fn getenv(__name: *const libc::c_char) -> *mut libc::c_char;
    static mut Version: *const libc::c_char;
    static mut exit_failure: libc::c_int;
    fn opendir(__name: *const libc::c_char) -> *mut DIR;
    fn closedir(__dirp: *mut DIR) -> libc::c_int;
    fn readdir(__dirp: *mut DIR) -> *mut dirent;
    fn dirfd(__dirp: *mut DIR) -> libc::c_int;
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
    fn xalloc_die();
    fn xmalloc(s: size_t) -> *mut libc::c_void;
    fn xreallocarray(p: *mut libc::c_void, n: size_t, s: size_t) -> *mut libc::c_void;
    fn xstrdup(str: *const libc::c_char) -> *mut libc::c_char;
    fn xnmalloc(n: size_t, s: size_t) -> *mut libc::c_void;
    fn last_component(filename: *const libc::c_char) -> *mut libc::c_char;
    fn dir_len(file: *const libc::c_char) -> size_t;
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
    fn clone_quoting_options(o: *mut quoting_options) -> *mut quoting_options;
    fn get_quoting_style(o: *const quoting_options) -> quoting_style;
    fn set_quoting_style(o: *mut quoting_options, s: quoting_style);
    fn set_char_quoting(
        o: *mut quoting_options,
        c: libc::c_char,
        i: libc::c_int,
    ) -> libc::c_int;
    fn quotearg_buffer(
        buffer: *mut libc::c_char,
        buffersize: size_t,
        arg: *const libc::c_char,
        argsize: size_t,
        o: *const quoting_options,
    ) -> size_t;
    fn quotearg_style(s: quoting_style, arg: *const libc::c_char) -> *mut libc::c_char;
    fn quotearg_n_style_colon(
        n: libc::c_int,
        s: quoting_style,
        arg: *const libc::c_char,
    ) -> *mut libc::c_char;
    fn gettime(_: *mut timespec);
    fn fnmatch(
        __pattern: *const libc::c_char,
        __name: *const libc::c_char,
        __flags: libc::c_int,
    ) -> libc::c_int;
    fn file_has_acl(_: *const libc::c_char, _: *const stat) -> libc::c_int;
    fn quote_n(n: libc::c_int, arg: *const libc::c_char) -> *const libc::c_char;
    fn quote(arg: *const libc::c_char) -> *const libc::c_char;
    fn argmatch(
        arg: *const libc::c_char,
        arglist: *const *const libc::c_char,
        vallist: *const libc::c_void,
        valsize: size_t,
    ) -> ptrdiff_t;
    static mut argmatch_die: argmatch_exit_fn;
    fn argmatch_invalid(
        context: *const libc::c_char,
        value: *const libc::c_char,
        problem: ptrdiff_t,
    );
    fn __xargmatch_internal(
        context: *const libc::c_char,
        arg: *const libc::c_char,
        arglist: *const *const libc::c_char,
        vallist: *const libc::c_void,
        valsize: size_t,
        exit_fn: argmatch_exit_fn,
        allow_abbreviation: bool,
    ) -> ptrdiff_t;
    fn c_strncasecmp(
        s1: *const libc::c_char,
        s2: *const libc::c_char,
        n: size_t,
    ) -> libc::c_int;
    fn error(
        __status: libc::c_int,
        __errnum: libc::c_int,
        __format: *const libc::c_char,
        _: ...
    );
    fn file_name_concat(
        dir: *const libc::c_char,
        base: *const libc::c_char,
        base_in_result: *mut *mut libc::c_char,
    ) -> *mut libc::c_char;
    fn hard_locale(category: libc::c_int) -> bool;
    fn hash_get_n_entries(table: *const Hash_table) -> size_t;
    fn hash_free(table: *mut Hash_table);
    fn hash_initialize(
        candidate: size_t,
        tuning: *const Hash_tuning,
        hasher: Hash_hasher,
        comparator: Hash_comparator,
        data_freer: Hash_data_freer,
    ) -> *mut Hash_table;
    fn hash_insert(
        table: *mut Hash_table,
        entry: *const libc::c_void,
    ) -> *mut libc::c_void;
    fn hash_remove(
        table: *mut Hash_table,
        entry: *const libc::c_void,
    ) -> *mut libc::c_void;
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
    fn xstrtoumax(
        _: *const libc::c_char,
        _: *mut *mut libc::c_char,
        _: libc::c_int,
        _: *mut uintmax_t,
        _: *const libc::c_char,
    ) -> strtol_error;
    fn filemodestring(statp: *const stat, str: *mut libc::c_char);
    fn filevercmp(a: *const libc::c_char, b: *const libc::c_char) -> libc::c_int;
    fn getuser(uid: uid_t) -> *mut libc::c_char;
    fn getgroup(gid: gid_t) -> *mut libc::c_char;
    static mut ls_mode: libc::c_int;
    fn gnu_mbswidth(string: *const libc::c_char, flags: libc::c_int) -> libc::c_int;
    fn mbsnwidth(
        buf: *const libc::c_char,
        nbytes: size_t,
        flags: libc::c_int,
    ) -> libc::c_int;
    fn mpsort(
        _: *mut *const libc::c_void,
        _: size_t,
        _: Option::<
            unsafe extern "C" fn(*const libc::c_void, *const libc::c_void) -> libc::c_int,
        >,
    );
    fn _obstack_newchunk(_: *mut obstack, _: size_t);
    fn _obstack_begin(
        _: *mut obstack,
        _: size_t,
        _: size_t,
        _: Option::<unsafe extern "C" fn(size_t) -> *mut libc::c_void>,
        _: Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
    ) -> libc::c_int;
    fn nstrftime(
        _: *mut libc::c_char,
        _: size_t,
        _: *const libc::c_char,
        _: *const tm,
        __tz: timezone_t,
        __ns: libc::c_int,
    ) -> size_t;
    fn xnumtoumax(
        n_str: *const libc::c_char,
        base: libc::c_int,
        min: uintmax_t,
        max: uintmax_t,
        suffixes: *const libc::c_char,
        err: *const libc::c_char,
        err_exit: libc::c_int,
    ) -> uintmax_t;
    fn xstrtol_fatal(
        _: strtol_error,
        _: libc::c_int,
        _: libc::c_char,
        _: *const option,
        _: *const libc::c_char,
    );
    fn areadlink_with_size(
        filename: *const libc::c_char,
        size_hint: size_t,
    ) -> *mut libc::c_char;
    fn mbsalign(
        src: *const libc::c_char,
        dest: *mut libc::c_char,
        dest_size: size_t,
        width: *mut size_t,
        align: mbs_align_t,
        flags: libc::c_int,
    ) -> size_t;
    fn xgethostname() -> *mut libc::c_char;
    fn canonicalize_filename_mode(
        _: *const libc::c_char,
        _: canonicalize_mode_t,
    ) -> *mut libc::c_char;
}
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
pub type __syscall_slong_t = libc::c_long;
pub type __sig_atomic_t = libc::c_int;
pub type ino_t = __ino_t;
pub type dev_t = __dev_t;
pub type gid_t = __gid_t;
pub type mode_t = __mode_t;
pub type uid_t = __uid_t;
pub type off_t = __off_t;
pub type time_t = __time_t;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct winsize {
    pub ws_row: libc::c_ushort,
    pub ws_col: libc::c_ushort,
    pub ws_xpixel: libc::c_ushort,
    pub ws_ypixel: libc::c_ushort,
}
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
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct obstack {
    pub chunk_size: size_t,
    pub chunk: *mut _obstack_chunk,
    pub object_base: *mut libc::c_char,
    pub next_free: *mut libc::c_char,
    pub chunk_limit: *mut libc::c_char,
    pub temp: C2RustUnnamed_2,
    pub alignment_mask: size_t,
    pub chunkfun: C2RustUnnamed_1,
    pub freefun: C2RustUnnamed_0,
    pub extra_arg: *mut libc::c_void,
    #[bitfield(name = "use_extra_arg", ty = "libc::c_uint", bits = "0..=0")]
    #[bitfield(name = "maybe_empty_object", ty = "libc::c_uint", bits = "1..=1")]
    #[bitfield(name = "alloc_failed", ty = "libc::c_uint", bits = "2..=2")]
    pub use_extra_arg_maybe_empty_object_alloc_failed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 7],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_0 {
    pub plain: Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
    pub extra: Option::<
        unsafe extern "C" fn(*mut libc::c_void, *mut libc::c_void) -> (),
    >,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_1 {
    pub plain: Option::<unsafe extern "C" fn(size_t) -> *mut libc::c_void>,
    pub extra: Option::<
        unsafe extern "C" fn(*mut libc::c_void, size_t) -> *mut libc::c_void,
    >,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_2 {
    pub i: size_t,
    pub p: *mut libc::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _obstack_chunk {
    pub limit: *mut libc::c_char,
    pub prev: *mut _obstack_chunk,
    pub contents: [libc::c_char; 0],
}
pub type ptrdiff_t = libc::c_long;
pub type wchar_t = libc::c_uint;
pub type __jmp_buf = [libc::c_ulonglong; 22];
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __jmp_buf_tag {
    pub __jmpbuf: __jmp_buf,
    pub __mask_was_saved: libc::c_int,
    pub __saved_mask: __sigset_t,
}
pub type jmp_buf = [__jmp_buf_tag; 1];
#[derive(Copy, Clone)]
#[repr(C)]
pub struct option {
    pub name: *const libc::c_char,
    pub has_arg: libc::c_int,
    pub flag: *mut libc::c_int,
    pub val: libc::c_int,
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
    pub _sifields: C2RustUnnamed_3,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_3 {
    pub _pad: [libc::c_int; 28],
    pub _kill: C2RustUnnamed_12,
    pub _timer: C2RustUnnamed_11,
    pub _rt: C2RustUnnamed_10,
    pub _sigchld: C2RustUnnamed_9,
    pub _sigfault: C2RustUnnamed_6,
    pub _sigpoll: C2RustUnnamed_5,
    pub _sigsys: C2RustUnnamed_4,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_4 {
    pub _call_addr: *mut libc::c_void,
    pub _syscall: libc::c_int,
    pub _arch: libc::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_5 {
    pub si_band: libc::c_long,
    pub si_fd: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_6 {
    pub si_addr: *mut libc::c_void,
    pub si_addr_lsb: libc::c_short,
    pub _bounds: C2RustUnnamed_7,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_7 {
    pub _addr_bnd: C2RustUnnamed_8,
    pub _pkey: __uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_8 {
    pub _lower: *mut libc::c_void,
    pub _upper: *mut libc::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_9 {
    pub si_pid: __pid_t,
    pub si_uid: __uid_t,
    pub si_status: libc::c_int,
    pub si_utime: __clock_t,
    pub si_stime: __clock_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_10 {
    pub si_pid: __pid_t,
    pub si_uid: __uid_t,
    pub si_sigval: __sigval_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_11 {
    pub si_tid: libc::c_int,
    pub si_overrun: libc::c_int,
    pub si_sigval: __sigval_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_12 {
    pub si_pid: __pid_t,
    pub si_uid: __uid_t,
}
pub type __sighandler_t = Option::<unsafe extern "C" fn(libc::c_int) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sigaction {
    pub __sigaction_handler: C2RustUnnamed_13,
    pub sa_mask: __sigset_t,
    pub sa_flags: libc::c_int,
    pub sa_restorer: Option::<unsafe extern "C" fn() -> ()>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_13 {
    pub sa_handler: __sighandler_t,
    pub sa_sigaction: Option::<
        unsafe extern "C" fn(libc::c_int, *mut siginfo_t, *mut libc::c_void) -> (),
    >,
}
pub type __u16 = libc::c_ushort;
pub type __s32 = libc::c_int;
pub type __u32 = libc::c_uint;
pub type __s64 = libc::c_longlong;
pub type __u64 = libc::c_ulonglong;
pub type mbstate_t = __mbstate_t;
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
pub type nl_item = libc::c_int;
pub type C2RustUnnamed_14 = libc::c_uint;
pub const _NL_NUM: C2RustUnnamed_14 = 786449;
pub const _NL_NUM_LC_IDENTIFICATION: C2RustUnnamed_14 = 786448;
pub const _NL_IDENTIFICATION_CODESET: C2RustUnnamed_14 = 786447;
pub const _NL_IDENTIFICATION_CATEGORY: C2RustUnnamed_14 = 786446;
pub const _NL_IDENTIFICATION_DATE: C2RustUnnamed_14 = 786445;
pub const _NL_IDENTIFICATION_REVISION: C2RustUnnamed_14 = 786444;
pub const _NL_IDENTIFICATION_ABBREVIATION: C2RustUnnamed_14 = 786443;
pub const _NL_IDENTIFICATION_APPLICATION: C2RustUnnamed_14 = 786442;
pub const _NL_IDENTIFICATION_AUDIENCE: C2RustUnnamed_14 = 786441;
pub const _NL_IDENTIFICATION_TERRITORY: C2RustUnnamed_14 = 786440;
pub const _NL_IDENTIFICATION_LANGUAGE: C2RustUnnamed_14 = 786439;
pub const _NL_IDENTIFICATION_FAX: C2RustUnnamed_14 = 786438;
pub const _NL_IDENTIFICATION_TEL: C2RustUnnamed_14 = 786437;
pub const _NL_IDENTIFICATION_EMAIL: C2RustUnnamed_14 = 786436;
pub const _NL_IDENTIFICATION_CONTACT: C2RustUnnamed_14 = 786435;
pub const _NL_IDENTIFICATION_ADDRESS: C2RustUnnamed_14 = 786434;
pub const _NL_IDENTIFICATION_SOURCE: C2RustUnnamed_14 = 786433;
pub const _NL_IDENTIFICATION_TITLE: C2RustUnnamed_14 = 786432;
pub const _NL_NUM_LC_MEASUREMENT: C2RustUnnamed_14 = 720898;
pub const _NL_MEASUREMENT_CODESET: C2RustUnnamed_14 = 720897;
pub const _NL_MEASUREMENT_MEASUREMENT: C2RustUnnamed_14 = 720896;
pub const _NL_NUM_LC_TELEPHONE: C2RustUnnamed_14 = 655365;
pub const _NL_TELEPHONE_CODESET: C2RustUnnamed_14 = 655364;
pub const _NL_TELEPHONE_INT_PREFIX: C2RustUnnamed_14 = 655363;
pub const _NL_TELEPHONE_INT_SELECT: C2RustUnnamed_14 = 655362;
pub const _NL_TELEPHONE_TEL_DOM_FMT: C2RustUnnamed_14 = 655361;
pub const _NL_TELEPHONE_TEL_INT_FMT: C2RustUnnamed_14 = 655360;
pub const _NL_NUM_LC_ADDRESS: C2RustUnnamed_14 = 589837;
pub const _NL_ADDRESS_CODESET: C2RustUnnamed_14 = 589836;
pub const _NL_ADDRESS_LANG_LIB: C2RustUnnamed_14 = 589835;
pub const _NL_ADDRESS_LANG_TERM: C2RustUnnamed_14 = 589834;
pub const _NL_ADDRESS_LANG_AB: C2RustUnnamed_14 = 589833;
pub const _NL_ADDRESS_LANG_NAME: C2RustUnnamed_14 = 589832;
pub const _NL_ADDRESS_COUNTRY_ISBN: C2RustUnnamed_14 = 589831;
pub const _NL_ADDRESS_COUNTRY_NUM: C2RustUnnamed_14 = 589830;
pub const _NL_ADDRESS_COUNTRY_CAR: C2RustUnnamed_14 = 589829;
pub const _NL_ADDRESS_COUNTRY_AB3: C2RustUnnamed_14 = 589828;
pub const _NL_ADDRESS_COUNTRY_AB2: C2RustUnnamed_14 = 589827;
pub const _NL_ADDRESS_COUNTRY_POST: C2RustUnnamed_14 = 589826;
pub const _NL_ADDRESS_COUNTRY_NAME: C2RustUnnamed_14 = 589825;
pub const _NL_ADDRESS_POSTAL_FMT: C2RustUnnamed_14 = 589824;
pub const _NL_NUM_LC_NAME: C2RustUnnamed_14 = 524295;
pub const _NL_NAME_CODESET: C2RustUnnamed_14 = 524294;
pub const _NL_NAME_NAME_MS: C2RustUnnamed_14 = 524293;
pub const _NL_NAME_NAME_MISS: C2RustUnnamed_14 = 524292;
pub const _NL_NAME_NAME_MRS: C2RustUnnamed_14 = 524291;
pub const _NL_NAME_NAME_MR: C2RustUnnamed_14 = 524290;
pub const _NL_NAME_NAME_GEN: C2RustUnnamed_14 = 524289;
pub const _NL_NAME_NAME_FMT: C2RustUnnamed_14 = 524288;
pub const _NL_NUM_LC_PAPER: C2RustUnnamed_14 = 458755;
pub const _NL_PAPER_CODESET: C2RustUnnamed_14 = 458754;
pub const _NL_PAPER_WIDTH: C2RustUnnamed_14 = 458753;
pub const _NL_PAPER_HEIGHT: C2RustUnnamed_14 = 458752;
pub const _NL_NUM_LC_MESSAGES: C2RustUnnamed_14 = 327685;
pub const _NL_MESSAGES_CODESET: C2RustUnnamed_14 = 327684;
pub const __NOSTR: C2RustUnnamed_14 = 327683;
pub const __YESSTR: C2RustUnnamed_14 = 327682;
pub const __NOEXPR: C2RustUnnamed_14 = 327681;
pub const __YESEXPR: C2RustUnnamed_14 = 327680;
pub const _NL_NUM_LC_NUMERIC: C2RustUnnamed_14 = 65542;
pub const _NL_NUMERIC_CODESET: C2RustUnnamed_14 = 65541;
pub const _NL_NUMERIC_THOUSANDS_SEP_WC: C2RustUnnamed_14 = 65540;
pub const _NL_NUMERIC_DECIMAL_POINT_WC: C2RustUnnamed_14 = 65539;
pub const __GROUPING: C2RustUnnamed_14 = 65538;
pub const THOUSEP: C2RustUnnamed_14 = 65537;
pub const __THOUSANDS_SEP: C2RustUnnamed_14 = 65537;
pub const RADIXCHAR: C2RustUnnamed_14 = 65536;
pub const __DECIMAL_POINT: C2RustUnnamed_14 = 65536;
pub const _NL_NUM_LC_MONETARY: C2RustUnnamed_14 = 262190;
pub const _NL_MONETARY_CODESET: C2RustUnnamed_14 = 262189;
pub const _NL_MONETARY_THOUSANDS_SEP_WC: C2RustUnnamed_14 = 262188;
pub const _NL_MONETARY_DECIMAL_POINT_WC: C2RustUnnamed_14 = 262187;
pub const _NL_MONETARY_CONVERSION_RATE: C2RustUnnamed_14 = 262186;
pub const _NL_MONETARY_DUO_VALID_TO: C2RustUnnamed_14 = 262185;
pub const _NL_MONETARY_DUO_VALID_FROM: C2RustUnnamed_14 = 262184;
pub const _NL_MONETARY_UNO_VALID_TO: C2RustUnnamed_14 = 262183;
pub const _NL_MONETARY_UNO_VALID_FROM: C2RustUnnamed_14 = 262182;
pub const _NL_MONETARY_DUO_INT_N_SIGN_POSN: C2RustUnnamed_14 = 262181;
pub const _NL_MONETARY_DUO_INT_P_SIGN_POSN: C2RustUnnamed_14 = 262180;
pub const _NL_MONETARY_DUO_N_SIGN_POSN: C2RustUnnamed_14 = 262179;
pub const _NL_MONETARY_DUO_P_SIGN_POSN: C2RustUnnamed_14 = 262178;
pub const _NL_MONETARY_DUO_INT_N_SEP_BY_SPACE: C2RustUnnamed_14 = 262177;
pub const _NL_MONETARY_DUO_INT_N_CS_PRECEDES: C2RustUnnamed_14 = 262176;
pub const _NL_MONETARY_DUO_INT_P_SEP_BY_SPACE: C2RustUnnamed_14 = 262175;
pub const _NL_MONETARY_DUO_INT_P_CS_PRECEDES: C2RustUnnamed_14 = 262174;
pub const _NL_MONETARY_DUO_N_SEP_BY_SPACE: C2RustUnnamed_14 = 262173;
pub const _NL_MONETARY_DUO_N_CS_PRECEDES: C2RustUnnamed_14 = 262172;
pub const _NL_MONETARY_DUO_P_SEP_BY_SPACE: C2RustUnnamed_14 = 262171;
pub const _NL_MONETARY_DUO_P_CS_PRECEDES: C2RustUnnamed_14 = 262170;
pub const _NL_MONETARY_DUO_FRAC_DIGITS: C2RustUnnamed_14 = 262169;
pub const _NL_MONETARY_DUO_INT_FRAC_DIGITS: C2RustUnnamed_14 = 262168;
pub const _NL_MONETARY_DUO_CURRENCY_SYMBOL: C2RustUnnamed_14 = 262167;
pub const _NL_MONETARY_DUO_INT_CURR_SYMBOL: C2RustUnnamed_14 = 262166;
pub const __INT_N_SIGN_POSN: C2RustUnnamed_14 = 262165;
pub const __INT_P_SIGN_POSN: C2RustUnnamed_14 = 262164;
pub const __INT_N_SEP_BY_SPACE: C2RustUnnamed_14 = 262163;
pub const __INT_N_CS_PRECEDES: C2RustUnnamed_14 = 262162;
pub const __INT_P_SEP_BY_SPACE: C2RustUnnamed_14 = 262161;
pub const __INT_P_CS_PRECEDES: C2RustUnnamed_14 = 262160;
pub const _NL_MONETARY_CRNCYSTR: C2RustUnnamed_14 = 262159;
pub const __N_SIGN_POSN: C2RustUnnamed_14 = 262158;
pub const __P_SIGN_POSN: C2RustUnnamed_14 = 262157;
pub const __N_SEP_BY_SPACE: C2RustUnnamed_14 = 262156;
pub const __N_CS_PRECEDES: C2RustUnnamed_14 = 262155;
pub const __P_SEP_BY_SPACE: C2RustUnnamed_14 = 262154;
pub const __P_CS_PRECEDES: C2RustUnnamed_14 = 262153;
pub const __FRAC_DIGITS: C2RustUnnamed_14 = 262152;
pub const __INT_FRAC_DIGITS: C2RustUnnamed_14 = 262151;
pub const __NEGATIVE_SIGN: C2RustUnnamed_14 = 262150;
pub const __POSITIVE_SIGN: C2RustUnnamed_14 = 262149;
pub const __MON_GROUPING: C2RustUnnamed_14 = 262148;
pub const __MON_THOUSANDS_SEP: C2RustUnnamed_14 = 262147;
pub const __MON_DECIMAL_POINT: C2RustUnnamed_14 = 262146;
pub const __CURRENCY_SYMBOL: C2RustUnnamed_14 = 262145;
pub const __INT_CURR_SYMBOL: C2RustUnnamed_14 = 262144;
pub const _NL_NUM_LC_CTYPE: C2RustUnnamed_14 = 86;
pub const _NL_CTYPE_EXTRA_MAP_14: C2RustUnnamed_14 = 85;
pub const _NL_CTYPE_EXTRA_MAP_13: C2RustUnnamed_14 = 84;
pub const _NL_CTYPE_EXTRA_MAP_12: C2RustUnnamed_14 = 83;
pub const _NL_CTYPE_EXTRA_MAP_11: C2RustUnnamed_14 = 82;
pub const _NL_CTYPE_EXTRA_MAP_10: C2RustUnnamed_14 = 81;
pub const _NL_CTYPE_EXTRA_MAP_9: C2RustUnnamed_14 = 80;
pub const _NL_CTYPE_EXTRA_MAP_8: C2RustUnnamed_14 = 79;
pub const _NL_CTYPE_EXTRA_MAP_7: C2RustUnnamed_14 = 78;
pub const _NL_CTYPE_EXTRA_MAP_6: C2RustUnnamed_14 = 77;
pub const _NL_CTYPE_EXTRA_MAP_5: C2RustUnnamed_14 = 76;
pub const _NL_CTYPE_EXTRA_MAP_4: C2RustUnnamed_14 = 75;
pub const _NL_CTYPE_EXTRA_MAP_3: C2RustUnnamed_14 = 74;
pub const _NL_CTYPE_EXTRA_MAP_2: C2RustUnnamed_14 = 73;
pub const _NL_CTYPE_EXTRA_MAP_1: C2RustUnnamed_14 = 72;
pub const _NL_CTYPE_NONASCII_CASE: C2RustUnnamed_14 = 71;
pub const _NL_CTYPE_MAP_TO_NONASCII: C2RustUnnamed_14 = 70;
pub const _NL_CTYPE_TRANSLIT_IGNORE: C2RustUnnamed_14 = 69;
pub const _NL_CTYPE_TRANSLIT_IGNORE_LEN: C2RustUnnamed_14 = 68;
pub const _NL_CTYPE_TRANSLIT_DEFAULT_MISSING: C2RustUnnamed_14 = 67;
pub const _NL_CTYPE_TRANSLIT_DEFAULT_MISSING_LEN: C2RustUnnamed_14 = 66;
pub const _NL_CTYPE_TRANSLIT_TO_TBL: C2RustUnnamed_14 = 65;
pub const _NL_CTYPE_TRANSLIT_TO_IDX: C2RustUnnamed_14 = 64;
pub const _NL_CTYPE_TRANSLIT_FROM_TBL: C2RustUnnamed_14 = 63;
pub const _NL_CTYPE_TRANSLIT_FROM_IDX: C2RustUnnamed_14 = 62;
pub const _NL_CTYPE_TRANSLIT_TAB_SIZE: C2RustUnnamed_14 = 61;
pub const _NL_CTYPE_OUTDIGIT9_WC: C2RustUnnamed_14 = 60;
pub const _NL_CTYPE_OUTDIGIT8_WC: C2RustUnnamed_14 = 59;
pub const _NL_CTYPE_OUTDIGIT7_WC: C2RustUnnamed_14 = 58;
pub const _NL_CTYPE_OUTDIGIT6_WC: C2RustUnnamed_14 = 57;
pub const _NL_CTYPE_OUTDIGIT5_WC: C2RustUnnamed_14 = 56;
pub const _NL_CTYPE_OUTDIGIT4_WC: C2RustUnnamed_14 = 55;
pub const _NL_CTYPE_OUTDIGIT3_WC: C2RustUnnamed_14 = 54;
pub const _NL_CTYPE_OUTDIGIT2_WC: C2RustUnnamed_14 = 53;
pub const _NL_CTYPE_OUTDIGIT1_WC: C2RustUnnamed_14 = 52;
pub const _NL_CTYPE_OUTDIGIT0_WC: C2RustUnnamed_14 = 51;
pub const _NL_CTYPE_OUTDIGIT9_MB: C2RustUnnamed_14 = 50;
pub const _NL_CTYPE_OUTDIGIT8_MB: C2RustUnnamed_14 = 49;
pub const _NL_CTYPE_OUTDIGIT7_MB: C2RustUnnamed_14 = 48;
pub const _NL_CTYPE_OUTDIGIT6_MB: C2RustUnnamed_14 = 47;
pub const _NL_CTYPE_OUTDIGIT5_MB: C2RustUnnamed_14 = 46;
pub const _NL_CTYPE_OUTDIGIT4_MB: C2RustUnnamed_14 = 45;
pub const _NL_CTYPE_OUTDIGIT3_MB: C2RustUnnamed_14 = 44;
pub const _NL_CTYPE_OUTDIGIT2_MB: C2RustUnnamed_14 = 43;
pub const _NL_CTYPE_OUTDIGIT1_MB: C2RustUnnamed_14 = 42;
pub const _NL_CTYPE_OUTDIGIT0_MB: C2RustUnnamed_14 = 41;
pub const _NL_CTYPE_INDIGITS9_WC: C2RustUnnamed_14 = 40;
pub const _NL_CTYPE_INDIGITS8_WC: C2RustUnnamed_14 = 39;
pub const _NL_CTYPE_INDIGITS7_WC: C2RustUnnamed_14 = 38;
pub const _NL_CTYPE_INDIGITS6_WC: C2RustUnnamed_14 = 37;
pub const _NL_CTYPE_INDIGITS5_WC: C2RustUnnamed_14 = 36;
pub const _NL_CTYPE_INDIGITS4_WC: C2RustUnnamed_14 = 35;
pub const _NL_CTYPE_INDIGITS3_WC: C2RustUnnamed_14 = 34;
pub const _NL_CTYPE_INDIGITS2_WC: C2RustUnnamed_14 = 33;
pub const _NL_CTYPE_INDIGITS1_WC: C2RustUnnamed_14 = 32;
pub const _NL_CTYPE_INDIGITS0_WC: C2RustUnnamed_14 = 31;
pub const _NL_CTYPE_INDIGITS_WC_LEN: C2RustUnnamed_14 = 30;
pub const _NL_CTYPE_INDIGITS9_MB: C2RustUnnamed_14 = 29;
pub const _NL_CTYPE_INDIGITS8_MB: C2RustUnnamed_14 = 28;
pub const _NL_CTYPE_INDIGITS7_MB: C2RustUnnamed_14 = 27;
pub const _NL_CTYPE_INDIGITS6_MB: C2RustUnnamed_14 = 26;
pub const _NL_CTYPE_INDIGITS5_MB: C2RustUnnamed_14 = 25;
pub const _NL_CTYPE_INDIGITS4_MB: C2RustUnnamed_14 = 24;
pub const _NL_CTYPE_INDIGITS3_MB: C2RustUnnamed_14 = 23;
pub const _NL_CTYPE_INDIGITS2_MB: C2RustUnnamed_14 = 22;
pub const _NL_CTYPE_INDIGITS1_MB: C2RustUnnamed_14 = 21;
pub const _NL_CTYPE_INDIGITS0_MB: C2RustUnnamed_14 = 20;
pub const _NL_CTYPE_INDIGITS_MB_LEN: C2RustUnnamed_14 = 19;
pub const _NL_CTYPE_MAP_OFFSET: C2RustUnnamed_14 = 18;
pub const _NL_CTYPE_CLASS_OFFSET: C2RustUnnamed_14 = 17;
pub const _NL_CTYPE_TOLOWER32: C2RustUnnamed_14 = 16;
pub const _NL_CTYPE_TOUPPER32: C2RustUnnamed_14 = 15;
pub const CODESET: C2RustUnnamed_14 = 14;
pub const _NL_CTYPE_CODESET_NAME: C2RustUnnamed_14 = 14;
pub const _NL_CTYPE_MB_CUR_MAX: C2RustUnnamed_14 = 13;
pub const _NL_CTYPE_WIDTH: C2RustUnnamed_14 = 12;
pub const _NL_CTYPE_MAP_NAMES: C2RustUnnamed_14 = 11;
pub const _NL_CTYPE_CLASS_NAMES: C2RustUnnamed_14 = 10;
pub const _NL_CTYPE_GAP6: C2RustUnnamed_14 = 9;
pub const _NL_CTYPE_GAP5: C2RustUnnamed_14 = 8;
pub const _NL_CTYPE_GAP4: C2RustUnnamed_14 = 7;
pub const _NL_CTYPE_GAP3: C2RustUnnamed_14 = 6;
pub const _NL_CTYPE_CLASS32: C2RustUnnamed_14 = 5;
pub const _NL_CTYPE_GAP2: C2RustUnnamed_14 = 4;
pub const _NL_CTYPE_TOLOWER: C2RustUnnamed_14 = 3;
pub const _NL_CTYPE_GAP1: C2RustUnnamed_14 = 2;
pub const _NL_CTYPE_TOUPPER: C2RustUnnamed_14 = 1;
pub const _NL_CTYPE_CLASS: C2RustUnnamed_14 = 0;
pub const _NL_NUM_LC_COLLATE: C2RustUnnamed_14 = 196627;
pub const _NL_COLLATE_CODESET: C2RustUnnamed_14 = 196626;
pub const _NL_COLLATE_COLLSEQWC: C2RustUnnamed_14 = 196625;
pub const _NL_COLLATE_COLLSEQMB: C2RustUnnamed_14 = 196624;
pub const _NL_COLLATE_SYMB_EXTRAMB: C2RustUnnamed_14 = 196623;
pub const _NL_COLLATE_SYMB_TABLEMB: C2RustUnnamed_14 = 196622;
pub const _NL_COLLATE_SYMB_HASH_SIZEMB: C2RustUnnamed_14 = 196621;
pub const _NL_COLLATE_INDIRECTWC: C2RustUnnamed_14 = 196620;
pub const _NL_COLLATE_EXTRAWC: C2RustUnnamed_14 = 196619;
pub const _NL_COLLATE_WEIGHTWC: C2RustUnnamed_14 = 196618;
pub const _NL_COLLATE_TABLEWC: C2RustUnnamed_14 = 196617;
pub const _NL_COLLATE_GAP3: C2RustUnnamed_14 = 196616;
pub const _NL_COLLATE_GAP2: C2RustUnnamed_14 = 196615;
pub const _NL_COLLATE_GAP1: C2RustUnnamed_14 = 196614;
pub const _NL_COLLATE_INDIRECTMB: C2RustUnnamed_14 = 196613;
pub const _NL_COLLATE_EXTRAMB: C2RustUnnamed_14 = 196612;
pub const _NL_COLLATE_WEIGHTMB: C2RustUnnamed_14 = 196611;
pub const _NL_COLLATE_TABLEMB: C2RustUnnamed_14 = 196610;
pub const _NL_COLLATE_RULESETS: C2RustUnnamed_14 = 196609;
pub const _NL_COLLATE_NRULES: C2RustUnnamed_14 = 196608;
pub const _NL_NUM_LC_TIME: C2RustUnnamed_14 = 131231;
pub const _NL_WABALTMON_12: C2RustUnnamed_14 = 131230;
pub const _NL_WABALTMON_11: C2RustUnnamed_14 = 131229;
pub const _NL_WABALTMON_10: C2RustUnnamed_14 = 131228;
pub const _NL_WABALTMON_9: C2RustUnnamed_14 = 131227;
pub const _NL_WABALTMON_8: C2RustUnnamed_14 = 131226;
pub const _NL_WABALTMON_7: C2RustUnnamed_14 = 131225;
pub const _NL_WABALTMON_6: C2RustUnnamed_14 = 131224;
pub const _NL_WABALTMON_5: C2RustUnnamed_14 = 131223;
pub const _NL_WABALTMON_4: C2RustUnnamed_14 = 131222;
pub const _NL_WABALTMON_3: C2RustUnnamed_14 = 131221;
pub const _NL_WABALTMON_2: C2RustUnnamed_14 = 131220;
pub const _NL_WABALTMON_1: C2RustUnnamed_14 = 131219;
pub const _NL_ABALTMON_12: C2RustUnnamed_14 = 131218;
pub const _NL_ABALTMON_11: C2RustUnnamed_14 = 131217;
pub const _NL_ABALTMON_10: C2RustUnnamed_14 = 131216;
pub const _NL_ABALTMON_9: C2RustUnnamed_14 = 131215;
pub const _NL_ABALTMON_8: C2RustUnnamed_14 = 131214;
pub const _NL_ABALTMON_7: C2RustUnnamed_14 = 131213;
pub const _NL_ABALTMON_6: C2RustUnnamed_14 = 131212;
pub const _NL_ABALTMON_5: C2RustUnnamed_14 = 131211;
pub const _NL_ABALTMON_4: C2RustUnnamed_14 = 131210;
pub const _NL_ABALTMON_3: C2RustUnnamed_14 = 131209;
pub const _NL_ABALTMON_2: C2RustUnnamed_14 = 131208;
pub const _NL_ABALTMON_1: C2RustUnnamed_14 = 131207;
pub const _NL_WALTMON_12: C2RustUnnamed_14 = 131206;
pub const _NL_WALTMON_11: C2RustUnnamed_14 = 131205;
pub const _NL_WALTMON_10: C2RustUnnamed_14 = 131204;
pub const _NL_WALTMON_9: C2RustUnnamed_14 = 131203;
pub const _NL_WALTMON_8: C2RustUnnamed_14 = 131202;
pub const _NL_WALTMON_7: C2RustUnnamed_14 = 131201;
pub const _NL_WALTMON_6: C2RustUnnamed_14 = 131200;
pub const _NL_WALTMON_5: C2RustUnnamed_14 = 131199;
pub const _NL_WALTMON_4: C2RustUnnamed_14 = 131198;
pub const _NL_WALTMON_3: C2RustUnnamed_14 = 131197;
pub const _NL_WALTMON_2: C2RustUnnamed_14 = 131196;
pub const _NL_WALTMON_1: C2RustUnnamed_14 = 131195;
pub const __ALTMON_12: C2RustUnnamed_14 = 131194;
pub const __ALTMON_11: C2RustUnnamed_14 = 131193;
pub const __ALTMON_10: C2RustUnnamed_14 = 131192;
pub const __ALTMON_9: C2RustUnnamed_14 = 131191;
pub const __ALTMON_8: C2RustUnnamed_14 = 131190;
pub const __ALTMON_7: C2RustUnnamed_14 = 131189;
pub const __ALTMON_6: C2RustUnnamed_14 = 131188;
pub const __ALTMON_5: C2RustUnnamed_14 = 131187;
pub const __ALTMON_4: C2RustUnnamed_14 = 131186;
pub const __ALTMON_3: C2RustUnnamed_14 = 131185;
pub const __ALTMON_2: C2RustUnnamed_14 = 131184;
pub const __ALTMON_1: C2RustUnnamed_14 = 131183;
pub const _NL_TIME_CODESET: C2RustUnnamed_14 = 131182;
pub const _NL_W_DATE_FMT: C2RustUnnamed_14 = 131181;
pub const _DATE_FMT: C2RustUnnamed_14 = 131180;
pub const _NL_TIME_TIMEZONE: C2RustUnnamed_14 = 131179;
pub const _NL_TIME_CAL_DIRECTION: C2RustUnnamed_14 = 131178;
pub const _NL_TIME_FIRST_WORKDAY: C2RustUnnamed_14 = 131177;
pub const _NL_TIME_FIRST_WEEKDAY: C2RustUnnamed_14 = 131176;
pub const _NL_TIME_WEEK_1STWEEK: C2RustUnnamed_14 = 131175;
pub const _NL_TIME_WEEK_1STDAY: C2RustUnnamed_14 = 131174;
pub const _NL_TIME_WEEK_NDAYS: C2RustUnnamed_14 = 131173;
pub const _NL_WERA_T_FMT: C2RustUnnamed_14 = 131172;
pub const _NL_WERA_D_T_FMT: C2RustUnnamed_14 = 131171;
pub const _NL_WALT_DIGITS: C2RustUnnamed_14 = 131170;
pub const _NL_WERA_D_FMT: C2RustUnnamed_14 = 131169;
pub const _NL_WERA_YEAR: C2RustUnnamed_14 = 131168;
pub const _NL_WT_FMT_AMPM: C2RustUnnamed_14 = 131167;
pub const _NL_WT_FMT: C2RustUnnamed_14 = 131166;
pub const _NL_WD_FMT: C2RustUnnamed_14 = 131165;
pub const _NL_WD_T_FMT: C2RustUnnamed_14 = 131164;
pub const _NL_WPM_STR: C2RustUnnamed_14 = 131163;
pub const _NL_WAM_STR: C2RustUnnamed_14 = 131162;
pub const _NL_WMON_12: C2RustUnnamed_14 = 131161;
pub const _NL_WMON_11: C2RustUnnamed_14 = 131160;
pub const _NL_WMON_10: C2RustUnnamed_14 = 131159;
pub const _NL_WMON_9: C2RustUnnamed_14 = 131158;
pub const _NL_WMON_8: C2RustUnnamed_14 = 131157;
pub const _NL_WMON_7: C2RustUnnamed_14 = 131156;
pub const _NL_WMON_6: C2RustUnnamed_14 = 131155;
pub const _NL_WMON_5: C2RustUnnamed_14 = 131154;
pub const _NL_WMON_4: C2RustUnnamed_14 = 131153;
pub const _NL_WMON_3: C2RustUnnamed_14 = 131152;
pub const _NL_WMON_2: C2RustUnnamed_14 = 131151;
pub const _NL_WMON_1: C2RustUnnamed_14 = 131150;
pub const _NL_WABMON_12: C2RustUnnamed_14 = 131149;
pub const _NL_WABMON_11: C2RustUnnamed_14 = 131148;
pub const _NL_WABMON_10: C2RustUnnamed_14 = 131147;
pub const _NL_WABMON_9: C2RustUnnamed_14 = 131146;
pub const _NL_WABMON_8: C2RustUnnamed_14 = 131145;
pub const _NL_WABMON_7: C2RustUnnamed_14 = 131144;
pub const _NL_WABMON_6: C2RustUnnamed_14 = 131143;
pub const _NL_WABMON_5: C2RustUnnamed_14 = 131142;
pub const _NL_WABMON_4: C2RustUnnamed_14 = 131141;
pub const _NL_WABMON_3: C2RustUnnamed_14 = 131140;
pub const _NL_WABMON_2: C2RustUnnamed_14 = 131139;
pub const _NL_WABMON_1: C2RustUnnamed_14 = 131138;
pub const _NL_WDAY_7: C2RustUnnamed_14 = 131137;
pub const _NL_WDAY_6: C2RustUnnamed_14 = 131136;
pub const _NL_WDAY_5: C2RustUnnamed_14 = 131135;
pub const _NL_WDAY_4: C2RustUnnamed_14 = 131134;
pub const _NL_WDAY_3: C2RustUnnamed_14 = 131133;
pub const _NL_WDAY_2: C2RustUnnamed_14 = 131132;
pub const _NL_WDAY_1: C2RustUnnamed_14 = 131131;
pub const _NL_WABDAY_7: C2RustUnnamed_14 = 131130;
pub const _NL_WABDAY_6: C2RustUnnamed_14 = 131129;
pub const _NL_WABDAY_5: C2RustUnnamed_14 = 131128;
pub const _NL_WABDAY_4: C2RustUnnamed_14 = 131127;
pub const _NL_WABDAY_3: C2RustUnnamed_14 = 131126;
pub const _NL_WABDAY_2: C2RustUnnamed_14 = 131125;
pub const _NL_WABDAY_1: C2RustUnnamed_14 = 131124;
pub const _NL_TIME_ERA_ENTRIES: C2RustUnnamed_14 = 131123;
pub const _NL_TIME_ERA_NUM_ENTRIES: C2RustUnnamed_14 = 131122;
pub const ERA_T_FMT: C2RustUnnamed_14 = 131121;
pub const ERA_D_T_FMT: C2RustUnnamed_14 = 131120;
pub const ALT_DIGITS: C2RustUnnamed_14 = 131119;
pub const ERA_D_FMT: C2RustUnnamed_14 = 131118;
pub const __ERA_YEAR: C2RustUnnamed_14 = 131117;
pub const ERA: C2RustUnnamed_14 = 131116;
pub const T_FMT_AMPM: C2RustUnnamed_14 = 131115;
pub const T_FMT: C2RustUnnamed_14 = 131114;
pub const D_FMT: C2RustUnnamed_14 = 131113;
pub const D_T_FMT: C2RustUnnamed_14 = 131112;
pub const PM_STR: C2RustUnnamed_14 = 131111;
pub const AM_STR: C2RustUnnamed_14 = 131110;
pub const MON_12: C2RustUnnamed_14 = 131109;
pub const MON_11: C2RustUnnamed_14 = 131108;
pub const MON_10: C2RustUnnamed_14 = 131107;
pub const MON_9: C2RustUnnamed_14 = 131106;
pub const MON_8: C2RustUnnamed_14 = 131105;
pub const MON_7: C2RustUnnamed_14 = 131104;
pub const MON_6: C2RustUnnamed_14 = 131103;
pub const MON_5: C2RustUnnamed_14 = 131102;
pub const MON_4: C2RustUnnamed_14 = 131101;
pub const MON_3: C2RustUnnamed_14 = 131100;
pub const MON_2: C2RustUnnamed_14 = 131099;
pub const MON_1: C2RustUnnamed_14 = 131098;
pub const ABMON_12: C2RustUnnamed_14 = 131097;
pub const ABMON_11: C2RustUnnamed_14 = 131096;
pub const ABMON_10: C2RustUnnamed_14 = 131095;
pub const ABMON_9: C2RustUnnamed_14 = 131094;
pub const ABMON_8: C2RustUnnamed_14 = 131093;
pub const ABMON_7: C2RustUnnamed_14 = 131092;
pub const ABMON_6: C2RustUnnamed_14 = 131091;
pub const ABMON_5: C2RustUnnamed_14 = 131090;
pub const ABMON_4: C2RustUnnamed_14 = 131089;
pub const ABMON_3: C2RustUnnamed_14 = 131088;
pub const ABMON_2: C2RustUnnamed_14 = 131087;
pub const ABMON_1: C2RustUnnamed_14 = 131086;
pub const DAY_7: C2RustUnnamed_14 = 131085;
pub const DAY_6: C2RustUnnamed_14 = 131084;
pub const DAY_5: C2RustUnnamed_14 = 131083;
pub const DAY_4: C2RustUnnamed_14 = 131082;
pub const DAY_3: C2RustUnnamed_14 = 131081;
pub const DAY_2: C2RustUnnamed_14 = 131080;
pub const DAY_1: C2RustUnnamed_14 = 131079;
pub const ABDAY_7: C2RustUnnamed_14 = 131078;
pub const ABDAY_6: C2RustUnnamed_14 = 131077;
pub const ABDAY_5: C2RustUnnamed_14 = 131076;
pub const ABDAY_4: C2RustUnnamed_14 = 131075;
pub const ABDAY_3: C2RustUnnamed_14 = 131074;
pub const ABDAY_2: C2RustUnnamed_14 = 131073;
pub const ABDAY_1: C2RustUnnamed_14 = 131072;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dirent {
    pub d_ino: __ino_t,
    pub d_off: __off_t,
    pub d_reclen: libc::c_ushort,
    pub d_type: libc::c_uchar,
    pub d_name: [libc::c_char; 256],
}
pub type C2RustUnnamed_15 = libc::c_uint;
pub const DT_WHT: C2RustUnnamed_15 = 14;
pub const DT_SOCK: C2RustUnnamed_15 = 12;
pub const DT_LNK: C2RustUnnamed_15 = 10;
pub const DT_REG: C2RustUnnamed_15 = 8;
pub const DT_BLK: C2RustUnnamed_15 = 6;
pub const DT_DIR: C2RustUnnamed_15 = 4;
pub const DT_CHR: C2RustUnnamed_15 = 2;
pub const DT_FIFO: C2RustUnnamed_15 = 1;
pub const DT_UNKNOWN: C2RustUnnamed_15 = 0;
pub type DIR = __dirstream;
pub type C2RustUnnamed_16 = libc::c_uint;
pub const NOT_AN_INODE_NUMBER: C2RustUnnamed_16 = 0;
pub type intmax_t = __intmax_t;
pub type uintmax_t = __uintmax_t;
pub type C2RustUnnamed_17 = libc::c_uint;
pub const _ISalnum: C2RustUnnamed_17 = 8;
pub const _ISpunct: C2RustUnnamed_17 = 4;
pub const _IScntrl: C2RustUnnamed_17 = 2;
pub const _ISblank: C2RustUnnamed_17 = 1;
pub const _ISgraph: C2RustUnnamed_17 = 32768;
pub const _ISprint: C2RustUnnamed_17 = 16384;
pub const _ISspace: C2RustUnnamed_17 = 8192;
pub const _ISxdigit: C2RustUnnamed_17 = 4096;
pub const _ISdigit: C2RustUnnamed_17 = 2048;
pub const _ISalpha: C2RustUnnamed_17 = 1024;
pub const _ISlower: C2RustUnnamed_17 = 512;
pub const _ISupper: C2RustUnnamed_17 = 256;
pub type C2RustUnnamed_18 = libc::c_int;
pub const GETOPT_VERSION_CHAR: C2RustUnnamed_18 = -3;
pub const GETOPT_HELP_CHAR: C2RustUnnamed_18 = -2;
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
pub type strtol_error = libc::c_uint;
pub const LONGINT_INVALID: strtol_error = 4;
pub const LONGINT_INVALID_SUFFIX_CHAR_WITH_OVERFLOW: strtol_error = 3;
pub const LONGINT_INVALID_SUFFIX_CHAR: strtol_error = 2;
pub const LONGINT_OVERFLOW: strtol_error = 1;
pub const LONGINT_OK: strtol_error = 0;
pub type C2RustUnnamed_19 = libc::c_uint;
pub const human_B: C2RustUnnamed_19 = 256;
pub const human_SI: C2RustUnnamed_19 = 128;
pub const human_space_before_unit: C2RustUnnamed_19 = 64;
pub const human_base_1024: C2RustUnnamed_19 = 32;
pub const human_autoscale: C2RustUnnamed_19 = 16;
pub const human_suppress_point_zero: C2RustUnnamed_19 = 8;
pub const human_group_digits: C2RustUnnamed_19 = 4;
pub const human_floor: C2RustUnnamed_19 = 2;
pub const human_round_to_nearest: C2RustUnnamed_19 = 1;
pub const human_ceiling: C2RustUnnamed_19 = 0;
pub type mbs_align_t = libc::c_uint;
pub const MBS_ALIGN_CENTER: mbs_align_t = 2;
pub const MBS_ALIGN_RIGHT: mbs_align_t = 1;
pub const MBS_ALIGN_LEFT: mbs_align_t = 0;
pub type canonicalize_mode_t = libc::c_uint;
pub const CAN_NOLINKS: canonicalize_mode_t = 4;
pub const CAN_MISSING: canonicalize_mode_t = 2;
pub const CAN_ALL_BUT_LAST: canonicalize_mode_t = 1;
pub const CAN_EXISTING: canonicalize_mode_t = 0;
pub type filetype = libc::c_uint;
pub const arg_directory: filetype = 9;
pub const whiteout: filetype = 8;
pub const sock: filetype = 7;
pub const symbolic_link: filetype = 6;
pub const normal: filetype = 5;
pub const blockdev: filetype = 4;
pub const directory: filetype = 3;
pub const chardev: filetype = 2;
pub const fifo: filetype = 1;
pub const unknown: filetype = 0;
pub type acl_type = libc::c_uint;
pub const ACL_T_YES: acl_type = 2;
pub const ACL_T_LSM_CONTEXT_ONLY: acl_type = 1;
pub const ACL_T_NONE: acl_type = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct fileinfo {
    pub name: *mut libc::c_char,
    pub linkname: *mut libc::c_char,
    pub absolute_name: *mut libc::c_char,
    pub stat: stat,
    pub filetype: filetype,
    pub linkmode: mode_t,
    pub scontext: *mut libc::c_char,
    pub stat_ok: bool,
    pub linkok: bool,
    pub acl_type: acl_type,
    pub has_capability: bool,
    pub quoted: libc::c_int,
    pub width: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct bin_str {
    pub len: size_t,
    pub string: *const libc::c_char,
}
pub const C_RIGHT: indicator_no = 1;
pub const C_RESET: indicator_no = 3;
pub const C_LEFT: indicator_no = 0;
pub const C_END: indicator_no = 2;
pub const nsigs: C2RustUnnamed_20 = 12;
pub type C2RustUnnamed_20 = libc::c_uint;
pub type indicator_no = libc::c_uint;
pub const C_CLR_TO_EOL: indicator_no = 23;
pub const C_MULTIHARDLINK: indicator_no = 22;
pub const C_CAP: indicator_no = 21;
pub const C_STICKY_OTHER_WRITABLE: indicator_no = 20;
pub const C_OTHER_WRITABLE: indicator_no = 19;
pub const C_STICKY: indicator_no = 18;
pub const C_SETGID: indicator_no = 17;
pub const C_SETUID: indicator_no = 16;
pub const C_DOOR: indicator_no = 15;
pub const C_EXEC: indicator_no = 14;
pub const C_ORPHAN: indicator_no = 13;
pub const C_MISSING: indicator_no = 12;
pub const C_CHR: indicator_no = 11;
pub const C_BLK: indicator_no = 10;
pub const C_SOCK: indicator_no = 9;
pub const C_FIFO: indicator_no = 8;
pub const C_LINK: indicator_no = 7;
pub const C_DIR: indicator_no = 6;
pub const C_FILE: indicator_no = 5;
pub const C_NORM: indicator_no = 4;
pub const ABFORMAT_SIZE: C2RustUnnamed_32 = 128;
pub const MAX_MON_WIDTH: C2RustUnnamed_31 = 12;
pub const locale_time_style: time_style = 3;
pub const iso_time_style: time_style = 2;
pub const long_iso_time_style: time_style = 1;
pub const full_iso_time_style: time_style = 0;
pub type time_style = libc::c_uint;
pub const LS_FAILURE: C2RustUnnamed_28 = 2;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_21 {
    pub _gl_dummy: libc::c_int,
}
pub const long_format: format = 0;
pub type format = libc::c_uint;
pub const with_commas: format = 4;
pub const horizontal: format = 3;
pub const many_per_line: format = 2;
pub const one_per_line: format = 1;
pub type sort_type = libc::c_uint;
pub const sort_numtypes: sort_type = 7;
pub const sort_none: sort_type = 6;
pub const sort_time: sort_type = 5;
pub const sort_version: sort_type = 4;
pub const sort_size: sort_type = 3;
pub const sort_width: sort_type = 2;
pub const sort_extension: sort_type = 1;
pub const sort_name: sort_type = 0;
pub const time_btime: time_type = 3;
pub type time_type = libc::c_uint;
pub const time_numtypes: time_type = 4;
pub const time_atime: time_type = 2;
pub const time_ctime: time_type = 1;
pub const time_mtime: time_type = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_22 {
    pub _gl_dummy: libc::c_int,
}
pub const file_type: indicator_style = 2;
pub type indicator_style = libc::c_uint;
pub const classify: indicator_style = 3;
pub const slash: indicator_style = 1;
pub const none: indicator_style = 0;
pub const MIN_COLUMN_WIDTH: C2RustUnnamed_30 = 3;
pub const ZERO_OPTION: C2RustUnnamed_29 = 273;
pub const SI_OPTION: C2RustUnnamed_29 = 269;
pub const AUTHOR_OPTION: C2RustUnnamed_29 = 256;
pub const BLOCK_SIZE_OPTION: C2RustUnnamed_29 = 257;
pub const HYPERLINK_OPTION: C2RustUnnamed_29 = 265;
pub const COLOR_OPTION: C2RustUnnamed_29 = 258;
pub const TIME_STYLE_OPTION: C2RustUnnamed_29 = 272;
pub const TIME_OPTION: C2RustUnnamed_29 = 271;
pub const SORT_OPTION: C2RustUnnamed_29 = 270;
pub const SHOW_CONTROL_CHARS_OPTION: C2RustUnnamed_29 = 268;
pub const FORMAT_OPTION: C2RustUnnamed_29 = 261;
pub const QUOTING_STYLE_OPTION: C2RustUnnamed_29 = 267;
pub const INDICATOR_STYLE_OPTION: C2RustUnnamed_29 = 266;
pub const HIDE_OPTION: C2RustUnnamed_29 = 264;
pub const DEREFERENCE_COMMAND_LINE_SYMLINK_TO_DIR_OPTION: C2RustUnnamed_29 = 259;
pub const FILE_TYPE_INDICATOR_OPTION: C2RustUnnamed_29 = 260;
pub const GROUP_DIRECTORIES_FIRST_OPTION: C2RustUnnamed_29 = 263;
pub const FULL_TIME_OPTION: C2RustUnnamed_29 = 262;
pub const when_if_tty: when_type = 2;
pub const when_always: when_type = 1;
pub type when_type = libc::c_uint;
pub const when_never: when_type = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ignore_pattern {
    pub pattern: *const libc::c_char,
    pub next: *mut ignore_pattern,
}
pub type Dereference_symlink = libc::c_uint;
pub const DEREF_ALWAYS: Dereference_symlink = 4;
pub const DEREF_COMMAND_LINE_SYMLINK_TO_DIR: Dereference_symlink = 3;
pub const DEREF_COMMAND_LINE_ARGUMENTS: Dereference_symlink = 2;
pub const DEREF_NEVER: Dereference_symlink = 1;
pub const DEREF_UNDEFINED: Dereference_symlink = 0;
pub type C2RustUnnamed_23 = libc::c_uint;
pub const IGNORE_MINIMAL: C2RustUnnamed_23 = 2;
pub const IGNORE_DOT_AND_DOTDOT: C2RustUnnamed_23 = 1;
pub const IGNORE_DEFAULT: C2RustUnnamed_23 = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_24 {
    pub _gl_dummy: libc::c_int,
}
pub const LS_MINOR_PROBLEM: C2RustUnnamed_28 = 1;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct color_ext_type {
    pub ext: bin_str,
    pub seq: bin_str,
    pub next: *mut color_ext_type,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pending {
    pub name: *mut libc::c_char,
    pub realname: *mut libc::c_char,
    pub command_line_arg: bool,
    pub next: *mut pending,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct column_info {
    pub valid_len: bool,
    pub line_len: size_t,
    pub col_arr: *mut size_t,
}
pub const TIME_STAMP_LEN_MAXIMUM: C2RustUnnamed_27 = 1000;
pub type qsortFunc = Option::<unsafe extern "C" fn(V, V) -> libc::c_int>;
pub type V = *const libc::c_void;
pub const PS_FAIL: parse_state = 6;
pub type parse_state = libc::c_uint;
pub const PS_DONE: parse_state = 5;
pub const PS_4: parse_state = 4;
pub const PS_3: parse_state = 3;
pub const PS_2: parse_state = 2;
pub const PS_START: parse_state = 1;
pub const ST_ERROR: C2RustUnnamed_25 = 6;
pub type C2RustUnnamed_25 = libc::c_uint;
pub const ST_END: C2RustUnnamed_25 = 5;
pub const ST_CARET: C2RustUnnamed_25 = 4;
pub const ST_HEX: C2RustUnnamed_25 = 3;
pub const ST_OCTAL: C2RustUnnamed_25 = 2;
pub const ST_BACKSLASH: C2RustUnnamed_25 = 1;
pub const ST_GND: C2RustUnnamed_25 = 0;
pub type C2RustUnnamed_26 = libc::c_uint;
pub const INITIAL_TABLE_SIZE: C2RustUnnamed_26 = 30;
pub type C2RustUnnamed_27 = libc::c_uint;
pub type C2RustUnnamed_28 = libc::c_uint;
pub type C2RustUnnamed_29 = libc::c_uint;
pub type C2RustUnnamed_30 = libc::c_uint;
pub type C2RustUnnamed_31 = libc::c_uint;
pub type C2RustUnnamed_32 = libc::c_uint;
#[inline]
unsafe extern "C" fn putc_unlocked(
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
unsafe extern "C" fn gnu_dev_minor(mut __dev: __dev_t) -> libc::c_uint {
    let mut __minor: libc::c_uint = 0;
    __minor = ((__dev & 0xff as libc::c_uint as __dev_t) >> 0 as libc::c_int)
        as libc::c_uint;
    __minor = (__minor as libc::c_ulong
        | (__dev & 0xffffff00000 as libc::c_ulong) >> 12 as libc::c_int) as libc::c_uint;
    return __minor;
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
unsafe extern "C" fn initialize_exit_failure(mut status: libc::c_int) {
    if status != 1 as libc::c_int {
        ::core::ptr::write_volatile(&mut exit_failure as *mut libc::c_int, status);
    }
}
#[inline]
unsafe extern "C" fn timespec_cmp(mut a: timespec, mut b: timespec) -> libc::c_int {
    return 2 as libc::c_int
        * ((a.tv_sec > b.tv_sec) as libc::c_int - (a.tv_sec < b.tv_sec) as libc::c_int)
        + ((a.tv_nsec > b.tv_nsec) as libc::c_int
            - (a.tv_nsec < b.tv_nsec) as libc::c_int);
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
unsafe extern "C" fn dot_or_dotdot(mut file_name: *const libc::c_char) -> bool {
    if *file_name.offset(0 as libc::c_int as isize) as libc::c_int == '.' as i32 {
        let mut sep: libc::c_char = *file_name
            .offset(
                ((*file_name.offset(1 as libc::c_int as isize) as libc::c_int
                    == '.' as i32) as libc::c_int + 1 as libc::c_int) as isize,
            );
        return sep == 0 || sep as libc::c_int == '/' as i32;
    } else {
        return 0 as libc::c_int != 0
    };
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
unsafe extern "C" fn is_ENOTSUP(mut err: libc::c_int) -> bool {
    return err == 95 as libc::c_int
        || 95 as libc::c_int != 95 as libc::c_int && err == 95 as libc::c_int;
}
#[inline]
unsafe extern "C" fn is_smack_enabled() -> bool {
    return 0 as libc::c_int != 0;
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
static mut G_line: [libc::c_char; 4808] = [
    '#' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    'C' as i32 as libc::c_char,
    'o' as i32 as libc::c_char,
    'n' as i32 as libc::c_char,
    'f' as i32 as libc::c_char,
    'i' as i32 as libc::c_char,
    'g' as i32 as libc::c_char,
    'u' as i32 as libc::c_char,
    'r' as i32 as libc::c_char,
    'a' as i32 as libc::c_char,
    't' as i32 as libc::c_char,
    'i' as i32 as libc::c_char,
    'o' as i32 as libc::c_char,
    'n' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    'f' as i32 as libc::c_char,
    'i' as i32 as libc::c_char,
    'l' as i32 as libc::c_char,
    'e' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    'f' as i32 as libc::c_char,
    'o' as i32 as libc::c_char,
    'r' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    'd' as i32 as libc::c_char,
    'i' as i32 as libc::c_char,
    'r' as i32 as libc::c_char,
    'c' as i32 as libc::c_char,
    'o' as i32 as libc::c_char,
    'l' as i32 as libc::c_char,
    'o' as i32 as libc::c_char,
    'r' as i32 as libc::c_char,
    's' as i32 as libc::c_char,
    ',' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    'a' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    'u' as i32 as libc::c_char,
    't' as i32 as libc::c_char,
    'i' as i32 as libc::c_char,
    'l' as i32 as libc::c_char,
    'i' as i32 as libc::c_char,
    't' as i32 as libc::c_char,
    'y' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    't' as i32 as libc::c_char,
    'o' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    'h' as i32 as libc::c_char,
    'e' as i32 as libc::c_char,
    'l' as i32 as libc::c_char,
    'p' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    'y' as i32 as libc::c_char,
    'o' as i32 as libc::c_char,
    'u' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    's' as i32 as libc::c_char,
    'e' as i32 as libc::c_char,
    't' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    't' as i32 as libc::c_char,
    'h' as i32 as libc::c_char,
    'e' as i32 as libc::c_char,
    0 as libc::c_int as libc::c_char,
    '#' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    'L' as i32 as libc::c_char,
    'S' as i32 as libc::c_char,
    '_' as i32 as libc::c_char,
    'C' as i32 as libc::c_char,
    'O' as i32 as libc::c_char,
    'L' as i32 as libc::c_char,
    'O' as i32 as libc::c_char,
    'R' as i32 as libc::c_char,
    'S' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    'e' as i32 as libc::c_char,
    'n' as i32 as libc::c_char,
    'v' as i32 as libc::c_char,
    'i' as i32 as libc::c_char,
    'r' as i32 as libc::c_char,
    'o' as i32 as libc::c_char,
    'n' as i32 as libc::c_char,
    'm' as i32 as libc::c_char,
    'e' as i32 as libc::c_char,
    'n' as i32 as libc::c_char,
    't' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    'v' as i32 as libc::c_char,
    'a' as i32 as libc::c_char,
    'r' as i32 as libc::c_char,
    'i' as i32 as libc::c_char,
    'a' as i32 as libc::c_char,
    'b' as i32 as libc::c_char,
    'l' as i32 as libc::c_char,
    'e' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    'u' as i32 as libc::c_char,
    's' as i32 as libc::c_char,
    'e' as i32 as libc::c_char,
    'd' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    'b' as i32 as libc::c_char,
    'y' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    'G' as i32 as libc::c_char,
    'N' as i32 as libc::c_char,
    'U' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    'l' as i32 as libc::c_char,
    's' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    'w' as i32 as libc::c_char,
    'i' as i32 as libc::c_char,
    't' as i32 as libc::c_char,
    'h' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    't' as i32 as libc::c_char,
    'h' as i32 as libc::c_char,
    'e' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    '-' as i32 as libc::c_char,
    '-' as i32 as libc::c_char,
    'c' as i32 as libc::c_char,
    'o' as i32 as libc::c_char,
    'l' as i32 as libc::c_char,
    'o' as i32 as libc::c_char,
    'r' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    'o' as i32 as libc::c_char,
    'p' as i32 as libc::c_char,
    't' as i32 as libc::c_char,
    'i' as i32 as libc::c_char,
    'o' as i32 as libc::c_char,
    'n' as i32 as libc::c_char,
    '.' as i32 as libc::c_char,
    0 as libc::c_int as libc::c_char,
    '#' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    'C' as i32 as libc::c_char,
    'o' as i32 as libc::c_char,
    'p' as i32 as libc::c_char,
    'y' as i32 as libc::c_char,
    'r' as i32 as libc::c_char,
    'i' as i32 as libc::c_char,
    'g' as i32 as libc::c_char,
    'h' as i32 as libc::c_char,
    't' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    '(' as i32 as libc::c_char,
    'C' as i32 as libc::c_char,
    ')' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    '1' as i32 as libc::c_char,
    '9' as i32 as libc::c_char,
    '9' as i32 as libc::c_char,
    '6' as i32 as libc::c_char,
    '-' as i32 as libc::c_char,
    '2' as i32 as libc::c_char,
    '0' as i32 as libc::c_char,
    '2' as i32 as libc::c_char,
    '3' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    'F' as i32 as libc::c_char,
    'r' as i32 as libc::c_char,
    'e' as i32 as libc::c_char,
    'e' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    'S' as i32 as libc::c_char,
    'o' as i32 as libc::c_char,
    'f' as i32 as libc::c_char,
    't' as i32 as libc::c_char,
    'w' as i32 as libc::c_char,
    'a' as i32 as libc::c_char,
    'r' as i32 as libc::c_char,
    'e' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    'F' as i32 as libc::c_char,
    'o' as i32 as libc::c_char,
    'u' as i32 as libc::c_char,
    'n' as i32 as libc::c_char,
    'd' as i32 as libc::c_char,
    'a' as i32 as libc::c_char,
    't' as i32 as libc::c_char,
    'i' as i32 as libc::c_char,
    'o' as i32 as libc::c_char,
    'n' as i32 as libc::c_char,
    ',' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    'I' as i32 as libc::c_char,
    'n' as i32 as libc::c_char,
    'c' as i32 as libc::c_char,
    '.' as i32 as libc::c_char,
    0 as libc::c_int as libc::c_char,
    '#' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    'C' as i32 as libc::c_char,
    'o' as i32 as libc::c_char,
    'p' as i32 as libc::c_char,
    'y' as i32 as libc::c_char,
    'i' as i32 as libc::c_char,
    'n' as i32 as libc::c_char,
    'g' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    'a' as i32 as libc::c_char,
    'n' as i32 as libc::c_char,
    'd' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    'd' as i32 as libc::c_char,
    'i' as i32 as libc::c_char,
    's' as i32 as libc::c_char,
    't' as i32 as libc::c_char,
    'r' as i32 as libc::c_char,
    'i' as i32 as libc::c_char,
    'b' as i32 as libc::c_char,
    'u' as i32 as libc::c_char,
    't' as i32 as libc::c_char,
    'i' as i32 as libc::c_char,
    'o' as i32 as libc::c_char,
    'n' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    'o' as i32 as libc::c_char,
    'f' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    't' as i32 as libc::c_char,
    'h' as i32 as libc::c_char,
    'i' as i32 as libc::c_char,
    's' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    'f' as i32 as libc::c_char,
    'i' as i32 as libc::c_char,
    'l' as i32 as libc::c_char,
    'e' as i32 as libc::c_char,
    ',' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    'w' as i32 as libc::c_char,
    'i' as i32 as libc::c_char,
    't' as i32 as libc::c_char,
    'h' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    'o' as i32 as libc::c_char,
    'r' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    'w' as i32 as libc::c_char,
    'i' as i32 as libc::c_char,
    't' as i32 as libc::c_char,
    'h' as i32 as libc::c_char,
    'o' as i32 as libc::c_char,
    'u' as i32 as libc::c_char,
    't' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    'm' as i32 as libc::c_char,
    'o' as i32 as libc::c_char,
    'd' as i32 as libc::c_char,
    'i' as i32 as libc::c_char,
    'f' as i32 as libc::c_char,
    'i' as i32 as libc::c_char,
    'c' as i32 as libc::c_char,
    'a' as i32 as libc::c_char,
    't' as i32 as libc::c_char,
    'i' as i32 as libc::c_char,
    'o' as i32 as libc::c_char,
    'n' as i32 as libc::c_char,
    ',' as i32 as libc::c_char,
    0 as libc::c_int as libc::c_char,
    '#' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    'a' as i32 as libc::c_char,
    'r' as i32 as libc::c_char,
    'e' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    'p' as i32 as libc::c_char,
    'e' as i32 as libc::c_char,
    'r' as i32 as libc::c_char,
    'm' as i32 as libc::c_char,
    'i' as i32 as libc::c_char,
    't' as i32 as libc::c_char,
    't' as i32 as libc::c_char,
    'e' as i32 as libc::c_char,
    'd' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    'p' as i32 as libc::c_char,
    'r' as i32 as libc::c_char,
    'o' as i32 as libc::c_char,
    'v' as i32 as libc::c_char,
    'i' as i32 as libc::c_char,
    'd' as i32 as libc::c_char,
    'e' as i32 as libc::c_char,
    'd' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    't' as i32 as libc::c_char,
    'h' as i32 as libc::c_char,
    'e' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    'c' as i32 as libc::c_char,
    'o' as i32 as libc::c_char,
    'p' as i32 as libc::c_char,
    'y' as i32 as libc::c_char,
    'r' as i32 as libc::c_char,
    'i' as i32 as libc::c_char,
    'g' as i32 as libc::c_char,
    'h' as i32 as libc::c_char,
    't' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    'n' as i32 as libc::c_char,
    'o' as i32 as libc::c_char,
    't' as i32 as libc::c_char,
    'i' as i32 as libc::c_char,
    'c' as i32 as libc::c_char,
    'e' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    'a' as i32 as libc::c_char,
    'n' as i32 as libc::c_char,
    'd' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    't' as i32 as libc::c_char,
    'h' as i32 as libc::c_char,
    'i' as i32 as libc::c_char,
    's' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    'n' as i32 as libc::c_char,
    'o' as i32 as libc::c_char,
    't' as i32 as libc::c_char,
    'i' as i32 as libc::c_char,
    'c' as i32 as libc::c_char,
    'e' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    'a' as i32 as libc::c_char,
    'r' as i32 as libc::c_char,
    'e' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    'p' as i32 as libc::c_char,
    'r' as i32 as libc::c_char,
    'e' as i32 as libc::c_char,
    's' as i32 as libc::c_char,
    'e' as i32 as libc::c_char,
    'r' as i32 as libc::c_char,
    'v' as i32 as libc::c_char,
    'e' as i32 as libc::c_char,
    'd' as i32 as libc::c_char,
    '.' as i32 as libc::c_char,
    0 as libc::c_int as libc::c_char,
    '#' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    'T' as i32 as libc::c_char,
    'h' as i32 as libc::c_char,
    'e' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    'k' as i32 as libc::c_char,
    'e' as i32 as libc::c_char,
    'y' as i32 as libc::c_char,
    'w' as i32 as libc::c_char,
    'o' as i32 as libc::c_char,
    'r' as i32 as libc::c_char,
    'd' as i32 as libc::c_char,
    's' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    'C' as i32 as libc::c_char,
    'O' as i32 as libc::c_char,
    'L' as i32 as libc::c_char,
    'O' as i32 as libc::c_char,
    'R' as i32 as libc::c_char,
    ',' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    'O' as i32 as libc::c_char,
    'P' as i32 as libc::c_char,
    'T' as i32 as libc::c_char,
    'I' as i32 as libc::c_char,
    'O' as i32 as libc::c_char,
    'N' as i32 as libc::c_char,
    'S' as i32 as libc::c_char,
    ',' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    'a' as i32 as libc::c_char,
    'n' as i32 as libc::c_char,
    'd' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    'E' as i32 as libc::c_char,
    'I' as i32 as libc::c_char,
    'G' as i32 as libc::c_char,
    'H' as i32 as libc::c_char,
    'T' as i32 as libc::c_char,
    'B' as i32 as libc::c_char,
    'I' as i32 as libc::c_char,
    'T' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    '(' as i32 as libc::c_char,
    'h' as i32 as libc::c_char,
    'o' as i32 as libc::c_char,
    'n' as i32 as libc::c_char,
    'o' as i32 as libc::c_char,
    'r' as i32 as libc::c_char,
    'e' as i32 as libc::c_char,
    'd' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    'b' as i32 as libc::c_char,
    'y' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    't' as i32 as libc::c_char,
    'h' as i32 as libc::c_char,
    'e' as i32 as libc::c_char,
    0 as libc::c_int as libc::c_char,
    '#' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    's' as i32 as libc::c_char,
    'l' as i32 as libc::c_char,
    'a' as i32 as libc::c_char,
    'c' as i32 as libc::c_char,
    'k' as i32 as libc::c_char,
    'w' as i32 as libc::c_char,
    'a' as i32 as libc::c_char,
    'r' as i32 as libc::c_char,
    'e' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    'v' as i32 as libc::c_char,
    'e' as i32 as libc::c_char,
    'r' as i32 as libc::c_char,
    's' as i32 as libc::c_char,
    'i' as i32 as libc::c_char,
    'o' as i32 as libc::c_char,
    'n' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    'o' as i32 as libc::c_char,
    'f' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    'd' as i32 as libc::c_char,
    'i' as i32 as libc::c_char,
    'r' as i32 as libc::c_char,
    'c' as i32 as libc::c_char,
    'o' as i32 as libc::c_char,
    'l' as i32 as libc::c_char,
    'o' as i32 as libc::c_char,
    'r' as i32 as libc::c_char,
    's' as i32 as libc::c_char,
    ')' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    'a' as i32 as libc::c_char,
    'r' as i32 as libc::c_char,
    'e' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    'r' as i32 as libc::c_char,
    'e' as i32 as libc::c_char,
    'c' as i32 as libc::c_char,
    'o' as i32 as libc::c_char,
    'g' as i32 as libc::c_char,
    'n' as i32 as libc::c_char,
    'i' as i32 as libc::c_char,
    'z' as i32 as libc::c_char,
    'e' as i32 as libc::c_char,
    'd' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    'b' as i32 as libc::c_char,
    'u' as i32 as libc::c_char,
    't' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    'i' as i32 as libc::c_char,
    'g' as i32 as libc::c_char,
    'n' as i32 as libc::c_char,
    'o' as i32 as libc::c_char,
    'r' as i32 as libc::c_char,
    'e' as i32 as libc::c_char,
    'd' as i32 as libc::c_char,
    '.' as i32 as libc::c_char,
    0 as libc::c_int as libc::c_char,
    '#' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    'G' as i32 as libc::c_char,
    'l' as i32 as libc::c_char,
    'o' as i32 as libc::c_char,
    'b' as i32 as libc::c_char,
    'a' as i32 as libc::c_char,
    'l' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    'c' as i32 as libc::c_char,
    'o' as i32 as libc::c_char,
    'n' as i32 as libc::c_char,
    'f' as i32 as libc::c_char,
    'i' as i32 as libc::c_char,
    'g' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    'o' as i32 as libc::c_char,
    'p' as i32 as libc::c_char,
    't' as i32 as libc::c_char,
    'i' as i32 as libc::c_char,
    'o' as i32 as libc::c_char,
    'n' as i32 as libc::c_char,
    's' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    'c' as i32 as libc::c_char,
    'a' as i32 as libc::c_char,
    'n' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    'b' as i32 as libc::c_char,
    'e' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    's' as i32 as libc::c_char,
    'p' as i32 as libc::c_char,
    'e' as i32 as libc::c_char,
    'c' as i32 as libc::c_char,
    'i' as i32 as libc::c_char,
    'f' as i32 as libc::c_char,
    'i' as i32 as libc::c_char,
    'e' as i32 as libc::c_char,
    'd' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    'b' as i32 as libc::c_char,
    'e' as i32 as libc::c_char,
    'f' as i32 as libc::c_char,
    'o' as i32 as libc::c_char,
    'r' as i32 as libc::c_char,
    'e' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    'T' as i32 as libc::c_char,
    'E' as i32 as libc::c_char,
    'R' as i32 as libc::c_char,
    'M' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    'o' as i32 as libc::c_char,
    'r' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    'C' as i32 as libc::c_char,
    'O' as i32 as libc::c_char,
    'L' as i32 as libc::c_char,
    'O' as i32 as libc::c_char,
    'R' as i32 as libc::c_char,
    'T' as i32 as libc::c_char,
    'E' as i32 as libc::c_char,
    'R' as i32 as libc::c_char,
    'M' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    'e' as i32 as libc::c_char,
    'n' as i32 as libc::c_char,
    't' as i32 as libc::c_char,
    'r' as i32 as libc::c_char,
    'i' as i32 as libc::c_char,
    'e' as i32 as libc::c_char,
    's' as i32 as libc::c_char,
    0 as libc::c_int as libc::c_char,
    '#' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    'B' as i32 as libc::c_char,
    'e' as i32 as libc::c_char,
    'l' as i32 as libc::c_char,
    'o' as i32 as libc::c_char,
    'w' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    'a' as i32 as libc::c_char,
    'r' as i32 as libc::c_char,
    'e' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    'T' as i32 as libc::c_char,
    'E' as i32 as libc::c_char,
    'R' as i32 as libc::c_char,
    'M' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    'o' as i32 as libc::c_char,
    'r' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    'C' as i32 as libc::c_char,
    'O' as i32 as libc::c_char,
    'L' as i32 as libc::c_char,
    'O' as i32 as libc::c_char,
    'R' as i32 as libc::c_char,
    'T' as i32 as libc::c_char,
    'E' as i32 as libc::c_char,
    'R' as i32 as libc::c_char,
    'M' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    'e' as i32 as libc::c_char,
    'n' as i32 as libc::c_char,
    't' as i32 as libc::c_char,
    'r' as i32 as libc::c_char,
    'i' as i32 as libc::c_char,
    'e' as i32 as libc::c_char,
    's' as i32 as libc::c_char,
    ',' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    'w' as i32 as libc::c_char,
    'h' as i32 as libc::c_char,
    'i' as i32 as libc::c_char,
    'c' as i32 as libc::c_char,
    'h' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    'c' as i32 as libc::c_char,
    'a' as i32 as libc::c_char,
    'n' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    'b' as i32 as libc::c_char,
    'e' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    'g' as i32 as libc::c_char,
    'l' as i32 as libc::c_char,
    'o' as i32 as libc::c_char,
    'b' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    'p' as i32 as libc::c_char,
    'a' as i32 as libc::c_char,
    't' as i32 as libc::c_char,
    't' as i32 as libc::c_char,
    'e' as i32 as libc::c_char,
    'r' as i32 as libc::c_char,
    'n' as i32 as libc::c_char,
    's' as i32 as libc::c_char,
    ',' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    'w' as i32 as libc::c_char,
    'h' as i32 as libc::c_char,
    'i' as i32 as libc::c_char,
    'c' as i32 as libc::c_char,
    'h' as i32 as libc::c_char,
    0 as libc::c_int as libc::c_char,
    '#' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    'r' as i32 as libc::c_char,
    'e' as i32 as libc::c_char,
    's' as i32 as libc::c_char,
    't' as i32 as libc::c_char,
    'r' as i32 as libc::c_char,
    'i' as i32 as libc::c_char,
    'c' as i32 as libc::c_char,
    't' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    'f' as i32 as libc::c_char,
    'o' as i32 as libc::c_char,
    'l' as i32 as libc::c_char,
    'l' as i32 as libc::c_char,
    'o' as i32 as libc::c_char,
    'w' as i32 as libc::c_char,
    'i' as i32 as libc::c_char,
    'n' as i32 as libc::c_char,
    'g' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    'c' as i32 as libc::c_char,
    'o' as i32 as libc::c_char,
    'n' as i32 as libc::c_char,
    'f' as i32 as libc::c_char,
    'i' as i32 as libc::c_char,
    'g' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    't' as i32 as libc::c_char,
    'o' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    's' as i32 as libc::c_char,
    'y' as i32 as libc::c_char,
    's' as i32 as libc::c_char,
    't' as i32 as libc::c_char,
    'e' as i32 as libc::c_char,
    'm' as i32 as libc::c_char,
    's' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    'w' as i32 as libc::c_char,
    'i' as i32 as libc::c_char,
    't' as i32 as libc::c_char,
    'h' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    'm' as i32 as libc::c_char,
    'a' as i32 as libc::c_char,
    't' as i32 as libc::c_char,
    'c' as i32 as libc::c_char,
    'h' as i32 as libc::c_char,
    'i' as i32 as libc::c_char,
    'n' as i32 as libc::c_char,
    'g' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    'e' as i32 as libc::c_char,
    'n' as i32 as libc::c_char,
    'v' as i32 as libc::c_char,
    'i' as i32 as libc::c_char,
    'r' as i32 as libc::c_char,
    'o' as i32 as libc::c_char,
    'n' as i32 as libc::c_char,
    'm' as i32 as libc::c_char,
    'e' as i32 as libc::c_char,
    'n' as i32 as libc::c_char,
    't' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    'v' as i32 as libc::c_char,
    'a' as i32 as libc::c_char,
    'r' as i32 as libc::c_char,
    'i' as i32 as libc::c_char,
    'a' as i32 as libc::c_char,
    'b' as i32 as libc::c_char,
    'l' as i32 as libc::c_char,
    'e' as i32 as libc::c_char,
    's' as i32 as libc::c_char,
    '.' as i32 as libc::c_char,
    0 as libc::c_int as libc::c_char,
    'C' as i32 as libc::c_char,
    'O' as i32 as libc::c_char,
    'L' as i32 as libc::c_char,
    'O' as i32 as libc::c_char,
    'R' as i32 as libc::c_char,
    'T' as i32 as libc::c_char,
    'E' as i32 as libc::c_char,
    'R' as i32 as libc::c_char,
    'M' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    '?' as i32 as libc::c_char,
    '*' as i32 as libc::c_char,
    0 as libc::c_int as libc::c_char,
    'T' as i32 as libc::c_char,
    'E' as i32 as libc::c_char,
    'R' as i32 as libc::c_char,
    'M' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    'E' as i32 as libc::c_char,
    't' as i32 as libc::c_char,
    'e' as i32 as libc::c_char,
    'r' as i32 as libc::c_char,
    'm' as i32 as libc::c_char,
    0 as libc::c_int as libc::c_char,
    'T' as i32 as libc::c_char,
    'E' as i32 as libc::c_char,
    'R' as i32 as libc::c_char,
    'M' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    'a' as i32 as libc::c_char,
    'n' as i32 as libc::c_char,
    's' as i32 as libc::c_char,
    'i' as i32 as libc::c_char,
    0 as libc::c_int as libc::c_char,
    'T' as i32 as libc::c_char,
    'E' as i32 as libc::c_char,
    'R' as i32 as libc::c_char,
    'M' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    '*' as i32 as libc::c_char,
    'c' as i32 as libc::c_char,
    'o' as i32 as libc::c_char,
    'l' as i32 as libc::c_char,
    'o' as i32 as libc::c_char,
    'r' as i32 as libc::c_char,
    '*' as i32 as libc::c_char,
    0 as libc::c_int as libc::c_char,
    'T' as i32 as libc::c_char,
    'E' as i32 as libc::c_char,
    'R' as i32 as libc::c_char,
    'M' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    'c' as i32 as libc::c_char,
    'o' as i32 as libc::c_char,
    'n' as i32 as libc::c_char,
    '[' as i32 as libc::c_char,
    '0' as i32 as libc::c_char,
    '-' as i32 as libc::c_char,
    '9' as i32 as libc::c_char,
    ']' as i32 as libc::c_char,
    '*' as i32 as libc::c_char,
    'x' as i32 as libc::c_char,
    '[' as i32 as libc::c_char,
    '0' as i32 as libc::c_char,
    '-' as i32 as libc::c_char,
    '9' as i32 as libc::c_char,
    ']' as i32 as libc::c_char,
    '*' as i32 as libc::c_char,
    0 as libc::c_int as libc::c_char,
    'T' as i32 as libc::c_char,
    'E' as i32 as libc::c_char,
    'R' as i32 as libc::c_char,
    'M' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    'c' as i32 as libc::c_char,
    'o' as i32 as libc::c_char,
    'n' as i32 as libc::c_char,
    's' as i32 as libc::c_char,
    '2' as i32 as libc::c_char,
    '5' as i32 as libc::c_char,
    0 as libc::c_int as libc::c_char,
    'T' as i32 as libc::c_char,
    'E' as i32 as libc::c_char,
    'R' as i32 as libc::c_char,
    'M' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    'c' as i32 as libc::c_char,
    'o' as i32 as libc::c_char,
    'n' as i32 as libc::c_char,
    's' as i32 as libc::c_char,
    'o' as i32 as libc::c_char,
    'l' as i32 as libc::c_char,
    'e' as i32 as libc::c_char,
    0 as libc::c_int as libc::c_char,
    'T' as i32 as libc::c_char,
    'E' as i32 as libc::c_char,
    'R' as i32 as libc::c_char,
    'M' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    'c' as i32 as libc::c_char,
    'y' as i32 as libc::c_char,
    'g' as i32 as libc::c_char,
    'w' as i32 as libc::c_char,
    'i' as i32 as libc::c_char,
    'n' as i32 as libc::c_char,
    0 as libc::c_int as libc::c_char,
    'T' as i32 as libc::c_char,
    'E' as i32 as libc::c_char,
    'R' as i32 as libc::c_char,
    'M' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    '*' as i32 as libc::c_char,
    'd' as i32 as libc::c_char,
    'i' as i32 as libc::c_char,
    'r' as i32 as libc::c_char,
    'e' as i32 as libc::c_char,
    'c' as i32 as libc::c_char,
    't' as i32 as libc::c_char,
    '*' as i32 as libc::c_char,
    0 as libc::c_int as libc::c_char,
    'T' as i32 as libc::c_char,
    'E' as i32 as libc::c_char,
    'R' as i32 as libc::c_char,
    'M' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    'd' as i32 as libc::c_char,
    't' as i32 as libc::c_char,
    't' as i32 as libc::c_char,
    'e' as i32 as libc::c_char,
    'r' as i32 as libc::c_char,
    'm' as i32 as libc::c_char,
    0 as libc::c_int as libc::c_char,
    'T' as i32 as libc::c_char,
    'E' as i32 as libc::c_char,
    'R' as i32 as libc::c_char,
    'M' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    'g' as i32 as libc::c_char,
    'n' as i32 as libc::c_char,
    'o' as i32 as libc::c_char,
    'm' as i32 as libc::c_char,
    'e' as i32 as libc::c_char,
    0 as libc::c_int as libc::c_char,
    'T' as i32 as libc::c_char,
    'E' as i32 as libc::c_char,
    'R' as i32 as libc::c_char,
    'M' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    'h' as i32 as libc::c_char,
    'u' as i32 as libc::c_char,
    'r' as i32 as libc::c_char,
    'd' as i32 as libc::c_char,
    0 as libc::c_int as libc::c_char,
    'T' as i32 as libc::c_char,
    'E' as i32 as libc::c_char,
    'R' as i32 as libc::c_char,
    'M' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    'j' as i32 as libc::c_char,
    'f' as i32 as libc::c_char,
    'b' as i32 as libc::c_char,
    't' as i32 as libc::c_char,
    'e' as i32 as libc::c_char,
    'r' as i32 as libc::c_char,
    'm' as i32 as libc::c_char,
    0 as libc::c_int as libc::c_char,
    'T' as i32 as libc::c_char,
    'E' as i32 as libc::c_char,
    'R' as i32 as libc::c_char,
    'M' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    'k' as i32 as libc::c_char,
    'o' as i32 as libc::c_char,
    'n' as i32 as libc::c_char,
    's' as i32 as libc::c_char,
    'o' as i32 as libc::c_char,
    'l' as i32 as libc::c_char,
    'e' as i32 as libc::c_char,
    0 as libc::c_int as libc::c_char,
    'T' as i32 as libc::c_char,
    'E' as i32 as libc::c_char,
    'R' as i32 as libc::c_char,
    'M' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    'k' as i32 as libc::c_char,
    't' as i32 as libc::c_char,
    'e' as i32 as libc::c_char,
    'r' as i32 as libc::c_char,
    'm' as i32 as libc::c_char,
    0 as libc::c_int as libc::c_char,
    'T' as i32 as libc::c_char,
    'E' as i32 as libc::c_char,
    'R' as i32 as libc::c_char,
    'M' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    'l' as i32 as libc::c_char,
    'i' as i32 as libc::c_char,
    'n' as i32 as libc::c_char,
    'u' as i32 as libc::c_char,
    'x' as i32 as libc::c_char,
    0 as libc::c_int as libc::c_char,
    'T' as i32 as libc::c_char,
    'E' as i32 as libc::c_char,
    'R' as i32 as libc::c_char,
    'M' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    'l' as i32 as libc::c_char,
    'i' as i32 as libc::c_char,
    'n' as i32 as libc::c_char,
    'u' as i32 as libc::c_char,
    'x' as i32 as libc::c_char,
    '-' as i32 as libc::c_char,
    'c' as i32 as libc::c_char,
    0 as libc::c_int as libc::c_char,
    'T' as i32 as libc::c_char,
    'E' as i32 as libc::c_char,
    'R' as i32 as libc::c_char,
    'M' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    'm' as i32 as libc::c_char,
    'l' as i32 as libc::c_char,
    't' as i32 as libc::c_char,
    'e' as i32 as libc::c_char,
    'r' as i32 as libc::c_char,
    'm' as i32 as libc::c_char,
    0 as libc::c_int as libc::c_char,
    'T' as i32 as libc::c_char,
    'E' as i32 as libc::c_char,
    'R' as i32 as libc::c_char,
    'M' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    'p' as i32 as libc::c_char,
    'u' as i32 as libc::c_char,
    't' as i32 as libc::c_char,
    't' as i32 as libc::c_char,
    'y' as i32 as libc::c_char,
    0 as libc::c_int as libc::c_char,
    'T' as i32 as libc::c_char,
    'E' as i32 as libc::c_char,
    'R' as i32 as libc::c_char,
    'M' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    'r' as i32 as libc::c_char,
    'x' as i32 as libc::c_char,
    'v' as i32 as libc::c_char,
    't' as i32 as libc::c_char,
    '*' as i32 as libc::c_char,
    0 as libc::c_int as libc::c_char,
    'T' as i32 as libc::c_char,
    'E' as i32 as libc::c_char,
    'R' as i32 as libc::c_char,
    'M' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    's' as i32 as libc::c_char,
    'c' as i32 as libc::c_char,
    'r' as i32 as libc::c_char,
    'e' as i32 as libc::c_char,
    'e' as i32 as libc::c_char,
    'n' as i32 as libc::c_char,
    '*' as i32 as libc::c_char,
    0 as libc::c_int as libc::c_char,
    'T' as i32 as libc::c_char,
    'E' as i32 as libc::c_char,
    'R' as i32 as libc::c_char,
    'M' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    's' as i32 as libc::c_char,
    't' as i32 as libc::c_char,
    0 as libc::c_int as libc::c_char,
    'T' as i32 as libc::c_char,
    'E' as i32 as libc::c_char,
    'R' as i32 as libc::c_char,
    'M' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    't' as i32 as libc::c_char,
    'e' as i32 as libc::c_char,
    'r' as i32 as libc::c_char,
    'm' as i32 as libc::c_char,
    'i' as i32 as libc::c_char,
    'n' as i32 as libc::c_char,
    'a' as i32 as libc::c_char,
    't' as i32 as libc::c_char,
    'o' as i32 as libc::c_char,
    'r' as i32 as libc::c_char,
    0 as libc::c_int as libc::c_char,
    'T' as i32 as libc::c_char,
    'E' as i32 as libc::c_char,
    'R' as i32 as libc::c_char,
    'M' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    't' as i32 as libc::c_char,
    'm' as i32 as libc::c_char,
    'u' as i32 as libc::c_char,
    'x' as i32 as libc::c_char,
    '*' as i32 as libc::c_char,
    0 as libc::c_int as libc::c_char,
    'T' as i32 as libc::c_char,
    'E' as i32 as libc::c_char,
    'R' as i32 as libc::c_char,
    'M' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    'v' as i32 as libc::c_char,
    't' as i32 as libc::c_char,
    '1' as i32 as libc::c_char,
    '0' as i32 as libc::c_char,
    '0' as i32 as libc::c_char,
    0 as libc::c_int as libc::c_char,
    'T' as i32 as libc::c_char,
    'E' as i32 as libc::c_char,
    'R' as i32 as libc::c_char,
    'M' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    'x' as i32 as libc::c_char,
    't' as i32 as libc::c_char,
    'e' as i32 as libc::c_char,
    'r' as i32 as libc::c_char,
    'm' as i32 as libc::c_char,
    '*' as i32 as libc::c_char,
    0 as libc::c_int as libc::c_char,
    '#' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    'B' as i32 as libc::c_char,
    'e' as i32 as libc::c_char,
    'l' as i32 as libc::c_char,
    'o' as i32 as libc::c_char,
    'w' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    'a' as i32 as libc::c_char,
    'r' as i32 as libc::c_char,
    'e' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    't' as i32 as libc::c_char,
    'h' as i32 as libc::c_char,
    'e' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    'c' as i32 as libc::c_char,
    'o' as i32 as libc::c_char,
    'l' as i32 as libc::c_char,
    'o' as i32 as libc::c_char,
    'r' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    'i' as i32 as libc::c_char,
    'n' as i32 as libc::c_char,
    'i' as i32 as libc::c_char,
    't' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    's' as i32 as libc::c_char,
    't' as i32 as libc::c_char,
    'r' as i32 as libc::c_char,
    'i' as i32 as libc::c_char,
    'n' as i32 as libc::c_char,
    'g' as i32 as libc::c_char,
    's' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    'f' as i32 as libc::c_char,
    'o' as i32 as libc::c_char,
    'r' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    't' as i32 as libc::c_char,
    'h' as i32 as libc::c_char,
    'e' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    'b' as i32 as libc::c_char,
    'a' as i32 as libc::c_char,
    's' as i32 as libc::c_char,
    'i' as i32 as libc::c_char,
    'c' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    'f' as i32 as libc::c_char,
    'i' as i32 as libc::c_char,
    'l' as i32 as libc::c_char,
    'e' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    't' as i32 as libc::c_char,
    'y' as i32 as libc::c_char,
    'p' as i32 as libc::c_char,
    'e' as i32 as libc::c_char,
    's' as i32 as libc::c_char,
    '.' as i32 as libc::c_char,
    0 as libc::c_int as libc::c_char,
    '#' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    'O' as i32 as libc::c_char,
    'n' as i32 as libc::c_char,
    'e' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    'c' as i32 as libc::c_char,
    'a' as i32 as libc::c_char,
    'n' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    'u' as i32 as libc::c_char,
    's' as i32 as libc::c_char,
    'e' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    'c' as i32 as libc::c_char,
    'o' as i32 as libc::c_char,
    'd' as i32 as libc::c_char,
    'e' as i32 as libc::c_char,
    's' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    'f' as i32 as libc::c_char,
    'o' as i32 as libc::c_char,
    'r' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    '2' as i32 as libc::c_char,
    '5' as i32 as libc::c_char,
    '6' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    'o' as i32 as libc::c_char,
    'r' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    'm' as i32 as libc::c_char,
    'o' as i32 as libc::c_char,
    'r' as i32 as libc::c_char,
    'e' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    'c' as i32 as libc::c_char,
    'o' as i32 as libc::c_char,
    'l' as i32 as libc::c_char,
    'o' as i32 as libc::c_char,
    'r' as i32 as libc::c_char,
    's' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    's' as i32 as libc::c_char,
    'u' as i32 as libc::c_char,
    'p' as i32 as libc::c_char,
    'p' as i32 as libc::c_char,
    'o' as i32 as libc::c_char,
    'r' as i32 as libc::c_char,
    't' as i32 as libc::c_char,
    'e' as i32 as libc::c_char,
    'd' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    'b' as i32 as libc::c_char,
    'y' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    'm' as i32 as libc::c_char,
    'o' as i32 as libc::c_char,
    'd' as i32 as libc::c_char,
    'e' as i32 as libc::c_char,
    'r' as i32 as libc::c_char,
    'n' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    't' as i32 as libc::c_char,
    'e' as i32 as libc::c_char,
    'r' as i32 as libc::c_char,
    'm' as i32 as libc::c_char,
    'i' as i32 as libc::c_char,
    'n' as i32 as libc::c_char,
    'a' as i32 as libc::c_char,
    'l' as i32 as libc::c_char,
    's' as i32 as libc::c_char,
    '.' as i32 as libc::c_char,
    0 as libc::c_int as libc::c_char,
    '#' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    'T' as i32 as libc::c_char,
    'h' as i32 as libc::c_char,
    'e' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    'd' as i32 as libc::c_char,
    'e' as i32 as libc::c_char,
    'f' as i32 as libc::c_char,
    'a' as i32 as libc::c_char,
    'u' as i32 as libc::c_char,
    'l' as i32 as libc::c_char,
    't' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    'c' as i32 as libc::c_char,
    'o' as i32 as libc::c_char,
    'l' as i32 as libc::c_char,
    'o' as i32 as libc::c_char,
    'r' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    'c' as i32 as libc::c_char,
    'o' as i32 as libc::c_char,
    'd' as i32 as libc::c_char,
    'e' as i32 as libc::c_char,
    's' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    'u' as i32 as libc::c_char,
    's' as i32 as libc::c_char,
    'e' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    't' as i32 as libc::c_char,
    'h' as i32 as libc::c_char,
    'e' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    'c' as i32 as libc::c_char,
    'a' as i32 as libc::c_char,
    'p' as i32 as libc::c_char,
    'a' as i32 as libc::c_char,
    'b' as i32 as libc::c_char,
    'i' as i32 as libc::c_char,
    'l' as i32 as libc::c_char,
    'i' as i32 as libc::c_char,
    't' as i32 as libc::c_char,
    'i' as i32 as libc::c_char,
    'e' as i32 as libc::c_char,
    's' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    'o' as i32 as libc::c_char,
    'f' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    'a' as i32 as libc::c_char,
    'n' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    '8' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    'c' as i32 as libc::c_char,
    'o' as i32 as libc::c_char,
    'l' as i32 as libc::c_char,
    'o' as i32 as libc::c_char,
    'r' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    't' as i32 as libc::c_char,
    'e' as i32 as libc::c_char,
    'r' as i32 as libc::c_char,
    'm' as i32 as libc::c_char,
    'i' as i32 as libc::c_char,
    'n' as i32 as libc::c_char,
    'a' as i32 as libc::c_char,
    'l' as i32 as libc::c_char,
    0 as libc::c_int as libc::c_char,
    '#' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    'w' as i32 as libc::c_char,
    'i' as i32 as libc::c_char,
    't' as i32 as libc::c_char,
    'h' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    's' as i32 as libc::c_char,
    'o' as i32 as libc::c_char,
    'm' as i32 as libc::c_char,
    'e' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    'a' as i32 as libc::c_char,
    'd' as i32 as libc::c_char,
    'd' as i32 as libc::c_char,
    'i' as i32 as libc::c_char,
    't' as i32 as libc::c_char,
    'i' as i32 as libc::c_char,
    'o' as i32 as libc::c_char,
    'n' as i32 as libc::c_char,
    'a' as i32 as libc::c_char,
    'l' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    'a' as i32 as libc::c_char,
    't' as i32 as libc::c_char,
    't' as i32 as libc::c_char,
    'r' as i32 as libc::c_char,
    'i' as i32 as libc::c_char,
    'b' as i32 as libc::c_char,
    'u' as i32 as libc::c_char,
    't' as i32 as libc::c_char,
    'e' as i32 as libc::c_char,
    's' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    'a' as i32 as libc::c_char,
    's' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    'p' as i32 as libc::c_char,
    'e' as i32 as libc::c_char,
    'r' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    't' as i32 as libc::c_char,
    'h' as i32 as libc::c_char,
    'e' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    'f' as i32 as libc::c_char,
    'o' as i32 as libc::c_char,
    'l' as i32 as libc::c_char,
    'l' as i32 as libc::c_char,
    'o' as i32 as libc::c_char,
    'w' as i32 as libc::c_char,
    'i' as i32 as libc::c_char,
    'n' as i32 as libc::c_char,
    'g' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    'c' as i32 as libc::c_char,
    'o' as i32 as libc::c_char,
    'd' as i32 as libc::c_char,
    'e' as i32 as libc::c_char,
    's' as i32 as libc::c_char,
    ':' as i32 as libc::c_char,
    0 as libc::c_int as libc::c_char,
    '#' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    'A' as i32 as libc::c_char,
    't' as i32 as libc::c_char,
    't' as i32 as libc::c_char,
    'r' as i32 as libc::c_char,
    'i' as i32 as libc::c_char,
    'b' as i32 as libc::c_char,
    'u' as i32 as libc::c_char,
    't' as i32 as libc::c_char,
    'e' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    'c' as i32 as libc::c_char,
    'o' as i32 as libc::c_char,
    'd' as i32 as libc::c_char,
    'e' as i32 as libc::c_char,
    's' as i32 as libc::c_char,
    ':' as i32 as libc::c_char,
    0 as libc::c_int as libc::c_char,
    '#' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    '0' as i32 as libc::c_char,
    '0' as i32 as libc::c_char,
    '=' as i32 as libc::c_char,
    'n' as i32 as libc::c_char,
    'o' as i32 as libc::c_char,
    'n' as i32 as libc::c_char,
    'e' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    '0' as i32 as libc::c_char,
    '1' as i32 as libc::c_char,
    '=' as i32 as libc::c_char,
    'b' as i32 as libc::c_char,
    'o' as i32 as libc::c_char,
    'l' as i32 as libc::c_char,
    'd' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    '0' as i32 as libc::c_char,
    '4' as i32 as libc::c_char,
    '=' as i32 as libc::c_char,
    'u' as i32 as libc::c_char,
    'n' as i32 as libc::c_char,
    'd' as i32 as libc::c_char,
    'e' as i32 as libc::c_char,
    'r' as i32 as libc::c_char,
    's' as i32 as libc::c_char,
    'c' as i32 as libc::c_char,
    'o' as i32 as libc::c_char,
    'r' as i32 as libc::c_char,
    'e' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    '0' as i32 as libc::c_char,
    '5' as i32 as libc::c_char,
    '=' as i32 as libc::c_char,
    'b' as i32 as libc::c_char,
    'l' as i32 as libc::c_char,
    'i' as i32 as libc::c_char,
    'n' as i32 as libc::c_char,
    'k' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    '0' as i32 as libc::c_char,
    '7' as i32 as libc::c_char,
    '=' as i32 as libc::c_char,
    'r' as i32 as libc::c_char,
    'e' as i32 as libc::c_char,
    'v' as i32 as libc::c_char,
    'e' as i32 as libc::c_char,
    'r' as i32 as libc::c_char,
    's' as i32 as libc::c_char,
    'e' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    '0' as i32 as libc::c_char,
    '8' as i32 as libc::c_char,
    '=' as i32 as libc::c_char,
    'c' as i32 as libc::c_char,
    'o' as i32 as libc::c_char,
    'n' as i32 as libc::c_char,
    'c' as i32 as libc::c_char,
    'e' as i32 as libc::c_char,
    'a' as i32 as libc::c_char,
    'l' as i32 as libc::c_char,
    'e' as i32 as libc::c_char,
    'd' as i32 as libc::c_char,
    0 as libc::c_int as libc::c_char,
    '#' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    'T' as i32 as libc::c_char,
    'e' as i32 as libc::c_char,
    'x' as i32 as libc::c_char,
    't' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    'c' as i32 as libc::c_char,
    'o' as i32 as libc::c_char,
    'l' as i32 as libc::c_char,
    'o' as i32 as libc::c_char,
    'r' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    'c' as i32 as libc::c_char,
    'o' as i32 as libc::c_char,
    'd' as i32 as libc::c_char,
    'e' as i32 as libc::c_char,
    's' as i32 as libc::c_char,
    ':' as i32 as libc::c_char,
    0 as libc::c_int as libc::c_char,
    '#' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    '3' as i32 as libc::c_char,
    '0' as i32 as libc::c_char,
    '=' as i32 as libc::c_char,
    'b' as i32 as libc::c_char,
    'l' as i32 as libc::c_char,
    'a' as i32 as libc::c_char,
    'c' as i32 as libc::c_char,
    'k' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    '3' as i32 as libc::c_char,
    '1' as i32 as libc::c_char,
    '=' as i32 as libc::c_char,
    'r' as i32 as libc::c_char,
    'e' as i32 as libc::c_char,
    'd' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    '3' as i32 as libc::c_char,
    '2' as i32 as libc::c_char,
    '=' as i32 as libc::c_char,
    'g' as i32 as libc::c_char,
    'r' as i32 as libc::c_char,
    'e' as i32 as libc::c_char,
    'e' as i32 as libc::c_char,
    'n' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    '3' as i32 as libc::c_char,
    '3' as i32 as libc::c_char,
    '=' as i32 as libc::c_char,
    'y' as i32 as libc::c_char,
    'e' as i32 as libc::c_char,
    'l' as i32 as libc::c_char,
    'l' as i32 as libc::c_char,
    'o' as i32 as libc::c_char,
    'w' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    '3' as i32 as libc::c_char,
    '4' as i32 as libc::c_char,
    '=' as i32 as libc::c_char,
    'b' as i32 as libc::c_char,
    'l' as i32 as libc::c_char,
    'u' as i32 as libc::c_char,
    'e' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    '3' as i32 as libc::c_char,
    '5' as i32 as libc::c_char,
    '=' as i32 as libc::c_char,
    'm' as i32 as libc::c_char,
    'a' as i32 as libc::c_char,
    'g' as i32 as libc::c_char,
    'e' as i32 as libc::c_char,
    'n' as i32 as libc::c_char,
    't' as i32 as libc::c_char,
    'a' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    '3' as i32 as libc::c_char,
    '6' as i32 as libc::c_char,
    '=' as i32 as libc::c_char,
    'c' as i32 as libc::c_char,
    'y' as i32 as libc::c_char,
    'a' as i32 as libc::c_char,
    'n' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    '3' as i32 as libc::c_char,
    '7' as i32 as libc::c_char,
    '=' as i32 as libc::c_char,
    'w' as i32 as libc::c_char,
    'h' as i32 as libc::c_char,
    'i' as i32 as libc::c_char,
    't' as i32 as libc::c_char,
    'e' as i32 as libc::c_char,
    0 as libc::c_int as libc::c_char,
    '#' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    'B' as i32 as libc::c_char,
    'a' as i32 as libc::c_char,
    'c' as i32 as libc::c_char,
    'k' as i32 as libc::c_char,
    'g' as i32 as libc::c_char,
    'r' as i32 as libc::c_char,
    'o' as i32 as libc::c_char,
    'u' as i32 as libc::c_char,
    'n' as i32 as libc::c_char,
    'd' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    'c' as i32 as libc::c_char,
    'o' as i32 as libc::c_char,
    'l' as i32 as libc::c_char,
    'o' as i32 as libc::c_char,
    'r' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    'c' as i32 as libc::c_char,
    'o' as i32 as libc::c_char,
    'd' as i32 as libc::c_char,
    'e' as i32 as libc::c_char,
    's' as i32 as libc::c_char,
    ':' as i32 as libc::c_char,
    0 as libc::c_int as libc::c_char,
    '#' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    '4' as i32 as libc::c_char,
    '0' as i32 as libc::c_char,
    '=' as i32 as libc::c_char,
    'b' as i32 as libc::c_char,
    'l' as i32 as libc::c_char,
    'a' as i32 as libc::c_char,
    'c' as i32 as libc::c_char,
    'k' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    '4' as i32 as libc::c_char,
    '1' as i32 as libc::c_char,
    '=' as i32 as libc::c_char,
    'r' as i32 as libc::c_char,
    'e' as i32 as libc::c_char,
    'd' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    '4' as i32 as libc::c_char,
    '2' as i32 as libc::c_char,
    '=' as i32 as libc::c_char,
    'g' as i32 as libc::c_char,
    'r' as i32 as libc::c_char,
    'e' as i32 as libc::c_char,
    'e' as i32 as libc::c_char,
    'n' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    '4' as i32 as libc::c_char,
    '3' as i32 as libc::c_char,
    '=' as i32 as libc::c_char,
    'y' as i32 as libc::c_char,
    'e' as i32 as libc::c_char,
    'l' as i32 as libc::c_char,
    'l' as i32 as libc::c_char,
    'o' as i32 as libc::c_char,
    'w' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    '4' as i32 as libc::c_char,
    '4' as i32 as libc::c_char,
    '=' as i32 as libc::c_char,
    'b' as i32 as libc::c_char,
    'l' as i32 as libc::c_char,
    'u' as i32 as libc::c_char,
    'e' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    '4' as i32 as libc::c_char,
    '5' as i32 as libc::c_char,
    '=' as i32 as libc::c_char,
    'm' as i32 as libc::c_char,
    'a' as i32 as libc::c_char,
    'g' as i32 as libc::c_char,
    'e' as i32 as libc::c_char,
    'n' as i32 as libc::c_char,
    't' as i32 as libc::c_char,
    'a' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    '4' as i32 as libc::c_char,
    '6' as i32 as libc::c_char,
    '=' as i32 as libc::c_char,
    'c' as i32 as libc::c_char,
    'y' as i32 as libc::c_char,
    'a' as i32 as libc::c_char,
    'n' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    '4' as i32 as libc::c_char,
    '7' as i32 as libc::c_char,
    '=' as i32 as libc::c_char,
    'w' as i32 as libc::c_char,
    'h' as i32 as libc::c_char,
    'i' as i32 as libc::c_char,
    't' as i32 as libc::c_char,
    'e' as i32 as libc::c_char,
    0 as libc::c_int as libc::c_char,
    '#' as i32 as libc::c_char,
    'N' as i32 as libc::c_char,
    'O' as i32 as libc::c_char,
    'R' as i32 as libc::c_char,
    'M' as i32 as libc::c_char,
    'A' as i32 as libc::c_char,
    'L' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    '0' as i32 as libc::c_char,
    '0' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    '#' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    'n' as i32 as libc::c_char,
    'o' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    'c' as i32 as libc::c_char,
    'o' as i32 as libc::c_char,
    'l' as i32 as libc::c_char,
    'o' as i32 as libc::c_char,
    'r' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    'c' as i32 as libc::c_char,
    'o' as i32 as libc::c_char,
    'd' as i32 as libc::c_char,
    'e' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    'a' as i32 as libc::c_char,
    't' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    'a' as i32 as libc::c_char,
    'l' as i32 as libc::c_char,
    'l' as i32 as libc::c_char,
    0 as libc::c_int as libc::c_char,
    '#' as i32 as libc::c_char,
    'F' as i32 as libc::c_char,
    'I' as i32 as libc::c_char,
    'L' as i32 as libc::c_char,
    'E' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    '0' as i32 as libc::c_char,
    '0' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    '#' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    'r' as i32 as libc::c_char,
    'e' as i32 as libc::c_char,
    'g' as i32 as libc::c_char,
    'u' as i32 as libc::c_char,
    'l' as i32 as libc::c_char,
    'a' as i32 as libc::c_char,
    'r' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    'f' as i32 as libc::c_char,
    'i' as i32 as libc::c_char,
    'l' as i32 as libc::c_char,
    'e' as i32 as libc::c_char,
    ':' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    'u' as i32 as libc::c_char,
    's' as i32 as libc::c_char,
    'e' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    'n' as i32 as libc::c_char,
    'o' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    'c' as i32 as libc::c_char,
    'o' as i32 as libc::c_char,
    'l' as i32 as libc::c_char,
    'o' as i32 as libc::c_char,
    'r' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    'a' as i32 as libc::c_char,
    't' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    'a' as i32 as libc::c_char,
    'l' as i32 as libc::c_char,
    'l' as i32 as libc::c_char,
    0 as libc::c_int as libc::c_char,
    'R' as i32 as libc::c_char,
    'E' as i32 as libc::c_char,
    'S' as i32 as libc::c_char,
    'E' as i32 as libc::c_char,
    'T' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    '0' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    '#' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    'r' as i32 as libc::c_char,
    'e' as i32 as libc::c_char,
    's' as i32 as libc::c_char,
    'e' as i32 as libc::c_char,
    't' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    't' as i32 as libc::c_char,
    'o' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    '"' as i32 as libc::c_char,
    'n' as i32 as libc::c_char,
    'o' as i32 as libc::c_char,
    'r' as i32 as libc::c_char,
    'm' as i32 as libc::c_char,
    'a' as i32 as libc::c_char,
    'l' as i32 as libc::c_char,
    '"' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    'c' as i32 as libc::c_char,
    'o' as i32 as libc::c_char,
    'l' as i32 as libc::c_char,
    'o' as i32 as libc::c_char,
    'r' as i32 as libc::c_char,
    0 as libc::c_int as libc::c_char,
    'D' as i32 as libc::c_char,
    'I' as i32 as libc::c_char,
    'R' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    '0' as i32 as libc::c_char,
    '1' as i32 as libc::c_char,
    ';' as i32 as libc::c_char,
    '3' as i32 as libc::c_char,
    '4' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    '#' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    'd' as i32 as libc::c_char,
    'i' as i32 as libc::c_char,
    'r' as i32 as libc::c_char,
    'e' as i32 as libc::c_char,
    'c' as i32 as libc::c_char,
    't' as i32 as libc::c_char,
    'o' as i32 as libc::c_char,
    'r' as i32 as libc::c_char,
    'y' as i32 as libc::c_char,
    0 as libc::c_int as libc::c_char,
    'L' as i32 as libc::c_char,
    'I' as i32 as libc::c_char,
    'N' as i32 as libc::c_char,
    'K' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    '0' as i32 as libc::c_char,
    '1' as i32 as libc::c_char,
    ';' as i32 as libc::c_char,
    '3' as i32 as libc::c_char,
    '6' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    '#' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    's' as i32 as libc::c_char,
    'y' as i32 as libc::c_char,
    'm' as i32 as libc::c_char,
    'b' as i32 as libc::c_char,
    'o' as i32 as libc::c_char,
    'l' as i32 as libc::c_char,
    'i' as i32 as libc::c_char,
    'c' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    'l' as i32 as libc::c_char,
    'i' as i32 as libc::c_char,
    'n' as i32 as libc::c_char,
    'k' as i32 as libc::c_char,
    '.' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    '(' as i32 as libc::c_char,
    'I' as i32 as libc::c_char,
    'f' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    'y' as i32 as libc::c_char,
    'o' as i32 as libc::c_char,
    'u' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    's' as i32 as libc::c_char,
    'e' as i32 as libc::c_char,
    't' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    't' as i32 as libc::c_char,
    'h' as i32 as libc::c_char,
    'i' as i32 as libc::c_char,
    's' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    't' as i32 as libc::c_char,
    'o' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    '\'' as i32 as libc::c_char,
    't' as i32 as libc::c_char,
    'a' as i32 as libc::c_char,
    'r' as i32 as libc::c_char,
    'g' as i32 as libc::c_char,
    'e' as i32 as libc::c_char,
    't' as i32 as libc::c_char,
    '\'' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    'i' as i32 as libc::c_char,
    'n' as i32 as libc::c_char,
    's' as i32 as libc::c_char,
    't' as i32 as libc::c_char,
    'e' as i32 as libc::c_char,
    'a' as i32 as libc::c_char,
    'd' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    'o' as i32 as libc::c_char,
    'f' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    'a' as i32 as libc::c_char,
    0 as libc::c_int as libc::c_char,
    ' ' as i32 as libc::c_char,
    '#' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    'n' as i32 as libc::c_char,
    'u' as i32 as libc::c_char,
    'm' as i32 as libc::c_char,
    'e' as i32 as libc::c_char,
    'r' as i32 as libc::c_char,
    'i' as i32 as libc::c_char,
    'c' as i32 as libc::c_char,
    'a' as i32 as libc::c_char,
    'l' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    'v' as i32 as libc::c_char,
    'a' as i32 as libc::c_char,
    'l' as i32 as libc::c_char,
    'u' as i32 as libc::c_char,
    'e' as i32 as libc::c_char,
    ',' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    't' as i32 as libc::c_char,
    'h' as i32 as libc::c_char,
    'e' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    'c' as i32 as libc::c_char,
    'o' as i32 as libc::c_char,
    'l' as i32 as libc::c_char,
    'o' as i32 as libc::c_char,
    'r' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    'i' as i32 as libc::c_char,
    's' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    'a' as i32 as libc::c_char,
    's' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    'f' as i32 as libc::c_char,
    'o' as i32 as libc::c_char,
    'r' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    't' as i32 as libc::c_char,
    'h' as i32 as libc::c_char,
    'e' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    'f' as i32 as libc::c_char,
    'i' as i32 as libc::c_char,
    'l' as i32 as libc::c_char,
    'e' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    'p' as i32 as libc::c_char,
    'o' as i32 as libc::c_char,
    'i' as i32 as libc::c_char,
    'n' as i32 as libc::c_char,
    't' as i32 as libc::c_char,
    'e' as i32 as libc::c_char,
    'd' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    't' as i32 as libc::c_char,
    'o' as i32 as libc::c_char,
    '.' as i32 as libc::c_char,
    ')' as i32 as libc::c_char,
    0 as libc::c_int as libc::c_char,
    'M' as i32 as libc::c_char,
    'U' as i32 as libc::c_char,
    'L' as i32 as libc::c_char,
    'T' as i32 as libc::c_char,
    'I' as i32 as libc::c_char,
    'H' as i32 as libc::c_char,
    'A' as i32 as libc::c_char,
    'R' as i32 as libc::c_char,
    'D' as i32 as libc::c_char,
    'L' as i32 as libc::c_char,
    'I' as i32 as libc::c_char,
    'N' as i32 as libc::c_char,
    'K' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    '0' as i32 as libc::c_char,
    '0' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    '#' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    'r' as i32 as libc::c_char,
    'e' as i32 as libc::c_char,
    'g' as i32 as libc::c_char,
    'u' as i32 as libc::c_char,
    'l' as i32 as libc::c_char,
    'a' as i32 as libc::c_char,
    'r' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    'f' as i32 as libc::c_char,
    'i' as i32 as libc::c_char,
    'l' as i32 as libc::c_char,
    'e' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    'w' as i32 as libc::c_char,
    'i' as i32 as libc::c_char,
    't' as i32 as libc::c_char,
    'h' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    'm' as i32 as libc::c_char,
    'o' as i32 as libc::c_char,
    'r' as i32 as libc::c_char,
    'e' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    't' as i32 as libc::c_char,
    'h' as i32 as libc::c_char,
    'a' as i32 as libc::c_char,
    'n' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    'o' as i32 as libc::c_char,
    'n' as i32 as libc::c_char,
    'e' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    'l' as i32 as libc::c_char,
    'i' as i32 as libc::c_char,
    'n' as i32 as libc::c_char,
    'k' as i32 as libc::c_char,
    0 as libc::c_int as libc::c_char,
    'F' as i32 as libc::c_char,
    'I' as i32 as libc::c_char,
    'F' as i32 as libc::c_char,
    'O' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    '4' as i32 as libc::c_char,
    '0' as i32 as libc::c_char,
    ';' as i32 as libc::c_char,
    '3' as i32 as libc::c_char,
    '3' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    '#' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    'p' as i32 as libc::c_char,
    'i' as i32 as libc::c_char,
    'p' as i32 as libc::c_char,
    'e' as i32 as libc::c_char,
    0 as libc::c_int as libc::c_char,
    'S' as i32 as libc::c_char,
    'O' as i32 as libc::c_char,
    'C' as i32 as libc::c_char,
    'K' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    '0' as i32 as libc::c_char,
    '1' as i32 as libc::c_char,
    ';' as i32 as libc::c_char,
    '3' as i32 as libc::c_char,
    '5' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    '#' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    's' as i32 as libc::c_char,
    'o' as i32 as libc::c_char,
    'c' as i32 as libc::c_char,
    'k' as i32 as libc::c_char,
    'e' as i32 as libc::c_char,
    't' as i32 as libc::c_char,
    0 as libc::c_int as libc::c_char,
    'D' as i32 as libc::c_char,
    'O' as i32 as libc::c_char,
    'O' as i32 as libc::c_char,
    'R' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    '0' as i32 as libc::c_char,
    '1' as i32 as libc::c_char,
    ';' as i32 as libc::c_char,
    '3' as i32 as libc::c_char,
    '5' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    '#' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    'd' as i32 as libc::c_char,
    'o' as i32 as libc::c_char,
    'o' as i32 as libc::c_char,
    'r' as i32 as libc::c_char,
    0 as libc::c_int as libc::c_char,
    'B' as i32 as libc::c_char,
    'L' as i32 as libc::c_char,
    'K' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    '4' as i32 as libc::c_char,
    '0' as i32 as libc::c_char,
    ';' as i32 as libc::c_char,
    '3' as i32 as libc::c_char,
    '3' as i32 as libc::c_char,
    ';' as i32 as libc::c_char,
    '0' as i32 as libc::c_char,
    '1' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    '#' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    'b' as i32 as libc::c_char,
    'l' as i32 as libc::c_char,
    'o' as i32 as libc::c_char,
    'c' as i32 as libc::c_char,
    'k' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    'd' as i32 as libc::c_char,
    'e' as i32 as libc::c_char,
    'v' as i32 as libc::c_char,
    'i' as i32 as libc::c_char,
    'c' as i32 as libc::c_char,
    'e' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    'd' as i32 as libc::c_char,
    'r' as i32 as libc::c_char,
    'i' as i32 as libc::c_char,
    'v' as i32 as libc::c_char,
    'e' as i32 as libc::c_char,
    'r' as i32 as libc::c_char,
    0 as libc::c_int as libc::c_char,
    'C' as i32 as libc::c_char,
    'H' as i32 as libc::c_char,
    'R' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    '4' as i32 as libc::c_char,
    '0' as i32 as libc::c_char,
    ';' as i32 as libc::c_char,
    '3' as i32 as libc::c_char,
    '3' as i32 as libc::c_char,
    ';' as i32 as libc::c_char,
    '0' as i32 as libc::c_char,
    '1' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    '#' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    'c' as i32 as libc::c_char,
    'h' as i32 as libc::c_char,
    'a' as i32 as libc::c_char,
    'r' as i32 as libc::c_char,
    'a' as i32 as libc::c_char,
    'c' as i32 as libc::c_char,
    't' as i32 as libc::c_char,
    'e' as i32 as libc::c_char,
    'r' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    'd' as i32 as libc::c_char,
    'e' as i32 as libc::c_char,
    'v' as i32 as libc::c_char,
    'i' as i32 as libc::c_char,
    'c' as i32 as libc::c_char,
    'e' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    'd' as i32 as libc::c_char,
    'r' as i32 as libc::c_char,
    'i' as i32 as libc::c_char,
    'v' as i32 as libc::c_char,
    'e' as i32 as libc::c_char,
    'r' as i32 as libc::c_char,
    0 as libc::c_int as libc::c_char,
    'O' as i32 as libc::c_char,
    'R' as i32 as libc::c_char,
    'P' as i32 as libc::c_char,
    'H' as i32 as libc::c_char,
    'A' as i32 as libc::c_char,
    'N' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    '4' as i32 as libc::c_char,
    '0' as i32 as libc::c_char,
    ';' as i32 as libc::c_char,
    '3' as i32 as libc::c_char,
    '1' as i32 as libc::c_char,
    ';' as i32 as libc::c_char,
    '0' as i32 as libc::c_char,
    '1' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    '#' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    's' as i32 as libc::c_char,
    'y' as i32 as libc::c_char,
    'm' as i32 as libc::c_char,
    'l' as i32 as libc::c_char,
    'i' as i32 as libc::c_char,
    'n' as i32 as libc::c_char,
    'k' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    't' as i32 as libc::c_char,
    'o' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    'n' as i32 as libc::c_char,
    'o' as i32 as libc::c_char,
    'n' as i32 as libc::c_char,
    'e' as i32 as libc::c_char,
    'x' as i32 as libc::c_char,
    'i' as i32 as libc::c_char,
    's' as i32 as libc::c_char,
    't' as i32 as libc::c_char,
    'e' as i32 as libc::c_char,
    'n' as i32 as libc::c_char,
    't' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    'f' as i32 as libc::c_char,
    'i' as i32 as libc::c_char,
    'l' as i32 as libc::c_char,
    'e' as i32 as libc::c_char,
    ',' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    'o' as i32 as libc::c_char,
    'r' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    'n' as i32 as libc::c_char,
    'o' as i32 as libc::c_char,
    'n' as i32 as libc::c_char,
    '-' as i32 as libc::c_char,
    's' as i32 as libc::c_char,
    't' as i32 as libc::c_char,
    'a' as i32 as libc::c_char,
    't' as i32 as libc::c_char,
    '\'' as i32 as libc::c_char,
    'a' as i32 as libc::c_char,
    'b' as i32 as libc::c_char,
    'l' as i32 as libc::c_char,
    'e' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    'f' as i32 as libc::c_char,
    'i' as i32 as libc::c_char,
    'l' as i32 as libc::c_char,
    'e' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    '.' as i32 as libc::c_char,
    '.' as i32 as libc::c_char,
    '.' as i32 as libc::c_char,
    0 as libc::c_int as libc::c_char,
    'M' as i32 as libc::c_char,
    'I' as i32 as libc::c_char,
    'S' as i32 as libc::c_char,
    'S' as i32 as libc::c_char,
    'I' as i32 as libc::c_char,
    'N' as i32 as libc::c_char,
    'G' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    '0' as i32 as libc::c_char,
    '0' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    '#' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    '.' as i32 as libc::c_char,
    '.' as i32 as libc::c_char,
    '.' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    'a' as i32 as libc::c_char,
    'n' as i32 as libc::c_char,
    'd' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    't' as i32 as libc::c_char,
    'h' as i32 as libc::c_char,
    'e' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    'f' as i32 as libc::c_char,
    'i' as i32 as libc::c_char,
    'l' as i32 as libc::c_char,
    'e' as i32 as libc::c_char,
    's' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    't' as i32 as libc::c_char,
    'h' as i32 as libc::c_char,
    'e' as i32 as libc::c_char,
    'y' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    'p' as i32 as libc::c_char,
    'o' as i32 as libc::c_char,
    'i' as i32 as libc::c_char,
    'n' as i32 as libc::c_char,
    't' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    't' as i32 as libc::c_char,
    'o' as i32 as libc::c_char,
    0 as libc::c_int as libc::c_char,
    'S' as i32 as libc::c_char,
    'E' as i32 as libc::c_char,
    'T' as i32 as libc::c_char,
    'U' as i32 as libc::c_char,
    'I' as i32 as libc::c_char,
    'D' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    '3' as i32 as libc::c_char,
    '7' as i32 as libc::c_char,
    ';' as i32 as libc::c_char,
    '4' as i32 as libc::c_char,
    '1' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    '#' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    'f' as i32 as libc::c_char,
    'i' as i32 as libc::c_char,
    'l' as i32 as libc::c_char,
    'e' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    't' as i32 as libc::c_char,
    'h' as i32 as libc::c_char,
    'a' as i32 as libc::c_char,
    't' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    'i' as i32 as libc::c_char,
    's' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    's' as i32 as libc::c_char,
    'e' as i32 as libc::c_char,
    't' as i32 as libc::c_char,
    'u' as i32 as libc::c_char,
    'i' as i32 as libc::c_char,
    'd' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    '(' as i32 as libc::c_char,
    'u' as i32 as libc::c_char,
    '+' as i32 as libc::c_char,
    's' as i32 as libc::c_char,
    ')' as i32 as libc::c_char,
    0 as libc::c_int as libc::c_char,
    'S' as i32 as libc::c_char,
    'E' as i32 as libc::c_char,
    'T' as i32 as libc::c_char,
    'G' as i32 as libc::c_char,
    'I' as i32 as libc::c_char,
    'D' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    '3' as i32 as libc::c_char,
    '0' as i32 as libc::c_char,
    ';' as i32 as libc::c_char,
    '4' as i32 as libc::c_char,
    '3' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    '#' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    'f' as i32 as libc::c_char,
    'i' as i32 as libc::c_char,
    'l' as i32 as libc::c_char,
    'e' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    't' as i32 as libc::c_char,
    'h' as i32 as libc::c_char,
    'a' as i32 as libc::c_char,
    't' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    'i' as i32 as libc::c_char,
    's' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    's' as i32 as libc::c_char,
    'e' as i32 as libc::c_char,
    't' as i32 as libc::c_char,
    'g' as i32 as libc::c_char,
    'i' as i32 as libc::c_char,
    'd' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    '(' as i32 as libc::c_char,
    'g' as i32 as libc::c_char,
    '+' as i32 as libc::c_char,
    's' as i32 as libc::c_char,
    ')' as i32 as libc::c_char,
    0 as libc::c_int as libc::c_char,
    'C' as i32 as libc::c_char,
    'A' as i32 as libc::c_char,
    'P' as i32 as libc::c_char,
    'A' as i32 as libc::c_char,
    'B' as i32 as libc::c_char,
    'I' as i32 as libc::c_char,
    'L' as i32 as libc::c_char,
    'I' as i32 as libc::c_char,
    'T' as i32 as libc::c_char,
    'Y' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    '0' as i32 as libc::c_char,
    '0' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    '#' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    'f' as i32 as libc::c_char,
    'i' as i32 as libc::c_char,
    'l' as i32 as libc::c_char,
    'e' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    'w' as i32 as libc::c_char,
    'i' as i32 as libc::c_char,
    't' as i32 as libc::c_char,
    'h' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    'c' as i32 as libc::c_char,
    'a' as i32 as libc::c_char,
    'p' as i32 as libc::c_char,
    'a' as i32 as libc::c_char,
    'b' as i32 as libc::c_char,
    'i' as i32 as libc::c_char,
    'l' as i32 as libc::c_char,
    'i' as i32 as libc::c_char,
    't' as i32 as libc::c_char,
    'y' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    '(' as i32 as libc::c_char,
    'v' as i32 as libc::c_char,
    'e' as i32 as libc::c_char,
    'r' as i32 as libc::c_char,
    'y' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    'e' as i32 as libc::c_char,
    'x' as i32 as libc::c_char,
    'p' as i32 as libc::c_char,
    'e' as i32 as libc::c_char,
    'n' as i32 as libc::c_char,
    's' as i32 as libc::c_char,
    'i' as i32 as libc::c_char,
    'v' as i32 as libc::c_char,
    'e' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    't' as i32 as libc::c_char,
    'o' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    'l' as i32 as libc::c_char,
    'o' as i32 as libc::c_char,
    'o' as i32 as libc::c_char,
    'k' as i32 as libc::c_char,
    'u' as i32 as libc::c_char,
    'p' as i32 as libc::c_char,
    ')' as i32 as libc::c_char,
    0 as libc::c_int as libc::c_char,
    'S' as i32 as libc::c_char,
    'T' as i32 as libc::c_char,
    'I' as i32 as libc::c_char,
    'C' as i32 as libc::c_char,
    'K' as i32 as libc::c_char,
    'Y' as i32 as libc::c_char,
    '_' as i32 as libc::c_char,
    'O' as i32 as libc::c_char,
    'T' as i32 as libc::c_char,
    'H' as i32 as libc::c_char,
    'E' as i32 as libc::c_char,
    'R' as i32 as libc::c_char,
    '_' as i32 as libc::c_char,
    'W' as i32 as libc::c_char,
    'R' as i32 as libc::c_char,
    'I' as i32 as libc::c_char,
    'T' as i32 as libc::c_char,
    'A' as i32 as libc::c_char,
    'B' as i32 as libc::c_char,
    'L' as i32 as libc::c_char,
    'E' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    '3' as i32 as libc::c_char,
    '0' as i32 as libc::c_char,
    ';' as i32 as libc::c_char,
    '4' as i32 as libc::c_char,
    '2' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    '#' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    'd' as i32 as libc::c_char,
    'i' as i32 as libc::c_char,
    'r' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    't' as i32 as libc::c_char,
    'h' as i32 as libc::c_char,
    'a' as i32 as libc::c_char,
    't' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    'i' as i32 as libc::c_char,
    's' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    's' as i32 as libc::c_char,
    't' as i32 as libc::c_char,
    'i' as i32 as libc::c_char,
    'c' as i32 as libc::c_char,
    'k' as i32 as libc::c_char,
    'y' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    'a' as i32 as libc::c_char,
    'n' as i32 as libc::c_char,
    'd' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    'o' as i32 as libc::c_char,
    't' as i32 as libc::c_char,
    'h' as i32 as libc::c_char,
    'e' as i32 as libc::c_char,
    'r' as i32 as libc::c_char,
    '-' as i32 as libc::c_char,
    'w' as i32 as libc::c_char,
    'r' as i32 as libc::c_char,
    'i' as i32 as libc::c_char,
    't' as i32 as libc::c_char,
    'a' as i32 as libc::c_char,
    'b' as i32 as libc::c_char,
    'l' as i32 as libc::c_char,
    'e' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    '(' as i32 as libc::c_char,
    '+' as i32 as libc::c_char,
    't' as i32 as libc::c_char,
    ',' as i32 as libc::c_char,
    'o' as i32 as libc::c_char,
    '+' as i32 as libc::c_char,
    'w' as i32 as libc::c_char,
    ')' as i32 as libc::c_char,
    0 as libc::c_int as libc::c_char,
    'O' as i32 as libc::c_char,
    'T' as i32 as libc::c_char,
    'H' as i32 as libc::c_char,
    'E' as i32 as libc::c_char,
    'R' as i32 as libc::c_char,
    '_' as i32 as libc::c_char,
    'W' as i32 as libc::c_char,
    'R' as i32 as libc::c_char,
    'I' as i32 as libc::c_char,
    'T' as i32 as libc::c_char,
    'A' as i32 as libc::c_char,
    'B' as i32 as libc::c_char,
    'L' as i32 as libc::c_char,
    'E' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    '3' as i32 as libc::c_char,
    '4' as i32 as libc::c_char,
    ';' as i32 as libc::c_char,
    '4' as i32 as libc::c_char,
    '2' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    '#' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    'd' as i32 as libc::c_char,
    'i' as i32 as libc::c_char,
    'r' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    't' as i32 as libc::c_char,
    'h' as i32 as libc::c_char,
    'a' as i32 as libc::c_char,
    't' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    'i' as i32 as libc::c_char,
    's' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    'o' as i32 as libc::c_char,
    't' as i32 as libc::c_char,
    'h' as i32 as libc::c_char,
    'e' as i32 as libc::c_char,
    'r' as i32 as libc::c_char,
    '-' as i32 as libc::c_char,
    'w' as i32 as libc::c_char,
    'r' as i32 as libc::c_char,
    'i' as i32 as libc::c_char,
    't' as i32 as libc::c_char,
    'a' as i32 as libc::c_char,
    'b' as i32 as libc::c_char,
    'l' as i32 as libc::c_char,
    'e' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    '(' as i32 as libc::c_char,
    'o' as i32 as libc::c_char,
    '+' as i32 as libc::c_char,
    'w' as i32 as libc::c_char,
    ')' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    'a' as i32 as libc::c_char,
    'n' as i32 as libc::c_char,
    'd' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    'n' as i32 as libc::c_char,
    'o' as i32 as libc::c_char,
    't' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    's' as i32 as libc::c_char,
    't' as i32 as libc::c_char,
    'i' as i32 as libc::c_char,
    'c' as i32 as libc::c_char,
    'k' as i32 as libc::c_char,
    'y' as i32 as libc::c_char,
    0 as libc::c_int as libc::c_char,
    'S' as i32 as libc::c_char,
    'T' as i32 as libc::c_char,
    'I' as i32 as libc::c_char,
    'C' as i32 as libc::c_char,
    'K' as i32 as libc::c_char,
    'Y' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    '3' as i32 as libc::c_char,
    '7' as i32 as libc::c_char,
    ';' as i32 as libc::c_char,
    '4' as i32 as libc::c_char,
    '4' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    '#' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    'd' as i32 as libc::c_char,
    'i' as i32 as libc::c_char,
    'r' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    'w' as i32 as libc::c_char,
    'i' as i32 as libc::c_char,
    't' as i32 as libc::c_char,
    'h' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    't' as i32 as libc::c_char,
    'h' as i32 as libc::c_char,
    'e' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    's' as i32 as libc::c_char,
    't' as i32 as libc::c_char,
    'i' as i32 as libc::c_char,
    'c' as i32 as libc::c_char,
    'k' as i32 as libc::c_char,
    'y' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    'b' as i32 as libc::c_char,
    'i' as i32 as libc::c_char,
    't' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    's' as i32 as libc::c_char,
    'e' as i32 as libc::c_char,
    't' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    '(' as i32 as libc::c_char,
    '+' as i32 as libc::c_char,
    't' as i32 as libc::c_char,
    ')' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    'a' as i32 as libc::c_char,
    'n' as i32 as libc::c_char,
    'd' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    'n' as i32 as libc::c_char,
    'o' as i32 as libc::c_char,
    't' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    'o' as i32 as libc::c_char,
    't' as i32 as libc::c_char,
    'h' as i32 as libc::c_char,
    'e' as i32 as libc::c_char,
    'r' as i32 as libc::c_char,
    '-' as i32 as libc::c_char,
    'w' as i32 as libc::c_char,
    'r' as i32 as libc::c_char,
    'i' as i32 as libc::c_char,
    't' as i32 as libc::c_char,
    'a' as i32 as libc::c_char,
    'b' as i32 as libc::c_char,
    'l' as i32 as libc::c_char,
    'e' as i32 as libc::c_char,
    0 as libc::c_int as libc::c_char,
    '#' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    'T' as i32 as libc::c_char,
    'h' as i32 as libc::c_char,
    'i' as i32 as libc::c_char,
    's' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    'i' as i32 as libc::c_char,
    's' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    'f' as i32 as libc::c_char,
    'o' as i32 as libc::c_char,
    'r' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    'f' as i32 as libc::c_char,
    'i' as i32 as libc::c_char,
    'l' as i32 as libc::c_char,
    'e' as i32 as libc::c_char,
    's' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    'w' as i32 as libc::c_char,
    'i' as i32 as libc::c_char,
    't' as i32 as libc::c_char,
    'h' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    'e' as i32 as libc::c_char,
    'x' as i32 as libc::c_char,
    'e' as i32 as libc::c_char,
    'c' as i32 as libc::c_char,
    'u' as i32 as libc::c_char,
    't' as i32 as libc::c_char,
    'e' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    'p' as i32 as libc::c_char,
    'e' as i32 as libc::c_char,
    'r' as i32 as libc::c_char,
    'm' as i32 as libc::c_char,
    'i' as i32 as libc::c_char,
    's' as i32 as libc::c_char,
    's' as i32 as libc::c_char,
    'i' as i32 as libc::c_char,
    'o' as i32 as libc::c_char,
    'n' as i32 as libc::c_char,
    ':' as i32 as libc::c_char,
    0 as libc::c_int as libc::c_char,
    'E' as i32 as libc::c_char,
    'X' as i32 as libc::c_char,
    'E' as i32 as libc::c_char,
    'C' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    '0' as i32 as libc::c_char,
    '1' as i32 as libc::c_char,
    ';' as i32 as libc::c_char,
    '3' as i32 as libc::c_char,
    '2' as i32 as libc::c_char,
    0 as libc::c_int as libc::c_char,
    '#' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    'L' as i32 as libc::c_char,
    'i' as i32 as libc::c_char,
    's' as i32 as libc::c_char,
    't' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    'a' as i32 as libc::c_char,
    'n' as i32 as libc::c_char,
    'y' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    'f' as i32 as libc::c_char,
    'i' as i32 as libc::c_char,
    'l' as i32 as libc::c_char,
    'e' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    'e' as i32 as libc::c_char,
    'x' as i32 as libc::c_char,
    't' as i32 as libc::c_char,
    'e' as i32 as libc::c_char,
    'n' as i32 as libc::c_char,
    's' as i32 as libc::c_char,
    'i' as i32 as libc::c_char,
    'o' as i32 as libc::c_char,
    'n' as i32 as libc::c_char,
    's' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    'l' as i32 as libc::c_char,
    'i' as i32 as libc::c_char,
    'k' as i32 as libc::c_char,
    'e' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    '\'' as i32 as libc::c_char,
    '.' as i32 as libc::c_char,
    'g' as i32 as libc::c_char,
    'z' as i32 as libc::c_char,
    '\'' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    'o' as i32 as libc::c_char,
    'r' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    '\'' as i32 as libc::c_char,
    '.' as i32 as libc::c_char,
    't' as i32 as libc::c_char,
    'a' as i32 as libc::c_char,
    'r' as i32 as libc::c_char,
    '\'' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    't' as i32 as libc::c_char,
    'h' as i32 as libc::c_char,
    'a' as i32 as libc::c_char,
    't' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    'y' as i32 as libc::c_char,
    'o' as i32 as libc::c_char,
    'u' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    'w' as i32 as libc::c_char,
    'o' as i32 as libc::c_char,
    'u' as i32 as libc::c_char,
    'l' as i32 as libc::c_char,
    'd' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    'l' as i32 as libc::c_char,
    'i' as i32 as libc::c_char,
    'k' as i32 as libc::c_char,
    'e' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    'l' as i32 as libc::c_char,
    's' as i32 as libc::c_char,
    0 as libc::c_int as libc::c_char,
    '#' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    't' as i32 as libc::c_char,
    'o' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    'c' as i32 as libc::c_char,
    'o' as i32 as libc::c_char,
    'l' as i32 as libc::c_char,
    'o' as i32 as libc::c_char,
    'r' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    'b' as i32 as libc::c_char,
    'e' as i32 as libc::c_char,
    'l' as i32 as libc::c_char,
    'o' as i32 as libc::c_char,
    'w' as i32 as libc::c_char,
    '.' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    'P' as i32 as libc::c_char,
    'u' as i32 as libc::c_char,
    't' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    't' as i32 as libc::c_char,
    'h' as i32 as libc::c_char,
    'e' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    'e' as i32 as libc::c_char,
    'x' as i32 as libc::c_char,
    't' as i32 as libc::c_char,
    'e' as i32 as libc::c_char,
    'n' as i32 as libc::c_char,
    's' as i32 as libc::c_char,
    'i' as i32 as libc::c_char,
    'o' as i32 as libc::c_char,
    'n' as i32 as libc::c_char,
    ',' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    'a' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    's' as i32 as libc::c_char,
    'p' as i32 as libc::c_char,
    'a' as i32 as libc::c_char,
    'c' as i32 as libc::c_char,
    'e' as i32 as libc::c_char,
    ',' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    'a' as i32 as libc::c_char,
    'n' as i32 as libc::c_char,
    'd' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    't' as i32 as libc::c_char,
    'h' as i32 as libc::c_char,
    'e' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    'c' as i32 as libc::c_char,
    'o' as i32 as libc::c_char,
    'l' as i32 as libc::c_char,
    'o' as i32 as libc::c_char,
    'r' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    'i' as i32 as libc::c_char,
    'n' as i32 as libc::c_char,
    'i' as i32 as libc::c_char,
    't' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    's' as i32 as libc::c_char,
    't' as i32 as libc::c_char,
    'r' as i32 as libc::c_char,
    'i' as i32 as libc::c_char,
    'n' as i32 as libc::c_char,
    'g' as i32 as libc::c_char,
    '.' as i32 as libc::c_char,
    0 as libc::c_int as libc::c_char,
    '#' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    '(' as i32 as libc::c_char,
    'a' as i32 as libc::c_char,
    'n' as i32 as libc::c_char,
    'd' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    'a' as i32 as libc::c_char,
    'n' as i32 as libc::c_char,
    'y' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    'c' as i32 as libc::c_char,
    'o' as i32 as libc::c_char,
    'm' as i32 as libc::c_char,
    'm' as i32 as libc::c_char,
    'e' as i32 as libc::c_char,
    'n' as i32 as libc::c_char,
    't' as i32 as libc::c_char,
    's' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    'y' as i32 as libc::c_char,
    'o' as i32 as libc::c_char,
    'u' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    'w' as i32 as libc::c_char,
    'a' as i32 as libc::c_char,
    'n' as i32 as libc::c_char,
    't' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    't' as i32 as libc::c_char,
    'o' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    'a' as i32 as libc::c_char,
    'd' as i32 as libc::c_char,
    'd' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    'a' as i32 as libc::c_char,
    'f' as i32 as libc::c_char,
    't' as i32 as libc::c_char,
    'e' as i32 as libc::c_char,
    'r' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    'a' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    '\'' as i32 as libc::c_char,
    '#' as i32 as libc::c_char,
    '\'' as i32 as libc::c_char,
    ')' as i32 as libc::c_char,
    0 as libc::c_int as libc::c_char,
    '#' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    'I' as i32 as libc::c_char,
    'f' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    'y' as i32 as libc::c_char,
    'o' as i32 as libc::c_char,
    'u' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    'u' as i32 as libc::c_char,
    's' as i32 as libc::c_char,
    'e' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    'D' as i32 as libc::c_char,
    'O' as i32 as libc::c_char,
    'S' as i32 as libc::c_char,
    '-' as i32 as libc::c_char,
    's' as i32 as libc::c_char,
    't' as i32 as libc::c_char,
    'y' as i32 as libc::c_char,
    'l' as i32 as libc::c_char,
    'e' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    's' as i32 as libc::c_char,
    'u' as i32 as libc::c_char,
    'f' as i32 as libc::c_char,
    'f' as i32 as libc::c_char,
    'i' as i32 as libc::c_char,
    'x' as i32 as libc::c_char,
    'e' as i32 as libc::c_char,
    's' as i32 as libc::c_char,
    ',' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    'y' as i32 as libc::c_char,
    'o' as i32 as libc::c_char,
    'u' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    'm' as i32 as libc::c_char,
    'a' as i32 as libc::c_char,
    'y' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    'w' as i32 as libc::c_char,
    'a' as i32 as libc::c_char,
    'n' as i32 as libc::c_char,
    't' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    't' as i32 as libc::c_char,
    'o' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    'u' as i32 as libc::c_char,
    'n' as i32 as libc::c_char,
    'c' as i32 as libc::c_char,
    'o' as i32 as libc::c_char,
    'm' as i32 as libc::c_char,
    'm' as i32 as libc::c_char,
    'e' as i32 as libc::c_char,
    'n' as i32 as libc::c_char,
    't' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    't' as i32 as libc::c_char,
    'h' as i32 as libc::c_char,
    'e' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    'f' as i32 as libc::c_char,
    'o' as i32 as libc::c_char,
    'l' as i32 as libc::c_char,
    'l' as i32 as libc::c_char,
    'o' as i32 as libc::c_char,
    'w' as i32 as libc::c_char,
    'i' as i32 as libc::c_char,
    'n' as i32 as libc::c_char,
    'g' as i32 as libc::c_char,
    ':' as i32 as libc::c_char,
    0 as libc::c_int as libc::c_char,
    '#' as i32 as libc::c_char,
    '.' as i32 as libc::c_char,
    'c' as i32 as libc::c_char,
    'm' as i32 as libc::c_char,
    'd' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    '0' as i32 as libc::c_char,
    '1' as i32 as libc::c_char,
    ';' as i32 as libc::c_char,
    '3' as i32 as libc::c_char,
    '2' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    '#' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    'e' as i32 as libc::c_char,
    'x' as i32 as libc::c_char,
    'e' as i32 as libc::c_char,
    'c' as i32 as libc::c_char,
    'u' as i32 as libc::c_char,
    't' as i32 as libc::c_char,
    'a' as i32 as libc::c_char,
    'b' as i32 as libc::c_char,
    'l' as i32 as libc::c_char,
    'e' as i32 as libc::c_char,
    's' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    '(' as i32 as libc::c_char,
    'b' as i32 as libc::c_char,
    'r' as i32 as libc::c_char,
    'i' as i32 as libc::c_char,
    'g' as i32 as libc::c_char,
    'h' as i32 as libc::c_char,
    't' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    'g' as i32 as libc::c_char,
    'r' as i32 as libc::c_char,
    'e' as i32 as libc::c_char,
    'e' as i32 as libc::c_char,
    'n' as i32 as libc::c_char,
    ')' as i32 as libc::c_char,
    0 as libc::c_int as libc::c_char,
    '#' as i32 as libc::c_char,
    '.' as i32 as libc::c_char,
    'e' as i32 as libc::c_char,
    'x' as i32 as libc::c_char,
    'e' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    '0' as i32 as libc::c_char,
    '1' as i32 as libc::c_char,
    ';' as i32 as libc::c_char,
    '3' as i32 as libc::c_char,
    '2' as i32 as libc::c_char,
    0 as libc::c_int as libc::c_char,
    '#' as i32 as libc::c_char,
    '.' as i32 as libc::c_char,
    'c' as i32 as libc::c_char,
    'o' as i32 as libc::c_char,
    'm' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    '0' as i32 as libc::c_char,
    '1' as i32 as libc::c_char,
    ';' as i32 as libc::c_char,
    '3' as i32 as libc::c_char,
    '2' as i32 as libc::c_char,
    0 as libc::c_int as libc::c_char,
    '#' as i32 as libc::c_char,
    '.' as i32 as libc::c_char,
    'b' as i32 as libc::c_char,
    't' as i32 as libc::c_char,
    'm' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    '0' as i32 as libc::c_char,
    '1' as i32 as libc::c_char,
    ';' as i32 as libc::c_char,
    '3' as i32 as libc::c_char,
    '2' as i32 as libc::c_char,
    0 as libc::c_int as libc::c_char,
    '#' as i32 as libc::c_char,
    '.' as i32 as libc::c_char,
    'b' as i32 as libc::c_char,
    'a' as i32 as libc::c_char,
    't' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    '0' as i32 as libc::c_char,
    '1' as i32 as libc::c_char,
    ';' as i32 as libc::c_char,
    '3' as i32 as libc::c_char,
    '2' as i32 as libc::c_char,
    0 as libc::c_int as libc::c_char,
    '#' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    'O' as i32 as libc::c_char,
    'r' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    'i' as i32 as libc::c_char,
    'f' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    'y' as i32 as libc::c_char,
    'o' as i32 as libc::c_char,
    'u' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    'w' as i32 as libc::c_char,
    'a' as i32 as libc::c_char,
    'n' as i32 as libc::c_char,
    't' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    't' as i32 as libc::c_char,
    'o' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    'c' as i32 as libc::c_char,
    'o' as i32 as libc::c_char,
    'l' as i32 as libc::c_char,
    'o' as i32 as libc::c_char,
    'r' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    's' as i32 as libc::c_char,
    'c' as i32 as libc::c_char,
    'r' as i32 as libc::c_char,
    'i' as i32 as libc::c_char,
    'p' as i32 as libc::c_char,
    't' as i32 as libc::c_char,
    's' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    'e' as i32 as libc::c_char,
    'v' as i32 as libc::c_char,
    'e' as i32 as libc::c_char,
    'n' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    'i' as i32 as libc::c_char,
    'f' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    't' as i32 as libc::c_char,
    'h' as i32 as libc::c_char,
    'e' as i32 as libc::c_char,
    'y' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    'd' as i32 as libc::c_char,
    'o' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    'n' as i32 as libc::c_char,
    'o' as i32 as libc::c_char,
    't' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    'h' as i32 as libc::c_char,
    'a' as i32 as libc::c_char,
    'v' as i32 as libc::c_char,
    'e' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    't' as i32 as libc::c_char,
    'h' as i32 as libc::c_char,
    'e' as i32 as libc::c_char,
    0 as libc::c_int as libc::c_char,
    '#' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    'e' as i32 as libc::c_char,
    'x' as i32 as libc::c_char,
    'e' as i32 as libc::c_char,
    'c' as i32 as libc::c_char,
    'u' as i32 as libc::c_char,
    't' as i32 as libc::c_char,
    'a' as i32 as libc::c_char,
    'b' as i32 as libc::c_char,
    'l' as i32 as libc::c_char,
    'e' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    'b' as i32 as libc::c_char,
    'i' as i32 as libc::c_char,
    't' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    'a' as i32 as libc::c_char,
    'c' as i32 as libc::c_char,
    't' as i32 as libc::c_char,
    'u' as i32 as libc::c_char,
    'a' as i32 as libc::c_char,
    'l' as i32 as libc::c_char,
    'l' as i32 as libc::c_char,
    'y' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    's' as i32 as libc::c_char,
    'e' as i32 as libc::c_char,
    't' as i32 as libc::c_char,
    '.' as i32 as libc::c_char,
    0 as libc::c_int as libc::c_char,
    '#' as i32 as libc::c_char,
    '.' as i32 as libc::c_char,
    's' as i32 as libc::c_char,
    'h' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    '0' as i32 as libc::c_char,
    '1' as i32 as libc::c_char,
    ';' as i32 as libc::c_char,
    '3' as i32 as libc::c_char,
    '2' as i32 as libc::c_char,
    0 as libc::c_int as libc::c_char,
    '#' as i32 as libc::c_char,
    '.' as i32 as libc::c_char,
    'c' as i32 as libc::c_char,
    's' as i32 as libc::c_char,
    'h' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    '0' as i32 as libc::c_char,
    '1' as i32 as libc::c_char,
    ';' as i32 as libc::c_char,
    '3' as i32 as libc::c_char,
    '2' as i32 as libc::c_char,
    0 as libc::c_int as libc::c_char,
    ' ' as i32 as libc::c_char,
    '#' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    'a' as i32 as libc::c_char,
    'r' as i32 as libc::c_char,
    'c' as i32 as libc::c_char,
    'h' as i32 as libc::c_char,
    'i' as i32 as libc::c_char,
    'v' as i32 as libc::c_char,
    'e' as i32 as libc::c_char,
    's' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    'o' as i32 as libc::c_char,
    'r' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    'c' as i32 as libc::c_char,
    'o' as i32 as libc::c_char,
    'm' as i32 as libc::c_char,
    'p' as i32 as libc::c_char,
    'r' as i32 as libc::c_char,
    'e' as i32 as libc::c_char,
    's' as i32 as libc::c_char,
    's' as i32 as libc::c_char,
    'e' as i32 as libc::c_char,
    'd' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    '(' as i32 as libc::c_char,
    'b' as i32 as libc::c_char,
    'r' as i32 as libc::c_char,
    'i' as i32 as libc::c_char,
    'g' as i32 as libc::c_char,
    'h' as i32 as libc::c_char,
    't' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    'r' as i32 as libc::c_char,
    'e' as i32 as libc::c_char,
    'd' as i32 as libc::c_char,
    ')' as i32 as libc::c_char,
    0 as libc::c_int as libc::c_char,
    '.' as i32 as libc::c_char,
    't' as i32 as libc::c_char,
    'a' as i32 as libc::c_char,
    'r' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    '0' as i32 as libc::c_char,
    '1' as i32 as libc::c_char,
    ';' as i32 as libc::c_char,
    '3' as i32 as libc::c_char,
    '1' as i32 as libc::c_char,
    0 as libc::c_int as libc::c_char,
    '.' as i32 as libc::c_char,
    't' as i32 as libc::c_char,
    'g' as i32 as libc::c_char,
    'z' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    '0' as i32 as libc::c_char,
    '1' as i32 as libc::c_char,
    ';' as i32 as libc::c_char,
    '3' as i32 as libc::c_char,
    '1' as i32 as libc::c_char,
    0 as libc::c_int as libc::c_char,
    '.' as i32 as libc::c_char,
    'a' as i32 as libc::c_char,
    'r' as i32 as libc::c_char,
    'c' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    '0' as i32 as libc::c_char,
    '1' as i32 as libc::c_char,
    ';' as i32 as libc::c_char,
    '3' as i32 as libc::c_char,
    '1' as i32 as libc::c_char,
    0 as libc::c_int as libc::c_char,
    '.' as i32 as libc::c_char,
    'a' as i32 as libc::c_char,
    'r' as i32 as libc::c_char,
    'j' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    '0' as i32 as libc::c_char,
    '1' as i32 as libc::c_char,
    ';' as i32 as libc::c_char,
    '3' as i32 as libc::c_char,
    '1' as i32 as libc::c_char,
    0 as libc::c_int as libc::c_char,
    '.' as i32 as libc::c_char,
    't' as i32 as libc::c_char,
    'a' as i32 as libc::c_char,
    'z' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    '0' as i32 as libc::c_char,
    '1' as i32 as libc::c_char,
    ';' as i32 as libc::c_char,
    '3' as i32 as libc::c_char,
    '1' as i32 as libc::c_char,
    0 as libc::c_int as libc::c_char,
    '.' as i32 as libc::c_char,
    'l' as i32 as libc::c_char,
    'h' as i32 as libc::c_char,
    'a' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    '0' as i32 as libc::c_char,
    '1' as i32 as libc::c_char,
    ';' as i32 as libc::c_char,
    '3' as i32 as libc::c_char,
    '1' as i32 as libc::c_char,
    0 as libc::c_int as libc::c_char,
    '.' as i32 as libc::c_char,
    'l' as i32 as libc::c_char,
    'z' as i32 as libc::c_char,
    '4' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    '0' as i32 as libc::c_char,
    '1' as i32 as libc::c_char,
    ';' as i32 as libc::c_char,
    '3' as i32 as libc::c_char,
    '1' as i32 as libc::c_char,
    0 as libc::c_int as libc::c_char,
    '.' as i32 as libc::c_char,
    'l' as i32 as libc::c_char,
    'z' as i32 as libc::c_char,
    'h' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    '0' as i32 as libc::c_char,
    '1' as i32 as libc::c_char,
    ';' as i32 as libc::c_char,
    '3' as i32 as libc::c_char,
    '1' as i32 as libc::c_char,
    0 as libc::c_int as libc::c_char,
    '.' as i32 as libc::c_char,
    'l' as i32 as libc::c_char,
    'z' as i32 as libc::c_char,
    'm' as i32 as libc::c_char,
    'a' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    '0' as i32 as libc::c_char,
    '1' as i32 as libc::c_char,
    ';' as i32 as libc::c_char,
    '3' as i32 as libc::c_char,
    '1' as i32 as libc::c_char,
    0 as libc::c_int as libc::c_char,
    '.' as i32 as libc::c_char,
    't' as i32 as libc::c_char,
    'l' as i32 as libc::c_char,
    'z' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    '0' as i32 as libc::c_char,
    '1' as i32 as libc::c_char,
    ';' as i32 as libc::c_char,
    '3' as i32 as libc::c_char,
    '1' as i32 as libc::c_char,
    0 as libc::c_int as libc::c_char,
    '.' as i32 as libc::c_char,
    't' as i32 as libc::c_char,
    'x' as i32 as libc::c_char,
    'z' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    '0' as i32 as libc::c_char,
    '1' as i32 as libc::c_char,
    ';' as i32 as libc::c_char,
    '3' as i32 as libc::c_char,
    '1' as i32 as libc::c_char,
    0 as libc::c_int as libc::c_char,
    '.' as i32 as libc::c_char,
    't' as i32 as libc::c_char,
    'z' as i32 as libc::c_char,
    'o' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    '0' as i32 as libc::c_char,
    '1' as i32 as libc::c_char,
    ';' as i32 as libc::c_char,
    '3' as i32 as libc::c_char,
    '1' as i32 as libc::c_char,
    0 as libc::c_int as libc::c_char,
    '.' as i32 as libc::c_char,
    't' as i32 as libc::c_char,
    '7' as i32 as libc::c_char,
    'z' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    '0' as i32 as libc::c_char,
    '1' as i32 as libc::c_char,
    ';' as i32 as libc::c_char,
    '3' as i32 as libc::c_char,
    '1' as i32 as libc::c_char,
    0 as libc::c_int as libc::c_char,
    '.' as i32 as libc::c_char,
    'z' as i32 as libc::c_char,
    'i' as i32 as libc::c_char,
    'p' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    '0' as i32 as libc::c_char,
    '1' as i32 as libc::c_char,
    ';' as i32 as libc::c_char,
    '3' as i32 as libc::c_char,
    '1' as i32 as libc::c_char,
    0 as libc::c_int as libc::c_char,
    '.' as i32 as libc::c_char,
    'z' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    '0' as i32 as libc::c_char,
    '1' as i32 as libc::c_char,
    ';' as i32 as libc::c_char,
    '3' as i32 as libc::c_char,
    '1' as i32 as libc::c_char,
    0 as libc::c_int as libc::c_char,
    '.' as i32 as libc::c_char,
    'd' as i32 as libc::c_char,
    'z' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    '0' as i32 as libc::c_char,
    '1' as i32 as libc::c_char,
    ';' as i32 as libc::c_char,
    '3' as i32 as libc::c_char,
    '1' as i32 as libc::c_char,
    0 as libc::c_int as libc::c_char,
    '.' as i32 as libc::c_char,
    'g' as i32 as libc::c_char,
    'z' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    '0' as i32 as libc::c_char,
    '1' as i32 as libc::c_char,
    ';' as i32 as libc::c_char,
    '3' as i32 as libc::c_char,
    '1' as i32 as libc::c_char,
    0 as libc::c_int as libc::c_char,
    '.' as i32 as libc::c_char,
    'l' as i32 as libc::c_char,
    'r' as i32 as libc::c_char,
    'z' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    '0' as i32 as libc::c_char,
    '1' as i32 as libc::c_char,
    ';' as i32 as libc::c_char,
    '3' as i32 as libc::c_char,
    '1' as i32 as libc::c_char,
    0 as libc::c_int as libc::c_char,
    '.' as i32 as libc::c_char,
    'l' as i32 as libc::c_char,
    'z' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    '0' as i32 as libc::c_char,
    '1' as i32 as libc::c_char,
    ';' as i32 as libc::c_char,
    '3' as i32 as libc::c_char,
    '1' as i32 as libc::c_char,
    0 as libc::c_int as libc::c_char,
    '.' as i32 as libc::c_char,
    'l' as i32 as libc::c_char,
    'z' as i32 as libc::c_char,
    'o' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    '0' as i32 as libc::c_char,
    '1' as i32 as libc::c_char,
    ';' as i32 as libc::c_char,
    '3' as i32 as libc::c_char,
    '1' as i32 as libc::c_char,
    0 as libc::c_int as libc::c_char,
    '.' as i32 as libc::c_char,
    'x' as i32 as libc::c_char,
    'z' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    '0' as i32 as libc::c_char,
    '1' as i32 as libc::c_char,
    ';' as i32 as libc::c_char,
    '3' as i32 as libc::c_char,
    '1' as i32 as libc::c_char,
    0 as libc::c_int as libc::c_char,
    '.' as i32 as libc::c_char,
    'z' as i32 as libc::c_char,
    's' as i32 as libc::c_char,
    't' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    '0' as i32 as libc::c_char,
    '1' as i32 as libc::c_char,
    ';' as i32 as libc::c_char,
    '3' as i32 as libc::c_char,
    '1' as i32 as libc::c_char,
    0 as libc::c_int as libc::c_char,
    '.' as i32 as libc::c_char,
    't' as i32 as libc::c_char,
    'z' as i32 as libc::c_char,
    's' as i32 as libc::c_char,
    't' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    '0' as i32 as libc::c_char,
    '1' as i32 as libc::c_char,
    ';' as i32 as libc::c_char,
    '3' as i32 as libc::c_char,
    '1' as i32 as libc::c_char,
    0 as libc::c_int as libc::c_char,
    '.' as i32 as libc::c_char,
    'b' as i32 as libc::c_char,
    'z' as i32 as libc::c_char,
    '2' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    '0' as i32 as libc::c_char,
    '1' as i32 as libc::c_char,
    ';' as i32 as libc::c_char,
    '3' as i32 as libc::c_char,
    '1' as i32 as libc::c_char,
    0 as libc::c_int as libc::c_char,
    '.' as i32 as libc::c_char,
    'b' as i32 as libc::c_char,
    'z' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    '0' as i32 as libc::c_char,
    '1' as i32 as libc::c_char,
    ';' as i32 as libc::c_char,
    '3' as i32 as libc::c_char,
    '1' as i32 as libc::c_char,
    0 as libc::c_int as libc::c_char,
    '.' as i32 as libc::c_char,
    't' as i32 as libc::c_char,
    'b' as i32 as libc::c_char,
    'z' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    '0' as i32 as libc::c_char,
    '1' as i32 as libc::c_char,
    ';' as i32 as libc::c_char,
    '3' as i32 as libc::c_char,
    '1' as i32 as libc::c_char,
    0 as libc::c_int as libc::c_char,
    '.' as i32 as libc::c_char,
    't' as i32 as libc::c_char,
    'b' as i32 as libc::c_char,
    'z' as i32 as libc::c_char,
    '2' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    '0' as i32 as libc::c_char,
    '1' as i32 as libc::c_char,
    ';' as i32 as libc::c_char,
    '3' as i32 as libc::c_char,
    '1' as i32 as libc::c_char,
    0 as libc::c_int as libc::c_char,
    '.' as i32 as libc::c_char,
    't' as i32 as libc::c_char,
    'z' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    '0' as i32 as libc::c_char,
    '1' as i32 as libc::c_char,
    ';' as i32 as libc::c_char,
    '3' as i32 as libc::c_char,
    '1' as i32 as libc::c_char,
    0 as libc::c_int as libc::c_char,
    '.' as i32 as libc::c_char,
    'd' as i32 as libc::c_char,
    'e' as i32 as libc::c_char,
    'b' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    '0' as i32 as libc::c_char,
    '1' as i32 as libc::c_char,
    ';' as i32 as libc::c_char,
    '3' as i32 as libc::c_char,
    '1' as i32 as libc::c_char,
    0 as libc::c_int as libc::c_char,
    '.' as i32 as libc::c_char,
    'r' as i32 as libc::c_char,
    'p' as i32 as libc::c_char,
    'm' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    '0' as i32 as libc::c_char,
    '1' as i32 as libc::c_char,
    ';' as i32 as libc::c_char,
    '3' as i32 as libc::c_char,
    '1' as i32 as libc::c_char,
    0 as libc::c_int as libc::c_char,
    '.' as i32 as libc::c_char,
    'j' as i32 as libc::c_char,
    'a' as i32 as libc::c_char,
    'r' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    '0' as i32 as libc::c_char,
    '1' as i32 as libc::c_char,
    ';' as i32 as libc::c_char,
    '3' as i32 as libc::c_char,
    '1' as i32 as libc::c_char,
    0 as libc::c_int as libc::c_char,
    '.' as i32 as libc::c_char,
    'w' as i32 as libc::c_char,
    'a' as i32 as libc::c_char,
    'r' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    '0' as i32 as libc::c_char,
    '1' as i32 as libc::c_char,
    ';' as i32 as libc::c_char,
    '3' as i32 as libc::c_char,
    '1' as i32 as libc::c_char,
    0 as libc::c_int as libc::c_char,
    '.' as i32 as libc::c_char,
    'e' as i32 as libc::c_char,
    'a' as i32 as libc::c_char,
    'r' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    '0' as i32 as libc::c_char,
    '1' as i32 as libc::c_char,
    ';' as i32 as libc::c_char,
    '3' as i32 as libc::c_char,
    '1' as i32 as libc::c_char,
    0 as libc::c_int as libc::c_char,
    '.' as i32 as libc::c_char,
    's' as i32 as libc::c_char,
    'a' as i32 as libc::c_char,
    'r' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    '0' as i32 as libc::c_char,
    '1' as i32 as libc::c_char,
    ';' as i32 as libc::c_char,
    '3' as i32 as libc::c_char,
    '1' as i32 as libc::c_char,
    0 as libc::c_int as libc::c_char,
    '.' as i32 as libc::c_char,
    'r' as i32 as libc::c_char,
    'a' as i32 as libc::c_char,
    'r' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    '0' as i32 as libc::c_char,
    '1' as i32 as libc::c_char,
    ';' as i32 as libc::c_char,
    '3' as i32 as libc::c_char,
    '1' as i32 as libc::c_char,
    0 as libc::c_int as libc::c_char,
    '.' as i32 as libc::c_char,
    'a' as i32 as libc::c_char,
    'l' as i32 as libc::c_char,
    'z' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    '0' as i32 as libc::c_char,
    '1' as i32 as libc::c_char,
    ';' as i32 as libc::c_char,
    '3' as i32 as libc::c_char,
    '1' as i32 as libc::c_char,
    0 as libc::c_int as libc::c_char,
    '.' as i32 as libc::c_char,
    'a' as i32 as libc::c_char,
    'c' as i32 as libc::c_char,
    'e' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    '0' as i32 as libc::c_char,
    '1' as i32 as libc::c_char,
    ';' as i32 as libc::c_char,
    '3' as i32 as libc::c_char,
    '1' as i32 as libc::c_char,
    0 as libc::c_int as libc::c_char,
    '.' as i32 as libc::c_char,
    'z' as i32 as libc::c_char,
    'o' as i32 as libc::c_char,
    'o' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    '0' as i32 as libc::c_char,
    '1' as i32 as libc::c_char,
    ';' as i32 as libc::c_char,
    '3' as i32 as libc::c_char,
    '1' as i32 as libc::c_char,
    0 as libc::c_int as libc::c_char,
    '.' as i32 as libc::c_char,
    'c' as i32 as libc::c_char,
    'p' as i32 as libc::c_char,
    'i' as i32 as libc::c_char,
    'o' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    '0' as i32 as libc::c_char,
    '1' as i32 as libc::c_char,
    ';' as i32 as libc::c_char,
    '3' as i32 as libc::c_char,
    '1' as i32 as libc::c_char,
    0 as libc::c_int as libc::c_char,
    '.' as i32 as libc::c_char,
    '7' as i32 as libc::c_char,
    'z' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    '0' as i32 as libc::c_char,
    '1' as i32 as libc::c_char,
    ';' as i32 as libc::c_char,
    '3' as i32 as libc::c_char,
    '1' as i32 as libc::c_char,
    0 as libc::c_int as libc::c_char,
    '.' as i32 as libc::c_char,
    'r' as i32 as libc::c_char,
    'z' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    '0' as i32 as libc::c_char,
    '1' as i32 as libc::c_char,
    ';' as i32 as libc::c_char,
    '3' as i32 as libc::c_char,
    '1' as i32 as libc::c_char,
    0 as libc::c_int as libc::c_char,
    '.' as i32 as libc::c_char,
    'c' as i32 as libc::c_char,
    'a' as i32 as libc::c_char,
    'b' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    '0' as i32 as libc::c_char,
    '1' as i32 as libc::c_char,
    ';' as i32 as libc::c_char,
    '3' as i32 as libc::c_char,
    '1' as i32 as libc::c_char,
    0 as libc::c_int as libc::c_char,
    '.' as i32 as libc::c_char,
    'w' as i32 as libc::c_char,
    'i' as i32 as libc::c_char,
    'm' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    '0' as i32 as libc::c_char,
    '1' as i32 as libc::c_char,
    ';' as i32 as libc::c_char,
    '3' as i32 as libc::c_char,
    '1' as i32 as libc::c_char,
    0 as libc::c_int as libc::c_char,
    '.' as i32 as libc::c_char,
    's' as i32 as libc::c_char,
    'w' as i32 as libc::c_char,
    'm' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    '0' as i32 as libc::c_char,
    '1' as i32 as libc::c_char,
    ';' as i32 as libc::c_char,
    '3' as i32 as libc::c_char,
    '1' as i32 as libc::c_char,
    0 as libc::c_int as libc::c_char,
    '.' as i32 as libc::c_char,
    'd' as i32 as libc::c_char,
    'w' as i32 as libc::c_char,
    'm' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    '0' as i32 as libc::c_char,
    '1' as i32 as libc::c_char,
    ';' as i32 as libc::c_char,
    '3' as i32 as libc::c_char,
    '1' as i32 as libc::c_char,
    0 as libc::c_int as libc::c_char,
    '.' as i32 as libc::c_char,
    'e' as i32 as libc::c_char,
    's' as i32 as libc::c_char,
    'd' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    '0' as i32 as libc::c_char,
    '1' as i32 as libc::c_char,
    ';' as i32 as libc::c_char,
    '3' as i32 as libc::c_char,
    '1' as i32 as libc::c_char,
    0 as libc::c_int as libc::c_char,
    '#' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    'i' as i32 as libc::c_char,
    'm' as i32 as libc::c_char,
    'a' as i32 as libc::c_char,
    'g' as i32 as libc::c_char,
    'e' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    'f' as i32 as libc::c_char,
    'o' as i32 as libc::c_char,
    'r' as i32 as libc::c_char,
    'm' as i32 as libc::c_char,
    'a' as i32 as libc::c_char,
    't' as i32 as libc::c_char,
    's' as i32 as libc::c_char,
    0 as libc::c_int as libc::c_char,
    '.' as i32 as libc::c_char,
    'a' as i32 as libc::c_char,
    'v' as i32 as libc::c_char,
    'i' as i32 as libc::c_char,
    'f' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    '0' as i32 as libc::c_char,
    '1' as i32 as libc::c_char,
    ';' as i32 as libc::c_char,
    '3' as i32 as libc::c_char,
    '5' as i32 as libc::c_char,
    0 as libc::c_int as libc::c_char,
    '.' as i32 as libc::c_char,
    'j' as i32 as libc::c_char,
    'p' as i32 as libc::c_char,
    'g' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    '0' as i32 as libc::c_char,
    '1' as i32 as libc::c_char,
    ';' as i32 as libc::c_char,
    '3' as i32 as libc::c_char,
    '5' as i32 as libc::c_char,
    0 as libc::c_int as libc::c_char,
    '.' as i32 as libc::c_char,
    'j' as i32 as libc::c_char,
    'p' as i32 as libc::c_char,
    'e' as i32 as libc::c_char,
    'g' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    '0' as i32 as libc::c_char,
    '1' as i32 as libc::c_char,
    ';' as i32 as libc::c_char,
    '3' as i32 as libc::c_char,
    '5' as i32 as libc::c_char,
    0 as libc::c_int as libc::c_char,
    '.' as i32 as libc::c_char,
    'm' as i32 as libc::c_char,
    'j' as i32 as libc::c_char,
    'p' as i32 as libc::c_char,
    'g' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    '0' as i32 as libc::c_char,
    '1' as i32 as libc::c_char,
    ';' as i32 as libc::c_char,
    '3' as i32 as libc::c_char,
    '5' as i32 as libc::c_char,
    0 as libc::c_int as libc::c_char,
    '.' as i32 as libc::c_char,
    'm' as i32 as libc::c_char,
    'j' as i32 as libc::c_char,
    'p' as i32 as libc::c_char,
    'e' as i32 as libc::c_char,
    'g' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    '0' as i32 as libc::c_char,
    '1' as i32 as libc::c_char,
    ';' as i32 as libc::c_char,
    '3' as i32 as libc::c_char,
    '5' as i32 as libc::c_char,
    0 as libc::c_int as libc::c_char,
    '.' as i32 as libc::c_char,
    'g' as i32 as libc::c_char,
    'i' as i32 as libc::c_char,
    'f' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    '0' as i32 as libc::c_char,
    '1' as i32 as libc::c_char,
    ';' as i32 as libc::c_char,
    '3' as i32 as libc::c_char,
    '5' as i32 as libc::c_char,
    0 as libc::c_int as libc::c_char,
    '.' as i32 as libc::c_char,
    'b' as i32 as libc::c_char,
    'm' as i32 as libc::c_char,
    'p' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    '0' as i32 as libc::c_char,
    '1' as i32 as libc::c_char,
    ';' as i32 as libc::c_char,
    '3' as i32 as libc::c_char,
    '5' as i32 as libc::c_char,
    0 as libc::c_int as libc::c_char,
    '.' as i32 as libc::c_char,
    'p' as i32 as libc::c_char,
    'b' as i32 as libc::c_char,
    'm' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    '0' as i32 as libc::c_char,
    '1' as i32 as libc::c_char,
    ';' as i32 as libc::c_char,
    '3' as i32 as libc::c_char,
    '5' as i32 as libc::c_char,
    0 as libc::c_int as libc::c_char,
    '.' as i32 as libc::c_char,
    'p' as i32 as libc::c_char,
    'g' as i32 as libc::c_char,
    'm' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    '0' as i32 as libc::c_char,
    '1' as i32 as libc::c_char,
    ';' as i32 as libc::c_char,
    '3' as i32 as libc::c_char,
    '5' as i32 as libc::c_char,
    0 as libc::c_int as libc::c_char,
    '.' as i32 as libc::c_char,
    'p' as i32 as libc::c_char,
    'p' as i32 as libc::c_char,
    'm' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    '0' as i32 as libc::c_char,
    '1' as i32 as libc::c_char,
    ';' as i32 as libc::c_char,
    '3' as i32 as libc::c_char,
    '5' as i32 as libc::c_char,
    0 as libc::c_int as libc::c_char,
    '.' as i32 as libc::c_char,
    't' as i32 as libc::c_char,
    'g' as i32 as libc::c_char,
    'a' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    '0' as i32 as libc::c_char,
    '1' as i32 as libc::c_char,
    ';' as i32 as libc::c_char,
    '3' as i32 as libc::c_char,
    '5' as i32 as libc::c_char,
    0 as libc::c_int as libc::c_char,
    '.' as i32 as libc::c_char,
    'x' as i32 as libc::c_char,
    'b' as i32 as libc::c_char,
    'm' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    '0' as i32 as libc::c_char,
    '1' as i32 as libc::c_char,
    ';' as i32 as libc::c_char,
    '3' as i32 as libc::c_char,
    '5' as i32 as libc::c_char,
    0 as libc::c_int as libc::c_char,
    '.' as i32 as libc::c_char,
    'x' as i32 as libc::c_char,
    'p' as i32 as libc::c_char,
    'm' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    '0' as i32 as libc::c_char,
    '1' as i32 as libc::c_char,
    ';' as i32 as libc::c_char,
    '3' as i32 as libc::c_char,
    '5' as i32 as libc::c_char,
    0 as libc::c_int as libc::c_char,
    '.' as i32 as libc::c_char,
    't' as i32 as libc::c_char,
    'i' as i32 as libc::c_char,
    'f' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    '0' as i32 as libc::c_char,
    '1' as i32 as libc::c_char,
    ';' as i32 as libc::c_char,
    '3' as i32 as libc::c_char,
    '5' as i32 as libc::c_char,
    0 as libc::c_int as libc::c_char,
    '.' as i32 as libc::c_char,
    't' as i32 as libc::c_char,
    'i' as i32 as libc::c_char,
    'f' as i32 as libc::c_char,
    'f' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    '0' as i32 as libc::c_char,
    '1' as i32 as libc::c_char,
    ';' as i32 as libc::c_char,
    '3' as i32 as libc::c_char,
    '5' as i32 as libc::c_char,
    0 as libc::c_int as libc::c_char,
    '.' as i32 as libc::c_char,
    'p' as i32 as libc::c_char,
    'n' as i32 as libc::c_char,
    'g' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    '0' as i32 as libc::c_char,
    '1' as i32 as libc::c_char,
    ';' as i32 as libc::c_char,
    '3' as i32 as libc::c_char,
    '5' as i32 as libc::c_char,
    0 as libc::c_int as libc::c_char,
    '.' as i32 as libc::c_char,
    's' as i32 as libc::c_char,
    'v' as i32 as libc::c_char,
    'g' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    '0' as i32 as libc::c_char,
    '1' as i32 as libc::c_char,
    ';' as i32 as libc::c_char,
    '3' as i32 as libc::c_char,
    '5' as i32 as libc::c_char,
    0 as libc::c_int as libc::c_char,
    '.' as i32 as libc::c_char,
    's' as i32 as libc::c_char,
    'v' as i32 as libc::c_char,
    'g' as i32 as libc::c_char,
    'z' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    '0' as i32 as libc::c_char,
    '1' as i32 as libc::c_char,
    ';' as i32 as libc::c_char,
    '3' as i32 as libc::c_char,
    '5' as i32 as libc::c_char,
    0 as libc::c_int as libc::c_char,
    '.' as i32 as libc::c_char,
    'm' as i32 as libc::c_char,
    'n' as i32 as libc::c_char,
    'g' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    '0' as i32 as libc::c_char,
    '1' as i32 as libc::c_char,
    ';' as i32 as libc::c_char,
    '3' as i32 as libc::c_char,
    '5' as i32 as libc::c_char,
    0 as libc::c_int as libc::c_char,
    '.' as i32 as libc::c_char,
    'p' as i32 as libc::c_char,
    'c' as i32 as libc::c_char,
    'x' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    '0' as i32 as libc::c_char,
    '1' as i32 as libc::c_char,
    ';' as i32 as libc::c_char,
    '3' as i32 as libc::c_char,
    '5' as i32 as libc::c_char,
    0 as libc::c_int as libc::c_char,
    '.' as i32 as libc::c_char,
    'm' as i32 as libc::c_char,
    'o' as i32 as libc::c_char,
    'v' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    '0' as i32 as libc::c_char,
    '1' as i32 as libc::c_char,
    ';' as i32 as libc::c_char,
    '3' as i32 as libc::c_char,
    '5' as i32 as libc::c_char,
    0 as libc::c_int as libc::c_char,
    '.' as i32 as libc::c_char,
    'm' as i32 as libc::c_char,
    'p' as i32 as libc::c_char,
    'g' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    '0' as i32 as libc::c_char,
    '1' as i32 as libc::c_char,
    ';' as i32 as libc::c_char,
    '3' as i32 as libc::c_char,
    '5' as i32 as libc::c_char,
    0 as libc::c_int as libc::c_char,
    '.' as i32 as libc::c_char,
    'm' as i32 as libc::c_char,
    'p' as i32 as libc::c_char,
    'e' as i32 as libc::c_char,
    'g' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    '0' as i32 as libc::c_char,
    '1' as i32 as libc::c_char,
    ';' as i32 as libc::c_char,
    '3' as i32 as libc::c_char,
    '5' as i32 as libc::c_char,
    0 as libc::c_int as libc::c_char,
    '.' as i32 as libc::c_char,
    'm' as i32 as libc::c_char,
    '2' as i32 as libc::c_char,
    'v' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    '0' as i32 as libc::c_char,
    '1' as i32 as libc::c_char,
    ';' as i32 as libc::c_char,
    '3' as i32 as libc::c_char,
    '5' as i32 as libc::c_char,
    0 as libc::c_int as libc::c_char,
    '.' as i32 as libc::c_char,
    'm' as i32 as libc::c_char,
    'k' as i32 as libc::c_char,
    'v' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    '0' as i32 as libc::c_char,
    '1' as i32 as libc::c_char,
    ';' as i32 as libc::c_char,
    '3' as i32 as libc::c_char,
    '5' as i32 as libc::c_char,
    0 as libc::c_int as libc::c_char,
    '.' as i32 as libc::c_char,
    'w' as i32 as libc::c_char,
    'e' as i32 as libc::c_char,
    'b' as i32 as libc::c_char,
    'm' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    '0' as i32 as libc::c_char,
    '1' as i32 as libc::c_char,
    ';' as i32 as libc::c_char,
    '3' as i32 as libc::c_char,
    '5' as i32 as libc::c_char,
    0 as libc::c_int as libc::c_char,
    '.' as i32 as libc::c_char,
    'w' as i32 as libc::c_char,
    'e' as i32 as libc::c_char,
    'b' as i32 as libc::c_char,
    'p' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    '0' as i32 as libc::c_char,
    '1' as i32 as libc::c_char,
    ';' as i32 as libc::c_char,
    '3' as i32 as libc::c_char,
    '5' as i32 as libc::c_char,
    0 as libc::c_int as libc::c_char,
    '.' as i32 as libc::c_char,
    'o' as i32 as libc::c_char,
    'g' as i32 as libc::c_char,
    'm' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    '0' as i32 as libc::c_char,
    '1' as i32 as libc::c_char,
    ';' as i32 as libc::c_char,
    '3' as i32 as libc::c_char,
    '5' as i32 as libc::c_char,
    0 as libc::c_int as libc::c_char,
    '.' as i32 as libc::c_char,
    'm' as i32 as libc::c_char,
    'p' as i32 as libc::c_char,
    '4' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    '0' as i32 as libc::c_char,
    '1' as i32 as libc::c_char,
    ';' as i32 as libc::c_char,
    '3' as i32 as libc::c_char,
    '5' as i32 as libc::c_char,
    0 as libc::c_int as libc::c_char,
    '.' as i32 as libc::c_char,
    'm' as i32 as libc::c_char,
    '4' as i32 as libc::c_char,
    'v' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    '0' as i32 as libc::c_char,
    '1' as i32 as libc::c_char,
    ';' as i32 as libc::c_char,
    '3' as i32 as libc::c_char,
    '5' as i32 as libc::c_char,
    0 as libc::c_int as libc::c_char,
    '.' as i32 as libc::c_char,
    'm' as i32 as libc::c_char,
    'p' as i32 as libc::c_char,
    '4' as i32 as libc::c_char,
    'v' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    '0' as i32 as libc::c_char,
    '1' as i32 as libc::c_char,
    ';' as i32 as libc::c_char,
    '3' as i32 as libc::c_char,
    '5' as i32 as libc::c_char,
    0 as libc::c_int as libc::c_char,
    '.' as i32 as libc::c_char,
    'v' as i32 as libc::c_char,
    'o' as i32 as libc::c_char,
    'b' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    '0' as i32 as libc::c_char,
    '1' as i32 as libc::c_char,
    ';' as i32 as libc::c_char,
    '3' as i32 as libc::c_char,
    '5' as i32 as libc::c_char,
    0 as libc::c_int as libc::c_char,
    '.' as i32 as libc::c_char,
    'q' as i32 as libc::c_char,
    't' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    '0' as i32 as libc::c_char,
    '1' as i32 as libc::c_char,
    ';' as i32 as libc::c_char,
    '3' as i32 as libc::c_char,
    '5' as i32 as libc::c_char,
    0 as libc::c_int as libc::c_char,
    '.' as i32 as libc::c_char,
    'n' as i32 as libc::c_char,
    'u' as i32 as libc::c_char,
    'v' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    '0' as i32 as libc::c_char,
    '1' as i32 as libc::c_char,
    ';' as i32 as libc::c_char,
    '3' as i32 as libc::c_char,
    '5' as i32 as libc::c_char,
    0 as libc::c_int as libc::c_char,
    '.' as i32 as libc::c_char,
    'w' as i32 as libc::c_char,
    'm' as i32 as libc::c_char,
    'v' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    '0' as i32 as libc::c_char,
    '1' as i32 as libc::c_char,
    ';' as i32 as libc::c_char,
    '3' as i32 as libc::c_char,
    '5' as i32 as libc::c_char,
    0 as libc::c_int as libc::c_char,
    '.' as i32 as libc::c_char,
    'a' as i32 as libc::c_char,
    's' as i32 as libc::c_char,
    'f' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    '0' as i32 as libc::c_char,
    '1' as i32 as libc::c_char,
    ';' as i32 as libc::c_char,
    '3' as i32 as libc::c_char,
    '5' as i32 as libc::c_char,
    0 as libc::c_int as libc::c_char,
    '.' as i32 as libc::c_char,
    'r' as i32 as libc::c_char,
    'm' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    '0' as i32 as libc::c_char,
    '1' as i32 as libc::c_char,
    ';' as i32 as libc::c_char,
    '3' as i32 as libc::c_char,
    '5' as i32 as libc::c_char,
    0 as libc::c_int as libc::c_char,
    '.' as i32 as libc::c_char,
    'r' as i32 as libc::c_char,
    'm' as i32 as libc::c_char,
    'v' as i32 as libc::c_char,
    'b' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    '0' as i32 as libc::c_char,
    '1' as i32 as libc::c_char,
    ';' as i32 as libc::c_char,
    '3' as i32 as libc::c_char,
    '5' as i32 as libc::c_char,
    0 as libc::c_int as libc::c_char,
    '.' as i32 as libc::c_char,
    'f' as i32 as libc::c_char,
    'l' as i32 as libc::c_char,
    'c' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    '0' as i32 as libc::c_char,
    '1' as i32 as libc::c_char,
    ';' as i32 as libc::c_char,
    '3' as i32 as libc::c_char,
    '5' as i32 as libc::c_char,
    0 as libc::c_int as libc::c_char,
    '.' as i32 as libc::c_char,
    'a' as i32 as libc::c_char,
    'v' as i32 as libc::c_char,
    'i' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    '0' as i32 as libc::c_char,
    '1' as i32 as libc::c_char,
    ';' as i32 as libc::c_char,
    '3' as i32 as libc::c_char,
    '5' as i32 as libc::c_char,
    0 as libc::c_int as libc::c_char,
    '.' as i32 as libc::c_char,
    'f' as i32 as libc::c_char,
    'l' as i32 as libc::c_char,
    'i' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    '0' as i32 as libc::c_char,
    '1' as i32 as libc::c_char,
    ';' as i32 as libc::c_char,
    '3' as i32 as libc::c_char,
    '5' as i32 as libc::c_char,
    0 as libc::c_int as libc::c_char,
    '.' as i32 as libc::c_char,
    'f' as i32 as libc::c_char,
    'l' as i32 as libc::c_char,
    'v' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    '0' as i32 as libc::c_char,
    '1' as i32 as libc::c_char,
    ';' as i32 as libc::c_char,
    '3' as i32 as libc::c_char,
    '5' as i32 as libc::c_char,
    0 as libc::c_int as libc::c_char,
    '.' as i32 as libc::c_char,
    'g' as i32 as libc::c_char,
    'l' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    '0' as i32 as libc::c_char,
    '1' as i32 as libc::c_char,
    ';' as i32 as libc::c_char,
    '3' as i32 as libc::c_char,
    '5' as i32 as libc::c_char,
    0 as libc::c_int as libc::c_char,
    '.' as i32 as libc::c_char,
    'd' as i32 as libc::c_char,
    'l' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    '0' as i32 as libc::c_char,
    '1' as i32 as libc::c_char,
    ';' as i32 as libc::c_char,
    '3' as i32 as libc::c_char,
    '5' as i32 as libc::c_char,
    0 as libc::c_int as libc::c_char,
    '.' as i32 as libc::c_char,
    'x' as i32 as libc::c_char,
    'c' as i32 as libc::c_char,
    'f' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    '0' as i32 as libc::c_char,
    '1' as i32 as libc::c_char,
    ';' as i32 as libc::c_char,
    '3' as i32 as libc::c_char,
    '5' as i32 as libc::c_char,
    0 as libc::c_int as libc::c_char,
    '.' as i32 as libc::c_char,
    'x' as i32 as libc::c_char,
    'w' as i32 as libc::c_char,
    'd' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    '0' as i32 as libc::c_char,
    '1' as i32 as libc::c_char,
    ';' as i32 as libc::c_char,
    '3' as i32 as libc::c_char,
    '5' as i32 as libc::c_char,
    0 as libc::c_int as libc::c_char,
    '.' as i32 as libc::c_char,
    'y' as i32 as libc::c_char,
    'u' as i32 as libc::c_char,
    'v' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    '0' as i32 as libc::c_char,
    '1' as i32 as libc::c_char,
    ';' as i32 as libc::c_char,
    '3' as i32 as libc::c_char,
    '5' as i32 as libc::c_char,
    0 as libc::c_int as libc::c_char,
    '.' as i32 as libc::c_char,
    'c' as i32 as libc::c_char,
    'g' as i32 as libc::c_char,
    'm' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    '0' as i32 as libc::c_char,
    '1' as i32 as libc::c_char,
    ';' as i32 as libc::c_char,
    '3' as i32 as libc::c_char,
    '5' as i32 as libc::c_char,
    0 as libc::c_int as libc::c_char,
    '.' as i32 as libc::c_char,
    'e' as i32 as libc::c_char,
    'm' as i32 as libc::c_char,
    'f' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    '0' as i32 as libc::c_char,
    '1' as i32 as libc::c_char,
    ';' as i32 as libc::c_char,
    '3' as i32 as libc::c_char,
    '5' as i32 as libc::c_char,
    0 as libc::c_int as libc::c_char,
    '#' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    'h' as i32 as libc::c_char,
    't' as i32 as libc::c_char,
    't' as i32 as libc::c_char,
    'p' as i32 as libc::c_char,
    's' as i32 as libc::c_char,
    ':' as i32 as libc::c_char,
    '/' as i32 as libc::c_char,
    '/' as i32 as libc::c_char,
    'w' as i32 as libc::c_char,
    'i' as i32 as libc::c_char,
    'k' as i32 as libc::c_char,
    'i' as i32 as libc::c_char,
    '.' as i32 as libc::c_char,
    'x' as i32 as libc::c_char,
    'i' as i32 as libc::c_char,
    'p' as i32 as libc::c_char,
    'h' as i32 as libc::c_char,
    '.' as i32 as libc::c_char,
    'o' as i32 as libc::c_char,
    'r' as i32 as libc::c_char,
    'g' as i32 as libc::c_char,
    '/' as i32 as libc::c_char,
    'M' as i32 as libc::c_char,
    'I' as i32 as libc::c_char,
    'M' as i32 as libc::c_char,
    'E' as i32 as libc::c_char,
    '_' as i32 as libc::c_char,
    'T' as i32 as libc::c_char,
    'y' as i32 as libc::c_char,
    'p' as i32 as libc::c_char,
    'e' as i32 as libc::c_char,
    's' as i32 as libc::c_char,
    '_' as i32 as libc::c_char,
    'a' as i32 as libc::c_char,
    'n' as i32 as libc::c_char,
    'd' as i32 as libc::c_char,
    '_' as i32 as libc::c_char,
    'F' as i32 as libc::c_char,
    'i' as i32 as libc::c_char,
    'l' as i32 as libc::c_char,
    'e' as i32 as libc::c_char,
    '_' as i32 as libc::c_char,
    'E' as i32 as libc::c_char,
    'x' as i32 as libc::c_char,
    't' as i32 as libc::c_char,
    'e' as i32 as libc::c_char,
    'n' as i32 as libc::c_char,
    's' as i32 as libc::c_char,
    'i' as i32 as libc::c_char,
    'o' as i32 as libc::c_char,
    'n' as i32 as libc::c_char,
    's' as i32 as libc::c_char,
    0 as libc::c_int as libc::c_char,
    '.' as i32 as libc::c_char,
    'o' as i32 as libc::c_char,
    'g' as i32 as libc::c_char,
    'v' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    '0' as i32 as libc::c_char,
    '1' as i32 as libc::c_char,
    ';' as i32 as libc::c_char,
    '3' as i32 as libc::c_char,
    '5' as i32 as libc::c_char,
    0 as libc::c_int as libc::c_char,
    '.' as i32 as libc::c_char,
    'o' as i32 as libc::c_char,
    'g' as i32 as libc::c_char,
    'x' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    '0' as i32 as libc::c_char,
    '1' as i32 as libc::c_char,
    ';' as i32 as libc::c_char,
    '3' as i32 as libc::c_char,
    '5' as i32 as libc::c_char,
    0 as libc::c_int as libc::c_char,
    '#' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    'a' as i32 as libc::c_char,
    'u' as i32 as libc::c_char,
    'd' as i32 as libc::c_char,
    'i' as i32 as libc::c_char,
    'o' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    'f' as i32 as libc::c_char,
    'o' as i32 as libc::c_char,
    'r' as i32 as libc::c_char,
    'm' as i32 as libc::c_char,
    'a' as i32 as libc::c_char,
    't' as i32 as libc::c_char,
    's' as i32 as libc::c_char,
    0 as libc::c_int as libc::c_char,
    '.' as i32 as libc::c_char,
    'a' as i32 as libc::c_char,
    'a' as i32 as libc::c_char,
    'c' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    '0' as i32 as libc::c_char,
    '0' as i32 as libc::c_char,
    ';' as i32 as libc::c_char,
    '3' as i32 as libc::c_char,
    '6' as i32 as libc::c_char,
    0 as libc::c_int as libc::c_char,
    '.' as i32 as libc::c_char,
    'a' as i32 as libc::c_char,
    'u' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    '0' as i32 as libc::c_char,
    '0' as i32 as libc::c_char,
    ';' as i32 as libc::c_char,
    '3' as i32 as libc::c_char,
    '6' as i32 as libc::c_char,
    0 as libc::c_int as libc::c_char,
    '.' as i32 as libc::c_char,
    'f' as i32 as libc::c_char,
    'l' as i32 as libc::c_char,
    'a' as i32 as libc::c_char,
    'c' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    '0' as i32 as libc::c_char,
    '0' as i32 as libc::c_char,
    ';' as i32 as libc::c_char,
    '3' as i32 as libc::c_char,
    '6' as i32 as libc::c_char,
    0 as libc::c_int as libc::c_char,
    '.' as i32 as libc::c_char,
    'm' as i32 as libc::c_char,
    '4' as i32 as libc::c_char,
    'a' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    '0' as i32 as libc::c_char,
    '0' as i32 as libc::c_char,
    ';' as i32 as libc::c_char,
    '3' as i32 as libc::c_char,
    '6' as i32 as libc::c_char,
    0 as libc::c_int as libc::c_char,
    '.' as i32 as libc::c_char,
    'm' as i32 as libc::c_char,
    'i' as i32 as libc::c_char,
    'd' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    '0' as i32 as libc::c_char,
    '0' as i32 as libc::c_char,
    ';' as i32 as libc::c_char,
    '3' as i32 as libc::c_char,
    '6' as i32 as libc::c_char,
    0 as libc::c_int as libc::c_char,
    '.' as i32 as libc::c_char,
    'm' as i32 as libc::c_char,
    'i' as i32 as libc::c_char,
    'd' as i32 as libc::c_char,
    'i' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    '0' as i32 as libc::c_char,
    '0' as i32 as libc::c_char,
    ';' as i32 as libc::c_char,
    '3' as i32 as libc::c_char,
    '6' as i32 as libc::c_char,
    0 as libc::c_int as libc::c_char,
    '.' as i32 as libc::c_char,
    'm' as i32 as libc::c_char,
    'k' as i32 as libc::c_char,
    'a' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    '0' as i32 as libc::c_char,
    '0' as i32 as libc::c_char,
    ';' as i32 as libc::c_char,
    '3' as i32 as libc::c_char,
    '6' as i32 as libc::c_char,
    0 as libc::c_int as libc::c_char,
    '.' as i32 as libc::c_char,
    'm' as i32 as libc::c_char,
    'p' as i32 as libc::c_char,
    '3' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    '0' as i32 as libc::c_char,
    '0' as i32 as libc::c_char,
    ';' as i32 as libc::c_char,
    '3' as i32 as libc::c_char,
    '6' as i32 as libc::c_char,
    0 as libc::c_int as libc::c_char,
    '.' as i32 as libc::c_char,
    'm' as i32 as libc::c_char,
    'p' as i32 as libc::c_char,
    'c' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    '0' as i32 as libc::c_char,
    '0' as i32 as libc::c_char,
    ';' as i32 as libc::c_char,
    '3' as i32 as libc::c_char,
    '6' as i32 as libc::c_char,
    0 as libc::c_int as libc::c_char,
    '.' as i32 as libc::c_char,
    'o' as i32 as libc::c_char,
    'g' as i32 as libc::c_char,
    'g' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    '0' as i32 as libc::c_char,
    '0' as i32 as libc::c_char,
    ';' as i32 as libc::c_char,
    '3' as i32 as libc::c_char,
    '6' as i32 as libc::c_char,
    0 as libc::c_int as libc::c_char,
    '.' as i32 as libc::c_char,
    'r' as i32 as libc::c_char,
    'a' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    '0' as i32 as libc::c_char,
    '0' as i32 as libc::c_char,
    ';' as i32 as libc::c_char,
    '3' as i32 as libc::c_char,
    '6' as i32 as libc::c_char,
    0 as libc::c_int as libc::c_char,
    '.' as i32 as libc::c_char,
    'w' as i32 as libc::c_char,
    'a' as i32 as libc::c_char,
    'v' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    '0' as i32 as libc::c_char,
    '0' as i32 as libc::c_char,
    ';' as i32 as libc::c_char,
    '3' as i32 as libc::c_char,
    '6' as i32 as libc::c_char,
    0 as libc::c_int as libc::c_char,
    '#' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    'h' as i32 as libc::c_char,
    't' as i32 as libc::c_char,
    't' as i32 as libc::c_char,
    'p' as i32 as libc::c_char,
    's' as i32 as libc::c_char,
    ':' as i32 as libc::c_char,
    '/' as i32 as libc::c_char,
    '/' as i32 as libc::c_char,
    'w' as i32 as libc::c_char,
    'i' as i32 as libc::c_char,
    'k' as i32 as libc::c_char,
    'i' as i32 as libc::c_char,
    '.' as i32 as libc::c_char,
    'x' as i32 as libc::c_char,
    'i' as i32 as libc::c_char,
    'p' as i32 as libc::c_char,
    'h' as i32 as libc::c_char,
    '.' as i32 as libc::c_char,
    'o' as i32 as libc::c_char,
    'r' as i32 as libc::c_char,
    'g' as i32 as libc::c_char,
    '/' as i32 as libc::c_char,
    'M' as i32 as libc::c_char,
    'I' as i32 as libc::c_char,
    'M' as i32 as libc::c_char,
    'E' as i32 as libc::c_char,
    '_' as i32 as libc::c_char,
    'T' as i32 as libc::c_char,
    'y' as i32 as libc::c_char,
    'p' as i32 as libc::c_char,
    'e' as i32 as libc::c_char,
    's' as i32 as libc::c_char,
    '_' as i32 as libc::c_char,
    'a' as i32 as libc::c_char,
    'n' as i32 as libc::c_char,
    'd' as i32 as libc::c_char,
    '_' as i32 as libc::c_char,
    'F' as i32 as libc::c_char,
    'i' as i32 as libc::c_char,
    'l' as i32 as libc::c_char,
    'e' as i32 as libc::c_char,
    '_' as i32 as libc::c_char,
    'E' as i32 as libc::c_char,
    'x' as i32 as libc::c_char,
    't' as i32 as libc::c_char,
    'e' as i32 as libc::c_char,
    'n' as i32 as libc::c_char,
    's' as i32 as libc::c_char,
    'i' as i32 as libc::c_char,
    'o' as i32 as libc::c_char,
    'n' as i32 as libc::c_char,
    's' as i32 as libc::c_char,
    0 as libc::c_int as libc::c_char,
    '.' as i32 as libc::c_char,
    'o' as i32 as libc::c_char,
    'g' as i32 as libc::c_char,
    'a' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    '0' as i32 as libc::c_char,
    '0' as i32 as libc::c_char,
    ';' as i32 as libc::c_char,
    '3' as i32 as libc::c_char,
    '6' as i32 as libc::c_char,
    0 as libc::c_int as libc::c_char,
    '.' as i32 as libc::c_char,
    'o' as i32 as libc::c_char,
    'p' as i32 as libc::c_char,
    'u' as i32 as libc::c_char,
    's' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    '0' as i32 as libc::c_char,
    '0' as i32 as libc::c_char,
    ';' as i32 as libc::c_char,
    '3' as i32 as libc::c_char,
    '6' as i32 as libc::c_char,
    0 as libc::c_int as libc::c_char,
    '.' as i32 as libc::c_char,
    's' as i32 as libc::c_char,
    'p' as i32 as libc::c_char,
    'x' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    '0' as i32 as libc::c_char,
    '0' as i32 as libc::c_char,
    ';' as i32 as libc::c_char,
    '3' as i32 as libc::c_char,
    '6' as i32 as libc::c_char,
    0 as libc::c_int as libc::c_char,
    '.' as i32 as libc::c_char,
    'x' as i32 as libc::c_char,
    's' as i32 as libc::c_char,
    'p' as i32 as libc::c_char,
    'f' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    '0' as i32 as libc::c_char,
    '0' as i32 as libc::c_char,
    ';' as i32 as libc::c_char,
    '3' as i32 as libc::c_char,
    '6' as i32 as libc::c_char,
    0 as libc::c_int as libc::c_char,
    '#' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    'b' as i32 as libc::c_char,
    'a' as i32 as libc::c_char,
    'c' as i32 as libc::c_char,
    'k' as i32 as libc::c_char,
    'u' as i32 as libc::c_char,
    'p' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    'f' as i32 as libc::c_char,
    'i' as i32 as libc::c_char,
    'l' as i32 as libc::c_char,
    'e' as i32 as libc::c_char,
    's' as i32 as libc::c_char,
    0 as libc::c_int as libc::c_char,
    '*' as i32 as libc::c_char,
    '~' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    '0' as i32 as libc::c_char,
    '0' as i32 as libc::c_char,
    ';' as i32 as libc::c_char,
    '9' as i32 as libc::c_char,
    '0' as i32 as libc::c_char,
    0 as libc::c_int as libc::c_char,
    '*' as i32 as libc::c_char,
    '#' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    '0' as i32 as libc::c_char,
    '0' as i32 as libc::c_char,
    ';' as i32 as libc::c_char,
    '9' as i32 as libc::c_char,
    '0' as i32 as libc::c_char,
    0 as libc::c_int as libc::c_char,
    '.' as i32 as libc::c_char,
    'b' as i32 as libc::c_char,
    'a' as i32 as libc::c_char,
    'k' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    '0' as i32 as libc::c_char,
    '0' as i32 as libc::c_char,
    ';' as i32 as libc::c_char,
    '9' as i32 as libc::c_char,
    '0' as i32 as libc::c_char,
    0 as libc::c_int as libc::c_char,
    '.' as i32 as libc::c_char,
    'o' as i32 as libc::c_char,
    'l' as i32 as libc::c_char,
    'd' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    '0' as i32 as libc::c_char,
    '0' as i32 as libc::c_char,
    ';' as i32 as libc::c_char,
    '9' as i32 as libc::c_char,
    '0' as i32 as libc::c_char,
    0 as libc::c_int as libc::c_char,
    '.' as i32 as libc::c_char,
    'o' as i32 as libc::c_char,
    'r' as i32 as libc::c_char,
    'i' as i32 as libc::c_char,
    'g' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    '0' as i32 as libc::c_char,
    '0' as i32 as libc::c_char,
    ';' as i32 as libc::c_char,
    '9' as i32 as libc::c_char,
    '0' as i32 as libc::c_char,
    0 as libc::c_int as libc::c_char,
    '.' as i32 as libc::c_char,
    'p' as i32 as libc::c_char,
    'a' as i32 as libc::c_char,
    'r' as i32 as libc::c_char,
    't' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    '0' as i32 as libc::c_char,
    '0' as i32 as libc::c_char,
    ';' as i32 as libc::c_char,
    '9' as i32 as libc::c_char,
    '0' as i32 as libc::c_char,
    0 as libc::c_int as libc::c_char,
    '.' as i32 as libc::c_char,
    'r' as i32 as libc::c_char,
    'e' as i32 as libc::c_char,
    'j' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    '0' as i32 as libc::c_char,
    '0' as i32 as libc::c_char,
    ';' as i32 as libc::c_char,
    '9' as i32 as libc::c_char,
    '0' as i32 as libc::c_char,
    0 as libc::c_int as libc::c_char,
    '.' as i32 as libc::c_char,
    's' as i32 as libc::c_char,
    'w' as i32 as libc::c_char,
    'p' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    '0' as i32 as libc::c_char,
    '0' as i32 as libc::c_char,
    ';' as i32 as libc::c_char,
    '9' as i32 as libc::c_char,
    '0' as i32 as libc::c_char,
    0 as libc::c_int as libc::c_char,
    '.' as i32 as libc::c_char,
    't' as i32 as libc::c_char,
    'm' as i32 as libc::c_char,
    'p' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    '0' as i32 as libc::c_char,
    '0' as i32 as libc::c_char,
    ';' as i32 as libc::c_char,
    '9' as i32 as libc::c_char,
    '0' as i32 as libc::c_char,
    0 as libc::c_int as libc::c_char,
    '.' as i32 as libc::c_char,
    'd' as i32 as libc::c_char,
    'p' as i32 as libc::c_char,
    'k' as i32 as libc::c_char,
    'g' as i32 as libc::c_char,
    '-' as i32 as libc::c_char,
    'd' as i32 as libc::c_char,
    'i' as i32 as libc::c_char,
    's' as i32 as libc::c_char,
    't' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    '0' as i32 as libc::c_char,
    '0' as i32 as libc::c_char,
    ';' as i32 as libc::c_char,
    '9' as i32 as libc::c_char,
    '0' as i32 as libc::c_char,
    0 as libc::c_int as libc::c_char,
    '.' as i32 as libc::c_char,
    'd' as i32 as libc::c_char,
    'p' as i32 as libc::c_char,
    'k' as i32 as libc::c_char,
    'g' as i32 as libc::c_char,
    '-' as i32 as libc::c_char,
    'o' as i32 as libc::c_char,
    'l' as i32 as libc::c_char,
    'd' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    '0' as i32 as libc::c_char,
    '0' as i32 as libc::c_char,
    ';' as i32 as libc::c_char,
    '9' as i32 as libc::c_char,
    '0' as i32 as libc::c_char,
    0 as libc::c_int as libc::c_char,
    '.' as i32 as libc::c_char,
    'u' as i32 as libc::c_char,
    'c' as i32 as libc::c_char,
    'f' as i32 as libc::c_char,
    '-' as i32 as libc::c_char,
    'd' as i32 as libc::c_char,
    'i' as i32 as libc::c_char,
    's' as i32 as libc::c_char,
    't' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    '0' as i32 as libc::c_char,
    '0' as i32 as libc::c_char,
    ';' as i32 as libc::c_char,
    '9' as i32 as libc::c_char,
    '0' as i32 as libc::c_char,
    0 as libc::c_int as libc::c_char,
    '.' as i32 as libc::c_char,
    'u' as i32 as libc::c_char,
    'c' as i32 as libc::c_char,
    'f' as i32 as libc::c_char,
    '-' as i32 as libc::c_char,
    'n' as i32 as libc::c_char,
    'e' as i32 as libc::c_char,
    'w' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    '0' as i32 as libc::c_char,
    '0' as i32 as libc::c_char,
    ';' as i32 as libc::c_char,
    '9' as i32 as libc::c_char,
    '0' as i32 as libc::c_char,
    0 as libc::c_int as libc::c_char,
    '.' as i32 as libc::c_char,
    'u' as i32 as libc::c_char,
    'c' as i32 as libc::c_char,
    'f' as i32 as libc::c_char,
    '-' as i32 as libc::c_char,
    'o' as i32 as libc::c_char,
    'l' as i32 as libc::c_char,
    'd' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    '0' as i32 as libc::c_char,
    '0' as i32 as libc::c_char,
    ';' as i32 as libc::c_char,
    '9' as i32 as libc::c_char,
    '0' as i32 as libc::c_char,
    0 as libc::c_int as libc::c_char,
    '.' as i32 as libc::c_char,
    'r' as i32 as libc::c_char,
    'p' as i32 as libc::c_char,
    'm' as i32 as libc::c_char,
    'n' as i32 as libc::c_char,
    'e' as i32 as libc::c_char,
    'w' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    '0' as i32 as libc::c_char,
    '0' as i32 as libc::c_char,
    ';' as i32 as libc::c_char,
    '9' as i32 as libc::c_char,
    '0' as i32 as libc::c_char,
    0 as libc::c_int as libc::c_char,
    '.' as i32 as libc::c_char,
    'r' as i32 as libc::c_char,
    'p' as i32 as libc::c_char,
    'm' as i32 as libc::c_char,
    'o' as i32 as libc::c_char,
    'r' as i32 as libc::c_char,
    'i' as i32 as libc::c_char,
    'g' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    '0' as i32 as libc::c_char,
    '0' as i32 as libc::c_char,
    ';' as i32 as libc::c_char,
    '9' as i32 as libc::c_char,
    '0' as i32 as libc::c_char,
    0 as libc::c_int as libc::c_char,
    '.' as i32 as libc::c_char,
    'r' as i32 as libc::c_char,
    'p' as i32 as libc::c_char,
    'm' as i32 as libc::c_char,
    's' as i32 as libc::c_char,
    'a' as i32 as libc::c_char,
    'v' as i32 as libc::c_char,
    'e' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    '0' as i32 as libc::c_char,
    '0' as i32 as libc::c_char,
    ';' as i32 as libc::c_char,
    '9' as i32 as libc::c_char,
    '0' as i32 as libc::c_char,
    0 as libc::c_int as libc::c_char,
    '#' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    'S' as i32 as libc::c_char,
    'u' as i32 as libc::c_char,
    'b' as i32 as libc::c_char,
    's' as i32 as libc::c_char,
    'e' as i32 as libc::c_char,
    'q' as i32 as libc::c_char,
    'u' as i32 as libc::c_char,
    'e' as i32 as libc::c_char,
    'n' as i32 as libc::c_char,
    't' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    'T' as i32 as libc::c_char,
    'E' as i32 as libc::c_char,
    'R' as i32 as libc::c_char,
    'M' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    'o' as i32 as libc::c_char,
    'r' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    'C' as i32 as libc::c_char,
    'O' as i32 as libc::c_char,
    'L' as i32 as libc::c_char,
    'O' as i32 as libc::c_char,
    'R' as i32 as libc::c_char,
    'T' as i32 as libc::c_char,
    'E' as i32 as libc::c_char,
    'R' as i32 as libc::c_char,
    'M' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    'e' as i32 as libc::c_char,
    'n' as i32 as libc::c_char,
    't' as i32 as libc::c_char,
    'r' as i32 as libc::c_char,
    'i' as i32 as libc::c_char,
    'e' as i32 as libc::c_char,
    's' as i32 as libc::c_char,
    ',' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    'c' as i32 as libc::c_char,
    'a' as i32 as libc::c_char,
    'n' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    'b' as i32 as libc::c_char,
    'e' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    'u' as i32 as libc::c_char,
    's' as i32 as libc::c_char,
    'e' as i32 as libc::c_char,
    'd' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    't' as i32 as libc::c_char,
    'o' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    'a' as i32 as libc::c_char,
    'd' as i32 as libc::c_char,
    'd' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    '/' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    'o' as i32 as libc::c_char,
    'v' as i32 as libc::c_char,
    'e' as i32 as libc::c_char,
    'r' as i32 as libc::c_char,
    'r' as i32 as libc::c_char,
    'i' as i32 as libc::c_char,
    'd' as i32 as libc::c_char,
    'e' as i32 as libc::c_char,
    0 as libc::c_int as libc::c_char,
    '#' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    'c' as i32 as libc::c_char,
    'o' as i32 as libc::c_char,
    'n' as i32 as libc::c_char,
    'f' as i32 as libc::c_char,
    'i' as i32 as libc::c_char,
    'g' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    's' as i32 as libc::c_char,
    'p' as i32 as libc::c_char,
    'e' as i32 as libc::c_char,
    'c' as i32 as libc::c_char,
    'i' as i32 as libc::c_char,
    'f' as i32 as libc::c_char,
    'i' as i32 as libc::c_char,
    'c' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    't' as i32 as libc::c_char,
    'o' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    't' as i32 as libc::c_char,
    'h' as i32 as libc::c_char,
    'o' as i32 as libc::c_char,
    's' as i32 as libc::c_char,
    'e' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    'm' as i32 as libc::c_char,
    'a' as i32 as libc::c_char,
    't' as i32 as libc::c_char,
    'c' as i32 as libc::c_char,
    'h' as i32 as libc::c_char,
    'i' as i32 as libc::c_char,
    'n' as i32 as libc::c_char,
    'g' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    'e' as i32 as libc::c_char,
    'n' as i32 as libc::c_char,
    'v' as i32 as libc::c_char,
    'i' as i32 as libc::c_char,
    'r' as i32 as libc::c_char,
    'o' as i32 as libc::c_char,
    'n' as i32 as libc::c_char,
    'm' as i32 as libc::c_char,
    'e' as i32 as libc::c_char,
    'n' as i32 as libc::c_char,
    't' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    'v' as i32 as libc::c_char,
    'a' as i32 as libc::c_char,
    'r' as i32 as libc::c_char,
    'i' as i32 as libc::c_char,
    'a' as i32 as libc::c_char,
    'b' as i32 as libc::c_char,
    'l' as i32 as libc::c_char,
    'e' as i32 as libc::c_char,
    's' as i32 as libc::c_char,
    '.' as i32 as libc::c_char,
    0 as libc::c_int as libc::c_char,
];
#[inline]
unsafe extern "C" fn c_isalnum(mut c: libc::c_int) -> bool {
    match c {
        48 | 49 | 50 | 51 | 52 | 53 | 54 | 55 | 56 | 57 | 97 | 98 | 99 | 100 | 101 | 102
        | 103 | 104 | 105 | 106 | 107 | 108 | 109 | 110 | 111 | 112 | 113 | 114 | 115
        | 116 | 117 | 118 | 119 | 120 | 121 | 122 | 65 | 66 | 67 | 68 | 69 | 70 | 71 | 72
        | 73 | 74 | 75 | 76 | 77 | 78 | 79 | 80 | 81 | 82 | 83 | 84 | 85 | 86 | 87 | 88
        | 89 | 90 => return 1 as libc::c_int != 0,
        _ => return 0 as libc::c_int != 0,
    };
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
static mut filetype_letter: [libc::c_char; 11] = unsafe {
    *::core::mem::transmute::<&[u8; 11], &[libc::c_char; 11]>(b"?pcdb-lswd\0")
};
static mut active_dir_set: *mut Hash_table = 0 as *const Hash_table as *mut Hash_table;
static mut cwd_file: *mut fileinfo = 0 as *const fileinfo as *mut fileinfo;
static mut cwd_n_alloc: size_t = 0;
static mut cwd_n_used: size_t = 0;
static mut cwd_some_quoted: bool = false;
static mut align_variable_outer_quotes: bool = false;
static mut sorted_file: *mut *mut libc::c_void = 0 as *const *mut libc::c_void
    as *mut *mut libc::c_void;
static mut sorted_file_alloc: size_t = 0;
static mut color_symlink_as_referent: bool = false;
static mut hostname: *const libc::c_char = 0 as *const libc::c_char;
unsafe extern "C" fn file_or_link_mode(mut file: *const fileinfo) -> mode_t {
    return if color_symlink_as_referent as libc::c_int != 0
        && (*file).linkok as libc::c_int != 0
    {
        (*file).linkmode
    } else {
        (*file).stat.st_mode
    };
}
static mut pending_dirs: *mut pending = 0 as *const pending as *mut pending;
static mut current_time: timespec = timespec { tv_sec: 0, tv_nsec: 0 };
static mut print_scontext: bool = false;
static mut UNKNOWN_SECURITY_CONTEXT: [libc::c_char; 2] = unsafe {
    *::core::mem::transmute::<&[u8; 2], &mut [libc::c_char; 2]>(b"?\0")
};
static mut any_has_acl: bool = false;
static mut inode_number_width: libc::c_int = 0;
static mut block_size_width: libc::c_int = 0;
static mut nlink_width: libc::c_int = 0;
static mut scontext_width: libc::c_int = 0;
static mut owner_width: libc::c_int = 0;
static mut group_width: libc::c_int = 0;
static mut author_width: libc::c_int = 0;
static mut major_device_number_width: libc::c_int = 0;
static mut minor_device_number_width: libc::c_int = 0;
static mut file_size_width: libc::c_int = 0;
static mut format: format = long_format;
static mut time_style_args: [*const libc::c_char; 5] = [
    b"full-iso\0" as *const u8 as *const libc::c_char,
    b"long-iso\0" as *const u8 as *const libc::c_char,
    b"iso\0" as *const u8 as *const libc::c_char,
    b"locale\0" as *const u8 as *const libc::c_char,
    0 as *const libc::c_char,
];
static mut time_style_types: [time_style; 4] = [
    full_iso_time_style,
    long_iso_time_style,
    iso_time_style,
    locale_time_style,
];
static mut time_type: time_type = time_mtime;
static mut sort_type: sort_type = sort_name;
static mut sort_reverse: bool = false;
static mut print_owner: bool = 1 as libc::c_int != 0;
static mut print_author: bool = false;
static mut print_group: bool = 1 as libc::c_int != 0;
static mut numeric_ids: bool = false;
static mut print_block_size: bool = false;
static mut human_output_opts: libc::c_int = 0;
static mut output_block_size: uintmax_t = 0;
static mut file_human_output_opts: libc::c_int = 0;
static mut file_output_block_size: uintmax_t = 1 as libc::c_int as uintmax_t;
static mut dired: bool = false;
static mut indicator_style: indicator_style = none;
static mut indicator_style_args: [*const libc::c_char; 5] = [
    b"none\0" as *const u8 as *const libc::c_char,
    b"slash\0" as *const u8 as *const libc::c_char,
    b"file-type\0" as *const u8 as *const libc::c_char,
    b"classify\0" as *const u8 as *const libc::c_char,
    0 as *const libc::c_char,
];
static mut indicator_style_types: [indicator_style; 4] = [
    none,
    slash,
    file_type,
    classify,
];
static mut print_with_color: bool = false;
static mut print_hyperlink: bool = false;
static mut used_color: bool = 0 as libc::c_int != 0;
static mut indicator_name: [*const libc::c_char; 25] = [
    b"lc\0" as *const u8 as *const libc::c_char,
    b"rc\0" as *const u8 as *const libc::c_char,
    b"ec\0" as *const u8 as *const libc::c_char,
    b"rs\0" as *const u8 as *const libc::c_char,
    b"no\0" as *const u8 as *const libc::c_char,
    b"fi\0" as *const u8 as *const libc::c_char,
    b"di\0" as *const u8 as *const libc::c_char,
    b"ln\0" as *const u8 as *const libc::c_char,
    b"pi\0" as *const u8 as *const libc::c_char,
    b"so\0" as *const u8 as *const libc::c_char,
    b"bd\0" as *const u8 as *const libc::c_char,
    b"cd\0" as *const u8 as *const libc::c_char,
    b"mi\0" as *const u8 as *const libc::c_char,
    b"or\0" as *const u8 as *const libc::c_char,
    b"ex\0" as *const u8 as *const libc::c_char,
    b"do\0" as *const u8 as *const libc::c_char,
    b"su\0" as *const u8 as *const libc::c_char,
    b"sg\0" as *const u8 as *const libc::c_char,
    b"st\0" as *const u8 as *const libc::c_char,
    b"ow\0" as *const u8 as *const libc::c_char,
    b"tw\0" as *const u8 as *const libc::c_char,
    b"ca\0" as *const u8 as *const libc::c_char,
    b"mh\0" as *const u8 as *const libc::c_char,
    b"cl\0" as *const u8 as *const libc::c_char,
    0 as *const libc::c_char,
];
static mut color_indicator: [bin_str; 24] = [bin_str {
    len: 0,
    string: 0 as *const libc::c_char,
}; 24];
static mut color_ext_list: *mut color_ext_type = 0 as *const color_ext_type
    as *mut color_ext_type;
static mut color_buf: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
static mut check_symlink_mode: bool = false;
static mut print_inode: bool = false;
static mut dereference: Dereference_symlink = DEREF_UNDEFINED;
static mut recursive: bool = false;
static mut immediate_dirs: bool = false;
static mut directories_first: bool = false;
static mut ignore_mode: C2RustUnnamed_23 = IGNORE_DEFAULT;
static mut ignore_patterns: *mut ignore_pattern = 0 as *const ignore_pattern
    as *mut ignore_pattern;
static mut hide_patterns: *mut ignore_pattern = 0 as *const ignore_pattern
    as *mut ignore_pattern;
static mut qmark_funny_chars: bool = false;
static mut filename_quoting_options: *mut quoting_options = 0 as *const quoting_options
    as *mut quoting_options;
static mut dirname_quoting_options: *mut quoting_options = 0 as *const quoting_options
    as *mut quoting_options;
static mut tabsize: size_t = 0;
static mut print_dir_name: bool = false;
static mut line_length: size_t = 0;
static mut localtz: timezone_t = 0 as *const tm_zone as *mut tm_zone;
static mut format_needs_stat: bool = false;
static mut format_needs_type: bool = false;
static mut long_time_format: [*const libc::c_char; 2] = [
    b"%b %e  %Y\0" as *const u8 as *const libc::c_char,
    b"%b %e %H:%M\0" as *const u8 as *const libc::c_char,
];
static mut caught_signals: sigset_t = sigset_t { __val: [0; 16] };
static mut interrupt_signal: sig_atomic_t = 0;
static mut stop_signal_count: sig_atomic_t = 0;
static mut exit_status: libc::c_int = 0;
static mut long_options: [option; 45] = [
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
            name: b"escape\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
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
            val: 'd' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"dired\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'D' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"full-time\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: FULL_TIME_OPTION as libc::c_int,
        };
        init
    },
    {
        let mut init = option {
            name: b"group-directories-first\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: GROUP_DIRECTORIES_FIRST_OPTION as libc::c_int,
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
            name: b"inode\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'i' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"kibibytes\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'k' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"numeric-uid-gid\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'n' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"no-group\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'G' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"hide-control-chars\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'q' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"reverse\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'r' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"size\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 's' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"width\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'w' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"almost-all\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'A' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"ignore-backups\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'B' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"classify\0" as *const u8 as *const libc::c_char,
            has_arg: 2 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'F' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"file-type\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: FILE_TYPE_INDICATOR_OPTION as libc::c_int,
        };
        init
    },
    {
        let mut init = option {
            name: b"si\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: SI_OPTION as libc::c_int,
        };
        init
    },
    {
        let mut init = option {
            name: b"dereference-command-line\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'H' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"dereference-command-line-symlink-to-dir\0" as *const u8
                as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: DEREFERENCE_COMMAND_LINE_SYMLINK_TO_DIR_OPTION as libc::c_int,
        };
        init
    },
    {
        let mut init = option {
            name: b"hide\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: HIDE_OPTION as libc::c_int,
        };
        init
    },
    {
        let mut init = option {
            name: b"ignore\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'I' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"indicator-style\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: INDICATOR_STYLE_OPTION as libc::c_int,
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
            name: b"literal\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'N' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"quote-name\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'Q' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"quoting-style\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: QUOTING_STYLE_OPTION as libc::c_int,
        };
        init
    },
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
            name: b"format\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: FORMAT_OPTION as libc::c_int,
        };
        init
    },
    {
        let mut init = option {
            name: b"show-control-chars\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: SHOW_CONTROL_CHARS_OPTION as libc::c_int,
        };
        init
    },
    {
        let mut init = option {
            name: b"sort\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: SORT_OPTION as libc::c_int,
        };
        init
    },
    {
        let mut init = option {
            name: b"tabsize\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'T' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"time\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
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
            name: b"zero\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: ZERO_OPTION as libc::c_int,
        };
        init
    },
    {
        let mut init = option {
            name: b"color\0" as *const u8 as *const libc::c_char,
            has_arg: 2 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: COLOR_OPTION as libc::c_int,
        };
        init
    },
    {
        let mut init = option {
            name: b"hyperlink\0" as *const u8 as *const libc::c_char,
            has_arg: 2 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: HYPERLINK_OPTION as libc::c_int,
        };
        init
    },
    {
        let mut init = option {
            name: b"block-size\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: BLOCK_SIZE_OPTION as libc::c_int,
        };
        init
    },
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
            name: b"author\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: AUTHOR_OPTION as libc::c_int,
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
static mut format_args: [*const libc::c_char; 8] = [
    b"verbose\0" as *const u8 as *const libc::c_char,
    b"long\0" as *const u8 as *const libc::c_char,
    b"commas\0" as *const u8 as *const libc::c_char,
    b"horizontal\0" as *const u8 as *const libc::c_char,
    b"across\0" as *const u8 as *const libc::c_char,
    b"vertical\0" as *const u8 as *const libc::c_char,
    b"single-column\0" as *const u8 as *const libc::c_char,
    0 as *const libc::c_char,
];
static mut format_types: [format; 7] = [
    long_format,
    long_format,
    with_commas,
    horizontal,
    horizontal,
    many_per_line,
    one_per_line,
];
static mut sort_args: [*const libc::c_char; 7] = [
    b"none\0" as *const u8 as *const libc::c_char,
    b"time\0" as *const u8 as *const libc::c_char,
    b"size\0" as *const u8 as *const libc::c_char,
    b"extension\0" as *const u8 as *const libc::c_char,
    b"version\0" as *const u8 as *const libc::c_char,
    b"width\0" as *const u8 as *const libc::c_char,
    0 as *const libc::c_char,
];
static mut sort_types: [sort_type; 6] = [
    sort_none,
    sort_time,
    sort_size,
    sort_extension,
    sort_version,
    sort_width,
];
static mut time_args: [*const libc::c_char; 10] = [
    b"atime\0" as *const u8 as *const libc::c_char,
    b"access\0" as *const u8 as *const libc::c_char,
    b"use\0" as *const u8 as *const libc::c_char,
    b"ctime\0" as *const u8 as *const libc::c_char,
    b"status\0" as *const u8 as *const libc::c_char,
    b"mtime\0" as *const u8 as *const libc::c_char,
    b"modification\0" as *const u8 as *const libc::c_char,
    b"birth\0" as *const u8 as *const libc::c_char,
    b"creation\0" as *const u8 as *const libc::c_char,
    0 as *const libc::c_char,
];
static mut time_types: [time_type; 9] = [
    time_atime,
    time_atime,
    time_atime,
    time_ctime,
    time_ctime,
    time_mtime,
    time_mtime,
    time_btime,
    time_btime,
];
static mut when_args: [*const libc::c_char; 10] = [
    b"always\0" as *const u8 as *const libc::c_char,
    b"yes\0" as *const u8 as *const libc::c_char,
    b"force\0" as *const u8 as *const libc::c_char,
    b"never\0" as *const u8 as *const libc::c_char,
    b"no\0" as *const u8 as *const libc::c_char,
    b"none\0" as *const u8 as *const libc::c_char,
    b"auto\0" as *const u8 as *const libc::c_char,
    b"tty\0" as *const u8 as *const libc::c_char,
    b"if-tty\0" as *const u8 as *const libc::c_char,
    0 as *const libc::c_char,
];
static mut when_types: [when_type; 9] = [
    when_always,
    when_always,
    when_always,
    when_never,
    when_never,
    when_never,
    when_if_tty,
    when_if_tty,
    when_if_tty,
];
static mut column_info: *mut column_info = 0 as *const column_info as *mut column_info;
static mut max_idx: size_t = 0;
static mut dired_pos: off_t = 0;
unsafe extern "C" fn dired_outbyte(mut c: libc::c_char) {
    dired_pos += 1;
    putchar_unlocked(c as libc::c_int);
}
unsafe extern "C" fn dired_outbuf(mut s: *const libc::c_char, mut s_len: size_t) {
    dired_pos = (dired_pos as libc::c_ulong).wrapping_add(s_len) as off_t as off_t;
    if 0 != 0 && 0 != 0
        && (::core::mem::size_of::<libc::c_char>() as libc::c_ulong).wrapping_mul(s_len)
            <= 8 as libc::c_int as libc::c_ulong
        && ::core::mem::size_of::<libc::c_char>() as libc::c_ulong
            != 0 as libc::c_int as libc::c_ulong
    {
        ({
            let mut __ptr: *const libc::c_char = s;
            let mut __stream: *mut FILE = stdout;
            let mut __cnt: size_t = 0;
            __cnt = (::core::mem::size_of::<libc::c_char>() as libc::c_ulong)
                .wrapping_mul(s_len);
            while __cnt > 0 as libc::c_int as libc::c_ulong {
                let fresh2 = __ptr;
                __ptr = __ptr.offset(1);
                if putc_unlocked(*fresh2 as libc::c_int, __stream) == -(1 as libc::c_int)
                {
                    break;
                }
                __cnt = __cnt.wrapping_sub(1);
            }
            compile_error!("Binary expression is not supposed to be used")
        });
    } else {
        if 0 != 0
            && ::core::mem::size_of::<libc::c_char>() as libc::c_ulong
                == 0 as libc::c_int as libc::c_ulong
            || 0 != 0 && s_len == 0 as libc::c_int as libc::c_ulong
        {} else {
            fwrite_unlocked(
                s as *const libc::c_void,
                ::core::mem::size_of::<libc::c_char>() as libc::c_ulong,
                s_len,
                stdout,
            );
        };
    };
}
unsafe extern "C" fn dired_outstring(mut s: *const libc::c_char) {
    dired_outbuf(s, strlen(s));
}
unsafe extern "C" fn dired_indent() {
    if dired {
        dired_outstring(b"  \0" as *const u8 as *const libc::c_char);
    }
}
static mut dired_obstack: obstack = obstack {
    chunk_size: 0,
    chunk: 0 as *const _obstack_chunk as *mut _obstack_chunk,
    object_base: 0 as *const libc::c_char as *mut libc::c_char,
    next_free: 0 as *const libc::c_char as *mut libc::c_char,
    chunk_limit: 0 as *const libc::c_char as *mut libc::c_char,
    temp: C2RustUnnamed_2 { i: 0 },
    alignment_mask: 0,
    chunkfun: C2RustUnnamed_1 { plain: None },
    freefun: C2RustUnnamed_0 { plain: None },
    extra_arg: 0 as *const libc::c_void as *mut libc::c_void,
    use_extra_arg_maybe_empty_object_alloc_failed: [0; 1],
    c2rust_padding: [0; 7],
};
static mut subdired_obstack: obstack = obstack {
    chunk_size: 0,
    chunk: 0 as *const _obstack_chunk as *mut _obstack_chunk,
    object_base: 0 as *const libc::c_char as *mut libc::c_char,
    next_free: 0 as *const libc::c_char as *mut libc::c_char,
    chunk_limit: 0 as *const libc::c_char as *mut libc::c_char,
    temp: C2RustUnnamed_2 { i: 0 },
    alignment_mask: 0,
    chunkfun: C2RustUnnamed_1 { plain: None },
    freefun: C2RustUnnamed_0 { plain: None },
    extra_arg: 0 as *const libc::c_void as *mut libc::c_void,
    use_extra_arg_maybe_empty_object_alloc_failed: [0; 1],
    c2rust_padding: [0; 7],
};
unsafe extern "C" fn push_current_dired_pos(mut obs: *mut obstack) {
    if dired {
        let mut __o: *mut obstack = obs;
        let mut __len: size_t = ::core::mem::size_of::<off_t>() as libc::c_ulong;
        if ({
            let mut __o1: *const obstack = __o;
            ((*__o1).chunk_limit).offset_from((*__o1).next_free) as libc::c_long
                as size_t
        }) < __len
        {
            _obstack_newchunk(__o, __len);
        }
        memcpy(
            (*__o).next_free as *mut libc::c_void,
            &mut dired_pos as *mut off_t as *const libc::c_void,
            __len,
        );
        (*__o).next_free = ((*__o).next_free).offset(__len as isize);
    }
}
static mut dev_ino_obstack: obstack = obstack {
    chunk_size: 0,
    chunk: 0 as *const _obstack_chunk as *mut _obstack_chunk,
    object_base: 0 as *const libc::c_char as *mut libc::c_char,
    next_free: 0 as *const libc::c_char as *mut libc::c_char,
    chunk_limit: 0 as *const libc::c_char as *mut libc::c_char,
    temp: C2RustUnnamed_2 { i: 0 },
    alignment_mask: 0,
    chunkfun: C2RustUnnamed_1 { plain: None },
    freefun: C2RustUnnamed_0 { plain: None },
    extra_arg: 0 as *const libc::c_void as *mut libc::c_void,
    use_extra_arg_maybe_empty_object_alloc_failed: [0; 1],
    c2rust_padding: [0; 7],
};
unsafe extern "C" fn dev_ino_push(mut dev: dev_t, mut ino: ino_t) {
    let mut vdi: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut di: *mut dev_ino = 0 as *mut dev_ino;
    let mut dev_ino_size: libc::c_int = ::core::mem::size_of::<dev_ino>()
        as libc::c_ulong as libc::c_int;
    let mut __o: *mut obstack = &mut dev_ino_obstack;
    let mut __len: size_t = dev_ino_size as size_t;
    if ({
        let mut __o1: *const obstack = __o;
        ((*__o1).chunk_limit).offset_from((*__o1).next_free) as libc::c_long as size_t
    }) < __len
    {
        _obstack_newchunk(__o, __len);
    }
    (*__o).next_free = ((*__o).next_free).offset(__len as isize);
    vdi = dev_ino_obstack.next_free as *mut libc::c_void;
    di = vdi as *mut dev_ino;
    di = di.offset(-1);
    (*di).st_dev = dev;
    (*di).st_ino = ino;
}
unsafe extern "C" fn dev_ino_pop() -> dev_ino {
    let mut vdi: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut di: *mut dev_ino = 0 as *mut dev_ino;
    let mut dev_ino_size: libc::c_int = ::core::mem::size_of::<dev_ino>()
        as libc::c_ulong as libc::c_int;
    if dev_ino_size as libc::c_ulong
        <= ({
            let mut __o: *const obstack = &mut dev_ino_obstack as *mut obstack;
            ((*__o).next_free).offset_from((*__o).object_base) as libc::c_long as size_t
        })
    {} else {
        __assert_fail(
            b"dev_ino_size <= obstack_object_size (&dev_ino_obstack)\0" as *const u8
                as *const libc::c_char,
            b"src/ls.c\0" as *const u8 as *const libc::c_char,
            1055 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 33],
                &[libc::c_char; 33],
            >(b"struct dev_ino dev_ino_pop(void)\0"))
                .as_ptr(),
        );
    }
    dev_ino_obstack
        .next_free = (dev_ino_obstack.next_free).offset(-dev_ino_size as isize);
    vdi = dev_ino_obstack.next_free as *mut libc::c_void;
    di = vdi as *mut dev_ino;
    return *di;
}
unsafe extern "C" fn assert_matching_dev_ino(
    mut name: *const libc::c_char,
    mut di: dev_ino,
) {
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
    if !name.is_null() {} else {
        __assert_fail(
            b"name\0" as *const u8 as *const libc::c_char,
            b"src/ls.c\0" as *const u8 as *const libc::c_char,
            1066 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 59],
                &[libc::c_char; 59],
            >(b"void assert_matching_dev_ino(const char *, struct dev_ino)\0"))
                .as_ptr(),
        );
    }
    if 0 as libc::c_int <= stat(name, &mut sb) {} else {
        __assert_fail(
            b"0 <= stat (name, &sb)\0" as *const u8 as *const libc::c_char,
            b"src/ls.c\0" as *const u8 as *const libc::c_char,
            1067 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 59],
                &[libc::c_char; 59],
            >(b"void assert_matching_dev_ino(const char *, struct dev_ino)\0"))
                .as_ptr(),
        );
    }
    if sb.st_dev == di.st_dev {} else {
        __assert_fail(
            b"sb.st_dev == di.st_dev\0" as *const u8 as *const libc::c_char,
            b"src/ls.c\0" as *const u8 as *const libc::c_char,
            1068 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 59],
                &[libc::c_char; 59],
            >(b"void assert_matching_dev_ino(const char *, struct dev_ino)\0"))
                .as_ptr(),
        );
    }
    if sb.st_ino == di.st_ino {} else {
        __assert_fail(
            b"sb.st_ino == di.st_ino\0" as *const u8 as *const libc::c_char,
            b"src/ls.c\0" as *const u8 as *const libc::c_char,
            1069 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 59],
                &[libc::c_char; 59],
            >(b"void assert_matching_dev_ino(const char *, struct dev_ino)\0"))
                .as_ptr(),
        );
    };
}
static mut eolbyte: libc::c_char = '\n' as i32 as libc::c_char;
unsafe extern "C" fn dired_dump_obstack(
    mut prefix: *const libc::c_char,
    mut os: *mut obstack,
) {
    let mut n_pos: size_t = 0;
    n_pos = ({
        let mut __o: *const obstack = os;
        ((*__o).next_free).offset_from((*__o).object_base) as libc::c_long as size_t
    })
        .wrapping_div(::core::mem::size_of::<off_t>() as libc::c_ulong);
    if n_pos > 0 as libc::c_int as libc::c_ulong {
        let mut pos: *mut off_t = ({
            let mut __o1: *mut obstack = os;
            let mut __value: *mut libc::c_void = (*__o1).object_base
                as *mut libc::c_void;
            if (*__o1).next_free == __value as *mut libc::c_char {
                (*__o1).set_maybe_empty_object(1 as libc::c_int as libc::c_uint);
            }
            (*__o1)
                .next_free = (if (::core::mem::size_of::<ptrdiff_t>() as libc::c_ulong)
                < ::core::mem::size_of::<*mut libc::c_void>() as libc::c_ulong
            {
                (*__o1).object_base
            } else {
                0 as *mut libc::c_char
            })
                .offset(
                    ((((*__o1).next_free)
                        .offset_from(
                            (if (::core::mem::size_of::<ptrdiff_t>() as libc::c_ulong)
                                < ::core::mem::size_of::<*mut libc::c_void>()
                                    as libc::c_ulong
                            {
                                (*__o1).object_base
                            } else {
                                0 as *mut libc::c_char
                            }),
                        ) as libc::c_long as libc::c_ulong)
                        .wrapping_add((*__o1).alignment_mask) & !(*__o1).alignment_mask)
                        as isize,
                );
            if ((*__o1).next_free).offset_from((*__o1).chunk as *mut libc::c_char)
                as libc::c_long as size_t
                > ((*__o1).chunk_limit).offset_from((*__o1).chunk as *mut libc::c_char)
                    as libc::c_long as size_t
            {
                (*__o1).next_free = (*__o1).chunk_limit;
            }
            (*__o1).object_base = (*__o1).next_free;
            __value
        }) as *mut off_t;
        fputs_unlocked(prefix, stdout);
        let mut i: size_t = 0 as libc::c_int as size_t;
        while i < n_pos {
            let mut p: intmax_t = *pos.offset(i as isize);
            printf(b" %ld\0" as *const u8 as *const libc::c_char, p);
            i = i.wrapping_add(1);
        }
        putchar_unlocked('\n' as i32);
    }
}
unsafe extern "C" fn get_stat_btime(mut st: *const stat) -> timespec {
    let mut btimespec: timespec = timespec { tv_sec: 0, tv_nsec: 0 };
    btimespec = get_stat_mtime(st);
    return btimespec;
}
unsafe extern "C" fn time_type_to_statx() -> libc::c_uint {
    match time_type as libc::c_uint {
        1 => return 0x80 as libc::c_uint,
        0 => return 0x40 as libc::c_uint,
        2 => return 0x20 as libc::c_uint,
        3 => return 0x800 as libc::c_uint,
        _ => {
            abort();
        }
    };
}
unsafe extern "C" fn calc_req_mask() -> libc::c_uint {
    let mut mask: libc::c_uint = 0x2 as libc::c_uint;
    if print_inode {
        mask |= 0x100 as libc::c_uint;
    }
    if print_block_size {
        mask |= 0x400 as libc::c_uint;
    }
    if format as libc::c_uint == long_format as libc::c_int as libc::c_uint {
        mask |= 0x4 as libc::c_uint | 0x200 as libc::c_uint | time_type_to_statx();
        if print_owner as libc::c_int != 0 || print_author as libc::c_int != 0 {
            mask |= 0x8 as libc::c_uint;
        }
        if print_group {
            mask |= 0x10 as libc::c_uint;
        }
    }
    match sort_type as libc::c_uint {
        6 | 0 | 4 | 1 | 2 => {}
        5 => {
            mask |= time_type_to_statx();
        }
        3 => {
            mask |= 0x200 as libc::c_uint;
        }
        _ => {
            abort();
        }
    }
    return mask;
}
unsafe extern "C" fn do_statx(
    mut fd: libc::c_int,
    mut name: *const libc::c_char,
    mut st: *mut stat,
    mut flags: libc::c_int,
    mut mask: libc::c_uint,
) -> libc::c_int {
    let mut stx: statx = statx {
        stx_mask: 0,
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
    let mut want_btime: bool = mask & 0x800 as libc::c_uint != 0;
    let mut ret: libc::c_int = statx(
        fd,
        name,
        flags | 0x800 as libc::c_int,
        mask,
        &mut stx,
    );
    if ret >= 0 as libc::c_int {
        statx_to_stat(&mut stx, st);
        if want_btime {
            if stx.stx_mask & 0x800 as libc::c_uint != 0 {
                (*st).st_mtim = statx_timestamp_to_timespec(stx.stx_btime);
            } else {
                (*st).st_mtim.tv_nsec = -(1 as libc::c_int) as __syscall_slong_t;
                (*st).st_mtim.tv_sec = (*st).st_mtim.tv_nsec;
            }
        }
    }
    return ret;
}
unsafe extern "C" fn do_stat(
    mut name: *const libc::c_char,
    mut st: *mut stat,
) -> libc::c_int {
    return do_statx(-(100 as libc::c_int), name, st, 0 as libc::c_int, calc_req_mask());
}
unsafe extern "C" fn do_lstat(
    mut name: *const libc::c_char,
    mut st: *mut stat,
) -> libc::c_int {
    return do_statx(
        -(100 as libc::c_int),
        name,
        st,
        0x100 as libc::c_int,
        calc_req_mask(),
    );
}
unsafe extern "C" fn stat_for_mode(
    mut name: *const libc::c_char,
    mut st: *mut stat,
) -> libc::c_int {
    return do_statx(
        -(100 as libc::c_int),
        name,
        st,
        0 as libc::c_int,
        0x2 as libc::c_uint,
    );
}
unsafe extern "C" fn stat_for_ino(
    mut name: *const libc::c_char,
    mut st: *mut stat,
) -> libc::c_int {
    return do_statx(
        -(100 as libc::c_int),
        name,
        st,
        0 as libc::c_int,
        0x100 as libc::c_uint,
    );
}
unsafe extern "C" fn fstat_for_ino(
    mut fd: libc::c_int,
    mut st: *mut stat,
) -> libc::c_int {
    return do_statx(
        fd,
        b"\0" as *const u8 as *const libc::c_char,
        st,
        0x1000 as libc::c_int,
        0x100 as libc::c_uint,
    );
}
unsafe extern "C" fn first_percent_b(
    mut fmt: *const libc::c_char,
) -> *const libc::c_char {
    while *fmt != 0 {
        if *fmt.offset(0 as libc::c_int as isize) as libc::c_int == '%' as i32 {
            match *fmt.offset(1 as libc::c_int as isize) as libc::c_int {
                98 => return fmt,
                37 => {
                    fmt = fmt.offset(1);
                }
                _ => {}
            }
        }
        fmt = fmt.offset(1);
    }
    return 0 as *const libc::c_char;
}
static mut RFC3986: [libc::c_char; 256] = [0; 256];
unsafe extern "C" fn file_escape_init() {
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < 256 as libc::c_int {
        RFC3986[i
            as usize] = (RFC3986[i as usize] as libc::c_int
            | (c_isalnum(i) as libc::c_int != 0 || i == '~' as i32 || i == '-' as i32
                || i == '.' as i32 || i == '_' as i32) as libc::c_int) as libc::c_char;
        i += 1;
    }
}
static mut abformat: [[[libc::c_char; 128]; 12]; 2] = [[[0; 128]; 12]; 2];
static mut use_abformat: bool = false;
unsafe extern "C" fn abmon_init(mut abmon: *mut [libc::c_char; 128]) -> bool {
    let mut required_mon_width: size_t = MAX_MON_WIDTH as libc::c_int as size_t;
    let mut curr_max_width: size_t = 0;
    loop {
        curr_max_width = required_mon_width;
        required_mon_width = 0 as libc::c_int as size_t;
        let mut i: libc::c_int = 0 as libc::c_int;
        while i < 12 as libc::c_int {
            let mut width: size_t = curr_max_width;
            let mut abbr: *const libc::c_char = nl_langinfo(ABMON_1 as libc::c_int + i);
            if !(strchr(abbr, '%' as i32)).is_null() {
                return 0 as libc::c_int != 0;
            }
            let mut alignment: mbs_align_t = (if *(*__ctype_b_loc())
                .offset(to_uchar(*abbr) as libc::c_int as isize) as libc::c_int
                & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int != 0
            {
                MBS_ALIGN_RIGHT as libc::c_int
            } else {
                MBS_ALIGN_LEFT as libc::c_int
            }) as mbs_align_t;
            let mut req: size_t = mbsalign(
                abbr,
                (*abmon.offset(i as isize)).as_mut_ptr(),
                ABFORMAT_SIZE as libc::c_int as size_t,
                &mut width,
                alignment,
                0 as libc::c_int,
            );
            if !(req < ABFORMAT_SIZE as libc::c_int as libc::c_ulong) {
                return 0 as libc::c_int != 0;
            }
            required_mon_width = if required_mon_width > width {
                required_mon_width
            } else {
                width
            };
            i += 1;
        }
        if !(curr_max_width > required_mon_width) {
            break;
        }
    }
    return 1 as libc::c_int != 0;
}
unsafe extern "C" fn abformat_init() {
    let mut pb: [*const libc::c_char; 2] = [0 as *const libc::c_char; 2];
    let mut recent: libc::c_int = 0 as libc::c_int;
    while recent < 2 as libc::c_int {
        pb[recent as usize] = first_percent_b(long_time_format[recent as usize]);
        recent += 1;
    }
    if !(!(pb[0 as libc::c_int as usize]).is_null()
        || !(pb[1 as libc::c_int as usize]).is_null())
    {
        return;
    }
    let mut abmon: [[libc::c_char; 128]; 12] = [[0; 128]; 12];
    if !abmon_init(abmon.as_mut_ptr()) {
        return;
    }
    let mut recent_0: libc::c_int = 0 as libc::c_int;
    while recent_0 < 2 as libc::c_int {
        let mut fmt: *const libc::c_char = long_time_format[recent_0 as usize];
        let mut i: libc::c_int = 0 as libc::c_int;
        while i < 12 as libc::c_int {
            let mut nfmt: *mut libc::c_char = (abformat[recent_0 as usize][i as usize])
                .as_mut_ptr();
            let mut nbytes: libc::c_int = 0;
            if (pb[recent_0 as usize]).is_null() {
                nbytes = snprintf(
                    nfmt,
                    ABFORMAT_SIZE as libc::c_int as libc::c_ulong,
                    b"%s\0" as *const u8 as *const libc::c_char,
                    fmt,
                );
            } else {
                if !((pb[recent_0 as usize]).offset_from(fmt) as libc::c_long
                    <= (if (ABFORMAT_SIZE as libc::c_int) < 2147483647 as libc::c_int {
                        ABFORMAT_SIZE as libc::c_int
                    } else {
                        2147483647 as libc::c_int
                    }) as libc::c_long)
                {
                    return;
                }
                let mut prefix_len: libc::c_int = (pb[recent_0 as usize])
                    .offset_from(fmt) as libc::c_long as libc::c_int;
                nbytes = snprintf(
                    nfmt,
                    ABFORMAT_SIZE as libc::c_int as libc::c_ulong,
                    b"%.*s%s%s\0" as *const u8 as *const libc::c_char,
                    prefix_len,
                    fmt,
                    (abmon[i as usize]).as_mut_ptr(),
                    (pb[recent_0 as usize]).offset(2 as libc::c_int as isize),
                );
            }
            if !(0 as libc::c_int <= nbytes && nbytes < ABFORMAT_SIZE as libc::c_int) {
                return;
            }
            i += 1;
        }
        recent_0 += 1;
    }
    use_abformat = 1 as libc::c_int != 0;
}
unsafe extern "C" fn dev_ino_hash(
    mut x: *const libc::c_void,
    mut table_size: size_t,
) -> size_t {
    let mut p: *const dev_ino = x as *const dev_ino;
    return ((*p).st_ino).wrapping_rem(table_size);
}
unsafe extern "C" fn dev_ino_compare(
    mut x: *const libc::c_void,
    mut y: *const libc::c_void,
) -> bool {
    let mut a: *const dev_ino = x as *const dev_ino;
    let mut b: *const dev_ino = y as *const dev_ino;
    return if (*a).st_ino == (*b).st_ino && (*a).st_dev == (*b).st_dev {
        1 as libc::c_int
    } else {
        0 as libc::c_int
    } != 0;
}
unsafe extern "C" fn dev_ino_free(mut x: *mut libc::c_void) {
    free(x);
}
unsafe extern "C" fn visit_dir(mut dev: dev_t, mut ino: ino_t) -> bool {
    let mut ent: *mut dev_ino = 0 as *mut dev_ino;
    let mut ent_from_table: *mut dev_ino = 0 as *mut dev_ino;
    let mut found_match: bool = false;
    ent = xmalloc(::core::mem::size_of::<dev_ino>() as libc::c_ulong) as *mut dev_ino;
    (*ent).st_ino = ino;
    (*ent).st_dev = dev;
    ent_from_table = hash_insert(active_dir_set, ent as *const libc::c_void)
        as *mut dev_ino;
    if ent_from_table.is_null() {
        xalloc_die();
    }
    found_match = ent_from_table != ent;
    if found_match {
        free(ent as *mut libc::c_void);
    }
    return found_match;
}
unsafe extern "C" fn free_pending_ent(mut p: *mut pending) {
    free((*p).name as *mut libc::c_void);
    free((*p).realname as *mut libc::c_void);
    free(p as *mut libc::c_void);
}
unsafe extern "C" fn is_colored(mut type_0: indicator_no) -> bool {
    let mut len: size_t = color_indicator[type_0 as usize].len;
    let mut s: *const libc::c_char = color_indicator[type_0 as usize].string;
    return !(len == 0 as libc::c_int as libc::c_ulong
        || len == 1 as libc::c_int as libc::c_ulong
            && strncmp(
                s,
                b"0\0" as *const u8 as *const libc::c_char,
                (::core::mem::size_of::<[libc::c_char; 2]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong),
            ) == 0 as libc::c_int
        || len == 2 as libc::c_int as libc::c_ulong
            && strncmp(
                s,
                b"00\0" as *const u8 as *const libc::c_char,
                (::core::mem::size_of::<[libc::c_char; 3]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong),
            ) == 0 as libc::c_int);
}
unsafe extern "C" fn restore_default_color() {
    put_indicator(
        &mut *color_indicator.as_mut_ptr().offset(C_LEFT as libc::c_int as isize),
    );
    put_indicator(
        &mut *color_indicator.as_mut_ptr().offset(C_RIGHT as libc::c_int as isize),
    );
}
unsafe extern "C" fn set_normal_color() {
    if print_with_color as libc::c_int != 0 && is_colored(C_NORM) as libc::c_int != 0 {
        put_indicator(
            &mut *color_indicator.as_mut_ptr().offset(C_LEFT as libc::c_int as isize),
        );
        put_indicator(
            &mut *color_indicator.as_mut_ptr().offset(C_NORM as libc::c_int as isize),
        );
        put_indicator(
            &mut *color_indicator.as_mut_ptr().offset(C_RIGHT as libc::c_int as isize),
        );
    }
}
unsafe extern "C" fn sighandler(mut sig: libc::c_int) {
    if 1 as libc::c_int == 0 {
        signal(
            sig,
            ::core::mem::transmute::<
                libc::intptr_t,
                __sighandler_t,
            >(1 as libc::c_int as libc::intptr_t),
        );
    }
    if interrupt_signal == 0 {
        ::core::ptr::write_volatile(&mut interrupt_signal as *mut sig_atomic_t, sig);
    }
}
unsafe extern "C" fn stophandler(mut sig: libc::c_int) {
    if 1 as libc::c_int == 0 {
        signal(sig, Some(stophandler as unsafe extern "C" fn(libc::c_int) -> ()));
    }
    if interrupt_signal == 0 {
        ::core::ptr::write_volatile(
            &mut stop_signal_count as *mut sig_atomic_t,
            ::core::ptr::read_volatile::<
                sig_atomic_t,
            >(&stop_signal_count as *const sig_atomic_t) + 1,
        );
    }
}
unsafe extern "C" fn process_signals() {
    while interrupt_signal != 0 || stop_signal_count != 0 {
        let mut sig: libc::c_int = 0;
        let mut stops: libc::c_int = 0;
        let mut oldset: sigset_t = sigset_t { __val: [0; 16] };
        if used_color {
            restore_default_color();
        }
        fflush_unlocked(stdout);
        sigprocmask(0 as libc::c_int, &mut caught_signals, &mut oldset);
        sig = interrupt_signal;
        stops = stop_signal_count;
        if stops != 0 {
            ::core::ptr::write_volatile(
                &mut stop_signal_count as *mut sig_atomic_t,
                stops - 1 as libc::c_int,
            );
            sig = 19 as libc::c_int;
        } else {
            signal(sig, None);
        }
        raise(sig);
        sigprocmask(2 as libc::c_int, &mut oldset, 0 as *mut sigset_t);
    }
}
unsafe extern "C" fn signal_setup(mut init: bool) {
    static mut sig: [libc::c_int; 12] = [
        20 as libc::c_int,
        14 as libc::c_int,
        1 as libc::c_int,
        2 as libc::c_int,
        13 as libc::c_int,
        3 as libc::c_int,
        15 as libc::c_int,
        29 as libc::c_int,
        27 as libc::c_int,
        26 as libc::c_int,
        24 as libc::c_int,
        25 as libc::c_int,
    ];
    let mut j: libc::c_int = 0;
    if init {
        let mut act: sigaction = sigaction {
            __sigaction_handler: C2RustUnnamed_13 {
                sa_handler: None,
            },
            sa_mask: sigset_t { __val: [0; 16] },
            sa_flags: 0,
            sa_restorer: None,
        };
        sigemptyset(&mut caught_signals);
        j = 0 as libc::c_int;
        while j < nsigs as libc::c_int {
            sigaction(sig[j as usize], 0 as *const sigaction, &mut act);
            if act.__sigaction_handler.sa_handler
                != ::core::mem::transmute::<
                    libc::intptr_t,
                    __sighandler_t,
                >(1 as libc::c_int as libc::intptr_t)
            {
                sigaddset(&mut caught_signals, sig[j as usize]);
            }
            j += 1;
        }
        act.sa_mask = caught_signals;
        act.sa_flags = 0x10000000 as libc::c_int;
        j = 0 as libc::c_int;
        while j < nsigs as libc::c_int {
            if sigismember(&mut caught_signals, sig[j as usize]) != 0 {
                act
                    .__sigaction_handler
                    .sa_handler = if sig[j as usize] == 20 as libc::c_int {
                    Some(stophandler as unsafe extern "C" fn(libc::c_int) -> ())
                } else {
                    Some(sighandler as unsafe extern "C" fn(libc::c_int) -> ())
                };
                sigaction(sig[j as usize], &mut act, 0 as *mut sigaction);
            }
            j += 1;
        }
    } else {
        j = 0 as libc::c_int;
        while j < nsigs as libc::c_int {
            if sigismember(&mut caught_signals, sig[j as usize]) != 0 {
                signal(sig[j as usize], None);
            }
            j += 1;
        }
    };
}
unsafe extern "C" fn signal_init() {
    signal_setup(1 as libc::c_int != 0);
}
unsafe extern "C" fn signal_restore() {
    signal_setup(0 as libc::c_int != 0);
}
unsafe fn main_0(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut thispend: *mut pending = 0 as *mut pending;
    let mut n_files: libc::c_int = 0;
    set_program_name(*argv.offset(0 as libc::c_int as isize));
    setlocale(6 as libc::c_int, b"\0" as *const u8 as *const libc::c_char);
    bindtextdomain(
        b"coreutils\0" as *const u8 as *const libc::c_char,
        b"/usr/local/share/locale\0" as *const u8 as *const libc::c_char,
    );
    textdomain(b"coreutils\0" as *const u8 as *const libc::c_char);
    initialize_exit_failure(LS_FAILURE as libc::c_int);
    atexit(Some(close_stdout as unsafe extern "C" fn() -> ()));
    if (::core::mem::size_of::<[bin_str; 24]>() as libc::c_ulong)
        .wrapping_div(::core::mem::size_of::<bin_str>() as libc::c_ulong)
        .wrapping_add(1 as libc::c_int as libc::c_ulong)
        == (::core::mem::size_of::<[*const libc::c_char; 25]>() as libc::c_ulong)
            .wrapping_div(::core::mem::size_of::<*const libc::c_char>() as libc::c_ulong)
    {} else {
        __assert_fail(
            b"ARRAY_CARDINALITY (color_indicator) + 1 == ARRAY_CARDINALITY (indicator_name)\0"
                as *const u8 as *const libc::c_char,
            b"src/ls.c\0" as *const u8 as *const libc::c_char,
            1663 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 23],
                &[libc::c_char; 23],
            >(b"int main(int, char **)\0"))
                .as_ptr(),
        );
    }
    exit_status = 0 as libc::c_int;
    print_dir_name = 1 as libc::c_int != 0;
    pending_dirs = 0 as *mut pending;
    current_time
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
    current_time.tv_nsec = -(1 as libc::c_int) as __syscall_slong_t;
    i = decode_switches(argc, argv);
    if print_with_color {
        parse_ls_color();
    }
    if print_with_color {
        tabsize = 0 as libc::c_int as size_t;
    }
    if directories_first {
        check_symlink_mode = 1 as libc::c_int != 0;
    } else if print_with_color {
        if is_colored(C_ORPHAN) as libc::c_int != 0
            || is_colored(C_EXEC) as libc::c_int != 0
                && color_symlink_as_referent as libc::c_int != 0
            || is_colored(C_MISSING) as libc::c_int != 0
                && format as libc::c_uint == long_format as libc::c_int as libc::c_uint
        {
            check_symlink_mode = 1 as libc::c_int != 0;
        }
    }
    if dereference as libc::c_uint == DEREF_UNDEFINED as libc::c_int as libc::c_uint {
        dereference = (if immediate_dirs as libc::c_int != 0
            || indicator_style as libc::c_uint == classify as libc::c_int as libc::c_uint
            || format as libc::c_uint == long_format as libc::c_int as libc::c_uint
        {
            DEREF_NEVER as libc::c_int
        } else {
            DEREF_COMMAND_LINE_SYMLINK_TO_DIR as libc::c_int
        }) as Dereference_symlink;
    }
    if recursive {
        active_dir_set = hash_initialize(
            INITIAL_TABLE_SIZE as libc::c_int as size_t,
            0 as *const Hash_tuning,
            Some(
                dev_ino_hash
                    as unsafe extern "C" fn(*const libc::c_void, size_t) -> size_t,
            ),
            Some(
                dev_ino_compare
                    as unsafe extern "C" fn(
                        *const libc::c_void,
                        *const libc::c_void,
                    ) -> bool,
            ),
            Some(dev_ino_free as unsafe extern "C" fn(*mut libc::c_void) -> ()),
        );
        if active_dir_set.is_null() {
            xalloc_die();
        }
        _obstack_begin(
            &mut dev_ino_obstack,
            0 as libc::c_int as size_t,
            0 as libc::c_int as size_t,
            Some(malloc as unsafe extern "C" fn(libc::c_ulong) -> *mut libc::c_void),
            Some(free as unsafe extern "C" fn(*mut libc::c_void) -> ()),
        );
    }
    localtz = tzalloc(getenv(b"TZ\0" as *const u8 as *const libc::c_char));
    format_needs_stat = sort_type as libc::c_uint
        == sort_time as libc::c_int as libc::c_uint
        || sort_type as libc::c_uint == sort_size as libc::c_int as libc::c_uint
        || format as libc::c_uint == long_format as libc::c_int as libc::c_uint
        || print_scontext as libc::c_int != 0 || print_block_size as libc::c_int != 0;
    format_needs_type = !format_needs_stat
        && (recursive as libc::c_int != 0 || print_with_color as libc::c_int != 0
            || indicator_style as libc::c_uint != none as libc::c_int as libc::c_uint
            || directories_first as libc::c_int != 0);
    if dired {
        _obstack_begin(
            &mut dired_obstack,
            0 as libc::c_int as size_t,
            0 as libc::c_int as size_t,
            Some(malloc as unsafe extern "C" fn(libc::c_ulong) -> *mut libc::c_void),
            Some(free as unsafe extern "C" fn(*mut libc::c_void) -> ()),
        );
        _obstack_begin(
            &mut subdired_obstack,
            0 as libc::c_int as size_t,
            0 as libc::c_int as size_t,
            Some(malloc as unsafe extern "C" fn(libc::c_ulong) -> *mut libc::c_void),
            Some(free as unsafe extern "C" fn(*mut libc::c_void) -> ()),
        );
    }
    if print_hyperlink {
        file_escape_init();
        hostname = xgethostname();
        if hostname.is_null() {
            hostname = b"\0" as *const u8 as *const libc::c_char;
        }
    }
    cwd_n_alloc = 100 as libc::c_int as size_t;
    cwd_file = xnmalloc(cwd_n_alloc, ::core::mem::size_of::<fileinfo>() as libc::c_ulong)
        as *mut fileinfo;
    cwd_n_used = 0 as libc::c_int as size_t;
    clear_files();
    n_files = argc - i;
    if n_files <= 0 as libc::c_int {
        if immediate_dirs {
            gobble_file(
                b".\0" as *const u8 as *const libc::c_char,
                directory,
                NOT_AN_INODE_NUMBER as libc::c_int as ino_t,
                1 as libc::c_int != 0,
                b"\0" as *const u8 as *const libc::c_char,
            );
        } else {
            queue_directory(
                b".\0" as *const u8 as *const libc::c_char,
                0 as *const libc::c_char,
                1 as libc::c_int != 0,
            );
        }
    } else {
        loop {
            let fresh3 = i;
            i = i + 1;
            gobble_file(
                *argv.offset(fresh3 as isize),
                unknown,
                NOT_AN_INODE_NUMBER as libc::c_int as ino_t,
                1 as libc::c_int != 0,
                b"\0" as *const u8 as *const libc::c_char,
            );
            if !(i < argc) {
                break;
            }
        }
    }
    if cwd_n_used != 0 {
        sort_files();
        if !immediate_dirs {
            extract_dirs_from_files(0 as *const libc::c_char, 1 as libc::c_int != 0);
        }
    }
    if cwd_n_used != 0 {
        print_current_files();
        if !pending_dirs.is_null() {
            dired_outbyte('\n' as i32 as libc::c_char);
        }
    } else if n_files <= 1 as libc::c_int && !pending_dirs.is_null()
        && ((*pending_dirs).next).is_null()
    {
        print_dir_name = 0 as libc::c_int != 0;
    }
    while !pending_dirs.is_null() {
        thispend = pending_dirs;
        pending_dirs = (*pending_dirs).next;
        if !active_dir_set.is_null() {
            if ((*thispend).name).is_null() {
                let mut di: dev_ino = dev_ino_pop();
                let mut found: *mut dev_ino = hash_remove(
                    active_dir_set,
                    &mut di as *mut dev_ino as *const libc::c_void,
                ) as *mut dev_ino;
                if !found.is_null() {} else {
                    __assert_fail(
                        b"found\0" as *const u8 as *const libc::c_char,
                        b"src/ls.c\0" as *const u8 as *const libc::c_char,
                        1807 as libc::c_int as libc::c_uint,
                        (*::core::mem::transmute::<
                            &[u8; 23],
                            &[libc::c_char; 23],
                        >(b"int main(int, char **)\0"))
                            .as_ptr(),
                    );
                }
                dev_ino_free(found as *mut libc::c_void);
                free_pending_ent(thispend);
                continue;
            }
        }
        print_dir((*thispend).name, (*thispend).realname, (*thispend).command_line_arg);
        free_pending_ent(thispend);
        print_dir_name = 1 as libc::c_int != 0;
    }
    if print_with_color as libc::c_int != 0 && used_color as libc::c_int != 0 {
        let mut j: libc::c_int = 0;
        if !(color_indicator[C_LEFT as libc::c_int as usize].len
            == 2 as libc::c_int as libc::c_ulong
            && memcmp(
                color_indicator[C_LEFT as libc::c_int as usize].string
                    as *const libc::c_void,
                b"\x1B[\0" as *const u8 as *const libc::c_char as *const libc::c_void,
                2 as libc::c_int as libc::c_ulong,
            ) == 0 as libc::c_int
            && color_indicator[C_RIGHT as libc::c_int as usize].len
                == 1 as libc::c_int as libc::c_ulong
            && *(color_indicator[C_RIGHT as libc::c_int as usize].string)
                .offset(0 as libc::c_int as isize) as libc::c_int == 'm' as i32)
        {
            restore_default_color();
        }
        fflush_unlocked(stdout);
        signal_restore();
        j = stop_signal_count;
        while j != 0 {
            raise(19 as libc::c_int);
            j -= 1;
        }
        j = interrupt_signal;
        if j != 0 {
            raise(j);
        }
    }
    if dired {
        dired_dump_obstack(
            b"//DIRED//\0" as *const u8 as *const libc::c_char,
            &mut dired_obstack,
        );
        dired_dump_obstack(
            b"//SUBDIRED//\0" as *const u8 as *const libc::c_char,
            &mut subdired_obstack,
        );
        printf(
            b"//DIRED-OPTIONS// --quoting-style=%s\n\0" as *const u8
                as *const libc::c_char,
            *quoting_style_args
                .as_ptr()
                .offset(get_quoting_style(filename_quoting_options) as isize),
        );
    }
    if !active_dir_set.is_null() {
        if hash_get_n_entries(active_dir_set) == 0 as libc::c_int as libc::c_ulong
        {} else {
            __assert_fail(
                b"hash_get_n_entries (active_dir_set) == 0\0" as *const u8
                    as *const libc::c_char,
                b"src/ls.c\0" as *const u8 as *const libc::c_char,
                1859 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 23],
                    &[libc::c_char; 23],
                >(b"int main(int, char **)\0"))
                    .as_ptr(),
            );
        }
        hash_free(active_dir_set);
    }
    return exit_status;
}
unsafe extern "C" fn decode_line_length(mut spec: *const libc::c_char) -> ptrdiff_t {
    let mut val: uintmax_t = 0;
    match xstrtoumax(
        spec,
        0 as *mut *mut libc::c_char,
        0 as libc::c_int,
        &mut val,
        b"\0" as *const u8 as *const libc::c_char,
    ) as libc::c_uint
    {
        0 => {
            return (if val
                <= (if (9223372036854775807 as libc::c_long as libc::c_ulong)
                    < 18446744073709551615 as libc::c_ulong
                {
                    9223372036854775807 as libc::c_long as libc::c_ulong
                } else {
                    18446744073709551615 as libc::c_ulong
                })
            {
                val
            } else {
                0 as libc::c_int as libc::c_ulong
            }) as ptrdiff_t;
        }
        1 => return 0 as libc::c_int as ptrdiff_t,
        _ => return -(1 as libc::c_int) as ptrdiff_t,
    };
}
unsafe extern "C" fn stdout_isatty() -> bool {
    static mut out_tty: libc::c_schar = -(1 as libc::c_int) as libc::c_schar;
    if (out_tty as libc::c_int) < 0 as libc::c_int {
        out_tty = isatty(1 as libc::c_int) as libc::c_schar;
    }
    if out_tty as libc::c_int == 0 as libc::c_int
        || out_tty as libc::c_int == 1 as libc::c_int
    {} else {
        unreachable!();
    };
    return out_tty != 0;
}
unsafe extern "C" fn decode_switches(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut time_style_option: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut kibibytes_specified: bool = 0 as libc::c_int != 0;
    let mut format_opt: libc::c_int = -(1 as libc::c_int);
    let mut hide_control_chars_opt: libc::c_int = -(1 as libc::c_int);
    let mut quoting_style_opt: libc::c_int = -(1 as libc::c_int);
    let mut sort_opt: libc::c_int = -(1 as libc::c_int);
    let mut tabsize_opt: ptrdiff_t = -(1 as libc::c_int) as ptrdiff_t;
    let mut width_opt: ptrdiff_t = -(1 as libc::c_int) as ptrdiff_t;
    loop {
        let mut oi: libc::c_int = -(1 as libc::c_int);
        let mut c: libc::c_int = getopt_long(
            argc,
            argv,
            b"abcdfghiklmnopqrstuvw:xABCDFGHI:LNQRST:UXZ1\0" as *const u8
                as *const libc::c_char,
            long_options.as_ptr(),
            &mut oi,
        );
        if c == -(1 as libc::c_int) {
            break;
        }
        match c {
            97 => {
                ignore_mode = IGNORE_MINIMAL;
            }
            98 => {
                quoting_style_opt = escape_quoting_style as libc::c_int;
            }
            99 => {
                time_type = time_ctime;
            }
            100 => {
                immediate_dirs = 1 as libc::c_int != 0;
            }
            102 => {
                ignore_mode = IGNORE_MINIMAL;
                sort_opt = sort_none as libc::c_int;
                if format_opt == long_format as libc::c_int {
                    format_opt = -(1 as libc::c_int);
                }
                print_with_color = 0 as libc::c_int != 0;
                print_hyperlink = 0 as libc::c_int != 0;
                print_block_size = 0 as libc::c_int != 0;
            }
            260 => {
                indicator_style = file_type;
            }
            103 => {
                format_opt = long_format as libc::c_int;
                print_owner = 0 as libc::c_int != 0;
            }
            104 => {
                human_output_opts = human_autoscale as libc::c_int
                    | human_SI as libc::c_int | human_base_1024 as libc::c_int;
                file_human_output_opts = human_output_opts;
                output_block_size = 1 as libc::c_int as uintmax_t;
                file_output_block_size = output_block_size;
            }
            105 => {
                print_inode = 1 as libc::c_int != 0;
            }
            107 => {
                kibibytes_specified = 1 as libc::c_int != 0;
            }
            108 => {
                format_opt = long_format as libc::c_int;
            }
            109 => {
                format_opt = with_commas as libc::c_int;
            }
            110 => {
                numeric_ids = 1 as libc::c_int != 0;
                format_opt = long_format as libc::c_int;
            }
            111 => {
                format_opt = long_format as libc::c_int;
                print_group = 0 as libc::c_int != 0;
            }
            112 => {
                indicator_style = slash;
            }
            113 => {
                hide_control_chars_opt = 1 as libc::c_int;
            }
            114 => {
                sort_reverse = 1 as libc::c_int != 0;
            }
            115 => {
                print_block_size = 1 as libc::c_int != 0;
            }
            116 => {
                sort_opt = sort_time as libc::c_int;
            }
            117 => {
                time_type = time_atime;
            }
            118 => {
                sort_opt = sort_version as libc::c_int;
            }
            119 => {
                width_opt = decode_line_length(optarg);
                if width_opt < 0 as libc::c_int as libc::c_long {
                    if ::core::mem::size_of::<C2RustUnnamed_24>() as libc::c_ulong != 0 {
                        error(
                            LS_FAILURE as libc::c_int,
                            0 as libc::c_int,
                            b"%s: %s\0" as *const u8 as *const libc::c_char,
                            dcgettext(
                                0 as *const libc::c_char,
                                b"invalid line width\0" as *const u8 as *const libc::c_char,
                                5 as libc::c_int,
                            ),
                            quote(optarg),
                        );
                        if 0 as libc::c_int != 0 {} else {
                            unreachable!();
                        };
                    } else {
                        error(
                            LS_FAILURE as libc::c_int,
                            0 as libc::c_int,
                            b"%s: %s\0" as *const u8 as *const libc::c_char,
                            dcgettext(
                                0 as *const libc::c_char,
                                b"invalid line width\0" as *const u8 as *const libc::c_char,
                                5 as libc::c_int,
                            ),
                            quote(optarg),
                        );
                        if 0 as libc::c_int != 0 {} else {
                            unreachable!();
                        };
                    };
                }
            }
            120 => {
                format_opt = horizontal as libc::c_int;
            }
            65 => {
                ignore_mode = IGNORE_DOT_AND_DOTDOT;
            }
            66 => {
                add_ignore_pattern(b"*~\0" as *const u8 as *const libc::c_char);
                add_ignore_pattern(b".*~\0" as *const u8 as *const libc::c_char);
            }
            67 => {
                format_opt = many_per_line as libc::c_int;
            }
            68 => {
                dired = 1 as libc::c_int != 0;
            }
            70 => {
                let mut i: libc::c_int = 0;
                if !optarg.is_null() {
                    i = when_types[__xargmatch_internal(
                        b"--classify\0" as *const u8 as *const libc::c_char,
                        optarg,
                        when_args.as_ptr(),
                        when_types.as_ptr() as *const libc::c_void,
                        ::core::mem::size_of::<when_type>() as libc::c_ulong,
                        argmatch_die,
                        1 as libc::c_int != 0,
                    ) as usize] as libc::c_int;
                } else {
                    i = when_always as libc::c_int;
                }
                if i == when_always as libc::c_int
                    || i == when_if_tty as libc::c_int
                        && stdout_isatty() as libc::c_int != 0
                {
                    indicator_style = classify;
                }
            }
            71 => {
                print_group = 0 as libc::c_int != 0;
            }
            72 => {
                dereference = DEREF_COMMAND_LINE_ARGUMENTS;
            }
            259 => {
                dereference = DEREF_COMMAND_LINE_SYMLINK_TO_DIR;
            }
            73 => {
                add_ignore_pattern(optarg);
            }
            76 => {
                dereference = DEREF_ALWAYS;
            }
            78 => {
                quoting_style_opt = literal_quoting_style as libc::c_int;
            }
            81 => {
                quoting_style_opt = c_quoting_style as libc::c_int;
            }
            82 => {
                recursive = 1 as libc::c_int != 0;
            }
            83 => {
                sort_opt = sort_size as libc::c_int;
            }
            84 => {
                tabsize_opt = xnumtoumax(
                    optarg,
                    0 as libc::c_int,
                    0 as libc::c_int as uintmax_t,
                    if (9223372036854775807 as libc::c_long as libc::c_ulong)
                        < 18446744073709551615 as libc::c_ulong
                    {
                        9223372036854775807 as libc::c_long as libc::c_ulong
                    } else {
                        18446744073709551615 as libc::c_ulong
                    },
                    b"\0" as *const u8 as *const libc::c_char,
                    dcgettext(
                        0 as *const libc::c_char,
                        b"invalid tab size\0" as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    LS_FAILURE as libc::c_int,
                ) as ptrdiff_t;
            }
            85 => {
                sort_opt = sort_none as libc::c_int;
            }
            88 => {
                sort_opt = sort_extension as libc::c_int;
            }
            49 => {
                if format_opt != long_format as libc::c_int {
                    format_opt = one_per_line as libc::c_int;
                }
            }
            256 => {
                print_author = 1 as libc::c_int != 0;
            }
            264 => {
                let mut hide: *mut ignore_pattern = xmalloc(
                    ::core::mem::size_of::<ignore_pattern>() as libc::c_ulong,
                ) as *mut ignore_pattern;
                (*hide).pattern = optarg;
                (*hide).next = hide_patterns;
                hide_patterns = hide;
            }
            270 => {
                sort_opt = sort_types[__xargmatch_internal(
                    b"--sort\0" as *const u8 as *const libc::c_char,
                    optarg,
                    sort_args.as_ptr(),
                    sort_types.as_ptr() as *const libc::c_void,
                    ::core::mem::size_of::<sort_type>() as libc::c_ulong,
                    argmatch_die,
                    1 as libc::c_int != 0,
                ) as usize] as libc::c_int;
            }
            263 => {
                directories_first = 1 as libc::c_int != 0;
            }
            271 => {
                time_type = time_types[__xargmatch_internal(
                    b"--time\0" as *const u8 as *const libc::c_char,
                    optarg,
                    time_args.as_ptr(),
                    time_types.as_ptr() as *const libc::c_void,
                    ::core::mem::size_of::<time_type>() as libc::c_ulong,
                    argmatch_die,
                    1 as libc::c_int != 0,
                ) as usize];
            }
            261 => {
                format_opt = format_types[__xargmatch_internal(
                    b"--format\0" as *const u8 as *const libc::c_char,
                    optarg,
                    format_args.as_ptr(),
                    format_types.as_ptr() as *const libc::c_void,
                    ::core::mem::size_of::<format>() as libc::c_ulong,
                    argmatch_die,
                    1 as libc::c_int != 0,
                ) as usize] as libc::c_int;
            }
            262 => {
                format_opt = long_format as libc::c_int;
                time_style_option = bad_cast(
                    b"full-iso\0" as *const u8 as *const libc::c_char,
                );
            }
            258 => {
                let mut i_0: libc::c_int = 0;
                if !optarg.is_null() {
                    i_0 = when_types[__xargmatch_internal(
                        b"--color\0" as *const u8 as *const libc::c_char,
                        optarg,
                        when_args.as_ptr(),
                        when_types.as_ptr() as *const libc::c_void,
                        ::core::mem::size_of::<when_type>() as libc::c_ulong,
                        argmatch_die,
                        1 as libc::c_int != 0,
                    ) as usize] as libc::c_int;
                } else {
                    i_0 = when_always as libc::c_int;
                }
                print_with_color = i_0 == when_always as libc::c_int
                    || i_0 == when_if_tty as libc::c_int
                        && stdout_isatty() as libc::c_int != 0;
            }
            265 => {
                let mut i_1: libc::c_int = 0;
                if !optarg.is_null() {
                    i_1 = when_types[__xargmatch_internal(
                        b"--hyperlink\0" as *const u8 as *const libc::c_char,
                        optarg,
                        when_args.as_ptr(),
                        when_types.as_ptr() as *const libc::c_void,
                        ::core::mem::size_of::<when_type>() as libc::c_ulong,
                        argmatch_die,
                        1 as libc::c_int != 0,
                    ) as usize] as libc::c_int;
                } else {
                    i_1 = when_always as libc::c_int;
                }
                print_hyperlink = i_1 == when_always as libc::c_int
                    || i_1 == when_if_tty as libc::c_int
                        && stdout_isatty() as libc::c_int != 0;
            }
            266 => {
                indicator_style = indicator_style_types[__xargmatch_internal(
                    b"--indicator-style\0" as *const u8 as *const libc::c_char,
                    optarg,
                    indicator_style_args.as_ptr(),
                    indicator_style_types.as_ptr() as *const libc::c_void,
                    ::core::mem::size_of::<indicator_style>() as libc::c_ulong,
                    argmatch_die,
                    1 as libc::c_int != 0,
                ) as usize];
            }
            267 => {
                quoting_style_opt = *quoting_style_vals
                    .as_ptr()
                    .offset(
                        __xargmatch_internal(
                            b"--quoting-style\0" as *const u8 as *const libc::c_char,
                            optarg,
                            quoting_style_args.as_ptr(),
                            quoting_style_vals.as_ptr() as *const libc::c_void,
                            ::core::mem::size_of::<quoting_style>() as libc::c_ulong,
                            argmatch_die,
                            1 as libc::c_int != 0,
                        ) as isize,
                    ) as libc::c_int;
            }
            272 => {
                time_style_option = optarg;
            }
            268 => {
                hide_control_chars_opt = 0 as libc::c_int;
            }
            257 => {
                let mut e: strtol_error = human_options(
                    optarg,
                    &mut human_output_opts,
                    &mut output_block_size,
                );
                if e as libc::c_uint != LONGINT_OK as libc::c_int as libc::c_uint {
                    xstrtol_fatal(
                        e,
                        oi,
                        0 as libc::c_int as libc::c_char,
                        long_options.as_ptr(),
                        optarg,
                    );
                }
                file_human_output_opts = human_output_opts;
                file_output_block_size = output_block_size;
            }
            269 => {
                human_output_opts = human_autoscale as libc::c_int
                    | human_SI as libc::c_int;
                file_human_output_opts = human_output_opts;
                output_block_size = 1 as libc::c_int as uintmax_t;
                file_output_block_size = output_block_size;
            }
            90 => {
                print_scontext = 1 as libc::c_int != 0;
            }
            273 => {
                eolbyte = 0 as libc::c_int as libc::c_char;
                hide_control_chars_opt = 0 as libc::c_int;
                if format_opt != long_format as libc::c_int {
                    format_opt = one_per_line as libc::c_int;
                }
                print_with_color = 0 as libc::c_int != 0;
                quoting_style_opt = literal_quoting_style as libc::c_int;
            }
            -2 => {
                usage(0 as libc::c_int);
            }
            -3 => {
                version_etc(
                    stdout,
                    if ls_mode == 1 as libc::c_int {
                        b"ls\0" as *const u8 as *const libc::c_char
                    } else if ls_mode == 2 as libc::c_int {
                        b"dir\0" as *const u8 as *const libc::c_char
                    } else {
                        b"vdir\0" as *const u8 as *const libc::c_char
                    },
                    b"GNU coreutils\0" as *const u8 as *const libc::c_char,
                    Version,
                    b"Richard M. Stallman\0" as *const u8 as *const libc::c_char,
                    b"David MacKenzie\0" as *const u8 as *const libc::c_char,
                    0 as *mut libc::c_void as *mut libc::c_char,
                );
                exit(0 as libc::c_int);
            }
            _ => {
                usage(LS_FAILURE as libc::c_int);
            }
        }
    }
    if output_block_size == 0 {
        let mut ls_block_size: *const libc::c_char = getenv(
            b"LS_BLOCK_SIZE\0" as *const u8 as *const libc::c_char,
        );
        human_options(ls_block_size, &mut human_output_opts, &mut output_block_size);
        if !ls_block_size.is_null()
            || !(getenv(b"BLOCK_SIZE\0" as *const u8 as *const libc::c_char)).is_null()
        {
            file_human_output_opts = human_output_opts;
            file_output_block_size = output_block_size;
        }
        if kibibytes_specified {
            human_output_opts = 0 as libc::c_int;
            output_block_size = 1024 as libc::c_int as uintmax_t;
        }
    }
    format = (if 0 as libc::c_int <= format_opt {
        format_opt
    } else if ls_mode == 1 as libc::c_int {
        if stdout_isatty() as libc::c_int != 0 {
            many_per_line as libc::c_int
        } else {
            one_per_line as libc::c_int
        }
    } else if ls_mode == 2 as libc::c_int {
        many_per_line as libc::c_int
    } else {
        long_format as libc::c_int
    }) as format;
    let mut linelen: ptrdiff_t = width_opt;
    if format as libc::c_uint == many_per_line as libc::c_int as libc::c_uint
        || format as libc::c_uint == horizontal as libc::c_int as libc::c_uint
        || format as libc::c_uint == with_commas as libc::c_int as libc::c_uint
        || print_with_color as libc::c_int != 0
    {
        if linelen < 0 as libc::c_int as libc::c_long {
            let mut ws: winsize = winsize {
                ws_row: 0,
                ws_col: 0,
                ws_xpixel: 0,
                ws_ypixel: 0,
            };
            if stdout_isatty() as libc::c_int != 0
                && 0 as libc::c_int
                    <= ioctl(
                        1 as libc::c_int,
                        0x5413 as libc::c_int as libc::c_ulong,
                        &mut ws as *mut winsize,
                    ) && (0 as libc::c_int) < ws.ws_col as libc::c_int
            {
                linelen = (if ws.ws_col as libc::c_ulong
                    <= (if (9223372036854775807 as libc::c_long as libc::c_ulong)
                        < 18446744073709551615 as libc::c_ulong
                    {
                        9223372036854775807 as libc::c_long as libc::c_ulong
                    } else {
                        18446744073709551615 as libc::c_ulong
                    })
                {
                    ws.ws_col as libc::c_int
                } else {
                    0 as libc::c_int
                }) as ptrdiff_t;
            }
        }
        if linelen < 0 as libc::c_int as libc::c_long {
            let mut p: *const libc::c_char = getenv(
                b"COLUMNS\0" as *const u8 as *const libc::c_char,
            );
            if !p.is_null() && *p as libc::c_int != 0 {
                linelen = decode_line_length(p);
                if linelen < 0 as libc::c_int as libc::c_long {
                    error(
                        0 as libc::c_int,
                        0 as libc::c_int,
                        dcgettext(
                            0 as *const libc::c_char,
                            b"ignoring invalid width in environment variable COLUMNS: %s\0"
                                as *const u8 as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                        quote(p),
                    );
                }
            }
        }
    }
    line_length = (if linelen < 0 as libc::c_int as libc::c_long {
        80 as libc::c_int as libc::c_long
    } else {
        linelen
    }) as size_t;
    max_idx = line_length.wrapping_div(MIN_COLUMN_WIDTH as libc::c_int as libc::c_ulong);
    max_idx = (max_idx as libc::c_ulong)
        .wrapping_add(
            (line_length.wrapping_rem(MIN_COLUMN_WIDTH as libc::c_int as libc::c_ulong)
                != 0 as libc::c_int as libc::c_ulong) as libc::c_int as libc::c_ulong,
        ) as size_t as size_t;
    if format as libc::c_uint == many_per_line as libc::c_int as libc::c_uint
        || format as libc::c_uint == horizontal as libc::c_int as libc::c_uint
        || format as libc::c_uint == with_commas as libc::c_int as libc::c_uint
    {
        if 0 as libc::c_int as libc::c_long <= tabsize_opt {
            tabsize = tabsize_opt as size_t;
        } else {
            tabsize = 8 as libc::c_int as size_t;
            let mut p_0: *const libc::c_char = getenv(
                b"TABSIZE\0" as *const u8 as *const libc::c_char,
            );
            if !p_0.is_null() {
                let mut tmp: uintmax_t = 0;
                if xstrtoumax(
                    p_0,
                    0 as *mut *mut libc::c_char,
                    0 as libc::c_int,
                    &mut tmp,
                    b"\0" as *const u8 as *const libc::c_char,
                ) as libc::c_uint == LONGINT_OK as libc::c_int as libc::c_uint
                    && tmp <= 18446744073709551615 as libc::c_ulong
                {
                    tabsize = tmp;
                } else {
                    error(
                        0 as libc::c_int,
                        0 as libc::c_int,
                        dcgettext(
                            0 as *const libc::c_char,
                            b"ignoring invalid tab size in environment variable TABSIZE: %s\0"
                                as *const u8 as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                        quote(p_0),
                    );
                }
            }
        }
    }
    qmark_funny_chars = if hide_control_chars_opt < 0 as libc::c_int {
        (ls_mode == 1 as libc::c_int && stdout_isatty() as libc::c_int != 0)
            as libc::c_int
    } else {
        hide_control_chars_opt
    } != 0;
    let mut qs: libc::c_int = quoting_style_opt;
    if qs < 0 as libc::c_int {
        qs = getenv_quoting_style();
    }
    if qs < 0 as libc::c_int {
        qs = if ls_mode == 1 as libc::c_int {
            if stdout_isatty() as libc::c_int != 0 {
                shell_escape_quoting_style as libc::c_int
            } else {
                -(1 as libc::c_int)
            }
        } else {
            escape_quoting_style as libc::c_int
        };
    }
    if 0 as libc::c_int <= qs {
        set_quoting_style(0 as *mut quoting_options, qs as quoting_style);
    }
    qs = get_quoting_style(0 as *const quoting_options) as libc::c_int;
    align_variable_outer_quotes = (format as libc::c_uint
        == long_format as libc::c_int as libc::c_uint
        || (format as libc::c_uint == many_per_line as libc::c_int as libc::c_uint
            || format as libc::c_uint == horizontal as libc::c_int as libc::c_uint)
            && line_length != 0)
        && (qs == shell_quoting_style as libc::c_int
            || qs == shell_escape_quoting_style as libc::c_int
            || qs == c_maybe_quoting_style as libc::c_int);
    filename_quoting_options = clone_quoting_options(0 as *mut quoting_options);
    if qs == escape_quoting_style as libc::c_int {
        set_char_quoting(
            filename_quoting_options,
            ' ' as i32 as libc::c_char,
            1 as libc::c_int,
        );
    }
    if file_type as libc::c_int as libc::c_uint <= indicator_style as libc::c_uint {
        let mut p_1: *const libc::c_char = 0 as *const libc::c_char;
        p_1 = &*(b"*=>@|\0" as *const u8 as *const libc::c_char)
            .offset(
                (indicator_style as libc::c_uint)
                    .wrapping_sub(file_type as libc::c_int as libc::c_uint) as isize,
            ) as *const libc::c_char;
        while *p_1 != 0 {
            set_char_quoting(filename_quoting_options, *p_1, 1 as libc::c_int);
            p_1 = p_1.offset(1);
        }
    }
    dirname_quoting_options = clone_quoting_options(0 as *mut quoting_options);
    set_char_quoting(
        dirname_quoting_options,
        ':' as i32 as libc::c_char,
        1 as libc::c_int,
    );
    dired = (dired as libc::c_int
        & ((format as libc::c_uint == long_format as libc::c_int as libc::c_uint)
            as libc::c_int & !print_hyperlink as libc::c_int)) as bool;
    if (eolbyte as libc::c_int) < dired as libc::c_int {
        if ::core::mem::size_of::<C2RustUnnamed_22>() as libc::c_ulong != 0 {
            error(
                LS_FAILURE as libc::c_int,
                0 as libc::c_int,
                dcgettext(
                    0 as *const libc::c_char,
                    b"--dired and --zero are incompatible\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
            );
            if 0 as libc::c_int != 0 {} else {
                unreachable!();
            };
        } else {
            error(
                LS_FAILURE as libc::c_int,
                0 as libc::c_int,
                dcgettext(
                    0 as *const libc::c_char,
                    b"--dired and --zero are incompatible\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
            );
            if 0 as libc::c_int != 0 {} else {
                unreachable!();
            };
        };
    }
    sort_type = (if 0 as libc::c_int <= sort_opt {
        sort_opt
    } else if format as libc::c_uint != long_format as libc::c_int as libc::c_uint
        && (time_type as libc::c_uint == time_ctime as libc::c_int as libc::c_uint
            || time_type as libc::c_uint == time_atime as libc::c_int as libc::c_uint
            || time_type as libc::c_uint == time_btime as libc::c_int as libc::c_uint)
    {
        sort_time as libc::c_int
    } else {
        sort_name as libc::c_int
    }) as sort_type;
    if format as libc::c_uint == long_format as libc::c_int as libc::c_uint {
        let mut style: *mut libc::c_char = time_style_option;
        static mut posix_prefix: [libc::c_char; 7] = unsafe {
            *::core::mem::transmute::<&[u8; 7], &[libc::c_char; 7]>(b"posix-\0")
        };
        if style.is_null() {
            style = getenv(b"TIME_STYLE\0" as *const u8 as *const libc::c_char);
            if style.is_null() {
                style = bad_cast(b"locale\0" as *const u8 as *const libc::c_char);
            }
        }
        while strncmp(
            style,
            posix_prefix.as_ptr(),
            (::core::mem::size_of::<[libc::c_char; 7]>() as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong),
        ) == 0 as libc::c_int
        {
            if !hard_locale(2 as libc::c_int) {
                return optind;
            }
            style = style
                .offset(
                    (::core::mem::size_of::<[libc::c_char; 7]>() as libc::c_ulong)
                        .wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
                );
        }
        if *style as libc::c_int == '+' as i32 {
            let mut p0: *mut libc::c_char = style.offset(1 as libc::c_int as isize);
            let mut p1: *mut libc::c_char = strchr(p0, '\n' as i32);
            if p1.is_null() {
                p1 = p0;
            } else {
                if !(strchr(p1.offset(1 as libc::c_int as isize), '\n' as i32)).is_null()
                {
                    if ::core::mem::size_of::<C2RustUnnamed_21>() as libc::c_ulong != 0 {
                        error(
                            LS_FAILURE as libc::c_int,
                            0 as libc::c_int,
                            dcgettext(
                                0 as *const libc::c_char,
                                b"invalid time style format %s\0" as *const u8
                                    as *const libc::c_char,
                                5 as libc::c_int,
                            ),
                            quote(p0),
                        );
                        if 0 as libc::c_int != 0 {} else {
                            unreachable!();
                        };
                    } else {
                        error(
                            LS_FAILURE as libc::c_int,
                            0 as libc::c_int,
                            dcgettext(
                                0 as *const libc::c_char,
                                b"invalid time style format %s\0" as *const u8
                                    as *const libc::c_char,
                                5 as libc::c_int,
                            ),
                            quote(p0),
                        );
                        if 0 as libc::c_int != 0 {} else {
                            unreachable!();
                        };
                    };
                }
                let fresh4 = p1;
                p1 = p1.offset(1);
                *fresh4 = '\0' as i32 as libc::c_char;
            }
            long_time_format[0 as libc::c_int as usize] = p0;
            long_time_format[1 as libc::c_int as usize] = p1;
        } else {
            let mut res: ptrdiff_t = argmatch(
                style,
                time_style_args.as_ptr(),
                time_style_types.as_ptr() as *const libc::c_char as *const libc::c_void,
                ::core::mem::size_of::<time_style>() as libc::c_ulong,
            );
            if res < 0 as libc::c_int as libc::c_long {
                argmatch_invalid(
                    b"time style\0" as *const u8 as *const libc::c_char,
                    style,
                    res,
                );
                fputs_unlocked(
                    dcgettext(
                        0 as *const libc::c_char,
                        b"Valid arguments are:\n\0" as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    stderr,
                );
                let mut p_2: *const *const libc::c_char = time_style_args.as_ptr();
                while !(*p_2).is_null() {
                    let fresh5 = p_2;
                    p_2 = p_2.offset(1);
                    fprintf(
                        stderr,
                        b"  - [posix-]%s\n\0" as *const u8 as *const libc::c_char,
                        *fresh5,
                    );
                }
                fputs_unlocked(
                    dcgettext(
                        0 as *const libc::c_char,
                        b"  - +FORMAT (e.g., +%H:%M) for a 'date'-style format\n\0"
                            as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    stderr,
                );
                usage(LS_FAILURE as libc::c_int);
            }
            match res {
                0 => {
                    long_time_format[1 as libc::c_int
                        as usize] = b"%Y-%m-%d %H:%M:%S.%N %z\0" as *const u8
                        as *const libc::c_char;
                    long_time_format[0 as libc::c_int
                        as usize] = long_time_format[1 as libc::c_int as usize];
                }
                1 => {
                    long_time_format[1 as libc::c_int
                        as usize] = b"%Y-%m-%d %H:%M\0" as *const u8
                        as *const libc::c_char;
                    long_time_format[0 as libc::c_int
                        as usize] = long_time_format[1 as libc::c_int as usize];
                }
                2 => {
                    long_time_format[0 as libc::c_int
                        as usize] = b"%Y-%m-%d \0" as *const u8 as *const libc::c_char;
                    long_time_format[1 as libc::c_int
                        as usize] = b"%m-%d %H:%M\0" as *const u8 as *const libc::c_char;
                }
                3 => {
                    if hard_locale(2 as libc::c_int) {
                        let mut i_2: libc::c_int = 0 as libc::c_int;
                        while i_2 < 2 as libc::c_int {
                            long_time_format[i_2
                                as usize] = dcgettext(
                                0 as *const libc::c_char,
                                long_time_format[i_2 as usize],
                                2 as libc::c_int,
                            );
                            i_2 += 1;
                        }
                    }
                }
                _ => {}
            }
        }
        abformat_init();
    }
    return optind;
}
unsafe extern "C" fn get_funky_string(
    mut dest: *mut *mut libc::c_char,
    mut src: *mut *const libc::c_char,
    mut equals_end: bool,
    mut output_count: *mut size_t,
) -> bool {
    let mut num: libc::c_char = 0;
    let mut count: size_t = 0;
    let mut state: C2RustUnnamed_25 = ST_GND;
    let mut p: *const libc::c_char = 0 as *const libc::c_char;
    let mut q: *mut libc::c_char = 0 as *mut libc::c_char;
    p = *src;
    q = *dest;
    count = 0 as libc::c_int as size_t;
    num = 0 as libc::c_int as libc::c_char;
    state = ST_GND;
    while (state as libc::c_uint) < ST_END as libc::c_int as libc::c_uint {
        match state as libc::c_uint {
            0 => {
                let mut current_block_13: u64;
                match *p as libc::c_int {
                    58 | 0 => {
                        state = ST_END;
                        current_block_13 = 8457315219000651999;
                    }
                    92 => {
                        state = ST_BACKSLASH;
                        p = p.offset(1);
                        current_block_13 = 8457315219000651999;
                    }
                    94 => {
                        state = ST_CARET;
                        p = p.offset(1);
                        current_block_13 = 8457315219000651999;
                    }
                    61 => {
                        if equals_end {
                            state = ST_END;
                            current_block_13 = 8457315219000651999;
                        } else {
                            current_block_13 = 115012762944750726;
                        }
                    }
                    _ => {
                        current_block_13 = 115012762944750726;
                    }
                }
                match current_block_13 {
                    115012762944750726 => {
                        let fresh6 = p;
                        p = p.offset(1);
                        let fresh7 = q;
                        q = q.offset(1);
                        *fresh7 = *fresh6;
                        count = count.wrapping_add(1);
                    }
                    _ => {}
                }
            }
            1 => {
                match *p as libc::c_int {
                    48 | 49 | 50 | 51 | 52 | 53 | 54 | 55 => {
                        state = ST_OCTAL;
                        num = (*p as libc::c_int - '0' as i32) as libc::c_char;
                    }
                    120 | 88 => {
                        state = ST_HEX;
                        num = 0 as libc::c_int as libc::c_char;
                    }
                    97 => {
                        num = '\u{7}' as i32 as libc::c_char;
                    }
                    98 => {
                        num = '\u{8}' as i32 as libc::c_char;
                    }
                    101 => {
                        num = 27 as libc::c_int as libc::c_char;
                    }
                    102 => {
                        num = '\u{c}' as i32 as libc::c_char;
                    }
                    110 => {
                        num = '\n' as i32 as libc::c_char;
                    }
                    114 => {
                        num = '\r' as i32 as libc::c_char;
                    }
                    116 => {
                        num = '\t' as i32 as libc::c_char;
                    }
                    118 => {
                        num = '\u{b}' as i32 as libc::c_char;
                    }
                    63 => {
                        num = 127 as libc::c_int as libc::c_char;
                    }
                    95 => {
                        num = ' ' as i32 as libc::c_char;
                    }
                    0 => {
                        state = ST_ERROR;
                    }
                    _ => {
                        num = *p;
                    }
                }
                if state as libc::c_uint == ST_BACKSLASH as libc::c_int as libc::c_uint {
                    let fresh8 = q;
                    q = q.offset(1);
                    *fresh8 = num;
                    count = count.wrapping_add(1);
                    state = ST_GND;
                }
                p = p.offset(1);
            }
            2 => {
                if (*p as libc::c_int) < '0' as i32 || *p as libc::c_int > '7' as i32 {
                    let fresh9 = q;
                    q = q.offset(1);
                    *fresh9 = num;
                    count = count.wrapping_add(1);
                    state = ST_GND;
                } else {
                    let fresh10 = p;
                    p = p.offset(1);
                    num = (((num as libc::c_int) << 3 as libc::c_int)
                        + (*fresh10 as libc::c_int - '0' as i32)) as libc::c_char;
                }
            }
            3 => {
                match *p as libc::c_int {
                    48 | 49 | 50 | 51 | 52 | 53 | 54 | 55 | 56 | 57 => {
                        let fresh11 = p;
                        p = p.offset(1);
                        num = (((num as libc::c_int) << 4 as libc::c_int)
                            + (*fresh11 as libc::c_int - '0' as i32)) as libc::c_char;
                    }
                    97 | 98 | 99 | 100 | 101 | 102 => {
                        let fresh12 = p;
                        p = p.offset(1);
                        num = (((num as libc::c_int) << 4 as libc::c_int)
                            + (*fresh12 as libc::c_int - 'a' as i32) + 10 as libc::c_int)
                            as libc::c_char;
                    }
                    65 | 66 | 67 | 68 | 69 | 70 => {
                        let fresh13 = p;
                        p = p.offset(1);
                        num = (((num as libc::c_int) << 4 as libc::c_int)
                            + (*fresh13 as libc::c_int - 'A' as i32) + 10 as libc::c_int)
                            as libc::c_char;
                    }
                    _ => {
                        let fresh14 = q;
                        q = q.offset(1);
                        *fresh14 = num;
                        count = count.wrapping_add(1);
                        state = ST_GND;
                    }
                }
            }
            4 => {
                state = ST_GND;
                if *p as libc::c_int >= '@' as i32 && *p as libc::c_int <= '~' as i32 {
                    let fresh15 = p;
                    p = p.offset(1);
                    let fresh16 = q;
                    q = q.offset(1);
                    *fresh16 = (*fresh15 as libc::c_int & 0o37 as libc::c_int)
                        as libc::c_char;
                    count = count.wrapping_add(1);
                } else if *p as libc::c_int == '?' as i32 {
                    let fresh17 = q;
                    q = q.offset(1);
                    *fresh17 = 127 as libc::c_int as libc::c_char;
                    count = count.wrapping_add(1);
                } else {
                    state = ST_ERROR;
                }
            }
            _ => {
                abort();
            }
        }
    }
    *dest = q;
    *src = p;
    *output_count = count;
    return state as libc::c_uint != ST_ERROR as libc::c_int as libc::c_uint;
}
unsafe extern "C" fn known_term_type() -> bool {
    let mut term: *const libc::c_char = getenv(
        b"TERM\0" as *const u8 as *const libc::c_char,
    );
    if term.is_null() || *term == 0 {
        return 0 as libc::c_int != 0;
    }
    let mut line: *const libc::c_char = G_line.as_ptr();
    while (line.offset_from(G_line.as_ptr()) as libc::c_long as libc::c_ulong)
        < ::core::mem::size_of::<[libc::c_char; 4808]>() as libc::c_ulong
    {
        if strncmp(
            line,
            b"TERM \0" as *const u8 as *const libc::c_char,
            (::core::mem::size_of::<[libc::c_char; 6]>() as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong),
        ) == 0 as libc::c_int
        {
            if fnmatch(line.offset(5 as libc::c_int as isize), term, 0 as libc::c_int)
                == 0 as libc::c_int
            {
                return 1 as libc::c_int != 0;
            }
        }
        line = line
            .offset(
                (strlen(line)).wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
            );
    }
    return 0 as libc::c_int != 0;
}
unsafe extern "C" fn parse_ls_color() {
    let mut p: *const libc::c_char = 0 as *const libc::c_char;
    let mut buf: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut ind_no: libc::c_int = 0;
    let mut label: [libc::c_char; 3] = [0; 3];
    let mut ext: *mut color_ext_type = 0 as *mut color_ext_type;
    p = getenv(b"LS_COLORS\0" as *const u8 as *const libc::c_char);
    if p.is_null() || *p as libc::c_int == '\0' as i32 {
        let mut colorterm: *const libc::c_char = getenv(
            b"COLORTERM\0" as *const u8 as *const libc::c_char,
        );
        if !(!colorterm.is_null() && *colorterm as libc::c_int != 0)
            && !known_term_type()
        {
            print_with_color = 0 as libc::c_int != 0;
        }
        return;
    }
    ext = 0 as *mut color_ext_type;
    strcpy(label.as_mut_ptr(), b"??\0" as *const u8 as *const libc::c_char);
    color_buf = xstrdup(p);
    buf = color_buf;
    let mut state: parse_state = PS_START;
    loop {
        match state as libc::c_uint {
            1 => {
                match *p as libc::c_int {
                    58 => {
                        p = p.offset(1);
                    }
                    42 => {
                        ext = xmalloc(
                            ::core::mem::size_of::<color_ext_type>() as libc::c_ulong,
                        ) as *mut color_ext_type;
                        (*ext).next = color_ext_list;
                        color_ext_list = ext;
                        p = p.offset(1);
                        (*ext).ext.string = buf;
                        state = (if get_funky_string(
                            &mut buf,
                            &mut p,
                            1 as libc::c_int != 0,
                            &mut (*ext).ext.len,
                        ) as libc::c_int != 0
                        {
                            PS_4 as libc::c_int
                        } else {
                            PS_FAIL as libc::c_int
                        }) as parse_state;
                    }
                    0 => {
                        state = PS_DONE;
                        break;
                    }
                    _ => {
                        let fresh18 = p;
                        p = p.offset(1);
                        label[0 as libc::c_int as usize] = *fresh18;
                        state = PS_2;
                    }
                }
            }
            2 => {
                if *p != 0 {
                    let fresh19 = p;
                    p = p.offset(1);
                    label[1 as libc::c_int as usize] = *fresh19;
                    state = PS_3;
                } else {
                    state = PS_FAIL;
                }
            }
            3 => {
                state = PS_FAIL;
                let fresh20 = p;
                p = p.offset(1);
                if *fresh20 as libc::c_int == '=' as i32 {
                    ind_no = 0 as libc::c_int;
                    while !(indicator_name[ind_no as usize]).is_null() {
                        if strcmp(label.as_mut_ptr(), indicator_name[ind_no as usize])
                            == 0 as libc::c_int
                        {
                            color_indicator[ind_no as usize].string = buf;
                            state = (if get_funky_string(
                                &mut buf,
                                &mut p,
                                0 as libc::c_int != 0,
                                &mut (*color_indicator.as_mut_ptr().offset(ind_no as isize))
                                    .len,
                            ) as libc::c_int != 0
                            {
                                PS_START as libc::c_int
                            } else {
                                PS_FAIL as libc::c_int
                            }) as parse_state;
                            break;
                        } else {
                            ind_no += 1;
                        }
                    }
                    if state as libc::c_uint == PS_FAIL as libc::c_int as libc::c_uint {
                        error(
                            0 as libc::c_int,
                            0 as libc::c_int,
                            dcgettext(
                                0 as *const libc::c_char,
                                b"unrecognized prefix: %s\0" as *const u8
                                    as *const libc::c_char,
                                5 as libc::c_int,
                            ),
                            quote(label.as_mut_ptr()),
                        );
                    }
                }
            }
            4 => {
                let fresh21 = p;
                p = p.offset(1);
                if *fresh21 as libc::c_int == '=' as i32 {
                    (*ext).seq.string = buf;
                    state = (if get_funky_string(
                        &mut buf,
                        &mut p,
                        0 as libc::c_int != 0,
                        &mut (*ext).seq.len,
                    ) as libc::c_int != 0
                    {
                        PS_START as libc::c_int
                    } else {
                        PS_FAIL as libc::c_int
                    }) as parse_state;
                } else {
                    state = PS_FAIL;
                }
            }
            6 => {
                break;
            }
            _ => {
                abort();
            }
        }
    }
    if state as libc::c_uint == PS_FAIL as libc::c_int as libc::c_uint {
        let mut e: *mut color_ext_type = 0 as *mut color_ext_type;
        let mut e2: *mut color_ext_type = 0 as *mut color_ext_type;
        error(
            0 as libc::c_int,
            0 as libc::c_int,
            dcgettext(
                0 as *const libc::c_char,
                b"unparsable value for LS_COLORS environment variable\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
        free(color_buf as *mut libc::c_void);
        e = color_ext_list;
        while !e.is_null() {
            e2 = e;
            e = (*e).next;
            free(e2 as *mut libc::c_void);
        }
        print_with_color = 0 as libc::c_int != 0;
    }
    if color_indicator[C_LINK as libc::c_int as usize].len
        == 6 as libc::c_int as libc::c_ulong
        && strncmp(
            color_indicator[C_LINK as libc::c_int as usize].string,
            b"target\0" as *const u8 as *const libc::c_char,
            (::core::mem::size_of::<[libc::c_char; 7]>() as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong),
        ) == 0
    {
        color_symlink_as_referent = 1 as libc::c_int != 0;
    }
}
unsafe extern "C" fn getenv_quoting_style() -> libc::c_int {
    let mut q_style: *const libc::c_char = getenv(
        b"QUOTING_STYLE\0" as *const u8 as *const libc::c_char,
    );
    if q_style.is_null() {
        return -(1 as libc::c_int);
    }
    let mut i: libc::c_int = argmatch(
        q_style,
        quoting_style_args.as_ptr(),
        quoting_style_vals.as_ptr() as *const libc::c_void,
        ::core::mem::size_of::<quoting_style>() as libc::c_ulong,
    ) as libc::c_int;
    if i < 0 as libc::c_int {
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
        return -(1 as libc::c_int);
    }
    return *quoting_style_vals.as_ptr().offset(i as isize) as libc::c_int;
}
unsafe extern "C" fn set_exit_status(mut serious: bool) {
    if serious {
        exit_status = LS_FAILURE as libc::c_int;
    } else if exit_status == 0 as libc::c_int {
        exit_status = LS_MINOR_PROBLEM as libc::c_int;
    }
}
unsafe extern "C" fn file_failure(
    mut serious: bool,
    mut message: *const libc::c_char,
    mut file: *const libc::c_char,
) {
    error(
        0 as libc::c_int,
        *__errno_location(),
        message,
        quotearg_style(shell_escape_always_quoting_style, file),
    );
    set_exit_status(serious);
}
unsafe extern "C" fn queue_directory(
    mut name: *const libc::c_char,
    mut realname: *const libc::c_char,
    mut command_line_arg: bool,
) {
    let mut new: *mut pending = xmalloc(
        ::core::mem::size_of::<pending>() as libc::c_ulong,
    ) as *mut pending;
    (*new)
        .realname = if !realname.is_null() {
        xstrdup(realname)
    } else {
        0 as *mut libc::c_char
    };
    (*new).name = if !name.is_null() { xstrdup(name) } else { 0 as *mut libc::c_char };
    (*new).command_line_arg = command_line_arg;
    (*new).next = pending_dirs;
    pending_dirs = new;
}
unsafe extern "C" fn print_dir(
    mut name: *const libc::c_char,
    mut realname: *const libc::c_char,
    mut command_line_arg: bool,
) {
    let mut dirp: *mut DIR = 0 as *mut DIR;
    let mut next: *mut dirent = 0 as *mut dirent;
    let mut total_blocks: uintmax_t = 0 as libc::c_int as uintmax_t;
    static mut first: bool = 1 as libc::c_int != 0;
    *__errno_location() = 0 as libc::c_int;
    dirp = opendir(name);
    if dirp.is_null() {
        file_failure(
            command_line_arg,
            dcgettext(
                0 as *const libc::c_char,
                b"cannot open directory %s\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            name,
        );
        return;
    }
    if !active_dir_set.is_null() {
        let mut dir_stat: stat = stat {
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
        let mut fd: libc::c_int = dirfd(dirp);
        if (if 0 as libc::c_int <= fd {
            fstat_for_ino(fd, &mut dir_stat)
        } else {
            stat_for_ino(name, &mut dir_stat)
        }) < 0 as libc::c_int
        {
            file_failure(
                command_line_arg,
                dcgettext(
                    0 as *const libc::c_char,
                    b"cannot determine device and inode of %s\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
                name,
            );
            closedir(dirp);
            return;
        }
        if visit_dir(dir_stat.st_dev, dir_stat.st_ino) {
            error(
                0 as libc::c_int,
                0 as libc::c_int,
                dcgettext(
                    0 as *const libc::c_char,
                    b"%s: not listing already-listed directory\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
                quotearg_n_style_colon(
                    0 as libc::c_int,
                    shell_escape_quoting_style,
                    name,
                ),
            );
            closedir(dirp);
            set_exit_status(1 as libc::c_int != 0);
            return;
        }
        dev_ino_push(dir_stat.st_dev, dir_stat.st_ino);
    }
    clear_files();
    if recursive as libc::c_int != 0 || print_dir_name as libc::c_int != 0 {
        if !first {
            dired_outbyte('\n' as i32 as libc::c_char);
        }
        first = 0 as libc::c_int != 0;
        dired_indent();
        let mut absolute_name: *mut libc::c_char = 0 as *mut libc::c_char;
        if print_hyperlink {
            absolute_name = canonicalize_filename_mode(name, CAN_MISSING);
            if absolute_name.is_null() {
                file_failure(
                    command_line_arg,
                    dcgettext(
                        0 as *const libc::c_char,
                        b"error canonicalizing %s\0" as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    name,
                );
            }
        }
        quote_name(
            if !realname.is_null() { realname } else { name },
            dirname_quoting_options,
            -(1 as libc::c_int),
            0 as *const bin_str,
            1 as libc::c_int != 0,
            &mut subdired_obstack,
            absolute_name,
        );
        free(absolute_name as *mut libc::c_void);
        dired_outstring(b":\n\0" as *const u8 as *const libc::c_char);
    }
    loop {
        *__errno_location() = 0 as libc::c_int;
        next = readdir(dirp);
        if !next.is_null() {
            if !file_ignored(((*next).d_name).as_mut_ptr()) {
                let mut type_0: filetype = unknown;
                match (*next).d_type as libc::c_int {
                    6 => {
                        type_0 = blockdev;
                    }
                    2 => {
                        type_0 = chardev;
                    }
                    4 => {
                        type_0 = directory;
                    }
                    1 => {
                        type_0 = fifo;
                    }
                    10 => {
                        type_0 = symbolic_link;
                    }
                    8 => {
                        type_0 = normal;
                    }
                    12 => {
                        type_0 = sock;
                    }
                    14 => {
                        type_0 = whiteout;
                    }
                    _ => {}
                }
                total_blocks = (total_blocks as libc::c_ulong)
                    .wrapping_add(
                        gobble_file(
                            ((*next).d_name).as_mut_ptr(),
                            type_0,
                            NOT_AN_INODE_NUMBER as libc::c_int as ino_t,
                            0 as libc::c_int != 0,
                            name,
                        ),
                    ) as uintmax_t as uintmax_t;
                if format as libc::c_uint == one_per_line as libc::c_int as libc::c_uint
                    && sort_type as libc::c_uint
                        == sort_none as libc::c_int as libc::c_uint && !print_block_size
                    && !recursive
                {
                    sort_files();
                    print_current_files();
                    clear_files();
                }
            }
        } else {
            if !(*__errno_location() != 0 as libc::c_int) {
                break;
            }
            file_failure(
                command_line_arg,
                dcgettext(
                    0 as *const libc::c_char,
                    b"reading directory %s\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
                name,
            );
            if *__errno_location() != 75 as libc::c_int {
                break;
            }
        }
        process_signals();
    }
    if closedir(dirp) != 0 as libc::c_int {
        file_failure(
            command_line_arg,
            dcgettext(
                0 as *const libc::c_char,
                b"closing directory %s\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            name,
        );
    }
    sort_files();
    if recursive {
        extract_dirs_from_files(name, 0 as libc::c_int != 0);
    }
    if format as libc::c_uint == long_format as libc::c_int as libc::c_uint
        || print_block_size as libc::c_int != 0
    {
        let mut buf: [libc::c_char; 654] = [0; 654];
        let mut p: *mut libc::c_char = human_readable(
            total_blocks,
            buf.as_mut_ptr().offset(1 as libc::c_int as isize),
            human_output_opts,
            512 as libc::c_int as uintmax_t,
            output_block_size,
        );
        let mut pend: *mut libc::c_char = p.offset(strlen(p) as isize);
        p = p.offset(-1);
        *p = ' ' as i32 as libc::c_char;
        let fresh22 = pend;
        pend = pend.offset(1);
        *fresh22 = eolbyte;
        dired_indent();
        dired_outstring(
            dcgettext(
                0 as *const libc::c_char,
                b"total\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
        dired_outbuf(p, pend.offset_from(p) as libc::c_long as size_t);
    }
    if cwd_n_used != 0 {
        print_current_files();
    }
}
unsafe extern "C" fn add_ignore_pattern(mut pattern: *const libc::c_char) {
    let mut ignore: *mut ignore_pattern = 0 as *mut ignore_pattern;
    ignore = xmalloc(::core::mem::size_of::<ignore_pattern>() as libc::c_ulong)
        as *mut ignore_pattern;
    (*ignore).pattern = pattern;
    (*ignore).next = ignore_patterns;
    ignore_patterns = ignore;
}
unsafe extern "C" fn patterns_match(
    mut patterns: *const ignore_pattern,
    mut file: *const libc::c_char,
) -> bool {
    let mut p: *const ignore_pattern = 0 as *const ignore_pattern;
    p = patterns;
    while !p.is_null() {
        if fnmatch((*p).pattern, file, (1 as libc::c_int) << 2 as libc::c_int)
            == 0 as libc::c_int
        {
            return 1 as libc::c_int != 0;
        }
        p = (*p).next;
    }
    return 0 as libc::c_int != 0;
}
unsafe extern "C" fn file_ignored(mut name: *const libc::c_char) -> bool {
    return ignore_mode as libc::c_uint != IGNORE_MINIMAL as libc::c_int as libc::c_uint
        && *name.offset(0 as libc::c_int as isize) as libc::c_int == '.' as i32
        && (ignore_mode as libc::c_uint == IGNORE_DEFAULT as libc::c_int as libc::c_uint
            || *name
                .offset(
                    (1 as libc::c_int
                        + (*name.offset(1 as libc::c_int as isize) as libc::c_int
                            == '.' as i32) as libc::c_int) as isize,
                ) == 0)
        || ignore_mode as libc::c_uint == IGNORE_DEFAULT as libc::c_int as libc::c_uint
            && patterns_match(hide_patterns, name) as libc::c_int != 0
        || patterns_match(ignore_patterns, name) as libc::c_int != 0;
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
unsafe extern "C" fn has_capability(mut name: *const libc::c_char) -> bool {
    *__errno_location() = 95 as libc::c_int;
    return 0 as libc::c_int != 0;
}
unsafe extern "C" fn free_ent(mut f: *mut fileinfo) {
    free((*f).name as *mut libc::c_void);
    free((*f).linkname as *mut libc::c_void);
    free((*f).absolute_name as *mut libc::c_void);
    if (*f).scontext != UNKNOWN_SECURITY_CONTEXT.as_mut_ptr() {
        if is_smack_enabled() {
            free((*f).scontext as *mut libc::c_void);
        } else {
            freecon((*f).scontext);
        }
    }
}
unsafe extern "C" fn clear_files() {
    let mut i: size_t = 0 as libc::c_int as size_t;
    while i < cwd_n_used {
        let mut f: *mut fileinfo = *sorted_file.offset(i as isize) as *mut fileinfo;
        free_ent(f);
        i = i.wrapping_add(1);
    }
    cwd_n_used = 0 as libc::c_int as size_t;
    cwd_some_quoted = 0 as libc::c_int != 0;
    any_has_acl = 0 as libc::c_int != 0;
    inode_number_width = 0 as libc::c_int;
    block_size_width = 0 as libc::c_int;
    nlink_width = 0 as libc::c_int;
    owner_width = 0 as libc::c_int;
    group_width = 0 as libc::c_int;
    author_width = 0 as libc::c_int;
    scontext_width = 0 as libc::c_int;
    major_device_number_width = 0 as libc::c_int;
    minor_device_number_width = 0 as libc::c_int;
    file_size_width = 0 as libc::c_int;
}
unsafe extern "C" fn errno_unsupported(mut err: libc::c_int) -> bool {
    return err == 22 as libc::c_int || err == 38 as libc::c_int
        || is_ENOTSUP(err) as libc::c_int != 0;
}
unsafe extern "C" fn getfilecon_cache(
    mut file: *const libc::c_char,
    mut f: *mut fileinfo,
    mut deref: bool,
) -> libc::c_int {
    static mut unsupported_device: dev_t = 0;
    if (*f).stat.st_dev == unsupported_device {
        *__errno_location() = 95 as libc::c_int;
        return -(1 as libc::c_int);
    }
    let mut r: libc::c_int = 0 as libc::c_int;
    r = if deref as libc::c_int != 0 {
        getfilecon(file, &mut (*f).scontext)
    } else {
        lgetfilecon(file, &mut (*f).scontext)
    };
    if r < 0 as libc::c_int && errno_unsupported(*__errno_location()) as libc::c_int != 0
    {
        unsupported_device = (*f).stat.st_dev;
    }
    return r;
}
unsafe extern "C" fn file_has_acl_cache(
    mut file: *const libc::c_char,
    mut f: *mut fileinfo,
) -> libc::c_int {
    static mut unsupported_device: dev_t = 0;
    if (*f).stat.st_dev == unsupported_device {
        *__errno_location() = 95 as libc::c_int;
        return 0 as libc::c_int;
    }
    *__errno_location() = 0 as libc::c_int;
    let mut n: libc::c_int = file_has_acl(file, &mut (*f).stat);
    if n <= 0 as libc::c_int
        && errno_unsupported(*__errno_location()) as libc::c_int != 0
    {
        unsupported_device = (*f).stat.st_dev;
    }
    return n;
}
unsafe extern "C" fn has_capability_cache(
    mut file: *const libc::c_char,
    mut f: *mut fileinfo,
) -> bool {
    static mut unsupported_device: dev_t = 0;
    if (*f).stat.st_dev == unsupported_device {
        *__errno_location() = 95 as libc::c_int;
        return 0 as libc::c_int != 0;
    }
    let mut b: bool = has_capability(file);
    if !b && errno_unsupported(*__errno_location()) as libc::c_int != 0 {
        unsupported_device = (*f).stat.st_dev;
    }
    return b;
}
unsafe extern "C" fn needs_quoting(mut name: *const libc::c_char) -> bool {
    let mut test: [libc::c_char; 2] = [0; 2];
    let mut len: size_t = quotearg_buffer(
        test.as_mut_ptr(),
        ::core::mem::size_of::<[libc::c_char; 2]>() as libc::c_ulong,
        name,
        -(1 as libc::c_int) as size_t,
        filename_quoting_options,
    );
    return *name as libc::c_int != *test.as_mut_ptr() as libc::c_int
        || strlen(name) != len;
}
unsafe extern "C" fn gobble_file(
    mut name: *const libc::c_char,
    mut type_0: filetype,
    mut inode: ino_t,
    mut command_line_arg: bool,
    mut dirname: *const libc::c_char,
) -> uintmax_t {
    let mut blocks: uintmax_t = 0 as libc::c_int as uintmax_t;
    let mut f: *mut fileinfo = 0 as *mut fileinfo;
    if !command_line_arg || inode == NOT_AN_INODE_NUMBER as libc::c_int as libc::c_ulong
    {} else {
        __assert_fail(
            b"! command_line_arg || inode == NOT_AN_INODE_NUMBER\0" as *const u8
                as *const libc::c_char,
            b"src/ls.c\0" as *const u8 as *const libc::c_char,
            3337 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 79],
                &[libc::c_char; 79],
            >(
                b"uintmax_t gobble_file(const char *, enum filetype, ino_t, _Bool, const char *)\0",
            ))
                .as_ptr(),
        );
    }
    if cwd_n_used == cwd_n_alloc {
        cwd_file = xnrealloc(
            cwd_file as *mut libc::c_void,
            cwd_n_alloc,
            (2 as libc::c_int as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<fileinfo>() as libc::c_ulong),
        ) as *mut fileinfo;
        cwd_n_alloc = (cwd_n_alloc as libc::c_ulong)
            .wrapping_mul(2 as libc::c_int as libc::c_ulong) as size_t as size_t;
    }
    f = &mut *cwd_file.offset(cwd_n_used as isize) as *mut fileinfo;
    memset(
        f as *mut libc::c_void,
        '\0' as i32,
        ::core::mem::size_of::<fileinfo>() as libc::c_ulong,
    );
    (*f).stat.st_ino = inode;
    (*f).filetype = type_0;
    (*f).quoted = -(1 as libc::c_int);
    if !cwd_some_quoted && align_variable_outer_quotes as libc::c_int != 0 {
        (*f).quoted = needs_quoting(name) as libc::c_int;
        if (*f).quoted != 0 {
            cwd_some_quoted = 1 as libc::c_int != 0;
        }
    }
    if command_line_arg as libc::c_int != 0 || print_hyperlink as libc::c_int != 0
        || format_needs_stat as libc::c_int != 0
        || type_0 as libc::c_uint == directory as libc::c_int as libc::c_uint
            && print_with_color as libc::c_int != 0
            && (is_colored(C_OTHER_WRITABLE) as libc::c_int != 0
                || is_colored(C_STICKY) as libc::c_int != 0
                || is_colored(C_STICKY_OTHER_WRITABLE) as libc::c_int != 0)
        || (print_inode as libc::c_int != 0 || format_needs_type as libc::c_int != 0)
            && (type_0 as libc::c_uint == symbolic_link as libc::c_int as libc::c_uint
                || type_0 as libc::c_uint == unknown as libc::c_int as libc::c_uint)
            && (dereference as libc::c_uint
                == DEREF_ALWAYS as libc::c_int as libc::c_uint
                || color_symlink_as_referent as libc::c_int != 0
                || check_symlink_mode as libc::c_int != 0)
        || print_inode as libc::c_int != 0
            && inode == NOT_AN_INODE_NUMBER as libc::c_int as libc::c_ulong
        || format_needs_type as libc::c_int != 0
            && (type_0 as libc::c_uint == unknown as libc::c_int as libc::c_uint
                || command_line_arg as libc::c_int != 0
                || type_0 as libc::c_uint == normal as libc::c_int as libc::c_uint
                    && (indicator_style as libc::c_uint
                        == classify as libc::c_int as libc::c_uint
                        || print_with_color as libc::c_int != 0
                            && (is_colored(C_EXEC) as libc::c_int != 0
                                || is_colored(C_SETUID) as libc::c_int != 0
                                || is_colored(C_SETGID) as libc::c_int != 0
                                || is_colored(C_CAP) as libc::c_int != 0)))
    {
        let mut full_name: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut do_deref: bool = false;
        let mut err: libc::c_int = 0;
        if *name.offset(0 as libc::c_int as isize) as libc::c_int == '/' as i32
            || *dirname.offset(0 as libc::c_int as isize) as libc::c_int
                == 0 as libc::c_int
        {
            full_name = name as *mut libc::c_char;
        } else {
            let mut fresh23 = ::std::vec::from_elem(
                0,
                (strlen(name))
                    .wrapping_add(strlen(dirname))
                    .wrapping_add(2 as libc::c_int as libc::c_ulong) as usize,
            );
            full_name = fresh23.as_mut_ptr() as *mut libc::c_char;
            attach(full_name, dirname, name);
        }
        if print_hyperlink {
            (*f).absolute_name = canonicalize_filename_mode(full_name, CAN_MISSING);
            if ((*f).absolute_name).is_null() {
                file_failure(
                    command_line_arg,
                    dcgettext(
                        0 as *const libc::c_char,
                        b"error canonicalizing %s\0" as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    full_name,
                );
            }
        }
        let mut current_block_32: u64;
        match dereference as libc::c_uint {
            4 => {
                err = do_stat(full_name, &mut (*f).stat);
                do_deref = 1 as libc::c_int != 0;
                current_block_32 = 18377268871191777778;
            }
            2 | 3 => {
                if command_line_arg {
                    let mut need_lstat: bool = false;
                    err = do_stat(full_name, &mut (*f).stat);
                    do_deref = 1 as libc::c_int != 0;
                    if dereference as libc::c_uint
                        == DEREF_COMMAND_LINE_ARGUMENTS as libc::c_int as libc::c_uint
                    {
                        current_block_32 = 18377268871191777778;
                    } else {
                        need_lstat = if err < 0 as libc::c_int {
                            (*__errno_location() == 2 as libc::c_int) as libc::c_int
                        } else {
                            !((*f).stat.st_mode & 0o170000 as libc::c_int as libc::c_uint
                                == 0o40000 as libc::c_int as libc::c_uint) as libc::c_int
                        } != 0;
                        if !need_lstat {
                            current_block_32 = 18377268871191777778;
                        } else {
                            current_block_32 = 1693991182630120465;
                        }
                    }
                } else {
                    current_block_32 = 1693991182630120465;
                }
            }
            _ => {
                current_block_32 = 1693991182630120465;
            }
        }
        match current_block_32 {
            1693991182630120465 => {
                err = do_lstat(full_name, &mut (*f).stat);
                do_deref = 0 as libc::c_int != 0;
            }
            _ => {}
        }
        if err != 0 as libc::c_int {
            file_failure(
                command_line_arg,
                dcgettext(
                    0 as *const libc::c_char,
                    b"cannot access %s\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
                full_name,
            );
            (*f).scontext = UNKNOWN_SECURITY_CONTEXT.as_mut_ptr();
            if command_line_arg {
                return 0 as libc::c_int as uintmax_t;
            }
            (*f).name = xstrdup(name);
            cwd_n_used = cwd_n_used.wrapping_add(1);
            return 0 as libc::c_int as uintmax_t;
        }
        (*f).stat_ok = 1 as libc::c_int != 0;
        if (type_0 as libc::c_uint == normal as libc::c_int as libc::c_uint
            || (*f).stat.st_mode & 0o170000 as libc::c_int as libc::c_uint
                == 0o100000 as libc::c_int as libc::c_uint)
            && print_with_color as libc::c_int != 0
            && is_colored(C_CAP) as libc::c_int != 0
        {
            (*f).has_capability = has_capability_cache(full_name, f);
        }
        if format as libc::c_uint == long_format as libc::c_int as libc::c_uint
            || print_scontext as libc::c_int != 0
        {
            let mut have_scontext: bool = 0 as libc::c_int != 0;
            let mut have_acl: bool = 0 as libc::c_int != 0;
            let mut attr_len: libc::c_int = getfilecon_cache(full_name, f, do_deref);
            err = (attr_len < 0 as libc::c_int) as libc::c_int;
            if err == 0 as libc::c_int {
                if is_smack_enabled() {
                    have_scontext = !(strcmp(
                        b"_\0" as *const u8 as *const libc::c_char,
                        (*f).scontext,
                    ) == 0 as libc::c_int);
                } else {
                    have_scontext = !(strcmp(
                        b"unlabeled\0" as *const u8 as *const libc::c_char,
                        (*f).scontext,
                    ) == 0 as libc::c_int);
                }
            } else {
                (*f).scontext = UNKNOWN_SECURITY_CONTEXT.as_mut_ptr();
                if is_ENOTSUP(*__errno_location()) as libc::c_int != 0
                    || *__errno_location() == 61 as libc::c_int
                {
                    err = 0 as libc::c_int;
                }
            }
            if err == 0 as libc::c_int
                && format as libc::c_uint == long_format as libc::c_int as libc::c_uint
            {
                let mut n: libc::c_int = file_has_acl_cache(full_name, f);
                err = (n < 0 as libc::c_int) as libc::c_int;
                have_acl = (0 as libc::c_int) < n;
            }
            (*f)
                .acl_type = (if !have_scontext && !have_acl {
                ACL_T_NONE as libc::c_int
            } else if have_scontext as libc::c_int != 0 && !have_acl {
                ACL_T_LSM_CONTEXT_ONLY as libc::c_int
            } else {
                ACL_T_YES as libc::c_int
            }) as acl_type;
            any_has_acl = (any_has_acl as libc::c_int
                | ((*f).acl_type as libc::c_uint
                    != ACL_T_NONE as libc::c_int as libc::c_uint) as libc::c_int)
                as bool;
            if err != 0 {
                error(
                    0 as libc::c_int,
                    *__errno_location(),
                    b"%s\0" as *const u8 as *const libc::c_char,
                    quotearg_n_style_colon(
                        0 as libc::c_int,
                        shell_escape_quoting_style,
                        full_name,
                    ),
                );
            }
        }
        if (*f).stat.st_mode & 0o170000 as libc::c_int as libc::c_uint
            == 0o120000 as libc::c_int as libc::c_uint
            && (format as libc::c_uint == long_format as libc::c_int as libc::c_uint
                || check_symlink_mode as libc::c_int != 0)
        {
            let mut linkstats: stat = stat {
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
            get_link_name(full_name, f, command_line_arg);
            let mut linkname: *mut libc::c_char = make_link_name(
                full_name,
                (*f).linkname,
            );
            if !linkname.is_null() && (*f).quoted == 0 as libc::c_int
                && needs_quoting((*f).linkname) as libc::c_int != 0
            {
                (*f).quoted = -(1 as libc::c_int);
            }
            if !linkname.is_null()
                && (file_type as libc::c_int as libc::c_uint
                    <= indicator_style as libc::c_uint
                    || check_symlink_mode as libc::c_int != 0)
                && stat_for_mode(linkname, &mut linkstats) == 0 as libc::c_int
            {
                (*f).linkok = 1 as libc::c_int != 0;
                (*f).linkmode = linkstats.st_mode;
            }
            free(linkname as *mut libc::c_void);
        }
        if (*f).stat.st_mode & 0o170000 as libc::c_int as libc::c_uint
            == 0o120000 as libc::c_int as libc::c_uint
        {
            (*f).filetype = symbolic_link;
        } else if (*f).stat.st_mode & 0o170000 as libc::c_int as libc::c_uint
            == 0o40000 as libc::c_int as libc::c_uint
        {
            if command_line_arg as libc::c_int != 0 && !immediate_dirs {
                (*f).filetype = arg_directory;
            } else {
                (*f).filetype = directory;
            }
        } else {
            (*f).filetype = normal;
        }
        blocks = (*f).stat.st_blocks as uintmax_t;
        if format as libc::c_uint == long_format as libc::c_int as libc::c_uint
            || print_block_size as libc::c_int != 0
        {
            let mut buf: [libc::c_char; 652] = [0; 652];
            let mut len: libc::c_int = gnu_mbswidth(
                human_readable(
                    blocks,
                    buf.as_mut_ptr(),
                    human_output_opts,
                    512 as libc::c_int as uintmax_t,
                    output_block_size,
                ),
                0 as libc::c_int,
            );
            if block_size_width < len {
                block_size_width = len;
            }
        }
        if format as libc::c_uint == long_format as libc::c_int as libc::c_uint {
            if print_owner {
                let mut len_0: libc::c_int = format_user_width((*f).stat.st_uid);
                if owner_width < len_0 {
                    owner_width = len_0;
                }
            }
            if print_group {
                let mut len_1: libc::c_int = format_group_width((*f).stat.st_gid);
                if group_width < len_1 {
                    group_width = len_1;
                }
            }
            if print_author {
                let mut len_2: libc::c_int = format_user_width((*f).stat.st_uid);
                if author_width < len_2 {
                    author_width = len_2;
                }
            }
        }
        if print_scontext {
            let mut len_3: libc::c_int = strlen((*f).scontext) as libc::c_int;
            if scontext_width < len_3 {
                scontext_width = len_3;
            }
        }
        if format as libc::c_uint == long_format as libc::c_int as libc::c_uint {
            let mut b: [libc::c_char; 21] = [0; 21];
            let mut b_len: libc::c_int = strlen(
                umaxtostr((*f).stat.st_nlink as uintmax_t, b.as_mut_ptr()),
            ) as libc::c_int;
            if nlink_width < b_len {
                nlink_width = b_len;
            }
            if (*f).stat.st_mode & 0o170000 as libc::c_int as libc::c_uint
                == 0o20000 as libc::c_int as libc::c_uint
                || (*f).stat.st_mode & 0o170000 as libc::c_int as libc::c_uint
                    == 0o60000 as libc::c_int as libc::c_uint
            {
                let mut buf_0: [libc::c_char; 21] = [0; 21];
                let mut len_4: libc::c_int = strlen(
                    umaxtostr(
                        gnu_dev_major((*f).stat.st_rdev) as uintmax_t,
                        buf_0.as_mut_ptr(),
                    ),
                ) as libc::c_int;
                if major_device_number_width < len_4 {
                    major_device_number_width = len_4;
                }
                len_4 = strlen(
                    umaxtostr(
                        gnu_dev_minor((*f).stat.st_rdev) as uintmax_t,
                        buf_0.as_mut_ptr(),
                    ),
                ) as libc::c_int;
                if minor_device_number_width < len_4 {
                    minor_device_number_width = len_4;
                }
                len_4 = major_device_number_width + 2 as libc::c_int
                    + minor_device_number_width;
                if file_size_width < len_4 {
                    file_size_width = len_4;
                }
            } else {
                let mut buf_1: [libc::c_char; 652] = [0; 652];
                let mut size: uintmax_t = unsigned_file_size((*f).stat.st_size);
                let mut len_5: libc::c_int = gnu_mbswidth(
                    human_readable(
                        size,
                        buf_1.as_mut_ptr(),
                        file_human_output_opts,
                        1 as libc::c_int as uintmax_t,
                        file_output_block_size,
                    ),
                    0 as libc::c_int,
                );
                if file_size_width < len_5 {
                    file_size_width = len_5;
                }
            }
        }
    }
    if print_inode {
        let mut buf_2: [libc::c_char; 21] = [0; 21];
        let mut len_6: libc::c_int = strlen(
            umaxtostr((*f).stat.st_ino, buf_2.as_mut_ptr()),
        ) as libc::c_int;
        if inode_number_width < len_6 {
            inode_number_width = len_6;
        }
    }
    (*f).name = xstrdup(name);
    cwd_n_used = cwd_n_used.wrapping_add(1);
    return blocks;
}
unsafe extern "C" fn is_directory(mut f: *const fileinfo) -> bool {
    return (*f).filetype as libc::c_uint == directory as libc::c_int as libc::c_uint
        || (*f).filetype as libc::c_uint == arg_directory as libc::c_int as libc::c_uint;
}
unsafe extern "C" fn is_linked_directory(mut f: *const fileinfo) -> bool {
    return (*f).filetype as libc::c_uint == directory as libc::c_int as libc::c_uint
        || (*f).filetype as libc::c_uint == arg_directory as libc::c_int as libc::c_uint
        || (*f).linkmode & 0o170000 as libc::c_int as libc::c_uint
            == 0o40000 as libc::c_int as libc::c_uint;
}
unsafe extern "C" fn get_link_name(
    mut filename: *const libc::c_char,
    mut f: *mut fileinfo,
    mut command_line_arg: bool,
) {
    (*f).linkname = areadlink_with_size(filename, (*f).stat.st_size as size_t);
    if ((*f).linkname).is_null() {
        file_failure(
            command_line_arg,
            dcgettext(
                0 as *const libc::c_char,
                b"cannot read symbolic link %s\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            filename,
        );
    }
}
unsafe extern "C" fn make_link_name(
    mut name: *const libc::c_char,
    mut linkname: *const libc::c_char,
) -> *mut libc::c_char {
    if linkname.is_null() {
        return 0 as *mut libc::c_char;
    }
    if *linkname.offset(0 as libc::c_int as isize) as libc::c_int == '/' as i32 {
        return xstrdup(linkname);
    }
    let mut prefix_len: size_t = dir_len(name);
    if prefix_len == 0 as libc::c_int as libc::c_ulong {
        return xstrdup(linkname);
    }
    let mut p: *mut libc::c_char = xmalloc(
        prefix_len
            .wrapping_add(1 as libc::c_int as libc::c_ulong)
            .wrapping_add(strlen(linkname))
            .wrapping_add(1 as libc::c_int as libc::c_ulong),
    ) as *mut libc::c_char;
    if !(*name
        .offset(prefix_len.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize)
        as libc::c_int == '/' as i32)
    {
        prefix_len = prefix_len.wrapping_add(1);
    }
    stpcpy(stpncpy(p, name, prefix_len), linkname);
    return p;
}
unsafe extern "C" fn basename_is_dot_or_dotdot(mut name: *const libc::c_char) -> bool {
    let mut base: *const libc::c_char = last_component(name);
    return dot_or_dotdot(base);
}
unsafe extern "C" fn extract_dirs_from_files(
    mut dirname: *const libc::c_char,
    mut command_line_arg: bool,
) {
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    let mut ignore_dot_and_dot_dot: bool = !dirname.is_null();
    if !dirname.is_null() && !active_dir_set.is_null() {
        queue_directory(0 as *const libc::c_char, dirname, 0 as libc::c_int != 0);
    }
    i = cwd_n_used;
    loop {
        let fresh24 = i;
        i = i.wrapping_sub(1);
        if !(fresh24 != 0 as libc::c_int as libc::c_ulong) {
            break;
        }
        let mut f: *mut fileinfo = *sorted_file.offset(i as isize) as *mut fileinfo;
        if is_directory(f) as libc::c_int != 0
            && (!ignore_dot_and_dot_dot || !basename_is_dot_or_dotdot((*f).name))
        {
            if dirname.is_null()
                || *((*f).name).offset(0 as libc::c_int as isize) as libc::c_int
                    == '/' as i32
            {
                queue_directory((*f).name, (*f).linkname, command_line_arg);
            } else {
                let mut name: *mut libc::c_char = file_name_concat(
                    dirname,
                    (*f).name,
                    0 as *mut *mut libc::c_char,
                );
                queue_directory(name, (*f).linkname, command_line_arg);
                free(name as *mut libc::c_void);
            }
            if (*f).filetype as libc::c_uint
                == arg_directory as libc::c_int as libc::c_uint
            {
                free_ent(f);
            }
        }
    }
    i = 0 as libc::c_int as size_t;
    j = 0 as libc::c_int as size_t;
    while i < cwd_n_used {
        let mut f_0: *mut fileinfo = *sorted_file.offset(i as isize) as *mut fileinfo;
        let ref mut fresh25 = *sorted_file.offset(j as isize);
        *fresh25 = f_0 as *mut libc::c_void;
        j = (j as libc::c_ulong)
            .wrapping_add(
                ((*f_0).filetype as libc::c_uint
                    != arg_directory as libc::c_int as libc::c_uint) as libc::c_int
                    as libc::c_ulong,
            ) as size_t as size_t;
        i = i.wrapping_add(1);
    }
    cwd_n_used = j;
}
static mut failed_strcoll: jmp_buf = [__jmp_buf_tag {
    __jmpbuf: [0; 22],
    __mask_was_saved: 0,
    __saved_mask: sigset_t { __val: [0; 16] },
}; 1];
unsafe extern "C" fn xstrcoll(
    mut a: *const libc::c_char,
    mut b: *const libc::c_char,
) -> libc::c_int {
    let mut diff: libc::c_int = 0;
    *__errno_location() = 0 as libc::c_int;
    diff = strcoll(a, b);
    if *__errno_location() != 0 {
        error(
            0 as libc::c_int,
            *__errno_location(),
            dcgettext(
                0 as *const libc::c_char,
                b"cannot compare file names %s and %s\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
            quote_n(0 as libc::c_int, a),
            quote_n(1 as libc::c_int, b),
        );
        set_exit_status(0 as libc::c_int != 0);
        longjmp(failed_strcoll.as_mut_ptr(), 1 as libc::c_int);
    }
    return diff;
}
unsafe extern "C" fn dirfirst_check(
    mut a: *const fileinfo,
    mut b: *const fileinfo,
    mut cmp: Option::<unsafe extern "C" fn(V, V) -> libc::c_int>,
) -> libc::c_int {
    let mut diff: libc::c_int = is_linked_directory(b) as libc::c_int
        - is_linked_directory(a) as libc::c_int;
    return if diff != 0 {
        diff
    } else {
        cmp.expect("non-null function pointer")(a as V, b as V)
    };
}
unsafe extern "C" fn cmp_ctime(
    mut a: *const fileinfo,
    mut b: *const fileinfo,
    mut cmp: Option::<
        unsafe extern "C" fn(*const libc::c_char, *const libc::c_char) -> libc::c_int,
    >,
) -> libc::c_int {
    let mut diff: libc::c_int = timespec_cmp(
        get_stat_ctime(&(*b).stat),
        get_stat_ctime(&(*a).stat),
    );
    return if diff != 0 {
        diff
    } else {
        cmp.expect("non-null function pointer")((*a).name, (*b).name)
    };
}
unsafe extern "C" fn cmp_mtime(
    mut a: *const fileinfo,
    mut b: *const fileinfo,
    mut cmp: Option::<
        unsafe extern "C" fn(*const libc::c_char, *const libc::c_char) -> libc::c_int,
    >,
) -> libc::c_int {
    let mut diff: libc::c_int = timespec_cmp(
        get_stat_mtime(&(*b).stat),
        get_stat_mtime(&(*a).stat),
    );
    return if diff != 0 {
        diff
    } else {
        cmp.expect("non-null function pointer")((*a).name, (*b).name)
    };
}
unsafe extern "C" fn cmp_atime(
    mut a: *const fileinfo,
    mut b: *const fileinfo,
    mut cmp: Option::<
        unsafe extern "C" fn(*const libc::c_char, *const libc::c_char) -> libc::c_int,
    >,
) -> libc::c_int {
    let mut diff: libc::c_int = timespec_cmp(
        get_stat_atime(&(*b).stat),
        get_stat_atime(&(*a).stat),
    );
    return if diff != 0 {
        diff
    } else {
        cmp.expect("non-null function pointer")((*a).name, (*b).name)
    };
}
unsafe extern "C" fn cmp_btime(
    mut a: *const fileinfo,
    mut b: *const fileinfo,
    mut cmp: Option::<
        unsafe extern "C" fn(*const libc::c_char, *const libc::c_char) -> libc::c_int,
    >,
) -> libc::c_int {
    let mut diff: libc::c_int = timespec_cmp(
        get_stat_btime(&(*b).stat),
        get_stat_btime(&(*a).stat),
    );
    return if diff != 0 {
        diff
    } else {
        cmp.expect("non-null function pointer")((*a).name, (*b).name)
    };
}
unsafe extern "C" fn off_cmp(mut a: off_t, mut b: off_t) -> libc::c_int {
    return (a > b) as libc::c_int - (a < b) as libc::c_int;
}
unsafe extern "C" fn cmp_size(
    mut a: *const fileinfo,
    mut b: *const fileinfo,
    mut cmp: Option::<
        unsafe extern "C" fn(*const libc::c_char, *const libc::c_char) -> libc::c_int,
    >,
) -> libc::c_int {
    let mut diff: libc::c_int = off_cmp((*b).stat.st_size, (*a).stat.st_size);
    return if diff != 0 {
        diff
    } else {
        cmp.expect("non-null function pointer")((*a).name, (*b).name)
    };
}
unsafe extern "C" fn cmp_name(
    mut a: *const fileinfo,
    mut b: *const fileinfo,
    mut cmp: Option::<
        unsafe extern "C" fn(*const libc::c_char, *const libc::c_char) -> libc::c_int,
    >,
) -> libc::c_int {
    return cmp.expect("non-null function pointer")((*a).name, (*b).name);
}
unsafe extern "C" fn cmp_extension(
    mut a: *const fileinfo,
    mut b: *const fileinfo,
    mut cmp: Option::<
        unsafe extern "C" fn(*const libc::c_char, *const libc::c_char) -> libc::c_int,
    >,
) -> libc::c_int {
    let mut base1: *const libc::c_char = strrchr((*a).name, '.' as i32);
    let mut base2: *const libc::c_char = strrchr((*b).name, '.' as i32);
    let mut diff: libc::c_int = cmp
        .expect(
            "non-null function pointer",
        )(
        if !base1.is_null() { base1 } else { b"\0" as *const u8 as *const libc::c_char },
        if !base2.is_null() { base2 } else { b"\0" as *const u8 as *const libc::c_char },
    );
    return if diff != 0 {
        diff
    } else {
        cmp.expect("non-null function pointer")((*a).name, (*b).name)
    };
}
unsafe extern "C" fn fileinfo_name_width(mut f: *const fileinfo) -> size_t {
    return if (*f).width != 0 {
        (*f).width
    } else {
        quote_name_width((*f).name, filename_quoting_options, (*f).quoted)
    };
}
unsafe extern "C" fn cmp_width(
    mut a: *const fileinfo,
    mut b: *const fileinfo,
    mut cmp: Option::<
        unsafe extern "C" fn(*const libc::c_char, *const libc::c_char) -> libc::c_int,
    >,
) -> libc::c_int {
    let mut diff: libc::c_int = (fileinfo_name_width(a))
        .wrapping_sub(fileinfo_name_width(b)) as libc::c_int;
    return if diff != 0 {
        diff
    } else {
        cmp.expect("non-null function pointer")((*a).name, (*b).name)
    };
}
unsafe extern "C" fn xstrcoll_df_ctime(mut a: V, mut b: V) -> libc::c_int {
    return dirfirst_check(
        a as *const fileinfo,
        b as *const fileinfo,
        Some(xstrcoll_ctime as unsafe extern "C" fn(V, V) -> libc::c_int),
    );
}
unsafe extern "C" fn xstrcoll_ctime(mut a: V, mut b: V) -> libc::c_int {
    return cmp_ctime(
        a as *const fileinfo,
        b as *const fileinfo,
        Some(
            xstrcoll
                as unsafe extern "C" fn(
                    *const libc::c_char,
                    *const libc::c_char,
                ) -> libc::c_int,
        ),
    );
}
unsafe extern "C" fn rev_strcmp_df_ctime(mut a: V, mut b: V) -> libc::c_int {
    return dirfirst_check(
        a as *const fileinfo,
        b as *const fileinfo,
        Some(rev_strcmp_ctime as unsafe extern "C" fn(V, V) -> libc::c_int),
    );
}
unsafe extern "C" fn rev_strcmp_ctime(mut a: V, mut b: V) -> libc::c_int {
    return cmp_ctime(
        b as *const fileinfo,
        a as *const fileinfo,
        Some(
            strcmp
                as unsafe extern "C" fn(
                    *const libc::c_char,
                    *const libc::c_char,
                ) -> libc::c_int,
        ),
    );
}
unsafe extern "C" fn strcmp_df_ctime(mut a: V, mut b: V) -> libc::c_int {
    return dirfirst_check(
        a as *const fileinfo,
        b as *const fileinfo,
        Some(strcmp_ctime as unsafe extern "C" fn(V, V) -> libc::c_int),
    );
}
unsafe extern "C" fn strcmp_ctime(mut a: V, mut b: V) -> libc::c_int {
    return cmp_ctime(
        a as *const fileinfo,
        b as *const fileinfo,
        Some(
            strcmp
                as unsafe extern "C" fn(
                    *const libc::c_char,
                    *const libc::c_char,
                ) -> libc::c_int,
        ),
    );
}
unsafe extern "C" fn rev_xstrcoll_df_ctime(mut a: V, mut b: V) -> libc::c_int {
    return dirfirst_check(
        a as *const fileinfo,
        b as *const fileinfo,
        Some(rev_xstrcoll_ctime as unsafe extern "C" fn(V, V) -> libc::c_int),
    );
}
unsafe extern "C" fn rev_xstrcoll_ctime(mut a: V, mut b: V) -> libc::c_int {
    return cmp_ctime(
        b as *const fileinfo,
        a as *const fileinfo,
        Some(
            xstrcoll
                as unsafe extern "C" fn(
                    *const libc::c_char,
                    *const libc::c_char,
                ) -> libc::c_int,
        ),
    );
}
unsafe extern "C" fn strcmp_df_mtime(mut a: V, mut b: V) -> libc::c_int {
    return dirfirst_check(
        a as *const fileinfo,
        b as *const fileinfo,
        Some(strcmp_mtime as unsafe extern "C" fn(V, V) -> libc::c_int),
    );
}
unsafe extern "C" fn xstrcoll_df_mtime(mut a: V, mut b: V) -> libc::c_int {
    return dirfirst_check(
        a as *const fileinfo,
        b as *const fileinfo,
        Some(xstrcoll_mtime as unsafe extern "C" fn(V, V) -> libc::c_int),
    );
}
unsafe extern "C" fn rev_xstrcoll_mtime(mut a: V, mut b: V) -> libc::c_int {
    return cmp_mtime(
        b as *const fileinfo,
        a as *const fileinfo,
        Some(
            xstrcoll
                as unsafe extern "C" fn(
                    *const libc::c_char,
                    *const libc::c_char,
                ) -> libc::c_int,
        ),
    );
}
unsafe extern "C" fn rev_xstrcoll_df_mtime(mut a: V, mut b: V) -> libc::c_int {
    return dirfirst_check(
        a as *const fileinfo,
        b as *const fileinfo,
        Some(rev_xstrcoll_mtime as unsafe extern "C" fn(V, V) -> libc::c_int),
    );
}
unsafe extern "C" fn strcmp_mtime(mut a: V, mut b: V) -> libc::c_int {
    return cmp_mtime(
        a as *const fileinfo,
        b as *const fileinfo,
        Some(
            strcmp
                as unsafe extern "C" fn(
                    *const libc::c_char,
                    *const libc::c_char,
                ) -> libc::c_int,
        ),
    );
}
unsafe extern "C" fn rev_strcmp_mtime(mut a: V, mut b: V) -> libc::c_int {
    return cmp_mtime(
        b as *const fileinfo,
        a as *const fileinfo,
        Some(
            strcmp
                as unsafe extern "C" fn(
                    *const libc::c_char,
                    *const libc::c_char,
                ) -> libc::c_int,
        ),
    );
}
unsafe extern "C" fn rev_strcmp_df_mtime(mut a: V, mut b: V) -> libc::c_int {
    return dirfirst_check(
        a as *const fileinfo,
        b as *const fileinfo,
        Some(rev_strcmp_mtime as unsafe extern "C" fn(V, V) -> libc::c_int),
    );
}
unsafe extern "C" fn xstrcoll_mtime(mut a: V, mut b: V) -> libc::c_int {
    return cmp_mtime(
        a as *const fileinfo,
        b as *const fileinfo,
        Some(
            xstrcoll
                as unsafe extern "C" fn(
                    *const libc::c_char,
                    *const libc::c_char,
                ) -> libc::c_int,
        ),
    );
}
unsafe extern "C" fn xstrcoll_atime(mut a: V, mut b: V) -> libc::c_int {
    return cmp_atime(
        a as *const fileinfo,
        b as *const fileinfo,
        Some(
            xstrcoll
                as unsafe extern "C" fn(
                    *const libc::c_char,
                    *const libc::c_char,
                ) -> libc::c_int,
        ),
    );
}
unsafe extern "C" fn rev_xstrcoll_df_atime(mut a: V, mut b: V) -> libc::c_int {
    return dirfirst_check(
        a as *const fileinfo,
        b as *const fileinfo,
        Some(rev_xstrcoll_atime as unsafe extern "C" fn(V, V) -> libc::c_int),
    );
}
unsafe extern "C" fn rev_xstrcoll_atime(mut a: V, mut b: V) -> libc::c_int {
    return cmp_atime(
        b as *const fileinfo,
        a as *const fileinfo,
        Some(
            xstrcoll
                as unsafe extern "C" fn(
                    *const libc::c_char,
                    *const libc::c_char,
                ) -> libc::c_int,
        ),
    );
}
unsafe extern "C" fn xstrcoll_df_atime(mut a: V, mut b: V) -> libc::c_int {
    return dirfirst_check(
        a as *const fileinfo,
        b as *const fileinfo,
        Some(xstrcoll_atime as unsafe extern "C" fn(V, V) -> libc::c_int),
    );
}
unsafe extern "C" fn strcmp_atime(mut a: V, mut b: V) -> libc::c_int {
    return cmp_atime(
        a as *const fileinfo,
        b as *const fileinfo,
        Some(
            strcmp
                as unsafe extern "C" fn(
                    *const libc::c_char,
                    *const libc::c_char,
                ) -> libc::c_int,
        ),
    );
}
unsafe extern "C" fn rev_strcmp_atime(mut a: V, mut b: V) -> libc::c_int {
    return cmp_atime(
        b as *const fileinfo,
        a as *const fileinfo,
        Some(
            strcmp
                as unsafe extern "C" fn(
                    *const libc::c_char,
                    *const libc::c_char,
                ) -> libc::c_int,
        ),
    );
}
unsafe extern "C" fn rev_strcmp_df_atime(mut a: V, mut b: V) -> libc::c_int {
    return dirfirst_check(
        a as *const fileinfo,
        b as *const fileinfo,
        Some(rev_strcmp_atime as unsafe extern "C" fn(V, V) -> libc::c_int),
    );
}
unsafe extern "C" fn strcmp_df_atime(mut a: V, mut b: V) -> libc::c_int {
    return dirfirst_check(
        a as *const fileinfo,
        b as *const fileinfo,
        Some(strcmp_atime as unsafe extern "C" fn(V, V) -> libc::c_int),
    );
}
unsafe extern "C" fn strcmp_btime(mut a: V, mut b: V) -> libc::c_int {
    return cmp_btime(
        a as *const fileinfo,
        b as *const fileinfo,
        Some(
            strcmp
                as unsafe extern "C" fn(
                    *const libc::c_char,
                    *const libc::c_char,
                ) -> libc::c_int,
        ),
    );
}
unsafe extern "C" fn xstrcoll_btime(mut a: V, mut b: V) -> libc::c_int {
    return cmp_btime(
        a as *const fileinfo,
        b as *const fileinfo,
        Some(
            xstrcoll
                as unsafe extern "C" fn(
                    *const libc::c_char,
                    *const libc::c_char,
                ) -> libc::c_int,
        ),
    );
}
unsafe extern "C" fn rev_xstrcoll_btime(mut a: V, mut b: V) -> libc::c_int {
    return cmp_btime(
        b as *const fileinfo,
        a as *const fileinfo,
        Some(
            xstrcoll
                as unsafe extern "C" fn(
                    *const libc::c_char,
                    *const libc::c_char,
                ) -> libc::c_int,
        ),
    );
}
unsafe extern "C" fn rev_xstrcoll_df_btime(mut a: V, mut b: V) -> libc::c_int {
    return dirfirst_check(
        a as *const fileinfo,
        b as *const fileinfo,
        Some(rev_xstrcoll_btime as unsafe extern "C" fn(V, V) -> libc::c_int),
    );
}
unsafe extern "C" fn xstrcoll_df_btime(mut a: V, mut b: V) -> libc::c_int {
    return dirfirst_check(
        a as *const fileinfo,
        b as *const fileinfo,
        Some(xstrcoll_btime as unsafe extern "C" fn(V, V) -> libc::c_int),
    );
}
unsafe extern "C" fn rev_strcmp_btime(mut a: V, mut b: V) -> libc::c_int {
    return cmp_btime(
        b as *const fileinfo,
        a as *const fileinfo,
        Some(
            strcmp
                as unsafe extern "C" fn(
                    *const libc::c_char,
                    *const libc::c_char,
                ) -> libc::c_int,
        ),
    );
}
unsafe extern "C" fn rev_strcmp_df_btime(mut a: V, mut b: V) -> libc::c_int {
    return dirfirst_check(
        a as *const fileinfo,
        b as *const fileinfo,
        Some(rev_strcmp_btime as unsafe extern "C" fn(V, V) -> libc::c_int),
    );
}
unsafe extern "C" fn strcmp_df_btime(mut a: V, mut b: V) -> libc::c_int {
    return dirfirst_check(
        a as *const fileinfo,
        b as *const fileinfo,
        Some(strcmp_btime as unsafe extern "C" fn(V, V) -> libc::c_int),
    );
}
unsafe extern "C" fn xstrcoll_df_size(mut a: V, mut b: V) -> libc::c_int {
    return dirfirst_check(
        a as *const fileinfo,
        b as *const fileinfo,
        Some(xstrcoll_size as unsafe extern "C" fn(V, V) -> libc::c_int),
    );
}
unsafe extern "C" fn xstrcoll_size(mut a: V, mut b: V) -> libc::c_int {
    return cmp_size(
        a as *const fileinfo,
        b as *const fileinfo,
        Some(
            xstrcoll
                as unsafe extern "C" fn(
                    *const libc::c_char,
                    *const libc::c_char,
                ) -> libc::c_int,
        ),
    );
}
unsafe extern "C" fn rev_strcmp_df_size(mut a: V, mut b: V) -> libc::c_int {
    return dirfirst_check(
        a as *const fileinfo,
        b as *const fileinfo,
        Some(rev_strcmp_size as unsafe extern "C" fn(V, V) -> libc::c_int),
    );
}
unsafe extern "C" fn rev_strcmp_size(mut a: V, mut b: V) -> libc::c_int {
    return cmp_size(
        b as *const fileinfo,
        a as *const fileinfo,
        Some(
            strcmp
                as unsafe extern "C" fn(
                    *const libc::c_char,
                    *const libc::c_char,
                ) -> libc::c_int,
        ),
    );
}
unsafe extern "C" fn strcmp_df_size(mut a: V, mut b: V) -> libc::c_int {
    return dirfirst_check(
        a as *const fileinfo,
        b as *const fileinfo,
        Some(strcmp_size as unsafe extern "C" fn(V, V) -> libc::c_int),
    );
}
unsafe extern "C" fn strcmp_size(mut a: V, mut b: V) -> libc::c_int {
    return cmp_size(
        a as *const fileinfo,
        b as *const fileinfo,
        Some(
            strcmp
                as unsafe extern "C" fn(
                    *const libc::c_char,
                    *const libc::c_char,
                ) -> libc::c_int,
        ),
    );
}
unsafe extern "C" fn rev_xstrcoll_df_size(mut a: V, mut b: V) -> libc::c_int {
    return dirfirst_check(
        a as *const fileinfo,
        b as *const fileinfo,
        Some(rev_xstrcoll_size as unsafe extern "C" fn(V, V) -> libc::c_int),
    );
}
unsafe extern "C" fn rev_xstrcoll_size(mut a: V, mut b: V) -> libc::c_int {
    return cmp_size(
        b as *const fileinfo,
        a as *const fileinfo,
        Some(
            xstrcoll
                as unsafe extern "C" fn(
                    *const libc::c_char,
                    *const libc::c_char,
                ) -> libc::c_int,
        ),
    );
}
unsafe extern "C" fn xstrcoll_name(mut a: V, mut b: V) -> libc::c_int {
    return cmp_name(
        a as *const fileinfo,
        b as *const fileinfo,
        Some(
            xstrcoll
                as unsafe extern "C" fn(
                    *const libc::c_char,
                    *const libc::c_char,
                ) -> libc::c_int,
        ),
    );
}
unsafe extern "C" fn rev_strcmp_df_name(mut a: V, mut b: V) -> libc::c_int {
    return dirfirst_check(
        a as *const fileinfo,
        b as *const fileinfo,
        Some(rev_strcmp_name as unsafe extern "C" fn(V, V) -> libc::c_int),
    );
}
unsafe extern "C" fn rev_strcmp_name(mut a: V, mut b: V) -> libc::c_int {
    return cmp_name(
        b as *const fileinfo,
        a as *const fileinfo,
        Some(
            strcmp
                as unsafe extern "C" fn(
                    *const libc::c_char,
                    *const libc::c_char,
                ) -> libc::c_int,
        ),
    );
}
unsafe extern "C" fn strcmp_df_name(mut a: V, mut b: V) -> libc::c_int {
    return dirfirst_check(
        a as *const fileinfo,
        b as *const fileinfo,
        Some(strcmp_name as unsafe extern "C" fn(V, V) -> libc::c_int),
    );
}
unsafe extern "C" fn strcmp_name(mut a: V, mut b: V) -> libc::c_int {
    return cmp_name(
        a as *const fileinfo,
        b as *const fileinfo,
        Some(
            strcmp
                as unsafe extern "C" fn(
                    *const libc::c_char,
                    *const libc::c_char,
                ) -> libc::c_int,
        ),
    );
}
unsafe extern "C" fn rev_xstrcoll_df_name(mut a: V, mut b: V) -> libc::c_int {
    return dirfirst_check(
        a as *const fileinfo,
        b as *const fileinfo,
        Some(rev_xstrcoll_name as unsafe extern "C" fn(V, V) -> libc::c_int),
    );
}
unsafe extern "C" fn rev_xstrcoll_name(mut a: V, mut b: V) -> libc::c_int {
    return cmp_name(
        b as *const fileinfo,
        a as *const fileinfo,
        Some(
            xstrcoll
                as unsafe extern "C" fn(
                    *const libc::c_char,
                    *const libc::c_char,
                ) -> libc::c_int,
        ),
    );
}
unsafe extern "C" fn xstrcoll_df_name(mut a: V, mut b: V) -> libc::c_int {
    return dirfirst_check(
        a as *const fileinfo,
        b as *const fileinfo,
        Some(xstrcoll_name as unsafe extern "C" fn(V, V) -> libc::c_int),
    );
}
unsafe extern "C" fn rev_strcmp_extension(mut a: V, mut b: V) -> libc::c_int {
    return cmp_extension(
        b as *const fileinfo,
        a as *const fileinfo,
        Some(
            strcmp
                as unsafe extern "C" fn(
                    *const libc::c_char,
                    *const libc::c_char,
                ) -> libc::c_int,
        ),
    );
}
unsafe extern "C" fn xstrcoll_extension(mut a: V, mut b: V) -> libc::c_int {
    return cmp_extension(
        a as *const fileinfo,
        b as *const fileinfo,
        Some(
            xstrcoll
                as unsafe extern "C" fn(
                    *const libc::c_char,
                    *const libc::c_char,
                ) -> libc::c_int,
        ),
    );
}
unsafe extern "C" fn rev_strcmp_df_extension(mut a: V, mut b: V) -> libc::c_int {
    return dirfirst_check(
        a as *const fileinfo,
        b as *const fileinfo,
        Some(rev_strcmp_extension as unsafe extern "C" fn(V, V) -> libc::c_int),
    );
}
unsafe extern "C" fn xstrcoll_df_extension(mut a: V, mut b: V) -> libc::c_int {
    return dirfirst_check(
        a as *const fileinfo,
        b as *const fileinfo,
        Some(xstrcoll_extension as unsafe extern "C" fn(V, V) -> libc::c_int),
    );
}
unsafe extern "C" fn strcmp_df_extension(mut a: V, mut b: V) -> libc::c_int {
    return dirfirst_check(
        a as *const fileinfo,
        b as *const fileinfo,
        Some(strcmp_extension as unsafe extern "C" fn(V, V) -> libc::c_int),
    );
}
unsafe extern "C" fn strcmp_extension(mut a: V, mut b: V) -> libc::c_int {
    return cmp_extension(
        a as *const fileinfo,
        b as *const fileinfo,
        Some(
            strcmp
                as unsafe extern "C" fn(
                    *const libc::c_char,
                    *const libc::c_char,
                ) -> libc::c_int,
        ),
    );
}
unsafe extern "C" fn rev_xstrcoll_df_extension(mut a: V, mut b: V) -> libc::c_int {
    return dirfirst_check(
        a as *const fileinfo,
        b as *const fileinfo,
        Some(rev_xstrcoll_extension as unsafe extern "C" fn(V, V) -> libc::c_int),
    );
}
unsafe extern "C" fn rev_xstrcoll_extension(mut a: V, mut b: V) -> libc::c_int {
    return cmp_extension(
        b as *const fileinfo,
        a as *const fileinfo,
        Some(
            xstrcoll
                as unsafe extern "C" fn(
                    *const libc::c_char,
                    *const libc::c_char,
                ) -> libc::c_int,
        ),
    );
}
unsafe extern "C" fn strcmp_df_width(mut a: V, mut b: V) -> libc::c_int {
    return dirfirst_check(
        a as *const fileinfo,
        b as *const fileinfo,
        Some(strcmp_width as unsafe extern "C" fn(V, V) -> libc::c_int),
    );
}
unsafe extern "C" fn rev_xstrcoll_df_width(mut a: V, mut b: V) -> libc::c_int {
    return dirfirst_check(
        a as *const fileinfo,
        b as *const fileinfo,
        Some(rev_xstrcoll_width as unsafe extern "C" fn(V, V) -> libc::c_int),
    );
}
unsafe extern "C" fn strcmp_width(mut a: V, mut b: V) -> libc::c_int {
    return cmp_width(
        a as *const fileinfo,
        b as *const fileinfo,
        Some(
            strcmp
                as unsafe extern "C" fn(
                    *const libc::c_char,
                    *const libc::c_char,
                ) -> libc::c_int,
        ),
    );
}
unsafe extern "C" fn xstrcoll_width(mut a: V, mut b: V) -> libc::c_int {
    return cmp_width(
        a as *const fileinfo,
        b as *const fileinfo,
        Some(
            xstrcoll
                as unsafe extern "C" fn(
                    *const libc::c_char,
                    *const libc::c_char,
                ) -> libc::c_int,
        ),
    );
}
unsafe extern "C" fn rev_strcmp_width(mut a: V, mut b: V) -> libc::c_int {
    return cmp_width(
        b as *const fileinfo,
        a as *const fileinfo,
        Some(
            strcmp
                as unsafe extern "C" fn(
                    *const libc::c_char,
                    *const libc::c_char,
                ) -> libc::c_int,
        ),
    );
}
unsafe extern "C" fn rev_strcmp_df_width(mut a: V, mut b: V) -> libc::c_int {
    return dirfirst_check(
        a as *const fileinfo,
        b as *const fileinfo,
        Some(rev_strcmp_width as unsafe extern "C" fn(V, V) -> libc::c_int),
    );
}
unsafe extern "C" fn rev_xstrcoll_width(mut a: V, mut b: V) -> libc::c_int {
    return cmp_width(
        b as *const fileinfo,
        a as *const fileinfo,
        Some(
            xstrcoll
                as unsafe extern "C" fn(
                    *const libc::c_char,
                    *const libc::c_char,
                ) -> libc::c_int,
        ),
    );
}
unsafe extern "C" fn xstrcoll_df_width(mut a: V, mut b: V) -> libc::c_int {
    return dirfirst_check(
        a as *const fileinfo,
        b as *const fileinfo,
        Some(xstrcoll_width as unsafe extern "C" fn(V, V) -> libc::c_int),
    );
}
unsafe extern "C" fn cmp_version(
    mut a: *const fileinfo,
    mut b: *const fileinfo,
) -> libc::c_int {
    let mut diff: libc::c_int = filevercmp((*a).name, (*b).name);
    return if diff != 0 { diff } else { strcmp((*a).name, (*b).name) };
}
unsafe extern "C" fn xstrcoll_version(mut a: V, mut b: V) -> libc::c_int {
    return cmp_version(a as *const fileinfo, b as *const fileinfo);
}
unsafe extern "C" fn rev_xstrcoll_version(mut a: V, mut b: V) -> libc::c_int {
    return cmp_version(b as *const fileinfo, a as *const fileinfo);
}
unsafe extern "C" fn xstrcoll_df_version(mut a: V, mut b: V) -> libc::c_int {
    return dirfirst_check(
        a as *const fileinfo,
        b as *const fileinfo,
        Some(xstrcoll_version as unsafe extern "C" fn(V, V) -> libc::c_int),
    );
}
unsafe extern "C" fn rev_xstrcoll_df_version(mut a: V, mut b: V) -> libc::c_int {
    return dirfirst_check(
        a as *const fileinfo,
        b as *const fileinfo,
        Some(rev_xstrcoll_version as unsafe extern "C" fn(V, V) -> libc::c_int),
    );
}
static mut sort_functions: [[[[qsortFunc; 2]; 2]; 2]; 9] = unsafe {
    [
        [
            [
                [
                    Some(xstrcoll_name as unsafe extern "C" fn(V, V) -> libc::c_int),
                    Some(xstrcoll_df_name as unsafe extern "C" fn(V, V) -> libc::c_int),
                ],
                [
                    Some(rev_xstrcoll_name as unsafe extern "C" fn(V, V) -> libc::c_int),
                    Some(
                        rev_xstrcoll_df_name as unsafe extern "C" fn(V, V) -> libc::c_int,
                    ),
                ],
            ],
            [
                [
                    Some(strcmp_name as unsafe extern "C" fn(V, V) -> libc::c_int),
                    Some(strcmp_df_name as unsafe extern "C" fn(V, V) -> libc::c_int),
                ],
                [
                    Some(rev_strcmp_name as unsafe extern "C" fn(V, V) -> libc::c_int),
                    Some(rev_strcmp_df_name as unsafe extern "C" fn(V, V) -> libc::c_int),
                ],
            ],
        ],
        [
            [
                [
                    Some(
                        xstrcoll_extension as unsafe extern "C" fn(V, V) -> libc::c_int,
                    ),
                    Some(
                        xstrcoll_df_extension
                            as unsafe extern "C" fn(V, V) -> libc::c_int,
                    ),
                ],
                [
                    Some(
                        rev_xstrcoll_extension
                            as unsafe extern "C" fn(V, V) -> libc::c_int,
                    ),
                    Some(
                        rev_xstrcoll_df_extension
                            as unsafe extern "C" fn(V, V) -> libc::c_int,
                    ),
                ],
            ],
            [
                [
                    Some(strcmp_extension as unsafe extern "C" fn(V, V) -> libc::c_int),
                    Some(
                        strcmp_df_extension as unsafe extern "C" fn(V, V) -> libc::c_int,
                    ),
                ],
                [
                    Some(
                        rev_strcmp_extension as unsafe extern "C" fn(V, V) -> libc::c_int,
                    ),
                    Some(
                        rev_strcmp_df_extension
                            as unsafe extern "C" fn(V, V) -> libc::c_int,
                    ),
                ],
            ],
        ],
        [
            [
                [
                    Some(xstrcoll_width as unsafe extern "C" fn(V, V) -> libc::c_int),
                    Some(xstrcoll_df_width as unsafe extern "C" fn(V, V) -> libc::c_int),
                ],
                [
                    Some(
                        rev_xstrcoll_width as unsafe extern "C" fn(V, V) -> libc::c_int,
                    ),
                    Some(
                        rev_xstrcoll_df_width
                            as unsafe extern "C" fn(V, V) -> libc::c_int,
                    ),
                ],
            ],
            [
                [
                    Some(strcmp_width as unsafe extern "C" fn(V, V) -> libc::c_int),
                    Some(strcmp_df_width as unsafe extern "C" fn(V, V) -> libc::c_int),
                ],
                [
                    Some(rev_strcmp_width as unsafe extern "C" fn(V, V) -> libc::c_int),
                    Some(
                        rev_strcmp_df_width as unsafe extern "C" fn(V, V) -> libc::c_int,
                    ),
                ],
            ],
        ],
        [
            [
                [
                    Some(xstrcoll_size as unsafe extern "C" fn(V, V) -> libc::c_int),
                    Some(xstrcoll_df_size as unsafe extern "C" fn(V, V) -> libc::c_int),
                ],
                [
                    Some(rev_xstrcoll_size as unsafe extern "C" fn(V, V) -> libc::c_int),
                    Some(
                        rev_xstrcoll_df_size as unsafe extern "C" fn(V, V) -> libc::c_int,
                    ),
                ],
            ],
            [
                [
                    Some(strcmp_size as unsafe extern "C" fn(V, V) -> libc::c_int),
                    Some(strcmp_df_size as unsafe extern "C" fn(V, V) -> libc::c_int),
                ],
                [
                    Some(rev_strcmp_size as unsafe extern "C" fn(V, V) -> libc::c_int),
                    Some(rev_strcmp_df_size as unsafe extern "C" fn(V, V) -> libc::c_int),
                ],
            ],
        ],
        [
            [
                [
                    Some(xstrcoll_version as unsafe extern "C" fn(V, V) -> libc::c_int),
                    Some(
                        xstrcoll_df_version as unsafe extern "C" fn(V, V) -> libc::c_int,
                    ),
                ],
                [
                    Some(
                        rev_xstrcoll_version as unsafe extern "C" fn(V, V) -> libc::c_int,
                    ),
                    Some(
                        rev_xstrcoll_df_version
                            as unsafe extern "C" fn(V, V) -> libc::c_int,
                    ),
                ],
            ],
            [[None, None], [None, None]],
        ],
        [
            [
                [
                    Some(xstrcoll_mtime as unsafe extern "C" fn(V, V) -> libc::c_int),
                    Some(xstrcoll_df_mtime as unsafe extern "C" fn(V, V) -> libc::c_int),
                ],
                [
                    Some(
                        rev_xstrcoll_mtime as unsafe extern "C" fn(V, V) -> libc::c_int,
                    ),
                    Some(
                        rev_xstrcoll_df_mtime
                            as unsafe extern "C" fn(V, V) -> libc::c_int,
                    ),
                ],
            ],
            [
                [
                    Some(strcmp_mtime as unsafe extern "C" fn(V, V) -> libc::c_int),
                    Some(strcmp_df_mtime as unsafe extern "C" fn(V, V) -> libc::c_int),
                ],
                [
                    Some(rev_strcmp_mtime as unsafe extern "C" fn(V, V) -> libc::c_int),
                    Some(
                        rev_strcmp_df_mtime as unsafe extern "C" fn(V, V) -> libc::c_int,
                    ),
                ],
            ],
        ],
        [
            [
                [
                    Some(xstrcoll_ctime as unsafe extern "C" fn(V, V) -> libc::c_int),
                    Some(xstrcoll_df_ctime as unsafe extern "C" fn(V, V) -> libc::c_int),
                ],
                [
                    Some(
                        rev_xstrcoll_ctime as unsafe extern "C" fn(V, V) -> libc::c_int,
                    ),
                    Some(
                        rev_xstrcoll_df_ctime
                            as unsafe extern "C" fn(V, V) -> libc::c_int,
                    ),
                ],
            ],
            [
                [
                    Some(strcmp_ctime as unsafe extern "C" fn(V, V) -> libc::c_int),
                    Some(strcmp_df_ctime as unsafe extern "C" fn(V, V) -> libc::c_int),
                ],
                [
                    Some(rev_strcmp_ctime as unsafe extern "C" fn(V, V) -> libc::c_int),
                    Some(
                        rev_strcmp_df_ctime as unsafe extern "C" fn(V, V) -> libc::c_int,
                    ),
                ],
            ],
        ],
        [
            [
                [
                    Some(xstrcoll_atime as unsafe extern "C" fn(V, V) -> libc::c_int),
                    Some(xstrcoll_df_atime as unsafe extern "C" fn(V, V) -> libc::c_int),
                ],
                [
                    Some(
                        rev_xstrcoll_atime as unsafe extern "C" fn(V, V) -> libc::c_int,
                    ),
                    Some(
                        rev_xstrcoll_df_atime
                            as unsafe extern "C" fn(V, V) -> libc::c_int,
                    ),
                ],
            ],
            [
                [
                    Some(strcmp_atime as unsafe extern "C" fn(V, V) -> libc::c_int),
                    Some(strcmp_df_atime as unsafe extern "C" fn(V, V) -> libc::c_int),
                ],
                [
                    Some(rev_strcmp_atime as unsafe extern "C" fn(V, V) -> libc::c_int),
                    Some(
                        rev_strcmp_df_atime as unsafe extern "C" fn(V, V) -> libc::c_int,
                    ),
                ],
            ],
        ],
        [
            [
                [
                    Some(xstrcoll_btime as unsafe extern "C" fn(V, V) -> libc::c_int),
                    Some(xstrcoll_df_btime as unsafe extern "C" fn(V, V) -> libc::c_int),
                ],
                [
                    Some(
                        rev_xstrcoll_btime as unsafe extern "C" fn(V, V) -> libc::c_int,
                    ),
                    Some(
                        rev_xstrcoll_df_btime
                            as unsafe extern "C" fn(V, V) -> libc::c_int,
                    ),
                ],
            ],
            [
                [
                    Some(strcmp_btime as unsafe extern "C" fn(V, V) -> libc::c_int),
                    Some(strcmp_df_btime as unsafe extern "C" fn(V, V) -> libc::c_int),
                ],
                [
                    Some(rev_strcmp_btime as unsafe extern "C" fn(V, V) -> libc::c_int),
                    Some(
                        rev_strcmp_df_btime as unsafe extern "C" fn(V, V) -> libc::c_int,
                    ),
                ],
            ],
        ],
    ]
};
unsafe extern "C" fn initialize_ordering_vector() {
    let mut i: size_t = 0 as libc::c_int as size_t;
    while i < cwd_n_used {
        let ref mut fresh26 = *sorted_file.offset(i as isize);
        *fresh26 = &mut *cwd_file.offset(i as isize) as *mut fileinfo
            as *mut libc::c_void;
        i = i.wrapping_add(1);
    }
}
unsafe extern "C" fn update_current_files_info() {
    if sort_type as libc::c_uint == sort_width as libc::c_int as libc::c_uint
        || line_length != 0
            && (format as libc::c_uint == many_per_line as libc::c_int as libc::c_uint
                || format as libc::c_uint == horizontal as libc::c_int as libc::c_uint)
    {
        let mut i: size_t = 0;
        i = 0 as libc::c_int as size_t;
        while i < cwd_n_used {
            let mut f: *mut fileinfo = *sorted_file.offset(i as isize) as *mut fileinfo;
            (*f).width = fileinfo_name_width(f);
            i = i.wrapping_add(1);
        }
    }
}
unsafe extern "C" fn sort_files() {
    let mut use_strcmp: bool = false;
    if sorted_file_alloc
        < cwd_n_used
            .wrapping_add(cwd_n_used.wrapping_div(2 as libc::c_int as libc::c_ulong))
    {
        free(sorted_file as *mut libc::c_void);
        sorted_file = xnmalloc(
            cwd_n_used,
            (3 as libc::c_int as libc::c_ulong)
                .wrapping_mul(
                    ::core::mem::size_of::<*mut libc::c_void>() as libc::c_ulong,
                ),
        ) as *mut *mut libc::c_void;
        sorted_file_alloc = (3 as libc::c_int as libc::c_ulong).wrapping_mul(cwd_n_used);
    }
    initialize_ordering_vector();
    update_current_files_info();
    if sort_type as libc::c_uint == sort_none as libc::c_int as libc::c_uint {
        return;
    }
    if _setjmp(failed_strcoll.as_mut_ptr()) == 0 {
        use_strcmp = 0 as libc::c_int != 0;
    } else {
        use_strcmp = 1 as libc::c_int != 0;
        if sort_type as libc::c_uint != sort_version as libc::c_int as libc::c_uint
        {} else {
            __assert_fail(
                b"sort_type != sort_version\0" as *const u8 as *const libc::c_char,
                b"src/ls.c\0" as *const u8 as *const libc::c_char,
                4106 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 22],
                    &[libc::c_char; 22],
                >(b"void sort_files(void)\0"))
                    .as_ptr(),
            );
        }
        initialize_ordering_vector();
    }
    mpsort(
        sorted_file as *mut *const libc::c_void,
        cwd_n_used,
        sort_functions[(sort_type as libc::c_uint)
            .wrapping_add(
                (if sort_type as libc::c_uint == sort_time as libc::c_int as libc::c_uint
                {
                    time_type as libc::c_uint
                } else {
                    0 as libc::c_int as libc::c_uint
                }),
            )
            as usize][use_strcmp
            as usize][sort_reverse as usize][directories_first as usize],
    );
}
unsafe extern "C" fn print_current_files() {
    let mut i: size_t = 0;
    match format as libc::c_uint {
        1 => {
            i = 0 as libc::c_int as size_t;
            while i < cwd_n_used {
                print_file_name_and_frills(
                    *sorted_file.offset(i as isize) as *const fileinfo,
                    0 as libc::c_int as size_t,
                );
                putchar_unlocked(eolbyte as libc::c_int);
                i = i.wrapping_add(1);
            }
        }
        2 => {
            if line_length == 0 {
                print_with_separator(' ' as i32 as libc::c_char);
            } else {
                print_many_per_line();
            }
        }
        3 => {
            if line_length == 0 {
                print_with_separator(' ' as i32 as libc::c_char);
            } else {
                print_horizontal();
            }
        }
        4 => {
            print_with_separator(',' as i32 as libc::c_char);
        }
        0 => {
            i = 0 as libc::c_int as size_t;
            while i < cwd_n_used {
                set_normal_color();
                print_long_format(*sorted_file.offset(i as isize) as *const fileinfo);
                dired_outbyte(eolbyte);
                i = i.wrapping_add(1);
            }
        }
        _ => {}
    };
}
unsafe extern "C" fn align_nstrftime(
    mut buf: *mut libc::c_char,
    mut size: size_t,
    mut recent: bool,
    mut tm: *const tm,
    mut tz: timezone_t,
    mut ns: libc::c_int,
) -> size_t {
    let mut nfmt: *const libc::c_char = if use_abformat as libc::c_int != 0 {
        (abformat[recent as usize][(*tm).tm_mon as usize]).as_mut_ptr()
            as *const libc::c_char
    } else {
        long_time_format[recent as usize]
    };
    return nstrftime(buf, size, nfmt, tm, tz, ns);
}
unsafe extern "C" fn long_time_expected_width() -> libc::c_int {
    static mut width: libc::c_int = -(1 as libc::c_int);
    if width < 0 as libc::c_int {
        let mut epoch: time_t = 0 as libc::c_int as time_t;
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
        let mut buf: [libc::c_char; 1001] = [0; 1001];
        if !(localtime_rz(localtz, &mut epoch, &mut tm)).is_null() {
            let mut len: size_t = align_nstrftime(
                buf.as_mut_ptr(),
                ::core::mem::size_of::<[libc::c_char; 1001]>() as libc::c_ulong,
                0 as libc::c_int != 0,
                &mut tm,
                localtz,
                0 as libc::c_int,
            );
            if len != 0 as libc::c_int as libc::c_ulong {
                width = mbsnwidth(buf.as_mut_ptr(), len, 0 as libc::c_int);
            }
        }
        if width < 0 as libc::c_int {
            width = 0 as libc::c_int;
        }
    }
    return width;
}
unsafe extern "C" fn format_user_or_group(
    mut name: *const libc::c_char,
    mut id: uintmax_t,
    mut width: libc::c_int,
) {
    if !name.is_null() {
        let mut width_gap: libc::c_int = width - gnu_mbswidth(name, 0 as libc::c_int);
        let mut pad: libc::c_int = if 0 as libc::c_int > width_gap {
            0 as libc::c_int
        } else {
            width_gap
        };
        dired_outstring(name);
        loop {
            dired_outbyte(' ' as i32 as libc::c_char);
            let fresh27 = pad;
            pad = pad - 1;
            if !(fresh27 != 0) {
                break;
            }
        }
    } else {
        dired_pos
            += printf(b"%*lu \0" as *const u8 as *const libc::c_char, width, id)
                as libc::c_long;
    };
}
unsafe extern "C" fn format_user(
    mut u: uid_t,
    mut width: libc::c_int,
    mut stat_ok: bool,
) {
    format_user_or_group(
        if !stat_ok {
            b"?\0" as *const u8 as *const libc::c_char
        } else {
            (if numeric_ids as libc::c_int != 0 {
                0 as *mut libc::c_char
            } else {
                getuser(u)
            }) as *const libc::c_char
        },
        u as uintmax_t,
        width,
    );
}
unsafe extern "C" fn format_group(
    mut g: gid_t,
    mut width: libc::c_int,
    mut stat_ok: bool,
) {
    format_user_or_group(
        if !stat_ok {
            b"?\0" as *const u8 as *const libc::c_char
        } else {
            (if numeric_ids as libc::c_int != 0 {
                0 as *mut libc::c_char
            } else {
                getgroup(g)
            }) as *const libc::c_char
        },
        g as uintmax_t,
        width,
    );
}
unsafe extern "C" fn format_user_or_group_width(
    mut name: *const libc::c_char,
    mut id: uintmax_t,
) -> libc::c_int {
    if !name.is_null() {
        let mut len: libc::c_int = gnu_mbswidth(name, 0 as libc::c_int);
        return if 0 as libc::c_int > len { 0 as libc::c_int } else { len };
    } else {
        return snprintf(
            0 as *mut libc::c_char,
            0 as libc::c_int as libc::c_ulong,
            b"%lu\0" as *const u8 as *const libc::c_char,
            id,
        )
    };
}
unsafe extern "C" fn format_user_width(mut u: uid_t) -> libc::c_int {
    return format_user_or_group_width(
        if numeric_ids as libc::c_int != 0 {
            0 as *mut libc::c_char
        } else {
            getuser(u)
        },
        u as uintmax_t,
    );
}
unsafe extern "C" fn format_group_width(mut g: gid_t) -> libc::c_int {
    return format_user_or_group_width(
        if numeric_ids as libc::c_int != 0 {
            0 as *mut libc::c_char
        } else {
            getgroup(g)
        },
        g as uintmax_t,
    );
}
unsafe extern "C" fn format_inode(
    mut buf: *mut libc::c_char,
    mut buflen: size_t,
    mut f: *const fileinfo,
) -> *mut libc::c_char {
    if (::core::mem::size_of::<uintmax_t>() as libc::c_ulong)
        .wrapping_mul(8 as libc::c_int as libc::c_ulong)
        .wrapping_sub(
            !((0 as libc::c_int as uintmax_t) < -(1 as libc::c_int) as uintmax_t)
                as libc::c_int as libc::c_ulong,
        )
        .wrapping_mul(146 as libc::c_int as libc::c_ulong)
        .wrapping_add(484 as libc::c_int as libc::c_ulong)
        .wrapping_div(485 as libc::c_int as libc::c_ulong)
        .wrapping_add(
            !((0 as libc::c_int as uintmax_t) < -(1 as libc::c_int) as uintmax_t)
                as libc::c_int as libc::c_ulong,
        )
        .wrapping_add(1 as libc::c_int as libc::c_ulong) <= buflen
    {} else {
        __assert_fail(
            b"INT_BUFSIZE_BOUND (uintmax_t) <= buflen\0" as *const u8
                as *const libc::c_char,
            b"src/ls.c\0" as *const u8 as *const libc::c_char,
            4288 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 60],
                &[libc::c_char; 60],
            >(b"char *format_inode(char *, size_t, const struct fileinfo *)\0"))
                .as_ptr(),
        );
    }
    return if (*f).stat_ok as libc::c_int != 0
        && (*f).stat.st_ino != NOT_AN_INODE_NUMBER as libc::c_int as libc::c_ulong
    {
        umaxtostr((*f).stat.st_ino, buf)
    } else {
        b"?\0" as *const u8 as *const libc::c_char as *mut libc::c_char
    };
}
unsafe extern "C" fn print_long_format(mut f: *const fileinfo) {
    let mut modebuf: [libc::c_char; 12] = [0; 12];
    let mut buf: [libc::c_char; 3643] = [0; 3643];
    let mut s: size_t = 0;
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut when_timespec: timespec = timespec { tv_sec: 0, tv_nsec: 0 };
    let mut when_local: tm = tm {
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
    let mut btime_ok: bool = 1 as libc::c_int != 0;
    if (*f).stat_ok {
        filemodestring(&(*f).stat, modebuf.as_mut_ptr());
    } else {
        modebuf[0 as libc::c_int as usize] = filetype_letter[(*f).filetype as usize];
        memset(
            modebuf.as_mut_ptr().offset(1 as libc::c_int as isize) as *mut libc::c_void,
            '?' as i32,
            10 as libc::c_int as libc::c_ulong,
        );
        modebuf[11 as libc::c_int as usize] = '\0' as i32 as libc::c_char;
    }
    if !any_has_acl {
        modebuf[10 as libc::c_int as usize] = '\0' as i32 as libc::c_char;
    } else if (*f).acl_type as libc::c_uint
        == ACL_T_LSM_CONTEXT_ONLY as libc::c_int as libc::c_uint
    {
        modebuf[10 as libc::c_int as usize] = '.' as i32 as libc::c_char;
    } else if (*f).acl_type as libc::c_uint == ACL_T_YES as libc::c_int as libc::c_uint {
        modebuf[10 as libc::c_int as usize] = '+' as i32 as libc::c_char;
    }
    match time_type as libc::c_uint {
        1 => {
            when_timespec = get_stat_ctime(&(*f).stat);
        }
        0 => {
            when_timespec = get_stat_mtime(&(*f).stat);
        }
        2 => {
            when_timespec = get_stat_atime(&(*f).stat);
        }
        3 => {
            when_timespec = get_stat_btime(&(*f).stat);
            if when_timespec.tv_sec == -(1 as libc::c_int) as libc::c_long
                && when_timespec.tv_nsec == -(1 as libc::c_int) as libc::c_long
            {
                btime_ok = 0 as libc::c_int != 0;
            }
        }
        _ => {
            abort();
        }
    }
    p = buf.as_mut_ptr();
    if print_inode {
        let mut hbuf: [libc::c_char; 21] = [0; 21];
        p = p
            .offset(
                sprintf(
                    p,
                    b"%*s \0" as *const u8 as *const libc::c_char,
                    inode_number_width,
                    format_inode(
                        hbuf.as_mut_ptr(),
                        ::core::mem::size_of::<[libc::c_char; 21]>() as libc::c_ulong,
                        f,
                    ),
                ) as isize,
            );
    }
    if print_block_size {
        let mut hbuf_0: [libc::c_char; 652] = [0; 652];
        let mut blocks: *const libc::c_char = if !(*f).stat_ok {
            b"?\0" as *const u8 as *const libc::c_char
        } else {
            human_readable(
                (*f).stat.st_blocks as uintmax_t,
                hbuf_0.as_mut_ptr(),
                human_output_opts,
                512 as libc::c_int as uintmax_t,
                output_block_size,
            ) as *const libc::c_char
        };
        let mut pad: libc::c_int = 0;
        pad = block_size_width - gnu_mbswidth(blocks, 0 as libc::c_int);
        while (0 as libc::c_int) < pad {
            let fresh28 = p;
            p = p.offset(1);
            *fresh28 = ' ' as i32 as libc::c_char;
            pad -= 1;
        }
        loop {
            let fresh29 = blocks;
            blocks = blocks.offset(1);
            let fresh30 = p;
            p = p.offset(1);
            *fresh30 = *fresh29;
            if !(*fresh30 != 0) {
                break;
            }
        }
        *p.offset(-(1 as libc::c_int) as isize) = ' ' as i32 as libc::c_char;
    }
    let mut hbuf_1: [libc::c_char; 21] = [0; 21];
    p = p
        .offset(
            sprintf(
                p,
                b"%s %*s \0" as *const u8 as *const libc::c_char,
                modebuf.as_mut_ptr(),
                nlink_width,
                if !(*f).stat_ok {
                    b"?\0" as *const u8 as *const libc::c_char
                } else {
                    umaxtostr((*f).stat.st_nlink as uintmax_t, hbuf_1.as_mut_ptr())
                        as *const libc::c_char
                },
            ) as isize,
        );
    dired_indent();
    if print_owner as libc::c_int != 0 || print_group as libc::c_int != 0
        || print_author as libc::c_int != 0 || print_scontext as libc::c_int != 0
    {
        dired_outbuf(
            buf.as_mut_ptr(),
            p.offset_from(buf.as_mut_ptr()) as libc::c_long as size_t,
        );
        if print_owner {
            format_user((*f).stat.st_uid, owner_width, (*f).stat_ok);
        }
        if print_group {
            format_group((*f).stat.st_gid, group_width, (*f).stat_ok);
        }
        if print_author {
            format_user((*f).stat.st_uid, author_width, (*f).stat_ok);
        }
        if print_scontext {
            format_user_or_group(
                (*f).scontext,
                0 as libc::c_int as uintmax_t,
                scontext_width,
            );
        }
        p = buf.as_mut_ptr();
    }
    if (*f).stat_ok as libc::c_int != 0
        && ((*f).stat.st_mode & 0o170000 as libc::c_int as libc::c_uint
            == 0o20000 as libc::c_int as libc::c_uint
            || (*f).stat.st_mode & 0o170000 as libc::c_int as libc::c_uint
                == 0o60000 as libc::c_int as libc::c_uint)
    {
        let mut majorbuf: [libc::c_char; 21] = [0; 21];
        let mut minorbuf: [libc::c_char; 21] = [0; 21];
        let mut blanks_width: libc::c_int = file_size_width
            - (major_device_number_width + 2 as libc::c_int + minor_device_number_width);
        p = p
            .offset(
                sprintf(
                    p,
                    b"%*s, %*s \0" as *const u8 as *const libc::c_char,
                    major_device_number_width
                        + (if 0 as libc::c_int > blanks_width {
                            0 as libc::c_int
                        } else {
                            blanks_width
                        }),
                    umaxtostr(
                        gnu_dev_major((*f).stat.st_rdev) as uintmax_t,
                        majorbuf.as_mut_ptr(),
                    ),
                    minor_device_number_width,
                    umaxtostr(
                        gnu_dev_minor((*f).stat.st_rdev) as uintmax_t,
                        minorbuf.as_mut_ptr(),
                    ),
                ) as isize,
            );
    } else {
        let mut hbuf_2: [libc::c_char; 652] = [0; 652];
        let mut size: *const libc::c_char = if !(*f).stat_ok {
            b"?\0" as *const u8 as *const libc::c_char
        } else {
            human_readable(
                unsigned_file_size((*f).stat.st_size),
                hbuf_2.as_mut_ptr(),
                file_human_output_opts,
                1 as libc::c_int as uintmax_t,
                file_output_block_size,
            ) as *const libc::c_char
        };
        let mut pad_0: libc::c_int = 0;
        pad_0 = file_size_width - gnu_mbswidth(size, 0 as libc::c_int);
        while (0 as libc::c_int) < pad_0 {
            let fresh31 = p;
            p = p.offset(1);
            *fresh31 = ' ' as i32 as libc::c_char;
            pad_0 -= 1;
        }
        loop {
            let fresh32 = size;
            size = size.offset(1);
            let fresh33 = p;
            p = p.offset(1);
            *fresh33 = *fresh32;
            if !(*fresh33 != 0) {
                break;
            }
        }
        *p.offset(-(1 as libc::c_int) as isize) = ' ' as i32 as libc::c_char;
    }
    s = 0 as libc::c_int as size_t;
    *p = '\u{1}' as i32 as libc::c_char;
    if (*f).stat_ok as libc::c_int != 0 && btime_ok as libc::c_int != 0
        && !(localtime_rz(localtz, &mut when_timespec.tv_sec, &mut when_local)).is_null()
    {
        let mut six_months_ago: timespec = timespec { tv_sec: 0, tv_nsec: 0 };
        let mut recent: bool = false;
        if timespec_cmp(current_time, when_timespec) < 0 as libc::c_int {
            gettime(&mut current_time);
        }
        six_months_ago
            .tv_sec = current_time.tv_sec
            - (31556952 as libc::c_int / 2 as libc::c_int) as libc::c_long;
        six_months_ago.tv_nsec = current_time.tv_nsec;
        recent = timespec_cmp(six_months_ago, when_timespec) < 0 as libc::c_int
            && timespec_cmp(when_timespec, current_time) < 0 as libc::c_int;
        s = align_nstrftime(
            p,
            (TIME_STAMP_LEN_MAXIMUM as libc::c_int + 1 as libc::c_int) as size_t,
            recent,
            &mut when_local,
            localtz,
            when_timespec.tv_nsec as libc::c_int,
        );
    }
    if s != 0 || *p == 0 {
        p = p.offset(s as isize);
        let fresh34 = p;
        p = p.offset(1);
        *fresh34 = ' ' as i32 as libc::c_char;
    } else {
        let mut hbuf_3: [libc::c_char; 21] = [0; 21];
        p = p
            .offset(
                sprintf(
                    p,
                    b"%*s \0" as *const u8 as *const libc::c_char,
                    long_time_expected_width(),
                    if !(*f).stat_ok || !btime_ok {
                        b"?\0" as *const u8 as *const libc::c_char
                    } else {
                        timetostr(when_timespec.tv_sec, hbuf_3.as_mut_ptr())
                            as *const libc::c_char
                    },
                ) as isize,
            );
    }
    dired_outbuf(
        buf.as_mut_ptr(),
        p.offset_from(buf.as_mut_ptr()) as libc::c_long as size_t,
    );
    let mut w: size_t = print_name_with_quoting(
        f,
        0 as libc::c_int != 0,
        &mut dired_obstack,
        p.offset_from(buf.as_mut_ptr()) as libc::c_long as size_t,
    );
    if (*f).filetype as libc::c_uint == symbolic_link as libc::c_int as libc::c_uint {
        if !((*f).linkname).is_null() {
            dired_outstring(b" -> \0" as *const u8 as *const libc::c_char);
            print_name_with_quoting(
                f,
                1 as libc::c_int != 0,
                0 as *mut obstack,
                (p.offset_from(buf.as_mut_ptr()) as libc::c_long as libc::c_ulong)
                    .wrapping_add(w)
                    .wrapping_add(4 as libc::c_int as libc::c_ulong),
            );
            if indicator_style as libc::c_uint != none as libc::c_int as libc::c_uint {
                print_type_indicator(1 as libc::c_int != 0, (*f).linkmode, unknown);
            }
        }
    } else if indicator_style as libc::c_uint != none as libc::c_int as libc::c_uint {
        print_type_indicator((*f).stat_ok, (*f).stat.st_mode, (*f).filetype);
    }
}
unsafe extern "C" fn quote_name_buf(
    mut inbuf: *mut *mut libc::c_char,
    mut bufsize: size_t,
    mut name: *mut libc::c_char,
    mut options: *const quoting_options,
    mut needs_general_quoting: libc::c_int,
    mut width: *mut size_t,
    mut pad: *mut bool,
) -> size_t {
    let mut buf: *mut libc::c_char = *inbuf;
    let mut displayed_width: size_t = 0 as libc::c_int as size_t;
    let mut len: size_t = 0 as libc::c_int as size_t;
    let mut quoted: bool = false;
    let mut qs: quoting_style = get_quoting_style(options);
    let mut needs_further_quoting: bool = qmark_funny_chars as libc::c_int != 0
        && (qs as libc::c_uint == shell_quoting_style as libc::c_int as libc::c_uint
            || qs as libc::c_uint
                == shell_always_quoting_style as libc::c_int as libc::c_uint
            || qs as libc::c_uint
                == literal_quoting_style as libc::c_int as libc::c_uint);
    if needs_general_quoting != 0 as libc::c_int {
        len = quotearg_buffer(
            buf,
            bufsize,
            name,
            -(1 as libc::c_int) as size_t,
            options,
        );
        if bufsize <= len {
            buf = xmalloc(len.wrapping_add(1 as libc::c_int as libc::c_ulong))
                as *mut libc::c_char;
            quotearg_buffer(
                buf,
                len.wrapping_add(1 as libc::c_int as libc::c_ulong),
                name,
                -(1 as libc::c_int) as size_t,
                options,
            );
        }
        quoted = *name as libc::c_int != *buf as libc::c_int || strlen(name) != len;
    } else if needs_further_quoting {
        len = strlen(name);
        if bufsize <= len {
            buf = xmalloc(len.wrapping_add(1 as libc::c_int as libc::c_ulong))
                as *mut libc::c_char;
        }
        memcpy(
            buf as *mut libc::c_void,
            name as *const libc::c_void,
            len.wrapping_add(1 as libc::c_int as libc::c_ulong),
        );
        quoted = 0 as libc::c_int != 0;
    } else {
        len = strlen(name);
        buf = name;
        quoted = 0 as libc::c_int != 0;
    }
    if needs_further_quoting {
        if __ctype_get_mb_cur_max() > 1 as libc::c_int as libc::c_ulong {
            let mut p: *const libc::c_char = buf;
            let mut plimit: *const libc::c_char = buf.offset(len as isize);
            let mut q: *mut libc::c_char = buf;
            displayed_width = 0 as libc::c_int as size_t;
            while p < plimit {
                match *p as libc::c_int {
                    32 | 33 | 34 | 35 | 37 | 38 | 39 | 40 | 41 | 42 | 43 | 44 | 45 | 46
                    | 47 | 48 | 49 | 50 | 51 | 52 | 53 | 54 | 55 | 56 | 57 | 58 | 59 | 60
                    | 61 | 62 | 63 | 65 | 66 | 67 | 68 | 69 | 70 | 71 | 72 | 73 | 74 | 75
                    | 76 | 77 | 78 | 79 | 80 | 81 | 82 | 83 | 84 | 85 | 86 | 87 | 88 | 89
                    | 90 | 91 | 92 | 93 | 94 | 95 | 97 | 98 | 99 | 100 | 101 | 102 | 103
                    | 104 | 105 | 106 | 107 | 108 | 109 | 110 | 111 | 112 | 113 | 114
                    | 115 | 116 | 117 | 118 | 119 | 120 | 121 | 122 | 123 | 124 | 125
                    | 126 => {
                        let fresh35 = p;
                        p = p.offset(1);
                        let fresh36 = q;
                        q = q.offset(1);
                        *fresh36 = *fresh35;
                        displayed_width = (displayed_width as libc::c_ulong)
                            .wrapping_add(1 as libc::c_int as libc::c_ulong) as size_t
                            as size_t;
                    }
                    _ => {
                        let mut mbstate: mbstate_t = {
                            let mut init = __mbstate_t {
                                __count: 0 as libc::c_int,
                                __value: C2RustUnnamed { __wch: 0 },
                            };
                            init
                        };
                        loop {
                            let mut wc: wchar_t = 0;
                            let mut bytes: size_t = 0;
                            let mut w: libc::c_int = 0;
                            bytes = rpl_mbrtowc(
                                &mut wc,
                                p,
                                plimit.offset_from(p) as libc::c_long as size_t,
                                &mut mbstate,
                            );
                            if bytes == -(1 as libc::c_int) as size_t {
                                p = p.offset(1);
                                let fresh37 = q;
                                q = q.offset(1);
                                *fresh37 = '?' as i32 as libc::c_char;
                                displayed_width = (displayed_width as libc::c_ulong)
                                    .wrapping_add(1 as libc::c_int as libc::c_ulong) as size_t
                                    as size_t;
                                break;
                            } else if bytes == -(2 as libc::c_int) as size_t {
                                p = plimit;
                                let fresh38 = q;
                                q = q.offset(1);
                                *fresh38 = '?' as i32 as libc::c_char;
                                displayed_width = (displayed_width as libc::c_ulong)
                                    .wrapping_add(1 as libc::c_int as libc::c_ulong) as size_t
                                    as size_t;
                                break;
                            } else {
                                if bytes == 0 as libc::c_int as libc::c_ulong {
                                    bytes = 1 as libc::c_int as size_t;
                                }
                                w = wcwidth(wc);
                                if w >= 0 as libc::c_int {
                                    while bytes > 0 as libc::c_int as libc::c_ulong {
                                        let fresh39 = p;
                                        p = p.offset(1);
                                        let fresh40 = q;
                                        q = q.offset(1);
                                        *fresh40 = *fresh39;
                                        bytes = bytes.wrapping_sub(1);
                                    }
                                    displayed_width = (displayed_width as libc::c_ulong)
                                        .wrapping_add(w as libc::c_ulong) as size_t as size_t;
                                } else {
                                    p = p.offset(bytes as isize);
                                    let fresh41 = q;
                                    q = q.offset(1);
                                    *fresh41 = '?' as i32 as libc::c_char;
                                    displayed_width = (displayed_width as libc::c_ulong)
                                        .wrapping_add(1 as libc::c_int as libc::c_ulong) as size_t
                                        as size_t;
                                }
                                if !(mbsinit(&mut mbstate) == 0) {
                                    break;
                                }
                            }
                        }
                    }
                }
            }
            len = q.offset_from(buf) as libc::c_long as size_t;
        } else {
            let mut p_0: *mut libc::c_char = buf;
            let mut plimit_0: *const libc::c_char = buf.offset(len as isize);
            while p_0 < plimit_0 as *mut libc::c_char {
                if *(*__ctype_b_loc()).offset(to_uchar(*p_0) as libc::c_int as isize)
                    as libc::c_int
                    & _ISprint as libc::c_int as libc::c_ushort as libc::c_int == 0
                {
                    *p_0 = '?' as i32 as libc::c_char;
                }
                p_0 = p_0.offset(1);
            }
            displayed_width = len;
        }
    } else if !width.is_null() {
        if __ctype_get_mb_cur_max() > 1 as libc::c_int as libc::c_ulong {
            displayed_width = mbsnwidth(buf, len, 0 as libc::c_int) as size_t;
        } else {
            let mut p_1: *const libc::c_char = buf;
            let mut plimit_1: *const libc::c_char = buf.offset(len as isize);
            displayed_width = 0 as libc::c_int as size_t;
            while p_1 < plimit_1 {
                if *(*__ctype_b_loc()).offset(to_uchar(*p_1) as libc::c_int as isize)
                    as libc::c_int
                    & _ISprint as libc::c_int as libc::c_ushort as libc::c_int != 0
                {
                    displayed_width = displayed_width.wrapping_add(1);
                }
                p_1 = p_1.offset(1);
            }
        }
    }
    *pad = align_variable_outer_quotes as libc::c_int != 0
        && cwd_some_quoted as libc::c_int != 0 && !quoted;
    if !width.is_null() {
        *width = displayed_width;
    }
    *inbuf = buf;
    return len;
}
unsafe extern "C" fn quote_name_width(
    mut name: *const libc::c_char,
    mut options: *const quoting_options,
    mut needs_general_quoting: libc::c_int,
) -> size_t {
    let mut smallbuf: [libc::c_char; 8192] = [0; 8192];
    let mut buf: *mut libc::c_char = smallbuf.as_mut_ptr();
    let mut width: size_t = 0;
    let mut pad: bool = false;
    quote_name_buf(
        &mut buf,
        ::core::mem::size_of::<[libc::c_char; 8192]>() as libc::c_ulong,
        name as *mut libc::c_char,
        options,
        needs_general_quoting,
        &mut width,
        &mut pad,
    );
    if buf != smallbuf.as_mut_ptr() && buf != name as *mut libc::c_char {
        free(buf as *mut libc::c_void);
    }
    width = (width as libc::c_ulong).wrapping_add(pad as libc::c_ulong) as size_t
        as size_t;
    return width;
}
unsafe extern "C" fn file_escape(
    mut str: *const libc::c_char,
    mut path: bool,
) -> *mut libc::c_char {
    let mut esc: *mut libc::c_char = xnmalloc(
        3 as libc::c_int as size_t,
        (strlen(str)).wrapping_add(1 as libc::c_int as libc::c_ulong),
    ) as *mut libc::c_char;
    let mut p: *mut libc::c_char = esc;
    while *str != 0 {
        if path as libc::c_int != 0 && *str as libc::c_int == '/' as i32 {
            let fresh42 = p;
            p = p.offset(1);
            *fresh42 = '/' as i32 as libc::c_char;
            str = str.offset(1);
        } else if RFC3986[to_uchar(*str) as usize] != 0 {
            let fresh43 = str;
            str = str.offset(1);
            let fresh44 = p;
            p = p.offset(1);
            *fresh44 = *fresh43;
        } else {
            let fresh45 = str;
            str = str.offset(1);
            p = p
                .offset(
                    sprintf(
                        p,
                        b"%%%02x\0" as *const u8 as *const libc::c_char,
                        to_uchar(*fresh45) as libc::c_int,
                    ) as isize,
                );
        }
    }
    *p = '\0' as i32 as libc::c_char;
    return esc;
}
unsafe extern "C" fn quote_name(
    mut name: *const libc::c_char,
    mut options: *const quoting_options,
    mut needs_general_quoting: libc::c_int,
    mut color: *const bin_str,
    mut allow_pad: bool,
    mut stack: *mut obstack,
    mut absolute_name: *const libc::c_char,
) -> size_t {
    let mut smallbuf: [libc::c_char; 8192] = [0; 8192];
    let mut buf: *mut libc::c_char = smallbuf.as_mut_ptr();
    let mut len: size_t = 0;
    let mut pad: bool = false;
    len = quote_name_buf(
        &mut buf,
        ::core::mem::size_of::<[libc::c_char; 8192]>() as libc::c_ulong,
        name as *mut libc::c_char,
        options,
        needs_general_quoting,
        0 as *mut size_t,
        &mut pad,
    );
    if pad as libc::c_int != 0 && allow_pad as libc::c_int != 0 {
        dired_outbyte(' ' as i32 as libc::c_char);
    }
    if !color.is_null() {
        print_color_indicator(color);
    }
    let mut skip_quotes: bool = 0 as libc::c_int != 0;
    if !absolute_name.is_null() {
        if align_variable_outer_quotes as libc::c_int != 0
            && cwd_some_quoted as libc::c_int != 0 && !pad
        {
            skip_quotes = 1 as libc::c_int != 0;
            putchar_unlocked(*buf as libc::c_int);
        }
        let mut h: *mut libc::c_char = file_escape(hostname, 0 as libc::c_int != 0);
        let mut n: *mut libc::c_char = file_escape(absolute_name, 1 as libc::c_int != 0);
        printf(
            b"\x1B]8;;file://%s%s%s\x07\0" as *const u8 as *const libc::c_char,
            h,
            if *n as libc::c_int == '/' as i32 {
                b"\0" as *const u8 as *const libc::c_char
            } else {
                b"/\0" as *const u8 as *const libc::c_char
            },
            n,
        );
        free(h as *mut libc::c_void);
        free(n as *mut libc::c_void);
    }
    if !stack.is_null() {
        push_current_dired_pos(stack);
    }
    if 0 != 0 && 0 != 0
        && (1 as libc::c_int as size_t)
            .wrapping_mul(
                len
                    .wrapping_sub(
                        (skip_quotes as libc::c_int * 2 as libc::c_int) as libc::c_ulong,
                    ),
            ) <= 8 as libc::c_int as libc::c_ulong
        && 1 as libc::c_int as size_t != 0 as libc::c_int as libc::c_ulong
    {
        ({
            let mut __ptr: *const libc::c_char = buf
                .offset(skip_quotes as libc::c_int as isize) as *const libc::c_char;
            let mut __stream: *mut FILE = stdout;
            let mut __cnt: size_t = 0;
            __cnt = (1 as libc::c_int as size_t)
                .wrapping_mul(
                    len
                        .wrapping_sub(
                            (skip_quotes as libc::c_int * 2 as libc::c_int)
                                as libc::c_ulong,
                        ),
                );
            while __cnt > 0 as libc::c_int as libc::c_ulong {
                let fresh46 = __ptr;
                __ptr = __ptr.offset(1);
                if putc_unlocked(*fresh46 as libc::c_int, __stream)
                    == -(1 as libc::c_int)
                {
                    break;
                }
                __cnt = __cnt.wrapping_sub(1);
            }
            compile_error!("Binary expression is not supposed to be used")
        });
    } else {
        if 0 != 0 && 1 as libc::c_int as size_t == 0 as libc::c_int as libc::c_ulong
            || 0 != 0
                && len
                    .wrapping_sub(
                        (skip_quotes as libc::c_int * 2 as libc::c_int) as libc::c_ulong,
                    ) == 0 as libc::c_int as libc::c_ulong
        {} else {
            fwrite_unlocked(
                buf.offset(skip_quotes as libc::c_int as isize) as *const libc::c_void,
                1 as libc::c_int as size_t,
                len
                    .wrapping_sub(
                        (skip_quotes as libc::c_int * 2 as libc::c_int) as libc::c_ulong,
                    ),
                stdout,
            );
        };
    };
    dired_pos = (dired_pos as libc::c_ulong).wrapping_add(len) as off_t as off_t;
    if !stack.is_null() {
        push_current_dired_pos(stack);
    }
    if !absolute_name.is_null() {
        fputs_unlocked(b"\x1B]8;;\x07\0" as *const u8 as *const libc::c_char, stdout);
        if skip_quotes {
            putchar_unlocked(
                *buf.offset(len as isize).offset(-(1 as libc::c_int as isize))
                    as libc::c_int,
            );
        }
    }
    if buf != smallbuf.as_mut_ptr() && buf != name as *mut libc::c_char {
        free(buf as *mut libc::c_void);
    }
    return len.wrapping_add(pad as libc::c_ulong);
}
unsafe extern "C" fn print_name_with_quoting(
    mut f: *const fileinfo,
    mut symlink_target: bool,
    mut stack: *mut obstack,
    mut start_col: size_t,
) -> size_t {
    let mut name: *const libc::c_char = if symlink_target as libc::c_int != 0 {
        (*f).linkname
    } else {
        (*f).name
    };
    let mut color: *const bin_str = if print_with_color as libc::c_int != 0 {
        get_color_indicator(f, symlink_target)
    } else {
        0 as *const bin_str
    };
    let mut used_color_this_time: bool = print_with_color as libc::c_int != 0
        && (!color.is_null() || is_colored(C_NORM) as libc::c_int != 0);
    let mut len: size_t = quote_name(
        name,
        filename_quoting_options,
        (*f).quoted,
        color,
        !symlink_target,
        stack,
        (*f).absolute_name,
    );
    process_signals();
    if used_color_this_time {
        prep_non_filename_text();
        if line_length != 0
            && start_col.wrapping_div(line_length)
                != start_col
                    .wrapping_add(len)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                    .wrapping_div(line_length)
        {
            put_indicator(
                &mut *color_indicator
                    .as_mut_ptr()
                    .offset(C_CLR_TO_EOL as libc::c_int as isize),
            );
        }
    }
    return len;
}
unsafe extern "C" fn prep_non_filename_text() {
    if !(color_indicator[C_END as libc::c_int as usize].string).is_null() {
        put_indicator(
            &mut *color_indicator.as_mut_ptr().offset(C_END as libc::c_int as isize),
        );
    } else {
        put_indicator(
            &mut *color_indicator.as_mut_ptr().offset(C_LEFT as libc::c_int as isize),
        );
        put_indicator(
            &mut *color_indicator.as_mut_ptr().offset(C_RESET as libc::c_int as isize),
        );
        put_indicator(
            &mut *color_indicator.as_mut_ptr().offset(C_RIGHT as libc::c_int as isize),
        );
    };
}
unsafe extern "C" fn print_file_name_and_frills(
    mut f: *const fileinfo,
    mut start_col: size_t,
) -> size_t {
    let mut buf: [libc::c_char; 652] = [0; 652];
    set_normal_color();
    if print_inode {
        printf(
            b"%*s \0" as *const u8 as *const libc::c_char,
            if format as libc::c_uint == with_commas as libc::c_int as libc::c_uint {
                0 as libc::c_int
            } else {
                inode_number_width
            },
            format_inode(
                buf.as_mut_ptr(),
                ::core::mem::size_of::<[libc::c_char; 652]>() as libc::c_ulong,
                f,
            ),
        );
    }
    if print_block_size {
        printf(
            b"%*s \0" as *const u8 as *const libc::c_char,
            if format as libc::c_uint == with_commas as libc::c_int as libc::c_uint {
                0 as libc::c_int
            } else {
                block_size_width
            },
            if !(*f).stat_ok {
                b"?\0" as *const u8 as *const libc::c_char
            } else {
                human_readable(
                    (*f).stat.st_blocks as uintmax_t,
                    buf.as_mut_ptr(),
                    human_output_opts,
                    512 as libc::c_int as uintmax_t,
                    output_block_size,
                ) as *const libc::c_char
            },
        );
    }
    if print_scontext {
        printf(
            b"%*s \0" as *const u8 as *const libc::c_char,
            if format as libc::c_uint == with_commas as libc::c_int as libc::c_uint {
                0 as libc::c_int
            } else {
                scontext_width
            },
            (*f).scontext,
        );
    }
    let mut width: size_t = print_name_with_quoting(
        f,
        0 as libc::c_int != 0,
        0 as *mut obstack,
        start_col,
    );
    if indicator_style as libc::c_uint != none as libc::c_int as libc::c_uint {
        width = (width as libc::c_ulong)
            .wrapping_add(
                print_type_indicator((*f).stat_ok, (*f).stat.st_mode, (*f).filetype)
                    as libc::c_ulong,
            ) as size_t as size_t;
    }
    return width;
}
unsafe extern "C" fn get_type_indicator(
    mut stat_ok: bool,
    mut mode: mode_t,
    mut type_0: filetype,
) -> libc::c_char {
    let mut c: libc::c_char = 0;
    if if stat_ok as libc::c_int != 0 {
        (mode & 0o170000 as libc::c_int as libc::c_uint
            == 0o100000 as libc::c_int as libc::c_uint) as libc::c_int
    } else {
        (type_0 as libc::c_uint == normal as libc::c_int as libc::c_uint) as libc::c_int
    } != 0
    {
        if stat_ok as libc::c_int != 0
            && indicator_style as libc::c_uint == classify as libc::c_int as libc::c_uint
            && mode
                & (0o100 as libc::c_int | 0o100 as libc::c_int >> 3 as libc::c_int
                    | 0o100 as libc::c_int >> 3 as libc::c_int >> 3 as libc::c_int)
                    as libc::c_uint != 0
        {
            c = '*' as i32 as libc::c_char;
        } else {
            c = 0 as libc::c_int as libc::c_char;
        }
    } else if if stat_ok as libc::c_int != 0 {
        (mode & 0o170000 as libc::c_int as libc::c_uint
            == 0o40000 as libc::c_int as libc::c_uint) as libc::c_int
    } else {
        (type_0 as libc::c_uint == directory as libc::c_int as libc::c_uint
            || type_0 as libc::c_uint == arg_directory as libc::c_int as libc::c_uint)
            as libc::c_int
    } != 0
    {
        c = '/' as i32 as libc::c_char;
    } else if indicator_style as libc::c_uint == slash as libc::c_int as libc::c_uint {
        c = 0 as libc::c_int as libc::c_char;
    } else if if stat_ok as libc::c_int != 0 {
        (mode & 0o170000 as libc::c_int as libc::c_uint
            == 0o120000 as libc::c_int as libc::c_uint) as libc::c_int
    } else {
        (type_0 as libc::c_uint == symbolic_link as libc::c_int as libc::c_uint)
            as libc::c_int
    } != 0
    {
        c = '@' as i32 as libc::c_char;
    } else if if stat_ok as libc::c_int != 0 {
        (mode & 0o170000 as libc::c_int as libc::c_uint
            == 0o10000 as libc::c_int as libc::c_uint) as libc::c_int
    } else {
        (type_0 as libc::c_uint == fifo as libc::c_int as libc::c_uint) as libc::c_int
    } != 0
    {
        c = '|' as i32 as libc::c_char;
    } else if if stat_ok as libc::c_int != 0 {
        (mode & 0o170000 as libc::c_int as libc::c_uint
            == 0o140000 as libc::c_int as libc::c_uint) as libc::c_int
    } else {
        (type_0 as libc::c_uint == sock as libc::c_int as libc::c_uint) as libc::c_int
    } != 0
    {
        c = '=' as i32 as libc::c_char;
    } else if stat_ok as libc::c_int != 0 && 0 as libc::c_int != 0 {
        c = '>' as i32 as libc::c_char;
    } else {
        c = 0 as libc::c_int as libc::c_char;
    }
    return c;
}
unsafe extern "C" fn print_type_indicator(
    mut stat_ok: bool,
    mut mode: mode_t,
    mut type_0: filetype,
) -> bool {
    let mut c: libc::c_char = get_type_indicator(stat_ok, mode, type_0);
    if c != 0 {
        dired_outbyte(c);
    }
    return c != 0;
}
unsafe extern "C" fn print_color_indicator(mut ind: *const bin_str) -> bool {
    if !ind.is_null() {
        if is_colored(C_NORM) {
            restore_default_color();
        }
        put_indicator(
            &mut *color_indicator.as_mut_ptr().offset(C_LEFT as libc::c_int as isize),
        );
        put_indicator(ind);
        put_indicator(
            &mut *color_indicator.as_mut_ptr().offset(C_RIGHT as libc::c_int as isize),
        );
    }
    return !ind.is_null();
}
unsafe extern "C" fn get_color_indicator(
    mut f: *const fileinfo,
    mut symlink_target: bool,
) -> *const bin_str {
    let mut type_0: indicator_no = C_LEFT;
    let mut ext: *mut color_ext_type = 0 as *mut color_ext_type;
    let mut len: size_t = 0;
    let mut name: *const libc::c_char = 0 as *const libc::c_char;
    let mut mode: mode_t = 0;
    let mut linkok: libc::c_int = 0;
    if symlink_target {
        name = (*f).linkname;
        mode = (*f).linkmode;
        linkok = if (*f).linkok as libc::c_int != 0 {
            0 as libc::c_int
        } else {
            -(1 as libc::c_int)
        };
    } else {
        name = (*f).name;
        mode = file_or_link_mode(f);
        linkok = (*f).linkok as libc::c_int;
    }
    if linkok == -(1 as libc::c_int) && is_colored(C_MISSING) as libc::c_int != 0 {
        type_0 = C_MISSING;
    } else if !(*f).stat_ok {
        static mut filetype_indicator: [indicator_no; 10] = [
            C_ORPHAN,
            C_FIFO,
            C_CHR,
            C_DIR,
            C_BLK,
            C_FILE,
            C_LINK,
            C_SOCK,
            C_FILE,
            C_DIR,
        ];
        type_0 = filetype_indicator[(*f).filetype as usize];
    } else if mode & 0o170000 as libc::c_int as libc::c_uint
        == 0o100000 as libc::c_int as libc::c_uint
    {
        type_0 = C_FILE;
        if mode & 0o4000 as libc::c_int as libc::c_uint
            != 0 as libc::c_int as libc::c_uint
            && is_colored(C_SETUID) as libc::c_int != 0
        {
            type_0 = C_SETUID;
        } else if mode & 0o2000 as libc::c_int as libc::c_uint
            != 0 as libc::c_int as libc::c_uint
            && is_colored(C_SETGID) as libc::c_int != 0
        {
            type_0 = C_SETGID;
        } else if is_colored(C_CAP) as libc::c_int != 0
            && (*f).has_capability as libc::c_int != 0
        {
            type_0 = C_CAP;
        } else if mode
            & (0o100 as libc::c_int | 0o100 as libc::c_int >> 3 as libc::c_int
                | 0o100 as libc::c_int >> 3 as libc::c_int >> 3 as libc::c_int)
                as libc::c_uint != 0 as libc::c_int as libc::c_uint
            && is_colored(C_EXEC) as libc::c_int != 0
        {
            type_0 = C_EXEC;
        } else if (1 as libc::c_int as libc::c_uint) < (*f).stat.st_nlink
            && is_colored(C_MULTIHARDLINK) as libc::c_int != 0
        {
            type_0 = C_MULTIHARDLINK;
        }
    } else if mode & 0o170000 as libc::c_int as libc::c_uint
        == 0o40000 as libc::c_int as libc::c_uint
    {
        type_0 = C_DIR;
        if mode & 0o1000 as libc::c_int as libc::c_uint != 0
            && mode
                & (0o200 as libc::c_int >> 3 as libc::c_int >> 3 as libc::c_int)
                    as libc::c_uint != 0
            && is_colored(C_STICKY_OTHER_WRITABLE) as libc::c_int != 0
        {
            type_0 = C_STICKY_OTHER_WRITABLE;
        } else if mode
            & (0o200 as libc::c_int >> 3 as libc::c_int >> 3 as libc::c_int)
                as libc::c_uint != 0 as libc::c_int as libc::c_uint
            && is_colored(C_OTHER_WRITABLE) as libc::c_int != 0
        {
            type_0 = C_OTHER_WRITABLE;
        } else if mode & 0o1000 as libc::c_int as libc::c_uint
            != 0 as libc::c_int as libc::c_uint
            && is_colored(C_STICKY) as libc::c_int != 0
        {
            type_0 = C_STICKY;
        }
    } else if mode & 0o170000 as libc::c_int as libc::c_uint
        == 0o120000 as libc::c_int as libc::c_uint
    {
        type_0 = C_LINK;
    } else if mode & 0o170000 as libc::c_int as libc::c_uint
        == 0o10000 as libc::c_int as libc::c_uint
    {
        type_0 = C_FIFO;
    } else if mode & 0o170000 as libc::c_int as libc::c_uint
        == 0o140000 as libc::c_int as libc::c_uint
    {
        type_0 = C_SOCK;
    } else if mode & 0o170000 as libc::c_int as libc::c_uint
        == 0o60000 as libc::c_int as libc::c_uint
    {
        type_0 = C_BLK;
    } else if mode & 0o170000 as libc::c_int as libc::c_uint
        == 0o20000 as libc::c_int as libc::c_uint
    {
        type_0 = C_CHR;
    } else {
        type_0 = C_ORPHAN;
    }
    ext = 0 as *mut color_ext_type;
    if type_0 as libc::c_uint == C_FILE as libc::c_int as libc::c_uint {
        len = strlen(name);
        name = name.offset(len as isize);
        ext = color_ext_list;
        while !ext.is_null() {
            if (*ext).ext.len <= len
                && c_strncasecmp(
                    name.offset(-((*ext).ext.len as isize)),
                    (*ext).ext.string,
                    (*ext).ext.len,
                ) == 0 as libc::c_int
            {
                break;
            }
            ext = (*ext).next;
        }
    }
    if type_0 as libc::c_uint == C_LINK as libc::c_int as libc::c_uint && linkok == 0 {
        if color_symlink_as_referent as libc::c_int != 0
            || is_colored(C_ORPHAN) as libc::c_int != 0
        {
            type_0 = C_ORPHAN;
        }
    }
    let s: *const bin_str = if !ext.is_null() {
        &mut (*ext).seq
    } else {
        &mut *color_indicator.as_mut_ptr().offset(type_0 as isize) as *mut bin_str
    };
    return if !((*s).string).is_null() { s } else { 0 as *const bin_str };
}
unsafe extern "C" fn put_indicator(mut ind: *const bin_str) {
    if !used_color {
        used_color = 1 as libc::c_int != 0;
        if 0 as libc::c_int <= tcgetpgrp(1 as libc::c_int) {
            signal_init();
        }
        prep_non_filename_text();
    }
    if 0 != 0 && 0 != 0
        && ((*ind).len).wrapping_mul(1 as libc::c_int as size_t)
            <= 8 as libc::c_int as libc::c_ulong
        && (*ind).len != 0 as libc::c_int as libc::c_ulong
    {
        ({
            let mut __ptr: *const libc::c_char = (*ind).string;
            let mut __stream: *mut FILE = stdout;
            let mut __cnt: size_t = 0;
            __cnt = ((*ind).len).wrapping_mul(1 as libc::c_int as size_t);
            while __cnt > 0 as libc::c_int as libc::c_ulong {
                let fresh47 = __ptr;
                __ptr = __ptr.offset(1);
                if putc_unlocked(*fresh47 as libc::c_int, __stream)
                    == -(1 as libc::c_int)
                {
                    break;
                }
                __cnt = __cnt.wrapping_sub(1);
            }
            compile_error!("Binary expression is not supposed to be used")
        });
    } else {
        if 0 != 0 && (*ind).len == 0 as libc::c_int as libc::c_ulong
            || 0 != 0 && 1 as libc::c_int as size_t == 0 as libc::c_int as libc::c_ulong
        {} else {
            fwrite_unlocked(
                (*ind).string as *const libc::c_void,
                (*ind).len,
                1 as libc::c_int as size_t,
                stdout,
            );
        };
    };
}
unsafe extern "C" fn length_of_file_name_and_frills(mut f: *const fileinfo) -> size_t {
    let mut len: size_t = 0 as libc::c_int as size_t;
    let mut buf: [libc::c_char; 652] = [0; 652];
    if print_inode {
        len = (len as libc::c_ulong)
            .wrapping_add(
                (1 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (if format as libc::c_uint
                            == with_commas as libc::c_int as libc::c_uint
                        {
                            strlen(umaxtostr((*f).stat.st_ino, buf.as_mut_ptr()))
                        } else {
                            inode_number_width as libc::c_ulong
                        }),
                    ),
            ) as size_t as size_t;
    }
    if print_block_size {
        len = (len as libc::c_ulong)
            .wrapping_add(
                (1 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (if format as libc::c_uint
                            == with_commas as libc::c_int as libc::c_uint
                        {
                            strlen(
                                (if !(*f).stat_ok {
                                    b"?\0" as *const u8 as *const libc::c_char
                                } else {
                                    human_readable(
                                        (*f).stat.st_blocks as uintmax_t,
                                        buf.as_mut_ptr(),
                                        human_output_opts,
                                        512 as libc::c_int as uintmax_t,
                                        output_block_size,
                                    ) as *const libc::c_char
                                }),
                            )
                        } else {
                            block_size_width as libc::c_ulong
                        }),
                    ),
            ) as size_t as size_t;
    }
    if print_scontext {
        len = (len as libc::c_ulong)
            .wrapping_add(
                (1 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (if format as libc::c_uint
                            == with_commas as libc::c_int as libc::c_uint
                        {
                            strlen((*f).scontext)
                        } else {
                            scontext_width as libc::c_ulong
                        }),
                    ),
            ) as size_t as size_t;
    }
    len = (len as libc::c_ulong).wrapping_add(fileinfo_name_width(f)) as size_t
        as size_t;
    if indicator_style as libc::c_uint != none as libc::c_int as libc::c_uint {
        let mut c: libc::c_char = get_type_indicator(
            (*f).stat_ok,
            (*f).stat.st_mode,
            (*f).filetype,
        );
        len = (len as libc::c_ulong)
            .wrapping_add(
                (c as libc::c_int != 0 as libc::c_int) as libc::c_int as libc::c_ulong,
            ) as size_t as size_t;
    }
    return len;
}
unsafe extern "C" fn print_many_per_line() {
    let mut row: size_t = 0;
    let mut cols: size_t = calculate_columns(1 as libc::c_int != 0);
    let mut line_fmt: *const column_info = &mut *column_info
        .offset(cols.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize)
        as *mut column_info;
    let mut rows: size_t = cwd_n_used
        .wrapping_div(cols)
        .wrapping_add(
            (cwd_n_used.wrapping_rem(cols) != 0 as libc::c_int as libc::c_ulong)
                as libc::c_int as libc::c_ulong,
        );
    row = 0 as libc::c_int as size_t;
    while row < rows {
        let mut col: size_t = 0 as libc::c_int as size_t;
        let mut filesno: size_t = row;
        let mut pos: size_t = 0 as libc::c_int as size_t;
        loop {
            let mut f: *const fileinfo = *sorted_file.offset(filesno as isize)
                as *const fileinfo;
            let mut name_length: size_t = length_of_file_name_and_frills(f);
            let fresh48 = col;
            col = col.wrapping_add(1);
            let mut max_name_length: size_t = *((*line_fmt).col_arr)
                .offset(fresh48 as isize);
            print_file_name_and_frills(f, pos);
            filesno = (filesno as libc::c_ulong).wrapping_add(rows) as size_t as size_t;
            if filesno >= cwd_n_used {
                break;
            }
            indent(pos.wrapping_add(name_length), pos.wrapping_add(max_name_length));
            pos = (pos as libc::c_ulong).wrapping_add(max_name_length) as size_t
                as size_t;
        }
        putchar_unlocked(eolbyte as libc::c_int);
        row = row.wrapping_add(1);
    }
}
unsafe extern "C" fn print_horizontal() {
    let mut filesno: size_t = 0;
    let mut pos: size_t = 0 as libc::c_int as size_t;
    let mut cols: size_t = calculate_columns(0 as libc::c_int != 0);
    let mut line_fmt: *const column_info = &mut *column_info
        .offset(cols.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize)
        as *mut column_info;
    let mut f: *const fileinfo = *sorted_file.offset(0 as libc::c_int as isize)
        as *const fileinfo;
    let mut name_length: size_t = length_of_file_name_and_frills(f);
    let mut max_name_length: size_t = *((*line_fmt).col_arr)
        .offset(0 as libc::c_int as isize);
    print_file_name_and_frills(f, 0 as libc::c_int as size_t);
    filesno = 1 as libc::c_int as size_t;
    while filesno < cwd_n_used {
        let mut col: size_t = filesno.wrapping_rem(cols);
        if col == 0 as libc::c_int as libc::c_ulong {
            putchar_unlocked(eolbyte as libc::c_int);
            pos = 0 as libc::c_int as size_t;
        } else {
            indent(pos.wrapping_add(name_length), pos.wrapping_add(max_name_length));
            pos = (pos as libc::c_ulong).wrapping_add(max_name_length) as size_t
                as size_t;
        }
        f = *sorted_file.offset(filesno as isize) as *const fileinfo;
        print_file_name_and_frills(f, pos);
        name_length = length_of_file_name_and_frills(f);
        max_name_length = *((*line_fmt).col_arr).offset(col as isize);
        filesno = filesno.wrapping_add(1);
    }
    putchar_unlocked(eolbyte as libc::c_int);
}
unsafe extern "C" fn print_with_separator(mut sep: libc::c_char) {
    let mut filesno: size_t = 0;
    let mut pos: size_t = 0 as libc::c_int as size_t;
    filesno = 0 as libc::c_int as size_t;
    while filesno < cwd_n_used {
        let mut f: *const fileinfo = *sorted_file.offset(filesno as isize)
            as *const fileinfo;
        let mut len: size_t = if line_length != 0 {
            length_of_file_name_and_frills(f)
        } else {
            0 as libc::c_int as libc::c_ulong
        };
        if filesno != 0 as libc::c_int as libc::c_ulong {
            let mut separator: libc::c_char = 0;
            if line_length == 0
                || pos.wrapping_add(len).wrapping_add(2 as libc::c_int as libc::c_ulong)
                    < line_length
                    && pos
                        <= (18446744073709551615 as libc::c_ulong)
                            .wrapping_sub(len)
                            .wrapping_sub(2 as libc::c_int as libc::c_ulong)
            {
                pos = (pos as libc::c_ulong)
                    .wrapping_add(2 as libc::c_int as libc::c_ulong) as size_t as size_t;
                separator = ' ' as i32 as libc::c_char;
            } else {
                pos = 0 as libc::c_int as size_t;
                separator = eolbyte;
            }
            putchar_unlocked(sep as libc::c_int);
            putchar_unlocked(separator as libc::c_int);
        }
        print_file_name_and_frills(f, pos);
        pos = (pos as libc::c_ulong).wrapping_add(len) as size_t as size_t;
        filesno = filesno.wrapping_add(1);
    }
    putchar_unlocked(eolbyte as libc::c_int);
}
unsafe extern "C" fn indent(mut from: size_t, mut to: size_t) {
    while from < to {
        if tabsize != 0 as libc::c_int as libc::c_ulong
            && to.wrapping_div(tabsize)
                > from
                    .wrapping_add(1 as libc::c_int as libc::c_ulong)
                    .wrapping_div(tabsize)
        {
            putchar_unlocked('\t' as i32);
            from = (from as libc::c_ulong)
                .wrapping_add(tabsize.wrapping_sub(from.wrapping_rem(tabsize))) as size_t
                as size_t;
        } else {
            putchar_unlocked(' ' as i32);
            from = from.wrapping_add(1);
        }
    }
}
unsafe extern "C" fn attach(
    mut dest: *mut libc::c_char,
    mut dirname: *const libc::c_char,
    mut name: *const libc::c_char,
) {
    let mut dirnamep: *const libc::c_char = dirname;
    if *dirname.offset(0 as libc::c_int as isize) as libc::c_int != '.' as i32
        || *dirname.offset(1 as libc::c_int as isize) as libc::c_int != 0 as libc::c_int
    {
        while *dirnamep != 0 {
            let fresh49 = dirnamep;
            dirnamep = dirnamep.offset(1);
            let fresh50 = dest;
            dest = dest.offset(1);
            *fresh50 = *fresh49;
        }
        if dirnamep > dirname
            && *dirnamep.offset(-(1 as libc::c_int) as isize) as libc::c_int
                != '/' as i32
        {
            let fresh51 = dest;
            dest = dest.offset(1);
            *fresh51 = '/' as i32 as libc::c_char;
        }
    }
    while *name != 0 {
        let fresh52 = name;
        name = name.offset(1);
        let fresh53 = dest;
        dest = dest.offset(1);
        *fresh53 = *fresh52;
    }
    *dest = 0 as libc::c_int as libc::c_char;
}
unsafe extern "C" fn init_column_info(mut max_cols: size_t) {
    let mut i: size_t = 0;
    static mut column_info_alloc: size_t = 0;
    if column_info_alloc < max_cols {
        let mut new_column_info_alloc: size_t = 0;
        let mut p: *mut size_t = 0 as *mut size_t;
        if max_idx == 0
            || max_cols < max_idx.wrapping_div(2 as libc::c_int as libc::c_ulong)
        {
            column_info = xnrealloc(
                column_info as *mut libc::c_void,
                max_cols,
                (2 as libc::c_int as libc::c_ulong)
                    .wrapping_mul(::core::mem::size_of::<column_info>() as libc::c_ulong),
            ) as *mut column_info;
            new_column_info_alloc = (2 as libc::c_int as libc::c_ulong)
                .wrapping_mul(max_cols);
        } else {
            column_info = xnrealloc(
                column_info as *mut libc::c_void,
                max_idx,
                ::core::mem::size_of::<column_info>() as libc::c_ulong,
            ) as *mut column_info;
            new_column_info_alloc = max_idx;
        }
        let mut column_info_growth: size_t = new_column_info_alloc
            .wrapping_sub(column_info_alloc);
        let mut s: size_t = column_info_alloc
            .wrapping_add(1 as libc::c_int as libc::c_ulong)
            .wrapping_add(new_column_info_alloc);
        let mut t: size_t = s.wrapping_mul(column_info_growth);
        if s < new_column_info_alloc || t.wrapping_div(column_info_growth) != s {
            xalloc_die();
        }
        p = xnmalloc(
            t.wrapping_div(2 as libc::c_int as libc::c_ulong),
            ::core::mem::size_of::<size_t>() as libc::c_ulong,
        ) as *mut size_t;
        i = column_info_alloc;
        while i < new_column_info_alloc {
            let ref mut fresh54 = (*column_info.offset(i as isize)).col_arr;
            *fresh54 = p;
            p = p.offset(i.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize);
            i = i.wrapping_add(1);
        }
        column_info_alloc = new_column_info_alloc;
    }
    i = 0 as libc::c_int as size_t;
    while i < max_cols {
        let mut j: size_t = 0;
        (*column_info.offset(i as isize)).valid_len = 1 as libc::c_int != 0;
        (*column_info.offset(i as isize))
            .line_len = i
            .wrapping_add(1 as libc::c_int as libc::c_ulong)
            .wrapping_mul(MIN_COLUMN_WIDTH as libc::c_int as libc::c_ulong);
        j = 0 as libc::c_int as size_t;
        while j <= i {
            *((*column_info.offset(i as isize)).col_arr)
                .offset(j as isize) = MIN_COLUMN_WIDTH as libc::c_int as size_t;
            j = j.wrapping_add(1);
        }
        i = i.wrapping_add(1);
    }
}
unsafe extern "C" fn calculate_columns(mut by_columns: bool) -> size_t {
    let mut filesno: size_t = 0;
    let mut cols: size_t = 0;
    let mut max_cols: size_t = if (0 as libc::c_int as libc::c_ulong) < max_idx
        && max_idx < cwd_n_used
    {
        max_idx
    } else {
        cwd_n_used
    };
    init_column_info(max_cols);
    filesno = 0 as libc::c_int as size_t;
    while filesno < cwd_n_used {
        let mut f: *const fileinfo = *sorted_file.offset(filesno as isize)
            as *const fileinfo;
        let mut name_length: size_t = length_of_file_name_and_frills(f);
        let mut i: size_t = 0 as libc::c_int as size_t;
        while i < max_cols {
            if (*column_info.offset(i as isize)).valid_len {
                let mut idx: size_t = if by_columns as libc::c_int != 0 {
                    filesno
                        .wrapping_div(
                            cwd_n_used
                                .wrapping_add(i)
                                .wrapping_div(
                                    i.wrapping_add(1 as libc::c_int as libc::c_ulong),
                                ),
                        )
                } else {
                    filesno
                        .wrapping_rem(i.wrapping_add(1 as libc::c_int as libc::c_ulong))
                };
                let mut real_length: size_t = name_length
                    .wrapping_add(
                        (if idx == i { 0 as libc::c_int } else { 2 as libc::c_int })
                            as libc::c_ulong,
                    );
                if *((*column_info.offset(i as isize)).col_arr).offset(idx as isize)
                    < real_length
                {
                    let ref mut fresh55 = (*column_info.offset(i as isize)).line_len;
                    *fresh55 = (*fresh55 as libc::c_ulong)
                        .wrapping_add(
                            real_length
                                .wrapping_sub(
                                    *((*column_info.offset(i as isize)).col_arr)
                                        .offset(idx as isize),
                                ),
                        ) as size_t as size_t;
                    *((*column_info.offset(i as isize)).col_arr)
                        .offset(idx as isize) = real_length;
                    (*column_info.offset(i as isize))
                        .valid_len = (*column_info.offset(i as isize)).line_len
                        < line_length;
                }
            }
            i = i.wrapping_add(1);
        }
        filesno = filesno.wrapping_add(1);
    }
    cols = max_cols;
    while (1 as libc::c_int as libc::c_ulong) < cols {
        if (*column_info
            .offset(cols.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize))
            .valid_len
        {
            break;
        }
        cols = cols.wrapping_sub(1);
    }
    return cols;
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
                b"List information about the FILEs (the current directory by default).\nSort entries alphabetically if none of -cftuvSUX nor --sort is specified.\n\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            stdout,
        );
        emit_mandatory_arg_note();
        fputs_unlocked(
            dcgettext(
                0 as *const libc::c_char,
                b"  -a, --all                  do not ignore entries starting with .\n  -A, --almost-all           do not list implied . and ..\n      --author               with -l, print the author of each file\n  -b, --escape               print C-style escapes for nongraphic characters\n\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            stdout,
        );
        fputs_unlocked(
            dcgettext(
                0 as *const libc::c_char,
                b"      --block-size=SIZE      with -l, scale sizes by SIZE when printing them;\n                             e.g., '--block-size=M'; see SIZE format below\n\n\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            stdout,
        );
        fputs_unlocked(
            dcgettext(
                0 as *const libc::c_char,
                b"  -B, --ignore-backups       do not list implied entries ending with ~\n\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            stdout,
        );
        fputs_unlocked(
            dcgettext(
                0 as *const libc::c_char,
                b"  -c                         with -lt: sort by, and show, ctime (time of last\n                             change of file status information);\n                             with -l: show ctime and sort by name;\n                             otherwise: sort by ctime, newest first\n\n\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            stdout,
        );
        fputs_unlocked(
            dcgettext(
                0 as *const libc::c_char,
                b"  -C                         list entries by columns\n      --color[=WHEN]         color the output WHEN; more info below\n  -d, --directory            list directories themselves, not their contents\n  -D, --dired                generate output designed for Emacs' dired mode\n\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            stdout,
        );
        fputs_unlocked(
            dcgettext(
                0 as *const libc::c_char,
                b"  -f                         list all entries in directory order\n  -F, --classify[=WHEN]      append indicator (one of */=>@|) to entries WHEN\n      --file-type            likewise, except do not append '*'\n\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            stdout,
        );
        fputs_unlocked(
            dcgettext(
                0 as *const libc::c_char,
                b"      --format=WORD          across -x, commas -m, horizontal -x, long -l,\n                             single-column -1, verbose -l, vertical -C\n\n\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            stdout,
        );
        fputs_unlocked(
            dcgettext(
                0 as *const libc::c_char,
                b"      --full-time            like -l --time-style=full-iso\n\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            stdout,
        );
        fputs_unlocked(
            dcgettext(
                0 as *const libc::c_char,
                b"  -g                         like -l, but do not list owner\n\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            stdout,
        );
        fputs_unlocked(
            dcgettext(
                0 as *const libc::c_char,
                b"      --group-directories-first\n                             group directories before files;\n                             can be augmented with a --sort option, but any\n                             use of --sort=none (-U) disables grouping\n\n\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            stdout,
        );
        fputs_unlocked(
            dcgettext(
                0 as *const libc::c_char,
                b"  -G, --no-group             in a long listing, don't print group names\n\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            stdout,
        );
        fputs_unlocked(
            dcgettext(
                0 as *const libc::c_char,
                b"  -h, --human-readable       with -l and -s, print sizes like 1K 234M 2G etc.\n      --si                   likewise, but use powers of 1000 not 1024\n\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            stdout,
        );
        fputs_unlocked(
            dcgettext(
                0 as *const libc::c_char,
                b"  -H, --dereference-command-line\n                             follow symbolic links listed on the command line\n\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            stdout,
        );
        fputs_unlocked(
            dcgettext(
                0 as *const libc::c_char,
                b"      --dereference-command-line-symlink-to-dir\n                             follow each command line symbolic link\n                             that points to a directory\n\n\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            stdout,
        );
        fputs_unlocked(
            dcgettext(
                0 as *const libc::c_char,
                b"      --hide=PATTERN         do not list implied entries matching shell PATTERN\n                             (overridden by -a or -A)\n\n\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            stdout,
        );
        fputs_unlocked(
            dcgettext(
                0 as *const libc::c_char,
                b"      --hyperlink[=WHEN]     hyperlink file names WHEN\n\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            stdout,
        );
        fputs_unlocked(
            dcgettext(
                0 as *const libc::c_char,
                b"      --indicator-style=WORD\n                             append indicator with style WORD to entry names:\n                             none (default), slash (-p),\n                             file-type (--file-type), classify (-F)\n\n\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            stdout,
        );
        fputs_unlocked(
            dcgettext(
                0 as *const libc::c_char,
                b"  -i, --inode                print the index number of each file\n  -I, --ignore=PATTERN       do not list implied entries matching shell PATTERN\n\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            stdout,
        );
        fputs_unlocked(
            dcgettext(
                0 as *const libc::c_char,
                b"  -k, --kibibytes            default to 1024-byte blocks for file system usage;\n                             used only with -s and per directory totals\n\n\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            stdout,
        );
        fputs_unlocked(
            dcgettext(
                0 as *const libc::c_char,
                b"  -l                         use a long listing format\n\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            stdout,
        );
        fputs_unlocked(
            dcgettext(
                0 as *const libc::c_char,
                b"  -L, --dereference          when showing file information for a symbolic\n                             link, show information for the file the link\n                             references rather than for the link itself\n\n\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            stdout,
        );
        fputs_unlocked(
            dcgettext(
                0 as *const libc::c_char,
                b"  -m                         fill width with a comma separated list of entries\n\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            stdout,
        );
        fputs_unlocked(
            dcgettext(
                0 as *const libc::c_char,
                b"  -n, --numeric-uid-gid      like -l, but list numeric user and group IDs\n  -N, --literal              print entry names without quoting\n  -o                         like -l, but do not list group information\n  -p, --indicator-style=slash\n                             append / indicator to directories\n\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            stdout,
        );
        fputs_unlocked(
            dcgettext(
                0 as *const libc::c_char,
                b"  -q, --hide-control-chars   print ? instead of nongraphic characters\n\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            stdout,
        );
        fputs_unlocked(
            dcgettext(
                0 as *const libc::c_char,
                b"      --show-control-chars   show nongraphic characters as-is (the default,\n                             unless program is 'ls' and output is a terminal)\n\n\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            stdout,
        );
        fputs_unlocked(
            dcgettext(
                0 as *const libc::c_char,
                b"  -Q, --quote-name           enclose entry names in double quotes\n\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            stdout,
        );
        fputs_unlocked(
            dcgettext(
                0 as *const libc::c_char,
                b"      --quoting-style=WORD   use quoting style WORD for entry names:\n                             literal, locale, shell, shell-always,\n                             shell-escape, shell-escape-always, c, escape\n                             (overrides QUOTING_STYLE environment variable)\n\n\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            stdout,
        );
        fputs_unlocked(
            dcgettext(
                0 as *const libc::c_char,
                b"  -r, --reverse              reverse order while sorting\n  -R, --recursive            list subdirectories recursively\n  -s, --size                 print the allocated size of each file, in blocks\n\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            stdout,
        );
        fputs_unlocked(
            dcgettext(
                0 as *const libc::c_char,
                b"  -S                         sort by file size, largest first\n\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            stdout,
        );
        fputs_unlocked(
            dcgettext(
                0 as *const libc::c_char,
                b"      --sort=WORD            sort by WORD instead of name: none (-U), size (-S),\n                             time (-t), version (-v), extension (-X), width\n\n\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            stdout,
        );
        fputs_unlocked(
            dcgettext(
                0 as *const libc::c_char,
                b"      --time=WORD            select which timestamp used to display or sort;\n                               access time (-u): atime, access, use;\n                               metadata change time (-c): ctime, status;\n                               modified time (default): mtime, modification;\n                               birth time: birth, creation;\n                             with -l, WORD determines which time to show;\n                             with --sort=time, sort by WORD (newest first)\n\n\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            stdout,
        );
        fputs_unlocked(
            dcgettext(
                0 as *const libc::c_char,
                b"      --time-style=TIME_STYLE\n                             time/date format with -l; see TIME_STYLE below\n\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            stdout,
        );
        fputs_unlocked(
            dcgettext(
                0 as *const libc::c_char,
                b"  -t                         sort by time, newest first; see --time\n  -T, --tabsize=COLS         assume tab stops at each COLS instead of 8\n\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            stdout,
        );
        fputs_unlocked(
            dcgettext(
                0 as *const libc::c_char,
                b"  -u                         with -lt: sort by, and show, access time;\n                             with -l: show access time and sort by name;\n                             otherwise: sort by access time, newest first\n\n\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            stdout,
        );
        fputs_unlocked(
            dcgettext(
                0 as *const libc::c_char,
                b"  -U                         do not sort; list entries in directory order\n\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            stdout,
        );
        fputs_unlocked(
            dcgettext(
                0 as *const libc::c_char,
                b"  -v                         natural sort of (version) numbers within text\n\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            stdout,
        );
        fputs_unlocked(
            dcgettext(
                0 as *const libc::c_char,
                b"  -w, --width=COLS           set output width to COLS.  0 means no limit\n  -x                         list entries by lines instead of by columns\n  -X                         sort alphabetically by entry extension\n  -Z, --context              print any security context of each file\n      --zero                 end each output line with NUL, not newline\n  -1                         list one file per line\n\0"
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
        emit_size_note();
        fputs_unlocked(
            dcgettext(
                0 as *const libc::c_char,
                b"\nThe TIME_STYLE argument can be full-iso, long-iso, iso, locale, or +FORMAT.\nFORMAT is interpreted like in date(1).  If FORMAT is FORMAT1<newline>FORMAT2,\nthen FORMAT1 applies to non-recent files and FORMAT2 to recent files.\nTIME_STYLE prefixed with 'posix-' takes effect only outside the POSIX locale.\nAlso the TIME_STYLE environment variable sets the default style to use.\n\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            stdout,
        );
        fputs_unlocked(
            dcgettext(
                0 as *const libc::c_char,
                b"\nThe WHEN argument defaults to 'always' and can also be 'auto' or 'never'.\n\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            stdout,
        );
        fputs_unlocked(
            dcgettext(
                0 as *const libc::c_char,
                b"\nUsing color to distinguish file types is disabled both by default and\nwith --color=never.  With --color=auto, ls emits color codes only when\nstandard output is connected to a terminal.  The LS_COLORS environment\nvariable can change the settings.  Use the dircolors(1) command to set it.\n\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            stdout,
        );
        fputs_unlocked(
            dcgettext(
                0 as *const libc::c_char,
                b"\nExit status:\n 0  if OK,\n 1  if minor problems (e.g., cannot access subdirectory),\n 2  if serious trouble (e.g., cannot access command-line argument).\n\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            stdout,
        );
        emit_ancillary_info(
            if ls_mode == 1 as libc::c_int {
                b"ls\0" as *const u8 as *const libc::c_char
            } else if ls_mode == 2 as libc::c_int {
                b"dir\0" as *const u8 as *const libc::c_char
            } else {
                b"vdir\0" as *const u8 as *const libc::c_char
            },
        );
    }
    exit(status);
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
    color_indicator = [
        {
            let mut init = bin_str {
                len: (::core::mem::size_of::<[libc::c_char; 3]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                string: b"\x1B[\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = bin_str {
                len: (::core::mem::size_of::<[libc::c_char; 2]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                string: b"m\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = bin_str {
                len: 0 as libc::c_int as size_t,
                string: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = bin_str {
                len: (::core::mem::size_of::<[libc::c_char; 2]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                string: b"0\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = bin_str {
                len: 0 as libc::c_int as size_t,
                string: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = bin_str {
                len: 0 as libc::c_int as size_t,
                string: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = bin_str {
                len: (::core::mem::size_of::<[libc::c_char; 6]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                string: b"01;34\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = bin_str {
                len: (::core::mem::size_of::<[libc::c_char; 6]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                string: b"01;36\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = bin_str {
                len: (::core::mem::size_of::<[libc::c_char; 3]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                string: b"33\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = bin_str {
                len: (::core::mem::size_of::<[libc::c_char; 6]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                string: b"01;35\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = bin_str {
                len: (::core::mem::size_of::<[libc::c_char; 6]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                string: b"01;33\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = bin_str {
                len: (::core::mem::size_of::<[libc::c_char; 6]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                string: b"01;33\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = bin_str {
                len: 0 as libc::c_int as size_t,
                string: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = bin_str {
                len: 0 as libc::c_int as size_t,
                string: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = bin_str {
                len: (::core::mem::size_of::<[libc::c_char; 6]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                string: b"01;32\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = bin_str {
                len: (::core::mem::size_of::<[libc::c_char; 6]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                string: b"01;35\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = bin_str {
                len: (::core::mem::size_of::<[libc::c_char; 6]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                string: b"37;41\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = bin_str {
                len: (::core::mem::size_of::<[libc::c_char; 6]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                string: b"30;43\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = bin_str {
                len: (::core::mem::size_of::<[libc::c_char; 6]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                string: b"37;44\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = bin_str {
                len: (::core::mem::size_of::<[libc::c_char; 6]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                string: b"34;42\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = bin_str {
                len: (::core::mem::size_of::<[libc::c_char; 6]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                string: b"30;42\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = bin_str {
                len: 0 as libc::c_int as size_t,
                string: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = bin_str {
                len: 0 as libc::c_int as size_t,
                string: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = bin_str {
                len: (::core::mem::size_of::<[libc::c_char; 4]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                string: b"\x1B[K\0" as *const u8 as *const libc::c_char,
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
