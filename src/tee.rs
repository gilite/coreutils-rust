#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![feature(extern_types)]
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn signal(__sig: libc::c_int, __handler: __sighandler_t) -> __sighandler_t;
    fn close(__fd: libc::c_int) -> libc::c_int;
    fn read(__fd: libc::c_int, __buf: *mut libc::c_void, __nbytes: size_t) -> ssize_t;
    static mut optind: libc::c_int;
    static mut optarg: *mut libc::c_char;
    fn getopt_long(
        ___argc: libc::c_int,
        ___argv: *const *mut libc::c_char,
        __shortopts: *const libc::c_char,
        __longopts: *const option,
        __longind: *mut libc::c_int,
    ) -> libc::c_int;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strncmp(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn free(_: *mut libc::c_void);
    fn __errno_location() -> *mut libc::c_int;
    fn atexit(__func: Option::<unsafe extern "C" fn() -> ()>) -> libc::c_int;
    fn exit(_: libc::c_int) -> !;
    static mut Version: *const libc::c_char;
    fn quotearg_n_style_colon(
        n: libc::c_int,
        s: quoting_style,
        arg: *const libc::c_char,
    ) -> *mut libc::c_char;
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
    fn xnmalloc(n: size_t, s: size_t) -> *mut libc::c_void;
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
    fn fputs_unlocked(__s: *const libc::c_char, __stream: *mut FILE) -> libc::c_int;
    fn clearerr_unlocked(__stream: *mut FILE);
    fn fileno(__stream: *mut FILE) -> libc::c_int;
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
    fn error(
        __status: libc::c_int,
        __errnum: libc::c_int,
        __format: *const libc::c_char,
        _: ...
    );
    fn fadvise(fp: *mut FILE, advice: fadvice_t);
    fn iopoll(fdin: libc::c_int, fdout: libc::c_int, block: bool) -> libc::c_int;
    fn iopoll_input_ok(fdin: libc::c_int) -> bool;
    fn iopoll_output_ok(fdout: libc::c_int) -> bool;
    fn fclose_nonblock(f: *mut FILE) -> bool;
    fn fwrite_nonblock(buf: *const libc::c_char, size: ssize_t, f: *mut FILE) -> bool;
    fn fopen_safer(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
}
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __ssize_t = libc::c_long;
pub type ssize_t = __ssize_t;
pub type size_t = libc::c_ulong;
pub type __sighandler_t = Option::<unsafe extern "C" fn(libc::c_int) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct option {
    pub name: *const libc::c_char,
    pub has_arg: libc::c_int,
    pub flag: *mut libc::c_int,
    pub val: libc::c_int,
}
pub type ptrdiff_t = libc::c_long;
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
pub type argmatch_exit_fn = Option::<unsafe extern "C" fn() -> ()>;
pub type fadvice_t = libc::c_uint;
pub const FADVISE_RANDOM: fadvice_t = 1;
pub const FADVISE_WILLNEED: fadvice_t = 3;
pub const FADVISE_DONTNEED: fadvice_t = 4;
pub const FADVISE_NOREUSE: fadvice_t = 5;
pub const FADVISE_SEQUENTIAL: fadvice_t = 2;
pub const FADVISE_NORMAL: fadvice_t = 0;
pub const output_error_warn: output_error = 1;
pub type output_error = libc::c_uint;
pub const output_error_exit_nopipe: output_error = 4;
pub const output_error_exit: output_error = 3;
pub const output_error_warn_nopipe: output_error = 2;
pub const output_error_sigpipe: output_error = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_0 {
    pub _gl_dummy: libc::c_int,
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
unsafe extern "C" fn xset_binary_mode_error() {}
#[inline]
unsafe extern "C" fn xset_binary_mode(mut fd: libc::c_int, mut mode: libc::c_int) {
    if set_binary_mode(fd, mode) < 0 as libc::c_int {
        xset_binary_mode_error();
    }
}
#[inline]
unsafe extern "C" fn set_binary_mode(
    mut fd: libc::c_int,
    mut mode: libc::c_int,
) -> libc::c_int {
    return __gl_setmode(fd, mode);
}
static mut append: bool = false;
static mut ignore_interrupts: bool = false;
static mut output_error: output_error = output_error_sigpipe;
static mut long_options: [option; 6] = [
    {
        let mut init = option {
            name: b"append\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'a' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"ignore-interrupts\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'i' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"output-error\0" as *const u8 as *const libc::c_char,
            has_arg: 2 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'p' as i32,
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
static mut output_error_args: [*const libc::c_char; 5] = [
    b"warn\0" as *const u8 as *const libc::c_char,
    b"warn-nopipe\0" as *const u8 as *const libc::c_char,
    b"exit\0" as *const u8 as *const libc::c_char,
    b"exit-nopipe\0" as *const u8 as *const libc::c_char,
    0 as *const libc::c_char,
];
static mut output_error_types: [output_error; 4] = [
    output_error_warn,
    output_error_warn_nopipe,
    output_error_exit,
    output_error_exit_nopipe,
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
                b"Copy standard input to each FILE, and also to standard output.\n\n  -a, --append              append to the given FILEs, do not overwrite\n  -i, --ignore-interrupts   ignore interrupt signals\n\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            stdout,
        );
        fputs_unlocked(
            dcgettext(
                0 as *const libc::c_char,
                b"  -p                        operate in a more appropriate MODE with pipes.\n      --output-error[=MODE]   set behavior on write error.  See MODE below\n\0"
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
                b"\nMODE determines behavior with write errors on the outputs:\n  warn           diagnose errors writing to any output\n  warn-nopipe    diagnose errors writing to any output not a pipe\n  exit           exit on error writing to any output\n  exit-nopipe    exit on error writing to any output not a pipe\nThe default MODE for the -p option is 'warn-nopipe'.\nWith \"nopipe\" MODEs, exit immediately if all outputs become broken pipes.\nThe default operation when --output-error is not specified, is to\nexit immediately on error writing to a pipe, and diagnose errors\nwriting to non pipe outputs.\n\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            stdout,
        );
        emit_ancillary_info(b"tee\0" as *const u8 as *const libc::c_char);
    }
    exit(status);
}
unsafe fn main_0(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
    set_program_name(*argv.offset(0 as libc::c_int as isize));
    setlocale(6 as libc::c_int, b"\0" as *const u8 as *const libc::c_char);
    bindtextdomain(
        b"coreutils\0" as *const u8 as *const libc::c_char,
        b"/usr/local/share/locale\0" as *const u8 as *const libc::c_char,
    );
    textdomain(b"coreutils\0" as *const u8 as *const libc::c_char);
    atexit(Some(close_stdout as unsafe extern "C" fn() -> ()));
    append = 0 as libc::c_int != 0;
    ignore_interrupts = 0 as libc::c_int != 0;
    let mut optc: libc::c_int = 0;
    loop {
        optc = getopt_long(
            argc,
            argv,
            b"aip\0" as *const u8 as *const libc::c_char,
            long_options.as_ptr(),
            0 as *mut libc::c_int,
        );
        if !(optc != -(1 as libc::c_int)) {
            break;
        }
        match optc {
            97 => {
                append = 1 as libc::c_int != 0;
            }
            105 => {
                ignore_interrupts = 1 as libc::c_int != 0;
            }
            112 => {
                if !optarg.is_null() {
                    output_error = output_error_types[__xargmatch_internal(
                        b"--output-error\0" as *const u8 as *const libc::c_char,
                        optarg,
                        output_error_args.as_ptr(),
                        output_error_types.as_ptr() as *const libc::c_void,
                        ::core::mem::size_of::<output_error>() as libc::c_ulong,
                        argmatch_die,
                        1 as libc::c_int != 0,
                    ) as usize];
                } else {
                    output_error = output_error_warn_nopipe;
                }
            }
            -2 => {
                usage(0 as libc::c_int);
            }
            -3 => {
                version_etc(
                    stdout,
                    b"tee\0" as *const u8 as *const libc::c_char,
                    b"GNU coreutils\0" as *const u8 as *const libc::c_char,
                    Version,
                    b"Mike Parker\0" as *const u8 as *const libc::c_char,
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
    if ignore_interrupts {
        signal(
            2 as libc::c_int,
            ::core::mem::transmute::<
                libc::intptr_t,
                __sighandler_t,
            >(1 as libc::c_int as libc::intptr_t),
        );
    }
    if output_error as libc::c_uint
        != output_error_sigpipe as libc::c_int as libc::c_uint
    {
        signal(
            13 as libc::c_int,
            ::core::mem::transmute::<
                libc::intptr_t,
                __sighandler_t,
            >(1 as libc::c_int as libc::intptr_t),
        );
    }
    let mut pipe_check: bool = (output_error as libc::c_uint
        == output_error_warn_nopipe as libc::c_int as libc::c_uint
        || output_error as libc::c_uint
            == output_error_exit_nopipe as libc::c_int as libc::c_uint)
        && iopoll_input_ok(0 as libc::c_int) as libc::c_int != 0;
    let mut ok: bool = tee_files(
        argc - optind,
        &mut *argv.offset(optind as isize),
        pipe_check,
    );
    if close(0 as libc::c_int) != 0 as libc::c_int {
        if ::core::mem::size_of::<C2RustUnnamed_0>() as libc::c_ulong != 0 {
            error(
                1 as libc::c_int,
                *__errno_location(),
                b"%s\0" as *const u8 as *const libc::c_char,
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
                b"%s\0" as *const u8 as *const libc::c_char,
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
unsafe extern "C" fn get_next_out(
    mut descriptors: *mut *mut FILE,
    mut nfiles: libc::c_int,
    mut idx: libc::c_int,
) -> libc::c_int {
    idx += 1;
    while idx <= nfiles {
        if !(*descriptors.offset(idx as isize)).is_null() {
            return idx;
        }
        idx += 1;
    }
    return -(1 as libc::c_int);
}
unsafe extern "C" fn fail_output(
    mut descriptors: *mut *mut FILE,
    mut files: *mut *mut libc::c_char,
    mut i: libc::c_int,
) -> bool {
    let mut w_errno: libc::c_int = *__errno_location();
    let mut fail: bool = *__errno_location() != 32 as libc::c_int
        || output_error as libc::c_uint
            == output_error_exit as libc::c_int as libc::c_uint
        || output_error as libc::c_uint
            == output_error_warn as libc::c_int as libc::c_uint;
    if *descriptors.offset(i as isize) == stdout {
        clearerr_unlocked(stdout);
    }
    if fail {
        error(
            (output_error as libc::c_uint
                == output_error_exit as libc::c_int as libc::c_uint
                || output_error as libc::c_uint
                    == output_error_exit_nopipe as libc::c_int as libc::c_uint)
                as libc::c_int,
            w_errno,
            b"%s\0" as *const u8 as *const libc::c_char,
            quotearg_n_style_colon(
                0 as libc::c_int,
                shell_escape_quoting_style,
                *files.offset(i as isize),
            ),
        );
    }
    let ref mut fresh0 = *descriptors.offset(i as isize);
    *fresh0 = 0 as *mut FILE;
    return fail;
}
unsafe extern "C" fn tee_files(
    mut nfiles: libc::c_int,
    mut files: *mut *mut libc::c_char,
    mut pipe_check: bool,
) -> bool {
    let mut n_outputs: size_t = 0 as libc::c_int as size_t;
    let mut descriptors: *mut *mut FILE = 0 as *mut *mut FILE;
    let mut out_pollable: *mut bool = 0 as *mut bool;
    let mut buffer: [libc::c_char; 8192] = [0; 8192];
    let mut bytes_read: ssize_t = 0 as libc::c_int as ssize_t;
    let mut i: libc::c_int = 0;
    let mut first_out: libc::c_int = 0 as libc::c_int;
    let mut ok: bool = 1 as libc::c_int != 0;
    let mut mode_string: *const libc::c_char = if 0 as libc::c_int != 0 {
        if append as libc::c_int != 0 {
            b"ab\0" as *const u8 as *const libc::c_char
        } else {
            b"wb\0" as *const u8 as *const libc::c_char
        }
    } else if append as libc::c_int != 0 {
        b"a\0" as *const u8 as *const libc::c_char
    } else {
        b"w\0" as *const u8 as *const libc::c_char
    };
    xset_binary_mode(0 as libc::c_int, 0 as libc::c_int);
    xset_binary_mode(1 as libc::c_int, 0 as libc::c_int);
    fadvise(stdin, FADVISE_SEQUENTIAL);
    descriptors = xnmalloc(
        (nfiles + 1 as libc::c_int) as size_t,
        ::core::mem::size_of::<*mut FILE>() as libc::c_ulong,
    ) as *mut *mut FILE;
    if pipe_check {
        out_pollable = xnmalloc(
            (nfiles + 1 as libc::c_int) as size_t,
            ::core::mem::size_of::<bool>() as libc::c_ulong,
        ) as *mut bool;
    }
    files = files.offset(-1);
    let ref mut fresh1 = *descriptors.offset(0 as libc::c_int as isize);
    *fresh1 = stdout;
    if pipe_check {
        *out_pollable
            .offset(
                0 as libc::c_int as isize,
            ) = iopoll_output_ok(fileno(*descriptors.offset(0 as libc::c_int as isize)));
    }
    let ref mut fresh2 = *files.offset(0 as libc::c_int as isize);
    *fresh2 = bad_cast(
        dcgettext(
            0 as *const libc::c_char,
            b"standard output\0" as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        ),
    );
    setvbuf(
        stdout,
        0 as *mut libc::c_char,
        2 as libc::c_int,
        0 as libc::c_int as size_t,
    );
    n_outputs = n_outputs.wrapping_add(1);
    i = 1 as libc::c_int;
    while i <= nfiles {
        let ref mut fresh3 = *descriptors.offset(i as isize);
        *fresh3 = fopen_safer(*files.offset(i as isize), mode_string);
        if (*descriptors.offset(i as isize)).is_null() {
            if pipe_check {
                *out_pollable.offset(i as isize) = 0 as libc::c_int != 0;
            }
            error(
                (output_error as libc::c_uint
                    == output_error_exit as libc::c_int as libc::c_uint
                    || output_error as libc::c_uint
                        == output_error_exit_nopipe as libc::c_int as libc::c_uint)
                    as libc::c_int,
                *__errno_location(),
                b"%s\0" as *const u8 as *const libc::c_char,
                quotearg_n_style_colon(
                    0 as libc::c_int,
                    shell_escape_quoting_style,
                    *files.offset(i as isize),
                ),
            );
            ok = 0 as libc::c_int != 0;
        } else {
            if pipe_check {
                *out_pollable
                    .offset(
                        i as isize,
                    ) = iopoll_output_ok(fileno(*descriptors.offset(i as isize)));
            }
            setvbuf(
                *descriptors.offset(i as isize),
                0 as *mut libc::c_char,
                2 as libc::c_int,
                0 as libc::c_int as size_t,
            );
            n_outputs = n_outputs.wrapping_add(1);
        }
        i += 1;
    }
    while n_outputs != 0 {
        if pipe_check as libc::c_int != 0
            && *out_pollable.offset(first_out as isize) as libc::c_int != 0
        {
            let mut err: libc::c_int = iopoll(
                0 as libc::c_int,
                fileno(*descriptors.offset(first_out as isize)),
                1 as libc::c_int != 0,
            );
            if err == -(2 as libc::c_int) {
                *__errno_location() = 32 as libc::c_int;
                if fail_output(descriptors, files, first_out) {
                    ok = 0 as libc::c_int != 0;
                }
                n_outputs = n_outputs.wrapping_sub(1);
                first_out = get_next_out(descriptors, nfiles, first_out);
                continue;
            } else if err == -(3 as libc::c_int) {
                error(
                    0 as libc::c_int,
                    *__errno_location(),
                    dcgettext(
                        0 as *const libc::c_char,
                        b"iopoll error\0" as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                );
                ok = 0 as libc::c_int != 0;
            }
        }
        bytes_read = read(
            0 as libc::c_int,
            buffer.as_mut_ptr() as *mut libc::c_void,
            ::core::mem::size_of::<[libc::c_char; 8192]>() as libc::c_ulong,
        );
        if bytes_read < 0 as libc::c_int as libc::c_long
            && *__errno_location() == 4 as libc::c_int
        {
            continue;
        }
        if bytes_read <= 0 as libc::c_int as libc::c_long {
            break;
        }
        i = 0 as libc::c_int;
        while i <= nfiles {
            if !(*descriptors.offset(i as isize)).is_null()
                && !fwrite_nonblock(
                    buffer.as_mut_ptr(),
                    bytes_read,
                    *descriptors.offset(i as isize),
                )
            {
                if fail_output(descriptors, files, i) {
                    ok = 0 as libc::c_int != 0;
                }
                n_outputs = n_outputs.wrapping_sub(1);
                if i == first_out {
                    first_out = get_next_out(descriptors, nfiles, first_out);
                }
            }
            i += 1;
        }
    }
    if bytes_read == -(1 as libc::c_int) as libc::c_long {
        error(
            0 as libc::c_int,
            *__errno_location(),
            dcgettext(
                0 as *const libc::c_char,
                b"read error\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
        ok = 0 as libc::c_int != 0;
    }
    i = 1 as libc::c_int;
    while i <= nfiles {
        if !(*descriptors.offset(i as isize)).is_null()
            && !fclose_nonblock(*descriptors.offset(i as isize))
        {
            error(
                0 as libc::c_int,
                *__errno_location(),
                b"%s\0" as *const u8 as *const libc::c_char,
                quotearg_n_style_colon(
                    0 as libc::c_int,
                    shell_escape_quoting_style,
                    *files.offset(i as isize),
                ),
            );
            ok = 0 as libc::c_int != 0;
        }
        i += 1;
    }
    free(descriptors as *mut libc::c_void);
    if pipe_check {
        free(out_pollable as *mut libc::c_void);
    }
    return ok;
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
