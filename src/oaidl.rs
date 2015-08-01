// Copyright © 2015, Peter Atashian
// Licensed under the MIT License <LICENSE.md>

//253
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct CYinternal {
    pub Lo: ::c_ulong,
    pub Hi: ::c_long,
}

#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct CY {
    pub int64: ::LONGLONG,
}
UNION!(CY, int64, internal, internal_mut, ::CYinternal);
pub type CURRENCY = CY;

#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct SAFEARRAYBOUND {
    pub cElements: ::ULONG,
    pub lLbound: ::LONG,
}
pub type LPSAFEARRAYBOUND = *mut SAFEARRAYBOUND;

//351
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct SAFEARRAY {
    pub cDims: ::USHORT,
    pub fFeatures: ::USHORT,
    pub cbElements: ::ULONG,
    pub cLocks: ::ULONG,
    pub pvData: ::PVOID,
    pub rgsabound: [SAFEARRAYBOUND; 1],
}
pub type LPSAFEARRAY = *mut SAFEARRAY;

pub const FADF_AUTO: ::USHORT = 0x1;
pub const FADF_STATIC: ::USHORT = 0x2;
pub const FADF_EMBEDDED: ::USHORT = 0x4;
pub const FADF_FIXEDSIZE: ::USHORT = 0x10;
pub const FADF_RECORD: ::USHORT = 0x20;
pub const FADF_HAVEIID: ::USHORT = 0x40;
pub const FADF_HAVEVARTYPE: ::USHORT = 0x80;
pub const FADF_BSTR: ::USHORT = 0x100;
pub const FADF_UNKNOWN: ::USHORT = 0x200;
pub const FADF_DISPATCH: ::USHORT = 0x400;
pub const FADF_VARIANT: ::USHORT = 0x800;
pub const FADF_RESERVED: ::USHORT = 0xf008;

#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct __BRECORD {
    pub pvRecord: ::PVOID,
    // pRecInfo: *mut ::IRecordInfo,
}

#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct __VARIANT {
    pub vt: ::VARTYPE,
    pub wReserved1: ::WORD,
    pub wReserved2: ::WORD,
    pub wReserved3: ::WORD,
    pub n3: [u8; 8],
}
UNION!(__VARIANT, n3, decVal, decVal_mut, ::DECIMAL);
UNION!(__VARIANT, n3, llVal, llVal_mut, ::LONGLONG);
UNION!(__VARIANT, n3, lVal, lVal_mut, ::LONG);
UNION!(__VARIANT, n3, bVal, bVal_mut, ::BYTE);
UNION!(__VARIANT, n3, iVal, iVal_mut, ::SHORT);
UNION!(__VARIANT, n3, fltVal, fltVal_mut, ::FLOAT);
UNION!(__VARIANT, n3, dblVal, dblVal_mut, ::DOUBLE);
UNION!(__VARIANT, n3, boolVal, boolVal_mut, ::VARIANT_BOOL);
UNION!(__VARIANT, n3, bool, bool_mut, ::_VARIANT_BOOL);
UNION!(__VARIANT, n3, scode, scode_mut, ::SCODE);
UNION!(__VARIANT, n3, cyVal, cyVal_mut, ::CY);
UNION!(__VARIANT, n3, date, date_mut, ::DATE);
UNION!(__VARIANT, n3, bstrVal, bstrVal_mut, ::BSTR);

UNION!(__VARIANT, n3, punkVal, punkVal_mut, *mut ::IUnknown);
UNION!(__VARIANT, n3, pdispVal, pdispVal_mut, *mut ::IDispatch);
UNION!(__VARIANT, n3, parray, parray_mut, *mut ::SAFEARRAY);
UNION!(__VARIANT, n3, pbVal, pbVal_mut, *mut ::BYTE);
UNION!(__VARIANT, n3, piVal, piVal_mut, *mut ::SHORT);
UNION!(__VARIANT, n3, plVal, plVal_mut, *mut ::LONG);
UNION!(__VARIANT, n3, pllVal, pllVal_mut, *mut ::LONGLONG);
UNION!(__VARIANT, n3, pfltVal, pfltVal_mut, *mut ::FLOAT);
UNION!(__VARIANT, n3, pdblVal, pdblVal_mut, *mut ::DOUBLE);
UNION!(__VARIANT, n3, pboolVal, pboolVal_mut, *mut ::VARIANT_BOOL);
UNION!(__VARIANT, n3, pbool, pbool_mut, *mut ::_VARIANT_BOOL);
UNION!(__VARIANT, n3, pscode, pscode_mut, *mut ::SCODE);
UNION!(__VARIANT, n3, pcyVal, pcyVal_mut, *mut ::CY);
UNION!(__VARIANT, n3, pdate, pdate_mut, *mut ::DATE);
UNION!(__VARIANT, n3, pbstrVal, pbstrVal_mut, *mut ::BSTR);

