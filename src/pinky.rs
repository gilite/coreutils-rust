#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![feature(extern_types)]
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    static mut optind: libc::c_int;
    fn getopt_long(
        ___argc: libc::c_int,
        ___argv: *const *mut libc::c_char,
        __shortopts: *const libc::c_char,
        __longopts: *const option,
        __longind: *mut libc::c_int,
    ) -> libc::c_int;
    fn getpwnam(__name: *const libc::c_char) -> *mut passwd;
    static mut stdout: *mut FILE;
    static mut stderr: *mut FILE;
    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn fputs_unlocked(__s: *const libc::c_char, __stream: *mut FILE) -> libc::c_int;
    fn __overflow(_: *mut FILE, _: libc::c_int) -> libc::c_int;
    fn __uflow(_: *mut FILE) -> libc::c_int;
    fn fread_unlocked(
        __ptr: *mut libc::c_void,
        __size: size_t,
        __n: size_t,
        __stream: *mut FILE,
    ) -> size_t;
    fn fwrite_unlocked(
        __ptr: *const libc::c_void,
        __size: size_t,
        __n: size_t,
        __stream: *mut FILE,
    ) -> size_t;
    fn rpl_fclose(stream: *mut FILE) -> libc::c_int;
    fn stat(__file: *const libc::c_char, __buf: *mut stat) -> libc::c_int;
    fn localtime(__timer: *const time_t) -> *mut tm;
    fn strftime(
        __s: *mut libc::c_char,
        __maxsize: size_t,
        __format: *const libc::c_char,
        __tp: *const tm,
    ) -> size_t;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strncmp(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn stpcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn free(_: *mut libc::c_void);
    fn __errno_location() -> *mut libc::c_int;
    fn atexit(__func: Option::<unsafe extern "C" fn() -> ()>) -> libc::c_int;
    fn exit(_: libc::c_int) -> !;
    fn quotearg_n_style_colon(
        n: libc::c_int,
        s: quoting_style,
        arg: *const libc::c_char,
    ) -> *mut libc::c_char;
    static mut Version: *const libc::c_char;
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
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
    fn time(__timer: *mut time_t) -> time_t;
    fn xalloc_die();
    fn xmalloc(s: size_t) -> *mut libc::c_void;
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
    fn imaxtostr(_: intmax_t, _: *mut libc::c_char) -> *mut libc::c_char;
    fn umaxtostr(_: uintmax_t, _: *mut libc::c_char) -> *mut libc::c_char;
    fn canon_host(host: *const libc::c_char) -> *mut libc::c_char;
    fn error(
        __status: libc::c_int,
        __errnum: libc::c_int,
        __format: *const libc::c_char,
        _: ...
    );
    fn hard_locale(category: libc::c_int) -> bool;
    fn read_utmp(
        file: *const libc::c_char,
        n_entries: *mut size_t,
        utmp_buf: *mut *mut STRUCT_UTMP,
        options: libc::c_int,
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
pub type __int32_t = libc::c_int;
pub type __intmax_t = libc::c_long;
pub type __uintmax_t = libc::c_ulong;
pub type __dev_t = libc::c_ulong;
pub type __uid_t = libc::c_uint;
pub type __gid_t = libc::c_uint;
pub type __ino_t = libc::c_ulong;
pub type __mode_t = libc::c_uint;
pub type __nlink_t = libc::c_uint;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __pid_t = libc::c_int;
pub type __time_t = libc::c_long;
pub type __suseconds_t = libc::c_long;
pub type __blksize_t = libc::c_int;
pub type __blkcnt_t = libc::c_long;
pub type __syscall_slong_t = libc::c_long;
pub type size_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct passwd {
    pub pw_name: *mut libc::c_char,
    pub pw_passwd: *mut libc::c_char,
    pub pw_uid: __uid_t,
    pub pw_gid: __gid_t,
    pub pw_gecos: *mut libc::c_char,
    pub pw_dir: *mut libc::c_char,
    pub pw_shell: *mut libc::c_char,
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
pub type time_t = __time_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timeval {
    pub tv_sec: __time_t,
    pub tv_usec: __suseconds_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
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
pub type intmax_t = __intmax_t;
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
pub type C2RustUnnamed_1 = libc::c_uint;
pub const UT_USER_SIZE: C2RustUnnamed_1 = 32;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_2 {
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
unsafe extern "C" fn getc_unlocked(mut __fp: *mut FILE) -> libc::c_int {
    return if ((*__fp)._IO_read_ptr >= (*__fp)._IO_read_end) as libc::c_int
        as libc::c_long != 0
    {
        __uflow(__fp)
    } else {
        let fresh2 = (*__fp)._IO_read_ptr;
        (*__fp)._IO_read_ptr = ((*__fp)._IO_read_ptr).offset(1);
        *(fresh2 as *mut libc::c_uchar) as libc::c_int
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
unsafe extern "C" fn timetostr(
    mut t: time_t,
    mut buf: *mut libc::c_char,
) -> *mut libc::c_char {
    return if !((0 as libc::c_int as time_t) < -(1 as libc::c_int) as time_t) {
        imaxtostr(t, buf)
    } else {
        umaxtostr(t as uintmax_t, buf)
    };
}
#[inline]
unsafe extern "C" fn stzncpy(
    mut dest: *mut libc::c_char,
    mut src: *const libc::c_char,
    mut len: size_t,
) -> *mut libc::c_char {
    let mut src_end: *const libc::c_char = src.offset(len as isize);
    while src < src_end && *src as libc::c_int != 0 {
        let fresh3 = src;
        src = src.offset(1);
        let fresh4 = dest;
        dest = dest.offset(1);
        *fresh4 = *fresh3;
    }
    *dest = 0 as libc::c_int as libc::c_char;
    return dest;
}
static mut include_idle: bool = 1 as libc::c_int != 0;
static mut include_heading: bool = 1 as libc::c_int != 0;
static mut include_fullname: bool = 1 as libc::c_int != 0;
static mut include_project: bool = 1 as libc::c_int != 0;
static mut include_plan: bool = 1 as libc::c_int != 0;
static mut include_home_and_shell: bool = 1 as libc::c_int != 0;
static mut do_short_format: bool = 1 as libc::c_int != 0;
static mut include_where: bool = 1 as libc::c_int != 0;
static mut time_format: *const libc::c_char = 0 as *const libc::c_char;
static mut time_format_width: libc::c_int = 0;
static mut longopts: [option; 3] = [
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
unsafe extern "C" fn count_ampersands(mut str: *const libc::c_char) -> size_t {
    let mut count: size_t = 0 as libc::c_int as size_t;
    loop {
        if *str as libc::c_int == '&' as i32 {
            count = count.wrapping_add(1);
        }
        let fresh5 = str;
        str = str.offset(1);
        if !(*fresh5 != 0) {
            break;
        }
    }
    return count;
}
unsafe extern "C" fn create_fullname(
    mut gecos_name: *const libc::c_char,
    mut user_name: *const libc::c_char,
) -> *mut libc::c_char {
    let mut rsize: size_t = (strlen(gecos_name))
        .wrapping_add(1 as libc::c_int as libc::c_ulong);
    let mut result: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut r: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut ampersands: size_t = count_ampersands(gecos_name);
    if ampersands != 0 as libc::c_int as libc::c_ulong {
        let mut ulen: size_t = strlen(user_name);
        let mut product: size_t = 0;
        if (if (0 as libc::c_int as size_t) < -(1 as libc::c_int) as size_t
            && (if 1 as libc::c_int != 0 {
                0 as libc::c_int as libc::c_ulong
            } else {
                ulen
            })
                .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                < 0 as libc::c_int as libc::c_ulong
            && (if 1 as libc::c_int != 0 {
                0 as libc::c_int as libc::c_ulong
            } else {
                ampersands.wrapping_sub(1 as libc::c_int as libc::c_ulong)
            })
                .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                < 0 as libc::c_int as libc::c_ulong
            && (if ampersands.wrapping_sub(1 as libc::c_int as libc::c_ulong)
                < 0 as libc::c_int as libc::c_ulong
            {
                (if ulen < 0 as libc::c_int as libc::c_ulong {
                    (if (if 1 as libc::c_int != 0 {
                        0 as libc::c_int as libc::c_ulong
                    } else {
                        (if 1 as libc::c_int != 0 {
                            0 as libc::c_int as libc::c_ulong
                        } else {
                            -(1 as libc::c_int) as size_t
                        })
                            .wrapping_add(
                                ampersands.wrapping_sub(1 as libc::c_int as libc::c_ulong),
                            )
                    })
                        .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                        < 0 as libc::c_int as libc::c_ulong
                    {
                        (ulen
                            < (-(1 as libc::c_int) as size_t)
                                .wrapping_div(
                                    ampersands.wrapping_sub(1 as libc::c_int as libc::c_ulong),
                                )) as libc::c_int
                    } else {
                        ((if (if (if (if 1 as libc::c_int != 0 {
                            0 as libc::c_int as libc::c_ulong
                        } else {
                            ampersands.wrapping_sub(1 as libc::c_int as libc::c_ulong)
                        })
                            .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                            < 0 as libc::c_int as libc::c_ulong
                        {
                            !((if 1 as libc::c_int != 0 {
                                0 as libc::c_int as libc::c_ulong
                            } else {
                                ampersands.wrapping_sub(1 as libc::c_int as libc::c_ulong)
                            })
                                .wrapping_add(1 as libc::c_int as libc::c_ulong)
                                << (::core::mem::size_of::<libc::c_ulong>()
                                    as libc::c_ulong)
                                    .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                    .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                .wrapping_mul(2 as libc::c_int as libc::c_ulong)
                                .wrapping_add(1 as libc::c_int as libc::c_ulong)
                        } else {
                            (if 1 as libc::c_int != 0 {
                                0 as libc::c_int as libc::c_ulong
                            } else {
                                ampersands.wrapping_sub(1 as libc::c_int as libc::c_ulong)
                            })
                                .wrapping_add(0 as libc::c_int as libc::c_ulong)
                        }) < 0 as libc::c_int as libc::c_ulong
                        {
                            (ampersands.wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                < (if (if 1 as libc::c_int != 0 {
                                    0 as libc::c_int as libc::c_ulong
                                } else {
                                    ampersands.wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                })
                                    .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                    < 0 as libc::c_int as libc::c_ulong
                                {
                                    ((if 1 as libc::c_int != 0 {
                                        0 as libc::c_int as libc::c_ulong
                                    } else {
                                        ampersands.wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                    })
                                        .wrapping_add(1 as libc::c_int as libc::c_ulong)
                                        << (::core::mem::size_of::<libc::c_ulong>()
                                            as libc::c_ulong)
                                            .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                            .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                        .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                        .wrapping_mul(2 as libc::c_int as libc::c_ulong)
                                        .wrapping_add(1 as libc::c_int as libc::c_ulong)
                                } else {
                                    (if 1 as libc::c_int != 0 {
                                        0 as libc::c_int as libc::c_ulong
                                    } else {
                                        ampersands.wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                    })
                                        .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                })
                                    .wrapping_neg()) as libc::c_int
                        } else {
                            ((0 as libc::c_int as libc::c_ulong)
                                < ampersands
                                    .wrapping_sub(1 as libc::c_int as libc::c_ulong))
                                as libc::c_int
                        }) != 0
                        {
                            (if 1 as libc::c_int != 0 {
                                0 as libc::c_int as libc::c_ulong
                            } else {
                                ampersands.wrapping_sub(1 as libc::c_int as libc::c_ulong)
                            })
                                .wrapping_add(-(1 as libc::c_int) as size_t)
                                >> (::core::mem::size_of::<libc::c_ulong>()
                                    as libc::c_ulong)
                                    .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                    .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                        } else {
                            (-(1 as libc::c_int) as size_t)
                                .wrapping_div(
                                    ampersands
                                        .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                        .wrapping_neg(),
                                )
                        }) <= (-(1 as libc::c_int) as libc::c_ulong).wrapping_sub(ulen))
                            as libc::c_int
                    })
                } else {
                    (if (if (if (if 1 as libc::c_int != 0 {
                        0 as libc::c_int as libc::c_ulong
                    } else {
                        (if 1 as libc::c_int != 0 {
                            0 as libc::c_int as libc::c_ulong
                        } else {
                            ampersands.wrapping_sub(1 as libc::c_int as libc::c_ulong)
                        })
                            .wrapping_add(0 as libc::c_int as libc::c_ulong)
                    })
                        .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                        < 0 as libc::c_int as libc::c_ulong
                    {
                        !((if 1 as libc::c_int != 0 {
                            0 as libc::c_int as libc::c_ulong
                        } else {
                            (if 1 as libc::c_int != 0 {
                                0 as libc::c_int as libc::c_ulong
                            } else {
                                ampersands.wrapping_sub(1 as libc::c_int as libc::c_ulong)
                            })
                                .wrapping_add(0 as libc::c_int as libc::c_ulong)
                        })
                            .wrapping_add(1 as libc::c_int as libc::c_ulong)
                            << (::core::mem::size_of::<libc::c_ulong>() as libc::c_ulong)
                                .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                            .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(2 as libc::c_int as libc::c_ulong)
                            .wrapping_add(1 as libc::c_int as libc::c_ulong)
                    } else {
                        (if 1 as libc::c_int != 0 {
                            0 as libc::c_int as libc::c_ulong
                        } else {
                            (if 1 as libc::c_int != 0 {
                                0 as libc::c_int as libc::c_ulong
                            } else {
                                ampersands.wrapping_sub(1 as libc::c_int as libc::c_ulong)
                            })
                                .wrapping_add(0 as libc::c_int as libc::c_ulong)
                        })
                            .wrapping_add(0 as libc::c_int as libc::c_ulong)
                    }) < 0 as libc::c_int as libc::c_ulong
                    {
                        ((if 1 as libc::c_int != 0 {
                            0 as libc::c_int as libc::c_ulong
                        } else {
                            ampersands.wrapping_sub(1 as libc::c_int as libc::c_ulong)
                        })
                            .wrapping_add(0 as libc::c_int as libc::c_ulong)
                            < (if (if 1 as libc::c_int != 0 {
                                0 as libc::c_int as libc::c_ulong
                            } else {
                                (if 1 as libc::c_int != 0 {
                                    0 as libc::c_int as libc::c_ulong
                                } else {
                                    ampersands.wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                })
                                    .wrapping_add(0 as libc::c_int as libc::c_ulong)
                            })
                                .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                < 0 as libc::c_int as libc::c_ulong
                            {
                                ((if 1 as libc::c_int != 0 {
                                    0 as libc::c_int as libc::c_ulong
                                } else {
                                    (if 1 as libc::c_int != 0 {
                                        0 as libc::c_int as libc::c_ulong
                                    } else {
                                        ampersands.wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                    })
                                        .wrapping_add(0 as libc::c_int as libc::c_ulong)
                                })
                                    .wrapping_add(1 as libc::c_int as libc::c_ulong)
                                    << (::core::mem::size_of::<libc::c_ulong>()
                                        as libc::c_ulong)
                                        .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                        .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                    .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                    .wrapping_mul(2 as libc::c_int as libc::c_ulong)
                                    .wrapping_add(1 as libc::c_int as libc::c_ulong)
                            } else {
                                (if 1 as libc::c_int != 0 {
                                    0 as libc::c_int as libc::c_ulong
                                } else {
                                    (if 1 as libc::c_int != 0 {
                                        0 as libc::c_int as libc::c_ulong
                                    } else {
                                        ampersands.wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                    })
                                        .wrapping_add(0 as libc::c_int as libc::c_ulong)
                                })
                                    .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                            })
                                .wrapping_neg()) as libc::c_int
                    } else {
                        ((0 as libc::c_int as libc::c_ulong)
                            < (if 1 as libc::c_int != 0 {
                                0 as libc::c_int as libc::c_ulong
                            } else {
                                ampersands.wrapping_sub(1 as libc::c_int as libc::c_ulong)
                            })
                                .wrapping_add(0 as libc::c_int as libc::c_ulong))
                            as libc::c_int
                    }) != 0
                        && ampersands.wrapping_sub(1 as libc::c_int as libc::c_ulong)
                            == -(1 as libc::c_int) as libc::c_ulong
                    {
                        (if (if 1 as libc::c_int != 0 {
                            0 as libc::c_int as libc::c_ulong
                        } else {
                            ulen
                        })
                            .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                            < 0 as libc::c_int as libc::c_ulong
                        {
                            ((0 as libc::c_int as libc::c_ulong)
                                < ulen.wrapping_add(0 as libc::c_int as libc::c_ulong))
                                as libc::c_int
                        } else {
                            ((0 as libc::c_int as libc::c_ulong) < ulen
                                && ((-(1 as libc::c_int) - 0 as libc::c_int)
                                    as libc::c_ulong)
                                    < ulen.wrapping_sub(1 as libc::c_int as libc::c_ulong))
                                as libc::c_int
                        })
                    } else {
                        ((0 as libc::c_int as libc::c_ulong)
                            .wrapping_div(
                                ampersands.wrapping_sub(1 as libc::c_int as libc::c_ulong),
                            ) < ulen) as libc::c_int
                    })
                })
            } else {
                (if ampersands.wrapping_sub(1 as libc::c_int as libc::c_ulong)
                    == 0 as libc::c_int as libc::c_ulong
                {
                    0 as libc::c_int
                } else {
                    (if ulen < 0 as libc::c_int as libc::c_ulong {
                        (if (if (if (if 1 as libc::c_int != 0 {
                            0 as libc::c_int as libc::c_ulong
                        } else {
                            (if 1 as libc::c_int != 0 {
                                0 as libc::c_int as libc::c_ulong
                            } else {
                                ulen
                            })
                                .wrapping_add(0 as libc::c_int as libc::c_ulong)
                        })
                            .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                            < 0 as libc::c_int as libc::c_ulong
                        {
                            !((if 1 as libc::c_int != 0 {
                                0 as libc::c_int as libc::c_ulong
                            } else {
                                (if 1 as libc::c_int != 0 {
                                    0 as libc::c_int as libc::c_ulong
                                } else {
                                    ulen
                                })
                                    .wrapping_add(0 as libc::c_int as libc::c_ulong)
                            })
                                .wrapping_add(1 as libc::c_int as libc::c_ulong)
                                << (::core::mem::size_of::<libc::c_ulong>()
                                    as libc::c_ulong)
                                    .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                    .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                .wrapping_mul(2 as libc::c_int as libc::c_ulong)
                                .wrapping_add(1 as libc::c_int as libc::c_ulong)
                        } else {
                            (if 1 as libc::c_int != 0 {
                                0 as libc::c_int as libc::c_ulong
                            } else {
                                (if 1 as libc::c_int != 0 {
                                    0 as libc::c_int as libc::c_ulong
                                } else {
                                    ulen
                                })
                                    .wrapping_add(0 as libc::c_int as libc::c_ulong)
                            })
                                .wrapping_add(0 as libc::c_int as libc::c_ulong)
                        }) < 0 as libc::c_int as libc::c_ulong
                        {
                            ((if 1 as libc::c_int != 0 {
                                0 as libc::c_int as libc::c_ulong
                            } else {
                                ulen
                            })
                                .wrapping_add(0 as libc::c_int as libc::c_ulong)
                                < (if (if 1 as libc::c_int != 0 {
                                    0 as libc::c_int as libc::c_ulong
                                } else {
                                    (if 1 as libc::c_int != 0 {
                                        0 as libc::c_int as libc::c_ulong
                                    } else {
                                        ulen
                                    })
                                        .wrapping_add(0 as libc::c_int as libc::c_ulong)
                                })
                                    .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                    < 0 as libc::c_int as libc::c_ulong
                                {
                                    ((if 1 as libc::c_int != 0 {
                                        0 as libc::c_int as libc::c_ulong
                                    } else {
                                        (if 1 as libc::c_int != 0 {
                                            0 as libc::c_int as libc::c_ulong
                                        } else {
                                            ulen
                                        })
                                            .wrapping_add(0 as libc::c_int as libc::c_ulong)
                                    })
                                        .wrapping_add(1 as libc::c_int as libc::c_ulong)
                                        << (::core::mem::size_of::<libc::c_ulong>()
                                            as libc::c_ulong)
                                            .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                            .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                        .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                        .wrapping_mul(2 as libc::c_int as libc::c_ulong)
                                        .wrapping_add(1 as libc::c_int as libc::c_ulong)
                                } else {
                                    (if 1 as libc::c_int != 0 {
                                        0 as libc::c_int as libc::c_ulong
                                    } else {
                                        (if 1 as libc::c_int != 0 {
                                            0 as libc::c_int as libc::c_ulong
                                        } else {
                                            ulen
                                        })
                                            .wrapping_add(0 as libc::c_int as libc::c_ulong)
                                    })
                                        .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                })
                                    .wrapping_neg()) as libc::c_int
                        } else {
                            ((0 as libc::c_int as libc::c_ulong)
                                < (if 1 as libc::c_int != 0 {
                                    0 as libc::c_int as libc::c_ulong
                                } else {
                                    ulen
                                })
                                    .wrapping_add(0 as libc::c_int as libc::c_ulong))
                                as libc::c_int
                        }) != 0 && ulen == -(1 as libc::c_int) as libc::c_ulong
                        {
                            (if (if 1 as libc::c_int != 0 {
                                0 as libc::c_int as libc::c_ulong
                            } else {
                                ampersands.wrapping_sub(1 as libc::c_int as libc::c_ulong)
                            })
                                .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                < 0 as libc::c_int as libc::c_ulong
                            {
                                ((0 as libc::c_int as libc::c_ulong)
                                    < ampersands
                                        .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                        .wrapping_add(0 as libc::c_int as libc::c_ulong))
                                    as libc::c_int
                            } else {
                                (((-(1 as libc::c_int) - 0 as libc::c_int) as libc::c_ulong)
                                    < ampersands
                                        .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                        .wrapping_sub(1 as libc::c_int as libc::c_ulong))
                                    as libc::c_int
                            })
                        } else {
                            ((0 as libc::c_int as libc::c_ulong).wrapping_div(ulen)
                                < ampersands
                                    .wrapping_sub(1 as libc::c_int as libc::c_ulong))
                                as libc::c_int
                        })
                    } else {
                        ((-(1 as libc::c_int) as size_t)
                            .wrapping_div(
                                ampersands.wrapping_sub(1 as libc::c_int as libc::c_ulong),
                            ) < ulen) as libc::c_int
                    })
                })
            }) != 0
        {
            let (fresh6, fresh7) = ulen
                .overflowing_mul(
                    ampersands.wrapping_sub(1 as libc::c_int as libc::c_ulong),
                );
            *(&mut product as *mut size_t) = fresh6;
            1 as libc::c_int
        } else {
            let (fresh8, fresh9) = ulen
                .overflowing_mul(
                    ampersands.wrapping_sub(1 as libc::c_int as libc::c_ulong),
                );
            *(&mut product as *mut size_t) = fresh8;
            fresh9 as libc::c_int
        }) != 0
            || {
                let (fresh10, fresh11) = rsize.overflowing_add(product);
                *(&mut rsize as *mut size_t) = fresh10;
                fresh11 as libc::c_int != 0
            }
        {
            xalloc_die();
        }
    }
    result = xmalloc(rsize) as *mut libc::c_char;
    r = result;
    while *gecos_name != 0 {
        if *gecos_name as libc::c_int == '&' as i32 {
            let mut uname: *const libc::c_char = user_name;
            if *(*__ctype_b_loc()).offset(to_uchar(*uname) as libc::c_int as isize)
                as libc::c_int & _ISlower as libc::c_int as libc::c_ushort as libc::c_int
                != 0
            {
                let fresh15 = r;
                r = r.offset(1);
                *fresh15 = ({
                    let mut __res: libc::c_int = 0;
                    if ::core::mem::size_of::<libc::c_uchar>() as libc::c_ulong
                        > 1 as libc::c_int as libc::c_ulong
                    {
                        if 0 != 0 {
                            let fresh12 = uname;
                            uname = uname.offset(1);
                            let mut __c: libc::c_int = to_uchar(*fresh12) as libc::c_int;
                            __res = if __c < -(128 as libc::c_int)
                                || __c > 255 as libc::c_int
                            {
                                __c
                            } else {
                                *(*__ctype_toupper_loc()).offset(__c as isize)
                            };
                        } else {
                            let fresh13 = uname;
                            uname = uname.offset(1);
                            __res = toupper(to_uchar(*fresh13) as libc::c_int);
                        }
                    } else {
                        let fresh14 = uname;
                        uname = uname.offset(1);
                        __res = *(*__ctype_toupper_loc())
                            .offset(to_uchar(*fresh14) as libc::c_int as isize);
                    }
                    __res
                }) as libc::c_char;
            }
            while *uname != 0 {
                let fresh16 = uname;
                uname = uname.offset(1);
                let fresh17 = r;
                r = r.offset(1);
                *fresh17 = *fresh16;
            }
        } else {
            let fresh18 = r;
            r = r.offset(1);
            *fresh18 = *gecos_name;
        }
        gecos_name = gecos_name.offset(1);
    }
    *r = 0 as libc::c_int as libc::c_char;
    return result;
}
unsafe extern "C" fn idle_string(mut when: time_t) -> *const libc::c_char {
    static mut now: time_t = 0 as libc::c_int as time_t;
    static mut buf: [libc::c_char; 22] = [0; 22];
    let mut seconds_idle: time_t = 0;
    if now == 0 as libc::c_int as libc::c_long {
        time(&mut now);
    }
    seconds_idle = now - when;
    if seconds_idle < 60 as libc::c_int as libc::c_long {
        return b"     \0" as *const u8 as *const libc::c_char;
    }
    if seconds_idle
        < (24 as libc::c_int * 60 as libc::c_int * 60 as libc::c_int) as libc::c_long
    {
        let mut hours: libc::c_int = (seconds_idle
            / (60 as libc::c_int * 60 as libc::c_int) as libc::c_long) as libc::c_int;
        let mut minutes: libc::c_int = (seconds_idle
            % (60 as libc::c_int * 60 as libc::c_int) as libc::c_long
            / 60 as libc::c_int as libc::c_long) as libc::c_int;
        sprintf(
            buf.as_mut_ptr(),
            b"%02d:%02d\0" as *const u8 as *const libc::c_char,
            hours,
            minutes,
        );
    } else {
        let mut days: libc::c_ulong = (seconds_idle
            / (24 as libc::c_int * 60 as libc::c_int * 60 as libc::c_int)
                as libc::c_long) as libc::c_ulong;
        sprintf(buf.as_mut_ptr(), b"%lud\0" as *const u8 as *const libc::c_char, days);
    }
    return buf.as_mut_ptr();
}
unsafe extern "C" fn time_string(
    mut utmp_ent: *const STRUCT_UTMP,
) -> *const libc::c_char {
    static mut buf: [libc::c_char; 33] = [0; 33];
    let mut t: time_t = (*utmp_ent).ut_tv.tv_sec;
    let mut tmp: *mut tm = localtime(&mut t);
    if !tmp.is_null() {
        strftime(
            buf.as_mut_ptr(),
            ::core::mem::size_of::<[libc::c_char; 33]>() as libc::c_ulong,
            time_format,
            tmp,
        );
        return buf.as_mut_ptr();
    } else {
        return timetostr(t, buf.as_mut_ptr())
    };
}
unsafe extern "C" fn print_entry(mut utmp_ent: *const STRUCT_UTMP) {
    let mut stats: stat = stat {
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
    let mut last_change: time_t = 0;
    let mut mesg: libc::c_char = 0;
    let mut line: [libc::c_char; 38] = [0; 38];
    let mut p: *mut libc::c_char = line.as_mut_ptr();
    if !((*utmp_ent).ut_line[0 as libc::c_int as usize] as libc::c_int == '/' as i32) {
        p = stpcpy(p, b"/dev/\0" as *const u8 as *const libc::c_char);
    }
    stzncpy(
        p,
        ((*utmp_ent).ut_line).as_ptr(),
        ::core::mem::size_of::<[libc::c_char; 32]>() as libc::c_ulong,
    );
    if stat(line.as_mut_ptr(), &mut stats) == 0 as libc::c_int {
        mesg = (if stats.st_mode
            & (0o200 as libc::c_int >> 3 as libc::c_int) as libc::c_uint != 0
        {
            ' ' as i32
        } else {
            '*' as i32
        }) as libc::c_char;
        last_change = stats.st_atim.tv_sec;
    } else {
        mesg = '?' as i32 as libc::c_char;
        last_change = 0 as libc::c_int as time_t;
    }
    printf(
        b"%-8.*s\0" as *const u8 as *const libc::c_char,
        UT_USER_SIZE as libc::c_int,
        ((*utmp_ent).ut_user).as_ptr(),
    );
    if include_fullname {
        let mut pw: *mut passwd = 0 as *mut passwd;
        let mut name: [libc::c_char; 33] = [0; 33];
        stzncpy(
            name.as_mut_ptr(),
            ((*utmp_ent).ut_user).as_ptr(),
            UT_USER_SIZE as libc::c_int as size_t,
        );
        pw = getpwnam(name.as_mut_ptr());
        if pw.is_null() {
            printf(
                b" %19s\0" as *const u8 as *const libc::c_char,
                dcgettext(
                    0 as *const libc::c_char,
                    b"        ???\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
            );
        } else {
            let comma: *mut libc::c_char = strchr((*pw).pw_gecos, ',' as i32);
            let mut result: *mut libc::c_char = 0 as *mut libc::c_char;
            if !comma.is_null() {
                *comma = '\0' as i32 as libc::c_char;
            }
            result = create_fullname((*pw).pw_gecos, (*pw).pw_name);
            printf(b" %-19.19s\0" as *const u8 as *const libc::c_char, result);
            free(result as *mut libc::c_void);
        }
    }
    printf(
        b" %c%-8.*s\0" as *const u8 as *const libc::c_char,
        mesg as libc::c_int,
        ::core::mem::size_of::<[libc::c_char; 32]>() as libc::c_ulong as libc::c_int,
        ((*utmp_ent).ut_line).as_ptr(),
    );
    if include_idle {
        if last_change != 0 {
            printf(
                b" %-6s\0" as *const u8 as *const libc::c_char,
                idle_string(last_change),
            );
        } else {
            printf(
                b" %-6s\0" as *const u8 as *const libc::c_char,
                dcgettext(
                    0 as *const libc::c_char,
                    b"?????\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
            );
        }
    }
    printf(b" %s\0" as *const u8 as *const libc::c_char, time_string(utmp_ent));
    if include_where as libc::c_int != 0
        && (*utmp_ent).ut_host[0 as libc::c_int as usize] as libc::c_int != 0
    {
        let mut ut_host: [libc::c_char; 257] = [0; 257];
        let mut host: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut display: *mut libc::c_char = 0 as *mut libc::c_char;
        stzncpy(
            ut_host.as_mut_ptr(),
            ((*utmp_ent).ut_host).as_ptr(),
            ::core::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong,
        );
        display = strchr(ut_host.as_mut_ptr(), ':' as i32);
        if !display.is_null() {
            let fresh19 = display;
            display = display.offset(1);
            *fresh19 = '\0' as i32 as libc::c_char;
        }
        if *ut_host.as_mut_ptr() != 0 {
            host = canon_host(ut_host.as_mut_ptr());
        }
        if host.is_null() {
            host = ut_host.as_mut_ptr();
        }
        if !display.is_null() {
            printf(b" %s:%s\0" as *const u8 as *const libc::c_char, host, display);
        } else {
            printf(b" %s\0" as *const u8 as *const libc::c_char, host);
        }
        if host != ut_host.as_mut_ptr() {
            free(host as *mut libc::c_void);
        }
    }
    putchar_unlocked('\n' as i32);
}
unsafe extern "C" fn print_long_entry(mut name: *const libc::c_char) {
    let mut pw: *mut passwd = 0 as *mut passwd;
    pw = getpwnam(name);
    printf(
        dcgettext(
            0 as *const libc::c_char,
            b"Login name: \0" as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        ),
    );
    printf(b"%-28s\0" as *const u8 as *const libc::c_char, name);
    printf(
        dcgettext(
            0 as *const libc::c_char,
            b"In real life: \0" as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        ),
    );
    if pw.is_null() {
        printf(
            b" %s\0" as *const u8 as *const libc::c_char,
            dcgettext(
                0 as *const libc::c_char,
                b"???\n\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
        return;
    } else {
        let comma: *mut libc::c_char = strchr((*pw).pw_gecos, ',' as i32);
        let mut result: *mut libc::c_char = 0 as *mut libc::c_char;
        if !comma.is_null() {
            *comma = '\0' as i32 as libc::c_char;
        }
        result = create_fullname((*pw).pw_gecos, (*pw).pw_name);
        printf(b" %s\0" as *const u8 as *const libc::c_char, result);
        free(result as *mut libc::c_void);
    }
    putchar_unlocked('\n' as i32);
    if include_home_and_shell {
        printf(
            dcgettext(
                0 as *const libc::c_char,
                b"Directory: \0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
        printf(b"%-29s\0" as *const u8 as *const libc::c_char, (*pw).pw_dir);
        printf(
            dcgettext(
                0 as *const libc::c_char,
                b"Shell: \0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
        printf(b" %s\0" as *const u8 as *const libc::c_char, (*pw).pw_shell);
        putchar_unlocked('\n' as i32);
    }
    if include_project {
        let mut stream: *mut FILE = 0 as *mut FILE;
        let mut buf: [libc::c_char; 1024] = [0; 1024];
        let baseproject: *const libc::c_char = b"/.project\0" as *const u8
            as *const libc::c_char;
        let project: *mut libc::c_char = xmalloc(
            (strlen((*pw).pw_dir))
                .wrapping_add(strlen(baseproject))
                .wrapping_add(1 as libc::c_int as libc::c_ulong),
        ) as *mut libc::c_char;
        stpcpy(stpcpy(project, (*pw).pw_dir), baseproject);
        stream = fopen(project, b"r\0" as *const u8 as *const libc::c_char);
        if !stream.is_null() {
            let mut bytes: size_t = 0;
            printf(
                dcgettext(
                    0 as *const libc::c_char,
                    b"Project: \0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
            );
            loop {
                bytes = (if 0 != 0 && 0 != 0
                    && (1 as libc::c_int as size_t)
                        .wrapping_mul(
                            ::core::mem::size_of::<[libc::c_char; 1024]>()
                                as libc::c_ulong,
                        ) <= 8 as libc::c_int as libc::c_ulong
                    && 1 as libc::c_int as size_t != 0 as libc::c_int as libc::c_ulong
                {
                    ({
                        let mut __ptr: *mut libc::c_char = buf.as_mut_ptr();
                        let mut __stream: *mut FILE = stream;
                        let mut __cnt: size_t = 0;
                        __cnt = (1 as libc::c_int as size_t)
                            .wrapping_mul(
                                ::core::mem::size_of::<[libc::c_char; 1024]>()
                                    as libc::c_ulong,
                            );
                        while __cnt > 0 as libc::c_int as libc::c_ulong {
                            let mut __c: libc::c_int = getc_unlocked(__stream);
                            if __c == -(1 as libc::c_int) {
                                break;
                            }
                            let fresh20 = __ptr;
                            __ptr = __ptr.offset(1);
                            *fresh20 = __c as libc::c_char;
                            __cnt = __cnt.wrapping_sub(1);
                        }
                        (1 as libc::c_int as size_t)
                            .wrapping_mul(
                                ::core::mem::size_of::<[libc::c_char; 1024]>()
                                    as libc::c_ulong,
                            )
                            .wrapping_sub(__cnt)
                            .wrapping_div(1 as libc::c_int as size_t)
                    })
                } else {
                    (if 0 != 0
                        && 1 as libc::c_int as size_t
                            == 0 as libc::c_int as libc::c_ulong
                        || 0 != 0
                            && ::core::mem::size_of::<[libc::c_char; 1024]>()
                                as libc::c_ulong == 0 as libc::c_int as libc::c_ulong
                    {
                        0 as libc::c_int as size_t
                    } else {
                        fread_unlocked(
                            buf.as_mut_ptr() as *mut libc::c_void,
                            1 as libc::c_int as size_t,
                            ::core::mem::size_of::<[libc::c_char; 1024]>()
                                as libc::c_ulong,
                            stream,
                        )
                    })
                });
                if !(bytes > 0 as libc::c_int as libc::c_ulong) {
                    break;
                }
                if 0 != 0 && 0 != 0
                    && (1 as libc::c_int as size_t).wrapping_mul(bytes)
                        <= 8 as libc::c_int as libc::c_ulong
                    && 1 as libc::c_int as size_t != 0 as libc::c_int as libc::c_ulong
                {
                    ({
                        let mut __ptr: *const libc::c_char = buf.as_mut_ptr()
                            as *const libc::c_char;
                        let mut __stream: *mut FILE = stdout;
                        let mut __cnt: size_t = 0;
                        __cnt = (1 as libc::c_int as size_t).wrapping_mul(bytes);
                        while __cnt > 0 as libc::c_int as libc::c_ulong {
                            let fresh21 = __ptr;
                            __ptr = __ptr.offset(1);
                            if putc_unlocked(*fresh21 as libc::c_int, __stream)
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
                        || 0 != 0 && bytes == 0 as libc::c_int as libc::c_ulong
                    {} else {
                        fwrite_unlocked(
                            buf.as_mut_ptr() as *const libc::c_void,
                            1 as libc::c_int as size_t,
                            bytes,
                            stdout,
                        );
                    };
                };
            }
            rpl_fclose(stream);
        }
        free(project as *mut libc::c_void);
    }
    if include_plan {
        let mut stream_0: *mut FILE = 0 as *mut FILE;
        let mut buf_0: [libc::c_char; 1024] = [0; 1024];
        let baseplan: *const libc::c_char = b"/.plan\0" as *const u8
            as *const libc::c_char;
        let plan: *mut libc::c_char = xmalloc(
            (strlen((*pw).pw_dir))
                .wrapping_add(strlen(baseplan))
                .wrapping_add(1 as libc::c_int as libc::c_ulong),
        ) as *mut libc::c_char;
        stpcpy(stpcpy(plan, (*pw).pw_dir), baseplan);
        stream_0 = fopen(plan, b"r\0" as *const u8 as *const libc::c_char);
        if !stream_0.is_null() {
            let mut bytes_0: size_t = 0;
            printf(
                dcgettext(
                    0 as *const libc::c_char,
                    b"Plan:\n\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
            );
            loop {
                bytes_0 = (if 0 != 0 && 0 != 0
                    && (1 as libc::c_int as size_t)
                        .wrapping_mul(
                            ::core::mem::size_of::<[libc::c_char; 1024]>()
                                as libc::c_ulong,
                        ) <= 8 as libc::c_int as libc::c_ulong
                    && 1 as libc::c_int as size_t != 0 as libc::c_int as libc::c_ulong
                {
                    ({
                        let mut __ptr: *mut libc::c_char = buf_0.as_mut_ptr();
                        let mut __stream: *mut FILE = stream_0;
                        let mut __cnt: size_t = 0;
                        __cnt = (1 as libc::c_int as size_t)
                            .wrapping_mul(
                                ::core::mem::size_of::<[libc::c_char; 1024]>()
                                    as libc::c_ulong,
                            );
                        while __cnt > 0 as libc::c_int as libc::c_ulong {
                            let mut __c: libc::c_int = getc_unlocked(__stream);
                            if __c == -(1 as libc::c_int) {
                                break;
                            }
                            let fresh22 = __ptr;
                            __ptr = __ptr.offset(1);
                            *fresh22 = __c as libc::c_char;
                            __cnt = __cnt.wrapping_sub(1);
                        }
                        (1 as libc::c_int as size_t)
                            .wrapping_mul(
                                ::core::mem::size_of::<[libc::c_char; 1024]>()
                                    as libc::c_ulong,
                            )
                            .wrapping_sub(__cnt)
                            .wrapping_div(1 as libc::c_int as size_t)
                    })
                } else {
                    (if 0 != 0
                        && 1 as libc::c_int as size_t
                            == 0 as libc::c_int as libc::c_ulong
                        || 0 != 0
                            && ::core::mem::size_of::<[libc::c_char; 1024]>()
                                as libc::c_ulong == 0 as libc::c_int as libc::c_ulong
                    {
                        0 as libc::c_int as size_t
                    } else {
                        fread_unlocked(
                            buf_0.as_mut_ptr() as *mut libc::c_void,
                            1 as libc::c_int as size_t,
                            ::core::mem::size_of::<[libc::c_char; 1024]>()
                                as libc::c_ulong,
                            stream_0,
                        )
                    })
                });
                if !(bytes_0 > 0 as libc::c_int as libc::c_ulong) {
                    break;
                }
                if 0 != 0 && 0 != 0
                    && (1 as libc::c_int as size_t).wrapping_mul(bytes_0)
                        <= 8 as libc::c_int as libc::c_ulong
                    && 1 as libc::c_int as size_t != 0 as libc::c_int as libc::c_ulong
                {
                    ({
                        let mut __ptr: *const libc::c_char = buf_0.as_mut_ptr()
                            as *const libc::c_char;
                        let mut __stream: *mut FILE = stdout;
                        let mut __cnt: size_t = 0;
                        __cnt = (1 as libc::c_int as size_t).wrapping_mul(bytes_0);
                        while __cnt > 0 as libc::c_int as libc::c_ulong {
                            let fresh23 = __ptr;
                            __ptr = __ptr.offset(1);
                            if putc_unlocked(*fresh23 as libc::c_int, __stream)
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
                        || 0 != 0 && bytes_0 == 0 as libc::c_int as libc::c_ulong
                    {} else {
                        fwrite_unlocked(
                            buf_0.as_mut_ptr() as *const libc::c_void,
                            1 as libc::c_int as size_t,
                            bytes_0,
                            stdout,
                        );
                    };
                };
            }
            rpl_fclose(stream_0);
        }
        free(plan as *mut libc::c_void);
    }
    putchar_unlocked('\n' as i32);
}
unsafe extern "C" fn print_heading() {
    printf(
        b"%-8s\0" as *const u8 as *const libc::c_char,
        dcgettext(
            0 as *const libc::c_char,
            b"Login\0" as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        ),
    );
    if include_fullname {
        printf(
            b" %-19s\0" as *const u8 as *const libc::c_char,
            dcgettext(
                0 as *const libc::c_char,
                b"Name\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
    }
    printf(
        b" %-9s\0" as *const u8 as *const libc::c_char,
        dcgettext(
            0 as *const libc::c_char,
            b" TTY\0" as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        ),
    );
    if include_idle {
        printf(
            b" %-6s\0" as *const u8 as *const libc::c_char,
            dcgettext(
                0 as *const libc::c_char,
                b"Idle\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
    }
    printf(
        b" %-*s\0" as *const u8 as *const libc::c_char,
        time_format_width,
        dcgettext(
            0 as *const libc::c_char,
            b"When\0" as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        ),
    );
    if include_where {
        printf(
            b" %s\0" as *const u8 as *const libc::c_char,
            dcgettext(
                0 as *const libc::c_char,
                b"Where\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
    }
    putchar_unlocked('\n' as i32);
}
unsafe extern "C" fn scan_entries(
    mut n: size_t,
    mut utmp_buf: *const STRUCT_UTMP,
    argc_names: libc::c_int,
    mut argv_names: *const *mut libc::c_char,
) {
    if hard_locale(2 as libc::c_int) {
        time_format = b"%Y-%m-%d %H:%M\0" as *const u8 as *const libc::c_char;
        time_format_width = 4 as libc::c_int + 1 as libc::c_int + 2 as libc::c_int
            + 1 as libc::c_int + 2 as libc::c_int + 1 as libc::c_int + 2 as libc::c_int
            + 1 as libc::c_int + 2 as libc::c_int;
    } else {
        time_format = b"%b %e %H:%M\0" as *const u8 as *const libc::c_char;
        time_format_width = 3 as libc::c_int + 1 as libc::c_int + 2 as libc::c_int
            + 1 as libc::c_int + 2 as libc::c_int + 1 as libc::c_int + 2 as libc::c_int;
    }
    if include_heading {
        print_heading();
    }
    loop {
        let fresh24 = n;
        n = n.wrapping_sub(1);
        if !(fresh24 != 0) {
            break;
        }
        if (*utmp_buf).ut_user[0 as libc::c_int as usize] as libc::c_int != 0
            && ((*utmp_buf).ut_type as libc::c_int == 7 as libc::c_int
                || 0 as libc::c_int != 0
                    && (*utmp_buf).ut_tv.tv_sec != 0 as libc::c_int as libc::c_long)
        {
            if argc_names != 0 {
                let mut i: libc::c_int = 0 as libc::c_int;
                while i < argc_names {
                    if strncmp(
                        ((*utmp_buf).ut_user).as_ptr(),
                        *argv_names.offset(i as isize),
                        UT_USER_SIZE as libc::c_int as libc::c_ulong,
                    ) == 0 as libc::c_int
                    {
                        print_entry(utmp_buf);
                        break;
                    } else {
                        i += 1;
                    }
                }
            } else {
                print_entry(utmp_buf);
            }
        }
        utmp_buf = utmp_buf.offset(1);
    };
}
unsafe extern "C" fn short_pinky(
    mut filename: *const libc::c_char,
    argc_names: libc::c_int,
    mut argv_names: *const *mut libc::c_char,
) {
    let mut n_users: size_t = 0;
    let mut utmp_buf: *mut STRUCT_UTMP = 0 as *mut STRUCT_UTMP;
    if read_utmp(filename, &mut n_users, &mut utmp_buf, 0 as libc::c_int)
        != 0 as libc::c_int
    {
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
    scan_entries(n_users, utmp_buf, argc_names, argv_names);
    exit(0 as libc::c_int);
}
unsafe extern "C" fn long_pinky(
    argc_names: libc::c_int,
    mut argv_names: *const *mut libc::c_char,
) {
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < argc_names {
        print_long_entry(*argv_names.offset(i as isize) as *const libc::c_char);
        i += 1;
    }
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
                b"Usage: %s [OPTION]... [USER]...\n\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
            program_name,
        );
        fputs_unlocked(
            dcgettext(
                0 as *const libc::c_char,
                b"\n  -l              produce long format output for the specified USERs\n  -b              omit the user's home directory and shell in long format\n  -h              omit the user's project file in long format\n  -p              omit the user's plan file in long format\n  -s              do short format output, this is the default\n\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            stdout,
        );
        fputs_unlocked(
            dcgettext(
                0 as *const libc::c_char,
                b"  -f              omit the line of column headings in short format\n  -w              omit the user's full name in short format\n  -i              omit the user's full name and remote host in short format\n  -q              omit the user's full name, remote host and idle time\n                  in short format\n\0"
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
        printf(
            dcgettext(
                0 as *const libc::c_char,
                b"\nA lightweight 'finger' program;  print user information.\nThe utmp file will be %s.\n\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            b"/var/run/utmp\0" as *const u8 as *const libc::c_char,
        );
        emit_ancillary_info(b"pinky\0" as *const u8 as *const libc::c_char);
    }
    exit(status);
}
unsafe fn main_0(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut optc: libc::c_int = 0;
    let mut n_users: libc::c_int = 0;
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
            b"sfwiqbhlp\0" as *const u8 as *const libc::c_char,
            longopts.as_ptr(),
            0 as *mut libc::c_int,
        );
        if !(optc != -(1 as libc::c_int)) {
            break;
        }
        match optc {
            115 => {
                do_short_format = 1 as libc::c_int != 0;
            }
            108 => {
                do_short_format = 0 as libc::c_int != 0;
            }
            102 => {
                include_heading = 0 as libc::c_int != 0;
            }
            119 => {
                include_fullname = 0 as libc::c_int != 0;
            }
            105 => {
                include_fullname = 0 as libc::c_int != 0;
                include_where = 0 as libc::c_int != 0;
            }
            113 => {
                include_fullname = 0 as libc::c_int != 0;
                include_where = 0 as libc::c_int != 0;
                include_idle = 0 as libc::c_int != 0;
            }
            104 => {
                include_project = 0 as libc::c_int != 0;
            }
            112 => {
                include_plan = 0 as libc::c_int != 0;
            }
            98 => {
                include_home_and_shell = 0 as libc::c_int != 0;
            }
            -2 => {
                usage(0 as libc::c_int);
            }
            -3 => {
                version_etc(
                    stdout,
                    b"pinky\0" as *const u8 as *const libc::c_char,
                    b"GNU coreutils\0" as *const u8 as *const libc::c_char,
                    Version,
                    b"Joseph Arceneaux\0" as *const u8 as *const libc::c_char,
                    b"David MacKenzie\0" as *const u8 as *const libc::c_char,
                    b"Kaveh Ghazi\0" as *const u8 as *const libc::c_char,
                    0 as *mut libc::c_void as *mut libc::c_char,
                );
                exit(0 as libc::c_int);
            }
            _ => {
                usage(1 as libc::c_int);
            }
        }
    }
    n_users = argc - optind;
    if !do_short_format && n_users == 0 as libc::c_int {
        error(
            0 as libc::c_int,
            0 as libc::c_int,
            dcgettext(
                0 as *const libc::c_char,
                b"no username specified; at least one must be specified when using -l\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
        usage(1 as libc::c_int);
    }
    if do_short_format {
        short_pinky(
            b"/var/run/utmp\0" as *const u8 as *const libc::c_char,
            n_users,
            argv.offset(optind as isize) as *const *mut libc::c_char,
        );
    } else {
        long_pinky(n_users, argv.offset(optind as isize) as *const *mut libc::c_char);
    }
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
