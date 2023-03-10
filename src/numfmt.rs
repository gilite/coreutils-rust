#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![feature(extern_types)]
use num_traits::ToPrimitive;
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
    static mut stdin: *mut FILE;
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
    fn getdelim(
        __lineptr: *mut *mut libc::c_char,
        __n: *mut size_t,
        __delimiter: libc::c_int,
        __stream: *mut FILE,
    ) -> __ssize_t;
    fn fputs_unlocked(__s: *const libc::c_char, __stream: *mut FILE) -> libc::c_int;
    fn __overflow(_: *mut FILE, _: libc::c_int) -> libc::c_int;
    fn nl_langinfo(__item: nl_item) -> *mut libc::c_char;
    fn strtol(
        _: *const libc::c_char,
        _: *mut *mut libc::c_char,
        _: libc::c_int,
    ) -> libc::c_long;
    fn free(_: *mut libc::c_void);
    fn abort() -> !;
    fn atexit(__func: Option::<unsafe extern "C" fn() -> ()>) -> libc::c_int;
    fn exit(_: libc::c_int) -> !;
    fn mbsalign(
        src: *const libc::c_char,
        dest: *mut libc::c_char,
        dest_size: size_t,
        width: *mut size_t,
        align: mbs_align_t,
        flags: libc::c_int,
    ) -> size_t;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strncat(
        _: *mut libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> *mut libc::c_char;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strncmp(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strspn(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_ulong;
    static mut argmatch_die: argmatch_exit_fn;
    fn stpcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
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
    fn setlocale(
        __category: libc::c_int,
        __locale: *const libc::c_char,
    ) -> *mut libc::c_char;
    fn quote_n(n: libc::c_int, arg: *const libc::c_char) -> *const libc::c_char;
    fn quote(arg: *const libc::c_char) -> *const libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn __xargmatch_internal(
        context: *const libc::c_char,
        arg: *const libc::c_char,
        arglist: *const *const libc::c_char,
        vallist: *const libc::c_void,
        valsize: size_t,
        exit_fn: argmatch_exit_fn,
        allow_abbreviation: bool,
    ) -> ptrdiff_t;
    fn error(
        __status: libc::c_int,
        __errnum: libc::c_int,
        __format: *const libc::c_char,
        _: ...
    );
    fn __errno_location() -> *mut libc::c_int;
    static mut Version: *const libc::c_char;
    static mut exit_failure: libc::c_int;
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    fn xmalloc(s: size_t) -> *mut libc::c_void;
    fn xrealloc(p: *mut libc::c_void, s: size_t) -> *mut libc::c_void;
    fn ximemdup0(p: *const libc::c_void, s: idx_t) -> *mut libc::c_char;
    fn xstrdup(str: *const libc::c_char) -> *mut libc::c_char;
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
    fn xstrtol(
        _: *const libc::c_char,
        _: *mut *mut libc::c_char,
        _: libc::c_int,
        _: *mut libc::c_long,
        _: *const libc::c_char,
    ) -> strtol_error;
    fn xstrtoumax(
        _: *const libc::c_char,
        _: *mut *mut libc::c_char,
        _: libc::c_int,
        _: *mut uintmax_t,
        _: *const libc::c_char,
    ) -> strtol_error;
    static mut frp: *mut field_range_pair;
    static mut n_frp: size_t;
    fn set_fields(fieldstr: *const libc::c_char, options: libc::c_uint);
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
pub type __intmax_t = libc::c_long;
pub type __uintmax_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __ssize_t = libc::c_long;
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
pub type ssize_t = __ssize_t;
pub type ptrdiff_t = libc::c_long;
pub type nl_item = libc::c_int;
pub type C2RustUnnamed = libc::c_uint;
pub const _NL_NUM: C2RustUnnamed = 786449;
pub const _NL_NUM_LC_IDENTIFICATION: C2RustUnnamed = 786448;
pub const _NL_IDENTIFICATION_CODESET: C2RustUnnamed = 786447;
pub const _NL_IDENTIFICATION_CATEGORY: C2RustUnnamed = 786446;
pub const _NL_IDENTIFICATION_DATE: C2RustUnnamed = 786445;
pub const _NL_IDENTIFICATION_REVISION: C2RustUnnamed = 786444;
pub const _NL_IDENTIFICATION_ABBREVIATION: C2RustUnnamed = 786443;
pub const _NL_IDENTIFICATION_APPLICATION: C2RustUnnamed = 786442;
pub const _NL_IDENTIFICATION_AUDIENCE: C2RustUnnamed = 786441;
pub const _NL_IDENTIFICATION_TERRITORY: C2RustUnnamed = 786440;
pub const _NL_IDENTIFICATION_LANGUAGE: C2RustUnnamed = 786439;
pub const _NL_IDENTIFICATION_FAX: C2RustUnnamed = 786438;
pub const _NL_IDENTIFICATION_TEL: C2RustUnnamed = 786437;
pub const _NL_IDENTIFICATION_EMAIL: C2RustUnnamed = 786436;
pub const _NL_IDENTIFICATION_CONTACT: C2RustUnnamed = 786435;
pub const _NL_IDENTIFICATION_ADDRESS: C2RustUnnamed = 786434;
pub const _NL_IDENTIFICATION_SOURCE: C2RustUnnamed = 786433;
pub const _NL_IDENTIFICATION_TITLE: C2RustUnnamed = 786432;
pub const _NL_NUM_LC_MEASUREMENT: C2RustUnnamed = 720898;
pub const _NL_MEASUREMENT_CODESET: C2RustUnnamed = 720897;
pub const _NL_MEASUREMENT_MEASUREMENT: C2RustUnnamed = 720896;
pub const _NL_NUM_LC_TELEPHONE: C2RustUnnamed = 655365;
pub const _NL_TELEPHONE_CODESET: C2RustUnnamed = 655364;
pub const _NL_TELEPHONE_INT_PREFIX: C2RustUnnamed = 655363;
pub const _NL_TELEPHONE_INT_SELECT: C2RustUnnamed = 655362;
pub const _NL_TELEPHONE_TEL_DOM_FMT: C2RustUnnamed = 655361;
pub const _NL_TELEPHONE_TEL_INT_FMT: C2RustUnnamed = 655360;
pub const _NL_NUM_LC_ADDRESS: C2RustUnnamed = 589837;
pub const _NL_ADDRESS_CODESET: C2RustUnnamed = 589836;
pub const _NL_ADDRESS_LANG_LIB: C2RustUnnamed = 589835;
pub const _NL_ADDRESS_LANG_TERM: C2RustUnnamed = 589834;
pub const _NL_ADDRESS_LANG_AB: C2RustUnnamed = 589833;
pub const _NL_ADDRESS_LANG_NAME: C2RustUnnamed = 589832;
pub const _NL_ADDRESS_COUNTRY_ISBN: C2RustUnnamed = 589831;
pub const _NL_ADDRESS_COUNTRY_NUM: C2RustUnnamed = 589830;
pub const _NL_ADDRESS_COUNTRY_CAR: C2RustUnnamed = 589829;
pub const _NL_ADDRESS_COUNTRY_AB3: C2RustUnnamed = 589828;
pub const _NL_ADDRESS_COUNTRY_AB2: C2RustUnnamed = 589827;
pub const _NL_ADDRESS_COUNTRY_POST: C2RustUnnamed = 589826;
pub const _NL_ADDRESS_COUNTRY_NAME: C2RustUnnamed = 589825;
pub const _NL_ADDRESS_POSTAL_FMT: C2RustUnnamed = 589824;
pub const _NL_NUM_LC_NAME: C2RustUnnamed = 524295;
pub const _NL_NAME_CODESET: C2RustUnnamed = 524294;
pub const _NL_NAME_NAME_MS: C2RustUnnamed = 524293;
pub const _NL_NAME_NAME_MISS: C2RustUnnamed = 524292;
pub const _NL_NAME_NAME_MRS: C2RustUnnamed = 524291;
pub const _NL_NAME_NAME_MR: C2RustUnnamed = 524290;
pub const _NL_NAME_NAME_GEN: C2RustUnnamed = 524289;
pub const _NL_NAME_NAME_FMT: C2RustUnnamed = 524288;
pub const _NL_NUM_LC_PAPER: C2RustUnnamed = 458755;
pub const _NL_PAPER_CODESET: C2RustUnnamed = 458754;
pub const _NL_PAPER_WIDTH: C2RustUnnamed = 458753;
pub const _NL_PAPER_HEIGHT: C2RustUnnamed = 458752;
pub const _NL_NUM_LC_MESSAGES: C2RustUnnamed = 327685;
pub const _NL_MESSAGES_CODESET: C2RustUnnamed = 327684;
pub const __NOSTR: C2RustUnnamed = 327683;
pub const __YESSTR: C2RustUnnamed = 327682;
pub const __NOEXPR: C2RustUnnamed = 327681;
pub const __YESEXPR: C2RustUnnamed = 327680;
pub const _NL_NUM_LC_NUMERIC: C2RustUnnamed = 65542;
pub const _NL_NUMERIC_CODESET: C2RustUnnamed = 65541;
pub const _NL_NUMERIC_THOUSANDS_SEP_WC: C2RustUnnamed = 65540;
pub const _NL_NUMERIC_DECIMAL_POINT_WC: C2RustUnnamed = 65539;
pub const __GROUPING: C2RustUnnamed = 65538;
pub const THOUSEP: C2RustUnnamed = 65537;
pub const __THOUSANDS_SEP: C2RustUnnamed = 65537;
pub const RADIXCHAR: C2RustUnnamed = 65536;
pub const __DECIMAL_POINT: C2RustUnnamed = 65536;
pub const _NL_NUM_LC_MONETARY: C2RustUnnamed = 262190;
pub const _NL_MONETARY_CODESET: C2RustUnnamed = 262189;
pub const _NL_MONETARY_THOUSANDS_SEP_WC: C2RustUnnamed = 262188;
pub const _NL_MONETARY_DECIMAL_POINT_WC: C2RustUnnamed = 262187;
pub const _NL_MONETARY_CONVERSION_RATE: C2RustUnnamed = 262186;
pub const _NL_MONETARY_DUO_VALID_TO: C2RustUnnamed = 262185;
pub const _NL_MONETARY_DUO_VALID_FROM: C2RustUnnamed = 262184;
pub const _NL_MONETARY_UNO_VALID_TO: C2RustUnnamed = 262183;
pub const _NL_MONETARY_UNO_VALID_FROM: C2RustUnnamed = 262182;
pub const _NL_MONETARY_DUO_INT_N_SIGN_POSN: C2RustUnnamed = 262181;
pub const _NL_MONETARY_DUO_INT_P_SIGN_POSN: C2RustUnnamed = 262180;
pub const _NL_MONETARY_DUO_N_SIGN_POSN: C2RustUnnamed = 262179;
pub const _NL_MONETARY_DUO_P_SIGN_POSN: C2RustUnnamed = 262178;
pub const _NL_MONETARY_DUO_INT_N_SEP_BY_SPACE: C2RustUnnamed = 262177;
pub const _NL_MONETARY_DUO_INT_N_CS_PRECEDES: C2RustUnnamed = 262176;
pub const _NL_MONETARY_DUO_INT_P_SEP_BY_SPACE: C2RustUnnamed = 262175;
pub const _NL_MONETARY_DUO_INT_P_CS_PRECEDES: C2RustUnnamed = 262174;
pub const _NL_MONETARY_DUO_N_SEP_BY_SPACE: C2RustUnnamed = 262173;
pub const _NL_MONETARY_DUO_N_CS_PRECEDES: C2RustUnnamed = 262172;
pub const _NL_MONETARY_DUO_P_SEP_BY_SPACE: C2RustUnnamed = 262171;
pub const _NL_MONETARY_DUO_P_CS_PRECEDES: C2RustUnnamed = 262170;
pub const _NL_MONETARY_DUO_FRAC_DIGITS: C2RustUnnamed = 262169;
pub const _NL_MONETARY_DUO_INT_FRAC_DIGITS: C2RustUnnamed = 262168;
pub const _NL_MONETARY_DUO_CURRENCY_SYMBOL: C2RustUnnamed = 262167;
pub const _NL_MONETARY_DUO_INT_CURR_SYMBOL: C2RustUnnamed = 262166;
pub const __INT_N_SIGN_POSN: C2RustUnnamed = 262165;
pub const __INT_P_SIGN_POSN: C2RustUnnamed = 262164;
pub const __INT_N_SEP_BY_SPACE: C2RustUnnamed = 262163;
pub const __INT_N_CS_PRECEDES: C2RustUnnamed = 262162;
pub const __INT_P_SEP_BY_SPACE: C2RustUnnamed = 262161;
pub const __INT_P_CS_PRECEDES: C2RustUnnamed = 262160;
pub const _NL_MONETARY_CRNCYSTR: C2RustUnnamed = 262159;
pub const __N_SIGN_POSN: C2RustUnnamed = 262158;
pub const __P_SIGN_POSN: C2RustUnnamed = 262157;
pub const __N_SEP_BY_SPACE: C2RustUnnamed = 262156;
pub const __N_CS_PRECEDES: C2RustUnnamed = 262155;
pub const __P_SEP_BY_SPACE: C2RustUnnamed = 262154;
pub const __P_CS_PRECEDES: C2RustUnnamed = 262153;
pub const __FRAC_DIGITS: C2RustUnnamed = 262152;
pub const __INT_FRAC_DIGITS: C2RustUnnamed = 262151;
pub const __NEGATIVE_SIGN: C2RustUnnamed = 262150;
pub const __POSITIVE_SIGN: C2RustUnnamed = 262149;
pub const __MON_GROUPING: C2RustUnnamed = 262148;
pub const __MON_THOUSANDS_SEP: C2RustUnnamed = 262147;
pub const __MON_DECIMAL_POINT: C2RustUnnamed = 262146;
pub const __CURRENCY_SYMBOL: C2RustUnnamed = 262145;
pub const __INT_CURR_SYMBOL: C2RustUnnamed = 262144;
pub const _NL_NUM_LC_CTYPE: C2RustUnnamed = 86;
pub const _NL_CTYPE_EXTRA_MAP_14: C2RustUnnamed = 85;
pub const _NL_CTYPE_EXTRA_MAP_13: C2RustUnnamed = 84;
pub const _NL_CTYPE_EXTRA_MAP_12: C2RustUnnamed = 83;
pub const _NL_CTYPE_EXTRA_MAP_11: C2RustUnnamed = 82;
pub const _NL_CTYPE_EXTRA_MAP_10: C2RustUnnamed = 81;
pub const _NL_CTYPE_EXTRA_MAP_9: C2RustUnnamed = 80;
pub const _NL_CTYPE_EXTRA_MAP_8: C2RustUnnamed = 79;
pub const _NL_CTYPE_EXTRA_MAP_7: C2RustUnnamed = 78;
pub const _NL_CTYPE_EXTRA_MAP_6: C2RustUnnamed = 77;
pub const _NL_CTYPE_EXTRA_MAP_5: C2RustUnnamed = 76;
pub const _NL_CTYPE_EXTRA_MAP_4: C2RustUnnamed = 75;
pub const _NL_CTYPE_EXTRA_MAP_3: C2RustUnnamed = 74;
pub const _NL_CTYPE_EXTRA_MAP_2: C2RustUnnamed = 73;
pub const _NL_CTYPE_EXTRA_MAP_1: C2RustUnnamed = 72;
pub const _NL_CTYPE_NONASCII_CASE: C2RustUnnamed = 71;
pub const _NL_CTYPE_MAP_TO_NONASCII: C2RustUnnamed = 70;
pub const _NL_CTYPE_TRANSLIT_IGNORE: C2RustUnnamed = 69;
pub const _NL_CTYPE_TRANSLIT_IGNORE_LEN: C2RustUnnamed = 68;
pub const _NL_CTYPE_TRANSLIT_DEFAULT_MISSING: C2RustUnnamed = 67;
pub const _NL_CTYPE_TRANSLIT_DEFAULT_MISSING_LEN: C2RustUnnamed = 66;
pub const _NL_CTYPE_TRANSLIT_TO_TBL: C2RustUnnamed = 65;
pub const _NL_CTYPE_TRANSLIT_TO_IDX: C2RustUnnamed = 64;
pub const _NL_CTYPE_TRANSLIT_FROM_TBL: C2RustUnnamed = 63;
pub const _NL_CTYPE_TRANSLIT_FROM_IDX: C2RustUnnamed = 62;
pub const _NL_CTYPE_TRANSLIT_TAB_SIZE: C2RustUnnamed = 61;
pub const _NL_CTYPE_OUTDIGIT9_WC: C2RustUnnamed = 60;
pub const _NL_CTYPE_OUTDIGIT8_WC: C2RustUnnamed = 59;
pub const _NL_CTYPE_OUTDIGIT7_WC: C2RustUnnamed = 58;
pub const _NL_CTYPE_OUTDIGIT6_WC: C2RustUnnamed = 57;
pub const _NL_CTYPE_OUTDIGIT5_WC: C2RustUnnamed = 56;
pub const _NL_CTYPE_OUTDIGIT4_WC: C2RustUnnamed = 55;
pub const _NL_CTYPE_OUTDIGIT3_WC: C2RustUnnamed = 54;
pub const _NL_CTYPE_OUTDIGIT2_WC: C2RustUnnamed = 53;
pub const _NL_CTYPE_OUTDIGIT1_WC: C2RustUnnamed = 52;
pub const _NL_CTYPE_OUTDIGIT0_WC: C2RustUnnamed = 51;
pub const _NL_CTYPE_OUTDIGIT9_MB: C2RustUnnamed = 50;
pub const _NL_CTYPE_OUTDIGIT8_MB: C2RustUnnamed = 49;
pub const _NL_CTYPE_OUTDIGIT7_MB: C2RustUnnamed = 48;
pub const _NL_CTYPE_OUTDIGIT6_MB: C2RustUnnamed = 47;
pub const _NL_CTYPE_OUTDIGIT5_MB: C2RustUnnamed = 46;
pub const _NL_CTYPE_OUTDIGIT4_MB: C2RustUnnamed = 45;
pub const _NL_CTYPE_OUTDIGIT3_MB: C2RustUnnamed = 44;
pub const _NL_CTYPE_OUTDIGIT2_MB: C2RustUnnamed = 43;
pub const _NL_CTYPE_OUTDIGIT1_MB: C2RustUnnamed = 42;
pub const _NL_CTYPE_OUTDIGIT0_MB: C2RustUnnamed = 41;
pub const _NL_CTYPE_INDIGITS9_WC: C2RustUnnamed = 40;
pub const _NL_CTYPE_INDIGITS8_WC: C2RustUnnamed = 39;
pub const _NL_CTYPE_INDIGITS7_WC: C2RustUnnamed = 38;
pub const _NL_CTYPE_INDIGITS6_WC: C2RustUnnamed = 37;
pub const _NL_CTYPE_INDIGITS5_WC: C2RustUnnamed = 36;
pub const _NL_CTYPE_INDIGITS4_WC: C2RustUnnamed = 35;
pub const _NL_CTYPE_INDIGITS3_WC: C2RustUnnamed = 34;
pub const _NL_CTYPE_INDIGITS2_WC: C2RustUnnamed = 33;
pub const _NL_CTYPE_INDIGITS1_WC: C2RustUnnamed = 32;
pub const _NL_CTYPE_INDIGITS0_WC: C2RustUnnamed = 31;
pub const _NL_CTYPE_INDIGITS_WC_LEN: C2RustUnnamed = 30;
pub const _NL_CTYPE_INDIGITS9_MB: C2RustUnnamed = 29;
pub const _NL_CTYPE_INDIGITS8_MB: C2RustUnnamed = 28;
pub const _NL_CTYPE_INDIGITS7_MB: C2RustUnnamed = 27;
pub const _NL_CTYPE_INDIGITS6_MB: C2RustUnnamed = 26;
pub const _NL_CTYPE_INDIGITS5_MB: C2RustUnnamed = 25;
pub const _NL_CTYPE_INDIGITS4_MB: C2RustUnnamed = 24;
pub const _NL_CTYPE_INDIGITS3_MB: C2RustUnnamed = 23;
pub const _NL_CTYPE_INDIGITS2_MB: C2RustUnnamed = 22;
pub const _NL_CTYPE_INDIGITS1_MB: C2RustUnnamed = 21;
pub const _NL_CTYPE_INDIGITS0_MB: C2RustUnnamed = 20;
pub const _NL_CTYPE_INDIGITS_MB_LEN: C2RustUnnamed = 19;
pub const _NL_CTYPE_MAP_OFFSET: C2RustUnnamed = 18;
pub const _NL_CTYPE_CLASS_OFFSET: C2RustUnnamed = 17;
pub const _NL_CTYPE_TOLOWER32: C2RustUnnamed = 16;
pub const _NL_CTYPE_TOUPPER32: C2RustUnnamed = 15;
pub const CODESET: C2RustUnnamed = 14;
pub const _NL_CTYPE_CODESET_NAME: C2RustUnnamed = 14;
pub const _NL_CTYPE_MB_CUR_MAX: C2RustUnnamed = 13;
pub const _NL_CTYPE_WIDTH: C2RustUnnamed = 12;
pub const _NL_CTYPE_MAP_NAMES: C2RustUnnamed = 11;
pub const _NL_CTYPE_CLASS_NAMES: C2RustUnnamed = 10;
pub const _NL_CTYPE_GAP6: C2RustUnnamed = 9;
pub const _NL_CTYPE_GAP5: C2RustUnnamed = 8;
pub const _NL_CTYPE_GAP4: C2RustUnnamed = 7;
pub const _NL_CTYPE_GAP3: C2RustUnnamed = 6;
pub const _NL_CTYPE_CLASS32: C2RustUnnamed = 5;
pub const _NL_CTYPE_GAP2: C2RustUnnamed = 4;
pub const _NL_CTYPE_TOLOWER: C2RustUnnamed = 3;
pub const _NL_CTYPE_GAP1: C2RustUnnamed = 2;
pub const _NL_CTYPE_TOUPPER: C2RustUnnamed = 1;
pub const _NL_CTYPE_CLASS: C2RustUnnamed = 0;
pub const _NL_NUM_LC_COLLATE: C2RustUnnamed = 196627;
pub const _NL_COLLATE_CODESET: C2RustUnnamed = 196626;
pub const _NL_COLLATE_COLLSEQWC: C2RustUnnamed = 196625;
pub const _NL_COLLATE_COLLSEQMB: C2RustUnnamed = 196624;
pub const _NL_COLLATE_SYMB_EXTRAMB: C2RustUnnamed = 196623;
pub const _NL_COLLATE_SYMB_TABLEMB: C2RustUnnamed = 196622;
pub const _NL_COLLATE_SYMB_HASH_SIZEMB: C2RustUnnamed = 196621;
pub const _NL_COLLATE_INDIRECTWC: C2RustUnnamed = 196620;
pub const _NL_COLLATE_EXTRAWC: C2RustUnnamed = 196619;
pub const _NL_COLLATE_WEIGHTWC: C2RustUnnamed = 196618;
pub const _NL_COLLATE_TABLEWC: C2RustUnnamed = 196617;
pub const _NL_COLLATE_GAP3: C2RustUnnamed = 196616;
pub const _NL_COLLATE_GAP2: C2RustUnnamed = 196615;
pub const _NL_COLLATE_GAP1: C2RustUnnamed = 196614;
pub const _NL_COLLATE_INDIRECTMB: C2RustUnnamed = 196613;
pub const _NL_COLLATE_EXTRAMB: C2RustUnnamed = 196612;
pub const _NL_COLLATE_WEIGHTMB: C2RustUnnamed = 196611;
pub const _NL_COLLATE_TABLEMB: C2RustUnnamed = 196610;
pub const _NL_COLLATE_RULESETS: C2RustUnnamed = 196609;
pub const _NL_COLLATE_NRULES: C2RustUnnamed = 196608;
pub const _NL_NUM_LC_TIME: C2RustUnnamed = 131231;
pub const _NL_WABALTMON_12: C2RustUnnamed = 131230;
pub const _NL_WABALTMON_11: C2RustUnnamed = 131229;
pub const _NL_WABALTMON_10: C2RustUnnamed = 131228;
pub const _NL_WABALTMON_9: C2RustUnnamed = 131227;
pub const _NL_WABALTMON_8: C2RustUnnamed = 131226;
pub const _NL_WABALTMON_7: C2RustUnnamed = 131225;
pub const _NL_WABALTMON_6: C2RustUnnamed = 131224;
pub const _NL_WABALTMON_5: C2RustUnnamed = 131223;
pub const _NL_WABALTMON_4: C2RustUnnamed = 131222;
pub const _NL_WABALTMON_3: C2RustUnnamed = 131221;
pub const _NL_WABALTMON_2: C2RustUnnamed = 131220;
pub const _NL_WABALTMON_1: C2RustUnnamed = 131219;
pub const _NL_ABALTMON_12: C2RustUnnamed = 131218;
pub const _NL_ABALTMON_11: C2RustUnnamed = 131217;
pub const _NL_ABALTMON_10: C2RustUnnamed = 131216;
pub const _NL_ABALTMON_9: C2RustUnnamed = 131215;
pub const _NL_ABALTMON_8: C2RustUnnamed = 131214;
pub const _NL_ABALTMON_7: C2RustUnnamed = 131213;
pub const _NL_ABALTMON_6: C2RustUnnamed = 131212;
pub const _NL_ABALTMON_5: C2RustUnnamed = 131211;
pub const _NL_ABALTMON_4: C2RustUnnamed = 131210;
pub const _NL_ABALTMON_3: C2RustUnnamed = 131209;
pub const _NL_ABALTMON_2: C2RustUnnamed = 131208;
pub const _NL_ABALTMON_1: C2RustUnnamed = 131207;
pub const _NL_WALTMON_12: C2RustUnnamed = 131206;
pub const _NL_WALTMON_11: C2RustUnnamed = 131205;
pub const _NL_WALTMON_10: C2RustUnnamed = 131204;
pub const _NL_WALTMON_9: C2RustUnnamed = 131203;
pub const _NL_WALTMON_8: C2RustUnnamed = 131202;
pub const _NL_WALTMON_7: C2RustUnnamed = 131201;
pub const _NL_WALTMON_6: C2RustUnnamed = 131200;
pub const _NL_WALTMON_5: C2RustUnnamed = 131199;
pub const _NL_WALTMON_4: C2RustUnnamed = 131198;
pub const _NL_WALTMON_3: C2RustUnnamed = 131197;
pub const _NL_WALTMON_2: C2RustUnnamed = 131196;
pub const _NL_WALTMON_1: C2RustUnnamed = 131195;
pub const __ALTMON_12: C2RustUnnamed = 131194;
pub const __ALTMON_11: C2RustUnnamed = 131193;
pub const __ALTMON_10: C2RustUnnamed = 131192;
pub const __ALTMON_9: C2RustUnnamed = 131191;
pub const __ALTMON_8: C2RustUnnamed = 131190;
pub const __ALTMON_7: C2RustUnnamed = 131189;
pub const __ALTMON_6: C2RustUnnamed = 131188;
pub const __ALTMON_5: C2RustUnnamed = 131187;
pub const __ALTMON_4: C2RustUnnamed = 131186;
pub const __ALTMON_3: C2RustUnnamed = 131185;
pub const __ALTMON_2: C2RustUnnamed = 131184;
pub const __ALTMON_1: C2RustUnnamed = 131183;
pub const _NL_TIME_CODESET: C2RustUnnamed = 131182;
pub const _NL_W_DATE_FMT: C2RustUnnamed = 131181;
pub const _DATE_FMT: C2RustUnnamed = 131180;
pub const _NL_TIME_TIMEZONE: C2RustUnnamed = 131179;
pub const _NL_TIME_CAL_DIRECTION: C2RustUnnamed = 131178;
pub const _NL_TIME_FIRST_WORKDAY: C2RustUnnamed = 131177;
pub const _NL_TIME_FIRST_WEEKDAY: C2RustUnnamed = 131176;
pub const _NL_TIME_WEEK_1STWEEK: C2RustUnnamed = 131175;
pub const _NL_TIME_WEEK_1STDAY: C2RustUnnamed = 131174;
pub const _NL_TIME_WEEK_NDAYS: C2RustUnnamed = 131173;
pub const _NL_WERA_T_FMT: C2RustUnnamed = 131172;
pub const _NL_WERA_D_T_FMT: C2RustUnnamed = 131171;
pub const _NL_WALT_DIGITS: C2RustUnnamed = 131170;
pub const _NL_WERA_D_FMT: C2RustUnnamed = 131169;
pub const _NL_WERA_YEAR: C2RustUnnamed = 131168;
pub const _NL_WT_FMT_AMPM: C2RustUnnamed = 131167;
pub const _NL_WT_FMT: C2RustUnnamed = 131166;
pub const _NL_WD_FMT: C2RustUnnamed = 131165;
pub const _NL_WD_T_FMT: C2RustUnnamed = 131164;
pub const _NL_WPM_STR: C2RustUnnamed = 131163;
pub const _NL_WAM_STR: C2RustUnnamed = 131162;
pub const _NL_WMON_12: C2RustUnnamed = 131161;
pub const _NL_WMON_11: C2RustUnnamed = 131160;
pub const _NL_WMON_10: C2RustUnnamed = 131159;
pub const _NL_WMON_9: C2RustUnnamed = 131158;
pub const _NL_WMON_8: C2RustUnnamed = 131157;
pub const _NL_WMON_7: C2RustUnnamed = 131156;
pub const _NL_WMON_6: C2RustUnnamed = 131155;
pub const _NL_WMON_5: C2RustUnnamed = 131154;
pub const _NL_WMON_4: C2RustUnnamed = 131153;
pub const _NL_WMON_3: C2RustUnnamed = 131152;
pub const _NL_WMON_2: C2RustUnnamed = 131151;
pub const _NL_WMON_1: C2RustUnnamed = 131150;
pub const _NL_WABMON_12: C2RustUnnamed = 131149;
pub const _NL_WABMON_11: C2RustUnnamed = 131148;
pub const _NL_WABMON_10: C2RustUnnamed = 131147;
pub const _NL_WABMON_9: C2RustUnnamed = 131146;
pub const _NL_WABMON_8: C2RustUnnamed = 131145;
pub const _NL_WABMON_7: C2RustUnnamed = 131144;
pub const _NL_WABMON_6: C2RustUnnamed = 131143;
pub const _NL_WABMON_5: C2RustUnnamed = 131142;
pub const _NL_WABMON_4: C2RustUnnamed = 131141;
pub const _NL_WABMON_3: C2RustUnnamed = 131140;
pub const _NL_WABMON_2: C2RustUnnamed = 131139;
pub const _NL_WABMON_1: C2RustUnnamed = 131138;
pub const _NL_WDAY_7: C2RustUnnamed = 131137;
pub const _NL_WDAY_6: C2RustUnnamed = 131136;
pub const _NL_WDAY_5: C2RustUnnamed = 131135;
pub const _NL_WDAY_4: C2RustUnnamed = 131134;
pub const _NL_WDAY_3: C2RustUnnamed = 131133;
pub const _NL_WDAY_2: C2RustUnnamed = 131132;
pub const _NL_WDAY_1: C2RustUnnamed = 131131;
pub const _NL_WABDAY_7: C2RustUnnamed = 131130;
pub const _NL_WABDAY_6: C2RustUnnamed = 131129;
pub const _NL_WABDAY_5: C2RustUnnamed = 131128;
pub const _NL_WABDAY_4: C2RustUnnamed = 131127;
pub const _NL_WABDAY_3: C2RustUnnamed = 131126;
pub const _NL_WABDAY_2: C2RustUnnamed = 131125;
pub const _NL_WABDAY_1: C2RustUnnamed = 131124;
pub const _NL_TIME_ERA_ENTRIES: C2RustUnnamed = 131123;
pub const _NL_TIME_ERA_NUM_ENTRIES: C2RustUnnamed = 131122;
pub const ERA_T_FMT: C2RustUnnamed = 131121;
pub const ERA_D_T_FMT: C2RustUnnamed = 131120;
pub const ALT_DIGITS: C2RustUnnamed = 131119;
pub const ERA_D_FMT: C2RustUnnamed = 131118;
pub const __ERA_YEAR: C2RustUnnamed = 131117;
pub const ERA: C2RustUnnamed = 131116;
pub const T_FMT_AMPM: C2RustUnnamed = 131115;
pub const T_FMT: C2RustUnnamed = 131114;
pub const D_FMT: C2RustUnnamed = 131113;
pub const D_T_FMT: C2RustUnnamed = 131112;
pub const PM_STR: C2RustUnnamed = 131111;
pub const AM_STR: C2RustUnnamed = 131110;
pub const MON_12: C2RustUnnamed = 131109;
pub const MON_11: C2RustUnnamed = 131108;
pub const MON_10: C2RustUnnamed = 131107;
pub const MON_9: C2RustUnnamed = 131106;
pub const MON_8: C2RustUnnamed = 131105;
pub const MON_7: C2RustUnnamed = 131104;
pub const MON_6: C2RustUnnamed = 131103;
pub const MON_5: C2RustUnnamed = 131102;
pub const MON_4: C2RustUnnamed = 131101;
pub const MON_3: C2RustUnnamed = 131100;
pub const MON_2: C2RustUnnamed = 131099;
pub const MON_1: C2RustUnnamed = 131098;
pub const ABMON_12: C2RustUnnamed = 131097;
pub const ABMON_11: C2RustUnnamed = 131096;
pub const ABMON_10: C2RustUnnamed = 131095;
pub const ABMON_9: C2RustUnnamed = 131094;
pub const ABMON_8: C2RustUnnamed = 131093;
pub const ABMON_7: C2RustUnnamed = 131092;
pub const ABMON_6: C2RustUnnamed = 131091;
pub const ABMON_5: C2RustUnnamed = 131090;
pub const ABMON_4: C2RustUnnamed = 131089;
pub const ABMON_3: C2RustUnnamed = 131088;
pub const ABMON_2: C2RustUnnamed = 131087;
pub const ABMON_1: C2RustUnnamed = 131086;
pub const DAY_7: C2RustUnnamed = 131085;
pub const DAY_6: C2RustUnnamed = 131084;
pub const DAY_5: C2RustUnnamed = 131083;
pub const DAY_4: C2RustUnnamed = 131082;
pub const DAY_3: C2RustUnnamed = 131081;
pub const DAY_2: C2RustUnnamed = 131080;
pub const DAY_1: C2RustUnnamed = 131079;
pub const ABDAY_7: C2RustUnnamed = 131078;
pub const ABDAY_6: C2RustUnnamed = 131077;
pub const ABDAY_5: C2RustUnnamed = 131076;
pub const ABDAY_4: C2RustUnnamed = 131075;
pub const ABDAY_3: C2RustUnnamed = 131074;
pub const ABDAY_2: C2RustUnnamed = 131073;
pub const ABDAY_1: C2RustUnnamed = 131072;
pub type mbs_align_t = libc::c_uint;
pub const MBS_ALIGN_CENTER: mbs_align_t = 2;
pub const MBS_ALIGN_RIGHT: mbs_align_t = 1;
pub const MBS_ALIGN_LEFT: mbs_align_t = 0;
pub type C2RustUnnamed_0 = libc::c_uint;
pub const MBA_NO_RIGHT_PAD: C2RustUnnamed_0 = 8;
pub const MBA_NO_LEFT_PAD: C2RustUnnamed_0 = 4;
pub const MBA_UNIBYTE_ONLY: C2RustUnnamed_0 = 2;
pub const MBA_UNIBYTE_FALLBACK: C2RustUnnamed_0 = 1;
pub type argmatch_exit_fn = Option::<unsafe extern "C" fn() -> ()>;
pub type intmax_t = __intmax_t;
pub type uintmax_t = __uintmax_t;
pub type C2RustUnnamed_1 = libc::c_uint;
pub const _ISalnum: C2RustUnnamed_1 = 8;
pub const _ISpunct: C2RustUnnamed_1 = 4;
pub const _IScntrl: C2RustUnnamed_1 = 2;
pub const _ISblank: C2RustUnnamed_1 = 1;
pub const _ISgraph: C2RustUnnamed_1 = 32768;
pub const _ISprint: C2RustUnnamed_1 = 16384;
pub const _ISspace: C2RustUnnamed_1 = 8192;
pub const _ISxdigit: C2RustUnnamed_1 = 4096;
pub const _ISdigit: C2RustUnnamed_1 = 2048;
pub const _ISalpha: C2RustUnnamed_1 = 1024;
pub const _ISlower: C2RustUnnamed_1 = 512;
pub const _ISupper: C2RustUnnamed_1 = 256;
pub type idx_t = ptrdiff_t;
pub type C2RustUnnamed_2 = libc::c_int;
pub const GETOPT_VERSION_CHAR: C2RustUnnamed_2 = -3;
pub const GETOPT_HELP_CHAR: C2RustUnnamed_2 = -2;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct infomap {
    pub program: *const libc::c_char,
    pub node: *const libc::c_char,
}
pub type strtol_error = libc::c_uint;
pub const LONGINT_INVALID: strtol_error = 4;
pub const LONGINT_INVALID_SUFFIX_CHAR_WITH_OVERFLOW: strtol_error = 3;
pub const LONGINT_INVALID_SUFFIX_CHAR: strtol_error = 2;
pub const LONGINT_OVERFLOW: strtol_error = 1;
pub const LONGINT_OK: strtol_error = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct field_range_pair {
    pub lo: uintmax_t,
    pub hi: uintmax_t,
}
pub type C2RustUnnamed_3 = libc::c_uint;
pub const SETFLD_ERRMSG_USE_POS: C2RustUnnamed_3 = 4;
pub const SETFLD_COMPLEMENT: C2RustUnnamed_3 = 2;
pub const SETFLD_ALLOW_DASH: C2RustUnnamed_3 = 1;
pub type C2RustUnnamed_4 = libc::c_uint;
pub const EXIT_CONVERSION_WARNINGS: C2RustUnnamed_4 = 2;
pub const TIMEOUT_FAILURE: C2RustUnnamed_4 = 1;
pub type C2RustUnnamed_5 = libc::c_uint;
pub const INVALID_OPTION: C2RustUnnamed_5 = 269;
pub const FORMAT_OPTION: C2RustUnnamed_5 = 268;
pub const HEADER_OPTION: C2RustUnnamed_5 = 267;
pub const DEV_DEBUG_OPTION: C2RustUnnamed_5 = 266;
pub const DEBUG_OPTION: C2RustUnnamed_5 = 265;
pub const FIELD_OPTION: C2RustUnnamed_5 = 264;
pub const PADDING_OPTION: C2RustUnnamed_5 = 263;
pub const GROUPING_OPTION: C2RustUnnamed_5 = 262;
pub const SUFFIX_OPTION: C2RustUnnamed_5 = 261;
pub const ROUND_OPTION: C2RustUnnamed_5 = 260;
pub const TO_UNIT_OPTION: C2RustUnnamed_5 = 259;
pub const TO_OPTION: C2RustUnnamed_5 = 258;
pub const FROM_UNIT_OPTION: C2RustUnnamed_5 = 257;
pub const FROM_OPTION: C2RustUnnamed_5 = 256;
pub type scale_type = libc::c_uint;
pub const scale_IEC_I: scale_type = 4;
pub const scale_IEC: scale_type = 3;
pub const scale_SI: scale_type = 2;
pub const scale_auto: scale_type = 1;
pub const scale_none: scale_type = 0;
pub type round_type = libc::c_uint;
pub const round_nearest: round_type = 4;
pub const round_to_zero: round_type = 3;
pub const round_from_zero: round_type = 2;
pub const round_floor: round_type = 1;
pub const round_ceiling: round_type = 0;
pub type inval_type = libc::c_uint;
pub const inval_ignore: inval_type = 3;
pub const inval_warn: inval_type = 2;
pub const inval_fail: inval_type = 1;
pub const inval_abort: inval_type = 0;
pub type C2RustUnnamed_6 = libc::c_uint;
pub const DELIMITER_DEFAULT: C2RustUnnamed_6 = 256;
pub type C2RustUnnamed_7 = libc::c_uint;
pub const MAX_UNSCALED_DIGITS: C2RustUnnamed_7 = 33;
pub type C2RustUnnamed_8 = libc::c_uint;
pub const MAX_ACCEPTABLE_DIGITS: C2RustUnnamed_8 = 33;
pub type simple_strtod_error = libc::c_uint;
pub const SSE_MISSING_I_SUFFIX: simple_strtod_error = 6;
pub const SSE_INVALID_SUFFIX: simple_strtod_error = 5;
pub const SSE_VALID_BUT_FORBIDDEN_SUFFIX: simple_strtod_error = 4;
pub const SSE_INVALID_NUMBER: simple_strtod_error = 3;
pub const SSE_OVERFLOW: simple_strtod_error = 2;
pub const SSE_OK_PRECISION_LOSS: simple_strtod_error = 1;
pub const SSE_OK: simple_strtod_error = 0;
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
#[inline]
unsafe extern "C" fn fputc_unlocked(
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
unsafe extern "C" fn ferror_unlocked(mut __stream: *mut FILE) -> libc::c_int {
    return ((*__stream)._flags & 0x20 as libc::c_int != 0 as libc::c_int) as libc::c_int;
}
#[inline]
unsafe extern "C" fn c_isdigit(mut c: libc::c_int) -> bool {
    match c {
        48 | 49 | 50 | 51 | 52 | 53 | 54 | 55 | 56 | 57 => return 1 as libc::c_int != 0,
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
unsafe extern "C" fn to_uchar(mut ch: libc::c_char) -> libc::c_uchar {
    return ch as libc::c_uchar;
}
#[inline]
unsafe extern "C" fn field_sep(mut ch: libc::c_uchar) -> bool {
    return *(*__ctype_b_loc()).offset(ch as libc::c_int as isize) as libc::c_int
        & _ISblank as libc::c_int as libc::c_ushort as libc::c_int != 0
        || ch as libc::c_int == '\n' as i32;
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
static mut scale_from_args: [*const libc::c_char; 6] = [
    b"none\0" as *const u8 as *const libc::c_char,
    b"auto\0" as *const u8 as *const libc::c_char,
    b"si\0" as *const u8 as *const libc::c_char,
    b"iec\0" as *const u8 as *const libc::c_char,
    b"iec-i\0" as *const u8 as *const libc::c_char,
    0 as *const libc::c_char,
];
static mut scale_from_types: [scale_type; 5] = [
    scale_none,
    scale_auto,
    scale_SI,
    scale_IEC,
    scale_IEC_I,
];
static mut scale_to_args: [*const libc::c_char; 5] = [
    b"none\0" as *const u8 as *const libc::c_char,
    b"si\0" as *const u8 as *const libc::c_char,
    b"iec\0" as *const u8 as *const libc::c_char,
    b"iec-i\0" as *const u8 as *const libc::c_char,
    0 as *const libc::c_char,
];
static mut scale_to_types: [scale_type; 4] = [
    scale_none,
    scale_SI,
    scale_IEC,
    scale_IEC_I,
];
static mut round_args: [*const libc::c_char; 6] = [
    b"up\0" as *const u8 as *const libc::c_char,
    b"down\0" as *const u8 as *const libc::c_char,
    b"from-zero\0" as *const u8 as *const libc::c_char,
    b"towards-zero\0" as *const u8 as *const libc::c_char,
    b"nearest\0" as *const u8 as *const libc::c_char,
    0 as *const libc::c_char,
];
static mut round_types: [round_type; 5] = [
    round_ceiling,
    round_floor,
    round_from_zero,
    round_to_zero,
    round_nearest,
];
static mut inval_args: [*const libc::c_char; 5] = [
    b"abort\0" as *const u8 as *const libc::c_char,
    b"fail\0" as *const u8 as *const libc::c_char,
    b"warn\0" as *const u8 as *const libc::c_char,
    b"ignore\0" as *const u8 as *const libc::c_char,
    0 as *const libc::c_char,
];
static mut inval_types: [inval_type; 4] = [
    inval_abort,
    inval_fail,
    inval_warn,
    inval_ignore,
];
static mut longopts: [option; 19] = [
    {
        let mut init = option {
            name: b"from\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: FROM_OPTION as libc::c_int,
        };
        init
    },
    {
        let mut init = option {
            name: b"from-unit\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: FROM_UNIT_OPTION as libc::c_int,
        };
        init
    },
    {
        let mut init = option {
            name: b"to\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: TO_OPTION as libc::c_int,
        };
        init
    },
    {
        let mut init = option {
            name: b"to-unit\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: TO_UNIT_OPTION as libc::c_int,
        };
        init
    },
    {
        let mut init = option {
            name: b"round\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: ROUND_OPTION as libc::c_int,
        };
        init
    },
    {
        let mut init = option {
            name: b"padding\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: PADDING_OPTION as libc::c_int,
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
            name: b"grouping\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: GROUPING_OPTION as libc::c_int,
        };
        init
    },
    {
        let mut init = option {
            name: b"delimiter\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'd' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"field\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: FIELD_OPTION as libc::c_int,
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
            name: b"-debug\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: DEV_DEBUG_OPTION as libc::c_int,
        };
        init
    },
    {
        let mut init = option {
            name: b"header\0" as *const u8 as *const libc::c_char,
            has_arg: 2 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: HEADER_OPTION as libc::c_int,
        };
        init
    },
    {
        let mut init = option {
            name: b"format\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: FORMAT_OPTION as libc::c_int,
        };
        init
    },
    {
        let mut init = option {
            name: b"invalid\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: INVALID_OPTION as libc::c_int,
        };
        init
    },
    {
        let mut init = option {
            name: b"zero-terminated\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'z' as i32,
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
static mut scale_from: scale_type = scale_none;
static mut scale_to: scale_type = scale_none;
static mut round_style: round_type = round_from_zero;
static mut inval_style: inval_type = inval_abort;
static mut suffix: *const libc::c_char = 0 as *const libc::c_char;
static mut from_unit_size: uintmax_t = 1 as libc::c_int as uintmax_t;
static mut to_unit_size: uintmax_t = 1 as libc::c_int as uintmax_t;
static mut grouping: libc::c_int = 0 as libc::c_int;
static mut padding_buffer: *mut libc::c_char = 0 as *const libc::c_char
    as *mut libc::c_char;
static mut padding_buffer_size: size_t = 0 as libc::c_int as size_t;
static mut padding_width: libc::c_long = 0 as libc::c_int as libc::c_long;
static mut zero_padding_width: libc::c_long = 0 as libc::c_int as libc::c_long;
static mut user_precision: libc::c_long = -(1 as libc::c_int) as libc::c_long;
static mut format_str: *const libc::c_char = 0 as *const libc::c_char;
static mut format_str_prefix: *mut libc::c_char = 0 as *const libc::c_char
    as *mut libc::c_char;
static mut format_str_suffix: *mut libc::c_char = 0 as *const libc::c_char
    as *mut libc::c_char;
static mut conv_exit_code: libc::c_int = EXIT_CONVERSION_WARNINGS as libc::c_int;
static mut auto_padding: libc::c_int = 0 as libc::c_int;
static mut padding_alignment: mbs_align_t = MBS_ALIGN_RIGHT;
static mut delimiter: libc::c_int = DELIMITER_DEFAULT as libc::c_int;
static mut line_delim: libc::c_uchar = '\n' as i32 as libc::c_uchar;
static mut header: uintmax_t = 0 as libc::c_int as uintmax_t;
static mut debug: bool = false;
static mut decimal_point: *const libc::c_char = 0 as *const libc::c_char;
static mut decimal_point_length: libc::c_int = 0;
static mut dev_debug: bool = 0 as libc::c_int != 0;
#[inline]
unsafe extern "C" fn default_scale_base(mut scale: scale_type) -> libc::c_int {
    match scale as libc::c_uint {
        3 | 4 => return 1024 as libc::c_int,
        0 | 1 | 2 | _ => return 1000 as libc::c_int,
    };
}
static mut zero_and_valid_suffixes: [libc::c_char; 12] = unsafe {
    *::core::mem::transmute::<&[u8; 12], &[libc::c_char; 12]>(b"0KMGTPEZYRQ\0")
};
static mut valid_suffixes: *const libc::c_char = 0 as *const libc::c_char;
#[inline]
unsafe extern "C" fn valid_suffix(suf: libc::c_char) -> bool {
    return !(strchr(valid_suffixes, suf as libc::c_int)).is_null();
}
#[inline]
unsafe extern "C" fn suffix_power(suf: libc::c_char) -> libc::c_int {
    match suf as libc::c_int {
        75 => return 1 as libc::c_int,
        77 => return 2 as libc::c_int,
        71 => return 3 as libc::c_int,
        84 => return 4 as libc::c_int,
        80 => return 5 as libc::c_int,
        69 => return 6 as libc::c_int,
        90 => return 7 as libc::c_int,
        89 => return 8 as libc::c_int,
        82 => return 9 as libc::c_int,
        81 => return 10 as libc::c_int,
        _ => return 0 as libc::c_int,
    };
}
#[inline]
unsafe extern "C" fn suffix_power_char(mut power: libc::c_uint) -> *const libc::c_char {
    match power {
        0 => return b"\0" as *const u8 as *const libc::c_char,
        1 => return b"K\0" as *const u8 as *const libc::c_char,
        2 => return b"M\0" as *const u8 as *const libc::c_char,
        3 => return b"G\0" as *const u8 as *const libc::c_char,
        4 => return b"T\0" as *const u8 as *const libc::c_char,
        5 => return b"P\0" as *const u8 as *const libc::c_char,
        6 => return b"E\0" as *const u8 as *const libc::c_char,
        7 => return b"Z\0" as *const u8 as *const libc::c_char,
        8 => return b"Y\0" as *const u8 as *const libc::c_char,
        9 => return b"R\0" as *const u8 as *const libc::c_char,
        10 => return b"Q\0" as *const u8 as *const libc::c_char,
        _ => return b"(error)\0" as *const u8 as *const libc::c_char,
    };
}
unsafe extern "C" fn powerld(mut base: f128::f128, mut x: libc::c_uint) -> f128::f128 {
    let mut result: f128::f128 = base;
    if x == 0 as libc::c_int as libc::c_uint {
        return f128::f128::new(1 as libc::c_int);
    }
    loop {
        x = x.wrapping_sub(1);
        if !(x != 0) {
            break;
        }
        result *= base;
    }
    return result;
}
#[inline]
unsafe extern "C" fn absld(mut val: f128::f128) -> f128::f128 {
    return if val < f128::f128::new(0 as libc::c_int) { -val } else { val };
}
unsafe extern "C" fn expld(
    mut val: f128::f128,
    mut base: libc::c_uint,
    mut x: *mut libc::c_uint,
) -> f128::f128 {
    let mut power: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    if val >= -f128::f128::new(1.18973149535723176508575932662800702e+4932)
        && val <= f128::f128::new(1.18973149535723176508575932662800702e+4932)
    {
        while absld(val) >= f128::f128::new(base) {
            power = power.wrapping_add(1);
            val /= f128::f128::new(base);
        }
    }
    if !x.is_null() {
        *x = power;
    }
    return val;
}
#[inline]
unsafe extern "C" fn simple_round_ceiling(mut val: f128::f128) -> intmax_t {
    let mut intval: intmax_t = val.to_i64().unwrap();
    if f128::f128::new(intval) < val {
        intval += 1;
    }
    return intval;
}
#[inline]
unsafe extern "C" fn simple_round_floor(mut val: f128::f128) -> intmax_t {
    return -simple_round_ceiling(-val);
}
#[inline]
unsafe extern "C" fn simple_round_from_zero(mut val: f128::f128) -> intmax_t {
    return if val < f128::f128::new(0 as libc::c_int) {
        simple_round_floor(val)
    } else {
        simple_round_ceiling(val)
    };
}
#[inline]
unsafe extern "C" fn simple_round_to_zero(mut val: f128::f128) -> intmax_t {
    return val.to_i64().unwrap();
}
#[inline]
unsafe extern "C" fn simple_round_nearest(mut val: f128::f128) -> intmax_t {
    return if val < f128::f128::new(0 as libc::c_int) {
        val - f128::f128::new(0.5f64)
    } else {
        val + f128::f128::new(0.5f64)
    }
        .to_i64()
        .unwrap();
}
#[inline]
unsafe extern "C" fn simple_round(mut val: f128::f128, mut t: round_type) -> f128::f128 {
    let mut rval: intmax_t = 0;
    let mut intmax_mul: intmax_t = (val
        / f128::f128::new(9223372036854775807 as libc::c_long))
        .to_i64()
        .unwrap();
    val
        -= f128::f128::new(9223372036854775807 as libc::c_long)
            * f128::f128::new(intmax_mul);
    match t as libc::c_uint {
        0 => {
            rval = simple_round_ceiling(val);
        }
        1 => {
            rval = simple_round_floor(val);
        }
        2 => {
            rval = simple_round_from_zero(val);
        }
        3 => {
            rval = simple_round_to_zero(val);
        }
        4 => {
            rval = simple_round_nearest(val);
        }
        _ => return f128::f128::new(0 as libc::c_int),
    }
    return f128::f128::new(9223372036854775807 as libc::c_long)
        * f128::f128::new(intmax_mul) + f128::f128::new(rval);
}
unsafe extern "C" fn simple_strtod_int(
    mut input_str: *const libc::c_char,
    mut endptr: *mut *mut libc::c_char,
    mut value: *mut f128::f128,
    mut negative: *mut bool,
) -> simple_strtod_error {
    let mut e: simple_strtod_error = SSE_OK;
    let mut val: f128::f128 = f128::f128::new(0 as libc::c_int);
    let mut digits: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    let mut found_digit: bool = 0 as libc::c_int != 0;
    if *input_str as libc::c_int == '-' as i32 {
        input_str = input_str.offset(1);
        *negative = 1 as libc::c_int != 0;
    } else {
        *negative = 0 as libc::c_int != 0;
    }
    *endptr = input_str as *mut libc::c_char;
    while c_isdigit(**endptr as libc::c_int) {
        let mut digit: libc::c_int = **endptr as libc::c_int - '0' as i32;
        found_digit = 1 as libc::c_int != 0;
        if val != 0. || digit != 0 {
            digits = digits.wrapping_add(1);
        }
        if digits > MAX_UNSCALED_DIGITS as libc::c_int as libc::c_uint {
            e = SSE_OK_PRECISION_LOSS;
        }
        if digits > MAX_ACCEPTABLE_DIGITS as libc::c_int as libc::c_uint {
            return SSE_OVERFLOW;
        }
        val *= f128::f128::new(10 as libc::c_int);
        val += f128::f128::new(digit);
        *endptr = (*endptr).offset(1);
    }
    if !found_digit
        && !(strncmp(*endptr, decimal_point, decimal_point_length as libc::c_ulong)
            == 0 as libc::c_int)
    {
        return SSE_INVALID_NUMBER;
    }
    if *negative {
        val = -val;
    }
    if !value.is_null() {
        *value = val;
    }
    return e;
}
unsafe extern "C" fn simple_strtod_float(
    mut input_str: *const libc::c_char,
    mut endptr: *mut *mut libc::c_char,
    mut value: *mut f128::f128,
    mut precision: *mut size_t,
) -> simple_strtod_error {
    let mut negative: bool = false;
    let mut e: simple_strtod_error = SSE_OK;
    if !precision.is_null() {
        *precision = 0 as libc::c_int as size_t;
    }
    e = simple_strtod_int(input_str, endptr, value, &mut negative);
    if e as libc::c_uint != SSE_OK as libc::c_int as libc::c_uint
        && e as libc::c_uint != SSE_OK_PRECISION_LOSS as libc::c_int as libc::c_uint
    {
        return e;
    }
    if strncmp(*endptr, decimal_point, decimal_point_length as libc::c_ulong)
        == 0 as libc::c_int
    {
        let mut ptr2: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut val_frac: f128::f128 = f128::f128::new(0 as libc::c_int);
        let mut neg_frac: bool = false;
        *endptr = (*endptr).offset(decimal_point_length as isize);
        let mut e2: simple_strtod_error = simple_strtod_int(
            *endptr,
            &mut ptr2,
            &mut val_frac,
            &mut neg_frac,
        );
        if e2 as libc::c_uint != SSE_OK as libc::c_int as libc::c_uint
            && e2 as libc::c_uint != SSE_OK_PRECISION_LOSS as libc::c_int as libc::c_uint
        {
            return e2;
        }
        if e2 as libc::c_uint == SSE_OK_PRECISION_LOSS as libc::c_int as libc::c_uint {
            e = e2;
        }
        if neg_frac {
            return SSE_INVALID_NUMBER;
        }
        let mut exponent: size_t = ptr2.offset_from(*endptr) as libc::c_long as size_t;
        val_frac = val_frac
            / powerld(f128::f128::new(10 as libc::c_int), exponent as libc::c_uint);
        if !value.is_null() {
            if negative {
                *value -= val_frac;
            } else {
                *value += val_frac;
            }
        }
        if !precision.is_null() {
            *precision = exponent;
        }
        *endptr = ptr2;
    }
    return e;
}
unsafe extern "C" fn simple_strtod_human(
    mut input_str: *const libc::c_char,
    mut endptr: *mut *mut libc::c_char,
    mut value: *mut f128::f128,
    mut precision: *mut size_t,
    mut allowed_scaling: scale_type,
) -> simple_strtod_error {
    let mut power: libc::c_int = 0 as libc::c_int;
    let mut scale_base: libc::c_int = default_scale_base(allowed_scaling);
    if dev_debug {
        fprintf(
            stderr,
            b"simple_strtod_human:\n  input string: %s\n  locale decimal-point: %s\n  MAX_UNSCALED_DIGITS: %d\n\0"
                as *const u8 as *const libc::c_char,
            quote_n(0 as libc::c_int, input_str),
            quote_n(1 as libc::c_int, decimal_point),
            MAX_UNSCALED_DIGITS as libc::c_int,
        );
    }
    let mut e: simple_strtod_error = simple_strtod_float(
        input_str,
        endptr,
        value,
        precision,
    );
    if e as libc::c_uint != SSE_OK as libc::c_int as libc::c_uint
        && e as libc::c_uint != SSE_OK_PRECISION_LOSS as libc::c_int as libc::c_uint
    {
        return e;
    }
    if dev_debug {
        fprintf(
            stderr,
            b"  parsed numeric value: %Lf\n  input precision = %d\n\0" as *const u8
                as *const libc::c_char,
            *value,
            *precision as libc::c_int,
        );
    }
    if **endptr as libc::c_int != '\0' as i32 {
        while *(*__ctype_b_loc()).offset(to_uchar(**endptr) as libc::c_int as isize)
            as libc::c_int & _ISblank as libc::c_int as libc::c_ushort as libc::c_int
            != 0
        {
            *endptr = (*endptr).offset(1);
        }
        if !valid_suffix(**endptr) {
            return SSE_INVALID_SUFFIX;
        }
        if allowed_scaling as libc::c_uint == scale_none as libc::c_int as libc::c_uint {
            return SSE_VALID_BUT_FORBIDDEN_SUFFIX;
        }
        power = suffix_power(**endptr);
        *endptr = (*endptr).offset(1);
        if allowed_scaling as libc::c_uint == scale_auto as libc::c_int as libc::c_uint
            && **endptr as libc::c_int == 'i' as i32
        {
            scale_base = 1024 as libc::c_int;
            *endptr = (*endptr).offset(1);
            if dev_debug {
                fprintf(
                    stderr,
                    b"  Auto-scaling, found 'i', switching to base %d\n\0" as *const u8
                        as *const libc::c_char,
                    scale_base,
                );
            }
        }
        *precision = 0 as libc::c_int as size_t;
    }
    if allowed_scaling as libc::c_uint == scale_IEC_I as libc::c_int as libc::c_uint {
        if **endptr as libc::c_int == 'i' as i32 {
            *endptr = (*endptr).offset(1);
        } else {
            return SSE_MISSING_I_SUFFIX
        }
    }
    let mut multiplier: f128::f128 = powerld(
        f128::f128::new(scale_base),
        power as libc::c_uint,
    );
    if dev_debug {
        fprintf(
            stderr,
            b"  suffix power=%d^%d = %Lf\n\0" as *const u8 as *const libc::c_char,
            scale_base,
            power,
            multiplier,
        );
    }
    *value = *value * multiplier;
    if dev_debug {
        fprintf(
            stderr,
            b"  returning value: %Lf (%LG)\n\0" as *const u8 as *const libc::c_char,
            *value,
            *value,
        );
    }
    return e;
}
unsafe extern "C" fn simple_strtod_fatal(
    mut err: simple_strtod_error,
    mut input_str: *const libc::c_char,
) {
    let mut msgid: *const libc::c_char = 0 as *const libc::c_char;
    match err as libc::c_uint {
        1 | 0 => {
            abort();
        }
        2 => {
            msgid = b"value too large to be converted: %s\0" as *const u8
                as *const libc::c_char;
        }
        3 => {
            msgid = b"invalid number: %s\0" as *const u8 as *const libc::c_char;
        }
        4 => {
            msgid = b"rejecting suffix in input: %s (consider using --from)\0"
                as *const u8 as *const libc::c_char;
        }
        5 => {
            msgid = b"invalid suffix in input: %s\0" as *const u8 as *const libc::c_char;
        }
        6 => {
            msgid = b"missing 'i' suffix in input: %s (e.g Ki/Mi/Gi)\0" as *const u8
                as *const libc::c_char;
        }
        _ => {}
    }
    if inval_style as libc::c_uint != inval_ignore as libc::c_int as libc::c_uint {
        error(
            conv_exit_code,
            0 as libc::c_int,
            dcgettext(0 as *const libc::c_char, msgid, 5 as libc::c_int),
            quote(input_str),
        );
    }
}
unsafe extern "C" fn double_to_human(
    mut val: f128::f128,
    mut precision: libc::c_int,
    mut buf: *mut libc::c_char,
    mut buf_size: size_t,
    mut scale: scale_type,
    mut group: libc::c_int,
    mut round: round_type,
) {
    let mut num_size: libc::c_int = 0;
    let mut fmt: [libc::c_char; 64] = [0; 64];
    let mut pfmt: *mut libc::c_char = fmt.as_mut_ptr();
    let fresh2 = pfmt;
    pfmt = pfmt.offset(1);
    *fresh2 = '%' as i32 as libc::c_char;
    if group != 0 {
        let fresh3 = pfmt;
        pfmt = pfmt.offset(1);
        *fresh3 = '\'' as i32 as libc::c_char;
    }
    if zero_padding_width != 0 {
        pfmt = pfmt
            .offset(
                snprintf(
                    pfmt,
                    (::core::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong)
                        .wrapping_sub(2 as libc::c_int as libc::c_ulong),
                    b"0%ld\0" as *const u8 as *const libc::c_char,
                    zero_padding_width,
                ) as isize,
            );
    }
    if dev_debug {
        fprintf(stderr, b"double_to_human:\n\0" as *const u8 as *const libc::c_char);
    }
    if scale as libc::c_uint == scale_none as libc::c_int as libc::c_uint {
        val *= powerld(f128::f128::new(10 as libc::c_int), precision as libc::c_uint);
        val = simple_round(val, round);
        val /= powerld(f128::f128::new(10 as libc::c_int), precision as libc::c_uint);
        if dev_debug {
            fprintf(
                stderr,
                if group != 0 {
                    b"  no scaling, returning (grouped) value: %'.*Lf\n\0" as *const u8
                        as *const libc::c_char
                } else {
                    b"  no scaling, returning value: %.*Lf\n\0" as *const u8
                        as *const libc::c_char
                },
                precision,
                val,
            );
        }
        stpcpy(pfmt, b".*Lf\0" as *const u8 as *const libc::c_char);
        num_size = snprintf(buf, buf_size, fmt.as_mut_ptr(), precision, val);
        if num_size < 0 as libc::c_int || num_size >= buf_size as libc::c_int {
            if ::core::mem::size_of::<C2RustUnnamed_10>() as libc::c_ulong != 0 {
                error(
                    1 as libc::c_int,
                    0 as libc::c_int,
                    dcgettext(
                        0 as *const libc::c_char,
                        b"failed to prepare value '%Lf' for printing\0" as *const u8
                            as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    val,
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
                        b"failed to prepare value '%Lf' for printing\0" as *const u8
                            as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    val,
                );
                if 0 as libc::c_int != 0 {} else {
                    unreachable!();
                };
            };
        }
        return;
    }
    let mut scale_base: libc::c_double = default_scale_base(scale) as libc::c_double;
    let mut power: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    val = expld(val, scale_base as libc::c_uint, &mut power);
    if dev_debug {
        fprintf(
            stderr,
            b"  scaled value to %Lf * %0.f ^ %u\n\0" as *const u8 as *const libc::c_char,
            val,
            scale_base,
            power,
        );
    }
    let mut power_adjust: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    if user_precision != -(1 as libc::c_int) as libc::c_long {
        power_adjust = (if (power.wrapping_mul(3 as libc::c_int as libc::c_uint)
            as libc::c_long) < user_precision
        {
            power.wrapping_mul(3 as libc::c_int as libc::c_uint) as libc::c_long
        } else {
            user_precision
        }) as libc::c_uint;
    } else if absld(val) < f128::f128::new(10 as libc::c_int) {
        power_adjust = 1 as libc::c_int as libc::c_uint;
    }
    val *= powerld(f128::f128::new(10 as libc::c_int), power_adjust);
    val = simple_round(val, round);
    val /= powerld(f128::f128::new(10 as libc::c_int), power_adjust);
    if absld(val) >= f128::f128::new(scale_base) {
        val /= f128::f128::new(scale_base);
        power = power.wrapping_add(1);
    }
    let mut show_decimal_point: libc::c_int = (val != f128::f128::new(0 as libc::c_int)
        && absld(val) < f128::f128::new(10 as libc::c_int)
        && power > 0 as libc::c_int as libc::c_uint) as libc::c_int;
    if dev_debug {
        fprintf(
            stderr,
            b"  after rounding, value=%Lf * %0.f ^ %u\n\0" as *const u8
                as *const libc::c_char,
            val,
            scale_base,
            power,
        );
    }
    stpcpy(pfmt, b".*Lf%s\0" as *const u8 as *const libc::c_char);
    let mut prec: libc::c_int = (if user_precision == -(1 as libc::c_int) as libc::c_long
    {
        show_decimal_point as libc::c_long
    } else {
        user_precision
    }) as libc::c_int;
    num_size = snprintf(
        buf,
        buf_size.wrapping_sub(1 as libc::c_int as libc::c_ulong),
        fmt.as_mut_ptr(),
        prec,
        val,
        suffix_power_char(power),
    );
    if num_size < 0 as libc::c_int
        || num_size >= buf_size as libc::c_int - 1 as libc::c_int
    {
        if ::core::mem::size_of::<C2RustUnnamed_9>() as libc::c_ulong != 0 {
            error(
                1 as libc::c_int,
                0 as libc::c_int,
                dcgettext(
                    0 as *const libc::c_char,
                    b"failed to prepare value '%Lf' for printing\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
                val,
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
                    b"failed to prepare value '%Lf' for printing\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
                val,
            );
            if 0 as libc::c_int != 0 {} else {
                unreachable!();
            };
        };
    }
    if scale as libc::c_uint == scale_IEC_I as libc::c_int as libc::c_uint
        && power > 0 as libc::c_int as libc::c_uint
    {
        strncat(
            buf,
            b"i\0" as *const u8 as *const libc::c_char,
            buf_size
                .wrapping_sub(num_size as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong),
        );
    }
    if dev_debug {
        fprintf(
            stderr,
            b"  returning value: %s\n\0" as *const u8 as *const libc::c_char,
            quote(buf),
        );
    }
}
unsafe extern "C" fn unit_to_umax(mut n_string: *const libc::c_char) -> uintmax_t {
    let mut s_err: strtol_error = LONGINT_OK;
    let mut c_string: *const libc::c_char = n_string;
    let mut t_string: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut n_len: size_t = strlen(n_string);
    let mut end: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut n: uintmax_t = 0;
    let mut suffixes: *const libc::c_char = valid_suffixes;
    if n_len != 0
        && !c_isdigit(
            *n_string
                .offset(n_len.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize)
                as libc::c_int,
        )
    {
        t_string = xmalloc(n_len.wrapping_add(2 as libc::c_int as libc::c_ulong))
            as *mut libc::c_char;
        end = t_string.offset(n_len as isize).offset(-(1 as libc::c_int as isize));
        memcpy(t_string as *mut libc::c_void, n_string as *const libc::c_void, n_len);
        if *end as libc::c_int == 'i' as i32
            && 2 as libc::c_int as libc::c_ulong <= n_len
            && !c_isdigit(*end.offset(-(1 as libc::c_int as isize)) as libc::c_int)
        {
            *end = '\0' as i32 as libc::c_char;
        } else {
            end = end.offset(1);
            *end = 'B' as i32 as libc::c_char;
            end = end.offset(1);
            *end = '\0' as i32 as libc::c_char;
            suffixes = zero_and_valid_suffixes.as_ptr();
        }
        c_string = t_string;
    }
    s_err = xstrtoumax(c_string, &mut end, 10 as libc::c_int, &mut n, suffixes);
    if s_err as libc::c_uint != LONGINT_OK as libc::c_int as libc::c_uint
        || *end as libc::c_int != 0 || n == 0 as libc::c_int as libc::c_ulong
    {
        free(t_string as *mut libc::c_void);
        if ::core::mem::size_of::<C2RustUnnamed_11>() as libc::c_ulong != 0 {
            error(
                1 as libc::c_int,
                0 as libc::c_int,
                dcgettext(
                    0 as *const libc::c_char,
                    b"invalid unit size: %s\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
                quote(n_string),
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
                    b"invalid unit size: %s\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
                quote(n_string),
            );
            if 0 as libc::c_int != 0 {} else {
                unreachable!();
            };
        };
    }
    free(t_string as *mut libc::c_void);
    return n;
}
unsafe extern "C" fn setup_padding_buffer(mut min_size: size_t) {
    if padding_buffer_size > min_size {
        return;
    }
    padding_buffer_size = min_size.wrapping_add(1 as libc::c_int as libc::c_ulong);
    padding_buffer = xrealloc(padding_buffer as *mut libc::c_void, padding_buffer_size)
        as *mut libc::c_char;
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
                b"Usage: %s [OPTION]... [NUMBER]...\n\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
            program_name,
        );
        fputs_unlocked(
            dcgettext(
                0 as *const libc::c_char,
                b"Reformat NUMBER(s), or the numbers from standard input if none are specified.\n\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            stdout,
        );
        emit_mandatory_arg_note();
        fputs_unlocked(
            dcgettext(
                0 as *const libc::c_char,
                b"      --debug          print warnings about invalid input\n\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            stdout,
        );
        fputs_unlocked(
            dcgettext(
                0 as *const libc::c_char,
                b"  -d, --delimiter=X    use X instead of whitespace for field delimiter\n\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            stdout,
        );
        fputs_unlocked(
            dcgettext(
                0 as *const libc::c_char,
                b"      --field=FIELDS   replace the numbers in these input fields (default=1);\n                         see FIELDS below\n\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            stdout,
        );
        fputs_unlocked(
            dcgettext(
                0 as *const libc::c_char,
                b"      --format=FORMAT  use printf style floating-point FORMAT;\n                         see FORMAT below for details\n\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            stdout,
        );
        fputs_unlocked(
            dcgettext(
                0 as *const libc::c_char,
                b"      --from=UNIT      auto-scale input numbers to UNITs; default is 'none';\n                         see UNIT below\n\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            stdout,
        );
        fputs_unlocked(
            dcgettext(
                0 as *const libc::c_char,
                b"      --from-unit=N    specify the input unit size (instead of the default 1)\n\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            stdout,
        );
        fputs_unlocked(
            dcgettext(
                0 as *const libc::c_char,
                b"      --grouping       use locale-defined grouping of digits, e.g. 1,000,000\n                         (which means it has no effect in the C/POSIX locale)\n\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            stdout,
        );
        fputs_unlocked(
            dcgettext(
                0 as *const libc::c_char,
                b"      --header[=N]     print (without converting) the first N header lines;\n                         N defaults to 1 if not specified\n\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            stdout,
        );
        fputs_unlocked(
            dcgettext(
                0 as *const libc::c_char,
                b"      --invalid=MODE   failure mode for invalid numbers: MODE can be:\n                         abort (default), fail, warn, ignore\n\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            stdout,
        );
        fputs_unlocked(
            dcgettext(
                0 as *const libc::c_char,
                b"      --padding=N      pad the output to N characters; positive N will\n                         right-align; negative N will left-align;\n                         padding is ignored if the output is wider than N;\n                         the default is to automatically pad if a whitespace\n                         is found\n\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            stdout,
        );
        fputs_unlocked(
            dcgettext(
                0 as *const libc::c_char,
                b"      --round=METHOD   use METHOD for rounding when scaling; METHOD can be:\n                         up, down, from-zero (default), towards-zero, nearest\n\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            stdout,
        );
        fputs_unlocked(
            dcgettext(
                0 as *const libc::c_char,
                b"      --suffix=SUFFIX  add SUFFIX to output numbers, and accept optional\n                         SUFFIX in input numbers\n\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            stdout,
        );
        fputs_unlocked(
            dcgettext(
                0 as *const libc::c_char,
                b"      --to=UNIT        auto-scale output numbers to UNITs; see UNIT below\n\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            stdout,
        );
        fputs_unlocked(
            dcgettext(
                0 as *const libc::c_char,
                b"      --to-unit=N      the output unit size (instead of the default 1)\n\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            stdout,
        );
        fputs_unlocked(
            dcgettext(
                0 as *const libc::c_char,
                b"  -z, --zero-terminated    line delimiter is NUL, not newline\n\0"
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
                b"\nUNIT options:\n\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            stdout,
        );
        fputs_unlocked(
            dcgettext(
                0 as *const libc::c_char,
                b"  none       no auto-scaling is done; suffixes will trigger an error\n\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            stdout,
        );
        fputs_unlocked(
            dcgettext(
                0 as *const libc::c_char,
                b"  auto       accept optional single/two letter suffix:\n               1K = 1000,\n               1Ki = 1024,\n               1M = 1000000,\n               1Mi = 1048576,\n\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            stdout,
        );
        fputs_unlocked(
            dcgettext(
                0 as *const libc::c_char,
                b"  si         accept optional single letter suffix:\n               1K = 1000,\n               1M = 1000000,\n               ...\n\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            stdout,
        );
        fputs_unlocked(
            dcgettext(
                0 as *const libc::c_char,
                b"  iec        accept optional single letter suffix:\n               1K = 1024,\n               1M = 1048576,\n               ...\n\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            stdout,
        );
        fputs_unlocked(
            dcgettext(
                0 as *const libc::c_char,
                b"  iec-i      accept optional two-letter suffix:\n               1Ki = 1024,\n               1Mi = 1048576,\n               ...\n\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            stdout,
        );
        fputs_unlocked(
            dcgettext(
                0 as *const libc::c_char,
                b"\nFIELDS supports cut(1) style field ranges:\n  N    N'th field, counted from 1\n  N-   from N'th field, to end of line\n  N-M  from N'th to M'th field (inclusive)\n  -M   from first to M'th field (inclusive)\n  -    all fields\nMultiple fields/ranges can be separated with commas\n\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            stdout,
        );
        fputs_unlocked(
            dcgettext(
                0 as *const libc::c_char,
                b"\nFORMAT must be suitable for printing one floating-point argument '%f'.\nOptional quote (%'f) will enable --grouping (if supported by current locale).\nOptional width value (%10f) will pad output. Optional zero (%010f) width\nwill zero pad the number. Optional negative values (%-10f) will left align.\nOptional precision (%.1f) will override the input determined precision.\n\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            stdout,
        );
        printf(
            dcgettext(
                0 as *const libc::c_char,
                b"\nExit status is 0 if all input numbers were successfully converted.\nBy default, %s will stop at the first conversion error with exit status 2.\nWith --invalid='fail' a warning is printed for each conversion error\nand the exit status is 2.  With --invalid='warn' each conversion error is\ndiagnosed, but the exit status is 0.  With --invalid='ignore' conversion\nerrors are not diagnosed and the exit status is 0.\n\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            program_name,
        );
        printf(
            dcgettext(
                0 as *const libc::c_char,
                b"\nExamples:\n  $ %s --to=si 1000\n            -> \"1.0K\"\n  $ %s --to=iec 2048\n           -> \"2.0K\"\n  $ %s --to=iec-i 4096\n           -> \"4.0Ki\"\n  $ echo 1K | %s --from=si\n           -> \"1000\"\n  $ echo 1K | %s --from=iec\n           -> \"1024\"\n  $ df -B1 | %s --header --field 2-4 --to=si\n  $ ls -l  | %s --header --field 5 --to=iec\n  $ ls -lh | %s --header --field 5 --from=iec --padding=10\n  $ ls -lh | %s --header --field 5 --from=iec --format %%10f\n\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            program_name,
            program_name,
            program_name,
            program_name,
            program_name,
            program_name,
            program_name,
            program_name,
            program_name,
        );
        emit_ancillary_info(b"numfmt\0" as *const u8 as *const libc::c_char);
    }
    exit(status);
}
unsafe extern "C" fn parse_format_string(mut fmt: *const libc::c_char) {
    let mut i: size_t = 0;
    let mut prefix_len: size_t = 0 as libc::c_int as size_t;
    let mut suffix_pos: size_t = 0;
    let mut pad: libc::c_long = 0 as libc::c_int as libc::c_long;
    let mut endptr: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut zero_padding: bool = 0 as libc::c_int != 0;
    i = 0 as libc::c_int as size_t;
    while !(*fmt.offset(i as isize) as libc::c_int == '%' as i32
        && *fmt.offset(i.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize)
            as libc::c_int != '%' as i32)
    {
        if *fmt.offset(i as isize) == 0 {
            if ::core::mem::size_of::<C2RustUnnamed_17>() as libc::c_ulong != 0 {
                error(
                    1 as libc::c_int,
                    0 as libc::c_int,
                    dcgettext(
                        0 as *const libc::c_char,
                        b"format %s has no %% directive\0" as *const u8
                            as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    quote(fmt),
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
                        b"format %s has no %% directive\0" as *const u8
                            as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    quote(fmt),
                );
                if 0 as libc::c_int != 0 {} else {
                    unreachable!();
                };
            };
        }
        prefix_len = prefix_len.wrapping_add(1);
        i = (i as libc::c_ulong)
            .wrapping_add(
                ((*fmt.offset(i as isize) as libc::c_int == '%' as i32) as libc::c_int
                    + 1 as libc::c_int) as libc::c_ulong,
            ) as size_t as size_t;
    }
    i = i.wrapping_add(1);
    loop {
        let mut skip: size_t = strspn(
            fmt.offset(i as isize),
            b" \0" as *const u8 as *const libc::c_char,
        );
        i = (i as libc::c_ulong).wrapping_add(skip) as size_t as size_t;
        if *fmt.offset(i as isize) as libc::c_int == '\'' as i32 {
            grouping = 1 as libc::c_int;
            i = i.wrapping_add(1);
        } else if *fmt.offset(i as isize) as libc::c_int == '0' as i32 {
            zero_padding = 1 as libc::c_int != 0;
            i = i.wrapping_add(1);
        } else if skip == 0 {
            break;
        }
    }
    *__errno_location() = 0 as libc::c_int;
    pad = strtol(fmt.offset(i as isize), &mut endptr, 10 as libc::c_int);
    if *__errno_location() == 34 as libc::c_int
        || pad < -(9223372036854775807 as libc::c_long)
    {
        if ::core::mem::size_of::<C2RustUnnamed_16>() as libc::c_ulong != 0 {
            error(
                1 as libc::c_int,
                0 as libc::c_int,
                dcgettext(
                    0 as *const libc::c_char,
                    b"invalid format %s (width overflow)\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
                quote(fmt),
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
                    b"invalid format %s (width overflow)\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
                quote(fmt),
            );
            if 0 as libc::c_int != 0 {} else {
                unreachable!();
            };
        };
    }
    if endptr != fmt.offset(i as isize) as *mut libc::c_char
        && pad != 0 as libc::c_int as libc::c_long
    {
        if debug as libc::c_int != 0 && padding_width != 0
            && !(zero_padding as libc::c_int != 0
                && pad > 0 as libc::c_int as libc::c_long)
        {
            error(
                0 as libc::c_int,
                0 as libc::c_int,
                dcgettext(
                    0 as *const libc::c_char,
                    b"--format padding overriding --padding\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
            );
        }
        if pad < 0 as libc::c_int as libc::c_long {
            padding_alignment = MBS_ALIGN_LEFT;
            padding_width = -pad;
        } else if zero_padding {
            zero_padding_width = pad;
        } else {
            padding_width = pad;
        }
    }
    i = endptr.offset_from(fmt) as libc::c_long as size_t;
    if *fmt.offset(i as isize) as libc::c_int == '\0' as i32 {
        if ::core::mem::size_of::<C2RustUnnamed_15>() as libc::c_ulong != 0 {
            error(
                1 as libc::c_int,
                0 as libc::c_int,
                dcgettext(
                    0 as *const libc::c_char,
                    b"format %s ends in %%\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
                quote(fmt),
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
                    b"format %s ends in %%\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
                quote(fmt),
            );
            if 0 as libc::c_int != 0 {} else {
                unreachable!();
            };
        };
    }
    if *fmt.offset(i as isize) as libc::c_int == '.' as i32 {
        i = i.wrapping_add(1);
        *__errno_location() = 0 as libc::c_int;
        user_precision = strtol(fmt.offset(i as isize), &mut endptr, 10 as libc::c_int);
        if *__errno_location() == 34 as libc::c_int
            || user_precision < 0 as libc::c_int as libc::c_long
            || (18446744073709551615 as libc::c_ulong) < user_precision as libc::c_ulong
            || *(*__ctype_b_loc())
                .offset(*fmt.offset(i as isize) as libc::c_int as isize) as libc::c_int
                & _ISblank as libc::c_int as libc::c_ushort as libc::c_int != 0
            || *fmt.offset(i as isize) as libc::c_int == '+' as i32
        {
            if ::core::mem::size_of::<C2RustUnnamed_14>() as libc::c_ulong != 0 {
                error(
                    1 as libc::c_int,
                    0 as libc::c_int,
                    dcgettext(
                        0 as *const libc::c_char,
                        b"invalid precision in format %s\0" as *const u8
                            as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    quote(fmt),
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
                        b"invalid precision in format %s\0" as *const u8
                            as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    quote(fmt),
                );
                if 0 as libc::c_int != 0 {} else {
                    unreachable!();
                };
            };
        }
        i = endptr.offset_from(fmt) as libc::c_long as size_t;
    }
    if *fmt.offset(i as isize) as libc::c_int != 'f' as i32 {
        if ::core::mem::size_of::<C2RustUnnamed_13>() as libc::c_ulong != 0 {
            error(
                1 as libc::c_int,
                0 as libc::c_int,
                dcgettext(
                    0 as *const libc::c_char,
                    b"invalid format %s, directive must be %%[0]['][-][N][.][N]f\0"
                        as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
                quote(fmt),
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
                    b"invalid format %s, directive must be %%[0]['][-][N][.][N]f\0"
                        as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
                quote(fmt),
            );
            if 0 as libc::c_int != 0 {} else {
                unreachable!();
            };
        };
    }
    i = i.wrapping_add(1);
    suffix_pos = i;
    while *fmt.offset(i as isize) as libc::c_int != '\0' as i32 {
        if *fmt.offset(i as isize) as libc::c_int == '%' as i32
            && *fmt.offset(i.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize)
                as libc::c_int != '%' as i32
        {
            if ::core::mem::size_of::<C2RustUnnamed_12>() as libc::c_ulong != 0 {
                error(
                    1 as libc::c_int,
                    0 as libc::c_int,
                    dcgettext(
                        0 as *const libc::c_char,
                        b"format %s has too many %% directives\0" as *const u8
                            as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    quote(fmt),
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
                        b"format %s has too many %% directives\0" as *const u8
                            as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    quote(fmt),
                );
                if 0 as libc::c_int != 0 {} else {
                    unreachable!();
                };
            };
        }
        i = (i as libc::c_ulong)
            .wrapping_add(
                ((*fmt.offset(i as isize) as libc::c_int == '%' as i32) as libc::c_int
                    + 1 as libc::c_int) as libc::c_ulong,
            ) as size_t as size_t;
    }
    if prefix_len != 0 {
        format_str_prefix = ximemdup0(fmt as *const libc::c_void, prefix_len as idx_t);
    }
    if *fmt.offset(suffix_pos as isize) as libc::c_int != '\0' as i32 {
        format_str_suffix = xstrdup(fmt.offset(suffix_pos as isize));
    }
    if dev_debug {
        fprintf(
            stderr,
            b"format String:\n  input: %s\n  grouping: %s\n  padding width: %ld\n  alignment: %s\n  prefix: %s\n  suffix: %s\n\0"
                as *const u8 as *const libc::c_char,
            quote_n(0 as libc::c_int, fmt),
            if grouping != 0 {
                b"yes\0" as *const u8 as *const libc::c_char
            } else {
                b"no\0" as *const u8 as *const libc::c_char
            },
            padding_width,
            if padding_alignment as libc::c_uint
                == MBS_ALIGN_LEFT as libc::c_int as libc::c_uint
            {
                b"Left\0" as *const u8 as *const libc::c_char
            } else {
                b"Right\0" as *const u8 as *const libc::c_char
            },
            quote_n(
                1 as libc::c_int,
                if !format_str_prefix.is_null() {
                    format_str_prefix as *const libc::c_char
                } else {
                    b"\0" as *const u8 as *const libc::c_char
                },
            ),
            quote_n(
                2 as libc::c_int,
                if !format_str_suffix.is_null() {
                    format_str_suffix as *const libc::c_char
                } else {
                    b"\0" as *const u8 as *const libc::c_char
                },
            ),
        );
    }
}
unsafe extern "C" fn parse_human_number(
    mut str: *const libc::c_char,
    mut value: *mut f128::f128,
    mut precision: *mut size_t,
) -> simple_strtod_error {
    let mut ptr: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut e: simple_strtod_error = simple_strtod_human(
        str,
        &mut ptr,
        value,
        precision,
        scale_from,
    );
    if e as libc::c_uint != SSE_OK as libc::c_int as libc::c_uint
        && e as libc::c_uint != SSE_OK_PRECISION_LOSS as libc::c_int as libc::c_uint
    {
        simple_strtod_fatal(e, str);
        return e;
    }
    if !ptr.is_null() && *ptr as libc::c_int != '\0' as i32 {
        if inval_style as libc::c_uint != inval_ignore as libc::c_int as libc::c_uint {
            error(
                conv_exit_code,
                0 as libc::c_int,
                dcgettext(
                    0 as *const libc::c_char,
                    b"invalid suffix in input %s: %s\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
                quote_n(0 as libc::c_int, str),
                quote_n(1 as libc::c_int, ptr),
            );
        }
        e = SSE_INVALID_SUFFIX;
    }
    return e;
}
unsafe extern "C" fn prepare_padded_number(
    val: f128::f128,
    mut precision: size_t,
) -> libc::c_int {
    let mut buf: [libc::c_char; 128] = [0; 128];
    let mut precision_used: size_t = if user_precision
        == -(1 as libc::c_int) as libc::c_long
    {
        precision
    } else {
        user_precision as libc::c_ulong
    };
    let mut x: libc::c_uint = 0;
    expld(val, 10 as libc::c_int as libc::c_uint, &mut x);
    if scale_to as libc::c_uint == scale_none as libc::c_int as libc::c_uint
        && (x as libc::c_ulong).wrapping_add(precision_used)
            > MAX_UNSCALED_DIGITS as libc::c_int as libc::c_ulong
    {
        if inval_style as libc::c_uint != inval_ignore as libc::c_int as libc::c_uint {
            if precision_used != 0 {
                error(
                    conv_exit_code,
                    0 as libc::c_int,
                    dcgettext(
                        0 as *const libc::c_char,
                        b"value/precision too large to be printed: '%Lg/%lu' (consider using --to)\0"
                            as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    val,
                    precision_used,
                );
            } else {
                error(
                    conv_exit_code,
                    0 as libc::c_int,
                    dcgettext(
                        0 as *const libc::c_char,
                        b"value too large to be printed: '%Lg' (consider using --to)\0"
                            as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    val,
                );
            }
        }
        return 0 as libc::c_int;
    }
    if x > (MAX_ACCEPTABLE_DIGITS as libc::c_int - 1 as libc::c_int) as libc::c_uint {
        if inval_style as libc::c_uint != inval_ignore as libc::c_int as libc::c_uint {
            error(
                conv_exit_code,
                0 as libc::c_int,
                dcgettext(
                    0 as *const libc::c_char,
                    b"value too large to be printed: '%Lg' (cannot handle values > 999Q)\0"
                        as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
                val,
            );
        }
        return 0 as libc::c_int;
    }
    double_to_human(
        val,
        precision_used as libc::c_int,
        buf.as_mut_ptr(),
        ::core::mem::size_of::<[libc::c_char; 128]>() as libc::c_ulong,
        scale_to,
        grouping,
        round_style,
    );
    if !suffix.is_null() {
        strncat(
            buf.as_mut_ptr(),
            suffix,
            (::core::mem::size_of::<[libc::c_char; 128]>() as libc::c_ulong)
                .wrapping_sub(strlen(buf.as_mut_ptr()))
                .wrapping_sub(1 as libc::c_int as libc::c_ulong),
        );
    }
    if dev_debug {
        fprintf(
            stderr,
            b"formatting output:\n  value: %Lf\n  humanized: %s\n\0" as *const u8
                as *const libc::c_char,
            val,
            quote(buf.as_mut_ptr()),
        );
    }
    if padding_width != 0 && strlen(buf.as_mut_ptr()) < padding_width as libc::c_ulong {
        let mut w: size_t = padding_width as size_t;
        mbsalign(
            buf.as_mut_ptr(),
            padding_buffer,
            padding_buffer_size,
            &mut w,
            padding_alignment,
            MBA_UNIBYTE_ONLY as libc::c_int,
        );
        if dev_debug {
            fprintf(
                stderr,
                b"  After padding: %s\n\0" as *const u8 as *const libc::c_char,
                quote(padding_buffer),
            );
        }
    } else {
        setup_padding_buffer(
            (strlen(buf.as_mut_ptr())).wrapping_add(1 as libc::c_int as libc::c_ulong),
        );
        strcpy(padding_buffer, buf.as_mut_ptr());
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn print_padded_number() {
    if !format_str_prefix.is_null() {
        fputs_unlocked(format_str_prefix, stdout);
    }
    fputs_unlocked(padding_buffer, stdout);
    if !format_str_suffix.is_null() {
        fputs_unlocked(format_str_suffix, stdout);
    }
}
unsafe extern "C" fn process_suffixed_number(
    mut text: *mut libc::c_char,
    mut result: *mut f128::f128,
    mut precision: *mut size_t,
    mut field: libc::c_long,
) -> libc::c_int {
    if !suffix.is_null() && strlen(text) > strlen(suffix) {
        let mut possible_suffix: *mut libc::c_char = text
            .offset(strlen(text) as isize)
            .offset(-(strlen(suffix) as isize));
        if strcmp(suffix, possible_suffix) == 0 as libc::c_int {
            *possible_suffix = '\0' as i32 as libc::c_char;
            if dev_debug {
                fprintf(
                    stderr,
                    b"trimming suffix %s\n\0" as *const u8 as *const libc::c_char,
                    quote(suffix),
                );
            }
        } else if dev_debug {
            fprintf(
                stderr,
                b"no valid suffix found\n\0" as *const u8 as *const libc::c_char,
            );
        }
    }
    let mut p: *mut libc::c_char = text;
    while *p as libc::c_int != 0
        && *(*__ctype_b_loc()).offset(to_uchar(*p) as libc::c_int as isize)
            as libc::c_int & _ISblank as libc::c_int as libc::c_ushort as libc::c_int
            != 0
    {
        p = p.offset(1);
    }
    let skip_count: libc::c_uint = text.offset_from(p) as libc::c_long as libc::c_uint;
    if auto_padding != 0 {
        if skip_count > 0 as libc::c_int as libc::c_uint
            || field > 1 as libc::c_int as libc::c_long
        {
            padding_width = strlen(text) as libc::c_long;
            setup_padding_buffer(padding_width as size_t);
        } else {
            padding_width = 0 as libc::c_int as libc::c_long;
        }
        if dev_debug {
            fprintf(
                stderr,
                b"setting Auto-Padding to %ld characters\n\0" as *const u8
                    as *const libc::c_char,
                padding_width,
            );
        }
    }
    let mut val: f128::f128 = f128::f128::new(0 as libc::c_int);
    let mut e: simple_strtod_error = parse_human_number(p, &mut val, precision);
    if e as libc::c_uint == SSE_OK_PRECISION_LOSS as libc::c_int as libc::c_uint
        && debug as libc::c_int != 0
    {
        error(
            0 as libc::c_int,
            0 as libc::c_int,
            dcgettext(
                0 as *const libc::c_char,
                b"large input value %s: possible precision loss\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
            quote(p),
        );
    }
    if from_unit_size != 1 as libc::c_int as libc::c_ulong
        || to_unit_size != 1 as libc::c_int as libc::c_ulong
    {
        val = val * f128::f128::new(from_unit_size) / f128::f128::new(to_unit_size);
    }
    *result = val;
    return (e as libc::c_uint == SSE_OK as libc::c_int as libc::c_uint
        || e as libc::c_uint == SSE_OK_PRECISION_LOSS as libc::c_int as libc::c_uint)
        as libc::c_int;
}
unsafe extern "C" fn next_field(mut line: *mut *mut libc::c_char) -> *mut libc::c_char {
    let mut field_start: *mut libc::c_char = *line;
    let mut field_end: *mut libc::c_char = field_start;
    if delimiter != DELIMITER_DEFAULT as libc::c_int {
        if *field_start as libc::c_int != delimiter {
            while *field_end as libc::c_int != 0
                && *field_end as libc::c_int != delimiter
            {
                field_end = field_end.offset(1);
            }
        }
    } else {
        while *field_end as libc::c_int != 0
            && field_sep(*field_end as libc::c_uchar) as libc::c_int != 0
        {
            field_end = field_end.offset(1);
        }
        while *field_end as libc::c_int != 0 && !field_sep(*field_end as libc::c_uchar) {
            field_end = field_end.offset(1);
        }
    }
    *line = field_end;
    return field_start;
}
unsafe extern "C" fn include_field(mut field: uintmax_t) -> bool {
    let mut p: *mut field_range_pair = frp;
    if p.is_null() {
        return field == 1 as libc::c_int as libc::c_ulong;
    }
    while (*p).lo != 18446744073709551615 as libc::c_ulong {
        if (*p).lo <= field && (*p).hi >= field {
            return 1 as libc::c_int != 0;
        }
        p = p.offset(1);
    }
    return 0 as libc::c_int != 0;
}
unsafe extern "C" fn process_field(
    mut text: *mut libc::c_char,
    mut field: uintmax_t,
) -> bool {
    let mut val: f128::f128 = f128::f128::new(0 as libc::c_int);
    let mut precision: size_t = 0 as libc::c_int as size_t;
    let mut valid_number: bool = 1 as libc::c_int != 0;
    if include_field(field) {
        valid_number = process_suffixed_number(
            text,
            &mut val,
            &mut precision,
            field as libc::c_long,
        ) != 0;
        if valid_number {
            valid_number = prepare_padded_number(val, precision) != 0;
        }
        if valid_number {
            print_padded_number();
        } else {
            fputs_unlocked(text, stdout);
        }
    } else {
        fputs_unlocked(text, stdout);
    }
    return valid_number;
}
unsafe extern "C" fn process_line(
    mut line: *mut libc::c_char,
    mut newline: bool,
) -> libc::c_int {
    let mut next: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut field: uintmax_t = 0 as libc::c_int as uintmax_t;
    let mut valid_number: bool = 1 as libc::c_int != 0;
    loop {
        field = field.wrapping_add(1);
        next = next_field(&mut line);
        if *line as libc::c_int != '\0' as i32 {
            *line = '\0' as i32 as libc::c_char;
            if !process_field(next, field) {
                valid_number = 0 as libc::c_int != 0;
            }
            fputc_unlocked(
                if delimiter == DELIMITER_DEFAULT as libc::c_int {
                    ' ' as i32
                } else {
                    delimiter
                },
                stdout,
            );
            line = line.offset(1);
        } else {
            if !process_field(next, field) {
                valid_number = 0 as libc::c_int != 0;
            }
            break;
        }
    }
    if newline {
        putchar_unlocked(line_delim as libc::c_int);
    }
    return valid_number as libc::c_int;
}
unsafe fn main_0(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut valid_numbers: libc::c_int = 1 as libc::c_int;
    let mut locale_ok: bool = false;
    set_program_name(*argv.offset(0 as libc::c_int as isize));
    locale_ok = !(setlocale(6 as libc::c_int, b"\0" as *const u8 as *const libc::c_char))
        .is_null();
    bindtextdomain(
        b"coreutils\0" as *const u8 as *const libc::c_char,
        b"/usr/local/share/locale\0" as *const u8 as *const libc::c_char,
    );
    textdomain(b"coreutils\0" as *const u8 as *const libc::c_char);
    decimal_point = nl_langinfo(RADIXCHAR as libc::c_int);
    if decimal_point.is_null()
        || strlen(decimal_point) == 0 as libc::c_int as libc::c_ulong
    {
        decimal_point = b".\0" as *const u8 as *const libc::c_char;
    }
    decimal_point_length = strlen(decimal_point) as libc::c_int;
    initialize_exit_failure(TIMEOUT_FAILURE as libc::c_int);
    atexit(Some(close_stdout as unsafe extern "C" fn() -> ()));
    loop {
        let mut c: libc::c_int = getopt_long(
            argc,
            argv,
            b"d:z\0" as *const u8 as *const libc::c_char,
            longopts.as_ptr(),
            0 as *mut libc::c_int,
        );
        if c == -(1 as libc::c_int) {
            break;
        }
        match c {
            256 => {
                scale_from = scale_from_types[__xargmatch_internal(
                    b"--from\0" as *const u8 as *const libc::c_char,
                    optarg,
                    scale_from_args.as_ptr(),
                    scale_from_types.as_ptr() as *const libc::c_void,
                    ::core::mem::size_of::<scale_type>() as libc::c_ulong,
                    argmatch_die,
                    1 as libc::c_int != 0,
                ) as usize];
            }
            257 => {
                from_unit_size = unit_to_umax(optarg);
            }
            258 => {
                scale_to = scale_to_types[__xargmatch_internal(
                    b"--to\0" as *const u8 as *const libc::c_char,
                    optarg,
                    scale_to_args.as_ptr(),
                    scale_to_types.as_ptr() as *const libc::c_void,
                    ::core::mem::size_of::<scale_type>() as libc::c_ulong,
                    argmatch_die,
                    1 as libc::c_int != 0,
                ) as usize];
            }
            259 => {
                to_unit_size = unit_to_umax(optarg);
            }
            260 => {
                round_style = round_types[__xargmatch_internal(
                    b"--round\0" as *const u8 as *const libc::c_char,
                    optarg,
                    round_args.as_ptr(),
                    round_types.as_ptr() as *const libc::c_void,
                    ::core::mem::size_of::<round_type>() as libc::c_ulong,
                    argmatch_die,
                    1 as libc::c_int != 0,
                ) as usize];
            }
            262 => {
                grouping = 1 as libc::c_int;
            }
            263 => {
                if xstrtol(
                    optarg,
                    0 as *mut *mut libc::c_char,
                    10 as libc::c_int,
                    &mut padding_width,
                    b"\0" as *const u8 as *const libc::c_char,
                ) as libc::c_uint != LONGINT_OK as libc::c_int as libc::c_uint
                    || padding_width == 0 as libc::c_int as libc::c_long
                    || padding_width < -(9223372036854775807 as libc::c_long)
                {
                    if ::core::mem::size_of::<C2RustUnnamed_23>() as libc::c_ulong != 0 {
                        error(
                            1 as libc::c_int,
                            0 as libc::c_int,
                            dcgettext(
                                0 as *const libc::c_char,
                                b"invalid padding value %s\0" as *const u8
                                    as *const libc::c_char,
                                5 as libc::c_int,
                            ),
                            quote(optarg),
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
                                b"invalid padding value %s\0" as *const u8
                                    as *const libc::c_char,
                                5 as libc::c_int,
                            ),
                            quote(optarg),
                        );
                        if 0 as libc::c_int != 0 {} else {
                            unreachable!();
                        };
                    };
                }
                if padding_width < 0 as libc::c_int as libc::c_long {
                    padding_alignment = MBS_ALIGN_LEFT;
                    padding_width = -padding_width;
                }
            }
            264 => {
                if n_frp != 0 {
                    if ::core::mem::size_of::<C2RustUnnamed_22>() as libc::c_ulong != 0 {
                        error(
                            1 as libc::c_int,
                            0 as libc::c_int,
                            dcgettext(
                                0 as *const libc::c_char,
                                b"multiple field specifications\0" as *const u8
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
                                b"multiple field specifications\0" as *const u8
                                    as *const libc::c_char,
                                5 as libc::c_int,
                            ),
                        );
                        if 0 as libc::c_int != 0 {} else {
                            unreachable!();
                        };
                    };
                }
                set_fields(optarg, SETFLD_ALLOW_DASH as libc::c_int as libc::c_uint);
            }
            100 => {
                if *optarg.offset(0 as libc::c_int as isize) as libc::c_int
                    != '\0' as i32
                    && *optarg.offset(1 as libc::c_int as isize) as libc::c_int
                        != '\0' as i32
                {
                    if ::core::mem::size_of::<C2RustUnnamed_21>() as libc::c_ulong != 0 {
                        error(
                            1 as libc::c_int,
                            0 as libc::c_int,
                            dcgettext(
                                0 as *const libc::c_char,
                                b"the delimiter must be a single character\0" as *const u8
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
                                b"the delimiter must be a single character\0" as *const u8
                                    as *const libc::c_char,
                                5 as libc::c_int,
                            ),
                        );
                        if 0 as libc::c_int != 0 {} else {
                            unreachable!();
                        };
                    };
                }
                delimiter = *optarg.offset(0 as libc::c_int as isize) as libc::c_int;
            }
            122 => {
                line_delim = '\0' as i32 as libc::c_uchar;
            }
            261 => {
                suffix = optarg;
            }
            265 => {
                debug = 1 as libc::c_int != 0;
            }
            266 => {
                dev_debug = 1 as libc::c_int != 0;
                debug = 1 as libc::c_int != 0;
            }
            267 => {
                if !optarg.is_null() {
                    if xstrtoumax(
                        optarg,
                        0 as *mut *mut libc::c_char,
                        10 as libc::c_int,
                        &mut header,
                        b"\0" as *const u8 as *const libc::c_char,
                    ) as libc::c_uint != LONGINT_OK as libc::c_int as libc::c_uint
                        || header == 0 as libc::c_int as libc::c_ulong
                    {
                        if ::core::mem::size_of::<C2RustUnnamed_20>() as libc::c_ulong
                            != 0
                        {
                            error(
                                1 as libc::c_int,
                                0 as libc::c_int,
                                dcgettext(
                                    0 as *const libc::c_char,
                                    b"invalid header value %s\0" as *const u8
                                        as *const libc::c_char,
                                    5 as libc::c_int,
                                ),
                                quote(optarg),
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
                                    b"invalid header value %s\0" as *const u8
                                        as *const libc::c_char,
                                    5 as libc::c_int,
                                ),
                                quote(optarg),
                            );
                            if 0 as libc::c_int != 0 {} else {
                                unreachable!();
                            };
                        };
                    }
                } else {
                    header = 1 as libc::c_int as uintmax_t;
                }
            }
            268 => {
                format_str = optarg;
            }
            269 => {
                inval_style = inval_types[__xargmatch_internal(
                    b"--invalid\0" as *const u8 as *const libc::c_char,
                    optarg,
                    inval_args.as_ptr(),
                    inval_types.as_ptr() as *const libc::c_void,
                    ::core::mem::size_of::<inval_type>() as libc::c_ulong,
                    argmatch_die,
                    1 as libc::c_int != 0,
                ) as usize];
            }
            -2 => {
                usage(0 as libc::c_int);
            }
            -3 => {
                version_etc(
                    stdout,
                    b"numfmt\0" as *const u8 as *const libc::c_char,
                    b"GNU coreutils\0" as *const u8 as *const libc::c_char,
                    Version,
                    b"Assaf Gordon\0" as *const u8 as *const libc::c_char,
                    0 as *mut libc::c_void as *mut libc::c_char,
                );
                exit(0 as libc::c_int);
            }
            _ => {
                usage(1 as libc::c_int);
            }
        }
    }
    if !format_str.is_null() && grouping != 0 {
        if ::core::mem::size_of::<C2RustUnnamed_19>() as libc::c_ulong != 0 {
            error(
                1 as libc::c_int,
                0 as libc::c_int,
                dcgettext(
                    0 as *const libc::c_char,
                    b"--grouping cannot be combined with --format\0" as *const u8
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
                    b"--grouping cannot be combined with --format\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
            );
            if 0 as libc::c_int != 0 {} else {
                unreachable!();
            };
        };
    }
    if debug as libc::c_int != 0 && !locale_ok {
        error(
            0 as libc::c_int,
            0 as libc::c_int,
            dcgettext(
                0 as *const libc::c_char,
                b"failed to set locale\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
    }
    if debug as libc::c_int != 0
        && scale_from as libc::c_uint == scale_none as libc::c_int as libc::c_uint
        && scale_to as libc::c_uint == scale_none as libc::c_int as libc::c_uint
        && grouping == 0 && padding_width == 0 as libc::c_int as libc::c_long
        && format_str.is_null()
    {
        error(
            0 as libc::c_int,
            0 as libc::c_int,
            dcgettext(
                0 as *const libc::c_char,
                b"no conversion option specified\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
    }
    if !format_str.is_null() {
        parse_format_string(format_str);
    }
    if grouping != 0 {
        if scale_to as libc::c_uint != scale_none as libc::c_int as libc::c_uint {
            if ::core::mem::size_of::<C2RustUnnamed_18>() as libc::c_ulong != 0 {
                error(
                    1 as libc::c_int,
                    0 as libc::c_int,
                    dcgettext(
                        0 as *const libc::c_char,
                        b"grouping cannot be combined with --to\0" as *const u8
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
                        b"grouping cannot be combined with --to\0" as *const u8
                            as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                );
                if 0 as libc::c_int != 0 {} else {
                    unreachable!();
                };
            };
        }
        if debug as libc::c_int != 0
            && strlen(nl_langinfo(THOUSEP as libc::c_int))
                == 0 as libc::c_int as libc::c_ulong
        {
            error(
                0 as libc::c_int,
                0 as libc::c_int,
                dcgettext(
                    0 as *const libc::c_char,
                    b"grouping has no effect in this locale\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
            );
        }
    }
    setup_padding_buffer(padding_width as size_t);
    auto_padding = (padding_width == 0 as libc::c_int as libc::c_long
        && delimiter == DELIMITER_DEFAULT as libc::c_int) as libc::c_int;
    if inval_style as libc::c_uint != inval_abort as libc::c_int as libc::c_uint {
        conv_exit_code = 0 as libc::c_int;
    }
    if argc > optind {
        if debug as libc::c_int != 0 && header != 0 {
            error(
                0 as libc::c_int,
                0 as libc::c_int,
                dcgettext(
                    0 as *const libc::c_char,
                    b"--header ignored with command-line input\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
            );
        }
        while optind < argc {
            valid_numbers
                &= process_line(*argv.offset(optind as isize), 1 as libc::c_int != 0);
            optind += 1;
        }
    } else {
        let mut line: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut line_allocated: size_t = 0 as libc::c_int as size_t;
        let mut len: ssize_t = 0;
        loop {
            let fresh4 = header;
            header = header.wrapping_sub(1);
            if !(fresh4 != 0
                && getdelim(
                    &mut line,
                    &mut line_allocated,
                    line_delim as libc::c_int,
                    stdin,
                ) > 0 as libc::c_int as libc::c_long)
            {
                break;
            }
            fputs_unlocked(line, stdout);
        }
        loop {
            len = getdelim(
                &mut line,
                &mut line_allocated,
                line_delim as libc::c_int,
                stdin,
            );
            if !(len > 0 as libc::c_int as libc::c_long) {
                break;
            }
            let mut newline: bool = *line
                .offset((len - 1 as libc::c_int as libc::c_long) as isize) as libc::c_int
                == line_delim as libc::c_int;
            if newline {
                *line
                    .offset(
                        (len - 1 as libc::c_int as libc::c_long) as isize,
                    ) = '\0' as i32 as libc::c_char;
            }
            valid_numbers &= process_line(line, newline);
        }
        if ferror_unlocked(stdin) != 0 {
            error(
                0 as libc::c_int,
                *__errno_location(),
                dcgettext(
                    0 as *const libc::c_char,
                    b"error reading input\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
            );
        }
    }
    if debug as libc::c_int != 0 && valid_numbers == 0 {
        error(
            0 as libc::c_int,
            0 as libc::c_int,
            dcgettext(
                0 as *const libc::c_char,
                b"failed to convert some of the input numbers\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
    }
    let mut exit_status: libc::c_int = 0 as libc::c_int;
    if valid_numbers == 0
        && inval_style as libc::c_uint != inval_warn as libc::c_int as libc::c_uint
        && inval_style as libc::c_uint != inval_ignore as libc::c_int as libc::c_uint
    {
        exit_status = EXIT_CONVERSION_WARNINGS as libc::c_int;
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
unsafe extern "C" fn run_static_initializers() {
    valid_suffixes = zero_and_valid_suffixes.as_ptr().offset(1 as libc::c_int as isize);
}
#[used]
#[cfg_attr(target_os = "linux", link_section = ".init_array")]
#[cfg_attr(target_os = "windows", link_section = ".CRT$XIB")]
#[cfg_attr(target_os = "macos", link_section = "__DATA,__mod_init_func")]
static INIT_ARRAY: [unsafe extern "C" fn(); 1] = [run_static_initializers];
