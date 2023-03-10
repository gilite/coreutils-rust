#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![feature(extern_types, label_break_value)]
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type re_dfa_t;
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
    fn __overflow(_: *mut FILE, _: libc::c_int) -> libc::c_int;
    fn puts(__s: *const libc::c_char) -> libc::c_int;
    fn fputs_unlocked(__s: *const libc::c_char, __stream: *mut FILE) -> libc::c_int;
    fn __ctype_get_mb_cur_max() -> size_t;
    fn abort() -> !;
    fn atexit(__func: Option::<unsafe extern "C" fn() -> ()>) -> libc::c_int;
    fn exit(_: libc::c_int) -> !;
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
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strncmp(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn strcoll(__s1: *const libc::c_char, __s2: *const libc::c_char) -> libc::c_int;
    fn strcspn(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_ulong;
    fn mempcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn free(_: *mut libc::c_void);
    fn mbslen(string_0: *const libc::c_char) -> size_t;
    fn mbschr(string_0: *const libc::c_char, c: libc::c_int) -> *mut libc::c_char;
    fn __errno_location() -> *mut libc::c_int;
    static mut exit_failure: libc::c_int;
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
    fn xstrdup(str: *const libc::c_char) -> *mut libc::c_char;
    fn close_stdout();
    static mut program_name: *const libc::c_char;
    fn set_program_name(argv0: *const libc::c_char);
    fn quotearg_n_style(
        n: libc::c_int,
        s: quoting_style,
        arg: *const libc::c_char,
    ) -> *mut libc::c_char;
    fn __gmpz_add(_: mpz_ptr, _: mpz_srcptr, _: mpz_srcptr);
    fn __gmpz_clear(_: mpz_ptr);
    fn __gmpz_get_str(
        _: *mut libc::c_char,
        _: libc::c_int,
        _: mpz_srcptr,
    ) -> *mut libc::c_char;
    fn __gmpz_init_set_str(
        _: mpz_ptr,
        _: *const libc::c_char,
        _: libc::c_int,
    ) -> libc::c_int;
    fn __gmpz_init_set_ui(_: mpz_ptr, _: libc::c_ulong);
    fn __gmpz_mul(_: mpz_ptr, _: mpz_srcptr, _: mpz_srcptr);
    fn __gmpz_out_str(_: *mut FILE, _: libc::c_int, _: mpz_srcptr) -> size_t;
    fn __gmpz_sub(_: mpz_ptr, _: mpz_srcptr, _: mpz_srcptr);
    fn __gmpz_tdiv_q(_: mpz_ptr, _: mpz_srcptr, _: mpz_srcptr);
    fn __gmpz_tdiv_r(_: mpz_ptr, _: mpz_srcptr, _: mpz_srcptr);
    static mut re_syntax_options: reg_syntax_t;
    fn re_compile_pattern(
        __pattern: *const libc::c_char,
        __length: size_t,
        __buffer: *mut re_pattern_buffer,
    ) -> *const libc::c_char;
    fn re_match(
        __buffer: *mut re_pattern_buffer,
        __String: *const libc::c_char,
        __length: regoff_t,
        __start: regoff_t,
        __regs: *mut re_registers,
    ) -> regoff_t;
    fn regfree(__preg: *mut regex_t);
    fn error(
        __status: libc::c_int,
        __errnum: libc::c_int,
        __format: *const libc::c_char,
        _: ...
    );
    fn parse_long_options(
        _argc: libc::c_int,
        _argv: *mut *mut libc::c_char,
        _command_name: *const libc::c_char,
        _package: *const libc::c_char,
        _version: *const libc::c_char,
        _usage: Option::<unsafe extern "C" fn(libc::c_int) -> ()>,
        _: ...
    );
    fn mbsinit(__ps: *const mbstate_t) -> libc::c_int;
    static is_basic_table: [libc::c_uint; 0];
    fn strnlen1(string_0: *const libc::c_char, maxlen: size_t) -> size_t;
    fn rpl_mbrtowc(
        pwc: *mut wchar_t,
        s: *const libc::c_char,
        n: size_t,
        ps: *mut mbstate_t,
    ) -> size_t;
    fn strintcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
}
pub type size_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
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
pub type ptrdiff_t = libc::c_long;
pub type wchar_t = libc::c_uint;
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
pub type mp_limb_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __mpz_struct {
    pub _mp_alloc: libc::c_int,
    pub _mp_size: libc::c_int,
    pub _mp_d: *mut mp_limb_t,
}
pub type mpz_t = [__mpz_struct; 1];
pub type mp_ptr = *mut mp_limb_t;
pub type mp_size_t = libc::c_long;
pub type mpz_srcptr = *const __mpz_struct;
pub type mpz_ptr = *mut __mpz_struct;
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
pub type regex_t = re_pattern_buffer;
pub type regoff_t = libc::c_int;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct re_registers {
    pub num_regs: __re_size_t,
    pub start: *mut regoff_t,
    pub end: *mut regoff_t,
}
pub type mbstate_t = __mbstate_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mbchar {
    pub ptr: *const libc::c_char,
    pub bytes: size_t,
    pub wc_valid: bool,
    pub wc: wchar_t,
    pub buf: [libc::c_char; 24],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mbuiter_multi {
    pub in_shift: bool,
    pub state: mbstate_t,
    pub next_done: bool,
    pub cur: mbchar,
}
pub type mbui_iterator_t = mbuiter_multi;
pub type C2RustUnnamed_0 = libc::c_uint;
pub const EXPR_FAILURE: C2RustUnnamed_0 = 3;
pub const EXPR_INVALID: C2RustUnnamed_0 = 2;
pub type valtype = libc::c_uint;
pub const string: valtype = 1;
pub const integer: valtype = 0;
pub type TYPE = valtype;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct valinfo {
    pub type_0: TYPE,
    pub u: C2RustUnnamed_1,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_1 {
    pub i: mpz_t,
    pub s: *mut libc::c_char,
}
pub type VALUE = valinfo;
pub const greater_than: C2RustUnnamed_2 = 5;
pub const greater_equal: C2RustUnnamed_2 = 4;
pub const not_equal: C2RustUnnamed_2 = 3;
pub const equal: C2RustUnnamed_2 = 2;
pub const less_equal: C2RustUnnamed_2 = 1;
pub const less_than: C2RustUnnamed_2 = 0;
pub type C2RustUnnamed_2 = libc::c_uint;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_3 {
    pub _gl_dummy: libc::c_int,
}
pub const plus: C2RustUnnamed_4 = 0;
pub type C2RustUnnamed_4 = libc::c_uint;
pub const minus: C2RustUnnamed_4 = 1;
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
pub const divide: C2RustUnnamed_7 = 1;
pub type C2RustUnnamed_7 = libc::c_uint;
pub const mod_0: C2RustUnnamed_7 = 2;
pub const multiply: C2RustUnnamed_7 = 0;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_15 {
    pub _gl_dummy: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_16 {
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
unsafe extern "C" fn initialize_exit_failure(mut status: libc::c_int) {
    if status != 1 as libc::c_int {
        ::core::ptr::write_volatile(&mut exit_failure as *mut libc::c_int, status);
    }
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
unsafe extern "C" fn __gmpz_fits_ulong_p(mut __gmp_z: mpz_srcptr) -> libc::c_int {
    let mut __gmp_n: mp_size_t = (*__gmp_z)._mp_size as mp_size_t;
    let mut __gmp_p: mp_ptr = (*__gmp_z)._mp_d;
    return (__gmp_n == 0 as libc::c_int as libc::c_long
        || __gmp_n == 1 as libc::c_int as libc::c_long
            && *__gmp_p.offset(0 as libc::c_int as isize)
                <= (9223372036854775807 as libc::c_long as libc::c_ulong)
                    .wrapping_mul(2 as libc::c_ulong)
                    .wrapping_add(1 as libc::c_ulong)) as libc::c_int;
}
#[inline]
unsafe extern "C" fn __gmpz_get_ui(mut __gmp_z: mpz_srcptr) -> libc::c_ulong {
    let mut __gmp_p: mp_ptr = (*__gmp_z)._mp_d;
    let mut __gmp_n: mp_size_t = (*__gmp_z)._mp_size as mp_size_t;
    let mut __gmp_l: mp_limb_t = *__gmp_p.offset(0 as libc::c_int as isize);
    return if __gmp_n != 0 as libc::c_int as libc::c_long {
        __gmp_l
    } else {
        0 as libc::c_int as libc::c_ulong
    };
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
unsafe extern "C" fn mbuiter_multi_next(mut iter: *mut mbuiter_multi) {
    let mut current_block: u64;
    if (*iter).next_done {
        return;
    }
    if (*iter).in_shift {
        current_block = 6968754670800765692;
    } else if is_basic(*(*iter).cur.ptr) {
        (*iter).cur.bytes = 1 as libc::c_int as size_t;
        (*iter).cur.wc = *(*iter).cur.ptr as wchar_t;
        (*iter).cur.wc_valid = 1 as libc::c_int != 0;
        current_block = 15089075282327824602;
    } else {
        if mbsinit(&mut (*iter).state) != 0 {} else {
            __assert_fail(
                b"mbsinit (&iter->state)\0" as *const u8 as *const libc::c_char,
                b"./lib/mbuiter.h\0" as *const u8 as *const libc::c_char,
                142 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 48],
                    &[libc::c_char; 48],
                >(b"void mbuiter_multi_next(struct mbuiter_multi *)\0"))
                    .as_ptr(),
            );
        }
        (*iter).in_shift = 1 as libc::c_int != 0;
        current_block = 6968754670800765692;
    }
    match current_block {
        6968754670800765692 => {
            (*iter)
                .cur
                .bytes = rpl_mbrtowc(
                &mut (*iter).cur.wc,
                (*iter).cur.ptr,
                strnlen1((*iter).cur.ptr, __ctype_get_mb_cur_max()),
                &mut (*iter).state,
            );
            if (*iter).cur.bytes == -(1 as libc::c_int) as size_t {
                (*iter).cur.bytes = 1 as libc::c_int as size_t;
                (*iter).cur.wc_valid = 0 as libc::c_int != 0;
            } else if (*iter).cur.bytes == -(2 as libc::c_int) as size_t {
                (*iter).cur.bytes = strlen((*iter).cur.ptr);
                (*iter).cur.wc_valid = 0 as libc::c_int != 0;
            } else {
                if (*iter).cur.bytes == 0 as libc::c_int as libc::c_ulong {
                    (*iter).cur.bytes = 1 as libc::c_int as size_t;
                    if *(*iter).cur.ptr as libc::c_int == '\0' as i32 {} else {
                        __assert_fail(
                            b"*iter->cur.ptr == '\\0'\0" as *const u8
                                as *const libc::c_char,
                            b"./lib/mbuiter.h\0" as *const u8 as *const libc::c_char,
                            170 as libc::c_int as libc::c_uint,
                            (*::core::mem::transmute::<
                                &[u8; 48],
                                &[libc::c_char; 48],
                            >(b"void mbuiter_multi_next(struct mbuiter_multi *)\0"))
                                .as_ptr(),
                        );
                    }
                    if (*iter).cur.wc == 0 as libc::c_int as libc::c_uint {} else {
                        __assert_fail(
                            b"iter->cur.wc == 0\0" as *const u8 as *const libc::c_char,
                            b"./lib/mbuiter.h\0" as *const u8 as *const libc::c_char,
                            171 as libc::c_int as libc::c_uint,
                            (*::core::mem::transmute::<
                                &[u8; 48],
                                &[libc::c_char; 48],
                            >(b"void mbuiter_multi_next(struct mbuiter_multi *)\0"))
                                .as_ptr(),
                        );
                    }
                }
                (*iter).cur.wc_valid = 1 as libc::c_int != 0;
                if mbsinit(&mut (*iter).state) != 0 {
                    (*iter).in_shift = 0 as libc::c_int != 0;
                }
            }
        }
        _ => {}
    }
    (*iter).next_done = 1 as libc::c_int != 0;
}
static mut args: *mut *mut libc::c_char = 0 as *const *mut libc::c_char
    as *mut *mut libc::c_char;
unsafe extern "C" fn mbs_logical_cspn(
    mut s: *const libc::c_char,
    mut accept: *const libc::c_char,
) -> size_t {
    let mut idx: size_t = 0 as libc::c_int as size_t;
    if *accept.offset(0 as libc::c_int as isize) as libc::c_int == '\0' as i32 {
        return 0 as libc::c_int as size_t;
    }
    if __ctype_get_mb_cur_max() > 1 as libc::c_int as libc::c_ulong {
        let mut iter: mbui_iterator_t = mbui_iterator_t {
            in_shift: false,
            state: mbstate_t {
                __count: 0,
                __value: C2RustUnnamed { __wch: 0 },
            },
            next_done: false,
            cur: mbchar {
                ptr: 0 as *const libc::c_char,
                bytes: 0,
                wc_valid: false,
                wc: 0,
                buf: [0; 24],
            },
        };
        iter.cur.ptr = s;
        iter.in_shift = 0 as libc::c_int != 0;
        memset(
            &mut iter.state as *mut mbstate_t as *mut libc::c_void,
            '\0' as i32,
            ::core::mem::size_of::<mbstate_t>() as libc::c_ulong,
        );
        iter.next_done = 0 as libc::c_int != 0;
        loop {
            mbuiter_multi_next(&mut iter);
            if !(!(iter.cur.wc_valid as libc::c_int != 0
                && iter.cur.wc == 0 as libc::c_int as libc::c_uint) as libc::c_int != 0)
            {
                break;
            }
            idx = idx.wrapping_add(1);
            if iter.cur.bytes == 1 as libc::c_int as libc::c_ulong {
                if !(mbschr(accept, *iter.cur.ptr as libc::c_int)).is_null() {
                    return idx;
                }
            } else {
                let mut aiter: mbui_iterator_t = mbui_iterator_t {
                    in_shift: false,
                    state: mbstate_t {
                        __count: 0,
                        __value: C2RustUnnamed { __wch: 0 },
                    },
                    next_done: false,
                    cur: mbchar {
                        ptr: 0 as *const libc::c_char,
                        bytes: 0,
                        wc_valid: false,
                        wc: 0,
                        buf: [0; 24],
                    },
                };
                aiter.cur.ptr = accept;
                aiter.in_shift = 0 as libc::c_int != 0;
                memset(
                    &mut aiter.state as *mut mbstate_t as *mut libc::c_void,
                    '\0' as i32,
                    ::core::mem::size_of::<mbstate_t>() as libc::c_ulong,
                );
                aiter.next_done = 0 as libc::c_int != 0;
                loop {
                    mbuiter_multi_next(&mut aiter);
                    if !(!(aiter.cur.wc_valid as libc::c_int != 0
                        && aiter.cur.wc == 0 as libc::c_int as libc::c_uint)
                        as libc::c_int != 0)
                    {
                        break;
                    }
                    if if aiter.cur.wc_valid as libc::c_int != 0
                        && iter.cur.wc_valid as libc::c_int != 0
                    {
                        (aiter.cur.wc == iter.cur.wc) as libc::c_int
                    } else {
                        (aiter.cur.bytes == iter.cur.bytes
                            && memcmp(
                                aiter.cur.ptr as *const libc::c_void,
                                iter.cur.ptr as *const libc::c_void,
                                aiter.cur.bytes,
                            ) == 0 as libc::c_int) as libc::c_int
                    } != 0
                    {
                        return idx;
                    }
                    aiter.cur.ptr = (aiter.cur.ptr).offset(aiter.cur.bytes as isize);
                    aiter.next_done = 0 as libc::c_int != 0;
                }
            }
            iter.cur.ptr = (iter.cur.ptr).offset(iter.cur.bytes as isize);
            iter.next_done = 0 as libc::c_int != 0;
        }
        return 0 as libc::c_int as size_t;
    } else {
        let mut i: size_t = strcspn(s, accept);
        return if *s.offset(i as isize) as libc::c_int != 0 {
            i.wrapping_add(1 as libc::c_int as libc::c_ulong)
        } else {
            0 as libc::c_int as libc::c_ulong
        };
    };
}
unsafe extern "C" fn mbs_logical_substr(
    mut s: *const libc::c_char,
    mut pos: size_t,
    mut len: size_t,
) -> *mut libc::c_char {
    let mut v: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut vlim: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut blen: size_t = strlen(s);
    let mut llen: size_t = if __ctype_get_mb_cur_max()
        > 1 as libc::c_int as libc::c_ulong
    {
        mbslen(s)
    } else {
        blen
    };
    if llen < pos || pos == 0 as libc::c_int as libc::c_ulong
        || len == 0 as libc::c_int as libc::c_ulong
        || len == 18446744073709551615 as libc::c_ulong
    {
        return xstrdup(b"\0" as *const u8 as *const libc::c_char);
    }
    let mut vlen: size_t = if len
        < llen.wrapping_sub(pos).wrapping_add(1 as libc::c_int as libc::c_ulong)
    {
        len
    } else {
        llen.wrapping_sub(pos).wrapping_add(1 as libc::c_int as libc::c_ulong)
    };
    if __ctype_get_mb_cur_max() == 1 as libc::c_int as libc::c_ulong {
        v = xmalloc(vlen.wrapping_add(1 as libc::c_int as libc::c_ulong))
            as *mut libc::c_char;
        vlim = mempcpy(
            v as *mut libc::c_void,
            s.offset(pos as isize).offset(-(1 as libc::c_int as isize))
                as *const libc::c_void,
            vlen,
        ) as *mut libc::c_char;
    } else {
        v = xmalloc(blen.wrapping_add(1 as libc::c_int as libc::c_ulong))
            as *mut libc::c_char;
        vlim = v;
        let mut iter: mbui_iterator_t = mbui_iterator_t {
            in_shift: false,
            state: mbstate_t {
                __count: 0,
                __value: C2RustUnnamed { __wch: 0 },
            },
            next_done: false,
            cur: mbchar {
                ptr: 0 as *const libc::c_char,
                bytes: 0,
                wc_valid: false,
                wc: 0,
                buf: [0; 24],
            },
        };
        let mut idx: size_t = 1 as libc::c_int as size_t;
        iter.cur.ptr = s;
        iter.in_shift = 0 as libc::c_int != 0;
        memset(
            &mut iter.state as *mut mbstate_t as *mut libc::c_void,
            '\0' as i32,
            ::core::mem::size_of::<mbstate_t>() as libc::c_ulong,
        );
        iter.next_done = 0 as libc::c_int != 0;
        loop {
            mbuiter_multi_next(&mut iter);
            if !(!(iter.cur.wc_valid as libc::c_int != 0
                && iter.cur.wc == 0 as libc::c_int as libc::c_uint) as libc::c_int != 0
                && vlen > 0 as libc::c_int as libc::c_ulong)
            {
                break;
            }
            if !(idx < pos) {
                vlen = vlen.wrapping_sub(1);
                vlim = mempcpy(
                    vlim as *mut libc::c_void,
                    iter.cur.ptr as *const libc::c_void,
                    iter.cur.bytes,
                ) as *mut libc::c_char;
            }
            iter.cur.ptr = (iter.cur.ptr).offset(iter.cur.bytes as isize);
            iter.next_done = 0 as libc::c_int != 0;
            idx = idx.wrapping_add(1);
        }
    }
    *vlim = '\0' as i32 as libc::c_char;
    return v;
}
unsafe extern "C" fn mbs_offset_to_chars(
    mut s: *const libc::c_char,
    mut ofs: size_t,
) -> size_t {
    let mut iter: mbui_iterator_t = mbui_iterator_t {
        in_shift: false,
        state: mbstate_t {
            __count: 0,
            __value: C2RustUnnamed { __wch: 0 },
        },
        next_done: false,
        cur: mbchar {
            ptr: 0 as *const libc::c_char,
            bytes: 0,
            wc_valid: false,
            wc: 0,
            buf: [0; 24],
        },
    };
    let mut c: size_t = 0 as libc::c_int as size_t;
    iter.cur.ptr = s;
    iter.in_shift = 0 as libc::c_int != 0;
    memset(
        &mut iter.state as *mut mbstate_t as *mut libc::c_void,
        '\0' as i32,
        ::core::mem::size_of::<mbstate_t>() as libc::c_ulong,
    );
    iter.next_done = 0 as libc::c_int != 0;
    loop {
        mbuiter_multi_next(&mut iter);
        if !(!(iter.cur.wc_valid as libc::c_int != 0
            && iter.cur.wc == 0 as libc::c_int as libc::c_uint) as libc::c_int != 0)
        {
            break;
        }
        let mut d: ptrdiff_t = (iter.cur.ptr).offset_from(s) as libc::c_long;
        if d as libc::c_ulong >= ofs {
            break;
        }
        c = c.wrapping_add(1);
        iter.cur.ptr = (iter.cur.ptr).offset(iter.cur.bytes as isize);
        iter.next_done = 0 as libc::c_int != 0;
    }
    return c;
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
                b"Usage: %s EXPRESSION\n  or:  %s OPTION\n\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
            program_name,
            program_name,
        );
        putchar_unlocked('\n' as i32);
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
                b"\nPrint the value of EXPRESSION to standard output.  A blank line below\nseparates increasing precedence groups.  EXPRESSION may be:\n\n  ARG1 | ARG2       ARG1 if it is neither null nor 0, otherwise ARG2\n\n  ARG1 & ARG2       ARG1 if neither argument is null or 0, otherwise 0\n\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            stdout,
        );
        fputs_unlocked(
            dcgettext(
                0 as *const libc::c_char,
                b"\n  ARG1 < ARG2       ARG1 is less than ARG2\n  ARG1 <= ARG2      ARG1 is less than or equal to ARG2\n  ARG1 = ARG2       ARG1 is equal to ARG2\n  ARG1 != ARG2      ARG1 is unequal to ARG2\n  ARG1 >= ARG2      ARG1 is greater than or equal to ARG2\n  ARG1 > ARG2       ARG1 is greater than ARG2\n\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            stdout,
        );
        fputs_unlocked(
            dcgettext(
                0 as *const libc::c_char,
                b"\n  ARG1 + ARG2       arithmetic sum of ARG1 and ARG2\n  ARG1 - ARG2       arithmetic difference of ARG1 and ARG2\n\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            stdout,
        );
        fputs_unlocked(
            dcgettext(
                0 as *const libc::c_char,
                b"\n  ARG1 * ARG2       arithmetic product of ARG1 and ARG2\n  ARG1 / ARG2       arithmetic quotient of ARG1 divided by ARG2\n  ARG1 % ARG2       arithmetic remainder of ARG1 divided by ARG2\n\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            stdout,
        );
        fputs_unlocked(
            dcgettext(
                0 as *const libc::c_char,
                b"\n  STRING : REGEXP   anchored pattern match of REGEXP in STRING\n\n  match STRING REGEXP        same as STRING : REGEXP\n  substr STRING POS LENGTH   substring of STRING, POS counted from 1\n  index STRING CHARS         index in STRING where any CHARS is found, or 0\n  length STRING              length of STRING\n\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            stdout,
        );
        fputs_unlocked(
            dcgettext(
                0 as *const libc::c_char,
                b"  + TOKEN                    interpret TOKEN as a string, even if it is a\n                               keyword like 'match' or an operator like '/'\n\n  ( EXPRESSION )             value of EXPRESSION\n\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            stdout,
        );
        fputs_unlocked(
            dcgettext(
                0 as *const libc::c_char,
                b"\nBeware that many operators need to be escaped or quoted for shells.\nComparisons are arithmetic if both ARGs are numbers, else lexicographical.\nPattern matches return the string matched between \\( and \\) or null; if\n\\( and \\) are not used, they return the number of characters matched or 0.\n\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            stdout,
        );
        fputs_unlocked(
            dcgettext(
                0 as *const libc::c_char,
                b"\nExit status is 0 if EXPRESSION is neither null nor 0, 1 if EXPRESSION is null\nor 0, 2 if EXPRESSION is syntactically invalid, and 3 if an error occurred.\n\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            stdout,
        );
        emit_ancillary_info(b"expr\0" as *const u8 as *const libc::c_char);
    }
    exit(status);
}
unsafe fn main_0(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut v: *mut VALUE = 0 as *mut VALUE;
    set_program_name(*argv.offset(0 as libc::c_int as isize));
    setlocale(6 as libc::c_int, b"\0" as *const u8 as *const libc::c_char);
    bindtextdomain(
        b"coreutils\0" as *const u8 as *const libc::c_char,
        b"/usr/local/share/locale\0" as *const u8 as *const libc::c_char,
    );
    textdomain(b"coreutils\0" as *const u8 as *const libc::c_char);
    initialize_exit_failure(EXPR_FAILURE as libc::c_int);
    atexit(Some(close_stdout as unsafe extern "C" fn() -> ()));
    parse_long_options(
        argc,
        argv,
        b"expr\0" as *const u8 as *const libc::c_char,
        b"GNU coreutils\0" as *const u8 as *const libc::c_char,
        b"9.1.193-1600\0" as *const u8 as *const libc::c_char,
        Some(usage as unsafe extern "C" fn(libc::c_int) -> ()),
        b"Mike Parker\0" as *const u8 as *const libc::c_char,
        b"James Youngman\0" as *const u8 as *const libc::c_char,
        b"Paul Eggert\0" as *const u8 as *const libc::c_char,
        0 as *mut libc::c_void as *const libc::c_char,
    );
    let mut u_argc: libc::c_uint = argc as libc::c_uint;
    if (1 as libc::c_int as libc::c_uint) < u_argc
        && strcmp(
            *argv.offset(1 as libc::c_int as isize),
            b"--\0" as *const u8 as *const libc::c_char,
        ) == 0 as libc::c_int
    {
        u_argc = u_argc.wrapping_sub(1);
        argv = argv.offset(1);
    }
    if u_argc <= 1 as libc::c_int as libc::c_uint {
        error(
            0 as libc::c_int,
            0 as libc::c_int,
            dcgettext(
                0 as *const libc::c_char,
                b"missing operand\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
        usage(EXPR_INVALID as libc::c_int);
    }
    args = argv.offset(1 as libc::c_int as isize);
    v = eval(1 as libc::c_int != 0);
    if !nomoreargs() {
        if ::core::mem::size_of::<C2RustUnnamed_16>() as libc::c_ulong != 0 {
            error(
                EXPR_INVALID as libc::c_int,
                0 as libc::c_int,
                dcgettext(
                    0 as *const libc::c_char,
                    b"syntax error: unexpected argument %s\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
                quotearg_n_style(0 as libc::c_int, locale_quoting_style, *args),
            );
            if 0 as libc::c_int != 0 {} else {
                unreachable!();
            };
        } else {
            error(
                EXPR_INVALID as libc::c_int,
                0 as libc::c_int,
                dcgettext(
                    0 as *const libc::c_char,
                    b"syntax error: unexpected argument %s\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
                quotearg_n_style(0 as libc::c_int, locale_quoting_style, *args),
            );
            if 0 as libc::c_int != 0 {} else {
                unreachable!();
            };
        };
    }
    printv(v);
    exit(null(v) as libc::c_int);
}
unsafe extern "C" fn int_value(mut i: libc::c_ulong) -> *mut VALUE {
    let mut v: *mut VALUE = xmalloc(::core::mem::size_of::<VALUE>() as libc::c_ulong)
        as *mut VALUE;
    (*v).type_0 = integer;
    __gmpz_init_set_ui(((*v).u.i).as_mut_ptr(), i);
    return v;
}
unsafe extern "C" fn str_value(mut s: *const libc::c_char) -> *mut VALUE {
    let mut v: *mut VALUE = xmalloc(::core::mem::size_of::<VALUE>() as libc::c_ulong)
        as *mut VALUE;
    (*v).type_0 = string;
    (*v).u.s = xstrdup(s);
    return v;
}
unsafe extern "C" fn freev(mut v: *mut VALUE) {
    if (*v).type_0 as libc::c_uint == string as libc::c_int as libc::c_uint {
        free((*v).u.s as *mut libc::c_void);
    } else {
        __gmpz_clear(((*v).u.i).as_mut_ptr());
    }
    free(v as *mut libc::c_void);
}
unsafe extern "C" fn printv(mut v: *mut VALUE) {
    match (*v).type_0 as libc::c_uint {
        0 => {
            __gmpz_out_str(
                stdout,
                10 as libc::c_int,
                ((*v).u.i).as_mut_ptr() as mpz_srcptr,
            );
            putchar_unlocked('\n' as i32);
        }
        1 => {
            puts((*v).u.s);
        }
        _ => {
            abort();
        }
    };
}
unsafe extern "C" fn null(mut v: *mut VALUE) -> bool {
    match (*v).type_0 as libc::c_uint {
        0 => {
            return (if (*((*v).u.i).as_mut_ptr())._mp_size < 0 as libc::c_int {
                -(1 as libc::c_int)
            } else {
                ((*((*v).u.i).as_mut_ptr())._mp_size > 0 as libc::c_int) as libc::c_int
            }) == 0 as libc::c_int;
        }
        1 => {
            let mut cp: *const libc::c_char = (*v).u.s;
            if *cp as libc::c_int == '\0' as i32 {
                return 1 as libc::c_int != 0;
            }
            cp = cp.offset((*cp as libc::c_int == '-' as i32) as libc::c_int as isize);
            loop {
                if *cp as libc::c_int != '0' as i32 {
                    return 0 as libc::c_int != 0;
                }
                cp = cp.offset(1);
                if !(*cp != 0) {
                    break;
                }
            }
            return 1 as libc::c_int != 0;
        }
        _ => {
            abort();
        }
    };
}
unsafe extern "C" fn looks_like_integer(mut cp: *const libc::c_char) -> bool {
    cp = cp.offset((*cp as libc::c_int == '-' as i32) as libc::c_int as isize);
    loop {
        if !((*cp as libc::c_uint).wrapping_sub('0' as i32 as libc::c_uint)
            <= 9 as libc::c_int as libc::c_uint)
        {
            return 0 as libc::c_int != 0;
        }
        cp = cp.offset(1);
        if !(*cp != 0) {
            break;
        }
    }
    return 1 as libc::c_int != 0;
}
unsafe extern "C" fn tostring(mut v: *mut VALUE) {
    match (*v).type_0 as libc::c_uint {
        0 => {
            let mut s: *mut libc::c_char = __gmpz_get_str(
                0 as *mut libc::c_char,
                10 as libc::c_int,
                ((*v).u.i).as_mut_ptr() as mpz_srcptr,
            );
            __gmpz_clear(((*v).u.i).as_mut_ptr());
            (*v).u.s = s;
            (*v).type_0 = string;
        }
        1 => {}
        _ => {
            abort();
        }
    };
}
unsafe extern "C" fn toarith(mut v: *mut VALUE) -> bool {
    match (*v).type_0 as libc::c_uint {
        0 => return 1 as libc::c_int != 0,
        1 => {
            let mut s: *mut libc::c_char = (*v).u.s;
            if !looks_like_integer(s) {
                return 0 as libc::c_int != 0;
            }
            if __gmpz_init_set_str(((*v).u.i).as_mut_ptr(), s, 10 as libc::c_int)
                != 0 as libc::c_int
            {
                if ::core::mem::size_of::<C2RustUnnamed_6>() as libc::c_ulong != 0 {
                    error(
                        EXPR_FAILURE as libc::c_int,
                        34 as libc::c_int,
                        b"%s\0" as *const u8 as *const libc::c_char,
                        s,
                    );
                    if 0 as libc::c_int != 0 {} else {
                        unreachable!();
                    };
                } else {
                    error(
                        EXPR_FAILURE as libc::c_int,
                        34 as libc::c_int,
                        b"%s\0" as *const u8 as *const libc::c_char,
                        s,
                    );
                    if 0 as libc::c_int != 0 {} else {
                        unreachable!();
                    };
                };
            }
            free(s as *mut libc::c_void);
            (*v).type_0 = integer;
            return 1 as libc::c_int != 0;
        }
        _ => {
            abort();
        }
    };
}
unsafe extern "C" fn getsize(mut i: *mut __mpz_struct) -> size_t {
    if (if (*i)._mp_size < 0 as libc::c_int {
        -(1 as libc::c_int)
    } else {
        ((*i)._mp_size > 0 as libc::c_int) as libc::c_int
    }) < 0 as libc::c_int
    {
        return 18446744073709551615 as libc::c_ulong;
    }
    if __gmpz_fits_ulong_p(i as mpz_srcptr) != 0 {
        let mut ul: libc::c_ulong = __gmpz_get_ui(i as mpz_srcptr);
        if ul < 18446744073709551615 as libc::c_ulong {
            return ul;
        }
    }
    return (18446744073709551615 as libc::c_ulong)
        .wrapping_sub(1 as libc::c_int as libc::c_ulong);
}
unsafe extern "C" fn nextarg(mut str: *const libc::c_char) -> bool {
    if (*args).is_null() {
        return 0 as libc::c_int != 0
    } else {
        let mut r: bool = strcmp(*args, str) == 0 as libc::c_int;
        args = args.offset(r as libc::c_int as isize);
        return r;
    };
}
unsafe extern "C" fn nomoreargs() -> bool {
    return (*args).is_null();
}
unsafe extern "C" fn require_more_args() {
    if nomoreargs() {
        if ::core::mem::size_of::<C2RustUnnamed_15>() as libc::c_ulong != 0 {
            error(
                EXPR_INVALID as libc::c_int,
                0 as libc::c_int,
                dcgettext(
                    0 as *const libc::c_char,
                    b"syntax error: missing argument after %s\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
                quotearg_n_style(
                    0 as libc::c_int,
                    locale_quoting_style,
                    *args.offset(-(1 as libc::c_int as isize)),
                ),
            );
            if 0 as libc::c_int != 0 {} else {
                unreachable!();
            };
        } else {
            error(
                EXPR_INVALID as libc::c_int,
                0 as libc::c_int,
                dcgettext(
                    0 as *const libc::c_char,
                    b"syntax error: missing argument after %s\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
                quotearg_n_style(
                    0 as libc::c_int,
                    locale_quoting_style,
                    *args.offset(-(1 as libc::c_int as isize)),
                ),
            );
            if 0 as libc::c_int != 0 {} else {
                unreachable!();
            };
        };
    }
}
unsafe extern "C" fn docolon(mut sv: *mut VALUE, mut pv: *mut VALUE) -> *mut VALUE {
    let mut v: *mut VALUE = 0 as *mut VALUE;
    let mut errmsg: *const libc::c_char = 0 as *const libc::c_char;
    let mut re_buffer: re_pattern_buffer = re_pattern_buffer {
        buffer: 0 as *mut re_dfa_t,
        allocated: 0,
        used: 0,
        syntax: 0,
        fastmap: 0 as *mut libc::c_char,
        translate: 0 as *mut libc::c_uchar,
        re_nsub: 0,
        can_be_null_regs_allocated_fastmap_accurate_no_sub_not_bol_not_eol_newline_anchor: [0; 1],
        c2rust_padding: [0; 7],
    };
    let mut fastmap: [libc::c_char; 256] = [0; 256];
    let mut re_regs: re_registers = re_registers {
        num_regs: 0,
        start: 0 as *mut regoff_t,
        end: 0 as *mut regoff_t,
    };
    let mut matchlen: regoff_t = 0;
    tostring(sv);
    tostring(pv);
    re_regs.num_regs = 0 as libc::c_int as __re_size_t;
    re_regs.start = 0 as *mut regoff_t;
    re_regs.end = 0 as *mut regoff_t;
    re_buffer.buffer = 0 as *mut re_dfa_t;
    re_buffer.allocated = 0 as libc::c_int as __re_long_size_t;
    re_buffer.fastmap = fastmap.as_mut_ptr();
    re_buffer.translate = 0 as *mut libc::c_uchar;
    re_syntax_options = (((1 as libc::c_int as libc::c_ulong) << 1 as libc::c_int)
        << 1 as libc::c_int
        | ((((((1 as libc::c_int as libc::c_ulong) << 1 as libc::c_int)
            << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
            << 1 as libc::c_int) << 1 as libc::c_int
        | (((((((1 as libc::c_int as libc::c_ulong) << 1 as libc::c_int)
            << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
            << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int
        | (((((((((1 as libc::c_int as libc::c_ulong) << 1 as libc::c_int)
            << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
            << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
            << 1 as libc::c_int) << 1 as libc::c_int
        | ((((((((((((((((1 as libc::c_int as libc::c_ulong) << 1 as libc::c_int)
            << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
            << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
            << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
            << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
            << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int
        | (1 as libc::c_int as libc::c_ulong) << 1 as libc::c_int
        | ((((((((((((((((((((((((1 as libc::c_int as libc::c_ulong) << 1 as libc::c_int)
            << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
            << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
            << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
            << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
            << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
            << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
            << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
            << 1 as libc::c_int) << 1 as libc::c_int)
        & !(((((((((((((((((((((((((1 as libc::c_int as libc::c_ulong)
            << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
            << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
            << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
            << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
            << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
            << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
            << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
            << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
        & !(((((((((((((((((1 as libc::c_int as libc::c_ulong) << 1 as libc::c_int)
            << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
            << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
            << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
            << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
            << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int);
    errmsg = re_compile_pattern((*pv).u.s, strlen((*pv).u.s), &mut re_buffer);
    if !errmsg.is_null() {
        if ::core::mem::size_of::<C2RustUnnamed_11>() as libc::c_ulong != 0 {
            error(
                EXPR_INVALID as libc::c_int,
                0 as libc::c_int,
                b"%s\0" as *const u8 as *const libc::c_char,
                errmsg,
            );
            if 0 as libc::c_int != 0 {} else {
                unreachable!();
            };
        } else {
            error(
                EXPR_INVALID as libc::c_int,
                0 as libc::c_int,
                b"%s\0" as *const u8 as *const libc::c_char,
                errmsg,
            );
            if 0 as libc::c_int != 0 {} else {
                unreachable!();
            };
        };
    }
    re_buffer.set_newline_anchor(0 as libc::c_int as libc::c_uint);
    matchlen = re_match(
        &mut re_buffer,
        (*sv).u.s,
        strlen((*sv).u.s) as regoff_t,
        0 as libc::c_int,
        &mut re_regs,
    );
    if 0 as libc::c_int <= matchlen {
        if re_buffer.re_nsub > 0 as libc::c_int as libc::c_ulong {
            if *(re_regs.end).offset(1 as libc::c_int as isize) < 0 as libc::c_int {
                v = str_value(b"\0" as *const u8 as *const libc::c_char);
            } else {
                *((*sv).u.s)
                    .offset(
                        *(re_regs.end).offset(1 as libc::c_int as isize) as isize,
                    ) = '\0' as i32 as libc::c_char;
                v = str_value(
                    ((*sv).u.s)
                        .offset(
                            *(re_regs.start).offset(1 as libc::c_int as isize) as isize,
                        ),
                );
            }
        } else {
            let mut i: size_t = if __ctype_get_mb_cur_max()
                == 1 as libc::c_int as libc::c_ulong
            {
                matchlen as libc::c_ulong
            } else {
                mbs_offset_to_chars((*sv).u.s, matchlen as size_t)
            };
            v = int_value(i);
        }
    } else if matchlen == -(1 as libc::c_int) {
        if re_buffer.re_nsub > 0 as libc::c_int as libc::c_ulong {
            v = str_value(b"\0" as *const u8 as *const libc::c_char);
        } else {
            v = int_value(0 as libc::c_int as libc::c_ulong);
        }
    } else {
        if ::core::mem::size_of::<C2RustUnnamed_10>() as libc::c_ulong != 0 {
            error(
                EXPR_FAILURE as libc::c_int,
                (if matchlen == -(2 as libc::c_int) {
                    *__errno_location()
                } else {
                    75 as libc::c_int
                }),
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
                EXPR_FAILURE as libc::c_int,
                (if matchlen == -(2 as libc::c_int) {
                    *__errno_location()
                } else {
                    75 as libc::c_int
                }),
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
    if (0 as libc::c_int as libc::c_uint) < re_regs.num_regs {
        free(re_regs.start as *mut libc::c_void);
        free(re_regs.end as *mut libc::c_void);
    }
    re_buffer.fastmap = 0 as *mut libc::c_char;
    regfree(&mut re_buffer);
    return v;
}
unsafe extern "C" fn eval7(mut evaluate: bool) -> *mut VALUE {
    let mut v: *mut VALUE = 0 as *mut VALUE;
    require_more_args();
    if nextarg(b"(\0" as *const u8 as *const libc::c_char) {
        v = eval(evaluate);
        if nomoreargs() {
            if ::core::mem::size_of::<C2RustUnnamed_14>() as libc::c_ulong != 0 {
                error(
                    EXPR_INVALID as libc::c_int,
                    0 as libc::c_int,
                    dcgettext(
                        0 as *const libc::c_char,
                        b"syntax error: expecting ')' after %s\0" as *const u8
                            as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    quotearg_n_style(
                        0 as libc::c_int,
                        locale_quoting_style,
                        *args.offset(-(1 as libc::c_int as isize)),
                    ),
                );
                if 0 as libc::c_int != 0 {} else {
                    unreachable!();
                };
            } else {
                error(
                    EXPR_INVALID as libc::c_int,
                    0 as libc::c_int,
                    dcgettext(
                        0 as *const libc::c_char,
                        b"syntax error: expecting ')' after %s\0" as *const u8
                            as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    quotearg_n_style(
                        0 as libc::c_int,
                        locale_quoting_style,
                        *args.offset(-(1 as libc::c_int as isize)),
                    ),
                );
                if 0 as libc::c_int != 0 {} else {
                    unreachable!();
                };
            };
        }
        if !nextarg(b")\0" as *const u8 as *const libc::c_char) {
            if ::core::mem::size_of::<C2RustUnnamed_13>() as libc::c_ulong != 0 {
                error(
                    EXPR_INVALID as libc::c_int,
                    0 as libc::c_int,
                    dcgettext(
                        0 as *const libc::c_char,
                        b"syntax error: expecting ')' instead of %s\0" as *const u8
                            as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    quotearg_n_style(0 as libc::c_int, locale_quoting_style, *args),
                );
                if 0 as libc::c_int != 0 {} else {
                    unreachable!();
                };
            } else {
                error(
                    EXPR_INVALID as libc::c_int,
                    0 as libc::c_int,
                    dcgettext(
                        0 as *const libc::c_char,
                        b"syntax error: expecting ')' instead of %s\0" as *const u8
                            as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    quotearg_n_style(0 as libc::c_int, locale_quoting_style, *args),
                );
                if 0 as libc::c_int != 0 {} else {
                    unreachable!();
                };
            };
        }
        return v;
    }
    if nextarg(b")\0" as *const u8 as *const libc::c_char) {
        if ::core::mem::size_of::<C2RustUnnamed_12>() as libc::c_ulong != 0 {
            error(
                EXPR_INVALID as libc::c_int,
                0 as libc::c_int,
                dcgettext(
                    0 as *const libc::c_char,
                    b"syntax error: unexpected ')'\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
            );
            if 0 as libc::c_int != 0 {} else {
                unreachable!();
            };
        } else {
            error(
                EXPR_INVALID as libc::c_int,
                0 as libc::c_int,
                dcgettext(
                    0 as *const libc::c_char,
                    b"syntax error: unexpected ')'\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
            );
            if 0 as libc::c_int != 0 {} else {
                unreachable!();
            };
        };
    }
    let fresh1 = args;
    args = args.offset(1);
    return str_value(*fresh1);
}
unsafe extern "C" fn eval6(mut evaluate: bool) -> *mut VALUE {
    let mut l: *mut VALUE = 0 as *mut VALUE;
    let mut r: *mut VALUE = 0 as *mut VALUE;
    let mut v: *mut VALUE = 0 as *mut VALUE;
    let mut i1: *mut VALUE = 0 as *mut VALUE;
    let mut i2: *mut VALUE = 0 as *mut VALUE;
    if nextarg(b"+\0" as *const u8 as *const libc::c_char) {
        require_more_args();
        let fresh2 = args;
        args = args.offset(1);
        return str_value(*fresh2);
    } else if nextarg(b"length\0" as *const u8 as *const libc::c_char) {
        r = eval6(evaluate);
        tostring(r);
        v = int_value(mbslen((*r).u.s));
        freev(r);
        return v;
    } else if nextarg(b"match\0" as *const u8 as *const libc::c_char) {
        l = eval6(evaluate);
        r = eval6(evaluate);
        if evaluate {
            v = docolon(l, r);
            freev(l);
        } else {
            v = l;
        }
        freev(r);
        return v;
    } else if nextarg(b"index\0" as *const u8 as *const libc::c_char) {
        let mut pos: size_t = 0;
        l = eval6(evaluate);
        r = eval6(evaluate);
        tostring(l);
        tostring(r);
        pos = mbs_logical_cspn((*l).u.s, (*r).u.s);
        v = int_value(pos);
        freev(l);
        freev(r);
        return v;
    } else if nextarg(b"substr\0" as *const u8 as *const libc::c_char) {
        l = eval6(evaluate);
        i1 = eval6(evaluate);
        i2 = eval6(evaluate);
        tostring(l);
        if !toarith(i1) || !toarith(i2) {
            v = str_value(b"\0" as *const u8 as *const libc::c_char);
        } else {
            let mut pos_0: size_t = getsize(((*i1).u.i).as_mut_ptr());
            let mut len: size_t = getsize(((*i2).u.i).as_mut_ptr());
            let mut s: *mut libc::c_char = mbs_logical_substr((*l).u.s, pos_0, len);
            v = str_value(s);
            free(s as *mut libc::c_void);
        }
        freev(l);
        freev(i1);
        freev(i2);
        return v;
    } else {
        return eval7(evaluate)
    };
}
unsafe extern "C" fn eval5(mut evaluate: bool) -> *mut VALUE {
    let mut l: *mut VALUE = 0 as *mut VALUE;
    let mut r: *mut VALUE = 0 as *mut VALUE;
    let mut v: *mut VALUE = 0 as *mut VALUE;
    l = eval6(evaluate);
    loop {
        if nextarg(b":\0" as *const u8 as *const libc::c_char) {
            r = eval6(evaluate);
            if evaluate {
                v = docolon(l, r);
                freev(l);
                l = v;
            }
            freev(r);
        } else {
            return l
        }
    };
}
unsafe extern "C" fn eval4(mut evaluate: bool) -> *mut VALUE {
    let mut l: *mut VALUE = 0 as *mut VALUE;
    let mut r: *mut VALUE = 0 as *mut VALUE;
    let mut fxn: C2RustUnnamed_7 = multiply;
    l = eval5(evaluate);
    loop {
        if nextarg(b"*\0" as *const u8 as *const libc::c_char) {
            fxn = multiply;
        } else if nextarg(b"/\0" as *const u8 as *const libc::c_char) {
            fxn = divide;
        } else if nextarg(b"%\0" as *const u8 as *const libc::c_char) {
            fxn = mod_0;
        } else {
            return l
        }
        r = eval5(evaluate);
        if evaluate {
            if !toarith(l) || !toarith(r) {
                if ::core::mem::size_of::<C2RustUnnamed_9>() as libc::c_ulong != 0 {
                    error(
                        EXPR_INVALID as libc::c_int,
                        0 as libc::c_int,
                        dcgettext(
                            0 as *const libc::c_char,
                            b"non-integer argument\0" as *const u8
                                as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                    );
                    if 0 as libc::c_int != 0 {} else {
                        unreachable!();
                    };
                } else {
                    error(
                        EXPR_INVALID as libc::c_int,
                        0 as libc::c_int,
                        dcgettext(
                            0 as *const libc::c_char,
                            b"non-integer argument\0" as *const u8
                                as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                    );
                    if 0 as libc::c_int != 0 {} else {
                        unreachable!();
                    };
                };
            }
            if fxn as libc::c_uint != multiply as libc::c_int as libc::c_uint
                && (if (*((*r).u.i).as_mut_ptr())._mp_size < 0 as libc::c_int {
                    -(1 as libc::c_int)
                } else {
                    ((*((*r).u.i).as_mut_ptr())._mp_size > 0 as libc::c_int)
                        as libc::c_int
                }) == 0 as libc::c_int
            {
                if ::core::mem::size_of::<C2RustUnnamed_8>() as libc::c_ulong != 0 {
                    error(
                        EXPR_INVALID as libc::c_int,
                        0 as libc::c_int,
                        dcgettext(
                            0 as *const libc::c_char,
                            b"division by zero\0" as *const u8 as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                    );
                    if 0 as libc::c_int != 0 {} else {
                        unreachable!();
                    };
                } else {
                    error(
                        EXPR_INVALID as libc::c_int,
                        0 as libc::c_int,
                        dcgettext(
                            0 as *const libc::c_char,
                            b"division by zero\0" as *const u8 as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                    );
                    if 0 as libc::c_int != 0 {} else {
                        unreachable!();
                    };
                };
            }
            if fxn as libc::c_uint == multiply as libc::c_int as libc::c_uint {
                Some(
                    __gmpz_mul
                        as unsafe extern "C" fn(mpz_ptr, mpz_srcptr, mpz_srcptr) -> (),
                )
            } else if fxn as libc::c_uint == divide as libc::c_int as libc::c_uint {
                Some(
                    __gmpz_tdiv_q
                        as unsafe extern "C" fn(mpz_ptr, mpz_srcptr, mpz_srcptr) -> (),
                )
            } else {
                Some(
                    __gmpz_tdiv_r
                        as unsafe extern "C" fn(mpz_ptr, mpz_srcptr, mpz_srcptr) -> (),
                )
            }
                .expect(
                    "non-null function pointer",
                )(
                ((*l).u.i).as_mut_ptr(),
                ((*l).u.i).as_mut_ptr() as mpz_srcptr,
                ((*r).u.i).as_mut_ptr() as mpz_srcptr,
            );
        }
        freev(r);
    };
}
unsafe extern "C" fn eval3(mut evaluate: bool) -> *mut VALUE {
    let mut l: *mut VALUE = 0 as *mut VALUE;
    let mut r: *mut VALUE = 0 as *mut VALUE;
    let mut fxn: C2RustUnnamed_4 = plus;
    l = eval4(evaluate);
    loop {
        if nextarg(b"+\0" as *const u8 as *const libc::c_char) {
            fxn = plus;
        } else if nextarg(b"-\0" as *const u8 as *const libc::c_char) {
            fxn = minus;
        } else {
            return l
        }
        r = eval4(evaluate);
        if evaluate {
            if !toarith(l) || !toarith(r) {
                if ::core::mem::size_of::<C2RustUnnamed_5>() as libc::c_ulong != 0 {
                    error(
                        EXPR_INVALID as libc::c_int,
                        0 as libc::c_int,
                        dcgettext(
                            0 as *const libc::c_char,
                            b"non-integer argument\0" as *const u8
                                as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                    );
                    if 0 as libc::c_int != 0 {} else {
                        unreachable!();
                    };
                } else {
                    error(
                        EXPR_INVALID as libc::c_int,
                        0 as libc::c_int,
                        dcgettext(
                            0 as *const libc::c_char,
                            b"non-integer argument\0" as *const u8
                                as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                    );
                    if 0 as libc::c_int != 0 {} else {
                        unreachable!();
                    };
                };
            }
            if fxn as libc::c_uint == plus as libc::c_int as libc::c_uint {
                Some(
                    __gmpz_add
                        as unsafe extern "C" fn(mpz_ptr, mpz_srcptr, mpz_srcptr) -> (),
                )
            } else {
                Some(
                    __gmpz_sub
                        as unsafe extern "C" fn(mpz_ptr, mpz_srcptr, mpz_srcptr) -> (),
                )
            }
                .expect(
                    "non-null function pointer",
                )(
                ((*l).u.i).as_mut_ptr(),
                ((*l).u.i).as_mut_ptr() as mpz_srcptr,
                ((*r).u.i).as_mut_ptr() as mpz_srcptr,
            );
        }
        freev(r);
    };
}
unsafe extern "C" fn eval2(mut evaluate: bool) -> *mut VALUE {
    let mut l: *mut VALUE = 0 as *mut VALUE;
    l = eval3(evaluate);
    loop {
        let mut r: *mut VALUE = 0 as *mut VALUE;
        let mut fxn: C2RustUnnamed_2 = less_than;
        let mut val: bool = 0 as libc::c_int != 0;
        if nextarg(b"<\0" as *const u8 as *const libc::c_char) {
            fxn = less_than;
        } else if nextarg(b"<=\0" as *const u8 as *const libc::c_char) {
            fxn = less_equal;
        } else if nextarg(b"=\0" as *const u8 as *const libc::c_char) as libc::c_int != 0
            || nextarg(b"==\0" as *const u8 as *const libc::c_char) as libc::c_int != 0
        {
            fxn = equal;
        } else if nextarg(b"!=\0" as *const u8 as *const libc::c_char) {
            fxn = not_equal;
        } else if nextarg(b">=\0" as *const u8 as *const libc::c_char) {
            fxn = greater_equal;
        } else if nextarg(b">\0" as *const u8 as *const libc::c_char) {
            fxn = greater_than;
        } else {
            return l
        }
        r = eval3(evaluate);
        if evaluate {
            let mut cmp: libc::c_int = 0;
            tostring(l);
            tostring(r);
            if looks_like_integer((*l).u.s) as libc::c_int != 0
                && looks_like_integer((*r).u.s) as libc::c_int != 0
            {
                cmp = strintcmp((*l).u.s, (*r).u.s);
            } else {
                *__errno_location() = 0 as libc::c_int;
                cmp = strcoll((*l).u.s, (*r).u.s);
                if *__errno_location() != 0 {
                    error(
                        0 as libc::c_int,
                        *__errno_location(),
                        dcgettext(
                            0 as *const libc::c_char,
                            b"string comparison failed\0" as *const u8
                                as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                    );
                    error(
                        0 as libc::c_int,
                        0 as libc::c_int,
                        dcgettext(
                            0 as *const libc::c_char,
                            b"set LC_ALL='C' to work around the problem\0" as *const u8
                                as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                    );
                    if ::core::mem::size_of::<C2RustUnnamed_3>() as libc::c_ulong != 0 {
                        error(
                            EXPR_INVALID as libc::c_int,
                            0 as libc::c_int,
                            dcgettext(
                                0 as *const libc::c_char,
                                b"the strings compared were %s and %s\0" as *const u8
                                    as *const libc::c_char,
                                5 as libc::c_int,
                            ),
                            quotearg_n_style(
                                0 as libc::c_int,
                                locale_quoting_style,
                                (*l).u.s,
                            ),
                            quotearg_n_style(
                                1 as libc::c_int,
                                locale_quoting_style,
                                (*r).u.s,
                            ),
                        );
                        if 0 as libc::c_int != 0 {} else {
                            unreachable!();
                        };
                    } else {
                        error(
                            EXPR_INVALID as libc::c_int,
                            0 as libc::c_int,
                            dcgettext(
                                0 as *const libc::c_char,
                                b"the strings compared were %s and %s\0" as *const u8
                                    as *const libc::c_char,
                                5 as libc::c_int,
                            ),
                            quotearg_n_style(
                                0 as libc::c_int,
                                locale_quoting_style,
                                (*l).u.s,
                            ),
                            quotearg_n_style(
                                1 as libc::c_int,
                                locale_quoting_style,
                                (*r).u.s,
                            ),
                        );
                        if 0 as libc::c_int != 0 {} else {
                            unreachable!();
                        };
                    };
                }
            }
            match fxn as libc::c_uint {
                0 => {
                    val = cmp < 0 as libc::c_int;
                }
                1 => {
                    val = cmp <= 0 as libc::c_int;
                }
                2 => {
                    val = cmp == 0 as libc::c_int;
                }
                3 => {
                    val = cmp != 0 as libc::c_int;
                }
                4 => {
                    val = cmp >= 0 as libc::c_int;
                }
                5 => {
                    val = cmp > 0 as libc::c_int;
                }
                _ => {
                    abort();
                }
            }
        }
        freev(l);
        freev(r);
        l = int_value(val as libc::c_ulong);
    };
}
unsafe extern "C" fn eval1(mut evaluate: bool) -> *mut VALUE {
    let mut l: *mut VALUE = 0 as *mut VALUE;
    let mut r: *mut VALUE = 0 as *mut VALUE;
    l = eval2(evaluate);
    loop {
        if nextarg(b"&\0" as *const u8 as *const libc::c_char) {
            r = eval2(evaluate as libc::c_int != 0 && !null(l));
            if null(l) as libc::c_int != 0 || null(r) as libc::c_int != 0 {
                freev(l);
                freev(r);
                l = int_value(0 as libc::c_int as libc::c_ulong);
            } else {
                freev(r);
            }
        } else {
            return l
        }
    };
}
unsafe extern "C" fn eval(mut evaluate: bool) -> *mut VALUE {
    let mut l: *mut VALUE = 0 as *mut VALUE;
    let mut r: *mut VALUE = 0 as *mut VALUE;
    l = eval1(evaluate);
    loop {
        if nextarg(b"|\0" as *const u8 as *const libc::c_char) {
            r = eval1(evaluate as libc::c_int != 0 && null(l) as libc::c_int != 0);
            if null(l) {
                freev(l);
                l = r;
                if null(l) {
                    freev(l);
                    l = int_value(0 as libc::c_int as libc::c_ulong);
                }
            } else {
                freev(r);
            }
        } else {
            return l
        }
    };
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