UNION!(__VARIANT, n3, ppunkVal, ppunkVal_mut, *mut *mut ::IUnknown);
UNION!(__VARIANT, n3, ppdispVal, ppdispVal_mut, *mut *mut ::IDispatch);
UNION!(__VARIANT, n3, pparray, pparray_mut, *mut *mut ::SAFEARRAY);

UNION!(__VARIANT, n3, pvarVal, pvarVal_mut, *mut ::VARIANT);

UNION!(__VARIANT, n3, byref, byref_mut, ::PVOID);
UNION!(__VARIANT, n3, cVal, cVal_mut, ::CHAR);
UNION!(__VARIANT, n3, uiVal, uiVal_mut, ::USHORT);
UNION!(__VARIANT, n3, ulVal, ulVal_mut, ::ULONG);
UNION!(__VARIANT, n3, ullVal, ullVal_mut, ::ULONGLONG);
UNION!(__VARIANT, n3, intVal, intVal_mut, ::INT);
UNION!(__VARIANT, n3, uintVal, uintVal_mut, ::UINT);

UNION!(__VARIANT, n3, pdecVal, pdecVal_mut, *mut ::DECIMAL);
UNION!(__VARIANT, n3, pcVal, pcVal_mut, *mut ::CHAR);
UNION!(__VARIANT, n3, puiVal, puiVal_mut, *mut ::USHORT);
UNION!(__VARIANT, n3, pulVal, pulVal_mut, *mut ::ULONG);
UNION!(__VARIANT, n3, pullVal, pullVal_mut, *mut ::ULONGLONG);
UNION!(__VARIANT, n3, pintVal, pintVal_mut, *mut ::INT);
UNION!(__VARIANT, n3, puintVal, puintVal_mut, *mut ::UINT);


#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct VARIANT {
    pub n1: [u8; 16],
}
UNION!(VARIANT, n1, decVal, decVal_mut, ::DECIMAL);
UNION!(VARIANT, n1, n2, n2_mut, __VARIANT);

pub type LPVARIANT = *mut VARIANT;
pub type VARIANTARG = VARIANT;
pub type LPVARIANTARG = *mut VARIANT;

pub type DISPID = ::LONG;
pub type MEMBERID = DISPID;
pub type HREFTYPE = ::DWORD;

#[repr(i32)] #[derive(Clone, Copy, Debug)]
pub enum TYPEKIND {
    TKIND_ENUM = 0,
    TKIND_RECORD = 1,
    TKIND_MODULE = 2,
    TKIND_INTERFACE = 3,
    TKIND_DISPATCH  = 4,
    TKIND_COCLASS = 5,
    TKIND_ALIAS = 6,
    TKIND_UNION = 7,
    TKIND_MAX = 8,
}

#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct TYPEDESC {
    pub DUMMYUNIONNAME: [u8; 6],
    pub vt: ::VARTYPE,
}
UNION!(TYPEDESC, DUMMYUNIONNAME, lptdesc, lptdesc_mut, *mut TYPEDESC);
UNION!(TYPEDESC, DUMMYUNIONNAME, lpadesc, lpadesc_mut, *mut ARRAYDESC);
UNION!(TYPEDESC, DUMMYUNIONNAME, hreftype, hreftype_mut, *mut ::HREFTYPE);

#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct ARRAYDESC {
    pub tdescElem: TYPEDESC,
    pub cDims: ::USHORT,
    pub rgbounds: [SAFEARRAYBOUND; 1],
}

