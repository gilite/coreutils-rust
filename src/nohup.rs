#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![feature(extern_types)]
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    static mut stdout: *mut FILE;
    static mut stderr: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn fputs_unlocked(__s: *const libc::c_char, __stream: *mut FILE) -> libc::c_int;
    fn signal(__sig: libc::c_int, __handler: __sighandler_t) -> __sighandler_t;
    fn close(__fd: libc::c_int) -> libc::c_int;
    fn dup2(__fd: libc::c_int, __fd2: libc::c_int) -> libc::c_int;
    fn execvp(
        __file: *const libc::c_char,
        __argv: *const *mut libc::c_char,
    ) -> libc::c_int;
    static mut optind: libc::c_int;
    fn isatty(__fd: libc::c_int) -> libc::c_int;
    fn umask(__mask: __mode_t) -> __mode_t;
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
    fn getenv(__name: *const libc::c_char) -> *mut libc::c_char;
    static mut Version: *const libc::c_char;
    static mut exit_failure: libc::c_int;
    fn open(__file: *const libc::c_char, __oflag: libc::c_int, _: ...) -> libc::c_int;
    fn rpl_fcntl(fd: libc::c_int, action: libc::c_int, _: ...) -> libc::c_int;
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
    fn close_stdout();
    static mut program_name: *const libc::c_char;
    fn set_program_name(argv0: *const libc::c_char);
    fn quotearg_style(s: quoting_style, arg: *const libc::c_char) -> *mut libc::c_char;
    fn error(
        __status: libc::c_int,
        __errnum: libc::c_int,
        __format: *const libc::c_char,
        _: ...
    );
    fn file_name_concat(
        dir: *const libc::c_char,
        base: *const libc::c_char,
        base_in_result: *mut *mut libc::c_char,
    ) -> *mut libc::c_char;
    fn fd_reopen(
        _: libc::c_int,
        _: *const libc::c_char,
        _: libc::c_int,
        _: mode_t,
    ) -> libc::c_int;
    fn parse_gnu_standard_options_only(
        argc: libc::c_int,
        argv: *mut *mut libc::c_char,
        command_name: *const libc::c_char,
        package: *const libc::c_char,
        version: *const libc::c_char,
        scan_all: bool,
        usage_func: Option::<unsafe extern "C" fn(libc::c_int) -> ()>,
        _: ...
    );
}
pub type size_t = libc::c_ulong;
pub type __mode_t = libc::c_uint;
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
pub type mode_t = __mode_t;
pub type __sighandler_t = Option::<unsafe extern "C" fn(libc::c_int) -> ()>;
pub type C2RustUnnamed = libc::c_uint;
pub const EXIT_ENOENT: C2RustUnnamed = 127;
pub const EXIT_CANNOT_INVOKE: C2RustUnnamed = 126;
pub const EXIT_CANCELED: C2RustUnnamed = 125;
pub const EXIT_TIMEDOUT: C2RustUnnamed = 124;
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
pub type C2RustUnnamed_0 = libc::c_uint;
pub const POSIX_NOHUP_FAILURE: C2RustUnnamed_0 = 127;
#[inline]
unsafe extern "C" fn ferror_unlocked(mut __stream: *mut FILE) -> libc::c_int {
    return ((*__stream)._flags & 0x20 as libc::c_int != 0 as libc::c_int) as libc::c_int;
}
#[inline]
unsafe extern "C" fn initialize_exit_failure(mut status: libc::c_int) {
    if status != 1 as libc::c_int {
        ::core::ptr::write_volatile(&mut exit_failure as *mut libc::c_int, status);
    }
}
#[inline]
unsafe extern "C" fn emit_exec_status(mut program: *const libc::c_char) {
    printf(
        dcgettext(
            0 as *const libc::c_char,
            b"\nExit status:\n  125  if the %s command itself fails\n  126  if COMMAND is found but cannot be invoked\n  127  if COMMAND cannot be found\n  -    the exit status of COMMAND otherwise\n\0"
                as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        ),
        program,
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
                b"Usage: %s COMMAND [ARG]...\n  or:  %s OPTION\n\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
            program_name,
            program_name,
        );
        fputs_unlocked(
            dcgettext(
                0 as *const libc::c_char,
                b"Run COMMAND, ignoring hangup signals.\n\n\0" as *const u8
                    as *const libc::c_char,
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
        printf(
            dcgettext(
                0 as *const libc::c_char,
                b"\nIf standard input is a terminal, redirect it from an unreadable file.\nIf standard output is a terminal, append output to 'nohup.out' if possible,\n'$HOME/nohup.out' otherwise.\nIf standard error is a terminal, redirect it to standard output.\nTo save output to FILE, use '%s COMMAND > FILE'.\n\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            program_name,
        );
        printf(
            dcgettext(
                0 as *const libc::c_char,
                b"\nNOTE: your shell may have its own version of %s, which usually supersedes\nthe version described here.  Please refer to your shell's documentation\nfor details about the options it supports.\n\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            b"nohup\0" as *const u8 as *const libc::c_char,
        );
        emit_exec_status(b"nohup\0" as *const u8 as *const libc::c_char);
        emit_ancillary_info(b"nohup\0" as *const u8 as *const libc::c_char);
    }
    exit(status);
}
unsafe fn main_0(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut out_fd: libc::c_int = 1 as libc::c_int;
    let mut saved_stderr_fd: libc::c_int = 2 as libc::c_int;
    let mut ignoring_input: bool = false;
    let mut redirecting_stdout: bool = false;
    let mut stdout_is_closed: bool = false;
    let mut redirecting_stderr: bool = false;
    let mut exit_internal_failure: libc::c_int = 0;
    set_program_name(*argv.offset(0 as libc::c_int as isize));
    setlocale(6 as libc::c_int, b"\0" as *const u8 as *const libc::c_char);
    bindtextdomain(
        b"coreutils\0" as *const u8 as *const libc::c_char,
        b"/usr/local/share/locale\0" as *const u8 as *const libc::c_char,
    );
    textdomain(b"coreutils\0" as *const u8 as *const libc::c_char);
    exit_internal_failure = if !(getenv(
        b"POSIXLY_CORRECT\0" as *const u8 as *const libc::c_char,
    ))
        .is_null()
    {
        POSIX_NOHUP_FAILURE as libc::c_int
    } else {
        EXIT_CANCELED as libc::c_int
    };
    initialize_exit_failure(exit_internal_failure);
    atexit(Some(close_stdout as unsafe extern "C" fn() -> ()));
    parse_gnu_standard_options_only(
        argc,
        argv,
        b"nohup\0" as *const u8 as *const libc::c_char,
        b"GNU coreutils\0" as *const u8 as *const libc::c_char,
        Version,
        0 as libc::c_int != 0,
        Some(usage as unsafe extern "C" fn(libc::c_int) -> ()),
        b"Jim Meyering\0" as *const u8 as *const libc::c_char,
        0 as *mut libc::c_void as *const libc::c_char,
    );
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
        usage(exit_internal_failure);
    }
    ignoring_input = isatty(0 as libc::c_int) != 0;
    redirecting_stdout = isatty(1 as libc::c_int) != 0;
    stdout_is_closed = !redirecting_stdout && *__errno_location() == 9 as libc::c_int;
    redirecting_stderr = isatty(2 as libc::c_int) != 0;
    if ignoring_input {
        if fd_reopen(
            0 as libc::c_int,
            b"/dev/null\0" as *const u8 as *const libc::c_char,
            0o1 as libc::c_int,
            0 as libc::c_int as mode_t,
        ) < 0 as libc::c_int
        {
            error(
                exit_internal_failure,
                *__errno_location(),
                dcgettext(
                    0 as *const libc::c_char,
                    b"failed to render standard input unusable\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
            );
        }
        if !redirecting_stdout && !redirecting_stderr {
            error(
                0 as libc::c_int,
                0 as libc::c_int,
                dcgettext(
                    0 as *const libc::c_char,
                    b"ignoring input\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
            );
        }
    }
    if redirecting_stdout as libc::c_int != 0
        || redirecting_stderr as libc::c_int != 0 && stdout_is_closed as libc::c_int != 0
    {
        let mut in_home: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut file: *const libc::c_char = b"nohup.out\0" as *const u8
            as *const libc::c_char;
        let mut flags: libc::c_int = 0o100 as libc::c_int | 0o1 as libc::c_int
            | 0o2000 as libc::c_int;
        let mut mode: mode_t = (0o400 as libc::c_int | 0o200 as libc::c_int) as mode_t;
        let mut umask_value: mode_t = umask(!mode);
        out_fd = if redirecting_stdout as libc::c_int != 0 {
            fd_reopen(1 as libc::c_int, file, flags, mode)
        } else {
            open(file, flags, mode)
        };
        if out_fd < 0 as libc::c_int {
            let mut saved_errno: libc::c_int = *__errno_location();
            let mut home: *const libc::c_char = getenv(
                b"HOME\0" as *const u8 as *const libc::c_char,
            );
            if !home.is_null() {
                in_home = file_name_concat(home, file, 0 as *mut *mut libc::c_char);
                out_fd = if redirecting_stdout as libc::c_int != 0 {
                    fd_reopen(1 as libc::c_int, in_home, flags, mode)
                } else {
                    open(in_home, flags, mode)
                };
            }
            if out_fd < 0 as libc::c_int {
                let mut saved_errno2: libc::c_int = *__errno_location();
                error(
                    0 as libc::c_int,
                    saved_errno,
                    dcgettext(
                        0 as *const libc::c_char,
                        b"failed to open %s\0" as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    quotearg_style(shell_escape_always_quoting_style, file),
                );
                if !in_home.is_null() {
                    error(
                        0 as libc::c_int,
                        saved_errno2,
                        dcgettext(
                            0 as *const libc::c_char,
                            b"failed to open %s\0" as *const u8 as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                        quotearg_style(shell_escape_always_quoting_style, in_home),
                    );
                }
                return exit_internal_failure;
            }
            file = in_home;
        }
        umask(umask_value);
        error(
            0 as libc::c_int,
            0 as libc::c_int,
            dcgettext(
                0 as *const libc::c_char,
                if ignoring_input as libc::c_int != 0 {
                    b"ignoring input and appending output to %s\0" as *const u8
                        as *const libc::c_char
                } else {
                    b"appending output to %s\0" as *const u8 as *const libc::c_char
                },
                5 as libc::c_int,
            ),
            quotearg_style(shell_escape_always_quoting_style, file),
        );
        free(in_home as *mut libc::c_void);
    }
    if redirecting_stderr {
        saved_stderr_fd = rpl_fcntl(
            2 as libc::c_int,
            1030 as libc::c_int,
            2 as libc::c_int + 1 as libc::c_int,
        );
        if !redirecting_stdout {
            error(
                0 as libc::c_int,
                0 as libc::c_int,
                dcgettext(
                    0 as *const libc::c_char,
                    if ignoring_input as libc::c_int != 0 {
                        b"ignoring input and redirecting stderr to stdout\0" as *const u8
                            as *const libc::c_char
                    } else {
                        b"redirecting stderr to stdout\0" as *const u8
                            as *const libc::c_char
                    },
                    5 as libc::c_int,
                ),
            );
        }
        if dup2(out_fd, 2 as libc::c_int) < 0 as libc::c_int {
            error(
                exit_internal_failure,
                *__errno_location(),
                dcgettext(
                    0 as *const libc::c_char,
                    b"failed to redirect standard error\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
            );
        }
        if stdout_is_closed {
            close(out_fd);
        }
    }
    if ferror_unlocked(stderr) != 0 {
        return exit_internal_failure;
    }
    signal(
        1 as libc::c_int,
        ::core::mem::transmute::<
            libc::intptr_t,
            __sighandler_t,
        >(1 as libc::c_int as libc::intptr_t),
    );
    let mut cmd: *mut *mut libc::c_char = argv.offset(optind as isize);
    execvp(*cmd, cmd as *const *mut libc::c_char);
    let mut exit_status: libc::c_int = if *__errno_location() == 2 as libc::c_int {
        EXIT_ENOENT as libc::c_int
    } else {
        EXIT_CANNOT_INVOKE as libc::c_int
    };
    let mut saved_errno_0: libc::c_int = *__errno_location();
    if dup2(saved_stderr_fd, 2 as libc::c_int) == 2 as libc::c_int {
        error(
            0 as libc::c_int,
            saved_errno_0,
            dcgettext(
                0 as *const libc::c_char,
                b"failed to run command %s\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            quotearg_style(shell_escape_always_quoting_style, *cmd),
        );
    }
    return exit_status;
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
