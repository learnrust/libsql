use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type stat;
    pub type Tcl_AsyncHandler_;
    pub type Tcl_Channel_;
    pub type Tcl_ChannelTypeVersion_;
    pub type Tcl_Command_;
    pub type Tcl_Condition_;
    pub type Tcl_Dict_;
    pub type Tcl_EncodingState_;
    pub type Tcl_Encoding_;
    pub type Tcl_InterpState_;
    pub type Tcl_LoadHandle_;
    pub type Tcl_Mutex_;
    pub type Tcl_Pid_;
    pub type Tcl_RegExp_;
    pub type Tcl_ThreadDataKey_;
    pub type Tcl_ThreadId_;
    pub type Tcl_TimerToken_;
    pub type Tcl_Trace_;
    pub type Tcl_ZLibStream_;
    pub type utimbuf;
    pub type Tcl_FSVersion_;
    pub type mp_int;
    pub type TclIntPlatStubs;
    pub type TclIntStubs;
    pub type sqlite3;
    pub type sqlite3_stmt;
    pub type sqlite3_value;
    pub type sqlite3_context;
    pub type sqlite3_blob;
    pub type sqlite3_backup;
    static mut tclStubsPtr: *const TclStubs;
    fn fgets(
        __s: *mut libc::c_char,
        __n: libc::c_int,
        __stream: *mut FILE,
    ) -> *mut libc::c_char;
    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    fn Tcl_InitStubs(
        interp: *mut Tcl_Interp,
        version: *const libc::c_char,
        exact: libc::c_int,
    ) -> *const libc::c_char;
    fn sqlite3_libversion() -> *const libc::c_char;
    fn sqlite3_sourceid() -> *const libc::c_char;
    fn sqlite3_close(_: *mut sqlite3) -> libc::c_int;
    fn sqlite3_exec(
        _: *mut sqlite3,
        sql: *const libc::c_char,
        callback: Option::<
            unsafe extern "C" fn(
                *mut libc::c_void,
                libc::c_int,
                *mut *mut libc::c_char,
                *mut *mut libc::c_char,
            ) -> libc::c_int,
        >,
        _: *mut libc::c_void,
        errmsg: *mut *mut libc::c_char,
    ) -> libc::c_int;
    fn sqlite3_db_config(_: *mut sqlite3, op: libc::c_int, _: ...) -> libc::c_int;
    fn sqlite3_last_insert_rowid(_: *mut sqlite3) -> sqlite3_int64;
    fn sqlite3_changes64(_: *mut sqlite3) -> sqlite3_int64;
    fn sqlite3_total_changes64(_: *mut sqlite3) -> sqlite3_int64;
    fn sqlite3_interrupt(_: *mut sqlite3);
    fn sqlite3_complete(sql: *const libc::c_char) -> libc::c_int;
    fn sqlite3_busy_handler(
        _: *mut sqlite3,
        _: Option::<unsafe extern "C" fn(*mut libc::c_void, libc::c_int) -> libc::c_int>,
        _: *mut libc::c_void,
    ) -> libc::c_int;
    fn sqlite3_busy_timeout(_: *mut sqlite3, ms: libc::c_int) -> libc::c_int;
    fn sqlite3_mprintf(_: *const libc::c_char, _: ...) -> *mut libc::c_char;
    fn sqlite3_snprintf(
        _: libc::c_int,
        _: *mut libc::c_char,
        _: *const libc::c_char,
        _: ...
    ) -> *mut libc::c_char;
    fn sqlite3_malloc64(_: sqlite3_uint64) -> *mut libc::c_void;
    fn sqlite3_free(_: *mut libc::c_void);
    fn sqlite3_set_authorizer(
        _: *mut sqlite3,
        xAuth: Option::<
            unsafe extern "C" fn(
                *mut libc::c_void,
                libc::c_int,
                *const libc::c_char,
                *const libc::c_char,
                *const libc::c_char,
                *const libc::c_char,
            ) -> libc::c_int,
        >,
        pUserData: *mut libc::c_void,
    ) -> libc::c_int;
    fn sqlite3_trace(
        _: *mut sqlite3,
        xTrace: Option::<
            unsafe extern "C" fn(*mut libc::c_void, *const libc::c_char) -> (),
        >,
        _: *mut libc::c_void,
    ) -> *mut libc::c_void;
    fn sqlite3_profile(
        _: *mut sqlite3,
        xProfile: Option::<
            unsafe extern "C" fn(
                *mut libc::c_void,
                *const libc::c_char,
                sqlite3_uint64,
            ) -> (),
        >,
        _: *mut libc::c_void,
    ) -> *mut libc::c_void;
    fn sqlite3_trace_v2(
        _: *mut sqlite3,
        uMask: libc::c_uint,
        xCallback: Option::<
            unsafe extern "C" fn(
                libc::c_uint,
                *mut libc::c_void,
                *mut libc::c_void,
                *mut libc::c_void,
            ) -> libc::c_int,
        >,
        pCtx: *mut libc::c_void,
    ) -> libc::c_int;
    fn sqlite3_progress_handler(
        _: *mut sqlite3,
        _: libc::c_int,
        _: Option::<unsafe extern "C" fn(*mut libc::c_void) -> libc::c_int>,
        _: *mut libc::c_void,
    );
    fn sqlite3_open_v2(
        filename: *const libc::c_char,
        ppDb: *mut *mut sqlite3,
        flags: libc::c_int,
        zVfs: *const libc::c_char,
    ) -> libc::c_int;
    fn sqlite3_errcode(db: *mut sqlite3) -> libc::c_int;
    fn sqlite3_errmsg(_: *mut sqlite3) -> *const libc::c_char;
    fn sqlite3_errstr(_: libc::c_int) -> *const libc::c_char;
    fn sqlite3_error_offset(db: *mut sqlite3) -> libc::c_int;
    fn sqlite3_prepare(
        db: *mut sqlite3,
        zSql: *const libc::c_char,
        nByte: libc::c_int,
        ppStmt: *mut *mut sqlite3_stmt,
        pzTail: *mut *const libc::c_char,
    ) -> libc::c_int;
    fn sqlite3_prepare_v3(
        db: *mut sqlite3,
        zSql: *const libc::c_char,
        nByte: libc::c_int,
        prepFlags: libc::c_uint,
        ppStmt: *mut *mut sqlite3_stmt,
        pzTail: *mut *const libc::c_char,
    ) -> libc::c_int;
    fn sqlite3_sql(pStmt: *mut sqlite3_stmt) -> *const libc::c_char;
    fn sqlite3_bind_blob(
        _: *mut sqlite3_stmt,
        _: libc::c_int,
        _: *const libc::c_void,
        n: libc::c_int,
        _: Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
    ) -> libc::c_int;
    fn sqlite3_bind_double(
        _: *mut sqlite3_stmt,
        _: libc::c_int,
        _: libc::c_double,
    ) -> libc::c_int;
    fn sqlite3_bind_int(
        _: *mut sqlite3_stmt,
        _: libc::c_int,
        _: libc::c_int,
    ) -> libc::c_int;
    fn sqlite3_bind_int64(
        _: *mut sqlite3_stmt,
        _: libc::c_int,
        _: sqlite3_int64,
    ) -> libc::c_int;
    fn sqlite3_bind_null(_: *mut sqlite3_stmt, _: libc::c_int) -> libc::c_int;
    fn sqlite3_bind_text(
        _: *mut sqlite3_stmt,
        _: libc::c_int,
        _: *const libc::c_char,
        _: libc::c_int,
        _: Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
    ) -> libc::c_int;
    fn sqlite3_bind_parameter_count(_: *mut sqlite3_stmt) -> libc::c_int;
    fn sqlite3_bind_parameter_name(
        _: *mut sqlite3_stmt,
        _: libc::c_int,
    ) -> *const libc::c_char;
    fn sqlite3_column_count(pStmt: *mut sqlite3_stmt) -> libc::c_int;
    fn sqlite3_column_name(_: *mut sqlite3_stmt, N: libc::c_int) -> *const libc::c_char;
    fn sqlite3_step(_: *mut sqlite3_stmt) -> libc::c_int;
    fn sqlite3_column_blob(
        _: *mut sqlite3_stmt,
        iCol: libc::c_int,
    ) -> *const libc::c_void;
    fn sqlite3_column_double(_: *mut sqlite3_stmt, iCol: libc::c_int) -> libc::c_double;
    fn sqlite3_column_int64(_: *mut sqlite3_stmt, iCol: libc::c_int) -> sqlite3_int64;
    fn sqlite3_column_text(
        _: *mut sqlite3_stmt,
        iCol: libc::c_int,
    ) -> *const libc::c_uchar;
    fn sqlite3_column_bytes(_: *mut sqlite3_stmt, iCol: libc::c_int) -> libc::c_int;
    fn sqlite3_column_type(_: *mut sqlite3_stmt, iCol: libc::c_int) -> libc::c_int;
    fn sqlite3_finalize(pStmt: *mut sqlite3_stmt) -> libc::c_int;
    fn sqlite3_reset(pStmt: *mut sqlite3_stmt) -> libc::c_int;
    fn sqlite3_create_function(
        db: *mut sqlite3,
        zFunctionName: *const libc::c_char,
        nArg: libc::c_int,
        eTextRep: libc::c_int,
        pApp: *mut libc::c_void,
        xFunc: Option::<
            unsafe extern "C" fn(
                *mut sqlite3_context,
                libc::c_int,
                *mut *mut sqlite3_value,
            ) -> (),
        >,
        xStep: Option::<
            unsafe extern "C" fn(
                *mut sqlite3_context,
                libc::c_int,
                *mut *mut sqlite3_value,
            ) -> (),
        >,
        xFinal: Option::<unsafe extern "C" fn(*mut sqlite3_context) -> ()>,
    ) -> libc::c_int;
    fn sqlite3_value_blob(_: *mut sqlite3_value) -> *const libc::c_void;
    fn sqlite3_value_double(_: *mut sqlite3_value) -> libc::c_double;
    fn sqlite3_value_int64(_: *mut sqlite3_value) -> sqlite3_int64;
    fn sqlite3_value_text(_: *mut sqlite3_value) -> *const libc::c_uchar;
    fn sqlite3_value_bytes(_: *mut sqlite3_value) -> libc::c_int;
    fn sqlite3_value_type(_: *mut sqlite3_value) -> libc::c_int;
    fn sqlite3_user_data(_: *mut sqlite3_context) -> *mut libc::c_void;
    fn sqlite3_result_blob(
        _: *mut sqlite3_context,
        _: *const libc::c_void,
        _: libc::c_int,
        _: Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
    );
    fn sqlite3_result_double(_: *mut sqlite3_context, _: libc::c_double);
    fn sqlite3_result_error(
        _: *mut sqlite3_context,
        _: *const libc::c_char,
        _: libc::c_int,
    );
    fn sqlite3_result_int64(_: *mut sqlite3_context, _: sqlite3_int64);
    fn sqlite3_result_text(
        _: *mut sqlite3_context,
        _: *const libc::c_char,
        _: libc::c_int,
        _: Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
    );
    fn sqlite3_create_collation(
        _: *mut sqlite3,
        zName: *const libc::c_char,
        eTextRep: libc::c_int,
        pArg: *mut libc::c_void,
        xCompare: Option::<
            unsafe extern "C" fn(
                *mut libc::c_void,
                libc::c_int,
                *const libc::c_void,
                libc::c_int,
                *const libc::c_void,
            ) -> libc::c_int,
        >,
    ) -> libc::c_int;
    fn sqlite3_collation_needed(
        _: *mut sqlite3,
        _: *mut libc::c_void,
        _: Option::<
            unsafe extern "C" fn(
                *mut libc::c_void,
                *mut sqlite3,
                libc::c_int,
                *const libc::c_char,
            ) -> (),
        >,
    ) -> libc::c_int;
    fn sqlite3_sleep(_: libc::c_int) -> libc::c_int;
    fn sqlite3_commit_hook(
        _: *mut sqlite3,
        _: Option::<unsafe extern "C" fn(*mut libc::c_void) -> libc::c_int>,
        _: *mut libc::c_void,
    ) -> *mut libc::c_void;
    fn sqlite3_rollback_hook(
        _: *mut sqlite3,
        _: Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
        _: *mut libc::c_void,
    ) -> *mut libc::c_void;
    fn sqlite3_update_hook(
        _: *mut sqlite3,
        _: Option::<
            unsafe extern "C" fn(
                *mut libc::c_void,
                libc::c_int,
                *const libc::c_char,
                *const libc::c_char,
                sqlite3_int64,
            ) -> (),
        >,
        _: *mut libc::c_void,
    ) -> *mut libc::c_void;
    fn sqlite3_enable_load_extension(
        db: *mut sqlite3,
        onoff: libc::c_int,
    ) -> libc::c_int;
    fn sqlite3_blob_open(
        _: *mut sqlite3,
        zDb: *const libc::c_char,
        zTable: *const libc::c_char,
        zColumn: *const libc::c_char,
        iRow: sqlite3_int64,
        flags: libc::c_int,
        ppBlob: *mut *mut sqlite3_blob,
    ) -> libc::c_int;
    fn sqlite3_blob_close(_: *mut sqlite3_blob) -> libc::c_int;
    fn sqlite3_blob_bytes(_: *mut sqlite3_blob) -> libc::c_int;
    fn sqlite3_blob_read(
        _: *mut sqlite3_blob,
        Z: *mut libc::c_void,
        N: libc::c_int,
        iOffset: libc::c_int,
    ) -> libc::c_int;
    fn sqlite3_blob_write(
        _: *mut sqlite3_blob,
        z: *const libc::c_void,
        n: libc::c_int,
        iOffset: libc::c_int,
    ) -> libc::c_int;
    fn sqlite3_file_control(
        _: *mut sqlite3,
        zDbName: *const libc::c_char,
        op: libc::c_int,
        _: *mut libc::c_void,
    ) -> libc::c_int;
    fn sqlite3_stmt_status(
        _: *mut sqlite3_stmt,
        op: libc::c_int,
        resetFlg: libc::c_int,
    ) -> libc::c_int;
    fn sqlite3_backup_init(
        pDest: *mut sqlite3,
        zDestName: *const libc::c_char,
        pSource: *mut sqlite3,
        zSourceName: *const libc::c_char,
    ) -> *mut sqlite3_backup;
    fn sqlite3_backup_step(p: *mut sqlite3_backup, nPage: libc::c_int) -> libc::c_int;
    fn sqlite3_backup_finish(p: *mut sqlite3_backup) -> libc::c_int;
    fn sqlite3_stricmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn sqlite3_wal_hook(
        _: *mut sqlite3,
        _: Option::<
            unsafe extern "C" fn(
                *mut libc::c_void,
                *mut sqlite3,
                *const libc::c_char,
                libc::c_int,
            ) -> libc::c_int,
        >,
        _: *mut libc::c_void,
    ) -> *mut libc::c_void;
    fn sqlite3_serialize(
        db: *mut sqlite3,
        zSchema: *const libc::c_char,
        piSize: *mut sqlite3_int64,
        mFlags: libc::c_uint,
    ) -> *mut libc::c_uchar;
    fn sqlite3_deserialize(
        db: *mut sqlite3,
        zSchema: *const libc::c_char,
        pData: *mut libc::c_uchar,
        szDb: sqlite3_int64,
        szBuf: sqlite3_int64,
        mFlags: libc::c_uint,
    ) -> libc::c_int;
    fn strtol(
        _: *const libc::c_char,
        _: *mut *mut libc::c_char,
        _: libc::c_int,
    ) -> libc::c_long;
    fn free(_: *mut libc::c_void);
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn strncmp(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn memcmp(
        _: *const libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __va_list_tag {
    pub gp_offset: libc::c_uint,
    pub fp_offset: libc::c_uint,
    pub overflow_arg_area: *mut libc::c_void,
    pub reg_save_area: *mut libc::c_void,
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
pub type ClientData = *mut libc::c_void;
pub type Tcl_WideInt = libc::c_longlong;
pub type Tcl_WideUInt = libc::c_ulonglong;
pub type Tcl_StatBuf = stat;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Tcl_Interp {
    pub resultDontUse: *mut libc::c_char,
    pub freeProcDontUse: Option::<unsafe extern "C" fn(*mut libc::c_char) -> ()>,
    pub errorLineDontUse: libc::c_int,
}
pub type Tcl_AsyncHandler = *mut Tcl_AsyncHandler_;
pub type Tcl_Channel = *mut Tcl_Channel_;
pub type Tcl_ChannelTypeVersion = *mut Tcl_ChannelTypeVersion_;
pub type Tcl_Command = *mut Tcl_Command_;
pub type Tcl_Condition = *mut Tcl_Condition_;
pub type Tcl_Dict = *mut Tcl_Dict_;
pub type Tcl_EncodingState = *mut Tcl_EncodingState_;
pub type Tcl_Encoding = *mut Tcl_Encoding_;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Tcl_Event {
    pub proc_0: Option::<Tcl_EventProc>,
    pub nextPtr: *mut Tcl_Event,
}
pub type Tcl_EventProc = unsafe extern "C" fn(
    *mut Tcl_Event,
    libc::c_int,
) -> libc::c_int;
pub type Tcl_InterpState = *mut Tcl_InterpState_;
pub type Tcl_LoadHandle = *mut Tcl_LoadHandle_;
pub type Tcl_Mutex = *mut Tcl_Mutex_;
pub type Tcl_Pid = *mut Tcl_Pid_;
pub type Tcl_RegExp = *mut Tcl_RegExp_;
pub type Tcl_ThreadDataKey = *mut Tcl_ThreadDataKey_;
pub type Tcl_ThreadId = *mut Tcl_ThreadId_;
pub type Tcl_TimerToken = *mut Tcl_TimerToken_;
pub type Tcl_Trace = *mut Tcl_Trace_;
pub type Tcl_ZlibStream = *mut Tcl_ZLibStream_;
pub type Tcl_ThreadCreateProc = unsafe extern "C" fn(ClientData) -> ();
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Tcl_RegExpIndices {
    pub start: libc::c_long,
    pub end: libc::c_long,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Tcl_RegExpInfo {
    pub nsubs: libc::c_int,
    pub matches: *mut Tcl_RegExpIndices,
    pub extendStart: libc::c_long,
    pub reserved: libc::c_long,
}
pub type Tcl_ValueType = libc::c_uint;
pub const TCL_WIDE_INT: Tcl_ValueType = 3;
pub const TCL_EITHER: Tcl_ValueType = 2;
pub const TCL_DOUBLE: Tcl_ValueType = 1;
pub const TCL_INT: Tcl_ValueType = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Tcl_Value {
    pub type_0: Tcl_ValueType,
    pub intValue: libc::c_long,
    pub doubleValue: libc::c_double,
    pub wideValue: Tcl_WideInt,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Tcl_Obj {
    pub refCount: libc::c_int,
    pub bytes: *mut libc::c_char,
    pub length: libc::c_int,
    pub typePtr: *const Tcl_ObjType,
    pub internalRep: C2RustUnnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub longValue: libc::c_long,
    pub doubleValue: libc::c_double,
    pub otherValuePtr: *mut libc::c_void,
    pub wideValue: Tcl_WideInt,
    pub twoPtrValue: C2RustUnnamed_1,
    pub ptrAndLongRep: C2RustUnnamed_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_0 {
    pub ptr: *mut libc::c_void,
    pub value: libc::c_ulong,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_1 {
    pub ptr1: *mut libc::c_void,
    pub ptr2: *mut libc::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Tcl_ObjType {
    pub name: *const libc::c_char,
    pub freeIntRepProc: Option::<Tcl_FreeInternalRepProc>,
    pub dupIntRepProc: Option::<Tcl_DupInternalRepProc>,
    pub updateStringProc: Option::<Tcl_UpdateStringProc>,
    pub setFromAnyProc: Option::<Tcl_SetFromAnyProc>,
}
pub type Tcl_SetFromAnyProc = unsafe extern "C" fn(
    *mut Tcl_Interp,
    *mut Tcl_Obj,
) -> libc::c_int;
pub type Tcl_UpdateStringProc = unsafe extern "C" fn(*mut Tcl_Obj) -> ();
pub type Tcl_DupInternalRepProc = unsafe extern "C" fn(*mut Tcl_Obj, *mut Tcl_Obj) -> ();
pub type Tcl_FreeInternalRepProc = unsafe extern "C" fn(*mut Tcl_Obj) -> ();
pub type Tcl_AsyncProc = unsafe extern "C" fn(
    ClientData,
    *mut Tcl_Interp,
    libc::c_int,
) -> libc::c_int;
pub type Tcl_ChannelProc = unsafe extern "C" fn(ClientData, libc::c_int) -> ();
pub type Tcl_CloseProc = unsafe extern "C" fn(ClientData) -> ();
pub type Tcl_CmdDeleteProc = unsafe extern "C" fn(ClientData) -> ();
pub type Tcl_CmdProc = unsafe extern "C" fn(
    ClientData,
    *mut Tcl_Interp,
    libc::c_int,
    *mut *const libc::c_char,
) -> libc::c_int;
pub type Tcl_CmdTraceProc = unsafe extern "C" fn(
    ClientData,
    *mut Tcl_Interp,
    libc::c_int,
    *mut libc::c_char,
    Option::<Tcl_CmdProc>,
    ClientData,
    libc::c_int,
    *mut *const libc::c_char,
) -> ();
pub type Tcl_CmdObjTraceProc = unsafe extern "C" fn(
    ClientData,
    *mut Tcl_Interp,
    libc::c_int,
    *const libc::c_char,
    Tcl_Command,
    libc::c_int,
    *const *mut Tcl_Obj,
) -> libc::c_int;
pub type Tcl_CmdObjTraceDeleteProc = unsafe extern "C" fn(ClientData) -> ();
pub type Tcl_EncodingConvertProc = unsafe extern "C" fn(
    ClientData,
    *const libc::c_char,
    libc::c_int,
    libc::c_int,
    *mut Tcl_EncodingState,
    *mut libc::c_char,
    libc::c_int,
    *mut libc::c_int,
    *mut libc::c_int,
    *mut libc::c_int,
) -> libc::c_int;
pub type Tcl_EncodingFreeProc = unsafe extern "C" fn(ClientData) -> ();
pub type Tcl_EventCheckProc = unsafe extern "C" fn(ClientData, libc::c_int) -> ();
pub type Tcl_EventDeleteProc = unsafe extern "C" fn(
    *mut Tcl_Event,
    ClientData,
) -> libc::c_int;
pub type Tcl_EventSetupProc = unsafe extern "C" fn(ClientData, libc::c_int) -> ();
pub type Tcl_ExitProc = unsafe extern "C" fn(ClientData) -> ();
pub type Tcl_FileProc = unsafe extern "C" fn(ClientData, libc::c_int) -> ();
pub type Tcl_FreeProc = unsafe extern "C" fn(*mut libc::c_char) -> ();
pub type Tcl_IdleProc = unsafe extern "C" fn(ClientData) -> ();
pub type Tcl_InterpDeleteProc = unsafe extern "C" fn(ClientData, *mut Tcl_Interp) -> ();
pub type Tcl_MathProc = unsafe extern "C" fn(
    ClientData,
    *mut Tcl_Interp,
    *mut Tcl_Value,
    *mut Tcl_Value,
) -> libc::c_int;
pub type Tcl_NamespaceDeleteProc = unsafe extern "C" fn(ClientData) -> ();
pub type Tcl_ObjCmdProc = unsafe extern "C" fn(
    ClientData,
    *mut Tcl_Interp,
    libc::c_int,
    *const *mut Tcl_Obj,
) -> libc::c_int;
pub type Tcl_PackageInitProc = unsafe extern "C" fn(*mut Tcl_Interp) -> libc::c_int;
pub type Tcl_PanicProc = unsafe extern "C" fn(*const libc::c_char, ...) -> ();
pub type Tcl_TcpAcceptProc = unsafe extern "C" fn(
    ClientData,
    Tcl_Channel,
    *mut libc::c_char,
    libc::c_int,
) -> ();
pub type Tcl_TimerProc = unsafe extern "C" fn(ClientData) -> ();
pub type Tcl_VarTraceProc = unsafe extern "C" fn(
    ClientData,
    *mut Tcl_Interp,
    *const libc::c_char,
    *const libc::c_char,
    libc::c_int,
) -> *mut libc::c_char;
pub type Tcl_CommandTraceProc = unsafe extern "C" fn(
    ClientData,
    *mut Tcl_Interp,
    *const libc::c_char,
    *const libc::c_char,
    libc::c_int,
) -> ();
pub type Tcl_CreateFileHandlerProc = unsafe extern "C" fn(
    libc::c_int,
    libc::c_int,
    Option::<Tcl_FileProc>,
    ClientData,
) -> ();
pub type Tcl_DeleteFileHandlerProc = unsafe extern "C" fn(libc::c_int) -> ();
pub type Tcl_AlertNotifierProc = unsafe extern "C" fn(ClientData) -> ();
pub type Tcl_ServiceModeHookProc = unsafe extern "C" fn(libc::c_int) -> ();
pub type Tcl_InitNotifierProc = unsafe extern "C" fn() -> ClientData;
pub type Tcl_FinalizeNotifierProc = unsafe extern "C" fn(ClientData) -> ();
pub type Tcl_MainLoopProc = unsafe extern "C" fn() -> ();
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Tcl_SavedResult {
    pub result: *mut libc::c_char,
    pub freeProc: Option::<Tcl_FreeProc>,
    pub objResultPtr: *mut Tcl_Obj,
    pub appendResult: *mut libc::c_char,
    pub appendAvl: libc::c_int,
    pub appendUsed: libc::c_int,
    pub resultSpace: [libc::c_char; 201],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Tcl_Namespace {
    pub name: *mut libc::c_char,
    pub fullName: *mut libc::c_char,
    pub clientData: ClientData,
    pub deleteProc: Option::<Tcl_NamespaceDeleteProc>,
    pub parentPtr: *mut Tcl_Namespace,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Tcl_CmdInfo {
    pub isNativeObjectProc: libc::c_int,
    pub objProc: Option::<Tcl_ObjCmdProc>,
    pub objClientData: ClientData,
    pub proc_0: Option::<Tcl_CmdProc>,
    pub clientData: ClientData,
    pub deleteProc: Option::<Tcl_CmdDeleteProc>,
    pub deleteData: ClientData,
    pub namespacePtr: *mut Tcl_Namespace,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Tcl_DString {
    pub string: *mut libc::c_char,
    pub length: libc::c_int,
    pub spaceAvl: libc::c_int,
    pub staticSpace: [libc::c_char; 200],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Tcl_HashKeyType {
    pub version: libc::c_int,
    pub flags: libc::c_int,
    pub hashKeyProc: Option::<Tcl_HashKeyProc>,
    pub compareKeysProc: Option::<Tcl_CompareHashKeysProc>,
    pub allocEntryProc: Option::<Tcl_AllocHashEntryProc>,
    pub freeEntryProc: Option::<Tcl_FreeHashEntryProc>,
}
pub type Tcl_FreeHashEntryProc = unsafe extern "C" fn(*mut Tcl_HashEntry) -> ();
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Tcl_HashEntry {
    pub nextPtr: *mut Tcl_HashEntry,
    pub tablePtr: *mut Tcl_HashTable,
    pub hash: *mut libc::c_void,
    pub clientData: ClientData,
    pub key: C2RustUnnamed_2,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_2 {
    pub oneWordValue: *mut libc::c_char,
    pub objPtr: *mut Tcl_Obj,
    pub words: [libc::c_int; 1],
    pub string: [libc::c_char; 1],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Tcl_HashTable {
    pub buckets: *mut *mut Tcl_HashEntry,
    pub staticBuckets: [*mut Tcl_HashEntry; 4],
    pub numBuckets: libc::c_int,
    pub numEntries: libc::c_int,
    pub rebuildSize: libc::c_int,
    pub downShift: libc::c_int,
    pub mask: libc::c_int,
    pub keyType: libc::c_int,
    pub findProc: Option::<
        unsafe extern "C" fn(
            *mut Tcl_HashTable,
            *const libc::c_char,
        ) -> *mut Tcl_HashEntry,
    >,
    pub createProc: Option::<
        unsafe extern "C" fn(
            *mut Tcl_HashTable,
            *const libc::c_char,
            *mut libc::c_int,
        ) -> *mut Tcl_HashEntry,
    >,
    pub typePtr: *const Tcl_HashKeyType,
}
pub type Tcl_AllocHashEntryProc = unsafe extern "C" fn(
    *mut Tcl_HashTable,
    *mut libc::c_void,
) -> *mut Tcl_HashEntry;
pub type Tcl_CompareHashKeysProc = unsafe extern "C" fn(
    *mut libc::c_void,
    *mut Tcl_HashEntry,
) -> libc::c_int;
pub type Tcl_HashKeyProc = unsafe extern "C" fn(
    *mut Tcl_HashTable,
    *mut libc::c_void,
) -> libc::c_uint;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Tcl_HashSearch {
    pub tablePtr: *mut Tcl_HashTable,
    pub nextIndex: libc::c_int,
    pub nextEntryPtr: *mut Tcl_HashEntry,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Tcl_DictSearch {
    pub next: *mut libc::c_void,
    pub epoch: libc::c_int,
    pub dictionaryPtr: Tcl_Dict,
}
pub type Tcl_QueuePosition = libc::c_uint;
pub const TCL_QUEUE_MARK: Tcl_QueuePosition = 2;
pub const TCL_QUEUE_HEAD: Tcl_QueuePosition = 1;
pub const TCL_QUEUE_TAIL: Tcl_QueuePosition = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Tcl_Time {
    pub sec: libc::c_long,
    pub usec: libc::c_long,
}
pub type Tcl_SetTimerProc = unsafe extern "C" fn(*const Tcl_Time) -> ();
pub type Tcl_WaitForEventProc = unsafe extern "C" fn(*const Tcl_Time) -> libc::c_int;
pub type Tcl_GetTimeProc = unsafe extern "C" fn(*mut Tcl_Time, ClientData) -> ();
pub type Tcl_ScaleTimeProc = unsafe extern "C" fn(*mut Tcl_Time, ClientData) -> ();
pub type Tcl_DriverBlockModeProc = unsafe extern "C" fn(
    ClientData,
    libc::c_int,
) -> libc::c_int;
pub type Tcl_DriverCloseProc = unsafe extern "C" fn(
    ClientData,
    *mut Tcl_Interp,
) -> libc::c_int;
pub type Tcl_DriverClose2Proc = unsafe extern "C" fn(
    ClientData,
    *mut Tcl_Interp,
    libc::c_int,
) -> libc::c_int;
pub type Tcl_DriverInputProc = unsafe extern "C" fn(
    ClientData,
    *mut libc::c_char,
    libc::c_int,
    *mut libc::c_int,
) -> libc::c_int;
pub type Tcl_DriverOutputProc = unsafe extern "C" fn(
    ClientData,
    *const libc::c_char,
    libc::c_int,
    *mut libc::c_int,
) -> libc::c_int;
pub type Tcl_DriverSeekProc = unsafe extern "C" fn(
    ClientData,
    libc::c_long,
    libc::c_int,
    *mut libc::c_int,
) -> libc::c_int;
pub type Tcl_DriverSetOptionProc = unsafe extern "C" fn(
    ClientData,
    *mut Tcl_Interp,
    *const libc::c_char,
    *const libc::c_char,
) -> libc::c_int;
pub type Tcl_DriverGetOptionProc = unsafe extern "C" fn(
    ClientData,
    *mut Tcl_Interp,
    *const libc::c_char,
    *mut Tcl_DString,
) -> libc::c_int;
pub type Tcl_DriverWatchProc = unsafe extern "C" fn(ClientData, libc::c_int) -> ();
pub type Tcl_DriverGetHandleProc = unsafe extern "C" fn(
    ClientData,
    libc::c_int,
    *mut ClientData,
) -> libc::c_int;
pub type Tcl_DriverFlushProc = unsafe extern "C" fn(ClientData) -> libc::c_int;
pub type Tcl_DriverHandlerProc = unsafe extern "C" fn(
    ClientData,
    libc::c_int,
) -> libc::c_int;
pub type Tcl_DriverWideSeekProc = unsafe extern "C" fn(
    ClientData,
    Tcl_WideInt,
    libc::c_int,
    *mut libc::c_int,
) -> Tcl_WideInt;
pub type Tcl_DriverThreadActionProc = unsafe extern "C" fn(
    ClientData,
    libc::c_int,
) -> ();
pub type Tcl_DriverTruncateProc = unsafe extern "C" fn(
    ClientData,
    Tcl_WideInt,
) -> libc::c_int;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Tcl_ChannelType {
    pub typeName: *const libc::c_char,
    pub version: Tcl_ChannelTypeVersion,
    pub closeProc: Option::<Tcl_DriverCloseProc>,
    pub inputProc: Option::<Tcl_DriverInputProc>,
    pub outputProc: Option::<Tcl_DriverOutputProc>,
    pub seekProc: Option::<Tcl_DriverSeekProc>,
    pub setOptionProc: Option::<Tcl_DriverSetOptionProc>,
    pub getOptionProc: Option::<Tcl_DriverGetOptionProc>,
    pub watchProc: Option::<Tcl_DriverWatchProc>,
    pub getHandleProc: Option::<Tcl_DriverGetHandleProc>,
    pub close2Proc: Option::<Tcl_DriverClose2Proc>,
    pub blockModeProc: Option::<Tcl_DriverBlockModeProc>,
    pub flushProc: Option::<Tcl_DriverFlushProc>,
    pub handlerProc: Option::<Tcl_DriverHandlerProc>,
    pub wideSeekProc: Option::<Tcl_DriverWideSeekProc>,
    pub threadActionProc: Option::<Tcl_DriverThreadActionProc>,
    pub truncateProc: Option::<Tcl_DriverTruncateProc>,
}
pub type Tcl_PathType = libc::c_uint;
pub const TCL_PATH_VOLUME_RELATIVE: Tcl_PathType = 2;
pub const TCL_PATH_RELATIVE: Tcl_PathType = 1;
pub const TCL_PATH_ABSOLUTE: Tcl_PathType = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Tcl_GlobTypeData {
    pub type_0: libc::c_int,
    pub perm: libc::c_int,
    pub macType: *mut Tcl_Obj,
    pub macCreator: *mut Tcl_Obj,
}
pub type Tcl_FSStatProc = unsafe extern "C" fn(
    *mut Tcl_Obj,
    *mut Tcl_StatBuf,
) -> libc::c_int;
pub type Tcl_FSAccessProc = unsafe extern "C" fn(
    *mut Tcl_Obj,
    libc::c_int,
) -> libc::c_int;
pub type Tcl_FSOpenFileChannelProc = unsafe extern "C" fn(
    *mut Tcl_Interp,
    *mut Tcl_Obj,
    libc::c_int,
    libc::c_int,
) -> Tcl_Channel;
pub type Tcl_FSMatchInDirectoryProc = unsafe extern "C" fn(
    *mut Tcl_Interp,
    *mut Tcl_Obj,
    *mut Tcl_Obj,
    *const libc::c_char,
    *mut Tcl_GlobTypeData,
) -> libc::c_int;
pub type Tcl_FSGetCwdProc = unsafe extern "C" fn(*mut Tcl_Interp) -> *mut Tcl_Obj;
pub type Tcl_FSChdirProc = unsafe extern "C" fn(*mut Tcl_Obj) -> libc::c_int;
pub type Tcl_FSLstatProc = unsafe extern "C" fn(
    *mut Tcl_Obj,
    *mut Tcl_StatBuf,
) -> libc::c_int;
pub type Tcl_FSCreateDirectoryProc = unsafe extern "C" fn(*mut Tcl_Obj) -> libc::c_int;
pub type Tcl_FSDeleteFileProc = unsafe extern "C" fn(*mut Tcl_Obj) -> libc::c_int;
pub type Tcl_FSCopyDirectoryProc = unsafe extern "C" fn(
    *mut Tcl_Obj,
    *mut Tcl_Obj,
    *mut *mut Tcl_Obj,
) -> libc::c_int;
pub type Tcl_FSCopyFileProc = unsafe extern "C" fn(
    *mut Tcl_Obj,
    *mut Tcl_Obj,
) -> libc::c_int;
pub type Tcl_FSRemoveDirectoryProc = unsafe extern "C" fn(
    *mut Tcl_Obj,
    libc::c_int,
    *mut *mut Tcl_Obj,
) -> libc::c_int;
pub type Tcl_FSRenameFileProc = unsafe extern "C" fn(
    *mut Tcl_Obj,
    *mut Tcl_Obj,
) -> libc::c_int;
pub type Tcl_FSUnloadFileProc = unsafe extern "C" fn(Tcl_LoadHandle) -> ();
pub type Tcl_FSListVolumesProc = unsafe extern "C" fn() -> *mut Tcl_Obj;
pub type Tcl_FSUtimeProc = unsafe extern "C" fn(
    *mut Tcl_Obj,
    *mut utimbuf,
) -> libc::c_int;
pub type Tcl_FSNormalizePathProc = unsafe extern "C" fn(
    *mut Tcl_Interp,
    *mut Tcl_Obj,
    libc::c_int,
) -> libc::c_int;
pub type Tcl_FSFileAttrsGetProc = unsafe extern "C" fn(
    *mut Tcl_Interp,
    libc::c_int,
    *mut Tcl_Obj,
    *mut *mut Tcl_Obj,
) -> libc::c_int;
pub type Tcl_FSFileAttrStringsProc = unsafe extern "C" fn(
    *mut Tcl_Obj,
    *mut *mut Tcl_Obj,
) -> *const *const libc::c_char;
pub type Tcl_FSFileAttrsSetProc = unsafe extern "C" fn(
    *mut Tcl_Interp,
    libc::c_int,
    *mut Tcl_Obj,
    *mut Tcl_Obj,
) -> libc::c_int;
pub type Tcl_FSLinkProc = unsafe extern "C" fn(
    *mut Tcl_Obj,
    *mut Tcl_Obj,
    libc::c_int,
) -> *mut Tcl_Obj;
pub type Tcl_FSLoadFileProc = unsafe extern "C" fn(
    *mut Tcl_Interp,
    *mut Tcl_Obj,
    *mut Tcl_LoadHandle,
    *mut Option::<Tcl_FSUnloadFileProc>,
) -> libc::c_int;
pub type Tcl_FSPathInFilesystemProc = unsafe extern "C" fn(
    *mut Tcl_Obj,
    *mut ClientData,
) -> libc::c_int;
pub type Tcl_FSFilesystemPathTypeProc = unsafe extern "C" fn(
    *mut Tcl_Obj,
) -> *mut Tcl_Obj;
pub type Tcl_FSFilesystemSeparatorProc = unsafe extern "C" fn(
    *mut Tcl_Obj,
) -> *mut Tcl_Obj;
pub type Tcl_FSFreeInternalRepProc = unsafe extern "C" fn(ClientData) -> ();
pub type Tcl_FSDupInternalRepProc = unsafe extern "C" fn(ClientData) -> ClientData;
pub type Tcl_FSInternalToNormalizedProc = unsafe extern "C" fn(
    ClientData,
) -> *mut Tcl_Obj;
pub type Tcl_FSCreateInternalRepProc = unsafe extern "C" fn(*mut Tcl_Obj) -> ClientData;
pub type Tcl_FSVersion = *mut Tcl_FSVersion_;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Tcl_Filesystem {
    pub typeName: *const libc::c_char,
    pub structureLength: libc::c_int,
    pub version: Tcl_FSVersion,
    pub pathInFilesystemProc: Option::<Tcl_FSPathInFilesystemProc>,
    pub dupInternalRepProc: Option::<Tcl_FSDupInternalRepProc>,
    pub freeInternalRepProc: Option::<Tcl_FSFreeInternalRepProc>,
    pub internalToNormalizedProc: Option::<Tcl_FSInternalToNormalizedProc>,
    pub createInternalRepProc: Option::<Tcl_FSCreateInternalRepProc>,
    pub normalizePathProc: Option::<Tcl_FSNormalizePathProc>,
    pub filesystemPathTypeProc: Option::<Tcl_FSFilesystemPathTypeProc>,
    pub filesystemSeparatorProc: Option::<Tcl_FSFilesystemSeparatorProc>,
    pub statProc: Option::<Tcl_FSStatProc>,
    pub accessProc: Option::<Tcl_FSAccessProc>,
    pub openFileChannelProc: Option::<Tcl_FSOpenFileChannelProc>,
    pub matchInDirectoryProc: Option::<Tcl_FSMatchInDirectoryProc>,
    pub utimeProc: Option::<Tcl_FSUtimeProc>,
    pub linkProc: Option::<Tcl_FSLinkProc>,
    pub listVolumesProc: Option::<Tcl_FSListVolumesProc>,
    pub fileAttrStringsProc: Option::<Tcl_FSFileAttrStringsProc>,
    pub fileAttrsGetProc: Option::<Tcl_FSFileAttrsGetProc>,
    pub fileAttrsSetProc: Option::<Tcl_FSFileAttrsSetProc>,
    pub createDirectoryProc: Option::<Tcl_FSCreateDirectoryProc>,
    pub removeDirectoryProc: Option::<Tcl_FSRemoveDirectoryProc>,
    pub deleteFileProc: Option::<Tcl_FSDeleteFileProc>,
    pub copyFileProc: Option::<Tcl_FSCopyFileProc>,
    pub renameFileProc: Option::<Tcl_FSRenameFileProc>,
    pub copyDirectoryProc: Option::<Tcl_FSCopyDirectoryProc>,
    pub lstatProc: Option::<Tcl_FSLstatProc>,
    pub loadFileProc: Option::<Tcl_FSLoadFileProc>,
    pub getCwdProc: Option::<Tcl_FSGetCwdProc>,
    pub chdirProc: Option::<Tcl_FSChdirProc>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Tcl_NotifierProcs {
    pub setTimerProc: Option::<Tcl_SetTimerProc>,
    pub waitForEventProc: Option::<Tcl_WaitForEventProc>,
    pub createFileHandlerProc: Option::<Tcl_CreateFileHandlerProc>,
    pub deleteFileHandlerProc: Option::<Tcl_DeleteFileHandlerProc>,
    pub initNotifierProc: Option::<Tcl_InitNotifierProc>,
    pub finalizeNotifierProc: Option::<Tcl_FinalizeNotifierProc>,
    pub alertNotifierProc: Option::<Tcl_AlertNotifierProc>,
    pub serviceModeHookProc: Option::<Tcl_ServiceModeHookProc>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Tcl_Token {
    pub type_0: libc::c_int,
    pub start: *const libc::c_char,
    pub size: libc::c_int,
    pub numComponents: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Tcl_Parse {
    pub commentStart: *const libc::c_char,
    pub commentSize: libc::c_int,
    pub commandStart: *const libc::c_char,
    pub commandSize: libc::c_int,
    pub numWords: libc::c_int,
    pub tokenPtr: *mut Tcl_Token,
    pub numTokens: libc::c_int,
    pub tokensAvailable: libc::c_int,
    pub errorType: libc::c_int,
    pub string: *const libc::c_char,
    pub end: *const libc::c_char,
    pub interp: *mut Tcl_Interp,
    pub term: *const libc::c_char,
    pub incomplete: libc::c_int,
    pub staticTokens: [Tcl_Token; 20],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Tcl_EncodingType {
    pub encodingName: *const libc::c_char,
    pub toUtfProc: Option::<Tcl_EncodingConvertProc>,
    pub fromUtfProc: Option::<Tcl_EncodingConvertProc>,
    pub freeProc: Option::<Tcl_EncodingFreeProc>,
    pub clientData: ClientData,
    pub nullSize: libc::c_int,
}
pub type Tcl_UniChar = libc::c_ushort;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Tcl_Config {
    pub key: *const libc::c_char,
    pub value: *const libc::c_char,
}
pub type Tcl_LimitHandlerProc = unsafe extern "C" fn(ClientData, *mut Tcl_Interp) -> ();
pub type Tcl_LimitHandlerDeleteProc = unsafe extern "C" fn(ClientData) -> ();
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Tcl_ArgvInfo {
    pub type_0: libc::c_int,
    pub keyStr: *const libc::c_char,
    pub srcPtr: *mut libc::c_void,
    pub dstPtr: *mut libc::c_void,
    pub helpStr: *const libc::c_char,
    pub clientData: ClientData,
}
pub type Tcl_NRPostProc = unsafe extern "C" fn(
    *mut ClientData,
    *mut Tcl_Interp,
    libc::c_int,
) -> libc::c_int;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TclStubHooks {
    pub tclPlatStubs: *const TclPlatStubs,
    pub tclIntStubs: *const TclIntStubs,
    pub tclIntPlatStubs: *const TclIntPlatStubs,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TclPlatStubs {
    pub magic: libc::c_int,
    pub hooks: *mut libc::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TclStubs {
    pub magic: libc::c_int,
    pub hooks: *const TclStubHooks,
    pub tcl_PkgProvideEx: Option::<
        unsafe extern "C" fn(
            *mut Tcl_Interp,
            *const libc::c_char,
            *const libc::c_char,
            *const libc::c_void,
        ) -> libc::c_int,
    >,
    pub tcl_PkgRequireEx: Option::<
        unsafe extern "C" fn(
            *mut Tcl_Interp,
            *const libc::c_char,
            *const libc::c_char,
            libc::c_int,
            *mut libc::c_void,
        ) -> *const libc::c_char,
    >,
    pub tcl_Panic: Option::<unsafe extern "C" fn(*const libc::c_char, ...) -> ()>,
    pub tcl_Alloc: Option::<unsafe extern "C" fn(libc::c_uint) -> *mut libc::c_char>,
    pub tcl_Free: Option::<unsafe extern "C" fn(*mut libc::c_char) -> ()>,
    pub tcl_Realloc: Option::<
        unsafe extern "C" fn(*mut libc::c_char, libc::c_uint) -> *mut libc::c_char,
    >,
    pub tcl_DbCkalloc: Option::<
        unsafe extern "C" fn(
            libc::c_uint,
            *const libc::c_char,
            libc::c_int,
        ) -> *mut libc::c_char,
    >,
    pub tcl_DbCkfree: Option::<
        unsafe extern "C" fn(*mut libc::c_char, *const libc::c_char, libc::c_int) -> (),
    >,
    pub tcl_DbCkrealloc: Option::<
        unsafe extern "C" fn(
            *mut libc::c_char,
            libc::c_uint,
            *const libc::c_char,
            libc::c_int,
        ) -> *mut libc::c_char,
    >,
    pub tcl_CreateFileHandler: Option::<
        unsafe extern "C" fn(
            libc::c_int,
            libc::c_int,
            Option::<Tcl_FileProc>,
            ClientData,
        ) -> (),
    >,
    pub tcl_DeleteFileHandler: Option::<unsafe extern "C" fn(libc::c_int) -> ()>,
    pub tcl_SetTimer: Option::<unsafe extern "C" fn(*const Tcl_Time) -> ()>,
    pub tcl_Sleep: Option::<unsafe extern "C" fn(libc::c_int) -> ()>,
    pub tcl_WaitForEvent: Option::<unsafe extern "C" fn(*const Tcl_Time) -> libc::c_int>,
    pub tcl_AppendAllObjTypes: Option::<
        unsafe extern "C" fn(*mut Tcl_Interp, *mut Tcl_Obj) -> libc::c_int,
    >,
    pub tcl_AppendStringsToObj: Option::<unsafe extern "C" fn(*mut Tcl_Obj, ...) -> ()>,
    pub tcl_AppendToObj: Option::<
        unsafe extern "C" fn(*mut Tcl_Obj, *const libc::c_char, libc::c_int) -> (),
    >,
    pub tcl_ConcatObj: Option::<
        unsafe extern "C" fn(libc::c_int, *const *mut Tcl_Obj) -> *mut Tcl_Obj,
    >,
    pub tcl_ConvertToType: Option::<
        unsafe extern "C" fn(
            *mut Tcl_Interp,
            *mut Tcl_Obj,
            *const Tcl_ObjType,
        ) -> libc::c_int,
    >,
    pub tcl_DbDecrRefCount: Option::<
        unsafe extern "C" fn(*mut Tcl_Obj, *const libc::c_char, libc::c_int) -> (),
    >,
    pub tcl_DbIncrRefCount: Option::<
        unsafe extern "C" fn(*mut Tcl_Obj, *const libc::c_char, libc::c_int) -> (),
    >,
    pub tcl_DbIsShared: Option::<
        unsafe extern "C" fn(
            *mut Tcl_Obj,
            *const libc::c_char,
            libc::c_int,
        ) -> libc::c_int,
    >,
    pub tcl_DbNewBooleanObj: Option::<
        unsafe extern "C" fn(
            libc::c_int,
            *const libc::c_char,
            libc::c_int,
        ) -> *mut Tcl_Obj,
    >,
    pub tcl_DbNewByteArrayObj: Option::<
        unsafe extern "C" fn(
            *const libc::c_uchar,
            libc::c_int,
            *const libc::c_char,
            libc::c_int,
        ) -> *mut Tcl_Obj,
    >,
    pub tcl_DbNewDoubleObj: Option::<
        unsafe extern "C" fn(
            libc::c_double,
            *const libc::c_char,
            libc::c_int,
        ) -> *mut Tcl_Obj,
    >,
    pub tcl_DbNewListObj: Option::<
        unsafe extern "C" fn(
            libc::c_int,
            *const *mut Tcl_Obj,
            *const libc::c_char,
            libc::c_int,
        ) -> *mut Tcl_Obj,
    >,
    pub tcl_DbNewLongObj: Option::<
        unsafe extern "C" fn(
            libc::c_long,
            *const libc::c_char,
            libc::c_int,
        ) -> *mut Tcl_Obj,
    >,
    pub tcl_DbNewObj: Option::<
        unsafe extern "C" fn(*const libc::c_char, libc::c_int) -> *mut Tcl_Obj,
    >,
    pub tcl_DbNewStringObj: Option::<
        unsafe extern "C" fn(
            *const libc::c_char,
            libc::c_int,
            *const libc::c_char,
            libc::c_int,
        ) -> *mut Tcl_Obj,
    >,
    pub tcl_DuplicateObj: Option::<unsafe extern "C" fn(*mut Tcl_Obj) -> *mut Tcl_Obj>,
    pub tclFreeObj: Option::<unsafe extern "C" fn(*mut Tcl_Obj) -> ()>,
    pub tcl_GetBoolean: Option::<
        unsafe extern "C" fn(
            *mut Tcl_Interp,
            *const libc::c_char,
            *mut libc::c_int,
        ) -> libc::c_int,
    >,
    pub tcl_GetBooleanFromObj: Option::<
        unsafe extern "C" fn(
            *mut Tcl_Interp,
            *mut Tcl_Obj,
            *mut libc::c_int,
        ) -> libc::c_int,
    >,
    pub tcl_GetByteArrayFromObj: Option::<
        unsafe extern "C" fn(*mut Tcl_Obj, *mut libc::c_int) -> *mut libc::c_uchar,
    >,
    pub tcl_GetDouble: Option::<
        unsafe extern "C" fn(
            *mut Tcl_Interp,
            *const libc::c_char,
            *mut libc::c_double,
        ) -> libc::c_int,
    >,
    pub tcl_GetDoubleFromObj: Option::<
        unsafe extern "C" fn(
            *mut Tcl_Interp,
            *mut Tcl_Obj,
            *mut libc::c_double,
        ) -> libc::c_int,
    >,
    pub tcl_GetIndexFromObj: Option::<
        unsafe extern "C" fn(
            *mut Tcl_Interp,
            *mut Tcl_Obj,
            *const *const libc::c_char,
            *const libc::c_char,
            libc::c_int,
            *mut libc::c_int,
        ) -> libc::c_int,
    >,
    pub tcl_GetInt: Option::<
        unsafe extern "C" fn(
            *mut Tcl_Interp,
            *const libc::c_char,
            *mut libc::c_int,
        ) -> libc::c_int,
    >,
    pub tcl_GetIntFromObj: Option::<
        unsafe extern "C" fn(
            *mut Tcl_Interp,
            *mut Tcl_Obj,
            *mut libc::c_int,
        ) -> libc::c_int,
    >,
    pub tcl_GetLongFromObj: Option::<
        unsafe extern "C" fn(
            *mut Tcl_Interp,
            *mut Tcl_Obj,
            *mut libc::c_long,
        ) -> libc::c_int,
    >,
    pub tcl_GetObjType: Option::<
        unsafe extern "C" fn(*const libc::c_char) -> *const Tcl_ObjType,
    >,
    pub tcl_GetStringFromObj: Option::<
        unsafe extern "C" fn(*mut Tcl_Obj, *mut libc::c_int) -> *mut libc::c_char,
    >,
    pub tcl_InvalidateStringRep: Option::<unsafe extern "C" fn(*mut Tcl_Obj) -> ()>,
    pub tcl_ListObjAppendList: Option::<
        unsafe extern "C" fn(*mut Tcl_Interp, *mut Tcl_Obj, *mut Tcl_Obj) -> libc::c_int,
    >,
    pub tcl_ListObjAppendElement: Option::<
        unsafe extern "C" fn(*mut Tcl_Interp, *mut Tcl_Obj, *mut Tcl_Obj) -> libc::c_int,
    >,
    pub tcl_ListObjGetElements: Option::<
        unsafe extern "C" fn(
            *mut Tcl_Interp,
            *mut Tcl_Obj,
            *mut libc::c_int,
            *mut *mut *mut Tcl_Obj,
        ) -> libc::c_int,
    >,
    pub tcl_ListObjIndex: Option::<
        unsafe extern "C" fn(
            *mut Tcl_Interp,
            *mut Tcl_Obj,
            libc::c_int,
            *mut *mut Tcl_Obj,
        ) -> libc::c_int,
    >,
    pub tcl_ListObjLength: Option::<
        unsafe extern "C" fn(
            *mut Tcl_Interp,
            *mut Tcl_Obj,
            *mut libc::c_int,
        ) -> libc::c_int,
    >,
    pub tcl_ListObjReplace: Option::<
        unsafe extern "C" fn(
            *mut Tcl_Interp,
            *mut Tcl_Obj,
            libc::c_int,
            libc::c_int,
            libc::c_int,
            *const *mut Tcl_Obj,
        ) -> libc::c_int,
    >,
    pub tcl_NewBooleanObj: Option::<unsafe extern "C" fn(libc::c_int) -> *mut Tcl_Obj>,
    pub tcl_NewByteArrayObj: Option::<
        unsafe extern "C" fn(*const libc::c_uchar, libc::c_int) -> *mut Tcl_Obj,
    >,
    pub tcl_NewDoubleObj: Option::<unsafe extern "C" fn(libc::c_double) -> *mut Tcl_Obj>,
    pub tcl_NewIntObj: Option::<unsafe extern "C" fn(libc::c_int) -> *mut Tcl_Obj>,
    pub tcl_NewListObj: Option::<
        unsafe extern "C" fn(libc::c_int, *const *mut Tcl_Obj) -> *mut Tcl_Obj,
    >,
    pub tcl_NewLongObj: Option::<unsafe extern "C" fn(libc::c_long) -> *mut Tcl_Obj>,
    pub tcl_NewObj: Option::<unsafe extern "C" fn() -> *mut Tcl_Obj>,
    pub tcl_NewStringObj: Option::<
        unsafe extern "C" fn(*const libc::c_char, libc::c_int) -> *mut Tcl_Obj,
    >,
    pub tcl_SetBooleanObj: Option::<
        unsafe extern "C" fn(*mut Tcl_Obj, libc::c_int) -> (),
    >,
    pub tcl_SetByteArrayLength: Option::<
        unsafe extern "C" fn(*mut Tcl_Obj, libc::c_int) -> *mut libc::c_uchar,
    >,
    pub tcl_SetByteArrayObj: Option::<
        unsafe extern "C" fn(*mut Tcl_Obj, *const libc::c_uchar, libc::c_int) -> (),
    >,
    pub tcl_SetDoubleObj: Option::<
        unsafe extern "C" fn(*mut Tcl_Obj, libc::c_double) -> (),
    >,
    pub tcl_SetIntObj: Option::<unsafe extern "C" fn(*mut Tcl_Obj, libc::c_int) -> ()>,
    pub tcl_SetListObj: Option::<
        unsafe extern "C" fn(*mut Tcl_Obj, libc::c_int, *const *mut Tcl_Obj) -> (),
    >,
    pub tcl_SetLongObj: Option::<unsafe extern "C" fn(*mut Tcl_Obj, libc::c_long) -> ()>,
    pub tcl_SetObjLength: Option::<
        unsafe extern "C" fn(*mut Tcl_Obj, libc::c_int) -> (),
    >,
    pub tcl_SetStringObj: Option::<
        unsafe extern "C" fn(*mut Tcl_Obj, *const libc::c_char, libc::c_int) -> (),
    >,
    pub tcl_AddErrorInfo: Option::<
        unsafe extern "C" fn(*mut Tcl_Interp, *const libc::c_char) -> (),
    >,
    pub tcl_AddObjErrorInfo: Option::<
        unsafe extern "C" fn(*mut Tcl_Interp, *const libc::c_char, libc::c_int) -> (),
    >,
    pub tcl_AllowExceptions: Option::<unsafe extern "C" fn(*mut Tcl_Interp) -> ()>,
    pub tcl_AppendElement: Option::<
        unsafe extern "C" fn(*mut Tcl_Interp, *const libc::c_char) -> (),
    >,
    pub tcl_AppendResult: Option::<unsafe extern "C" fn(*mut Tcl_Interp, ...) -> ()>,
    pub tcl_AsyncCreate: Option::<
        unsafe extern "C" fn(Option::<Tcl_AsyncProc>, ClientData) -> Tcl_AsyncHandler,
    >,
    pub tcl_AsyncDelete: Option::<unsafe extern "C" fn(Tcl_AsyncHandler) -> ()>,
    pub tcl_AsyncInvoke: Option::<
        unsafe extern "C" fn(*mut Tcl_Interp, libc::c_int) -> libc::c_int,
    >,
    pub tcl_AsyncMark: Option::<unsafe extern "C" fn(Tcl_AsyncHandler) -> ()>,
    pub tcl_AsyncReady: Option::<unsafe extern "C" fn() -> libc::c_int>,
    pub tcl_BackgroundError: Option::<unsafe extern "C" fn(*mut Tcl_Interp) -> ()>,
    pub tcl_Backslash: Option::<
        unsafe extern "C" fn(*const libc::c_char, *mut libc::c_int) -> libc::c_char,
    >,
    pub tcl_BadChannelOption: Option::<
        unsafe extern "C" fn(
            *mut Tcl_Interp,
            *const libc::c_char,
            *const libc::c_char,
        ) -> libc::c_int,
    >,
    pub tcl_CallWhenDeleted: Option::<
        unsafe extern "C" fn(
            *mut Tcl_Interp,
            Option::<Tcl_InterpDeleteProc>,
            ClientData,
        ) -> (),
    >,
    pub tcl_CancelIdleCall: Option::<
        unsafe extern "C" fn(Option::<Tcl_IdleProc>, ClientData) -> (),
    >,
    pub tcl_Close: Option::<
        unsafe extern "C" fn(*mut Tcl_Interp, Tcl_Channel) -> libc::c_int,
    >,
    pub tcl_CommandComplete: Option::<
        unsafe extern "C" fn(*const libc::c_char) -> libc::c_int,
    >,
    pub tcl_Concat: Option::<
        unsafe extern "C" fn(
            libc::c_int,
            *const *const libc::c_char,
        ) -> *mut libc::c_char,
    >,
    pub tcl_ConvertElement: Option::<
        unsafe extern "C" fn(
            *const libc::c_char,
            *mut libc::c_char,
            libc::c_int,
        ) -> libc::c_int,
    >,
    pub tcl_ConvertCountedElement: Option::<
        unsafe extern "C" fn(
            *const libc::c_char,
            libc::c_int,
            *mut libc::c_char,
            libc::c_int,
        ) -> libc::c_int,
    >,
    pub tcl_CreateAlias: Option::<
        unsafe extern "C" fn(
            *mut Tcl_Interp,
            *const libc::c_char,
            *mut Tcl_Interp,
            *const libc::c_char,
            libc::c_int,
            *const *const libc::c_char,
        ) -> libc::c_int,
    >,
    pub tcl_CreateAliasObj: Option::<
        unsafe extern "C" fn(
            *mut Tcl_Interp,
            *const libc::c_char,
            *mut Tcl_Interp,
            *const libc::c_char,
            libc::c_int,
            *const *mut Tcl_Obj,
        ) -> libc::c_int,
    >,
    pub tcl_CreateChannel: Option::<
        unsafe extern "C" fn(
            *const Tcl_ChannelType,
            *const libc::c_char,
            ClientData,
            libc::c_int,
        ) -> Tcl_Channel,
    >,
    pub tcl_CreateChannelHandler: Option::<
        unsafe extern "C" fn(
            Tcl_Channel,
            libc::c_int,
            Option::<Tcl_ChannelProc>,
            ClientData,
        ) -> (),
    >,
    pub tcl_CreateCloseHandler: Option::<
        unsafe extern "C" fn(Tcl_Channel, Option::<Tcl_CloseProc>, ClientData) -> (),
    >,
    pub tcl_CreateCommand: Option::<
        unsafe extern "C" fn(
            *mut Tcl_Interp,
            *const libc::c_char,
            Option::<Tcl_CmdProc>,
            ClientData,
            Option::<Tcl_CmdDeleteProc>,
        ) -> Tcl_Command,
    >,
    pub tcl_CreateEventSource: Option::<
        unsafe extern "C" fn(
            Option::<Tcl_EventSetupProc>,
            Option::<Tcl_EventCheckProc>,
            ClientData,
        ) -> (),
    >,
    pub tcl_CreateExitHandler: Option::<
        unsafe extern "C" fn(Option::<Tcl_ExitProc>, ClientData) -> (),
    >,
    pub tcl_CreateInterp: Option::<unsafe extern "C" fn() -> *mut Tcl_Interp>,
    pub tcl_CreateMathFunc: Option::<
        unsafe extern "C" fn(
            *mut Tcl_Interp,
            *const libc::c_char,
            libc::c_int,
            *mut Tcl_ValueType,
            Option::<Tcl_MathProc>,
            ClientData,
        ) -> (),
    >,
    pub tcl_CreateObjCommand: Option::<
        unsafe extern "C" fn(
            *mut Tcl_Interp,
            *const libc::c_char,
            Option::<Tcl_ObjCmdProc>,
            ClientData,
            Option::<Tcl_CmdDeleteProc>,
        ) -> Tcl_Command,
    >,
    pub tcl_CreateSlave: Option::<
        unsafe extern "C" fn(
            *mut Tcl_Interp,
            *const libc::c_char,
            libc::c_int,
        ) -> *mut Tcl_Interp,
    >,
    pub tcl_CreateTimerHandler: Option::<
        unsafe extern "C" fn(
            libc::c_int,
            Option::<Tcl_TimerProc>,
            ClientData,
        ) -> Tcl_TimerToken,
    >,
    pub tcl_CreateTrace: Option::<
        unsafe extern "C" fn(
            *mut Tcl_Interp,
            libc::c_int,
            Option::<Tcl_CmdTraceProc>,
            ClientData,
        ) -> Tcl_Trace,
    >,
    pub tcl_DeleteAssocData: Option::<
        unsafe extern "C" fn(*mut Tcl_Interp, *const libc::c_char) -> (),
    >,
    pub tcl_DeleteChannelHandler: Option::<
        unsafe extern "C" fn(Tcl_Channel, Option::<Tcl_ChannelProc>, ClientData) -> (),
    >,
    pub tcl_DeleteCloseHandler: Option::<
        unsafe extern "C" fn(Tcl_Channel, Option::<Tcl_CloseProc>, ClientData) -> (),
    >,
    pub tcl_DeleteCommand: Option::<
        unsafe extern "C" fn(*mut Tcl_Interp, *const libc::c_char) -> libc::c_int,
    >,
    pub tcl_DeleteCommandFromToken: Option::<
        unsafe extern "C" fn(*mut Tcl_Interp, Tcl_Command) -> libc::c_int,
    >,
    pub tcl_DeleteEvents: Option::<
        unsafe extern "C" fn(Option::<Tcl_EventDeleteProc>, ClientData) -> (),
    >,
    pub tcl_DeleteEventSource: Option::<
        unsafe extern "C" fn(
            Option::<Tcl_EventSetupProc>,
            Option::<Tcl_EventCheckProc>,
            ClientData,
        ) -> (),
    >,
    pub tcl_DeleteExitHandler: Option::<
        unsafe extern "C" fn(Option::<Tcl_ExitProc>, ClientData) -> (),
    >,
    pub tcl_DeleteHashEntry: Option::<unsafe extern "C" fn(*mut Tcl_HashEntry) -> ()>,
    pub tcl_DeleteHashTable: Option::<unsafe extern "C" fn(*mut Tcl_HashTable) -> ()>,
    pub tcl_DeleteInterp: Option::<unsafe extern "C" fn(*mut Tcl_Interp) -> ()>,
    pub tcl_DetachPids: Option::<unsafe extern "C" fn(libc::c_int, *mut Tcl_Pid) -> ()>,
    pub tcl_DeleteTimerHandler: Option::<unsafe extern "C" fn(Tcl_TimerToken) -> ()>,
    pub tcl_DeleteTrace: Option::<
        unsafe extern "C" fn(*mut Tcl_Interp, Tcl_Trace) -> (),
    >,
    pub tcl_DontCallWhenDeleted: Option::<
        unsafe extern "C" fn(
            *mut Tcl_Interp,
            Option::<Tcl_InterpDeleteProc>,
            ClientData,
        ) -> (),
    >,
    pub tcl_DoOneEvent: Option::<unsafe extern "C" fn(libc::c_int) -> libc::c_int>,
    pub tcl_DoWhenIdle: Option::<
        unsafe extern "C" fn(Option::<Tcl_IdleProc>, ClientData) -> (),
    >,
    pub tcl_DStringAppend: Option::<
        unsafe extern "C" fn(
            *mut Tcl_DString,
            *const libc::c_char,
            libc::c_int,
        ) -> *mut libc::c_char,
    >,
    pub tcl_DStringAppendElement: Option::<
        unsafe extern "C" fn(*mut Tcl_DString, *const libc::c_char) -> *mut libc::c_char,
    >,
    pub tcl_DStringEndSublist: Option::<unsafe extern "C" fn(*mut Tcl_DString) -> ()>,
    pub tcl_DStringFree: Option::<unsafe extern "C" fn(*mut Tcl_DString) -> ()>,
    pub tcl_DStringGetResult: Option::<
        unsafe extern "C" fn(*mut Tcl_Interp, *mut Tcl_DString) -> (),
    >,
    pub tcl_DStringInit: Option::<unsafe extern "C" fn(*mut Tcl_DString) -> ()>,
    pub tcl_DStringResult: Option::<
        unsafe extern "C" fn(*mut Tcl_Interp, *mut Tcl_DString) -> (),
    >,
    pub tcl_DStringSetLength: Option::<
        unsafe extern "C" fn(*mut Tcl_DString, libc::c_int) -> (),
    >,
    pub tcl_DStringStartSublist: Option::<unsafe extern "C" fn(*mut Tcl_DString) -> ()>,
    pub tcl_Eof: Option::<unsafe extern "C" fn(Tcl_Channel) -> libc::c_int>,
    pub tcl_ErrnoId: Option::<unsafe extern "C" fn() -> *const libc::c_char>,
    pub tcl_ErrnoMsg: Option::<unsafe extern "C" fn(libc::c_int) -> *const libc::c_char>,
    pub tcl_Eval: Option::<
        unsafe extern "C" fn(*mut Tcl_Interp, *const libc::c_char) -> libc::c_int,
    >,
    pub tcl_EvalFile: Option::<
        unsafe extern "C" fn(*mut Tcl_Interp, *const libc::c_char) -> libc::c_int,
    >,
    pub tcl_EvalObj: Option::<
        unsafe extern "C" fn(*mut Tcl_Interp, *mut Tcl_Obj) -> libc::c_int,
    >,
    pub tcl_EventuallyFree: Option::<
        unsafe extern "C" fn(ClientData, Option::<Tcl_FreeProc>) -> (),
    >,
    pub tcl_Exit: Option::<unsafe extern "C" fn(libc::c_int) -> ()>,
    pub tcl_ExposeCommand: Option::<
        unsafe extern "C" fn(
            *mut Tcl_Interp,
            *const libc::c_char,
            *const libc::c_char,
        ) -> libc::c_int,
    >,
    pub tcl_ExprBoolean: Option::<
        unsafe extern "C" fn(
            *mut Tcl_Interp,
            *const libc::c_char,
            *mut libc::c_int,
        ) -> libc::c_int,
    >,
    pub tcl_ExprBooleanObj: Option::<
        unsafe extern "C" fn(
            *mut Tcl_Interp,
            *mut Tcl_Obj,
            *mut libc::c_int,
        ) -> libc::c_int,
    >,
    pub tcl_ExprDouble: Option::<
        unsafe extern "C" fn(
            *mut Tcl_Interp,
            *const libc::c_char,
            *mut libc::c_double,
        ) -> libc::c_int,
    >,
    pub tcl_ExprDoubleObj: Option::<
        unsafe extern "C" fn(
            *mut Tcl_Interp,
            *mut Tcl_Obj,
            *mut libc::c_double,
        ) -> libc::c_int,
    >,
    pub tcl_ExprLong: Option::<
        unsafe extern "C" fn(
            *mut Tcl_Interp,
            *const libc::c_char,
            *mut libc::c_long,
        ) -> libc::c_int,
    >,
    pub tcl_ExprLongObj: Option::<
        unsafe extern "C" fn(
            *mut Tcl_Interp,
            *mut Tcl_Obj,
            *mut libc::c_long,
        ) -> libc::c_int,
    >,
    pub tcl_ExprObj: Option::<
        unsafe extern "C" fn(
            *mut Tcl_Interp,
            *mut Tcl_Obj,
            *mut *mut Tcl_Obj,
        ) -> libc::c_int,
    >,
    pub tcl_ExprString: Option::<
        unsafe extern "C" fn(*mut Tcl_Interp, *const libc::c_char) -> libc::c_int,
    >,
    pub tcl_Finalize: Option::<unsafe extern "C" fn() -> ()>,
    pub tcl_FindExecutable: Option::<unsafe extern "C" fn(*const libc::c_char) -> ()>,
    pub tcl_FirstHashEntry: Option::<
        unsafe extern "C" fn(
            *mut Tcl_HashTable,
            *mut Tcl_HashSearch,
        ) -> *mut Tcl_HashEntry,
    >,
    pub tcl_Flush: Option::<unsafe extern "C" fn(Tcl_Channel) -> libc::c_int>,
    pub tcl_FreeResult: Option::<unsafe extern "C" fn(*mut Tcl_Interp) -> ()>,
    pub tcl_GetAlias: Option::<
        unsafe extern "C" fn(
            *mut Tcl_Interp,
            *const libc::c_char,
            *mut *mut Tcl_Interp,
            *mut *const libc::c_char,
            *mut libc::c_int,
            *mut *mut *const libc::c_char,
        ) -> libc::c_int,
    >,
    pub tcl_GetAliasObj: Option::<
        unsafe extern "C" fn(
            *mut Tcl_Interp,
            *const libc::c_char,
            *mut *mut Tcl_Interp,
            *mut *const libc::c_char,
            *mut libc::c_int,
            *mut *mut *mut Tcl_Obj,
        ) -> libc::c_int,
    >,
    pub tcl_GetAssocData: Option::<
        unsafe extern "C" fn(
            *mut Tcl_Interp,
            *const libc::c_char,
            *mut Option::<Tcl_InterpDeleteProc>,
        ) -> ClientData,
    >,
    pub tcl_GetChannel: Option::<
        unsafe extern "C" fn(
            *mut Tcl_Interp,
            *const libc::c_char,
            *mut libc::c_int,
        ) -> Tcl_Channel,
    >,
    pub tcl_GetChannelBufferSize: Option::<
        unsafe extern "C" fn(Tcl_Channel) -> libc::c_int,
    >,
    pub tcl_GetChannelHandle: Option::<
        unsafe extern "C" fn(Tcl_Channel, libc::c_int, *mut ClientData) -> libc::c_int,
    >,
    pub tcl_GetChannelInstanceData: Option::<
        unsafe extern "C" fn(Tcl_Channel) -> ClientData,
    >,
    pub tcl_GetChannelMode: Option::<unsafe extern "C" fn(Tcl_Channel) -> libc::c_int>,
    pub tcl_GetChannelName: Option::<
        unsafe extern "C" fn(Tcl_Channel) -> *const libc::c_char,
    >,
    pub tcl_GetChannelOption: Option::<
        unsafe extern "C" fn(
            *mut Tcl_Interp,
            Tcl_Channel,
            *const libc::c_char,
            *mut Tcl_DString,
        ) -> libc::c_int,
    >,
    pub tcl_GetChannelType: Option::<
        unsafe extern "C" fn(Tcl_Channel) -> *const Tcl_ChannelType,
    >,
    pub tcl_GetCommandInfo: Option::<
        unsafe extern "C" fn(
            *mut Tcl_Interp,
            *const libc::c_char,
            *mut Tcl_CmdInfo,
        ) -> libc::c_int,
    >,
    pub tcl_GetCommandName: Option::<
        unsafe extern "C" fn(*mut Tcl_Interp, Tcl_Command) -> *const libc::c_char,
    >,
    pub tcl_GetErrno: Option::<unsafe extern "C" fn() -> libc::c_int>,
    pub tcl_GetHostName: Option::<unsafe extern "C" fn() -> *const libc::c_char>,
    pub tcl_GetInterpPath: Option::<
        unsafe extern "C" fn(*mut Tcl_Interp, *mut Tcl_Interp) -> libc::c_int,
    >,
    pub tcl_GetMaster: Option::<
        unsafe extern "C" fn(*mut Tcl_Interp) -> *mut Tcl_Interp,
    >,
    pub tcl_GetNameOfExecutable: Option::<unsafe extern "C" fn() -> *const libc::c_char>,
    pub tcl_GetObjResult: Option::<
        unsafe extern "C" fn(*mut Tcl_Interp) -> *mut Tcl_Obj,
    >,
    pub tcl_GetOpenFile: Option::<
        unsafe extern "C" fn(
            *mut Tcl_Interp,
            *const libc::c_char,
            libc::c_int,
            libc::c_int,
            *mut ClientData,
        ) -> libc::c_int,
    >,
    pub tcl_GetPathType: Option::<
        unsafe extern "C" fn(*const libc::c_char) -> Tcl_PathType,
    >,
    pub tcl_Gets: Option::<
        unsafe extern "C" fn(Tcl_Channel, *mut Tcl_DString) -> libc::c_int,
    >,
    pub tcl_GetsObj: Option::<
        unsafe extern "C" fn(Tcl_Channel, *mut Tcl_Obj) -> libc::c_int,
    >,
    pub tcl_GetServiceMode: Option::<unsafe extern "C" fn() -> libc::c_int>,
    pub tcl_GetSlave: Option::<
        unsafe extern "C" fn(*mut Tcl_Interp, *const libc::c_char) -> *mut Tcl_Interp,
    >,
    pub tcl_GetStdChannel: Option::<unsafe extern "C" fn(libc::c_int) -> Tcl_Channel>,
    pub tcl_GetStringResult: Option::<
        unsafe extern "C" fn(*mut Tcl_Interp) -> *const libc::c_char,
    >,
    pub tcl_GetVar: Option::<
        unsafe extern "C" fn(
            *mut Tcl_Interp,
            *const libc::c_char,
            libc::c_int,
        ) -> *const libc::c_char,
    >,
    pub tcl_GetVar2: Option::<
        unsafe extern "C" fn(
            *mut Tcl_Interp,
            *const libc::c_char,
            *const libc::c_char,
            libc::c_int,
        ) -> *const libc::c_char,
    >,
    pub tcl_GlobalEval: Option::<
        unsafe extern "C" fn(*mut Tcl_Interp, *const libc::c_char) -> libc::c_int,
    >,
    pub tcl_GlobalEvalObj: Option::<
        unsafe extern "C" fn(*mut Tcl_Interp, *mut Tcl_Obj) -> libc::c_int,
    >,
    pub tcl_HideCommand: Option::<
        unsafe extern "C" fn(
            *mut Tcl_Interp,
            *const libc::c_char,
            *const libc::c_char,
        ) -> libc::c_int,
    >,
    pub tcl_Init: Option::<unsafe extern "C" fn(*mut Tcl_Interp) -> libc::c_int>,
    pub tcl_InitHashTable: Option::<
        unsafe extern "C" fn(*mut Tcl_HashTable, libc::c_int) -> (),
    >,
    pub tcl_InputBlocked: Option::<unsafe extern "C" fn(Tcl_Channel) -> libc::c_int>,
    pub tcl_InputBuffered: Option::<unsafe extern "C" fn(Tcl_Channel) -> libc::c_int>,
    pub tcl_InterpDeleted: Option::<
        unsafe extern "C" fn(*mut Tcl_Interp) -> libc::c_int,
    >,
    pub tcl_IsSafe: Option::<unsafe extern "C" fn(*mut Tcl_Interp) -> libc::c_int>,
    pub tcl_JoinPath: Option::<
        unsafe extern "C" fn(
            libc::c_int,
            *const *const libc::c_char,
            *mut Tcl_DString,
        ) -> *mut libc::c_char,
    >,
    pub tcl_LinkVar: Option::<
        unsafe extern "C" fn(
            *mut Tcl_Interp,
            *const libc::c_char,
            *mut libc::c_char,
            libc::c_int,
        ) -> libc::c_int,
    >,
    pub reserved188: Option::<unsafe extern "C" fn() -> ()>,
    pub tcl_MakeFileChannel: Option::<
        unsafe extern "C" fn(ClientData, libc::c_int) -> Tcl_Channel,
    >,
    pub tcl_MakeSafe: Option::<unsafe extern "C" fn(*mut Tcl_Interp) -> libc::c_int>,
    pub tcl_MakeTcpClientChannel: Option::<
        unsafe extern "C" fn(ClientData) -> Tcl_Channel,
    >,
    pub tcl_Merge: Option::<
        unsafe extern "C" fn(
            libc::c_int,
            *const *const libc::c_char,
        ) -> *mut libc::c_char,
    >,
    pub tcl_NextHashEntry: Option::<
        unsafe extern "C" fn(*mut Tcl_HashSearch) -> *mut Tcl_HashEntry,
    >,
    pub tcl_NotifyChannel: Option::<
        unsafe extern "C" fn(Tcl_Channel, libc::c_int) -> (),
    >,
    pub tcl_ObjGetVar2: Option::<
        unsafe extern "C" fn(
            *mut Tcl_Interp,
            *mut Tcl_Obj,
            *mut Tcl_Obj,
            libc::c_int,
        ) -> *mut Tcl_Obj,
    >,
    pub tcl_ObjSetVar2: Option::<
        unsafe extern "C" fn(
            *mut Tcl_Interp,
            *mut Tcl_Obj,
            *mut Tcl_Obj,
            *mut Tcl_Obj,
            libc::c_int,
        ) -> *mut Tcl_Obj,
    >,
    pub tcl_OpenCommandChannel: Option::<
        unsafe extern "C" fn(
            *mut Tcl_Interp,
            libc::c_int,
            *mut *const libc::c_char,
            libc::c_int,
        ) -> Tcl_Channel,
    >,
    pub tcl_OpenFileChannel: Option::<
        unsafe extern "C" fn(
            *mut Tcl_Interp,
            *const libc::c_char,
            *const libc::c_char,
            libc::c_int,
        ) -> Tcl_Channel,
    >,
    pub tcl_OpenTcpClient: Option::<
        unsafe extern "C" fn(
            *mut Tcl_Interp,
            libc::c_int,
            *const libc::c_char,
            *const libc::c_char,
            libc::c_int,
            libc::c_int,
        ) -> Tcl_Channel,
    >,
    pub tcl_OpenTcpServer: Option::<
        unsafe extern "C" fn(
            *mut Tcl_Interp,
            libc::c_int,
            *const libc::c_char,
            Option::<Tcl_TcpAcceptProc>,
            ClientData,
        ) -> Tcl_Channel,
    >,
    pub tcl_Preserve: Option::<unsafe extern "C" fn(ClientData) -> ()>,
    pub tcl_PrintDouble: Option::<
        unsafe extern "C" fn(*mut Tcl_Interp, libc::c_double, *mut libc::c_char) -> (),
    >,
    pub tcl_PutEnv: Option::<unsafe extern "C" fn(*const libc::c_char) -> libc::c_int>,
    pub tcl_PosixError: Option::<
        unsafe extern "C" fn(*mut Tcl_Interp) -> *const libc::c_char,
    >,
    pub tcl_QueueEvent: Option::<
        unsafe extern "C" fn(*mut Tcl_Event, Tcl_QueuePosition) -> (),
    >,
    pub tcl_Read: Option::<
        unsafe extern "C" fn(Tcl_Channel, *mut libc::c_char, libc::c_int) -> libc::c_int,
    >,
    pub tcl_ReapDetachedProcs: Option::<unsafe extern "C" fn() -> ()>,
    pub tcl_RecordAndEval: Option::<
        unsafe extern "C" fn(
            *mut Tcl_Interp,
            *const libc::c_char,
            libc::c_int,
        ) -> libc::c_int,
    >,
    pub tcl_RecordAndEvalObj: Option::<
        unsafe extern "C" fn(*mut Tcl_Interp, *mut Tcl_Obj, libc::c_int) -> libc::c_int,
    >,
    pub tcl_RegisterChannel: Option::<
        unsafe extern "C" fn(*mut Tcl_Interp, Tcl_Channel) -> (),
    >,
    pub tcl_RegisterObjType: Option::<unsafe extern "C" fn(*const Tcl_ObjType) -> ()>,
    pub tcl_RegExpCompile: Option::<
        unsafe extern "C" fn(*mut Tcl_Interp, *const libc::c_char) -> Tcl_RegExp,
    >,
    pub tcl_RegExpExec: Option::<
        unsafe extern "C" fn(
            *mut Tcl_Interp,
            Tcl_RegExp,
            *const libc::c_char,
            *const libc::c_char,
        ) -> libc::c_int,
    >,
    pub tcl_RegExpMatch: Option::<
        unsafe extern "C" fn(
            *mut Tcl_Interp,
            *const libc::c_char,
            *const libc::c_char,
        ) -> libc::c_int,
    >,
    pub tcl_RegExpRange: Option::<
        unsafe extern "C" fn(
            Tcl_RegExp,
            libc::c_int,
            *mut *const libc::c_char,
            *mut *const libc::c_char,
        ) -> (),
    >,
    pub tcl_Release: Option::<unsafe extern "C" fn(ClientData) -> ()>,
    pub tcl_ResetResult: Option::<unsafe extern "C" fn(*mut Tcl_Interp) -> ()>,
    pub tcl_ScanElement: Option::<
        unsafe extern "C" fn(*const libc::c_char, *mut libc::c_int) -> libc::c_int,
    >,
    pub tcl_ScanCountedElement: Option::<
        unsafe extern "C" fn(
            *const libc::c_char,
            libc::c_int,
            *mut libc::c_int,
        ) -> libc::c_int,
    >,
    pub tcl_SeekOld: Option::<
        unsafe extern "C" fn(Tcl_Channel, libc::c_int, libc::c_int) -> libc::c_int,
    >,
    pub tcl_ServiceAll: Option::<unsafe extern "C" fn() -> libc::c_int>,
    pub tcl_ServiceEvent: Option::<unsafe extern "C" fn(libc::c_int) -> libc::c_int>,
    pub tcl_SetAssocData: Option::<
        unsafe extern "C" fn(
            *mut Tcl_Interp,
            *const libc::c_char,
            Option::<Tcl_InterpDeleteProc>,
            ClientData,
        ) -> (),
    >,
    pub tcl_SetChannelBufferSize: Option::<
        unsafe extern "C" fn(Tcl_Channel, libc::c_int) -> (),
    >,
    pub tcl_SetChannelOption: Option::<
        unsafe extern "C" fn(
            *mut Tcl_Interp,
            Tcl_Channel,
            *const libc::c_char,
            *const libc::c_char,
        ) -> libc::c_int,
    >,
    pub tcl_SetCommandInfo: Option::<
        unsafe extern "C" fn(
            *mut Tcl_Interp,
            *const libc::c_char,
            *const Tcl_CmdInfo,
        ) -> libc::c_int,
    >,
    pub tcl_SetErrno: Option::<unsafe extern "C" fn(libc::c_int) -> ()>,
    pub tcl_SetErrorCode: Option::<unsafe extern "C" fn(*mut Tcl_Interp, ...) -> ()>,
    pub tcl_SetMaxBlockTime: Option::<unsafe extern "C" fn(*const Tcl_Time) -> ()>,
    pub tcl_SetPanicProc: Option::<unsafe extern "C" fn(Option::<Tcl_PanicProc>) -> ()>,
    pub tcl_SetRecursionLimit: Option::<
        unsafe extern "C" fn(*mut Tcl_Interp, libc::c_int) -> libc::c_int,
    >,
    pub tcl_SetResult: Option::<
        unsafe extern "C" fn(
            *mut Tcl_Interp,
            *mut libc::c_char,
            Option::<Tcl_FreeProc>,
        ) -> (),
    >,
    pub tcl_SetServiceMode: Option::<unsafe extern "C" fn(libc::c_int) -> libc::c_int>,
    pub tcl_SetObjErrorCode: Option::<
        unsafe extern "C" fn(*mut Tcl_Interp, *mut Tcl_Obj) -> (),
    >,
    pub tcl_SetObjResult: Option::<
        unsafe extern "C" fn(*mut Tcl_Interp, *mut Tcl_Obj) -> (),
    >,
    pub tcl_SetStdChannel: Option::<
        unsafe extern "C" fn(Tcl_Channel, libc::c_int) -> (),
    >,
    pub tcl_SetVar: Option::<
        unsafe extern "C" fn(
            *mut Tcl_Interp,
            *const libc::c_char,
            *const libc::c_char,
            libc::c_int,
        ) -> *const libc::c_char,
    >,
    pub tcl_SetVar2: Option::<
        unsafe extern "C" fn(
            *mut Tcl_Interp,
            *const libc::c_char,
            *const libc::c_char,
            *const libc::c_char,
            libc::c_int,
        ) -> *const libc::c_char,
    >,
    pub tcl_SignalId: Option::<unsafe extern "C" fn(libc::c_int) -> *const libc::c_char>,
    pub tcl_SignalMsg: Option::<
        unsafe extern "C" fn(libc::c_int) -> *const libc::c_char,
    >,
    pub tcl_SourceRCFile: Option::<unsafe extern "C" fn(*mut Tcl_Interp) -> ()>,
    pub tcl_SplitList: Option::<
        unsafe extern "C" fn(
            *mut Tcl_Interp,
            *const libc::c_char,
            *mut libc::c_int,
            *mut *mut *const libc::c_char,
        ) -> libc::c_int,
    >,
    pub tcl_SplitPath: Option::<
        unsafe extern "C" fn(
            *const libc::c_char,
            *mut libc::c_int,
            *mut *mut *const libc::c_char,
        ) -> (),
    >,
    pub tcl_StaticPackage: Option::<
        unsafe extern "C" fn(
            *mut Tcl_Interp,
            *const libc::c_char,
            Option::<Tcl_PackageInitProc>,
            Option::<Tcl_PackageInitProc>,
        ) -> (),
    >,
    pub tcl_StringMatch: Option::<
        unsafe extern "C" fn(*const libc::c_char, *const libc::c_char) -> libc::c_int,
    >,
    pub tcl_TellOld: Option::<unsafe extern "C" fn(Tcl_Channel) -> libc::c_int>,
    pub tcl_TraceVar: Option::<
        unsafe extern "C" fn(
            *mut Tcl_Interp,
            *const libc::c_char,
            libc::c_int,
            Option::<Tcl_VarTraceProc>,
            ClientData,
        ) -> libc::c_int,
    >,
    pub tcl_TraceVar2: Option::<
        unsafe extern "C" fn(
            *mut Tcl_Interp,
            *const libc::c_char,
            *const libc::c_char,
            libc::c_int,
            Option::<Tcl_VarTraceProc>,
            ClientData,
        ) -> libc::c_int,
    >,
    pub tcl_TranslateFileName: Option::<
        unsafe extern "C" fn(
            *mut Tcl_Interp,
            *const libc::c_char,
            *mut Tcl_DString,
        ) -> *mut libc::c_char,
    >,
    pub tcl_Ungets: Option::<
        unsafe extern "C" fn(
            Tcl_Channel,
            *const libc::c_char,
            libc::c_int,
            libc::c_int,
        ) -> libc::c_int,
    >,
    pub tcl_UnlinkVar: Option::<
        unsafe extern "C" fn(*mut Tcl_Interp, *const libc::c_char) -> (),
    >,
    pub tcl_UnregisterChannel: Option::<
        unsafe extern "C" fn(*mut Tcl_Interp, Tcl_Channel) -> libc::c_int,
    >,
    pub tcl_UnsetVar: Option::<
        unsafe extern "C" fn(
            *mut Tcl_Interp,
            *const libc::c_char,
            libc::c_int,
        ) -> libc::c_int,
    >,
    pub tcl_UnsetVar2: Option::<
        unsafe extern "C" fn(
            *mut Tcl_Interp,
            *const libc::c_char,
            *const libc::c_char,
            libc::c_int,
        ) -> libc::c_int,
    >,
    pub tcl_UntraceVar: Option::<
        unsafe extern "C" fn(
            *mut Tcl_Interp,
            *const libc::c_char,
            libc::c_int,
            Option::<Tcl_VarTraceProc>,
            ClientData,
        ) -> (),
    >,
    pub tcl_UntraceVar2: Option::<
        unsafe extern "C" fn(
            *mut Tcl_Interp,
            *const libc::c_char,
            *const libc::c_char,
            libc::c_int,
            Option::<Tcl_VarTraceProc>,
            ClientData,
        ) -> (),
    >,
    pub tcl_UpdateLinkedVar: Option::<
        unsafe extern "C" fn(*mut Tcl_Interp, *const libc::c_char) -> (),
    >,
    pub tcl_UpVar: Option::<
        unsafe extern "C" fn(
            *mut Tcl_Interp,
            *const libc::c_char,
            *const libc::c_char,
            *const libc::c_char,
            libc::c_int,
        ) -> libc::c_int,
    >,
    pub tcl_UpVar2: Option::<
        unsafe extern "C" fn(
            *mut Tcl_Interp,
            *const libc::c_char,
            *const libc::c_char,
            *const libc::c_char,
            *const libc::c_char,
            libc::c_int,
        ) -> libc::c_int,
    >,
    pub tcl_VarEval: Option::<unsafe extern "C" fn(*mut Tcl_Interp, ...) -> libc::c_int>,
    pub tcl_VarTraceInfo: Option::<
        unsafe extern "C" fn(
            *mut Tcl_Interp,
            *const libc::c_char,
            libc::c_int,
            Option::<Tcl_VarTraceProc>,
            ClientData,
        ) -> ClientData,
    >,
    pub tcl_VarTraceInfo2: Option::<
        unsafe extern "C" fn(
            *mut Tcl_Interp,
            *const libc::c_char,
            *const libc::c_char,
            libc::c_int,
            Option::<Tcl_VarTraceProc>,
            ClientData,
        ) -> ClientData,
    >,
    pub tcl_Write: Option::<
        unsafe extern "C" fn(
            Tcl_Channel,
            *const libc::c_char,
            libc::c_int,
        ) -> libc::c_int,
    >,
    pub tcl_WrongNumArgs: Option::<
        unsafe extern "C" fn(
            *mut Tcl_Interp,
            libc::c_int,
            *const *mut Tcl_Obj,
            *const libc::c_char,
        ) -> (),
    >,
    pub tcl_DumpActiveMemory: Option::<
        unsafe extern "C" fn(*const libc::c_char) -> libc::c_int,
    >,
    pub tcl_ValidateAllMemory: Option::<
        unsafe extern "C" fn(*const libc::c_char, libc::c_int) -> (),
    >,
    pub tcl_AppendResultVA: Option::<
        unsafe extern "C" fn(*mut Tcl_Interp, ::std::ffi::VaList) -> (),
    >,
    pub tcl_AppendStringsToObjVA: Option::<
        unsafe extern "C" fn(*mut Tcl_Obj, ::std::ffi::VaList) -> (),
    >,
    pub tcl_HashStats: Option::<
        unsafe extern "C" fn(*mut Tcl_HashTable) -> *mut libc::c_char,
    >,
    pub tcl_ParseVar: Option::<
        unsafe extern "C" fn(
            *mut Tcl_Interp,
            *const libc::c_char,
            *mut *const libc::c_char,
        ) -> *const libc::c_char,
    >,
    pub tcl_PkgPresent: Option::<
        unsafe extern "C" fn(
            *mut Tcl_Interp,
            *const libc::c_char,
            *const libc::c_char,
            libc::c_int,
        ) -> *const libc::c_char,
    >,
    pub tcl_PkgPresentEx: Option::<
        unsafe extern "C" fn(
            *mut Tcl_Interp,
            *const libc::c_char,
            *const libc::c_char,
            libc::c_int,
            *mut libc::c_void,
        ) -> *const libc::c_char,
    >,
    pub tcl_PkgProvide: Option::<
        unsafe extern "C" fn(
            *mut Tcl_Interp,
            *const libc::c_char,
            *const libc::c_char,
        ) -> libc::c_int,
    >,
    pub tcl_PkgRequire: Option::<
        unsafe extern "C" fn(
            *mut Tcl_Interp,
            *const libc::c_char,
            *const libc::c_char,
            libc::c_int,
        ) -> *const libc::c_char,
    >,
    pub tcl_SetErrorCodeVA: Option::<
        unsafe extern "C" fn(*mut Tcl_Interp, ::std::ffi::VaList) -> (),
    >,
    pub tcl_VarEvalVA: Option::<
        unsafe extern "C" fn(*mut Tcl_Interp, ::std::ffi::VaList) -> libc::c_int,
    >,
    pub tcl_WaitPid: Option::<
        unsafe extern "C" fn(Tcl_Pid, *mut libc::c_int, libc::c_int) -> Tcl_Pid,
    >,
    pub tcl_PanicVA: Option::<
        unsafe extern "C" fn(*const libc::c_char, ::std::ffi::VaList) -> (),
    >,
    pub tcl_GetVersion: Option::<
        unsafe extern "C" fn(
            *mut libc::c_int,
            *mut libc::c_int,
            *mut libc::c_int,
            *mut libc::c_int,
        ) -> (),
    >,
    pub tcl_InitMemory: Option::<unsafe extern "C" fn(*mut Tcl_Interp) -> ()>,
    pub tcl_StackChannel: Option::<
        unsafe extern "C" fn(
            *mut Tcl_Interp,
            *const Tcl_ChannelType,
            ClientData,
            libc::c_int,
            Tcl_Channel,
        ) -> Tcl_Channel,
    >,
    pub tcl_UnstackChannel: Option::<
        unsafe extern "C" fn(*mut Tcl_Interp, Tcl_Channel) -> libc::c_int,
    >,
    pub tcl_GetStackedChannel: Option::<
        unsafe extern "C" fn(Tcl_Channel) -> Tcl_Channel,
    >,
    pub tcl_SetMainLoop: Option::<
        unsafe extern "C" fn(Option::<Tcl_MainLoopProc>) -> (),
    >,
    pub reserved285: Option::<unsafe extern "C" fn() -> ()>,
    pub tcl_AppendObjToObj: Option::<
        unsafe extern "C" fn(*mut Tcl_Obj, *mut Tcl_Obj) -> (),
    >,
    pub tcl_CreateEncoding: Option::<
        unsafe extern "C" fn(*const Tcl_EncodingType) -> Tcl_Encoding,
    >,
    pub tcl_CreateThreadExitHandler: Option::<
        unsafe extern "C" fn(Option::<Tcl_ExitProc>, ClientData) -> (),
    >,
    pub tcl_DeleteThreadExitHandler: Option::<
        unsafe extern "C" fn(Option::<Tcl_ExitProc>, ClientData) -> (),
    >,
    pub tcl_DiscardResult: Option::<unsafe extern "C" fn(*mut Tcl_SavedResult) -> ()>,
    pub tcl_EvalEx: Option::<
        unsafe extern "C" fn(
            *mut Tcl_Interp,
            *const libc::c_char,
            libc::c_int,
            libc::c_int,
        ) -> libc::c_int,
    >,
    pub tcl_EvalObjv: Option::<
        unsafe extern "C" fn(
            *mut Tcl_Interp,
            libc::c_int,
            *const *mut Tcl_Obj,
            libc::c_int,
        ) -> libc::c_int,
    >,
    pub tcl_EvalObjEx: Option::<
        unsafe extern "C" fn(*mut Tcl_Interp, *mut Tcl_Obj, libc::c_int) -> libc::c_int,
    >,
    pub tcl_ExitThread: Option::<unsafe extern "C" fn(libc::c_int) -> ()>,
    pub tcl_ExternalToUtf: Option::<
        unsafe extern "C" fn(
            *mut Tcl_Interp,
            Tcl_Encoding,
            *const libc::c_char,
            libc::c_int,
            libc::c_int,
            *mut Tcl_EncodingState,
            *mut libc::c_char,
            libc::c_int,
            *mut libc::c_int,
            *mut libc::c_int,
            *mut libc::c_int,
        ) -> libc::c_int,
    >,
    pub tcl_ExternalToUtfDString: Option::<
        unsafe extern "C" fn(
            Tcl_Encoding,
            *const libc::c_char,
            libc::c_int,
            *mut Tcl_DString,
        ) -> *mut libc::c_char,
    >,
    pub tcl_FinalizeThread: Option::<unsafe extern "C" fn() -> ()>,
    pub tcl_FinalizeNotifier: Option::<unsafe extern "C" fn(ClientData) -> ()>,
    pub tcl_FreeEncoding: Option::<unsafe extern "C" fn(Tcl_Encoding) -> ()>,
    pub tcl_GetCurrentThread: Option::<unsafe extern "C" fn() -> Tcl_ThreadId>,
    pub tcl_GetEncoding: Option::<
        unsafe extern "C" fn(*mut Tcl_Interp, *const libc::c_char) -> Tcl_Encoding,
    >,
    pub tcl_GetEncodingName: Option::<
        unsafe extern "C" fn(Tcl_Encoding) -> *const libc::c_char,
    >,
    pub tcl_GetEncodingNames: Option::<unsafe extern "C" fn(*mut Tcl_Interp) -> ()>,
    pub tcl_GetIndexFromObjStruct: Option::<
        unsafe extern "C" fn(
            *mut Tcl_Interp,
            *mut Tcl_Obj,
            *const libc::c_void,
            libc::c_int,
            *const libc::c_char,
            libc::c_int,
            *mut libc::c_int,
        ) -> libc::c_int,
    >,
    pub tcl_GetThreadData: Option::<
        unsafe extern "C" fn(*mut Tcl_ThreadDataKey, libc::c_int) -> *mut libc::c_void,
    >,
    pub tcl_GetVar2Ex: Option::<
        unsafe extern "C" fn(
            *mut Tcl_Interp,
            *const libc::c_char,
            *const libc::c_char,
            libc::c_int,
        ) -> *mut Tcl_Obj,
    >,
    pub tcl_InitNotifier: Option::<unsafe extern "C" fn() -> ClientData>,
    pub tcl_MutexLock: Option::<unsafe extern "C" fn(*mut Tcl_Mutex) -> ()>,
    pub tcl_MutexUnlock: Option::<unsafe extern "C" fn(*mut Tcl_Mutex) -> ()>,
    pub tcl_ConditionNotify: Option::<unsafe extern "C" fn(*mut Tcl_Condition) -> ()>,
    pub tcl_ConditionWait: Option::<
        unsafe extern "C" fn(*mut Tcl_Condition, *mut Tcl_Mutex, *const Tcl_Time) -> (),
    >,
    pub tcl_NumUtfChars: Option::<
        unsafe extern "C" fn(*const libc::c_char, libc::c_int) -> libc::c_int,
    >,
    pub tcl_ReadChars: Option::<
        unsafe extern "C" fn(
            Tcl_Channel,
            *mut Tcl_Obj,
            libc::c_int,
            libc::c_int,
        ) -> libc::c_int,
    >,
    pub tcl_RestoreResult: Option::<
        unsafe extern "C" fn(*mut Tcl_Interp, *mut Tcl_SavedResult) -> (),
    >,
    pub tcl_SaveResult: Option::<
        unsafe extern "C" fn(*mut Tcl_Interp, *mut Tcl_SavedResult) -> (),
    >,
    pub tcl_SetSystemEncoding: Option::<
        unsafe extern "C" fn(*mut Tcl_Interp, *const libc::c_char) -> libc::c_int,
    >,
    pub tcl_SetVar2Ex: Option::<
        unsafe extern "C" fn(
            *mut Tcl_Interp,
            *const libc::c_char,
            *const libc::c_char,
            *mut Tcl_Obj,
            libc::c_int,
        ) -> *mut Tcl_Obj,
    >,
    pub tcl_ThreadAlert: Option::<unsafe extern "C" fn(Tcl_ThreadId) -> ()>,
    pub tcl_ThreadQueueEvent: Option::<
        unsafe extern "C" fn(Tcl_ThreadId, *mut Tcl_Event, Tcl_QueuePosition) -> (),
    >,
    pub tcl_UniCharAtIndex: Option::<
        unsafe extern "C" fn(*const libc::c_char, libc::c_int) -> Tcl_UniChar,
    >,
    pub tcl_UniCharToLower: Option::<unsafe extern "C" fn(libc::c_int) -> Tcl_UniChar>,
    pub tcl_UniCharToTitle: Option::<unsafe extern "C" fn(libc::c_int) -> Tcl_UniChar>,
    pub tcl_UniCharToUpper: Option::<unsafe extern "C" fn(libc::c_int) -> Tcl_UniChar>,
    pub tcl_UniCharToUtf: Option::<
        unsafe extern "C" fn(libc::c_int, *mut libc::c_char) -> libc::c_int,
    >,
    pub tcl_UtfAtIndex: Option::<
        unsafe extern "C" fn(*const libc::c_char, libc::c_int) -> *const libc::c_char,
    >,
    pub tcl_UtfCharComplete: Option::<
        unsafe extern "C" fn(*const libc::c_char, libc::c_int) -> libc::c_int,
    >,
    pub tcl_UtfBackslash: Option::<
        unsafe extern "C" fn(
            *const libc::c_char,
            *mut libc::c_int,
            *mut libc::c_char,
        ) -> libc::c_int,
    >,
    pub tcl_UtfFindFirst: Option::<
        unsafe extern "C" fn(*const libc::c_char, libc::c_int) -> *const libc::c_char,
    >,
    pub tcl_UtfFindLast: Option::<
        unsafe extern "C" fn(*const libc::c_char, libc::c_int) -> *const libc::c_char,
    >,
    pub tcl_UtfNext: Option::<
        unsafe extern "C" fn(*const libc::c_char) -> *const libc::c_char,
    >,
    pub tcl_UtfPrev: Option::<
        unsafe extern "C" fn(
            *const libc::c_char,
            *const libc::c_char,
        ) -> *const libc::c_char,
    >,
    pub tcl_UtfToExternal: Option::<
        unsafe extern "C" fn(
            *mut Tcl_Interp,
            Tcl_Encoding,
            *const libc::c_char,
            libc::c_int,
            libc::c_int,
            *mut Tcl_EncodingState,
            *mut libc::c_char,
            libc::c_int,
            *mut libc::c_int,
            *mut libc::c_int,
            *mut libc::c_int,
        ) -> libc::c_int,
    >,
    pub tcl_UtfToExternalDString: Option::<
        unsafe extern "C" fn(
            Tcl_Encoding,
            *const libc::c_char,
            libc::c_int,
            *mut Tcl_DString,
        ) -> *mut libc::c_char,
    >,
    pub tcl_UtfToLower: Option::<unsafe extern "C" fn(*mut libc::c_char) -> libc::c_int>,
    pub tcl_UtfToTitle: Option::<unsafe extern "C" fn(*mut libc::c_char) -> libc::c_int>,
    pub tcl_UtfToUniChar: Option::<
        unsafe extern "C" fn(*const libc::c_char, *mut Tcl_UniChar) -> libc::c_int,
    >,
    pub tcl_UtfToUpper: Option::<unsafe extern "C" fn(*mut libc::c_char) -> libc::c_int>,
    pub tcl_WriteChars: Option::<
        unsafe extern "C" fn(
            Tcl_Channel,
            *const libc::c_char,
            libc::c_int,
        ) -> libc::c_int,
    >,
    pub tcl_WriteObj: Option::<
        unsafe extern "C" fn(Tcl_Channel, *mut Tcl_Obj) -> libc::c_int,
    >,
    pub tcl_GetString: Option::<unsafe extern "C" fn(*mut Tcl_Obj) -> *mut libc::c_char>,
    pub tcl_GetDefaultEncodingDir: Option::<
        unsafe extern "C" fn() -> *const libc::c_char,
    >,
    pub tcl_SetDefaultEncodingDir: Option::<
        unsafe extern "C" fn(*const libc::c_char) -> (),
    >,
    pub tcl_AlertNotifier: Option::<unsafe extern "C" fn(ClientData) -> ()>,
    pub tcl_ServiceModeHook: Option::<unsafe extern "C" fn(libc::c_int) -> ()>,
    pub tcl_UniCharIsAlnum: Option::<unsafe extern "C" fn(libc::c_int) -> libc::c_int>,
    pub tcl_UniCharIsAlpha: Option::<unsafe extern "C" fn(libc::c_int) -> libc::c_int>,
    pub tcl_UniCharIsDigit: Option::<unsafe extern "C" fn(libc::c_int) -> libc::c_int>,
    pub tcl_UniCharIsLower: Option::<unsafe extern "C" fn(libc::c_int) -> libc::c_int>,
    pub tcl_UniCharIsSpace: Option::<unsafe extern "C" fn(libc::c_int) -> libc::c_int>,
    pub tcl_UniCharIsUpper: Option::<unsafe extern "C" fn(libc::c_int) -> libc::c_int>,
    pub tcl_UniCharIsWordChar: Option::<
        unsafe extern "C" fn(libc::c_int) -> libc::c_int,
    >,
    pub tcl_UniCharLen: Option::<
        unsafe extern "C" fn(*const Tcl_UniChar) -> libc::c_int,
    >,
    pub tcl_UniCharNcmp: Option::<
        unsafe extern "C" fn(
            *const Tcl_UniChar,
            *const Tcl_UniChar,
            libc::c_ulong,
        ) -> libc::c_int,
    >,
    pub tcl_UniCharToUtfDString: Option::<
        unsafe extern "C" fn(
            *const Tcl_UniChar,
            libc::c_int,
            *mut Tcl_DString,
        ) -> *mut libc::c_char,
    >,
    pub tcl_UtfToUniCharDString: Option::<
        unsafe extern "C" fn(
            *const libc::c_char,
            libc::c_int,
            *mut Tcl_DString,
        ) -> *mut Tcl_UniChar,
    >,
    pub tcl_GetRegExpFromObj: Option::<
        unsafe extern "C" fn(*mut Tcl_Interp, *mut Tcl_Obj, libc::c_int) -> Tcl_RegExp,
    >,
    pub tcl_EvalTokens: Option::<
        unsafe extern "C" fn(
            *mut Tcl_Interp,
            *mut Tcl_Token,
            libc::c_int,
        ) -> *mut Tcl_Obj,
    >,
    pub tcl_FreeParse: Option::<unsafe extern "C" fn(*mut Tcl_Parse) -> ()>,
    pub tcl_LogCommandInfo: Option::<
        unsafe extern "C" fn(
            *mut Tcl_Interp,
            *const libc::c_char,
            *const libc::c_char,
            libc::c_int,
        ) -> (),
    >,
    pub tcl_ParseBraces: Option::<
        unsafe extern "C" fn(
            *mut Tcl_Interp,
            *const libc::c_char,
            libc::c_int,
            *mut Tcl_Parse,
            libc::c_int,
            *mut *const libc::c_char,
        ) -> libc::c_int,
    >,
    pub tcl_ParseCommand: Option::<
        unsafe extern "C" fn(
            *mut Tcl_Interp,
            *const libc::c_char,
            libc::c_int,
            libc::c_int,
            *mut Tcl_Parse,
        ) -> libc::c_int,
    >,
    pub tcl_ParseExpr: Option::<
        unsafe extern "C" fn(
            *mut Tcl_Interp,
            *const libc::c_char,
            libc::c_int,
            *mut Tcl_Parse,
        ) -> libc::c_int,
    >,
    pub tcl_ParseQuotedString: Option::<
        unsafe extern "C" fn(
            *mut Tcl_Interp,
            *const libc::c_char,
            libc::c_int,
            *mut Tcl_Parse,
            libc::c_int,
            *mut *const libc::c_char,
        ) -> libc::c_int,
    >,
    pub tcl_ParseVarName: Option::<
        unsafe extern "C" fn(
            *mut Tcl_Interp,
            *const libc::c_char,
            libc::c_int,
            *mut Tcl_Parse,
            libc::c_int,
        ) -> libc::c_int,
    >,
    pub tcl_GetCwd: Option::<
        unsafe extern "C" fn(*mut Tcl_Interp, *mut Tcl_DString) -> *mut libc::c_char,
    >,
    pub tcl_Chdir: Option::<unsafe extern "C" fn(*const libc::c_char) -> libc::c_int>,
    pub tcl_Access: Option::<
        unsafe extern "C" fn(*const libc::c_char, libc::c_int) -> libc::c_int,
    >,
    pub tcl_Stat: Option::<
        unsafe extern "C" fn(*const libc::c_char, *mut stat) -> libc::c_int,
    >,
    pub tcl_UtfNcmp: Option::<
        unsafe extern "C" fn(
            *const libc::c_char,
            *const libc::c_char,
            libc::c_ulong,
        ) -> libc::c_int,
    >,
    pub tcl_UtfNcasecmp: Option::<
        unsafe extern "C" fn(
            *const libc::c_char,
            *const libc::c_char,
            libc::c_ulong,
        ) -> libc::c_int,
    >,
    pub tcl_StringCaseMatch: Option::<
        unsafe extern "C" fn(
            *const libc::c_char,
            *const libc::c_char,
            libc::c_int,
        ) -> libc::c_int,
    >,
    pub tcl_UniCharIsControl: Option::<unsafe extern "C" fn(libc::c_int) -> libc::c_int>,
    pub tcl_UniCharIsGraph: Option::<unsafe extern "C" fn(libc::c_int) -> libc::c_int>,
    pub tcl_UniCharIsPrint: Option::<unsafe extern "C" fn(libc::c_int) -> libc::c_int>,
    pub tcl_UniCharIsPunct: Option::<unsafe extern "C" fn(libc::c_int) -> libc::c_int>,
    pub tcl_RegExpExecObj: Option::<
        unsafe extern "C" fn(
            *mut Tcl_Interp,
            Tcl_RegExp,
            *mut Tcl_Obj,
            libc::c_int,
            libc::c_int,
            libc::c_int,
        ) -> libc::c_int,
    >,
    pub tcl_RegExpGetInfo: Option::<
        unsafe extern "C" fn(Tcl_RegExp, *mut Tcl_RegExpInfo) -> (),
    >,
    pub tcl_NewUnicodeObj: Option::<
        unsafe extern "C" fn(*const Tcl_UniChar, libc::c_int) -> *mut Tcl_Obj,
    >,
    pub tcl_SetUnicodeObj: Option::<
        unsafe extern "C" fn(*mut Tcl_Obj, *const Tcl_UniChar, libc::c_int) -> (),
    >,
    pub tcl_GetCharLength: Option::<unsafe extern "C" fn(*mut Tcl_Obj) -> libc::c_int>,
    pub tcl_GetUniChar: Option::<
        unsafe extern "C" fn(*mut Tcl_Obj, libc::c_int) -> Tcl_UniChar,
    >,
    pub tcl_GetUnicode: Option::<unsafe extern "C" fn(*mut Tcl_Obj) -> *mut Tcl_UniChar>,
    pub tcl_GetRange: Option::<
        unsafe extern "C" fn(*mut Tcl_Obj, libc::c_int, libc::c_int) -> *mut Tcl_Obj,
    >,
    pub tcl_AppendUnicodeToObj: Option::<
        unsafe extern "C" fn(*mut Tcl_Obj, *const Tcl_UniChar, libc::c_int) -> (),
    >,
    pub tcl_RegExpMatchObj: Option::<
        unsafe extern "C" fn(*mut Tcl_Interp, *mut Tcl_Obj, *mut Tcl_Obj) -> libc::c_int,
    >,
    pub tcl_SetNotifier: Option::<unsafe extern "C" fn(*mut Tcl_NotifierProcs) -> ()>,
    pub tcl_GetAllocMutex: Option::<unsafe extern "C" fn() -> *mut Tcl_Mutex>,
    pub tcl_GetChannelNames: Option::<
        unsafe extern "C" fn(*mut Tcl_Interp) -> libc::c_int,
    >,
    pub tcl_GetChannelNamesEx: Option::<
        unsafe extern "C" fn(*mut Tcl_Interp, *const libc::c_char) -> libc::c_int,
    >,
    pub tcl_ProcObjCmd: Option::<
        unsafe extern "C" fn(
            ClientData,
            *mut Tcl_Interp,
            libc::c_int,
            *const *mut Tcl_Obj,
        ) -> libc::c_int,
    >,
    pub tcl_ConditionFinalize: Option::<unsafe extern "C" fn(*mut Tcl_Condition) -> ()>,
    pub tcl_MutexFinalize: Option::<unsafe extern "C" fn(*mut Tcl_Mutex) -> ()>,
    pub tcl_CreateThread: Option::<
        unsafe extern "C" fn(
            *mut Tcl_ThreadId,
            Option::<Tcl_ThreadCreateProc>,
            ClientData,
            libc::c_int,
            libc::c_int,
        ) -> libc::c_int,
    >,
    pub tcl_ReadRaw: Option::<
        unsafe extern "C" fn(Tcl_Channel, *mut libc::c_char, libc::c_int) -> libc::c_int,
    >,
    pub tcl_WriteRaw: Option::<
        unsafe extern "C" fn(
            Tcl_Channel,
            *const libc::c_char,
            libc::c_int,
        ) -> libc::c_int,
    >,
    pub tcl_GetTopChannel: Option::<unsafe extern "C" fn(Tcl_Channel) -> Tcl_Channel>,
    pub tcl_ChannelBuffered: Option::<unsafe extern "C" fn(Tcl_Channel) -> libc::c_int>,
    pub tcl_ChannelName: Option::<
        unsafe extern "C" fn(*const Tcl_ChannelType) -> *const libc::c_char,
    >,
    pub tcl_ChannelVersion: Option::<
        unsafe extern "C" fn(*const Tcl_ChannelType) -> Tcl_ChannelTypeVersion,
    >,
    pub tcl_ChannelBlockModeProc: Option::<
        unsafe extern "C" fn(*const Tcl_ChannelType) -> Option::<Tcl_DriverBlockModeProc>,
    >,
    pub tcl_ChannelCloseProc: Option::<
        unsafe extern "C" fn(*const Tcl_ChannelType) -> Option::<Tcl_DriverCloseProc>,
    >,
    pub tcl_ChannelClose2Proc: Option::<
        unsafe extern "C" fn(*const Tcl_ChannelType) -> Option::<Tcl_DriverClose2Proc>,
    >,
    pub tcl_ChannelInputProc: Option::<
        unsafe extern "C" fn(*const Tcl_ChannelType) -> Option::<Tcl_DriverInputProc>,
    >,
    pub tcl_ChannelOutputProc: Option::<
        unsafe extern "C" fn(*const Tcl_ChannelType) -> Option::<Tcl_DriverOutputProc>,
    >,
    pub tcl_ChannelSeekProc: Option::<
        unsafe extern "C" fn(*const Tcl_ChannelType) -> Option::<Tcl_DriverSeekProc>,
    >,
    pub tcl_ChannelSetOptionProc: Option::<
        unsafe extern "C" fn(*const Tcl_ChannelType) -> Option::<Tcl_DriverSetOptionProc>,
    >,
    pub tcl_ChannelGetOptionProc: Option::<
        unsafe extern "C" fn(*const Tcl_ChannelType) -> Option::<Tcl_DriverGetOptionProc>,
    >,
    pub tcl_ChannelWatchProc: Option::<
        unsafe extern "C" fn(*const Tcl_ChannelType) -> Option::<Tcl_DriverWatchProc>,
    >,
    pub tcl_ChannelGetHandleProc: Option::<
        unsafe extern "C" fn(*const Tcl_ChannelType) -> Option::<Tcl_DriverGetHandleProc>,
    >,
    pub tcl_ChannelFlushProc: Option::<
        unsafe extern "C" fn(*const Tcl_ChannelType) -> Option::<Tcl_DriverFlushProc>,
    >,
    pub tcl_ChannelHandlerProc: Option::<
        unsafe extern "C" fn(*const Tcl_ChannelType) -> Option::<Tcl_DriverHandlerProc>,
    >,
    pub tcl_JoinThread: Option::<
        unsafe extern "C" fn(Tcl_ThreadId, *mut libc::c_int) -> libc::c_int,
    >,
    pub tcl_IsChannelShared: Option::<unsafe extern "C" fn(Tcl_Channel) -> libc::c_int>,
    pub tcl_IsChannelRegistered: Option::<
        unsafe extern "C" fn(*mut Tcl_Interp, Tcl_Channel) -> libc::c_int,
    >,
    pub tcl_CutChannel: Option::<unsafe extern "C" fn(Tcl_Channel) -> ()>,
    pub tcl_SpliceChannel: Option::<unsafe extern "C" fn(Tcl_Channel) -> ()>,
    pub tcl_ClearChannelHandlers: Option::<unsafe extern "C" fn(Tcl_Channel) -> ()>,
    pub tcl_IsChannelExisting: Option::<
        unsafe extern "C" fn(*const libc::c_char) -> libc::c_int,
    >,
    pub tcl_UniCharNcasecmp: Option::<
        unsafe extern "C" fn(
            *const Tcl_UniChar,
            *const Tcl_UniChar,
            libc::c_ulong,
        ) -> libc::c_int,
    >,
    pub tcl_UniCharCaseMatch: Option::<
        unsafe extern "C" fn(
            *const Tcl_UniChar,
            *const Tcl_UniChar,
            libc::c_int,
        ) -> libc::c_int,
    >,
    pub tcl_FindHashEntry: Option::<
        unsafe extern "C" fn(
            *mut Tcl_HashTable,
            *const libc::c_void,
        ) -> *mut Tcl_HashEntry,
    >,
    pub tcl_CreateHashEntry: Option::<
        unsafe extern "C" fn(
            *mut Tcl_HashTable,
            *const libc::c_void,
            *mut libc::c_int,
        ) -> *mut Tcl_HashEntry,
    >,
    pub tcl_InitCustomHashTable: Option::<
        unsafe extern "C" fn(
            *mut Tcl_HashTable,
            libc::c_int,
            *const Tcl_HashKeyType,
        ) -> (),
    >,
    pub tcl_InitObjHashTable: Option::<unsafe extern "C" fn(*mut Tcl_HashTable) -> ()>,
    pub tcl_CommandTraceInfo: Option::<
        unsafe extern "C" fn(
            *mut Tcl_Interp,
            *const libc::c_char,
            libc::c_int,
            Option::<Tcl_CommandTraceProc>,
            ClientData,
        ) -> ClientData,
    >,
    pub tcl_TraceCommand: Option::<
        unsafe extern "C" fn(
            *mut Tcl_Interp,
            *const libc::c_char,
            libc::c_int,
            Option::<Tcl_CommandTraceProc>,
            ClientData,
        ) -> libc::c_int,
    >,
    pub tcl_UntraceCommand: Option::<
        unsafe extern "C" fn(
            *mut Tcl_Interp,
            *const libc::c_char,
            libc::c_int,
            Option::<Tcl_CommandTraceProc>,
            ClientData,
        ) -> (),
    >,
    pub tcl_AttemptAlloc: Option::<
        unsafe extern "C" fn(libc::c_uint) -> *mut libc::c_char,
    >,
    pub tcl_AttemptDbCkalloc: Option::<
        unsafe extern "C" fn(
            libc::c_uint,
            *const libc::c_char,
            libc::c_int,
        ) -> *mut libc::c_char,
    >,
    pub tcl_AttemptRealloc: Option::<
        unsafe extern "C" fn(*mut libc::c_char, libc::c_uint) -> *mut libc::c_char,
    >,
    pub tcl_AttemptDbCkrealloc: Option::<
        unsafe extern "C" fn(
            *mut libc::c_char,
            libc::c_uint,
            *const libc::c_char,
            libc::c_int,
        ) -> *mut libc::c_char,
    >,
    pub tcl_AttemptSetObjLength: Option::<
        unsafe extern "C" fn(*mut Tcl_Obj, libc::c_int) -> libc::c_int,
    >,
    pub tcl_GetChannelThread: Option::<
        unsafe extern "C" fn(Tcl_Channel) -> Tcl_ThreadId,
    >,
    pub tcl_GetUnicodeFromObj: Option::<
        unsafe extern "C" fn(*mut Tcl_Obj, *mut libc::c_int) -> *mut Tcl_UniChar,
    >,
    pub tcl_GetMathFuncInfo: Option::<
        unsafe extern "C" fn(
            *mut Tcl_Interp,
            *const libc::c_char,
            *mut libc::c_int,
            *mut *mut Tcl_ValueType,
            *mut Option::<Tcl_MathProc>,
            *mut ClientData,
        ) -> libc::c_int,
    >,
    pub tcl_ListMathFuncs: Option::<
        unsafe extern "C" fn(*mut Tcl_Interp, *const libc::c_char) -> *mut Tcl_Obj,
    >,
    pub tcl_SubstObj: Option::<
        unsafe extern "C" fn(*mut Tcl_Interp, *mut Tcl_Obj, libc::c_int) -> *mut Tcl_Obj,
    >,
    pub tcl_DetachChannel: Option::<
        unsafe extern "C" fn(*mut Tcl_Interp, Tcl_Channel) -> libc::c_int,
    >,
    pub tcl_IsStandardChannel: Option::<
        unsafe extern "C" fn(Tcl_Channel) -> libc::c_int,
    >,
    pub tcl_FSCopyFile: Option::<
        unsafe extern "C" fn(*mut Tcl_Obj, *mut Tcl_Obj) -> libc::c_int,
    >,
    pub tcl_FSCopyDirectory: Option::<
        unsafe extern "C" fn(
            *mut Tcl_Obj,
            *mut Tcl_Obj,
            *mut *mut Tcl_Obj,
        ) -> libc::c_int,
    >,
    pub tcl_FSCreateDirectory: Option::<
        unsafe extern "C" fn(*mut Tcl_Obj) -> libc::c_int,
    >,
    pub tcl_FSDeleteFile: Option::<unsafe extern "C" fn(*mut Tcl_Obj) -> libc::c_int>,
    pub tcl_FSLoadFile: Option::<
        unsafe extern "C" fn(
            *mut Tcl_Interp,
            *mut Tcl_Obj,
            *const libc::c_char,
            *const libc::c_char,
            *mut Option::<Tcl_PackageInitProc>,
            *mut Option::<Tcl_PackageInitProc>,
            *mut Tcl_LoadHandle,
            *mut Option::<Tcl_FSUnloadFileProc>,
        ) -> libc::c_int,
    >,
    pub tcl_FSMatchInDirectory: Option::<
        unsafe extern "C" fn(
            *mut Tcl_Interp,
            *mut Tcl_Obj,
            *mut Tcl_Obj,
            *const libc::c_char,
            *mut Tcl_GlobTypeData,
        ) -> libc::c_int,
    >,
    pub tcl_FSLink: Option::<
        unsafe extern "C" fn(*mut Tcl_Obj, *mut Tcl_Obj, libc::c_int) -> *mut Tcl_Obj,
    >,
    pub tcl_FSRemoveDirectory: Option::<
        unsafe extern "C" fn(*mut Tcl_Obj, libc::c_int, *mut *mut Tcl_Obj) -> libc::c_int,
    >,
    pub tcl_FSRenameFile: Option::<
        unsafe extern "C" fn(*mut Tcl_Obj, *mut Tcl_Obj) -> libc::c_int,
    >,
    pub tcl_FSLstat: Option::<
        unsafe extern "C" fn(*mut Tcl_Obj, *mut Tcl_StatBuf) -> libc::c_int,
    >,
    pub tcl_FSUtime: Option::<
        unsafe extern "C" fn(*mut Tcl_Obj, *mut utimbuf) -> libc::c_int,
    >,
    pub tcl_FSFileAttrsGet: Option::<
        unsafe extern "C" fn(
            *mut Tcl_Interp,
            libc::c_int,
            *mut Tcl_Obj,
            *mut *mut Tcl_Obj,
        ) -> libc::c_int,
    >,
    pub tcl_FSFileAttrsSet: Option::<
        unsafe extern "C" fn(
            *mut Tcl_Interp,
            libc::c_int,
            *mut Tcl_Obj,
            *mut Tcl_Obj,
        ) -> libc::c_int,
    >,
    pub tcl_FSFileAttrStrings: Option::<
        unsafe extern "C" fn(
            *mut Tcl_Obj,
            *mut *mut Tcl_Obj,
        ) -> *const *const libc::c_char,
    >,
    pub tcl_FSStat: Option::<
        unsafe extern "C" fn(*mut Tcl_Obj, *mut Tcl_StatBuf) -> libc::c_int,
    >,
    pub tcl_FSAccess: Option::<
        unsafe extern "C" fn(*mut Tcl_Obj, libc::c_int) -> libc::c_int,
    >,
    pub tcl_FSOpenFileChannel: Option::<
        unsafe extern "C" fn(
            *mut Tcl_Interp,
            *mut Tcl_Obj,
            *const libc::c_char,
            libc::c_int,
        ) -> Tcl_Channel,
    >,
    pub tcl_FSGetCwd: Option::<unsafe extern "C" fn(*mut Tcl_Interp) -> *mut Tcl_Obj>,
    pub tcl_FSChdir: Option::<unsafe extern "C" fn(*mut Tcl_Obj) -> libc::c_int>,
    pub tcl_FSConvertToPathType: Option::<
        unsafe extern "C" fn(*mut Tcl_Interp, *mut Tcl_Obj) -> libc::c_int,
    >,
    pub tcl_FSJoinPath: Option::<
        unsafe extern "C" fn(*mut Tcl_Obj, libc::c_int) -> *mut Tcl_Obj,
    >,
    pub tcl_FSSplitPath: Option::<
        unsafe extern "C" fn(*mut Tcl_Obj, *mut libc::c_int) -> *mut Tcl_Obj,
    >,
    pub tcl_FSEqualPaths: Option::<
        unsafe extern "C" fn(*mut Tcl_Obj, *mut Tcl_Obj) -> libc::c_int,
    >,
    pub tcl_FSGetNormalizedPath: Option::<
        unsafe extern "C" fn(*mut Tcl_Interp, *mut Tcl_Obj) -> *mut Tcl_Obj,
    >,
    pub tcl_FSJoinToPath: Option::<
        unsafe extern "C" fn(
            *mut Tcl_Obj,
            libc::c_int,
            *const *mut Tcl_Obj,
        ) -> *mut Tcl_Obj,
    >,
    pub tcl_FSGetInternalRep: Option::<
        unsafe extern "C" fn(*mut Tcl_Obj, *const Tcl_Filesystem) -> ClientData,
    >,
    pub tcl_FSGetTranslatedPath: Option::<
        unsafe extern "C" fn(*mut Tcl_Interp, *mut Tcl_Obj) -> *mut Tcl_Obj,
    >,
    pub tcl_FSEvalFile: Option::<
        unsafe extern "C" fn(*mut Tcl_Interp, *mut Tcl_Obj) -> libc::c_int,
    >,
    pub tcl_FSNewNativePath: Option::<
        unsafe extern "C" fn(*const Tcl_Filesystem, ClientData) -> *mut Tcl_Obj,
    >,
    pub tcl_FSGetNativePath: Option::<
        unsafe extern "C" fn(*mut Tcl_Obj) -> *const libc::c_void,
    >,
    pub tcl_FSFileSystemInfo: Option::<
        unsafe extern "C" fn(*mut Tcl_Obj) -> *mut Tcl_Obj,
    >,
    pub tcl_FSPathSeparator: Option::<
        unsafe extern "C" fn(*mut Tcl_Obj) -> *mut Tcl_Obj,
    >,
    pub tcl_FSListVolumes: Option::<unsafe extern "C" fn() -> *mut Tcl_Obj>,
    pub tcl_FSRegister: Option::<
        unsafe extern "C" fn(ClientData, *const Tcl_Filesystem) -> libc::c_int,
    >,
    pub tcl_FSUnregister: Option::<
        unsafe extern "C" fn(*const Tcl_Filesystem) -> libc::c_int,
    >,
    pub tcl_FSData: Option::<unsafe extern "C" fn(*const Tcl_Filesystem) -> ClientData>,
    pub tcl_FSGetTranslatedStringPath: Option::<
        unsafe extern "C" fn(*mut Tcl_Interp, *mut Tcl_Obj) -> *const libc::c_char,
    >,
    pub tcl_FSGetFileSystemForPath: Option::<
        unsafe extern "C" fn(*mut Tcl_Obj) -> *const Tcl_Filesystem,
    >,
    pub tcl_FSGetPathType: Option::<unsafe extern "C" fn(*mut Tcl_Obj) -> Tcl_PathType>,
    pub tcl_OutputBuffered: Option::<unsafe extern "C" fn(Tcl_Channel) -> libc::c_int>,
    pub tcl_FSMountsChanged: Option::<unsafe extern "C" fn(*const Tcl_Filesystem) -> ()>,
    pub tcl_EvalTokensStandard: Option::<
        unsafe extern "C" fn(*mut Tcl_Interp, *mut Tcl_Token, libc::c_int) -> libc::c_int,
    >,
    pub tcl_GetTime: Option::<unsafe extern "C" fn(*mut Tcl_Time) -> ()>,
    pub tcl_CreateObjTrace: Option::<
        unsafe extern "C" fn(
            *mut Tcl_Interp,
            libc::c_int,
            libc::c_int,
            Option::<Tcl_CmdObjTraceProc>,
            ClientData,
            Option::<Tcl_CmdObjTraceDeleteProc>,
        ) -> Tcl_Trace,
    >,
    pub tcl_GetCommandInfoFromToken: Option::<
        unsafe extern "C" fn(Tcl_Command, *mut Tcl_CmdInfo) -> libc::c_int,
    >,
    pub tcl_SetCommandInfoFromToken: Option::<
        unsafe extern "C" fn(Tcl_Command, *const Tcl_CmdInfo) -> libc::c_int,
    >,
    pub tcl_DbNewWideIntObj: Option::<
        unsafe extern "C" fn(
            Tcl_WideInt,
            *const libc::c_char,
            libc::c_int,
        ) -> *mut Tcl_Obj,
    >,
    pub tcl_GetWideIntFromObj: Option::<
        unsafe extern "C" fn(
            *mut Tcl_Interp,
            *mut Tcl_Obj,
            *mut Tcl_WideInt,
        ) -> libc::c_int,
    >,
    pub tcl_NewWideIntObj: Option::<unsafe extern "C" fn(Tcl_WideInt) -> *mut Tcl_Obj>,
    pub tcl_SetWideIntObj: Option::<
        unsafe extern "C" fn(*mut Tcl_Obj, Tcl_WideInt) -> (),
    >,
    pub tcl_AllocStatBuf: Option::<unsafe extern "C" fn() -> *mut Tcl_StatBuf>,
    pub tcl_Seek: Option::<
        unsafe extern "C" fn(Tcl_Channel, Tcl_WideInt, libc::c_int) -> Tcl_WideInt,
    >,
    pub tcl_Tell: Option::<unsafe extern "C" fn(Tcl_Channel) -> Tcl_WideInt>,
    pub tcl_ChannelWideSeekProc: Option::<
        unsafe extern "C" fn(*const Tcl_ChannelType) -> Option::<Tcl_DriverWideSeekProc>,
    >,
    pub tcl_DictObjPut: Option::<
        unsafe extern "C" fn(
            *mut Tcl_Interp,
            *mut Tcl_Obj,
            *mut Tcl_Obj,
            *mut Tcl_Obj,
        ) -> libc::c_int,
    >,
    pub tcl_DictObjGet: Option::<
        unsafe extern "C" fn(
            *mut Tcl_Interp,
            *mut Tcl_Obj,
            *mut Tcl_Obj,
            *mut *mut Tcl_Obj,
        ) -> libc::c_int,
    >,
    pub tcl_DictObjRemove: Option::<
        unsafe extern "C" fn(*mut Tcl_Interp, *mut Tcl_Obj, *mut Tcl_Obj) -> libc::c_int,
    >,
    pub tcl_DictObjSize: Option::<
        unsafe extern "C" fn(
            *mut Tcl_Interp,
            *mut Tcl_Obj,
            *mut libc::c_int,
        ) -> libc::c_int,
    >,
    pub tcl_DictObjFirst: Option::<
        unsafe extern "C" fn(
            *mut Tcl_Interp,
            *mut Tcl_Obj,
            *mut Tcl_DictSearch,
            *mut *mut Tcl_Obj,
            *mut *mut Tcl_Obj,
            *mut libc::c_int,
        ) -> libc::c_int,
    >,
    pub tcl_DictObjNext: Option::<
        unsafe extern "C" fn(
            *mut Tcl_DictSearch,
            *mut *mut Tcl_Obj,
            *mut *mut Tcl_Obj,
            *mut libc::c_int,
        ) -> (),
    >,
    pub tcl_DictObjDone: Option::<unsafe extern "C" fn(*mut Tcl_DictSearch) -> ()>,
    pub tcl_DictObjPutKeyList: Option::<
        unsafe extern "C" fn(
            *mut Tcl_Interp,
            *mut Tcl_Obj,
            libc::c_int,
            *const *mut Tcl_Obj,
            *mut Tcl_Obj,
        ) -> libc::c_int,
    >,
    pub tcl_DictObjRemoveKeyList: Option::<
        unsafe extern "C" fn(
            *mut Tcl_Interp,
            *mut Tcl_Obj,
            libc::c_int,
            *const *mut Tcl_Obj,
        ) -> libc::c_int,
    >,
    pub tcl_NewDictObj: Option::<unsafe extern "C" fn() -> *mut Tcl_Obj>,
    pub tcl_DbNewDictObj: Option::<
        unsafe extern "C" fn(*const libc::c_char, libc::c_int) -> *mut Tcl_Obj,
    >,
    pub tcl_RegisterConfig: Option::<
        unsafe extern "C" fn(
            *mut Tcl_Interp,
            *const libc::c_char,
            *const Tcl_Config,
            *const libc::c_char,
        ) -> (),
    >,
    pub tcl_CreateNamespace: Option::<
        unsafe extern "C" fn(
            *mut Tcl_Interp,
            *const libc::c_char,
            ClientData,
            Option::<Tcl_NamespaceDeleteProc>,
        ) -> *mut Tcl_Namespace,
    >,
    pub tcl_DeleteNamespace: Option::<unsafe extern "C" fn(*mut Tcl_Namespace) -> ()>,
    pub tcl_AppendExportList: Option::<
        unsafe extern "C" fn(
            *mut Tcl_Interp,
            *mut Tcl_Namespace,
            *mut Tcl_Obj,
        ) -> libc::c_int,
    >,
    pub tcl_Export: Option::<
        unsafe extern "C" fn(
            *mut Tcl_Interp,
            *mut Tcl_Namespace,
            *const libc::c_char,
            libc::c_int,
        ) -> libc::c_int,
    >,
    pub tcl_Import: Option::<
        unsafe extern "C" fn(
            *mut Tcl_Interp,
            *mut Tcl_Namespace,
            *const libc::c_char,
            libc::c_int,
        ) -> libc::c_int,
    >,
    pub tcl_ForgetImport: Option::<
        unsafe extern "C" fn(
            *mut Tcl_Interp,
            *mut Tcl_Namespace,
            *const libc::c_char,
        ) -> libc::c_int,
    >,
    pub tcl_GetCurrentNamespace: Option::<
        unsafe extern "C" fn(*mut Tcl_Interp) -> *mut Tcl_Namespace,
    >,
    pub tcl_GetGlobalNamespace: Option::<
        unsafe extern "C" fn(*mut Tcl_Interp) -> *mut Tcl_Namespace,
    >,
    pub tcl_FindNamespace: Option::<
        unsafe extern "C" fn(
            *mut Tcl_Interp,
            *const libc::c_char,
            *mut Tcl_Namespace,
            libc::c_int,
        ) -> *mut Tcl_Namespace,
    >,
    pub tcl_FindCommand: Option::<
        unsafe extern "C" fn(
            *mut Tcl_Interp,
            *const libc::c_char,
            *mut Tcl_Namespace,
            libc::c_int,
        ) -> Tcl_Command,
    >,
    pub tcl_GetCommandFromObj: Option::<
        unsafe extern "C" fn(*mut Tcl_Interp, *mut Tcl_Obj) -> Tcl_Command,
    >,
    pub tcl_GetCommandFullName: Option::<
        unsafe extern "C" fn(*mut Tcl_Interp, Tcl_Command, *mut Tcl_Obj) -> (),
    >,
    pub tcl_FSEvalFileEx: Option::<
        unsafe extern "C" fn(
            *mut Tcl_Interp,
            *mut Tcl_Obj,
            *const libc::c_char,
        ) -> libc::c_int,
    >,
    pub tcl_SetExitProc: Option::<
        unsafe extern "C" fn(Option::<Tcl_ExitProc>) -> Option::<Tcl_ExitProc>,
    >,
    pub tcl_LimitAddHandler: Option::<
        unsafe extern "C" fn(
            *mut Tcl_Interp,
            libc::c_int,
            Option::<Tcl_LimitHandlerProc>,
            ClientData,
            Option::<Tcl_LimitHandlerDeleteProc>,
        ) -> (),
    >,
    pub tcl_LimitRemoveHandler: Option::<
        unsafe extern "C" fn(
            *mut Tcl_Interp,
            libc::c_int,
            Option::<Tcl_LimitHandlerProc>,
            ClientData,
        ) -> (),
    >,
    pub tcl_LimitReady: Option::<unsafe extern "C" fn(*mut Tcl_Interp) -> libc::c_int>,
    pub tcl_LimitCheck: Option::<unsafe extern "C" fn(*mut Tcl_Interp) -> libc::c_int>,
    pub tcl_LimitExceeded: Option::<
        unsafe extern "C" fn(*mut Tcl_Interp) -> libc::c_int,
    >,
    pub tcl_LimitSetCommands: Option::<
        unsafe extern "C" fn(*mut Tcl_Interp, libc::c_int) -> (),
    >,
    pub tcl_LimitSetTime: Option::<
        unsafe extern "C" fn(*mut Tcl_Interp, *mut Tcl_Time) -> (),
    >,
    pub tcl_LimitSetGranularity: Option::<
        unsafe extern "C" fn(*mut Tcl_Interp, libc::c_int, libc::c_int) -> (),
    >,
    pub tcl_LimitTypeEnabled: Option::<
        unsafe extern "C" fn(*mut Tcl_Interp, libc::c_int) -> libc::c_int,
    >,
    pub tcl_LimitTypeExceeded: Option::<
        unsafe extern "C" fn(*mut Tcl_Interp, libc::c_int) -> libc::c_int,
    >,
    pub tcl_LimitTypeSet: Option::<
        unsafe extern "C" fn(*mut Tcl_Interp, libc::c_int) -> (),
    >,
    pub tcl_LimitTypeReset: Option::<
        unsafe extern "C" fn(*mut Tcl_Interp, libc::c_int) -> (),
    >,
    pub tcl_LimitGetCommands: Option::<
        unsafe extern "C" fn(*mut Tcl_Interp) -> libc::c_int,
    >,
    pub tcl_LimitGetTime: Option::<
        unsafe extern "C" fn(*mut Tcl_Interp, *mut Tcl_Time) -> (),
    >,
    pub tcl_LimitGetGranularity: Option::<
        unsafe extern "C" fn(*mut Tcl_Interp, libc::c_int) -> libc::c_int,
    >,
    pub tcl_SaveInterpState: Option::<
        unsafe extern "C" fn(*mut Tcl_Interp, libc::c_int) -> Tcl_InterpState,
    >,
    pub tcl_RestoreInterpState: Option::<
        unsafe extern "C" fn(*mut Tcl_Interp, Tcl_InterpState) -> libc::c_int,
    >,
    pub tcl_DiscardInterpState: Option::<unsafe extern "C" fn(Tcl_InterpState) -> ()>,
    pub tcl_SetReturnOptions: Option::<
        unsafe extern "C" fn(*mut Tcl_Interp, *mut Tcl_Obj) -> libc::c_int,
    >,
    pub tcl_GetReturnOptions: Option::<
        unsafe extern "C" fn(*mut Tcl_Interp, libc::c_int) -> *mut Tcl_Obj,
    >,
    pub tcl_IsEnsemble: Option::<unsafe extern "C" fn(Tcl_Command) -> libc::c_int>,
    pub tcl_CreateEnsemble: Option::<
        unsafe extern "C" fn(
            *mut Tcl_Interp,
            *const libc::c_char,
            *mut Tcl_Namespace,
            libc::c_int,
        ) -> Tcl_Command,
    >,
    pub tcl_FindEnsemble: Option::<
        unsafe extern "C" fn(*mut Tcl_Interp, *mut Tcl_Obj, libc::c_int) -> Tcl_Command,
    >,
    pub tcl_SetEnsembleSubcommandList: Option::<
        unsafe extern "C" fn(*mut Tcl_Interp, Tcl_Command, *mut Tcl_Obj) -> libc::c_int,
    >,
    pub tcl_SetEnsembleMappingDict: Option::<
        unsafe extern "C" fn(*mut Tcl_Interp, Tcl_Command, *mut Tcl_Obj) -> libc::c_int,
    >,
    pub tcl_SetEnsembleUnknownHandler: Option::<
        unsafe extern "C" fn(*mut Tcl_Interp, Tcl_Command, *mut Tcl_Obj) -> libc::c_int,
    >,
    pub tcl_SetEnsembleFlags: Option::<
        unsafe extern "C" fn(*mut Tcl_Interp, Tcl_Command, libc::c_int) -> libc::c_int,
    >,
    pub tcl_GetEnsembleSubcommandList: Option::<
        unsafe extern "C" fn(
            *mut Tcl_Interp,
            Tcl_Command,
            *mut *mut Tcl_Obj,
        ) -> libc::c_int,
    >,
    pub tcl_GetEnsembleMappingDict: Option::<
        unsafe extern "C" fn(
            *mut Tcl_Interp,
            Tcl_Command,
            *mut *mut Tcl_Obj,
        ) -> libc::c_int,
    >,
    pub tcl_GetEnsembleUnknownHandler: Option::<
        unsafe extern "C" fn(
            *mut Tcl_Interp,
            Tcl_Command,
            *mut *mut Tcl_Obj,
        ) -> libc::c_int,
    >,
    pub tcl_GetEnsembleFlags: Option::<
        unsafe extern "C" fn(
            *mut Tcl_Interp,
            Tcl_Command,
            *mut libc::c_int,
        ) -> libc::c_int,
    >,
    pub tcl_GetEnsembleNamespace: Option::<
        unsafe extern "C" fn(
            *mut Tcl_Interp,
            Tcl_Command,
            *mut *mut Tcl_Namespace,
        ) -> libc::c_int,
    >,
    pub tcl_SetTimeProc: Option::<
        unsafe extern "C" fn(
            Option::<Tcl_GetTimeProc>,
            Option::<Tcl_ScaleTimeProc>,
            ClientData,
        ) -> (),
    >,
    pub tcl_QueryTimeProc: Option::<
        unsafe extern "C" fn(
            *mut Option::<Tcl_GetTimeProc>,
            *mut Option::<Tcl_ScaleTimeProc>,
            *mut ClientData,
        ) -> (),
    >,
    pub tcl_ChannelThreadActionProc: Option::<
        unsafe extern "C" fn(
            *const Tcl_ChannelType,
        ) -> Option::<Tcl_DriverThreadActionProc>,
    >,
    pub tcl_NewBignumObj: Option::<unsafe extern "C" fn(*mut mp_int) -> *mut Tcl_Obj>,
    pub tcl_DbNewBignumObj: Option::<
        unsafe extern "C" fn(
            *mut mp_int,
            *const libc::c_char,
            libc::c_int,
        ) -> *mut Tcl_Obj,
    >,
    pub tcl_SetBignumObj: Option::<
        unsafe extern "C" fn(*mut Tcl_Obj, *mut mp_int) -> (),
    >,
    pub tcl_GetBignumFromObj: Option::<
        unsafe extern "C" fn(*mut Tcl_Interp, *mut Tcl_Obj, *mut mp_int) -> libc::c_int,
    >,
    pub tcl_TakeBignumFromObj: Option::<
        unsafe extern "C" fn(*mut Tcl_Interp, *mut Tcl_Obj, *mut mp_int) -> libc::c_int,
    >,
    pub tcl_TruncateChannel: Option::<
        unsafe extern "C" fn(Tcl_Channel, Tcl_WideInt) -> libc::c_int,
    >,
    pub tcl_ChannelTruncateProc: Option::<
        unsafe extern "C" fn(*const Tcl_ChannelType) -> Option::<Tcl_DriverTruncateProc>,
    >,
    pub tcl_SetChannelErrorInterp: Option::<
        unsafe extern "C" fn(*mut Tcl_Interp, *mut Tcl_Obj) -> (),
    >,
    pub tcl_GetChannelErrorInterp: Option::<
        unsafe extern "C" fn(*mut Tcl_Interp, *mut *mut Tcl_Obj) -> (),
    >,
    pub tcl_SetChannelError: Option::<
        unsafe extern "C" fn(Tcl_Channel, *mut Tcl_Obj) -> (),
    >,
    pub tcl_GetChannelError: Option::<
        unsafe extern "C" fn(Tcl_Channel, *mut *mut Tcl_Obj) -> (),
    >,
    pub tcl_InitBignumFromDouble: Option::<
        unsafe extern "C" fn(*mut Tcl_Interp, libc::c_double, *mut mp_int) -> libc::c_int,
    >,
    pub tcl_GetNamespaceUnknownHandler: Option::<
        unsafe extern "C" fn(*mut Tcl_Interp, *mut Tcl_Namespace) -> *mut Tcl_Obj,
    >,
    pub tcl_SetNamespaceUnknownHandler: Option::<
        unsafe extern "C" fn(
            *mut Tcl_Interp,
            *mut Tcl_Namespace,
            *mut Tcl_Obj,
        ) -> libc::c_int,
    >,
    pub tcl_GetEncodingFromObj: Option::<
        unsafe extern "C" fn(
            *mut Tcl_Interp,
            *mut Tcl_Obj,
            *mut Tcl_Encoding,
        ) -> libc::c_int,
    >,
    pub tcl_GetEncodingSearchPath: Option::<unsafe extern "C" fn() -> *mut Tcl_Obj>,
    pub tcl_SetEncodingSearchPath: Option::<
        unsafe extern "C" fn(*mut Tcl_Obj) -> libc::c_int,
    >,
    pub tcl_GetEncodingNameFromEnvironment: Option::<
        unsafe extern "C" fn(*mut Tcl_DString) -> *const libc::c_char,
    >,
    pub tcl_PkgRequireProc: Option::<
        unsafe extern "C" fn(
            *mut Tcl_Interp,
            *const libc::c_char,
            libc::c_int,
            *const *mut Tcl_Obj,
            *mut libc::c_void,
        ) -> libc::c_int,
    >,
    pub tcl_AppendObjToErrorInfo: Option::<
        unsafe extern "C" fn(*mut Tcl_Interp, *mut Tcl_Obj) -> (),
    >,
    pub tcl_AppendLimitedToObj: Option::<
        unsafe extern "C" fn(
            *mut Tcl_Obj,
            *const libc::c_char,
            libc::c_int,
            libc::c_int,
            *const libc::c_char,
        ) -> (),
    >,
    pub tcl_Format: Option::<
        unsafe extern "C" fn(
            *mut Tcl_Interp,
            *const libc::c_char,
            libc::c_int,
            *const *mut Tcl_Obj,
        ) -> *mut Tcl_Obj,
    >,
    pub tcl_AppendFormatToObj: Option::<
        unsafe extern "C" fn(
            *mut Tcl_Interp,
            *mut Tcl_Obj,
            *const libc::c_char,
            libc::c_int,
            *const *mut Tcl_Obj,
        ) -> libc::c_int,
    >,
    pub tcl_ObjPrintf: Option::<
        unsafe extern "C" fn(*const libc::c_char, ...) -> *mut Tcl_Obj,
    >,
    pub tcl_AppendPrintfToObj: Option::<
        unsafe extern "C" fn(*mut Tcl_Obj, *const libc::c_char, ...) -> (),
    >,
    pub tcl_CancelEval: Option::<
        unsafe extern "C" fn(
            *mut Tcl_Interp,
            *mut Tcl_Obj,
            ClientData,
            libc::c_int,
        ) -> libc::c_int,
    >,
    pub tcl_Canceled: Option::<
        unsafe extern "C" fn(*mut Tcl_Interp, libc::c_int) -> libc::c_int,
    >,
    pub tcl_CreatePipe: Option::<
        unsafe extern "C" fn(
            *mut Tcl_Interp,
            *mut Tcl_Channel,
            *mut Tcl_Channel,
            libc::c_int,
        ) -> libc::c_int,
    >,
    pub tcl_NRCreateCommand: Option::<
        unsafe extern "C" fn(
            *mut Tcl_Interp,
            *const libc::c_char,
            Option::<Tcl_ObjCmdProc>,
            Option::<Tcl_ObjCmdProc>,
            ClientData,
            Option::<Tcl_CmdDeleteProc>,
        ) -> Tcl_Command,
    >,
    pub tcl_NREvalObj: Option::<
        unsafe extern "C" fn(*mut Tcl_Interp, *mut Tcl_Obj, libc::c_int) -> libc::c_int,
    >,
    pub tcl_NREvalObjv: Option::<
        unsafe extern "C" fn(
            *mut Tcl_Interp,
            libc::c_int,
            *const *mut Tcl_Obj,
            libc::c_int,
        ) -> libc::c_int,
    >,
    pub tcl_NRCmdSwap: Option::<
        unsafe extern "C" fn(
            *mut Tcl_Interp,
            Tcl_Command,
            libc::c_int,
            *const *mut Tcl_Obj,
            libc::c_int,
        ) -> libc::c_int,
    >,
    pub tcl_NRAddCallback: Option::<
        unsafe extern "C" fn(
            *mut Tcl_Interp,
            Option::<Tcl_NRPostProc>,
            ClientData,
            ClientData,
            ClientData,
            ClientData,
        ) -> (),
    >,
    pub tcl_NRCallObjProc: Option::<
        unsafe extern "C" fn(
            *mut Tcl_Interp,
            Option::<Tcl_ObjCmdProc>,
            ClientData,
            libc::c_int,
            *const *mut Tcl_Obj,
        ) -> libc::c_int,
    >,
    pub tcl_GetFSDeviceFromStat: Option::<
        unsafe extern "C" fn(*const Tcl_StatBuf) -> libc::c_uint,
    >,
    pub tcl_GetFSInodeFromStat: Option::<
        unsafe extern "C" fn(*const Tcl_StatBuf) -> libc::c_uint,
    >,
    pub tcl_GetModeFromStat: Option::<
        unsafe extern "C" fn(*const Tcl_StatBuf) -> libc::c_uint,
    >,
    pub tcl_GetLinkCountFromStat: Option::<
        unsafe extern "C" fn(*const Tcl_StatBuf) -> libc::c_int,
    >,
    pub tcl_GetUserIdFromStat: Option::<
        unsafe extern "C" fn(*const Tcl_StatBuf) -> libc::c_int,
    >,
    pub tcl_GetGroupIdFromStat: Option::<
        unsafe extern "C" fn(*const Tcl_StatBuf) -> libc::c_int,
    >,
    pub tcl_GetDeviceTypeFromStat: Option::<
        unsafe extern "C" fn(*const Tcl_StatBuf) -> libc::c_int,
    >,
    pub tcl_GetAccessTimeFromStat: Option::<
        unsafe extern "C" fn(*const Tcl_StatBuf) -> Tcl_WideInt,
    >,
    pub tcl_GetModificationTimeFromStat: Option::<
        unsafe extern "C" fn(*const Tcl_StatBuf) -> Tcl_WideInt,
    >,
    pub tcl_GetChangeTimeFromStat: Option::<
        unsafe extern "C" fn(*const Tcl_StatBuf) -> Tcl_WideInt,
    >,
    pub tcl_GetSizeFromStat: Option::<
        unsafe extern "C" fn(*const Tcl_StatBuf) -> Tcl_WideUInt,
    >,
    pub tcl_GetBlocksFromStat: Option::<
        unsafe extern "C" fn(*const Tcl_StatBuf) -> Tcl_WideUInt,
    >,
    pub tcl_GetBlockSizeFromStat: Option::<
        unsafe extern "C" fn(*const Tcl_StatBuf) -> libc::c_uint,
    >,
    pub tcl_SetEnsembleParameterList: Option::<
        unsafe extern "C" fn(*mut Tcl_Interp, Tcl_Command, *mut Tcl_Obj) -> libc::c_int,
    >,
    pub tcl_GetEnsembleParameterList: Option::<
        unsafe extern "C" fn(
            *mut Tcl_Interp,
            Tcl_Command,
            *mut *mut Tcl_Obj,
        ) -> libc::c_int,
    >,
    pub tcl_ParseArgsObjv: Option::<
        unsafe extern "C" fn(
            *mut Tcl_Interp,
            *const Tcl_ArgvInfo,
            *mut libc::c_int,
            *const *mut Tcl_Obj,
            *mut *mut *mut Tcl_Obj,
        ) -> libc::c_int,
    >,
    pub tcl_GetErrorLine: Option::<unsafe extern "C" fn(*mut Tcl_Interp) -> libc::c_int>,
    pub tcl_SetErrorLine: Option::<
        unsafe extern "C" fn(*mut Tcl_Interp, libc::c_int) -> (),
    >,
    pub tcl_TransferResult: Option::<
        unsafe extern "C" fn(*mut Tcl_Interp, libc::c_int, *mut Tcl_Interp) -> (),
    >,
    pub tcl_InterpActive: Option::<unsafe extern "C" fn(*mut Tcl_Interp) -> libc::c_int>,
    pub tcl_BackgroundException: Option::<
        unsafe extern "C" fn(*mut Tcl_Interp, libc::c_int) -> (),
    >,
    pub tcl_ZlibDeflate: Option::<
        unsafe extern "C" fn(
            *mut Tcl_Interp,
            libc::c_int,
            *mut Tcl_Obj,
            libc::c_int,
            *mut Tcl_Obj,
        ) -> libc::c_int,
    >,
    pub tcl_ZlibInflate: Option::<
        unsafe extern "C" fn(
            *mut Tcl_Interp,
            libc::c_int,
            *mut Tcl_Obj,
            libc::c_int,
            *mut Tcl_Obj,
        ) -> libc::c_int,
    >,
    pub tcl_ZlibCRC32: Option::<
        unsafe extern "C" fn(
            libc::c_uint,
            *const libc::c_uchar,
            libc::c_int,
        ) -> libc::c_uint,
    >,
    pub tcl_ZlibAdler32: Option::<
        unsafe extern "C" fn(
            libc::c_uint,
            *const libc::c_uchar,
            libc::c_int,
        ) -> libc::c_uint,
    >,
    pub tcl_ZlibStreamInit: Option::<
        unsafe extern "C" fn(
            *mut Tcl_Interp,
            libc::c_int,
            libc::c_int,
            libc::c_int,
            *mut Tcl_Obj,
            *mut Tcl_ZlibStream,
        ) -> libc::c_int,
    >,
    pub tcl_ZlibStreamGetCommandName: Option::<
        unsafe extern "C" fn(Tcl_ZlibStream) -> *mut Tcl_Obj,
    >,
    pub tcl_ZlibStreamEof: Option::<unsafe extern "C" fn(Tcl_ZlibStream) -> libc::c_int>,
    pub tcl_ZlibStreamChecksum: Option::<
        unsafe extern "C" fn(Tcl_ZlibStream) -> libc::c_int,
    >,
    pub tcl_ZlibStreamPut: Option::<
        unsafe extern "C" fn(Tcl_ZlibStream, *mut Tcl_Obj, libc::c_int) -> libc::c_int,
    >,
    pub tcl_ZlibStreamGet: Option::<
        unsafe extern "C" fn(Tcl_ZlibStream, *mut Tcl_Obj, libc::c_int) -> libc::c_int,
    >,
    pub tcl_ZlibStreamClose: Option::<
        unsafe extern "C" fn(Tcl_ZlibStream) -> libc::c_int,
    >,
    pub tcl_ZlibStreamReset: Option::<
        unsafe extern "C" fn(Tcl_ZlibStream) -> libc::c_int,
    >,
    pub tcl_SetStartupScript: Option::<
        unsafe extern "C" fn(*mut Tcl_Obj, *const libc::c_char) -> (),
    >,
    pub tcl_GetStartupScript: Option::<
        unsafe extern "C" fn(*mut *const libc::c_char) -> *mut Tcl_Obj,
    >,
    pub tcl_CloseEx: Option::<
        unsafe extern "C" fn(*mut Tcl_Interp, Tcl_Channel, libc::c_int) -> libc::c_int,
    >,
    pub tcl_NRExprObj: Option::<
        unsafe extern "C" fn(*mut Tcl_Interp, *mut Tcl_Obj, *mut Tcl_Obj) -> libc::c_int,
    >,
    pub tcl_NRSubstObj: Option::<
        unsafe extern "C" fn(*mut Tcl_Interp, *mut Tcl_Obj, libc::c_int) -> libc::c_int,
    >,
    pub tcl_LoadFile: Option::<
        unsafe extern "C" fn(
            *mut Tcl_Interp,
            *mut Tcl_Obj,
            *const *const libc::c_char,
            libc::c_int,
            *mut libc::c_void,
            *mut Tcl_LoadHandle,
        ) -> libc::c_int,
    >,
    pub tcl_FindSymbol: Option::<
        unsafe extern "C" fn(
            *mut Tcl_Interp,
            Tcl_LoadHandle,
            *const libc::c_char,
        ) -> *mut libc::c_void,
    >,
    pub tcl_FSUnloadFile: Option::<
        unsafe extern "C" fn(*mut Tcl_Interp, Tcl_LoadHandle) -> libc::c_int,
    >,
    pub tcl_ZlibStreamSetCompressionDictionary: Option::<
        unsafe extern "C" fn(Tcl_ZlibStream, *mut Tcl_Obj) -> (),
    >,
}
pub type sqlite_int64 = libc::c_longlong;
pub type sqlite_uint64 = libc::c_ulonglong;
pub type sqlite3_int64 = sqlite_int64;
pub type sqlite3_uint64 = sqlite_uint64;
pub type sqlite3_destructor_type = Option::<
    unsafe extern "C" fn(*mut libc::c_void) -> (),
>;
pub type u8_0 = libc::c_uchar;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SqliteDb {
    pub db: *mut sqlite3,
    pub interp: *mut Tcl_Interp,
    pub zBusy: *mut libc::c_char,
    pub zCommit: *mut libc::c_char,
    pub zTrace: *mut libc::c_char,
    pub zTraceV2: *mut libc::c_char,
    pub zProfile: *mut libc::c_char,
    pub zProgress: *mut libc::c_char,
    pub zBindFallback: *mut libc::c_char,
    pub zAuth: *mut libc::c_char,
    pub disableAuth: libc::c_int,
    pub zNull: *mut libc::c_char,
    pub pFunc: *mut SqlFunc,
    pub pUpdateHook: *mut Tcl_Obj,
    pub pPreUpdateHook: *mut Tcl_Obj,
    pub pRollbackHook: *mut Tcl_Obj,
    pub pWalHook: *mut Tcl_Obj,
    pub pUnlockNotify: *mut Tcl_Obj,
    pub pCollate: *mut SqlCollate,
    pub rc: libc::c_int,
    pub pCollateNeeded: *mut Tcl_Obj,
    pub stmtList: *mut SqlPreparedStmt,
    pub stmtLast: *mut SqlPreparedStmt,
    pub maxStmt: libc::c_int,
    pub nStmt: libc::c_int,
    pub pIncrblob: *mut IncrblobChannel,
    pub nStep: libc::c_int,
    pub nSort: libc::c_int,
    pub nIndex: libc::c_int,
    pub nVMStep: libc::c_int,
    pub nTransaction: libc::c_int,
    pub openFlags: libc::c_int,
    pub nRef: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct IncrblobChannel {
    pub pBlob: *mut sqlite3_blob,
    pub pDb: *mut SqliteDb,
    pub iSeek: libc::c_int,
    pub channel: Tcl_Channel,
    pub pNext: *mut IncrblobChannel,
    pub pPrev: *mut IncrblobChannel,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SqlPreparedStmt {
    pub pNext: *mut SqlPreparedStmt,
    pub pPrev: *mut SqlPreparedStmt,
    pub pStmt: *mut sqlite3_stmt,
    pub nSql: libc::c_int,
    pub zSql: *const libc::c_char,
    pub nParm: libc::c_int,
    pub apParm: *mut *mut Tcl_Obj,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SqlCollate {
    pub interp: *mut Tcl_Interp,
    pub zScript: *mut libc::c_char,
    pub pNext: *mut SqlCollate,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SqlFunc {
    pub interp: *mut Tcl_Interp,
    pub pScript: *mut Tcl_Obj,
    pub pDb: *mut SqliteDb,
    pub useEvalObjv: libc::c_int,
    pub eType: libc::c_int,
    pub zName: *mut libc::c_char,
    pub pNext: *mut SqlFunc,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct DbEvalContext {
    pub pDb: *mut SqliteDb,
    pub pSql: *mut Tcl_Obj,
    pub zSql: *const libc::c_char,
    pub pPreStmt: *mut SqlPreparedStmt,
    pub nCol: libc::c_int,
    pub evalFlags: libc::c_int,
    pub pArray: *mut Tcl_Obj,
    pub apColName: *mut *mut Tcl_Obj,
}
pub const DB_VERSION: DB_enum = 40;
pub const DB_ROLLBACK_HOOK: DB_enum = 30;
pub const DB_UPDATE_HOOK: DB_enum = 39;
pub const DB_WAL_HOOK: DB_enum = 41;
pub const DB_PREUPDATE: DB_enum = 25;
pub const DB_UNLOCK_NOTIFY: DB_enum = 38;
pub const TTYPE_IMMEDIATE: TTYPE_enum = 2;
pub const TTYPE_EXCLUSIVE: TTYPE_enum = 1;
pub const TTYPE_DEFERRED: TTYPE_enum = 0;
pub type TTYPE_enum = libc::c_uint;
pub const DB_TRANSACTION: DB_enum = 37;
pub const TTYPE_CLOSE: TTYPE_enum_0 = 3;
pub const TTYPE_ROW: TTYPE_enum_0 = 2;
pub const TTYPE_PROFILE: TTYPE_enum_0 = 1;
pub const TTYPE_STMT: TTYPE_enum_0 = 0;
pub type TTYPE_enum_0 = libc::c_uint;
pub const DB_TRACE_V2: DB_enum = 36;
pub const DB_TRACE: DB_enum = 35;
pub const DB_TOTAL_CHANGES: DB_enum = 34;
pub const DB_TIMEOUT: DB_enum = 33;
pub const DB_STATUS: DB_enum = 32;
pub const DB_SERIALIZE: DB_enum = 31;
pub const DB_RESTORE: DB_enum = 29;
pub const DB_REKEY: DB_enum = 28;
pub const DB_PROFILE: DB_enum = 26;
pub const DB_PROGRESS: DB_enum = 27;
pub const DB_LAST_INSERT_ROWID: DB_enum = 22;
pub const DB_NULLVALUE: DB_enum = 23;
pub const DB_INTERRUPT: DB_enum = 21;
pub const DB_INCRBLOB: DB_enum = 20;
pub const DB_FUNCTION: DB_enum = 19;
pub const DB_EVAL: DB_enum = 17;
pub const DB_ONECOLUMN: DB_enum = 24;
pub const DB_EXISTS: DB_enum = 18;
pub const DB_ERROROFFSET: DB_enum = 16;
pub const DB_ERRORCODE: DB_enum = 15;
pub const DB_ENABLE_LOAD_EXTENSION: DB_enum = 14;
pub const DB_DESERIALIZE: DB_enum = 13;
pub const DB_COPY: DB_enum = 12;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct DbConfigChoices {
    pub zName: *const libc::c_char,
    pub op: libc::c_int,
}
pub const DB_CONFIG: DB_enum = 11;
pub const DB_COMPLETE: DB_enum = 10;
pub const DB_COMMIT_HOOK: DB_enum = 9;
pub const DB_COLLATION_NEEDED: DB_enum = 8;
pub const DB_COLLATE: DB_enum = 7;
pub const DB_CLOSE: DB_enum = 6;
pub const DB_CHANGES: DB_enum = 5;
pub const DB_CACHE: DB_enum = 4;
pub const DB_BUSY: DB_enum = 3;
pub const DB_BIND_FALLBACK: DB_enum = 2;
pub const DB_BACKUP: DB_enum = 1;
pub type sqlite3_auth_cb = Option::<
    unsafe extern "C" fn(
        *mut libc::c_void,
        libc::c_int,
        *const libc::c_char,
        *const libc::c_char,
        *const libc::c_char,
        *const libc::c_char,
    ) -> libc::c_int,
>;
pub const DB_AUTHORIZER: DB_enum = 0;
pub type DB_enum = libc::c_uint;
#[inline]
unsafe extern "C" fn atoi(mut __nptr: *const libc::c_char) -> libc::c_int {
    return strtol(
        __nptr,
        0 as *mut libc::c_void as *mut *mut libc::c_char,
        10 as libc::c_int,
    ) as libc::c_int;
}
unsafe extern "C" fn strlen30(mut z: *const libc::c_char) -> libc::c_int {
    let mut z2: *const libc::c_char = z;
    while *z2 != 0 {
        z2 = z2.offset(1);
    }
    return 0x3fffffff as libc::c_int & z2.offset_from(z) as libc::c_long as libc::c_int;
}
unsafe extern "C" fn closeIncrblobChannels(mut pDb: *mut SqliteDb) {
    let mut p: *mut IncrblobChannel = 0 as *mut IncrblobChannel;
    let mut pNext: *mut IncrblobChannel = 0 as *mut IncrblobChannel;
    p = (*pDb).pIncrblob;
    while !p.is_null() {
        pNext = (*p).pNext;
        ((*tclStubsPtr).tcl_UnregisterChannel)
            .expect("non-null function pointer")((*pDb).interp, (*p).channel);
        p = pNext;
    }
}
unsafe extern "C" fn incrblobClose(
    mut instanceData: ClientData,
    mut interp: *mut Tcl_Interp,
) -> libc::c_int {
    let mut p: *mut IncrblobChannel = instanceData as *mut IncrblobChannel;
    let mut rc: libc::c_int = sqlite3_blob_close((*p).pBlob);
    let mut db: *mut sqlite3 = (*(*p).pDb).db;
    if !((*p).pNext).is_null() {
        let ref mut fresh0 = (*(*p).pNext).pPrev;
        *fresh0 = (*p).pPrev;
    }
    if !((*p).pPrev).is_null() {
        let ref mut fresh1 = (*(*p).pPrev).pNext;
        *fresh1 = (*p).pNext;
    }
    if (*(*p).pDb).pIncrblob == p {
        let ref mut fresh2 = (*(*p).pDb).pIncrblob;
        *fresh2 = (*p).pNext;
    }
    ((*tclStubsPtr).tcl_Free)
        .expect("non-null function pointer")(p as *mut libc::c_char);
    if rc != 0 as libc::c_int {
        ((*tclStubsPtr).tcl_SetResult)
            .expect(
                "non-null function pointer",
            )(
            interp,
            sqlite3_errmsg(db) as *mut libc::c_char,
            ::std::mem::transmute::<
                libc::intptr_t,
                Option::<Tcl_FreeProc>,
            >(1 as libc::c_int as libc::intptr_t),
        );
        return 1 as libc::c_int;
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn incrblobInput(
    mut instanceData: ClientData,
    mut buf: *mut libc::c_char,
    mut bufSize: libc::c_int,
    mut errorCodePtr: *mut libc::c_int,
) -> libc::c_int {
    let mut p: *mut IncrblobChannel = instanceData as *mut IncrblobChannel;
    let mut nRead: libc::c_int = bufSize;
    let mut nBlob: libc::c_int = 0;
    let mut rc: libc::c_int = 0;
    nBlob = sqlite3_blob_bytes((*p).pBlob);
    if (*p).iSeek + nRead > nBlob {
        nRead = nBlob - (*p).iSeek;
    }
    if nRead <= 0 as libc::c_int {
        return 0 as libc::c_int;
    }
    rc = sqlite3_blob_read((*p).pBlob, buf as *mut libc::c_void, nRead, (*p).iSeek);
    if rc != 0 as libc::c_int {
        *errorCodePtr = rc;
        return -(1 as libc::c_int);
    }
    (*p).iSeek += nRead;
    return nRead;
}
unsafe extern "C" fn incrblobOutput(
    mut instanceData: ClientData,
    mut buf: *const libc::c_char,
    mut toWrite: libc::c_int,
    mut errorCodePtr: *mut libc::c_int,
) -> libc::c_int {
    let mut p: *mut IncrblobChannel = instanceData as *mut IncrblobChannel;
    let mut nWrite: libc::c_int = toWrite;
    let mut nBlob: libc::c_int = 0;
    let mut rc: libc::c_int = 0;
    nBlob = sqlite3_blob_bytes((*p).pBlob);
    if (*p).iSeek + nWrite > nBlob {
        *errorCodePtr = 22 as libc::c_int;
        return -(1 as libc::c_int);
    }
    if nWrite <= 0 as libc::c_int {
        return 0 as libc::c_int;
    }
    rc = sqlite3_blob_write((*p).pBlob, buf as *mut libc::c_void, nWrite, (*p).iSeek);
    if rc != 0 as libc::c_int {
        *errorCodePtr = 5 as libc::c_int;
        return -(1 as libc::c_int);
    }
    (*p).iSeek += nWrite;
    return nWrite;
}
unsafe extern "C" fn incrblobSeek(
    mut instanceData: ClientData,
    mut offset: libc::c_long,
    mut seekMode: libc::c_int,
    mut errorCodePtr: *mut libc::c_int,
) -> libc::c_int {
    let mut p: *mut IncrblobChannel = instanceData as *mut IncrblobChannel;
    match seekMode {
        0 => {
            (*p).iSeek = offset as libc::c_int;
        }
        1 => {
            let ref mut fresh3 = (*p).iSeek;
            *fresh3 = (*fresh3 as libc::c_long + offset) as libc::c_int;
        }
        2 => {
            (*p)
                .iSeek = (sqlite3_blob_bytes((*p).pBlob) as libc::c_long + offset)
                as libc::c_int;
        }
        _ => {}
    }
    return (*p).iSeek;
}
unsafe extern "C" fn incrblobWatch(mut instanceData: ClientData, mut mode: libc::c_int) {}
unsafe extern "C" fn incrblobHandle(
    mut instanceData: ClientData,
    mut dir: libc::c_int,
    mut hPtr: *mut ClientData,
) -> libc::c_int {
    return 1 as libc::c_int;
}
static mut IncrblobChannelType: Tcl_ChannelType = unsafe {
    {
        let mut init = Tcl_ChannelType {
            typeName: b"incrblob\0" as *const u8 as *const libc::c_char,
            version: 0x2 as libc::c_int as Tcl_ChannelTypeVersion,
            closeProc: Some(
                incrblobClose
                    as unsafe extern "C" fn(ClientData, *mut Tcl_Interp) -> libc::c_int,
            ),
            inputProc: Some(
                incrblobInput
                    as unsafe extern "C" fn(
                        ClientData,
                        *mut libc::c_char,
                        libc::c_int,
                        *mut libc::c_int,
                    ) -> libc::c_int,
            ),
            outputProc: Some(
                incrblobOutput
                    as unsafe extern "C" fn(
                        ClientData,
                        *const libc::c_char,
                        libc::c_int,
                        *mut libc::c_int,
                    ) -> libc::c_int,
            ),
            seekProc: Some(
                incrblobSeek
                    as unsafe extern "C" fn(
                        ClientData,
                        libc::c_long,
                        libc::c_int,
                        *mut libc::c_int,
                    ) -> libc::c_int,
            ),
            setOptionProc: None,
            getOptionProc: None,
            watchProc: Some(
                incrblobWatch as unsafe extern "C" fn(ClientData, libc::c_int) -> (),
            ),
            getHandleProc: Some(
                incrblobHandle
                    as unsafe extern "C" fn(
                        ClientData,
                        libc::c_int,
                        *mut ClientData,
                    ) -> libc::c_int,
            ),
            close2Proc: None,
            blockModeProc: None,
            flushProc: None,
            handlerProc: None,
            wideSeekProc: None,
            threadActionProc: None,
            truncateProc: None,
        };
        init
    }
};
unsafe extern "C" fn createIncrblobChannel(
    mut interp: *mut Tcl_Interp,
    mut pDb: *mut SqliteDb,
    mut zDb: *const libc::c_char,
    mut zTable: *const libc::c_char,
    mut zColumn: *const libc::c_char,
    mut iRow: sqlite_int64,
    mut isReadonly: libc::c_int,
) -> libc::c_int {
    let mut p: *mut IncrblobChannel = 0 as *mut IncrblobChannel;
    let mut db: *mut sqlite3 = (*pDb).db;
    let mut pBlob: *mut sqlite3_blob = 0 as *mut sqlite3_blob;
    let mut rc: libc::c_int = 0;
    let mut flags: libc::c_int = (1 as libc::c_int) << 1 as libc::c_int
        | (if isReadonly != 0 {
            0 as libc::c_int
        } else {
            (1 as libc::c_int) << 2 as libc::c_int
        });
    static mut count: libc::c_int = 0 as libc::c_int;
    let mut zChannel: [libc::c_char; 64] = [0; 64];
    rc = sqlite3_blob_open(
        db,
        zDb,
        zTable,
        zColumn,
        iRow,
        (isReadonly == 0) as libc::c_int,
        &mut pBlob,
    );
    if rc != 0 as libc::c_int {
        ((*tclStubsPtr).tcl_SetResult)
            .expect(
                "non-null function pointer",
            )(
            interp,
            sqlite3_errmsg((*pDb).db) as *mut libc::c_char,
            ::std::mem::transmute::<
                libc::intptr_t,
                Option::<Tcl_FreeProc>,
            >(1 as libc::c_int as libc::intptr_t),
        );
        return 1 as libc::c_int;
    }
    p = ((*tclStubsPtr).tcl_Alloc)
        .expect(
            "non-null function pointer",
        )(::std::mem::size_of::<IncrblobChannel>() as libc::c_ulong as libc::c_uint)
        as *mut IncrblobChannel;
    (*p).iSeek = 0 as libc::c_int;
    let ref mut fresh4 = (*p).pBlob;
    *fresh4 = pBlob;
    count += 1;
    sqlite3_snprintf(
        ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong as libc::c_int,
        zChannel.as_mut_ptr(),
        b"incrblob_%d\0" as *const u8 as *const libc::c_char,
        count,
    );
    let ref mut fresh5 = (*p).channel;
    *fresh5 = ((*tclStubsPtr).tcl_CreateChannel)
        .expect(
            "non-null function pointer",
        )(&mut IncrblobChannelType, zChannel.as_mut_ptr(), p as ClientData, flags);
    ((*tclStubsPtr).tcl_RegisterChannel)
        .expect("non-null function pointer")(interp, (*p).channel);
    let ref mut fresh6 = (*p).pNext;
    *fresh6 = (*pDb).pIncrblob;
    let ref mut fresh7 = (*p).pPrev;
    *fresh7 = 0 as *mut IncrblobChannel;
    if !((*p).pNext).is_null() {
        let ref mut fresh8 = (*(*p).pNext).pPrev;
        *fresh8 = p;
    }
    let ref mut fresh9 = (*pDb).pIncrblob;
    *fresh9 = p;
    let ref mut fresh10 = (*p).pDb;
    *fresh10 = pDb;
    ((*tclStubsPtr).tcl_SetResult)
        .expect(
            "non-null function pointer",
        )(
        interp,
        ((*tclStubsPtr).tcl_GetChannelName)
            .expect("non-null function pointer")((*p).channel) as *mut libc::c_char,
        ::std::mem::transmute::<
            libc::intptr_t,
            Option::<Tcl_FreeProc>,
        >(1 as libc::c_int as libc::intptr_t),
    );
    return 0 as libc::c_int;
}
unsafe extern "C" fn safeToUseEvalObjv(
    mut interp: *mut Tcl_Interp,
    mut pCmd: *mut Tcl_Obj,
) -> libc::c_int {
    let mut z: *const libc::c_char = 0 as *const libc::c_char;
    let mut n: libc::c_int = 0;
    z = ((*tclStubsPtr).tcl_GetStringFromObj)
        .expect("non-null function pointer")(pCmd, &mut n);
    loop {
        let fresh11 = n;
        n = n - 1;
        if !(fresh11 > 0 as libc::c_int) {
            break;
        }
        let fresh12 = z;
        z = z.offset(1);
        let mut c: libc::c_int = *fresh12 as libc::c_int;
        if c == '$' as i32 || c == '[' as i32 || c == ';' as i32 {
            return 0 as libc::c_int;
        }
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn findSqlFunc(
    mut pDb: *mut SqliteDb,
    mut zName: *const libc::c_char,
) -> *mut SqlFunc {
    let mut p: *mut SqlFunc = 0 as *mut SqlFunc;
    let mut pNew: *mut SqlFunc = 0 as *mut SqlFunc;
    let mut nName: libc::c_int = strlen30(zName);
    pNew = ((*tclStubsPtr).tcl_Alloc)
        .expect(
            "non-null function pointer",
        )(
        (::std::mem::size_of::<SqlFunc>() as libc::c_ulong)
            .wrapping_add(nName as libc::c_ulong)
            .wrapping_add(1 as libc::c_int as libc::c_ulong) as libc::c_uint,
    ) as *mut SqlFunc;
    let ref mut fresh13 = (*pNew).zName;
    *fresh13 = &mut *pNew.offset(1 as libc::c_int as isize) as *mut SqlFunc
        as *mut libc::c_char;
    memcpy(
        (*pNew).zName as *mut libc::c_void,
        zName as *const libc::c_void,
        (nName + 1 as libc::c_int) as libc::c_ulong,
    );
    p = (*pDb).pFunc;
    while !p.is_null() {
        if sqlite3_stricmp((*p).zName, (*pNew).zName) == 0 as libc::c_int {
            ((*tclStubsPtr).tcl_Free)
                .expect("non-null function pointer")(pNew as *mut libc::c_char);
            return p;
        }
        p = (*p).pNext;
    }
    let ref mut fresh14 = (*pNew).interp;
    *fresh14 = (*pDb).interp;
    let ref mut fresh15 = (*pNew).pDb;
    *fresh15 = pDb;
    let ref mut fresh16 = (*pNew).pScript;
    *fresh16 = 0 as *mut Tcl_Obj;
    let ref mut fresh17 = (*pNew).pNext;
    *fresh17 = (*pDb).pFunc;
    let ref mut fresh18 = (*pDb).pFunc;
    *fresh18 = pNew;
    return pNew;
}
unsafe extern "C" fn dbFreeStmt(mut pStmt: *mut SqlPreparedStmt) {
    sqlite3_finalize((*pStmt).pStmt);
    ((*tclStubsPtr).tcl_Free)
        .expect("non-null function pointer")(pStmt as *mut libc::c_char);
}
unsafe extern "C" fn flushStmtCache(mut pDb: *mut SqliteDb) {
    let mut pPreStmt: *mut SqlPreparedStmt = 0 as *mut SqlPreparedStmt;
    let mut pNext: *mut SqlPreparedStmt = 0 as *mut SqlPreparedStmt;
    pPreStmt = (*pDb).stmtList;
    while !pPreStmt.is_null() {
        pNext = (*pPreStmt).pNext;
        dbFreeStmt(pPreStmt);
        pPreStmt = pNext;
    }
    (*pDb).nStmt = 0 as libc::c_int;
    let ref mut fresh19 = (*pDb).stmtLast;
    *fresh19 = 0 as *mut SqlPreparedStmt;
    let ref mut fresh20 = (*pDb).stmtList;
    *fresh20 = 0 as *mut SqlPreparedStmt;
}
unsafe extern "C" fn addDatabaseRef(mut pDb: *mut SqliteDb) {
    let ref mut fresh21 = (*pDb).nRef;
    *fresh21 += 1;
}
unsafe extern "C" fn delDatabaseRef(mut pDb: *mut SqliteDb) {
    let ref mut fresh22 = (*pDb).nRef;
    *fresh22 -= 1;
    if (*pDb).nRef == 0 as libc::c_int {
        flushStmtCache(pDb);
        closeIncrblobChannels(pDb);
        sqlite3_close((*pDb).db);
        while !((*pDb).pFunc).is_null() {
            let mut pFunc: *mut SqlFunc = (*pDb).pFunc;
            let ref mut fresh23 = (*pDb).pFunc;
            *fresh23 = (*pFunc).pNext;
            let mut _objPtr: *mut Tcl_Obj = (*pFunc).pScript;
            let ref mut fresh24 = (*_objPtr).refCount;
            let fresh25 = *fresh24;
            *fresh24 = *fresh24 - 1;
            if fresh25 <= 1 as libc::c_int {
                ((*tclStubsPtr).tclFreeObj).expect("non-null function pointer")(_objPtr);
            }
            ((*tclStubsPtr).tcl_Free)
                .expect("non-null function pointer")(pFunc as *mut libc::c_char);
        }
        while !((*pDb).pCollate).is_null() {
            let mut pCollate: *mut SqlCollate = (*pDb).pCollate;
            let ref mut fresh26 = (*pDb).pCollate;
            *fresh26 = (*pCollate).pNext;
            ((*tclStubsPtr).tcl_Free)
                .expect("non-null function pointer")(pCollate as *mut libc::c_char);
        }
        if !((*pDb).zBusy).is_null() {
            ((*tclStubsPtr).tcl_Free).expect("non-null function pointer")((*pDb).zBusy);
        }
        if !((*pDb).zTrace).is_null() {
            ((*tclStubsPtr).tcl_Free).expect("non-null function pointer")((*pDb).zTrace);
        }
        if !((*pDb).zTraceV2).is_null() {
            ((*tclStubsPtr).tcl_Free)
                .expect("non-null function pointer")((*pDb).zTraceV2);
        }
        if !((*pDb).zProfile).is_null() {
            ((*tclStubsPtr).tcl_Free)
                .expect("non-null function pointer")((*pDb).zProfile);
        }
        if !((*pDb).zBindFallback).is_null() {
            ((*tclStubsPtr).tcl_Free)
                .expect("non-null function pointer")((*pDb).zBindFallback);
        }
        if !((*pDb).zAuth).is_null() {
            ((*tclStubsPtr).tcl_Free).expect("non-null function pointer")((*pDb).zAuth);
        }
        if !((*pDb).zNull).is_null() {
            ((*tclStubsPtr).tcl_Free).expect("non-null function pointer")((*pDb).zNull);
        }
        if !((*pDb).pUpdateHook).is_null() {
            let mut _objPtr_0: *mut Tcl_Obj = (*pDb).pUpdateHook;
            let ref mut fresh27 = (*_objPtr_0).refCount;
            let fresh28 = *fresh27;
            *fresh27 = *fresh27 - 1;
            if fresh28 <= 1 as libc::c_int {
                ((*tclStubsPtr).tclFreeObj)
                    .expect("non-null function pointer")(_objPtr_0);
            }
        }
        if !((*pDb).pPreUpdateHook).is_null() {
            let mut _objPtr_1: *mut Tcl_Obj = (*pDb).pPreUpdateHook;
            let ref mut fresh29 = (*_objPtr_1).refCount;
            let fresh30 = *fresh29;
            *fresh29 = *fresh29 - 1;
            if fresh30 <= 1 as libc::c_int {
                ((*tclStubsPtr).tclFreeObj)
                    .expect("non-null function pointer")(_objPtr_1);
            }
        }
        if !((*pDb).pRollbackHook).is_null() {
            let mut _objPtr_2: *mut Tcl_Obj = (*pDb).pRollbackHook;
            let ref mut fresh31 = (*_objPtr_2).refCount;
            let fresh32 = *fresh31;
            *fresh31 = *fresh31 - 1;
            if fresh32 <= 1 as libc::c_int {
                ((*tclStubsPtr).tclFreeObj)
                    .expect("non-null function pointer")(_objPtr_2);
            }
        }
        if !((*pDb).pWalHook).is_null() {
            let mut _objPtr_3: *mut Tcl_Obj = (*pDb).pWalHook;
            let ref mut fresh33 = (*_objPtr_3).refCount;
            let fresh34 = *fresh33;
            *fresh33 = *fresh33 - 1;
            if fresh34 <= 1 as libc::c_int {
                ((*tclStubsPtr).tclFreeObj)
                    .expect("non-null function pointer")(_objPtr_3);
            }
        }
        if !((*pDb).pCollateNeeded).is_null() {
            let mut _objPtr_4: *mut Tcl_Obj = (*pDb).pCollateNeeded;
            let ref mut fresh35 = (*_objPtr_4).refCount;
            let fresh36 = *fresh35;
            *fresh35 = *fresh35 - 1;
            if fresh36 <= 1 as libc::c_int {
                ((*tclStubsPtr).tclFreeObj)
                    .expect("non-null function pointer")(_objPtr_4);
            }
        }
        ((*tclStubsPtr).tcl_Free)
            .expect("non-null function pointer")(pDb as *mut libc::c_char);
    }
}
unsafe extern "C" fn DbDeleteCmd(mut db: *mut libc::c_void) {
    let mut pDb: *mut SqliteDb = db as *mut SqliteDb;
    delDatabaseRef(pDb);
}
unsafe extern "C" fn DbBusyHandler(
    mut cd: *mut libc::c_void,
    mut nTries: libc::c_int,
) -> libc::c_int {
    let mut pDb: *mut SqliteDb = cd as *mut SqliteDb;
    let mut rc: libc::c_int = 0;
    let mut zVal: [libc::c_char; 30] = [0; 30];
    sqlite3_snprintf(
        ::std::mem::size_of::<[libc::c_char; 30]>() as libc::c_ulong as libc::c_int,
        zVal.as_mut_ptr(),
        b"%d\0" as *const u8 as *const libc::c_char,
        nTries,
    );
    rc = ((*tclStubsPtr).tcl_VarEval)
        .expect(
            "non-null function pointer",
        )(
        (*pDb).interp,
        (*pDb).zBusy,
        b" \0" as *const u8 as *const libc::c_char,
        zVal.as_mut_ptr(),
        0 as *mut libc::c_char,
    );
    if rc != 0 as libc::c_int
        || atoi(
            ((*tclStubsPtr).tcl_GetStringResult)
                .expect("non-null function pointer")((*pDb).interp),
        ) != 0
    {
        return 0 as libc::c_int;
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn DbProgressHandler(mut cd: *mut libc::c_void) -> libc::c_int {
    let mut pDb: *mut SqliteDb = cd as *mut SqliteDb;
    let mut rc: libc::c_int = 0;
    rc = ((*tclStubsPtr).tcl_Eval)
        .expect("non-null function pointer")((*pDb).interp, (*pDb).zProgress);
    if rc != 0 as libc::c_int
        || atoi(
            ((*tclStubsPtr).tcl_GetStringResult)
                .expect("non-null function pointer")((*pDb).interp),
        ) != 0
    {
        return 1 as libc::c_int;
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn DbTraceHandler(
    mut cd: *mut libc::c_void,
    mut zSql: *const libc::c_char,
) {
    let mut pDb: *mut SqliteDb = cd as *mut SqliteDb;
    let mut str: Tcl_DString = Tcl_DString {
        string: 0 as *mut libc::c_char,
        length: 0,
        spaceAvl: 0,
        staticSpace: [0; 200],
    };
    ((*tclStubsPtr).tcl_DStringInit).expect("non-null function pointer")(&mut str);
    ((*tclStubsPtr).tcl_DStringAppend)
        .expect(
            "non-null function pointer",
        )(&mut str, (*pDb).zTrace, -(1 as libc::c_int));
    ((*tclStubsPtr).tcl_DStringAppendElement)
        .expect("non-null function pointer")(&mut str, zSql);
    ((*tclStubsPtr).tcl_Eval)
        .expect("non-null function pointer")((*pDb).interp, str.string);
    ((*tclStubsPtr).tcl_DStringFree).expect("non-null function pointer")(&mut str);
    ((*tclStubsPtr).tcl_ResetResult).expect("non-null function pointer")((*pDb).interp);
}
unsafe extern "C" fn DbTraceV2Handler(
    mut type_0: libc::c_uint,
    mut cd: *mut libc::c_void,
    mut pd: *mut libc::c_void,
    mut xd: *mut libc::c_void,
) -> libc::c_int {
    let mut pDb: *mut SqliteDb = cd as *mut SqliteDb;
    let mut pCmd: *mut Tcl_Obj = 0 as *mut Tcl_Obj;
    match type_0 {
        1 => {
            let mut pStmt: *mut sqlite3_stmt = pd as *mut sqlite3_stmt;
            let mut zSql: *mut libc::c_char = xd as *mut libc::c_char;
            pCmd = ((*tclStubsPtr).tcl_NewStringObj)
                .expect(
                    "non-null function pointer",
                )((*pDb).zTraceV2, -(1 as libc::c_int));
            let ref mut fresh37 = (*pCmd).refCount;
            *fresh37 += 1;
            ((*tclStubsPtr).tcl_ListObjAppendElement)
                .expect(
                    "non-null function pointer",
                )(
                (*pDb).interp,
                pCmd,
                ((*tclStubsPtr).tcl_NewWideIntObj)
                    .expect("non-null function pointer")(pStmt as Tcl_WideInt),
            );
            ((*tclStubsPtr).tcl_ListObjAppendElement)
                .expect(
                    "non-null function pointer",
                )(
                (*pDb).interp,
                pCmd,
                ((*tclStubsPtr).tcl_NewStringObj)
                    .expect("non-null function pointer")(zSql, -(1 as libc::c_int)),
            );
            ((*tclStubsPtr).tcl_EvalObjEx)
                .expect(
                    "non-null function pointer",
                )((*pDb).interp, pCmd, 0x40000 as libc::c_int);
            let mut _objPtr: *mut Tcl_Obj = pCmd;
            let ref mut fresh38 = (*_objPtr).refCount;
            let fresh39 = *fresh38;
            *fresh38 = *fresh38 - 1;
            if fresh39 <= 1 as libc::c_int {
                ((*tclStubsPtr).tclFreeObj).expect("non-null function pointer")(_objPtr);
            }
            ((*tclStubsPtr).tcl_ResetResult)
                .expect("non-null function pointer")((*pDb).interp);
        }
        2 => {
            let mut pStmt_0: *mut sqlite3_stmt = pd as *mut sqlite3_stmt;
            let mut ns: sqlite3_int64 = *(xd as *mut sqlite3_int64);
            pCmd = ((*tclStubsPtr).tcl_NewStringObj)
                .expect(
                    "non-null function pointer",
                )((*pDb).zTraceV2, -(1 as libc::c_int));
            let ref mut fresh40 = (*pCmd).refCount;
            *fresh40 += 1;
            ((*tclStubsPtr).tcl_ListObjAppendElement)
                .expect(
                    "non-null function pointer",
                )(
                (*pDb).interp,
                pCmd,
                ((*tclStubsPtr).tcl_NewWideIntObj)
                    .expect("non-null function pointer")(pStmt_0 as Tcl_WideInt),
            );
            ((*tclStubsPtr).tcl_ListObjAppendElement)
                .expect(
                    "non-null function pointer",
                )(
                (*pDb).interp,
                pCmd,
                ((*tclStubsPtr).tcl_NewWideIntObj)
                    .expect("non-null function pointer")(ns),
            );
            ((*tclStubsPtr).tcl_EvalObjEx)
                .expect(
                    "non-null function pointer",
                )((*pDb).interp, pCmd, 0x40000 as libc::c_int);
            let mut _objPtr_0: *mut Tcl_Obj = pCmd;
            let ref mut fresh41 = (*_objPtr_0).refCount;
            let fresh42 = *fresh41;
            *fresh41 = *fresh41 - 1;
            if fresh42 <= 1 as libc::c_int {
                ((*tclStubsPtr).tclFreeObj)
                    .expect("non-null function pointer")(_objPtr_0);
            }
            ((*tclStubsPtr).tcl_ResetResult)
                .expect("non-null function pointer")((*pDb).interp);
        }
        4 => {
            let mut pStmt_1: *mut sqlite3_stmt = pd as *mut sqlite3_stmt;
            pCmd = ((*tclStubsPtr).tcl_NewStringObj)
                .expect(
                    "non-null function pointer",
                )((*pDb).zTraceV2, -(1 as libc::c_int));
            let ref mut fresh43 = (*pCmd).refCount;
            *fresh43 += 1;
            ((*tclStubsPtr).tcl_ListObjAppendElement)
                .expect(
                    "non-null function pointer",
                )(
                (*pDb).interp,
                pCmd,
                ((*tclStubsPtr).tcl_NewWideIntObj)
                    .expect("non-null function pointer")(pStmt_1 as Tcl_WideInt),
            );
            ((*tclStubsPtr).tcl_EvalObjEx)
                .expect(
                    "non-null function pointer",
                )((*pDb).interp, pCmd, 0x40000 as libc::c_int);
            let mut _objPtr_1: *mut Tcl_Obj = pCmd;
            let ref mut fresh44 = (*_objPtr_1).refCount;
            let fresh45 = *fresh44;
            *fresh44 = *fresh44 - 1;
            if fresh45 <= 1 as libc::c_int {
                ((*tclStubsPtr).tclFreeObj)
                    .expect("non-null function pointer")(_objPtr_1);
            }
            ((*tclStubsPtr).tcl_ResetResult)
                .expect("non-null function pointer")((*pDb).interp);
        }
        8 => {
            let mut db: *mut sqlite3 = pd as *mut sqlite3;
            pCmd = ((*tclStubsPtr).tcl_NewStringObj)
                .expect(
                    "non-null function pointer",
                )((*pDb).zTraceV2, -(1 as libc::c_int));
            let ref mut fresh46 = (*pCmd).refCount;
            *fresh46 += 1;
            ((*tclStubsPtr).tcl_ListObjAppendElement)
                .expect(
                    "non-null function pointer",
                )(
                (*pDb).interp,
                pCmd,
                ((*tclStubsPtr).tcl_NewWideIntObj)
                    .expect("non-null function pointer")(db as Tcl_WideInt),
            );
            ((*tclStubsPtr).tcl_EvalObjEx)
                .expect(
                    "non-null function pointer",
                )((*pDb).interp, pCmd, 0x40000 as libc::c_int);
            let mut _objPtr_2: *mut Tcl_Obj = pCmd;
            let ref mut fresh47 = (*_objPtr_2).refCount;
            let fresh48 = *fresh47;
            *fresh47 = *fresh47 - 1;
            if fresh48 <= 1 as libc::c_int {
                ((*tclStubsPtr).tclFreeObj)
                    .expect("non-null function pointer")(_objPtr_2);
            }
            ((*tclStubsPtr).tcl_ResetResult)
                .expect("non-null function pointer")((*pDb).interp);
        }
        _ => {}
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn DbProfileHandler(
    mut cd: *mut libc::c_void,
    mut zSql: *const libc::c_char,
    mut tm: sqlite_uint64,
) {
    let mut pDb: *mut SqliteDb = cd as *mut SqliteDb;
    let mut str: Tcl_DString = Tcl_DString {
        string: 0 as *mut libc::c_char,
        length: 0,
        spaceAvl: 0,
        staticSpace: [0; 200],
    };
    let mut zTm: [libc::c_char; 100] = [0; 100];
    sqlite3_snprintf(
        (::std::mem::size_of::<[libc::c_char; 100]>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong) as libc::c_int,
        zTm.as_mut_ptr(),
        b"%lld\0" as *const u8 as *const libc::c_char,
        tm,
    );
    ((*tclStubsPtr).tcl_DStringInit).expect("non-null function pointer")(&mut str);
    ((*tclStubsPtr).tcl_DStringAppend)
        .expect(
            "non-null function pointer",
        )(&mut str, (*pDb).zProfile, -(1 as libc::c_int));
    ((*tclStubsPtr).tcl_DStringAppendElement)
        .expect("non-null function pointer")(&mut str, zSql);
    ((*tclStubsPtr).tcl_DStringAppendElement)
        .expect("non-null function pointer")(&mut str, zTm.as_mut_ptr());
    ((*tclStubsPtr).tcl_Eval)
        .expect("non-null function pointer")((*pDb).interp, str.string);
    ((*tclStubsPtr).tcl_DStringFree).expect("non-null function pointer")(&mut str);
    ((*tclStubsPtr).tcl_ResetResult).expect("non-null function pointer")((*pDb).interp);
}
unsafe extern "C" fn DbCommitHandler(mut cd: *mut libc::c_void) -> libc::c_int {
    let mut pDb: *mut SqliteDb = cd as *mut SqliteDb;
    let mut rc: libc::c_int = 0;
    rc = ((*tclStubsPtr).tcl_Eval)
        .expect("non-null function pointer")((*pDb).interp, (*pDb).zCommit);
    if rc != 0 as libc::c_int
        || atoi(
            ((*tclStubsPtr).tcl_GetStringResult)
                .expect("non-null function pointer")((*pDb).interp),
        ) != 0
    {
        return 1 as libc::c_int;
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn DbRollbackHandler(mut clientData: *mut libc::c_void) {
    let mut pDb: *mut SqliteDb = clientData as *mut SqliteDb;
    if 0 as libc::c_int
        != ((*tclStubsPtr).tcl_EvalObjEx)
            .expect(
                "non-null function pointer",
            )((*pDb).interp, (*pDb).pRollbackHook, 0 as libc::c_int)
    {
        ((*tclStubsPtr).tcl_BackgroundError)
            .expect("non-null function pointer")((*pDb).interp);
    }
}
unsafe extern "C" fn DbWalHandler(
    mut clientData: *mut libc::c_void,
    mut db: *mut sqlite3,
    mut zDb: *const libc::c_char,
    mut nEntry: libc::c_int,
) -> libc::c_int {
    let mut ret: libc::c_int = 0 as libc::c_int;
    let mut p: *mut Tcl_Obj = 0 as *mut Tcl_Obj;
    let mut pDb: *mut SqliteDb = clientData as *mut SqliteDb;
    let mut interp: *mut Tcl_Interp = (*pDb).interp;
    p = ((*tclStubsPtr).tcl_DuplicateObj)
        .expect("non-null function pointer")((*pDb).pWalHook);
    let ref mut fresh49 = (*p).refCount;
    *fresh49 += 1;
    ((*tclStubsPtr).tcl_ListObjAppendElement)
        .expect(
            "non-null function pointer",
        )(
        interp,
        p,
        ((*tclStubsPtr).tcl_NewStringObj)
            .expect("non-null function pointer")(zDb, -(1 as libc::c_int)),
    );
    ((*tclStubsPtr).tcl_ListObjAppendElement)
        .expect(
            "non-null function pointer",
        )(
        interp,
        p,
        ((*tclStubsPtr).tcl_NewIntObj).expect("non-null function pointer")(nEntry),
    );
    if 0 as libc::c_int
        != ((*tclStubsPtr).tcl_EvalObjEx)
            .expect("non-null function pointer")(interp, p, 0 as libc::c_int)
        || 0 as libc::c_int
            != ((*tclStubsPtr).tcl_GetIntFromObj)
                .expect(
                    "non-null function pointer",
                )(
                interp,
                ((*tclStubsPtr).tcl_GetObjResult)
                    .expect("non-null function pointer")(interp),
                &mut ret,
            )
    {
        ((*tclStubsPtr).tcl_BackgroundError).expect("non-null function pointer")(interp);
    }
    let mut _objPtr: *mut Tcl_Obj = p;
    let ref mut fresh50 = (*_objPtr).refCount;
    let fresh51 = *fresh50;
    *fresh50 = *fresh50 - 1;
    if fresh51 <= 1 as libc::c_int {
        ((*tclStubsPtr).tclFreeObj).expect("non-null function pointer")(_objPtr);
    }
    return ret;
}
unsafe extern "C" fn DbUpdateHandler(
    mut p: *mut libc::c_void,
    mut op: libc::c_int,
    mut zDb: *const libc::c_char,
    mut zTbl: *const libc::c_char,
    mut rowid: sqlite_int64,
) {
    let mut pDb: *mut SqliteDb = p as *mut SqliteDb;
    let mut pCmd: *mut Tcl_Obj = 0 as *mut Tcl_Obj;
    static mut azStr: [*const libc::c_char; 3] = [
        b"DELETE\0" as *const u8 as *const libc::c_char,
        b"INSERT\0" as *const u8 as *const libc::c_char,
        b"UPDATE\0" as *const u8 as *const libc::c_char,
    ];
    pCmd = ((*tclStubsPtr).tcl_DuplicateObj)
        .expect("non-null function pointer")((*pDb).pUpdateHook);
    let ref mut fresh52 = (*pCmd).refCount;
    *fresh52 += 1;
    ((*tclStubsPtr).tcl_ListObjAppendElement)
        .expect(
            "non-null function pointer",
        )(
        0 as *mut Tcl_Interp,
        pCmd,
        ((*tclStubsPtr).tcl_NewStringObj)
            .expect(
                "non-null function pointer",
            )(
            azStr[((op - 1 as libc::c_int) / 9 as libc::c_int) as usize],
            -(1 as libc::c_int),
        ),
    );
    ((*tclStubsPtr).tcl_ListObjAppendElement)
        .expect(
            "non-null function pointer",
        )(
        0 as *mut Tcl_Interp,
        pCmd,
        ((*tclStubsPtr).tcl_NewStringObj)
            .expect("non-null function pointer")(zDb, -(1 as libc::c_int)),
    );
    ((*tclStubsPtr).tcl_ListObjAppendElement)
        .expect(
            "non-null function pointer",
        )(
        0 as *mut Tcl_Interp,
        pCmd,
        ((*tclStubsPtr).tcl_NewStringObj)
            .expect("non-null function pointer")(zTbl, -(1 as libc::c_int)),
    );
    ((*tclStubsPtr).tcl_ListObjAppendElement)
        .expect(
            "non-null function pointer",
        )(
        0 as *mut Tcl_Interp,
        pCmd,
        ((*tclStubsPtr).tcl_NewWideIntObj).expect("non-null function pointer")(rowid),
    );
    ((*tclStubsPtr).tcl_EvalObjEx)
        .expect(
            "non-null function pointer",
        )((*pDb).interp, pCmd, 0x40000 as libc::c_int);
    let mut _objPtr: *mut Tcl_Obj = pCmd;
    let ref mut fresh53 = (*_objPtr).refCount;
    let fresh54 = *fresh53;
    *fresh53 = *fresh53 - 1;
    if fresh54 <= 1 as libc::c_int {
        ((*tclStubsPtr).tclFreeObj).expect("non-null function pointer")(_objPtr);
    }
}
unsafe extern "C" fn tclCollateNeeded(
    mut pCtx: *mut libc::c_void,
    mut db: *mut sqlite3,
    mut enc: libc::c_int,
    mut zName: *const libc::c_char,
) {
    let mut pDb: *mut SqliteDb = pCtx as *mut SqliteDb;
    let mut pScript: *mut Tcl_Obj = ((*tclStubsPtr).tcl_DuplicateObj)
        .expect("non-null function pointer")((*pDb).pCollateNeeded);
    let ref mut fresh55 = (*pScript).refCount;
    *fresh55 += 1;
    ((*tclStubsPtr).tcl_ListObjAppendElement)
        .expect(
            "non-null function pointer",
        )(
        0 as *mut Tcl_Interp,
        pScript,
        ((*tclStubsPtr).tcl_NewStringObj)
            .expect("non-null function pointer")(zName, -(1 as libc::c_int)),
    );
    ((*tclStubsPtr).tcl_EvalObjEx)
        .expect("non-null function pointer")((*pDb).interp, pScript, 0 as libc::c_int);
    let mut _objPtr: *mut Tcl_Obj = pScript;
    let ref mut fresh56 = (*_objPtr).refCount;
    let fresh57 = *fresh56;
    *fresh56 = *fresh56 - 1;
    if fresh57 <= 1 as libc::c_int {
        ((*tclStubsPtr).tclFreeObj).expect("non-null function pointer")(_objPtr);
    }
}
unsafe extern "C" fn tclSqlCollate(
    mut pCtx: *mut libc::c_void,
    mut nA: libc::c_int,
    mut zA: *const libc::c_void,
    mut nB: libc::c_int,
    mut zB: *const libc::c_void,
) -> libc::c_int {
    let mut p: *mut SqlCollate = pCtx as *mut SqlCollate;
    let mut pCmd: *mut Tcl_Obj = 0 as *mut Tcl_Obj;
    pCmd = ((*tclStubsPtr).tcl_NewStringObj)
        .expect("non-null function pointer")((*p).zScript, -(1 as libc::c_int));
    let ref mut fresh58 = (*pCmd).refCount;
    *fresh58 += 1;
    ((*tclStubsPtr).tcl_ListObjAppendElement)
        .expect(
            "non-null function pointer",
        )(
        (*p).interp,
        pCmd,
        ((*tclStubsPtr).tcl_NewStringObj)
            .expect("non-null function pointer")(zA as *const libc::c_char, nA),
    );
    ((*tclStubsPtr).tcl_ListObjAppendElement)
        .expect(
            "non-null function pointer",
        )(
        (*p).interp,
        pCmd,
        ((*tclStubsPtr).tcl_NewStringObj)
            .expect("non-null function pointer")(zB as *const libc::c_char, nB),
    );
    ((*tclStubsPtr).tcl_EvalObjEx)
        .expect("non-null function pointer")((*p).interp, pCmd, 0x40000 as libc::c_int);
    let mut _objPtr: *mut Tcl_Obj = pCmd;
    let ref mut fresh59 = (*_objPtr).refCount;
    let fresh60 = *fresh59;
    *fresh59 = *fresh59 - 1;
    if fresh60 <= 1 as libc::c_int {
        ((*tclStubsPtr).tclFreeObj).expect("non-null function pointer")(_objPtr);
    }
    return atoi(
        ((*tclStubsPtr).tcl_GetStringResult)
            .expect("non-null function pointer")((*p).interp),
    );
}
unsafe extern "C" fn tclSqlFunc(
    mut context: *mut sqlite3_context,
    mut argc: libc::c_int,
    mut argv: *mut *mut sqlite3_value,
) {
    let mut p: *mut SqlFunc = sqlite3_user_data(context) as *mut SqlFunc;
    let mut pCmd: *mut Tcl_Obj = 0 as *mut Tcl_Obj;
    let mut i: libc::c_int = 0;
    let mut rc: libc::c_int = 0;
    if argc == 0 as libc::c_int {
        pCmd = (*p).pScript;
        let ref mut fresh61 = (*pCmd).refCount;
        *fresh61 += 1;
        rc = ((*tclStubsPtr).tcl_EvalObjEx)
            .expect("non-null function pointer")((*p).interp, pCmd, 0 as libc::c_int);
        let mut _objPtr: *mut Tcl_Obj = pCmd;
        let ref mut fresh62 = (*_objPtr).refCount;
        let fresh63 = *fresh62;
        *fresh62 = *fresh62 - 1;
        if fresh63 <= 1 as libc::c_int {
            ((*tclStubsPtr).tclFreeObj).expect("non-null function pointer")(_objPtr);
        }
    } else {
        let mut aArg: *mut *mut Tcl_Obj = 0 as *mut *mut Tcl_Obj;
        let mut nArg: libc::c_int = 0;
        if ((*tclStubsPtr).tcl_ListObjGetElements)
            .expect(
                "non-null function pointer",
            )((*p).interp, (*p).pScript, &mut nArg, &mut aArg) != 0
        {
            sqlite3_result_error(
                context,
                ((*tclStubsPtr).tcl_GetStringResult)
                    .expect("non-null function pointer")((*p).interp),
                -(1 as libc::c_int),
            );
            return;
        }
        pCmd = ((*tclStubsPtr).tcl_NewListObj)
            .expect("non-null function pointer")(nArg, aArg as *const *mut Tcl_Obj);
        let ref mut fresh64 = (*pCmd).refCount;
        *fresh64 += 1;
        i = 0 as libc::c_int;
        while i < argc {
            let mut pIn: *mut sqlite3_value = *argv.offset(i as isize);
            let mut pVal: *mut Tcl_Obj = 0 as *mut Tcl_Obj;
            match sqlite3_value_type(pIn) {
                4 => {
                    let mut bytes: libc::c_int = sqlite3_value_bytes(pIn);
                    pVal = ((*tclStubsPtr).tcl_NewByteArrayObj)
                        .expect(
                            "non-null function pointer",
                        )(sqlite3_value_blob(pIn) as *const libc::c_uchar, bytes);
                }
                1 => {
                    let mut v: sqlite_int64 = sqlite3_value_int64(pIn);
                    if v >= -(2147483647 as libc::c_int) as libc::c_longlong
                        && v <= 2147483647 as libc::c_int as libc::c_longlong
                    {
                        pVal = ((*tclStubsPtr).tcl_NewIntObj)
                            .expect("non-null function pointer")(v as libc::c_int);
                    } else {
                        pVal = ((*tclStubsPtr).tcl_NewWideIntObj)
                            .expect("non-null function pointer")(v);
                    }
                }
                2 => {
                    let mut r: libc::c_double = sqlite3_value_double(pIn);
                    pVal = ((*tclStubsPtr).tcl_NewDoubleObj)
                        .expect("non-null function pointer")(r);
                }
                5 => {
                    pVal = ((*tclStubsPtr).tcl_NewStringObj)
                        .expect(
                            "non-null function pointer",
                        )((*(*p).pDb).zNull, -(1 as libc::c_int));
                }
                _ => {
                    let mut bytes_0: libc::c_int = sqlite3_value_bytes(pIn);
                    pVal = ((*tclStubsPtr).tcl_NewStringObj)
                        .expect(
                            "non-null function pointer",
                        )(sqlite3_value_text(pIn) as *mut libc::c_char, bytes_0);
                }
            }
            rc = ((*tclStubsPtr).tcl_ListObjAppendElement)
                .expect("non-null function pointer")((*p).interp, pCmd, pVal);
            if rc != 0 {
                let mut _objPtr_0: *mut Tcl_Obj = pCmd;
                let ref mut fresh65 = (*_objPtr_0).refCount;
                let fresh66 = *fresh65;
                *fresh65 = *fresh65 - 1;
                if fresh66 <= 1 as libc::c_int {
                    ((*tclStubsPtr).tclFreeObj)
                        .expect("non-null function pointer")(_objPtr_0);
                }
                sqlite3_result_error(
                    context,
                    ((*tclStubsPtr).tcl_GetStringResult)
                        .expect("non-null function pointer")((*p).interp),
                    -(1 as libc::c_int),
                );
                return;
            }
            i += 1;
        }
        if (*p).useEvalObjv == 0 {
            ((*tclStubsPtr).tcl_GetString).expect("non-null function pointer")(pCmd);
        }
        rc = ((*tclStubsPtr).tcl_EvalObjEx)
            .expect(
                "non-null function pointer",
            )((*p).interp, pCmd, 0x40000 as libc::c_int);
        let mut _objPtr_1: *mut Tcl_Obj = pCmd;
        let ref mut fresh67 = (*_objPtr_1).refCount;
        let fresh68 = *fresh67;
        *fresh67 = *fresh67 - 1;
        if fresh68 <= 1 as libc::c_int {
            ((*tclStubsPtr).tclFreeObj).expect("non-null function pointer")(_objPtr_1);
        }
    }
    if rc != 0 && rc != 2 as libc::c_int {
        sqlite3_result_error(
            context,
            ((*tclStubsPtr).tcl_GetStringResult)
                .expect("non-null function pointer")((*p).interp),
            -(1 as libc::c_int),
        );
    } else {
        let mut pVar: *mut Tcl_Obj = ((*tclStubsPtr).tcl_GetObjResult)
            .expect("non-null function pointer")((*p).interp);
        let mut n: libc::c_int = 0;
        let mut data: *mut u8_0 = 0 as *mut u8_0;
        let mut zType: *const libc::c_char = if !((*pVar).typePtr).is_null() {
            (*(*pVar).typePtr).name
        } else {
            b"\0" as *const u8 as *const libc::c_char
        };
        let mut c: libc::c_char = *zType.offset(0 as libc::c_int as isize);
        let mut eType: libc::c_int = (*p).eType;
        if eType == 5 as libc::c_int {
            if c as libc::c_int == 'b' as i32
                && strcmp(zType, b"bytearray\0" as *const u8 as *const libc::c_char)
                    == 0 as libc::c_int && ((*pVar).bytes).is_null()
            {
                eType = 4 as libc::c_int;
            } else if c as libc::c_int == 'b' as i32
                && strcmp(zType, b"boolean\0" as *const u8 as *const libc::c_char)
                    == 0 as libc::c_int
                || c as libc::c_int == 'w' as i32
                    && strcmp(zType, b"wideInt\0" as *const u8 as *const libc::c_char)
                        == 0 as libc::c_int
                || c as libc::c_int == 'i' as i32
                    && strcmp(zType, b"int\0" as *const u8 as *const libc::c_char)
                        == 0 as libc::c_int
            {
                eType = 1 as libc::c_int;
            } else if c as libc::c_int == 'd' as i32
                && strcmp(zType, b"double\0" as *const u8 as *const libc::c_char)
                    == 0 as libc::c_int
            {
                eType = 2 as libc::c_int;
            } else {
                eType = 3 as libc::c_int;
            }
        }
        let mut current_block_69: u64;
        match eType {
            4 => {
                data = ((*tclStubsPtr).tcl_GetByteArrayFromObj)
                    .expect("non-null function pointer")(pVar, &mut n);
                sqlite3_result_blob(
                    context,
                    data as *const libc::c_void,
                    n,
                    ::std::mem::transmute::<
                        libc::intptr_t,
                        sqlite3_destructor_type,
                    >(-(1 as libc::c_int) as libc::intptr_t),
                );
                current_block_69 = 517042441694919077;
            }
            1 => {
                let mut v_0: Tcl_WideInt = 0;
                if 0 as libc::c_int
                    == ((*tclStubsPtr).tcl_GetWideIntFromObj)
                        .expect(
                            "non-null function pointer",
                        )(0 as *mut Tcl_Interp, pVar, &mut v_0)
                {
                    sqlite3_result_int64(context, v_0);
                    current_block_69 = 517042441694919077;
                } else {
                    current_block_69 = 10393716428851982524;
                }
            }
            2 => {
                current_block_69 = 10393716428851982524;
            }
            _ => {
                current_block_69 = 10887629115603254199;
            }
        }
        match current_block_69 {
            10393716428851982524 => {
                let mut r_0: libc::c_double = 0.;
                if 0 as libc::c_int
                    == ((*tclStubsPtr).tcl_GetDoubleFromObj)
                        .expect(
                            "non-null function pointer",
                        )(0 as *mut Tcl_Interp, pVar, &mut r_0)
                {
                    sqlite3_result_double(context, r_0);
                    current_block_69 = 517042441694919077;
                } else {
                    current_block_69 = 10887629115603254199;
                }
            }
            _ => {}
        }
        match current_block_69 {
            10887629115603254199 => {
                data = ((*tclStubsPtr).tcl_GetStringFromObj)
                    .expect("non-null function pointer")(pVar, &mut n)
                    as *mut libc::c_uchar;
                sqlite3_result_text(
                    context,
                    data as *mut libc::c_char,
                    n,
                    ::std::mem::transmute::<
                        libc::intptr_t,
                        sqlite3_destructor_type,
                    >(-(1 as libc::c_int) as libc::intptr_t),
                );
            }
            _ => {}
        }
    };
}
unsafe extern "C" fn auth_callback(
    mut pArg: *mut libc::c_void,
    mut code: libc::c_int,
    mut zArg1: *const libc::c_char,
    mut zArg2: *const libc::c_char,
    mut zArg3: *const libc::c_char,
    mut zArg4: *const libc::c_char,
) -> libc::c_int {
    let mut zCode: *const libc::c_char = 0 as *const libc::c_char;
    let mut str: Tcl_DString = Tcl_DString {
        string: 0 as *mut libc::c_char,
        length: 0,
        spaceAvl: 0,
        staticSpace: [0; 200],
    };
    let mut rc: libc::c_int = 0;
    let mut zReply: *const libc::c_char = 0 as *const libc::c_char;
    let mut pDb: *mut SqliteDb = pArg as *mut SqliteDb;
    if (*pDb).disableAuth != 0 {
        return 0 as libc::c_int;
    }
    match code {
        0 => {
            zCode = b"SQLITE_COPY\0" as *const u8 as *const libc::c_char;
        }
        1 => {
            zCode = b"SQLITE_CREATE_INDEX\0" as *const u8 as *const libc::c_char;
        }
        2 => {
            zCode = b"SQLITE_CREATE_TABLE\0" as *const u8 as *const libc::c_char;
        }
        3 => {
            zCode = b"SQLITE_CREATE_TEMP_INDEX\0" as *const u8 as *const libc::c_char;
        }
        4 => {
            zCode = b"SQLITE_CREATE_TEMP_TABLE\0" as *const u8 as *const libc::c_char;
        }
        5 => {
            zCode = b"SQLITE_CREATE_TEMP_TRIGGER\0" as *const u8 as *const libc::c_char;
        }
        6 => {
            zCode = b"SQLITE_CREATE_TEMP_VIEW\0" as *const u8 as *const libc::c_char;
        }
        7 => {
            zCode = b"SQLITE_CREATE_TRIGGER\0" as *const u8 as *const libc::c_char;
        }
        8 => {
            zCode = b"SQLITE_CREATE_VIEW\0" as *const u8 as *const libc::c_char;
        }
        9 => {
            zCode = b"SQLITE_DELETE\0" as *const u8 as *const libc::c_char;
        }
        10 => {
            zCode = b"SQLITE_DROP_INDEX\0" as *const u8 as *const libc::c_char;
        }
        11 => {
            zCode = b"SQLITE_DROP_TABLE\0" as *const u8 as *const libc::c_char;
        }
        12 => {
            zCode = b"SQLITE_DROP_TEMP_INDEX\0" as *const u8 as *const libc::c_char;
        }
        13 => {
            zCode = b"SQLITE_DROP_TEMP_TABLE\0" as *const u8 as *const libc::c_char;
        }
        14 => {
            zCode = b"SQLITE_DROP_TEMP_TRIGGER\0" as *const u8 as *const libc::c_char;
        }
        15 => {
            zCode = b"SQLITE_DROP_TEMP_VIEW\0" as *const u8 as *const libc::c_char;
        }
        16 => {
            zCode = b"SQLITE_DROP_TRIGGER\0" as *const u8 as *const libc::c_char;
        }
        17 => {
            zCode = b"SQLITE_DROP_VIEW\0" as *const u8 as *const libc::c_char;
        }
        18 => {
            zCode = b"SQLITE_INSERT\0" as *const u8 as *const libc::c_char;
        }
        19 => {
            zCode = b"SQLITE_PRAGMA\0" as *const u8 as *const libc::c_char;
        }
        20 => {
            zCode = b"SQLITE_READ\0" as *const u8 as *const libc::c_char;
        }
        21 => {
            zCode = b"SQLITE_SELECT\0" as *const u8 as *const libc::c_char;
        }
        22 => {
            zCode = b"SQLITE_TRANSACTION\0" as *const u8 as *const libc::c_char;
        }
        23 => {
            zCode = b"SQLITE_UPDATE\0" as *const u8 as *const libc::c_char;
        }
        24 => {
            zCode = b"SQLITE_ATTACH\0" as *const u8 as *const libc::c_char;
        }
        25 => {
            zCode = b"SQLITE_DETACH\0" as *const u8 as *const libc::c_char;
        }
        26 => {
            zCode = b"SQLITE_ALTER_TABLE\0" as *const u8 as *const libc::c_char;
        }
        27 => {
            zCode = b"SQLITE_REINDEX\0" as *const u8 as *const libc::c_char;
        }
        28 => {
            zCode = b"SQLITE_ANALYZE\0" as *const u8 as *const libc::c_char;
        }
        29 => {
            zCode = b"SQLITE_CREATE_VTABLE\0" as *const u8 as *const libc::c_char;
        }
        30 => {
            zCode = b"SQLITE_DROP_VTABLE\0" as *const u8 as *const libc::c_char;
        }
        31 => {
            zCode = b"SQLITE_FUNCTION\0" as *const u8 as *const libc::c_char;
        }
        32 => {
            zCode = b"SQLITE_SAVEPOINT\0" as *const u8 as *const libc::c_char;
        }
        33 => {
            zCode = b"SQLITE_RECURSIVE\0" as *const u8 as *const libc::c_char;
        }
        _ => {
            zCode = b"????\0" as *const u8 as *const libc::c_char;
        }
    }
    ((*tclStubsPtr).tcl_DStringInit).expect("non-null function pointer")(&mut str);
    ((*tclStubsPtr).tcl_DStringAppend)
        .expect(
            "non-null function pointer",
        )(&mut str, (*pDb).zAuth, -(1 as libc::c_int));
    ((*tclStubsPtr).tcl_DStringAppendElement)
        .expect("non-null function pointer")(&mut str, zCode);
    ((*tclStubsPtr).tcl_DStringAppendElement)
        .expect(
            "non-null function pointer",
        )(
        &mut str,
        if !zArg1.is_null() { zArg1 } else { b"\0" as *const u8 as *const libc::c_char },
    );
    ((*tclStubsPtr).tcl_DStringAppendElement)
        .expect(
            "non-null function pointer",
        )(
        &mut str,
        if !zArg2.is_null() { zArg2 } else { b"\0" as *const u8 as *const libc::c_char },
    );
    ((*tclStubsPtr).tcl_DStringAppendElement)
        .expect(
            "non-null function pointer",
        )(
        &mut str,
        if !zArg3.is_null() { zArg3 } else { b"\0" as *const u8 as *const libc::c_char },
    );
    ((*tclStubsPtr).tcl_DStringAppendElement)
        .expect(
            "non-null function pointer",
        )(
        &mut str,
        if !zArg4.is_null() { zArg4 } else { b"\0" as *const u8 as *const libc::c_char },
    );
    rc = ((*tclStubsPtr).tcl_GlobalEval)
        .expect("non-null function pointer")((*pDb).interp, str.string);
    ((*tclStubsPtr).tcl_DStringFree).expect("non-null function pointer")(&mut str);
    zReply = if rc == 0 as libc::c_int {
        ((*tclStubsPtr).tcl_GetStringResult)
            .expect("non-null function pointer")((*pDb).interp)
    } else {
        b"SQLITE_DENY\0" as *const u8 as *const libc::c_char
    };
    if strcmp(zReply, b"SQLITE_OK\0" as *const u8 as *const libc::c_char)
        == 0 as libc::c_int
    {
        rc = 0 as libc::c_int;
    } else if strcmp(zReply, b"SQLITE_DENY\0" as *const u8 as *const libc::c_char)
        == 0 as libc::c_int
    {
        rc = 1 as libc::c_int;
    } else if strcmp(zReply, b"SQLITE_IGNORE\0" as *const u8 as *const libc::c_char)
        == 0 as libc::c_int
    {
        rc = 2 as libc::c_int;
    } else {
        rc = 999 as libc::c_int;
    }
    return rc;
}
unsafe extern "C" fn local_getline(
    mut zPrompt: *mut libc::c_char,
    mut in_0: *mut FILE,
) -> *mut libc::c_char {
    let mut zLine: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut nLine: libc::c_int = 0;
    let mut n: libc::c_int = 0;
    nLine = 100 as libc::c_int;
    zLine = malloc(nLine as libc::c_ulong) as *mut libc::c_char;
    if zLine.is_null() {
        return 0 as *mut libc::c_char;
    }
    n = 0 as libc::c_int;
    loop {
        if n + 100 as libc::c_int > nLine {
            nLine = nLine * 2 as libc::c_int + 100 as libc::c_int;
            zLine = realloc(zLine as *mut libc::c_void, nLine as libc::c_ulong)
                as *mut libc::c_char;
            if zLine.is_null() {
                return 0 as *mut libc::c_char;
            }
        }
        if (fgets(&mut *zLine.offset(n as isize), nLine - n, in_0)).is_null() {
            if n == 0 as libc::c_int {
                free(zLine as *mut libc::c_void);
                return 0 as *mut libc::c_char;
            }
            *zLine.offset(n as isize) = 0 as libc::c_int as libc::c_char;
            break;
        } else {
            while *zLine.offset(n as isize) != 0 {
                n += 1;
            }
            if !(n > 0 as libc::c_int
                && *zLine.offset((n - 1 as libc::c_int) as isize) as libc::c_int
                    == '\n' as i32)
            {
                continue;
            }
            n -= 1;
            *zLine.offset(n as isize) = 0 as libc::c_int as libc::c_char;
            break;
        }
    }
    zLine = realloc(zLine as *mut libc::c_void, (n + 1 as libc::c_int) as libc::c_ulong)
        as *mut libc::c_char;
    return zLine;
}
unsafe extern "C" fn DbTransPostCmd(
    mut data: *mut ClientData,
    mut interp: *mut Tcl_Interp,
    mut result: libc::c_int,
) -> libc::c_int {
    static mut azEnd: [*const libc::c_char; 4] = [
        b"RELEASE _tcl_transaction\0" as *const u8 as *const libc::c_char,
        b"COMMIT\0" as *const u8 as *const libc::c_char,
        b"ROLLBACK TO _tcl_transaction ; RELEASE _tcl_transaction\0" as *const u8
            as *const libc::c_char,
        b"ROLLBACK\0" as *const u8 as *const libc::c_char,
    ];
    let mut pDb: *mut SqliteDb = *data.offset(0 as libc::c_int as isize)
        as *mut SqliteDb;
    let mut rc: libc::c_int = result;
    let mut zEnd: *const libc::c_char = 0 as *const libc::c_char;
    let ref mut fresh69 = (*pDb).nTransaction;
    *fresh69 -= 1;
    zEnd = azEnd[((rc == 1 as libc::c_int) as libc::c_int * 2 as libc::c_int
        + ((*pDb).nTransaction == 0 as libc::c_int) as libc::c_int) as usize];
    let ref mut fresh70 = (*pDb).disableAuth;
    *fresh70 += 1;
    if sqlite3_exec(
        (*pDb).db,
        zEnd,
        None,
        0 as *mut libc::c_void,
        0 as *mut *mut libc::c_char,
    ) != 0
    {
        if rc != 1 as libc::c_int {
            ((*tclStubsPtr).tcl_AppendResult)
                .expect(
                    "non-null function pointer",
                )(interp, sqlite3_errmsg((*pDb).db), 0 as *mut libc::c_char);
            rc = 1 as libc::c_int;
        }
        sqlite3_exec(
            (*pDb).db,
            b"ROLLBACK\0" as *const u8 as *const libc::c_char,
            None,
            0 as *mut libc::c_void,
            0 as *mut *mut libc::c_char,
        );
    }
    let ref mut fresh71 = (*pDb).disableAuth;
    *fresh71 -= 1;
    delDatabaseRef(pDb);
    return rc;
}
unsafe extern "C" fn dbPrepare(
    mut pDb: *mut SqliteDb,
    mut zSql: *const libc::c_char,
    mut ppStmt: *mut *mut sqlite3_stmt,
    mut pzOut: *mut *const libc::c_char,
) -> libc::c_int {
    let mut prepFlags: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    if (*pDb).maxStmt > 5 as libc::c_int {
        prepFlags = 0x1 as libc::c_int as libc::c_uint;
    }
    return sqlite3_prepare_v3(
        (*pDb).db,
        zSql,
        -(1 as libc::c_int),
        prepFlags,
        ppStmt,
        pzOut,
    );
}
unsafe extern "C" fn dbPrepareAndBind(
    mut pDb: *mut SqliteDb,
    mut zIn: *const libc::c_char,
    mut pzOut: *mut *const libc::c_char,
    mut ppPreStmt: *mut *mut SqlPreparedStmt,
) -> libc::c_int {
    let mut zSql: *const libc::c_char = zIn;
    let mut pStmt: *mut sqlite3_stmt = 0 as *mut sqlite3_stmt;
    let mut pPreStmt: *mut SqlPreparedStmt = 0 as *mut SqlPreparedStmt;
    let mut nSql: libc::c_int = 0;
    let mut nVar: libc::c_int = 0 as libc::c_int;
    let mut iParm: libc::c_int = 0 as libc::c_int;
    let mut c: libc::c_char = 0;
    let mut i: libc::c_int = 0;
    let mut needResultReset: libc::c_int = 0 as libc::c_int;
    let mut rc: libc::c_int = 0 as libc::c_int;
    let mut interp: *mut Tcl_Interp = (*pDb).interp;
    *ppPreStmt = 0 as *mut SqlPreparedStmt;
    loop {
        c = *zSql.offset(0 as libc::c_int as isize);
        if !(c as libc::c_int == ' ' as i32 || c as libc::c_int == '\t' as i32
            || c as libc::c_int == '\r' as i32 || c as libc::c_int == '\n' as i32)
        {
            break;
        }
        zSql = zSql.offset(1);
    }
    nSql = strlen30(zSql);
    pPreStmt = (*pDb).stmtList;
    while !pPreStmt.is_null() {
        let mut n: libc::c_int = (*pPreStmt).nSql;
        if nSql >= n
            && memcmp(
                (*pPreStmt).zSql as *const libc::c_void,
                zSql as *const libc::c_void,
                n as libc::c_ulong,
            ) == 0 as libc::c_int
            && (*zSql.offset(n as isize) as libc::c_int == 0 as libc::c_int
                || *zSql.offset((n - 1 as libc::c_int) as isize) as libc::c_int
                    == ';' as i32)
        {
            pStmt = (*pPreStmt).pStmt;
            *pzOut = &*zSql.offset((*pPreStmt).nSql as isize) as *const libc::c_char;
            if !((*pPreStmt).pPrev).is_null() {
                let ref mut fresh72 = (*(*pPreStmt).pPrev).pNext;
                *fresh72 = (*pPreStmt).pNext;
            } else {
                let ref mut fresh73 = (*pDb).stmtList;
                *fresh73 = (*pPreStmt).pNext;
            }
            if !((*pPreStmt).pNext).is_null() {
                let ref mut fresh74 = (*(*pPreStmt).pNext).pPrev;
                *fresh74 = (*pPreStmt).pPrev;
            } else {
                let ref mut fresh75 = (*pDb).stmtLast;
                *fresh75 = (*pPreStmt).pPrev;
            }
            let ref mut fresh76 = (*pDb).nStmt;
            *fresh76 -= 1;
            nVar = sqlite3_bind_parameter_count(pStmt);
            break;
        } else {
            pPreStmt = (*pPreStmt).pNext;
        }
    }
    if pPreStmt.is_null() {
        let mut nByte: libc::c_int = 0;
        if 0 as libc::c_int != dbPrepare(pDb, zSql, &mut pStmt, pzOut) {
            ((*tclStubsPtr).tcl_SetObjResult)
                .expect(
                    "non-null function pointer",
                )(
                interp,
                ((*tclStubsPtr).tcl_NewStringObj)
                    .expect(
                        "non-null function pointer",
                    )(sqlite3_errmsg((*pDb).db), -(1 as libc::c_int)),
            );
            return 1 as libc::c_int;
        }
        if pStmt.is_null() {
            if 0 as libc::c_int != sqlite3_errcode((*pDb).db) {
                ((*tclStubsPtr).tcl_SetObjResult)
                    .expect(
                        "non-null function pointer",
                    )(
                    interp,
                    ((*tclStubsPtr).tcl_NewStringObj)
                        .expect(
                            "non-null function pointer",
                        )(sqlite3_errmsg((*pDb).db), -(1 as libc::c_int)),
                );
                return 1 as libc::c_int;
            } else {
                return 0 as libc::c_int
            }
        }
        nVar = sqlite3_bind_parameter_count(pStmt);
        nByte = (::std::mem::size_of::<SqlPreparedStmt>() as libc::c_ulong)
            .wrapping_add(
                (nVar as libc::c_ulong)
                    .wrapping_mul(::std::mem::size_of::<*mut Tcl_Obj>() as libc::c_ulong),
            ) as libc::c_int;
        pPreStmt = ((*tclStubsPtr).tcl_Alloc)
            .expect("non-null function pointer")(nByte as libc::c_uint)
            as *mut SqlPreparedStmt;
        memset(pPreStmt as *mut libc::c_void, 0 as libc::c_int, nByte as libc::c_ulong);
        let ref mut fresh77 = (*pPreStmt).pStmt;
        *fresh77 = pStmt;
        (*pPreStmt).nSql = (*pzOut).offset_from(zSql) as libc::c_long as libc::c_int;
        let ref mut fresh78 = (*pPreStmt).zSql;
        *fresh78 = sqlite3_sql(pStmt);
        let ref mut fresh79 = (*pPreStmt).apParm;
        *fresh79 = &mut *pPreStmt.offset(1 as libc::c_int as isize)
            as *mut SqlPreparedStmt as *mut *mut Tcl_Obj;
    }
    i = 1 as libc::c_int;
    while i <= nVar {
        let mut zVar: *const libc::c_char = sqlite3_bind_parameter_name(pStmt, i);
        if !zVar.is_null()
            && (*zVar.offset(0 as libc::c_int as isize) as libc::c_int == '$' as i32
                || *zVar.offset(0 as libc::c_int as isize) as libc::c_int == ':' as i32
                || *zVar.offset(0 as libc::c_int as isize) as libc::c_int == '@' as i32)
        {
            let mut pVar: *mut Tcl_Obj = ((*tclStubsPtr).tcl_GetVar2Ex)
                .expect(
                    "non-null function pointer",
                )(
                interp,
                &*zVar.offset(1 as libc::c_int as isize),
                0 as *const libc::c_char,
                0 as libc::c_int,
            );
            if pVar.is_null() && !((*pDb).zBindFallback).is_null() {
                let mut pCmd: *mut Tcl_Obj = 0 as *mut Tcl_Obj;
                let mut rx: libc::c_int = 0;
                pCmd = ((*tclStubsPtr).tcl_NewStringObj)
                    .expect(
                        "non-null function pointer",
                    )((*pDb).zBindFallback, -(1 as libc::c_int));
                let ref mut fresh80 = (*pCmd).refCount;
                *fresh80 += 1;
                ((*tclStubsPtr).tcl_ListObjAppendElement)
                    .expect(
                        "non-null function pointer",
                    )(
                    interp,
                    pCmd,
                    ((*tclStubsPtr).tcl_NewStringObj)
                        .expect("non-null function pointer")(zVar, -(1 as libc::c_int)),
                );
                if needResultReset != 0 {
                    ((*tclStubsPtr).tcl_ResetResult)
                        .expect("non-null function pointer")(interp);
                }
                needResultReset = 1 as libc::c_int;
                rx = ((*tclStubsPtr).tcl_EvalObjEx)
                    .expect(
                        "non-null function pointer",
                    )(interp, pCmd, 0x40000 as libc::c_int);
                let mut _objPtr: *mut Tcl_Obj = pCmd;
                let ref mut fresh81 = (*_objPtr).refCount;
                let fresh82 = *fresh81;
                *fresh81 = *fresh81 - 1;
                if fresh82 <= 1 as libc::c_int {
                    ((*tclStubsPtr).tclFreeObj)
                        .expect("non-null function pointer")(_objPtr);
                }
                if rx == 0 as libc::c_int {
                    pVar = ((*tclStubsPtr).tcl_GetObjResult)
                        .expect("non-null function pointer")(interp);
                } else if rx == 1 as libc::c_int {
                    rc = 1 as libc::c_int;
                    break;
                } else {
                    pVar = 0 as *mut Tcl_Obj;
                }
            }
            if !pVar.is_null() {
                let mut n_0: libc::c_int = 0;
                let mut data: *mut u8_0 = 0 as *mut u8_0;
                let mut zType: *const libc::c_char = if !((*pVar).typePtr).is_null() {
                    (*(*pVar).typePtr).name
                } else {
                    b"\0" as *const u8 as *const libc::c_char
                };
                c = *zType.offset(0 as libc::c_int as isize);
                if *zVar.offset(0 as libc::c_int as isize) as libc::c_int == '@' as i32
                    || c as libc::c_int == 'b' as i32
                        && strcmp(
                            zType,
                            b"bytearray\0" as *const u8 as *const libc::c_char,
                        ) == 0 as libc::c_int && ((*pVar).bytes).is_null()
                {
                    data = ((*tclStubsPtr).tcl_GetByteArrayFromObj)
                        .expect("non-null function pointer")(pVar, &mut n_0);
                    sqlite3_bind_blob(pStmt, i, data as *const libc::c_void, n_0, None);
                    let ref mut fresh83 = (*pVar).refCount;
                    *fresh83 += 1;
                    let fresh84 = iParm;
                    iParm = iParm + 1;
                    let ref mut fresh85 = *((*pPreStmt).apParm).offset(fresh84 as isize);
                    *fresh85 = pVar;
                } else if c as libc::c_int == 'b' as i32
                    && strcmp(zType, b"boolean\0" as *const u8 as *const libc::c_char)
                        == 0 as libc::c_int
                {
                    ((*tclStubsPtr).tcl_GetIntFromObj)
                        .expect("non-null function pointer")(interp, pVar, &mut n_0);
                    sqlite3_bind_int(pStmt, i, n_0);
                } else if c as libc::c_int == 'd' as i32
                    && strcmp(zType, b"double\0" as *const u8 as *const libc::c_char)
                        == 0 as libc::c_int
                {
                    let mut r: libc::c_double = 0.;
                    ((*tclStubsPtr).tcl_GetDoubleFromObj)
                        .expect("non-null function pointer")(interp, pVar, &mut r);
                    sqlite3_bind_double(pStmt, i, r);
                } else if c as libc::c_int == 'w' as i32
                    && strcmp(zType, b"wideInt\0" as *const u8 as *const libc::c_char)
                        == 0 as libc::c_int
                    || c as libc::c_int == 'i' as i32
                        && strcmp(zType, b"int\0" as *const u8 as *const libc::c_char)
                            == 0 as libc::c_int
                {
                    let mut v: Tcl_WideInt = 0;
                    ((*tclStubsPtr).tcl_GetWideIntFromObj)
                        .expect("non-null function pointer")(interp, pVar, &mut v);
                    sqlite3_bind_int64(pStmt, i, v);
                } else {
                    data = ((*tclStubsPtr).tcl_GetStringFromObj)
                        .expect("non-null function pointer")(pVar, &mut n_0)
                        as *mut libc::c_uchar;
                    sqlite3_bind_text(pStmt, i, data as *mut libc::c_char, n_0, None);
                    let ref mut fresh86 = (*pVar).refCount;
                    *fresh86 += 1;
                    let fresh87 = iParm;
                    iParm = iParm + 1;
                    let ref mut fresh88 = *((*pPreStmt).apParm).offset(fresh87 as isize);
                    *fresh88 = pVar;
                }
            } else {
                sqlite3_bind_null(pStmt, i);
            }
            if needResultReset != 0 {
                ((*tclStubsPtr).tcl_ResetResult)
                    .expect("non-null function pointer")((*pDb).interp);
            }
        }
        i += 1;
    }
    (*pPreStmt).nParm = iParm;
    *ppPreStmt = pPreStmt;
    if needResultReset != 0 && rc == 0 as libc::c_int {
        ((*tclStubsPtr).tcl_ResetResult)
            .expect("non-null function pointer")((*pDb).interp);
    }
    return rc;
}
unsafe extern "C" fn dbReleaseStmt(
    mut pDb: *mut SqliteDb,
    mut pPreStmt: *mut SqlPreparedStmt,
    mut discard: libc::c_int,
) {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < (*pPreStmt).nParm {
        let mut _objPtr: *mut Tcl_Obj = *((*pPreStmt).apParm).offset(i as isize);
        let ref mut fresh89 = (*_objPtr).refCount;
        let fresh90 = *fresh89;
        *fresh89 = *fresh89 - 1;
        if fresh90 <= 1 as libc::c_int {
            ((*tclStubsPtr).tclFreeObj).expect("non-null function pointer")(_objPtr);
        }
        i += 1;
    }
    (*pPreStmt).nParm = 0 as libc::c_int;
    if (*pDb).maxStmt <= 0 as libc::c_int || discard != 0 {
        dbFreeStmt(pPreStmt);
    } else {
        let ref mut fresh91 = (*pPreStmt).pNext;
        *fresh91 = (*pDb).stmtList;
        let ref mut fresh92 = (*pPreStmt).pPrev;
        *fresh92 = 0 as *mut SqlPreparedStmt;
        if !((*pDb).stmtList).is_null() {
            let ref mut fresh93 = (*(*pDb).stmtList).pPrev;
            *fresh93 = pPreStmt;
        }
        let ref mut fresh94 = (*pDb).stmtList;
        *fresh94 = pPreStmt;
        if ((*pDb).stmtLast).is_null() {
            let ref mut fresh95 = (*pDb).stmtLast;
            *fresh95 = pPreStmt;
        }
        let ref mut fresh96 = (*pDb).nStmt;
        *fresh96 += 1;
        while (*pDb).nStmt > (*pDb).maxStmt {
            let mut pLast: *mut SqlPreparedStmt = (*pDb).stmtLast;
            let ref mut fresh97 = (*pDb).stmtLast;
            *fresh97 = (*pLast).pPrev;
            let ref mut fresh98 = (*(*pDb).stmtLast).pNext;
            *fresh98 = 0 as *mut SqlPreparedStmt;
            let ref mut fresh99 = (*pDb).nStmt;
            *fresh99 -= 1;
            dbFreeStmt(pLast);
        }
    };
}
unsafe extern "C" fn dbReleaseColumnNames(mut p: *mut DbEvalContext) {
    if !((*p).apColName).is_null() {
        let mut i: libc::c_int = 0;
        i = 0 as libc::c_int;
        while i < (*p).nCol {
            let mut _objPtr: *mut Tcl_Obj = *((*p).apColName).offset(i as isize);
            let ref mut fresh100 = (*_objPtr).refCount;
            let fresh101 = *fresh100;
            *fresh100 = *fresh100 - 1;
            if fresh101 <= 1 as libc::c_int {
                ((*tclStubsPtr).tclFreeObj).expect("non-null function pointer")(_objPtr);
            }
            i += 1;
        }
        ((*tclStubsPtr).tcl_Free)
            .expect("non-null function pointer")((*p).apColName as *mut libc::c_char);
        let ref mut fresh102 = (*p).apColName;
        *fresh102 = 0 as *mut *mut Tcl_Obj;
    }
    (*p).nCol = 0 as libc::c_int;
}
unsafe extern "C" fn dbEvalInit(
    mut p: *mut DbEvalContext,
    mut pDb: *mut SqliteDb,
    mut pSql: *mut Tcl_Obj,
    mut pArray: *mut Tcl_Obj,
    mut evalFlags: libc::c_int,
) {
    memset(
        p as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<DbEvalContext>() as libc::c_ulong,
    );
    let ref mut fresh103 = (*p).pDb;
    *fresh103 = pDb;
    let ref mut fresh104 = (*p).zSql;
    *fresh104 = ((*tclStubsPtr).tcl_GetString).expect("non-null function pointer")(pSql);
    let ref mut fresh105 = (*p).pSql;
    *fresh105 = pSql;
    let ref mut fresh106 = (*pSql).refCount;
    *fresh106 += 1;
    if !pArray.is_null() {
        let ref mut fresh107 = (*p).pArray;
        *fresh107 = pArray;
        let ref mut fresh108 = (*pArray).refCount;
        *fresh108 += 1;
    }
    (*p).evalFlags = evalFlags;
    addDatabaseRef((*p).pDb);
}
unsafe extern "C" fn dbEvalRowInfo(
    mut p: *mut DbEvalContext,
    mut pnCol: *mut libc::c_int,
    mut papColName: *mut *mut *mut Tcl_Obj,
) {
    if ((*p).apColName).is_null() {
        let mut pStmt: *mut sqlite3_stmt = (*(*p).pPreStmt).pStmt;
        let mut i: libc::c_int = 0;
        let mut nCol: libc::c_int = 0;
        let mut apColName: *mut *mut Tcl_Obj = 0 as *mut *mut Tcl_Obj;
        nCol = sqlite3_column_count(pStmt);
        (*p).nCol = nCol;
        if nCol > 0 as libc::c_int && (!papColName.is_null() || !((*p).pArray).is_null())
        {
            apColName = ((*tclStubsPtr).tcl_Alloc)
                .expect(
                    "non-null function pointer",
                )(
                (::std::mem::size_of::<*mut Tcl_Obj>() as libc::c_ulong)
                    .wrapping_mul(nCol as libc::c_ulong) as libc::c_uint,
            ) as *mut *mut Tcl_Obj;
            i = 0 as libc::c_int;
            while i < nCol {
                let ref mut fresh109 = *apColName.offset(i as isize);
                *fresh109 = ((*tclStubsPtr).tcl_NewStringObj)
                    .expect(
                        "non-null function pointer",
                    )(sqlite3_column_name(pStmt, i), -(1 as libc::c_int));
                let ref mut fresh110 = (**apColName.offset(i as isize)).refCount;
                *fresh110 += 1;
                i += 1;
            }
            let ref mut fresh111 = (*p).apColName;
            *fresh111 = apColName;
        }
        if !((*p).pArray).is_null() {
            let mut interp: *mut Tcl_Interp = (*(*p).pDb).interp;
            let mut pColList: *mut Tcl_Obj = ((*tclStubsPtr).tcl_NewObj)
                .expect("non-null function pointer")();
            let mut pStar: *mut Tcl_Obj = ((*tclStubsPtr).tcl_NewStringObj)
                .expect(
                    "non-null function pointer",
                )(b"*\0" as *const u8 as *const libc::c_char, -(1 as libc::c_int));
            i = 0 as libc::c_int;
            while i < nCol {
                ((*tclStubsPtr).tcl_ListObjAppendElement)
                    .expect(
                        "non-null function pointer",
                    )(interp, pColList, *apColName.offset(i as isize));
                i += 1;
            }
            let ref mut fresh112 = (*pStar).refCount;
            *fresh112 += 1;
            ((*tclStubsPtr).tcl_ObjSetVar2)
                .expect(
                    "non-null function pointer",
                )(interp, (*p).pArray, pStar, pColList, 0 as libc::c_int);
            let mut _objPtr: *mut Tcl_Obj = pStar;
            let ref mut fresh113 = (*_objPtr).refCount;
            let fresh114 = *fresh113;
            *fresh113 = *fresh113 - 1;
            if fresh114 <= 1 as libc::c_int {
                ((*tclStubsPtr).tclFreeObj).expect("non-null function pointer")(_objPtr);
            }
        }
    }
    if !papColName.is_null() {
        *papColName = (*p).apColName;
    }
    if !pnCol.is_null() {
        *pnCol = (*p).nCol;
    }
}
unsafe extern "C" fn dbEvalStep(mut p: *mut DbEvalContext) -> libc::c_int {
    let mut zPrevSql: *const libc::c_char = 0 as *const libc::c_char;
    while *((*p).zSql).offset(0 as libc::c_int as isize) as libc::c_int != 0
        || !((*p).pPreStmt).is_null()
    {
        let mut rc: libc::c_int = 0;
        if ((*p).pPreStmt).is_null() {
            zPrevSql = if (*p).zSql == zPrevSql {
                0 as *const libc::c_char
            } else {
                (*p).zSql
            };
            rc = dbPrepareAndBind(
                (*p).pDb,
                (*p).zSql,
                &mut (*p).zSql,
                &mut (*p).pPreStmt,
            );
            if rc != 0 as libc::c_int {
                return rc;
            }
        } else {
            let mut rcs: libc::c_int = 0;
            let mut pDb: *mut SqliteDb = (*p).pDb;
            let mut pPreStmt: *mut SqlPreparedStmt = (*p).pPreStmt;
            let mut pStmt: *mut sqlite3_stmt = (*pPreStmt).pStmt;
            rcs = sqlite3_step(pStmt);
            if rcs == 100 as libc::c_int {
                return 0 as libc::c_int;
            }
            if !((*p).pArray).is_null() {
                dbEvalRowInfo(p, 0 as *mut libc::c_int, 0 as *mut *mut *mut Tcl_Obj);
            }
            rcs = sqlite3_reset(pStmt);
            (*pDb)
                .nStep = sqlite3_stmt_status(pStmt, 1 as libc::c_int, 1 as libc::c_int);
            (*pDb)
                .nSort = sqlite3_stmt_status(pStmt, 2 as libc::c_int, 1 as libc::c_int);
            (*pDb)
                .nIndex = sqlite3_stmt_status(pStmt, 3 as libc::c_int, 1 as libc::c_int);
            (*pDb)
                .nVMStep = sqlite3_stmt_status(
                pStmt,
                4 as libc::c_int,
                1 as libc::c_int,
            );
            dbReleaseColumnNames(p);
            let ref mut fresh115 = (*p).pPreStmt;
            *fresh115 = 0 as *mut SqlPreparedStmt;
            if rcs != 0 as libc::c_int {
                dbReleaseStmt(pDb, pPreStmt, 1 as libc::c_int);
                ((*tclStubsPtr).tcl_SetObjResult)
                    .expect(
                        "non-null function pointer",
                    )(
                    (*pDb).interp,
                    ((*tclStubsPtr).tcl_NewStringObj)
                        .expect(
                            "non-null function pointer",
                        )(sqlite3_errmsg((*pDb).db), -(1 as libc::c_int)),
                );
                return 1 as libc::c_int;
            } else {
                dbReleaseStmt(pDb, pPreStmt, 0 as libc::c_int);
            }
        }
    }
    return 3 as libc::c_int;
}
unsafe extern "C" fn dbEvalFinalize(mut p: *mut DbEvalContext) {
    if !((*p).pPreStmt).is_null() {
        sqlite3_reset((*(*p).pPreStmt).pStmt);
        dbReleaseStmt((*p).pDb, (*p).pPreStmt, 0 as libc::c_int);
        let ref mut fresh116 = (*p).pPreStmt;
        *fresh116 = 0 as *mut SqlPreparedStmt;
    }
    if !((*p).pArray).is_null() {
        let mut _objPtr: *mut Tcl_Obj = (*p).pArray;
        let ref mut fresh117 = (*_objPtr).refCount;
        let fresh118 = *fresh117;
        *fresh117 = *fresh117 - 1;
        if fresh118 <= 1 as libc::c_int {
            ((*tclStubsPtr).tclFreeObj).expect("non-null function pointer")(_objPtr);
        }
        let ref mut fresh119 = (*p).pArray;
        *fresh119 = 0 as *mut Tcl_Obj;
    }
    let mut _objPtr_0: *mut Tcl_Obj = (*p).pSql;
    let ref mut fresh120 = (*_objPtr_0).refCount;
    let fresh121 = *fresh120;
    *fresh120 = *fresh120 - 1;
    if fresh121 <= 1 as libc::c_int {
        ((*tclStubsPtr).tclFreeObj).expect("non-null function pointer")(_objPtr_0);
    }
    dbReleaseColumnNames(p);
    delDatabaseRef((*p).pDb);
}
unsafe extern "C" fn dbEvalColumnValue(
    mut p: *mut DbEvalContext,
    mut iCol: libc::c_int,
) -> *mut Tcl_Obj {
    let mut pStmt: *mut sqlite3_stmt = (*(*p).pPreStmt).pStmt;
    match sqlite3_column_type(pStmt, iCol) {
        4 => {
            let mut bytes: libc::c_int = sqlite3_column_bytes(pStmt, iCol);
            let mut zBlob: *const libc::c_char = sqlite3_column_blob(pStmt, iCol)
                as *const libc::c_char;
            if zBlob.is_null() {
                bytes = 0 as libc::c_int;
            }
            return ((*tclStubsPtr).tcl_NewByteArrayObj)
                .expect("non-null function pointer")(zBlob as *mut u8_0, bytes);
        }
        1 => {
            let mut v: sqlite_int64 = sqlite3_column_int64(pStmt, iCol);
            if v >= -(2147483647 as libc::c_int) as libc::c_longlong
                && v <= 2147483647 as libc::c_int as libc::c_longlong
            {
                return ((*tclStubsPtr).tcl_NewIntObj)
                    .expect("non-null function pointer")(v as libc::c_int)
            } else {
                return ((*tclStubsPtr).tcl_NewWideIntObj)
                    .expect("non-null function pointer")(v)
            }
        }
        2 => {
            return ((*tclStubsPtr).tcl_NewDoubleObj)
                .expect("non-null function pointer")(sqlite3_column_double(pStmt, iCol));
        }
        5 => {
            return ((*tclStubsPtr).tcl_NewStringObj)
                .expect(
                    "non-null function pointer",
                )((*(*p).pDb).zNull, -(1 as libc::c_int));
        }
        _ => {}
    }
    return ((*tclStubsPtr).tcl_NewStringObj)
        .expect(
            "non-null function pointer",
        )(sqlite3_column_text(pStmt, iCol) as *mut libc::c_char, -(1 as libc::c_int));
}
unsafe extern "C" fn DbUseNre() -> libc::c_int {
    let mut major: libc::c_int = 0;
    let mut minor: libc::c_int = 0;
    ((*tclStubsPtr).tcl_GetVersion)
        .expect(
            "non-null function pointer",
        )(&mut major, &mut minor, 0 as *mut libc::c_int, 0 as *mut libc::c_int);
    return (major == 8 as libc::c_int && minor >= 6 as libc::c_int
        || major > 8 as libc::c_int) as libc::c_int;
}
unsafe extern "C" fn DbEvalNextCmd(
    mut data: *mut ClientData,
    mut interp: *mut Tcl_Interp,
    mut result: libc::c_int,
) -> libc::c_int {
    let mut rc: libc::c_int = result;
    let mut p: *mut DbEvalContext = *data.offset(0 as libc::c_int as isize)
        as *mut DbEvalContext;
    let mut pScript: *mut Tcl_Obj = *data.offset(1 as libc::c_int as isize)
        as *mut Tcl_Obj;
    let mut pArray: *mut Tcl_Obj = (*p).pArray;
    while (rc == 0 as libc::c_int || rc == 4 as libc::c_int)
        && {
            rc = dbEvalStep(p);
            0 as libc::c_int == rc
        }
    {
        let mut i: libc::c_int = 0;
        let mut nCol: libc::c_int = 0;
        let mut apColName: *mut *mut Tcl_Obj = 0 as *mut *mut Tcl_Obj;
        dbEvalRowInfo(p, &mut nCol, &mut apColName);
        i = 0 as libc::c_int;
        while i < nCol {
            if pArray.is_null() {
                ((*tclStubsPtr).tcl_ObjSetVar2)
                    .expect(
                        "non-null function pointer",
                    )(
                    interp,
                    *apColName.offset(i as isize),
                    0 as *mut Tcl_Obj,
                    dbEvalColumnValue(p, i),
                    0 as libc::c_int,
                );
            } else if (*p).evalFlags & 0x1 as libc::c_int != 0 as libc::c_int
                && sqlite3_column_type((*(*p).pPreStmt).pStmt, i) == 5 as libc::c_int
            {
                ((*tclStubsPtr).tcl_UnsetVar2)
                    .expect(
                        "non-null function pointer",
                    )(
                    interp,
                    ((*tclStubsPtr).tcl_GetString)
                        .expect("non-null function pointer")(pArray),
                    ((*tclStubsPtr).tcl_GetString)
                        .expect(
                            "non-null function pointer",
                        )(*apColName.offset(i as isize)),
                    0 as libc::c_int,
                );
            } else {
                ((*tclStubsPtr).tcl_ObjSetVar2)
                    .expect(
                        "non-null function pointer",
                    )(
                    interp,
                    pArray,
                    *apColName.offset(i as isize),
                    dbEvalColumnValue(p, i),
                    0 as libc::c_int,
                );
            }
            i += 1;
        }
        if DbUseNre() != 0 {
            ((*tclStubsPtr).tcl_NRAddCallback)
                .expect(
                    "non-null function pointer",
                )(
                interp,
                Some(
                    DbEvalNextCmd
                        as unsafe extern "C" fn(
                            *mut ClientData,
                            *mut Tcl_Interp,
                            libc::c_int,
                        ) -> libc::c_int,
                ),
                p as *mut libc::c_void,
                pScript as *mut libc::c_void,
                0 as ClientData,
                0 as ClientData,
            );
            return ((*tclStubsPtr).tcl_NREvalObj)
                .expect("non-null function pointer")(interp, pScript, 0 as libc::c_int);
        } else {
            rc = ((*tclStubsPtr).tcl_EvalObjEx)
                .expect("non-null function pointer")(interp, pScript, 0 as libc::c_int);
        }
    }
    let mut _objPtr: *mut Tcl_Obj = pScript;
    let ref mut fresh122 = (*_objPtr).refCount;
    let fresh123 = *fresh122;
    *fresh122 = *fresh122 - 1;
    if fresh123 <= 1 as libc::c_int {
        ((*tclStubsPtr).tclFreeObj).expect("non-null function pointer")(_objPtr);
    }
    dbEvalFinalize(p);
    ((*tclStubsPtr).tcl_Free)
        .expect("non-null function pointer")(p as *mut libc::c_char);
    if rc == 0 as libc::c_int || rc == 3 as libc::c_int {
        ((*tclStubsPtr).tcl_ResetResult).expect("non-null function pointer")(interp);
        rc = 0 as libc::c_int;
    }
    return rc;
}
unsafe extern "C" fn DbHookCmd(
    mut interp: *mut Tcl_Interp,
    mut pDb: *mut SqliteDb,
    mut pArg: *mut Tcl_Obj,
    mut ppHook: *mut *mut Tcl_Obj,
) {
    let mut db: *mut sqlite3 = (*pDb).db;
    if !(*ppHook).is_null() {
        ((*tclStubsPtr).tcl_SetObjResult)
            .expect("non-null function pointer")(interp, *ppHook);
        if !pArg.is_null() {
            let mut _objPtr: *mut Tcl_Obj = *ppHook;
            let ref mut fresh124 = (*_objPtr).refCount;
            let fresh125 = *fresh124;
            *fresh124 = *fresh124 - 1;
            if fresh125 <= 1 as libc::c_int {
                ((*tclStubsPtr).tclFreeObj).expect("non-null function pointer")(_objPtr);
            }
            *ppHook = 0 as *mut Tcl_Obj;
        }
    }
    if !pArg.is_null() {
        if ((*tclStubsPtr).tcl_GetCharLength).expect("non-null function pointer")(pArg)
            > 0 as libc::c_int
        {
            *ppHook = pArg;
            let ref mut fresh126 = (**ppHook).refCount;
            *fresh126 += 1;
        }
    }
    sqlite3_update_hook(
        db,
        if !((*pDb).pUpdateHook).is_null() {
            Some(
                DbUpdateHandler
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        libc::c_int,
                        *const libc::c_char,
                        *const libc::c_char,
                        sqlite_int64,
                    ) -> (),
            )
        } else {
            None
        },
        pDb as *mut libc::c_void,
    );
    sqlite3_rollback_hook(
        db,
        if !((*pDb).pRollbackHook).is_null() {
            Some(DbRollbackHandler as unsafe extern "C" fn(*mut libc::c_void) -> ())
        } else {
            None
        },
        pDb as *mut libc::c_void,
    );
    sqlite3_wal_hook(
        db,
        if !((*pDb).pWalHook).is_null() {
            Some(
                DbWalHandler
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        *mut sqlite3,
                        *const libc::c_char,
                        libc::c_int,
                    ) -> libc::c_int,
            )
        } else {
            None
        },
        pDb as *mut libc::c_void,
    );
}
unsafe extern "C" fn DbObjCmd(
    mut cd: *mut libc::c_void,
    mut interp: *mut Tcl_Interp,
    mut objc: libc::c_int,
    mut objv: *const *mut Tcl_Obj,
) -> libc::c_int {
    let mut current_block: u64;
    let mut pDb: *mut SqliteDb = cd as *mut SqliteDb;
    let mut choice: libc::c_int = 0;
    let mut rc: libc::c_int = 0 as libc::c_int;
    static mut DB_strs: [*const libc::c_char; 43] = [
        b"authorizer\0" as *const u8 as *const libc::c_char,
        b"backup\0" as *const u8 as *const libc::c_char,
        b"bind_fallback\0" as *const u8 as *const libc::c_char,
        b"busy\0" as *const u8 as *const libc::c_char,
        b"cache\0" as *const u8 as *const libc::c_char,
        b"changes\0" as *const u8 as *const libc::c_char,
        b"close\0" as *const u8 as *const libc::c_char,
        b"collate\0" as *const u8 as *const libc::c_char,
        b"collation_needed\0" as *const u8 as *const libc::c_char,
        b"commit_hook\0" as *const u8 as *const libc::c_char,
        b"complete\0" as *const u8 as *const libc::c_char,
        b"config\0" as *const u8 as *const libc::c_char,
        b"copy\0" as *const u8 as *const libc::c_char,
        b"deserialize\0" as *const u8 as *const libc::c_char,
        b"enable_load_extension\0" as *const u8 as *const libc::c_char,
        b"errorcode\0" as *const u8 as *const libc::c_char,
        b"erroroffset\0" as *const u8 as *const libc::c_char,
        b"eval\0" as *const u8 as *const libc::c_char,
        b"exists\0" as *const u8 as *const libc::c_char,
        b"function\0" as *const u8 as *const libc::c_char,
        b"incrblob\0" as *const u8 as *const libc::c_char,
        b"interrupt\0" as *const u8 as *const libc::c_char,
        b"last_insert_rowid\0" as *const u8 as *const libc::c_char,
        b"nullvalue\0" as *const u8 as *const libc::c_char,
        b"onecolumn\0" as *const u8 as *const libc::c_char,
        b"preupdate\0" as *const u8 as *const libc::c_char,
        b"profile\0" as *const u8 as *const libc::c_char,
        b"progress\0" as *const u8 as *const libc::c_char,
        b"rekey\0" as *const u8 as *const libc::c_char,
        b"restore\0" as *const u8 as *const libc::c_char,
        b"rollback_hook\0" as *const u8 as *const libc::c_char,
        b"serialize\0" as *const u8 as *const libc::c_char,
        b"status\0" as *const u8 as *const libc::c_char,
        b"timeout\0" as *const u8 as *const libc::c_char,
        b"total_changes\0" as *const u8 as *const libc::c_char,
        b"trace\0" as *const u8 as *const libc::c_char,
        b"trace_v2\0" as *const u8 as *const libc::c_char,
        b"transaction\0" as *const u8 as *const libc::c_char,
        b"unlock_notify\0" as *const u8 as *const libc::c_char,
        b"update_hook\0" as *const u8 as *const libc::c_char,
        b"version\0" as *const u8 as *const libc::c_char,
        b"wal_hook\0" as *const u8 as *const libc::c_char,
        0 as *const libc::c_char,
    ];
    if objc < 2 as libc::c_int {
        ((*tclStubsPtr).tcl_WrongNumArgs)
            .expect(
                "non-null function pointer",
            )(
            interp,
            1 as libc::c_int,
            objv,
            b"SUBCOMMAND ...\0" as *const u8 as *const libc::c_char,
        );
        return 1 as libc::c_int;
    }
    if ((*tclStubsPtr).tcl_GetIndexFromObjStruct)
        .expect(
            "non-null function pointer",
        )(
        interp,
        *objv.offset(1 as libc::c_int as isize),
        DB_strs.as_mut_ptr() as *const libc::c_void,
        ::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong as libc::c_int,
        b"option\0" as *const u8 as *const libc::c_char,
        0 as libc::c_int,
        &mut choice,
    ) != 0
    {
        return 1 as libc::c_int;
    }
    match choice as DB_enum as libc::c_uint {
        0 => {
            if objc > 3 as libc::c_int {
                ((*tclStubsPtr).tcl_WrongNumArgs)
                    .expect(
                        "non-null function pointer",
                    )(
                    interp,
                    2 as libc::c_int,
                    objv,
                    b"?CALLBACK?\0" as *const u8 as *const libc::c_char,
                );
                return 1 as libc::c_int;
            } else {
                if objc == 2 as libc::c_int {
                    if !((*pDb).zAuth).is_null() {
                        ((*tclStubsPtr).tcl_AppendResult)
                            .expect(
                                "non-null function pointer",
                            )(interp, (*pDb).zAuth, 0 as *mut libc::c_char);
                    }
                } else {
                    let mut zAuth: *mut libc::c_char = 0 as *mut libc::c_char;
                    let mut len: libc::c_int = 0;
                    if !((*pDb).zAuth).is_null() {
                        ((*tclStubsPtr).tcl_Free)
                            .expect("non-null function pointer")((*pDb).zAuth);
                    }
                    zAuth = ((*tclStubsPtr).tcl_GetStringFromObj)
                        .expect(
                            "non-null function pointer",
                        )(*objv.offset(2 as libc::c_int as isize), &mut len);
                    if !zAuth.is_null() && len > 0 as libc::c_int {
                        let ref mut fresh127 = (*pDb).zAuth;
                        *fresh127 = ((*tclStubsPtr).tcl_Alloc)
                            .expect(
                                "non-null function pointer",
                            )((len + 1 as libc::c_int) as libc::c_uint);
                        memcpy(
                            (*pDb).zAuth as *mut libc::c_void,
                            zAuth as *const libc::c_void,
                            (len + 1 as libc::c_int) as libc::c_ulong,
                        );
                    } else {
                        let ref mut fresh128 = (*pDb).zAuth;
                        *fresh128 = 0 as *mut libc::c_char;
                    }
                    if !((*pDb).zAuth).is_null() {
                        let ref mut fresh129 = (*pDb).interp;
                        *fresh129 = interp;
                        sqlite3_set_authorizer(
                            (*pDb).db,
                            ::std::mem::transmute::<
                                Option::<
                                    unsafe extern "C" fn(
                                        *mut libc::c_void,
                                        libc::c_int,
                                        *const libc::c_char,
                                        *const libc::c_char,
                                        *const libc::c_char,
                                        *const libc::c_char,
                                    ) -> libc::c_int,
                                >,
                                sqlite3_auth_cb,
                            >(
                                Some(
                                    auth_callback
                                        as unsafe extern "C" fn(
                                            *mut libc::c_void,
                                            libc::c_int,
                                            *const libc::c_char,
                                            *const libc::c_char,
                                            *const libc::c_char,
                                            *const libc::c_char,
                                        ) -> libc::c_int,
                                ),
                            ),
                            pDb as *mut libc::c_void,
                        );
                    } else {
                        sqlite3_set_authorizer((*pDb).db, None, 0 as *mut libc::c_void);
                    }
                }
            }
        }
        1 => {
            let mut zDestFile: *const libc::c_char = 0 as *const libc::c_char;
            let mut zSrcDb: *const libc::c_char = 0 as *const libc::c_char;
            let mut pDest: *mut sqlite3 = 0 as *mut sqlite3;
            let mut pBackup: *mut sqlite3_backup = 0 as *mut sqlite3_backup;
            if objc == 3 as libc::c_int {
                zSrcDb = b"main\0" as *const u8 as *const libc::c_char;
                zDestFile = ((*tclStubsPtr).tcl_GetString)
                    .expect(
                        "non-null function pointer",
                    )(*objv.offset(2 as libc::c_int as isize));
            } else if objc == 4 as libc::c_int {
                zSrcDb = ((*tclStubsPtr).tcl_GetString)
                    .expect(
                        "non-null function pointer",
                    )(*objv.offset(2 as libc::c_int as isize));
                zDestFile = ((*tclStubsPtr).tcl_GetString)
                    .expect(
                        "non-null function pointer",
                    )(*objv.offset(3 as libc::c_int as isize));
            } else {
                ((*tclStubsPtr).tcl_WrongNumArgs)
                    .expect(
                        "non-null function pointer",
                    )(
                    interp,
                    2 as libc::c_int,
                    objv,
                    b"?DATABASE? FILENAME\0" as *const u8 as *const libc::c_char,
                );
                return 1 as libc::c_int;
            }
            rc = sqlite3_open_v2(
                zDestFile,
                &mut pDest,
                0x2 as libc::c_int | 0x4 as libc::c_int | (*pDb).openFlags,
                0 as *const libc::c_char,
            );
            if rc != 0 as libc::c_int {
                ((*tclStubsPtr).tcl_AppendResult)
                    .expect(
                        "non-null function pointer",
                    )(
                    interp,
                    b"cannot open target database: \0" as *const u8
                        as *const libc::c_char,
                    sqlite3_errmsg(pDest),
                    0 as *mut libc::c_char,
                );
                sqlite3_close(pDest);
                return 1 as libc::c_int;
            }
            pBackup = sqlite3_backup_init(
                pDest,
                b"main\0" as *const u8 as *const libc::c_char,
                (*pDb).db,
                zSrcDb,
            );
            if pBackup.is_null() {
                ((*tclStubsPtr).tcl_AppendResult)
                    .expect(
                        "non-null function pointer",
                    )(
                    interp,
                    b"backup failed: \0" as *const u8 as *const libc::c_char,
                    sqlite3_errmsg(pDest),
                    0 as *mut libc::c_char,
                );
                sqlite3_close(pDest);
                return 1 as libc::c_int;
            }
            loop {
                rc = sqlite3_backup_step(pBackup, 100 as libc::c_int);
                if !(rc == 0 as libc::c_int) {
                    break;
                }
            }
            sqlite3_backup_finish(pBackup);
            if rc == 101 as libc::c_int {
                rc = 0 as libc::c_int;
            } else {
                ((*tclStubsPtr).tcl_AppendResult)
                    .expect(
                        "non-null function pointer",
                    )(
                    interp,
                    b"backup failed: \0" as *const u8 as *const libc::c_char,
                    sqlite3_errmsg(pDest),
                    0 as *mut libc::c_char,
                );
                rc = 1 as libc::c_int;
            }
            sqlite3_close(pDest);
        }
        2 => {
            if objc > 3 as libc::c_int {
                ((*tclStubsPtr).tcl_WrongNumArgs)
                    .expect(
                        "non-null function pointer",
                    )(
                    interp,
                    2 as libc::c_int,
                    objv,
                    b"?CALLBACK?\0" as *const u8 as *const libc::c_char,
                );
                return 1 as libc::c_int;
            } else {
                if objc == 2 as libc::c_int {
                    if !((*pDb).zBindFallback).is_null() {
                        ((*tclStubsPtr).tcl_AppendResult)
                            .expect(
                                "non-null function pointer",
                            )(interp, (*pDb).zBindFallback, 0 as *mut libc::c_char);
                    }
                } else {
                    let mut zCallback: *mut libc::c_char = 0 as *mut libc::c_char;
                    let mut len_0: libc::c_int = 0;
                    if !((*pDb).zBindFallback).is_null() {
                        ((*tclStubsPtr).tcl_Free)
                            .expect("non-null function pointer")((*pDb).zBindFallback);
                    }
                    zCallback = ((*tclStubsPtr).tcl_GetStringFromObj)
                        .expect(
                            "non-null function pointer",
                        )(*objv.offset(2 as libc::c_int as isize), &mut len_0);
                    if !zCallback.is_null() && len_0 > 0 as libc::c_int {
                        let ref mut fresh130 = (*pDb).zBindFallback;
                        *fresh130 = ((*tclStubsPtr).tcl_Alloc)
                            .expect(
                                "non-null function pointer",
                            )((len_0 + 1 as libc::c_int) as libc::c_uint);
                        memcpy(
                            (*pDb).zBindFallback as *mut libc::c_void,
                            zCallback as *const libc::c_void,
                            (len_0 + 1 as libc::c_int) as libc::c_ulong,
                        );
                    } else {
                        let ref mut fresh131 = (*pDb).zBindFallback;
                        *fresh131 = 0 as *mut libc::c_char;
                    }
                }
            }
        }
        3 => {
            if objc > 3 as libc::c_int {
                ((*tclStubsPtr).tcl_WrongNumArgs)
                    .expect(
                        "non-null function pointer",
                    )(
                    interp,
                    2 as libc::c_int,
                    objv,
                    b"CALLBACK\0" as *const u8 as *const libc::c_char,
                );
                return 1 as libc::c_int;
            } else {
                if objc == 2 as libc::c_int {
                    if !((*pDb).zBusy).is_null() {
                        ((*tclStubsPtr).tcl_AppendResult)
                            .expect(
                                "non-null function pointer",
                            )(interp, (*pDb).zBusy, 0 as *mut libc::c_char);
                    }
                } else {
                    let mut zBusy: *mut libc::c_char = 0 as *mut libc::c_char;
                    let mut len_1: libc::c_int = 0;
                    if !((*pDb).zBusy).is_null() {
                        ((*tclStubsPtr).tcl_Free)
                            .expect("non-null function pointer")((*pDb).zBusy);
                    }
                    zBusy = ((*tclStubsPtr).tcl_GetStringFromObj)
                        .expect(
                            "non-null function pointer",
                        )(*objv.offset(2 as libc::c_int as isize), &mut len_1);
                    if !zBusy.is_null() && len_1 > 0 as libc::c_int {
                        let ref mut fresh132 = (*pDb).zBusy;
                        *fresh132 = ((*tclStubsPtr).tcl_Alloc)
                            .expect(
                                "non-null function pointer",
                            )((len_1 + 1 as libc::c_int) as libc::c_uint);
                        memcpy(
                            (*pDb).zBusy as *mut libc::c_void,
                            zBusy as *const libc::c_void,
                            (len_1 + 1 as libc::c_int) as libc::c_ulong,
                        );
                    } else {
                        let ref mut fresh133 = (*pDb).zBusy;
                        *fresh133 = 0 as *mut libc::c_char;
                    }
                    if !((*pDb).zBusy).is_null() {
                        let ref mut fresh134 = (*pDb).interp;
                        *fresh134 = interp;
                        sqlite3_busy_handler(
                            (*pDb).db,
                            Some(
                                DbBusyHandler
                                    as unsafe extern "C" fn(
                                        *mut libc::c_void,
                                        libc::c_int,
                                    ) -> libc::c_int,
                            ),
                            pDb as *mut libc::c_void,
                        );
                    } else {
                        sqlite3_busy_handler((*pDb).db, None, 0 as *mut libc::c_void);
                    }
                }
            }
        }
        4 => {
            let mut subCmd: *mut libc::c_char = 0 as *mut libc::c_char;
            let mut n: libc::c_int = 0;
            if objc <= 2 as libc::c_int {
                ((*tclStubsPtr).tcl_WrongNumArgs)
                    .expect(
                        "non-null function pointer",
                    )(
                    interp,
                    1 as libc::c_int,
                    objv,
                    b"cache option ?arg?\0" as *const u8 as *const libc::c_char,
                );
                return 1 as libc::c_int;
            }
            subCmd = ((*tclStubsPtr).tcl_GetStringFromObj)
                .expect(
                    "non-null function pointer",
                )(*objv.offset(2 as libc::c_int as isize), 0 as *mut libc::c_int);
            if *subCmd as libc::c_int == 'f' as i32
                && strcmp(subCmd, b"flush\0" as *const u8 as *const libc::c_char)
                    == 0 as libc::c_int
            {
                if objc != 3 as libc::c_int {
                    ((*tclStubsPtr).tcl_WrongNumArgs)
                        .expect(
                            "non-null function pointer",
                        )(
                        interp,
                        2 as libc::c_int,
                        objv,
                        b"flush\0" as *const u8 as *const libc::c_char,
                    );
                    return 1 as libc::c_int;
                } else {
                    flushStmtCache(pDb);
                }
            } else if *subCmd as libc::c_int == 's' as i32
                && strcmp(subCmd, b"size\0" as *const u8 as *const libc::c_char)
                    == 0 as libc::c_int
            {
                if objc != 4 as libc::c_int {
                    ((*tclStubsPtr).tcl_WrongNumArgs)
                        .expect(
                            "non-null function pointer",
                        )(
                        interp,
                        2 as libc::c_int,
                        objv,
                        b"size n\0" as *const u8 as *const libc::c_char,
                    );
                    return 1 as libc::c_int;
                } else {
                    if 1 as libc::c_int
                        == ((*tclStubsPtr).tcl_GetIntFromObj)
                            .expect(
                                "non-null function pointer",
                            )(interp, *objv.offset(3 as libc::c_int as isize), &mut n)
                    {
                        ((*tclStubsPtr).tcl_AppendResult)
                            .expect(
                                "non-null function pointer",
                            )(
                            interp,
                            b"cannot convert \"\0" as *const u8 as *const libc::c_char,
                            ((*tclStubsPtr).tcl_GetStringFromObj)
                                .expect(
                                    "non-null function pointer",
                                )(
                                *objv.offset(3 as libc::c_int as isize),
                                0 as *mut libc::c_int,
                            ),
                            b"\" to integer\0" as *const u8 as *const libc::c_char,
                            0 as *mut libc::c_char,
                        );
                        return 1 as libc::c_int;
                    } else {
                        if n < 0 as libc::c_int {
                            flushStmtCache(pDb);
                            n = 0 as libc::c_int;
                        } else if n > 100 as libc::c_int {
                            n = 100 as libc::c_int;
                        }
                        (*pDb).maxStmt = n;
                    }
                }
            } else {
                ((*tclStubsPtr).tcl_AppendResult)
                    .expect(
                        "non-null function pointer",
                    )(
                    interp,
                    b"bad option \"\0" as *const u8 as *const libc::c_char,
                    ((*tclStubsPtr).tcl_GetStringFromObj)
                        .expect(
                            "non-null function pointer",
                        )(
                        *objv.offset(2 as libc::c_int as isize),
                        0 as *mut libc::c_int,
                    ),
                    b"\": must be flush or size\0" as *const u8 as *const libc::c_char,
                    0 as *mut libc::c_char,
                );
                return 1 as libc::c_int;
            }
        }
        5 => {
            let mut pResult: *mut Tcl_Obj = 0 as *mut Tcl_Obj;
            if objc != 2 as libc::c_int {
                ((*tclStubsPtr).tcl_WrongNumArgs)
                    .expect(
                        "non-null function pointer",
                    )(
                    interp,
                    2 as libc::c_int,
                    objv,
                    b"\0" as *const u8 as *const libc::c_char,
                );
                return 1 as libc::c_int;
            }
            pResult = ((*tclStubsPtr).tcl_GetObjResult)
                .expect("non-null function pointer")(interp);
            ((*tclStubsPtr).tcl_SetWideIntObj)
                .expect(
                    "non-null function pointer",
                )(pResult, sqlite3_changes64((*pDb).db));
        }
        6 => {
            ((*tclStubsPtr).tcl_DeleteCommand)
                .expect(
                    "non-null function pointer",
                )(
                interp,
                ((*tclStubsPtr).tcl_GetStringFromObj)
                    .expect(
                        "non-null function pointer",
                    )(*objv.offset(0 as libc::c_int as isize), 0 as *mut libc::c_int),
            );
        }
        7 => {
            let mut pCollate: *mut SqlCollate = 0 as *mut SqlCollate;
            let mut zName: *mut libc::c_char = 0 as *mut libc::c_char;
            let mut zScript: *mut libc::c_char = 0 as *mut libc::c_char;
            let mut nScript: libc::c_int = 0;
            if objc != 4 as libc::c_int {
                ((*tclStubsPtr).tcl_WrongNumArgs)
                    .expect(
                        "non-null function pointer",
                    )(
                    interp,
                    2 as libc::c_int,
                    objv,
                    b"NAME SCRIPT\0" as *const u8 as *const libc::c_char,
                );
                return 1 as libc::c_int;
            }
            zName = ((*tclStubsPtr).tcl_GetStringFromObj)
                .expect(
                    "non-null function pointer",
                )(*objv.offset(2 as libc::c_int as isize), 0 as *mut libc::c_int);
            zScript = ((*tclStubsPtr).tcl_GetStringFromObj)
                .expect(
                    "non-null function pointer",
                )(*objv.offset(3 as libc::c_int as isize), &mut nScript);
            pCollate = ((*tclStubsPtr).tcl_Alloc)
                .expect(
                    "non-null function pointer",
                )(
                (::std::mem::size_of::<SqlCollate>() as libc::c_ulong)
                    .wrapping_add(nScript as libc::c_ulong)
                    .wrapping_add(1 as libc::c_int as libc::c_ulong) as libc::c_uint,
            ) as *mut SqlCollate;
            if pCollate.is_null() {
                return 1 as libc::c_int;
            }
            let ref mut fresh135 = (*pCollate).interp;
            *fresh135 = interp;
            let ref mut fresh136 = (*pCollate).pNext;
            *fresh136 = (*pDb).pCollate;
            let ref mut fresh137 = (*pCollate).zScript;
            *fresh137 = &mut *pCollate.offset(1 as libc::c_int as isize)
                as *mut SqlCollate as *mut libc::c_char;
            let ref mut fresh138 = (*pDb).pCollate;
            *fresh138 = pCollate;
            memcpy(
                (*pCollate).zScript as *mut libc::c_void,
                zScript as *const libc::c_void,
                (nScript + 1 as libc::c_int) as libc::c_ulong,
            );
            if sqlite3_create_collation(
                (*pDb).db,
                zName,
                1 as libc::c_int,
                pCollate as *mut libc::c_void,
                Some(
                    tclSqlCollate
                        as unsafe extern "C" fn(
                            *mut libc::c_void,
                            libc::c_int,
                            *const libc::c_void,
                            libc::c_int,
                            *const libc::c_void,
                        ) -> libc::c_int,
                ),
            ) != 0
            {
                ((*tclStubsPtr).tcl_SetResult)
                    .expect(
                        "non-null function pointer",
                    )(
                    interp,
                    sqlite3_errmsg((*pDb).db) as *mut libc::c_char,
                    ::std::mem::transmute::<
                        libc::intptr_t,
                        Option::<Tcl_FreeProc>,
                    >(1 as libc::c_int as libc::intptr_t),
                );
                return 1 as libc::c_int;
            }
        }
        8 => {
            if objc != 3 as libc::c_int {
                ((*tclStubsPtr).tcl_WrongNumArgs)
                    .expect(
                        "non-null function pointer",
                    )(
                    interp,
                    2 as libc::c_int,
                    objv,
                    b"SCRIPT\0" as *const u8 as *const libc::c_char,
                );
                return 1 as libc::c_int;
            }
            if !((*pDb).pCollateNeeded).is_null() {
                let mut _objPtr: *mut Tcl_Obj = (*pDb).pCollateNeeded;
                let ref mut fresh139 = (*_objPtr).refCount;
                let fresh140 = *fresh139;
                *fresh139 = *fresh139 - 1;
                if fresh140 <= 1 as libc::c_int {
                    ((*tclStubsPtr).tclFreeObj)
                        .expect("non-null function pointer")(_objPtr);
                }
            }
            let ref mut fresh141 = (*pDb).pCollateNeeded;
            *fresh141 = ((*tclStubsPtr).tcl_DuplicateObj)
                .expect(
                    "non-null function pointer",
                )(*objv.offset(2 as libc::c_int as isize));
            let ref mut fresh142 = (*(*pDb).pCollateNeeded).refCount;
            *fresh142 += 1;
            sqlite3_collation_needed(
                (*pDb).db,
                pDb as *mut libc::c_void,
                Some(
                    tclCollateNeeded
                        as unsafe extern "C" fn(
                            *mut libc::c_void,
                            *mut sqlite3,
                            libc::c_int,
                            *const libc::c_char,
                        ) -> (),
                ),
            );
        }
        9 => {
            if objc > 3 as libc::c_int {
                ((*tclStubsPtr).tcl_WrongNumArgs)
                    .expect(
                        "non-null function pointer",
                    )(
                    interp,
                    2 as libc::c_int,
                    objv,
                    b"?CALLBACK?\0" as *const u8 as *const libc::c_char,
                );
                return 1 as libc::c_int;
            } else {
                if objc == 2 as libc::c_int {
                    if !((*pDb).zCommit).is_null() {
                        ((*tclStubsPtr).tcl_AppendResult)
                            .expect(
                                "non-null function pointer",
                            )(interp, (*pDb).zCommit, 0 as *mut libc::c_char);
                    }
                } else {
                    let mut zCommit: *const libc::c_char = 0 as *const libc::c_char;
                    let mut len_2: libc::c_int = 0;
                    if !((*pDb).zCommit).is_null() {
                        ((*tclStubsPtr).tcl_Free)
                            .expect("non-null function pointer")((*pDb).zCommit);
                    }
                    zCommit = ((*tclStubsPtr).tcl_GetStringFromObj)
                        .expect(
                            "non-null function pointer",
                        )(*objv.offset(2 as libc::c_int as isize), &mut len_2);
                    if !zCommit.is_null() && len_2 > 0 as libc::c_int {
                        let ref mut fresh143 = (*pDb).zCommit;
                        *fresh143 = ((*tclStubsPtr).tcl_Alloc)
                            .expect(
                                "non-null function pointer",
                            )((len_2 + 1 as libc::c_int) as libc::c_uint);
                        memcpy(
                            (*pDb).zCommit as *mut libc::c_void,
                            zCommit as *const libc::c_void,
                            (len_2 + 1 as libc::c_int) as libc::c_ulong,
                        );
                    } else {
                        let ref mut fresh144 = (*pDb).zCommit;
                        *fresh144 = 0 as *mut libc::c_char;
                    }
                    if !((*pDb).zCommit).is_null() {
                        let ref mut fresh145 = (*pDb).interp;
                        *fresh145 = interp;
                        sqlite3_commit_hook(
                            (*pDb).db,
                            Some(
                                DbCommitHandler
                                    as unsafe extern "C" fn(*mut libc::c_void) -> libc::c_int,
                            ),
                            pDb as *mut libc::c_void,
                        );
                    } else {
                        sqlite3_commit_hook((*pDb).db, None, 0 as *mut libc::c_void);
                    }
                }
            }
        }
        10 => {
            let mut pResult_0: *mut Tcl_Obj = 0 as *mut Tcl_Obj;
            let mut isComplete: libc::c_int = 0;
            if objc != 3 as libc::c_int {
                ((*tclStubsPtr).tcl_WrongNumArgs)
                    .expect(
                        "non-null function pointer",
                    )(
                    interp,
                    2 as libc::c_int,
                    objv,
                    b"SQL\0" as *const u8 as *const libc::c_char,
                );
                return 1 as libc::c_int;
            }
            isComplete = sqlite3_complete(
                ((*tclStubsPtr).tcl_GetStringFromObj)
                    .expect(
                        "non-null function pointer",
                    )(*objv.offset(2 as libc::c_int as isize), 0 as *mut libc::c_int),
            );
            pResult_0 = ((*tclStubsPtr).tcl_GetObjResult)
                .expect("non-null function pointer")(interp);
            ((*tclStubsPtr).tcl_SetIntObj)
                .expect(
                    "non-null function pointer",
                )(pResult_0, (isComplete != 0 as libc::c_int) as libc::c_int);
        }
        11 => {
            static mut aDbConfig: [DbConfigChoices; 16] = [
                {
                    let mut init = DbConfigChoices {
                        zName: b"defensive\0" as *const u8 as *const libc::c_char,
                        op: 1010 as libc::c_int,
                    };
                    init
                },
                {
                    let mut init = DbConfigChoices {
                        zName: b"dqs_ddl\0" as *const u8 as *const libc::c_char,
                        op: 1014 as libc::c_int,
                    };
                    init
                },
                {
                    let mut init = DbConfigChoices {
                        zName: b"dqs_dml\0" as *const u8 as *const libc::c_char,
                        op: 1013 as libc::c_int,
                    };
                    init
                },
                {
                    let mut init = DbConfigChoices {
                        zName: b"enable_fkey\0" as *const u8 as *const libc::c_char,
                        op: 1002 as libc::c_int,
                    };
                    init
                },
                {
                    let mut init = DbConfigChoices {
                        zName: b"enable_qpsg\0" as *const u8 as *const libc::c_char,
                        op: 1007 as libc::c_int,
                    };
                    init
                },
                {
                    let mut init = DbConfigChoices {
                        zName: b"enable_trigger\0" as *const u8 as *const libc::c_char,
                        op: 1003 as libc::c_int,
                    };
                    init
                },
                {
                    let mut init = DbConfigChoices {
                        zName: b"enable_view\0" as *const u8 as *const libc::c_char,
                        op: 1015 as libc::c_int,
                    };
                    init
                },
                {
                    let mut init = DbConfigChoices {
                        zName: b"fts3_tokenizer\0" as *const u8 as *const libc::c_char,
                        op: 1004 as libc::c_int,
                    };
                    init
                },
                {
                    let mut init = DbConfigChoices {
                        zName: b"legacy_alter_table\0" as *const u8
                            as *const libc::c_char,
                        op: 1012 as libc::c_int,
                    };
                    init
                },
                {
                    let mut init = DbConfigChoices {
                        zName: b"legacy_file_format\0" as *const u8
                            as *const libc::c_char,
                        op: 1016 as libc::c_int,
                    };
                    init
                },
                {
                    let mut init = DbConfigChoices {
                        zName: b"load_extension\0" as *const u8 as *const libc::c_char,
                        op: 1005 as libc::c_int,
                    };
                    init
                },
                {
                    let mut init = DbConfigChoices {
                        zName: b"no_ckpt_on_close\0" as *const u8 as *const libc::c_char,
                        op: 1006 as libc::c_int,
                    };
                    init
                },
                {
                    let mut init = DbConfigChoices {
                        zName: b"reset_database\0" as *const u8 as *const libc::c_char,
                        op: 1009 as libc::c_int,
                    };
                    init
                },
                {
                    let mut init = DbConfigChoices {
                        zName: b"trigger_eqp\0" as *const u8 as *const libc::c_char,
                        op: 1008 as libc::c_int,
                    };
                    init
                },
                {
                    let mut init = DbConfigChoices {
                        zName: b"trusted_schema\0" as *const u8 as *const libc::c_char,
                        op: 1017 as libc::c_int,
                    };
                    init
                },
                {
                    let mut init = DbConfigChoices {
                        zName: b"writable_schema\0" as *const u8 as *const libc::c_char,
                        op: 1011 as libc::c_int,
                    };
                    init
                },
            ];
            let mut pResult_1: *mut Tcl_Obj = 0 as *mut Tcl_Obj;
            let mut ii: libc::c_int = 0;
            if objc > 4 as libc::c_int {
                ((*tclStubsPtr).tcl_WrongNumArgs)
                    .expect(
                        "non-null function pointer",
                    )(
                    interp,
                    2 as libc::c_int,
                    objv,
                    b"?OPTION? ?BOOLEAN?\0" as *const u8 as *const libc::c_char,
                );
                return 1 as libc::c_int;
            }
            if objc == 2 as libc::c_int {
                pResult_1 = ((*tclStubsPtr).tcl_NewListObj)
                    .expect(
                        "non-null function pointer",
                    )(0 as libc::c_int, 0 as *const *mut Tcl_Obj);
                ii = 0 as libc::c_int;
                while (ii as libc::c_ulong)
                    < (::std::mem::size_of::<[DbConfigChoices; 16]>() as libc::c_ulong)
                        .wrapping_div(
                            ::std::mem::size_of::<DbConfigChoices>() as libc::c_ulong,
                        )
                {
                    let mut v: libc::c_int = 0 as libc::c_int;
                    sqlite3_db_config(
                        (*pDb).db,
                        aDbConfig[ii as usize].op,
                        -(1 as libc::c_int),
                        &mut v as *mut libc::c_int,
                    );
                    ((*tclStubsPtr).tcl_ListObjAppendElement)
                        .expect(
                            "non-null function pointer",
                        )(
                        interp,
                        pResult_1,
                        ((*tclStubsPtr).tcl_NewStringObj)
                            .expect(
                                "non-null function pointer",
                            )(aDbConfig[ii as usize].zName, -(1 as libc::c_int)),
                    );
                    ((*tclStubsPtr).tcl_ListObjAppendElement)
                        .expect(
                            "non-null function pointer",
                        )(
                        interp,
                        pResult_1,
                        ((*tclStubsPtr).tcl_NewIntObj)
                            .expect("non-null function pointer")(v),
                    );
                    ii += 1;
                }
            } else {
                let mut zOpt: *const libc::c_char = ((*tclStubsPtr).tcl_GetString)
                    .expect(
                        "non-null function pointer",
                    )(*objv.offset(2 as libc::c_int as isize));
                let mut onoff: libc::c_int = -(1 as libc::c_int);
                let mut v_0: libc::c_int = 0 as libc::c_int;
                if *zOpt.offset(0 as libc::c_int as isize) as libc::c_int == '-' as i32 {
                    zOpt = zOpt.offset(1);
                }
                ii = 0 as libc::c_int;
                while (ii as libc::c_ulong)
                    < (::std::mem::size_of::<[DbConfigChoices; 16]>() as libc::c_ulong)
                        .wrapping_div(
                            ::std::mem::size_of::<DbConfigChoices>() as libc::c_ulong,
                        )
                {
                    if strcmp(aDbConfig[ii as usize].zName, zOpt) == 0 as libc::c_int {
                        break;
                    }
                    ii += 1;
                }
                if ii as libc::c_ulong
                    >= (::std::mem::size_of::<[DbConfigChoices; 16]>() as libc::c_ulong)
                        .wrapping_div(
                            ::std::mem::size_of::<DbConfigChoices>() as libc::c_ulong,
                        )
                {
                    ((*tclStubsPtr).tcl_AppendResult)
                        .expect(
                            "non-null function pointer",
                        )(
                        interp,
                        b"unknown config option: \"\0" as *const u8
                            as *const libc::c_char,
                        zOpt,
                        b"\"\0" as *const u8 as *const libc::c_char,
                        0 as *mut libc::c_void,
                    );
                    return 1 as libc::c_int;
                }
                if objc == 4 as libc::c_int {
                    if ((*tclStubsPtr).tcl_GetBooleanFromObj)
                        .expect(
                            "non-null function pointer",
                        )(interp, *objv.offset(3 as libc::c_int as isize), &mut onoff)
                        != 0
                    {
                        return 1 as libc::c_int;
                    }
                }
                sqlite3_db_config(
                    (*pDb).db,
                    aDbConfig[ii as usize].op,
                    onoff,
                    &mut v_0 as *mut libc::c_int,
                );
                pResult_1 = ((*tclStubsPtr).tcl_NewIntObj)
                    .expect("non-null function pointer")(v_0);
            }
            ((*tclStubsPtr).tcl_SetObjResult)
                .expect("non-null function pointer")(interp, pResult_1);
        }
        12 => {
            let mut zTable: *mut libc::c_char = 0 as *mut libc::c_char;
            let mut zFile: *mut libc::c_char = 0 as *mut libc::c_char;
            let mut zConflict: *mut libc::c_char = 0 as *mut libc::c_char;
            let mut pStmt: *mut sqlite3_stmt = 0 as *mut sqlite3_stmt;
            let mut nCol: libc::c_int = 0;
            let mut nByte: libc::c_int = 0;
            let mut i: libc::c_int = 0;
            let mut j: libc::c_int = 0;
            let mut nSep: libc::c_int = 0;
            let mut nNull: libc::c_int = 0;
            let mut zSql: *mut libc::c_char = 0 as *mut libc::c_char;
            let mut zLine: *mut libc::c_char = 0 as *mut libc::c_char;
            let mut azCol: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
            let mut zCommit_0: *const libc::c_char = 0 as *const libc::c_char;
            let mut in_0: *mut FILE = 0 as *mut FILE;
            let mut lineno: libc::c_int = 0 as libc::c_int;
            let mut zLineNum: [libc::c_char; 80] = [0; 80];
            let mut pResult_2: *mut Tcl_Obj = 0 as *mut Tcl_Obj;
            let mut zSep: *const libc::c_char = 0 as *const libc::c_char;
            let mut zNull: *const libc::c_char = 0 as *const libc::c_char;
            if objc < 5 as libc::c_int || objc > 7 as libc::c_int {
                ((*tclStubsPtr).tcl_WrongNumArgs)
                    .expect(
                        "non-null function pointer",
                    )(
                    interp,
                    2 as libc::c_int,
                    objv,
                    b"CONFLICT-ALGORITHM TABLE FILENAME ?SEPARATOR? ?NULLINDICATOR?\0"
                        as *const u8 as *const libc::c_char,
                );
                return 1 as libc::c_int;
            }
            if objc >= 6 as libc::c_int {
                zSep = ((*tclStubsPtr).tcl_GetStringFromObj)
                    .expect(
                        "non-null function pointer",
                    )(*objv.offset(5 as libc::c_int as isize), 0 as *mut libc::c_int);
            } else {
                zSep = b"\t\0" as *const u8 as *const libc::c_char;
            }
            if objc >= 7 as libc::c_int {
                zNull = ((*tclStubsPtr).tcl_GetStringFromObj)
                    .expect(
                        "non-null function pointer",
                    )(*objv.offset(6 as libc::c_int as isize), 0 as *mut libc::c_int);
            } else {
                zNull = b"\0" as *const u8 as *const libc::c_char;
            }
            zConflict = ((*tclStubsPtr).tcl_GetStringFromObj)
                .expect(
                    "non-null function pointer",
                )(*objv.offset(2 as libc::c_int as isize), 0 as *mut libc::c_int);
            zTable = ((*tclStubsPtr).tcl_GetStringFromObj)
                .expect(
                    "non-null function pointer",
                )(*objv.offset(3 as libc::c_int as isize), 0 as *mut libc::c_int);
            zFile = ((*tclStubsPtr).tcl_GetStringFromObj)
                .expect(
                    "non-null function pointer",
                )(*objv.offset(4 as libc::c_int as isize), 0 as *mut libc::c_int);
            nSep = strlen30(zSep);
            nNull = strlen30(zNull);
            if nSep == 0 as libc::c_int {
                ((*tclStubsPtr).tcl_AppendResult)
                    .expect(
                        "non-null function pointer",
                    )(
                    interp,
                    b"Error: non-null separator required for copy\0" as *const u8
                        as *const libc::c_char,
                    0 as *mut libc::c_char,
                );
                return 1 as libc::c_int;
            }
            if strcmp(zConflict, b"rollback\0" as *const u8 as *const libc::c_char)
                != 0 as libc::c_int
                && strcmp(zConflict, b"abort\0" as *const u8 as *const libc::c_char)
                    != 0 as libc::c_int
                && strcmp(zConflict, b"fail\0" as *const u8 as *const libc::c_char)
                    != 0 as libc::c_int
                && strcmp(zConflict, b"ignore\0" as *const u8 as *const libc::c_char)
                    != 0 as libc::c_int
                && strcmp(zConflict, b"replace\0" as *const u8 as *const libc::c_char)
                    != 0 as libc::c_int
            {
                ((*tclStubsPtr).tcl_AppendResult)
                    .expect(
                        "non-null function pointer",
                    )(
                    interp,
                    b"Error: \"\0" as *const u8 as *const libc::c_char,
                    zConflict,
                    b"\", conflict-algorithm must be one of: rollback, abort, fail, ignore, or replace\0"
                        as *const u8 as *const libc::c_char,
                    0 as *mut libc::c_char,
                );
                return 1 as libc::c_int;
            }
            zSql = sqlite3_mprintf(
                b"SELECT * FROM '%q'\0" as *const u8 as *const libc::c_char,
                zTable,
            );
            if zSql.is_null() {
                ((*tclStubsPtr).tcl_AppendResult)
                    .expect(
                        "non-null function pointer",
                    )(
                    interp,
                    b"Error: no such table: \0" as *const u8 as *const libc::c_char,
                    zTable,
                    0 as *mut libc::c_char,
                );
                return 1 as libc::c_int;
            }
            nByte = strlen30(zSql);
            rc = sqlite3_prepare(
                (*pDb).db,
                zSql,
                -(1 as libc::c_int),
                &mut pStmt,
                0 as *mut *const libc::c_char,
            );
            sqlite3_free(zSql as *mut libc::c_void);
            if rc != 0 {
                ((*tclStubsPtr).tcl_AppendResult)
                    .expect(
                        "non-null function pointer",
                    )(
                    interp,
                    b"Error: \0" as *const u8 as *const libc::c_char,
                    sqlite3_errmsg((*pDb).db),
                    0 as *mut libc::c_char,
                );
                nCol = 0 as libc::c_int;
            } else {
                nCol = sqlite3_column_count(pStmt);
            }
            sqlite3_finalize(pStmt);
            if nCol == 0 as libc::c_int {
                return 1 as libc::c_int;
            }
            zSql = malloc(
                (nByte + 50 as libc::c_int + nCol * 2 as libc::c_int) as libc::c_ulong,
            ) as *mut libc::c_char;
            if zSql.is_null() {
                ((*tclStubsPtr).tcl_AppendResult)
                    .expect(
                        "non-null function pointer",
                    )(
                    interp,
                    b"Error: can't malloc()\0" as *const u8 as *const libc::c_char,
                    0 as *mut libc::c_char,
                );
                return 1 as libc::c_int;
            }
            sqlite3_snprintf(
                nByte + 50 as libc::c_int,
                zSql,
                b"INSERT OR %q INTO '%q' VALUES(?\0" as *const u8 as *const libc::c_char,
                zConflict,
                zTable,
            );
            j = strlen30(zSql);
            i = 1 as libc::c_int;
            while i < nCol {
                let fresh146 = j;
                j = j + 1;
                *zSql.offset(fresh146 as isize) = ',' as i32 as libc::c_char;
                let fresh147 = j;
                j = j + 1;
                *zSql.offset(fresh147 as isize) = '?' as i32 as libc::c_char;
                i += 1;
            }
            let fresh148 = j;
            j = j + 1;
            *zSql.offset(fresh148 as isize) = ')' as i32 as libc::c_char;
            *zSql.offset(j as isize) = 0 as libc::c_int as libc::c_char;
            rc = sqlite3_prepare(
                (*pDb).db,
                zSql,
                -(1 as libc::c_int),
                &mut pStmt,
                0 as *mut *const libc::c_char,
            );
            free(zSql as *mut libc::c_void);
            if rc != 0 {
                ((*tclStubsPtr).tcl_AppendResult)
                    .expect(
                        "non-null function pointer",
                    )(
                    interp,
                    b"Error: \0" as *const u8 as *const libc::c_char,
                    sqlite3_errmsg((*pDb).db),
                    0 as *mut libc::c_char,
                );
                sqlite3_finalize(pStmt);
                return 1 as libc::c_int;
            }
            in_0 = fopen(zFile, b"rb\0" as *const u8 as *const libc::c_char);
            if in_0.is_null() {
                ((*tclStubsPtr).tcl_AppendResult)
                    .expect(
                        "non-null function pointer",
                    )(
                    interp,
                    b"Error: cannot open file: \0" as *const u8 as *const libc::c_char,
                    zFile,
                    0 as *mut libc::c_char,
                );
                sqlite3_finalize(pStmt);
                return 1 as libc::c_int;
            }
            azCol = malloc(
                (::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong)
                    .wrapping_mul((nCol + 1 as libc::c_int) as libc::c_ulong),
            ) as *mut *mut libc::c_char;
            if azCol.is_null() {
                ((*tclStubsPtr).tcl_AppendResult)
                    .expect(
                        "non-null function pointer",
                    )(
                    interp,
                    b"Error: can't malloc()\0" as *const u8 as *const libc::c_char,
                    0 as *mut libc::c_char,
                );
                fclose(in_0);
                return 1 as libc::c_int;
            }
            sqlite3_exec(
                (*pDb).db,
                b"BEGIN\0" as *const u8 as *const libc::c_char,
                None,
                0 as *mut libc::c_void,
                0 as *mut *mut libc::c_char,
            );
            zCommit_0 = b"COMMIT\0" as *const u8 as *const libc::c_char;
            loop {
                zLine = local_getline(0 as *mut libc::c_char, in_0);
                if zLine.is_null() {
                    break;
                }
                let mut z: *mut libc::c_char = 0 as *mut libc::c_char;
                lineno += 1;
                let ref mut fresh149 = *azCol.offset(0 as libc::c_int as isize);
                *fresh149 = zLine;
                i = 0 as libc::c_int;
                z = zLine;
                while *z != 0 {
                    if *z as libc::c_int
                        == *zSep.offset(0 as libc::c_int as isize) as libc::c_int
                        && strncmp(z, zSep, nSep as libc::c_ulong) == 0 as libc::c_int
                    {
                        *z = 0 as libc::c_int as libc::c_char;
                        i += 1;
                        if i < nCol {
                            let ref mut fresh150 = *azCol.offset(i as isize);
                            *fresh150 = &mut *z.offset(nSep as isize)
                                as *mut libc::c_char;
                            z = z.offset((nSep - 1 as libc::c_int) as isize);
                        }
                    }
                    z = z.offset(1);
                }
                if i + 1 as libc::c_int != nCol {
                    let mut zErr: *mut libc::c_char = 0 as *mut libc::c_char;
                    let mut nErr: libc::c_int = strlen30(zFile) + 200 as libc::c_int;
                    zErr = malloc(nErr as libc::c_ulong) as *mut libc::c_char;
                    if !zErr.is_null() {
                        sqlite3_snprintf(
                            nErr,
                            zErr,
                            b"Error: %s line %d: expected %d columns of data but found %d\0"
                                as *const u8 as *const libc::c_char,
                            zFile,
                            lineno,
                            nCol,
                            i + 1 as libc::c_int,
                        );
                        ((*tclStubsPtr).tcl_AppendResult)
                            .expect(
                                "non-null function pointer",
                            )(interp, zErr, 0 as *mut libc::c_char);
                        free(zErr as *mut libc::c_void);
                    }
                    zCommit_0 = b"ROLLBACK\0" as *const u8 as *const libc::c_char;
                    break;
                } else {
                    i = 0 as libc::c_int;
                    while i < nCol {
                        if nNull > 0 as libc::c_int
                            && strcmp(*azCol.offset(i as isize), zNull)
                                == 0 as libc::c_int
                            || strlen30(*azCol.offset(i as isize)) == 0 as libc::c_int
                        {
                            sqlite3_bind_null(pStmt, i + 1 as libc::c_int);
                        } else {
                            sqlite3_bind_text(
                                pStmt,
                                i + 1 as libc::c_int,
                                *azCol.offset(i as isize),
                                -(1 as libc::c_int),
                                None,
                            );
                        }
                        i += 1;
                    }
                    sqlite3_step(pStmt);
                    rc = sqlite3_reset(pStmt);
                    free(zLine as *mut libc::c_void);
                    if !(rc != 0 as libc::c_int) {
                        continue;
                    }
                    ((*tclStubsPtr).tcl_AppendResult)
                        .expect(
                            "non-null function pointer",
                        )(
                        interp,
                        b"Error: \0" as *const u8 as *const libc::c_char,
                        sqlite3_errmsg((*pDb).db),
                        0 as *mut libc::c_char,
                    );
                    zCommit_0 = b"ROLLBACK\0" as *const u8 as *const libc::c_char;
                    break;
                }
            }
            free(azCol as *mut libc::c_void);
            fclose(in_0);
            sqlite3_finalize(pStmt);
            sqlite3_exec(
                (*pDb).db,
                zCommit_0,
                None,
                0 as *mut libc::c_void,
                0 as *mut *mut libc::c_char,
            );
            if *zCommit_0.offset(0 as libc::c_int as isize) as libc::c_int == 'C' as i32
            {
                pResult_2 = ((*tclStubsPtr).tcl_GetObjResult)
                    .expect("non-null function pointer")(interp);
                ((*tclStubsPtr).tcl_SetIntObj)
                    .expect("non-null function pointer")(pResult_2, lineno);
                rc = 0 as libc::c_int;
            } else {
                sqlite3_snprintf(
                    ::std::mem::size_of::<[libc::c_char; 80]>() as libc::c_ulong
                        as libc::c_int,
                    zLineNum.as_mut_ptr(),
                    b"%d\0" as *const u8 as *const libc::c_char,
                    lineno,
                );
                ((*tclStubsPtr).tcl_AppendResult)
                    .expect(
                        "non-null function pointer",
                    )(
                    interp,
                    b", failed while processing line: \0" as *const u8
                        as *const libc::c_char,
                    zLineNum.as_mut_ptr(),
                    0 as *mut libc::c_char,
                );
                rc = 1 as libc::c_int;
            }
        }
        13 => {
            let mut zSchema: *const libc::c_char = 0 as *const libc::c_char;
            let mut pValue: *mut Tcl_Obj = 0 as *mut Tcl_Obj;
            let mut pBA: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
            let mut pData: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
            let mut len_3: libc::c_int = 0;
            let mut xrc: libc::c_int = 0;
            let mut mxSize: sqlite3_int64 = 0 as libc::c_int as sqlite3_int64;
            let mut i_0: libc::c_int = 0;
            let mut isReadonly: libc::c_int = 0 as libc::c_int;
            if objc < 3 as libc::c_int {
                ((*tclStubsPtr).tcl_WrongNumArgs)
                    .expect(
                        "non-null function pointer",
                    )(
                    interp,
                    2 as libc::c_int,
                    objv,
                    b"?DATABASE? VALUE\0" as *const u8 as *const libc::c_char,
                );
                rc = 1 as libc::c_int;
            } else {
                i_0 = 2 as libc::c_int;
                loop {
                    if !(i_0 < objc - 1 as libc::c_int) {
                        current_block = 4616860616151412918;
                        break;
                    }
                    let mut z_0: *const libc::c_char = ((*tclStubsPtr).tcl_GetString)
                        .expect("non-null function pointer")(*objv.offset(i_0 as isize));
                    if strcmp(z_0, b"-maxsize\0" as *const u8 as *const libc::c_char)
                        == 0 as libc::c_int && i_0 < objc - 2 as libc::c_int
                    {
                        let mut x: Tcl_WideInt = 0;
                        i_0 += 1;
                        rc = ((*tclStubsPtr).tcl_GetWideIntFromObj)
                            .expect(
                                "non-null function pointer",
                            )(interp, *objv.offset(i_0 as isize), &mut x);
                        if rc != 0 {
                            current_block = 12660005878960618547;
                            break;
                        }
                        mxSize = x;
                    } else if strcmp(
                        z_0,
                        b"-readonly\0" as *const u8 as *const libc::c_char,
                    ) == 0 as libc::c_int && i_0 < objc - 2 as libc::c_int
                    {
                        i_0 += 1;
                        rc = ((*tclStubsPtr).tcl_GetBooleanFromObj)
                            .expect(
                                "non-null function pointer",
                            )(interp, *objv.offset(i_0 as isize), &mut isReadonly);
                        if rc != 0 {
                            current_block = 12660005878960618547;
                            break;
                        }
                    } else if zSchema.is_null() && i_0 == objc - 2 as libc::c_int
                        && *z_0.offset(0 as libc::c_int as isize) as libc::c_int
                            != '-' as i32
                    {
                        zSchema = z_0;
                    } else {
                        ((*tclStubsPtr).tcl_AppendResult)
                            .expect(
                                "non-null function pointer",
                            )(
                            interp,
                            b"unknown option: \0" as *const u8 as *const libc::c_char,
                            z_0,
                            0 as *mut libc::c_char,
                        );
                        rc = 1 as libc::c_int;
                        current_block = 12660005878960618547;
                        break;
                    }
                    i_0 += 1;
                }
                match current_block {
                    12660005878960618547 => {}
                    _ => {
                        pValue = *objv.offset((objc - 1 as libc::c_int) as isize);
                        pBA = ((*tclStubsPtr).tcl_GetByteArrayFromObj)
                            .expect("non-null function pointer")(pValue, &mut len_3);
                        pData = sqlite3_malloc64(len_3 as sqlite3_uint64)
                            as *mut libc::c_uchar;
                        if pData.is_null() && len_3 > 0 as libc::c_int {
                            ((*tclStubsPtr).tcl_AppendResult)
                                .expect(
                                    "non-null function pointer",
                                )(
                                interp,
                                b"out of memory\0" as *const u8 as *const libc::c_char,
                                0 as *mut libc::c_char,
                            );
                            rc = 1 as libc::c_int;
                        } else {
                            let mut flags: libc::c_int = 0;
                            if len_3 > 0 as libc::c_int {
                                memcpy(
                                    pData as *mut libc::c_void,
                                    pBA as *const libc::c_void,
                                    len_3 as libc::c_ulong,
                                );
                            }
                            if isReadonly != 0 {
                                flags = 1 as libc::c_int | 4 as libc::c_int;
                            } else {
                                flags = 1 as libc::c_int | 2 as libc::c_int;
                            }
                            xrc = sqlite3_deserialize(
                                (*pDb).db,
                                zSchema,
                                pData,
                                len_3 as sqlite3_int64,
                                len_3 as sqlite3_int64,
                                flags as libc::c_uint,
                            );
                            if xrc != 0 {
                                ((*tclStubsPtr).tcl_AppendResult)
                                    .expect(
                                        "non-null function pointer",
                                    )(
                                    interp,
                                    b"unable to set MEMDB content\0" as *const u8
                                        as *const libc::c_char,
                                    0 as *mut libc::c_char,
                                );
                                rc = 1 as libc::c_int;
                            }
                            if mxSize > 0 as libc::c_int as libc::c_longlong {
                                sqlite3_file_control(
                                    (*pDb).db,
                                    zSchema,
                                    36 as libc::c_int,
                                    &mut mxSize as *mut sqlite3_int64 as *mut libc::c_void,
                                );
                            }
                        }
                    }
                }
            }
        }
        14 => {
            let mut onoff_0: libc::c_int = 0;
            if objc != 3 as libc::c_int {
                ((*tclStubsPtr).tcl_WrongNumArgs)
                    .expect(
                        "non-null function pointer",
                    )(
                    interp,
                    2 as libc::c_int,
                    objv,
                    b"BOOLEAN\0" as *const u8 as *const libc::c_char,
                );
                return 1 as libc::c_int;
            }
            if ((*tclStubsPtr).tcl_GetBooleanFromObj)
                .expect(
                    "non-null function pointer",
                )(interp, *objv.offset(2 as libc::c_int as isize), &mut onoff_0) != 0
            {
                return 1 as libc::c_int;
            }
            sqlite3_enable_load_extension((*pDb).db, onoff_0);
        }
        15 => {
            ((*tclStubsPtr).tcl_SetObjResult)
                .expect(
                    "non-null function pointer",
                )(
                interp,
                ((*tclStubsPtr).tcl_NewIntObj)
                    .expect("non-null function pointer")(sqlite3_errcode((*pDb).db)),
            );
        }
        16 => {
            ((*tclStubsPtr).tcl_SetObjResult)
                .expect(
                    "non-null function pointer",
                )(
                interp,
                ((*tclStubsPtr).tcl_NewIntObj)
                    .expect("non-null function pointer")(sqlite3_error_offset((*pDb).db)),
            );
        }
        18 | 24 => {
            let mut pResult_3: *mut Tcl_Obj = 0 as *mut Tcl_Obj;
            let mut sEval: DbEvalContext = DbEvalContext {
                pDb: 0 as *mut SqliteDb,
                pSql: 0 as *mut Tcl_Obj,
                zSql: 0 as *const libc::c_char,
                pPreStmt: 0 as *mut SqlPreparedStmt,
                nCol: 0,
                evalFlags: 0,
                pArray: 0 as *mut Tcl_Obj,
                apColName: 0 as *mut *mut Tcl_Obj,
            };
            if objc != 3 as libc::c_int {
                ((*tclStubsPtr).tcl_WrongNumArgs)
                    .expect(
                        "non-null function pointer",
                    )(
                    interp,
                    2 as libc::c_int,
                    objv,
                    b"SQL\0" as *const u8 as *const libc::c_char,
                );
                return 1 as libc::c_int;
            }
            dbEvalInit(
                &mut sEval,
                pDb,
                *objv.offset(2 as libc::c_int as isize),
                0 as *mut Tcl_Obj,
                0 as libc::c_int,
            );
            rc = dbEvalStep(&mut sEval);
            if choice == DB_ONECOLUMN as libc::c_int {
                if rc == 0 as libc::c_int {
                    pResult_3 = dbEvalColumnValue(&mut sEval, 0 as libc::c_int);
                } else if rc == 3 as libc::c_int {
                    ((*tclStubsPtr).tcl_ResetResult)
                        .expect("non-null function pointer")(interp);
                }
            } else if rc == 3 as libc::c_int || rc == 0 as libc::c_int {
                pResult_3 = ((*tclStubsPtr).tcl_NewIntObj)
                    .expect(
                        "non-null function pointer",
                    )(
                    ((rc == 0 as libc::c_int) as libc::c_int != 0 as libc::c_int)
                        as libc::c_int,
                );
            }
            dbEvalFinalize(&mut sEval);
            if !pResult_3.is_null() {
                ((*tclStubsPtr).tcl_SetObjResult)
                    .expect("non-null function pointer")(interp, pResult_3);
            }
            if rc == 3 as libc::c_int {
                rc = 0 as libc::c_int;
            }
        }
        17 => {
            let mut evalFlags: libc::c_int = 0 as libc::c_int;
            let mut zOpt_0: *const libc::c_char = 0 as *const libc::c_char;
            while objc > 3 as libc::c_int
                && {
                    zOpt_0 = ((*tclStubsPtr).tcl_GetString)
                        .expect(
                            "non-null function pointer",
                        )(*objv.offset(2 as libc::c_int as isize));
                    !zOpt_0.is_null()
                }
                && *zOpt_0.offset(0 as libc::c_int as isize) as libc::c_int == '-' as i32
            {
                if strcmp(zOpt_0, b"-withoutnulls\0" as *const u8 as *const libc::c_char)
                    == 0 as libc::c_int
                {
                    evalFlags |= 0x1 as libc::c_int;
                } else {
                    ((*tclStubsPtr).tcl_AppendResult)
                        .expect(
                            "non-null function pointer",
                        )(
                        interp,
                        b"unknown option: \"\0" as *const u8 as *const libc::c_char,
                        zOpt_0,
                        b"\"\0" as *const u8 as *const libc::c_char,
                        0 as *mut libc::c_void,
                    );
                    return 1 as libc::c_int;
                }
                objc -= 1;
                objv = objv.offset(1);
            }
            if objc < 3 as libc::c_int || objc > 5 as libc::c_int {
                ((*tclStubsPtr).tcl_WrongNumArgs)
                    .expect(
                        "non-null function pointer",
                    )(
                    interp,
                    2 as libc::c_int,
                    objv,
                    b"?OPTIONS? SQL ?ARRAY-NAME? ?SCRIPT?\0" as *const u8
                        as *const libc::c_char,
                );
                return 1 as libc::c_int;
            }
            if objc == 3 as libc::c_int {
                let mut sEval_0: DbEvalContext = DbEvalContext {
                    pDb: 0 as *mut SqliteDb,
                    pSql: 0 as *mut Tcl_Obj,
                    zSql: 0 as *const libc::c_char,
                    pPreStmt: 0 as *mut SqlPreparedStmt,
                    nCol: 0,
                    evalFlags: 0,
                    pArray: 0 as *mut Tcl_Obj,
                    apColName: 0 as *mut *mut Tcl_Obj,
                };
                let mut pRet: *mut Tcl_Obj = ((*tclStubsPtr).tcl_NewObj)
                    .expect("non-null function pointer")();
                let ref mut fresh151 = (*pRet).refCount;
                *fresh151 += 1;
                dbEvalInit(
                    &mut sEval_0,
                    pDb,
                    *objv.offset(2 as libc::c_int as isize),
                    0 as *mut Tcl_Obj,
                    0 as libc::c_int,
                );
                loop {
                    rc = dbEvalStep(&mut sEval_0);
                    if !(0 as libc::c_int == rc) {
                        break;
                    }
                    let mut i_1: libc::c_int = 0;
                    let mut nCol_0: libc::c_int = 0;
                    dbEvalRowInfo(
                        &mut sEval_0,
                        &mut nCol_0,
                        0 as *mut *mut *mut Tcl_Obj,
                    );
                    i_1 = 0 as libc::c_int;
                    while i_1 < nCol_0 {
                        ((*tclStubsPtr).tcl_ListObjAppendElement)
                            .expect(
                                "non-null function pointer",
                            )(interp, pRet, dbEvalColumnValue(&mut sEval_0, i_1));
                        i_1 += 1;
                    }
                }
                dbEvalFinalize(&mut sEval_0);
                if rc == 3 as libc::c_int {
                    ((*tclStubsPtr).tcl_SetObjResult)
                        .expect("non-null function pointer")(interp, pRet);
                    rc = 0 as libc::c_int;
                }
                let mut _objPtr_0: *mut Tcl_Obj = pRet;
                let ref mut fresh152 = (*_objPtr_0).refCount;
                let fresh153 = *fresh152;
                *fresh152 = *fresh152 - 1;
                if fresh153 <= 1 as libc::c_int {
                    ((*tclStubsPtr).tclFreeObj)
                        .expect("non-null function pointer")(_objPtr_0);
                }
            } else {
                let mut cd2: [ClientData; 2] = [0 as *mut libc::c_void; 2];
                let mut p: *mut DbEvalContext = 0 as *mut DbEvalContext;
                let mut pArray: *mut Tcl_Obj = 0 as *mut Tcl_Obj;
                let mut pScript: *mut Tcl_Obj = 0 as *mut Tcl_Obj;
                if objc >= 5 as libc::c_int
                    && *((*tclStubsPtr).tcl_GetString)
                        .expect(
                            "non-null function pointer",
                        )(*objv.offset(3 as libc::c_int as isize)) as libc::c_int != 0
                {
                    pArray = *objv.offset(3 as libc::c_int as isize);
                }
                pScript = *objv.offset((objc - 1 as libc::c_int) as isize);
                let ref mut fresh154 = (*pScript).refCount;
                *fresh154 += 1;
                p = ((*tclStubsPtr).tcl_Alloc)
                    .expect(
                        "non-null function pointer",
                    )(
                    ::std::mem::size_of::<DbEvalContext>() as libc::c_ulong
                        as libc::c_uint,
                ) as *mut DbEvalContext;
                dbEvalInit(
                    p,
                    pDb,
                    *objv.offset(2 as libc::c_int as isize),
                    pArray,
                    evalFlags,
                );
                cd2[0 as libc::c_int as usize] = p as *mut libc::c_void;
                cd2[1 as libc::c_int as usize] = pScript as *mut libc::c_void;
                rc = DbEvalNextCmd(cd2.as_mut_ptr(), interp, 0 as libc::c_int);
            }
        }
        19 => {
            let mut flags_0: libc::c_int = 1 as libc::c_int;
            let mut pFunc: *mut SqlFunc = 0 as *mut SqlFunc;
            let mut pScript_0: *mut Tcl_Obj = 0 as *mut Tcl_Obj;
            let mut zName_0: *mut libc::c_char = 0 as *mut libc::c_char;
            let mut nArg: libc::c_int = -(1 as libc::c_int);
            let mut i_2: libc::c_int = 0;
            let mut eType: libc::c_int = 5 as libc::c_int;
            if objc < 4 as libc::c_int {
                ((*tclStubsPtr).tcl_WrongNumArgs)
                    .expect(
                        "non-null function pointer",
                    )(
                    interp,
                    2 as libc::c_int,
                    objv,
                    b"NAME ?SWITCHES? SCRIPT\0" as *const u8 as *const libc::c_char,
                );
                return 1 as libc::c_int;
            }
            i_2 = 3 as libc::c_int;
            while i_2 < objc - 1 as libc::c_int {
                let mut z_1: *const libc::c_char = ((*tclStubsPtr).tcl_GetString)
                    .expect("non-null function pointer")(*objv.offset(i_2 as isize));
                let mut n_0: libc::c_int = strlen30(z_1);
                if n_0 > 1 as libc::c_int
                    && strncmp(
                        z_1,
                        b"-argcount\0" as *const u8 as *const libc::c_char,
                        n_0 as libc::c_ulong,
                    ) == 0 as libc::c_int
                {
                    if i_2 == objc - 2 as libc::c_int {
                        ((*tclStubsPtr).tcl_AppendResult)
                            .expect(
                                "non-null function pointer",
                            )(
                            interp,
                            b"option requires an argument: \0" as *const u8
                                as *const libc::c_char,
                            z_1,
                            0 as *mut libc::c_char,
                        );
                        return 1 as libc::c_int;
                    }
                    if ((*tclStubsPtr).tcl_GetIntFromObj)
                        .expect(
                            "non-null function pointer",
                        )(
                        interp,
                        *objv.offset((i_2 + 1 as libc::c_int) as isize),
                        &mut nArg,
                    ) != 0
                    {
                        return 1 as libc::c_int;
                    }
                    if nArg < 0 as libc::c_int {
                        ((*tclStubsPtr).tcl_AppendResult)
                            .expect(
                                "non-null function pointer",
                            )(
                            interp,
                            b"number of arguments must be non-negative\0" as *const u8
                                as *const libc::c_char,
                            0 as *mut libc::c_char,
                        );
                        return 1 as libc::c_int;
                    }
                    i_2 += 1;
                } else if n_0 > 1 as libc::c_int
                    && strncmp(
                        z_1,
                        b"-deterministic\0" as *const u8 as *const libc::c_char,
                        n_0 as libc::c_ulong,
                    ) == 0 as libc::c_int
                {
                    flags_0 |= 0x800 as libc::c_int;
                } else if n_0 > 1 as libc::c_int
                    && strncmp(
                        z_1,
                        b"-directonly\0" as *const u8 as *const libc::c_char,
                        n_0 as libc::c_ulong,
                    ) == 0 as libc::c_int
                {
                    flags_0 |= 0x80000 as libc::c_int;
                } else if n_0 > 1 as libc::c_int
                    && strncmp(
                        z_1,
                        b"-innocuous\0" as *const u8 as *const libc::c_char,
                        n_0 as libc::c_ulong,
                    ) == 0 as libc::c_int
                {
                    flags_0 |= 0x200000 as libc::c_int;
                } else if n_0 > 1 as libc::c_int
                    && strncmp(
                        z_1,
                        b"-returntype\0" as *const u8 as *const libc::c_char,
                        n_0 as libc::c_ulong,
                    ) == 0 as libc::c_int
                {
                    let mut azType: [*const libc::c_char; 6] = [
                        b"integer\0" as *const u8 as *const libc::c_char,
                        b"real\0" as *const u8 as *const libc::c_char,
                        b"text\0" as *const u8 as *const libc::c_char,
                        b"blob\0" as *const u8 as *const libc::c_char,
                        b"any\0" as *const u8 as *const libc::c_char,
                        0 as *const libc::c_char,
                    ];
                    if i_2 == objc - 2 as libc::c_int {
                        ((*tclStubsPtr).tcl_AppendResult)
                            .expect(
                                "non-null function pointer",
                            )(
                            interp,
                            b"option requires an argument: \0" as *const u8
                                as *const libc::c_char,
                            z_1,
                            0 as *mut libc::c_char,
                        );
                        return 1 as libc::c_int;
                    }
                    i_2 += 1;
                    if ((*tclStubsPtr).tcl_GetIndexFromObjStruct)
                        .expect(
                            "non-null function pointer",
                        )(
                        interp,
                        *objv.offset(i_2 as isize),
                        azType.as_mut_ptr() as *const libc::c_void,
                        ::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong
                            as libc::c_int,
                        b"type\0" as *const u8 as *const libc::c_char,
                        0 as libc::c_int,
                        &mut eType,
                    ) != 0
                    {
                        return 1 as libc::c_int;
                    }
                    eType += 1;
                } else {
                    ((*tclStubsPtr).tcl_AppendResult)
                        .expect(
                            "non-null function pointer",
                        )(
                        interp,
                        b"bad option \"\0" as *const u8 as *const libc::c_char,
                        z_1,
                        b"\": must be -argcount, -deterministic, -directonly, -innocuous, or -returntype\0"
                            as *const u8 as *const libc::c_char,
                        0 as *mut libc::c_char,
                    );
                    return 1 as libc::c_int;
                }
                i_2 += 1;
            }
            pScript_0 = *objv.offset((objc - 1 as libc::c_int) as isize);
            zName_0 = ((*tclStubsPtr).tcl_GetStringFromObj)
                .expect(
                    "non-null function pointer",
                )(*objv.offset(2 as libc::c_int as isize), 0 as *mut libc::c_int);
            pFunc = findSqlFunc(pDb, zName_0);
            if pFunc.is_null() {
                return 1 as libc::c_int;
            }
            if !((*pFunc).pScript).is_null() {
                let mut _objPtr_1: *mut Tcl_Obj = (*pFunc).pScript;
                let ref mut fresh155 = (*_objPtr_1).refCount;
                let fresh156 = *fresh155;
                *fresh155 = *fresh155 - 1;
                if fresh156 <= 1 as libc::c_int {
                    ((*tclStubsPtr).tclFreeObj)
                        .expect("non-null function pointer")(_objPtr_1);
                }
            }
            let ref mut fresh157 = (*pFunc).pScript;
            *fresh157 = pScript_0;
            let ref mut fresh158 = (*pScript_0).refCount;
            *fresh158 += 1;
            (*pFunc).useEvalObjv = safeToUseEvalObjv(interp, pScript_0);
            (*pFunc).eType = eType;
            rc = sqlite3_create_function(
                (*pDb).db,
                zName_0,
                nArg,
                flags_0,
                pFunc as *mut libc::c_void,
                Some(
                    tclSqlFunc
                        as unsafe extern "C" fn(
                            *mut sqlite3_context,
                            libc::c_int,
                            *mut *mut sqlite3_value,
                        ) -> (),
                ),
                None,
                None,
            );
            if rc != 0 as libc::c_int {
                rc = 1 as libc::c_int;
                ((*tclStubsPtr).tcl_SetResult)
                    .expect(
                        "non-null function pointer",
                    )(
                    interp,
                    sqlite3_errmsg((*pDb).db) as *mut libc::c_char,
                    ::std::mem::transmute::<
                        libc::intptr_t,
                        Option::<Tcl_FreeProc>,
                    >(1 as libc::c_int as libc::intptr_t),
                );
            }
        }
        20 => {
            let mut isReadonly_0: libc::c_int = 0 as libc::c_int;
            let mut zDb: *const libc::c_char = b"main\0" as *const u8
                as *const libc::c_char;
            let mut zTable_0: *const libc::c_char = 0 as *const libc::c_char;
            let mut zColumn: *const libc::c_char = 0 as *const libc::c_char;
            let mut iRow: Tcl_WideInt = 0;
            if objc > 3 as libc::c_int
                && strcmp(
                    ((*tclStubsPtr).tcl_GetString)
                        .expect(
                            "non-null function pointer",
                        )(*objv.offset(2 as libc::c_int as isize)),
                    b"-readonly\0" as *const u8 as *const libc::c_char,
                ) == 0 as libc::c_int
            {
                isReadonly_0 = 1 as libc::c_int;
            }
            if objc != 5 as libc::c_int + isReadonly_0
                && objc != 6 as libc::c_int + isReadonly_0
            {
                ((*tclStubsPtr).tcl_WrongNumArgs)
                    .expect(
                        "non-null function pointer",
                    )(
                    interp,
                    2 as libc::c_int,
                    objv,
                    b"?-readonly? ?DB? TABLE COLUMN ROWID\0" as *const u8
                        as *const libc::c_char,
                );
                return 1 as libc::c_int;
            }
            if objc == 6 as libc::c_int + isReadonly_0 {
                zDb = ((*tclStubsPtr).tcl_GetString)
                    .expect(
                        "non-null function pointer",
                    )(*objv.offset((2 as libc::c_int + isReadonly_0) as isize));
            }
            zTable_0 = ((*tclStubsPtr).tcl_GetString)
                .expect(
                    "non-null function pointer",
                )(*objv.offset((objc - 3 as libc::c_int) as isize));
            zColumn = ((*tclStubsPtr).tcl_GetString)
                .expect(
                    "non-null function pointer",
                )(*objv.offset((objc - 2 as libc::c_int) as isize));
            rc = ((*tclStubsPtr).tcl_GetWideIntFromObj)
                .expect(
                    "non-null function pointer",
                )(interp, *objv.offset((objc - 1 as libc::c_int) as isize), &mut iRow);
            if rc == 0 as libc::c_int {
                rc = createIncrblobChannel(
                    interp,
                    pDb,
                    zDb,
                    zTable_0,
                    zColumn,
                    iRow,
                    isReadonly_0,
                );
            }
        }
        21 => {
            sqlite3_interrupt((*pDb).db);
        }
        23 => {
            if objc != 2 as libc::c_int && objc != 3 as libc::c_int {
                ((*tclStubsPtr).tcl_WrongNumArgs)
                    .expect(
                        "non-null function pointer",
                    )(
                    interp,
                    2 as libc::c_int,
                    objv,
                    b"NULLVALUE\0" as *const u8 as *const libc::c_char,
                );
                return 1 as libc::c_int;
            }
            if objc == 3 as libc::c_int {
                let mut len_4: libc::c_int = 0;
                let mut zNull_0: *mut libc::c_char = ((*tclStubsPtr)
                    .tcl_GetStringFromObj)
                    .expect(
                        "non-null function pointer",
                    )(*objv.offset(2 as libc::c_int as isize), &mut len_4);
                if !((*pDb).zNull).is_null() {
                    ((*tclStubsPtr).tcl_Free)
                        .expect("non-null function pointer")((*pDb).zNull);
                }
                if !zNull_0.is_null() && len_4 > 0 as libc::c_int {
                    let ref mut fresh159 = (*pDb).zNull;
                    *fresh159 = ((*tclStubsPtr).tcl_Alloc)
                        .expect(
                            "non-null function pointer",
                        )((len_4 + 1 as libc::c_int) as libc::c_uint);
                    memcpy(
                        (*pDb).zNull as *mut libc::c_void,
                        zNull_0 as *const libc::c_void,
                        len_4 as libc::c_ulong,
                    );
                    *((*pDb).zNull).offset(len_4 as isize) = '\0' as i32 as libc::c_char;
                } else {
                    let ref mut fresh160 = (*pDb).zNull;
                    *fresh160 = 0 as *mut libc::c_char;
                }
            }
            ((*tclStubsPtr).tcl_SetObjResult)
                .expect(
                    "non-null function pointer",
                )(
                interp,
                ((*tclStubsPtr).tcl_NewStringObj)
                    .expect(
                        "non-null function pointer",
                    )((*pDb).zNull, -(1 as libc::c_int)),
            );
        }
        22 => {
            let mut pResult_4: *mut Tcl_Obj = 0 as *mut Tcl_Obj;
            let mut rowid: Tcl_WideInt = 0;
            if objc != 2 as libc::c_int {
                ((*tclStubsPtr).tcl_WrongNumArgs)
                    .expect(
                        "non-null function pointer",
                    )(
                    interp,
                    2 as libc::c_int,
                    objv,
                    b"\0" as *const u8 as *const libc::c_char,
                );
                return 1 as libc::c_int;
            }
            rowid = sqlite3_last_insert_rowid((*pDb).db);
            pResult_4 = ((*tclStubsPtr).tcl_GetObjResult)
                .expect("non-null function pointer")(interp);
            ((*tclStubsPtr).tcl_SetWideIntObj)
                .expect("non-null function pointer")(pResult_4, rowid);
        }
        27 => {
            if objc == 2 as libc::c_int {
                if !((*pDb).zProgress).is_null() {
                    ((*tclStubsPtr).tcl_AppendResult)
                        .expect(
                            "non-null function pointer",
                        )(interp, (*pDb).zProgress, 0 as *mut libc::c_char);
                }
            } else if objc == 4 as libc::c_int {
                let mut zProgress: *mut libc::c_char = 0 as *mut libc::c_char;
                let mut len_5: libc::c_int = 0;
                let mut N: libc::c_int = 0;
                if 0 as libc::c_int
                    != ((*tclStubsPtr).tcl_GetIntFromObj)
                        .expect(
                            "non-null function pointer",
                        )(interp, *objv.offset(2 as libc::c_int as isize), &mut N)
                {
                    return 1 as libc::c_int;
                }
                if !((*pDb).zProgress).is_null() {
                    ((*tclStubsPtr).tcl_Free)
                        .expect("non-null function pointer")((*pDb).zProgress);
                }
                zProgress = ((*tclStubsPtr).tcl_GetStringFromObj)
                    .expect(
                        "non-null function pointer",
                    )(*objv.offset(3 as libc::c_int as isize), &mut len_5);
                if !zProgress.is_null() && len_5 > 0 as libc::c_int {
                    let ref mut fresh161 = (*pDb).zProgress;
                    *fresh161 = ((*tclStubsPtr).tcl_Alloc)
                        .expect(
                            "non-null function pointer",
                        )((len_5 + 1 as libc::c_int) as libc::c_uint);
                    memcpy(
                        (*pDb).zProgress as *mut libc::c_void,
                        zProgress as *const libc::c_void,
                        (len_5 + 1 as libc::c_int) as libc::c_ulong,
                    );
                } else {
                    let ref mut fresh162 = (*pDb).zProgress;
                    *fresh162 = 0 as *mut libc::c_char;
                }
                if !((*pDb).zProgress).is_null() {
                    let ref mut fresh163 = (*pDb).interp;
                    *fresh163 = interp;
                    sqlite3_progress_handler(
                        (*pDb).db,
                        N,
                        Some(
                            DbProgressHandler
                                as unsafe extern "C" fn(*mut libc::c_void) -> libc::c_int,
                        ),
                        pDb as *mut libc::c_void,
                    );
                } else {
                    sqlite3_progress_handler(
                        (*pDb).db,
                        0 as libc::c_int,
                        None,
                        0 as *mut libc::c_void,
                    );
                }
            } else {
                ((*tclStubsPtr).tcl_WrongNumArgs)
                    .expect(
                        "non-null function pointer",
                    )(
                    interp,
                    2 as libc::c_int,
                    objv,
                    b"N CALLBACK\0" as *const u8 as *const libc::c_char,
                );
                return 1 as libc::c_int;
            }
        }
        26 => {
            if objc > 3 as libc::c_int {
                ((*tclStubsPtr).tcl_WrongNumArgs)
                    .expect(
                        "non-null function pointer",
                    )(
                    interp,
                    2 as libc::c_int,
                    objv,
                    b"?CALLBACK?\0" as *const u8 as *const libc::c_char,
                );
                return 1 as libc::c_int;
            } else {
                if objc == 2 as libc::c_int {
                    if !((*pDb).zProfile).is_null() {
                        ((*tclStubsPtr).tcl_AppendResult)
                            .expect(
                                "non-null function pointer",
                            )(interp, (*pDb).zProfile, 0 as *mut libc::c_char);
                    }
                } else {
                    let mut zProfile: *mut libc::c_char = 0 as *mut libc::c_char;
                    let mut len_6: libc::c_int = 0;
                    if !((*pDb).zProfile).is_null() {
                        ((*tclStubsPtr).tcl_Free)
                            .expect("non-null function pointer")((*pDb).zProfile);
                    }
                    zProfile = ((*tclStubsPtr).tcl_GetStringFromObj)
                        .expect(
                            "non-null function pointer",
                        )(*objv.offset(2 as libc::c_int as isize), &mut len_6);
                    if !zProfile.is_null() && len_6 > 0 as libc::c_int {
                        let ref mut fresh164 = (*pDb).zProfile;
                        *fresh164 = ((*tclStubsPtr).tcl_Alloc)
                            .expect(
                                "non-null function pointer",
                            )((len_6 + 1 as libc::c_int) as libc::c_uint);
                        memcpy(
                            (*pDb).zProfile as *mut libc::c_void,
                            zProfile as *const libc::c_void,
                            (len_6 + 1 as libc::c_int) as libc::c_ulong,
                        );
                    } else {
                        let ref mut fresh165 = (*pDb).zProfile;
                        *fresh165 = 0 as *mut libc::c_char;
                    }
                    if !((*pDb).zProfile).is_null() {
                        let ref mut fresh166 = (*pDb).interp;
                        *fresh166 = interp;
                        sqlite3_profile(
                            (*pDb).db,
                            Some(
                                DbProfileHandler
                                    as unsafe extern "C" fn(
                                        *mut libc::c_void,
                                        *const libc::c_char,
                                        sqlite_uint64,
                                    ) -> (),
                            ),
                            pDb as *mut libc::c_void,
                        );
                    } else {
                        sqlite3_profile((*pDb).db, None, 0 as *mut libc::c_void);
                    }
                }
            }
        }
        28 => {
            if objc != 3 as libc::c_int {
                ((*tclStubsPtr).tcl_WrongNumArgs)
                    .expect(
                        "non-null function pointer",
                    )(
                    interp,
                    2 as libc::c_int,
                    objv,
                    b"KEY\0" as *const u8 as *const libc::c_char,
                );
                return 1 as libc::c_int;
            }
        }
        29 => {
            let mut zSrcFile: *const libc::c_char = 0 as *const libc::c_char;
            let mut zDestDb: *const libc::c_char = 0 as *const libc::c_char;
            let mut pSrc: *mut sqlite3 = 0 as *mut sqlite3;
            let mut pBackup_0: *mut sqlite3_backup = 0 as *mut sqlite3_backup;
            let mut nTimeout: libc::c_int = 0 as libc::c_int;
            if objc == 3 as libc::c_int {
                zDestDb = b"main\0" as *const u8 as *const libc::c_char;
                zSrcFile = ((*tclStubsPtr).tcl_GetString)
                    .expect(
                        "non-null function pointer",
                    )(*objv.offset(2 as libc::c_int as isize));
            } else if objc == 4 as libc::c_int {
                zDestDb = ((*tclStubsPtr).tcl_GetString)
                    .expect(
                        "non-null function pointer",
                    )(*objv.offset(2 as libc::c_int as isize));
                zSrcFile = ((*tclStubsPtr).tcl_GetString)
                    .expect(
                        "non-null function pointer",
                    )(*objv.offset(3 as libc::c_int as isize));
            } else {
                ((*tclStubsPtr).tcl_WrongNumArgs)
                    .expect(
                        "non-null function pointer",
                    )(
                    interp,
                    2 as libc::c_int,
                    objv,
                    b"?DATABASE? FILENAME\0" as *const u8 as *const libc::c_char,
                );
                return 1 as libc::c_int;
            }
            rc = sqlite3_open_v2(
                zSrcFile,
                &mut pSrc,
                0x1 as libc::c_int | (*pDb).openFlags,
                0 as *const libc::c_char,
            );
            if rc != 0 as libc::c_int {
                ((*tclStubsPtr).tcl_AppendResult)
                    .expect(
                        "non-null function pointer",
                    )(
                    interp,
                    b"cannot open source database: \0" as *const u8
                        as *const libc::c_char,
                    sqlite3_errmsg(pSrc),
                    0 as *mut libc::c_char,
                );
                sqlite3_close(pSrc);
                return 1 as libc::c_int;
            }
            pBackup_0 = sqlite3_backup_init(
                (*pDb).db,
                zDestDb,
                pSrc,
                b"main\0" as *const u8 as *const libc::c_char,
            );
            if pBackup_0.is_null() {
                ((*tclStubsPtr).tcl_AppendResult)
                    .expect(
                        "non-null function pointer",
                    )(
                    interp,
                    b"restore failed: \0" as *const u8 as *const libc::c_char,
                    sqlite3_errmsg((*pDb).db),
                    0 as *mut libc::c_char,
                );
                sqlite3_close(pSrc);
                return 1 as libc::c_int;
            }
            loop {
                rc = sqlite3_backup_step(pBackup_0, 100 as libc::c_int);
                if !(rc == 0 as libc::c_int || rc == 5 as libc::c_int) {
                    break;
                }
                if !(rc == 5 as libc::c_int) {
                    continue;
                }
                let fresh167 = nTimeout;
                nTimeout = nTimeout + 1;
                if fresh167 >= 3 as libc::c_int {
                    break;
                }
                sqlite3_sleep(100 as libc::c_int);
            }
            sqlite3_backup_finish(pBackup_0);
            if rc == 101 as libc::c_int {
                rc = 0 as libc::c_int;
            } else if rc == 5 as libc::c_int || rc == 6 as libc::c_int {
                ((*tclStubsPtr).tcl_AppendResult)
                    .expect(
                        "non-null function pointer",
                    )(
                    interp,
                    b"restore failed: source database busy\0" as *const u8
                        as *const libc::c_char,
                    0 as *mut libc::c_char,
                );
                rc = 1 as libc::c_int;
            } else {
                ((*tclStubsPtr).tcl_AppendResult)
                    .expect(
                        "non-null function pointer",
                    )(
                    interp,
                    b"restore failed: \0" as *const u8 as *const libc::c_char,
                    sqlite3_errmsg((*pDb).db),
                    0 as *mut libc::c_char,
                );
                rc = 1 as libc::c_int;
            }
            sqlite3_close(pSrc);
        }
        31 => {
            let mut zSchema_0: *const libc::c_char = if objc >= 3 as libc::c_int {
                ((*tclStubsPtr).tcl_GetString)
                    .expect(
                        "non-null function pointer",
                    )(*objv.offset(2 as libc::c_int as isize)) as *const libc::c_char
            } else {
                b"main\0" as *const u8 as *const libc::c_char
            };
            let mut sz: sqlite3_int64 = 0 as libc::c_int as sqlite3_int64;
            let mut pData_0: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
            if objc != 2 as libc::c_int && objc != 3 as libc::c_int {
                ((*tclStubsPtr).tcl_WrongNumArgs)
                    .expect(
                        "non-null function pointer",
                    )(
                    interp,
                    2 as libc::c_int,
                    objv,
                    b"?DATABASE?\0" as *const u8 as *const libc::c_char,
                );
                rc = 1 as libc::c_int;
            } else {
                let mut needFree: libc::c_int = 0;
                pData_0 = sqlite3_serialize(
                    (*pDb).db,
                    zSchema_0,
                    &mut sz,
                    0x1 as libc::c_int as libc::c_uint,
                );
                if !pData_0.is_null() {
                    needFree = 0 as libc::c_int;
                } else {
                    pData_0 = sqlite3_serialize(
                        (*pDb).db,
                        zSchema_0,
                        &mut sz,
                        0 as libc::c_int as libc::c_uint,
                    );
                    needFree = 1 as libc::c_int;
                }
                ((*tclStubsPtr).tcl_SetObjResult)
                    .expect(
                        "non-null function pointer",
                    )(
                    interp,
                    ((*tclStubsPtr).tcl_NewByteArrayObj)
                        .expect("non-null function pointer")(pData_0, sz as libc::c_int),
                );
                if needFree != 0 {
                    sqlite3_free(pData_0 as *mut libc::c_void);
                }
            }
        }
        32 => {
            let mut v_1: libc::c_int = 0;
            let mut zOp: *const libc::c_char = 0 as *const libc::c_char;
            if objc != 3 as libc::c_int {
                ((*tclStubsPtr).tcl_WrongNumArgs)
                    .expect(
                        "non-null function pointer",
                    )(
                    interp,
                    2 as libc::c_int,
                    objv,
                    b"(step|sort|autoindex)\0" as *const u8 as *const libc::c_char,
                );
                return 1 as libc::c_int;
            }
            zOp = ((*tclStubsPtr).tcl_GetString)
                .expect(
                    "non-null function pointer",
                )(*objv.offset(2 as libc::c_int as isize));
            if strcmp(zOp, b"step\0" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
            {
                v_1 = (*pDb).nStep;
            } else if strcmp(zOp, b"sort\0" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
            {
                v_1 = (*pDb).nSort;
            } else if strcmp(zOp, b"autoindex\0" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
            {
                v_1 = (*pDb).nIndex;
            } else if strcmp(zOp, b"vmstep\0" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
            {
                v_1 = (*pDb).nVMStep;
            } else {
                ((*tclStubsPtr).tcl_AppendResult)
                    .expect(
                        "non-null function pointer",
                    )(
                    interp,
                    b"bad argument: should be autoindex, step, sort or vmstep\0"
                        as *const u8 as *const libc::c_char,
                    0 as *mut libc::c_char,
                );
                return 1 as libc::c_int;
            }
            ((*tclStubsPtr).tcl_SetObjResult)
                .expect(
                    "non-null function pointer",
                )(
                interp,
                ((*tclStubsPtr).tcl_NewIntObj).expect("non-null function pointer")(v_1),
            );
        }
        33 => {
            let mut ms: libc::c_int = 0;
            if objc != 3 as libc::c_int {
                ((*tclStubsPtr).tcl_WrongNumArgs)
                    .expect(
                        "non-null function pointer",
                    )(
                    interp,
                    2 as libc::c_int,
                    objv,
                    b"MILLISECONDS\0" as *const u8 as *const libc::c_char,
                );
                return 1 as libc::c_int;
            }
            if ((*tclStubsPtr).tcl_GetIntFromObj)
                .expect(
                    "non-null function pointer",
                )(interp, *objv.offset(2 as libc::c_int as isize), &mut ms) != 0
            {
                return 1 as libc::c_int;
            }
            sqlite3_busy_timeout((*pDb).db, ms);
        }
        34 => {
            let mut pResult_5: *mut Tcl_Obj = 0 as *mut Tcl_Obj;
            if objc != 2 as libc::c_int {
                ((*tclStubsPtr).tcl_WrongNumArgs)
                    .expect(
                        "non-null function pointer",
                    )(
                    interp,
                    2 as libc::c_int,
                    objv,
                    b"\0" as *const u8 as *const libc::c_char,
                );
                return 1 as libc::c_int;
            }
            pResult_5 = ((*tclStubsPtr).tcl_GetObjResult)
                .expect("non-null function pointer")(interp);
            ((*tclStubsPtr).tcl_SetWideIntObj)
                .expect(
                    "non-null function pointer",
                )(pResult_5, sqlite3_total_changes64((*pDb).db));
        }
        35 => {
            if objc > 3 as libc::c_int {
                ((*tclStubsPtr).tcl_WrongNumArgs)
                    .expect(
                        "non-null function pointer",
                    )(
                    interp,
                    2 as libc::c_int,
                    objv,
                    b"?CALLBACK?\0" as *const u8 as *const libc::c_char,
                );
                return 1 as libc::c_int;
            } else {
                if objc == 2 as libc::c_int {
                    if !((*pDb).zTrace).is_null() {
                        ((*tclStubsPtr).tcl_AppendResult)
                            .expect(
                                "non-null function pointer",
                            )(interp, (*pDb).zTrace, 0 as *mut libc::c_char);
                    }
                } else {
                    let mut zTrace: *mut libc::c_char = 0 as *mut libc::c_char;
                    let mut len_7: libc::c_int = 0;
                    if !((*pDb).zTrace).is_null() {
                        ((*tclStubsPtr).tcl_Free)
                            .expect("non-null function pointer")((*pDb).zTrace);
                    }
                    zTrace = ((*tclStubsPtr).tcl_GetStringFromObj)
                        .expect(
                            "non-null function pointer",
                        )(*objv.offset(2 as libc::c_int as isize), &mut len_7);
                    if !zTrace.is_null() && len_7 > 0 as libc::c_int {
                        let ref mut fresh168 = (*pDb).zTrace;
                        *fresh168 = ((*tclStubsPtr).tcl_Alloc)
                            .expect(
                                "non-null function pointer",
                            )((len_7 + 1 as libc::c_int) as libc::c_uint);
                        memcpy(
                            (*pDb).zTrace as *mut libc::c_void,
                            zTrace as *const libc::c_void,
                            (len_7 + 1 as libc::c_int) as libc::c_ulong,
                        );
                    } else {
                        let ref mut fresh169 = (*pDb).zTrace;
                        *fresh169 = 0 as *mut libc::c_char;
                    }
                    if !((*pDb).zTrace).is_null() {
                        let ref mut fresh170 = (*pDb).interp;
                        *fresh170 = interp;
                        sqlite3_trace(
                            (*pDb).db,
                            Some(
                                DbTraceHandler
                                    as unsafe extern "C" fn(
                                        *mut libc::c_void,
                                        *const libc::c_char,
                                    ) -> (),
                            ),
                            pDb as *mut libc::c_void,
                        );
                    } else {
                        sqlite3_trace((*pDb).db, None, 0 as *mut libc::c_void);
                    }
                }
            }
        }
        36 => {
            if objc > 4 as libc::c_int {
                ((*tclStubsPtr).tcl_WrongNumArgs)
                    .expect(
                        "non-null function pointer",
                    )(
                    interp,
                    2 as libc::c_int,
                    objv,
                    b"?CALLBACK? ?MASK?\0" as *const u8 as *const libc::c_char,
                );
                return 1 as libc::c_int;
            } else {
                if objc == 2 as libc::c_int {
                    if !((*pDb).zTraceV2).is_null() {
                        ((*tclStubsPtr).tcl_AppendResult)
                            .expect(
                                "non-null function pointer",
                            )(interp, (*pDb).zTraceV2, 0 as *mut libc::c_char);
                    }
                } else {
                    let mut zTraceV2: *mut libc::c_char = 0 as *mut libc::c_char;
                    let mut len_8: libc::c_int = 0;
                    let mut wMask: Tcl_WideInt = 0 as libc::c_int as Tcl_WideInt;
                    if objc == 4 as libc::c_int {
                        static mut TTYPE_strs: [*const libc::c_char; 5] = [
                            b"statement\0" as *const u8 as *const libc::c_char,
                            b"profile\0" as *const u8 as *const libc::c_char,
                            b"row\0" as *const u8 as *const libc::c_char,
                            b"close\0" as *const u8 as *const libc::c_char,
                            0 as *const libc::c_char,
                        ];
                        let mut i_3: libc::c_int = 0;
                        if 0 as libc::c_int
                            != ((*tclStubsPtr).tcl_ListObjLength)
                                .expect(
                                    "non-null function pointer",
                                )(
                                interp,
                                *objv.offset(3 as libc::c_int as isize),
                                &mut len_8,
                            )
                        {
                            return 1 as libc::c_int;
                        }
                        i_3 = 0 as libc::c_int;
                        while i_3 < len_8 {
                            let mut pObj: *mut Tcl_Obj = 0 as *mut Tcl_Obj;
                            let mut ttype: libc::c_int = 0;
                            if 0 as libc::c_int
                                != ((*tclStubsPtr).tcl_ListObjIndex)
                                    .expect(
                                        "non-null function pointer",
                                    )(
                                    interp,
                                    *objv.offset(3 as libc::c_int as isize),
                                    i_3,
                                    &mut pObj,
                                )
                            {
                                return 1 as libc::c_int;
                            }
                            if ((*tclStubsPtr).tcl_GetIndexFromObjStruct)
                                .expect(
                                    "non-null function pointer",
                                )(
                                interp,
                                pObj,
                                TTYPE_strs.as_mut_ptr() as *const libc::c_void,
                                ::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong
                                    as libc::c_int,
                                b"trace type\0" as *const u8 as *const libc::c_char,
                                0 as libc::c_int,
                                &mut ttype,
                            ) != 0 as libc::c_int
                            {
                                let mut wType: Tcl_WideInt = 0;
                                let mut pError: *mut Tcl_Obj = ((*tclStubsPtr)
                                    .tcl_DuplicateObj)
                                    .expect(
                                        "non-null function pointer",
                                    )(
                                    ((*tclStubsPtr).tcl_GetObjResult)
                                        .expect("non-null function pointer")(interp),
                                );
                                let ref mut fresh171 = (*pError).refCount;
                                *fresh171 += 1;
                                if 0 as libc::c_int
                                    == ((*tclStubsPtr).tcl_GetWideIntFromObj)
                                        .expect(
                                            "non-null function pointer",
                                        )(interp, pObj, &mut wType)
                                {
                                    let mut _objPtr_2: *mut Tcl_Obj = pError;
                                    let ref mut fresh172 = (*_objPtr_2).refCount;
                                    let fresh173 = *fresh172;
                                    *fresh172 = *fresh172 - 1;
                                    if fresh173 <= 1 as libc::c_int {
                                        ((*tclStubsPtr).tclFreeObj)
                                            .expect("non-null function pointer")(_objPtr_2);
                                    }
                                    wMask |= wType;
                                } else {
                                    ((*tclStubsPtr).tcl_SetObjResult)
                                        .expect("non-null function pointer")(interp, pError);
                                    let mut _objPtr_3: *mut Tcl_Obj = pError;
                                    let ref mut fresh174 = (*_objPtr_3).refCount;
                                    let fresh175 = *fresh174;
                                    *fresh174 = *fresh174 - 1;
                                    if fresh175 <= 1 as libc::c_int {
                                        ((*tclStubsPtr).tclFreeObj)
                                            .expect("non-null function pointer")(_objPtr_3);
                                    }
                                    return 1 as libc::c_int;
                                }
                            } else {
                                match ttype as TTYPE_enum_0 as libc::c_uint {
                                    0 => {
                                        wMask |= 0x1 as libc::c_int as libc::c_longlong;
                                    }
                                    1 => {
                                        wMask |= 0x2 as libc::c_int as libc::c_longlong;
                                    }
                                    2 => {
                                        wMask |= 0x4 as libc::c_int as libc::c_longlong;
                                    }
                                    3 => {
                                        wMask |= 0x8 as libc::c_int as libc::c_longlong;
                                    }
                                    _ => {}
                                }
                            }
                            i_3 += 1;
                        }
                    } else {
                        wMask = 0x1 as libc::c_int as Tcl_WideInt;
                    }
                    if !((*pDb).zTraceV2).is_null() {
                        ((*tclStubsPtr).tcl_Free)
                            .expect("non-null function pointer")((*pDb).zTraceV2);
                    }
                    zTraceV2 = ((*tclStubsPtr).tcl_GetStringFromObj)
                        .expect(
                            "non-null function pointer",
                        )(*objv.offset(2 as libc::c_int as isize), &mut len_8);
                    if !zTraceV2.is_null() && len_8 > 0 as libc::c_int {
                        let ref mut fresh176 = (*pDb).zTraceV2;
                        *fresh176 = ((*tclStubsPtr).tcl_Alloc)
                            .expect(
                                "non-null function pointer",
                            )((len_8 + 1 as libc::c_int) as libc::c_uint);
                        memcpy(
                            (*pDb).zTraceV2 as *mut libc::c_void,
                            zTraceV2 as *const libc::c_void,
                            (len_8 + 1 as libc::c_int) as libc::c_ulong,
                        );
                    } else {
                        let ref mut fresh177 = (*pDb).zTraceV2;
                        *fresh177 = 0 as *mut libc::c_char;
                    }
                    if !((*pDb).zTraceV2).is_null() {
                        let ref mut fresh178 = (*pDb).interp;
                        *fresh178 = interp;
                        sqlite3_trace_v2(
                            (*pDb).db,
                            wMask as libc::c_uint,
                            Some(
                                DbTraceV2Handler
                                    as unsafe extern "C" fn(
                                        libc::c_uint,
                                        *mut libc::c_void,
                                        *mut libc::c_void,
                                        *mut libc::c_void,
                                    ) -> libc::c_int,
                            ),
                            pDb as *mut libc::c_void,
                        );
                    } else {
                        sqlite3_trace_v2(
                            (*pDb).db,
                            0 as libc::c_int as libc::c_uint,
                            None,
                            0 as *mut libc::c_void,
                        );
                    }
                }
            }
        }
        37 => {
            let mut pScript_1: *mut Tcl_Obj = 0 as *mut Tcl_Obj;
            let mut zBegin: *const libc::c_char = b"SAVEPOINT _tcl_transaction\0"
                as *const u8 as *const libc::c_char;
            if objc != 3 as libc::c_int && objc != 4 as libc::c_int {
                ((*tclStubsPtr).tcl_WrongNumArgs)
                    .expect(
                        "non-null function pointer",
                    )(
                    interp,
                    2 as libc::c_int,
                    objv,
                    b"[TYPE] SCRIPT\0" as *const u8 as *const libc::c_char,
                );
                return 1 as libc::c_int;
            }
            if (*pDb).nTransaction == 0 as libc::c_int && objc == 4 as libc::c_int {
                static mut TTYPE_strs_0: [*const libc::c_char; 4] = [
                    b"deferred\0" as *const u8 as *const libc::c_char,
                    b"exclusive\0" as *const u8 as *const libc::c_char,
                    b"immediate\0" as *const u8 as *const libc::c_char,
                    0 as *const libc::c_char,
                ];
                let mut ttype_0: libc::c_int = 0;
                if ((*tclStubsPtr).tcl_GetIndexFromObjStruct)
                    .expect(
                        "non-null function pointer",
                    )(
                    interp,
                    *objv.offset(2 as libc::c_int as isize),
                    TTYPE_strs_0.as_mut_ptr() as *const libc::c_void,
                    ::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong
                        as libc::c_int,
                    b"transaction type\0" as *const u8 as *const libc::c_char,
                    0 as libc::c_int,
                    &mut ttype_0,
                ) != 0
                {
                    return 1 as libc::c_int;
                }
                match ttype_0 as TTYPE_enum as libc::c_uint {
                    1 => {
                        zBegin = b"BEGIN EXCLUSIVE\0" as *const u8
                            as *const libc::c_char;
                    }
                    2 => {
                        zBegin = b"BEGIN IMMEDIATE\0" as *const u8
                            as *const libc::c_char;
                    }
                    0 | _ => {}
                }
            }
            pScript_1 = *objv.offset((objc - 1 as libc::c_int) as isize);
            let ref mut fresh179 = (*pDb).disableAuth;
            *fresh179 += 1;
            rc = sqlite3_exec(
                (*pDb).db,
                zBegin,
                None,
                0 as *mut libc::c_void,
                0 as *mut *mut libc::c_char,
            );
            let ref mut fresh180 = (*pDb).disableAuth;
            *fresh180 -= 1;
            if rc != 0 as libc::c_int {
                ((*tclStubsPtr).tcl_AppendResult)
                    .expect(
                        "non-null function pointer",
                    )(interp, sqlite3_errmsg((*pDb).db), 0 as *mut libc::c_char);
                return 1 as libc::c_int;
            }
            let ref mut fresh181 = (*pDb).nTransaction;
            *fresh181 += 1;
            addDatabaseRef(pDb);
            if DbUseNre() != 0 {
                ((*tclStubsPtr).tcl_NRAddCallback)
                    .expect(
                        "non-null function pointer",
                    )(
                    interp,
                    Some(
                        DbTransPostCmd
                            as unsafe extern "C" fn(
                                *mut ClientData,
                                *mut Tcl_Interp,
                                libc::c_int,
                            ) -> libc::c_int,
                    ),
                    cd,
                    0 as ClientData,
                    0 as ClientData,
                    0 as ClientData,
                );
                ((*tclStubsPtr).tcl_NREvalObj)
                    .expect(
                        "non-null function pointer",
                    )(interp, pScript_1, 0 as libc::c_int);
            } else {
                rc = DbTransPostCmd(
                    &mut cd,
                    interp,
                    ((*tclStubsPtr).tcl_EvalObjEx)
                        .expect(
                            "non-null function pointer",
                        )(interp, pScript_1, 0 as libc::c_int),
                );
            }
        }
        38 => {
            ((*tclStubsPtr).tcl_AppendResult)
                .expect(
                    "non-null function pointer",
                )(
                interp,
                b"unlock_notify not available in this build\0" as *const u8
                    as *const libc::c_char,
                0 as *mut libc::c_char,
            );
            rc = 1 as libc::c_int;
        }
        25 => {
            ((*tclStubsPtr).tcl_AppendResult)
                .expect(
                    "non-null function pointer",
                )(
                interp,
                b"preupdate_hook was omitted at compile-time\0" as *const u8
                    as *const libc::c_char,
                0 as *mut libc::c_char,
            );
            rc = 1 as libc::c_int;
        }
        41 | 39 | 30 => {
            let mut ppHook: *mut *mut Tcl_Obj = 0 as *mut *mut Tcl_Obj;
            if choice == DB_WAL_HOOK as libc::c_int {
                ppHook = &mut (*pDb).pWalHook;
            }
            if choice == DB_UPDATE_HOOK as libc::c_int {
                ppHook = &mut (*pDb).pUpdateHook;
            }
            if choice == DB_ROLLBACK_HOOK as libc::c_int {
                ppHook = &mut (*pDb).pRollbackHook;
            }
            if objc > 3 as libc::c_int {
                ((*tclStubsPtr).tcl_WrongNumArgs)
                    .expect(
                        "non-null function pointer",
                    )(
                    interp,
                    2 as libc::c_int,
                    objv,
                    b"?SCRIPT?\0" as *const u8 as *const libc::c_char,
                );
                return 1 as libc::c_int;
            }
            DbHookCmd(
                interp,
                pDb,
                if objc == 3 as libc::c_int {
                    *objv.offset(2 as libc::c_int as isize)
                } else {
                    0 as *mut Tcl_Obj
                },
                ppHook,
            );
        }
        40 => {
            let mut i_4: libc::c_int = 0;
            i_4 = 2 as libc::c_int;
            if i_4 < objc {
                let mut zArg: *const libc::c_char = ((*tclStubsPtr).tcl_GetString)
                    .expect("non-null function pointer")(*objv.offset(i_4 as isize));
                ((*tclStubsPtr).tcl_AppendResult)
                    .expect(
                        "non-null function pointer",
                    )(
                    interp,
                    b"unknown argument: \0" as *const u8 as *const libc::c_char,
                    zArg,
                    0 as *mut libc::c_char,
                );
                return 1 as libc::c_int;
            }
            if i_4 == 2 as libc::c_int {
                ((*tclStubsPtr).tcl_SetResult)
                    .expect(
                        "non-null function pointer",
                    )(interp, sqlite3_libversion() as *mut libc::c_char, None);
            }
        }
        _ => {}
    }
    return rc;
}
unsafe extern "C" fn DbObjCmdAdaptor(
    mut cd: *mut libc::c_void,
    mut interp: *mut Tcl_Interp,
    mut objc: libc::c_int,
    mut objv: *const *mut Tcl_Obj,
) -> libc::c_int {
    return ((*tclStubsPtr).tcl_NRCallObjProc)
        .expect(
            "non-null function pointer",
        )(
        interp,
        Some(
            DbObjCmd
                as unsafe extern "C" fn(
                    *mut libc::c_void,
                    *mut Tcl_Interp,
                    libc::c_int,
                    *const *mut Tcl_Obj,
                ) -> libc::c_int,
        ),
        cd,
        objc,
        objv,
    );
}
unsafe extern "C" fn sqliteCmdUsage(
    mut interp: *mut Tcl_Interp,
    mut objv: *const *mut Tcl_Obj,
) -> libc::c_int {
    ((*tclStubsPtr).tcl_WrongNumArgs)
        .expect(
            "non-null function pointer",
        )(
        interp,
        1 as libc::c_int,
        objv,
        b"HANDLE ?FILENAME? ?-vfs VFSNAME? ?-readonly BOOLEAN? ?-create BOOLEAN? ?-nofollow BOOLEAN? ?-nomutex BOOLEAN? ?-fullmutex BOOLEAN? ?-uri BOOLEAN?\0"
            as *const u8 as *const libc::c_char,
    );
    return 1 as libc::c_int;
}
unsafe extern "C" fn DbMain(
    mut cd: *mut libc::c_void,
    mut interp: *mut Tcl_Interp,
    mut objc: libc::c_int,
    mut objv: *const *mut Tcl_Obj,
) -> libc::c_int {
    let mut p: *mut SqliteDb = 0 as *mut SqliteDb;
    let mut zArg: *const libc::c_char = 0 as *const libc::c_char;
    let mut zErrMsg: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut i: libc::c_int = 0;
    let mut zFile: *const libc::c_char = 0 as *const libc::c_char;
    let mut zVfs: *const libc::c_char = 0 as *const libc::c_char;
    let mut flags: libc::c_int = 0;
    let mut bTranslateFileName: libc::c_int = 1 as libc::c_int;
    let mut translatedFilename: Tcl_DString = Tcl_DString {
        string: 0 as *mut libc::c_char,
        length: 0,
        spaceAvl: 0,
        staticSpace: [0; 200],
    };
    let mut rc: libc::c_int = 0;
    flags = 0x2 as libc::c_int | 0x4 as libc::c_int | 0x8000 as libc::c_int;
    if objc == 1 as libc::c_int {
        return sqliteCmdUsage(interp, objv);
    }
    if objc == 2 as libc::c_int {
        zArg = ((*tclStubsPtr).tcl_GetStringFromObj)
            .expect(
                "non-null function pointer",
            )(*objv.offset(1 as libc::c_int as isize), 0 as *mut libc::c_int);
        if strcmp(zArg, b"-version\0" as *const u8 as *const libc::c_char)
            == 0 as libc::c_int
        {
            ((*tclStubsPtr).tcl_AppendResult)
                .expect(
                    "non-null function pointer",
                )(interp, sqlite3_libversion(), 0 as *mut libc::c_char);
            return 0 as libc::c_int;
        }
        if strcmp(zArg, b"-sourceid\0" as *const u8 as *const libc::c_char)
            == 0 as libc::c_int
        {
            ((*tclStubsPtr).tcl_AppendResult)
                .expect(
                    "non-null function pointer",
                )(interp, sqlite3_sourceid(), 0 as *mut libc::c_char);
            return 0 as libc::c_int;
        }
        if strcmp(zArg, b"-has-codec\0" as *const u8 as *const libc::c_char)
            == 0 as libc::c_int
        {
            ((*tclStubsPtr).tcl_AppendResult)
                .expect(
                    "non-null function pointer",
                )(
                interp,
                b"0\0" as *const u8 as *const libc::c_char,
                0 as *mut libc::c_char,
            );
            return 0 as libc::c_int;
        }
        if *zArg.offset(0 as libc::c_int as isize) as libc::c_int == '-' as i32 {
            return sqliteCmdUsage(interp, objv);
        }
    }
    i = 2 as libc::c_int;
    while i < objc {
        zArg = ((*tclStubsPtr).tcl_GetString)
            .expect("non-null function pointer")(*objv.offset(i as isize));
        if *zArg.offset(0 as libc::c_int as isize) as libc::c_int != '-' as i32 {
            if !zFile.is_null() {
                return sqliteCmdUsage(interp, objv);
            }
            zFile = zArg;
        } else {
            if i == objc - 1 as libc::c_int {
                return sqliteCmdUsage(interp, objv);
            }
            i += 1;
            if !(strcmp(zArg, b"-key\0" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int)
            {
                if strcmp(zArg, b"-vfs\0" as *const u8 as *const libc::c_char)
                    == 0 as libc::c_int
                {
                    zVfs = ((*tclStubsPtr).tcl_GetString)
                        .expect("non-null function pointer")(*objv.offset(i as isize));
                } else if strcmp(
                    zArg,
                    b"-readonly\0" as *const u8 as *const libc::c_char,
                ) == 0 as libc::c_int
                {
                    let mut b: libc::c_int = 0;
                    if ((*tclStubsPtr).tcl_GetBooleanFromObj)
                        .expect(
                            "non-null function pointer",
                        )(interp, *objv.offset(i as isize), &mut b) != 0
                    {
                        return 1 as libc::c_int;
                    }
                    if b != 0 {
                        flags &= !(0x2 as libc::c_int | 0x4 as libc::c_int);
                        flags |= 0x1 as libc::c_int;
                    } else {
                        flags &= !(0x1 as libc::c_int);
                        flags |= 0x2 as libc::c_int;
                    }
                } else if strcmp(zArg, b"-create\0" as *const u8 as *const libc::c_char)
                    == 0 as libc::c_int
                {
                    let mut b_0: libc::c_int = 0;
                    if ((*tclStubsPtr).tcl_GetBooleanFromObj)
                        .expect(
                            "non-null function pointer",
                        )(interp, *objv.offset(i as isize), &mut b_0) != 0
                    {
                        return 1 as libc::c_int;
                    }
                    if b_0 != 0 && flags & 0x1 as libc::c_int == 0 as libc::c_int {
                        flags |= 0x4 as libc::c_int;
                    } else {
                        flags &= !(0x4 as libc::c_int);
                    }
                } else if strcmp(
                    zArg,
                    b"-nofollow\0" as *const u8 as *const libc::c_char,
                ) == 0 as libc::c_int
                {
                    let mut b_1: libc::c_int = 0;
                    if ((*tclStubsPtr).tcl_GetBooleanFromObj)
                        .expect(
                            "non-null function pointer",
                        )(interp, *objv.offset(i as isize), &mut b_1) != 0
                    {
                        return 1 as libc::c_int;
                    }
                    if b_1 != 0 {
                        flags |= 0x1000000 as libc::c_int;
                    } else {
                        flags &= !(0x1000000 as libc::c_int);
                    }
                } else if strcmp(zArg, b"-nomutex\0" as *const u8 as *const libc::c_char)
                    == 0 as libc::c_int
                {
                    let mut b_2: libc::c_int = 0;
                    if ((*tclStubsPtr).tcl_GetBooleanFromObj)
                        .expect(
                            "non-null function pointer",
                        )(interp, *objv.offset(i as isize), &mut b_2) != 0
                    {
                        return 1 as libc::c_int;
                    }
                    if b_2 != 0 {
                        flags |= 0x8000 as libc::c_int;
                        flags &= !(0x10000 as libc::c_int);
                    } else {
                        flags &= !(0x8000 as libc::c_int);
                    }
                } else if strcmp(
                    zArg,
                    b"-fullmutex\0" as *const u8 as *const libc::c_char,
                ) == 0 as libc::c_int
                {
                    let mut b_3: libc::c_int = 0;
                    if ((*tclStubsPtr).tcl_GetBooleanFromObj)
                        .expect(
                            "non-null function pointer",
                        )(interp, *objv.offset(i as isize), &mut b_3) != 0
                    {
                        return 1 as libc::c_int;
                    }
                    if b_3 != 0 {
                        flags |= 0x10000 as libc::c_int;
                        flags &= !(0x8000 as libc::c_int);
                    } else {
                        flags &= !(0x10000 as libc::c_int);
                    }
                } else if strcmp(zArg, b"-uri\0" as *const u8 as *const libc::c_char)
                    == 0 as libc::c_int
                {
                    let mut b_4: libc::c_int = 0;
                    if ((*tclStubsPtr).tcl_GetBooleanFromObj)
                        .expect(
                            "non-null function pointer",
                        )(interp, *objv.offset(i as isize), &mut b_4) != 0
                    {
                        return 1 as libc::c_int;
                    }
                    if b_4 != 0 {
                        flags |= 0x40 as libc::c_int;
                    } else {
                        flags &= !(0x40 as libc::c_int);
                    }
                } else if strcmp(
                    zArg,
                    b"-translatefilename\0" as *const u8 as *const libc::c_char,
                ) == 0 as libc::c_int
                {
                    if ((*tclStubsPtr).tcl_GetBooleanFromObj)
                        .expect(
                            "non-null function pointer",
                        )(interp, *objv.offset(i as isize), &mut bTranslateFileName) != 0
                    {
                        return 1 as libc::c_int;
                    }
                } else {
                    ((*tclStubsPtr).tcl_AppendResult)
                        .expect(
                            "non-null function pointer",
                        )(
                        interp,
                        b"unknown option: \0" as *const u8 as *const libc::c_char,
                        zArg,
                        0 as *mut libc::c_char,
                    );
                    return 1 as libc::c_int;
                }
            }
        }
        i += 1;
    }
    zErrMsg = 0 as *mut libc::c_char;
    p = ((*tclStubsPtr).tcl_Alloc)
        .expect(
            "non-null function pointer",
        )(::std::mem::size_of::<SqliteDb>() as libc::c_ulong as libc::c_uint)
        as *mut SqliteDb;
    memset(
        p as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<SqliteDb>() as libc::c_ulong,
    );
    if zFile.is_null() {
        zFile = b"\0" as *const u8 as *const libc::c_char;
    }
    if bTranslateFileName != 0 {
        zFile = ((*tclStubsPtr).tcl_TranslateFileName)
            .expect("non-null function pointer")(interp, zFile, &mut translatedFilename);
    }
    rc = sqlite3_open_v2(zFile, &mut (*p).db, flags, zVfs);
    if bTranslateFileName != 0 {
        ((*tclStubsPtr).tcl_DStringFree)
            .expect("non-null function pointer")(&mut translatedFilename);
    }
    if !((*p).db).is_null() {
        if 0 as libc::c_int != sqlite3_errcode((*p).db) {
            zErrMsg = sqlite3_mprintf(
                b"%s\0" as *const u8 as *const libc::c_char,
                sqlite3_errmsg((*p).db),
            );
            sqlite3_close((*p).db);
            let ref mut fresh182 = (*p).db;
            *fresh182 = 0 as *mut sqlite3;
        }
    } else {
        zErrMsg = sqlite3_mprintf(
            b"%s\0" as *const u8 as *const libc::c_char,
            sqlite3_errstr(rc),
        );
    }
    if ((*p).db).is_null() {
        ((*tclStubsPtr).tcl_SetResult)
            .expect(
                "non-null function pointer",
            )(
            interp,
            zErrMsg,
            ::std::mem::transmute::<
                libc::intptr_t,
                Option::<Tcl_FreeProc>,
            >(1 as libc::c_int as libc::intptr_t),
        );
        ((*tclStubsPtr).tcl_Free)
            .expect("non-null function pointer")(p as *mut libc::c_char);
        sqlite3_free(zErrMsg as *mut libc::c_void);
        return 1 as libc::c_int;
    }
    (*p).maxStmt = 10 as libc::c_int;
    (*p).openFlags = flags & 0x40 as libc::c_int;
    let ref mut fresh183 = (*p).interp;
    *fresh183 = interp;
    zArg = ((*tclStubsPtr).tcl_GetStringFromObj)
        .expect(
            "non-null function pointer",
        )(*objv.offset(1 as libc::c_int as isize), 0 as *mut libc::c_int);
    if DbUseNre() != 0 {
        ((*tclStubsPtr).tcl_NRCreateCommand)
            .expect(
                "non-null function pointer",
            )(
            interp,
            zArg,
            Some(
                DbObjCmdAdaptor
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        *mut Tcl_Interp,
                        libc::c_int,
                        *const *mut Tcl_Obj,
                    ) -> libc::c_int,
            ),
            Some(
                DbObjCmd
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        *mut Tcl_Interp,
                        libc::c_int,
                        *const *mut Tcl_Obj,
                    ) -> libc::c_int,
            ),
            p as *mut libc::c_char as ClientData,
            Some(DbDeleteCmd as unsafe extern "C" fn(*mut libc::c_void) -> ()),
        );
    } else {
        ((*tclStubsPtr).tcl_CreateObjCommand)
            .expect(
                "non-null function pointer",
            )(
            interp,
            zArg,
            Some(
                DbObjCmd
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        *mut Tcl_Interp,
                        libc::c_int,
                        *const *mut Tcl_Obj,
                    ) -> libc::c_int,
            ),
            p as *mut libc::c_char as ClientData,
            Some(DbDeleteCmd as unsafe extern "C" fn(*mut libc::c_void) -> ()),
        );
    }
    (*p).nRef = 1 as libc::c_int;
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn Sqlite3_Init(mut interp: *mut Tcl_Interp) -> libc::c_int {
    let mut rc: libc::c_int = if !(Tcl_InitStubs(
        interp,
        b"8.4\0" as *const u8 as *const libc::c_char,
        0 as libc::c_int,
    ))
        .is_null()
    {
        0 as libc::c_int
    } else {
        1 as libc::c_int
    };
    if rc == 0 as libc::c_int {
        ((*tclStubsPtr).tcl_CreateObjCommand)
            .expect(
                "non-null function pointer",
            )(
            interp,
            b"sqlite3\0" as *const u8 as *const libc::c_char,
            ::std::mem::transmute::<
                Option::<
                    unsafe extern "C" fn(
                        *mut libc::c_void,
                        *mut Tcl_Interp,
                        libc::c_int,
                        *const *mut Tcl_Obj,
                    ) -> libc::c_int,
                >,
                Option::<Tcl_ObjCmdProc>,
            >(
                Some(
                    DbMain
                        as unsafe extern "C" fn(
                            *mut libc::c_void,
                            *mut Tcl_Interp,
                            libc::c_int,
                            *const *mut Tcl_Obj,
                        ) -> libc::c_int,
                ),
            ),
            0 as ClientData,
            None,
        );
        ((*tclStubsPtr).tcl_CreateObjCommand)
            .expect(
                "non-null function pointer",
            )(
            interp,
            b"sqlite\0" as *const u8 as *const libc::c_char,
            ::std::mem::transmute::<
                Option::<
                    unsafe extern "C" fn(
                        *mut libc::c_void,
                        *mut Tcl_Interp,
                        libc::c_int,
                        *const *mut Tcl_Obj,
                    ) -> libc::c_int,
                >,
                Option::<Tcl_ObjCmdProc>,
            >(
                Some(
                    DbMain
                        as unsafe extern "C" fn(
                            *mut libc::c_void,
                            *mut Tcl_Interp,
                            libc::c_int,
                            *const *mut Tcl_Obj,
                        ) -> libc::c_int,
                ),
            ),
            0 as ClientData,
            None,
        );
        rc = ((*tclStubsPtr).tcl_PkgProvideEx)
            .expect(
                "non-null function pointer",
            )(
            interp,
            b"sqlite3\0" as *const u8 as *const libc::c_char,
            b"3.41.0\0" as *const u8 as *const libc::c_char,
            0 as *const libc::c_void,
        );
    }
    return rc;
}
#[no_mangle]
pub unsafe extern "C" fn Tclsqlite3_Init(mut interp: *mut Tcl_Interp) -> libc::c_int {
    return Sqlite3_Init(interp);
}
#[no_mangle]
pub unsafe extern "C" fn Sqlite3_Unload(
    mut interp: *mut Tcl_Interp,
    mut flags: libc::c_int,
) -> libc::c_int {
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn Tclsqlite3_Unload(
    mut interp: *mut Tcl_Interp,
    mut flags: libc::c_int,
) -> libc::c_int {
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn Sqlite3_SafeInit(mut interp: *mut Tcl_Interp) -> libc::c_int {
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn Sqlite3_SafeUnload(
    mut interp: *mut Tcl_Interp,
    mut flags: libc::c_int,
) -> libc::c_int {
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn Sqlite_Init(mut interp: *mut Tcl_Interp) -> libc::c_int {
    return Sqlite3_Init(interp);
}
#[no_mangle]
pub unsafe extern "C" fn Tclsqlite_Init(mut interp: *mut Tcl_Interp) -> libc::c_int {
    return Sqlite3_Init(interp);
}
#[no_mangle]
pub unsafe extern "C" fn Sqlite_Unload(
    mut interp: *mut Tcl_Interp,
    mut flags: libc::c_int,
) -> libc::c_int {
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn Tclsqlite_Unload(
    mut interp: *mut Tcl_Interp,
    mut flags: libc::c_int,
) -> libc::c_int {
    return 0 as libc::c_int;
}
