#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![feature(extern_types)]
extern "C" {
    pub type __dirstream;
    pub type cycle_check_state;
    pub type hash_table;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn getpwuid(__uid: __uid_t) -> *mut passwd;
    fn getgrgid(__gid: __gid_t) -> *mut group;
    fn close(__fd: libc::c_int) -> libc::c_int;
    fn fchown(__fd: libc::c_int, __owner: __uid_t, __group: __gid_t) -> libc::c_int;
    fn fchownat(
        __fd: libc::c_int,
        __file: *const libc::c_char,
        __owner: __uid_t,
        __group: __gid_t,
        __flag: libc::c_int,
    ) -> libc::c_int;
    fn fstatat(
        __fd: libc::c_int,
        __file: *const libc::c_char,
        __buf: *mut stat,
        __flag: libc::c_int,
    ) -> libc::c_int;
    fn fstat(__fd: libc::c_int, __buf: *mut stat) -> libc::c_int;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn stpcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn free(_: *mut libc::c_void);
    fn __errno_location() -> *mut libc::c_int;
    fn quotearg_n_style_colon(
        n: libc::c_int,
        s: quoting_style,
        arg: *const libc::c_char,
    ) -> *mut libc::c_char;
    fn quotearg_style(s: quoting_style, arg: *const libc::c_char) -> *mut libc::c_char;
    fn quotearg_n_style(
        n: libc::c_int,
        s: quoting_style,
        arg: *const libc::c_char,
    ) -> *mut libc::c_char;
    fn umaxtostr(_: uintmax_t, _: *mut libc::c_char) -> *mut libc::c_char;
    fn imaxtostr(_: intmax_t, _: *mut libc::c_char) -> *mut libc::c_char;
    fn openat(
        __fd: libc::c_int,
        __file: *const libc::c_char,
        __oflag: libc::c_int,
        _: ...
    ) -> libc::c_int;
    fn dcgettext(
        __domainname: *const libc::c_char,
        __msgid: *const libc::c_char,
        __category: libc::c_int,
    ) -> *mut libc::c_char;
    fn xmalloc(s: size_t) -> *mut libc::c_void;
    fn xstrdup(str: *const libc::c_char) -> *mut libc::c_char;
    fn abort() -> !;
    fn error(
        __status: libc::c_int,
        __errnum: libc::c_int,
        __format: *const libc::c_char,
        _: ...
    );
    fn xfts_open(
        _: *const *mut libc::c_char,
        options: libc::c_int,
        _: Option::<
            unsafe extern "C" fn(*mut *const FTSENT, *mut *const FTSENT) -> libc::c_int,
        >,
    ) -> *mut FTS;
    fn rpl_fts_read(_: *mut FTS) -> *mut FTSENT;
    fn rpl_fts_close(_: *mut FTS) -> libc::c_int;
    fn cycle_warning_required(fts: *const FTS, ent: *const FTSENT) -> bool;
    fn rpl_fts_set(_: *mut FTS, _: *mut FTSENT, _: libc::c_int) -> libc::c_int;
}
pub type size_t = libc::c_ulong;
pub type __intmax_t = libc::c_long;
pub type __uintmax_t = libc::c_ulong;
pub type __dev_t = libc::c_ulong;
pub type __uid_t = libc::c_uint;
pub type __gid_t = libc::c_uint;
pub type __ino_t = libc::c_ulong;
pub type __mode_t = libc::c_uint;
pub type __nlink_t = libc::c_uint;
pub type __off_t = libc::c_long;
pub type __time_t = libc::c_long;
pub type __blksize_t = libc::c_int;
pub type __blkcnt_t = libc::c_long;
pub type __syscall_slong_t = libc::c_long;
pub type ptrdiff_t = libc::c_long;
pub type ino_t = __ino_t;
pub type dev_t = __dev_t;
pub type gid_t = __gid_t;
pub type uid_t = __uid_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
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
pub struct group {
    pub gr_name: *mut libc::c_char,
    pub gr_passwd: *mut libc::c_char,
    pub gr_gid: __gid_t,
    pub gr_mem: *mut *mut libc::c_char,
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
pub type DIR = __dirstream;
pub type intmax_t = __intmax_t;
pub type uintmax_t = __uintmax_t;
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
pub struct dev_ino {
    pub st_ino: ino_t,
    pub st_dev: dev_t,
}
pub type Change_status = libc::c_uint;
pub const CH_NO_CHANGE_REQUESTED: Change_status = 4;
pub const CH_FAILED: Change_status = 3;
pub const CH_SUCCEEDED: Change_status = 2;
pub const CH_NOT_APPLIED: Change_status = 1;
pub type Verbosity = libc::c_uint;
pub const V_off: Verbosity = 2;
pub const V_changes_only: Verbosity = 1;
pub const V_high: Verbosity = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Chown_option {
    pub verbosity: Verbosity,
    pub recurse: bool,
    pub root_dev_ino: *mut dev_ino,
    pub affect_symlink_referent: bool,
    pub force_silent: bool,
    pub user_name: *mut libc::c_char,
    pub group_name: *mut libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FTS {
    pub fts_cur: *mut _ftsent,
    pub fts_child: *mut _ftsent,
    pub fts_array: *mut *mut _ftsent,
    pub fts_dev: dev_t,
    pub fts_path: *mut libc::c_char,
    pub fts_rfd: libc::c_int,
    pub fts_cwd_fd: libc::c_int,
    pub fts_pathlen: size_t,
    pub fts_nitems: size_t,
    pub fts_compar: Option::<
        unsafe extern "C" fn(*mut *const _ftsent, *mut *const _ftsent) -> libc::c_int,
    >,
    pub fts_options: libc::c_int,
    pub fts_leaf_optimization_works_ht: *mut hash_table,
    pub fts_cycle: C2RustUnnamed,
    pub fts_fd_ring: I_ring,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct I_ring {
    pub ir_data: [libc::c_int; 4],
    pub ir_default_val: libc::c_int,
    pub ir_front: libc::c_uint,
    pub ir_back: libc::c_uint,
    pub ir_empty: bool,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub ht: *mut hash_table,
    pub state: *mut cycle_check_state,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _ftsent {
    pub fts_cycle: *mut _ftsent,
    pub fts_parent: *mut _ftsent,
    pub fts_link: *mut _ftsent,
    pub fts_dirp: *mut DIR,
    pub fts_number: libc::c_long,
    pub fts_pointer: *mut libc::c_void,
    pub fts_accpath: *mut libc::c_char,
    pub fts_path: *mut libc::c_char,
    pub fts_errno: libc::c_int,
    pub fts_symfd: libc::c_int,
    pub fts_pathlen: size_t,
    pub fts_fts: *mut FTS,
    pub fts_level: ptrdiff_t,
    pub fts_namelen: size_t,
    pub fts_info: libc::c_ushort,
    pub fts_flags: libc::c_ushort,
    pub fts_instr: libc::c_ushort,
    pub fts_statp: [stat; 1],
    pub fts_name: [libc::c_char; 0],
}
pub type FTSENT = _ftsent;
pub const RC_excluded: RCH_status = 3;
pub const RC_inode_changed: RCH_status = 4;
pub const RC_error: RCH_status = 6;
pub const RC_do_ordinary_chown: RCH_status = 5;
pub const RC_ok: RCH_status = 2;
pub type RCH_status = libc::c_uint;
#[inline]
unsafe extern "C" fn chownat(
    mut fd: libc::c_int,
    mut file: *const libc::c_char,
    mut owner: uid_t,
    mut group: gid_t,
) -> libc::c_int {
    return fchownat(fd, file, owner, group, 0 as libc::c_int);
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
#[no_mangle]
pub unsafe extern "C" fn chopt_init(mut chopt: *mut Chown_option) {
    (*chopt).verbosity = V_off;
    (*chopt).root_dev_ino = 0 as *mut dev_ino;
    (*chopt).affect_symlink_referent = 1 as libc::c_int != 0;
    (*chopt).recurse = 0 as libc::c_int != 0;
    (*chopt).force_silent = 0 as libc::c_int != 0;
    (*chopt).user_name = 0 as *mut libc::c_char;
    (*chopt).group_name = 0 as *mut libc::c_char;
}
#[no_mangle]
pub unsafe extern "C" fn chopt_free(mut chopt: *mut Chown_option) {
    free((*chopt).user_name as *mut libc::c_void);
    free((*chopt).group_name as *mut libc::c_void);
}
unsafe extern "C" fn uid_to_str(mut uid: uid_t) -> *mut libc::c_char {
    let mut buf: [libc::c_char; 21] = [0; 21];
    return xstrdup(
        if !((0 as libc::c_int as uid_t) < -(1 as libc::c_int) as uid_t) {
            imaxtostr(uid as intmax_t, buf.as_mut_ptr())
        } else {
            umaxtostr(uid as uintmax_t, buf.as_mut_ptr())
        },
    );
}
unsafe extern "C" fn gid_to_str(mut gid: gid_t) -> *mut libc::c_char {
    let mut buf: [libc::c_char; 21] = [0; 21];
    return xstrdup(
        if !((0 as libc::c_int as gid_t) < -(1 as libc::c_int) as gid_t) {
            imaxtostr(gid as intmax_t, buf.as_mut_ptr())
        } else {
            umaxtostr(gid as uintmax_t, buf.as_mut_ptr())
        },
    );
}
#[no_mangle]
pub unsafe extern "C" fn gid_to_name(mut gid: gid_t) -> *mut libc::c_char {
    let mut grp: *mut group = getgrgid(gid);
    return if !grp.is_null() { xstrdup((*grp).gr_name) } else { gid_to_str(gid) };
}
#[no_mangle]
pub unsafe extern "C" fn uid_to_name(mut uid: uid_t) -> *mut libc::c_char {
    let mut pwd: *mut passwd = getpwuid(uid);
    return if !pwd.is_null() { xstrdup((*pwd).pw_name) } else { uid_to_str(uid) };
}
unsafe extern "C" fn user_group_str(
    mut user: *const libc::c_char,
    mut group: *const libc::c_char,
) -> *mut libc::c_char {
    let mut spec: *mut libc::c_char = 0 as *mut libc::c_char;
    if !user.is_null() {
        if !group.is_null() {
            spec = xmalloc(
                (strlen(user))
                    .wrapping_add(1 as libc::c_int as libc::c_ulong)
                    .wrapping_add(strlen(group))
                    .wrapping_add(1 as libc::c_int as libc::c_ulong),
            ) as *mut libc::c_char;
            stpcpy(
                stpcpy(stpcpy(spec, user), b":\0" as *const u8 as *const libc::c_char),
                group,
            );
        } else {
            spec = xstrdup(user);
        }
    } else if !group.is_null() {
        spec = xstrdup(group);
    }
    return spec;
}
unsafe extern "C" fn describe_change(
    mut file: *const libc::c_char,
    mut changed: Change_status,
    mut old_user: *const libc::c_char,
    mut old_group: *const libc::c_char,
    mut user: *const libc::c_char,
    mut group: *const libc::c_char,
) {
    let mut fmt: *const libc::c_char = 0 as *const libc::c_char;
    let mut old_spec: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut spec: *mut libc::c_char = 0 as *mut libc::c_char;
    if changed as libc::c_uint == CH_NOT_APPLIED as libc::c_int as libc::c_uint {
        printf(
            dcgettext(
                0 as *const libc::c_char,
                b"neither symbolic link %s nor referent has been changed\n\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            quotearg_style(shell_escape_always_quoting_style, file),
        );
        return;
    }
    spec = user_group_str(user, group);
    old_spec = user_group_str(
        if !user.is_null() { old_user } else { 0 as *const libc::c_char },
        if !group.is_null() { old_group } else { 0 as *const libc::c_char },
    );
    match changed as libc::c_uint {
        2 => {
            fmt = if !user.is_null() {
                dcgettext(
                    0 as *const libc::c_char,
                    b"changed ownership of %s from %s to %s\n\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                )
            } else if !group.is_null() {
                dcgettext(
                    0 as *const libc::c_char,
                    b"changed group of %s from %s to %s\n\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                )
            } else {
                dcgettext(
                    0 as *const libc::c_char,
                    b"no change to ownership of %s\n\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                )
            };
        }
        3 => {
            if !old_spec.is_null() {
                fmt = if !user.is_null() {
                    dcgettext(
                        0 as *const libc::c_char,
                        b"failed to change ownership of %s from %s to %s\n\0"
                            as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    )
                } else if !group.is_null() {
                    dcgettext(
                        0 as *const libc::c_char,
                        b"failed to change group of %s from %s to %s\n\0" as *const u8
                            as *const libc::c_char,
                        5 as libc::c_int,
                    )
                } else {
                    dcgettext(
                        0 as *const libc::c_char,
                        b"failed to change ownership of %s\n\0" as *const u8
                            as *const libc::c_char,
                        5 as libc::c_int,
                    )
                };
            } else {
                fmt = if !user.is_null() {
                    dcgettext(
                        0 as *const libc::c_char,
                        b"failed to change ownership of %s to %s\n\0" as *const u8
                            as *const libc::c_char,
                        5 as libc::c_int,
                    )
                } else if !group.is_null() {
                    dcgettext(
                        0 as *const libc::c_char,
                        b"failed to change group of %s to %s\n\0" as *const u8
                            as *const libc::c_char,
                        5 as libc::c_int,
                    )
                } else {
                    dcgettext(
                        0 as *const libc::c_char,
                        b"failed to change ownership of %s\n\0" as *const u8
                            as *const libc::c_char,
                        5 as libc::c_int,
                    )
                };
                free(old_spec as *mut libc::c_void);
                old_spec = spec;
                spec = 0 as *mut libc::c_char;
            }
        }
        4 => {
            fmt = if !user.is_null() {
                dcgettext(
                    0 as *const libc::c_char,
                    b"ownership of %s retained as %s\n\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                )
            } else if !group.is_null() {
                dcgettext(
                    0 as *const libc::c_char,
                    b"group of %s retained as %s\n\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                )
            } else {
                dcgettext(
                    0 as *const libc::c_char,
                    b"ownership of %s retained\n\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                )
            };
        }
        _ => {
            abort();
        }
    }
    printf(fmt, quotearg_style(shell_escape_always_quoting_style, file), old_spec, spec);
    free(old_spec as *mut libc::c_void);
    free(spec as *mut libc::c_void);
}
unsafe extern "C" fn restricted_chown(
    mut cwd_fd: libc::c_int,
    mut file: *const libc::c_char,
    mut orig_st: *const stat,
    mut uid: uid_t,
    mut gid: gid_t,
    mut required_uid: uid_t,
    mut required_gid: gid_t,
) -> RCH_status {
    let mut status: RCH_status = RC_ok;
    let mut st: stat = stat {
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
    let mut open_flags: libc::c_int = 0o4000 as libc::c_int | 0o400 as libc::c_int;
    let mut fd: libc::c_int = 0;
    if required_uid == -(1 as libc::c_int) as uid_t
        && required_gid == -(1 as libc::c_int) as gid_t
    {
        return RC_do_ordinary_chown;
    }
    if !((*orig_st).st_mode & 0o170000 as libc::c_int as libc::c_uint
        == 0o100000 as libc::c_int as libc::c_uint)
    {
        if (*orig_st).st_mode & 0o170000 as libc::c_int as libc::c_uint
            == 0o40000 as libc::c_int as libc::c_uint
        {
            open_flags |= 0o40000 as libc::c_int;
        } else {
            return RC_do_ordinary_chown
        }
    }
    fd = openat(cwd_fd, file, 0 as libc::c_int | open_flags);
    if !(0 as libc::c_int <= fd
        || *__errno_location() == 13 as libc::c_int
            && (*orig_st).st_mode & 0o170000 as libc::c_int as libc::c_uint
                == 0o100000 as libc::c_int as libc::c_uint
            && {
                fd = openat(cwd_fd, file, 0o1 as libc::c_int | open_flags);
                0 as libc::c_int <= fd
            })
    {
        return (if *__errno_location() == 13 as libc::c_int {
            RC_do_ordinary_chown as libc::c_int
        } else {
            RC_error as libc::c_int
        }) as RCH_status;
    }
    if fstat(fd, &mut st) != 0 as libc::c_int {
        status = RC_error;
    } else if !((*orig_st).st_ino == st.st_ino && (*orig_st).st_dev == st.st_dev) {
        status = RC_inode_changed;
    } else if (required_uid == -(1 as libc::c_int) as uid_t || required_uid == st.st_uid)
        && (required_gid == -(1 as libc::c_int) as gid_t || required_gid == st.st_gid)
    {
        if fchown(fd, uid, gid) == 0 as libc::c_int {
            status = (if close(fd) == 0 as libc::c_int {
                RC_ok as libc::c_int
            } else {
                RC_error as libc::c_int
            }) as RCH_status;
            return status;
        } else {
            status = RC_error;
        }
    }
    let mut saved_errno: libc::c_int = *__errno_location();
    close(fd);
    *__errno_location() = saved_errno;
    return status;
}
unsafe extern "C" fn change_file_owner(
    mut fts: *mut FTS,
    mut ent: *mut FTSENT,
    mut uid: uid_t,
    mut gid: gid_t,
    mut required_uid: uid_t,
    mut required_gid: gid_t,
    mut chopt: *const Chown_option,
) -> bool {
    let mut file_full_name: *const libc::c_char = (*ent).fts_path;
    let mut file: *const libc::c_char = (*ent).fts_accpath;
    let mut file_stats: *const stat = 0 as *const stat;
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
    let mut ok: bool = 1 as libc::c_int != 0;
    let mut do_chown: bool = false;
    let mut symlink_changed: bool = 1 as libc::c_int != 0;
    match (*ent).fts_info as libc::c_int {
        1 => {
            if (*chopt).recurse {
                if !((*chopt).root_dev_ino).is_null()
                    && ((*((*ent).fts_statp).as_mut_ptr()).st_ino
                        == (*(*chopt).root_dev_ino).st_ino
                        && (*((*ent).fts_statp).as_mut_ptr()).st_dev
                            == (*(*chopt).root_dev_ino).st_dev)
                {
                    if strcmp(file_full_name, b"/\0" as *const u8 as *const libc::c_char)
                        == 0 as libc::c_int
                    {
                        error(
                            0 as libc::c_int,
                            0 as libc::c_int,
                            dcgettext(
                                0 as *const libc::c_char,
                                b"it is dangerous to operate recursively on %s\0"
                                    as *const u8 as *const libc::c_char,
                                5 as libc::c_int,
                            ),
                            quotearg_style(
                                shell_escape_always_quoting_style,
                                file_full_name,
                            ),
                        );
                    } else {
                        error(
                            0 as libc::c_int,
                            0 as libc::c_int,
                            dcgettext(
                                0 as *const libc::c_char,
                                b"it is dangerous to operate recursively on %s (same as %s)\0"
                                    as *const u8 as *const libc::c_char,
                                5 as libc::c_int,
                            ),
                            quotearg_n_style(
                                0 as libc::c_int,
                                shell_escape_always_quoting_style,
                                file_full_name,
                            ),
                            quotearg_n_style(
                                1 as libc::c_int,
                                shell_escape_always_quoting_style,
                                b"/\0" as *const u8 as *const libc::c_char,
                            ),
                        );
                    }
                    error(
                        0 as libc::c_int,
                        0 as libc::c_int,
                        dcgettext(
                            0 as *const libc::c_char,
                            b"use --no-preserve-root to override this failsafe\0"
                                as *const u8 as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                    );
                    rpl_fts_set(fts, ent, 4 as libc::c_int);
                    rpl_fts_read(fts);
                    return 0 as libc::c_int != 0;
                }
                return 1 as libc::c_int != 0;
            }
        }
        6 => {
            if !(*chopt).recurse {
                return 1 as libc::c_int != 0;
            }
        }
        10 => {
            if (*ent).fts_level == 0 as libc::c_int as libc::c_long
                && (*ent).fts_number == 0 as libc::c_int as libc::c_long
            {
                (*ent).fts_number = 1 as libc::c_int as libc::c_long;
                rpl_fts_set(fts, ent, 1 as libc::c_int);
                return 1 as libc::c_int != 0;
            }
            if !(*chopt).force_silent {
                error(
                    0 as libc::c_int,
                    (*ent).fts_errno,
                    dcgettext(
                        0 as *const libc::c_char,
                        b"cannot access %s\0" as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    quotearg_style(shell_escape_always_quoting_style, file_full_name),
                );
            }
            ok = 0 as libc::c_int != 0;
        }
        7 => {
            if !(*chopt).force_silent {
                error(
                    0 as libc::c_int,
                    (*ent).fts_errno,
                    b"%s\0" as *const u8 as *const libc::c_char,
                    quotearg_n_style_colon(
                        0 as libc::c_int,
                        shell_escape_quoting_style,
                        file_full_name,
                    ),
                );
            }
            ok = 0 as libc::c_int != 0;
        }
        4 => {
            if !(*chopt).force_silent {
                error(
                    0 as libc::c_int,
                    (*ent).fts_errno,
                    dcgettext(
                        0 as *const libc::c_char,
                        b"cannot read directory %s\0" as *const u8
                            as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    quotearg_style(shell_escape_always_quoting_style, file_full_name),
                );
            }
            ok = 0 as libc::c_int != 0;
        }
        2 => {
            if cycle_warning_required(fts, ent) {
                error(
                    0 as libc::c_int,
                    0 as libc::c_int,
                    dcgettext(
                        0 as *const libc::c_char,
                        b"WARNING: Circular directory structure.\nThis almost certainly means that you have a corrupted file system.\nNOTIFY YOUR SYSTEM MANAGER.\nThe following directory is part of the cycle:\n  %s\n\0"
                            as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    quotearg_n_style_colon(
                        0 as libc::c_int,
                        shell_escape_quoting_style,
                        file_full_name,
                    ),
                );
                return 0 as libc::c_int != 0;
            }
        }
        _ => {}
    }
    if !ok {
        do_chown = 0 as libc::c_int != 0;
        file_stats = 0 as *const stat;
    } else if required_uid == -(1 as libc::c_int) as uid_t
        && required_gid == -(1 as libc::c_int) as gid_t
        && (*chopt).verbosity as libc::c_uint == V_off as libc::c_int as libc::c_uint
        && ((*chopt).root_dev_ino).is_null() && !(*chopt).affect_symlink_referent
    {
        do_chown = 1 as libc::c_int != 0;
        file_stats = ((*ent).fts_statp).as_mut_ptr();
    } else {
        file_stats = ((*ent).fts_statp).as_mut_ptr();
        if (*chopt).affect_symlink_referent as libc::c_int != 0
            && (*file_stats).st_mode & 0o170000 as libc::c_int as libc::c_uint
                == 0o120000 as libc::c_int as libc::c_uint
        {
            if fstatat((*fts).fts_cwd_fd, file, &mut stat_buf, 0 as libc::c_int)
                != 0 as libc::c_int
            {
                if !(*chopt).force_silent {
                    error(
                        0 as libc::c_int,
                        *__errno_location(),
                        dcgettext(
                            0 as *const libc::c_char,
                            b"cannot dereference %s\0" as *const u8
                                as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                        quotearg_style(shell_escape_always_quoting_style, file_full_name),
                    );
                }
                ok = 0 as libc::c_int != 0;
            }
            file_stats = &mut stat_buf;
        }
        do_chown = ok as libc::c_int != 0
            && (required_uid == -(1 as libc::c_int) as uid_t
                || required_uid == (*file_stats).st_uid)
            && (required_gid == -(1 as libc::c_int) as gid_t
                || required_gid == (*file_stats).st_gid);
    }
    if ok as libc::c_int != 0
        && ((*ent).fts_info as libc::c_int == 1 as libc::c_int
            || (*ent).fts_info as libc::c_int == 2 as libc::c_int
            || (*ent).fts_info as libc::c_int == 6 as libc::c_int
            || (*ent).fts_info as libc::c_int == 4 as libc::c_int)
        && (!((*chopt).root_dev_ino).is_null()
            && ((*file_stats).st_ino == (*(*chopt).root_dev_ino).st_ino
                && (*file_stats).st_dev == (*(*chopt).root_dev_ino).st_dev))
    {
        if strcmp(file_full_name, b"/\0" as *const u8 as *const libc::c_char)
            == 0 as libc::c_int
        {
            error(
                0 as libc::c_int,
                0 as libc::c_int,
                dcgettext(
                    0 as *const libc::c_char,
                    b"it is dangerous to operate recursively on %s\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
                quotearg_style(shell_escape_always_quoting_style, file_full_name),
            );
        } else {
            error(
                0 as libc::c_int,
                0 as libc::c_int,
                dcgettext(
                    0 as *const libc::c_char,
                    b"it is dangerous to operate recursively on %s (same as %s)\0"
                        as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
                quotearg_n_style(
                    0 as libc::c_int,
                    shell_escape_always_quoting_style,
                    file_full_name,
                ),
                quotearg_n_style(
                    1 as libc::c_int,
                    shell_escape_always_quoting_style,
                    b"/\0" as *const u8 as *const libc::c_char,
                ),
            );
        }
        error(
            0 as libc::c_int,
            0 as libc::c_int,
            dcgettext(
                0 as *const libc::c_char,
                b"use --no-preserve-root to override this failsafe\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
        return 0 as libc::c_int != 0;
    }
    if do_chown {
        if !(*chopt).affect_symlink_referent {
            ok = lchownat((*fts).fts_cwd_fd, file, uid, gid) == 0 as libc::c_int;
            if !ok && *__errno_location() == 95 as libc::c_int {
                ok = 1 as libc::c_int != 0;
                symlink_changed = 0 as libc::c_int != 0;
            }
        } else {
            let mut err: RCH_status = restricted_chown(
                (*fts).fts_cwd_fd,
                file,
                file_stats,
                uid,
                gid,
                required_uid,
                required_gid,
            );
            match err as libc::c_uint {
                2 => {}
                5 => {
                    ok = chownat((*fts).fts_cwd_fd, file, uid, gid) == 0 as libc::c_int;
                }
                6 => {
                    ok = 0 as libc::c_int != 0;
                }
                4 | 3 => {
                    do_chown = 0 as libc::c_int != 0;
                    ok = 0 as libc::c_int != 0;
                }
                _ => {
                    abort();
                }
            }
        }
        if do_chown as libc::c_int != 0 && !ok && !(*chopt).force_silent {
            error(
                0 as libc::c_int,
                *__errno_location(),
                if uid != -(1 as libc::c_int) as uid_t {
                    dcgettext(
                        0 as *const libc::c_char,
                        b"changing ownership of %s\0" as *const u8
                            as *const libc::c_char,
                        5 as libc::c_int,
                    )
                } else {
                    dcgettext(
                        0 as *const libc::c_char,
                        b"changing group of %s\0" as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    )
                },
                quotearg_style(shell_escape_always_quoting_style, file_full_name),
            );
        }
    }
    if (*chopt).verbosity as libc::c_uint != V_off as libc::c_int as libc::c_uint {
        let mut changed: bool = do_chown as libc::c_int != 0 && ok as libc::c_int != 0
            && symlink_changed as libc::c_int != 0
            && !((uid == -(1 as libc::c_int) as uid_t || uid == (*file_stats).st_uid)
                && (gid == -(1 as libc::c_int) as gid_t || gid == (*file_stats).st_gid));
        if changed as libc::c_int != 0
            || (*chopt).verbosity as libc::c_uint
                == V_high as libc::c_int as libc::c_uint
        {
            let mut ch_status: Change_status = (if !ok {
                CH_FAILED as libc::c_int
            } else if !symlink_changed {
                CH_NOT_APPLIED as libc::c_int
            } else if !changed {
                CH_NO_CHANGE_REQUESTED as libc::c_int
            } else {
                CH_SUCCEEDED as libc::c_int
            }) as Change_status;
            let mut old_usr: *mut libc::c_char = if !file_stats.is_null() {
                uid_to_name((*file_stats).st_uid)
            } else {
                0 as *mut libc::c_char
            };
            let mut old_grp: *mut libc::c_char = if !file_stats.is_null() {
                gid_to_name((*file_stats).st_gid)
            } else {
                0 as *mut libc::c_char
            };
            let mut new_usr: *mut libc::c_char = if !((*chopt).user_name).is_null() {
                (*chopt).user_name
            } else if uid != -(1 as libc::c_int) as libc::c_uint {
                uid_to_str(uid)
            } else {
                0 as *mut libc::c_char
            };
            let mut new_grp: *mut libc::c_char = if !((*chopt).group_name).is_null() {
                (*chopt).group_name
            } else if gid != -(1 as libc::c_int) as libc::c_uint {
                gid_to_str(gid)
            } else {
                0 as *mut libc::c_char
            };
            describe_change(
                file_full_name,
                ch_status,
                old_usr,
                old_grp,
                new_usr,
                new_grp,
            );
            free(old_usr as *mut libc::c_void);
            free(old_grp as *mut libc::c_void);
            if new_usr != (*chopt).user_name {
                free(new_usr as *mut libc::c_void);
            }
            if new_grp != (*chopt).group_name {
                free(new_grp as *mut libc::c_void);
            }
        }
    }
    if !(*chopt).recurse {
        rpl_fts_set(fts, ent, 4 as libc::c_int);
    }
    return ok;
}
#[no_mangle]
pub unsafe extern "C" fn chown_files(
    mut files: *mut *mut libc::c_char,
    mut bit_flags: libc::c_int,
    mut uid: uid_t,
    mut gid: gid_t,
    mut required_uid: uid_t,
    mut required_gid: gid_t,
    mut chopt: *const Chown_option,
) -> bool {
    let mut ok: bool = 1 as libc::c_int != 0;
    let mut stat_flags: libc::c_int = if required_uid != -(1 as libc::c_int) as uid_t
        || required_gid != -(1 as libc::c_int) as gid_t
        || (*chopt).affect_symlink_referent as libc::c_int != 0
        || (*chopt).verbosity as libc::c_uint != V_off as libc::c_int as libc::c_uint
    {
        0 as libc::c_int
    } else {
        0x8 as libc::c_int
    };
    let mut fts: *mut FTS = xfts_open(files, bit_flags | stat_flags, None);
    loop {
        let mut ent: *mut FTSENT = 0 as *mut FTSENT;
        ent = rpl_fts_read(fts);
        if ent.is_null() {
            if *__errno_location() != 0 as libc::c_int {
                if !(*chopt).force_silent {
                    error(
                        0 as libc::c_int,
                        *__errno_location(),
                        dcgettext(
                            0 as *const libc::c_char,
                            b"fts_read failed\0" as *const u8 as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                    );
                }
                ok = 0 as libc::c_int != 0;
            }
            break;
        } else {
            ok = (ok as libc::c_int
                & change_file_owner(
                    fts,
                    ent,
                    uid,
                    gid,
                    required_uid,
                    required_gid,
                    chopt,
                ) as libc::c_int) as bool;
        }
    }
    if rpl_fts_close(fts) != 0 as libc::c_int {
        error(
            0 as libc::c_int,
            *__errno_location(),
            dcgettext(
                0 as *const libc::c_char,
                b"fts_close failed\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
        ok = 0 as libc::c_int != 0;
    }
    return ok;
}
