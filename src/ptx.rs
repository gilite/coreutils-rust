#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![feature(extern_types)]
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type re_dfa_t;
    static mut optarg: *mut libc::c_char;
    static mut optind: libc::c_int;
    fn getopt_long(
        ___argc: libc::c_int,
        ___argv: *const *mut libc::c_char,
        __shortopts: *const libc::c_char,
        __longopts: *const option,
        __longind: *mut libc::c_int,
    ) -> libc::c_int;
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
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn stpcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn free(_: *mut libc::c_void);
    fn __errno_location() -> *mut libc::c_int;
    fn atexit(__func: Option::<unsafe extern "C" fn() -> ()>) -> libc::c_int;
    fn exit(_: libc::c_int) -> !;
    fn qsort(
        __base: *mut libc::c_void,
        __nmemb: size_t,
        __size: size_t,
        __compar: __compar_fn_t,
    );
    static mut Version: *const libc::c_char;
    fn quotearg_n_style_colon(
        n: libc::c_int,
        s: quoting_style,
        arg: *const libc::c_char,
    ) -> *mut libc::c_char;
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
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
    fn textdomain(__domainname: *const libc::c_char) -> *mut libc::c_char;
    fn bindtextdomain(
        __domainname: *const libc::c_char,
        __dirname: *const libc::c_char,
    ) -> *mut libc::c_char;
    fn xmalloc(s: size_t) -> *mut libc::c_void;
    fn x2nrealloc(p: *mut libc::c_void, pn: *mut size_t, s: size_t) -> *mut libc::c_void;
    fn xnmalloc(n: size_t, s: size_t) -> *mut libc::c_void;
    static mut stdin: *mut FILE;
    static mut stdout: *mut FILE;
    static mut stderr: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn __overflow(_: *mut FILE, _: libc::c_int) -> libc::c_int;
    fn fputs_unlocked(__s: *const libc::c_char, __stream: *mut FILE) -> libc::c_int;
    fn clearerr_unlocked(__stream: *mut FILE);
    fn close_stdout();
    fn version_etc(
        stream: *mut FILE,
        command_name: *const libc::c_char,
        package: *const libc::c_char,
        version: *const libc::c_char,
        _: ...
    );
    fn proper_name_utf8(
        name_ascii: *const libc::c_char,
        name_utf8: *const libc::c_char,
    ) -> *const libc::c_char;
    static mut program_name: *const libc::c_char;
    fn set_program_name(argv0: *const libc::c_char);
    fn error(
        __status: libc::c_int,
        __errnum: libc::c_int,
        __format: *const libc::c_char,
        _: ...
    );
    fn re_compile_pattern(
        __pattern: *const libc::c_char,
        __length: size_t,
        __buffer: *mut re_pattern_buffer,
    ) -> *const libc::c_char;
    fn re_compile_fastmap(__buffer: *mut re_pattern_buffer) -> libc::c_int;
    fn re_search(
        __buffer: *mut re_pattern_buffer,
        __String: *const libc::c_char,
        __length: regoff_t,
        __start: regoff_t,
        __range: regoff_t,
        __regs: *mut re_registers,
    ) -> regoff_t;
    fn re_match(
        __buffer: *mut re_pattern_buffer,
        __String: *const libc::c_char,
        __length: regoff_t,
        __start: regoff_t,
        __regs: *mut re_registers,
    ) -> regoff_t;
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
    fn fread_file(
        stream: *mut FILE,
        flags: libc::c_int,
        length: *mut size_t,
    ) -> *mut libc::c_char;
    fn read_file(
        filename: *const libc::c_char,
        flags: libc::c_int,
        length: *mut size_t,
    ) -> *mut libc::c_char;
    fn freopen_safer(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: *mut FILE,
    ) -> *mut FILE;
    fn xstrtoimax(
        _: *const libc::c_char,
        _: *mut *mut libc::c_char,
        _: libc::c_int,
        _: *mut intmax_t,
        _: *const libc::c_char,
    ) -> strtol_error;
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
pub type __intmax_t = libc::c_long;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type size_t = libc::c_ulong;
pub type ptrdiff_t = libc::c_long;
pub type __compar_fn_t = Option::<
    unsafe extern "C" fn(*const libc::c_void, *const libc::c_void) -> libc::c_int,