#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct PARAMDESCEX {
    pub cBytes: ::ULONG,
    pub varDefaultValue: ::VARIANTARG,
}
pub type LPPARAMDESCEX = *mut PARAMDESCEX;

#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct PARAMDESC {
    pub pparamdescex: ::LPPARAMDESCEX,
    pub wParamFlags: ::USHORT,
}
pub type LPPARAMDESC = *mut PARAMDESC;

pub const PARAMFLAG_NONE: ::USHORT = 0;
pub const PARAMFLAG_FIN: ::USHORT = 0x1;
pub const PARAMFLAG_FOUT: ::USHORT = 0x2;
pub const PARAMFLAG_FLCID: ::USHORT = 0x4;
pub const PARAMFLAG_FRETVAL: ::USHORT = 0x8;
pub const PARAMFLAG_FOPT: ::USHORT = 0x10;
pub const PARAMFLAG_FHASDEFAULT: ::USHORT = 0x20;
pub const PARAMFLAG_FHASCUSTDATA: ::USHORT = 0x40;

#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct IDLDESC {
    pub dwReserved: ::ULONG_PTR,
    pub wIDLFlags: ::USHORT,
}
pub type LPIDLDESC = *mut IDLDESC;

pub const IDLFLAG_NONE: ::USHORT = PARAMFLAG_NONE;
pub const IDLFLAG_FIN: ::USHORT = PARAMFLAG_FIN;
pub const IDLFLAG_FOUT: ::USHORT = PARAMFLAG_FOUT;
pub const IDLFLAG_FLCID: ::USHORT = PARAMFLAG_FLCID;
pub const IDLFLAG_FRETVAL: ::USHORT = PARAMFLAG_FRETVAL;

#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct ELEMDESC {
    pub tdesc: TYPEDESC,
    pub DUMMYUNIONNAME: [u8; 6],
}
UNION!(ELEMDESC, DUMMYUNIONNAME, idldesc, idldesc_mut, *mut ::IDLDESC);
UNION!(ELEMDESC, DUMMYUNIONNAME, paramdesc, paramdesc_mut, *mut ::PARAMDESC);

pub type LPELEMDESC = *mut ELEMDESC;

#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct TYPEATTR {
    pub guid: ::GUID,
    pub lcid: ::LCID,
    pub dwReserved: ::DWORD,
    pub memidConstructor: ::MEMBERID,
    pub memidDestructor: ::MEMBERID,
    pub lpstrSchema: ::LPOLESTR,
    pub cbSizeInstance: ::ULONG,
    pub typekind: ::TYPEKIND,
    pub cFuncs: ::WORD,
    pub cVars: ::WORD,
    pub cImplTypes: ::WORD,
    pub cbSizeVft: ::WORD,
    pub cbAlignment: ::WORD,
    pub wTypeFlags: ::WORD,
    pub wMajorVerNum: ::WORD,
    pub wMinorVerNum: ::WORD,
    pub tdescAlias: ::TYPEDESC,
    pub idldescType: ::IDLDESC,
}
pub type LPTYPEATTR = *mut TYPEATTR;

#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct DISPPARAMS {
    pub rgvarg: *mut ::VARIANTARG,
    pub rgdispidNamedArgs: *mut ::DISPID,
    pub cArgs: ::UINT,
    pub cNamedArgs: ::UINT,
}

#[repr(C)]
pub struct EXCEPINFO {
    pub wCode: ::WORD,
    pub wReserved: ::WORD,
    pub bstrSource: ::BSTR,
    pub bstrDescription: ::BSTR,
    pub bstrHelpFile: ::BSTR,
    pub dwHelpContext: ::DWORD,
    pub pvReserved: ::PVOID,
    pub pfnDeferredFillIn: Option<unsafe extern "stdcall" fn(*mut EXCEPINFO) -> ::HRESULT>,
    pub scode: ::SCODE,
}
pub type LPEXCEPINFO = *mut EXCEPINFO;

