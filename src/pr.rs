#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![feature(extern_types)]
extern "C" {
    pub type tm_zone;
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
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
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn free(_: *mut libc::c_void);
    fn __errno_location() -> *mut libc::c_int;
    fn fstat(__fd: libc::c_int, __buf: *mut stat) -> libc::c_int;
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
    fn xmalloc(s: size_t) -> *mut libc::c_void;
    fn x2realloc(p: *mut libc::c_void, ps: *mut size_t) -> *mut libc::c_void;
    fn xnmalloc(n: size_t, s: size_t) -> *mut libc::c_void;
    static mut stdin: *mut FILE;
    static mut stdout: *mut FILE;
    static mut stderr: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn __uflow(_: *mut FILE) -> libc::c_int;
    fn __overflow(_: *mut FILE, _: libc::c_int) -> libc::c_int;
    fn ungetc(__c: libc::c_int, __stream: *mut FILE) -> libc::c_int;
    fn fputs_unlocked(__s: *const libc::c_char, __stream: *mut FILE) -> libc::c_int;
    fn clearerr_unlocked(__stream: *mut FILE);
    fn fileno(__stream: *mut FILE) -> libc::c_int;
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
    fn quotearg_n_style_colon(
        n: libc::c_int,
        s: quoting_style,
        arg: *const libc::c_char,
    ) -> *mut libc::c_char;
    fn gettime(_: *mut timespec);
    fn error(
        __status: libc::c_int,
        __errnum: libc::c_int,
        __format: *const libc::c_char,
        _: ...
    );
    fn fadvise(fp: *mut FILE, advice: fadvice_t);
    fn hard_locale(category: libc::c_int) -> bool;
    fn gnu_mbswidth(string: *const libc::c_char, flags: libc::c_int) -> libc::c_int;
    fn quote(arg: *const libc::c_char) -> *const libc::c_char;
    fn fopen_safer(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
    fn nstrftime(
        _: *mut libc::c_char,
        _: size_t,
        _: *const libc::c_char,
        _: *const tm,
        __tz: timezone_t,
        __ns: libc::c_int,
    ) -> size_t;
    fn xstrtol(
        _: *const libc::c_char,
        _: *mut *mut libc::c_char,
        _: libc::c_int,
        _: *mut libc::c_long,
        _: *const libc::c_char,
    ) -> strtol_error;
    fn xstrtoumax(
        _: *const libc::c_char,
        _: *mut *mut libc::c_char,
        _: libc::c_int,
        _: *mut uintmax_t,
        _: *const libc::c_char,
    ) -> strtol_error;
    fn xstrtol_fatal(
        _: strtol_error,
        _: libc::c_int,
        _: libc::c_char,
        _: *const option,
        _: *const libc::c_char,
    );
    fn xdectoimax(
        n_str: *const libc::c_char,
        min: intmax_t,
        max: intmax_t,
        suffixes: *const libc::c_char,
        err: *const libc::c_char,
        err_exit: libc::c_int,
    ) -> intmax_t;
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
pub type fadvice_t = libc::c_uint;
pub const FADVISE_RANDOM: fadvice_t = 1;
pub const FADVISE_WILLNEED: fadvice_t = 3;
pub const FADVISE_DONTNEED: fadvice_t = 4;
pub const FADVISE_NOREUSE: fadvice_t = 5;
pub const FADVISE_SEQUENTIAL: fadvice_t = 2;
pub const FADVISE_NORMAL: fadvice_t = 0;
pub type strtol_error = libc::c_uint;
pub const LONGINT_INVALID: strtol_error = 4;
pub const LONGINT_INVALID_SUFFIX_CHAR_WITH_OVERFLOW: strtol_error = 3;
pub const LONGINT_INVALID_SUFFIX_CHAR: strtol_error = 2;
pub const LONGINT_OVERFLOW: strtol_error = 1;
pub const LONGINT_OK: strtol_error = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct COLUMN {
    pub fp: *mut FILE,
    pub name: *const libc::c_char,
    pub status: C2RustUnnamed_1,
    pub print_func: Option::<unsafe extern "C" fn(*mut COLUMN) -> bool>,
    pub char_func: Option::<unsafe extern "C" fn(libc::c_char) -> ()>,
    pub current_line: libc::c_int,
    pub lines_stored: libc::c_int,
    pub lines_to_print: libc::c_int,
    pub start_position: libc::c_int,
    pub numbered: bool,
    pub full_page_printed: bool,
}
pub type C2RustUnnamed_1 = libc::c_uint;
pub const CLOSED: C2RustUnnamed_1 = 3;
pub const ON_HOLD: C2RustUnnamed_1 = 2;
pub const FF_FOUND: C2RustUnnamed_1 = 1;
pub const OPEN: C2RustUnnamed_1 = 0;
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
pub const lines_per_footer: C2RustUnnamed_8 = 5;
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
pub const lines_per_header: C2RustUnnamed_7 = 5;
pub type C2RustUnnamed_7 = libc::c_uint;
pub type C2RustUnnamed_8 = libc::c_uint;
pub type C2RustUnnamed_9 = libc::c_uint;
pub const PAGES_OPTION: C2RustUnnamed_9 = 257;
pub const COLUMNS_OPTION: C2RustUnnamed_9 = 256;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_10 {
    pub _gl_dummy: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_11 {
    pub _gl_dummy: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_12 {
    pub _gl_dummy: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_13 {
    pub _gl_dummy: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_14 {
    pub _gl_dummy: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_15 {
    pub _gl_dummy: libc::c_int,
}
#[inline]
unsafe extern "C" fn to_uchar(mut ch: libc::c_char) -> libc::c_uchar {
    return ch as libc::c_uchar;
}
#[inline]
unsafe extern "C" fn getc_unlocked(mut __fp: *mut FILE) -> libc::c_int {
    return if ((*__fp)._IO_read_ptr >= (*__fp)._IO_read_end) as libc::c_int
        as libc::c_long != 0
    {
        __uflow(__fp)
    } else {
        let fresh0 = (*__fp)._IO_read_ptr;
        (*__fp)._IO_read_ptr = ((*__fp)._IO_read_ptr).offset(1);
        *(fresh0 as *mut libc::c_uchar) as libc::c_int
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
unsafe extern "C" fn ferror_unlocked(mut __stream: *mut FILE) -> libc::c_int {
    return ((*__stream)._flags & 0x20 as libc::c_int != 0 as libc::c_int) as libc::c_int;
}
#[inline]
unsafe extern "C" fn emit_stdin_note() {
    fputs_unlocked(
        dcgettext(
            0 as *const libc::c_char,
            b"\nWith no FILE, or when FILE is -, read standard input.\n\0" as *const u8
                as *const libc::c_char,
            5 as libc::c_int,
        ),
        stdout,
    );
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
unsafe extern "C" fn get_stat_mtime(mut st: *const stat) -> timespec {
    return (*st).st_mtim;
}
static mut column_vector: *mut COLUMN = 0 as *const COLUMN as *mut COLUMN;
static mut buff: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
static mut buff_current: libc::c_uint = 0;
static mut buff_allocated: size_t = 0;
static mut line_vector: *mut libc::c_int = 0 as *const libc::c_int as *mut libc::c_int;
static mut end_vector: *mut libc::c_int = 0 as *const libc::c_int as *mut libc::c_int;
static mut parallel_files: bool = 0 as libc::c_int != 0;
static mut align_empty_cols: bool = false;
static mut empty_line: bool = false;
static mut FF_only: bool = false;
static mut explicit_columns: bool = 0 as libc::c_int != 0;
static mut extremities: bool = 1 as libc::c_int != 0;
static mut keep_FF: bool = 0 as libc::c_int != 0;
static mut print_a_FF: bool = 0 as libc::c_int != 0;
static mut print_a_header: bool = false;
static mut use_form_feed: bool = 0 as libc::c_int != 0;
static mut have_read_stdin: bool = 0 as libc::c_int != 0;
static mut print_across_flag: bool = 0 as libc::c_int != 0;
static mut storing_columns: bool = 1 as libc::c_int != 0;
static mut balance_columns: bool = 0 as libc::c_int != 0;
static mut lines_per_page: libc::c_int = 66 as libc::c_int;
static mut lines_per_body: libc::c_int = 0;
static mut chars_per_line: libc::c_int = 72 as libc::c_int;
static mut truncate_lines: bool = 0 as libc::c_int != 0;
static mut join_lines: bool = 0 as libc::c_int != 0;
static mut chars_per_column: libc::c_int = 0;
static mut untabify_input: bool = 0 as libc::c_int != 0;
static mut input_tab_char: libc::c_char = '\t' as i32 as libc::c_char;
static mut chars_per_input_tab: libc::c_int = 8 as libc::c_int;
static mut tabify_output: bool = 0 as libc::c_int != 0;
static mut output_tab_char: libc::c_char = '\t' as i32 as libc::c_char;
static mut chars_per_output_tab: libc::c_int = 8 as libc::c_int;
static mut spaces_not_printed: libc::c_int = 0;
static mut chars_per_margin: libc::c_int = 0 as libc::c_int;
static mut output_position: libc::c_int = 0;
static mut input_position: libc::c_int = 0;
static mut failed_opens: bool = 0 as libc::c_int != 0;
static mut columns: libc::c_int = 1 as libc::c_int;
static mut first_page_number: uintmax_t = 0 as libc::c_int as uintmax_t;
static mut last_page_number: uintmax_t = 18446744073709551615 as libc::c_ulong;
static mut files_ready_to_read: libc::c_int = 0 as libc::c_int;
static mut page_number: uintmax_t = 0;
static mut line_number: libc::c_int = 0;
static mut numbered_lines: bool = 0 as libc::c_int != 0;
static mut number_separator: libc::c_char = '\t' as i32 as libc::c_char;
static mut line_count: libc::c_int = 1 as libc::c_int;
static mut skip_count: bool = 1 as libc::c_int != 0;
static mut start_line_num: libc::c_int = 1 as libc::c_int;
static mut chars_per_number: libc::c_int = 5 as libc::c_int;
static mut number_width: libc::c_int = 0;
static mut number_buff: *mut libc::c_char = 0 as *const libc::c_char
    as *mut libc::c_char;
static mut use_esc_sequence: bool = 0 as libc::c_int != 0;
static mut use_cntrl_prefix: bool = 0 as libc::c_int != 0;
static mut double_space: bool = 0 as libc::c_int != 0;
static mut total_files: libc::c_int = 0 as libc::c_int;
static mut ignore_failed_opens: bool = 0 as libc::c_int != 0;
static mut use_col_separator: bool = 0 as libc::c_int != 0;
static mut col_sep_string: *const libc::c_char = b"\0" as *const u8
    as *const libc::c_char;
static mut col_sep_length: libc::c_int = 0 as libc::c_int;
static mut column_separator: *mut libc::c_char = b" \0" as *const u8
    as *const libc::c_char as *mut libc::c_char;
static mut line_separator: *mut libc::c_char = b"\t\0" as *const u8
    as *const libc::c_char as *mut libc::c_char;
static mut separators_not_printed: libc::c_int = 0;
static mut padding_not_printed: libc::c_int = 0;
static mut pad_vertically: bool = false;
static mut custom_header: *mut libc::c_char = 0 as *const libc::c_char
    as *mut libc::c_char;
static mut date_format: *const libc::c_char = 0 as *const libc::c_char;
static mut localtz: timezone_t = 0 as *const tm_zone as *mut tm_zone;
static mut date_text: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
static mut file_text: *const libc::c_char = 0 as *const libc::c_char;
static mut header_width_available: libc::c_int = 0;
static mut clump_buff: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
static mut last_line: bool = 0 as libc::c_int != 0;
static mut short_options: [libc::c_char; 53] = unsafe {
    *::core::mem::transmute::<
        &[u8; 53],
        &[libc::c_char; 53],
    >(b"-0123456789D:FJN:S::TW:abcde::fh:i::l:mn::o:rs::tvw:\0")
};
static mut long_options: [option; 27] = [
    {
        let mut init = option {
            name: b"pages\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: PAGES_OPTION as libc::c_int,
        };
        init
    },
    {
        let mut init = option {
            name: b"columns\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: COLUMNS_OPTION as libc::c_int,
        };
        init
    },
    {
        let mut init = option {
            name: b"across\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'a' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"show-control-chars\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'c' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"double-space\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'd' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"date-format\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'D' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"expand-tabs\0" as *const u8 as *const libc::c_char,
            has_arg: 2 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'e' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"form-feed\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'f' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"header\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'h' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"output-tabs\0" as *const u8 as *const libc::c_char,
            has_arg: 2 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'i' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"join-lines\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'J' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"length\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'l' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"merge\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'm' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"number-lines\0" as *const u8 as *const libc::c_char,
            has_arg: 2 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'n' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"first-line-number\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'N' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"indent\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'o' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"no-file-warnings\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'r' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"separator\0" as *const u8 as *const libc::c_char,
            has_arg: 2 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 's' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"sep-string\0" as *const u8 as *const libc::c_char,
            has_arg: 2 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'S' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"omit-header\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 't' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"omit-pagination\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'T' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"show-nonprinting\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'v' as i32,
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
            name: b"page-width\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'W' as i32,
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
unsafe extern "C" fn integer_overflow() {
    if ::core::mem::size_of::<C2RustUnnamed_5>() as libc::c_ulong != 0 {
        error(
            1 as libc::c_int,
            0 as libc::c_int,
            dcgettext(
                0 as *const libc::c_char,
                b"integer overflow\0" as *const u8 as *const libc::c_char,
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
                b"integer overflow\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
        if 0 as libc::c_int != 0 {} else {
            unreachable!();
        };
    };
}
unsafe extern "C" fn cols_ready_to_print() -> libc::c_uint {
    let mut q: *mut COLUMN = 0 as *mut COLUMN;
    let mut i: libc::c_uint = 0;
    let mut n: libc::c_uint = 0;
    n = 0 as libc::c_int as libc::c_uint;
    q = column_vector;
    i = 0 as libc::c_int as libc::c_uint;
    while i < columns as libc::c_uint {
        if (*q).status as libc::c_uint == OPEN as libc::c_int as libc::c_uint
            || (*q).status as libc::c_uint == FF_FOUND as libc::c_int as libc::c_uint
            || storing_columns as libc::c_int != 0
                && (*q).lines_stored > 0 as libc::c_int
                && (*q).lines_to_print > 0 as libc::c_int
        {
            n = n.wrapping_add(1);
        }
        q = q.offset(1);
        i = i.wrapping_add(1);
    }
    return n;
}
unsafe extern "C" fn first_last_page(
    mut oi: libc::c_int,
    mut c: libc::c_char,
    mut pages: *const libc::c_char,
) -> bool {
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut first: uintmax_t = 0;
    let mut last: uintmax_t = 18446744073709551615 as libc::c_ulong;
    let mut err: strtol_error = xstrtoumax(
        pages,
        &mut p,
        10 as libc::c_int,
        &mut first,
        b"\0" as *const u8 as *const libc::c_char,
    );
    if err as libc::c_uint != LONGINT_OK as libc::c_int as libc::c_uint
        && err as libc::c_uint
            != LONGINT_INVALID_SUFFIX_CHAR as libc::c_int as libc::c_uint
    {
        xstrtol_fatal(err, oi, c, long_options.as_ptr(), pages);
    }
    if p == pages as *mut libc::c_char || first == 0 {
        return 0 as libc::c_int != 0;
    }
    if *p as libc::c_int == ':' as i32 {
        let mut p1: *const libc::c_char = p.offset(1 as libc::c_int as isize);
        err = xstrtoumax(
            p1,
            &mut p,
            10 as libc::c_int,
            &mut last,
            b"\0" as *const u8 as *const libc::c_char,
        );
        if err as libc::c_uint != LONGINT_OK as libc::c_int as libc::c_uint {
            xstrtol_fatal(err, oi, c, long_options.as_ptr(), pages);
        }
        if p1 == p as *const libc::c_char || last < first {
            return 0 as libc::c_int != 0;
        }
    }
    if *p != 0 {
        return 0 as libc::c_int != 0;
    }
    first_page_number = first;
    last_page_number = last;
    return 1 as libc::c_int != 0;
}
unsafe extern "C" fn parse_column_count(mut s: *const libc::c_char) {
    getoptnum(
        s,
        1 as libc::c_int,
        &mut columns,
        dcgettext(
            0 as *const libc::c_char,
            b"invalid number of columns\0" as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        ),
    );
    explicit_columns = 1 as libc::c_int != 0;
}
unsafe extern "C" fn separator_string(mut optarg_S: *const libc::c_char) {
    let mut len: size_t = strlen(optarg_S);
    if (2147483647 as libc::c_int as libc::c_ulong) < len {
        integer_overflow();
    }
    col_sep_length = len as libc::c_int;
    col_sep_string = optarg_S;
}
unsafe fn main_0(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut n_files: libc::c_uint = 0;
    let mut old_options: bool = 0 as libc::c_int != 0;
    let mut old_w: bool = 0 as libc::c_int != 0;
    let mut old_s: bool = 0 as libc::c_int != 0;
    let mut file_names: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    let mut column_count_string: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut n_digits: size_t = 0 as libc::c_int as size_t;
    let mut n_alloc: size_t = 0 as libc::c_int as size_t;
    set_program_name(*argv.offset(0 as libc::c_int as isize));
    setlocale(6 as libc::c_int, b"\0" as *const u8 as *const libc::c_char);
    bindtextdomain(
        b"coreutils\0" as *const u8 as *const libc::c_char,
        b"/usr/local/share/locale\0" as *const u8 as *const libc::c_char,
    );
    textdomain(b"coreutils\0" as *const u8 as *const libc::c_char);
    atexit(Some(close_stdout as unsafe extern "C" fn() -> ()));
    n_files = 0 as libc::c_int as libc::c_uint;
    file_names = (if argc > 1 as libc::c_int {
        xnmalloc(
            (argc - 1 as libc::c_int) as size_t,
            ::core::mem::size_of::<*mut libc::c_char>() as libc::c_ulong,
        )
    } else {
        0 as *mut libc::c_void
    }) as *mut *mut libc::c_char;
    loop {
        let mut oi: libc::c_int = -(1 as libc::c_int);
        let mut c: libc::c_int = getopt_long(
            argc,
            argv,
            short_options.as_ptr(),
            long_options.as_ptr(),
            &mut oi,
        );
        if c == -(1 as libc::c_int) {
            break;
        }
        if (c as libc::c_uint).wrapping_sub('0' as i32 as libc::c_uint)
            <= 9 as libc::c_int as libc::c_uint
        {
            if n_digits.wrapping_add(1 as libc::c_int as libc::c_ulong) >= n_alloc {
                column_count_string = (if ::core::mem::size_of::<C2RustUnnamed_15>()
                    as libc::c_ulong != 0
                {
                    x2realloc(column_count_string as *mut libc::c_void, &mut n_alloc)
                } else {
                    x2realloc(column_count_string as *mut libc::c_void, &mut n_alloc)
                }) as *mut libc::c_char;
            }
            let fresh2 = n_digits;
            n_digits = n_digits.wrapping_add(1);
            *column_count_string.offset(fresh2 as isize) = c as libc::c_char;
            *column_count_string.offset(n_digits as isize) = '\0' as i32 as libc::c_char;
        } else {
            n_digits = 0 as libc::c_int as size_t;
            match c {
                1 => {
                    if !(first_page_number == 0 as libc::c_int as libc::c_ulong
                        && *optarg as libc::c_int == '+' as i32
                        && first_last_page(
                            -(2 as libc::c_int),
                            '+' as i32 as libc::c_char,
                            optarg.offset(1 as libc::c_int as isize),
                        ) as libc::c_int != 0)
                    {
                        let fresh3 = n_files;
                        n_files = n_files.wrapping_add(1);
                        let ref mut fresh4 = *file_names.offset(fresh3 as isize);
                        *fresh4 = optarg;
                    }
                }
                257 => {
                    if optarg.is_null() {
                        if ::core::mem::size_of::<C2RustUnnamed_14>() as libc::c_ulong
                            != 0
                        {
                            error(
                                1 as libc::c_int,
                                0 as libc::c_int,
                                dcgettext(
                                    0 as *const libc::c_char,
                                    b"'--pages=FIRST_PAGE[:LAST_PAGE]' missing argument\0"
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
                                    b"'--pages=FIRST_PAGE[:LAST_PAGE]' missing argument\0"
                                        as *const u8 as *const libc::c_char,
                                    5 as libc::c_int,
                                ),
                            );
                            if 0 as libc::c_int != 0 {} else {
                                unreachable!();
                            };
                        };
                    } else if !first_last_page(
                        oi,
                        0 as libc::c_int as libc::c_char,
                        optarg,
                    ) {
                        if ::core::mem::size_of::<C2RustUnnamed_13>() as libc::c_ulong
                            != 0
                        {
                            error(
                                1 as libc::c_int,
                                0 as libc::c_int,
                                dcgettext(
                                    0 as *const libc::c_char,
                                    b"invalid page range %s\0" as *const u8
                                        as *const libc::c_char,
                                    5 as libc::c_int,
                                ),
                                quote(optarg),
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
                                    b"invalid page range %s\0" as *const u8
                                        as *const libc::c_char,
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
                256 => {
                    parse_column_count(optarg);
                    free(column_count_string as *mut libc::c_void);
                    column_count_string = 0 as *mut libc::c_char;
                    n_alloc = 0 as libc::c_int as size_t;
                }
                97 => {
                    print_across_flag = 1 as libc::c_int != 0;
                    storing_columns = 0 as libc::c_int != 0;
                }
                98 => {
                    balance_columns = 1 as libc::c_int != 0;
                }
                99 => {
                    use_cntrl_prefix = 1 as libc::c_int != 0;
                }
                100 => {
                    double_space = 1 as libc::c_int != 0;
                }
                68 => {
                    date_format = optarg;
                }
                101 => {
                    if !optarg.is_null() {
                        getoptarg(
                            optarg,
                            'e' as i32 as libc::c_char,
                            &mut input_tab_char,
                            &mut chars_per_input_tab,
                        );
                    }
                    untabify_input = 1 as libc::c_int != 0;
                }
                102 | 70 => {
                    use_form_feed = 1 as libc::c_int != 0;
                }
                104 => {
                    custom_header = optarg;
                }
                105 => {
                    if !optarg.is_null() {
                        getoptarg(
                            optarg,
                            'i' as i32 as libc::c_char,
                            &mut output_tab_char,
                            &mut chars_per_output_tab,
                        );
                    }
                    tabify_output = 1 as libc::c_int != 0;
                }
                74 => {
                    join_lines = 1 as libc::c_int != 0;
                }
                108 => {
                    getoptnum(
                        optarg,
                        1 as libc::c_int,
                        &mut lines_per_page,
                        dcgettext(
                            0 as *const libc::c_char,
                            b"'-l PAGE_LENGTH' invalid number of lines\0" as *const u8
                                as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                    );
                }
                109 => {
                    parallel_files = 1 as libc::c_int != 0;
                    storing_columns = 0 as libc::c_int != 0;
                }
                110 => {
                    numbered_lines = 1 as libc::c_int != 0;
                    if !optarg.is_null() {
                        getoptarg(
                            optarg,
                            'n' as i32 as libc::c_char,
                            &mut number_separator,
                            &mut chars_per_number,
                        );
                    }
                }
                78 => {
                    skip_count = 0 as libc::c_int != 0;
                    getoptnum(
                        optarg,
                        -(2147483647 as libc::c_int) - 1 as libc::c_int,
                        &mut start_line_num,
                        dcgettext(
                            0 as *const libc::c_char,
                            b"'-N NUMBER' invalid starting line number\0" as *const u8
                                as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                    );
                }
                111 => {
                    getoptnum(
                        optarg,
                        0 as libc::c_int,
                        &mut chars_per_margin,
                        dcgettext(
                            0 as *const libc::c_char,
                            b"'-o MARGIN' invalid line offset\0" as *const u8
                                as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                    );
                }
                114 => {
                    ignore_failed_opens = 1 as libc::c_int != 0;
                }
                115 => {
                    old_options = 1 as libc::c_int != 0;
                    old_s = 1 as libc::c_int != 0;
                    if !use_col_separator && !optarg.is_null() {
                        separator_string(optarg);
                    }
                }
                83 => {
                    old_s = 0 as libc::c_int != 0;
                    col_sep_string = b"\0" as *const u8 as *const libc::c_char;
                    col_sep_length = 0 as libc::c_int;
                    use_col_separator = 1 as libc::c_int != 0;
                    if !optarg.is_null() {
                        separator_string(optarg);
                    }
                }
                116 => {
                    extremities = 0 as libc::c_int != 0;
                    keep_FF = 1 as libc::c_int != 0;
                }
                84 => {
                    extremities = 0 as libc::c_int != 0;
                    keep_FF = 0 as libc::c_int != 0;
                }
                118 => {
                    use_esc_sequence = 1 as libc::c_int != 0;
                }
                119 => {
                    old_options = 1 as libc::c_int != 0;
                    old_w = 1 as libc::c_int != 0;
                    let mut tmp_cpl: libc::c_int = 0;
                    getoptnum(
                        optarg,
                        1 as libc::c_int,
                        &mut tmp_cpl,
                        dcgettext(
                            0 as *const libc::c_char,
                            b"'-w PAGE_WIDTH' invalid number of characters\0"
                                as *const u8 as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                    );
                    if !truncate_lines {
                        chars_per_line = tmp_cpl;
                    }
                }
                87 => {
                    old_w = 0 as libc::c_int != 0;
                    truncate_lines = 1 as libc::c_int != 0;
                    getoptnum(
                        optarg,
                        1 as libc::c_int,
                        &mut chars_per_line,
                        dcgettext(
                            0 as *const libc::c_char,
                            b"'-W PAGE_WIDTH' invalid number of characters\0"
                                as *const u8 as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                    );
                }
                -2 => {
                    usage(0 as libc::c_int);
                }
                -3 => {
                    version_etc(
                        stdout,
                        b"pr\0" as *const u8 as *const libc::c_char,
                        b"GNU coreutils\0" as *const u8 as *const libc::c_char,
                        Version,
                        b"Pete TerMaat\0" as *const u8 as *const libc::c_char,
                        b"Roland Huebner\0" as *const u8 as *const libc::c_char,
                        0 as *mut libc::c_void as *mut libc::c_char,
                    );
                    exit(0 as libc::c_int);
                }
                _ => {
                    usage(1 as libc::c_int);
                }
            }
        }
    }
    if !column_count_string.is_null() {
        parse_column_count(column_count_string);
        free(column_count_string as *mut libc::c_void);
    }
    if date_format.is_null() {
        date_format = if !(getenv(
            b"POSIXLY_CORRECT\0" as *const u8 as *const libc::c_char,
        ))
            .is_null() && !hard_locale(2 as libc::c_int)
        {
            b"%b %e %H:%M %Y\0" as *const u8 as *const libc::c_char
        } else {
            b"%Y-%m-%d %H:%M\0" as *const u8 as *const libc::c_char
        };
    }
    localtz = tzalloc(getenv(b"TZ\0" as *const u8 as *const libc::c_char));
    if first_page_number == 0 as libc::c_int as libc::c_ulong {
        first_page_number = 1 as libc::c_int as uintmax_t;
    }
    if parallel_files as libc::c_int != 0 && explicit_columns as libc::c_int != 0 {
        if ::core::mem::size_of::<C2RustUnnamed_12>() as libc::c_ulong != 0 {
            error(
                1 as libc::c_int,
                0 as libc::c_int,
                dcgettext(
                    0 as *const libc::c_char,
                    b"cannot specify number of columns when printing in parallel\0"
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
                    b"cannot specify number of columns when printing in parallel\0"
                        as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
            );
            if 0 as libc::c_int != 0 {} else {
                unreachable!();
            };
        };
    }
    if parallel_files as libc::c_int != 0 && print_across_flag as libc::c_int != 0 {
        if ::core::mem::size_of::<C2RustUnnamed_11>() as libc::c_ulong != 0 {
            error(
                1 as libc::c_int,
                0 as libc::c_int,
                dcgettext(
                    0 as *const libc::c_char,
                    b"cannot specify both printing across and printing in parallel\0"
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
                    b"cannot specify both printing across and printing in parallel\0"
                        as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
            );
            if 0 as libc::c_int != 0 {} else {
                unreachable!();
            };
        };
    }
    if old_options {
        if old_w {
            if parallel_files as libc::c_int != 0 || explicit_columns as libc::c_int != 0
            {
                truncate_lines = 1 as libc::c_int != 0;
                if old_s {
                    use_col_separator = 1 as libc::c_int != 0;
                }
            } else {
                join_lines = 1 as libc::c_int != 0;
            }
        } else if !use_col_separator {
            if old_s as libc::c_int != 0
                && (parallel_files as libc::c_int != 0
                    || explicit_columns as libc::c_int != 0)
            {
                if !truncate_lines {
                    join_lines = 1 as libc::c_int != 0;
                    if col_sep_length > 0 as libc::c_int {
                        use_col_separator = 1 as libc::c_int != 0;
                    }
                } else {
                    use_col_separator = 1 as libc::c_int != 0;
                }
            }
        }
    }
    while optind < argc {
        let fresh5 = n_files;
        n_files = n_files.wrapping_add(1);
        let ref mut fresh6 = *file_names.offset(fresh5 as isize);
        *fresh6 = *argv.offset(optind as isize);
        optind += 1;
    }
    if n_files == 0 as libc::c_int as libc::c_uint {
        print_files(0 as libc::c_int, 0 as *mut *mut libc::c_char);
    } else if parallel_files {
        print_files(n_files as libc::c_int, file_names);
    } else {
        let mut i: libc::c_uint = 0 as libc::c_int as libc::c_uint;
        while i < n_files {
            print_files(1 as libc::c_int, &mut *file_names.offset(i as isize));
            i = i.wrapping_add(1);
        }
    }
    cleanup();
    if have_read_stdin as libc::c_int != 0 && rpl_fclose(stdin) == -(1 as libc::c_int) {
        if ::core::mem::size_of::<C2RustUnnamed_10>() as libc::c_ulong != 0 {
            error(
                1 as libc::c_int,
                *__errno_location(),
                dcgettext(
                    0 as *const libc::c_char,
                    b"standard input\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
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
                    b"standard input\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
            );
            if 0 as libc::c_int != 0 {} else {
                unreachable!();
            };
        };
    }
    exit(
        if failed_opens as libc::c_int != 0 {
            1 as libc::c_int
        } else {
            0 as libc::c_int
        },
    );
}
unsafe extern "C" fn getoptnum(
    mut n_str: *const libc::c_char,
    mut min: libc::c_int,
    mut num: *mut libc::c_int,
    mut err: *const libc::c_char,
) {
    let mut tnum: intmax_t = xdectoimax(
        n_str,
        min as intmax_t,
        2147483647 as libc::c_int as intmax_t,
        b"\0" as *const u8 as *const libc::c_char,
        err,
        0 as libc::c_int,
    );
    *num = tnum as libc::c_int;
}
unsafe extern "C" fn getoptarg(
    mut arg: *mut libc::c_char,
    mut switch_char: libc::c_char,
    mut character: *mut libc::c_char,
    mut number: *mut libc::c_int,
) {
    if !((*arg as libc::c_uint).wrapping_sub('0' as i32 as libc::c_uint)
        <= 9 as libc::c_int as libc::c_uint)
    {
        let fresh7 = arg;
        arg = arg.offset(1);
        *character = *fresh7;
    }
    if *arg != 0 {
        let mut tmp_long: libc::c_long = 0;
        let mut e: strtol_error = xstrtol(
            arg,
            0 as *mut *mut libc::c_char,
            10 as libc::c_int,
            &mut tmp_long,
            b"\0" as *const u8 as *const libc::c_char,
        );
        if e as libc::c_uint == LONGINT_OK as libc::c_int as libc::c_uint {
            if tmp_long <= 0 as libc::c_int as libc::c_long {
                e = LONGINT_INVALID;
            } else if (2147483647 as libc::c_int as libc::c_long) < tmp_long {
                e = LONGINT_OVERFLOW;
            }
        }
        if e as libc::c_uint != LONGINT_OK as libc::c_int as libc::c_uint {
            error(
                0 as libc::c_int,
                if e as libc::c_uint & LONGINT_OVERFLOW as libc::c_int as libc::c_uint
                    != 0
                {
                    75 as libc::c_int
                } else {
                    0 as libc::c_int
                },
                dcgettext(
                    0 as *const libc::c_char,
                    b"'-%c' extra characters or invalid number in the argument: %s\0"
                        as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
                switch_char as libc::c_int,
                quote(arg),
            );
            usage(1 as libc::c_int);
        }
        *number = tmp_long as libc::c_int;
    }
}
unsafe extern "C" fn init_parameters(mut number_of_files: libc::c_int) {
    let mut chars_used_by_number: libc::c_int = 0 as libc::c_int;
    lines_per_body = lines_per_page - lines_per_header as libc::c_int
        - lines_per_footer as libc::c_int;
    if lines_per_body <= 0 as libc::c_int {
        extremities = 0 as libc::c_int != 0;
        keep_FF = 1 as libc::c_int != 0;
    }
    if extremities as libc::c_int == 0 as libc::c_int {
        lines_per_body = lines_per_page;
    }
    if double_space {
        lines_per_body = lines_per_body / 2 as libc::c_int;
    }
    if number_of_files == 0 as libc::c_int {
        parallel_files = 0 as libc::c_int != 0;
    }
    if parallel_files {
        columns = number_of_files;
    }
    if storing_columns {
        balance_columns = 1 as libc::c_int != 0;
    }
    if columns > 1 as libc::c_int {
        if !use_col_separator {
            if join_lines {
                col_sep_string = line_separator;
            } else {
                col_sep_string = column_separator;
            }
            col_sep_length = 1 as libc::c_int;
            use_col_separator = 1 as libc::c_int != 0;
        } else if !join_lines && col_sep_length == 1 as libc::c_int
            && *col_sep_string as libc::c_int == '\t' as i32
        {
            col_sep_string = column_separator;
        }
        truncate_lines = 1 as libc::c_int != 0;
        if !(col_sep_length == 1 as libc::c_int
            && *col_sep_string as libc::c_int == '\t' as i32)
        {
            untabify_input = 1 as libc::c_int != 0;
        }
        tabify_output = 1 as libc::c_int != 0;
    } else {
        storing_columns = 0 as libc::c_int != 0;
    }
    if join_lines {
        truncate_lines = 0 as libc::c_int != 0;
    }
    if numbered_lines {
        let mut chars_per_default_tab: libc::c_int = 8 as libc::c_int;
        line_count = start_line_num;
        if number_separator as libc::c_int == '\t' as i32 {
            number_width = chars_per_number
                + (chars_per_default_tab - chars_per_number % chars_per_default_tab);
        } else {
            number_width = chars_per_number + 1 as libc::c_int;
        }
        if parallel_files {
            chars_used_by_number = number_width;
        }
    }
    let mut sep_chars: libc::c_int = 0;
    let mut useful_chars: libc::c_int = 0;
    if if (0 as libc::c_int) < -(1 as libc::c_int)
        && ((if 1 as libc::c_int != 0 {
            0 as libc::c_int
        } else {
            columns - 1 as libc::c_int
        }) - 1 as libc::c_int) < 0 as libc::c_int
        && ((if 1 as libc::c_int != 0 { 0 as libc::c_int } else { col_sep_length })
            - 1 as libc::c_int) < 0 as libc::c_int
        && (if col_sep_length < 0 as libc::c_int {
            (if (columns - 1 as libc::c_int) < 0 as libc::c_int {
                (if ((if 1 as libc::c_int != 0 {
                    0 as libc::c_int
                } else {
                    (if 1 as libc::c_int != 0 {
                        0 as libc::c_int
                    } else {
                        -(1 as libc::c_int)
                    }) + col_sep_length
                }) - 1 as libc::c_int) < 0 as libc::c_int
                {
                    ((columns - 1 as libc::c_int) < -(1 as libc::c_int) / col_sep_length)
                        as libc::c_int
                } else {
                    ((if (if (if ((if 1 as libc::c_int != 0 {
                        0 as libc::c_int
                    } else {
                        col_sep_length
                    }) - 1 as libc::c_int) < 0 as libc::c_int
                    {
                        !(((((if 1 as libc::c_int != 0 {
                            0 as libc::c_int
                        } else {
                            col_sep_length
                        }) + 1 as libc::c_int)
                            << (::core::mem::size_of::<libc::c_int>() as libc::c_ulong)
                                .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                            - 1 as libc::c_int) * 2 as libc::c_int + 1 as libc::c_int)
                    } else {
                        (if 1 as libc::c_int != 0 {
                            0 as libc::c_int
                        } else {
                            col_sep_length
                        }) + 0 as libc::c_int
                    }) < 0 as libc::c_int
                    {
                        (col_sep_length
                            < -(if ((if 1 as libc::c_int != 0 {
                                0 as libc::c_int
                            } else {
                                col_sep_length
                            }) - 1 as libc::c_int) < 0 as libc::c_int
                            {
                                ((((if 1 as libc::c_int != 0 {
                                    0 as libc::c_int
                                } else {
                                    col_sep_length
                                }) + 1 as libc::c_int)
                                    << (::core::mem::size_of::<libc::c_int>() as libc::c_ulong)
                                        .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                        .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                    - 1 as libc::c_int) * 2 as libc::c_int + 1 as libc::c_int
                            } else {
                                (if 1 as libc::c_int != 0 {
                                    0 as libc::c_int
                                } else {
                                    col_sep_length
                                }) - 1 as libc::c_int
                            })) as libc::c_int
                    } else {
                        ((0 as libc::c_int) < col_sep_length) as libc::c_int
                    }) != 0
                    {
                        (if 1 as libc::c_int != 0 {
                            0 as libc::c_int
                        } else {
                            col_sep_length
                        }) + -(1 as libc::c_int)
                            >> (::core::mem::size_of::<libc::c_int>() as libc::c_ulong)
                                .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                    } else {
                        -(1 as libc::c_int) / -col_sep_length
                    }) <= -(1 as libc::c_int) - (columns - 1 as libc::c_int))
                        as libc::c_int
                })
            } else {
                (if (if (if ((if 1 as libc::c_int != 0 {
                    0 as libc::c_int
                } else {
                    (if 1 as libc::c_int != 0 {
                        0 as libc::c_int
                    } else {
                        col_sep_length
                    }) + 0 as libc::c_int
                }) - 1 as libc::c_int) < 0 as libc::c_int
                {
                    !(((((if 1 as libc::c_int != 0 {
                        0 as libc::c_int
                    } else {
                        (if 1 as libc::c_int != 0 {
                            0 as libc::c_int
                        } else {
                            col_sep_length
                        }) + 0 as libc::c_int
                    }) + 1 as libc::c_int)
                        << (::core::mem::size_of::<libc::c_int>() as libc::c_ulong)
                            .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                            .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                        - 1 as libc::c_int) * 2 as libc::c_int + 1 as libc::c_int)
                } else {
                    (if 1 as libc::c_int != 0 {
                        0 as libc::c_int
                    } else {
                        (if 1 as libc::c_int != 0 {
                            0 as libc::c_int
                        } else {
                            col_sep_length
                        }) + 0 as libc::c_int
                    }) + 0 as libc::c_int
                }) < 0 as libc::c_int
                {
                    (((if 1 as libc::c_int != 0 {
                        0 as libc::c_int
                    } else {
                        col_sep_length
                    }) + 0 as libc::c_int)
                        < -(if ((if 1 as libc::c_int != 0 {
                            0 as libc::c_int
                        } else {
                            (if 1 as libc::c_int != 0 {
                                0 as libc::c_int
                            } else {
                                col_sep_length
                            }) + 0 as libc::c_int
                        }) - 1 as libc::c_int) < 0 as libc::c_int
                        {
                            ((((if 1 as libc::c_int != 0 {
                                0 as libc::c_int
                            } else {
                                (if 1 as libc::c_int != 0 {
                                    0 as libc::c_int
                                } else {
                                    col_sep_length
                                }) + 0 as libc::c_int
                            }) + 1 as libc::c_int)
                                << (::core::mem::size_of::<libc::c_int>() as libc::c_ulong)
                                    .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                    .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                - 1 as libc::c_int) * 2 as libc::c_int + 1 as libc::c_int
                        } else {
                            (if 1 as libc::c_int != 0 {
                                0 as libc::c_int
                            } else {
                                (if 1 as libc::c_int != 0 {
                                    0 as libc::c_int
                                } else {
                                    col_sep_length
                                }) + 0 as libc::c_int
                            }) - 1 as libc::c_int
                        })) as libc::c_int
                } else {
                    ((0 as libc::c_int)
                        < (if 1 as libc::c_int != 0 {
                            0 as libc::c_int
                        } else {
                            col_sep_length
                        }) + 0 as libc::c_int) as libc::c_int
                }) != 0 && col_sep_length == -(1 as libc::c_int)
                {
                    (if ((if 1 as libc::c_int != 0 {
                        0 as libc::c_int
                    } else {
                        columns - 1 as libc::c_int
                    }) - 1 as libc::c_int) < 0 as libc::c_int
                    {
                        ((0 as libc::c_int)
                            < columns - 1 as libc::c_int + 0 as libc::c_int)
                            as libc::c_int
                    } else {
                        ((0 as libc::c_int) < columns - 1 as libc::c_int
                            && (-(1 as libc::c_int) - 0 as libc::c_int)
                                < columns - 1 as libc::c_int - 1 as libc::c_int)
                            as libc::c_int
                    })
                } else {
                    (0 as libc::c_int / col_sep_length < columns - 1 as libc::c_int)
                        as libc::c_int
                })
            })
        } else {
            (if col_sep_length == 0 as libc::c_int {
                0 as libc::c_int
            } else {
                (if (columns - 1 as libc::c_int) < 0 as libc::c_int {
                    (if (if (if ((if 1 as libc::c_int != 0 {
                        0 as libc::c_int
                    } else {
                        (if 1 as libc::c_int != 0 {
                            0 as libc::c_int
                        } else {
                            columns - 1 as libc::c_int
                        }) + 0 as libc::c_int
                    }) - 1 as libc::c_int) < 0 as libc::c_int
                    {
                        !(((((if 1 as libc::c_int != 0 {
                            0 as libc::c_int
                        } else {
                            (if 1 as libc::c_int != 0 {
                                0 as libc::c_int
                            } else {
                                columns - 1 as libc::c_int
                            }) + 0 as libc::c_int
                        }) + 1 as libc::c_int)
                            << (::core::mem::size_of::<libc::c_int>() as libc::c_ulong)
                                .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                            - 1 as libc::c_int) * 2 as libc::c_int + 1 as libc::c_int)
                    } else {
                        (if 1 as libc::c_int != 0 {
                            0 as libc::c_int
                        } else {
                            (if 1 as libc::c_int != 0 {
                                0 as libc::c_int
                            } else {
                                columns - 1 as libc::c_int
                            }) + 0 as libc::c_int
                        }) + 0 as libc::c_int
                    }) < 0 as libc::c_int
                    {
                        (((if 1 as libc::c_int != 0 {
                            0 as libc::c_int
                        } else {
                            columns - 1 as libc::c_int
                        }) + 0 as libc::c_int)
                            < -(if ((if 1 as libc::c_int != 0 {
                                0 as libc::c_int
                            } else {
                                (if 1 as libc::c_int != 0 {
                                    0 as libc::c_int
                                } else {
                                    columns - 1 as libc::c_int
                                }) + 0 as libc::c_int
                            }) - 1 as libc::c_int) < 0 as libc::c_int
                            {
                                ((((if 1 as libc::c_int != 0 {
                                    0 as libc::c_int
                                } else {
                                    (if 1 as libc::c_int != 0 {
                                        0 as libc::c_int
                                    } else {
                                        columns - 1 as libc::c_int
                                    }) + 0 as libc::c_int
                                }) + 1 as libc::c_int)
                                    << (::core::mem::size_of::<libc::c_int>() as libc::c_ulong)
                                        .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                        .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                    - 1 as libc::c_int) * 2 as libc::c_int + 1 as libc::c_int
                            } else {
                                (if 1 as libc::c_int != 0 {
                                    0 as libc::c_int
                                } else {
                                    (if 1 as libc::c_int != 0 {
                                        0 as libc::c_int
                                    } else {
                                        columns - 1 as libc::c_int
                                    }) + 0 as libc::c_int
                                }) - 1 as libc::c_int
                            })) as libc::c_int
                    } else {
                        ((0 as libc::c_int)
                            < (if 1 as libc::c_int != 0 {
                                0 as libc::c_int
                            } else {
                                columns - 1 as libc::c_int
                            }) + 0 as libc::c_int) as libc::c_int
                    }) != 0 && columns - 1 as libc::c_int == -(1 as libc::c_int)
                    {
                        (if ((if 1 as libc::c_int != 0 {
                            0 as libc::c_int
                        } else {
                            col_sep_length
                        }) - 1 as libc::c_int) < 0 as libc::c_int
                        {
                            ((0 as libc::c_int) < col_sep_length + 0 as libc::c_int)
                                as libc::c_int
                        } else {
                            ((-(1 as libc::c_int) - 0 as libc::c_int)
                                < col_sep_length - 1 as libc::c_int) as libc::c_int
                        })
                    } else {
                        (0 as libc::c_int / (columns - 1 as libc::c_int)
                            < col_sep_length) as libc::c_int
                    })
                } else {
                    (-(1 as libc::c_int) / col_sep_length < columns - 1 as libc::c_int)
                        as libc::c_int
                })
            })
        }) != 0
    {
        let (fresh8, fresh9) = (columns - 1 as libc::c_int)
            .overflowing_mul(col_sep_length);
        *(&mut sep_chars as *mut libc::c_int) = fresh8;
        1 as libc::c_int
    } else {
        let (fresh10, fresh11) = (columns - 1 as libc::c_int)
            .overflowing_mul(col_sep_length);
        *(&mut sep_chars as *mut libc::c_int) = fresh10;
        fresh11 as libc::c_int
    } != 0
    {
        sep_chars = 2147483647 as libc::c_int;
    }
    let (fresh12, fresh13) = (chars_per_line - chars_used_by_number)
        .overflowing_sub(sep_chars);
    *(&mut useful_chars as *mut libc::c_int) = fresh12;
    if fresh13 {
        useful_chars = 0 as libc::c_int;
    }
    chars_per_column = useful_chars / columns;
    if chars_per_column < 1 as libc::c_int {
        if ::core::mem::size_of::<C2RustUnnamed_6>() as libc::c_ulong != 0 {
            error(
                1 as libc::c_int,
                0 as libc::c_int,
                dcgettext(
                    0 as *const libc::c_char,
                    b"page width too narrow\0" as *const u8 as *const libc::c_char,
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
                    b"page width too narrow\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
            );
            if 0 as libc::c_int != 0 {} else {
                unreachable!();
            };
        };
    }
    if numbered_lines {
        free(number_buff as *mut libc::c_void);
        number_buff = xmalloc(
            (if chars_per_number as libc::c_ulong
                > (::core::mem::size_of::<libc::c_int>() as libc::c_ulong)
                    .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                    .wrapping_sub(
                        !((0 as libc::c_int) < -(1 as libc::c_int)) as libc::c_int
                            as libc::c_ulong,
                    )
                    .wrapping_mul(146 as libc::c_int as libc::c_ulong)
                    .wrapping_add(484 as libc::c_int as libc::c_ulong)
                    .wrapping_div(485 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        !((0 as libc::c_int) < -(1 as libc::c_int)) as libc::c_int
                            as libc::c_ulong,
                    )
            {
                chars_per_number as libc::c_ulong
            } else {
                (::core::mem::size_of::<libc::c_int>() as libc::c_ulong)
                    .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                    .wrapping_sub(
                        !((0 as libc::c_int) < -(1 as libc::c_int)) as libc::c_int
                            as libc::c_ulong,
                    )
                    .wrapping_mul(146 as libc::c_int as libc::c_ulong)
                    .wrapping_add(484 as libc::c_int as libc::c_ulong)
                    .wrapping_div(485 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        !((0 as libc::c_int) < -(1 as libc::c_int)) as libc::c_int
                            as libc::c_ulong,
                    )
            })
                .wrapping_add(1 as libc::c_int as libc::c_ulong),
        ) as *mut libc::c_char;
    }
    free(clump_buff as *mut libc::c_void);
    clump_buff = xmalloc(
        (if 8 as libc::c_int > chars_per_input_tab {
            8 as libc::c_int
        } else {
            chars_per_input_tab
        }) as size_t,
    ) as *mut libc::c_char;
}
unsafe extern "C" fn init_fps(
    mut number_of_files: libc::c_int,
    mut av: *mut *mut libc::c_char,
) -> bool {
    let mut p: *mut COLUMN = 0 as *mut COLUMN;
    total_files = 0 as libc::c_int;
    free(column_vector as *mut libc::c_void);
    column_vector = xnmalloc(
        columns as size_t,
        ::core::mem::size_of::<COLUMN>() as libc::c_ulong,
    ) as *mut COLUMN;
    if parallel_files {
        let mut files_left: libc::c_int = number_of_files;
        p = column_vector;
        loop {
            let fresh14 = files_left;
            files_left = files_left - 1;
            if !(fresh14 != 0) {
                break;
            }
            if !open_file(*av, p) {
                p = p.offset(-1);
                columns -= 1;
            }
            p = p.offset(1);
            av = av.offset(1);
        }
        if columns == 0 as libc::c_int {
            return 0 as libc::c_int != 0;
        }
        init_header(b"\0" as *const u8 as *const libc::c_char, -(1 as libc::c_int));
    } else {
        p = column_vector;
        if number_of_files > 0 as libc::c_int {
            if !open_file(*av, p) {
                return 0 as libc::c_int != 0;
            }
            init_header(*av, fileno((*p).fp));
            (*p).lines_stored = 0 as libc::c_int;
        } else {
            (*p)
                .name = dcgettext(
                0 as *const libc::c_char,
                b"standard input\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            );
            (*p).fp = stdin;
            have_read_stdin = 1 as libc::c_int != 0;
            (*p).status = OPEN;
            (*p).full_page_printed = 0 as libc::c_int != 0;
            total_files += 1;
            init_header(b"\0" as *const u8 as *const libc::c_char, -(1 as libc::c_int));
            (*p).lines_stored = 0 as libc::c_int;
        }
        let mut firstname: *const libc::c_char = (*p).name;
        let mut firstfp: *mut FILE = (*p).fp;
        let mut i: libc::c_int = 0;
        i = columns - 1 as libc::c_int;
        p = p.offset(1);
        while i != 0 {
            (*p).name = firstname;
            (*p).fp = firstfp;
            (*p).status = OPEN;
            (*p).full_page_printed = 0 as libc::c_int != 0;
            (*p).lines_stored = 0 as libc::c_int;
            i -= 1;
            p = p.offset(1);
        }
    }
    files_ready_to_read = total_files;
    return 1 as libc::c_int != 0;
}
unsafe extern "C" fn init_funcs() {
    let mut i: libc::c_int = 0;
    let mut h: libc::c_int = 0;
    let mut h_next: libc::c_int = 0;
    let mut p: *mut COLUMN = 0 as *mut COLUMN;
    h = chars_per_margin;
    if !truncate_lines {
        h_next = 0 as libc::c_int;
    } else if parallel_files as libc::c_int != 0 && numbered_lines as libc::c_int != 0 {
        h_next = h + chars_per_column + number_width;
    } else {
        h_next = h + chars_per_column;
    }
    h = h + col_sep_length;
    p = column_vector;
    i = 1 as libc::c_int;
    while i < columns {
        if storing_columns {
            (*p)
                .char_func = Some(
                store_char as unsafe extern "C" fn(libc::c_char) -> (),
            );
            (*p)
                .print_func = Some(
                print_stored as unsafe extern "C" fn(*mut COLUMN) -> bool,
            );
        } else {
            (*p)
                .char_func = Some(
                print_char as unsafe extern "C" fn(libc::c_char) -> (),
            );
            (*p)
                .print_func = Some(
                read_line as unsafe extern "C" fn(*mut COLUMN) -> bool,
            );
        }
        (*p)
            .numbered = numbered_lines as libc::c_int != 0
            && (!parallel_files || i == 1 as libc::c_int);
        (*p).start_position = h;
        if !truncate_lines {
            h = 0 as libc::c_int;
            h_next = 0 as libc::c_int;
        } else {
            h = h_next + col_sep_length;
            h_next = h + chars_per_column;
        }
        p = p.offset(1);
        i += 1;
    }
    if storing_columns as libc::c_int != 0 && balance_columns as libc::c_int != 0 {
        (*p).char_func = Some(store_char as unsafe extern "C" fn(libc::c_char) -> ());
        (*p)
            .print_func = Some(
            print_stored as unsafe extern "C" fn(*mut COLUMN) -> bool,
        );
    } else {
        (*p).char_func = Some(print_char as unsafe extern "C" fn(libc::c_char) -> ());
        (*p).print_func = Some(read_line as unsafe extern "C" fn(*mut COLUMN) -> bool);
    }
    (*p)
        .numbered = numbered_lines as libc::c_int != 0
        && (!parallel_files || i == 1 as libc::c_int);
    (*p).start_position = h;
}
unsafe extern "C" fn open_file(mut name: *mut libc::c_char, mut p: *mut COLUMN) -> bool {
    if strcmp(name, b"-\0" as *const u8 as *const libc::c_char) == 0 as libc::c_int {
        (*p)
            .name = dcgettext(
            0 as *const libc::c_char,
            b"standard input\0" as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        );
        (*p).fp = stdin;
        have_read_stdin = 1 as libc::c_int != 0;
    } else {
        (*p).name = name;
        (*p).fp = fopen_safer(name, b"r\0" as *const u8 as *const libc::c_char);
    }
    if ((*p).fp).is_null() {
        failed_opens = 1 as libc::c_int != 0;
        if !ignore_failed_opens {
            error(
                0 as libc::c_int,
                *__errno_location(),
                b"%s\0" as *const u8 as *const libc::c_char,
                quotearg_n_style_colon(
                    0 as libc::c_int,
                    shell_escape_quoting_style,
                    name,
                ),
            );
        }
        return 0 as libc::c_int != 0;
    }
    fadvise((*p).fp, FADVISE_SEQUENTIAL);
    (*p).status = OPEN;
    (*p).full_page_printed = 0 as libc::c_int != 0;
    total_files += 1;
    return 1 as libc::c_int != 0;
}
unsafe extern "C" fn close_file(mut p: *mut COLUMN) {
    let mut q: *mut COLUMN = 0 as *mut COLUMN;
    let mut i: libc::c_int = 0;
    if (*p).status as libc::c_uint == CLOSED as libc::c_int as libc::c_uint {
        return;
    }
    let mut err: libc::c_int = *__errno_location();
    if ferror_unlocked((*p).fp) == 0 {
        err = 0 as libc::c_int;
    }
    if fileno((*p).fp) == 0 as libc::c_int {
        clearerr_unlocked((*p).fp);
    } else if rpl_fclose((*p).fp) != 0 as libc::c_int && err == 0 {
        err = *__errno_location();
    }
    if err != 0 {
        if ::core::mem::size_of::<C2RustUnnamed_2>() as libc::c_ulong != 0 {
            error(
                1 as libc::c_int,
                err,
                b"%s\0" as *const u8 as *const libc::c_char,
                quotearg_n_style_colon(
                    0 as libc::c_int,
                    shell_escape_quoting_style,
                    (*p).name,
                ),
            );
            if 0 as libc::c_int != 0 {} else {
                unreachable!();
            };
        } else {
            error(
                1 as libc::c_int,
                err,
                b"%s\0" as *const u8 as *const libc::c_char,
                quotearg_n_style_colon(
                    0 as libc::c_int,
                    shell_escape_quoting_style,
                    (*p).name,
                ),
            );
            if 0 as libc::c_int != 0 {} else {
                unreachable!();
            };
        };
    }
    if !parallel_files {
        q = column_vector;
        i = columns;
        while i != 0 {
            (*q).status = CLOSED;
            if (*q).lines_stored == 0 as libc::c_int {
                (*q).lines_to_print = 0 as libc::c_int;
            }
            q = q.offset(1);
            i -= 1;
        }
    } else {
        (*p).status = CLOSED;
        (*p).lines_to_print = 0 as libc::c_int;
    }
    files_ready_to_read -= 1;
}
unsafe extern "C" fn hold_file(mut p: *mut COLUMN) {
    let mut q: *mut COLUMN = 0 as *mut COLUMN;
    let mut i: libc::c_int = 0;
    if !parallel_files {
        q = column_vector;
        i = columns;
        while i != 0 {
            if storing_columns {
                (*q).status = FF_FOUND;
            } else {
                (*q).status = ON_HOLD;
            }
            q = q.offset(1);
            i -= 1;
        }
    } else {
        (*p).status = ON_HOLD;
    }
    (*p).lines_to_print = 0 as libc::c_int;
    files_ready_to_read -= 1;
}
unsafe extern "C" fn reset_status() {
    let mut i: libc::c_int = columns;
    let mut p: *mut COLUMN = 0 as *mut COLUMN;
    p = column_vector;
    while i != 0 {
        if (*p).status as libc::c_uint == ON_HOLD as libc::c_int as libc::c_uint {
            (*p).status = OPEN;
            files_ready_to_read += 1;
        }
        i -= 1;
        p = p.offset(1);
    }
    if storing_columns {
        if (*column_vector).status as libc::c_uint
            == CLOSED as libc::c_int as libc::c_uint
        {
            files_ready_to_read = 0 as libc::c_int;
        } else {
            files_ready_to_read = 1 as libc::c_int;
        }
    }
}
unsafe extern "C" fn print_files(
    mut number_of_files: libc::c_int,
    mut av: *mut *mut libc::c_char,
) {
    init_parameters(number_of_files);
    if !init_fps(number_of_files, av) {
        return;
    }
    if storing_columns {
        init_store_cols();
    }
    if first_page_number > 1 as libc::c_int as libc::c_ulong {
        if !skip_to_page(first_page_number) {
            return
        } else {
            page_number = first_page_number;
        }
    } else {
        page_number = 1 as libc::c_int as uintmax_t;
    }
    init_funcs();
    line_number = line_count;
    while print_page() {}
}
unsafe extern "C" fn init_header(
    mut filename: *const libc::c_char,
    mut desc: libc::c_int,
) {
    let mut buf: *mut libc::c_char = 0 as *mut libc::c_char;
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
    let mut t: timespec = timespec { tv_sec: 0, tv_nsec: 0 };
    let mut ns: libc::c_int = 0;
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
    if strcmp(filename, b"-\0" as *const u8 as *const libc::c_char) == 0 as libc::c_int {
        desc = -(1 as libc::c_int);
    }
    if 0 as libc::c_int <= desc && fstat(desc, &mut st) == 0 as libc::c_int {
        t = get_stat_mtime(&mut st);
    } else {
        static mut timespec: timespec = timespec { tv_sec: 0, tv_nsec: 0 };
        if timespec.tv_sec == 0 {
            gettime(&mut timespec);
        }
        t = timespec;
    }
    ns = t.tv_nsec as libc::c_int;
    if !(localtime_rz(localtz, &mut t.tv_sec, &mut tm)).is_null() {
        let mut bufsize: size_t = (nstrftime(
            0 as *mut libc::c_char,
            18446744073709551615 as libc::c_ulong,
            date_format,
            &mut tm,
            localtz,
            ns,
        ))
            .wrapping_add(1 as libc::c_int as libc::c_ulong);
        buf = xmalloc(bufsize) as *mut libc::c_char;
        nstrftime(buf, bufsize, date_format, &mut tm, localtz, ns);
    } else {
        let mut secbuf: [libc::c_char; 21] = [0; 21];
        buf = xmalloc(
            (::core::mem::size_of::<[libc::c_char; 21]>() as libc::c_ulong)
                .wrapping_add(
                    (if 10 as libc::c_int as libc::c_ulong
                        > (::core::mem::size_of::<libc::c_int>() as libc::c_ulong)
                            .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                            .wrapping_sub(
                                !((0 as libc::c_int) < -(1 as libc::c_int)) as libc::c_int
                                    as libc::c_ulong,
                            )
                            .wrapping_mul(146 as libc::c_int as libc::c_ulong)
                            .wrapping_add(484 as libc::c_int as libc::c_ulong)
                            .wrapping_div(485 as libc::c_int as libc::c_ulong)
                            .wrapping_add(
                                !((0 as libc::c_int) < -(1 as libc::c_int)) as libc::c_int
                                    as libc::c_ulong,
                            )
                            .wrapping_add(1 as libc::c_int as libc::c_ulong)
                    {
                        10 as libc::c_int as libc::c_ulong
                    } else {
                        (::core::mem::size_of::<libc::c_int>() as libc::c_ulong)
                            .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                            .wrapping_sub(
                                !((0 as libc::c_int) < -(1 as libc::c_int)) as libc::c_int
                                    as libc::c_ulong,
                            )
                            .wrapping_mul(146 as libc::c_int as libc::c_ulong)
                            .wrapping_add(484 as libc::c_int as libc::c_ulong)
                            .wrapping_div(485 as libc::c_int as libc::c_ulong)
                            .wrapping_add(
                                !((0 as libc::c_int) < -(1 as libc::c_int)) as libc::c_int
                                    as libc::c_ulong,
                            )
                            .wrapping_add(1 as libc::c_int as libc::c_ulong)
                    }),
                ),
        ) as *mut libc::c_char;
        sprintf(
            buf,
            b"%s.%09d\0" as *const u8 as *const libc::c_char,
            timetostr(t.tv_sec, secbuf.as_mut_ptr()),
            ns,
        );
    }
    free(date_text as *mut libc::c_void);
    date_text = buf;
    file_text = if !custom_header.is_null() {
        custom_header as *const libc::c_char
    } else if desc < 0 as libc::c_int {
        b"\0" as *const u8 as *const libc::c_char
    } else {
        filename
    };
    header_width_available = chars_per_line - gnu_mbswidth(date_text, 0 as libc::c_int)
        - gnu_mbswidth(file_text, 0 as libc::c_int);
}
unsafe extern "C" fn init_page() {
    let mut j: libc::c_int = 0;
    let mut p: *mut COLUMN = 0 as *mut COLUMN;
    if storing_columns {
        store_columns();
        j = columns - 1 as libc::c_int;
        p = column_vector;
        while j != 0 {
            (*p).lines_to_print = (*p).lines_stored;
            j -= 1;
            p = p.offset(1);
        }
        if balance_columns {
            (*p).lines_to_print = (*p).lines_stored;
        } else if (*p).status as libc::c_uint == OPEN as libc::c_int as libc::c_uint {
            (*p).lines_to_print = lines_per_body;
        } else {
            (*p).lines_to_print = 0 as libc::c_int;
        }
    } else {
        j = columns;
        p = column_vector;
        while j != 0 {
            if (*p).status as libc::c_uint == OPEN as libc::c_int as libc::c_uint {
                (*p).lines_to_print = lines_per_body;
            } else {
                (*p).lines_to_print = 0 as libc::c_int;
            }
            j -= 1;
            p = p.offset(1);
        }
    };
}
unsafe extern "C" fn align_column(mut p: *mut COLUMN) {
    padding_not_printed = (*p).start_position;
    if col_sep_length < padding_not_printed {
        pad_across_to(padding_not_printed - col_sep_length);
        padding_not_printed = 0 as libc::c_int;
    }
    if use_col_separator {
        print_sep_string();
    }
    if (*p).numbered {
        add_line_number(p);
    }
}
unsafe extern "C" fn print_page() -> bool {
    let mut j: libc::c_int = 0;
    let mut lines_left_on_page: libc::c_int = 0;
    let mut p: *mut COLUMN = 0 as *mut COLUMN;
    let mut pv: bool = false;
    init_page();
    if cols_ready_to_print() == 0 as libc::c_int as libc::c_uint {
        return 0 as libc::c_int != 0;
    }
    if extremities {
        print_a_header = 1 as libc::c_int != 0;
    }
    pad_vertically = 0 as libc::c_int != 0;
    pv = 0 as libc::c_int != 0;
    lines_left_on_page = lines_per_body;
    if double_space {
        lines_left_on_page *= 2 as libc::c_int;
    }
    while lines_left_on_page > 0 as libc::c_int
        && cols_ready_to_print() > 0 as libc::c_int as libc::c_uint
    {
        output_position = 0 as libc::c_int;
        spaces_not_printed = 0 as libc::c_int;
        separators_not_printed = 0 as libc::c_int;
        pad_vertically = 0 as libc::c_int != 0;
        align_empty_cols = 0 as libc::c_int != 0;
        empty_line = 1 as libc::c_int != 0;
        j = 1 as libc::c_int;
        p = column_vector;
        while j <= columns {
            input_position = 0 as libc::c_int;
            if (*p).lines_to_print > 0 as libc::c_int
                || (*p).status as libc::c_uint == FF_FOUND as libc::c_int as libc::c_uint
            {
                FF_only = 0 as libc::c_int != 0;
                padding_not_printed = (*p).start_position;
                if !((*p).print_func).expect("non-null function pointer")(p) {
                    read_rest_of_line(p);
                }
                pv = (pv as libc::c_int | pad_vertically as libc::c_int) as bool;
                (*p).lines_to_print -= 1;
                if (*p).lines_to_print <= 0 as libc::c_int {
                    if cols_ready_to_print() == 0 as libc::c_int as libc::c_uint {
                        break;
                    }
                }
                if parallel_files as libc::c_int != 0
                    && (*p).status as libc::c_uint != OPEN as libc::c_int as libc::c_uint
                {
                    if empty_line {
                        align_empty_cols = 1 as libc::c_int != 0;
                    } else if (*p).status as libc::c_uint
                        == CLOSED as libc::c_int as libc::c_uint
                        || (*p).status as libc::c_uint
                            == ON_HOLD as libc::c_int as libc::c_uint
                            && FF_only as libc::c_int != 0
                    {
                        align_column(p);
                    }
                }
            } else if parallel_files {
                if empty_line {
                    align_empty_cols = 1 as libc::c_int != 0;
                } else {
                    align_column(p);
                }
            }
            if use_col_separator {
                separators_not_printed += 1;
            }
            j += 1;
            p = p.offset(1);
        }
        if pad_vertically {
            putchar_unlocked('\n' as i32);
            lines_left_on_page -= 1;
        }
        if cols_ready_to_print() == 0 as libc::c_int as libc::c_uint && !extremities {
            break;
        }
        if double_space as libc::c_int != 0 && pv as libc::c_int != 0 {
            putchar_unlocked('\n' as i32);
            lines_left_on_page -= 1;
        }
    }
    if lines_left_on_page == 0 as libc::c_int {
        j = 1 as libc::c_int;
        p = column_vector;
        while j <= columns {
            if (*p).status as libc::c_uint == OPEN as libc::c_int as libc::c_uint {
                (*p).full_page_printed = 1 as libc::c_int != 0;
            }
            j += 1;
            p = p.offset(1);
        }
    }
    pad_vertically = pv;
    if pad_vertically as libc::c_int != 0 && extremities as libc::c_int != 0 {
        pad_down((lines_left_on_page + lines_per_footer as libc::c_int) as libc::c_uint);
    } else if keep_FF as libc::c_int != 0 && print_a_FF as libc::c_int != 0 {
        putchar_unlocked('\u{c}' as i32);
        print_a_FF = 0 as libc::c_int != 0;
    }
    page_number = page_number.wrapping_add(1);
    if last_page_number < page_number {
        return 0 as libc::c_int != 0;
    }
    reset_status();
    return 1 as libc::c_int != 0;
}
unsafe extern "C" fn init_store_cols() {
    let mut total_lines: libc::c_int = 0;
    let mut total_lines_1: libc::c_int = 0;
    let mut chars_per_column_1: libc::c_int = 0;
    let mut chars_if_truncate: libc::c_int = 0;
    if (if (0 as libc::c_int) < -(1 as libc::c_int)
        && ((if 1 as libc::c_int != 0 { 0 as libc::c_int } else { lines_per_body })
            - 1 as libc::c_int) < 0 as libc::c_int
        && ((if 1 as libc::c_int != 0 { 0 as libc::c_int } else { columns })
            - 1 as libc::c_int) < 0 as libc::c_int
        && (if columns < 0 as libc::c_int {
            (if lines_per_body < 0 as libc::c_int {
                (if ((if 1 as libc::c_int != 0 {
                    0 as libc::c_int
                } else {
                    (if 1 as libc::c_int != 0 {
                        0 as libc::c_int
                    } else {
                        -(1 as libc::c_int)
                    }) + columns
                }) - 1 as libc::c_int) < 0 as libc::c_int
                {
                    (lines_per_body < -(1 as libc::c_int) / columns) as libc::c_int
                } else {
                    ((if (if (if ((if 1 as libc::c_int != 0 {
                        0 as libc::c_int
                    } else {
                        columns
                    }) - 1 as libc::c_int) < 0 as libc::c_int
                    {
                        !(((((if 1 as libc::c_int != 0 {
                            0 as libc::c_int
                        } else {
                            columns
                        }) + 1 as libc::c_int)
                            << (::core::mem::size_of::<libc::c_int>() as libc::c_ulong)
                                .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                            - 1 as libc::c_int) * 2 as libc::c_int + 1 as libc::c_int)
                    } else {
                        (if 1 as libc::c_int != 0 { 0 as libc::c_int } else { columns })
                            + 0 as libc::c_int
                    }) < 0 as libc::c_int
                    {
                        (columns
                            < -(if ((if 1 as libc::c_int != 0 {
                                0 as libc::c_int
                            } else {
                                columns
                            }) - 1 as libc::c_int) < 0 as libc::c_int
                            {
                                ((((if 1 as libc::c_int != 0 {
                                    0 as libc::c_int
                                } else {
                                    columns
                                }) + 1 as libc::c_int)
                                    << (::core::mem::size_of::<libc::c_int>() as libc::c_ulong)
                                        .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                        .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                    - 1 as libc::c_int) * 2 as libc::c_int + 1 as libc::c_int
                            } else {
                                (if 1 as libc::c_int != 0 {
                                    0 as libc::c_int
                                } else {
                                    columns
                                }) - 1 as libc::c_int
                            })) as libc::c_int
                    } else {
                        ((0 as libc::c_int) < columns) as libc::c_int
                    }) != 0
                    {
                        (if 1 as libc::c_int != 0 { 0 as libc::c_int } else { columns })
                            + -(1 as libc::c_int)
                            >> (::core::mem::size_of::<libc::c_int>() as libc::c_ulong)
                                .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                    } else {
                        -(1 as libc::c_int) / -columns
                    }) <= -(1 as libc::c_int) - lines_per_body) as libc::c_int
                })
            } else {
                (if (if (if ((if 1 as libc::c_int != 0 {
                    0 as libc::c_int
                } else {
                    (if 1 as libc::c_int != 0 { 0 as libc::c_int } else { columns })
                        + 0 as libc::c_int
                }) - 1 as libc::c_int) < 0 as libc::c_int
                {
                    !(((((if 1 as libc::c_int != 0 {
                        0 as libc::c_int
                    } else {
                        (if 1 as libc::c_int != 0 { 0 as libc::c_int } else { columns })
                            + 0 as libc::c_int
                    }) + 1 as libc::c_int)
                        << (::core::mem::size_of::<libc::c_int>() as libc::c_ulong)
                            .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                            .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                        - 1 as libc::c_int) * 2 as libc::c_int + 1 as libc::c_int)
                } else {
                    (if 1 as libc::c_int != 0 {
                        0 as libc::c_int
                    } else {
                        (if 1 as libc::c_int != 0 { 0 as libc::c_int } else { columns })
                            + 0 as libc::c_int
                    }) + 0 as libc::c_int
                }) < 0 as libc::c_int
                {
                    (((if 1 as libc::c_int != 0 { 0 as libc::c_int } else { columns })
                        + 0 as libc::c_int)
                        < -(if ((if 1 as libc::c_int != 0 {
                            0 as libc::c_int
                        } else {
                            (if 1 as libc::c_int != 0 {
                                0 as libc::c_int
                            } else {
                                columns
                            }) + 0 as libc::c_int
                        }) - 1 as libc::c_int) < 0 as libc::c_int
                        {
                            ((((if 1 as libc::c_int != 0 {
                                0 as libc::c_int
                            } else {
                                (if 1 as libc::c_int != 0 {
                                    0 as libc::c_int
                                } else {
                                    columns
                                }) + 0 as libc::c_int
                            }) + 1 as libc::c_int)
                                << (::core::mem::size_of::<libc::c_int>() as libc::c_ulong)
                                    .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                    .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                - 1 as libc::c_int) * 2 as libc::c_int + 1 as libc::c_int
                        } else {
                            (if 1 as libc::c_int != 0 {
                                0 as libc::c_int
                            } else {
                                (if 1 as libc::c_int != 0 {
                                    0 as libc::c_int
                                } else {
                                    columns
                                }) + 0 as libc::c_int
                            }) - 1 as libc::c_int
                        })) as libc::c_int
                } else {
                    ((0 as libc::c_int)
                        < (if 1 as libc::c_int != 0 {
                            0 as libc::c_int
                        } else {
                            columns
                        }) + 0 as libc::c_int) as libc::c_int
                }) != 0 && columns == -(1 as libc::c_int)
                {
                    (if ((if 1 as libc::c_int != 0 {
                        0 as libc::c_int
                    } else {
                        lines_per_body
                    }) - 1 as libc::c_int) < 0 as libc::c_int
                    {
                        ((0 as libc::c_int) < lines_per_body + 0 as libc::c_int)
                            as libc::c_int
                    } else {
                        ((0 as libc::c_int) < lines_per_body
                            && (-(1 as libc::c_int) - 0 as libc::c_int)
                                < lines_per_body - 1 as libc::c_int) as libc::c_int
                    })
                } else {
                    (0 as libc::c_int / columns < lines_per_body) as libc::c_int
                })
            })
        } else {
            (if columns == 0 as libc::c_int {
                0 as libc::c_int
            } else {
                (if lines_per_body < 0 as libc::c_int {
                    (if (if (if ((if 1 as libc::c_int != 0 {
                        0 as libc::c_int
                    } else {
                        (if 1 as libc::c_int != 0 {
                            0 as libc::c_int
                        } else {
                            lines_per_body
                        }) + 0 as libc::c_int
                    }) - 1 as libc::c_int) < 0 as libc::c_int
                    {
                        !(((((if 1 as libc::c_int != 0 {
                            0 as libc::c_int
                        } else {
                            (if 1 as libc::c_int != 0 {
                                0 as libc::c_int
                            } else {
                                lines_per_body
                            }) + 0 as libc::c_int
                        }) + 1 as libc::c_int)
                            << (::core::mem::size_of::<libc::c_int>() as libc::c_ulong)
                                .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                            - 1 as libc::c_int) * 2 as libc::c_int + 1 as libc::c_int)
                    } else {
                        (if 1 as libc::c_int != 0 {
                            0 as libc::c_int
                        } else {
                            (if 1 as libc::c_int != 0 {
                                0 as libc::c_int
                            } else {
                                lines_per_body
                            }) + 0 as libc::c_int
                        }) + 0 as libc::c_int
                    }) < 0 as libc::c_int
                    {
                        (((if 1 as libc::c_int != 0 {
                            0 as libc::c_int
                        } else {
                            lines_per_body
                        }) + 0 as libc::c_int)
                            < -(if ((if 1 as libc::c_int != 0 {
                                0 as libc::c_int
                            } else {
                                (if 1 as libc::c_int != 0 {
                                    0 as libc::c_int
                                } else {
                                    lines_per_body
                                }) + 0 as libc::c_int
                            }) - 1 as libc::c_int) < 0 as libc::c_int
                            {
                                ((((if 1 as libc::c_int != 0 {
                                    0 as libc::c_int
                                } else {
                                    (if 1 as libc::c_int != 0 {
                                        0 as libc::c_int
                                    } else {
                                        lines_per_body
                                    }) + 0 as libc::c_int
                                }) + 1 as libc::c_int)
                                    << (::core::mem::size_of::<libc::c_int>() as libc::c_ulong)
                                        .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                        .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                    - 1 as libc::c_int) * 2 as libc::c_int + 1 as libc::c_int
                            } else {
                                (if 1 as libc::c_int != 0 {
                                    0 as libc::c_int
                                } else {
                                    (if 1 as libc::c_int != 0 {
                                        0 as libc::c_int
                                    } else {
                                        lines_per_body
                                    }) + 0 as libc::c_int
                                }) - 1 as libc::c_int
                            })) as libc::c_int
                    } else {
                        ((0 as libc::c_int)
                            < (if 1 as libc::c_int != 0 {
                                0 as libc::c_int
                            } else {
                                lines_per_body
                            }) + 0 as libc::c_int) as libc::c_int
                    }) != 0 && lines_per_body == -(1 as libc::c_int)
                    {
                        (if ((if 1 as libc::c_int != 0 {
                            0 as libc::c_int
                        } else {
                            columns
                        }) - 1 as libc::c_int) < 0 as libc::c_int
                        {
                            ((0 as libc::c_int) < columns + 0 as libc::c_int)
                                as libc::c_int
                        } else {
                            ((-(1 as libc::c_int) - 0 as libc::c_int)
                                < columns - 1 as libc::c_int) as libc::c_int
                        })
                    } else {
                        (0 as libc::c_int / lines_per_body < columns) as libc::c_int
                    })
                } else {
                    (-(1 as libc::c_int) / columns < lines_per_body) as libc::c_int
                })
            })
        }) != 0
    {
        let (fresh15, fresh16) = lines_per_body.overflowing_mul(columns);
        *(&mut total_lines as *mut libc::c_int) = fresh15;
        1 as libc::c_int
    } else {
        let (fresh17, fresh18) = lines_per_body.overflowing_mul(columns);
        *(&mut total_lines as *mut libc::c_int) = fresh17;
        fresh18 as libc::c_int
    }) != 0
        || {
            let (fresh19, fresh20) = total_lines.overflowing_add(1 as libc::c_int);
            *(&mut total_lines_1 as *mut libc::c_int) = fresh19;
            fresh20 as libc::c_int != 0
        }
        || {
            let (fresh21, fresh22) = chars_per_column.overflowing_add(1 as libc::c_int);
            *(&mut chars_per_column_1 as *mut libc::c_int) = fresh21;
            fresh22 as libc::c_int != 0
        }
        || (if (0 as libc::c_int) < -(1 as libc::c_int)
            && ((if 1 as libc::c_int != 0 { 0 as libc::c_int } else { total_lines })
                - 1 as libc::c_int) < 0 as libc::c_int
            && ((if 1 as libc::c_int != 0 {
                0 as libc::c_int
            } else {
                chars_per_column_1
            }) - 1 as libc::c_int) < 0 as libc::c_int
            && (if chars_per_column_1 < 0 as libc::c_int {
                (if total_lines < 0 as libc::c_int {
                    (if ((if 1 as libc::c_int != 0 {
                        0 as libc::c_int
                    } else {
                        (if 1 as libc::c_int != 0 {
                            0 as libc::c_int
                        } else {
                            -(1 as libc::c_int)
                        }) + chars_per_column_1
                    }) - 1 as libc::c_int) < 0 as libc::c_int
                    {
                        (total_lines < -(1 as libc::c_int) / chars_per_column_1)
                            as libc::c_int
                    } else {
                        ((if (if (if ((if 1 as libc::c_int != 0 {
                            0 as libc::c_int
                        } else {
                            chars_per_column_1
                        }) - 1 as libc::c_int) < 0 as libc::c_int
                        {
                            !(((((if 1 as libc::c_int != 0 {
                                0 as libc::c_int
                            } else {
                                chars_per_column_1
                            }) + 1 as libc::c_int)
                                << (::core::mem::size_of::<libc::c_int>() as libc::c_ulong)
                                    .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                    .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                - 1 as libc::c_int) * 2 as libc::c_int + 1 as libc::c_int)
                        } else {
                            (if 1 as libc::c_int != 0 {
                                0 as libc::c_int
                            } else {
                                chars_per_column_1
                            }) + 0 as libc::c_int
                        }) < 0 as libc::c_int
                        {
                            (chars_per_column_1
                                < -(if ((if 1 as libc::c_int != 0 {
                                    0 as libc::c_int
                                } else {
                                    chars_per_column_1
                                }) - 1 as libc::c_int) < 0 as libc::c_int
                                {
                                    ((((if 1 as libc::c_int != 0 {
                                        0 as libc::c_int
                                    } else {
                                        chars_per_column_1
                                    }) + 1 as libc::c_int)
                                        << (::core::mem::size_of::<libc::c_int>() as libc::c_ulong)
                                            .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                            .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                        - 1 as libc::c_int) * 2 as libc::c_int + 1 as libc::c_int
                                } else {
                                    (if 1 as libc::c_int != 0 {
                                        0 as libc::c_int
                                    } else {
                                        chars_per_column_1
                                    }) - 1 as libc::c_int
                                })) as libc::c_int
                        } else {
                            ((0 as libc::c_int) < chars_per_column_1) as libc::c_int
                        }) != 0
                        {
                            (if 1 as libc::c_int != 0 {
                                0 as libc::c_int
                            } else {
                                chars_per_column_1
                            }) + -(1 as libc::c_int)
                                >> (::core::mem::size_of::<libc::c_int>() as libc::c_ulong)
                                    .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                    .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                        } else {
                            -(1 as libc::c_int) / -chars_per_column_1
                        }) <= -(1 as libc::c_int) - total_lines) as libc::c_int
                    })
                } else {
                    (if (if (if ((if 1 as libc::c_int != 0 {
                        0 as libc::c_int
                    } else {
                        (if 1 as libc::c_int != 0 {
                            0 as libc::c_int
                        } else {
                            chars_per_column_1
                        }) + 0 as libc::c_int
                    }) - 1 as libc::c_int) < 0 as libc::c_int
                    {
                        !(((((if 1 as libc::c_int != 0 {
                            0 as libc::c_int
                        } else {
                            (if 1 as libc::c_int != 0 {
                                0 as libc::c_int
                            } else {
                                chars_per_column_1
                            }) + 0 as libc::c_int
                        }) + 1 as libc::c_int)
                            << (::core::mem::size_of::<libc::c_int>() as libc::c_ulong)
                                .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                            - 1 as libc::c_int) * 2 as libc::c_int + 1 as libc::c_int)
                    } else {
                        (if 1 as libc::c_int != 0 {
                            0 as libc::c_int
                        } else {
                            (if 1 as libc::c_int != 0 {
                                0 as libc::c_int
                            } else {
                                chars_per_column_1
                            }) + 0 as libc::c_int
                        }) + 0 as libc::c_int
                    }) < 0 as libc::c_int
                    {
                        (((if 1 as libc::c_int != 0 {
                            0 as libc::c_int
                        } else {
                            chars_per_column_1
                        }) + 0 as libc::c_int)
                            < -(if ((if 1 as libc::c_int != 0 {
                                0 as libc::c_int
                            } else {
                                (if 1 as libc::c_int != 0 {
                                    0 as libc::c_int
                                } else {
                                    chars_per_column_1
                                }) + 0 as libc::c_int
                            }) - 1 as libc::c_int) < 0 as libc::c_int
                            {
                                ((((if 1 as libc::c_int != 0 {
                                    0 as libc::c_int
                                } else {
                                    (if 1 as libc::c_int != 0 {
                                        0 as libc::c_int
                                    } else {
                                        chars_per_column_1
                                    }) + 0 as libc::c_int
                                }) + 1 as libc::c_int)
                                    << (::core::mem::size_of::<libc::c_int>() as libc::c_ulong)
                                        .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                        .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                    - 1 as libc::c_int) * 2 as libc::c_int + 1 as libc::c_int
                            } else {
                                (if 1 as libc::c_int != 0 {
                                    0 as libc::c_int
                                } else {
                                    (if 1 as libc::c_int != 0 {
                                        0 as libc::c_int
                                    } else {
                                        chars_per_column_1
                                    }) + 0 as libc::c_int
                                }) - 1 as libc::c_int
                            })) as libc::c_int
                    } else {
                        ((0 as libc::c_int)
                            < (if 1 as libc::c_int != 0 {
                                0 as libc::c_int
                            } else {
                                chars_per_column_1
                            }) + 0 as libc::c_int) as libc::c_int
                    }) != 0 && chars_per_column_1 == -(1 as libc::c_int)
                    {
                        (if ((if 1 as libc::c_int != 0 {
                            0 as libc::c_int
                        } else {
                            total_lines
                        }) - 1 as libc::c_int) < 0 as libc::c_int
                        {
                            ((0 as libc::c_int) < total_lines + 0 as libc::c_int)
                                as libc::c_int
                        } else {
                            ((0 as libc::c_int) < total_lines
                                && (-(1 as libc::c_int) - 0 as libc::c_int)
                                    < total_lines - 1 as libc::c_int) as libc::c_int
                        })
                    } else {
                        (0 as libc::c_int / chars_per_column_1 < total_lines)
                            as libc::c_int
                    })
                })
            } else {
                (if chars_per_column_1 == 0 as libc::c_int {
                    0 as libc::c_int
                } else {
                    (if total_lines < 0 as libc::c_int {
                        (if (if (if ((if 1 as libc::c_int != 0 {
                            0 as libc::c_int
                        } else {
                            (if 1 as libc::c_int != 0 {
                                0 as libc::c_int
                            } else {
                                total_lines
                            }) + 0 as libc::c_int
                        }) - 1 as libc::c_int) < 0 as libc::c_int
                        {
                            !(((((if 1 as libc::c_int != 0 {
                                0 as libc::c_int
                            } else {
                                (if 1 as libc::c_int != 0 {
                                    0 as libc::c_int
                                } else {
                                    total_lines
                                }) + 0 as libc::c_int
                            }) + 1 as libc::c_int)
                                << (::core::mem::size_of::<libc::c_int>() as libc::c_ulong)
                                    .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                    .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                - 1 as libc::c_int) * 2 as libc::c_int + 1 as libc::c_int)
                        } else {
                            (if 1 as libc::c_int != 0 {
                                0 as libc::c_int
                            } else {
                                (if 1 as libc::c_int != 0 {
                                    0 as libc::c_int
                                } else {
                                    total_lines
                                }) + 0 as libc::c_int
                            }) + 0 as libc::c_int
                        }) < 0 as libc::c_int
                        {
                            (((if 1 as libc::c_int != 0 {
                                0 as libc::c_int
                            } else {
                                total_lines
                            }) + 0 as libc::c_int)
                                < -(if ((if 1 as libc::c_int != 0 {
                                    0 as libc::c_int
                                } else {
                                    (if 1 as libc::c_int != 0 {
                                        0 as libc::c_int
                                    } else {
                                        total_lines
                                    }) + 0 as libc::c_int
                                }) - 1 as libc::c_int) < 0 as libc::c_int
                                {
                                    ((((if 1 as libc::c_int != 0 {
                                        0 as libc::c_int
                                    } else {
                                        (if 1 as libc::c_int != 0 {
                                            0 as libc::c_int
                                        } else {
                                            total_lines
                                        }) + 0 as libc::c_int
                                    }) + 1 as libc::c_int)
                                        << (::core::mem::size_of::<libc::c_int>() as libc::c_ulong)
                                            .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                            .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                        - 1 as libc::c_int) * 2 as libc::c_int + 1 as libc::c_int
                                } else {
                                    (if 1 as libc::c_int != 0 {
                                        0 as libc::c_int
                                    } else {
                                        (if 1 as libc::c_int != 0 {
                                            0 as libc::c_int
                                        } else {
                                            total_lines
                                        }) + 0 as libc::c_int
                                    }) - 1 as libc::c_int
                                })) as libc::c_int
                        } else {
                            ((0 as libc::c_int)
                                < (if 1 as libc::c_int != 0 {
                                    0 as libc::c_int
                                } else {
                                    total_lines
                                }) + 0 as libc::c_int) as libc::c_int
                        }) != 0 && total_lines == -(1 as libc::c_int)
                        {
                            (if ((if 1 as libc::c_int != 0 {
                                0 as libc::c_int
                            } else {
                                chars_per_column_1
                            }) - 1 as libc::c_int) < 0 as libc::c_int
                            {
                                ((0 as libc::c_int) < chars_per_column_1 + 0 as libc::c_int)
                                    as libc::c_int
                            } else {
                                ((-(1 as libc::c_int) - 0 as libc::c_int)
                                    < chars_per_column_1 - 1 as libc::c_int) as libc::c_int
                            })
                        } else {
                            (0 as libc::c_int / total_lines < chars_per_column_1)
                                as libc::c_int
                        })
                    } else {
                        (-(1 as libc::c_int) / chars_per_column_1 < total_lines)
                            as libc::c_int
                    })
                })
            }) != 0
        {
            let (fresh23, fresh24) = total_lines.overflowing_mul(chars_per_column_1);
            *(&mut chars_if_truncate as *mut libc::c_int) = fresh23;
            1 as libc::c_int
        } else {
            let (fresh25, fresh26) = total_lines.overflowing_mul(chars_per_column_1);
            *(&mut chars_if_truncate as *mut libc::c_int) = fresh25;
            fresh26 as libc::c_int
        }) != 0
    {
        integer_overflow();
    }
    free(line_vector as *mut libc::c_void);
    line_vector = xnmalloc(
        total_lines_1 as size_t,
        ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
    ) as *mut libc::c_int;
    free(end_vector as *mut libc::c_void);
    end_vector = xnmalloc(
        total_lines as size_t,
        ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
    ) as *mut libc::c_int;
    free(buff as *mut libc::c_void);
    buff = xnmalloc(
        chars_if_truncate as size_t,
        (use_col_separator as libc::c_int + 1 as libc::c_int) as size_t,
    ) as *mut libc::c_char;
    buff_allocated = chars_if_truncate as size_t;
    buff_allocated = (buff_allocated as libc::c_ulong)
        .wrapping_mul(
            (use_col_separator as libc::c_int + 1 as libc::c_int) as libc::c_ulong,
        ) as size_t as size_t;
}
unsafe extern "C" fn store_columns() {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut line: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    let mut buff_start: libc::c_uint = 0;
    let mut last_col: libc::c_int = 0;
    let mut p: *mut COLUMN = 0 as *mut COLUMN;
    buff_current = 0 as libc::c_int as libc::c_uint;
    buff_start = 0 as libc::c_int as libc::c_uint;
    if balance_columns {
        last_col = columns;
    } else {
        last_col = columns - 1 as libc::c_int;
    }
    i = 1 as libc::c_int;
    p = column_vector;
    while i <= last_col {
        (*p).lines_stored = 0 as libc::c_int;
        i += 1;
        p = p.offset(1);
    }
    i = 1 as libc::c_int;
    p = column_vector;
    while i <= last_col && files_ready_to_read != 0 {
        (*p).current_line = line as libc::c_int;
        j = lines_per_body;
        while j != 0 && files_ready_to_read != 0 {
            if (*p).status as libc::c_uint == OPEN as libc::c_int as libc::c_uint {
                input_position = 0 as libc::c_int;
                if !read_line(p) {
                    read_rest_of_line(p);
                }
                if (*p).status as libc::c_uint == OPEN as libc::c_int as libc::c_uint
                    || buff_start != buff_current
                {
                    (*p).lines_stored += 1;
                    *line_vector.offset(line as isize) = buff_start as libc::c_int;
                    let fresh27 = line;
                    line = line.wrapping_add(1);
                    *end_vector.offset(fresh27 as isize) = input_position;
                    buff_start = buff_current;
                }
            }
            j -= 1;
        }
        i += 1;
        p = p.offset(1);
    }
    *line_vector.offset(line as isize) = buff_start as libc::c_int;
    if balance_columns {
        balance(line as libc::c_int);
    }
}
unsafe extern "C" fn balance(mut total_stored: libc::c_int) {
    let mut p: *mut COLUMN = 0 as *mut COLUMN;
    let mut i: libc::c_int = 0;
    let mut lines: libc::c_int = 0;
    let mut first_line: libc::c_int = 0 as libc::c_int;
    i = 1 as libc::c_int;
    p = column_vector;
    while i <= columns {
        lines = total_stored / columns;
        if i <= total_stored % columns {
            lines += 1;
        }
        (*p).lines_stored = lines;
        (*p).current_line = first_line;
        first_line += lines;
        i += 1;
        p = p.offset(1);
    }
}
unsafe extern "C" fn store_char(mut c: libc::c_char) {
    if buff_current as libc::c_ulong >= buff_allocated {
        buff = (if ::core::mem::size_of::<C2RustUnnamed_4>() as libc::c_ulong != 0 {
            x2realloc(buff as *mut libc::c_void, &mut buff_allocated)
        } else {
            x2realloc(buff as *mut libc::c_void, &mut buff_allocated)
        }) as *mut libc::c_char;
    }
    let fresh28 = buff_current;
    buff_current = buff_current.wrapping_add(1);
    *buff.offset(fresh28 as isize) = c;
}
unsafe extern "C" fn add_line_number(mut p: *mut COLUMN) {
    let mut i: libc::c_int = 0;
    let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut num_width: libc::c_int = 0;
    num_width = sprintf(
        number_buff,
        b"%*d\0" as *const u8 as *const libc::c_char,
        chars_per_number,
        line_number,
    );
    line_number += 1;
    s = number_buff.offset((num_width - chars_per_number) as isize);
    i = chars_per_number;
    while i > 0 as libc::c_int {
        let fresh29 = s;
        s = s.offset(1);
        ((*p).char_func).expect("non-null function pointer")(*fresh29);
        i -= 1;
    }
    if columns > 1 as libc::c_int {
        if number_separator as libc::c_int == '\t' as i32 {
            i = number_width - chars_per_number;
            loop {
                let fresh30 = i;
                i = i - 1;
                if !(fresh30 > 0 as libc::c_int) {
                    break;
                }
                ((*p).char_func)
                    .expect("non-null function pointer")(' ' as i32 as libc::c_char);
            }
        } else {
            ((*p).char_func).expect("non-null function pointer")(number_separator);
        }
    } else {
        ((*p).char_func).expect("non-null function pointer")(number_separator);
        if number_separator as libc::c_int == '\t' as i32 {
            output_position = output_position
                + (chars_per_output_tab - output_position % chars_per_output_tab);
        }
    }
    if truncate_lines as libc::c_int != 0 && !parallel_files {
        input_position += number_width;
    }
}
unsafe extern "C" fn pad_across_to(mut position: libc::c_int) {
    let mut h: libc::c_int = output_position;
    if tabify_output {
        spaces_not_printed = position - output_position;
    } else {
        loop {
            h += 1;
            if !(h <= position) {
                break;
            }
            putchar_unlocked(' ' as i32);
        }
        output_position = position;
    };
}
unsafe extern "C" fn pad_down(mut lines: libc::c_uint) {
    if use_form_feed {
        putchar_unlocked('\u{c}' as i32);
    } else {
        let mut i: libc::c_uint = lines;
        while i != 0 {
            putchar_unlocked('\n' as i32);
            i = i.wrapping_sub(1);
        }
    };
}
unsafe extern "C" fn read_rest_of_line(mut p: *mut COLUMN) {
    let mut c: libc::c_int = 0;
    let mut f: *mut FILE = (*p).fp;
    loop {
        c = getc_unlocked(f);
        if !(c != '\n' as i32) {
            break;
        }
        if c == '\u{c}' as i32 {
            c = getc_unlocked(f);
            if c != '\n' as i32 {
                ungetc(c, f);
            }
            if keep_FF {
                print_a_FF = 1 as libc::c_int != 0;
            }
            hold_file(p);
            break;
        } else {
            if !(c == -(1 as libc::c_int)) {
                continue;
            }
            close_file(p);
            break;
        }
    };
}
unsafe extern "C" fn skip_read(mut p: *mut COLUMN, mut column_number: libc::c_int) {
    let mut c: libc::c_int = 0;
    let mut f: *mut FILE = (*p).fp;
    let mut i: libc::c_int = 0;
    let mut single_ff: bool = 0 as libc::c_int != 0;
    let mut q: *mut COLUMN = 0 as *mut COLUMN;
    c = getc_unlocked(f);
    if c == '\u{c}' as i32 && (*p).full_page_printed as libc::c_int != 0 {
        c = getc_unlocked(f);
        if c == '\n' as i32 {
            c = getc_unlocked(f);
        }
    }
    (*p).full_page_printed = 0 as libc::c_int != 0;
    if c == '\u{c}' as i32 {
        single_ff = 1 as libc::c_int != 0;
    }
    if last_line {
        (*p).full_page_printed = 1 as libc::c_int != 0;
    }
    while c != '\n' as i32 {
        if c == '\u{c}' as i32 {
            if last_line {
                if !parallel_files {
                    q = column_vector;
                    i = columns;
                    while i != 0 {
                        (*q).full_page_printed = 0 as libc::c_int != 0;
                        q = q.offset(1);
                        i -= 1;
                    }
                } else {
                    (*p).full_page_printed = 0 as libc::c_int != 0;
                }
            }
            c = getc_unlocked(f);
            if c != '\n' as i32 {
                ungetc(c, f);
            }
            hold_file(p);
            break;
        } else if c == -(1 as libc::c_int) {
            close_file(p);
            break;
        } else {
            c = getc_unlocked(f);
        }
    }
    if skip_count {
        if (!parallel_files || column_number == 1 as libc::c_int) && !single_ff {
            line_count += 1;
        }
    }
}
unsafe extern "C" fn print_white_space() {
    let mut h_new: libc::c_int = 0;
    let mut h_old: libc::c_int = output_position;
    let mut goal: libc::c_int = h_old + spaces_not_printed;
    while goal - h_old > 1 as libc::c_int
        && {
            h_new = h_old + (chars_per_output_tab - h_old % chars_per_output_tab);
            h_new <= goal
        }
    {
        putchar_unlocked(output_tab_char as libc::c_int);
        h_old = h_new;
    }
    loop {
        h_old += 1;
        if !(h_old <= goal) {
            break;
        }
        putchar_unlocked(' ' as i32);
    }
    output_position = goal;
    spaces_not_printed = 0 as libc::c_int;
}
unsafe extern "C" fn print_sep_string() {
    let mut s: *const libc::c_char = col_sep_string;
    let mut l: libc::c_int = col_sep_length;
    if separators_not_printed <= 0 as libc::c_int {
        if spaces_not_printed > 0 as libc::c_int {
            print_white_space();
        }
    } else {
        while separators_not_printed > 0 as libc::c_int {
            loop {
                let fresh31 = l;
                l = l - 1;
                if !(fresh31 > 0 as libc::c_int) {
                    break;
                }
                if *s as libc::c_int == ' ' as i32 {
                    s = s.offset(1);
                    spaces_not_printed += 1;
                } else {
                    if spaces_not_printed > 0 as libc::c_int {
                        print_white_space();
                    }
                    let fresh32 = s;
                    s = s.offset(1);
                    putchar_unlocked(*fresh32 as libc::c_int);
                    output_position += 1;
                }
            }
            if spaces_not_printed > 0 as libc::c_int {
                print_white_space();
            }
            separators_not_printed -= 1;
        }
    };
}
unsafe extern "C" fn print_clump(
    mut p: *mut COLUMN,
    mut n: libc::c_int,
    mut clump: *mut libc::c_char,
) {
    loop {
        let fresh33 = n;
        n = n - 1;
        if !(fresh33 != 0) {
            break;
        }
        let fresh34 = clump;
        clump = clump.offset(1);
        ((*p).char_func).expect("non-null function pointer")(*fresh34);
    };
}
unsafe extern "C" fn print_char(mut c: libc::c_char) {
    if tabify_output {
        if c as libc::c_int == ' ' as i32 {
            spaces_not_printed += 1;
            return;
        } else {
            if spaces_not_printed > 0 as libc::c_int {
                print_white_space();
            }
        }
        if *(*__ctype_b_loc()).offset(to_uchar(c) as libc::c_int as isize) as libc::c_int
            & _ISprint as libc::c_int as libc::c_ushort as libc::c_int == 0
        {
            if c as libc::c_int == '\u{8}' as i32 {
                output_position -= 1;
            }
        } else {
            output_position += 1;
        }
    }
    putchar_unlocked(c as libc::c_int);
}
unsafe extern "C" fn skip_to_page(mut page: uintmax_t) -> bool {
    let mut n: uintmax_t = 1 as libc::c_int as uintmax_t;
    while n < page {
        let mut p: *mut COLUMN = 0 as *mut COLUMN;
        let mut j: libc::c_int = 0;
        let mut i: libc::c_int = 1 as libc::c_int;
        while i < lines_per_body {
            j = 1 as libc::c_int;
            p = column_vector;
            while j <= columns {
                if (*p).status as libc::c_uint == OPEN as libc::c_int as libc::c_uint {
                    skip_read(p, j);
                }
                j += 1;
                p = p.offset(1);
            }
            i += 1;
        }
        last_line = 1 as libc::c_int != 0;
        j = 1 as libc::c_int;
        p = column_vector;
        while j <= columns {
            if (*p).status as libc::c_uint == OPEN as libc::c_int as libc::c_uint {
                skip_read(p, j);
            }
            j += 1;
            p = p.offset(1);
        }
        if storing_columns {
            j = 1 as libc::c_int;
            p = column_vector;
            while j <= columns {
                if (*p).status as libc::c_uint != CLOSED as libc::c_int as libc::c_uint {
                    (*p).status = ON_HOLD;
                }
                j += 1;
                p = p.offset(1);
            }
        }
        reset_status();
        last_line = 0 as libc::c_int != 0;
        if files_ready_to_read < 1 as libc::c_int {
            error(
                0 as libc::c_int,
                0 as libc::c_int,
                dcgettext(
                    0 as *const libc::c_char,
                    b"starting page number %lu exceeds page count %lu\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
                page,
                n,
            );
            break;
        } else {
            n = n.wrapping_add(1);
        }
    }
    return files_ready_to_read > 0 as libc::c_int;
}
unsafe extern "C" fn print_header() {
    let mut page_text: [libc::c_char; 276] = [0; 276];
    let mut available_width: libc::c_int = 0;
    let mut lhs_spaces: libc::c_int = 0;
    let mut rhs_spaces: libc::c_int = 0;
    output_position = 0 as libc::c_int;
    pad_across_to(chars_per_margin);
    print_white_space();
    if page_number == 0 as libc::c_int as libc::c_ulong {
        if ::core::mem::size_of::<C2RustUnnamed_3>() as libc::c_ulong != 0 {
            error(
                1 as libc::c_int,
                0 as libc::c_int,
                dcgettext(
                    0 as *const libc::c_char,
                    b"page number overflow\0" as *const u8 as *const libc::c_char,
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
                    b"page number overflow\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
            );
            if 0 as libc::c_int != 0 {} else {
                unreachable!();
            };
        };
    }
    sprintf(
        page_text.as_mut_ptr(),
        dcgettext(
            0 as *const libc::c_char,
            b"Page %lu\0" as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        ),
        page_number,
    );
    available_width = header_width_available
        - gnu_mbswidth(page_text.as_mut_ptr(), 0 as libc::c_int);
    available_width = if 0 as libc::c_int > available_width {
        0 as libc::c_int
    } else {
        available_width
    };
    lhs_spaces = available_width >> 1 as libc::c_int;
    rhs_spaces = available_width - lhs_spaces;
    printf(
        b"\n\n%*s%s%*s%s%*s%s\n\n\n\0" as *const u8 as *const libc::c_char,
        chars_per_margin,
        b"\0" as *const u8 as *const libc::c_char,
        date_text,
        lhs_spaces,
        b" \0" as *const u8 as *const libc::c_char,
        file_text,
        rhs_spaces,
        b" \0" as *const u8 as *const libc::c_char,
        page_text.as_mut_ptr(),
    );
    print_a_header = 0 as libc::c_int != 0;
    output_position = 0 as libc::c_int;
}
unsafe extern "C" fn read_line(mut p: *mut COLUMN) -> bool {
    let mut c: libc::c_int = 0;
    let mut chars: libc::c_int = 0;
    let mut last_input_position: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut q: *mut COLUMN = 0 as *mut COLUMN;
    c = getc_unlocked((*p).fp);
    last_input_position = input_position;
    if c == '\u{c}' as i32 && (*p).full_page_printed as libc::c_int != 0 {
        c = getc_unlocked((*p).fp);
        if c == '\n' as i32 {
            c = getc_unlocked((*p).fp);
        }
    }
    (*p).full_page_printed = 0 as libc::c_int != 0;
    match c {
        12 => {
            c = getc_unlocked((*p).fp);
            if c != '\n' as i32 {
                ungetc(c, (*p).fp);
            }
            FF_only = 1 as libc::c_int != 0;
            if print_a_header as libc::c_int != 0 && !storing_columns {
                pad_vertically = 1 as libc::c_int != 0;
                print_header();
            } else if keep_FF {
                print_a_FF = 1 as libc::c_int != 0;
            }
            hold_file(p);
            return 1 as libc::c_int != 0;
        }
        -1 => {
            close_file(p);
            return 1 as libc::c_int != 0;
        }
        10 => {}
        _ => {
            chars = char_to_clump(c as libc::c_char);
        }
    }
    if truncate_lines as libc::c_int != 0 && input_position > chars_per_column {
        input_position = last_input_position;
        return 0 as libc::c_int != 0;
    }
    if (*p).char_func != Some(store_char as unsafe extern "C" fn(libc::c_char) -> ()) {
        pad_vertically = 1 as libc::c_int != 0;
        if print_a_header as libc::c_int != 0 && !storing_columns {
            print_header();
        }
        if parallel_files as libc::c_int != 0 && align_empty_cols as libc::c_int != 0 {
            k = separators_not_printed;
            separators_not_printed = 0 as libc::c_int;
            j = 1 as libc::c_int;
            q = column_vector;
            while j <= k {
                align_column(q);
                separators_not_printed += 1 as libc::c_int;
                j += 1;
                q = q.offset(1);
            }
            padding_not_printed = (*p).start_position;
            if truncate_lines {
                spaces_not_printed = chars_per_column;
            } else {
                spaces_not_printed = 0 as libc::c_int;
            }
            align_empty_cols = 0 as libc::c_int != 0;
        }
        if col_sep_length < padding_not_printed {
            pad_across_to(padding_not_printed - col_sep_length);
            padding_not_printed = 0 as libc::c_int;
        }
        if use_col_separator {
            print_sep_string();
        }
    }
    if (*p).numbered {
        add_line_number(p);
    }
    empty_line = 0 as libc::c_int != 0;
    if c == '\n' as i32 {
        return 1 as libc::c_int != 0;
    }
    print_clump(p, chars, clump_buff);
    loop {
        c = getc_unlocked((*p).fp);
        match c {
            10 => return 1 as libc::c_int != 0,
            12 => {
                c = getc_unlocked((*p).fp);
                if c != '\n' as i32 {
                    ungetc(c, (*p).fp);
                }
                if keep_FF {
                    print_a_FF = 1 as libc::c_int != 0;
                }
                hold_file(p);
                return 1 as libc::c_int != 0;
            }
            -1 => {
                close_file(p);
                return 1 as libc::c_int != 0;
            }
            _ => {}
        }
        last_input_position = input_position;
        chars = char_to_clump(c as libc::c_char);
        if truncate_lines as libc::c_int != 0 && input_position > chars_per_column {
            input_position = last_input_position;
            return 0 as libc::c_int != 0;
        }
        print_clump(p, chars, clump_buff);
    };
}
unsafe extern "C" fn print_stored(mut p: *mut COLUMN) -> bool {
    let mut q: *mut COLUMN = 0 as *mut COLUMN;
    let fresh35 = (*p).current_line;
    (*p).current_line = (*p).current_line + 1;
    let mut line: libc::c_int = fresh35;
    let mut first: *mut libc::c_char = &mut *buff
        .offset(*line_vector.offset(line as isize) as isize) as *mut libc::c_char;
    let mut last: *mut libc::c_char = &mut *buff
        .offset(*line_vector.offset((line + 1 as libc::c_int) as isize) as isize)
        as *mut libc::c_char;
    pad_vertically = 1 as libc::c_int != 0;
    if print_a_header {
        print_header();
    }
    if (*p).status as libc::c_uint == FF_FOUND as libc::c_int as libc::c_uint {
        let mut i: libc::c_int = 0;
        i = 1 as libc::c_int;
        q = column_vector;
        while i <= columns {
            (*q).status = ON_HOLD;
            i += 1;
            q = q.offset(1);
        }
        if (*column_vector).lines_to_print <= 0 as libc::c_int {
            if !extremities {
                pad_vertically = 0 as libc::c_int != 0;
            }
            return 1 as libc::c_int != 0;
        }
    }
    if col_sep_length < padding_not_printed {
        pad_across_to(padding_not_printed - col_sep_length);
        padding_not_printed = 0 as libc::c_int;
    }
    if use_col_separator {
        print_sep_string();
    }
    while first != last {
        let fresh36 = first;
        first = first.offset(1);
        print_char(*fresh36);
    }
    if spaces_not_printed == 0 as libc::c_int {
        output_position = (*p).start_position + *end_vector.offset(line as isize);
        if (*p).start_position - col_sep_length == chars_per_margin {
            output_position -= col_sep_length;
        }
    }
    return 1 as libc::c_int != 0;
}
unsafe extern "C" fn char_to_clump(mut c: libc::c_char) -> libc::c_int {
    let mut uc: libc::c_uchar = c as libc::c_uchar;
    let mut s: *mut libc::c_char = clump_buff;
    let mut i: libc::c_int = 0;
    let mut esc_buff: [libc::c_char; 4] = [0; 4];
    let mut width: libc::c_int = 0;
    let mut chars: libc::c_int = 0;
    let mut chars_per_c: libc::c_int = 8 as libc::c_int;
    if c as libc::c_int == input_tab_char as libc::c_int {
        chars_per_c = chars_per_input_tab;
    }
    if c as libc::c_int == input_tab_char as libc::c_int
        || c as libc::c_int == '\t' as i32
    {
        width = chars_per_c - input_position % chars_per_c;
        if untabify_input {
            i = width;
            while i != 0 {
                let fresh37 = s;
                s = s.offset(1);
                *fresh37 = ' ' as i32 as libc::c_char;
                i -= 1;
            }
            chars = width;
        } else {
            *s = c;
            chars = 1 as libc::c_int;
        }
    } else if *(*__ctype_b_loc()).offset(uc as libc::c_int as isize) as libc::c_int
        & _ISprint as libc::c_int as libc::c_ushort as libc::c_int == 0
    {
        if use_esc_sequence {
            width = 4 as libc::c_int;
            chars = 4 as libc::c_int;
            let fresh38 = s;
            s = s.offset(1);
            *fresh38 = '\\' as i32 as libc::c_char;
            sprintf(
                esc_buff.as_mut_ptr(),
                b"%03o\0" as *const u8 as *const libc::c_char,
                uc as libc::c_int,
            );
            i = 0 as libc::c_int;
            while i <= 2 as libc::c_int {
                let fresh39 = s;
                s = s.offset(1);
                *fresh39 = esc_buff[i as usize];
                i += 1;
            }
        } else if use_cntrl_prefix {
            if (uc as libc::c_int) < 0o200 as libc::c_int {
                width = 2 as libc::c_int;
                chars = 2 as libc::c_int;
                let fresh40 = s;
                s = s.offset(1);
                *fresh40 = '^' as i32 as libc::c_char;
                *s = (c as libc::c_int ^ 0o100 as libc::c_int) as libc::c_char;
            } else {
                width = 4 as libc::c_int;
                chars = 4 as libc::c_int;
                let fresh41 = s;
                s = s.offset(1);
                *fresh41 = '\\' as i32 as libc::c_char;
                sprintf(
                    esc_buff.as_mut_ptr(),
                    b"%03o\0" as *const u8 as *const libc::c_char,
                    uc as libc::c_int,
                );
                i = 0 as libc::c_int;
                while i <= 2 as libc::c_int {
                    let fresh42 = s;
                    s = s.offset(1);
                    *fresh42 = esc_buff[i as usize];
                    i += 1;
                }
            }
        } else if c as libc::c_int == '\u{8}' as i32 {
            width = -(1 as libc::c_int);
            chars = 1 as libc::c_int;
            *s = c;
        } else {
            width = 0 as libc::c_int;
            chars = 1 as libc::c_int;
            *s = c;
        }
    } else {
        width = 1 as libc::c_int;
        chars = 1 as libc::c_int;
        *s = c;
    }
    if width < 0 as libc::c_int && input_position == 0 as libc::c_int {
        chars = 0 as libc::c_int;
        input_position = 0 as libc::c_int;
    } else if width < 0 as libc::c_int && input_position <= -width {
        input_position = 0 as libc::c_int;
    } else {
        input_position += width;
    }
    return chars;
}
unsafe extern "C" fn cleanup() {
    free(number_buff as *mut libc::c_void);
    free(clump_buff as *mut libc::c_void);
    free(column_vector as *mut libc::c_void);
    free(line_vector as *mut libc::c_void);
    free(end_vector as *mut libc::c_void);
    free(buff as *mut libc::c_void);
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
                b"Paginate or columnate FILE(s) for printing.\n\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
            stdout,
        );
        emit_stdin_note();
        emit_mandatory_arg_note();
        fputs_unlocked(
            dcgettext(
                0 as *const libc::c_char,
                b"  +FIRST_PAGE[:LAST_PAGE], --pages=FIRST_PAGE[:LAST_PAGE]\n                    begin [stop] printing with page FIRST_[LAST_]PAGE\n  -COLUMN, --columns=COLUMN\n                    output COLUMN columns and print columns down,\n                    unless -a is used. Balance number of lines in the\n                    columns on each page\n\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            stdout,
        );
        fputs_unlocked(
            dcgettext(
                0 as *const libc::c_char,
                b"  -a, --across      print columns across rather than down, used together\n                    with -COLUMN\n  -c, --show-control-chars\n                    use hat notation (^G) and octal backslash notation\n  -d, --double-space\n                    double space the output\n\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            stdout,
        );
        fputs_unlocked(
            dcgettext(
                0 as *const libc::c_char,
                b"  -D, --date-format=FORMAT\n                    use FORMAT for the header date\n  -e[CHAR[WIDTH]], --expand-tabs[=CHAR[WIDTH]]\n                    expand input CHARs (TABs) to tab WIDTH (8)\n  -F, -f, --form-feed\n                    use form feeds instead of newlines to separate pages\n                    (by a 3-line page header with -F or a 5-line header\n                    and trailer without -F)\n\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            stdout,
        );
        fputs_unlocked(
            dcgettext(
                0 as *const libc::c_char,
                b"  -h, --header=HEADER\n                    use a centered HEADER instead of filename in page header,\n                    -h \"\" prints a blank line, don't use -h\"\"\n  -i[CHAR[WIDTH]], --output-tabs[=CHAR[WIDTH]]\n                    replace spaces with CHARs (TABs) to tab WIDTH (8)\n  -J, --join-lines  merge full lines, turns off -W line truncation, no column\n                    alignment, --sep-string[=STRING] sets separators\n\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            stdout,
        );
        fputs_unlocked(
            dcgettext(
                0 as *const libc::c_char,
                b"  -l, --length=PAGE_LENGTH\n                    set the page length to PAGE_LENGTH (66) lines\n                    (default number of lines of text 56, and with -F 63).\n                    implies -t if PAGE_LENGTH <= 10\n\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            stdout,
        );
        fputs_unlocked(
            dcgettext(
                0 as *const libc::c_char,
                b"  -m, --merge       print all files in parallel, one in each column,\n                    truncate lines, but join lines of full length with -J\n\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            stdout,
        );
        fputs_unlocked(
            dcgettext(
                0 as *const libc::c_char,
                b"  -n[SEP[DIGITS]], --number-lines[=SEP[DIGITS]]\n                    number lines, use DIGITS (5) digits, then SEP (TAB),\n                    default counting starts with 1st line of input file\n  -N, --first-line-number=NUMBER\n                    start counting with NUMBER at 1st line of first\n                    page printed (see +FIRST_PAGE)\n\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            stdout,
        );
        fputs_unlocked(
            dcgettext(
                0 as *const libc::c_char,
                b"  -o, --indent=MARGIN\n                    offset each line with MARGIN (zero) spaces, do not\n                    affect -w or -W, MARGIN will be added to PAGE_WIDTH\n  -r, --no-file-warnings\n                    omit warning when a file cannot be opened\n\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            stdout,
        );
        fputs_unlocked(
            dcgettext(
                0 as *const libc::c_char,
                b"  -s[CHAR], --separator[=CHAR]\n                    separate columns by a single character, default for CHAR\n                    is the <TAB> character without -w and 'no char' with -w.\n                    -s[CHAR] turns off line truncation of all 3 column\n                    options (-COLUMN|-a -COLUMN|-m) except -w is set\n\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            stdout,
        );
        fputs_unlocked(
            dcgettext(
                0 as *const libc::c_char,
                b"  -S[STRING], --sep-string[=STRING]\n                    separate columns by STRING,\n                    without -S: Default separator <TAB> with -J and <space>\n                    otherwise (same as -S\" \"), no effect on column options\n\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            stdout,
        );
        fputs_unlocked(
            dcgettext(
                0 as *const libc::c_char,
                b"  -t, --omit-header  omit page headers and trailers;\n                     implied if PAGE_LENGTH <= 10\n\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            stdout,
        );
        fputs_unlocked(
            dcgettext(
                0 as *const libc::c_char,
                b"  -T, --omit-pagination\n                    omit page headers and trailers, eliminate any pagination\n                    by form feeds set in input files\n  -v, --show-nonprinting\n                    use octal backslash notation\n  -w, --width=PAGE_WIDTH\n                    set page width to PAGE_WIDTH (72) characters for\n                    multiple text-column output only, -s[char] turns off (72)\n\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            stdout,
        );
        fputs_unlocked(
            dcgettext(
                0 as *const libc::c_char,
                b"  -W, --page-width=PAGE_WIDTH\n                    set page width to PAGE_WIDTH (72) characters always,\n                    truncate lines, except -J option is set, no interference\n                    with -S or -s\n\0"
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
        emit_ancillary_info(b"pr\0" as *const u8 as *const libc::c_char);
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
