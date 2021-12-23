#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[cfg(feature = "Win32_System_Com_CallObj")]
pub mod CallObj;
#[cfg(feature = "Win32_System_Com_ChannelCredentials")]
pub mod ChannelCredentials;
#[cfg(feature = "Win32_System_Com_Events")]
pub mod Events;
#[cfg(feature = "Win32_System_Com_Marshal")]
pub mod Marshal;
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
pub mod StructuredStorage;
#[cfg(feature = "Win32_System_Com_UI")]
pub mod UI;
#[cfg(feature = "Win32_System_Com_Urlmon")]
pub mod Urlmon;
#[doc = "*Required features: 'Win32_System_Com'*"]
pub type ADVF = i32;
#[doc = "*Required features: 'Win32_System_Com'*"]
pub const ADVF_NODATA: ADVF = 1i32;
#[doc = "*Required features: 'Win32_System_Com'*"]
pub const ADVF_PRIMEFIRST: ADVF = 2i32;
#[doc = "*Required features: 'Win32_System_Com'*"]
pub const ADVF_ONLYONCE: ADVF = 4i32;
#[doc = "*Required features: 'Win32_System_Com'*"]
pub const ADVF_DATAONSTOP: ADVF = 64i32;
#[doc = "*Required features: 'Win32_System_Com'*"]
pub const ADVFCACHE_NOHANDLER: ADVF = 8i32;
#[doc = "*Required features: 'Win32_System_Com'*"]
pub const ADVFCACHE_FORCEBUILTIN: ADVF = 16i32;
#[doc = "*Required features: 'Win32_System_Com'*"]
pub const ADVFCACHE_ONSAVE: ADVF = 32i32;
#[doc = "*Required features: 'Win32_System_Com'*"]
pub const APPIDREGFLAGS_AAA_NO_IMPLICIT_ACTIVATE_AS_IU: u32 = 2048u32;
#[doc = "*Required features: 'Win32_System_Com'*"]
pub const APPIDREGFLAGS_ACTIVATE_IUSERVER_INDESKTOP: u32 = 1u32;
#[doc = "*Required features: 'Win32_System_Com'*"]
pub const APPIDREGFLAGS_ISSUE_ACTIVATION_RPC_AT_IDENTIFY: u32 = 4u32;
#[doc = "*Required features: 'Win32_System_Com'*"]
pub const APPIDREGFLAGS_IUSERVER_ACTIVATE_IN_CLIENT_SESSION_ONLY: u32 = 32u32;
#[doc = "*Required features: 'Win32_System_Com'*"]
pub const APPIDREGFLAGS_IUSERVER_SELF_SID_IN_LAUNCH_PERMISSION: u32 = 16u32;
#[doc = "*Required features: 'Win32_System_Com'*"]
pub const APPIDREGFLAGS_IUSERVER_UNMODIFIED_LOGON_TOKEN: u32 = 8u32;
#[doc = "*Required features: 'Win32_System_Com'*"]
pub const APPIDREGFLAGS_RESERVED1: u32 = 64u32;
#[doc = "*Required features: 'Win32_System_Com'*"]
pub const APPIDREGFLAGS_RESERVED2: u32 = 128u32;
#[doc = "*Required features: 'Win32_System_Com'*"]
pub const APPIDREGFLAGS_RESERVED3: u32 = 256u32;
#[doc = "*Required features: 'Win32_System_Com'*"]
pub const APPIDREGFLAGS_RESERVED4: u32 = 512u32;
#[doc = "*Required features: 'Win32_System_Com'*"]
pub const APPIDREGFLAGS_RESERVED5: u32 = 1024u32;
#[doc = "*Required features: 'Win32_System_Com'*"]
pub const APPIDREGFLAGS_RESERVED7: u32 = 4096u32;
#[doc = "*Required features: 'Win32_System_Com'*"]
pub const APPIDREGFLAGS_RESERVED8: u32 = 8192u32;
#[doc = "*Required features: 'Win32_System_Com'*"]
pub const APPIDREGFLAGS_RESERVED9: u32 = 16384u32;
#[doc = "*Required features: 'Win32_System_Com'*"]
pub const APPIDREGFLAGS_SECURE_SERVER_PROCESS_SD_AND_BIND: u32 = 2u32;
#[doc = "*Required features: 'Win32_System_Com'*"]
pub type APTTYPE = i32;
#[doc = "*Required features: 'Win32_System_Com'*"]
pub const APTTYPE_CURRENT: APTTYPE = -1i32;
#[doc = "*Required features: 'Win32_System_Com'*"]
pub const APTTYPE_STA: APTTYPE = 0i32;
#[doc = "*Required features: 'Win32_System_Com'*"]
pub const APTTYPE_MTA: APTTYPE = 1i32;
#[doc = "*Required features: 'Win32_System_Com'*"]
pub const APTTYPE_NA: APTTYPE = 2i32;
#[doc = "*Required features: 'Win32_System_Com'*"]
pub const APTTYPE_MAINSTA: APTTYPE = 3i32;
#[doc = "*Required features: 'Win32_System_Com'*"]
pub type APTTYPEQUALIFIER = i32;
#[doc = "*Required features: 'Win32_System_Com'*"]
pub const APTTYPEQUALIFIER_NONE: APTTYPEQUALIFIER = 0i32;
#[doc = "*Required features: 'Win32_System_Com'*"]
pub const APTTYPEQUALIFIER_IMPLICIT_MTA: APTTYPEQUALIFIER = 1i32;
#[doc = "*Required features: 'Win32_System_Com'*"]
pub const APTTYPEQUALIFIER_NA_ON_MTA: APTTYPEQUALIFIER = 2i32;
#[doc = "*Required features: 'Win32_System_Com'*"]
pub const APTTYPEQUALIFIER_NA_ON_STA: APTTYPEQUALIFIER = 3i32;
#[doc = "*Required features: 'Win32_System_Com'*"]
pub const APTTYPEQUALIFIER_NA_ON_IMPLICIT_MTA: APTTYPEQUALIFIER = 4i32;
#[doc = "*Required features: 'Win32_System_Com'*"]
pub const APTTYPEQUALIFIER_NA_ON_MAINSTA: APTTYPEQUALIFIER = 5i32;
#[doc = "*Required features: 'Win32_System_Com'*"]
pub const APTTYPEQUALIFIER_APPLICATION_STA: APTTYPEQUALIFIER = 6i32;
#[doc = "*Required features: 'Win32_System_Com'*"]
pub const APTTYPEQUALIFIER_RESERVED_1: APTTYPEQUALIFIER = 7i32;
#[doc = "*Required features: 'Win32_System_Com'*"]
pub const ASYNC_MODE_COMPATIBILITY: i32 = 1i32;
#[doc = "*Required features: 'Win32_System_Com'*"]
pub const ASYNC_MODE_DEFAULT: i32 = 0i32;
#[repr(C)]
#[doc = "*Required features: 'Win32_System_Com'*"]
pub struct AUTHENTICATEINFO {
    pub dwFlags: u32,
    pub dwReserved: u32,
}
impl ::core::marker::Copy for AUTHENTICATEINFO {}
impl ::core::clone::Clone for AUTHENTICATEINFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for AUTHENTICATEINFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("AUTHENTICATEINFO").field("dwFlags", &self.dwFlags).field("dwReserved", &self.dwReserved).finish()
    }
}
unsafe impl ::windows::core::Abi for AUTHENTICATEINFO {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for AUTHENTICATEINFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<AUTHENTICATEINFO>()) == 0 }
    }
}
impl ::core::cmp::Eq for AUTHENTICATEINFO {}
impl ::core::default::Default for AUTHENTICATEINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: 'Win32_System_Com'*"]
pub type ApplicationType = i32;
#[doc = "*Required features: 'Win32_System_Com'*"]
pub const ServerApplication: ApplicationType = 0i32;
#[doc = "*Required features: 'Win32_System_Com'*"]
pub const LibraryApplication: ApplicationType = 1i32;
#[doc = "*Required features: 'Win32_System_Com'*"]
#[repr(transparent)]
pub struct AsyncIAdviseSink(::windows::core::IUnknown);
impl AsyncIAdviseSink {
    #[doc = "*Required features: 'Win32_System_Com', 'Win32_Foundation', 'Win32_Graphics_Gdi', 'Win32_System_Com_StructuredStorage'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
    pub unsafe fn Begin_OnDataChange(&self, pformatetc: *const FORMATETC, pstgmed: *const STGMEDIUM) {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(pformatetc), ::core::mem::transmute(pstgmed))
    }
    #[doc = "*Required features: 'Win32_System_Com'*"]
    pub unsafe fn Finish_OnDataChange(&self) {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self))
    }
    #[doc = "*Required features: 'Win32_System_Com'*"]
    pub unsafe fn Begin_OnViewChange(&self, dwaspect: u32, lindex: i32) {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwaspect), ::core::mem::transmute(lindex))
    }
    #[doc = "*Required features: 'Win32_System_Com'*"]
    pub unsafe fn Finish_OnViewChange(&self) {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self))
    }
    #[doc = "*Required features: 'Win32_System_Com'*"]
    pub unsafe fn Begin_OnRename<'a, Param0: ::windows::core::IntoParam<'a, IMoniker>>(&self, pmk: Param0) {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), pmk.into_param().abi())
    }
    #[doc = "*Required features: 'Win32_System_Com'*"]
    pub unsafe fn Finish_OnRename(&self) {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self))
    }
    #[doc = "*Required features: 'Win32_System_Com'*"]
    pub unsafe fn Begin_OnSave(&self) {
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self))
    }
    #[doc = "*Required features: 'Win32_System_Com'*"]
    pub unsafe fn Finish_OnSave(&self) {
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self))
    }
    #[doc = "*Required features: 'Win32_System_Com'*"]
    pub unsafe fn Begin_OnClose(&self) {
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self))
    }
    #[doc = "*Required features: 'Win32_System_Com'*"]
    pub unsafe fn Finish_OnClose(&self) {
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self))
    }
}
impl ::core::convert::From<AsyncIAdviseSink> for ::windows::core::IUnknown {
    fn from(value: AsyncIAdviseSink) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AsyncIAdviseSink> for ::windows::core::IUnknown {
    fn from(value: &AsyncIAdviseSink) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for AsyncIAdviseSink {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &AsyncIAdviseSink {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for AsyncIAdviseSink {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AsyncIAdviseSink {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AsyncIAdviseSink {}
impl ::core::fmt::Debug for AsyncIAdviseSink {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AsyncIAdviseSink").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for AsyncIAdviseSink {
    type Vtable = AsyncIAdviseSinkVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x00000150_0000_0000_c000_000000000046);
}
#[repr(C)]
#[doc(hidden)]
pub struct AsyncIAdviseSinkVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pformatetc: *const FORMATETC, pstgmed: *const STGMEDIUM),
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage")))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void),
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwaspect: u32, lindex: i32),
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void),
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pmk: ::windows::core::RawPtr),
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void),
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void),
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void),
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void),
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void),
);
#[doc = "*Required features: 'Win32_System_Com'*"]
#[repr(transparent)]
pub struct AsyncIAdviseSink2(::windows::core::IUnknown);
impl AsyncIAdviseSink2 {
    #[doc = "*Required features: 'Win32_System_Com', 'Win32_Foundation', 'Win32_Graphics_Gdi', 'Win32_System_Com_StructuredStorage'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
    pub unsafe fn Begin_OnDataChange(&self, pformatetc: *const FORMATETC, pstgmed: *const STGMEDIUM) {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(pformatetc), ::core::mem::transmute(pstgmed))
    }
    #[doc = "*Required features: 'Win32_System_Com'*"]
    pub unsafe fn Finish_OnDataChange(&self) {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self))
    }
    #[doc = "*Required features: 'Win32_System_Com'*"]
    pub unsafe fn Begin_OnViewChange(&self, dwaspect: u32, lindex: i32) {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwaspect), ::core::mem::transmute(lindex))
    }
    #[doc = "*Required features: 'Win32_System_Com'*"]
    pub unsafe fn Finish_OnViewChange(&self) {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self))
    }
    #[doc = "*Required features: 'Win32_System_Com'*"]
    pub unsafe fn Begin_OnRename<'a, Param0: ::windows::core::IntoParam<'a, IMoniker>>(&self, pmk: Param0) {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), pmk.into_param().abi())
    }
    #[doc = "*Required features: 'Win32_System_Com'*"]
    pub unsafe fn Finish_OnRename(&self) {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self))
    }
    #[doc = "*Required features: 'Win32_System_Com'*"]
    pub unsafe fn Begin_OnSave(&self) {
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self))
    }
    #[doc = "*Required features: 'Win32_System_Com'*"]
    pub unsafe fn Finish_OnSave(&self) {
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self))
    }
    #[doc = "*Required features: 'Win32_System_Com'*"]
    pub unsafe fn Begin_OnClose(&self) {
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self))
    }
    #[doc = "*Required features: 'Win32_System_Com'*"]
    pub unsafe fn Finish_OnClose(&self) {
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self))
    }
    #[doc = "*Required features: 'Win32_System_Com'*"]
    pub unsafe fn Begin_OnLinkSrcChange<'a, Param0: ::windows::core::IntoParam<'a, IMoniker>>(&self, pmk: Param0) {
        (::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self), pmk.into_param().abi())
    }
    #[doc = "*Required features: 'Win32_System_Com'*"]
    pub unsafe fn Finish_OnLinkSrcChange(&self) {
        (::windows::core::Interface::vtable(self).14)(::core::mem::transmute_copy(self))
    }
}
impl ::core::convert::From<AsyncIAdviseSink2> for AsyncIAdviseSink {
    fn from(value: AsyncIAdviseSink2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AsyncIAdviseSink2> for AsyncIAdviseSink {
    fn from(value: &AsyncIAdviseSink2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, AsyncIAdviseSink> for AsyncIAdviseSink2 {
    fn into_param(self) -> ::windows::core::Param<'a, AsyncIAdviseSink> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, AsyncIAdviseSink> for &AsyncIAdviseSink2 {
    fn into_param(self) -> ::windows::core::Param<'a, AsyncIAdviseSink> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<AsyncIAdviseSink2> for ::windows::core::IUnknown {
    fn from(value: AsyncIAdviseSink2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AsyncIAdviseSink2> for ::windows::core::IUnknown {
    fn from(value: &AsyncIAdviseSink2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for AsyncIAdviseSink2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &AsyncIAdviseSink2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for AsyncIAdviseSink2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AsyncIAdviseSink2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AsyncIAdviseSink2 {}
impl ::core::fmt::Debug for AsyncIAdviseSink2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AsyncIAdviseSink2").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for AsyncIAdviseSink2 {
    type Vtable = AsyncIAdviseSink2Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x00000151_0000_0000_c000_000000000046);
}
#[repr(C)]
#[doc(hidden)]
pub struct AsyncIAdviseSink2Vtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pformatetc: *const FORMATETC, pstgmed: *const STGMEDIUM),
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage")))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void),
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwaspect: u32, lindex: i32),
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void),
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pmk: ::windows::core::RawPtr),
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void),
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void),
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void),
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void),
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void),
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pmk: ::windows::core::RawPtr),
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void),
);
#[doc = "*Required features: 'Win32_System_Com'*"]
#[repr(transparent)]
pub struct AsyncIMultiQI(::windows::core::IUnknown);
impl AsyncIMultiQI {
    #[doc = "*Required features: 'Win32_System_Com'*"]
    pub unsafe fn Begin_QueryMultipleInterfaces(&self, cmqis: u32, pmqis: *mut MULTI_QI) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(cmqis), ::core::mem::transmute(pmqis)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Com'*"]
    pub unsafe fn Finish_QueryMultipleInterfaces(&self, pmqis: *mut MULTI_QI) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(pmqis)).ok()
    }
}
impl ::core::convert::From<AsyncIMultiQI> for ::windows::core::IUnknown {
    fn from(value: AsyncIMultiQI) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AsyncIMultiQI> for ::windows::core::IUnknown {
    fn from(value: &AsyncIMultiQI) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for AsyncIMultiQI {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &AsyncIMultiQI {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for AsyncIMultiQI {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AsyncIMultiQI {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AsyncIMultiQI {}
impl ::core::fmt::Debug for AsyncIMultiQI {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AsyncIMultiQI").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for AsyncIMultiQI {
    type Vtable = AsyncIMultiQIVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x000e0020_0000_0000_c000_000000000046);
}
#[repr(C)]
#[doc(hidden)]
pub struct AsyncIMultiQIVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cmqis: u32, pmqis: *mut MULTI_QI) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pmqis: *mut MULTI_QI) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: 'Win32_System_Com'*"]
#[repr(transparent)]
pub struct AsyncIPipeByte(::windows::core::IUnknown);
impl AsyncIPipeByte {
    #[doc = "*Required features: 'Win32_System_Com'*"]
    pub unsafe fn Begin_Pull(&self, crequest: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(crequest)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Com'*"]
    pub unsafe fn Finish_Pull(&self, buf: *mut u8, pcreturned: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(buf), ::core::mem::transmute(pcreturned)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Com'*"]
    pub unsafe fn Begin_Push(&self, buf: *const u8, csent: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(buf), ::core::mem::transmute(csent)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Com'*"]
    pub unsafe fn Finish_Push(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self)).ok()
    }
}
impl ::core::convert::From<AsyncIPipeByte> for ::windows::core::IUnknown {
    fn from(value: AsyncIPipeByte) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AsyncIPipeByte> for ::windows::core::IUnknown {
    fn from(value: &AsyncIPipeByte) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for AsyncIPipeByte {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &AsyncIPipeByte {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for AsyncIPipeByte {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AsyncIPipeByte {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AsyncIPipeByte {}
impl ::core::fmt::Debug for AsyncIPipeByte {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AsyncIPipeByte").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for AsyncIPipeByte {
    type Vtable = AsyncIPipeByteVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdb2f3acb_2f86_11d1_8e04_00c04fb9989a);
}
#[repr(C)]
#[doc(hidden)]
pub struct AsyncIPipeByteVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, crequest: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, buf: *mut u8, pcreturned: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, buf: *const u8, csent: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: 'Win32_System_Com'*"]
#[repr(transparent)]
pub struct AsyncIPipeDouble(::windows::core::IUnknown);
impl AsyncIPipeDouble {
    #[doc = "*Required features: 'Win32_System_Com'*"]
    pub unsafe fn Begin_Pull(&self, crequest: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(crequest)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Com'*"]
    pub unsafe fn Finish_Pull(&self, buf: *mut f64, pcreturned: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(buf), ::core::mem::transmute(pcreturned)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Com'*"]
    pub unsafe fn Begin_Push(&self, buf: *const f64, csent: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(buf), ::core::mem::transmute(csent)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Com'*"]
    pub unsafe fn Finish_Push(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self)).ok()
    }
}
impl ::core::convert::From<AsyncIPipeDouble> for ::windows::core::IUnknown {
    fn from(value: AsyncIPipeDouble) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AsyncIPipeDouble> for ::windows::core::IUnknown {
    fn from(value: &AsyncIPipeDouble) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for AsyncIPipeDouble {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &AsyncIPipeDouble {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for AsyncIPipeDouble {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AsyncIPipeDouble {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AsyncIPipeDouble {}
impl ::core::fmt::Debug for AsyncIPipeDouble {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AsyncIPipeDouble").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for AsyncIPipeDouble {
    type Vtable = AsyncIPipeDoubleVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdb2f3acf_2f86_11d1_8e04_00c04fb9989a);
}
#[repr(C)]
#[doc(hidden)]
pub struct AsyncIPipeDoubleVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, crequest: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, buf: *mut f64, pcreturned: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, buf: *const f64, csent: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: 'Win32_System_Com'*"]
#[repr(transparent)]
pub struct AsyncIPipeLong(::windows::core::IUnknown);
impl AsyncIPipeLong {
    #[doc = "*Required features: 'Win32_System_Com'*"]
    pub unsafe fn Begin_Pull(&self, crequest: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(crequest)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Com'*"]
    pub unsafe fn Finish_Pull(&self, buf: *mut i32, pcreturned: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(buf), ::core::mem::transmute(pcreturned)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Com'*"]
    pub unsafe fn Begin_Push(&self, buf: *const i32, csent: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(buf), ::core::mem::transmute(csent)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Com'*"]
    pub unsafe fn Finish_Push(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self)).ok()
    }
}
impl ::core::convert::From<AsyncIPipeLong> for ::windows::core::IUnknown {
    fn from(value: AsyncIPipeLong) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AsyncIPipeLong> for ::windows::core::IUnknown {
    fn from(value: &AsyncIPipeLong) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for AsyncIPipeLong {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &AsyncIPipeLong {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for AsyncIPipeLong {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AsyncIPipeLong {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AsyncIPipeLong {}
impl ::core::fmt::Debug for AsyncIPipeLong {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AsyncIPipeLong").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for AsyncIPipeLong {
    type Vtable = AsyncIPipeLongVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdb2f3acd_2f86_11d1_8e04_00c04fb9989a);
}
#[repr(C)]
#[doc(hidden)]
pub struct AsyncIPipeLongVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, crequest: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, buf: *mut i32, pcreturned: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, buf: *const i32, csent: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: 'Win32_System_Com'*"]
#[repr(transparent)]
pub struct AsyncIUnknown(::windows::core::IUnknown);
impl AsyncIUnknown {
    #[doc = "*Required features: 'Win32_System_Com'*"]
    pub unsafe fn Begin_QueryInterface(&self, riid: *const ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(riid)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Com'*"]
    pub unsafe fn Finish_QueryInterface(&self, ppvobject: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(ppvobject)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Com'*"]
    pub unsafe fn Begin_AddRef(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Com'*"]
    pub unsafe fn Finish_AddRef(&self) -> u32 {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self)))
    }
    #[doc = "*Required features: 'Win32_System_Com'*"]
    pub unsafe fn Begin_Release(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Com'*"]
    pub unsafe fn Finish_Release(&self) -> u32 {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self)))
    }
}
impl ::core::convert::From<AsyncIUnknown> for ::windows::core::IUnknown {
    fn from(value: AsyncIUnknown) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AsyncIUnknown> for ::windows::core::IUnknown {
    fn from(value: &AsyncIUnknown) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for AsyncIUnknown {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &AsyncIUnknown {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for AsyncIUnknown {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AsyncIUnknown {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AsyncIUnknown {}
impl ::core::fmt::Debug for AsyncIUnknown {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AsyncIUnknown").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for AsyncIUnknown {
    type Vtable = AsyncIUnknownVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x000e0000_0000_0000_c000_000000000046);
}
#[repr(C)]
#[doc(hidden)]
pub struct AsyncIUnknownVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppvobject: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
);
#[repr(C)]
#[doc = "*Required features: 'Win32_System_Com', 'Win32_Foundation', 'Win32_Graphics_Gdi', 'Win32_Security', 'Win32_System_Com_StructuredStorage'*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_Security", feature = "Win32_System_Com_StructuredStorage"))]
pub struct BINDINFO {
    pub cbSize: u32,
    pub szExtraInfo: super::super::Foundation::PWSTR,
    pub stgmedData: STGMEDIUM,
    pub grfBindInfoF: u32,
    pub dwBindVerb: u32,
    pub szCustomVerb: super::super::Foundation::PWSTR,
    pub cbstgmedData: u32,
    pub dwOptions: u32,
    pub dwOptionsFlags: u32,
    pub dwCodePage: u32,
    pub securityAttributes: super::super::Security::SECURITY_ATTRIBUTES,
    pub iid: ::windows::core::GUID,
    pub pUnk: ::core::option::Option<::windows::core::IUnknown>,
    pub dwReserved: u32,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_Security", feature = "Win32_System_Com_StructuredStorage"))]
impl ::core::clone::Clone for BINDINFO {
    fn clone(&self) -> Self {
        Self {
            cbSize: self.cbSize,
            szExtraInfo: self.szExtraInfo,
            stgmedData: self.stgmedData.clone(),
            grfBindInfoF: self.grfBindInfoF,
            dwBindVerb: self.dwBindVerb,
            szCustomVerb: self.szCustomVerb,
            cbstgmedData: self.cbstgmedData,
            dwOptions: self.dwOptions,
            dwOptionsFlags: self.dwOptionsFlags,
            dwCodePage: self.dwCodePage,
            securityAttributes: self.securityAttributes,
            iid: self.iid,
            pUnk: self.pUnk.clone(),
            dwReserved: self.dwReserved,
        }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_Security", feature = "Win32_System_Com_StructuredStorage"))]
unsafe impl ::windows::core::Abi for BINDINFO {
    type Abi = ::core::mem::ManuallyDrop<Self>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_Security", feature = "Win32_System_Com_StructuredStorage"))]
impl ::core::cmp::PartialEq for BINDINFO {
    fn eq(&self, other: &Self) -> bool {
        self.cbSize == other.cbSize && self.szExtraInfo == other.szExtraInfo && self.stgmedData == other.stgmedData && self.grfBindInfoF == other.grfBindInfoF && self.dwBindVerb == other.dwBindVerb && self.szCustomVerb == other.szCustomVerb && self.cbstgmedData == other.cbstgmedData && self.dwOptions == other.dwOptions && self.dwOptionsFlags == other.dwOptionsFlags && self.dwCodePage == other.dwCodePage && self.securityAttributes == other.securityAttributes && self.iid == other.iid && self.pUnk == other.pUnk && self.dwReserved == other.dwReserved
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_Security", feature = "Win32_System_Com_StructuredStorage"))]
impl ::core::cmp::Eq for BINDINFO {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_Security", feature = "Win32_System_Com_StructuredStorage"))]
impl ::core::default::Default for BINDINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: 'Win32_System_Com'*"]
pub type BINDINFOF = i32;
#[doc = "*Required features: 'Win32_System_Com'*"]
pub const BINDINFOF_URLENCODESTGMEDDATA: BINDINFOF = 1i32;
#[doc = "*Required features: 'Win32_System_Com'*"]
pub const BINDINFOF_URLENCODEDEXTRAINFO: BINDINFOF = 2i32;
#[repr(C)]
#[doc = "*Required features: 'Win32_System_Com', 'Win32_Foundation', 'Win32_System_Ole'*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
pub union BINDPTR {
    pub lpfuncdesc: *mut FUNCDESC,
    pub lpvardesc: *mut VARDESC,
    pub lptcomp: ::core::mem::ManuallyDrop<::core::option::Option<ITypeComp>>,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
impl ::core::clone::Clone for BINDPTR {
    fn clone(&self) -> Self {
        unsafe { ::core::mem::transmute_copy(self) }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
unsafe impl ::windows::core::Abi for BINDPTR {
    type Abi = ::core::mem::ManuallyDrop<Self>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
impl ::core::cmp::PartialEq for BINDPTR {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<BINDPTR>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
impl ::core::cmp::Eq for BINDPTR {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
impl ::core::default::Default for BINDPTR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: 'Win32_System_Com'*"]
pub type BIND_FLAGS = i32;
#[doc = "*Required features: 'Win32_System_Com'*"]
pub const BIND_MAYBOTHERUSER: BIND_FLAGS = 1i32;
#[doc = "*Required features: 'Win32_System_Com'*"]
pub const BIND_JUSTTESTEXISTENCE: BIND_FLAGS = 2i32;
#[repr(C)]
#[doc = "*Required features: 'Win32_System_Com'*"]
pub struct BIND_OPTS {
    pub cbStruct: u32,
    pub grfFlags: u32,
    pub grfMode: u32,
    pub dwTickCountDeadline: u32,
}
impl ::core::marker::Copy for BIND_OPTS {}
impl ::core::clone::Clone for BIND_OPTS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for BIND_OPTS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("BIND_OPTS").field("cbStruct", &self.cbStruct).field("grfFlags", &self.grfFlags).field("grfMode", &self.grfMode).field("dwTickCountDeadline", &self.dwTickCountDeadline).finish()
    }
}
unsafe impl ::windows::core::Abi for BIND_OPTS {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for BIND_OPTS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<BIND_OPTS>()) == 0 }
    }
}
impl ::core::cmp::Eq for BIND_OPTS {}
impl ::core::default::Default for BIND_OPTS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_System_Com', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct BIND_OPTS2 {
    pub __AnonymousBase_objidl_L9017_C36: BIND_OPTS,
    pub dwTrackFlags: u32,
    pub dwClassContext: u32,
    pub locale: u32,
    pub pServerInfo: *mut COSERVERINFO,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for BIND_OPTS2 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for BIND_OPTS2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for BIND_OPTS2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("BIND_OPTS2").field("__AnonymousBase_objidl_L9017_C36", &self.__AnonymousBase_objidl_L9017_C36).field("dwTrackFlags", &self.dwTrackFlags).field("dwClassContext", &self.dwClassContext).field("locale", &self.locale).field("pServerInfo", &self.pServerInfo).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for BIND_OPTS2 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for BIND_OPTS2 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<BIND_OPTS2>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for BIND_OPTS2 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for BIND_OPTS2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_System_Com', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct BIND_OPTS3 {
    pub __AnonymousBase_objidl_L9041_C36: BIND_OPTS2,
    pub hwnd: super::super::Foundation::HWND,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for BIND_OPTS3 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for BIND_OPTS3 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for BIND_OPTS3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("BIND_OPTS3").field("__AnonymousBase_objidl_L9041_C36", &self.__AnonymousBase_objidl_L9041_C36).field("hwnd", &self.hwnd).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for BIND_OPTS3 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for BIND_OPTS3 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<BIND_OPTS3>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for BIND_OPTS3 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for BIND_OPTS3 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_System_Com'*"]
pub struct BLOB {
    pub cbSize: u32,
    pub pBlobData: *mut u8,
}
impl ::core::marker::Copy for BLOB {}
impl ::core::clone::Clone for BLOB {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for BLOB {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("BLOB").field("cbSize", &self.cbSize).field("pBlobData", &self.pBlobData).finish()
    }
}
unsafe impl ::windows::core::Abi for BLOB {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for BLOB {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<BLOB>()) == 0 }
    }
}
impl ::core::cmp::Eq for BLOB {}
impl ::core::default::Default for BLOB {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_System_Com'*"]
pub struct BYTE_BLOB {
    pub clSize: u32,
    pub abData: [u8; 1],
}
impl ::core::marker::Copy for BYTE_BLOB {}
impl ::core::clone::Clone for BYTE_BLOB {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for BYTE_BLOB {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("BYTE_BLOB").field("clSize", &self.clSize).field("abData", &self.abData).finish()
    }
}
unsafe impl ::windows::core::Abi for BYTE_BLOB {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for BYTE_BLOB {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<BYTE_BLOB>()) == 0 }
    }
}
impl ::core::cmp::Eq for BYTE_BLOB {}
impl ::core::default::Default for BYTE_BLOB {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_System_Com'*"]
pub struct BYTE_SIZEDARR {
    pub clSize: u32,
    pub pData: *mut u8,
}
impl ::core::marker::Copy for BYTE_SIZEDARR {}
impl ::core::clone::Clone for BYTE_SIZEDARR {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for BYTE_SIZEDARR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("BYTE_SIZEDARR").field("clSize", &self.clSize).field("pData", &self.pData).finish()
    }
}
unsafe impl ::windows::core::Abi for BYTE_SIZEDARR {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for BYTE_SIZEDARR {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<BYTE_SIZEDARR>()) == 0 }
    }
}
impl ::core::cmp::Eq for BYTE_SIZEDARR {}
impl ::core::default::Default for BYTE_SIZEDARR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: 'Win32_System_Com'*"]
#[inline]
pub unsafe fn BindMoniker<'a, Param0: ::windows::core::IntoParam<'a, IMoniker>>(pmk: Param0, grfopt: u32, iidresult: *const ::windows::core::GUID, ppvresult: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn BindMoniker(pmk: ::windows::core::RawPtr, grfopt: u32, iidresult: *const ::windows::core::GUID, ppvresult: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT;
        }
        BindMoniker(pmk.into_param().abi(), ::core::mem::transmute(grfopt), ::core::mem::transmute(iidresult), ::core::mem::transmute(ppvresult)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_System_Com'*"]
pub type CALLCONV = i32;
#[doc = "*Required features: 'Win32_System_Com'*"]
pub const CC_FASTCALL: CALLCONV = 0i32;
#[doc = "*Required features: 'Win32_System_Com'*"]
pub const CC_CDECL: CALLCONV = 1i32;
#[doc = "*Required features: 'Win32_System_Com'*"]
pub const CC_MSCPASCAL: CALLCONV = 2i32;
#[doc = "*Required features: 'Win32_System_Com'*"]
pub const CC_PASCAL: CALLCONV = 2i32;
#[doc = "*Required features: 'Win32_System_Com'*"]
pub const CC_MACPASCAL: CALLCONV = 3i32;
#[doc = "*Required features: 'Win32_System_Com'*"]
pub const CC_STDCALL: CALLCONV = 4i32;
#[doc = "*Required features: 'Win32_System_Com'*"]
pub const CC_FPFASTCALL: CALLCONV = 5i32;
#[doc = "*Required features: 'Win32_System_Com'*"]
pub const CC_SYSCALL: CALLCONV = 6i32;
#[doc = "*Required features: 'Win32_System_Com'*"]
pub const CC_MPWCDECL: CALLCONV = 7i32;
#[doc = "*Required features: 'Win32_System_Com'*"]
pub const CC_MPWPASCAL: CALLCONV = 8i32;
#[doc = "*Required features: 'Win32_System_Com'*"]
pub const CC_MAX: CALLCONV = 9i32;
#[doc = "*Required features: 'Win32_System_Com'*"]
pub type CALLTYPE = i32;
#[doc = "*Required features: 'Win32_System_Com'*"]
pub const CALLTYPE_TOPLEVEL: CALLTYPE = 1i32;
#[doc = "*Required features: 'Win32_System_Com'*"]
pub const CALLTYPE_NESTED: CALLTYPE = 2i32;
#[doc = "*Required features: 'Win32_System_Com'*"]
pub const CALLTYPE_ASYNC: CALLTYPE = 3i32;
#[doc = "*Required features: 'Win32_System_Com'*"]
pub const CALLTYPE_TOPLEVEL_CALLPENDING: CALLTYPE = 4i32;
#[doc = "*Required features: 'Win32_System_Com'*"]
pub const CALLTYPE_ASYNC_CALLPENDING: CALLTYPE = 5i32;
#[repr(C)]
#[doc = "*Required features: 'Win32_System_Com'*"]
pub struct CATEGORYINFO {
    pub catid: ::windows::core::GUID,
    pub lcid: u32,
    pub szDescription: [u16; 128],
}
impl ::core::marker::Copy for CATEGORYINFO {}
impl ::core::clone::Clone for CATEGORYINFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for CATEGORYINFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CATEGORYINFO").field("catid", &self.catid).field("lcid", &self.lcid).field("szDescription", &self.szDescription).finish()
    }
}
unsafe impl ::windows::core::Abi for CATEGORYINFO {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for CATEGORYINFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<CATEGORYINFO>()) == 0 }
    }
}
impl ::core::cmp::Eq for CATEGORYINFO {}
impl ::core::default::Default for CATEGORYINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: 'Win32_System_Com'*"]
pub type CLSCTX = u32;
#[doc = "*Required features: 'Win32_System_Com'*"]
pub const CLSCTX_INPROC_SERVER: CLSCTX = 1u32;
#[doc = "*Required features: 'Win32_System_Com'*"]
pub const CLSCTX_INPROC_HANDLER: CLSCTX = 2u32;
#[doc = "*Required features: 'Win32_System_Com'*"]
pub const CLSCTX_LOCAL_SERVER: CLSCTX = 4u32;
#[doc = "*Required features: 'Win32_System_Com'*"]
pub const CLSCTX_INPROC_SERVER16: CLSCTX = 8u32;
#[doc = "*Required features: 'Win32_System_Com'*"]
pub const CLSCTX_REMOTE_SERVER: CLSCTX = 16u32;
#[doc = "*Required features: 'Win32_System_Com'*"]
pub const CLSCTX_INPROC_HANDLER16: CLSCTX = 32u32;
#[doc = "*Required features: 'Win32_System_Com'*"]
pub const CLSCTX_RESERVED1: CLSCTX = 64u32;
#[doc = "*Required features: 'Win32_System_Com'*"]
pub const CLSCTX_RESERVED2: CLSCTX = 128u32;
#[doc = "*Required features: 'Win32_System_Com'*"]
pub const CLSCTX_RESERVED3: CLSCTX = 256u32;
#[doc = "*Required features: 'Win32_System_Com'*"]
pub const CLSCTX_RESERVED4: CLSCTX = 512u32;
#[doc = "*Required features: 'Win32_System_Com'*"]
pub const CLSCTX_NO_CODE_DOWNLOAD: CLSCTX = 1024u32;
#[doc = "*Required features: 'Win32_System_Com'*"]
pub const CLSCTX_RESERVED5: CLSCTX = 2048u32;
#[doc = "*Required features: 'Win32_System_Com'*"]
pub const CLSCTX_NO_CUSTOM_MARSHAL: CLSCTX = 4096u32;
#[doc = "*Required features: 'Win32_System_Com'*"]
pub const CLSCTX_ENABLE_CODE_DOWNLOAD: CLSCTX = 8192u32;
#[doc = "*Required features: 'Win32_System_Com'*"]
pub const CLSCTX_NO_FAILURE_LOG: CLSCTX = 16384u32;
#[doc = "*Required features: 'Win32_System_Com'*"]
pub const CLSCTX_DISABLE_AAA: CLSCTX = 32768u32;
#[doc = "*Required features: 'Win32_System_Com'*"]
pub const CLSCTX_ENABLE_AAA: CLSCTX = 65536u32;
#[doc = "*Required features: 'Win32_System_Com'*"]
pub const CLSCTX_FROM_DEFAULT_CONTEXT: CLSCTX = 131072u32;
#[doc = "*Required features: 'Win32_System_Com'*"]
pub const CLSCTX_ACTIVATE_X86_SERVER: CLSCTX = 262144u32;
#[doc = "*Required features: 'Win32_System_Com'*"]
pub const CLSCTX_ACTIVATE_32_BIT_SERVER: CLSCTX = 262144u32;
#[doc = "*Required features: 'Win32_System_Com'*"]
pub const CLSCTX_ACTIVATE_64_BIT_SERVER: CLSCTX = 524288u32;
#[doc = "*Required features: 'Win32_System_Com'*"]
pub const CLSCTX_ENABLE_CLOAKING: CLSCTX = 1048576u32;
#[doc = "*Required features: 'Win32_System_Com'*"]
pub const CLSCTX_APPCONTAINER: CLSCTX = 4194304u32;
#[doc = "*Required features: 'Win32_System_Com'*"]
pub const CLSCTX_ACTIVATE_AAA_AS_IU: CLSCTX = 8388608u32;
#[doc = "*Required features: 'Win32_System_Com'*"]
pub const CLSCTX_RESERVED6: CLSCTX = 16777216u32;
#[doc = "*Required features: 'Win32_System_Com'*"]
pub const CLSCTX_ACTIVATE_ARM32_SERVER: CLSCTX = 33554432u32;
#[doc = "*Required features: 'Win32_System_Com'*"]
pub const CLSCTX_PS_DLL: CLSCTX = 2147483648u32;
#[doc = "*Required features: 'Win32_System_Com'*"]
pub const CLSCTX_ALL: CLSCTX = 23u32;
#[doc = "*Required features: 'Win32_System_Com'*"]
pub const CLSCTX_SERVER: CLSCTX = 21u32;
#[doc = "*Required features: 'Win32_System_Com', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CLSIDFromProgID<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(lpszprogid: Param0) -> ::windows::core::Result<::windows::core::GUID> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CLSIDFromProgID(lpszprogid: super::super::Foundation::PWSTR, lpclsid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT;
        }
        let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
        CLSIDFromProgID(lpszprogid.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::GUID>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_System_Com', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CLSIDFromProgIDEx<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(lpszprogid: Param0) -> ::windows::core::Result<::windows::core::GUID> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CLSIDFromProgIDEx(lpszprogid: super::super::Foundation::PWSTR, lpclsid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT;
        }
        let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
        CLSIDFromProgIDEx(lpszprogid.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::GUID>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_System_Com', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CLSIDFromString<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(lpsz: Param0) -> ::windows::core::Result<::windows::core::GUID> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CLSIDFromString(lpsz: super::super::Foundation::PWSTR, pclsid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT;
        }
        let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
        CLSIDFromString(lpsz.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::GUID>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[repr(C)]
#[doc = "*Required features: 'Win32_System_Com'*"]
pub struct COAUTHIDENTITY {
    pub User: *mut u16,
    pub UserLength: u32,
    pub Domain: *mut u16,
    pub DomainLength: u32,
    pub Password: *mut u16,
    pub PasswordLength: u32,
    pub Flags: u32,
}
impl ::core::marker::Copy for COAUTHIDENTITY {}
impl ::core::clone::Clone for COAUTHIDENTITY {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for COAUTHIDENTITY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("COAUTHIDENTITY").field("User", &self.User).field("UserLength", &self.UserLength).field("Domain", &self.Domain).field("DomainLength", &self.DomainLength).field("Password", &self.Password).field("PasswordLength", &self.PasswordLength).field("Flags", &self.Flags).finish()
    }
}
unsafe impl ::windows::core::Abi for COAUTHIDENTITY {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for COAUTHIDENTITY {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<COAUTHIDENTITY>()) == 0 }
    }
}
impl ::core::cmp::Eq for COAUTHIDENTITY {}
impl ::core::default::Default for COAUTHIDENTITY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_System_Com', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct COAUTHINFO {
    pub dwAuthnSvc: u32,
    pub dwAuthzSvc: u32,
    pub pwszServerPrincName: super::super::Foundation::PWSTR,
    pub dwAuthnLevel: u32,
    pub dwImpersonationLevel: u32,
    pub pAuthIdentityData: *mut COAUTHIDENTITY,
    pub dwCapabilities: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for COAUTHINFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for COAUTHINFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for COAUTHINFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("COAUTHINFO").field("dwAuthnSvc", &self.dwAuthnSvc).field("dwAuthzSvc", &self.dwAuthzSvc).field("pwszServerPrincName", &self.pwszServerPrincName).field("dwAuthnLevel", &self.dwAuthnLevel).field("dwImpersonationLevel", &self.dwImpersonationLevel).field("pAuthIdentityData", &self.pAuthIdentityData).field("dwCapabilities", &self.dwCapabilities).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for COAUTHINFO {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for COAUTHINFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<COAUTHINFO>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for COAUTHINFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for COAUTHINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: 'Win32_System_Com'*"]
pub type COINIT = u32;
#[doc = "*Required features: 'Win32_System_Com'*"]
pub const COINIT_APARTMENTTHREADED: COINIT = 2u32;
#[doc = "*Required features: 'Win32_System_Com'*"]
pub const COINIT_MULTITHREADED: COINIT = 0u32;
#[doc = "*Required features: 'Win32_System_Com'*"]
pub const COINIT_DISABLE_OLE1DDE: COINIT = 4u32;
#[doc = "*Required features: 'Win32_System_Com'*"]
pub const COINIT_SPEED_OVER_MEMORY: COINIT = 8u32;
#[doc = "*Required features: 'Win32_System_Com'*"]
pub type COINITBASE = i32;
#[doc = "*Required features: 'Win32_System_Com'*"]
pub const COINITBASE_MULTITHREADED: COINITBASE = 0i32;
#[doc = "*Required features: 'Win32_System_Com'*"]
pub type COMSD = i32;
#[doc = "*Required features: 'Win32_System_Com'*"]
pub const SD_LAUNCHPERMISSIONS: COMSD = 0i32;
#[doc = "*Required features: 'Win32_System_Com'*"]
pub const SD_ACCESSPERMISSIONS: COMSD = 1i32;
#[doc = "*Required features: 'Win32_System_Com'*"]
pub const SD_LAUNCHRESTRICTIONS: COMSD = 2i32;
#[doc = "*Required features: 'Win32_System_Com'*"]
pub const SD_ACCESSRESTRICTIONS: COMSD = 3i32;
#[doc = "*Required features: 'Win32_System_Com'*"]
pub const COM_RIGHTS_ACTIVATE_LOCAL: u32 = 8u32;
#[doc = "*Required features: 'Win32_System_Com'*"]
pub const COM_RIGHTS_ACTIVATE_REMOTE: u32 = 16u32;
#[doc = "*Required features: 'Win32_System_Com'*"]
pub const COM_RIGHTS_EXECUTE: u32 = 1u32;
#[doc = "*Required features: 'Win32_System_Com'*"]
pub const COM_RIGHTS_EXECUTE_LOCAL: u32 = 2u32;
#[doc = "*Required features: 'Win32_System_Com'*"]
pub const COM_RIGHTS_EXECUTE_REMOTE: u32 = 4u32;
#[doc = "*Required features: 'Win32_System_Com'*"]
pub const COM_RIGHTS_RESERVED1: u32 = 32u32;
#[doc = "*Required features: 'Win32_System_Com'*"]
pub const COM_RIGHTS_RESERVED2: u32 = 64u32;
#[repr(C)]
#[doc = "*Required features: 'Win32_System_Com'*"]
pub struct CONNECTDATA {
    pub pUnk: ::core::option::Option<::windows::core::IUnknown>,
    pub dwCookie: u32,
}
impl ::core::clone::Clone for CONNECTDATA {
    fn clone(&self) -> Self {
        Self { pUnk: self.pUnk.clone(), dwCookie: self.dwCookie }
    }
}
impl ::core::fmt::Debug for CONNECTDATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CONNECTDATA").field("pUnk", &self.pUnk).field("dwCookie", &self.dwCookie).finish()
    }
}
unsafe impl ::windows::core::Abi for CONNECTDATA {
    type Abi = ::core::mem::ManuallyDrop<Self>;
}
impl ::core::cmp::PartialEq for CONNECTDATA {
    fn eq(&self, other: &Self) -> bool {
        self.pUnk == other.pUnk && self.dwCookie == other.dwCookie
    }
}
impl ::core::cmp::Eq for CONNECTDATA {}
impl ::core::default::Default for CONNECTDATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_System_Com', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct COSERVERINFO {
    pub dwReserved1: u32,
    pub pwszName: super::super::Foundation::PWSTR,
    pub pAuthInfo: *mut COAUTHINFO,
    pub dwReserved2: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for COSERVERINFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for COSERVERINFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for COSERVERINFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("COSERVERINFO").field("dwReserved1", &self.dwReserved1).field("pwszName", &self.pwszName).field("pAuthInfo", &self.pAuthInfo).field("dwReserved2", &self.dwReserved2).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for COSERVERINFO {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for COSERVERINFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<COSERVERINFO>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for COSERVERINFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for COSERVERINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: 'Win32_System_Com'*"]
pub type COWAIT_FLAGS = i32;
#[doc = "*Required features: 'Win32_System_Com'*"]
pub const COWAIT_DEFAULT: COWAIT_FLAGS = 0i32;
#[doc = "*Required features: 'Win32_System_Com'*"]
pub const COWAIT_WAITALL: COWAIT_FLAGS = 1i32;
#[doc = "*Required features: 'Win32_System_Com'*"]
pub const COWAIT_ALERTABLE: COWAIT_FLAGS = 2i32;
#[doc = "*Required features: 'Win32_System_Com'*"]
pub const COWAIT_INPUTAVAILABLE: COWAIT_FLAGS = 4i32;
#[doc = "*Required features: 'Win32_System_Com'*"]
pub const COWAIT_DISPATCH_CALLS: COWAIT_FLAGS = 8i32;
#[doc = "*Required features: 'Win32_System_Com'*"]
pub const COWAIT_DISPATCH_WINDOW_MESSAGES: COWAIT_FLAGS = 16i32;
pub type CO_DEVICE_CATALOG_COOKIE = isize;
#[doc = "*Required features: 'Win32_System_Com'*"]
pub type CO_MARSHALING_CONTEXT_ATTRIBUTES = i32;
#[doc = "*Required features: 'Win32_System_Com'*"]
pub const CO_MARSHALING_SOURCE_IS_APP_CONTAINER: CO_MARSHALING_CONTEXT_ATTRIBUTES = 0i32;
#[doc = "*Required features: 'Win32_System_Com'*"]
pub const CO_MARSHALING_CONTEXT_ATTRIBUTE_RESERVED_1: CO_MARSHALING_CONTEXT_ATTRIBUTES = -2147483648i32;
#[doc = "*Required features: 'Win32_System_Com'*"]
pub const CO_MARSHALING_CONTEXT_ATTRIBUTE_RESERVED_2: CO_MARSHALING_CONTEXT_ATTRIBUTES = -2147483647i32;
#[doc = "*Required features: 'Win32_System_Com'*"]
pub const CO_MARSHALING_CONTEXT_ATTRIBUTE_RESERVED_3: CO_MARSHALING_CONTEXT_ATTRIBUTES = -2147483646i32;
#[doc = "*Required features: 'Win32_System_Com'*"]
pub const CO_MARSHALING_CONTEXT_ATTRIBUTE_RESERVED_4: CO_MARSHALING_CONTEXT_ATTRIBUTES = -2147483645i32;
#[doc = "*Required features: 'Win32_System_Com'*"]
pub const CO_MARSHALING_CONTEXT_ATTRIBUTE_RESERVED_5: CO_MARSHALING_CONTEXT_ATTRIBUTES = -2147483644i32;
#[doc = "*Required features: 'Win32_System_Com'*"]
pub const CO_MARSHALING_CONTEXT_ATTRIBUTE_RESERVED_6: CO_MARSHALING_CONTEXT_ATTRIBUTES = -2147483643i32;
#[doc = "*Required features: 'Win32_System_Com'*"]
pub const CO_MARSHALING_CONTEXT_ATTRIBUTE_RESERVED_7: CO_MARSHALING_CONTEXT_ATTRIBUTES = -2147483642i32;
#[doc = "*Required features: 'Win32_System_Com'*"]
pub const CO_MARSHALING_CONTEXT_ATTRIBUTE_RESERVED_8: CO_MARSHALING_CONTEXT_ATTRIBUTES = -2147483641i32;
#[doc = "*Required features: 'Win32_System_Com'*"]
pub const CO_MARSHALING_CONTEXT_ATTRIBUTE_RESERVED_9: CO_MARSHALING_CONTEXT_ATTRIBUTES = -2147483640i32;
#[doc = "*Required features: 'Win32_System_Com'*"]
pub const CO_MARSHALING_CONTEXT_ATTRIBUTE_RESERVED_10: CO_MARSHALING_CONTEXT_ATTRIBUTES = -2147483639i32;
#[doc = "*Required features: 'Win32_System_Com'*"]
pub const CO_MARSHALING_CONTEXT_ATTRIBUTE_RESERVED_11: CO_MARSHALING_CONTEXT_ATTRIBUTES = -2147483638i32;
#[doc = "*Required features: 'Win32_System_Com'*"]
pub const CO_MARSHALING_CONTEXT_ATTRIBUTE_RESERVED_12: CO_MARSHALING_CONTEXT_ATTRIBUTES = -2147483637i32;
#[doc = "*Required features: 'Win32_System_Com'*"]
pub const CO_MARSHALING_CONTEXT_ATTRIBUTE_RESERVED_13: CO_MARSHALING_CONTEXT_ATTRIBUTES = -2147483636i32;
#[doc = "*Required features: 'Win32_System_Com'*"]
pub const CO_MARSHALING_CONTEXT_ATTRIBUTE_RESERVED_14: CO_MARSHALING_CONTEXT_ATTRIBUTES = -2147483635i32;
#[doc = "*Required features: 'Win32_System_Com'*"]
pub const CO_MARSHALING_CONTEXT_ATTRIBUTE_RESERVED_15: CO_MARSHALING_CONTEXT_ATTRIBUTES = -2147483634i32;
#[doc = "*Required features: 'Win32_System_Com'*"]
pub const CO_MARSHALING_CONTEXT_ATTRIBUTE_RESERVED_16: CO_MARSHALING_CONTEXT_ATTRIBUTES = -2147483633i32;
#[doc = "*Required features: 'Win32_System_Com'*"]
pub const CO_MARSHALING_CONTEXT_ATTRIBUTE_RESERVED_17: CO_MARSHALING_CONTEXT_ATTRIBUTES = -2147483632i32;
#[doc = "*Required features: 'Win32_System_Com'*"]
pub const CO_MARSHALING_CONTEXT_ATTRIBUTE_RESERVED_18: CO_MARSHALING_CONTEXT_ATTRIBUTES = -2147483631i32;
pub type CO_MTA_USAGE_COOKIE = isize;
#[repr(C)]
#[doc = "*Required features: 'Win32_System_Com'*"]
pub struct CSPLATFORM {
    pub dwPlatformId: u32,
    pub dwVersionHi: u32,
    pub dwVersionLo: u32,
    pub dwProcessorArch: u32,
}
impl ::core::marker::Copy for CSPLATFORM {}
impl ::core::clone::Clone for CSPLATFORM {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for CSPLATFORM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CSPLATFORM").field("dwPlatformId", &self.dwPlatformId).field("dwVersionHi", &self.dwVersionHi).field("dwVersionLo", &self.dwVersionLo).field("dwProcessorArch", &self.dwProcessorArch).finish()
    }
}
unsafe impl ::windows::core::Abi for CSPLATFORM {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for CSPLATFORM {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<CSPLATFORM>()) == 0 }
    }
}
impl ::core::cmp::Eq for CSPLATFORM {}
impl ::core::default::Default for CSPLATFORM {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_System_Com', 'Win32_Foundation', 'Win32_System_Ole'*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
pub struct CUSTDATA {
    pub cCustData: u32,
    pub prgCustData: *mut CUSTDATAITEM,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
impl ::core::marker::Copy for CUSTDATA {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
impl ::core::clone::Clone for CUSTDATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
impl ::core::fmt::Debug for CUSTDATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CUSTDATA").field("cCustData", &self.cCustData).field("prgCustData", &self.prgCustData).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
unsafe impl ::windows::core::Abi for CUSTDATA {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
impl ::core::cmp::PartialEq for CUSTDATA {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<CUSTDATA>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
impl ::core::cmp::Eq for CUSTDATA {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
impl ::core::default::Default for CUSTDATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_System_Com', 'Win32_Foundation', 'Win32_System_Ole'*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
pub struct CUSTDATAITEM {
    pub guid: ::windows::core::GUID,
    pub varValue: VARIANT,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
impl ::core::clone::Clone for CUSTDATAITEM {
    fn clone(&self) -> Self {
        Self { guid: self.guid, varValue: self.varValue.clone() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
unsafe impl ::windows::core::Abi for CUSTDATAITEM {
    type Abi = ::core::mem::ManuallyDrop<Self>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
impl ::core::cmp::PartialEq for CUSTDATAITEM {
    fn eq(&self, other: &Self) -> bool {
        self.guid == other.guid && self.varValue == other.varValue
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
impl ::core::cmp::Eq for CUSTDATAITEM {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
impl ::core::default::Default for CUSTDATAITEM {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: 'Win32_System_Com'*"]
pub type CWMO_FLAGS = i32;
#[doc = "*Required features: 'Win32_System_Com'*"]
pub const CWMO_DEFAULT: CWMO_FLAGS = 0i32;
#[doc = "*Required features: 'Win32_System_Com'*"]
pub const CWMO_DISPATCH_CALLS: CWMO_FLAGS = 1i32;
#[doc = "*Required features: 'Win32_System_Com'*"]
pub const CWMO_DISPATCH_WINDOW_MESSAGES: CWMO_FLAGS = 2i32;
#[doc = "*Required features: 'Win32_System_Com'*"]
pub const CWMO_MAX_HANDLES: u32 = 56u32;
#[repr(C)]
#[doc = "*Required features: 'Win32_System_Com'*"]
pub union CY {
    pub Anonymous: CY_0,
    pub int64: i64,
}
impl ::core::marker::Copy for CY {}
impl ::core::clone::Clone for CY {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for CY {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for CY {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<CY>()) == 0 }
    }
}
impl ::core::cmp::Eq for CY {}
impl ::core::default::Default for CY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_System_Com'*"]
pub struct CY_0 {
    pub Lo: u32,
    pub Hi: i32,
}
impl ::core::marker::Copy for CY_0 {}
impl ::core::clone::Clone for CY_0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for CY_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CY_0").field("Lo", &self.Lo).field("Hi", &self.Hi).finish()
    }
}
unsafe impl ::windows::core::Abi for CY_0 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for CY_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<CY_0>()) == 0 }
    }
}
impl ::core::cmp::Eq for CY_0 {}
impl ::core::default::Default for CY_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: 'Win32_System_Com'*"]
#[inline]
pub unsafe fn CoAddRefServerProcess() -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CoAddRefServerProcess() -> u32;
        }
        ::core::mem::transmute(CoAddRefServerProcess())
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_System_Com'*"]
#[inline]
pub unsafe fn CoAllowSetForegroundWindow<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>>(punk: Param0, lpvreserved: *const ::core::ffi::c_void) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CoAllowSetForegroundWindow(punk: *mut ::core::ffi::c_void, lpvreserved: *const ::core::ffi::c_void) -> ::windows::core::HRESULT;
        }
        CoAllowSetForegroundWindow(punk.into_param().abi(), ::core::mem::transmute(lpvreserved)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_System_Com'*"]
#[inline]
pub unsafe fn CoAllowUnmarshalerCLSID(clsid: *const ::windows::core::GUID) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CoAllowUnmarshalerCLSID(clsid: *const ::windows::core::GUID) -> ::windows::core::HRESULT;
        }
        CoAllowUnmarshalerCLSID(::core::mem::transmute(clsid)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_System_Com'*"]
#[inline]
pub unsafe fn CoBuildVersion() -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CoBuildVersion() -> u32;
        }
        ::core::mem::transmute(CoBuildVersion())
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_System_Com'*"]
#[inline]
pub unsafe fn CoCancelCall(dwthreadid: u32, ultimeout: u32) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CoCancelCall(dwthreadid: u32, ultimeout: u32) -> ::windows::core::HRESULT;
        }
        CoCancelCall(::core::mem::transmute(dwthreadid), ::core::mem::transmute(ultimeout)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_System_Com'*"]
#[inline]
pub unsafe fn CoCopyProxy<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>>(pproxy: Param0) -> ::windows::core::Result<::windows::core::IUnknown> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CoCopyProxy(pproxy: *mut ::core::ffi::c_void, ppcopy: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT;
        }
        let mut result__: *mut ::core::ffi::c_void = ::core::mem::zeroed();
        CoCopyProxy(pproxy.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::IUnknown>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_System_Com'*"]
#[inline]
pub unsafe fn CoCreateFreeThreadedMarshaler<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>>(punkouter: Param0) -> ::windows::core::Result<::windows::core::IUnknown> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CoCreateFreeThreadedMarshaler(punkouter: *mut ::core::ffi::c_void, ppunkmarshal: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT;
        }
        let mut result__: *mut ::core::ffi::c_void = ::core::mem::zeroed();
        CoCreateFreeThreadedMarshaler(punkouter.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::IUnknown>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_System_Com'*"]
#[inline]
pub unsafe fn CoCreateGuid() -> ::windows::core::Result<::windows::core::GUID> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CoCreateGuid(pguid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT;
        }
        let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
        CoCreateGuid(::core::mem::transmute(&mut result__)).from_abi::<::windows::core::GUID>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_System_Com'*"]
#[inline]
pub unsafe fn CoCreateInstance<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>, T: ::windows::core::Interface>(rclsid: *const ::windows::core::GUID, punkouter: Param1, dwclscontext: CLSCTX) -> ::windows::core::Result<T> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CoCreateInstance(rclsid: *const ::windows::core::GUID, punkouter: *mut ::core::ffi::c_void, dwclscontext: CLSCTX, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT;
        }
        let mut result__ = ::core::option::Option::None;
        CoCreateInstance(::core::mem::transmute(rclsid), punkouter.into_param().abi(), ::core::mem::transmute(dwclscontext), &<T as ::windows::core::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_System_Com', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CoCreateInstanceEx<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>>(clsid: *const ::windows::core::GUID, punkouter: Param1, dwclsctx: CLSCTX, pserverinfo: *const COSERVERINFO, dwcount: u32, presults: *mut MULTI_QI) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CoCreateInstanceEx(clsid: *const ::windows::core::GUID, punkouter: *mut ::core::ffi::c_void, dwclsctx: CLSCTX, pserverinfo: *const COSERVERINFO, dwcount: u32, presults: *mut MULTI_QI) -> ::windows::core::HRESULT;
        }
        CoCreateInstanceEx(::core::mem::transmute(clsid), punkouter.into_param().abi(), ::core::mem::transmute(dwclsctx), ::core::mem::transmute(pserverinfo), ::core::mem::transmute(dwcount), ::core::mem::transmute(presults)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_System_Com'*"]
#[inline]
pub unsafe fn CoCreateInstanceFromApp<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>>(clsid: *const ::windows::core::GUID, punkouter: Param1, dwclsctx: CLSCTX, reserved: *const ::core::ffi::c_void, dwcount: u32, presults: *mut MULTI_QI) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CoCreateInstanceFromApp(clsid: *const ::windows::core::GUID, punkouter: *mut ::core::ffi::c_void, dwclsctx: CLSCTX, reserved: *const ::core::ffi::c_void, dwcount: u32, presults: *mut MULTI_QI) -> ::windows::core::HRESULT;
        }
        CoCreateInstanceFromApp(::core::mem::transmute(clsid), punkouter.into_param().abi(), ::core::mem::transmute(dwclsctx), ::core::mem::transmute(reserved), ::core::mem::transmute(dwcount), ::core::mem::transmute(presults)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_System_Com'*"]
#[inline]
pub unsafe fn CoDecrementMTAUsage<'a, Param0: ::windows::core::IntoParam<'a, CO_MTA_USAGE_COOKIE>>(cookie: Param0) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CoDecrementMTAUsage(cookie: CO_MTA_USAGE_COOKIE) -> ::windows::core::HRESULT;
        }
        CoDecrementMTAUsage(cookie.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_System_Com'*"]
#[inline]
pub unsafe fn CoDisableCallCancellation(preserved: *const ::core::ffi::c_void) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CoDisableCallCancellation(preserved: *const ::core::ffi::c_void) -> ::windows::core::HRESULT;
        }
        CoDisableCallCancellation(::core::mem::transmute(preserved)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_System_Com'*"]
#[inline]
pub unsafe fn CoDisconnectContext(dwtimeout: u32) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CoDisconnectContext(dwtimeout: u32) -> ::windows::core::HRESULT;
        }
        CoDisconnectContext(::core::mem::transmute(dwtimeout)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_System_Com'*"]
#[inline]
pub unsafe fn CoDisconnectObject<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>>(punk: Param0, dwreserved: u32) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CoDisconnectObject(punk: *mut ::core::ffi::c_void, dwreserved: u32) -> ::windows::core::HRESULT;
        }
        CoDisconnectObject(punk.into_param().abi(), ::core::mem::transmute(dwreserved)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_System_Com', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CoDosDateTimeToFileTime(ndosdate: u16, ndostime: u16, lpfiletime: *mut super::super::Foundation::FILETIME) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CoDosDateTimeToFileTime(ndosdate: u16, ndostime: u16, lpfiletime: *mut super::super::Foundation::FILETIME) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(CoDosDateTimeToFileTime(::core::mem::transmute(ndosdate), ::core::mem::transmute(ndostime), ::core::mem::transmute(lpfiletime)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_System_Com'*"]
#[inline]
pub unsafe fn CoEnableCallCancellation(preserved: *const ::core::ffi::c_void) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CoEnableCallCancellation(preserved: *const ::core::ffi::c_void) -> ::windows::core::HRESULT;
        }
        CoEnableCallCancellation(::core::mem::transmute(preserved)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_System_Com', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CoFileTimeNow() -> ::windows::core::Result<super::super::Foundation::FILETIME> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CoFileTimeNow(lpfiletime: *mut super::super::Foundation::FILETIME) -> ::windows::core::HRESULT;
        }
        let mut result__: super::super::Foundation::FILETIME = ::core::mem::zeroed();
        CoFileTimeNow(::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::FILETIME>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_System_Com', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CoFileTimeToDosDateTime(lpfiletime: *const super::super::Foundation::FILETIME, lpdosdate: *mut u16, lpdostime: *mut u16) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CoFileTimeToDosDateTime(lpfiletime: *const super::super::Foundation::FILETIME, lpdosdate: *mut u16, lpdostime: *mut u16) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(CoFileTimeToDosDateTime(::core::mem::transmute(lpfiletime), ::core::mem::transmute(lpdosdate), ::core::mem::transmute(lpdostime)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_System_Com'*"]
#[inline]
pub unsafe fn CoFreeAllLibraries() {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CoFreeAllLibraries();
        }
        CoFreeAllLibraries()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_System_Com', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CoFreeLibrary<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HINSTANCE>>(hinst: Param0) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CoFreeLibrary(hinst: super::super::Foundation::HINSTANCE);
        }
        CoFreeLibrary(hinst.into_param().abi())
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_System_Com'*"]
#[inline]
pub unsafe fn CoFreeUnusedLibraries() {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CoFreeUnusedLibraries();
        }
        CoFreeUnusedLibraries()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_System_Com'*"]
#[inline]
pub unsafe fn CoFreeUnusedLibrariesEx(dwunloaddelay: u32, dwreserved: u32) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CoFreeUnusedLibrariesEx(dwunloaddelay: u32, dwreserved: u32);
        }
        CoFreeUnusedLibrariesEx(::core::mem::transmute(dwunloaddelay), ::core::mem::transmute(dwreserved))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_System_Com'*"]
#[inline]
pub unsafe fn CoGetApartmentType(papttype: *mut APTTYPE, paptqualifier: *mut APTTYPEQUALIFIER) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CoGetApartmentType(papttype: *mut APTTYPE, paptqualifier: *mut APTTYPEQUALIFIER) -> ::windows::core::HRESULT;
        }
        CoGetApartmentType(::core::mem::transmute(papttype), ::core::mem::transmute(paptqualifier)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_System_Com'*"]
#[inline]
pub unsafe fn CoGetCallContext(riid: *const ::windows::core::GUID, ppinterface: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CoGetCallContext(riid: *const ::windows::core::GUID, ppinterface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT;
        }
        CoGetCallContext(::core::mem::transmute(riid), ::core::mem::transmute(ppinterface)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_System_Com'*"]
#[inline]
pub unsafe fn CoGetCallerTID() -> ::windows::core::Result<u32> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CoGetCallerTID(lpdwtid: *mut u32) -> ::windows::core::HRESULT;
        }
        let mut result__: u32 = ::core::mem::zeroed();
        CoGetCallerTID(::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_System_Com'*"]
#[inline]
pub unsafe fn CoGetCancelObject(dwthreadid: u32, iid: *const ::windows::core::GUID, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CoGetCancelObject(dwthreadid: u32, iid: *const ::windows::core::GUID, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT;
        }
        CoGetCancelObject(::core::mem::transmute(dwthreadid), ::core::mem::transmute(iid), ::core::mem::transmute(ppunk)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_System_Com'*"]
#[inline]
pub unsafe fn CoGetClassObject(rclsid: *const ::windows::core::GUID, dwclscontext: CLSCTX, pvreserved: *const ::core::ffi::c_void, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CoGetClassObject(rclsid: *const ::windows::core::GUID, dwclscontext: CLSCTX, pvreserved: *const ::core::ffi::c_void, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT;
        }
        CoGetClassObject(::core::mem::transmute(rclsid), ::core::mem::transmute(dwclscontext), ::core::mem::transmute(pvreserved), ::core::mem::transmute(riid), ::core::mem::transmute(ppv)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_System_Com'*"]
#[inline]
pub unsafe fn CoGetContextToken() -> ::windows::core::Result<usize> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CoGetContextToken(ptoken: *mut usize) -> ::windows::core::HRESULT;
        }
        let mut result__: usize = ::core::mem::zeroed();
        CoGetContextToken(::core::mem::transmute(&mut result__)).from_abi::<usize>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_System_Com'*"]
#[inline]
pub unsafe fn CoGetCurrentLogicalThreadId() -> ::windows::core::Result<::windows::core::GUID> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CoGetCurrentLogicalThreadId(pguid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT;
        }
        let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
        CoGetCurrentLogicalThreadId(::core::mem::transmute(&mut result__)).from_abi::<::windows::core::GUID>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_System_Com'*"]
#[inline]
pub unsafe fn CoGetCurrentProcess() -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CoGetCurrentProcess() -> u32;
        }
        ::core::mem::transmute(CoGetCurrentProcess())
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_System_Com'*"]
#[inline]
pub unsafe fn CoGetMalloc(dwmemcontext: u32) -> ::windows::core::Result<IMalloc> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CoGetMalloc(dwmemcontext: u32, ppmalloc: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT;
        }
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        CoGetMalloc(::core::mem::transmute(dwmemcontext), ::core::mem::transmute(&mut result__)).from_abi::<IMalloc>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_System_Com', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CoGetObject<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(pszname: Param0, pbindoptions: *const BIND_OPTS, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CoGetObject(pszname: super::super::Foundation::PWSTR, pbindoptions: *const BIND_OPTS, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT;
        }
        CoGetObject(pszname.into_param().abi(), ::core::mem::transmute(pbindoptions), ::core::mem::transmute(riid), ::core::mem::transmute(ppv)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_System_Com'*"]
#[inline]
pub unsafe fn CoGetObjectContext(riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CoGetObjectContext(riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT;
        }
        CoGetObjectContext(::core::mem::transmute(riid), ::core::mem::transmute(ppv)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_System_Com'*"]
#[inline]
pub unsafe fn CoGetPSClsid(riid: *const ::windows::core::GUID) -> ::windows::core::Result<::windows::core::GUID> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CoGetPSClsid(riid: *const ::windows::core::GUID, pclsid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT;
        }
        let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
        CoGetPSClsid(::core::mem::transmute(riid), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::GUID>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_System_Com', 'Win32_Foundation', 'Win32_Security'*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn CoGetSystemSecurityPermissions(comsdtype: COMSD, ppsd: *mut *mut super::super::Security::SECURITY_DESCRIPTOR) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CoGetSystemSecurityPermissions(comsdtype: COMSD, ppsd: *mut *mut super::super::Security::SECURITY_DESCRIPTOR) -> ::windows::core::HRESULT;
        }
        CoGetSystemSecurityPermissions(::core::mem::transmute(comsdtype), ::core::mem::transmute(ppsd)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_System_Com'*"]
#[inline]
pub unsafe fn CoGetTreatAsClass(clsidold: *const ::windows::core::GUID) -> ::windows::core::Result<::windows::core::GUID> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CoGetTreatAsClass(clsidold: *const ::windows::core::GUID, pclsidnew: *mut ::windows::core::GUID) -> ::windows::core::HRESULT;
        }
        let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
        CoGetTreatAsClass(::core::mem::transmute(clsidold), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::GUID>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_System_Com'*"]
#[inline]
pub unsafe fn CoImpersonateClient() -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CoImpersonateClient() -> ::windows::core::HRESULT;
        }
        CoImpersonateClient().ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_System_Com'*"]
#[inline]
pub unsafe fn CoIncrementMTAUsage() -> ::windows::core::Result<CO_MTA_USAGE_COOKIE> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CoIncrementMTAUsage(pcookie: *mut CO_MTA_USAGE_COOKIE) -> ::windows::core::HRESULT;
        }
        let mut result__: CO_MTA_USAGE_COOKIE = ::core::mem::zeroed();
        CoIncrementMTAUsage(::core::mem::transmute(&mut result__)).from_abi::<CO_MTA_USAGE_COOKIE>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_System_Com'*"]
#[inline]
pub unsafe fn CoInitialize(pvreserved: *const ::core::ffi::c_void) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CoInitialize(pvreserved: *const ::core::ffi::c_void) -> ::windows::core::HRESULT;
        }
        CoInitialize(::core::mem::transmute(pvreserved)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_System_Com'*"]
#[inline]
pub unsafe fn CoInitializeEx(pvreserved: *const ::core::ffi::c_void, dwcoinit: COINIT) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CoInitializeEx(pvreserved: *const ::core::ffi::c_void, dwcoinit: COINIT) -> ::windows::core::HRESULT;
        }
        CoInitializeEx(::core::mem::transmute(pvreserved), ::core::mem::transmute(dwcoinit)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_System_Com', 'Win32_Foundation', 'Win32_Security'*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn CoInitializeSecurity(psecdesc: *const super::super::Security::SECURITY_DESCRIPTOR, cauthsvc: i32, asauthsvc: *const SOLE_AUTHENTICATION_SERVICE, preserved1: *const ::core::ffi::c_void, dwauthnlevel: RPC_C_AUTHN_LEVEL, dwimplevel: RPC_C_IMP_LEVEL, pauthlist: *const ::core::ffi::c_void, dwcapabilities: EOLE_AUTHENTICATION_CAPABILITIES, preserved3: *const ::core::ffi::c_void) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CoInitializeSecurity(psecdesc: *const super::super::Security::SECURITY_DESCRIPTOR, cauthsvc: i32, asauthsvc: *const SOLE_AUTHENTICATION_SERVICE, preserved1: *const ::core::ffi::c_void, dwauthnlevel: RPC_C_AUTHN_LEVEL, dwimplevel: RPC_C_IMP_LEVEL, pauthlist: *const ::core::ffi::c_void, dwcapabilities: EOLE_AUTHENTICATION_CAPABILITIES, preserved3: *const ::core::ffi::c_void) -> ::windows::core::HRESULT;
        }
        CoInitializeSecurity(::core::mem::transmute(psecdesc), ::core::mem::transmute(cauthsvc), ::core::mem::transmute(asauthsvc), ::core::mem::transmute(preserved1), ::core::mem::transmute(dwauthnlevel), ::core::mem::transmute(dwimplevel), ::core::mem::transmute(pauthlist), ::core::mem::transmute(dwcapabilities), ::core::mem::transmute(preserved3)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_System_Com', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CoInstall<'a, Param0: ::windows::core::IntoParam<'a, IBindCtx>, Param4: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(pbc: Param0, dwflags: u32, pclassspec: *const uCLSSPEC, pquery: *const QUERYCONTEXT, pszcodebase: Param4) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CoInstall(pbc: ::windows::core::RawPtr, dwflags: u32, pclassspec: *const uCLSSPEC, pquery: *const QUERYCONTEXT, pszcodebase: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT;
        }
        CoInstall(pbc.into_param().abi(), ::core::mem::transmute(dwflags), ::core::mem::transmute(pclassspec), ::core::mem::transmute(pquery), pszcodebase.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_System_Com', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CoInvalidateRemoteMachineBindings<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(pszmachinename: Param0) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CoInvalidateRemoteMachineBindings(pszmachinename: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT;
        }
        CoInvalidateRemoteMachineBindings(pszmachinename.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_System_Com', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CoIsHandlerConnected<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>>(punk: Param0) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CoIsHandlerConnected(punk: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(CoIsHandlerConnected(punk.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_System_Com', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CoIsOle1Class(rclsid: *const ::windows::core::GUID) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CoIsOle1Class(rclsid: *const ::windows::core::GUID) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(CoIsOle1Class(::core::mem::transmute(rclsid)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_System_Com', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CoLoadLibrary<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(lpszlibname: Param0, bautofree: Param1) -> super::super::Foundation::HINSTANCE {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CoLoadLibrary(lpszlibname: super::super::Foundation::PWSTR, bautofree: super::super::Foundation::BOOL) -> super::super::Foundation::HINSTANCE;
        }
        ::core::mem::transmute(CoLoadLibrary(lpszlibname.into_param().abi(), bautofree.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_System_Com', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CoLockObjectExternal<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(punk: Param0, flock: Param1, flastunlockreleases: Param2) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CoLockObjectExternal(punk: *mut ::core::ffi::c_void, flock: super::super::Foundation::BOOL, flastunlockreleases: super::super::Foundation::BOOL) -> ::windows::core::HRESULT;
        }
        CoLockObjectExternal(punk.into_param().abi(), flock.into_param().abi(), flastunlockreleases.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_System_Com', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CoQueryAuthenticationServices(pcauthsvc: *mut u32, asauthsvc: *mut *mut SOLE_AUTHENTICATION_SERVICE) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CoQueryAuthenticationServices(pcauthsvc: *mut u32, asauthsvc: *mut *mut SOLE_AUTHENTICATION_SERVICE) -> ::windows::core::HRESULT;
        }
        CoQueryAuthenticationServices(::core::mem::transmute(pcauthsvc), ::core::mem::transmute(asauthsvc)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_System_Com', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CoQueryClientBlanket(pauthnsvc: *mut u32, pauthzsvc: *mut u32, pserverprincname: *mut super::super::Foundation::PWSTR, pauthnlevel: *mut u32, pimplevel: *mut u32, pprivs: *mut *mut ::core::ffi::c_void, pcapabilities: *mut u32) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CoQueryClientBlanket(pauthnsvc: *mut u32, pauthzsvc: *mut u32, pserverprincname: *mut super::super::Foundation::PWSTR, pauthnlevel: *mut u32, pimplevel: *mut u32, pprivs: *mut *mut ::core::ffi::c_void, pcapabilities: *mut u32) -> ::windows::core::HRESULT;
        }
        CoQueryClientBlanket(::core::mem::transmute(pauthnsvc), ::core::mem::transmute(pauthzsvc), ::core::mem::transmute(pserverprincname), ::core::mem::transmute(pauthnlevel), ::core::mem::transmute(pimplevel), ::core::mem::transmute(pprivs), ::core::mem::transmute(pcapabilities)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_System_Com', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CoQueryProxyBlanket<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>>(pproxy: Param0, pwauthnsvc: *mut u32, pauthzsvc: *mut u32, pserverprincname: *mut super::super::Foundation::PWSTR, pauthnlevel: *mut u32, pimplevel: *mut u32, pauthinfo: *mut *mut ::core::ffi::c_void, pcapabilites: *mut u32) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CoQueryProxyBlanket(pproxy: *mut ::core::ffi::c_void, pwauthnsvc: *mut u32, pauthzsvc: *mut u32, pserverprincname: *mut super::super::Foundation::PWSTR, pauthnlevel: *mut u32, pimplevel: *mut u32, pauthinfo: *mut *mut ::core::ffi::c_void, pcapabilites: *mut u32) -> ::windows::core::HRESULT;
        }
        CoQueryProxyBlanket(pproxy.into_param().abi(), ::core::mem::transmute(pwauthnsvc), ::core::mem::transmute(pauthzsvc), ::core::mem::transmute(pserverprincname), ::core::mem::transmute(pauthnlevel), ::core::mem::transmute(pimplevel), ::core::mem::transmute(pauthinfo), ::core::mem::transmute(pcapabilites)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_System_Com'*"]
#[inline]
pub unsafe fn CoRegisterActivationFilter<'a, Param0: ::windows::core::IntoParam<'a, IActivationFilter>>(pactivationfilter: Param0) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CoRegisterActivationFilter(pactivationfilter: ::windows::core::RawPtr) -> ::windows::core::HRESULT;
        }
        CoRegisterActivationFilter(pactivationfilter.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_System_Com'*"]
#[inline]
pub unsafe fn CoRegisterChannelHook<'a, Param1: ::windows::core::IntoParam<'a, IChannelHook>>(extensionuuid: *const ::windows::core::GUID, pchannelhook: Param1) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CoRegisterChannelHook(extensionuuid: *const ::windows::core::GUID, pchannelhook: ::windows::core::RawPtr) -> ::windows::core::HRESULT;
        }
        CoRegisterChannelHook(::core::mem::transmute(extensionuuid), pchannelhook.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_System_Com'*"]
#[inline]
pub unsafe fn CoRegisterClassObject<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>>(rclsid: *const ::windows::core::GUID, punk: Param1, dwclscontext: CLSCTX, flags: u32) -> ::windows::core::Result<u32> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CoRegisterClassObject(rclsid: *const ::windows::core::GUID, punk: *mut ::core::ffi::c_void, dwclscontext: CLSCTX, flags: u32, lpdwregister: *mut u32) -> ::windows::core::HRESULT;
        }
        let mut result__: u32 = ::core::mem::zeroed();
        CoRegisterClassObject(::core::mem::transmute(rclsid), punk.into_param().abi(), ::core::mem::transmute(dwclscontext), ::core::mem::transmute(flags), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_System_Com', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CoRegisterDeviceCatalog<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(deviceinstanceid: Param0) -> ::windows::core::Result<CO_DEVICE_CATALOG_COOKIE> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CoRegisterDeviceCatalog(deviceinstanceid: super::super::Foundation::PWSTR, cookie: *mut CO_DEVICE_CATALOG_COOKIE) -> ::windows::core::HRESULT;
        }
        let mut result__: CO_DEVICE_CATALOG_COOKIE = ::core::mem::zeroed();
        CoRegisterDeviceCatalog(deviceinstanceid.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<CO_DEVICE_CATALOG_COOKIE>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_System_Com'*"]
#[inline]
pub unsafe fn CoRegisterInitializeSpy<'a, Param0: ::windows::core::IntoParam<'a, IInitializeSpy>>(pspy: Param0) -> ::windows::core::Result<u64> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CoRegisterInitializeSpy(pspy: ::windows::core::RawPtr, pulicookie: *mut u64) -> ::windows::core::HRESULT;
        }
        let mut result__: u64 = ::core::mem::zeroed();
        CoRegisterInitializeSpy(pspy.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<u64>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_System_Com'*"]
#[inline]
pub unsafe fn CoRegisterMallocSpy<'a, Param0: ::windows::core::IntoParam<'a, IMallocSpy>>(pmallocspy: Param0) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CoRegisterMallocSpy(pmallocspy: ::windows::core::RawPtr) -> ::windows::core::HRESULT;
        }
        CoRegisterMallocSpy(pmallocspy.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_System_Com'*"]
#[inline]
pub unsafe fn CoRegisterPSClsid(riid: *const ::windows::core::GUID, rclsid: *const ::windows::core::GUID) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CoRegisterPSClsid(riid: *const ::windows::core::GUID, rclsid: *const ::windows::core::GUID) -> ::windows::core::HRESULT;
        }
        CoRegisterPSClsid(::core::mem::transmute(riid), ::core::mem::transmute(rclsid)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_System_Com'*"]
#[inline]
pub unsafe fn CoRegisterSurrogate<'a, Param0: ::windows::core::IntoParam<'a, ISurrogate>>(psurrogate: Param0) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CoRegisterSurrogate(psurrogate: ::windows::core::RawPtr) -> ::windows::core::HRESULT;
        }
        CoRegisterSurrogate(psurrogate.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_System_Com'*"]
#[inline]
pub unsafe fn CoReleaseServerProcess() -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CoReleaseServerProcess() -> u32;
        }
        ::core::mem::transmute(CoReleaseServerProcess())
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_System_Com'*"]
#[inline]
pub unsafe fn CoResumeClassObjects() -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CoResumeClassObjects() -> ::windows::core::HRESULT;
        }
        CoResumeClassObjects().ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_System_Com'*"]
#[inline]
pub unsafe fn CoRevertToSelf() -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CoRevertToSelf() -> ::windows::core::HRESULT;
        }
        CoRevertToSelf().ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_System_Com'*"]
#[inline]
pub unsafe fn CoRevokeClassObject(dwregister: u32) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CoRevokeClassObject(dwregister: u32) -> ::windows::core::HRESULT;
        }
        CoRevokeClassObject(::core::mem::transmute(dwregister)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_System_Com'*"]
#[inline]
pub unsafe fn CoRevokeDeviceCatalog<'a, Param0: ::windows::core::IntoParam<'a, CO_DEVICE_CATALOG_COOKIE>>(cookie: Param0) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CoRevokeDeviceCatalog(cookie: CO_DEVICE_CATALOG_COOKIE) -> ::windows::core::HRESULT;
        }
        CoRevokeDeviceCatalog(cookie.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_System_Com'*"]
#[inline]
pub unsafe fn CoRevokeInitializeSpy(ulicookie: u64) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CoRevokeInitializeSpy(ulicookie: u64) -> ::windows::core::HRESULT;
        }
        CoRevokeInitializeSpy(::core::mem::transmute(ulicookie)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_System_Com'*"]
#[inline]
pub unsafe fn CoRevokeMallocSpy() -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CoRevokeMallocSpy() -> ::windows::core::HRESULT;
        }
        CoRevokeMallocSpy().ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_System_Com'*"]
#[inline]
pub unsafe fn CoSetCancelObject<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>>(punk: Param0) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CoSetCancelObject(punk: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT;
        }
        CoSetCancelObject(punk.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_System_Com', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CoSetProxyBlanket<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>, Param3: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(pproxy: Param0, dwauthnsvc: u32, dwauthzsvc: u32, pserverprincname: Param3, dwauthnlevel: RPC_C_AUTHN_LEVEL, dwimplevel: RPC_C_IMP_LEVEL, pauthinfo: *const ::core::ffi::c_void, dwcapabilities: EOLE_AUTHENTICATION_CAPABILITIES) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CoSetProxyBlanket(pproxy: *mut ::core::ffi::c_void, dwauthnsvc: u32, dwauthzsvc: u32, pserverprincname: super::super::Foundation::PWSTR, dwauthnlevel: RPC_C_AUTHN_LEVEL, dwimplevel: RPC_C_IMP_LEVEL, pauthinfo: *const ::core::ffi::c_void, dwcapabilities: EOLE_AUTHENTICATION_CAPABILITIES) -> ::windows::core::HRESULT;
        }
        CoSetProxyBlanket(pproxy.into_param().abi(), ::core::mem::transmute(dwauthnsvc), ::core::mem::transmute(dwauthzsvc), pserverprincname.into_param().abi(), ::core::mem::transmute(dwauthnlevel), ::core::mem::transmute(dwimplevel), ::core::mem::transmute(pauthinfo), ::core::mem::transmute(dwcapabilities)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_System_Com'*"]
#[inline]
pub unsafe fn CoSuspendClassObjects() -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CoSuspendClassObjects() -> ::windows::core::HRESULT;
        }
        CoSuspendClassObjects().ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_System_Com'*"]
#[inline]
pub unsafe fn CoSwitchCallContext<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>>(pnewobject: Param0) -> ::windows::core::Result<::windows::core::IUnknown> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CoSwitchCallContext(pnewobject: *mut ::core::ffi::c_void, ppoldobject: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT;
        }
        let mut result__: *mut ::core::ffi::c_void = ::core::mem::zeroed();
        CoSwitchCallContext(pnewobject.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::IUnknown>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_System_Com'*"]
#[inline]
pub unsafe fn CoTaskMemAlloc(cb: usize) -> *mut ::core::ffi::c_void {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CoTaskMemAlloc(cb: usize) -> *mut ::core::ffi::c_void;
        }
        ::core::mem::transmute(CoTaskMemAlloc(::core::mem::transmute(cb)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_System_Com'*"]
#[inline]
pub unsafe fn CoTaskMemFree(pv: *const ::core::ffi::c_void) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CoTaskMemFree(pv: *const ::core::ffi::c_void);
        }
        CoTaskMemFree(::core::mem::transmute(pv))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_System_Com'*"]
#[inline]
pub unsafe fn CoTaskMemRealloc(pv: *const ::core::ffi::c_void, cb: usize) -> *mut ::core::ffi::c_void {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CoTaskMemRealloc(pv: *const ::core::ffi::c_void, cb: usize) -> *mut ::core::ffi::c_void;
        }
        ::core::mem::transmute(CoTaskMemRealloc(::core::mem::transmute(pv), ::core::mem::transmute(cb)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_System_Com'*"]
#[inline]
pub unsafe fn CoTestCancel() -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CoTestCancel() -> ::windows::core::HRESULT;
        }
        CoTestCancel().ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_System_Com'*"]
#[inline]
pub unsafe fn CoTreatAsClass(clsidold: *const ::windows::core::GUID, clsidnew: *const ::windows::core::GUID) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CoTreatAsClass(clsidold: *const ::windows::core::GUID, clsidnew: *const ::windows::core::GUID) -> ::windows::core::HRESULT;
        }
        CoTreatAsClass(::core::mem::transmute(clsidold), ::core::mem::transmute(clsidnew)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_System_Com'*"]
#[inline]
pub unsafe fn CoUninitialize() {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CoUninitialize();
        }
        CoUninitialize()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_System_Com', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CoWaitForMultipleHandles(dwflags: u32, dwtimeout: u32, chandles: u32, phandles: *const super::super::Foundation::HANDLE) -> ::windows::core::Result<u32> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CoWaitForMultipleHandles(dwflags: u32, dwtimeout: u32, chandles: u32, phandles: *const super::super::Foundation::HANDLE, lpdwindex: *mut u32) -> ::windows::core::HRESULT;
        }
        let mut result__: u32 = ::core::mem::zeroed();
        CoWaitForMultipleHandles(::core::mem::transmute(dwflags), ::core::mem::transmute(dwtimeout), ::core::mem::transmute(chandles), ::core::mem::transmute(phandles), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_System_Com', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CoWaitForMultipleObjects(dwflags: u32, dwtimeout: u32, chandles: u32, phandles: *const super::super::Foundation::HANDLE) -> ::windows::core::Result<u32> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CoWaitForMultipleObjects(dwflags: u32, dwtimeout: u32, chandles: u32, phandles: *const super::super::Foundation::HANDLE, lpdwindex: *mut u32) -> ::windows::core::HRESULT;
        }
        let mut result__: u32 = ::core::mem::zeroed();
        CoWaitForMultipleObjects(::core::mem::transmute(dwflags), ::core::mem::transmute(dwtimeout), ::core::mem::transmute(chandles), ::core::mem::transmute(phandles), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[repr(C)]
#[doc = "*Required features: 'Win32_System_Com'*"]
pub struct ComCallData {
    pub dwDispid: u32,
    pub dwReserved: u32,
    pub pUserDefined: *mut ::core::ffi::c_void,
}
impl ::core::marker::Copy for ComCallData {}
impl ::core::clone::Clone for ComCallData {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for ComCallData {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ComCallData").field("dwDispid", &self.dwDispid).field("dwReserved", &self.dwReserved).field("pUserDefined", &self.pUserDefined).finish()
    }
}
unsafe impl ::windows::core::Abi for ComCallData {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for ComCallData {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<ComCallData>()) == 0 }
    }
}
impl ::core::cmp::Eq for ComCallData {}
impl ::core::default::Default for ComCallData {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: 'Win32_System_Com'*"]
#[inline]
pub unsafe fn CreateAntiMoniker() -> ::windows::core::Result<IMoniker> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CreateAntiMoniker(ppmk: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT;
        }
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        CreateAntiMoniker(::core::mem::transmute(&mut result__)).from_abi::<IMoniker>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_System_Com'*"]
#[inline]
pub unsafe fn CreateBindCtx(reserved: u32) -> ::windows::core::Result<IBindCtx> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CreateBindCtx(reserved: u32, ppbc: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT;
        }
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        CreateBindCtx(::core::mem::transmute(reserved), ::core::mem::transmute(&mut result__)).from_abi::<IBindCtx>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_System_Com'*"]
#[inline]
pub unsafe fn CreateClassMoniker(rclsid: *const ::windows::core::GUID) -> ::windows::core::Result<IMoniker> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CreateClassMoniker(rclsid: *const ::windows::core::GUID, ppmk: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT;
        }
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        CreateClassMoniker(::core::mem::transmute(rclsid), ::core::mem::transmute(&mut result__)).from_abi::<IMoniker>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_System_Com'*"]
#[inline]
pub unsafe fn CreateDataAdviseHolder() -> ::windows::core::Result<IDataAdviseHolder> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CreateDataAdviseHolder(ppdaholder: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT;
        }
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        CreateDataAdviseHolder(::core::mem::transmute(&mut result__)).from_abi::<IDataAdviseHolder>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_System_Com'*"]
#[inline]
pub unsafe fn CreateDataCache<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>>(punkouter: Param0, rclsid: *const ::windows::core::GUID, iid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CreateDataCache(punkouter: *mut ::core::ffi::c_void, rclsid: *const ::windows::core::GUID, iid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT;
        }
        CreateDataCache(punkouter.into_param().abi(), ::core::mem::transmute(rclsid), ::core::mem::transmute(iid), ::core::mem::transmute(ppv)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_System_Com', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CreateFileMoniker<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(lpszpathname: Param0) -> ::windows::core::Result<IMoniker> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CreateFileMoniker(lpszpathname: super::super::Foundation::PWSTR, ppmk: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT;
        }
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        CreateFileMoniker(lpszpathname.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IMoniker>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_System_Com'*"]
#[inline]
pub unsafe fn CreateGenericComposite<'a, Param0: ::windows::core::IntoParam<'a, IMoniker>, Param1: ::windows::core::IntoParam<'a, IMoniker>>(pmkfirst: Param0, pmkrest: Param1) -> ::windows::core::Result<IMoniker> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CreateGenericComposite(pmkfirst: ::windows::core::RawPtr, pmkrest: ::windows::core::RawPtr, ppmkcomposite: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT;
        }
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        CreateGenericComposite(pmkfirst.into_param().abi(), pmkrest.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IMoniker>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_System_Com'*"]
#[inline]
pub unsafe fn CreateIUriBuilder<'a, Param0: ::windows::core::IntoParam<'a, IUri>>(piuri: Param0, dwflags: u32, dwreserved: usize) -> ::windows::core::Result<IUriBuilder> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CreateIUriBuilder(piuri: ::windows::core::RawPtr, dwflags: u32, dwreserved: usize, ppiuribuilder: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT;
        }
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        CreateIUriBuilder(piuri.into_param().abi(), ::core::mem::transmute(dwflags), ::core::mem::transmute(dwreserved), ::core::mem::transmute(&mut result__)).from_abi::<IUriBuilder>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_System_Com', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CreateItemMoniker<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(lpszdelim: Param0, lpszitem: Param1) -> ::windows::core::Result<IMoniker> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CreateItemMoniker(lpszdelim: super::super::Foundation::PWSTR, lpszitem: super::super::Foundation::PWSTR, ppmk: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT;
        }
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        CreateItemMoniker(lpszdelim.into_param().abi(), lpszitem.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IMoniker>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_System_Com'*"]
#[inline]
pub unsafe fn CreateObjrefMoniker<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>>(punk: Param0) -> ::windows::core::Result<IMoniker> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CreateObjrefMoniker(punk: *mut ::core::ffi::c_void, ppmk: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT;
        }
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        CreateObjrefMoniker(punk.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IMoniker>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_System_Com'*"]
#[inline]
pub unsafe fn CreatePointerMoniker<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>>(punk: Param0) -> ::windows::core::Result<IMoniker> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CreatePointerMoniker(punk: *mut ::core::ffi::c_void, ppmk: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT;
        }
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        CreatePointerMoniker(punk.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IMoniker>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_System_Com', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CreateStdProgressIndicator<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HWND>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param2: ::windows::core::IntoParam<'a, IBindStatusCallback>>(hwndparent: Param0, psztitle: Param1, pibsccaller: Param2) -> ::windows::core::Result<IBindStatusCallback> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CreateStdProgressIndicator(hwndparent: super::super::Foundation::HWND, psztitle: super::super::Foundation::PWSTR, pibsccaller: ::windows::core::RawPtr, ppibsc: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT;
        }
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        CreateStdProgressIndicator(hwndparent.into_param().abi(), psztitle.into_param().abi(), pibsccaller.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IBindStatusCallback>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_System_Com', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CreateUri<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(pwzuri: Param0, dwflags: URI_CREATE_FLAGS, dwreserved: usize) -> ::windows::core::Result<IUri> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CreateUri(pwzuri: super::super::Foundation::PWSTR, dwflags: URI_CREATE_FLAGS, dwreserved: usize, ppuri: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT;
        }
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        CreateUri(pwzuri.into_param().abi(), ::core::mem::transmute(dwflags), ::core::mem::transmute(dwreserved), ::core::mem::transmute(&mut result__)).from_abi::<IUri>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_System_Com', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CreateUriFromMultiByteString<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PSTR>>(pszansiinputuri: Param0, dwencodingflags: u32, dwcodepage: u32, dwcreateflags: u32, dwreserved: usize) -> ::windows::core::Result<IUri> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CreateUriFromMultiByteString(pszansiinputuri: super::super::Foundation::PSTR, dwencodingflags: u32, dwcodepage: u32, dwcreateflags: u32, dwreserved: usize, ppuri: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT;
        }
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        CreateUriFromMultiByteString(pszansiinputuri.into_param().abi(), ::core::mem::transmute(dwencodingflags), ::core::mem::transmute(dwcodepage), ::core::mem::transmute(dwcreateflags), ::core::mem::transmute(dwreserved), ::core::mem::transmute(&mut result__)).from_abi::<IUri>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_System_Com', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CreateUriWithFragment<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(pwzuri: Param0, pwzfragment: Param1, dwflags: u32, dwreserved: usize) -> ::windows::core::Result<IUri> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CreateUriWithFragment(pwzuri: super::super::Foundation::PWSTR, pwzfragment: super::super::Foundation::PWSTR, dwflags: u32, dwreserved: usize, ppuri: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT;
        }
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        CreateUriWithFragment(pwzuri.into_param().abi(), pwzfragment.into_param().abi(), ::core::mem::transmute(dwflags), ::core::mem::transmute(dwreserved), ::core::mem::transmute(&mut result__)).from_abi::<IUri>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_System_Com'*"]
pub type DATADIR = i32;
#[doc = "*Required features: 'Win32_System_Com'*"]
pub const DATADIR_GET: DATADIR = 1i32;
#[doc = "*Required features: 'Win32_System_Com'*"]
pub const DATADIR_SET: DATADIR = 2i32;
#[doc = "*Required features: 'Win32_System_Com'*"]
pub const DCOMSCM_ACTIVATION_DISALLOW_UNSECURE_CALL: u32 = 2u32;
#[doc = "*Required features: 'Win32_System_Com'*"]
pub const DCOMSCM_ACTIVATION_USE_ALL_AUTHNSERVICES: u32 = 1u32;
#[doc = "*Required features: 'Win32_System_Com'*"]
pub const DCOMSCM_PING_DISALLOW_UNSECURE_CALL: u32 = 32u32;
#[doc = "*Required features: 'Win32_System_Com'*"]
pub const DCOMSCM_PING_USE_MID_AUTHNSERVICE: u32 = 16u32;
#[doc = "*Required features: 'Win32_System_Com'*"]
pub const DCOMSCM_RESOLVE_DISALLOW_UNSECURE_CALL: u32 = 8u32;
#[doc = "*Required features: 'Win32_System_Com'*"]
pub const DCOMSCM_RESOLVE_USE_ALL_AUTHNSERVICES: u32 = 4u32;
#[doc = "*Required features: 'Win32_System_Com'*"]
pub type DCOM_CALL_STATE = i32;
#[doc = "*Required features: 'Win32_System_Com'*"]
pub const DCOM_NONE: DCOM_CALL_STATE = 0i32;
#[doc = "*Required features: 'Win32_System_Com'*"]
pub const DCOM_CALL_COMPLETE: DCOM_CALL_STATE = 1i32;
#[doc = "*Required features: 'Win32_System_Com'*"]
pub const DCOM_CALL_CANCELED: DCOM_CALL_STATE = 2i32;
#[doc = "*Required features: 'Win32_System_Com'*"]
pub type DESCKIND = i32;
#[doc = "*Required features: 'Win32_System_Com'*"]
pub const DESCKIND_NONE: DESCKIND = 0i32;
#[doc = "*Required features: 'Win32_System_Com'*"]
pub const DESCKIND_FUNCDESC: DESCKIND = 1i32;
#[doc = "*Required features: 'Win32_System_Com'*"]
pub const DESCKIND_VARDESC: DESCKIND = 2i32;
#[doc = "*Required features: 'Win32_System_Com'*"]
pub const DESCKIND_TYPECOMP: DESCKIND = 3i32;
#[doc = "*Required features: 'Win32_System_Com'*"]
pub const DESCKIND_IMPLICITAPPOBJ: DESCKIND = 4i32;
#[doc = "*Required features: 'Win32_System_Com'*"]
pub const DESCKIND_MAX: DESCKIND = 5i32;
#[repr(C)]
#[doc = "*Required features: 'Win32_System_Com', 'Win32_Foundation', 'Win32_System_Ole'*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
pub struct DISPPARAMS {
    pub rgvarg: *mut VARIANT,
    pub rgdispidNamedArgs: *mut i32,
    pub cArgs: u32,
    pub cNamedArgs: u32,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
impl ::core::marker::Copy for DISPPARAMS {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
impl ::core::clone::Clone for DISPPARAMS {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
impl ::core::fmt::Debug for DISPPARAMS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DISPPARAMS").field("rgvarg", &self.rgvarg).field("rgdispidNamedArgs", &self.rgdispidNamedArgs).field("cArgs", &self.cArgs).field("cNamedArgs", &self.cNamedArgs).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
unsafe impl ::windows::core::Abi for DISPPARAMS {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
impl ::core::cmp::PartialEq for DISPPARAMS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DISPPARAMS>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
impl ::core::cmp::Eq for DISPPARAMS {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
impl ::core::default::Default for DISPPARAMS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: 'Win32_System_Com'*"]
pub const DMUS_ERRBASE: u32 = 4096u32;
#[doc = "*Required features: 'Win32_System_Com'*"]
pub type DVASPECT = i32;
#[doc = "*Required features: 'Win32_System_Com'*"]
pub const DVASPECT_CONTENT: DVASPECT = 1i32;
#[doc = "*Required features: 'Win32_System_Com'*"]
pub const DVASPECT_THUMBNAIL: DVASPECT = 2i32;
#[doc = "*Required features: 'Win32_System_Com'*"]
pub const DVASPECT_ICON: DVASPECT = 4i32;
#[doc = "*Required features: 'Win32_System_Com'*"]
pub const DVASPECT_DOCPRINT: DVASPECT = 8i32;
#[repr(C)]
#[doc = "*Required features: 'Win32_System_Com'*"]
pub struct DVTARGETDEVICE {
    pub tdSize: u32,
    pub tdDriverNameOffset: u16,
    pub tdDeviceNameOffset: u16,
    pub tdPortNameOffset: u16,
    pub tdExtDevmodeOffset: u16,
    pub tdData: [u8; 1],
}
impl ::core::marker::Copy for DVTARGETDEVICE {}
impl ::core::clone::Clone for DVTARGETDEVICE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DVTARGETDEVICE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DVTARGETDEVICE").field("tdSize", &self.tdSize).field("tdDriverNameOffset", &self.tdDriverNameOffset).field("tdDeviceNameOffset", &self.tdDeviceNameOffset).field("tdPortNameOffset", &self.tdPortNameOffset).field("tdExtDevmodeOffset", &self.tdExtDevmodeOffset).field("tdData", &self.tdData).finish()
    }
}
unsafe impl ::windows::core::Abi for DVTARGETDEVICE {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DVTARGETDEVICE {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DVTARGETDEVICE>()) == 0 }
    }
}
impl ::core::cmp::Eq for DVTARGETDEVICE {}
impl ::core::default::Default for DVTARGETDEVICE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_System_Com'*"]
pub struct DWORD_BLOB {
    pub clSize: u32,
    pub alData: [u32; 1],
}
impl ::core::marker::Copy for DWORD_BLOB {}
impl ::core::clone::Clone for DWORD_BLOB {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DWORD_BLOB {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DWORD_BLOB").field("clSize", &self.clSize).field("alData", &self.alData).finish()
    }
}
unsafe impl ::windows::core::Abi for DWORD_BLOB {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DWORD_BLOB {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DWORD_BLOB>()) == 0 }
    }
}
impl ::core::cmp::Eq for DWORD_BLOB {}
impl ::core::default::Default for DWORD_BLOB {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: 'Win32_System_Com'*"]
#[inline]
pub unsafe fn DcomChannelSetHResult(pvreserved: *const ::core::ffi::c_void, pulreserved: *const u32, appshr: ::windows::core::HRESULT) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DcomChannelSetHResult(pvreserved: *const ::core::ffi::c_void, pulreserved: *const u32, appshr: ::windows::core::HRESULT) -> ::windows::core::HRESULT;
        }
        DcomChannelSetHResult(::core::mem::transmute(pvreserved), ::core::mem::transmute(pulreserved), ::core::mem::transmute(appshr)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[repr(C)]
#[doc = "*Required features: 'Win32_System_Com', 'Win32_Foundation', 'Win32_System_Ole'*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
pub struct ELEMDESC {
    pub tdesc: TYPEDESC,
    pub Anonymous: ELEMDESC_0,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
impl ::core::marker::Copy for ELEMDESC {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
impl ::core::clone::Clone for ELEMDESC {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
unsafe impl ::windows::core::Abi for ELEMDESC {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
impl ::core::cmp::PartialEq for ELEMDESC {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<ELEMDESC>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
impl ::core::cmp::Eq for ELEMDESC {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
impl ::core::default::Default for ELEMDESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_System_Com', 'Win32_Foundation', 'Win32_System_Ole'*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
pub union ELEMDESC_0 {
    pub idldesc: IDLDESC,
    pub paramdesc: super::Ole::PARAMDESC,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
impl ::core::marker::Copy for ELEMDESC_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
impl ::core::clone::Clone for ELEMDESC_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
unsafe impl ::windows::core::Abi for ELEMDESC_0 {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
impl ::core::cmp::PartialEq for ELEMDESC_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<ELEMDESC_0>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
impl ::core::cmp::Eq for ELEMDESC_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
impl ::core::default::Default for ELEMDESC_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: 'Win32_System_Com'*"]
pub type EOLE_AUTHENTICATION_CAPABILITIES = i32;
#[doc = "*Required features: 'Win32_System_Com'*"]
pub const EOAC_NONE: EOLE_AUTHENTICATION_CAPABILITIES = 0i32;
#[doc = "*Required features: 'Win32_System_Com'*"]
pub const EOAC_MUTUAL_AUTH: EOLE_AUTHENTICATION_CAPABILITIES = 1i32;
#[doc = "*Required features: 'Win32_System_Com'*"]
pub const EOAC_STATIC_CLOAKING: EOLE_AUTHENTICATION_CAPABILITIES = 32i32;
#[doc = "*Required features: 'Win32_System_Com'*"]
pub const EOAC_DYNAMIC_CLOAKING: EOLE_AUTHENTICATION_CAPABILITIES = 64i32;
#[doc = "*Required features: 'Win32_System_Com'*"]
pub const EOAC_ANY_AUTHORITY: EOLE_AUTHENTICATION_CAPABILITIES = 128i32;
#[doc = "*Required features: 'Win32_System_Com'*"]
pub const EOAC_MAKE_FULLSIC: EOLE_AUTHENTICATION_CAPABILITIES = 256i32;
#[doc = "*Required features: 'Win32_System_Com'*"]
pub const EOAC_DEFAULT: EOLE_AUTHENTICATION_CAPABILITIES = 2048i32;
#[doc = "*Required features: 'Win32_System_Com'*"]
pub const EOAC_SECURE_REFS: EOLE_AUTHENTICATION_CAPABILITIES = 2i32;
#[doc = "*Required features: 'Win32_System_Com'*"]
pub const EOAC_ACCESS_CONTROL: EOLE_AUTHENTICATION_CAPABILITIES = 4i32;
#[doc = "*Required features: 'Win32_System_Com'*"]
pub const EOAC_APPID: EOLE_AUTHENTICATION_CAPABILITIES = 8i32;
#[doc = "*Required features: 'Win32_System_Com'*"]
pub const EOAC_DYNAMIC: EOLE_AUTHENTICATION_CAPABILITIES = 16i32;
#[doc = "*Required features: 'Win32_System_Com'*"]
pub const EOAC_REQUIRE_FULLSIC: EOLE_AUTHENTICATION_CAPABILITIES = 512i32;
#[doc = "*Required features: 'Win32_System_Com'*"]
pub const EOAC_AUTO_IMPERSONATE: EOLE_AUTHENTICATION_CAPABILITIES = 1024i32;
#[doc = "*Required features: 'Win32_System_Com'*"]
pub const EOAC_DISABLE_AAA: EOLE_AUTHENTICATION_CAPABILITIES = 4096i32;
#[doc = "*Required features: 'Win32_System_Com'*"]
pub const EOAC_NO_CUSTOM_MARSHAL: EOLE_AUTHENTICATION_CAPABILITIES = 8192i32;
#[doc = "*Required features: 'Win32_System_Com'*"]
pub const EOAC_RESERVED1: EOLE_AUTHENTICATION_CAPABILITIES = 16384i32;
#[repr(C)]
#[doc = "*Required features: 'Win32_System_Com', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct EXCEPINFO {
    pub wCode: u16,
    pub wReserved: u16,
    pub bstrSource: super::super::Foundation::BSTR,
    pub bstrDescription: super::super::Foundation::BSTR,
    pub bstrHelpFile: super::super::Foundation::BSTR,
    pub dwHelpContext: u32,
    pub pvReserved: *mut ::core::ffi::c_void,
    pub pfnDeferredFillIn: LPEXCEPFINO_DEFERRED_FILLIN,
    pub scode: i32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for EXCEPINFO {
    fn clone(&self) -> Self {
        Self {
            wCode: self.wCode,
            wReserved: self.wReserved,
            bstrSource: self.bstrSource.clone(),
            bstrDescription: self.bstrDescription.clone(),
            bstrHelpFile: self.bstrHelpFile.clone(),
            dwHelpContext: self.dwHelpContext,
            pvReserved: self.pvReserved,
            pfnDeferredFillIn: self.pfnDeferredFillIn,
            scode: self.scode,
        }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for EXCEPINFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("EXCEPINFO").field("wCode", &self.wCode).field("wReserved", &self.wReserved).field("bstrSource", &self.bstrSource).field("bstrDescription", &self.bstrDescription).field("bstrHelpFile", &self.bstrHelpFile).field("dwHelpContext", &self.dwHelpContext).field("pvReserved", &self.pvReserved).field("pfnDeferredFillIn", &self.pfnDeferredFillIn.map(|f| f as usize)).field("scode", &self.scode).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for EXCEPINFO {
    type Abi = ::core::mem::ManuallyDrop<Self>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for EXCEPINFO {
    fn eq(&self, other: &Self) -> bool {
        self.wCode == other.wCode && self.wReserved == other.wReserved && self.bstrSource == other.bstrSource && self.bstrDescription == other.bstrDescription && self.bstrHelpFile == other.bstrHelpFile && self.dwHelpContext == other.dwHelpContext && self.pvReserved == other.pvReserved && self.pfnDeferredFillIn.map(|f| f as usize) == other.pfnDeferredFillIn.map(|f| f as usize) && self.scode == other.scode
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for EXCEPINFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for EXCEPINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: 'Win32_System_Com'*"]
pub type EXTCONN = i32;
#[doc = "*Required features: 'Win32_System_Com'*"]
pub const EXTCONN_STRONG: EXTCONN = 1i32;
#[doc = "*Required features: 'Win32_System_Com'*"]
pub const EXTCONN_WEAK: EXTCONN = 2i32;
#[doc = "*Required features: 'Win32_System_Com'*"]
pub const EXTCONN_CALLABLE: EXTCONN = 4i32;
#[repr(C)]
#[doc = "*Required features: 'Win32_System_Com'*"]
pub struct FLAGGED_BYTE_BLOB {
    pub fFlags: u32,
    pub clSize: u32,
    pub abData: [u8; 1],
}
impl ::core::marker::Copy for FLAGGED_BYTE_BLOB {}
impl ::core::clone::Clone for FLAGGED_BYTE_BLOB {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for FLAGGED_BYTE_BLOB {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FLAGGED_BYTE_BLOB").field("fFlags", &self.fFlags).field("clSize", &self.clSize).field("abData", &self.abData).finish()
    }
}
unsafe impl ::windows::core::Abi for FLAGGED_BYTE_BLOB {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for FLAGGED_BYTE_BLOB {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<FLAGGED_BYTE_BLOB>()) == 0 }
    }
}
impl ::core::cmp::Eq for FLAGGED_BYTE_BLOB {}
impl ::core::default::Default for FLAGGED_BYTE_BLOB {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_System_Com'*"]
pub struct FLAGGED_WORD_BLOB {
    pub fFlags: u32,
    pub clSize: u32,
    pub asData: [u16; 1],
}
impl ::core::marker::Copy for FLAGGED_WORD_BLOB {}
impl ::core::clone::Clone for FLAGGED_WORD_BLOB {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for FLAGGED_WORD_BLOB {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FLAGGED_WORD_BLOB").field("fFlags", &self.fFlags).field("clSize", &self.clSize).field("asData", &self.asData).finish()
    }
}
unsafe impl ::windows::core::Abi for FLAGGED_WORD_BLOB {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for FLAGGED_WORD_BLOB {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<FLAGGED_WORD_BLOB>()) == 0 }
    }
}
impl ::core::cmp::Eq for FLAGGED_WORD_BLOB {}
impl ::core::default::Default for FLAGGED_WORD_BLOB {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_System_Com', 'Win32_Foundation', 'Win32_Graphics_Gdi', 'Win32_System_Com_StructuredStorage'*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
pub struct FLAG_STGMEDIUM {
    pub ContextFlags: i32,
    pub fPassOwnership: i32,
    pub Stgmed: STGMEDIUM,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
impl ::core::clone::Clone for FLAG_STGMEDIUM {
    fn clone(&self) -> Self {
        Self { ContextFlags: self.ContextFlags, fPassOwnership: self.fPassOwnership, Stgmed: self.Stgmed.clone() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
unsafe impl ::windows::core::Abi for FLAG_STGMEDIUM {
    type Abi = ::core::mem::ManuallyDrop<Self>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
impl ::core::cmp::PartialEq for FLAG_STGMEDIUM {
    fn eq(&self, other: &Self) -> bool {
        self.ContextFlags == other.ContextFlags && self.fPassOwnership == other.fPassOwnership && self.Stgmed == other.Stgmed
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
impl ::core::cmp::Eq for FLAG_STGMEDIUM {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
impl ::core::default::Default for FLAG_STGMEDIUM {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_System_Com'*"]
pub struct FORMATETC {
    pub cfFormat: u16,
    pub ptd: *mut DVTARGETDEVICE,
    pub dwAspect: u32,
    pub lindex: i32,
    pub tymed: u32,
}
impl ::core::marker::Copy for FORMATETC {}
impl ::core::clone::Clone for FORMATETC {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for FORMATETC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FORMATETC").field("cfFormat", &self.cfFormat).field("ptd", &self.ptd).field("dwAspect", &self.dwAspect).field("lindex", &self.lindex).field("tymed", &self.tymed).finish()
    }
}
unsafe impl ::windows::core::Abi for FORMATETC {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for FORMATETC {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<FORMATETC>()) == 0 }
    }
}
impl ::core::cmp::Eq for FORMATETC {}
impl ::core::default::Default for FORMATETC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_System_Com', 'Win32_Foundation', 'Win32_System_Ole'*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
pub struct FUNCDESC {
    pub memid: i32,
    pub lprgscode: *mut i32,
    pub lprgelemdescParam: *mut ELEMDESC,
    pub funckind: FUNCKIND,
    pub invkind: INVOKEKIND,
    pub callconv: CALLCONV,
    pub cParams: i16,
    pub cParamsOpt: i16,
    pub oVft: i16,
    pub cScodes: i16,
    pub elemdescFunc: ELEMDESC,
    pub wFuncFlags: u16,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
impl ::core::marker::Copy for FUNCDESC {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
impl ::core::clone::Clone for FUNCDESC {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
unsafe impl ::windows::core::Abi for FUNCDESC {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
impl ::core::cmp::PartialEq for FUNCDESC {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<FUNCDESC>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
impl ::core::cmp::Eq for FUNCDESC {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
impl ::core::default::Default for FUNCDESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: 'Win32_System_Com'*"]
pub type FUNCKIND = i32;
#[doc = "*Required features: 'Win32_System_Com'*"]
pub const FUNC_VIRTUAL: FUNCKIND = 0i32;
#[doc = "*Required features: 'Win32_System_Com'*"]
pub const FUNC_PUREVIRTUAL: FUNCKIND = 1i32;
#[doc = "*Required features: 'Win32_System_Com'*"]
pub const FUNC_NONVIRTUAL: FUNCKIND = 2i32;
#[doc = "*Required features: 'Win32_System_Com'*"]
pub const FUNC_STATIC: FUNCKIND = 3i32;
#[doc = "*Required features: 'Win32_System_Com'*"]
pub const FUNC_DISPATCH: FUNCKIND = 4i32;
#[repr(C)]
#[doc = "*Required features: 'Win32_System_Com', 'Win32_Graphics_Gdi', 'Win32_System_SystemServices'*"]
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_SystemServices"))]
pub struct GDI_OBJECT {
    pub ObjectType: u32,
    pub u: GDI_OBJECT_0,
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_SystemServices"))]
impl ::core::marker::Copy for GDI_OBJECT {}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_SystemServices"))]
impl ::core::clone::Clone for GDI_OBJECT {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_SystemServices"))]
unsafe impl ::windows::core::Abi for GDI_OBJECT {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_SystemServices"))]
impl ::core::cmp::PartialEq for GDI_OBJECT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<GDI_OBJECT>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_SystemServices"))]
impl ::core::cmp::Eq for GDI_OBJECT {}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_SystemServices"))]
impl ::core::default::Default for GDI_OBJECT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_System_Com', 'Win32_Graphics_Gdi', 'Win32_System_SystemServices'*"]
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_SystemServices"))]
pub union GDI_OBJECT_0 {
    pub hBitmap: *mut super::SystemServices::userHBITMAP,
    pub hPalette: *mut super::SystemServices::userHPALETTE,
    pub hGeneric: *mut super::SystemServices::userHGLOBAL,
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_SystemServices"))]
impl ::core::marker::Copy for GDI_OBJECT_0 {}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_SystemServices"))]
impl ::core::clone::Clone for GDI_OBJECT_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_SystemServices"))]
unsafe impl ::windows::core::Abi for GDI_OBJECT_0 {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_SystemServices"))]
impl ::core::cmp::PartialEq for GDI_OBJECT_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<GDI_OBJECT_0>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_SystemServices"))]
impl ::core::cmp::Eq for GDI_OBJECT_0 {}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_SystemServices"))]
impl ::core::default::Default for GDI_OBJECT_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: 'Win32_System_Com'*"]
pub type GLOBALOPT_EH_VALUES = i32;
#[doc = "*Required features: 'Win32_System_Com'*"]
pub const COMGLB_EXCEPTION_HANDLE: GLOBALOPT_EH_VALUES = 0i32;
#[doc = "*Required features: 'Win32_System_Com'*"]
pub const COMGLB_EXCEPTION_DONOT_HANDLE_FATAL: GLOBALOPT_EH_VALUES = 1i32;
#[doc = "*Required features: 'Win32_System_Com'*"]
pub const COMGLB_EXCEPTION_DONOT_HANDLE: GLOBALOPT_EH_VALUES = 1i32;
#[doc = "*Required features: 'Win32_System_Com'*"]
pub const COMGLB_EXCEPTION_DONOT_HANDLE_ANY: GLOBALOPT_EH_VALUES = 2i32;
#[doc = "*Required features: 'Win32_System_Com'*"]
pub type GLOBALOPT_PROPERTIES = i32;
#[doc = "*Required features: 'Win32_System_Com'*"]
pub const COMGLB_EXCEPTION_HANDLING: GLOBALOPT_PROPERTIES = 1i32;
#[doc = "*Required features: 'Win32_System_Com'*"]
pub const COMGLB_APPID: GLOBALOPT_PROPERTIES = 2i32;
#[doc = "*Required features: 'Win32_System_Com'*"]
pub const COMGLB_RPC_THREADPOOL_SETTING: GLOBALOPT_PROPERTIES = 3i32;
#[doc = "*Required features: 'Win32_System_Com'*"]
pub const COMGLB_RO_SETTINGS: GLOBALOPT_PROPERTIES = 4i32;
#[doc = "*Required features: 'Win32_System_Com'*"]
pub const COMGLB_UNMARSHALING_POLICY: GLOBALOPT_PROPERTIES = 5i32;
#[doc = "*Required features: 'Win32_System_Com'*"]
pub const COMGLB_PROPERTIES_RESERVED1: GLOBALOPT_PROPERTIES = 6i32;
#[doc = "*Required features: 'Win32_System_Com'*"]
pub const COMGLB_PROPERTIES_RESERVED2: GLOBALOPT_PROPERTIES = 7i32;
#[doc = "*Required features: 'Win32_System_Com'*"]
pub const COMGLB_PROPERTIES_RESERVED3: GLOBALOPT_PROPERTIES = 8i32;
#[doc = "*Required features: 'Win32_System_Com'*"]
pub type GLOBALOPT_RO_FLAGS = i32;
#[doc = "*Required features: 'Win32_System_Com'*"]
pub const COMGLB_STA_MODALLOOP_REMOVE_TOUCH_MESSAGES: GLOBALOPT_RO_FLAGS = 1i32;
#[doc = "*Required features: 'Win32_System_Com'*"]
pub const COMGLB_STA_MODALLOOP_SHARED_QUEUE_REMOVE_INPUT_MESSAGES: GLOBALOPT_RO_FLAGS = 2i32;
#[doc = "*Required features: 'Win32_System_Com'*"]
pub const COMGLB_STA_MODALLOOP_SHARED_QUEUE_DONOT_REMOVE_INPUT_MESSAGES: GLOBALOPT_RO_FLAGS = 4i32;
#[doc = "*Required features: 'Win32_System_Com'*"]
pub const COMGLB_FAST_RUNDOWN: GLOBALOPT_RO_FLAGS = 8i32;
#[doc = "*Required features: 'Win32_System_Com'*"]
pub const COMGLB_RESERVED1: GLOBALOPT_RO_FLAGS = 16i32;
#[doc = "*Required features: 'Win32_System_Com'*"]
pub const COMGLB_RESERVED2: GLOBALOPT_RO_FLAGS = 32i32;
#[doc = "*Required features: 'Win32_System_Com'*"]
pub const COMGLB_RESERVED3: GLOBALOPT_RO_FLAGS = 64i32;
#[doc = "*Required features: 'Win32_System_Com'*"]
pub const COMGLB_STA_MODALLOOP_SHARED_QUEUE_REORDER_POINTER_MESSAGES: GLOBALOPT_RO_FLAGS = 128i32;
#[doc = "*Required features: 'Win32_System_Com'*"]
pub const COMGLB_RESERVED4: GLOBALOPT_RO_FLAGS = 256i32;
#[doc = "*Required features: 'Win32_System_Com'*"]
pub const COMGLB_RESERVED5: GLOBALOPT_RO_FLAGS = 512i32;
#[doc = "*Required features: 'Win32_System_Com'*"]
pub const COMGLB_RESERVED6: GLOBALOPT_RO_FLAGS = 1024i32;
#[doc = "*Required features: 'Win32_System_Com'*"]
pub type GLOBALOPT_RPCTP_VALUES = i32;
#[doc = "*Required features: 'Win32_System_Com'*"]
pub const COMGLB_RPC_THREADPOOL_SETTING_DEFAULT_POOL: GLOBALOPT_RPCTP_VALUES = 0i32;
#[doc = "*Required features: 'Win32_System_Com'*"]
pub const COMGLB_RPC_THREADPOOL_SETTING_PRIVATE_POOL: GLOBALOPT_RPCTP_VALUES = 1i32;
#[doc = "*Required features: 'Win32_System_Com'*"]
pub type GLOBALOPT_UNMARSHALING_POLICY_VALUES = i32;
#[doc = "*Required features: 'Win32_System_Com'*"]
pub const COMGLB_UNMARSHALING_POLICY_NORMAL: GLOBALOPT_UNMARSHALING_POLICY_VALUES = 0i32;
#[doc = "*Required features: 'Win32_System_Com'*"]
pub const COMGLB_UNMARSHALING_POLICY_STRONG: GLOBALOPT_UNMARSHALING_POLICY_VALUES = 1i32;
#[doc = "*Required features: 'Win32_System_Com'*"]
pub const COMGLB_UNMARSHALING_POLICY_HYBRID: GLOBALOPT_UNMARSHALING_POLICY_VALUES = 2i32;
#[doc = "*Required features: 'Win32_System_Com', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetClassFile<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(szfilename: Param0) -> ::windows::core::Result<::windows::core::GUID> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetClassFile(szfilename: super::super::Foundation::PWSTR, pclsid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT;
        }
        let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
        GetClassFile(szfilename.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::GUID>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_System_Com'*"]
#[inline]
pub unsafe fn GetErrorInfo(dwreserved: u32) -> ::windows::core::Result<IErrorInfo> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetErrorInfo(dwreserved: u32, pperrinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT;
        }
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        GetErrorInfo(::core::mem::transmute(dwreserved), ::core::mem::transmute(&mut result__)).from_abi::<IErrorInfo>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_System_Com'*"]
#[inline]
pub unsafe fn GetRunningObjectTable(reserved: u32) -> ::windows::core::Result<IRunningObjectTable> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetRunningObjectTable(reserved: u32, pprot: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT;
        }
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        GetRunningObjectTable(::core::mem::transmute(reserved), ::core::mem::transmute(&mut result__)).from_abi::<IRunningObjectTable>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[repr(C)]
#[doc = "*Required features: 'Win32_System_Com'*"]
pub struct HYPER_SIZEDARR {
    pub clSize: u32,
    pub pData: *mut i64,
}
impl ::core::marker::Copy for HYPER_SIZEDARR {}
impl ::core::clone::Clone for HYPER_SIZEDARR {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for HYPER_SIZEDARR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("HYPER_SIZEDARR").field("clSize", &self.clSize).field("pData", &self.pData).finish()
    }
}
unsafe impl ::windows::core::Abi for HYPER_SIZEDARR {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for HYPER_SIZEDARR {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<HYPER_SIZEDARR>()) == 0 }
    }
}
impl ::core::cmp::Eq for HYPER_SIZEDARR {}
impl ::core::default::Default for HYPER_SIZEDARR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: 'Win32_System_Com'*"]
#[repr(transparent)]
pub struct IActivationFilter(::windows::core::IUnknown);
impl IActivationFilter {
    #[doc = "*Required features: 'Win32_System_Com'*"]
    pub unsafe fn HandleActivation(&self, dwactivationtype: u32, rclsid: *const ::windows::core::GUID) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwactivationtype), ::core::mem::transmute(rclsid), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::GUID>(result__)
    }
}
impl ::core::convert::From<IActivationFilter> for ::windows::core::IUnknown {
    fn from(value: IActivationFilter) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IActivationFilter> for ::windows::core::IUnknown {
    fn from(value: &IActivationFilter) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IActivationFilter {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IActivationFilter {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IActivationFilter {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IActivationFilter {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IActivationFilter {}
impl ::core::fmt::Debug for IActivationFilter {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IActivationFilter").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IActivationFilter {
    type Vtable = IActivationFilterVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x00000017_0000_0000_c000_000000000046);
}
#[repr(C)]
#[doc(hidden)]
pub struct IActivationFilterVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwactivationtype: u32, rclsid: *const ::windows::core::GUID, preplacementclsid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: 'Win32_System_Com'*"]
#[repr(transparent)]
pub struct IAddrExclusionControl(::windows::core::IUnknown);
impl IAddrExclusionControl {
    #[doc = "*Required features: 'Win32_System_Com'*"]
    pub unsafe fn GetCurrentAddrExclusionList(&self, riid: *const ::windows::core::GUID, ppenumerator: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(riid), ::core::mem::transmute(ppenumerator)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Com'*"]
    pub unsafe fn UpdateAddrExclusionList<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>>(&self, penumerator: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), penumerator.into_param().abi()).ok()
    }
}
impl ::core::convert::From<IAddrExclusionControl> for ::windows::core::IUnknown {
    fn from(value: IAddrExclusionControl) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IAddrExclusionControl> for ::windows::core::IUnknown {
    fn from(value: &IAddrExclusionControl) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IAddrExclusionControl {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IAddrExclusionControl {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IAddrExclusionControl {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IAddrExclusionControl {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAddrExclusionControl {}
impl ::core::fmt::Debug for IAddrExclusionControl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAddrExclusionControl").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IAddrExclusionControl {
    type Vtable = IAddrExclusionControlVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x00000148_0000_0000_c000_000000000046);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAddrExclusionControlVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, ppenumerator: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, penumerator: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: 'Win32_System_Com'*"]
#[repr(transparent)]
pub struct IAddrTrackingControl(::windows::core::IUnknown);
impl IAddrTrackingControl {
    #[doc = "*Required features: 'Win32_System_Com'*"]
    pub unsafe fn EnableCOMDynamicAddrTracking(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Com'*"]
    pub unsafe fn DisableCOMDynamicAddrTracking(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self)).ok()
    }
}
impl ::core::convert::From<IAddrTrackingControl> for ::windows::core::IUnknown {
    fn from(value: IAddrTrackingControl) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IAddrTrackingControl> for ::windows::core::IUnknown {
    fn from(value: &IAddrTrackingControl) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IAddrTrackingControl {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IAddrTrackingControl {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IAddrTrackingControl {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IAddrTrackingControl {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAddrTrackingControl {}
impl ::core::fmt::Debug for IAddrTrackingControl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAddrTrackingControl").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IAddrTrackingControl {
    type Vtable = IAddrTrackingControlVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x00000147_0000_0000_c000_000000000046);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAddrTrackingControlVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: 'Win32_System_Com'*"]
#[repr(transparent)]
pub struct IAdviseSink(::windows::core::IUnknown);
impl IAdviseSink {
    #[doc = "*Required features: 'Win32_System_Com', 'Win32_Foundation', 'Win32_Graphics_Gdi', 'Win32_System_Com_StructuredStorage'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
    pub unsafe fn OnDataChange(&self, pformatetc: *const FORMATETC, pstgmed: *const STGMEDIUM) {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(pformatetc), ::core::mem::transmute(pstgmed))
    }
    #[doc = "*Required features: 'Win32_System_Com'*"]
    pub unsafe fn OnViewChange(&self, dwaspect: u32, lindex: i32) {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwaspect), ::core::mem::transmute(lindex))
    }
    #[doc = "*Required features: 'Win32_System_Com'*"]
    pub unsafe fn OnRename<'a, Param0: ::windows::core::IntoParam<'a, IMoniker>>(&self, pmk: Param0) {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), pmk.into_param().abi())
    }
    #[doc = "*Required features: 'Win32_System_Com'*"]
    pub unsafe fn OnSave(&self) {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self))
    }
    #[doc = "*Required features: 'Win32_System_Com'*"]
    pub unsafe fn OnClose(&self) {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self))
    }
}
impl ::core::convert::From<IAdviseSink> for ::windows::core::IUnknown {
    fn from(value: IAdviseSink) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IAdviseSink> for ::windows::core::IUnknown {
    fn from(value: &IAdviseSink) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IAdviseSink {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IAdviseSink {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IAdviseSink {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IAdviseSink {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAdviseSink {}
impl ::core::fmt::Debug for IAdviseSink {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAdviseSink").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IAdviseSink {
    type Vtable = IAdviseSinkVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0000010f_0000_0000_c000_000000000046);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAdviseSinkVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pformatetc: *const FORMATETC, pstgmed: *const STGMEDIUM),
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage")))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwaspect: u32, lindex: i32),
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pmk: ::windows::core::RawPtr),
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void),
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void),
);
#[doc = "*Required features: 'Win32_System_Com'*"]
#[repr(transparent)]
pub struct IAdviseSink2(::windows::core::IUnknown);
impl IAdviseSink2 {
    #[doc = "*Required features: 'Win32_System_Com', 'Win32_Foundation', 'Win32_Graphics_Gdi', 'Win32_System_Com_StructuredStorage'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
    pub unsafe fn OnDataChange(&self, pformatetc: *const FORMATETC, pstgmed: *const STGMEDIUM) {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(pformatetc), ::core::mem::transmute(pstgmed))
    }
    #[doc = "*Required features: 'Win32_System_Com'*"]
    pub unsafe fn OnViewChange(&self, dwaspect: u32, lindex: i32) {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwaspect), ::core::mem::transmute(lindex))
    }
    #[doc = "*Required features: 'Win32_System_Com'*"]
    pub unsafe fn OnRename<'a, Param0: ::windows::core::IntoParam<'a, IMoniker>>(&self, pmk: Param0) {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), pmk.into_param().abi())
    }
    #[doc = "*Required features: 'Win32_System_Com'*"]
    pub unsafe fn OnSave(&self) {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self))
    }
    #[doc = "*Required features: 'Win32_System_Com'*"]
    pub unsafe fn OnClose(&self) {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self))
    }
    #[doc = "*Required features: 'Win32_System_Com'*"]
    pub unsafe fn OnLinkSrcChange<'a, Param0: ::windows::core::IntoParam<'a, IMoniker>>(&self, pmk: Param0) {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), pmk.into_param().abi())
    }
}
impl ::core::convert::From<IAdviseSink2> for IAdviseSink {
    fn from(value: IAdviseSink2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IAdviseSink2> for IAdviseSink {
    fn from(value: &IAdviseSink2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IAdviseSink> for IAdviseSink2 {
    fn into_param(self) -> ::windows::core::Param<'a, IAdviseSink> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IAdviseSink> for &IAdviseSink2 {
    fn into_param(self) -> ::windows::core::Param<'a, IAdviseSink> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IAdviseSink2> for ::windows::core::IUnknown {
    fn from(value: IAdviseSink2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IAdviseSink2> for ::windows::core::IUnknown {
    fn from(value: &IAdviseSink2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IAdviseSink2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IAdviseSink2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IAdviseSink2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IAdviseSink2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAdviseSink2 {}
impl ::core::fmt::Debug for IAdviseSink2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAdviseSink2").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IAdviseSink2 {
    type Vtable = IAdviseSink2Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x00000125_0000_0000_c000_000000000046);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAdviseSink2Vtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pformatetc: *const FORMATETC, pstgmed: *const STGMEDIUM),
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage")))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwaspect: u32, lindex: i32),
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pmk: ::windows::core::RawPtr),
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void),
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void),
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pmk: ::windows::core::RawPtr),
);
#[doc = "*Required features: 'Win32_System_Com'*"]
#[repr(transparent)]
pub struct IAgileObject(::windows::core::IUnknown);
impl IAgileObject {}
impl ::core::convert::From<IAgileObject> for ::windows::core::IUnknown {
    fn from(value: IAgileObject) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IAgileObject> for ::windows::core::IUnknown {
    fn from(value: &IAgileObject) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IAgileObject {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IAgileObject {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IAgileObject {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IAgileObject {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAgileObject {}
impl ::core::fmt::Debug for IAgileObject {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAgileObject").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IAgileObject {
    type Vtable = IAgileObjectVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x94ea2b94_e9cc_49e0_c0ff_ee64ca8f5b90);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAgileObjectVtbl(pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT, pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32, pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32);
#[doc = "*Required features: 'Win32_System_Com'*"]
#[repr(transparent)]
pub struct IAsyncManager(::windows::core::IUnknown);
impl IAsyncManager {
    #[doc = "*Required features: 'Win32_System_Com'*"]
    pub unsafe fn CompleteCall(&self, result: ::windows::core::HRESULT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(result)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Com'*"]
    pub unsafe fn GetCallContext(&self, riid: *const ::windows::core::GUID, pinterface: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(riid), ::core::mem::transmute(pinterface)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Com'*"]
    pub unsafe fn GetState(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
}
impl ::core::convert::From<IAsyncManager> for ::windows::core::IUnknown {
    fn from(value: IAsyncManager) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IAsyncManager> for ::windows::core::IUnknown {
    fn from(value: &IAsyncManager) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IAsyncManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IAsyncManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IAsyncManager {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IAsyncManager {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAsyncManager {}
impl ::core::fmt::Debug for IAsyncManager {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAsyncManager").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IAsyncManager {
    type Vtable = IAsyncManagerVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0000002a_0000_0000_c000_000000000046);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAsyncManagerVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result: ::windows::core::HRESULT) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, pinterface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pulstateflags: *mut u32) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: 'Win32_System_Com'*"]
#[repr(transparent)]
pub struct IAsyncRpcChannelBuffer(::windows::core::IUnknown);
impl IAsyncRpcChannelBuffer {
    #[doc = "*Required features: 'Win32_System_Com'*"]
    pub unsafe fn GetBuffer(&self, pmessage: *mut RPCOLEMESSAGE, riid: *const ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(pmessage), ::core::mem::transmute(riid)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Com'*"]
    pub unsafe fn SendReceive(&self, pmessage: *mut RPCOLEMESSAGE, pstatus: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(pmessage), ::core::mem::transmute(pstatus)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Com'*"]
    pub unsafe fn FreeBuffer(&self, pmessage: *mut RPCOLEMESSAGE) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(pmessage)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Com'*"]
    pub unsafe fn GetDestCtx(&self, pdwdestcontext: *mut u32, ppvdestcontext: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(pdwdestcontext), ::core::mem::transmute(ppvdestcontext)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Com'*"]
    pub unsafe fn IsConnected(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Com'*"]
    pub unsafe fn GetProtocolVersion(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Com'*"]
    pub unsafe fn Send<'a, Param1: ::windows::core::IntoParam<'a, ISynchronize>>(&self, pmsg: *mut RPCOLEMESSAGE, psync: Param1, pulstatus: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(pmsg), psync.into_param().abi(), ::core::mem::transmute(pulstatus)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Com'*"]
    pub unsafe fn Receive(&self, pmsg: *mut RPCOLEMESSAGE, pulstatus: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), ::core::mem::transmute(pmsg), ::core::mem::transmute(pulstatus)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Com'*"]
    pub unsafe fn GetDestCtxEx(&self, pmsg: *const RPCOLEMESSAGE, pdwdestcontext: *mut u32, ppvdestcontext: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), ::core::mem::transmute(pmsg), ::core::mem::transmute(pdwdestcontext), ::core::mem::transmute(ppvdestcontext)).ok()
    }
}
impl ::core::convert::From<IAsyncRpcChannelBuffer> for IRpcChannelBuffer2 {
    fn from(value: IAsyncRpcChannelBuffer) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IAsyncRpcChannelBuffer> for IRpcChannelBuffer2 {
    fn from(value: &IAsyncRpcChannelBuffer) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IRpcChannelBuffer2> for IAsyncRpcChannelBuffer {
    fn into_param(self) -> ::windows::core::Param<'a, IRpcChannelBuffer2> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IRpcChannelBuffer2> for &IAsyncRpcChannelBuffer {
    fn into_param(self) -> ::windows::core::Param<'a, IRpcChannelBuffer2> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IAsyncRpcChannelBuffer> for IRpcChannelBuffer {
    fn from(value: IAsyncRpcChannelBuffer) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IAsyncRpcChannelBuffer> for IRpcChannelBuffer {
    fn from(value: &IAsyncRpcChannelBuffer) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IRpcChannelBuffer> for IAsyncRpcChannelBuffer {
    fn into_param(self) -> ::windows::core::Param<'a, IRpcChannelBuffer> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IRpcChannelBuffer> for &IAsyncRpcChannelBuffer {
    fn into_param(self) -> ::windows::core::Param<'a, IRpcChannelBuffer> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IAsyncRpcChannelBuffer> for ::windows::core::IUnknown {
    fn from(value: IAsyncRpcChannelBuffer) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IAsyncRpcChannelBuffer> for ::windows::core::IUnknown {
    fn from(value: &IAsyncRpcChannelBuffer) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IAsyncRpcChannelBuffer {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IAsyncRpcChannelBuffer {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IAsyncRpcChannelBuffer {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IAsyncRpcChannelBuffer {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAsyncRpcChannelBuffer {}
impl ::core::fmt::Debug for IAsyncRpcChannelBuffer {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAsyncRpcChannelBuffer").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IAsyncRpcChannelBuffer {
    type Vtable = IAsyncRpcChannelBufferVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa5029fb6_3c34_11d1_9c99_00c04fb998aa);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAsyncRpcChannelBufferVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pmessage: *mut RPCOLEMESSAGE, riid: *const ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pmessage: *mut RPCOLEMESSAGE, pstatus: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pmessage: *mut RPCOLEMESSAGE) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwdestcontext: *mut u32, ppvdestcontext: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwversion: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pmsg: *mut RPCOLEMESSAGE, psync: ::windows::core::RawPtr, pulstatus: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pmsg: *mut RPCOLEMESSAGE, pulstatus: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pmsg: *const RPCOLEMESSAGE, pdwdestcontext: *mut u32, ppvdestcontext: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: 'Win32_System_Com'*"]
#[repr(transparent)]
pub struct IAuthenticate(::windows::core::IUnknown);
impl IAuthenticate {
    #[doc = "*Required features: 'Win32_System_Com', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Authenticate(&self, phwnd: *mut super::super::Foundation::HWND, pszusername: *mut super::super::Foundation::PWSTR, pszpassword: *mut super::super::Foundation::PWSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(phwnd), ::core::mem::transmute(pszusername), ::core::mem::transmute(pszpassword)).ok()
    }
}
impl ::core::convert::From<IAuthenticate> for ::windows::core::IUnknown {
    fn from(value: IAuthenticate) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IAuthenticate> for ::windows::core::IUnknown {
    fn from(value: &IAuthenticate) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IAuthenticate {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IAuthenticate {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IAuthenticate {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IAuthenticate {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAuthenticate {}
impl ::core::fmt::Debug for IAuthenticate {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAuthenticate").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IAuthenticate {
    type Vtable = IAuthenticateVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x79eac9d0_baf9_11ce_8c82_00aa004ba90b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAuthenticateVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, phwnd: *mut super::super::Foundation::HWND, pszusername: *mut super::super::Foundation::PWSTR, pszpassword: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: 'Win32_System_Com'*"]
#[repr(transparent)]
pub struct IAuthenticateEx(::windows::core::IUnknown);
impl IAuthenticateEx {
    #[doc = "*Required features: 'Win32_System_Com', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Authenticate(&self, phwnd: *mut super::super::Foundation::HWND, pszusername: *mut super::super::Foundation::PWSTR, pszpassword: *mut super::super::Foundation::PWSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(phwnd), ::core::mem::transmute(pszusername), ::core::mem::transmute(pszpassword)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Com', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn AuthenticateEx(&self, phwnd: *mut super::super::Foundation::HWND, pszusername: *mut super::super::Foundation::PWSTR, pszpassword: *mut super::super::Foundation::PWSTR, pauthinfo: *const AUTHENTICATEINFO) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(phwnd), ::core::mem::transmute(pszusername), ::core::mem::transmute(pszpassword), ::core::mem::transmute(pauthinfo)).ok()
    }
}
impl ::core::convert::From<IAuthenticateEx> for IAuthenticate {
    fn from(value: IAuthenticateEx) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IAuthenticateEx> for IAuthenticate {
    fn from(value: &IAuthenticateEx) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IAuthenticate> for IAuthenticateEx {
    fn into_param(self) -> ::windows::core::Param<'a, IAuthenticate> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IAuthenticate> for &IAuthenticateEx {
    fn into_param(self) -> ::windows::core::Param<'a, IAuthenticate> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IAuthenticateEx> for ::windows::core::IUnknown {
    fn from(value: IAuthenticateEx) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IAuthenticateEx> for ::windows::core::IUnknown {
    fn from(value: &IAuthenticateEx) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IAuthenticateEx {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IAuthenticateEx {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IAuthenticateEx {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IAuthenticateEx {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAuthenticateEx {}
impl ::core::fmt::Debug for IAuthenticateEx {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAuthenticateEx").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IAuthenticateEx {
    type Vtable = IAuthenticateExVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2ad1edaf_d83d_48b5_9adf_03dbe19f53bd);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAuthenticateExVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, phwnd: *mut super::super::Foundation::HWND, pszusername: *mut super::super::Foundation::PWSTR, pszpassword: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, phwnd: *mut super::super::Foundation::HWND, pszusername: *mut super::super::Foundation::PWSTR, pszpassword: *mut super::super::Foundation::PWSTR, pauthinfo: *const AUTHENTICATEINFO) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: 'Win32_System_Com'*"]
#[repr(transparent)]
pub struct IBindCtx(::windows::core::IUnknown);
impl IBindCtx {
    #[doc = "*Required features: 'Win32_System_Com'*"]
    pub unsafe fn RegisterObjectBound<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>>(&self, punk: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), punk.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_System_Com'*"]
    pub unsafe fn RevokeObjectBound<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>>(&self, punk: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), punk.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_System_Com'*"]
    pub unsafe fn ReleaseBoundObjects(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Com'*"]
    pub unsafe fn SetBindOptions(&self, pbindopts: *const BIND_OPTS) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(pbindopts)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Com'*"]
    pub unsafe fn GetBindOptions(&self, pbindopts: *mut BIND_OPTS) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(pbindopts)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Com'*"]
    pub unsafe fn GetRunningObjectTable(&self) -> ::windows::core::Result<IRunningObjectTable> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IRunningObjectTable>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Com', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RegisterObjectParam<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>>(&self, pszkey: Param0, punk: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), pszkey.into_param().abi(), punk.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_System_Com', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetObjectParam<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pszkey: Param0) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__: *mut ::core::ffi::c_void = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), pszkey.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::IUnknown>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Com'*"]
    pub unsafe fn EnumObjectParam(&self) -> ::windows::core::Result<IEnumString> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IEnumString>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Com', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RevokeObjectParam<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pszkey: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), pszkey.into_param().abi()).ok()
    }
}
impl ::core::convert::From<IBindCtx> for ::windows::core::IUnknown {
    fn from(value: IBindCtx) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IBindCtx> for ::windows::core::IUnknown {
    fn from(value: &IBindCtx) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IBindCtx {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IBindCtx {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IBindCtx {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IBindCtx {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IBindCtx {}
impl ::core::fmt::Debug for IBindCtx {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IBindCtx").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IBindCtx {
    type Vtable = IBindCtxVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0000000e_0000_0000_c000_000000000046);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBindCtxVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, punk: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, punk: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbindopts: *const BIND_OPTS) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbindopts: *mut BIND_OPTS) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pprot: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszkey: super::super::Foundation::PWSTR, punk: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszkey: super::super::Foundation::PWSTR, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszkey: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: 'Win32_System_Com'*"]
#[repr(transparent)]
pub struct IBindHost(::windows::core::IUnknown);
impl IBindHost {
    #[doc = "*Required features: 'Win32_System_Com', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateMoniker<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::core::IntoParam<'a, IBindCtx>>(&self, szname: Param0, pbc: Param1, ppmk: *mut ::core::option::Option<IMoniker>, dwreserved: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), szname.into_param().abi(), pbc.into_param().abi(), ::core::mem::transmute(ppmk), ::core::mem::transmute(dwreserved)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Com'*"]
    pub unsafe fn MonikerBindToStorage<'a, Param0: ::windows::core::IntoParam<'a, IMoniker>, Param1: ::windows::core::IntoParam<'a, IBindCtx>, Param2: ::windows::core::IntoParam<'a, IBindStatusCallback>>(&self, pmk: Param0, pbc: Param1, pbsc: Param2, riid: *const ::windows::core::GUID, ppvobj: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), pmk.into_param().abi(), pbc.into_param().abi(), pbsc.into_param().abi(), ::core::mem::transmute(riid), ::core::mem::transmute(ppvobj)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Com'*"]
    pub unsafe fn MonikerBindToObject<'a, Param0: ::windows::core::IntoParam<'a, IMoniker>, Param1: ::windows::core::IntoParam<'a, IBindCtx>, Param2: ::windows::core::IntoParam<'a, IBindStatusCallback>>(&self, pmk: Param0, pbc: Param1, pbsc: Param2, riid: *const ::windows::core::GUID, ppvobj: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), pmk.into_param().abi(), pbc.into_param().abi(), pbsc.into_param().abi(), ::core::mem::transmute(riid), ::core::mem::transmute(ppvobj)).ok()
    }
}
impl ::core::convert::From<IBindHost> for ::windows::core::IUnknown {
    fn from(value: IBindHost) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IBindHost> for ::windows::core::IUnknown {
    fn from(value: &IBindHost) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IBindHost {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IBindHost {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IBindHost {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IBindHost {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IBindHost {}
impl ::core::fmt::Debug for IBindHost {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IBindHost").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IBindHost {
    type Vtable = IBindHostVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfc4801a1_2ba9_11cf_a229_00aa003d7352);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBindHostVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, szname: super::super::Foundation::PWSTR, pbc: ::windows::core::RawPtr, ppmk: *mut ::windows::core::RawPtr, dwreserved: u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pmk: ::windows::core::RawPtr, pbc: ::windows::core::RawPtr, pbsc: ::windows::core::RawPtr, riid: *const ::windows::core::GUID, ppvobj: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pmk: ::windows::core::RawPtr, pbc: ::windows::core::RawPtr, pbsc: ::windows::core::RawPtr, riid: *const ::windows::core::GUID, ppvobj: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: 'Win32_System_Com'*"]
#[repr(transparent)]
pub struct IBindStatusCallback(::windows::core::IUnknown);
impl IBindStatusCallback {
    #[doc = "*Required features: 'Win32_System_Com'*"]
    pub unsafe fn OnStartBinding<'a, Param1: ::windows::core::IntoParam<'a, IBinding>>(&self, dwreserved: u32, pib: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwreserved), pib.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_System_Com'*"]
    pub unsafe fn GetPriority(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Com'*"]
    pub unsafe fn OnLowResource(&self, reserved: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(reserved)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Com', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn OnProgress<'a, Param3: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, ulprogress: u32, ulprogressmax: u32, ulstatuscode: u32, szstatustext: Param3) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(ulprogress), ::core::mem::transmute(ulprogressmax), ::core::mem::transmute(ulstatuscode), szstatustext.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_System_Com', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn OnStopBinding<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, hresult: ::windows::core::HRESULT, szerror: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(hresult), szerror.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_System_Com', 'Win32_Foundation', 'Win32_Graphics_Gdi', 'Win32_Security', 'Win32_System_Com_StructuredStorage'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_Security", feature = "Win32_System_Com_StructuredStorage"))]
    pub unsafe fn GetBindInfo(&self, grfbindf: *mut u32, pbindinfo: *mut BINDINFO) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(grfbindf), ::core::mem::transmute(pbindinfo)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Com', 'Win32_Foundation', 'Win32_Graphics_Gdi', 'Win32_System_Com_StructuredStorage'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
    pub unsafe fn OnDataAvailable(&self, grfbscf: u32, dwsize: u32, pformatetc: *const FORMATETC, pstgmed: *const STGMEDIUM) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(grfbscf), ::core::mem::transmute(dwsize), ::core::mem::transmute(pformatetc), ::core::mem::transmute(pstgmed)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Com'*"]
    pub unsafe fn OnObjectAvailable<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>>(&self, riid: *const ::windows::core::GUID, punk: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), ::core::mem::transmute(riid), punk.into_param().abi()).ok()
    }
}
impl ::core::convert::From<IBindStatusCallback> for ::windows::core::IUnknown {
    fn from(value: IBindStatusCallback) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IBindStatusCallback> for ::windows::core::IUnknown {
    fn from(value: &IBindStatusCallback) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IBindStatusCallback {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IBindStatusCallback {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IBindStatusCallback {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IBindStatusCallback {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IBindStatusCallback {}
impl ::core::fmt::Debug for IBindStatusCallback {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IBindStatusCallback").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IBindStatusCallback {
    type Vtable = IBindStatusCallbackVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x79eac9c1_baf9_11ce_8c82_00aa004ba90b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBindStatusCallbackVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwreserved: u32, pib: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pnpriority: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, reserved: u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ulprogress: u32, ulprogressmax: u32, ulstatuscode: u32, szstatustext: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hresult: ::windows::core::HRESULT, szerror: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_Security", feature = "Win32_System_Com_StructuredStorage"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, grfbindf: *mut u32, pbindinfo: *mut BINDINFO) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_Security", feature = "Win32_System_Com_StructuredStorage")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, grfbscf: u32, dwsize: u32, pformatetc: *const FORMATETC, pstgmed: *const STGMEDIUM) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage")))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, punk: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: 'Win32_System_Com'*"]
#[repr(transparent)]
pub struct IBindStatusCallbackEx(::windows::core::IUnknown);
impl IBindStatusCallbackEx {
    #[doc = "*Required features: 'Win32_System_Com'*"]
    pub unsafe fn OnStartBinding<'a, Param1: ::windows::core::IntoParam<'a, IBinding>>(&self, dwreserved: u32, pib: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwreserved), pib.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_System_Com'*"]
    pub unsafe fn GetPriority(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Com'*"]
    pub unsafe fn OnLowResource(&self, reserved: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(reserved)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Com', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn OnProgress<'a, Param3: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, ulprogress: u32, ulprogressmax: u32, ulstatuscode: u32, szstatustext: Param3) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(ulprogress), ::core::mem::transmute(ulprogressmax), ::core::mem::transmute(ulstatuscode), szstatustext.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_System_Com', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn OnStopBinding<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, hresult: ::windows::core::HRESULT, szerror: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(hresult), szerror.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_System_Com', 'Win32_Foundation', 'Win32_Graphics_Gdi', 'Win32_Security', 'Win32_System_Com_StructuredStorage'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_Security", feature = "Win32_System_Com_StructuredStorage"))]
    pub unsafe fn GetBindInfo(&self, grfbindf: *mut u32, pbindinfo: *mut BINDINFO) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(grfbindf), ::core::mem::transmute(pbindinfo)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Com', 'Win32_Foundation', 'Win32_Graphics_Gdi', 'Win32_System_Com_StructuredStorage'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
    pub unsafe fn OnDataAvailable(&self, grfbscf: u32, dwsize: u32, pformatetc: *const FORMATETC, pstgmed: *const STGMEDIUM) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(grfbscf), ::core::mem::transmute(dwsize), ::core::mem::transmute(pformatetc), ::core::mem::transmute(pstgmed)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Com'*"]
    pub unsafe fn OnObjectAvailable<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>>(&self, riid: *const ::windows::core::GUID, punk: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), ::core::mem::transmute(riid), punk.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_System_Com', 'Win32_Foundation', 'Win32_Graphics_Gdi', 'Win32_Security', 'Win32_System_Com_StructuredStorage'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_Security", feature = "Win32_System_Com_StructuredStorage"))]
    pub unsafe fn GetBindInfoEx(&self, grfbindf: *mut u32, pbindinfo: *mut BINDINFO, grfbindf2: *mut u32, pdwreserved: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), ::core::mem::transmute(grfbindf), ::core::mem::transmute(pbindinfo), ::core::mem::transmute(grfbindf2), ::core::mem::transmute(pdwreserved)).ok()
    }
}
impl ::core::convert::From<IBindStatusCallbackEx> for IBindStatusCallback {
    fn from(value: IBindStatusCallbackEx) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IBindStatusCallbackEx> for IBindStatusCallback {
    fn from(value: &IBindStatusCallbackEx) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IBindStatusCallback> for IBindStatusCallbackEx {
    fn into_param(self) -> ::windows::core::Param<'a, IBindStatusCallback> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IBindStatusCallback> for &IBindStatusCallbackEx {
    fn into_param(self) -> ::windows::core::Param<'a, IBindStatusCallback> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IBindStatusCallbackEx> for ::windows::core::IUnknown {
    fn from(value: IBindStatusCallbackEx) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IBindStatusCallbackEx> for ::windows::core::IUnknown {
    fn from(value: &IBindStatusCallbackEx) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IBindStatusCallbackEx {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IBindStatusCallbackEx {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IBindStatusCallbackEx {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IBindStatusCallbackEx {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IBindStatusCallbackEx {}
impl ::core::fmt::Debug for IBindStatusCallbackEx {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IBindStatusCallbackEx").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IBindStatusCallbackEx {
    type Vtable = IBindStatusCallbackExVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xaaa74ef9_8ee7_4659_88d9_f8c504da73cc);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBindStatusCallbackExVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwreserved: u32, pib: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pnpriority: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, reserved: u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ulprogress: u32, ulprogressmax: u32, ulstatuscode: u32, szstatustext: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hresult: ::windows::core::HRESULT, szerror: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_Security", feature = "Win32_System_Com_StructuredStorage"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, grfbindf: *mut u32, pbindinfo: *mut BINDINFO) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_Security", feature = "Win32_System_Com_StructuredStorage")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, grfbscf: u32, dwsize: u32, pformatetc: *const FORMATETC, pstgmed: *const STGMEDIUM) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage")))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, punk: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_Security", feature = "Win32_System_Com_StructuredStorage"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, grfbindf: *mut u32, pbindinfo: *mut BINDINFO, grfbindf2: *mut u32, pdwreserved: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_Security", feature = "Win32_System_Com_StructuredStorage")))] usize,
);
#[doc = "*Required features: 'Win32_System_Com'*"]
#[repr(transparent)]
pub struct IBinding(::windows::core::IUnknown);
impl IBinding {
    #[doc = "*Required features: 'Win32_System_Com'*"]
    pub unsafe fn Abort(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Com'*"]
    pub unsafe fn Suspend(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Com'*"]
    pub unsafe fn Resume(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Com'*"]
    pub unsafe fn SetPriority(&self, npriority: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(npriority)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Com'*"]
    pub unsafe fn GetPriority(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Com', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetBindResult(&self, pclsidprotocol: *mut ::windows::core::GUID, pdwresult: *mut u32, pszresult: *mut super::super::Foundation::PWSTR, pdwreserved: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(pclsidprotocol), ::core::mem::transmute(pdwresult), ::core::mem::transmute(pszresult), ::core::mem::transmute(pdwreserved)).ok()
    }
}
impl ::core::convert::From<IBinding> for ::windows::core::IUnknown {
    fn from(value: IBinding) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IBinding> for ::windows::core::IUnknown {
    fn from(value: &IBinding) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IBinding {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IBinding {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IBinding {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IBinding {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IBinding {}
impl ::core::fmt::Debug for IBinding {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IBinding").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IBinding {
    type Vtable = IBindingVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x79eac9c0_baf9_11ce_8c82_00aa004ba90b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBindingVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, npriority: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pnpriority: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pclsidprotocol: *mut ::windows::core::GUID, pdwresult: *mut u32, pszresult: *mut super::super::Foundation::PWSTR, pdwreserved: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: 'Win32_System_Com'*"]
#[repr(transparent)]
pub struct IBlockingLock(::windows::core::IUnknown);
impl IBlockingLock {
    #[doc = "*Required features: 'Win32_System_Com'*"]
    pub unsafe fn Lock(&self, dwtimeout: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwtimeout)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Com'*"]
    pub unsafe fn Unlock(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self)).ok()
    }
}
impl ::core::convert::From<IBlockingLock> for ::windows::core::IUnknown {
    fn from(value: IBlockingLock) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IBlockingLock> for ::windows::core::IUnknown {
    fn from(value: &IBlockingLock) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IBlockingLock {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IBlockingLock {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IBlockingLock {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IBlockingLock {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IBlockingLock {}
impl ::core::fmt::Debug for IBlockingLock {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IBlockingLock").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IBlockingLock {
    type Vtable = IBlockingLockVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x30f3d47a_6447_11d1_8e3c_00c04fb9386d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBlockingLockVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwtimeout: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: 'Win32_System_Com'*"]
#[repr(transparent)]
pub struct ICallFactory(::windows::core::IUnknown);
impl ICallFactory {
    #[doc = "*Required features: 'Win32_System_Com'*"]
    pub unsafe fn CreateCall<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>>(&self, riid: *const ::windows::core::GUID, pctrlunk: Param1, riid2: *const ::windows::core::GUID) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__: *mut ::core::ffi::c_void = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(riid), pctrlunk.into_param().abi(), ::core::mem::transmute(riid2), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::IUnknown>(result__)
    }
}
impl ::core::convert::From<ICallFactory> for ::windows::core::IUnknown {
    fn from(value: ICallFactory) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ICallFactory> for ::windows::core::IUnknown {
    fn from(value: &ICallFactory) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ICallFactory {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &ICallFactory {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ICallFactory {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ICallFactory {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ICallFactory {}
impl ::core::fmt::Debug for ICallFactory {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ICallFactory").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ICallFactory {
    type Vtable = ICallFactoryVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1c733a30_2a1c_11ce_ade5_00aa0044773d);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICallFactoryVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, pctrlunk: *mut ::core::ffi::c_void, riid2: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: 'Win32_System_Com'*"]
#[repr(transparent)]
pub struct ICancelMethodCalls(::windows::core::IUnknown);
impl ICancelMethodCalls {
    #[doc = "*Required features: 'Win32_System_Com'*"]
    pub unsafe fn Cancel(&self, ulseconds: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(ulseconds)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Com'*"]
    pub unsafe fn TestCancel(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self)).ok()
    }
}
impl ::core::convert::From<ICancelMethodCalls> for ::windows::core::IUnknown {
    fn from(value: ICancelMethodCalls) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ICancelMethodCalls> for ::windows::core::IUnknown {
    fn from(value: &ICancelMethodCalls) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ICancelMethodCalls {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &ICancelMethodCalls {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ICancelMethodCalls {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ICancelMethodCalls {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ICancelMethodCalls {}
impl ::core::fmt::Debug for ICancelMethodCalls {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ICancelMethodCalls").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ICancelMethodCalls {
    type Vtable = ICancelMethodCallsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x00000029_0000_0000_c000_000000000046);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICancelMethodCallsVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ulseconds: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: 'Win32_System_Com'*"]
#[repr(transparent)]
pub struct ICatInformation(::windows::core::IUnknown);
impl ICatInformation {
    #[doc = "*Required features: 'Win32_System_Com'*"]
    pub unsafe fn EnumCategories(&self, lcid: u32) -> ::windows::core::Result<IEnumCATEGORYINFO> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(lcid), ::core::mem::transmute(&mut result__)).from_abi::<IEnumCATEGORYINFO>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Com', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetCategoryDesc(&self, rcatid: *const ::windows::core::GUID, lcid: u32) -> ::windows::core::Result<super::super::Foundation::PWSTR> {
        let mut result__: super::super::Foundation::PWSTR = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(rcatid), ::core::mem::transmute(lcid), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::PWSTR>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Com'*"]
    pub unsafe fn EnumClassesOfCategories(&self, cimplemented: u32, rgcatidimpl: *const ::windows::core::GUID, crequired: u32, rgcatidreq: *const ::windows::core::GUID) -> ::windows::core::Result<IEnumGUID> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(cimplemented), ::core::mem::transmute(rgcatidimpl), ::core::mem::transmute(crequired), ::core::mem::transmute(rgcatidreq), ::core::mem::transmute(&mut result__)).from_abi::<IEnumGUID>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Com'*"]
    pub unsafe fn IsClassOfCategories(&self, rclsid: *const ::windows::core::GUID, cimplemented: u32, rgcatidimpl: *const ::windows::core::GUID, crequired: u32, rgcatidreq: *const ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(rclsid), ::core::mem::transmute(cimplemented), ::core::mem::transmute(rgcatidimpl), ::core::mem::transmute(crequired), ::core::mem::transmute(rgcatidreq)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Com'*"]
    pub unsafe fn EnumImplCategoriesOfClass(&self, rclsid: *const ::windows::core::GUID) -> ::windows::core::Result<IEnumGUID> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(rclsid), ::core::mem::transmute(&mut result__)).from_abi::<IEnumGUID>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Com'*"]
    pub unsafe fn EnumReqCategoriesOfClass(&self, rclsid: *const ::windows::core::GUID) -> ::windows::core::Result<IEnumGUID> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(rclsid), ::core::mem::transmute(&mut result__)).from_abi::<IEnumGUID>(result__)
    }
}
impl ::core::convert::From<ICatInformation> for ::windows::core::IUnknown {
    fn from(value: ICatInformation) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ICatInformation> for ::windows::core::IUnknown {
    fn from(value: &ICatInformation) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ICatInformation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &ICatInformation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ICatInformation {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ICatInformation {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ICatInformation {}
impl ::core::fmt::Debug for ICatInformation {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ICatInformation").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ICatInformation {
    type Vtable = ICatInformationVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0002e013_0000_0000_c000_000000000046);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICatInformationVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lcid: u32, ppenumcategoryinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, rcatid: *const ::windows::core::GUID, lcid: u32, pszdesc: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cimplemented: u32, rgcatidimpl: *const ::windows::core::GUID, crequired: u32, rgcatidreq: *const ::windows::core::GUID, ppenumclsid: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, rclsid: *const ::windows::core::GUID, cimplemented: u32, rgcatidimpl: *const ::windows::core::GUID, crequired: u32, rgcatidreq: *const ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, rclsid: *const ::windows::core::GUID, ppenumcatid: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, rclsid: *const ::windows::core::GUID, ppenumcatid: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: 'Win32_System_Com'*"]
#[repr(transparent)]
pub struct ICatRegister(::windows::core::IUnknown);
impl ICatRegister {
    #[doc = "*Required features: 'Win32_System_Com'*"]
    pub unsafe fn RegisterCategories(&self, ccategories: u32, rgcategoryinfo: *const CATEGORYINFO) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(ccategories), ::core::mem::transmute(rgcategoryinfo)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Com'*"]
    pub unsafe fn UnRegisterCategories(&self, ccategories: u32, rgcatid: *const ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(ccategories), ::core::mem::transmute(rgcatid)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Com'*"]
    pub unsafe fn RegisterClassImplCategories(&self, rclsid: *const ::windows::core::GUID, ccategories: u32, rgcatid: *const ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(rclsid), ::core::mem::transmute(ccategories), ::core::mem::transmute(rgcatid)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Com'*"]
    pub unsafe fn UnRegisterClassImplCategories(&self, rclsid: *const ::windows::core::GUID, ccategories: u32, rgcatid: *const ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(rclsid), ::core::mem::transmute(ccategories), ::core::mem::transmute(rgcatid)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Com'*"]
    pub unsafe fn RegisterClassReqCategories(&self, rclsid: *const ::windows::core::GUID, ccategories: u32, rgcatid: *const ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(rclsid), ::core::mem::transmute(ccategories), ::core::mem::transmute(rgcatid)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Com'*"]
    pub unsafe fn UnRegisterClassReqCategories(&self, rclsid: *const ::windows::core::GUID, ccategories: u32, rgcatid: *const ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(rclsid), ::core::mem::transmute(ccategories), ::core::mem::transmute(rgcatid)).ok()
    }
}
impl ::core::convert::From<ICatRegister> for ::windows::core::IUnknown {
    fn from(value: ICatRegister) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ICatRegister> for ::windows::core::IUnknown {
    fn from(value: &ICatRegister) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ICatRegister {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &ICatRegister {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ICatRegister {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ICatRegister {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ICatRegister {}
impl ::core::fmt::Debug for ICatRegister {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ICatRegister").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ICatRegister {
    type Vtable = ICatRegisterVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0002e012_0000_0000_c000_000000000046);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICatRegisterVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ccategories: u32, rgcategoryinfo: *const CATEGORYINFO) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ccategories: u32, rgcatid: *const ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, rclsid: *const ::windows::core::GUID, ccategories: u32, rgcatid: *const ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, rclsid: *const ::windows::core::GUID, ccategories: u32, rgcatid: *const ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, rclsid: *const ::windows::core::GUID, ccategories: u32, rgcatid: *const ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, rclsid: *const ::windows::core::GUID, ccategories: u32, rgcatid: *const ::windows::core::GUID) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: 'Win32_System_Com'*"]
#[repr(transparent)]
pub struct IChannelHook(::windows::core::IUnknown);
impl IChannelHook {
    #[doc = "*Required features: 'Win32_System_Com'*"]
    pub unsafe fn ClientGetSize(&self, uextent: *const ::windows::core::GUID, riid: *const ::windows::core::GUID, pdatasize: *mut u32) {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(uextent), ::core::mem::transmute(riid), ::core::mem::transmute(pdatasize))
    }
    #[doc = "*Required features: 'Win32_System_Com'*"]
    pub unsafe fn ClientFillBuffer(&self, uextent: *const ::windows::core::GUID, riid: *const ::windows::core::GUID, pdatasize: *mut u32, pdatabuffer: *const ::core::ffi::c_void) {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(uextent), ::core::mem::transmute(riid), ::core::mem::transmute(pdatasize), ::core::mem::transmute(pdatabuffer))
    }
    #[doc = "*Required features: 'Win32_System_Com'*"]
    pub unsafe fn ClientNotify(&self, uextent: *const ::windows::core::GUID, riid: *const ::windows::core::GUID, cbdatasize: u32, pdatabuffer: *const ::core::ffi::c_void, ldatarep: u32, hrfault: ::windows::core::HRESULT) {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(uextent), ::core::mem::transmute(riid), ::core::mem::transmute(cbdatasize), ::core::mem::transmute(pdatabuffer), ::core::mem::transmute(ldatarep), ::core::mem::transmute(hrfault))
    }
    #[doc = "*Required features: 'Win32_System_Com'*"]
    pub unsafe fn ServerNotify(&self, uextent: *const ::windows::core::GUID, riid: *const ::windows::core::GUID, cbdatasize: u32, pdatabuffer: *const ::core::ffi::c_void, ldatarep: u32) {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(uextent), ::core::mem::transmute(riid), ::core::mem::transmute(cbdatasize), ::core::mem::transmute(pdatabuffer), ::core::mem::transmute(ldatarep))
    }
    #[doc = "*Required features: 'Win32_System_Com'*"]
    pub unsafe fn ServerGetSize(&self, uextent: *const ::windows::core::GUID, riid: *const ::windows::core::GUID, hrfault: ::windows::core::HRESULT, pdatasize: *mut u32) {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(uextent), ::core::mem::transmute(riid), ::core::mem::transmute(hrfault), ::core::mem::transmute(pdatasize))
    }
    #[doc = "*Required features: 'Win32_System_Com'*"]
    pub unsafe fn ServerFillBuffer(&self, uextent: *const ::windows::core::GUID, riid: *const ::windows::core::GUID, pdatasize: *mut u32, pdatabuffer: *const ::core::ffi::c_void, hrfault: ::windows::core::HRESULT) {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(uextent), ::core::mem::transmute(riid), ::core::mem::transmute(pdatasize), ::core::mem::transmute(pdatabuffer), ::core::mem::transmute(hrfault))
    }
}
impl ::core::convert::From<IChannelHook> for ::windows::core::IUnknown {
    fn from(value: IChannelHook) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IChannelHook> for ::windows::core::IUnknown {
    fn from(value: &IChannelHook) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IChannelHook {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IChannelHook {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IChannelHook {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IChannelHook {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IChannelHook {}
impl ::core::fmt::Debug for IChannelHook {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IChannelHook").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IChannelHook {
    type Vtable = IChannelHookVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1008c4a0_7613_11cf_9af1_0020af6e72f4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IChannelHookVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uextent: *const ::windows::core::GUID, riid: *const ::windows::core::GUID, pdatasize: *mut u32),
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uextent: *const ::windows::core::GUID, riid: *const ::windows::core::GUID, pdatasize: *mut u32, pdatabuffer: *const ::core::ffi::c_void),
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uextent: *const ::windows::core::GUID, riid: *const ::windows::core::GUID, cbdatasize: u32, pdatabuffer: *const ::core::ffi::c_void, ldatarep: u32, hrfault: ::windows::core::HRESULT),
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uextent: *const ::windows::core::GUID, riid: *const ::windows::core::GUID, cbdatasize: u32, pdatabuffer: *const ::core::ffi::c_void, ldatarep: u32),
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uextent: *const ::windows::core::GUID, riid: *const ::windows::core::GUID, hrfault: ::windows::core::HRESULT, pdatasize: *mut u32),
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uextent: *const ::windows::core::GUID, riid: *const ::windows::core::GUID, pdatasize: *mut u32, pdatabuffer: *const ::core::ffi::c_void, hrfault: ::windows::core::HRESULT),
);
#[doc = "*Required features: 'Win32_System_Com'*"]
#[repr(transparent)]
pub struct IClassActivator(::windows::core::IUnknown);
impl IClassActivator {
    #[doc = "*Required features: 'Win32_System_Com'*"]
    pub unsafe fn GetClassObject<T: ::windows::core::Interface>(&self, rclsid: *const ::windows::core::GUID, dwclasscontext: u32, locale: u32) -> ::windows::core::Result<T> {
        let mut result__ = ::core::option::Option::None;
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(rclsid), ::core::mem::transmute(dwclasscontext), ::core::mem::transmute(locale), &<T as ::windows::core::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
}
impl ::core::convert::From<IClassActivator> for ::windows::core::IUnknown {
    fn from(value: IClassActivator) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IClassActivator> for ::windows::core::IUnknown {
    fn from(value: &IClassActivator) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IClassActivator {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IClassActivator {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IClassActivator {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IClassActivator {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IClassActivator {}
impl ::core::fmt::Debug for IClassActivator {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IClassActivator").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IClassActivator {
    type Vtable = IClassActivatorVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x00000140_0000_0000_c000_000000000046);
}
#[repr(C)]
#[doc(hidden)]
pub struct IClassActivatorVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, rclsid: *const ::windows::core::GUID, dwclasscontext: u32, locale: u32, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: 'Win32_System_Com'*"]
#[repr(transparent)]
pub struct IClassFactory(::windows::core::IUnknown);
impl IClassFactory {
    #[doc = "*Required features: 'Win32_System_Com'*"]
    pub unsafe fn CreateInstance<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>, T: ::windows::core::Interface>(&self, punkouter: Param0) -> ::windows::core::Result<T> {
        let mut result__ = ::core::option::Option::None;
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), punkouter.into_param().abi(), &<T as ::windows::core::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
    #[doc = "*Required features: 'Win32_System_Com', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn LockServer<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, flock: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), flock.into_param().abi()).ok()
    }
}
impl ::core::convert::From<IClassFactory> for ::windows::core::IUnknown {
    fn from(value: IClassFactory) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IClassFactory> for ::windows::core::IUnknown {
    fn from(value: &IClassFactory) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IClassFactory {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IClassFactory {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IClassFactory {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IClassFactory {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IClassFactory {}
impl ::core::fmt::Debug for IClassFactory {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IClassFactory").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IClassFactory {
    type Vtable = IClassFactoryVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x00000001_0000_0000_c000_000000000046);
}
#[repr(C)]
#[doc(hidden)]
pub struct IClassFactoryVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, punkouter: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, ppvobject: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, flock: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: 'Win32_System_Com'*"]
#[repr(transparent)]
pub struct IClientSecurity(::windows::core::IUnknown);
impl IClientSecurity {
    #[doc = "*Required features: 'Win32_System_Com'*"]
    pub unsafe fn QueryBlanket<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>>(&self, pproxy: Param0, pauthnsvc: *mut u32, pauthzsvc: *mut u32, pserverprincname: *mut *mut u16, pauthnlevel: *mut RPC_C_AUTHN_LEVEL, pimplevel: *mut RPC_C_IMP_LEVEL, pauthinfo: *mut *mut ::core::ffi::c_void, pcapabilites: *mut EOLE_AUTHENTICATION_CAPABILITIES) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), pproxy.into_param().abi(), ::core::mem::transmute(pauthnsvc), ::core::mem::transmute(pauthzsvc), ::core::mem::transmute(pserverprincname), ::core::mem::transmute(pauthnlevel), ::core::mem::transmute(pimplevel), ::core::mem::transmute(pauthinfo), ::core::mem::transmute(pcapabilites)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Com', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetBlanket<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>, Param3: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pproxy: Param0, dwauthnsvc: u32, dwauthzsvc: u32, pserverprincname: Param3, dwauthnlevel: RPC_C_AUTHN_LEVEL, dwimplevel: RPC_C_IMP_LEVEL, pauthinfo: *const ::core::ffi::c_void, dwcapabilities: EOLE_AUTHENTICATION_CAPABILITIES) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), pproxy.into_param().abi(), ::core::mem::transmute(dwauthnsvc), ::core::mem::transmute(dwauthzsvc), pserverprincname.into_param().abi(), ::core::mem::transmute(dwauthnlevel), ::core::mem::transmute(dwimplevel), ::core::mem::transmute(pauthinfo), ::core::mem::transmute(dwcapabilities)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Com'*"]
    pub unsafe fn CopyProxy<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>>(&self, pproxy: Param0) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__: *mut ::core::ffi::c_void = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), pproxy.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::IUnknown>(result__)
    }
}
impl ::core::convert::From<IClientSecurity> for ::windows::core::IUnknown {
    fn from(value: IClientSecurity) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IClientSecurity> for ::windows::core::IUnknown {
    fn from(value: &IClientSecurity) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IClientSecurity {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IClientSecurity {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IClientSecurity {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IClientSecurity {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IClientSecurity {}
impl ::core::fmt::Debug for IClientSecurity {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IClientSecurity").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IClientSecurity {
    type Vtable = IClientSecurityVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0000013d_0000_0000_c000_000000000046);
}
#[repr(C)]
#[doc(hidden)]
pub struct IClientSecurityVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pproxy: *mut ::core::ffi::c_void, pauthnsvc: *mut u32, pauthzsvc: *mut u32, pserverprincname: *mut *mut u16, pauthnlevel: *mut RPC_C_AUTHN_LEVEL, pimplevel: *mut RPC_C_IMP_LEVEL, pauthinfo: *mut *mut ::core::ffi::c_void, pcapabilites: *mut EOLE_AUTHENTICATION_CAPABILITIES) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pproxy: *mut ::core::ffi::c_void, dwauthnsvc: u32, dwauthzsvc: u32, pserverprincname: super::super::Foundation::PWSTR, dwauthnlevel: RPC_C_AUTHN_LEVEL, dwimplevel: RPC_C_IMP_LEVEL, pauthinfo: *const ::core::ffi::c_void, dwcapabilities: EOLE_AUTHENTICATION_CAPABILITIES) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pproxy: *mut ::core::ffi::c_void, ppcopy: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: 'Win32_System_Com'*"]
#[repr(transparent)]
pub struct IComThreadingInfo(::windows::core::IUnknown);
impl IComThreadingInfo {
    #[doc = "*Required features: 'Win32_System_Com'*"]
    pub unsafe fn GetCurrentApartmentType(&self) -> ::windows::core::Result<APTTYPE> {
        let mut result__: APTTYPE = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<APTTYPE>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Com'*"]
    pub unsafe fn GetCurrentThreadType(&self) -> ::windows::core::Result<THDTYPE> {
        let mut result__: THDTYPE = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<THDTYPE>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Com'*"]
    pub unsafe fn GetCurrentLogicalThreadId(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::GUID>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Com'*"]
    pub unsafe fn SetCurrentLogicalThreadId(&self, rguid: *const ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(rguid)).ok()
    }
}
impl ::core::convert::From<IComThreadingInfo> for ::windows::core::IUnknown {
    fn from(value: IComThreadingInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IComThreadingInfo> for ::windows::core::IUnknown {
    fn from(value: &IComThreadingInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IComThreadingInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IComThreadingInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IComThreadingInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IComThreadingInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IComThreadingInfo {}
impl ::core::fmt::Debug for IComThreadingInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IComThreadingInfo").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IComThreadingInfo {
    type Vtable = IComThreadingInfoVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x000001ce_0000_0000_c000_000000000046);
}
#[repr(C)]
#[doc(hidden)]
pub struct IComThreadingInfoVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, papttype: *mut APTTYPE) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pthreadtype: *mut THDTYPE) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pguidlogicalthreadid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, rguid: *const ::windows::core::GUID) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: 'Win32_System_Com'*"]
#[repr(transparent)]
pub struct IConnectionPoint(::windows::core::IUnknown);
impl IConnectionPoint {
    #[doc = "*Required features: 'Win32_System_Com'*"]
    pub unsafe fn GetConnectionInterface(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::GUID>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Com'*"]
    pub unsafe fn GetConnectionPointContainer(&self) -> ::windows::core::Result<IConnectionPointContainer> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IConnectionPointContainer>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Com'*"]
    pub unsafe fn Advise<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>>(&self, punksink: Param0) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), punksink.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Com'*"]
    pub unsafe fn Unadvise(&self, dwcookie: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwcookie)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Com'*"]
    pub unsafe fn EnumConnections(&self) -> ::windows::core::Result<IEnumConnections> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IEnumConnections>(result__)
    }
}
impl ::core::convert::From<IConnectionPoint> for ::windows::core::IUnknown {
    fn from(value: IConnectionPoint) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IConnectionPoint> for ::windows::core::IUnknown {
    fn from(value: &IConnectionPoint) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IConnectionPoint {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IConnectionPoint {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IConnectionPoint {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IConnectionPoint {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IConnectionPoint {}
impl ::core::fmt::Debug for IConnectionPoint {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IConnectionPoint").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IConnectionPoint {
    type Vtable = IConnectionPointVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb196b286_bab4_101a_b69c_00aa00341d07);
}
#[repr(C)]
#[doc(hidden)]
pub struct IConnectionPointVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, piid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppcpc: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, punksink: *mut ::core::ffi::c_void, pdwcookie: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwcookie: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: 'Win32_System_Com'*"]
#[repr(transparent)]
pub struct IConnectionPointContainer(::windows::core::IUnknown);
impl IConnectionPointContainer {
    #[doc = "*Required features: 'Win32_System_Com'*"]
    pub unsafe fn EnumConnectionPoints(&self) -> ::windows::core::Result<IEnumConnectionPoints> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IEnumConnectionPoints>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Com'*"]
    pub unsafe fn FindConnectionPoint(&self, riid: *const ::windows::core::GUID) -> ::windows::core::Result<IConnectionPoint> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(riid), ::core::mem::transmute(&mut result__)).from_abi::<IConnectionPoint>(result__)
    }
}
impl ::core::convert::From<IConnectionPointContainer> for ::windows::core::IUnknown {
    fn from(value: IConnectionPointContainer) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IConnectionPointContainer> for ::windows::core::IUnknown {
    fn from(value: &IConnectionPointContainer) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IConnectionPointContainer {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IConnectionPointContainer {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IConnectionPointContainer {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IConnectionPointContainer {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IConnectionPointContainer {}
impl ::core::fmt::Debug for IConnectionPointContainer {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IConnectionPointContainer").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IConnectionPointContainer {
    type Vtable = IConnectionPointContainerVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb196b284_bab4_101a_b69c_00aa00341d07);
}
#[repr(C)]
#[doc(hidden)]
pub struct IConnectionPointContainerVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, ppcp: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(C)]
pub struct IContext(pub u8);
#[doc = "*Required features: 'Win32_System_Com'*"]
#[repr(transparent)]
pub struct IContextCallback(::windows::core::IUnknown);
impl IContextCallback {
    #[doc = "*Required features: 'Win32_System_Com'*"]
    pub unsafe fn ContextCallback<'a, Param4: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>>(&self, pfncallback: PFNCONTEXTCALL, pparam: *const ComCallData, riid: *const ::windows::core::GUID, imethod: i32, punk: Param4) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(pfncallback), ::core::mem::transmute(pparam), ::core::mem::transmute(riid), ::core::mem::transmute(imethod), punk.into_param().abi()).ok()
    }
}
impl ::core::convert::From<IContextCallback> for ::windows::core::IUnknown {
    fn from(value: IContextCallback) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IContextCallback> for ::windows::core::IUnknown {
    fn from(value: &IContextCallback) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IContextCallback {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IContextCallback {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IContextCallback {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IContextCallback {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IContextCallback {}
impl ::core::fmt::Debug for IContextCallback {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IContextCallback").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IContextCallback {
    type Vtable = IContextCallbackVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x000001da_0000_0000_c000_000000000046);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContextCallbackVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfncallback: ::windows::core::RawPtr, pparam: *const ComCallData, riid: *const ::windows::core::GUID, imethod: i32, punk: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
);
#[repr(C)]
#[doc = "*Required features: 'Win32_System_Com'*"]
pub struct IDLDESC {
    pub dwReserved: usize,
    pub wIDLFlags: u16,
}
impl ::core::marker::Copy for IDLDESC {}
impl ::core::clone::Clone for IDLDESC {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for IDLDESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IDLDESC").field("dwReserved", &self.dwReserved).field("wIDLFlags", &self.wIDLFlags).finish()
    }
}
unsafe impl ::windows::core::Abi for IDLDESC {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for IDLDESC {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<IDLDESC>()) == 0 }
    }
}
impl ::core::cmp::Eq for IDLDESC {}
impl ::core::default::Default for IDLDESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: 'Win32_System_Com'*"]
#[repr(transparent)]
pub struct IDataAdviseHolder(::windows::core::IUnknown);
impl IDataAdviseHolder {
    #[doc = "*Required features: 'Win32_System_Com'*"]
    pub unsafe fn Advise<'a, Param0: ::windows::core::IntoParam<'a, IDataObject>, Param3: ::windows::core::IntoParam<'a, IAdviseSink>>(&self, pdataobject: Param0, pfetc: *const FORMATETC, advf: u32, padvise: Param3) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), pdataobject.into_param().abi(), ::core::mem::transmute(pfetc), ::core::mem::transmute(advf), padvise.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Com'*"]
    pub unsafe fn Unadvise(&self, dwconnection: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwconnection)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Com'*"]
    pub unsafe fn EnumAdvise(&self) -> ::windows::core::Result<IEnumSTATDATA> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IEnumSTATDATA>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Com'*"]
    pub unsafe fn SendOnDataChange<'a, Param0: ::windows::core::IntoParam<'a, IDataObject>>(&self, pdataobject: Param0, dwreserved: u32, advf: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), pdataobject.into_param().abi(), ::core::mem::transmute(dwreserved), ::core::mem::transmute(advf)).ok()
    }
}
impl ::core::convert::From<IDataAdviseHolder> for ::windows::core::IUnknown {
    fn from(value: IDataAdviseHolder) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDataAdviseHolder> for ::windows::core::IUnknown {
    fn from(value: &IDataAdviseHolder) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IDataAdviseHolder {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IDataAdviseHolder {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IDataAdviseHolder {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDataAdviseHolder {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDataAdviseHolder {}
impl ::core::fmt::Debug for IDataAdviseHolder {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDataAdviseHolder").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IDataAdviseHolder {
    type Vtable = IDataAdviseHolderVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x00000110_0000_0000_c000_000000000046);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDataAdviseHolderVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdataobject: ::windows::core::RawPtr, pfetc: *const FORMATETC, advf: u32, padvise: ::windows::core::RawPtr, pdwconnection: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwconnection: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenumadvise: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdataobject: ::windows::core::RawPtr, dwreserved: u32, advf: u32) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: 'Win32_System_Com'*"]
#[repr(transparent)]
pub struct IDataObject(::windows::core::IUnknown);
impl IDataObject {
    #[doc = "*Required features: 'Win32_System_Com', 'Win32_Foundation', 'Win32_Graphics_Gdi', 'Win32_System_Com_StructuredStorage'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
    pub unsafe fn GetData(&self, pformatetcin: *const FORMATETC) -> ::windows::core::Result<STGMEDIUM> {
        let mut result__: ::core::mem::ManuallyDrop<STGMEDIUM> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(pformatetcin), ::core::mem::transmute(&mut result__)).from_abi::<STGMEDIUM>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Com', 'Win32_Foundation', 'Win32_Graphics_Gdi', 'Win32_System_Com_StructuredStorage'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
    pub unsafe fn GetDataHere(&self, pformatetc: *const FORMATETC, pmedium: *mut STGMEDIUM) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(pformatetc), ::core::mem::transmute(pmedium)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Com'*"]
    pub unsafe fn QueryGetData(&self, pformatetc: *const FORMATETC) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(pformatetc)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Com'*"]
    pub unsafe fn GetCanonicalFormatEtc(&self, pformatectin: *const FORMATETC) -> ::windows::core::Result<FORMATETC> {
        let mut result__: FORMATETC = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(pformatectin), ::core::mem::transmute(&mut result__)).from_abi::<FORMATETC>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Com', 'Win32_Foundation', 'Win32_Graphics_Gdi', 'Win32_System_Com_StructuredStorage'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
    pub unsafe fn SetData<'a, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, pformatetc: *const FORMATETC, pmedium: *const STGMEDIUM, frelease: Param2) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(pformatetc), ::core::mem::transmute(pmedium), frelease.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_System_Com'*"]
    pub unsafe fn EnumFormatEtc(&self, dwdirection: u32) -> ::windows::core::Result<IEnumFORMATETC> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwdirection), ::core::mem::transmute(&mut result__)).from_abi::<IEnumFORMATETC>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Com'*"]
    pub unsafe fn DAdvise<'a, Param2: ::windows::core::IntoParam<'a, IAdviseSink>>(&self, pformatetc: *const FORMATETC, advf: u32, padvsink: Param2) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(pformatetc), ::core::mem::transmute(advf), padvsink.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Com'*"]
    pub unsafe fn DUnadvise(&self, dwconnection: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwconnection)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Com'*"]
    pub unsafe fn EnumDAdvise(&self) -> ::windows::core::Result<IEnumSTATDATA> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IEnumSTATDATA>(result__)
    }
}
impl ::core::convert::From<IDataObject> for ::windows::core::IUnknown {
    fn from(value: IDataObject) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDataObject> for ::windows::core::IUnknown {
    fn from(value: &IDataObject) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IDataObject {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IDataObject {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IDataObject {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDataObject {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDataObject {}
impl ::core::fmt::Debug for IDataObject {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDataObject").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IDataObject {
    type Vtable = IDataObjectVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0000010e_0000_0000_c000_000000000046);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDataObjectVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pformatetcin: *const FORMATETC, pmedium: *mut STGMEDIUM) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pformatetc: *const FORMATETC, pmedium: *mut STGMEDIUM) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage")))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pformatetc: *const FORMATETC) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pformatectin: *const FORMATETC, pformatetcout: *mut FORMATETC) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pformatetc: *const FORMATETC, pmedium: *const STGMEDIUM, frelease: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage")))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwdirection: u32, ppenumformatetc: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pformatetc: *const FORMATETC, advf: u32, padvsink: ::windows::core::RawPtr, pdwconnection: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwconnection: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenumadvise: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: 'Win32_System_Com'*"]
#[repr(transparent)]
pub struct IDispatch(::windows::core::IUnknown);
impl IDispatch {
    #[doc = "*Required features: 'Win32_System_Com'*"]
    pub unsafe fn GetTypeInfoCount(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Com'*"]
    pub unsafe fn GetTypeInfo(&self, itinfo: u32, lcid: u32) -> ::windows::core::Result<ITypeInfo> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(itinfo), ::core::mem::transmute(lcid), ::core::mem::transmute(&mut result__)).from_abi::<ITypeInfo>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Com', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetIDsOfNames(&self, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(riid), ::core::mem::transmute(rgsznames), ::core::mem::transmute(cnames), ::core::mem::transmute(lcid), ::core::mem::transmute(rgdispid)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Com', 'Win32_Foundation', 'Win32_System_Ole'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
    pub unsafe fn Invoke(&self, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const DISPPARAMS, pvarresult: *mut VARIANT, pexcepinfo: *mut EXCEPINFO, puargerr: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(dispidmember), ::core::mem::transmute(riid), ::core::mem::transmute(lcid), ::core::mem::transmute(wflags), ::core::mem::transmute(pdispparams), ::core::mem::transmute(pvarresult), ::core::mem::transmute(pexcepinfo), ::core::mem::transmute(puargerr)).ok()
    }
}
impl ::core::convert::From<IDispatch> for ::windows::core::IUnknown {
    fn from(value: IDispatch) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDispatch> for ::windows::core::IUnknown {
    fn from(value: &IDispatch) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IDispatch {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IDispatch {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IDispatch {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDispatch {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDispatch {}
impl ::core::fmt::Debug for IDispatch {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDispatch").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IDispatch {
    type Vtable = IDispatchVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x00020400_0000_0000_c000_000000000046);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDispatchVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pctinfo: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const DISPPARAMS, pvarresult: *mut VARIANT, pexcepinfo: *mut EXCEPINFO, puargerr: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Ole")))] usize,
);
#[doc = "*Required features: 'Win32_System_Com'*"]
#[repr(transparent)]
pub struct IEnumCATEGORYINFO(::windows::core::IUnknown);
impl IEnumCATEGORYINFO {
    #[doc = "*Required features: 'Win32_System_Com'*"]
    pub unsafe fn Next(&self, celt: u32, rgelt: *mut CATEGORYINFO, pceltfetched: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(celt), ::core::mem::transmute(rgelt), ::core::mem::transmute(pceltfetched)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Com'*"]
    pub unsafe fn Skip(&self, celt: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(celt)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Com'*"]
    pub unsafe fn Reset(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Com'*"]
    pub unsafe fn Clone(&self) -> ::windows::core::Result<IEnumCATEGORYINFO> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IEnumCATEGORYINFO>(result__)
    }
}
impl ::core::convert::From<IEnumCATEGORYINFO> for ::windows::core::IUnknown {
    fn from(value: IEnumCATEGORYINFO) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IEnumCATEGORYINFO> for ::windows::core::IUnknown {
    fn from(value: &IEnumCATEGORYINFO) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IEnumCATEGORYINFO {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IEnumCATEGORYINFO {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IEnumCATEGORYINFO {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IEnumCATEGORYINFO {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IEnumCATEGORYINFO {}
impl ::core::fmt::Debug for IEnumCATEGORYINFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IEnumCATEGORYINFO").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IEnumCATEGORYINFO {
    type Vtable = IEnumCATEGORYINFOVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0002e011_0000_0000_c000_000000000046);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnumCATEGORYINFOVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, celt: u32, rgelt: *mut CATEGORYINFO, pceltfetched: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: 'Win32_System_Com'*"]
#[repr(transparent)]
pub struct IEnumConnectionPoints(::windows::core::IUnknown);
impl IEnumConnectionPoints {
    #[doc = "*Required features: 'Win32_System_Com'*"]
    pub unsafe fn Next(&self, cconnections: u32, ppcp: *mut ::core::option::Option<IConnectionPoint>, pcfetched: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(cconnections), ::core::mem::transmute(ppcp), ::core::mem::transmute(pcfetched)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Com'*"]
    pub unsafe fn Skip(&self, cconnections: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(cconnections)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Com'*"]
    pub unsafe fn Reset(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Com'*"]
    pub unsafe fn Clone(&self) -> ::windows::core::Result<IEnumConnectionPoints> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IEnumConnectionPoints>(result__)
    }
}
impl ::core::convert::From<IEnumConnectionPoints> for ::windows::core::IUnknown {
    fn from(value: IEnumConnectionPoints) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IEnumConnectionPoints> for ::windows::core::IUnknown {
    fn from(value: &IEnumConnectionPoints) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IEnumConnectionPoints {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IEnumConnectionPoints {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IEnumConnectionPoints {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IEnumConnectionPoints {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IEnumConnectionPoints {}
impl ::core::fmt::Debug for IEnumConnectionPoints {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IEnumConnectionPoints").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IEnumConnectionPoints {
    type Vtable = IEnumConnectionPointsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb196b285_bab4_101a_b69c_00aa00341d07);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnumConnectionPointsVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cconnections: u32, ppcp: *mut ::windows::core::RawPtr, pcfetched: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cconnections: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: 'Win32_System_Com'*"]
#[repr(transparent)]
pub struct IEnumConnections(::windows::core::IUnknown);
impl IEnumConnections {
    #[doc = "*Required features: 'Win32_System_Com'*"]
    pub unsafe fn Next(&self, cconnections: u32, rgcd: *mut CONNECTDATA, pcfetched: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(cconnections), ::core::mem::transmute(rgcd), ::core::mem::transmute(pcfetched)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Com'*"]
    pub unsafe fn Skip(&self, cconnections: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(cconnections)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Com'*"]
    pub unsafe fn Reset(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Com'*"]
    pub unsafe fn Clone(&self) -> ::windows::core::Result<IEnumConnections> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IEnumConnections>(result__)
    }
}
impl ::core::convert::From<IEnumConnections> for ::windows::core::IUnknown {
    fn from(value: IEnumConnections) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IEnumConnections> for ::windows::core::IUnknown {
    fn from(value: &IEnumConnections) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IEnumConnections {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IEnumConnections {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IEnumConnections {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IEnumConnections {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IEnumConnections {}
impl ::core::fmt::Debug for IEnumConnections {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IEnumConnections").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IEnumConnections {
    type Vtable = IEnumConnectionsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb196b287_bab4_101a_b69c_00aa00341d07);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnumConnectionsVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cconnections: u32, rgcd: *mut CONNECTDATA, pcfetched: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cconnections: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(C)]
pub struct IEnumContextProps(pub u8);
#[doc = "*Required features: 'Win32_System_Com'*"]
#[repr(transparent)]
pub struct IEnumFORMATETC(::windows::core::IUnknown);
impl IEnumFORMATETC {
    #[doc = "*Required features: 'Win32_System_Com'*"]
    pub unsafe fn Next(&self, celt: u32, rgelt: *mut FORMATETC, pceltfetched: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(celt), ::core::mem::transmute(rgelt), ::core::mem::transmute(pceltfetched)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Com'*"]
    pub unsafe fn Skip(&self, celt: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(celt)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Com'*"]
    pub unsafe fn Reset(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Com'*"]
    pub unsafe fn Clone(&self) -> ::windows::core::Result<IEnumFORMATETC> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IEnumFORMATETC>(result__)
    }
}
impl ::core::convert::From<IEnumFORMATETC> for ::windows::core::IUnknown {
    fn from(value: IEnumFORMATETC) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IEnumFORMATETC> for ::windows::core::IUnknown {
    fn from(value: &IEnumFORMATETC) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IEnumFORMATETC {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IEnumFORMATETC {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IEnumFORMATETC {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IEnumFORMATETC {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IEnumFORMATETC {}
impl ::core::fmt::Debug for IEnumFORMATETC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IEnumFORMATETC").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IEnumFORMATETC {
    type Vtable = IEnumFORMATETCVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x00000103_0000_0000_c000_000000000046);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnumFORMATETCVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, celt: u32, rgelt: *mut FORMATETC, pceltfetched: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: 'Win32_System_Com'*"]
#[repr(transparent)]
pub struct IEnumGUID(::windows::core::IUnknown);
impl IEnumGUID {
    #[doc = "*Required features: 'Win32_System_Com'*"]
    pub unsafe fn Next(&self, celt: u32, rgelt: *mut ::windows::core::GUID, pceltfetched: *mut u32) -> ::windows::core::HRESULT {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(celt), ::core::mem::transmute(rgelt), ::core::mem::transmute(pceltfetched)))
    }
    #[doc = "*Required features: 'Win32_System_Com'*"]
    pub unsafe fn Skip(&self, celt: u32) -> ::windows::core::HRESULT {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(celt)))
    }
    #[doc = "*Required features: 'Win32_System_Com'*"]
    pub unsafe fn Reset(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Com'*"]
    pub unsafe fn Clone(&self) -> ::windows::core::Result<IEnumGUID> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IEnumGUID>(result__)
    }
}
impl ::core::convert::From<IEnumGUID> for ::windows::core::IUnknown {
    fn from(value: IEnumGUID) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IEnumGUID> for ::windows::core::IUnknown {
    fn from(value: &IEnumGUID) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IEnumGUID {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IEnumGUID {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IEnumGUID {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IEnumGUID {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IEnumGUID {}
impl ::core::fmt::Debug for IEnumGUID {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IEnumGUID").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IEnumGUID {
    type Vtable = IEnumGUIDVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0002e000_0000_0000_c000_000000000046);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnumGUIDVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, celt: u32, rgelt: *mut ::windows::core::GUID, pceltfetched: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: 'Win32_System_Com'*"]
#[repr(transparent)]
pub struct IEnumMoniker(::windows::core::IUnknown);
impl IEnumMoniker {
    #[doc = "*Required features: 'Win32_System_Com'*"]
    pub unsafe fn Next(&self, celt: u32, rgelt: *mut ::core::option::Option<IMoniker>, pceltfetched: *mut u32) -> ::windows::core::HRESULT {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(celt), ::core::mem::transmute(rgelt), ::core::mem::transmute(pceltfetched)))
    }
    #[doc = "*Required features: 'Win32_System_Com'*"]
    pub unsafe fn Skip(&self, celt: u32) -> ::windows::core::HRESULT {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(celt)))
    }
    #[doc = "*Required features: 'Win32_System_Com'*"]
    pub unsafe fn Reset(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Com'*"]
    pub unsafe fn Clone(&self) -> ::windows::core::Result<IEnumMoniker> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IEnumMoniker>(result__)
    }
}
impl ::core::convert::From<IEnumMoniker> for ::windows::core::IUnknown {
    fn from(value: IEnumMoniker) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IEnumMoniker> for ::windows::core::IUnknown {
    fn from(value: &IEnumMoniker) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IEnumMoniker {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IEnumMoniker {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IEnumMoniker {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IEnumMoniker {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IEnumMoniker {}
impl ::core::fmt::Debug for IEnumMoniker {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IEnumMoniker").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IEnumMoniker {
    type Vtable = IEnumMonikerVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x00000102_0000_0000_c000_000000000046);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnumMonikerVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, celt: u32, rgelt: *mut ::windows::core::RawPtr, pceltfetched: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: 'Win32_System_Com'*"]
#[repr(transparent)]
pub struct IEnumSTATDATA(::windows::core::IUnknown);
impl IEnumSTATDATA {
    #[doc = "*Required features: 'Win32_System_Com'*"]
    pub unsafe fn Next(&self, celt: u32, rgelt: *mut STATDATA, pceltfetched: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(celt), ::core::mem::transmute(rgelt), ::core::mem::transmute(pceltfetched)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Com'*"]
    pub unsafe fn Skip(&self, celt: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(celt)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Com'*"]
    pub unsafe fn Reset(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Com'*"]
    pub unsafe fn Clone(&self) -> ::windows::core::Result<IEnumSTATDATA> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IEnumSTATDATA>(result__)
    }
}
impl ::core::convert::From<IEnumSTATDATA> for ::windows::core::IUnknown {
    fn from(value: IEnumSTATDATA) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IEnumSTATDATA> for ::windows::core::IUnknown {
    fn from(value: &IEnumSTATDATA) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IEnumSTATDATA {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IEnumSTATDATA {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IEnumSTATDATA {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IEnumSTATDATA {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IEnumSTATDATA {}
impl ::core::fmt::Debug for IEnumSTATDATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IEnumSTATDATA").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IEnumSTATDATA {
    type Vtable = IEnumSTATDATAVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x00000105_0000_0000_c000_000000000046);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnumSTATDATAVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, celt: u32, rgelt: *mut STATDATA, pceltfetched: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: 'Win32_System_Com'*"]
#[repr(transparent)]
pub struct IEnumString(::windows::core::IUnknown);
impl IEnumString {
    #[doc = "*Required features: 'Win32_System_Com', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Next(&self, celt: u32, rgelt: *mut super::super::Foundation::PWSTR, pceltfetched: *mut u32) -> ::windows::core::HRESULT {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(celt), ::core::mem::transmute(rgelt), ::core::mem::transmute(pceltfetched)))
    }
    #[doc = "*Required features: 'Win32_System_Com'*"]
    pub unsafe fn Skip(&self, celt: u32) -> ::windows::core::HRESULT {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(celt)))
    }
    #[doc = "*Required features: 'Win32_System_Com'*"]
    pub unsafe fn Reset(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Com'*"]
    pub unsafe fn Clone(&self) -> ::windows::core::Result<IEnumString> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IEnumString>(result__)
    }
}
impl ::core::convert::From<IEnumString> for ::windows::core::IUnknown {
    fn from(value: IEnumString) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IEnumString> for ::windows::core::IUnknown {
    fn from(value: &IEnumString) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IEnumString {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IEnumString {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IEnumString {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IEnumString {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IEnumString {}
impl ::core::fmt::Debug for IEnumString {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IEnumString").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IEnumString {
    type Vtable = IEnumStringVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x00000101_0000_0000_c000_000000000046);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnumStringVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, celt: u32, rgelt: *mut super::super::Foundation::PWSTR, pceltfetched: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: 'Win32_System_Com'*"]
#[repr(transparent)]
pub struct IEnumUnknown(::windows::core::IUnknown);
impl IEnumUnknown {
    #[doc = "*Required features: 'Win32_System_Com'*"]
    pub unsafe fn Next(&self, celt: u32, rgelt: *mut ::core::option::Option<::windows::core::IUnknown>, pceltfetched: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(celt), ::core::mem::transmute(rgelt), ::core::mem::transmute(pceltfetched)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Com'*"]
    pub unsafe fn Skip(&self, celt: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(celt)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Com'*"]
    pub unsafe fn Reset(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Com'*"]
    pub unsafe fn Clone(&self) -> ::windows::core::Result<IEnumUnknown> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IEnumUnknown>(result__)
    }
}
impl ::core::convert::From<IEnumUnknown> for ::windows::core::IUnknown {
    fn from(value: IEnumUnknown) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IEnumUnknown> for ::windows::core::IUnknown {
    fn from(value: &IEnumUnknown) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IEnumUnknown {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IEnumUnknown {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IEnumUnknown {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IEnumUnknown {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IEnumUnknown {}
impl ::core::fmt::Debug for IEnumUnknown {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IEnumUnknown").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IEnumUnknown {
    type Vtable = IEnumUnknownVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x00000100_0000_0000_c000_000000000046);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnumUnknownVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, celt: u32, rgelt: *mut *mut ::core::ffi::c_void, pceltfetched: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: 'Win32_System_Com'*"]
#[repr(transparent)]
pub struct IErrorInfo(::windows::core::IUnknown);
impl IErrorInfo {
    #[doc = "*Required features: 'Win32_System_Com'*"]
    pub unsafe fn GetGUID(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::GUID>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Com', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetSource(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Com', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetDescription(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Com', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetHelpFile(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Com'*"]
    pub unsafe fn GetHelpContext(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
}
impl ::core::convert::From<IErrorInfo> for ::windows::core::IUnknown {
    fn from(value: IErrorInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IErrorInfo> for ::windows::core::IUnknown {
    fn from(value: &IErrorInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IErrorInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IErrorInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IErrorInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IErrorInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IErrorInfo {}
impl ::core::fmt::Debug for IErrorInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IErrorInfo").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IErrorInfo {
    type Vtable = IErrorInfoVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1cf2b120_547d_101b_8e65_08002b2bd119);
}
#[repr(C)]
#[doc(hidden)]
pub struct IErrorInfoVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pguid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrsource: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrdescription: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrhelpfile: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwhelpcontext: *mut u32) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: 'Win32_System_Com'*"]
#[repr(transparent)]
pub struct IErrorLog(::windows::core::IUnknown);
impl IErrorLog {
    #[doc = "*Required features: 'Win32_System_Com', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn AddError<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pszpropname: Param0, pexcepinfo: *const EXCEPINFO) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), pszpropname.into_param().abi(), ::core::mem::transmute(pexcepinfo)).ok()
    }
}
impl ::core::convert::From<IErrorLog> for ::windows::core::IUnknown {
    fn from(value: IErrorLog) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IErrorLog> for ::windows::core::IUnknown {
    fn from(value: &IErrorLog) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IErrorLog {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IErrorLog {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IErrorLog {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IErrorLog {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IErrorLog {}
impl ::core::fmt::Debug for IErrorLog {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IErrorLog").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IErrorLog {
    type Vtable = IErrorLogVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3127ca40_446e_11ce_8135_00aa004bb851);
}
#[repr(C)]
#[doc(hidden)]
pub struct IErrorLogVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszpropname: super::super::Foundation::PWSTR, pexcepinfo: *const EXCEPINFO) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: 'Win32_System_Com'*"]
#[repr(transparent)]
pub struct IExternalConnection(::windows::core::IUnknown);
impl IExternalConnection {
    #[doc = "*Required features: 'Win32_System_Com'*"]
    pub unsafe fn AddConnection(&self, extconn: u32, reserved: u32) -> u32 {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(extconn), ::core::mem::transmute(reserved)))
    }
    #[doc = "*Required features: 'Win32_System_Com', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ReleaseConnection<'a, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, extconn: u32, reserved: u32, flastreleasecloses: Param2) -> u32 {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(extconn), ::core::mem::transmute(reserved), flastreleasecloses.into_param().abi()))
    }
}
impl ::core::convert::From<IExternalConnection> for ::windows::core::IUnknown {
    fn from(value: IExternalConnection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IExternalConnection> for ::windows::core::IUnknown {
    fn from(value: &IExternalConnection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IExternalConnection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IExternalConnection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IExternalConnection {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IExternalConnection {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IExternalConnection {}
impl ::core::fmt::Debug for IExternalConnection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IExternalConnection").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IExternalConnection {
    type Vtable = IExternalConnectionVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x00000019_0000_0000_c000_000000000046);
}
#[repr(C)]
#[doc(hidden)]
pub struct IExternalConnectionVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, extconn: u32, reserved: u32) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, extconn: u32, reserved: u32, flastreleasecloses: super::super::Foundation::BOOL) -> u32,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: 'Win32_System_Com'*"]
#[repr(transparent)]
pub struct IFastRundown(::windows::core::IUnknown);
impl IFastRundown {}
impl ::core::convert::From<IFastRundown> for ::windows::core::IUnknown {
    fn from(value: IFastRundown) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IFastRundown> for ::windows::core::IUnknown {
    fn from(value: &IFastRundown) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IFastRundown {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IFastRundown {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IFastRundown {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IFastRundown {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IFastRundown {}
impl ::core::fmt::Debug for IFastRundown {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFastRundown").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IFastRundown {
    type Vtable = IFastRundownVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x00000040_0000_0000_c000_000000000046);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFastRundownVtbl(pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT, pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32, pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32);
#[doc = "*Required features: 'Win32_System_Com'*"]
#[repr(transparent)]
pub struct IForegroundTransfer(::windows::core::IUnknown);
impl IForegroundTransfer {
    #[doc = "*Required features: 'Win32_System_Com'*"]
    pub unsafe fn AllowForegroundTransfer(&self, lpvreserved: *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(lpvreserved)).ok()
    }
}
impl ::core::convert::From<IForegroundTransfer> for ::windows::core::IUnknown {
    fn from(value: IForegroundTransfer) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IForegroundTransfer> for ::windows::core::IUnknown {
    fn from(value: &IForegroundTransfer) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IForegroundTransfer {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IForegroundTransfer {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IForegroundTransfer {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IForegroundTransfer {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IForegroundTransfer {}
impl ::core::fmt::Debug for IForegroundTransfer {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IForegroundTransfer").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IForegroundTransfer {
    type Vtable = IForegroundTransferVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x00000145_0000_0000_c000_000000000046);
}
#[repr(C)]
#[doc(hidden)]
pub struct IForegroundTransferVtbl(pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT, pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32, pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32, pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lpvreserved: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT);
#[doc = "*Required features: 'Win32_System_Com'*"]
#[repr(transparent)]
pub struct IGlobalInterfaceTable(::windows::core::IUnknown);
impl IGlobalInterfaceTable {
    #[doc = "*Required features: 'Win32_System_Com'*"]
    pub unsafe fn RegisterInterfaceInGlobal<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>>(&self, punk: Param0, riid: *const ::windows::core::GUID) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), punk.into_param().abi(), ::core::mem::transmute(riid), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Com'*"]
    pub unsafe fn RevokeInterfaceFromGlobal(&self, dwcookie: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwcookie)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Com'*"]
    pub unsafe fn GetInterfaceFromGlobal(&self, dwcookie: u32, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwcookie), ::core::mem::transmute(riid), ::core::mem::transmute(ppv)).ok()
    }
}
impl ::core::convert::From<IGlobalInterfaceTable> for ::windows::core::IUnknown {
    fn from(value: IGlobalInterfaceTable) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IGlobalInterfaceTable> for ::windows::core::IUnknown {
    fn from(value: &IGlobalInterfaceTable) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IGlobalInterfaceTable {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IGlobalInterfaceTable {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IGlobalInterfaceTable {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IGlobalInterfaceTable {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IGlobalInterfaceTable {}
impl ::core::fmt::Debug for IGlobalInterfaceTable {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IGlobalInterfaceTable").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IGlobalInterfaceTable {
    type Vtable = IGlobalInterfaceTableVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x00000146_0000_0000_c000_000000000046);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGlobalInterfaceTableVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, punk: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, pdwcookie: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwcookie: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwcookie: u32, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: 'Win32_System_Com'*"]
#[repr(transparent)]
pub struct IGlobalOptions(::windows::core::IUnknown);
impl IGlobalOptions {
    #[doc = "*Required features: 'Win32_System_Com'*"]
    pub unsafe fn Set(&self, dwproperty: GLOBALOPT_PROPERTIES, dwvalue: usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwproperty), ::core::mem::transmute(dwvalue)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Com'*"]
    pub unsafe fn Query(&self, dwproperty: GLOBALOPT_PROPERTIES) -> ::windows::core::Result<usize> {
        let mut result__: usize = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwproperty), ::core::mem::transmute(&mut result__)).from_abi::<usize>(result__)
    }
}
impl ::core::convert::From<IGlobalOptions> for ::windows::core::IUnknown {
    fn from(value: IGlobalOptions) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IGlobalOptions> for ::windows::core::IUnknown {
    fn from(value: &IGlobalOptions) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IGlobalOptions {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IGlobalOptions {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IGlobalOptions {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IGlobalOptions {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IGlobalOptions {}
impl ::core::fmt::Debug for IGlobalOptions {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IGlobalOptions").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IGlobalOptions {
    type Vtable = IGlobalOptionsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0000015b_0000_0000_c000_000000000046);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGlobalOptionsVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwproperty: GLOBALOPT_PROPERTIES, dwvalue: usize) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwproperty: GLOBALOPT_PROPERTIES, pdwvalue: *mut usize) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: 'Win32_System_Com', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn IIDFromString<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(lpsz: Param0) -> ::windows::core::Result<::windows::core::GUID> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn IIDFromString(lpsz: super::super::Foundation::PWSTR, lpiid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT;
        }
        let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
        IIDFromString(lpsz.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::GUID>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_System_Com'*"]
#[repr(transparent)]
pub struct IInitializeSpy(::windows::core::IUnknown);
impl IInitializeSpy {
    #[doc = "*Required features: 'Win32_System_Com'*"]
    pub unsafe fn PreInitialize(&self, dwcoinit: u32, dwcurthreadaptrefs: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwcoinit), ::core::mem::transmute(dwcurthreadaptrefs)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Com'*"]
    pub unsafe fn PostInitialize(&self, hrcoinit: ::windows::core::HRESULT, dwcoinit: u32, dwnewthreadaptrefs: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(hrcoinit), ::core::mem::transmute(dwcoinit), ::core::mem::transmute(dwnewthreadaptrefs)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Com'*"]
    pub unsafe fn PreUninitialize(&self, dwcurthreadaptrefs: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwcurthreadaptrefs)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Com'*"]
    pub unsafe fn PostUninitialize(&self, dwnewthreadaptrefs: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwnewthreadaptrefs)).ok()
    }
}
impl ::core::convert::From<IInitializeSpy> for ::windows::core::IUnknown {
    fn from(value: IInitializeSpy) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IInitializeSpy> for ::windows::core::IUnknown {
    fn from(value: &IInitializeSpy) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IInitializeSpy {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IInitializeSpy {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IInitializeSpy {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IInitializeSpy {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IInitializeSpy {}
impl ::core::fmt::Debug for IInitializeSpy {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IInitializeSpy").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IInitializeSpy {
    type Vtable = IInitializeSpyVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x00000034_0000_0000_c000_000000000046);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInitializeSpyVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwcoinit: u32, dwcurthreadaptrefs: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hrcoinit: ::windows::core::HRESULT, dwcoinit: u32, dwnewthreadaptrefs: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwcurthreadaptrefs: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwnewthreadaptrefs: u32) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: 'Win32_System_Com'*"]
#[repr(transparent)]
pub struct IInternalUnknown(::windows::core::IUnknown);
impl IInternalUnknown {
    #[doc = "*Required features: 'Win32_System_Com'*"]
    pub unsafe fn QueryInternalInterface(&self, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(riid), ::core::mem::transmute(ppv)).ok()
    }
}
impl ::core::convert::From<IInternalUnknown> for ::windows::core::IUnknown {
    fn from(value: IInternalUnknown) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IInternalUnknown> for ::windows::core::IUnknown {
    fn from(value: &IInternalUnknown) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IInternalUnknown {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IInternalUnknown {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IInternalUnknown {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IInternalUnknown {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IInternalUnknown {}
impl ::core::fmt::Debug for IInternalUnknown {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IInternalUnknown").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IInternalUnknown {
    type Vtable = IInternalUnknownVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x00000021_0000_0000_c000_000000000046);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInternalUnknownVtbl(pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT, pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32, pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32, pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT);
#[doc = "*Required features: 'Win32_System_Com'*"]
#[repr(transparent)]
pub struct IMachineGlobalObjectTable(::windows::core::IUnknown);
impl IMachineGlobalObjectTable {
    #[doc = "*Required features: 'Win32_System_Com', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RegisterObject<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param2: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>>(&self, clsid: *const ::windows::core::GUID, identifier: Param1, object: Param2) -> ::windows::core::Result<*mut MachineGlobalObjectTableRegistrationToken__> {
        let mut result__: *mut MachineGlobalObjectTableRegistrationToken__ = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(clsid), identifier.into_param().abi(), object.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<*mut MachineGlobalObjectTableRegistrationToken__>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Com', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetObject<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, T: ::windows::core::Interface>(&self, clsid: *const ::windows::core::GUID, identifier: Param1) -> ::windows::core::Result<T> {
        let mut result__ = ::core::option::Option::None;
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(clsid), identifier.into_param().abi(), &<T as ::windows::core::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
    #[doc = "*Required features: 'Win32_System_Com'*"]
    pub unsafe fn RevokeObject(&self, token: *const MachineGlobalObjectTableRegistrationToken__) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(token)).ok()
    }
}
impl ::core::convert::From<IMachineGlobalObjectTable> for ::windows::core::IUnknown {
    fn from(value: IMachineGlobalObjectTable) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IMachineGlobalObjectTable> for ::windows::core::IUnknown {
    fn from(value: &IMachineGlobalObjectTable) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IMachineGlobalObjectTable {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IMachineGlobalObjectTable {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IMachineGlobalObjectTable {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IMachineGlobalObjectTable {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMachineGlobalObjectTable {}
impl ::core::fmt::Debug for IMachineGlobalObjectTable {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMachineGlobalObjectTable").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IMachineGlobalObjectTable {
    type Vtable = IMachineGlobalObjectTableVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x26d709ac_f70b_4421_a96f_d2878fafb00d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMachineGlobalObjectTableVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, clsid: *const ::windows::core::GUID, identifier: super::super::Foundation::PWSTR, object: *mut ::core::ffi::c_void, token: *mut *mut MachineGlobalObjectTableRegistrationToken__) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, clsid: *const ::windows::core::GUID, identifier: super::super::Foundation::PWSTR, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: *const MachineGlobalObjectTableRegistrationToken__) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: 'Win32_System_Com'*"]
#[repr(transparent)]
pub struct IMalloc(::windows::core::IUnknown);
impl IMalloc {
    #[doc = "*Required features: 'Win32_System_Com'*"]
    pub unsafe fn Alloc(&self, cb: usize) -> *mut ::core::ffi::c_void {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(cb)))
    }
    #[doc = "*Required features: 'Win32_System_Com'*"]
    pub unsafe fn Realloc(&self, pv: *const ::core::ffi::c_void, cb: usize) -> *mut ::core::ffi::c_void {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(pv), ::core::mem::transmute(cb)))
    }
    #[doc = "*Required features: 'Win32_System_Com'*"]
    pub unsafe fn Free(&self, pv: *const ::core::ffi::c_void) {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(pv))
    }
    #[doc = "*Required features: 'Win32_System_Com'*"]
    pub unsafe fn GetSize(&self, pv: *const ::core::ffi::c_void) -> usize {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(pv)))
    }
    #[doc = "*Required features: 'Win32_System_Com'*"]
    pub unsafe fn DidAlloc(&self, pv: *const ::core::ffi::c_void) -> i32 {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(pv)))
    }
    #[doc = "*Required features: 'Win32_System_Com'*"]
    pub unsafe fn HeapMinimize(&self) {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self))
    }
}
impl ::core::convert::From<IMalloc> for ::windows::core::IUnknown {
    fn from(value: IMalloc) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IMalloc> for ::windows::core::IUnknown {
    fn from(value: &IMalloc) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IMalloc {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IMalloc {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IMalloc {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IMalloc {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMalloc {}
impl ::core::fmt::Debug for IMalloc {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMalloc").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IMalloc {
    type Vtable = IMallocVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x00000002_0000_0000_c000_000000000046);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMallocVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cb: usize) -> *mut ::core::ffi::c_void,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pv: *const ::core::ffi::c_void, cb: usize) -> *mut ::core::ffi::c_void,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pv: *const ::core::ffi::c_void),
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pv: *const ::core::ffi::c_void) -> usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pv: *const ::core::ffi::c_void) -> i32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void),
);
#[doc = "*Required features: 'Win32_System_Com'*"]
#[repr(transparent)]
pub struct IMallocSpy(::windows::core::IUnknown);
impl IMallocSpy {
    #[doc = "*Required features: 'Win32_System_Com'*"]
    pub unsafe fn PreAlloc(&self, cbrequest: usize) -> usize {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(cbrequest)))
    }
    #[doc = "*Required features: 'Win32_System_Com'*"]
    pub unsafe fn PostAlloc(&self, pactual: *const ::core::ffi::c_void) -> *mut ::core::ffi::c_void {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(pactual)))
    }
    #[doc = "*Required features: 'Win32_System_Com', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn PreFree<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, prequest: *const ::core::ffi::c_void, fspyed: Param1) -> *mut ::core::ffi::c_void {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(prequest), fspyed.into_param().abi()))
    }
    #[doc = "*Required features: 'Win32_System_Com', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn PostFree<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, fspyed: Param0) {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), fspyed.into_param().abi())
    }
    #[doc = "*Required features: 'Win32_System_Com', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn PreRealloc<'a, Param3: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, prequest: *const ::core::ffi::c_void, cbrequest: usize, ppnewrequest: *mut *mut ::core::ffi::c_void, fspyed: Param3) -> usize {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(prequest), ::core::mem::transmute(cbrequest), ::core::mem::transmute(ppnewrequest), fspyed.into_param().abi()))
    }
    #[doc = "*Required features: 'Win32_System_Com', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn PostRealloc<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, pactual: *const ::core::ffi::c_void, fspyed: Param1) -> *mut ::core::ffi::c_void {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(pactual), fspyed.into_param().abi()))
    }
    #[doc = "*Required features: 'Win32_System_Com', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn PreGetSize<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, prequest: *const ::core::ffi::c_void, fspyed: Param1) -> *mut ::core::ffi::c_void {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(prequest), fspyed.into_param().abi()))
    }
    #[doc = "*Required features: 'Win32_System_Com', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn PostGetSize<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, cbactual: usize, fspyed: Param1) -> usize {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), ::core::mem::transmute(cbactual), fspyed.into_param().abi()))
    }
    #[doc = "*Required features: 'Win32_System_Com', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn PreDidAlloc<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, prequest: *const ::core::ffi::c_void, fspyed: Param1) -> *mut ::core::ffi::c_void {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), ::core::mem::transmute(prequest), fspyed.into_param().abi()))
    }
    #[doc = "*Required features: 'Win32_System_Com', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn PostDidAlloc<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, prequest: *const ::core::ffi::c_void, fspyed: Param1, factual: i32) -> i32 {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), ::core::mem::transmute(prequest), fspyed.into_param().abi(), ::core::mem::transmute(factual)))
    }
    #[doc = "*Required features: 'Win32_System_Com'*"]
    pub unsafe fn PreHeapMinimize(&self) {
        (::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self))
    }
    #[doc = "*Required features: 'Win32_System_Com'*"]
    pub unsafe fn PostHeapMinimize(&self) {
        (::windows::core::Interface::vtable(self).14)(::core::mem::transmute_copy(self))
    }
}
impl ::core::convert::From<IMallocSpy> for ::windows::core::IUnknown {
    fn from(value: IMallocSpy) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IMallocSpy> for ::windows::core::IUnknown {
    fn from(value: &IMallocSpy) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IMallocSpy {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IMallocSpy {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IMallocSpy {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IMallocSpy {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMallocSpy {}
impl ::core::fmt::Debug for IMallocSpy {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMallocSpy").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IMallocSpy {
    type Vtable = IMallocSpyVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0000001d_0000_0000_c000_000000000046);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMallocSpyVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cbrequest: usize) -> usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pactual: *const ::core::ffi::c_void) -> *mut ::core::ffi::c_void,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, prequest: *const ::core::ffi::c_void, fspyed: super::super::Foundation::BOOL) -> *mut ::core::ffi::c_void,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fspyed: super::super::Foundation::BOOL),
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, prequest: *const ::core::ffi::c_void, cbrequest: usize, ppnewrequest: *mut *mut ::core::ffi::c_void, fspyed: super::super::Foundation::BOOL) -> usize,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pactual: *const ::core::ffi::c_void, fspyed: super::super::Foundation::BOOL) -> *mut ::core::ffi::c_void,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, prequest: *const ::core::ffi::c_void, fspyed: super::super::Foundation::BOOL) -> *mut ::core::ffi::c_void,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cbactual: usize, fspyed: super::super::Foundation::BOOL) -> usize,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, prequest: *const ::core::ffi::c_void, fspyed: super::super::Foundation::BOOL) -> *mut ::core::ffi::c_void,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, prequest: *const ::core::ffi::c_void, fspyed: super::super::Foundation::BOOL, factual: i32) -> i32,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void),
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void),
);
#[doc = "*Required features: 'Win32_System_Com'*"]
#[repr(transparent)]
pub struct IMoniker(::windows::core::IUnknown);
impl IMoniker {
    #[doc = "*Required features: 'Win32_System_Com'*"]
    pub unsafe fn GetClassID(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::GUID>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Com'*"]
    pub unsafe fn IsDirty(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Com'*"]
    pub unsafe fn Load<'a, Param0: ::windows::core::IntoParam<'a, IStream>>(&self, pstm: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), pstm.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_System_Com', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Save<'a, Param0: ::windows::core::IntoParam<'a, IStream>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, pstm: Param0, fcleardirty: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), pstm.into_param().abi(), fcleardirty.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_System_Com'*"]
    pub unsafe fn GetSizeMax(&self) -> ::windows::core::Result<u64> {
        let mut result__: u64 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u64>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Com'*"]
    pub unsafe fn BindToObject<'a, Param0: ::windows::core::IntoParam<'a, IBindCtx>, Param1: ::windows::core::IntoParam<'a, IMoniker>>(&self, pbc: Param0, pmktoleft: Param1, riidresult: *const ::windows::core::GUID, ppvresult: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), pbc.into_param().abi(), pmktoleft.into_param().abi(), ::core::mem::transmute(riidresult), ::core::mem::transmute(ppvresult)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Com'*"]
    pub unsafe fn BindToStorage<'a, Param0: ::windows::core::IntoParam<'a, IBindCtx>, Param1: ::windows::core::IntoParam<'a, IMoniker>>(&self, pbc: Param0, pmktoleft: Param1, riid: *const ::windows::core::GUID, ppvobj: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), pbc.into_param().abi(), pmktoleft.into_param().abi(), ::core::mem::transmute(riid), ::core::mem::transmute(ppvobj)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Com'*"]
    pub unsafe fn Reduce<'a, Param0: ::windows::core::IntoParam<'a, IBindCtx>>(&self, pbc: Param0, dwreducehowfar: u32, ppmktoleft: *mut ::core::option::Option<IMoniker>, ppmkreduced: *mut ::core::option::Option<IMoniker>) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), pbc.into_param().abi(), ::core::mem::transmute(dwreducehowfar), ::core::mem::transmute(ppmktoleft), ::core::mem::transmute(ppmkreduced)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Com', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ComposeWith<'a, Param0: ::windows::core::IntoParam<'a, IMoniker>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, pmkright: Param0, fonlyifnotgeneric: Param1) -> ::windows::core::Result<IMoniker> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), pmkright.into_param().abi(), fonlyifnotgeneric.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IMoniker>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Com', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Enum<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, fforward: Param0) -> ::windows::core::Result<IEnumMoniker> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), fforward.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IEnumMoniker>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Com'*"]
    pub unsafe fn IsEqual<'a, Param0: ::windows::core::IntoParam<'a, IMoniker>>(&self, pmkothermoniker: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self), pmkothermoniker.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_System_Com'*"]
    pub unsafe fn Hash(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).14)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Com'*"]
    pub unsafe fn IsRunning<'a, Param0: ::windows::core::IntoParam<'a, IBindCtx>, Param1: ::windows::core::IntoParam<'a, IMoniker>, Param2: ::windows::core::IntoParam<'a, IMoniker>>(&self, pbc: Param0, pmktoleft: Param1, pmknewlyrunning: Param2) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).15)(::core::mem::transmute_copy(self), pbc.into_param().abi(), pmktoleft.into_param().abi(), pmknewlyrunning.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_System_Com', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetTimeOfLastChange<'a, Param0: ::windows::core::IntoParam<'a, IBindCtx>, Param1: ::windows::core::IntoParam<'a, IMoniker>>(&self, pbc: Param0, pmktoleft: Param1) -> ::windows::core::Result<super::super::Foundation::FILETIME> {
        let mut result__: super::super::Foundation::FILETIME = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).16)(::core::mem::transmute_copy(self), pbc.into_param().abi(), pmktoleft.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::FILETIME>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Com'*"]
    pub unsafe fn Inverse(&self) -> ::windows::core::Result<IMoniker> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).17)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IMoniker>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Com'*"]
    pub unsafe fn CommonPrefixWith<'a, Param0: ::windows::core::IntoParam<'a, IMoniker>>(&self, pmkother: Param0) -> ::windows::core::Result<IMoniker> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).18)(::core::mem::transmute_copy(self), pmkother.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IMoniker>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Com'*"]
    pub unsafe fn RelativePathTo<'a, Param0: ::windows::core::IntoParam<'a, IMoniker>>(&self, pmkother: Param0) -> ::windows::core::Result<IMoniker> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).19)(::core::mem::transmute_copy(self), pmkother.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IMoniker>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Com', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetDisplayName<'a, Param0: ::windows::core::IntoParam<'a, IBindCtx>, Param1: ::windows::core::IntoParam<'a, IMoniker>>(&self, pbc: Param0, pmktoleft: Param1) -> ::windows::core::Result<super::super::Foundation::PWSTR> {
        let mut result__: super::super::Foundation::PWSTR = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).20)(::core::mem::transmute_copy(self), pbc.into_param().abi(), pmktoleft.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::PWSTR>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Com', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ParseDisplayName<'a, Param0: ::windows::core::IntoParam<'a, IBindCtx>, Param1: ::windows::core::IntoParam<'a, IMoniker>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pbc: Param0, pmktoleft: Param1, pszdisplayname: Param2, pcheaten: *mut u32, ppmkout: *mut ::core::option::Option<IMoniker>) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).21)(::core::mem::transmute_copy(self), pbc.into_param().abi(), pmktoleft.into_param().abi(), pszdisplayname.into_param().abi(), ::core::mem::transmute(pcheaten), ::core::mem::transmute(ppmkout)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Com'*"]
    pub unsafe fn IsSystemMoniker(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).22)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
}
impl ::core::convert::From<IMoniker> for IPersistStream {
    fn from(value: IMoniker) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IMoniker> for IPersistStream {
    fn from(value: &IMoniker) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IPersistStream> for IMoniker {
    fn into_param(self) -> ::windows::core::Param<'a, IPersistStream> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IPersistStream> for &IMoniker {
    fn into_param(self) -> ::windows::core::Param<'a, IPersistStream> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IMoniker> for IPersist {
    fn from(value: IMoniker) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IMoniker> for IPersist {
    fn from(value: &IMoniker) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IPersist> for IMoniker {
    fn into_param(self) -> ::windows::core::Param<'a, IPersist> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IPersist> for &IMoniker {
    fn into_param(self) -> ::windows::core::Param<'a, IPersist> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IMoniker> for ::windows::core::IUnknown {
    fn from(value: IMoniker) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IMoniker> for ::windows::core::IUnknown {
    fn from(value: &IMoniker) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IMoniker {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IMoniker {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IMoniker {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IMoniker {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMoniker {}
impl ::core::fmt::Debug for IMoniker {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMoniker").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IMoniker {
    type Vtable = IMonikerVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0000000f_0000_0000_c000_000000000046);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMonikerVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pclassid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pstm: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pstm: ::windows::core::RawPtr, fcleardirty: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcbsize: *mut u64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbc: ::windows::core::RawPtr, pmktoleft: ::windows::core::RawPtr, riidresult: *const ::windows::core::GUID, ppvresult: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbc: ::windows::core::RawPtr, pmktoleft: ::windows::core::RawPtr, riid: *const ::windows::core::GUID, ppvobj: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbc: ::windows::core::RawPtr, dwreducehowfar: u32, ppmktoleft: *mut ::windows::core::RawPtr, ppmkreduced: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pmkright: ::windows::core::RawPtr, fonlyifnotgeneric: super::super::Foundation::BOOL, ppmkcomposite: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fforward: super::super::Foundation::BOOL, ppenummoniker: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pmkothermoniker: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwhash: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbc: ::windows::core::RawPtr, pmktoleft: ::windows::core::RawPtr, pmknewlyrunning: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbc: ::windows::core::RawPtr, pmktoleft: ::windows::core::RawPtr, pfiletime: *mut super::super::Foundation::FILETIME) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppmk: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pmkother: ::windows::core::RawPtr, ppmkprefix: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pmkother: ::windows::core::RawPtr, ppmkrelpath: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbc: ::windows::core::RawPtr, pmktoleft: ::windows::core::RawPtr, ppszdisplayname: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbc: ::windows::core::RawPtr, pmktoleft: ::windows::core::RawPtr, pszdisplayname: super::super::Foundation::PWSTR, pcheaten: *mut u32, ppmkout: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwmksys: *mut u32) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: 'Win32_System_Com'*"]
#[repr(transparent)]
pub struct IMultiQI(::windows::core::IUnknown);
impl IMultiQI {
    #[doc = "*Required features: 'Win32_System_Com'*"]
    pub unsafe fn QueryMultipleInterfaces(&self, cmqis: u32, pmqis: *mut MULTI_QI) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(cmqis), ::core::mem::transmute(pmqis)).ok()
    }
}
impl ::core::convert::From<IMultiQI> for ::windows::core::IUnknown {
    fn from(value: IMultiQI) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IMultiQI> for ::windows::core::IUnknown {
    fn from(value: &IMultiQI) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IMultiQI {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IMultiQI {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IMultiQI {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IMultiQI {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMultiQI {}
impl ::core::fmt::Debug for IMultiQI {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMultiQI").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IMultiQI {
    type Vtable = IMultiQIVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x00000020_0000_0000_c000_000000000046);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMultiQIVtbl(pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT, pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32, pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32, pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cmqis: u32, pmqis: *mut MULTI_QI) -> ::windows::core::HRESULT);
#[repr(C)]
#[doc = "*Required features: 'Win32_System_Com'*"]
pub struct INTERFACEINFO {
    pub pUnk: ::core::option::Option<::windows::core::IUnknown>,
    pub iid: ::windows::core::GUID,
    pub wMethod: u16,
}
impl ::core::clone::Clone for INTERFACEINFO {
    fn clone(&self) -> Self {
        Self { pUnk: self.pUnk.clone(), iid: self.iid, wMethod: self.wMethod }
    }
}
impl ::core::fmt::Debug for INTERFACEINFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("INTERFACEINFO").field("pUnk", &self.pUnk).field("iid", &self.iid).field("wMethod", &self.wMethod).finish()
    }
}
unsafe impl ::windows::core::Abi for INTERFACEINFO {
    type Abi = ::core::mem::ManuallyDrop<Self>;
}
impl ::core::cmp::PartialEq for INTERFACEINFO {
    fn eq(&self, other: &Self) -> bool {
        self.pUnk == other.pUnk && self.iid == other.iid && self.wMethod == other.wMethod
    }
}
impl ::core::cmp::Eq for INTERFACEINFO {}
impl ::core::default::Default for INTERFACEINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: 'Win32_System_Com'*"]
pub type INVOKEKIND = i32;
#[doc = "*Required features: 'Win32_System_Com'*"]
pub const INVOKE_FUNC: INVOKEKIND = 1i32;
#[doc = "*Required features: 'Win32_System_Com'*"]
pub const INVOKE_PROPERTYGET: INVOKEKIND = 2i32;
#[doc = "*Required features: 'Win32_System_Com'*"]
pub const INVOKE_PROPERTYPUT: INVOKEKIND = 4i32;
#[doc = "*Required features: 'Win32_System_Com'*"]
pub const INVOKE_PROPERTYPUTREF: INVOKEKIND = 8i32;
#[doc = "*Required features: 'Win32_System_Com'*"]
#[repr(transparent)]
pub struct INoMarshal(::windows::core::IUnknown);
impl INoMarshal {}
impl ::core::convert::From<INoMarshal> for ::windows::core::IUnknown {
    fn from(value: INoMarshal) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&INoMarshal> for ::windows::core::IUnknown {
    fn from(value: &INoMarshal) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for INoMarshal {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &INoMarshal {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for INoMarshal {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for INoMarshal {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for INoMarshal {}
impl ::core::fmt::Debug for INoMarshal {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("INoMarshal").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for INoMarshal {
    type Vtable = INoMarshalVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xecc8691b_c1db_4dc0_855e_65f6c551af49);
}
#[repr(C)]
#[doc(hidden)]
pub struct INoMarshalVtbl(pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT, pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32, pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32);
#[doc = "*Required features: 'Win32_System_Com'*"]
#[repr(transparent)]
pub struct IOplockStorage(::windows::core::IUnknown);
impl IOplockStorage {
    #[doc = "*Required features: 'Win32_System_Com', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateStorageEx<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, T: ::windows::core::Interface>(&self, pwcsname: Param0, grfmode: u32, stgfmt: u32, grfattrs: u32) -> ::windows::core::Result<T> {
        let mut result__ = ::core::option::Option::None;
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), pwcsname.into_param().abi(), ::core::mem::transmute(grfmode), ::core::mem::transmute(stgfmt), ::core::mem::transmute(grfattrs), &<T as ::windows::core::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
    #[doc = "*Required features: 'Win32_System_Com', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn OpenStorageEx<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, T: ::windows::core::Interface>(&self, pwcsname: Param0, grfmode: u32, stgfmt: u32, grfattrs: u32) -> ::windows::core::Result<T> {
        let mut result__ = ::core::option::Option::None;
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), pwcsname.into_param().abi(), ::core::mem::transmute(grfmode), ::core::mem::transmute(stgfmt), ::core::mem::transmute(grfattrs), &<T as ::windows::core::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
}
impl ::core::convert::From<IOplockStorage> for ::windows::core::IUnknown {
    fn from(value: IOplockStorage) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IOplockStorage> for ::windows::core::IUnknown {
    fn from(value: &IOplockStorage) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IOplockStorage {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IOplockStorage {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IOplockStorage {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IOplockStorage {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IOplockStorage {}
impl ::core::fmt::Debug for IOplockStorage {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IOplockStorage").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IOplockStorage {
    type Vtable = IOplockStorageVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8d19c834_8879_11d1_83e9_00c04fc2c6d4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IOplockStorageVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwcsname: super::super::Foundation::PWSTR, grfmode: u32, stgfmt: u32, grfattrs: u32, riid: *const ::windows::core::GUID, ppstgopen: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwcsname: super::super::Foundation::PWSTR, grfmode: u32, stgfmt: u32, grfattrs: u32, riid: *const ::windows::core::GUID, ppstgopen: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: 'Win32_System_Com'*"]
#[repr(transparent)]
pub struct IPSFactoryBuffer(::windows::core::IUnknown);
impl IPSFactoryBuffer {
    #[doc = "*Required features: 'Win32_System_Com'*"]
    pub unsafe fn CreateProxy<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>>(&self, punkouter: Param0, riid: *const ::windows::core::GUID, ppproxy: *mut ::core::option::Option<IRpcProxyBuffer>, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), punkouter.into_param().abi(), ::core::mem::transmute(riid), ::core::mem::transmute(ppproxy), ::core::mem::transmute(ppv)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Com'*"]
    pub unsafe fn CreateStub<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>>(&self, riid: *const ::windows::core::GUID, punkserver: Param1) -> ::windows::core::Result<IRpcStubBuffer> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(riid), punkserver.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IRpcStubBuffer>(result__)
    }
}
impl ::core::convert::From<IPSFactoryBuffer> for ::windows::core::IUnknown {
    fn from(value: IPSFactoryBuffer) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IPSFactoryBuffer> for ::windows::core::IUnknown {
    fn from(value: &IPSFactoryBuffer) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IPSFactoryBuffer {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IPSFactoryBuffer {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IPSFactoryBuffer {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IPSFactoryBuffer {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPSFactoryBuffer {}
impl ::core::fmt::Debug for IPSFactoryBuffer {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPSFactoryBuffer").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IPSFactoryBuffer {
    type Vtable = IPSFactoryBufferVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd5f569d0_593b_101a_b569_08002b2dbf7a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPSFactoryBufferVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, punkouter: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, ppproxy: *mut ::windows::core::RawPtr, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, punkserver: *mut ::core::ffi::c_void, ppstub: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: 'Win32_System_Com'*"]
#[repr(transparent)]
pub struct IPersist(::windows::core::IUnknown);
impl IPersist {
    #[doc = "*Required features: 'Win32_System_Com'*"]
    pub unsafe fn GetClassID(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::GUID>(result__)
    }
}
impl ::core::convert::From<IPersist> for ::windows::core::IUnknown {
    fn from(value: IPersist) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IPersist> for ::windows::core::IUnknown {
    fn from(value: &IPersist) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IPersist {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IPersist {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IPersist {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IPersist {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPersist {}
impl ::core::fmt::Debug for IPersist {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPersist").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IPersist {
    type Vtable = IPersistVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0000010c_0000_0000_c000_000000000046);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPersistVtbl(pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT, pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32, pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32, pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pclassid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT);
#[doc = "*Required features: 'Win32_System_Com'*"]
#[repr(transparent)]
pub struct IPersistFile(::windows::core::IUnknown);
impl IPersistFile {
    #[doc = "*Required features: 'Win32_System_Com'*"]
    pub unsafe fn GetClassID(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::GUID>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Com'*"]
    pub unsafe fn IsDirty(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Com', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Load<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pszfilename: Param0, dwmode: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), pszfilename.into_param().abi(), ::core::mem::transmute(dwmode)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Com', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Save<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, pszfilename: Param0, fremember: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), pszfilename.into_param().abi(), fremember.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_System_Com', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SaveCompleted<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pszfilename: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), pszfilename.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_System_Com', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetCurFile(&self) -> ::windows::core::Result<super::super::Foundation::PWSTR> {
        let mut result__: super::super::Foundation::PWSTR = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::PWSTR>(result__)
    }
}
impl ::core::convert::From<IPersistFile> for IPersist {
    fn from(value: IPersistFile) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IPersistFile> for IPersist {
    fn from(value: &IPersistFile) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IPersist> for IPersistFile {
    fn into_param(self) -> ::windows::core::Param<'a, IPersist> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IPersist> for &IPersistFile {
    fn into_param(self) -> ::windows::core::Param<'a, IPersist> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IPersistFile> for ::windows::core::IUnknown {
    fn from(value: IPersistFile) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IPersistFile> for ::windows::core::IUnknown {
    fn from(value: &IPersistFile) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IPersistFile {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IPersistFile {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IPersistFile {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IPersistFile {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPersistFile {}
impl ::core::fmt::Debug for IPersistFile {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPersistFile").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IPersistFile {
    type Vtable = IPersistFileVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0000010b_0000_0000_c000_000000000046);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPersistFileVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pclassid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszfilename: super::super::Foundation::PWSTR, dwmode: u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszfilename: super::super::Foundation::PWSTR, fremember: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszfilename: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppszfilename: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: 'Win32_System_Com'*"]
#[repr(transparent)]
pub struct IPersistMemory(::windows::core::IUnknown);
impl IPersistMemory {
    #[doc = "*Required features: 'Win32_System_Com'*"]
    pub unsafe fn GetClassID(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::GUID>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Com'*"]
    pub unsafe fn IsDirty(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Com'*"]
    pub unsafe fn Load(&self, pmem: *const ::core::ffi::c_void, cbsize: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(pmem), ::core::mem::transmute(cbsize)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Com', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Save<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, pmem: *mut ::core::ffi::c_void, fcleardirty: Param1, cbsize: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(pmem), fcleardirty.into_param().abi(), ::core::mem::transmute(cbsize)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Com'*"]
    pub unsafe fn GetSizeMax(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Com'*"]
    pub unsafe fn InitNew(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self)).ok()
    }
}
impl ::core::convert::From<IPersistMemory> for IPersist {
    fn from(value: IPersistMemory) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IPersistMemory> for IPersist {
    fn from(value: &IPersistMemory) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IPersist> for IPersistMemory {
    fn into_param(self) -> ::windows::core::Param<'a, IPersist> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IPersist> for &IPersistMemory {
    fn into_param(self) -> ::windows::core::Param<'a, IPersist> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IPersistMemory> for ::windows::core::IUnknown {
    fn from(value: IPersistMemory) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IPersistMemory> for ::windows::core::IUnknown {
    fn from(value: &IPersistMemory) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IPersistMemory {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IPersistMemory {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IPersistMemory {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IPersistMemory {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPersistMemory {}
impl ::core::fmt::Debug for IPersistMemory {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPersistMemory").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IPersistMemory {
    type Vtable = IPersistMemoryVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbd1ae5e0_a6ae_11ce_bd37_504200c10000);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPersistMemoryVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pclassid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pmem: *const ::core::ffi::c_void, cbsize: u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pmem: *mut ::core::ffi::c_void, fcleardirty: super::super::Foundation::BOOL, cbsize: u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcbsize: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: 'Win32_System_Com'*"]
#[repr(transparent)]
pub struct IPersistStream(::windows::core::IUnknown);
impl IPersistStream {
    #[doc = "*Required features: 'Win32_System_Com'*"]
    pub unsafe fn GetClassID(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::GUID>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Com'*"]
    pub unsafe fn IsDirty(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Com'*"]
    pub unsafe fn Load<'a, Param0: ::windows::core::IntoParam<'a, IStream>>(&self, pstm: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), pstm.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_System_Com', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Save<'a, Param0: ::windows::core::IntoParam<'a, IStream>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, pstm: Param0, fcleardirty: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), pstm.into_param().abi(), fcleardirty.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_System_Com'*"]
    pub unsafe fn GetSizeMax(&self) -> ::windows::core::Result<u64> {
        let mut result__: u64 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u64>(result__)
    }
}
impl ::core::convert::From<IPersistStream> for IPersist {
    fn from(value: IPersistStream) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IPersistStream> for IPersist {
    fn from(value: &IPersistStream) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IPersist> for IPersistStream {
    fn into_param(self) -> ::windows::core::Param<'a, IPersist> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IPersist> for &IPersistStream {
    fn into_param(self) -> ::windows::core::Param<'a, IPersist> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IPersistStream> for ::windows::core::IUnknown {
    fn from(value: IPersistStream) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IPersistStream> for ::windows::core::IUnknown {
    fn from(value: &IPersistStream) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IPersistStream {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IPersistStream {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IPersistStream {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IPersistStream {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPersistStream {}
impl ::core::fmt::Debug for IPersistStream {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPersistStream").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IPersistStream {
    type Vtable = IPersistStreamVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x00000109_0000_0000_c000_000000000046);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPersistStreamVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pclassid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pstm: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pstm: ::windows::core::RawPtr, fcleardirty: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcbsize: *mut u64) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: 'Win32_System_Com'*"]
#[repr(transparent)]
pub struct IPersistStreamInit(::windows::core::IUnknown);
impl IPersistStreamInit {
    #[doc = "*Required features: 'Win32_System_Com'*"]
    pub unsafe fn GetClassID(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::GUID>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Com'*"]
    pub unsafe fn IsDirty(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Com'*"]
    pub unsafe fn Load<'a, Param0: ::windows::core::IntoParam<'a, IStream>>(&self, pstm: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), pstm.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_System_Com', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Save<'a, Param0: ::windows::core::IntoParam<'a, IStream>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, pstm: Param0, fcleardirty: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), pstm.into_param().abi(), fcleardirty.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_System_Com'*"]
    pub unsafe fn GetSizeMax(&self) -> ::windows::core::Result<u64> {
        let mut result__: u64 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u64>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Com'*"]
    pub unsafe fn InitNew(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self)).ok()
    }
}
impl ::core::convert::From<IPersistStreamInit> for IPersist {
    fn from(value: IPersistStreamInit) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IPersistStreamInit> for IPersist {
    fn from(value: &IPersistStreamInit) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IPersist> for IPersistStreamInit {
    fn into_param(self) -> ::windows::core::Param<'a, IPersist> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IPersist> for &IPersistStreamInit {
    fn into_param(self) -> ::windows::core::Param<'a, IPersist> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IPersistStreamInit> for ::windows::core::IUnknown {
    fn from(value: IPersistStreamInit) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IPersistStreamInit> for ::windows::core::IUnknown {
    fn from(value: &IPersistStreamInit) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IPersistStreamInit {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IPersistStreamInit {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IPersistStreamInit {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IPersistStreamInit {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPersistStreamInit {}
impl ::core::fmt::Debug for IPersistStreamInit {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPersistStreamInit").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IPersistStreamInit {
    type Vtable = IPersistStreamInitVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7fd52380_4e07_101b_ae2d_08002b2ec713);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPersistStreamInitVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pclassid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pstm: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pstm: ::windows::core::RawPtr, fcleardirty: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcbsize: *mut u64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: 'Win32_System_Com'*"]
#[repr(transparent)]
pub struct IPipeByte(::windows::core::IUnknown);
impl IPipeByte {
    #[doc = "*Required features: 'Win32_System_Com'*"]
    pub unsafe fn Pull(&self, buf: *mut u8, crequest: u32, pcreturned: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(buf), ::core::mem::transmute(crequest), ::core::mem::transmute(pcreturned)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Com'*"]
    pub unsafe fn Push(&self, buf: *const u8, csent: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(buf), ::core::mem::transmute(csent)).ok()
    }
}
impl ::core::convert::From<IPipeByte> for ::windows::core::IUnknown {
    fn from(value: IPipeByte) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IPipeByte> for ::windows::core::IUnknown {
    fn from(value: &IPipeByte) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IPipeByte {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IPipeByte {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IPipeByte {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IPipeByte {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPipeByte {}
impl ::core::fmt::Debug for IPipeByte {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPipeByte").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IPipeByte {
    type Vtable = IPipeByteVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdb2f3aca_2f86_11d1_8e04_00c04fb9989a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPipeByteVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, buf: *mut u8, crequest: u32, pcreturned: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, buf: *const u8, csent: u32) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: 'Win32_System_Com'*"]
#[repr(transparent)]
pub struct IPipeDouble(::windows::core::IUnknown);
impl IPipeDouble {
    #[doc = "*Required features: 'Win32_System_Com'*"]
    pub unsafe fn Pull(&self, buf: *mut f64, crequest: u32, pcreturned: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(buf), ::core::mem::transmute(crequest), ::core::mem::transmute(pcreturned)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Com'*"]
    pub unsafe fn Push(&self, buf: *const f64, csent: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(buf), ::core::mem::transmute(csent)).ok()
    }
}
impl ::core::convert::From<IPipeDouble> for ::windows::core::IUnknown {
    fn from(value: IPipeDouble) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IPipeDouble> for ::windows::core::IUnknown {
    fn from(value: &IPipeDouble) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IPipeDouble {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IPipeDouble {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IPipeDouble {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IPipeDouble {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPipeDouble {}
impl ::core::fmt::Debug for IPipeDouble {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPipeDouble").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IPipeDouble {
    type Vtable = IPipeDoubleVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdb2f3ace_2f86_11d1_8e04_00c04fb9989a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPipeDoubleVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, buf: *mut f64, crequest: u32, pcreturned: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, buf: *const f64, csent: u32) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: 'Win32_System_Com'*"]
#[repr(transparent)]
pub struct IPipeLong(::windows::core::IUnknown);
impl IPipeLong {
    #[doc = "*Required features: 'Win32_System_Com'*"]
    pub unsafe fn Pull(&self, buf: *mut i32, crequest: u32, pcreturned: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(buf), ::core::mem::transmute(crequest), ::core::mem::transmute(pcreturned)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Com'*"]
    pub unsafe fn Push(&self, buf: *const i32, csent: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(buf), ::core::mem::transmute(csent)).ok()
    }
}
impl ::core::convert::From<IPipeLong> for ::windows::core::IUnknown {
    fn from(value: IPipeLong) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IPipeLong> for ::windows::core::IUnknown {
    fn from(value: &IPipeLong) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IPipeLong {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IPipeLong {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IPipeLong {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IPipeLong {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPipeLong {}
impl ::core::fmt::Debug for IPipeLong {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPipeLong").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IPipeLong {
    type Vtable = IPipeLongVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdb2f3acc_2f86_11d1_8e04_00c04fb9989a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPipeLongVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, buf: *mut i32, crequest: u32, pcreturned: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, buf: *const i32, csent: u32) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: 'Win32_System_Com'*"]
#[repr(transparent)]
pub struct IProcessInitControl(::windows::core::IUnknown);
impl IProcessInitControl {
    #[doc = "*Required features: 'Win32_System_Com'*"]
    pub unsafe fn ResetInitializerTimeout(&self, dwsecondsremaining: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwsecondsremaining)).ok()
    }
}
impl ::core::convert::From<IProcessInitControl> for ::windows::core::IUnknown {
    fn from(value: IProcessInitControl) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IProcessInitControl> for ::windows::core::IUnknown {
    fn from(value: &IProcessInitControl) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IProcessInitControl {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IProcessInitControl {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IProcessInitControl {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IProcessInitControl {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IProcessInitControl {}
impl ::core::fmt::Debug for IProcessInitControl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IProcessInitControl").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IProcessInitControl {
    type Vtable = IProcessInitControlVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x72380d55_8d2b_43a3_8513_2b6ef31434e9);
}
#[repr(C)]
#[doc(hidden)]
pub struct IProcessInitControlVtbl(pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT, pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32, pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32, pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwsecondsremaining: u32) -> ::windows::core::HRESULT);
#[doc = "*Required features: 'Win32_System_Com'*"]
#[repr(transparent)]
pub struct IProcessLock(::windows::core::IUnknown);
impl IProcessLock {
    #[doc = "*Required features: 'Win32_System_Com'*"]
    pub unsafe fn AddRefOnProcess(&self) -> u32 {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self)))
    }
    #[doc = "*Required features: 'Win32_System_Com'*"]
    pub unsafe fn ReleaseRefOnProcess(&self) -> u32 {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self)))
    }
}
impl ::core::convert::From<IProcessLock> for ::windows::core::IUnknown {
    fn from(value: IProcessLock) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IProcessLock> for ::windows::core::IUnknown {
    fn from(value: &IProcessLock) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IProcessLock {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IProcessLock {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IProcessLock {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IProcessLock {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IProcessLock {}
impl ::core::fmt::Debug for IProcessLock {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IProcessLock").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IProcessLock {
    type Vtable = IProcessLockVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x000001d5_0000_0000_c000_000000000046);
}
#[repr(C)]
#[doc(hidden)]
pub struct IProcessLockVtbl(pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT, pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32, pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32, pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32, pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32);
#[doc = "*Required features: 'Win32_System_Com'*"]
#[repr(transparent)]
pub struct IProgressNotify(::windows::core::IUnknown);
impl IProgressNotify {
    #[doc = "*Required features: 'Win32_System_Com', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn OnProgress<'a, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>, Param3: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, dwprogresscurrent: u32, dwprogressmaximum: u32, faccurate: Param2, fowner: Param3) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwprogresscurrent), ::core::mem::transmute(dwprogressmaximum), faccurate.into_param().abi(), fowner.into_param().abi()).ok()
    }
}
impl ::core::convert::From<IProgressNotify> for ::windows::core::IUnknown {
    fn from(value: IProgressNotify) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IProgressNotify> for ::windows::core::IUnknown {
    fn from(value: &IProgressNotify) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IProgressNotify {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IProgressNotify {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IProgressNotify {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IProgressNotify {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IProgressNotify {}
impl ::core::fmt::Debug for IProgressNotify {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IProgressNotify").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IProgressNotify {
    type Vtable = IProgressNotifyVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa9d758a0_4617_11cf_95fc_00aa00680db4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IProgressNotifyVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwprogresscurrent: u32, dwprogressmaximum: u32, faccurate: super::super::Foundation::BOOL, fowner: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: 'Win32_System_Com'*"]
#[repr(transparent)]
pub struct IROTData(::windows::core::IUnknown);
impl IROTData {
    #[doc = "*Required features: 'Win32_System_Com'*"]
    pub unsafe fn GetComparisonData(&self, pbdata: *mut u8, cbmax: u32, pcbdata: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(pbdata), ::core::mem::transmute(cbmax), ::core::mem::transmute(pcbdata)).ok()
    }
}
impl ::core::convert::From<IROTData> for ::windows::core::IUnknown {
    fn from(value: IROTData) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IROTData> for ::windows::core::IUnknown {
    fn from(value: &IROTData) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IROTData {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IROTData {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IROTData {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IROTData {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IROTData {}
impl ::core::fmt::Debug for IROTData {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IROTData").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IROTData {
    type Vtable = IROTDataVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf29f6bc0_5021_11ce_aa15_00006901293f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IROTDataVtbl(pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT, pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32, pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32, pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbdata: *mut u8, cbmax: u32, pcbdata: *mut u32) -> ::windows::core::HRESULT);
#[doc = "*Required features: 'Win32_System_Com'*"]
#[repr(transparent)]
pub struct IReleaseMarshalBuffers(::windows::core::IUnknown);
impl IReleaseMarshalBuffers {
    #[doc = "*Required features: 'Win32_System_Com'*"]
    pub unsafe fn ReleaseMarshalBuffer<'a, Param2: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>>(&self, pmsg: *mut RPCOLEMESSAGE, dwflags: u32, pchnl: Param2) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(pmsg), ::core::mem::transmute(dwflags), pchnl.into_param().abi()).ok()
    }
}
impl ::core::convert::From<IReleaseMarshalBuffers> for ::windows::core::IUnknown {
    fn from(value: IReleaseMarshalBuffers) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IReleaseMarshalBuffers> for ::windows::core::IUnknown {
    fn from(value: &IReleaseMarshalBuffers) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IReleaseMarshalBuffers {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IReleaseMarshalBuffers {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IReleaseMarshalBuffers {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IReleaseMarshalBuffers {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IReleaseMarshalBuffers {}
impl ::core::fmt::Debug for IReleaseMarshalBuffers {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IReleaseMarshalBuffers").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IReleaseMarshalBuffers {
    type Vtable = IReleaseMarshalBuffersVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xeb0cb9e8_7996_11d2_872e_0000f8080859);
}
#[repr(C)]
#[doc(hidden)]
pub struct IReleaseMarshalBuffersVtbl(pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT, pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32, pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32, pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pmsg: *mut RPCOLEMESSAGE, dwflags: u32, pchnl: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT);
#[doc = "*Required features: 'Win32_System_Com'*"]
#[repr(transparent)]
pub struct IRpcChannelBuffer(::windows::core::IUnknown);
impl IRpcChannelBuffer {
    #[doc = "*Required features: 'Win32_System_Com'*"]
    pub unsafe fn GetBuffer(&self, pmessage: *mut RPCOLEMESSAGE, riid: *const ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(pmessage), ::core::mem::transmute(riid)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Com'*"]
    pub unsafe fn SendReceive(&self, pmessage: *mut RPCOLEMESSAGE, pstatus: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(pmessage), ::core::mem::transmute(pstatus)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Com'*"]
    pub unsafe fn FreeBuffer(&self, pmessage: *mut RPCOLEMESSAGE) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(pmessage)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Com'*"]
    pub unsafe fn GetDestCtx(&self, pdwdestcontext: *mut u32, ppvdestcontext: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(pdwdestcontext), ::core::mem::transmute(ppvdestcontext)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Com'*"]
    pub unsafe fn IsConnected(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self)).ok()
    }
}
impl ::core::convert::From<IRpcChannelBuffer> for ::windows::core::IUnknown {
    fn from(value: IRpcChannelBuffer) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IRpcChannelBuffer> for ::windows::core::IUnknown {
    fn from(value: &IRpcChannelBuffer) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IRpcChannelBuffer {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IRpcChannelBuffer {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IRpcChannelBuffer {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IRpcChannelBuffer {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IRpcChannelBuffer {}
impl ::core::fmt::Debug for IRpcChannelBuffer {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IRpcChannelBuffer").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IRpcChannelBuffer {
    type Vtable = IRpcChannelBufferVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd5f56b60_593b_101a_b569_08002b2dbf7a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRpcChannelBufferVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pmessage: *mut RPCOLEMESSAGE, riid: *const ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pmessage: *mut RPCOLEMESSAGE, pstatus: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pmessage: *mut RPCOLEMESSAGE) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwdestcontext: *mut u32, ppvdestcontext: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: 'Win32_System_Com'*"]
#[repr(transparent)]
pub struct IRpcChannelBuffer2(::windows::core::IUnknown);
impl IRpcChannelBuffer2 {
    #[doc = "*Required features: 'Win32_System_Com'*"]
    pub unsafe fn GetBuffer(&self, pmessage: *mut RPCOLEMESSAGE, riid: *const ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(pmessage), ::core::mem::transmute(riid)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Com'*"]
    pub unsafe fn SendReceive(&self, pmessage: *mut RPCOLEMESSAGE, pstatus: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(pmessage), ::core::mem::transmute(pstatus)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Com'*"]
    pub unsafe fn FreeBuffer(&self, pmessage: *mut RPCOLEMESSAGE) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(pmessage)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Com'*"]
    pub unsafe fn GetDestCtx(&self, pdwdestcontext: *mut u32, ppvdestcontext: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(pdwdestcontext), ::core::mem::transmute(ppvdestcontext)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Com'*"]
    pub unsafe fn IsConnected(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Com'*"]
    pub unsafe fn GetProtocolVersion(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
}
impl ::core::convert::From<IRpcChannelBuffer2> for IRpcChannelBuffer {
    fn from(value: IRpcChannelBuffer2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IRpcChannelBuffer2> for IRpcChannelBuffer {
    fn from(value: &IRpcChannelBuffer2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IRpcChannelBuffer> for IRpcChannelBuffer2 {
    fn into_param(self) -> ::windows::core::Param<'a, IRpcChannelBuffer> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IRpcChannelBuffer> for &IRpcChannelBuffer2 {
    fn into_param(self) -> ::windows::core::Param<'a, IRpcChannelBuffer> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IRpcChannelBuffer2> for ::windows::core::IUnknown {
    fn from(value: IRpcChannelBuffer2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IRpcChannelBuffer2> for ::windows::core::IUnknown {
    fn from(value: &IRpcChannelBuffer2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IRpcChannelBuffer2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IRpcChannelBuffer2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IRpcChannelBuffer2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IRpcChannelBuffer2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IRpcChannelBuffer2 {}
impl ::core::fmt::Debug for IRpcChannelBuffer2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IRpcChannelBuffer2").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IRpcChannelBuffer2 {
    type Vtable = IRpcChannelBuffer2Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x594f31d0_7f19_11d0_b194_00a0c90dc8bf);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRpcChannelBuffer2Vtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pmessage: *mut RPCOLEMESSAGE, riid: *const ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pmessage: *mut RPCOLEMESSAGE, pstatus: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pmessage: *mut RPCOLEMESSAGE) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwdestcontext: *mut u32, ppvdestcontext: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwversion: *mut u32) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: 'Win32_System_Com'*"]
#[repr(transparent)]
pub struct IRpcChannelBuffer3(::windows::core::IUnknown);
impl IRpcChannelBuffer3 {
    #[doc = "*Required features: 'Win32_System_Com'*"]
    pub unsafe fn GetBuffer(&self, pmessage: *mut RPCOLEMESSAGE, riid: *const ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(pmessage), ::core::mem::transmute(riid)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Com'*"]
    pub unsafe fn SendReceive(&self, pmessage: *mut RPCOLEMESSAGE, pstatus: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(pmessage), ::core::mem::transmute(pstatus)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Com'*"]
    pub unsafe fn FreeBuffer(&self, pmessage: *mut RPCOLEMESSAGE) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(pmessage)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Com'*"]
    pub unsafe fn GetDestCtx(&self, pdwdestcontext: *mut u32, ppvdestcontext: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(pdwdestcontext), ::core::mem::transmute(ppvdestcontext)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Com'*"]
    pub unsafe fn IsConnected(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Com'*"]
    pub unsafe fn GetProtocolVersion(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Com'*"]
    pub unsafe fn Send(&self, pmsg: *mut RPCOLEMESSAGE, pulstatus: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(pmsg), ::core::mem::transmute(pulstatus)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Com'*"]
    pub unsafe fn Receive(&self, pmsg: *mut RPCOLEMESSAGE, ulsize: u32, pulstatus: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), ::core::mem::transmute(pmsg), ::core::mem::transmute(ulsize), ::core::mem::transmute(pulstatus)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Com'*"]
    pub unsafe fn Cancel(&self, pmsg: *mut RPCOLEMESSAGE) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), ::core::mem::transmute(pmsg)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Com'*"]
    pub unsafe fn GetCallContext(&self, pmsg: *const RPCOLEMESSAGE, riid: *const ::windows::core::GUID, pinterface: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), ::core::mem::transmute(pmsg), ::core::mem::transmute(riid), ::core::mem::transmute(pinterface)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Com'*"]
    pub unsafe fn GetDestCtxEx(&self, pmsg: *const RPCOLEMESSAGE, pdwdestcontext: *mut u32, ppvdestcontext: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self), ::core::mem::transmute(pmsg), ::core::mem::transmute(pdwdestcontext), ::core::mem::transmute(ppvdestcontext)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Com'*"]
    pub unsafe fn GetState(&self, pmsg: *const RPCOLEMESSAGE) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).14)(::core::mem::transmute_copy(self), ::core::mem::transmute(pmsg), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Com'*"]
    pub unsafe fn RegisterAsync<'a, Param1: ::windows::core::IntoParam<'a, IAsyncManager>>(&self, pmsg: *mut RPCOLEMESSAGE, pasyncmgr: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).15)(::core::mem::transmute_copy(self), ::core::mem::transmute(pmsg), pasyncmgr.into_param().abi()).ok()
    }
}
impl ::core::convert::From<IRpcChannelBuffer3> for IRpcChannelBuffer2 {
    fn from(value: IRpcChannelBuffer3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IRpcChannelBuffer3> for IRpcChannelBuffer2 {
    fn from(value: &IRpcChannelBuffer3) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IRpcChannelBuffer2> for IRpcChannelBuffer3 {
    fn into_param(self) -> ::windows::core::Param<'a, IRpcChannelBuffer2> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IRpcChannelBuffer2> for &IRpcChannelBuffer3 {
    fn into_param(self) -> ::windows::core::Param<'a, IRpcChannelBuffer2> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IRpcChannelBuffer3> for IRpcChannelBuffer {
    fn from(value: IRpcChannelBuffer3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IRpcChannelBuffer3> for IRpcChannelBuffer {
    fn from(value: &IRpcChannelBuffer3) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IRpcChannelBuffer> for IRpcChannelBuffer3 {
    fn into_param(self) -> ::windows::core::Param<'a, IRpcChannelBuffer> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IRpcChannelBuffer> for &IRpcChannelBuffer3 {
    fn into_param(self) -> ::windows::core::Param<'a, IRpcChannelBuffer> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IRpcChannelBuffer3> for ::windows::core::IUnknown {
    fn from(value: IRpcChannelBuffer3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IRpcChannelBuffer3> for ::windows::core::IUnknown {
    fn from(value: &IRpcChannelBuffer3) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IRpcChannelBuffer3 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IRpcChannelBuffer3 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IRpcChannelBuffer3 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IRpcChannelBuffer3 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IRpcChannelBuffer3 {}
impl ::core::fmt::Debug for IRpcChannelBuffer3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IRpcChannelBuffer3").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IRpcChannelBuffer3 {
    type Vtable = IRpcChannelBuffer3Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x25b15600_0115_11d0_bf0d_00aa00b8dfd2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRpcChannelBuffer3Vtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pmessage: *mut RPCOLEMESSAGE, riid: *const ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pmessage: *mut RPCOLEMESSAGE, pstatus: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pmessage: *mut RPCOLEMESSAGE) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwdestcontext: *mut u32, ppvdestcontext: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwversion: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pmsg: *mut RPCOLEMESSAGE, pulstatus: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pmsg: *mut RPCOLEMESSAGE, ulsize: u32, pulstatus: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pmsg: *mut RPCOLEMESSAGE) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pmsg: *const RPCOLEMESSAGE, riid: *const ::windows::core::GUID, pinterface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pmsg: *const RPCOLEMESSAGE, pdwdestcontext: *mut u32, ppvdestcontext: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pmsg: *const RPCOLEMESSAGE, pstate: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pmsg: *mut RPCOLEMESSAGE, pasyncmgr: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: 'Win32_System_Com'*"]
#[repr(transparent)]
pub struct IRpcHelper(::windows::core::IUnknown);
impl IRpcHelper {
    #[doc = "*Required features: 'Win32_System_Com'*"]
    pub unsafe fn GetDCOMProtocolVersion(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Com'*"]
    pub unsafe fn GetIIDFromOBJREF(&self, pobjref: *const ::core::ffi::c_void) -> ::windows::core::Result<*mut ::windows::core::GUID> {
        let mut result__: *mut ::windows::core::GUID = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(pobjref), ::core::mem::transmute(&mut result__)).from_abi::<*mut ::windows::core::GUID>(result__)
    }
}
impl ::core::convert::From<IRpcHelper> for ::windows::core::IUnknown {
    fn from(value: IRpcHelper) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IRpcHelper> for ::windows::core::IUnknown {
    fn from(value: &IRpcHelper) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IRpcHelper {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IRpcHelper {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IRpcHelper {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IRpcHelper {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IRpcHelper {}
impl ::core::fmt::Debug for IRpcHelper {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IRpcHelper").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IRpcHelper {
    type Vtable = IRpcHelperVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x00000149_0000_0000_c000_000000000046);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRpcHelperVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcomversion: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pobjref: *const ::core::ffi::c_void, piid: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: 'Win32_System_Com'*"]
#[repr(transparent)]
pub struct IRpcOptions(::windows::core::IUnknown);
impl IRpcOptions {
    #[doc = "*Required features: 'Win32_System_Com'*"]
    pub unsafe fn Set<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>>(&self, pprx: Param0, dwproperty: RPCOPT_PROPERTIES, dwvalue: usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), pprx.into_param().abi(), ::core::mem::transmute(dwproperty), ::core::mem::transmute(dwvalue)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Com'*"]
    pub unsafe fn Query<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>>(&self, pprx: Param0, dwproperty: RPCOPT_PROPERTIES) -> ::windows::core::Result<usize> {
        let mut result__: usize = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), pprx.into_param().abi(), ::core::mem::transmute(dwproperty), ::core::mem::transmute(&mut result__)).from_abi::<usize>(result__)
    }
}
impl ::core::convert::From<IRpcOptions> for ::windows::core::IUnknown {
    fn from(value: IRpcOptions) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IRpcOptions> for ::windows::core::IUnknown {
    fn from(value: &IRpcOptions) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IRpcOptions {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IRpcOptions {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IRpcOptions {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IRpcOptions {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IRpcOptions {}
impl ::core::fmt::Debug for IRpcOptions {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IRpcOptions").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IRpcOptions {
    type Vtable = IRpcOptionsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x00000144_0000_0000_c000_000000000046);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRpcOptionsVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pprx: *mut ::core::ffi::c_void, dwproperty: RPCOPT_PROPERTIES, dwvalue: usize) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pprx: *mut ::core::ffi::c_void, dwproperty: RPCOPT_PROPERTIES, pdwvalue: *mut usize) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: 'Win32_System_Com'*"]
#[repr(transparent)]
pub struct IRpcProxyBuffer(::windows::core::IUnknown);
impl IRpcProxyBuffer {
    #[doc = "*Required features: 'Win32_System_Com'*"]
    pub unsafe fn Connect<'a, Param0: ::windows::core::IntoParam<'a, IRpcChannelBuffer>>(&self, prpcchannelbuffer: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), prpcchannelbuffer.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_System_Com'*"]
    pub unsafe fn Disconnect(&self) {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self))
    }
}
impl ::core::convert::From<IRpcProxyBuffer> for ::windows::core::IUnknown {
    fn from(value: IRpcProxyBuffer) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IRpcProxyBuffer> for ::windows::core::IUnknown {
    fn from(value: &IRpcProxyBuffer) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IRpcProxyBuffer {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IRpcProxyBuffer {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IRpcProxyBuffer {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IRpcProxyBuffer {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IRpcProxyBuffer {}
impl ::core::fmt::Debug for IRpcProxyBuffer {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IRpcProxyBuffer").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IRpcProxyBuffer {
    type Vtable = IRpcProxyBufferVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd5f56a34_593b_101a_b569_08002b2dbf7a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRpcProxyBufferVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, prpcchannelbuffer: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void),
);
#[doc = "*Required features: 'Win32_System_Com'*"]
#[repr(transparent)]
pub struct IRpcStubBuffer(::windows::core::IUnknown);
impl IRpcStubBuffer {
    #[doc = "*Required features: 'Win32_System_Com'*"]
    pub unsafe fn Connect<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>>(&self, punkserver: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), punkserver.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_System_Com'*"]
    pub unsafe fn Disconnect(&self) {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self))
    }
    #[doc = "*Required features: 'Win32_System_Com'*"]
    pub unsafe fn Invoke<'a, Param1: ::windows::core::IntoParam<'a, IRpcChannelBuffer>>(&self, _prpcmsg: *mut RPCOLEMESSAGE, _prpcchannelbuffer: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(_prpcmsg), _prpcchannelbuffer.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_System_Com'*"]
    pub unsafe fn IsIIDSupported(&self, riid: *const ::windows::core::GUID) -> ::core::option::Option<IRpcStubBuffer> {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(riid)))
    }
    #[doc = "*Required features: 'Win32_System_Com'*"]
    pub unsafe fn CountRefs(&self) -> u32 {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self)))
    }
    #[doc = "*Required features: 'Win32_System_Com'*"]
    pub unsafe fn DebugServerQueryInterface(&self, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(ppv)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Com'*"]
    pub unsafe fn DebugServerRelease(&self, pv: *const ::core::ffi::c_void) {
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(pv))
    }
}
impl ::core::convert::From<IRpcStubBuffer> for ::windows::core::IUnknown {
    fn from(value: IRpcStubBuffer) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IRpcStubBuffer> for ::windows::core::IUnknown {
    fn from(value: &IRpcStubBuffer) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IRpcStubBuffer {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IRpcStubBuffer {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IRpcStubBuffer {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IRpcStubBuffer {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IRpcStubBuffer {}
impl ::core::fmt::Debug for IRpcStubBuffer {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IRpcStubBuffer").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IRpcStubBuffer {
    type Vtable = IRpcStubBufferVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd5f56afc_593b_101a_b569_08002b2dbf7a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRpcStubBufferVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, punkserver: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void),
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, _prpcmsg: *mut RPCOLEMESSAGE, _prpcchannelbuffer: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID) -> ::core::option::Option<IRpcStubBuffer>,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pv: *const ::core::ffi::c_void),
);
#[doc = "*Required features: 'Win32_System_Com'*"]
#[repr(transparent)]
pub struct IRpcSyntaxNegotiate(::windows::core::IUnknown);
impl IRpcSyntaxNegotiate {
    #[doc = "*Required features: 'Win32_System_Com'*"]
    pub unsafe fn NegotiateSyntax(&self, pmsg: *mut RPCOLEMESSAGE) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(pmsg)).ok()
    }
}
impl ::core::convert::From<IRpcSyntaxNegotiate> for ::windows::core::IUnknown {
    fn from(value: IRpcSyntaxNegotiate) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IRpcSyntaxNegotiate> for ::windows::core::IUnknown {
    fn from(value: &IRpcSyntaxNegotiate) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IRpcSyntaxNegotiate {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IRpcSyntaxNegotiate {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IRpcSyntaxNegotiate {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IRpcSyntaxNegotiate {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IRpcSyntaxNegotiate {}
impl ::core::fmt::Debug for IRpcSyntaxNegotiate {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IRpcSyntaxNegotiate").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IRpcSyntaxNegotiate {
    type Vtable = IRpcSyntaxNegotiateVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x58a08519_24c8_4935_b482_3fd823333a4f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRpcSyntaxNegotiateVtbl(pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT, pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32, pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32, pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pmsg: *mut RPCOLEMESSAGE) -> ::windows::core::HRESULT);
#[doc = "*Required features: 'Win32_System_Com'*"]
#[repr(transparent)]
pub struct IRunnableObject(::windows::core::IUnknown);
impl IRunnableObject {
    #[doc = "*Required features: 'Win32_System_Com'*"]
    pub unsafe fn GetRunningClass(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::GUID>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Com'*"]
    pub unsafe fn Run<'a, Param0: ::windows::core::IntoParam<'a, IBindCtx>>(&self, pbc: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), pbc.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_System_Com', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsRunning(&self) -> super::super::Foundation::BOOL {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self)))
    }
    #[doc = "*Required features: 'Win32_System_Com', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn LockRunning<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, flock: Param0, flastunlockcloses: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), flock.into_param().abi(), flastunlockcloses.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_System_Com', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetContainedObject<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, fcontained: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), fcontained.into_param().abi()).ok()
    }
}
impl ::core::convert::From<IRunnableObject> for ::windows::core::IUnknown {
    fn from(value: IRunnableObject) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IRunnableObject> for ::windows::core::IUnknown {
    fn from(value: &IRunnableObject) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IRunnableObject {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IRunnableObject {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IRunnableObject {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IRunnableObject {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IRunnableObject {}
impl ::core::fmt::Debug for IRunnableObject {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IRunnableObject").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IRunnableObject {
    type Vtable = IRunnableObjectVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x00000126_0000_0000_c000_000000000046);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRunnableObjectVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lpclsid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbc: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, flock: super::super::Foundation::BOOL, flastunlockcloses: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fcontained: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: 'Win32_System_Com'*"]
#[repr(transparent)]
pub struct IRunningObjectTable(::windows::core::IUnknown);
impl IRunningObjectTable {
    #[doc = "*Required features: 'Win32_System_Com'*"]
    pub unsafe fn Register<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>, Param2: ::windows::core::IntoParam<'a, IMoniker>>(&self, grfflags: u32, punkobject: Param1, pmkobjectname: Param2) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(grfflags), punkobject.into_param().abi(), pmkobjectname.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Com'*"]
    pub unsafe fn Revoke(&self, dwregister: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwregister)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Com'*"]
    pub unsafe fn IsRunning<'a, Param0: ::windows::core::IntoParam<'a, IMoniker>>(&self, pmkobjectname: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), pmkobjectname.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_System_Com'*"]
    pub unsafe fn GetObject<'a, Param0: ::windows::core::IntoParam<'a, IMoniker>>(&self, pmkobjectname: Param0) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__: *mut ::core::ffi::c_void = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), pmkobjectname.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::IUnknown>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Com', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn NoteChangeTime(&self, dwregister: u32, pfiletime: *const super::super::Foundation::FILETIME) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwregister), ::core::mem::transmute(pfiletime)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Com', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetTimeOfLastChange<'a, Param0: ::windows::core::IntoParam<'a, IMoniker>>(&self, pmkobjectname: Param0) -> ::windows::core::Result<super::super::Foundation::FILETIME> {
        let mut result__: super::super::Foundation::FILETIME = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), pmkobjectname.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::FILETIME>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Com'*"]
    pub unsafe fn EnumRunning(&self) -> ::windows::core::Result<IEnumMoniker> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IEnumMoniker>(result__)
    }
}
impl ::core::convert::From<IRunningObjectTable> for ::windows::core::IUnknown {
    fn from(value: IRunningObjectTable) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IRunningObjectTable> for ::windows::core::IUnknown {
    fn from(value: &IRunningObjectTable) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IRunningObjectTable {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IRunningObjectTable {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IRunningObjectTable {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IRunningObjectTable {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IRunningObjectTable {}
impl ::core::fmt::Debug for IRunningObjectTable {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IRunningObjectTable").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IRunningObjectTable {
    type Vtable = IRunningObjectTableVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x00000010_0000_0000_c000_000000000046);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRunningObjectTableVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, grfflags: u32, punkobject: *mut ::core::ffi::c_void, pmkobjectname: ::windows::core::RawPtr, pdwregister: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwregister: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pmkobjectname: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pmkobjectname: ::windows::core::RawPtr, ppunkobject: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwregister: u32, pfiletime: *const super::super::Foundation::FILETIME) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pmkobjectname: ::windows::core::RawPtr, pfiletime: *mut super::super::Foundation::FILETIME) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenummoniker: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: 'Win32_System_Com'*"]
#[repr(transparent)]
pub struct ISequentialStream(::windows::core::IUnknown);
impl ISequentialStream {
    #[doc = "*Required features: 'Win32_System_Com'*"]
    pub unsafe fn Read(&self, pv: *mut ::core::ffi::c_void, cb: u32, pcbread: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(pv), ::core::mem::transmute(cb), ::core::mem::transmute(pcbread)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Com'*"]
    pub unsafe fn Write(&self, pv: *const ::core::ffi::c_void, cb: u32) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(pv), ::core::mem::transmute(cb), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
}
impl ::core::convert::From<ISequentialStream> for ::windows::core::IUnknown {
    fn from(value: ISequentialStream) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ISequentialStream> for ::windows::core::IUnknown {
    fn from(value: &ISequentialStream) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ISequentialStream {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &ISequentialStream {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ISequentialStream {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ISequentialStream {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISequentialStream {}
impl ::core::fmt::Debug for ISequentialStream {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISequentialStream").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ISequentialStream {
    type Vtable = ISequentialStreamVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0c733a30_2a1c_11ce_ade5_00aa0044773d);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISequentialStreamVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pv: *mut ::core::ffi::c_void, cb: u32, pcbread: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pv: *const ::core::ffi::c_void, cb: u32, pcbwritten: *mut u32) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: 'Win32_System_Com'*"]
#[repr(transparent)]
pub struct IServerSecurity(::windows::core::IUnknown);
impl IServerSecurity {
    #[doc = "*Required features: 'Win32_System_Com'*"]
    pub unsafe fn QueryBlanket(&self, pauthnsvc: *mut u32, pauthzsvc: *mut u32, pserverprincname: *mut *mut u16, pauthnlevel: *mut u32, pimplevel: *mut u32, pprivs: *mut *mut ::core::ffi::c_void, pcapabilities: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(pauthnsvc), ::core::mem::transmute(pauthzsvc), ::core::mem::transmute(pserverprincname), ::core::mem::transmute(pauthnlevel), ::core::mem::transmute(pimplevel), ::core::mem::transmute(pprivs), ::core::mem::transmute(pcapabilities)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Com'*"]
    pub unsafe fn ImpersonateClient(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Com'*"]
    pub unsafe fn RevertToSelf(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Com', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsImpersonating(&self) -> super::super::Foundation::BOOL {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self)))
    }
}
impl ::core::convert::From<IServerSecurity> for ::windows::core::IUnknown {
    fn from(value: IServerSecurity) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IServerSecurity> for ::windows::core::IUnknown {
    fn from(value: &IServerSecurity) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IServerSecurity {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IServerSecurity {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IServerSecurity {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IServerSecurity {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IServerSecurity {}
impl ::core::fmt::Debug for IServerSecurity {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IServerSecurity").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IServerSecurity {
    type Vtable = IServerSecurityVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0000013e_0000_0000_c000_000000000046);
}
#[repr(C)]
#[doc(hidden)]
pub struct IServerSecurityVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pauthnsvc: *mut u32, pauthzsvc: *mut u32, pserverprincname: *mut *mut u16, pauthnlevel: *mut u32, pimplevel: *mut u32, pprivs: *mut *mut ::core::ffi::c_void, pcapabilities: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: 'Win32_System_Com'*"]
#[repr(transparent)]
pub struct IServiceProvider(::windows::core::IUnknown);
impl IServiceProvider {
    #[doc = "*Required features: 'Win32_System_Com'*"]
    pub unsafe fn QueryService(&self, guidservice: *const ::windows::core::GUID, riid: *const ::windows::core::GUID, ppvobject: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(guidservice), ::core::mem::transmute(riid), ::core::mem::transmute(ppvobject)).ok()
    }
}
impl ::core::convert::From<IServiceProvider> for ::windows::core::IUnknown {
    fn from(value: IServiceProvider) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IServiceProvider> for ::windows::core::IUnknown {
    fn from(value: &IServiceProvider) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IServiceProvider {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IServiceProvider {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IServiceProvider {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IServiceProvider {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IServiceProvider {}
impl ::core::fmt::Debug for IServiceProvider {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IServiceProvider").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IServiceProvider {
    type Vtable = IServiceProviderVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6d5140c1_7436_11ce_8034_00aa006009fa);
}
#[repr(C)]
#[doc(hidden)]
pub struct IServiceProviderVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, guidservice: *const ::windows::core::GUID, riid: *const ::windows::core::GUID, ppvobject: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: 'Win32_System_Com'*"]
#[repr(transparent)]
pub struct IStdMarshalInfo(::windows::core::IUnknown);
impl IStdMarshalInfo {
    #[doc = "*Required features: 'Win32_System_Com'*"]
    pub unsafe fn GetClassForHandler(&self, dwdestcontext: u32, pvdestcontext: *mut ::core::ffi::c_void, pclsid: *mut ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwdestcontext), ::core::mem::transmute(pvdestcontext), ::core::mem::transmute(pclsid)).ok()
    }
}
impl ::core::convert::From<IStdMarshalInfo> for ::windows::core::IUnknown {
    fn from(value: IStdMarshalInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IStdMarshalInfo> for ::windows::core::IUnknown {
    fn from(value: &IStdMarshalInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IStdMarshalInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IStdMarshalInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IStdMarshalInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IStdMarshalInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IStdMarshalInfo {}
impl ::core::fmt::Debug for IStdMarshalInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IStdMarshalInfo").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IStdMarshalInfo {
    type Vtable = IStdMarshalInfoVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x00000018_0000_0000_c000_000000000046);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStdMarshalInfoVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwdestcontext: u32, pvdestcontext: *mut ::core::ffi::c_void, pclsid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: 'Win32_System_Com'*"]
#[repr(transparent)]
pub struct IStream(::windows::core::IUnknown);
impl IStream {
    #[doc = "*Required features: 'Win32_System_Com'*"]
    pub unsafe fn Read(&self, pv: *mut ::core::ffi::c_void, cb: u32, pcbread: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(pv), ::core::mem::transmute(cb), ::core::mem::transmute(pcbread)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Com'*"]
    pub unsafe fn Write(&self, pv: *const ::core::ffi::c_void, cb: u32) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(pv), ::core::mem::transmute(cb), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Com'*"]
    pub unsafe fn Seek(&self, dlibmove: i64, dworigin: STREAM_SEEK) -> ::windows::core::Result<u64> {
        let mut result__: u64 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(dlibmove), ::core::mem::transmute(dworigin), ::core::mem::transmute(&mut result__)).from_abi::<u64>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Com'*"]
    pub unsafe fn SetSize(&self, libnewsize: u64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(libnewsize)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Com'*"]
    pub unsafe fn CopyTo<'a, Param0: ::windows::core::IntoParam<'a, IStream>>(&self, pstm: Param0, cb: u64, pcbread: *mut u64, pcbwritten: *mut u64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), pstm.into_param().abi(), ::core::mem::transmute(cb), ::core::mem::transmute(pcbread), ::core::mem::transmute(pcbwritten)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Com', 'Win32_System_Com_StructuredStorage'*"]
    #[cfg(feature = "Win32_System_Com_StructuredStorage")]
    pub unsafe fn Commit(&self, grfcommitflags: StructuredStorage::STGC) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(grfcommitflags)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Com'*"]
    pub unsafe fn Revert(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Com'*"]
    pub unsafe fn LockRegion(&self, liboffset: u64, cb: u64, dwlocktype: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), ::core::mem::transmute(liboffset), ::core::mem::transmute(cb), ::core::mem::transmute(dwlocktype)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Com'*"]
    pub unsafe fn UnlockRegion(&self, liboffset: u64, cb: u64, dwlocktype: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), ::core::mem::transmute(liboffset), ::core::mem::transmute(cb), ::core::mem::transmute(dwlocktype)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Com', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Stat(&self, pstatstg: *mut STATSTG, grfstatflag: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), ::core::mem::transmute(pstatstg), ::core::mem::transmute(grfstatflag)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Com'*"]
    pub unsafe fn Clone(&self) -> ::windows::core::Result<IStream> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IStream>(result__)
    }
}
impl ::core::convert::From<IStream> for ISequentialStream {
    fn from(value: IStream) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IStream> for ISequentialStream {
    fn from(value: &IStream) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ISequentialStream> for IStream {
    fn into_param(self) -> ::windows::core::Param<'a, ISequentialStream> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ISequentialStream> for &IStream {
    fn into_param(self) -> ::windows::core::Param<'a, ISequentialStream> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IStream> for ::windows::core::IUnknown {
    fn from(value: IStream) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IStream> for ::windows::core::IUnknown {
    fn from(value: &IStream) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IStream {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IStream {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IStream {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IStream {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IStream {}
impl ::core::fmt::Debug for IStream {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IStream").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IStream {
    type Vtable = IStreamVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0000000c_0000_0000_c000_000000000046);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStreamVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pv: *mut ::core::ffi::c_void, cb: u32, pcbread: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pv: *const ::core::ffi::c_void, cb: u32, pcbwritten: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dlibmove: i64, dworigin: STREAM_SEEK, plibnewposition: *mut u64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, libnewsize: u64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pstm: ::windows::core::RawPtr, cb: u64, pcbread: *mut u64, pcbwritten: *mut u64) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com_StructuredStorage")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, grfcommitflags: StructuredStorage::STGC) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com_StructuredStorage"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, liboffset: u64, cb: u64, dwlocktype: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, liboffset: u64, cb: u64, dwlocktype: u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pstatstg: *mut STATSTG, grfstatflag: u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppstm: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: 'Win32_System_Com'*"]
#[repr(transparent)]
pub struct ISupportErrorInfo(::windows::core::IUnknown);
impl ISupportErrorInfo {
    #[doc = "*Required features: 'Win32_System_Com'*"]
    pub unsafe fn InterfaceSupportsErrorInfo(&self, riid: *const ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(riid)).ok()
    }
}
impl ::core::convert::From<ISupportErrorInfo> for ::windows::core::IUnknown {
    fn from(value: ISupportErrorInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ISupportErrorInfo> for ::windows::core::IUnknown {
    fn from(value: &ISupportErrorInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ISupportErrorInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &ISupportErrorInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ISupportErrorInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ISupportErrorInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISupportErrorInfo {}
impl ::core::fmt::Debug for ISupportErrorInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISupportErrorInfo").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ISupportErrorInfo {
    type Vtable = ISupportErrorInfoVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdf0b3d60_548f_101b_8e65_08002b2bd119);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISupportErrorInfoVtbl(pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT, pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32, pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32, pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID) -> ::windows::core::HRESULT);
#[doc = "*Required features: 'Win32_System_Com'*"]
#[repr(transparent)]
pub struct ISurrogate(::windows::core::IUnknown);
impl ISurrogate {
    #[doc = "*Required features: 'Win32_System_Com'*"]
    pub unsafe fn LoadDllServer(&self, clsid: *const ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(clsid)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Com'*"]
    pub unsafe fn FreeSurrogate(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self)).ok()
    }
}
impl ::core::convert::From<ISurrogate> for ::windows::core::IUnknown {
    fn from(value: ISurrogate) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ISurrogate> for ::windows::core::IUnknown {
    fn from(value: &ISurrogate) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ISurrogate {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &ISurrogate {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ISurrogate {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ISurrogate {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISurrogate {}
impl ::core::fmt::Debug for ISurrogate {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISurrogate").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ISurrogate {
    type Vtable = ISurrogateVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x00000022_0000_0000_c000_000000000046);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISurrogateVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, clsid: *const ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: 'Win32_System_Com'*"]
#[repr(transparent)]
pub struct ISurrogateService(::windows::core::IUnknown);
impl ISurrogateService {
    #[doc = "*Required features: 'Win32_System_Com', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Init<'a, Param1: ::windows::core::IntoParam<'a, IProcessLock>>(&self, rguidprocessid: *const ::windows::core::GUID, pprocesslock: Param1) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__: super::super::Foundation::BOOL = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(rguidprocessid), pprocesslock.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Com'*"]
    pub unsafe fn ApplicationLaunch(&self, rguidapplid: *const ::windows::core::GUID, apptype: ApplicationType) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(rguidapplid), ::core::mem::transmute(apptype)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Com'*"]
    pub unsafe fn ApplicationFree(&self, rguidapplid: *const ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(rguidapplid)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Com'*"]
    pub unsafe fn CatalogRefresh(&self, ulreserved: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(ulreserved)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Com'*"]
    pub unsafe fn ProcessShutdown(&self, shutdowntype: ShutdownType) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(shutdowntype)).ok()
    }
}
impl ::core::convert::From<ISurrogateService> for ::windows::core::IUnknown {
    fn from(value: ISurrogateService) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ISurrogateService> for ::windows::core::IUnknown {
    fn from(value: &ISurrogateService) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ISurrogateService {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &ISurrogateService {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ISurrogateService {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ISurrogateService {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISurrogateService {}
impl ::core::fmt::Debug for ISurrogateService {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISurrogateService").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ISurrogateService {
    type Vtable = ISurrogateServiceVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x000001d4_0000_0000_c000_000000000046);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISurrogateServiceVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, rguidprocessid: *const ::windows::core::GUID, pprocesslock: ::windows::core::RawPtr, pfapplicationaware: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, rguidapplid: *const ::windows::core::GUID, apptype: ApplicationType) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, rguidapplid: *const ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ulreserved: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, shutdowntype: ShutdownType) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: 'Win32_System_Com'*"]
#[repr(transparent)]
pub struct ISynchronize(::windows::core::IUnknown);
impl ISynchronize {
    #[doc = "*Required features: 'Win32_System_Com'*"]
    pub unsafe fn Wait(&self, dwflags: u32, dwmilliseconds: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwflags), ::core::mem::transmute(dwmilliseconds)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Com'*"]
    pub unsafe fn Signal(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Com'*"]
    pub unsafe fn Reset(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self)).ok()
    }
}
impl ::core::convert::From<ISynchronize> for ::windows::core::IUnknown {
    fn from(value: ISynchronize) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ISynchronize> for ::windows::core::IUnknown {
    fn from(value: &ISynchronize) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ISynchronize {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &ISynchronize {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ISynchronize {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ISynchronize {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISynchronize {}
impl ::core::fmt::Debug for ISynchronize {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISynchronize").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ISynchronize {
    type Vtable = ISynchronizeVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x00000030_0000_0000_c000_000000000046);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISynchronizeVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwflags: u32, dwmilliseconds: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: 'Win32_System_Com'*"]
#[repr(transparent)]
pub struct ISynchronizeContainer(::windows::core::IUnknown);
impl ISynchronizeContainer {
    #[doc = "*Required features: 'Win32_System_Com'*"]
    pub unsafe fn AddSynchronize<'a, Param0: ::windows::core::IntoParam<'a, ISynchronize>>(&self, psync: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), psync.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_System_Com'*"]
    pub unsafe fn WaitMultiple(&self, dwflags: u32, dwtimeout: u32) -> ::windows::core::Result<ISynchronize> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwflags), ::core::mem::transmute(dwtimeout), ::core::mem::transmute(&mut result__)).from_abi::<ISynchronize>(result__)
    }
}
impl ::core::convert::From<ISynchronizeContainer> for ::windows::core::IUnknown {
    fn from(value: ISynchronizeContainer) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ISynchronizeContainer> for ::windows::core::IUnknown {
    fn from(value: &ISynchronizeContainer) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ISynchronizeContainer {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &ISynchronizeContainer {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ISynchronizeContainer {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ISynchronizeContainer {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISynchronizeContainer {}
impl ::core::fmt::Debug for ISynchronizeContainer {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISynchronizeContainer").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ISynchronizeContainer {
    type Vtable = ISynchronizeContainerVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x00000033_0000_0000_c000_000000000046);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISynchronizeContainerVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, psync: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwflags: u32, dwtimeout: u32, ppsync: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: 'Win32_System_Com'*"]
#[repr(transparent)]
pub struct ISynchronizeEvent(::windows::core::IUnknown);
impl ISynchronizeEvent {
    #[doc = "*Required features: 'Win32_System_Com', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetHandle(&self) -> ::windows::core::Result<super::super::Foundation::HANDLE> {
        let mut result__: super::super::Foundation::HANDLE = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::HANDLE>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Com', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetEventHandle(&self, ph: *const super::super::Foundation::HANDLE) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(ph)).ok()
    }
}
impl ::core::convert::From<ISynchronizeEvent> for ISynchronizeHandle {
    fn from(value: ISynchronizeEvent) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ISynchronizeEvent> for ISynchronizeHandle {
    fn from(value: &ISynchronizeEvent) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ISynchronizeHandle> for ISynchronizeEvent {
    fn into_param(self) -> ::windows::core::Param<'a, ISynchronizeHandle> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ISynchronizeHandle> for &ISynchronizeEvent {
    fn into_param(self) -> ::windows::core::Param<'a, ISynchronizeHandle> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ISynchronizeEvent> for ::windows::core::IUnknown {
    fn from(value: ISynchronizeEvent) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ISynchronizeEvent> for ::windows::core::IUnknown {
    fn from(value: &ISynchronizeEvent) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ISynchronizeEvent {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &ISynchronizeEvent {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ISynchronizeEvent {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ISynchronizeEvent {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISynchronizeEvent {}
impl ::core::fmt::Debug for ISynchronizeEvent {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISynchronizeEvent").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ISynchronizeEvent {
    type Vtable = ISynchronizeEventVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x00000032_0000_0000_c000_000000000046);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISynchronizeEventVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ph: *mut super::super::Foundation::HANDLE) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ph: *const super::super::Foundation::HANDLE) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: 'Win32_System_Com'*"]
#[repr(transparent)]
pub struct ISynchronizeHandle(::windows::core::IUnknown);
impl ISynchronizeHandle {
    #[doc = "*Required features: 'Win32_System_Com', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetHandle(&self) -> ::windows::core::Result<super::super::Foundation::HANDLE> {
        let mut result__: super::super::Foundation::HANDLE = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::HANDLE>(result__)
    }
}
impl ::core::convert::From<ISynchronizeHandle> for ::windows::core::IUnknown {
    fn from(value: ISynchronizeHandle) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ISynchronizeHandle> for ::windows::core::IUnknown {
    fn from(value: &ISynchronizeHandle) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ISynchronizeHandle {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &ISynchronizeHandle {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ISynchronizeHandle {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ISynchronizeHandle {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISynchronizeHandle {}
impl ::core::fmt::Debug for ISynchronizeHandle {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISynchronizeHandle").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ISynchronizeHandle {
    type Vtable = ISynchronizeHandleVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x00000031_0000_0000_c000_000000000046);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISynchronizeHandleVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ph: *mut super::super::Foundation::HANDLE) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: 'Win32_System_Com'*"]
#[repr(transparent)]
pub struct ISynchronizeMutex(::windows::core::IUnknown);
impl ISynchronizeMutex {
    #[doc = "*Required features: 'Win32_System_Com'*"]
    pub unsafe fn Wait(&self, dwflags: u32, dwmilliseconds: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwflags), ::core::mem::transmute(dwmilliseconds)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Com'*"]
    pub unsafe fn Signal(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Com'*"]
    pub unsafe fn Reset(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Com'*"]
    pub unsafe fn ReleaseMutex(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self)).ok()
    }
}
impl ::core::convert::From<ISynchronizeMutex> for ISynchronize {
    fn from(value: ISynchronizeMutex) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ISynchronizeMutex> for ISynchronize {
    fn from(value: &ISynchronizeMutex) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ISynchronize> for ISynchronizeMutex {
    fn into_param(self) -> ::windows::core::Param<'a, ISynchronize> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ISynchronize> for &ISynchronizeMutex {
    fn into_param(self) -> ::windows::core::Param<'a, ISynchronize> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ISynchronizeMutex> for ::windows::core::IUnknown {
    fn from(value: ISynchronizeMutex) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ISynchronizeMutex> for ::windows::core::IUnknown {
    fn from(value: &ISynchronizeMutex) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ISynchronizeMutex {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &ISynchronizeMutex {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ISynchronizeMutex {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ISynchronizeMutex {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISynchronizeMutex {}
impl ::core::fmt::Debug for ISynchronizeMutex {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISynchronizeMutex").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ISynchronizeMutex {
    type Vtable = ISynchronizeMutexVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x00000025_0000_0000_c000_000000000046);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISynchronizeMutexVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwflags: u32, dwmilliseconds: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: 'Win32_System_Com'*"]
#[repr(transparent)]
pub struct ITimeAndNoticeControl(::windows::core::IUnknown);
impl ITimeAndNoticeControl {
    #[doc = "*Required features: 'Win32_System_Com'*"]
    pub unsafe fn SuppressChanges(&self, res1: u32, res2: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(res1), ::core::mem::transmute(res2)).ok()
    }
}
impl ::core::convert::From<ITimeAndNoticeControl> for ::windows::core::IUnknown {
    fn from(value: ITimeAndNoticeControl) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ITimeAndNoticeControl> for ::windows::core::IUnknown {
    fn from(value: &ITimeAndNoticeControl) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ITimeAndNoticeControl {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &ITimeAndNoticeControl {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ITimeAndNoticeControl {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ITimeAndNoticeControl {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITimeAndNoticeControl {}
impl ::core::fmt::Debug for ITimeAndNoticeControl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITimeAndNoticeControl").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ITimeAndNoticeControl {
    type Vtable = ITimeAndNoticeControlVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbc0bf6ae_8878_11d1_83e9_00c04fc2c6d4);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITimeAndNoticeControlVtbl(pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT, pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32, pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32, pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, res1: u32, res2: u32) -> ::windows::core::HRESULT);
#[doc = "*Required features: 'Win32_System_Com'*"]
#[repr(transparent)]
pub struct ITypeComp(::windows::core::IUnknown);
impl ITypeComp {
    #[doc = "*Required features: 'Win32_System_Com', 'Win32_Foundation', 'Win32_System_Ole'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
    pub unsafe fn Bind<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, szname: Param0, lhashval: u32, wflags: u16, pptinfo: *mut ::core::option::Option<ITypeInfo>, pdesckind: *mut DESCKIND, pbindptr: *mut BINDPTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), szname.into_param().abi(), ::core::mem::transmute(lhashval), ::core::mem::transmute(wflags), ::core::mem::transmute(pptinfo), ::core::mem::transmute(pdesckind), ::core::mem::transmute(pbindptr)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Com', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn BindType<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, szname: Param0, lhashval: u32, pptinfo: *mut ::core::option::Option<ITypeInfo>, pptcomp: *mut ::core::option::Option<ITypeComp>) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), szname.into_param().abi(), ::core::mem::transmute(lhashval), ::core::mem::transmute(pptinfo), ::core::mem::transmute(pptcomp)).ok()
    }
}
impl ::core::convert::From<ITypeComp> for ::windows::core::IUnknown {
    fn from(value: ITypeComp) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ITypeComp> for ::windows::core::IUnknown {
    fn from(value: &ITypeComp) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ITypeComp {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &ITypeComp {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ITypeComp {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ITypeComp {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITypeComp {}
impl ::core::fmt::Debug for ITypeComp {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITypeComp").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ITypeComp {
    type Vtable = ITypeCompVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x00020403_0000_0000_c000_000000000046);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITypeCompVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, szname: super::super::Foundation::PWSTR, lhashval: u32, wflags: u16, pptinfo: *mut ::windows::core::RawPtr, pdesckind: *mut DESCKIND, pbindptr: *mut BINDPTR) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Ole")))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, szname: super::super::Foundation::PWSTR, lhashval: u32, pptinfo: *mut ::windows::core::RawPtr, pptcomp: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: 'Win32_System_Com'*"]
#[repr(transparent)]
pub struct ITypeInfo(::windows::core::IUnknown);
impl ITypeInfo {
    #[doc = "*Required features: 'Win32_System_Com', 'Win32_Foundation', 'Win32_System_Ole'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
    pub unsafe fn GetTypeAttr(&self) -> ::windows::core::Result<*mut TYPEATTR> {
        let mut result__: *mut TYPEATTR = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<*mut TYPEATTR>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Com'*"]
    pub unsafe fn GetTypeComp(&self) -> ::windows::core::Result<ITypeComp> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<ITypeComp>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Com', 'Win32_Foundation', 'Win32_System_Ole'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
    pub unsafe fn GetFuncDesc(&self, index: u32) -> ::windows::core::Result<*mut FUNCDESC> {
        let mut result__: *mut FUNCDESC = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(index), ::core::mem::transmute(&mut result__)).from_abi::<*mut FUNCDESC>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Com', 'Win32_Foundation', 'Win32_System_Ole'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
    pub unsafe fn GetVarDesc(&self, index: u32) -> ::windows::core::Result<*mut VARDESC> {
        let mut result__: *mut VARDESC = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(index), ::core::mem::transmute(&mut result__)).from_abi::<*mut VARDESC>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Com', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetNames(&self, memid: i32, rgbstrnames: *mut super::super::Foundation::BSTR, cmaxnames: u32, pcnames: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(memid), ::core::mem::transmute(rgbstrnames), ::core::mem::transmute(cmaxnames), ::core::mem::transmute(pcnames)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Com'*"]
    pub unsafe fn GetRefTypeOfImplType(&self, index: u32) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(index), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Com'*"]
    pub unsafe fn GetImplTypeFlags(&self, index: u32) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(index), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Com', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetIDsOfNames(&self, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, pmemid: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), ::core::mem::transmute(rgsznames), ::core::mem::transmute(cnames), ::core::mem::transmute(pmemid)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Com', 'Win32_Foundation', 'Win32_System_Ole'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
    pub unsafe fn Invoke(&self, pvinstance: *const ::core::ffi::c_void, memid: i32, wflags: u16, pdispparams: *mut DISPPARAMS, pvarresult: *mut VARIANT, pexcepinfo: *mut EXCEPINFO, puargerr: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), ::core::mem::transmute(pvinstance), ::core::mem::transmute(memid), ::core::mem::transmute(wflags), ::core::mem::transmute(pdispparams), ::core::mem::transmute(pvarresult), ::core::mem::transmute(pexcepinfo), ::core::mem::transmute(puargerr)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Com', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetDocumentation(&self, memid: i32, pbstrname: *mut super::super::Foundation::BSTR, pbstrdocstring: *mut super::super::Foundation::BSTR, pdwhelpcontext: *mut u32, pbstrhelpfile: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), ::core::mem::transmute(memid), ::core::mem::transmute(pbstrname), ::core::mem::transmute(pbstrdocstring), ::core::mem::transmute(pdwhelpcontext), ::core::mem::transmute(pbstrhelpfile)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Com', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetDllEntry(&self, memid: i32, invkind: INVOKEKIND, pbstrdllname: *mut super::super::Foundation::BSTR, pbstrname: *mut super::super::Foundation::BSTR, pwordinal: *mut u16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self), ::core::mem::transmute(memid), ::core::mem::transmute(invkind), ::core::mem::transmute(pbstrdllname), ::core::mem::transmute(pbstrname), ::core::mem::transmute(pwordinal)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Com'*"]
    pub unsafe fn GetRefTypeInfo(&self, hreftype: u32) -> ::windows::core::Result<ITypeInfo> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).14)(::core::mem::transmute_copy(self), ::core::mem::transmute(hreftype), ::core::mem::transmute(&mut result__)).from_abi::<ITypeInfo>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Com'*"]
    pub unsafe fn AddressOfMember(&self, memid: i32, invkind: INVOKEKIND, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).15)(::core::mem::transmute_copy(self), ::core::mem::transmute(memid), ::core::mem::transmute(invkind), ::core::mem::transmute(ppv)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Com'*"]
    pub unsafe fn CreateInstance<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>, T: ::windows::core::Interface>(&self, punkouter: Param0) -> ::windows::core::Result<T> {
        let mut result__ = ::core::option::Option::None;
        (::windows::core::Interface::vtable(self).16)(::core::mem::transmute_copy(self), punkouter.into_param().abi(), &<T as ::windows::core::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
    #[doc = "*Required features: 'Win32_System_Com', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetMops(&self, memid: i32) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).17)(::core::mem::transmute_copy(self), ::core::mem::transmute(memid), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Com'*"]
    pub unsafe fn GetContainingTypeLib(&self, pptlib: *mut ::core::option::Option<ITypeLib>, pindex: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).18)(::core::mem::transmute_copy(self), ::core::mem::transmute(pptlib), ::core::mem::transmute(pindex)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Com', 'Win32_Foundation', 'Win32_System_Ole'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
    pub unsafe fn ReleaseTypeAttr(&self, ptypeattr: *const TYPEATTR) {
        (::windows::core::Interface::vtable(self).19)(::core::mem::transmute_copy(self), ::core::mem::transmute(ptypeattr))
    }
    #[doc = "*Required features: 'Win32_System_Com', 'Win32_Foundation', 'Win32_System_Ole'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
    pub unsafe fn ReleaseFuncDesc(&self, pfuncdesc: *const FUNCDESC) {
        (::windows::core::Interface::vtable(self).20)(::core::mem::transmute_copy(self), ::core::mem::transmute(pfuncdesc))
    }
    #[doc = "*Required features: 'Win32_System_Com', 'Win32_Foundation', 'Win32_System_Ole'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
    pub unsafe fn ReleaseVarDesc(&self, pvardesc: *const VARDESC) {
        (::windows::core::Interface::vtable(self).21)(::core::mem::transmute_copy(self), ::core::mem::transmute(pvardesc))
    }
}
impl ::core::convert::From<ITypeInfo> for ::windows::core::IUnknown {
    fn from(value: ITypeInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ITypeInfo> for ::windows::core::IUnknown {
    fn from(value: &ITypeInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ITypeInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &ITypeInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ITypeInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ITypeInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITypeInfo {}
impl ::core::fmt::Debug for ITypeInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITypeInfo").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ITypeInfo {
    type Vtable = ITypeInfoVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x00020401_0000_0000_c000_000000000046);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITypeInfoVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pptypeattr: *mut *mut TYPEATTR) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pptcomp: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32, ppfuncdesc: *mut *mut FUNCDESC) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Ole")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32, ppvardesc: *mut *mut VARDESC) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Ole")))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, memid: i32, rgbstrnames: *mut super::super::Foundation::BSTR, cmaxnames: u32, pcnames: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32, preftype: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32, pimpltypeflags: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, pmemid: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvinstance: *const ::core::ffi::c_void, memid: i32, wflags: u16, pdispparams: *mut DISPPARAMS, pvarresult: *mut VARIANT, pexcepinfo: *mut EXCEPINFO, puargerr: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Ole")))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, memid: i32, pbstrname: *mut super::super::Foundation::BSTR, pbstrdocstring: *mut super::super::Foundation::BSTR, pdwhelpcontext: *mut u32, pbstrhelpfile: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, memid: i32, invkind: INVOKEKIND, pbstrdllname: *mut super::super::Foundation::BSTR, pbstrname: *mut super::super::Foundation::BSTR, pwordinal: *mut u16) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hreftype: u32, pptinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, memid: i32, invkind: INVOKEKIND, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, punkouter: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, ppvobj: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, memid: i32, pbstrmops: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pptlib: *mut ::windows::core::RawPtr, pindex: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ptypeattr: *const TYPEATTR),
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Ole")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfuncdesc: *const FUNCDESC),
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Ole")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvardesc: *const VARDESC),
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Ole")))] usize,
);
#[doc = "*Required features: 'Win32_System_Com'*"]
#[repr(transparent)]
pub struct ITypeInfo2(::windows::core::IUnknown);
impl ITypeInfo2 {
    #[doc = "*Required features: 'Win32_System_Com', 'Win32_Foundation', 'Win32_System_Ole'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
    pub unsafe fn GetTypeAttr(&self) -> ::windows::core::Result<*mut TYPEATTR> {
        let mut result__: *mut TYPEATTR = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<*mut TYPEATTR>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Com'*"]
    pub unsafe fn GetTypeComp(&self) -> ::windows::core::Result<ITypeComp> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<ITypeComp>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Com', 'Win32_Foundation', 'Win32_System_Ole'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
    pub unsafe fn GetFuncDesc(&self, index: u32) -> ::windows::core::Result<*mut FUNCDESC> {
        let mut result__: *mut FUNCDESC = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(index), ::core::mem::transmute(&mut result__)).from_abi::<*mut FUNCDESC>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Com', 'Win32_Foundation', 'Win32_System_Ole'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
    pub unsafe fn GetVarDesc(&self, index: u32) -> ::windows::core::Result<*mut VARDESC> {
        let mut result__: *mut VARDESC = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(index), ::core::mem::transmute(&mut result__)).from_abi::<*mut VARDESC>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Com', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetNames(&self, memid: i32, rgbstrnames: *mut super::super::Foundation::BSTR, cmaxnames: u32, pcnames: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(memid), ::core::mem::transmute(rgbstrnames), ::core::mem::transmute(cmaxnames), ::core::mem::transmute(pcnames)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Com'*"]
    pub unsafe fn GetRefTypeOfImplType(&self, index: u32) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(index), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Com'*"]
    pub unsafe fn GetImplTypeFlags(&self, index: u32) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(index), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Com', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetIDsOfNames(&self, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, pmemid: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), ::core::mem::transmute(rgsznames), ::core::mem::transmute(cnames), ::core::mem::transmute(pmemid)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Com', 'Win32_Foundation', 'Win32_System_Ole'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
    pub unsafe fn Invoke(&self, pvinstance: *const ::core::ffi::c_void, memid: i32, wflags: u16, pdispparams: *mut DISPPARAMS, pvarresult: *mut VARIANT, pexcepinfo: *mut EXCEPINFO, puargerr: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), ::core::mem::transmute(pvinstance), ::core::mem::transmute(memid), ::core::mem::transmute(wflags), ::core::mem::transmute(pdispparams), ::core::mem::transmute(pvarresult), ::core::mem::transmute(pexcepinfo), ::core::mem::transmute(puargerr)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Com', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetDocumentation(&self, memid: i32, pbstrname: *mut super::super::Foundation::BSTR, pbstrdocstring: *mut super::super::Foundation::BSTR, pdwhelpcontext: *mut u32, pbstrhelpfile: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), ::core::mem::transmute(memid), ::core::mem::transmute(pbstrname), ::core::mem::transmute(pbstrdocstring), ::core::mem::transmute(pdwhelpcontext), ::core::mem::transmute(pbstrhelpfile)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Com', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetDllEntry(&self, memid: i32, invkind: INVOKEKIND, pbstrdllname: *mut super::super::Foundation::BSTR, pbstrname: *mut super::super::Foundation::BSTR, pwordinal: *mut u16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self), ::core::mem::transmute(memid), ::core::mem::transmute(invkind), ::core::mem::transmute(pbstrdllname), ::core::mem::transmute(pbstrname), ::core::mem::transmute(pwordinal)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Com'*"]
    pub unsafe fn GetRefTypeInfo(&self, hreftype: u32) -> ::windows::core::Result<ITypeInfo> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).14)(::core::mem::transmute_copy(self), ::core::mem::transmute(hreftype), ::core::mem::transmute(&mut result__)).from_abi::<ITypeInfo>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Com'*"]
    pub unsafe fn AddressOfMember(&self, memid: i32, invkind: INVOKEKIND, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).15)(::core::mem::transmute_copy(self), ::core::mem::transmute(memid), ::core::mem::transmute(invkind), ::core::mem::transmute(ppv)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Com'*"]
    pub unsafe fn CreateInstance<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>, T: ::windows::core::Interface>(&self, punkouter: Param0) -> ::windows::core::Result<T> {
        let mut result__ = ::core::option::Option::None;
        (::windows::core::Interface::vtable(self).16)(::core::mem::transmute_copy(self), punkouter.into_param().abi(), &<T as ::windows::core::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
    #[doc = "*Required features: 'Win32_System_Com', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetMops(&self, memid: i32) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).17)(::core::mem::transmute_copy(self), ::core::mem::transmute(memid), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Com'*"]
    pub unsafe fn GetContainingTypeLib(&self, pptlib: *mut ::core::option::Option<ITypeLib>, pindex: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).18)(::core::mem::transmute_copy(self), ::core::mem::transmute(pptlib), ::core::mem::transmute(pindex)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Com', 'Win32_Foundation', 'Win32_System_Ole'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
    pub unsafe fn ReleaseTypeAttr(&self, ptypeattr: *const TYPEATTR) {
        (::windows::core::Interface::vtable(self).19)(::core::mem::transmute_copy(self), ::core::mem::transmute(ptypeattr))
    }
    #[doc = "*Required features: 'Win32_System_Com', 'Win32_Foundation', 'Win32_System_Ole'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
    pub unsafe fn ReleaseFuncDesc(&self, pfuncdesc: *const FUNCDESC) {
        (::windows::core::Interface::vtable(self).20)(::core::mem::transmute_copy(self), ::core::mem::transmute(pfuncdesc))
    }
    #[doc = "*Required features: 'Win32_System_Com', 'Win32_Foundation', 'Win32_System_Ole'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
    pub unsafe fn ReleaseVarDesc(&self, pvardesc: *const VARDESC) {
        (::windows::core::Interface::vtable(self).21)(::core::mem::transmute_copy(self), ::core::mem::transmute(pvardesc))
    }
    #[doc = "*Required features: 'Win32_System_Com'*"]
    pub unsafe fn GetTypeKind(&self) -> ::windows::core::Result<TYPEKIND> {
        let mut result__: TYPEKIND = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).22)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<TYPEKIND>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Com'*"]
    pub unsafe fn GetTypeFlags(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).23)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Com'*"]
    pub unsafe fn GetFuncIndexOfMemId(&self, memid: i32, invkind: INVOKEKIND) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).24)(::core::mem::transmute_copy(self), ::core::mem::transmute(memid), ::core::mem::transmute(invkind), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Com'*"]
    pub unsafe fn GetVarIndexOfMemId(&self, memid: i32) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).25)(::core::mem::transmute_copy(self), ::core::mem::transmute(memid), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Com', 'Win32_Foundation', 'Win32_System_Ole'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
    pub unsafe fn GetCustData(&self, guid: *const ::windows::core::GUID) -> ::windows::core::Result<VARIANT> {
        let mut result__: ::core::mem::ManuallyDrop<VARIANT> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).26)(::core::mem::transmute_copy(self), ::core::mem::transmute(guid), ::core::mem::transmute(&mut result__)).from_abi::<VARIANT>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Com', 'Win32_Foundation', 'Win32_System_Ole'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
    pub unsafe fn GetFuncCustData(&self, index: u32, guid: *const ::windows::core::GUID) -> ::windows::core::Result<VARIANT> {
        let mut result__: ::core::mem::ManuallyDrop<VARIANT> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).27)(::core::mem::transmute_copy(self), ::core::mem::transmute(index), ::core::mem::transmute(guid), ::core::mem::transmute(&mut result__)).from_abi::<VARIANT>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Com', 'Win32_Foundation', 'Win32_System_Ole'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
    pub unsafe fn GetParamCustData(&self, indexfunc: u32, indexparam: u32, guid: *const ::windows::core::GUID) -> ::windows::core::Result<VARIANT> {
        let mut result__: ::core::mem::ManuallyDrop<VARIANT> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).28)(::core::mem::transmute_copy(self), ::core::mem::transmute(indexfunc), ::core::mem::transmute(indexparam), ::core::mem::transmute(guid), ::core::mem::transmute(&mut result__)).from_abi::<VARIANT>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Com', 'Win32_Foundation', 'Win32_System_Ole'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
    pub unsafe fn GetVarCustData(&self, index: u32, guid: *const ::windows::core::GUID) -> ::windows::core::Result<VARIANT> {
        let mut result__: ::core::mem::ManuallyDrop<VARIANT> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).29)(::core::mem::transmute_copy(self), ::core::mem::transmute(index), ::core::mem::transmute(guid), ::core::mem::transmute(&mut result__)).from_abi::<VARIANT>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Com', 'Win32_Foundation', 'Win32_System_Ole'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
    pub unsafe fn GetImplTypeCustData(&self, index: u32, guid: *const ::windows::core::GUID) -> ::windows::core::Result<VARIANT> {
        let mut result__: ::core::mem::ManuallyDrop<VARIANT> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).30)(::core::mem::transmute_copy(self), ::core::mem::transmute(index), ::core::mem::transmute(guid), ::core::mem::transmute(&mut result__)).from_abi::<VARIANT>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Com', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetDocumentation2(&self, memid: i32, lcid: u32, pbstrhelpstring: *mut super::super::Foundation::BSTR, pdwhelpstringcontext: *mut u32, pbstrhelpstringdll: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).31)(::core::mem::transmute_copy(self), ::core::mem::transmute(memid), ::core::mem::transmute(lcid), ::core::mem::transmute(pbstrhelpstring), ::core::mem::transmute(pdwhelpstringcontext), ::core::mem::transmute(pbstrhelpstringdll)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Com', 'Win32_Foundation', 'Win32_System_Ole'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
    pub unsafe fn GetAllCustData(&self) -> ::windows::core::Result<CUSTDATA> {
        let mut result__: CUSTDATA = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).32)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<CUSTDATA>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Com', 'Win32_Foundation', 'Win32_System_Ole'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
    pub unsafe fn GetAllFuncCustData(&self, index: u32) -> ::windows::core::Result<CUSTDATA> {
        let mut result__: CUSTDATA = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).33)(::core::mem::transmute_copy(self), ::core::mem::transmute(index), ::core::mem::transmute(&mut result__)).from_abi::<CUSTDATA>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Com', 'Win32_Foundation', 'Win32_System_Ole'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
    pub unsafe fn GetAllParamCustData(&self, indexfunc: u32, indexparam: u32) -> ::windows::core::Result<CUSTDATA> {
        let mut result__: CUSTDATA = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).34)(::core::mem::transmute_copy(self), ::core::mem::transmute(indexfunc), ::core::mem::transmute(indexparam), ::core::mem::transmute(&mut result__)).from_abi::<CUSTDATA>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Com', 'Win32_Foundation', 'Win32_System_Ole'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
    pub unsafe fn GetAllVarCustData(&self, index: u32) -> ::windows::core::Result<CUSTDATA> {
        let mut result__: CUSTDATA = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).35)(::core::mem::transmute_copy(self), ::core::mem::transmute(index), ::core::mem::transmute(&mut result__)).from_abi::<CUSTDATA>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Com', 'Win32_Foundation', 'Win32_System_Ole'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
    pub unsafe fn GetAllImplTypeCustData(&self, index: u32) -> ::windows::core::Result<CUSTDATA> {
        let mut result__: CUSTDATA = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).36)(::core::mem::transmute_copy(self), ::core::mem::transmute(index), ::core::mem::transmute(&mut result__)).from_abi::<CUSTDATA>(result__)
    }
}
impl ::core::convert::From<ITypeInfo2> for ITypeInfo {
    fn from(value: ITypeInfo2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ITypeInfo2> for ITypeInfo {
    fn from(value: &ITypeInfo2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ITypeInfo> for ITypeInfo2 {
    fn into_param(self) -> ::windows::core::Param<'a, ITypeInfo> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ITypeInfo> for &ITypeInfo2 {
    fn into_param(self) -> ::windows::core::Param<'a, ITypeInfo> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ITypeInfo2> for ::windows::core::IUnknown {
    fn from(value: ITypeInfo2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ITypeInfo2> for ::windows::core::IUnknown {
    fn from(value: &ITypeInfo2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ITypeInfo2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &ITypeInfo2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ITypeInfo2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ITypeInfo2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITypeInfo2 {}
impl ::core::fmt::Debug for ITypeInfo2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITypeInfo2").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ITypeInfo2 {
    type Vtable = ITypeInfo2Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x00020412_0000_0000_c000_000000000046);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITypeInfo2Vtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pptypeattr: *mut *mut TYPEATTR) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pptcomp: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32, ppfuncdesc: *mut *mut FUNCDESC) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Ole")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32, ppvardesc: *mut *mut VARDESC) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Ole")))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, memid: i32, rgbstrnames: *mut super::super::Foundation::BSTR, cmaxnames: u32, pcnames: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32, preftype: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32, pimpltypeflags: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, pmemid: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvinstance: *const ::core::ffi::c_void, memid: i32, wflags: u16, pdispparams: *mut DISPPARAMS, pvarresult: *mut VARIANT, pexcepinfo: *mut EXCEPINFO, puargerr: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Ole")))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, memid: i32, pbstrname: *mut super::super::Foundation::BSTR, pbstrdocstring: *mut super::super::Foundation::BSTR, pdwhelpcontext: *mut u32, pbstrhelpfile: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, memid: i32, invkind: INVOKEKIND, pbstrdllname: *mut super::super::Foundation::BSTR, pbstrname: *mut super::super::Foundation::BSTR, pwordinal: *mut u16) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hreftype: u32, pptinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, memid: i32, invkind: INVOKEKIND, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, punkouter: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, ppvobj: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, memid: i32, pbstrmops: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pptlib: *mut ::windows::core::RawPtr, pindex: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ptypeattr: *const TYPEATTR),
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Ole")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfuncdesc: *const FUNCDESC),
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Ole")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvardesc: *const VARDESC),
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ptypekind: *mut TYPEKIND) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ptypeflags: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, memid: i32, invkind: INVOKEKIND, pfuncindex: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, memid: i32, pvarindex: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, guid: *const ::windows::core::GUID, pvarval: *mut VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Ole")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32, guid: *const ::windows::core::GUID, pvarval: *mut VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Ole")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, indexfunc: u32, indexparam: u32, guid: *const ::windows::core::GUID, pvarval: *mut VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Ole")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32, guid: *const ::windows::core::GUID, pvarval: *mut VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Ole")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32, guid: *const ::windows::core::GUID, pvarval: *mut VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Ole")))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, memid: i32, lcid: u32, pbstrhelpstring: *mut super::super::Foundation::BSTR, pdwhelpstringcontext: *mut u32, pbstrhelpstringdll: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcustdata: *mut CUSTDATA) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Ole")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32, pcustdata: *mut CUSTDATA) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Ole")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, indexfunc: u32, indexparam: u32, pcustdata: *mut CUSTDATA) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Ole")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32, pcustdata: *mut CUSTDATA) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Ole")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32, pcustdata: *mut CUSTDATA) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Ole")))] usize,
);
#[doc = "*Required features: 'Win32_System_Com'*"]
#[repr(transparent)]
pub struct ITypeLib(::windows::core::IUnknown);
impl ITypeLib {
    #[doc = "*Required features: 'Win32_System_Com'*"]
    pub unsafe fn GetTypeInfoCount(&self) -> u32 {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self)))
    }
    #[doc = "*Required features: 'Win32_System_Com'*"]
    pub unsafe fn GetTypeInfo(&self, index: u32) -> ::windows::core::Result<ITypeInfo> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(index), ::core::mem::transmute(&mut result__)).from_abi::<ITypeInfo>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Com'*"]
    pub unsafe fn GetTypeInfoType(&self, index: u32) -> ::windows::core::Result<TYPEKIND> {
        let mut result__: TYPEKIND = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(index), ::core::mem::transmute(&mut result__)).from_abi::<TYPEKIND>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Com'*"]
    pub unsafe fn GetTypeInfoOfGuid(&self, guid: *const ::windows::core::GUID) -> ::windows::core::Result<ITypeInfo> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(guid), ::core::mem::transmute(&mut result__)).from_abi::<ITypeInfo>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Com'*"]
    pub unsafe fn GetLibAttr(&self) -> ::windows::core::Result<*mut TLIBATTR> {
        let mut result__: *mut TLIBATTR = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<*mut TLIBATTR>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Com'*"]
    pub unsafe fn GetTypeComp(&self) -> ::windows::core::Result<ITypeComp> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<ITypeComp>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Com', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetDocumentation(&self, index: i32, pbstrname: *mut super::super::Foundation::BSTR, pbstrdocstring: *mut super::super::Foundation::BSTR, pdwhelpcontext: *mut u32, pbstrhelpfile: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(index), ::core::mem::transmute(pbstrname), ::core::mem::transmute(pbstrdocstring), ::core::mem::transmute(pdwhelpcontext), ::core::mem::transmute(pbstrhelpfile)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Com', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsName<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, sznamebuf: Param0, lhashval: u32, pfname: *mut super::super::Foundation::BOOL) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), sznamebuf.into_param().abi(), ::core::mem::transmute(lhashval), ::core::mem::transmute(pfname)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Com', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn FindName<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, sznamebuf: Param0, lhashval: u32, pptinfo: *mut ::core::option::Option<ITypeInfo>, rgmemid: *mut i32, pcfound: *mut u16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), sznamebuf.into_param().abi(), ::core::mem::transmute(lhashval), ::core::mem::transmute(pptinfo), ::core::mem::transmute(rgmemid), ::core::mem::transmute(pcfound)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Com'*"]
    pub unsafe fn ReleaseTLibAttr(&self, ptlibattr: *const TLIBATTR) {
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), ::core::mem::transmute(ptlibattr))
    }
}
impl ::core::convert::From<ITypeLib> for ::windows::core::IUnknown {
    fn from(value: ITypeLib) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ITypeLib> for ::windows::core::IUnknown {
    fn from(value: &ITypeLib) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ITypeLib {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &ITypeLib {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ITypeLib {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ITypeLib {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITypeLib {}
impl ::core::fmt::Debug for ITypeLib {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITypeLib").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ITypeLib {
    type Vtable = ITypeLibVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x00020402_0000_0000_c000_000000000046);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITypeLibVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32, pptinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32, ptkind: *mut TYPEKIND) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, guid: *const ::windows::core::GUID, pptinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pptlibattr: *mut *mut TLIBATTR) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pptcomp: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: i32, pbstrname: *mut super::super::Foundation::BSTR, pbstrdocstring: *mut super::super::Foundation::BSTR, pdwhelpcontext: *mut u32, pbstrhelpfile: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sznamebuf: super::super::Foundation::PWSTR, lhashval: u32, pfname: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sznamebuf: super::super::Foundation::PWSTR, lhashval: u32, pptinfo: *mut ::windows::core::RawPtr, rgmemid: *mut i32, pcfound: *mut u16) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ptlibattr: *const TLIBATTR),
);
#[doc = "*Required features: 'Win32_System_Com'*"]
#[repr(transparent)]
pub struct ITypeLib2(::windows::core::IUnknown);
impl ITypeLib2 {
    #[doc = "*Required features: 'Win32_System_Com'*"]
    pub unsafe fn GetTypeInfoCount(&self) -> u32 {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self)))
    }
    #[doc = "*Required features: 'Win32_System_Com'*"]
    pub unsafe fn GetTypeInfo(&self, index: u32) -> ::windows::core::Result<ITypeInfo> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(index), ::core::mem::transmute(&mut result__)).from_abi::<ITypeInfo>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Com'*"]
    pub unsafe fn GetTypeInfoType(&self, index: u32) -> ::windows::core::Result<TYPEKIND> {
        let mut result__: TYPEKIND = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(index), ::core::mem::transmute(&mut result__)).from_abi::<TYPEKIND>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Com'*"]
    pub unsafe fn GetTypeInfoOfGuid(&self, guid: *const ::windows::core::GUID) -> ::windows::core::Result<ITypeInfo> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(guid), ::core::mem::transmute(&mut result__)).from_abi::<ITypeInfo>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Com'*"]
    pub unsafe fn GetLibAttr(&self) -> ::windows::core::Result<*mut TLIBATTR> {
        let mut result__: *mut TLIBATTR = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<*mut TLIBATTR>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Com'*"]
    pub unsafe fn GetTypeComp(&self) -> ::windows::core::Result<ITypeComp> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<ITypeComp>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Com', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetDocumentation(&self, index: i32, pbstrname: *mut super::super::Foundation::BSTR, pbstrdocstring: *mut super::super::Foundation::BSTR, pdwhelpcontext: *mut u32, pbstrhelpfile: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(index), ::core::mem::transmute(pbstrname), ::core::mem::transmute(pbstrdocstring), ::core::mem::transmute(pdwhelpcontext), ::core::mem::transmute(pbstrhelpfile)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Com', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsName<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, sznamebuf: Param0, lhashval: u32, pfname: *mut super::super::Foundation::BOOL) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), sznamebuf.into_param().abi(), ::core::mem::transmute(lhashval), ::core::mem::transmute(pfname)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Com', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn FindName<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, sznamebuf: Param0, lhashval: u32, pptinfo: *mut ::core::option::Option<ITypeInfo>, rgmemid: *mut i32, pcfound: *mut u16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), sznamebuf.into_param().abi(), ::core::mem::transmute(lhashval), ::core::mem::transmute(pptinfo), ::core::mem::transmute(rgmemid), ::core::mem::transmute(pcfound)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Com'*"]
    pub unsafe fn ReleaseTLibAttr(&self, ptlibattr: *const TLIBATTR) {
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), ::core::mem::transmute(ptlibattr))
    }
    #[doc = "*Required features: 'Win32_System_Com', 'Win32_Foundation', 'Win32_System_Ole'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
    pub unsafe fn GetCustData(&self, guid: *const ::windows::core::GUID) -> ::windows::core::Result<VARIANT> {
        let mut result__: ::core::mem::ManuallyDrop<VARIANT> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self), ::core::mem::transmute(guid), ::core::mem::transmute(&mut result__)).from_abi::<VARIANT>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Com'*"]
    pub unsafe fn GetLibStatistics(&self, pcuniquenames: *mut u32, pcchuniquenames: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).14)(::core::mem::transmute_copy(self), ::core::mem::transmute(pcuniquenames), ::core::mem::transmute(pcchuniquenames)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Com', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetDocumentation2(&self, index: i32, lcid: u32, pbstrhelpstring: *mut super::super::Foundation::BSTR, pdwhelpstringcontext: *mut u32, pbstrhelpstringdll: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).15)(::core::mem::transmute_copy(self), ::core::mem::transmute(index), ::core::mem::transmute(lcid), ::core::mem::transmute(pbstrhelpstring), ::core::mem::transmute(pdwhelpstringcontext), ::core::mem::transmute(pbstrhelpstringdll)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Com', 'Win32_Foundation', 'Win32_System_Ole'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
    pub unsafe fn GetAllCustData(&self) -> ::windows::core::Result<CUSTDATA> {
        let mut result__: CUSTDATA = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).16)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<CUSTDATA>(result__)
    }
}
impl ::core::convert::From<ITypeLib2> for ITypeLib {
    fn from(value: ITypeLib2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ITypeLib2> for ITypeLib {
    fn from(value: &ITypeLib2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ITypeLib> for ITypeLib2 {
    fn into_param(self) -> ::windows::core::Param<'a, ITypeLib> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ITypeLib> for &ITypeLib2 {
    fn into_param(self) -> ::windows::core::Param<'a, ITypeLib> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ITypeLib2> for ::windows::core::IUnknown {
    fn from(value: ITypeLib2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ITypeLib2> for ::windows::core::IUnknown {
    fn from(value: &ITypeLib2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ITypeLib2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &ITypeLib2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ITypeLib2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ITypeLib2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITypeLib2 {}
impl ::core::fmt::Debug for ITypeLib2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITypeLib2").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ITypeLib2 {
    type Vtable = ITypeLib2Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x00020411_0000_0000_c000_000000000046);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITypeLib2Vtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32, pptinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32, ptkind: *mut TYPEKIND) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, guid: *const ::windows::core::GUID, pptinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pptlibattr: *mut *mut TLIBATTR) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pptcomp: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: i32, pbstrname: *mut super::super::Foundation::BSTR, pbstrdocstring: *mut super::super::Foundation::BSTR, pdwhelpcontext: *mut u32, pbstrhelpfile: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sznamebuf: super::super::Foundation::PWSTR, lhashval: u32, pfname: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sznamebuf: super::super::Foundation::PWSTR, lhashval: u32, pptinfo: *mut ::windows::core::RawPtr, rgmemid: *mut i32, pcfound: *mut u16) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ptlibattr: *const TLIBATTR),
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, guid: *const ::windows::core::GUID, pvarval: *mut VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcuniquenames: *mut u32, pcchuniquenames: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: i32, lcid: u32, pbstrhelpstring: *mut super::super::Foundation::BSTR, pdwhelpstringcontext: *mut u32, pbstrhelpstringdll: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcustdata: *mut CUSTDATA) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Ole")))] usize,
);
#[doc = "*Required features: 'Win32_System_Com'*"]
#[repr(transparent)]
pub struct ITypeLibRegistration(::windows::core::IUnknown);
impl ITypeLibRegistration {
    #[doc = "*Required features: 'Win32_System_Com'*"]
    pub unsafe fn GetGuid(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::GUID>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Com', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetVersion(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Com'*"]
    pub unsafe fn GetLcid(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Com', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetWin32Path(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Com', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetWin64Path(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Com', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetDisplayName(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Com'*"]
    pub unsafe fn GetFlags(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Com', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetHelpDir(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
}
impl ::core::convert::From<ITypeLibRegistration> for ::windows::core::IUnknown {
    fn from(value: ITypeLibRegistration) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ITypeLibRegistration> for ::windows::core::IUnknown {
    fn from(value: &ITypeLibRegistration) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ITypeLibRegistration {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &ITypeLibRegistration {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ITypeLibRegistration {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ITypeLibRegistration {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITypeLibRegistration {}
impl ::core::fmt::Debug for ITypeLibRegistration {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITypeLibRegistration").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ITypeLibRegistration {
    type Vtable = ITypeLibRegistrationVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x76a3e735_02df_4a12_98eb_043ad3600af3);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITypeLibRegistrationVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pguid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pversion: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plcid: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwin32path: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwin64path: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdisplayname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pflags: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, phelpdir: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: 'Win32_System_Com'*"]
#[repr(transparent)]
pub struct ITypeLibRegistrationReader(::windows::core::IUnknown);
impl ITypeLibRegistrationReader {
    #[doc = "*Required features: 'Win32_System_Com'*"]
    pub unsafe fn EnumTypeLibRegistrations(&self) -> ::windows::core::Result<IEnumUnknown> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IEnumUnknown>(result__)
    }
}
impl ::core::convert::From<ITypeLibRegistrationReader> for ::windows::core::IUnknown {
    fn from(value: ITypeLibRegistrationReader) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ITypeLibRegistrationReader> for ::windows::core::IUnknown {
    fn from(value: &ITypeLibRegistrationReader) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ITypeLibRegistrationReader {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &ITypeLibRegistrationReader {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ITypeLibRegistrationReader {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ITypeLibRegistrationReader {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITypeLibRegistrationReader {}
impl ::core::fmt::Debug for ITypeLibRegistrationReader {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITypeLibRegistrationReader").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ITypeLibRegistrationReader {
    type Vtable = ITypeLibRegistrationReaderVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xed6a8a2a_b160_4e77_8f73_aa7435cd5c27);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITypeLibRegistrationReaderVtbl(pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT, pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32, pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32, pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenumunknown: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT);
#[doc = "*Required features: 'Win32_System_Com'*"]
#[repr(transparent)]
pub struct IUri(::windows::core::IUnknown);
impl IUri {
    #[doc = "*Required features: 'Win32_System_Com', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetPropertyBSTR(&self, uriprop: Uri_PROPERTY, pbstrproperty: *mut super::super::Foundation::BSTR, dwflags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(uriprop), ::core::mem::transmute(pbstrproperty), ::core::mem::transmute(dwflags)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Com'*"]
    pub unsafe fn GetPropertyLength(&self, uriprop: Uri_PROPERTY, pcchproperty: *mut u32, dwflags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(uriprop), ::core::mem::transmute(pcchproperty), ::core::mem::transmute(dwflags)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Com'*"]
    pub unsafe fn GetPropertyDWORD(&self, uriprop: Uri_PROPERTY, pdwproperty: *mut u32, dwflags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(uriprop), ::core::mem::transmute(pdwproperty), ::core::mem::transmute(dwflags)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Com', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn HasProperty(&self, uriprop: Uri_PROPERTY) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__: super::super::Foundation::BOOL = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(uriprop), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Com', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetAbsoluteUri(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Com', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetAuthority(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Com', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetDisplayUri(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Com', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetDomain(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Com', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetExtension(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Com', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetFragment(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Com', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetHost(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Com', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetPassword(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).14)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Com', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetPath(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).15)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Com', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetPathAndQuery(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).16)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Com', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetQuery(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).17)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Com', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetRawUri(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).18)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Com', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetSchemeName(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).19)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Com', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetUserInfo(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).20)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Com', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetUserName(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).21)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Com'*"]
    pub unsafe fn GetHostType(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).22)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Com'*"]
    pub unsafe fn GetPort(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).23)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Com'*"]
    pub unsafe fn GetScheme(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).24)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Com'*"]
    pub unsafe fn GetZone(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).25)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Com'*"]
    pub unsafe fn GetProperties(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).26)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Com', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsEqual<'a, Param0: ::windows::core::IntoParam<'a, IUri>>(&self, puri: Param0) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__: super::super::Foundation::BOOL = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).27)(::core::mem::transmute_copy(self), puri.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BOOL>(result__)
    }
}
impl ::core::convert::From<IUri> for ::windows::core::IUnknown {
    fn from(value: IUri) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IUri> for ::windows::core::IUnknown {
    fn from(value: &IUri) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IUri {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IUri {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IUri {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IUri {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IUri {}
impl ::core::fmt::Debug for IUri {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IUri").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IUri {
    type Vtable = IUriVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa39ee748_6a27_4817_a6f2_13914bef5890);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUriVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uriprop: Uri_PROPERTY, pbstrproperty: *mut super::super::Foundation::BSTR, dwflags: u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uriprop: Uri_PROPERTY, pcchproperty: *mut u32, dwflags: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uriprop: Uri_PROPERTY, pdwproperty: *mut u32, dwflags: u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uriprop: Uri_PROPERTY, pfhasproperty: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrabsoluteuri: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrauthority: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrdisplaystring: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrdomain: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrextension: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrfragment: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrhost: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrpassword: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrpath: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrpathandquery: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrquery: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrrawuri: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrschemename: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstruserinfo: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrusername: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwhosttype: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwport: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwscheme: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwzone: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwflags: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, puri: ::windows::core::RawPtr, pfequal: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: 'Win32_System_Com'*"]
#[repr(transparent)]
pub struct IUriBuilder(::windows::core::IUnknown);
impl IUriBuilder {
    #[doc = "*Required features: 'Win32_System_Com'*"]
    pub unsafe fn CreateUriSimple(&self, dwallowencodingpropertymask: u32, dwreserved: usize) -> ::windows::core::Result<IUri> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwallowencodingpropertymask), ::core::mem::transmute(dwreserved), ::core::mem::transmute(&mut result__)).from_abi::<IUri>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Com'*"]
    pub unsafe fn CreateUri(&self, dwcreateflags: u32, dwallowencodingpropertymask: u32, dwreserved: usize) -> ::windows::core::Result<IUri> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwcreateflags), ::core::mem::transmute(dwallowencodingpropertymask), ::core::mem::transmute(dwreserved), ::core::mem::transmute(&mut result__)).from_abi::<IUri>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Com'*"]
    pub unsafe fn CreateUriWithFlags(&self, dwcreateflags: u32, dwuribuilderflags: u32, dwallowencodingpropertymask: u32, dwreserved: usize) -> ::windows::core::Result<IUri> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwcreateflags), ::core::mem::transmute(dwuribuilderflags), ::core::mem::transmute(dwallowencodingpropertymask), ::core::mem::transmute(dwreserved), ::core::mem::transmute(&mut result__)).from_abi::<IUri>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Com'*"]
    pub unsafe fn GetIUri(&self) -> ::windows::core::Result<IUri> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IUri>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Com'*"]
    pub unsafe fn SetIUri<'a, Param0: ::windows::core::IntoParam<'a, IUri>>(&self, piuri: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), piuri.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_System_Com', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetFragment(&self, pcchfragment: *mut u32, ppwzfragment: *mut super::super::Foundation::PWSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(pcchfragment), ::core::mem::transmute(ppwzfragment)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Com', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetHost(&self, pcchhost: *mut u32, ppwzhost: *mut super::super::Foundation::PWSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(pcchhost), ::core::mem::transmute(ppwzhost)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Com', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetPassword(&self, pcchpassword: *mut u32, ppwzpassword: *mut super::super::Foundation::PWSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), ::core::mem::transmute(pcchpassword), ::core::mem::transmute(ppwzpassword)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Com', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetPath(&self, pcchpath: *mut u32, ppwzpath: *mut super::super::Foundation::PWSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), ::core::mem::transmute(pcchpath), ::core::mem::transmute(ppwzpath)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Com', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetPort(&self, pfhasport: *mut super::super::Foundation::BOOL, pdwport: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), ::core::mem::transmute(pfhasport), ::core::mem::transmute(pdwport)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Com', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetQuery(&self, pcchquery: *mut u32, ppwzquery: *mut super::super::Foundation::PWSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self), ::core::mem::transmute(pcchquery), ::core::mem::transmute(ppwzquery)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Com', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetSchemeName(&self, pcchschemename: *mut u32, ppwzschemename: *mut super::super::Foundation::PWSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).14)(::core::mem::transmute_copy(self), ::core::mem::transmute(pcchschemename), ::core::mem::transmute(ppwzschemename)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Com', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetUserName(&self, pcchusername: *mut u32, ppwzusername: *mut super::super::Foundation::PWSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).15)(::core::mem::transmute_copy(self), ::core::mem::transmute(pcchusername), ::core::mem::transmute(ppwzusername)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Com', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetFragment<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pwznewvalue: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).16)(::core::mem::transmute_copy(self), pwznewvalue.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_System_Com', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetHost<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pwznewvalue: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).17)(::core::mem::transmute_copy(self), pwznewvalue.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_System_Com', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetPassword<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pwznewvalue: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).18)(::core::mem::transmute_copy(self), pwznewvalue.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_System_Com', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetPath<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pwznewvalue: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).19)(::core::mem::transmute_copy(self), pwznewvalue.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_System_Com', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetPort<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, fhasport: Param0, dwnewvalue: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).20)(::core::mem::transmute_copy(self), fhasport.into_param().abi(), ::core::mem::transmute(dwnewvalue)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Com', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetQuery<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pwznewvalue: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).21)(::core::mem::transmute_copy(self), pwznewvalue.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_System_Com', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetSchemeName<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pwznewvalue: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).22)(::core::mem::transmute_copy(self), pwznewvalue.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_System_Com', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetUserName<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pwznewvalue: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).23)(::core::mem::transmute_copy(self), pwznewvalue.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_System_Com'*"]
    pub unsafe fn RemoveProperties(&self, dwpropertymask: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).24)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwpropertymask)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Com', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn HasBeenModified(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__: super::super::Foundation::BOOL = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).25)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BOOL>(result__)
    }
}
impl ::core::convert::From<IUriBuilder> for ::windows::core::IUnknown {
    fn from(value: IUriBuilder) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IUriBuilder> for ::windows::core::IUnknown {
    fn from(value: &IUriBuilder) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IUriBuilder {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IUriBuilder {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IUriBuilder {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IUriBuilder {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IUriBuilder {}
impl ::core::fmt::Debug for IUriBuilder {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IUriBuilder").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IUriBuilder {
    type Vtable = IUriBuilderVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4221b2e1_8955_46c0_bd5b_de9897565de7);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUriBuilderVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwallowencodingpropertymask: u32, dwreserved: usize, ppiuri: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwcreateflags: u32, dwallowencodingpropertymask: u32, dwreserved: usize, ppiuri: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwcreateflags: u32, dwuribuilderflags: u32, dwallowencodingpropertymask: u32, dwreserved: usize, ppiuri: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppiuri: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, piuri: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcchfragment: *mut u32, ppwzfragment: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcchhost: *mut u32, ppwzhost: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcchpassword: *mut u32, ppwzpassword: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcchpath: *mut u32, ppwzpath: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfhasport: *mut super::super::Foundation::BOOL, pdwport: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcchquery: *mut u32, ppwzquery: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcchschemename: *mut u32, ppwzschemename: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcchusername: *mut u32, ppwzusername: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwznewvalue: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwznewvalue: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwznewvalue: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwznewvalue: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fhasport: super::super::Foundation::BOOL, dwnewvalue: u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwznewvalue: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwznewvalue: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwznewvalue: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwpropertymask: u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfmodified: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: 'Win32_System_Com'*"]
#[repr(transparent)]
pub struct IUrlMon(::windows::core::IUnknown);
impl IUrlMon {
    #[doc = "*Required features: 'Win32_System_Com', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn AsyncGetClassBits<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param5: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param6: ::windows::core::IntoParam<'a, IBindCtx>>(&self, rclsid: *const ::windows::core::GUID, psztype: Param1, pszext: Param2, dwfileversionms: u32, dwfileversionls: u32, pszcodebase: Param5, pbc: Param6, dwclasscontext: u32, riid: *const ::windows::core::GUID, flags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(rclsid), psztype.into_param().abi(), pszext.into_param().abi(), ::core::mem::transmute(dwfileversionms), ::core::mem::transmute(dwfileversionls), pszcodebase.into_param().abi(), pbc.into_param().abi(), ::core::mem::transmute(dwclasscontext), ::core::mem::transmute(riid), ::core::mem::transmute(flags)).ok()
    }
}
impl ::core::convert::From<IUrlMon> for ::windows::core::IUnknown {
    fn from(value: IUrlMon) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IUrlMon> for ::windows::core::IUnknown {
    fn from(value: &IUrlMon) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IUrlMon {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IUrlMon {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IUrlMon {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IUrlMon {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IUrlMon {}
impl ::core::fmt::Debug for IUrlMon {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IUrlMon").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IUrlMon {
    type Vtable = IUrlMonVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x00000026_0000_0000_c000_000000000046);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUrlMonVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, rclsid: *const ::windows::core::GUID, psztype: super::super::Foundation::PWSTR, pszext: super::super::Foundation::PWSTR, dwfileversionms: u32, dwfileversionls: u32, pszcodebase: super::super::Foundation::PWSTR, pbc: ::windows::core::RawPtr, dwclasscontext: u32, riid: *const ::windows::core::GUID, flags: u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: 'Win32_System_Com'*"]
#[repr(transparent)]
pub struct IWaitMultiple(::windows::core::IUnknown);
impl IWaitMultiple {
    #[doc = "*Required features: 'Win32_System_Com'*"]
    pub unsafe fn WaitMultiple(&self, timeout: u32) -> ::windows::core::Result<ISynchronize> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(timeout), ::core::mem::transmute(&mut result__)).from_abi::<ISynchronize>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Com'*"]
    pub unsafe fn AddSynchronize<'a, Param0: ::windows::core::IntoParam<'a, ISynchronize>>(&self, psync: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), psync.into_param().abi()).ok()
    }
}
impl ::core::convert::From<IWaitMultiple> for ::windows::core::IUnknown {
    fn from(value: IWaitMultiple) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWaitMultiple> for ::windows::core::IUnknown {
    fn from(value: &IWaitMultiple) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWaitMultiple {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IWaitMultiple {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWaitMultiple {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWaitMultiple {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWaitMultiple {}
impl ::core::fmt::Debug for IWaitMultiple {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWaitMultiple").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IWaitMultiple {
    type Vtable = IWaitMultipleVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0000002b_0000_0000_c000_000000000046);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWaitMultipleVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, timeout: u32, psync: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, psync: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(C)]
#[doc = "*Required features: 'Win32_System_Com'*"]
pub struct LONG_SIZEDARR {
    pub clSize: u32,
    pub pData: *mut u32,
}
impl ::core::marker::Copy for LONG_SIZEDARR {}
impl ::core::clone::Clone for LONG_SIZEDARR {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for LONG_SIZEDARR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("LONG_SIZEDARR").field("clSize", &self.clSize).field("pData", &self.pData).finish()
    }
}
unsafe impl ::windows::core::Abi for LONG_SIZEDARR {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for LONG_SIZEDARR {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<LONG_SIZEDARR>()) == 0 }
    }
}
impl ::core::cmp::Eq for LONG_SIZEDARR {}
impl ::core::default::Default for LONG_SIZEDARR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: 'Win32_System_Com', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub type LPEXCEPFINO_DEFERRED_FILLIN = ::core::option::Option<unsafe extern "system" fn(pexcepinfo: *mut EXCEPINFO) -> ::windows::core::HRESULT>;
#[doc = "*Required features: 'Win32_System_Com'*"]
pub type LPFNCANUNLOADNOW = ::core::option::Option<unsafe extern "system" fn() -> ::windows::core::HRESULT>;
#[doc = "*Required features: 'Win32_System_Com'*"]
pub type LPFNGETCLASSOBJECT = ::core::option::Option<unsafe extern "system" fn(param0: *const ::windows::core::GUID, param1: *const ::windows::core::GUID, param2: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT>;
#[doc = "*Required features: 'Win32_System_Com'*"]
pub const MARSHALINTERFACE_MIN: u32 = 500u32;
#[doc = "*Required features: 'Win32_System_Com'*"]
pub const MAXLSN: u64 = 9223372036854775807u64;
#[doc = "*Required features: 'Win32_System_Com'*"]
pub type MEMCTX = i32;
#[doc = "*Required features: 'Win32_System_Com'*"]
pub const MEMCTX_TASK: MEMCTX = 1i32;
#[doc = "*Required features: 'Win32_System_Com'*"]
pub const MEMCTX_SHARED: MEMCTX = 2i32;
#[doc = "*Required features: 'Win32_System_Com'*"]
pub const MEMCTX_MACSYSTEM: MEMCTX = 3i32;
#[doc = "*Required features: 'Win32_System_Com'*"]
pub const MEMCTX_UNKNOWN: MEMCTX = -1i32;
#[doc = "*Required features: 'Win32_System_Com'*"]
pub const MEMCTX_SAME: MEMCTX = -2i32;
#[doc = "*Required features: 'Win32_System_Com'*"]
pub type MKREDUCE = i32;
#[doc = "*Required features: 'Win32_System_Com'*"]
pub const MKRREDUCE_ONE: MKREDUCE = 196608i32;
#[doc = "*Required features: 'Win32_System_Com'*"]
pub const MKRREDUCE_TOUSER: MKREDUCE = 131072i32;
#[doc = "*Required features: 'Win32_System_Com'*"]
pub const MKRREDUCE_THROUGHUSER: MKREDUCE = 65536i32;
#[doc = "*Required features: 'Win32_System_Com'*"]
pub const MKRREDUCE_ALL: MKREDUCE = 0i32;
#[doc = "*Required features: 'Win32_System_Com'*"]
pub type MKSYS = i32;
#[doc = "*Required features: 'Win32_System_Com'*"]
pub const MKSYS_NONE: MKSYS = 0i32;
#[doc = "*Required features: 'Win32_System_Com'*"]
pub const MKSYS_GENERICCOMPOSITE: MKSYS = 1i32;
#[doc = "*Required features: 'Win32_System_Com'*"]
pub const MKSYS_FILEMONIKER: MKSYS = 2i32;
#[doc = "*Required features: 'Win32_System_Com'*"]
pub const MKSYS_ANTIMONIKER: MKSYS = 3i32;
#[doc = "*Required features: 'Win32_System_Com'*"]
pub const MKSYS_ITEMMONIKER: MKSYS = 4i32;
#[doc = "*Required features: 'Win32_System_Com'*"]
pub const MKSYS_POINTERMONIKER: MKSYS = 5i32;
#[doc = "*Required features: 'Win32_System_Com'*"]
pub const MKSYS_CLASSMONIKER: MKSYS = 7i32;
#[doc = "*Required features: 'Win32_System_Com'*"]
pub const MKSYS_OBJREFMONIKER: MKSYS = 8i32;
#[doc = "*Required features: 'Win32_System_Com'*"]
pub const MKSYS_SESSIONMONIKER: MKSYS = 9i32;
#[doc = "*Required features: 'Win32_System_Com'*"]
pub const MKSYS_LUAMONIKER: MKSYS = 10i32;
#[doc = "*Required features: 'Win32_System_Com'*"]
pub type MSHCTX = i32;
#[doc = "*Required features: 'Win32_System_Com'*"]
pub const MSHCTX_LOCAL: MSHCTX = 0i32;
#[doc = "*Required features: 'Win32_System_Com'*"]
pub const MSHCTX_NOSHAREDMEM: MSHCTX = 1i32;
#[doc = "*Required features: 'Win32_System_Com'*"]
pub const MSHCTX_DIFFERENTMACHINE: MSHCTX = 2i32;
#[doc = "*Required features: 'Win32_System_Com'*"]
pub const MSHCTX_INPROC: MSHCTX = 3i32;
#[doc = "*Required features: 'Win32_System_Com'*"]
pub const MSHCTX_CROSSCTX: MSHCTX = 4i32;
#[doc = "*Required features: 'Win32_System_Com'*"]
pub const MSHCTX_CONTAINER: MSHCTX = 5i32;
#[doc = "*Required features: 'Win32_System_Com'*"]
pub type MSHLFLAGS = i32;
#[doc = "*Required features: 'Win32_System_Com'*"]
pub const MSHLFLAGS_NORMAL: MSHLFLAGS = 0i32;
#[doc = "*Required features: 'Win32_System_Com'*"]
pub const MSHLFLAGS_TABLESTRONG: MSHLFLAGS = 1i32;
#[doc = "*Required features: 'Win32_System_Com'*"]
pub const MSHLFLAGS_TABLEWEAK: MSHLFLAGS = 2i32;
#[doc = "*Required features: 'Win32_System_Com'*"]
pub const MSHLFLAGS_NOPING: MSHLFLAGS = 4i32;
#[doc = "*Required features: 'Win32_System_Com'*"]
pub const MSHLFLAGS_RESERVED1: MSHLFLAGS = 8i32;
#[doc = "*Required features: 'Win32_System_Com'*"]
pub const MSHLFLAGS_RESERVED2: MSHLFLAGS = 16i32;
#[doc = "*Required features: 'Win32_System_Com'*"]
pub const MSHLFLAGS_RESERVED3: MSHLFLAGS = 32i32;
#[doc = "*Required features: 'Win32_System_Com'*"]
pub const MSHLFLAGS_RESERVED4: MSHLFLAGS = 64i32;
#[repr(C)]
#[doc = "*Required features: 'Win32_System_Com'*"]
pub struct MULTI_QI {
    pub pIID: *mut ::windows::core::GUID,
    pub pItf: ::core::option::Option<::windows::core::IUnknown>,
    pub hr: ::windows::core::HRESULT,
}
impl ::core::clone::Clone for MULTI_QI {
    fn clone(&self) -> Self {
        Self { pIID: self.pIID, pItf: self.pItf.clone(), hr: self.hr }
    }
}
impl ::core::fmt::Debug for MULTI_QI {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MULTI_QI").field("pIID", &self.pIID).field("pItf", &self.pItf).field("hr", &self.hr).finish()
    }
}
unsafe impl ::windows::core::Abi for MULTI_QI {
    type Abi = ::core::mem::ManuallyDrop<Self>;
}
impl ::core::cmp::PartialEq for MULTI_QI {
    fn eq(&self, other: &Self) -> bool {
        self.pIID == other.pIID && self.pItf == other.pItf && self.hr == other.hr
    }
}
impl ::core::cmp::Eq for MULTI_QI {}
impl ::core::default::Default for MULTI_QI {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_System_Com'*"]
pub struct MachineGlobalObjectTableRegistrationToken__ {
    pub unused: i32,
}
impl ::core::marker::Copy for MachineGlobalObjectTableRegistrationToken__ {}
impl ::core::clone::Clone for MachineGlobalObjectTableRegistrationToken__ {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MachineGlobalObjectTableRegistrationToken__ {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MachineGlobalObjectTableRegistrationToken__").field("unused", &self.unused).finish()
    }
}
unsafe impl ::windows::core::Abi for MachineGlobalObjectTableRegistrationToken__ {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MachineGlobalObjectTableRegistrationToken__ {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MachineGlobalObjectTableRegistrationToken__>()) == 0 }
    }
}
impl ::core::cmp::Eq for MachineGlobalObjectTableRegistrationToken__ {}
impl ::core::default::Default for MachineGlobalObjectTableRegistrationToken__ {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: 'Win32_System_Com', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MkParseDisplayName<'a, Param0: ::windows::core::IntoParam<'a, IBindCtx>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(pbc: Param0, szusername: Param1, pcheaten: *mut u32, ppmk: *mut ::core::option::Option<IMoniker>) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MkParseDisplayName(pbc: ::windows::core::RawPtr, szusername: super::super::Foundation::PWSTR, pcheaten: *mut u32, ppmk: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT;
        }
        MkParseDisplayName(pbc.into_param().abi(), szusername.into_param().abi(), ::core::mem::transmute(pcheaten), ::core::mem::transmute(ppmk)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_System_Com'*"]
#[inline]
pub unsafe fn MonikerCommonPrefixWith<'a, Param0: ::windows::core::IntoParam<'a, IMoniker>, Param1: ::windows::core::IntoParam<'a, IMoniker>>(pmkthis: Param0, pmkother: Param1) -> ::windows::core::Result<IMoniker> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MonikerCommonPrefixWith(pmkthis: ::windows::core::RawPtr, pmkother: ::windows::core::RawPtr, ppmkcommon: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT;
        }
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        MonikerCommonPrefixWith(pmkthis.into_param().abi(), pmkother.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IMoniker>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_System_Com', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MonikerRelativePathTo<'a, Param0: ::windows::core::IntoParam<'a, IMoniker>, Param1: ::windows::core::IntoParam<'a, IMoniker>, Param3: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(pmksrc: Param0, pmkdest: Param1, ppmkrelpath: *mut ::core::option::Option<IMoniker>, dwreserved: Param3) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MonikerRelativePathTo(pmksrc: ::windows::core::RawPtr, pmkdest: ::windows::core::RawPtr, ppmkrelpath: *mut ::windows::core::RawPtr, dwreserved: super::super::Foundation::BOOL) -> ::windows::core::HRESULT;
        }
        MonikerRelativePathTo(pmksrc.into_param().abi(), pmkdest.into_param().abi(), ::core::mem::transmute(ppmkrelpath), dwreserved.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_System_Com'*"]
pub type PENDINGMSG = i32;
#[doc = "*Required features: 'Win32_System_Com'*"]
pub const PENDINGMSG_CANCELCALL: PENDINGMSG = 0i32;
#[doc = "*Required features: 'Win32_System_Com'*"]
pub const PENDINGMSG_WAITNOPROCESS: PENDINGMSG = 1i32;
#[doc = "*Required features: 'Win32_System_Com'*"]
pub const PENDINGMSG_WAITDEFPROCESS: PENDINGMSG = 2i32;
#[doc = "*Required features: 'Win32_System_Com'*"]
pub type PENDINGTYPE = i32;
#[doc = "*Required features: 'Win32_System_Com'*"]
pub const PENDINGTYPE_TOPLEVEL: PENDINGTYPE = 1i32;
#[doc = "*Required features: 'Win32_System_Com'*"]
pub const PENDINGTYPE_NESTED: PENDINGTYPE = 2i32;
#[doc = "*Required features: 'Win32_System_Com'*"]
pub type PFNCONTEXTCALL = ::core::option::Option<unsafe extern "system" fn(pparam: *mut ComCallData) -> ::windows::core::HRESULT>;
#[doc = "*Required features: 'Win32_System_Com', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ProgIDFromCLSID(clsid: *const ::windows::core::GUID) -> ::windows::core::Result<super::super::Foundation::PWSTR> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ProgIDFromCLSID(clsid: *const ::windows::core::GUID, lplpszprogid: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT;
        }
        let mut result__: super::super::Foundation::PWSTR = ::core::mem::zeroed();
        ProgIDFromCLSID(::core::mem::transmute(clsid), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::PWSTR>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[repr(C)]
#[doc = "*Required features: 'Win32_System_Com'*"]
pub struct QUERYCONTEXT {
    pub dwContext: u32,
    pub Platform: CSPLATFORM,
    pub Locale: u32,
    pub dwVersionHi: u32,
    pub dwVersionLo: u32,
}
impl ::core::marker::Copy for QUERYCONTEXT {}
impl ::core::clone::Clone for QUERYCONTEXT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for QUERYCONTEXT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("QUERYCONTEXT").field("dwContext", &self.dwContext).field("Platform", &self.Platform).field("Locale", &self.Locale).field("dwVersionHi", &self.dwVersionHi).field("dwVersionLo", &self.dwVersionLo).finish()
    }
}
unsafe impl ::windows::core::Abi for QUERYCONTEXT {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for QUERYCONTEXT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<QUERYCONTEXT>()) == 0 }
    }
}
impl ::core::cmp::Eq for QUERYCONTEXT {}
impl ::core::default::Default for QUERYCONTEXT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: 'Win32_System_Com'*"]
pub type REGCLS = i32;
#[doc = "*Required features: 'Win32_System_Com'*"]
pub const REGCLS_SINGLEUSE: REGCLS = 0i32;
#[doc = "*Required features: 'Win32_System_Com'*"]
pub const REGCLS_MULTIPLEUSE: REGCLS = 1i32;
#[doc = "*Required features: 'Win32_System_Com'*"]
pub const REGCLS_MULTI_SEPARATE: REGCLS = 2i32;
#[doc = "*Required features: 'Win32_System_Com'*"]
pub const REGCLS_SUSPENDED: REGCLS = 4i32;
#[doc = "*Required features: 'Win32_System_Com'*"]
pub const REGCLS_SURROGATE: REGCLS = 8i32;
#[doc = "*Required features: 'Win32_System_Com'*"]
pub const REGCLS_AGILE: REGCLS = 16i32;
#[doc = "*Required features: 'Win32_System_Com'*"]
pub const ROTREGFLAGS_ALLOWANYCLIENT: u32 = 1u32;
#[repr(C)]
#[doc = "*Required features: 'Win32_System_Com'*"]
pub struct RPCOLEMESSAGE {
    pub reserved1: *mut ::core::ffi::c_void,
    pub dataRepresentation: u32,
    pub Buffer: *mut ::core::ffi::c_void,
    pub cbBuffer: u32,
    pub iMethod: u32,
    pub reserved2: [*mut ::core::ffi::c_void; 5],
    pub rpcFlags: u32,
}
impl ::core::marker::Copy for RPCOLEMESSAGE {}
impl ::core::clone::Clone for RPCOLEMESSAGE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for RPCOLEMESSAGE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RPCOLEMESSAGE").field("reserved1", &self.reserved1).field("dataRepresentation", &self.dataRepresentation).field("Buffer", &self.Buffer).field("cbBuffer", &self.cbBuffer).field("iMethod", &self.iMethod).field("reserved2", &self.reserved2).field("rpcFlags", &self.rpcFlags).finish()
    }
}
unsafe impl ::windows::core::Abi for RPCOLEMESSAGE {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for RPCOLEMESSAGE {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<RPCOLEMESSAGE>()) == 0 }
    }
}
impl ::core::cmp::Eq for RPCOLEMESSAGE {}
impl ::core::default::Default for RPCOLEMESSAGE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: 'Win32_System_Com'*"]
pub type RPCOPT_PROPERTIES = i32;
#[doc = "*Required features: 'Win32_System_Com'*"]
pub const COMBND_RPCTIMEOUT: RPCOPT_PROPERTIES = 1i32;
#[doc = "*Required features: 'Win32_System_Com'*"]
pub const COMBND_SERVER_LOCALITY: RPCOPT_PROPERTIES = 2i32;
#[doc = "*Required features: 'Win32_System_Com'*"]
pub const COMBND_RESERVED1: RPCOPT_PROPERTIES = 4i32;
#[doc = "*Required features: 'Win32_System_Com'*"]
pub const COMBND_RESERVED2: RPCOPT_PROPERTIES = 5i32;
#[doc = "*Required features: 'Win32_System_Com'*"]
pub const COMBND_RESERVED3: RPCOPT_PROPERTIES = 8i32;
#[doc = "*Required features: 'Win32_System_Com'*"]
pub const COMBND_RESERVED4: RPCOPT_PROPERTIES = 16i32;
#[doc = "*Required features: 'Win32_System_Com'*"]
pub type RPCOPT_SERVER_LOCALITY_VALUES = i32;
#[doc = "*Required features: 'Win32_System_Com'*"]
pub const SERVER_LOCALITY_PROCESS_LOCAL: RPCOPT_SERVER_LOCALITY_VALUES = 0i32;
#[doc = "*Required features: 'Win32_System_Com'*"]
pub const SERVER_LOCALITY_MACHINE_LOCAL: RPCOPT_SERVER_LOCALITY_VALUES = 1i32;
#[doc = "*Required features: 'Win32_System_Com'*"]
pub const SERVER_LOCALITY_REMOTE: RPCOPT_SERVER_LOCALITY_VALUES = 2i32;
#[doc = "*Required features: 'Win32_System_Com'*"]
pub type RPC_C_AUTHN_LEVEL = u32;
#[doc = "*Required features: 'Win32_System_Com'*"]
pub const RPC_C_AUTHN_LEVEL_DEFAULT: RPC_C_AUTHN_LEVEL = 0u32;
#[doc = "*Required features: 'Win32_System_Com'*"]
pub const RPC_C_AUTHN_LEVEL_NONE: RPC_C_AUTHN_LEVEL = 1u32;
#[doc = "*Required features: 'Win32_System_Com'*"]
pub const RPC_C_AUTHN_LEVEL_CONNECT: RPC_C_AUTHN_LEVEL = 2u32;
#[doc = "*Required features: 'Win32_System_Com'*"]
pub const RPC_C_AUTHN_LEVEL_CALL: RPC_C_AUTHN_LEVEL = 3u32;
#[doc = "*Required features: 'Win32_System_Com'*"]
pub const RPC_C_AUTHN_LEVEL_PKT: RPC_C_AUTHN_LEVEL = 4u32;
#[doc = "*Required features: 'Win32_System_Com'*"]
pub const RPC_C_AUTHN_LEVEL_PKT_INTEGRITY: RPC_C_AUTHN_LEVEL = 5u32;
#[doc = "*Required features: 'Win32_System_Com'*"]
pub const RPC_C_AUTHN_LEVEL_PKT_PRIVACY: RPC_C_AUTHN_LEVEL = 6u32;
#[doc = "*Required features: 'Win32_System_Com'*"]
pub type RPC_C_IMP_LEVEL = u32;
#[doc = "*Required features: 'Win32_System_Com'*"]
pub const RPC_C_IMP_LEVEL_DEFAULT: RPC_C_IMP_LEVEL = 0u32;
#[doc = "*Required features: 'Win32_System_Com'*"]
pub const RPC_C_IMP_LEVEL_ANONYMOUS: RPC_C_IMP_LEVEL = 1u32;
#[doc = "*Required features: 'Win32_System_Com'*"]
pub const RPC_C_IMP_LEVEL_IDENTIFY: RPC_C_IMP_LEVEL = 2u32;
#[doc = "*Required features: 'Win32_System_Com'*"]
pub const RPC_C_IMP_LEVEL_IMPERSONATE: RPC_C_IMP_LEVEL = 3u32;
#[doc = "*Required features: 'Win32_System_Com'*"]
pub const RPC_C_IMP_LEVEL_DELEGATE: RPC_C_IMP_LEVEL = 4u32;
#[repr(C)]
#[doc = "*Required features: 'Win32_System_Com'*"]
pub struct RemSTGMEDIUM {
    pub tymed: u32,
    pub dwHandleType: u32,
    pub pData: u32,
    pub pUnkForRelease: u32,
    pub cbData: u32,
    pub data: [u8; 1],
}
impl ::core::marker::Copy for RemSTGMEDIUM {}
impl ::core::clone::Clone for RemSTGMEDIUM {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for RemSTGMEDIUM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RemSTGMEDIUM").field("tymed", &self.tymed).field("dwHandleType", &self.dwHandleType).field("pData", &self.pData).field("pUnkForRelease", &self.pUnkForRelease).field("cbData", &self.cbData).field("data", &self.data).finish()
    }
}
unsafe impl ::windows::core::Abi for RemSTGMEDIUM {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for RemSTGMEDIUM {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<RemSTGMEDIUM>()) == 0 }
    }
}
impl ::core::cmp::Eq for RemSTGMEDIUM {}
impl ::core::default::Default for RemSTGMEDIUM {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_System_Com'*"]
pub struct SAFEARRAY {
    pub cDims: u16,
    pub fFeatures: u16,
    pub cbElements: u32,
    pub cLocks: u32,
    pub pvData: *mut ::core::ffi::c_void,
    pub rgsabound: [SAFEARRAYBOUND; 1],
}
impl ::core::marker::Copy for SAFEARRAY {}
impl ::core::clone::Clone for SAFEARRAY {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SAFEARRAY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SAFEARRAY").field("cDims", &self.cDims).field("fFeatures", &self.fFeatures).field("cbElements", &self.cbElements).field("cLocks", &self.cLocks).field("pvData", &self.pvData).field("rgsabound", &self.rgsabound).finish()
    }
}
unsafe impl ::windows::core::Abi for SAFEARRAY {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for SAFEARRAY {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SAFEARRAY>()) == 0 }
    }
}
impl ::core::cmp::Eq for SAFEARRAY {}
impl ::core::default::Default for SAFEARRAY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_System_Com'*"]
pub struct SAFEARRAYBOUND {
    pub cElements: u32,
    pub lLbound: i32,
}
impl ::core::marker::Copy for SAFEARRAYBOUND {}
impl ::core::clone::Clone for SAFEARRAYBOUND {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SAFEARRAYBOUND {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SAFEARRAYBOUND").field("cElements", &self.cElements).field("lLbound", &self.lLbound).finish()
    }
}
unsafe impl ::windows::core::Abi for SAFEARRAYBOUND {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for SAFEARRAYBOUND {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SAFEARRAYBOUND>()) == 0 }
    }
}
impl ::core::cmp::Eq for SAFEARRAYBOUND {}
impl ::core::default::Default for SAFEARRAYBOUND {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_System_Com'*"]
pub struct SChannelHookCallInfo {
    pub iid: ::windows::core::GUID,
    pub cbSize: u32,
    pub uCausality: ::windows::core::GUID,
    pub dwServerPid: u32,
    pub iMethod: u32,
    pub pObject: *mut ::core::ffi::c_void,
}
impl ::core::marker::Copy for SChannelHookCallInfo {}
impl ::core::clone::Clone for SChannelHookCallInfo {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SChannelHookCallInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SChannelHookCallInfo").field("iid", &self.iid).field("cbSize", &self.cbSize).field("uCausality", &self.uCausality).field("dwServerPid", &self.dwServerPid).field("iMethod", &self.iMethod).field("pObject", &self.pObject).finish()
    }
}
unsafe impl ::windows::core::Abi for SChannelHookCallInfo {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for SChannelHookCallInfo {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SChannelHookCallInfo>()) == 0 }
    }
}
impl ::core::cmp::Eq for SChannelHookCallInfo {}
impl ::core::default::Default for SChannelHookCallInfo {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: 'Win32_System_Com'*"]
pub type SERVERCALL = i32;
#[doc = "*Required features: 'Win32_System_Com'*"]
pub const SERVERCALL_ISHANDLED: SERVERCALL = 0i32;
#[doc = "*Required features: 'Win32_System_Com'*"]
pub const SERVERCALL_REJECTED: SERVERCALL = 1i32;
#[doc = "*Required features: 'Win32_System_Com'*"]
pub const SERVERCALL_RETRYLATER: SERVERCALL = 2i32;
#[repr(C)]
#[doc = "*Required features: 'Win32_System_Com'*"]
pub struct SHORT_SIZEDARR {
    pub clSize: u32,
    pub pData: *mut u16,
}
impl ::core::marker::Copy for SHORT_SIZEDARR {}
impl ::core::clone::Clone for SHORT_SIZEDARR {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SHORT_SIZEDARR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SHORT_SIZEDARR").field("clSize", &self.clSize).field("pData", &self.pData).finish()
    }
}
unsafe impl ::windows::core::Abi for SHORT_SIZEDARR {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for SHORT_SIZEDARR {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SHORT_SIZEDARR>()) == 0 }
    }
}
impl ::core::cmp::Eq for SHORT_SIZEDARR {}
impl ::core::default::Default for SHORT_SIZEDARR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_System_Com'*"]
pub struct SOLE_AUTHENTICATION_INFO {
    pub dwAuthnSvc: u32,
    pub dwAuthzSvc: u32,
    pub pAuthInfo: *mut ::core::ffi::c_void,
}
impl ::core::marker::Copy for SOLE_AUTHENTICATION_INFO {}
impl ::core::clone::Clone for SOLE_AUTHENTICATION_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SOLE_AUTHENTICATION_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SOLE_AUTHENTICATION_INFO").field("dwAuthnSvc", &self.dwAuthnSvc).field("dwAuthzSvc", &self.dwAuthzSvc).field("pAuthInfo", &self.pAuthInfo).finish()
    }
}
unsafe impl ::windows::core::Abi for SOLE_AUTHENTICATION_INFO {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for SOLE_AUTHENTICATION_INFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SOLE_AUTHENTICATION_INFO>()) == 0 }
    }
}
impl ::core::cmp::Eq for SOLE_AUTHENTICATION_INFO {}
impl ::core::default::Default for SOLE_AUTHENTICATION_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_System_Com'*"]
pub struct SOLE_AUTHENTICATION_LIST {
    pub cAuthInfo: u32,
    pub aAuthInfo: *mut SOLE_AUTHENTICATION_INFO,
}
impl ::core::marker::Copy for SOLE_AUTHENTICATION_LIST {}
impl ::core::clone::Clone for SOLE_AUTHENTICATION_LIST {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SOLE_AUTHENTICATION_LIST {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SOLE_AUTHENTICATION_LIST").field("cAuthInfo", &self.cAuthInfo).field("aAuthInfo", &self.aAuthInfo).finish()
    }
}
unsafe impl ::windows::core::Abi for SOLE_AUTHENTICATION_LIST {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for SOLE_AUTHENTICATION_LIST {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SOLE_AUTHENTICATION_LIST>()) == 0 }
    }
}
impl ::core::cmp::Eq for SOLE_AUTHENTICATION_LIST {}
impl ::core::default::Default for SOLE_AUTHENTICATION_LIST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_System_Com', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct SOLE_AUTHENTICATION_SERVICE {
    pub dwAuthnSvc: u32,
    pub dwAuthzSvc: u32,
    pub pPrincipalName: super::super::Foundation::PWSTR,
    pub hr: ::windows::core::HRESULT,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for SOLE_AUTHENTICATION_SERVICE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for SOLE_AUTHENTICATION_SERVICE {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for SOLE_AUTHENTICATION_SERVICE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SOLE_AUTHENTICATION_SERVICE").field("dwAuthnSvc", &self.dwAuthnSvc).field("dwAuthzSvc", &self.dwAuthzSvc).field("pPrincipalName", &self.pPrincipalName).field("hr", &self.hr).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for SOLE_AUTHENTICATION_SERVICE {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for SOLE_AUTHENTICATION_SERVICE {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SOLE_AUTHENTICATION_SERVICE>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for SOLE_AUTHENTICATION_SERVICE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for SOLE_AUTHENTICATION_SERVICE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_System_Com'*"]
pub struct STATDATA {
    pub formatetc: FORMATETC,
    pub advf: u32,
    pub pAdvSink: ::core::option::Option<IAdviseSink>,
    pub dwConnection: u32,
}
impl ::core::clone::Clone for STATDATA {
    fn clone(&self) -> Self {
        Self { formatetc: self.formatetc, advf: self.advf, pAdvSink: self.pAdvSink.clone(), dwConnection: self.dwConnection }
    }
}
impl ::core::fmt::Debug for STATDATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("STATDATA").field("formatetc", &self.formatetc).field("advf", &self.advf).field("pAdvSink", &self.pAdvSink).field("dwConnection", &self.dwConnection).finish()
    }
}
unsafe impl ::windows::core::Abi for STATDATA {
    type Abi = ::core::mem::ManuallyDrop<Self>;
}
impl ::core::cmp::PartialEq for STATDATA {
    fn eq(&self, other: &Self) -> bool {
        self.formatetc == other.formatetc && self.advf == other.advf && self.pAdvSink == other.pAdvSink && self.dwConnection == other.dwConnection
    }
}
impl ::core::cmp::Eq for STATDATA {}
impl ::core::default::Default for STATDATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_System_Com', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct STATSTG {
    pub pwcsName: super::super::Foundation::PWSTR,
    pub r#type: u32,
    pub cbSize: u64,
    pub mtime: super::super::Foundation::FILETIME,
    pub ctime: super::super::Foundation::FILETIME,
    pub atime: super::super::Foundation::FILETIME,
    pub grfMode: u32,
    pub grfLocksSupported: u32,
    pub clsid: ::windows::core::GUID,
    pub grfStateBits: u32,
    pub reserved: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for STATSTG {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for STATSTG {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for STATSTG {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("STATSTG").field("pwcsName", &self.pwcsName).field("type", &self.r#type).field("cbSize", &self.cbSize).field("mtime", &self.mtime).field("ctime", &self.ctime).field("atime", &self.atime).field("grfMode", &self.grfMode).field("grfLocksSupported", &self.grfLocksSupported).field("clsid", &self.clsid).field("grfStateBits", &self.grfStateBits).field("reserved", &self.reserved).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for STATSTG {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for STATSTG {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<STATSTG>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for STATSTG {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for STATSTG {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_System_Com', 'Win32_Foundation', 'Win32_Graphics_Gdi', 'Win32_System_Com_StructuredStorage'*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
pub struct STGMEDIUM {
    pub tymed: u32,
    pub Anonymous: STGMEDIUM_0,
    pub pUnkForRelease: ::core::option::Option<::windows::core::IUnknown>,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
impl ::core::clone::Clone for STGMEDIUM {
    fn clone(&self) -> Self {
        Self { tymed: self.tymed, Anonymous: self.Anonymous.clone(), pUnkForRelease: self.pUnkForRelease.clone() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
unsafe impl ::windows::core::Abi for STGMEDIUM {
    type Abi = ::core::mem::ManuallyDrop<Self>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
impl ::core::cmp::PartialEq for STGMEDIUM {
    fn eq(&self, other: &Self) -> bool {
        self.tymed == other.tymed && self.Anonymous == other.Anonymous && self.pUnkForRelease == other.pUnkForRelease
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
impl ::core::cmp::Eq for STGMEDIUM {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
impl ::core::default::Default for STGMEDIUM {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_System_Com', 'Win32_Foundation', 'Win32_Graphics_Gdi', 'Win32_System_Com_StructuredStorage'*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
pub union STGMEDIUM_0 {
    pub hBitmap: super::super::Graphics::Gdi::HBITMAP,
    pub hMetaFilePict: *mut ::core::ffi::c_void,
    pub hEnhMetaFile: super::super::Graphics::Gdi::HENHMETAFILE,
    pub hGlobal: isize,
    pub lpszFileName: super::super::Foundation::PWSTR,
    pub pstm: ::core::mem::ManuallyDrop<::core::option::Option<IStream>>,
    pub pstg: ::core::mem::ManuallyDrop<::core::option::Option<StructuredStorage::IStorage>>,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
impl ::core::clone::Clone for STGMEDIUM_0 {
    fn clone(&self) -> Self {
        unsafe { ::core::mem::transmute_copy(self) }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
unsafe impl ::windows::core::Abi for STGMEDIUM_0 {
    type Abi = ::core::mem::ManuallyDrop<Self>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
impl ::core::cmp::PartialEq for STGMEDIUM_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<STGMEDIUM_0>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
impl ::core::cmp::Eq for STGMEDIUM_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
impl ::core::default::Default for STGMEDIUM_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: 'Win32_System_Com'*"]
pub type STGTY = i32;
#[doc = "*Required features: 'Win32_System_Com'*"]
pub const STGTY_STORAGE: STGTY = 1i32;
#[doc = "*Required features: 'Win32_System_Com'*"]
pub const STGTY_STREAM: STGTY = 2i32;
#[doc = "*Required features: 'Win32_System_Com'*"]
pub const STGTY_LOCKBYTES: STGTY = 3i32;
#[doc = "*Required features: 'Win32_System_Com'*"]
pub const STGTY_PROPERTY: STGTY = 4i32;
#[doc = "*Required features: 'Win32_System_Com'*"]
pub const STGTY_REPEAT: i32 = 256i32;
#[doc = "*Required features: 'Win32_System_Com'*"]
pub const STG_LAYOUT_INTERLEAVED: i32 = 1i32;
#[doc = "*Required features: 'Win32_System_Com'*"]
pub const STG_LAYOUT_SEQUENTIAL: i32 = 0i32;
#[doc = "*Required features: 'Win32_System_Com'*"]
pub const STG_TOEND: i32 = -1i32;
#[doc = "*Required features: 'Win32_System_Com'*"]
pub type STREAM_SEEK = u32;
#[doc = "*Required features: 'Win32_System_Com'*"]
pub const STREAM_SEEK_SET: STREAM_SEEK = 0u32;
#[doc = "*Required features: 'Win32_System_Com'*"]
pub const STREAM_SEEK_CUR: STREAM_SEEK = 1u32;
#[doc = "*Required features: 'Win32_System_Com'*"]
pub const STREAM_SEEK_END: STREAM_SEEK = 2u32;
#[doc = "*Required features: 'Win32_System_Com'*"]
pub type SYSKIND = i32;
#[doc = "*Required features: 'Win32_System_Com'*"]
pub const SYS_WIN16: SYSKIND = 0i32;
#[doc = "*Required features: 'Win32_System_Com'*"]
pub const SYS_WIN32: SYSKIND = 1i32;
#[doc = "*Required features: 'Win32_System_Com'*"]
pub const SYS_MAC: SYSKIND = 2i32;
#[doc = "*Required features: 'Win32_System_Com'*"]
pub const SYS_WIN64: SYSKIND = 3i32;
#[doc = "*Required features: 'Win32_System_Com'*"]
#[inline]
pub unsafe fn SetErrorInfo<'a, Param1: ::windows::core::IntoParam<'a, IErrorInfo>>(dwreserved: u32, perrinfo: Param1) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetErrorInfo(dwreserved: u32, perrinfo: ::windows::core::RawPtr) -> ::windows::core::HRESULT;
        }
        SetErrorInfo(::core::mem::transmute(dwreserved), perrinfo.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_System_Com'*"]
pub type ShutdownType = i32;
#[doc = "*Required features: 'Win32_System_Com'*"]
pub const IdleShutdown: ShutdownType = 0i32;
#[doc = "*Required features: 'Win32_System_Com'*"]
pub const ForcedShutdown: ShutdownType = 1i32;
#[repr(C)]
#[doc = "*Required features: 'Win32_System_Com', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct StorageLayout {
    pub LayoutType: u32,
    pub pwcsElementName: super::super::Foundation::PWSTR,
    pub cOffset: i64,
    pub cBytes: i64,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for StorageLayout {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for StorageLayout {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for StorageLayout {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("StorageLayout").field("LayoutType", &self.LayoutType).field("pwcsElementName", &self.pwcsElementName).field("cOffset", &self.cOffset).field("cBytes", &self.cBytes).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for StorageLayout {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for StorageLayout {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<StorageLayout>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for StorageLayout {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for StorageLayout {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: 'Win32_System_Com', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn StringFromCLSID(rclsid: *const ::windows::core::GUID) -> ::windows::core::Result<super::super::Foundation::PWSTR> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn StringFromCLSID(rclsid: *const ::windows::core::GUID, lplpsz: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT;
        }
        let mut result__: super::super::Foundation::PWSTR = ::core::mem::zeroed();
        StringFromCLSID(::core::mem::transmute(rclsid), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::PWSTR>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_System_Com', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn StringFromGUID2(rguid: *const ::windows::core::GUID, lpsz: super::super::Foundation::PWSTR, cchmax: i32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn StringFromGUID2(rguid: *const ::windows::core::GUID, lpsz: super::super::Foundation::PWSTR, cchmax: i32) -> i32;
        }
        ::core::mem::transmute(StringFromGUID2(::core::mem::transmute(rguid), ::core::mem::transmute(lpsz), ::core::mem::transmute(cchmax)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_System_Com', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn StringFromIID(rclsid: *const ::windows::core::GUID) -> ::windows::core::Result<super::super::Foundation::PWSTR> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn StringFromIID(rclsid: *const ::windows::core::GUID, lplpsz: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT;
        }
        let mut result__: super::super::Foundation::PWSTR = ::core::mem::zeroed();
        StringFromIID(::core::mem::transmute(rclsid), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::PWSTR>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_System_Com'*"]
pub type THDTYPE = i32;
#[doc = "*Required features: 'Win32_System_Com'*"]
pub const THDTYPE_BLOCKMESSAGES: THDTYPE = 0i32;
#[doc = "*Required features: 'Win32_System_Com'*"]
pub const THDTYPE_PROCESSMESSAGES: THDTYPE = 1i32;
#[repr(C)]
#[doc = "*Required features: 'Win32_System_Com'*"]
pub struct TLIBATTR {
    pub guid: ::windows::core::GUID,
    pub lcid: u32,
    pub syskind: SYSKIND,
    pub wMajorVerNum: u16,
    pub wMinorVerNum: u16,
    pub wLibFlags: u16,
}
impl ::core::marker::Copy for TLIBATTR {}
impl ::core::clone::Clone for TLIBATTR {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for TLIBATTR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TLIBATTR").field("guid", &self.guid).field("lcid", &self.lcid).field("syskind", &self.syskind).field("wMajorVerNum", &self.wMajorVerNum).field("wMinorVerNum", &self.wMinorVerNum).field("wLibFlags", &self.wLibFlags).finish()
    }
}
unsafe impl ::windows::core::Abi for TLIBATTR {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for TLIBATTR {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<TLIBATTR>()) == 0 }
    }
}
impl ::core::cmp::Eq for TLIBATTR {}
impl ::core::default::Default for TLIBATTR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: 'Win32_System_Com'*"]
pub type TYMED = i32;
#[doc = "*Required features: 'Win32_System_Com'*"]
pub const TYMED_HGLOBAL: TYMED = 1i32;
#[doc = "*Required features: 'Win32_System_Com'*"]
pub const TYMED_FILE: TYMED = 2i32;
#[doc = "*Required features: 'Win32_System_Com'*"]
pub const TYMED_ISTREAM: TYMED = 4i32;
#[doc = "*Required features: 'Win32_System_Com'*"]
pub const TYMED_ISTORAGE: TYMED = 8i32;
#[doc = "*Required features: 'Win32_System_Com'*"]
pub const TYMED_GDI: TYMED = 16i32;
#[doc = "*Required features: 'Win32_System_Com'*"]
pub const TYMED_MFPICT: TYMED = 32i32;
#[doc = "*Required features: 'Win32_System_Com'*"]
pub const TYMED_ENHMF: TYMED = 64i32;
#[doc = "*Required features: 'Win32_System_Com'*"]
pub const TYMED_NULL: TYMED = 0i32;
#[repr(C)]
#[doc = "*Required features: 'Win32_System_Com', 'Win32_Foundation', 'Win32_System_Ole'*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
pub struct TYPEATTR {
    pub guid: ::windows::core::GUID,
    pub lcid: u32,
    pub dwReserved: u32,
    pub memidConstructor: i32,
    pub memidDestructor: i32,
    pub lpstrSchema: super::super::Foundation::PWSTR,
    pub cbSizeInstance: u32,
    pub typekind: TYPEKIND,
    pub cFuncs: u16,
    pub cVars: u16,
    pub cImplTypes: u16,
    pub cbSizeVft: u16,
    pub cbAlignment: u16,
    pub wTypeFlags: u16,
    pub wMajorVerNum: u16,
    pub wMinorVerNum: u16,
    pub tdescAlias: TYPEDESC,
    pub idldescType: IDLDESC,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
impl ::core::marker::Copy for TYPEATTR {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
impl ::core::clone::Clone for TYPEATTR {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
unsafe impl ::windows::core::Abi for TYPEATTR {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
impl ::core::cmp::PartialEq for TYPEATTR {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<TYPEATTR>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
impl ::core::cmp::Eq for TYPEATTR {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
impl ::core::default::Default for TYPEATTR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_System_Com', 'Win32_System_Ole'*"]
#[cfg(feature = "Win32_System_Ole")]
pub struct TYPEDESC {
    pub Anonymous: TYPEDESC_0,
    pub vt: u16,
}
#[cfg(feature = "Win32_System_Ole")]
impl ::core::marker::Copy for TYPEDESC {}
#[cfg(feature = "Win32_System_Ole")]
impl ::core::clone::Clone for TYPEDESC {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_System_Ole")]
unsafe impl ::windows::core::Abi for TYPEDESC {
    type Abi = Self;
}
#[cfg(feature = "Win32_System_Ole")]
impl ::core::cmp::PartialEq for TYPEDESC {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<TYPEDESC>()) == 0 }
    }
}
#[cfg(feature = "Win32_System_Ole")]
impl ::core::cmp::Eq for TYPEDESC {}
#[cfg(feature = "Win32_System_Ole")]
impl ::core::default::Default for TYPEDESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_System_Com', 'Win32_System_Ole'*"]
#[cfg(feature = "Win32_System_Ole")]
pub union TYPEDESC_0 {
    pub lptdesc: *mut TYPEDESC,
    pub lpadesc: *mut super::Ole::ARRAYDESC,
    pub hreftype: u32,
}
#[cfg(feature = "Win32_System_Ole")]
impl ::core::marker::Copy for TYPEDESC_0 {}
#[cfg(feature = "Win32_System_Ole")]
impl ::core::clone::Clone for TYPEDESC_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_System_Ole")]
unsafe impl ::windows::core::Abi for TYPEDESC_0 {
    type Abi = Self;
}
#[cfg(feature = "Win32_System_Ole")]
impl ::core::cmp::PartialEq for TYPEDESC_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<TYPEDESC_0>()) == 0 }
    }
}
#[cfg(feature = "Win32_System_Ole")]
impl ::core::cmp::Eq for TYPEDESC_0 {}
#[cfg(feature = "Win32_System_Ole")]
impl ::core::default::Default for TYPEDESC_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: 'Win32_System_Com'*"]
pub type TYPEKIND = i32;
#[doc = "*Required features: 'Win32_System_Com'*"]
pub const TKIND_ENUM: TYPEKIND = 0i32;
#[doc = "*Required features: 'Win32_System_Com'*"]
pub const TKIND_RECORD: TYPEKIND = 1i32;
#[doc = "*Required features: 'Win32_System_Com'*"]
pub const TKIND_MODULE: TYPEKIND = 2i32;
#[doc = "*Required features: 'Win32_System_Com'*"]
pub const TKIND_INTERFACE: TYPEKIND = 3i32;
#[doc = "*Required features: 'Win32_System_Com'*"]
pub const TKIND_DISPATCH: TYPEKIND = 4i32;
#[doc = "*Required features: 'Win32_System_Com'*"]
pub const TKIND_COCLASS: TYPEKIND = 5i32;
#[doc = "*Required features: 'Win32_System_Com'*"]
pub const TKIND_ALIAS: TYPEKIND = 6i32;
#[doc = "*Required features: 'Win32_System_Com'*"]
pub const TKIND_UNION: TYPEKIND = 7i32;
#[doc = "*Required features: 'Win32_System_Com'*"]
pub const TKIND_MAX: TYPEKIND = 8i32;
#[doc = "*Required features: 'Win32_System_Com'*"]
pub type TYSPEC = i32;
#[doc = "*Required features: 'Win32_System_Com'*"]
pub const TYSPEC_CLSID: TYSPEC = 0i32;
#[doc = "*Required features: 'Win32_System_Com'*"]
pub const TYSPEC_FILEEXT: TYSPEC = 1i32;
#[doc = "*Required features: 'Win32_System_Com'*"]
pub const TYSPEC_MIMETYPE: TYSPEC = 2i32;
#[doc = "*Required features: 'Win32_System_Com'*"]
pub const TYSPEC_FILENAME: TYSPEC = 3i32;
#[doc = "*Required features: 'Win32_System_Com'*"]
pub const TYSPEC_PROGID: TYSPEC = 4i32;
#[doc = "*Required features: 'Win32_System_Com'*"]
pub const TYSPEC_PACKAGENAME: TYSPEC = 5i32;
#[doc = "*Required features: 'Win32_System_Com'*"]
pub const TYSPEC_OBJECTID: TYSPEC = 6i32;
#[doc = "*Required features: 'Win32_System_Com'*"]
pub type URI_CREATE_FLAGS = u32;
#[doc = "*Required features: 'Win32_System_Com'*"]
pub const Uri_CREATE_ALLOW_RELATIVE: URI_CREATE_FLAGS = 1u32;
#[doc = "*Required features: 'Win32_System_Com'*"]
pub const Uri_CREATE_ALLOW_IMPLICIT_WILDCARD_SCHEME: URI_CREATE_FLAGS = 2u32;
#[doc = "*Required features: 'Win32_System_Com'*"]
pub const Uri_CREATE_ALLOW_IMPLICIT_FILE_SCHEME: URI_CREATE_FLAGS = 4u32;
#[doc = "*Required features: 'Win32_System_Com'*"]
pub const Uri_CREATE_NOFRAG: URI_CREATE_FLAGS = 8u32;
#[doc = "*Required features: 'Win32_System_Com'*"]
pub const Uri_CREATE_NO_CANONICALIZE: URI_CREATE_FLAGS = 16u32;
#[doc = "*Required features: 'Win32_System_Com'*"]
pub const Uri_CREATE_CANONICALIZE: URI_CREATE_FLAGS = 256u32;
#[doc = "*Required features: 'Win32_System_Com'*"]
pub const Uri_CREATE_FILE_USE_DOS_PATH: URI_CREATE_FLAGS = 32u32;
#[doc = "*Required features: 'Win32_System_Com'*"]
pub const Uri_CREATE_DECODE_EXTRA_INFO: URI_CREATE_FLAGS = 64u32;
#[doc = "*Required features: 'Win32_System_Com'*"]
pub const Uri_CREATE_NO_DECODE_EXTRA_INFO: URI_CREATE_FLAGS = 128u32;
#[doc = "*Required features: 'Win32_System_Com'*"]
pub const Uri_CREATE_CRACK_UNKNOWN_SCHEMES: URI_CREATE_FLAGS = 512u32;
#[doc = "*Required features: 'Win32_System_Com'*"]
pub const Uri_CREATE_NO_CRACK_UNKNOWN_SCHEMES: URI_CREATE_FLAGS = 1024u32;
#[doc = "*Required features: 'Win32_System_Com'*"]
pub const Uri_CREATE_PRE_PROCESS_HTML_URI: URI_CREATE_FLAGS = 2048u32;
#[doc = "*Required features: 'Win32_System_Com'*"]
pub const Uri_CREATE_NO_PRE_PROCESS_HTML_URI: URI_CREATE_FLAGS = 4096u32;
#[doc = "*Required features: 'Win32_System_Com'*"]
pub const Uri_CREATE_IE_SETTINGS: URI_CREATE_FLAGS = 8192u32;
#[doc = "*Required features: 'Win32_System_Com'*"]
pub const Uri_CREATE_NO_IE_SETTINGS: URI_CREATE_FLAGS = 16384u32;
#[doc = "*Required features: 'Win32_System_Com'*"]
pub const Uri_CREATE_NO_ENCODE_FORBIDDEN_CHARACTERS: URI_CREATE_FLAGS = 32768u32;
#[doc = "*Required features: 'Win32_System_Com'*"]
pub const Uri_CREATE_NORMALIZE_INTL_CHARACTERS: URI_CREATE_FLAGS = 65536u32;
#[doc = "*Required features: 'Win32_System_Com'*"]
pub const Uri_CREATE_CANONICALIZE_ABSOLUTE: URI_CREATE_FLAGS = 131072u32;
#[doc = "*Required features: 'Win32_System_Com'*"]
pub type Uri_PROPERTY = i32;
#[doc = "*Required features: 'Win32_System_Com'*"]
pub const Uri_PROPERTY_ABSOLUTE_URI: Uri_PROPERTY = 0i32;
#[doc = "*Required features: 'Win32_System_Com'*"]
pub const Uri_PROPERTY_STRING_START: Uri_PROPERTY = 0i32;
#[doc = "*Required features: 'Win32_System_Com'*"]
pub const Uri_PROPERTY_AUTHORITY: Uri_PROPERTY = 1i32;
#[doc = "*Required features: 'Win32_System_Com'*"]
pub const Uri_PROPERTY_DISPLAY_URI: Uri_PROPERTY = 2i32;
#[doc = "*Required features: 'Win32_System_Com'*"]
pub const Uri_PROPERTY_DOMAIN: Uri_PROPERTY = 3i32;
#[doc = "*Required features: 'Win32_System_Com'*"]
pub const Uri_PROPERTY_EXTENSION: Uri_PROPERTY = 4i32;
#[doc = "*Required features: 'Win32_System_Com'*"]
pub const Uri_PROPERTY_FRAGMENT: Uri_PROPERTY = 5i32;
#[doc = "*Required features: 'Win32_System_Com'*"]
pub const Uri_PROPERTY_HOST: Uri_PROPERTY = 6i32;
#[doc = "*Required features: 'Win32_System_Com'*"]
pub const Uri_PROPERTY_PASSWORD: Uri_PROPERTY = 7i32;
#[doc = "*Required features: 'Win32_System_Com'*"]
pub const Uri_PROPERTY_PATH: Uri_PROPERTY = 8i32;
#[doc = "*Required features: 'Win32_System_Com'*"]
pub const Uri_PROPERTY_PATH_AND_QUERY: Uri_PROPERTY = 9i32;
#[doc = "*Required features: 'Win32_System_Com'*"]
pub const Uri_PROPERTY_QUERY: Uri_PROPERTY = 10i32;
#[doc = "*Required features: 'Win32_System_Com'*"]
pub const Uri_PROPERTY_RAW_URI: Uri_PROPERTY = 11i32;
#[doc = "*Required features: 'Win32_System_Com'*"]
pub const Uri_PROPERTY_SCHEME_NAME: Uri_PROPERTY = 12i32;
#[doc = "*Required features: 'Win32_System_Com'*"]
pub const Uri_PROPERTY_USER_INFO: Uri_PROPERTY = 13i32;
#[doc = "*Required features: 'Win32_System_Com'*"]
pub const Uri_PROPERTY_USER_NAME: Uri_PROPERTY = 14i32;
#[doc = "*Required features: 'Win32_System_Com'*"]
pub const Uri_PROPERTY_STRING_LAST: Uri_PROPERTY = 14i32;
#[doc = "*Required features: 'Win32_System_Com'*"]
pub const Uri_PROPERTY_HOST_TYPE: Uri_PROPERTY = 15i32;
#[doc = "*Required features: 'Win32_System_Com'*"]
pub const Uri_PROPERTY_DWORD_START: Uri_PROPERTY = 15i32;
#[doc = "*Required features: 'Win32_System_Com'*"]
pub const Uri_PROPERTY_PORT: Uri_PROPERTY = 16i32;
#[doc = "*Required features: 'Win32_System_Com'*"]
pub const Uri_PROPERTY_SCHEME: Uri_PROPERTY = 17i32;
#[doc = "*Required features: 'Win32_System_Com'*"]
pub const Uri_PROPERTY_ZONE: Uri_PROPERTY = 18i32;
#[doc = "*Required features: 'Win32_System_Com'*"]
pub const Uri_PROPERTY_DWORD_LAST: Uri_PROPERTY = 18i32;
#[repr(C)]
#[doc = "*Required features: 'Win32_System_Com', 'Win32_Foundation', 'Win32_System_Ole'*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
pub struct VARDESC {
    pub memid: i32,
    pub lpstrSchema: super::super::Foundation::PWSTR,
    pub Anonymous: VARDESC_0,
    pub elemdescVar: ELEMDESC,
    pub wVarFlags: u16,
    pub varkind: VARKIND,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
impl ::core::marker::Copy for VARDESC {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
impl ::core::clone::Clone for VARDESC {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
unsafe impl ::windows::core::Abi for VARDESC {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
impl ::core::cmp::PartialEq for VARDESC {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<VARDESC>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
impl ::core::cmp::Eq for VARDESC {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
impl ::core::default::Default for VARDESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_System_Com', 'Win32_Foundation', 'Win32_System_Ole'*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
pub union VARDESC_0 {
    pub oInst: u32,
    pub lpvarValue: *mut VARIANT,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
impl ::core::marker::Copy for VARDESC_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
impl ::core::clone::Clone for VARDESC_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
unsafe impl ::windows::core::Abi for VARDESC_0 {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
impl ::core::cmp::PartialEq for VARDESC_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<VARDESC_0>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
impl ::core::cmp::Eq for VARDESC_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
impl ::core::default::Default for VARDESC_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_System_Com', 'Win32_Foundation', 'Win32_System_Ole'*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
pub struct VARIANT {
    pub Anonymous: VARIANT_0,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
impl ::core::clone::Clone for VARIANT {
    fn clone(&self) -> Self {
        Self { Anonymous: self.Anonymous.clone() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
unsafe impl ::windows::core::Abi for VARIANT {
    type Abi = ::core::mem::ManuallyDrop<Self>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
impl ::core::cmp::PartialEq for VARIANT {
    fn eq(&self, other: &Self) -> bool {
        self.Anonymous == other.Anonymous
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
impl ::core::cmp::Eq for VARIANT {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
impl ::core::default::Default for VARIANT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_System_Com', 'Win32_Foundation', 'Win32_System_Ole'*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
pub union VARIANT_0 {
    pub Anonymous: ::core::mem::ManuallyDrop<VARIANT_0_0>,
    pub decVal: super::super::Foundation::DECIMAL,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
impl ::core::clone::Clone for VARIANT_0 {
    fn clone(&self) -> Self {
        unsafe { ::core::mem::transmute_copy(self) }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
unsafe impl ::windows::core::Abi for VARIANT_0 {
    type Abi = ::core::mem::ManuallyDrop<Self>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
impl ::core::cmp::PartialEq for VARIANT_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<VARIANT_0>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
impl ::core::cmp::Eq for VARIANT_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
impl ::core::default::Default for VARIANT_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_System_Com', 'Win32_Foundation', 'Win32_System_Ole'*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
pub struct VARIANT_0_0 {
    pub vt: u16,
    pub wReserved1: u16,
    pub wReserved2: u16,
    pub wReserved3: u16,
    pub Anonymous: VARIANT_0_0_0,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
impl ::core::clone::Clone for VARIANT_0_0 {
    fn clone(&self) -> Self {
        Self { vt: self.vt, wReserved1: self.wReserved1, wReserved2: self.wReserved2, wReserved3: self.wReserved3, Anonymous: self.Anonymous.clone() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
unsafe impl ::windows::core::Abi for VARIANT_0_0 {
    type Abi = ::core::mem::ManuallyDrop<Self>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
impl ::core::cmp::PartialEq for VARIANT_0_0 {
    fn eq(&self, other: &Self) -> bool {
        self.vt == other.vt && self.wReserved1 == other.wReserved1 && self.wReserved2 == other.wReserved2 && self.wReserved3 == other.wReserved3 && self.Anonymous == other.Anonymous
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
impl ::core::cmp::Eq for VARIANT_0_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
impl ::core::default::Default for VARIANT_0_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_System_Com', 'Win32_Foundation', 'Win32_System_Ole'*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
pub union VARIANT_0_0_0 {
    pub llVal: i64,
    pub lVal: i32,
    pub bVal: u8,
    pub iVal: i16,
    pub fltVal: f32,
    pub dblVal: f64,
    pub boolVal: i16,
    pub __OBSOLETE__VARIANT_BOOL: i16,
    pub scode: i32,
    pub cyVal: CY,
    pub date: f64,
    pub bstrVal: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>,
    pub punkVal: ::core::mem::ManuallyDrop<::core::option::Option<::windows::core::IUnknown>>,
    pub pdispVal: ::core::mem::ManuallyDrop<::core::option::Option<IDispatch>>,
    pub parray: *mut SAFEARRAY,
    pub pbVal: *mut u8,
    pub piVal: *mut i16,
    pub plVal: *mut i32,
    pub pllVal: *mut i64,
    pub pfltVal: *mut f32,
    pub pdblVal: *mut f64,
    pub pboolVal: *mut i16,
    pub __OBSOLETE__VARIANT_PBOOL: *mut i16,
    pub pscode: *mut i32,
    pub pcyVal: *mut CY,
    pub pdate: *mut f64,
    pub pbstrVal: *mut super::super::Foundation::BSTR,
    pub ppunkVal: *mut ::core::option::Option<::windows::core::IUnknown>,
    pub ppdispVal: *mut ::core::option::Option<IDispatch>,
    pub pparray: *mut *mut SAFEARRAY,
    pub pvarVal: *mut VARIANT,
    pub byref: *mut ::core::ffi::c_void,
    pub cVal: super::super::Foundation::CHAR,
    pub uiVal: u16,
    pub ulVal: u32,
    pub ullVal: u64,
    pub intVal: i32,
    pub uintVal: u32,
    pub pdecVal: *mut super::super::Foundation::DECIMAL,
    pub pcVal: super::super::Foundation::PSTR,
    pub puiVal: *mut u16,
    pub pulVal: *mut u32,
    pub pullVal: *mut u64,
    pub pintVal: *mut i32,
    pub puintVal: *mut u32,
    pub Anonymous: ::core::mem::ManuallyDrop<VARIANT_0_0_0_0>,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
impl ::core::clone::Clone for VARIANT_0_0_0 {
    fn clone(&self) -> Self {
        unsafe { ::core::mem::transmute_copy(self) }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
unsafe impl ::windows::core::Abi for VARIANT_0_0_0 {
    type Abi = ::core::mem::ManuallyDrop<Self>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
impl ::core::cmp::PartialEq for VARIANT_0_0_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<VARIANT_0_0_0>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
impl ::core::cmp::Eq for VARIANT_0_0_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
impl ::core::default::Default for VARIANT_0_0_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_System_Com', 'Win32_Foundation', 'Win32_System_Ole'*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
pub struct VARIANT_0_0_0_0 {
    pub pvRecord: *mut ::core::ffi::c_void,
    pub pRecInfo: ::core::option::Option<super::Ole::IRecordInfo>,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
impl ::core::clone::Clone for VARIANT_0_0_0_0 {
    fn clone(&self) -> Self {
        Self { pvRecord: self.pvRecord, pRecInfo: self.pRecInfo.clone() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
impl ::core::fmt::Debug for VARIANT_0_0_0_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("VARIANT_0_0_0_0").field("pvRecord", &self.pvRecord).field("pRecInfo", &self.pRecInfo).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
unsafe impl ::windows::core::Abi for VARIANT_0_0_0_0 {
    type Abi = ::core::mem::ManuallyDrop<Self>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
impl ::core::cmp::PartialEq for VARIANT_0_0_0_0 {
    fn eq(&self, other: &Self) -> bool {
        self.pvRecord == other.pvRecord && self.pRecInfo == other.pRecInfo
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
impl ::core::cmp::Eq for VARIANT_0_0_0_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
impl ::core::default::Default for VARIANT_0_0_0_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: 'Win32_System_Com'*"]
pub type VARKIND = i32;
#[doc = "*Required features: 'Win32_System_Com'*"]
pub const VAR_PERINSTANCE: VARKIND = 0i32;
#[doc = "*Required features: 'Win32_System_Com'*"]
pub const VAR_STATIC: VARKIND = 1i32;
#[doc = "*Required features: 'Win32_System_Com'*"]
pub const VAR_CONST: VARKIND = 2i32;
#[doc = "*Required features: 'Win32_System_Com'*"]
pub const VAR_DISPATCH: VARKIND = 3i32;
#[repr(C)]
#[doc = "*Required features: 'Win32_System_Com'*"]
pub struct WORD_BLOB {
    pub clSize: u32,
    pub asData: [u16; 1],
}
impl ::core::marker::Copy for WORD_BLOB {}
impl ::core::clone::Clone for WORD_BLOB {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WORD_BLOB {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WORD_BLOB").field("clSize", &self.clSize).field("asData", &self.asData).finish()
    }
}
unsafe impl ::windows::core::Abi for WORD_BLOB {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WORD_BLOB {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WORD_BLOB>()) == 0 }
    }
}
impl ::core::cmp::Eq for WORD_BLOB {}
impl ::core::default::Default for WORD_BLOB {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_System_Com', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct uCLSSPEC {
    pub tyspec: u32,
    pub tagged_union: uCLSSPEC_0,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for uCLSSPEC {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for uCLSSPEC {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for uCLSSPEC {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for uCLSSPEC {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<uCLSSPEC>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for uCLSSPEC {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for uCLSSPEC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_System_Com', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub union uCLSSPEC_0 {
    pub clsid: ::windows::core::GUID,
    pub pFileExt: super::super::Foundation::PWSTR,
    pub pMimeType: super::super::Foundation::PWSTR,
    pub pProgId: super::super::Foundation::PWSTR,
    pub pFileName: super::super::Foundation::PWSTR,
    pub ByName: uCLSSPEC_0_0,
    pub ByObjectId: uCLSSPEC_0_1,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for uCLSSPEC_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for uCLSSPEC_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for uCLSSPEC_0 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for uCLSSPEC_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<uCLSSPEC_0>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for uCLSSPEC_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for uCLSSPEC_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_System_Com', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct uCLSSPEC_0_0 {
    pub pPackageName: super::super::Foundation::PWSTR,
    pub PolicyId: ::windows::core::GUID,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for uCLSSPEC_0_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for uCLSSPEC_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for uCLSSPEC_0_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("uCLSSPEC_0_0").field("pPackageName", &self.pPackageName).field("PolicyId", &self.PolicyId).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for uCLSSPEC_0_0 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for uCLSSPEC_0_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<uCLSSPEC_0_0>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for uCLSSPEC_0_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for uCLSSPEC_0_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_System_Com', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct uCLSSPEC_0_1 {
    pub ObjectId: ::windows::core::GUID,
    pub PolicyId: ::windows::core::GUID,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for uCLSSPEC_0_1 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for uCLSSPEC_0_1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for uCLSSPEC_0_1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("uCLSSPEC_0_1").field("ObjectId", &self.ObjectId).field("PolicyId", &self.PolicyId).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for uCLSSPEC_0_1 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for uCLSSPEC_0_1 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<uCLSSPEC_0_1>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for uCLSSPEC_0_1 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for uCLSSPEC_0_1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_System_Com'*"]
pub struct userFLAG_STGMEDIUM {
    pub ContextFlags: i32,
    pub fPassOwnership: i32,
    pub Stgmed: userSTGMEDIUM,
}
impl ::core::clone::Clone for userFLAG_STGMEDIUM {
    fn clone(&self) -> Self {
        Self { ContextFlags: self.ContextFlags, fPassOwnership: self.fPassOwnership, Stgmed: self.Stgmed.clone() }
    }
}
impl ::core::fmt::Debug for userFLAG_STGMEDIUM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("userFLAG_STGMEDIUM").field("ContextFlags", &self.ContextFlags).field("fPassOwnership", &self.fPassOwnership).field("Stgmed", &self.Stgmed).finish()
    }
}
unsafe impl ::windows::core::Abi for userFLAG_STGMEDIUM {
    type Abi = ::core::mem::ManuallyDrop<Self>;
}
impl ::core::cmp::PartialEq for userFLAG_STGMEDIUM {
    fn eq(&self, other: &Self) -> bool {
        self.ContextFlags == other.ContextFlags && self.fPassOwnership == other.fPassOwnership && self.Stgmed == other.Stgmed
    }
}
impl ::core::cmp::Eq for userFLAG_STGMEDIUM {}
impl ::core::default::Default for userFLAG_STGMEDIUM {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_System_Com'*"]
pub struct userSTGMEDIUM {
    pub pUnkForRelease: ::core::option::Option<::windows::core::IUnknown>,
}
impl ::core::clone::Clone for userSTGMEDIUM {
    fn clone(&self) -> Self {
        Self { pUnkForRelease: self.pUnkForRelease.clone() }
    }
}
impl ::core::fmt::Debug for userSTGMEDIUM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("userSTGMEDIUM").field("pUnkForRelease", &self.pUnkForRelease).finish()
    }
}
unsafe impl ::windows::core::Abi for userSTGMEDIUM {
    type Abi = ::core::mem::ManuallyDrop<Self>;
}
impl ::core::cmp::PartialEq for userSTGMEDIUM {
    fn eq(&self, other: &Self) -> bool {
        self.pUnkForRelease == other.pUnkForRelease
    }
}
impl ::core::cmp::Eq for userSTGMEDIUM {}
impl ::core::default::Default for userSTGMEDIUM {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_System_Com', 'Win32_Foundation', 'Win32_Graphics_Gdi', 'Win32_System_SystemServices'*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_SystemServices"))]
pub struct userSTGMEDIUM_0 {
    pub tymed: u32,
    pub u: userSTGMEDIUM_0_0,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_SystemServices"))]
impl ::core::marker::Copy for userSTGMEDIUM_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_SystemServices"))]
impl ::core::clone::Clone for userSTGMEDIUM_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_SystemServices"))]
unsafe impl ::windows::core::Abi for userSTGMEDIUM_0 {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_SystemServices"))]
impl ::core::cmp::PartialEq for userSTGMEDIUM_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<userSTGMEDIUM_0>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_SystemServices"))]
impl ::core::cmp::Eq for userSTGMEDIUM_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_SystemServices"))]
impl ::core::default::Default for userSTGMEDIUM_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_System_Com', 'Win32_Foundation', 'Win32_Graphics_Gdi', 'Win32_System_SystemServices'*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_SystemServices"))]
pub union userSTGMEDIUM_0_0 {
    pub hMetaFilePict: *mut super::SystemServices::userHMETAFILEPICT,
    pub hHEnhMetaFile: *mut super::SystemServices::userHENHMETAFILE,
    pub hGdiHandle: *mut GDI_OBJECT,
    pub hGlobal: *mut super::SystemServices::userHGLOBAL,
    pub lpszFileName: super::super::Foundation::PWSTR,
    pub pstm: *mut BYTE_BLOB,
    pub pstg: *mut BYTE_BLOB,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_SystemServices"))]
impl ::core::marker::Copy for userSTGMEDIUM_0_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_SystemServices"))]
impl ::core::clone::Clone for userSTGMEDIUM_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_SystemServices"))]
unsafe impl ::windows::core::Abi for userSTGMEDIUM_0_0 {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_SystemServices"))]
impl ::core::cmp::PartialEq for userSTGMEDIUM_0_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<userSTGMEDIUM_0_0>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_SystemServices"))]
impl ::core::cmp::Eq for userSTGMEDIUM_0_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_SystemServices"))]
impl ::core::default::Default for userSTGMEDIUM_0_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
