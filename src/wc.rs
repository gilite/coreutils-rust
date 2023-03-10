#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![feature(extern_types, label_break_value)]
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type argv_iterator;
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    fn rpl_fclose(stream: *mut FILE) -> libc::c_int;
    static mut stdin: *mut FILE;
    static mut stdout: *mut FILE;
    static mut stderr: *mut FILE;
    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
    fn setvbuf(
        __stream: *mut FILE,
        __buf: *mut libc::c_char,
        __modes: libc::c_int,
        __n: size_t,
    ) -> libc::c_int;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn __overflow(_: *mut FILE, _: libc::c_int) -> libc::c_int;
    fn fputs_unlocked(__s: *const libc::c_char, __stream: *mut FILE) -> libc::c_int;
    fn fileno(__stream: *mut FILE) -> libc::c_int;
    static mut optarg: *mut libc::c_char;
    fn getopt_long(
        ___argc: libc::c_int,
        ___argv: *const *mut libc::c_char,
        __shortopts: *const libc::c_char,
        __longopts: *const option,
        __longind: *mut libc::c_int,
    ) -> libc::c_int;
    static mut optind: libc::c_int;
    fn __btowc_alias(__c: libc::c_int) -> wint_t;
    fn mbsinit(__ps: *const mbstate_t) -> libc::c_int;
    fn wcwidth(__c: wchar_t) -> libc::c_int;
    fn free(_: *mut libc::c_void);
    fn rpl_mbrtowc(
        pwc: *mut wchar_t,
        s: *const libc::c_char,
        n: size_t,
        ps: *mut mbstate_t,
    ) -> size_t;
    fn iswprint(__wc: wint_t) -> libc::c_int;
    fn iswspace(__wc: wint_t) -> libc::c_int;
    fn lseek(__fd: libc::c_int, __offset: __off_t, __whence: libc::c_int) -> __off_t;
    fn close(__fd: libc::c_int) -> libc::c_int;
    fn stat(__file: *const libc::c_char, __buf: *mut stat) -> libc::c_int;
    fn fstat(__fd: libc::c_int, __buf: *mut stat) -> libc::c_int;
    fn getpagesize() -> libc::c_int;
    fn memmove(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn rawmemchr(__s: *const libc::c_void, __c: libc::c_int) -> *mut libc::c_void;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strncmp(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn __errno_location() -> *mut libc::c_int;
    fn __ctype_get_mb_cur_max() -> size_t;
    fn quotearg_n_style_colon(
        n: libc::c_int,
        s: quoting_style,
        arg: *const libc::c_char,
    ) -> *mut libc::c_char;
    fn quotearg_style(s: quoting_style, arg: *const libc::c_char) -> *mut libc::c_char;
    fn umaxtostr(_: uintmax_t, _: *mut libc::c_char) -> *mut libc::c_char;
    static mut Version: *const libc::c_char;
    fn open(__file: *const libc::c_char, __oflag: libc::c_int, _: ...) -> libc::c_int;
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
    fn atexit(__func: Option::<unsafe extern "C" fn() -> ()>) -> libc::c_int;
    fn exit(_: libc::c_int) -> !;
    fn getenv(__name: *const libc::c_char) -> *mut libc::c_char;
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
    fn argv_iter_free(_: *mut argv_iterator);
    fn argv_iter_init_argv(argv: *mut *mut libc::c_char) -> *mut argv_iterator;
    fn argv_iter_init_stream(fp: *mut FILE) -> *mut argv_iterator;
    fn argv_iter(_: *mut argv_iterator, _: *mut argv_iter_err) -> *mut libc::c_char;
    fn argv_iter_n_args(_: *const argv_iterator) -> size_t;
    fn error(
        __status: libc::c_int,
        __errnum: libc::c_int,
        __format: *const libc::c_char,
        _: ...
    );
    fn fdadvise(fd: libc::c_int, offset: off_t, len: off_t, advice: fadvice_t);
    static is_basic_table: [libc::c_uint; 0];
    fn physmem_available() -> libc::c_double;
    fn readtokens0_init(t: *mut Tokens);
    fn readtokens0_free(t: *mut Tokens);
    fn readtokens0(in_0: *mut FILE, t: *mut Tokens) -> bool;
    fn safe_read(fd: libc::c_int, buf: *mut libc::c_void, count: size_t) -> size_t;
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
pub type off_t = __off_t;
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
pub type C2RustUnnamed_3 = libc::c_uint;
pub const _ISalnum: C2RustUnnamed_3 = 8;
pub const _ISpunct: C2RustUnnamed_3 = 4;
pub const _IScntrl: C2RustUnnamed_3 = 2;
pub const _ISblank: C2RustUnnamed_3 = 1;
pub const _ISgraph: C2RustUnnamed_3 = 32768;
pub const _ISprint: C2RustUnnamed_3 = 16384;
pub const _ISspace: C2RustUnnamed_3 = 8192;
pub const _ISxdigit: C2RustUnnamed_3 = 4096;
pub const _ISdigit: C2RustUnnamed_3 = 2048;
pub const _ISalpha: C2RustUnnamed_3 = 1024;
pub const _ISlower: C2RustUnnamed_3 = 512;
pub const _ISupper: C2RustUnnamed_3 = 256;
pub type C2RustUnnamed_4 = libc::c_int;
pub const GETOPT_VERSION_CHAR: C2RustUnnamed_4 = -3;
pub const GETOPT_HELP_CHAR: C2RustUnnamed_4 = -2;
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
pub type fadvice_t = libc::c_uint;
pub const FADVISE_RANDOM: fadvice_t = 1;
pub const FADVISE_WILLNEED: fadvice_t = 3;
pub const FADVISE_DONTNEED: fadvice_t = 4;
pub const FADVISE_NOREUSE: fadvice_t = 5;
pub const FADVISE_SEQUENTIAL: fadvice_t = 2;
pub const FADVISE_NORMAL: fadvice_t = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Tokens {
    pub n_tok: size_t,
    pub tok: *mut *mut libc::c_char,
    pub tok_len: *mut size_t,
    pub o_data: obstack,
    pub o_tok: obstack,
    pub o_tok_len: obstack,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct fstatus {
    pub failed: libc::c_int,
    pub st: stat,
}
pub type C2RustUnnamed_5 = libc::c_uint;
pub const TOTAL_OPTION: C2RustUnnamed_5 = 258;
pub const FILES0_FROM_OPTION: C2RustUnnamed_5 = 257;
pub const DEBUG_PROGRAM_OPTION: C2RustUnnamed_5 = 256;
pub type total_type = libc::c_uint;
pub const total_never: total_type = 3;
pub const total_only: total_type = 2;
pub const total_always: total_type = 1;
pub const total_auto: total_type = 0;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_8 {
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
unsafe extern "C" fn btowc(mut __c: libc::c_int) -> wint_t {
    return if 0 != 0 && __c >= '\0' as i32 && __c <= '\u{7f}' as i32 {
        __c as wint_t
    } else {
        __btowc_alias(__c)
    };
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
unsafe extern "C" fn to_uchar(mut ch: libc::c_char) -> libc::c_uchar {
    return ch as libc::c_uchar;
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
unsafe extern "C" fn is_basic(mut c: libc::c_char) -> bool {
    return *is_basic_table
        .as_ptr()
        .offset((c as libc::c_uchar as libc::c_int >> 5 as libc::c_int) as isize)
        >> (c as libc::c_uchar as libc::c_int & 31 as libc::c_int)
        & 1 as libc::c_int as libc::c_uint != 0;
}
#[inline]
unsafe extern "C" fn xset_binary_mode_error() {}
#[inline]
unsafe extern "C" fn xset_binary_mode(mut fd: libc::c_int, mut mode: libc::c_int) {
    if set_binary_mode(fd, mode) < 0 as libc::c_int {
        xset_binary_mode_error();
    }
}
#[inline]
unsafe extern "C" fn __gl_setmode(
    mut fd: libc::c_int,
    mut mode: libc::c_int,
) -> libc::c_int {
    return 0 as libc::c_int;
}
#[inline]
unsafe extern "C" fn set_binary_mode(
    mut fd: libc::c_int,
    mut mode: libc::c_int,
) -> libc::c_int {
    return __gl_setmode(fd, mode);
}
static mut wc_lines_p: Option::<
    unsafe extern "C" fn(
        *const libc::c_char,
        libc::c_int,
        *mut uintmax_t,
        *mut uintmax_t,
    ) -> bool,
> = unsafe {
    Some(
        wc_lines
            as unsafe extern "C" fn(
                *const libc::c_char,
                libc::c_int,
                *mut uintmax_t,
                *mut uintmax_t,
            ) -> bool,
    )
};
static mut debug: bool = false;
static mut total_lines: uintmax_t = 0;
static mut total_words: uintmax_t = 0;
static mut total_chars: uintmax_t = 0;
static mut total_bytes: uintmax_t = 0;
static mut max_line_length: uintmax_t = 0;
static mut print_lines: bool = false;
static mut print_words: bool = false;
static mut print_chars: bool = false;
static mut print_bytes: bool = false;
static mut print_linelength: bool = false;
static mut number_width: libc::c_int = 0;
static mut have_read_stdin: bool = false;
static mut page_size: size_t = 0;
static mut posixly_correct: bool = false;
static mut longopts: [option; 11] = [
    {
        let mut init = option {
            name: b"bytes\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'c' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"chars\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'm' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"lines\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'l' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"words\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'w' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"debug\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: DEBUG_PROGRAM_OPTION as libc::c_int,
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
            name: b"max-line-length\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'L' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"total\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: TOTAL_OPTION as libc::c_int,
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
static mut total_args: [*const libc::c_char; 5] = [
    b"auto\0" as *const u8 as *const libc::c_char,
    b"always\0" as *const u8 as *const libc::c_char,
    b"only\0" as *const u8 as *const libc::c_char,
    b"never\0" as *const u8 as *const libc::c_char,
    0 as *const libc::c_char,
];
static mut total_types: [total_type; 4] = [
    total_auto,
    total_always,
    total_only,
    total_never,
];
static mut total_mode: total_type = total_auto;
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
                b"Print newline, word, and byte counts for each FILE, and a total line if\nmore than one FILE is specified.  A word is a non-zero-length sequence of\nprintable characters delimited by white space.\n\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            stdout,
        );
        emit_stdin_note();
        fputs_unlocked(
            dcgettext(
                0 as *const libc::c_char,
                b"\nThe options below may be used to select which counts are printed, always in\nthe following order: newline, word, character, byte, maximum line length.\n  -c, --bytes            print the byte counts\n  -m, --chars            print the character counts\n  -l, --lines            print the newline counts\n\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            stdout,
        );
        fputs_unlocked(
            dcgettext(
                0 as *const libc::c_char,
                b"      --files0-from=F    read input from the files specified by\n                           NUL-terminated names in file F;\n                           If F is - then read names from standard input\n  -L, --max-line-length  print the maximum display width\n  -w, --words            print the word counts\n\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            stdout,
        );
        fputs_unlocked(
            dcgettext(
                0 as *const libc::c_char,
                b"      --total=WHEN       when to print a line with total counts;\n                           WHEN can be: auto, always, only, never\n\0"
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
        emit_ancillary_info(b"wc\0" as *const u8 as *const libc::c_char);
    }
    exit(status);
}
unsafe extern "C" fn iswnbspace(mut wc_0: wint_t) -> libc::c_int {
    return (!posixly_correct
        && (wc_0 == 0xa0 as libc::c_int as libc::c_uint
            || wc_0 == 0x2007 as libc::c_int as libc::c_uint
            || wc_0 == 0x202f as libc::c_int as libc::c_uint
            || wc_0 == 0x2060 as libc::c_int as libc::c_uint)) as libc::c_int;
}
unsafe extern "C" fn isnbspace(mut c: libc::c_int) -> libc::c_int {
    return iswnbspace(btowc(c));
}
unsafe extern "C" fn write_counts(
    mut lines: uintmax_t,
    mut words: uintmax_t,
    mut chars: uintmax_t,
    mut bytes: uintmax_t,
    mut linelength: uintmax_t,
    mut file: *const libc::c_char,
) {
    static mut format_sp_int: [libc::c_char; 5] = unsafe {
        *::core::mem::transmute::<&[u8; 5], &[libc::c_char; 5]>(b" %*s\0")
    };
    let mut format_int: *const libc::c_char = format_sp_int
        .as_ptr()
        .offset(1 as libc::c_int as isize);
    let mut buf: [libc::c_char; 21] = [0; 21];
    if print_lines {
        printf(format_int, number_width, umaxtostr(lines, buf.as_mut_ptr()));
        format_int = format_sp_int.as_ptr();
    }
    if print_words {
        printf(format_int, number_width, umaxtostr(words, buf.as_mut_ptr()));
        format_int = format_sp_int.as_ptr();
    }
    if print_chars {
        printf(format_int, number_width, umaxtostr(chars, buf.as_mut_ptr()));
        format_int = format_sp_int.as_ptr();
    }
    if print_bytes {
        printf(format_int, number_width, umaxtostr(bytes, buf.as_mut_ptr()));
        format_int = format_sp_int.as_ptr();
    }
    if print_linelength {
        printf(format_int, number_width, umaxtostr(linelength, buf.as_mut_ptr()));
    }
    if !file.is_null() {
        printf(
            b" %s\0" as *const u8 as *const libc::c_char,
            if !(strchr(file, '\n' as i32)).is_null() {
                quotearg_n_style_colon(
                    0 as libc::c_int,
                    shell_escape_quoting_style,
                    file,
                ) as *const libc::c_char
            } else {
                file
            },
        );
    }
    putchar_unlocked('\n' as i32);
}
unsafe extern "C" fn wc_lines(
    mut file: *const libc::c_char,
    mut fd: libc::c_int,
    mut lines_out: *mut uintmax_t,
    mut bytes_out: *mut uintmax_t,
) -> bool {
    let mut bytes_read: size_t = 0;
    let mut lines: uintmax_t = 0;
    let mut bytes: uintmax_t = 0;
    let mut buf: [libc::c_char; 16385] = [0; 16385];
    let mut long_lines: bool = 0 as libc::c_int != 0;
    if lines_out.is_null() || bytes_out.is_null() {
        return 0 as libc::c_int != 0;
    }
    bytes = 0 as libc::c_int as uintmax_t;
    lines = bytes;
    loop {
        bytes_read = safe_read(
            fd,
            buf.as_mut_ptr() as *mut libc::c_void,
            (16 as libc::c_int * 1024 as libc::c_int) as size_t,
        );
        if !(bytes_read > 0 as libc::c_int as libc::c_ulong) {
            break;
        }
        if bytes_read == -(1 as libc::c_int) as size_t {
            error(
                0 as libc::c_int,
                *__errno_location(),
                b"%s\0" as *const u8 as *const libc::c_char,
                quotearg_n_style_colon(
                    0 as libc::c_int,
                    shell_escape_quoting_style,
                    file,
                ),
            );
            return 0 as libc::c_int != 0;
        }
        bytes = (bytes as libc::c_ulong).wrapping_add(bytes_read) as uintmax_t
            as uintmax_t;
        let mut p: *mut libc::c_char = buf.as_mut_ptr();
        let mut end: *mut libc::c_char = buf.as_mut_ptr().offset(bytes_read as isize);
        let mut plines: uintmax_t = lines;
        if !long_lines {
            while p != end {
                let fresh1 = p;
                p = p.offset(1);
                lines = (lines as libc::c_ulong)
                    .wrapping_add(
                        (*fresh1 as libc::c_int == '\n' as i32) as libc::c_int
                            as libc::c_ulong,
                    ) as uintmax_t as uintmax_t;
            }
        } else {
            *end = '\n' as i32 as libc::c_char;
            loop {
                p = rawmemchr(p as *const libc::c_void, '\n' as i32)
                    as *mut libc::c_char;
                if !(p < end) {
                    break;
                }
                p = p.offset(1);
                lines = lines.wrapping_add(1);
            }
        }
        if lines.wrapping_sub(plines)
            <= bytes_read.wrapping_div(15 as libc::c_int as libc::c_ulong)
        {
            long_lines = 1 as libc::c_int != 0;
        } else {
            long_lines = 0 as libc::c_int != 0;
        }
    }
    *bytes_out = bytes;
    *lines_out = lines;
    return 1 as libc::c_int != 0;
}
unsafe extern "C" fn wc(
    mut fd: libc::c_int,
    mut file_x: *const libc::c_char,
    mut fstatus: *mut fstatus,
    mut current_pos: off_t,
) -> bool {
    let mut current_block: u64;
    let mut ok: bool = 1 as libc::c_int != 0;
    let mut buf: [libc::c_char; 16385] = [0; 16385];
    let mut bytes_read: size_t = 0;
    let mut lines: uintmax_t = 0;
    let mut words: uintmax_t = 0;
    let mut chars: uintmax_t = 0;
    let mut bytes: uintmax_t = 0;
    let mut linelength: uintmax_t = 0;
    let mut count_bytes: bool = false;
    let mut count_chars: bool = false;
    let mut count_complicated: bool = false;
    let mut file: *const libc::c_char = if !file_x.is_null() {
        file_x
    } else {
        dcgettext(
            0 as *const libc::c_char,
            b"standard input\0" as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        ) as *const libc::c_char
    };
    linelength = 0 as libc::c_int as uintmax_t;
    bytes = linelength;
    chars = bytes;
    words = chars;
    lines = words;
    if __ctype_get_mb_cur_max() > 1 as libc::c_int as libc::c_ulong {
        count_bytes = print_bytes;
        count_chars = print_chars;
    } else {
        count_bytes = print_bytes as libc::c_int != 0 || print_chars as libc::c_int != 0;
        count_chars = 0 as libc::c_int != 0;
    }
    count_complicated = print_words as libc::c_int != 0
        || print_linelength as libc::c_int != 0;
    if !count_bytes || count_chars as libc::c_int != 0 || print_lines as libc::c_int != 0
        || count_complicated as libc::c_int != 0
    {
        fdadvise(
            fd,
            0 as libc::c_int as off_t,
            0 as libc::c_int as off_t,
            FADVISE_SEQUENTIAL,
        );
    }
    if count_bytes as libc::c_int != 0 && !count_chars && !print_lines
        && !count_complicated
    {
        let mut skip_read: bool = 0 as libc::c_int != 0;
        if (0 as libc::c_int) < (*fstatus).failed {
            (*fstatus).failed = fstat(fd, &mut (*fstatus).st);
        }
        if (*fstatus).failed == 0
            && usable_st_size(&mut (*fstatus).st) as libc::c_int != 0
            && 0 as libc::c_int as libc::c_long <= (*fstatus).st.st_size
        {
            let mut end_pos: off_t = (*fstatus).st.st_size;
            if current_pos < 0 as libc::c_int as libc::c_long {
                current_pos = lseek(fd, 0 as libc::c_int as __off_t, 1 as libc::c_int);
            }
            if (end_pos as libc::c_ulong).wrapping_rem(page_size) != 0 {
                bytes = (if end_pos < current_pos {
                    0 as libc::c_int as libc::c_long
                } else {
                    end_pos - current_pos
                }) as uintmax_t;
                skip_read = 1 as libc::c_int != 0;
            } else {
                let mut hi_pos: off_t = end_pos
                    - end_pos
                        % ((if (0 as libc::c_int) < (*fstatus).st.st_blksize
                            && (*fstatus).st.st_blksize as libc::c_ulong
                                <= (-(1 as libc::c_int) as size_t)
                                    .wrapping_div(8 as libc::c_int as libc::c_ulong)
                                    .wrapping_add(1 as libc::c_int as libc::c_ulong)
                        {
                            (*fstatus).st.st_blksize
                        } else {
                            512 as libc::c_int
                        }) + 1 as libc::c_int) as libc::c_long;
                if 0 as libc::c_int as libc::c_long <= current_pos
                    && current_pos < hi_pos
                    && 0 as libc::c_int as libc::c_long
                        <= lseek(fd, hi_pos, 1 as libc::c_int)
                {
                    bytes = (hi_pos - current_pos) as uintmax_t;
                }
            }
        }
        if !skip_read {
            fdadvise(
                fd,
                0 as libc::c_int as off_t,
                0 as libc::c_int as off_t,
                FADVISE_SEQUENTIAL,
            );
            loop {
                bytes_read = safe_read(
                    fd,
                    buf.as_mut_ptr() as *mut libc::c_void,
                    (16 as libc::c_int * 1024 as libc::c_int) as size_t,
                );
                if !(bytes_read > 0 as libc::c_int as libc::c_ulong) {
                    break;
                }
                if bytes_read == -(1 as libc::c_int) as size_t {
                    error(
                        0 as libc::c_int,
                        *__errno_location(),
                        b"%s\0" as *const u8 as *const libc::c_char,
                        quotearg_n_style_colon(
                            0 as libc::c_int,
                            shell_escape_quoting_style,
                            file,
                        ),
                    );
                    ok = 0 as libc::c_int != 0;
                    break;
                } else {
                    bytes = (bytes as libc::c_ulong).wrapping_add(bytes_read)
                        as uintmax_t as uintmax_t;
                }
            }
        }
    } else if !count_chars && !count_complicated {
        ok = wc_lines_p
            .expect("non-null function pointer")(file, fd, &mut lines, &mut bytes);
    } else if __ctype_get_mb_cur_max() > 1 as libc::c_int as libc::c_ulong {
        let mut in_word: bool = 0 as libc::c_int != 0;
        let mut linepos: uintmax_t = 0 as libc::c_int as uintmax_t;
        let mut state: mbstate_t = {
            let mut init = __mbstate_t {
                __count: 0 as libc::c_int,
                __value: C2RustUnnamed { __wch: 0 },
            };
            init
        };
        let mut in_shift: bool = 0 as libc::c_int != 0;
        let mut prev: size_t = 0 as libc::c_int as size_t;
        loop {
            bytes_read = safe_read(
                fd,
                buf.as_mut_ptr().offset(prev as isize) as *mut libc::c_void,
                ((16 as libc::c_int * 1024 as libc::c_int) as libc::c_ulong)
                    .wrapping_sub(prev),
            );
            if !(bytes_read > 0 as libc::c_int as libc::c_ulong) {
                break;
            }
            let mut p: *const libc::c_char = 0 as *const libc::c_char;
            let mut backup_state: mbstate_t = mbstate_t {
                __count: 0,
                __value: C2RustUnnamed { __wch: 0 },
            };
            if bytes_read == -(1 as libc::c_int) as size_t {
                error(
                    0 as libc::c_int,
                    *__errno_location(),
                    b"%s\0" as *const u8 as *const libc::c_char,
                    quotearg_n_style_colon(
                        0 as libc::c_int,
                        shell_escape_quoting_style,
                        file,
                    ),
                );
                ok = 0 as libc::c_int != 0;
                break;
            } else {
                bytes = (bytes as libc::c_ulong).wrapping_add(bytes_read) as uintmax_t
                    as uintmax_t;
                p = buf.as_mut_ptr();
                bytes_read = (bytes_read as libc::c_ulong).wrapping_add(prev) as size_t
                    as size_t;
                loop {
                    let mut wide_char: wchar_t = 0;
                    let mut n: size_t = 0;
                    let mut wide: bool = 1 as libc::c_int != 0;
                    if !in_shift && is_basic(*p) as libc::c_int != 0 {
                        n = 1 as libc::c_int as size_t;
                        wide_char = *p as wchar_t;
                        wide = 0 as libc::c_int != 0;
                        current_block = 6721012065216013753;
                    } else {
                        in_shift = 1 as libc::c_int != 0;
                        backup_state = state;
                        n = rpl_mbrtowc(&mut wide_char, p, bytes_read, &mut state);
                        if n == -(2 as libc::c_int) as size_t {
                            state = backup_state;
                            break;
                        } else if n == -(1 as libc::c_int) as size_t {
                            p = p.offset(1);
                            bytes_read = bytes_read.wrapping_sub(1);
                            current_block = 4567019141635105728;
                        } else {
                            if mbsinit(&mut state) != 0 {
                                in_shift = 0 as libc::c_int != 0;
                            }
                            if n == 0 as libc::c_int as libc::c_ulong {
                                wide_char = 0 as libc::c_int as wchar_t;
                                n = 1 as libc::c_int as size_t;
                            }
                            current_block = 6721012065216013753;
                        }
                    }
                    match current_block {
                        6721012065216013753 => {
                            match wide_char {
                                10 => {
                                    lines = lines.wrapping_add(1);
                                    current_block = 10311054474372074981;
                                }
                                13 | 12 => {
                                    current_block = 10311054474372074981;
                                }
                                9 => {
                                    linepos = (linepos as libc::c_ulong)
                                        .wrapping_add(
                                            (8 as libc::c_int as libc::c_ulong)
                                                .wrapping_sub(
                                                    linepos.wrapping_rem(8 as libc::c_int as libc::c_ulong),
                                                ),
                                        ) as uintmax_t as uintmax_t;
                                    current_block = 17446828256691269598;
                                }
                                32 => {
                                    linepos = linepos.wrapping_add(1);
                                    current_block = 17446828256691269598;
                                }
                                11 => {
                                    current_block = 17446828256691269598;
                                }
                                _ => {
                                    if wide as libc::c_int != 0 && iswprint(wide_char) != 0 {
                                        if print_linelength {
                                            let mut width: libc::c_int = wcwidth(wide_char);
                                            if width > 0 as libc::c_int {
                                                linepos = (linepos as libc::c_ulong)
                                                    .wrapping_add(width as libc::c_ulong) as uintmax_t
                                                    as uintmax_t;
                                            }
                                        }
                                        if iswspace(wide_char) != 0 || iswnbspace(wide_char) != 0 {
                                            current_block = 17446828256691269598;
                                        } else {
                                            in_word = 1 as libc::c_int != 0;
                                            current_block = 576355610076403033;
                                        }
                                    } else if !wide
                                        && *(*__ctype_b_loc())
                                            .offset(to_uchar(*p) as libc::c_int as isize) as libc::c_int
                                            & _ISprint as libc::c_int as libc::c_ushort as libc::c_int
                                            != 0
                                    {
                                        linepos = linepos.wrapping_add(1);
                                        if *(*__ctype_b_loc())
                                            .offset(to_uchar(*p) as libc::c_int as isize) as libc::c_int
                                            & _ISspace as libc::c_int as libc::c_ushort as libc::c_int
                                            != 0
                                        {
                                            current_block = 17446828256691269598;
                                        } else {
                                            in_word = 1 as libc::c_int != 0;
                                            current_block = 576355610076403033;
                                        }
                                    } else {
                                        current_block = 576355610076403033;
                                    }
                                }
                            }
                            match current_block {
                                10311054474372074981 => {
                                    if linepos > linelength {
                                        linelength = linepos;
                                    }
                                    linepos = 0 as libc::c_int as uintmax_t;
                                    current_block = 17446828256691269598;
                                }
                                _ => {}
                            }
                            match current_block {
                                17446828256691269598 => {
                                    words = (words as libc::c_ulong)
                                        .wrapping_add(in_word as libc::c_ulong) as uintmax_t
                                        as uintmax_t;
                                    in_word = 0 as libc::c_int != 0;
                                }
                                _ => {}
                            }
                            p = p.offset(n as isize);
                            bytes_read = (bytes_read as libc::c_ulong).wrapping_sub(n)
                                as size_t as size_t;
                            chars = chars.wrapping_add(1);
                        }
                        _ => {}
                    }
                    if !(bytes_read > 0 as libc::c_int as libc::c_ulong) {
                        break;
                    }
                }
                if bytes_read > 0 as libc::c_int as libc::c_ulong {
                    if bytes_read
                        == (16 as libc::c_int * 1024 as libc::c_int) as libc::c_ulong
                    {
                        p = p.offset(1);
                        bytes_read = bytes_read.wrapping_sub(1);
                    }
                    memmove(
                        buf.as_mut_ptr() as *mut libc::c_void,
                        p as *const libc::c_void,
                        bytes_read,
                    );
                }
                prev = bytes_read;
            }
        }
        if linepos > linelength {
            linelength = linepos;
        }
        words = (words as libc::c_ulong).wrapping_add(in_word as libc::c_ulong)
            as uintmax_t as uintmax_t;
    } else {
        let mut in_word_0: bool = 0 as libc::c_int != 0;
        let mut linepos_0: uintmax_t = 0 as libc::c_int as uintmax_t;
        loop {
            bytes_read = safe_read(
                fd,
                buf.as_mut_ptr() as *mut libc::c_void,
                (16 as libc::c_int * 1024 as libc::c_int) as size_t,
            );
            if !(bytes_read > 0 as libc::c_int as libc::c_ulong) {
                break;
            }
            let mut p_0: *const libc::c_char = buf.as_mut_ptr();
            if bytes_read == -(1 as libc::c_int) as size_t {
                error(
                    0 as libc::c_int,
                    *__errno_location(),
                    b"%s\0" as *const u8 as *const libc::c_char,
                    quotearg_n_style_colon(
                        0 as libc::c_int,
                        shell_escape_quoting_style,
                        file,
                    ),
                );
                ok = 0 as libc::c_int != 0;
                break;
            } else {
                bytes = (bytes as libc::c_ulong).wrapping_add(bytes_read) as uintmax_t
                    as uintmax_t;
                loop {
                    let fresh2 = p_0;
                    p_0 = p_0.offset(1);
                    match *fresh2 as libc::c_int {
                        10 => {
                            lines = lines.wrapping_add(1);
                            current_block = 6511716454539559718;
                        }
                        13 | 12 => {
                            current_block = 6511716454539559718;
                        }
                        9 => {
                            linepos_0 = (linepos_0 as libc::c_ulong)
                                .wrapping_add(
                                    (8 as libc::c_int as libc::c_ulong)
                                        .wrapping_sub(
                                            linepos_0.wrapping_rem(8 as libc::c_int as libc::c_ulong),
                                        ),
                                ) as uintmax_t as uintmax_t;
                            current_block = 2600291065076048191;
                        }
                        32 => {
                            linepos_0 = linepos_0.wrapping_add(1);
                            current_block = 2600291065076048191;
                        }
                        11 => {
                            current_block = 2600291065076048191;
                        }
                        _ => {
                            if *(*__ctype_b_loc())
                                .offset(
                                    to_uchar(*p_0.offset(-(1 as libc::c_int) as isize))
                                        as libc::c_int as isize,
                                ) as libc::c_int
                                & _ISprint as libc::c_int as libc::c_ushort as libc::c_int
                                != 0
                            {
                                linepos_0 = linepos_0.wrapping_add(1);
                                if *(*__ctype_b_loc())
                                    .offset(
                                        to_uchar(*p_0.offset(-(1 as libc::c_int) as isize))
                                            as libc::c_int as isize,
                                    ) as libc::c_int
                                    & _ISspace as libc::c_int as libc::c_ushort as libc::c_int
                                    != 0
                                    || isnbspace(
                                        to_uchar(*p_0.offset(-(1 as libc::c_int) as isize))
                                            as libc::c_int,
                                    ) != 0
                                {
                                    current_block = 2600291065076048191;
                                } else {
                                    in_word_0 = 1 as libc::c_int != 0;
                                    current_block = 1934991416718554651;
                                }
                            } else {
                                current_block = 1934991416718554651;
                            }
                        }
                    }
                    match current_block {
                        6511716454539559718 => {
                            if linepos_0 > linelength {
                                linelength = linepos_0;
                            }
                            linepos_0 = 0 as libc::c_int as uintmax_t;
                            current_block = 2600291065076048191;
                        }
                        _ => {}
                    }
                    match current_block {
                        2600291065076048191 => {
                            words = (words as libc::c_ulong)
                                .wrapping_add(in_word_0 as libc::c_ulong) as uintmax_t
                                as uintmax_t;
                            in_word_0 = 0 as libc::c_int != 0;
                        }
                        _ => {}
                    }
                    bytes_read = bytes_read.wrapping_sub(1);
                    if !(bytes_read != 0) {
                        break;
                    }
                }
            }
        }
        if linepos_0 > linelength {
            linelength = linepos_0;
        }
        words = (words as libc::c_ulong).wrapping_add(in_word_0 as libc::c_ulong)
            as uintmax_t as uintmax_t;
    }
    if (count_chars as libc::c_int) < print_chars as libc::c_int {
        chars = bytes;
    }
    if total_mode as libc::c_uint != total_only as libc::c_int as libc::c_uint {
        write_counts(lines, words, chars, bytes, linelength, file_x);
    }
    total_lines = (total_lines as libc::c_ulong).wrapping_add(lines) as uintmax_t
        as uintmax_t;
    total_words = (total_words as libc::c_ulong).wrapping_add(words) as uintmax_t
        as uintmax_t;
    total_chars = (total_chars as libc::c_ulong).wrapping_add(chars) as uintmax_t
        as uintmax_t;
    total_bytes = (total_bytes as libc::c_ulong).wrapping_add(bytes) as uintmax_t
        as uintmax_t;
    if linelength > max_line_length {
        max_line_length = linelength;
    }
    return ok;
}
unsafe extern "C" fn wc_file(
    mut file: *const libc::c_char,
    mut fstatus: *mut fstatus,
) -> bool {
    if file.is_null()
        || strcmp(file, b"-\0" as *const u8 as *const libc::c_char) == 0 as libc::c_int
    {
        have_read_stdin = 1 as libc::c_int != 0;
        xset_binary_mode(0 as libc::c_int, 0 as libc::c_int);
        return wc(0 as libc::c_int, file, fstatus, -(1 as libc::c_int) as off_t);
    } else {
        let mut fd: libc::c_int = open(file, 0 as libc::c_int | 0 as libc::c_int);
        if fd == -(1 as libc::c_int) {
            error(
                0 as libc::c_int,
                *__errno_location(),
                b"%s\0" as *const u8 as *const libc::c_char,
                quotearg_n_style_colon(
                    0 as libc::c_int,
                    shell_escape_quoting_style,
                    file,
                ),
            );
            return 0 as libc::c_int != 0;
        } else {
            let mut ok: bool = wc(fd, file, fstatus, 0 as libc::c_int as off_t);
            if close(fd) != 0 as libc::c_int {
                error(
                    0 as libc::c_int,
                    *__errno_location(),
                    b"%s\0" as *const u8 as *const libc::c_char,
                    quotearg_n_style_colon(
                        0 as libc::c_int,
                        shell_escape_quoting_style,
                        file,
                    ),
                );
                return 0 as libc::c_int != 0;
            }
            return ok;
        }
    };
}
unsafe extern "C" fn get_input_fstatus(
    mut nfiles: size_t,
    mut file: *const *mut libc::c_char,
) -> *mut fstatus {
    let mut fstatus: *mut fstatus = xnmalloc(
        if nfiles != 0 { nfiles } else { 1 as libc::c_int as libc::c_ulong },
        ::core::mem::size_of::<fstatus>() as libc::c_ulong,
    ) as *mut fstatus;
    if nfiles == 0 as libc::c_int as libc::c_ulong
        || nfiles == 1 as libc::c_int as libc::c_ulong
            && print_lines as libc::c_int + print_words as libc::c_int
                + print_chars as libc::c_int + print_bytes as libc::c_int
                + print_linelength as libc::c_int == 1 as libc::c_int
    {
        (*fstatus.offset(0 as libc::c_int as isize)).failed = 1 as libc::c_int;
    } else {
        let mut i: size_t = 0 as libc::c_int as size_t;
        while i < nfiles {
            (*fstatus.offset(i as isize))
                .failed = if (*file.offset(i as isize)).is_null()
                || strcmp(
                    *file.offset(i as isize),
                    b"-\0" as *const u8 as *const libc::c_char,
                ) == 0 as libc::c_int
            {
                fstat(0 as libc::c_int, &mut (*fstatus.offset(i as isize)).st)
            } else {
                stat(*file.offset(i as isize), &mut (*fstatus.offset(i as isize)).st)
            };
            i = i.wrapping_add(1);
        }
    }
    return fstatus;
}
unsafe extern "C" fn compute_number_width(
    mut nfiles: size_t,
    mut fstatus: *const fstatus,
) -> libc::c_int {
    let mut width: libc::c_int = 1 as libc::c_int;
    if (0 as libc::c_int as libc::c_ulong) < nfiles
        && (*fstatus.offset(0 as libc::c_int as isize)).failed <= 0 as libc::c_int
    {
        let mut minimum_width: libc::c_int = 1 as libc::c_int;
        let mut regular_total: uintmax_t = 0 as libc::c_int as uintmax_t;
        let mut i: size_t = 0 as libc::c_int as size_t;
        while i < nfiles {
            if (*fstatus.offset(i as isize)).failed == 0 {
                if (*fstatus.offset(i as isize)).st.st_mode
                    & 0o170000 as libc::c_int as libc::c_uint
                    == 0o100000 as libc::c_int as libc::c_uint
                {
                    regular_total = (regular_total as libc::c_ulong)
                        .wrapping_add(
                            (*fstatus.offset(i as isize)).st.st_size as libc::c_ulong,
                        ) as uintmax_t as uintmax_t;
                } else {
                    minimum_width = 7 as libc::c_int;
                }
            }
            i = i.wrapping_add(1);
        }
        while 10 as libc::c_int as libc::c_ulong <= regular_total {
            width += 1;
            regular_total = (regular_total as libc::c_ulong)
                .wrapping_div(10 as libc::c_int as libc::c_ulong) as uintmax_t
                as uintmax_t;
        }
        if width < minimum_width {
            width = minimum_width;
        }
    }
    return width;
}
unsafe fn main_0(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut ok: bool = false;
    let mut optc: libc::c_int = 0;
    let mut nfiles: size_t = 0;
    let mut files: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    let mut files_from: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut fstatus: *mut fstatus = 0 as *mut fstatus;
    let mut tok: Tokens = Tokens {
        n_tok: 0,
        tok: 0 as *mut *mut libc::c_char,
        tok_len: 0 as *mut size_t,
        o_data: obstack {
            chunk_size: 0,
            chunk: 0 as *mut _obstack_chunk,
            object_base: 0 as *mut libc::c_char,
            next_free: 0 as *mut libc::c_char,
            chunk_limit: 0 as *mut libc::c_char,
            temp: C2RustUnnamed_2 { i: 0 },
            alignment_mask: 0,
            chunkfun: C2RustUnnamed_1 { plain: None },
            freefun: C2RustUnnamed_0 { plain: None },
            extra_arg: 0 as *mut libc::c_void,
            use_extra_arg_maybe_empty_object_alloc_failed: [0; 1],
            c2rust_padding: [0; 7],
        },
        o_tok: obstack {
            chunk_size: 0,
            chunk: 0 as *mut _obstack_chunk,
            object_base: 0 as *mut libc::c_char,
            next_free: 0 as *mut libc::c_char,
            chunk_limit: 0 as *mut libc::c_char,
            temp: C2RustUnnamed_2 { i: 0 },
            alignment_mask: 0,
            chunkfun: C2RustUnnamed_1 { plain: None },
            freefun: C2RustUnnamed_0 { plain: None },
            extra_arg: 0 as *mut libc::c_void,
            use_extra_arg_maybe_empty_object_alloc_failed: [0; 1],
            c2rust_padding: [0; 7],
        },
        o_tok_len: obstack {
            chunk_size: 0,
            chunk: 0 as *mut _obstack_chunk,
            object_base: 0 as *mut libc::c_char,
            next_free: 0 as *mut libc::c_char,
            chunk_limit: 0 as *mut libc::c_char,
            temp: C2RustUnnamed_2 { i: 0 },
            alignment_mask: 0,
            chunkfun: C2RustUnnamed_1 { plain: None },
            freefun: C2RustUnnamed_0 { plain: None },
            extra_arg: 0 as *mut libc::c_void,
            use_extra_arg_maybe_empty_object_alloc_failed: [0; 1],
            c2rust_padding: [0; 7],
        },
    };
    set_program_name(*argv.offset(0 as libc::c_int as isize));
    setlocale(6 as libc::c_int, b"\0" as *const u8 as *const libc::c_char);
    bindtextdomain(
        b"coreutils\0" as *const u8 as *const libc::c_char,
        b"/usr/local/share/locale\0" as *const u8 as *const libc::c_char,
    );
    textdomain(b"coreutils\0" as *const u8 as *const libc::c_char);
    atexit(Some(close_stdout as unsafe extern "C" fn() -> ()));
    page_size = getpagesize() as size_t;
    setvbuf(
        stdout,
        0 as *mut libc::c_char,
        1 as libc::c_int,
        0 as libc::c_int as size_t,
    );
    posixly_correct = !(getenv(b"POSIXLY_CORRECT\0" as *const u8 as *const libc::c_char))
        .is_null();
    print_bytes = 0 as libc::c_int != 0;
    print_chars = print_bytes;
    print_words = print_chars;
    print_lines = print_words;
    print_linelength = 0 as libc::c_int != 0;
    max_line_length = 0 as libc::c_int as uintmax_t;
    total_bytes = max_line_length;
    total_chars = total_bytes;
    total_words = total_chars;
    total_lines = total_words;
    loop {
        optc = getopt_long(
            argc,
            argv,
            b"clLmw\0" as *const u8 as *const libc::c_char,
            longopts.as_ptr(),
            0 as *mut libc::c_int,
        );
        if !(optc != -(1 as libc::c_int)) {
            break;
        }
        match optc {
            99 => {
                print_bytes = 1 as libc::c_int != 0;
            }
            109 => {
                print_chars = 1 as libc::c_int != 0;
            }
            108 => {
                print_lines = 1 as libc::c_int != 0;
            }
            119 => {
                print_words = 1 as libc::c_int != 0;
            }
            76 => {
                print_linelength = 1 as libc::c_int != 0;
            }
            256 => {
                debug = 1 as libc::c_int != 0;
            }
            257 => {
                files_from = optarg;
            }
            258 => {
                total_mode = total_types[__xargmatch_internal(
                    b"--total\0" as *const u8 as *const libc::c_char,
                    optarg,
                    total_args.as_ptr(),
                    total_types.as_ptr() as *const libc::c_void,
                    ::core::mem::size_of::<total_type>() as libc::c_ulong,
                    argmatch_die,
                    1 as libc::c_int != 0,
                ) as usize];
            }
            -2 => {
                usage(0 as libc::c_int);
            }
            -3 => {
                version_etc(
                    stdout,
                    b"wc\0" as *const u8 as *const libc::c_char,
                    b"GNU coreutils\0" as *const u8 as *const libc::c_char,
                    Version,
                    b"Paul Rubin\0" as *const u8 as *const libc::c_char,
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
    if !(print_lines as libc::c_int != 0 || print_words as libc::c_int != 0
        || print_chars as libc::c_int != 0 || print_bytes as libc::c_int != 0
        || print_linelength as libc::c_int != 0)
    {
        print_bytes = 1 as libc::c_int != 0;
        print_words = print_bytes;
        print_lines = print_words;
    }
    let mut read_tokens: bool = 0 as libc::c_int != 0;
    let mut ai: *mut argv_iterator = 0 as *mut argv_iterator;
    if !files_from.is_null() {
        let mut stream: *mut FILE = 0 as *mut FILE;
        if optind < argc {
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
                    *argv.offset(optind as isize),
                ),
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
        if strcmp(files_from, b"-\0" as *const u8 as *const libc::c_char)
            == 0 as libc::c_int
        {
            stream = stdin;
        } else {
            stream = fopen(files_from, b"r\0" as *const u8 as *const libc::c_char);
            if stream.is_null() {
                if ::core::mem::size_of::<C2RustUnnamed_8>() as libc::c_ulong != 0 {
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
        }
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
        if fstat(fileno(stream), &mut st) == 0 as libc::c_int
            && st.st_mode & 0o170000 as libc::c_int as libc::c_uint
                == 0o100000 as libc::c_int as libc::c_uint
            && st.st_size as libc::c_double
                <= (if ((10 as libc::c_int * 1024 as libc::c_int * 1024 as libc::c_int)
                    as libc::c_double)
                    < physmem_available() / 2 as libc::c_int as libc::c_double
                {
                    (10 as libc::c_int * 1024 as libc::c_int * 1024 as libc::c_int)
                        as libc::c_double
                } else {
                    physmem_available() / 2 as libc::c_int as libc::c_double
                })
        {
            read_tokens = 1 as libc::c_int != 0;
            readtokens0_init(&mut tok);
            if !readtokens0(stream, &mut tok) || rpl_fclose(stream) != 0 as libc::c_int {
                if ::core::mem::size_of::<C2RustUnnamed_7>() as libc::c_ulong != 0 {
                    error(
                        1 as libc::c_int,
                        0 as libc::c_int,
                        dcgettext(
                            0 as *const libc::c_char,
                            b"cannot read file names from %s\0" as *const u8
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
                        0 as libc::c_int,
                        dcgettext(
                            0 as *const libc::c_char,
                            b"cannot read file names from %s\0" as *const u8
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
            files = tok.tok;
            nfiles = tok.n_tok;
            ai = argv_iter_init_argv(files);
        } else {
            files = 0 as *mut *mut libc::c_char;
            nfiles = 0 as libc::c_int as size_t;
            ai = argv_iter_init_stream(stream);
        }
    } else {
        static mut stdin_only: [*mut libc::c_char; 1] = [
            0 as *const libc::c_char as *mut libc::c_char,
        ];
        files = if optind < argc {
            argv.offset(optind as isize)
        } else {
            stdin_only.as_mut_ptr()
        };
        nfiles = (if optind < argc { argc - optind } else { 1 as libc::c_int })
            as size_t;
        ai = argv_iter_init_argv(files);
    }
    if ai.is_null() {
        xalloc_die();
    }
    fstatus = get_input_fstatus(nfiles, files);
    if total_mode as libc::c_uint == total_only as libc::c_int as libc::c_uint {
        number_width = 1 as libc::c_int;
    } else {
        number_width = compute_number_width(nfiles, fstatus);
    }
    ok = 1 as libc::c_int != 0;
    let mut i: libc::c_int = 0 as libc::c_int;
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
                    b"src/wc.c\0" as *const u8 as *const libc::c_char,
                    971 as libc::c_int as libc::c_uint,
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
            ok = (ok as libc::c_int
                & wc_file(
                    file_name,
                    &mut *fstatus
                        .offset(
                            (if nfiles != 0 { i } else { 0 as libc::c_int }) as isize,
                        ),
                ) as libc::c_int) as bool;
        }
        if nfiles == 0 {
            (*fstatus.offset(0 as libc::c_int as isize)).failed = 1 as libc::c_int;
        }
        i += 1;
    }
    if ok as libc::c_int != 0 && files_from.is_null()
        && argv_iter_n_args(ai) == 0 as libc::c_int as libc::c_ulong
    {
        ok = (ok as libc::c_int
            & wc_file(
                0 as *const libc::c_char,
                &mut *fstatus.offset(0 as libc::c_int as isize),
            ) as libc::c_int) as bool;
    }
    if read_tokens {
        readtokens0_free(&mut tok);
    }
    if total_mode as libc::c_uint != total_never as libc::c_int as libc::c_uint
        && (total_mode as libc::c_uint != total_auto as libc::c_int as libc::c_uint
            || (1 as libc::c_int as libc::c_ulong) < argv_iter_n_args(ai))
    {
        write_counts(
            total_lines,
            total_words,
            total_chars,
            total_bytes,
            max_line_length,
            if total_mode as libc::c_uint != total_only as libc::c_int as libc::c_uint {
                dcgettext(
                    0 as *const libc::c_char,
                    b"total\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                )
            } else {
                0 as *mut libc::c_char
            },
        );
    }
    argv_iter_free(ai);
    free(fstatus as *mut libc::c_void);
    if have_read_stdin as libc::c_int != 0 && close(0 as libc::c_int) != 0 as libc::c_int
    {
        if ::core::mem::size_of::<C2RustUnnamed_6>() as libc::c_ulong != 0 {
            error(
                1 as libc::c_int,
                *__errno_location(),
                b"-\0" as *const u8 as *const libc::c_char,
            );
            if 0 as libc::c_int != 0 {} else {
                unreachable!();
            };
        } else {
            error(
                1 as libc::c_int,
                *__errno_location(),
                b"-\0" as *const u8 as *const libc::c_char,
            );
            if 0 as libc::c_int != 0 {} else {
                unreachable!();
            };
        };
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
