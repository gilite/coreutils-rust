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
    static mut stdout: *mut FILE;
    static mut stderr: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn snprintf(
        _: *mut libc::c_char,
        _: libc::c_ulong,
        _: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn fputs_unlocked(__s: *const libc::c_char, __stream: *mut FILE) -> libc::c_int;
    fn raise(__sig: libc::c_int) -> libc::c_int;
    fn sigemptyset(__set: *mut sigset_t) -> libc::c_int;
    fn sigaddset(__set: *mut sigset_t, __signo: libc::c_int) -> libc::c_int;
    fn sigprocmask(
        __how: libc::c_int,
        __set: *const sigset_t,
        __oset: *mut sigset_t,
    ) -> libc::c_int;
    fn sigsuspend(__set: *const sigset_t) -> libc::c_int;
    fn sigaction(
        __sig: libc::c_int,
        __act: *const sigaction,
        __oact: *mut sigaction,
    ) -> libc::c_int;
    fn execvp(
        __file: *const libc::c_char,
        __argv: *const *mut libc::c_char,
    ) -> libc::c_int;
    fn alarm(__seconds: libc::c_uint) -> libc::c_uint;
    fn _exit(_: libc::c_int) -> !;
    fn setpgid(__pid: __pid_t, __pgid: __pid_t) -> libc::c_int;
    fn fork() -> __pid_t;
    fn signal(__sig: libc::c_int, __handler: __sighandler_t) -> __sighandler_t;
    fn kill(__pid: __pid_t, __sig: libc::c_int) -> libc::c_int;
    fn prctl(__option: libc::c_int, _: ...) -> libc::c_int;
    fn waitpid(
        __pid: __pid_t,
        __stat_loc: *mut libc::c_int,
        __options: libc::c_int,
    ) -> __pid_t;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strncmp(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn __errno_location() -> *mut libc::c_int;
    fn atexit(__func: Option::<unsafe extern "C" fn() -> ()>) -> libc::c_int;
    fn exit(_: libc::c_int) -> !;
    static mut Version: *const libc::c_char;
    static mut exit_failure: libc::c_int;
    fn timer_create(
        __clock_id: clockid_t,
        __evp: *mut sigevent,
        __timerid: *mut timer_t,
    ) -> libc::c_int;
    fn timer_delete(__timerid: timer_t) -> libc::c_int;
    fn dtotimespec(_: libc::c_double) -> timespec;
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
    fn version_etc(
        stream: *mut FILE,
        command_name: *const libc::c_char,
        package: *const libc::c_char,
        version: *const libc::c_char,
        _: ...
    );
    static mut program_name: *const libc::c_char;
    fn set_program_name(argv0: *const libc::c_char);
    fn timer_settime(
        __timerid: timer_t,
        __flags: libc::c_int,
        __value: *const itimerspec,
        __ovalue: *mut itimerspec,
    ) -> libc::c_int;
    fn cl_strtod(_: *const libc::c_char, _: *mut *mut libc::c_char) -> libc::c_double;
    fn xstrtod(
        str: *const libc::c_char,
        ptr: *mut *const libc::c_char,
        result: *mut libc::c_double,
        convert: Option::<
            unsafe extern "C" fn(
                *const libc::c_char,
                *mut *mut libc::c_char,
            ) -> libc::c_double,
        >,
    ) -> bool;
    fn sig2str(_: libc::c_int, _: *mut libc::c_char) -> libc::c_int;
    fn operand2sig(
        operand: *const libc::c_char,
        signame: *mut libc::c_char,
    ) -> libc::c_int;
    fn error(
        __status: libc::c_int,
        __errnum: libc::c_int,
        __format: *const libc::c_char,
        _: ...
    );
    fn quote(arg: *const libc::c_char) -> *const libc::c_char;
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct option {
    pub name: *const libc::c_char,
    pub has_arg: libc::c_int,
    pub flag: *mut libc::c_int,
    pub val: libc::c_int,
}
pub type size_t = libc::c_ulong;
pub type __uint32_t = libc::c_uint;
pub type __uid_t = libc::c_uint;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __pid_t = libc::c_int;
pub type __clock_t = libc::c_long;
pub type __time_t = libc::c_long;
pub type __clockid_t = libc::c_int;
pub type __timer_t = *mut libc::c_void;
pub type __syscall_slong_t = libc::c_long;
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
pub type pid_t = __pid_t;
pub type clockid_t = __clockid_t;
pub type timer_t = __timer_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __sigset_t {
    pub __val: [libc::c_ulong; 16],
}
pub type sigset_t = __sigset_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union pthread_attr_t {
    pub __size: [libc::c_char; 64],
    pub __align: libc::c_long,
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sigevent {
    pub sigev_value: __sigval_t,
    pub sigev_signo: libc::c_int,
    pub sigev_notify: libc::c_int,
    pub _sigev_un: C2RustUnnamed_9,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_9 {
    pub _pad: [libc::c_int; 12],
    pub _tid: __pid_t,
    pub _sigev_thread: C2RustUnnamed_10,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_10 {
    pub _function: Option::<unsafe extern "C" fn(__sigval_t) -> ()>,
    pub _attribute: *mut pthread_attr_t,
}
pub type __sighandler_t = Option::<unsafe extern "C" fn(libc::c_int) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sigaction {
    pub __sigaction_handler: C2RustUnnamed_11,
    pub sa_mask: __sigset_t,
    pub sa_flags: libc::c_int,
    pub sa_restorer: Option::<unsafe extern "C" fn() -> ()>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_11 {
    pub sa_handler: __sighandler_t,
    pub sa_sigaction: Option::<
        unsafe extern "C" fn(libc::c_int, *mut siginfo_t, *mut libc::c_void) -> (),
    >,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct itimerspec {
    pub it_interval: timespec,
    pub it_value: timespec,
}
pub type C2RustUnnamed_12 = libc::c_uint;
pub const EXIT_ENOENT: C2RustUnnamed_12 = 127;
pub const EXIT_CANNOT_INVOKE: C2RustUnnamed_12 = 126;
pub const EXIT_CANCELED: C2RustUnnamed_12 = 125;
pub const EXIT_TIMEDOUT: C2RustUnnamed_12 = 124;
pub type C2RustUnnamed_13 = libc::c_int;
pub const GETOPT_VERSION_CHAR: C2RustUnnamed_13 = -3;
pub const GETOPT_HELP_CHAR: C2RustUnnamed_13 = -2;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct infomap {
    pub program: *const libc::c_char,
    pub node: *const libc::c_char,
}
pub type C2RustUnnamed_14 = libc::c_uint;
pub const PRESERVE_STATUS_OPTION: C2RustUnnamed_14 = 257;
pub const FOREGROUND_OPTION: C2RustUnnamed_14 = 256;
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
static mut timed_out: libc::c_int = 0;
static mut term_signal: libc::c_int = 15 as libc::c_int;
static mut monitored_pid: pid_t = 0;
static mut kill_after: libc::c_double = 0.;
static mut foreground: bool = false;
static mut preserve_status: bool = false;
static mut verbose: bool = false;
static mut command: *const libc::c_char = 0 as *const libc::c_char;
static mut long_options: [option; 8] = [
    {
        let mut init = option {
            name: b"kill-after\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'k' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"signal\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 's' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"verbose\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'v' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"foreground\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: FOREGROUND_OPTION as libc::c_int,
        };
        init
    },
    {
        let mut init = option {
            name: b"preserve-status\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: PRESERVE_STATUS_OPTION as libc::c_int,
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
unsafe extern "C" fn settimeout(mut duration: libc::c_double, mut warn: bool) {
    let mut ts: timespec = dtotimespec(duration);
    let mut its: itimerspec = {
        let mut init = itimerspec {
            it_interval: {
                let mut init = timespec {
                    tv_sec: 0 as libc::c_int as __time_t,
                    tv_nsec: 0 as libc::c_int as __syscall_slong_t,
                };
                init
            },
            it_value: ts,
        };
        init
    };
    let mut timerid: timer_t = 0 as *mut libc::c_void;
    if timer_create(0 as libc::c_int, 0 as *mut sigevent, &mut timerid)
        == 0 as libc::c_int
    {
        if timer_settime(timerid, 0 as libc::c_int, &mut its, 0 as *mut itimerspec)
            == 0 as libc::c_int
        {
            return
        } else {
            if warn {
                error(
                    0 as libc::c_int,
                    *__errno_location(),
                    dcgettext(
                        0 as *const libc::c_char,
                        b"warning: timer_settime\0" as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                );
            }
            timer_delete(timerid);
        }
    } else if warn as libc::c_int != 0 && *__errno_location() != 38 as libc::c_int {
        error(
            0 as libc::c_int,
            *__errno_location(),
            dcgettext(
                0 as *const libc::c_char,
                b"warning: timer_create\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
    }
    let mut timeint: libc::c_uint = 0;
    if (2147483647 as libc::c_int as libc::c_uint)
        .wrapping_mul(2 as libc::c_uint)
        .wrapping_add(1 as libc::c_uint) as libc::c_double <= duration
    {
        timeint = (2147483647 as libc::c_int as libc::c_uint)
            .wrapping_mul(2 as libc::c_uint)
            .wrapping_add(1 as libc::c_uint);
    } else {
        let mut duration_floor: libc::c_uint = duration as libc::c_uint;
        timeint = duration_floor
            .wrapping_add(
                ((duration_floor as libc::c_double) < duration) as libc::c_int
                    as libc::c_uint,
            );
    }
    alarm(timeint);
}
unsafe extern "C" fn send_sig(mut where_0: pid_t, mut sig: libc::c_int) -> libc::c_int {
    if where_0 == 0 as libc::c_int {
        signal(
            sig,
            ::core::mem::transmute::<
                libc::intptr_t,
                __sighandler_t,
            >(1 as libc::c_int as libc::intptr_t),
        );
    }
    return kill(where_0, sig);
}
unsafe extern "C" fn chld(mut sig: libc::c_int) {}
unsafe extern "C" fn cleanup(mut sig: libc::c_int) {
    if sig == 14 as libc::c_int {
        timed_out = 1 as libc::c_int;
        sig = term_signal;
    }
    if monitored_pid != 0 {
        if kill_after != 0. {
            let mut saved_errno: libc::c_int = *__errno_location();
            term_signal = 9 as libc::c_int;
            settimeout(kill_after, 0 as libc::c_int != 0);
            kill_after = 0 as libc::c_int as libc::c_double;
            *__errno_location() = saved_errno;
        }
        if verbose {
            let mut signame: [libc::c_char; 19] = [0; 19];
            if sig2str(sig, signame.as_mut_ptr()) != 0 as libc::c_int {
                snprintf(
                    signame.as_mut_ptr(),
                    ::core::mem::size_of::<[libc::c_char; 19]>() as libc::c_ulong,
                    b"%d\0" as *const u8 as *const libc::c_char,
                    sig,
                );
            }
            error(
                0 as libc::c_int,
                0 as libc::c_int,
                dcgettext(
                    0 as *const libc::c_char,
                    b"sending signal %s to command %s\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
                signame.as_mut_ptr(),
                quote(command),
            );
        }
        send_sig(monitored_pid, sig);
        if !foreground {
            send_sig(0 as libc::c_int, sig);
            if sig != 9 as libc::c_int && sig != 18 as libc::c_int {
                send_sig(monitored_pid, 18 as libc::c_int);
                send_sig(0 as libc::c_int, 18 as libc::c_int);
            }
        }
    } else {
        _exit(128 as libc::c_int + sig);
    };
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
                b"Usage: %s [OPTION] DURATION COMMAND [ARG]...\n  or:  %s [OPTION]\n\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            program_name,
            program_name,
        );
        fputs_unlocked(
            dcgettext(
                0 as *const libc::c_char,
                b"Start COMMAND, and kill it if still running after DURATION.\n\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            stdout,
        );
        emit_mandatory_arg_note();
        fputs_unlocked(
            dcgettext(
                0 as *const libc::c_char,
                b"      --preserve-status\n                 exit with the same status as COMMAND, even when the\n                   command times out\n      --foreground\n                 when not running timeout directly from a shell prompt,\n                   allow COMMAND to read from the TTY and get TTY signals;\n                   in this mode, children of COMMAND will not be timed out\n  -k, --kill-after=DURATION\n                 also send a KILL signal if COMMAND is still running\n                   this long after the initial signal was sent\n  -s, --signal=SIGNAL\n                 specify the signal to be sent on timeout;\n                   SIGNAL may be a name like 'HUP' or a number;\n                   see 'kill -l' for a list of signals\n\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            stdout,
        );
        fputs_unlocked(
            dcgettext(
                0 as *const libc::c_char,
                b"  -v, --verbose  diagnose to stderr any signal sent upon timeout\n\0"
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
                b"\nDURATION is a floating point number with an optional suffix:\n's' for seconds (the default), 'm' for minutes, 'h' for hours or 'd' for days.\nA duration of 0 disables the associated timeout.\n\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            stdout,
        );
        fputs_unlocked(
            dcgettext(
                0 as *const libc::c_char,
                b"\nUpon timeout, send the TERM signal to COMMAND, if no other SIGNAL specified.\nThe TERM signal kills any process that does not block or catch that signal.\nIt may be necessary to use the KILL signal, since this signal can't be caught.\n\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            stdout,
        );
        fputs_unlocked(
            dcgettext(
                0 as *const libc::c_char,
                b"\nExit status:\n  124  if COMMAND times out, and --preserve-status is not specified\n  125  if the timeout command itself fails\n  126  if COMMAND is found but cannot be invoked\n  127  if COMMAND cannot be found\n  137  if COMMAND (or timeout itself) is sent the KILL (9) signal (128+9)\n  -    the exit status of COMMAND otherwise\n\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            stdout,
        );
        emit_ancillary_info(b"timeout\0" as *const u8 as *const libc::c_char);
    }
    exit(status);
}
unsafe extern "C" fn apply_time_suffix(
    mut x: *mut libc::c_double,
    mut suffix_char: libc::c_char,
) -> bool {
    let mut multiplier: libc::c_int = 0;
    match suffix_char as libc::c_int {
        0 | 115 => {
            multiplier = 1 as libc::c_int;
        }
        109 => {
            multiplier = 60 as libc::c_int;
        }
        104 => {
            multiplier = 60 as libc::c_int * 60 as libc::c_int;
        }
        100 => {
            multiplier = 60 as libc::c_int * 60 as libc::c_int * 24 as libc::c_int;
        }
        _ => return 0 as libc::c_int != 0,
    }
    *x *= multiplier as libc::c_double;
    return 1 as libc::c_int != 0;
}
unsafe extern "C" fn parse_duration(mut str: *const libc::c_char) -> libc::c_double {
    let mut duration: libc::c_double = 0.;
    let mut ep: *const libc::c_char = 0 as *const libc::c_char;
    if !(xstrtod(
        str,
        &mut ep,
        &mut duration,
        Some(
            cl_strtod
                as unsafe extern "C" fn(
                    *const libc::c_char,
                    *mut *mut libc::c_char,
                ) -> libc::c_double,
        ),
    ) as libc::c_int != 0 || *__errno_location() == 34 as libc::c_int)
        || !(0 as libc::c_int as libc::c_double <= duration)
        || *ep as libc::c_int != 0
            && *ep.offset(1 as libc::c_int as isize) as libc::c_int != 0
        || !apply_time_suffix(&mut duration, *ep)
    {
        error(
            0 as libc::c_int,
            0 as libc::c_int,
            dcgettext(
                0 as *const libc::c_char,
                b"invalid time interval %s\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            quote(str),
        );
        usage(EXIT_CANCELED as libc::c_int);
    }
    return duration;
}
unsafe extern "C" fn unblock_signal(mut sig: libc::c_int) {
    let mut unblock_set: sigset_t = sigset_t { __val: [0; 16] };
    sigemptyset(&mut unblock_set);
    sigaddset(&mut unblock_set, sig);
    if sigprocmask(1 as libc::c_int, &mut unblock_set, 0 as *mut sigset_t)
        != 0 as libc::c_int
    {
        error(
            0 as libc::c_int,
            *__errno_location(),
            dcgettext(
                0 as *const libc::c_char,
                b"warning: sigprocmask\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
    }
}
unsafe extern "C" fn install_sigchld() {
    let mut sa: sigaction = sigaction {
        __sigaction_handler: C2RustUnnamed_11 {
            sa_handler: None,
        },
        sa_mask: sigset_t { __val: [0; 16] },
        sa_flags: 0,
        sa_restorer: None,
    };
    sigemptyset(&mut sa.sa_mask);
    sa
        .__sigaction_handler
        .sa_handler = Some(chld as unsafe extern "C" fn(libc::c_int) -> ());
    sa.sa_flags = 0x10000000 as libc::c_int;
    sigaction(17 as libc::c_int, &mut sa, 0 as *mut sigaction);
    unblock_signal(17 as libc::c_int);
}
unsafe extern "C" fn install_cleanup(mut sigterm: libc::c_int) {
    let mut sa: sigaction = sigaction {
        __sigaction_handler: C2RustUnnamed_11 {
            sa_handler: None,
        },
        sa_mask: sigset_t { __val: [0; 16] },
        sa_flags: 0,
        sa_restorer: None,
    };
    sigemptyset(&mut sa.sa_mask);
    sa
        .__sigaction_handler
        .sa_handler = Some(cleanup as unsafe extern "C" fn(libc::c_int) -> ());
    sa.sa_flags = 0x10000000 as libc::c_int;
    sigaction(14 as libc::c_int, &mut sa, 0 as *mut sigaction);
    sigaction(2 as libc::c_int, &mut sa, 0 as *mut sigaction);
    sigaction(3 as libc::c_int, &mut sa, 0 as *mut sigaction);
    sigaction(1 as libc::c_int, &mut sa, 0 as *mut sigaction);
    sigaction(15 as libc::c_int, &mut sa, 0 as *mut sigaction);
    sigaction(sigterm, &mut sa, 0 as *mut sigaction);
}
unsafe extern "C" fn block_cleanup_and_chld(
    mut sigterm: libc::c_int,
    mut old_set: *mut sigset_t,
) {
    let mut block_set: sigset_t = sigset_t { __val: [0; 16] };
    sigemptyset(&mut block_set);
    sigaddset(&mut block_set, 14 as libc::c_int);
    sigaddset(&mut block_set, 2 as libc::c_int);
    sigaddset(&mut block_set, 3 as libc::c_int);
    sigaddset(&mut block_set, 1 as libc::c_int);
    sigaddset(&mut block_set, 15 as libc::c_int);
    sigaddset(&mut block_set, sigterm);
    sigaddset(&mut block_set, 17 as libc::c_int);
    if sigprocmask(0 as libc::c_int, &mut block_set, old_set) != 0 as libc::c_int {
        error(
            0 as libc::c_int,
            *__errno_location(),
            dcgettext(
                0 as *const libc::c_char,
                b"warning: sigprocmask\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
    }
}
unsafe extern "C" fn disable_core_dumps() -> bool {
    if prctl(4 as libc::c_int, 0 as libc::c_int) == 0 as libc::c_int {
        return 1 as libc::c_int != 0;
    }
    error(
        0 as libc::c_int,
        *__errno_location(),
        dcgettext(
            0 as *const libc::c_char,
            b"warning: disabling core dumps failed\0" as *const u8
                as *const libc::c_char,
            5 as libc::c_int,
        ),
    );
    return 0 as libc::c_int != 0;
}
unsafe fn main_0(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut timeout: libc::c_double = 0.;
    let mut signame: [libc::c_char; 19] = [0; 19];
    let mut c: libc::c_int = 0;
    set_program_name(*argv.offset(0 as libc::c_int as isize));
    setlocale(6 as libc::c_int, b"\0" as *const u8 as *const libc::c_char);
    bindtextdomain(
        b"coreutils\0" as *const u8 as *const libc::c_char,
        b"/usr/local/share/locale\0" as *const u8 as *const libc::c_char,
    );
    textdomain(b"coreutils\0" as *const u8 as *const libc::c_char);
    initialize_exit_failure(EXIT_CANCELED as libc::c_int);
    atexit(Some(close_stdout as unsafe extern "C" fn() -> ()));
    loop {
        c = getopt_long(
            argc,
            argv,
            b"+k:s:v\0" as *const u8 as *const libc::c_char,
            long_options.as_ptr(),
            0 as *mut libc::c_int,
        );
        if !(c != -(1 as libc::c_int)) {
            break;
        }
        match c {
            107 => {
                kill_after = parse_duration(optarg);
            }
            115 => {
                term_signal = operand2sig(optarg, signame.as_mut_ptr());
                if term_signal == -(1 as libc::c_int) {
                    usage(EXIT_CANCELED as libc::c_int);
                }
            }
            118 => {
                verbose = 1 as libc::c_int != 0;
            }
            256 => {
                foreground = 1 as libc::c_int != 0;
            }
            257 => {
                preserve_status = 1 as libc::c_int != 0;
            }
            -2 => {
                usage(0 as libc::c_int);
            }
            -3 => {
                version_etc(
                    stdout,
                    b"timeout\0" as *const u8 as *const libc::c_char,
                    b"GNU coreutils\0" as *const u8 as *const libc::c_char,
                    Version,
                    b"Padraig Brady\0" as *const u8 as *const libc::c_char,
                    0 as *mut libc::c_void as *mut libc::c_char,
                );
                exit(0 as libc::c_int);
            }
            _ => {
                usage(EXIT_CANCELED as libc::c_int);
            }
        }
    }
    if argc - optind < 2 as libc::c_int {
        usage(EXIT_CANCELED as libc::c_int);
    }
    let fresh0 = optind;
    optind = optind + 1;
    timeout = parse_duration(*argv.offset(fresh0 as isize));
    argv = argv.offset(optind as isize);
    command = *argv.offset(0 as libc::c_int as isize);
    if !foreground {
        setpgid(0 as libc::c_int, 0 as libc::c_int);
    }
    install_cleanup(term_signal);
    signal(
        21 as libc::c_int,
        ::core::mem::transmute::<
            libc::intptr_t,
            __sighandler_t,
        >(1 as libc::c_int as libc::intptr_t),
    );
    signal(
        22 as libc::c_int,
        ::core::mem::transmute::<
            libc::intptr_t,
            __sighandler_t,
        >(1 as libc::c_int as libc::intptr_t),
    );
    install_sigchld();
    monitored_pid = fork();
    if monitored_pid == -(1 as libc::c_int) {
        error(
            0 as libc::c_int,
            *__errno_location(),
            dcgettext(
                0 as *const libc::c_char,
                b"fork system call failed\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
        return EXIT_CANCELED as libc::c_int;
    } else if monitored_pid == 0 as libc::c_int {
        signal(21 as libc::c_int, None);
        signal(22 as libc::c_int, None);
        execvp(
            *argv.offset(0 as libc::c_int as isize),
            argv as *const *mut libc::c_char,
        );
        let mut exit_status: libc::c_int = if *__errno_location() == 2 as libc::c_int {
            EXIT_ENOENT as libc::c_int
        } else {
            EXIT_CANNOT_INVOKE as libc::c_int
        };
        error(
            0 as libc::c_int,
            *__errno_location(),
            dcgettext(
                0 as *const libc::c_char,
                b"failed to run command %s\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            quote(command),
        );
        return exit_status;
    } else {
        let mut wait_result: pid_t = 0;
        let mut status: libc::c_int = 0;
        unblock_signal(14 as libc::c_int);
        settimeout(timeout, 1 as libc::c_int != 0);
        let mut cleanup_set: sigset_t = sigset_t { __val: [0; 16] };
        block_cleanup_and_chld(term_signal, &mut cleanup_set);
        loop {
            wait_result = waitpid(monitored_pid, &mut status, 1 as libc::c_int);
            if !(wait_result == 0 as libc::c_int) {
                break;
            }
            sigsuspend(&mut cleanup_set);
        }
        if wait_result < 0 as libc::c_int {
            error(
                0 as libc::c_int,
                *__errno_location(),
                dcgettext(
                    0 as *const libc::c_char,
                    b"error waiting for command\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
            );
            status = EXIT_CANCELED as libc::c_int;
        } else if status & 0x7f as libc::c_int == 0 as libc::c_int {
            status = (status & 0xff00 as libc::c_int) >> 8 as libc::c_int;
        } else if ((status & 0x7f as libc::c_int) + 1 as libc::c_int) as libc::c_schar
            as libc::c_int >> 1 as libc::c_int > 0 as libc::c_int
        {
            let mut sig: libc::c_int = status & 0x7f as libc::c_int;
            if status & 0x80 as libc::c_int != 0 {
                error(
                    0 as libc::c_int,
                    0 as libc::c_int,
                    dcgettext(
                        0 as *const libc::c_char,
                        b"the monitored command dumped core\0" as *const u8
                            as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                );
            }
            if timed_out == 0 && disable_core_dumps() as libc::c_int != 0 {
                signal(sig, None);
                unblock_signal(sig);
                raise(sig);
            }
            if timed_out != 0 && sig == 9 as libc::c_int {
                preserve_status = 1 as libc::c_int != 0;
            }
            status = sig + 128 as libc::c_int;
        } else {
            error(
                0 as libc::c_int,
                0 as libc::c_int,
                dcgettext(
                    0 as *const libc::c_char,
                    b"unknown status from command (%d)\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
                status,
            );
            status = 1 as libc::c_int;
        }
        if timed_out != 0 && !preserve_status {
            status = EXIT_TIMEDOUT as libc::c_int;
        }
        return status;
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
