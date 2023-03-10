#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![feature(c_variadic, extern_types, label_break_value)]
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    static mut stdout: *mut FILE;
    static mut stderr: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn rpl_vasprintf(
        result: *mut *mut libc::c_char,
        format: *const libc::c_char,
        args: ::core::ffi::VaList,
    ) -> libc::c_int;
    fn fputs_unlocked(__s: *const libc::c_char, __stream: *mut FILE) -> libc::c_int;
    fn __overflow(_: *mut FILE, _: libc::c_int) -> libc::c_int;
    fn cfgetospeed(__termios_p: *const termios) -> speed_t;
    fn cfgetispeed(__termios_p: *const termios) -> speed_t;
    fn cfsetospeed(__termios_p: *mut termios, __speed: speed_t) -> libc::c_int;
    fn cfsetispeed(__termios_p: *mut termios, __speed: speed_t) -> libc::c_int;
    fn tcgetattr(__fd: libc::c_int, __termios_p: *mut termios) -> libc::c_int;
    fn tcsetattr(
        __fd: libc::c_int,
        __optional_actions: libc::c_int,
        __termios_p: *const termios,
    ) -> libc::c_int;
    fn ioctl(__fd: libc::c_int, __request: libc::c_ulong, _: ...) -> libc::c_int;
    fn getopt_long(
        ___argc: libc::c_int,
        ___argv: *const *mut libc::c_char,
        __shortopts: *const libc::c_char,
        __longopts: *const option,
        __longind: *mut libc::c_int,
    ) -> libc::c_int;
    static mut optarg: *mut libc::c_char;
    static mut optind: libc::c_int;
    static mut opterr: libc::c_int;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
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
    fn strtoul(
        _: *const libc::c_char,
        _: *mut *mut libc::c_char,
        _: libc::c_int,
    ) -> libc::c_ulong;
    fn abort() -> !;
    fn atexit(__func: Option::<unsafe extern "C" fn() -> ()>) -> libc::c_int;
    fn exit(_: libc::c_int) -> !;
    fn getenv(__name: *const libc::c_char) -> *mut libc::c_char;
    static mut Version: *const libc::c_char;
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
    fn xalloc_die();
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
    fn error(
        __status: libc::c_int,
        __errnum: libc::c_int,
        __format: *const libc::c_char,
        _: ...
    );
    fn fd_reopen(
        _: libc::c_int,
        _: *const libc::c_char,
        _: libc::c_int,
        _: mode_t,
    ) -> libc::c_int;
    fn quote(arg: *const libc::c_char) -> *const libc::c_char;
    fn xnumtoumax(
        n_str: *const libc::c_char,
        base: libc::c_int,
        min: uintmax_t,
        max: uintmax_t,
        suffixes: *const libc::c_char,
        err: *const libc::c_char,
        err_exit: libc::c_int,
    ) -> uintmax_t;
    fn xstrtol(
        _: *const libc::c_char,
        _: *mut *mut libc::c_char,
        _: libc::c_int,
        _: *mut libc::c_long,
        _: *const libc::c_char,
    ) -> strtol_error;
}
pub type __builtin_va_list = __va_list;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __va_list {
    pub __stack: *mut libc::c_void,
    pub __gr_top: *mut libc::c_void,
    pub __vr_top: *mut libc::c_void,
    pub __gr_offs: libc::c_int,
    pub __vr_offs: libc::c_int,
}
pub type size_t = libc::c_ulong;
pub type va_list = __builtin_va_list;
pub type __uintmax_t = libc::c_ulong;
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
pub type cc_t = libc::c_uchar;
pub type speed_t = libc::c_uint;
pub type tcflag_t = libc::c_uint;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct termios {
    pub c_iflag: tcflag_t,
    pub c_oflag: tcflag_t,
    pub c_cflag: tcflag_t,
    pub c_lflag: tcflag_t,
    pub c_line: cc_t,
    pub c_cc: [cc_t; 32],
    pub c_ispeed: speed_t,
    pub c_ospeed: speed_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct winsize {
    pub ws_row: libc::c_ushort,
    pub ws_col: libc::c_ushort,
    pub ws_xpixel: libc::c_ushort,
    pub ws_ypixel: libc::c_ushort,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct option {
    pub name: *const libc::c_char,
    pub has_arg: libc::c_int,
    pub flag: *mut libc::c_int,
    pub val: libc::c_int,
}
pub type uintmax_t = __uintmax_t;
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
pub type strtol_error = libc::c_uint;
pub const LONGINT_INVALID: strtol_error = 4;
pub const LONGINT_INVALID_SUFFIX_CHAR_WITH_OVERFLOW: strtol_error = 3;
pub const LONGINT_INVALID_SUFFIX_CHAR: strtol_error = 2;
pub const LONGINT_OVERFLOW: strtol_error = 1;
pub const LONGINT_OK: strtol_error = 0;
pub type speed_setting = libc::c_uint;
pub const both_speeds: speed_setting = 2;
pub const output_speed: speed_setting = 1;
pub const input_speed: speed_setting = 0;
pub type output_type = libc::c_uint;
pub const recoverable: output_type = 2;
pub const all: output_type = 1;
pub const changed: output_type = 0;
pub type mode_type = libc::c_uint;
pub const combination: mode_type = 4;
pub const local: mode_type = 3;
pub const output: mode_type = 2;
pub const input: mode_type = 1;
pub const control: mode_type = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mode_info {
    pub name: *const libc::c_char,
    pub type_0: mode_type,
    pub flags: libc::c_char,
    pub bits: libc::c_ulong,
    pub mask: libc::c_ulong,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct control_info {
    pub name: *const libc::c_char,
    pub saneval: cc_t,
    pub offset: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct speed_map {
    pub string: *const libc::c_char,
    pub speed: speed_t,
    pub value: libc::c_ulong,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_0 {
    pub _gl_dummy: libc::c_int,
}
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_6 {
    pub _gl_dummy: libc::c_int,
}
pub type C2RustUnnamed_7 = libc::c_uint;
pub const DEV_DEBUG_OPTION: C2RustUnnamed_7 = 256;
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
unsafe extern "C" fn to_uchar(mut ch: libc::c_char) -> libc::c_uchar {
    return ch as libc::c_uchar;
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
static mut mode_info: [mode_info; 90] = [
    {
        let mut init = mode_info {
            name: b"parenb\0" as *const u8 as *const libc::c_char,
            type_0: control,
            flags: 4 as libc::c_int as libc::c_char,
            bits: 0o400 as libc::c_int as libc::c_ulong,
            mask: 0 as libc::c_int as libc::c_ulong,
        };
        init
    },
    {
        let mut init = mode_info {
            name: b"parodd\0" as *const u8 as *const libc::c_char,
            type_0: control,
            flags: 4 as libc::c_int as libc::c_char,
            bits: 0o1000 as libc::c_int as libc::c_ulong,
            mask: 0 as libc::c_int as libc::c_ulong,
        };
        init
    },
    {
        let mut init = mode_info {
            name: b"cmspar\0" as *const u8 as *const libc::c_char,
            type_0: control,
            flags: 4 as libc::c_int as libc::c_char,
            bits: 0o10000000000 as libc::c_int as libc::c_ulong,
            mask: 0 as libc::c_int as libc::c_ulong,
        };
        init
    },
    {
        let mut init = mode_info {
            name: b"cs5\0" as *const u8 as *const libc::c_char,
            type_0: control,
            flags: 0 as libc::c_int as libc::c_char,
            bits: 0 as libc::c_int as libc::c_ulong,
            mask: 0o60 as libc::c_int as libc::c_ulong,
        };
        init
    },
    {
        let mut init = mode_info {
            name: b"cs6\0" as *const u8 as *const libc::c_char,
            type_0: control,
            flags: 0 as libc::c_int as libc::c_char,
            bits: 0o20 as libc::c_int as libc::c_ulong,
            mask: 0o60 as libc::c_int as libc::c_ulong,
        };
        init
    },
    {
        let mut init = mode_info {
            name: b"cs7\0" as *const u8 as *const libc::c_char,
            type_0: control,
            flags: 0 as libc::c_int as libc::c_char,
            bits: 0o40 as libc::c_int as libc::c_ulong,
            mask: 0o60 as libc::c_int as libc::c_ulong,
        };
        init
    },
    {
        let mut init = mode_info {
            name: b"cs8\0" as *const u8 as *const libc::c_char,
            type_0: control,
            flags: 0 as libc::c_int as libc::c_char,
            bits: 0o60 as libc::c_int as libc::c_ulong,
            mask: 0o60 as libc::c_int as libc::c_ulong,
        };
        init
    },
    {
        let mut init = mode_info {
            name: b"hupcl\0" as *const u8 as *const libc::c_char,
            type_0: control,
            flags: 4 as libc::c_int as libc::c_char,
            bits: 0o2000 as libc::c_int as libc::c_ulong,
            mask: 0 as libc::c_int as libc::c_ulong,
        };
        init
    },
    {
        let mut init = mode_info {
            name: b"hup\0" as *const u8 as *const libc::c_char,
            type_0: control,
            flags: (4 as libc::c_int | 8 as libc::c_int) as libc::c_char,
            bits: 0o2000 as libc::c_int as libc::c_ulong,
            mask: 0 as libc::c_int as libc::c_ulong,
        };
        init
    },
    {
        let mut init = mode_info {
            name: b"cstopb\0" as *const u8 as *const libc::c_char,
            type_0: control,
            flags: 4 as libc::c_int as libc::c_char,
            bits: 0o100 as libc::c_int as libc::c_ulong,
            mask: 0 as libc::c_int as libc::c_ulong,
        };
        init
    },
    {
        let mut init = mode_info {
            name: b"cread\0" as *const u8 as *const libc::c_char,
            type_0: control,
            flags: (1 as libc::c_int | 4 as libc::c_int) as libc::c_char,
            bits: 0o200 as libc::c_int as libc::c_ulong,
            mask: 0 as libc::c_int as libc::c_ulong,
        };
        init
    },
    {
        let mut init = mode_info {
            name: b"clocal\0" as *const u8 as *const libc::c_char,
            type_0: control,
            flags: 4 as libc::c_int as libc::c_char,
            bits: 0o4000 as libc::c_int as libc::c_ulong,
            mask: 0 as libc::c_int as libc::c_ulong,
        };
        init
    },
    {
        let mut init = mode_info {
            name: b"crtscts\0" as *const u8 as *const libc::c_char,
            type_0: control,
            flags: 4 as libc::c_int as libc::c_char,
            bits: 0o20000000000 as libc::c_uint as libc::c_ulong,
            mask: 0 as libc::c_int as libc::c_ulong,
        };
        init
    },
    {
        let mut init = mode_info {
            name: b"ignbrk\0" as *const u8 as *const libc::c_char,
            type_0: input,
            flags: (2 as libc::c_int | 4 as libc::c_int) as libc::c_char,
            bits: 0o1 as libc::c_int as libc::c_ulong,
            mask: 0 as libc::c_int as libc::c_ulong,
        };
        init
    },
    {
        let mut init = mode_info {
            name: b"brkint\0" as *const u8 as *const libc::c_char,
            type_0: input,
            flags: (1 as libc::c_int | 4 as libc::c_int) as libc::c_char,
            bits: 0o2 as libc::c_int as libc::c_ulong,
            mask: 0 as libc::c_int as libc::c_ulong,
        };
        init
    },
    {
        let mut init = mode_info {
            name: b"ignpar\0" as *const u8 as *const libc::c_char,
            type_0: input,
            flags: 4 as libc::c_int as libc::c_char,
            bits: 0o4 as libc::c_int as libc::c_ulong,
            mask: 0 as libc::c_int as libc::c_ulong,
        };
        init
    },
    {
        let mut init = mode_info {
            name: b"parmrk\0" as *const u8 as *const libc::c_char,
            type_0: input,
            flags: 4 as libc::c_int as libc::c_char,
            bits: 0o10 as libc::c_int as libc::c_ulong,
            mask: 0 as libc::c_int as libc::c_ulong,
        };
        init
    },
    {
        let mut init = mode_info {
            name: b"inpck\0" as *const u8 as *const libc::c_char,
            type_0: input,
            flags: 4 as libc::c_int as libc::c_char,
            bits: 0o20 as libc::c_int as libc::c_ulong,
            mask: 0 as libc::c_int as libc::c_ulong,
        };
        init
    },
    {
        let mut init = mode_info {
            name: b"istrip\0" as *const u8 as *const libc::c_char,
            type_0: input,
            flags: 4 as libc::c_int as libc::c_char,
            bits: 0o40 as libc::c_int as libc::c_ulong,
            mask: 0 as libc::c_int as libc::c_ulong,
        };
        init
    },
    {
        let mut init = mode_info {
            name: b"inlcr\0" as *const u8 as *const libc::c_char,
            type_0: input,
            flags: (2 as libc::c_int | 4 as libc::c_int) as libc::c_char,
            bits: 0o100 as libc::c_int as libc::c_ulong,
            mask: 0 as libc::c_int as libc::c_ulong,
        };
        init
    },
    {
        let mut init = mode_info {
            name: b"igncr\0" as *const u8 as *const libc::c_char,
            type_0: input,
            flags: (2 as libc::c_int | 4 as libc::c_int) as libc::c_char,
            bits: 0o200 as libc::c_int as libc::c_ulong,
            mask: 0 as libc::c_int as libc::c_ulong,
        };
        init
    },
    {
        let mut init = mode_info {
            name: b"icrnl\0" as *const u8 as *const libc::c_char,
            type_0: input,
            flags: (1 as libc::c_int | 4 as libc::c_int) as libc::c_char,
            bits: 0o400 as libc::c_int as libc::c_ulong,
            mask: 0 as libc::c_int as libc::c_ulong,
        };
        init
    },
    {
        let mut init = mode_info {
            name: b"ixon\0" as *const u8 as *const libc::c_char,
            type_0: input,
            flags: 4 as libc::c_int as libc::c_char,
            bits: 0o2000 as libc::c_int as libc::c_ulong,
            mask: 0 as libc::c_int as libc::c_ulong,
        };
        init
    },
    {
        let mut init = mode_info {
            name: b"ixoff\0" as *const u8 as *const libc::c_char,
            type_0: input,
            flags: (2 as libc::c_int | 4 as libc::c_int) as libc::c_char,
            bits: 0o10000 as libc::c_int as libc::c_ulong,
            mask: 0 as libc::c_int as libc::c_ulong,
        };
        init
    },
    {
        let mut init = mode_info {
            name: b"tandem\0" as *const u8 as *const libc::c_char,
            type_0: input,
            flags: (4 as libc::c_int | 8 as libc::c_int) as libc::c_char,
            bits: 0o10000 as libc::c_int as libc::c_ulong,
            mask: 0 as libc::c_int as libc::c_ulong,
        };
        init
    },
    {
        let mut init = mode_info {
            name: b"iuclc\0" as *const u8 as *const libc::c_char,
            type_0: input,
            flags: (2 as libc::c_int | 4 as libc::c_int) as libc::c_char,
            bits: 0o1000 as libc::c_int as libc::c_ulong,
            mask: 0 as libc::c_int as libc::c_ulong,
        };
        init
    },
    {
        let mut init = mode_info {
            name: b"ixany\0" as *const u8 as *const libc::c_char,
            type_0: input,
            flags: (2 as libc::c_int | 4 as libc::c_int) as libc::c_char,
            bits: 0o4000 as libc::c_int as libc::c_ulong,
            mask: 0 as libc::c_int as libc::c_ulong,
        };
        init
    },
    {
        let mut init = mode_info {
            name: b"imaxbel\0" as *const u8 as *const libc::c_char,
            type_0: input,
            flags: (1 as libc::c_int | 4 as libc::c_int) as libc::c_char,
            bits: 0o20000 as libc::c_int as libc::c_ulong,
            mask: 0 as libc::c_int as libc::c_ulong,
        };
        init
    },
    {
        let mut init = mode_info {
            name: b"iutf8\0" as *const u8 as *const libc::c_char,
            type_0: input,
            flags: (2 as libc::c_int | 4 as libc::c_int) as libc::c_char,
            bits: 0o40000 as libc::c_int as libc::c_ulong,
            mask: 0 as libc::c_int as libc::c_ulong,
        };
        init
    },
    {
        let mut init = mode_info {
            name: b"opost\0" as *const u8 as *const libc::c_char,
            type_0: output,
            flags: (1 as libc::c_int | 4 as libc::c_int) as libc::c_char,
            bits: 0o1 as libc::c_int as libc::c_ulong,
            mask: 0 as libc::c_int as libc::c_ulong,
        };
        init
    },
    {
        let mut init = mode_info {
            name: b"olcuc\0" as *const u8 as *const libc::c_char,
            type_0: output,
            flags: (2 as libc::c_int | 4 as libc::c_int) as libc::c_char,
            bits: 0o2 as libc::c_int as libc::c_ulong,
            mask: 0 as libc::c_int as libc::c_ulong,
        };
        init
    },
    {
        let mut init = mode_info {
            name: b"ocrnl\0" as *const u8 as *const libc::c_char,
            type_0: output,
            flags: (2 as libc::c_int | 4 as libc::c_int) as libc::c_char,
            bits: 0o10 as libc::c_int as libc::c_ulong,
            mask: 0 as libc::c_int as libc::c_ulong,
        };
        init
    },
    {
        let mut init = mode_info {
            name: b"onlcr\0" as *const u8 as *const libc::c_char,
            type_0: output,
            flags: (1 as libc::c_int | 4 as libc::c_int) as libc::c_char,
            bits: 0o4 as libc::c_int as libc::c_ulong,
            mask: 0 as libc::c_int as libc::c_ulong,
        };
        init
    },
    {
        let mut init = mode_info {
            name: b"onocr\0" as *const u8 as *const libc::c_char,
            type_0: output,
            flags: (2 as libc::c_int | 4 as libc::c_int) as libc::c_char,
            bits: 0o20 as libc::c_int as libc::c_ulong,
            mask: 0 as libc::c_int as libc::c_ulong,
        };
        init
    },
    {
        let mut init = mode_info {
            name: b"onlret\0" as *const u8 as *const libc::c_char,
            type_0: output,
            flags: (2 as libc::c_int | 4 as libc::c_int) as libc::c_char,
            bits: 0o40 as libc::c_int as libc::c_ulong,
            mask: 0 as libc::c_int as libc::c_ulong,
        };
        init
    },
    {
        let mut init = mode_info {
            name: b"ofill\0" as *const u8 as *const libc::c_char,
            type_0: output,
            flags: (2 as libc::c_int | 4 as libc::c_int) as libc::c_char,
            bits: 0o100 as libc::c_int as libc::c_ulong,
            mask: 0 as libc::c_int as libc::c_ulong,
        };
        init
    },
    {
        let mut init = mode_info {
            name: b"ofdel\0" as *const u8 as *const libc::c_char,
            type_0: output,
            flags: (2 as libc::c_int | 4 as libc::c_int) as libc::c_char,
            bits: 0o200 as libc::c_int as libc::c_ulong,
            mask: 0 as libc::c_int as libc::c_ulong,
        };
        init
    },
    {
        let mut init = mode_info {
            name: b"nl1\0" as *const u8 as *const libc::c_char,
            type_0: output,
            flags: 2 as libc::c_int as libc::c_char,
            bits: 0o400 as libc::c_int as libc::c_ulong,
            mask: 0o400 as libc::c_int as libc::c_ulong,
        };
        init
    },
    {
        let mut init = mode_info {
            name: b"nl0\0" as *const u8 as *const libc::c_char,
            type_0: output,
            flags: 1 as libc::c_int as libc::c_char,
            bits: 0 as libc::c_int as libc::c_ulong,
            mask: 0o400 as libc::c_int as libc::c_ulong,
        };
        init
    },
    {
        let mut init = mode_info {
            name: b"cr3\0" as *const u8 as *const libc::c_char,
            type_0: output,
            flags: 2 as libc::c_int as libc::c_char,
            bits: 0o3000 as libc::c_int as libc::c_ulong,
            mask: 0o3000 as libc::c_int as libc::c_ulong,
        };
        init
    },
    {
        let mut init = mode_info {
            name: b"cr2\0" as *const u8 as *const libc::c_char,
            type_0: output,
            flags: 2 as libc::c_int as libc::c_char,
            bits: 0o2000 as libc::c_int as libc::c_ulong,
            mask: 0o3000 as libc::c_int as libc::c_ulong,
        };
        init
    },
    {
        let mut init = mode_info {
            name: b"cr1\0" as *const u8 as *const libc::c_char,
            type_0: output,
            flags: 2 as libc::c_int as libc::c_char,
            bits: 0o1000 as libc::c_int as libc::c_ulong,
            mask: 0o3000 as libc::c_int as libc::c_ulong,
        };
        init
    },
    {
        let mut init = mode_info {
            name: b"cr0\0" as *const u8 as *const libc::c_char,
            type_0: output,
            flags: 1 as libc::c_int as libc::c_char,
            bits: 0 as libc::c_int as libc::c_ulong,
            mask: 0o3000 as libc::c_int as libc::c_ulong,
        };
        init
    },
    {
        let mut init = mode_info {
            name: b"tab3\0" as *const u8 as *const libc::c_char,
            type_0: output,
            flags: 2 as libc::c_int as libc::c_char,
            bits: 0o14000 as libc::c_int as libc::c_ulong,
            mask: 0o14000 as libc::c_int as libc::c_ulong,
        };
        init
    },
    {
        let mut init = mode_info {
            name: b"tab2\0" as *const u8 as *const libc::c_char,
            type_0: output,
            flags: 2 as libc::c_int as libc::c_char,
            bits: 0o10000 as libc::c_int as libc::c_ulong,
            mask: 0o14000 as libc::c_int as libc::c_ulong,
        };
        init
    },
    {
        let mut init = mode_info {
            name: b"tab1\0" as *const u8 as *const libc::c_char,
            type_0: output,
            flags: 2 as libc::c_int as libc::c_char,
            bits: 0o4000 as libc::c_int as libc::c_ulong,
            mask: 0o14000 as libc::c_int as libc::c_ulong,
        };
        init
    },
    {
        let mut init = mode_info {
            name: b"tab0\0" as *const u8 as *const libc::c_char,
            type_0: output,
            flags: 1 as libc::c_int as libc::c_char,
            bits: 0 as libc::c_int as libc::c_ulong,
            mask: 0o14000 as libc::c_int as libc::c_ulong,
        };
        init
    },
    {
        let mut init = mode_info {
            name: b"bs1\0" as *const u8 as *const libc::c_char,
            type_0: output,
            flags: 2 as libc::c_int as libc::c_char,
            bits: 0o20000 as libc::c_int as libc::c_ulong,
            mask: 0o20000 as libc::c_int as libc::c_ulong,
        };
        init
    },
    {
        let mut init = mode_info {
            name: b"bs0\0" as *const u8 as *const libc::c_char,
            type_0: output,
            flags: 1 as libc::c_int as libc::c_char,
            bits: 0 as libc::c_int as libc::c_ulong,
            mask: 0o20000 as libc::c_int as libc::c_ulong,
        };
        init
    },
    {
        let mut init = mode_info {
            name: b"vt1\0" as *const u8 as *const libc::c_char,
            type_0: output,
            flags: 2 as libc::c_int as libc::c_char,
            bits: 0o40000 as libc::c_int as libc::c_ulong,
            mask: 0o40000 as libc::c_int as libc::c_ulong,
        };
        init
    },
    {
        let mut init = mode_info {
            name: b"vt0\0" as *const u8 as *const libc::c_char,
            type_0: output,
            flags: 1 as libc::c_int as libc::c_char,
            bits: 0 as libc::c_int as libc::c_ulong,
            mask: 0o40000 as libc::c_int as libc::c_ulong,
        };
        init
    },
    {
        let mut init = mode_info {
            name: b"ff1\0" as *const u8 as *const libc::c_char,
            type_0: output,
            flags: 2 as libc::c_int as libc::c_char,
            bits: 0o100000 as libc::c_int as libc::c_ulong,
            mask: 0o100000 as libc::c_int as libc::c_ulong,
        };
        init
    },
    {
        let mut init = mode_info {
            name: b"ff0\0" as *const u8 as *const libc::c_char,
            type_0: output,
            flags: 1 as libc::c_int as libc::c_char,
            bits: 0 as libc::c_int as libc::c_ulong,
            mask: 0o100000 as libc::c_int as libc::c_ulong,
        };
        init
    },
    {
        let mut init = mode_info {
            name: b"isig\0" as *const u8 as *const libc::c_char,
            type_0: local,
            flags: (1 as libc::c_int | 4 as libc::c_int) as libc::c_char,
            bits: 0o1 as libc::c_int as libc::c_ulong,
            mask: 0 as libc::c_int as libc::c_ulong,
        };
        init
    },
    {
        let mut init = mode_info {
            name: b"icanon\0" as *const u8 as *const libc::c_char,
            type_0: local,
            flags: (1 as libc::c_int | 4 as libc::c_int) as libc::c_char,
            bits: 0o2 as libc::c_int as libc::c_ulong,
            mask: 0 as libc::c_int as libc::c_ulong,
        };
        init
    },
    {
        let mut init = mode_info {
            name: b"iexten\0" as *const u8 as *const libc::c_char,
            type_0: local,
            flags: (1 as libc::c_int | 4 as libc::c_int) as libc::c_char,
            bits: 0o100000 as libc::c_int as libc::c_ulong,
            mask: 0 as libc::c_int as libc::c_ulong,
        };
        init
    },
    {
        let mut init = mode_info {
            name: b"echo\0" as *const u8 as *const libc::c_char,
            type_0: local,
            flags: (1 as libc::c_int | 4 as libc::c_int) as libc::c_char,
            bits: 0o10 as libc::c_int as libc::c_ulong,
            mask: 0 as libc::c_int as libc::c_ulong,
        };
        init
    },
    {
        let mut init = mode_info {
            name: b"echoe\0" as *const u8 as *const libc::c_char,
            type_0: local,
            flags: (1 as libc::c_int | 4 as libc::c_int) as libc::c_char,
            bits: 0o20 as libc::c_int as libc::c_ulong,
            mask: 0 as libc::c_int as libc::c_ulong,
        };
        init
    },
    {
        let mut init = mode_info {
            name: b"crterase\0" as *const u8 as *const libc::c_char,
            type_0: local,
            flags: (4 as libc::c_int | 8 as libc::c_int) as libc::c_char,
            bits: 0o20 as libc::c_int as libc::c_ulong,
            mask: 0 as libc::c_int as libc::c_ulong,
        };
        init
    },
    {
        let mut init = mode_info {
            name: b"echok\0" as *const u8 as *const libc::c_char,
            type_0: local,
            flags: (1 as libc::c_int | 4 as libc::c_int) as libc::c_char,
            bits: 0o40 as libc::c_int as libc::c_ulong,
            mask: 0 as libc::c_int as libc::c_ulong,
        };
        init
    },
    {
        let mut init = mode_info {
            name: b"echonl\0" as *const u8 as *const libc::c_char,
            type_0: local,
            flags: (2 as libc::c_int | 4 as libc::c_int) as libc::c_char,
            bits: 0o100 as libc::c_int as libc::c_ulong,
            mask: 0 as libc::c_int as libc::c_ulong,
        };
        init
    },
    {
        let mut init = mode_info {
            name: b"noflsh\0" as *const u8 as *const libc::c_char,
            type_0: local,
            flags: (2 as libc::c_int | 4 as libc::c_int) as libc::c_char,
            bits: 0o200 as libc::c_int as libc::c_ulong,
            mask: 0 as libc::c_int as libc::c_ulong,
        };
        init
    },
    {
        let mut init = mode_info {
            name: b"xcase\0" as *const u8 as *const libc::c_char,
            type_0: local,
            flags: (2 as libc::c_int | 4 as libc::c_int) as libc::c_char,
            bits: 0o4 as libc::c_int as libc::c_ulong,
            mask: 0 as libc::c_int as libc::c_ulong,
        };
        init
    },
    {
        let mut init = mode_info {
            name: b"tostop\0" as *const u8 as *const libc::c_char,
            type_0: local,
            flags: (2 as libc::c_int | 4 as libc::c_int) as libc::c_char,
            bits: 0o400 as libc::c_int as libc::c_ulong,
            mask: 0 as libc::c_int as libc::c_ulong,
        };
        init
    },
    {
        let mut init = mode_info {
            name: b"echoprt\0" as *const u8 as *const libc::c_char,
            type_0: local,
            flags: (2 as libc::c_int | 4 as libc::c_int) as libc::c_char,
            bits: 0o2000 as libc::c_int as libc::c_ulong,
            mask: 0 as libc::c_int as libc::c_ulong,
        };
        init
    },
    {
        let mut init = mode_info {
            name: b"prterase\0" as *const u8 as *const libc::c_char,
            type_0: local,
            flags: (4 as libc::c_int | 8 as libc::c_int) as libc::c_char,
            bits: 0o2000 as libc::c_int as libc::c_ulong,
            mask: 0 as libc::c_int as libc::c_ulong,
        };
        init
    },
    {
        let mut init = mode_info {
            name: b"echoctl\0" as *const u8 as *const libc::c_char,
            type_0: local,
            flags: (1 as libc::c_int | 4 as libc::c_int) as libc::c_char,
            bits: 0o1000 as libc::c_int as libc::c_ulong,
            mask: 0 as libc::c_int as libc::c_ulong,
        };
        init
    },
    {
        let mut init = mode_info {
            name: b"ctlecho\0" as *const u8 as *const libc::c_char,
            type_0: local,
            flags: (4 as libc::c_int | 8 as libc::c_int) as libc::c_char,
            bits: 0o1000 as libc::c_int as libc::c_ulong,
            mask: 0 as libc::c_int as libc::c_ulong,
        };
        init
    },
    {
        let mut init = mode_info {
            name: b"echoke\0" as *const u8 as *const libc::c_char,
            type_0: local,
            flags: (1 as libc::c_int | 4 as libc::c_int) as libc::c_char,
            bits: 0o4000 as libc::c_int as libc::c_ulong,
            mask: 0 as libc::c_int as libc::c_ulong,
        };
        init
    },
    {
        let mut init = mode_info {
            name: b"crtkill\0" as *const u8 as *const libc::c_char,
            type_0: local,
            flags: (4 as libc::c_int | 8 as libc::c_int) as libc::c_char,
            bits: 0o4000 as libc::c_int as libc::c_ulong,
            mask: 0 as libc::c_int as libc::c_ulong,
        };
        init
    },
    {
        let mut init = mode_info {
            name: b"flusho\0" as *const u8 as *const libc::c_char,
            type_0: local,
            flags: (2 as libc::c_int | 4 as libc::c_int) as libc::c_char,
            bits: 0o10000 as libc::c_int as libc::c_ulong,
            mask: 0 as libc::c_int as libc::c_ulong,
        };
        init
    },
    {
        let mut init = mode_info {
            name: b"extproc\0" as *const u8 as *const libc::c_char,
            type_0: local,
            flags: (2 as libc::c_int | 4 as libc::c_int) as libc::c_char,
            bits: 0o200000 as libc::c_int as libc::c_ulong,
            mask: 0 as libc::c_int as libc::c_ulong,
        };
        init
    },
    {
        let mut init = mode_info {
            name: b"evenp\0" as *const u8 as *const libc::c_char,
            type_0: combination,
            flags: (4 as libc::c_int | 8 as libc::c_int) as libc::c_char,
            bits: 0 as libc::c_int as libc::c_ulong,
            mask: 0 as libc::c_int as libc::c_ulong,
        };
        init
    },
    {
        let mut init = mode_info {
            name: b"parity\0" as *const u8 as *const libc::c_char,
            type_0: combination,
            flags: (4 as libc::c_int | 8 as libc::c_int) as libc::c_char,
            bits: 0 as libc::c_int as libc::c_ulong,
            mask: 0 as libc::c_int as libc::c_ulong,
        };
        init
    },
    {
        let mut init = mode_info {
            name: b"oddp\0" as *const u8 as *const libc::c_char,
            type_0: combination,
            flags: (4 as libc::c_int | 8 as libc::c_int) as libc::c_char,
            bits: 0 as libc::c_int as libc::c_ulong,
            mask: 0 as libc::c_int as libc::c_ulong,
        };
        init
    },
    {
        let mut init = mode_info {
            name: b"nl\0" as *const u8 as *const libc::c_char,
            type_0: combination,
            flags: (4 as libc::c_int | 8 as libc::c_int) as libc::c_char,
            bits: 0 as libc::c_int as libc::c_ulong,
            mask: 0 as libc::c_int as libc::c_ulong,
        };
        init
    },
    {
        let mut init = mode_info {
            name: b"ek\0" as *const u8 as *const libc::c_char,
            type_0: combination,
            flags: 8 as libc::c_int as libc::c_char,
            bits: 0 as libc::c_int as libc::c_ulong,
            mask: 0 as libc::c_int as libc::c_ulong,
        };
        init
    },
    {
        let mut init = mode_info {
            name: b"sane\0" as *const u8 as *const libc::c_char,
            type_0: combination,
            flags: 8 as libc::c_int as libc::c_char,
            bits: 0 as libc::c_int as libc::c_ulong,
            mask: 0 as libc::c_int as libc::c_ulong,
        };
        init
    },
    {
        let mut init = mode_info {
            name: b"cooked\0" as *const u8 as *const libc::c_char,
            type_0: combination,
            flags: (4 as libc::c_int | 8 as libc::c_int) as libc::c_char,
            bits: 0 as libc::c_int as libc::c_ulong,
            mask: 0 as libc::c_int as libc::c_ulong,
        };
        init
    },
    {
        let mut init = mode_info {
            name: b"raw\0" as *const u8 as *const libc::c_char,
            type_0: combination,
            flags: (4 as libc::c_int | 8 as libc::c_int) as libc::c_char,
            bits: 0 as libc::c_int as libc::c_ulong,
            mask: 0 as libc::c_int as libc::c_ulong,
        };
        init
    },
    {
        let mut init = mode_info {
            name: b"pass8\0" as *const u8 as *const libc::c_char,
            type_0: combination,
            flags: (4 as libc::c_int | 8 as libc::c_int) as libc::c_char,
            bits: 0 as libc::c_int as libc::c_ulong,
            mask: 0 as libc::c_int as libc::c_ulong,
        };
        init
    },
    {
        let mut init = mode_info {
            name: b"litout\0" as *const u8 as *const libc::c_char,
            type_0: combination,
            flags: (4 as libc::c_int | 8 as libc::c_int) as libc::c_char,
            bits: 0 as libc::c_int as libc::c_ulong,
            mask: 0 as libc::c_int as libc::c_ulong,
        };
        init
    },
    {
        let mut init = mode_info {
            name: b"cbreak\0" as *const u8 as *const libc::c_char,
            type_0: combination,
            flags: (4 as libc::c_int | 8 as libc::c_int) as libc::c_char,
            bits: 0 as libc::c_int as libc::c_ulong,
            mask: 0 as libc::c_int as libc::c_ulong,
        };
        init
    },
    {
        let mut init = mode_info {
            name: b"decctlq\0" as *const u8 as *const libc::c_char,
            type_0: combination,
            flags: (4 as libc::c_int | 8 as libc::c_int) as libc::c_char,
            bits: 0 as libc::c_int as libc::c_ulong,
            mask: 0 as libc::c_int as libc::c_ulong,
        };
        init
    },
    {
        let mut init = mode_info {
            name: b"tabs\0" as *const u8 as *const libc::c_char,
            type_0: combination,
            flags: (4 as libc::c_int | 8 as libc::c_int) as libc::c_char,
            bits: 0 as libc::c_int as libc::c_ulong,
            mask: 0 as libc::c_int as libc::c_ulong,
        };
        init
    },
    {
        let mut init = mode_info {
            name: b"lcase\0" as *const u8 as *const libc::c_char,
            type_0: combination,
            flags: (4 as libc::c_int | 8 as libc::c_int) as libc::c_char,
            bits: 0 as libc::c_int as libc::c_ulong,
            mask: 0 as libc::c_int as libc::c_ulong,
        };
        init
    },
    {
        let mut init = mode_info {
            name: b"LCASE\0" as *const u8 as *const libc::c_char,
            type_0: combination,
            flags: (4 as libc::c_int | 8 as libc::c_int) as libc::c_char,
            bits: 0 as libc::c_int as libc::c_ulong,
            mask: 0 as libc::c_int as libc::c_ulong,
        };
        init
    },
    {
        let mut init = mode_info {
            name: b"crt\0" as *const u8 as *const libc::c_char,
            type_0: combination,
            flags: 8 as libc::c_int as libc::c_char,
            bits: 0 as libc::c_int as libc::c_ulong,
            mask: 0 as libc::c_int as libc::c_ulong,
        };
        init
    },
    {
        let mut init = mode_info {
            name: b"dec\0" as *const u8 as *const libc::c_char,
            type_0: combination,
            flags: 8 as libc::c_int as libc::c_char,
            bits: 0 as libc::c_int as libc::c_ulong,
            mask: 0 as libc::c_int as libc::c_ulong,
        };
        init
    },
    {
        let mut init = mode_info {
            name: 0 as *const libc::c_char,
            type_0: control,
            flags: 0 as libc::c_int as libc::c_char,
            bits: 0 as libc::c_int as libc::c_ulong,
            mask: 0 as libc::c_int as libc::c_ulong,
        };
        init
    },
];
static mut control_info: [control_info; 19] = [
    {
        let mut init = control_info {
            name: b"intr\0" as *const u8 as *const libc::c_char,
            saneval: ('c' as i32 & 0o37 as libc::c_int) as cc_t,
            offset: 0 as libc::c_int as size_t,
        };
        init
    },
    {
        let mut init = control_info {
            name: b"quit\0" as *const u8 as *const libc::c_char,
            saneval: 0o34 as libc::c_int as cc_t,
            offset: 1 as libc::c_int as size_t,
        };
        init
    },
    {
        let mut init = control_info {
            name: b"erase\0" as *const u8 as *const libc::c_char,
            saneval: 0o177 as libc::c_int as cc_t,
            offset: 2 as libc::c_int as size_t,
        };
        init
    },
    {
        let mut init = control_info {
            name: b"kill\0" as *const u8 as *const libc::c_char,
            saneval: ('u' as i32 & 0o37 as libc::c_int) as cc_t,
            offset: 3 as libc::c_int as size_t,
        };
        init
    },
    {
        let mut init = control_info {
            name: b"eof\0" as *const u8 as *const libc::c_char,
            saneval: ('d' as i32 & 0o37 as libc::c_int) as cc_t,
            offset: 4 as libc::c_int as size_t,
        };
        init
    },
    {
        let mut init = control_info {
            name: b"eol\0" as *const u8 as *const libc::c_char,
            saneval: '\0' as i32 as cc_t,
            offset: 11 as libc::c_int as size_t,
        };
        init
    },
    {
        let mut init = control_info {
            name: b"eol2\0" as *const u8 as *const libc::c_char,
            saneval: '\0' as i32 as cc_t,
            offset: 16 as libc::c_int as size_t,
        };
        init
    },
    {
        let mut init = control_info {
            name: b"swtch\0" as *const u8 as *const libc::c_char,
            saneval: '\0' as i32 as cc_t,
            offset: 7 as libc::c_int as size_t,
        };
        init
    },
    {
        let mut init = control_info {
            name: b"start\0" as *const u8 as *const libc::c_char,
            saneval: ('q' as i32 & 0o37 as libc::c_int) as cc_t,
            offset: 8 as libc::c_int as size_t,
        };
        init
    },
    {
        let mut init = control_info {
            name: b"stop\0" as *const u8 as *const libc::c_char,
            saneval: ('s' as i32 & 0o37 as libc::c_int) as cc_t,
            offset: 9 as libc::c_int as size_t,
        };
        init
    },
    {
        let mut init = control_info {
            name: b"susp\0" as *const u8 as *const libc::c_char,
            saneval: ('z' as i32 & 0o37 as libc::c_int) as cc_t,
            offset: 10 as libc::c_int as size_t,
        };
        init
    },
    {
        let mut init = control_info {
            name: b"rprnt\0" as *const u8 as *const libc::c_char,
            saneval: ('r' as i32 & 0o37 as libc::c_int) as cc_t,
            offset: 12 as libc::c_int as size_t,
        };
        init
    },
    {
        let mut init = control_info {
            name: b"werase\0" as *const u8 as *const libc::c_char,
            saneval: ('w' as i32 & 0o37 as libc::c_int) as cc_t,
            offset: 14 as libc::c_int as size_t,
        };
        init
    },
    {
        let mut init = control_info {
            name: b"lnext\0" as *const u8 as *const libc::c_char,
            saneval: ('v' as i32 & 0o37 as libc::c_int) as cc_t,
            offset: 15 as libc::c_int as size_t,
        };
        init
    },
    {
        let mut init = control_info {
            name: b"flush\0" as *const u8 as *const libc::c_char,
            saneval: ('o' as i32 & 0x1f as libc::c_int) as cc_t,
            offset: 13 as libc::c_int as size_t,
        };
        init
    },
    {
        let mut init = control_info {
            name: b"discard\0" as *const u8 as *const libc::c_char,
            saneval: ('o' as i32 & 0x1f as libc::c_int) as cc_t,
            offset: 13 as libc::c_int as size_t,
        };
        init
    },
    {
        let mut init = control_info {
            name: b"min\0" as *const u8 as *const libc::c_char,
            saneval: 1 as libc::c_int as cc_t,
            offset: 6 as libc::c_int as size_t,
        };
        init
    },
    {
        let mut init = control_info {
            name: b"time\0" as *const u8 as *const libc::c_char,
            saneval: 0 as libc::c_int as cc_t,
            offset: 5 as libc::c_int as size_t,
        };
        init
    },
    {
        let mut init = control_info {
            name: 0 as *const libc::c_char,
            saneval: 0 as libc::c_int as cc_t,
            offset: 0 as libc::c_int as size_t,
        };
        init
    },
];
static mut max_col: libc::c_int = 0;
static mut current_col: libc::c_int = 0;
static mut tcsetattr_options: libc::c_int = 1 as libc::c_int;
static mut dev_debug: bool = false;
static mut last_ibaud: speed_t = -(1 as libc::c_int) as speed_t;
static mut last_obaud: speed_t = -(1 as libc::c_int) as speed_t;
static mut longopts: [option; 7] = [
    {
        let mut init = option {
            name: b"all\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'a' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"save\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'g' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"file\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'F' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"-debug\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: DEV_DEBUG_OPTION as libc::c_int,
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
unsafe extern "C" fn wrapf(mut message: *const libc::c_char, mut args: ...) {
    let mut args_0: ::core::ffi::VaListImpl;
    let mut buf: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut buflen: libc::c_int = 0;
    args_0 = args.clone();
    buflen = rpl_vasprintf(&mut buf, message, args_0.as_va_list());
    if buflen < 0 as libc::c_int {
        xalloc_die();
    }
    if (0 as libc::c_int) < current_col {
        if max_col - current_col <= buflen {
            putchar_unlocked('\n' as i32);
            current_col = 0 as libc::c_int;
        } else {
            putchar_unlocked(' ' as i32);
            current_col += 1;
        }
    }
    fputs_unlocked(buf, stdout);
    free(buf as *mut libc::c_void);
    current_col += buflen;
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
                b"Usage: %s [-F DEVICE | --file=DEVICE] [SETTING]...\n  or:  %s [-F DEVICE | --file=DEVICE] [-a|--all]\n  or:  %s [-F DEVICE | --file=DEVICE] [-g|--save]\n\0"
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
                b"Print or change terminal characteristics.\n\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
            stdout,
        );
        emit_mandatory_arg_note();
        fputs_unlocked(
            dcgettext(
                0 as *const libc::c_char,
                b"  -a, --all          print all current settings in human-readable form\n  -g, --save         print all current settings in a stty-readable form\n  -F, --file=DEVICE  open and use the specified DEVICE instead of stdin\n\0"
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
                b"\nOptional - before SETTING indicates negation.  An * marks non-POSIX\nsettings.  The underlying system defines which settings are available.\n\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            stdout,
        );
        fputs_unlocked(
            dcgettext(
                0 as *const libc::c_char,
                b"\nSpecial characters:\n\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            stdout,
        );
        fputs_unlocked(
            dcgettext(
                0 as *const libc::c_char,
                b" * discard CHAR  CHAR will toggle discarding of output\n\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            stdout,
        );
        fputs_unlocked(
            dcgettext(
                0 as *const libc::c_char,
                b"   eof CHAR      CHAR will send an end of file (terminate the input)\n   eol CHAR      CHAR will end the line\n\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            stdout,
        );
        fputs_unlocked(
            dcgettext(
                0 as *const libc::c_char,
                b" * eol2 CHAR     alternate CHAR for ending the line\n\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
            stdout,
        );
        fputs_unlocked(
            dcgettext(
                0 as *const libc::c_char,
                b"   erase CHAR    CHAR will erase the last character typed\n   intr CHAR     CHAR will send an interrupt signal\n   kill CHAR     CHAR will erase the current line\n\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            stdout,
        );
        fputs_unlocked(
            dcgettext(
                0 as *const libc::c_char,
                b" * lnext CHAR    CHAR will enter the next character quoted\n\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            stdout,
        );
        fputs_unlocked(
            dcgettext(
                0 as *const libc::c_char,
                b"   quit CHAR     CHAR will send a quit signal\n\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
            stdout,
        );
        fputs_unlocked(
            dcgettext(
                0 as *const libc::c_char,
                b" * rprnt CHAR    CHAR will redraw the current line\n\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
            stdout,
        );
        fputs_unlocked(
            dcgettext(
                0 as *const libc::c_char,
                b"   start CHAR    CHAR will restart the output after stopping it\n   stop CHAR     CHAR will stop the output\n   susp CHAR     CHAR will send a terminal stop signal\n\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            stdout,
        );
        fputs_unlocked(
            dcgettext(
                0 as *const libc::c_char,
                b" * swtch CHAR    CHAR will switch to a different shell layer\n\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            stdout,
        );
        fputs_unlocked(
            dcgettext(
                0 as *const libc::c_char,
                b" * werase CHAR   CHAR will erase the last word typed\n\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
            stdout,
        );
        fputs_unlocked(
            dcgettext(
                0 as *const libc::c_char,
                b"\nSpecial settings:\n   N             set the input and output speeds to N bauds\n\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            stdout,
        );
        fputs_unlocked(
            dcgettext(
                0 as *const libc::c_char,
                b" * cols N        tell the kernel that the terminal has N columns\n * columns N     same as cols N\n\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            stdout,
        );
        printf(
            dcgettext(
                0 as *const libc::c_char,
                b" * [-]drain      wait for transmission before applying settings (%s by default)\n\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            if tcsetattr_options == 1 as libc::c_int {
                dcgettext(
                    0 as *const libc::c_char,
                    b"on\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                )
            } else {
                dcgettext(
                    0 as *const libc::c_char,
                    b"off\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                )
            },
        );
        fputs_unlocked(
            dcgettext(
                0 as *const libc::c_char,
                b"   ispeed N      set the input speed to N\n\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
            stdout,
        );
        fputs_unlocked(
            dcgettext(
                0 as *const libc::c_char,
                b" * line N        use line discipline N\n\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
            stdout,
        );
        fputs_unlocked(
            dcgettext(
                0 as *const libc::c_char,
                b"   min N         with -icanon, set N characters minimum for a completed read\n   ospeed N      set the output speed to N\n\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            stdout,
        );
        fputs_unlocked(
            dcgettext(
                0 as *const libc::c_char,
                b" * rows N        tell the kernel that the terminal has N rows\n * size          print the number of rows and columns according to the kernel\n\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            stdout,
        );
        fputs_unlocked(
            dcgettext(
                0 as *const libc::c_char,
                b"   speed         print the terminal speed\n   time N        with -icanon, set read timeout of N tenths of a second\n\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            stdout,
        );
        fputs_unlocked(
            dcgettext(
                0 as *const libc::c_char,
                b"\nControl settings:\n   [-]clocal     disable modem control signals\n   [-]cread      allow input to be received\n\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            stdout,
        );
        fputs_unlocked(
            dcgettext(
                0 as *const libc::c_char,
                b" * [-]crtscts    enable RTS/CTS handshaking\n\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
            stdout,
        );
        fputs_unlocked(
            dcgettext(
                0 as *const libc::c_char,
                b"   csN           set character size to N bits, N in [5..8]\n\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            stdout,
        );
        fputs_unlocked(
            dcgettext(
                0 as *const libc::c_char,
                b"   [-]cstopb     use two stop bits per character (one with '-')\n   [-]hup        send a hangup signal when the last process closes the tty\n   [-]hupcl      same as [-]hup\n   [-]parenb     generate parity bit in output and expect parity bit in input\n   [-]parodd     set odd parity (or even parity with '-')\n\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            stdout,
        );
        fputs_unlocked(
            dcgettext(
                0 as *const libc::c_char,
                b" * [-]cmspar     use \"stick\" (mark/space) parity\n\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
            stdout,
        );
        fputs_unlocked(
            dcgettext(
                0 as *const libc::c_char,
                b"\nInput settings:\n   [-]brkint     breaks cause an interrupt signal\n   [-]icrnl      translate carriage return to newline\n   [-]ignbrk     ignore break characters\n   [-]igncr      ignore carriage return\n   [-]ignpar     ignore characters with parity errors\n\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            stdout,
        );
        fputs_unlocked(
            dcgettext(
                0 as *const libc::c_char,
                b" * [-]imaxbel    beep and do not flush a full input buffer on a character\n\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            stdout,
        );
        fputs_unlocked(
            dcgettext(
                0 as *const libc::c_char,
                b"   [-]inlcr      translate newline to carriage return\n   [-]inpck      enable input parity checking\n   [-]istrip     clear high (8th) bit of input characters\n\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            stdout,
        );
        fputs_unlocked(
            dcgettext(
                0 as *const libc::c_char,
                b" * [-]iutf8      assume input characters are UTF-8 encoded\n\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            stdout,
        );
        fputs_unlocked(
            dcgettext(
                0 as *const libc::c_char,
                b" * [-]iuclc      translate uppercase characters to lowercase\n\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            stdout,
        );
        fputs_unlocked(
            dcgettext(
                0 as *const libc::c_char,
                b" * [-]ixany      let any character restart output, not only start character\n\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            stdout,
        );
        fputs_unlocked(
            dcgettext(
                0 as *const libc::c_char,
                b"   [-]ixoff      enable sending of start/stop characters\n   [-]ixon       enable XON/XOFF flow control\n   [-]parmrk     mark parity errors (with a 255-0-character sequence)\n   [-]tandem     same as [-]ixoff\n\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            stdout,
        );
        fputs_unlocked(
            dcgettext(
                0 as *const libc::c_char,
                b"\nOutput settings:\n\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            stdout,
        );
        fputs_unlocked(
            dcgettext(
                0 as *const libc::c_char,
                b" * bsN           backspace delay style, N in [0..1]\n\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
            stdout,
        );
        fputs_unlocked(
            dcgettext(
                0 as *const libc::c_char,
                b" * crN           carriage return delay style, N in [0..3]\n\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            stdout,
        );
        fputs_unlocked(
            dcgettext(
                0 as *const libc::c_char,
                b" * ffN           form feed delay style, N in [0..1]\n\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
            stdout,
        );
        fputs_unlocked(
            dcgettext(
                0 as *const libc::c_char,
                b" * nlN           newline delay style, N in [0..1]\n\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
            stdout,
        );
        fputs_unlocked(
            dcgettext(
                0 as *const libc::c_char,
                b" * [-]ocrnl      translate carriage return to newline\n\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
            stdout,
        );
        fputs_unlocked(
            dcgettext(
                0 as *const libc::c_char,
                b" * [-]ofdel      use delete characters for fill instead of NUL characters\n\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            stdout,
        );
        fputs_unlocked(
            dcgettext(
                0 as *const libc::c_char,
                b" * [-]ofill      use fill (padding) characters instead of timing for delays\n\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            stdout,
        );
        fputs_unlocked(
            dcgettext(
                0 as *const libc::c_char,
                b" * [-]olcuc      translate lowercase characters to uppercase\n\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            stdout,
        );
        fputs_unlocked(
            dcgettext(
                0 as *const libc::c_char,
                b" * [-]onlcr      translate newline to carriage return-newline\n\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            stdout,
        );
        fputs_unlocked(
            dcgettext(
                0 as *const libc::c_char,
                b" * [-]onlret     newline performs a carriage return\n\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
            stdout,
        );
        fputs_unlocked(
            dcgettext(
                0 as *const libc::c_char,
                b" * [-]onocr      do not print carriage returns in the first column\n\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            stdout,
        );
        fputs_unlocked(
            dcgettext(
                0 as *const libc::c_char,
                b"   [-]opost      postprocess output\n\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
            stdout,
        );
        fputs_unlocked(
            dcgettext(
                0 as *const libc::c_char,
                b" * tabN          horizontal tab delay style, N in [0..3]\n * tabs          same as tab0\n * -tabs         same as tab3\n\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            stdout,
        );
        fputs_unlocked(
            dcgettext(
                0 as *const libc::c_char,
                b" * vtN           vertical tab delay style, N in [0..1]\n\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            stdout,
        );
        fputs_unlocked(
            dcgettext(
                0 as *const libc::c_char,
                b"\nLocal settings:\n   [-]crterase   echo erase characters as backspace-space-backspace\n\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            stdout,
        );
        fputs_unlocked(
            dcgettext(
                0 as *const libc::c_char,
                b" * crtkill       kill all line by obeying the echoprt and echoe settings\n * -crtkill      kill all line by obeying the echoctl and echok settings\n\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            stdout,
        );
        fputs_unlocked(
            dcgettext(
                0 as *const libc::c_char,
                b" * [-]ctlecho    echo control characters in hat notation ('^c')\n\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            stdout,
        );
        fputs_unlocked(
            dcgettext(
                0 as *const libc::c_char,
                b"   [-]echo       echo input characters\n\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
            stdout,
        );
        fputs_unlocked(
            dcgettext(
                0 as *const libc::c_char,
                b" * [-]echoctl    same as [-]ctlecho\n\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
            stdout,
        );
        fputs_unlocked(
            dcgettext(
                0 as *const libc::c_char,
                b"   [-]echoe      same as [-]crterase\n   [-]echok      echo a newline after a kill character\n\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            stdout,
        );
        fputs_unlocked(
            dcgettext(
                0 as *const libc::c_char,
                b" * [-]echoke     same as [-]crtkill\n\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
            stdout,
        );
        fputs_unlocked(
            dcgettext(
                0 as *const libc::c_char,
                b"   [-]echonl     echo newline even if not echoing other characters\n\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            stdout,
        );
        fputs_unlocked(
            dcgettext(
                0 as *const libc::c_char,
                b" * [-]echoprt    echo erased characters backward, between '\\' and '/'\n\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            stdout,
        );
        fputs_unlocked(
            dcgettext(
                0 as *const libc::c_char,
                b" * [-]extproc    enable \"LINEMODE\"; useful with high latency links\n\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            stdout,
        );
        fputs_unlocked(
            dcgettext(
                0 as *const libc::c_char,
                b" * [-]flusho     discard output\n\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
            stdout,
        );
        printf(
            dcgettext(
                0 as *const libc::c_char,
                b"   [-]icanon     enable special characters: %s\n   [-]iexten     enable non-POSIX special characters\n\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            b"erase, kill, werase, rprnt\0" as *const u8 as *const libc::c_char,
        );
        fputs_unlocked(
            dcgettext(
                0 as *const libc::c_char,
                b"   [-]isig       enable interrupt, quit, and suspend special characters\n   [-]noflsh     disable flushing after interrupt and quit special characters\n\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            stdout,
        );
        fputs_unlocked(
            dcgettext(
                0 as *const libc::c_char,
                b" * [-]prterase   same as [-]echoprt\n\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
            stdout,
        );
        fputs_unlocked(
            dcgettext(
                0 as *const libc::c_char,
                b" * [-]tostop     stop background jobs that try to write to the terminal\n\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            stdout,
        );
        fputs_unlocked(
            dcgettext(
                0 as *const libc::c_char,
                b" * [-]xcase      with icanon, escape with '\\' for uppercase characters\n\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            stdout,
        );
        fputs_unlocked(
            dcgettext(
                0 as *const libc::c_char,
                b"\nCombination settings:\n\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            stdout,
        );
        fputs_unlocked(
            dcgettext(
                0 as *const libc::c_char,
                b" * [-]LCASE      same as [-]lcase\n\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
            stdout,
        );
        fputs_unlocked(
            dcgettext(
                0 as *const libc::c_char,
                b"   cbreak        same as -icanon\n   -cbreak       same as icanon\n\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            stdout,
        );
        fputs_unlocked(
            dcgettext(
                0 as *const libc::c_char,
                b"   cooked        same as brkint ignpar istrip icrnl ixon opost isig\n                 icanon, eof and eol characters to their default values\n   -cooked       same as raw\n\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            stdout,
        );
        printf(
            dcgettext(
                0 as *const libc::c_char,
                b"   crt           same as %s\n\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            b"echoe echoctl echoke\0" as *const u8 as *const libc::c_char,
        );
        printf(
            dcgettext(
                0 as *const libc::c_char,
                b"   dec           same as %s intr ^c erase 0177\n                 kill ^u\n\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            b"echoe echoctl echoke -ixany\0" as *const u8 as *const libc::c_char,
        );
        fputs_unlocked(
            dcgettext(
                0 as *const libc::c_char,
                b" * [-]decctlq    same as [-]ixany\n\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
            stdout,
        );
        fputs_unlocked(
            dcgettext(
                0 as *const libc::c_char,
                b"   ek            erase and kill characters to their default values\n   evenp         same as parenb -parodd cs7\n   -evenp        same as -parenb cs8\n\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            stdout,
        );
        fputs_unlocked(
            dcgettext(
                0 as *const libc::c_char,
                b" * [-]lcase      same as xcase iuclc olcuc\n\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
            stdout,
        );
        fputs_unlocked(
            dcgettext(
                0 as *const libc::c_char,
                b"   litout        same as -parenb -istrip -opost cs8\n   -litout       same as parenb istrip opost cs7\n\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            stdout,
        );
        printf(
            dcgettext(
                0 as *const libc::c_char,
                b"   nl            same as %s\n   -nl           same as %s\n\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            b"-icrnl -onlcr\0" as *const u8 as *const libc::c_char,
            b"icrnl -inlcr -igncr onlcr -ocrnl -onlret\0" as *const u8
                as *const libc::c_char,
        );
        fputs_unlocked(
            dcgettext(
                0 as *const libc::c_char,
                b"   oddp          same as parenb parodd cs7\n   -oddp         same as -parenb cs8\n   [-]parity     same as [-]evenp\n   pass8         same as -parenb -istrip cs8\n   -pass8        same as parenb istrip cs7\n\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            stdout,
        );
        printf(
            dcgettext(
                0 as *const libc::c_char,
                b"   raw           same as -ignbrk -brkint -ignpar -parmrk -inpck -istrip\n                 -inlcr -igncr -icrnl -ixon -ixoff -icanon -opost\n                 -isig%s min 1 time 0\n   -raw          same as cooked\n\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            b" -iuclc -ixany -imaxbel -xcase\0" as *const u8 as *const libc::c_char,
        );
        printf(
            dcgettext(
                0 as *const libc::c_char,
                b"   sane          same as cread -ignbrk brkint -inlcr -igncr icrnl\n                 icanon iexten echo echoe echok -echonl -noflsh\n                 %s\n                 %s\n                 %s,\n                 all special characters to their default values\n\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            b"-ixoff -iutf8 -iuclc -ixany imaxbel -xcase -olcuc -ocrnl\0" as *const u8
                as *const libc::c_char,
            b"opost -ofill onlcr -onocr -onlret nl0 cr0 tab0 bs0 vt0 ff0\0" as *const u8
                as *const libc::c_char,
            b"isig -tostop -ofdel -echoprt echoctl echoke -extproc -flusho\0"
                as *const u8 as *const libc::c_char,
        );
        fputs_unlocked(
            dcgettext(
                0 as *const libc::c_char,
                b"\nHandle the tty line connected to standard input.  Without arguments,\nprints baud rate, line discipline, and deviations from stty sane.  In\nsettings, CHAR is taken literally, or coded as in ^c, 0x37, 0177 or\n127; special values ^- or undef used to disable special characters.\n\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            stdout,
        );
        emit_ancillary_info(b"stty\0" as *const u8 as *const libc::c_char);
    }
    exit(status);
}
unsafe extern "C" fn apply_settings(
    mut checking: bool,
    mut device_name: *const libc::c_char,
    mut settings: *const *mut libc::c_char,
    mut n_settings: libc::c_int,
    mut mode: *mut termios,
    mut require_set_attr: *mut bool,
) {
    let mut k: libc::c_int = 1 as libc::c_int;
    while k < n_settings {
        let mut arg: *const libc::c_char = *settings.offset(k as isize);
        let mut match_found: bool = 0 as libc::c_int != 0;
        let mut not_set_attr: bool = 0 as libc::c_int != 0;
        let mut reversed: bool = 0 as libc::c_int != 0;
        let mut i: libc::c_int = 0;
        if !arg.is_null() {
            if *arg.offset(0 as libc::c_int as isize) as libc::c_int == '-' as i32 {
                arg = arg.offset(1);
                reversed = 1 as libc::c_int != 0;
            }
            if strcmp(arg, b"drain\0" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
            {
                tcsetattr_options = if reversed as libc::c_int != 0 {
                    0 as libc::c_int
                } else {
                    1 as libc::c_int
                };
            } else {
                i = 0 as libc::c_int;
                while !(mode_info[i as usize].name).is_null() {
                    if strcmp(arg, mode_info[i as usize].name) == 0 as libc::c_int {
                        if mode_info[i as usize].flags as libc::c_int & 16 as libc::c_int
                            == 0 as libc::c_int
                        {
                            match_found = set_mode(
                                &*mode_info.as_ptr().offset(i as isize),
                                reversed,
                                mode,
                            );
                            *require_set_attr = 1 as libc::c_int != 0;
                        } else {
                            not_set_attr = 1 as libc::c_int != 0;
                            match_found = not_set_attr;
                        }
                        break;
                    } else {
                        i += 1;
                    }
                }
                if !match_found && reversed as libc::c_int != 0 {
                    error(
                        0 as libc::c_int,
                        0 as libc::c_int,
                        dcgettext(
                            0 as *const libc::c_char,
                            b"invalid argument %s\0" as *const u8 as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                        quote(arg.offset(-(1 as libc::c_int as isize))),
                    );
                    usage(1 as libc::c_int);
                }
                if !match_found {
                    i = 0 as libc::c_int;
                    while !(control_info[i as usize].name).is_null() {
                        if strcmp(arg, control_info[i as usize].name) == 0 as libc::c_int
                        {
                            if k == n_settings - 1 as libc::c_int
                                || (*settings.offset((k + 1 as libc::c_int) as isize))
                                    .is_null()
                            {
                                error(
                                    0 as libc::c_int,
                                    0 as libc::c_int,
                                    dcgettext(
                                        0 as *const libc::c_char,
                                        b"missing argument to %s\0" as *const u8
                                            as *const libc::c_char,
                                        5 as libc::c_int,
                                    ),
                                    quote(arg),
                                );
                                usage(1 as libc::c_int);
                            }
                            match_found = 1 as libc::c_int != 0;
                            k += 1;
                            set_control_char(
                                &*control_info.as_ptr().offset(i as isize),
                                *settings.offset(k as isize),
                                mode,
                            );
                            *require_set_attr = 1 as libc::c_int != 0;
                            break;
                        } else {
                            i += 1;
                        }
                    }
                }
                if !match_found || not_set_attr as libc::c_int != 0 {
                    if strcmp(arg, b"ispeed\0" as *const u8 as *const libc::c_char)
                        == 0 as libc::c_int
                    {
                        if k == n_settings - 1 as libc::c_int
                            || (*settings.offset((k + 1 as libc::c_int) as isize))
                                .is_null()
                        {
                            error(
                                0 as libc::c_int,
                                0 as libc::c_int,
                                dcgettext(
                                    0 as *const libc::c_char,
                                    b"missing argument to %s\0" as *const u8
                                        as *const libc::c_char,
                                    5 as libc::c_int,
                                ),
                                quote(arg),
                            );
                            usage(1 as libc::c_int);
                        }
                        k += 1;
                        if string_to_baud(*settings.offset(k as isize))
                            == -(1 as libc::c_int) as speed_t
                        {
                            error(
                                0 as libc::c_int,
                                0 as libc::c_int,
                                dcgettext(
                                    0 as *const libc::c_char,
                                    b"invalid ispeed %s\0" as *const u8 as *const libc::c_char,
                                    5 as libc::c_int,
                                ),
                                quote(*settings.offset(k as isize)),
                            );
                            usage(1 as libc::c_int);
                        }
                        set_speed(input_speed, *settings.offset(k as isize), mode);
                        if !checking {
                            *require_set_attr = 1 as libc::c_int != 0;
                        }
                    } else if strcmp(
                        arg,
                        b"ospeed\0" as *const u8 as *const libc::c_char,
                    ) == 0 as libc::c_int
                    {
                        if k == n_settings - 1 as libc::c_int
                            || (*settings.offset((k + 1 as libc::c_int) as isize))
                                .is_null()
                        {
                            error(
                                0 as libc::c_int,
                                0 as libc::c_int,
                                dcgettext(
                                    0 as *const libc::c_char,
                                    b"missing argument to %s\0" as *const u8
                                        as *const libc::c_char,
                                    5 as libc::c_int,
                                ),
                                quote(arg),
                            );
                            usage(1 as libc::c_int);
                        }
                        k += 1;
                        if string_to_baud(*settings.offset(k as isize))
                            == -(1 as libc::c_int) as speed_t
                        {
                            error(
                                0 as libc::c_int,
                                0 as libc::c_int,
                                dcgettext(
                                    0 as *const libc::c_char,
                                    b"invalid ospeed %s\0" as *const u8 as *const libc::c_char,
                                    5 as libc::c_int,
                                ),
                                quote(*settings.offset(k as isize)),
                            );
                            usage(1 as libc::c_int);
                        }
                        set_speed(output_speed, *settings.offset(k as isize), mode);
                        if !checking {
                            *require_set_attr = 1 as libc::c_int != 0;
                        }
                    } else if strcmp(arg, b"rows\0" as *const u8 as *const libc::c_char)
                        == 0 as libc::c_int
                    {
                        if k == n_settings - 1 as libc::c_int
                            || (*settings.offset((k + 1 as libc::c_int) as isize))
                                .is_null()
                        {
                            error(
                                0 as libc::c_int,
                                0 as libc::c_int,
                                dcgettext(
                                    0 as *const libc::c_char,
                                    b"missing argument to %s\0" as *const u8
                                        as *const libc::c_char,
                                    5 as libc::c_int,
                                ),
                                quote(arg),
                            );
                            usage(1 as libc::c_int);
                        }
                        k += 1;
                        if !checking {
                            set_window_size(
                                integer_arg(
                                    *settings.offset(k as isize),
                                    2147483647 as libc::c_int as libc::c_ulong,
                                ) as libc::c_int,
                                -(1 as libc::c_int),
                                device_name,
                            );
                        }
                    } else if strcmp(arg, b"cols\0" as *const u8 as *const libc::c_char)
                        == 0 as libc::c_int
                        || strcmp(arg, b"columns\0" as *const u8 as *const libc::c_char)
                            == 0 as libc::c_int
                    {
                        if k == n_settings - 1 as libc::c_int
                            || (*settings.offset((k + 1 as libc::c_int) as isize))
                                .is_null()
                        {
                            error(
                                0 as libc::c_int,
                                0 as libc::c_int,
                                dcgettext(
                                    0 as *const libc::c_char,
                                    b"missing argument to %s\0" as *const u8
                                        as *const libc::c_char,
                                    5 as libc::c_int,
                                ),
                                quote(arg),
                            );
                            usage(1 as libc::c_int);
                        }
                        k += 1;
                        if !checking {
                            set_window_size(
                                -(1 as libc::c_int),
                                integer_arg(
                                    *settings.offset(k as isize),
                                    2147483647 as libc::c_int as libc::c_ulong,
                                ) as libc::c_int,
                                device_name,
                            );
                        }
                    } else if strcmp(arg, b"size\0" as *const u8 as *const libc::c_char)
                        == 0 as libc::c_int
                    {
                        if !checking {
                            max_col = screen_columns();
                            current_col = 0 as libc::c_int;
                            display_window_size(0 as libc::c_int != 0, device_name);
                        }
                    } else if strcmp(arg, b"line\0" as *const u8 as *const libc::c_char)
                        == 0 as libc::c_int
                    {
                        let mut value: libc::c_ulong = 0;
                        if k == n_settings - 1 as libc::c_int
                            || (*settings.offset((k + 1 as libc::c_int) as isize))
                                .is_null()
                        {
                            error(
                                0 as libc::c_int,
                                0 as libc::c_int,
                                dcgettext(
                                    0 as *const libc::c_char,
                                    b"missing argument to %s\0" as *const u8
                                        as *const libc::c_char,
                                    5 as libc::c_int,
                                ),
                                quote(arg),
                            );
                            usage(1 as libc::c_int);
                        }
                        k += 1;
                        value = integer_arg(
                            *settings.offset(k as isize),
                            (9223372036854775807 as libc::c_long as libc::c_ulong)
                                .wrapping_mul(2 as libc::c_ulong)
                                .wrapping_add(1 as libc::c_ulong),
                        );
                        (*mode).c_line = value as cc_t;
                        if (*mode).c_line as libc::c_ulong != value {
                            error(
                                0 as libc::c_int,
                                0 as libc::c_int,
                                dcgettext(
                                    0 as *const libc::c_char,
                                    b"invalid line discipline %s\0" as *const u8
                                        as *const libc::c_char,
                                    5 as libc::c_int,
                                ),
                                quote(*settings.offset(k as isize)),
                            );
                        }
                        *require_set_attr = 1 as libc::c_int != 0;
                    } else if strcmp(arg, b"speed\0" as *const u8 as *const libc::c_char)
                        == 0 as libc::c_int
                    {
                        if !checking {
                            max_col = screen_columns();
                            display_speed(mode, 0 as libc::c_int != 0);
                        }
                    } else if string_to_baud(arg) != -(1 as libc::c_int) as speed_t {
                        set_speed(both_speeds, arg, mode);
                        if !checking {
                            *require_set_attr = 1 as libc::c_int != 0;
                        }
                    } else {
                        if !recover_mode(arg, mode) {
                            error(
                                0 as libc::c_int,
                                0 as libc::c_int,
                                dcgettext(
                                    0 as *const libc::c_char,
                                    b"invalid argument %s\0" as *const u8
                                        as *const libc::c_char,
                                    5 as libc::c_int,
                                ),
                                quote(arg),
                            );
                            usage(1 as libc::c_int);
                        }
                        *require_set_attr = 1 as libc::c_int != 0;
                    }
                }
            }
        }
        k += 1;
    }
    if checking {
        check_speed(mode);
    }
}
unsafe fn main_0(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut mode: termios = termios {
        c_iflag: 0,
        c_oflag: 0,
        c_cflag: 0,
        c_lflag: 0,
        c_line: 0,
        c_cc: [0; 32],
        c_ispeed: 0,
        c_ospeed: 0,
    };
    let mut output_type: output_type = changed;
    let mut optc: libc::c_int = 0;
    let mut argi: libc::c_int = 0 as libc::c_int;
    let mut opti: libc::c_int = 1 as libc::c_int;
    let mut require_set_attr: bool = false;
    let mut verbose_output: bool = false;
    let mut recoverable_output: bool = false;
    let mut noargs: bool = 1 as libc::c_int != 0;
    let mut file_name: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut device_name: *const libc::c_char = 0 as *const libc::c_char;
    set_program_name(*argv.offset(0 as libc::c_int as isize));
    setlocale(6 as libc::c_int, b"\0" as *const u8 as *const libc::c_char);
    bindtextdomain(
        b"coreutils\0" as *const u8 as *const libc::c_char,
        b"/usr/local/share/locale\0" as *const u8 as *const libc::c_char,
    );
    textdomain(b"coreutils\0" as *const u8 as *const libc::c_char);
    atexit(Some(close_stdout as unsafe extern "C" fn() -> ()));
    output_type = changed;
    verbose_output = 0 as libc::c_int != 0;
    recoverable_output = 0 as libc::c_int != 0;
    opterr = 0 as libc::c_int;
    loop {
        optc = getopt_long(
            argc - argi,
            argv.offset(argi as isize),
            b"-agF:\0" as *const u8 as *const libc::c_char,
            longopts.as_ptr(),
            0 as *mut libc::c_int,
        );
        if !(optc != -(1 as libc::c_int)) {
            break;
        }
        match optc {
            97 => {
                verbose_output = 1 as libc::c_int != 0;
                output_type = all;
            }
            103 => {
                recoverable_output = 1 as libc::c_int != 0;
                output_type = recoverable;
            }
            70 => {
                if !file_name.is_null() {
                    if ::core::mem::size_of::<C2RustUnnamed_16>() as libc::c_ulong != 0 {
                        error(
                            1 as libc::c_int,
                            0 as libc::c_int,
                            dcgettext(
                                0 as *const libc::c_char,
                                b"only one device may be specified\0" as *const u8
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
                                b"only one device may be specified\0" as *const u8
                                    as *const libc::c_char,
                                5 as libc::c_int,
                            ),
                        );
                        if 0 as libc::c_int != 0 {} else {
                            unreachable!();
                        };
                    };
                }
                file_name = optarg;
            }
            256 => {
                dev_debug = 1 as libc::c_int != 0;
            }
            -2 => {
                usage(0 as libc::c_int);
            }
            -3 => {
                version_etc(
                    stdout,
                    b"stty\0" as *const u8 as *const libc::c_char,
                    b"GNU coreutils\0" as *const u8 as *const libc::c_char,
                    Version,
                    b"David MacKenzie\0" as *const u8 as *const libc::c_char,
                    0 as *mut libc::c_void as *mut libc::c_char,
                );
                exit(0 as libc::c_int);
            }
            _ => {
                if !(strcmp(
                    *argv.offset((argi + opti) as isize),
                    b"-drain\0" as *const u8 as *const libc::c_char,
                ) == 0 as libc::c_int)
                    && !(strcmp(
                        *argv.offset((argi + opti) as isize),
                        b"drain\0" as *const u8 as *const libc::c_char,
                    ) == 0 as libc::c_int)
                {
                    noargs = 0 as libc::c_int != 0;
                }
                argi += opti;
                opti = 1 as libc::c_int;
                optind = 0 as libc::c_int;
            }
        }
        while opti < optind {
            let fresh1 = opti;
            opti = opti + 1;
            let ref mut fresh2 = *argv.offset((argi + fresh1) as isize);
            *fresh2 = 0 as *mut libc::c_char;
        }
    }
    if verbose_output as libc::c_int != 0 && recoverable_output as libc::c_int != 0 {
        if ::core::mem::size_of::<C2RustUnnamed_15>() as libc::c_ulong != 0 {
            error(
                1 as libc::c_int,
                0 as libc::c_int,
                dcgettext(
                    0 as *const libc::c_char,
                    b"the options for verbose and stty-readable output styles are\nmutually exclusive\0"
                        as *const u8 as *const libc::c_char,
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
                    b"the options for verbose and stty-readable output styles are\nmutually exclusive\0"
                        as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
            );
            if 0 as libc::c_int != 0 {} else {
                unreachable!();
            };
        };
    }
    if !noargs
        && (verbose_output as libc::c_int != 0 || recoverable_output as libc::c_int != 0)
    {
        if ::core::mem::size_of::<C2RustUnnamed_14>() as libc::c_ulong != 0 {
            error(
                1 as libc::c_int,
                0 as libc::c_int,
                dcgettext(
                    0 as *const libc::c_char,
                    b"when specifying an output style, modes may not be set\0"
                        as *const u8 as *const libc::c_char,
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
                    b"when specifying an output style, modes may not be set\0"
                        as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
            );
            if 0 as libc::c_int != 0 {} else {
                unreachable!();
            };
        };
    }
    device_name = if !file_name.is_null() {
        file_name
    } else {
        dcgettext(
            0 as *const libc::c_char,
            b"standard input\0" as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        )
    };
    if !noargs && !verbose_output && !recoverable_output {
        static mut check_mode: termios = termios {
            c_iflag: 0,
            c_oflag: 0,
            c_cflag: 0,
            c_lflag: 0,
            c_line: 0,
            c_cc: [0; 32],
            c_ispeed: 0,
            c_ospeed: 0,
        };
        apply_settings(
            1 as libc::c_int != 0,
            device_name,
            argv,
            argc,
            &mut check_mode,
            &mut require_set_attr,
        );
    }
    if !file_name.is_null() {
        let mut fdflags: libc::c_int = 0;
        if fd_reopen(
            0 as libc::c_int,
            device_name,
            0 as libc::c_int | 0o4000 as libc::c_int,
            0 as libc::c_int as mode_t,
        ) < 0 as libc::c_int
        {
            if ::core::mem::size_of::<C2RustUnnamed_13>() as libc::c_ulong != 0 {
                error(
                    1 as libc::c_int,
                    *__errno_location(),
                    b"%s\0" as *const u8 as *const libc::c_char,
                    quotearg_n_style_colon(
                        0 as libc::c_int,
                        shell_escape_quoting_style,
                        device_name,
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
                        device_name,
                    ),
                );
                if 0 as libc::c_int != 0 {} else {
                    unreachable!();
                };
            };
        }
        fdflags = rpl_fcntl(0 as libc::c_int, 3 as libc::c_int);
        if fdflags == -(1 as libc::c_int)
            || rpl_fcntl(
                0 as libc::c_int,
                4 as libc::c_int,
                fdflags & !(0o4000 as libc::c_int),
            ) < 0 as libc::c_int
        {
            if ::core::mem::size_of::<C2RustUnnamed_12>() as libc::c_ulong != 0 {
                error(
                    1 as libc::c_int,
                    *__errno_location(),
                    dcgettext(
                        0 as *const libc::c_char,
                        b"%s: couldn't reset non-blocking mode\0" as *const u8
                            as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    quotearg_n_style_colon(
                        0 as libc::c_int,
                        shell_escape_quoting_style,
                        device_name,
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
                        b"%s: couldn't reset non-blocking mode\0" as *const u8
                            as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    quotearg_n_style_colon(
                        0 as libc::c_int,
                        shell_escape_quoting_style,
                        device_name,
                    ),
                );
                if 0 as libc::c_int != 0 {} else {
                    unreachable!();
                };
            };
        }
    }
    if tcgetattr(0 as libc::c_int, &mut mode) != 0 {
        if ::core::mem::size_of::<C2RustUnnamed_11>() as libc::c_ulong != 0 {
            error(
                1 as libc::c_int,
                *__errno_location(),
                b"%s\0" as *const u8 as *const libc::c_char,
                quotearg_n_style_colon(
                    0 as libc::c_int,
                    shell_escape_quoting_style,
                    device_name,
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
                    device_name,
                ),
            );
            if 0 as libc::c_int != 0 {} else {
                unreachable!();
            };
        };
    }
    if verbose_output as libc::c_int != 0 || recoverable_output as libc::c_int != 0
        || noargs as libc::c_int != 0
    {
        max_col = screen_columns();
        current_col = 0 as libc::c_int;
        display_settings(output_type, &mut mode, device_name);
        return 0 as libc::c_int;
    }
    require_set_attr = 0 as libc::c_int != 0;
    apply_settings(
        0 as libc::c_int != 0,
        device_name,
        argv,
        argc,
        &mut mode,
        &mut require_set_attr,
    );
    if require_set_attr {
        let mut new_mode: termios = termios {
            c_iflag: 0,
            c_oflag: 0,
            c_cflag: 0,
            c_lflag: 0,
            c_line: 0,
            c_cc: [0; 32],
            c_ispeed: 0,
            c_ospeed: 0,
        };
        if tcsetattr(0 as libc::c_int, tcsetattr_options, &mut mode) != 0 {
            if ::core::mem::size_of::<C2RustUnnamed_10>() as libc::c_ulong != 0 {
                error(
                    1 as libc::c_int,
                    *__errno_location(),
                    b"%s\0" as *const u8 as *const libc::c_char,
                    quotearg_n_style_colon(
                        0 as libc::c_int,
                        shell_escape_quoting_style,
                        device_name,
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
                        device_name,
                    ),
                );
                if 0 as libc::c_int != 0 {} else {
                    unreachable!();
                };
            };
        }
        if tcgetattr(0 as libc::c_int, &mut new_mode) != 0 {
            if ::core::mem::size_of::<C2RustUnnamed_9>() as libc::c_ulong != 0 {
                error(
                    1 as libc::c_int,
                    *__errno_location(),
                    b"%s\0" as *const u8 as *const libc::c_char,
                    quotearg_n_style_colon(
                        0 as libc::c_int,
                        shell_escape_quoting_style,
                        device_name,
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
                        device_name,
                    ),
                );
                if 0 as libc::c_int != 0 {} else {
                    unreachable!();
                };
            };
        }
        if !eq_mode(&mut mode, &mut new_mode) {
            if dev_debug {
                error(
                    0 as libc::c_int,
                    0 as libc::c_int,
                    dcgettext(
                        0 as *const libc::c_char,
                        b"indx: mode: actual mode\0" as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                );
                let mut i: libc::c_uint = 0 as libc::c_int as libc::c_uint;
                while (i as libc::c_ulong)
                    < ::core::mem::size_of::<termios>() as libc::c_ulong
                {
                    let mut newc: libc::c_uint = *(&mut new_mode as *mut termios
                        as *mut libc::c_uchar)
                        .offset(i as isize) as libc::c_uint;
                    let mut oldc: libc::c_uint = *(&mut mode as *mut termios
                        as *mut libc::c_uchar)
                        .offset(i as isize) as libc::c_uint;
                    error(
                        0 as libc::c_int,
                        0 as libc::c_int,
                        b"0x%02x, 0x%02x: 0x%02x%s\0" as *const u8
                            as *const libc::c_char,
                        i,
                        oldc,
                        newc,
                        if newc == oldc {
                            b"\0" as *const u8 as *const libc::c_char
                        } else {
                            b" *\0" as *const u8 as *const libc::c_char
                        },
                    );
                    i = i.wrapping_add(1);
                }
            }
            if ::core::mem::size_of::<C2RustUnnamed_8>() as libc::c_ulong != 0 {
                error(
                    1 as libc::c_int,
                    0 as libc::c_int,
                    dcgettext(
                        0 as *const libc::c_char,
                        b"%s: unable to perform all requested operations\0" as *const u8
                            as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    quotearg_n_style_colon(
                        0 as libc::c_int,
                        shell_escape_quoting_style,
                        device_name,
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
                        b"%s: unable to perform all requested operations\0" as *const u8
                            as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    quotearg_n_style_colon(
                        0 as libc::c_int,
                        shell_escape_quoting_style,
                        device_name,
                    ),
                );
                if 0 as libc::c_int != 0 {} else {
                    unreachable!();
                };
            };
        }
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn eq_mode(mut mode1: *mut termios, mut mode2: *mut termios) -> bool {
    return (*mode1).c_iflag == (*mode2).c_iflag && (*mode1).c_oflag == (*mode2).c_oflag
        && (*mode1).c_cflag == (*mode2).c_cflag && (*mode1).c_lflag == (*mode2).c_lflag
        && (*mode1).c_line as libc::c_int == (*mode2).c_line as libc::c_int
        && memcmp(
            ((*mode1).c_cc).as_mut_ptr() as *const libc::c_void,
            ((*mode2).c_cc).as_mut_ptr() as *const libc::c_void,
            ::core::mem::size_of::<[cc_t; 32]>() as libc::c_ulong,
        ) == 0 as libc::c_int && cfgetispeed(mode1) == cfgetispeed(mode2)
        && cfgetospeed(mode1) == cfgetospeed(mode2);
}
unsafe extern "C" fn set_mode(
    mut info: *const mode_info,
    mut reversed: bool,
    mut mode: *mut termios,
) -> bool {
    let mut bitsp: *mut tcflag_t = 0 as *mut tcflag_t;
    if reversed as libc::c_int != 0
        && (*info).flags as libc::c_int & 4 as libc::c_int == 0 as libc::c_int
    {
        return 0 as libc::c_int != 0;
    }
    bitsp = mode_type_flag((*info).type_0, mode);
    if bitsp.is_null() {
        if strcmp((*info).name, b"evenp\0" as *const u8 as *const libc::c_char)
            == 0 as libc::c_int
            || strcmp((*info).name, b"parity\0" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
        {
            if reversed {
                (*mode)
                    .c_cflag = (*mode).c_cflag & !(0o400 as libc::c_int) as libc::c_uint
                    & !(0o60 as libc::c_int) as libc::c_uint
                    | 0o60 as libc::c_int as libc::c_uint;
            } else {
                (*mode)
                    .c_cflag = (*mode).c_cflag & !(0o1000 as libc::c_int) as libc::c_uint
                    & !(0o60 as libc::c_int) as libc::c_uint
                    | 0o400 as libc::c_int as libc::c_uint
                    | 0o40 as libc::c_int as libc::c_uint;
            }
        } else if strcmp((*info).name, b"oddp\0" as *const u8 as *const libc::c_char)
            == 0 as libc::c_int
        {
            if reversed {
                (*mode)
                    .c_cflag = (*mode).c_cflag & !(0o400 as libc::c_int) as libc::c_uint
                    & !(0o60 as libc::c_int) as libc::c_uint
                    | 0o60 as libc::c_int as libc::c_uint;
            } else {
                (*mode)
                    .c_cflag = (*mode).c_cflag & !(0o60 as libc::c_int) as libc::c_uint
                    | 0o40 as libc::c_int as libc::c_uint
                    | 0o1000 as libc::c_int as libc::c_uint
                    | 0o400 as libc::c_int as libc::c_uint;
            }
        } else if strcmp((*info).name, b"nl\0" as *const u8 as *const libc::c_char)
            == 0 as libc::c_int
        {
            if reversed {
                (*mode)
                    .c_iflag = ((*mode).c_iflag | 0o400 as libc::c_int as libc::c_uint)
                    & !(0o100 as libc::c_int) as libc::c_uint
                    & !(0o200 as libc::c_int) as libc::c_uint;
                (*mode)
                    .c_oflag = ((*mode).c_oflag | 0o4 as libc::c_int as libc::c_uint)
                    & !(0o10 as libc::c_int) as libc::c_uint
                    & !(0o40 as libc::c_int) as libc::c_uint;
            } else {
                (*mode)
                    .c_iflag = (*mode).c_iflag & !(0o400 as libc::c_int) as libc::c_uint;
                (*mode)
                    .c_oflag = (*mode).c_oflag & !(0o4 as libc::c_int) as libc::c_uint;
            }
        } else if strcmp((*info).name, b"ek\0" as *const u8 as *const libc::c_char)
            == 0 as libc::c_int
        {
            (*mode).c_cc[2 as libc::c_int as usize] = 0o177 as libc::c_int as cc_t;
            (*mode)
                .c_cc[3 as libc::c_int
                as usize] = ('u' as i32 & 0o37 as libc::c_int) as cc_t;
        } else if strcmp((*info).name, b"sane\0" as *const u8 as *const libc::c_char)
            == 0 as libc::c_int
        {
            sane_mode(mode);
        } else if strcmp((*info).name, b"cbreak\0" as *const u8 as *const libc::c_char)
            == 0 as libc::c_int
        {
            if reversed {
                (*mode).c_lflag |= 0o2 as libc::c_int as libc::c_uint;
            } else {
                (*mode).c_lflag &= !(0o2 as libc::c_int) as libc::c_uint;
            }
        } else if strcmp((*info).name, b"pass8\0" as *const u8 as *const libc::c_char)
            == 0 as libc::c_int
        {
            if reversed {
                (*mode)
                    .c_cflag = (*mode).c_cflag & !(0o60 as libc::c_int) as libc::c_uint
                    | 0o40 as libc::c_int as libc::c_uint
                    | 0o400 as libc::c_int as libc::c_uint;
                (*mode).c_iflag |= 0o40 as libc::c_int as libc::c_uint;
            } else {
                (*mode)
                    .c_cflag = (*mode).c_cflag & !(0o400 as libc::c_int) as libc::c_uint
                    & !(0o60 as libc::c_int) as libc::c_uint
                    | 0o60 as libc::c_int as libc::c_uint;
                (*mode).c_iflag &= !(0o40 as libc::c_int) as libc::c_uint;
            }
        } else if strcmp((*info).name, b"litout\0" as *const u8 as *const libc::c_char)
            == 0 as libc::c_int
        {
            if reversed {
                (*mode)
                    .c_cflag = (*mode).c_cflag & !(0o60 as libc::c_int) as libc::c_uint
                    | 0o40 as libc::c_int as libc::c_uint
                    | 0o400 as libc::c_int as libc::c_uint;
                (*mode).c_iflag |= 0o40 as libc::c_int as libc::c_uint;
                (*mode).c_oflag |= 0o1 as libc::c_int as libc::c_uint;
            } else {
                (*mode)
                    .c_cflag = (*mode).c_cflag & !(0o400 as libc::c_int) as libc::c_uint
                    & !(0o60 as libc::c_int) as libc::c_uint
                    | 0o60 as libc::c_int as libc::c_uint;
                (*mode).c_iflag &= !(0o40 as libc::c_int) as libc::c_uint;
                (*mode).c_oflag &= !(0o1 as libc::c_int) as libc::c_uint;
            }
        } else if strcmp((*info).name, b"raw\0" as *const u8 as *const libc::c_char)
            == 0 as libc::c_int
            || strcmp((*info).name, b"cooked\0" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
        {
            if *((*info).name).offset(0 as libc::c_int as isize) as libc::c_int
                == 'r' as i32 && reversed as libc::c_int != 0
                || *((*info).name).offset(0 as libc::c_int as isize) as libc::c_int
                    == 'c' as i32 && !reversed
            {
                (*mode).c_iflag
                    |= (0o2 as libc::c_int | 0o4 as libc::c_int | 0o40 as libc::c_int
                        | 0o400 as libc::c_int | 0o2000 as libc::c_int) as libc::c_uint;
                (*mode).c_oflag |= 0o1 as libc::c_int as libc::c_uint;
                (*mode).c_lflag
                    |= (0o1 as libc::c_int | 0o2 as libc::c_int) as libc::c_uint;
            } else {
                (*mode).c_iflag = 0 as libc::c_int as tcflag_t;
                (*mode).c_oflag &= !(0o1 as libc::c_int) as libc::c_uint;
                (*mode).c_lflag
                    &= !(0o1 as libc::c_int | 0o2 as libc::c_int | 0o4 as libc::c_int)
                        as libc::c_uint;
                (*mode).c_cc[6 as libc::c_int as usize] = 1 as libc::c_int as cc_t;
                (*mode).c_cc[5 as libc::c_int as usize] = 0 as libc::c_int as cc_t;
            }
        } else if strcmp((*info).name, b"decctlq\0" as *const u8 as *const libc::c_char)
            == 0 as libc::c_int
        {
            if reversed {
                (*mode).c_iflag |= 0o4000 as libc::c_int as libc::c_uint;
            } else {
                (*mode).c_iflag &= !(0o4000 as libc::c_int) as libc::c_uint;
            }
        } else if strcmp((*info).name, b"tabs\0" as *const u8 as *const libc::c_char)
            == 0 as libc::c_int
        {
            if reversed {
                (*mode)
                    .c_oflag = (*mode).c_oflag
                    & !(0o14000 as libc::c_int) as libc::c_uint
                    | 0o14000 as libc::c_int as libc::c_uint;
            } else {
                (*mode)
                    .c_oflag = (*mode).c_oflag
                    & !(0o14000 as libc::c_int) as libc::c_uint
                    | 0 as libc::c_int as libc::c_uint;
            }
        } else if strcmp((*info).name, b"lcase\0" as *const u8 as *const libc::c_char)
            == 0 as libc::c_int
            || strcmp((*info).name, b"LCASE\0" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
        {
            if reversed {
                (*mode).c_lflag &= !(0o4 as libc::c_int) as libc::c_uint;
                (*mode).c_iflag &= !(0o1000 as libc::c_int) as libc::c_uint;
                (*mode).c_oflag &= !(0o2 as libc::c_int) as libc::c_uint;
            } else {
                (*mode).c_lflag |= 0o4 as libc::c_int as libc::c_uint;
                (*mode).c_iflag |= 0o1000 as libc::c_int as libc::c_uint;
                (*mode).c_oflag |= 0o2 as libc::c_int as libc::c_uint;
            }
        } else if strcmp((*info).name, b"crt\0" as *const u8 as *const libc::c_char)
            == 0 as libc::c_int
        {
            (*mode).c_lflag
                |= (0o20 as libc::c_int | 0o1000 as libc::c_int | 0o4000 as libc::c_int)
                    as libc::c_uint;
        } else if strcmp((*info).name, b"dec\0" as *const u8 as *const libc::c_char)
            == 0 as libc::c_int
        {
            (*mode).c_cc[0 as libc::c_int as usize] = 3 as libc::c_int as cc_t;
            (*mode).c_cc[2 as libc::c_int as usize] = 127 as libc::c_int as cc_t;
            (*mode).c_cc[3 as libc::c_int as usize] = 21 as libc::c_int as cc_t;
            (*mode).c_lflag
                |= (0o20 as libc::c_int | 0o1000 as libc::c_int | 0o4000 as libc::c_int)
                    as libc::c_uint;
            (*mode).c_iflag &= !(0o4000 as libc::c_int) as libc::c_uint;
        }
    } else if reversed {
        *bitsp = (*bitsp as libc::c_ulong & !(*info).mask & !(*info).bits) as tcflag_t;
    } else {
        *bitsp = (*bitsp as libc::c_ulong & !(*info).mask | (*info).bits) as tcflag_t;
    }
    return 1 as libc::c_int != 0;
}
unsafe extern "C" fn set_control_char(
    mut info: *const control_info,
    mut arg: *const libc::c_char,
    mut mode: *mut termios,
) {
    let mut value: libc::c_ulong = 0;
    if strcmp((*info).name, b"min\0" as *const u8 as *const libc::c_char)
        == 0 as libc::c_int
        || strcmp((*info).name, b"time\0" as *const u8 as *const libc::c_char)
            == 0 as libc::c_int
    {
        value = integer_arg(
            arg,
            (if (0 as libc::c_int as cc_t as libc::c_int)
                < -(1 as libc::c_int) as cc_t as libc::c_int
            {
                -(1 as libc::c_int) as cc_t as libc::c_int
            } else {
                (((1 as libc::c_int as cc_t as libc::c_int)
                    << (::core::mem::size_of::<cc_t>() as libc::c_ulong)
                        .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                        .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                    - 1 as libc::c_int) * 2 as libc::c_int + 1 as libc::c_int
            }) as cc_t as libc::c_ulong,
        );
    } else if *arg.offset(0 as libc::c_int as isize) as libc::c_int == '\0' as i32
        || *arg.offset(1 as libc::c_int as isize) as libc::c_int == '\0' as i32
    {
        value = to_uchar(*arg.offset(0 as libc::c_int as isize)) as libc::c_ulong;
    } else if strcmp(arg, b"^-\0" as *const u8 as *const libc::c_char)
        == 0 as libc::c_int
        || strcmp(arg, b"undef\0" as *const u8 as *const libc::c_char)
            == 0 as libc::c_int
    {
        value = '\0' as i32 as libc::c_ulong;
    } else if *arg.offset(0 as libc::c_int as isize) as libc::c_int == '^' as i32
        && *arg.offset(1 as libc::c_int as isize) as libc::c_int != '\0' as i32
    {
        if *arg.offset(1 as libc::c_int as isize) as libc::c_int == '?' as i32 {
            value = 127 as libc::c_int as libc::c_ulong;
        } else {
            value = (to_uchar(*arg.offset(1 as libc::c_int as isize)) as libc::c_int
                & !(0o140 as libc::c_int)) as libc::c_ulong;
        }
    } else {
        value = integer_arg(
            arg,
            (if (0 as libc::c_int as cc_t as libc::c_int)
                < -(1 as libc::c_int) as cc_t as libc::c_int
            {
                -(1 as libc::c_int) as cc_t as libc::c_int
            } else {
                (((1 as libc::c_int as cc_t as libc::c_int)
                    << (::core::mem::size_of::<cc_t>() as libc::c_ulong)
                        .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                        .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                    - 1 as libc::c_int) * 2 as libc::c_int + 1 as libc::c_int
            }) as cc_t as libc::c_ulong,
        );
    }
    (*mode).c_cc[(*info).offset as usize] = value as cc_t;
}
unsafe extern "C" fn set_speed(
    mut type_0: speed_setting,
    mut arg: *const libc::c_char,
    mut mode: *mut termios,
) {
    let mut baud: speed_t = string_to_baud(arg);
    if baud != -(1 as libc::c_int) as speed_t {} else {
        __assert_fail(
            b"baud != (speed_t) -1\0" as *const u8 as *const libc::c_char,
            b"src/stty.c\0" as *const u8 as *const libc::c_char,
            1725 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 67],
                &[libc::c_char; 67],
            >(b"void set_speed(enum speed_setting, const char *, struct termios *)\0"))
                .as_ptr(),
        );
    }
    if type_0 as libc::c_uint == input_speed as libc::c_int as libc::c_uint
        || type_0 as libc::c_uint == both_speeds as libc::c_int as libc::c_uint
    {
        last_ibaud = baud;
        if cfsetispeed(mode, baud) != 0 {
            if ::core::mem::size_of::<C2RustUnnamed_4>() as libc::c_ulong != 0 {
                error(
                    1 as libc::c_int,
                    0 as libc::c_int,
                    b"unsupported ispeed %s\0" as *const u8 as *const libc::c_char,
                    quotearg_n_style_colon(
                        0 as libc::c_int,
                        shell_escape_quoting_style,
                        arg,
                    ),
                );
                if 0 as libc::c_int != 0 {} else {
                    unreachable!();
                };
            } else {
                error(
                    1 as libc::c_int,
                    0 as libc::c_int,
                    b"unsupported ispeed %s\0" as *const u8 as *const libc::c_char,
                    quotearg_n_style_colon(
                        0 as libc::c_int,
                        shell_escape_quoting_style,
                        arg,
                    ),
                );
                if 0 as libc::c_int != 0 {} else {
                    unreachable!();
                };
            };
        }
    }
    if type_0 as libc::c_uint == output_speed as libc::c_int as libc::c_uint
        || type_0 as libc::c_uint == both_speeds as libc::c_int as libc::c_uint
    {
        last_obaud = baud;
        if cfsetospeed(mode, baud) != 0 {
            if ::core::mem::size_of::<C2RustUnnamed_3>() as libc::c_ulong != 0 {
                error(
                    1 as libc::c_int,
                    0 as libc::c_int,
                    b"unsupported ospeed %s\0" as *const u8 as *const libc::c_char,
                    quotearg_n_style_colon(
                        0 as libc::c_int,
                        shell_escape_quoting_style,
                        arg,
                    ),
                );
                if 0 as libc::c_int != 0 {} else {
                    unreachable!();
                };
            } else {
                error(
                    1 as libc::c_int,
                    0 as libc::c_int,
                    b"unsupported ospeed %s\0" as *const u8 as *const libc::c_char,
                    quotearg_n_style_colon(
                        0 as libc::c_int,
                        shell_escape_quoting_style,
                        arg,
                    ),
                );
                if 0 as libc::c_int != 0 {} else {
                    unreachable!();
                };
            };
        }
    }
}
unsafe extern "C" fn get_win_size(
    mut fd: libc::c_int,
    mut win: *mut winsize,
) -> libc::c_int {
    let mut err: libc::c_int = ioctl(
        fd,
        0x5413 as libc::c_int as libc::c_ulong,
        win as *mut libc::c_char,
    );
    return err;
}
unsafe extern "C" fn set_window_size(
    mut rows: libc::c_int,
    mut cols: libc::c_int,
    mut device_name: *const libc::c_char,
) {
    let mut win: winsize = winsize {
        ws_row: 0,
        ws_col: 0,
        ws_xpixel: 0,
        ws_ypixel: 0,
    };
    if get_win_size(0 as libc::c_int, &mut win) != 0 {
        if *__errno_location() != 22 as libc::c_int {
            if ::core::mem::size_of::<C2RustUnnamed_6>() as libc::c_ulong != 0 {
                error(
                    1 as libc::c_int,
                    *__errno_location(),
                    b"%s\0" as *const u8 as *const libc::c_char,
                    quotearg_n_style_colon(
                        0 as libc::c_int,
                        shell_escape_quoting_style,
                        device_name,
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
                        device_name,
                    ),
                );
                if 0 as libc::c_int != 0 {} else {
                    unreachable!();
                };
            };
        }
        memset(
            &mut win as *mut winsize as *mut libc::c_void,
            0 as libc::c_int,
            ::core::mem::size_of::<winsize>() as libc::c_ulong,
        );
    }
    if rows >= 0 as libc::c_int {
        win.ws_row = rows as libc::c_ushort;
    }
    if cols >= 0 as libc::c_int {
        win.ws_col = cols as libc::c_ushort;
    }
    if ioctl(
        0 as libc::c_int,
        0x5414 as libc::c_int as libc::c_ulong,
        &mut win as *mut winsize as *mut libc::c_char,
    ) != 0
    {
        if ::core::mem::size_of::<C2RustUnnamed_5>() as libc::c_ulong != 0 {
            error(
                1 as libc::c_int,
                *__errno_location(),
                b"%s\0" as *const u8 as *const libc::c_char,
                quotearg_n_style_colon(
                    0 as libc::c_int,
                    shell_escape_quoting_style,
                    device_name,
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
                    device_name,
                ),
            );
            if 0 as libc::c_int != 0 {} else {
                unreachable!();
            };
        };
    }
}
unsafe extern "C" fn display_window_size(
    mut fancy: bool,
    mut device_name: *const libc::c_char,
) {
    let mut win: winsize = winsize {
        ws_row: 0,
        ws_col: 0,
        ws_xpixel: 0,
        ws_ypixel: 0,
    };
    if get_win_size(0 as libc::c_int, &mut win) != 0 {
        if *__errno_location() != 22 as libc::c_int {
            if ::core::mem::size_of::<C2RustUnnamed_1>() as libc::c_ulong != 0 {
                error(
                    1 as libc::c_int,
                    *__errno_location(),
                    b"%s\0" as *const u8 as *const libc::c_char,
                    quotearg_n_style_colon(
                        0 as libc::c_int,
                        shell_escape_quoting_style,
                        device_name,
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
                        device_name,
                    ),
                );
                if 0 as libc::c_int != 0 {} else {
                    unreachable!();
                };
            };
        }
        if !fancy {
            if ::core::mem::size_of::<C2RustUnnamed_0>() as libc::c_ulong != 0 {
                error(
                    1 as libc::c_int,
                    0 as libc::c_int,
                    dcgettext(
                        0 as *const libc::c_char,
                        b"%s: no size information for this device\0" as *const u8
                            as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    quotearg_n_style_colon(
                        0 as libc::c_int,
                        shell_escape_quoting_style,
                        device_name,
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
                        b"%s: no size information for this device\0" as *const u8
                            as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    quotearg_n_style_colon(
                        0 as libc::c_int,
                        shell_escape_quoting_style,
                        device_name,
                    ),
                );
                if 0 as libc::c_int != 0 {} else {
                    unreachable!();
                };
            };
        }
    } else {
        wrapf(
            if fancy as libc::c_int != 0 {
                b"rows %d; columns %d;\0" as *const u8 as *const libc::c_char
            } else {
                b"%d %d\n\0" as *const u8 as *const libc::c_char
            },
            win.ws_row as libc::c_int,
            win.ws_col as libc::c_int,
        );
        if !fancy {
            current_col = 0 as libc::c_int;
        }
    };
}
unsafe extern "C" fn screen_columns() -> libc::c_int {
    let mut win: winsize = winsize {
        ws_row: 0,
        ws_col: 0,
        ws_xpixel: 0,
        ws_ypixel: 0,
    };
    if get_win_size(1 as libc::c_int, &mut win) == 0 as libc::c_int
        && (0 as libc::c_int) < win.ws_col as libc::c_int
    {
        return win.ws_col as libc::c_int;
    }
    let mut col_string: *mut libc::c_char = getenv(
        b"COLUMNS\0" as *const u8 as *const libc::c_char,
    );
    let mut n_columns: libc::c_long = 0;
    if !(!col_string.is_null()
        && xstrtol(
            col_string,
            0 as *mut *mut libc::c_char,
            0 as libc::c_int,
            &mut n_columns,
            b"\0" as *const u8 as *const libc::c_char,
        ) as libc::c_uint == LONGINT_OK as libc::c_int as libc::c_uint
        && (0 as libc::c_int as libc::c_long) < n_columns
        && n_columns <= 2147483647 as libc::c_int as libc::c_long)
    {
        n_columns = 80 as libc::c_int as libc::c_long;
    }
    return n_columns as libc::c_int;
}
unsafe extern "C" fn mode_type_flag(
    mut type_0: mode_type,
    mut mode: *mut termios,
) -> *mut tcflag_t {
    match type_0 as libc::c_uint {
        0 => return &mut (*mode).c_cflag,
        1 => return &mut (*mode).c_iflag,
        2 => return &mut (*mode).c_oflag,
        3 => return &mut (*mode).c_lflag,
        4 => return 0 as *mut tcflag_t,
        _ => {
            abort();
        }
    };
}
unsafe extern "C" fn display_settings(
    mut output_type: output_type,
    mut mode: *mut termios,
    mut device_name: *const libc::c_char,
) {
    match output_type as libc::c_uint {
        0 => {
            display_changed(mode);
        }
        1 => {
            display_all(mode, device_name);
        }
        2 => {
            display_recoverable(mode);
        }
        _ => {}
    };
}
unsafe extern "C" fn display_changed(mut mode: *mut termios) {
    let mut i: libc::c_int = 0;
    let mut empty_line: bool = false;
    let mut bitsp: *mut tcflag_t = 0 as *mut tcflag_t;
    let mut mask: libc::c_ulong = 0;
    let mut prev_type: mode_type = control;
    display_speed(mode, 1 as libc::c_int != 0);
    wrapf(
        b"line = %d;\0" as *const u8 as *const libc::c_char,
        (*mode).c_line as libc::c_int,
    );
    putchar_unlocked('\n' as i32);
    current_col = 0 as libc::c_int;
    empty_line = 1 as libc::c_int != 0;
    i = 0 as libc::c_int;
    while !(strcmp(
        control_info[i as usize].name,
        b"min\0" as *const u8 as *const libc::c_char,
    ) == 0 as libc::c_int)
    {
        if !((*mode).c_cc[control_info[i as usize].offset as usize] as libc::c_int
            == control_info[i as usize].saneval as libc::c_int)
        {
            if !(strcmp(
                control_info[i as usize].name,
                b"flush\0" as *const u8 as *const libc::c_char,
            ) == 0 as libc::c_int)
            {
                empty_line = 0 as libc::c_int != 0;
                wrapf(
                    b"%s = %s;\0" as *const u8 as *const libc::c_char,
                    control_info[i as usize].name,
                    visible((*mode).c_cc[control_info[i as usize].offset as usize]),
                );
            }
        }
        i += 1;
    }
    if (*mode).c_lflag & 0o2 as libc::c_int as libc::c_uint
        == 0 as libc::c_int as libc::c_uint
    {
        wrapf(
            b"min = %lu; time = %lu;\n\0" as *const u8 as *const libc::c_char,
            (*mode).c_cc[6 as libc::c_int as usize] as libc::c_ulong,
            (*mode).c_cc[5 as libc::c_int as usize] as libc::c_ulong,
        );
    } else if !empty_line {
        putchar_unlocked('\n' as i32);
    }
    current_col = 0 as libc::c_int;
    empty_line = 1 as libc::c_int != 0;
    i = 0 as libc::c_int;
    while !(mode_info[i as usize].name).is_null() {
        if !(mode_info[i as usize].flags as libc::c_int & 8 as libc::c_int != 0) {
            if mode_info[i as usize].type_0 as libc::c_uint != prev_type as libc::c_uint
            {
                if !empty_line {
                    putchar_unlocked('\n' as i32);
                    current_col = 0 as libc::c_int;
                    empty_line = 1 as libc::c_int != 0;
                }
                prev_type = mode_info[i as usize].type_0;
            }
            bitsp = mode_type_flag(mode_info[i as usize].type_0, mode);
            mask = if mode_info[i as usize].mask != 0 {
                mode_info[i as usize].mask
            } else {
                mode_info[i as usize].bits
            };
            if !bitsp.is_null() {} else {
                __assert_fail(
                    b"bitsp\0" as *const u8 as *const libc::c_char,
                    b"src/stty.c\0" as *const u8 as *const libc::c_char,
                    1986 as libc::c_int as libc::c_uint,
                    (*::core::mem::transmute::<
                        &[u8; 39],
                        &[libc::c_char; 39],
                    >(b"void display_changed(struct termios *)\0"))
                        .as_ptr(),
                );
            }
            if *bitsp as libc::c_ulong & mask == mode_info[i as usize].bits {
                if mode_info[i as usize].flags as libc::c_int & 2 as libc::c_int != 0 {
                    wrapf(
                        b"%s\0" as *const u8 as *const libc::c_char,
                        mode_info[i as usize].name,
                    );
                    empty_line = 0 as libc::c_int != 0;
                }
            } else if mode_info[i as usize].flags as libc::c_int
                & (1 as libc::c_int | 4 as libc::c_int)
                == 1 as libc::c_int | 4 as libc::c_int
            {
                wrapf(
                    b"-%s\0" as *const u8 as *const libc::c_char,
                    mode_info[i as usize].name,
                );
                empty_line = 0 as libc::c_int != 0;
            }
        }
        i += 1;
    }
    if !empty_line {
        putchar_unlocked('\n' as i32);
    }
    current_col = 0 as libc::c_int;
}
unsafe extern "C" fn display_all(
    mut mode: *mut termios,
    mut device_name: *const libc::c_char,
) {
    let mut i: libc::c_int = 0;
    let mut bitsp: *mut tcflag_t = 0 as *mut tcflag_t;
    let mut mask: libc::c_ulong = 0;
    let mut prev_type: mode_type = control;
    display_speed(mode, 1 as libc::c_int != 0);
    display_window_size(1 as libc::c_int != 0, device_name);
    wrapf(
        b"line = %d;\0" as *const u8 as *const libc::c_char,
        (*mode).c_line as libc::c_int,
    );
    putchar_unlocked('\n' as i32);
    current_col = 0 as libc::c_int;
    i = 0 as libc::c_int;
    while !(strcmp(
        control_info[i as usize].name,
        b"min\0" as *const u8 as *const libc::c_char,
    ) == 0 as libc::c_int)
    {
        if !(strcmp(
            control_info[i as usize].name,
            b"flush\0" as *const u8 as *const libc::c_char,
        ) == 0 as libc::c_int)
        {
            wrapf(
                b"%s = %s;\0" as *const u8 as *const libc::c_char,
                control_info[i as usize].name,
                visible((*mode).c_cc[control_info[i as usize].offset as usize]),
            );
        }
        i += 1;
    }
    wrapf(
        b"min = %lu; time = %lu;\0" as *const u8 as *const libc::c_char,
        (*mode).c_cc[6 as libc::c_int as usize] as libc::c_ulong,
        (*mode).c_cc[5 as libc::c_int as usize] as libc::c_ulong,
    );
    if current_col != 0 as libc::c_int {
        putchar_unlocked('\n' as i32);
    }
    current_col = 0 as libc::c_int;
    i = 0 as libc::c_int;
    while !(mode_info[i as usize].name).is_null() {
        if !(mode_info[i as usize].flags as libc::c_int & 8 as libc::c_int != 0) {
            if mode_info[i as usize].type_0 as libc::c_uint != prev_type as libc::c_uint
            {
                putchar_unlocked('\n' as i32);
                current_col = 0 as libc::c_int;
                prev_type = mode_info[i as usize].type_0;
            }
            bitsp = mode_type_flag(mode_info[i as usize].type_0, mode);
            mask = if mode_info[i as usize].mask != 0 {
                mode_info[i as usize].mask
            } else {
                mode_info[i as usize].bits
            };
            if !bitsp.is_null() {} else {
                __assert_fail(
                    b"bitsp\0" as *const u8 as *const libc::c_char,
                    b"src/stty.c\0" as *const u8 as *const libc::c_char,
                    2070 as libc::c_int as libc::c_uint,
                    (*::core::mem::transmute::<
                        &[u8; 49],
                        &[libc::c_char; 49],
                    >(b"void display_all(struct termios *, const char *)\0"))
                        .as_ptr(),
                );
            }
            if *bitsp as libc::c_ulong & mask == mode_info[i as usize].bits {
                wrapf(
                    b"%s\0" as *const u8 as *const libc::c_char,
                    mode_info[i as usize].name,
                );
            } else if mode_info[i as usize].flags as libc::c_int & 4 as libc::c_int != 0
            {
                wrapf(
                    b"-%s\0" as *const u8 as *const libc::c_char,
                    mode_info[i as usize].name,
                );
            }
        }
        i += 1;
    }
    putchar_unlocked('\n' as i32);
    current_col = 0 as libc::c_int;
}
unsafe extern "C" fn check_speed(mut mode: *mut termios) {
    if last_ibaud != -(1 as libc::c_int) as libc::c_uint
        && last_obaud != -(1 as libc::c_int) as libc::c_uint
    {
        if cfgetispeed(mode) != last_ibaud || cfgetospeed(mode) != last_obaud {
            if ::core::mem::size_of::<C2RustUnnamed_2>() as libc::c_ulong != 0 {
                error(
                    1 as libc::c_int,
                    0 as libc::c_int,
                    dcgettext(
                        0 as *const libc::c_char,
                        b"asymmetric input (%lu), output (%lu) speeds not supported\0"
                            as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    baud_to_value(last_ibaud),
                    baud_to_value(last_obaud),
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
                        b"asymmetric input (%lu), output (%lu) speeds not supported\0"
                            as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    baud_to_value(last_ibaud),
                    baud_to_value(last_obaud),
                );
                if 0 as libc::c_int != 0 {} else {
                    unreachable!();
                };
            };
        }
    }
}
unsafe extern "C" fn display_speed(mut mode: *mut termios, mut fancy: bool) {
    if cfgetispeed(mode) == 0 as libc::c_int as libc::c_uint
        || cfgetispeed(mode) == cfgetospeed(mode)
    {
        wrapf(
            if fancy as libc::c_int != 0 {
                b"speed %lu baud;\0" as *const u8 as *const libc::c_char
            } else {
                b"%lu\n\0" as *const u8 as *const libc::c_char
            },
            baud_to_value(cfgetospeed(mode)),
        );
    } else {
        wrapf(
            if fancy as libc::c_int != 0 {
                b"ispeed %lu baud; ospeed %lu baud;\0" as *const u8
                    as *const libc::c_char
            } else {
                b"%lu %lu\n\0" as *const u8 as *const libc::c_char
            },
            baud_to_value(cfgetispeed(mode)),
            baud_to_value(cfgetospeed(mode)),
        );
    }
    if !fancy {
        current_col = 0 as libc::c_int;
    }
}
unsafe extern "C" fn display_recoverable(mut mode: *mut termios) {
    printf(
        b"%lx:%lx:%lx:%lx\0" as *const u8 as *const libc::c_char,
        (*mode).c_iflag as libc::c_ulong,
        (*mode).c_oflag as libc::c_ulong,
        (*mode).c_cflag as libc::c_ulong,
        (*mode).c_lflag as libc::c_ulong,
    );
    let mut i: size_t = 0 as libc::c_int as size_t;
    while i < 32 as libc::c_int as libc::c_ulong {
        printf(
            b":%lx\0" as *const u8 as *const libc::c_char,
            (*mode).c_cc[i as usize] as libc::c_ulong,
        );
        i = i.wrapping_add(1);
    }
    putchar_unlocked('\n' as i32);
}
unsafe extern "C" fn strtoul_tcflag_t(
    mut s: *const libc::c_char,
    mut base: libc::c_int,
    mut p: *mut *mut libc::c_char,
    mut result: *mut tcflag_t,
    mut delim: libc::c_char,
) -> libc::c_int {
    let mut ul: libc::c_ulong = 0;
    *__errno_location() = 0 as libc::c_int;
    ul = strtoul(s, p, base);
    if *__errno_location() != 0 || **p as libc::c_int != delim as libc::c_int
        || *p == s as *mut libc::c_char || ul as tcflag_t as libc::c_ulong != ul
    {
        return -(1 as libc::c_int);
    }
    *result = ul as tcflag_t;
    return 0 as libc::c_int;
}
unsafe extern "C" fn strtoul_cc_t(
    mut s: *const libc::c_char,
    mut base: libc::c_int,
    mut p: *mut *mut libc::c_char,
    mut result: *mut cc_t,
    mut delim: libc::c_char,
) -> libc::c_int {
    let mut ul: libc::c_ulong = 0;
    *__errno_location() = 0 as libc::c_int;
    ul = strtoul(s, p, base);
    if *__errno_location() != 0 || **p as libc::c_int != delim as libc::c_int
        || *p == s as *mut libc::c_char || ul as cc_t as libc::c_ulong != ul
    {
        return -(1 as libc::c_int);
    }
    *result = ul as cc_t;
    return 0 as libc::c_int;
}
unsafe extern "C" fn recover_mode(
    mut arg: *const libc::c_char,
    mut mode: *mut termios,
) -> bool {
    let mut flag: [tcflag_t; 4] = [0; 4];
    let mut s: *const libc::c_char = arg;
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < 4 as libc::c_int as libc::c_ulong {
        let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
        if strtoul_tcflag_t(
            s,
            16 as libc::c_int,
            &mut p,
            flag.as_mut_ptr().offset(i as isize),
            ':' as i32 as libc::c_char,
        ) != 0 as libc::c_int
        {
            return 0 as libc::c_int != 0;
        }
        s = p.offset(1 as libc::c_int as isize);
        i = i.wrapping_add(1);
    }
    (*mode).c_iflag = flag[0 as libc::c_int as usize];
    (*mode).c_oflag = flag[1 as libc::c_int as usize];
    (*mode).c_cflag = flag[2 as libc::c_int as usize];
    (*mode).c_lflag = flag[3 as libc::c_int as usize];
    i = 0 as libc::c_int as size_t;
    while i < 32 as libc::c_int as libc::c_ulong {
        let mut p_0: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut delim: libc::c_char = (if i
            < (32 as libc::c_int - 1 as libc::c_int) as libc::c_ulong
        {
            ':' as i32
        } else {
            '\0' as i32
        }) as libc::c_char;
        if strtoul_cc_t(
            s,
            16 as libc::c_int,
            &mut p_0,
            ((*mode).c_cc).as_mut_ptr().offset(i as isize),
            delim,
        ) != 0 as libc::c_int
        {
            return 0 as libc::c_int != 0;
        }
        s = p_0.offset(1 as libc::c_int as isize);
        i = i.wrapping_add(1);
    }
    return 1 as libc::c_int != 0;
}
static mut speeds: [speed_map; 35] = [
    {
        let mut init = speed_map {
            string: b"0\0" as *const u8 as *const libc::c_char,
            speed: 0 as libc::c_int as speed_t,
            value: 0 as libc::c_int as libc::c_ulong,
        };
        init
    },
    {
        let mut init = speed_map {
            string: b"50\0" as *const u8 as *const libc::c_char,
            speed: 0o1 as libc::c_int as speed_t,
            value: 50 as libc::c_int as libc::c_ulong,
        };
        init
    },
    {
        let mut init = speed_map {
            string: b"75\0" as *const u8 as *const libc::c_char,
            speed: 0o2 as libc::c_int as speed_t,
            value: 75 as libc::c_int as libc::c_ulong,
        };
        init
    },
    {
        let mut init = speed_map {
            string: b"110\0" as *const u8 as *const libc::c_char,
            speed: 0o3 as libc::c_int as speed_t,
            value: 110 as libc::c_int as libc::c_ulong,
        };
        init
    },
    {
        let mut init = speed_map {
            string: b"134\0" as *const u8 as *const libc::c_char,
            speed: 0o4 as libc::c_int as speed_t,
            value: 134 as libc::c_int as libc::c_ulong,
        };
        init
    },
    {
        let mut init = speed_map {
            string: b"134.5\0" as *const u8 as *const libc::c_char,
            speed: 0o4 as libc::c_int as speed_t,
            value: 134 as libc::c_int as libc::c_ulong,
        };
        init
    },
    {
        let mut init = speed_map {
            string: b"150\0" as *const u8 as *const libc::c_char,
            speed: 0o5 as libc::c_int as speed_t,
            value: 150 as libc::c_int as libc::c_ulong,
        };
        init
    },
    {
        let mut init = speed_map {
            string: b"200\0" as *const u8 as *const libc::c_char,
            speed: 0o6 as libc::c_int as speed_t,
            value: 200 as libc::c_int as libc::c_ulong,
        };
        init
    },
    {
        let mut init = speed_map {
            string: b"300\0" as *const u8 as *const libc::c_char,
            speed: 0o7 as libc::c_int as speed_t,
            value: 300 as libc::c_int as libc::c_ulong,
        };
        init
    },
    {
        let mut init = speed_map {
            string: b"600\0" as *const u8 as *const libc::c_char,
            speed: 0o10 as libc::c_int as speed_t,
            value: 600 as libc::c_int as libc::c_ulong,
        };
        init
    },
    {
        let mut init = speed_map {
            string: b"1200\0" as *const u8 as *const libc::c_char,
            speed: 0o11 as libc::c_int as speed_t,
            value: 1200 as libc::c_int as libc::c_ulong,
        };
        init
    },
    {
        let mut init = speed_map {
            string: b"1800\0" as *const u8 as *const libc::c_char,
            speed: 0o12 as libc::c_int as speed_t,
            value: 1800 as libc::c_int as libc::c_ulong,
        };
        init
    },
    {
        let mut init = speed_map {
            string: b"2400\0" as *const u8 as *const libc::c_char,
            speed: 0o13 as libc::c_int as speed_t,
            value: 2400 as libc::c_int as libc::c_ulong,
        };
        init
    },
    {
        let mut init = speed_map {
            string: b"4800\0" as *const u8 as *const libc::c_char,
            speed: 0o14 as libc::c_int as speed_t,
            value: 4800 as libc::c_int as libc::c_ulong,
        };
        init
    },
    {
        let mut init = speed_map {
            string: b"9600\0" as *const u8 as *const libc::c_char,
            speed: 0o15 as libc::c_int as speed_t,
            value: 9600 as libc::c_int as libc::c_ulong,
        };
        init
    },
    {
        let mut init = speed_map {
            string: b"19200\0" as *const u8 as *const libc::c_char,
            speed: 0o16 as libc::c_int as speed_t,
            value: 19200 as libc::c_int as libc::c_ulong,
        };
        init
    },
    {
        let mut init = speed_map {
            string: b"38400\0" as *const u8 as *const libc::c_char,
            speed: 0o17 as libc::c_int as speed_t,
            value: 38400 as libc::c_int as libc::c_ulong,
        };
        init
    },
    {
        let mut init = speed_map {
            string: b"exta\0" as *const u8 as *const libc::c_char,
            speed: 0o16 as libc::c_int as speed_t,
            value: 19200 as libc::c_int as libc::c_ulong,
        };
        init
    },
    {
        let mut init = speed_map {
            string: b"extb\0" as *const u8 as *const libc::c_char,
            speed: 0o17 as libc::c_int as speed_t,
            value: 38400 as libc::c_int as libc::c_ulong,
        };
        init
    },
    {
        let mut init = speed_map {
            string: b"57600\0" as *const u8 as *const libc::c_char,
            speed: 0o10001 as libc::c_int as speed_t,
            value: 57600 as libc::c_int as libc::c_ulong,
        };
        init
    },
    {
        let mut init = speed_map {
            string: b"115200\0" as *const u8 as *const libc::c_char,
            speed: 0o10002 as libc::c_int as speed_t,
            value: 115200 as libc::c_int as libc::c_ulong,
        };
        init
    },
    {
        let mut init = speed_map {
            string: b"230400\0" as *const u8 as *const libc::c_char,
            speed: 0o10003 as libc::c_int as speed_t,
            value: 230400 as libc::c_int as libc::c_ulong,
        };
        init
    },
    {
        let mut init = speed_map {
            string: b"460800\0" as *const u8 as *const libc::c_char,
            speed: 0o10004 as libc::c_int as speed_t,
            value: 460800 as libc::c_int as libc::c_ulong,
        };
        init
    },
    {
        let mut init = speed_map {
            string: b"500000\0" as *const u8 as *const libc::c_char,
            speed: 0o10005 as libc::c_int as speed_t,
            value: 500000 as libc::c_int as libc::c_ulong,
        };
        init
    },
    {
        let mut init = speed_map {
            string: b"576000\0" as *const u8 as *const libc::c_char,
            speed: 0o10006 as libc::c_int as speed_t,
            value: 576000 as libc::c_int as libc::c_ulong,
        };
        init
    },
    {
        let mut init = speed_map {
            string: b"921600\0" as *const u8 as *const libc::c_char,
            speed: 0o10007 as libc::c_int as speed_t,
            value: 921600 as libc::c_int as libc::c_ulong,
        };
        init
    },
    {
        let mut init = speed_map {
            string: b"1000000\0" as *const u8 as *const libc::c_char,
            speed: 0o10010 as libc::c_int as speed_t,
            value: 1000000 as libc::c_int as libc::c_ulong,
        };
        init
    },
    {
        let mut init = speed_map {
            string: b"1152000\0" as *const u8 as *const libc::c_char,
            speed: 0o10011 as libc::c_int as speed_t,
            value: 1152000 as libc::c_int as libc::c_ulong,
        };
        init
    },
    {
        let mut init = speed_map {
            string: b"1500000\0" as *const u8 as *const libc::c_char,
            speed: 0o10012 as libc::c_int as speed_t,
            value: 1500000 as libc::c_int as libc::c_ulong,
        };
        init
    },
    {
        let mut init = speed_map {
            string: b"2000000\0" as *const u8 as *const libc::c_char,
            speed: 0o10013 as libc::c_int as speed_t,
            value: 2000000 as libc::c_int as libc::c_ulong,
        };
        init
    },
    {
        let mut init = speed_map {
            string: b"2500000\0" as *const u8 as *const libc::c_char,
            speed: 0o10014 as libc::c_int as speed_t,
            value: 2500000 as libc::c_int as libc::c_ulong,
        };
        init
    },
    {
        let mut init = speed_map {
            string: b"3000000\0" as *const u8 as *const libc::c_char,
            speed: 0o10015 as libc::c_int as speed_t,
            value: 3000000 as libc::c_int as libc::c_ulong,
        };
        init
    },
    {
        let mut init = speed_map {
            string: b"3500000\0" as *const u8 as *const libc::c_char,
            speed: 0o10016 as libc::c_int as speed_t,
            value: 3500000 as libc::c_int as libc::c_ulong,
        };
        init
    },
    {
        let mut init = speed_map {
            string: b"4000000\0" as *const u8 as *const libc::c_char,
            speed: 0o10017 as libc::c_int as speed_t,
            value: 4000000 as libc::c_int as libc::c_ulong,
        };
        init
    },
    {
        let mut init = speed_map {
            string: 0 as *const libc::c_char,
            speed: 0 as libc::c_int as speed_t,
            value: 0 as libc::c_int as libc::c_ulong,
        };
        init
    },
];
unsafe extern "C" fn string_to_baud(mut arg: *const libc::c_char) -> speed_t {
    let mut i: libc::c_int = 0 as libc::c_int;
    while !(speeds[i as usize].string).is_null() {
        if strcmp(arg, speeds[i as usize].string) == 0 as libc::c_int {
            return speeds[i as usize].speed;
        }
        i += 1;
    }
    return -(1 as libc::c_int) as speed_t;
}
unsafe extern "C" fn baud_to_value(mut speed: speed_t) -> libc::c_ulong {
    let mut i: libc::c_int = 0 as libc::c_int;
    while !(speeds[i as usize].string).is_null() {
        if speed == speeds[i as usize].speed {
            return speeds[i as usize].value;
        }
        i += 1;
    }
    return 0 as libc::c_int as libc::c_ulong;
}
unsafe extern "C" fn sane_mode(mut mode: *mut termios) {
    let mut i: libc::c_int = 0;
    let mut bitsp: *mut tcflag_t = 0 as *mut tcflag_t;
    i = 0 as libc::c_int;
    while !(control_info[i as usize].name).is_null() {
        (*mode)
            .c_cc[control_info[i as usize].offset
            as usize] = control_info[i as usize].saneval;
        i += 1;
    }
    i = 0 as libc::c_int;
    while !(mode_info[i as usize].name).is_null() {
        if !(mode_info[i as usize].flags as libc::c_int & 16 as libc::c_int != 0) {
            if mode_info[i as usize].flags as libc::c_int & 1 as libc::c_int != 0 {
                bitsp = mode_type_flag(mode_info[i as usize].type_0, mode);
                if !bitsp.is_null() {} else {
                    __assert_fail(
                        b"bitsp\0" as *const u8 as *const libc::c_char,
                        b"src/stty.c\0" as *const u8 as *const libc::c_char,
                        2302 as libc::c_int as libc::c_uint,
                        (*::core::mem::transmute::<
                            &[u8; 33],
                            &[libc::c_char; 33],
                        >(b"void sane_mode(struct termios *)\0"))
                            .as_ptr(),
                    );
                }
                *bitsp = (*bitsp as libc::c_ulong & !mode_info[i as usize].mask
                    | mode_info[i as usize].bits) as tcflag_t;
            } else if mode_info[i as usize].flags as libc::c_int & 2 as libc::c_int != 0
            {
                bitsp = mode_type_flag(mode_info[i as usize].type_0, mode);
                if !bitsp.is_null() {} else {
                    __assert_fail(
                        b"bitsp\0" as *const u8 as *const libc::c_char,
                        b"src/stty.c\0" as *const u8 as *const libc::c_char,
                        2308 as libc::c_int as libc::c_uint,
                        (*::core::mem::transmute::<
                            &[u8; 33],
                            &[libc::c_char; 33],
                        >(b"void sane_mode(struct termios *)\0"))
                            .as_ptr(),
                    );
                }
                *bitsp = (*bitsp as libc::c_ulong & !mode_info[i as usize].mask
                    & !mode_info[i as usize].bits) as tcflag_t;
            }
        }
        i += 1;
    }
}
unsafe extern "C" fn visible(mut ch: cc_t) -> *const libc::c_char {
    static mut buf: [libc::c_char; 10] = [0; 10];
    let mut bpout: *mut libc::c_char = buf.as_mut_ptr();
    if ch as libc::c_int == '\0' as i32 {
        return b"<undef>\0" as *const u8 as *const libc::c_char;
    }
    if ch as libc::c_int >= 32 as libc::c_int {
        if (ch as libc::c_int) < 127 as libc::c_int {
            let fresh3 = bpout;
            bpout = bpout.offset(1);
            *fresh3 = ch as libc::c_char;
        } else if ch as libc::c_int == 127 as libc::c_int {
            let fresh4 = bpout;
            bpout = bpout.offset(1);
            *fresh4 = '^' as i32 as libc::c_char;
            let fresh5 = bpout;
            bpout = bpout.offset(1);
            *fresh5 = '?' as i32 as libc::c_char;
        } else {
            let fresh6 = bpout;
            bpout = bpout.offset(1);
            *fresh6 = 'M' as i32 as libc::c_char;
            let fresh7 = bpout;
            bpout = bpout.offset(1);
            *fresh7 = '-' as i32 as libc::c_char;
            if ch as libc::c_int >= 128 as libc::c_int + 32 as libc::c_int {
                if (ch as libc::c_int) < 128 as libc::c_int + 127 as libc::c_int {
                    let fresh8 = bpout;
                    bpout = bpout.offset(1);
                    *fresh8 = (ch as libc::c_int - 128 as libc::c_int) as libc::c_char;
                } else {
                    let fresh9 = bpout;
                    bpout = bpout.offset(1);
                    *fresh9 = '^' as i32 as libc::c_char;
                    let fresh10 = bpout;
                    bpout = bpout.offset(1);
                    *fresh10 = '?' as i32 as libc::c_char;
                }
            } else {
                let fresh11 = bpout;
                bpout = bpout.offset(1);
                *fresh11 = '^' as i32 as libc::c_char;
                let fresh12 = bpout;
                bpout = bpout.offset(1);
                *fresh12 = (ch as libc::c_int - 128 as libc::c_int + 64 as libc::c_int)
                    as libc::c_char;
            }
        }
    } else {
        let fresh13 = bpout;
        bpout = bpout.offset(1);
        *fresh13 = '^' as i32 as libc::c_char;
        let fresh14 = bpout;
        bpout = bpout.offset(1);
        *fresh14 = (ch as libc::c_int + 64 as libc::c_int) as libc::c_char;
    }
    *bpout = '\0' as i32 as libc::c_char;
    return buf.as_mut_ptr() as *const libc::c_char;
}
unsafe extern "C" fn integer_arg(
    mut s: *const libc::c_char,
    mut maxval: libc::c_ulong,
) -> libc::c_ulong {
    return xnumtoumax(
        s,
        0 as libc::c_int,
        0 as libc::c_int as uintmax_t,
        maxval,
        b"bB\0" as *const u8 as *const libc::c_char,
        dcgettext(
            0 as *const libc::c_char,
            b"invalid integer argument\0" as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        ),
        0 as libc::c_int,
    );
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
