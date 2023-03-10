#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
extern "C" {
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn mempcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn linkat(
        __fromfd: libc::c_int,
        __from: *const libc::c_char,
        __tofd: libc::c_int,
        __to: *const libc::c_char,
        __flags: libc::c_int,
    ) -> libc::c_int;
    fn symlinkat(
        __from: *const libc::c_char,
        __tofd: libc::c_int,
        __to: *const libc::c_char,
    ) -> libc::c_int;
    fn unlinkat(
        __fd: libc::c_int,
        __name: *const libc::c_char,
        __flag: libc::c_int,
    ) -> libc::c_int;
    fn free(_: *mut libc::c_void);
    fn __errno_location() -> *mut libc::c_int;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn renameat(
        __oldfd: libc::c_int,
        __old: *const libc::c_char,
        __newfd: libc::c_int,
        __new: *const libc::c_char,
    ) -> libc::c_int;
    fn last_component(filename: *const libc::c_char) -> *mut libc::c_char;
    fn try_tempname_len(
        tmpl: *mut libc::c_char,
        suffixlen: libc::c_int,
        args: *mut libc::c_void,
        tryfunc: Option::<
            unsafe extern "C" fn(*mut libc::c_char, *mut libc::c_void) -> libc::c_int,
        >,
        x_suffix_len_0: size_t,
    ) -> libc::c_int;
}
pub type size_t = libc::c_ulong;
pub type ptrdiff_t = libc::c_long;
pub const smallsize: C2RustUnnamed_0 = 256;
pub const x_suffix_len: C2RustUnnamed = 6;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct link_arg {
    pub srcdir: libc::c_int,
    pub srcname: *const libc::c_char,
    pub dstdir: libc::c_int,
    pub flags: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct symlink_arg {
    pub srcname: *const libc::c_char,
    pub dstdir: libc::c_int,
}
pub type C2RustUnnamed = libc::c_uint;
pub type C2RustUnnamed_0 = libc::c_uint;
static mut simple_pattern: [libc::c_char; 9] = unsafe {
    *::core::mem::transmute::<&[u8; 9], &[libc::c_char; 9]>(b"CuXXXXXX\0")
};
unsafe extern "C" fn samedir_template(
    mut dstname: *const libc::c_char,
    mut buf: *mut libc::c_char,
) -> *mut libc::c_char {
    let mut dstdirlen: ptrdiff_t = (last_component(dstname)).offset_from(dstname)
        as libc::c_long;
    let mut dsttmpsize: size_t = (dstdirlen as libc::c_ulong)
        .wrapping_add(::core::mem::size_of::<[libc::c_char; 9]>() as libc::c_ulong);
    let mut dsttmp: *mut libc::c_char = 0 as *mut libc::c_char;
    if dsttmpsize <= smallsize as libc::c_int as libc::c_ulong {
        dsttmp = buf;
    } else {
        dsttmp = malloc(dsttmpsize) as *mut libc::c_char;
        if dsttmp.is_null() {
            return dsttmp;
        }
    }
    strcpy(
        mempcpy(
            dsttmp as *mut libc::c_void,
            dstname as *const libc::c_void,
            dstdirlen as libc::c_ulong,
        ) as *mut libc::c_char,
        simple_pattern.as_ptr(),
    );
    return dsttmp;
}
unsafe extern "C" fn try_link(
    mut dest: *mut libc::c_char,
    mut arg: *mut libc::c_void,
) -> libc::c_int {
    let mut a: *mut link_arg = arg as *mut link_arg;
    return linkat((*a).srcdir, (*a).srcname, (*a).dstdir, dest, (*a).flags);
}
#[no_mangle]
pub unsafe extern "C" fn force_linkat(
    mut srcdir: libc::c_int,
    mut srcname: *const libc::c_char,
    mut dstdir: libc::c_int,
    mut dstname: *const libc::c_char,
    mut flags: libc::c_int,
    mut force: bool,
    mut linkat_errno: libc::c_int,
) -> libc::c_int {
    if linkat_errno < 0 as libc::c_int {
        linkat_errno = if linkat(srcdir, srcname, dstdir, dstname, flags)
            == 0 as libc::c_int
        {
            0 as libc::c_int
        } else {
            *__errno_location()
        };
    }
    if !force || linkat_errno != 17 as libc::c_int {
        return linkat_errno;
    }
    let mut buf: [libc::c_char; 256] = [0; 256];
    let mut dsttmp: *mut libc::c_char = samedir_template(dstname, buf.as_mut_ptr());
    if dsttmp.is_null() {
        return *__errno_location();
    }
    let mut arg: link_arg = {
        let mut init = link_arg {
            srcdir: srcdir,
            srcname: srcname,
            dstdir: dstdir,
            flags: flags,
        };
        init
    };
    let mut err: libc::c_int = 0;
    if try_tempname_len(
        dsttmp,
        0 as libc::c_int,
        &mut arg as *mut link_arg as *mut libc::c_void,
        Some(
            try_link
                as unsafe extern "C" fn(
                    *mut libc::c_char,
                    *mut libc::c_void,
                ) -> libc::c_int,
        ),
        x_suffix_len as libc::c_int as size_t,
    ) != 0 as libc::c_int
    {
        err = *__errno_location();
    } else {
        err = if renameat(dstdir, dsttmp, dstdir, dstname) == 0 as libc::c_int {
            -(1 as libc::c_int)
        } else {
            *__errno_location()
        };
        unlinkat(dstdir, dsttmp, 0 as libc::c_int);
    }
    if dsttmp != buf.as_mut_ptr() {
        free(dsttmp as *mut libc::c_void);
    }
    return err;
}
unsafe extern "C" fn try_symlink(
    mut dest: *mut libc::c_char,
    mut arg: *mut libc::c_void,
) -> libc::c_int {
    let mut a: *mut symlink_arg = arg as *mut symlink_arg;
    return symlinkat((*a).srcname, (*a).dstdir, dest);
}
#[no_mangle]
pub unsafe extern "C" fn force_symlinkat(
    mut srcname: *const libc::c_char,
    mut dstdir: libc::c_int,
    mut dstname: *const libc::c_char,
    mut force: bool,
    mut symlinkat_errno: libc::c_int,
) -> libc::c_int {
    if symlinkat_errno < 0 as libc::c_int {
        symlinkat_errno = if symlinkat(srcname, dstdir, dstname) == 0 as libc::c_int {
            0 as libc::c_int
        } else {
            *__errno_location()
        };
    }
    if !force || symlinkat_errno != 17 as libc::c_int {
        return symlinkat_errno;
    }
    let mut buf: [libc::c_char; 256] = [0; 256];
    let mut dsttmp: *mut libc::c_char = samedir_template(dstname, buf.as_mut_ptr());
    if dsttmp.is_null() {
        return *__errno_location();
    }
    let mut arg: symlink_arg = {
        let mut init = symlink_arg {
            srcname: srcname,
            dstdir: dstdir,
        };
        init
    };
    let mut err: libc::c_int = 0;
    if try_tempname_len(
        dsttmp,
        0 as libc::c_int,
        &mut arg as *mut symlink_arg as *mut libc::c_void,
        Some(
            try_symlink
                as unsafe extern "C" fn(
                    *mut libc::c_char,
                    *mut libc::c_void,
                ) -> libc::c_int,
        ),
        x_suffix_len as libc::c_int as size_t,
    ) != 0 as libc::c_int
    {
        err = *__errno_location();
    } else if renameat(dstdir, dsttmp, dstdir, dstname) != 0 as libc::c_int {
        err = *__errno_location();
        unlinkat(dstdir, dsttmp, 0 as libc::c_int);
    } else {
        err = -(1 as libc::c_int);
    }
    if dsttmp != buf.as_mut_ptr() {
        free(dsttmp as *mut libc::c_void);
    }
    return err;
}
