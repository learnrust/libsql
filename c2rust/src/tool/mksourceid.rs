use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn exit(_: libc::c_int) -> !;
    static mut stderr: *mut FILE;
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn fgets(
        __s: *mut libc::c_char,
        __n: libc::c_int,
        __stream: *mut FILE,
    ) -> *mut libc::c_char;
    fn fread(
        _: *mut libc::c_void,
        _: libc::c_ulong,
        _: libc::c_ulong,
        _: *mut FILE,
    ) -> libc::c_ulong;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strncmp(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
}
pub type size_t = libc::c_ulong;
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
pub type u64_0 = libc::c_ulonglong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SHA3Context {
    pub u: C2RustUnnamed_0,
    pub nRate: libc::c_uint,
    pub nLoaded: libc::c_uint,
    pub ixMask: libc::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_0 {
    pub s: [u64_0; 25],
    pub x: [libc::c_uchar; 1600],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SHA1Context {
    pub state: [libc::c_uint; 5],
    pub count: [libc::c_uint; 2],
    pub buffer: [libc::c_uchar; 64],
}
unsafe extern "C" fn KeccakF1600Step(mut p: *mut SHA3Context) {
    let mut i: libc::c_int = 0;
    let mut B0: u64_0 = 0;
    let mut B1: u64_0 = 0;
    let mut B2: u64_0 = 0;
    let mut B3: u64_0 = 0;
    let mut B4: u64_0 = 0;
    let mut C0: u64_0 = 0;
    let mut C1: u64_0 = 0;
    let mut C2: u64_0 = 0;
    let mut C3: u64_0 = 0;
    let mut C4: u64_0 = 0;
    let mut D0: u64_0 = 0;
    let mut D1: u64_0 = 0;
    let mut D2: u64_0 = 0;
    let mut D3: u64_0 = 0;
    let mut D4: u64_0 = 0;
    static mut RC: [u64_0; 24] = [
        0x1 as libc::c_ulonglong,
        0x8082 as libc::c_ulonglong,
        0x800000000000808a as libc::c_ulonglong,
        0x8000000080008000 as libc::c_ulonglong,
        0x808b as libc::c_ulonglong,
        0x80000001 as libc::c_ulonglong,
        0x8000000080008081 as libc::c_ulonglong,
        0x8000000000008009 as libc::c_ulonglong,
        0x8a as libc::c_ulonglong,
        0x88 as libc::c_ulonglong,
        0x80008009 as libc::c_ulonglong,
        0x8000000a as libc::c_ulonglong,
        0x8000808b as libc::c_ulonglong,
        0x800000000000008b as libc::c_ulonglong,
        0x8000000000008089 as libc::c_ulonglong,
        0x8000000000008003 as libc::c_ulonglong,
        0x8000000000008002 as libc::c_ulonglong,
        0x8000000000000080 as libc::c_ulonglong,
        0x800a as libc::c_ulonglong,
        0x800000008000000a as libc::c_ulonglong,
        0x8000000080008081 as libc::c_ulonglong,
        0x8000000000008080 as libc::c_ulonglong,
        0x80000001 as libc::c_ulonglong,
        0x8000000080008008 as libc::c_ulonglong,
    ];
    i = 0 as libc::c_int;
    while i < 24 as libc::c_int {
        C0 = (*p).u.s[0 as libc::c_int as usize] ^ (*p).u.s[5 as libc::c_int as usize]
            ^ (*p).u.s[10 as libc::c_int as usize] ^ (*p).u.s[15 as libc::c_int as usize]
            ^ (*p).u.s[20 as libc::c_int as usize];
        C1 = (*p).u.s[1 as libc::c_int as usize] ^ (*p).u.s[6 as libc::c_int as usize]
            ^ (*p).u.s[11 as libc::c_int as usize] ^ (*p).u.s[16 as libc::c_int as usize]
            ^ (*p).u.s[21 as libc::c_int as usize];
        C2 = (*p).u.s[2 as libc::c_int as usize] ^ (*p).u.s[7 as libc::c_int as usize]
            ^ (*p).u.s[12 as libc::c_int as usize] ^ (*p).u.s[17 as libc::c_int as usize]
            ^ (*p).u.s[22 as libc::c_int as usize];
        C3 = (*p).u.s[3 as libc::c_int as usize] ^ (*p).u.s[8 as libc::c_int as usize]
            ^ (*p).u.s[13 as libc::c_int as usize] ^ (*p).u.s[18 as libc::c_int as usize]
            ^ (*p).u.s[23 as libc::c_int as usize];
        C4 = (*p).u.s[4 as libc::c_int as usize] ^ (*p).u.s[9 as libc::c_int as usize]
            ^ (*p).u.s[14 as libc::c_int as usize] ^ (*p).u.s[19 as libc::c_int as usize]
            ^ (*p).u.s[24 as libc::c_int as usize];
        D0 = C4 ^ (C1 << 1 as libc::c_int | C1 >> 64 as libc::c_int - 1 as libc::c_int);
        D1 = C0 ^ (C2 << 1 as libc::c_int | C2 >> 64 as libc::c_int - 1 as libc::c_int);
        D2 = C1 ^ (C3 << 1 as libc::c_int | C3 >> 64 as libc::c_int - 1 as libc::c_int);
        D3 = C2 ^ (C4 << 1 as libc::c_int | C4 >> 64 as libc::c_int - 1 as libc::c_int);
        D4 = C3 ^ (C0 << 1 as libc::c_int | C0 >> 64 as libc::c_int - 1 as libc::c_int);
        B0 = (*p).u.s[0 as libc::c_int as usize] ^ D0;
        B1 = ((*p).u.s[6 as libc::c_int as usize] ^ D1) << 44 as libc::c_int
            | ((*p).u.s[6 as libc::c_int as usize] ^ D1)
                >> 64 as libc::c_int - 44 as libc::c_int;
        B2 = ((*p).u.s[12 as libc::c_int as usize] ^ D2) << 43 as libc::c_int
            | ((*p).u.s[12 as libc::c_int as usize] ^ D2)
                >> 64 as libc::c_int - 43 as libc::c_int;
        B3 = ((*p).u.s[18 as libc::c_int as usize] ^ D3) << 21 as libc::c_int
            | ((*p).u.s[18 as libc::c_int as usize] ^ D3)
                >> 64 as libc::c_int - 21 as libc::c_int;
        B4 = ((*p).u.s[24 as libc::c_int as usize] ^ D4) << 14 as libc::c_int
            | ((*p).u.s[24 as libc::c_int as usize] ^ D4)
                >> 64 as libc::c_int - 14 as libc::c_int;
        (*p).u.s[0 as libc::c_int as usize] = B0 ^ !B1 & B2;
        let ref mut fresh0 = (*p).u.s[0 as libc::c_int as usize];
        *fresh0 ^= RC[i as usize];
        (*p).u.s[6 as libc::c_int as usize] = B1 ^ !B2 & B3;
        (*p).u.s[12 as libc::c_int as usize] = B2 ^ !B3 & B4;
        (*p).u.s[18 as libc::c_int as usize] = B3 ^ !B4 & B0;
        (*p).u.s[24 as libc::c_int as usize] = B4 ^ !B0 & B1;
        B2 = ((*p).u.s[10 as libc::c_int as usize] ^ D0) << 3 as libc::c_int
            | ((*p).u.s[10 as libc::c_int as usize] ^ D0)
                >> 64 as libc::c_int - 3 as libc::c_int;
        B3 = ((*p).u.s[16 as libc::c_int as usize] ^ D1) << 45 as libc::c_int
            | ((*p).u.s[16 as libc::c_int as usize] ^ D1)
                >> 64 as libc::c_int - 45 as libc::c_int;
        B4 = ((*p).u.s[22 as libc::c_int as usize] ^ D2) << 61 as libc::c_int
            | ((*p).u.s[22 as libc::c_int as usize] ^ D2)
                >> 64 as libc::c_int - 61 as libc::c_int;
        B0 = ((*p).u.s[3 as libc::c_int as usize] ^ D3) << 28 as libc::c_int
            | ((*p).u.s[3 as libc::c_int as usize] ^ D3)
                >> 64 as libc::c_int - 28 as libc::c_int;
        B1 = ((*p).u.s[9 as libc::c_int as usize] ^ D4) << 20 as libc::c_int
            | ((*p).u.s[9 as libc::c_int as usize] ^ D4)
                >> 64 as libc::c_int - 20 as libc::c_int;
        (*p).u.s[10 as libc::c_int as usize] = B0 ^ !B1 & B2;
        (*p).u.s[16 as libc::c_int as usize] = B1 ^ !B2 & B3;
        (*p).u.s[22 as libc::c_int as usize] = B2 ^ !B3 & B4;
        (*p).u.s[3 as libc::c_int as usize] = B3 ^ !B4 & B0;
        (*p).u.s[9 as libc::c_int as usize] = B4 ^ !B0 & B1;
        B4 = ((*p).u.s[20 as libc::c_int as usize] ^ D0) << 18 as libc::c_int
            | ((*p).u.s[20 as libc::c_int as usize] ^ D0)
                >> 64 as libc::c_int - 18 as libc::c_int;
        B0 = ((*p).u.s[1 as libc::c_int as usize] ^ D1) << 1 as libc::c_int
            | ((*p).u.s[1 as libc::c_int as usize] ^ D1)
                >> 64 as libc::c_int - 1 as libc::c_int;
        B1 = ((*p).u.s[7 as libc::c_int as usize] ^ D2) << 6 as libc::c_int
            | ((*p).u.s[7 as libc::c_int as usize] ^ D2)
                >> 64 as libc::c_int - 6 as libc::c_int;
        B2 = ((*p).u.s[13 as libc::c_int as usize] ^ D3) << 25 as libc::c_int
            | ((*p).u.s[13 as libc::c_int as usize] ^ D3)
                >> 64 as libc::c_int - 25 as libc::c_int;
        B3 = ((*p).u.s[19 as libc::c_int as usize] ^ D4) << 8 as libc::c_int
            | ((*p).u.s[19 as libc::c_int as usize] ^ D4)
                >> 64 as libc::c_int - 8 as libc::c_int;
        (*p).u.s[20 as libc::c_int as usize] = B0 ^ !B1 & B2;
        (*p).u.s[1 as libc::c_int as usize] = B1 ^ !B2 & B3;
        (*p).u.s[7 as libc::c_int as usize] = B2 ^ !B3 & B4;
        (*p).u.s[13 as libc::c_int as usize] = B3 ^ !B4 & B0;
        (*p).u.s[19 as libc::c_int as usize] = B4 ^ !B0 & B1;
        B1 = ((*p).u.s[5 as libc::c_int as usize] ^ D0) << 36 as libc::c_int
            | ((*p).u.s[5 as libc::c_int as usize] ^ D0)
                >> 64 as libc::c_int - 36 as libc::c_int;
        B2 = ((*p).u.s[11 as libc::c_int as usize] ^ D1) << 10 as libc::c_int
            | ((*p).u.s[11 as libc::c_int as usize] ^ D1)
                >> 64 as libc::c_int - 10 as libc::c_int;
        B3 = ((*p).u.s[17 as libc::c_int as usize] ^ D2) << 15 as libc::c_int
            | ((*p).u.s[17 as libc::c_int as usize] ^ D2)
                >> 64 as libc::c_int - 15 as libc::c_int;
        B4 = ((*p).u.s[23 as libc::c_int as usize] ^ D3) << 56 as libc::c_int
            | ((*p).u.s[23 as libc::c_int as usize] ^ D3)
                >> 64 as libc::c_int - 56 as libc::c_int;
        B0 = ((*p).u.s[4 as libc::c_int as usize] ^ D4) << 27 as libc::c_int
            | ((*p).u.s[4 as libc::c_int as usize] ^ D4)
                >> 64 as libc::c_int - 27 as libc::c_int;
        (*p).u.s[5 as libc::c_int as usize] = B0 ^ !B1 & B2;
        (*p).u.s[11 as libc::c_int as usize] = B1 ^ !B2 & B3;
        (*p).u.s[17 as libc::c_int as usize] = B2 ^ !B3 & B4;
        (*p).u.s[23 as libc::c_int as usize] = B3 ^ !B4 & B0;
        (*p).u.s[4 as libc::c_int as usize] = B4 ^ !B0 & B1;
        B3 = ((*p).u.s[15 as libc::c_int as usize] ^ D0) << 41 as libc::c_int
            | ((*p).u.s[15 as libc::c_int as usize] ^ D0)
                >> 64 as libc::c_int - 41 as libc::c_int;
        B4 = ((*p).u.s[21 as libc::c_int as usize] ^ D1) << 2 as libc::c_int
            | ((*p).u.s[21 as libc::c_int as usize] ^ D1)
                >> 64 as libc::c_int - 2 as libc::c_int;
        B0 = ((*p).u.s[2 as libc::c_int as usize] ^ D2) << 62 as libc::c_int
            | ((*p).u.s[2 as libc::c_int as usize] ^ D2)
                >> 64 as libc::c_int - 62 as libc::c_int;
        B1 = ((*p).u.s[8 as libc::c_int as usize] ^ D3) << 55 as libc::c_int
            | ((*p).u.s[8 as libc::c_int as usize] ^ D3)
                >> 64 as libc::c_int - 55 as libc::c_int;
        B2 = ((*p).u.s[14 as libc::c_int as usize] ^ D4) << 39 as libc::c_int
            | ((*p).u.s[14 as libc::c_int as usize] ^ D4)
                >> 64 as libc::c_int - 39 as libc::c_int;
        (*p).u.s[15 as libc::c_int as usize] = B0 ^ !B1 & B2;
        (*p).u.s[21 as libc::c_int as usize] = B1 ^ !B2 & B3;
        (*p).u.s[2 as libc::c_int as usize] = B2 ^ !B3 & B4;
        (*p).u.s[8 as libc::c_int as usize] = B3 ^ !B4 & B0;
        (*p).u.s[14 as libc::c_int as usize] = B4 ^ !B0 & B1;
        C0 = (*p).u.s[0 as libc::c_int as usize] ^ (*p).u.s[10 as libc::c_int as usize]
            ^ (*p).u.s[20 as libc::c_int as usize] ^ (*p).u.s[5 as libc::c_int as usize]
            ^ (*p).u.s[15 as libc::c_int as usize];
        C1 = (*p).u.s[6 as libc::c_int as usize] ^ (*p).u.s[16 as libc::c_int as usize]
            ^ (*p).u.s[1 as libc::c_int as usize] ^ (*p).u.s[11 as libc::c_int as usize]
            ^ (*p).u.s[21 as libc::c_int as usize];
        C2 = (*p).u.s[12 as libc::c_int as usize] ^ (*p).u.s[22 as libc::c_int as usize]
            ^ (*p).u.s[7 as libc::c_int as usize] ^ (*p).u.s[17 as libc::c_int as usize]
            ^ (*p).u.s[2 as libc::c_int as usize];
        C3 = (*p).u.s[18 as libc::c_int as usize] ^ (*p).u.s[3 as libc::c_int as usize]
            ^ (*p).u.s[13 as libc::c_int as usize] ^ (*p).u.s[23 as libc::c_int as usize]
            ^ (*p).u.s[8 as libc::c_int as usize];
        C4 = (*p).u.s[24 as libc::c_int as usize] ^ (*p).u.s[9 as libc::c_int as usize]
            ^ (*p).u.s[19 as libc::c_int as usize] ^ (*p).u.s[4 as libc::c_int as usize]
            ^ (*p).u.s[14 as libc::c_int as usize];
        D0 = C4 ^ (C1 << 1 as libc::c_int | C1 >> 64 as libc::c_int - 1 as libc::c_int);
        D1 = C0 ^ (C2 << 1 as libc::c_int | C2 >> 64 as libc::c_int - 1 as libc::c_int);
        D2 = C1 ^ (C3 << 1 as libc::c_int | C3 >> 64 as libc::c_int - 1 as libc::c_int);
        D3 = C2 ^ (C4 << 1 as libc::c_int | C4 >> 64 as libc::c_int - 1 as libc::c_int);
        D4 = C3 ^ (C0 << 1 as libc::c_int | C0 >> 64 as libc::c_int - 1 as libc::c_int);
        B0 = (*p).u.s[0 as libc::c_int as usize] ^ D0;
        B1 = ((*p).u.s[16 as libc::c_int as usize] ^ D1) << 44 as libc::c_int
            | ((*p).u.s[16 as libc::c_int as usize] ^ D1)
                >> 64 as libc::c_int - 44 as libc::c_int;
        B2 = ((*p).u.s[7 as libc::c_int as usize] ^ D2) << 43 as libc::c_int
            | ((*p).u.s[7 as libc::c_int as usize] ^ D2)
                >> 64 as libc::c_int - 43 as libc::c_int;
        B3 = ((*p).u.s[23 as libc::c_int as usize] ^ D3) << 21 as libc::c_int
            | ((*p).u.s[23 as libc::c_int as usize] ^ D3)
                >> 64 as libc::c_int - 21 as libc::c_int;
        B4 = ((*p).u.s[14 as libc::c_int as usize] ^ D4) << 14 as libc::c_int
            | ((*p).u.s[14 as libc::c_int as usize] ^ D4)
                >> 64 as libc::c_int - 14 as libc::c_int;
        (*p).u.s[0 as libc::c_int as usize] = B0 ^ !B1 & B2;
        let ref mut fresh1 = (*p).u.s[0 as libc::c_int as usize];
        *fresh1 ^= RC[(i + 1 as libc::c_int) as usize];
        (*p).u.s[16 as libc::c_int as usize] = B1 ^ !B2 & B3;
        (*p).u.s[7 as libc::c_int as usize] = B2 ^ !B3 & B4;
        (*p).u.s[23 as libc::c_int as usize] = B3 ^ !B4 & B0;
        (*p).u.s[14 as libc::c_int as usize] = B4 ^ !B0 & B1;
        B2 = ((*p).u.s[20 as libc::c_int as usize] ^ D0) << 3 as libc::c_int
            | ((*p).u.s[20 as libc::c_int as usize] ^ D0)
                >> 64 as libc::c_int - 3 as libc::c_int;
        B3 = ((*p).u.s[11 as libc::c_int as usize] ^ D1) << 45 as libc::c_int
            | ((*p).u.s[11 as libc::c_int as usize] ^ D1)
                >> 64 as libc::c_int - 45 as libc::c_int;
        B4 = ((*p).u.s[2 as libc::c_int as usize] ^ D2) << 61 as libc::c_int
            | ((*p).u.s[2 as libc::c_int as usize] ^ D2)
                >> 64 as libc::c_int - 61 as libc::c_int;
        B0 = ((*p).u.s[18 as libc::c_int as usize] ^ D3) << 28 as libc::c_int
            | ((*p).u.s[18 as libc::c_int as usize] ^ D3)
                >> 64 as libc::c_int - 28 as libc::c_int;
        B1 = ((*p).u.s[9 as libc::c_int as usize] ^ D4) << 20 as libc::c_int
            | ((*p).u.s[9 as libc::c_int as usize] ^ D4)
                >> 64 as libc::c_int - 20 as libc::c_int;
        (*p).u.s[20 as libc::c_int as usize] = B0 ^ !B1 & B2;
        (*p).u.s[11 as libc::c_int as usize] = B1 ^ !B2 & B3;
        (*p).u.s[2 as libc::c_int as usize] = B2 ^ !B3 & B4;
        (*p).u.s[18 as libc::c_int as usize] = B3 ^ !B4 & B0;
        (*p).u.s[9 as libc::c_int as usize] = B4 ^ !B0 & B1;
        B4 = ((*p).u.s[15 as libc::c_int as usize] ^ D0) << 18 as libc::c_int
            | ((*p).u.s[15 as libc::c_int as usize] ^ D0)
                >> 64 as libc::c_int - 18 as libc::c_int;
        B0 = ((*p).u.s[6 as libc::c_int as usize] ^ D1) << 1 as libc::c_int
            | ((*p).u.s[6 as libc::c_int as usize] ^ D1)
                >> 64 as libc::c_int - 1 as libc::c_int;
        B1 = ((*p).u.s[22 as libc::c_int as usize] ^ D2) << 6 as libc::c_int
            | ((*p).u.s[22 as libc::c_int as usize] ^ D2)
                >> 64 as libc::c_int - 6 as libc::c_int;
        B2 = ((*p).u.s[13 as libc::c_int as usize] ^ D3) << 25 as libc::c_int
            | ((*p).u.s[13 as libc::c_int as usize] ^ D3)
                >> 64 as libc::c_int - 25 as libc::c_int;
        B3 = ((*p).u.s[4 as libc::c_int as usize] ^ D4) << 8 as libc::c_int
            | ((*p).u.s[4 as libc::c_int as usize] ^ D4)
                >> 64 as libc::c_int - 8 as libc::c_int;
        (*p).u.s[15 as libc::c_int as usize] = B0 ^ !B1 & B2;
        (*p).u.s[6 as libc::c_int as usize] = B1 ^ !B2 & B3;
        (*p).u.s[22 as libc::c_int as usize] = B2 ^ !B3 & B4;
        (*p).u.s[13 as libc::c_int as usize] = B3 ^ !B4 & B0;
        (*p).u.s[4 as libc::c_int as usize] = B4 ^ !B0 & B1;
        B1 = ((*p).u.s[10 as libc::c_int as usize] ^ D0) << 36 as libc::c_int
            | ((*p).u.s[10 as libc::c_int as usize] ^ D0)
                >> 64 as libc::c_int - 36 as libc::c_int;
        B2 = ((*p).u.s[1 as libc::c_int as usize] ^ D1) << 10 as libc::c_int
            | ((*p).u.s[1 as libc::c_int as usize] ^ D1)
                >> 64 as libc::c_int - 10 as libc::c_int;
        B3 = ((*p).u.s[17 as libc::c_int as usize] ^ D2) << 15 as libc::c_int
            | ((*p).u.s[17 as libc::c_int as usize] ^ D2)
                >> 64 as libc::c_int - 15 as libc::c_int;
        B4 = ((*p).u.s[8 as libc::c_int as usize] ^ D3) << 56 as libc::c_int
            | ((*p).u.s[8 as libc::c_int as usize] ^ D3)
                >> 64 as libc::c_int - 56 as libc::c_int;
        B0 = ((*p).u.s[24 as libc::c_int as usize] ^ D4) << 27 as libc::c_int
            | ((*p).u.s[24 as libc::c_int as usize] ^ D4)
                >> 64 as libc::c_int - 27 as libc::c_int;
        (*p).u.s[10 as libc::c_int as usize] = B0 ^ !B1 & B2;
        (*p).u.s[1 as libc::c_int as usize] = B1 ^ !B2 & B3;
        (*p).u.s[17 as libc::c_int as usize] = B2 ^ !B3 & B4;
        (*p).u.s[8 as libc::c_int as usize] = B3 ^ !B4 & B0;
        (*p).u.s[24 as libc::c_int as usize] = B4 ^ !B0 & B1;
        B3 = ((*p).u.s[5 as libc::c_int as usize] ^ D0) << 41 as libc::c_int
            | ((*p).u.s[5 as libc::c_int as usize] ^ D0)
                >> 64 as libc::c_int - 41 as libc::c_int;
        B4 = ((*p).u.s[21 as libc::c_int as usize] ^ D1) << 2 as libc::c_int
            | ((*p).u.s[21 as libc::c_int as usize] ^ D1)
                >> 64 as libc::c_int - 2 as libc::c_int;
        B0 = ((*p).u.s[12 as libc::c_int as usize] ^ D2) << 62 as libc::c_int
            | ((*p).u.s[12 as libc::c_int as usize] ^ D2)
                >> 64 as libc::c_int - 62 as libc::c_int;
        B1 = ((*p).u.s[3 as libc::c_int as usize] ^ D3) << 55 as libc::c_int
            | ((*p).u.s[3 as libc::c_int as usize] ^ D3)
                >> 64 as libc::c_int - 55 as libc::c_int;
        B2 = ((*p).u.s[19 as libc::c_int as usize] ^ D4) << 39 as libc::c_int
            | ((*p).u.s[19 as libc::c_int as usize] ^ D4)
                >> 64 as libc::c_int - 39 as libc::c_int;
        (*p).u.s[5 as libc::c_int as usize] = B0 ^ !B1 & B2;
        (*p).u.s[21 as libc::c_int as usize] = B1 ^ !B2 & B3;
        (*p).u.s[12 as libc::c_int as usize] = B2 ^ !B3 & B4;
        (*p).u.s[3 as libc::c_int as usize] = B3 ^ !B4 & B0;
        (*p).u.s[19 as libc::c_int as usize] = B4 ^ !B0 & B1;
        C0 = (*p).u.s[0 as libc::c_int as usize] ^ (*p).u.s[20 as libc::c_int as usize]
            ^ (*p).u.s[15 as libc::c_int as usize] ^ (*p).u.s[10 as libc::c_int as usize]
            ^ (*p).u.s[5 as libc::c_int as usize];
        C1 = (*p).u.s[16 as libc::c_int as usize] ^ (*p).u.s[11 as libc::c_int as usize]
            ^ (*p).u.s[6 as libc::c_int as usize] ^ (*p).u.s[1 as libc::c_int as usize]
            ^ (*p).u.s[21 as libc::c_int as usize];
        C2 = (*p).u.s[7 as libc::c_int as usize] ^ (*p).u.s[2 as libc::c_int as usize]
            ^ (*p).u.s[22 as libc::c_int as usize] ^ (*p).u.s[17 as libc::c_int as usize]
            ^ (*p).u.s[12 as libc::c_int as usize];
        C3 = (*p).u.s[23 as libc::c_int as usize] ^ (*p).u.s[18 as libc::c_int as usize]
            ^ (*p).u.s[13 as libc::c_int as usize] ^ (*p).u.s[8 as libc::c_int as usize]
            ^ (*p).u.s[3 as libc::c_int as usize];
        C4 = (*p).u.s[14 as libc::c_int as usize] ^ (*p).u.s[9 as libc::c_int as usize]
            ^ (*p).u.s[4 as libc::c_int as usize] ^ (*p).u.s[24 as libc::c_int as usize]
            ^ (*p).u.s[19 as libc::c_int as usize];
        D0 = C4 ^ (C1 << 1 as libc::c_int | C1 >> 64 as libc::c_int - 1 as libc::c_int);
        D1 = C0 ^ (C2 << 1 as libc::c_int | C2 >> 64 as libc::c_int - 1 as libc::c_int);
        D2 = C1 ^ (C3 << 1 as libc::c_int | C3 >> 64 as libc::c_int - 1 as libc::c_int);
        D3 = C2 ^ (C4 << 1 as libc::c_int | C4 >> 64 as libc::c_int - 1 as libc::c_int);
        D4 = C3 ^ (C0 << 1 as libc::c_int | C0 >> 64 as libc::c_int - 1 as libc::c_int);
        B0 = (*p).u.s[0 as libc::c_int as usize] ^ D0;
        B1 = ((*p).u.s[11 as libc::c_int as usize] ^ D1) << 44 as libc::c_int
            | ((*p).u.s[11 as libc::c_int as usize] ^ D1)
                >> 64 as libc::c_int - 44 as libc::c_int;
        B2 = ((*p).u.s[22 as libc::c_int as usize] ^ D2) << 43 as libc::c_int
            | ((*p).u.s[22 as libc::c_int as usize] ^ D2)
                >> 64 as libc::c_int - 43 as libc::c_int;
        B3 = ((*p).u.s[8 as libc::c_int as usize] ^ D3) << 21 as libc::c_int
            | ((*p).u.s[8 as libc::c_int as usize] ^ D3)
                >> 64 as libc::c_int - 21 as libc::c_int;
        B4 = ((*p).u.s[19 as libc::c_int as usize] ^ D4) << 14 as libc::c_int
            | ((*p).u.s[19 as libc::c_int as usize] ^ D4)
                >> 64 as libc::c_int - 14 as libc::c_int;
        (*p).u.s[0 as libc::c_int as usize] = B0 ^ !B1 & B2;
        let ref mut fresh2 = (*p).u.s[0 as libc::c_int as usize];
        *fresh2 ^= RC[(i + 2 as libc::c_int) as usize];
        (*p).u.s[11 as libc::c_int as usize] = B1 ^ !B2 & B3;
        (*p).u.s[22 as libc::c_int as usize] = B2 ^ !B3 & B4;
        (*p).u.s[8 as libc::c_int as usize] = B3 ^ !B4 & B0;
        (*p).u.s[19 as libc::c_int as usize] = B4 ^ !B0 & B1;
        B2 = ((*p).u.s[15 as libc::c_int as usize] ^ D0) << 3 as libc::c_int
            | ((*p).u.s[15 as libc::c_int as usize] ^ D0)
                >> 64 as libc::c_int - 3 as libc::c_int;
        B3 = ((*p).u.s[1 as libc::c_int as usize] ^ D1) << 45 as libc::c_int
            | ((*p).u.s[1 as libc::c_int as usize] ^ D1)
                >> 64 as libc::c_int - 45 as libc::c_int;
        B4 = ((*p).u.s[12 as libc::c_int as usize] ^ D2) << 61 as libc::c_int
            | ((*p).u.s[12 as libc::c_int as usize] ^ D2)
                >> 64 as libc::c_int - 61 as libc::c_int;
        B0 = ((*p).u.s[23 as libc::c_int as usize] ^ D3) << 28 as libc::c_int
            | ((*p).u.s[23 as libc::c_int as usize] ^ D3)
                >> 64 as libc::c_int - 28 as libc::c_int;
        B1 = ((*p).u.s[9 as libc::c_int as usize] ^ D4) << 20 as libc::c_int
            | ((*p).u.s[9 as libc::c_int as usize] ^ D4)
                >> 64 as libc::c_int - 20 as libc::c_int;
        (*p).u.s[15 as libc::c_int as usize] = B0 ^ !B1 & B2;
        (*p).u.s[1 as libc::c_int as usize] = B1 ^ !B2 & B3;
        (*p).u.s[12 as libc::c_int as usize] = B2 ^ !B3 & B4;
        (*p).u.s[23 as libc::c_int as usize] = B3 ^ !B4 & B0;
        (*p).u.s[9 as libc::c_int as usize] = B4 ^ !B0 & B1;
        B4 = ((*p).u.s[5 as libc::c_int as usize] ^ D0) << 18 as libc::c_int
            | ((*p).u.s[5 as libc::c_int as usize] ^ D0)
                >> 64 as libc::c_int - 18 as libc::c_int;
        B0 = ((*p).u.s[16 as libc::c_int as usize] ^ D1) << 1 as libc::c_int
            | ((*p).u.s[16 as libc::c_int as usize] ^ D1)
                >> 64 as libc::c_int - 1 as libc::c_int;
        B1 = ((*p).u.s[2 as libc::c_int as usize] ^ D2) << 6 as libc::c_int
            | ((*p).u.s[2 as libc::c_int as usize] ^ D2)
                >> 64 as libc::c_int - 6 as libc::c_int;
        B2 = ((*p).u.s[13 as libc::c_int as usize] ^ D3) << 25 as libc::c_int
            | ((*p).u.s[13 as libc::c_int as usize] ^ D3)
                >> 64 as libc::c_int - 25 as libc::c_int;
        B3 = ((*p).u.s[24 as libc::c_int as usize] ^ D4) << 8 as libc::c_int
            | ((*p).u.s[24 as libc::c_int as usize] ^ D4)
                >> 64 as libc::c_int - 8 as libc::c_int;
        (*p).u.s[5 as libc::c_int as usize] = B0 ^ !B1 & B2;
        (*p).u.s[16 as libc::c_int as usize] = B1 ^ !B2 & B3;
        (*p).u.s[2 as libc::c_int as usize] = B2 ^ !B3 & B4;
        (*p).u.s[13 as libc::c_int as usize] = B3 ^ !B4 & B0;
        (*p).u.s[24 as libc::c_int as usize] = B4 ^ !B0 & B1;
        B1 = ((*p).u.s[20 as libc::c_int as usize] ^ D0) << 36 as libc::c_int
            | ((*p).u.s[20 as libc::c_int as usize] ^ D0)
                >> 64 as libc::c_int - 36 as libc::c_int;
        B2 = ((*p).u.s[6 as libc::c_int as usize] ^ D1) << 10 as libc::c_int
            | ((*p).u.s[6 as libc::c_int as usize] ^ D1)
                >> 64 as libc::c_int - 10 as libc::c_int;
        B3 = ((*p).u.s[17 as libc::c_int as usize] ^ D2) << 15 as libc::c_int
            | ((*p).u.s[17 as libc::c_int as usize] ^ D2)
                >> 64 as libc::c_int - 15 as libc::c_int;
        B4 = ((*p).u.s[3 as libc::c_int as usize] ^ D3) << 56 as libc::c_int
            | ((*p).u.s[3 as libc::c_int as usize] ^ D3)
                >> 64 as libc::c_int - 56 as libc::c_int;
        B0 = ((*p).u.s[14 as libc::c_int as usize] ^ D4) << 27 as libc::c_int
            | ((*p).u.s[14 as libc::c_int as usize] ^ D4)
                >> 64 as libc::c_int - 27 as libc::c_int;
        (*p).u.s[20 as libc::c_int as usize] = B0 ^ !B1 & B2;
        (*p).u.s[6 as libc::c_int as usize] = B1 ^ !B2 & B3;
        (*p).u.s[17 as libc::c_int as usize] = B2 ^ !B3 & B4;
        (*p).u.s[3 as libc::c_int as usize] = B3 ^ !B4 & B0;
        (*p).u.s[14 as libc::c_int as usize] = B4 ^ !B0 & B1;
        B3 = ((*p).u.s[10 as libc::c_int as usize] ^ D0) << 41 as libc::c_int
            | ((*p).u.s[10 as libc::c_int as usize] ^ D0)
                >> 64 as libc::c_int - 41 as libc::c_int;
        B4 = ((*p).u.s[21 as libc::c_int as usize] ^ D1) << 2 as libc::c_int
            | ((*p).u.s[21 as libc::c_int as usize] ^ D1)
                >> 64 as libc::c_int - 2 as libc::c_int;
        B0 = ((*p).u.s[7 as libc::c_int as usize] ^ D2) << 62 as libc::c_int
            | ((*p).u.s[7 as libc::c_int as usize] ^ D2)
                >> 64 as libc::c_int - 62 as libc::c_int;
        B1 = ((*p).u.s[18 as libc::c_int as usize] ^ D3) << 55 as libc::c_int
            | ((*p).u.s[18 as libc::c_int as usize] ^ D3)
                >> 64 as libc::c_int - 55 as libc::c_int;
        B2 = ((*p).u.s[4 as libc::c_int as usize] ^ D4) << 39 as libc::c_int
            | ((*p).u.s[4 as libc::c_int as usize] ^ D4)
                >> 64 as libc::c_int - 39 as libc::c_int;
        (*p).u.s[10 as libc::c_int as usize] = B0 ^ !B1 & B2;
        (*p).u.s[21 as libc::c_int as usize] = B1 ^ !B2 & B3;
        (*p).u.s[7 as libc::c_int as usize] = B2 ^ !B3 & B4;
        (*p).u.s[18 as libc::c_int as usize] = B3 ^ !B4 & B0;
        (*p).u.s[4 as libc::c_int as usize] = B4 ^ !B0 & B1;
        C0 = (*p).u.s[0 as libc::c_int as usize] ^ (*p).u.s[15 as libc::c_int as usize]
            ^ (*p).u.s[5 as libc::c_int as usize] ^ (*p).u.s[20 as libc::c_int as usize]
            ^ (*p).u.s[10 as libc::c_int as usize];
        C1 = (*p).u.s[11 as libc::c_int as usize] ^ (*p).u.s[1 as libc::c_int as usize]
            ^ (*p).u.s[16 as libc::c_int as usize] ^ (*p).u.s[6 as libc::c_int as usize]
            ^ (*p).u.s[21 as libc::c_int as usize];
        C2 = (*p).u.s[22 as libc::c_int as usize] ^ (*p).u.s[12 as libc::c_int as usize]
            ^ (*p).u.s[2 as libc::c_int as usize] ^ (*p).u.s[17 as libc::c_int as usize]
            ^ (*p).u.s[7 as libc::c_int as usize];
        C3 = (*p).u.s[8 as libc::c_int as usize] ^ (*p).u.s[23 as libc::c_int as usize]
            ^ (*p).u.s[13 as libc::c_int as usize] ^ (*p).u.s[3 as libc::c_int as usize]
            ^ (*p).u.s[18 as libc::c_int as usize];
        C4 = (*p).u.s[19 as libc::c_int as usize] ^ (*p).u.s[9 as libc::c_int as usize]
            ^ (*p).u.s[24 as libc::c_int as usize] ^ (*p).u.s[14 as libc::c_int as usize]
            ^ (*p).u.s[4 as libc::c_int as usize];
        D0 = C4 ^ (C1 << 1 as libc::c_int | C1 >> 64 as libc::c_int - 1 as libc::c_int);
        D1 = C0 ^ (C2 << 1 as libc::c_int | C2 >> 64 as libc::c_int - 1 as libc::c_int);
        D2 = C1 ^ (C3 << 1 as libc::c_int | C3 >> 64 as libc::c_int - 1 as libc::c_int);
        D3 = C2 ^ (C4 << 1 as libc::c_int | C4 >> 64 as libc::c_int - 1 as libc::c_int);
        D4 = C3 ^ (C0 << 1 as libc::c_int | C0 >> 64 as libc::c_int - 1 as libc::c_int);
        B0 = (*p).u.s[0 as libc::c_int as usize] ^ D0;
        B1 = ((*p).u.s[1 as libc::c_int as usize] ^ D1) << 44 as libc::c_int
            | ((*p).u.s[1 as libc::c_int as usize] ^ D1)
                >> 64 as libc::c_int - 44 as libc::c_int;
        B2 = ((*p).u.s[2 as libc::c_int as usize] ^ D2) << 43 as libc::c_int
            | ((*p).u.s[2 as libc::c_int as usize] ^ D2)
                >> 64 as libc::c_int - 43 as libc::c_int;
        B3 = ((*p).u.s[3 as libc::c_int as usize] ^ D3) << 21 as libc::c_int
            | ((*p).u.s[3 as libc::c_int as usize] ^ D3)
                >> 64 as libc::c_int - 21 as libc::c_int;
        B4 = ((*p).u.s[4 as libc::c_int as usize] ^ D4) << 14 as libc::c_int
            | ((*p).u.s[4 as libc::c_int as usize] ^ D4)
                >> 64 as libc::c_int - 14 as libc::c_int;
        (*p).u.s[0 as libc::c_int as usize] = B0 ^ !B1 & B2;
        let ref mut fresh3 = (*p).u.s[0 as libc::c_int as usize];
        *fresh3 ^= RC[(i + 3 as libc::c_int) as usize];
        (*p).u.s[1 as libc::c_int as usize] = B1 ^ !B2 & B3;
        (*p).u.s[2 as libc::c_int as usize] = B2 ^ !B3 & B4;
        (*p).u.s[3 as libc::c_int as usize] = B3 ^ !B4 & B0;
        (*p).u.s[4 as libc::c_int as usize] = B4 ^ !B0 & B1;
        B2 = ((*p).u.s[5 as libc::c_int as usize] ^ D0) << 3 as libc::c_int
            | ((*p).u.s[5 as libc::c_int as usize] ^ D0)
                >> 64 as libc::c_int - 3 as libc::c_int;
        B3 = ((*p).u.s[6 as libc::c_int as usize] ^ D1) << 45 as libc::c_int
            | ((*p).u.s[6 as libc::c_int as usize] ^ D1)
                >> 64 as libc::c_int - 45 as libc::c_int;
        B4 = ((*p).u.s[7 as libc::c_int as usize] ^ D2) << 61 as libc::c_int
            | ((*p).u.s[7 as libc::c_int as usize] ^ D2)
                >> 64 as libc::c_int - 61 as libc::c_int;
        B0 = ((*p).u.s[8 as libc::c_int as usize] ^ D3) << 28 as libc::c_int
            | ((*p).u.s[8 as libc::c_int as usize] ^ D3)
                >> 64 as libc::c_int - 28 as libc::c_int;
        B1 = ((*p).u.s[9 as libc::c_int as usize] ^ D4) << 20 as libc::c_int
            | ((*p).u.s[9 as libc::c_int as usize] ^ D4)
                >> 64 as libc::c_int - 20 as libc::c_int;
        (*p).u.s[5 as libc::c_int as usize] = B0 ^ !B1 & B2;
        (*p).u.s[6 as libc::c_int as usize] = B1 ^ !B2 & B3;
        (*p).u.s[7 as libc::c_int as usize] = B2 ^ !B3 & B4;
        (*p).u.s[8 as libc::c_int as usize] = B3 ^ !B4 & B0;
        (*p).u.s[9 as libc::c_int as usize] = B4 ^ !B0 & B1;
        B4 = ((*p).u.s[10 as libc::c_int as usize] ^ D0) << 18 as libc::c_int
            | ((*p).u.s[10 as libc::c_int as usize] ^ D0)
                >> 64 as libc::c_int - 18 as libc::c_int;
        B0 = ((*p).u.s[11 as libc::c_int as usize] ^ D1) << 1 as libc::c_int
            | ((*p).u.s[11 as libc::c_int as usize] ^ D1)
                >> 64 as libc::c_int - 1 as libc::c_int;
        B1 = ((*p).u.s[12 as libc::c_int as usize] ^ D2) << 6 as libc::c_int
            | ((*p).u.s[12 as libc::c_int as usize] ^ D2)
                >> 64 as libc::c_int - 6 as libc::c_int;
        B2 = ((*p).u.s[13 as libc::c_int as usize] ^ D3) << 25 as libc::c_int
            | ((*p).u.s[13 as libc::c_int as usize] ^ D3)
                >> 64 as libc::c_int - 25 as libc::c_int;
        B3 = ((*p).u.s[14 as libc::c_int as usize] ^ D4) << 8 as libc::c_int
            | ((*p).u.s[14 as libc::c_int as usize] ^ D4)
                >> 64 as libc::c_int - 8 as libc::c_int;
        (*p).u.s[10 as libc::c_int as usize] = B0 ^ !B1 & B2;
        (*p).u.s[11 as libc::c_int as usize] = B1 ^ !B2 & B3;
        (*p).u.s[12 as libc::c_int as usize] = B2 ^ !B3 & B4;
        (*p).u.s[13 as libc::c_int as usize] = B3 ^ !B4 & B0;
        (*p).u.s[14 as libc::c_int as usize] = B4 ^ !B0 & B1;
        B1 = ((*p).u.s[15 as libc::c_int as usize] ^ D0) << 36 as libc::c_int
            | ((*p).u.s[15 as libc::c_int as usize] ^ D0)
                >> 64 as libc::c_int - 36 as libc::c_int;
        B2 = ((*p).u.s[16 as libc::c_int as usize] ^ D1) << 10 as libc::c_int
            | ((*p).u.s[16 as libc::c_int as usize] ^ D1)
                >> 64 as libc::c_int - 10 as libc::c_int;
        B3 = ((*p).u.s[17 as libc::c_int as usize] ^ D2) << 15 as libc::c_int
            | ((*p).u.s[17 as libc::c_int as usize] ^ D2)
                >> 64 as libc::c_int - 15 as libc::c_int;
        B4 = ((*p).u.s[18 as libc::c_int as usize] ^ D3) << 56 as libc::c_int
            | ((*p).u.s[18 as libc::c_int as usize] ^ D3)
                >> 64 as libc::c_int - 56 as libc::c_int;
        B0 = ((*p).u.s[19 as libc::c_int as usize] ^ D4) << 27 as libc::c_int
            | ((*p).u.s[19 as libc::c_int as usize] ^ D4)
                >> 64 as libc::c_int - 27 as libc::c_int;
        (*p).u.s[15 as libc::c_int as usize] = B0 ^ !B1 & B2;
        (*p).u.s[16 as libc::c_int as usize] = B1 ^ !B2 & B3;
        (*p).u.s[17 as libc::c_int as usize] = B2 ^ !B3 & B4;
        (*p).u.s[18 as libc::c_int as usize] = B3 ^ !B4 & B0;
        (*p).u.s[19 as libc::c_int as usize] = B4 ^ !B0 & B1;
        B3 = ((*p).u.s[20 as libc::c_int as usize] ^ D0) << 41 as libc::c_int
            | ((*p).u.s[20 as libc::c_int as usize] ^ D0)
                >> 64 as libc::c_int - 41 as libc::c_int;
        B4 = ((*p).u.s[21 as libc::c_int as usize] ^ D1) << 2 as libc::c_int
            | ((*p).u.s[21 as libc::c_int as usize] ^ D1)
                >> 64 as libc::c_int - 2 as libc::c_int;
        B0 = ((*p).u.s[22 as libc::c_int as usize] ^ D2) << 62 as libc::c_int
            | ((*p).u.s[22 as libc::c_int as usize] ^ D2)
                >> 64 as libc::c_int - 62 as libc::c_int;
        B1 = ((*p).u.s[23 as libc::c_int as usize] ^ D3) << 55 as libc::c_int
            | ((*p).u.s[23 as libc::c_int as usize] ^ D3)
                >> 64 as libc::c_int - 55 as libc::c_int;
        B2 = ((*p).u.s[24 as libc::c_int as usize] ^ D4) << 39 as libc::c_int
            | ((*p).u.s[24 as libc::c_int as usize] ^ D4)
                >> 64 as libc::c_int - 39 as libc::c_int;
        (*p).u.s[20 as libc::c_int as usize] = B0 ^ !B1 & B2;
        (*p).u.s[21 as libc::c_int as usize] = B1 ^ !B2 & B3;
        (*p).u.s[22 as libc::c_int as usize] = B2 ^ !B3 & B4;
        (*p).u.s[23 as libc::c_int as usize] = B3 ^ !B4 & B0;
        (*p).u.s[24 as libc::c_int as usize] = B4 ^ !B0 & B1;
        i += 4 as libc::c_int;
    }
}
unsafe extern "C" fn SHA3Init(mut p: *mut SHA3Context, mut iSize: libc::c_int) {
    memset(
        p as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<SHA3Context>() as libc::c_ulong,
    );
    if iSize >= 128 as libc::c_int && iSize <= 512 as libc::c_int {
        (*p)
            .nRate = ((1600 as libc::c_int
            - (iSize + 31 as libc::c_int & !(31 as libc::c_int)) * 2 as libc::c_int)
            / 8 as libc::c_int) as libc::c_uint;
    } else {
        (*p)
            .nRate = ((1600 as libc::c_int - 2 as libc::c_int * 256 as libc::c_int)
            / 8 as libc::c_int) as libc::c_uint;
    };
}
unsafe extern "C" fn SHA3Update(
    mut p: *mut SHA3Context,
    mut aData: *const libc::c_uchar,
    mut nData: libc::c_uint,
) {
    let mut i: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    if ((*p).nLoaded).wrapping_rem(8 as libc::c_int as libc::c_uint)
        == 0 as libc::c_int as libc::c_uint
        && aData.offset_from(0 as *const libc::c_uchar) as libc::c_long
            & 7 as libc::c_int as libc::c_long == 0 as libc::c_int as libc::c_long
    {
        while i.wrapping_add(7 as libc::c_int as libc::c_uint) < nData {
            let ref mut fresh4 = (*p)
                .u
                .s[((*p).nLoaded).wrapping_div(8 as libc::c_int as libc::c_uint)
                as usize];
            *fresh4
                ^= *(&*aData.offset(i as isize) as *const libc::c_uchar as *mut u64_0);
            let ref mut fresh5 = (*p).nLoaded;
            *fresh5 = (*fresh5).wrapping_add(8 as libc::c_int as libc::c_uint);
            if (*p).nLoaded >= (*p).nRate {
                KeccakF1600Step(p);
                (*p).nLoaded = 0 as libc::c_int as libc::c_uint;
            }
            i = i.wrapping_add(8 as libc::c_int as libc::c_uint);
        }
    }
    while i < nData {
        let ref mut fresh6 = (*p).u.x[(*p).nLoaded as usize];
        *fresh6 = (*fresh6 as libc::c_int ^ *aData.offset(i as isize) as libc::c_int)
            as libc::c_uchar;
        let ref mut fresh7 = (*p).nLoaded;
        *fresh7 = (*fresh7).wrapping_add(1);
        if (*p).nLoaded == (*p).nRate {
            KeccakF1600Step(p);
            (*p).nLoaded = 0 as libc::c_int as libc::c_uint;
        }
        i = i.wrapping_add(1);
    }
}
unsafe extern "C" fn SHA3Final(mut p: *mut SHA3Context) -> *mut libc::c_uchar {
    let mut i: libc::c_uint = 0;
    if (*p).nLoaded == ((*p).nRate).wrapping_sub(1 as libc::c_int as libc::c_uint) {
        let c1: libc::c_uchar = 0x86 as libc::c_int as libc::c_uchar;
        SHA3Update(p, &c1, 1 as libc::c_int as libc::c_uint);
    } else {
        let c2: libc::c_uchar = 0x6 as libc::c_int as libc::c_uchar;
        let c3: libc::c_uchar = 0x80 as libc::c_int as libc::c_uchar;
        SHA3Update(p, &c2, 1 as libc::c_int as libc::c_uint);
        (*p).nLoaded = ((*p).nRate).wrapping_sub(1 as libc::c_int as libc::c_uint);
        SHA3Update(p, &c3, 1 as libc::c_int as libc::c_uint);
    }
    i = 0 as libc::c_int as libc::c_uint;
    while i < (*p).nRate {
        (*p)
            .u
            .x[i.wrapping_add((*p).nRate)
            as usize] = (*p).u.x[(i ^ (*p).ixMask) as usize];
        i = i.wrapping_add(1);
    }
    return &mut *((*p).u.x).as_mut_ptr().offset((*p).nRate as isize)
        as *mut libc::c_uchar;
}
unsafe extern "C" fn DigestToBase16(
    mut digest: *mut libc::c_uchar,
    mut zBuf: *mut libc::c_char,
    mut nByte: libc::c_int,
) {
    static mut zEncode: [libc::c_char; 17] = unsafe {
        *::std::mem::transmute::<&[u8; 17], &[libc::c_char; 17]>(b"0123456789abcdef\0")
    };
    let mut ix: libc::c_int = 0;
    ix = 0 as libc::c_int;
    while ix < nByte {
        let fresh8 = zBuf;
        zBuf = zBuf.offset(1);
        *fresh8 = zEncode[(*digest as libc::c_int >> 4 as libc::c_int
            & 0xf as libc::c_int) as usize];
        let fresh9 = digest;
        digest = digest.offset(1);
        let fresh10 = zBuf;
        zBuf = zBuf.offset(1);
        *fresh10 = zEncode[(*fresh9 as libc::c_int & 0xf as libc::c_int) as usize];
        ix += 1;
    }
    *zBuf = '\0' as i32 as libc::c_char;
}
unsafe extern "C" fn sha3sum_file(
    mut zFilename: *const libc::c_char,
    mut iSize: libc::c_int,
    mut pCksum: *mut libc::c_char,
) -> libc::c_int {
    let mut in_0: *mut FILE = 0 as *mut FILE;
    let mut ctx: SHA3Context = SHA3Context {
        u: C2RustUnnamed_0 { s: [0; 25] },
        nRate: 0,
        nLoaded: 0,
        ixMask: 0,
    };
    let mut zBuf: [libc::c_char; 10240] = [0; 10240];
    in_0 = fopen(zFilename, b"rb\0" as *const u8 as *const libc::c_char);
    if in_0.is_null() {
        return 1 as libc::c_int;
    }
    SHA3Init(&mut ctx, iSize);
    loop {
        let mut n: libc::c_int = fread(
            zBuf.as_mut_ptr() as *mut libc::c_void,
            1 as libc::c_int as libc::c_ulong,
            ::std::mem::size_of::<[libc::c_char; 10240]>() as libc::c_ulong,
            in_0,
        ) as libc::c_int;
        if n <= 0 as libc::c_int {
            break;
        }
        SHA3Update(&mut ctx, zBuf.as_mut_ptr() as *mut libc::c_uchar, n as libc::c_uint);
    }
    fclose(in_0);
    DigestToBase16(SHA3Final(&mut ctx), pCksum, iSize / 8 as libc::c_int);
    return 0 as libc::c_int;
}
unsafe extern "C" fn SHA1Transform(
    mut state: *mut libc::c_uint,
    mut buffer: *const libc::c_uchar,
) {
    let mut qq: [libc::c_uint; 5] = [0; 5];
    static mut one: libc::c_int = 1 as libc::c_int;
    let mut block: [libc::c_uint; 16] = [0; 16];
    memcpy(
        block.as_mut_ptr() as *mut libc::c_void,
        buffer as *const libc::c_void,
        64 as libc::c_int as libc::c_ulong,
    );
    memcpy(
        qq.as_mut_ptr() as *mut libc::c_void,
        state as *const libc::c_void,
        (5 as libc::c_int as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<libc::c_uint>() as libc::c_ulong),
    );
    if 1 as libc::c_int
        == *(&mut one as *mut libc::c_int as *mut libc::c_uchar) as libc::c_int
    {
        block[0 as libc::c_int
            as usize] = (block[0 as libc::c_int as usize]
            << 32 as libc::c_int - 8 as libc::c_int
            | block[0 as libc::c_int as usize] >> 8 as libc::c_int)
            & 0xff00ff00 as libc::c_uint
            | (block[0 as libc::c_int as usize] << 8 as libc::c_int
                | block[0 as libc::c_int as usize]
                    >> 32 as libc::c_int - 8 as libc::c_int)
                & 0xff00ff as libc::c_int as libc::c_uint;
        qq[4 as libc::c_int
            as usize] = (qq[4 as libc::c_int as usize])
            .wrapping_add(
                (qq[1 as libc::c_int as usize]
                    & (qq[2 as libc::c_int as usize] ^ qq[3 as libc::c_int as usize])
                    ^ qq[3 as libc::c_int as usize])
                    .wrapping_add(block[0 as libc::c_int as usize])
                    .wrapping_add(0x5a827999 as libc::c_int as libc::c_uint)
                    .wrapping_add(
                        qq[0 as libc::c_int as usize] << 5 as libc::c_int
                            | qq[0 as libc::c_int as usize]
                                >> 32 as libc::c_int - 5 as libc::c_int,
                    ),
            );
        qq[1 as libc::c_int
            as usize] = qq[1 as libc::c_int as usize]
            << 32 as libc::c_int - 2 as libc::c_int
            | qq[1 as libc::c_int as usize] >> 2 as libc::c_int;
        block[1 as libc::c_int
            as usize] = (block[1 as libc::c_int as usize]
            << 32 as libc::c_int - 8 as libc::c_int
            | block[1 as libc::c_int as usize] >> 8 as libc::c_int)
            & 0xff00ff00 as libc::c_uint
            | (block[1 as libc::c_int as usize] << 8 as libc::c_int
                | block[1 as libc::c_int as usize]
                    >> 32 as libc::c_int - 8 as libc::c_int)
                & 0xff00ff as libc::c_int as libc::c_uint;
        qq[3 as libc::c_int
            as usize] = (qq[3 as libc::c_int as usize])
            .wrapping_add(
                (qq[0 as libc::c_int as usize]
                    & (qq[1 as libc::c_int as usize] ^ qq[2 as libc::c_int as usize])
                    ^ qq[2 as libc::c_int as usize])
                    .wrapping_add(block[1 as libc::c_int as usize])
                    .wrapping_add(0x5a827999 as libc::c_int as libc::c_uint)
                    .wrapping_add(
                        qq[4 as libc::c_int as usize] << 5 as libc::c_int
                            | qq[4 as libc::c_int as usize]
                                >> 32 as libc::c_int - 5 as libc::c_int,
                    ),
            );
        qq[0 as libc::c_int
            as usize] = qq[0 as libc::c_int as usize]
            << 32 as libc::c_int - 2 as libc::c_int
            | qq[0 as libc::c_int as usize] >> 2 as libc::c_int;
        block[2 as libc::c_int
            as usize] = (block[2 as libc::c_int as usize]
            << 32 as libc::c_int - 8 as libc::c_int
            | block[2 as libc::c_int as usize] >> 8 as libc::c_int)
            & 0xff00ff00 as libc::c_uint
            | (block[2 as libc::c_int as usize] << 8 as libc::c_int
                | block[2 as libc::c_int as usize]
                    >> 32 as libc::c_int - 8 as libc::c_int)
                & 0xff00ff as libc::c_int as libc::c_uint;
        qq[2 as libc::c_int
            as usize] = (qq[2 as libc::c_int as usize])
            .wrapping_add(
                (qq[4 as libc::c_int as usize]
                    & (qq[0 as libc::c_int as usize] ^ qq[1 as libc::c_int as usize])
                    ^ qq[1 as libc::c_int as usize])
                    .wrapping_add(block[2 as libc::c_int as usize])
                    .wrapping_add(0x5a827999 as libc::c_int as libc::c_uint)
                    .wrapping_add(
                        qq[3 as libc::c_int as usize] << 5 as libc::c_int
                            | qq[3 as libc::c_int as usize]
                                >> 32 as libc::c_int - 5 as libc::c_int,
                    ),
            );
        qq[4 as libc::c_int
            as usize] = qq[4 as libc::c_int as usize]
            << 32 as libc::c_int - 2 as libc::c_int
            | qq[4 as libc::c_int as usize] >> 2 as libc::c_int;
        block[3 as libc::c_int
            as usize] = (block[3 as libc::c_int as usize]
            << 32 as libc::c_int - 8 as libc::c_int
            | block[3 as libc::c_int as usize] >> 8 as libc::c_int)
            & 0xff00ff00 as libc::c_uint
            | (block[3 as libc::c_int as usize] << 8 as libc::c_int
                | block[3 as libc::c_int as usize]
                    >> 32 as libc::c_int - 8 as libc::c_int)
                & 0xff00ff as libc::c_int as libc::c_uint;
        qq[1 as libc::c_int
            as usize] = (qq[1 as libc::c_int as usize])
            .wrapping_add(
                (qq[3 as libc::c_int as usize]
                    & (qq[4 as libc::c_int as usize] ^ qq[0 as libc::c_int as usize])
                    ^ qq[0 as libc::c_int as usize])
                    .wrapping_add(block[3 as libc::c_int as usize])
                    .wrapping_add(0x5a827999 as libc::c_int as libc::c_uint)
                    .wrapping_add(
                        qq[2 as libc::c_int as usize] << 5 as libc::c_int
                            | qq[2 as libc::c_int as usize]
                                >> 32 as libc::c_int - 5 as libc::c_int,
                    ),
            );
        qq[3 as libc::c_int
            as usize] = qq[3 as libc::c_int as usize]
            << 32 as libc::c_int - 2 as libc::c_int
            | qq[3 as libc::c_int as usize] >> 2 as libc::c_int;
        block[4 as libc::c_int
            as usize] = (block[4 as libc::c_int as usize]
            << 32 as libc::c_int - 8 as libc::c_int
            | block[4 as libc::c_int as usize] >> 8 as libc::c_int)
            & 0xff00ff00 as libc::c_uint
            | (block[4 as libc::c_int as usize] << 8 as libc::c_int
                | block[4 as libc::c_int as usize]
                    >> 32 as libc::c_int - 8 as libc::c_int)
                & 0xff00ff as libc::c_int as libc::c_uint;
        qq[0 as libc::c_int
            as usize] = (qq[0 as libc::c_int as usize])
            .wrapping_add(
                (qq[2 as libc::c_int as usize]
                    & (qq[3 as libc::c_int as usize] ^ qq[4 as libc::c_int as usize])
                    ^ qq[4 as libc::c_int as usize])
                    .wrapping_add(block[4 as libc::c_int as usize])
                    .wrapping_add(0x5a827999 as libc::c_int as libc::c_uint)
                    .wrapping_add(
                        qq[1 as libc::c_int as usize] << 5 as libc::c_int
                            | qq[1 as libc::c_int as usize]
                                >> 32 as libc::c_int - 5 as libc::c_int,
                    ),
            );
        qq[2 as libc::c_int
            as usize] = qq[2 as libc::c_int as usize]
            << 32 as libc::c_int - 2 as libc::c_int
            | qq[2 as libc::c_int as usize] >> 2 as libc::c_int;
        block[5 as libc::c_int
            as usize] = (block[5 as libc::c_int as usize]
            << 32 as libc::c_int - 8 as libc::c_int
            | block[5 as libc::c_int as usize] >> 8 as libc::c_int)
            & 0xff00ff00 as libc::c_uint
            | (block[5 as libc::c_int as usize] << 8 as libc::c_int
                | block[5 as libc::c_int as usize]
                    >> 32 as libc::c_int - 8 as libc::c_int)
                & 0xff00ff as libc::c_int as libc::c_uint;
        qq[4 as libc::c_int
            as usize] = (qq[4 as libc::c_int as usize])
            .wrapping_add(
                (qq[1 as libc::c_int as usize]
                    & (qq[2 as libc::c_int as usize] ^ qq[3 as libc::c_int as usize])
                    ^ qq[3 as libc::c_int as usize])
                    .wrapping_add(block[5 as libc::c_int as usize])
                    .wrapping_add(0x5a827999 as libc::c_int as libc::c_uint)
                    .wrapping_add(
                        qq[0 as libc::c_int as usize] << 5 as libc::c_int
                            | qq[0 as libc::c_int as usize]
                                >> 32 as libc::c_int - 5 as libc::c_int,
                    ),
            );
        qq[1 as libc::c_int
            as usize] = qq[1 as libc::c_int as usize]
            << 32 as libc::c_int - 2 as libc::c_int
            | qq[1 as libc::c_int as usize] >> 2 as libc::c_int;
        block[6 as libc::c_int
            as usize] = (block[6 as libc::c_int as usize]
            << 32 as libc::c_int - 8 as libc::c_int
            | block[6 as libc::c_int as usize] >> 8 as libc::c_int)
            & 0xff00ff00 as libc::c_uint
            | (block[6 as libc::c_int as usize] << 8 as libc::c_int
                | block[6 as libc::c_int as usize]
                    >> 32 as libc::c_int - 8 as libc::c_int)
                & 0xff00ff as libc::c_int as libc::c_uint;
        qq[3 as libc::c_int
            as usize] = (qq[3 as libc::c_int as usize])
            .wrapping_add(
                (qq[0 as libc::c_int as usize]
                    & (qq[1 as libc::c_int as usize] ^ qq[2 as libc::c_int as usize])
                    ^ qq[2 as libc::c_int as usize])
                    .wrapping_add(block[6 as libc::c_int as usize])
                    .wrapping_add(0x5a827999 as libc::c_int as libc::c_uint)
                    .wrapping_add(
                        qq[4 as libc::c_int as usize] << 5 as libc::c_int
                            | qq[4 as libc::c_int as usize]
                                >> 32 as libc::c_int - 5 as libc::c_int,
                    ),
            );
        qq[0 as libc::c_int
            as usize] = qq[0 as libc::c_int as usize]
            << 32 as libc::c_int - 2 as libc::c_int
            | qq[0 as libc::c_int as usize] >> 2 as libc::c_int;
        block[7 as libc::c_int
            as usize] = (block[7 as libc::c_int as usize]
            << 32 as libc::c_int - 8 as libc::c_int
            | block[7 as libc::c_int as usize] >> 8 as libc::c_int)
            & 0xff00ff00 as libc::c_uint
            | (block[7 as libc::c_int as usize] << 8 as libc::c_int
                | block[7 as libc::c_int as usize]
                    >> 32 as libc::c_int - 8 as libc::c_int)
                & 0xff00ff as libc::c_int as libc::c_uint;
        qq[2 as libc::c_int
            as usize] = (qq[2 as libc::c_int as usize])
            .wrapping_add(
                (qq[4 as libc::c_int as usize]
                    & (qq[0 as libc::c_int as usize] ^ qq[1 as libc::c_int as usize])
                    ^ qq[1 as libc::c_int as usize])
                    .wrapping_add(block[7 as libc::c_int as usize])
                    .wrapping_add(0x5a827999 as libc::c_int as libc::c_uint)
                    .wrapping_add(
                        qq[3 as libc::c_int as usize] << 5 as libc::c_int
                            | qq[3 as libc::c_int as usize]
                                >> 32 as libc::c_int - 5 as libc::c_int,
                    ),
            );
        qq[4 as libc::c_int
            as usize] = qq[4 as libc::c_int as usize]
            << 32 as libc::c_int - 2 as libc::c_int
            | qq[4 as libc::c_int as usize] >> 2 as libc::c_int;
        block[8 as libc::c_int
            as usize] = (block[8 as libc::c_int as usize]
            << 32 as libc::c_int - 8 as libc::c_int
            | block[8 as libc::c_int as usize] >> 8 as libc::c_int)
            & 0xff00ff00 as libc::c_uint
            | (block[8 as libc::c_int as usize] << 8 as libc::c_int
                | block[8 as libc::c_int as usize]
                    >> 32 as libc::c_int - 8 as libc::c_int)
                & 0xff00ff as libc::c_int as libc::c_uint;
        qq[1 as libc::c_int
            as usize] = (qq[1 as libc::c_int as usize])
            .wrapping_add(
                (qq[3 as libc::c_int as usize]
                    & (qq[4 as libc::c_int as usize] ^ qq[0 as libc::c_int as usize])
                    ^ qq[0 as libc::c_int as usize])
                    .wrapping_add(block[8 as libc::c_int as usize])
                    .wrapping_add(0x5a827999 as libc::c_int as libc::c_uint)
                    .wrapping_add(
                        qq[2 as libc::c_int as usize] << 5 as libc::c_int
                            | qq[2 as libc::c_int as usize]
                                >> 32 as libc::c_int - 5 as libc::c_int,
                    ),
            );
        qq[3 as libc::c_int
            as usize] = qq[3 as libc::c_int as usize]
            << 32 as libc::c_int - 2 as libc::c_int
            | qq[3 as libc::c_int as usize] >> 2 as libc::c_int;
        block[9 as libc::c_int
            as usize] = (block[9 as libc::c_int as usize]
            << 32 as libc::c_int - 8 as libc::c_int
            | block[9 as libc::c_int as usize] >> 8 as libc::c_int)
            & 0xff00ff00 as libc::c_uint
            | (block[9 as libc::c_int as usize] << 8 as libc::c_int
                | block[9 as libc::c_int as usize]
                    >> 32 as libc::c_int - 8 as libc::c_int)
                & 0xff00ff as libc::c_int as libc::c_uint;
        qq[0 as libc::c_int
            as usize] = (qq[0 as libc::c_int as usize])
            .wrapping_add(
                (qq[2 as libc::c_int as usize]
                    & (qq[3 as libc::c_int as usize] ^ qq[4 as libc::c_int as usize])
                    ^ qq[4 as libc::c_int as usize])
                    .wrapping_add(block[9 as libc::c_int as usize])
                    .wrapping_add(0x5a827999 as libc::c_int as libc::c_uint)
                    .wrapping_add(
                        qq[1 as libc::c_int as usize] << 5 as libc::c_int
                            | qq[1 as libc::c_int as usize]
                                >> 32 as libc::c_int - 5 as libc::c_int,
                    ),
            );
        qq[2 as libc::c_int
            as usize] = qq[2 as libc::c_int as usize]
            << 32 as libc::c_int - 2 as libc::c_int
            | qq[2 as libc::c_int as usize] >> 2 as libc::c_int;
        block[10 as libc::c_int
            as usize] = (block[10 as libc::c_int as usize]
            << 32 as libc::c_int - 8 as libc::c_int
            | block[10 as libc::c_int as usize] >> 8 as libc::c_int)
            & 0xff00ff00 as libc::c_uint
            | (block[10 as libc::c_int as usize] << 8 as libc::c_int
                | block[10 as libc::c_int as usize]
                    >> 32 as libc::c_int - 8 as libc::c_int)
                & 0xff00ff as libc::c_int as libc::c_uint;
        qq[4 as libc::c_int
            as usize] = (qq[4 as libc::c_int as usize])
            .wrapping_add(
                (qq[1 as libc::c_int as usize]
                    & (qq[2 as libc::c_int as usize] ^ qq[3 as libc::c_int as usize])
                    ^ qq[3 as libc::c_int as usize])
                    .wrapping_add(block[10 as libc::c_int as usize])
                    .wrapping_add(0x5a827999 as libc::c_int as libc::c_uint)
                    .wrapping_add(
                        qq[0 as libc::c_int as usize] << 5 as libc::c_int
                            | qq[0 as libc::c_int as usize]
                                >> 32 as libc::c_int - 5 as libc::c_int,
                    ),
            );
        qq[1 as libc::c_int
            as usize] = qq[1 as libc::c_int as usize]
            << 32 as libc::c_int - 2 as libc::c_int
            | qq[1 as libc::c_int as usize] >> 2 as libc::c_int;
        block[11 as libc::c_int
            as usize] = (block[11 as libc::c_int as usize]
            << 32 as libc::c_int - 8 as libc::c_int
            | block[11 as libc::c_int as usize] >> 8 as libc::c_int)
            & 0xff00ff00 as libc::c_uint
            | (block[11 as libc::c_int as usize] << 8 as libc::c_int
                | block[11 as libc::c_int as usize]
                    >> 32 as libc::c_int - 8 as libc::c_int)
                & 0xff00ff as libc::c_int as libc::c_uint;
        qq[3 as libc::c_int
            as usize] = (qq[3 as libc::c_int as usize])
            .wrapping_add(
                (qq[0 as libc::c_int as usize]
                    & (qq[1 as libc::c_int as usize] ^ qq[2 as libc::c_int as usize])
                    ^ qq[2 as libc::c_int as usize])
                    .wrapping_add(block[11 as libc::c_int as usize])
                    .wrapping_add(0x5a827999 as libc::c_int as libc::c_uint)
                    .wrapping_add(
                        qq[4 as libc::c_int as usize] << 5 as libc::c_int
                            | qq[4 as libc::c_int as usize]
                                >> 32 as libc::c_int - 5 as libc::c_int,
                    ),
            );
        qq[0 as libc::c_int
            as usize] = qq[0 as libc::c_int as usize]
            << 32 as libc::c_int - 2 as libc::c_int
            | qq[0 as libc::c_int as usize] >> 2 as libc::c_int;
        block[12 as libc::c_int
            as usize] = (block[12 as libc::c_int as usize]
            << 32 as libc::c_int - 8 as libc::c_int
            | block[12 as libc::c_int as usize] >> 8 as libc::c_int)
            & 0xff00ff00 as libc::c_uint
            | (block[12 as libc::c_int as usize] << 8 as libc::c_int
                | block[12 as libc::c_int as usize]
                    >> 32 as libc::c_int - 8 as libc::c_int)
                & 0xff00ff as libc::c_int as libc::c_uint;
        qq[2 as libc::c_int
            as usize] = (qq[2 as libc::c_int as usize])
            .wrapping_add(
                (qq[4 as libc::c_int as usize]
                    & (qq[0 as libc::c_int as usize] ^ qq[1 as libc::c_int as usize])
                    ^ qq[1 as libc::c_int as usize])
                    .wrapping_add(block[12 as libc::c_int as usize])
                    .wrapping_add(0x5a827999 as libc::c_int as libc::c_uint)
                    .wrapping_add(
                        qq[3 as libc::c_int as usize] << 5 as libc::c_int
                            | qq[3 as libc::c_int as usize]
                                >> 32 as libc::c_int - 5 as libc::c_int,
                    ),
            );
        qq[4 as libc::c_int
            as usize] = qq[4 as libc::c_int as usize]
            << 32 as libc::c_int - 2 as libc::c_int
            | qq[4 as libc::c_int as usize] >> 2 as libc::c_int;
        block[13 as libc::c_int
            as usize] = (block[13 as libc::c_int as usize]
            << 32 as libc::c_int - 8 as libc::c_int
            | block[13 as libc::c_int as usize] >> 8 as libc::c_int)
            & 0xff00ff00 as libc::c_uint
            | (block[13 as libc::c_int as usize] << 8 as libc::c_int
                | block[13 as libc::c_int as usize]
                    >> 32 as libc::c_int - 8 as libc::c_int)
                & 0xff00ff as libc::c_int as libc::c_uint;
        qq[1 as libc::c_int
            as usize] = (qq[1 as libc::c_int as usize])
            .wrapping_add(
                (qq[3 as libc::c_int as usize]
                    & (qq[4 as libc::c_int as usize] ^ qq[0 as libc::c_int as usize])
                    ^ qq[0 as libc::c_int as usize])
                    .wrapping_add(block[13 as libc::c_int as usize])
                    .wrapping_add(0x5a827999 as libc::c_int as libc::c_uint)
                    .wrapping_add(
                        qq[2 as libc::c_int as usize] << 5 as libc::c_int
                            | qq[2 as libc::c_int as usize]
                                >> 32 as libc::c_int - 5 as libc::c_int,
                    ),
            );
        qq[3 as libc::c_int
            as usize] = qq[3 as libc::c_int as usize]
            << 32 as libc::c_int - 2 as libc::c_int
            | qq[3 as libc::c_int as usize] >> 2 as libc::c_int;
        block[14 as libc::c_int
            as usize] = (block[14 as libc::c_int as usize]
            << 32 as libc::c_int - 8 as libc::c_int
            | block[14 as libc::c_int as usize] >> 8 as libc::c_int)
            & 0xff00ff00 as libc::c_uint
            | (block[14 as libc::c_int as usize] << 8 as libc::c_int
                | block[14 as libc::c_int as usize]
                    >> 32 as libc::c_int - 8 as libc::c_int)
                & 0xff00ff as libc::c_int as libc::c_uint;
        qq[0 as libc::c_int
            as usize] = (qq[0 as libc::c_int as usize])
            .wrapping_add(
                (qq[2 as libc::c_int as usize]
                    & (qq[3 as libc::c_int as usize] ^ qq[4 as libc::c_int as usize])
                    ^ qq[4 as libc::c_int as usize])
                    .wrapping_add(block[14 as libc::c_int as usize])
                    .wrapping_add(0x5a827999 as libc::c_int as libc::c_uint)
                    .wrapping_add(
                        qq[1 as libc::c_int as usize] << 5 as libc::c_int
                            | qq[1 as libc::c_int as usize]
                                >> 32 as libc::c_int - 5 as libc::c_int,
                    ),
            );
        qq[2 as libc::c_int
            as usize] = qq[2 as libc::c_int as usize]
            << 32 as libc::c_int - 2 as libc::c_int
            | qq[2 as libc::c_int as usize] >> 2 as libc::c_int;
        block[15 as libc::c_int
            as usize] = (block[15 as libc::c_int as usize]
            << 32 as libc::c_int - 8 as libc::c_int
            | block[15 as libc::c_int as usize] >> 8 as libc::c_int)
            & 0xff00ff00 as libc::c_uint
            | (block[15 as libc::c_int as usize] << 8 as libc::c_int
                | block[15 as libc::c_int as usize]
                    >> 32 as libc::c_int - 8 as libc::c_int)
                & 0xff00ff as libc::c_int as libc::c_uint;
        qq[4 as libc::c_int
            as usize] = (qq[4 as libc::c_int as usize])
            .wrapping_add(
                (qq[1 as libc::c_int as usize]
                    & (qq[2 as libc::c_int as usize] ^ qq[3 as libc::c_int as usize])
                    ^ qq[3 as libc::c_int as usize])
                    .wrapping_add(block[15 as libc::c_int as usize])
                    .wrapping_add(0x5a827999 as libc::c_int as libc::c_uint)
                    .wrapping_add(
                        qq[0 as libc::c_int as usize] << 5 as libc::c_int
                            | qq[0 as libc::c_int as usize]
                                >> 32 as libc::c_int - 5 as libc::c_int,
                    ),
            );
        qq[1 as libc::c_int
            as usize] = qq[1 as libc::c_int as usize]
            << 32 as libc::c_int - 2 as libc::c_int
            | qq[1 as libc::c_int as usize] >> 2 as libc::c_int;
    } else {
        qq[4 as libc::c_int
            as usize] = (qq[4 as libc::c_int as usize])
            .wrapping_add(
                (qq[1 as libc::c_int as usize]
                    & (qq[2 as libc::c_int as usize] ^ qq[3 as libc::c_int as usize])
                    ^ qq[3 as libc::c_int as usize])
                    .wrapping_add(block[0 as libc::c_int as usize])
                    .wrapping_add(0x5a827999 as libc::c_int as libc::c_uint)
                    .wrapping_add(
                        qq[0 as libc::c_int as usize] << 5 as libc::c_int
                            | qq[0 as libc::c_int as usize]
                                >> 32 as libc::c_int - 5 as libc::c_int,
                    ),
            );
        qq[1 as libc::c_int
            as usize] = qq[1 as libc::c_int as usize]
            << 32 as libc::c_int - 2 as libc::c_int
            | qq[1 as libc::c_int as usize] >> 2 as libc::c_int;
        qq[3 as libc::c_int
            as usize] = (qq[3 as libc::c_int as usize])
            .wrapping_add(
                (qq[0 as libc::c_int as usize]
                    & (qq[1 as libc::c_int as usize] ^ qq[2 as libc::c_int as usize])
                    ^ qq[2 as libc::c_int as usize])
                    .wrapping_add(block[1 as libc::c_int as usize])
                    .wrapping_add(0x5a827999 as libc::c_int as libc::c_uint)
                    .wrapping_add(
                        qq[4 as libc::c_int as usize] << 5 as libc::c_int
                            | qq[4 as libc::c_int as usize]
                                >> 32 as libc::c_int - 5 as libc::c_int,
                    ),
            );
        qq[0 as libc::c_int
            as usize] = qq[0 as libc::c_int as usize]
            << 32 as libc::c_int - 2 as libc::c_int
            | qq[0 as libc::c_int as usize] >> 2 as libc::c_int;
        qq[2 as libc::c_int
            as usize] = (qq[2 as libc::c_int as usize])
            .wrapping_add(
                (qq[4 as libc::c_int as usize]
                    & (qq[0 as libc::c_int as usize] ^ qq[1 as libc::c_int as usize])
                    ^ qq[1 as libc::c_int as usize])
                    .wrapping_add(block[2 as libc::c_int as usize])
                    .wrapping_add(0x5a827999 as libc::c_int as libc::c_uint)
                    .wrapping_add(
                        qq[3 as libc::c_int as usize] << 5 as libc::c_int
                            | qq[3 as libc::c_int as usize]
                                >> 32 as libc::c_int - 5 as libc::c_int,
                    ),
            );
        qq[4 as libc::c_int
            as usize] = qq[4 as libc::c_int as usize]
            << 32 as libc::c_int - 2 as libc::c_int
            | qq[4 as libc::c_int as usize] >> 2 as libc::c_int;
        qq[1 as libc::c_int
            as usize] = (qq[1 as libc::c_int as usize])
            .wrapping_add(
                (qq[3 as libc::c_int as usize]
                    & (qq[4 as libc::c_int as usize] ^ qq[0 as libc::c_int as usize])
                    ^ qq[0 as libc::c_int as usize])
                    .wrapping_add(block[3 as libc::c_int as usize])
                    .wrapping_add(0x5a827999 as libc::c_int as libc::c_uint)
                    .wrapping_add(
                        qq[2 as libc::c_int as usize] << 5 as libc::c_int
                            | qq[2 as libc::c_int as usize]
                                >> 32 as libc::c_int - 5 as libc::c_int,
                    ),
            );
        qq[3 as libc::c_int
            as usize] = qq[3 as libc::c_int as usize]
            << 32 as libc::c_int - 2 as libc::c_int
            | qq[3 as libc::c_int as usize] >> 2 as libc::c_int;
        qq[0 as libc::c_int
            as usize] = (qq[0 as libc::c_int as usize])
            .wrapping_add(
                (qq[2 as libc::c_int as usize]
                    & (qq[3 as libc::c_int as usize] ^ qq[4 as libc::c_int as usize])
                    ^ qq[4 as libc::c_int as usize])
                    .wrapping_add(block[4 as libc::c_int as usize])
                    .wrapping_add(0x5a827999 as libc::c_int as libc::c_uint)
                    .wrapping_add(
                        qq[1 as libc::c_int as usize] << 5 as libc::c_int
                            | qq[1 as libc::c_int as usize]
                                >> 32 as libc::c_int - 5 as libc::c_int,
                    ),
            );
        qq[2 as libc::c_int
            as usize] = qq[2 as libc::c_int as usize]
            << 32 as libc::c_int - 2 as libc::c_int
            | qq[2 as libc::c_int as usize] >> 2 as libc::c_int;
        qq[4 as libc::c_int
            as usize] = (qq[4 as libc::c_int as usize])
            .wrapping_add(
                (qq[1 as libc::c_int as usize]
                    & (qq[2 as libc::c_int as usize] ^ qq[3 as libc::c_int as usize])
                    ^ qq[3 as libc::c_int as usize])
                    .wrapping_add(block[5 as libc::c_int as usize])
                    .wrapping_add(0x5a827999 as libc::c_int as libc::c_uint)
                    .wrapping_add(
                        qq[0 as libc::c_int as usize] << 5 as libc::c_int
                            | qq[0 as libc::c_int as usize]
                                >> 32 as libc::c_int - 5 as libc::c_int,
                    ),
            );
        qq[1 as libc::c_int
            as usize] = qq[1 as libc::c_int as usize]
            << 32 as libc::c_int - 2 as libc::c_int
            | qq[1 as libc::c_int as usize] >> 2 as libc::c_int;
        qq[3 as libc::c_int
            as usize] = (qq[3 as libc::c_int as usize])
            .wrapping_add(
                (qq[0 as libc::c_int as usize]
                    & (qq[1 as libc::c_int as usize] ^ qq[2 as libc::c_int as usize])
                    ^ qq[2 as libc::c_int as usize])
                    .wrapping_add(block[6 as libc::c_int as usize])
                    .wrapping_add(0x5a827999 as libc::c_int as libc::c_uint)
                    .wrapping_add(
                        qq[4 as libc::c_int as usize] << 5 as libc::c_int
                            | qq[4 as libc::c_int as usize]
                                >> 32 as libc::c_int - 5 as libc::c_int,
                    ),
            );
        qq[0 as libc::c_int
            as usize] = qq[0 as libc::c_int as usize]
            << 32 as libc::c_int - 2 as libc::c_int
            | qq[0 as libc::c_int as usize] >> 2 as libc::c_int;
        qq[2 as libc::c_int
            as usize] = (qq[2 as libc::c_int as usize])
            .wrapping_add(
                (qq[4 as libc::c_int as usize]
                    & (qq[0 as libc::c_int as usize] ^ qq[1 as libc::c_int as usize])
                    ^ qq[1 as libc::c_int as usize])
                    .wrapping_add(block[7 as libc::c_int as usize])
                    .wrapping_add(0x5a827999 as libc::c_int as libc::c_uint)
                    .wrapping_add(
                        qq[3 as libc::c_int as usize] << 5 as libc::c_int
                            | qq[3 as libc::c_int as usize]
                                >> 32 as libc::c_int - 5 as libc::c_int,
                    ),
            );
        qq[4 as libc::c_int
            as usize] = qq[4 as libc::c_int as usize]
            << 32 as libc::c_int - 2 as libc::c_int
            | qq[4 as libc::c_int as usize] >> 2 as libc::c_int;
        qq[1 as libc::c_int
            as usize] = (qq[1 as libc::c_int as usize])
            .wrapping_add(
                (qq[3 as libc::c_int as usize]
                    & (qq[4 as libc::c_int as usize] ^ qq[0 as libc::c_int as usize])
                    ^ qq[0 as libc::c_int as usize])
                    .wrapping_add(block[8 as libc::c_int as usize])
                    .wrapping_add(0x5a827999 as libc::c_int as libc::c_uint)
                    .wrapping_add(
                        qq[2 as libc::c_int as usize] << 5 as libc::c_int
                            | qq[2 as libc::c_int as usize]
                                >> 32 as libc::c_int - 5 as libc::c_int,
                    ),
            );
        qq[3 as libc::c_int
            as usize] = qq[3 as libc::c_int as usize]
            << 32 as libc::c_int - 2 as libc::c_int
            | qq[3 as libc::c_int as usize] >> 2 as libc::c_int;
        qq[0 as libc::c_int
            as usize] = (qq[0 as libc::c_int as usize])
            .wrapping_add(
                (qq[2 as libc::c_int as usize]
                    & (qq[3 as libc::c_int as usize] ^ qq[4 as libc::c_int as usize])
                    ^ qq[4 as libc::c_int as usize])
                    .wrapping_add(block[9 as libc::c_int as usize])
                    .wrapping_add(0x5a827999 as libc::c_int as libc::c_uint)
                    .wrapping_add(
                        qq[1 as libc::c_int as usize] << 5 as libc::c_int
                            | qq[1 as libc::c_int as usize]
                                >> 32 as libc::c_int - 5 as libc::c_int,
                    ),
            );
        qq[2 as libc::c_int
            as usize] = qq[2 as libc::c_int as usize]
            << 32 as libc::c_int - 2 as libc::c_int
            | qq[2 as libc::c_int as usize] >> 2 as libc::c_int;
        qq[4 as libc::c_int
            as usize] = (qq[4 as libc::c_int as usize])
            .wrapping_add(
                (qq[1 as libc::c_int as usize]
                    & (qq[2 as libc::c_int as usize] ^ qq[3 as libc::c_int as usize])
                    ^ qq[3 as libc::c_int as usize])
                    .wrapping_add(block[10 as libc::c_int as usize])
                    .wrapping_add(0x5a827999 as libc::c_int as libc::c_uint)
                    .wrapping_add(
                        qq[0 as libc::c_int as usize] << 5 as libc::c_int
                            | qq[0 as libc::c_int as usize]
                                >> 32 as libc::c_int - 5 as libc::c_int,
                    ),
            );
        qq[1 as libc::c_int
            as usize] = qq[1 as libc::c_int as usize]
            << 32 as libc::c_int - 2 as libc::c_int
            | qq[1 as libc::c_int as usize] >> 2 as libc::c_int;
        qq[3 as libc::c_int
            as usize] = (qq[3 as libc::c_int as usize])
            .wrapping_add(
                (qq[0 as libc::c_int as usize]
                    & (qq[1 as libc::c_int as usize] ^ qq[2 as libc::c_int as usize])
                    ^ qq[2 as libc::c_int as usize])
                    .wrapping_add(block[11 as libc::c_int as usize])
                    .wrapping_add(0x5a827999 as libc::c_int as libc::c_uint)
                    .wrapping_add(
                        qq[4 as libc::c_int as usize] << 5 as libc::c_int
                            | qq[4 as libc::c_int as usize]
                                >> 32 as libc::c_int - 5 as libc::c_int,
                    ),
            );
        qq[0 as libc::c_int
            as usize] = qq[0 as libc::c_int as usize]
            << 32 as libc::c_int - 2 as libc::c_int
            | qq[0 as libc::c_int as usize] >> 2 as libc::c_int;
        qq[2 as libc::c_int
            as usize] = (qq[2 as libc::c_int as usize])
            .wrapping_add(
                (qq[4 as libc::c_int as usize]
                    & (qq[0 as libc::c_int as usize] ^ qq[1 as libc::c_int as usize])
                    ^ qq[1 as libc::c_int as usize])
                    .wrapping_add(block[12 as libc::c_int as usize])
                    .wrapping_add(0x5a827999 as libc::c_int as libc::c_uint)
                    .wrapping_add(
                        qq[3 as libc::c_int as usize] << 5 as libc::c_int
                            | qq[3 as libc::c_int as usize]
                                >> 32 as libc::c_int - 5 as libc::c_int,
                    ),
            );
        qq[4 as libc::c_int
            as usize] = qq[4 as libc::c_int as usize]
            << 32 as libc::c_int - 2 as libc::c_int
            | qq[4 as libc::c_int as usize] >> 2 as libc::c_int;
        qq[1 as libc::c_int
            as usize] = (qq[1 as libc::c_int as usize])
            .wrapping_add(
                (qq[3 as libc::c_int as usize]
                    & (qq[4 as libc::c_int as usize] ^ qq[0 as libc::c_int as usize])
                    ^ qq[0 as libc::c_int as usize])
                    .wrapping_add(block[13 as libc::c_int as usize])
                    .wrapping_add(0x5a827999 as libc::c_int as libc::c_uint)
                    .wrapping_add(
                        qq[2 as libc::c_int as usize] << 5 as libc::c_int
                            | qq[2 as libc::c_int as usize]
                                >> 32 as libc::c_int - 5 as libc::c_int,
                    ),
            );
        qq[3 as libc::c_int
            as usize] = qq[3 as libc::c_int as usize]
            << 32 as libc::c_int - 2 as libc::c_int
            | qq[3 as libc::c_int as usize] >> 2 as libc::c_int;
        qq[0 as libc::c_int
            as usize] = (qq[0 as libc::c_int as usize])
            .wrapping_add(
                (qq[2 as libc::c_int as usize]
                    & (qq[3 as libc::c_int as usize] ^ qq[4 as libc::c_int as usize])
                    ^ qq[4 as libc::c_int as usize])
                    .wrapping_add(block[14 as libc::c_int as usize])
                    .wrapping_add(0x5a827999 as libc::c_int as libc::c_uint)
                    .wrapping_add(
                        qq[1 as libc::c_int as usize] << 5 as libc::c_int
                            | qq[1 as libc::c_int as usize]
                                >> 32 as libc::c_int - 5 as libc::c_int,
                    ),
            );
        qq[2 as libc::c_int
            as usize] = qq[2 as libc::c_int as usize]
            << 32 as libc::c_int - 2 as libc::c_int
            | qq[2 as libc::c_int as usize] >> 2 as libc::c_int;
        qq[4 as libc::c_int
            as usize] = (qq[4 as libc::c_int as usize])
            .wrapping_add(
                (qq[1 as libc::c_int as usize]
                    & (qq[2 as libc::c_int as usize] ^ qq[3 as libc::c_int as usize])
                    ^ qq[3 as libc::c_int as usize])
                    .wrapping_add(block[15 as libc::c_int as usize])
                    .wrapping_add(0x5a827999 as libc::c_int as libc::c_uint)
                    .wrapping_add(
                        qq[0 as libc::c_int as usize] << 5 as libc::c_int
                            | qq[0 as libc::c_int as usize]
                                >> 32 as libc::c_int - 5 as libc::c_int,
                    ),
            );
        qq[1 as libc::c_int
            as usize] = qq[1 as libc::c_int as usize]
            << 32 as libc::c_int - 2 as libc::c_int
            | qq[1 as libc::c_int as usize] >> 2 as libc::c_int;
    }
    block[(16 as libc::c_int & 15 as libc::c_int)
        as usize] = (block[(16 as libc::c_int + 13 as libc::c_int & 15 as libc::c_int)
        as usize]
        ^ block[(16 as libc::c_int + 8 as libc::c_int & 15 as libc::c_int) as usize]
        ^ block[(16 as libc::c_int + 2 as libc::c_int & 15 as libc::c_int) as usize]
        ^ block[(16 as libc::c_int & 15 as libc::c_int) as usize]) << 1 as libc::c_int
        | (block[(16 as libc::c_int + 13 as libc::c_int & 15 as libc::c_int) as usize]
            ^ block[(16 as libc::c_int + 8 as libc::c_int & 15 as libc::c_int) as usize]
            ^ block[(16 as libc::c_int + 2 as libc::c_int & 15 as libc::c_int) as usize]
            ^ block[(16 as libc::c_int & 15 as libc::c_int) as usize])
            >> 32 as libc::c_int - 1 as libc::c_int;
    qq[3 as libc::c_int
        as usize] = (qq[3 as libc::c_int as usize])
        .wrapping_add(
            (qq[0 as libc::c_int as usize]
                & (qq[1 as libc::c_int as usize] ^ qq[2 as libc::c_int as usize])
                ^ qq[2 as libc::c_int as usize])
                .wrapping_add(block[(16 as libc::c_int & 15 as libc::c_int) as usize])
                .wrapping_add(0x5a827999 as libc::c_int as libc::c_uint)
                .wrapping_add(
                    qq[4 as libc::c_int as usize] << 5 as libc::c_int
                        | qq[4 as libc::c_int as usize]
                            >> 32 as libc::c_int - 5 as libc::c_int,
                ),
        );
    qq[0 as libc::c_int
        as usize] = qq[0 as libc::c_int as usize] << 32 as libc::c_int - 2 as libc::c_int
        | qq[0 as libc::c_int as usize] >> 2 as libc::c_int;
    block[(17 as libc::c_int & 15 as libc::c_int)
        as usize] = (block[(17 as libc::c_int + 13 as libc::c_int & 15 as libc::c_int)
        as usize]
        ^ block[(17 as libc::c_int + 8 as libc::c_int & 15 as libc::c_int) as usize]
        ^ block[(17 as libc::c_int + 2 as libc::c_int & 15 as libc::c_int) as usize]
        ^ block[(17 as libc::c_int & 15 as libc::c_int) as usize]) << 1 as libc::c_int
        | (block[(17 as libc::c_int + 13 as libc::c_int & 15 as libc::c_int) as usize]
            ^ block[(17 as libc::c_int + 8 as libc::c_int & 15 as libc::c_int) as usize]
            ^ block[(17 as libc::c_int + 2 as libc::c_int & 15 as libc::c_int) as usize]
            ^ block[(17 as libc::c_int & 15 as libc::c_int) as usize])
            >> 32 as libc::c_int - 1 as libc::c_int;
    qq[2 as libc::c_int
        as usize] = (qq[2 as libc::c_int as usize])
        .wrapping_add(
            (qq[4 as libc::c_int as usize]
                & (qq[0 as libc::c_int as usize] ^ qq[1 as libc::c_int as usize])
                ^ qq[1 as libc::c_int as usize])
                .wrapping_add(block[(17 as libc::c_int & 15 as libc::c_int) as usize])
                .wrapping_add(0x5a827999 as libc::c_int as libc::c_uint)
                .wrapping_add(
                    qq[3 as libc::c_int as usize] << 5 as libc::c_int
                        | qq[3 as libc::c_int as usize]
                            >> 32 as libc::c_int - 5 as libc::c_int,
                ),
        );
    qq[4 as libc::c_int
        as usize] = qq[4 as libc::c_int as usize] << 32 as libc::c_int - 2 as libc::c_int
        | qq[4 as libc::c_int as usize] >> 2 as libc::c_int;
    block[(18 as libc::c_int & 15 as libc::c_int)
        as usize] = (block[(18 as libc::c_int + 13 as libc::c_int & 15 as libc::c_int)
        as usize]
        ^ block[(18 as libc::c_int + 8 as libc::c_int & 15 as libc::c_int) as usize]
        ^ block[(18 as libc::c_int + 2 as libc::c_int & 15 as libc::c_int) as usize]
        ^ block[(18 as libc::c_int & 15 as libc::c_int) as usize]) << 1 as libc::c_int
        | (block[(18 as libc::c_int + 13 as libc::c_int & 15 as libc::c_int) as usize]
            ^ block[(18 as libc::c_int + 8 as libc::c_int & 15 as libc::c_int) as usize]
            ^ block[(18 as libc::c_int + 2 as libc::c_int & 15 as libc::c_int) as usize]
            ^ block[(18 as libc::c_int & 15 as libc::c_int) as usize])
            >> 32 as libc::c_int - 1 as libc::c_int;
    qq[1 as libc::c_int
        as usize] = (qq[1 as libc::c_int as usize])
        .wrapping_add(
            (qq[3 as libc::c_int as usize]
                & (qq[4 as libc::c_int as usize] ^ qq[0 as libc::c_int as usize])
                ^ qq[0 as libc::c_int as usize])
                .wrapping_add(block[(18 as libc::c_int & 15 as libc::c_int) as usize])
                .wrapping_add(0x5a827999 as libc::c_int as libc::c_uint)
                .wrapping_add(
                    qq[2 as libc::c_int as usize] << 5 as libc::c_int
                        | qq[2 as libc::c_int as usize]
                            >> 32 as libc::c_int - 5 as libc::c_int,
                ),
        );
    qq[3 as libc::c_int
        as usize] = qq[3 as libc::c_int as usize] << 32 as libc::c_int - 2 as libc::c_int
        | qq[3 as libc::c_int as usize] >> 2 as libc::c_int;
    block[(19 as libc::c_int & 15 as libc::c_int)
        as usize] = (block[(19 as libc::c_int + 13 as libc::c_int & 15 as libc::c_int)
        as usize]
        ^ block[(19 as libc::c_int + 8 as libc::c_int & 15 as libc::c_int) as usize]
        ^ block[(19 as libc::c_int + 2 as libc::c_int & 15 as libc::c_int) as usize]
        ^ block[(19 as libc::c_int & 15 as libc::c_int) as usize]) << 1 as libc::c_int
        | (block[(19 as libc::c_int + 13 as libc::c_int & 15 as libc::c_int) as usize]
            ^ block[(19 as libc::c_int + 8 as libc::c_int & 15 as libc::c_int) as usize]
            ^ block[(19 as libc::c_int + 2 as libc::c_int & 15 as libc::c_int) as usize]
            ^ block[(19 as libc::c_int & 15 as libc::c_int) as usize])
            >> 32 as libc::c_int - 1 as libc::c_int;
    qq[0 as libc::c_int
        as usize] = (qq[0 as libc::c_int as usize])
        .wrapping_add(
            (qq[2 as libc::c_int as usize]
                & (qq[3 as libc::c_int as usize] ^ qq[4 as libc::c_int as usize])
                ^ qq[4 as libc::c_int as usize])
                .wrapping_add(block[(19 as libc::c_int & 15 as libc::c_int) as usize])
                .wrapping_add(0x5a827999 as libc::c_int as libc::c_uint)
                .wrapping_add(
                    qq[1 as libc::c_int as usize] << 5 as libc::c_int
                        | qq[1 as libc::c_int as usize]
                            >> 32 as libc::c_int - 5 as libc::c_int,
                ),
        );
    qq[2 as libc::c_int
        as usize] = qq[2 as libc::c_int as usize] << 32 as libc::c_int - 2 as libc::c_int
        | qq[2 as libc::c_int as usize] >> 2 as libc::c_int;
    block[(20 as libc::c_int & 15 as libc::c_int)
        as usize] = (block[(20 as libc::c_int + 13 as libc::c_int & 15 as libc::c_int)
        as usize]
        ^ block[(20 as libc::c_int + 8 as libc::c_int & 15 as libc::c_int) as usize]
        ^ block[(20 as libc::c_int + 2 as libc::c_int & 15 as libc::c_int) as usize]
        ^ block[(20 as libc::c_int & 15 as libc::c_int) as usize]) << 1 as libc::c_int
        | (block[(20 as libc::c_int + 13 as libc::c_int & 15 as libc::c_int) as usize]
            ^ block[(20 as libc::c_int + 8 as libc::c_int & 15 as libc::c_int) as usize]
            ^ block[(20 as libc::c_int + 2 as libc::c_int & 15 as libc::c_int) as usize]
            ^ block[(20 as libc::c_int & 15 as libc::c_int) as usize])
            >> 32 as libc::c_int - 1 as libc::c_int;
    qq[4 as libc::c_int
        as usize] = (qq[4 as libc::c_int as usize])
        .wrapping_add(
            (qq[1 as libc::c_int as usize] ^ qq[2 as libc::c_int as usize]
                ^ qq[3 as libc::c_int as usize])
                .wrapping_add(block[(20 as libc::c_int & 15 as libc::c_int) as usize])
                .wrapping_add(0x6ed9eba1 as libc::c_int as libc::c_uint)
                .wrapping_add(
                    qq[0 as libc::c_int as usize] << 5 as libc::c_int
                        | qq[0 as libc::c_int as usize]
                            >> 32 as libc::c_int - 5 as libc::c_int,
                ),
        );
    qq[1 as libc::c_int
        as usize] = qq[1 as libc::c_int as usize] << 32 as libc::c_int - 2 as libc::c_int
        | qq[1 as libc::c_int as usize] >> 2 as libc::c_int;
    block[(21 as libc::c_int & 15 as libc::c_int)
        as usize] = (block[(21 as libc::c_int + 13 as libc::c_int & 15 as libc::c_int)
        as usize]
        ^ block[(21 as libc::c_int + 8 as libc::c_int & 15 as libc::c_int) as usize]
        ^ block[(21 as libc::c_int + 2 as libc::c_int & 15 as libc::c_int) as usize]
        ^ block[(21 as libc::c_int & 15 as libc::c_int) as usize]) << 1 as libc::c_int
        | (block[(21 as libc::c_int + 13 as libc::c_int & 15 as libc::c_int) as usize]
            ^ block[(21 as libc::c_int + 8 as libc::c_int & 15 as libc::c_int) as usize]
            ^ block[(21 as libc::c_int + 2 as libc::c_int & 15 as libc::c_int) as usize]
            ^ block[(21 as libc::c_int & 15 as libc::c_int) as usize])
            >> 32 as libc::c_int - 1 as libc::c_int;
    qq[3 as libc::c_int
        as usize] = (qq[3 as libc::c_int as usize])
        .wrapping_add(
            (qq[0 as libc::c_int as usize] ^ qq[1 as libc::c_int as usize]
                ^ qq[2 as libc::c_int as usize])
                .wrapping_add(block[(21 as libc::c_int & 15 as libc::c_int) as usize])
                .wrapping_add(0x6ed9eba1 as libc::c_int as libc::c_uint)
                .wrapping_add(
                    qq[4 as libc::c_int as usize] << 5 as libc::c_int
                        | qq[4 as libc::c_int as usize]
                            >> 32 as libc::c_int - 5 as libc::c_int,
                ),
        );
    qq[0 as libc::c_int
        as usize] = qq[0 as libc::c_int as usize] << 32 as libc::c_int - 2 as libc::c_int
        | qq[0 as libc::c_int as usize] >> 2 as libc::c_int;
    block[(22 as libc::c_int & 15 as libc::c_int)
        as usize] = (block[(22 as libc::c_int + 13 as libc::c_int & 15 as libc::c_int)
        as usize]
        ^ block[(22 as libc::c_int + 8 as libc::c_int & 15 as libc::c_int) as usize]
        ^ block[(22 as libc::c_int + 2 as libc::c_int & 15 as libc::c_int) as usize]
        ^ block[(22 as libc::c_int & 15 as libc::c_int) as usize]) << 1 as libc::c_int
        | (block[(22 as libc::c_int + 13 as libc::c_int & 15 as libc::c_int) as usize]
            ^ block[(22 as libc::c_int + 8 as libc::c_int & 15 as libc::c_int) as usize]
            ^ block[(22 as libc::c_int + 2 as libc::c_int & 15 as libc::c_int) as usize]
            ^ block[(22 as libc::c_int & 15 as libc::c_int) as usize])
            >> 32 as libc::c_int - 1 as libc::c_int;
    qq[2 as libc::c_int
        as usize] = (qq[2 as libc::c_int as usize])
        .wrapping_add(
            (qq[4 as libc::c_int as usize] ^ qq[0 as libc::c_int as usize]
                ^ qq[1 as libc::c_int as usize])
                .wrapping_add(block[(22 as libc::c_int & 15 as libc::c_int) as usize])
                .wrapping_add(0x6ed9eba1 as libc::c_int as libc::c_uint)
                .wrapping_add(
                    qq[3 as libc::c_int as usize] << 5 as libc::c_int
                        | qq[3 as libc::c_int as usize]
                            >> 32 as libc::c_int - 5 as libc::c_int,
                ),
        );
    qq[4 as libc::c_int
        as usize] = qq[4 as libc::c_int as usize] << 32 as libc::c_int - 2 as libc::c_int
        | qq[4 as libc::c_int as usize] >> 2 as libc::c_int;
    block[(23 as libc::c_int & 15 as libc::c_int)
        as usize] = (block[(23 as libc::c_int + 13 as libc::c_int & 15 as libc::c_int)
        as usize]
        ^ block[(23 as libc::c_int + 8 as libc::c_int & 15 as libc::c_int) as usize]
        ^ block[(23 as libc::c_int + 2 as libc::c_int & 15 as libc::c_int) as usize]
        ^ block[(23 as libc::c_int & 15 as libc::c_int) as usize]) << 1 as libc::c_int
        | (block[(23 as libc::c_int + 13 as libc::c_int & 15 as libc::c_int) as usize]
            ^ block[(23 as libc::c_int + 8 as libc::c_int & 15 as libc::c_int) as usize]
            ^ block[(23 as libc::c_int + 2 as libc::c_int & 15 as libc::c_int) as usize]
            ^ block[(23 as libc::c_int & 15 as libc::c_int) as usize])
            >> 32 as libc::c_int - 1 as libc::c_int;
    qq[1 as libc::c_int
        as usize] = (qq[1 as libc::c_int as usize])
        .wrapping_add(
            (qq[3 as libc::c_int as usize] ^ qq[4 as libc::c_int as usize]
                ^ qq[0 as libc::c_int as usize])
                .wrapping_add(block[(23 as libc::c_int & 15 as libc::c_int) as usize])
                .wrapping_add(0x6ed9eba1 as libc::c_int as libc::c_uint)
                .wrapping_add(
                    qq[2 as libc::c_int as usize] << 5 as libc::c_int
                        | qq[2 as libc::c_int as usize]
                            >> 32 as libc::c_int - 5 as libc::c_int,
                ),
        );
    qq[3 as libc::c_int
        as usize] = qq[3 as libc::c_int as usize] << 32 as libc::c_int - 2 as libc::c_int
        | qq[3 as libc::c_int as usize] >> 2 as libc::c_int;
    block[(24 as libc::c_int & 15 as libc::c_int)
        as usize] = (block[(24 as libc::c_int + 13 as libc::c_int & 15 as libc::c_int)
        as usize]
        ^ block[(24 as libc::c_int + 8 as libc::c_int & 15 as libc::c_int) as usize]
        ^ block[(24 as libc::c_int + 2 as libc::c_int & 15 as libc::c_int) as usize]
        ^ block[(24 as libc::c_int & 15 as libc::c_int) as usize]) << 1 as libc::c_int
        | (block[(24 as libc::c_int + 13 as libc::c_int & 15 as libc::c_int) as usize]
            ^ block[(24 as libc::c_int + 8 as libc::c_int & 15 as libc::c_int) as usize]
            ^ block[(24 as libc::c_int + 2 as libc::c_int & 15 as libc::c_int) as usize]
            ^ block[(24 as libc::c_int & 15 as libc::c_int) as usize])
            >> 32 as libc::c_int - 1 as libc::c_int;
    qq[0 as libc::c_int
        as usize] = (qq[0 as libc::c_int as usize])
        .wrapping_add(
            (qq[2 as libc::c_int as usize] ^ qq[3 as libc::c_int as usize]
                ^ qq[4 as libc::c_int as usize])
                .wrapping_add(block[(24 as libc::c_int & 15 as libc::c_int) as usize])
                .wrapping_add(0x6ed9eba1 as libc::c_int as libc::c_uint)
                .wrapping_add(
                    qq[1 as libc::c_int as usize] << 5 as libc::c_int
                        | qq[1 as libc::c_int as usize]
                            >> 32 as libc::c_int - 5 as libc::c_int,
                ),
        );
    qq[2 as libc::c_int
        as usize] = qq[2 as libc::c_int as usize] << 32 as libc::c_int - 2 as libc::c_int
        | qq[2 as libc::c_int as usize] >> 2 as libc::c_int;
    block[(25 as libc::c_int & 15 as libc::c_int)
        as usize] = (block[(25 as libc::c_int + 13 as libc::c_int & 15 as libc::c_int)
        as usize]
        ^ block[(25 as libc::c_int + 8 as libc::c_int & 15 as libc::c_int) as usize]
        ^ block[(25 as libc::c_int + 2 as libc::c_int & 15 as libc::c_int) as usize]
        ^ block[(25 as libc::c_int & 15 as libc::c_int) as usize]) << 1 as libc::c_int
        | (block[(25 as libc::c_int + 13 as libc::c_int & 15 as libc::c_int) as usize]
            ^ block[(25 as libc::c_int + 8 as libc::c_int & 15 as libc::c_int) as usize]
            ^ block[(25 as libc::c_int + 2 as libc::c_int & 15 as libc::c_int) as usize]
            ^ block[(25 as libc::c_int & 15 as libc::c_int) as usize])
            >> 32 as libc::c_int - 1 as libc::c_int;
    qq[4 as libc::c_int
        as usize] = (qq[4 as libc::c_int as usize])
        .wrapping_add(
            (qq[1 as libc::c_int as usize] ^ qq[2 as libc::c_int as usize]
                ^ qq[3 as libc::c_int as usize])
                .wrapping_add(block[(25 as libc::c_int & 15 as libc::c_int) as usize])
                .wrapping_add(0x6ed9eba1 as libc::c_int as libc::c_uint)
                .wrapping_add(
                    qq[0 as libc::c_int as usize] << 5 as libc::c_int
                        | qq[0 as libc::c_int as usize]
                            >> 32 as libc::c_int - 5 as libc::c_int,
                ),
        );
    qq[1 as libc::c_int
        as usize] = qq[1 as libc::c_int as usize] << 32 as libc::c_int - 2 as libc::c_int
        | qq[1 as libc::c_int as usize] >> 2 as libc::c_int;
    block[(26 as libc::c_int & 15 as libc::c_int)
        as usize] = (block[(26 as libc::c_int + 13 as libc::c_int & 15 as libc::c_int)
        as usize]
        ^ block[(26 as libc::c_int + 8 as libc::c_int & 15 as libc::c_int) as usize]
        ^ block[(26 as libc::c_int + 2 as libc::c_int & 15 as libc::c_int) as usize]
        ^ block[(26 as libc::c_int & 15 as libc::c_int) as usize]) << 1 as libc::c_int
        | (block[(26 as libc::c_int + 13 as libc::c_int & 15 as libc::c_int) as usize]
            ^ block[(26 as libc::c_int + 8 as libc::c_int & 15 as libc::c_int) as usize]
            ^ block[(26 as libc::c_int + 2 as libc::c_int & 15 as libc::c_int) as usize]
            ^ block[(26 as libc::c_int & 15 as libc::c_int) as usize])
            >> 32 as libc::c_int - 1 as libc::c_int;
    qq[3 as libc::c_int
        as usize] = (qq[3 as libc::c_int as usize])
        .wrapping_add(
            (qq[0 as libc::c_int as usize] ^ qq[1 as libc::c_int as usize]
                ^ qq[2 as libc::c_int as usize])
                .wrapping_add(block[(26 as libc::c_int & 15 as libc::c_int) as usize])
                .wrapping_add(0x6ed9eba1 as libc::c_int as libc::c_uint)
                .wrapping_add(
                    qq[4 as libc::c_int as usize] << 5 as libc::c_int
                        | qq[4 as libc::c_int as usize]
                            >> 32 as libc::c_int - 5 as libc::c_int,
                ),
        );
    qq[0 as libc::c_int
        as usize] = qq[0 as libc::c_int as usize] << 32 as libc::c_int - 2 as libc::c_int
        | qq[0 as libc::c_int as usize] >> 2 as libc::c_int;
    block[(27 as libc::c_int & 15 as libc::c_int)
        as usize] = (block[(27 as libc::c_int + 13 as libc::c_int & 15 as libc::c_int)
        as usize]
        ^ block[(27 as libc::c_int + 8 as libc::c_int & 15 as libc::c_int) as usize]
        ^ block[(27 as libc::c_int + 2 as libc::c_int & 15 as libc::c_int) as usize]
        ^ block[(27 as libc::c_int & 15 as libc::c_int) as usize]) << 1 as libc::c_int
        | (block[(27 as libc::c_int + 13 as libc::c_int & 15 as libc::c_int) as usize]
            ^ block[(27 as libc::c_int + 8 as libc::c_int & 15 as libc::c_int) as usize]
            ^ block[(27 as libc::c_int + 2 as libc::c_int & 15 as libc::c_int) as usize]
            ^ block[(27 as libc::c_int & 15 as libc::c_int) as usize])
            >> 32 as libc::c_int - 1 as libc::c_int;
    qq[2 as libc::c_int
        as usize] = (qq[2 as libc::c_int as usize])
        .wrapping_add(
            (qq[4 as libc::c_int as usize] ^ qq[0 as libc::c_int as usize]
                ^ qq[1 as libc::c_int as usize])
                .wrapping_add(block[(27 as libc::c_int & 15 as libc::c_int) as usize])
                .wrapping_add(0x6ed9eba1 as libc::c_int as libc::c_uint)
                .wrapping_add(
                    qq[3 as libc::c_int as usize] << 5 as libc::c_int
                        | qq[3 as libc::c_int as usize]
                            >> 32 as libc::c_int - 5 as libc::c_int,
                ),
        );
    qq[4 as libc::c_int
        as usize] = qq[4 as libc::c_int as usize] << 32 as libc::c_int - 2 as libc::c_int
        | qq[4 as libc::c_int as usize] >> 2 as libc::c_int;
    block[(28 as libc::c_int & 15 as libc::c_int)
        as usize] = (block[(28 as libc::c_int + 13 as libc::c_int & 15 as libc::c_int)
        as usize]
        ^ block[(28 as libc::c_int + 8 as libc::c_int & 15 as libc::c_int) as usize]
        ^ block[(28 as libc::c_int + 2 as libc::c_int & 15 as libc::c_int) as usize]
        ^ block[(28 as libc::c_int & 15 as libc::c_int) as usize]) << 1 as libc::c_int
        | (block[(28 as libc::c_int + 13 as libc::c_int & 15 as libc::c_int) as usize]
            ^ block[(28 as libc::c_int + 8 as libc::c_int & 15 as libc::c_int) as usize]
            ^ block[(28 as libc::c_int + 2 as libc::c_int & 15 as libc::c_int) as usize]
            ^ block[(28 as libc::c_int & 15 as libc::c_int) as usize])
            >> 32 as libc::c_int - 1 as libc::c_int;
    qq[1 as libc::c_int
        as usize] = (qq[1 as libc::c_int as usize])
        .wrapping_add(
            (qq[3 as libc::c_int as usize] ^ qq[4 as libc::c_int as usize]
                ^ qq[0 as libc::c_int as usize])
                .wrapping_add(block[(28 as libc::c_int & 15 as libc::c_int) as usize])
                .wrapping_add(0x6ed9eba1 as libc::c_int as libc::c_uint)
                .wrapping_add(
                    qq[2 as libc::c_int as usize] << 5 as libc::c_int
                        | qq[2 as libc::c_int as usize]
                            >> 32 as libc::c_int - 5 as libc::c_int,
                ),
        );
    qq[3 as libc::c_int
        as usize] = qq[3 as libc::c_int as usize] << 32 as libc::c_int - 2 as libc::c_int
        | qq[3 as libc::c_int as usize] >> 2 as libc::c_int;
    block[(29 as libc::c_int & 15 as libc::c_int)
        as usize] = (block[(29 as libc::c_int + 13 as libc::c_int & 15 as libc::c_int)
        as usize]
        ^ block[(29 as libc::c_int + 8 as libc::c_int & 15 as libc::c_int) as usize]
        ^ block[(29 as libc::c_int + 2 as libc::c_int & 15 as libc::c_int) as usize]
        ^ block[(29 as libc::c_int & 15 as libc::c_int) as usize]) << 1 as libc::c_int
        | (block[(29 as libc::c_int + 13 as libc::c_int & 15 as libc::c_int) as usize]
            ^ block[(29 as libc::c_int + 8 as libc::c_int & 15 as libc::c_int) as usize]
            ^ block[(29 as libc::c_int + 2 as libc::c_int & 15 as libc::c_int) as usize]
            ^ block[(29 as libc::c_int & 15 as libc::c_int) as usize])
            >> 32 as libc::c_int - 1 as libc::c_int;
    qq[0 as libc::c_int
        as usize] = (qq[0 as libc::c_int as usize])
        .wrapping_add(
            (qq[2 as libc::c_int as usize] ^ qq[3 as libc::c_int as usize]
                ^ qq[4 as libc::c_int as usize])
                .wrapping_add(block[(29 as libc::c_int & 15 as libc::c_int) as usize])
                .wrapping_add(0x6ed9eba1 as libc::c_int as libc::c_uint)
                .wrapping_add(
                    qq[1 as libc::c_int as usize] << 5 as libc::c_int
                        | qq[1 as libc::c_int as usize]
                            >> 32 as libc::c_int - 5 as libc::c_int,
                ),
        );
    qq[2 as libc::c_int
        as usize] = qq[2 as libc::c_int as usize] << 32 as libc::c_int - 2 as libc::c_int
        | qq[2 as libc::c_int as usize] >> 2 as libc::c_int;
    block[(30 as libc::c_int & 15 as libc::c_int)
        as usize] = (block[(30 as libc::c_int + 13 as libc::c_int & 15 as libc::c_int)
        as usize]
        ^ block[(30 as libc::c_int + 8 as libc::c_int & 15 as libc::c_int) as usize]
        ^ block[(30 as libc::c_int + 2 as libc::c_int & 15 as libc::c_int) as usize]
        ^ block[(30 as libc::c_int & 15 as libc::c_int) as usize]) << 1 as libc::c_int
        | (block[(30 as libc::c_int + 13 as libc::c_int & 15 as libc::c_int) as usize]
            ^ block[(30 as libc::c_int + 8 as libc::c_int & 15 as libc::c_int) as usize]
            ^ block[(30 as libc::c_int + 2 as libc::c_int & 15 as libc::c_int) as usize]
            ^ block[(30 as libc::c_int & 15 as libc::c_int) as usize])
            >> 32 as libc::c_int - 1 as libc::c_int;
    qq[4 as libc::c_int
        as usize] = (qq[4 as libc::c_int as usize])
        .wrapping_add(
            (qq[1 as libc::c_int as usize] ^ qq[2 as libc::c_int as usize]
                ^ qq[3 as libc::c_int as usize])
                .wrapping_add(block[(30 as libc::c_int & 15 as libc::c_int) as usize])
                .wrapping_add(0x6ed9eba1 as libc::c_int as libc::c_uint)
                .wrapping_add(
                    qq[0 as libc::c_int as usize] << 5 as libc::c_int
                        | qq[0 as libc::c_int as usize]
                            >> 32 as libc::c_int - 5 as libc::c_int,
                ),
        );
    qq[1 as libc::c_int
        as usize] = qq[1 as libc::c_int as usize] << 32 as libc::c_int - 2 as libc::c_int
        | qq[1 as libc::c_int as usize] >> 2 as libc::c_int;
    block[(31 as libc::c_int & 15 as libc::c_int)
        as usize] = (block[(31 as libc::c_int + 13 as libc::c_int & 15 as libc::c_int)
        as usize]
        ^ block[(31 as libc::c_int + 8 as libc::c_int & 15 as libc::c_int) as usize]
        ^ block[(31 as libc::c_int + 2 as libc::c_int & 15 as libc::c_int) as usize]
        ^ block[(31 as libc::c_int & 15 as libc::c_int) as usize]) << 1 as libc::c_int
        | (block[(31 as libc::c_int + 13 as libc::c_int & 15 as libc::c_int) as usize]
            ^ block[(31 as libc::c_int + 8 as libc::c_int & 15 as libc::c_int) as usize]
            ^ block[(31 as libc::c_int + 2 as libc::c_int & 15 as libc::c_int) as usize]
            ^ block[(31 as libc::c_int & 15 as libc::c_int) as usize])
            >> 32 as libc::c_int - 1 as libc::c_int;
    qq[3 as libc::c_int
        as usize] = (qq[3 as libc::c_int as usize])
        .wrapping_add(
            (qq[0 as libc::c_int as usize] ^ qq[1 as libc::c_int as usize]
                ^ qq[2 as libc::c_int as usize])
                .wrapping_add(block[(31 as libc::c_int & 15 as libc::c_int) as usize])
                .wrapping_add(0x6ed9eba1 as libc::c_int as libc::c_uint)
                .wrapping_add(
                    qq[4 as libc::c_int as usize] << 5 as libc::c_int
                        | qq[4 as libc::c_int as usize]
                            >> 32 as libc::c_int - 5 as libc::c_int,
                ),
        );
    qq[0 as libc::c_int
        as usize] = qq[0 as libc::c_int as usize] << 32 as libc::c_int - 2 as libc::c_int
        | qq[0 as libc::c_int as usize] >> 2 as libc::c_int;
    block[(32 as libc::c_int & 15 as libc::c_int)
        as usize] = (block[(32 as libc::c_int + 13 as libc::c_int & 15 as libc::c_int)
        as usize]
        ^ block[(32 as libc::c_int + 8 as libc::c_int & 15 as libc::c_int) as usize]
        ^ block[(32 as libc::c_int + 2 as libc::c_int & 15 as libc::c_int) as usize]
        ^ block[(32 as libc::c_int & 15 as libc::c_int) as usize]) << 1 as libc::c_int
        | (block[(32 as libc::c_int + 13 as libc::c_int & 15 as libc::c_int) as usize]
            ^ block[(32 as libc::c_int + 8 as libc::c_int & 15 as libc::c_int) as usize]
            ^ block[(32 as libc::c_int + 2 as libc::c_int & 15 as libc::c_int) as usize]
            ^ block[(32 as libc::c_int & 15 as libc::c_int) as usize])
            >> 32 as libc::c_int - 1 as libc::c_int;
    qq[2 as libc::c_int
        as usize] = (qq[2 as libc::c_int as usize])
        .wrapping_add(
            (qq[4 as libc::c_int as usize] ^ qq[0 as libc::c_int as usize]
                ^ qq[1 as libc::c_int as usize])
                .wrapping_add(block[(32 as libc::c_int & 15 as libc::c_int) as usize])
                .wrapping_add(0x6ed9eba1 as libc::c_int as libc::c_uint)
                .wrapping_add(
                    qq[3 as libc::c_int as usize] << 5 as libc::c_int
                        | qq[3 as libc::c_int as usize]
                            >> 32 as libc::c_int - 5 as libc::c_int,
                ),
        );
    qq[4 as libc::c_int
        as usize] = qq[4 as libc::c_int as usize] << 32 as libc::c_int - 2 as libc::c_int
        | qq[4 as libc::c_int as usize] >> 2 as libc::c_int;
    block[(33 as libc::c_int & 15 as libc::c_int)
        as usize] = (block[(33 as libc::c_int + 13 as libc::c_int & 15 as libc::c_int)
        as usize]
        ^ block[(33 as libc::c_int + 8 as libc::c_int & 15 as libc::c_int) as usize]
        ^ block[(33 as libc::c_int + 2 as libc::c_int & 15 as libc::c_int) as usize]
        ^ block[(33 as libc::c_int & 15 as libc::c_int) as usize]) << 1 as libc::c_int
        | (block[(33 as libc::c_int + 13 as libc::c_int & 15 as libc::c_int) as usize]
            ^ block[(33 as libc::c_int + 8 as libc::c_int & 15 as libc::c_int) as usize]
            ^ block[(33 as libc::c_int + 2 as libc::c_int & 15 as libc::c_int) as usize]
            ^ block[(33 as libc::c_int & 15 as libc::c_int) as usize])
            >> 32 as libc::c_int - 1 as libc::c_int;
    qq[1 as libc::c_int
        as usize] = (qq[1 as libc::c_int as usize])
        .wrapping_add(
            (qq[3 as libc::c_int as usize] ^ qq[4 as libc::c_int as usize]
                ^ qq[0 as libc::c_int as usize])
                .wrapping_add(block[(33 as libc::c_int & 15 as libc::c_int) as usize])
                .wrapping_add(0x6ed9eba1 as libc::c_int as libc::c_uint)
                .wrapping_add(
                    qq[2 as libc::c_int as usize] << 5 as libc::c_int
                        | qq[2 as libc::c_int as usize]
                            >> 32 as libc::c_int - 5 as libc::c_int,
                ),
        );
    qq[3 as libc::c_int
        as usize] = qq[3 as libc::c_int as usize] << 32 as libc::c_int - 2 as libc::c_int
        | qq[3 as libc::c_int as usize] >> 2 as libc::c_int;
    block[(34 as libc::c_int & 15 as libc::c_int)
        as usize] = (block[(34 as libc::c_int + 13 as libc::c_int & 15 as libc::c_int)
        as usize]
        ^ block[(34 as libc::c_int + 8 as libc::c_int & 15 as libc::c_int) as usize]
        ^ block[(34 as libc::c_int + 2 as libc::c_int & 15 as libc::c_int) as usize]
        ^ block[(34 as libc::c_int & 15 as libc::c_int) as usize]) << 1 as libc::c_int
        | (block[(34 as libc::c_int + 13 as libc::c_int & 15 as libc::c_int) as usize]
            ^ block[(34 as libc::c_int + 8 as libc::c_int & 15 as libc::c_int) as usize]
            ^ block[(34 as libc::c_int + 2 as libc::c_int & 15 as libc::c_int) as usize]
            ^ block[(34 as libc::c_int & 15 as libc::c_int) as usize])
            >> 32 as libc::c_int - 1 as libc::c_int;
    qq[0 as libc::c_int
        as usize] = (qq[0 as libc::c_int as usize])
        .wrapping_add(
            (qq[2 as libc::c_int as usize] ^ qq[3 as libc::c_int as usize]
                ^ qq[4 as libc::c_int as usize])
                .wrapping_add(block[(34 as libc::c_int & 15 as libc::c_int) as usize])
                .wrapping_add(0x6ed9eba1 as libc::c_int as libc::c_uint)
                .wrapping_add(
                    qq[1 as libc::c_int as usize] << 5 as libc::c_int
                        | qq[1 as libc::c_int as usize]
                            >> 32 as libc::c_int - 5 as libc::c_int,
                ),
        );
    qq[2 as libc::c_int
        as usize] = qq[2 as libc::c_int as usize] << 32 as libc::c_int - 2 as libc::c_int
        | qq[2 as libc::c_int as usize] >> 2 as libc::c_int;
    block[(35 as libc::c_int & 15 as libc::c_int)
        as usize] = (block[(35 as libc::c_int + 13 as libc::c_int & 15 as libc::c_int)
        as usize]
        ^ block[(35 as libc::c_int + 8 as libc::c_int & 15 as libc::c_int) as usize]
        ^ block[(35 as libc::c_int + 2 as libc::c_int & 15 as libc::c_int) as usize]
        ^ block[(35 as libc::c_int & 15 as libc::c_int) as usize]) << 1 as libc::c_int
        | (block[(35 as libc::c_int + 13 as libc::c_int & 15 as libc::c_int) as usize]
            ^ block[(35 as libc::c_int + 8 as libc::c_int & 15 as libc::c_int) as usize]
            ^ block[(35 as libc::c_int + 2 as libc::c_int & 15 as libc::c_int) as usize]
            ^ block[(35 as libc::c_int & 15 as libc::c_int) as usize])
            >> 32 as libc::c_int - 1 as libc::c_int;
    qq[4 as libc::c_int
        as usize] = (qq[4 as libc::c_int as usize])
        .wrapping_add(
            (qq[1 as libc::c_int as usize] ^ qq[2 as libc::c_int as usize]
                ^ qq[3 as libc::c_int as usize])
                .wrapping_add(block[(35 as libc::c_int & 15 as libc::c_int) as usize])
                .wrapping_add(0x6ed9eba1 as libc::c_int as libc::c_uint)
                .wrapping_add(
                    qq[0 as libc::c_int as usize] << 5 as libc::c_int
                        | qq[0 as libc::c_int as usize]
                            >> 32 as libc::c_int - 5 as libc::c_int,
                ),
        );
    qq[1 as libc::c_int
        as usize] = qq[1 as libc::c_int as usize] << 32 as libc::c_int - 2 as libc::c_int
        | qq[1 as libc::c_int as usize] >> 2 as libc::c_int;
    block[(36 as libc::c_int & 15 as libc::c_int)
        as usize] = (block[(36 as libc::c_int + 13 as libc::c_int & 15 as libc::c_int)
        as usize]
        ^ block[(36 as libc::c_int + 8 as libc::c_int & 15 as libc::c_int) as usize]
        ^ block[(36 as libc::c_int + 2 as libc::c_int & 15 as libc::c_int) as usize]
        ^ block[(36 as libc::c_int & 15 as libc::c_int) as usize]) << 1 as libc::c_int
        | (block[(36 as libc::c_int + 13 as libc::c_int & 15 as libc::c_int) as usize]
            ^ block[(36 as libc::c_int + 8 as libc::c_int & 15 as libc::c_int) as usize]
            ^ block[(36 as libc::c_int + 2 as libc::c_int & 15 as libc::c_int) as usize]
            ^ block[(36 as libc::c_int & 15 as libc::c_int) as usize])
            >> 32 as libc::c_int - 1 as libc::c_int;
    qq[3 as libc::c_int
        as usize] = (qq[3 as libc::c_int as usize])
        .wrapping_add(
            (qq[0 as libc::c_int as usize] ^ qq[1 as libc::c_int as usize]
                ^ qq[2 as libc::c_int as usize])
                .wrapping_add(block[(36 as libc::c_int & 15 as libc::c_int) as usize])
                .wrapping_add(0x6ed9eba1 as libc::c_int as libc::c_uint)
                .wrapping_add(
                    qq[4 as libc::c_int as usize] << 5 as libc::c_int
                        | qq[4 as libc::c_int as usize]
                            >> 32 as libc::c_int - 5 as libc::c_int,
                ),
        );
    qq[0 as libc::c_int
        as usize] = qq[0 as libc::c_int as usize] << 32 as libc::c_int - 2 as libc::c_int
        | qq[0 as libc::c_int as usize] >> 2 as libc::c_int;
    block[(37 as libc::c_int & 15 as libc::c_int)
        as usize] = (block[(37 as libc::c_int + 13 as libc::c_int & 15 as libc::c_int)
        as usize]
        ^ block[(37 as libc::c_int + 8 as libc::c_int & 15 as libc::c_int) as usize]
        ^ block[(37 as libc::c_int + 2 as libc::c_int & 15 as libc::c_int) as usize]
        ^ block[(37 as libc::c_int & 15 as libc::c_int) as usize]) << 1 as libc::c_int
        | (block[(37 as libc::c_int + 13 as libc::c_int & 15 as libc::c_int) as usize]
            ^ block[(37 as libc::c_int + 8 as libc::c_int & 15 as libc::c_int) as usize]
            ^ block[(37 as libc::c_int + 2 as libc::c_int & 15 as libc::c_int) as usize]
            ^ block[(37 as libc::c_int & 15 as libc::c_int) as usize])
            >> 32 as libc::c_int - 1 as libc::c_int;
    qq[2 as libc::c_int
        as usize] = (qq[2 as libc::c_int as usize])
        .wrapping_add(
            (qq[4 as libc::c_int as usize] ^ qq[0 as libc::c_int as usize]
                ^ qq[1 as libc::c_int as usize])
                .wrapping_add(block[(37 as libc::c_int & 15 as libc::c_int) as usize])
                .wrapping_add(0x6ed9eba1 as libc::c_int as libc::c_uint)
                .wrapping_add(
                    qq[3 as libc::c_int as usize] << 5 as libc::c_int
                        | qq[3 as libc::c_int as usize]
                            >> 32 as libc::c_int - 5 as libc::c_int,
                ),
        );
    qq[4 as libc::c_int
        as usize] = qq[4 as libc::c_int as usize] << 32 as libc::c_int - 2 as libc::c_int
        | qq[4 as libc::c_int as usize] >> 2 as libc::c_int;
    block[(38 as libc::c_int & 15 as libc::c_int)
        as usize] = (block[(38 as libc::c_int + 13 as libc::c_int & 15 as libc::c_int)
        as usize]
        ^ block[(38 as libc::c_int + 8 as libc::c_int & 15 as libc::c_int) as usize]
        ^ block[(38 as libc::c_int + 2 as libc::c_int & 15 as libc::c_int) as usize]
        ^ block[(38 as libc::c_int & 15 as libc::c_int) as usize]) << 1 as libc::c_int
        | (block[(38 as libc::c_int + 13 as libc::c_int & 15 as libc::c_int) as usize]
            ^ block[(38 as libc::c_int + 8 as libc::c_int & 15 as libc::c_int) as usize]
            ^ block[(38 as libc::c_int + 2 as libc::c_int & 15 as libc::c_int) as usize]
            ^ block[(38 as libc::c_int & 15 as libc::c_int) as usize])
            >> 32 as libc::c_int - 1 as libc::c_int;
    qq[1 as libc::c_int
        as usize] = (qq[1 as libc::c_int as usize])
        .wrapping_add(
            (qq[3 as libc::c_int as usize] ^ qq[4 as libc::c_int as usize]
                ^ qq[0 as libc::c_int as usize])
                .wrapping_add(block[(38 as libc::c_int & 15 as libc::c_int) as usize])
                .wrapping_add(0x6ed9eba1 as libc::c_int as libc::c_uint)
                .wrapping_add(
                    qq[2 as libc::c_int as usize] << 5 as libc::c_int
                        | qq[2 as libc::c_int as usize]
                            >> 32 as libc::c_int - 5 as libc::c_int,
                ),
        );
    qq[3 as libc::c_int
        as usize] = qq[3 as libc::c_int as usize] << 32 as libc::c_int - 2 as libc::c_int
        | qq[3 as libc::c_int as usize] >> 2 as libc::c_int;
    block[(39 as libc::c_int & 15 as libc::c_int)
        as usize] = (block[(39 as libc::c_int + 13 as libc::c_int & 15 as libc::c_int)
        as usize]
        ^ block[(39 as libc::c_int + 8 as libc::c_int & 15 as libc::c_int) as usize]
        ^ block[(39 as libc::c_int + 2 as libc::c_int & 15 as libc::c_int) as usize]
        ^ block[(39 as libc::c_int & 15 as libc::c_int) as usize]) << 1 as libc::c_int
        | (block[(39 as libc::c_int + 13 as libc::c_int & 15 as libc::c_int) as usize]
            ^ block[(39 as libc::c_int + 8 as libc::c_int & 15 as libc::c_int) as usize]
            ^ block[(39 as libc::c_int + 2 as libc::c_int & 15 as libc::c_int) as usize]
            ^ block[(39 as libc::c_int & 15 as libc::c_int) as usize])
            >> 32 as libc::c_int - 1 as libc::c_int;
    qq[0 as libc::c_int
        as usize] = (qq[0 as libc::c_int as usize])
        .wrapping_add(
            (qq[2 as libc::c_int as usize] ^ qq[3 as libc::c_int as usize]
                ^ qq[4 as libc::c_int as usize])
                .wrapping_add(block[(39 as libc::c_int & 15 as libc::c_int) as usize])
                .wrapping_add(0x6ed9eba1 as libc::c_int as libc::c_uint)
                .wrapping_add(
                    qq[1 as libc::c_int as usize] << 5 as libc::c_int
                        | qq[1 as libc::c_int as usize]
                            >> 32 as libc::c_int - 5 as libc::c_int,
                ),
        );
    qq[2 as libc::c_int
        as usize] = qq[2 as libc::c_int as usize] << 32 as libc::c_int - 2 as libc::c_int
        | qq[2 as libc::c_int as usize] >> 2 as libc::c_int;
    block[(40 as libc::c_int & 15 as libc::c_int)
        as usize] = (block[(40 as libc::c_int + 13 as libc::c_int & 15 as libc::c_int)
        as usize]
        ^ block[(40 as libc::c_int + 8 as libc::c_int & 15 as libc::c_int) as usize]
        ^ block[(40 as libc::c_int + 2 as libc::c_int & 15 as libc::c_int) as usize]
        ^ block[(40 as libc::c_int & 15 as libc::c_int) as usize]) << 1 as libc::c_int
        | (block[(40 as libc::c_int + 13 as libc::c_int & 15 as libc::c_int) as usize]
            ^ block[(40 as libc::c_int + 8 as libc::c_int & 15 as libc::c_int) as usize]
            ^ block[(40 as libc::c_int + 2 as libc::c_int & 15 as libc::c_int) as usize]
            ^ block[(40 as libc::c_int & 15 as libc::c_int) as usize])
            >> 32 as libc::c_int - 1 as libc::c_int;
    qq[4 as libc::c_int
        as usize] = (qq[4 as libc::c_int as usize])
        .wrapping_add(
            ((qq[1 as libc::c_int as usize] | qq[2 as libc::c_int as usize])
                & qq[3 as libc::c_int as usize]
                | qq[1 as libc::c_int as usize] & qq[2 as libc::c_int as usize])
                .wrapping_add(block[(40 as libc::c_int & 15 as libc::c_int) as usize])
                .wrapping_add(0x8f1bbcdc as libc::c_uint)
                .wrapping_add(
                    qq[0 as libc::c_int as usize] << 5 as libc::c_int
                        | qq[0 as libc::c_int as usize]
                            >> 32 as libc::c_int - 5 as libc::c_int,
                ),
        );
    qq[1 as libc::c_int
        as usize] = qq[1 as libc::c_int as usize] << 32 as libc::c_int - 2 as libc::c_int
        | qq[1 as libc::c_int as usize] >> 2 as libc::c_int;
    block[(41 as libc::c_int & 15 as libc::c_int)
        as usize] = (block[(41 as libc::c_int + 13 as libc::c_int & 15 as libc::c_int)
        as usize]
        ^ block[(41 as libc::c_int + 8 as libc::c_int & 15 as libc::c_int) as usize]
        ^ block[(41 as libc::c_int + 2 as libc::c_int & 15 as libc::c_int) as usize]
        ^ block[(41 as libc::c_int & 15 as libc::c_int) as usize]) << 1 as libc::c_int
        | (block[(41 as libc::c_int + 13 as libc::c_int & 15 as libc::c_int) as usize]
            ^ block[(41 as libc::c_int + 8 as libc::c_int & 15 as libc::c_int) as usize]
            ^ block[(41 as libc::c_int + 2 as libc::c_int & 15 as libc::c_int) as usize]
            ^ block[(41 as libc::c_int & 15 as libc::c_int) as usize])
            >> 32 as libc::c_int - 1 as libc::c_int;
    qq[3 as libc::c_int
        as usize] = (qq[3 as libc::c_int as usize])
        .wrapping_add(
            ((qq[0 as libc::c_int as usize] | qq[1 as libc::c_int as usize])
                & qq[2 as libc::c_int as usize]
                | qq[0 as libc::c_int as usize] & qq[1 as libc::c_int as usize])
                .wrapping_add(block[(41 as libc::c_int & 15 as libc::c_int) as usize])
                .wrapping_add(0x8f1bbcdc as libc::c_uint)
                .wrapping_add(
                    qq[4 as libc::c_int as usize] << 5 as libc::c_int
                        | qq[4 as libc::c_int as usize]
                            >> 32 as libc::c_int - 5 as libc::c_int,
                ),
        );
    qq[0 as libc::c_int
        as usize] = qq[0 as libc::c_int as usize] << 32 as libc::c_int - 2 as libc::c_int
        | qq[0 as libc::c_int as usize] >> 2 as libc::c_int;
    block[(42 as libc::c_int & 15 as libc::c_int)
        as usize] = (block[(42 as libc::c_int + 13 as libc::c_int & 15 as libc::c_int)
        as usize]
        ^ block[(42 as libc::c_int + 8 as libc::c_int & 15 as libc::c_int) as usize]
        ^ block[(42 as libc::c_int + 2 as libc::c_int & 15 as libc::c_int) as usize]
        ^ block[(42 as libc::c_int & 15 as libc::c_int) as usize]) << 1 as libc::c_int
        | (block[(42 as libc::c_int + 13 as libc::c_int & 15 as libc::c_int) as usize]
            ^ block[(42 as libc::c_int + 8 as libc::c_int & 15 as libc::c_int) as usize]
            ^ block[(42 as libc::c_int + 2 as libc::c_int & 15 as libc::c_int) as usize]
            ^ block[(42 as libc::c_int & 15 as libc::c_int) as usize])
            >> 32 as libc::c_int - 1 as libc::c_int;
    qq[2 as libc::c_int
        as usize] = (qq[2 as libc::c_int as usize])
        .wrapping_add(
            ((qq[4 as libc::c_int as usize] | qq[0 as libc::c_int as usize])
                & qq[1 as libc::c_int as usize]
                | qq[4 as libc::c_int as usize] & qq[0 as libc::c_int as usize])
                .wrapping_add(block[(42 as libc::c_int & 15 as libc::c_int) as usize])
                .wrapping_add(0x8f1bbcdc as libc::c_uint)
                .wrapping_add(
                    qq[3 as libc::c_int as usize] << 5 as libc::c_int
                        | qq[3 as libc::c_int as usize]
                            >> 32 as libc::c_int - 5 as libc::c_int,
                ),
        );
    qq[4 as libc::c_int
        as usize] = qq[4 as libc::c_int as usize] << 32 as libc::c_int - 2 as libc::c_int
        | qq[4 as libc::c_int as usize] >> 2 as libc::c_int;
    block[(43 as libc::c_int & 15 as libc::c_int)
        as usize] = (block[(43 as libc::c_int + 13 as libc::c_int & 15 as libc::c_int)
        as usize]
        ^ block[(43 as libc::c_int + 8 as libc::c_int & 15 as libc::c_int) as usize]
        ^ block[(43 as libc::c_int + 2 as libc::c_int & 15 as libc::c_int) as usize]
        ^ block[(43 as libc::c_int & 15 as libc::c_int) as usize]) << 1 as libc::c_int
        | (block[(43 as libc::c_int + 13 as libc::c_int & 15 as libc::c_int) as usize]
            ^ block[(43 as libc::c_int + 8 as libc::c_int & 15 as libc::c_int) as usize]
            ^ block[(43 as libc::c_int + 2 as libc::c_int & 15 as libc::c_int) as usize]
            ^ block[(43 as libc::c_int & 15 as libc::c_int) as usize])
            >> 32 as libc::c_int - 1 as libc::c_int;
    qq[1 as libc::c_int
        as usize] = (qq[1 as libc::c_int as usize])
        .wrapping_add(
            ((qq[3 as libc::c_int as usize] | qq[4 as libc::c_int as usize])
                & qq[0 as libc::c_int as usize]
                | qq[3 as libc::c_int as usize] & qq[4 as libc::c_int as usize])
                .wrapping_add(block[(43 as libc::c_int & 15 as libc::c_int) as usize])
                .wrapping_add(0x8f1bbcdc as libc::c_uint)
                .wrapping_add(
                    qq[2 as libc::c_int as usize] << 5 as libc::c_int
                        | qq[2 as libc::c_int as usize]
                            >> 32 as libc::c_int - 5 as libc::c_int,
                ),
        );
    qq[3 as libc::c_int
        as usize] = qq[3 as libc::c_int as usize] << 32 as libc::c_int - 2 as libc::c_int
        | qq[3 as libc::c_int as usize] >> 2 as libc::c_int;
    block[(44 as libc::c_int & 15 as libc::c_int)
        as usize] = (block[(44 as libc::c_int + 13 as libc::c_int & 15 as libc::c_int)
        as usize]
        ^ block[(44 as libc::c_int + 8 as libc::c_int & 15 as libc::c_int) as usize]
        ^ block[(44 as libc::c_int + 2 as libc::c_int & 15 as libc::c_int) as usize]
        ^ block[(44 as libc::c_int & 15 as libc::c_int) as usize]) << 1 as libc::c_int
        | (block[(44 as libc::c_int + 13 as libc::c_int & 15 as libc::c_int) as usize]
            ^ block[(44 as libc::c_int + 8 as libc::c_int & 15 as libc::c_int) as usize]
            ^ block[(44 as libc::c_int + 2 as libc::c_int & 15 as libc::c_int) as usize]
            ^ block[(44 as libc::c_int & 15 as libc::c_int) as usize])
            >> 32 as libc::c_int - 1 as libc::c_int;
    qq[0 as libc::c_int
        as usize] = (qq[0 as libc::c_int as usize])
        .wrapping_add(
            ((qq[2 as libc::c_int as usize] | qq[3 as libc::c_int as usize])
                & qq[4 as libc::c_int as usize]
                | qq[2 as libc::c_int as usize] & qq[3 as libc::c_int as usize])
                .wrapping_add(block[(44 as libc::c_int & 15 as libc::c_int) as usize])
                .wrapping_add(0x8f1bbcdc as libc::c_uint)
                .wrapping_add(
                    qq[1 as libc::c_int as usize] << 5 as libc::c_int
                        | qq[1 as libc::c_int as usize]
                            >> 32 as libc::c_int - 5 as libc::c_int,
                ),
        );
    qq[2 as libc::c_int
        as usize] = qq[2 as libc::c_int as usize] << 32 as libc::c_int - 2 as libc::c_int
        | qq[2 as libc::c_int as usize] >> 2 as libc::c_int;
    block[(45 as libc::c_int & 15 as libc::c_int)
        as usize] = (block[(45 as libc::c_int + 13 as libc::c_int & 15 as libc::c_int)
        as usize]
        ^ block[(45 as libc::c_int + 8 as libc::c_int & 15 as libc::c_int) as usize]
        ^ block[(45 as libc::c_int + 2 as libc::c_int & 15 as libc::c_int) as usize]
        ^ block[(45 as libc::c_int & 15 as libc::c_int) as usize]) << 1 as libc::c_int
        | (block[(45 as libc::c_int + 13 as libc::c_int & 15 as libc::c_int) as usize]
            ^ block[(45 as libc::c_int + 8 as libc::c_int & 15 as libc::c_int) as usize]
            ^ block[(45 as libc::c_int + 2 as libc::c_int & 15 as libc::c_int) as usize]
            ^ block[(45 as libc::c_int & 15 as libc::c_int) as usize])
            >> 32 as libc::c_int - 1 as libc::c_int;
    qq[4 as libc::c_int
        as usize] = (qq[4 as libc::c_int as usize])
        .wrapping_add(
            ((qq[1 as libc::c_int as usize] | qq[2 as libc::c_int as usize])
                & qq[3 as libc::c_int as usize]
                | qq[1 as libc::c_int as usize] & qq[2 as libc::c_int as usize])
                .wrapping_add(block[(45 as libc::c_int & 15 as libc::c_int) as usize])
                .wrapping_add(0x8f1bbcdc as libc::c_uint)
                .wrapping_add(
                    qq[0 as libc::c_int as usize] << 5 as libc::c_int
                        | qq[0 as libc::c_int as usize]
                            >> 32 as libc::c_int - 5 as libc::c_int,
                ),
        );
    qq[1 as libc::c_int
        as usize] = qq[1 as libc::c_int as usize] << 32 as libc::c_int - 2 as libc::c_int
        | qq[1 as libc::c_int as usize] >> 2 as libc::c_int;
    block[(46 as libc::c_int & 15 as libc::c_int)
        as usize] = (block[(46 as libc::c_int + 13 as libc::c_int & 15 as libc::c_int)
        as usize]
        ^ block[(46 as libc::c_int + 8 as libc::c_int & 15 as libc::c_int) as usize]
        ^ block[(46 as libc::c_int + 2 as libc::c_int & 15 as libc::c_int) as usize]
        ^ block[(46 as libc::c_int & 15 as libc::c_int) as usize]) << 1 as libc::c_int
        | (block[(46 as libc::c_int + 13 as libc::c_int & 15 as libc::c_int) as usize]
            ^ block[(46 as libc::c_int + 8 as libc::c_int & 15 as libc::c_int) as usize]
            ^ block[(46 as libc::c_int + 2 as libc::c_int & 15 as libc::c_int) as usize]
            ^ block[(46 as libc::c_int & 15 as libc::c_int) as usize])
            >> 32 as libc::c_int - 1 as libc::c_int;
    qq[3 as libc::c_int
        as usize] = (qq[3 as libc::c_int as usize])
        .wrapping_add(
            ((qq[0 as libc::c_int as usize] | qq[1 as libc::c_int as usize])
                & qq[2 as libc::c_int as usize]
                | qq[0 as libc::c_int as usize] & qq[1 as libc::c_int as usize])
                .wrapping_add(block[(46 as libc::c_int & 15 as libc::c_int) as usize])
                .wrapping_add(0x8f1bbcdc as libc::c_uint)
                .wrapping_add(
                    qq[4 as libc::c_int as usize] << 5 as libc::c_int
                        | qq[4 as libc::c_int as usize]
                            >> 32 as libc::c_int - 5 as libc::c_int,
                ),
        );
    qq[0 as libc::c_int
        as usize] = qq[0 as libc::c_int as usize] << 32 as libc::c_int - 2 as libc::c_int
        | qq[0 as libc::c_int as usize] >> 2 as libc::c_int;
    block[(47 as libc::c_int & 15 as libc::c_int)
        as usize] = (block[(47 as libc::c_int + 13 as libc::c_int & 15 as libc::c_int)
        as usize]
        ^ block[(47 as libc::c_int + 8 as libc::c_int & 15 as libc::c_int) as usize]
        ^ block[(47 as libc::c_int + 2 as libc::c_int & 15 as libc::c_int) as usize]
        ^ block[(47 as libc::c_int & 15 as libc::c_int) as usize]) << 1 as libc::c_int
        | (block[(47 as libc::c_int + 13 as libc::c_int & 15 as libc::c_int) as usize]
            ^ block[(47 as libc::c_int + 8 as libc::c_int & 15 as libc::c_int) as usize]
            ^ block[(47 as libc::c_int + 2 as libc::c_int & 15 as libc::c_int) as usize]
            ^ block[(47 as libc::c_int & 15 as libc::c_int) as usize])
            >> 32 as libc::c_int - 1 as libc::c_int;
    qq[2 as libc::c_int
        as usize] = (qq[2 as libc::c_int as usize])
        .wrapping_add(
            ((qq[4 as libc::c_int as usize] | qq[0 as libc::c_int as usize])
                & qq[1 as libc::c_int as usize]
                | qq[4 as libc::c_int as usize] & qq[0 as libc::c_int as usize])
                .wrapping_add(block[(47 as libc::c_int & 15 as libc::c_int) as usize])
                .wrapping_add(0x8f1bbcdc as libc::c_uint)
                .wrapping_add(
                    qq[3 as libc::c_int as usize] << 5 as libc::c_int
                        | qq[3 as libc::c_int as usize]
                            >> 32 as libc::c_int - 5 as libc::c_int,
                ),
        );
    qq[4 as libc::c_int
        as usize] = qq[4 as libc::c_int as usize] << 32 as libc::c_int - 2 as libc::c_int
        | qq[4 as libc::c_int as usize] >> 2 as libc::c_int;
    block[(48 as libc::c_int & 15 as libc::c_int)
        as usize] = (block[(48 as libc::c_int + 13 as libc::c_int & 15 as libc::c_int)
        as usize]
        ^ block[(48 as libc::c_int + 8 as libc::c_int & 15 as libc::c_int) as usize]
        ^ block[(48 as libc::c_int + 2 as libc::c_int & 15 as libc::c_int) as usize]
        ^ block[(48 as libc::c_int & 15 as libc::c_int) as usize]) << 1 as libc::c_int
        | (block[(48 as libc::c_int + 13 as libc::c_int & 15 as libc::c_int) as usize]
            ^ block[(48 as libc::c_int + 8 as libc::c_int & 15 as libc::c_int) as usize]
            ^ block[(48 as libc::c_int + 2 as libc::c_int & 15 as libc::c_int) as usize]
            ^ block[(48 as libc::c_int & 15 as libc::c_int) as usize])
            >> 32 as libc::c_int - 1 as libc::c_int;
    qq[1 as libc::c_int
        as usize] = (qq[1 as libc::c_int as usize])
        .wrapping_add(
            ((qq[3 as libc::c_int as usize] | qq[4 as libc::c_int as usize])
                & qq[0 as libc::c_int as usize]
                | qq[3 as libc::c_int as usize] & qq[4 as libc::c_int as usize])
                .wrapping_add(block[(48 as libc::c_int & 15 as libc::c_int) as usize])
                .wrapping_add(0x8f1bbcdc as libc::c_uint)
                .wrapping_add(
                    qq[2 as libc::c_int as usize] << 5 as libc::c_int
                        | qq[2 as libc::c_int as usize]
                            >> 32 as libc::c_int - 5 as libc::c_int,
                ),
        );
    qq[3 as libc::c_int
        as usize] = qq[3 as libc::c_int as usize] << 32 as libc::c_int - 2 as libc::c_int
        | qq[3 as libc::c_int as usize] >> 2 as libc::c_int;
    block[(49 as libc::c_int & 15 as libc::c_int)
        as usize] = (block[(49 as libc::c_int + 13 as libc::c_int & 15 as libc::c_int)
        as usize]
        ^ block[(49 as libc::c_int + 8 as libc::c_int & 15 as libc::c_int) as usize]
        ^ block[(49 as libc::c_int + 2 as libc::c_int & 15 as libc::c_int) as usize]
        ^ block[(49 as libc::c_int & 15 as libc::c_int) as usize]) << 1 as libc::c_int
        | (block[(49 as libc::c_int + 13 as libc::c_int & 15 as libc::c_int) as usize]
            ^ block[(49 as libc::c_int + 8 as libc::c_int & 15 as libc::c_int) as usize]
            ^ block[(49 as libc::c_int + 2 as libc::c_int & 15 as libc::c_int) as usize]
            ^ block[(49 as libc::c_int & 15 as libc::c_int) as usize])
            >> 32 as libc::c_int - 1 as libc::c_int;
    qq[0 as libc::c_int
        as usize] = (qq[0 as libc::c_int as usize])
        .wrapping_add(
            ((qq[2 as libc::c_int as usize] | qq[3 as libc::c_int as usize])
                & qq[4 as libc::c_int as usize]
                | qq[2 as libc::c_int as usize] & qq[3 as libc::c_int as usize])
                .wrapping_add(block[(49 as libc::c_int & 15 as libc::c_int) as usize])
                .wrapping_add(0x8f1bbcdc as libc::c_uint)
                .wrapping_add(
                    qq[1 as libc::c_int as usize] << 5 as libc::c_int
                        | qq[1 as libc::c_int as usize]
                            >> 32 as libc::c_int - 5 as libc::c_int,
                ),
        );
    qq[2 as libc::c_int
        as usize] = qq[2 as libc::c_int as usize] << 32 as libc::c_int - 2 as libc::c_int
        | qq[2 as libc::c_int as usize] >> 2 as libc::c_int;
    block[(50 as libc::c_int & 15 as libc::c_int)
        as usize] = (block[(50 as libc::c_int + 13 as libc::c_int & 15 as libc::c_int)
        as usize]
        ^ block[(50 as libc::c_int + 8 as libc::c_int & 15 as libc::c_int) as usize]
        ^ block[(50 as libc::c_int + 2 as libc::c_int & 15 as libc::c_int) as usize]
        ^ block[(50 as libc::c_int & 15 as libc::c_int) as usize]) << 1 as libc::c_int
        | (block[(50 as libc::c_int + 13 as libc::c_int & 15 as libc::c_int) as usize]
            ^ block[(50 as libc::c_int + 8 as libc::c_int & 15 as libc::c_int) as usize]
            ^ block[(50 as libc::c_int + 2 as libc::c_int & 15 as libc::c_int) as usize]
            ^ block[(50 as libc::c_int & 15 as libc::c_int) as usize])
            >> 32 as libc::c_int - 1 as libc::c_int;
    qq[4 as libc::c_int
        as usize] = (qq[4 as libc::c_int as usize])
        .wrapping_add(
            ((qq[1 as libc::c_int as usize] | qq[2 as libc::c_int as usize])
                & qq[3 as libc::c_int as usize]
                | qq[1 as libc::c_int as usize] & qq[2 as libc::c_int as usize])
                .wrapping_add(block[(50 as libc::c_int & 15 as libc::c_int) as usize])
                .wrapping_add(0x8f1bbcdc as libc::c_uint)
                .wrapping_add(
                    qq[0 as libc::c_int as usize] << 5 as libc::c_int
                        | qq[0 as libc::c_int as usize]
                            >> 32 as libc::c_int - 5 as libc::c_int,
                ),
        );
    qq[1 as libc::c_int
        as usize] = qq[1 as libc::c_int as usize] << 32 as libc::c_int - 2 as libc::c_int
        | qq[1 as libc::c_int as usize] >> 2 as libc::c_int;
    block[(51 as libc::c_int & 15 as libc::c_int)
        as usize] = (block[(51 as libc::c_int + 13 as libc::c_int & 15 as libc::c_int)
        as usize]
        ^ block[(51 as libc::c_int + 8 as libc::c_int & 15 as libc::c_int) as usize]
        ^ block[(51 as libc::c_int + 2 as libc::c_int & 15 as libc::c_int) as usize]
        ^ block[(51 as libc::c_int & 15 as libc::c_int) as usize]) << 1 as libc::c_int
        | (block[(51 as libc::c_int + 13 as libc::c_int & 15 as libc::c_int) as usize]
            ^ block[(51 as libc::c_int + 8 as libc::c_int & 15 as libc::c_int) as usize]
            ^ block[(51 as libc::c_int + 2 as libc::c_int & 15 as libc::c_int) as usize]
            ^ block[(51 as libc::c_int & 15 as libc::c_int) as usize])
            >> 32 as libc::c_int - 1 as libc::c_int;
    qq[3 as libc::c_int
        as usize] = (qq[3 as libc::c_int as usize])
        .wrapping_add(
            ((qq[0 as libc::c_int as usize] | qq[1 as libc::c_int as usize])
                & qq[2 as libc::c_int as usize]
                | qq[0 as libc::c_int as usize] & qq[1 as libc::c_int as usize])
                .wrapping_add(block[(51 as libc::c_int & 15 as libc::c_int) as usize])
                .wrapping_add(0x8f1bbcdc as libc::c_uint)
                .wrapping_add(
                    qq[4 as libc::c_int as usize] << 5 as libc::c_int
                        | qq[4 as libc::c_int as usize]
                            >> 32 as libc::c_int - 5 as libc::c_int,
                ),
        );
    qq[0 as libc::c_int
        as usize] = qq[0 as libc::c_int as usize] << 32 as libc::c_int - 2 as libc::c_int
        | qq[0 as libc::c_int as usize] >> 2 as libc::c_int;
    block[(52 as libc::c_int & 15 as libc::c_int)
        as usize] = (block[(52 as libc::c_int + 13 as libc::c_int & 15 as libc::c_int)
        as usize]
        ^ block[(52 as libc::c_int + 8 as libc::c_int & 15 as libc::c_int) as usize]
        ^ block[(52 as libc::c_int + 2 as libc::c_int & 15 as libc::c_int) as usize]
        ^ block[(52 as libc::c_int & 15 as libc::c_int) as usize]) << 1 as libc::c_int
        | (block[(52 as libc::c_int + 13 as libc::c_int & 15 as libc::c_int) as usize]
            ^ block[(52 as libc::c_int + 8 as libc::c_int & 15 as libc::c_int) as usize]
            ^ block[(52 as libc::c_int + 2 as libc::c_int & 15 as libc::c_int) as usize]
            ^ block[(52 as libc::c_int & 15 as libc::c_int) as usize])
            >> 32 as libc::c_int - 1 as libc::c_int;
    qq[2 as libc::c_int
        as usize] = (qq[2 as libc::c_int as usize])
        .wrapping_add(
            ((qq[4 as libc::c_int as usize] | qq[0 as libc::c_int as usize])
                & qq[1 as libc::c_int as usize]
                | qq[4 as libc::c_int as usize] & qq[0 as libc::c_int as usize])
                .wrapping_add(block[(52 as libc::c_int & 15 as libc::c_int) as usize])
                .wrapping_add(0x8f1bbcdc as libc::c_uint)
                .wrapping_add(
                    qq[3 as libc::c_int as usize] << 5 as libc::c_int
                        | qq[3 as libc::c_int as usize]
                            >> 32 as libc::c_int - 5 as libc::c_int,
                ),
        );
    qq[4 as libc::c_int
        as usize] = qq[4 as libc::c_int as usize] << 32 as libc::c_int - 2 as libc::c_int
        | qq[4 as libc::c_int as usize] >> 2 as libc::c_int;
    block[(53 as libc::c_int & 15 as libc::c_int)
        as usize] = (block[(53 as libc::c_int + 13 as libc::c_int & 15 as libc::c_int)
        as usize]
        ^ block[(53 as libc::c_int + 8 as libc::c_int & 15 as libc::c_int) as usize]
        ^ block[(53 as libc::c_int + 2 as libc::c_int & 15 as libc::c_int) as usize]
        ^ block[(53 as libc::c_int & 15 as libc::c_int) as usize]) << 1 as libc::c_int
        | (block[(53 as libc::c_int + 13 as libc::c_int & 15 as libc::c_int) as usize]
            ^ block[(53 as libc::c_int + 8 as libc::c_int & 15 as libc::c_int) as usize]
            ^ block[(53 as libc::c_int + 2 as libc::c_int & 15 as libc::c_int) as usize]
            ^ block[(53 as libc::c_int & 15 as libc::c_int) as usize])
            >> 32 as libc::c_int - 1 as libc::c_int;
    qq[1 as libc::c_int
        as usize] = (qq[1 as libc::c_int as usize])
        .wrapping_add(
            ((qq[3 as libc::c_int as usize] | qq[4 as libc::c_int as usize])
                & qq[0 as libc::c_int as usize]
                | qq[3 as libc::c_int as usize] & qq[4 as libc::c_int as usize])
                .wrapping_add(block[(53 as libc::c_int & 15 as libc::c_int) as usize])
                .wrapping_add(0x8f1bbcdc as libc::c_uint)
                .wrapping_add(
                    qq[2 as libc::c_int as usize] << 5 as libc::c_int
                        | qq[2 as libc::c_int as usize]
                            >> 32 as libc::c_int - 5 as libc::c_int,
                ),
        );
    qq[3 as libc::c_int
        as usize] = qq[3 as libc::c_int as usize] << 32 as libc::c_int - 2 as libc::c_int
        | qq[3 as libc::c_int as usize] >> 2 as libc::c_int;
    block[(54 as libc::c_int & 15 as libc::c_int)
        as usize] = (block[(54 as libc::c_int + 13 as libc::c_int & 15 as libc::c_int)
        as usize]
        ^ block[(54 as libc::c_int + 8 as libc::c_int & 15 as libc::c_int) as usize]
        ^ block[(54 as libc::c_int + 2 as libc::c_int & 15 as libc::c_int) as usize]
        ^ block[(54 as libc::c_int & 15 as libc::c_int) as usize]) << 1 as libc::c_int
        | (block[(54 as libc::c_int + 13 as libc::c_int & 15 as libc::c_int) as usize]
            ^ block[(54 as libc::c_int + 8 as libc::c_int & 15 as libc::c_int) as usize]
            ^ block[(54 as libc::c_int + 2 as libc::c_int & 15 as libc::c_int) as usize]
            ^ block[(54 as libc::c_int & 15 as libc::c_int) as usize])
            >> 32 as libc::c_int - 1 as libc::c_int;
    qq[0 as libc::c_int
        as usize] = (qq[0 as libc::c_int as usize])
        .wrapping_add(
            ((qq[2 as libc::c_int as usize] | qq[3 as libc::c_int as usize])
                & qq[4 as libc::c_int as usize]
                | qq[2 as libc::c_int as usize] & qq[3 as libc::c_int as usize])
                .wrapping_add(block[(54 as libc::c_int & 15 as libc::c_int) as usize])
                .wrapping_add(0x8f1bbcdc as libc::c_uint)
                .wrapping_add(
                    qq[1 as libc::c_int as usize] << 5 as libc::c_int
                        | qq[1 as libc::c_int as usize]
                            >> 32 as libc::c_int - 5 as libc::c_int,
                ),
        );
    qq[2 as libc::c_int
        as usize] = qq[2 as libc::c_int as usize] << 32 as libc::c_int - 2 as libc::c_int
        | qq[2 as libc::c_int as usize] >> 2 as libc::c_int;
    block[(55 as libc::c_int & 15 as libc::c_int)
        as usize] = (block[(55 as libc::c_int + 13 as libc::c_int & 15 as libc::c_int)
        as usize]
        ^ block[(55 as libc::c_int + 8 as libc::c_int & 15 as libc::c_int) as usize]
        ^ block[(55 as libc::c_int + 2 as libc::c_int & 15 as libc::c_int) as usize]
        ^ block[(55 as libc::c_int & 15 as libc::c_int) as usize]) << 1 as libc::c_int
        | (block[(55 as libc::c_int + 13 as libc::c_int & 15 as libc::c_int) as usize]
            ^ block[(55 as libc::c_int + 8 as libc::c_int & 15 as libc::c_int) as usize]
            ^ block[(55 as libc::c_int + 2 as libc::c_int & 15 as libc::c_int) as usize]
            ^ block[(55 as libc::c_int & 15 as libc::c_int) as usize])
            >> 32 as libc::c_int - 1 as libc::c_int;
    qq[4 as libc::c_int
        as usize] = (qq[4 as libc::c_int as usize])
        .wrapping_add(
            ((qq[1 as libc::c_int as usize] | qq[2 as libc::c_int as usize])
                & qq[3 as libc::c_int as usize]
                | qq[1 as libc::c_int as usize] & qq[2 as libc::c_int as usize])
                .wrapping_add(block[(55 as libc::c_int & 15 as libc::c_int) as usize])
                .wrapping_add(0x8f1bbcdc as libc::c_uint)
                .wrapping_add(
                    qq[0 as libc::c_int as usize] << 5 as libc::c_int
                        | qq[0 as libc::c_int as usize]
                            >> 32 as libc::c_int - 5 as libc::c_int,
                ),
        );
    qq[1 as libc::c_int
        as usize] = qq[1 as libc::c_int as usize] << 32 as libc::c_int - 2 as libc::c_int
        | qq[1 as libc::c_int as usize] >> 2 as libc::c_int;
    block[(56 as libc::c_int & 15 as libc::c_int)
        as usize] = (block[(56 as libc::c_int + 13 as libc::c_int & 15 as libc::c_int)
        as usize]
        ^ block[(56 as libc::c_int + 8 as libc::c_int & 15 as libc::c_int) as usize]
        ^ block[(56 as libc::c_int + 2 as libc::c_int & 15 as libc::c_int) as usize]
        ^ block[(56 as libc::c_int & 15 as libc::c_int) as usize]) << 1 as libc::c_int
        | (block[(56 as libc::c_int + 13 as libc::c_int & 15 as libc::c_int) as usize]
            ^ block[(56 as libc::c_int + 8 as libc::c_int & 15 as libc::c_int) as usize]
            ^ block[(56 as libc::c_int + 2 as libc::c_int & 15 as libc::c_int) as usize]
            ^ block[(56 as libc::c_int & 15 as libc::c_int) as usize])
            >> 32 as libc::c_int - 1 as libc::c_int;
    qq[3 as libc::c_int
        as usize] = (qq[3 as libc::c_int as usize])
        .wrapping_add(
            ((qq[0 as libc::c_int as usize] | qq[1 as libc::c_int as usize])
                & qq[2 as libc::c_int as usize]
                | qq[0 as libc::c_int as usize] & qq[1 as libc::c_int as usize])
                .wrapping_add(block[(56 as libc::c_int & 15 as libc::c_int) as usize])
                .wrapping_add(0x8f1bbcdc as libc::c_uint)
                .wrapping_add(
                    qq[4 as libc::c_int as usize] << 5 as libc::c_int
                        | qq[4 as libc::c_int as usize]
                            >> 32 as libc::c_int - 5 as libc::c_int,
                ),
        );
    qq[0 as libc::c_int
        as usize] = qq[0 as libc::c_int as usize] << 32 as libc::c_int - 2 as libc::c_int
        | qq[0 as libc::c_int as usize] >> 2 as libc::c_int;
    block[(57 as libc::c_int & 15 as libc::c_int)
        as usize] = (block[(57 as libc::c_int + 13 as libc::c_int & 15 as libc::c_int)
        as usize]
        ^ block[(57 as libc::c_int + 8 as libc::c_int & 15 as libc::c_int) as usize]
        ^ block[(57 as libc::c_int + 2 as libc::c_int & 15 as libc::c_int) as usize]
        ^ block[(57 as libc::c_int & 15 as libc::c_int) as usize]) << 1 as libc::c_int
        | (block[(57 as libc::c_int + 13 as libc::c_int & 15 as libc::c_int) as usize]
            ^ block[(57 as libc::c_int + 8 as libc::c_int & 15 as libc::c_int) as usize]
            ^ block[(57 as libc::c_int + 2 as libc::c_int & 15 as libc::c_int) as usize]
            ^ block[(57 as libc::c_int & 15 as libc::c_int) as usize])
            >> 32 as libc::c_int - 1 as libc::c_int;
    qq[2 as libc::c_int
        as usize] = (qq[2 as libc::c_int as usize])
        .wrapping_add(
            ((qq[4 as libc::c_int as usize] | qq[0 as libc::c_int as usize])
                & qq[1 as libc::c_int as usize]
                | qq[4 as libc::c_int as usize] & qq[0 as libc::c_int as usize])
                .wrapping_add(block[(57 as libc::c_int & 15 as libc::c_int) as usize])
                .wrapping_add(0x8f1bbcdc as libc::c_uint)
                .wrapping_add(
                    qq[3 as libc::c_int as usize] << 5 as libc::c_int
                        | qq[3 as libc::c_int as usize]
                            >> 32 as libc::c_int - 5 as libc::c_int,
                ),
        );
    qq[4 as libc::c_int
        as usize] = qq[4 as libc::c_int as usize] << 32 as libc::c_int - 2 as libc::c_int
        | qq[4 as libc::c_int as usize] >> 2 as libc::c_int;
    block[(58 as libc::c_int & 15 as libc::c_int)
        as usize] = (block[(58 as libc::c_int + 13 as libc::c_int & 15 as libc::c_int)
        as usize]
        ^ block[(58 as libc::c_int + 8 as libc::c_int & 15 as libc::c_int) as usize]
        ^ block[(58 as libc::c_int + 2 as libc::c_int & 15 as libc::c_int) as usize]
        ^ block[(58 as libc::c_int & 15 as libc::c_int) as usize]) << 1 as libc::c_int
        | (block[(58 as libc::c_int + 13 as libc::c_int & 15 as libc::c_int) as usize]
            ^ block[(58 as libc::c_int + 8 as libc::c_int & 15 as libc::c_int) as usize]
            ^ block[(58 as libc::c_int + 2 as libc::c_int & 15 as libc::c_int) as usize]
            ^ block[(58 as libc::c_int & 15 as libc::c_int) as usize])
            >> 32 as libc::c_int - 1 as libc::c_int;
    qq[1 as libc::c_int
        as usize] = (qq[1 as libc::c_int as usize])
        .wrapping_add(
            ((qq[3 as libc::c_int as usize] | qq[4 as libc::c_int as usize])
                & qq[0 as libc::c_int as usize]
                | qq[3 as libc::c_int as usize] & qq[4 as libc::c_int as usize])
                .wrapping_add(block[(58 as libc::c_int & 15 as libc::c_int) as usize])
                .wrapping_add(0x8f1bbcdc as libc::c_uint)
                .wrapping_add(
                    qq[2 as libc::c_int as usize] << 5 as libc::c_int
                        | qq[2 as libc::c_int as usize]
                            >> 32 as libc::c_int - 5 as libc::c_int,
                ),
        );
    qq[3 as libc::c_int
        as usize] = qq[3 as libc::c_int as usize] << 32 as libc::c_int - 2 as libc::c_int
        | qq[3 as libc::c_int as usize] >> 2 as libc::c_int;
    block[(59 as libc::c_int & 15 as libc::c_int)
        as usize] = (block[(59 as libc::c_int + 13 as libc::c_int & 15 as libc::c_int)
        as usize]
        ^ block[(59 as libc::c_int + 8 as libc::c_int & 15 as libc::c_int) as usize]
        ^ block[(59 as libc::c_int + 2 as libc::c_int & 15 as libc::c_int) as usize]
        ^ block[(59 as libc::c_int & 15 as libc::c_int) as usize]) << 1 as libc::c_int
        | (block[(59 as libc::c_int + 13 as libc::c_int & 15 as libc::c_int) as usize]
            ^ block[(59 as libc::c_int + 8 as libc::c_int & 15 as libc::c_int) as usize]
            ^ block[(59 as libc::c_int + 2 as libc::c_int & 15 as libc::c_int) as usize]
            ^ block[(59 as libc::c_int & 15 as libc::c_int) as usize])
            >> 32 as libc::c_int - 1 as libc::c_int;
    qq[0 as libc::c_int
        as usize] = (qq[0 as libc::c_int as usize])
        .wrapping_add(
            ((qq[2 as libc::c_int as usize] | qq[3 as libc::c_int as usize])
                & qq[4 as libc::c_int as usize]
                | qq[2 as libc::c_int as usize] & qq[3 as libc::c_int as usize])
                .wrapping_add(block[(59 as libc::c_int & 15 as libc::c_int) as usize])
                .wrapping_add(0x8f1bbcdc as libc::c_uint)
                .wrapping_add(
                    qq[1 as libc::c_int as usize] << 5 as libc::c_int
                        | qq[1 as libc::c_int as usize]
                            >> 32 as libc::c_int - 5 as libc::c_int,
                ),
        );
    qq[2 as libc::c_int
        as usize] = qq[2 as libc::c_int as usize] << 32 as libc::c_int - 2 as libc::c_int
        | qq[2 as libc::c_int as usize] >> 2 as libc::c_int;
    block[(60 as libc::c_int & 15 as libc::c_int)
        as usize] = (block[(60 as libc::c_int + 13 as libc::c_int & 15 as libc::c_int)
        as usize]
        ^ block[(60 as libc::c_int + 8 as libc::c_int & 15 as libc::c_int) as usize]
        ^ block[(60 as libc::c_int + 2 as libc::c_int & 15 as libc::c_int) as usize]
        ^ block[(60 as libc::c_int & 15 as libc::c_int) as usize]) << 1 as libc::c_int
        | (block[(60 as libc::c_int + 13 as libc::c_int & 15 as libc::c_int) as usize]
            ^ block[(60 as libc::c_int + 8 as libc::c_int & 15 as libc::c_int) as usize]
            ^ block[(60 as libc::c_int + 2 as libc::c_int & 15 as libc::c_int) as usize]
            ^ block[(60 as libc::c_int & 15 as libc::c_int) as usize])
            >> 32 as libc::c_int - 1 as libc::c_int;
    qq[4 as libc::c_int
        as usize] = (qq[4 as libc::c_int as usize])
        .wrapping_add(
            (qq[1 as libc::c_int as usize] ^ qq[2 as libc::c_int as usize]
                ^ qq[3 as libc::c_int as usize])
                .wrapping_add(block[(60 as libc::c_int & 15 as libc::c_int) as usize])
                .wrapping_add(0xca62c1d6 as libc::c_uint)
                .wrapping_add(
                    qq[0 as libc::c_int as usize] << 5 as libc::c_int
                        | qq[0 as libc::c_int as usize]
                            >> 32 as libc::c_int - 5 as libc::c_int,
                ),
        );
    qq[1 as libc::c_int
        as usize] = qq[1 as libc::c_int as usize] << 32 as libc::c_int - 2 as libc::c_int
        | qq[1 as libc::c_int as usize] >> 2 as libc::c_int;
    block[(61 as libc::c_int & 15 as libc::c_int)
        as usize] = (block[(61 as libc::c_int + 13 as libc::c_int & 15 as libc::c_int)
        as usize]
        ^ block[(61 as libc::c_int + 8 as libc::c_int & 15 as libc::c_int) as usize]
        ^ block[(61 as libc::c_int + 2 as libc::c_int & 15 as libc::c_int) as usize]
        ^ block[(61 as libc::c_int & 15 as libc::c_int) as usize]) << 1 as libc::c_int
        | (block[(61 as libc::c_int + 13 as libc::c_int & 15 as libc::c_int) as usize]
            ^ block[(61 as libc::c_int + 8 as libc::c_int & 15 as libc::c_int) as usize]
            ^ block[(61 as libc::c_int + 2 as libc::c_int & 15 as libc::c_int) as usize]
            ^ block[(61 as libc::c_int & 15 as libc::c_int) as usize])
            >> 32 as libc::c_int - 1 as libc::c_int;
    qq[3 as libc::c_int
        as usize] = (qq[3 as libc::c_int as usize])
        .wrapping_add(
            (qq[0 as libc::c_int as usize] ^ qq[1 as libc::c_int as usize]
                ^ qq[2 as libc::c_int as usize])
                .wrapping_add(block[(61 as libc::c_int & 15 as libc::c_int) as usize])
                .wrapping_add(0xca62c1d6 as libc::c_uint)
                .wrapping_add(
                    qq[4 as libc::c_int as usize] << 5 as libc::c_int
                        | qq[4 as libc::c_int as usize]
                            >> 32 as libc::c_int - 5 as libc::c_int,
                ),
        );
    qq[0 as libc::c_int
        as usize] = qq[0 as libc::c_int as usize] << 32 as libc::c_int - 2 as libc::c_int
        | qq[0 as libc::c_int as usize] >> 2 as libc::c_int;
    block[(62 as libc::c_int & 15 as libc::c_int)
        as usize] = (block[(62 as libc::c_int + 13 as libc::c_int & 15 as libc::c_int)
        as usize]
        ^ block[(62 as libc::c_int + 8 as libc::c_int & 15 as libc::c_int) as usize]
        ^ block[(62 as libc::c_int + 2 as libc::c_int & 15 as libc::c_int) as usize]
        ^ block[(62 as libc::c_int & 15 as libc::c_int) as usize]) << 1 as libc::c_int
        | (block[(62 as libc::c_int + 13 as libc::c_int & 15 as libc::c_int) as usize]
            ^ block[(62 as libc::c_int + 8 as libc::c_int & 15 as libc::c_int) as usize]
            ^ block[(62 as libc::c_int + 2 as libc::c_int & 15 as libc::c_int) as usize]
            ^ block[(62 as libc::c_int & 15 as libc::c_int) as usize])
            >> 32 as libc::c_int - 1 as libc::c_int;
    qq[2 as libc::c_int
        as usize] = (qq[2 as libc::c_int as usize])
        .wrapping_add(
            (qq[4 as libc::c_int as usize] ^ qq[0 as libc::c_int as usize]
                ^ qq[1 as libc::c_int as usize])
                .wrapping_add(block[(62 as libc::c_int & 15 as libc::c_int) as usize])
                .wrapping_add(0xca62c1d6 as libc::c_uint)
                .wrapping_add(
                    qq[3 as libc::c_int as usize] << 5 as libc::c_int
                        | qq[3 as libc::c_int as usize]
                            >> 32 as libc::c_int - 5 as libc::c_int,
                ),
        );
    qq[4 as libc::c_int
        as usize] = qq[4 as libc::c_int as usize] << 32 as libc::c_int - 2 as libc::c_int
        | qq[4 as libc::c_int as usize] >> 2 as libc::c_int;
    block[(63 as libc::c_int & 15 as libc::c_int)
        as usize] = (block[(63 as libc::c_int + 13 as libc::c_int & 15 as libc::c_int)
        as usize]
        ^ block[(63 as libc::c_int + 8 as libc::c_int & 15 as libc::c_int) as usize]
        ^ block[(63 as libc::c_int + 2 as libc::c_int & 15 as libc::c_int) as usize]
        ^ block[(63 as libc::c_int & 15 as libc::c_int) as usize]) << 1 as libc::c_int
        | (block[(63 as libc::c_int + 13 as libc::c_int & 15 as libc::c_int) as usize]
            ^ block[(63 as libc::c_int + 8 as libc::c_int & 15 as libc::c_int) as usize]
            ^ block[(63 as libc::c_int + 2 as libc::c_int & 15 as libc::c_int) as usize]
            ^ block[(63 as libc::c_int & 15 as libc::c_int) as usize])
            >> 32 as libc::c_int - 1 as libc::c_int;
    qq[1 as libc::c_int
        as usize] = (qq[1 as libc::c_int as usize])
        .wrapping_add(
            (qq[3 as libc::c_int as usize] ^ qq[4 as libc::c_int as usize]
                ^ qq[0 as libc::c_int as usize])
                .wrapping_add(block[(63 as libc::c_int & 15 as libc::c_int) as usize])
                .wrapping_add(0xca62c1d6 as libc::c_uint)
                .wrapping_add(
                    qq[2 as libc::c_int as usize] << 5 as libc::c_int
                        | qq[2 as libc::c_int as usize]
                            >> 32 as libc::c_int - 5 as libc::c_int,
                ),
        );
    qq[3 as libc::c_int
        as usize] = qq[3 as libc::c_int as usize] << 32 as libc::c_int - 2 as libc::c_int
        | qq[3 as libc::c_int as usize] >> 2 as libc::c_int;
    block[(64 as libc::c_int & 15 as libc::c_int)
        as usize] = (block[(64 as libc::c_int + 13 as libc::c_int & 15 as libc::c_int)
        as usize]
        ^ block[(64 as libc::c_int + 8 as libc::c_int & 15 as libc::c_int) as usize]
        ^ block[(64 as libc::c_int + 2 as libc::c_int & 15 as libc::c_int) as usize]
        ^ block[(64 as libc::c_int & 15 as libc::c_int) as usize]) << 1 as libc::c_int
        | (block[(64 as libc::c_int + 13 as libc::c_int & 15 as libc::c_int) as usize]
            ^ block[(64 as libc::c_int + 8 as libc::c_int & 15 as libc::c_int) as usize]
            ^ block[(64 as libc::c_int + 2 as libc::c_int & 15 as libc::c_int) as usize]
            ^ block[(64 as libc::c_int & 15 as libc::c_int) as usize])
            >> 32 as libc::c_int - 1 as libc::c_int;
    qq[0 as libc::c_int
        as usize] = (qq[0 as libc::c_int as usize])
        .wrapping_add(
            (qq[2 as libc::c_int as usize] ^ qq[3 as libc::c_int as usize]
                ^ qq[4 as libc::c_int as usize])
                .wrapping_add(block[(64 as libc::c_int & 15 as libc::c_int) as usize])
                .wrapping_add(0xca62c1d6 as libc::c_uint)
                .wrapping_add(
                    qq[1 as libc::c_int as usize] << 5 as libc::c_int
                        | qq[1 as libc::c_int as usize]
                            >> 32 as libc::c_int - 5 as libc::c_int,
                ),
        );
    qq[2 as libc::c_int
        as usize] = qq[2 as libc::c_int as usize] << 32 as libc::c_int - 2 as libc::c_int
        | qq[2 as libc::c_int as usize] >> 2 as libc::c_int;
    block[(65 as libc::c_int & 15 as libc::c_int)
        as usize] = (block[(65 as libc::c_int + 13 as libc::c_int & 15 as libc::c_int)
        as usize]
        ^ block[(65 as libc::c_int + 8 as libc::c_int & 15 as libc::c_int) as usize]
        ^ block[(65 as libc::c_int + 2 as libc::c_int & 15 as libc::c_int) as usize]
        ^ block[(65 as libc::c_int & 15 as libc::c_int) as usize]) << 1 as libc::c_int
        | (block[(65 as libc::c_int + 13 as libc::c_int & 15 as libc::c_int) as usize]
            ^ block[(65 as libc::c_int + 8 as libc::c_int & 15 as libc::c_int) as usize]
            ^ block[(65 as libc::c_int + 2 as libc::c_int & 15 as libc::c_int) as usize]
            ^ block[(65 as libc::c_int & 15 as libc::c_int) as usize])
            >> 32 as libc::c_int - 1 as libc::c_int;
    qq[4 as libc::c_int
        as usize] = (qq[4 as libc::c_int as usize])
        .wrapping_add(
            (qq[1 as libc::c_int as usize] ^ qq[2 as libc::c_int as usize]
                ^ qq[3 as libc::c_int as usize])
                .wrapping_add(block[(65 as libc::c_int & 15 as libc::c_int) as usize])
                .wrapping_add(0xca62c1d6 as libc::c_uint)
                .wrapping_add(
                    qq[0 as libc::c_int as usize] << 5 as libc::c_int
                        | qq[0 as libc::c_int as usize]
                            >> 32 as libc::c_int - 5 as libc::c_int,
                ),
        );
    qq[1 as libc::c_int
        as usize] = qq[1 as libc::c_int as usize] << 32 as libc::c_int - 2 as libc::c_int
        | qq[1 as libc::c_int as usize] >> 2 as libc::c_int;
    block[(66 as libc::c_int & 15 as libc::c_int)
        as usize] = (block[(66 as libc::c_int + 13 as libc::c_int & 15 as libc::c_int)
        as usize]
        ^ block[(66 as libc::c_int + 8 as libc::c_int & 15 as libc::c_int) as usize]
        ^ block[(66 as libc::c_int + 2 as libc::c_int & 15 as libc::c_int) as usize]
        ^ block[(66 as libc::c_int & 15 as libc::c_int) as usize]) << 1 as libc::c_int
        | (block[(66 as libc::c_int + 13 as libc::c_int & 15 as libc::c_int) as usize]
            ^ block[(66 as libc::c_int + 8 as libc::c_int & 15 as libc::c_int) as usize]
            ^ block[(66 as libc::c_int + 2 as libc::c_int & 15 as libc::c_int) as usize]
            ^ block[(66 as libc::c_int & 15 as libc::c_int) as usize])
            >> 32 as libc::c_int - 1 as libc::c_int;
    qq[3 as libc::c_int
        as usize] = (qq[3 as libc::c_int as usize])
        .wrapping_add(
            (qq[0 as libc::c_int as usize] ^ qq[1 as libc::c_int as usize]
                ^ qq[2 as libc::c_int as usize])
                .wrapping_add(block[(66 as libc::c_int & 15 as libc::c_int) as usize])
                .wrapping_add(0xca62c1d6 as libc::c_uint)
                .wrapping_add(
                    qq[4 as libc::c_int as usize] << 5 as libc::c_int
                        | qq[4 as libc::c_int as usize]
                            >> 32 as libc::c_int - 5 as libc::c_int,
                ),
        );
    qq[0 as libc::c_int
        as usize] = qq[0 as libc::c_int as usize] << 32 as libc::c_int - 2 as libc::c_int
        | qq[0 as libc::c_int as usize] >> 2 as libc::c_int;
    block[(67 as libc::c_int & 15 as libc::c_int)
        as usize] = (block[(67 as libc::c_int + 13 as libc::c_int & 15 as libc::c_int)
        as usize]
        ^ block[(67 as libc::c_int + 8 as libc::c_int & 15 as libc::c_int) as usize]
        ^ block[(67 as libc::c_int + 2 as libc::c_int & 15 as libc::c_int) as usize]
        ^ block[(67 as libc::c_int & 15 as libc::c_int) as usize]) << 1 as libc::c_int
        | (block[(67 as libc::c_int + 13 as libc::c_int & 15 as libc::c_int) as usize]
            ^ block[(67 as libc::c_int + 8 as libc::c_int & 15 as libc::c_int) as usize]
            ^ block[(67 as libc::c_int + 2 as libc::c_int & 15 as libc::c_int) as usize]
            ^ block[(67 as libc::c_int & 15 as libc::c_int) as usize])
            >> 32 as libc::c_int - 1 as libc::c_int;
    qq[2 as libc::c_int
        as usize] = (qq[2 as libc::c_int as usize])
        .wrapping_add(
            (qq[4 as libc::c_int as usize] ^ qq[0 as libc::c_int as usize]
                ^ qq[1 as libc::c_int as usize])
                .wrapping_add(block[(67 as libc::c_int & 15 as libc::c_int) as usize])
                .wrapping_add(0xca62c1d6 as libc::c_uint)
                .wrapping_add(
                    qq[3 as libc::c_int as usize] << 5 as libc::c_int
                        | qq[3 as libc::c_int as usize]
                            >> 32 as libc::c_int - 5 as libc::c_int,
                ),
        );
    qq[4 as libc::c_int
        as usize] = qq[4 as libc::c_int as usize] << 32 as libc::c_int - 2 as libc::c_int
        | qq[4 as libc::c_int as usize] >> 2 as libc::c_int;
    block[(68 as libc::c_int & 15 as libc::c_int)
        as usize] = (block[(68 as libc::c_int + 13 as libc::c_int & 15 as libc::c_int)
        as usize]
        ^ block[(68 as libc::c_int + 8 as libc::c_int & 15 as libc::c_int) as usize]
        ^ block[(68 as libc::c_int + 2 as libc::c_int & 15 as libc::c_int) as usize]
        ^ block[(68 as libc::c_int & 15 as libc::c_int) as usize]) << 1 as libc::c_int
        | (block[(68 as libc::c_int + 13 as libc::c_int & 15 as libc::c_int) as usize]
            ^ block[(68 as libc::c_int + 8 as libc::c_int & 15 as libc::c_int) as usize]
            ^ block[(68 as libc::c_int + 2 as libc::c_int & 15 as libc::c_int) as usize]
            ^ block[(68 as libc::c_int & 15 as libc::c_int) as usize])
            >> 32 as libc::c_int - 1 as libc::c_int;
    qq[1 as libc::c_int
        as usize] = (qq[1 as libc::c_int as usize])
        .wrapping_add(
            (qq[3 as libc::c_int as usize] ^ qq[4 as libc::c_int as usize]
                ^ qq[0 as libc::c_int as usize])
                .wrapping_add(block[(68 as libc::c_int & 15 as libc::c_int) as usize])
                .wrapping_add(0xca62c1d6 as libc::c_uint)
                .wrapping_add(
                    qq[2 as libc::c_int as usize] << 5 as libc::c_int
                        | qq[2 as libc::c_int as usize]
                            >> 32 as libc::c_int - 5 as libc::c_int,
                ),
        );
    qq[3 as libc::c_int
        as usize] = qq[3 as libc::c_int as usize] << 32 as libc::c_int - 2 as libc::c_int
        | qq[3 as libc::c_int as usize] >> 2 as libc::c_int;
    block[(69 as libc::c_int & 15 as libc::c_int)
        as usize] = (block[(69 as libc::c_int + 13 as libc::c_int & 15 as libc::c_int)
        as usize]
        ^ block[(69 as libc::c_int + 8 as libc::c_int & 15 as libc::c_int) as usize]
        ^ block[(69 as libc::c_int + 2 as libc::c_int & 15 as libc::c_int) as usize]
        ^ block[(69 as libc::c_int & 15 as libc::c_int) as usize]) << 1 as libc::c_int
        | (block[(69 as libc::c_int + 13 as libc::c_int & 15 as libc::c_int) as usize]
            ^ block[(69 as libc::c_int + 8 as libc::c_int & 15 as libc::c_int) as usize]
            ^ block[(69 as libc::c_int + 2 as libc::c_int & 15 as libc::c_int) as usize]
            ^ block[(69 as libc::c_int & 15 as libc::c_int) as usize])
            >> 32 as libc::c_int - 1 as libc::c_int;
    qq[0 as libc::c_int
        as usize] = (qq[0 as libc::c_int as usize])
        .wrapping_add(
            (qq[2 as libc::c_int as usize] ^ qq[3 as libc::c_int as usize]
                ^ qq[4 as libc::c_int as usize])
                .wrapping_add(block[(69 as libc::c_int & 15 as libc::c_int) as usize])
                .wrapping_add(0xca62c1d6 as libc::c_uint)
                .wrapping_add(
                    qq[1 as libc::c_int as usize] << 5 as libc::c_int
                        | qq[1 as libc::c_int as usize]
                            >> 32 as libc::c_int - 5 as libc::c_int,
                ),
        );
    qq[2 as libc::c_int
        as usize] = qq[2 as libc::c_int as usize] << 32 as libc::c_int - 2 as libc::c_int
        | qq[2 as libc::c_int as usize] >> 2 as libc::c_int;
    block[(70 as libc::c_int & 15 as libc::c_int)
        as usize] = (block[(70 as libc::c_int + 13 as libc::c_int & 15 as libc::c_int)
        as usize]
        ^ block[(70 as libc::c_int + 8 as libc::c_int & 15 as libc::c_int) as usize]
        ^ block[(70 as libc::c_int + 2 as libc::c_int & 15 as libc::c_int) as usize]
        ^ block[(70 as libc::c_int & 15 as libc::c_int) as usize]) << 1 as libc::c_int
        | (block[(70 as libc::c_int + 13 as libc::c_int & 15 as libc::c_int) as usize]
            ^ block[(70 as libc::c_int + 8 as libc::c_int & 15 as libc::c_int) as usize]
            ^ block[(70 as libc::c_int + 2 as libc::c_int & 15 as libc::c_int) as usize]
            ^ block[(70 as libc::c_int & 15 as libc::c_int) as usize])
            >> 32 as libc::c_int - 1 as libc::c_int;
    qq[4 as libc::c_int
        as usize] = (qq[4 as libc::c_int as usize])
        .wrapping_add(
            (qq[1 as libc::c_int as usize] ^ qq[2 as libc::c_int as usize]
                ^ qq[3 as libc::c_int as usize])
                .wrapping_add(block[(70 as libc::c_int & 15 as libc::c_int) as usize])
                .wrapping_add(0xca62c1d6 as libc::c_uint)
                .wrapping_add(
                    qq[0 as libc::c_int as usize] << 5 as libc::c_int
                        | qq[0 as libc::c_int as usize]
                            >> 32 as libc::c_int - 5 as libc::c_int,
                ),
        );
    qq[1 as libc::c_int
        as usize] = qq[1 as libc::c_int as usize] << 32 as libc::c_int - 2 as libc::c_int
        | qq[1 as libc::c_int as usize] >> 2 as libc::c_int;
    block[(71 as libc::c_int & 15 as libc::c_int)
        as usize] = (block[(71 as libc::c_int + 13 as libc::c_int & 15 as libc::c_int)
        as usize]
        ^ block[(71 as libc::c_int + 8 as libc::c_int & 15 as libc::c_int) as usize]
        ^ block[(71 as libc::c_int + 2 as libc::c_int & 15 as libc::c_int) as usize]
        ^ block[(71 as libc::c_int & 15 as libc::c_int) as usize]) << 1 as libc::c_int
        | (block[(71 as libc::c_int + 13 as libc::c_int & 15 as libc::c_int) as usize]
            ^ block[(71 as libc::c_int + 8 as libc::c_int & 15 as libc::c_int) as usize]
            ^ block[(71 as libc::c_int + 2 as libc::c_int & 15 as libc::c_int) as usize]
            ^ block[(71 as libc::c_int & 15 as libc::c_int) as usize])
            >> 32 as libc::c_int - 1 as libc::c_int;
    qq[3 as libc::c_int
        as usize] = (qq[3 as libc::c_int as usize])
        .wrapping_add(
            (qq[0 as libc::c_int as usize] ^ qq[1 as libc::c_int as usize]
                ^ qq[2 as libc::c_int as usize])
                .wrapping_add(block[(71 as libc::c_int & 15 as libc::c_int) as usize])
                .wrapping_add(0xca62c1d6 as libc::c_uint)
                .wrapping_add(
                    qq[4 as libc::c_int as usize] << 5 as libc::c_int
                        | qq[4 as libc::c_int as usize]
                            >> 32 as libc::c_int - 5 as libc::c_int,
                ),
        );
    qq[0 as libc::c_int
        as usize] = qq[0 as libc::c_int as usize] << 32 as libc::c_int - 2 as libc::c_int
        | qq[0 as libc::c_int as usize] >> 2 as libc::c_int;
    block[(72 as libc::c_int & 15 as libc::c_int)
        as usize] = (block[(72 as libc::c_int + 13 as libc::c_int & 15 as libc::c_int)
        as usize]
        ^ block[(72 as libc::c_int + 8 as libc::c_int & 15 as libc::c_int) as usize]
        ^ block[(72 as libc::c_int + 2 as libc::c_int & 15 as libc::c_int) as usize]
        ^ block[(72 as libc::c_int & 15 as libc::c_int) as usize]) << 1 as libc::c_int
        | (block[(72 as libc::c_int + 13 as libc::c_int & 15 as libc::c_int) as usize]
            ^ block[(72 as libc::c_int + 8 as libc::c_int & 15 as libc::c_int) as usize]
            ^ block[(72 as libc::c_int + 2 as libc::c_int & 15 as libc::c_int) as usize]
            ^ block[(72 as libc::c_int & 15 as libc::c_int) as usize])
            >> 32 as libc::c_int - 1 as libc::c_int;
    qq[2 as libc::c_int
        as usize] = (qq[2 as libc::c_int as usize])
        .wrapping_add(
            (qq[4 as libc::c_int as usize] ^ qq[0 as libc::c_int as usize]
                ^ qq[1 as libc::c_int as usize])
                .wrapping_add(block[(72 as libc::c_int & 15 as libc::c_int) as usize])
                .wrapping_add(0xca62c1d6 as libc::c_uint)
                .wrapping_add(
                    qq[3 as libc::c_int as usize] << 5 as libc::c_int
                        | qq[3 as libc::c_int as usize]
                            >> 32 as libc::c_int - 5 as libc::c_int,
                ),
        );
    qq[4 as libc::c_int
        as usize] = qq[4 as libc::c_int as usize] << 32 as libc::c_int - 2 as libc::c_int
        | qq[4 as libc::c_int as usize] >> 2 as libc::c_int;
    block[(73 as libc::c_int & 15 as libc::c_int)
        as usize] = (block[(73 as libc::c_int + 13 as libc::c_int & 15 as libc::c_int)
        as usize]
        ^ block[(73 as libc::c_int + 8 as libc::c_int & 15 as libc::c_int) as usize]
        ^ block[(73 as libc::c_int + 2 as libc::c_int & 15 as libc::c_int) as usize]
        ^ block[(73 as libc::c_int & 15 as libc::c_int) as usize]) << 1 as libc::c_int
        | (block[(73 as libc::c_int + 13 as libc::c_int & 15 as libc::c_int) as usize]
            ^ block[(73 as libc::c_int + 8 as libc::c_int & 15 as libc::c_int) as usize]
            ^ block[(73 as libc::c_int + 2 as libc::c_int & 15 as libc::c_int) as usize]
            ^ block[(73 as libc::c_int & 15 as libc::c_int) as usize])
            >> 32 as libc::c_int - 1 as libc::c_int;
    qq[1 as libc::c_int
        as usize] = (qq[1 as libc::c_int as usize])
        .wrapping_add(
            (qq[3 as libc::c_int as usize] ^ qq[4 as libc::c_int as usize]
                ^ qq[0 as libc::c_int as usize])
                .wrapping_add(block[(73 as libc::c_int & 15 as libc::c_int) as usize])
                .wrapping_add(0xca62c1d6 as libc::c_uint)
                .wrapping_add(
                    qq[2 as libc::c_int as usize] << 5 as libc::c_int
                        | qq[2 as libc::c_int as usize]
                            >> 32 as libc::c_int - 5 as libc::c_int,
                ),
        );
    qq[3 as libc::c_int
        as usize] = qq[3 as libc::c_int as usize] << 32 as libc::c_int - 2 as libc::c_int
        | qq[3 as libc::c_int as usize] >> 2 as libc::c_int;
    block[(74 as libc::c_int & 15 as libc::c_int)
        as usize] = (block[(74 as libc::c_int + 13 as libc::c_int & 15 as libc::c_int)
        as usize]
        ^ block[(74 as libc::c_int + 8 as libc::c_int & 15 as libc::c_int) as usize]
        ^ block[(74 as libc::c_int + 2 as libc::c_int & 15 as libc::c_int) as usize]
        ^ block[(74 as libc::c_int & 15 as libc::c_int) as usize]) << 1 as libc::c_int
        | (block[(74 as libc::c_int + 13 as libc::c_int & 15 as libc::c_int) as usize]
            ^ block[(74 as libc::c_int + 8 as libc::c_int & 15 as libc::c_int) as usize]
            ^ block[(74 as libc::c_int + 2 as libc::c_int & 15 as libc::c_int) as usize]
            ^ block[(74 as libc::c_int & 15 as libc::c_int) as usize])
            >> 32 as libc::c_int - 1 as libc::c_int;
    qq[0 as libc::c_int
        as usize] = (qq[0 as libc::c_int as usize])
        .wrapping_add(
            (qq[2 as libc::c_int as usize] ^ qq[3 as libc::c_int as usize]
                ^ qq[4 as libc::c_int as usize])
                .wrapping_add(block[(74 as libc::c_int & 15 as libc::c_int) as usize])
                .wrapping_add(0xca62c1d6 as libc::c_uint)
                .wrapping_add(
                    qq[1 as libc::c_int as usize] << 5 as libc::c_int
                        | qq[1 as libc::c_int as usize]
                            >> 32 as libc::c_int - 5 as libc::c_int,
                ),
        );
    qq[2 as libc::c_int
        as usize] = qq[2 as libc::c_int as usize] << 32 as libc::c_int - 2 as libc::c_int
        | qq[2 as libc::c_int as usize] >> 2 as libc::c_int;
    block[(75 as libc::c_int & 15 as libc::c_int)
        as usize] = (block[(75 as libc::c_int + 13 as libc::c_int & 15 as libc::c_int)
        as usize]
        ^ block[(75 as libc::c_int + 8 as libc::c_int & 15 as libc::c_int) as usize]
        ^ block[(75 as libc::c_int + 2 as libc::c_int & 15 as libc::c_int) as usize]
        ^ block[(75 as libc::c_int & 15 as libc::c_int) as usize]) << 1 as libc::c_int
        | (block[(75 as libc::c_int + 13 as libc::c_int & 15 as libc::c_int) as usize]
            ^ block[(75 as libc::c_int + 8 as libc::c_int & 15 as libc::c_int) as usize]
            ^ block[(75 as libc::c_int + 2 as libc::c_int & 15 as libc::c_int) as usize]
            ^ block[(75 as libc::c_int & 15 as libc::c_int) as usize])
            >> 32 as libc::c_int - 1 as libc::c_int;
    qq[4 as libc::c_int
        as usize] = (qq[4 as libc::c_int as usize])
        .wrapping_add(
            (qq[1 as libc::c_int as usize] ^ qq[2 as libc::c_int as usize]
                ^ qq[3 as libc::c_int as usize])
                .wrapping_add(block[(75 as libc::c_int & 15 as libc::c_int) as usize])
                .wrapping_add(0xca62c1d6 as libc::c_uint)
                .wrapping_add(
                    qq[0 as libc::c_int as usize] << 5 as libc::c_int
                        | qq[0 as libc::c_int as usize]
                            >> 32 as libc::c_int - 5 as libc::c_int,
                ),
        );
    qq[1 as libc::c_int
        as usize] = qq[1 as libc::c_int as usize] << 32 as libc::c_int - 2 as libc::c_int
        | qq[1 as libc::c_int as usize] >> 2 as libc::c_int;
    block[(76 as libc::c_int & 15 as libc::c_int)
        as usize] = (block[(76 as libc::c_int + 13 as libc::c_int & 15 as libc::c_int)
        as usize]
        ^ block[(76 as libc::c_int + 8 as libc::c_int & 15 as libc::c_int) as usize]
        ^ block[(76 as libc::c_int + 2 as libc::c_int & 15 as libc::c_int) as usize]
        ^ block[(76 as libc::c_int & 15 as libc::c_int) as usize]) << 1 as libc::c_int
        | (block[(76 as libc::c_int + 13 as libc::c_int & 15 as libc::c_int) as usize]
            ^ block[(76 as libc::c_int + 8 as libc::c_int & 15 as libc::c_int) as usize]
            ^ block[(76 as libc::c_int + 2 as libc::c_int & 15 as libc::c_int) as usize]
            ^ block[(76 as libc::c_int & 15 as libc::c_int) as usize])
            >> 32 as libc::c_int - 1 as libc::c_int;
    qq[3 as libc::c_int
        as usize] = (qq[3 as libc::c_int as usize])
        .wrapping_add(
            (qq[0 as libc::c_int as usize] ^ qq[1 as libc::c_int as usize]
                ^ qq[2 as libc::c_int as usize])
                .wrapping_add(block[(76 as libc::c_int & 15 as libc::c_int) as usize])
                .wrapping_add(0xca62c1d6 as libc::c_uint)
                .wrapping_add(
                    qq[4 as libc::c_int as usize] << 5 as libc::c_int
                        | qq[4 as libc::c_int as usize]
                            >> 32 as libc::c_int - 5 as libc::c_int,
                ),
        );
    qq[0 as libc::c_int
        as usize] = qq[0 as libc::c_int as usize] << 32 as libc::c_int - 2 as libc::c_int
        | qq[0 as libc::c_int as usize] >> 2 as libc::c_int;
    block[(77 as libc::c_int & 15 as libc::c_int)
        as usize] = (block[(77 as libc::c_int + 13 as libc::c_int & 15 as libc::c_int)
        as usize]
        ^ block[(77 as libc::c_int + 8 as libc::c_int & 15 as libc::c_int) as usize]
        ^ block[(77 as libc::c_int + 2 as libc::c_int & 15 as libc::c_int) as usize]
        ^ block[(77 as libc::c_int & 15 as libc::c_int) as usize]) << 1 as libc::c_int
        | (block[(77 as libc::c_int + 13 as libc::c_int & 15 as libc::c_int) as usize]
            ^ block[(77 as libc::c_int + 8 as libc::c_int & 15 as libc::c_int) as usize]
            ^ block[(77 as libc::c_int + 2 as libc::c_int & 15 as libc::c_int) as usize]
            ^ block[(77 as libc::c_int & 15 as libc::c_int) as usize])
            >> 32 as libc::c_int - 1 as libc::c_int;
    qq[2 as libc::c_int
        as usize] = (qq[2 as libc::c_int as usize])
        .wrapping_add(
            (qq[4 as libc::c_int as usize] ^ qq[0 as libc::c_int as usize]
                ^ qq[1 as libc::c_int as usize])
                .wrapping_add(block[(77 as libc::c_int & 15 as libc::c_int) as usize])
                .wrapping_add(0xca62c1d6 as libc::c_uint)
                .wrapping_add(
                    qq[3 as libc::c_int as usize] << 5 as libc::c_int
                        | qq[3 as libc::c_int as usize]
                            >> 32 as libc::c_int - 5 as libc::c_int,
                ),
        );
    qq[4 as libc::c_int
        as usize] = qq[4 as libc::c_int as usize] << 32 as libc::c_int - 2 as libc::c_int
        | qq[4 as libc::c_int as usize] >> 2 as libc::c_int;
    block[(78 as libc::c_int & 15 as libc::c_int)
        as usize] = (block[(78 as libc::c_int + 13 as libc::c_int & 15 as libc::c_int)
        as usize]
        ^ block[(78 as libc::c_int + 8 as libc::c_int & 15 as libc::c_int) as usize]
        ^ block[(78 as libc::c_int + 2 as libc::c_int & 15 as libc::c_int) as usize]
        ^ block[(78 as libc::c_int & 15 as libc::c_int) as usize]) << 1 as libc::c_int
        | (block[(78 as libc::c_int + 13 as libc::c_int & 15 as libc::c_int) as usize]
            ^ block[(78 as libc::c_int + 8 as libc::c_int & 15 as libc::c_int) as usize]
            ^ block[(78 as libc::c_int + 2 as libc::c_int & 15 as libc::c_int) as usize]
            ^ block[(78 as libc::c_int & 15 as libc::c_int) as usize])
            >> 32 as libc::c_int - 1 as libc::c_int;
    qq[1 as libc::c_int
        as usize] = (qq[1 as libc::c_int as usize])
        .wrapping_add(
            (qq[3 as libc::c_int as usize] ^ qq[4 as libc::c_int as usize]
                ^ qq[0 as libc::c_int as usize])
                .wrapping_add(block[(78 as libc::c_int & 15 as libc::c_int) as usize])
                .wrapping_add(0xca62c1d6 as libc::c_uint)
                .wrapping_add(
                    qq[2 as libc::c_int as usize] << 5 as libc::c_int
                        | qq[2 as libc::c_int as usize]
                            >> 32 as libc::c_int - 5 as libc::c_int,
                ),
        );
    qq[3 as libc::c_int
        as usize] = qq[3 as libc::c_int as usize] << 32 as libc::c_int - 2 as libc::c_int
        | qq[3 as libc::c_int as usize] >> 2 as libc::c_int;
    block[(79 as libc::c_int & 15 as libc::c_int)
        as usize] = (block[(79 as libc::c_int + 13 as libc::c_int & 15 as libc::c_int)
        as usize]
        ^ block[(79 as libc::c_int + 8 as libc::c_int & 15 as libc::c_int) as usize]
        ^ block[(79 as libc::c_int + 2 as libc::c_int & 15 as libc::c_int) as usize]
        ^ block[(79 as libc::c_int & 15 as libc::c_int) as usize]) << 1 as libc::c_int
        | (block[(79 as libc::c_int + 13 as libc::c_int & 15 as libc::c_int) as usize]
            ^ block[(79 as libc::c_int + 8 as libc::c_int & 15 as libc::c_int) as usize]
            ^ block[(79 as libc::c_int + 2 as libc::c_int & 15 as libc::c_int) as usize]
            ^ block[(79 as libc::c_int & 15 as libc::c_int) as usize])
            >> 32 as libc::c_int - 1 as libc::c_int;
    qq[0 as libc::c_int
        as usize] = (qq[0 as libc::c_int as usize])
        .wrapping_add(
            (qq[2 as libc::c_int as usize] ^ qq[3 as libc::c_int as usize]
                ^ qq[4 as libc::c_int as usize])
                .wrapping_add(block[(79 as libc::c_int & 15 as libc::c_int) as usize])
                .wrapping_add(0xca62c1d6 as libc::c_uint)
                .wrapping_add(
                    qq[1 as libc::c_int as usize] << 5 as libc::c_int
                        | qq[1 as libc::c_int as usize]
                            >> 32 as libc::c_int - 5 as libc::c_int,
                ),
        );
    qq[2 as libc::c_int
        as usize] = qq[2 as libc::c_int as usize] << 32 as libc::c_int - 2 as libc::c_int
        | qq[2 as libc::c_int as usize] >> 2 as libc::c_int;
    let ref mut fresh11 = *state.offset(0 as libc::c_int as isize);
    *fresh11 = (*fresh11).wrapping_add(qq[0 as libc::c_int as usize]);
    let ref mut fresh12 = *state.offset(1 as libc::c_int as isize);
    *fresh12 = (*fresh12).wrapping_add(qq[1 as libc::c_int as usize]);
    let ref mut fresh13 = *state.offset(2 as libc::c_int as isize);
    *fresh13 = (*fresh13).wrapping_add(qq[2 as libc::c_int as usize]);
    let ref mut fresh14 = *state.offset(3 as libc::c_int as isize);
    *fresh14 = (*fresh14).wrapping_add(qq[3 as libc::c_int as usize]);
    let ref mut fresh15 = *state.offset(4 as libc::c_int as isize);
    *fresh15 = (*fresh15).wrapping_add(qq[4 as libc::c_int as usize]);
}
unsafe extern "C" fn SHA1Init(mut context: *mut SHA1Context) {
    (*context)
        .state[0 as libc::c_int as usize] = 0x67452301 as libc::c_int as libc::c_uint;
    (*context).state[1 as libc::c_int as usize] = 0xefcdab89 as libc::c_uint;
    (*context).state[2 as libc::c_int as usize] = 0x98badcfe as libc::c_uint;
    (*context)
        .state[3 as libc::c_int as usize] = 0x10325476 as libc::c_int as libc::c_uint;
    (*context).state[4 as libc::c_int as usize] = 0xc3d2e1f0 as libc::c_uint;
    let ref mut fresh16 = (*context).count[1 as libc::c_int as usize];
    *fresh16 = 0 as libc::c_int as libc::c_uint;
    (*context).count[0 as libc::c_int as usize] = *fresh16;
}
unsafe extern "C" fn SHA1Update(
    mut context: *mut SHA1Context,
    mut data: *const libc::c_uchar,
    mut len: libc::c_uint,
) {
    let mut i: libc::c_uint = 0;
    let mut j: libc::c_uint = 0;
    j = (*context).count[0 as libc::c_int as usize];
    let ref mut fresh17 = (*context).count[0 as libc::c_int as usize];
    *fresh17 = (*fresh17).wrapping_add(len << 3 as libc::c_int);
    if *fresh17 < j {
        let ref mut fresh18 = (*context).count[1 as libc::c_int as usize];
        *fresh18 = (*fresh18)
            .wrapping_add(
                (len >> 29 as libc::c_int).wrapping_add(1 as libc::c_int as libc::c_uint),
            );
    }
    j = j >> 3 as libc::c_int & 63 as libc::c_int as libc::c_uint;
    if j.wrapping_add(len) > 63 as libc::c_int as libc::c_uint {
        i = (64 as libc::c_int as libc::c_uint).wrapping_sub(j);
        memcpy(
            &mut *((*context).buffer).as_mut_ptr().offset(j as isize)
                as *mut libc::c_uchar as *mut libc::c_void,
            data as *const libc::c_void,
            i as libc::c_ulong,
        );
        SHA1Transform(
            ((*context).state).as_mut_ptr(),
            ((*context).buffer).as_mut_ptr() as *const libc::c_uchar,
        );
        while i.wrapping_add(63 as libc::c_int as libc::c_uint) < len {
            SHA1Transform(((*context).state).as_mut_ptr(), &*data.offset(i as isize));
            i = i.wrapping_add(64 as libc::c_int as libc::c_uint);
        }
        j = 0 as libc::c_int as libc::c_uint;
    } else {
        i = 0 as libc::c_int as libc::c_uint;
    }
    memcpy(
        &mut *((*context).buffer).as_mut_ptr().offset(j as isize) as *mut libc::c_uchar
            as *mut libc::c_void,
        &*data.offset(i as isize) as *const libc::c_uchar as *const libc::c_void,
        len.wrapping_sub(i) as libc::c_ulong,
    );
}
unsafe extern "C" fn SHA1Final(
    mut digest: *mut libc::c_uchar,
    mut context: *mut SHA1Context,
) {
    let mut i: libc::c_uint = 0;
    let mut finalcount: [libc::c_uchar; 8] = [0; 8];
    i = 0 as libc::c_int as libc::c_uint;
    while i < 8 as libc::c_int as libc::c_uint {
        finalcount[i
            as usize] = ((*context)
            .count[(if i >= 4 as libc::c_int as libc::c_uint {
            0 as libc::c_int
        } else {
            1 as libc::c_int
        }) as usize]
            >> (3 as libc::c_int as libc::c_uint)
                .wrapping_sub(i & 3 as libc::c_int as libc::c_uint)
                .wrapping_mul(8 as libc::c_int as libc::c_uint)
            & 255 as libc::c_int as libc::c_uint) as libc::c_uchar;
        i = i.wrapping_add(1);
    }
    SHA1Update(
        context,
        b"\x80\0" as *const u8 as *const libc::c_char as *const libc::c_uchar,
        1 as libc::c_int as libc::c_uint,
    );
    while (*context).count[0 as libc::c_int as usize]
        & 504 as libc::c_int as libc::c_uint != 448 as libc::c_int as libc::c_uint
    {
        SHA1Update(
            context,
            b"\0\0" as *const u8 as *const libc::c_char as *const libc::c_uchar,
            1 as libc::c_int as libc::c_uint,
        );
    }
    SHA1Update(context, finalcount.as_mut_ptr(), 8 as libc::c_int as libc::c_uint);
    if !digest.is_null() {
        i = 0 as libc::c_int as libc::c_uint;
        while i < 20 as libc::c_int as libc::c_uint {
            *digest
                .offset(
                    i as isize,
                ) = ((*context).state[(i >> 2 as libc::c_int) as usize]
                >> (3 as libc::c_int as libc::c_uint)
                    .wrapping_sub(i & 3 as libc::c_int as libc::c_uint)
                    .wrapping_mul(8 as libc::c_int as libc::c_uint)
                & 255 as libc::c_int as libc::c_uint) as libc::c_uchar;
            i = i.wrapping_add(1);
        }
    }
}
unsafe extern "C" fn sha1sum_file(
    mut zFilename: *const libc::c_char,
    mut pCksum: *mut libc::c_char,
) -> libc::c_int {
    let mut in_0: *mut FILE = 0 as *mut FILE;
    let mut ctx: SHA1Context = SHA1Context {
        state: [0; 5],
        count: [0; 2],
        buffer: [0; 64],
    };
    let mut zResult: [libc::c_uchar; 20] = [0; 20];
    let mut zBuf: [libc::c_char; 10240] = [0; 10240];
    in_0 = fopen(zFilename, b"rb\0" as *const u8 as *const libc::c_char);
    if in_0.is_null() {
        return 1 as libc::c_int;
    }
    SHA1Init(&mut ctx);
    loop {
        let mut n: libc::c_int = fread(
            zBuf.as_mut_ptr() as *mut libc::c_void,
            1 as libc::c_int as libc::c_ulong,
            ::std::mem::size_of::<[libc::c_char; 10240]>() as libc::c_ulong,
            in_0,
        ) as libc::c_int;
        if n <= 0 as libc::c_int {
            break;
        }
        SHA1Update(&mut ctx, zBuf.as_mut_ptr() as *mut libc::c_uchar, n as libc::c_uint);
    }
    fclose(in_0);
    SHA1Final(zResult.as_mut_ptr(), &mut ctx);
    DigestToBase16(zResult.as_mut_ptr(), pCksum, 20 as libc::c_int);
    return 0 as libc::c_int;
}
unsafe extern "C" fn usage(mut argv0: *const libc::c_char) {
    fprintf(
        stderr,
        b"Usage: %s manifest\nOptions:\n   -v  Diagnostic output\n\0" as *const u8
            as *const libc::c_char,
        argv0,
    );
    exit(1 as libc::c_int);
}
unsafe extern "C" fn nextToken(mut z: *mut libc::c_char) -> *mut libc::c_char {
    while *z as libc::c_int != 0
        && *(*__ctype_b_loc()).offset(*z as libc::c_int as isize) as libc::c_int
            & _ISspace as libc::c_int as libc::c_ushort as libc::c_int == 0
    {
        z = z.offset(1);
    }
    if *z as libc::c_int == 0 as libc::c_int {
        return z;
    }
    *z = 0 as libc::c_int as libc::c_char;
    return &mut *z.offset(1 as libc::c_int as isize) as *mut libc::c_char;
}
unsafe fn main_0(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut zManifest: *const libc::c_char = 0 as *const libc::c_char;
    let mut i: libc::c_int = 0;
    let mut bVerbose: libc::c_int = 0 as libc::c_int;
    let mut in_0: *mut FILE = 0 as *mut FILE;
    let mut allValid: libc::c_int = 1 as libc::c_int;
    let mut rc: libc::c_int = 0;
    let mut ctx: SHA3Context = SHA3Context {
        u: C2RustUnnamed_0 { s: [0; 25] },
        nRate: 0,
        nLoaded: 0,
        ixMask: 0,
    };
    let mut zDate: [libc::c_char; 50] = [0; 50];
    let mut zHash: [libc::c_char; 100] = [0; 100];
    let mut zLine: [libc::c_char; 20000] = [0; 20000];
    i = 1 as libc::c_int;
    while i < argc {
        let mut z: *const libc::c_char = *argv.offset(i as isize);
        if *z.offset(0 as libc::c_int as isize) as libc::c_int == '-' as i32 {
            if *z.offset(1 as libc::c_int as isize) as libc::c_int == '-' as i32 {
                z = z.offset(1);
            }
            if strcmp(z, b"-v\0" as *const u8 as *const libc::c_char) == 0 as libc::c_int
            {
                bVerbose = 1 as libc::c_int;
            } else {
                fprintf(
                    stderr,
                    b"unknown option \"%s\"\0" as *const u8 as *const libc::c_char,
                    *argv.offset(i as isize),
                );
                exit(1 as libc::c_int);
            }
        } else if !zManifest.is_null() {
            usage(*argv.offset(0 as libc::c_int as isize));
        } else {
            zManifest = z;
        }
        i += 1;
    }
    if zManifest.is_null() {
        usage(*argv.offset(0 as libc::c_int as isize));
    }
    zDate[0 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    in_0 = fopen(zManifest, b"rb\0" as *const u8 as *const libc::c_char);
    if in_0.is_null() {
        fprintf(
            stderr,
            b"cannot open \"%s\" for reading\n\0" as *const u8 as *const libc::c_char,
            zManifest,
        );
        exit(1 as libc::c_int);
    }
    SHA3Init(&mut ctx, 256 as libc::c_int);
    while !(fgets(
        zLine.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 20000]>() as libc::c_ulong as libc::c_int,
        in_0,
    ))
        .is_null()
    {
        if strncmp(
            zLine.as_mut_ptr(),
            b"# Remove this line\0" as *const u8 as *const libc::c_char,
            18 as libc::c_int as libc::c_ulong,
        ) != 0 as libc::c_int
        {
            SHA3Update(
                &mut ctx,
                zLine.as_mut_ptr() as *mut libc::c_uchar,
                strlen(zLine.as_mut_ptr()) as libc::c_uint,
            );
        }
        if strncmp(
            zLine.as_mut_ptr(),
            b"D 20\0" as *const u8 as *const libc::c_char,
            4 as libc::c_int as libc::c_ulong,
        ) == 0 as libc::c_int
        {
            memcpy(
                zDate.as_mut_ptr() as *mut libc::c_void,
                &mut *zLine.as_mut_ptr().offset(2 as libc::c_int as isize)
                    as *mut libc::c_char as *const libc::c_void,
                10 as libc::c_int as libc::c_ulong,
            );
            zDate[10 as libc::c_int as usize] = ' ' as i32 as libc::c_char;
            memcpy(
                &mut *zDate.as_mut_ptr().offset(11 as libc::c_int as isize)
                    as *mut libc::c_char as *mut libc::c_void,
                &mut *zLine.as_mut_ptr().offset(13 as libc::c_int as isize)
                    as *mut libc::c_char as *const libc::c_void,
                8 as libc::c_int as libc::c_ulong,
            );
            zDate[19 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
        } else if strncmp(
            zLine.as_mut_ptr(),
            b"F \0" as *const u8 as *const libc::c_char,
            2 as libc::c_int as libc::c_ulong,
        ) == 0 as libc::c_int
        {
            let mut zFilename: *mut libc::c_char = &mut *zLine
                .as_mut_ptr()
                .offset(2 as libc::c_int as isize) as *mut libc::c_char;
            let mut zMHash: *mut libc::c_char = nextToken(zFilename);
            nextToken(zMHash);
            if strlen(zMHash) == 40 as libc::c_int as libc::c_ulong {
                rc = sha1sum_file(zFilename, zHash.as_mut_ptr());
            } else {
                rc = sha3sum_file(zFilename, 256 as libc::c_int, zHash.as_mut_ptr());
            }
            if rc != 0 {
                allValid = 0 as libc::c_int;
                if bVerbose != 0 {
                    printf(
                        b"hash failed: %s\n\0" as *const u8 as *const libc::c_char,
                        zFilename,
                    );
                }
            } else if strcmp(zHash.as_mut_ptr(), zMHash) != 0 as libc::c_int {
                allValid = 0 as libc::c_int;
                if bVerbose != 0 {
                    printf(
                        b"wrong hash: %s\n\0" as *const u8 as *const libc::c_char,
                        zFilename,
                    );
                    printf(
                        b"... expected: %s\n\0" as *const u8 as *const libc::c_char,
                        zMHash,
                    );
                    printf(
                        b"... got:      %s\n\0" as *const u8 as *const libc::c_char,
                        zHash.as_mut_ptr(),
                    );
                }
            }
        }
    }
    fclose(in_0);
    DigestToBase16(
        SHA3Final(&mut ctx),
        zHash.as_mut_ptr(),
        256 as libc::c_int / 8 as libc::c_int,
    );
    if allValid == 0 {
        printf(
            b"%s %.60salt1\n\0" as *const u8 as *const libc::c_char,
            zDate.as_mut_ptr(),
            zHash.as_mut_ptr(),
        );
    } else {
        printf(
            b"%s %s\n\0" as *const u8 as *const libc::c_char,
            zDate.as_mut_ptr(),
            zHash.as_mut_ptr(),
        );
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
    args.push(::std::ptr::null_mut());
    unsafe {
        ::std::process::exit(
            main_0(
                (args.len() - 1) as libc::c_int,
                args.as_mut_ptr() as *mut *mut libc::c_char,
            ) as i32,
        )
    }
}
