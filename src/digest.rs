#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![feature(extern_types)]
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    static mut optind: libc::c_int;
    fn getopt_long(
        ___argc: libc::c_int,
        ___argv: *const *mut libc::c_char,
        __shortopts: *const libc::c_char,
        __longopts: *const option,
        __longind: *mut libc::c_int,
    ) -> libc::c_int;
    fn atexit(__func: Option::<unsafe extern "C" fn() -> ()>) -> libc::c_int;
    fn exit(_: libc::c_int) -> !;
    fn isatty(__fd: libc::c_int) -> libc::c_int;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strncmp(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn free(_: *mut libc::c_void);
    fn __errno_location() -> *mut libc::c_int;
    static mut Version: *const libc::c_char;
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    fn __ctype_tolower_loc() -> *mut *const __int32_t;
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
    static mut stdin: *mut FILE;
    static mut stdout: *mut FILE;
    static mut stderr: *mut FILE;
    fn setvbuf(
        __stream: *mut FILE,
        __buf: *mut libc::c_char,
        __modes: libc::c_int,
        __n: size_t,
    ) -> libc::c_int;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn __overflow(_: *mut FILE, _: libc::c_int) -> libc::c_int;
    fn __getdelim(
        __lineptr: *mut *mut libc::c_char,
        __n: *mut size_t,
        __delimiter: libc::c_int,
        __stream: *mut FILE,
    ) -> __ssize_t;
    fn fputs_unlocked(__s: *const libc::c_char, __stream: *mut FILE) -> libc::c_int;
    fn clearerr_unlocked(__stream: *mut FILE);
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
    fn quotearg_n_style_colon(
        n: libc::c_int,
        s: quoting_style,
        arg: *const libc::c_char,
    ) -> *mut libc::c_char;
    fn bsd_sum_stream(
        stream: *mut FILE,
        resstream: *mut libc::c_void,
        length: *mut uintmax_t,
    ) -> libc::c_int;
    fn sysv_sum_stream(
        stream: *mut FILE,
        resstream: *mut libc::c_void,
        length: *mut uintmax_t,
    ) -> libc::c_int;
    fn output_bsd(
        file: *const libc::c_char,
        binary_file: libc::c_int,
        digest: *const libc::c_void,
        raw: bool,
        tagged: bool,
        delim: libc::c_uchar,
        args: bool,
        length: uintmax_t,
    );
    fn output_sysv(
        file: *const libc::c_char,
        binary_file: libc::c_int,
        digest: *const libc::c_void,
        raw: bool,
        tagged: bool,
        delim: libc::c_uchar,
        args: bool,
        length: uintmax_t,
    );
    fn error(
        __status: libc::c_int,
        __errnum: libc::c_int,
        __format: *const libc::c_char,
        _: ...
    );
    fn fadvise(fp: *mut FILE, advice: fadvice_t);
    fn fopen_safer(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct option {
    pub name: *const libc::c_char,
    pub has_arg: libc::c_int,
    pub flag: *mut libc::c_int,
    pub val: libc::c_int,
}
pub type __int32_t = libc::c_int;
pub type __uintmax_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __ssize_t = libc::c_long;
pub type ssize_t = __ssize_t;
pub type size_t = libc::c_ulong;
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
pub const PLURAL_REDUCER: C2RustUnnamed_0 = 1000000;
pub type C2RustUnnamed_0 = libc::c_uint;
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
pub type C2RustUnnamed_1 = libc::c_int;
pub const GETOPT_VERSION_CHAR: C2RustUnnamed_1 = -3;
pub const GETOPT_HELP_CHAR: C2RustUnnamed_1 = -2;
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
pub type sumfn = Option::<
    unsafe extern "C" fn(*mut FILE, *mut libc::c_void, *mut uintmax_t) -> libc::c_int,
>;
pub type fadvice_t = libc::c_uint;
pub const FADVISE_RANDOM: fadvice_t = 1;
pub const FADVISE_WILLNEED: fadvice_t = 3;
pub const FADVISE_DONTNEED: fadvice_t = 4;
pub const FADVISE_NOREUSE: fadvice_t = 5;
pub const FADVISE_SEQUENTIAL: fadvice_t = 2;
pub const FADVISE_NORMAL: fadvice_t = 0;
pub type digest_output_fn = Option::<
    unsafe extern "C" fn(
        *const libc::c_char,
        libc::c_int,
        *const libc::c_void,
        bool,
        bool,
        libc::c_uchar,
        bool,
        uintmax_t,
    ) -> (),
>;
pub type Algorithm = libc::c_uint;
pub const sysv: Algorithm = 1;
pub const bsd: Algorithm = 0;
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
#[inline]
unsafe extern "C" fn tolower(mut __c: libc::c_int) -> libc::c_int {
    return if __c >= -(128 as libc::c_int) && __c < 256 as libc::c_int {
        *(*__ctype_tolower_loc()).offset(__c as isize)
    } else {
        __c
    };
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
unsafe extern "C" fn getline(
    mut __lineptr: *mut *mut libc::c_char,
    mut __n: *mut size_t,
    mut __stream: *mut FILE,
) -> __ssize_t {
    return __getdelim(__lineptr, __n, '\n' as i32, __stream);
}
#[inline]
unsafe extern "C" fn feof_unlocked(mut __stream: *mut FILE) -> libc::c_int {
    return ((*__stream)._flags & 0x10 as libc::c_int != 0 as libc::c_int) as libc::c_int;
}
#[inline]
unsafe extern "C" fn ferror_unlocked(mut __stream: *mut FILE) -> libc::c_int {
    return ((*__stream)._flags & 0x20 as libc::c_int != 0 as libc::c_int) as libc::c_int;
}
#[inline]
unsafe extern "C" fn ptr_align(
    mut ptr: *const libc::c_void,
    mut alignment: size_t,
) -> *mut libc::c_void {
    let mut p0: *const libc::c_char = ptr as *const libc::c_char;
    let mut p1: *const libc::c_char = p0
        .offset(alignment as isize)
        .offset(-(1 as libc::c_int as isize));
    return p1.offset(-((p1 as size_t).wrapping_rem(alignment) as isize))
        as *mut libc::c_void;
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
unsafe extern "C" fn bad_cast(mut s: *const libc::c_char) -> *mut libc::c_char {
    return s as *mut libc::c_char;
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
#[inline]
unsafe extern "C" fn xset_binary_mode_error() {}
#[inline]
unsafe extern "C" fn xset_binary_mode(mut fd: libc::c_int, mut mode: libc::c_int) {
    if set_binary_mode(fd, mode) < 0 as libc::c_int {
        xset_binary_mode_error();
    }
}
static mut have_read_stdin: bool = false;
static mut min_digest_line_length: size_t = 0;
static mut digest_hex_bytes: size_t = 0;
static mut status_only: bool = 0 as libc::c_int != 0;
static mut warn: bool = 0 as libc::c_int != 0;
static mut ignore_missing: bool = 0 as libc::c_int != 0;
static mut quiet: bool = 0 as libc::c_int != 0;
static mut strict: bool = 0 as libc::c_int != 0;
static mut bsd_reversed: libc::c_int = -(1 as libc::c_int);
static mut digest_delim: libc::c_uchar = '\n' as i32 as libc::c_uchar;
static mut raw_digest: bool = 0 as libc::c_int != 0;
static mut sum_algorithm: Algorithm = bsd;
static mut sumfns: [sumfn; 2] = unsafe {
    [
        Some(
            bsd_sum_stream
                as unsafe extern "C" fn(
                    *mut FILE,
                    *mut libc::c_void,
                    *mut uintmax_t,
                ) -> libc::c_int,
        ),
        Some(
            sysv_sum_stream
                as unsafe extern "C" fn(
                    *mut FILE,
                    *mut libc::c_void,
                    *mut uintmax_t,
                ) -> libc::c_int,
        ),
    ]
};
static mut sum_output_fns: [digest_output_fn; 2] = unsafe {
    [
        Some(
            output_bsd
                as unsafe extern "C" fn(
                    *const libc::c_char,
                    libc::c_int,
                    *const libc::c_void,
                    bool,
                    bool,
                    libc::c_uchar,
                    bool,
                    uintmax_t,
                ) -> (),
        ),
        Some(
            output_sysv
                as unsafe extern "C" fn(
                    *const libc::c_char,
                    libc::c_int,
                    *const libc::c_void,
                    bool,
                    bool,
                    libc::c_uchar,
                    bool,
                    uintmax_t,
                ) -> (),
        ),
    ]
};
static mut long_options: [option; 4] = [
    {
        let mut init = option {
            name: b"sysv\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 's' as i32,
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
                b"Usage: %s [OPTION]... [FILE]...\n\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
            program_name,
        );
        printf(
            dcgettext(
                0 as *const libc::c_char,
                b"Print or check %s (%d-bit) checksums.\n\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
            b"BSD\0" as *const u8 as *const libc::c_char,
            16 as libc::c_int,
        );
        emit_stdin_note();
        fputs_unlocked(
            dcgettext(
                0 as *const libc::c_char,
                b"\n  -r              use BSD sum algorithm (the default), use 1K blocks\n  -s, --sysv      use System V sum algorithm, use 512 bytes blocks\n\0"
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
        emit_ancillary_info(b"sum\0" as *const u8 as *const libc::c_char);
    }
    exit(status);
}
unsafe extern "C" fn filename_unescape(
    mut s: *mut libc::c_char,
    mut s_len: size_t,
) -> *mut libc::c_char {
    let mut dst: *mut libc::c_char = s;
    let mut i: size_t = 0 as libc::c_int as size_t;
    while i < s_len {
        match *s.offset(i as isize) as libc::c_int {
            92 => {
                if i == s_len.wrapping_sub(1 as libc::c_int as libc::c_ulong) {
                    return 0 as *mut libc::c_char;
                }
                i = i.wrapping_add(1);
                match *s.offset(i as isize) as libc::c_int {
                    110 => {
                        let fresh1 = dst;
                        dst = dst.offset(1);
                        *fresh1 = '\n' as i32 as libc::c_char;
                    }
                    114 => {
                        let fresh2 = dst;
                        dst = dst.offset(1);
                        *fresh2 = '\r' as i32 as libc::c_char;
                    }
                    92 => {
                        let fresh3 = dst;
                        dst = dst.offset(1);
                        *fresh3 = '\\' as i32 as libc::c_char;
                    }
                    _ => return 0 as *mut libc::c_char,
                }
            }
            0 => return 0 as *mut libc::c_char,
            _ => {
                let fresh4 = dst;
                dst = dst.offset(1);
                *fresh4 = *s.offset(i as isize);
            }
        }
        i = i.wrapping_add(1);
    }
    if dst < s.offset(s_len as isize) {
        *dst = '\0' as i32 as libc::c_char;
    }
    return s;
}
unsafe extern "C" fn valid_digits(mut s: *const libc::c_uchar, mut len: size_t) -> bool {
    if len == digest_hex_bytes {
        let mut i: libc::c_uint = 0 as libc::c_int as libc::c_uint;
        while (i as libc::c_ulong) < digest_hex_bytes {
            if *(*__ctype_b_loc()).offset(*s as libc::c_int as isize) as libc::c_int
                & _ISxdigit as libc::c_int as libc::c_ushort as libc::c_int == 0
            {
                return 0 as libc::c_int != 0;
            }
            s = s.offset(1);
            i = i.wrapping_add(1);
        }
    } else {
        return 0 as libc::c_int != 0
    }
    return *s as libc::c_int == '\0' as i32;
}
unsafe extern "C" fn bsd_split_3(
    mut s: *mut libc::c_char,
    mut s_len: size_t,
    mut digest: *mut *mut libc::c_uchar,
    mut d_len: *mut size_t,
    mut file_name: *mut *mut libc::c_char,
    mut escaped_filename: bool,
) -> bool {
    if s_len == 0 as libc::c_int as libc::c_ulong {
        return 0 as libc::c_int != 0;
    }
    let mut i: size_t = s_len.wrapping_sub(1 as libc::c_int as libc::c_ulong);
    while i != 0 && *s.offset(i as isize) as libc::c_int != ')' as i32 {
        i = i.wrapping_sub(1);
    }
    if *s.offset(i as isize) as libc::c_int != ')' as i32 {
        return 0 as libc::c_int != 0;
    }
    *file_name = s;
    if escaped_filename as libc::c_int != 0 && (filename_unescape(s, i)).is_null() {
        return 0 as libc::c_int != 0;
    }
    let fresh5 = i;
    i = i.wrapping_add(1);
    *s.offset(fresh5 as isize) = '\0' as i32 as libc::c_char;
    while *s.offset(i as isize) as libc::c_int == ' ' as i32
        || *s.offset(i as isize) as libc::c_int == '\t' as i32
    {
        i = i.wrapping_add(1);
    }
    if *s.offset(i as isize) as libc::c_int != '=' as i32 {
        return 0 as libc::c_int != 0;
    }
    i = i.wrapping_add(1);
    while *s.offset(i as isize) as libc::c_int == ' ' as i32
        || *s.offset(i as isize) as libc::c_int == '\t' as i32
    {
        i = i.wrapping_add(1);
    }
    *digest = &mut *s.offset(i as isize) as *mut libc::c_char as *mut libc::c_uchar;
    *d_len = s_len.wrapping_sub(i);
    return valid_digits(*digest, *d_len);
}
unsafe extern "C" fn split_3(
    mut s: *mut libc::c_char,
    mut s_len: size_t,
    mut digest: *mut *mut libc::c_uchar,
    mut d_len: *mut size_t,
    mut binary: *mut libc::c_int,
    mut file_name: *mut *mut libc::c_char,
) -> bool {
    let mut escaped_filename: bool = 0 as libc::c_int != 0;
    let mut algo_name_len: size_t = 0;
    let mut i: size_t = 0 as libc::c_int as size_t;
    while *s.offset(i as isize) as libc::c_int == ' ' as i32
        || *s.offset(i as isize) as libc::c_int == '\t' as i32
    {
        i = i.wrapping_add(1);
    }
    if *s.offset(i as isize) as libc::c_int == '\\' as i32 {
        i = i.wrapping_add(1);
        escaped_filename = 1 as libc::c_int != 0;
    }
    algo_name_len = strlen(b"BSD\0" as *const u8 as *const libc::c_char);
    if strncmp(
        s.offset(i as isize),
        b"BSD\0" as *const u8 as *const libc::c_char,
        algo_name_len,
    ) == 0 as libc::c_int
    {
        i = (i as libc::c_ulong).wrapping_add(algo_name_len) as size_t as size_t;
        if *s.offset(i as isize) as libc::c_int == ' ' as i32 {
            i = i.wrapping_add(1);
        }
        if *s.offset(i as isize) as libc::c_int == '(' as i32 {
            i = i.wrapping_add(1);
            *binary = 0 as libc::c_int;
            return bsd_split_3(
                s.offset(i as isize),
                s_len.wrapping_sub(i),
                digest,
                d_len,
                file_name,
                escaped_filename,
            );
        }
        return 0 as libc::c_int != 0;
    }
    if s_len.wrapping_sub(i)
        < min_digest_line_length
            .wrapping_add(
                (*s.offset(i as isize) as libc::c_int == '\\' as i32) as libc::c_int
                    as libc::c_ulong,
            )
    {
        return 0 as libc::c_int != 0;
    }
    *digest = &mut *s.offset(i as isize) as *mut libc::c_char as *mut libc::c_uchar;
    while *s.offset(i as isize) as libc::c_int != 0
        && !(*s.offset(i as isize) as libc::c_int == ' ' as i32
            || *s.offset(i as isize) as libc::c_int == '\t' as i32)
    {
        i = i.wrapping_add(1);
    }
    *d_len = (&mut *s.offset(i as isize) as *mut libc::c_char)
        .offset_from(*digest as *mut libc::c_char) as libc::c_long as size_t;
    let fresh6 = i;
    i = i.wrapping_add(1);
    *s.offset(fresh6 as isize) = '\0' as i32 as libc::c_char;
    if !valid_digits(*digest, *d_len) {
        return 0 as libc::c_int != 0;
    }
    if s_len.wrapping_sub(i) == 1 as libc::c_int as libc::c_ulong
        || *s.offset(i as isize) as libc::c_int != ' ' as i32
            && *s.offset(i as isize) as libc::c_int != '*' as i32
    {
        if bsd_reversed == 0 as libc::c_int {
            return 0 as libc::c_int != 0;
        }
        bsd_reversed = 1 as libc::c_int;
    } else if bsd_reversed != 1 as libc::c_int {
        bsd_reversed = 0 as libc::c_int;
        let fresh7 = i;
        i = i.wrapping_add(1);
        *binary = (*s.offset(fresh7 as isize) as libc::c_int == '*' as i32)
            as libc::c_int;
    }
    *file_name = &mut *s.offset(i as isize) as *mut libc::c_char;
    if escaped_filename {
        return !(filename_unescape(&mut *s.offset(i as isize), s_len.wrapping_sub(i)))
            .is_null();
    }
    return 1 as libc::c_int != 0;
}
unsafe extern "C" fn print_filename(mut file: *const libc::c_char, mut escape: bool) {
    if !escape {
        fputs_unlocked(file, stdout);
        return;
    }
    while *file != 0 {
        match *file as libc::c_int {
            10 => {
                fputs_unlocked(b"\\n\0" as *const u8 as *const libc::c_char, stdout);
            }
            13 => {
                fputs_unlocked(b"\\r\0" as *const u8 as *const libc::c_char, stdout);
            }
            92 => {
                fputs_unlocked(b"\\\\\0" as *const u8 as *const libc::c_char, stdout);
            }
            _ => {
                putchar_unlocked(*file as libc::c_int);
            }
        }
        file = file.offset(1);
    }
}
unsafe extern "C" fn digest_file(
    mut filename: *const libc::c_char,
    mut binary: *mut libc::c_int,
    mut bin_result: *mut libc::c_uchar,
    mut missing: *mut bool,
    mut length: *mut uintmax_t,
) -> bool {
    let mut fp: *mut FILE = 0 as *mut FILE;
    let mut err: libc::c_int = 0;
    let mut is_stdin: bool = strcmp(filename, b"-\0" as *const u8 as *const libc::c_char)
        == 0 as libc::c_int;
    *missing = 0 as libc::c_int != 0;
    if is_stdin {
        have_read_stdin = 1 as libc::c_int != 0;
        fp = stdin;
        if 0 as libc::c_int != 0 && *binary != 0 {
            if *binary < 0 as libc::c_int {
                *binary = (isatty(0 as libc::c_int) == 0) as libc::c_int;
            }
            if *binary != 0 {
                xset_binary_mode(0 as libc::c_int, 0 as libc::c_int);
            }
        }
    } else {
        fp = fopen_safer(
            filename,
            if 0 as libc::c_int != 0 && *binary != 0 {
                b"rb\0" as *const u8 as *const libc::c_char
            } else {
                b"r\0" as *const u8 as *const libc::c_char
            },
        );
        if fp.is_null() {
            if ignore_missing as libc::c_int != 0
                && *__errno_location() == 2 as libc::c_int
            {
                *missing = 1 as libc::c_int != 0;
                return 1 as libc::c_int != 0;
            }
            error(
                0 as libc::c_int,
                *__errno_location(),
                b"%s\0" as *const u8 as *const libc::c_char,
                quotearg_n_style_colon(
                    0 as libc::c_int,
                    shell_escape_quoting_style,
                    filename,
                ),
            );
            return 0 as libc::c_int != 0;
        }
    }
    fadvise(fp, FADVISE_SEQUENTIAL);
    err = (sumfns[sum_algorithm as usize])
        .expect(
            "non-null function pointer",
        )(fp, bin_result as *mut libc::c_void, length);
    err = if err != 0 { *__errno_location() } else { 0 as libc::c_int };
    if is_stdin {
        clearerr_unlocked(fp);
    } else if rpl_fclose(fp) != 0 as libc::c_int && err == 0 {
        err = *__errno_location();
    }
    if err != 0 {
        error(
            0 as libc::c_int,
            err,
            b"%s\0" as *const u8 as *const libc::c_char,
            quotearg_n_style_colon(
                0 as libc::c_int,
                shell_escape_quoting_style,
                filename,
            ),
        );
        return 0 as libc::c_int != 0;
    }
    return 1 as libc::c_int != 0;
}
unsafe extern "C" fn hex_equal(
    mut hex_digest: *const libc::c_uchar,
    mut bin_buffer: *const libc::c_uchar,
) -> bool {
    static mut bin2hex: [libc::c_char; 16] = [
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
        'a' as i32 as libc::c_char,
        'b' as i32 as libc::c_char,
        'c' as i32 as libc::c_char,
        'd' as i32 as libc::c_char,
        'e' as i32 as libc::c_char,
        'f' as i32 as libc::c_char,
    ];
    let mut digest_bin_bytes: size_t = digest_hex_bytes
        .wrapping_div(2 as libc::c_int as libc::c_ulong);
    let mut cnt: size_t = 0;
    cnt = 0 as libc::c_int as size_t;
    while cnt < digest_bin_bytes {
        if ({
            let mut __res: libc::c_int = 0;
            if ::core::mem::size_of::<libc::c_uchar>() as libc::c_ulong
                > 1 as libc::c_int as libc::c_ulong
            {
                if 0 != 0 {
                    let mut __c: libc::c_int = *hex_digest
                        .offset(
                            (2 as libc::c_int as libc::c_ulong).wrapping_mul(cnt)
                                as isize,
                        ) as libc::c_int;
                    __res = (if __c < -(128 as libc::c_int) || __c > 255 as libc::c_int {
                        __c
                    } else {
                        *(*__ctype_tolower_loc()).offset(__c as isize)
                    });
                } else {
                    __res = tolower(
                        *hex_digest
                            .offset(
                                (2 as libc::c_int as libc::c_ulong).wrapping_mul(cnt)
                                    as isize,
                            ) as libc::c_int,
                    );
                }
            } else {
                __res = *(*__ctype_tolower_loc())
                    .offset(
                        *hex_digest
                            .offset(
                                (2 as libc::c_int as libc::c_ulong).wrapping_mul(cnt)
                                    as isize,
                            ) as libc::c_int as isize,
                    );
            }
            __res
        })
            != bin2hex[(*bin_buffer.offset(cnt as isize) as libc::c_int
                >> 4 as libc::c_int) as usize] as libc::c_int
            || ({
                let mut __res: libc::c_int = 0;
                if ::core::mem::size_of::<libc::c_uchar>() as libc::c_ulong
                    > 1 as libc::c_int as libc::c_ulong
                {
                    if 0 != 0 {
                        let mut __c: libc::c_int = *hex_digest
                            .offset(
                                (2 as libc::c_int as libc::c_ulong)
                                    .wrapping_mul(cnt)
                                    .wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                            ) as libc::c_int;
                        __res = (if __c < -(128 as libc::c_int)
                            || __c > 255 as libc::c_int
                        {
                            __c
                        } else {
                            *(*__ctype_tolower_loc()).offset(__c as isize)
                        });
                    } else {
                        __res = tolower(
                            *hex_digest
                                .offset(
                                    (2 as libc::c_int as libc::c_ulong)
                                        .wrapping_mul(cnt)
                                        .wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                                ) as libc::c_int,
                        );
                    }
                } else {
                    __res = *(*__ctype_tolower_loc())
                        .offset(
                            *hex_digest
                                .offset(
                                    (2 as libc::c_int as libc::c_ulong)
                                        .wrapping_mul(cnt)
                                        .wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                                ) as libc::c_int as isize,
                        );
                }
                __res
            })
                != bin2hex[(*bin_buffer.offset(cnt as isize) as libc::c_int
                    & 0xf as libc::c_int) as usize] as libc::c_int
        {
            break;
        }
        cnt = cnt.wrapping_add(1);
    }
    return cnt == digest_bin_bytes;
}
unsafe extern "C" fn digest_check(mut checkfile_name: *const libc::c_char) -> bool {
    let mut checkfile_stream: *mut FILE = 0 as *mut FILE;
    let mut n_misformatted_lines: uintmax_t = 0 as libc::c_int as uintmax_t;
    let mut n_mismatched_checksums: uintmax_t = 0 as libc::c_int as uintmax_t;
    let mut n_open_or_read_failures: uintmax_t = 0 as libc::c_int as uintmax_t;
    let mut properly_formatted_lines: bool = 0 as libc::c_int != 0;
    let mut matched_checksums: bool = 0 as libc::c_int != 0;
    let mut bin_buffer_unaligned: [libc::c_uchar; 6] = [0; 6];
    let mut bin_buffer: *mut libc::c_uchar = ptr_align(
        bin_buffer_unaligned.as_mut_ptr() as *const libc::c_void,
        4 as libc::c_int as size_t,
    ) as *mut libc::c_uchar;
    let mut line_number: uintmax_t = 0;
    let mut line: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut line_chars_allocated: size_t = 0;
    let mut is_stdin: bool = strcmp(
        checkfile_name,
        b"-\0" as *const u8 as *const libc::c_char,
    ) == 0 as libc::c_int;
    if is_stdin {
        have_read_stdin = 1 as libc::c_int != 0;
        checkfile_name = dcgettext(
            0 as *const libc::c_char,
            b"standard input\0" as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        );
        checkfile_stream = stdin;
    } else {
        checkfile_stream = fopen_safer(
            checkfile_name,
            b"r\0" as *const u8 as *const libc::c_char,
        );
        if checkfile_stream.is_null() {
            error(
                0 as libc::c_int,
                *__errno_location(),
                b"%s\0" as *const u8 as *const libc::c_char,
                quotearg_n_style_colon(
                    0 as libc::c_int,
                    shell_escape_quoting_style,
                    checkfile_name,
                ),
            );
            return 0 as libc::c_int != 0;
        }
    }
    line_number = 0 as libc::c_int as uintmax_t;
    line = 0 as *mut libc::c_char;
    line_chars_allocated = 0 as libc::c_int as size_t;
    loop {
        let mut filename: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut binary: libc::c_int = 0;
        let mut digest: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
        let mut line_length: ssize_t = 0;
        line_number = line_number.wrapping_add(1);
        if line_number == 0 as libc::c_int as libc::c_ulong {
            if ::core::mem::size_of::<C2RustUnnamed_2>() as libc::c_ulong != 0 {
                error(
                    1 as libc::c_int,
                    0 as libc::c_int,
                    dcgettext(
                        0 as *const libc::c_char,
                        b"%s: too many checksum lines\0" as *const u8
                            as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    quotearg_n_style_colon(
                        0 as libc::c_int,
                        shell_escape_quoting_style,
                        checkfile_name,
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
                        b"%s: too many checksum lines\0" as *const u8
                            as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    quotearg_n_style_colon(
                        0 as libc::c_int,
                        shell_escape_quoting_style,
                        checkfile_name,
                    ),
                );
                if 0 as libc::c_int != 0 {} else {
                    unreachable!();
                };
            };
        }
        line_length = getline(&mut line, &mut line_chars_allocated, checkfile_stream);
        if line_length <= 0 as libc::c_int as libc::c_long {
            break;
        }
        if !(*line.offset(0 as libc::c_int as isize) as libc::c_int == '#' as i32) {
            line_length
                -= (*line
                    .offset((line_length - 1 as libc::c_int as libc::c_long) as isize)
                    as libc::c_int == '\n' as i32) as libc::c_int as libc::c_long;
            line_length
                -= (*line
                    .offset(
                        (line_length
                            - ((0 as libc::c_int as libc::c_long) < line_length)
                                as libc::c_int as libc::c_long) as isize,
                    ) as libc::c_int == '\r' as i32) as libc::c_int as libc::c_long;
            if !(line_length == 0 as libc::c_int as libc::c_long) {
                *line.offset(line_length as isize) = '\0' as i32 as libc::c_char;
                let mut d_len: size_t = 0;
                if !(split_3(
                    line,
                    line_length as size_t,
                    &mut digest,
                    &mut d_len,
                    &mut binary,
                    &mut filename,
                ) as libc::c_int != 0
                    && !(is_stdin as libc::c_int != 0
                        && strcmp(filename, b"-\0" as *const u8 as *const libc::c_char)
                            == 0 as libc::c_int))
                {
                    n_misformatted_lines = n_misformatted_lines.wrapping_add(1);
                    if warn {
                        error(
                            0 as libc::c_int,
                            0 as libc::c_int,
                            dcgettext(
                                0 as *const libc::c_char,
                                b"%s: %lu: improperly formatted %s checksum line\0"
                                    as *const u8 as *const libc::c_char,
                                5 as libc::c_int,
                            ),
                            quotearg_n_style_colon(
                                0 as libc::c_int,
                                shell_escape_quoting_style,
                                checkfile_name,
                            ),
                            line_number,
                            b"BSD\0" as *const u8 as *const libc::c_char,
                        );
                    }
                } else {
                    let mut ok: bool = false;
                    let mut missing: bool = false;
                    let mut needs_escape: bool = !status_only
                        && !(strchr(filename, '\n' as i32)).is_null();
                    properly_formatted_lines = 1 as libc::c_int != 0;
                    let mut length: uintmax_t = 0;
                    ok = digest_file(
                        filename,
                        &mut binary,
                        bin_buffer,
                        &mut missing,
                        &mut length,
                    );
                    if !ok {
                        n_open_or_read_failures = n_open_or_read_failures
                            .wrapping_add(1);
                        if !status_only {
                            if needs_escape {
                                putchar_unlocked('\\' as i32);
                            }
                            print_filename(filename, needs_escape);
                            printf(
                                b": %s\n\0" as *const u8 as *const libc::c_char,
                                dcgettext(
                                    0 as *const libc::c_char,
                                    b"FAILED open or read\0" as *const u8
                                        as *const libc::c_char,
                                    5 as libc::c_int,
                                ),
                            );
                        }
                    } else if !(ignore_missing as libc::c_int != 0
                        && missing as libc::c_int != 0)
                    {
                        let mut match_0: bool = 0 as libc::c_int != 0;
                        if d_len == digest_hex_bytes {
                            match_0 = hex_equal(digest, bin_buffer);
                        }
                        if match_0 {
                            matched_checksums = 1 as libc::c_int != 0;
                        } else {
                            n_mismatched_checksums = n_mismatched_checksums
                                .wrapping_add(1);
                        }
                        if !status_only {
                            if !matched_checksums || !quiet {
                                if needs_escape {
                                    putchar_unlocked('\\' as i32);
                                }
                                print_filename(filename, needs_escape);
                            }
                            if !matched_checksums {
                                printf(
                                    b": %s\n\0" as *const u8 as *const libc::c_char,
                                    dcgettext(
                                        0 as *const libc::c_char,
                                        b"FAILED\0" as *const u8 as *const libc::c_char,
                                        5 as libc::c_int,
                                    ),
                                );
                            } else if !quiet {
                                printf(
                                    b": %s\n\0" as *const u8 as *const libc::c_char,
                                    dcgettext(
                                        0 as *const libc::c_char,
                                        b"OK\0" as *const u8 as *const libc::c_char,
                                        5 as libc::c_int,
                                    ),
                                );
                            }
                        }
                    }
                }
            }
        }
        if !(feof_unlocked(checkfile_stream) == 0
            && ferror_unlocked(checkfile_stream) == 0)
        {
            break;
        }
    }
    free(line as *mut libc::c_void);
    let mut err: libc::c_int = if ferror_unlocked(checkfile_stream) != 0 {
        0 as libc::c_int
    } else {
        -(1 as libc::c_int)
    };
    if is_stdin {
        clearerr_unlocked(checkfile_stream);
    } else if rpl_fclose(checkfile_stream) != 0 as libc::c_int && err < 0 as libc::c_int
    {
        err = *__errno_location();
    }
    if 0 as libc::c_int <= err {
        error(
            0 as libc::c_int,
            err,
            if err != 0 {
                b"%s\0" as *const u8 as *const libc::c_char
            } else {
                dcgettext(
                    0 as *const libc::c_char,
                    b"%s: read error\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ) as *const libc::c_char
            },
            quotearg_n_style_colon(
                0 as libc::c_int,
                shell_escape_quoting_style,
                checkfile_name,
            ),
        );
        return 0 as libc::c_int != 0;
    }
    if !properly_formatted_lines {
        error(
            0 as libc::c_int,
            0 as libc::c_int,
            dcgettext(
                0 as *const libc::c_char,
                b"%s: no properly formatted checksum lines found\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
            quotearg_n_style_colon(
                0 as libc::c_int,
                shell_escape_quoting_style,
                checkfile_name,
            ),
        );
    } else if !status_only {
        if n_misformatted_lines != 0 as libc::c_int as libc::c_ulong {
            error(
                0 as libc::c_int,
                0 as libc::c_int,
                dcngettext(
                    0 as *const libc::c_char,
                    b"WARNING: %lu line is improperly formatted\0" as *const u8
                        as *const libc::c_char,
                    b"WARNING: %lu lines are improperly formatted\0" as *const u8
                        as *const libc::c_char,
                    select_plural(n_misformatted_lines),
                    5 as libc::c_int,
                ),
                n_misformatted_lines,
            );
        }
        if n_open_or_read_failures != 0 as libc::c_int as libc::c_ulong {
            error(
                0 as libc::c_int,
                0 as libc::c_int,
                dcngettext(
                    0 as *const libc::c_char,
                    b"WARNING: %lu listed file could not be read\0" as *const u8
                        as *const libc::c_char,
                    b"WARNING: %lu listed files could not be read\0" as *const u8
                        as *const libc::c_char,
                    select_plural(n_open_or_read_failures),
                    5 as libc::c_int,
                ),
                n_open_or_read_failures,
            );
        }
        if n_mismatched_checksums != 0 as libc::c_int as libc::c_ulong {
            error(
                0 as libc::c_int,
                0 as libc::c_int,
                dcngettext(
                    0 as *const libc::c_char,
                    b"WARNING: %lu computed checksum did NOT match\0" as *const u8
                        as *const libc::c_char,
                    b"WARNING: %lu computed checksums did NOT match\0" as *const u8
                        as *const libc::c_char,
                    select_plural(n_mismatched_checksums),
                    5 as libc::c_int,
                ),
                n_mismatched_checksums,
            );
        }
        if ignore_missing as libc::c_int != 0 && !matched_checksums {
            error(
                0 as libc::c_int,
                0 as libc::c_int,
                dcgettext(
                    0 as *const libc::c_char,
                    b"%s: no file was verified\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
                quotearg_n_style_colon(
                    0 as libc::c_int,
                    shell_escape_quoting_style,
                    checkfile_name,
                ),
            );
        }
    }
    return properly_formatted_lines as libc::c_int != 0
        && matched_checksums as libc::c_int != 0
        && n_mismatched_checksums == 0 as libc::c_int as libc::c_ulong
        && n_open_or_read_failures == 0 as libc::c_int as libc::c_ulong
        && (!strict || n_misformatted_lines == 0 as libc::c_int as libc::c_ulong);
}
unsafe fn main_0(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut bin_buffer_unaligned: [libc::c_uchar; 6] = [0; 6];
    let mut bin_buffer: *mut libc::c_uchar = ptr_align(
        bin_buffer_unaligned.as_mut_ptr() as *const libc::c_void,
        4 as libc::c_int as size_t,
    ) as *mut libc::c_uchar;
    let mut do_check: bool = 0 as libc::c_int != 0;
    let mut opt: libc::c_int = 0;
    let mut ok: bool = 1 as libc::c_int != 0;
    let mut binary: libc::c_int = -(1 as libc::c_int);
    let mut prefix_tag: bool = 0 as libc::c_int != 0;
    set_program_name(*argv.offset(0 as libc::c_int as isize));
    setlocale(6 as libc::c_int, b"\0" as *const u8 as *const libc::c_char);
    bindtextdomain(
        b"coreutils\0" as *const u8 as *const libc::c_char,
        b"/usr/local/share/locale\0" as *const u8 as *const libc::c_char,
    );
    textdomain(b"coreutils\0" as *const u8 as *const libc::c_char);
    atexit(Some(close_stdout as unsafe extern "C" fn() -> ()));
    setvbuf(
        stdout,
        0 as *mut libc::c_char,
        1 as libc::c_int,
        0 as libc::c_int as size_t,
    );
    let mut short_opts: *const libc::c_char = b"rs\0" as *const u8
        as *const libc::c_char;
    loop {
        opt = getopt_long(
            argc,
            argv,
            short_opts,
            long_options.as_ptr(),
            0 as *mut libc::c_int,
        );
        if !(opt != -(1 as libc::c_int)) {
            break;
        }
        match opt {
            114 => {
                sum_algorithm = bsd;
            }
            115 => {
                sum_algorithm = sysv;
            }
            -2 => {
                usage(0 as libc::c_int);
            }
            -3 => {
                version_etc(
                    stdout,
                    b"sum\0" as *const u8 as *const libc::c_char,
                    b"GNU coreutils\0" as *const u8 as *const libc::c_char,
                    Version,
                    b"Kayvan Aghaiepour\0" as *const u8 as *const libc::c_char,
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
    min_digest_line_length = (16 as libc::c_int / 4 as libc::c_int + 1 as libc::c_int
        + 1 as libc::c_int) as size_t;
    digest_hex_bytes = (16 as libc::c_int / 4 as libc::c_int) as size_t;
    if prefix_tag as libc::c_int != 0 && binary == 0 {
        error(
            0 as libc::c_int,
            0 as libc::c_int,
            dcgettext(
                0 as *const libc::c_char,
                b"--tag does not support --text mode\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
        usage(1 as libc::c_int);
    }
    if digest_delim as libc::c_int != '\n' as i32 && do_check as libc::c_int != 0 {
        error(
            0 as libc::c_int,
            0 as libc::c_int,
            dcgettext(
                0 as *const libc::c_char,
                b"the --zero option is not supported when verifying checksums\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
        usage(1 as libc::c_int);
    }
    if prefix_tag as libc::c_int != 0 && do_check as libc::c_int != 0 {
        error(
            0 as libc::c_int,
            0 as libc::c_int,
            dcgettext(
                0 as *const libc::c_char,
                b"the --tag option is meaningless when verifying checksums\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
        usage(1 as libc::c_int);
    }
    if 0 as libc::c_int <= binary && do_check as libc::c_int != 0 {
        error(
            0 as libc::c_int,
            0 as libc::c_int,
            dcgettext(
                0 as *const libc::c_char,
                b"the --binary and --text options are meaningless when verifying checksums\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
        usage(1 as libc::c_int);
    }
    if ignore_missing as libc::c_int != 0 && !do_check {
        error(
            0 as libc::c_int,
            0 as libc::c_int,
            dcgettext(
                0 as *const libc::c_char,
                b"the --ignore-missing option is meaningful only when verifying checksums\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
        usage(1 as libc::c_int);
    }
    if status_only as libc::c_int != 0 && !do_check {
        error(
            0 as libc::c_int,
            0 as libc::c_int,
            dcgettext(
                0 as *const libc::c_char,
                b"the --status option is meaningful only when verifying checksums\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
        usage(1 as libc::c_int);
    }
    if warn as libc::c_int != 0 && !do_check {
        error(
            0 as libc::c_int,
            0 as libc::c_int,
            dcgettext(
                0 as *const libc::c_char,
                b"the --warn option is meaningful only when verifying checksums\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
        usage(1 as libc::c_int);
    }
    if quiet as libc::c_int != 0 && !do_check {
        error(
            0 as libc::c_int,
            0 as libc::c_int,
            dcgettext(
                0 as *const libc::c_char,
                b"the --quiet option is meaningful only when verifying checksums\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
        usage(1 as libc::c_int);
    }
    if strict as libc::c_int & !do_check as libc::c_int != 0 {
        error(
            0 as libc::c_int,
            0 as libc::c_int,
            dcgettext(
                0 as *const libc::c_char,
                b"the --strict option is meaningful only when verifying checksums\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
        usage(1 as libc::c_int);
    }
    if 0 as libc::c_int == 0 && binary < 0 as libc::c_int {
        binary = 0 as libc::c_int;
    }
    let mut operand_lim: *mut *mut libc::c_char = argv.offset(argc as isize);
    if optind == argc {
        let fresh8 = operand_lim;
        operand_lim = operand_lim.offset(1);
        *fresh8 = bad_cast(b"-\0" as *const u8 as *const libc::c_char);
    } else if (1 as libc::c_int) < argc - optind && raw_digest as libc::c_int != 0 {
        if ::core::mem::size_of::<C2RustUnnamed_4>() as libc::c_ulong != 0 {
            error(
                1 as libc::c_int,
                0 as libc::c_int,
                dcgettext(
                    0 as *const libc::c_char,
                    b"the --raw option is not supported with multiple files\0"
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
                    b"the --raw option is not supported with multiple files\0"
                        as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
            );
            if 0 as libc::c_int != 0 {} else {
                unreachable!();
            };
        };
    }
    let mut operandp: *mut *mut libc::c_char = argv.offset(optind as isize);
    while operandp < operand_lim {
        let mut file: *mut libc::c_char = *operandp;
        if do_check {
            ok = (ok as libc::c_int & digest_check(file) as libc::c_int) as bool;
        } else {
            let mut binary_file: libc::c_int = binary;
            let mut missing: bool = false;
            let mut length: uintmax_t = 0;
            if !digest_file(
                file,
                &mut binary_file,
                bin_buffer,
                &mut missing,
                &mut length,
            ) {
                ok = 0 as libc::c_int != 0;
            } else {
                (sum_output_fns[sum_algorithm as usize])
                    .expect(
                        "non-null function pointer",
                    )(
                    file,
                    binary_file,
                    bin_buffer as *const libc::c_void,
                    raw_digest,
                    prefix_tag,
                    digest_delim,
                    optind != argc,
                    length,
                );
            }
        }
        operandp = operandp.offset(1);
    }
    if have_read_stdin as libc::c_int != 0 && rpl_fclose(stdin) == -(1 as libc::c_int) {
        if ::core::mem::size_of::<C2RustUnnamed_3>() as libc::c_ulong != 0 {
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
