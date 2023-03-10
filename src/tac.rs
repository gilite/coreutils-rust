#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![feature(extern_types, label_break_value)]
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type re_dfa_t;
    static mut stdout: *mut FILE;
    static mut stderr: *mut FILE;
    fn fflush_unlocked(__stream: *mut FILE) -> libc::c_int;
    fn fdopen(__fd: libc::c_int, __modes: *const libc::c_char) -> *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn fputs_unlocked(__s: *const libc::c_char, __stream: *mut FILE) -> libc::c_int;
    fn fwrite_unlocked(
        __ptr: *const libc::c_void,
        __size: size_t,
        __n: size_t,
        __stream: *mut FILE,
    ) -> size_t;
    fn __overflow(_: *mut FILE, _: libc::c_int) -> libc::c_int;
    fn clearerr_unlocked(__stream: *mut FILE);
    fn fileno(__stream: *mut FILE) -> libc::c_int;
    fn rpl_fseeko(fp: *mut FILE, offset: off_t, whence: libc::c_int) -> libc::c_int;
    static mut optarg: *mut libc::c_char;
    static mut optind: libc::c_int;
    fn getopt_long(
        ___argc: libc::c_int,
        ___argv: *const *mut libc::c_char,
        __shortopts: *const libc::c_char,
        __longopts: *const option,
        __longind: *mut libc::c_int,
    ) -> libc::c_int;
    fn lseek(__fd: libc::c_int, __offset: __off_t, __whence: libc::c_int) -> __off_t;
    fn close(__fd: libc::c_int) -> libc::c_int;
    fn isatty(__fd: libc::c_int) -> libc::c_int;
    fn unlink(__name: *const libc::c_char) -> libc::c_int;
    fn ftruncate(__fd: libc::c_int, __length: __off_t) -> libc::c_int;
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
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn free(_: *mut libc::c_void);
    fn __errno_location() -> *mut libc::c_int;
    fn atexit(__func: Option::<unsafe extern "C" fn() -> ()>) -> libc::c_int;
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
    fn xrealloc(p: *mut libc::c_void, s: size_t) -> *mut libc::c_void;
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
    fn exit(_: libc::c_int) -> !;
    fn getenv(__name: *const libc::c_char) -> *mut libc::c_char;
    fn re_compile_pattern(
        __pattern: *const libc::c_char,
        __length: size_t,
        __buffer: *mut re_pattern_buffer,
    ) -> *const libc::c_char;
    fn re_search(
        __buffer: *mut re_pattern_buffer,
        __String: *const libc::c_char,
        __length: regoff_t,
        __start: regoff_t,
        __range: regoff_t,
        __regs: *mut re_registers,
    ) -> regoff_t;
    fn error(
        __status: libc::c_int,
        __errnum: libc::c_int,
        __format: *const libc::c_char,
        _: ...
    );
    fn mfile_name_concat(
        dir: *const libc::c_char,
        base: *const libc::c_char,
        base_in_result: *mut *mut libc::c_char,
    ) -> *mut libc::c_char;
    fn safe_read(fd: libc::c_int, buf: *mut libc::c_void, count: size_t) -> size_t;
    fn mkstemp_safer(_: *mut libc::c_char) -> libc::c_int;
}
pub type size_t = libc::c_ulong;
pub type __uintmax_t = libc::c_ulong;
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
pub type off_t = __off_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct option {
    pub name: *const libc::c_char,
    pub has_arg: libc::c_int,
    pub flag: *mut libc::c_int,
    pub val: libc::c_int,
}
pub type uintmax_t = __uintmax_t;
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
pub type __re_size_t = libc::c_uint;
pub type __re_long_size_t = libc::c_ulong;
pub type reg_syntax_t = libc::c_ulong;
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct re_pattern_buffer {
    pub buffer: *mut re_dfa_t,
    pub allocated: __re_long_size_t,
    pub used: __re_long_size_t,
    pub syntax: reg_syntax_t,
    pub fastmap: *mut libc::c_char,
    pub translate: *mut libc::c_uchar,
    pub re_nsub: size_t,
    #[bitfield(name = "can_be_null", ty = "libc::c_uint", bits = "0..=0")]
    #[bitfield(name = "regs_allocated", ty = "libc::c_uint", bits = "1..=2")]
    #[bitfield(name = "fastmap_accurate", ty = "libc::c_uint", bits = "3..=3")]
    #[bitfield(name = "no_sub", ty = "libc::c_uint", bits = "4..=4")]
    #[bitfield(name = "not_bol", ty = "libc::c_uint", bits = "5..=5")]
    #[bitfield(name = "not_eol", ty = "libc::c_uint", bits = "6..=6")]
    #[bitfield(name = "newline_anchor", ty = "libc::c_uint", bits = "7..=7")]
    pub can_be_null_regs_allocated_fastmap_accurate_no_sub_not_bol_not_eol_newline_anchor: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 7],
}
pub type regoff_t = libc::c_int;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct re_registers {
    pub num_regs: __re_size_t,
    pub start: *mut regoff_t,
    pub end: *mut regoff_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_0 {
    pub _gl_dummy: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_1 {
    pub _gl_dummy: libc::c_int,
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
static mut separator: *const libc::c_char = 0 as *const libc::c_char;
static mut have_read_stdin: bool = 0 as libc::c_int != 0;
static mut separator_ends_record: bool = false;
static mut sentinel_length: size_t = 0;
static mut match_length: size_t = 0;
static mut G_buffer: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
static mut read_size: size_t = 0;
static mut G_buffer_size: size_t = 0;
static mut compiled_separator: re_pattern_buffer = re_pattern_buffer {
    buffer: 0 as *const re_dfa_t as *mut re_dfa_t,
    allocated: 0,
    used: 0,
    syntax: 0,
    fastmap: 0 as *const libc::c_char as *mut libc::c_char,
    translate: 0 as *const libc::c_uchar as *mut libc::c_uchar,
    re_nsub: 0,
    can_be_null_regs_allocated_fastmap_accurate_no_sub_not_bol_not_eol_newline_anchor: [0; 1],
    c2rust_padding: [0; 7],
};
static mut compiled_separator_fastmap: [libc::c_char; 256] = [0; 256];
static mut regs: re_registers = re_registers {
    num_regs: 0,
    start: 0 as *const regoff_t as *mut regoff_t,
    end: 0 as *const regoff_t as *mut regoff_t,
};
static mut longopts: [option; 6] = [
    {
        let mut init = option {
            name: b"before\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'b' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"regex\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'r' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"separator\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
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
        fputs_unlocked(
            dcgettext(
                0 as *const libc::c_char,
                b"Write each FILE to standard output, last line first.\n\0" as *const u8
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
                b"  -b, --before             attach the separator before instead of after\n  -r, --regex              interpret the separator as a regular expression\n  -s, --separator=STRING   use STRING as the separator instead of newline\n\0"
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
        emit_ancillary_info(b"tac\0" as *const u8 as *const libc::c_char);
    }
    exit(status);
}
unsafe extern "C" fn output(
    mut start: *const libc::c_char,
    mut past_end: *const libc::c_char,
) {
    static mut buffer: [libc::c_char; 8192] = [0; 8192];
    static mut bytes_in_buffer: size_t = 0 as libc::c_int as size_t;
    let mut bytes_to_add: size_t = past_end.offset_from(start) as libc::c_long as size_t;
    let mut bytes_available: size_t = (8192 as libc::c_int as libc::c_ulong)
        .wrapping_sub(bytes_in_buffer);
    if start.is_null() {
        if 0 != 0 && 0 != 0
            && (1 as libc::c_int as size_t).wrapping_mul(bytes_in_buffer)
                <= 8 as libc::c_int as libc::c_ulong
            && 1 as libc::c_int as size_t != 0 as libc::c_int as libc::c_ulong
        {
            ({
                let mut __ptr: *const libc::c_char = buffer.as_mut_ptr()
                    as *const libc::c_char;
                let mut __stream: *mut FILE = stdout;
                let mut __cnt: size_t = 0;
                __cnt = (1 as libc::c_int as size_t).wrapping_mul(bytes_in_buffer);
                while __cnt > 0 as libc::c_int as libc::c_ulong {
                    let fresh1 = __ptr;
                    __ptr = __ptr.offset(1);
                    if putc_unlocked(*fresh1 as libc::c_int, __stream)
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
                || 0 != 0 && bytes_in_buffer == 0 as libc::c_int as libc::c_ulong
            {} else {
                fwrite_unlocked(
                    buffer.as_mut_ptr() as *const libc::c_void,
                    1 as libc::c_int as size_t,
                    bytes_in_buffer,
                    stdout,
                );
            };
        };
        bytes_in_buffer = 0 as libc::c_int as size_t;
        return;
    }
    while bytes_to_add >= bytes_available {
        memcpy(
            buffer.as_mut_ptr().offset(bytes_in_buffer as isize) as *mut libc::c_void,
            start as *const libc::c_void,
            bytes_available,
        );
        bytes_to_add = (bytes_to_add as libc::c_ulong).wrapping_sub(bytes_available)
            as size_t as size_t;
        start = start.offset(bytes_available as isize);
        if 0 != 0 && 0 != 0
            && (1 as libc::c_int as size_t).wrapping_mul(8192 as libc::c_int as size_t)
                <= 8 as libc::c_int as libc::c_ulong
            && 1 as libc::c_int as size_t != 0 as libc::c_int as libc::c_ulong
        {
            ({
                let mut __ptr: *const libc::c_char = buffer.as_mut_ptr()
                    as *const libc::c_char;
                let mut __stream: *mut FILE = stdout;
                let mut __cnt: size_t = 0;
                __cnt = (1 as libc::c_int as size_t)
                    .wrapping_mul(8192 as libc::c_int as size_t);
                while __cnt > 0 as libc::c_int as libc::c_ulong {
                    let fresh2 = __ptr;
                    __ptr = __ptr.offset(1);
                    if putc_unlocked(*fresh2 as libc::c_int, __stream)
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
                    && 8192 as libc::c_int as size_t == 0 as libc::c_int as libc::c_ulong
            {} else {
                fwrite_unlocked(
                    buffer.as_mut_ptr() as *const libc::c_void,
                    1 as libc::c_int as size_t,
                    8192 as libc::c_int as size_t,
                    stdout,
                );
            };
        };
        bytes_in_buffer = 0 as libc::c_int as size_t;
        bytes_available = 8192 as libc::c_int as size_t;
    }
    memcpy(
        buffer.as_mut_ptr().offset(bytes_in_buffer as isize) as *mut libc::c_void,
        start as *const libc::c_void,
        bytes_to_add,
    );
    bytes_in_buffer = (bytes_in_buffer as libc::c_ulong).wrapping_add(bytes_to_add)
        as size_t as size_t;
}
unsafe extern "C" fn tac_seekable(
    mut input_fd: libc::c_int,
    mut file: *const libc::c_char,
    mut file_pos: off_t,
) -> bool {
    let mut match_start: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut past_end: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut saved_record_size: size_t = 0;
    let mut first_time: bool = 1 as libc::c_int != 0;
    let mut first_char: libc::c_char = *separator;
    let mut separator1: *const libc::c_char = separator
        .offset(1 as libc::c_int as isize);
    let mut match_length1: size_t = match_length
        .wrapping_sub(1 as libc::c_int as libc::c_ulong);
    let mut remainder: size_t = (file_pos as libc::c_ulong).wrapping_rem(read_size);
    if remainder != 0 as libc::c_int as libc::c_ulong {
        file_pos = (file_pos as libc::c_ulong).wrapping_sub(remainder) as off_t as off_t;
        if lseek(input_fd, file_pos, 0 as libc::c_int) < 0 as libc::c_int as libc::c_long
        {
            error(
                0 as libc::c_int,
                *__errno_location(),
                dcgettext(
                    0 as *const libc::c_char,
                    b"%s: seek failed\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
                quotearg_n_style_colon(
                    0 as libc::c_int,
                    shell_escape_quoting_style,
                    file,
                ),
            );
        }
    }
    loop {
        saved_record_size = safe_read(
            input_fd,
            G_buffer as *mut libc::c_void,
            read_size,
        );
        if !(saved_record_size == 0 as libc::c_int as libc::c_ulong
            && file_pos != 0 as libc::c_int as libc::c_long)
        {
            break;
        }
        let mut rsize: off_t = read_size as off_t;
        if lseek(input_fd, -rsize, 1 as libc::c_int) < 0 as libc::c_int as libc::c_long {
            error(
                0 as libc::c_int,
                *__errno_location(),
                dcgettext(
                    0 as *const libc::c_char,
                    b"%s: seek failed\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
                quotearg_n_style_colon(
                    0 as libc::c_int,
                    shell_escape_quoting_style,
                    file,
                ),
            );
        }
        file_pos = (file_pos as libc::c_ulong).wrapping_sub(read_size) as off_t as off_t;
    }
    while saved_record_size == read_size {
        let mut nread: size_t = safe_read(
            input_fd,
            G_buffer as *mut libc::c_void,
            read_size,
        );
        if nread == 0 as libc::c_int as libc::c_ulong {
            break;
        }
        saved_record_size = nread;
        if saved_record_size == -(1 as libc::c_int) as size_t {
            break;
        }
        file_pos = (file_pos as libc::c_ulong).wrapping_add(nread) as off_t as off_t;
    }
    if saved_record_size == -(1 as libc::c_int) as size_t {
        error(
            0 as libc::c_int,
            *__errno_location(),
            dcgettext(
                0 as *const libc::c_char,
                b"%s: read error\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            quotearg_n_style_colon(0 as libc::c_int, shell_escape_quoting_style, file),
        );
        return 0 as libc::c_int != 0;
    }
    past_end = G_buffer.offset(saved_record_size as isize);
    match_start = past_end;
    if sentinel_length != 0 {
        match_start = match_start.offset(-(match_length1 as isize));
    }
    loop {
        if sentinel_length == 0 as libc::c_int as libc::c_ulong {
            let mut i: size_t = match_start.offset_from(G_buffer) as libc::c_long
                as size_t;
            let mut ri: regoff_t = i as regoff_t;
            let mut range: regoff_t = 1 as libc::c_int - ri;
            let mut ret: regoff_t = 0;
            if (1 as libc::c_int) < range {
                if ::core::mem::size_of::<C2RustUnnamed_1>() as libc::c_ulong != 0 {
                    error(
                        1 as libc::c_int,
                        0 as libc::c_int,
                        dcgettext(
                            0 as *const libc::c_char,
                            b"record too large\0" as *const u8 as *const libc::c_char,
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
                            b"record too large\0" as *const u8 as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                    );
                    if 0 as libc::c_int != 0 {} else {
                        unreachable!();
                    };
                };
            }
            if range == 1 as libc::c_int
                || {
                    ret = re_search(
                        &mut compiled_separator,
                        G_buffer,
                        i as regoff_t,
                        i.wrapping_sub(1 as libc::c_int as libc::c_ulong) as regoff_t,
                        range,
                        &mut regs,
                    );
                    ret == -(1 as libc::c_int)
                }
            {
                match_start = G_buffer.offset(-(1 as libc::c_int as isize));
            } else if ret == -(2 as libc::c_int) {
                if ::core::mem::size_of::<C2RustUnnamed_0>() as libc::c_ulong != 0 {
                    error(
                        1 as libc::c_int,
                        0 as libc::c_int,
                        dcgettext(
                            0 as *const libc::c_char,
                            b"error in regular expression search\0" as *const u8
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
                            b"error in regular expression search\0" as *const u8
                                as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                    );
                    if 0 as libc::c_int != 0 {} else {
                        unreachable!();
                    };
                };
            } else {
                match_start = G_buffer
                    .offset(*(regs.start).offset(0 as libc::c_int as isize) as isize);
                match_length = (*(regs.end).offset(0 as libc::c_int as isize)
                    - *(regs.start).offset(0 as libc::c_int as isize)) as size_t;
            }
        } else {
            loop {
                match_start = match_start.offset(-1);
                if !(*match_start as libc::c_int != first_char as libc::c_int
                    || match_length1 != 0
                        && !(strncmp(
                            match_start.offset(1 as libc::c_int as isize),
                            separator1,
                            match_length1,
                        ) == 0 as libc::c_int))
                {
                    break;
                }
            }
        }
        if match_start < G_buffer {
            if file_pos == 0 as libc::c_int as libc::c_long {
                output(G_buffer, past_end);
                return 1 as libc::c_int != 0;
            }
            saved_record_size = past_end.offset_from(G_buffer) as libc::c_long as size_t;
            if saved_record_size > read_size {
                let mut newbuffer: *mut libc::c_char = 0 as *mut libc::c_char;
                let mut offset: size_t = if sentinel_length != 0 {
                    sentinel_length
                } else {
                    1 as libc::c_int as libc::c_ulong
                };
                let mut old_G_buffer_size: size_t = G_buffer_size;
                read_size = (read_size as libc::c_ulong)
                    .wrapping_mul(2 as libc::c_int as libc::c_ulong) as size_t as size_t;
                G_buffer_size = read_size
                    .wrapping_mul(2 as libc::c_int as libc::c_ulong)
                    .wrapping_add(sentinel_length)
                    .wrapping_add(2 as libc::c_int as libc::c_ulong);
                if G_buffer_size < old_G_buffer_size {
                    xalloc_die();
                }
                newbuffer = xrealloc(
                    G_buffer.offset(-(offset as isize)) as *mut libc::c_void,
                    G_buffer_size,
                ) as *mut libc::c_char;
                newbuffer = newbuffer.offset(offset as isize);
                G_buffer = newbuffer;
            }
            if file_pos as libc::c_ulong >= read_size {
                file_pos = (file_pos as libc::c_ulong).wrapping_sub(read_size) as off_t
                    as off_t;
            } else {
                read_size = file_pos as size_t;
                file_pos = 0 as libc::c_int as off_t;
            }
            if lseek(input_fd, file_pos, 0 as libc::c_int)
                < 0 as libc::c_int as libc::c_long
            {
                error(
                    0 as libc::c_int,
                    *__errno_location(),
                    dcgettext(
                        0 as *const libc::c_char,
                        b"%s: seek failed\0" as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    quotearg_n_style_colon(
                        0 as libc::c_int,
                        shell_escape_quoting_style,
                        file,
                    ),
                );
            }
            memmove(
                G_buffer.offset(read_size as isize) as *mut libc::c_void,
                G_buffer as *const libc::c_void,
                saved_record_size,
            );
            past_end = G_buffer
                .offset(read_size as isize)
                .offset(saved_record_size as isize);
            if sentinel_length != 0 {
                match_start = G_buffer.offset(read_size as isize);
            } else {
                match_start = past_end;
            }
            if safe_read(input_fd, G_buffer as *mut libc::c_void, read_size) != read_size
            {
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
                        file,
                    ),
                );
                return 0 as libc::c_int != 0;
            }
        } else {
            if separator_ends_record {
                let mut match_end: *mut libc::c_char = match_start
                    .offset(match_length as isize);
                if !first_time || match_end != past_end {
                    output(match_end, past_end);
                }
                past_end = match_end;
                first_time = 0 as libc::c_int != 0;
            } else {
                output(match_start, past_end);
                past_end = match_start;
            }
            if sentinel_length > 0 as libc::c_int as libc::c_ulong {
                match_start = match_start
                    .offset(
                        -(match_length.wrapping_sub(1 as libc::c_int as libc::c_ulong)
                            as isize),
                    );
            }
        }
    };
}
unsafe extern "C" fn record_or_unlink_tempfile(
    mut fn_0: *const libc::c_char,
    mut fp: *mut FILE,
) {
    unlink(fn_0);
}
unsafe extern "C" fn temp_stream(
    mut fp: *mut *mut FILE,
    mut file_name: *mut *mut libc::c_char,
) -> bool {
    static mut tempfile: *mut libc::c_char = 0 as *const libc::c_char
        as *mut libc::c_char;
    static mut tmp_fp: *mut FILE = 0 as *const FILE as *mut FILE;
    if tempfile.is_null() {
        's_73: {
            let mut t: *const libc::c_char = getenv(
                b"TMPDIR\0" as *const u8 as *const libc::c_char,
            );
            let mut tempdir: *const libc::c_char = if !t.is_null() {
                t
            } else {
                b"/tmp\0" as *const u8 as *const libc::c_char
            };
            tempfile = mfile_name_concat(
                tempdir,
                b"tacXXXXXX\0" as *const u8 as *const libc::c_char,
                0 as *mut *mut libc::c_char,
            );
            if tempdir.is_null() {
                error(
                    0 as libc::c_int,
                    0 as libc::c_int,
                    dcgettext(
                        0 as *const libc::c_char,
                        b"memory exhausted\0" as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                );
                return 0 as libc::c_int != 0;
            }
            let mut fd: libc::c_int = mkstemp_safer(tempfile);
            if fd < 0 as libc::c_int {
                error(
                    0 as libc::c_int,
                    *__errno_location(),
                    dcgettext(
                        0 as *const libc::c_char,
                        b"failed to create temporary file in %s\0" as *const u8
                            as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    quotearg_style(shell_escape_always_quoting_style, tempdir),
                );
            } else {
                tmp_fp = fdopen(
                    fd,
                    if 0 as libc::c_int != 0 {
                        b"w+b\0" as *const u8 as *const libc::c_char
                    } else {
                        b"w+\0" as *const u8 as *const libc::c_char
                    },
                );
                if tmp_fp.is_null() {
                    error(
                        0 as libc::c_int,
                        *__errno_location(),
                        dcgettext(
                            0 as *const libc::c_char,
                            b"failed to open %s for writing\0" as *const u8
                                as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                        quotearg_style(shell_escape_always_quoting_style, tempfile),
                    );
                    close(fd);
                    unlink(tempfile);
                } else {
                    record_or_unlink_tempfile(tempfile, tmp_fp);
                    break 's_73;
                }
            }
            free(tempfile as *mut libc::c_void);
            tempfile = 0 as *mut libc::c_char;
            return 0 as libc::c_int != 0;
        }
    } else {
        clearerr_unlocked(tmp_fp);
        if rpl_fseeko(tmp_fp, 0 as libc::c_int as off_t, 0 as libc::c_int)
            < 0 as libc::c_int
            || ftruncate(fileno(tmp_fp), 0 as libc::c_int as __off_t) < 0 as libc::c_int
        {
            error(
                0 as libc::c_int,
                *__errno_location(),
                dcgettext(
                    0 as *const libc::c_char,
                    b"failed to rewind stream for %s\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
                quotearg_style(shell_escape_always_quoting_style, tempfile),
            );
            return 0 as libc::c_int != 0;
        }
    }
    *fp = tmp_fp;
    *file_name = tempfile;
    return 1 as libc::c_int != 0;
}
unsafe extern "C" fn copy_to_temp(
    mut g_tmp: *mut *mut FILE,
    mut g_tempfile: *mut *mut libc::c_char,
    mut input_fd: libc::c_int,
    mut file: *const libc::c_char,
) -> off_t {
    let mut fp: *mut FILE = 0 as *mut FILE;
    let mut file_name: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut bytes_copied: uintmax_t = 0 as libc::c_int as uintmax_t;
    if !temp_stream(&mut fp, &mut file_name) {
        return -(1 as libc::c_int) as off_t;
    }
    loop {
        let mut bytes_read: size_t = safe_read(
            input_fd,
            G_buffer as *mut libc::c_void,
            read_size,
        );
        if bytes_read == 0 as libc::c_int as libc::c_ulong {
            break;
        }
        if bytes_read == -(1 as libc::c_int) as size_t {
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
                    file,
                ),
            );
            return -(1 as libc::c_int) as off_t;
        }
        if (if 0 != 0 && 0 != 0
            && (1 as libc::c_int as size_t).wrapping_mul(bytes_read)
                <= 8 as libc::c_int as libc::c_ulong
            && 1 as libc::c_int as size_t != 0 as libc::c_int as libc::c_ulong
        {
            ({
                let mut __ptr: *const libc::c_char = G_buffer as *const libc::c_char;
                let mut __stream: *mut FILE = fp;
                let mut __cnt: size_t = 0;
                __cnt = (1 as libc::c_int as size_t).wrapping_mul(bytes_read);
                while __cnt > 0 as libc::c_int as libc::c_ulong {
                    let fresh3 = __ptr;
                    __ptr = __ptr.offset(1);
                    if putc_unlocked(*fresh3 as libc::c_int, __stream)
                        == -(1 as libc::c_int)
                    {
                        break;
                    }
                    __cnt = __cnt.wrapping_sub(1);
                }
                (1 as libc::c_int as size_t)
                    .wrapping_mul(bytes_read)
                    .wrapping_sub(__cnt)
                    .wrapping_div(1 as libc::c_int as size_t)
            })
        } else {
            (if 0 != 0 && 1 as libc::c_int as size_t == 0 as libc::c_int as libc::c_ulong
                || 0 != 0 && bytes_read == 0 as libc::c_int as libc::c_ulong
            {
                0 as libc::c_int as size_t
            } else {
                fwrite_unlocked(
                    G_buffer as *const libc::c_void,
                    1 as libc::c_int as size_t,
                    bytes_read,
                    fp,
                )
            })
        }) != bytes_read
        {
            error(
                0 as libc::c_int,
                *__errno_location(),
                dcgettext(
                    0 as *const libc::c_char,
                    b"%s: write error\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
                quotearg_n_style_colon(
                    0 as libc::c_int,
                    shell_escape_quoting_style,
                    file_name,
                ),
            );
            return -(1 as libc::c_int) as off_t;
        }
        bytes_copied = (bytes_copied as libc::c_ulong).wrapping_add(bytes_read)
            as uintmax_t as uintmax_t;
    }
    if fflush_unlocked(fp) != 0 as libc::c_int {
        error(
            0 as libc::c_int,
            *__errno_location(),
            dcgettext(
                0 as *const libc::c_char,
                b"%s: write error\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            quotearg_n_style_colon(
                0 as libc::c_int,
                shell_escape_quoting_style,
                file_name,
            ),
        );
        return -(1 as libc::c_int) as off_t;
    }
    *g_tmp = fp;
    *g_tempfile = file_name;
    return bytes_copied as off_t;
}
unsafe extern "C" fn tac_nonseekable(
    mut input_fd: libc::c_int,
    mut file: *const libc::c_char,
) -> bool {
    let mut tmp_stream: *mut FILE = 0 as *mut FILE;
    let mut tmp_file: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut bytes_copied: off_t = copy_to_temp(
        &mut tmp_stream,
        &mut tmp_file,
        input_fd,
        file,
    );
    if bytes_copied < 0 as libc::c_int as libc::c_long {
        return 0 as libc::c_int != 0;
    }
    let mut ok: bool = tac_seekable(fileno(tmp_stream), tmp_file, bytes_copied);
    return ok;
}
unsafe extern "C" fn tac_file(mut filename: *const libc::c_char) -> bool {
    let mut ok: bool = false;
    let mut file_size: off_t = 0;
    let mut fd: libc::c_int = 0;
    let mut is_stdin: bool = strcmp(filename, b"-\0" as *const u8 as *const libc::c_char)
        == 0 as libc::c_int;
    if is_stdin {
        have_read_stdin = 1 as libc::c_int != 0;
        fd = 0 as libc::c_int;
        filename = dcgettext(
            0 as *const libc::c_char,
            b"standard input\0" as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        );
        xset_binary_mode(0 as libc::c_int, 0 as libc::c_int);
    } else {
        fd = open(filename, 0 as libc::c_int | 0 as libc::c_int);
        if fd < 0 as libc::c_int {
            error(
                0 as libc::c_int,
                *__errno_location(),
                dcgettext(
                    0 as *const libc::c_char,
                    b"failed to open %s for reading\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
                quotearg_style(shell_escape_always_quoting_style, filename),
            );
            return 0 as libc::c_int != 0;
        }
    }
    file_size = lseek(fd, 0 as libc::c_int as __off_t, 2 as libc::c_int);
    ok = if file_size < 0 as libc::c_int as libc::c_long || isatty(fd) != 0 {
        tac_nonseekable(fd, filename) as libc::c_int
    } else {
        tac_seekable(fd, filename, file_size) as libc::c_int
    } != 0;
    if !is_stdin && close(fd) != 0 as libc::c_int {
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
                filename,
            ),
        );
        ok = 0 as libc::c_int != 0;
    }
    return ok;
}
unsafe fn main_0(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut error_message: *const libc::c_char = 0 as *const libc::c_char;
    let mut optc: libc::c_int = 0;
    let mut ok: bool = false;
    let mut half_buffer_size: size_t = 0;
    static mut default_file_list: [*const libc::c_char; 2] = [
        b"-\0" as *const u8 as *const libc::c_char,
        0 as *const libc::c_char,
    ];
    let mut file: *const *const libc::c_char = 0 as *const *const libc::c_char;
    set_program_name(*argv.offset(0 as libc::c_int as isize));
    setlocale(6 as libc::c_int, b"\0" as *const u8 as *const libc::c_char);
    bindtextdomain(
        b"coreutils\0" as *const u8 as *const libc::c_char,
        b"/usr/local/share/locale\0" as *const u8 as *const libc::c_char,
    );
    textdomain(b"coreutils\0" as *const u8 as *const libc::c_char);
    atexit(Some(close_stdout as unsafe extern "C" fn() -> ()));
    separator = b"\n\0" as *const u8 as *const libc::c_char;
    sentinel_length = 1 as libc::c_int as size_t;
    separator_ends_record = 1 as libc::c_int != 0;
    loop {
        optc = getopt_long(
            argc,
            argv,
            b"brs:\0" as *const u8 as *const libc::c_char,
            longopts.as_ptr(),
            0 as *mut libc::c_int,
        );
        if !(optc != -(1 as libc::c_int)) {
            break;
        }
        match optc {
            98 => {
                separator_ends_record = 0 as libc::c_int != 0;
            }
            114 => {
                sentinel_length = 0 as libc::c_int as size_t;
            }
            115 => {
                separator = optarg;
            }
            -2 => {
                usage(0 as libc::c_int);
            }
            -3 => {
                version_etc(
                    stdout,
                    b"tac\0" as *const u8 as *const libc::c_char,
                    b"GNU coreutils\0" as *const u8 as *const libc::c_char,
                    Version,
                    b"Jay Lepreau\0" as *const u8 as *const libc::c_char,
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
    if sentinel_length == 0 as libc::c_int as libc::c_ulong {
        if *separator as libc::c_int == 0 as libc::c_int {
            if ::core::mem::size_of::<C2RustUnnamed_3>() as libc::c_ulong != 0 {
                error(
                    1 as libc::c_int,
                    0 as libc::c_int,
                    dcgettext(
                        0 as *const libc::c_char,
                        b"separator cannot be empty\0" as *const u8
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
                        b"separator cannot be empty\0" as *const u8
                            as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                );
                if 0 as libc::c_int != 0 {} else {
                    unreachable!();
                };
            };
        }
        compiled_separator.buffer = 0 as *mut re_dfa_t;
        compiled_separator.allocated = 0 as libc::c_int as __re_long_size_t;
        compiled_separator.fastmap = compiled_separator_fastmap.as_mut_ptr();
        compiled_separator.translate = 0 as *mut libc::c_uchar;
        error_message = re_compile_pattern(
            separator,
            strlen(separator),
            &mut compiled_separator,
        );
        if !error_message.is_null() {
            if ::core::mem::size_of::<C2RustUnnamed_2>() as libc::c_ulong != 0 {
                error(
                    1 as libc::c_int,
                    0 as libc::c_int,
                    b"%s\0" as *const u8 as *const libc::c_char,
                    error_message,
                );
                if 0 as libc::c_int != 0 {} else {
                    unreachable!();
                };
            } else {
                error(
                    1 as libc::c_int,
                    0 as libc::c_int,
                    b"%s\0" as *const u8 as *const libc::c_char,
                    error_message,
                );
                if 0 as libc::c_int != 0 {} else {
                    unreachable!();
                };
            };
        }
    } else {
        sentinel_length = if *separator as libc::c_int != 0 {
            strlen(separator)
        } else {
            1 as libc::c_int as libc::c_ulong
        };
        match_length = sentinel_length;
    }
    read_size = 8192 as libc::c_int as size_t;
    while sentinel_length >= read_size.wrapping_div(2 as libc::c_int as libc::c_ulong) {
        if (18446744073709551615 as libc::c_ulong)
            .wrapping_div(2 as libc::c_int as libc::c_ulong) < read_size
        {
            xalloc_die();
        }
        read_size = (read_size as libc::c_ulong)
            .wrapping_mul(2 as libc::c_int as libc::c_ulong) as size_t as size_t;
    }
    half_buffer_size = read_size
        .wrapping_add(sentinel_length)
        .wrapping_add(1 as libc::c_int as libc::c_ulong);
    G_buffer_size = (2 as libc::c_int as libc::c_ulong).wrapping_mul(half_buffer_size);
    if !(read_size < half_buffer_size && half_buffer_size < G_buffer_size) {
        xalloc_die();
    }
    G_buffer = xmalloc(G_buffer_size) as *mut libc::c_char;
    if sentinel_length != 0 {
        memcpy(
            G_buffer as *mut libc::c_void,
            separator as *const libc::c_void,
            sentinel_length.wrapping_add(1 as libc::c_int as libc::c_ulong),
        );
        G_buffer = G_buffer.offset(sentinel_length as isize);
    } else {
        G_buffer = G_buffer.offset(1);
    }
    file = if optind < argc {
        &mut *argv.offset(optind as isize) as *mut *mut libc::c_char
            as *const *const libc::c_char
    } else {
        default_file_list.as_ptr()
    };
    xset_binary_mode(1 as libc::c_int, 0 as libc::c_int);
    ok = 1 as libc::c_int != 0;
    let mut i: size_t = 0 as libc::c_int as size_t;
    while !(*file.offset(i as isize)).is_null() {
        ok = (ok as libc::c_int & tac_file(*file.offset(i as isize)) as libc::c_int)
            as bool;
        i = i.wrapping_add(1);
    }
    output(
        0 as *mut libc::c_void as *mut libc::c_char,
        0 as *mut libc::c_void as *mut libc::c_char,
    );
    if have_read_stdin as libc::c_int != 0 && close(0 as libc::c_int) < 0 as libc::c_int
    {
        error(
            0 as libc::c_int,
            *__errno_location(),
            b"-\0" as *const u8 as *const libc::c_char,
        );
        ok = 0 as libc::c_int != 0;
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
