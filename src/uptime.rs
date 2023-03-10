#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![feature(extern_types)]
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type tm_zone;
    static mut stdout: *mut FILE;
    static mut stderr: *mut FILE;
    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn fgets_unlocked(
        __s: *mut libc::c_char,
        __n: libc::c_int,
        __stream: *mut FILE,
    ) -> *mut libc::c_char;
    fn fputs_unlocked(__s: *const libc::c_char, __stream: *mut FILE) -> libc::c_int;
    fn __overflow(_: *mut FILE, _: libc::c_int) -> libc::c_int;
    fn rpl_fclose(stream: *mut FILE) -> libc::c_int;
    fn localtime(__timer: *const time_t) -> *mut tm;
    static mut optind: libc::c_int;
    fn time(__timer: *mut time_t) -> time_t;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strncmp(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn __errno_location() -> *mut libc::c_int;
    fn quotearg_n_style_colon(
        n: libc::c_int,
        s: quoting_style,
        arg: *const libc::c_char,
    ) -> *mut libc::c_char;
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
    fn dcngettext(
        __domainname: *const libc::c_char,
        __msgid1: *const libc::c_char,
        __msgid2: *const libc::c_char,
        __n: libc::c_ulong,
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
    fn atexit(__func: Option::<unsafe extern "C" fn() -> ()>) -> libc::c_int;
    fn exit(_: libc::c_int) -> !;
    fn getloadavg(__loadavg: *mut libc::c_double, __nelem: libc::c_int) -> libc::c_int;
    fn c_strtod(
        nptr: *const libc::c_char,
        endptr: *mut *mut libc::c_char,
    ) -> libc::c_double;
    fn error(
        __status: libc::c_int,
        __errnum: libc::c_int,
        __format: *const libc::c_char,
        _: ...
    );
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
    fn quote(arg: *const libc::c_char) -> *const libc::c_char;
    fn read_utmp(
        file: *const libc::c_char,
        n_entries: *mut size_t,
        utmp_buf: *mut *mut STRUCT_UTMP,
        options: libc::c_int,
    ) -> libc::c_int;
    fn fprintftime(
        fp: *mut FILE,
        fmt: *const libc::c_char,
        tm: *const tm,
        zone: timezone_t,
        nanoseconds: libc::c_int,
    ) -> size_t;
}
pub type size_t = libc::c_ulong;
pub type __int32_t = libc::c_int;
pub type __uintmax_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __pid_t = libc::c_int;
pub type __time_t = libc::c_long;
pub type __suseconds_t = libc::c_long;
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
pub type time_t = __time_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timeval {
    pub tv_sec: __time_t,
    pub tv_usec: __suseconds_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tm {
    pub tm_sec: libc::c_int,
    pub tm_min: libc::c_int,
    pub tm_hour: libc::c_int,
    pub tm_mday: libc::c_int,
    pub tm_mon: libc::c_int,
    pub tm_year: libc::c_int,
    pub tm_wday: libc::c_int,
    pub tm_yday: libc::c_int,
    pub tm_isdst: libc::c_int,
    pub tm_gmtoff: libc::c_long,
    pub tm_zone: *const libc::c_char,
}
pub type timezone_t = *mut tm_zone;
pub type uintmax_t = __uintmax_t;
pub const PLURAL_REDUCER: C2RustUnnamed = 1000000;
pub type C2RustUnnamed = libc::c_uint;
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
pub struct __exit_status {
    pub e_termination: libc::c_short,
    pub e_exit: libc::c_short,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct utmpx {
    pub ut_type: libc::c_short,
    pub ut_pid: __pid_t,
    pub ut_line: [libc::c_char; 32],
    pub ut_id: [libc::c_char; 4],
    pub ut_user: [libc::c_char; 32],
    pub ut_host: [libc::c_char; 256],
    pub ut_exit: __exit_status,
    pub ut_session: libc::c_long,
    pub ut_tv: timeval,
    pub ut_addr_v6: [__int32_t; 4],
    pub __glibc_reserved: [libc::c_char; 20],
}
pub type STRUCT_UTMP = utmpx;
pub type C2RustUnnamed_0 = libc::c_uint;
pub const READ_UTMP_USER_PROCESS: C2RustUnnamed_0 = 2;
pub const READ_UTMP_CHECK_PIDS: C2RustUnnamed_0 = 1;
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
unsafe extern "C" fn select_plural(mut n: uintmax_t) -> libc::c_ulong {
    return if n
        <= (9223372036854775807 as libc::c_long as libc::c_ulong)
            .wrapping_mul(2 as libc::c_ulong)
            .wrapping_add(1 as libc::c_ulong)
    {
        n
    } else {
        n.wrapping_rem(PLURAL_REDUCER as libc::c_int as libc::c_ulong)
            .wrapping_add(PLURAL_REDUCER as libc::c_int as libc::c_ulong)
    };
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
unsafe extern "C" fn print_uptime(mut n: size_t, mut this: *const STRUCT_UTMP) {
    let mut entries: size_t = 0 as libc::c_int as size_t;
    let mut boot_time: time_t = 0 as libc::c_int as time_t;
    let mut time_now: time_t = 0;
    let mut uptime_0: time_t = 0 as libc::c_int as time_t;
    let mut updays: libc::c_long = 0;
    let mut uphours: libc::c_int = 0;
    let mut upmins: libc::c_int = 0;
    let mut tmn: *mut tm = 0 as *mut tm;
    let mut avg: [libc::c_double; 3] = [0.; 3];
    let mut loads: libc::c_int = 0;
    let mut fp: *mut FILE = 0 as *mut FILE;
    fp = fopen(
        b"/proc/uptime\0" as *const u8 as *const libc::c_char,
        b"r\0" as *const u8 as *const libc::c_char,
    );
    if !fp.is_null() {
        let mut buf: [libc::c_char; 8192] = [0; 8192];
        let mut b: *mut libc::c_char = fgets_unlocked(
            buf.as_mut_ptr(),
            8192 as libc::c_int,
            fp,
        );
        if b == buf.as_mut_ptr() {
            let mut end_ptr: *mut libc::c_char = 0 as *mut libc::c_char;
            let mut upsecs: libc::c_double = c_strtod(buf.as_mut_ptr(), &mut end_ptr);
            if buf.as_mut_ptr() != end_ptr {
                uptime_0 = (if 0 as libc::c_int as libc::c_double <= upsecs
                    && upsecs
                        < (if (0 as libc::c_int as time_t)
                            < -(1 as libc::c_int) as time_t
                        {
                            -(1 as libc::c_int) as time_t
                        } else {
                            (((1 as libc::c_int as time_t)
                                << (::core::mem::size_of::<time_t>() as libc::c_ulong)
                                    .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                    .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                - 1 as libc::c_int as libc::c_long)
                                * 2 as libc::c_int as libc::c_long
                                + 1 as libc::c_int as libc::c_long
                        }) as libc::c_double
                {
                    upsecs
                } else {
                    -(1 as libc::c_int) as libc::c_double
                }) as time_t;
            }
        }
        rpl_fclose(fp);
    }
    loop {
        let fresh1 = n;
        n = n.wrapping_sub(1);
        if !(fresh1 != 0) {
            break;
        }
        entries = (entries as libc::c_ulong)
            .wrapping_add(
                ((*this).ut_user[0 as libc::c_int as usize] as libc::c_int != 0
                    && ((*this).ut_type as libc::c_int == 7 as libc::c_int
                        || 0 as libc::c_int != 0
                            && (*this).ut_tv.tv_sec != 0 as libc::c_int as libc::c_long))
                    as libc::c_int as libc::c_ulong,
            ) as size_t as size_t;
        if (*this).ut_type as libc::c_int == 2 as libc::c_int {
            boot_time = (*this).ut_tv.tv_sec;
        }
        this = this.offset(1);
    }
    time_now = time(0 as *mut time_t);
    if uptime_0 == 0 as libc::c_int as libc::c_long {
        if boot_time == 0 as libc::c_int as libc::c_long {
            if ::core::mem::size_of::<C2RustUnnamed_1>() as libc::c_ulong != 0 {
                error(
                    1 as libc::c_int,
                    *__errno_location(),
                    dcgettext(
                        0 as *const libc::c_char,
                        b"couldn't get boot time\0" as *const u8 as *const libc::c_char,
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
                        b"couldn't get boot time\0" as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                );
                if 0 as libc::c_int != 0 {} else {
                    unreachable!();
                };
            };
        }
        uptime_0 = time_now - boot_time;
    }
    updays = uptime_0 / 86400 as libc::c_int as libc::c_long;
    uphours = ((uptime_0 - updays * 86400 as libc::c_int as libc::c_long)
        / 3600 as libc::c_int as libc::c_long) as libc::c_int;
    upmins = ((uptime_0 - updays * 86400 as libc::c_int as libc::c_long
        - (uphours * 3600 as libc::c_int) as libc::c_long)
        / 60 as libc::c_int as libc::c_long) as libc::c_int;
    tmn = localtime(&mut time_now);
    if !tmn.is_null() {
        fprintftime(
            stdout,
            dcgettext(
                0 as *const libc::c_char,
                b" %H:%M:%S  \0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            tmn,
            0 as timezone_t,
            0 as libc::c_int,
        );
    } else {
        printf(
            dcgettext(
                0 as *const libc::c_char,
                b" ??:????  \0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
    }
    if uptime_0 == -(1 as libc::c_int) as time_t {
        printf(
            dcgettext(
                0 as *const libc::c_char,
                b"up ???? days ??:??,  \0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
    } else if (0 as libc::c_int as libc::c_long) < updays {
        printf(
            dcngettext(
                0 as *const libc::c_char,
                b"up %ld day %2d:%02d,  \0" as *const u8 as *const libc::c_char,
                b"up %ld days %2d:%02d,  \0" as *const u8 as *const libc::c_char,
                select_plural(updays as uintmax_t),
                5 as libc::c_int,
            ),
            updays,
            uphours,
            upmins,
        );
    } else {
        printf(
            dcgettext(
                0 as *const libc::c_char,
                b"up  %2d:%02d,  \0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            uphours,
            upmins,
        );
    }
    printf(
        dcngettext(
            0 as *const libc::c_char,
            b"%lu user\0" as *const u8 as *const libc::c_char,
            b"%lu users\0" as *const u8 as *const libc::c_char,
            select_plural(entries),
            5 as libc::c_int,
        ),
        entries,
    );
    loads = getloadavg(avg.as_mut_ptr(), 3 as libc::c_int);
    if loads == -(1 as libc::c_int) {
        putchar_unlocked('\n' as i32);
    } else {
        if loads > 0 as libc::c_int {
            printf(
                dcgettext(
                    0 as *const libc::c_char,
                    b",  load average: %.2f\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
                avg[0 as libc::c_int as usize],
            );
        }
        if loads > 1 as libc::c_int {
            printf(
                b", %.2f\0" as *const u8 as *const libc::c_char,
                avg[1 as libc::c_int as usize],
            );
        }
        if loads > 2 as libc::c_int {
            printf(
                b", %.2f\0" as *const u8 as *const libc::c_char,
                avg[2 as libc::c_int as usize],
            );
        }
        if loads > 0 as libc::c_int {
            putchar_unlocked('\n' as i32);
        }
    };
}
unsafe extern "C" fn uptime(
    mut filename: *const libc::c_char,
    mut options: libc::c_int,
) {
    let mut n_users: size_t = 0;
    let mut utmp_buf: *mut STRUCT_UTMP = 0 as *mut STRUCT_UTMP;
    if read_utmp(filename, &mut n_users, &mut utmp_buf, options) != 0 as libc::c_int {
        if ::core::mem::size_of::<C2RustUnnamed_2>() as libc::c_ulong != 0 {
            error(
                1 as libc::c_int,
                *__errno_location(),
                b"%s\0" as *const u8 as *const libc::c_char,
                quotearg_n_style_colon(
                    0 as libc::c_int,
                    shell_escape_quoting_style,
                    filename,
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
                    filename,
                ),
            );
            if 0 as libc::c_int != 0 {} else {
                unreachable!();
            };
        };
    }
    print_uptime(n_users, utmp_buf);
    exit(0 as libc::c_int);
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
                b"Usage: %s [OPTION]... [FILE]\n\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            program_name,
        );
        printf(
            dcgettext(
                0 as *const libc::c_char,
                b"Print the current time, the length of time the system has been up,\nthe number of users on the system, and the average number of jobs\nin the run queue over the last 1, 5 and 15 minutes.\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
        printf(
            dcgettext(
                0 as *const libc::c_char,
                b"  Processes in\nan uninterruptible sleep state also contribute to the load average.\n\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
        printf(
            dcgettext(
                0 as *const libc::c_char,
                b"If FILE is not specified, use %s.  %s as FILE is common.\n\n\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            b"/var/run/utmp\0" as *const u8 as *const libc::c_char,
            b"/var/log/wtmp\0" as *const u8 as *const libc::c_char,
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
        emit_ancillary_info(b"uptime\0" as *const u8 as *const libc::c_char);
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
    parse_gnu_standard_options_only(
        argc,
        argv,
        b"uptime\0" as *const u8 as *const libc::c_char,
        b"GNU coreutils\0" as *const u8 as *const libc::c_char,
        Version,
        1 as libc::c_int != 0,
        Some(usage as unsafe extern "C" fn(libc::c_int) -> ()),
        b"Joseph Arceneaux\0" as *const u8 as *const libc::c_char,
        b"David MacKenzie\0" as *const u8 as *const libc::c_char,
        b"Kaveh Ghazi\0" as *const u8 as *const libc::c_char,
        0 as *mut libc::c_void as *const libc::c_char,
    );
    match argc - optind {
        0 => {
            uptime(
                b"/var/run/utmp\0" as *const u8 as *const libc::c_char,
                READ_UTMP_CHECK_PIDS as libc::c_int,
            );
        }
        1 => {
            uptime(*argv.offset(optind as isize), 0 as libc::c_int);
        }
        _ => {
            error(
                0 as libc::c_int,
                0 as libc::c_int,
                dcgettext(
                    0 as *const libc::c_char,
                    b"extra operand %s\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
                quote(*argv.offset((optind + 1 as libc::c_int) as isize)),
            );
            usage(1 as libc::c_int);
        }
    }
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
