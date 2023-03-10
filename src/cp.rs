#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![feature(extern_types)]
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type selinux_opt;
    pub type selabel_handle;
    pub type hash_table;
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
    fn __errno_location() -> *mut libc::c_int;
    fn fchownat(
        __fd: libc::c_int,
        __file: *const libc::c_char,
        __owner: __uid_t,
        __group: __gid_t,
        __flag: libc::c_int,
    ) -> libc::c_int;
    fn utimensat(
        __fd: libc::c_int,
        __path: *const libc::c_char,
        __times: *const timespec,
        __flags: libc::c_int,
    ) -> libc::c_int;
    fn mkdirat(
        __fd: libc::c_int,
        __path: *const libc::c_char,
        __mode: __mode_t,
    ) -> libc::c_int;
    fn fchmodat(
        __fd: libc::c_int,
        __file: *const libc::c_char,
        __mode: __mode_t,
        __flag: libc::c_int,
    ) -> libc::c_int;
    fn fstatat(
        __fd: libc::c_int,
        __file: *const libc::c_char,
        __buf: *mut stat,
        __flag: libc::c_int,
    ) -> libc::c_int;
    fn stat(__file: *const libc::c_char, __buf: *mut stat) -> libc::c_int;
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
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn free(_: *mut libc::c_void);
    fn abort() -> !;
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
    fn xmalloc(s: size_t) -> *mut libc::c_void;
    fn xstrdup(str: *const libc::c_char) -> *mut libc::c_char;
    fn last_component(filename: *const libc::c_char) -> *mut libc::c_char;
    fn dir_len(file: *const libc::c_char) -> size_t;
    fn strip_trailing_slashes(file: *mut libc::c_char) -> bool;
    fn close_stdin();
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
    fn quote(arg: *const libc::c_char) -> *const libc::c_char;
    fn set_simple_backup_suffix(_: *const libc::c_char);
    fn find_backup_file_name(
        _: libc::c_int,
        _: *const libc::c_char,
        _: backup_type,
    ) -> *mut libc::c_char;
    fn xget_version(
        context: *const libc::c_char,
        arg: *const libc::c_char,
    ) -> backup_type;
    fn cp_options_default(_: *mut cp_options);
    fn copy(
        src_name: *const libc::c_char,
        dst_name: *const libc::c_char,
        dst_dirfd: libc::c_int,
        dst_relname: *const libc::c_char,
        nonexistent_dst: libc::c_int,
        options: *const cp_options,
        copy_into_self: *mut bool,
        rename_succeeded: *mut bool,
    ) -> bool;
    fn set_process_security_ctx(
        src_name: *const libc::c_char,
        dst_name: *const libc::c_char,
        mode: mode_t,
        new_dst: bool,
        x: *const cp_options,
    ) -> bool;
    fn set_file_security_ctx(
        dst_name: *const libc::c_char,
        recurse: bool,
        x: *const cp_options,
    ) -> bool;
    fn dest_info_init(_: *mut cp_options);
    fn src_info_init(_: *mut cp_options);
    fn chown_failure_ok(_: *const cp_options) -> bool;
    fn cached_umask() -> mode_t;
    fn hash_init();
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
    fn target_directory_operand(file: *const libc::c_char, st: *mut stat) -> libc::c_int;
    fn copy_acl(
        _: *const libc::c_char,
        _: libc::c_int,
        _: *const libc::c_char,
        _: libc::c_int,
        _: mode_t,
    ) -> libc::c_int;
}
pub type size_t = libc::c_ulong;
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
pub type gid_t = __gid_t;
pub type mode_t = __mode_t;
pub type uid_t = __uid_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
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
pub type C2RustUnnamed = libc::c_uint;
pub const O_PATHSEARCH: C2RustUnnamed = 2097152;
pub type idx_t = ptrdiff_t;
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
pub type argmatch_exit_fn = Option::<unsafe extern "C" fn() -> ()>;
pub type backup_type = libc::c_uint;
pub const numbered_backups: backup_type = 3;
pub const numbered_existing_backups: backup_type = 2;
pub const simple_backups: backup_type = 1;
pub const no_backups: backup_type = 0;
pub type Hash_table = hash_table;
pub type Sparse_type = libc::c_uint;
pub const SPARSE_ALWAYS: Sparse_type = 3;
pub const SPARSE_AUTO: Sparse_type = 2;
pub const SPARSE_NEVER: Sparse_type = 1;
pub const SPARSE_UNUSED: Sparse_type = 0;
pub type Reflink_type = libc::c_uint;
pub const REFLINK_ALWAYS: Reflink_type = 2;
pub const REFLINK_AUTO: Reflink_type = 1;
pub const REFLINK_NEVER: Reflink_type = 0;
pub type Interactive = libc::c_uint;
pub const I_UNSPECIFIED: Interactive = 4;
pub const I_ASK_USER: Interactive = 3;
pub const I_ALWAYS_NO: Interactive = 2;
pub const I_ALWAYS_YES: Interactive = 1;
pub type Dereference_symlink = libc::c_uint;
pub const DEREF_ALWAYS: Dereference_symlink = 4;
pub const DEREF_COMMAND_LINE_ARGUMENTS: Dereference_symlink = 3;
pub const DEREF_NEVER: Dereference_symlink = 2;
pub const DEREF_UNDEFINED: Dereference_symlink = 1;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cp_options {
    pub backup_type: backup_type,
    pub dereference: Dereference_symlink,
    pub interactive: Interactive,
    pub sparse_mode: Sparse_type,
    pub mode: mode_t,
    pub copy_as_regular: bool,
    pub unlink_dest_before_opening: bool,
    pub unlink_dest_after_failed_open: bool,
    pub hard_link: bool,
    pub move_mode: bool,
    pub no_copy: bool,
    pub install_mode: bool,
    pub chown_privileges: bool,
    pub owner_privileges: bool,
    pub one_file_system: bool,
    pub preserve_ownership: bool,
    pub preserve_mode: bool,
    pub preserve_timestamps: bool,
    pub explicit_no_preserve_mode: bool,
    pub set_security_context: *mut selabel_handle,
    pub preserve_links: bool,
    pub data_copy_required: bool,
    pub require_preserve: bool,
    pub preserve_security_context: bool,
    pub require_preserve_context: bool,
    pub preserve_xattr: bool,
    pub require_preserve_xattr: bool,
    pub reduce_diagnostics: bool,
    pub recursive: bool,
    pub set_mode: bool,
    pub symbolic_link: bool,
    pub update: bool,
    pub verbose: bool,
    pub debug: bool,
    pub stdin_tty: bool,
    pub open_dangling_dest_symlink: bool,
    pub last_file: bool,
    pub rename_errno: libc::c_int,
    pub reflink_mode: Reflink_type,
    pub dest_info: *mut Hash_table,
    pub src_info: *mut Hash_table,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dir_attr {
    pub st: stat,
    pub restore_mode: bool,
    pub slash_offset: size_t,
    pub next: *mut dir_attr,
}
pub type C2RustUnnamed_1 = libc::c_uint;
pub const UNLINK_DEST_BEFORE_OPENING: C2RustUnnamed_1 = 265;
pub const STRIP_TRAILING_SLASHES_OPTION: C2RustUnnamed_1 = 264;
pub const SPARSE_OPTION: C2RustUnnamed_1 = 263;
pub const REFLINK_OPTION: C2RustUnnamed_1 = 262;
pub const PRESERVE_ATTRIBUTES_OPTION: C2RustUnnamed_1 = 261;
pub const PARENTS_OPTION: C2RustUnnamed_1 = 260;
pub const NO_PRESERVE_ATTRIBUTES_OPTION: C2RustUnnamed_1 = 259;
pub const DEBUG_OPTION: C2RustUnnamed_1 = 258;
pub const COPY_CONTENTS_OPTION: C2RustUnnamed_1 = 257;
pub const ATTRIBUTES_ONLY_OPTION: C2RustUnnamed_1 = 256;
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
pub const PRESERVE_ALL: File_attribute = 6;
pub const PRESERVE_XATTR: File_attribute = 5;
pub const PRESERVE_CONTEXT: File_attribute = 4;
pub const PRESERVE_LINK: File_attribute = 3;
pub const PRESERVE_OWNERSHIP: File_attribute = 2;
pub const PRESERVE_TIMESTAMPS: File_attribute = 1;
pub const PRESERVE_MODE: File_attribute = 0;
pub type File_attribute = libc::c_uint;
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
#[inline]
unsafe extern "C" fn setfscreatecon(mut con: *const libc::c_char) -> libc::c_int {
    *__errno_location() = 95 as libc::c_int;
    return -(1 as libc::c_int);
}
#[inline]
unsafe extern "C" fn selabel_open(
    mut backend: libc::c_int,
    mut options: *mut selinux_opt,
    mut nopt: libc::c_uint,
) -> *mut selabel_handle {
    *__errno_location() = 95 as libc::c_int;
    return 0 as *mut selabel_handle;
}
#[inline]
unsafe extern "C" fn lchownat(
    mut fd: libc::c_int,
    mut file: *const libc::c_char,
    mut owner: uid_t,
    mut group: gid_t,
) -> libc::c_int {
    return fchownat(fd, file, owner, group, 0x100 as libc::c_int);
}
#[inline]
unsafe extern "C" fn lchmodat(
    mut fd: libc::c_int,
    mut file: *const libc::c_char,
    mut mode: mode_t,
) -> libc::c_int {
    return fchmodat(fd, file, mode, 0x100 as libc::c_int);
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
unsafe extern "C" fn emit_backup_suffix_note() {
    fputs_unlocked(
        dcgettext(
            0 as *const libc::c_char,
            b"\nThe backup suffix is '~', unless set with --suffix or SIMPLE_BACKUP_SUFFIX.\nThe version control method may be selected via the --backup option or through\nthe VERSION_CONTROL environment variable.  Here are the values:\n\n\0"
                as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        ),
        stdout,
    );
    fputs_unlocked(
        dcgettext(
            0 as *const libc::c_char,
            b"  none, off       never make backups (even if --backup is given)\n  numbered, t     make numbered backups\n  existing, nil   numbered if numbered backups exist, simple otherwise\n  simple, never   always make simple backups\n\0"
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
unsafe extern "C" fn get_stat_atime(mut st: *const stat) -> timespec {
    return (*st).st_atim;
}
#[inline]
unsafe extern "C" fn get_stat_mtime(mut st: *const stat) -> timespec {
    return (*st).st_mtim;
}
#[inline]
unsafe extern "C" fn target_dirfd_valid(mut fd: libc::c_int) -> bool {
    return fd
        != -(1 as libc::c_int)
            - (-(100 as libc::c_int) == -(1 as libc::c_int)) as libc::c_int;
}
static mut selinux_enabled: bool = false;
static mut parents_option: bool = 0 as libc::c_int != 0;
static mut remove_trailing_slashes: bool = false;
static mut sparse_type_string: [*const libc::c_char; 4] = [
    b"never\0" as *const u8 as *const libc::c_char,
    b"auto\0" as *const u8 as *const libc::c_char,
    b"always\0" as *const u8 as *const libc::c_char,
    0 as *const libc::c_char,
];
static mut sparse_type: [Sparse_type; 3] = [SPARSE_NEVER, SPARSE_AUTO, SPARSE_ALWAYS];
static mut reflink_type_string: [*const libc::c_char; 4] = [
    b"auto\0" as *const u8 as *const libc::c_char,
    b"always\0" as *const u8 as *const libc::c_char,
    b"never\0" as *const u8 as *const libc::c_char,
    0 as *const libc::c_char,
];
static mut reflink_type: [Reflink_type; 3] = [
    REFLINK_AUTO,
    REFLINK_ALWAYS,
    REFLINK_NEVER,
];
static mut long_opts: [option; 31] = [
    {
        let mut init = option {
            name: b"archive\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'a' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"attributes-only\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: ATTRIBUTES_ONLY_OPTION as libc::c_int,
        };
        init
    },
    {
        let mut init = option {
            name: b"backup\0" as *const u8 as *const libc::c_char,
            has_arg: 2 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'b' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"copy-contents\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: COPY_CONTENTS_OPTION as libc::c_int,
        };
        init
    },
    {
        let mut init = option {
            name: b"debug\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: DEBUG_OPTION as libc::c_int,
        };
        init
    },
    {
        let mut init = option {
            name: b"dereference\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'L' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"force\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'f' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"interactive\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'i' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"link\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'l' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"no-clobber\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'n' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"no-dereference\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'P' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"no-preserve\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: NO_PRESERVE_ATTRIBUTES_OPTION as libc::c_int,
        };
        init
    },
    {
        let mut init = option {
            name: b"no-target-directory\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'T' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"one-file-system\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'x' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"parents\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: PARENTS_OPTION as libc::c_int,
        };
        init
    },
    {
        let mut init = option {
            name: b"path\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: PARENTS_OPTION as libc::c_int,
        };
        init
    },
    {
        let mut init = option {
            name: b"preserve\0" as *const u8 as *const libc::c_char,
            has_arg: 2 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: PRESERVE_ATTRIBUTES_OPTION as libc::c_int,
        };
        init
    },
    {
        let mut init = option {
            name: b"recursive\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'R' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"remove-destination\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: UNLINK_DEST_BEFORE_OPENING as libc::c_int,
        };
        init
    },
    {
        let mut init = option {
            name: b"sparse\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: SPARSE_OPTION as libc::c_int,
        };
        init
    },
    {
        let mut init = option {
            name: b"reflink\0" as *const u8 as *const libc::c_char,
            has_arg: 2 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: REFLINK_OPTION as libc::c_int,
        };
        init
    },
    {
        let mut init = option {
            name: b"strip-trailing-slashes\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: STRIP_TRAILING_SLASHES_OPTION as libc::c_int,
        };
        init
    },
    {
        let mut init = option {
            name: b"suffix\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'S' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"symbolic-link\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 's' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"target-directory\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 't' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"update\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'u' as i32,
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
            name: b"context\0" as *const u8 as *const libc::c_char,
            has_arg: 2 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'Z' as i32,
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
                b"Usage: %s [OPTION]... [-T] SOURCE DEST\n  or:  %s [OPTION]... SOURCE... DIRECTORY\n  or:  %s [OPTION]... -t DIRECTORY SOURCE...\n\0"
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
                b"Copy SOURCE to DEST, or multiple SOURCE(s) to DIRECTORY.\n\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            stdout,
        );
        emit_mandatory_arg_note();
        fputs_unlocked(
            dcgettext(
                0 as *const libc::c_char,
                b"  -a, --archive                same as -dR --preserve=all\n      --attributes-only        don't copy the file data, just the attributes\n      --backup[=CONTROL]       make a backup of each existing destination file\n  -b                           like --backup but does not accept an argument\n      --copy-contents          copy contents of special files when recursive\n  -d                           same as --no-dereference --preserve=links\n\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            stdout,
        );
        fputs_unlocked(
            dcgettext(
                0 as *const libc::c_char,
                b"      --debug                  explain how a file is copied.  Implies -v\n\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            stdout,
        );
        fputs_unlocked(
            dcgettext(
                0 as *const libc::c_char,
                b"  -f, --force                  if an existing destination file cannot be\n                                 opened, remove it and try again (this option\n                                 is ignored when the -n option is also used)\n  -i, --interactive            prompt before overwrite (overrides a previous -n\n                                  option)\n  -H                           follow command-line symbolic links in SOURCE\n\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            stdout,
        );
        fputs_unlocked(
            dcgettext(
                0 as *const libc::c_char,
                b"  -l, --link                   hard link files instead of copying\n  -L, --dereference            always follow symbolic links in SOURCE\n\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            stdout,
        );
        fputs_unlocked(
            dcgettext(
                0 as *const libc::c_char,
                b"  -n, --no-clobber             do not overwrite an existing file (overrides\n                                 a previous -i option)\n  -P, --no-dereference         never follow symbolic links in SOURCE\n\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            stdout,
        );
        fputs_unlocked(
            dcgettext(
                0 as *const libc::c_char,
                b"  -p                           same as --preserve=mode,ownership,timestamps\n      --preserve[=ATTR_LIST]   preserve the specified attributes\n\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            stdout,
        );
        fputs_unlocked(
            dcgettext(
                0 as *const libc::c_char,
                b"      --no-preserve=ATTR_LIST  don't preserve the specified attributes\n      --parents                use full source file name under DIRECTORY\n\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            stdout,
        );
        fputs_unlocked(
            dcgettext(
                0 as *const libc::c_char,
                b"  -R, -r, --recursive          copy directories recursively\n      --reflink[=WHEN]         control clone/CoW copies. See below\n      --remove-destination     remove each existing destination file before\n                                 attempting to open it (contrast with --force)\n\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            stdout,
        );
        fputs_unlocked(
            dcgettext(
                0 as *const libc::c_char,
                b"      --sparse=WHEN            control creation of sparse files. See below\n      --strip-trailing-slashes  remove any trailing slashes from each SOURCE\n                                 argument\n\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            stdout,
        );
        fputs_unlocked(
            dcgettext(
                0 as *const libc::c_char,
                b"  -s, --symbolic-link          make symbolic links instead of copying\n  -S, --suffix=SUFFIX          override the usual backup suffix\n  -t, --target-directory=DIRECTORY  copy all SOURCE arguments into DIRECTORY\n  -T, --no-target-directory    treat DEST as a normal file\n\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            stdout,
        );
        fputs_unlocked(
            dcgettext(
                0 as *const libc::c_char,
                b"  -u, --update                 copy only when the SOURCE file is newer\n                                 than the destination file or when the\n                                 destination file is missing\n  -v, --verbose                explain what is being done\n  -x, --one-file-system        stay on this file system\n\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            stdout,
        );
        fputs_unlocked(
            dcgettext(
                0 as *const libc::c_char,
                b"  -Z                           set SELinux security context of destination\n                                 file to default type\n      --context[=CTX]          like -Z, or if CTX is specified then set the\n                                 SELinux or SMACK security context to CTX\n\0"
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
                b"\nATTR_LIST is a comma-separated list of attributes. Attributes are 'mode' for\npermissions (including any ACL and xattr permissions), 'ownership' for user\nand group, 'timestamps' for file timestamps, 'links' for hard links, 'context'\nfor security context, 'xattr' for extended attributes, and 'all' for all\nattributes.\n\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            stdout,
        );
        fputs_unlocked(
            dcgettext(
                0 as *const libc::c_char,
                b"\nBy default, sparse SOURCE files are detected by a crude heuristic and the\ncorresponding DEST file is made sparse as well.  That is the behavior\nselected by --sparse=auto.  Specify --sparse=always to create a sparse DEST\nfile whenever the SOURCE file contains a long enough sequence of zero bytes.\nUse --sparse=never to inhibit creation of sparse files.\n\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            stdout,
        );
        fputs_unlocked(
            dcgettext(
                0 as *const libc::c_char,
                b"\nWhen --reflink[=always] is specified, perform a lightweight copy, where the\ndata blocks are copied only when modified.  If this is not possible the copy\nfails, or if --reflink=auto is specified, fall back to a standard copy.\nUse --reflink=never to ensure a standard copy is performed.\n\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            stdout,
        );
        emit_backup_suffix_note();
        fputs_unlocked(
            dcgettext(
                0 as *const libc::c_char,
                b"\nAs a special case, cp makes a backup of SOURCE when the force and backup\noptions are given and SOURCE and DEST are the same name for an existing,\nregular file.\n\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            stdout,
        );
        emit_ancillary_info(b"cp\0" as *const u8 as *const libc::c_char);
    }
    exit(status);
}
unsafe extern "C" fn re_protect(
    mut const_dst_name: *const libc::c_char,
    mut dst_dirfd: libc::c_int,
    mut dst_relname: *const libc::c_char,
    mut attr_list: *mut dir_attr,
    mut x: *const cp_options,
) -> bool {
    let mut p: *mut dir_attr = 0 as *mut dir_attr;
    let mut dst_name: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut src_name: *mut libc::c_char = 0 as *mut libc::c_char;
    dst_name = ({
        let mut __old: *const libc::c_char = const_dst_name;
        let mut __len: size_t = (strlen(__old))
            .wrapping_add(1 as libc::c_int as libc::c_ulong);
        let mut fresh0 = ::std::vec::from_elem(0, __len as usize);
        let mut __new: *mut libc::c_char = fresh0.as_mut_ptr() as *mut libc::c_char;
        memcpy(__new as *mut libc::c_void, __old as *const libc::c_void, __len)
            as *mut libc::c_char
    });
    src_name = dst_name
        .offset(dst_relname.offset_from(const_dst_name) as libc::c_long as isize);
    p = attr_list;
    while !p.is_null() {
        *dst_name.offset((*p).slash_offset as isize) = '\0' as i32 as libc::c_char;
        if (*x).preserve_timestamps {
            let mut timespec: [timespec; 2] = [timespec { tv_sec: 0, tv_nsec: 0 }; 2];
            timespec[0 as libc::c_int as usize] = get_stat_atime(&mut (*p).st);
            timespec[1 as libc::c_int as usize] = get_stat_mtime(&mut (*p).st);
            if utimensat(
                dst_dirfd,
                src_name,
                timespec.as_mut_ptr() as *const timespec,
                0 as libc::c_int,
            ) != 0
            {
                error(
                    0 as libc::c_int,
                    *__errno_location(),
                    dcgettext(
                        0 as *const libc::c_char,
                        b"failed to preserve times for %s\0" as *const u8
                            as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    quotearg_style(shell_escape_always_quoting_style, dst_name),
                );
                return 0 as libc::c_int != 0;
            }
        }
        if (*x).preserve_ownership {
            if lchownat(dst_dirfd, src_name, (*p).st.st_uid, (*p).st.st_gid)
                != 0 as libc::c_int
            {
                if !chown_failure_ok(x) {
                    error(
                        0 as libc::c_int,
                        *__errno_location(),
                        dcgettext(
                            0 as *const libc::c_char,
                            b"failed to preserve ownership for %s\0" as *const u8
                                as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                        quotearg_style(shell_escape_always_quoting_style, dst_name),
                    );
                    return 0 as libc::c_int != 0;
                }
                lchownat(
                    dst_dirfd,
                    src_name,
                    -(1 as libc::c_int) as uid_t,
                    (*p).st.st_gid,
                );
            }
        }
        if (*x).preserve_mode {
            if copy_acl(
                src_name,
                -(1 as libc::c_int),
                dst_name,
                -(1 as libc::c_int),
                (*p).st.st_mode,
            ) != 0 as libc::c_int
            {
                return 0 as libc::c_int != 0;
            }
        } else if (*p).restore_mode {
            if lchmodat(dst_dirfd, src_name, (*p).st.st_mode) != 0 as libc::c_int {
                error(
                    0 as libc::c_int,
                    *__errno_location(),
                    dcgettext(
                        0 as *const libc::c_char,
                        b"failed to preserve permissions for %s\0" as *const u8
                            as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    quotearg_style(shell_escape_always_quoting_style, dst_name),
                );
                return 0 as libc::c_int != 0;
            }
        }
        *dst_name.offset((*p).slash_offset as isize) = '/' as i32 as libc::c_char;
        p = (*p).next;
    }
    return 1 as libc::c_int != 0;
}
unsafe extern "C" fn make_dir_parents_private(
    mut const_dir: *const libc::c_char,
    mut src_offset: size_t,
    mut dst_dirfd: libc::c_int,
    mut verbose_fmt_string: *const libc::c_char,
    mut attr_list: *mut *mut dir_attr,
    mut new_dst: *mut bool,
    mut x: *const cp_options,
) -> bool {
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
    let mut dir: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut src: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut dst_dir: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut dirlen: idx_t = dir_len(const_dir) as idx_t;
    *attr_list = 0 as *mut dir_attr;
    if dirlen as libc::c_ulong <= src_offset {
        return 1 as libc::c_int != 0;
    }
    dir = ({
        let mut __old: *const libc::c_char = const_dir;
        let mut __len: size_t = (strlen(__old))
            .wrapping_add(1 as libc::c_int as libc::c_ulong);
        let mut fresh1 = ::std::vec::from_elem(0, __len as usize);
        let mut __new: *mut libc::c_char = fresh1.as_mut_ptr() as *mut libc::c_char;
        memcpy(__new as *mut libc::c_void, __old as *const libc::c_void, __len)
            as *mut libc::c_char
    });
    src = dir.offset(src_offset as isize);
    let mut fresh2 = ::std::vec::from_elem(
        0,
        (dirlen + 1 as libc::c_int as libc::c_long) as libc::c_ulong as usize,
    );
    dst_dir = fresh2.as_mut_ptr() as *mut libc::c_char;
    memcpy(
        dst_dir as *mut libc::c_void,
        dir as *const libc::c_void,
        dirlen as libc::c_ulong,
    );
    *dst_dir.offset(dirlen as isize) = '\0' as i32 as libc::c_char;
    let mut dst_reldir: *const libc::c_char = dst_dir.offset(src_offset as isize);
    while *dst_reldir as libc::c_int == '/' as i32 {
        dst_reldir = dst_reldir.offset(1);
    }
    if fstatat(dst_dirfd, dst_reldir, &mut stats, 0 as libc::c_int) != 0 as libc::c_int {
        let mut slash: *mut libc::c_char = 0 as *mut libc::c_char;
        slash = src;
        while *slash as libc::c_int == '/' as i32 {
            slash = slash.offset(1);
        }
        dst_reldir = slash;
        loop {
            slash = strchr(slash, '/' as i32);
            if slash.is_null() {
                break;
            }
            let mut new: *mut dir_attr = 0 as *mut dir_attr;
            let mut missing_dir: bool = false;
            *slash = '\0' as i32 as libc::c_char;
            missing_dir = fstatat(dst_dirfd, dst_reldir, &mut stats, 0 as libc::c_int)
                != 0 as libc::c_int;
            if missing_dir as libc::c_int != 0
                || (*x).preserve_ownership as libc::c_int != 0
                || (*x).preserve_mode as libc::c_int != 0
                || (*x).preserve_timestamps as libc::c_int != 0
            {
                let mut src_st: stat = stat {
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
                let mut src_errno: libc::c_int = if stat(src, &mut src_st)
                    != 0 as libc::c_int
                {
                    *__errno_location()
                } else if src_st.st_mode & 0o170000 as libc::c_int as libc::c_uint
                    == 0o40000 as libc::c_int as libc::c_uint
                {
                    0 as libc::c_int
                } else {
                    20 as libc::c_int
                };
                if src_errno != 0 {
                    error(
                        0 as libc::c_int,
                        src_errno,
                        dcgettext(
                            0 as *const libc::c_char,
                            b"failed to get attributes of %s\0" as *const u8
                                as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                        quotearg_style(shell_escape_always_quoting_style, src),
                    );
                    return 0 as libc::c_int != 0;
                }
                new = xmalloc(::core::mem::size_of::<dir_attr>() as libc::c_ulong)
                    as *mut dir_attr;
                (*new).st = src_st;
                (*new).slash_offset = slash.offset_from(dir) as libc::c_long as size_t;
                (*new).restore_mode = 0 as libc::c_int != 0;
                (*new).next = *attr_list;
                *attr_list = new;
            }
            if !set_process_security_ctx(
                src,
                dir,
                if missing_dir as libc::c_int != 0 {
                    (*new).st.st_mode
                } else {
                    0 as libc::c_int as libc::c_uint
                },
                missing_dir,
                x,
            ) {
                return 0 as libc::c_int != 0;
            }
            if missing_dir {
                let mut src_mode: mode_t = 0;
                let mut omitted_permissions: mode_t = 0;
                let mut mkdir_mode: mode_t = 0;
                *new_dst = 1 as libc::c_int != 0;
                src_mode = (*new).st.st_mode;
                omitted_permissions = src_mode
                    & (if (*x).preserve_ownership as libc::c_int != 0 {
                        (0o400 as libc::c_int | 0o200 as libc::c_int
                            | 0o100 as libc::c_int) >> 3 as libc::c_int
                            | (0o400 as libc::c_int | 0o200 as libc::c_int
                                | 0o100 as libc::c_int) >> 3 as libc::c_int
                                >> 3 as libc::c_int
                    } else {
                        (if (*x).preserve_mode as libc::c_int != 0 {
                            0o200 as libc::c_int >> 3 as libc::c_int
                                | 0o200 as libc::c_int >> 3 as libc::c_int
                                    >> 3 as libc::c_int
                        } else {
                            0 as libc::c_int
                        })
                    }) as libc::c_uint;
                mkdir_mode = if (*x).explicit_no_preserve_mode as libc::c_int != 0 {
                    (0o400 as libc::c_int | 0o200 as libc::c_int | 0o100 as libc::c_int
                        | (0o400 as libc::c_int | 0o200 as libc::c_int
                            | 0o100 as libc::c_int) >> 3 as libc::c_int
                        | (0o400 as libc::c_int | 0o200 as libc::c_int
                            | 0o100 as libc::c_int) >> 3 as libc::c_int
                            >> 3 as libc::c_int) as libc::c_uint
                } else {
                    src_mode
                };
                mkdir_mode
                    &= (0o4000 as libc::c_int | 0o2000 as libc::c_int
                        | 0o1000 as libc::c_int
                        | (0o400 as libc::c_int | 0o200 as libc::c_int
                            | 0o100 as libc::c_int)
                        | (0o400 as libc::c_int | 0o200 as libc::c_int
                            | 0o100 as libc::c_int) >> 3 as libc::c_int
                        | (0o400 as libc::c_int | 0o200 as libc::c_int
                            | 0o100 as libc::c_int) >> 3 as libc::c_int
                            >> 3 as libc::c_int) as libc::c_uint & !omitted_permissions;
                if mkdirat(dst_dirfd, dst_reldir, mkdir_mode) != 0 as libc::c_int {
                    error(
                        0 as libc::c_int,
                        *__errno_location(),
                        dcgettext(
                            0 as *const libc::c_char,
                            b"cannot make directory %s\0" as *const u8
                                as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                        quotearg_style(shell_escape_always_quoting_style, dir),
                    );
                    return 0 as libc::c_int != 0;
                } else {
                    if !verbose_fmt_string.is_null() {
                        printf(verbose_fmt_string, src, dir);
                    }
                }
                if fstatat(dst_dirfd, dst_reldir, &mut stats, 0x100 as libc::c_int) != 0
                {
                    error(
                        0 as libc::c_int,
                        *__errno_location(),
                        dcgettext(
                            0 as *const libc::c_char,
                            b"failed to get attributes of %s\0" as *const u8
                                as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                        quotearg_style(shell_escape_always_quoting_style, dir),
                    );
                    return 0 as libc::c_int != 0;
                }
                if !(*x).preserve_mode {
                    if omitted_permissions & !stats.st_mode != 0 {
                        omitted_permissions &= !cached_umask();
                    }
                    if omitted_permissions & !stats.st_mode != 0
                        || stats.st_mode
                            & (0o400 as libc::c_int | 0o200 as libc::c_int
                                | 0o100 as libc::c_int) as libc::c_uint
                            != (0o400 as libc::c_int | 0o200 as libc::c_int
                                | 0o100 as libc::c_int) as libc::c_uint
                    {
                        (*new).st.st_mode = stats.st_mode | omitted_permissions;
                        (*new).restore_mode = 1 as libc::c_int != 0;
                    }
                }
                let mut accessible: mode_t = stats.st_mode
                    | (0o400 as libc::c_int | 0o200 as libc::c_int
                        | 0o100 as libc::c_int) as libc::c_uint;
                if stats.st_mode != accessible {
                    if lchmodat(dst_dirfd, dst_reldir, accessible) != 0 as libc::c_int {
                        error(
                            0 as libc::c_int,
                            *__errno_location(),
                            dcgettext(
                                0 as *const libc::c_char,
                                b"setting permissions for %s\0" as *const u8
                                    as *const libc::c_char,
                                5 as libc::c_int,
                            ),
                            quotearg_style(shell_escape_always_quoting_style, dir),
                        );
                        return 0 as libc::c_int != 0;
                    }
                }
            } else if !(stats.st_mode & 0o170000 as libc::c_int as libc::c_uint
                == 0o40000 as libc::c_int as libc::c_uint)
            {
                error(
                    0 as libc::c_int,
                    0 as libc::c_int,
                    dcgettext(
                        0 as *const libc::c_char,
                        b"%s exists but is not a directory\0" as *const u8
                            as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    quotearg_style(shell_escape_always_quoting_style, dir),
                );
                return 0 as libc::c_int != 0;
            } else {
                *new_dst = 0 as libc::c_int != 0;
            }
            if !*new_dst
                && (!((*x).set_security_context).is_null()
                    || (*x).preserve_security_context as libc::c_int != 0)
            {
                if !set_file_security_ctx(dir, 0 as libc::c_int != 0, x)
                    && (*x).require_preserve_context as libc::c_int != 0
                {
                    return 0 as libc::c_int != 0;
                }
            }
            let fresh3 = slash;
            slash = slash.offset(1);
            *fresh3 = '/' as i32 as libc::c_char;
            while *slash as libc::c_int == '/' as i32 {
                slash = slash.offset(1);
            }
        }
    } else if !(stats.st_mode & 0o170000 as libc::c_int as libc::c_uint
        == 0o40000 as libc::c_int as libc::c_uint)
    {
        error(
            0 as libc::c_int,
            0 as libc::c_int,
            dcgettext(
                0 as *const libc::c_char,
                b"%s exists but is not a directory\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
            quotearg_style(shell_escape_always_quoting_style, dst_dir),
        );
        return 0 as libc::c_int != 0;
    } else {
        *new_dst = 0 as libc::c_int != 0;
    }
    return 1 as libc::c_int != 0;
}
unsafe extern "C" fn do_copy(
    mut n_files: libc::c_int,
    mut file: *mut *mut libc::c_char,
    mut target_directory: *const libc::c_char,
    mut no_target_directory: bool,
    mut x: *mut cp_options,
) -> bool {
    let mut sb: stat = stat {
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
    let mut new_dst: bool = 0 as libc::c_int != 0;
    let mut ok: bool = 1 as libc::c_int != 0;
    if n_files <= target_directory.is_null() as libc::c_int {
        if n_files <= 0 as libc::c_int {
            error(
                0 as libc::c_int,
                0 as libc::c_int,
                dcgettext(
                    0 as *const libc::c_char,
                    b"missing file operand\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
            );
        } else {
            error(
                0 as libc::c_int,
                0 as libc::c_int,
                dcgettext(
                    0 as *const libc::c_char,
                    b"missing destination file operand after %s\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
                quotearg_style(
                    shell_escape_always_quoting_style,
                    *file.offset(0 as libc::c_int as isize),
                ),
            );
        }
        usage(1 as libc::c_int);
    }
    sb.st_mode = 0 as libc::c_int as __mode_t;
    let mut target_dirfd: libc::c_int = -(100 as libc::c_int);
    if no_target_directory {
        if !target_directory.is_null() {
            if ::core::mem::size_of::<C2RustUnnamed_4>() as libc::c_ulong != 0 {
                error(
                    1 as libc::c_int,
                    0 as libc::c_int,
                    dcgettext(
                        0 as *const libc::c_char,
                        b"cannot combine --target-directory (-t) and --no-target-directory (-T)\0"
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
                        b"cannot combine --target-directory (-t) and --no-target-directory (-T)\0"
                            as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                );
                if 0 as libc::c_int != 0 {} else {
                    unreachable!();
                };
            };
        }
        if (2 as libc::c_int) < n_files {
            error(
                0 as libc::c_int,
                0 as libc::c_int,
                dcgettext(
                    0 as *const libc::c_char,
                    b"extra operand %s\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
                quotearg_style(
                    shell_escape_always_quoting_style,
                    *file.offset(2 as libc::c_int as isize),
                ),
            );
            usage(1 as libc::c_int);
        }
    } else if !target_directory.is_null() {
        target_dirfd = target_directory_operand(target_directory, &mut sb);
        if !target_dirfd_valid(target_dirfd) {
            if ::core::mem::size_of::<C2RustUnnamed_3>() as libc::c_ulong != 0 {
                error(
                    1 as libc::c_int,
                    *__errno_location(),
                    dcgettext(
                        0 as *const libc::c_char,
                        b"target directory %s\0" as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    quotearg_style(shell_escape_always_quoting_style, target_directory),
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
                        b"target directory %s\0" as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    quotearg_style(shell_escape_always_quoting_style, target_directory),
                );
                if 0 as libc::c_int != 0 {} else {
                    unreachable!();
                };
            };
        }
    } else {
        let mut lastfile: *const libc::c_char = *file
            .offset((n_files - 1 as libc::c_int) as isize);
        let mut fd: libc::c_int = target_directory_operand(lastfile, &mut sb);
        if target_dirfd_valid(fd) {
            target_dirfd = fd;
            target_directory = lastfile;
            n_files -= 1;
        } else {
            let mut err: libc::c_int = *__errno_location();
            if err == 2 as libc::c_int {
                new_dst = 1 as libc::c_int != 0;
            }
            if (2 as libc::c_int) < n_files
                || O_PATHSEARCH as libc::c_int == 0 as libc::c_int
                    && err == 13 as libc::c_int
                    && (sb.st_mode != 0 || stat(lastfile, &mut sb) == 0 as libc::c_int)
                    && sb.st_mode & 0o170000 as libc::c_int as libc::c_uint
                        == 0o40000 as libc::c_int as libc::c_uint
            {
                if ::core::mem::size_of::<C2RustUnnamed_2>() as libc::c_ulong != 0 {
                    error(
                        1 as libc::c_int,
                        err,
                        dcgettext(
                            0 as *const libc::c_char,
                            b"target %s\0" as *const u8 as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                        quotearg_style(shell_escape_always_quoting_style, lastfile),
                    );
                    if 0 as libc::c_int != 0 {} else {
                        unreachable!();
                    };
                } else {
                    error(
                        1 as libc::c_int,
                        err,
                        dcgettext(
                            0 as *const libc::c_char,
                            b"target %s\0" as *const u8 as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                        quotearg_style(shell_escape_always_quoting_style, lastfile),
                    );
                    if 0 as libc::c_int != 0 {} else {
                        unreachable!();
                    };
                };
            }
        }
    }
    if !target_directory.is_null() {
        if 2 as libc::c_int <= n_files {
            dest_info_init(x);
            src_info_init(x);
        }
        let mut i: libc::c_int = 0 as libc::c_int;
        while i < n_files {
            let mut dst_name: *mut libc::c_char = 0 as *mut libc::c_char;
            let mut parent_exists: bool = 1 as libc::c_int != 0;
            let mut attr_list: *mut dir_attr = 0 as *mut dir_attr;
            let mut arg_in_concat: *mut libc::c_char = 0 as *mut libc::c_char;
            let mut arg: *mut libc::c_char = *file.offset(i as isize);
            if remove_trailing_slashes {
                strip_trailing_slashes(arg);
            }
            if parents_option {
                let mut arg_no_trailing_slash: *mut libc::c_char = 0
                    as *mut libc::c_char;
                arg_no_trailing_slash = ({
                    let mut __old: *const libc::c_char = arg;
                    let mut __len: size_t = (strlen(__old))
                        .wrapping_add(1 as libc::c_int as libc::c_ulong);
                    let mut fresh4 = ::std::vec::from_elem(0, __len as usize);
                    let mut __new: *mut libc::c_char = fresh4.as_mut_ptr()
                        as *mut libc::c_char;
                    memcpy(
                        __new as *mut libc::c_void,
                        __old as *const libc::c_void,
                        __len,
                    ) as *mut libc::c_char
                });
                strip_trailing_slashes(arg_no_trailing_slash);
                dst_name = file_name_concat(
                    target_directory,
                    arg_no_trailing_slash,
                    &mut arg_in_concat,
                );
                parent_exists = make_dir_parents_private(
                    dst_name,
                    arg_in_concat.offset_from(dst_name) as libc::c_long as size_t,
                    target_dirfd,
                    if (*x).verbose as libc::c_int != 0 {
                        b"%s -> %s\n\0" as *const u8 as *const libc::c_char
                    } else {
                        0 as *const libc::c_char
                    },
                    &mut attr_list,
                    &mut new_dst,
                    x,
                );
                while *arg_in_concat as libc::c_int == '/' as i32 {
                    arg_in_concat = arg_in_concat.offset(1);
                }
            } else {
                let mut arg_base: *mut libc::c_char = 0 as *mut libc::c_char;
                arg_base = ({
                    let mut __old: *const libc::c_char = last_component(arg);
                    let mut __len: size_t = (strlen(__old))
                        .wrapping_add(1 as libc::c_int as libc::c_ulong);
                    let mut fresh5 = ::std::vec::from_elem(0, __len as usize);
                    let mut __new: *mut libc::c_char = fresh5.as_mut_ptr()
                        as *mut libc::c_char;
                    memcpy(
                        __new as *mut libc::c_void,
                        __old as *const libc::c_void,
                        __len,
                    ) as *mut libc::c_char
                });
                strip_trailing_slashes(arg_base);
                arg_base = arg_base
                    .offset(
                        (strcmp(arg_base, b"..\0" as *const u8 as *const libc::c_char)
                            == 0 as libc::c_int) as libc::c_int as isize,
                    );
                dst_name = file_name_concat(
                    target_directory,
                    arg_base,
                    &mut arg_in_concat,
                );
            }
            if !parent_exists {
                ok = 0 as libc::c_int != 0;
            } else {
                let mut copy_into_self: bool = false;
                ok = (ok as libc::c_int
                    & copy(
                        arg,
                        dst_name,
                        target_dirfd,
                        arg_in_concat,
                        new_dst as libc::c_int,
                        x,
                        &mut copy_into_self,
                        0 as *mut bool,
                    ) as libc::c_int) as bool;
                if parents_option {
                    ok = (ok as libc::c_int
                        & re_protect(dst_name, target_dirfd, arg_in_concat, attr_list, x)
                            as libc::c_int) as bool;
                }
            }
            if parents_option {
                while !attr_list.is_null() {
                    let mut p: *mut dir_attr = attr_list;
                    attr_list = (*attr_list).next;
                    free(p as *mut libc::c_void);
                }
            }
            free(dst_name as *mut libc::c_void);
            i += 1;
        }
    } else {
        let mut source: *const libc::c_char = *file.offset(0 as libc::c_int as isize);
        let mut dest: *const libc::c_char = *file.offset(1 as libc::c_int as isize);
        let mut unused: bool = false;
        if parents_option {
            error(
                0 as libc::c_int,
                0 as libc::c_int,
                dcgettext(
                    0 as *const libc::c_char,
                    b"with --parents, the destination must be a directory\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
            );
            usage(1 as libc::c_int);
        }
        if (*x).unlink_dest_after_failed_open as libc::c_int != 0
            && (*x).backup_type as libc::c_uint
                != no_backups as libc::c_int as libc::c_uint
            && strcmp(source, dest) == 0 as libc::c_int && !new_dst
            && (sb.st_mode != 0 as libc::c_int as libc::c_uint
                || stat(dest, &mut sb) == 0 as libc::c_int)
            && sb.st_mode & 0o170000 as libc::c_int as libc::c_uint
                == 0o100000 as libc::c_int as libc::c_uint
        {
            static mut x_tmp: cp_options = cp_options {
                backup_type: no_backups,
                dereference: 0 as Dereference_symlink,
                interactive: 0 as Interactive,
                sparse_mode: SPARSE_UNUSED,
                mode: 0,
                copy_as_regular: false,
                unlink_dest_before_opening: false,
                unlink_dest_after_failed_open: false,
                hard_link: false,
                move_mode: false,
                no_copy: false,
                install_mode: false,
                chown_privileges: false,
                owner_privileges: false,
                one_file_system: false,
                preserve_ownership: false,
                preserve_mode: false,
                preserve_timestamps: false,
                explicit_no_preserve_mode: false,
                set_security_context: 0 as *const selabel_handle as *mut selabel_handle,
                preserve_links: false,
                data_copy_required: false,
                require_preserve: false,
                preserve_security_context: false,
                require_preserve_context: false,
                preserve_xattr: false,
                require_preserve_xattr: false,
                reduce_diagnostics: false,
                recursive: false,
                set_mode: false,
                symbolic_link: false,
                update: false,
                verbose: false,
                debug: false,
                stdin_tty: false,
                open_dangling_dest_symlink: false,
                last_file: false,
                rename_errno: 0,
                reflink_mode: REFLINK_NEVER,
                dest_info: 0 as *const Hash_table as *mut Hash_table,
                src_info: 0 as *const Hash_table as *mut Hash_table,
            };
            dest = find_backup_file_name(-(100 as libc::c_int), dest, (*x).backup_type);
            x_tmp = *x;
            x_tmp.backup_type = no_backups;
            x = &mut x_tmp;
        }
        ok = copy(
            source,
            dest,
            -(100 as libc::c_int),
            dest,
            -(new_dst as libc::c_int),
            x,
            &mut unused,
            0 as *mut bool,
        );
    }
    return ok;
}
unsafe extern "C" fn cp_option_init(mut x: *mut cp_options) {
    cp_options_default(x);
    (*x).copy_as_regular = 1 as libc::c_int != 0;
    (*x).dereference = DEREF_UNDEFINED;
    (*x).unlink_dest_before_opening = 0 as libc::c_int != 0;
    (*x).unlink_dest_after_failed_open = 0 as libc::c_int != 0;
    (*x).hard_link = 0 as libc::c_int != 0;
    (*x).interactive = I_UNSPECIFIED;
    (*x).move_mode = 0 as libc::c_int != 0;
    (*x).install_mode = 0 as libc::c_int != 0;
    (*x).one_file_system = 0 as libc::c_int != 0;
    (*x).reflink_mode = REFLINK_AUTO;
    (*x).preserve_ownership = 0 as libc::c_int != 0;
    (*x).preserve_links = 0 as libc::c_int != 0;
    (*x).preserve_mode = 0 as libc::c_int != 0;
    (*x).preserve_timestamps = 0 as libc::c_int != 0;
    (*x).explicit_no_preserve_mode = 0 as libc::c_int != 0;
    (*x).preserve_security_context = 0 as libc::c_int != 0;
    (*x).require_preserve_context = 0 as libc::c_int != 0;
    (*x).set_security_context = 0 as *mut selabel_handle;
    (*x).preserve_xattr = 0 as libc::c_int != 0;
    (*x).reduce_diagnostics = 0 as libc::c_int != 0;
    (*x).require_preserve_xattr = 0 as libc::c_int != 0;
    (*x).data_copy_required = 1 as libc::c_int != 0;
    (*x).require_preserve = 0 as libc::c_int != 0;
    (*x).recursive = 0 as libc::c_int != 0;
    (*x).sparse_mode = SPARSE_AUTO;
    (*x).symbolic_link = 0 as libc::c_int != 0;
    (*x).set_mode = 0 as libc::c_int != 0;
    (*x).mode = 0 as libc::c_int as mode_t;
    (*x).stdin_tty = 0 as libc::c_int != 0;
    (*x).update = 0 as libc::c_int != 0;
    (*x).verbose = 0 as libc::c_int != 0;
    (*x)
        .open_dangling_dest_symlink = !(getenv(
        b"POSIXLY_CORRECT\0" as *const u8 as *const libc::c_char,
    ))
        .is_null();
    (*x).dest_info = 0 as *mut Hash_table;
    (*x).src_info = 0 as *mut Hash_table;
}
unsafe extern "C" fn decode_preserve_arg(
    mut arg: *const libc::c_char,
    mut x: *mut cp_options,
    mut on_off: bool,
) {
    static mut preserve_vals: [File_attribute; 7] = [
        PRESERVE_MODE,
        PRESERVE_TIMESTAMPS,
        PRESERVE_OWNERSHIP,
        PRESERVE_LINK,
        PRESERVE_CONTEXT,
        PRESERVE_XATTR,
        PRESERVE_ALL,
    ];
    static mut preserve_args: [*const libc::c_char; 8] = [
        b"mode\0" as *const u8 as *const libc::c_char,
        b"timestamps\0" as *const u8 as *const libc::c_char,
        b"ownership\0" as *const u8 as *const libc::c_char,
        b"links\0" as *const u8 as *const libc::c_char,
        b"context\0" as *const u8 as *const libc::c_char,
        b"xattr\0" as *const u8 as *const libc::c_char,
        b"all\0" as *const u8 as *const libc::c_char,
        0 as *const libc::c_char,
    ];
    let mut arg_writable: *mut libc::c_char = xstrdup(arg);
    let mut s: *mut libc::c_char = arg_writable;
    loop {
        let mut comma: *mut libc::c_char = strchr(s, ',' as i32);
        let mut val: File_attribute = PRESERVE_MODE;
        if !comma.is_null() {
            let fresh6 = comma;
            comma = comma.offset(1);
            *fresh6 = 0 as libc::c_int as libc::c_char;
        }
        val = preserve_vals[__xargmatch_internal(
            if on_off as libc::c_int != 0 {
                b"--preserve\0" as *const u8 as *const libc::c_char
            } else {
                b"--no-preserve\0" as *const u8 as *const libc::c_char
            },
            s,
            preserve_args.as_ptr(),
            preserve_vals.as_ptr() as *const libc::c_void,
            ::core::mem::size_of::<File_attribute>() as libc::c_ulong,
            argmatch_die,
            1 as libc::c_int != 0,
        ) as usize];
        match val as libc::c_uint {
            0 => {
                (*x).preserve_mode = on_off;
                (*x).explicit_no_preserve_mode = !on_off;
            }
            1 => {
                (*x).preserve_timestamps = on_off;
            }
            2 => {
                (*x).preserve_ownership = on_off;
            }
            3 => {
                (*x).preserve_links = on_off;
            }
            4 => {
                (*x).require_preserve_context = on_off;
                (*x).preserve_security_context = on_off;
            }
            5 => {
                (*x).preserve_xattr = on_off;
                (*x).require_preserve_xattr = on_off;
            }
            6 => {
                (*x).preserve_mode = on_off;
                (*x).preserve_timestamps = on_off;
                (*x).preserve_ownership = on_off;
                (*x).preserve_links = on_off;
                (*x).explicit_no_preserve_mode = !on_off;
                if selinux_enabled {
                    (*x).preserve_security_context = on_off;
                }
                (*x).preserve_xattr = on_off;
            }
            _ => {
                abort();
            }
        }
        s = comma;
        if s.is_null() {
            break;
        }
    }
    free(arg_writable as *mut libc::c_void);
}
unsafe fn main_0(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut c: libc::c_int = 0;
    let mut ok: bool = false;
    let mut make_backups: bool = 0 as libc::c_int != 0;
    let mut backup_suffix: *const libc::c_char = 0 as *const libc::c_char;
    let mut version_control_string: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut x: cp_options = cp_options {
        backup_type: no_backups,
        dereference: 0 as Dereference_symlink,
        interactive: 0 as Interactive,
        sparse_mode: SPARSE_UNUSED,
        mode: 0,
        copy_as_regular: false,
        unlink_dest_before_opening: false,
        unlink_dest_after_failed_open: false,
        hard_link: false,
        move_mode: false,
        no_copy: false,
        install_mode: false,
        chown_privileges: false,
        owner_privileges: false,
        one_file_system: false,
        preserve_ownership: false,
        preserve_mode: false,
        preserve_timestamps: false,
        explicit_no_preserve_mode: false,
        set_security_context: 0 as *const selabel_handle as *mut selabel_handle,
        preserve_links: false,
        data_copy_required: false,
        require_preserve: false,
        preserve_security_context: false,
        require_preserve_context: false,
        preserve_xattr: false,
        require_preserve_xattr: false,
        reduce_diagnostics: false,
        recursive: false,
        set_mode: false,
        symbolic_link: false,
        update: false,
        verbose: false,
        debug: false,
        stdin_tty: false,
        open_dangling_dest_symlink: false,
        last_file: false,
        rename_errno: 0,
        reflink_mode: REFLINK_NEVER,
        dest_info: 0 as *const Hash_table as *mut Hash_table,
        src_info: 0 as *const Hash_table as *mut Hash_table,
    };
    let mut copy_contents: bool = 0 as libc::c_int != 0;
    let mut target_directory: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut no_target_directory: bool = 0 as libc::c_int != 0;
    let mut scontext: *const libc::c_char = 0 as *const libc::c_char;
    set_program_name(*argv.offset(0 as libc::c_int as isize));
    setlocale(6 as libc::c_int, b"\0" as *const u8 as *const libc::c_char);
    bindtextdomain(
        b"coreutils\0" as *const u8 as *const libc::c_char,
        b"/usr/local/share/locale\0" as *const u8 as *const libc::c_char,
    );
    textdomain(b"coreutils\0" as *const u8 as *const libc::c_char);
    atexit(Some(close_stdin as unsafe extern "C" fn() -> ()));
    selinux_enabled = (0 as libc::c_int) < 0 as libc::c_int;
    cp_option_init(&mut x);
    loop {
        c = getopt_long(
            argc,
            argv,
            b"abdfHilLnprst:uvxPRS:TZ\0" as *const u8 as *const libc::c_char,
            long_opts.as_ptr(),
            0 as *mut libc::c_int,
        );
        if !(c != -(1 as libc::c_int)) {
            break;
        }
        let mut current_block_77: u64;
        match c {
            263 => {
                x
                    .sparse_mode = sparse_type[__xargmatch_internal(
                    b"--sparse\0" as *const u8 as *const libc::c_char,
                    optarg,
                    sparse_type_string.as_ptr(),
                    sparse_type.as_ptr() as *const libc::c_void,
                    ::core::mem::size_of::<Sparse_type>() as libc::c_ulong,
                    argmatch_die,
                    1 as libc::c_int != 0,
                ) as usize];
                current_block_77 = 15855550149339537395;
            }
            262 => {
                if optarg.is_null() {
                    x.reflink_mode = REFLINK_ALWAYS;
                } else {
                    x
                        .reflink_mode = reflink_type[__xargmatch_internal(
                        b"--reflink\0" as *const u8 as *const libc::c_char,
                        optarg,
                        reflink_type_string.as_ptr(),
                        reflink_type.as_ptr() as *const libc::c_void,
                        ::core::mem::size_of::<Reflink_type>() as libc::c_ulong,
                        argmatch_die,
                        1 as libc::c_int != 0,
                    ) as usize];
                }
                current_block_77 = 15855550149339537395;
            }
            97 => {
                x.dereference = DEREF_NEVER;
                x.preserve_links = 1 as libc::c_int != 0;
                x.preserve_ownership = 1 as libc::c_int != 0;
                x.preserve_mode = 1 as libc::c_int != 0;
                x.preserve_timestamps = 1 as libc::c_int != 0;
                x.require_preserve = 1 as libc::c_int != 0;
                if selinux_enabled {
                    x.preserve_security_context = 1 as libc::c_int != 0;
                }
                x.preserve_xattr = 1 as libc::c_int != 0;
                x.reduce_diagnostics = 1 as libc::c_int != 0;
                x.recursive = 1 as libc::c_int != 0;
                current_block_77 = 15855550149339537395;
            }
            98 => {
                make_backups = 1 as libc::c_int != 0;
                if !optarg.is_null() {
                    version_control_string = optarg;
                }
                current_block_77 = 15855550149339537395;
            }
            256 => {
                x.data_copy_required = 0 as libc::c_int != 0;
                current_block_77 = 15855550149339537395;
            }
            258 => {
                x.verbose = 1 as libc::c_int != 0;
                x.debug = x.verbose;
                current_block_77 = 15855550149339537395;
            }
            257 => {
                copy_contents = 1 as libc::c_int != 0;
                current_block_77 = 15855550149339537395;
            }
            100 => {
                x.preserve_links = 1 as libc::c_int != 0;
                x.dereference = DEREF_NEVER;
                current_block_77 = 15855550149339537395;
            }
            102 => {
                x.unlink_dest_after_failed_open = 1 as libc::c_int != 0;
                current_block_77 = 15855550149339537395;
            }
            72 => {
                x.dereference = DEREF_COMMAND_LINE_ARGUMENTS;
                current_block_77 = 15855550149339537395;
            }
            105 => {
                x.interactive = I_ASK_USER;
                current_block_77 = 15855550149339537395;
            }
            108 => {
                x.hard_link = 1 as libc::c_int != 0;
                current_block_77 = 15855550149339537395;
            }
            76 => {
                x.dereference = DEREF_ALWAYS;
                current_block_77 = 15855550149339537395;
            }
            110 => {
                x.interactive = I_ALWAYS_NO;
                current_block_77 = 15855550149339537395;
            }
            80 => {
                x.dereference = DEREF_NEVER;
                current_block_77 = 15855550149339537395;
            }
            259 => {
                decode_preserve_arg(optarg, &mut x, 0 as libc::c_int != 0);
                current_block_77 = 15855550149339537395;
            }
            261 => {
                if optarg.is_null() {
                    current_block_77 = 5212612941259282577;
                } else {
                    decode_preserve_arg(optarg, &mut x, 1 as libc::c_int != 0);
                    x.require_preserve = 1 as libc::c_int != 0;
                    current_block_77 = 15855550149339537395;
                }
            }
            112 => {
                current_block_77 = 5212612941259282577;
            }
            260 => {
                parents_option = 1 as libc::c_int != 0;
                current_block_77 = 15855550149339537395;
            }
            114 | 82 => {
                x.recursive = 1 as libc::c_int != 0;
                current_block_77 = 15855550149339537395;
            }
            265 => {
                x.unlink_dest_before_opening = 1 as libc::c_int != 0;
                current_block_77 = 15855550149339537395;
            }
            264 => {
                remove_trailing_slashes = 1 as libc::c_int != 0;
                current_block_77 = 15855550149339537395;
            }
            115 => {
                x.symbolic_link = 1 as libc::c_int != 0;
                current_block_77 = 15855550149339537395;
            }
            116 => {
                if !target_directory.is_null() {
                    if ::core::mem::size_of::<C2RustUnnamed_9>() as libc::c_ulong != 0 {
                        error(
                            1 as libc::c_int,
                            0 as libc::c_int,
                            dcgettext(
                                0 as *const libc::c_char,
                                b"multiple target directories specified\0" as *const u8
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
                                b"multiple target directories specified\0" as *const u8
                                    as *const libc::c_char,
                                5 as libc::c_int,
                            ),
                        );
                        if 0 as libc::c_int != 0 {} else {
                            unreachable!();
                        };
                    };
                }
                target_directory = optarg;
                current_block_77 = 15855550149339537395;
            }
            84 => {
                no_target_directory = 1 as libc::c_int != 0;
                current_block_77 = 15855550149339537395;
            }
            117 => {
                x.update = 1 as libc::c_int != 0;
                current_block_77 = 15855550149339537395;
            }
            118 => {
                x.verbose = 1 as libc::c_int != 0;
                current_block_77 = 15855550149339537395;
            }
            120 => {
                x.one_file_system = 1 as libc::c_int != 0;
                current_block_77 = 15855550149339537395;
            }
            90 => {
                if selinux_enabled {
                    if !optarg.is_null() {
                        scontext = optarg;
                    } else {
                        x
                            .set_security_context = selabel_open(
                            0 as libc::c_int,
                            0 as *mut selinux_opt,
                            0 as libc::c_int as libc::c_uint,
                        );
                        if (x.set_security_context).is_null() {
                            error(
                                0 as libc::c_int,
                                *__errno_location(),
                                dcgettext(
                                    0 as *const libc::c_char,
                                    b"warning: ignoring --context\0" as *const u8
                                        as *const libc::c_char,
                                    5 as libc::c_int,
                                ),
                            );
                        }
                    }
                } else if !optarg.is_null() {
                    error(
                        0 as libc::c_int,
                        0 as libc::c_int,
                        dcgettext(
                            0 as *const libc::c_char,
                            b"warning: ignoring --context; it requires an SELinux-enabled kernel\0"
                                as *const u8 as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                    );
                }
                current_block_77 = 15855550149339537395;
            }
            83 => {
                make_backups = 1 as libc::c_int != 0;
                backup_suffix = optarg;
                current_block_77 = 15855550149339537395;
            }
            -2 => {
                usage(0 as libc::c_int);
                current_block_77 = 15855550149339537395;
            }
            -3 => {
                version_etc(
                    stdout,
                    b"cp\0" as *const u8 as *const libc::c_char,
                    b"GNU coreutils\0" as *const u8 as *const libc::c_char,
                    Version,
                    b"Torbjorn Granlund\0" as *const u8 as *const libc::c_char,
                    b"David MacKenzie\0" as *const u8 as *const libc::c_char,
                    b"Jim Meyering\0" as *const u8 as *const libc::c_char,
                    0 as *mut libc::c_void as *mut libc::c_char,
                );
                exit(0 as libc::c_int);
            }
            _ => {
                usage(1 as libc::c_int);
                current_block_77 = 15855550149339537395;
            }
        }
        match current_block_77 {
            5212612941259282577 => {
                x.preserve_ownership = 1 as libc::c_int != 0;
                x.preserve_mode = 1 as libc::c_int != 0;
                x.preserve_timestamps = 1 as libc::c_int != 0;
                x.require_preserve = 1 as libc::c_int != 0;
            }
            _ => {}
        }
    }
    if x.hard_link as libc::c_int != 0 && x.symbolic_link as libc::c_int != 0 {
        error(
            0 as libc::c_int,
            0 as libc::c_int,
            dcgettext(
                0 as *const libc::c_char,
                b"cannot make both hard and symbolic links\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
        usage(1 as libc::c_int);
    }
    if x.interactive as libc::c_uint == I_ALWAYS_NO as libc::c_int as libc::c_uint {
        x.update = 0 as libc::c_int != 0;
    }
    if make_backups as libc::c_int != 0
        && x.interactive as libc::c_uint == I_ALWAYS_NO as libc::c_int as libc::c_uint
    {
        error(
            0 as libc::c_int,
            0 as libc::c_int,
            dcgettext(
                0 as *const libc::c_char,
                b"options --backup and --no-clobber are mutually exclusive\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
        usage(1 as libc::c_int);
    }
    if x.reflink_mode as libc::c_uint == REFLINK_ALWAYS as libc::c_int as libc::c_uint
        && x.sparse_mode as libc::c_uint != SPARSE_AUTO as libc::c_int as libc::c_uint
    {
        error(
            0 as libc::c_int,
            0 as libc::c_int,
            dcgettext(
                0 as *const libc::c_char,
                b"--reflink can be used only with --sparse=auto\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
        usage(1 as libc::c_int);
    }
    x
        .backup_type = (if make_backups as libc::c_int != 0 {
        xget_version(
            dcgettext(
                0 as *const libc::c_char,
                b"backup type\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            version_control_string,
        ) as libc::c_uint
    } else {
        no_backups as libc::c_int as libc::c_uint
    }) as backup_type;
    set_simple_backup_suffix(backup_suffix);
    if x.dereference as libc::c_uint == DEREF_UNDEFINED as libc::c_int as libc::c_uint {
        if x.recursive as libc::c_int != 0 && !x.hard_link {
            x.dereference = DEREF_NEVER;
        } else {
            x.dereference = DEREF_ALWAYS;
        }
    }
    if x.recursive {
        x.copy_as_regular = copy_contents;
    }
    if (!(x.set_security_context).is_null() || !scontext.is_null())
        && !x.require_preserve_context
    {
        x.preserve_security_context = 0 as libc::c_int != 0;
    }
    if x.preserve_security_context as libc::c_int != 0
        && (!(x.set_security_context).is_null() || !scontext.is_null())
    {
        if ::core::mem::size_of::<C2RustUnnamed_8>() as libc::c_ulong != 0 {
            error(
                1 as libc::c_int,
                0 as libc::c_int,
                dcgettext(
                    0 as *const libc::c_char,
                    b"cannot set target context and preserve it\0" as *const u8
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
                    b"cannot set target context and preserve it\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
            );
            if 0 as libc::c_int != 0 {} else {
                unreachable!();
            };
        };
    }
    if x.require_preserve_context as libc::c_int != 0 && !selinux_enabled {
        if ::core::mem::size_of::<C2RustUnnamed_7>() as libc::c_ulong != 0 {
            error(
                1 as libc::c_int,
                0 as libc::c_int,
                dcgettext(
                    0 as *const libc::c_char,
                    b"cannot preserve security context without an SELinux-enabled kernel\0"
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
                    b"cannot preserve security context without an SELinux-enabled kernel\0"
                        as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
            );
            if 0 as libc::c_int != 0 {} else {
                unreachable!();
            };
        };
    }
    if !scontext.is_null() && setfscreatecon(scontext) < 0 as libc::c_int {
        if ::core::mem::size_of::<C2RustUnnamed_6>() as libc::c_ulong != 0 {
            error(
                1 as libc::c_int,
                *__errno_location(),
                dcgettext(
                    0 as *const libc::c_char,
                    b"failed to set default file creation context to %s\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
                quote(scontext),
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
                    b"failed to set default file creation context to %s\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
                quote(scontext),
            );
            if 0 as libc::c_int != 0 {} else {
                unreachable!();
            };
        };
    }
    if x.require_preserve_xattr {
        if ::core::mem::size_of::<C2RustUnnamed_5>() as libc::c_ulong != 0 {
            error(
                1 as libc::c_int,
                0 as libc::c_int,
                dcgettext(
                    0 as *const libc::c_char,
                    b"cannot preserve extended attributes, cp is built without xattr support\0"
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
                    b"cannot preserve extended attributes, cp is built without xattr support\0"
                        as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
            );
            if 0 as libc::c_int != 0 {} else {
                unreachable!();
            };
        };
    }
    hash_init();
    ok = do_copy(
        argc - optind,
        argv.offset(optind as isize),
        target_directory,
        no_target_directory,
        &mut x,
    );
    exit(if ok as libc::c_int != 0 { 0 as libc::c_int } else { 1 as libc::c_int });
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
