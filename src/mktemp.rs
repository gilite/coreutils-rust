#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![feature(extern_types)]
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn getopt_long(
        ___argc: libc::c_int,
        ___argv: *const *mut libc::c_char,
        __shortopts: *const libc::c_char,
        __longopts: *const option,
        __longind: *mut libc::c_int,
    ) -> libc::c_int;
    static mut optarg: *mut libc::c_char;
    static mut optind: libc::c_int;
    fn close(__fd: libc::c_int) -> libc::c_int;
    fn _exit(_: libc::c_int) -> !;
    fn memcpy(
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
    fn strrchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn free(_: *mut libc::c_void);
    fn __errno_location() -> *mut libc::c_int;
    fn atexit(__func: Option::<unsafe extern "C" fn() -> ()>) -> libc::c_int;
    fn exit(_: libc::c_int) -> !;
    fn getenv(__name: *const libc::c_char) -> *mut libc::c_char;
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
    fn xstrdup(str: *const libc::c_char) -> *mut libc::c_char;
    fn xcharalloc(n: size_t) -> *mut libc::c_char;
    static mut stdout: *mut FILE;
    static mut stderr: *mut FILE;
    fn remove(__filename: *const libc::c_char) -> libc::c_int;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn puts(__s: *const libc::c_char) -> libc::c_int;
    fn fputs_unlocked(__s: *const libc::c_char, __stream: *mut FILE) -> libc::c_int;
    fn last_component(filename: *const libc::c_char) -> *mut libc::c_char;
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
    fn close_stream(stream: *mut FILE) -> libc::c_int;
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
    fn quote(arg: *const libc::c_char) -> *const libc::c_char;
    fn gen_tempname_len(
        tmpl: *mut libc::c_char,
        suffixlen: libc::c_int,
        flags: libc::c_int,
        kind: libc::c_int,
        x_suffix_len: size_t,
    ) -> libc::c_int;
}
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type size_t = libc::c_ulong;
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
pub type C2RustUnnamed_0 = libc::c_uint;
pub const SUFFIX_OPTION: C2RustUnnamed_0 = 256;
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
static mut default_template: *const libc::c_char = b"tmp.XXXXXXXXXX\0" as *const u8
    as *const libc::c_char;
