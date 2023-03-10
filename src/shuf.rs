#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![feature(extern_types)]
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type randint_source;
    fn atexit(__func: Option::<unsafe extern "C" fn() -> ()>) -> libc::c_int;
    fn exit(_: libc::c_int) -> !;
    fn lseek(__fd: libc::c_int, __offset: __off_t, __whence: libc::c_int) -> __off_t;
    static mut optarg: *mut libc::c_char;
    static mut optind: libc::c_int;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn rawmemchr(__s: *const libc::c_void, __c: libc::c_int) -> *mut libc::c_void;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strncmp(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn stpcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn __errno_location() -> *mut libc::c_int;
    fn fstat(__fd: libc::c_int, __buf: *mut stat) -> libc::c_int;
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
    fn xcalloc(n: size_t, s: size_t) -> *mut libc::c_void;
    fn xreallocarray(p: *mut libc::c_void, n: size_t, s: size_t) -> *mut libc::c_void;
    fn xnmalloc(n: size_t, s: size_t) -> *mut libc::c_void;
    static mut stdin: *mut FILE;
    static mut stdout: *mut FILE;
    static mut stderr: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
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
    fn quotearg_n_style_colon(
        n: libc::c_int,
        s: quoting_style,
        arg: *const libc::c_char,
    ) -> *mut libc::c_char;
    fn __overflow(_: *mut FILE, _: libc::c_int) -> libc::c_int;
    fn error(
        __status: libc::c_int,
        __errnum: libc::c_int,
        __format: *const libc::c_char,
        _: ...
    );
    fn fadvise(fp: *mut FILE, advice: fadvice_t);
    fn getopt_long(
        ___argc: libc::c_int,
        ___argv: *const *mut libc::c_char,
        __shortopts: *const libc::c_char,
        __longopts: *const option,
        __longind: *mut libc::c_int,
    ) -> libc::c_int;
    fn initbuffer(linebuffer: *mut linebuffer);
    fn readlinebuffer_delim(
        linebuffer: *mut linebuffer,
        stream: *mut FILE,
        delimiter: libc::c_char,
    ) -> *mut linebuffer;
    fn freebuffer(_: *mut linebuffer);
    fn quote(arg: *const libc::c_char) -> *const libc::c_char;
    fn randint_all_new(_: *const libc::c_char, _: size_t) -> *mut randint_source;
    fn randint_genmax(_: *mut randint_source, genmax: randint) -> randint;
    fn randperm_bound(_: size_t, _: size_t) -> size_t;
    fn randperm_new(_: *mut randint_source, _: size_t, _: size_t) -> *mut size_t;
    fn fread_file(
        stream: *mut FILE,
        flags: libc::c_int,
        length: *mut size_t,
    ) -> *mut libc::c_char;
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
}
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
pub type off_t = __off_t;
pub type size_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
pub type ptrdiff_t = libc::c_long;
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
pub type fadvice_t = libc::c_uint;
pub const FADVISE_RANDOM: fadvice_t = 1;
pub const FADVISE_WILLNEED: fadvice_t = 3;
pub const FADVISE_DONTNEED: fadvice_t = 4;
pub const FADVISE_NOREUSE: fadvice_t = 5;
pub const FADVISE_SEQUENTIAL: fadvice_t = 2;
pub const FADVISE_NORMAL: fadvice_t = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct option {
    pub name: *const libc::c_char,
    pub has_arg: libc::c_int,
    pub flag: *mut libc::c_int,
    pub val: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct linebuffer {
    pub size: idx_t,
    pub length: idx_t,
    pub buffer: *mut libc::c_char,
}
pub type randint = uintmax_t;
pub type strtol_error = libc::c_uint;
pub const LONGINT_INVALID: strtol_error = 4;
pub const LONGINT_INVALID_SUFFIX_CHAR_WITH_OVERFLOW: strtol_error = 3;
pub const LONGINT_INVALID_SUFFIX_CHAR: strtol_error = 2;
pub const LONGINT_OVERFLOW: strtol_error = 1;
pub const LONGINT_OK: strtol_error = 0;
pub type C2RustUnnamed_0 = libc::c_uint;
pub const RESERVOIR_LINES_INCREMENT: C2RustUnnamed_0 = 1024;
pub type C2RustUnnamed_1 = libc::c_uint;
pub const RESERVOIR_MIN_INPUT: C2RustUnnamed_1 = 8388608;
pub type C2RustUnnamed_2 = libc::c_uint;
pub const RANDOM_SOURCE_OPTION: C2RustUnnamed_2 = 256;
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
unsafe extern "C" fn xnrealloc(
    mut p: *mut libc::c_void,
    mut n: size_t,
    mut s: size_t,
) -> *mut libc::c_void {
    return xreallocarray(p, n, s);
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
unsafe extern "C" fn usable_st_size(mut sb: *const stat) -> bool {
    return (*sb).st_mode & 0o170000 as libc::c_int as libc::c_uint
        == 0o100000 as libc::c_int as libc::c_uint
        || (*sb).st_mode & 0o170000 as libc::c_int as libc::c_uint
            == 0o120000 as libc::c_int as libc::c_uint
        || ((*sb).st_mode).wrapping_sub((*sb).st_mode) != 0 || 0 as libc::c_int != 0;
}
#[inline]
unsafe extern "C" fn randint_choose(
    mut s: *mut randint_source,
    mut choices: randint,
) -> randint {
    return randint_genmax(s, choices.wrapping_sub(1 as libc::c_int as libc::c_ulong));
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
                b"Usage: %s [OPTION]... [FILE]\n  or:  %s -e [OPTION]... [ARG]...\n  or:  %s -i LO-HI [OPTION]...\n\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            program_name,
            program_name,
            program_name,
        );
        fputs_unlocked(
            dcgettext(
                0 as *const libc::c_char,
                b"Write a random permutation of the input lines to standard output.\n\0"
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
                b"  -e, --echo                treat each ARG as an input line\n  -i, --input-range=LO-HI   treat each number LO through HI as an input line\n  -n, --head-count=COUNT    output at most COUNT lines\n  -o, --output=FILE         write result to FILE instead of standard output\n      --random-source=FILE  get random bytes from FILE\n  -r, --repeat              output lines can be repeated\n\0"
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
        emit_ancillary_info(b"shuf\0" as *const u8 as *const libc::c_char);
    }
    exit(status);
}
static mut long_opts: [option; 10] = [
    {
        let mut init = option {
            name: b"echo\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'e' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"input-range\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'i' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"head-count\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'n' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"output\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'o' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"random-source\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: RANDOM_SOURCE_OPTION as libc::c_int,
        };
        init
    },
    {
        let mut init = option {
            name: b"repeat\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'r' as i32,
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
unsafe extern "C" fn input_from_argv(
    mut operand: *mut *mut libc::c_char,
    mut n_operands: libc::c_int,
    mut eolbyte: libc::c_char,
) {
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut size: size_t = n_operands as size_t;
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < n_operands {
        size = (size as libc::c_ulong).wrapping_add(strlen(*operand.offset(i as isize)))
            as size_t as size_t;
        i += 1;
    }
    p = xmalloc(size) as *mut libc::c_char;
    i = 0 as libc::c_int;
    while i < n_operands {
        let mut p1: *mut libc::c_char = stpcpy(p, *operand.offset(i as isize));
        let ref mut fresh1 = *operand.offset(i as isize);
        *fresh1 = p;
        p = p1;
        let fresh2 = p;
        p = p.offset(1);
        *fresh2 = eolbyte;
        i += 1;
    }
    let ref mut fresh3 = *operand.offset(n_operands as isize);
    *fresh3 = p;
}
unsafe extern "C" fn next_line(
    mut line: *mut libc::c_char,
    mut eolbyte: libc::c_char,
) -> *mut libc::c_char {
    let mut p: *mut libc::c_char = rawmemchr(
        line as *const libc::c_void,
        eolbyte as libc::c_int,
    ) as *mut libc::c_char;
    return p.offset(1 as libc::c_int as isize);
}
unsafe extern "C" fn input_size() -> off_t {
    let mut file_size: off_t = 0;
    let mut stat_buf: stat = stat {
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
    if fstat(0 as libc::c_int, &mut stat_buf) != 0 as libc::c_int {
        return if (0 as libc::c_int as off_t) < -(1 as libc::c_int) as off_t {
            -(1 as libc::c_int) as off_t
        } else {
            (((1 as libc::c_int as off_t)
                << (::core::mem::size_of::<off_t>() as libc::c_ulong)
                    .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                    .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                - 1 as libc::c_int as libc::c_long) * 2 as libc::c_int as libc::c_long
                + 1 as libc::c_int as libc::c_long
        };
    }
    if usable_st_size(&mut stat_buf) {
        file_size = stat_buf.st_size;
    } else {
        return if (0 as libc::c_int as off_t) < -(1 as libc::c_int) as off_t {
            -(1 as libc::c_int) as off_t
        } else {
            (((1 as libc::c_int as off_t)
                << (::core::mem::size_of::<off_t>() as libc::c_ulong)
                    .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                    .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                - 1 as libc::c_int as libc::c_long) * 2 as libc::c_int as libc::c_long
                + 1 as libc::c_int as libc::c_long
        }
    }
    let mut input_offset: off_t = lseek(
        0 as libc::c_int,
        0 as libc::c_int as __off_t,
        1 as libc::c_int,
    );
    if input_offset < 0 as libc::c_int as libc::c_long {
        return if (0 as libc::c_int as off_t) < -(1 as libc::c_int) as off_t {
            -(1 as libc::c_int) as off_t
        } else {
            (((1 as libc::c_int as off_t)
                << (::core::mem::size_of::<off_t>() as libc::c_ulong)
                    .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                    .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                - 1 as libc::c_int as libc::c_long) * 2 as libc::c_int as libc::c_long
                + 1 as libc::c_int as libc::c_long
        };
    }
    file_size -= input_offset;
    return file_size;
}
unsafe extern "C" fn read_input_reservoir_sampling(
    mut in_0: *mut FILE,
    mut eolbyte: libc::c_char,
    mut k: size_t,
    mut s: *mut randint_source,
    mut out_rsrv: *mut *mut linebuffer,
) -> size_t {
    let mut n_lines: randint = 0 as libc::c_int as randint;
    let mut n_alloc_lines: size_t = if k
        < RESERVOIR_LINES_INCREMENT as libc::c_int as libc::c_ulong
    {
        k
    } else {
        RESERVOIR_LINES_INCREMENT as libc::c_int as libc::c_ulong
    };
    let mut line: *mut linebuffer = 0 as *mut linebuffer;
    let mut rsrv: *mut linebuffer = 0 as *mut linebuffer;
    rsrv = xcalloc(n_alloc_lines, ::core::mem::size_of::<linebuffer>() as libc::c_ulong)
        as *mut linebuffer;
    while n_lines < k
        && {
            line = readlinebuffer_delim(
                &mut *rsrv.offset(n_lines as isize),
                in_0,
                eolbyte,
            );
            !line.is_null()
        }
    {
        n_lines = n_lines.wrapping_add(1);
        if n_lines >= n_alloc_lines {
            n_alloc_lines = (n_alloc_lines as libc::c_ulong)
                .wrapping_add(RESERVOIR_LINES_INCREMENT as libc::c_int as libc::c_ulong)
                as size_t as size_t;
            rsrv = xnrealloc(
                rsrv as *mut libc::c_void,
                n_alloc_lines,
                ::core::mem::size_of::<linebuffer>() as libc::c_ulong,
            ) as *mut linebuffer;
            memset(
                &mut *rsrv.offset(n_lines as isize) as *mut linebuffer
                    as *mut libc::c_void,
                0 as libc::c_int,
                (RESERVOIR_LINES_INCREMENT as libc::c_int as libc::c_ulong)
                    .wrapping_mul(::core::mem::size_of::<linebuffer>() as libc::c_ulong),
            );
        }
    }
    if !line.is_null() {
        let mut dummy: linebuffer = linebuffer {
            size: 0,
            length: 0,
            buffer: 0 as *mut libc::c_char,
        };
        initbuffer(&mut dummy);
        loop {
            let mut j: randint = randint_choose(
                s,
                n_lines.wrapping_add(1 as libc::c_int as libc::c_ulong),
            );
            line = if j < k {
                &mut *rsrv.offset(j as isize) as *mut linebuffer
            } else {
                &mut dummy
            };
            if !(!(readlinebuffer_delim(line, in_0, eolbyte)).is_null()
                && {
                    let fresh4 = n_lines;
                    n_lines = n_lines.wrapping_add(1);
                    fresh4 != 0
                })
            {
                break;
            }
        }
        if n_lines == 0 {
            if ::core::mem::size_of::<C2RustUnnamed_4>() as libc::c_ulong != 0 {
                error(
                    1 as libc::c_int,
                    75 as libc::c_int,
                    dcgettext(
                        0 as *const libc::c_char,
                        b"too many input lines\0" as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                );
                if 0 as libc::c_int != 0 {} else {
                    unreachable!();
                };
            } else {
                error(
                    1 as libc::c_int,
                    75 as libc::c_int,
                    dcgettext(
                        0 as *const libc::c_char,
                        b"too many input lines\0" as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                );
                if 0 as libc::c_int != 0 {} else {
                    unreachable!();
                };
            };
        }
        freebuffer(&mut dummy);
    }
    if ferror_unlocked(in_0) != 0 {
        if ::core::mem::size_of::<C2RustUnnamed_3>() as libc::c_ulong != 0 {
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
    *out_rsrv = rsrv;
    return if k < n_lines { k } else { n_lines };
}
unsafe extern "C" fn write_permuted_output_reservoir(
    mut n_lines: size_t,
    mut lines: *mut linebuffer,
    mut permutation: *const size_t,
) -> libc::c_int {
    let mut i: size_t = 0 as libc::c_int as size_t;
    while i < n_lines {
        let mut p: *const linebuffer = &mut *lines
            .offset(*permutation.offset(i as isize) as isize) as *mut linebuffer;
        if (if 0 != 0 && 0 != 0
            && (::core::mem::size_of::<libc::c_char>() as libc::c_ulong)
                .wrapping_mul((*p).length as size_t) <= 8 as libc::c_int as libc::c_ulong
            && ::core::mem::size_of::<libc::c_char>() as libc::c_ulong
                != 0 as libc::c_int as libc::c_ulong
        {
            ({
                let mut __ptr: *const libc::c_char = (*p).buffer as *const libc::c_char;
                let mut __stream: *mut FILE = stdout;
                let mut __cnt: size_t = 0;
                __cnt = (::core::mem::size_of::<libc::c_char>() as libc::c_ulong)
                    .wrapping_mul((*p).length as size_t);
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
                (::core::mem::size_of::<libc::c_char>() as libc::c_ulong)
                    .wrapping_mul((*p).length as size_t)
                    .wrapping_sub(__cnt)
                    .wrapping_div(
                        ::core::mem::size_of::<libc::c_char>() as libc::c_ulong,
                    )
            })
        } else {
            (if 0 != 0
                && ::core::mem::size_of::<libc::c_char>() as libc::c_ulong
                    == 0 as libc::c_int as libc::c_ulong
                || 0 != 0 && (*p).length as size_t == 0 as libc::c_int as libc::c_ulong
            {
                0 as libc::c_int as size_t
            } else {
                fwrite_unlocked(
                    (*p).buffer as *const libc::c_void,
                    ::core::mem::size_of::<libc::c_char>() as libc::c_ulong,
                    (*p).length as size_t,
                    stdout,
                )
            })
        }) != (*p).length as libc::c_ulong
        {
            return -(1 as libc::c_int);
        }
        i = i.wrapping_add(1);
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn read_input(
    mut in_0: *mut FILE,
    mut eolbyte: libc::c_char,
    mut pline: *mut *mut *mut libc::c_char,
) -> size_t {
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut buf: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut used: size_t = 0;
    let mut lim: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut line: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    let mut n_lines: size_t = 0;
    buf = fread_file(in_0, 0 as libc::c_int, &mut used);
    if buf.is_null() {
        if ::core::mem::size_of::<C2RustUnnamed_5>() as libc::c_ulong != 0 {
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
    if used != 0
        && *buf.offset(used.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize)
            as libc::c_int != eolbyte as libc::c_int
    {
        let fresh6 = used;
        used = used.wrapping_add(1);
        *buf.offset(fresh6 as isize) = eolbyte;
    }
    lim = buf.offset(used as isize);
    n_lines = 0 as libc::c_int as size_t;
    p = buf;
    while p < lim {
        n_lines = n_lines.wrapping_add(1);
        p = next_line(p, eolbyte);
    }
    line = xnmalloc(
        n_lines.wrapping_add(1 as libc::c_int as libc::c_ulong),
        ::core::mem::size_of::<*mut libc::c_char>() as libc::c_ulong,
    ) as *mut *mut libc::c_char;
    *pline = line;
    p = buf;
    let ref mut fresh7 = *line.offset(0 as libc::c_int as isize);
    *fresh7 = p;
    let mut i: size_t = 1 as libc::c_int as size_t;
    while i <= n_lines {
        p = next_line(p, eolbyte);
        let ref mut fresh8 = *line.offset(i as isize);
        *fresh8 = p;
        i = i.wrapping_add(1);
    }
    return n_lines;
}
unsafe extern "C" fn write_permuted_lines(
    mut n_lines: size_t,
    mut line: *const *mut libc::c_char,
    mut permutation: *const size_t,
) -> libc::c_int {
    let mut i: size_t = 0 as libc::c_int as size_t;
    while i < n_lines {
        let mut p: *const *mut libc::c_char = line
            .offset(*permutation.offset(i as isize) as isize);
        let mut len: size_t = (*p.offset(1 as libc::c_int as isize))
            .offset_from(*p.offset(0 as libc::c_int as isize)) as libc::c_long as size_t;
        if (if 0 != 0 && 0 != 0
            && (::core::mem::size_of::<libc::c_char>() as libc::c_ulong)
                .wrapping_mul(len) <= 8 as libc::c_int as libc::c_ulong
            && ::core::mem::size_of::<libc::c_char>() as libc::c_ulong
                != 0 as libc::c_int as libc::c_ulong
        {
            ({
                let mut __ptr: *const libc::c_char = *p.offset(0 as libc::c_int as isize)
                    as *const libc::c_char;
                let mut __stream: *mut FILE = stdout;
                let mut __cnt: size_t = 0;
                __cnt = (::core::mem::size_of::<libc::c_char>() as libc::c_ulong)
                    .wrapping_mul(len);
                while __cnt > 0 as libc::c_int as libc::c_ulong {
                    let fresh9 = __ptr;
                    __ptr = __ptr.offset(1);
                    if putc_unlocked(*fresh9 as libc::c_int, __stream)
                        == -(1 as libc::c_int)
                    {
                        break;
                    }
                    __cnt = __cnt.wrapping_sub(1);
                }
                (::core::mem::size_of::<libc::c_char>() as libc::c_ulong)
                    .wrapping_mul(len)
                    .wrapping_sub(__cnt)
                    .wrapping_div(
                        ::core::mem::size_of::<libc::c_char>() as libc::c_ulong,
                    )
            })
        } else {
            (if 0 != 0
                && ::core::mem::size_of::<libc::c_char>() as libc::c_ulong
                    == 0 as libc::c_int as libc::c_ulong
                || 0 != 0 && len == 0 as libc::c_int as libc::c_ulong
            {
                0 as libc::c_int as size_t
            } else {
                fwrite_unlocked(
                    *p.offset(0 as libc::c_int as isize) as *const libc::c_void,
                    ::core::mem::size_of::<libc::c_char>() as libc::c_ulong,
                    len,
                    stdout,
                )
            })
        }) != len
        {
            return -(1 as libc::c_int);
        }
        i = i.wrapping_add(1);
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn write_permuted_numbers(
    mut n_lines: size_t,
    mut lo_input: size_t,
    mut permutation: *const size_t,
    mut eolbyte: libc::c_char,
) -> libc::c_int {
    let mut i: size_t = 0 as libc::c_int as size_t;
    while i < n_lines {
        let mut n: libc::c_ulong = lo_input
            .wrapping_add(*permutation.offset(i as isize));
        if printf(
            b"%lu%c\0" as *const u8 as *const libc::c_char,
            n,
            eolbyte as libc::c_int,
        ) < 0 as libc::c_int
        {
            return -(1 as libc::c_int);
        }
        i = i.wrapping_add(1);
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn write_random_numbers(
    mut s: *mut randint_source,
    mut count: size_t,
    mut lo_input: size_t,
    mut hi_input: size_t,
    mut eolbyte: libc::c_char,
) -> libc::c_int {
    let range: randint = hi_input
        .wrapping_sub(lo_input)
        .wrapping_add(1 as libc::c_int as libc::c_ulong);
    let mut i: size_t = 0 as libc::c_int as size_t;
    while i < count {
        let mut j: libc::c_ulong = lo_input.wrapping_add(randint_choose(s, range));
        if printf(
            b"%lu%c\0" as *const u8 as *const libc::c_char,
            j,
            eolbyte as libc::c_int,
        ) < 0 as libc::c_int
        {
            return -(1 as libc::c_int);
        }
        i = i.wrapping_add(1);
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn write_random_lines(
    mut s: *mut randint_source,
    mut count: size_t,
    mut lines: *const *mut libc::c_char,
    mut n_lines: size_t,
) -> libc::c_int {
    let mut i: size_t = 0 as libc::c_int as size_t;
    while i < count {
        let j: randint = randint_choose(s, n_lines);
        let mut p: *const *mut libc::c_char = lines.offset(j as isize);
        let mut len: size_t = (*p.offset(1 as libc::c_int as isize))
            .offset_from(*p.offset(0 as libc::c_int as isize)) as libc::c_long as size_t;
        if (if 0 != 0 && 0 != 0
            && (::core::mem::size_of::<libc::c_char>() as libc::c_ulong)
                .wrapping_mul(len) <= 8 as libc::c_int as libc::c_ulong
            && ::core::mem::size_of::<libc::c_char>() as libc::c_ulong
                != 0 as libc::c_int as libc::c_ulong
        {
            ({
                let mut __ptr: *const libc::c_char = *p.offset(0 as libc::c_int as isize)
                    as *const libc::c_char;
                let mut __stream: *mut FILE = stdout;
                let mut __cnt: size_t = 0;
                __cnt = (::core::mem::size_of::<libc::c_char>() as libc::c_ulong)
                    .wrapping_mul(len);
                while __cnt > 0 as libc::c_int as libc::c_ulong {
                    let fresh10 = __ptr;
                    __ptr = __ptr.offset(1);
                    if putc_unlocked(*fresh10 as libc::c_int, __stream)
                        == -(1 as libc::c_int)
                    {
                        break;
                    }
                    __cnt = __cnt.wrapping_sub(1);
                }
                (::core::mem::size_of::<libc::c_char>() as libc::c_ulong)
                    .wrapping_mul(len)
                    .wrapping_sub(__cnt)
                    .wrapping_div(
                        ::core::mem::size_of::<libc::c_char>() as libc::c_ulong,
                    )
            })
        } else {
            (if 0 != 0
                && ::core::mem::size_of::<libc::c_char>() as libc::c_ulong
                    == 0 as libc::c_int as libc::c_ulong
                || 0 != 0 && len == 0 as libc::c_int as libc::c_ulong
            {
                0 as libc::c_int as size_t
            } else {
                fwrite_unlocked(
                    *p.offset(0 as libc::c_int as isize) as *const libc::c_void,
                    ::core::mem::size_of::<libc::c_char>() as libc::c_ulong,
                    len,
                    stdout,
                )
            })
        }) != len
        {
            return -(1 as libc::c_int);
        }
        i = i.wrapping_add(1);
    }
    return 0 as libc::c_int;
}
unsafe fn main_0(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut echo: bool = 0 as libc::c_int != 0;
    let mut input_range: bool = 0 as libc::c_int != 0;
    let mut lo_input: size_t = 18446744073709551615 as libc::c_ulong;
    let mut hi_input: size_t = 0 as libc::c_int as size_t;
    let mut head_lines: size_t = 18446744073709551615 as libc::c_ulong;
    let mut outfile: *const libc::c_char = 0 as *const libc::c_char;
    let mut random_source: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut eolbyte: libc::c_char = '\n' as i32 as libc::c_char;
    let mut input_lines: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    let mut use_reservoir_sampling: bool = 0 as libc::c_int != 0;
    let mut repeat: bool = 0 as libc::c_int != 0;
    let mut optc: libc::c_int = 0;
    let mut n_operands: libc::c_int = 0;
    let mut operand: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    let mut n_lines: size_t = 0;
    let mut line: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    let mut reservoir: *mut linebuffer = 0 as *mut linebuffer;
    let mut randint_source: *mut randint_source = 0 as *mut randint_source;
    let mut permutation: *mut size_t = 0 as *mut size_t;
    let mut i: libc::c_int = 0;
    set_program_name(*argv.offset(0 as libc::c_int as isize));
    setlocale(6 as libc::c_int, b"\0" as *const u8 as *const libc::c_char);
    bindtextdomain(
        b"coreutils\0" as *const u8 as *const libc::c_char,
        b"/usr/local/share/locale\0" as *const u8 as *const libc::c_char,
    );
    textdomain(b"coreutils\0" as *const u8 as *const libc::c_char);
    atexit(Some(close_stdout as unsafe extern "C" fn() -> ()));
    loop {
        optc = getopt_long(
            argc,
            argv,
            b"ei:n:o:rz\0" as *const u8 as *const libc::c_char,
            long_opts.as_ptr(),
            0 as *mut libc::c_int,
        );
        if !(optc != -(1 as libc::c_int)) {
            break;
        }
        match optc {
            101 => {
                echo = 1 as libc::c_int != 0;
            }
            105 => {
                if input_range {
                    if ::core::mem::size_of::<C2RustUnnamed_16>() as libc::c_ulong != 0 {
                        error(
                            1 as libc::c_int,
                            0 as libc::c_int,
                            dcgettext(
                                0 as *const libc::c_char,
                                b"multiple -i options specified\0" as *const u8
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
                                b"multiple -i options specified\0" as *const u8
                                    as *const libc::c_char,
                                5 as libc::c_int,
                            ),
                        );
                        if 0 as libc::c_int != 0 {} else {
                            unreachable!();
                        };
                    };
                }
                input_range = 1 as libc::c_int != 0;
                let mut u: uintmax_t = 0;
                let mut lo_end: *mut libc::c_char = 0 as *mut libc::c_char;
                let mut err: strtol_error = xstrtoumax(
                    optarg,
                    &mut lo_end,
                    10 as libc::c_int,
                    &mut u,
                    0 as *const libc::c_char,
                );
                if err as libc::c_uint == LONGINT_OK as libc::c_int as libc::c_uint {
                    lo_input = u;
                    if lo_input != u {
                        err = LONGINT_OVERFLOW;
                    } else if *lo_end as libc::c_int != '-' as i32 {
                        err = LONGINT_INVALID;
                    } else {
                        err = xstrtoumax(
                            lo_end.offset(1 as libc::c_int as isize),
                            0 as *mut *mut libc::c_char,
                            10 as libc::c_int,
                            &mut u,
                            b"\0" as *const u8 as *const libc::c_char,
                        );
                        if err as libc::c_uint
                            == LONGINT_OK as libc::c_int as libc::c_uint
                        {
                            hi_input = u;
                            if hi_input != u {
                                err = LONGINT_OVERFLOW;
                            }
                        }
                    }
                }
                n_lines = hi_input
                    .wrapping_sub(lo_input)
                    .wrapping_add(1 as libc::c_int as libc::c_ulong);
                if err as libc::c_uint != LONGINT_OK as libc::c_int as libc::c_uint
                    || (lo_input <= hi_input) as libc::c_int
                        == (n_lines == 0 as libc::c_int as libc::c_ulong) as libc::c_int
                {
                    if ::core::mem::size_of::<C2RustUnnamed_15>() as libc::c_ulong != 0 {
                        error(
                            1 as libc::c_int,
                            (if err as libc::c_uint
                                == LONGINT_OVERFLOW as libc::c_int as libc::c_uint
                            {
                                75 as libc::c_int
                            } else {
                                0 as libc::c_int
                            }),
                            b"%s: %s\0" as *const u8 as *const libc::c_char,
                            dcgettext(
                                0 as *const libc::c_char,
                                b"invalid input range\0" as *const u8
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
                            (if err as libc::c_uint
                                == LONGINT_OVERFLOW as libc::c_int as libc::c_uint
                            {
                                75 as libc::c_int
                            } else {
                                0 as libc::c_int
                            }),
                            b"%s: %s\0" as *const u8 as *const libc::c_char,
                            dcgettext(
                                0 as *const libc::c_char,
                                b"invalid input range\0" as *const u8
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
            110 => {
                let mut argval: uintmax_t = 0;
                let mut e: strtol_error = xstrtoumax(
                    optarg,
                    0 as *mut *mut libc::c_char,
                    10 as libc::c_int,
                    &mut argval,
                    b"\0" as *const u8 as *const libc::c_char,
                );
                if e as libc::c_uint == LONGINT_OK as libc::c_int as libc::c_uint {
                    head_lines = if head_lines < argval { head_lines } else { argval };
                } else if e as libc::c_uint
                    != LONGINT_OVERFLOW as libc::c_int as libc::c_uint
                {
                    if ::core::mem::size_of::<C2RustUnnamed_14>() as libc::c_ulong != 0 {
                        error(
                            1 as libc::c_int,
                            0 as libc::c_int,
                            dcgettext(
                                0 as *const libc::c_char,
                                b"invalid line count: %s\0" as *const u8
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
                                b"invalid line count: %s\0" as *const u8
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
            111 => {
                if !outfile.is_null() && !(strcmp(outfile, optarg) == 0 as libc::c_int) {
                    if ::core::mem::size_of::<C2RustUnnamed_13>() as libc::c_ulong != 0 {
                        error(
                            1 as libc::c_int,
                            0 as libc::c_int,
                            dcgettext(
                                0 as *const libc::c_char,
                                b"multiple output files specified\0" as *const u8
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
                                b"multiple output files specified\0" as *const u8
                                    as *const libc::c_char,
                                5 as libc::c_int,
                            ),
                        );
                        if 0 as libc::c_int != 0 {} else {
                            unreachable!();
                        };
                    };
                }
                outfile = optarg;
            }
            256 => {
                if !random_source.is_null()
                    && !(strcmp(random_source, optarg) == 0 as libc::c_int)
                {
                    if ::core::mem::size_of::<C2RustUnnamed_12>() as libc::c_ulong != 0 {
                        error(
                            1 as libc::c_int,
                            0 as libc::c_int,
                            dcgettext(
                                0 as *const libc::c_char,
                                b"multiple random sources specified\0" as *const u8
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
                                b"multiple random sources specified\0" as *const u8
                                    as *const libc::c_char,
                                5 as libc::c_int,
                            ),
                        );
                        if 0 as libc::c_int != 0 {} else {
                            unreachable!();
                        };
                    };
                }
                random_source = optarg;
            }
            114 => {
                repeat = 1 as libc::c_int != 0;
            }
            122 => {
                eolbyte = '\0' as i32 as libc::c_char;
            }
            -2 => {
                usage(0 as libc::c_int);
            }
            -3 => {
                version_etc(
                    stdout,
                    b"shuf\0" as *const u8 as *const libc::c_char,
                    b"GNU coreutils\0" as *const u8 as *const libc::c_char,
                    Version,
                    b"Paul Eggert\0" as *const u8 as *const libc::c_char,
                    0 as *mut libc::c_void as *mut libc::c_char,
                );
                exit(0 as libc::c_int);
            }
            _ => {
                usage(1 as libc::c_int);
            }
        }
    }
    n_operands = argc - optind;
    operand = argv.offset(optind as isize);
    if echo as libc::c_int != 0 && input_range as libc::c_int != 0 {
        error(
            0 as libc::c_int,
            0 as libc::c_int,
            dcgettext(
                0 as *const libc::c_char,
                b"cannot combine -e and -i options\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
        usage(1 as libc::c_int);
    }
    if if input_range as libc::c_int != 0 {
        ((0 as libc::c_int) < n_operands) as libc::c_int
    } else {
        (!echo && (1 as libc::c_int) < n_operands) as libc::c_int
    } != 0
    {
        error(
            0 as libc::c_int,
            0 as libc::c_int,
            dcgettext(
                0 as *const libc::c_char,
                b"extra operand %s\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            quote(*operand.offset(!input_range as libc::c_int as isize)),
        );
        usage(1 as libc::c_int);
    }
    if head_lines == 0 as libc::c_int as libc::c_ulong {
        n_lines = 0 as libc::c_int as size_t;
        line = 0 as *mut *mut libc::c_char;
    } else if echo {
        input_from_argv(operand, n_operands, eolbyte);
        n_lines = n_operands as size_t;
        line = operand;
    } else if input_range {
        n_lines = hi_input
            .wrapping_sub(lo_input)
            .wrapping_add(1 as libc::c_int as libc::c_ulong);
        line = 0 as *mut *mut libc::c_char;
    } else {
        if n_operands == 1 as libc::c_int
            && !(strcmp(
                *operand.offset(0 as libc::c_int as isize),
                b"-\0" as *const u8 as *const libc::c_char,
            ) == 0 as libc::c_int
                || !(freopen_safer(
                    *operand.offset(0 as libc::c_int as isize),
                    b"r\0" as *const u8 as *const libc::c_char,
                    stdin,
                ))
                    .is_null())
        {
            if ::core::mem::size_of::<C2RustUnnamed_11>() as libc::c_ulong != 0 {
                error(
                    1 as libc::c_int,
                    *__errno_location(),
                    b"%s\0" as *const u8 as *const libc::c_char,
                    quotearg_n_style_colon(
                        0 as libc::c_int,
                        shell_escape_quoting_style,
                        *operand.offset(0 as libc::c_int as isize),
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
                        *operand.offset(0 as libc::c_int as isize),
                    ),
                );
                if 0 as libc::c_int != 0 {} else {
                    unreachable!();
                };
            };
        }
        fadvise(stdin, FADVISE_SEQUENTIAL);
        if repeat as libc::c_int != 0
            || head_lines == 18446744073709551615 as libc::c_ulong
            || input_size() <= RESERVOIR_MIN_INPUT as libc::c_int as libc::c_long
        {
            n_lines = read_input(stdin, eolbyte, &mut input_lines);
            line = input_lines;
        } else {
            use_reservoir_sampling = 1 as libc::c_int != 0;
            n_lines = 18446744073709551615 as libc::c_ulong;
        }
    }
    let mut ahead_lines: size_t = if repeat as libc::c_int != 0 || head_lines < n_lines {
        head_lines
    } else {
        n_lines
    };
    randint_source = randint_all_new(
        random_source,
        if use_reservoir_sampling as libc::c_int != 0 || repeat as libc::c_int != 0 {
            18446744073709551615 as libc::c_ulong
        } else {
            randperm_bound(ahead_lines, n_lines)
        },
    );
    if randint_source.is_null() {
        if ::core::mem::size_of::<C2RustUnnamed_10>() as libc::c_ulong != 0 {
            error(
                1 as libc::c_int,
                *__errno_location(),
                b"%s\0" as *const u8 as *const libc::c_char,
                quotearg_n_style_colon(
                    0 as libc::c_int,
                    shell_escape_quoting_style,
                    (if !random_source.is_null() {
                        random_source as *const libc::c_char
                    } else {
                        b"getrandom\0" as *const u8 as *const libc::c_char
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
                    (if !random_source.is_null() {
                        random_source as *const libc::c_char
                    } else {
                        b"getrandom\0" as *const u8 as *const libc::c_char
                    }),
                ),
            );
            if 0 as libc::c_int != 0 {} else {
                unreachable!();
            };
        };
    }
    if use_reservoir_sampling {
        n_lines = read_input_reservoir_sampling(
            stdin,
            eolbyte,
            ahead_lines,
            randint_source,
            &mut reservoir,
        );
        ahead_lines = n_lines;
    }
    if !(head_lines == 0 as libc::c_int as libc::c_ulong || echo as libc::c_int != 0
        || input_range as libc::c_int != 0 || rpl_fclose(stdin) == 0 as libc::c_int)
    {
        if ::core::mem::size_of::<C2RustUnnamed_9>() as libc::c_ulong != 0 {
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
    if !repeat {
        permutation = randperm_new(randint_source, ahead_lines, n_lines);
    }
    if !outfile.is_null()
        && (freopen_safer(outfile, b"w\0" as *const u8 as *const libc::c_char, stdout))
            .is_null()
    {
        if ::core::mem::size_of::<C2RustUnnamed_8>() as libc::c_ulong != 0 {
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
    if repeat {
        if head_lines == 0 as libc::c_int as libc::c_ulong {
            i = 0 as libc::c_int;
        } else {
            if n_lines == 0 as libc::c_int as libc::c_ulong {
                if ::core::mem::size_of::<C2RustUnnamed_7>() as libc::c_ulong != 0 {
                    error(
                        1 as libc::c_int,
                        0 as libc::c_int,
                        dcgettext(
                            0 as *const libc::c_char,
                            b"no lines to repeat\0" as *const u8 as *const libc::c_char,
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
                            b"no lines to repeat\0" as *const u8 as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                    );
                    if 0 as libc::c_int != 0 {} else {
                        unreachable!();
                    };
                };
            }
            if input_range {
                i = write_random_numbers(
                    randint_source,
                    ahead_lines,
                    lo_input,
                    hi_input,
                    eolbyte,
                );
            } else {
                i = write_random_lines(randint_source, ahead_lines, line, n_lines);
            }
        }
    } else if use_reservoir_sampling {
        i = write_permuted_output_reservoir(n_lines, reservoir, permutation);
    } else if input_range {
        i = write_permuted_numbers(ahead_lines, lo_input, permutation, eolbyte);
    } else {
        i = write_permuted_lines(ahead_lines, line, permutation);
    }
    if i != 0 as libc::c_int {
        if ::core::mem::size_of::<C2RustUnnamed_6>() as libc::c_ulong != 0 {
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