#[repr(i32)] #[derive(Clone, Copy, Debug)]
pub enum CALLCONV {
    CC_FASTCALL = 0,
    CC_CDECL = 1,
    CC_MSCPASCAL = 2,
    // CC_PASCAL = 2,
    CC_MACPASCAL = 3,
    CC_STDCALL = 4,
    CC_FPFASTCALL = 5,
    CC_SYSCALL = 6,
    CC_MPWCDECL = 7,
    CC_MPWPASCAL = 8,
    CC_MAX = 9,
}

#[repr(i32)] #[derive(Clone, Copy, Debug)]
pub enum FUNCKIND {
    FUNC_VIRTUAL = 0,
    FUNC_PUREVIRTUAL = 1,
    FUNC_NONVIRTUAL = 2,
    FUNC_STATIC = 3,
    FUNC_DISPATCH = 4,
}

#[repr(i32)] #[derive(Clone, Copy, Debug)]
pub enum INVOKEKIND {
    INVOKE_FUNC = 1,
    INVOKE_PROPERTYGET = 2,
    INVOKE_PROPERTYPUT = 4,
    INVOKE_PROPERTYPUTREF = 8,
}

#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct FUNCDESC {
    pub memid: ::MEMBERID,
    pub lprgscode: *mut ::SCODE,
    pub lprgelemdescParam: *mut ::ELEMDESC,
    pub funckind: ::FUNCKIND,
    pub invkind: ::INVOKEKIND,
    pub callconv: ::CALLCONV,
    pub cParams: ::SHORT,
    pub cParamsOpt: ::SHORT,
    pub oVft: ::SHORT,
    pub cScodes: ::SHORT,
    pub elemdescFunc: ::ELEMDESC,
    pub wFuncFlags: ::WORD,
}
pub type LPFUNCDESC = *mut FUNCDESC;

#[repr(i32)] #[derive(Clone, Copy, Debug)]
pub enum VARKIND {
        VAR_PERINSTANCE = 0,
        VAR_STATIC = 1,
        VAR_CONST = 2,
        VAR_DISPATCH = 3,
}

pub const IMPLTYPEFLAG_FDEFAULT: ::USHORT = 0x1;
pub const IMPLTYPEFLAG_FSOURCE: ::USHORT = 0x2;
pub const IMPLTYPEFLAG_FRESTRICTED: ::USHORT = 0x4;
pub const IMPLTYPEFLAG_FDEFAULTVTABLE: ::USHORT = 0x8;

#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct VARDESC {
    pub memid: ::MEMBERID,
    pub lpstrSchema: ::LPOLESTR,
    pub oInst: ::ULONG,
    pub elemdescVar: ::ELEMDESC,
    pub wVarFlags: ::WORD,
    pub varkind: ::VARKIND,
}
UNION!(VARDESC, oInst, lpvarValue, lpvarValue_mut, *mut ::VARIANT);
pub type LPVARDESC = *mut VARDESC;

#[repr(i32)] #[derive(Clone, Copy, Debug)]
pub enum TYPEFLAGS {
    TYPEFLAG_FAPPOBJECT = 0x1,
    TYPEFLAG_FCANCREATE = 0x2,
    TYPEFLAG_FLICENSED  = 0x4,
    TYPEFLAG_FPREDECLID = 0x8,
    TYPEFLAG_FHIDDEN    = 0x10,
    TYPEFLAG_FCONTROL   = 0x20,
    TYPEFLAG_FDUAL  = 0x40,
    TYPEFLAG_FNONEXTENSIBLE = 0x80,
    TYPEFLAG_FOLEAUTOMATION = 0x100,
    TYPEFLAG_FRESTRICTED    = 0x200,
    TYPEFLAG_FAGGREGATABLE  = 0x400,
    TYPEFLAG_FREPLACEABLE   = 0x800,
    TYPEFLAG_FDISPATCHABLE  = 0x1000,
    TYPEFLAG_FREVERSEBIND   = 0x2000,
    TYPEFLAG_FPROXY = 0x4000,
}

