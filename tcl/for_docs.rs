macro_rules! t {
    () => {
        eprintln!("The crate clib failed in probing tcl library for the crate tcl, any invocation of Tcl's C API will abort the program.");
        std::process::abort();
    }
}
pub type ClientData = *mut ::std::os::raw::c_void;
pub fn Tcl_Alloc(size: ::std::os::raw::c_uint) -> *mut ::std::os::raw::c_char {t!();}
pub type Tcl_AppInitProc =
    ::std::option::Option<unsafe extern "C" fn(interp: *mut Tcl_Interp) -> ::std::os::raw::c_int>;
pub const TCL_APPEND_VALUE: u32 = 4;
pub type Tcl_CmdDeleteProc = ::std::option::Option<unsafe extern "C" fn(clientData: ClientData)>;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Tcl_Command_ {
    _unused: [u8; 0],
}
pub type Tcl_Command = *mut Tcl_Command_;
pub fn Tcl_CreateInterp() -> *mut Tcl_Interp {t!();}
pub fn Tcl_CreateObjCommand(
    interp: *mut Tcl_Interp,
    cmdName: *const ::std::os::raw::c_char,
    proc_: Tcl_ObjCmdProc,
    clientData: ClientData,
    deleteProc: Tcl_CmdDeleteProc,
) -> Tcl_Command {t!();}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Tcl_DString {
    pub string: *mut ::std::os::raw::c_char,
    pub length: ::std::os::raw::c_int,
    pub spaceAvl: ::std::os::raw::c_int,
    pub staticSpace: [::std::os::raw::c_char; 200usize],
}
pub fn Tcl_DStringInit(dsPtr: *mut Tcl_DString) {t!();}
pub fn Tcl_DStringAppendElement(
    dsPtr: *mut Tcl_DString,
    element: *const ::std::os::raw::c_char,
) -> *mut ::std::os::raw::c_char {t!();}
pub fn Tcl_DStringStartSublist(dsPtr: *mut Tcl_DString) {t!();}
pub fn Tcl_DStringEndSublist(dsPtr: *mut Tcl_DString) {t!();}
pub fn Tcl_DStringFree(dsPtr: *mut Tcl_DString) {t!();}
pub fn Tcl_DeleteInterp(interp: *mut Tcl_Interp) {t!();}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Tcl_Dict_ {
    _unused: [u8; 0],
}
pub type Tcl_Dict = *mut Tcl_Dict_;
pub fn Tcl_DictObjFirst(
    interp: *mut Tcl_Interp,
    dictPtr: *mut Tcl_Obj,
    searchPtr: *mut Tcl_DictSearch,
    keyPtrPtr: *mut *mut Tcl_Obj,
    valuePtrPtr: *mut *mut Tcl_Obj,
    donePtr: *mut ::std::os::raw::c_int,
) -> ::std::os::raw::c_int {t!();}
pub fn Tcl_DictObjGet(
    interp: *mut Tcl_Interp,
    dictPtr: *mut Tcl_Obj,
    keyPtr: *mut Tcl_Obj,
    valuePtrPtr: *mut *mut Tcl_Obj,
) -> ::std::os::raw::c_int {t!();}
pub fn Tcl_DictObjNext(
    searchPtr: *mut Tcl_DictSearch,
    keyPtrPtr: *mut *mut Tcl_Obj,
    valuePtrPtr: *mut *mut Tcl_Obj,
    donePtr: *mut ::std::os::raw::c_int,
) {t!();}
pub fn Tcl_DictObjPut(
    interp: *mut Tcl_Interp,
    dictPtr: *mut Tcl_Obj,
    keyPtr: *mut Tcl_Obj,
    valuePtr: *mut Tcl_Obj,
) -> ::std::os::raw::c_int {t!();}
pub fn Tcl_DictObjRemove(
    interp: *mut Tcl_Interp,
    dictPtr: *mut Tcl_Obj,
    keyPtr: *mut Tcl_Obj,
) -> ::std::os::raw::c_int {t!();}
pub fn Tcl_DictObjSize(
    interp: *mut Tcl_Interp,
    dictPtr: *mut Tcl_Obj,
    sizePtr: *mut ::std::os::raw::c_int,
) -> ::std::os::raw::c_int {t!();}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Tcl_DictSearch {
    pub next: *mut ::std::os::raw::c_void,
    pub epoch: ::std::os::raw::c_int,
    pub dictionaryPtr: Tcl_Dict,
}
pub fn Tcl_DoOneEvent(flags: ::std::os::raw::c_int) -> ::std::os::raw::c_int {t!();}
pub type Tcl_DupInternalRepProc =
    ::std::option::Option<unsafe extern "C" fn(srcPtr: *mut Tcl_Obj, dupPtr: *mut Tcl_Obj)>;
pub const TCL_ERROR: u32 = 1;
pub fn Tcl_EvalObjEx(
    interp: *mut Tcl_Interp,
    objPtr: *mut Tcl_Obj,
    flags: ::std::os::raw::c_int,
) -> ::std::os::raw::c_int {t!();}
pub fn Tcl_FindExecutable(argv0: *const ::std::os::raw::c_char) {t!();}
pub type Tcl_FreeInternalRepProc =
    ::std::option::Option<unsafe extern "C" fn(objPtr: *mut Tcl_Obj)>;
