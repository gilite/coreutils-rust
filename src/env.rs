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
    static mut optarg: *mut libc::c_char;
    static mut optind: libc::c_int;
    fn getopt_long(
        ___argc: libc::c_int,
        ___argv: *const *mut libc::c_char,
        __shortopts: *const libc::c_char,
        __longopts: *const option,
        __longind: *mut libc::c_int,
    ) -> libc::c_int;
    fn sigemptyset(__set: *mut sigset_t) -> libc::c_int;
    fn sigfillset(__set: *mut sigset_t) -> libc::c_int;
    fn sigaddset(__set: *mut sigset_t, __signo: libc::c_int) -> libc::c_int;
    fn sigdelset(__set: *mut sigset_t, __signo: libc::c_int) -> libc::c_int;
    fn sigismember(__set: *const sigset_t, __signo: libc::c_int) -> libc::c_int;
    fn sigprocmask(
        __how: libc::c_int,
        __set: *const sigset_t,
        __oset: *mut sigset_t,
    ) -> libc::c_int;
    fn sigaction(
        __sig: libc::c_int,
        __act: *const sigaction,
        __oact: *mut sigaction,
    ) -> libc::c_int;
    fn chdir(__path: *const libc::c_char) -> libc::c_int;
    static mut environ: *mut *mut libc::c_char;
    fn execvp(
        __file: *const libc::c_char,
        __argv: *const *mut libc::c_char,
    ) -> libc::c_int;
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
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strspn(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_ulong;
    fn strpbrk(_: *const libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strtok(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn free(_: *mut libc::c_void);
    fn __errno_location() -> *mut libc::c_int;
    fn atexit(__func: Option::<unsafe extern "C" fn() -> ()>) -> libc::c_int;
    fn exit(_: libc::c_int) -> !;
    fn getenv(__name: *const libc::c_char) -> *mut libc::c_char;
    fn putenv(__string: *mut libc::c_char) -> libc::c_int;
    fn unsetenv(__name: *const libc::c_char) -> libc::c_int;
    static mut Version: *const libc::c_char;
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
    fn xrealloc(p: *mut libc::c_void, s: size_t) -> *mut libc::c_void;
    fn x2nrealloc(p: *mut libc::c_void, pn: *mut size_t, s: size_t) -> *mut libc::c_void;
    fn xpalloc(
        pa: *mut libc::c_void,
        pn: *mut idx_t,
        n_incr_min: idx_t,
        n_max: ptrdiff_t,
        s: idx_t,
    ) -> *mut libc::c_void;
    fn xstrdup(str: *const libc::c_char) -> *mut libc::c_char;
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
    fn quotearg_style(s: quoting_style, arg: *const libc::c_char) -> *mut libc::c_char;
    fn error(
        __status: libc::c_int,
        __errnum: libc::c_int,
        __format: *const libc::c_char,
        _: ...
    );
    fn operand2sig(
        operand: *const libc::c_char,
        signame: *mut libc::c_char,
    ) -> libc::c_int;
    fn quote(arg: *const libc::c_char) -> *const libc::c_char;
    fn sig2str(_: libc::c_int, _: *mut libc::c_char) -> libc::c_int;
}
pub type size_t = libc::c_ulong;
pub type __uint32_t = libc::c_uint;
pub type __uid_t = libc::c_uint;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __pid_t = libc::c_int;
pub type __clock_t = libc::c_long;
pub type __intptr_t = libc::c_long;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __sigset_t {
    pub __val: [libc::c_ulong; 16],
}
pub type sigset_t = __sigset_t;
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
pub union sigval {
    pub sival_int: libc::c_int,
    pub sival_ptr: *mut libc::c_void,
}
pub type __sigval_t = sigval;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct siginfo_t {
    pub si_signo: libc::c_int,
    pub si_errno: libc::c_int,
    pub si_code: libc::c_int,
    pub __pad0: libc::c_int,
    pub _sifields: C2RustUnnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub _pad: [libc::c_int; 28],
    pub _kill: C2RustUnnamed_8,
    pub _timer: C2RustUnnamed_7,
    pub _rt: C2RustUnnamed_6,
    pub _sigchld: C2RustUnnamed_5,
    pub _sigfault: C2RustUnnamed_2,
    pub _sigpoll: C2RustUnnamed_1,
    pub _sigsys: C2RustUnnamed_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_0 {
    pub _call_addr: *mut libc::c_void,
    pub _syscall: libc::c_int,
    pub _arch: libc::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_1 {
    pub si_band: libc::c_long,
    pub si_fd: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_2 {
    pub si_addr: *mut libc::c_void,
    pub si_addr_lsb: libc::c_short,
    pub _bounds: C2RustUnnamed_3,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_3 {
    pub _addr_bnd: C2RustUnnamed_4,
    pub _pkey: __uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_4 {
    pub _lower: *mut libc::c_void,
    pub _upper: *mut libc::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_5 {
    pub si_pid: __pid_t,
    pub si_uid: __uid_t,
    pub si_status: libc::c_int,
    pub si_utime: __clock_t,
    pub si_stime: __clock_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_6 {
    pub si_pid: __pid_t,
    pub si_uid: __uid_t,
    pub si_sigval: __sigval_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_7 {
    pub si_tid: libc::c_int,
    pub si_overrun: libc::c_int,
    pub si_sigval: __sigval_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_8 {
    pub si_pid: __pid_t,
    pub si_uid: __uid_t,
}
pub type __sighandler_t = Option::<unsafe extern "C" fn(libc::c_int) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sigaction {
    pub __sigaction_handler: C2RustUnnamed_9,
    pub sa_mask: __sigset_t,
    pub sa_flags: libc::c_int,
    pub sa_restorer: Option::<unsafe extern "C" fn() -> ()>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_9 {
    pub sa_handler: __sighandler_t,
    pub sa_sigaction: Option::<
        unsafe extern "C" fn(libc::c_int, *mut siginfo_t, *mut libc::c_void) -> (),
    >,
}
pub type intptr_t = __intptr_t;
pub type C2RustUnnamed_10 = libc::c_uint;
pub const EXIT_ENOENT: C2RustUnnamed_10 = 127;
pub const EXIT_CANNOT_INVOKE: C2RustUnnamed_10 = 126;
pub const EXIT_CANCELED: C2RustUnnamed_10 = 125;
pub const EXIT_TIMEDOUT: C2RustUnnamed_10 = 124;
pub type idx_t = ptrdiff_t;
pub type C2RustUnnamed_11 = libc::c_int;
pub const GETOPT_VERSION_CHAR: C2RustUnnamed_11 = -3;
pub const GETOPT_HELP_CHAR: C2RustUnnamed_11 = -2;
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
pub type SIGNAL_MODE = libc::c_uint;
pub const IGNORE_NOERR: SIGNAL_MODE = 4;
pub const IGNORE: SIGNAL_MODE = 3;
pub const DEFAULT_NOERR: SIGNAL_MODE = 2;
pub const DEFAULT: SIGNAL_MODE = 1;
pub const UNCHANGED: SIGNAL_MODE = 0;
pub type C2RustUnnamed_12 = libc::c_uint;
pub const LIST_SIGNAL_HANDLING_OPTION: C2RustUnnamed_12 = 259;
pub const BLOCK_SIGNAL_OPTION: C2RustUnnamed_12 = 258;
pub const IGNORE_SIGNAL_OPTION: C2RustUnnamed_12 = 257;
pub const DEFAULT_SIGNAL_OPTION: C2RustUnnamed_12 = 256;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_13 {
    pub _gl_dummy: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct splitbuf {
    pub argv: *mut *mut libc::c_char,
    pub argc: libc::c_int,
    pub half_alloc: idx_t,
    pub extra_argc: libc::c_int,
    pub sep: bool,
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_20 {
    pub _gl_dummy: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_21 {
    pub _gl_dummy: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_22 {
    pub _gl_dummy: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_23 {
    pub _gl_dummy: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_24 {
    pub _gl_dummy: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_25 {
    pub _gl_dummy: libc::c_int,
}
#[inline]
unsafe extern "C" fn c_isalnum(mut c: libc::c_int) -> bool {
    match c {
        48 | 49 | 50 | 51 | 52 | 53 | 54 | 55 | 56 | 57 | 97 | 98 | 99 | 100 | 101 | 102
        | 103 | 104 | 105 | 106 | 107 | 108 | 109 | 110 | 111 | 112 | 113 | 114 | 115
        | 116 | 117 | 118 | 119 | 120 | 121 | 122 | 65 | 66 | 67 | 68 | 69 | 70 | 71 | 72
        | 73 | 74 | 75 | 76 | 77 | 78 | 79 | 80 | 81 | 82 | 83 | 84 | 85 | 86 | 87 | 88
        | 89 | 90 => return 1 as libc::c_int != 0,
        _ => return 0 as libc::c_int != 0,
    };
}
#[inline]
unsafe extern "C" fn c_isalpha(mut c: libc::c_int) -> bool {
    match c {
        97 | 98 | 99 | 100 | 101 | 102 | 103 | 104 | 105 | 106 | 107 | 108 | 109 | 110
        | 111 | 112 | 113 | 114 | 115 | 116 | 117 | 118 | 119 | 120 | 121 | 122 | 65 | 66
        | 67 | 68 | 69 | 70 | 71 | 72 | 73 | 74 | 75 | 76 | 77 | 78 | 79 | 80 | 81 | 82
        | 83 | 84 | 85 | 86 | 87 | 88 | 89 | 90 => return 1 as libc::c_int != 0,
        _ => return 0 as libc::c_int != 0,
    };
}
#[inline]
unsafe extern "C" fn initialize_exit_failure(mut status: libc::c_int) {
    if status != 1 as libc::c_int {
        ::core::ptr::write_volatile(&mut exit_failure as *mut libc::c_int, status);
    }
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
static mut usvars: *mut *const libc::c_char = 0 as *const *const libc::c_char
    as *mut *const libc::c_char;
static mut usvars_alloc: size_t = 0;
static mut usvars_used: idx_t = 0;
static mut dev_debug: bool = false;
static mut varname: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
static mut vnlen: idx_t = 0;
static mut signals: *mut SIGNAL_MODE = 0 as *const SIGNAL_MODE as *mut SIGNAL_MODE;
static mut block_signals: sigset_t = sigset_t { __val: [0; 16] };
static mut unblock_signals: sigset_t = sigset_t { __val: [0; 16] };
static mut sig_mask_changed: bool = false;
static mut report_signal_handling: bool = false;
static mut shortopts: [libc::c_char; 17] = unsafe {
    *::core::mem::transmute::<
        &[u8; 17],
        &[libc::c_char; 17],
    >(b"+C:iS:u:v0 \t\n\x0B\x0C\r\0")
};
static mut longopts: [option; 13] = [
    {
        let mut init = option {
            name: b"ignore-environment\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'i' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"null\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: '0' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"unset\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'u' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"chdir\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'C' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"default-signal\0" as *const u8 as *const libc::c_char,
            has_arg: 2 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: DEFAULT_SIGNAL_OPTION as libc::c_int,
        };
        init
    },
    {
        let mut init = option {
            name: b"ignore-signal\0" as *const u8 as *const libc::c_char,
            has_arg: 2 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: IGNORE_SIGNAL_OPTION as libc::c_int,
        };
        init
    },
    {
        let mut init = option {
            name: b"block-signal\0" as *const u8 as *const libc::c_char,
            has_arg: 2 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: BLOCK_SIGNAL_OPTION as libc::c_int,
        };
        init
    },
    {
        let mut init = option {
            name: b"list-signal-handling\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: LIST_SIGNAL_HANDLING_OPTION as libc::c_int,
        };
        init
    },
    {
        let mut init = option {
            name: b"debug\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'v' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"split-string\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'S' as i32,
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
                b"Usage: %s [OPTION]... [-] [NAME=VALUE]... [COMMAND [ARG]...]\n\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            program_name,
        );
        fputs_unlocked(
            dcgettext(
                0 as *const libc::c_char,
                b"Set each NAME to VALUE in the environment and run COMMAND.\n\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            stdout,
        );
        emit_mandatory_arg_note();
        fputs_unlocked(
            dcgettext(
                0 as *const libc::c_char,
                b"  -i, --ignore-environment  start with an empty environment\n  -0, --null           end each output line with NUL, not newline\n  -u, --unset=NAME     remove variable from the environment\n\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            stdout,
        );
        fputs_unlocked(
            dcgettext(
                0 as *const libc::c_char,
                b"  -C, --chdir=DIR      change working directory to DIR\n\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            stdout,
        );
        fputs_unlocked(
            dcgettext(
                0 as *const libc::c_char,
                b"  -S, --split-string=S  process and split S into separate arguments;\n                        used to pass multiple arguments on shebang lines\n\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            stdout,
        );
        fputs_unlocked(
            dcgettext(
                0 as *const libc::c_char,
                b"      --block-signal[=SIG]    block delivery of SIG signal(s) to COMMAND\n\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            stdout,
        );
        fputs_unlocked(
            dcgettext(
                0 as *const libc::c_char,
                b"      --default-signal[=SIG]  reset handling of SIG signal(s) to the default\n\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            stdout,
        );
        fputs_unlocked(
            dcgettext(
                0 as *const libc::c_char,
                b"      --ignore-signal[=SIG]   set handling of SIG signal(s) to do nothing\n\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            stdout,
        );
        fputs_unlocked(
            dcgettext(
                0 as *const libc::c_char,
                b"      --list-signal-handling  list non default signal handling to stderr\n\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            stdout,
        );
        fputs_unlocked(
            dcgettext(
                0 as *const libc::c_char,
                b"  -v, --debug          print verbose information for each processing step\n\0"
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
                b"\nA mere - implies -i.  If no COMMAND, print the resulting environment.\n\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            stdout,
        );
        fputs_unlocked(
            dcgettext(
                0 as *const libc::c_char,
                b"\nSIG may be a signal name like 'PIPE', or a signal number like '13'.\nWithout SIG, all known signals are included.  Multiple signals can be\ncomma-separated.  An empty SIG argument is a no-op.\n\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            stdout,
        );
        emit_exec_status(b"env\0" as *const u8 as *const libc::c_char);
        emit_ancillary_info(b"env\0" as *const u8 as *const libc::c_char);
    }
    exit(status);
}
unsafe extern "C" fn append_unset_var(mut var: *const libc::c_char) {
    if usvars_used as libc::c_ulong == usvars_alloc {
        usvars = x2nrealloc(
            usvars as *mut libc::c_void,
            &mut usvars_alloc,
            ::core::mem::size_of::<*const libc::c_char>() as libc::c_ulong,
        ) as *mut *const libc::c_char;
    }
    let fresh0 = usvars_used;
    usvars_used = usvars_used + 1;
    let ref mut fresh1 = *usvars.offset(fresh0 as isize);
    *fresh1 = var;
}
unsafe extern "C" fn unset_envvars() {
    let mut i: idx_t = 0 as libc::c_int as idx_t;
    while i < usvars_used {
        if dev_debug {
            fprintf(
                stderr,
                b"unset:    %s\n\0" as *const u8 as *const libc::c_char,
                *usvars.offset(i as isize),
            );
        }
        if unsetenv(*usvars.offset(i as isize)) != 0 {
            if ::core::mem::size_of::<C2RustUnnamed_13>() as libc::c_ulong != 0 {
                error(
                    EXIT_CANCELED as libc::c_int,
                    *__errno_location(),
                    dcgettext(
                        0 as *const libc::c_char,
                        b"cannot unset %s\0" as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    quote(*usvars.offset(i as isize)),
                );
                if 0 as libc::c_int != 0 {} else {
                    unreachable!();
                };
            } else {
                error(
                    EXIT_CANCELED as libc::c_int,
                    *__errno_location(),
                    dcgettext(
                        0 as *const libc::c_char,
                        b"cannot unset %s\0" as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    quote(*usvars.offset(i as isize)),
                );
                if 0 as libc::c_int != 0 {} else {
                    unreachable!();
                };
            };
        }
        i += 1;
    }
}
unsafe extern "C" fn scan_varname(mut str: *const libc::c_char) -> *const libc::c_char {
    if *str.offset(1 as libc::c_int as isize) as libc::c_int == '{' as i32
        && (c_isalpha(*str.offset(2 as libc::c_int as isize) as libc::c_int)
            as libc::c_int != 0
            || *str.offset(2 as libc::c_int as isize) as libc::c_int == '_' as i32)
    {
        let mut end: *const libc::c_char = str.offset(3 as libc::c_int as isize);
        while c_isalnum(*end as libc::c_int) as libc::c_int != 0
            || *end as libc::c_int == '_' as i32
        {
            end = end.offset(1);
        }
        if *end as libc::c_int == '}' as i32 {
            return end;
        }
    }
    return 0 as *const libc::c_char;
}
unsafe extern "C" fn extract_varname(mut str: *const libc::c_char) -> *mut libc::c_char {
    let mut i: idx_t = 0;
    let mut p: *const libc::c_char = 0 as *const libc::c_char;
    p = scan_varname(str);
    if p.is_null() {
        return 0 as *mut libc::c_char;
    }
    i = p.offset_from(str) as libc::c_long - 2 as libc::c_int as libc::c_long;
    if i >= vnlen {
        vnlen = i + 1 as libc::c_int as libc::c_long;
        varname = xrealloc(varname as *mut libc::c_void, vnlen as size_t)
            as *mut libc::c_char;
    }
    memcpy(
        varname as *mut libc::c_void,
        str.offset(2 as libc::c_int as isize) as *const libc::c_void,
        i as libc::c_ulong,
    );
    *varname.offset(i as isize) = 0 as libc::c_int as libc::c_char;
    return varname;
}
unsafe extern "C" fn splitbuf_grow(mut ss: *mut splitbuf) {
    let mut old_half_alloc: idx_t = (*ss).half_alloc;
    let mut string_bytes: idx_t = *((*ss).argv).offset((*ss).argc as isize) as intptr_t;
    (*ss)
        .argv = xpalloc(
        (*ss).argv as *mut libc::c_void,
        &mut (*ss).half_alloc,
        1 as libc::c_int as idx_t,
        if (2147483647 as libc::c_int as libc::c_long)
            < 9223372036854775807 as libc::c_long
        {
            2147483647 as libc::c_int as libc::c_long
        } else {
            9223372036854775807 as libc::c_long
        },
        (2 as libc::c_int as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<*mut libc::c_char>() as libc::c_ulong)
            as idx_t,
    ) as *mut *mut libc::c_char;
    memmove(
        ((*ss).argv).offset((*ss).half_alloc as isize) as *mut libc::c_void,
        ((*ss).argv).offset(old_half_alloc as isize) as *const libc::c_void,
        string_bytes as libc::c_ulong,
    );
}
unsafe extern "C" fn splitbuf_append_byte(mut ss: *mut splitbuf, mut c: libc::c_char) {
    let mut string_bytes: idx_t = *((*ss).argv).offset((*ss).argc as isize) as intptr_t;
    if ((*ss).half_alloc as libc::c_ulong)
        .wrapping_mul(::core::mem::size_of::<*mut libc::c_char>() as libc::c_ulong)
        <= string_bytes as libc::c_ulong
    {
        splitbuf_grow(ss);
    }
    *(((*ss).argv).offset((*ss).half_alloc as isize) as *mut libc::c_char)
        .offset(string_bytes as isize) = c;
    let ref mut fresh2 = *((*ss).argv).offset((*ss).argc as isize);
    *fresh2 = (string_bytes + 1 as libc::c_int as libc::c_long) as *mut libc::c_char;
}
unsafe extern "C" fn check_start_new_arg(mut ss: *mut splitbuf) {
    if (*ss).sep {
        splitbuf_append_byte(ss, '\0' as i32 as libc::c_char);
        let mut argc: libc::c_int = (*ss).argc;
        if (*ss).half_alloc
            <= (argc + (*ss).extra_argc + 1 as libc::c_int) as libc::c_long
        {
            splitbuf_grow(ss);
        }
        let ref mut fresh3 = *((*ss).argv).offset((argc + 1 as libc::c_int) as isize);
        *fresh3 = *((*ss).argv).offset(argc as isize);
        (*ss).argc = argc + 1 as libc::c_int;
        (*ss).sep = 0 as libc::c_int != 0;
    }
}
unsafe extern "C" fn splitbuf_finishup(mut ss: *mut splitbuf) -> *mut *mut libc::c_char {
    let mut argc: libc::c_int = (*ss).argc;
    let mut argv: *mut *mut libc::c_char = (*ss).argv;
    let mut stringbase: *mut libc::c_char = ((*ss).argv)
        .offset((*ss).half_alloc as isize) as *mut libc::c_char;
    let mut i: libc::c_int = 1 as libc::c_int;
    while i < argc {
        let ref mut fresh4 = *argv.offset(i as isize);
        *fresh4 = stringbase.offset(*argv.offset(i as isize) as intptr_t as isize);
        i += 1;
    }
    return argv;
}
unsafe extern "C" fn build_argv(
    mut str: *const libc::c_char,
    mut extra_argc: libc::c_int,
    mut argc: *mut libc::c_int,
) -> *mut *mut libc::c_char {
    let mut current_block: u64;
    let mut dq: bool = 0 as libc::c_int != 0;
    let mut sq: bool = 0 as libc::c_int != 0;
    let mut ss: splitbuf = splitbuf {
        argv: 0 as *mut *mut libc::c_char,
        argc: 0,
        half_alloc: 0,
        extra_argc: 0,
        sep: false,
    };
    ss
        .argv = xnmalloc(
        (extra_argc + 2 as libc::c_int) as size_t,
        (2 as libc::c_int as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<*mut libc::c_char>() as libc::c_ulong),
    ) as *mut *mut libc::c_char;
    ss.argc = 1 as libc::c_int;
    ss.half_alloc = (extra_argc + 2 as libc::c_int) as idx_t;
    ss.extra_argc = extra_argc;
    ss.sep = 1 as libc::c_int != 0;
    let ref mut fresh5 = *(ss.argv).offset(ss.argc as isize);
    *fresh5 = 0 as *mut libc::c_char;
    loop {
        if !(*str != 0) {
            current_block = 8835654301469918283;
            break;
        }
        let mut newc: libc::c_char = *str;
        match *str as libc::c_int {
            39 => {
                if !dq {
                    sq = !sq;
                    check_start_new_arg(&mut ss);
                    str = str.offset(1);
                    continue;
                }
            }
            34 => {
                if !sq {
                    dq = !dq;
                    check_start_new_arg(&mut ss);
                    str = str.offset(1);
                    continue;
                }
            }
            32 | 9 | 10 | 11 | 12 | 13 => {
                if !(sq as libc::c_int != 0 || dq as libc::c_int != 0) {
                    ss.sep = 1 as libc::c_int != 0;
                    str = str
                        .offset(
                            strspn(
                                str,
                                b" \t\n\x0B\x0C\r\0" as *const u8 as *const libc::c_char,
                            ) as isize,
                        );
                    continue;
                }
            }
            35 => {
                if ss.sep {
                    current_block = 9101223997307740060;
                    break;
                }
            }
            92 => {
                if !(sq as libc::c_int != 0
                    && *str.offset(1 as libc::c_int as isize) as libc::c_int
                        != '\\' as i32
                    && *str.offset(1 as libc::c_int as isize) as libc::c_int
                        != '\'' as i32)
                {
                    str = str.offset(1);
                    newc = *str;
                    match newc as libc::c_int {
                        34 | 35 | 36 | 39 | 92 => {}
                        95 => {
                            current_block = 6049455476851897752;
                            match current_block {
                                6049455476851897752 => {
                                    if !dq {
                                        str = str.offset(1);
                                        ss.sep = 1 as libc::c_int != 0;
                                        continue;
                                    } else {
                                        newc = ' ' as i32 as libc::c_char;
                                    }
                                    current_block = 9241535491006583629;
                                }
                                15127182143015129983 => {
                                    if dq {
                                        if ::core::mem::size_of::<C2RustUnnamed_18>()
                                            as libc::c_ulong != 0
                                        {
                                            error(
                                                EXIT_CANCELED as libc::c_int,
                                                0 as libc::c_int,
                                                dcgettext(
                                                    0 as *const libc::c_char,
                                                    b"'\\c' must not appear in double-quoted -S string\0"
                                                        as *const u8 as *const libc::c_char,
                                                    5 as libc::c_int,
                                                ),
                                            );
                                            if 0 as libc::c_int != 0 {} else {
                                                unreachable!();
                                            };
                                        } else {
                                            error(
                                                EXIT_CANCELED as libc::c_int,
                                                0 as libc::c_int,
                                                dcgettext(
                                                    0 as *const libc::c_char,
                                                    b"'\\c' must not appear in double-quoted -S string\0"
                                                        as *const u8 as *const libc::c_char,
                                                    5 as libc::c_int,
                                                ),
                                            );
                                            if 0 as libc::c_int != 0 {} else {
                                                unreachable!();
                                            };
                                        };
                                    }
                                    current_block = 9101223997307740060;
                                    break;
                                }
                                17674171488089097684 => {
                                    newc = '\u{c}' as i32 as libc::c_char;
                                    current_block = 9241535491006583629;
                                }
                                2585176034642815015 => {
                                    newc = '\n' as i32 as libc::c_char;
                                    current_block = 9241535491006583629;
                                }
                                16206231157784423297 => {
                                    newc = '\r' as i32 as libc::c_char;
                                    current_block = 9241535491006583629;
                                }
                                2459324531873169266 => {
                                    newc = '\t' as i32 as libc::c_char;
                                    current_block = 9241535491006583629;
                                }
                                11717047739380720017 => {
                                    newc = '\u{b}' as i32 as libc::c_char;
                                    current_block = 9241535491006583629;
                                }
                                9304275656119448851 => {
                                    if ::core::mem::size_of::<C2RustUnnamed_17>()
                                        as libc::c_ulong != 0
                                    {
                                        error(
                                            EXIT_CANCELED as libc::c_int,
                                            0 as libc::c_int,
                                            dcgettext(
                                                0 as *const libc::c_char,
                                                b"invalid backslash at end of string in -S\0" as *const u8
                                                    as *const libc::c_char,
                                                5 as libc::c_int,
                                            ),
                                        );
                                        if 0 as libc::c_int != 0 {} else {
                                            unreachable!();
                                        };
                                    } else {
                                        error(
                                            EXIT_CANCELED as libc::c_int,
                                            0 as libc::c_int,
                                            dcgettext(
                                                0 as *const libc::c_char,
                                                b"invalid backslash at end of string in -S\0" as *const u8
                                                    as *const libc::c_char,
                                                5 as libc::c_int,
                                            ),
                                        );
                                        if 0 as libc::c_int != 0 {} else {
                                            unreachable!();
                                        };
                                    };
                                    current_block = 17014196479992252067;
                                }
                                _ => {}
                            }
                            match current_block {
                                9241535491006583629 => {}
                                _ => {
                                    if ::core::mem::size_of::<C2RustUnnamed_16>()
                                        as libc::c_ulong != 0
                                    {
                                        error(
                                            EXIT_CANCELED as libc::c_int,
                                            0 as libc::c_int,
                                            dcgettext(
                                                0 as *const libc::c_char,
                                                b"invalid sequence '\\%c' in -S\0" as *const u8
                                                    as *const libc::c_char,
                                                5 as libc::c_int,
                                            ),
                                            newc as libc::c_int,
                                        );
                                        if 0 as libc::c_int != 0 {} else {
                                            unreachable!();
                                        };
                                    } else {
                                        error(
                                            EXIT_CANCELED as libc::c_int,
                                            0 as libc::c_int,
                                            dcgettext(
                                                0 as *const libc::c_char,
                                                b"invalid sequence '\\%c' in -S\0" as *const u8
                                                    as *const libc::c_char,
                                                5 as libc::c_int,
                                            ),
                                            newc as libc::c_int,
                                        );
                                        if 0 as libc::c_int != 0 {} else {
                                            unreachable!();
                                        };
                                    };
                                }
                            }
                        }
                        99 => {
                            current_block = 15127182143015129983;
                            match current_block {
                                6049455476851897752 => {
                                    if !dq {
                                        str = str.offset(1);
                                        ss.sep = 1 as libc::c_int != 0;
                                        continue;
                                    } else {
                                        newc = ' ' as i32 as libc::c_char;
                                    }
                                    current_block = 9241535491006583629;
                                }
                                15127182143015129983 => {
                                    if dq {
                                        if ::core::mem::size_of::<C2RustUnnamed_18>()
                                            as libc::c_ulong != 0
                                        {
                                            error(
                                                EXIT_CANCELED as libc::c_int,
                                                0 as libc::c_int,
                                                dcgettext(
                                                    0 as *const libc::c_char,
                                                    b"'\\c' must not appear in double-quoted -S string\0"
                                                        as *const u8 as *const libc::c_char,
                                                    5 as libc::c_int,
                                                ),
                                            );
                                            if 0 as libc::c_int != 0 {} else {
                                                unreachable!();
                                            };
                                        } else {
                                            error(
                                                EXIT_CANCELED as libc::c_int,
                                                0 as libc::c_int,
                                                dcgettext(
                                                    0 as *const libc::c_char,
                                                    b"'\\c' must not appear in double-quoted -S string\0"
                                                        as *const u8 as *const libc::c_char,
                                                    5 as libc::c_int,
                                                ),
                                            );
                                            if 0 as libc::c_int != 0 {} else {
                                                unreachable!();
                                            };
                                        };
                                    }
                                    current_block = 9101223997307740060;
                                    break;
                                }
                                17674171488089097684 => {
                                    newc = '\u{c}' as i32 as libc::c_char;
                                    current_block = 9241535491006583629;
                                }
                                2585176034642815015 => {
                                    newc = '\n' as i32 as libc::c_char;
                                    current_block = 9241535491006583629;
                                }
                                16206231157784423297 => {
                                    newc = '\r' as i32 as libc::c_char;
                                    current_block = 9241535491006583629;
                                }
                                2459324531873169266 => {
                                    newc = '\t' as i32 as libc::c_char;
                                    current_block = 9241535491006583629;
                                }
                                11717047739380720017 => {
                                    newc = '\u{b}' as i32 as libc::c_char;
                                    current_block = 9241535491006583629;
                                }
                                9304275656119448851 => {
                                    if ::core::mem::size_of::<C2RustUnnamed_17>()
                                        as libc::c_ulong != 0
                                    {
                                        error(
                                            EXIT_CANCELED as libc::c_int,
                                            0 as libc::c_int,
                                            dcgettext(
                                                0 as *const libc::c_char,
                                                b"invalid backslash at end of string in -S\0" as *const u8
                                                    as *const libc::c_char,
                                                5 as libc::c_int,
                                            ),
                                        );
                                        if 0 as libc::c_int != 0 {} else {
                                            unreachable!();
                                        };
                                    } else {
                                        error(
                                            EXIT_CANCELED as libc::c_int,
                                            0 as libc::c_int,
                                            dcgettext(
                                                0 as *const libc::c_char,
                                                b"invalid backslash at end of string in -S\0" as *const u8
                                                    as *const libc::c_char,
                                                5 as libc::c_int,
                                            ),
                                        );
                                        if 0 as libc::c_int != 0 {} else {
                                            unreachable!();
                                        };
                                    };
                                    current_block = 17014196479992252067;
                                }
                                _ => {}
                            }
                            match current_block {
                                9241535491006583629 => {}
                                _ => {
                                    if ::core::mem::size_of::<C2RustUnnamed_16>()
                                        as libc::c_ulong != 0
                                    {
                                        error(
                                            EXIT_CANCELED as libc::c_int,
                                            0 as libc::c_int,
                                            dcgettext(
                                                0 as *const libc::c_char,
                                                b"invalid sequence '\\%c' in -S\0" as *const u8
                                                    as *const libc::c_char,
                                                5 as libc::c_int,
                                            ),
                                            newc as libc::c_int,
                                        );
                                        if 0 as libc::c_int != 0 {} else {
                                            unreachable!();
                                        };
                                    } else {
                                        error(
                                            EXIT_CANCELED as libc::c_int,
                                            0 as libc::c_int,
                                            dcgettext(
                                                0 as *const libc::c_char,
                                                b"invalid sequence '\\%c' in -S\0" as *const u8
                                                    as *const libc::c_char,
                                                5 as libc::c_int,
                                            ),
                                            newc as libc::c_int,
                                        );
                                        if 0 as libc::c_int != 0 {} else {
                                            unreachable!();
                                        };
                                    };
                                }
                            }
                        }
                        102 => {
                            current_block = 17674171488089097684;
                            match current_block {
                                6049455476851897752 => {
                                    if !dq {
                                        str = str.offset(1);
                                        ss.sep = 1 as libc::c_int != 0;
                                        continue;
                                    } else {
                                        newc = ' ' as i32 as libc::c_char;
                                    }
                                    current_block = 9241535491006583629;
                                }
                                15127182143015129983 => {
                                    if dq {
                                        if ::core::mem::size_of::<C2RustUnnamed_18>()
                                            as libc::c_ulong != 0
                                        {
                                            error(
                                                EXIT_CANCELED as libc::c_int,
                                                0 as libc::c_int,
                                                dcgettext(
                                                    0 as *const libc::c_char,
                                                    b"'\\c' must not appear in double-quoted -S string\0"
                                                        as *const u8 as *const libc::c_char,
                                                    5 as libc::c_int,
                                                ),
                                            );
                                            if 0 as libc::c_int != 0 {} else {
                                                unreachable!();
                                            };
                                        } else {
                                            error(
                                                EXIT_CANCELED as libc::c_int,
                                                0 as libc::c_int,
                                                dcgettext(
                                                    0 as *const libc::c_char,
                                                    b"'\\c' must not appear in double-quoted -S string\0"
                                                        as *const u8 as *const libc::c_char,
                                                    5 as libc::c_int,
                                                ),
                                            );
                                            if 0 as libc::c_int != 0 {} else {
                                                unreachable!();
                                            };
                                        };
                                    }
                                    current_block = 9101223997307740060;
                                    break;
                                }
                                17674171488089097684 => {
                                    newc = '\u{c}' as i32 as libc::c_char;
                                    current_block = 9241535491006583629;
                                }
                                2585176034642815015 => {
                                    newc = '\n' as i32 as libc::c_char;
                                    current_block = 9241535491006583629;
                                }
                                16206231157784423297 => {
                                    newc = '\r' as i32 as libc::c_char;
                                    current_block = 9241535491006583629;
                                }
                                2459324531873169266 => {
                                    newc = '\t' as i32 as libc::c_char;
                                    current_block = 9241535491006583629;
                                }
                                11717047739380720017 => {
                                    newc = '\u{b}' as i32 as libc::c_char;
                                    current_block = 9241535491006583629;
                                }
                                9304275656119448851 => {
                                    if ::core::mem::size_of::<C2RustUnnamed_17>()
                                        as libc::c_ulong != 0
                                    {
                                        error(
                                            EXIT_CANCELED as libc::c_int,
                                            0 as libc::c_int,
                                            dcgettext(
                                                0 as *const libc::c_char,
                                                b"invalid backslash at end of string in -S\0" as *const u8
                                                    as *const libc::c_char,
                                                5 as libc::c_int,
                                            ),
                                        );
                                        if 0 as libc::c_int != 0 {} else {
                                            unreachable!();
                                        };
                                    } else {
                                        error(
                                            EXIT_CANCELED as libc::c_int,
                                            0 as libc::c_int,
                                            dcgettext(
                                                0 as *const libc::c_char,
                                                b"invalid backslash at end of string in -S\0" as *const u8
                                                    as *const libc::c_char,
                                                5 as libc::c_int,
                                            ),
                                        );
                                        if 0 as libc::c_int != 0 {} else {
                                            unreachable!();
                                        };
                                    };
                                    current_block = 17014196479992252067;
                                }
                                _ => {}
                            }
                            match current_block {
                                9241535491006583629 => {}
                                _ => {
                                    if ::core::mem::size_of::<C2RustUnnamed_16>()
                                        as libc::c_ulong != 0
                                    {
                                        error(
                                            EXIT_CANCELED as libc::c_int,
                                            0 as libc::c_int,
                                            dcgettext(
                                                0 as *const libc::c_char,
                                                b"invalid sequence '\\%c' in -S\0" as *const u8
                                                    as *const libc::c_char,
                                                5 as libc::c_int,
                                            ),
                                            newc as libc::c_int,
                                        );
                                        if 0 as libc::c_int != 0 {} else {
                                            unreachable!();
                                        };
                                    } else {
                                        error(
                                            EXIT_CANCELED as libc::c_int,
                                            0 as libc::c_int,
                                            dcgettext(
                                                0 as *const libc::c_char,
                                                b"invalid sequence '\\%c' in -S\0" as *const u8
                                                    as *const libc::c_char,
                                                5 as libc::c_int,
                                            ),
                                            newc as libc::c_int,
                                        );
                                        if 0 as libc::c_int != 0 {} else {
                                            unreachable!();
                                        };
                                    };
                                }
                            }
                        }
                        110 => {
                            current_block = 2585176034642815015;
                            match current_block {
                                6049455476851897752 => {
                                    if !dq {
                                        str = str.offset(1);
                                        ss.sep = 1 as libc::c_int != 0;
                                        continue;
                                    } else {
                                        newc = ' ' as i32 as libc::c_char;
                                    }
                                    current_block = 9241535491006583629;
                                }
                                15127182143015129983 => {
                                    if dq {
                                        if ::core::mem::size_of::<C2RustUnnamed_18>()
                                            as libc::c_ulong != 0
                                        {
                                            error(
                                                EXIT_CANCELED as libc::c_int,
                                                0 as libc::c_int,
                                                dcgettext(
                                                    0 as *const libc::c_char,
                                                    b"'\\c' must not appear in double-quoted -S string\0"
                                                        as *const u8 as *const libc::c_char,
                                                    5 as libc::c_int,
                                                ),
                                            );
                                            if 0 as libc::c_int != 0 {} else {
                                                unreachable!();
                                            };
                                        } else {
                                            error(
                                                EXIT_CANCELED as libc::c_int,
                                                0 as libc::c_int,
                                                dcgettext(
                                                    0 as *const libc::c_char,
                                                    b"'\\c' must not appear in double-quoted -S string\0"
                                                        as *const u8 as *const libc::c_char,
                                                    5 as libc::c_int,
                                                ),
                                            );
                                            if 0 as libc::c_int != 0 {} else {
                                                unreachable!();
                                            };
                                        };
                                    }
                                    current_block = 9101223997307740060;
                                    break;
                                }
                                17674171488089097684 => {
                                    newc = '\u{c}' as i32 as libc::c_char;
                                    current_block = 9241535491006583629;
                                }
                                2585176034642815015 => {
                                    newc = '\n' as i32 as libc::c_char;
                                    current_block = 9241535491006583629;
                                }
                                16206231157784423297 => {
                                    newc = '\r' as i32 as libc::c_char;
                                    current_block = 9241535491006583629;
                                }
                                2459324531873169266 => {
                                    newc = '\t' as i32 as libc::c_char;
                                    current_block = 9241535491006583629;
                                }
                                11717047739380720017 => {
                                    newc = '\u{b}' as i32 as libc::c_char;
                                    current_block = 9241535491006583629;
                                }
                                9304275656119448851 => {
                                    if ::core::mem::size_of::<C2RustUnnamed_17>()
                                        as libc::c_ulong != 0
                                    {
                                        error(
                                            EXIT_CANCELED as libc::c_int,
                                            0 as libc::c_int,
                                            dcgettext(
                                                0 as *const libc::c_char,
                                                b"invalid backslash at end of string in -S\0" as *const u8
                                                    as *const libc::c_char,
                                                5 as libc::c_int,
                                            ),
                                        );
                                        if 0 as libc::c_int != 0 {} else {
                                            unreachable!();
                                        };
                                    } else {
                                        error(
                                            EXIT_CANCELED as libc::c_int,
                                            0 as libc::c_int,
                                            dcgettext(
                                                0 as *const libc::c_char,
                                                b"invalid backslash at end of string in -S\0" as *const u8
                                                    as *const libc::c_char,
                                                5 as libc::c_int,
                                            ),
                                        );
                                        if 0 as libc::c_int != 0 {} else {
                                            unreachable!();
                                        };
                                    };
                                    current_block = 17014196479992252067;
                                }
                                _ => {}
                            }
                            match current_block {
                                9241535491006583629 => {}
                                _ => {
                                    if ::core::mem::size_of::<C2RustUnnamed_16>()
                                        as libc::c_ulong != 0
                                    {
                                        error(
                                            EXIT_CANCELED as libc::c_int,
                                            0 as libc::c_int,
                                            dcgettext(
                                                0 as *const libc::c_char,
                                                b"invalid sequence '\\%c' in -S\0" as *const u8
                                                    as *const libc::c_char,
                                                5 as libc::c_int,
                                            ),
                                            newc as libc::c_int,
                                        );
                                        if 0 as libc::c_int != 0 {} else {
                                            unreachable!();
                                        };
                                    } else {
                                        error(
                                            EXIT_CANCELED as libc::c_int,
                                            0 as libc::c_int,
                                            dcgettext(
                                                0 as *const libc::c_char,
                                                b"invalid sequence '\\%c' in -S\0" as *const u8
                                                    as *const libc::c_char,
                                                5 as libc::c_int,
                                            ),
                                            newc as libc::c_int,
                                        );
                                        if 0 as libc::c_int != 0 {} else {
                                            unreachable!();
                                        };
                                    };
                                }
                            }
                        }
                        114 => {
                            current_block = 16206231157784423297;
                            match current_block {
                                6049455476851897752 => {
                                    if !dq {
                                        str = str.offset(1);
                                        ss.sep = 1 as libc::c_int != 0;
                                        continue;
                                    } else {
                                        newc = ' ' as i32 as libc::c_char;
                                    }
                                    current_block = 9241535491006583629;
                                }
                                15127182143015129983 => {
                                    if dq {
                                        if ::core::mem::size_of::<C2RustUnnamed_18>()
                                            as libc::c_ulong != 0
                                        {
                                            error(
                                                EXIT_CANCELED as libc::c_int,
                                                0 as libc::c_int,
                                                dcgettext(
                                                    0 as *const libc::c_char,
                                                    b"'\\c' must not appear in double-quoted -S string\0"
                                                        as *const u8 as *const libc::c_char,
                                                    5 as libc::c_int,
                                                ),
                                            );
                                            if 0 as libc::c_int != 0 {} else {
                                                unreachable!();
                                            };
                                        } else {
                                            error(
                                                EXIT_CANCELED as libc::c_int,
                                                0 as libc::c_int,
                                                dcgettext(
                                                    0 as *const libc::c_char,
                                                    b"'\\c' must not appear in double-quoted -S string\0"
                                                        as *const u8 as *const libc::c_char,
                                                    5 as libc::c_int,
                                                ),
                                            );
                                            if 0 as libc::c_int != 0 {} else {
                                                unreachable!();
                                            };
                                        };
                                    }
                                    current_block = 9101223997307740060;
                                    break;
                                }
                                17674171488089097684 => {
                                    newc = '\u{c}' as i32 as libc::c_char;
                                    current_block = 9241535491006583629;
                                }
                                2585176034642815015 => {
                                    newc = '\n' as i32 as libc::c_char;
                                    current_block = 9241535491006583629;
                                }
                                16206231157784423297 => {
                                    newc = '\r' as i32 as libc::c_char;
                                    current_block = 9241535491006583629;
                                }
                                2459324531873169266 => {
                                    newc = '\t' as i32 as libc::c_char;
                                    current_block = 9241535491006583629;
                                }
                                11717047739380720017 => {
                                    newc = '\u{b}' as i32 as libc::c_char;
                                    current_block = 9241535491006583629;
                                }
                                9304275656119448851 => {
                                    if ::core::mem::size_of::<C2RustUnnamed_17>()
                                        as libc::c_ulong != 0
                                    {
                                        error(
                                            EXIT_CANCELED as libc::c_int,
                                            0 as libc::c_int,
                                            dcgettext(
                                                0 as *const libc::c_char,
                                                b"invalid backslash at end of string in -S\0" as *const u8
                                                    as *const libc::c_char,
                                                5 as libc::c_int,
                                            ),
                                        );
                                        if 0 as libc::c_int != 0 {} else {
                                            unreachable!();
                                        };
                                    } else {
                                        error(
                                            EXIT_CANCELED as libc::c_int,
                                            0 as libc::c_int,
                                            dcgettext(
                                                0 as *const libc::c_char,
                                                b"invalid backslash at end of string in -S\0" as *const u8
                                                    as *const libc::c_char,
                                                5 as libc::c_int,
                                            ),
                                        );
                                        if 0 as libc::c_int != 0 {} else {
                                            unreachable!();
                                        };
                                    };
                                    current_block = 17014196479992252067;
                                }
                                _ => {}
                            }
                            match current_block {
                                9241535491006583629 => {}
                                _ => {
                                    if ::core::mem::size_of::<C2RustUnnamed_16>()
                                        as libc::c_ulong != 0
                                    {
                                        error(
                                            EXIT_CANCELED as libc::c_int,
                                            0 as libc::c_int,
                                            dcgettext(
                                                0 as *const libc::c_char,
                                                b"invalid sequence '\\%c' in -S\0" as *const u8
                                                    as *const libc::c_char,
                                                5 as libc::c_int,
                                            ),
                                            newc as libc::c_int,
                                        );
                                        if 0 as libc::c_int != 0 {} else {
                                            unreachable!();
                                        };
                                    } else {
                                        error(
                                            EXIT_CANCELED as libc::c_int,
                                            0 as libc::c_int,
                                            dcgettext(
                                                0 as *const libc::c_char,
                                                b"invalid sequence '\\%c' in -S\0" as *const u8
                                                    as *const libc::c_char,
                                                5 as libc::c_int,
                                            ),
                                            newc as libc::c_int,
                                        );
                                        if 0 as libc::c_int != 0 {} else {
                                            unreachable!();
                                        };
                                    };
                                }
                            }
                        }
                        116 => {
                            current_block = 2459324531873169266;
                            match current_block {
                                6049455476851897752 => {
                                    if !dq {
                                        str = str.offset(1);
                                        ss.sep = 1 as libc::c_int != 0;
                                        continue;
                                    } else {
                                        newc = ' ' as i32 as libc::c_char;
                                    }
                                    current_block = 9241535491006583629;
                                }
                                15127182143015129983 => {
                                    if dq {
                                        if ::core::mem::size_of::<C2RustUnnamed_18>()
                                            as libc::c_ulong != 0
                                        {
                                            error(
                                                EXIT_CANCELED as libc::c_int,
                                                0 as libc::c_int,
                                                dcgettext(
                                                    0 as *const libc::c_char,
                                                    b"'\\c' must not appear in double-quoted -S string\0"
                                                        as *const u8 as *const libc::c_char,
                                                    5 as libc::c_int,
                                                ),
                                            );
                                            if 0 as libc::c_int != 0 {} else {
                                                unreachable!();
                                            };
                                        } else {
                                            error(
                                                EXIT_CANCELED as libc::c_int,
                                                0 as libc::c_int,
                                                dcgettext(
                                                    0 as *const libc::c_char,
                                                    b"'\\c' must not appear in double-quoted -S string\0"
                                                        as *const u8 as *const libc::c_char,
                                                    5 as libc::c_int,
                                                ),
                                            );
                                            if 0 as libc::c_int != 0 {} else {
                                                unreachable!();
                                            };
                                        };
                                    }
                                    current_block = 9101223997307740060;
                                    break;
                                }
                                17674171488089097684 => {
                                    newc = '\u{c}' as i32 as libc::c_char;
                                    current_block = 9241535491006583629;
                                }
                                2585176034642815015 => {
                                    newc = '\n' as i32 as libc::c_char;
                                    current_block = 9241535491006583629;
                                }
                                16206231157784423297 => {
                                    newc = '\r' as i32 as libc::c_char;
                                    current_block = 9241535491006583629;
                                }
                                2459324531873169266 => {
                                    newc = '\t' as i32 as libc::c_char;
                                    current_block = 9241535491006583629;
                                }
                                11717047739380720017 => {
                                    newc = '\u{b}' as i32 as libc::c_char;
                                    current_block = 9241535491006583629;
                                }
                                9304275656119448851 => {
                                    if ::core::mem::size_of::<C2RustUnnamed_17>()
                                        as libc::c_ulong != 0
                                    {
                                        error(
                                            EXIT_CANCELED as libc::c_int,
                                            0 as libc::c_int,
                                            dcgettext(
                                                0 as *const libc::c_char,
                                                b"invalid backslash at end of string in -S\0" as *const u8
                                                    as *const libc::c_char,
                                                5 as libc::c_int,
                                            ),
                                        );
                                        if 0 as libc::c_int != 0 {} else {
                                            unreachable!();
                                        };
                                    } else {
                                        error(
                                            EXIT_CANCELED as libc::c_int,
                                            0 as libc::c_int,
                                            dcgettext(
                                                0 as *const libc::c_char,
                                                b"invalid backslash at end of string in -S\0" as *const u8
                                                    as *const libc::c_char,
                                                5 as libc::c_int,
                                            ),
                                        );
                                        if 0 as libc::c_int != 0 {} else {
                                            unreachable!();
                                        };
                                    };
                                    current_block = 17014196479992252067;
                                }
                                _ => {}
                            }
                            match current_block {
                                9241535491006583629 => {}
                                _ => {
                                    if ::core::mem::size_of::<C2RustUnnamed_16>()
                                        as libc::c_ulong != 0
                                    {
                                        error(
                                            EXIT_CANCELED as libc::c_int,
                                            0 as libc::c_int,
                                            dcgettext(
                                                0 as *const libc::c_char,
                                                b"invalid sequence '\\%c' in -S\0" as *const u8
                                                    as *const libc::c_char,
                                                5 as libc::c_int,
                                            ),
                                            newc as libc::c_int,
                                        );
                                        if 0 as libc::c_int != 0 {} else {
                                            unreachable!();
                                        };
                                    } else {
                                        error(
                                            EXIT_CANCELED as libc::c_int,
                                            0 as libc::c_int,
                                            dcgettext(
                                                0 as *const libc::c_char,
                                                b"invalid sequence '\\%c' in -S\0" as *const u8
                                                    as *const libc::c_char,
                                                5 as libc::c_int,
                                            ),
                                            newc as libc::c_int,
                                        );
                                        if 0 as libc::c_int != 0 {} else {
                                            unreachable!();
                                        };
                                    };
                                }
                            }
                        }
                        118 => {
                            current_block = 11717047739380720017;
                            match current_block {
                                6049455476851897752 => {
                                    if !dq {
                                        str = str.offset(1);
                                        ss.sep = 1 as libc::c_int != 0;
                                        continue;
                                    } else {
                                        newc = ' ' as i32 as libc::c_char;
                                    }
                                    current_block = 9241535491006583629;
                                }
                                15127182143015129983 => {
                                    if dq {
                                        if ::core::mem::size_of::<C2RustUnnamed_18>()
                                            as libc::c_ulong != 0
                                        {
                                            error(
                                                EXIT_CANCELED as libc::c_int,
                                                0 as libc::c_int,
                                                dcgettext(
                                                    0 as *const libc::c_char,
                                                    b"'\\c' must not appear in double-quoted -S string\0"
                                                        as *const u8 as *const libc::c_char,
                                                    5 as libc::c_int,
                                                ),
                                            );
                                            if 0 as libc::c_int != 0 {} else {
                                                unreachable!();
                                            };
                                        } else {
                                            error(
                                                EXIT_CANCELED as libc::c_int,
                                                0 as libc::c_int,
                                                dcgettext(
                                                    0 as *const libc::c_char,
                                                    b"'\\c' must not appear in double-quoted -S string\0"
                                                        as *const u8 as *const libc::c_char,
                                                    5 as libc::c_int,
                                                ),
                                            );
                                            if 0 as libc::c_int != 0 {} else {
                                                unreachable!();
                                            };
                                        };
                                    }
                                    current_block = 9101223997307740060;
                                    break;
                                }
                                17674171488089097684 => {
                                    newc = '\u{c}' as i32 as libc::c_char;
                                    current_block = 9241535491006583629;
                                }
                                2585176034642815015 => {
                                    newc = '\n' as i32 as libc::c_char;
                                    current_block = 9241535491006583629;
                                }
                                16206231157784423297 => {
                                    newc = '\r' as i32 as libc::c_char;
                                    current_block = 9241535491006583629;
                                }
                                2459324531873169266 => {
                                    newc = '\t' as i32 as libc::c_char;
                                    current_block = 9241535491006583629;
                                }
                                11717047739380720017 => {
                                    newc = '\u{b}' as i32 as libc::c_char;
                                    current_block = 9241535491006583629;
                                }
                                9304275656119448851 => {
                                    if ::core::mem::size_of::<C2RustUnnamed_17>()
                                        as libc::c_ulong != 0
                                    {
                                        error(
                                            EXIT_CANCELED as libc::c_int,
                                            0 as libc::c_int,
                                            dcgettext(
                                                0 as *const libc::c_char,
                                                b"invalid backslash at end of string in -S\0" as *const u8
                                                    as *const libc::c_char,
                                                5 as libc::c_int,
                                            ),
                                        );
                                        if 0 as libc::c_int != 0 {} else {
                                            unreachable!();
                                        };
                                    } else {
                                        error(
                                            EXIT_CANCELED as libc::c_int,
                                            0 as libc::c_int,
                                            dcgettext(
                                                0 as *const libc::c_char,
                                                b"invalid backslash at end of string in -S\0" as *const u8
                                                    as *const libc::c_char,
                                                5 as libc::c_int,
                                            ),
                                        );
                                        if 0 as libc::c_int != 0 {} else {
                                            unreachable!();
                                        };
                                    };
                                    current_block = 17014196479992252067;
                                }
                                _ => {}
                            }
                            match current_block {
                                9241535491006583629 => {}
                                _ => {
                                    if ::core::mem::size_of::<C2RustUnnamed_16>()
                                        as libc::c_ulong != 0
                                    {
                                        error(
                                            EXIT_CANCELED as libc::c_int,
                                            0 as libc::c_int,
                                            dcgettext(
                                                0 as *const libc::c_char,
                                                b"invalid sequence '\\%c' in -S\0" as *const u8
                                                    as *const libc::c_char,
                                                5 as libc::c_int,
                                            ),
                                            newc as libc::c_int,
                                        );
                                        if 0 as libc::c_int != 0 {} else {
                                            unreachable!();
                                        };
                                    } else {
                                        error(
                                            EXIT_CANCELED as libc::c_int,
                                            0 as libc::c_int,
                                            dcgettext(
                                                0 as *const libc::c_char,
                                                b"invalid sequence '\\%c' in -S\0" as *const u8
                                                    as *const libc::c_char,
                                                5 as libc::c_int,
                                            ),
                                            newc as libc::c_int,
                                        );
                                        if 0 as libc::c_int != 0 {} else {
                                            unreachable!();
                                        };
                                    };
                                }
                            }
                        }
                        0 => {
                            current_block = 9304275656119448851;
                            match current_block {
                                6049455476851897752 => {
                                    if !dq {
                                        str = str.offset(1);
                                        ss.sep = 1 as libc::c_int != 0;
                                        continue;
                                    } else {
                                        newc = ' ' as i32 as libc::c_char;
                                    }
                                    current_block = 9241535491006583629;
                                }
                                15127182143015129983 => {
                                    if dq {
                                        if ::core::mem::size_of::<C2RustUnnamed_18>()
                                            as libc::c_ulong != 0
                                        {
                                            error(
                                                EXIT_CANCELED as libc::c_int,
                                                0 as libc::c_int,
                                                dcgettext(
                                                    0 as *const libc::c_char,
                                                    b"'\\c' must not appear in double-quoted -S string\0"
                                                        as *const u8 as *const libc::c_char,
                                                    5 as libc::c_int,
                                                ),
                                            );
                                            if 0 as libc::c_int != 0 {} else {
                                                unreachable!();
                                            };
                                        } else {
                                            error(
                                                EXIT_CANCELED as libc::c_int,
                                                0 as libc::c_int,
                                                dcgettext(
                                                    0 as *const libc::c_char,
                                                    b"'\\c' must not appear in double-quoted -S string\0"
                                                        as *const u8 as *const libc::c_char,
                                                    5 as libc::c_int,
                                                ),
                                            );
                                            if 0 as libc::c_int != 0 {} else {
                                                unreachable!();
                                            };
                                        };
                                    }
                                    current_block = 9101223997307740060;
                                    break;
                                }
                                17674171488089097684 => {
                                    newc = '\u{c}' as i32 as libc::c_char;
                                    current_block = 9241535491006583629;
                                }
                                2585176034642815015 => {
                                    newc = '\n' as i32 as libc::c_char;
                                    current_block = 9241535491006583629;
                                }
                                16206231157784423297 => {
                                    newc = '\r' as i32 as libc::c_char;
                                    current_block = 9241535491006583629;
                                }
                                2459324531873169266 => {
                                    newc = '\t' as i32 as libc::c_char;
                                    current_block = 9241535491006583629;
                                }
                                11717047739380720017 => {
                                    newc = '\u{b}' as i32 as libc::c_char;
                                    current_block = 9241535491006583629;
                                }
                                9304275656119448851 => {
                                    if ::core::mem::size_of::<C2RustUnnamed_17>()
                                        as libc::c_ulong != 0
                                    {
                                        error(
                                            EXIT_CANCELED as libc::c_int,
                                            0 as libc::c_int,
                                            dcgettext(
                                                0 as *const libc::c_char,
                                                b"invalid backslash at end of string in -S\0" as *const u8
                                                    as *const libc::c_char,
                                                5 as libc::c_int,
                                            ),
                                        );
                                        if 0 as libc::c_int != 0 {} else {
                                            unreachable!();
                                        };
                                    } else {
                                        error(
                                            EXIT_CANCELED as libc::c_int,
                                            0 as libc::c_int,
                                            dcgettext(
                                                0 as *const libc::c_char,
                                                b"invalid backslash at end of string in -S\0" as *const u8
                                                    as *const libc::c_char,
                                                5 as libc::c_int,
                                            ),
                                        );
                                        if 0 as libc::c_int != 0 {} else {
                                            unreachable!();
                                        };
                                    };
                                    current_block = 17014196479992252067;
                                }
                                _ => {}
                            }
                            match current_block {
                                9241535491006583629 => {}
                                _ => {
                                    if ::core::mem::size_of::<C2RustUnnamed_16>()
                                        as libc::c_ulong != 0
                                    {
                                        error(
                                            EXIT_CANCELED as libc::c_int,
                                            0 as libc::c_int,
                                            dcgettext(
                                                0 as *const libc::c_char,
                                                b"invalid sequence '\\%c' in -S\0" as *const u8
                                                    as *const libc::c_char,
                                                5 as libc::c_int,
                                            ),
                                            newc as libc::c_int,
                                        );
                                        if 0 as libc::c_int != 0 {} else {
                                            unreachable!();
                                        };
                                    } else {
                                        error(
                                            EXIT_CANCELED as libc::c_int,
                                            0 as libc::c_int,
                                            dcgettext(
                                                0 as *const libc::c_char,
                                                b"invalid sequence '\\%c' in -S\0" as *const u8
                                                    as *const libc::c_char,
                                                5 as libc::c_int,
                                            ),
                                            newc as libc::c_int,
                                        );
                                        if 0 as libc::c_int != 0 {} else {
                                            unreachable!();
                                        };
                                    };
                                }
                            }
                        }
                        _ => {
                            current_block = 17014196479992252067;
                            match current_block {
                                6049455476851897752 => {
                                    if !dq {
                                        str = str.offset(1);
                                        ss.sep = 1 as libc::c_int != 0;
                                        continue;
                                    } else {
                                        newc = ' ' as i32 as libc::c_char;
                                    }
                                    current_block = 9241535491006583629;
                                }
                                15127182143015129983 => {
                                    if dq {
                                        if ::core::mem::size_of::<C2RustUnnamed_18>()
                                            as libc::c_ulong != 0
                                        {
                                            error(
                                                EXIT_CANCELED as libc::c_int,
                                                0 as libc::c_int,
                                                dcgettext(
                                                    0 as *const libc::c_char,
                                                    b"'\\c' must not appear in double-quoted -S string\0"
                                                        as *const u8 as *const libc::c_char,
                                                    5 as libc::c_int,
                                                ),
                                            );
                                            if 0 as libc::c_int != 0 {} else {
                                                unreachable!();
                                            };
                                        } else {
                                            error(
                                                EXIT_CANCELED as libc::c_int,
                                                0 as libc::c_int,
                                                dcgettext(
                                                    0 as *const libc::c_char,
                                                    b"'\\c' must not appear in double-quoted -S string\0"
                                                        as *const u8 as *const libc::c_char,
                                                    5 as libc::c_int,
                                                ),
                                            );
                                            if 0 as libc::c_int != 0 {} else {
                                                unreachable!();
                                            };
                                        };
                                    }
                                    current_block = 9101223997307740060;
                                    break;
                                }
                                17674171488089097684 => {
                                    newc = '\u{c}' as i32 as libc::c_char;
                                    current_block = 9241535491006583629;
                                }
                                2585176034642815015 => {
                                    newc = '\n' as i32 as libc::c_char;
                                    current_block = 9241535491006583629;
                                }
                                16206231157784423297 => {
                                    newc = '\r' as i32 as libc::c_char;
                                    current_block = 9241535491006583629;
                                }
                                2459324531873169266 => {
                                    newc = '\t' as i32 as libc::c_char;
                                    current_block = 9241535491006583629;
                                }
                                11717047739380720017 => {
                                    newc = '\u{b}' as i32 as libc::c_char;
                                    current_block = 9241535491006583629;
                                }
                                9304275656119448851 => {
                                    if ::core::mem::size_of::<C2RustUnnamed_17>()
                                        as libc::c_ulong != 0
                                    {
                                        error(
                                            EXIT_CANCELED as libc::c_int,
                                            0 as libc::c_int,
                                            dcgettext(
                                                0 as *const libc::c_char,
                                                b"invalid backslash at end of string in -S\0" as *const u8
                                                    as *const libc::c_char,
                                                5 as libc::c_int,
                                            ),
                                        );
                                        if 0 as libc::c_int != 0 {} else {
                                            unreachable!();
                                        };
                                    } else {
                                        error(
                                            EXIT_CANCELED as libc::c_int,
                                            0 as libc::c_int,
                                            dcgettext(
                                                0 as *const libc::c_char,
                                                b"invalid backslash at end of string in -S\0" as *const u8
                                                    as *const libc::c_char,
                                                5 as libc::c_int,
                                            ),
                                        );
                                        if 0 as libc::c_int != 0 {} else {
                                            unreachable!();
                                        };
                                    };
                                    current_block = 17014196479992252067;
                                }
                                _ => {}
                            }
                            match current_block {
                                9241535491006583629 => {}
                                _ => {
                                    if ::core::mem::size_of::<C2RustUnnamed_16>()
                                        as libc::c_ulong != 0
                                    {
                                        error(
                                            EXIT_CANCELED as libc::c_int,
                                            0 as libc::c_int,
                                            dcgettext(
                                                0 as *const libc::c_char,
                                                b"invalid sequence '\\%c' in -S\0" as *const u8
                                                    as *const libc::c_char,
                                                5 as libc::c_int,
                                            ),
                                            newc as libc::c_int,
                                        );
                                        if 0 as libc::c_int != 0 {} else {
                                            unreachable!();
                                        };
                                    } else {
                                        error(
                                            EXIT_CANCELED as libc::c_int,
                                            0 as libc::c_int,
                                            dcgettext(
                                                0 as *const libc::c_char,
                                                b"invalid sequence '\\%c' in -S\0" as *const u8
                                                    as *const libc::c_char,
                                                5 as libc::c_int,
                                            ),
                                            newc as libc::c_int,
                                        );
                                        if 0 as libc::c_int != 0 {} else {
                                            unreachable!();
                                        };
                                    };
                                }
                            }
                        }
                    }
                }
            }
            36 => {
                if !sq {
                    let mut n: *mut libc::c_char = extract_varname(str);
                    if n.is_null() {
                        if ::core::mem::size_of::<C2RustUnnamed_15>() as libc::c_ulong
                            != 0
                        {
                            error(
                                EXIT_CANCELED as libc::c_int,
                                0 as libc::c_int,
                                dcgettext(
                                    0 as *const libc::c_char,
                                    b"only ${VARNAME} expansion is supported, error at: %s\0"
                                        as *const u8 as *const libc::c_char,
                                    5 as libc::c_int,
                                ),
                                str,
                            );
                            if 0 as libc::c_int != 0 {} else {
                                unreachable!();
                            };
                        } else {
                            error(
                                EXIT_CANCELED as libc::c_int,
                                0 as libc::c_int,
                                dcgettext(
                                    0 as *const libc::c_char,
                                    b"only ${VARNAME} expansion is supported, error at: %s\0"
                                        as *const u8 as *const libc::c_char,
                                    5 as libc::c_int,
                                ),
                                str,
                            );
                            if 0 as libc::c_int != 0 {} else {
                                unreachable!();
                            };
                        };
                    }
                    let mut v: *mut libc::c_char = getenv(n);
                    if !v.is_null() {
                        check_start_new_arg(&mut ss);
                        if dev_debug {
                            fprintf(
                                stderr,
                                b"expanding ${%s} into %s\n\0" as *const u8
                                    as *const libc::c_char,
                                n,
                                quote(v),
                            );
                        }
                        while *v != 0 {
                            splitbuf_append_byte(&mut ss, *v);
                            v = v.offset(1);
                        }
                    } else if dev_debug {
                        fprintf(
                            stderr,
                            b"replacing ${%s} with null string\n\0" as *const u8
                                as *const libc::c_char,
                            n,
                        );
                    }
                    str = (strchr(str, '}' as i32)).offset(1 as libc::c_int as isize);
                    continue;
                }
            }
            _ => {}
        }
        check_start_new_arg(&mut ss);
        splitbuf_append_byte(&mut ss, newc);
        str = str.offset(1);
    }
    match current_block {
        8835654301469918283 => {
            if dq as libc::c_int != 0 || sq as libc::c_int != 0 {
                if ::core::mem::size_of::<C2RustUnnamed_14>() as libc::c_ulong != 0 {
                    error(
                        EXIT_CANCELED as libc::c_int,
                        0 as libc::c_int,
                        dcgettext(
                            0 as *const libc::c_char,
                            b"no terminating quote in -S string\0" as *const u8
                                as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                    );
                    if 0 as libc::c_int != 0 {} else {
                        unreachable!();
                    };
                } else {
                    error(
                        EXIT_CANCELED as libc::c_int,
                        0 as libc::c_int,
                        dcgettext(
                            0 as *const libc::c_char,
                            b"no terminating quote in -S string\0" as *const u8
                                as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                    );
                    if 0 as libc::c_int != 0 {} else {
                        unreachable!();
                    };
                };
            }
        }
        _ => {}
    }
    splitbuf_append_byte(&mut ss, '\0' as i32 as libc::c_char);
    *argc = ss.argc;
    return splitbuf_finishup(&mut ss);
}
unsafe extern "C" fn parse_split_string(
    mut str: *const libc::c_char,
    mut orig_optind: *mut libc::c_int,
    mut orig_argc: *mut libc::c_int,
    mut orig_argv: *mut *mut *mut libc::c_char,
) {
    let mut extra_argc: libc::c_int = *orig_argc - *orig_optind;
    let mut newargc: libc::c_int = 0;
    let mut newargv: *mut *mut libc::c_char = build_argv(str, extra_argc, &mut newargc);
    *newargv = *(*orig_argv).offset(0 as libc::c_int as isize);
    if dev_debug as libc::c_int != 0 && (1 as libc::c_int) < newargc {
        if dev_debug {
            fprintf(
                stderr,
                b"split -S:  %s\n\0" as *const u8 as *const libc::c_char,
                quote(str),
            );
        }
        if dev_debug {
            fprintf(
                stderr,
                b" into:    %s\n\0" as *const u8 as *const libc::c_char,
                quote(*newargv.offset(1 as libc::c_int as isize)),
            );
        }
        let mut i: libc::c_int = 2 as libc::c_int;
        while i < newargc {
            if dev_debug {
                fprintf(
                    stderr,
                    b"     &    %s\n\0" as *const u8 as *const libc::c_char,
                    quote(*newargv.offset(i as isize)),
                );
            }
            i += 1;
        }
    }
    memcpy(
        newargv.offset(newargc as isize) as *mut libc::c_void,
        (*orig_argv).offset(*orig_optind as isize) as *const libc::c_void,
        ((extra_argc + 1 as libc::c_int) as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<*mut libc::c_char>() as libc::c_ulong),
    );
    *orig_argc = newargc + extra_argc;
    *orig_argv = newargv;
    *orig_optind = 0 as libc::c_int;
}
unsafe extern "C" fn parse_signal_action_params(
    mut optarg_0: *const libc::c_char,
    mut set_default: bool,
) {
    let mut signame: [libc::c_char; 19] = [0; 19];
    let mut opt_sig: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut optarg_writable: *mut libc::c_char = 0 as *mut libc::c_char;
    if optarg_0.is_null() {
        let mut i: libc::c_int = 1 as libc::c_int;
        while i <= 64 as libc::c_int + 1 as libc::c_int - 1 as libc::c_int {
            if sig2str(i, signame.as_mut_ptr()) == 0 as libc::c_int {
                *signals
                    .offset(
                        i as isize,
                    ) = (if set_default as libc::c_int != 0 {
                    DEFAULT_NOERR as libc::c_int
                } else {
                    IGNORE_NOERR as libc::c_int
                }) as SIGNAL_MODE;
            }
            i += 1;
        }
        return;
    }
    optarg_writable = xstrdup(optarg_0);
    opt_sig = strtok(optarg_writable, b",\0" as *const u8 as *const libc::c_char);
    while !opt_sig.is_null() {
        let mut signum: libc::c_int = operand2sig(opt_sig, signame.as_mut_ptr());
        if signum == 0 as libc::c_int {
            error(
                0 as libc::c_int,
                0 as libc::c_int,
                dcgettext(
                    0 as *const libc::c_char,
                    b"%s: invalid signal\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
                quote(opt_sig),
            );
        }
        if signum <= 0 as libc::c_int {
            usage(exit_failure);
        }
        *signals
            .offset(
                signum as isize,
            ) = (if set_default as libc::c_int != 0 {
            DEFAULT as libc::c_int
        } else {
            IGNORE as libc::c_int
        }) as SIGNAL_MODE;
        opt_sig = strtok(
            0 as *mut libc::c_char,
            b",\0" as *const u8 as *const libc::c_char,
        );
    }
    free(optarg_writable as *mut libc::c_void);
}
unsafe extern "C" fn reset_signal_handlers() {
    let mut i: libc::c_int = 1 as libc::c_int;
    while i <= 64 as libc::c_int + 1 as libc::c_int - 1 as libc::c_int {
        let mut act: sigaction = sigaction {
            __sigaction_handler: C2RustUnnamed_9 {
                sa_handler: None,
            },
            sa_mask: sigset_t { __val: [0; 16] },
            sa_flags: 0,
            sa_restorer: None,
        };
        if !(*signals.offset(i as isize) as libc::c_uint
            == UNCHANGED as libc::c_int as libc::c_uint)
        {
            let mut ignore_errors: bool = *signals.offset(i as isize) as libc::c_uint
                == DEFAULT_NOERR as libc::c_int as libc::c_uint
                || *signals.offset(i as isize) as libc::c_uint
                    == IGNORE_NOERR as libc::c_int as libc::c_uint;
            let mut set_to_default: bool = *signals.offset(i as isize) as libc::c_uint
                == DEFAULT as libc::c_int as libc::c_uint
                || *signals.offset(i as isize) as libc::c_uint
                    == DEFAULT_NOERR as libc::c_int as libc::c_uint;
            let mut sig_err: libc::c_int = sigaction(i, 0 as *const sigaction, &mut act);
            if sig_err != 0 && !ignore_errors {
                if ::core::mem::size_of::<C2RustUnnamed_20>() as libc::c_ulong != 0 {
                    error(
                        EXIT_CANCELED as libc::c_int,
                        *__errno_location(),
                        dcgettext(
                            0 as *const libc::c_char,
                            b"failed to get signal action for signal %d\0" as *const u8
                                as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                        i,
                    );
                    if 0 as libc::c_int != 0 {} else {
                        unreachable!();
                    };
                } else {
                    error(
                        EXIT_CANCELED as libc::c_int,
                        *__errno_location(),
                        dcgettext(
                            0 as *const libc::c_char,
                            b"failed to get signal action for signal %d\0" as *const u8
                                as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                        i,
                    );
                    if 0 as libc::c_int != 0 {} else {
                        unreachable!();
                    };
                };
            }
            if sig_err == 0 {
                act
                    .__sigaction_handler
                    .sa_handler = if set_to_default as libc::c_int != 0 {
                    None
                } else {
                    ::core::mem::transmute::<
                        libc::intptr_t,
                        __sighandler_t,
                    >(1 as libc::c_int as libc::intptr_t)
                };
                sig_err = sigaction(i, &mut act, 0 as *mut sigaction);
                if sig_err != 0 && !ignore_errors {
                    if ::core::mem::size_of::<C2RustUnnamed_19>() as libc::c_ulong != 0 {
                        error(
                            EXIT_CANCELED as libc::c_int,
                            *__errno_location(),
                            dcgettext(
                                0 as *const libc::c_char,
                                b"failed to set signal action for signal %d\0" as *const u8
                                    as *const libc::c_char,
                                5 as libc::c_int,
                            ),
                            i,
                        );
                        if 0 as libc::c_int != 0 {} else {
                            unreachable!();
                        };
                    } else {
                        error(
                            EXIT_CANCELED as libc::c_int,
                            *__errno_location(),
                            dcgettext(
                                0 as *const libc::c_char,
                                b"failed to set signal action for signal %d\0" as *const u8
                                    as *const libc::c_char,
                                5 as libc::c_int,
                            ),
                            i,
                        );
                        if 0 as libc::c_int != 0 {} else {
                            unreachable!();
                        };
                    };
                }
            }
            if dev_debug {
                let mut signame: [libc::c_char; 19] = [0; 19];
                sig2str(i, signame.as_mut_ptr());
                if dev_debug {
                    fprintf(
                        stderr,
                        b"Reset signal %s (%d) to %s%s\n\0" as *const u8
                            as *const libc::c_char,
                        signame.as_mut_ptr(),
                        i,
                        if set_to_default as libc::c_int != 0 {
                            b"DEFAULT\0" as *const u8 as *const libc::c_char
                        } else {
                            b"IGNORE\0" as *const u8 as *const libc::c_char
                        },
                        if sig_err != 0 {
                            b" (failure ignored)\0" as *const u8 as *const libc::c_char
                        } else {
                            b"\0" as *const u8 as *const libc::c_char
                        },
                    );
                }
            }
        }
        i += 1;
    }
}
unsafe extern "C" fn parse_block_signal_params(
    mut optarg_0: *const libc::c_char,
    mut block: bool,
) {
    let mut signame: [libc::c_char; 19] = [0; 19];
    let mut opt_sig: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut optarg_writable: *mut libc::c_char = 0 as *mut libc::c_char;
    if optarg_0.is_null() {
        sigfillset(
            if block as libc::c_int != 0 {
                &mut block_signals
            } else {
                &mut unblock_signals
            },
        );
        sigemptyset(
            if block as libc::c_int != 0 {
                &mut unblock_signals
            } else {
                &mut block_signals
            },
        );
    } else if !sig_mask_changed {
        sigemptyset(&mut block_signals);
        sigemptyset(&mut unblock_signals);
    }
    sig_mask_changed = 1 as libc::c_int != 0;
    if optarg_0.is_null() {
        return;
    }
    optarg_writable = xstrdup(optarg_0);
    opt_sig = strtok(optarg_writable, b",\0" as *const u8 as *const libc::c_char);
    while !opt_sig.is_null() {
        let mut signum: libc::c_int = operand2sig(opt_sig, signame.as_mut_ptr());
        if signum == 0 as libc::c_int {
            error(
                0 as libc::c_int,
                0 as libc::c_int,
                dcgettext(
                    0 as *const libc::c_char,
                    b"%s: invalid signal\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
                quote(opt_sig),
            );
        }
        if signum <= 0 as libc::c_int {
            usage(exit_failure);
        }
        sigaddset(
            if block as libc::c_int != 0 {
                &mut block_signals
            } else {
                &mut unblock_signals
            },
            signum,
        );
        sigdelset(
            if block as libc::c_int != 0 {
                &mut unblock_signals
            } else {
                &mut block_signals
            },
            signum,
        );
        opt_sig = strtok(
            0 as *mut libc::c_char,
            b",\0" as *const u8 as *const libc::c_char,
        );
    }
    free(optarg_writable as *mut libc::c_void);
}
unsafe extern "C" fn set_signal_proc_mask() {
    let mut set: sigset_t = sigset_t { __val: [0; 16] };
    let mut debug_act: *const libc::c_char = 0 as *const libc::c_char;
    sigemptyset(&mut set);
    if sigprocmask(0 as libc::c_int, 0 as *const sigset_t, &mut set) != 0 {
        if ::core::mem::size_of::<C2RustUnnamed_22>() as libc::c_ulong != 0 {
            error(
                EXIT_CANCELED as libc::c_int,
                *__errno_location(),
                dcgettext(
                    0 as *const libc::c_char,
                    b"failed to get signal process mask\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
            );
            if 0 as libc::c_int != 0 {} else {
                unreachable!();
            };
        } else {
            error(
                EXIT_CANCELED as libc::c_int,
                *__errno_location(),
                dcgettext(
                    0 as *const libc::c_char,
                    b"failed to get signal process mask\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
            );
            if 0 as libc::c_int != 0 {} else {
                unreachable!();
            };
        };
    }
    let mut i: libc::c_int = 1 as libc::c_int;
    while i <= 64 as libc::c_int + 1 as libc::c_int - 1 as libc::c_int {
        if sigismember(&mut block_signals, i) != 0 {
            sigaddset(&mut set, i);
            debug_act = b"BLOCK\0" as *const u8 as *const libc::c_char;
        } else if sigismember(&mut unblock_signals, i) != 0 {
            sigdelset(&mut set, i);
            debug_act = b"UNBLOCK\0" as *const u8 as *const libc::c_char;
        } else {
            debug_act = 0 as *const libc::c_char;
        }
        if dev_debug as libc::c_int != 0 && !debug_act.is_null() {
            let mut signame: [libc::c_char; 19] = [0; 19];
            sig2str(i, signame.as_mut_ptr());
            if dev_debug {
                fprintf(
                    stderr,
                    b"signal %s (%d) mask set to %s\n\0" as *const u8
                        as *const libc::c_char,
                    signame.as_mut_ptr(),
                    i,
                    debug_act,
                );
            }
        }
        i += 1;
    }
    if sigprocmask(2 as libc::c_int, &mut set, 0 as *mut sigset_t) != 0 {
        if ::core::mem::size_of::<C2RustUnnamed_21>() as libc::c_ulong != 0 {
            error(
                EXIT_CANCELED as libc::c_int,
                *__errno_location(),
                dcgettext(
                    0 as *const libc::c_char,
                    b"failed to set signal process mask\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
            );
            if 0 as libc::c_int != 0 {} else {
                unreachable!();
            };
        } else {
            error(
                EXIT_CANCELED as libc::c_int,
                *__errno_location(),
                dcgettext(
                    0 as *const libc::c_char,
                    b"failed to set signal process mask\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
            );
            if 0 as libc::c_int != 0 {} else {
                unreachable!();
            };
        };
    }
}
unsafe extern "C" fn list_signal_handling() {
    let mut set: sigset_t = sigset_t { __val: [0; 16] };
    let mut signame: [libc::c_char; 19] = [0; 19];
    sigemptyset(&mut set);
    if sigprocmask(0 as libc::c_int, 0 as *const sigset_t, &mut set) != 0 {
        if ::core::mem::size_of::<C2RustUnnamed_23>() as libc::c_ulong != 0 {
            error(
                EXIT_CANCELED as libc::c_int,
                *__errno_location(),
                dcgettext(
                    0 as *const libc::c_char,
                    b"failed to get signal process mask\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
            );
            if 0 as libc::c_int != 0 {} else {
                unreachable!();
            };
        } else {
            error(
                EXIT_CANCELED as libc::c_int,
                *__errno_location(),
                dcgettext(
                    0 as *const libc::c_char,
                    b"failed to get signal process mask\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
            );
            if 0 as libc::c_int != 0 {} else {
                unreachable!();
            };
        };
    }
    let mut i: libc::c_int = 1 as libc::c_int;
    while i <= 64 as libc::c_int + 1 as libc::c_int - 1 as libc::c_int {
        let mut act: sigaction = sigaction {
            __sigaction_handler: C2RustUnnamed_9 {
                sa_handler: None,
            },
            sa_mask: sigset_t { __val: [0; 16] },
            sa_flags: 0,
            sa_restorer: None,
        };
        if !(sigaction(i, 0 as *const sigaction, &mut act) != 0) {
            let mut ignored: *const libc::c_char = if act.__sigaction_handler.sa_handler
                == ::core::mem::transmute::<
                    libc::intptr_t,
                    __sighandler_t,
                >(1 as libc::c_int as libc::intptr_t)
            {
                b"IGNORE\0" as *const u8 as *const libc::c_char
            } else {
                b"\0" as *const u8 as *const libc::c_char
            };
            let mut blocked: *const libc::c_char = if sigismember(&mut set, i) != 0 {
                b"BLOCK\0" as *const u8 as *const libc::c_char
            } else {
                b"\0" as *const u8 as *const libc::c_char
            };
            let mut connect: *const libc::c_char = if *ignored as libc::c_int != 0
                && *blocked as libc::c_int != 0
            {
                b",\0" as *const u8 as *const libc::c_char
            } else {
                b"\0" as *const u8 as *const libc::c_char
            };
            if !(*ignored == 0 && *blocked == 0) {
                sig2str(i, signame.as_mut_ptr());
                fprintf(
                    stderr,
                    b"%-10s (%2d): %s%s%s\n\0" as *const u8 as *const libc::c_char,
                    signame.as_mut_ptr(),
                    i,
                    blocked,
                    connect,
                    ignored,
                );
            }
        }
        i += 1;
    }
}
unsafe extern "C" fn initialize_signals() {
    signals = xmalloc(
        (::core::mem::size_of::<SIGNAL_MODE>() as libc::c_ulong)
            .wrapping_mul(
                (64 as libc::c_int + 1 as libc::c_int - 1 as libc::c_int
                    + 1 as libc::c_int) as libc::c_ulong,
            ),
    ) as *mut SIGNAL_MODE;
    let mut i: libc::c_int = 0 as libc::c_int;
    while i <= 64 as libc::c_int + 1 as libc::c_int - 1 as libc::c_int {
        *signals.offset(i as isize) = UNCHANGED;
        i += 1;
    }
}
unsafe fn main_0(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut optc: libc::c_int = 0;
    let mut ignore_environment: bool = 0 as libc::c_int != 0;
    let mut opt_nul_terminate_output: bool = 0 as libc::c_int != 0;
    let mut newdir: *const libc::c_char = 0 as *const libc::c_char;
    set_program_name(*argv.offset(0 as libc::c_int as isize));
    setlocale(6 as libc::c_int, b"\0" as *const u8 as *const libc::c_char);
    bindtextdomain(
        b"coreutils\0" as *const u8 as *const libc::c_char,
        b"/usr/local/share/locale\0" as *const u8 as *const libc::c_char,
    );
    textdomain(b"coreutils\0" as *const u8 as *const libc::c_char);
    initialize_exit_failure(EXIT_CANCELED as libc::c_int);
    atexit(Some(close_stdout as unsafe extern "C" fn() -> ()));
    initialize_signals();
    loop {
        optc = getopt_long(
            argc,
            argv,
            shortopts.as_ptr(),
            longopts.as_ptr(),
            0 as *mut libc::c_int,
        );
        if !(optc != -(1 as libc::c_int)) {
            break;
        }
        let mut current_block_27: u64;
        match optc {
            105 => {
                ignore_environment = 1 as libc::c_int != 0;
                current_block_27 = 18377268871191777778;
            }
            117 => {
                append_unset_var(optarg);
                current_block_27 = 18377268871191777778;
            }
            118 => {
                dev_debug = 1 as libc::c_int != 0;
                current_block_27 = 18377268871191777778;
            }
            48 => {
                opt_nul_terminate_output = 1 as libc::c_int != 0;
                current_block_27 = 18377268871191777778;
            }
            256 => {
                parse_signal_action_params(optarg, 1 as libc::c_int != 0);
                parse_block_signal_params(optarg, 0 as libc::c_int != 0);
                current_block_27 = 18377268871191777778;
            }
            257 => {
                parse_signal_action_params(optarg, 0 as libc::c_int != 0);
                current_block_27 = 18377268871191777778;
            }
            258 => {
                parse_block_signal_params(optarg, 1 as libc::c_int != 0);
                current_block_27 = 18377268871191777778;
            }
            259 => {
                report_signal_handling = 1 as libc::c_int != 0;
                current_block_27 = 18377268871191777778;
            }
            67 => {
                newdir = optarg;
                current_block_27 = 18377268871191777778;
            }
            83 => {
                parse_split_string(optarg, &mut optind, &mut argc, &mut argv);
                current_block_27 = 18377268871191777778;
            }
            32 | 9 | 10 | 11 | 12 | 13 => {
                error(
                    0 as libc::c_int,
                    0 as libc::c_int,
                    dcgettext(
                        0 as *const libc::c_char,
                        b"invalid option -- '%c'\0" as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    optc,
                );
                error(
                    0 as libc::c_int,
                    0 as libc::c_int,
                    dcgettext(
                        0 as *const libc::c_char,
                        b"use -[v]S to pass options in shebang lines\0" as *const u8
                            as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                );
                usage(EXIT_CANCELED as libc::c_int);
                current_block_27 = 18169099900438651441;
            }
            -2 => {
                current_block_27 = 18169099900438651441;
            }
            -3 => {
                version_etc(
                    stdout,
                    b"env\0" as *const u8 as *const libc::c_char,
                    b"GNU coreutils\0" as *const u8 as *const libc::c_char,
                    Version,
                    b"Richard Mlynarik\0" as *const u8 as *const libc::c_char,
                    b"David MacKenzie\0" as *const u8 as *const libc::c_char,
                    b"Assaf Gordon\0" as *const u8 as *const libc::c_char,
                    0 as *mut libc::c_void as *mut libc::c_char,
                );
                exit(0 as libc::c_int);
            }
            _ => {
                usage(EXIT_CANCELED as libc::c_int);
                current_block_27 = 18377268871191777778;
            }
        }
        match current_block_27 {
            18169099900438651441 => {
                usage(0 as libc::c_int);
            }
            _ => {}
        }
    }
    if optind < argc
        && strcmp(
            *argv.offset(optind as isize),
            b"-\0" as *const u8 as *const libc::c_char,
        ) == 0 as libc::c_int
    {
        ignore_environment = 1 as libc::c_int != 0;
        optind += 1;
    }
    if ignore_environment {
        if dev_debug {
            fprintf(stderr, b"cleaning environ\n\0" as *const u8 as *const libc::c_char);
        }
        static mut dummy_environ: [*mut libc::c_char; 1] = [
            0 as *const libc::c_char as *mut libc::c_char,
        ];
        environ = dummy_environ.as_mut_ptr();
    } else {
        unset_envvars();
    }
    let mut eq: *mut libc::c_char = 0 as *mut libc::c_char;
    while optind < argc
        && {
            eq = strchr(*argv.offset(optind as isize), '=' as i32);
            !eq.is_null()
        }
    {
        if dev_debug {
            fprintf(
                stderr,
                b"setenv:   %s\n\0" as *const u8 as *const libc::c_char,
                *argv.offset(optind as isize),
            );
        }
        if putenv(*argv.offset(optind as isize)) != 0 {
            *eq = '\0' as i32 as libc::c_char;
            if ::core::mem::size_of::<C2RustUnnamed_25>() as libc::c_ulong != 0 {
                error(
                    EXIT_CANCELED as libc::c_int,
                    *__errno_location(),
                    dcgettext(
                        0 as *const libc::c_char,
                        b"cannot set %s\0" as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    quote(*argv.offset(optind as isize)),
                );
                if 0 as libc::c_int != 0 {} else {
                    unreachable!();
                };
            } else {
                error(
                    EXIT_CANCELED as libc::c_int,
                    *__errno_location(),
                    dcgettext(
                        0 as *const libc::c_char,
                        b"cannot set %s\0" as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    quote(*argv.offset(optind as isize)),
                );
                if 0 as libc::c_int != 0 {} else {
                    unreachable!();
                };
            };
        }
        optind += 1;
    }
    let mut program_specified: bool = optind < argc;
    if opt_nul_terminate_output as libc::c_int != 0
        && program_specified as libc::c_int != 0
    {
        error(
            0 as libc::c_int,
            0 as libc::c_int,
            dcgettext(
                0 as *const libc::c_char,
                b"cannot specify --null (-0) with command\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
        usage(EXIT_CANCELED as libc::c_int);
    }
    if !newdir.is_null() && !program_specified {
        error(
            0 as libc::c_int,
            0 as libc::c_int,
            dcgettext(
                0 as *const libc::c_char,
                b"must specify command with --chdir (-C)\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
        usage(EXIT_CANCELED as libc::c_int);
    }
    if !program_specified {
        let mut e: *const *mut libc::c_char = environ;
        while !(*e).is_null() {
            let fresh6 = e;
            e = e.offset(1);
            printf(
                b"%s%c\0" as *const u8 as *const libc::c_char,
                *fresh6,
                if opt_nul_terminate_output as libc::c_int != 0 {
                    '\0' as i32
                } else {
                    '\n' as i32
                },
            );
        }
        return 0 as libc::c_int;
    }
    reset_signal_handlers();
    if sig_mask_changed {
        set_signal_proc_mask();
    }
    if report_signal_handling {
        list_signal_handling();
    }
    if !newdir.is_null() {
        if dev_debug {
            fprintf(
                stderr,
                b"chdir:    %s\n\0" as *const u8 as *const libc::c_char,
                quotearg_style(shell_escape_always_quoting_style, newdir),
            );
        }
        if chdir(newdir) != 0 as libc::c_int {
            if ::core::mem::size_of::<C2RustUnnamed_24>() as libc::c_ulong != 0 {
                error(
                    EXIT_CANCELED as libc::c_int,
                    *__errno_location(),
                    dcgettext(
                        0 as *const libc::c_char,
                        b"cannot change directory to %s\0" as *const u8
                            as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    quotearg_style(shell_escape_always_quoting_style, newdir),
                );
                if 0 as libc::c_int != 0 {} else {
                    unreachable!();
                };
            } else {
                error(
                    EXIT_CANCELED as libc::c_int,
                    *__errno_location(),
                    dcgettext(
                        0 as *const libc::c_char,
                        b"cannot change directory to %s\0" as *const u8
                            as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    quotearg_style(shell_escape_always_quoting_style, newdir),
                );
                if 0 as libc::c_int != 0 {} else {
                    unreachable!();
                };
            };
        }
    }
    if dev_debug {
        if dev_debug {
            fprintf(
                stderr,
                b"executing: %s\n\0" as *const u8 as *const libc::c_char,
                *argv.offset(optind as isize),
            );
        }
        let mut i: libc::c_int = optind;
        while i < argc {
            if dev_debug {
                fprintf(
                    stderr,
                    b"   arg[%d]= %s\n\0" as *const u8 as *const libc::c_char,
                    i - optind,
                    quote(*argv.offset(i as isize)),
                );
            }
            i += 1;
        }
    }
    execvp(
        *argv.offset(optind as isize),
        &mut *argv.offset(optind as isize) as *mut *mut libc::c_char
            as *const *mut libc::c_char,
    );
    let mut exit_status: libc::c_int = if *__errno_location() == 2 as libc::c_int {
        EXIT_ENOENT as libc::c_int
    } else {
        EXIT_CANNOT_INVOKE as libc::c_int
    };
    error(
        0 as libc::c_int,
        *__errno_location(),
        b"%s\0" as *const u8 as *const libc::c_char,
        quote(*argv.offset(optind as isize)),
    );
    if exit_status == EXIT_ENOENT as libc::c_int
        && !(strpbrk(
            *argv.offset(optind as isize),
            b" \t\n\x0B\x0C\r\0" as *const u8 as *const libc::c_char,
        ))
            .is_null()
    {
        error(
            0 as libc::c_int,
            0 as libc::c_int,
            dcgettext(
                0 as *const libc::c_char,
                b"use -[v]S to pass options in shebang lines\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
    }
    exit(exit_status);
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