#[repr(i32)] #[derive(Clone, Copy, Debug)]
pub enum FUNCFLAGS {
    FUNCFLAG_FRESTRICTED    = 0x1,
    FUNCFLAG_FSOURCE    = 0x2,
    FUNCFLAG_FBINDABLE  = 0x4,
    FUNCFLAG_FREQUESTEDIT   = 0x8,
    FUNCFLAG_FDISPLAYBIND   = 0x10,
    FUNCFLAG_FDEFAULTBIND   = 0x20,
    FUNCFLAG_FHIDDEN    = 0x40,
    FUNCFLAG_FUSESGETLASTERROR  = 0x80,
    FUNCFLAG_FDEFAULTCOLLELEM   = 0x100,
    FUNCFLAG_FUIDEFAULT = 0x200,
    FUNCFLAG_FNONBROWSABLE  = 0x400,
    FUNCFLAG_FREPLACEABLE   = 0x800,
    FUNCFLAG_FIMMEDIATEBIND = 0x1000
}

#[repr(i32)] #[derive(Clone, Copy, Debug)]
pub enum VARFLAGS {
    VARFLAG_FREADONLY   = 0x1,
    VARFLAG_FSOURCE = 0x2,
    VARFLAG_FBINDABLE   = 0x4,
    VARFLAG_FREQUESTEDIT    = 0x8,
    VARFLAG_FDISPLAYBIND    = 0x10,
    VARFLAG_FDEFAULTBIND    = 0x20,
    VARFLAG_FHIDDEN = 0x40,
    VARFLAG_FRESTRICTED = 0x80,
    VARFLAG_FDEFAULTCOLLELEM    = 0x100,
    VARFLAG_FUIDEFAULT  = 0x200,
    VARFLAG_FNONBROWSABLE   = 0x400,
    VARFLAG_FREPLACEABLE    = 0x800,
    VARFLAG_FIMMEDIATEBIND  = 0x1000
}

#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct CLEANLOCALSTORAGE {
    pub pInterface: *mut ::IUnknown,
    pub pStorage: ::PVOID,
    pub flags: ::DWORD,
}

#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct CUSTDATAITEM {
    pub guid: ::GUID,
    pub varValue: ::VARIANTARG,
}
pub type LPCUSTDATAITEM = *mut CUSTDATAITEM;

#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct CUSTDATA {
    pub cCustData: ::DWORD,
    pub prgCustData: ::LPCUSTDATAITEM,
}
pub type LPCUSTDATA = *mut CUSTDATA;

//2149
pub const DISPID_UNKNOWN: DISPID = -1;
pub const DISPID_VALUE: DISPID = 0;
pub const DISPID_PROPERTYPUT: DISPID = -3;
pub const DISPID_NEWENUM: DISPID = -4;
pub const DISPID_EVALUATE: DISPID = -5;
pub const DISPID_CONSTRUCTOR: DISPID = -6;
pub const DISPID_DESTRUCTOR: DISPID = -7;
pub const DISPID_COLLECT: DISPID = -8;

RIDL!(
interface IDispatch(IDispatchVtbl): IUnknown(IUnknownVtbl) {
    fn GetTypeInfoCount(&mut self, pctinfo: *mut ::UINT) -> ::HRESULT,
    fn GetTypeInfo(
        &mut self, iTInfo: ::UINT, lcid: ::LCID, ppTInfo: *mut *mut ::ITypeInfo
    ) -> ::HRESULT,
    fn GetIDsOfNames(
        &mut self, riid: ::REFIID, rgszNames: *mut ::LPOLESTR, cNames: ::UINT, lcid: ::LCID,
        rgDispId: *mut ::DISPID
    ) -> ::HRESULT,
    fn Invoke(
        &mut self, dispIdMember: ::DISPID, riid: ::REFIID, lcid: ::LCID, wFlags: ::WORD,
        pDispParams: *mut ::DISPPARAMS, pVarResult: *mut ::VARIANT, pExcepInfo: *mut ::EXCEPINFO,
        puArgErr: *mut ::UINT
    ) -> ::HRESULT
}
);
pub type LPDISPATCH = *mut IDispatch;

