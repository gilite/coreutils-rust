#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![feature(extern_types)]
extern "C" {
    pub type hash_table;
    fn free(_: *mut libc::c_void);
    fn xalloc_die();
    fn xmalloc(s: size_t) -> *mut libc::c_void;
    fn xstrdup(str: *const libc::c_char) -> *mut libc::c_char;
    fn hash_lookup(
        table: *const Hash_table,
        entry: *const libc::c_void,
    ) -> *mut libc::c_void;
    fn hash_initialize(
        candidate: size_t,
        tuning: *const Hash_tuning,
        hasher: Hash_hasher,
        comparator: Hash_comparator,
        data_freer: Hash_data_freer,
    ) -> *mut Hash_table;
    fn hash_insert(
        table: *mut Hash_table,
        entry: *const libc::c_void,
    ) -> *mut libc::c_void;
    fn hash_remove(
        table: *mut Hash_table,
        entry: *const libc::c_void,
    ) -> *mut libc::c_void;
}
pub type __uintmax_t = libc::c_ulong;
pub type __dev_t = libc::c_ulong;
pub type __ino_t = libc::c_ulong;
pub type ino_t = __ino_t;
pub type dev_t = __dev_t;
pub type size_t = libc::c_ulong;
pub type uintmax_t = __uintmax_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct hash_tuning {
    pub shrink_threshold: libc::c_float,
    pub shrink_factor: libc::c_float,
    pub growth_threshold: libc::c_float,
    pub growth_factor: libc::c_float,
    pub is_n_buckets: bool,
}
pub type Hash_tuning = hash_tuning;
pub type Hash_table = hash_table;
pub type Hash_hasher = Option::<
    unsafe extern "C" fn(*const libc::c_void, size_t) -> size_t,
>;
pub type Hash_comparator = Option::<
    unsafe extern "C" fn(*const libc::c_void, *const libc::c_void) -> bool,
>;
pub type Hash_data_freer = Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Src_to_dest {
    pub st_ino: ino_t,
    pub st_dev: dev_t,
    pub name: *mut libc::c_char,
}
static mut src_to_dest: *mut Hash_table = 0 as *const Hash_table as *mut Hash_table;
unsafe extern "C" fn src_to_dest_hash(
    mut x: *const libc::c_void,
    mut table_size: size_t,
) -> size_t {
    let mut p: *const Src_to_dest = x as *const Src_to_dest;
    return ((*p).st_ino).wrapping_rem(table_size);
}
unsafe extern "C" fn src_to_dest_compare(
    mut x: *const libc::c_void,
    mut y: *const libc::c_void,
) -> bool {
    let mut a: *const Src_to_dest = x as *const Src_to_dest;
    let mut b: *const Src_to_dest = y as *const Src_to_dest;
    return if (*a).st_ino == (*b).st_ino && (*a).st_dev == (*b).st_dev {
        1 as libc::c_int
    } else {
        0 as libc::c_int
    } != 0;
}
unsafe extern "C" fn src_to_dest_free(mut x: *mut libc::c_void) {
    let mut a: *mut Src_to_dest = x as *mut Src_to_dest;
    free((*a).name as *mut libc::c_void);
    free(x);
}
#[no_mangle]
pub unsafe extern "C" fn forget_created(mut ino: ino_t, mut dev: dev_t) {
    let mut probe: Src_to_dest = Src_to_dest {
        st_ino: 0,
        st_dev: 0,
        name: 0 as *mut libc::c_char,
    };
    let mut ent: *mut Src_to_dest = 0 as *mut Src_to_dest;
    probe.st_ino = ino;
    probe.st_dev = dev;
    probe.name = 0 as *mut libc::c_char;
    ent = hash_remove(src_to_dest, &mut probe as *mut Src_to_dest as *const libc::c_void)
        as *mut Src_to_dest;
    if !ent.is_null() {
        src_to_dest_free(ent as *mut libc::c_void);
    }
}
#[no_mangle]
pub unsafe extern "C" fn src_to_dest_lookup(
    mut ino: ino_t,
    mut dev: dev_t,
) -> *mut libc::c_char {
    let mut ent: Src_to_dest = Src_to_dest {
        st_ino: 0,
        st_dev: 0,
        name: 0 as *mut libc::c_char,
    };
    let mut e: *const Src_to_dest = 0 as *const Src_to_dest;
    ent.st_ino = ino;
    ent.st_dev = dev;
    e = hash_lookup(src_to_dest, &mut ent as *mut Src_to_dest as *const libc::c_void)
        as *const Src_to_dest;
    return if !e.is_null() { (*e).name } else { 0 as *mut libc::c_char };
}
#[no_mangle]
pub unsafe extern "C" fn remember_copied(
    mut name: *const libc::c_char,
    mut ino: ino_t,
    mut dev: dev_t,
) -> *mut libc::c_char {
    let mut ent: *mut Src_to_dest = 0 as *mut Src_to_dest;
    let mut ent_from_table: *mut Src_to_dest = 0 as *mut Src_to_dest;
    ent = xmalloc(::core::mem::size_of::<Src_to_dest>() as libc::c_ulong)
        as *mut Src_to_dest;
    (*ent).name = xstrdup(name);
    (*ent).st_ino = ino;
    (*ent).st_dev = dev;
    ent_from_table = hash_insert(src_to_dest, ent as *const libc::c_void)
        as *mut Src_to_dest;
    if ent_from_table.is_null() {
        xalloc_die();
    }
    if ent_from_table != ent {
        src_to_dest_free(ent as *mut libc::c_void);
        return (*ent_from_table).name;
    }
    return 0 as *mut libc::c_char;
}
#[no_mangle]
pub unsafe extern "C" fn hash_init() {
    src_to_dest = hash_initialize(
        103 as libc::c_int as size_t,
        0 as *const Hash_tuning,
        Some(
            src_to_dest_hash
                as unsafe extern "C" fn(*const libc::c_void, size_t) -> size_t,
        ),
        Some(
            src_to_dest_compare
                as unsafe extern "C" fn(*const libc::c_void, *const libc::c_void) -> bool,
        ),
        Some(src_to_dest_free as unsafe extern "C" fn(*mut libc::c_void) -> ()),
    );
    if src_to_dest.is_null() {
        xalloc_die();
    }
}