>;
pub type intmax_t = __intmax_t;
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
pub type argmatch_exit_fn = Option::<unsafe extern "C" fn() -> ()>;
pub type strtol_error = libc::c_uint;
pub const LONGINT_INVALID: strtol_error = 4;
pub const LONGINT_INVALID_SUFFIX_CHAR_WITH_OVERFLOW: strtol_error = 3;
pub const LONGINT_INVALID_SUFFIX_CHAR: strtol_error = 2;
pub const LONGINT_OVERFLOW: strtol_error = 1;
pub const LONGINT_OK: strtol_error = 0;
pub type Format = libc::c_uint;
pub const TEX_FORMAT: Format = 3;
pub const ROFF_FORMAT: Format = 2;
pub const DUMB_FORMAT: Format = 1;
pub const UNKNOWN_FORMAT: Format = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct regex_data {
    pub string: *const libc::c_char,
    pub pattern: re_pattern_buffer,
    pub fastmap: [libc::c_char; 256],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct BLOCK {
    pub start: *mut libc::c_char,
    pub end: *mut libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct WORD {
    pub start: *mut libc::c_char,
    pub size: ptrdiff_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct WORD_TABLE {
    pub start: *mut WORD,
    pub alloc: size_t,
    pub length: ptrdiff_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct OCCURS {
    pub key: WORD,
    pub left: ptrdiff_t,
    pub right: ptrdiff_t,
    pub reference: intmax_t,
    pub file_index: libc::c_int,
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
static mut gnu_extensions: bool = 1 as libc::c_int != 0;
static mut auto_reference: bool = 0 as libc::c_int != 0;
static mut input_reference: bool = 0 as libc::c_int != 0;
static mut right_reference: bool = 0 as libc::c_int != 0;
static mut line_width: ptrdiff_t = 72 as libc::c_int as ptrdiff_t;
static mut gap_size: ptrdiff_t = 3 as libc::c_int as ptrdiff_t;
static mut truncation_string: *const libc::c_char = b"/\0" as *const u8
    as *const libc::c_char;
static mut macro_name: *const libc::c_char = b"xx\0" as *const u8 as *const libc::c_char;
static mut output_format: Format = UNKNOWN_FORMAT;
static mut ignore_case: bool = 0 as libc::c_int != 0;
static mut break_file: *const libc::c_char = 0 as *const libc::c_char;
static mut only_file: *const libc::c_char = 0 as *const libc::c_char;
static mut ignore_file: *const libc::c_char = 0 as *const libc::c_char;
static mut context_regex: regex_data = regex_data {
    string: 0 as *const libc::c_char,
    pattern: re_pattern_buffer {
        buffer: 0 as *const re_dfa_t as *mut re_dfa_t,
        allocated: 0,
        used: 0,
        syntax: 0,
        fastmap: 0 as *const libc::c_char as *mut libc::c_char,
        translate: 0 as *const libc::c_uchar as *mut libc::c_uchar,
        re_nsub: 0,
        can_be_null_regs_allocated_fastmap_accurate_no_sub_not_bol_not_eol_newline_anchor: [0; 1],
        c2rust_padding: [0; 7],
    },
    fastmap: [0; 256],
};
static mut word_regex: regex_data = regex_data {
    string: 0 as *const libc::c_char,
    pattern: re_pattern_buffer {
        buffer: 0 as *const re_dfa_t as *mut re_dfa_t,
        allocated: 0,
        used: 0,
        syntax: 0,
        fastmap: 0 as *const libc::c_char as *mut libc::c_char,
        translate: 0 as *const libc::c_uchar as *mut libc::c_uchar,
        re_nsub: 0,
        can_be_null_regs_allocated_fastmap_accurate_no_sub_not_bol_not_eol_newline_anchor: [0; 1],
        c2rust_padding: [0; 7],
    },
    fastmap: [0; 256],
};
static mut folded_chars: [libc::c_uchar; 256] = [0; 256];
static mut context_regs: re_registers = re_registers {
    num_regs: 0,
    start: 0 as *const regoff_t as *mut regoff_t,
    end: 0 as *const regoff_t as *mut regoff_t,
};
static mut word_regs: re_registers = re_registers {
    num_regs: 0,
    start: 0 as *const regoff_t as *mut regoff_t,
    end: 0 as *const regoff_t as *mut regoff_t,
};
static mut word_fastmap: [libc::c_char; 256] = [0; 256];
static mut maximum_word_length: ptrdiff_t = 0;
static mut reference_max_width: ptrdiff_t = 0;
static mut ignore_table: WORD_TABLE = WORD_TABLE {
    start: 0 as *const WORD as *mut WORD,
    alloc: 0,
    length: 0,
};
static mut only_table: WORD_TABLE = WORD_TABLE {
    start: 0 as *const WORD as *mut WORD,
    alloc: 0,
    length: 0,
};
static mut number_input_files: libc::c_int = 0;
static mut total_line_count: intmax_t = 0;
static mut input_file_name: *mut *const libc::c_char = 0 as *const *const libc::c_char
    as *mut *const libc::c_char;
static mut file_line_count: *mut intmax_t = 0 as *const intmax_t as *mut intmax_t;
static mut text_buffers: *mut BLOCK = 0 as *const BLOCK as *mut BLOCK;
static mut occurs_table: [*mut OCCURS; 1] = [0 as *const OCCURS as *mut OCCURS; 1];
static mut occurs_alloc: [size_t; 1] = [0; 1];
static mut number_of_occurs: [ptrdiff_t; 1] = [0; 1];
static mut edited_flag: [libc::c_char; 256] = [0; 256];
static mut half_line_width: ptrdiff_t = 0;
static mut before_max_width: ptrdiff_t = 0;
static mut keyafter_max_width: ptrdiff_t = 0;
static mut truncation_string_length: ptrdiff_t = 0;
static mut tail: BLOCK = BLOCK {
    start: 0 as *const libc::c_char as *mut libc::c_char,
    end: 0 as *const libc::c_char as *mut libc::c_char,
};
static mut tail_truncation: bool = false;
static mut before: BLOCK = BLOCK {
    start: 0 as *const libc::c_char as *mut libc::c_char,
    end: 0 as *const libc::c_char as *mut libc::c_char,
};
static mut before_truncation: bool = false;
static mut keyafter: BLOCK = BLOCK {
    start: 0 as *const libc::c_char as *mut libc::c_char,
    end: 0 as *const libc::c_char as *mut libc::c_char,
};
static mut keyafter_truncation: bool = false;
static mut head: BLOCK = BLOCK {
    start: 0 as *const libc::c_char as *mut libc::c_char,
    end: 0 as *const libc::c_char as *mut libc::c_char,
};
static mut head_truncation: bool = false;
static mut reference: BLOCK = BLOCK {
    start: 0 as *const libc::c_char as *mut libc::c_char,
    end: 0 as *const libc::c_char as *mut libc::c_char,
};
unsafe extern "C" fn matcher_error() {
    if ::core::mem::size_of::<C2RustUnnamed_1>() as libc::c_ulong != 0 {
        error(
            1 as libc::c_int,
            *__errno_location(),
            dcgettext(
                0 as *const libc::c_char,
                b"error in regular expression matcher\0" as *const u8
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
            *__errno_location(),
            dcgettext(
                0 as *const libc::c_char,
                b"error in regular expression matcher\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
        if 0 as libc::c_int != 0 {} else {
            unreachable!();
        };
    };
}
unsafe extern "C" fn unescape_string(mut string: *mut libc::c_char) {
    let mut cursor: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut value: libc::c_int = 0;
    let mut length: libc::c_int = 0;
    cursor = string;
    while *string != 0 {
        if *string as libc::c_int == '\\' as i32 {
            string = string.offset(1);
            match *string as libc::c_int {
                120 => {
                    value = 0 as libc::c_int;
                    length = 0 as libc::c_int;
                    string = string.offset(1);
                    while length < 3 as libc::c_int
                        && *(*__ctype_b_loc())
                            .offset(to_uchar(*string) as libc::c_int as isize)
                            as libc::c_int
                            & _ISxdigit as libc::c_int as libc::c_ushort as libc::c_int
                            != 0
                    {
                        value = value * 16 as libc::c_int
                            + (if *string as libc::c_int >= 'a' as i32
                                && *string as libc::c_int <= 'f' as i32
                            {
                                *string as libc::c_int - 'a' as i32 + 10 as libc::c_int
                            } else {
                                (if *string as libc::c_int >= 'A' as i32
                                    && *string as libc::c_int <= 'F' as i32
                                {
                                    *string as libc::c_int - 'A' as i32 + 10 as libc::c_int
                                } else {
                                    *string as libc::c_int - '0' as i32
                                })
                            });
                        length += 1;
                        string = string.offset(1);
                    }
                    if length == 0 as libc::c_int {
                        let fresh1 = cursor;
                        cursor = cursor.offset(1);
                        *fresh1 = '\\' as i32 as libc::c_char;
                        let fresh2 = cursor;
                        cursor = cursor.offset(1);
                        *fresh2 = 'x' as i32 as libc::c_char;
                    } else {
                        let fresh3 = cursor;
                        cursor = cursor.offset(1);
                        *fresh3 = value as libc::c_char;
                    }
                }
                48 => {
                    value = 0 as libc::c_int;
                    length = 0 as libc::c_int;
                    string = string.offset(1);
                    while length < 3 as libc::c_int
                        && (*string as libc::c_int >= '0' as i32
                            && *string as libc::c_int <= '7' as i32)
                    {
                        value = value * 8 as libc::c_int
                            + (*string as libc::c_int - '0' as i32);
                        length += 1;
                        string = string.offset(1);
                    }
                    let fresh4 = cursor;
                    cursor = cursor.offset(1);
                    *fresh4 = value as libc::c_char;
                }
                97 => {
                    let fresh5 = cursor;
                    cursor = cursor.offset(1);
                    *fresh5 = '\u{7}' as i32 as libc::c_char;
                    string = string.offset(1);
                }
                98 => {
                    let fresh6 = cursor;
                    cursor = cursor.offset(1);
                    *fresh6 = '\u{8}' as i32 as libc::c_char;
                    string = string.offset(1);
                }
                99 => {
                    while *string != 0 {
                        string = string.offset(1);
                    }
                }
                102 => {
                    let fresh7 = cursor;
                    cursor = cursor.offset(1);
                    *fresh7 = '\u{c}' as i32 as libc::c_char;
                    string = string.offset(1);
                }
                110 => {
                    let fresh8 = cursor;
                    cursor = cursor.offset(1);
                    *fresh8 = '\n' as i32 as libc::c_char;
                    string = string.offset(1);
                }
                114 => {
                    let fresh9 = cursor;
                    cursor = cursor.offset(1);
                    *fresh9 = '\r' as i32 as libc::c_char;
                    string = string.offset(1);
                }
                116 => {
                    let fresh10 = cursor;
                    cursor = cursor.offset(1);
                    *fresh10 = '\t' as i32 as libc::c_char;
                    string = string.offset(1);
                }
                118 => {
                    let fresh11 = cursor;
                    cursor = cursor.offset(1);
                    *fresh11 = '\u{b}' as i32 as libc::c_char;
                    string = string.offset(1);
                }
                0 => {}
                _ => {
                    let fresh12 = cursor;
                    cursor = cursor.offset(1);
                    *fresh12 = '\\' as i32 as libc::c_char;
                    let fresh13 = string;
                    string = string.offset(1);
                    let fresh14 = cursor;
                    cursor = cursor.offset(1);
                    *fresh14 = *fresh13;
                }
            }
        } else {
            let fresh15 = string;
            string = string.offset(1);
            let fresh16 = cursor;
            cursor = cursor.offset(1);
            *fresh16 = *fresh15;
        }
    }
    *cursor = '\0' as i32 as libc::c_char;
}
unsafe extern "C" fn compile_regex(mut regex: *mut regex_data) {
    let mut pattern: *mut re_pattern_buffer = &mut (*regex).pattern;
    let mut string: *const libc::c_char = (*regex).string;
    let mut message: *const libc::c_char = 0 as *const libc::c_char;
    (*pattern).buffer = 0 as *mut re_dfa_t;
    (*pattern).allocated = 0 as libc::c_int as __re_long_size_t;
    (*pattern).fastmap = ((*regex).fastmap).as_mut_ptr();
    (*pattern)
        .translate = if ignore_case as libc::c_int != 0 {
        folded_chars.as_mut_ptr()
    } else {
        0 as *mut libc::c_uchar
    };
    message = re_compile_pattern(string, strlen(string), pattern);
    if !message.is_null() {
        if ::core::mem::size_of::<C2RustUnnamed_2>() as libc::c_ulong != 0 {
            error(
                1 as libc::c_int,
                0 as libc::c_int,
                dcgettext(
                    0 as *const libc::c_char,
                    b"%s (for regexp %s)\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
                message,
                quote(string),
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
                    b"%s (for regexp %s)\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
                message,
                quote(string),
            );
            if 0 as libc::c_int != 0 {} else {
                unreachable!();
            };
        };
    }
    re_compile_fastmap(pattern);
}
unsafe extern "C" fn initialize_regex() {
    let mut character: libc::c_int = 0;
    if ignore_case {
        character = 0 as libc::c_int;
        while character < 256 as libc::c_int {
            folded_chars[character
                as usize] = ({
                let mut __res: libc::c_int = 0;
                if ::core::mem::size_of::<libc::c_int>() as libc::c_ulong
                    > 1 as libc::c_int as libc::c_ulong
                {
                    if 0 != 0 {
                        let mut __c: libc::c_int = character;
                        __res = if __c < -(128 as libc::c_int)
                            || __c > 255 as libc::c_int
                        {
                            __c
                        } else {
                            *(*__ctype_toupper_loc()).offset(__c as isize)
                        };
                    } else {
                        __res = toupper(character);
                    }
                } else {
                    __res = *(*__ctype_toupper_loc()).offset(character as isize);
                }
                __res
            }) as libc::c_uchar;
            character += 1;
        }
    }
    if !(context_regex.string).is_null() {
        if *context_regex.string == 0 {
            context_regex.string = 0 as *const libc::c_char;
        }
    } else if gnu_extensions as libc::c_int != 0 && !input_reference {
        context_regex
            .string = b"[.?!][]\"')}]*\\($\\|\t\\|  \\)[ \t\n]*\0" as *const u8
            as *const libc::c_char;
    } else {
        context_regex.string = b"\n\0" as *const u8 as *const libc::c_char;
    }
    if !(context_regex.string).is_null() {
        compile_regex(&mut context_regex);
    }
    if !(word_regex.string).is_null() {
        compile_regex(&mut word_regex);
    } else if break_file.is_null() {
        if gnu_extensions {
            character = 0 as libc::c_int;
            while character < 256 as libc::c_int {
                word_fastmap[character
                    as usize] = (*(*__ctype_b_loc()).offset(character as isize)
                    as libc::c_int
                    & _ISalpha as libc::c_int as libc::c_ushort as libc::c_int != 0)
                    as libc::c_int as libc::c_char;
                character += 1;
            }
        } else {
            memset(
                word_fastmap.as_mut_ptr() as *mut libc::c_void,
                1 as libc::c_int,
                256 as libc::c_int as libc::c_ulong,
            );
            word_fastmap[' ' as i32 as usize] = 0 as libc::c_int as libc::c_char;
            word_fastmap['\t' as i32 as usize] = 0 as libc::c_int as libc::c_char;
            word_fastmap['\n' as i32 as usize] = 0 as libc::c_int as libc::c_char;
        }
    }
}
unsafe extern "C" fn swallow_file_in_memory(
    mut file_name: *const libc::c_char,
    mut block: *mut BLOCK,
) {
    let mut used_length: size_t = 0;
    let mut using_stdin: bool = file_name.is_null() || *file_name == 0
        || strcmp(file_name, b"-\0" as *const u8 as *const libc::c_char)
            == 0 as libc::c_int;
    if using_stdin {
        (*block).start = fread_file(stdin, 0 as libc::c_int, &mut used_length);
    } else {
        (*block).start = read_file(file_name, 0 as libc::c_int, &mut used_length);
    }
    if ((*block).start).is_null() {
        if ::core::mem::size_of::<C2RustUnnamed_3>() as libc::c_ulong != 0 {
            error(
                1 as libc::c_int,
                *__errno_location(),
                b"%s\0" as *const u8 as *const libc::c_char,
                quotearg_n_style_colon(
                    0 as libc::c_int,
                    shell_escape_quoting_style,
                    (if using_stdin as libc::c_int != 0 {
                        b"-\0" as *const u8 as *const libc::c_char
                    } else {
                        file_name
                    }),
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
                    (if using_stdin as libc::c_int != 0 {
                        b"-\0" as *const u8 as *const libc::c_char
                    } else {
                        file_name
                    }),
                ),
            );
            if 0 as libc::c_int != 0 {} else {
                unreachable!();
            };
        };
    }
    if using_stdin {
        clearerr_unlocked(stdin);
    }
    (*block).end = ((*block).start).offset(used_length as isize);
}
unsafe extern "C" fn compare_words(
    mut void_first: *const libc::c_void,
    mut void_second: *const libc::c_void,
) -> libc::c_int {
    let mut length: ptrdiff_t = 0;
    let mut counter: ptrdiff_t = 0;
    let mut value: libc::c_int = 0;
    length = if (*(void_first as *const WORD)).size
        < (*(void_second as *const WORD)).size
    {
        (*(void_first as *const WORD)).size
    } else {
        (*(void_second as *const WORD)).size
    };
    if ignore_case {
        counter = 0 as libc::c_int as ptrdiff_t;
        while counter < length {
            value = folded_chars[to_uchar(
                *((*(void_first as *const WORD)).start).offset(counter as isize),
            ) as usize] as libc::c_int
                - folded_chars[to_uchar(
                    *((*(void_second as *const WORD)).start).offset(counter as isize),
                ) as usize] as libc::c_int;
            if value != 0 as libc::c_int {
                return value;
            }
            counter += 1;
        }
    } else {
        counter = 0 as libc::c_int as ptrdiff_t;
        while counter < length {
            value = to_uchar(
                *((*(void_first as *const WORD)).start).offset(counter as isize),
            ) as libc::c_int
                - to_uchar(
                    *((*(void_second as *const WORD)).start).offset(counter as isize),
                ) as libc::c_int;
            if value != 0 as libc::c_int {
                return value;
            }
            counter += 1;
        }
    }
    return ((*(void_first as *const WORD)).size > (*(void_second as *const WORD)).size)
        as libc::c_int
        - ((*(void_first as *const WORD)).size < (*(void_second as *const WORD)).size)
            as libc::c_int;
}
unsafe extern "C" fn compare_occurs(
    mut void_first: *const libc::c_void,
    mut void_second: *const libc::c_void,
) -> libc::c_int {
    let mut value: libc::c_int = 0;
    value = compare_words(
        &(*(void_first as *const OCCURS)).key as *const WORD as *const libc::c_void,
        &(*(void_second as *const OCCURS)).key as *const WORD as *const libc::c_void,
    );
    return if value != 0 {
        value
    } else {
        ((*(void_first as *const OCCURS)).key.start
            > (*(void_second as *const OCCURS)).key.start) as libc::c_int
            - ((*(void_first as *const OCCURS)).key.start
                < (*(void_second as *const OCCURS)).key.start) as libc::c_int
    };
}
unsafe extern "C" fn search_table(
    mut word: *mut WORD,
    mut table: *mut WORD_TABLE,
) -> bool {
    let mut lowest: ptrdiff_t = 0;
    let mut highest: ptrdiff_t = 0;
    let mut middle: ptrdiff_t = 0;
    let mut value: libc::c_int = 0;
    lowest = 0 as libc::c_int as ptrdiff_t;
    highest = (*table).length - 1 as libc::c_int as libc::c_long;
    while lowest <= highest {
        middle = (lowest + highest) / 2 as libc::c_int as libc::c_long;
        value = compare_words(
            word as *const libc::c_void,
            ((*table).start).offset(middle as isize) as *const libc::c_void,
        );
        if value < 0 as libc::c_int {
            highest = middle - 1 as libc::c_int as libc::c_long;
        } else if value > 0 as libc::c_int {
            lowest = middle + 1 as libc::c_int as libc::c_long;
        } else {
            return 1 as libc::c_int != 0
        }
    }
    return 0 as libc::c_int != 0;
}
unsafe extern "C" fn sort_found_occurs() {
    if number_of_occurs[0 as libc::c_int as usize] != 0 {
        qsort(
            occurs_table[0 as libc::c_int as usize] as *mut libc::c_void,
            number_of_occurs[0 as libc::c_int as usize] as size_t,
            ::core::mem::size_of::<OCCURS>() as libc::c_ulong,
            Some(
                compare_occurs
                    as unsafe extern "C" fn(
                        *const libc::c_void,
                        *const libc::c_void,
                    ) -> libc::c_int,
            ),
        );
    }
}
unsafe extern "C" fn digest_break_file(mut file_name: *const libc::c_char) {
    let mut file_contents: BLOCK = BLOCK {
        start: 0 as *const libc::c_char as *mut libc::c_char,
        end: 0 as *const libc::c_char as *mut libc::c_char,
    };
    let mut cursor: *mut libc::c_char = 0 as *mut libc::c_char;
    swallow_file_in_memory(file_name, &mut file_contents);
    memset(
        word_fastmap.as_mut_ptr() as *mut libc::c_void,
        1 as libc::c_int,
        256 as libc::c_int as libc::c_ulong,
    );
    cursor = file_contents.start;
    while cursor < file_contents.end {
        word_fastmap[to_uchar(*cursor) as usize] = 0 as libc::c_int as libc::c_char;
        cursor = cursor.offset(1);
    }
    if !gnu_extensions {
        word_fastmap[' ' as i32 as usize] = 0 as libc::c_int as libc::c_char;
        word_fastmap['\t' as i32 as usize] = 0 as libc::c_int as libc::c_char;
        word_fastmap['\n' as i32 as usize] = 0 as libc::c_int as libc::c_char;
    }
    free(file_contents.start as *mut libc::c_void);
}
unsafe extern "C" fn digest_word_file(
    mut file_name: *const libc::c_char,
    mut table: *mut WORD_TABLE,
) {
    let mut file_contents: BLOCK = BLOCK {
        start: 0 as *const libc::c_char as *mut libc::c_char,
        end: 0 as *const libc::c_char as *mut libc::c_char,
    };
    let mut cursor: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut word_start: *mut libc::c_char = 0 as *mut libc::c_char;
    swallow_file_in_memory(file_name, &mut file_contents);
    (*table).start = 0 as *mut WORD;
    (*table).alloc = 0 as libc::c_int as size_t;
    (*table).length = 0 as libc::c_int as ptrdiff_t;
    cursor = file_contents.start;
    while cursor < file_contents.end {
        word_start = cursor;
        while cursor < file_contents.end && *cursor as libc::c_int != '\n' as i32 {
            cursor = cursor.offset(1);
        }
        if cursor > word_start {
            if (*table).length as libc::c_ulong == (*table).alloc {
                (*table)
                    .start = x2nrealloc(
                    (*table).start as *mut libc::c_void,
                    &mut (*table).alloc,
                    ::core::mem::size_of::<WORD>() as libc::c_ulong,
                ) as *mut WORD;
            }
            let ref mut fresh17 = (*((*table).start).offset((*table).length as isize))
                .start;
            *fresh17 = word_start;
            (*((*table).start).offset((*table).length as isize))
                .size = cursor.offset_from(word_start) as libc::c_long;
            (*table).length += 1;
        }
        if cursor < file_contents.end {
            cursor = cursor.offset(1);
        }
    }
    qsort(
        (*table).start as *mut libc::c_void,
        (*table).length as size_t,
        ::core::mem::size_of::<WORD>() as libc::c_ulong,
        Some(
            compare_words
                as unsafe extern "C" fn(
                    *const libc::c_void,
                    *const libc::c_void,
                ) -> libc::c_int,
        ),
    );
}
unsafe extern "C" fn find_occurs_in_text(mut file_index: libc::c_int) {
    let mut cursor: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut scan: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut line_start: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut line_scan: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut reference_length: ptrdiff_t = 0;
    let mut possible_key: WORD = WORD {
        start: 0 as *mut libc::c_char,
        size: 0,
    };
    let mut occurs_cursor: *mut OCCURS = 0 as *mut OCCURS;
    let mut context_start: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut context_end: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut word_start: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut word_end: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut next_context_start: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut text_buffer: *const BLOCK = &mut *text_buffers.offset(file_index as isize)
        as *mut BLOCK;
    reference_length = 0 as libc::c_int as ptrdiff_t;
    line_start = (*text_buffer).start;
    line_scan = line_start;
    if input_reference {
        while line_scan < (*text_buffer).end
            && *(*__ctype_b_loc()).offset(to_uchar(*line_scan) as libc::c_int as isize)
                as libc::c_int & _ISspace as libc::c_int as libc::c_ushort as libc::c_int
                == 0
        {
            line_scan = line_scan.offset(1);
        }
        reference_length = line_scan.offset_from(line_start) as libc::c_long;
        while line_scan < (*text_buffer).end
            && *(*__ctype_b_loc()).offset(to_uchar(*line_scan) as libc::c_int as isize)
                as libc::c_int & _ISspace as libc::c_int as libc::c_ushort as libc::c_int
                != 0
        {
            line_scan = line_scan.offset(1);
        }
    }
    cursor = (*text_buffer).start;
    while cursor < (*text_buffer).end {
        context_start = cursor;
        next_context_start = (*text_buffer).end;
        if !(context_regex.string).is_null() {
            let mut current_block_15: u64;
            match re_search(
                &mut context_regex.pattern,
                cursor,
                ((*text_buffer).end).offset_from(cursor) as libc::c_long as regoff_t,
                0 as libc::c_int,
                ((*text_buffer).end).offset_from(cursor) as libc::c_long as regoff_t,
                &mut context_regs,
            ) {
                -2 => {
                    matcher_error();
                    current_block_15 = 224731115979188411;
                }
                -1 => {
                    current_block_15 = 224731115979188411;
                }
                0 => {
                    if ::core::mem::size_of::<C2RustUnnamed_4>() as libc::c_ulong != 0 {
                        error(
                            1 as libc::c_int,
                            0 as libc::c_int,
                            dcgettext(
                                0 as *const libc::c_char,
                                b"error: regular expression has a match of length zero: %s\0"
                                    as *const u8 as *const libc::c_char,
                                5 as libc::c_int,
                            ),
                            quote(context_regex.string),
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
                                b"error: regular expression has a match of length zero: %s\0"
                                    as *const u8 as *const libc::c_char,
                                5 as libc::c_int,
                            ),
                            quote(context_regex.string),
                        );
                        if 0 as libc::c_int != 0 {} else {
                            unreachable!();
                        };
                    };
                    current_block_15 = 3209275802459515391;
                }
                _ => {
                    current_block_15 = 3209275802459515391;
                }
            }
            match current_block_15 {
                3209275802459515391 => {
                    next_context_start = cursor
                        .offset(
                            *(context_regs.end).offset(0 as libc::c_int as isize)
                                as isize,
                        );
                }
                _ => {}
            }
        }
        context_end = next_context_start;
        while context_end > context_start
            && *(*__ctype_b_loc())
                .offset(
                    to_uchar(*context_end.offset(-(1 as libc::c_int) as isize))
                        as libc::c_int as isize,
                ) as libc::c_int
                & _ISspace as libc::c_int as libc::c_ushort as libc::c_int != 0
        {
            context_end = context_end.offset(-1);
        }
        loop {
            if !(word_regex.string).is_null() {
                let mut r: regoff_t = re_search(
                    &mut word_regex.pattern,
                    cursor,
                    context_end.offset_from(cursor) as libc::c_long as regoff_t,
                    0 as libc::c_int,
                    context_end.offset_from(cursor) as libc::c_long as regoff_t,
                    &mut word_regs,
                );
                if r == -(2 as libc::c_int) {
                    matcher_error();
                }
                if r == -(1 as libc::c_int) {
                    break;
                }
                word_start = cursor
                    .offset(
                        *(word_regs.start).offset(0 as libc::c_int as isize) as isize,
                    );
                word_end = cursor
                    .offset(*(word_regs.end).offset(0 as libc::c_int as isize) as isize);
            } else {
                scan = cursor;
                while scan < context_end && word_fastmap[to_uchar(*scan) as usize] == 0 {
                    scan = scan.offset(1);
                }
                if scan == context_end {
                    break;
                }
                word_start = scan;
                while scan < context_end
                    && word_fastmap[to_uchar(*scan) as usize] as libc::c_int != 0
                {
                    scan = scan.offset(1);
                }
                word_end = scan;
            }
            cursor = word_start;
            if word_end == word_start {
                cursor = cursor.offset(1);
            } else {
                possible_key.start = cursor;
                possible_key.size = word_end.offset_from(word_start) as libc::c_long;
                cursor = cursor.offset(possible_key.size as isize);
                if possible_key.size > maximum_word_length {
                    maximum_word_length = possible_key.size;
                }
                if input_reference {
                    while line_scan < possible_key.start {
                        if *line_scan as libc::c_int == '\n' as i32 {
                            total_line_count += 1;
                            line_scan = line_scan.offset(1);
                            line_start = line_scan;
                            while line_scan < (*text_buffer).end
                                && *(*__ctype_b_loc())
                                    .offset(to_uchar(*line_scan) as libc::c_int as isize)
                                    as libc::c_int
                                    & _ISspace as libc::c_int as libc::c_ushort as libc::c_int
                                    == 0
                            {
                                line_scan = line_scan.offset(1);
                            }
                            reference_length = line_scan.offset_from(line_start)
                                as libc::c_long;
                        } else {
                            line_scan = line_scan.offset(1);
                        }
                    }
                    if line_scan > possible_key.start {
                        continue;
                    }
                }
                if !ignore_file.is_null()
                    && search_table(&mut possible_key, &mut ignore_table) as libc::c_int
                        != 0
                {
                    continue;
                }
                if !only_file.is_null()
                    && !search_table(&mut possible_key, &mut only_table)
                {
                    continue;
                }
                if number_of_occurs[0 as libc::c_int as usize] as libc::c_ulong
                    == occurs_alloc[0 as libc::c_int as usize]
                {
                    occurs_table[0 as libc::c_int
                        as usize] = x2nrealloc(
                        occurs_table[0 as libc::c_int as usize] as *mut libc::c_void,
                        &mut *occurs_alloc
                            .as_mut_ptr()
                            .offset(0 as libc::c_int as isize),
                        ::core::mem::size_of::<OCCURS>() as libc::c_ulong,
                    ) as *mut OCCURS;
                }
                occurs_cursor = (occurs_table[0 as libc::c_int as usize])
                    .offset(number_of_occurs[0 as libc::c_int as usize] as isize);
                if auto_reference {
                    while line_scan < possible_key.start {
                        if *line_scan as libc::c_int == '\n' as i32 {
                            total_line_count += 1;
                            line_scan = line_scan.offset(1);
                            line_start = line_scan;
                            while line_scan < (*text_buffer).end
                                && *(*__ctype_b_loc())
                                    .offset(to_uchar(*line_scan) as libc::c_int as isize)
                                    as libc::c_int
                                    & _ISspace as libc::c_int as libc::c_ushort as libc::c_int
                                    == 0
                            {
                                line_scan = line_scan.offset(1);
                            }
                        } else {
                            line_scan = line_scan.offset(1);
                        }
                    }
                    (*occurs_cursor).reference = total_line_count;
                } else if input_reference {
                    (*occurs_cursor)
                        .reference = line_start.offset_from(possible_key.start)
                        as libc::c_long;
                    if reference_length > reference_max_width {
                        reference_max_width = reference_length;
                    }
                }
                if input_reference as libc::c_int != 0 && line_start == context_start {
                    while context_start < context_end
                        && *(*__ctype_b_loc())
                            .offset(to_uchar(*context_start) as libc::c_int as isize)
                            as libc::c_int
                            & _ISspace as libc::c_int as libc::c_ushort as libc::c_int
                            == 0
                    {
                        context_start = context_start.offset(1);
                    }
                    while context_start < context_end
                        && *(*__ctype_b_loc())
                            .offset(to_uchar(*context_start) as libc::c_int as isize)
                            as libc::c_int
                            & _ISspace as libc::c_int as libc::c_ushort as libc::c_int
                            != 0
                    {
                        context_start = context_start.offset(1);
                    }
                }
                (*occurs_cursor).key = possible_key;
                (*occurs_cursor)
                    .left = context_start.offset_from(possible_key.start)
                    as libc::c_long;
                (*occurs_cursor)
                    .right = context_end.offset_from(possible_key.start) as libc::c_long;
                (*occurs_cursor).file_index = file_index;
                number_of_occurs[0 as libc::c_int as usize] += 1;
            }
        }
        cursor = next_context_start;
    }
}
unsafe extern "C" fn print_spaces(mut number: ptrdiff_t) {
    let mut counter: ptrdiff_t = number;
    while counter > 0 as libc::c_int as libc::c_long {
        putchar_unlocked(' ' as i32);
        counter -= 1;
    }
}
unsafe extern "C" fn print_field(mut field: BLOCK) {
    let mut cursor: *mut libc::c_char = 0 as *mut libc::c_char;
    cursor = field.start;
    while cursor < field.end {
        let mut character: libc::c_uchar = *cursor as libc::c_uchar;
        if edited_flag[character as usize] != 0 {
            match character as libc::c_int {
                34 => {
                    putchar_unlocked('"' as i32);
                    putchar_unlocked('"' as i32);
                }
                36 | 37 | 38 | 35 | 95 => {
                    putchar_unlocked('\\' as i32);
                    putchar_unlocked(character as libc::c_int);
                }
                123 | 125 => {
                    printf(
                        b"$\\%c$\0" as *const u8 as *const libc::c_char,
                        character as libc::c_int,
                    );
                }
                92 => {
                    fputs_unlocked(
                        b"\\backslash{}\0" as *const u8 as *const libc::c_char,
                        stdout,
                    );
                }
                _ => {
                    putchar_unlocked(' ' as i32);
                }
            }
        } else {
            putchar_unlocked(*cursor as libc::c_int);
        }
        cursor = cursor.offset(1);
    }
}
unsafe extern "C" fn fix_output_parameters() {
    let mut file_index: size_t = 0;
    let mut line_ordinal: intmax_t = 0;
    let mut reference_width: ptrdiff_t = 0;
    let mut character: libc::c_int = 0;
    let mut cursor: *const libc::c_char = 0 as *const libc::c_char;
    if auto_reference {
        reference_max_width = 0 as libc::c_int as ptrdiff_t;
        file_index = 0 as libc::c_int as size_t;
        while file_index < number_input_files as libc::c_ulong {
            line_ordinal = *file_line_count.offset(file_index as isize)
                + 1 as libc::c_int as libc::c_long;
            if file_index > 0 as libc::c_int as libc::c_ulong {
                line_ordinal
                    -= *file_line_count
                        .offset(
                            file_index.wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                as isize,
                        );
            }
            let mut ordinal_string: [libc::c_char; 21] = [0; 21];
            reference_width = sprintf(
                ordinal_string.as_mut_ptr(),
                b"%ld\0" as *const u8 as *const libc::c_char,
                line_ordinal,
            ) as ptrdiff_t;
            if !(*input_file_name.offset(file_index as isize)).is_null() {
                reference_width = (reference_width as libc::c_ulong)
                    .wrapping_add(strlen(*input_file_name.offset(file_index as isize)))
                    as ptrdiff_t as ptrdiff_t;
            }
            if reference_width > reference_max_width {
                reference_max_width = reference_width;
            }
            file_index = file_index.wrapping_add(1);
        }
        reference_max_width += 1;
        reference
            .start = xmalloc(
            (reference_max_width + 1 as libc::c_int as libc::c_long) as size_t,
        ) as *mut libc::c_char;
    }
    if (auto_reference as libc::c_int != 0 || input_reference as libc::c_int != 0)
        && !right_reference
    {
        line_width -= reference_max_width + gap_size;
    }
    if line_width < 0 as libc::c_int as libc::c_long {
        line_width = 0 as libc::c_int as ptrdiff_t;
    }
    half_line_width = line_width / 2 as libc::c_int as libc::c_long;
    before_max_width = half_line_width - gap_size;
    keyafter_max_width = half_line_width;
    if !truncation_string.is_null() && *truncation_string as libc::c_int != 0 {
        truncation_string_length = strlen(truncation_string) as ptrdiff_t;
    } else {
        truncation_string = 0 as *const libc::c_char;
    }
    if gnu_extensions {
        before_max_width -= 2 as libc::c_int as libc::c_long * truncation_string_length;
        if before_max_width < 0 as libc::c_int as libc::c_long {
            before_max_width = 0 as libc::c_int as ptrdiff_t;
        }
        keyafter_max_width
            -= 2 as libc::c_int as libc::c_long * truncation_string_length;
    } else {
        keyafter_max_width
            -= 2 as libc::c_int as libc::c_long * truncation_string_length
                + 1 as libc::c_int as libc::c_long;
    }
    character = 0 as libc::c_int;
    while character < 256 as libc::c_int {
        edited_flag[character
            as usize] = (*(*__ctype_b_loc()).offset(character as isize) as libc::c_int
            & _ISspace as libc::c_int as libc::c_ushort as libc::c_int != 0)
            as libc::c_int as libc::c_char;
        character += 1;
    }
    edited_flag['\u{c}' as i32 as usize] = 1 as libc::c_int as libc::c_char;
    match output_format as libc::c_uint {
        2 => {
            edited_flag['"' as i32 as usize] = 1 as libc::c_int as libc::c_char;
        }
        3 => {
            cursor = b"$%&#_{}\\\0" as *const u8 as *const libc::c_char;
            while *cursor != 0 {
                edited_flag[to_uchar(*cursor)
                    as usize] = 1 as libc::c_int as libc::c_char;
                cursor = cursor.offset(1);
            }
        }
        0 | 1 | _ => {}
    };
}
unsafe extern "C" fn define_all_fields(mut occurs: *mut OCCURS) {
    let mut tail_max_width: ptrdiff_t = 0;
    let mut head_max_width: ptrdiff_t = 0;
    let mut cursor: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut left_context_start: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut right_context_end: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut left_field_start: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut file_name: *const libc::c_char = 0 as *const libc::c_char;
    let mut line_ordinal: intmax_t = 0;
    let mut buffer_start: *const libc::c_char = 0 as *const libc::c_char;
    let mut buffer_end: *const libc::c_char = 0 as *const libc::c_char;
    keyafter.start = (*occurs).key.start;
    keyafter.end = (keyafter.start).offset((*occurs).key.size as isize);
    left_context_start = (keyafter.start).offset((*occurs).left as isize);
    right_context_end = (keyafter.start).offset((*occurs).right as isize);
    buffer_start = (*text_buffers.offset((*occurs).file_index as isize)).start;
    buffer_end = (*text_buffers.offset((*occurs).file_index as isize)).end;
    cursor = keyafter.end;
    while cursor < right_context_end
        && cursor <= (keyafter.start).offset(keyafter_max_width as isize)
    {
        keyafter.end = cursor;
        if !(word_regex.string).is_null() {
            let mut count: regoff_t = 0;
            count = re_match(
                &mut word_regex.pattern,
                cursor,
                right_context_end.offset_from(cursor) as libc::c_long as regoff_t,
                0 as libc::c_int,
                0 as *mut re_registers,
            );
            if count == -(2 as libc::c_int) {
                matcher_error();
            }
            cursor = cursor
                .offset(
                    (if count == -(1 as libc::c_int) { 1 as libc::c_int } else { count })
                        as isize,
                );
        } else if word_fastmap[to_uchar(*cursor) as usize] != 0 {
            while cursor < right_context_end
                && word_fastmap[to_uchar(*cursor) as usize] as libc::c_int != 0
            {
                cursor = cursor.offset(1);
            }
        } else {
            cursor = cursor.offset(1);
        }
    }
    if cursor <= (keyafter.start).offset(keyafter_max_width as isize) {
        keyafter.end = cursor;
    }
    keyafter_truncation = !truncation_string.is_null()
        && keyafter.end < right_context_end;
    while keyafter.end > keyafter.start
        && *(*__ctype_b_loc())
            .offset(
                to_uchar(*(keyafter.end).offset(-(1 as libc::c_int) as isize))
                    as libc::c_int as isize,
            ) as libc::c_int & _ISspace as libc::c_int as libc::c_ushort as libc::c_int
            != 0
    {
        keyafter.end = (keyafter.end).offset(-1);
    }
    if -(*occurs).left > half_line_width + maximum_word_length {
        left_field_start = (keyafter.start)
            .offset(-((half_line_width + maximum_word_length) as isize));
        if !(word_regex.string).is_null() {
            let mut count_0: regoff_t = 0;
            count_0 = re_match(
                &mut word_regex.pattern,
                left_field_start,
                (keyafter.start).offset_from(left_field_start) as libc::c_long
                    as regoff_t,
                0 as libc::c_int,
                0 as *mut re_registers,
            );
            if count_0 == -(2 as libc::c_int) {
                matcher_error();
            }
            left_field_start = left_field_start
                .offset(
                    (if count_0 == -(1 as libc::c_int) {
                        1 as libc::c_int
                    } else {
                        count_0
                    }) as isize,
                );
        } else if word_fastmap[to_uchar(*left_field_start) as usize] != 0 {
            while left_field_start < keyafter.start
                && word_fastmap[to_uchar(*left_field_start) as usize] as libc::c_int != 0
            {
                left_field_start = left_field_start.offset(1);
            }
        } else {
            left_field_start = left_field_start.offset(1);
        }
    } else {
        left_field_start = (keyafter.start).offset((*occurs).left as isize);
    }
    before.start = left_field_start;
    before.end = keyafter.start;
    while before.end > before.start
        && *(*__ctype_b_loc())
            .offset(
                to_uchar(*(before.end).offset(-(1 as libc::c_int) as isize))
                    as libc::c_int as isize,
            ) as libc::c_int & _ISspace as libc::c_int as libc::c_ushort as libc::c_int
            != 0
    {
        before.end = (before.end).offset(-1);
    }
    while (before.start).offset(before_max_width as isize) < before.end {
        if !(word_regex.string).is_null() {
            let mut count_1: regoff_t = 0;
            count_1 = re_match(
                &mut word_regex.pattern,
                before.start,
                (before.end).offset_from(before.start) as libc::c_long as regoff_t,
                0 as libc::c_int,
                0 as *mut re_registers,
            );
            if count_1 == -(2 as libc::c_int) {
                matcher_error();
            }
            before
                .start = (before.start)
                .offset(
                    (if count_1 == -(1 as libc::c_int) {
                        1 as libc::c_int
                    } else {
                        count_1
                    }) as isize,
                );
        } else if word_fastmap[to_uchar(*before.start) as usize] != 0 {
            while before.start < before.end
                && word_fastmap[to_uchar(*before.start) as usize] as libc::c_int != 0
            {
                before.start = (before.start).offset(1);
            }
        } else {
            before.start = (before.start).offset(1);
        }
    }
    if !truncation_string.is_null() {
        cursor = before.start;
        while cursor > buffer_start as *mut libc::c_char
            && *(*__ctype_b_loc())
                .offset(
                    to_uchar(*cursor.offset(-(1 as libc::c_int) as isize)) as libc::c_int
                        as isize,
                ) as libc::c_int
                & _ISspace as libc::c_int as libc::c_ushort as libc::c_int != 0
        {
            cursor = cursor.offset(-1);
        }
        before_truncation = cursor > left_context_start;
    } else {
        before_truncation = 0 as libc::c_int != 0;
    }
    while before.start < buffer_end as *mut libc::c_char
        && *(*__ctype_b_loc()).offset(to_uchar(*before.start) as libc::c_int as isize)
            as libc::c_int & _ISspace as libc::c_int as libc::c_ushort as libc::c_int
            != 0
    {
        before.start = (before.start).offset(1);
    }
    tail_max_width = before_max_width
        - (before.end).offset_from(before.start) as libc::c_long - gap_size;
    if tail_max_width > 0 as libc::c_int as libc::c_long {
        tail.start = keyafter.end;
        while tail.start < buffer_end as *mut libc::c_char
            && *(*__ctype_b_loc()).offset(to_uchar(*tail.start) as libc::c_int as isize)
                as libc::c_int & _ISspace as libc::c_int as libc::c_ushort as libc::c_int
                != 0
        {
            tail.start = (tail.start).offset(1);
        }
        tail.end = tail.start;
        cursor = tail.end;
        while cursor < right_context_end
            && cursor < (tail.start).offset(tail_max_width as isize)
        {
            tail.end = cursor;
            if !(word_regex.string).is_null() {
                let mut count_2: regoff_t = 0;
                count_2 = re_match(
                    &mut word_regex.pattern,
                    cursor,
                    right_context_end.offset_from(cursor) as libc::c_long as regoff_t,
                    0 as libc::c_int,
                    0 as *mut re_registers,
                );
                if count_2 == -(2 as libc::c_int) {
                    matcher_error();
                }
                cursor = cursor
                    .offset(
                        (if count_2 == -(1 as libc::c_int) {
                            1 as libc::c_int
                        } else {
                            count_2
                        }) as isize,
                    );
            } else if word_fastmap[to_uchar(*cursor) as usize] != 0 {
                while cursor < right_context_end
                    && word_fastmap[to_uchar(*cursor) as usize] as libc::c_int != 0
                {
                    cursor = cursor.offset(1);
                }
            } else {
                cursor = cursor.offset(1);
            }
        }
        if cursor < (tail.start).offset(tail_max_width as isize) {
            tail.end = cursor;
        }
        if tail.end > tail.start {
            keyafter_truncation = 0 as libc::c_int != 0;
            tail_truncation = !truncation_string.is_null()
                && tail.end < right_context_end;
        } else {
            tail_truncation = 0 as libc::c_int != 0;
        }
        while tail.end > tail.start
            && *(*__ctype_b_loc())
                .offset(
                    to_uchar(*(tail.end).offset(-(1 as libc::c_int) as isize))
                        as libc::c_int as isize,
                ) as libc::c_int
                & _ISspace as libc::c_int as libc::c_ushort as libc::c_int != 0
        {
            tail.end = (tail.end).offset(-1);
        }
    } else {
        tail.start = 0 as *mut libc::c_char;
        tail.end = 0 as *mut libc::c_char;
        tail_truncation = 0 as libc::c_int != 0;
    }
    head_max_width = keyafter_max_width
        - (keyafter.end).offset_from(keyafter.start) as libc::c_long - gap_size;
    if head_max_width > 0 as libc::c_int as libc::c_long {
        head.end = before.start;
        while head.end > buffer_start as *mut libc::c_char
            && *(*__ctype_b_loc())
                .offset(
                    to_uchar(*(head.end).offset(-(1 as libc::c_int) as isize))
                        as libc::c_int as isize,
                ) as libc::c_int
                & _ISspace as libc::c_int as libc::c_ushort as libc::c_int != 0
        {
            head.end = (head.end).offset(-1);
        }
        head.start = left_field_start;
        while (head.start).offset(head_max_width as isize) < head.end {
            if !(word_regex.string).is_null() {
                let mut count_3: regoff_t = 0;
                count_3 = re_match(
                    &mut word_regex.pattern,
                    head.start,
                    (head.end).offset_from(head.start) as libc::c_long as regoff_t,
                    0 as libc::c_int,
                    0 as *mut re_registers,
                );
                if count_3 == -(2 as libc::c_int) {
                    matcher_error();
                }
                head
                    .start = (head.start)
                    .offset(
                        (if count_3 == -(1 as libc::c_int) {
                            1 as libc::c_int
                        } else {
                            count_3
                        }) as isize,
                    );
            } else if word_fastmap[to_uchar(*head.start) as usize] != 0 {
                while head.start < head.end
                    && word_fastmap[to_uchar(*head.start) as usize] as libc::c_int != 0
                {
                    head.start = (head.start).offset(1);
                }
            } else {
                head.start = (head.start).offset(1);
            }
        }
        if head.end > head.start {
            before_truncation = 0 as libc::c_int != 0;
            head_truncation = !truncation_string.is_null()
                && head.start > left_context_start;
        } else {
            head_truncation = 0 as libc::c_int != 0;
        }
        while head.start < head.end
            && *(*__ctype_b_loc()).offset(to_uchar(*head.start) as libc::c_int as isize)
                as libc::c_int & _ISspace as libc::c_int as libc::c_ushort as libc::c_int
                != 0
        {
            head.start = (head.start).offset(1);
        }
    } else {
        head.start = 0 as *mut libc::c_char;
        head.end = 0 as *mut libc::c_char;
        head_truncation = 0 as libc::c_int != 0;
    }
    if auto_reference {
        file_name = *input_file_name.offset((*occurs).file_index as isize);
        if file_name.is_null() {
            file_name = b"\0" as *const u8 as *const libc::c_char;
        }
        line_ordinal = (*occurs).reference + 1 as libc::c_int as libc::c_long;
        if (*occurs).file_index > 0 as libc::c_int {
            line_ordinal
                -= *file_line_count
                    .offset(((*occurs).file_index - 1 as libc::c_int) as isize);
        }
        let mut file_end: *mut libc::c_char = stpcpy(reference.start, file_name);
        reference
            .end = file_end
            .offset(
                sprintf(
                    file_end,
                    b":%ld\0" as *const u8 as *const libc::c_char,
                    line_ordinal,
                ) as isize,
            );
    } else if input_reference {
        reference.start = (keyafter.start).offset((*occurs).reference as isize);
        reference.end = reference.start;
        while reference.end < right_context_end
            && *(*__ctype_b_loc())
                .offset(to_uchar(*reference.end) as libc::c_int as isize) as libc::c_int
                & _ISspace as libc::c_int as libc::c_ushort as libc::c_int == 0
        {
            reference.end = (reference.end).offset(1);
        }
    }
}
unsafe extern "C" fn output_one_roff_line() {
    printf(b".%s \"\0" as *const u8 as *const libc::c_char, macro_name);
    print_field(tail);
    if tail_truncation {
        fputs_unlocked(truncation_string, stdout);
    }
    putchar_unlocked('"' as i32);
    fputs_unlocked(b" \"\0" as *const u8 as *const libc::c_char, stdout);
    if before_truncation {
        fputs_unlocked(truncation_string, stdout);
    }
    print_field(before);
    putchar_unlocked('"' as i32);
    fputs_unlocked(b" \"\0" as *const u8 as *const libc::c_char, stdout);
    print_field(keyafter);
    if keyafter_truncation {
        fputs_unlocked(truncation_string, stdout);
    }
    putchar_unlocked('"' as i32);
    fputs_unlocked(b" \"\0" as *const u8 as *const libc::c_char, stdout);
    if head_truncation {
        fputs_unlocked(truncation_string, stdout);
    }
    print_field(head);
    putchar_unlocked('"' as i32);
    if auto_reference as libc::c_int != 0 || input_reference as libc::c_int != 0 {
        fputs_unlocked(b" \"\0" as *const u8 as *const libc::c_char, stdout);
        print_field(reference);
        putchar_unlocked('"' as i32);
    }
    putchar_unlocked('\n' as i32);
}
unsafe extern "C" fn output_one_tex_line() {
    let mut key: BLOCK = BLOCK {
        start: 0 as *const libc::c_char as *mut libc::c_char,
        end: 0 as *const libc::c_char as *mut libc::c_char,
    };
    let mut after: BLOCK = BLOCK {
        start: 0 as *const libc::c_char as *mut libc::c_char,
        end: 0 as *const libc::c_char as *mut libc::c_char,
    };
    let mut cursor: *mut libc::c_char = 0 as *mut libc::c_char;
    printf(b"\\%s \0" as *const u8 as *const libc::c_char, macro_name);
    putchar_unlocked('{' as i32);
    print_field(tail);
    fputs_unlocked(b"}{\0" as *const u8 as *const libc::c_char, stdout);
    print_field(before);
    fputs_unlocked(b"}{\0" as *const u8 as *const libc::c_char, stdout);
    key.start = keyafter.start;
    after.end = keyafter.end;
    cursor = keyafter.start;
    if !(word_regex.string).is_null() {
        let mut count: regoff_t = 0;
        count = re_match(
            &mut word_regex.pattern,
            cursor,
            (keyafter.end).offset_from(cursor) as libc::c_long as regoff_t,
            0 as libc::c_int,
            0 as *mut re_registers,
        );
        if count == -(2 as libc::c_int) {
            matcher_error();
        }
        cursor = cursor
            .offset(
                (if count == -(1 as libc::c_int) { 1 as libc::c_int } else { count })
                    as isize,
            );
    } else if word_fastmap[to_uchar(*cursor) as usize] != 0 {
        while cursor < keyafter.end
            && word_fastmap[to_uchar(*cursor) as usize] as libc::c_int != 0
        {
            cursor = cursor.offset(1);
        }
    } else {
        cursor = cursor.offset(1);
    }
    key.end = cursor;
    after.start = cursor;
    print_field(key);
    fputs_unlocked(b"}{\0" as *const u8 as *const libc::c_char, stdout);
    print_field(after);
    fputs_unlocked(b"}{\0" as *const u8 as *const libc::c_char, stdout);
    print_field(head);
    putchar_unlocked('}' as i32);
    if auto_reference as libc::c_int != 0 || input_reference as libc::c_int != 0 {
        putchar_unlocked('{' as i32);
        print_field(reference);
        putchar_unlocked('}' as i32);
    }
    putchar_unlocked('\n' as i32);
}
unsafe extern "C" fn output_one_dumb_line() {
    if !right_reference {
        if auto_reference {
            print_field(reference);
            putchar_unlocked(':' as i32);
            print_spaces(
                reference_max_width + gap_size
                    - (reference.end).offset_from(reference.start) as libc::c_long
                    - 1 as libc::c_int as libc::c_long,
            );
        } else {
            print_field(reference);
            print_spaces(
                reference_max_width + gap_size
                    - (reference.end).offset_from(reference.start) as libc::c_long,
            );
        }
    }
    if tail.start < tail.end {
        print_field(tail);
        if tail_truncation {
            fputs_unlocked(truncation_string, stdout);
        }
        print_spaces(
            half_line_width - gap_size
                - (before.end).offset_from(before.start) as libc::c_long
                - (if before_truncation as libc::c_int != 0 {
                    truncation_string_length
                } else {
                    0 as libc::c_int as libc::c_long
                }) - (tail.end).offset_from(tail.start) as libc::c_long
                - (if tail_truncation as libc::c_int != 0 {
                    truncation_string_length
                } else {
                    0 as libc::c_int as libc::c_long
                }),
        );
    } else {
        print_spaces(
            half_line_width - gap_size
                - (before.end).offset_from(before.start) as libc::c_long
                - (if before_truncation as libc::c_int != 0 {
                    truncation_string_length
                } else {
                    0 as libc::c_int as libc::c_long
                }),
        );
    }
    if before_truncation {
        fputs_unlocked(truncation_string, stdout);
    }
    print_field(before);
    print_spaces(gap_size);
    print_field(keyafter);
    if keyafter_truncation {
        fputs_unlocked(truncation_string, stdout);
    }
    if head.start < head.end {
        print_spaces(
            half_line_width - (keyafter.end).offset_from(keyafter.start) as libc::c_long
                - (if keyafter_truncation as libc::c_int != 0 {
                    truncation_string_length
                } else {
                    0 as libc::c_int as libc::c_long
                }) - (head.end).offset_from(head.start) as libc::c_long
                - (if head_truncation as libc::c_int != 0 {
                    truncation_string_length
                } else {
                    0 as libc::c_int as libc::c_long
                }),
        );
        if head_truncation {
            fputs_unlocked(truncation_string, stdout);
        }
        print_field(head);
    } else if (auto_reference as libc::c_int != 0 || input_reference as libc::c_int != 0)
        && right_reference as libc::c_int != 0
    {
        print_spaces(
            half_line_width - (keyafter.end).offset_from(keyafter.start) as libc::c_long
                - (if keyafter_truncation as libc::c_int != 0 {
                    truncation_string_length
                } else {
                    0 as libc::c_int as libc::c_long
                }),
        );
    }
    if (auto_reference as libc::c_int != 0 || input_reference as libc::c_int != 0)
        && right_reference as libc::c_int != 0
    {
        print_spaces(gap_size);
        print_field(reference);
    }
    putchar_unlocked('\n' as i32);
}
unsafe extern "C" fn generate_all_output() {
    let mut occurs_index: ptrdiff_t = 0;
    let mut occurs_cursor: *mut OCCURS = 0 as *mut OCCURS;
    tail.start = 0 as *mut libc::c_char;
    tail.end = 0 as *mut libc::c_char;
    tail_truncation = 0 as libc::c_int != 0;
    head.start = 0 as *mut libc::c_char;
    head.end = 0 as *mut libc::c_char;
    head_truncation = 0 as libc::c_int != 0;
    occurs_cursor = occurs_table[0 as libc::c_int as usize];
    occurs_index = 0 as libc::c_int as ptrdiff_t;
    while occurs_index < number_of_occurs[0 as libc::c_int as usize] {
        define_all_fields(occurs_cursor);
        match output_format as libc::c_uint {
            0 | 1 => {
                output_one_dumb_line();
            }
            2 => {
                output_one_roff_line();
            }
            3 => {
                output_one_tex_line();
            }
            _ => {}
        }
        occurs_cursor = occurs_cursor.offset(1);
        occurs_index += 1;
    }
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
                b"Usage: %s [OPTION]... [INPUT]...   (without -G)\n  or:  %s -G [OPTION]... [INPUT [OUTPUT]]\n\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            program_name,
            program_name,
        );
        fputs_unlocked(
            dcgettext(
                0 as *const libc::c_char,
                b"Output a permuted index, including context, of the words in the input files.\n\0"
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
                b"  -A, --auto-reference           output automatically generated references\n  -G, --traditional              behave more like System V 'ptx'\n\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            stdout,
        );
        fputs_unlocked(
            dcgettext(
                0 as *const libc::c_char,
                b"  -F, --flag-truncation=STRING   use STRING for flagging line truncations.\n                                 The default is '/'\n\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            stdout,
        );
        fputs_unlocked(
            dcgettext(
                0 as *const libc::c_char,
                b"  -M, --macro-name=STRING        macro name to use instead of 'xx'\n  -O, --format=roff              generate output as roff directives\n  -R, --right-side-refs          put references at right, not counted in -w\n  -S, --sentence-regexp=REGEXP   for end of lines or end of sentences\n  -T, --format=tex               generate output as TeX directives\n\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            stdout,
        );
        fputs_unlocked(
            dcgettext(
                0 as *const libc::c_char,
                b"  -W, --word-regexp=REGEXP       use REGEXP to match each keyword\n  -b, --break-file=FILE          word break characters in this FILE\n  -f, --ignore-case              fold lower case to upper case for sorting\n  -g, --gap-size=NUMBER          gap size in columns between output fields\n  -i, --ignore-file=FILE         read ignore word list from FILE\n  -o, --only-file=FILE           read only word list from this FILE\n\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            stdout,
        );
        fputs_unlocked(
            dcgettext(
                0 as *const libc::c_char,
                b"  -r, --references               first field of each line is a reference\n  -t, --typeset-mode               - not implemented -\n  -w, --width=NUMBER             output width in columns, reference excluded\n\0"
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
        emit_ancillary_info(b"ptx\0" as *const u8 as *const libc::c_char);
    }
    exit(status);
}
static mut long_options: [option; 19] = [
    {
        let mut init = option {
            name: b"auto-reference\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'A' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"break-file\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'b' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"flag-truncation\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'F' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"ignore-case\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'f' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"gap-size\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'g' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"ignore-file\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'i' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"macro-name\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'M' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"only-file\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'o' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"references\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'r' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"right-side-refs\0" as *const u8 as *const libc::c_char,
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
            val: 10 as libc::c_int,
        };
        init
    },
    {
        let mut init = option {
            name: b"sentence-regexp\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'S' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"traditional\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'G' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"typeset-mode\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 't' as i32,
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
            name: b"word-regexp\0" as *const u8 as *const libc::c_char,
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
static mut format_args: [*const libc::c_char; 3] = [
    b"roff\0" as *const u8 as *const libc::c_char,
    b"tex\0" as *const u8 as *const libc::c_char,
    0 as *const libc::c_char,
];
static mut format_vals: [Format; 2] = [ROFF_FORMAT, TEX_FORMAT];
unsafe fn main_0(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut optchar: libc::c_int = 0;
    let mut file_index: libc::c_int = 0;
    set_program_name(*argv.offset(0 as libc::c_int as isize));
    setlocale(6 as libc::c_int, b"\0" as *const u8 as *const libc::c_char);
    bindtextdomain(
        b"coreutils\0" as *const u8 as *const libc::c_char,
        b"/usr/local/share/locale\0" as *const u8 as *const libc::c_char,
    );
    textdomain(b"coreutils\0" as *const u8 as *const libc::c_char);
    atexit(Some(close_stdout as unsafe extern "C" fn() -> ()));
    loop {
        optchar = getopt_long(
            argc,
            argv,
            b"AF:GM:ORS:TW:b:i:fg:o:trw:\0" as *const u8 as *const libc::c_char,
            long_options.as_ptr(),
            0 as *mut libc::c_int,
        );
        if !(optchar != -(1 as libc::c_int)) {
            break;
        }
        let mut current_block_37: u64;
        match optchar {
            71 => {
                current_block_37 = 1505195771936801158;
            }
            98 => {
                break_file = optarg;
                current_block_37 = 1847472278776910194;
            }
            102 => {
                ignore_case = 1 as libc::c_int != 0;
                current_block_37 = 1847472278776910194;
            }
            103 => {
                let mut tmp: intmax_t = 0;
                if !(xstrtoimax(
                    optarg,
                    0 as *mut *mut libc::c_char,
                    0 as libc::c_int,
                    &mut tmp,
                    b"\0" as *const u8 as *const libc::c_char,
                ) as libc::c_uint == LONGINT_OK as libc::c_int as libc::c_uint
                    && (0 as libc::c_int as libc::c_long) < tmp
                    && tmp <= 9223372036854775807 as libc::c_long)
                {
                    if ::core::mem::size_of::<C2RustUnnamed_7>() as libc::c_ulong != 0 {
                        error(
                            1 as libc::c_int,
                            0 as libc::c_int,
                            dcgettext(
                                0 as *const libc::c_char,
                                b"invalid gap width: %s\0" as *const u8
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
                                b"invalid gap width: %s\0" as *const u8
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
                gap_size = tmp;
                current_block_37 = 1847472278776910194;
            }
            105 => {
                ignore_file = optarg;
                current_block_37 = 1847472278776910194;
            }
            111 => {
                only_file = optarg;
                current_block_37 = 1847472278776910194;
            }
            114 => {
                input_reference = 1 as libc::c_int != 0;
                current_block_37 = 1847472278776910194;
            }
            116 => {
                current_block_37 = 1847472278776910194;
            }
            119 => {
                let mut tmp_0: intmax_t = 0;
                if !(xstrtoimax(
                    optarg,
                    0 as *mut *mut libc::c_char,
                    0 as libc::c_int,
                    &mut tmp_0,
                    b"\0" as *const u8 as *const libc::c_char,
                ) as libc::c_uint == LONGINT_OK as libc::c_int as libc::c_uint
                    && (0 as libc::c_int as libc::c_long) < tmp_0
                    && tmp_0 <= 9223372036854775807 as libc::c_long)
                {
                    if ::core::mem::size_of::<C2RustUnnamed_6>() as libc::c_ulong != 0 {
                        error(
                            1 as libc::c_int,
                            0 as libc::c_int,
                            dcgettext(
                                0 as *const libc::c_char,
                                b"invalid line width: %s\0" as *const u8
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
                                b"invalid line width: %s\0" as *const u8
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
                line_width = tmp_0;
                current_block_37 = 1847472278776910194;
            }
            65 => {
                auto_reference = 1 as libc::c_int != 0;
                current_block_37 = 1847472278776910194;
            }
            70 => {
                truncation_string = optarg;
                unescape_string(optarg);
                current_block_37 = 1847472278776910194;
            }
            77 => {
                macro_name = optarg;
                current_block_37 = 1847472278776910194;
            }
            79 => {
                output_format = ROFF_FORMAT;
                current_block_37 = 1847472278776910194;
            }
            82 => {
                right_reference = 1 as libc::c_int != 0;
                current_block_37 = 1847472278776910194;
            }
            83 => {
                context_regex.string = optarg;
                unescape_string(optarg);
                current_block_37 = 1847472278776910194;
            }
            84 => {
                output_format = TEX_FORMAT;
                current_block_37 = 1847472278776910194;
            }
            87 => {
                word_regex.string = optarg;
                unescape_string(optarg);
                if *word_regex.string == 0 {
                    word_regex.string = 0 as *const libc::c_char;
                }
                current_block_37 = 1847472278776910194;
            }
            10 => {
                output_format = format_vals[__xargmatch_internal(
                    b"--format\0" as *const u8 as *const libc::c_char,
                    optarg,
                    format_args.as_ptr(),
                    format_vals.as_ptr() as *const libc::c_void,
                    ::core::mem::size_of::<Format>() as libc::c_ulong,
                    argmatch_die,
                    1 as libc::c_int != 0,
                ) as usize];
                current_block_37 = 1847472278776910194;
            }
            -2 => {
                usage(0 as libc::c_int);
                current_block_37 = 1847472278776910194;
            }
            -3 => {
                version_etc(
                    stdout,
                    b"ptx\0" as *const u8 as *const libc::c_char,
                    b"GNU coreutils\0" as *const u8 as *const libc::c_char,
                    Version,
                    proper_name_utf8(
                        b"F. Pinard\0" as *const u8 as *const libc::c_char,
                        b"Fran\xC3\xA7ois Pinard\0" as *const u8 as *const libc::c_char,
                    ),
                    0 as *mut libc::c_void as *mut libc::c_char,
                );
                exit(0 as libc::c_int);
            }
            _ => {
                usage(1 as libc::c_int);
                current_block_37 = 1505195771936801158;
            }
        }
        match current_block_37 {
            1505195771936801158 => {
                gnu_extensions = 0 as libc::c_int != 0;
            }
            _ => {}
        }
    }
    if optind == argc {
        input_file_name = xmalloc(
            ::core::mem::size_of::<*const libc::c_char>() as libc::c_ulong,
        ) as *mut *const libc::c_char;
        file_line_count = xmalloc(::core::mem::size_of::<intmax_t>() as libc::c_ulong)
            as *mut intmax_t;
        text_buffers = xmalloc(::core::mem::size_of::<BLOCK>() as libc::c_ulong)
            as *mut BLOCK;
        number_input_files = 1 as libc::c_int;
        let ref mut fresh18 = *input_file_name.offset(0 as libc::c_int as isize);
        *fresh18 = 0 as *const libc::c_char;
    } else if gnu_extensions {
        number_input_files = argc - optind;
        input_file_name = xnmalloc(
            number_input_files as size_t,
            ::core::mem::size_of::<*const libc::c_char>() as libc::c_ulong,
        ) as *mut *const libc::c_char;
        file_line_count = xnmalloc(
            number_input_files as size_t,
            ::core::mem::size_of::<intmax_t>() as libc::c_ulong,
        ) as *mut intmax_t;
        text_buffers = xnmalloc(
            number_input_files as size_t,
            ::core::mem::size_of::<BLOCK>() as libc::c_ulong,
        ) as *mut BLOCK;
        file_index = 0 as libc::c_int;
        while file_index < number_input_files {
            if **argv.offset(optind as isize) == 0
                || strcmp(
                    *argv.offset(optind as isize),
                    b"-\0" as *const u8 as *const libc::c_char,
                ) == 0 as libc::c_int
            {
                let ref mut fresh19 = *input_file_name.offset(file_index as isize);
                *fresh19 = 0 as *const libc::c_char;
            } else {
                let ref mut fresh20 = *input_file_name.offset(file_index as isize);
                *fresh20 = *argv.offset(optind as isize);
            }
            optind += 1;
            file_index += 1;
        }
    } else {
        number_input_files = 1 as libc::c_int;
        input_file_name = xmalloc(
            ::core::mem::size_of::<*const libc::c_char>() as libc::c_ulong,
        ) as *mut *const libc::c_char;
        file_line_count = xmalloc(::core::mem::size_of::<intmax_t>() as libc::c_ulong)
            as *mut intmax_t;
        text_buffers = xmalloc(::core::mem::size_of::<BLOCK>() as libc::c_ulong)
            as *mut BLOCK;
        if **argv.offset(optind as isize) == 0
            || strcmp(
                *argv.offset(optind as isize),
                b"-\0" as *const u8 as *const libc::c_char,
            ) == 0 as libc::c_int
        {
            let ref mut fresh21 = *input_file_name.offset(0 as libc::c_int as isize);
            *fresh21 = 0 as *const libc::c_char;
        } else {
            let ref mut fresh22 = *input_file_name.offset(0 as libc::c_int as isize);
            *fresh22 = *argv.offset(optind as isize);
        }
        optind += 1;
        if optind < argc {
            if (freopen_safer(
                *argv.offset(optind as isize),
                b"w\0" as *const u8 as *const libc::c_char,
                stdout,
            ))
                .is_null()
            {
                if ::core::mem::size_of::<C2RustUnnamed_5>() as libc::c_ulong != 0 {
                    error(
                        1 as libc::c_int,
                        *__errno_location(),
                        b"%s\0" as *const u8 as *const libc::c_char,
                        quotearg_n_style_colon(
                            0 as libc::c_int,
                            shell_escape_quoting_style,
                            *argv.offset(optind as isize),
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
                            *argv.offset(optind as isize),
                        ),
                    );
                    if 0 as libc::c_int != 0 {} else {
                        unreachable!();
                    };
                };
            }
            optind += 1;
        }
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
            usage(1 as libc::c_int);
        }
    }
    if output_format as libc::c_uint == UNKNOWN_FORMAT as libc::c_int as libc::c_uint {
        output_format = (if gnu_extensions as libc::c_int != 0 {
            DUMB_FORMAT as libc::c_int
        } else {
            ROFF_FORMAT as libc::c_int
        }) as Format;
    }
    initialize_regex();
    if !break_file.is_null() {
        digest_break_file(break_file);
    }
    if !ignore_file.is_null() {
        digest_word_file(ignore_file, &mut ignore_table);
        if ignore_table.length == 0 as libc::c_int as libc::c_long {
            ignore_file = 0 as *const libc::c_char;
        }
    }
    if !only_file.is_null() {
        digest_word_file(only_file, &mut only_table);
        if only_table.length == 0 as libc::c_int as libc::c_long {
            only_file = 0 as *const libc::c_char;
        }
    }
    number_of_occurs[0 as libc::c_int as usize] = 0 as libc::c_int as ptrdiff_t;
    total_line_count = 0 as libc::c_int as intmax_t;
    maximum_word_length = 0 as libc::c_int as ptrdiff_t;
    reference_max_width = 0 as libc::c_int as ptrdiff_t;
    file_index = 0 as libc::c_int;
    while file_index < number_input_files {
        let mut text_buffer: *mut BLOCK = text_buffers.offset(file_index as isize);
        swallow_file_in_memory(
            *input_file_name.offset(file_index as isize),
            text_buffer,
        );
        find_occurs_in_text(file_index);
        total_line_count += 1;
        *file_line_count.offset(file_index as isize) = total_line_count;
        file_index += 1;
    }
    sort_found_occurs();
    fix_output_parameters();
    generate_all_output();
    return 0 as libc::c_int;
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