pub fn Tcl_GetBooleanFromObj(
    interp: *mut Tcl_Interp,
    objPtr: *mut Tcl_Obj,
    intPtr: *mut ::std::os::raw::c_int,
) -> ::std::os::raw::c_int {t!();}
pub fn Tcl_GetDoubleFromObj(
    interp: *mut Tcl_Interp,
    objPtr: *mut Tcl_Obj,
    doublePtr: *mut f64,
) -> ::std::os::raw::c_int {t!();}
pub fn Tcl_GetIntFromObj(
    interp: *mut Tcl_Interp,
    objPtr: *mut Tcl_Obj,
    intPtr: *mut ::std::os::raw::c_int,
) -> ::std::os::raw::c_int {t!();}
pub fn Tcl_GetLongFromObj(
    interp: *mut Tcl_Interp,
    objPtr: *mut Tcl_Obj,
    longPtr: *mut ::std::os::raw::c_long,
) -> ::std::os::raw::c_int {t!();}
pub fn Tcl_GetNameOfExecutable() -> *const ::std::os::raw::c_char {t!();}
pub fn Tcl_GetObjResult(interp: *mut Tcl_Interp) -> *mut Tcl_Obj {t!();}
pub fn Tcl_GetObjType(typeName: *const ::std::os::raw::c_char) -> *const Tcl_ObjType {t!();}
pub fn Tcl_GetReturnOptions(
    interp: *mut Tcl_Interp,
    result: ::std::os::raw::c_int,
) -> *mut Tcl_Obj {t!();}
pub fn Tcl_GetStringFromObj(
    objPtr: *mut Tcl_Obj,
    lengthPtr: *mut ::std::os::raw::c_int,
) -> *mut ::std::os::raw::c_char {t!();}
pub fn Tcl_GetWideIntFromObj(
    interp: *mut Tcl_Interp,
    objPtr: *mut Tcl_Obj,
    widePtr: *mut Tcl_WideInt,
) -> ::std::os::raw::c_int {t!();}
pub fn Tcl_Init(interp: *mut Tcl_Interp) -> ::std::os::raw::c_int {t!();}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Tcl_Interp {
    pub resultDontUse: *mut ::std::os::raw::c_char,
    pub freeProcDontUse:
        ::std::option::Option<unsafe extern "C" fn(arg1: *mut ::std::os::raw::c_char)>,
    pub errorLineDontUse: ::std::os::raw::c_int,
}
pub fn Tcl_InvalidateStringRep(objPtr: *mut Tcl_Obj) {t!();}
pub const TCL_LEAVE_ERR_MSG: u32 = 512;
pub const TCL_LIST_ELEMENT: u32 = 8;
pub fn Tcl_ListObjAppendElement(
    interp: *mut Tcl_Interp,
    listPtr: *mut Tcl_Obj,
    objPtr: *mut Tcl_Obj,
) -> ::std::os::raw::c_int {t!();}
pub fn Tcl_ListObjAppendList(
    interp: *mut Tcl_Interp,
    listPtr: *mut Tcl_Obj,
    elemListPtr: *mut Tcl_Obj,
) -> ::std::os::raw::c_int {t!();}
pub fn Tcl_ListObjGetElements(
    interp: *mut Tcl_Interp,
    listPtr: *mut Tcl_Obj,
    objcPtr: *mut ::std::os::raw::c_int,
    objvPtr: *mut *mut *mut Tcl_Obj,
) -> ::std::os::raw::c_int {t!();}
pub fn Tcl_ListObjIndex(
    interp: *mut Tcl_Interp,
    listPtr: *mut Tcl_Obj,
    index: ::std::os::raw::c_int,
    objPtrPtr: *mut *mut Tcl_Obj,
) -> ::std::os::raw::c_int {t!();}
pub fn Tcl_ListObjLength(
    interp: *mut Tcl_Interp,
    listPtr: *mut Tcl_Obj,
    lengthPtr: *mut ::std::os::raw::c_int,
) -> ::std::os::raw::c_int {t!();}
pub fn Tcl_ListObjReplace(
    interp: *mut Tcl_Interp,
    listPtr: *mut Tcl_Obj,
    first: ::std::os::raw::c_int,
    count: ::std::os::raw::c_int,
    objc: ::std::os::raw::c_int,
    objv: *const *mut Tcl_Obj,
) -> ::std::os::raw::c_int {t!();}
pub fn Tcl_NewDictObj() -> *mut Tcl_Obj {t!();}
pub fn Tcl_NewDoubleObj(doubleValue: f64) -> *mut Tcl_Obj {t!();}
pub fn Tcl_NewIntObj(intValue: ::std::os::raw::c_int) -> *mut Tcl_Obj {t!();}
pub fn Tcl_NewListObj(objc: ::std::os::raw::c_int, objv: *const *mut Tcl_Obj) -> *mut Tcl_Obj {t!();}
pub fn Tcl_NewObj() -> *mut Tcl_Obj {t!();}
pub fn Tcl_NewStringObj(
    bytes: *const ::std::os::raw::c_char,
    length: ::std::os::raw::c_int,
) -> *mut Tcl_Obj {t!();}
pub fn Tcl_NewWideIntObj(wideValue: Tcl_WideInt) -> *mut Tcl_Obj {t!();}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct Tcl_Obj {
    pub refCount: ::std::os::raw::c_int,
    pub bytes: *mut ::std::os::raw::c_char,
    pub length: ::std::os::raw::c_int,
    pub typePtr: *const Tcl_ObjType,
    pub internalRep: Tcl_Obj__bindgen_ty_1,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union Tcl_Obj__bindgen_ty_1 {
    pub longValue: ::std::os::raw::c_long,
    pub doubleValue: f64,
    pub otherValuePtr: *mut ::std::os::raw::c_void,
    pub wideValue: Tcl_WideInt,
    pub twoPtrValue: Tcl_Obj__bindgen_ty_1__bindgen_ty_1,
    pub ptrAndLongRep: Tcl_Obj__bindgen_ty_1__bindgen_ty_2,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Tcl_Obj__bindgen_ty_1__bindgen_ty_1 {
    pub ptr1: *mut ::std::os::raw::c_void,
    pub ptr2: *mut ::std::os::raw::c_void,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Tcl_Obj__bindgen_ty_1__bindgen_ty_2 {
    pub ptr: *mut ::std::os::raw::c_void,
    pub value: ::std::os::raw::c_ulong,
}
pub fn Tcl_ObjGetVar2(
    interp: *mut Tcl_Interp,
    part1Ptr: *mut Tcl_Obj,
    part2Ptr: *mut Tcl_Obj,
    flags: ::std::os::raw::c_int,
) -> *mut Tcl_Obj {t!();}
pub fn Tcl_ObjSetVar2(
    interp: *mut Tcl_Interp,
    part1Ptr: *mut Tcl_Obj,
    part2Ptr: *mut Tcl_Obj,
    newValuePtr: *mut Tcl_Obj,
    flags: ::std::os::raw::c_int,
) -> *mut Tcl_Obj {t!();}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Tcl_ObjType {
    pub name: *const ::std::os::raw::c_char,
    pub freeIntRepProc: Tcl_FreeInternalRepProc,
    pub dupIntRepProc: Tcl_DupInternalRepProc,
    pub updateStringProc: Tcl_UpdateStringProc,
    pub setFromAnyProc: Tcl_SetFromAnyProc,
}
pub const TCL_OK: u32 = 0;
pub fn Tcl_Panic(format: *const ::std::os::raw::c_char) -> ! {t!();}
pub fn Tcl_RegisterObjType(typePtr: *const Tcl_ObjType) {t!();}
pub type Tcl_SetFromAnyProc = ::std::option::Option<
    unsafe extern "C" fn(interp: *mut Tcl_Interp, objPtr: *mut Tcl_Obj) -> ::std::os::raw::c_int,
>;
pub fn Tcl_SetListObj(
    objPtr: *mut Tcl_Obj,
    objc: ::std::os::raw::c_int,
    objv: *const *mut Tcl_Obj,
) {t!();}
pub fn Tcl_SetObjErrorCode(interp: *mut Tcl_Interp, errorObjPtr: *mut Tcl_Obj) {t!();}
pub fn Tcl_SetObjResult(interp: *mut Tcl_Interp, resultObjPtr: *mut Tcl_Obj) {t!();}
pub type Tcl_ObjCmdProc = ::std::option::Option<
    unsafe extern "C" fn(
        clientData: ClientData,
        interp: *mut Tcl_Interp,
        objc: ::std::os::raw::c_int,
        objv: *const *mut Tcl_Obj,
    ) -> ::std::os::raw::c_int,
>;
pub fn Tcl_UnsetVar(
    interp: *mut Tcl_Interp,
    varName: *const ::std::os::raw::c_char,
    flags: ::std::os::raw::c_int,
) -> ::std::os::raw::c_int {t!();}
pub fn Tcl_UnsetVar2(
    interp: *mut Tcl_Interp,
    part1: *const ::std::os::raw::c_char,
    part2: *const ::std::os::raw::c_char,
    flags: ::std::os::raw::c_int,
) -> ::std::os::raw::c_int {t!();}
pub type Tcl_UpdateStringProc = ::std::option::Option<unsafe extern "C" fn(objPtr: *mut Tcl_Obj)>;
pub type Tcl_WideInt = ::std::os::raw::c_longlong;
pub fn Tcl_WrongNumArgs(
    interp: *mut Tcl_Interp,
    objc: ::std::os::raw::c_int,
    objv: *const *mut Tcl_Obj,
    message: *const ::std::os::raw::c_char,
) {t!();}
pub fn TclFreeObj(objPtr: *mut Tcl_Obj) {t!();}
