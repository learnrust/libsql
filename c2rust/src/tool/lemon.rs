use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    static mut stdout: *mut FILE;
    static mut stderr: *mut FILE;
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn vfprintf(
        _: *mut FILE,
        _: *const libc::c_char,
        _: ::std::ffi::VaList,
    ) -> libc::c_int;
    fn fgetc(__stream: *mut FILE) -> libc::c_int;
    fn fputc(__c: libc::c_int, __stream: *mut FILE) -> libc::c_int;
    fn putc(__c: libc::c_int, __stream: *mut FILE) -> libc::c_int;
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
    fn fseek(
        __stream: *mut FILE,
        __off: libc::c_long,
        __whence: libc::c_int,
    ) -> libc::c_int;
    fn ftell(__stream: *mut FILE) -> libc::c_long;
    fn rewind(__stream: *mut FILE);
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
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strrchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strstr(_: *const libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    fn strtod(_: *const libc::c_char, _: *mut *mut libc::c_char) -> libc::c_double;
    fn strtol(
        _: *const libc::c_char,
        _: *mut *mut libc::c_char,
        _: libc::c_int,
    ) -> libc::c_long;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn exit(_: libc::c_int) -> !;
    fn getenv(__name: *const libc::c_char) -> *mut libc::c_char;
    fn qsort(
        __base: *mut libc::c_void,
        __nmemb: size_t,
        __size: size_t,
        __compar: __compar_fn_t,
    );
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    fn access(__name: *const libc::c_char, __type: libc::c_int) -> libc::c_int;
}
pub type __builtin_va_list = [__va_list_tag; 1];
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __va_list_tag {
    pub gp_offset: libc::c_uint,
    pub fp_offset: libc::c_uint,
    pub overflow_arg_area: *mut libc::c_void,
    pub reg_save_area: *mut libc::c_void,
}
pub type size_t = libc::c_ulong;
pub type va_list = __builtin_va_list;
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
pub type __compar_fn_t = Option::<
    unsafe extern "C" fn(*const libc::c_void, *const libc::c_void) -> libc::c_int,
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct rule {
    pub lhs: *mut symbol,
    pub lhsalias: *const libc::c_char,
    pub lhsStart: libc::c_int,
    pub ruleline: libc::c_int,
    pub nrhs: libc::c_int,
    pub rhs: *mut *mut symbol,
    pub rhsalias: *mut *const libc::c_char,
    pub line: libc::c_int,
    pub code: *const libc::c_char,
    pub codePrefix: *const libc::c_char,
    pub codeSuffix: *const libc::c_char,
    pub precsym: *mut symbol,
    pub index: libc::c_int,
    pub iRule: libc::c_int,
    pub noCode: Boolean,
    pub codeEmitted: Boolean,
    pub canReduce: Boolean,
    pub doesReduce: Boolean,
    pub neverReduce: Boolean,
    pub nextlhs: *mut rule,
    pub next: *mut rule,
}
pub type Boolean = libc::c_uint;
pub const LEMON_TRUE: Boolean = 1;
pub const LEMON_FALSE: Boolean = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct symbol {
    pub name: *const libc::c_char,
    pub index: libc::c_int,
    pub type_0: symbol_type,
    pub rule: *mut rule,
    pub fallback: *mut symbol,
    pub prec: libc::c_int,
    pub assoc: e_assoc,
    pub firstset: *mut libc::c_char,
    pub lambda: Boolean,
    pub useCnt: libc::c_int,
    pub destructor: *mut libc::c_char,
    pub destLineno: libc::c_int,
    pub datatype: *mut libc::c_char,
    pub dtnum: libc::c_int,
    pub bContent: libc::c_int,
    pub nsubsym: libc::c_int,
    pub subsym: *mut *mut symbol,
}
pub type e_assoc = libc::c_uint;
pub const UNK: e_assoc = 3;
pub const NONE: e_assoc = 2;
pub const RIGHT: e_assoc = 1;
pub const LEFT: e_assoc = 0;
pub type symbol_type = libc::c_uint;
pub const MULTITERMINAL: symbol_type = 2;
pub const NONTERMINAL: symbol_type = 1;
pub const TERMINAL: symbol_type = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct lemon {
    pub sorted: *mut *mut state,
    pub rule: *mut rule,
    pub startRule: *mut rule,
    pub nstate: libc::c_int,
    pub nxstate: libc::c_int,
    pub nrule: libc::c_int,
    pub nruleWithAction: libc::c_int,
    pub nsymbol: libc::c_int,
    pub nterminal: libc::c_int,
    pub minShiftReduce: libc::c_int,
    pub errAction: libc::c_int,
    pub accAction: libc::c_int,
    pub noAction: libc::c_int,
    pub minReduce: libc::c_int,
    pub maxAction: libc::c_int,
    pub symbols: *mut *mut symbol,
    pub errorcnt: libc::c_int,
    pub errsym: *mut symbol,
    pub wildcard: *mut symbol,
    pub name: *mut libc::c_char,
    pub arg: *mut libc::c_char,
    pub ctx: *mut libc::c_char,
    pub tokentype: *mut libc::c_char,
    pub vartype: *mut libc::c_char,
    pub start: *mut libc::c_char,
    pub stacksize: *mut libc::c_char,
    pub include: *mut libc::c_char,
    pub error: *mut libc::c_char,
    pub overflow: *mut libc::c_char,
    pub failure: *mut libc::c_char,
    pub accept: *mut libc::c_char,
    pub extracode: *mut libc::c_char,
    pub tokendest: *mut libc::c_char,
    pub vardest: *mut libc::c_char,
    pub filename: *mut libc::c_char,
    pub outname: *mut libc::c_char,
    pub tokenprefix: *mut libc::c_char,
    pub nconflict: libc::c_int,
    pub nactiontab: libc::c_int,
    pub nlookaheadtab: libc::c_int,
    pub tablesize: libc::c_int,
    pub basisflag: libc::c_int,
    pub printPreprocessed: libc::c_int,
    pub has_fallback: libc::c_int,
    pub nolinenosflag: libc::c_int,
    pub argv0: *mut libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct state {
    pub bp: *mut config,
    pub cfp: *mut config,
    pub statenum: libc::c_int,
    pub ap: *mut action,
    pub nTknAct: libc::c_int,
    pub nNtAct: libc::c_int,
    pub iTknOfst: libc::c_int,
    pub iNtOfst: libc::c_int,
    pub iDfltReduce: libc::c_int,
    pub pDfltReduce: *mut rule,
    pub autoReduce: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct action {
    pub sp: *mut symbol,
    pub type_0: e_action,
    pub x: C2RustUnnamed_0,
    pub spOpt: *mut symbol,
    pub next: *mut action,
    pub collide: *mut action,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_0 {
    pub stp: *mut state,
    pub rp: *mut rule,
}
pub type e_action = libc::c_uint;
pub const SHIFTREDUCE: e_action = 10;
pub const NOT_USED: e_action = 9;
pub const RD_RESOLVED: e_action = 8;
pub const SH_RESOLVED: e_action = 7;
pub const RRCONFLICT: e_action = 6;
pub const SRCONFLICT: e_action = 5;
pub const SSCONFLICT: e_action = 4;
pub const ERROR: e_action = 3;
pub const REDUCE: e_action = 2;
pub const ACCEPT: e_action = 1;
pub const SHIFT: e_action = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct config {
    pub rp: *mut rule,
    pub dot: libc::c_int,
    pub fws: *mut libc::c_char,
    pub fplp: *mut plink,
    pub bplp: *mut plink,
    pub stp: *mut state,
    pub status: cfgstatus,
    pub next: *mut config,
    pub bp: *mut config,
}
pub type cfgstatus = libc::c_uint;
pub const INCOMPLETE: cfgstatus = 1;
pub const COMPLETE: cfgstatus = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct plink {
    pub cfp: *mut config,
    pub next: *mut plink,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct s_x4node {
    pub data: *mut config,
    pub next: *mut s_x4node,
    pub from: *mut *mut s_x4node,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct s_x4 {
    pub size: libc::c_int,
    pub count: libc::c_int,
    pub tbl: *mut s_x4node,
    pub ht: *mut *mut s_x4node,
}
pub type x4node = s_x4node;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct s_x3node {
    pub data: *mut state,
    pub key: *mut config,
    pub next: *mut s_x3node,
    pub from: *mut *mut s_x3node,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct s_x3 {
    pub size: libc::c_int,
    pub count: libc::c_int,
    pub tbl: *mut s_x3node,
    pub ht: *mut *mut s_x3node,
}
pub type x3node = s_x3node;
pub type x2node = s_x2node;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct s_x2node {
    pub data: *mut symbol,
    pub key: *const libc::c_char,
    pub next: *mut s_x2node,
    pub from: *mut *mut s_x2node,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct s_x2 {
    pub size: libc::c_int,
    pub count: libc::c_int,
    pub tbl: *mut s_x2node,
    pub ht: *mut *mut s_x2node,
}
pub type option_type = libc::c_uint;
pub const OPT_FSTR: option_type = 8;
pub const OPT_FDBL: option_type = 7;
pub const OPT_FINT: option_type = 6;
pub const OPT_FFLAG: option_type = 5;
pub const OPT_STR: option_type = 4;
pub const OPT_DBL: option_type = 3;
pub const OPT_INT: option_type = 2;
pub const OPT_FLAG: option_type = 1;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct s_options {
    pub type_0: option_type,
    pub label: *const libc::c_char,
    pub arg: *mut libc::c_char,
    pub message: *const libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pstate {
    pub filename: *mut libc::c_char,
    pub tokenlineno: libc::c_int,
    pub errorcnt: libc::c_int,
    pub tokenstart: *mut libc::c_char,
    pub gp: *mut lemon,
    pub state: e_state,
    pub fallback: *mut symbol,
    pub tkclass: *mut symbol,
    pub lhs: *mut symbol,
    pub lhsalias: *const libc::c_char,
    pub nrhs: libc::c_int,
    pub rhs: [*mut symbol; 1000],
    pub alias: [*const libc::c_char; 1000],
    pub prevrule: *mut rule,
    pub declkeyword: *const libc::c_char,
    pub declargslot: *mut *mut libc::c_char,
    pub insertLineMacro: libc::c_int,
    pub decllinenoslot: *mut libc::c_int,
    pub declassoc: e_assoc,
    pub preccounter: libc::c_int,
    pub firstrule: *mut rule,
    pub lastrule: *mut rule,
}
pub type e_state = libc::c_uint;
pub const WAITING_FOR_TOKEN_NAME: e_state = 22;
pub const WAITING_FOR_CLASS_TOKEN: e_state = 21;
pub const WAITING_FOR_CLASS_ID: e_state = 20;
pub const WAITING_FOR_WILDCARD_ID: e_state = 19;
pub const WAITING_FOR_FALLBACK_ID: e_state = 18;
pub const WAITING_FOR_DATATYPE_SYMBOL: e_state = 17;
pub const WAITING_FOR_DESTRUCTOR_SYMBOL: e_state = 16;
pub const RESYNC_AFTER_DECL_ERROR: e_state = 15;
pub const RESYNC_AFTER_RULE_ERROR: e_state = 14;
pub const PRECEDENCE_MARK_2: e_state = 13;
pub const PRECEDENCE_MARK_1: e_state = 12;
pub const RHS_ALIAS_2: e_state = 11;
pub const RHS_ALIAS_1: e_state = 10;
pub const LHS_ALIAS_3: e_state = 9;
pub const LHS_ALIAS_2: e_state = 8;
pub const LHS_ALIAS_1: e_state = 7;
pub const IN_RHS: e_state = 6;
pub const WAITING_FOR_ARROW: e_state = 5;
pub const WAITING_FOR_PRECEDENCE_SYMBOL: e_state = 4;
pub const WAITING_FOR_DECL_ARG: e_state = 3;
pub const WAITING_FOR_DECL_KEYWORD: e_state = 2;
pub const WAITING_FOR_DECL_OR_RULE: e_state = 1;
pub const INITIALIZE: e_state = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct s_x1node {
    pub data: *const libc::c_char,
    pub next: *mut s_x1node,
    pub from: *mut *mut s_x1node,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct s_x1 {
    pub size: libc::c_int,
    pub count: libc::c_int,
    pub tbl: *mut s_x1node,
    pub ht: *mut *mut s_x1node,
}
pub type x1node = s_x1node;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct acttab {
    pub nAction: libc::c_int,
    pub nActionAlloc: libc::c_int,
    pub aAction: *mut lookahead_action,
    pub aLookahead: *mut lookahead_action,
    pub mnLookahead: libc::c_int,
    pub mnAction: libc::c_int,
    pub mxLookahead: libc::c_int,
    pub nLookahead: libc::c_int,
    pub nLookaheadAlloc: libc::c_int,
    pub nterminal: libc::c_int,
    pub nsymbol: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct lookahead_action {
    pub lookahead: libc::c_int,
    pub action: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct axset {
    pub stp: *mut state,
    pub isTkn: libc::c_int,
    pub nAction: libc::c_int,
    pub iOrder: libc::c_int,
}
static mut showPrecedenceConflict: libc::c_int = 0 as libc::c_int;
unsafe extern "C" fn lemon_addtext(
    mut zBuf: *mut libc::c_char,
    mut pnUsed: *mut libc::c_int,
    mut zIn: *const libc::c_char,
    mut nIn: libc::c_int,
    mut iWidth: libc::c_int,
) {
    if nIn < 0 as libc::c_int {
        nIn = 0 as libc::c_int;
        while *zIn.offset(nIn as isize) != 0 {
            nIn += 1;
        }
    }
    while iWidth > nIn {
        let fresh0 = *pnUsed;
        *pnUsed = *pnUsed + 1;
        *zBuf.offset(fresh0 as isize) = ' ' as i32 as libc::c_char;
        iWidth -= 1;
    }
    if nIn == 0 as libc::c_int {
        return;
    }
    memcpy(
        &mut *zBuf.offset(*pnUsed as isize) as *mut libc::c_char as *mut libc::c_void,
        zIn as *const libc::c_void,
        nIn as libc::c_ulong,
    );
    *pnUsed += nIn;
    while -iWidth > nIn {
        let fresh1 = *pnUsed;
        *pnUsed = *pnUsed + 1;
        *zBuf.offset(fresh1 as isize) = ' ' as i32 as libc::c_char;
        iWidth += 1;
    }
    *zBuf.offset(*pnUsed as isize) = 0 as libc::c_int as libc::c_char;
}
unsafe extern "C" fn lemon_vsprintf(
    mut str: *mut libc::c_char,
    mut zFormat: *const libc::c_char,
    mut ap: ::std::ffi::VaList,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut c: libc::c_int = 0;
    let mut nUsed: libc::c_int = 0 as libc::c_int;
    let mut z: *const libc::c_char = 0 as *const libc::c_char;
    let mut zTemp: [libc::c_char; 50] = [0; 50];
    *str.offset(0 as libc::c_int as isize) = 0 as libc::c_int as libc::c_char;
    j = 0 as libc::c_int;
    i = j;
    loop {
        c = *zFormat.offset(i as isize) as libc::c_int;
        if !(c != 0 as libc::c_int) {
            break;
        }
        if c == '%' as i32 {
            let mut iWidth: libc::c_int = 0 as libc::c_int;
            lemon_addtext(
                str,
                &mut nUsed,
                &*zFormat.offset(j as isize),
                i - j,
                0 as libc::c_int,
            );
            i += 1;
            c = *zFormat.offset(i as isize) as libc::c_int;
            if *(*__ctype_b_loc()).offset(c as libc::c_uchar as libc::c_int as isize)
                as libc::c_int & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int
                != 0
                || c == '-' as i32
                    && *(*__ctype_b_loc())
                        .offset(
                            *zFormat.offset((i + 1 as libc::c_int) as isize)
                                as libc::c_uchar as libc::c_int as isize,
                        ) as libc::c_int
                        & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int != 0
            {
                if c == '-' as i32 {
                    i += 1;
                }
                while *(*__ctype_b_loc())
                    .offset(
                        *zFormat.offset(i as isize) as libc::c_uchar as libc::c_int
                            as isize,
                    ) as libc::c_int
                    & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int != 0
                {
                    let fresh2 = i;
                    i = i + 1;
                    iWidth = iWidth * 10 as libc::c_int
                        + *zFormat.offset(fresh2 as isize) as libc::c_int - '0' as i32;
                }
                if c == '-' as i32 {
                    iWidth = -iWidth;
                }
                c = *zFormat.offset(i as isize) as libc::c_int;
            }
            if c == 'd' as i32 {
                let mut v: libc::c_int = ap.arg::<libc::c_int>();
                if v < 0 as libc::c_int {
                    lemon_addtext(
                        str,
                        &mut nUsed,
                        b"-\0" as *const u8 as *const libc::c_char,
                        1 as libc::c_int,
                        iWidth,
                    );
                    v = -v;
                } else if v == 0 as libc::c_int {
                    lemon_addtext(
                        str,
                        &mut nUsed,
                        b"0\0" as *const u8 as *const libc::c_char,
                        1 as libc::c_int,
                        iWidth,
                    );
                }
                k = 0 as libc::c_int;
                while v > 0 as libc::c_int {
                    k += 1;
                    zTemp[(::std::mem::size_of::<[libc::c_char; 50]>() as libc::c_ulong)
                        .wrapping_sub(k as libc::c_ulong)
                        as usize] = (v % 10 as libc::c_int + '0' as i32) as libc::c_char;
                    v /= 10 as libc::c_int;
                }
                lemon_addtext(
                    str,
                    &mut nUsed,
                    &mut *zTemp
                        .as_mut_ptr()
                        .offset(
                            (::std::mem::size_of::<[libc::c_char; 50]>()
                                as libc::c_ulong)
                                .wrapping_sub(k as libc::c_ulong) as isize,
                        ),
                    k,
                    iWidth,
                );
            } else if c == 's' as i32 {
                z = ap.arg::<*const libc::c_char>();
                lemon_addtext(str, &mut nUsed, z, -(1 as libc::c_int), iWidth);
            } else if c == '.' as i32
                && memcmp(
                    &*zFormat.offset(i as isize) as *const libc::c_char
                        as *const libc::c_void,
                    b".*s\0" as *const u8 as *const libc::c_char as *const libc::c_void,
                    3 as libc::c_int as libc::c_ulong,
                ) == 0 as libc::c_int
            {
                i += 2 as libc::c_int;
                k = ap.arg::<libc::c_int>();
                z = ap.arg::<*const libc::c_char>();
                lemon_addtext(str, &mut nUsed, z, k, iWidth);
            } else if c == '%' as i32 {
                lemon_addtext(
                    str,
                    &mut nUsed,
                    b"%\0" as *const u8 as *const libc::c_char,
                    1 as libc::c_int,
                    0 as libc::c_int,
                );
            } else {
                fprintf(
                    stderr,
                    b"illegal format\n\0" as *const u8 as *const libc::c_char,
                );
                exit(1 as libc::c_int);
            }
            j = i + 1 as libc::c_int;
        }
        i += 1;
    }
    lemon_addtext(
        str,
        &mut nUsed,
        &*zFormat.offset(j as isize),
        i - j,
        0 as libc::c_int,
    );
    return nUsed;
}
unsafe extern "C" fn lemon_sprintf(
    mut str: *mut libc::c_char,
    mut format: *const libc::c_char,
    mut args: ...
) -> libc::c_int {
    let mut ap: ::std::ffi::VaListImpl;
    let mut rc: libc::c_int = 0;
    ap = args.clone();
    rc = lemon_vsprintf(str, format, ap.as_va_list());
    return rc;
}
unsafe extern "C" fn lemon_strcpy(
    mut dest: *mut libc::c_char,
    mut src: *const libc::c_char,
) {
    loop {
        let fresh3 = src;
        src = src.offset(1);
        let fresh4 = dest;
        dest = dest.offset(1);
        *fresh4 = *fresh3;
        if !(*fresh4 as libc::c_int != 0 as libc::c_int) {
            break;
        }
    };
}
unsafe extern "C" fn lemon_strcat(
    mut dest: *mut libc::c_char,
    mut src: *const libc::c_char,
) {
    while *dest != 0 {
        dest = dest.offset(1);
    }
    lemon_strcpy(dest, src);
}
unsafe extern "C" fn Action_new() -> *mut action {
    static mut actionfreelist: *mut action = 0 as *const action as *mut action;
    let mut newaction: *mut action = 0 as *mut action;
    if actionfreelist.is_null() {
        let mut i: libc::c_int = 0;
        let mut amt: libc::c_int = 100 as libc::c_int;
        actionfreelist = calloc(
            amt as libc::c_ulong,
            ::std::mem::size_of::<action>() as libc::c_ulong,
        ) as *mut action;
        if actionfreelist.is_null() {
            fprintf(
                stderr,
                b"Unable to allocate memory for a new parser action.\0" as *const u8
                    as *const libc::c_char,
            );
            exit(1 as libc::c_int);
        }
        i = 0 as libc::c_int;
        while i < amt - 1 as libc::c_int {
            let ref mut fresh5 = (*actionfreelist.offset(i as isize)).next;
            *fresh5 = &mut *actionfreelist.offset((i + 1 as libc::c_int) as isize)
                as *mut action;
            i += 1;
        }
        let ref mut fresh6 = (*actionfreelist.offset((amt - 1 as libc::c_int) as isize))
            .next;
        *fresh6 = 0 as *mut action;
    }
    newaction = actionfreelist;
    actionfreelist = (*actionfreelist).next;
    return newaction;
}
unsafe extern "C" fn actioncmp(
    mut ap1: *mut action,
    mut ap2: *mut action,
) -> libc::c_int {
    let mut rc: libc::c_int = 0;
    rc = (*(*ap1).sp).index - (*(*ap2).sp).index;
    if rc == 0 as libc::c_int {
        rc = (*ap1).type_0 as libc::c_int - (*ap2).type_0 as libc::c_int;
    }
    if rc == 0 as libc::c_int
        && ((*ap1).type_0 as libc::c_uint == REDUCE as libc::c_int as libc::c_uint
            || (*ap1).type_0 as libc::c_uint
                == SHIFTREDUCE as libc::c_int as libc::c_uint)
    {
        rc = (*(*ap1).x.rp).index - (*(*ap2).x.rp).index;
    }
    if rc == 0 as libc::c_int {
        rc = ap2.offset_from(ap1) as libc::c_long as libc::c_int;
    }
    return rc;
}
unsafe extern "C" fn Action_sort(mut ap: *mut action) -> *mut action {
    ap = msort(
        ap as *mut libc::c_char,
        &mut (*ap).next as *mut *mut action as *mut *mut libc::c_char,
        ::std::mem::transmute::<
            Option::<unsafe extern "C" fn(*mut action, *mut action) -> libc::c_int>,
            Option::<
                unsafe extern "C" fn(
                    *const libc::c_char,
                    *const libc::c_char,
                ) -> libc::c_int,
            >,
        >(
            Some(
                actioncmp
                    as unsafe extern "C" fn(*mut action, *mut action) -> libc::c_int,
            ),
        ),
    ) as *mut action;
    return ap;
}
#[no_mangle]
pub unsafe extern "C" fn Action_add(
    mut app: *mut *mut action,
    mut type_0: e_action,
    mut sp: *mut symbol,
    mut arg: *mut libc::c_char,
) {
    let mut newaction: *mut action = 0 as *mut action;
    newaction = Action_new();
    let ref mut fresh7 = (*newaction).next;
    *fresh7 = *app;
    *app = newaction;
    (*newaction).type_0 = type_0;
    let ref mut fresh8 = (*newaction).sp;
    *fresh8 = sp;
    let ref mut fresh9 = (*newaction).spOpt;
    *fresh9 = 0 as *mut symbol;
    if type_0 as libc::c_uint == SHIFT as libc::c_int as libc::c_uint {
        let ref mut fresh10 = (*newaction).x.stp;
        *fresh10 = arg as *mut state;
    } else {
        let ref mut fresh11 = (*newaction).x.rp;
        *fresh11 = arg as *mut rule;
    };
}
#[no_mangle]
pub unsafe extern "C" fn acttab_free(mut p: *mut acttab) {
    free((*p).aAction as *mut libc::c_void);
    free((*p).aLookahead as *mut libc::c_void);
    free(p as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn acttab_alloc(
    mut nsymbol: libc::c_int,
    mut nterminal: libc::c_int,
) -> *mut acttab {
    let mut p: *mut acttab = calloc(
        1 as libc::c_int as libc::c_ulong,
        ::std::mem::size_of::<acttab>() as libc::c_ulong,
    ) as *mut acttab;
    if p.is_null() {
        fprintf(
            stderr,
            b"Unable to allocate memory for a new acttab.\0" as *const u8
                as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    memset(
        p as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<acttab>() as libc::c_ulong,
    );
    (*p).nsymbol = nsymbol;
    (*p).nterminal = nterminal;
    return p;
}
#[no_mangle]
pub unsafe extern "C" fn acttab_action(
    mut p: *mut acttab,
    mut lookahead: libc::c_int,
    mut action: libc::c_int,
) {
    if (*p).nLookahead >= (*p).nLookaheadAlloc {
        (*p).nLookaheadAlloc += 25 as libc::c_int;
        let ref mut fresh12 = (*p).aLookahead;
        *fresh12 = realloc(
            (*p).aLookahead as *mut libc::c_void,
            (::std::mem::size_of::<lookahead_action>() as libc::c_ulong)
                .wrapping_mul((*p).nLookaheadAlloc as libc::c_ulong),
        ) as *mut lookahead_action;
        if ((*p).aLookahead).is_null() {
            fprintf(stderr, b"malloc failed\n\0" as *const u8 as *const libc::c_char);
            exit(1 as libc::c_int);
        }
    }
    if (*p).nLookahead == 0 as libc::c_int {
        (*p).mxLookahead = lookahead;
        (*p).mnLookahead = lookahead;
        (*p).mnAction = action;
    } else {
        if (*p).mxLookahead < lookahead {
            (*p).mxLookahead = lookahead;
        }
        if (*p).mnLookahead > lookahead {
            (*p).mnLookahead = lookahead;
            (*p).mnAction = action;
        }
    }
    (*((*p).aLookahead).offset((*p).nLookahead as isize)).lookahead = lookahead;
    (*((*p).aLookahead).offset((*p).nLookahead as isize)).action = action;
    let ref mut fresh13 = (*p).nLookahead;
    *fresh13 += 1;
}
#[no_mangle]
pub unsafe extern "C" fn acttab_insert(
    mut p: *mut acttab,
    mut makeItSafe: libc::c_int,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut n: libc::c_int = 0;
    let mut end: libc::c_int = 0;
    if (*p).nLookahead > 0 as libc::c_int {} else {
        __assert_fail(
            b"p->nLookahead>0\0" as *const u8 as *const libc::c_char,
            b"/home/me/github/learnrust/libsql/tool/lemon.c\0" as *const u8
                as *const libc::c_char,
            682 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 33],
                &[libc::c_char; 33],
            >(b"int acttab_insert(acttab *, int)\0"))
                .as_ptr(),
        );
    }
    n = (*p).nsymbol + 1 as libc::c_int;
    if (*p).nAction + n >= (*p).nActionAlloc {
        let mut oldAlloc: libc::c_int = (*p).nActionAlloc;
        (*p).nActionAlloc = (*p).nAction + n + (*p).nActionAlloc + 20 as libc::c_int;
        let ref mut fresh14 = (*p).aAction;
        *fresh14 = realloc(
            (*p).aAction as *mut libc::c_void,
            (::std::mem::size_of::<lookahead_action>() as libc::c_ulong)
                .wrapping_mul((*p).nActionAlloc as libc::c_ulong),
        ) as *mut lookahead_action;
        if ((*p).aAction).is_null() {
            fprintf(stderr, b"malloc failed\n\0" as *const u8 as *const libc::c_char);
            exit(1 as libc::c_int);
        }
        i = oldAlloc;
        while i < (*p).nActionAlloc {
            (*((*p).aAction).offset(i as isize)).lookahead = -(1 as libc::c_int);
            (*((*p).aAction).offset(i as isize)).action = -(1 as libc::c_int);
            i += 1;
        }
    }
    end = if makeItSafe != 0 { (*p).mnLookahead } else { 0 as libc::c_int };
    i = (*p).nAction - 1 as libc::c_int;
    while i >= end {
        if (*((*p).aAction).offset(i as isize)).lookahead == (*p).mnLookahead {
            if !((*((*p).aAction).offset(i as isize)).action != (*p).mnAction) {
                j = 0 as libc::c_int;
                while j < (*p).nLookahead {
                    k = (*((*p).aLookahead).offset(j as isize)).lookahead
                        - (*p).mnLookahead + i;
                    if k < 0 as libc::c_int || k >= (*p).nAction {
                        break;
                    }
                    if (*((*p).aLookahead).offset(j as isize)).lookahead
                        != (*((*p).aAction).offset(k as isize)).lookahead
                    {
                        break;
                    }
                    if (*((*p).aLookahead).offset(j as isize)).action
                        != (*((*p).aAction).offset(k as isize)).action
                    {
                        break;
                    }
                    j += 1;
                }
                if !(j < (*p).nLookahead) {
                    n = 0 as libc::c_int;
                    j = 0 as libc::c_int;
                    while j < (*p).nAction {
                        if !((*((*p).aAction).offset(j as isize)).lookahead
                            < 0 as libc::c_int)
                        {
                            if (*((*p).aAction).offset(j as isize)).lookahead
                                == j + (*p).mnLookahead - i
                            {
                                n += 1;
                            }
                        }
                        j += 1;
                    }
                    if n == (*p).nLookahead {
                        break;
                    }
                }
            }
        }
        i -= 1;
    }
    if i < end {
        i = if makeItSafe != 0 { (*p).mnLookahead } else { 0 as libc::c_int };
        while i < (*p).nActionAlloc - (*p).mxLookahead {
            if (*((*p).aAction).offset(i as isize)).lookahead < 0 as libc::c_int {
                j = 0 as libc::c_int;
                while j < (*p).nLookahead {
                    k = (*((*p).aLookahead).offset(j as isize)).lookahead
                        - (*p).mnLookahead + i;
                    if k < 0 as libc::c_int {
                        break;
                    }
                    if (*((*p).aAction).offset(k as isize)).lookahead >= 0 as libc::c_int
                    {
                        break;
                    }
                    j += 1;
                }
                if !(j < (*p).nLookahead) {
                    j = 0 as libc::c_int;
                    while j < (*p).nAction {
                        if (*((*p).aAction).offset(j as isize)).lookahead
                            == j + (*p).mnLookahead - i
                        {
                            break;
                        }
                        j += 1;
                    }
                    if j == (*p).nAction {
                        break;
                    }
                }
            }
            i += 1;
        }
    }
    j = 0 as libc::c_int;
    while j < (*p).nLookahead {
        k = (*((*p).aLookahead).offset(j as isize)).lookahead - (*p).mnLookahead + i;
        *((*p).aAction).offset(k as isize) = *((*p).aLookahead).offset(j as isize);
        if k >= (*p).nAction {
            (*p).nAction = k + 1 as libc::c_int;
        }
        j += 1;
    }
    if makeItSafe != 0 && i + (*p).nterminal >= (*p).nAction {
        (*p).nAction = i + (*p).nterminal + 1 as libc::c_int;
    }
    (*p).nLookahead = 0 as libc::c_int;
    return i - (*p).mnLookahead;
}
#[no_mangle]
pub unsafe extern "C" fn acttab_action_size(mut p: *mut acttab) -> libc::c_int {
    let mut n: libc::c_int = (*p).nAction;
    while n > 0 as libc::c_int
        && (*((*p).aAction).offset((n - 1 as libc::c_int) as isize)).lookahead
            < 0 as libc::c_int
    {
        n -= 1;
    }
    return n;
}
#[no_mangle]
pub unsafe extern "C" fn FindRulePrecedences(mut xp: *mut lemon) {
    let mut rp: *mut rule = 0 as *mut rule;
    rp = (*xp).rule;
    while !rp.is_null() {
        if ((*rp).precsym).is_null() {
            let mut i: libc::c_int = 0;
            let mut j: libc::c_int = 0;
            i = 0 as libc::c_int;
            while i < (*rp).nrhs && ((*rp).precsym).is_null() {
                let mut sp: *mut symbol = *((*rp).rhs).offset(i as isize);
                if (*sp).type_0 as libc::c_uint
                    == MULTITERMINAL as libc::c_int as libc::c_uint
                {
                    j = 0 as libc::c_int;
                    while j < (*sp).nsubsym {
                        if (**((*sp).subsym).offset(j as isize)).prec >= 0 as libc::c_int
                        {
                            let ref mut fresh15 = (*rp).precsym;
                            *fresh15 = *((*sp).subsym).offset(j as isize);
                            break;
                        } else {
                            j += 1;
                        }
                    }
                } else if (*sp).prec >= 0 as libc::c_int {
                    let ref mut fresh16 = (*rp).precsym;
                    *fresh16 = *((*rp).rhs).offset(i as isize);
                }
                i += 1;
            }
        }
        rp = (*rp).next;
    }
}
#[no_mangle]
pub unsafe extern "C" fn FindFirstSets(mut lemp: *mut lemon) {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut rp: *mut rule = 0 as *mut rule;
    let mut progress: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < (*lemp).nsymbol {
        (**((*lemp).symbols).offset(i as isize)).lambda = LEMON_FALSE;
        i += 1;
    }
    i = (*lemp).nterminal;
    while i < (*lemp).nsymbol {
        let ref mut fresh17 = (**((*lemp).symbols).offset(i as isize)).firstset;
        *fresh17 = SetNew();
        i += 1;
    }
    loop {
        progress = 0 as libc::c_int;
        rp = (*lemp).rule;
        while !rp.is_null() {
            if !((*(*rp).lhs).lambda as u64 != 0) {
                i = 0 as libc::c_int;
                while i < (*rp).nrhs {
                    let mut sp: *mut symbol = *((*rp).rhs).offset(i as isize);
                    if (*sp).type_0 as libc::c_uint
                        == NONTERMINAL as libc::c_int as libc::c_uint
                        || (*sp).lambda as libc::c_uint
                            == LEMON_FALSE as libc::c_int as libc::c_uint
                    {} else {
                        __assert_fail(
                            b"sp->type==NONTERMINAL || sp->lambda==LEMON_FALSE\0"
                                as *const u8 as *const libc::c_char,
                            b"/home/me/github/learnrust/libsql/tool/lemon.c\0"
                                as *const u8 as *const libc::c_char,
                            859 as libc::c_int as libc::c_uint,
                            (*::std::mem::transmute::<
                                &[u8; 35],
                                &[libc::c_char; 35],
                            >(b"void FindFirstSets(struct lemon *)\0"))
                                .as_ptr(),
                        );
                    }
                    if (*sp).lambda as libc::c_uint
                        == LEMON_FALSE as libc::c_int as libc::c_uint
                    {
                        break;
                    }
                    i += 1;
                }
                if i == (*rp).nrhs {
                    (*(*rp).lhs).lambda = LEMON_TRUE;
                    progress = 1 as libc::c_int;
                }
            }
            rp = (*rp).next;
        }
        if !(progress != 0) {
            break;
        }
    }
    loop {
        let mut s1: *mut symbol = 0 as *mut symbol;
        let mut s2: *mut symbol = 0 as *mut symbol;
        progress = 0 as libc::c_int;
        rp = (*lemp).rule;
        while !rp.is_null() {
            s1 = (*rp).lhs;
            i = 0 as libc::c_int;
            while i < (*rp).nrhs {
                s2 = *((*rp).rhs).offset(i as isize);
                if (*s2).type_0 as libc::c_uint
                    == TERMINAL as libc::c_int as libc::c_uint
                {
                    progress += SetAdd((*s1).firstset, (*s2).index);
                    break;
                } else if (*s2).type_0 as libc::c_uint
                    == MULTITERMINAL as libc::c_int as libc::c_uint
                {
                    j = 0 as libc::c_int;
                    while j < (*s2).nsubsym {
                        progress
                            += SetAdd(
                                (*s1).firstset,
                                (**((*s2).subsym).offset(j as isize)).index,
                            );
                        j += 1;
                    }
                    break;
                } else {
                    if s1 == s2 {
                        if (*s1).lambda as libc::c_uint
                            == LEMON_FALSE as libc::c_int as libc::c_uint
                        {
                            break;
                        }
                    } else {
                        progress += SetUnion((*s1).firstset, (*s2).firstset);
                        if (*s2).lambda as libc::c_uint
                            == LEMON_FALSE as libc::c_int as libc::c_uint
                        {
                            break;
                        }
                    }
                    i += 1;
                }
            }
            rp = (*rp).next;
        }
        if !(progress != 0) {
            break;
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn FindStates(mut lemp: *mut lemon) {
    let mut sp: *mut symbol = 0 as *mut symbol;
    let mut rp: *mut rule = 0 as *mut rule;
    Configlist_init();
    if !((*lemp).start).is_null() {
        sp = Symbol_find((*lemp).start);
        if sp.is_null() {
            ErrorMsg(
                (*lemp).filename,
                0 as libc::c_int,
                b"The specified start symbol \"%s\" is not in a nonterminal of the grammar.  \"%s\" will be used as the start symbol instead.\0"
                    as *const u8 as *const libc::c_char,
                (*lemp).start,
                (*(*(*lemp).startRule).lhs).name,
            );
            let ref mut fresh18 = (*lemp).errorcnt;
            *fresh18 += 1;
            sp = (*(*lemp).startRule).lhs;
        }
    } else if !((*lemp).startRule).is_null() {
        sp = (*(*lemp).startRule).lhs;
    } else {
        ErrorMsg(
            (*lemp).filename,
            0 as libc::c_int,
            b"Internal error - no start rule\n\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    rp = (*lemp).rule;
    while !rp.is_null() {
        let mut i: libc::c_int = 0;
        i = 0 as libc::c_int;
        while i < (*rp).nrhs {
            if *((*rp).rhs).offset(i as isize) == sp {
                ErrorMsg(
                    (*lemp).filename,
                    0 as libc::c_int,
                    b"The start symbol \"%s\" occurs on the right-hand side of a rule. This will result in a parser which does not work properly.\0"
                        as *const u8 as *const libc::c_char,
                    (*sp).name,
                );
                let ref mut fresh19 = (*lemp).errorcnt;
                *fresh19 += 1;
            }
            i += 1;
        }
        rp = (*rp).next;
    }
    rp = (*sp).rule;
    while !rp.is_null() {
        let mut newcfp: *mut config = 0 as *mut config;
        (*rp).lhsStart = 1 as libc::c_int;
        newcfp = Configlist_addbasis(rp, 0 as libc::c_int);
        SetAdd((*newcfp).fws, 0 as libc::c_int);
        rp = (*rp).nextlhs;
    }
    getstate(lemp);
}
#[no_mangle]
pub unsafe extern "C" fn getstate(mut lemp: *mut lemon) -> *mut state {
    let mut cfp: *mut config = 0 as *mut config;
    let mut bp: *mut config = 0 as *mut config;
    let mut stp: *mut state = 0 as *mut state;
    Configlist_sortbasis();
    bp = Configlist_basis();
    stp = State_find(bp);
    if !stp.is_null() {
        let mut x: *mut config = 0 as *mut config;
        let mut y: *mut config = 0 as *mut config;
        x = bp;
        y = (*stp).bp;
        while !x.is_null() && !y.is_null() {
            Plink_copy(&mut (*y).bplp, (*x).bplp);
            Plink_delete((*x).fplp);
            let ref mut fresh20 = (*x).bplp;
            *fresh20 = 0 as *mut plink;
            let ref mut fresh21 = (*x).fplp;
            *fresh21 = *fresh20;
            x = (*x).bp;
            y = (*y).bp;
        }
        cfp = Configlist_return();
        Configlist_eat(cfp);
    } else {
        Configlist_closure(lemp);
        Configlist_sort();
        cfp = Configlist_return();
        stp = State_new();
        if stp.is_null() {
            memory_error();
        }
        let ref mut fresh22 = (*stp).bp;
        *fresh22 = bp;
        let ref mut fresh23 = (*stp).cfp;
        *fresh23 = cfp;
        let ref mut fresh24 = (*lemp).nstate;
        let fresh25 = *fresh24;
        *fresh24 = *fresh24 + 1;
        (*stp).statenum = fresh25;
        let ref mut fresh26 = (*stp).ap;
        *fresh26 = 0 as *mut action;
        State_insert(stp, (*stp).bp);
        buildshifts(lemp, stp);
    }
    return stp;
}
#[no_mangle]
pub unsafe extern "C" fn same_symbol(
    mut a: *mut symbol,
    mut b: *mut symbol,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    if a == b {
        return 1 as libc::c_int;
    }
    if (*a).type_0 as libc::c_uint != MULTITERMINAL as libc::c_int as libc::c_uint {
        return 0 as libc::c_int;
    }
    if (*b).type_0 as libc::c_uint != MULTITERMINAL as libc::c_int as libc::c_uint {
        return 0 as libc::c_int;
    }
    if (*a).nsubsym != (*b).nsubsym {
        return 0 as libc::c_int;
    }
    i = 0 as libc::c_int;
    while i < (*a).nsubsym {
        if *((*a).subsym).offset(i as isize) != *((*b).subsym).offset(i as isize) {
            return 0 as libc::c_int;
        }
        i += 1;
    }
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn buildshifts(mut lemp: *mut lemon, mut stp: *mut state) {
    let mut cfp: *mut config = 0 as *mut config;
    let mut bcfp: *mut config = 0 as *mut config;
    let mut newcfg: *mut config = 0 as *mut config;
    let mut sp: *mut symbol = 0 as *mut symbol;
    let mut bsp: *mut symbol = 0 as *mut symbol;
    let mut newstp: *mut state = 0 as *mut state;
    cfp = (*stp).cfp;
    while !cfp.is_null() {
        (*cfp).status = INCOMPLETE;
        cfp = (*cfp).next;
    }
    cfp = (*stp).cfp;
    while !cfp.is_null() {
        if !((*cfp).status as libc::c_uint == COMPLETE as libc::c_int as libc::c_uint) {
            if !((*cfp).dot >= (*(*cfp).rp).nrhs) {
                Configlist_reset();
                sp = *((*(*cfp).rp).rhs).offset((*cfp).dot as isize);
                bcfp = cfp;
                while !bcfp.is_null() {
                    if !((*bcfp).status as libc::c_uint
                        == COMPLETE as libc::c_int as libc::c_uint)
                    {
                        if !((*bcfp).dot >= (*(*bcfp).rp).nrhs) {
                            bsp = *((*(*bcfp).rp).rhs).offset((*bcfp).dot as isize);
                            if !(same_symbol(bsp, sp) == 0) {
                                (*bcfp).status = COMPLETE;
                                newcfg = Configlist_addbasis(
                                    (*bcfp).rp,
                                    (*bcfp).dot + 1 as libc::c_int,
                                );
                                Plink_add(&mut (*newcfg).bplp, bcfp);
                            }
                        }
                    }
                    bcfp = (*bcfp).next;
                }
                newstp = getstate(lemp);
                if (*sp).type_0 as libc::c_uint
                    == MULTITERMINAL as libc::c_int as libc::c_uint
                {
                    let mut i: libc::c_int = 0;
                    i = 0 as libc::c_int;
                    while i < (*sp).nsubsym {
                        Action_add(
                            &mut (*stp).ap,
                            SHIFT,
                            *((*sp).subsym).offset(i as isize),
                            newstp as *mut libc::c_char,
                        );
                        i += 1;
                    }
                } else {
                    Action_add(&mut (*stp).ap, SHIFT, sp, newstp as *mut libc::c_char);
                }
            }
        }
        cfp = (*cfp).next;
    }
}
#[no_mangle]
pub unsafe extern "C" fn FindLinks(mut lemp: *mut lemon) {
    let mut i: libc::c_int = 0;
    let mut cfp: *mut config = 0 as *mut config;
    let mut other: *mut config = 0 as *mut config;
    let mut stp: *mut state = 0 as *mut state;
    let mut plp: *mut plink = 0 as *mut plink;
    i = 0 as libc::c_int;
    while i < (*lemp).nstate {
        stp = *((*lemp).sorted).offset(i as isize);
        cfp = if !stp.is_null() { (*stp).cfp } else { 0 as *mut config };
        while !cfp.is_null() {
            let ref mut fresh27 = (*cfp).stp;
            *fresh27 = stp;
            cfp = (*cfp).next;
        }
        i += 1;
    }
    i = 0 as libc::c_int;
    while i < (*lemp).nstate {
        stp = *((*lemp).sorted).offset(i as isize);
        cfp = if !stp.is_null() { (*stp).cfp } else { 0 as *mut config };
        while !cfp.is_null() {
            plp = (*cfp).bplp;
            while !plp.is_null() {
                other = (*plp).cfp;
                Plink_add(&mut (*other).fplp, cfp);
                plp = (*plp).next;
            }
            cfp = (*cfp).next;
        }
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn FindFollowSets(mut lemp: *mut lemon) {
    let mut i: libc::c_int = 0;
    let mut cfp: *mut config = 0 as *mut config;
    let mut plp: *mut plink = 0 as *mut plink;
    let mut progress: libc::c_int = 0;
    let mut change: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < (*lemp).nstate {
        if !(*((*lemp).sorted).offset(i as isize)).is_null() {} else {
            __assert_fail(
                b"lemp->sorted[i]!=0\0" as *const u8 as *const libc::c_char,
                b"/home/me/github/learnrust/libsql/tool/lemon.c\0" as *const u8
                    as *const libc::c_char,
                1121 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 36],
                    &[libc::c_char; 36],
                >(b"void FindFollowSets(struct lemon *)\0"))
                    .as_ptr(),
            );
        }
        cfp = (**((*lemp).sorted).offset(i as isize)).cfp;
        while !cfp.is_null() {
            (*cfp).status = INCOMPLETE;
            cfp = (*cfp).next;
        }
        i += 1;
    }
    loop {
        progress = 0 as libc::c_int;
        i = 0 as libc::c_int;
        while i < (*lemp).nstate {
            if !(*((*lemp).sorted).offset(i as isize)).is_null() {} else {
                __assert_fail(
                    b"lemp->sorted[i]!=0\0" as *const u8 as *const libc::c_char,
                    b"/home/me/github/learnrust/libsql/tool/lemon.c\0" as *const u8
                        as *const libc::c_char,
                    1130 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 36],
                        &[libc::c_char; 36],
                    >(b"void FindFollowSets(struct lemon *)\0"))
                        .as_ptr(),
                );
            }
            cfp = (**((*lemp).sorted).offset(i as isize)).cfp;
            while !cfp.is_null() {
                if !((*cfp).status as libc::c_uint
                    == COMPLETE as libc::c_int as libc::c_uint)
                {
                    plp = (*cfp).fplp;
                    while !plp.is_null() {
                        change = SetUnion((*(*plp).cfp).fws, (*cfp).fws);
                        if change != 0 {
                            (*(*plp).cfp).status = INCOMPLETE;
                            progress = 1 as libc::c_int;
                        }
                        plp = (*plp).next;
                    }
                    (*cfp).status = COMPLETE;
                }
                cfp = (*cfp).next;
            }
            i += 1;
        }
        if !(progress != 0) {
            break;
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn FindActions(mut lemp: *mut lemon) {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut cfp: *mut config = 0 as *mut config;
    let mut stp: *mut state = 0 as *mut state;
    let mut sp: *mut symbol = 0 as *mut symbol;
    let mut rp: *mut rule = 0 as *mut rule;
    i = 0 as libc::c_int;
    while i < (*lemp).nstate {
        stp = *((*lemp).sorted).offset(i as isize);
        cfp = (*stp).cfp;
        while !cfp.is_null() {
            if (*(*cfp).rp).nrhs == (*cfp).dot {
                j = 0 as libc::c_int;
                while j < (*lemp).nterminal {
                    if *((*cfp).fws).offset(j as isize) != 0 {
                        Action_add(
                            &mut (*stp).ap,
                            REDUCE,
                            *((*lemp).symbols).offset(j as isize),
                            (*cfp).rp as *mut libc::c_char,
                        );
                    }
                    j += 1;
                }
            }
            cfp = (*cfp).next;
        }
        i += 1;
    }
    if !((*lemp).start).is_null() {
        sp = Symbol_find((*lemp).start);
        if sp.is_null() {
            if ((*lemp).startRule).is_null() {
                fprintf(
                    stderr,
                    b"internal error on source line %d: no start rule\n\0" as *const u8
                        as *const libc::c_char,
                    1183 as libc::c_int,
                );
                exit(1 as libc::c_int);
            }
            sp = (*(*lemp).startRule).lhs;
        }
    } else {
        sp = (*(*lemp).startRule).lhs;
    }
    Action_add(
        &mut (**((*lemp).sorted).offset(0 as libc::c_int as isize)).ap,
        ACCEPT,
        sp,
        0 as *mut libc::c_char,
    );
    i = 0 as libc::c_int;
    while i < (*lemp).nstate {
        let mut ap: *mut action = 0 as *mut action;
        let mut nap: *mut action = 0 as *mut action;
        stp = *((*lemp).sorted).offset(i as isize);
        let ref mut fresh28 = (*stp).ap;
        *fresh28 = Action_sort((*stp).ap);
        ap = (*stp).ap;
        while !ap.is_null() && !((*ap).next).is_null() {
            nap = (*ap).next;
            while !nap.is_null() && (*nap).sp == (*ap).sp {
                (*lemp).nconflict += resolve_conflict(ap, nap);
                nap = (*nap).next;
            }
            ap = (*ap).next;
        }
        i += 1;
    }
    rp = (*lemp).rule;
    while !rp.is_null() {
        (*rp).canReduce = LEMON_FALSE;
        rp = (*rp).next;
    }
    i = 0 as libc::c_int;
    while i < (*lemp).nstate {
        let mut ap_0: *mut action = 0 as *mut action;
        ap_0 = (**((*lemp).sorted).offset(i as isize)).ap;
        while !ap_0.is_null() {
            if (*ap_0).type_0 as libc::c_uint == REDUCE as libc::c_int as libc::c_uint {
                (*(*ap_0).x.rp).canReduce = LEMON_TRUE;
            }
            ap_0 = (*ap_0).next;
        }
        i += 1;
    }
    rp = (*lemp).rule;
    while !rp.is_null() {
        if !((*rp).canReduce as u64 != 0) {
            ErrorMsg(
                (*lemp).filename,
                (*rp).ruleline,
                b"This rule can not be reduced.\n\0" as *const u8 as *const libc::c_char,
            );
            let ref mut fresh29 = (*lemp).errorcnt;
            *fresh29 += 1;
        }
        rp = (*rp).next;
    }
}
unsafe extern "C" fn resolve_conflict(
    mut apx: *mut action,
    mut apy: *mut action,
) -> libc::c_int {
    let mut spx: *mut symbol = 0 as *mut symbol;
    let mut spy: *mut symbol = 0 as *mut symbol;
    let mut errcnt: libc::c_int = 0 as libc::c_int;
    if (*apx).sp == (*apy).sp {} else {
        __assert_fail(
            b"apx->sp==apy->sp\0" as *const u8 as *const libc::c_char,
            b"/home/me/github/learnrust/libsql/tool/lemon.c\0" as *const u8
                as *const libc::c_char,
            1245 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 55],
                &[libc::c_char; 55],
            >(b"int resolve_conflict(struct action *, struct action *)\0"))
                .as_ptr(),
        );
    }
    if (*apx).type_0 as libc::c_uint == SHIFT as libc::c_int as libc::c_uint
        && (*apy).type_0 as libc::c_uint == SHIFT as libc::c_int as libc::c_uint
    {
        (*apy).type_0 = SSCONFLICT;
        errcnt += 1;
    }
    if (*apx).type_0 as libc::c_uint == SHIFT as libc::c_int as libc::c_uint
        && (*apy).type_0 as libc::c_uint == REDUCE as libc::c_int as libc::c_uint
    {
        spx = (*apx).sp;
        spy = (*(*apy).x.rp).precsym;
        if spy.is_null() || (*spx).prec < 0 as libc::c_int
            || (*spy).prec < 0 as libc::c_int
        {
            (*apy).type_0 = SRCONFLICT;
            errcnt += 1;
        } else if (*spx).prec > (*spy).prec {
            (*apy).type_0 = RD_RESOLVED;
        } else if (*spx).prec < (*spy).prec {
            (*apx).type_0 = SH_RESOLVED;
        } else if (*spx).prec == (*spy).prec
            && (*spx).assoc as libc::c_uint == RIGHT as libc::c_int as libc::c_uint
        {
            (*apy).type_0 = RD_RESOLVED;
        } else if (*spx).prec == (*spy).prec
            && (*spx).assoc as libc::c_uint == LEFT as libc::c_int as libc::c_uint
        {
            (*apx).type_0 = SH_RESOLVED;
        } else {
            if (*spx).prec == (*spy).prec
                && (*spx).assoc as libc::c_uint == NONE as libc::c_int as libc::c_uint
            {} else {
                __assert_fail(
                    b"spx->prec==spy->prec && spx->assoc==NONE\0" as *const u8
                        as *const libc::c_char,
                    b"/home/me/github/learnrust/libsql/tool/lemon.c\0" as *const u8
                        as *const libc::c_char,
                    1266 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 55],
                        &[libc::c_char; 55],
                    >(b"int resolve_conflict(struct action *, struct action *)\0"))
                        .as_ptr(),
                );
            }
            (*apx).type_0 = ERROR;
        }
    } else if (*apx).type_0 as libc::c_uint == REDUCE as libc::c_int as libc::c_uint
        && (*apy).type_0 as libc::c_uint == REDUCE as libc::c_int as libc::c_uint
    {
        spx = (*(*apx).x.rp).precsym;
        spy = (*(*apy).x.rp).precsym;
        if spx.is_null() || spy.is_null() || (*spx).prec < 0 as libc::c_int
            || (*spy).prec < 0 as libc::c_int || (*spx).prec == (*spy).prec
        {
            (*apy).type_0 = RRCONFLICT;
            errcnt += 1;
        } else if (*spx).prec > (*spy).prec {
            (*apy).type_0 = RD_RESOLVED;
        } else if (*spx).prec < (*spy).prec {
            (*apx).type_0 = RD_RESOLVED;
        }
    } else if (*apx).type_0 as libc::c_uint == SH_RESOLVED as libc::c_int as libc::c_uint
        || (*apx).type_0 as libc::c_uint == RD_RESOLVED as libc::c_int as libc::c_uint
        || (*apx).type_0 as libc::c_uint == SSCONFLICT as libc::c_int as libc::c_uint
        || (*apx).type_0 as libc::c_uint == SRCONFLICT as libc::c_int as libc::c_uint
        || (*apx).type_0 as libc::c_uint == RRCONFLICT as libc::c_int as libc::c_uint
        || (*apy).type_0 as libc::c_uint == SH_RESOLVED as libc::c_int as libc::c_uint
        || (*apy).type_0 as libc::c_uint == RD_RESOLVED as libc::c_int as libc::c_uint
        || (*apy).type_0 as libc::c_uint == SSCONFLICT as libc::c_int as libc::c_uint
        || (*apy).type_0 as libc::c_uint == SRCONFLICT as libc::c_int as libc::c_uint
        || (*apy).type_0 as libc::c_uint == RRCONFLICT as libc::c_int as libc::c_uint
    {} else {
        __assert_fail(
            b"apx->type==SH_RESOLVED || apx->type==RD_RESOLVED || apx->type==SSCONFLICT || apx->type==SRCONFLICT || apx->type==RRCONFLICT || apy->type==SH_RESOLVED || apy->type==RD_RESOLVED || apy->type==SSCONFLICT || apy->type==SRCONFLICT || apy->type==RRCONFLICT\0"
                as *const u8 as *const libc::c_char,
            b"/home/me/github/learnrust/libsql/tool/lemon.c\0" as *const u8
                as *const libc::c_char,
            1293 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 55],
                &[libc::c_char; 55],
            >(b"int resolve_conflict(struct action *, struct action *)\0"))
                .as_ptr(),
        );
    }
    return errcnt;
}
static mut freelist: *mut config = 0 as *const config as *mut config;
static mut current: *mut config = 0 as *const config as *mut config;
static mut currentend: *mut *mut config = 0 as *const *mut config as *mut *mut config;
static mut basis: *mut config = 0 as *const config as *mut config;
static mut basisend: *mut *mut config = 0 as *const *mut config as *mut *mut config;
#[no_mangle]
pub unsafe extern "C" fn newconfig() -> *mut config {
    return calloc(
        1 as libc::c_int as libc::c_ulong,
        ::std::mem::size_of::<config>() as libc::c_ulong,
    ) as *mut config;
}
#[no_mangle]
pub unsafe extern "C" fn deleteconfig(mut old: *mut config) {
    let ref mut fresh30 = (*old).next;
    *fresh30 = freelist;
    freelist = old;
}
#[no_mangle]
pub unsafe extern "C" fn Configlist_init() {
    current = 0 as *mut config;
    currentend = &mut current;
    basis = 0 as *mut config;
    basisend = &mut basis;
    Configtable_init();
}
#[no_mangle]
pub unsafe extern "C" fn Configlist_reset() {
    current = 0 as *mut config;
    currentend = &mut current;
    basis = 0 as *mut config;
    basisend = &mut basis;
    Configtable_clear(None);
}
#[no_mangle]
pub unsafe extern "C" fn Configlist_add(
    mut rp: *mut rule,
    mut dot: libc::c_int,
) -> *mut config {
    let mut cfp: *mut config = 0 as *mut config;
    let mut model: config = config {
        rp: 0 as *mut rule,
        dot: 0,
        fws: 0 as *mut libc::c_char,
        fplp: 0 as *mut plink,
        bplp: 0 as *mut plink,
        stp: 0 as *mut state,
        status: COMPLETE,
        next: 0 as *mut config,
        bp: 0 as *mut config,
    };
    if !currentend.is_null() {} else {
        __assert_fail(
            b"currentend!=0\0" as *const u8 as *const libc::c_char,
            b"/home/me/github/learnrust/libsql/tool/lemon.c\0" as *const u8
                as *const libc::c_char,
            1351 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 50],
                &[libc::c_char; 50],
            >(b"struct config *Configlist_add(struct rule *, int)\0"))
                .as_ptr(),
        );
    }
    model.rp = rp;
    model.dot = dot;
    cfp = Configtable_find(&mut model);
    if cfp.is_null() {
        cfp = newconfig();
        let ref mut fresh31 = (*cfp).rp;
        *fresh31 = rp;
        (*cfp).dot = dot;
        let ref mut fresh32 = (*cfp).fws;
        *fresh32 = SetNew();
        let ref mut fresh33 = (*cfp).stp;
        *fresh33 = 0 as *mut state;
        let ref mut fresh34 = (*cfp).bplp;
        *fresh34 = 0 as *mut plink;
        let ref mut fresh35 = (*cfp).fplp;
        *fresh35 = *fresh34;
        let ref mut fresh36 = (*cfp).next;
        *fresh36 = 0 as *mut config;
        let ref mut fresh37 = (*cfp).bp;
        *fresh37 = 0 as *mut config;
        *currentend = cfp;
        currentend = &mut (*cfp).next;
        Configtable_insert(cfp);
    }
    return cfp;
}
#[no_mangle]
pub unsafe extern "C" fn Configlist_addbasis(
    mut rp: *mut rule,
    mut dot: libc::c_int,
) -> *mut config {
    let mut cfp: *mut config = 0 as *mut config;
    let mut model: config = config {
        rp: 0 as *mut rule,
        dot: 0,
        fws: 0 as *mut libc::c_char,
        fplp: 0 as *mut plink,
        bplp: 0 as *mut plink,
        stp: 0 as *mut state,
        status: COMPLETE,
        next: 0 as *mut config,
        bp: 0 as *mut config,
    };
    if !basisend.is_null() {} else {
        __assert_fail(
            b"basisend!=0\0" as *const u8 as *const libc::c_char,
            b"/home/me/github/learnrust/libsql/tool/lemon.c\0" as *const u8
                as *const libc::c_char,
            1376 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 55],
                &[libc::c_char; 55],
            >(b"struct config *Configlist_addbasis(struct rule *, int)\0"))
                .as_ptr(),
        );
    }
    if !currentend.is_null() {} else {
        __assert_fail(
            b"currentend!=0\0" as *const u8 as *const libc::c_char,
            b"/home/me/github/learnrust/libsql/tool/lemon.c\0" as *const u8
                as *const libc::c_char,
            1377 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 55],
                &[libc::c_char; 55],
            >(b"struct config *Configlist_addbasis(struct rule *, int)\0"))
                .as_ptr(),
        );
    }
    model.rp = rp;
    model.dot = dot;
    cfp = Configtable_find(&mut model);
    if cfp.is_null() {
        cfp = newconfig();
        let ref mut fresh38 = (*cfp).rp;
        *fresh38 = rp;
        (*cfp).dot = dot;
        let ref mut fresh39 = (*cfp).fws;
        *fresh39 = SetNew();
        let ref mut fresh40 = (*cfp).stp;
        *fresh40 = 0 as *mut state;
        let ref mut fresh41 = (*cfp).bplp;
        *fresh41 = 0 as *mut plink;
        let ref mut fresh42 = (*cfp).fplp;
        *fresh42 = *fresh41;
        let ref mut fresh43 = (*cfp).next;
        *fresh43 = 0 as *mut config;
        let ref mut fresh44 = (*cfp).bp;
        *fresh44 = 0 as *mut config;
        *currentend = cfp;
        currentend = &mut (*cfp).next;
        *basisend = cfp;
        basisend = &mut (*cfp).bp;
        Configtable_insert(cfp);
    }
    return cfp;
}
#[no_mangle]
pub unsafe extern "C" fn Configlist_closure(mut lemp: *mut lemon) {
    let mut cfp: *mut config = 0 as *mut config;
    let mut newcfp: *mut config = 0 as *mut config;
    let mut rp: *mut rule = 0 as *mut rule;
    let mut newrp: *mut rule = 0 as *mut rule;
    let mut sp: *mut symbol = 0 as *mut symbol;
    let mut xsp: *mut symbol = 0 as *mut symbol;
    let mut i: libc::c_int = 0;
    let mut dot: libc::c_int = 0;
    if !currentend.is_null() {} else {
        __assert_fail(
            b"currentend!=0\0" as *const u8 as *const libc::c_char,
            b"/home/me/github/learnrust/libsql/tool/lemon.c\0" as *const u8
                as *const libc::c_char,
            1407 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 40],
                &[libc::c_char; 40],
            >(b"void Configlist_closure(struct lemon *)\0"))
                .as_ptr(),
        );
    }
    cfp = current;
    while !cfp.is_null() {
        rp = (*cfp).rp;
        dot = (*cfp).dot;
        if !(dot >= (*rp).nrhs) {
            sp = *((*rp).rhs).offset(dot as isize);
            if (*sp).type_0 as libc::c_uint == NONTERMINAL as libc::c_int as libc::c_uint
            {
                if ((*sp).rule).is_null() && sp != (*lemp).errsym {
                    ErrorMsg(
                        (*lemp).filename,
                        (*rp).line,
                        b"Nonterminal \"%s\" has no rules.\0" as *const u8
                            as *const libc::c_char,
                        (*sp).name,
                    );
                    let ref mut fresh45 = (*lemp).errorcnt;
                    *fresh45 += 1;
                }
                newrp = (*sp).rule;
                while !newrp.is_null() {
                    newcfp = Configlist_add(newrp, 0 as libc::c_int);
                    i = dot + 1 as libc::c_int;
                    while i < (*rp).nrhs {
                        xsp = *((*rp).rhs).offset(i as isize);
                        if (*xsp).type_0 as libc::c_uint
                            == TERMINAL as libc::c_int as libc::c_uint
                        {
                            SetAdd((*newcfp).fws, (*xsp).index);
                            break;
                        } else if (*xsp).type_0 as libc::c_uint
                            == MULTITERMINAL as libc::c_int as libc::c_uint
                        {
                            let mut k: libc::c_int = 0;
                            k = 0 as libc::c_int;
                            while k < (*xsp).nsubsym {
                                SetAdd(
                                    (*newcfp).fws,
                                    (**((*xsp).subsym).offset(k as isize)).index,
                                );
                                k += 1;
                            }
                            break;
                        } else {
                            SetUnion((*newcfp).fws, (*xsp).firstset);
                            if (*xsp).lambda as libc::c_uint
                                == LEMON_FALSE as libc::c_int as libc::c_uint
                            {
                                break;
                            }
                            i += 1;
                        }
                    }
                    if i == (*rp).nrhs {
                        Plink_add(&mut (*cfp).fplp, newcfp);
                    }
                    newrp = (*newrp).nextlhs;
                }
            }
        }
        cfp = (*cfp).next;
    }
}
#[no_mangle]
pub unsafe extern "C" fn Configlist_sort() {
    current = msort(
        current as *mut libc::c_char,
        &mut (*current).next as *mut *mut config as *mut *mut libc::c_char,
        Some(
            Configcmp
                as unsafe extern "C" fn(
                    *const libc::c_char,
                    *const libc::c_char,
                ) -> libc::c_int,
        ),
    ) as *mut config;
    currentend = 0 as *mut *mut config;
}
#[no_mangle]
pub unsafe extern "C" fn Configlist_sortbasis() {
    basis = msort(
        current as *mut libc::c_char,
        &mut (*current).bp as *mut *mut config as *mut *mut libc::c_char,
        Some(
            Configcmp
                as unsafe extern "C" fn(
                    *const libc::c_char,
                    *const libc::c_char,
                ) -> libc::c_int,
        ),
    ) as *mut config;
    basisend = 0 as *mut *mut config;
}
#[no_mangle]
pub unsafe extern "C" fn Configlist_return() -> *mut config {
    let mut old: *mut config = 0 as *mut config;
    old = current;
    current = 0 as *mut config;
    currentend = 0 as *mut *mut config;
    return old;
}
#[no_mangle]
pub unsafe extern "C" fn Configlist_basis() -> *mut config {
    let mut old: *mut config = 0 as *mut config;
    old = basis;
    basis = 0 as *mut config;
    basisend = 0 as *mut *mut config;
    return old;
}
#[no_mangle]
pub unsafe extern "C" fn Configlist_eat(mut cfp: *mut config) {
    let mut nextcfp: *mut config = 0 as *mut config;
    while !cfp.is_null() {
        nextcfp = (*cfp).next;
        if ((*cfp).fplp).is_null() {} else {
            __assert_fail(
                b"cfp->fplp==0\0" as *const u8 as *const libc::c_char,
                b"/home/me/github/learnrust/libsql/tool/lemon.c\0" as *const u8
                    as *const libc::c_char,
                1486 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 37],
                    &[libc::c_char; 37],
                >(b"void Configlist_eat(struct config *)\0"))
                    .as_ptr(),
            );
        }
        if ((*cfp).bplp).is_null() {} else {
            __assert_fail(
                b"cfp->bplp==0\0" as *const u8 as *const libc::c_char,
                b"/home/me/github/learnrust/libsql/tool/lemon.c\0" as *const u8
                    as *const libc::c_char,
                1487 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 37],
                    &[libc::c_char; 37],
                >(b"void Configlist_eat(struct config *)\0"))
                    .as_ptr(),
            );
        }
        if !((*cfp).fws).is_null() {
            SetFree((*cfp).fws);
        }
        deleteconfig(cfp);
        cfp = nextcfp;
    }
}
#[no_mangle]
pub unsafe extern "C" fn ErrorMsg(
    mut filename: *const libc::c_char,
    mut lineno: libc::c_int,
    mut format: *const libc::c_char,
    mut args: ...
) {
    let mut ap: ::std::ffi::VaListImpl;
    fprintf(stderr, b"%s:%d: \0" as *const u8 as *const libc::c_char, filename, lineno);
    ap = args.clone();
    vfprintf(stderr, format, ap.as_va_list());
    fprintf(stderr, b"\n\0" as *const u8 as *const libc::c_char);
}
#[no_mangle]
pub unsafe extern "C" fn memory_error() {
    fprintf(
        stderr,
        b"Out of memory.  Aborting...\n\0" as *const u8 as *const libc::c_char,
    );
    exit(1 as libc::c_int);
}
static mut nDefine: libc::c_int = 0 as libc::c_int;
static mut azDefine: *mut *mut libc::c_char = 0 as *const *mut libc::c_char
    as *mut *mut libc::c_char;
unsafe extern "C" fn handle_D_option(mut z: *mut libc::c_char) {
    let mut paz: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    nDefine += 1;
    azDefine = realloc(
        azDefine as *mut libc::c_void,
        (::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong)
            .wrapping_mul(nDefine as libc::c_ulong),
    ) as *mut *mut libc::c_char;
    if azDefine.is_null() {
        fprintf(stderr, b"out of memory\n\0" as *const u8 as *const libc::c_char);
        exit(1 as libc::c_int);
    }
    paz = &mut *azDefine.offset((nDefine - 1 as libc::c_int) as isize)
        as *mut *mut libc::c_char;
    *paz = malloc((strlen(z) as libc::c_int + 1 as libc::c_int) as libc::c_ulong)
        as *mut libc::c_char;
    if (*paz).is_null() {
        fprintf(stderr, b"out of memory\n\0" as *const u8 as *const libc::c_char);
        exit(1 as libc::c_int);
    }
    lemon_strcpy(*paz, z);
    z = *paz;
    while *z as libc::c_int != 0 && *z as libc::c_int != '=' as i32 {
        z = z.offset(1);
    }
    *z = 0 as libc::c_int as libc::c_char;
}
static mut outputDir: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
unsafe extern "C" fn handle_d_option(mut z: *mut libc::c_char) {
    outputDir = malloc((strlen(z) as libc::c_int + 1 as libc::c_int) as libc::c_ulong)
        as *mut libc::c_char;
    if outputDir.is_null() {
        fprintf(stderr, b"out of memory\n\0" as *const u8 as *const libc::c_char);
        exit(1 as libc::c_int);
    }
    lemon_strcpy(outputDir, z);
}
static mut user_templatename: *mut libc::c_char = 0 as *const libc::c_char
    as *mut libc::c_char;
unsafe extern "C" fn handle_T_option(mut z: *mut libc::c_char) {
    user_templatename = malloc(
        (strlen(z) as libc::c_int + 1 as libc::c_int) as libc::c_ulong,
    ) as *mut libc::c_char;
    if user_templatename.is_null() {
        memory_error();
    }
    lemon_strcpy(user_templatename, z);
}
unsafe extern "C" fn Rule_merge(mut pA: *mut rule, mut pB: *mut rule) -> *mut rule {
    let mut pFirst: *mut rule = 0 as *mut rule;
    let mut ppPrev: *mut *mut rule = &mut pFirst;
    while !pA.is_null() && !pB.is_null() {
        if (*pA).iRule < (*pB).iRule {
            *ppPrev = pA;
            ppPrev = &mut (*pA).next;
            pA = (*pA).next;
        } else {
            *ppPrev = pB;
            ppPrev = &mut (*pB).next;
            pB = (*pB).next;
        }
    }
    if !pA.is_null() {
        *ppPrev = pA;
    } else {
        *ppPrev = pB;
    }
    return pFirst;
}
unsafe extern "C" fn Rule_sort(mut rp: *mut rule) -> *mut rule {
    let mut i: libc::c_uint = 0;
    let mut pNext: *mut rule = 0 as *mut rule;
    let mut x: [*mut rule; 32] = [0 as *mut rule; 32];
    memset(
        x.as_mut_ptr() as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<[*mut rule; 32]>() as libc::c_ulong,
    );
    while !rp.is_null() {
        pNext = (*rp).next;
        let ref mut fresh46 = (*rp).next;
        *fresh46 = 0 as *mut rule;
        i = 0 as libc::c_int as libc::c_uint;
        while (i as libc::c_ulong)
            < (::std::mem::size_of::<[*mut rule; 32]>() as libc::c_ulong)
                .wrapping_div(::std::mem::size_of::<*mut rule>() as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong)
            && !(x[i as usize]).is_null()
        {
            rp = Rule_merge(x[i as usize], rp);
            x[i as usize] = 0 as *mut rule;
            i = i.wrapping_add(1);
        }
        x[i as usize] = rp;
        rp = pNext;
    }
    rp = 0 as *mut rule;
    i = 0 as libc::c_int as libc::c_uint;
    while (i as libc::c_ulong)
        < (::std::mem::size_of::<[*mut rule; 32]>() as libc::c_ulong)
            .wrapping_div(::std::mem::size_of::<*mut rule>() as libc::c_ulong)
    {
        rp = Rule_merge(x[i as usize], rp);
        i = i.wrapping_add(1);
    }
    return rp;
}
unsafe extern "C" fn stats_line(
    mut zLabel: *const libc::c_char,
    mut iValue: libc::c_int,
) {
    let mut nLabel: libc::c_int = strlen(zLabel) as libc::c_int;
    printf(
        b"  %s%.*s %5d\n\0" as *const u8 as *const libc::c_char,
        zLabel,
        35 as libc::c_int - nLabel,
        b"................................\0" as *const u8 as *const libc::c_char,
        iValue,
    );
}
unsafe fn main_0(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
    static mut version: libc::c_int = 0 as libc::c_int;
    static mut rpflag: libc::c_int = 0 as libc::c_int;
    static mut basisflag: libc::c_int = 0 as libc::c_int;
    static mut compress: libc::c_int = 0 as libc::c_int;
    static mut quiet: libc::c_int = 0 as libc::c_int;
    static mut statistics: libc::c_int = 0 as libc::c_int;
    static mut mhflag: libc::c_int = 0 as libc::c_int;
    static mut nolinenosflag: libc::c_int = 0 as libc::c_int;
    static mut noResort: libc::c_int = 0 as libc::c_int;
    static mut sqlFlag: libc::c_int = 0 as libc::c_int;
    static mut printPP: libc::c_int = 0 as libc::c_int;
    static mut options: [s_options; 20] = unsafe {
        [
            {
                let mut init = s_options {
                    type_0: OPT_FLAG,
                    label: b"b\0" as *const u8 as *const libc::c_char,
                    arg: &basisflag as *const libc::c_int as *mut libc::c_int
                        as *mut libc::c_char,
                    message: b"Print only the basis in report.\0" as *const u8
                        as *const libc::c_char,
                };
                init
            },
            {
                let mut init = s_options {
                    type_0: OPT_FLAG,
                    label: b"c\0" as *const u8 as *const libc::c_char,
                    arg: &compress as *const libc::c_int as *mut libc::c_int
                        as *mut libc::c_char,
                    message: b"Don't compress the action table.\0" as *const u8
                        as *const libc::c_char,
                };
                init
            },
            {
                let mut init = s_options {
                    type_0: OPT_FSTR,
                    label: b"d\0" as *const u8 as *const libc::c_char,
                    arg: ::std::mem::transmute::<
                        Option::<unsafe extern "C" fn(*mut libc::c_char) -> ()>,
                        *mut libc::c_char,
                    >(
                        Some(
                            handle_d_option
                                as unsafe extern "C" fn(*mut libc::c_char) -> (),
                        ),
                    ),
                    message: b"Output directory.  Default '.'\0" as *const u8
                        as *const libc::c_char,
                };
                init
            },
            {
                let mut init = s_options {
                    type_0: OPT_FSTR,
                    label: b"D\0" as *const u8 as *const libc::c_char,
                    arg: ::std::mem::transmute::<
                        Option::<unsafe extern "C" fn(*mut libc::c_char) -> ()>,
                        *mut libc::c_char,
                    >(
                        Some(
                            handle_D_option
                                as unsafe extern "C" fn(*mut libc::c_char) -> (),
                        ),
                    ),
                    message: b"Define an %ifdef macro.\0" as *const u8
                        as *const libc::c_char,
                };
                init
            },
            {
                let mut init = s_options {
                    type_0: OPT_FLAG,
                    label: b"E\0" as *const u8 as *const libc::c_char,
                    arg: &printPP as *const libc::c_int as *mut libc::c_int
                        as *mut libc::c_char,
                    message: b"Print input file after preprocessing.\0" as *const u8
                        as *const libc::c_char,
                };
                init
            },
            {
                let mut init = s_options {
                    type_0: OPT_FSTR,
                    label: b"f\0" as *const u8 as *const libc::c_char,
                    arg: 0 as *const libc::c_char as *mut libc::c_char,
                    message: b"Ignored.  (Placeholder for -f compiler options.)\0"
                        as *const u8 as *const libc::c_char,
                };
                init
            },
            {
                let mut init = s_options {
                    type_0: OPT_FLAG,
                    label: b"g\0" as *const u8 as *const libc::c_char,
                    arg: &rpflag as *const libc::c_int as *mut libc::c_int
                        as *mut libc::c_char,
                    message: b"Print grammar without actions.\0" as *const u8
                        as *const libc::c_char,
                };
                init
            },
            {
                let mut init = s_options {
                    type_0: OPT_FSTR,
                    label: b"I\0" as *const u8 as *const libc::c_char,
                    arg: 0 as *const libc::c_char as *mut libc::c_char,
                    message: b"Ignored.  (Placeholder for '-I' compiler options.)\0"
                        as *const u8 as *const libc::c_char,
                };
                init
            },
            {
                let mut init = s_options {
                    type_0: OPT_FLAG,
                    label: b"m\0" as *const u8 as *const libc::c_char,
                    arg: &mhflag as *const libc::c_int as *mut libc::c_int
                        as *mut libc::c_char,
                    message: b"Output a makeheaders compatible file.\0" as *const u8
                        as *const libc::c_char,
                };
                init
            },
            {
                let mut init = s_options {
                    type_0: OPT_FLAG,
                    label: b"l\0" as *const u8 as *const libc::c_char,
                    arg: &nolinenosflag as *const libc::c_int as *mut libc::c_int
                        as *mut libc::c_char,
                    message: b"Do not print #line statements.\0" as *const u8
                        as *const libc::c_char,
                };
                init
            },
            {
                let mut init = s_options {
                    type_0: OPT_FSTR,
                    label: b"O\0" as *const u8 as *const libc::c_char,
                    arg: 0 as *const libc::c_char as *mut libc::c_char,
                    message: b"Ignored.  (Placeholder for '-O' compiler options.)\0"
                        as *const u8 as *const libc::c_char,
                };
                init
            },
            {
                let mut init = s_options {
                    type_0: OPT_FLAG,
                    label: b"p\0" as *const u8 as *const libc::c_char,
                    arg: &showPrecedenceConflict as *const libc::c_int
                        as *mut libc::c_int as *mut libc::c_char,
                    message: b"Show conflicts resolved by precedence rules\0"
                        as *const u8 as *const libc::c_char,
                };
                init
            },
            {
                let mut init = s_options {
                    type_0: OPT_FLAG,
                    label: b"q\0" as *const u8 as *const libc::c_char,
                    arg: &quiet as *const libc::c_int as *mut libc::c_int
                        as *mut libc::c_char,
                    message: b"(Quiet) Don't print the report file.\0" as *const u8
                        as *const libc::c_char,
                };
                init
            },
            {
                let mut init = s_options {
                    type_0: OPT_FLAG,
                    label: b"r\0" as *const u8 as *const libc::c_char,
                    arg: &noResort as *const libc::c_int as *mut libc::c_int
                        as *mut libc::c_char,
                    message: b"Do not sort or renumber states\0" as *const u8
                        as *const libc::c_char,
                };
                init
            },
            {
                let mut init = s_options {
                    type_0: OPT_FLAG,
                    label: b"s\0" as *const u8 as *const libc::c_char,
                    arg: &statistics as *const libc::c_int as *mut libc::c_int
                        as *mut libc::c_char,
                    message: b"Print parser stats to standard output.\0" as *const u8
                        as *const libc::c_char,
                };
                init
            },
            {
                let mut init = s_options {
                    type_0: OPT_FLAG,
                    label: b"S\0" as *const u8 as *const libc::c_char,
                    arg: &sqlFlag as *const libc::c_int as *mut libc::c_int
                        as *mut libc::c_char,
                    message: b"Generate the *.sql file describing the parser tables.\0"
                        as *const u8 as *const libc::c_char,
                };
                init
            },
            {
                let mut init = s_options {
                    type_0: OPT_FLAG,
                    label: b"x\0" as *const u8 as *const libc::c_char,
                    arg: &version as *const libc::c_int as *mut libc::c_int
                        as *mut libc::c_char,
                    message: b"Print the version number.\0" as *const u8
                        as *const libc::c_char,
                };
                init
            },
            {
                let mut init = s_options {
                    type_0: OPT_FSTR,
                    label: b"T\0" as *const u8 as *const libc::c_char,
                    arg: ::std::mem::transmute::<
                        Option::<unsafe extern "C" fn(*mut libc::c_char) -> ()>,
                        *mut libc::c_char,
                    >(
                        Some(
                            handle_T_option
                                as unsafe extern "C" fn(*mut libc::c_char) -> (),
                        ),
                    ),
                    message: b"Specify a template file.\0" as *const u8
                        as *const libc::c_char,
                };
                init
            },
            {
                let mut init = s_options {
                    type_0: OPT_FSTR,
                    label: b"W\0" as *const u8 as *const libc::c_char,
                    arg: 0 as *const libc::c_char as *mut libc::c_char,
                    message: b"Ignored.  (Placeholder for '-W' compiler options.)\0"
                        as *const u8 as *const libc::c_char,
                };
                init
            },
            {
                let mut init = s_options {
                    type_0: OPT_FLAG,
                    label: 0 as *const libc::c_char,
                    arg: 0 as *const libc::c_char as *mut libc::c_char,
                    message: 0 as *const libc::c_char,
                };
                init
            },
        ]
    };
    let mut i: libc::c_int = 0;
    let mut exitcode: libc::c_int = 0;
    let mut lem: lemon = lemon {
        sorted: 0 as *mut *mut state,
        rule: 0 as *mut rule,
        startRule: 0 as *mut rule,
        nstate: 0,
        nxstate: 0,
        nrule: 0,
        nruleWithAction: 0,
        nsymbol: 0,
        nterminal: 0,
        minShiftReduce: 0,
        errAction: 0,
        accAction: 0,
        noAction: 0,
        minReduce: 0,
        maxAction: 0,
        symbols: 0 as *mut *mut symbol,
        errorcnt: 0,
        errsym: 0 as *mut symbol,
        wildcard: 0 as *mut symbol,
        name: 0 as *mut libc::c_char,
        arg: 0 as *mut libc::c_char,
        ctx: 0 as *mut libc::c_char,
        tokentype: 0 as *mut libc::c_char,
        vartype: 0 as *mut libc::c_char,
        start: 0 as *mut libc::c_char,
        stacksize: 0 as *mut libc::c_char,
        include: 0 as *mut libc::c_char,
        error: 0 as *mut libc::c_char,
        overflow: 0 as *mut libc::c_char,
        failure: 0 as *mut libc::c_char,
        accept: 0 as *mut libc::c_char,
        extracode: 0 as *mut libc::c_char,
        tokendest: 0 as *mut libc::c_char,
        vardest: 0 as *mut libc::c_char,
        filename: 0 as *mut libc::c_char,
        outname: 0 as *mut libc::c_char,
        tokenprefix: 0 as *mut libc::c_char,
        nconflict: 0,
        nactiontab: 0,
        nlookaheadtab: 0,
        tablesize: 0,
        basisflag: 0,
        printPreprocessed: 0,
        has_fallback: 0,
        nolinenosflag: 0,
        argv0: 0 as *mut libc::c_char,
    };
    let mut rp: *mut rule = 0 as *mut rule;
    OptInit(argv, options.as_mut_ptr(), stderr);
    if version != 0 {
        printf(b"Lemon version 1.0\n\0" as *const u8 as *const libc::c_char);
        exit(0 as libc::c_int);
    }
    if OptNArgs() != 1 as libc::c_int {
        fprintf(
            stderr,
            b"Exactly one filename argument is required.\n\0" as *const u8
                as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    memset(
        &mut lem as *mut lemon as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<lemon>() as libc::c_ulong,
    );
    lem.errorcnt = 0 as libc::c_int;
    Strsafe_init();
    Symbol_init();
    State_init();
    lem.argv0 = *argv.offset(0 as libc::c_int as isize);
    lem.filename = OptArg(0 as libc::c_int);
    lem.basisflag = basisflag;
    lem.nolinenosflag = nolinenosflag;
    lem.printPreprocessed = printPP;
    Symbol_new(b"$\0" as *const u8 as *const libc::c_char);
    Parse(&mut lem);
    if lem.printPreprocessed != 0 || lem.errorcnt != 0 {
        exit(lem.errorcnt);
    }
    if lem.nrule == 0 as libc::c_int {
        fprintf(stderr, b"Empty grammar.\n\0" as *const u8 as *const libc::c_char);
        exit(1 as libc::c_int);
    }
    lem.errsym = Symbol_find(b"error\0" as *const u8 as *const libc::c_char);
    Symbol_new(b"{default}\0" as *const u8 as *const libc::c_char);
    lem.nsymbol = Symbol_count();
    lem.symbols = Symbol_arrayof();
    i = 0 as libc::c_int;
    while i < lem.nsymbol {
        (**(lem.symbols).offset(i as isize)).index = i;
        i += 1;
    }
    qsort(
        lem.symbols as *mut libc::c_void,
        lem.nsymbol as size_t,
        ::std::mem::size_of::<*mut symbol>() as libc::c_ulong,
        Some(
            Symbolcmpp
                as unsafe extern "C" fn(
                    *const libc::c_void,
                    *const libc::c_void,
                ) -> libc::c_int,
        ),
    );
    i = 0 as libc::c_int;
    while i < lem.nsymbol {
        (**(lem.symbols).offset(i as isize)).index = i;
        i += 1;
    }
    while (**(lem.symbols).offset((i - 1 as libc::c_int) as isize)).type_0
        as libc::c_uint == MULTITERMINAL as libc::c_int as libc::c_uint
    {
        i -= 1;
    }
    if strcmp(
        (**(lem.symbols).offset((i - 1 as libc::c_int) as isize)).name,
        b"{default}\0" as *const u8 as *const libc::c_char,
    ) == 0 as libc::c_int
    {} else {
        __assert_fail(
            b"strcmp(lem.symbols[i-1]->name,\"{default}\")==0\0" as *const u8
                as *const libc::c_char,
            b"/home/me/github/learnrust/libsql/tool/lemon.c\0" as *const u8
                as *const libc::c_char,
            1710 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 23],
                &[libc::c_char; 23],
            >(b"int main(int, char **)\0"))
                .as_ptr(),
        );
    }
    lem.nsymbol = i - 1 as libc::c_int;
    i = 1 as libc::c_int;
    while *(*__ctype_b_loc())
        .offset(
            *((**(lem.symbols).offset(i as isize)).name)
                .offset(0 as libc::c_int as isize) as libc::c_uchar as libc::c_int
                as isize,
        ) as libc::c_int & _ISupper as libc::c_int as libc::c_ushort as libc::c_int != 0
    {
        i += 1;
    }
    lem.nterminal = i;
    i = 0 as libc::c_int;
    rp = lem.rule;
    while !rp.is_null() {
        (*rp)
            .iRule = if !((*rp).code).is_null() {
            let fresh47 = i;
            i = i + 1;
            fresh47
        } else {
            -(1 as libc::c_int)
        };
        rp = (*rp).next;
    }
    lem.nruleWithAction = i;
    rp = lem.rule;
    while !rp.is_null() {
        if (*rp).iRule < 0 as libc::c_int {
            let fresh48 = i;
            i = i + 1;
            (*rp).iRule = fresh48;
        }
        rp = (*rp).next;
    }
    lem.startRule = lem.rule;
    lem.rule = Rule_sort(lem.rule);
    if rpflag != 0 {
        Reprint(&mut lem);
    } else {
        SetSize(lem.nterminal + 1 as libc::c_int);
        FindRulePrecedences(&mut lem);
        FindFirstSets(&mut lem);
        lem.nstate = 0 as libc::c_int;
        FindStates(&mut lem);
        lem.sorted = State_arrayof();
        FindLinks(&mut lem);
        FindFollowSets(&mut lem);
        FindActions(&mut lem);
        if compress == 0 as libc::c_int {
            CompressTables(&mut lem);
        }
        if noResort == 0 as libc::c_int {
            ResortStates(&mut lem);
        }
        if quiet == 0 {
            ReportOutput(&mut lem);
        }
        ReportTable(&mut lem, mhflag, sqlFlag);
        if mhflag == 0 {
            ReportHeader(&mut lem);
        }
    }
    if statistics != 0 {
        printf(b"Parser statistics:\n\0" as *const u8 as *const libc::c_char);
        stats_line(
            b"terminal symbols\0" as *const u8 as *const libc::c_char,
            lem.nterminal,
        );
        stats_line(
            b"non-terminal symbols\0" as *const u8 as *const libc::c_char,
            lem.nsymbol - lem.nterminal,
        );
        stats_line(b"total symbols\0" as *const u8 as *const libc::c_char, lem.nsymbol);
        stats_line(b"rules\0" as *const u8 as *const libc::c_char, lem.nrule);
        stats_line(b"states\0" as *const u8 as *const libc::c_char, lem.nxstate);
        stats_line(b"conflicts\0" as *const u8 as *const libc::c_char, lem.nconflict);
        stats_line(
            b"action table entries\0" as *const u8 as *const libc::c_char,
            lem.nactiontab,
        );
        stats_line(
            b"lookahead table entries\0" as *const u8 as *const libc::c_char,
            lem.nlookaheadtab,
        );
        stats_line(
            b"total table size (bytes)\0" as *const u8 as *const libc::c_char,
            lem.tablesize,
        );
    }
    if lem.nconflict > 0 as libc::c_int {
        fprintf(
            stderr,
            b"%d parsing conflicts.\n\0" as *const u8 as *const libc::c_char,
            lem.nconflict,
        );
    }
    exitcode = if lem.errorcnt > 0 as libc::c_int || lem.nconflict > 0 as libc::c_int {
        1 as libc::c_int
    } else {
        0 as libc::c_int
    };
    exit(exitcode);
}
unsafe extern "C" fn merge(
    mut a: *mut libc::c_char,
    mut b: *mut libc::c_char,
    mut cmp: Option::<
        unsafe extern "C" fn(*const libc::c_char, *const libc::c_char) -> libc::c_int,
    >,
    mut offset: libc::c_int,
) -> *mut libc::c_char {
    let mut ptr: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut head: *mut libc::c_char = 0 as *mut libc::c_char;
    if a.is_null() {
        head = b;
    } else if b.is_null() {
        head = a;
    } else {
        if (Some(cmp.expect("non-null function pointer")))
            .expect("non-null function pointer")(a, b) <= 0 as libc::c_int
        {
            ptr = a;
            a = *(a.offset(offset as isize) as *mut *mut libc::c_char);
        } else {
            ptr = b;
            b = *(b.offset(offset as isize) as *mut *mut libc::c_char);
        }
        head = ptr;
        while !a.is_null() && !b.is_null() {
            if (Some(cmp.expect("non-null function pointer")))
                .expect("non-null function pointer")(a, b) <= 0 as libc::c_int
            {
                let ref mut fresh49 = *(ptr.offset(offset as isize)
                    as *mut *mut libc::c_char);
                *fresh49 = a;
                ptr = a;
                a = *(a.offset(offset as isize) as *mut *mut libc::c_char);
            } else {
                let ref mut fresh50 = *(ptr.offset(offset as isize)
                    as *mut *mut libc::c_char);
                *fresh50 = b;
                ptr = b;
                b = *(b.offset(offset as isize) as *mut *mut libc::c_char);
            }
        }
        if !a.is_null() {
            let ref mut fresh51 = *(ptr.offset(offset as isize)
                as *mut *mut libc::c_char);
            *fresh51 = a;
        } else {
            let ref mut fresh52 = *(ptr.offset(offset as isize)
                as *mut *mut libc::c_char);
            *fresh52 = b;
        }
    }
    return head;
}
unsafe extern "C" fn msort(
    mut list: *mut libc::c_char,
    mut next: *mut *mut libc::c_char,
    mut cmp: Option::<
        unsafe extern "C" fn(*const libc::c_char, *const libc::c_char) -> libc::c_int,
    >,
) -> *mut libc::c_char {
    let mut offset: libc::c_ulong = 0;
    let mut ep: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut set: [*mut libc::c_char; 30] = [0 as *mut libc::c_char; 30];
    let mut i: libc::c_int = 0;
    offset = (next as *mut libc::c_char).offset_from(list) as libc::c_long
        as libc::c_ulong;
    i = 0 as libc::c_int;
    while i < 30 as libc::c_int {
        set[i as usize] = 0 as *mut libc::c_char;
        i += 1;
    }
    while !list.is_null() {
        ep = list;
        list = *(list.offset(offset as isize) as *mut *mut libc::c_char);
        let ref mut fresh53 = *(ep.offset(offset as isize) as *mut *mut libc::c_char);
        *fresh53 = 0 as *mut libc::c_char;
        i = 0 as libc::c_int;
        while i < 30 as libc::c_int - 1 as libc::c_int && !(set[i as usize]).is_null() {
            ep = merge(ep, set[i as usize], cmp, offset as libc::c_int);
            set[i as usize] = 0 as *mut libc::c_char;
            i += 1;
        }
        set[i as usize] = ep;
    }
    ep = 0 as *mut libc::c_char;
    i = 0 as libc::c_int;
    while i < 30 as libc::c_int {
        if !(set[i as usize]).is_null() {
            ep = merge(set[i as usize], ep, cmp, offset as libc::c_int);
        }
        i += 1;
    }
    return ep;
}
static mut g_argv: *mut *mut libc::c_char = 0 as *const *mut libc::c_char
    as *mut *mut libc::c_char;
static mut op: *mut s_options = 0 as *const s_options as *mut s_options;
static mut errstream: *mut FILE = 0 as *const FILE as *mut FILE;
unsafe extern "C" fn errline(
    mut n: libc::c_int,
    mut k: libc::c_int,
    mut err: *mut FILE,
) {
    let mut spcnt: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    if !(*g_argv.offset(0 as libc::c_int as isize)).is_null() {
        fprintf(
            err,
            b"%s\0" as *const u8 as *const libc::c_char,
            *g_argv.offset(0 as libc::c_int as isize),
        );
        spcnt = strlen(*g_argv.offset(0 as libc::c_int as isize)) as libc::c_int
            + 1 as libc::c_int;
    } else {
        spcnt = 0 as libc::c_int;
    }
    i = 1 as libc::c_int;
    while i < n && !(*g_argv.offset(i as isize)).is_null() {
        fprintf(
            err,
            b" %s\0" as *const u8 as *const libc::c_char,
            *g_argv.offset(i as isize),
        );
        spcnt += strlen(*g_argv.offset(i as isize)) as libc::c_int + 1 as libc::c_int;
        i += 1;
    }
    spcnt += k;
    while !(*g_argv.offset(i as isize)).is_null() {
        fprintf(
            err,
            b" %s\0" as *const u8 as *const libc::c_char,
            *g_argv.offset(i as isize),
        );
        i += 1;
    }
    if spcnt < 20 as libc::c_int {
        fprintf(
            err,
            b"\n%*s^-- here\n\0" as *const u8 as *const libc::c_char,
            spcnt,
            b"\0" as *const u8 as *const libc::c_char,
        );
    } else {
        fprintf(
            err,
            b"\n%*shere --^\n\0" as *const u8 as *const libc::c_char,
            spcnt - 7 as libc::c_int,
            b"\0" as *const u8 as *const libc::c_char,
        );
    };
}
unsafe extern "C" fn argindex(mut n: libc::c_int) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut dashdash: libc::c_int = 0 as libc::c_int;
    if !g_argv.is_null() && !(*g_argv).is_null() {
        i = 1 as libc::c_int;
        while !(*g_argv.offset(i as isize)).is_null() {
            if dashdash != 0
                || !(*(*g_argv.offset(i as isize)).offset(0 as libc::c_int as isize)
                    as libc::c_int == '-' as i32
                    || *(*g_argv.offset(i as isize)).offset(0 as libc::c_int as isize)
                        as libc::c_int == '+' as i32
                    || !(strchr(*g_argv.offset(i as isize), '=' as i32)).is_null())
            {
                if n == 0 as libc::c_int {
                    return i;
                }
                n -= 1;
            }
            if strcmp(
                *g_argv.offset(i as isize),
                b"--\0" as *const u8 as *const libc::c_char,
            ) == 0 as libc::c_int
            {
                dashdash = 1 as libc::c_int;
            }
            i += 1;
        }
    }
    return -(1 as libc::c_int);
}
static mut emsg: [libc::c_char; 28] = unsafe {
    *::std::mem::transmute::<
        &[u8; 28],
        &mut [libc::c_char; 28],
    >(b"Command line syntax error: \0")
};
unsafe extern "C" fn handleflags(mut i: libc::c_int, mut err: *mut FILE) -> libc::c_int {
    let mut v: libc::c_int = 0;
    let mut errcnt: libc::c_int = 0 as libc::c_int;
    let mut j: libc::c_int = 0;
    j = 0 as libc::c_int;
    while !((*op.offset(j as isize)).label).is_null() {
        if strncmp(
            &mut *(*g_argv.offset(i as isize)).offset(1 as libc::c_int as isize),
            (*op.offset(j as isize)).label,
            strlen((*op.offset(j as isize)).label) as libc::c_int as libc::c_ulong,
        ) == 0 as libc::c_int
        {
            break;
        }
        j += 1;
    }
    v = if *(*g_argv.offset(i as isize)).offset(0 as libc::c_int as isize) as libc::c_int
        == '-' as i32
    {
        1 as libc::c_int
    } else {
        0 as libc::c_int
    };
    if ((*op.offset(j as isize)).label).is_null() {
        if !err.is_null() {
            fprintf(
                err,
                b"%sundefined option.\n\0" as *const u8 as *const libc::c_char,
                emsg.as_mut_ptr(),
            );
            errline(i, 1 as libc::c_int, err);
        }
        errcnt += 1;
    } else if !((*op.offset(j as isize)).arg).is_null() {
        if (*op.offset(j as isize)).type_0 as libc::c_uint
            == OPT_FLAG as libc::c_int as libc::c_uint
        {
            *((*op.offset(j as isize)).arg as *mut libc::c_int) = v;
        } else if (*op.offset(j as isize)).type_0 as libc::c_uint
            == OPT_FFLAG as libc::c_int as libc::c_uint
        {
            (Some(
                (::std::mem::transmute::<
                    *mut libc::c_char,
                    Option::<unsafe extern "C" fn(libc::c_int) -> ()>,
                >((*op.offset(j as isize)).arg))
                    .expect("non-null function pointer"),
            ))
                .expect("non-null function pointer")(v);
        } else if (*op.offset(j as isize)).type_0 as libc::c_uint
            == OPT_FSTR as libc::c_int as libc::c_uint
        {
            (Some(
                (::std::mem::transmute::<
                    *mut libc::c_char,
                    Option::<unsafe extern "C" fn(*mut libc::c_char) -> ()>,
                >((*op.offset(j as isize)).arg))
                    .expect("non-null function pointer"),
            ))
                .expect(
                    "non-null function pointer",
                )(&mut *(*g_argv.offset(i as isize)).offset(2 as libc::c_int as isize));
        } else {
            if !err.is_null() {
                fprintf(
                    err,
                    b"%smissing argument on switch.\n\0" as *const u8
                        as *const libc::c_char,
                    emsg.as_mut_ptr(),
                );
                errline(i, 1 as libc::c_int, err);
            }
            errcnt += 1;
        }
    }
    return errcnt;
}
unsafe extern "C" fn handleswitch(
    mut i: libc::c_int,
    mut err: *mut FILE,
) -> libc::c_int {
    let mut lv: libc::c_int = 0 as libc::c_int;
    let mut dv: libc::c_double = 0.0f64;
    let mut sv: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut end: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut cp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut j: libc::c_int = 0;
    let mut errcnt: libc::c_int = 0 as libc::c_int;
    cp = strchr(*g_argv.offset(i as isize), '=' as i32);
    if !cp.is_null() {} else {
        __assert_fail(
            b"cp!=0\0" as *const u8 as *const libc::c_char,
            b"/home/me/github/learnrust/libsql/tool/lemon.c\0" as *const u8
                as *const libc::c_char,
            2022 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 30],
                &[libc::c_char; 30],
            >(b"int handleswitch(int, FILE *)\0"))
                .as_ptr(),
        );
    }
    *cp = 0 as libc::c_int as libc::c_char;
    j = 0 as libc::c_int;
    while !((*op.offset(j as isize)).label).is_null() {
        if strcmp(*g_argv.offset(i as isize), (*op.offset(j as isize)).label)
            == 0 as libc::c_int
        {
            break;
        }
        j += 1;
    }
    *cp = '=' as i32 as libc::c_char;
    if ((*op.offset(j as isize)).label).is_null() {
        if !err.is_null() {
            fprintf(
                err,
                b"%sundefined option.\n\0" as *const u8 as *const libc::c_char,
                emsg.as_mut_ptr(),
            );
            errline(i, 0 as libc::c_int, err);
        }
        errcnt += 1;
    } else {
        cp = cp.offset(1);
        match (*op.offset(j as isize)).type_0 as libc::c_uint {
            1 | 5 => {
                if !err.is_null() {
                    fprintf(
                        err,
                        b"%soption requires an argument.\n\0" as *const u8
                            as *const libc::c_char,
                        emsg.as_mut_ptr(),
                    );
                    errline(i, 0 as libc::c_int, err);
                }
                errcnt += 1;
            }
            3 | 7 => {
                dv = strtod(cp, &mut end);
                if *end != 0 {
                    if !err.is_null() {
                        fprintf(
                            err,
                            b"%sillegal character in floating-point argument.\n\0"
                                as *const u8 as *const libc::c_char,
                            emsg.as_mut_ptr(),
                        );
                        errline(
                            i,
                            end.offset_from(*g_argv.offset(i as isize)) as libc::c_long
                                as libc::c_int,
                            err,
                        );
                    }
                    errcnt += 1;
                }
            }
            2 | 6 => {
                lv = strtol(cp, &mut end, 0 as libc::c_int) as libc::c_int;
                if *end != 0 {
                    if !err.is_null() {
                        fprintf(
                            err,
                            b"%sillegal character in integer argument.\n\0" as *const u8
                                as *const libc::c_char,
                            emsg.as_mut_ptr(),
                        );
                        errline(
                            i,
                            end.offset_from(*g_argv.offset(i as isize)) as libc::c_long
                                as libc::c_int,
                            err,
                        );
                    }
                    errcnt += 1;
                }
            }
            4 | 8 => {
                sv = cp;
            }
            _ => {}
        }
        match (*op.offset(j as isize)).type_0 as libc::c_uint {
            3 => {
                *((*op.offset(j as isize)).arg as *mut libc::c_double) = dv;
            }
            7 => {
                (Some(
                    (::std::mem::transmute::<
                        *mut libc::c_char,
                        Option::<unsafe extern "C" fn(libc::c_double) -> ()>,
                    >((*op.offset(j as isize)).arg))
                        .expect("non-null function pointer"),
                ))
                    .expect("non-null function pointer")(dv);
            }
            2 => {
                *((*op.offset(j as isize)).arg as *mut libc::c_int) = lv;
            }
            6 => {
                (Some(
                    (::std::mem::transmute::<
                        *mut libc::c_char,
                        Option::<unsafe extern "C" fn(libc::c_int) -> ()>,
                    >((*op.offset(j as isize)).arg))
                        .expect("non-null function pointer"),
                ))
                    .expect("non-null function pointer")(lv);
            }
            4 => {
                let ref mut fresh54 = *((*op.offset(j as isize)).arg
                    as *mut *mut libc::c_char);
                *fresh54 = sv;
            }
            8 => {
                (Some(
                    (::std::mem::transmute::<
                        *mut libc::c_char,
                        Option::<unsafe extern "C" fn(*mut libc::c_char) -> ()>,
                    >((*op.offset(j as isize)).arg))
                        .expect("non-null function pointer"),
                ))
                    .expect("non-null function pointer")(sv);
            }
            1 | 5 | _ => {}
        }
    }
    return errcnt;
}
#[no_mangle]
pub unsafe extern "C" fn OptInit(
    mut a: *mut *mut libc::c_char,
    mut o: *mut s_options,
    mut err: *mut FILE,
) -> libc::c_int {
    let mut errcnt: libc::c_int = 0 as libc::c_int;
    g_argv = a;
    op = o;
    errstream = err;
    if !g_argv.is_null() && !(*g_argv).is_null() && !op.is_null() {
        let mut i: libc::c_int = 0;
        i = 1 as libc::c_int;
        while !(*g_argv.offset(i as isize)).is_null() {
            if *(*g_argv.offset(i as isize)).offset(0 as libc::c_int as isize)
                as libc::c_int == '+' as i32
                || *(*g_argv.offset(i as isize)).offset(0 as libc::c_int as isize)
                    as libc::c_int == '-' as i32
            {
                errcnt += handleflags(i, err);
            } else if !(strchr(*g_argv.offset(i as isize), '=' as i32)).is_null() {
                errcnt += handleswitch(i, err);
            }
            i += 1;
        }
    }
    if errcnt > 0 as libc::c_int {
        fprintf(
            err,
            b"Valid command line options for \"%s\" are:\n\0" as *const u8
                as *const libc::c_char,
            *a,
        );
        OptPrint();
        exit(1 as libc::c_int);
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn OptNArgs() -> libc::c_int {
    let mut cnt: libc::c_int = 0 as libc::c_int;
    let mut dashdash: libc::c_int = 0 as libc::c_int;
    let mut i: libc::c_int = 0;
    if !g_argv.is_null() && !(*g_argv.offset(0 as libc::c_int as isize)).is_null() {
        i = 1 as libc::c_int;
        while !(*g_argv.offset(i as isize)).is_null() {
            if dashdash != 0
                || !(*(*g_argv.offset(i as isize)).offset(0 as libc::c_int as isize)
                    as libc::c_int == '-' as i32
                    || *(*g_argv.offset(i as isize)).offset(0 as libc::c_int as isize)
                        as libc::c_int == '+' as i32
                    || !(strchr(*g_argv.offset(i as isize), '=' as i32)).is_null())
            {
                cnt += 1;
            }
            if strcmp(
                *g_argv.offset(i as isize),
                b"--\0" as *const u8 as *const libc::c_char,
            ) == 0 as libc::c_int
            {
                dashdash = 1 as libc::c_int;
            }
            i += 1;
        }
    }
    return cnt;
}
#[no_mangle]
pub unsafe extern "C" fn OptArg(mut n: libc::c_int) -> *mut libc::c_char {
    let mut i: libc::c_int = 0;
    i = argindex(n);
    return if i >= 0 as libc::c_int {
        *g_argv.offset(i as isize)
    } else {
        0 as *mut libc::c_char
    };
}
#[no_mangle]
pub unsafe extern "C" fn OptErr(mut n: libc::c_int) {
    let mut i: libc::c_int = 0;
    i = argindex(n);
    if i >= 0 as libc::c_int {
        errline(i, 0 as libc::c_int, errstream);
    }
}
#[no_mangle]
pub unsafe extern "C" fn OptPrint() {
    let mut i: libc::c_int = 0;
    let mut max: libc::c_int = 0;
    let mut len: libc::c_int = 0;
    max = 0 as libc::c_int;
    i = 0 as libc::c_int;
    while !((*op.offset(i as isize)).label).is_null() {
        len = strlen((*op.offset(i as isize)).label) as libc::c_int + 1 as libc::c_int;
        match (*op.offset(i as isize)).type_0 as libc::c_uint {
            2 | 6 => {
                len += 9 as libc::c_int;
            }
            3 | 7 => {
                len += 6 as libc::c_int;
            }
            4 | 8 => {
                len += 8 as libc::c_int;
            }
            1 | 5 | _ => {}
        }
        if len > max {
            max = len;
        }
        i += 1;
    }
    i = 0 as libc::c_int;
    while !((*op.offset(i as isize)).label).is_null() {
        match (*op.offset(i as isize)).type_0 as libc::c_uint {
            1 | 5 => {
                fprintf(
                    errstream,
                    b"  -%-*s  %s\n\0" as *const u8 as *const libc::c_char,
                    max,
                    (*op.offset(i as isize)).label,
                    (*op.offset(i as isize)).message,
                );
            }
            2 | 6 => {
                fprintf(
                    errstream,
                    b"  -%s<integer>%*s  %s\n\0" as *const u8 as *const libc::c_char,
                    (*op.offset(i as isize)).label,
                    max - strlen((*op.offset(i as isize)).label) as libc::c_int
                        - 9 as libc::c_int,
                    b"\0" as *const u8 as *const libc::c_char,
                    (*op.offset(i as isize)).message,
                );
            }
            3 | 7 => {
                fprintf(
                    errstream,
                    b"  -%s<real>%*s  %s\n\0" as *const u8 as *const libc::c_char,
                    (*op.offset(i as isize)).label,
                    max - strlen((*op.offset(i as isize)).label) as libc::c_int
                        - 6 as libc::c_int,
                    b"\0" as *const u8 as *const libc::c_char,
                    (*op.offset(i as isize)).message,
                );
            }
            4 | 8 => {
                fprintf(
                    errstream,
                    b"  -%s<string>%*s  %s\n\0" as *const u8 as *const libc::c_char,
                    (*op.offset(i as isize)).label,
                    max - strlen((*op.offset(i as isize)).label) as libc::c_int
                        - 8 as libc::c_int,
                    b"\0" as *const u8 as *const libc::c_char,
                    (*op.offset(i as isize)).message,
                );
            }
            _ => {}
        }
        i += 1;
    }
}
unsafe extern "C" fn parseonetoken(mut psp: *mut pstate) {
    let mut x: *const libc::c_char = 0 as *const libc::c_char;
    x = Strsafe((*psp).tokenstart);
    let mut current_block_454: u64;
    match (*psp).state as libc::c_uint {
        0 => {
            let ref mut fresh55 = (*psp).prevrule;
            *fresh55 = 0 as *mut rule;
            (*psp).preccounter = 0 as libc::c_int;
            let ref mut fresh56 = (*psp).lastrule;
            *fresh56 = 0 as *mut rule;
            let ref mut fresh57 = (*psp).firstrule;
            *fresh57 = *fresh56;
            (*(*psp).gp).nrule = 0 as libc::c_int;
            current_block_454 = 3403847451326750966;
        }
        1 => {
            current_block_454 = 3403847451326750966;
        }
        12 => {
            if *(*__ctype_b_loc())
                .offset(
                    *x.offset(0 as libc::c_int as isize) as libc::c_uchar as libc::c_int
                        as isize,
                ) as libc::c_int
                & _ISupper as libc::c_int as libc::c_ushort as libc::c_int == 0
            {
                ErrorMsg(
                    (*psp).filename,
                    (*psp).tokenlineno,
                    b"The precedence symbol must be a terminal.\0" as *const u8
                        as *const libc::c_char,
                );
                let ref mut fresh64 = (*psp).errorcnt;
                *fresh64 += 1;
            } else if ((*psp).prevrule).is_null() {
                ErrorMsg(
                    (*psp).filename,
                    (*psp).tokenlineno,
                    b"There is no prior rule to assign precedence \"[%s]\".\0"
                        as *const u8 as *const libc::c_char,
                    x,
                );
                let ref mut fresh65 = (*psp).errorcnt;
                *fresh65 += 1;
            } else if !((*(*psp).prevrule).precsym).is_null() {
                ErrorMsg(
                    (*psp).filename,
                    (*psp).tokenlineno,
                    b"Precedence mark on this line is not the first to follow the previous rule.\0"
                        as *const u8 as *const libc::c_char,
                );
                let ref mut fresh66 = (*psp).errorcnt;
                *fresh66 += 1;
            } else {
                let ref mut fresh67 = (*(*psp).prevrule).precsym;
                *fresh67 = Symbol_new(x);
            }
            (*psp).state = PRECEDENCE_MARK_2;
            current_block_454 = 4711096562713818440;
        }
        13 => {
            if *x.offset(0 as libc::c_int as isize) as libc::c_int != ']' as i32 {
                ErrorMsg(
                    (*psp).filename,
                    (*psp).tokenlineno,
                    b"Missing \"]\" on precedence mark.\0" as *const u8
                        as *const libc::c_char,
                );
                let ref mut fresh68 = (*psp).errorcnt;
                *fresh68 += 1;
            }
            (*psp).state = WAITING_FOR_DECL_OR_RULE;
            current_block_454 = 4711096562713818440;
        }
        5 => {
            if *x.offset(0 as libc::c_int as isize) as libc::c_int == ':' as i32
                && *x.offset(1 as libc::c_int as isize) as libc::c_int == ':' as i32
                && *x.offset(2 as libc::c_int as isize) as libc::c_int == '=' as i32
            {
                (*psp).state = IN_RHS;
            } else if *x.offset(0 as libc::c_int as isize) as libc::c_int == '(' as i32 {
                (*psp).state = LHS_ALIAS_1;
            } else {
                ErrorMsg(
                    (*psp).filename,
                    (*psp).tokenlineno,
                    b"Expected to see a \":\" following the LHS symbol \"%s\".\0"
                        as *const u8 as *const libc::c_char,
                    (*(*psp).lhs).name,
                );
                let ref mut fresh69 = (*psp).errorcnt;
                *fresh69 += 1;
                (*psp).state = RESYNC_AFTER_RULE_ERROR;
            }
            current_block_454 = 4711096562713818440;
        }
        7 => {
            if *(*__ctype_b_loc())
                .offset(
                    *x.offset(0 as libc::c_int as isize) as libc::c_uchar as libc::c_int
                        as isize,
                ) as libc::c_int
                & _ISalpha as libc::c_int as libc::c_ushort as libc::c_int != 0
            {
                let ref mut fresh70 = (*psp).lhsalias;
                *fresh70 = x;
                (*psp).state = LHS_ALIAS_2;
            } else {
                ErrorMsg(
                    (*psp).filename,
                    (*psp).tokenlineno,
                    b"\"%s\" is not a valid alias for the LHS \"%s\"\n\0" as *const u8
                        as *const libc::c_char,
                    x,
                    (*(*psp).lhs).name,
                );
                let ref mut fresh71 = (*psp).errorcnt;
                *fresh71 += 1;
                (*psp).state = RESYNC_AFTER_RULE_ERROR;
            }
            current_block_454 = 4711096562713818440;
        }
        8 => {
            if *x.offset(0 as libc::c_int as isize) as libc::c_int == ')' as i32 {
                (*psp).state = LHS_ALIAS_3;
            } else {
                ErrorMsg(
                    (*psp).filename,
                    (*psp).tokenlineno,
                    b"Missing \")\" following LHS alias name \"%s\".\0" as *const u8
                        as *const libc::c_char,
                    (*psp).lhsalias,
                );
                let ref mut fresh72 = (*psp).errorcnt;
                *fresh72 += 1;
                (*psp).state = RESYNC_AFTER_RULE_ERROR;
            }
            current_block_454 = 4711096562713818440;
        }
        9 => {
            if *x.offset(0 as libc::c_int as isize) as libc::c_int == ':' as i32
                && *x.offset(1 as libc::c_int as isize) as libc::c_int == ':' as i32
                && *x.offset(2 as libc::c_int as isize) as libc::c_int == '=' as i32
            {
                (*psp).state = IN_RHS;
            } else {
                ErrorMsg(
                    (*psp).filename,
                    (*psp).tokenlineno,
                    b"Missing \"->\" following: \"%s(%s)\".\0" as *const u8
                        as *const libc::c_char,
                    (*(*psp).lhs).name,
                    (*psp).lhsalias,
                );
                let ref mut fresh73 = (*psp).errorcnt;
                *fresh73 += 1;
                (*psp).state = RESYNC_AFTER_RULE_ERROR;
            }
            current_block_454 = 4711096562713818440;
        }
        6 => {
            if *x.offset(0 as libc::c_int as isize) as libc::c_int == '.' as i32 {
                let mut rp: *mut rule = 0 as *mut rule;
                rp = calloc(
                    (::std::mem::size_of::<rule>() as libc::c_ulong)
                        .wrapping_add(
                            (::std::mem::size_of::<*mut symbol>() as libc::c_ulong)
                                .wrapping_mul((*psp).nrhs as libc::c_ulong),
                        )
                        .wrapping_add(
                            (::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong)
                                .wrapping_mul((*psp).nrhs as libc::c_ulong),
                        ),
                    1 as libc::c_int as libc::c_ulong,
                ) as *mut rule;
                if rp.is_null() {
                    ErrorMsg(
                        (*psp).filename,
                        (*psp).tokenlineno,
                        b"Can't allocate enough memory for this rule.\0" as *const u8
                            as *const libc::c_char,
                    );
                    let ref mut fresh74 = (*psp).errorcnt;
                    *fresh74 += 1;
                    let ref mut fresh75 = (*psp).prevrule;
                    *fresh75 = 0 as *mut rule;
                } else {
                    let mut i: libc::c_int = 0;
                    (*rp).ruleline = (*psp).tokenlineno;
                    let ref mut fresh76 = (*rp).rhs;
                    *fresh76 = &mut *rp.offset(1 as libc::c_int as isize) as *mut rule
                        as *mut *mut symbol;
                    let ref mut fresh77 = (*rp).rhsalias;
                    *fresh77 = &mut *((*rp).rhs).offset((*psp).nrhs as isize)
                        as *mut *mut symbol as *mut *const libc::c_char;
                    i = 0 as libc::c_int;
                    while i < (*psp).nrhs {
                        let ref mut fresh78 = *((*rp).rhs).offset(i as isize);
                        *fresh78 = (*psp).rhs[i as usize];
                        let ref mut fresh79 = *((*rp).rhsalias).offset(i as isize);
                        *fresh79 = (*psp).alias[i as usize];
                        if !(*((*rp).rhsalias).offset(i as isize)).is_null() {
                            (**((*rp).rhs).offset(i as isize))
                                .bContent = 1 as libc::c_int;
                        }
                        i += 1;
                    }
                    let ref mut fresh80 = (*rp).lhs;
                    *fresh80 = (*psp).lhs;
                    let ref mut fresh81 = (*rp).lhsalias;
                    *fresh81 = (*psp).lhsalias;
                    (*rp).nrhs = (*psp).nrhs;
                    let ref mut fresh82 = (*rp).code;
                    *fresh82 = 0 as *const libc::c_char;
                    (*rp).noCode = LEMON_TRUE;
                    let ref mut fresh83 = (*rp).precsym;
                    *fresh83 = 0 as *mut symbol;
                    let ref mut fresh84 = (*(*psp).gp).nrule;
                    let fresh85 = *fresh84;
                    *fresh84 = *fresh84 + 1;
                    (*rp).index = fresh85;
                    let ref mut fresh86 = (*rp).nextlhs;
                    *fresh86 = (*(*rp).lhs).rule;
                    let ref mut fresh87 = (*(*rp).lhs).rule;
                    *fresh87 = rp;
                    let ref mut fresh88 = (*rp).next;
                    *fresh88 = 0 as *mut rule;
                    if ((*psp).firstrule).is_null() {
                        let ref mut fresh89 = (*psp).lastrule;
                        *fresh89 = rp;
                        let ref mut fresh90 = (*psp).firstrule;
                        *fresh90 = *fresh89;
                    } else {
                        let ref mut fresh91 = (*(*psp).lastrule).next;
                        *fresh91 = rp;
                        let ref mut fresh92 = (*psp).lastrule;
                        *fresh92 = rp;
                    }
                    let ref mut fresh93 = (*psp).prevrule;
                    *fresh93 = rp;
                }
                (*psp).state = WAITING_FOR_DECL_OR_RULE;
            } else if *(*__ctype_b_loc())
                .offset(
                    *x.offset(0 as libc::c_int as isize) as libc::c_uchar as libc::c_int
                        as isize,
                ) as libc::c_int
                & _ISalpha as libc::c_int as libc::c_ushort as libc::c_int != 0
            {
                if (*psp).nrhs >= 1000 as libc::c_int {
                    ErrorMsg(
                        (*psp).filename,
                        (*psp).tokenlineno,
                        b"Too many symbols on RHS of rule beginning at \"%s\".\0"
                            as *const u8 as *const libc::c_char,
                        x,
                    );
                    let ref mut fresh94 = (*psp).errorcnt;
                    *fresh94 += 1;
                    (*psp).state = RESYNC_AFTER_RULE_ERROR;
                } else {
                    let ref mut fresh95 = (*psp).rhs[(*psp).nrhs as usize];
                    *fresh95 = Symbol_new(x);
                    let ref mut fresh96 = (*psp).alias[(*psp).nrhs as usize];
                    *fresh96 = 0 as *const libc::c_char;
                    let ref mut fresh97 = (*psp).nrhs;
                    *fresh97 += 1;
                }
            } else if (*x.offset(0 as libc::c_int as isize) as libc::c_int == '|' as i32
                || *x.offset(0 as libc::c_int as isize) as libc::c_int == '/' as i32)
                && (*psp).nrhs > 0 as libc::c_int
                && *(*__ctype_b_loc())
                    .offset(
                        *x.offset(1 as libc::c_int as isize) as libc::c_uchar
                            as libc::c_int as isize,
                    ) as libc::c_int
                    & _ISupper as libc::c_int as libc::c_ushort as libc::c_int != 0
            {
                let mut msp: *mut symbol = (*psp)
                    .rhs[((*psp).nrhs - 1 as libc::c_int) as usize];
                if (*msp).type_0 as libc::c_uint
                    != MULTITERMINAL as libc::c_int as libc::c_uint
                {
                    let mut origsp: *mut symbol = msp;
                    msp = calloc(
                        1 as libc::c_int as libc::c_ulong,
                        ::std::mem::size_of::<symbol>() as libc::c_ulong,
                    ) as *mut symbol;
                    memset(
                        msp as *mut libc::c_void,
                        0 as libc::c_int,
                        ::std::mem::size_of::<symbol>() as libc::c_ulong,
                    );
                    (*msp).type_0 = MULTITERMINAL;
                    (*msp).nsubsym = 1 as libc::c_int;
                    let ref mut fresh98 = (*msp).subsym;
                    *fresh98 = calloc(
                        1 as libc::c_int as libc::c_ulong,
                        ::std::mem::size_of::<*mut symbol>() as libc::c_ulong,
                    ) as *mut *mut symbol;
                    let ref mut fresh99 = *((*msp).subsym)
                        .offset(0 as libc::c_int as isize);
                    *fresh99 = origsp;
                    let ref mut fresh100 = (*msp).name;
                    *fresh100 = (*origsp).name;
                    let ref mut fresh101 = (*psp)
                        .rhs[((*psp).nrhs - 1 as libc::c_int) as usize];
                    *fresh101 = msp;
                }
                let ref mut fresh102 = (*msp).nsubsym;
                *fresh102 += 1;
                let ref mut fresh103 = (*msp).subsym;
                *fresh103 = realloc(
                    (*msp).subsym as *mut libc::c_void,
                    (::std::mem::size_of::<*mut symbol>() as libc::c_ulong)
                        .wrapping_mul((*msp).nsubsym as libc::c_ulong),
                ) as *mut *mut symbol;
                let ref mut fresh104 = *((*msp).subsym)
                    .offset(((*msp).nsubsym - 1 as libc::c_int) as isize);
                *fresh104 = Symbol_new(&*x.offset(1 as libc::c_int as isize));
                if *(*__ctype_b_loc())
                    .offset(
                        *x.offset(1 as libc::c_int as isize) as libc::c_uchar
                            as libc::c_int as isize,
                    ) as libc::c_int
                    & _ISlower as libc::c_int as libc::c_ushort as libc::c_int != 0
                    || *(*__ctype_b_loc())
                        .offset(
                            *((**((*msp).subsym).offset(0 as libc::c_int as isize)).name)
                                .offset(0 as libc::c_int as isize) as libc::c_uchar
                                as libc::c_int as isize,
                        ) as libc::c_int
                        & _ISlower as libc::c_int as libc::c_ushort as libc::c_int != 0
                {
                    ErrorMsg(
                        (*psp).filename,
                        (*psp).tokenlineno,
                        b"Cannot form a compound containing a non-terminal\0"
                            as *const u8 as *const libc::c_char,
                    );
                    let ref mut fresh105 = (*psp).errorcnt;
                    *fresh105 += 1;
                }
            } else if *x.offset(0 as libc::c_int as isize) as libc::c_int == '(' as i32
                && (*psp).nrhs > 0 as libc::c_int
            {
                (*psp).state = RHS_ALIAS_1;
            } else {
                ErrorMsg(
                    (*psp).filename,
                    (*psp).tokenlineno,
                    b"Illegal character on RHS of rule: \"%s\".\0" as *const u8
                        as *const libc::c_char,
                    x,
                );
                let ref mut fresh106 = (*psp).errorcnt;
                *fresh106 += 1;
                (*psp).state = RESYNC_AFTER_RULE_ERROR;
            }
            current_block_454 = 4711096562713818440;
        }
        10 => {
            if *(*__ctype_b_loc())
                .offset(
                    *x.offset(0 as libc::c_int as isize) as libc::c_uchar as libc::c_int
                        as isize,
                ) as libc::c_int
                & _ISalpha as libc::c_int as libc::c_ushort as libc::c_int != 0
            {
                let ref mut fresh107 = (*psp)
                    .alias[((*psp).nrhs - 1 as libc::c_int) as usize];
                *fresh107 = x;
                (*psp).state = RHS_ALIAS_2;
            } else {
                ErrorMsg(
                    (*psp).filename,
                    (*psp).tokenlineno,
                    b"\"%s\" is not a valid alias for the RHS symbol \"%s\"\n\0"
                        as *const u8 as *const libc::c_char,
                    x,
                    (*(*psp).rhs[((*psp).nrhs - 1 as libc::c_int) as usize]).name,
                );
                let ref mut fresh108 = (*psp).errorcnt;
                *fresh108 += 1;
                (*psp).state = RESYNC_AFTER_RULE_ERROR;
            }
            current_block_454 = 4711096562713818440;
        }
        11 => {
            if *x.offset(0 as libc::c_int as isize) as libc::c_int == ')' as i32 {
                (*psp).state = IN_RHS;
            } else {
                ErrorMsg(
                    (*psp).filename,
                    (*psp).tokenlineno,
                    b"Missing \")\" following LHS alias name \"%s\".\0" as *const u8
                        as *const libc::c_char,
                    (*psp).lhsalias,
                );
                let ref mut fresh109 = (*psp).errorcnt;
                *fresh109 += 1;
                (*psp).state = RESYNC_AFTER_RULE_ERROR;
            }
            current_block_454 = 4711096562713818440;
        }
        2 => {
            if *(*__ctype_b_loc())
                .offset(
                    *x.offset(0 as libc::c_int as isize) as libc::c_uchar as libc::c_int
                        as isize,
                ) as libc::c_int
                & _ISalpha as libc::c_int as libc::c_ushort as libc::c_int != 0
            {
                let ref mut fresh110 = (*psp).declkeyword;
                *fresh110 = x;
                let ref mut fresh111 = (*psp).declargslot;
                *fresh111 = 0 as *mut *mut libc::c_char;
                let ref mut fresh112 = (*psp).decllinenoslot;
                *fresh112 = 0 as *mut libc::c_int;
                (*psp).insertLineMacro = 1 as libc::c_int;
                (*psp).state = WAITING_FOR_DECL_ARG;
                if strcmp(x, b"name\0" as *const u8 as *const libc::c_char)
                    == 0 as libc::c_int
                {
                    let ref mut fresh113 = (*psp).declargslot;
                    *fresh113 = &mut (*(*psp).gp).name;
                    (*psp).insertLineMacro = 0 as libc::c_int;
                } else if strcmp(x, b"include\0" as *const u8 as *const libc::c_char)
                    == 0 as libc::c_int
                {
                    let ref mut fresh114 = (*psp).declargslot;
                    *fresh114 = &mut (*(*psp).gp).include;
                } else if strcmp(x, b"code\0" as *const u8 as *const libc::c_char)
                    == 0 as libc::c_int
                {
                    let ref mut fresh115 = (*psp).declargslot;
                    *fresh115 = &mut (*(*psp).gp).extracode;
                } else if strcmp(
                    x,
                    b"token_destructor\0" as *const u8 as *const libc::c_char,
                ) == 0 as libc::c_int
                {
                    let ref mut fresh116 = (*psp).declargslot;
                    *fresh116 = &mut (*(*psp).gp).tokendest;
                } else if strcmp(
                    x,
                    b"default_destructor\0" as *const u8 as *const libc::c_char,
                ) == 0 as libc::c_int
                {
                    let ref mut fresh117 = (*psp).declargslot;
                    *fresh117 = &mut (*(*psp).gp).vardest;
                } else if strcmp(
                    x,
                    b"token_prefix\0" as *const u8 as *const libc::c_char,
                ) == 0 as libc::c_int
                {
                    let ref mut fresh118 = (*psp).declargslot;
                    *fresh118 = &mut (*(*psp).gp).tokenprefix;
                    (*psp).insertLineMacro = 0 as libc::c_int;
                } else if strcmp(
                    x,
                    b"syntax_error\0" as *const u8 as *const libc::c_char,
                ) == 0 as libc::c_int
                {
                    let ref mut fresh119 = (*psp).declargslot;
                    *fresh119 = &mut (*(*psp).gp).error;
                } else if strcmp(
                    x,
                    b"parse_accept\0" as *const u8 as *const libc::c_char,
                ) == 0 as libc::c_int
                {
                    let ref mut fresh120 = (*psp).declargslot;
                    *fresh120 = &mut (*(*psp).gp).accept;
                } else if strcmp(
                    x,
                    b"parse_failure\0" as *const u8 as *const libc::c_char,
                ) == 0 as libc::c_int
                {
                    let ref mut fresh121 = (*psp).declargslot;
                    *fresh121 = &mut (*(*psp).gp).failure;
                } else if strcmp(
                    x,
                    b"stack_overflow\0" as *const u8 as *const libc::c_char,
                ) == 0 as libc::c_int
                {
                    let ref mut fresh122 = (*psp).declargslot;
                    *fresh122 = &mut (*(*psp).gp).overflow;
                } else if strcmp(
                    x,
                    b"extra_argument\0" as *const u8 as *const libc::c_char,
                ) == 0 as libc::c_int
                {
                    let ref mut fresh123 = (*psp).declargslot;
                    *fresh123 = &mut (*(*psp).gp).arg;
                    (*psp).insertLineMacro = 0 as libc::c_int;
                } else if strcmp(
                    x,
                    b"extra_context\0" as *const u8 as *const libc::c_char,
                ) == 0 as libc::c_int
                {
                    let ref mut fresh124 = (*psp).declargslot;
                    *fresh124 = &mut (*(*psp).gp).ctx;
                    (*psp).insertLineMacro = 0 as libc::c_int;
                } else if strcmp(x, b"token_type\0" as *const u8 as *const libc::c_char)
                    == 0 as libc::c_int
                {
                    let ref mut fresh125 = (*psp).declargslot;
                    *fresh125 = &mut (*(*psp).gp).tokentype;
                    (*psp).insertLineMacro = 0 as libc::c_int;
                } else if strcmp(
                    x,
                    b"default_type\0" as *const u8 as *const libc::c_char,
                ) == 0 as libc::c_int
                {
                    let ref mut fresh126 = (*psp).declargslot;
                    *fresh126 = &mut (*(*psp).gp).vartype;
                    (*psp).insertLineMacro = 0 as libc::c_int;
                } else if strcmp(x, b"stack_size\0" as *const u8 as *const libc::c_char)
                    == 0 as libc::c_int
                {
                    let ref mut fresh127 = (*psp).declargslot;
                    *fresh127 = &mut (*(*psp).gp).stacksize;
                    (*psp).insertLineMacro = 0 as libc::c_int;
                } else if strcmp(
                    x,
                    b"start_symbol\0" as *const u8 as *const libc::c_char,
                ) == 0 as libc::c_int
                {
                    let ref mut fresh128 = (*psp).declargslot;
                    *fresh128 = &mut (*(*psp).gp).start;
                    (*psp).insertLineMacro = 0 as libc::c_int;
                } else if strcmp(x, b"left\0" as *const u8 as *const libc::c_char)
                    == 0 as libc::c_int
                {
                    let ref mut fresh129 = (*psp).preccounter;
                    *fresh129 += 1;
                    (*psp).declassoc = LEFT;
                    (*psp).state = WAITING_FOR_PRECEDENCE_SYMBOL;
                } else if strcmp(x, b"right\0" as *const u8 as *const libc::c_char)
                    == 0 as libc::c_int
                {
                    let ref mut fresh130 = (*psp).preccounter;
                    *fresh130 += 1;
                    (*psp).declassoc = RIGHT;
                    (*psp).state = WAITING_FOR_PRECEDENCE_SYMBOL;
                } else if strcmp(x, b"nonassoc\0" as *const u8 as *const libc::c_char)
                    == 0 as libc::c_int
                {
                    let ref mut fresh131 = (*psp).preccounter;
                    *fresh131 += 1;
                    (*psp).declassoc = NONE;
                    (*psp).state = WAITING_FOR_PRECEDENCE_SYMBOL;
                } else if strcmp(x, b"destructor\0" as *const u8 as *const libc::c_char)
                    == 0 as libc::c_int
                {
                    (*psp).state = WAITING_FOR_DESTRUCTOR_SYMBOL;
                } else if strcmp(x, b"type\0" as *const u8 as *const libc::c_char)
                    == 0 as libc::c_int
                {
                    (*psp).state = WAITING_FOR_DATATYPE_SYMBOL;
                } else if strcmp(x, b"fallback\0" as *const u8 as *const libc::c_char)
                    == 0 as libc::c_int
                {
                    let ref mut fresh132 = (*psp).fallback;
                    *fresh132 = 0 as *mut symbol;
                    (*psp).state = WAITING_FOR_FALLBACK_ID;
                } else if strcmp(x, b"token\0" as *const u8 as *const libc::c_char)
                    == 0 as libc::c_int
                {
                    (*psp).state = WAITING_FOR_TOKEN_NAME;
                } else if strcmp(x, b"wildcard\0" as *const u8 as *const libc::c_char)
                    == 0 as libc::c_int
                {
                    (*psp).state = WAITING_FOR_WILDCARD_ID;
                } else if strcmp(x, b"token_class\0" as *const u8 as *const libc::c_char)
                    == 0 as libc::c_int
                {
                    (*psp).state = WAITING_FOR_CLASS_ID;
                } else {
                    ErrorMsg(
                        (*psp).filename,
                        (*psp).tokenlineno,
                        b"Unknown declaration keyword: \"%%%s\".\0" as *const u8
                            as *const libc::c_char,
                        x,
                    );
                    let ref mut fresh133 = (*psp).errorcnt;
                    *fresh133 += 1;
                    (*psp).state = RESYNC_AFTER_DECL_ERROR;
                }
            } else {
                ErrorMsg(
                    (*psp).filename,
                    (*psp).tokenlineno,
                    b"Illegal declaration keyword: \"%s\".\0" as *const u8
                        as *const libc::c_char,
                    x,
                );
                let ref mut fresh134 = (*psp).errorcnt;
                *fresh134 += 1;
                (*psp).state = RESYNC_AFTER_DECL_ERROR;
            }
            current_block_454 = 4711096562713818440;
        }
        16 => {
            if *(*__ctype_b_loc())
                .offset(
                    *x.offset(0 as libc::c_int as isize) as libc::c_uchar as libc::c_int
                        as isize,
                ) as libc::c_int
                & _ISalpha as libc::c_int as libc::c_ushort as libc::c_int == 0
            {
                ErrorMsg(
                    (*psp).filename,
                    (*psp).tokenlineno,
                    b"Symbol name missing after %%destructor keyword\0" as *const u8
                        as *const libc::c_char,
                );
                let ref mut fresh135 = (*psp).errorcnt;
                *fresh135 += 1;
                (*psp).state = RESYNC_AFTER_DECL_ERROR;
            } else {
                let mut sp: *mut symbol = Symbol_new(x);
                let ref mut fresh136 = (*psp).declargslot;
                *fresh136 = &mut (*sp).destructor;
                let ref mut fresh137 = (*psp).decllinenoslot;
                *fresh137 = &mut (*sp).destLineno;
                (*psp).insertLineMacro = 1 as libc::c_int;
                (*psp).state = WAITING_FOR_DECL_ARG;
            }
            current_block_454 = 4711096562713818440;
        }
        17 => {
            if *(*__ctype_b_loc())
                .offset(
                    *x.offset(0 as libc::c_int as isize) as libc::c_uchar as libc::c_int
                        as isize,
                ) as libc::c_int
                & _ISalpha as libc::c_int as libc::c_ushort as libc::c_int == 0
            {
                ErrorMsg(
                    (*psp).filename,
                    (*psp).tokenlineno,
                    b"Symbol name missing after %%type keyword\0" as *const u8
                        as *const libc::c_char,
                );
                let ref mut fresh138 = (*psp).errorcnt;
                *fresh138 += 1;
                (*psp).state = RESYNC_AFTER_DECL_ERROR;
            } else {
                let mut sp_0: *mut symbol = Symbol_find(x);
                if !sp_0.is_null() && !((*sp_0).datatype).is_null() {
                    ErrorMsg(
                        (*psp).filename,
                        (*psp).tokenlineno,
                        b"Symbol %%type \"%s\" already defined\0" as *const u8
                            as *const libc::c_char,
                        x,
                    );
                    let ref mut fresh139 = (*psp).errorcnt;
                    *fresh139 += 1;
                    (*psp).state = RESYNC_AFTER_DECL_ERROR;
                } else {
                    if sp_0.is_null() {
                        sp_0 = Symbol_new(x);
                    }
                    let ref mut fresh140 = (*psp).declargslot;
                    *fresh140 = &mut (*sp_0).datatype;
                    (*psp).insertLineMacro = 0 as libc::c_int;
                    (*psp).state = WAITING_FOR_DECL_ARG;
                }
            }
            current_block_454 = 4711096562713818440;
        }
        4 => {
            if *x.offset(0 as libc::c_int as isize) as libc::c_int == '.' as i32 {
                (*psp).state = WAITING_FOR_DECL_OR_RULE;
            } else if *(*__ctype_b_loc())
                .offset(
                    *x.offset(0 as libc::c_int as isize) as libc::c_uchar as libc::c_int
                        as isize,
                ) as libc::c_int
                & _ISupper as libc::c_int as libc::c_ushort as libc::c_int != 0
            {
                let mut sp_1: *mut symbol = 0 as *mut symbol;
                sp_1 = Symbol_new(x);
                if (*sp_1).prec >= 0 as libc::c_int {
                    ErrorMsg(
                        (*psp).filename,
                        (*psp).tokenlineno,
                        b"Symbol \"%s\" has already be given a precedence.\0"
                            as *const u8 as *const libc::c_char,
                        x,
                    );
                    let ref mut fresh141 = (*psp).errorcnt;
                    *fresh141 += 1;
                } else {
                    (*sp_1).prec = (*psp).preccounter;
                    (*sp_1).assoc = (*psp).declassoc;
                }
            } else {
                ErrorMsg(
                    (*psp).filename,
                    (*psp).tokenlineno,
                    b"Can't assign a precedence to \"%s\".\0" as *const u8
                        as *const libc::c_char,
                    x,
                );
                let ref mut fresh142 = (*psp).errorcnt;
                *fresh142 += 1;
            }
            current_block_454 = 4711096562713818440;
        }
        3 => {
            if *x.offset(0 as libc::c_int as isize) as libc::c_int == '{' as i32
                || *x.offset(0 as libc::c_int as isize) as libc::c_int == '"' as i32
                || *(*__ctype_b_loc())
                    .offset(
                        *x.offset(0 as libc::c_int as isize) as libc::c_uchar
                            as libc::c_int as isize,
                    ) as libc::c_int
                    & _ISalnum as libc::c_int as libc::c_ushort as libc::c_int != 0
            {
                let mut zOld: *const libc::c_char = 0 as *const libc::c_char;
                let mut zNew: *const libc::c_char = 0 as *const libc::c_char;
                let mut zBuf: *mut libc::c_char = 0 as *mut libc::c_char;
                let mut z: *mut libc::c_char = 0 as *mut libc::c_char;
                let mut nOld: libc::c_int = 0;
                let mut n: libc::c_int = 0;
                let mut nLine: libc::c_int = 0 as libc::c_int;
                let mut nNew: libc::c_int = 0;
                let mut nBack: libc::c_int = 0;
                let mut addLineMacro: libc::c_int = 0;
                let mut zLine: [libc::c_char; 50] = [0; 50];
                zNew = x;
                if *zNew.offset(0 as libc::c_int as isize) as libc::c_int == '"' as i32
                    || *zNew.offset(0 as libc::c_int as isize) as libc::c_int
                        == '{' as i32
                {
                    zNew = zNew.offset(1);
                }
                nNew = strlen(zNew) as libc::c_int;
                if !(*(*psp).declargslot).is_null() {
                    zOld = *(*psp).declargslot;
                } else {
                    zOld = b"\0" as *const u8 as *const libc::c_char;
                }
                nOld = strlen(zOld) as libc::c_int;
                n = nOld + nNew + 20 as libc::c_int;
                addLineMacro = ((*(*psp).gp).nolinenosflag == 0
                    && (*psp).insertLineMacro != 0
                    && (*psp).tokenlineno > 1 as libc::c_int
                    && (((*psp).decllinenoslot).is_null()
                        || *((*psp).decllinenoslot).offset(0 as libc::c_int as isize)
                            != 0 as libc::c_int)) as libc::c_int;
                if addLineMacro != 0 {
                    z = (*psp).filename;
                    nBack = 0 as libc::c_int;
                    while *z != 0 {
                        if *z as libc::c_int == '\\' as i32 {
                            nBack += 1;
                        }
                        z = z.offset(1);
                    }
                    lemon_sprintf(
                        zLine.as_mut_ptr(),
                        b"#line %d \0" as *const u8 as *const libc::c_char,
                        (*psp).tokenlineno,
                    );
                    nLine = strlen(zLine.as_mut_ptr()) as libc::c_int;
                    n += nLine + strlen((*psp).filename) as libc::c_int + nBack;
                }
                let ref mut fresh143 = *(*psp).declargslot;
                *fresh143 = realloc(
                    *(*psp).declargslot as *mut libc::c_void,
                    n as libc::c_ulong,
                ) as *mut libc::c_char;
                zBuf = (*(*psp).declargslot).offset(nOld as isize);
                if addLineMacro != 0 {
                    if nOld != 0
                        && *zBuf.offset(-(1 as libc::c_int) as isize) as libc::c_int
                            != '\n' as i32
                    {
                        let fresh144 = zBuf;
                        zBuf = zBuf.offset(1);
                        *fresh144 = '\n' as i32 as libc::c_char;
                    }
                    memcpy(
                        zBuf as *mut libc::c_void,
                        zLine.as_mut_ptr() as *const libc::c_void,
                        nLine as libc::c_ulong,
                    );
                    zBuf = zBuf.offset(nLine as isize);
                    let fresh145 = zBuf;
                    zBuf = zBuf.offset(1);
                    *fresh145 = '"' as i32 as libc::c_char;
                    z = (*psp).filename;
                    while *z != 0 {
                        if *z as libc::c_int == '\\' as i32 {
                            let fresh146 = zBuf;
                            zBuf = zBuf.offset(1);
                            *fresh146 = '\\' as i32 as libc::c_char;
                        }
                        let fresh147 = zBuf;
                        zBuf = zBuf.offset(1);
                        *fresh147 = *z;
                        z = z.offset(1);
                    }
                    let fresh148 = zBuf;
                    zBuf = zBuf.offset(1);
                    *fresh148 = '"' as i32 as libc::c_char;
                    let fresh149 = zBuf;
                    zBuf = zBuf.offset(1);
                    *fresh149 = '\n' as i32 as libc::c_char;
                }
                if !((*psp).decllinenoslot).is_null()
                    && *((*psp).decllinenoslot).offset(0 as libc::c_int as isize)
                        == 0 as libc::c_int
                {
                    *((*psp).decllinenoslot)
                        .offset(0 as libc::c_int as isize) = (*psp).tokenlineno;
                }
                memcpy(
                    zBuf as *mut libc::c_void,
                    zNew as *const libc::c_void,
                    nNew as libc::c_ulong,
                );
                zBuf = zBuf.offset(nNew as isize);
                *zBuf = 0 as libc::c_int as libc::c_char;
                (*psp).state = WAITING_FOR_DECL_OR_RULE;
            } else {
                ErrorMsg(
                    (*psp).filename,
                    (*psp).tokenlineno,
                    b"Illegal argument to %%%s: %s\0" as *const u8
                        as *const libc::c_char,
                    (*psp).declkeyword,
                    x,
                );
                let ref mut fresh150 = (*psp).errorcnt;
                *fresh150 += 1;
                (*psp).state = RESYNC_AFTER_DECL_ERROR;
            }
            current_block_454 = 4711096562713818440;
        }
        18 => {
            if *x.offset(0 as libc::c_int as isize) as libc::c_int == '.' as i32 {
                (*psp).state = WAITING_FOR_DECL_OR_RULE;
            } else if *(*__ctype_b_loc())
                .offset(
                    *x.offset(0 as libc::c_int as isize) as libc::c_uchar as libc::c_int
                        as isize,
                ) as libc::c_int
                & _ISupper as libc::c_int as libc::c_ushort as libc::c_int == 0
            {
                ErrorMsg(
                    (*psp).filename,
                    (*psp).tokenlineno,
                    b"%%fallback argument \"%s\" should be a token\0" as *const u8
                        as *const libc::c_char,
                    x,
                );
                let ref mut fresh151 = (*psp).errorcnt;
                *fresh151 += 1;
            } else {
                let mut sp_2: *mut symbol = Symbol_new(x);
                if ((*psp).fallback).is_null() {
                    let ref mut fresh152 = (*psp).fallback;
                    *fresh152 = sp_2;
                } else if !((*sp_2).fallback).is_null() {
                    ErrorMsg(
                        (*psp).filename,
                        (*psp).tokenlineno,
                        b"More than one fallback assigned to token %s\0" as *const u8
                            as *const libc::c_char,
                        x,
                    );
                    let ref mut fresh153 = (*psp).errorcnt;
                    *fresh153 += 1;
                } else {
                    let ref mut fresh154 = (*sp_2).fallback;
                    *fresh154 = (*psp).fallback;
                    (*(*psp).gp).has_fallback = 1 as libc::c_int;
                }
            }
            current_block_454 = 4711096562713818440;
        }
        22 => {
            if *x.offset(0 as libc::c_int as isize) as libc::c_int == '.' as i32 {
                (*psp).state = WAITING_FOR_DECL_OR_RULE;
            } else if *(*__ctype_b_loc())
                .offset(
                    *x.offset(0 as libc::c_int as isize) as libc::c_uchar as libc::c_int
                        as isize,
                ) as libc::c_int
                & _ISupper as libc::c_int as libc::c_ushort as libc::c_int == 0
            {
                ErrorMsg(
                    (*psp).filename,
                    (*psp).tokenlineno,
                    b"%%token argument \"%s\" should be a token\0" as *const u8
                        as *const libc::c_char,
                    x,
                );
                let ref mut fresh155 = (*psp).errorcnt;
                *fresh155 += 1;
            } else {
                Symbol_new(x);
            }
            current_block_454 = 4711096562713818440;
        }
        19 => {
            if *x.offset(0 as libc::c_int as isize) as libc::c_int == '.' as i32 {
                (*psp).state = WAITING_FOR_DECL_OR_RULE;
            } else if *(*__ctype_b_loc())
                .offset(
                    *x.offset(0 as libc::c_int as isize) as libc::c_uchar as libc::c_int
                        as isize,
                ) as libc::c_int
                & _ISupper as libc::c_int as libc::c_ushort as libc::c_int == 0
            {
                ErrorMsg(
                    (*psp).filename,
                    (*psp).tokenlineno,
                    b"%%wildcard argument \"%s\" should be a token\0" as *const u8
                        as *const libc::c_char,
                    x,
                );
                let ref mut fresh156 = (*psp).errorcnt;
                *fresh156 += 1;
            } else {
                let mut sp_3: *mut symbol = Symbol_new(x);
                if ((*(*psp).gp).wildcard).is_null() {
                    let ref mut fresh157 = (*(*psp).gp).wildcard;
                    *fresh157 = sp_3;
                } else {
                    ErrorMsg(
                        (*psp).filename,
                        (*psp).tokenlineno,
                        b"Extra wildcard to token: %s\0" as *const u8
                            as *const libc::c_char,
                        x,
                    );
                    let ref mut fresh158 = (*psp).errorcnt;
                    *fresh158 += 1;
                }
            }
            current_block_454 = 4711096562713818440;
        }
        20 => {
            if *(*__ctype_b_loc())
                .offset(
                    *x.offset(0 as libc::c_int as isize) as libc::c_uchar as libc::c_int
                        as isize,
                ) as libc::c_int
                & _ISlower as libc::c_int as libc::c_ushort as libc::c_int == 0
            {
                ErrorMsg(
                    (*psp).filename,
                    (*psp).tokenlineno,
                    b"%%token_class must be followed by an identifier: %s\0" as *const u8
                        as *const libc::c_char,
                    x,
                );
                let ref mut fresh159 = (*psp).errorcnt;
                *fresh159 += 1;
                (*psp).state = RESYNC_AFTER_DECL_ERROR;
            } else if !(Symbol_find(x)).is_null() {
                ErrorMsg(
                    (*psp).filename,
                    (*psp).tokenlineno,
                    b"Symbol \"%s\" already used\0" as *const u8 as *const libc::c_char,
                    x,
                );
                let ref mut fresh160 = (*psp).errorcnt;
                *fresh160 += 1;
                (*psp).state = RESYNC_AFTER_DECL_ERROR;
            } else {
                let ref mut fresh161 = (*psp).tkclass;
                *fresh161 = Symbol_new(x);
                (*(*psp).tkclass).type_0 = MULTITERMINAL;
                (*psp).state = WAITING_FOR_CLASS_TOKEN;
            }
            current_block_454 = 4711096562713818440;
        }
        21 => {
            if *x.offset(0 as libc::c_int as isize) as libc::c_int == '.' as i32 {
                (*psp).state = WAITING_FOR_DECL_OR_RULE;
            } else if *(*__ctype_b_loc())
                .offset(
                    *x.offset(0 as libc::c_int as isize) as libc::c_uchar as libc::c_int
                        as isize,
                ) as libc::c_int
                & _ISupper as libc::c_int as libc::c_ushort as libc::c_int != 0
                || (*x.offset(0 as libc::c_int as isize) as libc::c_int == '|' as i32
                    || *x.offset(0 as libc::c_int as isize) as libc::c_int == '/' as i32)
                    && *(*__ctype_b_loc())
                        .offset(
                            *x.offset(1 as libc::c_int as isize) as libc::c_uchar
                                as libc::c_int as isize,
                        ) as libc::c_int
                        & _ISupper as libc::c_int as libc::c_ushort as libc::c_int != 0
            {
                let mut msp_0: *mut symbol = (*psp).tkclass;
                let ref mut fresh162 = (*msp_0).nsubsym;
                *fresh162 += 1;
                let ref mut fresh163 = (*msp_0).subsym;
                *fresh163 = realloc(
                    (*msp_0).subsym as *mut libc::c_void,
                    (::std::mem::size_of::<*mut symbol>() as libc::c_ulong)
                        .wrapping_mul((*msp_0).nsubsym as libc::c_ulong),
                ) as *mut *mut symbol;
                if *(*__ctype_b_loc())
                    .offset(
                        *x.offset(0 as libc::c_int as isize) as libc::c_uchar
                            as libc::c_int as isize,
                    ) as libc::c_int
                    & _ISupper as libc::c_int as libc::c_ushort as libc::c_int == 0
                {
                    x = x.offset(1);
                }
                let ref mut fresh164 = *((*msp_0).subsym)
                    .offset(((*msp_0).nsubsym - 1 as libc::c_int) as isize);
                *fresh164 = Symbol_new(x);
            } else {
                ErrorMsg(
                    (*psp).filename,
                    (*psp).tokenlineno,
                    b"%%token_class argument \"%s\" should be a token\0" as *const u8
                        as *const libc::c_char,
                    x,
                );
                let ref mut fresh165 = (*psp).errorcnt;
                *fresh165 += 1;
                (*psp).state = RESYNC_AFTER_DECL_ERROR;
            }
            current_block_454 = 4711096562713818440;
        }
        14 | 15 => {
            if *x.offset(0 as libc::c_int as isize) as libc::c_int == '.' as i32 {
                (*psp).state = WAITING_FOR_DECL_OR_RULE;
            }
            if *x.offset(0 as libc::c_int as isize) as libc::c_int == '%' as i32 {
                (*psp).state = WAITING_FOR_DECL_KEYWORD;
            }
            current_block_454 = 4711096562713818440;
        }
        _ => {
            current_block_454 = 4711096562713818440;
        }
    }
    match current_block_454 {
        3403847451326750966 => {
            if *x.offset(0 as libc::c_int as isize) as libc::c_int == '%' as i32 {
                (*psp).state = WAITING_FOR_DECL_KEYWORD;
            } else if *(*__ctype_b_loc())
                .offset(
                    *x.offset(0 as libc::c_int as isize) as libc::c_uchar as libc::c_int
                        as isize,
                ) as libc::c_int
                & _ISlower as libc::c_int as libc::c_ushort as libc::c_int != 0
            {
                let ref mut fresh58 = (*psp).lhs;
                *fresh58 = Symbol_new(x);
                (*psp).nrhs = 0 as libc::c_int;
                let ref mut fresh59 = (*psp).lhsalias;
                *fresh59 = 0 as *const libc::c_char;
                (*psp).state = WAITING_FOR_ARROW;
            } else if *x.offset(0 as libc::c_int as isize) as libc::c_int == '{' as i32 {
                if ((*psp).prevrule).is_null() {
                    ErrorMsg(
                        (*psp).filename,
                        (*psp).tokenlineno,
                        b"There is no prior rule upon which to attach the code fragment which begins on this line.\0"
                            as *const u8 as *const libc::c_char,
                    );
                    let ref mut fresh60 = (*psp).errorcnt;
                    *fresh60 += 1;
                } else if !((*(*psp).prevrule).code).is_null() {
                    ErrorMsg(
                        (*psp).filename,
                        (*psp).tokenlineno,
                        b"Code fragment beginning on this line is not the first to follow the previous rule.\0"
                            as *const u8 as *const libc::c_char,
                    );
                    let ref mut fresh61 = (*psp).errorcnt;
                    *fresh61 += 1;
                } else if strcmp(
                    x,
                    b"{NEVER-REDUCE\0" as *const u8 as *const libc::c_char,
                ) == 0 as libc::c_int
                {
                    (*(*psp).prevrule).neverReduce = LEMON_TRUE;
                } else {
                    (*(*psp).prevrule).line = (*psp).tokenlineno;
                    let ref mut fresh62 = (*(*psp).prevrule).code;
                    *fresh62 = &*x.offset(1 as libc::c_int as isize)
                        as *const libc::c_char;
                    (*(*psp).prevrule).noCode = LEMON_FALSE;
                }
            } else if *x.offset(0 as libc::c_int as isize) as libc::c_int == '[' as i32 {
                (*psp).state = PRECEDENCE_MARK_1;
            } else {
                ErrorMsg(
                    (*psp).filename,
                    (*psp).tokenlineno,
                    b"Token \"%s\" should be either \"%%\" or a nonterminal name.\0"
                        as *const u8 as *const libc::c_char,
                    x,
                );
                let ref mut fresh63 = (*psp).errorcnt;
                *fresh63 += 1;
            }
        }
        _ => {}
    };
}
unsafe extern "C" fn eval_preprocessor_boolean(
    mut z: *mut libc::c_char,
    mut lineno: libc::c_int,
) -> libc::c_int {
    let mut current_block: u64;
    let mut neg: libc::c_int = 0 as libc::c_int;
    let mut res: libc::c_int = 0 as libc::c_int;
    let mut okTerm: libc::c_int = 1 as libc::c_int;
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    's_15: loop {
        if !(*z.offset(i as isize) as libc::c_int != 0 as libc::c_int) {
            current_block = 12264624100856317061;
            break;
        }
        if !(*(*__ctype_b_loc())
            .offset(*z.offset(i as isize) as libc::c_uchar as libc::c_int as isize)
            as libc::c_int & _ISspace as libc::c_int as libc::c_ushort as libc::c_int
            != 0)
        {
            if *z.offset(i as isize) as libc::c_int == '!' as i32 {
                if okTerm == 0 {
                    current_block = 6525116257640989054;
                    break;
                }
                neg = (neg == 0) as libc::c_int;
            } else if *z.offset(i as isize) as libc::c_int == '|' as i32
                && *z.offset((i + 1 as libc::c_int) as isize) as libc::c_int
                    == '|' as i32
            {
                if okTerm != 0 {
                    current_block = 6525116257640989054;
                    break;
                }
                if res != 0 {
                    return 1 as libc::c_int;
                }
                i += 1;
                okTerm = 1 as libc::c_int;
            } else if *z.offset(i as isize) as libc::c_int == '&' as i32
                && *z.offset((i + 1 as libc::c_int) as isize) as libc::c_int
                    == '&' as i32
            {
                if okTerm != 0 {
                    current_block = 6525116257640989054;
                    break;
                }
                if res == 0 {
                    return 0 as libc::c_int;
                }
                i += 1;
                okTerm = 1 as libc::c_int;
            } else if *z.offset(i as isize) as libc::c_int == '(' as i32 {
                let mut k: libc::c_int = 0;
                let mut n: libc::c_int = 1 as libc::c_int;
                if okTerm == 0 {
                    current_block = 6525116257640989054;
                    break;
                }
                k = i + 1 as libc::c_int;
                while *z.offset(k as isize) != 0 {
                    if *z.offset(k as isize) as libc::c_int == ')' as i32 {
                        n -= 1;
                        if n == 0 as libc::c_int {
                            *z.offset(k as isize) = 0 as libc::c_int as libc::c_char;
                            res = eval_preprocessor_boolean(
                                &mut *z.offset((i + 1 as libc::c_int) as isize),
                                -(1 as libc::c_int),
                            );
                            *z.offset(k as isize) = ')' as i32 as libc::c_char;
                            if res < 0 as libc::c_int {
                                i = i - res;
                                current_block = 6525116257640989054;
                                break 's_15;
                            } else {
                                i = k;
                                break;
                            }
                        }
                    } else if *z.offset(k as isize) as libc::c_int == '(' as i32 {
                        n += 1;
                    } else if *z.offset(k as isize) as libc::c_int == 0 as libc::c_int {
                        i = k;
                        current_block = 6525116257640989054;
                        break 's_15;
                    }
                    k += 1;
                }
                if neg != 0 {
                    res = (res == 0) as libc::c_int;
                    neg = 0 as libc::c_int;
                }
                okTerm = 0 as libc::c_int;
            } else {
                if !(*(*__ctype_b_loc())
                    .offset(
                        *z.offset(i as isize) as libc::c_uchar as libc::c_int as isize,
                    ) as libc::c_int
                    & _ISalpha as libc::c_int as libc::c_ushort as libc::c_int != 0)
                {
                    current_block = 6525116257640989054;
                    break;
                }
                let mut j: libc::c_int = 0;
                let mut k_0: libc::c_int = 0;
                let mut n_0: libc::c_int = 0;
                if okTerm == 0 {
                    current_block = 6525116257640989054;
                    break;
                }
                k_0 = i + 1 as libc::c_int;
                while *(*__ctype_b_loc())
                    .offset(
                        *z.offset(k_0 as isize) as libc::c_uchar as libc::c_int as isize,
                    ) as libc::c_int
                    & _ISalnum as libc::c_int as libc::c_ushort as libc::c_int != 0
                    || *z.offset(k_0 as isize) as libc::c_int == '_' as i32
                {
                    k_0 += 1;
                }
                n_0 = k_0 - i;
                res = 0 as libc::c_int;
                j = 0 as libc::c_int;
                while j < nDefine {
                    if strncmp(
                        *azDefine.offset(j as isize),
                        &mut *z.offset(i as isize),
                        n_0 as libc::c_ulong,
                    ) == 0 as libc::c_int
                        && *(*azDefine.offset(j as isize)).offset(n_0 as isize)
                            as libc::c_int == 0 as libc::c_int
                    {
                        res = 1 as libc::c_int;
                        break;
                    } else {
                        j += 1;
                    }
                }
                i = k_0 - 1 as libc::c_int;
                if neg != 0 {
                    res = (res == 0) as libc::c_int;
                    neg = 0 as libc::c_int;
                }
                okTerm = 0 as libc::c_int;
            }
        }
        i += 1;
    }
    match current_block {
        6525116257640989054 => {
            if lineno > 0 as libc::c_int {
                fprintf(
                    stderr,
                    b"%%if syntax error on line %d.\n\0" as *const u8
                        as *const libc::c_char,
                    lineno,
                );
                fprintf(
                    stderr,
                    b"  %.*s <-- syntax error here\n\0" as *const u8
                        as *const libc::c_char,
                    i + 1 as libc::c_int,
                    z,
                );
                exit(1 as libc::c_int);
            } else {
                return -(i + 1 as libc::c_int)
            }
        }
        _ => return res,
    };
}
unsafe extern "C" fn preprocess_input(mut z: *mut libc::c_char) {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut exclude: libc::c_int = 0 as libc::c_int;
    let mut start: libc::c_int = 0 as libc::c_int;
    let mut lineno: libc::c_int = 1 as libc::c_int;
    let mut start_lineno: libc::c_int = 1 as libc::c_int;
    i = 0 as libc::c_int;
    while *z.offset(i as isize) != 0 {
        if *z.offset(i as isize) as libc::c_int == '\n' as i32 {
            lineno += 1;
        }
        if !(*z.offset(i as isize) as libc::c_int != '%' as i32
            || i > 0 as libc::c_int
                && *z.offset((i - 1 as libc::c_int) as isize) as libc::c_int
                    != '\n' as i32)
        {
            if strncmp(
                &mut *z.offset(i as isize),
                b"%endif\0" as *const u8 as *const libc::c_char,
                6 as libc::c_int as libc::c_ulong,
            ) == 0 as libc::c_int
                && *(*__ctype_b_loc())
                    .offset(
                        *z.offset((i + 6 as libc::c_int) as isize) as libc::c_uchar
                            as libc::c_int as isize,
                    ) as libc::c_int
                    & _ISspace as libc::c_int as libc::c_ushort as libc::c_int != 0
            {
                if exclude != 0 {
                    exclude -= 1;
                    if exclude == 0 as libc::c_int {
                        j = start;
                        while j < i {
                            if *z.offset(j as isize) as libc::c_int != '\n' as i32 {
                                *z.offset(j as isize) = ' ' as i32 as libc::c_char;
                            }
                            j += 1;
                        }
                    }
                }
                j = i;
                while *z.offset(j as isize) as libc::c_int != 0
                    && *z.offset(j as isize) as libc::c_int != '\n' as i32
                {
                    *z.offset(j as isize) = ' ' as i32 as libc::c_char;
                    j += 1;
                }
            } else if strncmp(
                &mut *z.offset(i as isize),
                b"%else\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int as libc::c_ulong,
            ) == 0 as libc::c_int
                && *(*__ctype_b_loc())
                    .offset(
                        *z.offset((i + 5 as libc::c_int) as isize) as libc::c_uchar
                            as libc::c_int as isize,
                    ) as libc::c_int
                    & _ISspace as libc::c_int as libc::c_ushort as libc::c_int != 0
            {
                if exclude == 1 as libc::c_int {
                    exclude = 0 as libc::c_int;
                    j = start;
                    while j < i {
                        if *z.offset(j as isize) as libc::c_int != '\n' as i32 {
                            *z.offset(j as isize) = ' ' as i32 as libc::c_char;
                        }
                        j += 1;
                    }
                } else if exclude == 0 as libc::c_int {
                    exclude = 1 as libc::c_int;
                    start = i;
                    start_lineno = lineno;
                }
                j = i;
                while *z.offset(j as isize) as libc::c_int != 0
                    && *z.offset(j as isize) as libc::c_int != '\n' as i32
                {
                    *z.offset(j as isize) = ' ' as i32 as libc::c_char;
                    j += 1;
                }
            } else if strncmp(
                &mut *z.offset(i as isize),
                b"%ifdef \0" as *const u8 as *const libc::c_char,
                7 as libc::c_int as libc::c_ulong,
            ) == 0 as libc::c_int
                || strncmp(
                    &mut *z.offset(i as isize),
                    b"%if \0" as *const u8 as *const libc::c_char,
                    4 as libc::c_int as libc::c_ulong,
                ) == 0 as libc::c_int
                || strncmp(
                    &mut *z.offset(i as isize),
                    b"%ifndef \0" as *const u8 as *const libc::c_char,
                    8 as libc::c_int as libc::c_ulong,
                ) == 0 as libc::c_int
            {
                if exclude != 0 {
                    exclude += 1;
                } else {
                    let mut isNot: libc::c_int = 0;
                    let mut iBool: libc::c_int = 0;
                    j = i;
                    while *z.offset(j as isize) as libc::c_int != 0
                        && *(*__ctype_b_loc())
                            .offset(
                                *z.offset(j as isize) as libc::c_uchar as libc::c_int
                                    as isize,
                            ) as libc::c_int
                            & _ISspace as libc::c_int as libc::c_ushort as libc::c_int
                            == 0
                    {
                        j += 1;
                    }
                    iBool = j;
                    isNot = (j == i + 7 as libc::c_int) as libc::c_int;
                    while *z.offset(j as isize) as libc::c_int != 0
                        && *z.offset(j as isize) as libc::c_int != '\n' as i32
                    {
                        j += 1;
                    }
                    k = *z.offset(j as isize) as libc::c_int;
                    *z.offset(j as isize) = 0 as libc::c_int as libc::c_char;
                    exclude = eval_preprocessor_boolean(
                        &mut *z.offset(iBool as isize),
                        lineno,
                    );
                    *z.offset(j as isize) = k as libc::c_char;
                    if isNot == 0 {
                        exclude = (exclude == 0) as libc::c_int;
                    }
                    if exclude != 0 {
                        start = i;
                        start_lineno = lineno;
                    }
                }
                j = i;
                while *z.offset(j as isize) as libc::c_int != 0
                    && *z.offset(j as isize) as libc::c_int != '\n' as i32
                {
                    *z.offset(j as isize) = ' ' as i32 as libc::c_char;
                    j += 1;
                }
            }
        }
        i += 1;
    }
    if exclude != 0 {
        fprintf(
            stderr,
            b"unterminated %%ifdef starting on line %d\n\0" as *const u8
                as *const libc::c_char,
            start_lineno,
        );
        exit(1 as libc::c_int);
    }
}
#[no_mangle]
pub unsafe extern "C" fn Parse(mut gp: *mut lemon) {
    let mut ps: pstate = pstate {
        filename: 0 as *mut libc::c_char,
        tokenlineno: 0,
        errorcnt: 0,
        tokenstart: 0 as *mut libc::c_char,
        gp: 0 as *mut lemon,
        state: INITIALIZE,
        fallback: 0 as *mut symbol,
        tkclass: 0 as *mut symbol,
        lhs: 0 as *mut symbol,
        lhsalias: 0 as *const libc::c_char,
        nrhs: 0,
        rhs: [0 as *mut symbol; 1000],
        alias: [0 as *const libc::c_char; 1000],
        prevrule: 0 as *mut rule,
        declkeyword: 0 as *const libc::c_char,
        declargslot: 0 as *mut *mut libc::c_char,
        insertLineMacro: 0,
        decllinenoslot: 0 as *mut libc::c_int,
        declassoc: LEFT,
        preccounter: 0,
        firstrule: 0 as *mut rule,
        lastrule: 0 as *mut rule,
    };
    let mut fp: *mut FILE = 0 as *mut FILE;
    let mut filebuf: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut filesize: libc::c_uint = 0;
    let mut lineno: libc::c_int = 0;
    let mut c: libc::c_int = 0;
    let mut cp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut nextcp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut startline: libc::c_int = 0 as libc::c_int;
    memset(
        &mut ps as *mut pstate as *mut libc::c_void,
        '\0' as i32,
        ::std::mem::size_of::<pstate>() as libc::c_ulong,
    );
    ps.gp = gp;
    ps.filename = (*gp).filename;
    ps.errorcnt = 0 as libc::c_int;
    ps.state = INITIALIZE;
    fp = fopen(ps.filename, b"rb\0" as *const u8 as *const libc::c_char);
    if fp.is_null() {
        ErrorMsg(
            ps.filename,
            0 as libc::c_int,
            b"Can't open this file for reading.\0" as *const u8 as *const libc::c_char,
        );
        let ref mut fresh166 = (*gp).errorcnt;
        *fresh166 += 1;
        return;
    }
    fseek(fp, 0 as libc::c_int as libc::c_long, 2 as libc::c_int);
    filesize = ftell(fp) as libc::c_uint;
    rewind(fp);
    filebuf = malloc(
        filesize.wrapping_add(1 as libc::c_int as libc::c_uint) as libc::c_ulong,
    ) as *mut libc::c_char;
    if filesize > 100000000 as libc::c_int as libc::c_uint || filebuf.is_null() {
        ErrorMsg(
            ps.filename,
            0 as libc::c_int,
            b"Input file too large.\0" as *const u8 as *const libc::c_char,
        );
        free(filebuf as *mut libc::c_void);
        let ref mut fresh167 = (*gp).errorcnt;
        *fresh167 += 1;
        fclose(fp);
        return;
    }
    if fread(
        filebuf as *mut libc::c_void,
        1 as libc::c_int as libc::c_ulong,
        filesize as libc::c_ulong,
        fp,
    ) != filesize as libc::c_ulong
    {
        ErrorMsg(
            ps.filename,
            0 as libc::c_int,
            b"Can't read in all %d bytes of this file.\0" as *const u8
                as *const libc::c_char,
            filesize,
        );
        free(filebuf as *mut libc::c_void);
        let ref mut fresh168 = (*gp).errorcnt;
        *fresh168 += 1;
        fclose(fp);
        return;
    }
    fclose(fp);
    *filebuf.offset(filesize as isize) = 0 as libc::c_int as libc::c_char;
    preprocess_input(filebuf);
    if (*gp).printPreprocessed != 0 {
        printf(b"%s\n\0" as *const u8 as *const libc::c_char, filebuf);
        return;
    }
    lineno = 1 as libc::c_int;
    cp = filebuf;
    loop {
        c = *cp as libc::c_int;
        if !(c != 0 as libc::c_int) {
            break;
        }
        if c == '\n' as i32 {
            lineno += 1;
        }
        if *(*__ctype_b_loc()).offset(c as libc::c_uchar as libc::c_int as isize)
            as libc::c_int & _ISspace as libc::c_int as libc::c_ushort as libc::c_int
            != 0
        {
            cp = cp.offset(1);
        } else if c == '/' as i32
            && *cp.offset(1 as libc::c_int as isize) as libc::c_int == '/' as i32
        {
            cp = cp.offset(2 as libc::c_int as isize);
            loop {
                c = *cp as libc::c_int;
                if !(c != 0 as libc::c_int && c != '\n' as i32) {
                    break;
                }
                cp = cp.offset(1);
            }
        } else if c == '/' as i32
            && *cp.offset(1 as libc::c_int as isize) as libc::c_int == '*' as i32
        {
            cp = cp.offset(2 as libc::c_int as isize);
            if *cp as libc::c_int == '/' as i32 {
                cp = cp.offset(1);
            }
            loop {
                c = *cp as libc::c_int;
                if !(c != 0 as libc::c_int
                    && (c != '/' as i32
                        || *cp.offset(-(1 as libc::c_int) as isize) as libc::c_int
                            != '*' as i32))
                {
                    break;
                }
                if c == '\n' as i32 {
                    lineno += 1;
                }
                cp = cp.offset(1);
            }
            if c != 0 {
                cp = cp.offset(1);
            }
        } else {
            ps.tokenstart = cp;
            ps.tokenlineno = lineno;
            if c == '"' as i32 {
                cp = cp.offset(1);
                loop {
                    c = *cp as libc::c_int;
                    if !(c != 0 as libc::c_int && c != '"' as i32) {
                        break;
                    }
                    if c == '\n' as i32 {
                        lineno += 1;
                    }
                    cp = cp.offset(1);
                }
                if c == 0 as libc::c_int {
                    ErrorMsg(
                        ps.filename,
                        startline,
                        b"String starting on this line is not terminated before the end of the file.\0"
                            as *const u8 as *const libc::c_char,
                    );
                    ps.errorcnt += 1;
                    nextcp = cp;
                } else {
                    nextcp = cp.offset(1 as libc::c_int as isize);
                }
            } else if c == '{' as i32 {
                let mut level: libc::c_int = 0;
                cp = cp.offset(1);
                level = 1 as libc::c_int;
                loop {
                    c = *cp as libc::c_int;
                    if !(c != 0 as libc::c_int
                        && (level > 1 as libc::c_int || c != '}' as i32))
                    {
                        break;
                    }
                    if c == '\n' as i32 {
                        lineno += 1;
                    } else if c == '{' as i32 {
                        level += 1;
                    } else if c == '}' as i32 {
                        level -= 1;
                    } else if c == '/' as i32
                        && *cp.offset(1 as libc::c_int as isize) as libc::c_int
                            == '*' as i32
                    {
                        let mut prevc: libc::c_int = 0;
                        cp = &mut *cp.offset(2 as libc::c_int as isize)
                            as *mut libc::c_char;
                        prevc = 0 as libc::c_int;
                        loop {
                            c = *cp as libc::c_int;
                            if !(c != 0 as libc::c_int
                                && (c != '/' as i32 || prevc != '*' as i32))
                            {
                                break;
                            }
                            if c == '\n' as i32 {
                                lineno += 1;
                            }
                            prevc = c;
                            cp = cp.offset(1);
                        }
                    } else if c == '/' as i32
                        && *cp.offset(1 as libc::c_int as isize) as libc::c_int
                            == '/' as i32
                    {
                        cp = &mut *cp.offset(2 as libc::c_int as isize)
                            as *mut libc::c_char;
                        loop {
                            c = *cp as libc::c_int;
                            if !(c != 0 as libc::c_int && c != '\n' as i32) {
                                break;
                            }
                            cp = cp.offset(1);
                        }
                        if c != 0 {
                            lineno += 1;
                        }
                    } else if c == '\'' as i32 || c == '"' as i32 {
                        let mut startchar: libc::c_int = 0;
                        let mut prevc_0: libc::c_int = 0;
                        startchar = c;
                        prevc_0 = 0 as libc::c_int;
                        cp = cp.offset(1);
                        loop {
                            c = *cp as libc::c_int;
                            if !(c != 0 as libc::c_int
                                && (c != startchar || prevc_0 == '\\' as i32))
                            {
                                break;
                            }
                            if c == '\n' as i32 {
                                lineno += 1;
                            }
                            if prevc_0 == '\\' as i32 {
                                prevc_0 = 0 as libc::c_int;
                            } else {
                                prevc_0 = c;
                            }
                            cp = cp.offset(1);
                        }
                    }
                    cp = cp.offset(1);
                }
                if c == 0 as libc::c_int {
                    ErrorMsg(
                        ps.filename,
                        ps.tokenlineno,
                        b"C code starting on this line is not terminated before the end of the file.\0"
                            as *const u8 as *const libc::c_char,
                    );
                    ps.errorcnt += 1;
                    nextcp = cp;
                } else {
                    nextcp = cp.offset(1 as libc::c_int as isize);
                }
            } else if *(*__ctype_b_loc())
                .offset(c as libc::c_uchar as libc::c_int as isize) as libc::c_int
                & _ISalnum as libc::c_int as libc::c_ushort as libc::c_int != 0
            {
                loop {
                    c = *cp as libc::c_int;
                    if !(c != 0 as libc::c_int
                        && (*(*__ctype_b_loc())
                            .offset(c as libc::c_uchar as libc::c_int as isize)
                            as libc::c_int
                            & _ISalnum as libc::c_int as libc::c_ushort as libc::c_int
                            != 0 || c == '_' as i32))
                    {
                        break;
                    }
                    cp = cp.offset(1);
                }
                nextcp = cp;
            } else if c == ':' as i32
                && *cp.offset(1 as libc::c_int as isize) as libc::c_int == ':' as i32
                && *cp.offset(2 as libc::c_int as isize) as libc::c_int == '=' as i32
            {
                cp = cp.offset(3 as libc::c_int as isize);
                nextcp = cp;
            } else if (c == '/' as i32 || c == '|' as i32)
                && *(*__ctype_b_loc())
                    .offset(
                        *cp.offset(1 as libc::c_int as isize) as libc::c_uchar
                            as libc::c_int as isize,
                    ) as libc::c_int
                    & _ISalpha as libc::c_int as libc::c_ushort as libc::c_int != 0
            {
                cp = cp.offset(2 as libc::c_int as isize);
                loop {
                    c = *cp as libc::c_int;
                    if !(c != 0 as libc::c_int
                        && (*(*__ctype_b_loc())
                            .offset(c as libc::c_uchar as libc::c_int as isize)
                            as libc::c_int
                            & _ISalnum as libc::c_int as libc::c_ushort as libc::c_int
                            != 0 || c == '_' as i32))
                    {
                        break;
                    }
                    cp = cp.offset(1);
                }
                nextcp = cp;
            } else {
                cp = cp.offset(1);
                nextcp = cp;
            }
            c = *cp as libc::c_int;
            *cp = 0 as libc::c_int as libc::c_char;
            parseonetoken(&mut ps);
            *cp = c as libc::c_char;
            cp = nextcp;
        }
    }
    free(filebuf as *mut libc::c_void);
    let ref mut fresh169 = (*gp).rule;
    *fresh169 = ps.firstrule;
    (*gp).errorcnt = ps.errorcnt;
}
static mut plink_freelist: *mut plink = 0 as *const plink as *mut plink;
#[no_mangle]
pub unsafe extern "C" fn Plink_new() -> *mut plink {
    let mut newlink: *mut plink = 0 as *mut plink;
    if plink_freelist.is_null() {
        let mut i: libc::c_int = 0;
        let mut amt: libc::c_int = 100 as libc::c_int;
        plink_freelist = calloc(
            amt as libc::c_ulong,
            ::std::mem::size_of::<plink>() as libc::c_ulong,
        ) as *mut plink;
        if plink_freelist.is_null() {
            fprintf(
                stderr,
                b"Unable to allocate memory for a new follow-set propagation link.\n\0"
                    as *const u8 as *const libc::c_char,
            );
            exit(1 as libc::c_int);
        }
        i = 0 as libc::c_int;
        while i < amt - 1 as libc::c_int {
            let ref mut fresh170 = (*plink_freelist.offset(i as isize)).next;
            *fresh170 = &mut *plink_freelist.offset((i + 1 as libc::c_int) as isize)
                as *mut plink;
            i += 1;
        }
        let ref mut fresh171 = (*plink_freelist
            .offset((amt - 1 as libc::c_int) as isize))
            .next;
        *fresh171 = 0 as *mut plink;
    }
    newlink = plink_freelist;
    plink_freelist = (*plink_freelist).next;
    return newlink;
}
#[no_mangle]
pub unsafe extern "C" fn Plink_add(mut plpp: *mut *mut plink, mut cfp: *mut config) {
    let mut newlink: *mut plink = 0 as *mut plink;
    newlink = Plink_new();
    let ref mut fresh172 = (*newlink).next;
    *fresh172 = *plpp;
    *plpp = newlink;
    let ref mut fresh173 = (*newlink).cfp;
    *fresh173 = cfp;
}
#[no_mangle]
pub unsafe extern "C" fn Plink_copy(mut to: *mut *mut plink, mut from: *mut plink) {
    let mut nextpl: *mut plink = 0 as *mut plink;
    while !from.is_null() {
        nextpl = (*from).next;
        let ref mut fresh174 = (*from).next;
        *fresh174 = *to;
        *to = from;
        from = nextpl;
    }
}
#[no_mangle]
pub unsafe extern "C" fn Plink_delete(mut plp: *mut plink) {
    let mut nextpl: *mut plink = 0 as *mut plink;
    while !plp.is_null() {
        nextpl = (*plp).next;
        let ref mut fresh175 = (*plp).next;
        *fresh175 = plink_freelist;
        plink_freelist = plp;
        plp = nextpl;
    }
}
#[no_mangle]
pub unsafe extern "C" fn file_makename(
    mut lemp: *mut lemon,
    mut suffix: *const libc::c_char,
) -> *mut libc::c_char {
    let mut name: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut cp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut filename: *mut libc::c_char = (*lemp).filename;
    let mut sz: libc::c_int = 0;
    if !outputDir.is_null() {
        cp = strrchr(filename, '/' as i32);
        if !cp.is_null() {
            filename = cp.offset(1 as libc::c_int as isize);
        }
    }
    sz = strlen(filename) as libc::c_int;
    sz += strlen(suffix) as libc::c_int;
    if !outputDir.is_null() {
        sz += strlen(outputDir) as libc::c_int + 1 as libc::c_int;
    }
    sz += 5 as libc::c_int;
    name = malloc(sz as libc::c_ulong) as *mut libc::c_char;
    if name.is_null() {
        fprintf(
            stderr,
            b"Can't allocate space for a filename.\n\0" as *const u8
                as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    *name.offset(0 as libc::c_int as isize) = 0 as libc::c_int as libc::c_char;
    if !outputDir.is_null() {
        lemon_strcpy(name, outputDir);
        lemon_strcat(name, b"/\0" as *const u8 as *const libc::c_char);
    }
    lemon_strcat(name, filename);
    cp = strrchr(name, '.' as i32);
    if !cp.is_null() {
        *cp = 0 as libc::c_int as libc::c_char;
    }
    lemon_strcat(name, suffix);
    return name;
}
#[no_mangle]
pub unsafe extern "C" fn file_open(
    mut lemp: *mut lemon,
    mut suffix: *const libc::c_char,
    mut mode: *const libc::c_char,
) -> *mut FILE {
    let mut fp: *mut FILE = 0 as *mut FILE;
    if !((*lemp).outname).is_null() {
        free((*lemp).outname as *mut libc::c_void);
    }
    let ref mut fresh176 = (*lemp).outname;
    *fresh176 = file_makename(lemp, suffix);
    fp = fopen((*lemp).outname, mode);
    if fp.is_null() && *mode as libc::c_int == 'w' as i32 {
        fprintf(
            stderr,
            b"Can't open file \"%s\".\n\0" as *const u8 as *const libc::c_char,
            (*lemp).outname,
        );
        let ref mut fresh177 = (*lemp).errorcnt;
        *fresh177 += 1;
        return 0 as *mut FILE;
    }
    return fp;
}
#[no_mangle]
pub unsafe extern "C" fn rule_print(mut out: *mut FILE, mut rp: *mut rule) {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    fprintf(out, b"%s\0" as *const u8 as *const libc::c_char, (*(*rp).lhs).name);
    fprintf(out, b" ::=\0" as *const u8 as *const libc::c_char);
    i = 0 as libc::c_int;
    while i < (*rp).nrhs {
        let mut sp: *mut symbol = *((*rp).rhs).offset(i as isize);
        if (*sp).type_0 as libc::c_uint == MULTITERMINAL as libc::c_int as libc::c_uint {
            fprintf(
                out,
                b" %s\0" as *const u8 as *const libc::c_char,
                (**((*sp).subsym).offset(0 as libc::c_int as isize)).name,
            );
            j = 1 as libc::c_int;
            while j < (*sp).nsubsym {
                fprintf(
                    out,
                    b"|%s\0" as *const u8 as *const libc::c_char,
                    (**((*sp).subsym).offset(j as isize)).name,
                );
                j += 1;
            }
        } else {
            fprintf(out, b" %s\0" as *const u8 as *const libc::c_char, (*sp).name);
        }
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn Reprint(mut lemp: *mut lemon) {
    let mut rp: *mut rule = 0 as *mut rule;
    let mut sp: *mut symbol = 0 as *mut symbol;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut maxlen: libc::c_int = 0;
    let mut len: libc::c_int = 0;
    let mut ncolumns: libc::c_int = 0;
    let mut skip: libc::c_int = 0;
    printf(
        b"// Reprint of input file \"%s\".\n// Symbols:\n\0" as *const u8
            as *const libc::c_char,
        (*lemp).filename,
    );
    maxlen = 10 as libc::c_int;
    i = 0 as libc::c_int;
    while i < (*lemp).nsymbol {
        sp = *((*lemp).symbols).offset(i as isize);
        len = strlen((*sp).name) as libc::c_int;
        if len > maxlen {
            maxlen = len;
        }
        i += 1;
    }
    ncolumns = 76 as libc::c_int / (maxlen + 5 as libc::c_int);
    if ncolumns < 1 as libc::c_int {
        ncolumns = 1 as libc::c_int;
    }
    skip = ((*lemp).nsymbol + ncolumns - 1 as libc::c_int) / ncolumns;
    i = 0 as libc::c_int;
    while i < skip {
        printf(b"//\0" as *const u8 as *const libc::c_char);
        j = i;
        while j < (*lemp).nsymbol {
            sp = *((*lemp).symbols).offset(j as isize);
            if (*sp).index == j {} else {
                __assert_fail(
                    b"sp->index==j\0" as *const u8 as *const libc::c_char,
                    b"/home/me/github/learnrust/libsql/tool/lemon.c\0" as *const u8
                        as *const libc::c_char,
                    3273 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 29],
                        &[libc::c_char; 29],
                    >(b"void Reprint(struct lemon *)\0"))
                        .as_ptr(),
                );
            }
            printf(
                b" %3d %-*.*s\0" as *const u8 as *const libc::c_char,
                j,
                maxlen,
                maxlen,
                (*sp).name,
            );
            j += skip;
        }
        printf(b"\n\0" as *const u8 as *const libc::c_char);
        i += 1;
    }
    rp = (*lemp).rule;
    while !rp.is_null() {
        rule_print(stdout, rp);
        printf(b".\0" as *const u8 as *const libc::c_char);
        if !((*rp).precsym).is_null() {
            printf(
                b" [%s]\0" as *const u8 as *const libc::c_char,
                (*(*rp).precsym).name,
            );
        }
        printf(b"\n\0" as *const u8 as *const libc::c_char);
        rp = (*rp).next;
    }
}
#[no_mangle]
pub unsafe extern "C" fn RulePrint(
    mut fp: *mut FILE,
    mut rp: *mut rule,
    mut iCursor: libc::c_int,
) {
    let mut sp: *mut symbol = 0 as *mut symbol;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    fprintf(fp, b"%s ::=\0" as *const u8 as *const libc::c_char, (*(*rp).lhs).name);
    i = 0 as libc::c_int;
    while i <= (*rp).nrhs {
        if i == iCursor {
            fprintf(fp, b" *\0" as *const u8 as *const libc::c_char);
        }
        if i == (*rp).nrhs {
            break;
        }
        sp = *((*rp).rhs).offset(i as isize);
        if (*sp).type_0 as libc::c_uint == MULTITERMINAL as libc::c_int as libc::c_uint {
            fprintf(
                fp,
                b" %s\0" as *const u8 as *const libc::c_char,
                (**((*sp).subsym).offset(0 as libc::c_int as isize)).name,
            );
            j = 1 as libc::c_int;
            while j < (*sp).nsubsym {
                fprintf(
                    fp,
                    b"|%s\0" as *const u8 as *const libc::c_char,
                    (**((*sp).subsym).offset(j as isize)).name,
                );
                j += 1;
            }
        } else {
            fprintf(fp, b" %s\0" as *const u8 as *const libc::c_char, (*sp).name);
        }
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn ConfigPrint(mut fp: *mut FILE, mut cfp: *mut config) {
    RulePrint(fp, (*cfp).rp, (*cfp).dot);
}
#[no_mangle]
pub unsafe extern "C" fn PrintAction(
    mut ap: *mut action,
    mut fp: *mut FILE,
    mut indent: libc::c_int,
) -> libc::c_int {
    let mut result: libc::c_int = 1 as libc::c_int;
    match (*ap).type_0 as libc::c_uint {
        0 => {
            let mut stp: *mut state = (*ap).x.stp;
            fprintf(
                fp,
                b"%*s shift        %-7d\0" as *const u8 as *const libc::c_char,
                indent,
                (*(*ap).sp).name,
                (*stp).statenum,
            );
        }
        2 => {
            let mut rp: *mut rule = (*ap).x.rp;
            fprintf(
                fp,
                b"%*s reduce       %-7d\0" as *const u8 as *const libc::c_char,
                indent,
                (*(*ap).sp).name,
                (*rp).iRule,
            );
            RulePrint(fp, rp, -(1 as libc::c_int));
        }
        10 => {
            let mut rp_0: *mut rule = (*ap).x.rp;
            fprintf(
                fp,
                b"%*s shift-reduce %-7d\0" as *const u8 as *const libc::c_char,
                indent,
                (*(*ap).sp).name,
                (*rp_0).iRule,
            );
            RulePrint(fp, rp_0, -(1 as libc::c_int));
        }
        1 => {
            fprintf(
                fp,
                b"%*s accept\0" as *const u8 as *const libc::c_char,
                indent,
                (*(*ap).sp).name,
            );
        }
        3 => {
            fprintf(
                fp,
                b"%*s error\0" as *const u8 as *const libc::c_char,
                indent,
                (*(*ap).sp).name,
            );
        }
        5 | 6 => {
            fprintf(
                fp,
                b"%*s reduce       %-7d ** Parsing conflict **\0" as *const u8
                    as *const libc::c_char,
                indent,
                (*(*ap).sp).name,
                (*(*ap).x.rp).iRule,
            );
        }
        4 => {
            fprintf(
                fp,
                b"%*s shift        %-7d ** Parsing conflict **\0" as *const u8
                    as *const libc::c_char,
                indent,
                (*(*ap).sp).name,
                (*(*ap).x.stp).statenum,
            );
        }
        7 => {
            if showPrecedenceConflict != 0 {
                fprintf(
                    fp,
                    b"%*s shift        %-7d -- dropped by precedence\0" as *const u8
                        as *const libc::c_char,
                    indent,
                    (*(*ap).sp).name,
                    (*(*ap).x.stp).statenum,
                );
            } else {
                result = 0 as libc::c_int;
            }
        }
        8 => {
            if showPrecedenceConflict != 0 {
                fprintf(
                    fp,
                    b"%*s reduce %-7d -- dropped by precedence\0" as *const u8
                        as *const libc::c_char,
                    indent,
                    (*(*ap).sp).name,
                    (*(*ap).x.rp).iRule,
                );
            } else {
                result = 0 as libc::c_int;
            }
        }
        9 => {
            result = 0 as libc::c_int;
        }
        _ => {}
    }
    if result != 0 && !((*ap).spOpt).is_null() {
        fprintf(
            fp,
            b"  /* because %s==%s */\0" as *const u8 as *const libc::c_char,
            (*(*ap).sp).name,
            (*(*ap).spOpt).name,
        );
    }
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn ReportOutput(mut lemp: *mut lemon) {
    let mut i: libc::c_int = 0;
    let mut n: libc::c_int = 0;
    let mut stp: *mut state = 0 as *mut state;
    let mut cfp: *mut config = 0 as *mut config;
    let mut ap: *mut action = 0 as *mut action;
    let mut rp: *mut rule = 0 as *mut rule;
    let mut fp: *mut FILE = 0 as *mut FILE;
    fp = file_open(
        lemp,
        b".out\0" as *const u8 as *const libc::c_char,
        b"wb\0" as *const u8 as *const libc::c_char,
    );
    if fp.is_null() {
        return;
    }
    i = 0 as libc::c_int;
    while i < (*lemp).nxstate {
        stp = *((*lemp).sorted).offset(i as isize);
        fprintf(
            fp,
            b"State %d:\n\0" as *const u8 as *const libc::c_char,
            (*stp).statenum,
        );
        if (*lemp).basisflag != 0 {
            cfp = (*stp).bp;
        } else {
            cfp = (*stp).cfp;
        }
        while !cfp.is_null() {
            let mut buf: [libc::c_char; 20] = [0; 20];
            if (*cfp).dot == (*(*cfp).rp).nrhs {
                lemon_sprintf(
                    buf.as_mut_ptr(),
                    b"(%d)\0" as *const u8 as *const libc::c_char,
                    (*(*cfp).rp).iRule,
                );
                fprintf(
                    fp,
                    b"    %5s \0" as *const u8 as *const libc::c_char,
                    buf.as_mut_ptr(),
                );
            } else {
                fprintf(fp, b"          \0" as *const u8 as *const libc::c_char);
            }
            ConfigPrint(fp, cfp);
            fprintf(fp, b"\n\0" as *const u8 as *const libc::c_char);
            if (*lemp).basisflag != 0 {
                cfp = (*cfp).bp;
            } else {
                cfp = (*cfp).next;
            }
        }
        fprintf(fp, b"\n\0" as *const u8 as *const libc::c_char);
        ap = (*stp).ap;
        while !ap.is_null() {
            if PrintAction(ap, fp, 30 as libc::c_int) != 0 {
                fprintf(fp, b"\n\0" as *const u8 as *const libc::c_char);
            }
            ap = (*ap).next;
        }
        fprintf(fp, b"\n\0" as *const u8 as *const libc::c_char);
        i += 1;
    }
    fprintf(
        fp,
        b"----------------------------------------------------\n\0" as *const u8
            as *const libc::c_char,
    );
    fprintf(fp, b"Symbols:\n\0" as *const u8 as *const libc::c_char);
    fprintf(
        fp,
        b"The first-set of non-terminals is shown after the name.\n\n\0" as *const u8
            as *const libc::c_char,
    );
    i = 0 as libc::c_int;
    while i < (*lemp).nsymbol {
        let mut j: libc::c_int = 0;
        let mut sp: *mut symbol = 0 as *mut symbol;
        sp = *((*lemp).symbols).offset(i as isize);
        fprintf(fp, b"  %3d: %s\0" as *const u8 as *const libc::c_char, i, (*sp).name);
        if (*sp).type_0 as libc::c_uint == NONTERMINAL as libc::c_int as libc::c_uint {
            fprintf(fp, b":\0" as *const u8 as *const libc::c_char);
            if (*sp).lambda as u64 != 0 {
                fprintf(fp, b" <lambda>\0" as *const u8 as *const libc::c_char);
            }
            j = 0 as libc::c_int;
            while j < (*lemp).nterminal {
                if !((*sp).firstset).is_null()
                    && *((*sp).firstset).offset(j as isize) as libc::c_int != 0
                {
                    fprintf(
                        fp,
                        b" %s\0" as *const u8 as *const libc::c_char,
                        (**((*lemp).symbols).offset(j as isize)).name,
                    );
                }
                j += 1;
            }
        }
        if (*sp).prec >= 0 as libc::c_int {
            fprintf(
                fp,
                b" (precedence=%d)\0" as *const u8 as *const libc::c_char,
                (*sp).prec,
            );
        }
        fprintf(fp, b"\n\0" as *const u8 as *const libc::c_char);
        i += 1;
    }
    fprintf(
        fp,
        b"----------------------------------------------------\n\0" as *const u8
            as *const libc::c_char,
    );
    fprintf(fp, b"Syntax-only Symbols:\n\0" as *const u8 as *const libc::c_char);
    fprintf(
        fp,
        b"The following symbols never carry semantic content.\n\n\0" as *const u8
            as *const libc::c_char,
    );
    n = 0 as libc::c_int;
    i = n;
    while i < (*lemp).nsymbol {
        let mut w: libc::c_int = 0;
        let mut sp_0: *mut symbol = *((*lemp).symbols).offset(i as isize);
        if !((*sp_0).bContent != 0) {
            w = strlen((*sp_0).name) as libc::c_int;
            if n > 0 as libc::c_int && n + w > 75 as libc::c_int {
                fprintf(fp, b"\n\0" as *const u8 as *const libc::c_char);
                n = 0 as libc::c_int;
            }
            if n > 0 as libc::c_int {
                fprintf(fp, b" \0" as *const u8 as *const libc::c_char);
                n += 1;
            }
            fprintf(fp, b"%s\0" as *const u8 as *const libc::c_char, (*sp_0).name);
            n += w;
        }
        i += 1;
    }
    if n > 0 as libc::c_int {
        fprintf(fp, b"\n\0" as *const u8 as *const libc::c_char);
    }
    fprintf(
        fp,
        b"----------------------------------------------------\n\0" as *const u8
            as *const libc::c_char,
    );
    fprintf(fp, b"Rules:\n\0" as *const u8 as *const libc::c_char);
    rp = (*lemp).rule;
    while !rp.is_null() {
        fprintf(fp, b"%4d: \0" as *const u8 as *const libc::c_char, (*rp).iRule);
        rule_print(fp, rp);
        fprintf(fp, b".\0" as *const u8 as *const libc::c_char);
        if !((*rp).precsym).is_null() {
            fprintf(
                fp,
                b" [%s precedence=%d]\0" as *const u8 as *const libc::c_char,
                (*(*rp).precsym).name,
                (*(*rp).precsym).prec,
            );
        }
        fprintf(fp, b"\n\0" as *const u8 as *const libc::c_char);
        rp = (*rp).next;
    }
    fclose(fp);
}
#[no_mangle]
pub unsafe extern "C" fn pathsearch(
    mut argv0: *mut libc::c_char,
    mut name: *mut libc::c_char,
    mut modemask: libc::c_int,
) -> *mut libc::c_char {
    let mut pathlist: *const libc::c_char = 0 as *const libc::c_char;
    let mut pathbufptr: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut pathbuf: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut path: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut cp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut c: libc::c_char = 0;
    cp = strrchr(argv0, '/' as i32);
    if !cp.is_null() {
        c = *cp;
        *cp = 0 as libc::c_int as libc::c_char;
        path = malloc(
            (strlen(argv0) as libc::c_int + strlen(name) as libc::c_int
                + 2 as libc::c_int) as libc::c_ulong,
        ) as *mut libc::c_char;
        if !path.is_null() {
            lemon_sprintf(
                path,
                b"%s/%s\0" as *const u8 as *const libc::c_char,
                argv0,
                name,
            );
        }
        *cp = c;
    } else {
        pathlist = getenv(b"PATH\0" as *const u8 as *const libc::c_char);
        if pathlist.is_null() {
            pathlist = b".:/bin:/usr/bin\0" as *const u8 as *const libc::c_char;
        }
        pathbuf = malloc(
            (strlen(pathlist) as libc::c_int + 1 as libc::c_int) as libc::c_ulong,
        ) as *mut libc::c_char;
        path = malloc(
            (strlen(pathlist) as libc::c_int + strlen(name) as libc::c_int
                + 2 as libc::c_int) as libc::c_ulong,
        ) as *mut libc::c_char;
        if !pathbuf.is_null() && !path.is_null() {
            pathbufptr = pathbuf;
            lemon_strcpy(pathbuf, pathlist);
            while *pathbuf != 0 {
                cp = strchr(pathbuf, ':' as i32);
                if cp.is_null() {
                    cp = &mut *pathbuf
                        .offset(
                            (strlen
                                as unsafe extern "C" fn(
                                    *const libc::c_char,
                                ) -> libc::c_ulong)(pathbuf) as libc::c_int as isize,
                        ) as *mut libc::c_char;
                }
                c = *cp;
                *cp = 0 as libc::c_int as libc::c_char;
                lemon_sprintf(
                    path,
                    b"%s/%s\0" as *const u8 as *const libc::c_char,
                    pathbuf,
                    name,
                );
                *cp = c;
                if c as libc::c_int == 0 as libc::c_int {
                    *pathbuf
                        .offset(
                            0 as libc::c_int as isize,
                        ) = 0 as libc::c_int as libc::c_char;
                } else {
                    pathbuf = &mut *cp.offset(1 as libc::c_int as isize)
                        as *mut libc::c_char;
                }
                if access(path, modemask) == 0 as libc::c_int {
                    break;
                }
            }
        }
        free(pathbufptr as *mut libc::c_void);
    }
    return path;
}
#[no_mangle]
pub unsafe extern "C" fn compute_action(
    mut lemp: *mut lemon,
    mut ap: *mut action,
) -> libc::c_int {
    let mut act: libc::c_int = 0;
    match (*ap).type_0 as libc::c_uint {
        0 => {
            act = (*(*ap).x.stp).statenum;
        }
        10 => {
            if (*(*ap).sp).index >= (*lemp).nterminal
                && (((*lemp).errsym).is_null()
                    || (*(*ap).sp).index != (*(*lemp).errsym).index)
            {
                act = (*lemp).minReduce + (*(*ap).x.rp).iRule;
            } else {
                act = (*lemp).minShiftReduce + (*(*ap).x.rp).iRule;
            }
        }
        2 => {
            act = (*lemp).minReduce + (*(*ap).x.rp).iRule;
        }
        3 => {
            act = (*lemp).errAction;
        }
        1 => {
            act = (*lemp).accAction;
        }
        _ => {
            act = -(1 as libc::c_int);
        }
    }
    return act;
}
#[no_mangle]
pub unsafe extern "C" fn tplt_xfer(
    mut name: *mut libc::c_char,
    mut in_0: *mut FILE,
    mut out: *mut FILE,
    mut lineno: *mut libc::c_int,
) {
    let mut i: libc::c_int = 0;
    let mut iStart: libc::c_int = 0;
    let mut line: [libc::c_char; 1000] = [0; 1000];
    while !(fgets(line.as_mut_ptr(), 1000 as libc::c_int, in_0)).is_null()
        && (line[0 as libc::c_int as usize] as libc::c_int != '%' as i32
            || line[1 as libc::c_int as usize] as libc::c_int != '%' as i32)
    {
        *lineno += 1;
        iStart = 0 as libc::c_int;
        if !name.is_null() {
            i = 0 as libc::c_int;
            while line[i as usize] != 0 {
                if line[i as usize] as libc::c_int == 'P' as i32
                    && strncmp(
                        &mut *line.as_mut_ptr().offset(i as isize),
                        b"Parse\0" as *const u8 as *const libc::c_char,
                        5 as libc::c_int as libc::c_ulong,
                    ) == 0 as libc::c_int
                    && (i == 0 as libc::c_int
                        || *(*__ctype_b_loc())
                            .offset(
                                line[(i - 1 as libc::c_int) as usize] as libc::c_uchar
                                    as libc::c_int as isize,
                            ) as libc::c_int
                            & _ISalpha as libc::c_int as libc::c_ushort as libc::c_int
                            == 0)
                {
                    if i > iStart {
                        fprintf(
                            out,
                            b"%.*s\0" as *const u8 as *const libc::c_char,
                            i - iStart,
                            &mut *line.as_mut_ptr().offset(iStart as isize)
                                as *mut libc::c_char,
                        );
                    }
                    fprintf(out, b"%s\0" as *const u8 as *const libc::c_char, name);
                    i += 4 as libc::c_int;
                    iStart = i + 1 as libc::c_int;
                }
                i += 1;
            }
        }
        fprintf(
            out,
            b"%s\0" as *const u8 as *const libc::c_char,
            &mut *line.as_mut_ptr().offset(iStart as isize) as *mut libc::c_char,
        );
    }
}
#[no_mangle]
pub unsafe extern "C" fn tplt_skip_header(
    mut in_0: *mut FILE,
    mut lineno: *mut libc::c_int,
) {
    let mut line: [libc::c_char; 1000] = [0; 1000];
    while !(fgets(line.as_mut_ptr(), 1000 as libc::c_int, in_0)).is_null()
        && (line[0 as libc::c_int as usize] as libc::c_int != '%' as i32
            || line[1 as libc::c_int as usize] as libc::c_int != '%' as i32)
    {
        *lineno += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn tplt_open(mut lemp: *mut lemon) -> *mut FILE {
    static mut templatename: [libc::c_char; 9] = unsafe {
        *::std::mem::transmute::<&[u8; 9], &mut [libc::c_char; 9]>(b"lempar.c\0")
    };
    let mut buf: [libc::c_char; 1000] = [0; 1000];
    let mut in_0: *mut FILE = 0 as *mut FILE;
    let mut tpltname: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut toFree: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut cp: *mut libc::c_char = 0 as *mut libc::c_char;
    if !user_templatename.is_null() {
        if access(user_templatename, 0o4 as libc::c_int) == -(1 as libc::c_int) {
            fprintf(
                stderr,
                b"Can't find the parser driver template file \"%s\".\n\0" as *const u8
                    as *const libc::c_char,
                user_templatename,
            );
            let ref mut fresh178 = (*lemp).errorcnt;
            *fresh178 += 1;
            return 0 as *mut FILE;
        }
        in_0 = fopen(user_templatename, b"rb\0" as *const u8 as *const libc::c_char);
        if in_0.is_null() {
            fprintf(
                stderr,
                b"Can't open the template file \"%s\".\n\0" as *const u8
                    as *const libc::c_char,
                user_templatename,
            );
            let ref mut fresh179 = (*lemp).errorcnt;
            *fresh179 += 1;
            return 0 as *mut FILE;
        }
        return in_0;
    }
    cp = strrchr((*lemp).filename, '.' as i32);
    if !cp.is_null() {
        lemon_sprintf(
            buf.as_mut_ptr(),
            b"%.*s.lt\0" as *const u8 as *const libc::c_char,
            cp.offset_from((*lemp).filename) as libc::c_long as libc::c_int,
            (*lemp).filename,
        );
    } else {
        lemon_sprintf(
            buf.as_mut_ptr(),
            b"%s.lt\0" as *const u8 as *const libc::c_char,
            (*lemp).filename,
        );
    }
    if access(buf.as_mut_ptr(), 0o4 as libc::c_int) == 0 as libc::c_int {
        tpltname = buf.as_mut_ptr();
    } else if access(templatename.as_mut_ptr(), 0o4 as libc::c_int) == 0 as libc::c_int {
        tpltname = templatename.as_mut_ptr();
    } else {
        tpltname = pathsearch(
            (*lemp).argv0,
            templatename.as_mut_ptr(),
            0 as libc::c_int,
        );
        toFree = tpltname;
    }
    if tpltname.is_null() {
        fprintf(
            stderr,
            b"Can't find the parser driver template file \"%s\".\n\0" as *const u8
                as *const libc::c_char,
            templatename.as_mut_ptr(),
        );
        let ref mut fresh180 = (*lemp).errorcnt;
        *fresh180 += 1;
        return 0 as *mut FILE;
    }
    in_0 = fopen(tpltname, b"rb\0" as *const u8 as *const libc::c_char);
    if in_0.is_null() {
        fprintf(
            stderr,
            b"Can't open the template file \"%s\".\n\0" as *const u8
                as *const libc::c_char,
            tpltname,
        );
        let ref mut fresh181 = (*lemp).errorcnt;
        *fresh181 += 1;
    }
    free(toFree as *mut libc::c_void);
    return in_0;
}
#[no_mangle]
pub unsafe extern "C" fn tplt_linedir(
    mut out: *mut FILE,
    mut lineno: libc::c_int,
    mut filename: *mut libc::c_char,
) {
    fprintf(out, b"#line %d \"\0" as *const u8 as *const libc::c_char, lineno);
    while *filename != 0 {
        if *filename as libc::c_int == '\\' as i32 {
            putc('\\' as i32, out);
        }
        putc(*filename as libc::c_int, out);
        filename = filename.offset(1);
    }
    fprintf(out, b"\"\n\0" as *const u8 as *const libc::c_char);
}
#[no_mangle]
pub unsafe extern "C" fn tplt_print(
    mut out: *mut FILE,
    mut lemp: *mut lemon,
    mut str: *mut libc::c_char,
    mut lineno: *mut libc::c_int,
) {
    if str.is_null() {
        return;
    }
    while *str != 0 {
        putc(*str as libc::c_int, out);
        if *str as libc::c_int == '\n' as i32 {
            *lineno += 1;
        }
        str = str.offset(1);
    }
    if *str.offset(-(1 as libc::c_int) as isize) as libc::c_int != '\n' as i32 {
        putc('\n' as i32, out);
        *lineno += 1;
    }
    if (*lemp).nolinenosflag == 0 {
        *lineno += 1;
        tplt_linedir(out, *lineno, (*lemp).outname);
    }
}
#[no_mangle]
pub unsafe extern "C" fn emit_destructor_code(
    mut out: *mut FILE,
    mut sp: *mut symbol,
    mut lemp: *mut lemon,
    mut lineno: *mut libc::c_int,
) {
    let mut cp: *mut libc::c_char = 0 as *mut libc::c_char;
    if (*sp).type_0 as libc::c_uint == TERMINAL as libc::c_int as libc::c_uint {
        cp = (*lemp).tokendest;
        if cp.is_null() {
            return;
        }
        fprintf(out, b"{\n\0" as *const u8 as *const libc::c_char);
        *lineno += 1;
    } else if !((*sp).destructor).is_null() {
        cp = (*sp).destructor;
        fprintf(out, b"{\n\0" as *const u8 as *const libc::c_char);
        *lineno += 1;
        if (*lemp).nolinenosflag == 0 {
            *lineno += 1;
            tplt_linedir(out, (*sp).destLineno, (*lemp).filename);
        }
    } else if !((*lemp).vardest).is_null() {
        cp = (*lemp).vardest;
        if cp.is_null() {
            return;
        }
        fprintf(out, b"{\n\0" as *const u8 as *const libc::c_char);
        *lineno += 1;
    } else {
        __assert_fail(
            b"0\0" as *const u8 as *const libc::c_char,
            b"/home/me/github/learnrust/libsql/tool/lemon.c\0" as *const u8
                as *const libc::c_char,
            3753 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 74],
                &[libc::c_char; 74],
            >(
                b"void emit_destructor_code(FILE *, struct symbol *, struct lemon *, int *)\0",
            ))
                .as_ptr(),
        );
    }
    while *cp != 0 {
        if *cp as libc::c_int == '$' as i32
            && *cp.offset(1 as libc::c_int as isize) as libc::c_int == '$' as i32
        {
            fprintf(
                out,
                b"(yypminor->yy%d)\0" as *const u8 as *const libc::c_char,
                (*sp).dtnum,
            );
            cp = cp.offset(1);
        } else {
            if *cp as libc::c_int == '\n' as i32 {
                *lineno += 1;
            }
            fputc(*cp as libc::c_int, out);
        }
        cp = cp.offset(1);
    }
    fprintf(out, b"\n\0" as *const u8 as *const libc::c_char);
    *lineno += 1;
    if (*lemp).nolinenosflag == 0 {
        *lineno += 1;
        tplt_linedir(out, *lineno, (*lemp).outname);
    }
    fprintf(out, b"}\n\0" as *const u8 as *const libc::c_char);
    *lineno += 1;
}
#[no_mangle]
pub unsafe extern "C" fn has_destructor(
    mut sp: *mut symbol,
    mut lemp: *mut lemon,
) -> libc::c_int {
    let mut ret: libc::c_int = 0;
    if (*sp).type_0 as libc::c_uint == TERMINAL as libc::c_int as libc::c_uint {
        ret = ((*lemp).tokendest != 0 as *mut libc::c_char) as libc::c_int;
    } else {
        ret = (!((*lemp).vardest).is_null() || !((*sp).destructor).is_null())
            as libc::c_int;
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn append_str(
    mut zText: *const libc::c_char,
    mut n: libc::c_int,
    mut p1: libc::c_int,
    mut p2: libc::c_int,
) -> *mut libc::c_char {
    static mut empty: [libc::c_char; 1] = [0 as libc::c_int as libc::c_char];
    static mut z: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
    static mut alloced: libc::c_int = 0 as libc::c_int;
    static mut used: libc::c_int = 0 as libc::c_int;
    let mut c: libc::c_int = 0;
    let mut zInt: [libc::c_char; 40] = [0; 40];
    if zText.is_null() {
        if used == 0 as libc::c_int && !z.is_null() {
            *z.offset(0 as libc::c_int as isize) = 0 as libc::c_int as libc::c_char;
        }
        used = 0 as libc::c_int;
        return z;
    }
    if n <= 0 as libc::c_int {
        if n < 0 as libc::c_int {
            used += n;
            if used >= 0 as libc::c_int {} else {
                __assert_fail(
                    b"used>=0\0" as *const u8 as *const libc::c_char,
                    b"/home/me/github/learnrust/libsql/tool/lemon.c\0" as *const u8
                        as *const libc::c_char,
                    3813 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 46],
                        &[libc::c_char; 46],
                    >(b"char *append_str(const char *, int, int, int)\0"))
                        .as_ptr(),
                );
            }
        }
        n = strlen(zText) as libc::c_int;
    }
    if (n as libc::c_ulong)
        .wrapping_add(
            (::std::mem::size_of::<[libc::c_char; 40]>() as libc::c_ulong)
                .wrapping_mul(2 as libc::c_int as libc::c_ulong),
        )
        .wrapping_add(used as libc::c_ulong) as libc::c_int >= alloced
    {
        alloced = (n as libc::c_ulong)
            .wrapping_add(
                (::std::mem::size_of::<[libc::c_char; 40]>() as libc::c_ulong)
                    .wrapping_mul(2 as libc::c_int as libc::c_ulong),
            )
            .wrapping_add(used as libc::c_ulong)
            .wrapping_add(200 as libc::c_int as libc::c_ulong) as libc::c_int;
        z = realloc(z as *mut libc::c_void, alloced as libc::c_ulong)
            as *mut libc::c_char;
    }
    if z.is_null() {
        return empty.as_mut_ptr();
    }
    loop {
        let fresh182 = n;
        n = n - 1;
        if !(fresh182 > 0 as libc::c_int) {
            break;
        }
        let fresh183 = zText;
        zText = zText.offset(1);
        c = *fresh183 as libc::c_int;
        if c == '%' as i32 && n > 0 as libc::c_int
            && *zText.offset(0 as libc::c_int as isize) as libc::c_int == 'd' as i32
        {
            lemon_sprintf(
                zInt.as_mut_ptr(),
                b"%d\0" as *const u8 as *const libc::c_char,
                p1,
            );
            p1 = p2;
            lemon_strcpy(&mut *z.offset(used as isize), zInt.as_mut_ptr());
            used += strlen(&mut *z.offset(used as isize)) as libc::c_int;
            zText = zText.offset(1);
            n -= 1;
        } else {
            let fresh184 = used;
            used = used + 1;
            *z.offset(fresh184 as isize) = c as libc::c_char;
        }
    }
    *z.offset(used as isize) = 0 as libc::c_int as libc::c_char;
    return z;
}
#[no_mangle]
pub unsafe extern "C" fn translate_code(
    mut lemp: *mut lemon,
    mut rp: *mut rule,
) -> libc::c_int {
    let mut cp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut xp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut i: libc::c_int = 0;
    let mut rc: libc::c_int = 0 as libc::c_int;
    let mut dontUseRhs0: libc::c_int = 0 as libc::c_int;
    let mut zSkip: *const libc::c_char = 0 as *const libc::c_char;
    let mut lhsused: libc::c_char = 0 as libc::c_int as libc::c_char;
    let mut lhsdirect: libc::c_char = 0;
    let mut used: [libc::c_char; 1000] = [0; 1000];
    let mut zLhs: [libc::c_char; 50] = [0; 50];
    let mut zOvwrt: [libc::c_char; 900] = [0; 900];
    i = 0 as libc::c_int;
    while i < (*rp).nrhs {
        used[i as usize] = 0 as libc::c_int as libc::c_char;
        i += 1;
    }
    lhsused = 0 as libc::c_int as libc::c_char;
    if ((*rp).code).is_null() {
        static mut newlinestr: [libc::c_char; 2] = [
            '\n' as i32 as libc::c_char,
            '\0' as i32 as libc::c_char,
        ];
        let ref mut fresh185 = (*rp).code;
        *fresh185 = newlinestr.as_mut_ptr();
        (*rp).line = (*rp).ruleline;
        (*rp).noCode = LEMON_TRUE;
    } else {
        (*rp).noCode = LEMON_FALSE;
    }
    if (*rp).nrhs == 0 as libc::c_int {
        lhsdirect = 1 as libc::c_int as libc::c_char;
    } else if (*((*rp).rhsalias).offset(0 as libc::c_int as isize)).is_null() {
        lhsdirect = 1 as libc::c_int as libc::c_char;
        if has_destructor(*((*rp).rhs).offset(0 as libc::c_int as isize), lemp) != 0 {
            append_str(
                0 as *const libc::c_char,
                0 as libc::c_int,
                0 as libc::c_int,
                0 as libc::c_int,
            );
            append_str(
                b"  yy_destructor(yypParser,%d,&yymsp[%d].minor);\n\0" as *const u8
                    as *const libc::c_char,
                0 as libc::c_int,
                (**((*rp).rhs).offset(0 as libc::c_int as isize)).index,
                1 as libc::c_int - (*rp).nrhs,
            );
            let ref mut fresh186 = (*rp).codePrefix;
            *fresh186 = Strsafe(
                append_str(
                    0 as *const libc::c_char,
                    0 as libc::c_int,
                    0 as libc::c_int,
                    0 as libc::c_int,
                ),
            );
            (*rp).noCode = LEMON_FALSE;
        }
    } else if ((*rp).lhsalias).is_null() {
        lhsdirect = 1 as libc::c_int as libc::c_char;
    } else if strcmp((*rp).lhsalias, *((*rp).rhsalias).offset(0 as libc::c_int as isize))
        == 0 as libc::c_int
    {
        lhsdirect = 1 as libc::c_int as libc::c_char;
        lhsused = 1 as libc::c_int as libc::c_char;
        used[0 as libc::c_int as usize] = 1 as libc::c_int as libc::c_char;
        if (*(*rp).lhs).dtnum != (**((*rp).rhs).offset(0 as libc::c_int as isize)).dtnum
        {
            ErrorMsg(
                (*lemp).filename,
                (*rp).ruleline,
                b"%s(%s) and %s(%s) share the same label but have different datatypes.\0"
                    as *const u8 as *const libc::c_char,
                (*(*rp).lhs).name,
                (*rp).lhsalias,
                (**((*rp).rhs).offset(0 as libc::c_int as isize)).name,
                *((*rp).rhsalias).offset(0 as libc::c_int as isize),
            );
            let ref mut fresh187 = (*lemp).errorcnt;
            *fresh187 += 1;
        }
    } else {
        lemon_sprintf(
            zOvwrt.as_mut_ptr(),
            b"/*%s-overwrites-%s*/\0" as *const u8 as *const libc::c_char,
            (*rp).lhsalias,
            *((*rp).rhsalias).offset(0 as libc::c_int as isize),
        );
        zSkip = strstr((*rp).code, zOvwrt.as_mut_ptr());
        if !zSkip.is_null() {
            lhsdirect = 1 as libc::c_int as libc::c_char;
        } else {
            lhsdirect = 0 as libc::c_int as libc::c_char;
        }
    }
    if lhsdirect != 0 {
        sprintf(
            zLhs.as_mut_ptr(),
            b"yymsp[%d].minor.yy%d\0" as *const u8 as *const libc::c_char,
            1 as libc::c_int - (*rp).nrhs,
            (*(*rp).lhs).dtnum,
        );
    } else {
        rc = 1 as libc::c_int;
        sprintf(
            zLhs.as_mut_ptr(),
            b"yylhsminor.yy%d\0" as *const u8 as *const libc::c_char,
            (*(*rp).lhs).dtnum,
        );
    }
    append_str(
        0 as *const libc::c_char,
        0 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
    );
    cp = (*rp).code as *mut libc::c_char;
    while *cp != 0 {
        if cp == zSkip as *mut libc::c_char {
            append_str(
                zOvwrt.as_mut_ptr(),
                0 as libc::c_int,
                0 as libc::c_int,
                0 as libc::c_int,
            );
            cp = cp
                .offset(
                    (strlen(zOvwrt.as_mut_ptr()) as libc::c_int - 1 as libc::c_int)
                        as isize,
                );
            dontUseRhs0 = 1 as libc::c_int;
        } else {
            if *(*__ctype_b_loc()).offset(*cp as libc::c_uchar as libc::c_int as isize)
                as libc::c_int & _ISalpha as libc::c_int as libc::c_ushort as libc::c_int
                != 0
                && (cp == (*rp).code as *mut libc::c_char
                    || *(*__ctype_b_loc())
                        .offset(
                            *cp.offset(-(1 as libc::c_int) as isize) as libc::c_uchar
                                as libc::c_int as isize,
                        ) as libc::c_int
                        & _ISalnum as libc::c_int as libc::c_ushort as libc::c_int == 0
                        && *cp.offset(-(1 as libc::c_int) as isize) as libc::c_int
                            != '_' as i32)
            {
                let mut saved: libc::c_char = 0;
                xp = &mut *cp.offset(1 as libc::c_int as isize) as *mut libc::c_char;
                while *(*__ctype_b_loc())
                    .offset(*xp as libc::c_uchar as libc::c_int as isize) as libc::c_int
                    & _ISalnum as libc::c_int as libc::c_ushort as libc::c_int != 0
                    || *xp as libc::c_int == '_' as i32
                {
                    xp = xp.offset(1);
                }
                saved = *xp;
                *xp = 0 as libc::c_int as libc::c_char;
                if !((*rp).lhsalias).is_null()
                    && strcmp(cp, (*rp).lhsalias) == 0 as libc::c_int
                {
                    append_str(
                        zLhs.as_mut_ptr(),
                        0 as libc::c_int,
                        0 as libc::c_int,
                        0 as libc::c_int,
                    );
                    cp = xp;
                    lhsused = 1 as libc::c_int as libc::c_char;
                } else {
                    i = 0 as libc::c_int;
                    while i < (*rp).nrhs {
                        if !(*((*rp).rhsalias).offset(i as isize)).is_null()
                            && strcmp(cp, *((*rp).rhsalias).offset(i as isize))
                                == 0 as libc::c_int
                        {
                            if i == 0 as libc::c_int && dontUseRhs0 != 0 {
                                ErrorMsg(
                                    (*lemp).filename,
                                    (*rp).ruleline,
                                    b"Label %s used after '%s'.\0" as *const u8
                                        as *const libc::c_char,
                                    *((*rp).rhsalias).offset(0 as libc::c_int as isize),
                                    zOvwrt.as_mut_ptr(),
                                );
                                let ref mut fresh188 = (*lemp).errorcnt;
                                *fresh188 += 1;
                            } else if cp != (*rp).code as *mut libc::c_char
                                && *cp.offset(-(1 as libc::c_int) as isize) as libc::c_int
                                    == '@' as i32
                            {
                                append_str(
                                    b"yymsp[%d].major\0" as *const u8 as *const libc::c_char,
                                    -(1 as libc::c_int),
                                    i - (*rp).nrhs + 1 as libc::c_int,
                                    0 as libc::c_int,
                                );
                            } else {
                                let mut sp: *mut symbol = *((*rp).rhs).offset(i as isize);
                                let mut dtnum: libc::c_int = 0;
                                if (*sp).type_0 as libc::c_uint
                                    == MULTITERMINAL as libc::c_int as libc::c_uint
                                {
                                    dtnum = (**((*sp).subsym).offset(0 as libc::c_int as isize))
                                        .dtnum;
                                } else {
                                    dtnum = (*sp).dtnum;
                                }
                                append_str(
                                    b"yymsp[%d].minor.yy%d\0" as *const u8
                                        as *const libc::c_char,
                                    0 as libc::c_int,
                                    i - (*rp).nrhs + 1 as libc::c_int,
                                    dtnum,
                                );
                            }
                            cp = xp;
                            used[i as usize] = 1 as libc::c_int as libc::c_char;
                            break;
                        } else {
                            i += 1;
                        }
                    }
                }
                *xp = saved;
            }
            append_str(cp, 1 as libc::c_int, 0 as libc::c_int, 0 as libc::c_int);
        }
        cp = cp.offset(1);
    }
    cp = append_str(
        0 as *const libc::c_char,
        0 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
    );
    if !cp.is_null() && *cp.offset(0 as libc::c_int as isize) as libc::c_int != 0 {
        let ref mut fresh189 = (*rp).code;
        *fresh189 = Strsafe(cp);
    }
    append_str(
        0 as *const libc::c_char,
        0 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
    );
    if !((*rp).lhsalias).is_null() && lhsused == 0 {
        ErrorMsg(
            (*lemp).filename,
            (*rp).ruleline,
            b"Label \"%s\" for \"%s(%s)\" is never used.\0" as *const u8
                as *const libc::c_char,
            (*rp).lhsalias,
            (*(*rp).lhs).name,
            (*rp).lhsalias,
        );
        let ref mut fresh190 = (*lemp).errorcnt;
        *fresh190 += 1;
    }
    i = 0 as libc::c_int;
    while i < (*rp).nrhs {
        if !(*((*rp).rhsalias).offset(i as isize)).is_null() {
            if i > 0 as libc::c_int {
                let mut j: libc::c_int = 0;
                if !((*rp).lhsalias).is_null()
                    && strcmp((*rp).lhsalias, *((*rp).rhsalias).offset(i as isize))
                        == 0 as libc::c_int
                {
                    ErrorMsg(
                        (*lemp).filename,
                        (*rp).ruleline,
                        b"%s(%s) has the same label as the LHS but is not the left-most symbol on the RHS.\0"
                            as *const u8 as *const libc::c_char,
                        (**((*rp).rhs).offset(i as isize)).name,
                        *((*rp).rhsalias).offset(i as isize),
                    );
                    let ref mut fresh191 = (*lemp).errorcnt;
                    *fresh191 += 1;
                }
                j = 0 as libc::c_int;
                while j < i {
                    if !(*((*rp).rhsalias).offset(j as isize)).is_null()
                        && strcmp(
                            *((*rp).rhsalias).offset(j as isize),
                            *((*rp).rhsalias).offset(i as isize),
                        ) == 0 as libc::c_int
                    {
                        ErrorMsg(
                            (*lemp).filename,
                            (*rp).ruleline,
                            b"Label %s used for multiple symbols on the RHS of a rule.\0"
                                as *const u8 as *const libc::c_char,
                            *((*rp).rhsalias).offset(i as isize),
                        );
                        let ref mut fresh192 = (*lemp).errorcnt;
                        *fresh192 += 1;
                        break;
                    } else {
                        j += 1;
                    }
                }
            }
            if used[i as usize] == 0 {
                ErrorMsg(
                    (*lemp).filename,
                    (*rp).ruleline,
                    b"Label %s for \"%s(%s)\" is never used.\0" as *const u8
                        as *const libc::c_char,
                    *((*rp).rhsalias).offset(i as isize),
                    (**((*rp).rhs).offset(i as isize)).name,
                    *((*rp).rhsalias).offset(i as isize),
                );
                let ref mut fresh193 = (*lemp).errorcnt;
                *fresh193 += 1;
            }
        } else if i > 0 as libc::c_int
            && has_destructor(*((*rp).rhs).offset(i as isize), lemp) != 0
        {
            append_str(
                b"  yy_destructor(yypParser,%d,&yymsp[%d].minor);\n\0" as *const u8
                    as *const libc::c_char,
                0 as libc::c_int,
                (**((*rp).rhs).offset(i as isize)).index,
                i - (*rp).nrhs + 1 as libc::c_int,
            );
        }
        i += 1;
    }
    if lhsdirect as libc::c_int == 0 as libc::c_int {
        append_str(
            b"  yymsp[%d].minor.yy%d = \0" as *const u8 as *const libc::c_char,
            0 as libc::c_int,
            1 as libc::c_int - (*rp).nrhs,
            (*(*rp).lhs).dtnum,
        );
        append_str(
            zLhs.as_mut_ptr(),
            0 as libc::c_int,
            0 as libc::c_int,
            0 as libc::c_int,
        );
        append_str(
            b";\n\0" as *const u8 as *const libc::c_char,
            0 as libc::c_int,
            0 as libc::c_int,
            0 as libc::c_int,
        );
    }
    cp = append_str(
        0 as *const libc::c_char,
        0 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
    );
    if !cp.is_null() && *cp.offset(0 as libc::c_int as isize) as libc::c_int != 0 {
        let ref mut fresh194 = (*rp).codeSuffix;
        *fresh194 = Strsafe(cp);
        (*rp).noCode = LEMON_FALSE;
    }
    return rc;
}
#[no_mangle]
pub unsafe extern "C" fn emit_code(
    mut out: *mut FILE,
    mut rp: *mut rule,
    mut lemp: *mut lemon,
    mut lineno: *mut libc::c_int,
) {
    let mut cp: *const libc::c_char = 0 as *const libc::c_char;
    if !((*rp).codePrefix).is_null()
        && *((*rp).codePrefix).offset(0 as libc::c_int as isize) as libc::c_int != 0
    {
        fprintf(out, b"{%s\0" as *const u8 as *const libc::c_char, (*rp).codePrefix);
        cp = (*rp).codePrefix;
        while *cp != 0 {
            if *cp as libc::c_int == '\n' as i32 {
                *lineno += 1;
            }
            cp = cp.offset(1);
        }
    }
    if !((*rp).code).is_null() {
        if (*lemp).nolinenosflag == 0 {
            *lineno += 1;
            tplt_linedir(out, (*rp).line, (*lemp).filename);
        }
        fprintf(out, b"{%s\0" as *const u8 as *const libc::c_char, (*rp).code);
        cp = (*rp).code;
        while *cp != 0 {
            if *cp as libc::c_int == '\n' as i32 {
                *lineno += 1;
            }
            cp = cp.offset(1);
        }
        fprintf(out, b"}\n\0" as *const u8 as *const libc::c_char);
        *lineno += 1;
        if (*lemp).nolinenosflag == 0 {
            *lineno += 1;
            tplt_linedir(out, *lineno, (*lemp).outname);
        }
    }
    if !((*rp).codeSuffix).is_null()
        && *((*rp).codeSuffix).offset(0 as libc::c_int as isize) as libc::c_int != 0
    {
        fprintf(out, b"%s\0" as *const u8 as *const libc::c_char, (*rp).codeSuffix);
        cp = (*rp).codeSuffix;
        while *cp != 0 {
            if *cp as libc::c_int == '\n' as i32 {
                *lineno += 1;
            }
            cp = cp.offset(1);
        }
    }
    if !((*rp).codePrefix).is_null() {
        fprintf(out, b"}\n\0" as *const u8 as *const libc::c_char);
        *lineno += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn print_stack_union(
    mut out: *mut FILE,
    mut lemp: *mut lemon,
    mut plineno: *mut libc::c_int,
    mut mhflag: libc::c_int,
) {
    let mut lineno: libc::c_int = 0;
    let mut types: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    let mut arraysize: libc::c_int = 0;
    let mut maxdtlength: libc::c_int = 0;
    let mut stddt: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut hash: libc::c_uint = 0;
    let mut name: *const libc::c_char = 0 as *const libc::c_char;
    arraysize = (*lemp).nsymbol * 2 as libc::c_int;
    types = calloc(
        arraysize as libc::c_ulong,
        ::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong,
    ) as *mut *mut libc::c_char;
    if types.is_null() {
        fprintf(stderr, b"Out of memory.\n\0" as *const u8 as *const libc::c_char);
        exit(1 as libc::c_int);
    }
    i = 0 as libc::c_int;
    while i < arraysize {
        let ref mut fresh195 = *types.offset(i as isize);
        *fresh195 = 0 as *mut libc::c_char;
        i += 1;
    }
    maxdtlength = 0 as libc::c_int;
    if !((*lemp).vartype).is_null() {
        maxdtlength = strlen((*lemp).vartype) as libc::c_int;
    }
    i = 0 as libc::c_int;
    while i < (*lemp).nsymbol {
        let mut len: libc::c_int = 0;
        let mut sp: *mut symbol = *((*lemp).symbols).offset(i as isize);
        if !((*sp).datatype).is_null() {
            len = strlen((*sp).datatype) as libc::c_int;
            if len > maxdtlength {
                maxdtlength = len;
            }
        }
        i += 1;
    }
    stddt = malloc((maxdtlength * 2 as libc::c_int + 1 as libc::c_int) as libc::c_ulong)
        as *mut libc::c_char;
    if stddt.is_null() {
        fprintf(stderr, b"Out of memory.\n\0" as *const u8 as *const libc::c_char);
        exit(1 as libc::c_int);
    }
    i = 0 as libc::c_int;
    while i < (*lemp).nsymbol {
        let mut sp_0: *mut symbol = *((*lemp).symbols).offset(i as isize);
        let mut cp: *mut libc::c_char = 0 as *mut libc::c_char;
        if sp_0 == (*lemp).errsym {
            (*sp_0).dtnum = arraysize + 1 as libc::c_int;
        } else if (*sp_0).type_0 as libc::c_uint
            != NONTERMINAL as libc::c_int as libc::c_uint
            || ((*sp_0).datatype).is_null() && ((*lemp).vartype).is_null()
        {
            (*sp_0).dtnum = 0 as libc::c_int;
        } else {
            cp = (*sp_0).datatype;
            if cp.is_null() {
                cp = (*lemp).vartype;
            }
            j = 0 as libc::c_int;
            while *(*__ctype_b_loc())
                .offset(*cp as libc::c_uchar as libc::c_int as isize) as libc::c_int
                & _ISspace as libc::c_int as libc::c_ushort as libc::c_int != 0
            {
                cp = cp.offset(1);
            }
            while *cp != 0 {
                let fresh196 = cp;
                cp = cp.offset(1);
                let fresh197 = j;
                j = j + 1;
                *stddt.offset(fresh197 as isize) = *fresh196;
            }
            while j > 0 as libc::c_int
                && *(*__ctype_b_loc())
                    .offset(
                        *stddt.offset((j - 1 as libc::c_int) as isize) as libc::c_uchar
                            as libc::c_int as isize,
                    ) as libc::c_int
                    & _ISspace as libc::c_int as libc::c_ushort as libc::c_int != 0
            {
                j -= 1;
            }
            *stddt.offset(j as isize) = 0 as libc::c_int as libc::c_char;
            if !((*lemp).tokentype).is_null()
                && strcmp(stddt, (*lemp).tokentype) == 0 as libc::c_int
            {
                (*sp_0).dtnum = 0 as libc::c_int;
            } else {
                hash = 0 as libc::c_int as libc::c_uint;
                j = 0 as libc::c_int;
                while *stddt.offset(j as isize) != 0 {
                    hash = hash
                        .wrapping_mul(53 as libc::c_int as libc::c_uint)
                        .wrapping_add(*stddt.offset(j as isize) as libc::c_uint);
                    j += 1;
                }
                hash = (hash & 0x7fffffff as libc::c_int as libc::c_uint)
                    .wrapping_rem(arraysize as libc::c_uint);
                while !(*types.offset(hash as isize)).is_null() {
                    if strcmp(*types.offset(hash as isize), stddt) == 0 as libc::c_int {
                        (*sp_0)
                            .dtnum = hash.wrapping_add(1 as libc::c_int as libc::c_uint)
                            as libc::c_int;
                        break;
                    } else {
                        hash = hash.wrapping_add(1);
                        if hash >= arraysize as libc::c_uint {
                            hash = 0 as libc::c_int as libc::c_uint;
                        }
                    }
                }
                if (*types.offset(hash as isize)).is_null() {
                    (*sp_0)
                        .dtnum = hash.wrapping_add(1 as libc::c_int as libc::c_uint)
                        as libc::c_int;
                    let ref mut fresh198 = *types.offset(hash as isize);
                    *fresh198 = malloc(
                        (strlen(stddt) as libc::c_int + 1 as libc::c_int)
                            as libc::c_ulong,
                    ) as *mut libc::c_char;
                    if (*types.offset(hash as isize)).is_null() {
                        fprintf(
                            stderr,
                            b"Out of memory.\n\0" as *const u8 as *const libc::c_char,
                        );
                        exit(1 as libc::c_int);
                    }
                    lemon_strcpy(*types.offset(hash as isize), stddt);
                }
            }
        }
        i += 1;
    }
    name = if !((*lemp).name).is_null() {
        (*lemp).name as *const libc::c_char
    } else {
        b"Parse\0" as *const u8 as *const libc::c_char
    };
    lineno = *plineno;
    if mhflag != 0 {
        fprintf(out, b"#if INTERFACE\n\0" as *const u8 as *const libc::c_char);
        lineno += 1;
    }
    fprintf(
        out,
        b"#define %sTOKENTYPE %s\n\0" as *const u8 as *const libc::c_char,
        name,
        if !((*lemp).tokentype).is_null() {
            (*lemp).tokentype as *const libc::c_char
        } else {
            b"void*\0" as *const u8 as *const libc::c_char
        },
    );
    lineno += 1;
    if mhflag != 0 {
        fprintf(out, b"#endif\n\0" as *const u8 as *const libc::c_char);
        lineno += 1;
    }
    fprintf(out, b"typedef union {\n\0" as *const u8 as *const libc::c_char);
    lineno += 1;
    fprintf(out, b"  int yyinit;\n\0" as *const u8 as *const libc::c_char);
    lineno += 1;
    fprintf(out, b"  %sTOKENTYPE yy0;\n\0" as *const u8 as *const libc::c_char, name);
    lineno += 1;
    i = 0 as libc::c_int;
    while i < arraysize {
        if !(*types.offset(i as isize)).is_null() {
            fprintf(
                out,
                b"  %s yy%d;\n\0" as *const u8 as *const libc::c_char,
                *types.offset(i as isize),
                i + 1 as libc::c_int,
            );
            lineno += 1;
            free(*types.offset(i as isize) as *mut libc::c_void);
        }
        i += 1;
    }
    if !((*lemp).errsym).is_null() && (*(*lemp).errsym).useCnt != 0 {
        fprintf(
            out,
            b"  int yy%d;\n\0" as *const u8 as *const libc::c_char,
            (*(*lemp).errsym).dtnum,
        );
        lineno += 1;
    }
    free(stddt as *mut libc::c_void);
    free(types as *mut libc::c_void);
    fprintf(out, b"} YYMINORTYPE;\n\0" as *const u8 as *const libc::c_char);
    lineno += 1;
    *plineno = lineno;
}
unsafe extern "C" fn minimum_size_type(
    mut lwr: libc::c_int,
    mut upr: libc::c_int,
    mut pnByte: *mut libc::c_int,
) -> *const libc::c_char {
    let mut zType: *const libc::c_char = b"int\0" as *const u8 as *const libc::c_char;
    let mut nByte: libc::c_int = 4 as libc::c_int;
    if lwr >= 0 as libc::c_int {
        if upr <= 255 as libc::c_int {
            zType = b"unsigned char\0" as *const u8 as *const libc::c_char;
            nByte = 1 as libc::c_int;
        } else if upr < 65535 as libc::c_int {
            zType = b"unsigned short int\0" as *const u8 as *const libc::c_char;
            nByte = 2 as libc::c_int;
        } else {
            zType = b"unsigned int\0" as *const u8 as *const libc::c_char;
            nByte = 4 as libc::c_int;
        }
    } else if lwr >= -(127 as libc::c_int) && upr <= 127 as libc::c_int {
        zType = b"signed char\0" as *const u8 as *const libc::c_char;
        nByte = 1 as libc::c_int;
    } else if lwr >= -(32767 as libc::c_int) && upr < 32767 as libc::c_int {
        zType = b"short\0" as *const u8 as *const libc::c_char;
        nByte = 2 as libc::c_int;
    }
    if !pnByte.is_null() {
        *pnByte = nByte;
    }
    return zType;
}
unsafe extern "C" fn axset_compare(
    mut a: *const libc::c_void,
    mut b: *const libc::c_void,
) -> libc::c_int {
    let mut p1: *mut axset = a as *mut axset;
    let mut p2: *mut axset = b as *mut axset;
    let mut c: libc::c_int = 0;
    c = (*p2).nAction - (*p1).nAction;
    if c == 0 as libc::c_int {
        c = (*p1).iOrder - (*p2).iOrder;
    }
    if c != 0 as libc::c_int || p1 == p2 {} else {
        __assert_fail(
            b"c!=0 || p1==p2\0" as *const u8 as *const libc::c_char,
            b"/home/me/github/learnrust/libsql/tool/lemon.c\0" as *const u8
                as *const libc::c_char,
            4262 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 46],
                &[libc::c_char; 46],
            >(b"int axset_compare(const void *, const void *)\0"))
                .as_ptr(),
        );
    }
    return c;
}
unsafe extern "C" fn writeRuleText(mut out: *mut FILE, mut rp: *mut rule) {
    let mut j: libc::c_int = 0;
    fprintf(out, b"%s ::=\0" as *const u8 as *const libc::c_char, (*(*rp).lhs).name);
    j = 0 as libc::c_int;
    while j < (*rp).nrhs {
        let mut sp: *mut symbol = *((*rp).rhs).offset(j as isize);
        if (*sp).type_0 as libc::c_uint != MULTITERMINAL as libc::c_int as libc::c_uint {
            fprintf(out, b" %s\0" as *const u8 as *const libc::c_char, (*sp).name);
        } else {
            let mut k: libc::c_int = 0;
            fprintf(
                out,
                b" %s\0" as *const u8 as *const libc::c_char,
                (**((*sp).subsym).offset(0 as libc::c_int as isize)).name,
            );
            k = 1 as libc::c_int;
            while k < (*sp).nsubsym {
                fprintf(
                    out,
                    b"|%s\0" as *const u8 as *const libc::c_char,
                    (**((*sp).subsym).offset(k as isize)).name,
                );
                k += 1;
            }
        }
        j += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn ReportTable(
    mut lemp: *mut lemon,
    mut mhflag: libc::c_int,
    mut sqlFlag: libc::c_int,
) {
    let mut out: *mut FILE = 0 as *mut FILE;
    let mut in_0: *mut FILE = 0 as *mut FILE;
    let mut sql: *mut FILE = 0 as *mut FILE;
    let mut lineno: libc::c_int = 0;
    let mut stp: *mut state = 0 as *mut state;
    let mut ap: *mut action = 0 as *mut action;
    let mut rp: *mut rule = 0 as *mut rule;
    let mut pActtab: *mut acttab = 0 as *mut acttab;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut n: libc::c_int = 0;
    let mut sz: libc::c_int = 0;
    let mut nLookAhead: libc::c_int = 0;
    let mut szActionType: libc::c_int = 0;
    let mut szCodeType: libc::c_int = 0;
    let mut name: *const libc::c_char = 0 as *const libc::c_char;
    let mut mnTknOfst: libc::c_int = 0;
    let mut mxTknOfst: libc::c_int = 0;
    let mut mnNtOfst: libc::c_int = 0;
    let mut mxNtOfst: libc::c_int = 0;
    let mut ax: *mut axset = 0 as *mut axset;
    let mut prefix: *mut libc::c_char = 0 as *mut libc::c_char;
    (*lemp).minShiftReduce = (*lemp).nstate;
    (*lemp).errAction = (*lemp).minShiftReduce + (*lemp).nrule;
    (*lemp).accAction = (*lemp).errAction + 1 as libc::c_int;
    (*lemp).noAction = (*lemp).accAction + 1 as libc::c_int;
    (*lemp).minReduce = (*lemp).noAction + 1 as libc::c_int;
    (*lemp).maxAction = (*lemp).minReduce + (*lemp).nrule;
    in_0 = tplt_open(lemp);
    if in_0.is_null() {
        return;
    }
    out = file_open(
        lemp,
        b".c\0" as *const u8 as *const libc::c_char,
        b"wb\0" as *const u8 as *const libc::c_char,
    );
    if out.is_null() {
        fclose(in_0);
        return;
    }
    if sqlFlag == 0 as libc::c_int {
        sql = 0 as *mut FILE;
    } else {
        sql = file_open(
            lemp,
            b".sql\0" as *const u8 as *const libc::c_char,
            b"wb\0" as *const u8 as *const libc::c_char,
        );
        if sql.is_null() {
            fclose(in_0);
            fclose(out);
            return;
        }
        fprintf(
            sql,
            b"BEGIN;\nCREATE TABLE symbol(\n  id INTEGER PRIMARY KEY,\n  name TEXT NOT NULL,\n  isTerminal BOOLEAN NOT NULL,\n  fallback INTEGER REFERENCES symbol DEFERRABLE INITIALLY DEFERRED\n);\n\0"
                as *const u8 as *const libc::c_char,
        );
        i = 0 as libc::c_int;
        while i < (*lemp).nsymbol {
            fprintf(
                sql,
                b"INSERT INTO symbol(id,name,isTerminal,fallback)VALUES(%d,'%s',%s\0"
                    as *const u8 as *const libc::c_char,
                i,
                (**((*lemp).symbols).offset(i as isize)).name,
                if i < (*lemp).nterminal {
                    b"TRUE\0" as *const u8 as *const libc::c_char
                } else {
                    b"FALSE\0" as *const u8 as *const libc::c_char
                },
            );
            if !((**((*lemp).symbols).offset(i as isize)).fallback).is_null() {
                fprintf(
                    sql,
                    b",%d);\n\0" as *const u8 as *const libc::c_char,
                    (*(**((*lemp).symbols).offset(i as isize)).fallback).index,
                );
            } else {
                fprintf(sql, b",NULL);\n\0" as *const u8 as *const libc::c_char);
            }
            i += 1;
        }
        fprintf(
            sql,
            b"CREATE TABLE rule(\n  ruleid INTEGER PRIMARY KEY,\n  lhs INTEGER REFERENCES symbol(id),\n  txt TEXT\n);\nCREATE TABLE rulerhs(\n  ruleid INTEGER REFERENCES rule(ruleid),\n  pos INTEGER,\n  sym INTEGER REFERENCES symbol(id)\n);\n\0"
                as *const u8 as *const libc::c_char,
        );
        i = 0 as libc::c_int;
        rp = (*lemp).rule;
        while !rp.is_null() {
            if i == (*rp).iRule {} else {
                __assert_fail(
                    b"i==rp->iRule\0" as *const u8 as *const libc::c_char,
                    b"/home/me/github/learnrust/libsql/tool/lemon.c\0" as *const u8
                        as *const libc::c_char,
                    4368 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 43],
                        &[libc::c_char; 43],
                    >(b"void ReportTable(struct lemon *, int, int)\0"))
                        .as_ptr(),
                );
            }
            fprintf(
                sql,
                b"INSERT INTO rule(ruleid,lhs,txt)VALUES(%d,%d,'\0" as *const u8
                    as *const libc::c_char,
                (*rp).iRule,
                (*(*rp).lhs).index,
            );
            writeRuleText(sql, rp);
            fprintf(sql, b"');\n\0" as *const u8 as *const libc::c_char);
            j = 0 as libc::c_int;
            while j < (*rp).nrhs {
                let mut sp: *mut symbol = *((*rp).rhs).offset(j as isize);
                if (*sp).type_0 as libc::c_uint
                    != MULTITERMINAL as libc::c_int as libc::c_uint
                {
                    fprintf(
                        sql,
                        b"INSERT INTO rulerhs(ruleid,pos,sym)VALUES(%d,%d,%d);\n\0"
                            as *const u8 as *const libc::c_char,
                        i,
                        j,
                        (*sp).index,
                    );
                } else {
                    let mut k: libc::c_int = 0;
                    k = 0 as libc::c_int;
                    while k < (*sp).nsubsym {
                        fprintf(
                            sql,
                            b"INSERT INTO rulerhs(ruleid,pos,sym)VALUES(%d,%d,%d);\n\0"
                                as *const u8 as *const libc::c_char,
                            i,
                            j,
                            (**((*sp).subsym).offset(k as isize)).index,
                        );
                        k += 1;
                    }
                }
                j += 1;
            }
            rp = (*rp).next;
            i += 1;
        }
        fprintf(sql, b"COMMIT;\n\0" as *const u8 as *const libc::c_char);
    }
    lineno = 1 as libc::c_int;
    fprintf(
        out,
        b"/* This file is automatically generated by Lemon from input grammar\n** source file \"%s\". */\n\0"
            as *const u8 as *const libc::c_char,
        (*lemp).filename,
    );
    lineno += 2 as libc::c_int;
    if ((*lemp).include).is_null() {
        let ref mut fresh199 = (*lemp).include;
        *fresh199 = b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
    }
    i = 0 as libc::c_int;
    while *(*__ctype_b_loc())
        .offset(
            *((*lemp).include).offset(i as isize) as libc::c_uchar as libc::c_int
                as isize,
        ) as libc::c_int & _ISspace as libc::c_int as libc::c_ushort as libc::c_int != 0
    {
        if *((*lemp).include).offset(i as isize) as libc::c_int == '\n' as i32 {
            let ref mut fresh200 = (*lemp).include;
            *fresh200 = (*fresh200).offset((i + 1 as libc::c_int) as isize);
            i = -(1 as libc::c_int);
        }
        i += 1;
    }
    if *((*lemp).include).offset(0 as libc::c_int as isize) as libc::c_int == '/' as i32
    {
        tplt_skip_header(in_0, &mut lineno);
    } else {
        tplt_xfer((*lemp).name, in_0, out, &mut lineno);
    }
    tplt_print(out, lemp, (*lemp).include, &mut lineno);
    if mhflag != 0 {
        let mut incName: *mut libc::c_char = file_makename(
            lemp,
            b".h\0" as *const u8 as *const libc::c_char,
        );
        fprintf(
            out,
            b"#include \"%s\"\n\0" as *const u8 as *const libc::c_char,
            incName,
        );
        lineno += 1;
        free(incName as *mut libc::c_void);
    }
    tplt_xfer((*lemp).name, in_0, out, &mut lineno);
    if !((*lemp).tokenprefix).is_null() {
        prefix = (*lemp).tokenprefix;
    } else {
        prefix = b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
    }
    if mhflag != 0 {
        fprintf(out, b"#if INTERFACE\n\0" as *const u8 as *const libc::c_char);
        lineno += 1;
    } else {
        fprintf(
            out,
            b"#ifndef %s%s\n\0" as *const u8 as *const libc::c_char,
            prefix,
            (**((*lemp).symbols).offset(1 as libc::c_int as isize)).name,
        );
    }
    i = 1 as libc::c_int;
    while i < (*lemp).nterminal {
        fprintf(
            out,
            b"#define %s%-30s %2d\n\0" as *const u8 as *const libc::c_char,
            prefix,
            (**((*lemp).symbols).offset(i as isize)).name,
            i,
        );
        lineno += 1;
        i += 1;
    }
    fprintf(out, b"#endif\n\0" as *const u8 as *const libc::c_char);
    lineno += 1;
    tplt_xfer((*lemp).name, in_0, out, &mut lineno);
    fprintf(
        out,
        b"#define YYCODETYPE %s\n\0" as *const u8 as *const libc::c_char,
        minimum_size_type(0 as libc::c_int, (*lemp).nsymbol, &mut szCodeType),
    );
    lineno += 1;
    fprintf(
        out,
        b"#define YYNOCODE %d\n\0" as *const u8 as *const libc::c_char,
        (*lemp).nsymbol,
    );
    lineno += 1;
    fprintf(
        out,
        b"#define YYACTIONTYPE %s\n\0" as *const u8 as *const libc::c_char,
        minimum_size_type(0 as libc::c_int, (*lemp).maxAction, &mut szActionType),
    );
    lineno += 1;
    if !((*lemp).wildcard).is_null() {
        fprintf(
            out,
            b"#define YYWILDCARD %d\n\0" as *const u8 as *const libc::c_char,
            (*(*lemp).wildcard).index,
        );
        lineno += 1;
    }
    print_stack_union(out, lemp, &mut lineno, mhflag);
    fprintf(out, b"#ifndef YYSTACKDEPTH\n\0" as *const u8 as *const libc::c_char);
    lineno += 1;
    if !((*lemp).stacksize).is_null() {
        fprintf(
            out,
            b"#define YYSTACKDEPTH %s\n\0" as *const u8 as *const libc::c_char,
            (*lemp).stacksize,
        );
        lineno += 1;
    } else {
        fprintf(
            out,
            b"#define YYSTACKDEPTH 100\n\0" as *const u8 as *const libc::c_char,
        );
        lineno += 1;
    }
    fprintf(out, b"#endif\n\0" as *const u8 as *const libc::c_char);
    lineno += 1;
    if mhflag != 0 {
        fprintf(out, b"#if INTERFACE\n\0" as *const u8 as *const libc::c_char);
        lineno += 1;
    }
    name = if !((*lemp).name).is_null() {
        (*lemp).name as *const libc::c_char
    } else {
        b"Parse\0" as *const u8 as *const libc::c_char
    };
    if !((*lemp).arg).is_null()
        && *((*lemp).arg).offset(0 as libc::c_int as isize) as libc::c_int != 0
    {
        i = strlen((*lemp).arg) as libc::c_int;
        while i >= 1 as libc::c_int
            && *(*__ctype_b_loc())
                .offset(
                    *((*lemp).arg).offset((i - 1 as libc::c_int) as isize)
                        as libc::c_uchar as libc::c_int as isize,
                ) as libc::c_int
                & _ISspace as libc::c_int as libc::c_ushort as libc::c_int != 0
        {
            i -= 1;
        }
        while i >= 1 as libc::c_int
            && (*(*__ctype_b_loc())
                .offset(
                    *((*lemp).arg).offset((i - 1 as libc::c_int) as isize)
                        as libc::c_uchar as libc::c_int as isize,
                ) as libc::c_int
                & _ISalnum as libc::c_int as libc::c_ushort as libc::c_int != 0
                || *((*lemp).arg).offset((i - 1 as libc::c_int) as isize) as libc::c_int
                    == '_' as i32)
        {
            i -= 1;
        }
        fprintf(
            out,
            b"#define %sARG_SDECL %s;\n\0" as *const u8 as *const libc::c_char,
            name,
            (*lemp).arg,
        );
        lineno += 1;
        fprintf(
            out,
            b"#define %sARG_PDECL ,%s\n\0" as *const u8 as *const libc::c_char,
            name,
            (*lemp).arg,
        );
        lineno += 1;
        fprintf(
            out,
            b"#define %sARG_PARAM ,%s\n\0" as *const u8 as *const libc::c_char,
            name,
            &mut *((*lemp).arg).offset(i as isize) as *mut libc::c_char,
        );
        lineno += 1;
        fprintf(
            out,
            b"#define %sARG_FETCH %s=yypParser->%s;\n\0" as *const u8
                as *const libc::c_char,
            name,
            (*lemp).arg,
            &mut *((*lemp).arg).offset(i as isize) as *mut libc::c_char,
        );
        lineno += 1;
        fprintf(
            out,
            b"#define %sARG_STORE yypParser->%s=%s;\n\0" as *const u8
                as *const libc::c_char,
            name,
            &mut *((*lemp).arg).offset(i as isize) as *mut libc::c_char,
            &mut *((*lemp).arg).offset(i as isize) as *mut libc::c_char,
        );
        lineno += 1;
    } else {
        fprintf(
            out,
            b"#define %sARG_SDECL\n\0" as *const u8 as *const libc::c_char,
            name,
        );
        lineno += 1;
        fprintf(
            out,
            b"#define %sARG_PDECL\n\0" as *const u8 as *const libc::c_char,
            name,
        );
        lineno += 1;
        fprintf(
            out,
            b"#define %sARG_PARAM\n\0" as *const u8 as *const libc::c_char,
            name,
        );
        lineno += 1;
        fprintf(
            out,
            b"#define %sARG_FETCH\n\0" as *const u8 as *const libc::c_char,
            name,
        );
        lineno += 1;
        fprintf(
            out,
            b"#define %sARG_STORE\n\0" as *const u8 as *const libc::c_char,
            name,
        );
        lineno += 1;
    }
    if !((*lemp).ctx).is_null()
        && *((*lemp).ctx).offset(0 as libc::c_int as isize) as libc::c_int != 0
    {
        i = strlen((*lemp).ctx) as libc::c_int;
        while i >= 1 as libc::c_int
            && *(*__ctype_b_loc())
                .offset(
                    *((*lemp).ctx).offset((i - 1 as libc::c_int) as isize)
                        as libc::c_uchar as libc::c_int as isize,
                ) as libc::c_int
                & _ISspace as libc::c_int as libc::c_ushort as libc::c_int != 0
        {
            i -= 1;
        }
        while i >= 1 as libc::c_int
            && (*(*__ctype_b_loc())
                .offset(
                    *((*lemp).ctx).offset((i - 1 as libc::c_int) as isize)
                        as libc::c_uchar as libc::c_int as isize,
                ) as libc::c_int
                & _ISalnum as libc::c_int as libc::c_ushort as libc::c_int != 0
                || *((*lemp).ctx).offset((i - 1 as libc::c_int) as isize) as libc::c_int
                    == '_' as i32)
        {
            i -= 1;
        }
        fprintf(
            out,
            b"#define %sCTX_SDECL %s;\n\0" as *const u8 as *const libc::c_char,
            name,
            (*lemp).ctx,
        );
        lineno += 1;
        fprintf(
            out,
            b"#define %sCTX_PDECL ,%s\n\0" as *const u8 as *const libc::c_char,
            name,
            (*lemp).ctx,
        );
        lineno += 1;
        fprintf(
            out,
            b"#define %sCTX_PARAM ,%s\n\0" as *const u8 as *const libc::c_char,
            name,
            &mut *((*lemp).ctx).offset(i as isize) as *mut libc::c_char,
        );
        lineno += 1;
        fprintf(
            out,
            b"#define %sCTX_FETCH %s=yypParser->%s;\n\0" as *const u8
                as *const libc::c_char,
            name,
            (*lemp).ctx,
            &mut *((*lemp).ctx).offset(i as isize) as *mut libc::c_char,
        );
        lineno += 1;
        fprintf(
            out,
            b"#define %sCTX_STORE yypParser->%s=%s;\n\0" as *const u8
                as *const libc::c_char,
            name,
            &mut *((*lemp).ctx).offset(i as isize) as *mut libc::c_char,
            &mut *((*lemp).ctx).offset(i as isize) as *mut libc::c_char,
        );
        lineno += 1;
    } else {
        fprintf(
            out,
            b"#define %sCTX_SDECL\n\0" as *const u8 as *const libc::c_char,
            name,
        );
        lineno += 1;
        fprintf(
            out,
            b"#define %sCTX_PDECL\n\0" as *const u8 as *const libc::c_char,
            name,
        );
        lineno += 1;
        fprintf(
            out,
            b"#define %sCTX_PARAM\n\0" as *const u8 as *const libc::c_char,
            name,
        );
        lineno += 1;
        fprintf(
            out,
            b"#define %sCTX_FETCH\n\0" as *const u8 as *const libc::c_char,
            name,
        );
        lineno += 1;
        fprintf(
            out,
            b"#define %sCTX_STORE\n\0" as *const u8 as *const libc::c_char,
            name,
        );
        lineno += 1;
    }
    if mhflag != 0 {
        fprintf(out, b"#endif\n\0" as *const u8 as *const libc::c_char);
        lineno += 1;
    }
    if !((*lemp).errsym).is_null() && (*(*lemp).errsym).useCnt != 0 {
        fprintf(
            out,
            b"#define YYERRORSYMBOL %d\n\0" as *const u8 as *const libc::c_char,
            (*(*lemp).errsym).index,
        );
        lineno += 1;
        fprintf(
            out,
            b"#define YYERRSYMDT yy%d\n\0" as *const u8 as *const libc::c_char,
            (*(*lemp).errsym).dtnum,
        );
        lineno += 1;
    }
    if (*lemp).has_fallback != 0 {
        fprintf(out, b"#define YYFALLBACK 1\n\0" as *const u8 as *const libc::c_char);
        lineno += 1;
    }
    ax = calloc(
        ((*lemp).nxstate * 2 as libc::c_int) as libc::c_ulong,
        ::std::mem::size_of::<axset>() as libc::c_ulong,
    ) as *mut axset;
    if ax.is_null() {
        fprintf(stderr, b"malloc failed\n\0" as *const u8 as *const libc::c_char);
        exit(1 as libc::c_int);
    }
    i = 0 as libc::c_int;
    while i < (*lemp).nxstate {
        stp = *((*lemp).sorted).offset(i as isize);
        let ref mut fresh201 = (*ax.offset((i * 2 as libc::c_int) as isize)).stp;
        *fresh201 = stp;
        (*ax.offset((i * 2 as libc::c_int) as isize)).isTkn = 1 as libc::c_int;
        (*ax.offset((i * 2 as libc::c_int) as isize)).nAction = (*stp).nTknAct;
        let ref mut fresh202 = (*ax
            .offset((i * 2 as libc::c_int + 1 as libc::c_int) as isize))
            .stp;
        *fresh202 = stp;
        (*ax.offset((i * 2 as libc::c_int + 1 as libc::c_int) as isize))
            .isTkn = 0 as libc::c_int;
        (*ax.offset((i * 2 as libc::c_int + 1 as libc::c_int) as isize))
            .nAction = (*stp).nNtAct;
        i += 1;
    }
    mnTknOfst = 0 as libc::c_int;
    mxTknOfst = mnTknOfst;
    mnNtOfst = 0 as libc::c_int;
    mxNtOfst = mnNtOfst;
    i = 0 as libc::c_int;
    while i < (*lemp).nxstate * 2 as libc::c_int {
        (*ax.offset(i as isize)).iOrder = i;
        i += 1;
    }
    qsort(
        ax as *mut libc::c_void,
        ((*lemp).nxstate * 2 as libc::c_int) as size_t,
        ::std::mem::size_of::<axset>() as libc::c_ulong,
        Some(
            axset_compare
                as unsafe extern "C" fn(
                    *const libc::c_void,
                    *const libc::c_void,
                ) -> libc::c_int,
        ),
    );
    pActtab = acttab_alloc((*lemp).nsymbol, (*lemp).nterminal);
    i = 0 as libc::c_int;
    while i < (*lemp).nxstate * 2 as libc::c_int
        && (*ax.offset(i as isize)).nAction > 0 as libc::c_int
    {
        stp = (*ax.offset(i as isize)).stp;
        if (*ax.offset(i as isize)).isTkn != 0 {
            ap = (*stp).ap;
            while !ap.is_null() {
                let mut action: libc::c_int = 0;
                if !((*(*ap).sp).index >= (*lemp).nterminal) {
                    action = compute_action(lemp, ap);
                    if !(action < 0 as libc::c_int) {
                        acttab_action(pActtab, (*(*ap).sp).index, action);
                    }
                }
                ap = (*ap).next;
            }
            (*stp).iTknOfst = acttab_insert(pActtab, 1 as libc::c_int);
            if (*stp).iTknOfst < mnTknOfst {
                mnTknOfst = (*stp).iTknOfst;
            }
            if (*stp).iTknOfst > mxTknOfst {
                mxTknOfst = (*stp).iTknOfst;
            }
        } else {
            ap = (*stp).ap;
            while !ap.is_null() {
                let mut action_0: libc::c_int = 0;
                if !((*(*ap).sp).index < (*lemp).nterminal) {
                    if !((*(*ap).sp).index == (*lemp).nsymbol) {
                        action_0 = compute_action(lemp, ap);
                        if !(action_0 < 0 as libc::c_int) {
                            acttab_action(pActtab, (*(*ap).sp).index, action_0);
                        }
                    }
                }
                ap = (*ap).next;
            }
            (*stp).iNtOfst = acttab_insert(pActtab, 0 as libc::c_int);
            if (*stp).iNtOfst < mnNtOfst {
                mnNtOfst = (*stp).iNtOfst;
            }
            if (*stp).iNtOfst > mxNtOfst {
                mxNtOfst = (*stp).iNtOfst;
            }
        }
        i += 1;
    }
    free(ax as *mut libc::c_void);
    rp = (*lemp).rule;
    while !rp.is_null() {
        (*rp).doesReduce = LEMON_FALSE;
        rp = (*rp).next;
    }
    i = 0 as libc::c_int;
    while i < (*lemp).nxstate {
        ap = (**((*lemp).sorted).offset(i as isize)).ap;
        while !ap.is_null() {
            if (*ap).type_0 as libc::c_uint == REDUCE as libc::c_int as libc::c_uint
                || (*ap).type_0 as libc::c_uint
                    == SHIFTREDUCE as libc::c_int as libc::c_uint
            {
                (*(*ap).x.rp).doesReduce = LEMON_TRUE;
            }
            ap = (*ap).next;
        }
        i += 1;
    }
    fprintf(
        out,
        b"#define YYNSTATE             %d\n\0" as *const u8 as *const libc::c_char,
        (*lemp).nxstate,
    );
    lineno += 1;
    fprintf(
        out,
        b"#define YYNRULE              %d\n\0" as *const u8 as *const libc::c_char,
        (*lemp).nrule,
    );
    lineno += 1;
    fprintf(
        out,
        b"#define YYNRULE_WITH_ACTION  %d\n\0" as *const u8 as *const libc::c_char,
        (*lemp).nruleWithAction,
    );
    lineno += 1;
    fprintf(
        out,
        b"#define YYNTOKEN             %d\n\0" as *const u8 as *const libc::c_char,
        (*lemp).nterminal,
    );
    lineno += 1;
    fprintf(
        out,
        b"#define YY_MAX_SHIFT         %d\n\0" as *const u8 as *const libc::c_char,
        (*lemp).nxstate - 1 as libc::c_int,
    );
    lineno += 1;
    i = (*lemp).minShiftReduce;
    fprintf(
        out,
        b"#define YY_MIN_SHIFTREDUCE   %d\n\0" as *const u8 as *const libc::c_char,
        i,
    );
    lineno += 1;
    i += (*lemp).nrule;
    fprintf(
        out,
        b"#define YY_MAX_SHIFTREDUCE   %d\n\0" as *const u8 as *const libc::c_char,
        i - 1 as libc::c_int,
    );
    lineno += 1;
    fprintf(
        out,
        b"#define YY_ERROR_ACTION      %d\n\0" as *const u8 as *const libc::c_char,
        (*lemp).errAction,
    );
    lineno += 1;
    fprintf(
        out,
        b"#define YY_ACCEPT_ACTION     %d\n\0" as *const u8 as *const libc::c_char,
        (*lemp).accAction,
    );
    lineno += 1;
    fprintf(
        out,
        b"#define YY_NO_ACTION         %d\n\0" as *const u8 as *const libc::c_char,
        (*lemp).noAction,
    );
    lineno += 1;
    fprintf(
        out,
        b"#define YY_MIN_REDUCE        %d\n\0" as *const u8 as *const libc::c_char,
        (*lemp).minReduce,
    );
    lineno += 1;
    i = (*lemp).minReduce + (*lemp).nrule;
    fprintf(
        out,
        b"#define YY_MAX_REDUCE        %d\n\0" as *const u8 as *const libc::c_char,
        i - 1 as libc::c_int,
    );
    lineno += 1;
    tplt_xfer((*lemp).name, in_0, out, &mut lineno);
    n = acttab_action_size(pActtab);
    (*lemp).nactiontab = n;
    (*lemp).tablesize += n * szActionType;
    fprintf(
        out,
        b"#define YY_ACTTAB_COUNT (%d)\n\0" as *const u8 as *const libc::c_char,
        n,
    );
    lineno += 1;
    fprintf(
        out,
        b"static const YYACTIONTYPE yy_action[] = {\n\0" as *const u8
            as *const libc::c_char,
    );
    lineno += 1;
    j = 0 as libc::c_int;
    i = j;
    while i < n {
        let mut action_1: libc::c_int = (*((*pActtab).aAction).offset(i as isize))
            .action;
        if action_1 < 0 as libc::c_int {
            action_1 = (*lemp).noAction;
        }
        if j == 0 as libc::c_int {
            fprintf(out, b" /* %5d */ \0" as *const u8 as *const libc::c_char, i);
        }
        fprintf(out, b" %4d,\0" as *const u8 as *const libc::c_char, action_1);
        if j == 9 as libc::c_int || i == n - 1 as libc::c_int {
            fprintf(out, b"\n\0" as *const u8 as *const libc::c_char);
            lineno += 1;
            j = 0 as libc::c_int;
        } else {
            j += 1;
        }
        i += 1;
    }
    fprintf(out, b"};\n\0" as *const u8 as *const libc::c_char);
    lineno += 1;
    n = (*pActtab).nAction;
    (*lemp).nlookaheadtab = n;
    (*lemp).tablesize += n * szCodeType;
    fprintf(
        out,
        b"static const YYCODETYPE yy_lookahead[] = {\n\0" as *const u8
            as *const libc::c_char,
    );
    lineno += 1;
    j = 0 as libc::c_int;
    i = j;
    while i < n {
        let mut la: libc::c_int = (*((*pActtab).aAction).offset(i as isize)).lookahead;
        if la < 0 as libc::c_int {
            la = (*lemp).nsymbol;
        }
        if j == 0 as libc::c_int {
            fprintf(out, b" /* %5d */ \0" as *const u8 as *const libc::c_char, i);
        }
        fprintf(out, b" %4d,\0" as *const u8 as *const libc::c_char, la);
        if j == 9 as libc::c_int {
            fprintf(out, b"\n\0" as *const u8 as *const libc::c_char);
            lineno += 1;
            j = 0 as libc::c_int;
        } else {
            j += 1;
        }
        i += 1;
    }
    nLookAhead = (*lemp).nterminal + (*lemp).nactiontab;
    while i < nLookAhead {
        if j == 0 as libc::c_int {
            fprintf(out, b" /* %5d */ \0" as *const u8 as *const libc::c_char, i);
        }
        fprintf(out, b" %4d,\0" as *const u8 as *const libc::c_char, (*lemp).nterminal);
        if j == 9 as libc::c_int {
            fprintf(out, b"\n\0" as *const u8 as *const libc::c_char);
            lineno += 1;
            j = 0 as libc::c_int;
        } else {
            j += 1;
        }
        i += 1;
    }
    if j > 0 as libc::c_int {
        fprintf(out, b"\n\0" as *const u8 as *const libc::c_char);
        lineno += 1;
    }
    fprintf(out, b"};\n\0" as *const u8 as *const libc::c_char);
    lineno += 1;
    n = (*lemp).nxstate;
    while n > 0 as libc::c_int
        && (**((*lemp).sorted).offset((n - 1 as libc::c_int) as isize)).iTknOfst
            == -(2147483647 as libc::c_int)
    {
        n -= 1;
    }
    fprintf(
        out,
        b"#define YY_SHIFT_COUNT    (%d)\n\0" as *const u8 as *const libc::c_char,
        n - 1 as libc::c_int,
    );
    lineno += 1;
    fprintf(
        out,
        b"#define YY_SHIFT_MIN      (%d)\n\0" as *const u8 as *const libc::c_char,
        mnTknOfst,
    );
    lineno += 1;
    fprintf(
        out,
        b"#define YY_SHIFT_MAX      (%d)\n\0" as *const u8 as *const libc::c_char,
        mxTknOfst,
    );
    lineno += 1;
    fprintf(
        out,
        b"static const %s yy_shift_ofst[] = {\n\0" as *const u8 as *const libc::c_char,
        minimum_size_type(mnTknOfst, (*lemp).nterminal + (*lemp).nactiontab, &mut sz),
    );
    lineno += 1;
    (*lemp).tablesize += n * sz;
    j = 0 as libc::c_int;
    i = j;
    while i < n {
        let mut ofst: libc::c_int = 0;
        stp = *((*lemp).sorted).offset(i as isize);
        ofst = (*stp).iTknOfst;
        if ofst == -(2147483647 as libc::c_int) {
            ofst = (*lemp).nactiontab;
        }
        if j == 0 as libc::c_int {
            fprintf(out, b" /* %5d */ \0" as *const u8 as *const libc::c_char, i);
        }
        fprintf(out, b" %4d,\0" as *const u8 as *const libc::c_char, ofst);
        if j == 9 as libc::c_int || i == n - 1 as libc::c_int {
            fprintf(out, b"\n\0" as *const u8 as *const libc::c_char);
            lineno += 1;
            j = 0 as libc::c_int;
        } else {
            j += 1;
        }
        i += 1;
    }
    fprintf(out, b"};\n\0" as *const u8 as *const libc::c_char);
    lineno += 1;
    n = (*lemp).nxstate;
    while n > 0 as libc::c_int
        && (**((*lemp).sorted).offset((n - 1 as libc::c_int) as isize)).iNtOfst
            == -(2147483647 as libc::c_int)
    {
        n -= 1;
    }
    fprintf(
        out,
        b"#define YY_REDUCE_COUNT (%d)\n\0" as *const u8 as *const libc::c_char,
        n - 1 as libc::c_int,
    );
    lineno += 1;
    fprintf(
        out,
        b"#define YY_REDUCE_MIN   (%d)\n\0" as *const u8 as *const libc::c_char,
        mnNtOfst,
    );
    lineno += 1;
    fprintf(
        out,
        b"#define YY_REDUCE_MAX   (%d)\n\0" as *const u8 as *const libc::c_char,
        mxNtOfst,
    );
    lineno += 1;
    fprintf(
        out,
        b"static const %s yy_reduce_ofst[] = {\n\0" as *const u8 as *const libc::c_char,
        minimum_size_type(mnNtOfst - 1 as libc::c_int, mxNtOfst, &mut sz),
    );
    lineno += 1;
    (*lemp).tablesize += n * sz;
    j = 0 as libc::c_int;
    i = j;
    while i < n {
        let mut ofst_0: libc::c_int = 0;
        stp = *((*lemp).sorted).offset(i as isize);
        ofst_0 = (*stp).iNtOfst;
        if ofst_0 == -(2147483647 as libc::c_int) {
            ofst_0 = mnNtOfst - 1 as libc::c_int;
        }
        if j == 0 as libc::c_int {
            fprintf(out, b" /* %5d */ \0" as *const u8 as *const libc::c_char, i);
        }
        fprintf(out, b" %4d,\0" as *const u8 as *const libc::c_char, ofst_0);
        if j == 9 as libc::c_int || i == n - 1 as libc::c_int {
            fprintf(out, b"\n\0" as *const u8 as *const libc::c_char);
            lineno += 1;
            j = 0 as libc::c_int;
        } else {
            j += 1;
        }
        i += 1;
    }
    fprintf(out, b"};\n\0" as *const u8 as *const libc::c_char);
    lineno += 1;
    fprintf(
        out,
        b"static const YYACTIONTYPE yy_default[] = {\n\0" as *const u8
            as *const libc::c_char,
    );
    lineno += 1;
    n = (*lemp).nxstate;
    (*lemp).tablesize += n * szActionType;
    j = 0 as libc::c_int;
    i = j;
    while i < n {
        stp = *((*lemp).sorted).offset(i as isize);
        if j == 0 as libc::c_int {
            fprintf(out, b" /* %5d */ \0" as *const u8 as *const libc::c_char, i);
        }
        if (*stp).iDfltReduce < 0 as libc::c_int {
            fprintf(
                out,
                b" %4d,\0" as *const u8 as *const libc::c_char,
                (*lemp).errAction,
            );
        } else {
            fprintf(
                out,
                b" %4d,\0" as *const u8 as *const libc::c_char,
                (*stp).iDfltReduce + (*lemp).minReduce,
            );
        }
        if j == 9 as libc::c_int || i == n - 1 as libc::c_int {
            fprintf(out, b"\n\0" as *const u8 as *const libc::c_char);
            lineno += 1;
            j = 0 as libc::c_int;
        } else {
            j += 1;
        }
        i += 1;
    }
    fprintf(out, b"};\n\0" as *const u8 as *const libc::c_char);
    lineno += 1;
    tplt_xfer((*lemp).name, in_0, out, &mut lineno);
    if (*lemp).has_fallback != 0 {
        let mut mx: libc::c_int = (*lemp).nterminal - 1 as libc::c_int;
        (*lemp).tablesize += (mx + 1 as libc::c_int) * szCodeType;
        i = 0 as libc::c_int;
        while i <= mx {
            let mut p: *mut symbol = *((*lemp).symbols).offset(i as isize);
            if ((*p).fallback).is_null() {
                fprintf(
                    out,
                    b"    0,  /* %10s => nothing */\n\0" as *const u8
                        as *const libc::c_char,
                    (*p).name,
                );
            } else {
                fprintf(
                    out,
                    b"  %3d,  /* %10s => %s */\n\0" as *const u8 as *const libc::c_char,
                    (*(*p).fallback).index,
                    (*p).name,
                    (*(*p).fallback).name,
                );
            }
            lineno += 1;
            i += 1;
        }
    }
    tplt_xfer((*lemp).name, in_0, out, &mut lineno);
    i = 0 as libc::c_int;
    while i < (*lemp).nsymbol {
        fprintf(
            out,
            b"  /* %4d */ \"%s\",\n\0" as *const u8 as *const libc::c_char,
            i,
            (**((*lemp).symbols).offset(i as isize)).name,
        );
        lineno += 1;
        i += 1;
    }
    tplt_xfer((*lemp).name, in_0, out, &mut lineno);
    i = 0 as libc::c_int;
    rp = (*lemp).rule;
    while !rp.is_null() {
        if (*rp).iRule == i {} else {
            __assert_fail(
                b"rp->iRule==i\0" as *const u8 as *const libc::c_char,
                b"/home/me/github/learnrust/libsql/tool/lemon.c\0" as *const u8
                    as *const libc::c_char,
                4777 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 43],
                    &[libc::c_char; 43],
                >(b"void ReportTable(struct lemon *, int, int)\0"))
                    .as_ptr(),
            );
        }
        fprintf(out, b" /* %3d */ \"\0" as *const u8 as *const libc::c_char, i);
        writeRuleText(out, rp);
        fprintf(out, b"\",\n\0" as *const u8 as *const libc::c_char);
        lineno += 1;
        rp = (*rp).next;
        i += 1;
    }
    tplt_xfer((*lemp).name, in_0, out, &mut lineno);
    if !((*lemp).tokendest).is_null() {
        let mut once: libc::c_int = 1 as libc::c_int;
        i = 0 as libc::c_int;
        while i < (*lemp).nsymbol {
            let mut sp_0: *mut symbol = *((*lemp).symbols).offset(i as isize);
            if !(sp_0.is_null()
                || (*sp_0).type_0 as libc::c_uint
                    != TERMINAL as libc::c_int as libc::c_uint)
            {
                if once != 0 {
                    fprintf(
                        out,
                        b"      /* TERMINAL Destructor */\n\0" as *const u8
                            as *const libc::c_char,
                    );
                    lineno += 1;
                    once = 0 as libc::c_int;
                }
                fprintf(
                    out,
                    b"    case %d: /* %s */\n\0" as *const u8 as *const libc::c_char,
                    (*sp_0).index,
                    (*sp_0).name,
                );
                lineno += 1;
            }
            i += 1;
        }
        i = 0 as libc::c_int;
        while i < (*lemp).nsymbol
            && (**((*lemp).symbols).offset(i as isize)).type_0 as libc::c_uint
                != TERMINAL as libc::c_int as libc::c_uint
        {
            i += 1;
        }
        if i < (*lemp).nsymbol {
            emit_destructor_code(
                out,
                *((*lemp).symbols).offset(i as isize),
                lemp,
                &mut lineno,
            );
            fprintf(out, b"      break;\n\0" as *const u8 as *const libc::c_char);
            lineno += 1;
        }
    }
    if !((*lemp).vardest).is_null() {
        let mut dflt_sp: *mut symbol = 0 as *mut symbol;
        let mut once_0: libc::c_int = 1 as libc::c_int;
        i = 0 as libc::c_int;
        while i < (*lemp).nsymbol {
            let mut sp_1: *mut symbol = *((*lemp).symbols).offset(i as isize);
            if !(sp_1.is_null()
                || (*sp_1).type_0 as libc::c_uint
                    == TERMINAL as libc::c_int as libc::c_uint
                || (*sp_1).index <= 0 as libc::c_int || !((*sp_1).destructor).is_null())
            {
                if once_0 != 0 {
                    fprintf(
                        out,
                        b"      /* Default NON-TERMINAL Destructor */\n\0" as *const u8
                            as *const libc::c_char,
                    );
                    lineno += 1;
                    once_0 = 0 as libc::c_int;
                }
                fprintf(
                    out,
                    b"    case %d: /* %s */\n\0" as *const u8 as *const libc::c_char,
                    (*sp_1).index,
                    (*sp_1).name,
                );
                lineno += 1;
                dflt_sp = sp_1;
            }
            i += 1;
        }
        if !dflt_sp.is_null() {
            emit_destructor_code(out, dflt_sp, lemp, &mut lineno);
        }
        fprintf(out, b"      break;\n\0" as *const u8 as *const libc::c_char);
        lineno += 1;
    }
    i = 0 as libc::c_int;
    while i < (*lemp).nsymbol {
        let mut sp_2: *mut symbol = *((*lemp).symbols).offset(i as isize);
        if !(sp_2.is_null()
            || (*sp_2).type_0 as libc::c_uint == TERMINAL as libc::c_int as libc::c_uint
            || ((*sp_2).destructor).is_null())
        {
            if !((*sp_2).destLineno < 0 as libc::c_int) {
                fprintf(
                    out,
                    b"    case %d: /* %s */\n\0" as *const u8 as *const libc::c_char,
                    (*sp_2).index,
                    (*sp_2).name,
                );
                lineno += 1;
                j = i + 1 as libc::c_int;
                while j < (*lemp).nsymbol {
                    let mut sp2: *mut symbol = *((*lemp).symbols).offset(j as isize);
                    if !sp2.is_null()
                        && (*sp2).type_0 as libc::c_uint
                            != TERMINAL as libc::c_int as libc::c_uint
                        && !((*sp2).destructor).is_null()
                        && (*sp2).dtnum == (*sp_2).dtnum
                        && strcmp((*sp_2).destructor, (*sp2).destructor)
                            == 0 as libc::c_int
                    {
                        fprintf(
                            out,
                            b"    case %d: /* %s */\n\0" as *const u8
                                as *const libc::c_char,
                            (*sp2).index,
                            (*sp2).name,
                        );
                        lineno += 1;
                        (*sp2).destLineno = -(1 as libc::c_int);
                    }
                    j += 1;
                }
                emit_destructor_code(
                    out,
                    *((*lemp).symbols).offset(i as isize),
                    lemp,
                    &mut lineno,
                );
                fprintf(out, b"      break;\n\0" as *const u8 as *const libc::c_char);
                lineno += 1;
            }
        }
        i += 1;
    }
    tplt_xfer((*lemp).name, in_0, out, &mut lineno);
    tplt_print(out, lemp, (*lemp).overflow, &mut lineno);
    tplt_xfer((*lemp).name, in_0, out, &mut lineno);
    i = 0 as libc::c_int;
    rp = (*lemp).rule;
    while !rp.is_null() {
        fprintf(
            out,
            b"  %4d,  /* (%d) \0" as *const u8 as *const libc::c_char,
            (*(*rp).lhs).index,
            i,
        );
        rule_print(out, rp);
        fprintf(out, b" */\n\0" as *const u8 as *const libc::c_char);
        lineno += 1;
        rp = (*rp).next;
        i += 1;
    }
    tplt_xfer((*lemp).name, in_0, out, &mut lineno);
    i = 0 as libc::c_int;
    rp = (*lemp).rule;
    while !rp.is_null() {
        fprintf(
            out,
            b"  %3d,  /* (%d) \0" as *const u8 as *const libc::c_char,
            -(*rp).nrhs,
            i,
        );
        rule_print(out, rp);
        fprintf(out, b" */\n\0" as *const u8 as *const libc::c_char);
        lineno += 1;
        rp = (*rp).next;
        i += 1;
    }
    tplt_xfer((*lemp).name, in_0, out, &mut lineno);
    i = 0 as libc::c_int;
    rp = (*lemp).rule;
    while !rp.is_null() {
        i += translate_code(lemp, rp);
        rp = (*rp).next;
    }
    if i != 0 {
        fprintf(
            out,
            b"        YYMINORTYPE yylhsminor;\n\0" as *const u8 as *const libc::c_char,
        );
        lineno += 1;
    }
    rp = (*lemp).rule;
    while !rp.is_null() {
        let mut rp2: *mut rule = 0 as *mut rule;
        if !((*rp).codeEmitted as u64 != 0) {
            if !((*rp).noCode as u64 != 0) {
                fprintf(
                    out,
                    b"      case %d: /* \0" as *const u8 as *const libc::c_char,
                    (*rp).iRule,
                );
                writeRuleText(out, rp);
                fprintf(out, b" */\n\0" as *const u8 as *const libc::c_char);
                lineno += 1;
                rp2 = (*rp).next;
                while !rp2.is_null() {
                    if (*rp2).code == (*rp).code && (*rp2).codePrefix == (*rp).codePrefix
                        && (*rp2).codeSuffix == (*rp).codeSuffix
                    {
                        fprintf(
                            out,
                            b"      case %d: /* \0" as *const u8 as *const libc::c_char,
                            (*rp2).iRule,
                        );
                        writeRuleText(out, rp2);
                        fprintf(
                            out,
                            b" */ yytestcase(yyruleno==%d);\n\0" as *const u8
                                as *const libc::c_char,
                            (*rp2).iRule,
                        );
                        lineno += 1;
                        (*rp2).codeEmitted = LEMON_TRUE;
                    }
                    rp2 = (*rp2).next;
                }
                emit_code(out, rp, lemp, &mut lineno);
                fprintf(out, b"        break;\n\0" as *const u8 as *const libc::c_char);
                lineno += 1;
                (*rp).codeEmitted = LEMON_TRUE;
            }
        }
        rp = (*rp).next;
    }
    fprintf(out, b"      default:\n\0" as *const u8 as *const libc::c_char);
    lineno += 1;
    rp = (*lemp).rule;
    while !rp.is_null() {
        if !((*rp).codeEmitted as u64 != 0) {
            if (*rp).noCode as u64 != 0 {} else {
                __assert_fail(
                    b"rp->noCode\0" as *const u8 as *const libc::c_char,
                    b"/home/me/github/learnrust/libsql/tool/lemon.c\0" as *const u8
                        as *const libc::c_char,
                    4907 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 43],
                        &[libc::c_char; 43],
                    >(b"void ReportTable(struct lemon *, int, int)\0"))
                        .as_ptr(),
                );
            }
            fprintf(
                out,
                b"      /* (%d) \0" as *const u8 as *const libc::c_char,
                (*rp).iRule,
            );
            writeRuleText(out, rp);
            if (*rp).neverReduce as u64 != 0 {
                fprintf(
                    out,
                    b" (NEVER REDUCES) */ assert(yyruleno!=%d);\n\0" as *const u8
                        as *const libc::c_char,
                    (*rp).iRule,
                );
                lineno += 1;
            } else if (*rp).doesReduce as u64 != 0 {
                fprintf(
                    out,
                    b" */ yytestcase(yyruleno==%d);\n\0" as *const u8
                        as *const libc::c_char,
                    (*rp).iRule,
                );
                lineno += 1;
            } else {
                fprintf(
                    out,
                    b" (OPTIMIZED OUT) */ assert(yyruleno!=%d);\n\0" as *const u8
                        as *const libc::c_char,
                    (*rp).iRule,
                );
                lineno += 1;
            }
        }
        rp = (*rp).next;
    }
    fprintf(out, b"        break;\n\0" as *const u8 as *const libc::c_char);
    lineno += 1;
    tplt_xfer((*lemp).name, in_0, out, &mut lineno);
    tplt_print(out, lemp, (*lemp).failure, &mut lineno);
    tplt_xfer((*lemp).name, in_0, out, &mut lineno);
    tplt_print(out, lemp, (*lemp).error, &mut lineno);
    tplt_xfer((*lemp).name, in_0, out, &mut lineno);
    tplt_print(out, lemp, (*lemp).accept, &mut lineno);
    tplt_xfer((*lemp).name, in_0, out, &mut lineno);
    tplt_print(out, lemp, (*lemp).extracode, &mut lineno);
    acttab_free(pActtab);
    fclose(in_0);
    fclose(out);
    if !sql.is_null() {
        fclose(sql);
    }
}
#[no_mangle]
pub unsafe extern "C" fn ReportHeader(mut lemp: *mut lemon) {
    let mut out: *mut FILE = 0 as *mut FILE;
    let mut in_0: *mut FILE = 0 as *mut FILE;
    let mut prefix: *const libc::c_char = 0 as *const libc::c_char;
    let mut line: [libc::c_char; 1000] = [0; 1000];
    let mut pattern: [libc::c_char; 1000] = [0; 1000];
    let mut i: libc::c_int = 0;
    if !((*lemp).tokenprefix).is_null() {
        prefix = (*lemp).tokenprefix;
    } else {
        prefix = b"\0" as *const u8 as *const libc::c_char;
    }
    in_0 = file_open(
        lemp,
        b".h\0" as *const u8 as *const libc::c_char,
        b"rb\0" as *const u8 as *const libc::c_char,
    );
    if !in_0.is_null() {
        let mut nextChar: libc::c_int = 0;
        i = 1 as libc::c_int;
        while i < (*lemp).nterminal
            && !(fgets(line.as_mut_ptr(), 1000 as libc::c_int, in_0)).is_null()
        {
            lemon_sprintf(
                pattern.as_mut_ptr(),
                b"#define %s%-30s %3d\n\0" as *const u8 as *const libc::c_char,
                prefix,
                (**((*lemp).symbols).offset(i as isize)).name,
                i,
            );
            if strcmp(line.as_mut_ptr(), pattern.as_mut_ptr()) != 0 {
                break;
            }
            i += 1;
        }
        nextChar = fgetc(in_0);
        fclose(in_0);
        if i == (*lemp).nterminal && nextChar == -(1 as libc::c_int) {
            return;
        }
    }
    out = file_open(
        lemp,
        b".h\0" as *const u8 as *const libc::c_char,
        b"wb\0" as *const u8 as *const libc::c_char,
    );
    if !out.is_null() {
        i = 1 as libc::c_int;
        while i < (*lemp).nterminal {
            fprintf(
                out,
                b"#define %s%-30s %3d\n\0" as *const u8 as *const libc::c_char,
                prefix,
                (**((*lemp).symbols).offset(i as isize)).name,
                i,
            );
            i += 1;
        }
        fclose(out);
    }
}
#[no_mangle]
pub unsafe extern "C" fn CompressTables(mut lemp: *mut lemon) {
    let mut stp: *mut state = 0 as *mut state;
    let mut ap: *mut action = 0 as *mut action;
    let mut ap2: *mut action = 0 as *mut action;
    let mut nextap: *mut action = 0 as *mut action;
    let mut rp: *mut rule = 0 as *mut rule;
    let mut rp2: *mut rule = 0 as *mut rule;
    let mut rbest: *mut rule = 0 as *mut rule;
    let mut nbest: libc::c_int = 0;
    let mut n: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut usesWildcard: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < (*lemp).nstate {
        stp = *((*lemp).sorted).offset(i as isize);
        nbest = 0 as libc::c_int;
        rbest = 0 as *mut rule;
        usesWildcard = 0 as libc::c_int;
        ap = (*stp).ap;
        while !ap.is_null() {
            if (*ap).type_0 as libc::c_uint == SHIFT as libc::c_int as libc::c_uint
                && (*ap).sp == (*lemp).wildcard
            {
                usesWildcard = 1 as libc::c_int;
            }
            if !((*ap).type_0 as libc::c_uint != REDUCE as libc::c_int as libc::c_uint) {
                rp = (*ap).x.rp;
                if !((*rp).lhsStart != 0) {
                    if !(rp == rbest) {
                        n = 1 as libc::c_int;
                        ap2 = (*ap).next;
                        while !ap2.is_null() {
                            if !((*ap2).type_0 as libc::c_uint
                                != REDUCE as libc::c_int as libc::c_uint)
                            {
                                rp2 = (*ap2).x.rp;
                                if !(rp2 == rbest) {
                                    if rp2 == rp {
                                        n += 1;
                                    }
                                }
                            }
                            ap2 = (*ap2).next;
                        }
                        if n > nbest {
                            nbest = n;
                            rbest = rp;
                        }
                    }
                }
            }
            ap = (*ap).next;
        }
        if !(nbest < 1 as libc::c_int || usesWildcard != 0) {
            ap = (*stp).ap;
            while !ap.is_null() {
                if (*ap).type_0 as libc::c_uint == REDUCE as libc::c_int as libc::c_uint
                    && (*ap).x.rp == rbest
                {
                    break;
                }
                ap = (*ap).next;
            }
            if !ap.is_null() {} else {
                __assert_fail(
                    b"ap\0" as *const u8 as *const libc::c_char,
                    b"/home/me/github/learnrust/libsql/tool/lemon.c\0" as *const u8
                        as *const libc::c_char,
                    5035 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 36],
                        &[libc::c_char; 36],
                    >(b"void CompressTables(struct lemon *)\0"))
                        .as_ptr(),
                );
            }
            let ref mut fresh203 = (*ap).sp;
            *fresh203 = Symbol_new(b"{default}\0" as *const u8 as *const libc::c_char);
            ap = (*ap).next;
            while !ap.is_null() {
                if (*ap).type_0 as libc::c_uint == REDUCE as libc::c_int as libc::c_uint
                    && (*ap).x.rp == rbest
                {
                    (*ap).type_0 = NOT_USED;
                }
                ap = (*ap).next;
            }
            let ref mut fresh204 = (*stp).ap;
            *fresh204 = Action_sort((*stp).ap);
            ap = (*stp).ap;
            while !ap.is_null() {
                if (*ap).type_0 as libc::c_uint == SHIFT as libc::c_int as libc::c_uint {
                    break;
                }
                if (*ap).type_0 as libc::c_uint == REDUCE as libc::c_int as libc::c_uint
                    && (*ap).x.rp != rbest
                {
                    break;
                }
                ap = (*ap).next;
            }
            if ap.is_null() {
                (*stp).autoReduce = 1 as libc::c_int;
                let ref mut fresh205 = (*stp).pDfltReduce;
                *fresh205 = rbest;
            }
        }
        i += 1;
    }
    i = 0 as libc::c_int;
    while i < (*lemp).nstate {
        stp = *((*lemp).sorted).offset(i as isize);
        ap = (*stp).ap;
        while !ap.is_null() {
            let mut pNextState: *mut state = 0 as *mut state;
            if !((*ap).type_0 as libc::c_uint != SHIFT as libc::c_int as libc::c_uint) {
                pNextState = (*ap).x.stp;
                if (*pNextState).autoReduce != 0
                    && !((*pNextState).pDfltReduce).is_null()
                {
                    (*ap).type_0 = SHIFTREDUCE;
                    let ref mut fresh206 = (*ap).x.rp;
                    *fresh206 = (*pNextState).pDfltReduce;
                }
            }
            ap = (*ap).next;
        }
        i += 1;
    }
    i = 0 as libc::c_int;
    while i < (*lemp).nstate {
        stp = *((*lemp).sorted).offset(i as isize);
        ap = (*stp).ap;
        while !ap.is_null() {
            nextap = (*ap).next;
            if !((*ap).type_0 as libc::c_uint
                != SHIFTREDUCE as libc::c_int as libc::c_uint)
            {
                rp = (*ap).x.rp;
                if !((*rp).noCode as libc::c_uint == 0 as libc::c_int as libc::c_uint) {
                    if !((*rp).nrhs != 1 as libc::c_int) {
                        if !((*(*ap).sp).index < (*lemp).nterminal) {
                            nextap = ap;
                            ap2 = (*stp).ap;
                            while !ap2.is_null() && (ap2 == ap || (*ap2).sp != (*rp).lhs)
                            {
                                ap2 = (*ap2).next;
                            }
                            if !ap2.is_null() {} else {
                                __assert_fail(
                                    b"ap2!=0\0" as *const u8 as *const libc::c_char,
                                    b"/home/me/github/learnrust/libsql/tool/lemon.c\0"
                                        as *const u8 as *const libc::c_char,
                                    5092 as libc::c_int as libc::c_uint,
                                    (*::std::mem::transmute::<
                                        &[u8; 36],
                                        &[libc::c_char; 36],
                                    >(b"void CompressTables(struct lemon *)\0"))
                                        .as_ptr(),
                                );
                            }
                            let ref mut fresh207 = (*ap).spOpt;
                            *fresh207 = (*ap2).sp;
                            (*ap).type_0 = (*ap2).type_0;
                            (*ap).x = (*ap2).x;
                        }
                    }
                }
            }
            ap = nextap;
        }
        i += 1;
    }
}
unsafe extern "C" fn stateResortCompare(
    mut a: *const libc::c_void,
    mut b: *const libc::c_void,
) -> libc::c_int {
    let mut pA: *const state = *(a as *mut *const state);
    let mut pB: *const state = *(b as *mut *const state);
    let mut n: libc::c_int = 0;
    n = (*pB).nNtAct - (*pA).nNtAct;
    if n == 0 as libc::c_int {
        n = (*pB).nTknAct - (*pA).nTknAct;
        if n == 0 as libc::c_int {
            n = (*pB).statenum - (*pA).statenum;
        }
    }
    if n != 0 as libc::c_int {} else {
        __assert_fail(
            b"n!=0\0" as *const u8 as *const libc::c_char,
            b"/home/me/github/learnrust/libsql/tool/lemon.c\0" as *const u8
                as *const libc::c_char,
            5119 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 51],
                &[libc::c_char; 51],
            >(b"int stateResortCompare(const void *, const void *)\0"))
                .as_ptr(),
        );
    }
    return n;
}
#[no_mangle]
pub unsafe extern "C" fn ResortStates(mut lemp: *mut lemon) {
    let mut i: libc::c_int = 0;
    let mut stp: *mut state = 0 as *mut state;
    let mut ap: *mut action = 0 as *mut action;
    i = 0 as libc::c_int;
    while i < (*lemp).nstate {
        stp = *((*lemp).sorted).offset(i as isize);
        let ref mut fresh208 = (*stp).nNtAct;
        *fresh208 = 0 as libc::c_int;
        (*stp).nTknAct = *fresh208;
        (*stp).iDfltReduce = -(1 as libc::c_int);
        (*stp).iTknOfst = -(2147483647 as libc::c_int);
        (*stp).iNtOfst = -(2147483647 as libc::c_int);
        ap = (*stp).ap;
        while !ap.is_null() {
            let mut iAction: libc::c_int = compute_action(lemp, ap);
            if iAction >= 0 as libc::c_int {
                if (*(*ap).sp).index < (*lemp).nterminal {
                    let ref mut fresh209 = (*stp).nTknAct;
                    *fresh209 += 1;
                } else if (*(*ap).sp).index < (*lemp).nsymbol {
                    let ref mut fresh210 = (*stp).nNtAct;
                    *fresh210 += 1;
                } else {
                    if (*stp).autoReduce == 0 as libc::c_int
                        || (*stp).pDfltReduce == (*ap).x.rp
                    {} else {
                        __assert_fail(
                            b"stp->autoReduce==0 || stp->pDfltReduce==ap->x.rp\0"
                                as *const u8 as *const libc::c_char,
                            b"/home/me/github/learnrust/libsql/tool/lemon.c\0"
                                as *const u8 as *const libc::c_char,
                            5148 as libc::c_int as libc::c_uint,
                            (*::std::mem::transmute::<
                                &[u8; 34],
                                &[libc::c_char; 34],
                            >(b"void ResortStates(struct lemon *)\0"))
                                .as_ptr(),
                        );
                    }
                    (*stp).iDfltReduce = iAction;
                }
            }
            ap = (*ap).next;
        }
        i += 1;
    }
    qsort(
        &mut *((*lemp).sorted).offset(1 as libc::c_int as isize) as *mut *mut state
            as *mut libc::c_void,
        ((*lemp).nstate - 1 as libc::c_int) as size_t,
        ::std::mem::size_of::<*mut state>() as libc::c_ulong,
        Some(
            stateResortCompare
                as unsafe extern "C" fn(
                    *const libc::c_void,
                    *const libc::c_void,
                ) -> libc::c_int,
        ),
    );
    i = 0 as libc::c_int;
    while i < (*lemp).nstate {
        (**((*lemp).sorted).offset(i as isize)).statenum = i;
        i += 1;
    }
    (*lemp).nxstate = (*lemp).nstate;
    while (*lemp).nxstate > 1 as libc::c_int
        && (**((*lemp).sorted).offset(((*lemp).nxstate - 1 as libc::c_int) as isize))
            .autoReduce != 0
    {
        let ref mut fresh211 = (*lemp).nxstate;
        *fresh211 -= 1;
    }
}
static mut size: libc::c_int = 0 as libc::c_int;
#[no_mangle]
pub unsafe extern "C" fn SetSize(mut n: libc::c_int) {
    size = n + 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn SetNew() -> *mut libc::c_char {
    let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
    s = calloc(size as libc::c_ulong, 1 as libc::c_int as libc::c_ulong)
        as *mut libc::c_char;
    if s.is_null() {
        memory_error();
    }
    return s;
}
#[no_mangle]
pub unsafe extern "C" fn SetFree(mut s: *mut libc::c_char) {
    free(s as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn SetAdd(
    mut s: *mut libc::c_char,
    mut e: libc::c_int,
) -> libc::c_int {
    let mut rv: libc::c_int = 0;
    if e >= 0 as libc::c_int && e < size {} else {
        __assert_fail(
            b"e>=0 && e<size\0" as *const u8 as *const libc::c_char,
            b"/home/me/github/learnrust/libsql/tool/lemon.c\0" as *const u8
                as *const libc::c_char,
            5200 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 24],
                &[libc::c_char; 24],
            >(b"int SetAdd(char *, int)\0"))
                .as_ptr(),
        );
    }
    rv = *s.offset(e as isize) as libc::c_int;
    *s.offset(e as isize) = 1 as libc::c_int as libc::c_char;
    return (rv == 0) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn SetUnion(
    mut s1: *mut libc::c_char,
    mut s2: *mut libc::c_char,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut progress: libc::c_int = 0;
    progress = 0 as libc::c_int;
    i = 0 as libc::c_int;
    while i < size {
        if !(*s2.offset(i as isize) as libc::c_int == 0 as libc::c_int) {
            if *s1.offset(i as isize) as libc::c_int == 0 as libc::c_int {
                progress = 1 as libc::c_int;
                *s1.offset(i as isize) = 1 as libc::c_int as libc::c_char;
            }
        }
        i += 1;
    }
    return progress;
}
#[no_mangle]
pub unsafe extern "C" fn strhash(mut x: *const libc::c_char) -> libc::c_uint {
    let mut h: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    while *x != 0 {
        let fresh212 = x;
        x = x.offset(1);
        h = h
            .wrapping_mul(13 as libc::c_int as libc::c_uint)
            .wrapping_add(*fresh212 as libc::c_uint);
    }
    return h;
}
#[no_mangle]
pub unsafe extern "C" fn Strsafe(mut y: *const libc::c_char) -> *const libc::c_char {
    let mut z: *const libc::c_char = 0 as *const libc::c_char;
    let mut cpy: *mut libc::c_char = 0 as *mut libc::c_char;
    if y.is_null() {
        return 0 as *const libc::c_char;
    }
    z = Strsafe_find(y);
    if z.is_null()
        && {
            cpy = malloc((strlen(y) as libc::c_int + 1 as libc::c_int) as libc::c_ulong)
                as *mut libc::c_char;
            !cpy.is_null()
        }
    {
        lemon_strcpy(cpy, y);
        z = cpy;
        Strsafe_insert(z);
    }
    if z.is_null() {
        memory_error();
    }
    return z;
}
static mut x1a: *mut s_x1 = 0 as *const s_x1 as *mut s_x1;
#[no_mangle]
pub unsafe extern "C" fn Strsafe_init() {
    if !x1a.is_null() {
        return;
    }
    x1a = malloc(::std::mem::size_of::<s_x1>() as libc::c_ulong) as *mut s_x1;
    if !x1a.is_null() {
        (*x1a).size = 1024 as libc::c_int;
        (*x1a).count = 0 as libc::c_int;
        let ref mut fresh213 = (*x1a).tbl;
        *fresh213 = calloc(
            1024 as libc::c_int as libc::c_ulong,
            (::std::mem::size_of::<x1node>() as libc::c_ulong)
                .wrapping_add(::std::mem::size_of::<*mut x1node>() as libc::c_ulong),
        ) as *mut x1node;
        if ((*x1a).tbl).is_null() {
            free(x1a as *mut libc::c_void);
            x1a = 0 as *mut s_x1;
        } else {
            let mut i: libc::c_int = 0;
            let ref mut fresh214 = (*x1a).ht;
            *fresh214 = &mut *((*x1a).tbl).offset(1024 as libc::c_int as isize)
                as *mut s_x1node as *mut *mut x1node;
            i = 0 as libc::c_int;
            while i < 1024 as libc::c_int {
                let ref mut fresh215 = *((*x1a).ht).offset(i as isize);
                *fresh215 = 0 as *mut s_x1node;
                i += 1;
            }
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn Strsafe_insert(mut data: *const libc::c_char) -> libc::c_int {
    let mut np: *mut x1node = 0 as *mut x1node;
    let mut h: libc::c_uint = 0;
    let mut ph: libc::c_uint = 0;
    if x1a.is_null() {
        return 0 as libc::c_int;
    }
    ph = strhash(data);
    h = ph & ((*x1a).size - 1 as libc::c_int) as libc::c_uint;
    np = *((*x1a).ht).offset(h as isize);
    while !np.is_null() {
        if strcmp((*np).data, data) == 0 as libc::c_int {
            return 0 as libc::c_int;
        }
        np = (*np).next;
    }
    if (*x1a).count >= (*x1a).size {
        let mut i: libc::c_int = 0;
        let mut arrSize: libc::c_int = 0;
        let mut array: s_x1 = s_x1 {
            size: 0,
            count: 0,
            tbl: 0 as *mut s_x1node,
            ht: 0 as *mut *mut s_x1node,
        };
        arrSize = (*x1a).size * 2 as libc::c_int;
        array.size = arrSize;
        array.count = (*x1a).count;
        array
            .tbl = calloc(
            arrSize as libc::c_ulong,
            (::std::mem::size_of::<x1node>() as libc::c_ulong)
                .wrapping_add(::std::mem::size_of::<*mut x1node>() as libc::c_ulong),
        ) as *mut x1node;
        if (array.tbl).is_null() {
            return 0 as libc::c_int;
        }
        array
            .ht = &mut *(array.tbl).offset(arrSize as isize) as *mut s_x1node
            as *mut *mut x1node;
        i = 0 as libc::c_int;
        while i < arrSize {
            let ref mut fresh216 = *(array.ht).offset(i as isize);
            *fresh216 = 0 as *mut s_x1node;
            i += 1;
        }
        i = 0 as libc::c_int;
        while i < (*x1a).count {
            let mut oldnp: *mut x1node = 0 as *mut x1node;
            let mut newnp: *mut x1node = 0 as *mut x1node;
            oldnp = &mut *((*x1a).tbl).offset(i as isize) as *mut s_x1node;
            h = strhash((*oldnp).data) & (arrSize - 1 as libc::c_int) as libc::c_uint;
            newnp = &mut *(array.tbl).offset(i as isize) as *mut s_x1node;
            if !(*(array.ht).offset(h as isize)).is_null() {
                let ref mut fresh217 = (**(array.ht).offset(h as isize)).from;
                *fresh217 = &mut (*newnp).next;
            }
            let ref mut fresh218 = (*newnp).next;
            *fresh218 = *(array.ht).offset(h as isize);
            let ref mut fresh219 = (*newnp).data;
            *fresh219 = (*oldnp).data;
            let ref mut fresh220 = (*newnp).from;
            *fresh220 = &mut *(array.ht).offset(h as isize) as *mut *mut s_x1node;
            let ref mut fresh221 = *(array.ht).offset(h as isize);
            *fresh221 = newnp;
            i += 1;
        }
        *x1a = array;
    }
    h = ph & ((*x1a).size - 1 as libc::c_int) as libc::c_uint;
    let ref mut fresh222 = (*x1a).count;
    let fresh223 = *fresh222;
    *fresh222 = *fresh222 + 1;
    np = &mut *((*x1a).tbl).offset(fresh223 as isize) as *mut s_x1node;
    let ref mut fresh224 = (*np).data;
    *fresh224 = data;
    if !(*((*x1a).ht).offset(h as isize)).is_null() {
        let ref mut fresh225 = (**((*x1a).ht).offset(h as isize)).from;
        *fresh225 = &mut (*np).next;
    }
    let ref mut fresh226 = (*np).next;
    *fresh226 = *((*x1a).ht).offset(h as isize);
    let ref mut fresh227 = *((*x1a).ht).offset(h as isize);
    *fresh227 = np;
    let ref mut fresh228 = (*np).from;
    *fresh228 = &mut *((*x1a).ht).offset(h as isize) as *mut *mut s_x1node;
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn Strsafe_find(
    mut key: *const libc::c_char,
) -> *const libc::c_char {
    let mut h: libc::c_uint = 0;
    let mut np: *mut x1node = 0 as *mut x1node;
    if x1a.is_null() {
        return 0 as *const libc::c_char;
    }
    h = strhash(key) & ((*x1a).size - 1 as libc::c_int) as libc::c_uint;
    np = *((*x1a).ht).offset(h as isize);
    while !np.is_null() {
        if strcmp((*np).data, key) == 0 as libc::c_int {
            break;
        }
        np = (*np).next;
    }
    return if !np.is_null() { (*np).data } else { 0 as *const libc::c_char };
}
#[no_mangle]
pub unsafe extern "C" fn Symbol_new(mut x: *const libc::c_char) -> *mut symbol {
    let mut sp: *mut symbol = 0 as *mut symbol;
    sp = Symbol_find(x);
    if sp.is_null() {
        sp = calloc(
            1 as libc::c_int as libc::c_ulong,
            ::std::mem::size_of::<symbol>() as libc::c_ulong,
        ) as *mut symbol;
        if sp.is_null() {
            memory_error();
        }
        let ref mut fresh229 = (*sp).name;
        *fresh229 = Strsafe(x);
        (*sp)
            .type_0 = (if *(*__ctype_b_loc())
            .offset(*x as libc::c_uchar as libc::c_int as isize) as libc::c_int
            & _ISupper as libc::c_int as libc::c_ushort as libc::c_int != 0
        {
            TERMINAL as libc::c_int
        } else {
            NONTERMINAL as libc::c_int
        }) as symbol_type;
        let ref mut fresh230 = (*sp).rule;
        *fresh230 = 0 as *mut rule;
        let ref mut fresh231 = (*sp).fallback;
        *fresh231 = 0 as *mut symbol;
        (*sp).prec = -(1 as libc::c_int);
        (*sp).assoc = UNK;
        let ref mut fresh232 = (*sp).firstset;
        *fresh232 = 0 as *mut libc::c_char;
        (*sp).lambda = LEMON_FALSE;
        let ref mut fresh233 = (*sp).destructor;
        *fresh233 = 0 as *mut libc::c_char;
        (*sp).destLineno = 0 as libc::c_int;
        let ref mut fresh234 = (*sp).datatype;
        *fresh234 = 0 as *mut libc::c_char;
        (*sp).useCnt = 0 as libc::c_int;
        Symbol_insert(sp, (*sp).name);
    }
    let ref mut fresh235 = (*sp).useCnt;
    *fresh235 += 1;
    return sp;
}
#[no_mangle]
pub unsafe extern "C" fn Symbolcmpp(
    mut _a: *const libc::c_void,
    mut _b: *const libc::c_void,
) -> libc::c_int {
    let mut a: *const symbol = *(_a as *mut *const symbol);
    let mut b: *const symbol = *(_b as *mut *const symbol);
    let mut i1: libc::c_int = if (*a).type_0 as libc::c_uint
        == MULTITERMINAL as libc::c_int as libc::c_uint
    {
        3 as libc::c_int
    } else if *((*a).name).offset(0 as libc::c_int as isize) as libc::c_int > 'Z' as i32
    {
        2 as libc::c_int
    } else {
        1 as libc::c_int
    };
    let mut i2: libc::c_int = if (*b).type_0 as libc::c_uint
        == MULTITERMINAL as libc::c_int as libc::c_uint
    {
        3 as libc::c_int
    } else if *((*b).name).offset(0 as libc::c_int as isize) as libc::c_int > 'Z' as i32
    {
        2 as libc::c_int
    } else {
        1 as libc::c_int
    };
    return if i1 == i2 { (*a).index - (*b).index } else { i1 - i2 };
}
static mut x2a: *mut s_x2 = 0 as *const s_x2 as *mut s_x2;
#[no_mangle]
pub unsafe extern "C" fn Symbol_init() {
    if !x2a.is_null() {
        return;
    }
    x2a = malloc(::std::mem::size_of::<s_x2>() as libc::c_ulong) as *mut s_x2;
    if !x2a.is_null() {
        (*x2a).size = 128 as libc::c_int;
        (*x2a).count = 0 as libc::c_int;
        let ref mut fresh236 = (*x2a).tbl;
        *fresh236 = calloc(
            128 as libc::c_int as libc::c_ulong,
            (::std::mem::size_of::<x2node>() as libc::c_ulong)
                .wrapping_add(::std::mem::size_of::<*mut x2node>() as libc::c_ulong),
        ) as *mut x2node;
        if ((*x2a).tbl).is_null() {
            free(x2a as *mut libc::c_void);
            x2a = 0 as *mut s_x2;
        } else {
            let mut i: libc::c_int = 0;
            let ref mut fresh237 = (*x2a).ht;
            *fresh237 = &mut *((*x2a).tbl).offset(128 as libc::c_int as isize)
                as *mut s_x2node as *mut *mut x2node;
            i = 0 as libc::c_int;
            while i < 128 as libc::c_int {
                let ref mut fresh238 = *((*x2a).ht).offset(i as isize);
                *fresh238 = 0 as *mut s_x2node;
                i += 1;
            }
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn Symbol_insert(
    mut data: *mut symbol,
    mut key: *const libc::c_char,
) -> libc::c_int {
    let mut np: *mut x2node = 0 as *mut x2node;
    let mut h: libc::c_uint = 0;
    let mut ph: libc::c_uint = 0;
    if x2a.is_null() {
        return 0 as libc::c_int;
    }
    ph = strhash(key);
    h = ph & ((*x2a).size - 1 as libc::c_int) as libc::c_uint;
    np = *((*x2a).ht).offset(h as isize);
    while !np.is_null() {
        if strcmp((*np).key, key) == 0 as libc::c_int {
            return 0 as libc::c_int;
        }
        np = (*np).next;
    }
    if (*x2a).count >= (*x2a).size {
        let mut i: libc::c_int = 0;
        let mut arrSize: libc::c_int = 0;
        let mut array: s_x2 = s_x2 {
            size: 0,
            count: 0,
            tbl: 0 as *mut s_x2node,
            ht: 0 as *mut *mut s_x2node,
        };
        arrSize = (*x2a).size * 2 as libc::c_int;
        array.size = arrSize;
        array.count = (*x2a).count;
        array
            .tbl = calloc(
            arrSize as libc::c_ulong,
            (::std::mem::size_of::<x2node>() as libc::c_ulong)
                .wrapping_add(::std::mem::size_of::<*mut x2node>() as libc::c_ulong),
        ) as *mut x2node;
        if (array.tbl).is_null() {
            return 0 as libc::c_int;
        }
        array
            .ht = &mut *(array.tbl).offset(arrSize as isize) as *mut s_x2node
            as *mut *mut x2node;
        i = 0 as libc::c_int;
        while i < arrSize {
            let ref mut fresh239 = *(array.ht).offset(i as isize);
            *fresh239 = 0 as *mut s_x2node;
            i += 1;
        }
        i = 0 as libc::c_int;
        while i < (*x2a).count {
            let mut oldnp: *mut x2node = 0 as *mut x2node;
            let mut newnp: *mut x2node = 0 as *mut x2node;
            oldnp = &mut *((*x2a).tbl).offset(i as isize) as *mut s_x2node;
            h = strhash((*oldnp).key) & (arrSize - 1 as libc::c_int) as libc::c_uint;
            newnp = &mut *(array.tbl).offset(i as isize) as *mut s_x2node;
            if !(*(array.ht).offset(h as isize)).is_null() {
                let ref mut fresh240 = (**(array.ht).offset(h as isize)).from;
                *fresh240 = &mut (*newnp).next;
            }
            let ref mut fresh241 = (*newnp).next;
            *fresh241 = *(array.ht).offset(h as isize);
            let ref mut fresh242 = (*newnp).key;
            *fresh242 = (*oldnp).key;
            let ref mut fresh243 = (*newnp).data;
            *fresh243 = (*oldnp).data;
            let ref mut fresh244 = (*newnp).from;
            *fresh244 = &mut *(array.ht).offset(h as isize) as *mut *mut s_x2node;
            let ref mut fresh245 = *(array.ht).offset(h as isize);
            *fresh245 = newnp;
            i += 1;
        }
        *x2a = array;
    }
    h = ph & ((*x2a).size - 1 as libc::c_int) as libc::c_uint;
    let ref mut fresh246 = (*x2a).count;
    let fresh247 = *fresh246;
    *fresh246 = *fresh246 + 1;
    np = &mut *((*x2a).tbl).offset(fresh247 as isize) as *mut s_x2node;
    let ref mut fresh248 = (*np).key;
    *fresh248 = key;
    let ref mut fresh249 = (*np).data;
    *fresh249 = data;
    if !(*((*x2a).ht).offset(h as isize)).is_null() {
        let ref mut fresh250 = (**((*x2a).ht).offset(h as isize)).from;
        *fresh250 = &mut (*np).next;
    }
    let ref mut fresh251 = (*np).next;
    *fresh251 = *((*x2a).ht).offset(h as isize);
    let ref mut fresh252 = *((*x2a).ht).offset(h as isize);
    *fresh252 = np;
    let ref mut fresh253 = (*np).from;
    *fresh253 = &mut *((*x2a).ht).offset(h as isize) as *mut *mut s_x2node;
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn Symbol_find(mut key: *const libc::c_char) -> *mut symbol {
    let mut h: libc::c_uint = 0;
    let mut np: *mut x2node = 0 as *mut x2node;
    if x2a.is_null() {
        return 0 as *mut symbol;
    }
    h = strhash(key) & ((*x2a).size - 1 as libc::c_int) as libc::c_uint;
    np = *((*x2a).ht).offset(h as isize);
    while !np.is_null() {
        if strcmp((*np).key, key) == 0 as libc::c_int {
            break;
        }
        np = (*np).next;
    }
    return if !np.is_null() { (*np).data } else { 0 as *mut symbol };
}
#[no_mangle]
pub unsafe extern "C" fn Symbol_Nth(mut n: libc::c_int) -> *mut symbol {
    let mut data: *mut symbol = 0 as *mut symbol;
    if !x2a.is_null() && n > 0 as libc::c_int && n <= (*x2a).count {
        data = (*((*x2a).tbl).offset((n - 1 as libc::c_int) as isize)).data;
    } else {
        data = 0 as *mut symbol;
    }
    return data;
}
#[no_mangle]
pub unsafe extern "C" fn Symbol_count() -> libc::c_int {
    return if !x2a.is_null() { (*x2a).count } else { 0 as libc::c_int };
}
#[no_mangle]
pub unsafe extern "C" fn Symbol_arrayof() -> *mut *mut symbol {
    let mut array: *mut *mut symbol = 0 as *mut *mut symbol;
    let mut i: libc::c_int = 0;
    let mut arrSize: libc::c_int = 0;
    if x2a.is_null() {
        return 0 as *mut *mut symbol;
    }
    arrSize = (*x2a).count;
    array = calloc(
        arrSize as libc::c_ulong,
        ::std::mem::size_of::<*mut symbol>() as libc::c_ulong,
    ) as *mut *mut symbol;
    if !array.is_null() {
        i = 0 as libc::c_int;
        while i < arrSize {
            let ref mut fresh254 = *array.offset(i as isize);
            *fresh254 = (*((*x2a).tbl).offset(i as isize)).data;
            i += 1;
        }
    }
    return array;
}
#[no_mangle]
pub unsafe extern "C" fn Configcmp(
    mut _a: *const libc::c_char,
    mut _b: *const libc::c_char,
) -> libc::c_int {
    let mut a: *const config = _a as *mut config;
    let mut b: *const config = _b as *mut config;
    let mut x: libc::c_int = 0;
    x = (*(*a).rp).index - (*(*b).rp).index;
    if x == 0 as libc::c_int {
        x = (*a).dot - (*b).dot;
    }
    return x;
}
#[no_mangle]
pub unsafe extern "C" fn statecmp(
    mut a: *mut config,
    mut b: *mut config,
) -> libc::c_int {
    let mut rc: libc::c_int = 0;
    rc = 0 as libc::c_int;
    while rc == 0 as libc::c_int && !a.is_null() && !b.is_null() {
        rc = (*(*a).rp).index - (*(*b).rp).index;
        if rc == 0 as libc::c_int {
            rc = (*a).dot - (*b).dot;
        }
        a = (*a).bp;
        b = (*b).bp;
    }
    if rc == 0 as libc::c_int {
        if !a.is_null() {
            rc = 1 as libc::c_int;
        }
        if !b.is_null() {
            rc = -(1 as libc::c_int);
        }
    }
    return rc;
}
#[no_mangle]
pub unsafe extern "C" fn statehash(mut a: *mut config) -> libc::c_uint {
    let mut h: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    while !a.is_null() {
        h = h
            .wrapping_mul(571 as libc::c_int as libc::c_uint)
            .wrapping_add(((*(*a).rp).index * 37 as libc::c_int) as libc::c_uint)
            .wrapping_add((*a).dot as libc::c_uint);
        a = (*a).bp;
    }
    return h;
}
#[no_mangle]
pub unsafe extern "C" fn State_new() -> *mut state {
    let mut newstate: *mut state = 0 as *mut state;
    newstate = calloc(
        1 as libc::c_int as libc::c_ulong,
        ::std::mem::size_of::<state>() as libc::c_ulong,
    ) as *mut state;
    if newstate.is_null() {
        memory_error();
    }
    return newstate;
}
static mut x3a: *mut s_x3 = 0 as *const s_x3 as *mut s_x3;
#[no_mangle]
pub unsafe extern "C" fn State_init() {
    if !x3a.is_null() {
        return;
    }
    x3a = malloc(::std::mem::size_of::<s_x3>() as libc::c_ulong) as *mut s_x3;
    if !x3a.is_null() {
        (*x3a).size = 128 as libc::c_int;
        (*x3a).count = 0 as libc::c_int;
        let ref mut fresh255 = (*x3a).tbl;
        *fresh255 = calloc(
            128 as libc::c_int as libc::c_ulong,
            (::std::mem::size_of::<x3node>() as libc::c_ulong)
                .wrapping_add(::std::mem::size_of::<*mut x3node>() as libc::c_ulong),
        ) as *mut x3node;
        if ((*x3a).tbl).is_null() {
            free(x3a as *mut libc::c_void);
            x3a = 0 as *mut s_x3;
        } else {
            let mut i: libc::c_int = 0;
            let ref mut fresh256 = (*x3a).ht;
            *fresh256 = &mut *((*x3a).tbl).offset(128 as libc::c_int as isize)
                as *mut s_x3node as *mut *mut x3node;
            i = 0 as libc::c_int;
            while i < 128 as libc::c_int {
                let ref mut fresh257 = *((*x3a).ht).offset(i as isize);
                *fresh257 = 0 as *mut s_x3node;
                i += 1;
            }
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn State_insert(
    mut data: *mut state,
    mut key: *mut config,
) -> libc::c_int {
    let mut np: *mut x3node = 0 as *mut x3node;
    let mut h: libc::c_uint = 0;
    let mut ph: libc::c_uint = 0;
    if x3a.is_null() {
        return 0 as libc::c_int;
    }
    ph = statehash(key);
    h = ph & ((*x3a).size - 1 as libc::c_int) as libc::c_uint;
    np = *((*x3a).ht).offset(h as isize);
    while !np.is_null() {
        if statecmp((*np).key, key) == 0 as libc::c_int {
            return 0 as libc::c_int;
        }
        np = (*np).next;
    }
    if (*x3a).count >= (*x3a).size {
        let mut i: libc::c_int = 0;
        let mut arrSize: libc::c_int = 0;
        let mut array: s_x3 = s_x3 {
            size: 0,
            count: 0,
            tbl: 0 as *mut s_x3node,
            ht: 0 as *mut *mut s_x3node,
        };
        arrSize = (*x3a).size * 2 as libc::c_int;
        array.size = arrSize;
        array.count = (*x3a).count;
        array
            .tbl = calloc(
            arrSize as libc::c_ulong,
            (::std::mem::size_of::<x3node>() as libc::c_ulong)
                .wrapping_add(::std::mem::size_of::<*mut x3node>() as libc::c_ulong),
        ) as *mut x3node;
        if (array.tbl).is_null() {
            return 0 as libc::c_int;
        }
        array
            .ht = &mut *(array.tbl).offset(arrSize as isize) as *mut s_x3node
            as *mut *mut x3node;
        i = 0 as libc::c_int;
        while i < arrSize {
            let ref mut fresh258 = *(array.ht).offset(i as isize);
            *fresh258 = 0 as *mut s_x3node;
            i += 1;
        }
        i = 0 as libc::c_int;
        while i < (*x3a).count {
            let mut oldnp: *mut x3node = 0 as *mut x3node;
            let mut newnp: *mut x3node = 0 as *mut x3node;
            oldnp = &mut *((*x3a).tbl).offset(i as isize) as *mut s_x3node;
            h = statehash((*oldnp).key) & (arrSize - 1 as libc::c_int) as libc::c_uint;
            newnp = &mut *(array.tbl).offset(i as isize) as *mut s_x3node;
            if !(*(array.ht).offset(h as isize)).is_null() {
                let ref mut fresh259 = (**(array.ht).offset(h as isize)).from;
                *fresh259 = &mut (*newnp).next;
            }
            let ref mut fresh260 = (*newnp).next;
            *fresh260 = *(array.ht).offset(h as isize);
            let ref mut fresh261 = (*newnp).key;
            *fresh261 = (*oldnp).key;
            let ref mut fresh262 = (*newnp).data;
            *fresh262 = (*oldnp).data;
            let ref mut fresh263 = (*newnp).from;
            *fresh263 = &mut *(array.ht).offset(h as isize) as *mut *mut s_x3node;
            let ref mut fresh264 = *(array.ht).offset(h as isize);
            *fresh264 = newnp;
            i += 1;
        }
        free((*x3a).tbl as *mut libc::c_void);
        *x3a = array;
    }
    h = ph & ((*x3a).size - 1 as libc::c_int) as libc::c_uint;
    let ref mut fresh265 = (*x3a).count;
    let fresh266 = *fresh265;
    *fresh265 = *fresh265 + 1;
    np = &mut *((*x3a).tbl).offset(fresh266 as isize) as *mut s_x3node;
    let ref mut fresh267 = (*np).key;
    *fresh267 = key;
    let ref mut fresh268 = (*np).data;
    *fresh268 = data;
    if !(*((*x3a).ht).offset(h as isize)).is_null() {
        let ref mut fresh269 = (**((*x3a).ht).offset(h as isize)).from;
        *fresh269 = &mut (*np).next;
    }
    let ref mut fresh270 = (*np).next;
    *fresh270 = *((*x3a).ht).offset(h as isize);
    let ref mut fresh271 = *((*x3a).ht).offset(h as isize);
    *fresh271 = np;
    let ref mut fresh272 = (*np).from;
    *fresh272 = &mut *((*x3a).ht).offset(h as isize) as *mut *mut s_x3node;
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn State_find(mut key: *mut config) -> *mut state {
    let mut h: libc::c_uint = 0;
    let mut np: *mut x3node = 0 as *mut x3node;
    if x3a.is_null() {
        return 0 as *mut state;
    }
    h = statehash(key) & ((*x3a).size - 1 as libc::c_int) as libc::c_uint;
    np = *((*x3a).ht).offset(h as isize);
    while !np.is_null() {
        if statecmp((*np).key, key) == 0 as libc::c_int {
            break;
        }
        np = (*np).next;
    }
    return if !np.is_null() { (*np).data } else { 0 as *mut state };
}
#[no_mangle]
pub unsafe extern "C" fn State_arrayof() -> *mut *mut state {
    let mut array: *mut *mut state = 0 as *mut *mut state;
    let mut i: libc::c_int = 0;
    let mut arrSize: libc::c_int = 0;
    if x3a.is_null() {
        return 0 as *mut *mut state;
    }
    arrSize = (*x3a).count;
    array = calloc(
        arrSize as libc::c_ulong,
        ::std::mem::size_of::<*mut state>() as libc::c_ulong,
    ) as *mut *mut state;
    if !array.is_null() {
        i = 0 as libc::c_int;
        while i < arrSize {
            let ref mut fresh273 = *array.offset(i as isize);
            *fresh273 = (*((*x3a).tbl).offset(i as isize)).data;
            i += 1;
        }
    }
    return array;
}
#[no_mangle]
pub unsafe extern "C" fn confighash(mut a: *mut config) -> libc::c_uint {
    let mut h: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    h = h
        .wrapping_mul(571 as libc::c_int as libc::c_uint)
        .wrapping_add(((*(*a).rp).index * 37 as libc::c_int) as libc::c_uint)
        .wrapping_add((*a).dot as libc::c_uint);
    return h;
}
static mut x4a: *mut s_x4 = 0 as *const s_x4 as *mut s_x4;
#[no_mangle]
pub unsafe extern "C" fn Configtable_init() {
    if !x4a.is_null() {
        return;
    }
    x4a = malloc(::std::mem::size_of::<s_x4>() as libc::c_ulong) as *mut s_x4;
    if !x4a.is_null() {
        (*x4a).size = 64 as libc::c_int;
        (*x4a).count = 0 as libc::c_int;
        let ref mut fresh274 = (*x4a).tbl;
        *fresh274 = calloc(
            64 as libc::c_int as libc::c_ulong,
            (::std::mem::size_of::<x4node>() as libc::c_ulong)
                .wrapping_add(::std::mem::size_of::<*mut x4node>() as libc::c_ulong),
        ) as *mut x4node;
        if ((*x4a).tbl).is_null() {
            free(x4a as *mut libc::c_void);
            x4a = 0 as *mut s_x4;
        } else {
            let mut i: libc::c_int = 0;
            let ref mut fresh275 = (*x4a).ht;
            *fresh275 = &mut *((*x4a).tbl).offset(64 as libc::c_int as isize)
                as *mut s_x4node as *mut *mut x4node;
            i = 0 as libc::c_int;
            while i < 64 as libc::c_int {
                let ref mut fresh276 = *((*x4a).ht).offset(i as isize);
                *fresh276 = 0 as *mut s_x4node;
                i += 1;
            }
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn Configtable_insert(mut data: *mut config) -> libc::c_int {
    let mut np: *mut x4node = 0 as *mut x4node;
    let mut h: libc::c_uint = 0;
    let mut ph: libc::c_uint = 0;
    if x4a.is_null() {
        return 0 as libc::c_int;
    }
    ph = confighash(data);
    h = ph & ((*x4a).size - 1 as libc::c_int) as libc::c_uint;
    np = *((*x4a).ht).offset(h as isize);
    while !np.is_null() {
        if Configcmp((*np).data as *const libc::c_char, data as *const libc::c_char)
            == 0 as libc::c_int
        {
            return 0 as libc::c_int;
        }
        np = (*np).next;
    }
    if (*x4a).count >= (*x4a).size {
        let mut i: libc::c_int = 0;
        let mut arrSize: libc::c_int = 0;
        let mut array: s_x4 = s_x4 {
            size: 0,
            count: 0,
            tbl: 0 as *mut s_x4node,
            ht: 0 as *mut *mut s_x4node,
        };
        arrSize = (*x4a).size * 2 as libc::c_int;
        array.size = arrSize;
        array.count = (*x4a).count;
        array
            .tbl = calloc(
            arrSize as libc::c_ulong,
            (::std::mem::size_of::<x4node>() as libc::c_ulong)
                .wrapping_add(::std::mem::size_of::<*mut x4node>() as libc::c_ulong),
        ) as *mut x4node;
        if (array.tbl).is_null() {
            return 0 as libc::c_int;
        }
        array
            .ht = &mut *(array.tbl).offset(arrSize as isize) as *mut s_x4node
            as *mut *mut x4node;
        i = 0 as libc::c_int;
        while i < arrSize {
            let ref mut fresh277 = *(array.ht).offset(i as isize);
            *fresh277 = 0 as *mut s_x4node;
            i += 1;
        }
        i = 0 as libc::c_int;
        while i < (*x4a).count {
            let mut oldnp: *mut x4node = 0 as *mut x4node;
            let mut newnp: *mut x4node = 0 as *mut x4node;
            oldnp = &mut *((*x4a).tbl).offset(i as isize) as *mut s_x4node;
            h = confighash((*oldnp).data) & (arrSize - 1 as libc::c_int) as libc::c_uint;
            newnp = &mut *(array.tbl).offset(i as isize) as *mut s_x4node;
            if !(*(array.ht).offset(h as isize)).is_null() {
                let ref mut fresh278 = (**(array.ht).offset(h as isize)).from;
                *fresh278 = &mut (*newnp).next;
            }
            let ref mut fresh279 = (*newnp).next;
            *fresh279 = *(array.ht).offset(h as isize);
            let ref mut fresh280 = (*newnp).data;
            *fresh280 = (*oldnp).data;
            let ref mut fresh281 = (*newnp).from;
            *fresh281 = &mut *(array.ht).offset(h as isize) as *mut *mut s_x4node;
            let ref mut fresh282 = *(array.ht).offset(h as isize);
            *fresh282 = newnp;
            i += 1;
        }
        *x4a = array;
    }
    h = ph & ((*x4a).size - 1 as libc::c_int) as libc::c_uint;
    let ref mut fresh283 = (*x4a).count;
    let fresh284 = *fresh283;
    *fresh283 = *fresh283 + 1;
    np = &mut *((*x4a).tbl).offset(fresh284 as isize) as *mut s_x4node;
    let ref mut fresh285 = (*np).data;
    *fresh285 = data;
    if !(*((*x4a).ht).offset(h as isize)).is_null() {
        let ref mut fresh286 = (**((*x4a).ht).offset(h as isize)).from;
        *fresh286 = &mut (*np).next;
    }
    let ref mut fresh287 = (*np).next;
    *fresh287 = *((*x4a).ht).offset(h as isize);
    let ref mut fresh288 = *((*x4a).ht).offset(h as isize);
    *fresh288 = np;
    let ref mut fresh289 = (*np).from;
    *fresh289 = &mut *((*x4a).ht).offset(h as isize) as *mut *mut s_x4node;
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn Configtable_find(mut key: *mut config) -> *mut config {
    let mut h: libc::c_int = 0;
    let mut np: *mut x4node = 0 as *mut x4node;
    if x4a.is_null() {
        return 0 as *mut config;
    }
    h = (confighash(key) & ((*x4a).size - 1 as libc::c_int) as libc::c_uint)
        as libc::c_int;
    np = *((*x4a).ht).offset(h as isize);
    while !np.is_null() {
        if Configcmp((*np).data as *const libc::c_char, key as *const libc::c_char)
            == 0 as libc::c_int
        {
            break;
        }
        np = (*np).next;
    }
    return if !np.is_null() { (*np).data } else { 0 as *mut config };
}
#[no_mangle]
pub unsafe extern "C" fn Configtable_clear(
    mut f: Option::<unsafe extern "C" fn(*mut config) -> libc::c_int>,
) {
    let mut i: libc::c_int = 0;
    if x4a.is_null() || (*x4a).count == 0 as libc::c_int {
        return;
    }
    if f.is_some() {
        i = 0 as libc::c_int;
        while i < (*x4a).count {
            (Some(f.expect("non-null function pointer")))
                .expect(
                    "non-null function pointer",
                )((*((*x4a).tbl).offset(i as isize)).data);
            i += 1;
        }
    }
    i = 0 as libc::c_int;
    while i < (*x4a).size {
        let ref mut fresh290 = *((*x4a).ht).offset(i as isize);
        *fresh290 = 0 as *mut s_x4node;
        i += 1;
    }
    (*x4a).count = 0 as libc::c_int;
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
