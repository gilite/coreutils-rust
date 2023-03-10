#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![feature(extern_types, label_break_value)]
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
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn __overflow(_: *mut FILE, _: libc::c_int) -> libc::c_int;
    fn fputs_unlocked(__s: *const libc::c_char, __stream: *mut FILE) -> libc::c_int;
    fn fwrite_unlocked(
        __ptr: *const libc::c_void,
        __size: size_t,
        __n: size_t,
        __stream: *mut FILE,
    ) -> size_t;
    static mut optind: libc::c_int;
    fn getopt_long(
        ___argc: libc::c_int,
        ___argv: *const *mut libc::c_char,
        __shortopts: *const libc::c_char,
        __longopts: *const option,
        __longind: *mut libc::c_int,
    ) -> libc::c_int;
    fn close(__fd: libc::c_int) -> libc::c_int;
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
    fn abort() -> !;
    fn atexit(__func: Option::<unsafe extern "C" fn() -> ()>) -> libc::c_int;
    fn exit(_: libc::c_int) -> !;
    static mut Version: *const libc::c_char;
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
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
    fn textdomain(__domainname: *const libc::c_char) -> *mut libc::c_char;
    fn bindtextdomain(
        __domainname: *const libc::c_char,
        __dirname: *const libc::c_char,
    ) -> *mut libc::c_char;
    fn xmalloc(s: size_t) -> *mut libc::c_void;
    fn xcalloc(n: size_t, s: size_t) -> *mut libc::c_void;
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
    fn error(
        __status: libc::c_int,
        __errnum: libc::c_int,
        __format: *const libc::c_char,
        _: ...
    );
    fn fadvise(fp: *mut FILE, advice: fadvice_t);
    fn quote(arg: *const libc::c_char) -> *const libc::c_char;
    fn safe_read(fd: libc::c_int, buf: *mut libc::c_void, count: size_t) -> size_t;
    fn xstrtoumax(
        _: *const libc::c_char,
        _: *mut *mut libc::c_char,
        _: libc::c_int,
        _: *mut uintmax_t,
        _: *const libc::c_char,
    ) -> strtol_error;
}
pub type size_t = libc::c_ulong;
pub type __int32_t = libc::c_int;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct option {
    pub name: *const libc::c_char,
    pub has_arg: libc::c_int,
    pub flag: *mut libc::c_int,
    pub val: libc::c_int,
}
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
pub type C2RustUnnamed_0 = libc::c_int;
pub const GETOPT_VERSION_CHAR: C2RustUnnamed_0 = -3;
pub const GETOPT_HELP_CHAR: C2RustUnnamed_0 = -2;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct infomap {
    pub program: *const libc::c_char,
    pub node: *const libc::c_char,
}
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
pub type C2RustUnnamed_1 = libc::c_uint;
pub const N_CHARS: C2RustUnnamed_1 = 256;
pub type count = uintmax_t;
pub type Char_class = libc::c_uint;
pub const CC_NO_CLASS: Char_class = 9999;
pub const CC_XDIGIT: Char_class = 11;
pub const CC_UPPER: Char_class = 10;
pub const CC_SPACE: Char_class = 9;
pub const CC_PUNCT: Char_class = 8;
pub const CC_PRINT: Char_class = 7;
pub const CC_LOWER: Char_class = 6;
pub const CC_GRAPH: Char_class = 5;
pub const CC_DIGIT: Char_class = 4;
pub const CC_CNTRL: Char_class = 3;
pub const CC_BLANK: Char_class = 2;
pub const CC_ALPHA: Char_class = 1;
pub const CC_ALNUM: Char_class = 0;
pub type Upper_Lower_class = libc::c_uint;
pub const UL_NONE: Upper_Lower_class = 2;
pub const UL_UPPER: Upper_Lower_class = 1;
pub const UL_LOWER: Upper_Lower_class = 0;
pub type Range_element_type = libc::c_uint;
pub const RE_REPEATED_CHAR: Range_element_type = 4;
pub const RE_EQUIV_CLASS: Range_element_type = 3;
pub const RE_CHAR_CLASS: Range_element_type = 2;
pub const RE_RANGE: Range_element_type = 1;
pub const RE_NORMAL_CHAR: Range_element_type = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct List_element {
    pub type_0: Range_element_type,
    pub next: *mut List_element,
    pub u: C2RustUnnamed_2,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_2 {
    pub normal_char: libc::c_uchar,
    pub range: C2RustUnnamed_4,
    pub char_class: Char_class,
    pub equiv_code: libc::c_uchar,
    pub repeated_char: C2RustUnnamed_3,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_3 {
    pub the_repeated_char: libc::c_uchar,
    pub repeat_count: count,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_4 {
    pub first_char: libc::c_uchar,
    pub last_char: libc::c_uchar,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Spec_list {
    pub head: *mut List_element,
    pub tail: *mut List_element,
    pub state: count,
    pub length: count,
    pub n_indefinite_repeats: size_t,
    pub indefinite_repeat_element: *mut List_element,
    pub has_equiv_class: bool,
    pub has_char_class: bool,
    pub has_restricted_char_class: bool,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct E_string {
    pub s: *mut libc::c_char,
    pub escaped: *mut bool,
    pub len: size_t,
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
#[inline]
unsafe extern "C" fn es_match(
    mut es: *const E_string,
    mut i: size_t,
    mut c: libc::c_char,
) -> bool {
    return *((*es).s).offset(i as isize) as libc::c_int == c as libc::c_int
        && !*((*es).escaped).offset(i as isize);
}
static mut squeeze_repeats: bool = 0 as libc::c_int != 0;
static mut delete: bool = 0 as libc::c_int != 0;
static mut complement: bool = 0 as libc::c_int != 0;
static mut truncate_set1: bool = 0 as libc::c_int != 0;
static mut translating: bool = false;
static mut io_buf: [libc::c_char; 8192] = [0; 8192];
static mut char_class_name: [*const libc::c_char; 12] = [
    b"alnum\0" as *const u8 as *const libc::c_char,
    b"alpha\0" as *const u8 as *const libc::c_char,
    b"blank\0" as *const u8 as *const libc::c_char,
    b"cntrl\0" as *const u8 as *const libc::c_char,
    b"digit\0" as *const u8 as *const libc::c_char,
    b"graph\0" as *const u8 as *const libc::c_char,
    b"lower\0" as *const u8 as *const libc::c_char,
    b"print\0" as *const u8 as *const libc::c_char,
    b"punct\0" as *const u8 as *const libc::c_char,
    b"space\0" as *const u8 as *const libc::c_char,
    b"upper\0" as *const u8 as *const libc::c_char,
    b"xdigit\0" as *const u8 as *const libc::c_char,
];
static mut in_squeeze_set: [bool; 256] = [false; 256];
static mut in_delete_set: [bool; 256] = [false; 256];
static mut xlate: [libc::c_char; 256] = [0; 256];
static mut long_options: [option; 7] = [
    {
        let mut init = option {
            name: b"complement\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'c' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"delete\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'd' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"squeeze-repeats\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 's' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"truncate-set1\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 't' as i32,
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
                b"Usage: %s [OPTION]... STRING1 [STRING2]\n\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
            program_name,
        );
        fputs_unlocked(
            dcgettext(
                0 as *const libc::c_char,
                b"Translate, squeeze, and/or delete characters from standard input,\nwriting to standard output.  STRING1 and STRING2 specify arrays of\ncharacters ARRAY1 and ARRAY2 that control the action.\n\n  -c, -C, --complement    use the complement of ARRAY1\n  -d, --delete            delete characters in ARRAY1, do not translate\n  -s, --squeeze-repeats   replace each sequence of a repeated character\n                            that is listed in the last specified ARRAY,\n                            with a single occurrence of that character\n  -t, --truncate-set1     first truncate ARRAY1 to length of ARRAY2\n\0"
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
                b"\nARRAYs are specified as strings of characters.  Most represent themselves.\nInterpreted sequences are:\n\n  \\NNN            character with octal value NNN (1 to 3 octal digits)\n  \\\\              backslash\n  \\a              audible BEL\n  \\b              backspace\n  \\f              form feed\n  \\n              new line\n  \\r              return\n  \\t              horizontal tab\n\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            stdout,
        );
        fputs_unlocked(
            dcgettext(
                0 as *const libc::c_char,
                b"  \\v              vertical tab\n  CHAR1-CHAR2     all characters from CHAR1 to CHAR2 in ascending order\n  [CHAR*]         in ARRAY2, copies of CHAR until length of ARRAY1\n  [CHAR*REPEAT]   REPEAT copies of CHAR, REPEAT octal if starting with 0\n  [:alnum:]       all letters and digits\n  [:alpha:]       all letters\n  [:blank:]       all horizontal whitespace\n  [:cntrl:]       all control characters\n  [:digit:]       all digits\n\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            stdout,
        );
        fputs_unlocked(
            dcgettext(
                0 as *const libc::c_char,
                b"  [:graph:]       all printable characters, not including space\n  [:lower:]       all lower case letters\n  [:print:]       all printable characters, including space\n  [:punct:]       all punctuation characters\n  [:space:]       all horizontal or vertical whitespace\n  [:upper:]       all upper case letters\n  [:xdigit:]      all hexadecimal digits\n  [=CHAR=]        all characters which are equivalent to CHAR\n\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            stdout,
        );
        fputs_unlocked(
            dcgettext(
                0 as *const libc::c_char,
                b"\nTranslation occurs if -d is not given and both STRING1 and STRING2 appear.\n-t is only significant when translating.  ARRAY2 is extended to length of\nARRAY1 by repeating its last character as necessary.  Excess characters\nof ARRAY2 are ignored.  Character classes expand in unspecified order;\nwhile translating, [:lower:] and [:upper:] may be used in pairs to\nspecify case conversion.  Squeezing occurs after translation or deletion.\n\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            stdout,
        );
        emit_ancillary_info(b"tr\0" as *const u8 as *const libc::c_char);
    }
    exit(status);
}
#[inline]
unsafe extern "C" fn is_equiv_class_member(
    mut equiv_class: libc::c_uchar,
    mut c: libc::c_uchar,
) -> bool {
    return equiv_class as libc::c_int == c as libc::c_int;
}
unsafe extern "C" fn is_char_class_member(
    mut char_class: Char_class,
    mut c: libc::c_uchar,
) -> bool {
    let mut result: libc::c_int = 0;
    match char_class as libc::c_uint {
        0 => {
            result = *(*__ctype_b_loc()).offset(c as libc::c_int as isize) as libc::c_int
                & _ISalnum as libc::c_int as libc::c_ushort as libc::c_int;
        }
        1 => {
            result = *(*__ctype_b_loc()).offset(c as libc::c_int as isize) as libc::c_int
                & _ISalpha as libc::c_int as libc::c_ushort as libc::c_int;
        }
        2 => {
            result = *(*__ctype_b_loc()).offset(c as libc::c_int as isize) as libc::c_int
                & _ISblank as libc::c_int as libc::c_ushort as libc::c_int;
        }
        3 => {
            result = *(*__ctype_b_loc()).offset(c as libc::c_int as isize) as libc::c_int
                & _IScntrl as libc::c_int as libc::c_ushort as libc::c_int;
        }
        4 => {
            result = *(*__ctype_b_loc()).offset(c as libc::c_int as isize) as libc::c_int
                & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int;
        }
        5 => {
            result = *(*__ctype_b_loc()).offset(c as libc::c_int as isize) as libc::c_int
                & _ISgraph as libc::c_int as libc::c_ushort as libc::c_int;
        }
        6 => {
            result = *(*__ctype_b_loc()).offset(c as libc::c_int as isize) as libc::c_int
                & _ISlower as libc::c_int as libc::c_ushort as libc::c_int;
        }
        7 => {
            result = *(*__ctype_b_loc()).offset(c as libc::c_int as isize) as libc::c_int
                & _ISprint as libc::c_int as libc::c_ushort as libc::c_int;
        }
        8 => {
            result = *(*__ctype_b_loc()).offset(c as libc::c_int as isize) as libc::c_int
                & _ISpunct as libc::c_int as libc::c_ushort as libc::c_int;
        }
        9 => {
            result = *(*__ctype_b_loc()).offset(c as libc::c_int as isize) as libc::c_int
                & _ISspace as libc::c_int as libc::c_ushort as libc::c_int;
        }
        10 => {
            result = *(*__ctype_b_loc()).offset(c as libc::c_int as isize) as libc::c_int
                & _ISupper as libc::c_int as libc::c_ushort as libc::c_int;
        }
        11 => {
            result = *(*__ctype_b_loc()).offset(c as libc::c_int as isize) as libc::c_int
                & _ISxdigit as libc::c_int as libc::c_ushort as libc::c_int;
        }
        _ => {
            abort();
        }
    }
    return result != 0;
}
unsafe extern "C" fn es_free(mut es: *mut E_string) {
    free((*es).s as *mut libc::c_void);
    free((*es).escaped as *mut libc::c_void);
}
unsafe extern "C" fn unquote(mut s: *const libc::c_char, mut es: *mut E_string) -> bool {
    let mut len: size_t = strlen(s);
    (*es).s = xmalloc(len) as *mut libc::c_char;
    (*es)
        .escaped = xcalloc(len, ::core::mem::size_of::<bool>() as libc::c_ulong)
        as *mut bool;
    let mut j: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    let mut i: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    while *s.offset(i as isize) != 0 {
        let mut c: libc::c_uchar = 0;
        let mut oct_digit: libc::c_int = 0;
        match *s.offset(i as isize) as libc::c_int {
            92 => {
                *((*es).escaped).offset(j as isize) = 1 as libc::c_int != 0;
                match *s
                    .offset(i.wrapping_add(1 as libc::c_int as libc::c_uint) as isize)
                    as libc::c_int
                {
                    92 => {
                        c = '\\' as i32 as libc::c_uchar;
                    }
                    97 => {
                        c = '\u{7}' as i32 as libc::c_uchar;
                    }
                    98 => {
                        c = '\u{8}' as i32 as libc::c_uchar;
                    }
                    102 => {
                        c = '\u{c}' as i32 as libc::c_uchar;
                    }
                    110 => {
                        c = '\n' as i32 as libc::c_uchar;
                    }
                    114 => {
                        c = '\r' as i32 as libc::c_uchar;
                    }
                    116 => {
                        c = '\t' as i32 as libc::c_uchar;
                    }
                    118 => {
                        c = '\u{b}' as i32 as libc::c_uchar;
                    }
                    48 | 49 | 50 | 51 | 52 | 53 | 54 | 55 => {
                        c = (*s
                            .offset(
                                i.wrapping_add(1 as libc::c_int as libc::c_uint) as isize,
                            ) as libc::c_int - '0' as i32) as libc::c_uchar;
                        oct_digit = *s
                            .offset(
                                i.wrapping_add(2 as libc::c_int as libc::c_uint) as isize,
                            ) as libc::c_int - '0' as i32;
                        if 0 as libc::c_int <= oct_digit && oct_digit <= 7 as libc::c_int
                        {
                            c = (8 as libc::c_int * c as libc::c_int + oct_digit)
                                as libc::c_uchar;
                            i = i.wrapping_add(1);
                            oct_digit = *s
                                .offset(
                                    i.wrapping_add(2 as libc::c_int as libc::c_uint) as isize,
                                ) as libc::c_int - '0' as i32;
                            if 0 as libc::c_int <= oct_digit
                                && oct_digit <= 7 as libc::c_int
                            {
                                if 8 as libc::c_int * c as libc::c_int + oct_digit
                                    < N_CHARS as libc::c_int
                                {
                                    c = (8 as libc::c_int * c as libc::c_int + oct_digit)
                                        as libc::c_uchar;
                                    i = i.wrapping_add(1);
                                } else {
                                    error(
                                        0 as libc::c_int,
                                        0 as libc::c_int,
                                        dcgettext(
                                            0 as *const libc::c_char,
                                            b"warning: the ambiguous octal escape \\%c%c%c is being\n\tinterpreted as the 2-byte sequence \\0%c%c, %c\0"
                                                as *const u8 as *const libc::c_char,
                                            5 as libc::c_int,
                                        ),
                                        *s.offset(i as isize) as libc::c_int,
                                        *s
                                            .offset(
                                                i.wrapping_add(1 as libc::c_int as libc::c_uint) as isize,
                                            ) as libc::c_int,
                                        *s
                                            .offset(
                                                i.wrapping_add(2 as libc::c_int as libc::c_uint) as isize,
                                            ) as libc::c_int,
                                        *s.offset(i as isize) as libc::c_int,
                                        *s
                                            .offset(
                                                i.wrapping_add(1 as libc::c_int as libc::c_uint) as isize,
                                            ) as libc::c_int,
                                        *s
                                            .offset(
                                                i.wrapping_add(2 as libc::c_int as libc::c_uint) as isize,
                                            ) as libc::c_int,
                                    );
                                }
                            }
                        }
                    }
                    0 => {
                        error(
                            0 as libc::c_int,
                            0 as libc::c_int,
                            dcgettext(
                                0 as *const libc::c_char,
                                b"warning: an unescaped backslash at end of string is not portable\0"
                                    as *const u8 as *const libc::c_char,
                                5 as libc::c_int,
                            ),
                        );
                        *((*es).escaped).offset(j as isize) = 0 as libc::c_int != 0;
                        i = i.wrapping_sub(1);
                        c = '\\' as i32 as libc::c_uchar;
                    }
                    _ => {
                        c = *s
                            .offset(
                                i.wrapping_add(1 as libc::c_int as libc::c_uint) as isize,
                            ) as libc::c_uchar;
                    }
                }
                i = i.wrapping_add(1);
                let fresh1 = j;
                j = j.wrapping_add(1);
                *((*es).s).offset(fresh1 as isize) = c as libc::c_char;
            }
            _ => {
                let fresh2 = j;
                j = j.wrapping_add(1);
                *((*es).s).offset(fresh2 as isize) = *s.offset(i as isize);
            }
        }
        i = i.wrapping_add(1);
    }
    (*es).len = j as size_t;
    return 1 as libc::c_int != 0;
}
unsafe extern "C" fn look_up_char_class(
    mut class_str: *const libc::c_char,
    mut len: size_t,
) -> Char_class {
    let mut i: Char_class = CC_ALNUM;
    i = CC_ALNUM;
    while (i as libc::c_ulong)
        < (::core::mem::size_of::<[*const libc::c_char; 12]>() as libc::c_ulong)
            .wrapping_div(::core::mem::size_of::<*const libc::c_char>() as libc::c_ulong)
    {
        if strncmp(class_str, char_class_name[i as usize], len) == 0 as libc::c_int
            && strlen(char_class_name[i as usize]) == len
        {
            return i;
        }
        i += 1;
    }
    return CC_NO_CLASS;
}
unsafe extern "C" fn make_printable_char(mut c: libc::c_uchar) -> *mut libc::c_char {
    let mut buf: *mut libc::c_char = xmalloc(5 as libc::c_int as size_t)
        as *mut libc::c_char;
    if *(*__ctype_b_loc()).offset(c as libc::c_int as isize) as libc::c_int
        & _ISprint as libc::c_int as libc::c_ushort as libc::c_int != 0
    {
        *buf.offset(0 as libc::c_int as isize) = c as libc::c_char;
        *buf.offset(1 as libc::c_int as isize) = '\0' as i32 as libc::c_char;
    } else {
        sprintf(buf, b"\\%03o\0" as *const u8 as *const libc::c_char, c as libc::c_int);
    }
    return buf;
}
unsafe extern "C" fn make_printable_str(
    mut s: *const libc::c_char,
    mut len: size_t,
) -> *mut libc::c_char {
    let mut printable_buf: *mut libc::c_char = xnmalloc(
        len.wrapping_add(1 as libc::c_int as libc::c_ulong),
        4 as libc::c_int as size_t,
    ) as *mut libc::c_char;
    let mut p: *mut libc::c_char = printable_buf;
    let mut i: size_t = 0 as libc::c_int as size_t;
    while i < len {
        let mut buf: [libc::c_char; 5] = [0; 5];
        let mut tmp: *const libc::c_char = 0 as *const libc::c_char;
        let mut c: libc::c_uchar = *s.offset(i as isize) as libc::c_uchar;
        match c as libc::c_int {
            92 => {
                tmp = b"\\\0" as *const u8 as *const libc::c_char;
            }
            7 => {
                tmp = b"\\a\0" as *const u8 as *const libc::c_char;
            }
            8 => {
                tmp = b"\\b\0" as *const u8 as *const libc::c_char;
            }
            12 => {
                tmp = b"\\f\0" as *const u8 as *const libc::c_char;
            }
            10 => {
                tmp = b"\\n\0" as *const u8 as *const libc::c_char;
            }
            13 => {
                tmp = b"\\r\0" as *const u8 as *const libc::c_char;
            }
            9 => {
                tmp = b"\\t\0" as *const u8 as *const libc::c_char;
            }
            11 => {
                tmp = b"\\v\0" as *const u8 as *const libc::c_char;
            }
            _ => {
                if *(*__ctype_b_loc()).offset(c as libc::c_int as isize) as libc::c_int
                    & _ISprint as libc::c_int as libc::c_ushort as libc::c_int != 0
                {
                    buf[0 as libc::c_int as usize] = c as libc::c_char;
                    buf[1 as libc::c_int as usize] = '\0' as i32 as libc::c_char;
                } else {
                    sprintf(
                        buf.as_mut_ptr(),
                        b"\\%03o\0" as *const u8 as *const libc::c_char,
                        c as libc::c_int,
                    );
                }
                tmp = buf.as_mut_ptr();
            }
        }
        p = stpcpy(p, tmp);
        i = i.wrapping_add(1);
    }
    return printable_buf;
}
unsafe extern "C" fn append_normal_char(mut list: *mut Spec_list, mut c: libc::c_uchar) {
    let mut new: *mut List_element = xmalloc(
        ::core::mem::size_of::<List_element>() as libc::c_ulong,
    ) as *mut List_element;
    (*new).next = 0 as *mut List_element;
    (*new).type_0 = RE_NORMAL_CHAR;
    (*new).u.normal_char = c;
    if !((*list).tail).is_null() {} else {
        __assert_fail(
            b"list->tail\0" as *const u8 as *const libc::c_char,
            b"src/tr.c\0" as *const u8 as *const libc::c_char,
            649 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 59],
                &[libc::c_char; 59],
            >(b"void append_normal_char(struct Spec_list *, unsigned char)\0"))
                .as_ptr(),
        );
    }
    (*(*list).tail).next = new;
    (*list).tail = new;
}
unsafe extern "C" fn append_range(
    mut list: *mut Spec_list,
    mut first: libc::c_uchar,
    mut last: libc::c_uchar,
) -> bool {
    if (last as libc::c_int) < first as libc::c_int {
        let mut tmp1: *mut libc::c_char = make_printable_char(first);
        let mut tmp2: *mut libc::c_char = make_printable_char(last);
        error(
            0 as libc::c_int,
            0 as libc::c_int,
            dcgettext(
                0 as *const libc::c_char,
                b"range-endpoints of '%s-%s' are in reverse collating sequence order\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            tmp1,
            tmp2,
        );
        free(tmp1 as *mut libc::c_void);
        free(tmp2 as *mut libc::c_void);
        return 0 as libc::c_int != 0;
    }
    let mut new: *mut List_element = xmalloc(
        ::core::mem::size_of::<List_element>() as libc::c_ulong,
    ) as *mut List_element;
    (*new).next = 0 as *mut List_element;
    (*new).type_0 = RE_RANGE;
    (*new).u.range.first_char = first;
    (*new).u.range.last_char = last;
    if !((*list).tail).is_null() {} else {
        __assert_fail(
            b"list->tail\0" as *const u8 as *const libc::c_char,
            b"src/tr.c\0" as *const u8 as *const libc::c_char,
            679 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 69],
                &[libc::c_char; 69],
            >(b"_Bool append_range(struct Spec_list *, unsigned char, unsigned char)\0"))
                .as_ptr(),
        );
    }
    (*(*list).tail).next = new;
    (*list).tail = new;
    return 1 as libc::c_int != 0;
}
unsafe extern "C" fn append_char_class(
    mut list: *mut Spec_list,
    mut char_class_str: *const libc::c_char,
    mut len: size_t,
) -> bool {
    let mut char_class: Char_class = look_up_char_class(char_class_str, len);
    if char_class as libc::c_uint == CC_NO_CLASS as libc::c_int as libc::c_uint {
        return 0 as libc::c_int != 0;
    }
    let mut new: *mut List_element = xmalloc(
        ::core::mem::size_of::<List_element>() as libc::c_ulong,
    ) as *mut List_element;
    (*new).next = 0 as *mut List_element;
    (*new).type_0 = RE_CHAR_CLASS;
    (*new).u.char_class = char_class;
    if !((*list).tail).is_null() {} else {
        __assert_fail(
            b"list->tail\0" as *const u8 as *const libc::c_char,
            b"src/tr.c\0" as *const u8 as *const libc::c_char,
            701 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 66],
                &[libc::c_char; 66],
            >(b"_Bool append_char_class(struct Spec_list *, const char *, size_t)\0"))
                .as_ptr(),
        );
    }
    (*(*list).tail).next = new;
    (*list).tail = new;
    return 1 as libc::c_int != 0;
}
unsafe extern "C" fn append_repeated_char(
    mut list: *mut Spec_list,
    mut the_char: libc::c_uchar,
    mut repeat_count: count,
) {
    let mut new: *mut List_element = xmalloc(
        ::core::mem::size_of::<List_element>() as libc::c_ulong,
    ) as *mut List_element;
    (*new).next = 0 as *mut List_element;
    (*new).type_0 = RE_REPEATED_CHAR;
    (*new).u.repeated_char.the_repeated_char = the_char;
    (*new).u.repeated_char.repeat_count = repeat_count;
    if !((*list).tail).is_null() {} else {
        __assert_fail(
            b"list->tail\0" as *const u8 as *const libc::c_char,
            b"src/tr.c\0" as *const u8 as *const libc::c_char,
            721 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 68],
                &[libc::c_char; 68],
            >(b"void append_repeated_char(struct Spec_list *, unsigned char, count)\0"))
                .as_ptr(),
        );
    }
    (*(*list).tail).next = new;
    (*list).tail = new;
}
unsafe extern "C" fn append_equiv_class(
    mut list: *mut Spec_list,
    mut equiv_class_str: *const libc::c_char,
    mut len: size_t,
) -> bool {
    if len != 1 as libc::c_int as libc::c_ulong {
        return 0 as libc::c_int != 0;
    }
    let mut new: *mut List_element = xmalloc(
        ::core::mem::size_of::<List_element>() as libc::c_ulong,
    ) as *mut List_element;
    (*new).next = 0 as *mut List_element;
    (*new).type_0 = RE_EQUIV_CLASS;
    (*new).u.equiv_code = *equiv_class_str as libc::c_uchar;
    if !((*list).tail).is_null() {} else {
        __assert_fail(
            b"list->tail\0" as *const u8 as *const libc::c_char,
            b"src/tr.c\0" as *const u8 as *const libc::c_char,
            743 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 67],
                &[libc::c_char; 67],
            >(b"_Bool append_equiv_class(struct Spec_list *, const char *, size_t)\0"))
                .as_ptr(),
        );
    }
    (*(*list).tail).next = new;
    (*list).tail = new;
    return 1 as libc::c_int != 0;
}
unsafe extern "C" fn find_closing_delim(
    mut es: *const E_string,
    mut start_idx: size_t,
    mut pre_bracket_char: libc::c_char,
    mut result_idx: *mut size_t,
) -> bool {
    let mut i: size_t = start_idx;
    while i < ((*es).len).wrapping_sub(1 as libc::c_int as libc::c_ulong) {
        if *((*es).s).offset(i as isize) as libc::c_int
            == pre_bracket_char as libc::c_int
            && *((*es).s)
                .offset(i.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize)
                as libc::c_int == ']' as i32 && !*((*es).escaped).offset(i as isize)
            && !*((*es).escaped)
                .offset(i.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize)
        {
            *result_idx = i;
            return 1 as libc::c_int != 0;
        }
        i = i.wrapping_add(1);
    }
    return 0 as libc::c_int != 0;
}
unsafe extern "C" fn find_bracketed_repeat(
    mut es: *const E_string,
    mut start_idx: size_t,
    mut char_to_repeat: *mut libc::c_uchar,
    mut repeat_count: *mut count,
    mut closing_bracket_idx: *mut size_t,
) -> libc::c_int {
    if start_idx.wrapping_add(1 as libc::c_int as libc::c_ulong) < (*es).len {} else {
        __assert_fail(
            b"start_idx + 1 < es->len\0" as *const u8 as *const libc::c_char,
            b"src/tr.c\0" as *const u8 as *const libc::c_char,
            784 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 95],
                &[libc::c_char; 95],
            >(
                b"int find_bracketed_repeat(const struct E_string *, size_t, unsigned char *, count *, size_t *)\0",
            ))
                .as_ptr(),
        );
    }
    if !es_match(
        es,
        start_idx.wrapping_add(1 as libc::c_int as libc::c_ulong),
        '*' as i32 as libc::c_char,
    ) {
        return -(1 as libc::c_int);
    }
    let mut i: size_t = start_idx.wrapping_add(2 as libc::c_int as libc::c_ulong);
    while i < (*es).len && !*((*es).escaped).offset(i as isize) {
        if *((*es).s).offset(i as isize) as libc::c_int == ']' as i32 {
            let mut digit_str_len: size_t = i
                .wrapping_sub(start_idx)
                .wrapping_sub(2 as libc::c_int as libc::c_ulong);
            *char_to_repeat = *((*es).s).offset(start_idx as isize) as libc::c_uchar;
            if digit_str_len == 0 as libc::c_int as libc::c_ulong {
                *repeat_count = 0 as libc::c_int as count;
            } else {
                let mut digit_str: *const libc::c_char = &mut *((*es).s)
                    .offset(
                        start_idx.wrapping_add(2 as libc::c_int as libc::c_ulong)
                            as isize,
                    ) as *mut libc::c_char;
                let mut d_end: *mut libc::c_char = 0 as *mut libc::c_char;
                if xstrtoumax(
                    digit_str,
                    &mut d_end,
                    (if *digit_str as libc::c_int == '0' as i32 {
                        8 as libc::c_int
                    } else {
                        10 as libc::c_int
                    }),
                    repeat_count,
                    0 as *const libc::c_char,
                ) as libc::c_uint != LONGINT_OK as libc::c_int as libc::c_uint
                    || (18446744073709551615 as libc::c_ulong)
                        .wrapping_sub(1 as libc::c_int as libc::c_ulong) < *repeat_count
                    || digit_str.offset(digit_str_len as isize)
                        != d_end as *const libc::c_char
                {
                    let mut tmp: *mut libc::c_char = make_printable_str(
                        digit_str,
                        digit_str_len,
                    );
                    error(
                        0 as libc::c_int,
                        0 as libc::c_int,
                        dcgettext(
                            0 as *const libc::c_char,
                            b"invalid repeat count %s in [c*n] construct\0" as *const u8
                                as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                        quote(tmp),
                    );
                    free(tmp as *mut libc::c_void);
                    return -(2 as libc::c_int);
                }
            }
            *closing_bracket_idx = i;
            return 0 as libc::c_int;
        }
        i = i.wrapping_add(1);
    }
    return -(1 as libc::c_int);
}
unsafe extern "C" fn star_digits_closebracket(
    mut es: *const E_string,
    mut idx: size_t,
) -> bool {
    if !es_match(es, idx, '*' as i32 as libc::c_char) {
        return 0 as libc::c_int != 0;
    }
    let mut i: size_t = idx.wrapping_add(1 as libc::c_int as libc::c_ulong);
    while i < (*es).len {
        if !((to_uchar(*((*es).s).offset(i as isize)) as libc::c_uint)
            .wrapping_sub('0' as i32 as libc::c_uint)
            <= 9 as libc::c_int as libc::c_uint)
            || *((*es).escaped).offset(i as isize) as libc::c_int != 0
        {
            return es_match(es, i, ']' as i32 as libc::c_char);
        }
        i = i.wrapping_add(1);
    }
    return 0 as libc::c_int != 0;
}
unsafe extern "C" fn build_spec_list(
    mut es: *const E_string,
    mut result: *mut Spec_list,
) -> bool {
    let mut current_block: u64;
    let mut p: *const libc::c_char = (*es).s;
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i.wrapping_add(2 as libc::c_int as libc::c_ulong) < (*es).len {
        if es_match(es, i, '[' as i32 as libc::c_char) {
            let mut matched_multi_char_construct: bool = false;
            let mut closing_bracket_idx: size_t = 0;
            let mut char_to_repeat: libc::c_uchar = 0;
            let mut repeat_count: count = 0;
            let mut err: libc::c_int = 0;
            matched_multi_char_construct = 1 as libc::c_int != 0;
            if es_match(
                es,
                i.wrapping_add(1 as libc::c_int as libc::c_ulong),
                ':' as i32 as libc::c_char,
            ) as libc::c_int != 0
                || es_match(
                    es,
                    i.wrapping_add(1 as libc::c_int as libc::c_ulong),
                    '=' as i32 as libc::c_char,
                ) as libc::c_int != 0
            {
                let mut closing_delim_idx: size_t = 0;
                if find_closing_delim(
                    es,
                    i.wrapping_add(2 as libc::c_int as libc::c_ulong),
                    *p
                        .offset(
                            i.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                        ),
                    &mut closing_delim_idx,
                ) {
                    let mut opnd_str_len: size_t = closing_delim_idx
                        .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                        .wrapping_sub(i.wrapping_add(2 as libc::c_int as libc::c_ulong))
                        .wrapping_add(1 as libc::c_int as libc::c_ulong);
                    let mut opnd_str: *const libc::c_char = p
                        .offset(i as isize)
                        .offset(2 as libc::c_int as isize);
                    if opnd_str_len == 0 as libc::c_int as libc::c_ulong {
                        if *p
                            .offset(
                                i.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                            ) as libc::c_int == ':' as i32
                        {
                            error(
                                0 as libc::c_int,
                                0 as libc::c_int,
                                dcgettext(
                                    0 as *const libc::c_char,
                                    b"missing character class name '[::]'\0" as *const u8
                                        as *const libc::c_char,
                                    5 as libc::c_int,
                                ),
                            );
                        } else {
                            error(
                                0 as libc::c_int,
                                0 as libc::c_int,
                                dcgettext(
                                    0 as *const libc::c_char,
                                    b"missing equivalence class character '[==]'\0" as *const u8
                                        as *const libc::c_char,
                                    5 as libc::c_int,
                                ),
                            );
                        }
                        return 0 as libc::c_int != 0;
                    }
                    if *p
                        .offset(
                            i.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                        ) as libc::c_int == ':' as i32
                    {
                        if !append_char_class(result, opnd_str, opnd_str_len) {
                            if star_digits_closebracket(
                                es,
                                i.wrapping_add(2 as libc::c_int as libc::c_ulong),
                            ) {
                                current_block = 2237306555495136881;
                            } else {
                                let mut tmp: *mut libc::c_char = make_printable_str(
                                    opnd_str,
                                    opnd_str_len,
                                );
                                error(
                                    0 as libc::c_int,
                                    0 as libc::c_int,
                                    dcgettext(
                                        0 as *const libc::c_char,
                                        b"invalid character class %s\0" as *const u8
                                            as *const libc::c_char,
                                        5 as libc::c_int,
                                    ),
                                    quote(tmp),
                                );
                                free(tmp as *mut libc::c_void);
                                return 0 as libc::c_int != 0;
                            }
                        } else {
                            current_block = 15768484401365413375;
                        }
                    } else if !append_equiv_class(result, opnd_str, opnd_str_len) {
                        if star_digits_closebracket(
                            es,
                            i.wrapping_add(2 as libc::c_int as libc::c_ulong),
                        ) {
                            current_block = 2237306555495136881;
                        } else {
                            let mut tmp_0: *mut libc::c_char = make_printable_str(
                                opnd_str,
                                opnd_str_len,
                            );
                            error(
                                0 as libc::c_int,
                                0 as libc::c_int,
                                dcgettext(
                                    0 as *const libc::c_char,
                                    b"%s: equivalence class operand must be a single character\0"
                                        as *const u8 as *const libc::c_char,
                                    5 as libc::c_int,
                                ),
                                tmp_0,
                            );
                            free(tmp_0 as *mut libc::c_void);
                            return 0 as libc::c_int != 0;
                        }
                    } else {
                        current_block = 15768484401365413375;
                    }
                    match current_block {
                        2237306555495136881 => {}
                        _ => {
                            i = closing_delim_idx
                                .wrapping_add(2 as libc::c_int as libc::c_ulong);
                            continue;
                        }
                    }
                }
            }
            err = find_bracketed_repeat(
                es,
                i.wrapping_add(1 as libc::c_int as libc::c_ulong),
                &mut char_to_repeat,
                &mut repeat_count,
                &mut closing_bracket_idx,
            );
            if err == 0 as libc::c_int {
                append_repeated_char(result, char_to_repeat, repeat_count);
                i = closing_bracket_idx.wrapping_add(1 as libc::c_int as libc::c_ulong);
            } else if err == -(1 as libc::c_int) {
                matched_multi_char_construct = 0 as libc::c_int != 0;
            } else {
                return 0 as libc::c_int != 0
            }
            if matched_multi_char_construct {
                continue;
            }
        }
        if es_match(
            es,
            i.wrapping_add(1 as libc::c_int as libc::c_ulong),
            '-' as i32 as libc::c_char,
        ) {
            if !append_range(
                result,
                *p.offset(i as isize) as libc::c_uchar,
                *p.offset(i.wrapping_add(2 as libc::c_int as libc::c_ulong) as isize)
                    as libc::c_uchar,
            ) {
                return 0 as libc::c_int != 0;
            }
            i = (i as libc::c_ulong).wrapping_add(3 as libc::c_int as libc::c_ulong)
                as size_t as size_t;
        } else {
            append_normal_char(result, *p.offset(i as isize) as libc::c_uchar);
            i = i.wrapping_add(1);
        }
    }
    while i < (*es).len {
        append_normal_char(result, *p.offset(i as isize) as libc::c_uchar);
        i = i.wrapping_add(1);
    }
    return 1 as libc::c_int != 0;
}
unsafe extern "C" fn skip_construct(mut s: *mut Spec_list) {
    (*s).tail = (*(*s).tail).next;
    (*s)
        .state = (18446744073709551615 as libc::c_ulong)
        .wrapping_sub(1 as libc::c_int as libc::c_ulong)
        .wrapping_add(1 as libc::c_int as libc::c_ulong);
}
unsafe extern "C" fn get_next(
    mut s: *mut Spec_list,
    mut class: *mut Upper_Lower_class,
) -> libc::c_int {
    let mut p: *mut List_element = 0 as *mut List_element;
    let mut return_val: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    if !class.is_null() {
        *class = UL_NONE;
    }
    if (*s).state
        == (18446744073709551615 as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong)
    {
        (*s).tail = (*(*s).head).next;
        (*s)
            .state = (18446744073709551615 as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong)
            .wrapping_add(1 as libc::c_int as libc::c_ulong);
    }
    p = (*s).tail;
    if p.is_null() {
        return -(1 as libc::c_int);
    }
    match (*p).type_0 as libc::c_uint {
        0 => {
            return_val = (*p).u.normal_char as libc::c_int;
            (*s)
                .state = (18446744073709551615 as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                .wrapping_add(1 as libc::c_int as libc::c_ulong);
            (*s).tail = (*p).next;
        }
        1 => {
            if (*s).state
                == (18446744073709551615 as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                    .wrapping_add(1 as libc::c_int as libc::c_ulong)
            {
                (*s).state = (*p).u.range.first_char as count;
            } else {
                (*s).state = ((*s).state).wrapping_add(1);
            }
            return_val = (*s).state as libc::c_int;
            if (*s).state == (*p).u.range.last_char as libc::c_ulong {
                (*s).tail = (*p).next;
                (*s)
                    .state = (18446744073709551615 as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                    .wrapping_add(1 as libc::c_int as libc::c_ulong);
            }
        }
        2 => {
            if !class.is_null() {
                match (*p).u.char_class as libc::c_uint {
                    6 => {
                        *class = UL_LOWER;
                    }
                    10 => {
                        *class = UL_UPPER;
                    }
                    _ => {}
                }
            }
            if (*s).state
                == (18446744073709551615 as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                    .wrapping_add(1 as libc::c_int as libc::c_ulong)
            {
                i = 0 as libc::c_int;
                while i < N_CHARS as libc::c_int {
                    if is_char_class_member((*p).u.char_class, i as libc::c_uchar) {
                        break;
                    }
                    i += 1;
                }
                if i < N_CHARS as libc::c_int {} else {
                    __assert_fail(
                        b"i < N_CHARS\0" as *const u8 as *const libc::c_char,
                        b"src/tr.c\0" as *const u8 as *const libc::c_char,
                        1079 as libc::c_int as libc::c_uint,
                        (*::core::mem::transmute::<
                            &[u8; 59],
                            &[libc::c_char; 59],
                        >(
                            b"int get_next(struct Spec_list *, enum Upper_Lower_class *)\0",
                        ))
                            .as_ptr(),
                    );
                }
                (*s).state = i as count;
            }
            if is_char_class_member(
                (*p).u.char_class,
                (*s).state as libc::c_uchar,
            ) {} else {
                __assert_fail(
                    b"is_char_class_member (p->u.char_class, s->state)\0" as *const u8
                        as *const libc::c_char,
                    b"src/tr.c\0" as *const u8 as *const libc::c_char,
                    1082 as libc::c_int as libc::c_uint,
                    (*::core::mem::transmute::<
                        &[u8; 59],
                        &[libc::c_char; 59],
                    >(b"int get_next(struct Spec_list *, enum Upper_Lower_class *)\0"))
                        .as_ptr(),
                );
            }
            return_val = (*s).state as libc::c_int;
            i = ((*s).state).wrapping_add(1 as libc::c_int as libc::c_ulong)
                as libc::c_int;
            while i < N_CHARS as libc::c_int {
                if is_char_class_member((*p).u.char_class, i as libc::c_uchar) {
                    break;
                }
                i += 1;
            }
            if i < N_CHARS as libc::c_int {
                (*s).state = i as count;
            } else {
                (*s).tail = (*p).next;
                (*s)
                    .state = (18446744073709551615 as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                    .wrapping_add(1 as libc::c_int as libc::c_ulong);
            }
        }
        3 => {
            return_val = (*p).u.equiv_code as libc::c_int;
            (*s)
                .state = (18446744073709551615 as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                .wrapping_add(1 as libc::c_int as libc::c_ulong);
            (*s).tail = (*p).next;
        }
        4 => {
            if (*p).u.repeated_char.repeat_count == 0 as libc::c_int as libc::c_ulong {
                (*s).tail = (*p).next;
                (*s)
                    .state = (18446744073709551615 as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                    .wrapping_add(1 as libc::c_int as libc::c_ulong);
                return_val = get_next(s, class);
            } else {
                if (*s).state
                    == (18446744073709551615 as libc::c_ulong)
                        .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                        .wrapping_add(1 as libc::c_int as libc::c_ulong)
                {
                    (*s).state = 0 as libc::c_int as count;
                }
                (*s).state = ((*s).state).wrapping_add(1);
                return_val = (*p).u.repeated_char.the_repeated_char as libc::c_int;
                if (*s).state == (*p).u.repeated_char.repeat_count {
                    (*s).tail = (*p).next;
                    (*s)
                        .state = (18446744073709551615 as libc::c_ulong)
                        .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                        .wrapping_add(1 as libc::c_int as libc::c_ulong);
                }
            }
        }
        _ => {
            abort();
        }
    }
    return return_val;
}
unsafe extern "C" fn card_of_complement(mut s: *mut Spec_list) -> libc::c_int {
    let mut c: libc::c_int = 0;
    let mut cardinality: libc::c_int = N_CHARS as libc::c_int;
    let mut in_set: [bool; 256] = [
        0 as libc::c_int != 0,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
    ];
    (*s)
        .state = (18446744073709551615 as libc::c_ulong)
        .wrapping_sub(1 as libc::c_int as libc::c_ulong);
    loop {
        c = get_next(s, 0 as *mut Upper_Lower_class);
        if !(c != -(1 as libc::c_int)) {
            break;
        }
        cardinality -= !in_set[c as usize] as libc::c_int;
        in_set[c as usize] = 1 as libc::c_int != 0;
    }
    return cardinality;
}
unsafe extern "C" fn validate_case_classes(
    mut s1: *mut Spec_list,
    mut s2: *mut Spec_list,
) {
    let mut n_upper: size_t = 0 as libc::c_int as size_t;
    let mut n_lower: size_t = 0 as libc::c_int as size_t;
    let mut c1: libc::c_int = 0 as libc::c_int;
    let mut c2: libc::c_int = 0 as libc::c_int;
    let mut old_s1_len: count = (*s1).length;
    let mut old_s2_len: count = (*s2).length;
    let mut s1_tail: *mut List_element = (*s1).tail;
    let mut s2_tail: *mut List_element = (*s2).tail;
    let mut s1_new_element: bool = 1 as libc::c_int != 0;
    let mut s2_new_element: bool = 1 as libc::c_int != 0;
    if complement as libc::c_int != 0 || !(*s2).has_char_class {
        return;
    }
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < N_CHARS as libc::c_int {
        if *(*__ctype_b_loc()).offset(i as isize) as libc::c_int
            & _ISupper as libc::c_int as libc::c_ushort as libc::c_int != 0
        {
            n_upper = n_upper.wrapping_add(1);
        }
        if *(*__ctype_b_loc()).offset(i as isize) as libc::c_int
            & _ISlower as libc::c_int as libc::c_ushort as libc::c_int != 0
        {
            n_lower = n_lower.wrapping_add(1);
        }
        i += 1;
    }
    (*s1)
        .state = (18446744073709551615 as libc::c_ulong)
        .wrapping_sub(1 as libc::c_int as libc::c_ulong);
    (*s2)
        .state = (18446744073709551615 as libc::c_ulong)
        .wrapping_sub(1 as libc::c_int as libc::c_ulong);
    while c1 != -(1 as libc::c_int) && c2 != -(1 as libc::c_int) {
        let mut class_s1: Upper_Lower_class = UL_LOWER;
        let mut class_s2: Upper_Lower_class = UL_LOWER;
        c1 = get_next(s1, &mut class_s1);
        c2 = get_next(s2, &mut class_s2);
        if s2_new_element as libc::c_int != 0
            && class_s2 as libc::c_uint != UL_NONE as libc::c_int as libc::c_uint
            && !(s1_new_element as libc::c_int != 0
                && class_s1 as libc::c_uint != UL_NONE as libc::c_int as libc::c_uint)
        {
            if ::core::mem::size_of::<C2RustUnnamed_5>() as libc::c_ulong != 0 {
                error(
                    1 as libc::c_int,
                    0 as libc::c_int,
                    dcgettext(
                        0 as *const libc::c_char,
                        b"misaligned [:upper:] and/or [:lower:] construct\0" as *const u8
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
                        b"misaligned [:upper:] and/or [:lower:] construct\0" as *const u8
                            as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                );
                if 0 as libc::c_int != 0 {} else {
                    unreachable!();
                };
            };
        }
        if class_s2 as libc::c_uint != UL_NONE as libc::c_int as libc::c_uint {
            skip_construct(s1);
            skip_construct(s2);
            (*s1)
                .length = ((*s1).length as libc::c_ulong)
                .wrapping_sub(
                    (if class_s1 as libc::c_uint
                        == UL_UPPER as libc::c_int as libc::c_uint
                    {
                        n_upper
                    } else {
                        n_lower
                    })
                        .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                ) as count as count;
            (*s2)
                .length = ((*s2).length as libc::c_ulong)
                .wrapping_sub(
                    (if class_s2 as libc::c_uint
                        == UL_UPPER as libc::c_int as libc::c_uint
                    {
                        n_upper
                    } else {
                        n_lower
                    })
                        .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                ) as count as count;
        }
        s1_new_element = (*s1).state
            == (18446744073709551615 as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                .wrapping_add(1 as libc::c_int as libc::c_ulong);
        s2_new_element = (*s2).state
            == (18446744073709551615 as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                .wrapping_add(1 as libc::c_int as libc::c_ulong);
    }
    if old_s1_len >= (*s1).length && old_s2_len >= (*s2).length {} else {
        __assert_fail(
            b"old_s1_len >= s1->length && old_s2_len >= s2->length\0" as *const u8
                as *const libc::c_char,
            b"src/tr.c\0" as *const u8 as *const libc::c_char,
            1224 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 67],
                &[libc::c_char; 67],
            >(b"void validate_case_classes(struct Spec_list *, struct Spec_list *)\0"))
                .as_ptr(),
        );
    }
    (*s1).tail = s1_tail;
    (*s2).tail = s2_tail;
}
unsafe extern "C" fn get_spec_stats(mut s: *mut Spec_list) {
    let mut p: *mut List_element = 0 as *mut List_element;
    let mut length: count = 0 as libc::c_int as count;
    (*s).n_indefinite_repeats = 0 as libc::c_int as size_t;
    (*s).has_equiv_class = 0 as libc::c_int != 0;
    (*s).has_restricted_char_class = 0 as libc::c_int != 0;
    (*s).has_char_class = 0 as libc::c_int != 0;
    p = (*(*s).head).next;
    while !p.is_null() {
        let mut len: count = 0 as libc::c_int as count;
        let mut new_length: count = 0;
        match (*p).type_0 as libc::c_uint {
            0 => {
                len = 1 as libc::c_int as count;
            }
            1 => {
                if (*p).u.range.last_char as libc::c_int
                    >= (*p).u.range.first_char as libc::c_int
                {} else {
                    __assert_fail(
                        b"p->u.range.last_char >= p->u.range.first_char\0" as *const u8
                            as *const libc::c_char,
                        b"src/tr.c\0" as *const u8 as *const libc::c_char,
                        1265 as libc::c_int as libc::c_uint,
                        (*::core::mem::transmute::<
                            &[u8; 40],
                            &[libc::c_char; 40],
                        >(b"void get_spec_stats(struct Spec_list *)\0"))
                            .as_ptr(),
                    );
                }
                len = ((*p).u.range.last_char as libc::c_int
                    - (*p).u.range.first_char as libc::c_int + 1 as libc::c_int)
                    as count;
            }
            2 => {
                (*s).has_char_class = 1 as libc::c_int != 0;
                let mut i: libc::c_int = 0 as libc::c_int;
                while i < N_CHARS as libc::c_int {
                    if is_char_class_member((*p).u.char_class, i as libc::c_uchar) {
                        len = len.wrapping_add(1);
                    }
                    i += 1;
                }
                match (*p).u.char_class as libc::c_uint {
                    10 | 6 => {}
                    _ => {
                        (*s).has_restricted_char_class = 1 as libc::c_int != 0;
                    }
                }
            }
            3 => {
                let mut i_0: libc::c_int = 0 as libc::c_int;
                while i_0 < N_CHARS as libc::c_int {
                    if is_equiv_class_member((*p).u.equiv_code, i_0 as libc::c_uchar) {
                        len = len.wrapping_add(1);
                    }
                    i_0 += 1;
                }
                (*s).has_equiv_class = 1 as libc::c_int != 0;
            }
            4 => {
                if (*p).u.repeated_char.repeat_count > 0 as libc::c_int as libc::c_ulong
                {
                    len = (*p).u.repeated_char.repeat_count;
                } else {
                    (*s).indefinite_repeat_element = p;
                    (*s)
                        .n_indefinite_repeats = ((*s).n_indefinite_repeats)
                        .wrapping_add(1);
                }
            }
            _ => {
                abort();
            }
        }
        new_length = length.wrapping_add(len);
        if !(length <= new_length
            && new_length
                <= (18446744073709551615 as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong))
        {
            if ::core::mem::size_of::<C2RustUnnamed_6>() as libc::c_ulong != 0 {
                error(
                    1 as libc::c_int,
                    0 as libc::c_int,
                    dcgettext(
                        0 as *const libc::c_char,
                        b"too many characters in set\0" as *const u8
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
                        b"too many characters in set\0" as *const u8
                            as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                );
                if 0 as libc::c_int != 0 {} else {
                    unreachable!();
                };
            };
        }
        length = new_length;
        p = (*p).next;
    }
    (*s).length = length;
}
unsafe extern "C" fn get_s1_spec_stats(mut s1: *mut Spec_list) {
    get_spec_stats(s1);
    if complement {
        (*s1).length = card_of_complement(s1) as count;
    }
}
unsafe extern "C" fn get_s2_spec_stats(mut s2: *mut Spec_list, mut len_s1: count) {
    get_spec_stats(s2);
    if len_s1 >= (*s2).length
        && (*s2).n_indefinite_repeats == 1 as libc::c_int as libc::c_ulong
    {
        (*(*s2).indefinite_repeat_element)
            .u
            .repeated_char
            .repeat_count = len_s1.wrapping_sub((*s2).length);
        (*s2).length = len_s1;
    }
}
unsafe extern "C" fn spec_init(mut spec_list: *mut Spec_list) {
    let mut new: *mut List_element = xmalloc(
        ::core::mem::size_of::<List_element>() as libc::c_ulong,
    ) as *mut List_element;
    (*spec_list).tail = new;
    (*spec_list).head = (*spec_list).tail;
    (*(*spec_list).head).next = 0 as *mut List_element;
}
unsafe extern "C" fn parse_str(
    mut s: *const libc::c_char,
    mut spec_list: *mut Spec_list,
) -> bool {
    let mut es: E_string = E_string {
        s: 0 as *mut libc::c_char,
        escaped: 0 as *mut bool,
        len: 0,
    };
    let mut ok: bool = unquote(s, &mut es) as libc::c_int != 0
        && build_spec_list(&mut es, spec_list) as libc::c_int != 0;
    es_free(&mut es);
    return ok;
}
unsafe extern "C" fn string2_extend(mut s1: *const Spec_list, mut s2: *mut Spec_list) {
    let mut p: *mut List_element = 0 as *mut List_element;
    let mut char_to_repeat: libc::c_uchar = 0;
    if translating {} else {
        __assert_fail(
            b"translating\0" as *const u8 as *const libc::c_char,
            b"src/tr.c\0" as *const u8 as *const libc::c_char,
            1377 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 66],
                &[libc::c_char; 66],
            >(b"void string2_extend(const struct Spec_list *, struct Spec_list *)\0"))
                .as_ptr(),
        );
    }
    if (*s1).length > (*s2).length {} else {
        __assert_fail(
            b"s1->length > s2->length\0" as *const u8 as *const libc::c_char,
            b"src/tr.c\0" as *const u8 as *const libc::c_char,
            1378 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 66],
                &[libc::c_char; 66],
            >(b"void string2_extend(const struct Spec_list *, struct Spec_list *)\0"))
                .as_ptr(),
        );
    }
    if (*s2).length > 0 as libc::c_int as libc::c_ulong {} else {
        __assert_fail(
            b"s2->length > 0\0" as *const u8 as *const libc::c_char,
            b"src/tr.c\0" as *const u8 as *const libc::c_char,
            1379 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 66],
                &[libc::c_char; 66],
            >(b"void string2_extend(const struct Spec_list *, struct Spec_list *)\0"))
                .as_ptr(),
        );
    }
    p = (*s2).tail;
    let mut current_block_9: u64;
    match (*p).type_0 as libc::c_uint {
        0 => {
            char_to_repeat = (*p).u.normal_char;
            current_block_9 = 2868539653012386629;
        }
        1 => {
            char_to_repeat = (*p).u.range.last_char;
            current_block_9 = 2868539653012386629;
        }
        2 => {
            if ::core::mem::size_of::<C2RustUnnamed_7>() as libc::c_ulong != 0 {
                error(
                    1 as libc::c_int,
                    0 as libc::c_int,
                    dcgettext(
                        0 as *const libc::c_char,
                        b"when translating with string1 longer than string2,\nthe latter string must not end with a character class\0"
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
                        b"when translating with string1 longer than string2,\nthe latter string must not end with a character class\0"
                            as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                );
                if 0 as libc::c_int != 0 {} else {
                    unreachable!();
                };
            };
            current_block_9 = 10975911783743850565;
        }
        4 => {
            current_block_9 = 10975911783743850565;
        }
        3 => {
            abort();
        }
        _ => {
            abort();
        }
    }
    match current_block_9 {
        10975911783743850565 => {
            char_to_repeat = (*p).u.repeated_char.the_repeated_char;
        }
        _ => {}
    }
    append_repeated_char(s2, char_to_repeat, ((*s1).length).wrapping_sub((*s2).length));
    (*s2).length = (*s1).length;
}
unsafe extern "C" fn homogeneous_spec_list(mut s: *mut Spec_list) -> bool {
    let mut b: libc::c_int = 0;
    let mut c: libc::c_int = 0;
    (*s)
        .state = (18446744073709551615 as libc::c_ulong)
        .wrapping_sub(1 as libc::c_int as libc::c_ulong);
    b = get_next(s, 0 as *mut Upper_Lower_class);
    if b == -(1 as libc::c_int) {
        return 0 as libc::c_int != 0;
    }
    loop {
        c = get_next(s, 0 as *mut Upper_Lower_class);
        if !(c != -(1 as libc::c_int)) {
            break;
        }
        if c != b {
            return 0 as libc::c_int != 0;
        }
    }
    return 1 as libc::c_int != 0;
}
unsafe extern "C" fn validate(mut s1: *mut Spec_list, mut s2: *mut Spec_list) {
    get_s1_spec_stats(s1);
    if (*s1).n_indefinite_repeats > 0 as libc::c_int as libc::c_ulong {
        if ::core::mem::size_of::<C2RustUnnamed_14>() as libc::c_ulong != 0 {
            error(
                1 as libc::c_int,
                0 as libc::c_int,
                dcgettext(
                    0 as *const libc::c_char,
                    b"the [c*] repeat construct may not appear in string1\0" as *const u8
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
                    b"the [c*] repeat construct may not appear in string1\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
            );
            if 0 as libc::c_int != 0 {} else {
                unreachable!();
            };
        };
    }
    if !s2.is_null() {
        get_s2_spec_stats(s2, (*s1).length);
        if (*s2).n_indefinite_repeats > 1 as libc::c_int as libc::c_ulong {
            if ::core::mem::size_of::<C2RustUnnamed_13>() as libc::c_ulong != 0 {
                error(
                    1 as libc::c_int,
                    0 as libc::c_int,
                    dcgettext(
                        0 as *const libc::c_char,
                        b"only one [c*] repeat construct may appear in string2\0"
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
                        b"only one [c*] repeat construct may appear in string2\0"
                            as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                );
                if 0 as libc::c_int != 0 {} else {
                    unreachable!();
                };
            };
        }
        if translating {
            if (*s2).has_equiv_class {
                if ::core::mem::size_of::<C2RustUnnamed_12>() as libc::c_ulong != 0 {
                    error(
                        1 as libc::c_int,
                        0 as libc::c_int,
                        dcgettext(
                            0 as *const libc::c_char,
                            b"[=c=] expressions may not appear in string2 when translating\0"
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
                            b"[=c=] expressions may not appear in string2 when translating\0"
                                as *const u8 as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                    );
                    if 0 as libc::c_int != 0 {} else {
                        unreachable!();
                    };
                };
            }
            if (*s2).has_restricted_char_class {
                if ::core::mem::size_of::<C2RustUnnamed_11>() as libc::c_ulong != 0 {
                    error(
                        1 as libc::c_int,
                        0 as libc::c_int,
                        dcgettext(
                            0 as *const libc::c_char,
                            b"when translating, the only character classes that may appear in\nstring2 are 'upper' and 'lower'\0"
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
                            b"when translating, the only character classes that may appear in\nstring2 are 'upper' and 'lower'\0"
                                as *const u8 as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                    );
                    if 0 as libc::c_int != 0 {} else {
                        unreachable!();
                    };
                };
            }
            validate_case_classes(s1, s2);
            if (*s1).length > (*s2).length {
                if !truncate_set1 {
                    if (*s2).length == 0 as libc::c_int as libc::c_ulong {
                        if ::core::mem::size_of::<C2RustUnnamed_10>() as libc::c_ulong
                            != 0
                        {
                            error(
                                1 as libc::c_int,
                                0 as libc::c_int,
                                dcgettext(
                                    0 as *const libc::c_char,
                                    b"when not truncating set1, string2 must be non-empty\0"
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
                                    b"when not truncating set1, string2 must be non-empty\0"
                                        as *const u8 as *const libc::c_char,
                                    5 as libc::c_int,
                                ),
                            );
                            if 0 as libc::c_int != 0 {} else {
                                unreachable!();
                            };
                        };
                    }
                    string2_extend(s1, s2);
                }
            }
            if complement as libc::c_int != 0 && (*s1).has_char_class as libc::c_int != 0
                && !((*s2).length == (*s1).length
                    && homogeneous_spec_list(s2) as libc::c_int != 0)
            {
                if ::core::mem::size_of::<C2RustUnnamed_9>() as libc::c_ulong != 0 {
                    error(
                        1 as libc::c_int,
                        0 as libc::c_int,
                        dcgettext(
                            0 as *const libc::c_char,
                            b"when translating with complemented character classes,\nstring2 must map all characters in the domain to one\0"
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
                            b"when translating with complemented character classes,\nstring2 must map all characters in the domain to one\0"
                                as *const u8 as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                    );
                    if 0 as libc::c_int != 0 {} else {
                        unreachable!();
                    };
                };
            }
        } else if (*s2).n_indefinite_repeats > 0 as libc::c_int as libc::c_ulong {
            if ::core::mem::size_of::<C2RustUnnamed_8>() as libc::c_ulong != 0 {
                error(
                    1 as libc::c_int,
                    0 as libc::c_int,
                    dcgettext(
                        0 as *const libc::c_char,
                        b"the [c*] construct may appear in string2 only when translating\0"
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
                        b"the [c*] construct may appear in string2 only when translating\0"
                            as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                );
                if 0 as libc::c_int != 0 {} else {
                    unreachable!();
                };
            };
        }
    }
}
unsafe extern "C" fn squeeze_filter(
    mut buf: *mut libc::c_char,
    mut size: size_t,
    mut reader: Option::<unsafe extern "C" fn(*mut libc::c_char, size_t) -> size_t>,
) {
    let NOT_A_CHAR: libc::c_int = 2147483647 as libc::c_int;
    let mut char_to_squeeze: libc::c_int = NOT_A_CHAR;
    let mut i: size_t = 0 as libc::c_int as size_t;
    let mut nr: size_t = 0 as libc::c_int as size_t;
    loop {
        if i >= nr {
            nr = reader.expect("non-null function pointer")(buf, size);
            if nr == 0 as libc::c_int as libc::c_ulong {
                break;
            }
            i = 0 as libc::c_int as size_t;
        }
        let mut begin: size_t = i;
        if char_to_squeeze == NOT_A_CHAR {
            let mut out_len: size_t = 0;
            while i < nr && !in_squeeze_set[to_uchar(*buf.offset(i as isize)) as usize] {
                i = (i as libc::c_ulong).wrapping_add(2 as libc::c_int as libc::c_ulong)
                    as size_t as size_t;
            }
            if i == nr
                && in_squeeze_set[to_uchar(
                    *buf
                        .offset(
                            i.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
                        ),
                ) as usize] as libc::c_int != 0
            {
                i = i.wrapping_sub(1);
            }
            if i >= nr {
                out_len = nr.wrapping_sub(begin);
            } else {
                char_to_squeeze = *buf.offset(i as isize) as libc::c_int;
                out_len = i
                    .wrapping_sub(begin)
                    .wrapping_add(1 as libc::c_int as libc::c_ulong);
                if i > 0 as libc::c_int as libc::c_ulong
                    && *buf
                        .offset(
                            i.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
                        ) as libc::c_int == char_to_squeeze
                {
                    out_len = out_len.wrapping_sub(1);
                }
                i = i.wrapping_add(1);
            }
            if out_len > 0 as libc::c_int as libc::c_ulong
                && (if 0 != 0 && 0 != 0
                    && (1 as libc::c_int as size_t).wrapping_mul(out_len)
                        <= 8 as libc::c_int as libc::c_ulong
                    && 1 as libc::c_int as size_t != 0 as libc::c_int as libc::c_ulong
                {
                    ({
                        let mut __ptr: *const libc::c_char = &mut *buf
                            .offset(begin as isize) as *mut libc::c_char
                            as *const libc::c_char;
                        let mut __stream: *mut FILE = stdout;
                        let mut __cnt: size_t = 0;
                        __cnt = (1 as libc::c_int as size_t).wrapping_mul(out_len);
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
                            .wrapping_mul(out_len)
                            .wrapping_sub(__cnt)
                            .wrapping_div(1 as libc::c_int as size_t)
                    })
                } else {
                    (if 0 != 0
                        && 1 as libc::c_int as size_t
                            == 0 as libc::c_int as libc::c_ulong
                        || 0 != 0 && out_len == 0 as libc::c_int as libc::c_ulong
                    {
                        0 as libc::c_int as size_t
                    } else {
                        fwrite_unlocked(
                            &mut *buf.offset(begin as isize) as *mut libc::c_char
                                as *const libc::c_void,
                            1 as libc::c_int as size_t,
                            out_len,
                            stdout,
                        )
                    })
                }) != out_len
            {
                if ::core::mem::size_of::<C2RustUnnamed_15>() as libc::c_ulong != 0 {
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
        }
        if char_to_squeeze != NOT_A_CHAR {
            while i < nr && *buf.offset(i as isize) as libc::c_int == char_to_squeeze {
                i = i.wrapping_add(1);
            }
            if i < nr {
                char_to_squeeze = NOT_A_CHAR;
            }
        }
    };
}
unsafe extern "C" fn plain_read(mut buf: *mut libc::c_char, mut size: size_t) -> size_t {
    let mut nr: size_t = safe_read(0 as libc::c_int, buf as *mut libc::c_void, size);
    if nr == -(1 as libc::c_int) as size_t {
        if ::core::mem::size_of::<C2RustUnnamed_16>() as libc::c_ulong != 0 {
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
    return nr;
}
unsafe extern "C" fn read_and_delete(
    mut buf: *mut libc::c_char,
    mut size: size_t,
) -> size_t {
    let mut n_saved: size_t = 0;
    loop {
        let mut nr: size_t = plain_read(buf, size);
        if nr == 0 as libc::c_int as libc::c_ulong {
            return 0 as libc::c_int as size_t;
        }
        let mut i: size_t = 0;
        i = 0 as libc::c_int as size_t;
        while i < nr && !in_delete_set[to_uchar(*buf.offset(i as isize)) as usize] {
            i = i.wrapping_add(1);
        }
        n_saved = i;
        i = i.wrapping_add(1);
        while i < nr {
            if !in_delete_set[to_uchar(*buf.offset(i as isize)) as usize] {
                let fresh4 = n_saved;
                n_saved = n_saved.wrapping_add(1);
                *buf.offset(fresh4 as isize) = *buf.offset(i as isize);
            }
            i = i.wrapping_add(1);
        }
        if !(n_saved == 0 as libc::c_int as libc::c_ulong) {
            break;
        }
    }
    return n_saved;
}
unsafe extern "C" fn read_and_xlate(
    mut buf: *mut libc::c_char,
    mut size: size_t,
) -> size_t {
    let mut bytes_read: size_t = plain_read(buf, size);
    let mut i: size_t = 0 as libc::c_int as size_t;
    while i < bytes_read {
        *buf.offset(i as isize) = xlate[to_uchar(*buf.offset(i as isize)) as usize];
        i = i.wrapping_add(1);
    }
    return bytes_read;
}
unsafe extern "C" fn set_initialize(
    mut s: *mut Spec_list,
    mut complement_this_set: bool,
    mut in_set: *mut bool,
) {
    let mut c: libc::c_int = 0;
    (*s)
        .state = (18446744073709551615 as libc::c_ulong)
        .wrapping_sub(1 as libc::c_int as libc::c_ulong);
    loop {
        c = get_next(s, 0 as *mut Upper_Lower_class);
        if !(c != -(1 as libc::c_int)) {
            break;
        }
        *in_set.offset(c as isize) = 1 as libc::c_int != 0;
    }
    if complement_this_set {
        let mut i: size_t = 0 as libc::c_int as size_t;
        while i < N_CHARS as libc::c_int as libc::c_ulong {
            *in_set.offset(i as isize) = !*in_set.offset(i as isize);
            i = i.wrapping_add(1);
        }
    }
}
unsafe fn main_0(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut c: libc::c_int = 0;
    let mut non_option_args: libc::c_int = 0;
    let mut min_operands: libc::c_int = 0;
    let mut max_operands: libc::c_int = 0;
    let mut buf1: Spec_list = Spec_list {
        head: 0 as *mut List_element,
        tail: 0 as *mut List_element,
        state: 0,
        length: 0,
        n_indefinite_repeats: 0,
        indefinite_repeat_element: 0 as *mut List_element,
        has_equiv_class: false,
        has_char_class: false,
        has_restricted_char_class: false,
    };
    let mut buf2: Spec_list = Spec_list {
        head: 0 as *mut List_element,
        tail: 0 as *mut List_element,
        state: 0,
        length: 0,
        n_indefinite_repeats: 0,
        indefinite_repeat_element: 0 as *mut List_element,
        has_equiv_class: false,
        has_char_class: false,
        has_restricted_char_class: false,
    };
    let mut s1: *mut Spec_list = &mut buf1;
    let mut s2: *mut Spec_list = &mut buf2;
    set_program_name(*argv.offset(0 as libc::c_int as isize));
    setlocale(6 as libc::c_int, b"\0" as *const u8 as *const libc::c_char);
    bindtextdomain(
        b"coreutils\0" as *const u8 as *const libc::c_char,
        b"/usr/local/share/locale\0" as *const u8 as *const libc::c_char,
    );
    textdomain(b"coreutils\0" as *const u8 as *const libc::c_char);
    atexit(Some(close_stdout as unsafe extern "C" fn() -> ()));
    loop {
        c = getopt_long(
            argc,
            argv,
            b"+AcCdst\0" as *const u8 as *const libc::c_char,
            long_options.as_ptr(),
            0 as *mut libc::c_int,
        );
        if !(c != -(1 as libc::c_int)) {
            break;
        }
        match c {
            65 => {
                setlocale(3 as libc::c_int, b"C\0" as *const u8 as *const libc::c_char);
                setlocale(0 as libc::c_int, b"C\0" as *const u8 as *const libc::c_char);
            }
            99 | 67 => {
                complement = 1 as libc::c_int != 0;
            }
            100 => {
                delete = 1 as libc::c_int != 0;
            }
            115 => {
                squeeze_repeats = 1 as libc::c_int != 0;
            }
            116 => {
                truncate_set1 = 1 as libc::c_int != 0;
            }
            -2 => {
                usage(0 as libc::c_int);
            }
            -3 => {
                version_etc(
                    stdout,
                    b"tr\0" as *const u8 as *const libc::c_char,
                    b"GNU coreutils\0" as *const u8 as *const libc::c_char,
                    Version,
                    b"Jim Meyering\0" as *const u8 as *const libc::c_char,
                    0 as *mut libc::c_void as *mut libc::c_char,
                );
                exit(0 as libc::c_int);
            }
            _ => {
                usage(1 as libc::c_int);
            }
        }
    }
    non_option_args = argc - optind;
    translating = non_option_args == 2 as libc::c_int && !delete;
    min_operands = 1 as libc::c_int
        + (delete as libc::c_int == squeeze_repeats as libc::c_int) as libc::c_int;
    max_operands = 1 as libc::c_int
        + (delete as libc::c_int <= squeeze_repeats as libc::c_int) as libc::c_int;
    if non_option_args < min_operands {
        if non_option_args == 0 as libc::c_int {
            error(
                0 as libc::c_int,
                0 as libc::c_int,
                dcgettext(
                    0 as *const libc::c_char,
                    b"missing operand\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
            );
        } else {
            error(
                0 as libc::c_int,
                0 as libc::c_int,
                dcgettext(
                    0 as *const libc::c_char,
                    b"missing operand after %s\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
                quote(*argv.offset((argc - 1 as libc::c_int) as isize)),
            );
            fprintf(
                stderr,
                b"%s\n\0" as *const u8 as *const libc::c_char,
                dcgettext(
                    0 as *const libc::c_char,
                    if squeeze_repeats as libc::c_int != 0 {
                        b"Two strings must be given when both deleting and squeezing repeats.\0"
                            as *const u8 as *const libc::c_char
                    } else {
                        b"Two strings must be given when translating.\0" as *const u8
                            as *const libc::c_char
                    },
                    5 as libc::c_int,
                ),
            );
        }
        usage(1 as libc::c_int);
    }
    if max_operands < non_option_args {
        error(
            0 as libc::c_int,
            0 as libc::c_int,
            dcgettext(
                0 as *const libc::c_char,
                b"extra operand %s\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            quote(*argv.offset((optind + max_operands) as isize)),
        );
        if non_option_args == 2 as libc::c_int {
            fprintf(
                stderr,
                b"%s\n\0" as *const u8 as *const libc::c_char,
                dcgettext(
                    0 as *const libc::c_char,
                    b"Only one string may be given when deleting without squeezing repeats.\0"
                        as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
            );
        }
        usage(1 as libc::c_int);
    }
    spec_init(s1);
    if !parse_str(*argv.offset(optind as isize), s1) {
        exit(1 as libc::c_int);
    }
    if non_option_args == 2 as libc::c_int {
        spec_init(s2);
        if !parse_str(*argv.offset((optind + 1 as libc::c_int) as isize), s2) {
            exit(1 as libc::c_int);
        }
    } else {
        s2 = 0 as *mut Spec_list;
    }
    validate(s1, s2);
    xset_binary_mode(0 as libc::c_int, 0 as libc::c_int);
    xset_binary_mode(1 as libc::c_int, 0 as libc::c_int);
    fadvise(stdin, FADVISE_SEQUENTIAL);
    if squeeze_repeats as libc::c_int != 0 && non_option_args == 1 as libc::c_int {
        set_initialize(s1, complement, in_squeeze_set.as_mut_ptr());
        squeeze_filter(
            io_buf.as_mut_ptr(),
            ::core::mem::size_of::<[libc::c_char; 8192]>() as libc::c_ulong,
            Some(plain_read as unsafe extern "C" fn(*mut libc::c_char, size_t) -> size_t),
        );
    } else if delete as libc::c_int != 0 && non_option_args == 1 as libc::c_int {
        set_initialize(s1, complement, in_delete_set.as_mut_ptr());
        loop {
            let mut nr: size_t = read_and_delete(
                io_buf.as_mut_ptr(),
                ::core::mem::size_of::<[libc::c_char; 8192]>() as libc::c_ulong,
            );
            if nr == 0 as libc::c_int as libc::c_ulong {
                break;
            }
            if (if 0 != 0 && 0 != 0
                && (1 as libc::c_int as size_t).wrapping_mul(nr)
                    <= 8 as libc::c_int as libc::c_ulong
                && 1 as libc::c_int as size_t != 0 as libc::c_int as libc::c_ulong
            {
                ({
                    let mut __ptr: *const libc::c_char = io_buf.as_mut_ptr()
                        as *const libc::c_char;
                    let mut __stream: *mut FILE = stdout;
                    let mut __cnt: size_t = 0;
                    __cnt = (1 as libc::c_int as size_t).wrapping_mul(nr);
                    while __cnt > 0 as libc::c_int as libc::c_ulong {
                        let fresh5 = __ptr;
                        __ptr = __ptr.offset(1);
                        if putc_unlocked(*fresh5 as libc::c_int, __stream)
                            == -(1 as libc::c_int)
                        {
                            break;
                        }
                        __cnt = __cnt.wrapping_sub(1);
                    }
                    (1 as libc::c_int as size_t)
                        .wrapping_mul(nr)
                        .wrapping_sub(__cnt)
                        .wrapping_div(1 as libc::c_int as size_t)
                })
            } else {
                (if 0 != 0
                    && 1 as libc::c_int as size_t == 0 as libc::c_int as libc::c_ulong
                    || 0 != 0 && nr == 0 as libc::c_int as libc::c_ulong
                {
                    0 as libc::c_int as size_t
                } else {
                    fwrite_unlocked(
                        io_buf.as_mut_ptr() as *const libc::c_void,
                        1 as libc::c_int as size_t,
                        nr,
                        stdout,
                    )
                })
            }) != nr
            {
                if ::core::mem::size_of::<C2RustUnnamed_19>() as libc::c_ulong != 0 {
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
        }
    } else if squeeze_repeats as libc::c_int != 0 && delete as libc::c_int != 0
        && non_option_args == 2 as libc::c_int
    {
        set_initialize(s1, complement, in_delete_set.as_mut_ptr());
        set_initialize(s2, 0 as libc::c_int != 0, in_squeeze_set.as_mut_ptr());
        squeeze_filter(
            io_buf.as_mut_ptr(),
            ::core::mem::size_of::<[libc::c_char; 8192]>() as libc::c_ulong,
            Some(
                read_and_delete
                    as unsafe extern "C" fn(*mut libc::c_char, size_t) -> size_t,
            ),
        );
    } else if translating {
        if complement {
            let mut in_s1: *mut bool = in_delete_set.as_mut_ptr();
            set_initialize(s1, 0 as libc::c_int != 0, in_s1);
            (*s2)
                .state = (18446744073709551615 as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong);
            let mut i: libc::c_int = 0 as libc::c_int;
            while i < N_CHARS as libc::c_int {
                xlate[i as usize] = i as libc::c_char;
                i += 1;
            }
            let mut i_0: libc::c_int = 0 as libc::c_int;
            while i_0 < N_CHARS as libc::c_int {
                if !*in_s1.offset(i_0 as isize) {
                    let mut ch: libc::c_int = get_next(s2, 0 as *mut Upper_Lower_class);
                    if ch != -(1 as libc::c_int) || truncate_set1 as libc::c_int != 0
                    {} else {
                        __assert_fail(
                            b"ch != -1 || truncate_set1\0" as *const u8
                                as *const libc::c_char,
                            b"src/tr.c\0" as *const u8 as *const libc::c_char,
                            1840 as libc::c_int as libc::c_uint,
                            (*::core::mem::transmute::<
                                &[u8; 23],
                                &[libc::c_char; 23],
                            >(b"int main(int, char **)\0"))
                                .as_ptr(),
                        );
                    }
                    if ch == -(1 as libc::c_int) {
                        break;
                    }
                    xlate[i_0 as usize] = ch as libc::c_char;
                }
                i_0 += 1;
            }
        } else {
            let mut c1: libc::c_int = 0;
            let mut c2: libc::c_int = 0;
            let mut class_s1: Upper_Lower_class = UL_LOWER;
            let mut class_s2: Upper_Lower_class = UL_LOWER;
            let mut i_1: libc::c_int = 0 as libc::c_int;
            while i_1 < N_CHARS as libc::c_int {
                xlate[i_1 as usize] = i_1 as libc::c_char;
                i_1 += 1;
            }
            (*s1)
                .state = (18446744073709551615 as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong);
            (*s2)
                .state = (18446744073709551615 as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong);
            loop {
                c1 = get_next(s1, &mut class_s1);
                c2 = get_next(s2, &mut class_s2);
                if class_s1 as libc::c_uint == UL_LOWER as libc::c_int as libc::c_uint
                    && class_s2 as libc::c_uint
                        == UL_UPPER as libc::c_int as libc::c_uint
                {
                    let mut i_2: libc::c_int = 0 as libc::c_int;
                    while i_2 < N_CHARS as libc::c_int {
                        if *(*__ctype_b_loc()).offset(i_2 as isize) as libc::c_int
                            & _ISlower as libc::c_int as libc::c_ushort as libc::c_int
                            != 0
                        {
                            xlate[i_2
                                as usize] = ({
                                let mut __res: libc::c_int = 0;
                                if ::core::mem::size_of::<libc::c_int>() as libc::c_ulong
                                    > 1 as libc::c_int as libc::c_ulong
                                {
                                    if 0 != 0 {
                                        let mut __c: libc::c_int = i_2;
                                        __res = if __c < -(128 as libc::c_int)
                                            || __c > 255 as libc::c_int
                                        {
                                            __c
                                        } else {
                                            *(*__ctype_toupper_loc()).offset(__c as isize)
                                        };
                                    } else {
                                        __res = toupper(i_2);
                                    }
                                } else {
                                    __res = *(*__ctype_toupper_loc()).offset(i_2 as isize);
                                }
                                __res
                            }) as libc::c_char;
                        }
                        i_2 += 1;
                    }
                } else if class_s1 as libc::c_uint
                    == UL_UPPER as libc::c_int as libc::c_uint
                    && class_s2 as libc::c_uint
                        == UL_LOWER as libc::c_int as libc::c_uint
                {
                    let mut i_3: libc::c_int = 0 as libc::c_int;
                    while i_3 < N_CHARS as libc::c_int {
                        if *(*__ctype_b_loc()).offset(i_3 as isize) as libc::c_int
                            & _ISupper as libc::c_int as libc::c_ushort as libc::c_int
                            != 0
                        {
                            xlate[i_3
                                as usize] = ({
                                let mut __res: libc::c_int = 0;
                                if ::core::mem::size_of::<libc::c_int>() as libc::c_ulong
                                    > 1 as libc::c_int as libc::c_ulong
                                {
                                    if 0 != 0 {
                                        let mut __c: libc::c_int = i_3;
                                        __res = if __c < -(128 as libc::c_int)
                                            || __c > 255 as libc::c_int
                                        {
                                            __c
                                        } else {
                                            *(*__ctype_tolower_loc()).offset(__c as isize)
                                        };
                                    } else {
                                        __res = tolower(i_3);
                                    }
                                } else {
                                    __res = *(*__ctype_tolower_loc()).offset(i_3 as isize);
                                }
                                __res
                            }) as libc::c_char;
                        }
                        i_3 += 1;
                    }
                } else {
                    if c1 == -(1 as libc::c_int) || c2 == -(1 as libc::c_int) {
                        break;
                    }
                    xlate[c1 as usize] = c2 as libc::c_char;
                }
                if class_s2 as libc::c_uint != UL_NONE as libc::c_int as libc::c_uint {
                    skip_construct(s1);
                    skip_construct(s2);
                }
            }
            if c1 == -(1 as libc::c_int) || truncate_set1 as libc::c_int != 0 {} else {
                __assert_fail(
                    b"c1 == -1 || truncate_set1\0" as *const u8 as *const libc::c_char,
                    b"src/tr.c\0" as *const u8 as *const libc::c_char,
                    1893 as libc::c_int as libc::c_uint,
                    (*::core::mem::transmute::<
                        &[u8; 23],
                        &[libc::c_char; 23],
                    >(b"int main(int, char **)\0"))
                        .as_ptr(),
                );
            }
        }
        if squeeze_repeats {
            set_initialize(s2, 0 as libc::c_int != 0, in_squeeze_set.as_mut_ptr());
            squeeze_filter(
                io_buf.as_mut_ptr(),
                ::core::mem::size_of::<[libc::c_char; 8192]>() as libc::c_ulong,
                Some(
                    read_and_xlate
                        as unsafe extern "C" fn(*mut libc::c_char, size_t) -> size_t,
                ),
            );
        } else {
            loop {
                let mut bytes_read: size_t = read_and_xlate(
                    io_buf.as_mut_ptr(),
                    ::core::mem::size_of::<[libc::c_char; 8192]>() as libc::c_ulong,
                );
                if bytes_read == 0 as libc::c_int as libc::c_ulong {
                    break;
                }
                if (if 0 != 0 && 0 != 0
                    && (1 as libc::c_int as size_t).wrapping_mul(bytes_read)
                        <= 8 as libc::c_int as libc::c_ulong
                    && 1 as libc::c_int as size_t != 0 as libc::c_int as libc::c_ulong
                {
                    ({
                        let mut __ptr: *const libc::c_char = io_buf.as_mut_ptr()
                            as *const libc::c_char;
                        let mut __stream: *mut FILE = stdout;
                        let mut __cnt: size_t = 0;
                        __cnt = (1 as libc::c_int as size_t).wrapping_mul(bytes_read);
                        while __cnt > 0 as libc::c_int as libc::c_ulong {
                            let fresh6 = __ptr;
                            __ptr = __ptr.offset(1);
                            if putc_unlocked(*fresh6 as libc::c_int, __stream)
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
                    (if 0 != 0
                        && 1 as libc::c_int as size_t
                            == 0 as libc::c_int as libc::c_ulong
                        || 0 != 0 && bytes_read == 0 as libc::c_int as libc::c_ulong
                    {
                        0 as libc::c_int as size_t
                    } else {
                        fwrite_unlocked(
                            io_buf.as_mut_ptr() as *const libc::c_void,
                            1 as libc::c_int as size_t,
                            bytes_read,
                            stdout,
                        )
                    })
                }) != bytes_read
                {
                    if ::core::mem::size_of::<C2RustUnnamed_18>() as libc::c_ulong != 0 {
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
            }
        }
    }
    if close(0 as libc::c_int) != 0 as libc::c_int {
        if ::core::mem::size_of::<C2RustUnnamed_17>() as libc::c_ulong != 0 {
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
    exit(0 as libc::c_int);
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