static mut longopts: [option; 8] = [
    {
        let mut init = option {
            name: b"directory\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'd' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"quiet\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'q' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"dry-run\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'u' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"suffix\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: SUFFIX_OPTION as libc::c_int,
        };
        init
    },
    {
        let mut init = option {
            name: b"tmpdir\0" as *const u8 as *const libc::c_char,
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
                b"Usage: %s [OPTION]... [TEMPLATE]\n\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
            program_name,
        );
        fputs_unlocked(
            dcgettext(
                0 as *const libc::c_char,
                b"Create a temporary file or directory, safely, and print its name.\nTEMPLATE must contain at least 3 consecutive 'X's in last component.\nIf TEMPLATE is not specified, use tmp.XXXXXXXXXX, and --tmpdir is implied.\n\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            stdout,
        );
        fputs_unlocked(
            dcgettext(
                0 as *const libc::c_char,
                b"Files are created u+rw, and directories u+rwx, minus umask restrictions.\n\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            stdout,
        );
        fputs_unlocked(b"\n\0" as *const u8 as *const libc::c_char, stdout);
        fputs_unlocked(
            dcgettext(
                0 as *const libc::c_char,
                b"  -d, --directory     create a directory, not a file\n  -u, --dry-run       do not create anything; merely print a name (unsafe)\n  -q, --quiet         suppress diagnostics about file/dir-creation failure\n\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            stdout,
        );
        fputs_unlocked(
            dcgettext(
                0 as *const libc::c_char,
                b"      --suffix=SUFF   append SUFF to TEMPLATE; SUFF must not contain a slash.\n                        This option is implied if TEMPLATE does not end in X\n\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            stdout,
        );
        fputs_unlocked(
            dcgettext(
                0 as *const libc::c_char,
                b"  -p DIR, --tmpdir[=DIR]  interpret TEMPLATE relative to DIR; if DIR is not\n                        specified, use $TMPDIR if set, else /tmp.  With\n                        this option, TEMPLATE must not be an absolute name;\n                        unlike with -t, TEMPLATE may contain slashes, but\n                        mktemp creates only the final component\n\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            stdout,
        );
        fputs_unlocked(
            dcgettext(
                0 as *const libc::c_char,
                b"  -t                  interpret TEMPLATE as a single file name component,\n                        relative to a directory: $TMPDIR, if set; else the\n                        directory specified via -p; else /tmp [deprecated]\n\0"
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
        emit_ancillary_info(b"mktemp\0" as *const u8 as *const libc::c_char);
    }
    exit(status);
}
unsafe extern "C" fn count_consecutive_X_s(
    mut s: *const libc::c_char,
    mut len: size_t,
) -> size_t {
    let mut n: size_t = 0 as libc::c_int as size_t;
    while len != 0
        && *s.offset(len.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize)
            as libc::c_int == 'X' as i32
    {
        n = n.wrapping_add(1);
        len = len.wrapping_sub(1);
    }
    return n;
}
unsafe extern "C" fn mkstemp_len(
    mut tmpl: *mut libc::c_char,
    mut suff_len: size_t,
    mut x_len: size_t,
    mut dry_run: bool,
) -> libc::c_int {
    return gen_tempname_len(
        tmpl,
        suff_len as libc::c_int,
        0 as libc::c_int,
        if dry_run as libc::c_int != 0 { 2 as libc::c_int } else { 0 as libc::c_int },
        x_len,
    );
}
unsafe extern "C" fn mkdtemp_len(
    mut tmpl: *mut libc::c_char,
    mut suff_len: size_t,
    mut x_len: size_t,
    mut dry_run: bool,
) -> libc::c_int {
    return gen_tempname_len(
        tmpl,
        suff_len as libc::c_int,
        0 as libc::c_int,
        if dry_run as libc::c_int != 0 { 2 as libc::c_int } else { 1 as libc::c_int },
        x_len,
    );
}
static mut stdout_closed: bool = false;
unsafe extern "C" fn maybe_close_stdout() {
    if !stdout_closed {
        close_stdout();
    } else if close_stream(stderr) != 0 as libc::c_int {
        _exit(1 as libc::c_int);
    }
}
unsafe fn main_0(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut dest_dir: *const libc::c_char = 0 as *const libc::c_char;
    let mut dest_dir_arg: *const libc::c_char = 0 as *const libc::c_char;
    let mut suppress_file_err: bool = 0 as libc::c_int != 0;
    let mut c: libc::c_int = 0;
    let mut n_args: libc::c_uint = 0;
    let mut template: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut suffix: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut use_dest_dir: bool = 0 as libc::c_int != 0;
    let mut deprecated_t_option: bool = 0 as libc::c_int != 0;
    let mut create_directory: bool = 0 as libc::c_int != 0;
    let mut dry_run: bool = 0 as libc::c_int != 0;
    let mut status: libc::c_int = 0 as libc::c_int;
    let mut x_count: size_t = 0;
    let mut suffix_len: size_t = 0;
    let mut dest_name: *mut libc::c_char = 0 as *mut libc::c_char;
    set_program_name(*argv.offset(0 as libc::c_int as isize));
    setlocale(6 as libc::c_int, b"\0" as *const u8 as *const libc::c_char);
    bindtextdomain(
        b"coreutils\0" as *const u8 as *const libc::c_char,
        b"/usr/local/share/locale\0" as *const u8 as *const libc::c_char,
    );
    textdomain(b"coreutils\0" as *const u8 as *const libc::c_char);
    atexit(Some(maybe_close_stdout as unsafe extern "C" fn() -> ()));
    loop {
        c = getopt_long(
            argc,
            argv,
            b"dp:qtuV\0" as *const u8 as *const libc::c_char,
            longopts.as_ptr(),
            0 as *mut libc::c_int,
        );
        if !(c != -(1 as libc::c_int)) {
            break;
        }
        match c {
            100 => {
                create_directory = 1 as libc::c_int != 0;
            }
            112 => {
                dest_dir_arg = optarg;
                use_dest_dir = 1 as libc::c_int != 0;
            }
            113 => {
                suppress_file_err = 1 as libc::c_int != 0;
            }
            116 => {
                use_dest_dir = 1 as libc::c_int != 0;
                deprecated_t_option = 1 as libc::c_int != 0;
            }
            117 => {
                dry_run = 1 as libc::c_int != 0;
            }
            256 => {
                suffix = optarg;
            }
            -2 => {
                usage(0 as libc::c_int);
            }
            86 | -3 => {
                version_etc(
                    stdout,
                    b"mktemp\0" as *const u8 as *const libc::c_char,
                    b"GNU coreutils\0" as *const u8 as *const libc::c_char,
                    Version,
                    b"Jim Meyering\0" as *const u8 as *const libc::c_char,
                    b"Eric Blake\0" as *const u8 as *const libc::c_char,
                    0 as *mut libc::c_void as *mut libc::c_char,
                );
                exit(0 as libc::c_int);
            }
            _ => {
                usage(1 as libc::c_int);
            }
        }
    }
    n_args = (argc - optind) as libc::c_uint;
    if 2 as libc::c_int as libc::c_uint <= n_args {
        error(
            0 as libc::c_int,
            0 as libc::c_int,
            dcgettext(
                0 as *const libc::c_char,
                b"too many templates\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
        usage(1 as libc::c_int);
    }
    if n_args == 0 as libc::c_int as libc::c_uint {
        use_dest_dir = 1 as libc::c_int != 0;
        template = default_template as *mut libc::c_char;
    } else {
        template = *argv.offset(optind as isize);
    }
    if !suffix.is_null() {
        let mut len: size_t = strlen(template);
        if len == 0
            || *template
                .offset(len.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize)
                as libc::c_int != 'X' as i32
        {
            if ::core::mem::size_of::<C2RustUnnamed_5>() as libc::c_ulong != 0 {
                error(
                    1 as libc::c_int,
                    0 as libc::c_int,
                    dcgettext(
                        0 as *const libc::c_char,
                        b"with --suffix, template %s must end in X\0" as *const u8
                            as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    quote(template),
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
                        b"with --suffix, template %s must end in X\0" as *const u8
                            as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    quote(template),
                );
                if 0 as libc::c_int != 0 {} else {
                    unreachable!();
                };
            };
        }
        suffix_len = strlen(suffix);
        dest_name = xcharalloc(
            len.wrapping_add(suffix_len).wrapping_add(1 as libc::c_int as libc::c_ulong),
        );
        memcpy(dest_name as *mut libc::c_void, template as *const libc::c_void, len);
        memcpy(
            dest_name.offset(len as isize) as *mut libc::c_void,
            suffix as *const libc::c_void,
            suffix_len.wrapping_add(1 as libc::c_int as libc::c_ulong),
        );
        template = dest_name;
        suffix = dest_name.offset(len as isize);
    } else {
        template = xstrdup(template);
        suffix = strrchr(template, 'X' as i32);
        if suffix.is_null() {
            suffix = strchr(template, '\0' as i32);
        } else {
            suffix = suffix.offset(1);
        }
        suffix_len = strlen(suffix);
    }
    if suffix_len != 0 && last_component(suffix) != suffix {
        if ::core::mem::size_of::<C2RustUnnamed_4>() as libc::c_ulong != 0 {
            error(
                1 as libc::c_int,
                0 as libc::c_int,
                dcgettext(
                    0 as *const libc::c_char,
                    b"invalid suffix %s, contains directory separator\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
                quote(suffix),
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
                    b"invalid suffix %s, contains directory separator\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
                quote(suffix),
            );
            if 0 as libc::c_int != 0 {} else {
                unreachable!();
            };
        };
    }
    x_count = count_consecutive_X_s(
        template,
        suffix.offset_from(template) as libc::c_long as size_t,
    );
    if x_count < 3 as libc::c_int as libc::c_ulong {
        if ::core::mem::size_of::<C2RustUnnamed_3>() as libc::c_ulong != 0 {
            error(
                1 as libc::c_int,
                0 as libc::c_int,
                dcgettext(
                    0 as *const libc::c_char,
                    b"too few X's in template %s\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
                quote(template),
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
                    b"too few X's in template %s\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
                quote(template),
            );
            if 0 as libc::c_int != 0 {} else {
                unreachable!();
            };
        };
    }
    if use_dest_dir {
        if deprecated_t_option {
            let mut env: *mut libc::c_char = getenv(
                b"TMPDIR\0" as *const u8 as *const libc::c_char,
            );
            if !env.is_null() && *env as libc::c_int != 0 {
                dest_dir = env;
            } else if !dest_dir_arg.is_null() && *dest_dir_arg as libc::c_int != 0 {
                dest_dir = dest_dir_arg;
            } else {
                dest_dir = b"/tmp\0" as *const u8 as *const libc::c_char;
            }
            if last_component(template) != template {
                if ::core::mem::size_of::<C2RustUnnamed_2>() as libc::c_ulong != 0 {
                    error(
                        1 as libc::c_int,
                        0 as libc::c_int,
                        dcgettext(
                            0 as *const libc::c_char,
                            b"invalid template, %s, contains directory separator\0"
                                as *const u8 as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                        quote(template),
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
                            b"invalid template, %s, contains directory separator\0"
                                as *const u8 as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                        quote(template),
                    );
                    if 0 as libc::c_int != 0 {} else {
                        unreachable!();
                    };
                };
            }
        } else {
            if !dest_dir_arg.is_null() && *dest_dir_arg as libc::c_int != 0 {
                dest_dir = dest_dir_arg;
            } else {
                let mut env_0: *mut libc::c_char = getenv(
                    b"TMPDIR\0" as *const u8 as *const libc::c_char,
                );
                dest_dir = if !env_0.is_null() && *env_0 as libc::c_int != 0 {
                    env_0 as *const libc::c_char
                } else {
                    b"/tmp\0" as *const u8 as *const libc::c_char
                };
            }
            if *template.offset(0 as libc::c_int as isize) as libc::c_int == '/' as i32 {
                if ::core::mem::size_of::<C2RustUnnamed_1>() as libc::c_ulong != 0 {
                    error(
                        1 as libc::c_int,
                        0 as libc::c_int,
                        dcgettext(
                            0 as *const libc::c_char,
                            b"invalid template, %s; with --tmpdir, it may not be absolute\0"
                                as *const u8 as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                        quote(template),
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
                            b"invalid template, %s; with --tmpdir, it may not be absolute\0"
                                as *const u8 as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                        quote(template),
                    );
                    if 0 as libc::c_int != 0 {} else {
                        unreachable!();
                    };
                };
            }
        }
        dest_name = file_name_concat(dest_dir, template, 0 as *mut *mut libc::c_char);
        free(template as *mut libc::c_void);
        template = dest_name;
    }
    dest_name = xstrdup(template);
    if create_directory {
        let mut err: libc::c_int = mkdtemp_len(dest_name, suffix_len, x_count, dry_run);
        if err != 0 as libc::c_int {
            if !suppress_file_err {
                error(
                    0 as libc::c_int,
                    *__errno_location(),
                    dcgettext(
                        0 as *const libc::c_char,
                        b"failed to create directory via template %s\0" as *const u8
                            as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    quote(template),
                );
            }
            status = 1 as libc::c_int;
        }
    } else {
        let mut fd: libc::c_int = mkstemp_len(dest_name, suffix_len, x_count, dry_run);
        if fd < 0 as libc::c_int || !dry_run && close(fd) != 0 as libc::c_int {
            if !suppress_file_err {
                error(
                    0 as libc::c_int,
                    *__errno_location(),
                    dcgettext(
                        0 as *const libc::c_char,
                        b"failed to create file via template %s\0" as *const u8
                            as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    quote(template),
                );
            }
            status = 1 as libc::c_int;
        }
    }
    if status == 0 as libc::c_int {
        puts(dest_name);
        if !dry_run
            && {
                stdout_closed = 1 as libc::c_int != 0;
                close_stream(stdout) != 0 as libc::c_int
            }
        {
            let mut saved_errno: libc::c_int = *__errno_location();
            remove(dest_name);
            if !suppress_file_err {
                error(
                    0 as libc::c_int,
                    saved_errno,
                    dcgettext(
                        0 as *const libc::c_char,
                        b"write error\0" as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                );
            }
            status = 1 as libc::c_int;
        }
    }
    exit(status);
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
