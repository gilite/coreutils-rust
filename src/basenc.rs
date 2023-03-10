#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![feature(extern_types, label_break_value)]
extern crate libc;

extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    static mut stdin: *mut FILE;
    static mut stdout: *mut FILE;
    static mut stderr: *mut FILE;
    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn fputs_unlocked(__s: *const libc::c_char, __stream: *mut FILE) -> libc::c_int;
    fn fread_unlocked(
        __ptr: *mut libc::c_void,
        __size: size_t,
        __n: size_t,
        __stream: *mut FILE,
    ) -> size_t;
    fn fwrite_unlocked(
        __ptr: *const libc::c_void,
        __size: size_t,
        __n: size_t,
        __stream: *mut FILE,
    ) -> size_t;
    fn __uflow(_: *mut FILE) -> libc::c_int;
    fn __overflow(_: *mut FILE, _: libc::c_int) -> libc::c_int;
    fn rpl_fclose(stream: *mut FILE) -> libc::c_int;
    static mut optarg: *mut libc::c_char;
    static mut optind: libc::c_int;
    fn getopt_long(
        ___argc: libc::c_int,
        ___argv: *const *mut libc::c_char,
        __shortopts: *const libc::c_char,
        __longopts: *const option,
        __longind: *mut libc::c_int,
    ) -> libc::c_int;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn memmove(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strncmp(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn __errno_location() -> *mut libc::c_int;
    fn atexit(__func: Option::<unsafe extern "C" fn() -> ()>) -> libc::c_int;
    fn exit(_: libc::c_int) -> !;
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
    fn xmalloc(s: size_t) -> *mut libc::c_void;
    fn xreallocarray(p: *mut libc::c_void, n: size_t, s: size_t) -> *mut libc::c_void;
    fn xcharalloc(n: size_t) -> *mut libc::c_char;
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
    fn error(
        __status: libc::c_int,
        __errnum: libc::c_int,
        __format: *const libc::c_char,
        _: ...
    );
    fn fadvise(fp: *mut FILE, advice: fadvice_t);
    fn quote(arg: *const libc::c_char) -> *const libc::c_char;
    fn xstrtoimax(
        _: *const libc::c_char,
        _: *mut *mut libc::c_char,
        _: libc::c_int,
        _: *mut intmax_t,
        _: *const libc::c_char,
    ) -> strtol_error;
    fn isbase32(ch: libc::c_char) -> bool;
    fn base32_encode(
        in_0: *const libc::c_char,
        inlen: idx_t,
        out: *mut libc::c_char,
        outlen: idx_t,
    );
    fn base32_decode_ctx_init(ctx: *mut base32_decode_context);
    fn base32_decode_ctx(
        ctx: *mut base32_decode_context,
        in_0: *const libc::c_char,
        inlen: idx_t,
        out: *mut libc::c_char,
        outlen: *mut idx_t,
    ) -> bool;
    fn isbase64(ch: libc::c_char) -> bool;
    fn base64_encode(
        in_0: *const libc::c_char,
        inlen: idx_t,
        out: *mut libc::c_char,
        outlen: idx_t,
    );
    fn base64_decode_ctx_init(ctx: *mut base64_decode_context);
    fn base64_decode_ctx(
        ctx: *mut base64_decode_context,
        in_0: *const libc::c_char,
        inlen: idx_t,
        out: *mut libc::c_char,
        outlen: *mut idx_t,
    ) -> bool;
}
pub type size_t = libc::c_ulong;
pub type __intmax_t = libc::c_long;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
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
pub struct option {
    pub name: *const libc::c_char,
    pub has_arg: libc::c_int,
    pub flag: *mut libc::c_int,
    pub val: libc::c_int,
}
pub type int_fast64_t = libc::c_long;
pub type intmax_t = __intmax_t;
pub type idx_t = ptrdiff_t;
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
pub struct base32_decode_context {
    pub i: libc::c_int,
    pub buf: [libc::c_char; 8],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct base64_decode_context {
    pub i: libc::c_int,
    pub buf: [libc::c_char; 4],
}
pub type C2RustUnnamed_0 = libc::c_uint;
pub const Z85_OPTION: C2RustUnnamed_0 = 263;
pub const BASE2LSBF_OPTION: C2RustUnnamed_0 = 262;
pub const BASE2MSBF_OPTION: C2RustUnnamed_0 = 261;
pub const BASE16_OPTION: C2RustUnnamed_0 = 260;
pub const BASE32HEX_OPTION: C2RustUnnamed_0 = 259;
pub const BASE32_OPTION: C2RustUnnamed_0 = 258;
pub const BASE64URL_OPTION: C2RustUnnamed_0 = 257;
pub const BASE64_OPTION: C2RustUnnamed_0 = 256;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct base16_decode_context {
    pub nibble: libc::c_char,
    pub have_nibble: bool,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct z85_decode_context {
    pub i: libc::c_int,
    pub octets: [libc::c_uchar; 5],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct base2_decode_context {
    pub octet: libc::c_uchar,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct base_decode_context {
    pub i: libc::c_int,
    pub ctx: C2RustUnnamed_1,
    pub inbuf: *mut libc::c_char,
    pub bufsize: idx_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_1 {
    pub base64: base64_decode_context,
    pub base32: base32_decode_context,
    pub base16: base16_decode_context,
    pub base2: base2_decode_context,
    pub z85: z85_decode_context,
}
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_8 {
    pub _gl_dummy: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_9 {
    pub _gl_dummy: libc::c_int,
}
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
unsafe extern "C" fn fputc_unlocked(
    mut __c: libc::c_int,
    mut __stream: *mut FILE,
) -> libc::c_int {
    return if ((*__stream)._IO_write_ptr >= (*__stream)._IO_write_end) as libc::c_int
        as libc::c_long != 0
    {
        __overflow(__stream, __c as libc::c_uchar as libc::c_int)
    } else {
        let fresh1 = (*__stream)._IO_write_ptr;
        (*__stream)._IO_write_ptr = ((*__stream)._IO_write_ptr).offset(1);
        *fresh1 = __c as libc::c_char;
        *fresh1 as libc::c_uchar as libc::c_int
    };
}
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
        let fresh2 = (*__stream)._IO_write_ptr;
        (*__stream)._IO_write_ptr = ((*__stream)._IO_write_ptr).offset(1);
        *fresh2 = __c as libc::c_char;
        *fresh2 as libc::c_uchar as libc::c_int
    };
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
unsafe extern "C" fn xnrealloc(
    mut p: *mut libc::c_void,
    mut n: size_t,
    mut s: size_t,
) -> *mut libc::c_void {
    return xreallocarray(p, n, s);
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
static mut long_options: [option; 14] = [
    {
        let mut init = option {
            name: b"decode\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'd' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"wrap\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'w' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"ignore-garbage\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'i' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"base64\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: BASE64_OPTION as libc::c_int,
        };
        init
    },
    {
        let mut init = option {
            name: b"base64url\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: BASE64URL_OPTION as libc::c_int,
        };
        init
    },
    {
        let mut init = option {
            name: b"base32\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: BASE32_OPTION as libc::c_int,
        };
        init
    },
    {
        let mut init = option {
            name: b"base32hex\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: BASE32HEX_OPTION as libc::c_int,
        };
        init
    },
    {
        let mut init = option {
            name: b"base16\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: BASE16_OPTION as libc::c_int,
        };
        init
    },
    {
        let mut init = option {
            name: b"base2msbf\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: BASE2MSBF_OPTION as libc::c_int,
        };
        init
    },
    {
        let mut init = option {
            name: b"base2lsbf\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: BASE2LSBF_OPTION as libc::c_int,
        };
        init
    },
    {
        let mut init = option {
            name: b"z85\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: Z85_OPTION as libc::c_int,
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
                b"Usage: %s [OPTION]... [FILE]\n\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            program_name,
        );
        fputs_unlocked(
            dcgettext(
                0 as *const libc::c_char,
                b"basenc encode or decode FILE, or standard input, to standard output.\n\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            stdout,
        );
        emit_stdin_note();
        emit_mandatory_arg_note();
        fputs_unlocked(
            dcgettext(
                0 as *const libc::c_char,
                b"      --base64          same as 'base64' program (RFC4648 section 4)\n\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            stdout,
        );
        fputs_unlocked(
            dcgettext(
                0 as *const libc::c_char,
                b"      --base64url       file- and url-safe base64 (RFC4648 section 5)\n\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            stdout,
        );
        fputs_unlocked(
            dcgettext(
                0 as *const libc::c_char,
                b"      --base32          same as 'base32' program (RFC4648 section 6)\n\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            stdout,
        );
        fputs_unlocked(
            dcgettext(
                0 as *const libc::c_char,
                b"      --base32hex       extended hex alphabet base32 (RFC4648 section 7)\n\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            stdout,
        );
        fputs_unlocked(
            dcgettext(
                0 as *const libc::c_char,
                b"      --base16          hex encoding (RFC4648 section 8)\n\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            stdout,
        );
        fputs_unlocked(
            dcgettext(
                0 as *const libc::c_char,
                b"      --base2msbf       bit string with most significant bit (msb) first\n\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            stdout,
        );
        fputs_unlocked(
            dcgettext(
                0 as *const libc::c_char,
                b"      --base2lsbf       bit string with least significant bit (lsb) first\n\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            stdout,
        );
        fputs_unlocked(
            dcgettext(
                0 as *const libc::c_char,
                b"  -d, --decode          decode data\n  -i, --ignore-garbage  when decoding, ignore non-alphabet characters\n  -w, --wrap=COLS       wrap encoded lines after COLS character (default 76).\n                          Use 0 to disable line wrapping\n\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            stdout,
        );
        fputs_unlocked(
            dcgettext(
                0 as *const libc::c_char,
                b"      --z85             ascii85-like encoding (ZeroMQ spec:32/Z85);\n                        when encoding, input length must be a multiple of 4;\n                        when decoding, input length must be a multiple of 5\n\0"
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
                b"\nWhen decoding, the input may contain newlines in addition to the bytes of\nthe formal alphabet.  Use --ignore-garbage to attempt to recover\nfrom any other non-alphabet bytes in the encoded stream.\n\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            stdout,
        );
        emit_ancillary_info(b"basenc\0" as *const u8 as *const libc::c_char);
    }
    exit(status);
}
static mut base_length: Option::<unsafe extern "C" fn(libc::c_int) -> libc::c_int> = None;
static mut isbase: Option::<unsafe extern "C" fn(libc::c_char) -> bool> = None;
static mut base_encode: Option::<
    unsafe extern "C" fn(*const libc::c_char, idx_t, *mut libc::c_char, idx_t) -> (),
> = None;
static mut base_decode_ctx_init: Option::<
    unsafe extern "C" fn(*mut base_decode_context) -> (),
> = None;
static mut base_decode_ctx: Option::<
    unsafe extern "C" fn(
        *mut base_decode_context,
        *const libc::c_char,
        idx_t,
        *mut libc::c_char,
        *mut idx_t,
    ) -> bool,
> = None;
unsafe extern "C" fn base64_length_wrapper(mut len: libc::c_int) -> libc::c_int {
    return (len + 2 as libc::c_int) / 3 as libc::c_int * 4 as libc::c_int;
}
unsafe extern "C" fn base64_decode_ctx_init_wrapper(mut ctx: *mut base_decode_context) {
    base64_decode_ctx_init(&mut (*ctx).ctx.base64);
}
unsafe extern "C" fn base64_decode_ctx_wrapper(
    mut ctx: *mut base_decode_context,
    mut in_0: *const libc::c_char,
    mut inlen: idx_t,
    mut out: *mut libc::c_char,
    mut outlen: *mut idx_t,
) -> bool {
    let mut b: bool = base64_decode_ctx(
        &mut (*ctx).ctx.base64,
        in_0,
        inlen,
        out,
        outlen,
    );
    (*ctx).i = (*ctx).ctx.base64.i;
    return b;
}
unsafe extern "C" fn init_inbuf(mut ctx: *mut base_decode_context) {
    (*ctx).bufsize = 4200 as libc::c_int as idx_t;
    (*ctx).inbuf = xcharalloc((*ctx).bufsize as size_t);
}
unsafe extern "C" fn prepare_inbuf(mut ctx: *mut base_decode_context, mut inlen: idx_t) {
    if (*ctx).bufsize < inlen {
        (*ctx).bufsize = inlen * 2 as libc::c_int as libc::c_long;
        (*ctx)
            .inbuf = xnrealloc(
            (*ctx).inbuf as *mut libc::c_void,
            (*ctx).bufsize as size_t,
            ::core::mem::size_of::<libc::c_char>() as libc::c_ulong,
        ) as *mut libc::c_char;
    }
}
unsafe extern "C" fn base64url_encode(
    mut in_0: *const libc::c_char,
    mut inlen: idx_t,
    mut out: *mut libc::c_char,
    mut outlen: idx_t,
) {
    base64_encode(in_0, inlen, out, outlen);
    let mut p: *mut libc::c_char = out;
    loop {
        let fresh3 = outlen;
        outlen = outlen - 1;
        if !(fresh3 != 0) {
            break;
        }
        if *p as libc::c_int == '+' as i32 {
            *p = '-' as i32 as libc::c_char;
        } else if *p as libc::c_int == '/' as i32 {
            *p = '_' as i32 as libc::c_char;
        }
        p = p.offset(1);
    };
}
unsafe extern "C" fn isbase64url(mut ch: libc::c_char) -> bool {
    return ch as libc::c_int == '-' as i32 || ch as libc::c_int == '_' as i32
        || ch as libc::c_int != '+' as i32 && ch as libc::c_int != '/' as i32
            && isbase64(ch) as libc::c_int != 0;
}
unsafe extern "C" fn base64url_decode_ctx_init_wrapper(
    mut ctx: *mut base_decode_context,
) {
    base64_decode_ctx_init(&mut (*ctx).ctx.base64);
    init_inbuf(ctx);
}
unsafe extern "C" fn base64url_decode_ctx_wrapper(
    mut ctx: *mut base_decode_context,
    mut in_0: *const libc::c_char,
    mut inlen: idx_t,
    mut out: *mut libc::c_char,
    mut outlen: *mut idx_t,
) -> bool {
    prepare_inbuf(ctx, inlen);
    memcpy(
        (*ctx).inbuf as *mut libc::c_void,
        in_0 as *const libc::c_void,
        inlen as libc::c_ulong,
    );
    let mut i: idx_t = inlen;
    let mut p: *mut libc::c_char = (*ctx).inbuf;
    loop {
        let fresh4 = i;
        i = i - 1;
        if !(fresh4 != 0) {
            break;
        }
        if *p as libc::c_int == '+' as i32 || *p as libc::c_int == '/' as i32 {
            *outlen = 0 as libc::c_int as idx_t;
            return 0 as libc::c_int != 0;
        } else {
            if *p as libc::c_int == '-' as i32 {
                *p = '+' as i32 as libc::c_char;
            } else if *p as libc::c_int == '_' as i32 {
                *p = '/' as i32 as libc::c_char;
            }
        }
        p = p.offset(1);
    }
    let mut b: bool = base64_decode_ctx(
        &mut (*ctx).ctx.base64,
        (*ctx).inbuf,
        inlen,
        out,
        outlen,
    );
    (*ctx).i = (*ctx).ctx.base64.i;
    return b;
}
unsafe extern "C" fn base32_length_wrapper(mut len: libc::c_int) -> libc::c_int {
    return (len + 4 as libc::c_int) / 5 as libc::c_int * 8 as libc::c_int;
}
unsafe extern "C" fn base32_decode_ctx_init_wrapper(mut ctx: *mut base_decode_context) {
    base32_decode_ctx_init(&mut (*ctx).ctx.base32);
}
unsafe extern "C" fn base32_decode_ctx_wrapper(
    mut ctx: *mut base_decode_context,
    mut in_0: *const libc::c_char,
    mut inlen: idx_t,
    mut out: *mut libc::c_char,
    mut outlen: *mut idx_t,
) -> bool {
    let mut b: bool = base32_decode_ctx(
        &mut (*ctx).ctx.base32,
        in_0,
        inlen,
        out,
        outlen,
    );
    (*ctx).i = (*ctx).ctx.base32.i;
    return b;
}
static mut base32_norm_to_hex: [libc::c_char; 41] = [
    'Q' as i32 as libc::c_char,
    'R' as i32 as libc::c_char,
    'S' as i32 as libc::c_char,
    'T' as i32 as libc::c_char,
    'U' as i32 as libc::c_char,
    'V' as i32 as libc::c_char,
    0x38 as libc::c_int as libc::c_char,
    0x39 as libc::c_int as libc::c_char,
    0x3a as libc::c_int as libc::c_char,
    0x3b as libc::c_int as libc::c_char,
    0x3c as libc::c_int as libc::c_char,
    0x3d as libc::c_int as libc::c_char,
    0x3e as libc::c_int as libc::c_char,
    0x3f as libc::c_int as libc::c_char,
    0x40 as libc::c_int as libc::c_char,
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
    'A' as i32 as libc::c_char,
    'B' as i32 as libc::c_char,
    'C' as i32 as libc::c_char,
    'D' as i32 as libc::c_char,
    'E' as i32 as libc::c_char,
    'F' as i32 as libc::c_char,
    'G' as i32 as libc::c_char,
    'H' as i32 as libc::c_char,
    'I' as i32 as libc::c_char,
    'J' as i32 as libc::c_char,
    'K' as i32 as libc::c_char,
    'L' as i32 as libc::c_char,
    'M' as i32 as libc::c_char,
    'N' as i32 as libc::c_char,
    'O' as i32 as libc::c_char,
    'P' as i32 as libc::c_char,
];
static mut base32_hex_to_norm: [libc::c_char; 41] = [
    'A' as i32 as libc::c_char,
    'B' as i32 as libc::c_char,
    'C' as i32 as libc::c_char,
    'D' as i32 as libc::c_char,
    'E' as i32 as libc::c_char,
    'F' as i32 as libc::c_char,
    'G' as i32 as libc::c_char,
    'H' as i32 as libc::c_char,
    'I' as i32 as libc::c_char,
    'J' as i32 as libc::c_char,
    0x3a as libc::c_int as libc::c_char,
    0x3b as libc::c_int as libc::c_char,
    0x3c as libc::c_int as libc::c_char,
    0x3d as libc::c_int as libc::c_char,
    0x3e as libc::c_int as libc::c_char,
    0x3f as libc::c_int as libc::c_char,
    0x40 as libc::c_int as libc::c_char,
    'K' as i32 as libc::c_char,
    'L' as i32 as libc::c_char,
    'M' as i32 as libc::c_char,
    'N' as i32 as libc::c_char,
    'O' as i32 as libc::c_char,
    'P' as i32 as libc::c_char,
    'Q' as i32 as libc::c_char,
    'R' as i32 as libc::c_char,
    'S' as i32 as libc::c_char,
    'T' as i32 as libc::c_char,
    'U' as i32 as libc::c_char,
    'V' as i32 as libc::c_char,
    'W' as i32 as libc::c_char,
    'X' as i32 as libc::c_char,
    'Y' as i32 as libc::c_char,
    'Z' as i32 as libc::c_char,
    '2' as i32 as libc::c_char,
    '3' as i32 as libc::c_char,
    '4' as i32 as libc::c_char,
    '5' as i32 as libc::c_char,
    '6' as i32 as libc::c_char,
    '7' as i32 as libc::c_char,
    0,
    0,
];
#[inline]
unsafe extern "C" fn isbase32hex(mut ch: libc::c_char) -> bool {
    return '0' as i32 <= ch as libc::c_int && ch as libc::c_int <= '9' as i32
        || 'A' as i32 <= ch as libc::c_int && ch as libc::c_int <= 'V' as i32;
}
unsafe extern "C" fn base32hex_encode(
    mut in_0: *const libc::c_char,
    mut inlen: idx_t,
    mut out: *mut libc::c_char,
    mut outlen: idx_t,
) {
    base32_encode(in_0, inlen, out, outlen);
    let mut p: *mut libc::c_char = out;
    loop {
        let fresh5 = outlen;
        outlen = outlen - 1;
        if !(fresh5 != 0) {
            break;
        }
        if 0x32 as libc::c_int <= *p as libc::c_int
            && *p as libc::c_int <= 0x5a as libc::c_int
        {} else {
            __assert_fail(
                b"0x32 <= *p && *p <= 0x5a\0" as *const u8 as *const libc::c_char,
                b"src/basenc.c\0" as *const u8 as *const libc::c_char,
                452 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 74],
                    &[libc::c_char; 74],
                >(
                    b"void base32hex_encode(const char *restrict, idx_t, char *restrict, idx_t)\0",
                ))
                    .as_ptr(),
            );
        }
        *p = base32_norm_to_hex[(*p as libc::c_int - 0x32 as libc::c_int) as usize];
        p = p.offset(1);
    };
}
unsafe extern "C" fn base32hex_decode_ctx_init_wrapper(
    mut ctx: *mut base_decode_context,
) {
    base32_decode_ctx_init(&mut (*ctx).ctx.base32);
    init_inbuf(ctx);
}
unsafe extern "C" fn base32hex_decode_ctx_wrapper(
    mut ctx: *mut base_decode_context,
    mut in_0: *const libc::c_char,
    mut inlen: idx_t,
    mut out: *mut libc::c_char,
    mut outlen: *mut idx_t,
) -> bool {
    prepare_inbuf(ctx, inlen);
    let mut i: idx_t = inlen;
    let mut p: *mut libc::c_char = (*ctx).inbuf;
    loop {
        let fresh6 = i;
        i = i - 1;
        if !(fresh6 != 0) {
            break;
        }
        if isbase32hex(*in_0) {
            *p = base32_hex_to_norm[(*in_0 as libc::c_int - 0x30 as libc::c_int)
                as usize];
        } else {
            *p = *in_0;
        }
        p = p.offset(1);
        in_0 = in_0.offset(1);
    }
    let mut b: bool = base32_decode_ctx(
        &mut (*ctx).ctx.base32,
        (*ctx).inbuf,
        inlen,
        out,
        outlen,
    );
    (*ctx).i = (*ctx).ctx.base32.i;
    return b;
}
unsafe extern "C" fn isbase16(mut ch: libc::c_char) -> bool {
    return '0' as i32 <= ch as libc::c_int && ch as libc::c_int <= '9' as i32
        || 'A' as i32 <= ch as libc::c_int && ch as libc::c_int <= 'F' as i32;
}
unsafe extern "C" fn base16_length(mut len: libc::c_int) -> libc::c_int {
    return len * 2 as libc::c_int;
}
static mut base16: [libc::c_char; 16] = unsafe {
    *::core::mem::transmute::<&[u8; 16], &[libc::c_char; 16]>(b"0123456789ABCDEF")
};
unsafe extern "C" fn base16_encode(
    mut in_0: *const libc::c_char,
    mut inlen: idx_t,
    mut out: *mut libc::c_char,
    mut outlen: idx_t,
) {
    loop {
        let fresh7 = inlen;
        inlen = inlen - 1;
        if !(fresh7 != 0) {
            break;
        }
        let mut c: libc::c_uchar = *in_0 as libc::c_uchar;
        let fresh8 = out;
        out = out.offset(1);
        *fresh8 = base16[(c as libc::c_int >> 4 as libc::c_int) as usize];
        let fresh9 = out;
        out = out.offset(1);
        *fresh9 = base16[(c as libc::c_int & 0xf as libc::c_int) as usize];
        in_0 = in_0.offset(1);
    };
}
unsafe extern "C" fn base16_decode_ctx_init(mut ctx: *mut base_decode_context) {
    init_inbuf(ctx);
    (*ctx).ctx.base16.have_nibble = 0 as libc::c_int != 0;
    (*ctx).i = 1 as libc::c_int;
}
unsafe extern "C" fn base16_decode_ctx(
    mut ctx: *mut base_decode_context,
    mut in_0: *const libc::c_char,
    mut inlen: idx_t,
    mut out: *mut libc::c_char,
    mut outlen: *mut idx_t,
) -> bool {
    let mut ignore_lines: bool = 1 as libc::c_int != 0;
    *outlen = 0 as libc::c_int as idx_t;
    if inlen == 0 as libc::c_int as libc::c_long {
        return !(*ctx).ctx.base16.have_nibble;
    }
    loop {
        let fresh10 = inlen;
        inlen = inlen - 1;
        if !(fresh10 != 0) {
            break;
        }
        if ignore_lines as libc::c_int != 0 && *in_0 as libc::c_int == '\n' as i32 {
            in_0 = in_0.offset(1);
        } else {
            let fresh11 = in_0;
            in_0 = in_0.offset(1);
            let mut nib: libc::c_int = *fresh11 as libc::c_int;
            if '0' as i32 <= nib && nib <= '9' as i32 {
                nib -= '0' as i32;
            } else if 'A' as i32 <= nib && nib <= 'F' as i32 {
                nib -= 'A' as i32 - 10 as libc::c_int;
            } else {
                return 0 as libc::c_int != 0
            }
            if (*ctx).ctx.base16.have_nibble {
                let fresh12 = out;
                out = out.offset(1);
                *fresh12 = ((((*ctx).ctx.base16.nibble as libc::c_int)
                    << 4 as libc::c_int) + nib) as libc::c_char;
                *outlen += 1;
            } else {
                (*ctx).ctx.base16.nibble = nib as libc::c_char;
            }
            (*ctx).ctx.base16.have_nibble = !(*ctx).ctx.base16.have_nibble;
        }
    }
    return 1 as libc::c_int != 0;
}
unsafe extern "C" fn z85_length(mut len: libc::c_int) -> libc::c_int {
    let mut outlen: libc::c_int = len * 5 as libc::c_int / 4 as libc::c_int;
    return outlen;
}
unsafe extern "C" fn isz85(mut ch: libc::c_char) -> bool {
    return c_isalnum(ch as libc::c_int) as libc::c_int != 0
        || !(strchr(
            b".-:+=^!/*?&<>()[]{}@%$#\0" as *const u8 as *const libc::c_char,
            ch as libc::c_int,
        ))
            .is_null();
}
static mut z85_encoding: [libc::c_char; 85] = unsafe {
    *::core::mem::transmute::<
        &[u8; 85],
        &[libc::c_char; 85],
    >(
        b"0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ.-:+=^!/*?&<>()[]{}@%$#",
    )
};
unsafe extern "C" fn z85_encode(
    mut in_0: *const libc::c_char,
    mut inlen: idx_t,
    mut out: *mut libc::c_char,
    mut outlen: idx_t,
) {
    let mut i: libc::c_int = 0 as libc::c_int;
    let mut quad: [libc::c_uchar; 4] = [0; 4];
    let mut outidx: idx_t = 0 as libc::c_int as idx_t;
    loop {
        if inlen == 0 as libc::c_int as libc::c_long {
            if i == 0 as libc::c_int {
                return;
            }
            if ::core::mem::size_of::<C2RustUnnamed_2>() as libc::c_ulong != 0 {
                error(
                    1 as libc::c_int,
                    0 as libc::c_int,
                    dcgettext(
                        0 as *const libc::c_char,
                        b"invalid input (length must be multiple of 4 characters)\0"
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
                        b"invalid input (length must be multiple of 4 characters)\0"
                            as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                );
                if 0 as libc::c_int != 0 {} else {
                    unreachable!();
                };
            };
        } else {
            let fresh13 = in_0;
            in_0 = in_0.offset(1);
            let fresh14 = i;
            i = i + 1;
            quad[fresh14 as usize] = *fresh13 as libc::c_uchar;
            inlen -= 1;
        }
        if i == 4 as libc::c_int {
            let mut val: int_fast64_t = quad[0 as libc::c_int as usize] as int_fast64_t;
            val = (val << 24 as libc::c_int)
                + ((quad[1 as libc::c_int as usize] as libc::c_int) << 16 as libc::c_int)
                    as libc::c_long
                + ((quad[2 as libc::c_int as usize] as libc::c_int) << 8 as libc::c_int)
                    as libc::c_long + quad[3 as libc::c_int as usize] as libc::c_long;
            let mut j: libc::c_int = 4 as libc::c_int;
            while j >= 0 as libc::c_int {
                let mut c: libc::c_int = (val % 85 as libc::c_int as libc::c_long)
                    as libc::c_int;
                val /= 85 as libc::c_int as libc::c_long;
                if (outidx + j as libc::c_long) < outlen {
                    *out.offset(j as isize) = z85_encoding[c as usize];
                }
                j -= 1;
            }
            out = out.offset(5 as libc::c_int as isize);
            outidx += 5 as libc::c_int as libc::c_long;
            i = 0 as libc::c_int;
        }
    };
}
unsafe extern "C" fn z85_decode_ctx_init(mut ctx: *mut base_decode_context) {
    init_inbuf(ctx);
    (*ctx).ctx.z85.i = 0 as libc::c_int;
    (*ctx).i = 1 as libc::c_int;
}
static mut z85_decoding: [libc::c_schar; 93] = [
    68 as libc::c_int as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    84 as libc::c_int as libc::c_schar,
    83 as libc::c_int as libc::c_schar,
    82 as libc::c_int as libc::c_schar,
    72 as libc::c_int as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    75 as libc::c_int as libc::c_schar,
    76 as libc::c_int as libc::c_schar,
    70 as libc::c_int as libc::c_schar,
    65 as libc::c_int as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    63 as libc::c_int as libc::c_schar,
    62 as libc::c_int as libc::c_schar,
    69 as libc::c_int as libc::c_schar,
    0 as libc::c_int as libc::c_schar,
    1 as libc::c_int as libc::c_schar,
    2 as libc::c_int as libc::c_schar,
    3 as libc::c_int as libc::c_schar,
    4 as libc::c_int as libc::c_schar,
    5 as libc::c_int as libc::c_schar,
    6 as libc::c_int as libc::c_schar,
    7 as libc::c_int as libc::c_schar,
    8 as libc::c_int as libc::c_schar,
    9 as libc::c_int as libc::c_schar,
    64 as libc::c_int as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    73 as libc::c_int as libc::c_schar,
    66 as libc::c_int as libc::c_schar,
    74 as libc::c_int as libc::c_schar,
    71 as libc::c_int as libc::c_schar,
    81 as libc::c_int as libc::c_schar,
    36 as libc::c_int as libc::c_schar,
    37 as libc::c_int as libc::c_schar,
    38 as libc::c_int as libc::c_schar,
    39 as libc::c_int as libc::c_schar,
    40 as libc::c_int as libc::c_schar,
    41 as libc::c_int as libc::c_schar,
    42 as libc::c_int as libc::c_schar,
    43 as libc::c_int as libc::c_schar,
    44 as libc::c_int as libc::c_schar,
    45 as libc::c_int as libc::c_schar,
    46 as libc::c_int as libc::c_schar,
    47 as libc::c_int as libc::c_schar,
    48 as libc::c_int as libc::c_schar,
    49 as libc::c_int as libc::c_schar,
    50 as libc::c_int as libc::c_schar,
    51 as libc::c_int as libc::c_schar,
    52 as libc::c_int as libc::c_schar,
    53 as libc::c_int as libc::c_schar,
    54 as libc::c_int as libc::c_schar,
    55 as libc::c_int as libc::c_schar,
    56 as libc::c_int as libc::c_schar,
    57 as libc::c_int as libc::c_schar,
    58 as libc::c_int as libc::c_schar,
    59 as libc::c_int as libc::c_schar,
    60 as libc::c_int as libc::c_schar,
    61 as libc::c_int as libc::c_schar,
    77 as libc::c_int as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    78 as libc::c_int as libc::c_schar,
    67 as libc::c_int as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    10 as libc::c_int as libc::c_schar,
    11 as libc::c_int as libc::c_schar,
    12 as libc::c_int as libc::c_schar,
    13 as libc::c_int as libc::c_schar,
    14 as libc::c_int as libc::c_schar,
    15 as libc::c_int as libc::c_schar,
    16 as libc::c_int as libc::c_schar,
    17 as libc::c_int as libc::c_schar,
    18 as libc::c_int as libc::c_schar,
    19 as libc::c_int as libc::c_schar,
    20 as libc::c_int as libc::c_schar,
    21 as libc::c_int as libc::c_schar,
    22 as libc::c_int as libc::c_schar,
    23 as libc::c_int as libc::c_schar,
    24 as libc::c_int as libc::c_schar,
    25 as libc::c_int as libc::c_schar,
    26 as libc::c_int as libc::c_schar,
    27 as libc::c_int as libc::c_schar,
    28 as libc::c_int as libc::c_schar,
    29 as libc::c_int as libc::c_schar,
    30 as libc::c_int as libc::c_schar,
    31 as libc::c_int as libc::c_schar,
    32 as libc::c_int as libc::c_schar,
    33 as libc::c_int as libc::c_schar,
    34 as libc::c_int as libc::c_schar,
    35 as libc::c_int as libc::c_schar,
    79 as libc::c_int as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    80 as libc::c_int as libc::c_schar,
];
unsafe extern "C" fn z85_decode_ctx(
    mut ctx: *mut base_decode_context,
    mut in_0: *const libc::c_char,
    mut inlen: idx_t,
    mut out: *mut libc::c_char,
    mut outlen: *mut idx_t,
) -> bool {
    let mut ignore_lines: bool = 1 as libc::c_int != 0;
    *outlen = 0 as libc::c_int as idx_t;
    if inlen == 0 as libc::c_int as libc::c_long {
        if (*ctx).ctx.z85.i > 0 as libc::c_int {
            return 0 as libc::c_int != 0;
        }
        return 1 as libc::c_int != 0;
    }
    loop {
        let fresh15 = inlen;
        inlen = inlen - 1;
        if !(fresh15 != 0) {
            break;
        }
        if ignore_lines as libc::c_int != 0 && *in_0 as libc::c_int == '\n' as i32 {
            in_0 = in_0.offset(1);
        } else {
            let mut c: libc::c_uchar = *in_0 as libc::c_uchar;
            if c as libc::c_int >= 33 as libc::c_int
                && c as libc::c_int <= 125 as libc::c_int
            {
                let mut ch: libc::c_schar = z85_decoding[(c as libc::c_int
                    - 33 as libc::c_int) as usize];
                if (ch as libc::c_int) < 0 as libc::c_int {
                    return 0 as libc::c_int != 0;
                }
                c = ch as libc::c_uchar;
            } else {
                return 0 as libc::c_int != 0
            }
            in_0 = in_0.offset(1);
            let fresh16 = (*ctx).ctx.z85.i;
            (*ctx).ctx.z85.i = (*ctx).ctx.z85.i + 1;
            (*ctx).ctx.z85.octets[fresh16 as usize] = c;
            if (*ctx).ctx.z85.i == 5 as libc::c_int {
                let mut val: int_fast64_t = ((*ctx)
                    .ctx
                    .z85
                    .octets[1 as libc::c_int as usize] as libc::c_int * 85 as libc::c_int
                    * 85 as libc::c_int * 85 as libc::c_int
                    + (*ctx).ctx.z85.octets[2 as libc::c_int as usize] as libc::c_int
                        * 85 as libc::c_int * 85 as libc::c_int
                    + (*ctx).ctx.z85.octets[3 as libc::c_int as usize] as libc::c_int
                        * 85 as libc::c_int
                    + (*ctx).ctx.z85.octets[4 as libc::c_int as usize] as libc::c_int)
                    as int_fast64_t;
                val
                    += (*ctx).ctx.z85.octets[0 as libc::c_int as usize] as int_fast64_t
                        * 85 as libc::c_int as libc::c_long
                        * 85 as libc::c_int as libc::c_long
                        * 85 as libc::c_int as libc::c_long
                        * 85 as libc::c_int as libc::c_long;
                if val >> 24 as libc::c_int & !(0xff as libc::c_int) as libc::c_long != 0
                {
                    return 0 as libc::c_int != 0;
                }
                let fresh17 = out;
                out = out.offset(1);
                *fresh17 = (val >> 24 as libc::c_int) as libc::c_char;
                let fresh18 = out;
                out = out.offset(1);
                *fresh18 = (val >> 16 as libc::c_int
                    & 0xff as libc::c_int as libc::c_long) as libc::c_char;
                let fresh19 = out;
                out = out.offset(1);
                *fresh19 = (val >> 8 as libc::c_int
                    & 0xff as libc::c_int as libc::c_long) as libc::c_char;
                let fresh20 = out;
                out = out.offset(1);
                *fresh20 = (val & 0xff as libc::c_int as libc::c_long) as libc::c_char;
                *outlen += 4 as libc::c_int as libc::c_long;
                (*ctx).ctx.z85.i = 0 as libc::c_int;
            }
        }
    }
    (*ctx).i = (*ctx).ctx.z85.i;
    return 1 as libc::c_int != 0;
}
#[inline]
unsafe extern "C" fn isbase2(mut ch: libc::c_char) -> bool {
    return ch as libc::c_int == '0' as i32 || ch as libc::c_int == '1' as i32;
}
unsafe extern "C" fn base2_length(mut len: libc::c_int) -> libc::c_int {
    return len * 8 as libc::c_int;
}
#[inline]
unsafe extern "C" fn base2msbf_encode(
    mut in_0: *const libc::c_char,
    mut inlen: idx_t,
    mut out: *mut libc::c_char,
    mut outlen: idx_t,
) {
    loop {
        let fresh21 = inlen;
        inlen = inlen - 1;
        if !(fresh21 != 0) {
            break;
        }
        let mut c: libc::c_uchar = *in_0 as libc::c_uchar;
        let mut i: libc::c_int = 0 as libc::c_int;
        while i < 8 as libc::c_int {
            let fresh22 = out;
            out = out.offset(1);
            *fresh22 = (if c as libc::c_int & 0x80 as libc::c_int != 0 {
                '1' as i32
            } else {
                '0' as i32
            }) as libc::c_char;
            c = ((c as libc::c_int) << 1 as libc::c_int) as libc::c_uchar;
            i += 1;
        }
        outlen -= 8 as libc::c_int as libc::c_long;
        in_0 = in_0.offset(1);
    };
}
#[inline]
unsafe extern "C" fn base2lsbf_encode(
    mut in_0: *const libc::c_char,
    mut inlen: idx_t,
    mut out: *mut libc::c_char,
    mut outlen: idx_t,
) {
    loop {
        let fresh23 = inlen;
        inlen = inlen - 1;
        if !(fresh23 != 0) {
            break;
        }
        let mut c: libc::c_uchar = *in_0 as libc::c_uchar;
        let mut i: libc::c_int = 0 as libc::c_int;
        while i < 8 as libc::c_int {
            let fresh24 = out;
            out = out.offset(1);
            *fresh24 = (if c as libc::c_int & 0x1 as libc::c_int != 0 {
                '1' as i32
            } else {
                '0' as i32
            }) as libc::c_char;
            c = (c as libc::c_int >> 1 as libc::c_int) as libc::c_uchar;
            i += 1;
        }
        outlen -= 8 as libc::c_int as libc::c_long;
        in_0 = in_0.offset(1);
    };
}
unsafe extern "C" fn base2_decode_ctx_init(mut ctx: *mut base_decode_context) {
    init_inbuf(ctx);
    (*ctx).ctx.base2.octet = 0 as libc::c_int as libc::c_uchar;
    (*ctx).i = 0 as libc::c_int;
}
unsafe extern "C" fn base2lsbf_decode_ctx(
    mut ctx: *mut base_decode_context,
    mut in_0: *const libc::c_char,
    mut inlen: idx_t,
    mut out: *mut libc::c_char,
    mut outlen: *mut idx_t,
) -> bool {
    let mut ignore_lines: bool = 1 as libc::c_int != 0;
    *outlen = 0 as libc::c_int as idx_t;
    if inlen == 0 as libc::c_int as libc::c_long {
        return (*ctx).i == 0 as libc::c_int;
    }
    loop {
        let fresh25 = inlen;
        inlen = inlen - 1;
        if !(fresh25 != 0) {
            break;
        }
        if ignore_lines as libc::c_int != 0 && *in_0 as libc::c_int == '\n' as i32 {
            in_0 = in_0.offset(1);
        } else {
            if !isbase2(*in_0) {
                return 0 as libc::c_int != 0;
            }
            let mut bit: bool = *in_0 as libc::c_int == '1' as i32;
            (*ctx)
                .ctx
                .base2
                .octet = ((*ctx).ctx.base2.octet as libc::c_int
                | (bit as libc::c_int) << (*ctx).i) as libc::c_uchar;
            (*ctx).i += 1;
            if (*ctx).i == 8 as libc::c_int {
                let fresh26 = out;
                out = out.offset(1);
                *fresh26 = (*ctx).ctx.base2.octet as libc::c_char;
                (*ctx).ctx.base2.octet = 0 as libc::c_int as libc::c_uchar;
                *outlen += 1;
                (*ctx).i = 0 as libc::c_int;
            }
            in_0 = in_0.offset(1);
        }
    }
    return 1 as libc::c_int != 0;
}
unsafe extern "C" fn base2msbf_decode_ctx(
    mut ctx: *mut base_decode_context,
    mut in_0: *const libc::c_char,
    mut inlen: idx_t,
    mut out: *mut libc::c_char,
    mut outlen: *mut idx_t,
) -> bool {
    let mut ignore_lines: bool = 1 as libc::c_int != 0;
    *outlen = 0 as libc::c_int as idx_t;
    if inlen == 0 as libc::c_int as libc::c_long {
        return (*ctx).i == 0 as libc::c_int;
    }
    loop {
        let fresh27 = inlen;
        inlen = inlen - 1;
        if !(fresh27 != 0) {
            break;
        }
        if ignore_lines as libc::c_int != 0 && *in_0 as libc::c_int == '\n' as i32 {
            in_0 = in_0.offset(1);
        } else {
            if !isbase2(*in_0) {
                return 0 as libc::c_int != 0;
            }
            let mut bit: bool = *in_0 as libc::c_int == '1' as i32;
            if (*ctx).i == 0 as libc::c_int {
                (*ctx).i = 8 as libc::c_int;
            }
            (*ctx).i -= 1;
            (*ctx)
                .ctx
                .base2
                .octet = ((*ctx).ctx.base2.octet as libc::c_int
                | (bit as libc::c_int) << (*ctx).i) as libc::c_uchar;
            if (*ctx).i == 0 as libc::c_int {
                let fresh28 = out;
                out = out.offset(1);
                *fresh28 = (*ctx).ctx.base2.octet as libc::c_char;
                (*ctx).ctx.base2.octet = 0 as libc::c_int as libc::c_uchar;
                *outlen += 1;
                (*ctx).i = 0 as libc::c_int;
            }
            in_0 = in_0.offset(1);
        }
    }
    return 1 as libc::c_int != 0;
}
unsafe extern "C" fn wrap_write(
    mut buffer: *const libc::c_char,
    mut len: idx_t,
    mut wrap_column: idx_t,
    mut current_column: *mut idx_t,
    mut out: *mut FILE,
) {
    if wrap_column == 0 as libc::c_int as libc::c_long {
        if (if 0 != 0 && 0 != 0
            && (1 as libc::c_int as size_t).wrapping_mul(len as size_t)
                <= 8 as libc::c_int as libc::c_ulong
            && 1 as libc::c_int as size_t != 0 as libc::c_int as libc::c_ulong
        {
            ({
                let mut __ptr: *const libc::c_char = buffer;
                let mut __stream: *mut FILE = stdout;
                let mut __cnt: size_t = 0;
                __cnt = (1 as libc::c_int as size_t).wrapping_mul(len as size_t);
                while __cnt > 0 as libc::c_int as libc::c_ulong {
                    let fresh29 = __ptr;
                    __ptr = __ptr.offset(1);
                    if putc_unlocked(*fresh29 as libc::c_int, __stream)
                        == -(1 as libc::c_int)
                    {
                        break;
                    }
                    __cnt = __cnt.wrapping_sub(1);
                }
                (1 as libc::c_int as size_t)
                    .wrapping_mul(len as size_t)
                    .wrapping_sub(__cnt)
                    .wrapping_div(1 as libc::c_int as size_t)
            })
        } else {
            (if 0 != 0 && 1 as libc::c_int as size_t == 0 as libc::c_int as libc::c_ulong
                || 0 != 0 && len as size_t == 0 as libc::c_int as libc::c_ulong
            {
                0 as libc::c_int as size_t
            } else {
                fwrite_unlocked(
                    buffer as *const libc::c_void,
                    1 as libc::c_int as size_t,
                    len as size_t,
                    stdout,
                )
            })
        }) < len as libc::c_ulong
        {
            if ::core::mem::size_of::<C2RustUnnamed_5>() as libc::c_ulong != 0 {
                error(
                    1 as libc::c_int,
                    *__errno_location(),
                    dcgettext(
                        0 as *const libc::c_char,
                        b"write error\0" as *const u8 as *const libc::c_char,
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
                        b"write error\0" as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                );
                if 0 as libc::c_int != 0 {} else {
                    unreachable!();
                };
            };
        }
    } else {
        let mut written: idx_t = 0 as libc::c_int as idx_t;
        while written < len {
            let mut to_write: idx_t = if wrap_column - *current_column < len - written {
                wrap_column - *current_column
            } else {
                len - written
            };
            if to_write == 0 as libc::c_int as libc::c_long {
                if fputc_unlocked('\n' as i32, out) == -(1 as libc::c_int) {
                    if ::core::mem::size_of::<C2RustUnnamed_4>() as libc::c_ulong != 0 {
                        error(
                            1 as libc::c_int,
                            *__errno_location(),
                            dcgettext(
                                0 as *const libc::c_char,
                                b"write error\0" as *const u8 as *const libc::c_char,
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
                                b"write error\0" as *const u8 as *const libc::c_char,
                                5 as libc::c_int,
                            ),
                        );
                        if 0 as libc::c_int != 0 {} else {
                            unreachable!();
                        };
                    };
                }
                *current_column = 0 as libc::c_int as idx_t;
            } else {
                if (if 0 != 0 && 0 != 0
                    && (1 as libc::c_int as size_t).wrapping_mul(to_write as size_t)
                        <= 8 as libc::c_int as libc::c_ulong
                    && 1 as libc::c_int as size_t != 0 as libc::c_int as libc::c_ulong
                {
                    ({
                        let mut __ptr: *const libc::c_char = buffer
                            .offset(written as isize);
                        let mut __stream: *mut FILE = stdout;
                        let mut __cnt: size_t = 0;
                        __cnt = (1 as libc::c_int as size_t)
                            .wrapping_mul(to_write as size_t);
                        while __cnt > 0 as libc::c_int as libc::c_ulong {
                            let fresh30 = __ptr;
                            __ptr = __ptr.offset(1);
                            if putc_unlocked(*fresh30 as libc::c_int, __stream)
                                == -(1 as libc::c_int)
                            {
                                break;
                            }
                            __cnt = __cnt.wrapping_sub(1);
                        }
                        (1 as libc::c_int as size_t)
                            .wrapping_mul(to_write as size_t)
                            .wrapping_sub(__cnt)
                            .wrapping_div(1 as libc::c_int as size_t)
                    })
                } else {
                    (if 0 != 0
                        && 1 as libc::c_int as size_t
                            == 0 as libc::c_int as libc::c_ulong
                        || 0 != 0
                            && to_write as size_t == 0 as libc::c_int as libc::c_ulong
                    {
                        0 as libc::c_int as size_t
                    } else {
                        fwrite_unlocked(
                            buffer.offset(written as isize) as *const libc::c_void,
                            1 as libc::c_int as size_t,
                            to_write as size_t,
                            stdout,
                        )
                    })
                }) < to_write as libc::c_ulong
                {
                    if ::core::mem::size_of::<C2RustUnnamed_3>() as libc::c_ulong != 0 {
                        error(
                            1 as libc::c_int,
                            *__errno_location(),
                            dcgettext(
                                0 as *const libc::c_char,
                                b"write error\0" as *const u8 as *const libc::c_char,
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
                                b"write error\0" as *const u8 as *const libc::c_char,
                                5 as libc::c_int,
                            ),
                        );
                        if 0 as libc::c_int != 0 {} else {
                            unreachable!();
                        };
                    };
                }
                *current_column += to_write;
                written += to_write;
            }
        }
    };
}
unsafe extern "C" fn finish_and_exit(
    mut in_0: *mut FILE,
    mut infile: *const libc::c_char,
) {
    if rpl_fclose(in_0) != 0 as libc::c_int {
        if strcmp(infile, b"-\0" as *const u8 as *const libc::c_char) == 0 as libc::c_int
        {
            if ::core::mem::size_of::<C2RustUnnamed_7>() as libc::c_ulong != 0 {
                error(
                    1 as libc::c_int,
                    *__errno_location(),
                    dcgettext(
                        0 as *const libc::c_char,
                        b"closing standard input\0" as *const u8 as *const libc::c_char,
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
                        b"closing standard input\0" as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                );
                if 0 as libc::c_int != 0 {} else {
                    unreachable!();
                };
            };
        } else {
            if ::core::mem::size_of::<C2RustUnnamed_6>() as libc::c_ulong != 0 {
                error(
                    1 as libc::c_int,
                    *__errno_location(),
                    b"%s\0" as *const u8 as *const libc::c_char,
                    quotearg_n_style_colon(
                        0 as libc::c_int,
                        shell_escape_quoting_style,
                        infile,
                    ),
                );
                if 0 as libc::c_int != 0 {} else {
                    unreachable!();
                };
            } else {
                error(
                    1 as libc::c_int,
                    *__errno_location(),
                    b"%s\0" as *const u8 as *const libc::c_char,
                    quotearg_n_style_colon(
                        0 as libc::c_int,
                        shell_escape_quoting_style,
                        infile,
                    ),
                );
                if 0 as libc::c_int != 0 {} else {
                    unreachable!();
                };
            };
        }
    }
    exit(0 as libc::c_int);
}
unsafe extern "C" fn do_encode(
    mut in_0: *mut FILE,
    mut infile: *const libc::c_char,
    mut out: *mut FILE,
    mut wrap_column: idx_t,
) {
    let mut current_column: idx_t = 0 as libc::c_int as idx_t;
    let mut inbuf: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut outbuf: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut sum: idx_t = 0;
    inbuf = xmalloc(
        (1024 as libc::c_int * 3 as libc::c_int * 10 as libc::c_int) as size_t,
    ) as *mut libc::c_char;
    outbuf = xmalloc(
        base_length
            .expect(
                "non-null function pointer",
            )(1024 as libc::c_int * 3 as libc::c_int * 10 as libc::c_int) as size_t,
    ) as *mut libc::c_char;
    loop {
        let mut n: idx_t = 0;
        sum = 0 as libc::c_int as idx_t;
        loop {
            n = (if 0 != 0 && 0 != 0
                && (1 as libc::c_int as size_t)
                    .wrapping_mul(
                        ((1024 as libc::c_int * 3 as libc::c_int * 10 as libc::c_int)
                            as libc::c_long - sum) as size_t,
                    ) <= 8 as libc::c_int as libc::c_ulong
                && 1 as libc::c_int as size_t != 0 as libc::c_int as libc::c_ulong
            {
                ({
                    let mut __ptr: *mut libc::c_char = inbuf.offset(sum as isize);
                    let mut __stream: *mut FILE = in_0;
                    let mut __cnt: size_t = 0;
                    __cnt = (1 as libc::c_int as size_t)
                        .wrapping_mul(
                            ((1024 as libc::c_int * 3 as libc::c_int * 10 as libc::c_int)
                                as libc::c_long - sum) as size_t,
                        );
                    while __cnt > 0 as libc::c_int as libc::c_ulong {
                        let mut __c: libc::c_int = getc_unlocked(__stream);
                        if __c == -(1 as libc::c_int) {
                            break;
                        }
                        let fresh31 = __ptr;
                        __ptr = __ptr.offset(1);
                        *fresh31 = __c as libc::c_char;
                        __cnt = __cnt.wrapping_sub(1);
                    }
                    (1 as libc::c_int as size_t)
                        .wrapping_mul(
                            ((1024 as libc::c_int * 3 as libc::c_int * 10 as libc::c_int)
                                as libc::c_long - sum) as size_t,
                        )
                        .wrapping_sub(__cnt)
                        .wrapping_div(1 as libc::c_int as size_t)
                })
            } else if 0 != 0
                && 1 as libc::c_int as size_t == 0 as libc::c_int as libc::c_ulong
                || 0 != 0
                    && ((1024 as libc::c_int * 3 as libc::c_int * 10 as libc::c_int)
                        as libc::c_long - sum) as size_t
                        == 0 as libc::c_int as libc::c_ulong
            {
                0 as libc::c_int as size_t
            } else {
                fread_unlocked(
                    inbuf.offset(sum as isize) as *mut libc::c_void,
                    1 as libc::c_int as size_t,
                    ((1024 as libc::c_int * 3 as libc::c_int * 10 as libc::c_int)
                        as libc::c_long - sum) as size_t,
                    in_0,
                )
            }) as idx_t;
            sum += n;
            if !(feof_unlocked(in_0) == 0 && ferror_unlocked(in_0) == 0
                && sum
                    < (1024 as libc::c_int * 3 as libc::c_int * 10 as libc::c_int)
                        as libc::c_long)
            {
                break;
            }
        }
        if sum > 0 as libc::c_int as libc::c_long {
            base_encode
                .expect(
                    "non-null function pointer",
                )(
                inbuf,
                sum,
                outbuf,
                base_length.expect("non-null function pointer")(sum as libc::c_int)
                    as idx_t,
            );
            wrap_write(
                outbuf,
                base_length.expect("non-null function pointer")(sum as libc::c_int)
                    as idx_t,
                wrap_column,
                &mut current_column,
                out,
            );
        }
        if !(feof_unlocked(in_0) == 0 && ferror_unlocked(in_0) == 0
            && sum
                == (1024 as libc::c_int * 3 as libc::c_int * 10 as libc::c_int)
                    as libc::c_long)
        {
            break;
        }
    }
    if wrap_column != 0 && current_column > 0 as libc::c_int as libc::c_long
        && fputc_unlocked('\n' as i32, out) == -(1 as libc::c_int)
    {
        if ::core::mem::size_of::<C2RustUnnamed_9>() as libc::c_ulong != 0 {
            error(
                1 as libc::c_int,
                *__errno_location(),
                dcgettext(
                    0 as *const libc::c_char,
                    b"write error\0" as *const u8 as *const libc::c_char,
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
                    b"write error\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
            );
            if 0 as libc::c_int != 0 {} else {
                unreachable!();
            };
        };
    }
    if ferror_unlocked(in_0) != 0 {
        if ::core::mem::size_of::<C2RustUnnamed_8>() as libc::c_ulong != 0 {
            error(
                1 as libc::c_int,
                *__errno_location(),
                dcgettext(
                    0 as *const libc::c_char,
                    b"read error\0" as *const u8 as *const libc::c_char,
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
                    b"read error\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
            );
            if 0 as libc::c_int != 0 {} else {
                unreachable!();
            };
        };
    }
    finish_and_exit(in_0, infile);
}
unsafe extern "C" fn do_decode(
    mut in_0: *mut FILE,
    mut infile: *const libc::c_char,
    mut out: *mut FILE,
    mut ignore_garbage: bool,
) {
    let mut inbuf: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut outbuf: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut sum: idx_t = 0;
    let mut ctx: base_decode_context = base_decode_context {
        i: 0,
        ctx: C2RustUnnamed_1 {
            base64: base64_decode_context {
                i: 0,
                buf: [0; 4],
            },
        },
        inbuf: 0 as *mut libc::c_char,
        bufsize: 0,
    };
    inbuf = xmalloc(
        base_length.expect("non-null function pointer")(4200 as libc::c_int) as size_t,
    ) as *mut libc::c_char;
    outbuf = xmalloc(4200 as libc::c_int as size_t) as *mut libc::c_char;
    ctx.inbuf = 0 as *mut libc::c_char;
    base_decode_ctx_init.expect("non-null function pointer")(&mut ctx);
    loop {
        let mut ok: bool = false;
        sum = 0 as libc::c_int as idx_t;
        loop {
            let mut n: idx_t = (if 0 != 0 && 0 != 0
                && (1 as libc::c_int as size_t)
                    .wrapping_mul(
                        (base_length
                            .expect("non-null function pointer")(4200 as libc::c_int)
                            as libc::c_long - sum) as size_t,
                    ) <= 8 as libc::c_int as libc::c_ulong
                && 1 as libc::c_int as size_t != 0 as libc::c_int as libc::c_ulong
            {
                ({
                    let mut __ptr: *mut libc::c_char = inbuf.offset(sum as isize);
                    let mut __stream: *mut FILE = in_0;
                    let mut __cnt: size_t = 0;
                    __cnt = (1 as libc::c_int as size_t)
                        .wrapping_mul(
                            (base_length
                                .expect("non-null function pointer")(4200 as libc::c_int)
                                as libc::c_long - sum) as size_t,
                        );
                    while __cnt > 0 as libc::c_int as libc::c_ulong {
                        let mut __c: libc::c_int = getc_unlocked(__stream);
                        if __c == -(1 as libc::c_int) {
                            break;
                        }
                        let fresh32 = __ptr;
                        __ptr = __ptr.offset(1);
                        *fresh32 = __c as libc::c_char;
                        __cnt = __cnt.wrapping_sub(1);
                    }
                    (1 as libc::c_int as size_t)
                        .wrapping_mul(
                            (base_length
                                .expect("non-null function pointer")(4200 as libc::c_int)
                                as libc::c_long - sum) as size_t,
                        )
                        .wrapping_sub(__cnt)
                        .wrapping_div(1 as libc::c_int as size_t)
                })
            } else if 0 != 0
                && 1 as libc::c_int as size_t == 0 as libc::c_int as libc::c_ulong
                || 0 != 0
                    && (base_length
                        .expect("non-null function pointer")(4200 as libc::c_int)
                        as libc::c_long - sum) as size_t
                        == 0 as libc::c_int as libc::c_ulong
            {
                base_length.expect("non-null function pointer")(4200 as libc::c_int);
                0 as libc::c_int as size_t
            } else {
                fread_unlocked(
                    inbuf.offset(sum as isize) as *mut libc::c_void,
                    1 as libc::c_int as size_t,
                    (base_length.expect("non-null function pointer")(4200 as libc::c_int)
                        as libc::c_long - sum) as size_t,
                    in_0,
                )
            }) as idx_t;
            if ignore_garbage {
                let mut i: idx_t = 0 as libc::c_int as idx_t;
                while n > 0 as libc::c_int as libc::c_long && i < n {
                    if isbase
                        .expect(
                            "non-null function pointer",
                        )(*inbuf.offset((sum + i) as isize)) as libc::c_int != 0
                        || *inbuf.offset((sum + i) as isize) as libc::c_int == '=' as i32
                    {
                        i += 1;
                    } else {
                        n -= 1;
                        memmove(
                            inbuf.offset(sum as isize).offset(i as isize)
                                as *mut libc::c_void,
                            inbuf
                                .offset(sum as isize)
                                .offset(i as isize)
                                .offset(1 as libc::c_int as isize) as *const libc::c_void,
                            (n - i) as libc::c_ulong,
                        );
                    }
                }
            }
            sum += n;
            if ferror_unlocked(in_0) != 0 {
                if ::core::mem::size_of::<C2RustUnnamed_12>() as libc::c_ulong != 0 {
                    error(
                        1 as libc::c_int,
                        *__errno_location(),
                        dcgettext(
                            0 as *const libc::c_char,
                            b"read error\0" as *const u8 as *const libc::c_char,
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
                            b"read error\0" as *const u8 as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                    );
                    if 0 as libc::c_int != 0 {} else {
                        unreachable!();
                    };
                };
            }
            if !(sum
                < base_length.expect("non-null function pointer")(4200 as libc::c_int)
                    as libc::c_long && feof_unlocked(in_0) == 0)
            {
                break;
            }
        }
        let mut k: libc::c_int = 0 as libc::c_int;
        while k < 1 as libc::c_int + (feof_unlocked(in_0) != 0) as libc::c_int {
            if k == 1 as libc::c_int && ctx.i == 0 as libc::c_int {
                break;
            }
            let mut n_0: idx_t = 4200 as libc::c_int as idx_t;
            ok = base_decode_ctx
                .expect(
                    "non-null function pointer",
                )(
                &mut ctx,
                inbuf,
                if k == 0 as libc::c_int {
                    sum
                } else {
                    0 as libc::c_int as libc::c_long
                },
                outbuf,
                &mut n_0,
            );
            if (if 0 != 0 && 0 != 0
                && (1 as libc::c_int as size_t).wrapping_mul(n_0 as size_t)
                    <= 8 as libc::c_int as libc::c_ulong
                && 1 as libc::c_int as size_t != 0 as libc::c_int as libc::c_ulong
            {
                ({
                    let mut __ptr: *const libc::c_char = outbuf as *const libc::c_char;
                    let mut __stream: *mut FILE = out;
                    let mut __cnt: size_t = 0;
                    __cnt = (1 as libc::c_int as size_t).wrapping_mul(n_0 as size_t);
                    while __cnt > 0 as libc::c_int as libc::c_ulong {
                        let fresh33 = __ptr;
                        __ptr = __ptr.offset(1);
                        if putc_unlocked(*fresh33 as libc::c_int, __stream)
                            == -(1 as libc::c_int)
                        {
                            break;
                        }
                        __cnt = __cnt.wrapping_sub(1);
                    }
                    (1 as libc::c_int as size_t)
                        .wrapping_mul(n_0 as size_t)
                        .wrapping_sub(__cnt)
                        .wrapping_div(1 as libc::c_int as size_t)
                })
            } else {
                (if 0 != 0
                    && 1 as libc::c_int as size_t == 0 as libc::c_int as libc::c_ulong
                    || 0 != 0 && n_0 as size_t == 0 as libc::c_int as libc::c_ulong
                {
                    0 as libc::c_int as size_t
                } else {
                    fwrite_unlocked(
                        outbuf as *const libc::c_void,
                        1 as libc::c_int as size_t,
                        n_0 as size_t,
                        out,
                    )
                })
            }) < n_0 as libc::c_ulong
            {
                if ::core::mem::size_of::<C2RustUnnamed_11>() as libc::c_ulong != 0 {
                    error(
                        1 as libc::c_int,
                        *__errno_location(),
                        dcgettext(
                            0 as *const libc::c_char,
                            b"write error\0" as *const u8 as *const libc::c_char,
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
                            b"write error\0" as *const u8 as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                    );
                    if 0 as libc::c_int != 0 {} else {
                        unreachable!();
                    };
                };
            }
            if !ok {
                if ::core::mem::size_of::<C2RustUnnamed_10>() as libc::c_ulong != 0 {
                    error(
                        1 as libc::c_int,
                        0 as libc::c_int,
                        dcgettext(
                            0 as *const libc::c_char,
                            b"invalid input\0" as *const u8 as *const libc::c_char,
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
                            b"invalid input\0" as *const u8 as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                    );
                    if 0 as libc::c_int != 0 {} else {
                        unreachable!();
                    };
                };
            }
            k += 1;
        }
        if !(feof_unlocked(in_0) == 0) {
            break;
        }
    }
    finish_and_exit(in_0, infile);
}
unsafe fn main_0(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut opt: libc::c_int = 0;
    let mut input_fh: *mut FILE = 0 as *mut FILE;
    let mut infile: *const libc::c_char = 0 as *const libc::c_char;
    let mut decode: bool = 0 as libc::c_int != 0;
    let mut ignore_garbage: bool = 0 as libc::c_int != 0;
    let mut wrap_column: idx_t = 76 as libc::c_int as idx_t;
    let mut base_type: libc::c_int = 0 as libc::c_int;
    set_program_name(*argv.offset(0 as libc::c_int as isize));
    setlocale(6 as libc::c_int, b"\0" as *const u8 as *const libc::c_char);
    bindtextdomain(
        b"coreutils\0" as *const u8 as *const libc::c_char,
        b"/usr/local/share/locale\0" as *const u8 as *const libc::c_char,
    );
    textdomain(b"coreutils\0" as *const u8 as *const libc::c_char);
    atexit(Some(close_stdout as unsafe extern "C" fn() -> ()));
    loop {
        opt = getopt_long(
            argc,
            argv,
            b"diw:\0" as *const u8 as *const libc::c_char,
            long_options.as_ptr(),
            0 as *mut libc::c_int,
        );
        if !(opt != -(1 as libc::c_int)) {
            break;
        }
        match opt {
            100 => {
                decode = 1 as libc::c_int != 0;
            }
            119 => {
                let mut w: intmax_t = 0;
                let mut s_err: strtol_error = xstrtoimax(
                    optarg,
                    0 as *mut *mut libc::c_char,
                    10 as libc::c_int,
                    &mut w,
                    b"\0" as *const u8 as *const libc::c_char,
                );
                if (LONGINT_OVERFLOW as libc::c_int as libc::c_uint)
                    < s_err as libc::c_uint || w < 0 as libc::c_int as libc::c_long
                {
                    if ::core::mem::size_of::<C2RustUnnamed_14>() as libc::c_ulong != 0 {
                        error(
                            1 as libc::c_int,
                            0 as libc::c_int,
                            b"%s: %s\0" as *const u8 as *const libc::c_char,
                            dcgettext(
                                0 as *const libc::c_char,
                                b"invalid wrap size\0" as *const u8 as *const libc::c_char,
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
                            b"%s: %s\0" as *const u8 as *const libc::c_char,
                            dcgettext(
                                0 as *const libc::c_char,
                                b"invalid wrap size\0" as *const u8 as *const libc::c_char,
                                5 as libc::c_int,
                            ),
                            quote(optarg),
                        );
                        if 0 as libc::c_int != 0 {} else {
                            unreachable!();
                        };
                    };
                }
                wrap_column = if s_err as libc::c_uint
                    == LONGINT_OVERFLOW as libc::c_int as libc::c_uint
                    || (9223372036854775807 as libc::c_long) < w
                {
                    0 as libc::c_int as libc::c_long
                } else {
                    w
                };
            }
            105 => {
                ignore_garbage = 1 as libc::c_int != 0;
            }
            256 | 257 | 258 | 259 | 260 | 261 | 262 | 263 => {
                base_type = opt;
            }
            -2 => {
                usage(0 as libc::c_int);
            }
            -3 => {
                version_etc(
                    stdout,
                    b"basenc\0" as *const u8 as *const libc::c_char,
                    b"GNU coreutils\0" as *const u8 as *const libc::c_char,
                    Version,
                    b"Simon Josefsson\0" as *const u8 as *const libc::c_char,
                    b"Assaf Gordon\0" as *const u8 as *const libc::c_char,
                    0 as *mut libc::c_void as *mut libc::c_char,
                );
                exit(0 as libc::c_int);
            }
            _ => {
                usage(1 as libc::c_int);
            }
        }
    }
    match base_type {
        256 => {
            base_length = Some(
                base64_length_wrapper as unsafe extern "C" fn(libc::c_int) -> libc::c_int,
            );
            isbase = Some(isbase64 as unsafe extern "C" fn(libc::c_char) -> bool);
            base_encode = Some(
                base64_encode
                    as unsafe extern "C" fn(
                        *const libc::c_char,
                        idx_t,
                        *mut libc::c_char,
                        idx_t,
                    ) -> (),
            );
            base_decode_ctx_init = Some(
                base64_decode_ctx_init_wrapper
                    as unsafe extern "C" fn(*mut base_decode_context) -> (),
            );
            base_decode_ctx = Some(
                base64_decode_ctx_wrapper
                    as unsafe extern "C" fn(
                        *mut base_decode_context,
                        *const libc::c_char,
                        idx_t,
                        *mut libc::c_char,
                        *mut idx_t,
                    ) -> bool,
            );
        }
        257 => {
            base_length = Some(
                base64_length_wrapper as unsafe extern "C" fn(libc::c_int) -> libc::c_int,
            );
            isbase = Some(isbase64url as unsafe extern "C" fn(libc::c_char) -> bool);
            base_encode = Some(
                base64url_encode
                    as unsafe extern "C" fn(
                        *const libc::c_char,
                        idx_t,
                        *mut libc::c_char,
                        idx_t,
                    ) -> (),
            );
            base_decode_ctx_init = Some(
                base64url_decode_ctx_init_wrapper
                    as unsafe extern "C" fn(*mut base_decode_context) -> (),
            );
            base_decode_ctx = Some(
                base64url_decode_ctx_wrapper
                    as unsafe extern "C" fn(
                        *mut base_decode_context,
                        *const libc::c_char,
                        idx_t,
                        *mut libc::c_char,
                        *mut idx_t,
                    ) -> bool,
            );
        }
        258 => {
            base_length = Some(
                base32_length_wrapper as unsafe extern "C" fn(libc::c_int) -> libc::c_int,
            );
            isbase = Some(isbase32 as unsafe extern "C" fn(libc::c_char) -> bool);
            base_encode = Some(
                base32_encode
                    as unsafe extern "C" fn(
                        *const libc::c_char,
                        idx_t,
                        *mut libc::c_char,
                        idx_t,
                    ) -> (),
            );
            base_decode_ctx_init = Some(
                base32_decode_ctx_init_wrapper
                    as unsafe extern "C" fn(*mut base_decode_context) -> (),
            );
            base_decode_ctx = Some(
                base32_decode_ctx_wrapper
                    as unsafe extern "C" fn(
                        *mut base_decode_context,
                        *const libc::c_char,
                        idx_t,
                        *mut libc::c_char,
                        *mut idx_t,
                    ) -> bool,
            );
        }
        259 => {
            base_length = Some(
                base32_length_wrapper as unsafe extern "C" fn(libc::c_int) -> libc::c_int,
            );
            isbase = Some(isbase32hex as unsafe extern "C" fn(libc::c_char) -> bool);
            base_encode = Some(
                base32hex_encode
                    as unsafe extern "C" fn(
                        *const libc::c_char,
                        idx_t,
                        *mut libc::c_char,
                        idx_t,
                    ) -> (),
            );
            base_decode_ctx_init = Some(
                base32hex_decode_ctx_init_wrapper
                    as unsafe extern "C" fn(*mut base_decode_context) -> (),
            );
            base_decode_ctx = Some(
                base32hex_decode_ctx_wrapper
                    as unsafe extern "C" fn(
                        *mut base_decode_context,
                        *const libc::c_char,
                        idx_t,
                        *mut libc::c_char,
                        *mut idx_t,
                    ) -> bool,
            );
        }
        260 => {
            base_length = Some(
                base16_length as unsafe extern "C" fn(libc::c_int) -> libc::c_int,
            );
            isbase = Some(isbase16 as unsafe extern "C" fn(libc::c_char) -> bool);
            base_encode = Some(
                base16_encode
                    as unsafe extern "C" fn(
                        *const libc::c_char,
                        idx_t,
                        *mut libc::c_char,
                        idx_t,
                    ) -> (),
            );
            base_decode_ctx_init = Some(
                base16_decode_ctx_init
                    as unsafe extern "C" fn(*mut base_decode_context) -> (),
            );
            base_decode_ctx = Some(
                base16_decode_ctx
                    as unsafe extern "C" fn(
                        *mut base_decode_context,
                        *const libc::c_char,
                        idx_t,
                        *mut libc::c_char,
                        *mut idx_t,
                    ) -> bool,
            );
        }
        261 => {
            base_length = Some(
                base2_length as unsafe extern "C" fn(libc::c_int) -> libc::c_int,
            );
            isbase = Some(isbase2 as unsafe extern "C" fn(libc::c_char) -> bool);
            base_encode = Some(
                base2msbf_encode
                    as unsafe extern "C" fn(
                        *const libc::c_char,
                        idx_t,
                        *mut libc::c_char,
                        idx_t,
                    ) -> (),
            );
            base_decode_ctx_init = Some(
                base2_decode_ctx_init
                    as unsafe extern "C" fn(*mut base_decode_context) -> (),
            );
            base_decode_ctx = Some(
                base2msbf_decode_ctx
                    as unsafe extern "C" fn(
                        *mut base_decode_context,
                        *const libc::c_char,
                        idx_t,
                        *mut libc::c_char,
                        *mut idx_t,
                    ) -> bool,
            );
        }
        262 => {
            base_length = Some(
                base2_length as unsafe extern "C" fn(libc::c_int) -> libc::c_int,
            );
            isbase = Some(isbase2 as unsafe extern "C" fn(libc::c_char) -> bool);
            base_encode = Some(
                base2lsbf_encode
                    as unsafe extern "C" fn(
                        *const libc::c_char,
                        idx_t,
                        *mut libc::c_char,
                        idx_t,
                    ) -> (),
            );
            base_decode_ctx_init = Some(
                base2_decode_ctx_init
                    as unsafe extern "C" fn(*mut base_decode_context) -> (),
            );
            base_decode_ctx = Some(
                base2lsbf_decode_ctx
                    as unsafe extern "C" fn(
                        *mut base_decode_context,
                        *const libc::c_char,
                        idx_t,
                        *mut libc::c_char,
                        *mut idx_t,
                    ) -> bool,
            );
        }
        263 => {
            base_length = Some(
                z85_length as unsafe extern "C" fn(libc::c_int) -> libc::c_int,
            );
            isbase = Some(isz85 as unsafe extern "C" fn(libc::c_char) -> bool);
            base_encode = Some(
                z85_encode
                    as unsafe extern "C" fn(
                        *const libc::c_char,
                        idx_t,
                        *mut libc::c_char,
                        idx_t,
                    ) -> (),
            );
            base_decode_ctx_init = Some(
                z85_decode_ctx_init
                    as unsafe extern "C" fn(*mut base_decode_context) -> (),
            );
            base_decode_ctx = Some(
                z85_decode_ctx
                    as unsafe extern "C" fn(
                        *mut base_decode_context,
                        *const libc::c_char,
                        idx_t,
                        *mut libc::c_char,
                        *mut idx_t,
                    ) -> bool,
            );
        }
        _ => {
            error(
                0 as libc::c_int,
                0 as libc::c_int,
                dcgettext(
                    0 as *const libc::c_char,
                    b"missing encoding type\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
            );
            usage(1 as libc::c_int);
        }
    }
    if argc - optind > 1 as libc::c_int {
        error(
            0 as libc::c_int,
            0 as libc::c_int,
            dcgettext(
                0 as *const libc::c_char,
                b"extra operand %s\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            quote(*argv.offset((optind + 1 as libc::c_int) as isize)),
        );
        usage(1 as libc::c_int);
    }
    if optind < argc {
        infile = *argv.offset(optind as isize);
    } else {
        infile = b"-\0" as *const u8 as *const libc::c_char;
    }
    if strcmp(infile, b"-\0" as *const u8 as *const libc::c_char) == 0 as libc::c_int {
        xset_binary_mode(0 as libc::c_int, 0 as libc::c_int);
        input_fh = stdin;
    } else {
        input_fh = fopen(infile, b"rb\0" as *const u8 as *const libc::c_char);
        if input_fh.is_null() {
            if ::core::mem::size_of::<C2RustUnnamed_13>() as libc::c_ulong != 0 {
                error(
                    1 as libc::c_int,
                    *__errno_location(),
                    b"%s\0" as *const u8 as *const libc::c_char,
                    quotearg_n_style_colon(
                        0 as libc::c_int,
                        shell_escape_quoting_style,
                        infile,
                    ),
                );
                if 0 as libc::c_int != 0 {} else {
                    unreachable!();
                };
            } else {
                error(
                    1 as libc::c_int,
                    *__errno_location(),
                    b"%s\0" as *const u8 as *const libc::c_char,
                    quotearg_n_style_colon(
                        0 as libc::c_int,
                        shell_escape_quoting_style,
                        infile,
                    ),
                );
                if 0 as libc::c_int != 0 {} else {
                    unreachable!();
                };
            };
        }
    }
    fadvise(input_fh, FADVISE_SEQUENTIAL);
    if decode {
        do_decode(input_fh, infile, stdout, ignore_garbage);
    } else {
        do_encode(input_fh, infile, stdout, wrap_column);
    }
    return 0;
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