RIDL!(
interface IEnumVARIANT(IEnumVARIANTVtbl): IUnknown(IUnknownVtbl) {
    fn Next(
        &mut self, celt: ::ULONG, rgVar: *mut ::VARIANT, pCeltFetched: *mut ::ULONG
    ) -> ::HRESULT,
    fn Skip(&mut self, celt: ::ULONG) -> ::HRESULT,
    fn Reset(&mut self) -> ::HRESULT,
    fn Clone(&mut self, ppEnum: *mut *mut ::IEnumVARIANT) -> ::HRESULT
}
);
pub type LPENUMVARIANT = *mut IEnumVARIANT;

//2483
#[repr(i32)] #[derive(Clone, Copy, Debug)]
pub enum DESCKIND {
    DESCKIND_NONE = 0,
    DESCKIND_FUNCDESC = 1,
    DESCKIND_VARDESC = 2,
    DESCKIND_TYPECOMP = 3,
    DESCKIND_IMPLICITAPPOBJ = 4,
    DESCKIND_MAX = 5,
}

#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct BINDPTR {
    lpfuncdesc: *mut FUNCDESC,
}
UNION!(BINDPTR, lpfuncdesc, lpvardesc, lpvardesc_mut, *mut ::VARDESC);
UNION!(BINDPTR, lpfuncdesc, lptcomp, lptcomp_mut, *mut ::ITypeComp);
pub type LPBINDPTR = *mut BINDPTR;

RIDL!(
interface ITypeComp(ITypeCompVtbl): IUnknown(IUnknownVtbl) {
    fn Bind(
        &mut self, szName: ::LPOLESTR, lHashVal: ::ULONG, wFlags: ::WORD,
        ppTInfo: *mut *mut ::ITypeInfo, pDescKind: *mut ::DESCKIND, pBindPtr: *mut ::BINDPTR
    ) -> ::HRESULT,
    fn BindType(
        &mut self, szName: ::LPOLESTR, lHashVal: ::ULONG, ppTInfo: *mut *mut ::ITypeInfo,
        ppTComp: *mut *mut ::ITypeComp
    ) -> ::HRESULT
}
);
pub type LPTYPECOMP = *mut ITypeComp;

RIDL!(
interface ITypeInfo(ITypeInfoVtbl): IUnknown(IUnknownVtbl) {
    fn GetTypeAttr(&mut self, ppTypeAttr: *mut *mut ::TYPEATTR) -> ::HRESULT,
    fn GetTypeComp(&mut self, ppTComp: *mut *mut ::ITypeComp) -> ::HRESULT,
    fn GetFuncDesc(&mut self, index: ::UINT, ppFuncDesc: *mut *mut ::FUNCDESC) -> ::HRESULT,
    fn GetVarDesc(&mut self, index: ::UINT, ppVarDesc: *mut *mut ::VARDESC) -> ::HRESULT,
    fn GetNames(
        &mut self, memid: ::MEMBERID, rgBstrNames: *mut ::BSTR, cMaxNames: ::UINT,
        pcNames: *mut ::UINT
    ) -> ::HRESULT,
    fn GetRefTypeOfImplType(&mut self, index: ::UINT, pRefType: *mut ::HREFTYPE) -> ::HRESULT,
    fn GetImplTypeFlags(&mut self, index: ::UINT, pImplTypeFlags: *mut ::INT) -> ::HRESULT,
    fn GetIDsOfNames(
        &mut self, rgszNames: *mut ::LPOLESTR, cNames: ::UINT, pMemId: *mut ::MEMBERID
    ) -> ::HRESULT,
    fn Invoke(
        &mut self, pvInstance: ::PVOID, memid: ::MEMBERID, wFlags: ::WORD,
        pDispParams: *mut ::DISPPARAMS, pVarResult: *mut ::VARIANT, pExcepInfo: *mut ::EXCEPINFO,
        puArgErr: *mut ::UINT
    ) -> ::HRESULT,
    fn GetDocumentation(
        &mut self, memid: ::MEMBERID, pBstrName: *mut ::BSTR, pBstrDocString: *mut ::BSTR,
        pdwHelpContext: *mut ::DWORD, pBstrHelpFile: *mut ::BSTR
    ) -> ::HRESULT,
    fn GetDllEntry(
        &mut self, memid: ::MEMBERID, invKind: ::INVOKEKIND, pBstrDllName: *mut ::BSTR,
        pBstrName: *mut ::BSTR, pwOrdinal: *mut ::WORD
    ) -> ::HRESULT,
    fn GetRefTypeInfo(
        &mut self, hRefType: ::HREFTYPE, ppTInfo: *mut *mut ::ITypeInfo
    ) -> ::HRESULT,
    fn AddressOfMember(
        &mut self, memid: ::MEMBERID, invKind: ::INVOKEKIND, ppv: *mut ::PVOID
    ) -> ::HRESULT,
    fn CreateInstance(
        &mut self, pUnkOuter: *mut ::IUnknown, riid: ::REFIID, ppvObj: *mut ::PVOID
    ) -> ::HRESULT,
    fn GetMops(&mut self, memid: ::MEMBERID, pBstrMops: *mut ::BSTR) -> ::HRESULT,
    fn GetContainingTypeLib(
        &mut self, ppTLib: *mut *mut ::ITypeLib, pIndex: *mut ::UINT
    ) -> ::HRESULT,
    fn ReleaseTypeAttr(&mut self, pTypeAttr: *mut ::TYPEATTR) -> ::c_void,
    fn ReleaseFuncDesc(&mut self, pFuncDesc: *mut ::FUNCDESC) -> ::c_void,
    fn ReleaseVarDesc(&mut self, pVarDesc: *mut ::VARDESC) -> ::c_void
}
);
pub type LPTYPEINFO = *mut ITypeInfo;

