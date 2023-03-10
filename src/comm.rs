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
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn __errno_location() -> *mut libc::c_int;
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
    fn umaxtostr(_: uintmax_t, _: *mut libc::c_char) -> *mut libc::c_char;
    fn quotearg_n_style_colon(
        n: libc::c_int,
        s: quoting_style,
        arg: *const libc::c_char,
    ) -> *mut libc::c_char;
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
    fn hard_locale(category: libc::c_int) -> bool;
    fn quote(arg: *const libc::c_char) -> *const libc::c_char;
    fn fopen_safer(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
    fn memcmp2(
        s1: *const libc::c_char,
        n1: size_t,
        s2: *const libc::c_char,
        n2: size_t,
    ) -> libc::c_int;
    fn xmemcoll(
        _: *mut libc::c_char,
        _: size_t,
        _: *mut libc::c_char,
        _: size_t,
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
pub type C2RustUnnamed_0 = libc::c_uint;
pub const CHECK_ORDER_DISABLED: C2RustUnnamed_0 = 2;
pub const CHECK_ORDER_ENABLED: C2RustUnnamed_0 = 1;
pub const CHECK_ORDER_DEFAULT: C2RustUnnamed_0 = 0;
pub type C2RustUnnamed_1 = libc::c_uint;
pub const TOTAL_OPTION: C2RustUnnamed_1 = 259;
pub const OUTPUT_DELIMITER_OPTION: C2RustUnnamed_1 = 258;
pub const NOCHECK_ORDER_OPTION: C2RustUnnamed_1 = 257;
pub const CHECK_ORDER_OPTION: C2RustUnnamed_1 = 256;
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
unsafe extern "C" fn ferror_unlocked(mut __stream: *mut FILE) -> libc::c_int {
    return ((*__stream)._flags & 0x20 as libc::c_int != 0 as libc::c_int) as libc::c_int;
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
static mut hard_LC_COLLATE: bool = false;
static mut only_file_1: bool = false;
static mut only_file_2: bool = false;
static mut both: bool = false;
static mut seen_unpairable: bool = false;
static mut issued_disorder_warning: [bool; 2] = [false; 2];
static mut delim: libc::c_uchar = '\n' as i32 as libc::c_uchar;
static mut total_option: bool = false;
static mut check_input_order: C2RustUnnamed_0 = CHECK_ORDER_DEFAULT;
static mut col_sep: *const libc::c_char = b"\t\0" as *const u8 as *const libc::c_char;
static mut col_sep_len: size_t = 0 as libc::c_int as size_t;
static mut long_options: [option; 8] = [
    {
        let mut init = option {
            name: b"check-order\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: CHECK_ORDER_OPTION as libc::c_int,
        };
        init
    },
    {
        let mut init = option {
            name: b"nocheck-order\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: NOCHECK_ORDER_OPTION as libc::c_int,
        };
        init
    },
    {
        let mut init = option {
            name: b"output-delimiter\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: OUTPUT_DELIMITER_OPTION as libc::c_int,
        };
        init
    },
    {
        let mut init = option {
            name: b"total\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: TOTAL_OPTION as libc::c_int,
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
                b"Usage: %s [OPTION]... FILE1 FILE2\n\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
            program_name,
        );
        fputs_unlocked(
            dcgettext(
                0 as *const libc::c_char,
                b"Compare sorted files FILE1 and FILE2 line by line.\n\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
            stdout,
        );
        fputs_unlocked(
            dcgettext(
                0 as *const libc::c_char,
                b"\nWhen FILE1 or FILE2 (not both) is -, read standard input.\n\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            stdout,
        );
        fputs_unlocked(
            dcgettext(
                0 as *const libc::c_char,
                b"\nWith no options, produce three-column output.  Column one contains\nlines unique to FILE1, column two contains lines unique to FILE2,\nand column three contains lines common to both files.\n\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            stdout,
        );
        fputs_unlocked(
            dcgettext(
                0 as *const libc::c_char,
                b"\n  -1                      suppress column 1 (lines unique to FILE1)\n  -2                      suppress column 2 (lines unique to FILE2)\n  -3                      suppress column 3 (lines that appear in both files)\n\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            stdout,
        );
        fputs_unlocked(
            dcgettext(
                0 as *const libc::c_char,
                b"\n      --check-order       check that the input is correctly sorted, even\n                            if all input lines are pairable\n      --nocheck-order     do not check that the input is correctly sorted\n\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            stdout,
        );
        fputs_unlocked(
            dcgettext(
                0 as *const libc::c_char,
                b"      --output-delimiter=STR  separate columns with STR\n\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            stdout,
        );
        fputs_unlocked(
            dcgettext(
                0 as *const libc::c_char,
                b"      --total             output a summary\n\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
            stdout,
        );
        fputs_unlocked(
            dcgettext(
                0 as *const libc::c_char,
                b"  -z, --zero-terminated   line delimiter is NUL, not newline\n\0"
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
                b"\nNote, comparisons honor the rules specified by 'LC_COLLATE'.\n\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            stdout,
        );
        printf(
            dcgettext(
                0 as *const libc::c_char,
                b"\nExamples:\n  %s -12 file1 file2  Print only lines present in both file1 and file2.\n  %s -3 file1 file2  Print lines in file1 not in file2, and vice versa.\n\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            program_name,
            program_name,
        );
        emit_ancillary_info(b"comm\0" as *const u8 as *const libc::c_char);
    }
    exit(status);
}
unsafe extern "C" fn writeline(
    mut line: *const linebuffer,
    mut stream: *mut FILE,
    mut class: libc::c_int,
) {
    match class {
        1 => {
            if !only_file_1 {
                return;
            }
        }
        2 => {
            if !only_file_2 {
                return;
            }
            if only_file_1 {
                if 0 != 0 && 0 != 0
                    && (1 as libc::c_int as size_t).wrapping_mul(col_sep_len)
                        <= 8 as libc::c_int as libc::c_ulong
                    && 1 as libc::c_int as size_t != 0 as libc::c_int as libc::c_ulong
                {
                    ({
                        let mut __ptr: *const libc::c_char = col_sep;
                        let mut __stream: *mut FILE = stream;
                        let mut __cnt: size_t = 0;
                        __cnt = (1 as libc::c_int as size_t).wrapping_mul(col_sep_len);
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
                    if 0 != 0
                        && 1 as libc::c_int as size_t
                            == 0 as libc::c_int as libc::c_ulong
                        || 0 != 0 && col_sep_len == 0 as libc::c_int as libc::c_ulong
                    {} else {
                        fwrite_unlocked(
                            col_sep as *const libc::c_void,
                            1 as libc::c_int as size_t,
                            col_sep_len,
                            stream,
                        );
                    };
                };
            }
        }
        3 => {
            if !both {
                return;
            }
            if only_file_1 {
                if 0 != 0 && 0 != 0
                    && (1 as libc::c_int as size_t).wrapping_mul(col_sep_len)
                        <= 8 as libc::c_int as libc::c_ulong
                    && 1 as libc::c_int as size_t != 0 as libc::c_int as libc::c_ulong
                {
                    ({
                        let mut __ptr: *const libc::c_char = col_sep;
                        let mut __stream: *mut FILE = stream;
                        let mut __cnt: size_t = 0;
                        __cnt = (1 as libc::c_int as size_t).wrapping_mul(col_sep_len);
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
                    if 0 != 0
                        && 1 as libc::c_int as size_t
                            == 0 as libc::c_int as libc::c_ulong
                        || 0 != 0 && col_sep_len == 0 as libc::c_int as libc::c_ulong
                    {} else {
                        fwrite_unlocked(
                            col_sep as *const libc::c_void,
                            1 as libc::c_int as size_t,
                            col_sep_len,
                            stream,
                        );
                    };
                };
            }
            if only_file_2 {
                if 0 != 0 && 0 != 0
                    && (1 as libc::c_int as size_t).wrapping_mul(col_sep_len)
                        <= 8 as libc::c_int as libc::c_ulong
                    && 1 as libc::c_int as size_t != 0 as libc::c_int as libc::c_ulong
                {
                    ({
                        let mut __ptr: *const libc::c_char = col_sep;
                        let mut __stream: *mut FILE = stream;
                        let mut __cnt: size_t = 0;
                        __cnt = (1 as libc::c_int as size_t).wrapping_mul(col_sep_len);
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
                        && 1 as libc::c_int as size_t
                            == 0 as libc::c_int as libc::c_ulong
                        || 0 != 0 && col_sep_len == 0 as libc::c_int as libc::c_ulong
                    {} else {
                        fwrite_unlocked(
                            col_sep as *const libc::c_void,
                            1 as libc::c_int as size_t,
                            col_sep_len,
                            stream,
                        );
                    };
                };
            }
        }
        _ => {}
    }
    if 0 != 0 && 0 != 0
        && (::core::mem::size_of::<libc::c_char>() as libc::c_ulong)
            .wrapping_mul((*line).length as size_t) <= 8 as libc::c_int as libc::c_ulong
        && ::core::mem::size_of::<libc::c_char>() as libc::c_ulong
            != 0 as libc::c_int as libc::c_ulong
    {
        ({
            let mut __ptr: *const libc::c_char = (*line).buffer as *const libc::c_char;
            let mut __stream: *mut FILE = stream;
            let mut __cnt: size_t = 0;
            __cnt = (::core::mem::size_of::<libc::c_char>() as libc::c_ulong)
                .wrapping_mul((*line).length as size_t);
            while __cnt > 0 as libc::c_int as libc::c_ulong {
                let fresh4 = __ptr;
                __ptr = __ptr.offset(1);
                if putc_unlocked(*fresh4 as libc::c_int, __stream) == -(1 as libc::c_int)
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
                stream,
            );
        };
    };
}
unsafe extern "C" fn check_order(
    mut prev: *const linebuffer,
    mut current: *const linebuffer,
    mut whatfile: libc::c_int,
) {
    if check_input_order as libc::c_uint
        != CHECK_ORDER_DISABLED as libc::c_int as libc::c_uint
        && (check_input_order as libc::c_uint
            == CHECK_ORDER_ENABLED as libc::c_int as libc::c_uint
            || seen_unpairable as libc::c_int != 0)
    {
        if !issued_disorder_warning[(whatfile - 1 as libc::c_int) as usize] {
            let mut order: libc::c_int = 0;
            if hard_LC_COLLATE {
                order = xmemcoll(
                    (*prev).buffer,
                    ((*prev).length - 1 as libc::c_int as libc::c_long) as size_t,
                    (*current).buffer,
                    ((*current).length - 1 as libc::c_int as libc::c_long) as size_t,
                );
            } else {
                order = memcmp2(
                    (*prev).buffer,
                    ((*prev).length - 1 as libc::c_int as libc::c_long) as size_t,
                    (*current).buffer,
                    ((*current).length - 1 as libc::c_int as libc::c_long) as size_t,
                );
            }
            if (0 as libc::c_int) < order {
                error(
                    if check_input_order as libc::c_uint
                        == CHECK_ORDER_ENABLED as libc::c_int as libc::c_uint
                    {
                        1 as libc::c_int
                    } else {
                        0 as libc::c_int
                    },
                    0 as libc::c_int,
                    dcgettext(
                        0 as *const libc::c_char,
                        b"file %d is not in sorted order\0" as *const u8
                            as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    whatfile,
                );
                issued_disorder_warning[(whatfile - 1 as libc::c_int)
                    as usize] = 1 as libc::c_int != 0;
            }
        }
    }
}
unsafe extern "C" fn compare_files(mut infiles: *mut *mut libc::c_char) {
    let mut lba: [[linebuffer; 4]; 2] = [[linebuffer {
        size: 0,
        length: 0,
        buffer: 0 as *mut libc::c_char,
    }; 4]; 2];
    let mut thisline: [*mut linebuffer; 2] = [0 as *mut linebuffer; 2];
    let mut all_line: [[*mut linebuffer; 4]; 2] = [[0 as *mut linebuffer; 4]; 2];
    let mut alt: [[libc::c_int; 3]; 2] = [[0; 3]; 2];
    let mut streams: [*mut FILE; 2] = [0 as *mut FILE; 2];
    let mut total: [uintmax_t; 3] = [
        0 as libc::c_int as uintmax_t,
        0 as libc::c_int as uintmax_t,
        0 as libc::c_int as uintmax_t,
    ];
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < 2 as libc::c_int {
        j = 0 as libc::c_int;
        while j < 4 as libc::c_int {
            initbuffer(
                &mut *(*lba.as_mut_ptr().offset(i as isize))
                    .as_mut_ptr()
                    .offset(j as isize),
            );
            all_line[i
                as usize][j
                as usize] = &mut *(*lba.as_mut_ptr().offset(i as isize))
                .as_mut_ptr()
                .offset(j as isize) as *mut linebuffer;
            j += 1;
        }
        alt[i as usize][0 as libc::c_int as usize] = 0 as libc::c_int;
        alt[i as usize][1 as libc::c_int as usize] = 0 as libc::c_int;
        alt[i as usize][2 as libc::c_int as usize] = 0 as libc::c_int;
        streams[i
            as usize] = if strcmp(
            *infiles.offset(i as isize),
            b"-\0" as *const u8 as *const libc::c_char,
        ) == 0 as libc::c_int
        {
            stdin
        } else {
            fopen_safer(
                *infiles.offset(i as isize),
                b"r\0" as *const u8 as *const libc::c_char,
            )
        };
        if (streams[i as usize]).is_null() {
            if ::core::mem::size_of::<C2RustUnnamed_6>() as libc::c_ulong != 0 {
                error(
                    1 as libc::c_int,
                    *__errno_location(),
                    b"%s\0" as *const u8 as *const libc::c_char,
                    quotearg_n_style_colon(
                        0 as libc::c_int,
                        shell_escape_quoting_style,
                        *infiles.offset(i as isize),
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
                        *infiles.offset(i as isize),
                    ),
                );
                if 0 as libc::c_int != 0 {} else {
                    unreachable!();
                };
            };
        }
        fadvise(streams[i as usize], FADVISE_SEQUENTIAL);
        thisline[i
            as usize] = readlinebuffer_delim(
            all_line[i as usize][alt[i as usize][0 as libc::c_int as usize] as usize],
            streams[i as usize],
            delim as libc::c_char,
        );
        if ferror_unlocked(streams[i as usize]) != 0 {
            if ::core::mem::size_of::<C2RustUnnamed_5>() as libc::c_ulong != 0 {
                error(
                    1 as libc::c_int,
                    *__errno_location(),
                    b"%s\0" as *const u8 as *const libc::c_char,
                    quotearg_n_style_colon(
                        0 as libc::c_int,
                        shell_escape_quoting_style,
                        *infiles.offset(i as isize),
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
                        *infiles.offset(i as isize),
                    ),
                );
                if 0 as libc::c_int != 0 {} else {
                    unreachable!();
                };
            };
        }
        i += 1;
    }
    while !(thisline[0 as libc::c_int as usize]).is_null()
        || !(thisline[1 as libc::c_int as usize]).is_null()
    {
        let mut order: libc::c_int = 0;
        let mut fill_up: [bool; 2] = [0 as libc::c_int != 0, 0 as libc::c_int != 0];
        if (thisline[0 as libc::c_int as usize]).is_null() {
            order = 1 as libc::c_int;
        } else if (thisline[1 as libc::c_int as usize]).is_null() {
            order = -(1 as libc::c_int);
        } else if hard_LC_COLLATE {
            order = xmemcoll(
                (*thisline[0 as libc::c_int as usize]).buffer,
                ((*thisline[0 as libc::c_int as usize]).length
                    - 1 as libc::c_int as libc::c_long) as size_t,
                (*thisline[1 as libc::c_int as usize]).buffer,
                ((*thisline[1 as libc::c_int as usize]).length
                    - 1 as libc::c_int as libc::c_long) as size_t,
            );
        } else {
            let mut len: size_t = ((if (*thisline[0 as libc::c_int as usize]).length
                < (*thisline[1 as libc::c_int as usize]).length
            {
                (*thisline[0 as libc::c_int as usize]).length
            } else {
                (*thisline[1 as libc::c_int as usize]).length
            }) - 1 as libc::c_int as libc::c_long) as size_t;
            order = memcmp(
                (*thisline[0 as libc::c_int as usize]).buffer as *const libc::c_void,
                (*thisline[1 as libc::c_int as usize]).buffer as *const libc::c_void,
                len,
            );
            if order == 0 as libc::c_int {
                order = ((*thisline[0 as libc::c_int as usize]).length
                    > (*thisline[1 as libc::c_int as usize]).length) as libc::c_int
                    - ((*thisline[0 as libc::c_int as usize]).length
                        < (*thisline[1 as libc::c_int as usize]).length) as libc::c_int;
            }
        }
        if order == 0 as libc::c_int {
            total[2 as libc::c_int
                as usize] = (total[2 as libc::c_int as usize]).wrapping_add(1);
            writeline(thisline[1 as libc::c_int as usize], stdout, 3 as libc::c_int);
        } else {
            seen_unpairable = 1 as libc::c_int != 0;
            if order <= 0 as libc::c_int {
                total[0 as libc::c_int
                    as usize] = (total[0 as libc::c_int as usize]).wrapping_add(1);
                writeline(thisline[0 as libc::c_int as usize], stdout, 1 as libc::c_int);
            } else {
                total[1 as libc::c_int
                    as usize] = (total[1 as libc::c_int as usize]).wrapping_add(1);
                writeline(thisline[1 as libc::c_int as usize], stdout, 2 as libc::c_int);
            }
        }
        if 0 as libc::c_int <= order {
            fill_up[1 as libc::c_int as usize] = 1 as libc::c_int != 0;
        }
        if order <= 0 as libc::c_int {
            fill_up[0 as libc::c_int as usize] = 1 as libc::c_int != 0;
        }
        i = 0 as libc::c_int;
        while i < 2 as libc::c_int {
            if fill_up[i as usize] {
                alt[i
                    as usize][2 as libc::c_int
                    as usize] = alt[i as usize][1 as libc::c_int as usize];
                alt[i
                    as usize][1 as libc::c_int
                    as usize] = alt[i as usize][0 as libc::c_int as usize];
                alt[i
                    as usize][0 as libc::c_int
                    as usize] = alt[i as usize][0 as libc::c_int as usize]
                    + 1 as libc::c_int & 0x3 as libc::c_int;
                thisline[i
                    as usize] = readlinebuffer_delim(
                    all_line[i
                        as usize][alt[i as usize][0 as libc::c_int as usize] as usize],
                    streams[i as usize],
                    delim as libc::c_char,
                );
                if !(thisline[i as usize]).is_null() {
                    check_order(
                        all_line[i
                            as usize][alt[i as usize][1 as libc::c_int as usize]
                            as usize],
                        thisline[i as usize],
                        i + 1 as libc::c_int,
                    );
                } else if !((*all_line[i
                    as usize][alt[i as usize][2 as libc::c_int as usize] as usize])
                    .buffer)
                    .is_null()
                {
                    check_order(
                        all_line[i
                            as usize][alt[i as usize][2 as libc::c_int as usize]
                            as usize],
                        all_line[i
                            as usize][alt[i as usize][1 as libc::c_int as usize]
                            as usize],
                        i + 1 as libc::c_int,
                    );
                }
                if ferror_unlocked(streams[i as usize]) != 0 {
                    if ::core::mem::size_of::<C2RustUnnamed_4>() as libc::c_ulong != 0 {
                        error(
                            1 as libc::c_int,
                            *__errno_location(),
                            b"%s\0" as *const u8 as *const libc::c_char,
                            quotearg_n_style_colon(
                                0 as libc::c_int,
                                shell_escape_quoting_style,
                                *infiles.offset(i as isize),
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
                                *infiles.offset(i as isize),
                            ),
                        );
                        if 0 as libc::c_int != 0 {} else {
                            unreachable!();
                        };
                    };
                }
                fill_up[i as usize] = 0 as libc::c_int != 0;
            }
            i += 1;
        }
    }
    i = 0 as libc::c_int;
    while i < 2 as libc::c_int {
        if rpl_fclose(streams[i as usize]) != 0 as libc::c_int {
            if ::core::mem::size_of::<C2RustUnnamed_3>() as libc::c_ulong != 0 {
                error(
                    1 as libc::c_int,
                    *__errno_location(),
                    b"%s\0" as *const u8 as *const libc::c_char,
                    quotearg_n_style_colon(
                        0 as libc::c_int,
                        shell_escape_quoting_style,
                        *infiles.offset(i as isize),
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
                        *infiles.offset(i as isize),
                    ),
                );
                if 0 as libc::c_int != 0 {} else {
                    unreachable!();
                };
            };
        }
        i += 1;
    }
    if total_option {
        let mut buf1: [libc::c_char; 21] = [0; 21];
        let mut buf2: [libc::c_char; 21] = [0; 21];
        let mut buf3: [libc::c_char; 21] = [0; 21];
        if col_sep_len == 1 as libc::c_int as libc::c_ulong {
            printf(
                b"%s%c%s%c%s%c%s%c\0" as *const u8 as *const libc::c_char,
                umaxtostr(total[0 as libc::c_int as usize], buf1.as_mut_ptr()),
                *col_sep as libc::c_int,
                umaxtostr(total[1 as libc::c_int as usize], buf2.as_mut_ptr()),
                *col_sep as libc::c_int,
                umaxtostr(total[2 as libc::c_int as usize], buf3.as_mut_ptr()),
                *col_sep as libc::c_int,
                dcgettext(
                    0 as *const libc::c_char,
                    b"total\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
                delim as libc::c_int,
            );
        } else {
            printf(
                b"%s%s%s%s%s%s%s%c\0" as *const u8 as *const libc::c_char,
                umaxtostr(total[0 as libc::c_int as usize], buf1.as_mut_ptr()),
                col_sep,
                umaxtostr(total[1 as libc::c_int as usize], buf2.as_mut_ptr()),
                col_sep,
                umaxtostr(total[2 as libc::c_int as usize], buf3.as_mut_ptr()),
                col_sep,
                dcgettext(
                    0 as *const libc::c_char,
                    b"total\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
                delim as libc::c_int,
            );
        }
    }
    if issued_disorder_warning[0 as libc::c_int as usize] as libc::c_int != 0
        || issued_disorder_warning[1 as libc::c_int as usize] as libc::c_int != 0
    {
        if ::core::mem::size_of::<C2RustUnnamed_2>() as libc::c_ulong != 0 {
            error(
                1 as libc::c_int,
                0 as libc::c_int,
                dcgettext(
                    0 as *const libc::c_char,
                    b"input is not in sorted order\0" as *const u8
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
                    b"input is not in sorted order\0" as *const u8
                        as *const libc::c_char,
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
unsafe fn main_0(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut c: libc::c_int = 0;
    set_program_name(*argv.offset(0 as libc::c_int as isize));
    setlocale(6 as libc::c_int, b"\0" as *const u8 as *const libc::c_char);
    bindtextdomain(
        b"coreutils\0" as *const u8 as *const libc::c_char,
        b"/usr/local/share/locale\0" as *const u8 as *const libc::c_char,
    );
    textdomain(b"coreutils\0" as *const u8 as *const libc::c_char);
    hard_LC_COLLATE = hard_locale(3 as libc::c_int);
    atexit(Some(close_stdout as unsafe extern "C" fn() -> ()));
    only_file_1 = 1 as libc::c_int != 0;
    only_file_2 = 1 as libc::c_int != 0;
    both = 1 as libc::c_int != 0;
    seen_unpairable = 0 as libc::c_int != 0;
    issued_disorder_warning[1 as libc::c_int as usize] = 0 as libc::c_int != 0;
    issued_disorder_warning[0 as libc::c_int
        as usize] = issued_disorder_warning[1 as libc::c_int as usize];
    check_input_order = CHECK_ORDER_DEFAULT;
    total_option = 0 as libc::c_int != 0;
    loop {
        c = getopt_long(
            argc,
            argv,
            b"123z\0" as *const u8 as *const libc::c_char,
            long_options.as_ptr(),
            0 as *mut libc::c_int,
        );
        if !(c != -(1 as libc::c_int)) {
            break;
        }
        match c {
            49 => {
                only_file_1 = 0 as libc::c_int != 0;
            }
            50 => {
                only_file_2 = 0 as libc::c_int != 0;
            }
            51 => {
                both = 0 as libc::c_int != 0;
            }
            122 => {
                delim = '\0' as i32 as libc::c_uchar;
            }
            257 => {
                check_input_order = CHECK_ORDER_DISABLED;
            }
            256 => {
                check_input_order = CHECK_ORDER_ENABLED;
            }
            258 => {
                if col_sep_len != 0 && !(strcmp(col_sep, optarg) == 0 as libc::c_int) {
                    if ::core::mem::size_of::<C2RustUnnamed_7>() as libc::c_ulong != 0 {
                        error(
                            1 as libc::c_int,
                            0 as libc::c_int,
                            dcgettext(
                                0 as *const libc::c_char,
                                b"multiple output delimiters specified\0" as *const u8
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
                                b"multiple output delimiters specified\0" as *const u8
                                    as *const libc::c_char,
                                5 as libc::c_int,
                            ),
                        );
                        if 0 as libc::c_int != 0 {} else {
                            unreachable!();
                        };
                    };
                }
                col_sep = optarg;
                col_sep_len = if *optarg as libc::c_int != 0 {
                    strlen(optarg)
                } else {
                    1 as libc::c_int as libc::c_ulong
                };
            }
            259 => {
                total_option = 1 as libc::c_int != 0;
            }
            -2 => {
                usage(0 as libc::c_int);
            }
            -3 => {
                version_etc(
                    stdout,
                    b"comm\0" as *const u8 as *const libc::c_char,
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
    if col_sep_len == 0 {
        col_sep_len = 1 as libc::c_int as size_t;
    }
    if argc - optind < 2 as libc::c_int {
        if argc <= optind {
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
        }
        usage(1 as libc::c_int);
    }
    if (2 as libc::c_int) < argc - optind {
        error(
            0 as libc::c_int,
            0 as libc::c_int,
            dcgettext(
                0 as *const libc::c_char,
                b"extra operand %s\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            quote(*argv.offset((optind + 2 as libc::c_int) as isize)),
        );
        usage(1 as libc::c_int);
    }
    compare_files(argv.offset(optind as isize));
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
