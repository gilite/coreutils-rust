#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![feature(extern_types)]
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    static mut optarg: *mut libc::c_char;
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
    fn getenv(__name: *const libc::c_char) -> *mut libc::c_char;
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
    fn free(_: *mut libc::c_void);
    fn __errno_location() -> *mut libc::c_int;
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
    static mut stdin: *mut FILE;
    static mut stdout: *mut FILE;
    static mut stderr: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn __overflow(_: *mut FILE, _: libc::c_int) -> libc::c_int;
    fn fputs_unlocked(__s: *const libc::c_char, __stream: *mut FILE) -> libc::c_int;
    fn fwrite_unlocked(
        __ptr: *const libc::c_void,
        __size: size_t,
        __n: size_t,
        __stream: *mut FILE,
    ) -> size_t;
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
    fn quotearg_style(s: quoting_style, arg: *const libc::c_char) -> *mut libc::c_char;
    fn quotearg_n_style_colon(
        n: libc::c_int,
        s: quoting_style,
        arg: *const libc::c_char,
    ) -> *mut libc::c_char;
    fn quote(arg: *const libc::c_char) -> *const libc::c_char;
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
    fn initbuffer(linebuffer: *mut linebuffer);
    fn readlinebuffer_delim(
        linebuffer: *mut linebuffer,
        stream: *mut FILE,
        delimiter: libc::c_char,
    ) -> *mut linebuffer;
    fn error(
        __status: libc::c_int,
        __errnum: libc::c_int,
        __format: *const libc::c_char,
        _: ...
    );
    fn fadvise(fp: *mut FILE, advice: fadvice_t);
    fn posix2_version() -> libc::c_int;
    fn freopen_safer(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: *mut FILE,
    ) -> *mut FILE;
    fn xstrtoumax(
        _: *const libc::c_char,
        _: *mut *mut libc::c_char,
        _: libc::c_int,
        _: *mut uintmax_t,
        _: *const libc::c_char,
    ) -> strtol_error;
    fn memcasecmp(
        vs1: *const libc::c_void,
        vs2: *const libc::c_void,
        n: size_t,
    ) -> libc::c_int;
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct option {
    pub name: *const libc::c_char,
    pub has_arg: libc::c_int,
    pub flag: *mut libc::c_int,
    pub val: libc::c_int,
}
pub type __uintmax_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type size_t = libc::c_ulong;
pub type ptrdiff_t = libc::c_long;
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
pub type idx_t = ptrdiff_t;
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
pub type argmatch_exit_fn = Option::<unsafe extern "C" fn() -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct linebuffer {
    pub size: idx_t,
    pub length: idx_t,
    pub buffer: *mut libc::c_char,
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
pub type countmode = libc::c_uint;
pub const count_none: countmode = 1;
pub const count_occurrences: countmode = 0;
pub type delimit_method = libc::c_uint;
pub const DM_SEPARATE: delimit_method = 2;
pub const DM_PREPEND: delimit_method = 1;
pub const DM_NONE: delimit_method = 0;
pub type grouping_method = libc::c_uint;
pub const GM_BOTH: grouping_method = 4;
pub const GM_SEPARATE: grouping_method = 3;
pub const GM_APPEND: grouping_method = 2;
pub const GM_PREPEND: grouping_method = 1;
pub const GM_NONE: grouping_method = 0;
pub type C2RustUnnamed_1 = libc::c_uint;
pub const GROUP_OPTION: C2RustUnnamed_1 = 256;
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
pub type Skip_field_option_type = libc::c_uint;
pub const SFO_NEW: Skip_field_option_type = 2;
pub const SFO_OBSOLETE: Skip_field_option_type = 1;
pub const SFO_NONE: Skip_field_option_type = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_7 {
    pub _gl_dummy: libc::c_int,
}
#[inline]
unsafe extern "C" fn field_sep(mut ch: libc::c_uchar) -> bool {
    return *(*__ctype_b_loc()).offset(ch as libc::c_int as isize) as libc::c_int
        & _ISblank as libc::c_int as libc::c_ushort as libc::c_int != 0
        || ch as libc::c_int == '\n' as i32;
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
unsafe extern "C" fn feof_unlocked(mut __stream: *mut FILE) -> libc::c_int {
    return ((*__stream)._flags & 0x10 as libc::c_int != 0 as libc::c_int) as libc::c_int;
}
#[inline]
unsafe extern "C" fn ferror_unlocked(mut __stream: *mut FILE) -> libc::c_int {
    return ((*__stream)._flags & 0x20 as libc::c_int != 0 as libc::c_int) as libc::c_int;
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
static mut skip_fields: size_t = 0;
static mut skip_chars: size_t = 0;
static mut check_chars: size_t = 0;
static mut countmode: countmode = count_occurrences;
static mut output_unique: bool = false;
static mut output_first_repeated: bool = false;
static mut output_later_repeated: bool = false;
static mut ignore_case: bool = false;
static mut delimit_method_string: [*const libc::c_char; 4] = [
    b"none\0" as *const u8 as *const libc::c_char,
    b"prepend\0" as *const u8 as *const libc::c_char,
    b"separate\0" as *const u8 as *const libc::c_char,
    0 as *const libc::c_char,
];
static mut delimit_method_map: [delimit_method; 3] = [DM_NONE, DM_PREPEND, DM_SEPARATE];
static mut delimit_groups: delimit_method = DM_NONE;
static mut grouping_method_string: [*const libc::c_char; 5] = [
    b"prepend\0" as *const u8 as *const libc::c_char,
    b"append\0" as *const u8 as *const libc::c_char,
    b"separate\0" as *const u8 as *const libc::c_char,
    b"both\0" as *const u8 as *const libc::c_char,
    0 as *const libc::c_char,
];
static mut grouping_method_map: [grouping_method; 4] = [
    GM_PREPEND,
    GM_APPEND,
    GM_SEPARATE,
    GM_BOTH,
];
static mut grouping: grouping_method = GM_NONE;
static mut longopts: [option; 13] = [
    {
        let mut init = option {
            name: b"count\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'c' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"repeated\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'd' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"all-repeated\0" as *const u8 as *const libc::c_char,
            has_arg: 2 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'D' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"group\0" as *const u8 as *const libc::c_char,
            has_arg: 2 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: GROUP_OPTION as libc::c_int,
        };
        init
    },
    {
        let mut init = option {
            name: b"ignore-case\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'i' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"unique\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'u' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"skip-fields\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'f' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"skip-chars\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 's' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"check-chars\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'w' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"zero-terminated\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'z' as i32,
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
                b"Usage: %s [OPTION]... [INPUT [OUTPUT]]\n\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
            program_name,
        );
        fputs_unlocked(
            dcgettext(
                0 as *const libc::c_char,
                b"Filter adjacent matching lines from INPUT (or standard input),\nwriting to OUTPUT (or standard output).\n\nWith no options, matching lines are merged to the first occurrence.\n\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            stdout,
        );
        emit_mandatory_arg_note();
        fputs_unlocked(
            dcgettext(
                0 as *const libc::c_char,
                b"  -c, --count           prefix lines by the number of occurrences\n  -d, --repeated        only print duplicate lines, one for each group\n\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            stdout,
        );
        fputs_unlocked(
            dcgettext(
                0 as *const libc::c_char,
                b"  -D                    print all duplicate lines\n      --all-repeated[=METHOD]  like -D, but allow separating groups\n                                 with an empty line;\n                                 METHOD={none(default),prepend,separate}\n\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            stdout,
        );
        fputs_unlocked(
            dcgettext(
                0 as *const libc::c_char,
                b"  -f, --skip-fields=N   avoid comparing the first N fields\n\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            stdout,
        );
        fputs_unlocked(
            dcgettext(
                0 as *const libc::c_char,
                b"      --group[=METHOD]  show all items, separating groups with an empty line;\n                          METHOD={separate(default),prepend,append,both}\n\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            stdout,
        );
        fputs_unlocked(
            dcgettext(
                0 as *const libc::c_char,
                b"  -i, --ignore-case     ignore differences in case when comparing\n  -s, --skip-chars=N    avoid comparing the first N characters\n  -u, --unique          only print unique lines\n\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            stdout,
        );
        fputs_unlocked(
            dcgettext(
                0 as *const libc::c_char,
                b"  -z, --zero-terminated     line delimiter is NUL, not newline\n\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            stdout,
        );
        fputs_unlocked(
            dcgettext(
                0 as *const libc::c_char,
                b"  -w, --check-chars=N   compare no more than N characters in lines\n\0"
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
                b"\nA field is a run of blanks (usually spaces and/or TABs), then non-blank\ncharacters.  Fields are skipped before chars.\n\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            stdout,
        );
        fputs_unlocked(
            dcgettext(
                0 as *const libc::c_char,
                b"\nNote: 'uniq' does not detect repeated lines unless they are adjacent.\nYou may want to sort the input first, or use 'sort -u' without 'uniq'.\n\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            stdout,
        );
        emit_ancillary_info(b"uniq\0" as *const u8 as *const libc::c_char);
    }
    exit(status);
}
unsafe extern "C" fn strict_posix2() -> bool {
    let mut posix_ver: libc::c_int = posix2_version();
    return 200112 as libc::c_int <= posix_ver && posix_ver < 200809 as libc::c_int;
}
unsafe extern "C" fn size_opt(
    mut opt: *const libc::c_char,
    mut msgid: *const libc::c_char,
) -> size_t {
    let mut size: uintmax_t = 0;
    match xstrtoumax(
        opt,
        0 as *mut *mut libc::c_char,
        10 as libc::c_int,
        &mut size,
        b"\0" as *const u8 as *const libc::c_char,
    ) as libc::c_uint
    {
        0 | 1 => {}
        _ => {
            if ::core::mem::size_of::<C2RustUnnamed_2>() as libc::c_ulong != 0 {
                error(
                    1 as libc::c_int,
                    0 as libc::c_int,
                    b"%s: %s\0" as *const u8 as *const libc::c_char,
                    opt,
                    dcgettext(0 as *const libc::c_char, msgid, 5 as libc::c_int),
                );
                if 0 as libc::c_int != 0 {} else {
                    unreachable!();
                };
            } else {
                error(
                    1 as libc::c_int,
                    0 as libc::c_int,
                    b"%s: %s\0" as *const u8 as *const libc::c_char,
                    opt,
                    dcgettext(0 as *const libc::c_char, msgid, 5 as libc::c_int),
                );
                if 0 as libc::c_int != 0 {} else {
                    unreachable!();
                };
            };
        }
    }
    return if size < 18446744073709551615 as libc::c_ulong {
        size
    } else {
        18446744073709551615 as libc::c_ulong
    };
}
unsafe extern "C" fn find_field(mut line: *const linebuffer) -> *mut libc::c_char {
    let mut count: size_t = 0;
    let mut lp: *const libc::c_char = (*line).buffer;
    let mut size: size_t = ((*line).length - 1 as libc::c_int as libc::c_long) as size_t;
    let mut i: size_t = 0 as libc::c_int as size_t;
    count = 0 as libc::c_int as size_t;
    while count < skip_fields && i < size {
        while i < size
            && field_sep(*lp.offset(i as isize) as libc::c_uchar) as libc::c_int != 0
        {
            i = i.wrapping_add(1);
        }
        while i < size && !field_sep(*lp.offset(i as isize) as libc::c_uchar) {
            i = i.wrapping_add(1);
        }
        count = count.wrapping_add(1);
    }
    i = (i as libc::c_ulong)
        .wrapping_add(
            if skip_chars < size.wrapping_sub(i) {
                skip_chars
            } else {
                size.wrapping_sub(i)
            },
        ) as size_t as size_t;
    return ((*line).buffer).offset(i as isize);
}
unsafe extern "C" fn different(
    mut old: *mut libc::c_char,
    mut new: *mut libc::c_char,
    mut oldlen: size_t,
    mut newlen: size_t,
) -> bool {
    if check_chars < oldlen {
        oldlen = check_chars;
    }
    if check_chars < newlen {
        newlen = check_chars;
    }
    if ignore_case {
        return oldlen != newlen
            || memcasecmp(old as *const libc::c_void, new as *const libc::c_void, oldlen)
                != 0
    } else {
        return oldlen != newlen
            || memcmp(old as *const libc::c_void, new as *const libc::c_void, oldlen)
                != 0
    };
}
unsafe extern "C" fn writeline(
    mut line: *const linebuffer,
    mut match_0: bool,
    mut linecount: uintmax_t,
) {
    if if linecount == 0 as libc::c_int as libc::c_ulong {
        output_unique as libc::c_int
    } else if !match_0 {
        output_first_repeated as libc::c_int
    } else {
        output_later_repeated as libc::c_int
    } == 0
    {
        return;
    }
    if countmode as libc::c_uint == count_occurrences as libc::c_int as libc::c_uint {
        printf(
            b"%7lu \0" as *const u8 as *const libc::c_char,
            linecount.wrapping_add(1 as libc::c_int as libc::c_ulong),
        );
    }
    if 0 != 0 && 0 != 0
        && (::core::mem::size_of::<libc::c_char>() as libc::c_ulong)
            .wrapping_mul((*line).length as size_t) <= 8 as libc::c_int as libc::c_ulong
        && ::core::mem::size_of::<libc::c_char>() as libc::c_ulong
            != 0 as libc::c_int as libc::c_ulong
    {
        ({
            let mut __ptr: *const libc::c_char = (*line).buffer as *const libc::c_char;
            let mut __stream: *mut FILE = stdout;
            let mut __cnt: size_t = 0;
            __cnt = (::core::mem::size_of::<libc::c_char>() as libc::c_ulong)
                .wrapping_mul((*line).length as size_t);
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
            || 0 != 0 && (*line).length as size_t == 0 as libc::c_int as libc::c_ulong
        {} else {
            fwrite_unlocked(
                (*line).buffer as *const libc::c_void,
                ::core::mem::size_of::<libc::c_char>() as libc::c_ulong,
                (*line).length as size_t,
                stdout,
            );
        };
    };
}
unsafe extern "C" fn check_file(
    mut infile: *const libc::c_char,
    mut outfile: *const libc::c_char,
    mut delimiter: libc::c_char,
) {
    let mut current_block: u64;
    let mut lb1: linebuffer = linebuffer {
        size: 0,
        length: 0,
        buffer: 0 as *mut libc::c_char,
    };
    let mut lb2: linebuffer = linebuffer {
        size: 0,
        length: 0,
        buffer: 0 as *mut libc::c_char,
    };
    let mut thisline: *mut linebuffer = 0 as *mut linebuffer;
    let mut prevline: *mut linebuffer = 0 as *mut linebuffer;
    if !(strcmp(infile, b"-\0" as *const u8 as *const libc::c_char) == 0 as libc::c_int
        || !(freopen_safer(infile, b"r\0" as *const u8 as *const libc::c_char, stdin))
            .is_null())
    {
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
    if !(strcmp(outfile, b"-\0" as *const u8 as *const libc::c_char) == 0 as libc::c_int
        || !(freopen_safer(outfile, b"w\0" as *const u8 as *const libc::c_char, stdout))
            .is_null())
    {
        if ::core::mem::size_of::<C2RustUnnamed_5>() as libc::c_ulong != 0 {
            error(
                1 as libc::c_int,
                *__errno_location(),
                b"%s\0" as *const u8 as *const libc::c_char,
                quotearg_n_style_colon(
                    0 as libc::c_int,
                    shell_escape_quoting_style,
                    outfile,
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
                    outfile,
                ),
            );
            if 0 as libc::c_int != 0 {} else {
                unreachable!();
            };
        };
    }
    fadvise(stdin, FADVISE_SEQUENTIAL);
    thisline = &mut lb1;
    prevline = &mut lb2;
    initbuffer(thisline);
    initbuffer(prevline);
    if output_unique as libc::c_int != 0 && output_first_repeated as libc::c_int != 0
        && countmode as libc::c_uint == count_none as libc::c_int as libc::c_uint
    {
        let mut prevfield: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut prevlen: size_t = 0;
        let mut first_group_printed: bool = 0 as libc::c_int != 0;
        while feof_unlocked(stdin) == 0 {
            let mut thisfield: *mut libc::c_char = 0 as *mut libc::c_char;
            let mut thislen: size_t = 0;
            let mut new_group: bool = false;
            if (readlinebuffer_delim(thisline, stdin, delimiter)).is_null() {
                break;
            }
            thisfield = find_field(thisline);
            thislen = ((*thisline).length - 1 as libc::c_int as libc::c_long
                - thisfield.offset_from((*thisline).buffer) as libc::c_long) as size_t;
            new_group = prevfield.is_null()
                || different(thisfield, prevfield, thislen, prevlen) as libc::c_int != 0;
            if new_group as libc::c_int != 0
                && grouping as libc::c_uint != GM_NONE as libc::c_int as libc::c_uint
                && (grouping as libc::c_uint == GM_PREPEND as libc::c_int as libc::c_uint
                    || grouping as libc::c_uint == GM_BOTH as libc::c_int as libc::c_uint
                    || first_group_printed as libc::c_int != 0
                        && (grouping as libc::c_uint
                            == GM_APPEND as libc::c_int as libc::c_uint
                            || grouping as libc::c_uint
                                == GM_SEPARATE as libc::c_int as libc::c_uint))
            {
                putchar_unlocked(delimiter as libc::c_int);
            }
            if new_group as libc::c_int != 0
                || grouping as libc::c_uint != GM_NONE as libc::c_int as libc::c_uint
            {
                if 0 != 0 && 0 != 0
                    && (::core::mem::size_of::<libc::c_char>() as libc::c_ulong)
                        .wrapping_mul((*thisline).length as size_t)
                        <= 8 as libc::c_int as libc::c_ulong
                    && ::core::mem::size_of::<libc::c_char>() as libc::c_ulong
                        != 0 as libc::c_int as libc::c_ulong
                {
                    ({
                        let mut __ptr: *const libc::c_char = (*thisline).buffer
                            as *const libc::c_char;
                        let mut __stream: *mut FILE = stdout;
                        let mut __cnt: size_t = 0;
                        __cnt = (::core::mem::size_of::<libc::c_char>() as libc::c_ulong)
                            .wrapping_mul((*thisline).length as size_t);
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
                        compile_error!("Binary expression is not supposed to be used")
                    });
                } else {
                    if 0 != 0
                        && ::core::mem::size_of::<libc::c_char>() as libc::c_ulong
                            == 0 as libc::c_int as libc::c_ulong
                        || 0 != 0
                            && (*thisline).length as size_t
                                == 0 as libc::c_int as libc::c_ulong
                    {} else {
                        fwrite_unlocked(
                            (*thisline).buffer as *const libc::c_void,
                            ::core::mem::size_of::<libc::c_char>() as libc::c_ulong,
                            (*thisline).length as size_t,
                            stdout,
                        );
                    };
                };
                let mut _tmp: *mut linebuffer = 0 as *mut linebuffer;
                _tmp = prevline;
                prevline = thisline;
                thisline = _tmp;
                prevfield = thisfield;
                prevlen = thislen;
                first_group_printed = 1 as libc::c_int != 0;
            }
        }
        if (grouping as libc::c_uint == GM_BOTH as libc::c_int as libc::c_uint
            || grouping as libc::c_uint == GM_APPEND as libc::c_int as libc::c_uint)
            && first_group_printed as libc::c_int != 0
        {
            putchar_unlocked(delimiter as libc::c_int);
        }
    } else {
        let mut prevfield_0: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut prevlen_0: size_t = 0;
        let mut match_count: uintmax_t = 0 as libc::c_int as uintmax_t;
        let mut first_delimiter: bool = 1 as libc::c_int != 0;
        if !(readlinebuffer_delim(prevline, stdin, delimiter)).is_null() {
            prevfield_0 = find_field(prevline);
            prevlen_0 = ((*prevline).length - 1 as libc::c_int as libc::c_long
                - prevfield_0.offset_from((*prevline).buffer) as libc::c_long) as size_t;
            loop {
                if !(feof_unlocked(stdin) == 0) {
                    current_block = 17836213544692497527;
                    break;
                }
                let mut match_0: bool = false;
                let mut thisfield_0: *mut libc::c_char = 0 as *mut libc::c_char;
                let mut thislen_0: size_t = 0;
                if (readlinebuffer_delim(thisline, stdin, delimiter)).is_null() {
                    if ferror_unlocked(stdin) != 0 {
                        current_block = 10482074384504356869;
                        break;
                    } else {
                        current_block = 17836213544692497527;
                        break;
                    }
                } else {
                    thisfield_0 = find_field(thisline);
                    thislen_0 = ((*thisline).length - 1 as libc::c_int as libc::c_long
                        - thisfield_0.offset_from((*thisline).buffer) as libc::c_long)
                        as size_t;
                    match_0 = !different(thisfield_0, prevfield_0, thislen_0, prevlen_0);
                    match_count = (match_count as libc::c_ulong)
                        .wrapping_add(match_0 as libc::c_ulong) as uintmax_t
                        as uintmax_t;
                    if match_count == 18446744073709551615 as libc::c_ulong {
                        if count_occurrences as libc::c_int != 0 {
                            if ::core::mem::size_of::<C2RustUnnamed_4>() as libc::c_ulong
                                != 0
                            {
                                error(
                                    1 as libc::c_int,
                                    0 as libc::c_int,
                                    dcgettext(
                                        0 as *const libc::c_char,
                                        b"too many repeated lines\0" as *const u8
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
                                        b"too many repeated lines\0" as *const u8
                                            as *const libc::c_char,
                                        5 as libc::c_int,
                                    ),
                                );
                                if 0 as libc::c_int != 0 {} else {
                                    unreachable!();
                                };
                            };
                        }
                        match_count = match_count.wrapping_sub(1);
                    }
                    if delimit_groups as libc::c_uint
                        != DM_NONE as libc::c_int as libc::c_uint
                    {
                        if !match_0 {
                            if match_count != 0 {
                                first_delimiter = 0 as libc::c_int != 0;
                            }
                        } else if match_count == 1 as libc::c_int as libc::c_ulong {
                            if delimit_groups as libc::c_uint
                                == DM_PREPEND as libc::c_int as libc::c_uint
                                || delimit_groups as libc::c_uint
                                    == DM_SEPARATE as libc::c_int as libc::c_uint
                                    && !first_delimiter
                            {
                                putchar_unlocked(delimiter as libc::c_int);
                            }
                        }
                    }
                    if !match_0 || output_later_repeated as libc::c_int != 0 {
                        writeline(prevline, match_0, match_count);
                        let mut _tmp_0: *mut linebuffer = 0 as *mut linebuffer;
                        _tmp_0 = prevline;
                        prevline = thisline;
                        thisline = _tmp_0;
                        prevfield_0 = thisfield_0;
                        prevlen_0 = thislen_0;
                        if !match_0 {
                            match_count = 0 as libc::c_int as uintmax_t;
                        }
                    }
                }
            }
            match current_block {
                10482074384504356869 => {}
                _ => {
                    writeline(prevline, 0 as libc::c_int != 0, match_count);
                }
            }
        }
    }
    if ferror_unlocked(stdin) != 0 || rpl_fclose(stdin) != 0 as libc::c_int {
        if ::core::mem::size_of::<C2RustUnnamed_3>() as libc::c_ulong != 0 {
            error(
                1 as libc::c_int,
                0 as libc::c_int,
                dcgettext(
                    0 as *const libc::c_char,
                    b"error reading %s\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
                quotearg_style(shell_escape_always_quoting_style, infile),
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
                    b"error reading %s\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
                quotearg_style(shell_escape_always_quoting_style, infile),
            );
            if 0 as libc::c_int != 0 {} else {
                unreachable!();
            };
        };
    }
    free(lb1.buffer as *mut libc::c_void);
    free(lb2.buffer as *mut libc::c_void);
}
unsafe fn main_0(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut optc: libc::c_int = 0 as libc::c_int;
    let mut posixly_correct: bool = !(getenv(
        b"POSIXLY_CORRECT\0" as *const u8 as *const libc::c_char,
    ))
        .is_null();
    let mut skip_field_option_type: Skip_field_option_type = SFO_NONE;
    let mut nfiles: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    let mut file: [*const libc::c_char; 2] = [0 as *const libc::c_char; 2];
    let mut delimiter: libc::c_char = '\n' as i32 as libc::c_char;
    let mut output_option_used: bool = 0 as libc::c_int != 0;
    file[1 as libc::c_int as usize] = b"-\0" as *const u8 as *const libc::c_char;
    file[0 as libc::c_int as usize] = file[1 as libc::c_int as usize];
    set_program_name(*argv.offset(0 as libc::c_int as isize));
    setlocale(6 as libc::c_int, b"\0" as *const u8 as *const libc::c_char);
    bindtextdomain(
        b"coreutils\0" as *const u8 as *const libc::c_char,
        b"/usr/local/share/locale\0" as *const u8 as *const libc::c_char,
    );
    textdomain(b"coreutils\0" as *const u8 as *const libc::c_char);
    atexit(Some(close_stdout as unsafe extern "C" fn() -> ()));
    skip_chars = 0 as libc::c_int as size_t;
    skip_fields = 0 as libc::c_int as size_t;
    check_chars = 18446744073709551615 as libc::c_ulong;
    output_first_repeated = 1 as libc::c_int != 0;
    output_unique = output_first_repeated;
    output_later_repeated = 0 as libc::c_int != 0;
    countmode = count_none;
    delimit_groups = DM_NONE;
    loop {
        if optc == -(1 as libc::c_int)
            || posixly_correct as libc::c_int != 0
                && nfiles != 0 as libc::c_int as libc::c_uint
            || {
                optc = getopt_long(
                    argc,
                    argv,
                    b"-0123456789Dcdf:is:uw:z\0" as *const u8 as *const libc::c_char,
                    longopts.as_ptr(),
                    0 as *mut libc::c_int,
                );
                optc == -(1 as libc::c_int)
            }
        {
            if argc <= optind {
                break;
            }
            if nfiles == 2 as libc::c_int as libc::c_uint {
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
            let fresh4 = optind;
            optind = optind + 1;
            let fresh5 = nfiles;
            nfiles = nfiles.wrapping_add(1);
            file[fresh5 as usize] = *argv.offset(fresh4 as isize);
        } else {
            match optc {
                1 => {
                    let mut size: uintmax_t = 0;
                    if *optarg.offset(0 as libc::c_int as isize) as libc::c_int
                        == '+' as i32 && !strict_posix2()
                        && xstrtoumax(
                            optarg,
                            0 as *mut *mut libc::c_char,
                            10 as libc::c_int,
                            &mut size,
                            b"\0" as *const u8 as *const libc::c_char,
                        ) as libc::c_uint == LONGINT_OK as libc::c_int as libc::c_uint
                        && size <= 18446744073709551615 as libc::c_ulong
                    {
                        skip_chars = size;
                    } else if nfiles == 2 as libc::c_int as libc::c_uint {
                        error(
                            0 as libc::c_int,
                            0 as libc::c_int,
                            dcgettext(
                                0 as *const libc::c_char,
                                b"extra operand %s\0" as *const u8 as *const libc::c_char,
                                5 as libc::c_int,
                            ),
                            quote(optarg),
                        );
                        usage(1 as libc::c_int);
                    } else {
                        let fresh6 = nfiles;
                        nfiles = nfiles.wrapping_add(1);
                        file[fresh6 as usize] = optarg;
                    }
                }
                48 | 49 | 50 | 51 | 52 | 53 | 54 | 55 | 56 | 57 => {
                    if skip_field_option_type as libc::c_uint
                        == SFO_NEW as libc::c_int as libc::c_uint
                    {
                        skip_fields = 0 as libc::c_int as size_t;
                    }
                    if (if ::core::mem::size_of::<C2RustUnnamed_7>() as libc::c_ulong
                        != 0
                    {
                        (if (-(1 as libc::c_int) as size_t)
                            .wrapping_div(10 as libc::c_int as libc::c_ulong)
                            < skip_fields
                            || skip_fields
                                .wrapping_mul(10 as libc::c_int as libc::c_ulong)
                                .wrapping_add((optc - '0' as i32) as libc::c_ulong)
                                < skip_fields
                        {
                            0 as libc::c_int
                        } else {
                            skip_fields = skip_fields
                                .wrapping_mul(10 as libc::c_int as libc::c_ulong)
                                .wrapping_add((optc - '0' as i32) as libc::c_ulong);
                            1 as libc::c_int
                        })
                    } else {
                        (if (-(1 as libc::c_int) as size_t)
                            .wrapping_div(10 as libc::c_int as libc::c_ulong)
                            < skip_fields
                            || skip_fields
                                .wrapping_mul(10 as libc::c_int as libc::c_ulong)
                                .wrapping_add((optc - '0' as i32) as libc::c_ulong)
                                < skip_fields
                        {
                            0 as libc::c_int
                        } else {
                            skip_fields = skip_fields
                                .wrapping_mul(10 as libc::c_int as libc::c_ulong)
                                .wrapping_add((optc - '0' as i32) as libc::c_ulong);
                            1 as libc::c_int
                        })
                    }) == 0
                    {
                        skip_fields = 18446744073709551615 as libc::c_ulong;
                    }
                    skip_field_option_type = SFO_OBSOLETE;
                }
                99 => {
                    countmode = count_occurrences;
                    output_option_used = 1 as libc::c_int != 0;
                }
                100 => {
                    output_unique = 0 as libc::c_int != 0;
                    output_option_used = 1 as libc::c_int != 0;
                }
                68 => {
                    output_unique = 0 as libc::c_int != 0;
                    output_later_repeated = 1 as libc::c_int != 0;
                    if optarg.is_null() {
                        delimit_groups = DM_NONE;
                    } else {
                        delimit_groups = delimit_method_map[__xargmatch_internal(
                            b"--all-repeated\0" as *const u8 as *const libc::c_char,
                            optarg,
                            delimit_method_string.as_ptr(),
                            delimit_method_map.as_ptr() as *const libc::c_void,
                            ::core::mem::size_of::<delimit_method>() as libc::c_ulong,
                            argmatch_die,
                            1 as libc::c_int != 0,
                        ) as usize];
                    }
                    output_option_used = 1 as libc::c_int != 0;
                }
                256 => {
                    if optarg.is_null() {
                        grouping = GM_SEPARATE;
                    } else {
                        grouping = grouping_method_map[__xargmatch_internal(
                            b"--group\0" as *const u8 as *const libc::c_char,
                            optarg,
                            grouping_method_string.as_ptr(),
                            grouping_method_map.as_ptr() as *const libc::c_void,
                            ::core::mem::size_of::<grouping_method>() as libc::c_ulong,
                            argmatch_die,
                            1 as libc::c_int != 0,
                        ) as usize];
                    }
                }
                102 => {
                    skip_field_option_type = SFO_NEW;
                    skip_fields = size_opt(
                        optarg,
                        b"invalid number of fields to skip\0" as *const u8
                            as *const libc::c_char,
                    );
                }
                105 => {
                    ignore_case = 1 as libc::c_int != 0;
                }
                115 => {
                    skip_chars = size_opt(
                        optarg,
                        b"invalid number of bytes to skip\0" as *const u8
                            as *const libc::c_char,
                    );
                }
                117 => {
                    output_first_repeated = 0 as libc::c_int != 0;
                    output_option_used = 1 as libc::c_int != 0;
                }
                119 => {
                    check_chars = size_opt(
                        optarg,
                        b"invalid number of bytes to compare\0" as *const u8
                            as *const libc::c_char,
                    );
                }
                122 => {
                    delimiter = '\0' as i32 as libc::c_char;
                }
                -2 => {
                    usage(0 as libc::c_int);
                }
                -3 => {
                    version_etc(
                        stdout,
                        b"uniq\0" as *const u8 as *const libc::c_char,
                        b"GNU coreutils\0" as *const u8 as *const libc::c_char,
                        Version,
                        b"Richard M. Stallman\0" as *const u8 as *const libc::c_char,
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
    }
    if grouping as libc::c_uint != GM_NONE as libc::c_int as libc::c_uint
        && output_option_used as libc::c_int != 0
    {
        error(
            0 as libc::c_int,
            0 as libc::c_int,
            dcgettext(
                0 as *const libc::c_char,
                b"--group is mutually exclusive with -c/-d/-D/-u\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
        usage(1 as libc::c_int);
    }
    if grouping as libc::c_uint != GM_NONE as libc::c_int as libc::c_uint
        && countmode as libc::c_uint != count_none as libc::c_int as libc::c_uint
    {
        error(
            0 as libc::c_int,
            0 as libc::c_int,
            dcgettext(
                0 as *const libc::c_char,
                b"grouping and printing repeat counts is meaningless\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
        usage(1 as libc::c_int);
    }
    if countmode as libc::c_uint == count_occurrences as libc::c_int as libc::c_uint
        && output_later_repeated as libc::c_int != 0
    {
        error(
            0 as libc::c_int,
            0 as libc::c_int,
            dcgettext(
                0 as *const libc::c_char,
                b"printing all duplicated lines and repeat counts is meaningless\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
        usage(1 as libc::c_int);
    }
    check_file(
        file[0 as libc::c_int as usize],
        file[1 as libc::c_int as usize],
        delimiter,
    );
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