//3624
#[repr(i32)] #[derive(Clone, Copy, Debug)]
pub enum SYSKIND {
    SYS_WIN16 = 0,
    SYS_WIN32 = 1,
    SYS_MAC = 2,
    SYS_WIN64 = 3
}

#[repr(i32)] #[derive(Clone, Copy, Debug)]
pub enum LIBFLAGS {
    LIBFLAG_FRESTRICTED = 0x1,
    LIBFLAG_FCONTROL = 0x2,
    LIBFLAG_FHIDDEN = 0x4,
    LIBFLAG_FHASDISKIMAGE = 0x8,
}

#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct TLIBATTR {
    pub guid: ::GUID,
    pub lcid: ::LCID,
    pub syskind: ::SYSKIND,
    pub wMajorVerNum: ::WORD,
    pub wMinorVerNum: ::WORD,
    pub wLibFlags: ::WORD,
}
pub type LPTLIBATTR = *mut TLIBATTR;

RIDL!(
interface ITypeLib(ITypeLibVtbl): IUnknown(IUnknownVtbl) {
    fn GetTypeInfoCount(&mut self) -> ::UINT,
    fn GetTypeInfo(&mut self, index: ::UINT, ppTInfo: *mut *mut ::ITypeInfo) -> ::HRESULT,
    fn GetTypeInfoType(&mut self, index: ::UINT, pTKind: *mut ::TYPEKIND) -> ::HRESULT,
    fn GetTypeInfoOfGuid(&mut self, guid: ::REFGUID, ppTinfo: *mut *mut ::ITypeInfo) -> ::HRESULT,
    fn GetLibAttr(&mut self, ppTLibAttr: *mut *mut ::TLIBATTR) -> ::HRESULT,
    fn GetTypeComp(&mut self, ppTComp: *mut *mut ::ITypeComp) -> ::HRESULT,
    fn GetDocumentation(
        &mut self, index: ::INT, pBstrName: *mut ::BSTR, pBstrDocString: *mut ::BSTR,
        pdwHelpContext: *mut ::DWORD, pBstrHelpFile: *mut ::BSTR
    ) -> ::HRESULT,
    fn IsName(
        &mut self, szNameBuf: ::LPOLESTR, lHashVal: ::ULONG, pfName: *mut ::BOOL
    ) -> ::HRESULT,
    fn FindName(
        &mut self, szNameBuf: ::LPOLESTR, lHashVal: ::ULONG, ppTInfo: *mut *mut ::ITypeInfo,
        rgMemId: *mut ::MEMBERID, pcFound: *mut ::USHORT
    ) -> ::HRESULT,
    fn ReleaseTLibAttr(&mut self, pTLibAttr: *mut ::TLIBATTR) -> ::c_void
}
);
pub type LPTYPELIB = *mut ITypeLib;
